//! The SLM builds dependency trees by logical derivation, step by step, as it solves arithmetic: every arc follows
//! from an attachment rule (an axiom) with a reason — "the → det of house: a DET attaches to the nearest NOUN to its
//! right". The rules are induced from trees, not written by hand.
//!
//!   world derive <train.conllu>... --test <test.conllu>... [--out <dir>] [--min 30] [--show N]
//!
//! A rule: dependent part of speech (and lemma for closed classes) → the k-th nearest word of part of speech Y to the
//! left or right (k ≤ 3), with the relation it most often has. Induction counts, for every rule, on how many words it
//! applies (the target exists) and how often it hits the gold head; each dependent class keeps a decision list sorted
//! by precision. Derivation: the root by the root rule (the first finite verb, else the first verb, noun, adjective),
//! then every word by the first applicable rule of its list; a word whose rule would close a cycle goes to the root
//! (stated as a fallback). Words are given with their parts of speech (as the numbers of a word problem are given).

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use en::gram::{Rel, UPos};

/// Closed classes keep their lemma in the rule ("of", "to", "will" behave differently).
fn closed(u: UPos) -> bool {
    matches!(u, UPos::ADP | UPos::AUX | UPos::PART | UPos::SCONJ | UPos::CCONJ | UPos::DET | UPos::PRON)
}

/// The dependent class of a word: its part of speech, with the lemma for closed classes, and the lemma of its
/// case/mark child when it has one ("NOUN+in", "VERB+that") — a noun with "in" attaches like an oblique.
fn class(upos: UPos, lemma: &str) -> String {
    if closed(upos) { format!("{}:{}", upos.name(), lemma.to_lowercase()) } else { upos.name().to_string() }
}

fn class_with(upos: UPos, lemma: &str, marker: Option<&str>) -> String {
    match marker {
        Some(m) if !closed(upos) => format!("{}+{}", upos.name(), m),
        _ => class(upos, lemma),
    }
}

/// The lemma of a case/mark child of word `i` in gold (training) — the marker that the derivation attaches first.
fn gold_marker(s: &S, i: usize) -> Option<String> {
    (0..s.upos.len()).find(|&j| s.head[j] == i + 1 && matches!(s.rel[j], Rel::Case | Rel::Mark)).map(|j| s.lemma[j].clone())
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rule {
    pub dep: String,
    /// +1 right, −1 left
    pub dir: i8,
    pub head: UPos,
    /// k-th nearest
    pub k: u8,
}

#[derive(Clone, Debug)]
pub struct Stat {
    pub applies: usize,
    pub hits: usize,
    pub rels: BTreeMap<Rel, usize>,
}

impl Stat {
    fn precision(&self) -> f64 {
        self.hits as f64 / self.applies.max(1) as f64
    }
    fn rel(&self) -> Rel {
        self.rels.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|x| *x.0).unwrap_or(Rel::Dep)
    }
}

impl Rule {
    pub fn show(&self, st: &Stat) -> String {
        let nth = ["", "nearest", "second nearest", "third nearest"][self.k as usize];
        format!("{} attaches to the {nth} {} to its {} as {} (precision {:.1}% of {})", self.dep, self.head.name(), if self.dir > 0 { "right" } else { "left" }, st.rel(), 100.0 * st.precision(), st.applies)
    }
}

/// A sentence for derivation: forms, lemmas, parts of speech, finite flags, gold heads and relations.
pub struct S {
    pub form: Vec<String>,
    pub lemma: Vec<String>,
    pub upos: Vec<UPos>,
    pub finite: Vec<bool>,
    pub head: Vec<usize>,
    pub rel: Vec<Rel>,
}

pub fn load(paths: &[PathBuf]) -> Result<Vec<S>> {
    let mut out = Vec::new();
    for p in paths {
        for s in en::conllu::read(p)? {
            if s.tokens.is_empty() {
                continue;
            }
            out.push(S {
                form: s.tokens.iter().map(|t| t.form.clone()).collect(),
                lemma: s.tokens.iter().map(|t| if t.lemma.is_empty() || t.lemma == "_" { t.form.to_lowercase() } else { t.lemma.to_lowercase() }).collect(),
                upos: s.tokens.iter().map(|t| t.upos.unwrap_or(UPos::X)).collect(),
                finite: s.tokens.iter().map(|t| t.feats.to_string().contains("VerbForm=Fin")).collect(),
                head: s.tokens.iter().map(|t| t.head).collect(),
                rel: s.tokens.iter().map(|t| t.rel).collect(),
            });
        }
    }
    Ok(out)
}

/// The k-th nearest word of part of speech `u` from `i` in direction `dir` (0-based index).
fn nth(s: &S, i: usize, dir: i8, u: UPos, k: u8) -> Option<usize> {
    let mut seen = 0u8;
    let mut j = i as isize;
    loop {
        j += dir as isize;
        if j < 0 || j as usize >= s.upos.len() {
            return None;
        }
        if s.upos[j as usize] == u {
            seen += 1;
            if seen == k {
                return Some(j as usize);
            }
        }
    }
}

const HEADS: [UPos; 6] = [UPos::NOUN, UPos::VERB, UPos::ADJ, UPos::PROPN, UPos::PRON, UPos::NUM];

/// Is word `j` still a phrase head when `i` attaches (bottom-up: material whose gold head lies strictly inside the
/// span between `i` and the target has been attached already)?
fn pending(s: &S, i: usize, j: usize, lo: usize, hi: usize) -> bool {
    let h = s.head[j];
    j != i && !(h > 0 && h - 1 > lo && h - 1 < hi && h - 1 != i)
}

/// The k-th nearest pending word of part of speech `u` from `i` (training, gold attachments).
fn nth_pending(s: &S, i: usize, dir: i8, u: UPos, k: u8, far: usize) -> Option<usize> {
    let mut seen = 0u8;
    let mut j = i as isize;
    loop {
        j += dir as isize;
        if j < 0 || j as usize >= s.upos.len() {
            return None;
        }
        let ju = j as usize;
        let (lo, hi) = if dir > 0 { (i, far.max(ju)) } else { (far.min(ju), i) };
        if s.upos[ju] == u && pending(s, i, ju, lo, hi) {
            seen += 1;
            if seen == k {
                return Some(ju);
            }
        }
    }
}

/// Induce rule statistics from gold trees.
pub fn induce(data: &[S]) -> BTreeMap<Rule, Stat> {
    induce_mode(data, false)
}

/// `easy`: classes with markers and pending-word counting (for `derive_easy`); otherwise plain classes and all words.
pub fn induce_mode(data: &[S], easy: bool) -> BTreeMap<Rule, Stat> {
    let mut st: BTreeMap<Rule, Stat> = BTreeMap::new();
    for s in data {
        for i in 0..s.upos.len() {
            if s.head[i] == 0 || s.upos[i] == UPos::PUNCT {
                continue;
            }
            let dep = if easy { class_with(s.upos[i], &s.lemma[i], gold_marker(s, i).as_deref()) } else { class(s.upos[i], &s.lemma[i]) };
            let gh = s.head[i] - 1;
            for dir in [-1i8, 1] {
                for &u in &HEADS {
                    for k in 1..=3u8 {
                        let Some(j) = (if easy { nth_pending(s, i, dir, u, k, gh) } else { nth(s, i, dir, u, k) }) else { break };
                        let e = st.entry(Rule { dep: dep.clone(), dir, head: u, k }).or_insert(Stat { applies: 0, hits: 0, rels: BTreeMap::new() });
                        e.applies += 1;
                        if j + 1 == s.head[i] {
                            e.hits += 1;
                            *e.rels.entry(s.rel[i]).or_default() += 1;
                        }
                    }
                }
            }
        }
    }
    st
}

/// Decision lists: dependent class → rules sorted by precision (rules with ≥ `min` applications and ≥ 1 hit);
/// a class unseen in training falls back to its part of speech (`VERB`, `NOUN` …) list.
pub fn lists(st: &BTreeMap<Rule, Stat>, min: usize, hidden: &[Rule]) -> BTreeMap<String, Vec<(Rule, Stat)>> {
    let mut out: BTreeMap<String, Vec<(Rule, Stat)>> = BTreeMap::new();
    for (r, s) in st {
        if s.applies >= min && s.hits > 0 && !hidden.contains(r) {
            out.entry(r.dep.clone()).or_default().push((r.clone(), s.clone()));
        }
    }
    for v in out.values_mut() {
        v.sort_by(|a, b| b.1.precision().total_cmp(&a.1.precision()).then(b.1.applies.cmp(&a.1.applies)).then(a.0.cmp(&b.0)));
    }
    out
}

/// One derivation step: word, head, relation, reason.
pub struct Step {
    pub word: usize,
    pub head: usize,
    pub rel: Rel,
    pub why: String,
}

/// Tried 02.10: easy-first derivation — on EWT test UAS 53.5 against 56.6 of the plain decision lists (`derive`);
/// kept for comparison.
/// Derive a tree, easy steps first (Goldberg & Elhadad 2010, easy-first): among the words not yet attached, the
/// one whose best applicable rule has the highest precision attaches; attached words hide inside their head's
/// phrase, "nearest" counts only phrase heads, and a word's class includes the case/mark child it has got.
/// Returns heads (1-based, 0 root), relations, and the steps with reasons, in the order they were taken.
pub fn derive_easy(s: &S, l: &BTreeMap<String, Vec<(Rule, Stat)>>) -> (Vec<usize>, Vec<Rel>, Vec<Step>) {
    let n = s.upos.len();
    let mut head = vec![usize::MAX; n];
    let mut rel = vec![Rel::Dep; n];
    let mut steps = Vec::new();
    let root = (0..n)
        .find(|&i| s.upos[i] == UPos::VERB && s.finite[i])
        .or_else(|| (0..n).find(|&i| s.upos[i] == UPos::VERB))
        .or_else(|| (0..n).find(|&i| matches!(s.upos[i], UPos::NOUN | UPos::PROPN | UPos::ADJ)))
        .unwrap_or(0);
    head[root] = 0;
    rel[root] = Rel::Root;
    steps.push(Step { word: root, head: 0, rel: Rel::Root, why: "root rule: the first finite verb (else verb, noun, adjective)".into() });
    for i in 0..n {
        if s.upos[i] == UPos::PUNCT && i != root {
            head[i] = root + 1;
            rel[i] = Rel::Punct;
        }
    }
    let mut marker: Vec<Option<String>> = vec![None; n];
    let is_pending = |head: &[usize], j: usize| head[j] == usize::MAX || head[j] == 0;
    loop {
        // best (precision, word, target, rule) over unattached words
        let mut best: Option<(f64, usize, usize, &Rule, &Stat)> = None;
        for i in 0..n {
            if head[i] != usize::MAX {
                continue;
            }
            let c = class_with(s.upos[i], &s.lemma[i], marker[i].as_deref());
            let Some(list) = l.get(&c).or_else(|| l.get(&class(s.upos[i], &s.lemma[i]))).or_else(|| l.get(s.upos[i].name())) else { continue };
            for (r, st) in list {
                // the k-th nearest pending word of the rule's part of speech
                let mut seen = 0u8;
                let mut j = i as isize;
                let target = loop {
                    j += r.dir as isize;
                    if j < 0 || j as usize >= n {
                        break None;
                    }
                    let ju = j as usize;
                    if s.upos[ju] == r.head && is_pending(&head, ju) {
                        seen += 1;
                        if seen == r.k {
                            break Some(ju);
                        }
                    }
                };
                if let Some(t) = target {
                    if best.as_ref().is_none_or(|b| st.precision() > b.0) {
                        best = Some((st.precision(), i, t, r, st));
                    }
                    break;
                }
            }
        }
        let Some((_, i, t, r, st)) = best else { break };
        head[i] = t + 1;
        rel[i] = st.rel();
        if matches!(st.rel(), Rel::Case | Rel::Mark) && marker[t].is_none() {
            marker[t] = Some(s.lemma[i].clone());
        }
        steps.push(Step { word: i, head: t + 1, rel: st.rel(), why: r.show(st) });
    }
    for i in 0..n {
        if head[i] == usize::MAX {
            head[i] = root + 1;
            steps.push(Step { word: i, head: root + 1, rel: Rel::Dep, why: "no rule applies: fallback to the root".into() });
        }
    }
    (head, rel, steps)
}

/// Derive a tree with plain decision lists: the root by the root rule, then every word by the first applicable rule
/// of its class (rules sorted by precision); a rule that would close a cycle is skipped; no rule — the root.
pub fn derive(s: &S, l: &BTreeMap<String, Vec<(Rule, Stat)>>) -> (Vec<usize>, Vec<Rel>, Vec<Step>) {
    let n = s.upos.len();
    let mut head = vec![usize::MAX; n];
    let mut rel = vec![Rel::Dep; n];
    let mut steps = Vec::new();
    let root = (0..n)
        .find(|&i| s.upos[i] == UPos::VERB && s.finite[i])
        .or_else(|| (0..n).find(|&i| s.upos[i] == UPos::VERB))
        .or_else(|| (0..n).find(|&i| matches!(s.upos[i], UPos::NOUN | UPos::PROPN | UPos::ADJ)))
        .unwrap_or(0);
    head[root] = 0;
    rel[root] = Rel::Root;
    steps.push(Step { word: root, head: 0, rel: Rel::Root, why: "root rule: the first finite verb (else verb, noun, adjective)".into() });
    let cycle = |head: &[usize], from: usize, to: usize| -> bool {
        let mut x = to;
        for _ in 0..=n {
            if x == from {
                return true;
            }
            match head[x] {
                usize::MAX | 0 => return false,
                h => x = h - 1,
            }
        }
        true
    };
    for i in 0..n {
        if i == root {
            continue;
        }
        if s.upos[i] == UPos::PUNCT {
            head[i] = root + 1;
            rel[i] = Rel::Punct;
            steps.push(Step { word: i, head: root + 1, rel: Rel::Punct, why: "punctuation attaches to the root".into() });
            continue;
        }
        let c = class(s.upos[i], &s.lemma[i]);
        let mut done = false;
        if let Some(list) = l.get(&c).or_else(|| l.get(s.upos[i].name())) {
            for (r, st) in list {
                if let Some(j) = nth(s, i, r.dir, r.head, r.k) {
                    if cycle(&head, i, j) {
                        continue;
                    }
                    head[i] = j + 1;
                    rel[i] = st.rel();
                    steps.push(Step { word: i, head: j + 1, rel: st.rel(), why: r.show(st) });
                    done = true;
                    break;
                }
            }
        }
        if !done {
            head[i] = root + 1;
            steps.push(Step { word: i, head: root + 1, rel: Rel::Dep, why: "no rule applies: fallback to the root".into() });
        }
    }
    for i in 0..n {
        if head[i] != 0 && head[i] != usize::MAX && cycle(&head, i, head[i] - 1) {
            head[i] = root + 1;
        }
    }
    (head, rel, steps)
}

fn score(data: &[S], l: &BTreeMap<String, Vec<(Rule, Stat)>>) -> (f64, f64) {
    let (mut u, mut lb, mut n) = (0usize, 0usize, 0usize);
    for s in data {
        let (h, r, _) = derive(s, l);
        for i in 0..s.upos.len() {
            n += 1;
            if h[i] == s.head[i] {
                u += 1;
                lb += (r[i] == s.rel[i]) as usize;
            }
        }
    }
    (100.0 * u as f64 / n.max(1) as f64, 100.0 * lb as f64 / n.max(1) as f64)
}

/// `world derive`.
pub fn run(train: &[PathBuf], test: &[PathBuf], out: Option<&Path>, min: usize, show: usize) -> Result<()> {
    let tr = load(train)?;
    let st = induce(&tr);
    let l = lists(&st, min, &[]);
    let nrules: usize = l.values().map(Vec::len).sum();
    println!("train sentences {}  rules {nrules} in {} dependent classes", tr.len(), l.len());
    let (u, lb) = score(&tr, &l);
    println!("train UAS {u:.2} LAS {lb:.2}");
    for p in test {
        let ts = load(std::slice::from_ref(p))?;
        let (u, lb) = score(&ts, &l);
        println!("{}: UAS {u:.2} LAS {lb:.2}", p.display());
        for s in ts.iter().take(show) {
            let (h, _, steps) = derive(s, &l);
            println!("  {}", s.form.join(" "));
            for st in steps {
                let hd = if st.head == 0 { "ROOT".to_string() } else { s.form[st.head - 1].clone() };
                println!("    {} → {} {}{}: {}", s.form[st.word], st.rel, hd, if h[st.word] == s.head[st.word] { "" } else { " ✗" }, st.why);
            }
        }
    }
    if let Some(o) = out {
        std::fs::create_dir_all(o)?;
        let mut f = std::io::BufWriter::new(std::fs::File::create(o.join("rules.tsv"))?);
        writeln!(f, "dep\tdir\thead\tk\tapplies\thits\tprecision\trel\treading")?;
        for (c, v) in &l {
            for (r, s) in v {
                writeln!(f, "{c}\t{}\t{}\t{}\t{}\t{}\t{:.4}\t{}\t{}", r.dir, r.head.name(), r.k, s.applies, s.hits, s.precision(), s.rel(), r.show(s))?;
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------
// Learning to derive the parser's trees: transformation rules on top of the decision lists (Brill-style), trained on
// the SLM parser's own output for random book sentences (no gold): the logical deriver learns to reach the same
// trees, every transformation a readable rule.

/// A condition atom of a word in the current derived tree.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cond {
    Class(String),
    HeadPos(String),
    HeadSide(i8),
    Prev(String),
    Next(String),
    Rel(Rel),
}

impl Cond {
    fn show(&self) -> String {
        match self {
            Cond::Class(c) => format!("word is {c}"),
            Cond::HeadPos(u) => format!("its head is {u}"),
            Cond::HeadSide(d) => format!("its head is to the {}", if *d > 0 { "right" } else { "left" }),
            Cond::Prev(u) => format!("previous word is {u}"),
            Cond::Next(u) => format!("next word is {u}"),
            Cond::Rel(r) => format!("it is {r}"),
        }
    }
}

/// A transformation: if all conditions hold, reattach to the k-th nearest `head` in direction `dir` as `rel`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tr {
    pub conds: Vec<Cond>,
    pub dir: i8,
    pub head: UPos,
    pub k: u8,
    pub rel: Rel,
}

impl Tr {
    pub fn show(&self) -> String {
        let nth = ["", "nearest", "second nearest", "third nearest"][self.k as usize];
        format!("IF {} THEN attach to the {nth} {} to the {} as {}", self.conds.iter().map(Cond::show).collect::<Vec<_>>().join(" AND "), self.head.name(), if self.dir > 0 { "right" } else { "left" }, self.rel)
    }
}

fn conds(s: &S, head: &[usize], rel: &[Rel], i: usize) -> Vec<Cond> {
    let mut c = vec![Cond::Class(class(s.upos[i], &s.lemma[i]))];
    let h = head[i];
    c.push(Cond::HeadPos(if h == 0 { "ROOT".into() } else { s.upos[h - 1].name().to_string() }));
    if h > 0 {
        c.push(Cond::HeadSide(if h - 1 > i { 1 } else { -1 }));
    }
    c.push(Cond::Prev(if i == 0 { "START".into() } else { s.upos[i - 1].name().to_string() }));
    c.push(Cond::Next(s.upos.get(i + 1).map_or("END".to_string(), |u| u.name().to_string())));
    c.push(Cond::Rel(rel[i]));
    c
}

/// Condition sets: the class plus up to two more atoms.
fn cond_sets(c: &[Cond]) -> Vec<Vec<Cond>> {
    let mut out = vec![vec![c[0].clone()]];
    for a in 1..c.len() {
        out.push(vec![c[0].clone(), c[a].clone()]);
        for b in a + 1..c.len() {
            out.push(vec![c[0].clone(), c[a].clone(), c[b].clone()]);
        }
    }
    out
}

fn makes_cycle(head: &[usize], from: usize, to: usize) -> bool {
    let mut x = to;
    for _ in 0..=head.len() {
        if x == from {
            return true;
        }
        match head[x] {
            0 => return false,
            h => x = h - 1,
        }
    }
    true
}

/// Learn transformations: greedy, best `fixed − broken` with precision ≥ 0.8 and ≥ `min` fixes, up to `iters`.
pub fn learn(data: &[S], cur: &mut [(Vec<usize>, Vec<Rel>)], iters: usize, min: usize) -> Vec<(Tr, usize, usize)> {
    use std::collections::HashMap;
    let mut out = Vec::new();
    for _ in 0..iters {
        let mut fix: HashMap<Tr, usize> = HashMap::new();
        for (s, (h, r)) in data.iter().zip(cur.iter()) {
            for i in 0..s.upos.len() {
                if s.head[i] == h[i] || s.head[i] == 0 {
                    continue;
                }
                let gt = s.head[i] - 1;
                let gu = s.upos[gt];
                let dir: i8 = if gt > i { 1 } else { -1 };
                let Some(k) = (1..=3u8).find(|&k| nth(s, i, dir, gu, k) == Some(gt)) else { continue };
                // a move that would close a cycle is not applied, so it fixes nothing (tried 02.10: without this check
                // one rule was chosen twenty times in a row with 1150 "fixes" that never happened)
                if makes_cycle(h, i, gt) {
                    continue;
                }
                for cs in cond_sets(&conds(s, h, r, i)) {
                    *fix.entry(Tr { conds: cs, dir, head: gu, k, rel: s.rel[i] }).or_default() += 1;
                }
            }
        }
        fix.retain(|_, n| *n >= min);
        if fix.is_empty() {
            break;
        }
        let mut by_conds: HashMap<Vec<Cond>, Vec<Tr>> = HashMap::new();
        for t in fix.keys() {
            by_conds.entry(t.conds.clone()).or_default().push(t.clone());
        }
        let mut brk: HashMap<&Tr, usize> = HashMap::new();
        for (s, (h, r)) in data.iter().zip(cur.iter()) {
            for i in 0..s.upos.len() {
                if s.head[i] != h[i] || h[i] == 0 {
                    continue;
                }
                for cs in cond_sets(&conds(s, h, r, i)) {
                    if let Some(ts) = by_conds.get(&cs) {
                        for t in ts {
                            if nth(s, i, t.dir, t.head, t.k).is_some_and(|j| j + 1 != h[i]) {
                                *brk.entry(fix.get_key_value(t).unwrap().0).or_default() += 1;
                            }
                        }
                    }
                }
            }
        }
        let mut cands: Vec<(Tr, usize, usize)> = fix.iter().map(|(t, &f)| (t.clone(), f, brk.get(t).copied().unwrap_or(0))).collect();
        cands.sort_by(|a, b| (b.1 as i64 - b.2 as i64).cmp(&(a.1 as i64 - a.2 as i64)).then(a.0.conds.len().cmp(&b.0.conds.len())).then(a.0.cmp(&b.0)));
        let Some((t, f, b)) = cands.into_iter().find(|(t, f, b)| *f as f64 / (*f + *b) as f64 >= 0.8 && !out.iter().any(|(o, _, _): &(Tr, usize, usize)| o == t)) else { break };
        if f < min + b {
            break;
        }
        let mut moved = 0usize;
        for (s, (h, r)) in data.iter().zip(cur.iter_mut()) {
            let before: Vec<usize> = h.clone();
            apply_tr(s, h, r, &t);
            moved += before.iter().zip(h.iter()).filter(|(a, b)| a != b).count();
        }
        if moved == 0 {
            break;
        }
        out.push((t, f, b));
    }
    out
}

/// Apply one transformation to a derived tree (skipping moves that would close a cycle).
pub fn apply_tr(s: &S, h: &mut [usize], r: &mut [Rel], t: &Tr) {
    let moves: Vec<(usize, usize)> = (0..s.upos.len())
        .filter(|&i| h[i] != 0)
        .filter(|&i| {
            let c = conds(s, h, r, i);
            t.conds.iter().all(|x| c.contains(x))
        })
        .filter_map(|i| nth(s, i, t.dir, t.head, t.k).map(|j| (i, j)))
        .collect();
    for (i, j) in moves {
        if !makes_cycle(h, i, j) {
            h[i] = j + 1;
            r[i] = t.rel;
        }
    }
}

/// Sentences parsed by the SLM (the teacher): random book sentences (every 7th, ≤ 20 per book), annotated and
/// repaired by the working pipeline; heads and relations are the parser's.
pub fn from_books(list: &Path, n: usize) -> Result<Vec<S>> {
    let a = crate::tree::annotator()?;
    let mut out = Vec::new();
    for b in std::fs::read_to_string(list)?.lines() {
        if out.len() >= n {
            break;
        }
        let Ok(t) = std::fs::read_to_string(b) else { continue };
        for sent in crate::events::sentences(crate::events::book_body(&t)).into_iter().step_by(7).take(20) {
            let forms: Vec<String> = a.tokenize(&sent).into_iter().map(|t| t.form).collect();
            if forms.is_empty() {
                continue;
            }
            let mut ws = a.annotate(&forms);
            crate::rerank::repair_words(&mut ws);
            out.push(words_to_s(&ws, None));
        }
    }
    out.truncate(n);
    Ok(out)
}

fn words_to_s(ws: &[en::annotate::Word], gold: Option<&[(usize, Rel)]>) -> S {
    S {
        form: ws.iter().map(|w| w.form.clone()).collect(),
        lemma: ws.iter().map(|w| w.lemma.to_lowercase()).collect(),
        upos: ws.iter().map(|w| w.upos).collect(),
        finite: ws.iter().map(|w| w.feats.to_string().contains("VerbForm=Fin")).collect(),
        head: gold.map_or_else(|| ws.iter().map(|w| w.head).collect(), |g| g.iter().map(|x| x.0).collect()),
        rel: gold.map_or_else(|| ws.iter().map(|w| w.rel).collect(), |g| g.iter().map(|x| x.1).collect()),
    }
}

/// Gold trees with the SLM's own tags (as at work: parts of speech come from the tagger, heads from gold).
pub fn gold_with_tags(paths: &[PathBuf]) -> Result<Vec<S>> {
    let a = crate::tree::annotator()?;
    let mut out = Vec::new();
    for p in paths {
        for s in en::conllu::read(p)? {
            let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            if forms.is_empty() {
                continue;
            }
            let ws = a.annotate(&forms);
            let g: Vec<(usize, Rel)> = s.tokens.iter().map(|t| (t.head, t.rel)).collect();
            out.push(words_to_s(&ws, Some(&g)));
        }
    }
    Ok(out)
}

fn agree(data: &[S], cur: &[(Vec<usize>, Vec<Rel>)]) -> (f64, f64) {
    let (mut u, mut l, mut n) = (0usize, 0usize, 0usize);
    for (s, (h, r)) in data.iter().zip(cur) {
        for i in 0..s.upos.len() {
            n += 1;
            if h[i] == s.head[i] {
                u += 1;
                l += (r[i] == s.rel[i]) as usize;
            }
        }
    }
    (100.0 * u as f64 / n.max(1) as f64, 100.0 * l as f64 / n.max(1) as f64)
}

/// `world derive-learn <book-list> [--sentences N] [--iters K] [--min M] --test <gold.conllu>...`: decision lists
/// and transformations learned from the SLM parser's trees of random book sentences; agreement with the parser on
/// held-out book sentences, and UAS/LAS against gold (with the SLM's tags).
pub fn run_learn(list: &Path, n: usize, iters: usize, min: usize, test: &[PathBuf], out: Option<&Path>) -> Result<()> {
    let all = from_books(list, n)?;
    let cut = all.len() * 9 / 10;
    let (tr, held) = all.split_at(cut);
    let st = induce(tr);
    let l = lists(&st, 30, &[]);
    let mut cur: Vec<(Vec<usize>, Vec<Rel>)> = tr.iter().map(|s| { let (h, r, _) = derive(s, &l); (h, r) }).collect();
    println!("train {} parser trees (book sentences), held-out {}; decision lists: {} rules", tr.len(), held.len(), l.values().map(Vec::len).sum::<usize>());
    let (u0, l0) = agree(tr, &cur);
    println!("train agreement with the parser: decision lists UAS {u0:.2} LAS {l0:.2}");
    let t_learn = std::time::Instant::now();
    let trs = if std::env::var("DERIVE_FULL_RESCORE").is_ok() { learn(tr, &mut cur, iters, min) } else { learn_incremental(tr, &mut cur, iters, min) };
    println!("learning: {:.1} s", t_learn.elapsed().as_secs_f64());
    let (u1, l1) = agree(tr, &cur);
    println!("train agreement after {} transformations: UAS {u1:.2} LAS {l1:.2}", trs.len());
    for (t, f, b) in trs.iter().take(25) {
        println!("  fixed {f:5} broken {b:4}  {}", t.show());
    }
    let eval = |name: &str, data: &[S]| {
        let mut c: Vec<(Vec<usize>, Vec<Rel>)> = data.iter().map(|s| { let (h, r, _) = derive(s, &l); (h, r) }).collect();
        let (a0, b0) = agree(data, &c);
        for (s, (h, r)) in data.iter().zip(c.iter_mut()) {
            for (t, _, _) in &trs {
                apply_tr(s, h, r, t);
            }
        }
        let (a1, b1) = agree(data, &c);
        println!("{name}: decision lists UAS {a0:.2} LAS {b0:.2} → with transformations UAS {a1:.2} LAS {b1:.2}");
    };
    eval("held-out book sentences, agreement with the parser", held);
    for p in test {
        eval(&format!("{} (gold, SLM tags)", p.display()), &gold_with_tags(std::slice::from_ref(p))?);
    }
    if let Some(o) = out {
        std::fs::create_dir_all(o)?;
        let mut f = std::io::BufWriter::new(std::fs::File::create(o.join("transformations.tsv"))?);
        writeln!(f, "fixed\tbroken\trule")?;
        for (t, fx, b) in &trs {
            writeln!(f, "{fx}\t{b}\t{}", t.show())?;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------
// Scan of the whole corpus: the parser's trees of millions of book sentences, cached compactly; the SLM sees what it
// already derives with its rules (agreement by word class, the biggest disagreements) and learns in rounds — two
// parallel passes per round (fixes of candidates, then breaks of the best), several non-interfering rules per round.

/// One cached sentence: parts of speech, classes (interned), finite flags, the parser's heads and relations.
#[derive(Clone)]
struct Packed {
    upos: Vec<u8>,
    class: Vec<u16>,
    finite: Vec<bool>,
    head: Vec<u16>,
    rel: Vec<u8>,
}

fn unpack(p: &Packed, classes: &[String]) -> S {
    S {
        form: vec![String::new(); p.upos.len()],
        // the class string carries the lemma of closed classes ("ADP:of"); open classes need none
        lemma: p.class.iter().map(|&c| classes[c as usize].split(':').nth(1).unwrap_or("").to_string()).collect(),
        upos: p.upos.iter().map(|&u| UPos::ALL[u as usize]).collect(),
        finite: p.finite.clone(),
        head: p.head.iter().map(|&h| h as usize).collect(),
        rel: p.rel.iter().map(|&r| Rel::ALL[r as usize]).collect(),
    }
}

/// Packed parser trees from the tree store (`world store`), selected by a query (`prefix=1%`, `subj>=1 …`) in
/// the latest version — no parsing again.
pub fn scan_store(dir: &Path, query: &str, rounds: usize, per_round: usize, threads: usize, out: &Path) -> Result<()> {
    let st = crate::store::Store::open(dir)?;
    let hash = st.latest().context("empty store")?.to_string();
    let ids = crate::store::select(dir, query, &hash)?;
    let lemmas = crate::store::lemma_table(dir);
    let mut classes_m: BTreeMap<String, u16> = BTreeMap::new();
    let mut data = Vec::with_capacity(ids.len());
    for &i in &ids {
        let recs = crate::store::raw_tree(&st, &hash, i as usize)?;
        let mut cls = Vec::with_capacity(recs.len());
        let mut finite = Vec::with_capacity(recs.len());
        for r in &recs {
            let (u, _, _, l) = crate::store::decode(r);
            let name = class(u, lemmas.get(l as usize).map(String::as_str).unwrap_or(""));
            let n = classes_m.len() as u16;
            cls.push(*classes_m.entry(name).or_insert(n));
            finite.push(matches!(crate::store::decode_tag(r), en::gram::Tag::VBD | en::gram::Tag::VBZ | en::gram::Tag::VBP | en::gram::Tag::MD));
        }
        data.push(Packed { upos: recs.iter().map(|r| r[0]).collect(), class: cls, finite, head: recs.iter().map(|r| u16::from_le_bytes([r[2], r[3]])).collect(), rel: recs.iter().map(|r| r[1]).collect() });
    }
    let mut classes = vec![String::new(); classes_m.len()];
    for (k, i) in classes_m {
        classes[i as usize] = k;
    }
    println!("store {} model {hash}: «{query}» → {} sentences", dir.display(), data.len());
    std::fs::create_dir_all(out)?;
    scan_rounds(data, classes, rounds, per_round, threads, out)
}

/// `world derive-scan <book-list> [--rounds R] [--per-round K] [--threads T] [--limit N] [--out <dir>]`.
pub fn scan(list: &Path, rounds: usize, per_round: usize, threads: usize, limit: usize, out: &Path) -> Result<()> {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let a = crate::tree::annotator()?;
    let books: Vec<String> = std::fs::read_to_string(list)?.lines().map(String::from).collect();
    let classes: Mutex<BTreeMap<String, u16>> = Mutex::new(BTreeMap::new());
    let cache: Mutex<Vec<Packed>> = Mutex::new(Vec::new());
    let next = AtomicUsize::new(0);
    let count = AtomicUsize::new(0);
    let t0 = std::time::Instant::now();
    // pass 0: parse every sentence once (the working pipeline), pack
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::SeqCst);
                let Some(b) = books.get(k) else { break };
                if count.load(Ordering::SeqCst) >= limit {
                    break;
                }
                let Ok(t) = std::fs::read_to_string(b) else { continue };
                let mut local = Vec::new();
                for sent in crate::events::sentences(crate::events::book_body(&t)) {
                    let forms: Vec<String> = a.tokenize(&sent).into_iter().map(|t| t.form).collect();
                    if forms.is_empty() || forms.len() > 60 {
                        continue;
                    }
                    let mut ws = a.annotate(&forms);
                    crate::rerank::repair_words(&mut ws);
                    let cls: Vec<u16> = {
                        let mut c = classes.lock().unwrap();
                        ws.iter().map(|w| {
                            let name = class(w.upos, &w.lemma);
                            let n = c.len() as u16;
                            *c.entry(name).or_insert(n)
                        }).collect()
                    };
                    local.push(Packed {
                        upos: ws.iter().map(|w| UPos::ALL.iter().position(|u| *u == w.upos).unwrap_or(0) as u8).collect(),
                        class: cls,
                        finite: ws.iter().map(|w| w.feats.to_string().contains("VerbForm=Fin")).collect(),
                        head: ws.iter().map(|w| w.head as u16).collect(),
                        rel: ws.iter().map(|w| Rel::ALL.iter().position(|r| *r == w.rel).unwrap_or(0) as u8).collect(),
                    });
                }
                let n = count.fetch_add(local.len(), Ordering::SeqCst) + local.len();
                cache.lock().unwrap().extend(local);
                if k % 100 == 0 {
                    eprintln!("[{k}/{}] books, {n} sentences, {:.0} s", books.len(), t0.elapsed().as_secs_f64());
                }
            });
        }
    });
    let classes: Vec<String> = {
        let c = classes.into_inner().unwrap();
        let mut v = vec![String::new(); c.len()];
        for (k, i) in c {
            v[i as usize] = k;
        }
        v
    };
    let data = cache.into_inner().unwrap();
    scan_rounds(data, classes, rounds, per_round, threads, out)
}

/// The learning rounds over packed parser trees (from parsing books or from the tree store); the report and the
/// learned rules are written after every round, so a stopped run keeps what it learned.
fn scan_rounds(data: Vec<Packed>, classes: Vec<String>, rounds: usize, per_round: usize, threads: usize, out: &Path) -> Result<()> {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let t0 = std::time::Instant::now();
    // a training stage: the scheme is frozen while it runs; a `stop-gain: G` line in the memory cells (knowledge the
    // SLM wrote for itself) stops the stage early when a round gains less than G points
    let mut stage = crate::state::begin_stage("derive-scan")?;
    let stop_gain: Option<f64> = crate::state::memory_value("stop-gain").and_then(|v| v.parse().ok());
    let mut last_uas: Option<f64> = None;
    // the brain steers the direction: the last decision letter of this kind of stage — "still growing" asks for more
    // rounds, "flattened" for a shorter stage (the next idea is another rule form, not more of the same)
    let rounds = match crate::state::letter_last().ok().flatten().map(|(_, t, _)| t) {
        Some(t) if t.starts_with("Stage derive-scan") && t.contains("still growing") => {
            println!("my last decision said the curve was still growing: {} rounds instead of {rounds}", rounds + 5);
            rounds + 5
        }
        Some(t) if t.starts_with("Stage derive-scan") && t.contains("flattened") => {
            println!("my last decision said the curve had flattened: a short stage of {} rounds", rounds.min(2));
            rounds.min(2)
        }
        _ => rounds,
    };
    let words: usize = data.iter().map(|p| p.upos.len()).sum();
    println!("cached {} sentences, {words} words, {} classes, {:.0} s", data.len(), classes.len(), t0.elapsed().as_secs_f64());
    // decision lists from a sample of the parser trees (every 20th sentence)
    let sample: Vec<S> = data.iter().step_by(20).map(|p| unpack(p, &classes)).collect();
    let l = lists(&induce(&sample), 30, &[]);
    drop(sample);
    std::fs::create_dir_all(out)?;
    let mut trs: Vec<Tr> = Vec::new();
    // current derived trees are recomputed per pass from the rules (lists + transformations) — no extra memory
    let derive_all = |p: &Packed, trs: &[Tr]| -> (S, Vec<usize>, Vec<Rel>) {
        let s = unpack(p, &classes);
        let (mut h, mut r, _) = derive(&s, &l);
        for t in trs {
            apply_tr(&s, &mut h, &mut r, t);
        }
        (s, h, r)
    };
    let par = |f: &(dyn Fn(&Packed) + Sync)| {
        let next = AtomicUsize::new(0);
        std::thread::scope(|sc| {
            for _ in 0..threads.max(1) {
                sc.spawn(|| loop {
                    let k = next.fetch_add(4096, Ordering::SeqCst);
                    if k >= data.len() {
                        break;
                    }
                    for p in &data[k..(k + 4096).min(data.len())] {
                        f(p);
                    }
                });
            }
        });
    };
    let mut report = String::new();
    let save = |report: &str, trs: &[Tr]| -> Result<()> {
        std::fs::write(out.join("report.txt"), report)?;
        std::fs::write(out.join("transformations.tsv"), trs.iter().map(|t| format!("{}\n", t.show())).collect::<String>())?;
        Ok(())
    };
    for round in 0..=rounds {
        // what it already derives: agreement with the parser, by word class (part of speech)
        let agree_by: Mutex<BTreeMap<String, (usize, usize)>> = Mutex::new(BTreeMap::new());
        let total = Mutex::new((0usize, 0usize));
        par(&|p| {
            let (s, h, _) = derive_all(p, &trs);
            let mut loc: BTreeMap<String, (usize, usize)> = BTreeMap::new();
            let mut t = (0usize, 0usize);
            for i in 0..s.upos.len() {
                let e = loc.entry(s.upos[i].name().to_string()).or_default();
                e.0 += 1;
                let ok = h[i] == s.head[i];
                e.1 += ok as usize;
                t.0 += 1;
                t.1 += ok as usize;
            }
            let mut g = agree_by.lock().unwrap();
            for (k, (n, c)) in loc {
                let e = g.entry(k).or_default();
                e.0 += n;
                e.1 += c;
            }
            let mut tt = total.lock().unwrap();
            tt.0 += t.0;
            tt.1 += t.1;
        });
        let (n, c) = total.into_inner().unwrap();
        let mut by: Vec<(String, (usize, usize))> = agree_by.into_inner().unwrap().into_iter().collect();
        by.sort_by(|a, b| (b.1.0 - b.1.1).cmp(&(a.1.0 - a.1.1)));
        let line = format!("round {round}: agreement with the parser on {n} words: UAS {:.2}; biggest disagreements by part of speech: {}\n", 100.0 * c as f64 / n.max(1) as f64, by.iter().take(6).map(|(k, (n, c))| format!("{k} {:.0}% ({} words off)", 100.0 * *c as f64 / *n as f64, n - c)).collect::<Vec<_>>().join(", "));
        print!("{line}");
        report += &line;
        save(&report, &trs)?;
        let uas = 100.0 * c as f64 / n.max(1) as f64;
        stage.note("Progress of my current stage", &format!("Stage derive-scan, {} sentences: agreement by round so far: {}.\nRules learned: {}.", data.len(), report.lines().filter(|l| l.starts_with("round")).map(|l| l.split("UAS ").nth(1).and_then(|x| x.split(';').next()).unwrap_or("?").to_string()).collect::<Vec<_>>().join(" → "), trs.len()));
        if let (Some(g), Some(prev)) = (stop_gain, last_uas) {
            if uas - prev < g {
                let l = format!("stopped early: round gain {:.2} < stop-gain {g} (from my brain)\n", uas - prev);
                print!("{l}");
                report += &l;
                save(&report, &trs)?;
                break;
            }
        }
        last_uas = Some(uas);
        if round == rounds {
            break;
        }
        // pass 1: candidate fixes
        let fix: Mutex<std::collections::HashMap<Tr, usize>> = Mutex::new(Default::default());
        par(&|p| {
            let (s, h, r) = derive_all(p, &trs);
            let mut loc: std::collections::HashMap<Tr, usize> = Default::default();
            for i in 0..s.upos.len() {
                if s.head[i] == h[i] || s.head[i] == 0 {
                    continue;
                }
                let gt = s.head[i] - 1;
                let gu = s.upos[gt];
                let dir: i8 = if gt > i { 1 } else { -1 };
                let Some(k) = (1..=3u8).find(|&k| nth(&s, i, dir, gu, k) == Some(gt)) else { continue };
                if makes_cycle(&h, i, gt) {
                    continue;
                }
                for cs in cond_sets(&conds(&s, &h, &r, i)) {
                    *loc.entry(Tr { conds: cs, dir, head: gu, k, rel: s.rel[i] }).or_default() += 1;
                }
            }
            let mut g = fix.lock().unwrap();
            for (t, n) in loc {
                *g.entry(t).or_default() += n;
            }
        });
        let mut fx: Vec<(Tr, usize)> = fix.into_inner().unwrap().into_iter().filter(|(t, _)| !trs.contains(t)).collect();
        fx.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        fx.truncate(2000);
        // pass 2: breaks of the top candidates
        let cand: std::collections::HashMap<Vec<Cond>, Vec<usize>> = fx.iter().enumerate().fold(Default::default(), |mut m, (i, (t, _))| {
            m.entry(t.conds.clone()).or_default().push(i);
            m
        });
        let brk: Mutex<Vec<usize>> = Mutex::new(vec![0; fx.len()]);
        par(&|p| {
            let (s, h, r) = derive_all(p, &trs);
            let mut loc = vec![0usize; fx.len()];
            for i in 0..s.upos.len() {
                if s.head[i] != h[i] || h[i] == 0 {
                    continue;
                }
                for cs in cond_sets(&conds(&s, &h, &r, i)) {
                    if let Some(ids) = cand.get(&cs) {
                        for &id in ids {
                            let t = &fx[id].0;
                            if nth(&s, i, t.dir, t.head, t.k).is_some_and(|j| j + 1 != h[i]) {
                                loc[id] += 1;
                            }
                        }
                    }
                }
            }
            let mut g = brk.lock().unwrap();
            for (a, b) in g.iter_mut().zip(loc) {
                *a += b;
            }
        });
        let brk = brk.into_inner().unwrap();
        let mut scored: Vec<(Tr, usize, usize)> = fx.into_iter().zip(brk).map(|((t, f), b)| (t, f, b)).filter(|(_, f, b)| *f as f64 / (*f + *b) as f64 >= 0.8 && *f > *b + 100).collect();
        scored.sort_by(|a, b| (b.1 as i64 - b.2 as i64).cmp(&(a.1 as i64 - a.2 as i64)).then(a.0.conds.len().cmp(&b.0.conds.len())).then(a.0.cmp(&b.0)));
        // several non-interfering rules per round: different word class and different target
        let mut taken: Vec<(Tr, usize, usize)> = Vec::new();
        for (t, f, b) in scored {
            if taken.len() >= per_round {
                break;
            }
            if taken.iter().any(|(o, _, _)| o.conds[0] == t.conds[0] || (o.head == t.head && o.dir == t.dir)) {
                continue;
            }
            taken.push((t, f, b));
        }
        if taken.is_empty() {
            report += "no rule left with precision ≥ 0.8\n";
            break;
        }
        for (t, f, b) in &taken {
            let l = format!("  learned (fixed {f}, broken {b}): {}\n", t.show());
            print!("{l}");
            report += &l;
            trs.push(t.clone());
        }
        save(&report, &trs)?;
    }
    save(&report, &trs)?;
    // the signal: the stage ends and the state will change — decide where to go next, as a separate letter
    let curve: Vec<f64> = report.lines().filter(|l| l.starts_with("round")).filter_map(|l| l.split("UAS ").nth(1).and_then(|x| x.split(';').next()).and_then(|x| x.trim().parse().ok())).collect();
    let last_gain = if curve.len() >= 2 { curve[curve.len() - 1] - curve[curve.len() - 2] } else { f64::NAN };
    let decision = if last_gain.is_nan() {
        "Decision: no curve to judge; run more rounds before deciding.".to_string()
    } else if last_gain < 0.2 {
        format!("Decision: the curve has flattened (last round +{last_gain:.2}). More rounds of the same rule form will not pay; next, try new condition atoms or another rule form, and keep these {} rules as the base.", trs.len())
    } else {
        format!("Decision: still growing (last round +{last_gain:.2}); continue with more rounds on the same data before changing the rule form.")
    };
    for (sec, body) in crate::state::brain_sections(&trs.iter().map(Tr::show).collect::<Vec<_>>(), &curve) {
        stage.note(&sec, &body);
    }
    stage.decide(&format!("Stage derive-scan ended. Agreement with the parser: {}.\n{decision}", curve.iter().map(|x| format!("{x:.2}")).collect::<Vec<_>>().join(" → ")));
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------
// Incremental learning (after Ngai & Florian 2001, "Transformation-based learning in the fast lane"): fix counts are
// kept and updated only for the words a rule moved; an index from condition sets to words gives the breaks of a
// candidate without a pass over the data. Same choices as `learn`, a fraction of the time.

/// One word's candidate fixes (rules that would move it to its target head).
fn word_fixes(s: &S, h: &[usize], r: &[Rel], i: usize) -> Vec<Tr> {
    if s.head[i] == h[i] || s.head[i] == 0 {
        return Vec::new();
    }
    let gt = s.head[i] - 1;
    let gu = s.upos[gt];
    let dir: i8 = if gt > i { 1 } else { -1 };
    let Some(k) = (1..=3u8).find(|&k| nth(s, i, dir, gu, k) == Some(gt)) else { return Vec::new() };
    if makes_cycle(h, i, gt) {
        return Vec::new();
    }
    cond_sets(&conds(s, h, r, i)).into_iter().map(|cs| Tr { conds: cs, dir, head: gu, k, rel: s.rel[i] }).collect()
}

pub fn learn_incremental(data: &[S], cur: &mut [(Vec<usize>, Vec<Rel>)], iters: usize, min: usize) -> Vec<(Tr, usize, usize)> {
    use std::collections::{HashMap, HashSet};
    let mut fix: HashMap<Tr, i64> = HashMap::new();
    let mut by_conds: HashMap<Vec<Cond>, HashSet<(u32, u16)>> = HashMap::new();
    let mut word_cs: HashMap<(u32, u16), Vec<Vec<Cond>>> = HashMap::new();
    let mut word_fx: HashMap<(u32, u16), Vec<Tr>> = HashMap::new();
    let mut add_word = |si: usize, i: usize, h: &[usize], r: &[Rel], fix: &mut HashMap<Tr, i64>, by_conds: &mut HashMap<Vec<Cond>, HashSet<(u32, u16)>>, word_cs: &mut HashMap<(u32, u16), Vec<Vec<Cond>>>, word_fx: &mut HashMap<(u32, u16), Vec<Tr>>| {
        let s = &data[si];
        let key = (si as u32, i as u16);
        let cs = if h[i] == 0 { Vec::new() } else { cond_sets(&conds(s, h, r, i)) };
        for c in &cs {
            by_conds.entry(c.clone()).or_default().insert(key);
        }
        let fx = word_fixes(s, h, r, i);
        for t in &fx {
            *fix.entry(t.clone()).or_default() += 1;
        }
        word_cs.insert(key, cs);
        word_fx.insert(key, fx);
    };
    let remove_word = |key: (u32, u16), fix: &mut HashMap<Tr, i64>, by_conds: &mut HashMap<Vec<Cond>, HashSet<(u32, u16)>>, word_cs: &mut HashMap<(u32, u16), Vec<Vec<Cond>>>, word_fx: &mut HashMap<(u32, u16), Vec<Tr>>| {
        for c in word_cs.remove(&key).unwrap_or_default() {
            if let Some(set) = by_conds.get_mut(&c) {
                set.remove(&key);
            }
        }
        for t in word_fx.remove(&key).unwrap_or_default() {
            if let Some(n) = fix.get_mut(&t) {
                *n -= 1;
            }
        }
    };
    for (si, (h, r)) in cur.iter().enumerate() {
        for i in 0..data[si].upos.len() {
            add_word(si, i, h, r, &mut fix, &mut by_conds, &mut word_cs, &mut word_fx);
        }
    }
    let mut out: Vec<(Tr, usize, usize)> = Vec::new();
    for _ in 0..iters {
        // the top candidates by fixes; breaks from the condition index (words with that condition set, correct now,
        // that the action would move elsewhere)
        let mut top: Vec<(&Tr, i64)> = fix.iter().filter(|(t, n)| **n >= min as i64 && !out.iter().any(|(o, _, _)| o == *t)).map(|(t, n)| (t, *n)).collect();
        top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        let mut best: Option<(Tr, usize, usize)> = None;
        for (t, f) in top {
            if let Some((_, bf, bb)) = &best {
                if (f as i64) <= (*bf as i64 - *bb as i64) {
                    break; // fixes only fall from here; no candidate can beat the best net
                }
            }
            let mut b = 0usize;
            if let Some(set) = by_conds.get(&t.conds) {
                for &(si, i) in set {
                    let (s, (h, _)) = (&data[si as usize], &cur[si as usize]);
                    let i = i as usize;
                    if s.head[i] == h[i] && nth(s, i, t.dir, t.head, t.k).is_some_and(|j| j + 1 != h[i]) {
                        b += 1;
                    }
                }
            }
            let f = f as usize;
            if f as f64 / (f + b) as f64 >= 0.8 && f >= min + b && best.as_ref().is_none_or(|(_, bf, bb)| (f as i64 - b as i64) > (*bf as i64 - *bb as i64)) {
                best = Some((t.clone(), f, b));
            }
        }
        let Some((t, f, b)) = best else { break };
        // apply to the words that carry the rule's condition set; update only them
        let words: Vec<(u32, u16)> = by_conds.get(&t.conds).map(|s| s.iter().copied().collect()).unwrap_or_default();
        let mut moved = 0usize;
        let mut touched: Vec<(u32, u16)> = Vec::new();
        for (si, i) in words {
            let (s, (h, r)) = (&data[si as usize], &mut cur[si as usize]);
            let iu = i as usize;
            if h[iu] == 0 {
                continue;
            }
            if let Some(j) = nth(s, iu, t.dir, t.head, t.k) {
                if j + 1 != h[iu] && !makes_cycle(h, iu, j) {
                    h[iu] = j + 1;
                    r[iu] = t.rel;
                    moved += 1;
                    touched.push((si, i));
                }
            }
        }
        if moved == 0 {
            break;
        }
        // a move can change the cycle test of other words of the same sentence: refresh whole sentences touched
        let sents: std::collections::BTreeSet<u32> = touched.iter().map(|x| x.0).collect();
        for si in sents {
            for i in 0..data[si as usize].upos.len() {
                remove_word((si, i as u16), &mut fix, &mut by_conds, &mut word_cs, &mut word_fx);
            }
            let (h, r) = (&cur[si as usize].0.clone(), &cur[si as usize].1.clone());
            for i in 0..data[si as usize].upos.len() {
                add_word(si as usize, i, h, r, &mut fix, &mut by_conds, &mut word_cs, &mut word_fx);
            }
        }
        out.push((t, f, b));
    }
    out
}
