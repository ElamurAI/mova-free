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
