//! Linearization of a dependency tree (word-order generator) — tables over classes, trained on UD.
//! 1) Side of a dependent relative to its head: a table with back-off from the exact key (lemma of closed classes,
//!    head tag, sentence type, construction flags — whether the head has expl/aux) to the coarse one (relation).
//! 2) Order on each side: a Markov chain over classes (<s> d1 … dk <H> on the left,
//!    <H> d1 … dk </s> on the right), three key levels with smoothing; the best sequence —
//!    dynamic programming over subsets. Subtrees are contiguous (projective).
//! Keys are structures of enums and lemma numbers, no strings.

use crate::conllu::Sentence;
use crate::dict;
use crate::gram::{Rel, Sym, Tag};
use crate::hash::FastMap;
use std::collections::BTreeMap;

/// Node for linearization: tag, relation to head, head (0 — root), lowercase lemma.
#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub tag: Tag,
    pub rel: Rel,
    pub head: usize,
    pub lex: Sym,
}

/// Node class: relation and tag; for closed classes and punctuation — also the lemma ("maybe" ≠ "too", "," ≠ ".").
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
struct Class {
    rel: Rel,
    tag: Tag,
    lex: Option<Sym>,
}

fn class(n: &Node) -> Class {
    let lexical = n.rel == Rel::Punct || n.tag.is_closed();
    Class { rel: n.rel, tag: n.tag, lex: lexical.then_some(n.lex) }
}

/// Side-table keys, from exact to coarse.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
enum SideKey {
    /// class, head tag, interrogative, head has expl, head has aux, dependent is "heavy"
    S4(Class, Tag, bool, bool, bool, bool),
    S3(Class, Tag, bool, bool),
    S2(Class, Tag),
    S1(Rel, Tag, Tag),
    S0(Rel),
}

/// Chain symbol on a side: boundaries and a node at one of three levels of detail.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
enum Link {
    Start,
    Head,
    End,
    /// level 0: class with lemma + sentence type
    L0(Class, bool),
    /// level 1: relation and tag
    L1(Rel, Tag),
    /// level 2: relation
    L2(Rel),
}

fn link(n: &Node, level: u8, q: bool) -> Link {
    match level {
        0 => Link::L0(class(n), q),
        1 => Link::L1(n.rel, n.tag),
        _ => Link::L2(n.rel),
    }
}

/// Sentence type — interrogative (in the graph — a sentence bit set by the encoder from the original order):
/// there is a "?" (`qmark`), or the sentence starts with an auxiliary/modal verb or a question word
/// ("is it raining", "what's the forecast" — queries often lack punctuation).
pub fn mood(nodes: &[Node], qmark: bool) -> bool {
    qmark
        || nodes.first().is_some_and(|f| {
            f.tag.is_wh() || f.tag == Tag::MD || (matches!(f.rel, Rel::Aux | Rel::Cop | Rel::AuxPass) && f.tag.is_verb())
        })
}

/// Nodes of a treebank sentence; `lex` gives the lemma key (lowercase number); the second value —
/// whether "?" is among the punctuation.
pub fn sentence_nodes(s: &Sentence, mut lex: impl FnMut(&str) -> Sym) -> (Vec<Node>, bool) {
    let nodes = s.tokens.iter().map(|t| Node { tag: t.tag.unwrap_or(Tag::XX), rel: t.rel, head: t.head, lex: lex(&t.lemma) }).collect();
    let qmark = s.tokens.iter().any(|t| t.rel == Rel::Punct && t.lemma.contains('?'));
    (nodes, qmark)
}

const MIN: u32 = 3;

/// Decision on the side of a dependent (for explanation).
#[derive(Clone, Copy, Debug)]
pub struct Side {
    pub dep: usize,
    pub head: usize,
    pub left: bool,
    pub level: Option<u8>,
    pub counts: (u32, u32),
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Linearizer {
    /// side key → (left, right)
    side: FastMap<SideKey, (u32, u32)>,
    /// (level, side, previous, next) → count; (level, side, previous) → sum
    big: FastMap<(u8, bool, Link, Link), f64>,
    ctx: FastMap<(u8, bool, Link), f64>,
}

/// Construction flags of the head: whether it has expl and aux dependents.
fn construction(nodes: &[Node], kids: &[usize]) -> (bool, bool) {
    let has = |r: Rel| kids.iter().any(|&k| nodes[k - 1].rel.base() == r);
    (has(Rel::Expl), has(Rel::Aux))
}

/// A "heavy" dependent — has its own complement or adverbial (obl, nmod, ccomp, xcomp, advcl, acl, obj).
/// So a modifier with a complement goes after the noun: "gusts as high as 20 mph", "people aware of it".
fn heavy(nodes: &[Node], kids: &[usize]) -> bool {
    kids.iter().any(|&k| matches!(nodes[k - 1].rel.base(), Rel::Obl | Rel::Nmod | Rel::Ccomp | Rel::Xcomp | Rel::Advcl | Rel::Acl | Rel::Obj))
}

fn side_keys(n: &Node, hx: Tag, q: bool, cons: (bool, bool), hv: bool) -> [SideKey; 5] {
    let c = class(n);
    [SideKey::S4(c, hx, q, cons.0, cons.1, hv), SideKey::S3(c, hx, q, hv), SideKey::S2(c, hx), SideKey::S1(n.rel, n.tag, hx), SideKey::S0(n.rel)]
}

impl Linearizer {
    /// Training on treebank sentences; lemma keys of closed classes are numbers from the built-in dictionary.
    pub fn train(sents: &[Sentence]) -> Linearizer {
        let mut m = Linearizer::default();
        for s in sents.iter().filter(|s| s.tagged()) {
            let (nodes, qmark) = sentence_nodes(s, dict::lex);
            m.learn(&nodes, mood(&nodes, qmark));
        }
        m
    }

    fn learn(&mut self, nodes: &[Node], q: bool) {
        let n = nodes.len();
        let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for (i, t) in nodes.iter().enumerate() {
            kids[t.head.min(n)].push(i + 1);
        }
        for h in 1..=n {
            let hx = nodes[h - 1].tag;
            let cons = construction(nodes, &kids[h]);
            for &d in &kids[h] {
                let left = d < h;
                for k in side_keys(&nodes[d - 1], hx, q, cons, heavy(nodes, &kids[d])) {
                    let e = self.side.entry(k).or_default();
                    if left { e.0 += 1 } else { e.1 += 1 }
                }
            }
            for side_left in [true, false] {
                let mut ds: Vec<usize> = kids[h].iter().copied().filter(|&d| (d < h) == side_left).collect();
                ds.sort_unstable();
                for level in 0..3u8 {
                    let mut seq: Vec<Link> = Vec::with_capacity(ds.len() + 2);
                    seq.push(if side_left { Link::Start } else { Link::Head });
                    seq.extend(ds.iter().map(|&d| link(&nodes[d - 1], level, q)));
                    seq.push(if side_left { Link::Head } else { Link::End });
                    for w in seq.windows(2) {
                        *self.big.entry((level, side_left, w[0], w[1])).or_default() += 1.0;
                        *self.ctx.entry((level, side_left, w[0])).or_default() += 1.0;
                    }
                }
            }
        }
    }

    /// Side decision for each dependent (for explanation): which key level fired
    /// (4 — most exact … 0 — relation only; `None` — no key, default is right) and
    /// how many times in training such a dependent stood left / right.
    pub fn explain(&self, nodes: &[Node], q: bool) -> Vec<Side> {
        let n = nodes.len();
        let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for (i, nd) in nodes.iter().enumerate() {
            kids[nd.head.min(n)].push(i + 1);
        }
        let mut out = Vec::new();
        for h in 1..=n {
            let hx = nodes[h - 1].tag;
            let cons = construction(nodes, &kids[h]);
            for &d in &kids[h] {
                let keys = side_keys(&nodes[d - 1], hx, q, cons, heavy(nodes, &kids[d]));
                let hit = keys.iter().enumerate().find_map(|(i, k)| self.side.get(k).filter(|e| e.0 + e.1 >= MIN).map(|e| (4 - i as u8, *e)));
                out.push(Side { dep: d, head: h, left: hit.is_some_and(|(_, e)| e.0 > e.1), level: hit.map(|x| x.0), counts: hit.map_or((0, 0), |x| x.1) });
            }
        }
        out
    }

    fn left(&self, n: &Node, hx: Tag, q: bool, cons: (bool, bool), hv: bool) -> bool {
        for k in side_keys(n, hx, q, cons, hv) {
            if let Some(e) = self.side.get(&k) {
                if e.0 + e.1 >= MIN {
                    return e.0 > e.1;
                }
            }
        }
        false
    }

    /// Word order (numbers 1..n) for a tree with an explicit sentence-type bit (from the graph).
    pub fn order(&self, nodes: &[Node], q: bool) -> Vec<usize> {
        let n = nodes.len();
        let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for (i, nd) in nodes.iter().enumerate() {
            kids[nd.head.min(n)].push(i + 1);
        }
        let mut out = Vec::with_capacity(n);
        for &r in &kids[0] {
            self.emit(r, nodes, &kids, &mut out, 0, q);
        }
        out
    }

    fn emit(&self, h: usize, nodes: &[Node], kids: &[Vec<usize>], out: &mut Vec<usize>, depth: usize, q: bool) {
        if depth > 200 {
            out.push(h);
            return;
        }
        let hx = nodes[h - 1].tag;
        let cons = construction(nodes, &kids[h]);
        let (mut l, mut r): (Vec<usize>, Vec<usize>) = kids[h].iter().copied().partition(|&d| self.left(&nodes[d - 1], hx, q, cons, heavy(nodes, &kids[d])));
        self.sort_side(&mut l, true, nodes, q);
        self.sort_side(&mut r, false, nodes, q);
        for d in l {
            self.emit(d, nodes, kids, out, depth + 1, q);
        }
        out.push(h);
        for d in r {
            self.emit(d, nodes, kids, out, depth + 1, q);
        }
    }

    /// ln P(b | a) — interpolation of the three chain levels and the uniform distribution.
    fn lp(&self, left: bool, a: [Link; 3], b: [Link; 3]) -> f64 {
        const L: [f64; 3] = [0.5, 0.3, 0.15];
        let mut p = 0.05 / 64.0;
        for lv in 0..3u8 {
            let c = self.ctx.get(&(lv, left, a[lv as usize])).copied().unwrap_or(0.0);
            if c > 0.0 {
                let n = self.big.get(&(lv, left, a[lv as usize], b[lv as usize])).copied().unwrap_or(0.0);
                p += L[lv as usize] * n / c;
            }
        }
        p.ln()
    }

    /// Order of dependents on a side — the most probable chain sequence (dynamic programming over subsets
    /// up to 10 dependents; longer ones — greedily).
    fn sort_side(&self, ds: &mut Vec<usize>, left: bool, nodes: &[Node], q: bool) {
        let k = ds.len();
        if k < 2 {
            return;
        }
        let syms: Vec<[Link; 3]> = ds.iter().map(|&d| [0, 1, 2].map(|lv| link(&nodes[d - 1], lv, q))).collect();
        let (start, end) = if left { ([Link::Start; 3], [Link::Head; 3]) } else { ([Link::Head; 3], [Link::End; 3]) };
        if k > 10 {
            let mut rest: Vec<usize> = (0..k).collect();
            let mut prev = start;
            let mut out = Vec::with_capacity(k);
            while !rest.is_empty() {
                let (bi, _) = rest.iter().enumerate().map(|(j, &i)| (j, self.lp(left, prev, syms[i]))).max_by(|a, b| a.1.total_cmp(&b.1)).unwrap_or((0, 0.0));
                let i = rest.remove(bi);
                out.push(ds[i]);
                prev = syms[i];
            }
            *ds = out;
            return;
        }
        let full = (1usize << k) - 1;
        let mut dp = vec![vec![f64::NEG_INFINITY; k]; 1 << k];
        let mut back = vec![vec![usize::MAX; k]; 1 << k];
        for i in 0..k {
            dp[1 << i][i] = self.lp(left, start, syms[i]);
        }
        for mask in 1..=full {
            for last in 0..k {
                let cur = dp[mask][last];
                if cur == f64::NEG_INFINITY {
                    continue;
                }
                for nx in 0..k {
                    if mask & (1 << nx) != 0 {
                        continue;
                    }
                    let v = cur + self.lp(left, syms[last], syms[nx]);
                    let m2 = mask | (1 << nx);
                    if v > dp[m2][nx] {
                        dp[m2][nx] = v;
                        back[m2][nx] = last;
                    }
                }
            }
        }
        let mut last = (0..k).max_by(|&a, &b| (dp[full][a] + self.lp(left, syms[a], end)).total_cmp(&(dp[full][b] + self.lp(left, syms[b], end)))).unwrap_or(0);
        let (mut mask, mut order) = (full, Vec::with_capacity(k));
        loop {
            order.push(ds[last]);
            let prev = back[mask][last];
            mask &= !(1 << last);
            if prev == usize::MAX {
                break;
            }
            last = prev;
        }
        order.reverse();
        *ds = order;
    }
}

/// Punctuation from clause structure, for a generated tree that has none (the frames' realizer): the marks to add,
/// each as (head, mark, tag) with heads 1-based; the linearizer then orders them like any dependent. Rules after the
/// UD English conventions (a mark hangs on the phrase it introduces or closes):
/// - the root takes the final full stop;
/// - a coordinated clause with its own subject takes a comma before its conjunction («…, and she sat»), and in a list
///   of three or more every conjunct after the first takes one («A, B, and C»);
/// - a parataxis, a vocative, a discourse word take a comma;
/// - an apposition takes a comma before it («Bill, the lizard»; the closing one is left out: the linearizer cannot
///   tell two commas of one phrase apart and puts both before it).
///
/// With a learned table (`set_punct_table`), a word whose (relation, tag, head's tag) the table knows takes the marks
/// most often hung on such words in the training text; the rules stay for the words it does not know.
pub fn punctuate(tags: &[Tag], rels: &[Rel], heads: &[usize]) -> Vec<(usize, String, Tag)> {
    let n = tags.len().min(rels.len()).min(heads.len());
    let table = PUNCT.get();
    let known = |i: usize| table.and_then(|t| t.predict(&PunctTable::key(tags, rels, heads, i)));
    if table.is_some() {
        let mut out = Vec::new();
        for i in 0..n {
            match known(i) {
                Some(marks) => out.extend(marks.into_iter().map(|(m, t)| (i + 1, m, t))),
                None => out.extend(rule_marks(tags, rels, heads, i).into_iter().map(|(m, t)| (i + 1, m.to_string(), t))),
            }
        }
        return out;
    }
    (0..n).flat_map(|i| rule_marks(tags, rels, heads, i).into_iter().map(move |(m, t)| (i + 1, m.to_string(), t))).collect()
}

/// The rules' marks for word `i` (0-based).
fn rule_marks(tags: &[Tag], rels: &[Rel], heads: &[usize], i: usize) -> Vec<(&'static str, Tag)> {
    let n = tags.len().min(rels.len()).min(heads.len());
    let mut out: Vec<(&'static str, Tag)> = Vec::new();
    let kids = |h: usize| (0..n).filter(move |&k| heads[k] == h + 1);
    match rels[i].base() {
        Rel::Conj => {
            let h = heads[i];
            let clause = tags[i].is_verb() && kids(i).any(|k| matches!(rels[k].base(), Rel::Nsubj | Rel::Expl));
            let list = h > 0 && kids(h - 1).filter(|&k| rels[k].base() == Rel::Conj).count() >= 2;
            if clause || list {
                out.push((",", Tag::Comma));
            }
        }
        Rel::Parataxis | Rel::Vocative | Rel::Discourse | Rel::Appos => out.push((",", Tag::Comma)),
        _ => {}
    }
    if heads[i] == 0 {
        out.push((".", Tag::Stop));
    }
    out
}

/// Punctuation learned from parsed text (the UD convention of attaching a mark to the phrase it opens or closes:
/// demarneffe-2021): for a word's (relation, tag, head's tag), how often each set of marks hung
/// on it (`.`, `, ,`, `( )`, none …). Counted in a `BTreeMap`, so the table and its choices never depend on order.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PunctTable {
    by: BTreeMap<String, BTreeMap<String, u32>>,
}

/// The table `punctuate` uses, once set.
static PUNCT: std::sync::OnceLock<PunctTable> = std::sync::OnceLock::new();

/// Sets the learned punctuation for this process (the first call wins).
pub fn set_punct_table(t: PunctTable) {
    let _ = PUNCT.set(t);
}

impl PunctTable {
    /// The key of word `i` (0-based): its relation, its tag, its head's tag (`ROOT` for the root).
    fn key(tags: &[Tag], rels: &[Rel], heads: &[usize], i: usize) -> String {
        let h = heads[i];
        format!("{}\t{}\t{}", rels[i].name(), tags[i].name(), if h == 0 { "ROOT" } else { tags[h - 1].name() })
    }

    /// Counts the marks hung on every word of a parsed sentence (a mark: a word whose relation is `punct`).
    pub fn learn(&mut self, forms: &[&str], tags: &[Tag], rels: &[Rel], heads: &[usize]) {
        let n = forms.len().min(tags.len()).min(rels.len()).min(heads.len());
        for i in (0..n).filter(|&i| rels[i].base() != Rel::Punct) {
            let mut marks: Vec<String> = (0..n).filter(|&k| heads[k] == i + 1 && rels[k].base() == Rel::Punct).map(|k| format!("{}\u{1}{}", forms[k], tags[k].name())).collect();
            marks.sort();
            *self.by.entry(Self::key(tags, rels, heads, i)).or_default().entry(marks.join(" ")).or_default() += 1;
        }
    }

    /// The marks for a key: the most frequent set, when the key was seen at least 3 times and the set holds more than
    /// half of them; `Some(empty)` when that set is «no mark»; `None` when the table does not know.
    fn predict(&self, key: &str) -> Option<Vec<(String, Tag)>> {
        let m = self.by.get(key)?;
        let total: u32 = m.values().sum();
        let (sig, best) = m.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))?;
        if total < 3 || *best * 2 <= total {
            return None;
        }
        Some(sig.split(' ').filter(|x| !x.is_empty()).filter_map(|x| {
            let (f, t) = x.split_once('\u{1}')?;
            Some((f.to_string(), Tag::parse(t)?))
        }).collect())
    }

    /// As TSV: key (three columns), the marks, the count.
    pub fn write(&self) -> String {
        let mut o = String::new();
        for (k, m) in &self.by {
            for (sig, c) in m {
                o.push_str(&format!("{k}\t{}\t{c}\n", sig.replace('\u{1}', "\u{2}")));
            }
        }
        o
    }

    pub fn read(s: &str) -> PunctTable {
        let mut t = PunctTable::default();
        for l in s.lines() {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() != 5 {
                continue;
            }
            if let Ok(n) = c[4].parse::<u32>() {
                t.by.entry(format!("{}\t{}\t{}", c[0], c[1], c[2])).or_default().insert(c[3].replace('\u{2}', "\u{1}"), n);
            }
        }
        t
    }
}

#[cfg(test)]
mod punct_tests {
    use super::*;

    /// «the dog barked and she sat»: a comma before the coordinated clause and the full stop; the control without a
    /// subject in the second clause («barked and sat») gets no comma.
    #[test]
    fn commas_from_clause_structure() {
        let tags = [Tag::DT, Tag::NN, Tag::VBD, Tag::CC, Tag::PRP, Tag::VBD];
        let rels = [Rel::Det, Rel::Nsubj, Rel::Root, Rel::Cc, Rel::Nsubj, Rel::Conj];
        let heads = [2, 3, 0, 6, 6, 3];
        assert_eq!(punctuate(&tags, &rels, &heads), vec![(3, ".".to_string(), Tag::Stop), (6, ",".to_string(), Tag::Comma)]);
        let tags = [Tag::DT, Tag::NN, Tag::VBD, Tag::CC, Tag::VBD];
        let rels = [Rel::Det, Rel::Nsubj, Rel::Root, Rel::Cc, Rel::Conj];
        let heads = [2, 3, 0, 5, 3];
        assert_eq!(punctuate(&tags, &rels, &heads), vec![(3, ".".to_string(), Tag::Stop)]);
    }

    /// The table learns «Label: value» (a colon on an appos after a noun root), keeps the rules for what it has not
    /// seen, and needs a majority: a key seen 2:2 says nothing.
    #[test]
    fn learned_marks_and_their_limits() {
        let mut t = PunctTable::default();
        let (forms, tags, rels, heads) = (["Who", "acts", ":", "you"], [Tag::WP, Tag::NNS, Tag::Colon, Tag::PRP], [Rel::Compound, Rel::Root, Rel::Punct, Rel::Appos], [2, 0, 4, 2]);
        for _ in 0..3 {
            t.learn(&forms, &tags, &rels, &heads);
        }
        assert_eq!(t.predict(&PunctTable::key(&tags, &rels, &heads, 3)), Some(vec![(":".to_string(), Tag::Colon)]));
        // the root never had a mark here: the table says «none», not the rules' full stop
        assert_eq!(t.predict(&PunctTable::key(&tags, &rels, &heads, 1)), Some(vec![]));
        // the TSV keeps it
        assert_eq!(PunctTable::read(&t.write()), t);
        // negative control: a tie is not a majority
        let mut u = PunctTable::default();
        u.learn(&forms, &tags, &rels, &heads);
        u.learn(&forms, &tags, &rels, &heads);
        let other = [Rel::Compound, Rel::Root, Rel::Punct, Rel::Appos];
        u.learn(&["Who", "acts", ",", "you"], &[Tag::WP, Tag::NNS, Tag::Comma, Tag::PRP], &other, &heads);
        u.learn(&["Who", "acts", ",", "you"], &[Tag::WP, Tag::NNS, Tag::Comma, Tag::PRP], &other, &heads);
        assert_eq!(u.predict(&PunctTable::key(&tags, &rels, &heads, 3)), None);
    }
}

#[cfg(test)]
mod wave27 {
    use super::*;
    #[test]
    fn bounded_contract() {
        let m = Linearizer::default();
        let nodes = [
            Node {
                tag: Tag::NN,
                rel: Rel::Root,
                head: 0,
                lex: dict::w::THE,
            },
            Node {
                tag: Tag::DT,
                rel: Rel::Det,
                head: 1,
                lex: dict::w::THE,
            },
        ];
        assert_eq!(m.order(&nodes, false), vec![1, 2]);
        let s = m.explain(&nodes, false);
        assert_eq!((s[0].left, s[0].level, s[0].counts), (false, None, (0, 0)));
        assert!(m.order(&[], false).is_empty());
        assert!(!mood(&[], false));
        assert!(mood(&[], true));
    }
}
