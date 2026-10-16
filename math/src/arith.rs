//! v3 (30.09): word problems: a fast core + a slow logical model ("sometimes the fast one computes, while
//! the slow logical one roughly understands how exactly").
//!
//! The logical model does not compute. It only judges which number from the problem is needed (relevance) and which operation
//! stands between each pair of numbers in the expression tree (the operation at the lowest common ancestor: the approach of
//! Roy & Roth 2015, "Solving General Arithmetic Word Problems", EMNLP). Both judgments are an averaged
//! perceptron over word features: you can see which words outweighed and with what weight.
//!
//! The core enumerates expression trees bottom-up (dynamic programming over subsets of numbers, a beam per
//! subset) and computes values. A tree's score decomposes: score(L ∘ R) = score(L) + score(R) +
//! Σ over pairs (i ∈ L, j ∈ R) weight(pair features, ∘), so the beam is honest. The tree recomputes
//! the final answer exactly in rational numbers.
//!
//! Training uses no LLM and no plans: weak supervision from the reference answer. For a training problem the core
//! finds the smallest tree that yields the reference; it becomes the "gold" one for the structured perceptron.

use std::collections::{HashMap, HashSet};

use crate::big::*;
use crate::memory::NUM_WORDS;

// ---------- quantities from the problem ----------

#[derive(Clone, Debug)]
pub struct Qty {
    pub val: f64,
    pub exact: Q,
    /// position in tokens (for constants: end of text)
    pub pos: usize,
    /// surrounding words (±3, no numbers), stemmed
    pub ctx: Vec<String>,
    /// name for the explanation: "18", "20%", "const:7"
    pub name: String,
}

fn stem(w: &str) -> String {
    for suf in ["ing", "ed", "es", "s"] {
        if w.len() > suf.len() + 2 && w.ends_with(suf) {
            return w[..w.len() - suf.len()].to_string();
        }
    }
    w.to_string()
}

#[derive(Clone, Debug)]
enum Tok {
    W(String),
    N(Q, String),
}

fn tokenize(text: &str) -> Vec<Tok> {
    let cs: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_ascii_digit() || (c == '.' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit()) {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || ((cs[i] == ',' || cs[i] == '.' || cs[i] == '/') && i + 1 < cs.len() && cs[i + 1].is_ascii_digit())) {
                i += 1;
            }
            let s: String = cs[st..i].iter().collect();
            let s = s.replace(',', "");
            let v = if let Some((a, b)) = s.split_once('/') { parse_q(a).zip(parse_q(b)).and_then(|(a, b)| if b == q(0) { None } else { Some(a / b) }) } else { parse_q(&s) };
            let pct = i < cs.len() && cs[i] == '%';
            if let Some(mut v) = v {
                let mut name = s.clone();
                if pct {
                    v = v / q(100);
                    name.push('%');
                    i += 1;
                }
                out.push(Tok::N(v, name));
            }
            continue;
        }
        if c.is_alphabetic() {
            let st = i;
            while i < cs.len() && (cs[i].is_alphabetic() || cs[i] == '\'') {
                i += 1;
            }
            let w: String = cs[st..i].iter().collect::<String>().to_lowercase();
            if w == "percent" {
                if let Some(Tok::N(v, n)) = out.last_mut() {
                    *v = v.clone() / q(100);
                    n.push('%');
                    continue;
                }
            }
            match NUM_WORDS.iter().find(|(k, _)| *k == w) {
                // "one" as a pronoun ("one of", "each one") is not a number
                Some(("one", _)) if matches!(cs.get(i..i + 3).map(|s| s.iter().collect::<String>()).as_deref(), Some(" of")) => out.push(Tok::W(w)),
                Some((_, v)) => out.push(Tok::N(q(*v), w.clone())),
                None => out.push(Tok::W(w)),
            }
            continue;
        }
        if c == '$' {
            out.push(Tok::W("$".into()));
        }
        i += 1;
    }
    out
}

/// Constants the problem may imply: by a pair of unit words (week + day → 7).
const CONSTS: &[(&str, &str, i64)] = &[
    ("week", "day", 7), ("hour", "minute", 60), ("minute", "second", 60), ("day", "hour", 24), ("year", "month", 12), ("year", "week", 52),
    ("year", "day", 365), ("dollar", "cent", 100), ("foot", "inch", 12), ("feet", "inch", 12), ("meter", "centimeter", 100), ("kilogram", "gram", 1000),
    ("pound", "ounce", 16), ("gallon", "quart", 4), ("yard", "feet", 3), ("decade", "year", 10), ("century", "year", 100), ("pair", "shoe", 2),
];

pub fn quantities(text: &str) -> (Vec<Qty>, Vec<String>) {
    let toks = tokenize(text);
    let words: Vec<String> = toks.iter().map(|t| match t { Tok::W(w) => stem(w), Tok::N(..) => "#".into() }).collect();
    let mut qs = Vec::new();
    for (i, t) in toks.iter().enumerate() {
        if let Tok::N(v, name) = t {
            let lo = i.saturating_sub(3);
            let hi = (i + 4).min(toks.len());
            let ctx: Vec<String> = (lo..hi).filter(|&k| k != i && words[k] != "#").map(|k| format!("{}{}", if k < i { "<" } else { ">" }, words[k])).collect();
            qs.push(Qty { val: to_f64(v), exact: v.clone(), pos: i, ctx, name: name.clone() });
        }
    }
    let set: HashSet<&str> = words.iter().map(|s| s.as_str()).collect();
    let has = |w: &str| set.contains(stem(w).as_str()) || set.contains(w);
    for (a, b, c) in CONSTS {
        if has(a) && has(b) && !qs.iter().any(|x| x.exact == q(*c)) {
            qs.push(Qty { val: *c as f64, exact: q(*c), pos: toks.len(), ctx: vec![format!("const:{a}-{b}")], name: format!("const:{c}") });
        }
    }
    // MATH_IMPLIED=1: also general implicit constants (2, 100, 60…) if absent from the text, at the end; the model skips or takes them
    if std::env::var("MATH_IMPLIED").as_deref() == Ok("1") {
        for c in [2i64, 100, 60, 12, 7, 4, 10, 3, 24, 30, 52, 1000] {
            if !qs.iter().any(|x| x.exact == q(c)) {
                qs.push(Qty { val: c as f64, exact: q(c), pos: toks.len(), ctx: vec!["const:implied".into()], name: format!("const:{c}") });
            }
        }
    }
    (qs, words)
}

// ---------- features ----------

fn h(s: &str) -> u64 {
    // FNV-1a
    let mut x: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        x ^= b as u64;
        x = x.wrapping_mul(0x100000001b3);
    }
    x
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn sym(self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
    }
}

/// Pair label: the operation and, for − and ÷, the direction relative to the order in the text.
fn pair_label(op: Op, i_first_in_text: bool) -> &'static str {
    match (op, i_first_in_text) {
        (Op::Add, _) => "add",
        (Op::Mul, _) => "mul",
        (Op::Sub, true) => "sub>",
        (Op::Sub, false) => "sub<",
        (Op::Div, true) => "div>",
        (Op::Div, false) => "div<",
    }
}

const LABELS: [&str; 6] = ["add", "mul", "sub>", "sub<", "div>", "div<"];

pub struct Prob {
    pub qs: Vec<Qty>,
    /// features of pair (i, j), i < j in number order; already hashed for each label
    pair: Vec<Vec<[Vec<u64>; 6]>>,
    /// relevance features of each number
    rel: Vec<Vec<u64>>,
    pair_txt: Vec<Vec<Vec<String>>>,
    rel_txt: Vec<Vec<String>>,
}

fn question_words(words: &[String]) -> Vec<String> {
    // last question sentence: words after the last period before the end
    let n = words.len();
    let st = n.saturating_sub(12);
    words[st..].iter().filter(|w| *w != "#").cloned().collect()
}

pub fn prepare(text: &str, max_q: usize) -> Prob {
    let (mut qs, words) = quantities(text);
    qs.truncate(max_q);
    let qw = question_words(&words);
    let n = qs.len();
    let mut pair_txt = vec![vec![Vec::new(); n]; n];
    let mut pair = vec![vec![std::array::from_fn(|_| Vec::new()); n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (a, b) = (&qs[i], &qs[j]);
            let mut f: Vec<String> = Vec::new();
            f.push("bias".into());
            f.extend(a.ctx.iter().map(|w| format!("a{w}")));
            f.extend(b.ctx.iter().map(|w| format!("b{w}")));
            let (lo, hi) = if a.pos < b.pos { (a.pos, b.pos) } else { (b.pos, a.pos) };
            let between: Vec<&String> = words.iter().take(hi.min(words.len())).skip(lo + 1).filter(|w| *w != "#").collect();
            if between.len() <= 25 {
                f.extend(between.iter().map(|w| format!("m:{w}")));
            } else {
                f.push("m:far".into());
            }
            f.extend(qw.iter().map(|w| format!("q:{w}")));
            if a.ctx.iter().any(|w| w.starts_with('>') && b.ctx.contains(w)) {
                f.push("sameunit".into());
            }
            if b.exact < q(1) && b.exact > q(0) {
                f.push("b<1".into());
            }
            if a.exact < q(1) && a.exact > q(0) {
                f.push("a<1".into());
            }
            if b.name.ends_with('%') {
                f.push("b%".into());
            }
            if a.name.ends_with('%') {
                f.push("a%".into());
            }
            let first = a.pos < b.pos;
            for (k, lab) in LABELS.iter().enumerate() {
                let _ = first;
                pair[i][j][k] = f.iter().map(|s| h(&format!("{lab}|{s}"))).collect();
            }
            pair_txt[i][j] = f;
        }
    }
    let rel_txt: Vec<Vec<String>> = qs.iter().map(|a| {
        let mut f = vec!["bias".to_string()];
        f.extend(a.ctx.iter().cloned());
        f.extend(qw.iter().map(|w| format!("q:{w}")));
        if a.name.starts_with("const") {
            f.push("const".into());
        }
        f
    }).collect();
    let rel = rel_txt.iter().map(|f| f.iter().map(|s| h(&format!("rel|{s}"))).collect()).collect();
    Prob { qs, pair, rel, pair_txt, rel_txt }
}

// ---------- trees ----------

#[derive(Clone, Debug)]
pub enum Tree {
    Leaf(usize),
    Node(Op, Box<Tree>, Box<Tree>),
}

impl Tree {
    pub fn show(&self, qs: &[Qty]) -> String {
        match self {
            Tree::Leaf(i) => qs[*i].name.clone(),
            Tree::Node(op, l, r) => format!("({} {} {})", l.show(qs), op.sym(), r.show(qs)),
        }
    }
    pub fn exact(&self, qs: &[Qty]) -> Option<Q> {
        Some(match self {
            Tree::Leaf(i) => qs[*i].exact.clone(),
            Tree::Node(op, l, r) => {
                let (a, b) = (l.exact(qs)?, r.exact(qs)?);
                match op {
                    Op::Add => a + b,
                    Op::Sub => a - b,
                    Op::Mul => a * b,
                    Op::Div => {
                        if b == q(0) {
                            return None;
                        }
                        a / b
                    }
                }
            }
        })
    }
    fn leaves(&self, out: &mut Vec<usize>) {
        match self {
            Tree::Leaf(i) => out.push(*i),
            Tree::Node(_, l, r) => {
                l.leaves(out);
                r.leaves(out);
            }
        }
    }
    /// tree features: leaf relevance + pair labels at common ancestors
    fn feats(&self, p: &Prob, out: &mut Vec<u64>) {
        if let Tree::Node(op, l, r) = self {
            let (mut a, mut b) = (Vec::new(), Vec::new());
            l.leaves(&mut a);
            r.leaves(&mut b);
            for &i in &a {
                for &j in &b {
                    let k = LABELS.iter().position(|x| *x == pair_label(*op, p.qs[i].pos < p.qs[j].pos)).unwrap();
                    out.extend(&p.pair[i][j][k]);
                }
            }
            l.feats(p, out);
            r.feats(p, out);
        }
    }
    pub fn all_feats(&self, p: &Prob) -> Vec<u64> {
        let mut out = Vec::new();
        let mut ls = Vec::new();
        self.leaves(&mut ls);
        for &i in &ls {
            out.extend(&p.rel[i]);
        }
        self.feats(p, &mut out);
        out
    }
}

#[derive(Clone)]
struct Cand {
    val: f64,
    score: f64,
    tree: Tree,
}

// ---------- model ----------

#[derive(Default, Clone)]
pub struct Model {
    w: HashMap<u64, f64>,
    /// for averaging
    acc: HashMap<u64, f64>,
    stamp: HashMap<u64, u64>,
    t: u64,
    pub max_leaves: usize,
    pub beam: usize,
    pub gold_beam: usize,
}

impl Model {
    pub fn new(max_leaves: usize, beam: usize) -> Model {
        Model { max_leaves, beam, gold_beam: 400, ..Default::default() }
    }
    fn wsum(&self, fs: &[u64]) -> f64 {
        fs.iter().map(|f| self.w.get(f).copied().unwrap_or(0.0)).sum()
    }
    fn update(&mut self, fs: &[u64], d: f64) {
        for f in fs {
            // lazy averaging
            let t = self.t;
            let w = self.w.entry(*f).or_insert(0.0);
            let s = self.stamp.entry(*f).or_insert(0);
            *self.acc.entry(*f).or_insert(0.0) += *w * (t - *s) as f64;
            *s = t;
            *w += d;
        }
    }
    pub fn tick(&mut self) {
        self.t += 1;
    }
    pub fn averaged(&self) -> Model {
        let mut m = Model::new(self.max_leaves, self.beam);
        m.gold_beam = self.gold_beam;
        for (f, w) in &self.w {
            let s = self.stamp.get(f).copied().unwrap_or(0);
            let a = self.acc.get(f).copied().unwrap_or(0.0) + w * (self.t - s) as f64;
            m.w.insert(*f, a / self.t.max(1) as f64);
        }
        m
    }

    /// Beam over subsets: for each mask, up to `beam` best trees with distinct values.
    fn search(&self, p: &Prob, score: bool, beam: usize) -> std::collections::BTreeMap<u32, Vec<Cand>> {
        let n = p.qs.len();
        let mut by: std::collections::BTreeMap<u32, Vec<Cand>> = std::collections::BTreeMap::new();
        // table of pair scores
        let mut ps = vec![vec![[0.0f64; 6]; n]; n];
        if score {
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        for k in 0..6 {
                            ps[i][j][k] = self.wsum(&p.pair[i][j][k]);
                        }
                    }
                }
            }
        }
        for i in 0..n {
            by.insert(1 << i, vec![Cand { val: p.qs[i].val, score: 0.0, tree: Tree::Leaf(i) }]);
        }
        let mut masks: Vec<u32> = (1u32..(1 << n)).filter(|m| m.count_ones() >= 2 && m.count_ones() as usize <= self.max_leaves).collect();
        masks.sort_by_key(|m| m.count_ones());
        for m in masks {
            let mut out: Vec<Cand> = Vec::new();
            // split m into two non-empty parts (l is the submask with the lowest bit, to avoid duplicates)
            let low = m & m.wrapping_neg();
            let mut l = (m - 1) & m;
            while l > 0 {
                let r = m ^ l;
                if l & low != 0 {
                    if let (Some(ls), Some(rs)) = (by.get(&l), by.get(&r)) {
                        let li: Vec<usize> = (0..n).filter(|b| l >> b & 1 == 1).collect();
                        let ri: Vec<usize> = (0..n).filter(|b| r >> b & 1 == 1).collect();
                        let mut cross = [0.0f64; 6];
                        let mut cross_rev = [0.0f64; 6]; // for Sub/Div with R on the left
                        for &i in &li {
                            for &j in &ri {
                                let fwd = p.qs[i].pos < p.qs[j].pos;
                                for (k, op) in [Op::Add, Op::Mul, Op::Sub, Op::Div].iter().enumerate() {
                                    let a = LABELS.iter().position(|x| *x == pair_label(*op, fwd)).unwrap();
                                    let b = LABELS.iter().position(|x| *x == pair_label(*op, !fwd)).unwrap();
                                    cross[k] += ps[i][j][a];
                                    cross_rev[k] += ps[j][i][b];
                                }
                            }
                        }
                        for a in ls {
                            for b in rs {
                                let base = a.score + b.score;
                                let mut push = |val: f64, sc: f64, op: Op, x: &Cand, y: &Cand| {
                                    if val.is_finite() && val >= 0.0 && val < 1e12 {
                                        out.push(Cand { val, score: sc, tree: Tree::Node(op, Box::new(x.tree.clone()), Box::new(y.tree.clone())) });
                                    }
                                };
                                push(a.val + b.val, base + cross[0], Op::Add, a, b);
                                push(a.val * b.val, base + cross[1], Op::Mul, a, b);
                                push(a.val - b.val, base + cross[2], Op::Sub, a, b);
                                push(b.val - a.val, base + cross_rev[2], Op::Sub, b, a);
                                if b.val != 0.0 {
                                    push(a.val / b.val, base + cross[3], Op::Div, a, b);
                                }
                                if a.val != 0.0 {
                                    push(b.val / a.val, base + cross_rev[3], Op::Div, b, a);
                                }
                            }
                        }
                    }
                }
                l = (l - 1) & m;
            }
            // beam: distinct values, best by score
            out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
            let mut seen: HashSet<i64> = HashSet::new();
            let mut keep = Vec::new();
            for c in out {
                let key = (c.val * 1e6).round() as i64;
                if seen.insert(key) {
                    keep.push(c);
                    if keep.len() >= beam {
                        break;
                    }
                }
            }
            by.insert(m, keep);
        }
        by
    }

    /// Best tree with a non-negative integer answer (GSM8K); returns (tree, score, margin to the second value).
    pub fn predict(&self, p: &Prob, int_answer: bool) -> Option<(Tree, f64, f64)> {
        let by = self.search(p, true, self.beam);
        let mut all: Vec<(f64, f64, &Tree)> = Vec::new();
        for (m, cs) in &by {
            let rel: f64 = (0..p.qs.len()).filter(|b| m >> b & 1 == 1).map(|b| self.wsum(&p.rel[b])).sum();
            for c in cs {
                if int_answer && (c.val - c.val.round()).abs() > 1e-6 {
                    continue;
                }
                all.push((c.score + rel, c.val, &c.tree));
            }
        }
        all.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let best = all.first()?;
        let second = all.iter().find(|x| (x.1 - best.1).abs() > 1e-6).map(|x| x.0).unwrap_or(best.0 - 10.0);
        Some((best.2.clone(), best.0, best.0 - second))
    }

    /// Smallest tree that yields the reference (weak supervision); among equal sizes, the one with the better score.
    pub fn gold_tree(&self, p: &Prob, gold: f64) -> Option<Tree> {
        // for the reference the beam is wider: we look for whether a tree exists at all, not only among likely ones
        let by = self.search(p, true, self.gold_beam.max(self.beam));
        let mut best: Option<(u32, f64, Tree)> = None;
        for (m, cs) in &by {
            for c in cs {
                if (c.val - gold).abs() <= 1e-6 * gold.abs().max(1.0) {
                    let size = m.count_ones();
                    let rel: f64 = (0..p.qs.len()).filter(|b| m >> b & 1 == 1).map(|b| self.wsum(&p.rel[b])).sum();
                    let sc = c.score + rel;
                    if best.as_ref().is_none_or(|(s, b, _)| size < *s || (size == *s && sc > *b)) {
                        best = Some((size, sc, c.tree.clone()));
                    }
                }
            }
        }
        best.map(|(_, _, t)| t)
    }

    /// Gold tree from LLM plan supervision: leaves are only numbers the plan actually used (`used`);
    /// among trees that yield the reference, the one covering the most used numbers, then the smallest.
    /// All trees that yield the reference (by used numbers, if `used`), best first: more used numbers
    /// (for plan supervision) or fewer leaves (weak supervision), then by score.
    pub fn gold_trees(&self, p: &Prob, gold: f64, used: Option<&[f64]>) -> Vec<Tree> {
        let ok_leaf: Vec<bool> = p.qs.iter().map(|x| used.is_none_or(|u| u.iter().any(|v| (v - x.val).abs() < 1e-9))).collect();
        let by = self.search(p, true, self.gold_beam.max(self.beam));
        let mut all: Vec<(i64, f64, Tree)> = Vec::new();
        for (m, cs) in &by {
            if (0..p.qs.len()).any(|b| m >> b & 1 == 1 && !ok_leaf[b]) {
                continue;
            }
            for c in cs {
                if (c.val - gold).abs() <= 1e-6 * gold.abs().max(1.0) {
                    let n = m.count_ones() as i64;
                    let key = if used.is_some() { n } else { -n };
                    let rel: f64 = (0..p.qs.len()).filter(|b| m >> b & 1 == 1).map(|b| self.wsum(&p.rel[b])).sum();
                    all.push((key, c.score + rel, c.tree.clone()));
                }
            }
        }
        all.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)));
        all.into_iter().map(|x| x.2).collect()
    }

    pub fn gold_tree_used(&self, p: &Prob, gold: f64, used: &[f64]) -> Option<Tree> {
        let ok_leaf: Vec<bool> = p.qs.iter().map(|x| used.iter().any(|u| (u - x.val).abs() < 1e-9)).collect();
        let by = self.search(p, true, self.gold_beam.max(self.beam));
        let mut best: Option<(i64, f64, Tree)> = None;
        for (m, cs) in &by {
            if (0..p.qs.len()).any(|b| m >> b & 1 == 1 && !ok_leaf[b]) {
                continue;
            }
            for c in cs {
                if (c.val - gold).abs() <= 1e-6 * gold.abs().max(1.0) {
                    let key = m.count_ones() as i64; // more used numbers is better
                    let rel: f64 = (0..p.qs.len()).filter(|b| m >> b & 1 == 1).map(|b| self.wsum(&p.rel[b])).sum();
                    let sc = c.score + rel;
                    if best.as_ref().is_none_or(|(k, b, _)| key > *k || (key == *k && sc > *b)) {
                        best = Some((key, sc, c.tree.clone()));
                    }
                }
            }
        }
        best.map(|(_, _, t)| t)
    }

    /// Structured perceptron step: if prediction ≠ reference, reinforce the gold tree and weaken the prediction.
    pub fn learn(&mut self, p: &Prob, gold_t: &Tree, gold: f64) -> bool {
        self.tick();
        let Some((pred, _, _)) = self.predict(p, true) else {
            self.update(&gold_t.all_feats(p), 1.0);
            return false;
        };
        let pv = pred.exact(&p.qs).map(|x| to_f64(&x)).unwrap_or(f64::NAN);
        if (pv - gold).abs() <= 1e-6 * gold.abs().max(1.0) {
            return true;
        }
        self.update(&gold_t.all_feats(p), 1.0);
        self.update(&pred.all_feats(p), -1.0);
        false
    }

    /// Explanation: the weightiest features of each tree node.
    pub fn explain(&self, p: &Prob, t: &Tree) -> Vec<String> {
        let mut out = Vec::new();
        self.explain_rec(p, t, &mut out);
        out
    }
    fn explain_rec(&self, p: &Prob, t: &Tree, out: &mut Vec<String>) {
        if let Tree::Node(op, l, r) = t {
            let (mut a, mut b) = (Vec::new(), Vec::new());
            l.leaves(&mut a);
            r.leaves(&mut b);
            let mut contrib: HashMap<String, f64> = HashMap::new();
            for &i in &a {
                for &j in &b {
                    let lab = pair_label(*op, p.qs[i].pos < p.qs[j].pos);
                    for s in &p.pair_txt[i][j] {
                        *contrib.entry(s.clone()).or_insert(0.0) += self.w.get(&h(&format!("{lab}|{s}"))).copied().unwrap_or(0.0);
                    }
                }
            }
            let mut v: Vec<(String, f64)> = contrib.into_iter().filter(|(_, w)| *w != 0.0).collect();
            v.sort_by(|x, y| y.1.abs().partial_cmp(&x.1.abs()).unwrap());
            out.push(format!("{} {} {}: {}", l.show(&p.qs), op.sym(), r.show(&p.qs), v.iter().take(4).map(|(s, w)| format!("{s} {w:+.1}")).collect::<Vec<_>>().join(", ")));
            self.explain_rec(p, l, out);
            self.explain_rec(p, r, out);
        }
        let _ = &p.rel_txt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantities_words_percent_consts() {
        let (qs, _) = quantities("She eats three eggs and sells 20% of 150 eggs a week for 2 days.");
        let names: Vec<&str> = qs.iter().map(|x| x.name.as_str()).collect();
        assert_eq!(names, ["three", "20%", "150", "2", "const:7"]);
        assert_eq!(qs[1].exact, q(1) / q(5));
    }

    #[test]
    fn gold_tree_found_and_negative_control() {
        let m = Model::new(4, 20);
        let p = prepare("Janet's ducks lay 16 eggs per day. She eats three and bakes with four. She sells the rest for $2 each. How much does she make?", 9);
        let t = m.gold_tree(&p, 18.0).expect("tree for 18");
        assert_eq!(t.exact(&p.qs), Some(q(18)));
        // negative control: a value that cannot be built from these numbers has no tree
        assert!(m.gold_tree(&p, 1_000_003.0).is_none());
    }
}
