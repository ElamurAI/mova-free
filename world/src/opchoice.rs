//! The choice of the operation: which two told quantities a problem joins, and how (+, −, ×, ÷), and whether a third
//! joins the result in a second step — learned from the answers of a book of problems, after Roy & Roth (2015): every
//! ordered pair of mentions with every operation is a candidate, and with a third mention and a second operation; its features are the question's words, the two mentions' things, verbs and contexts, whether their
//! things are the question's, their order; an averaged perceptron with integer weights learns to put first a
//! candidate whose value is the book's answer. Every choice can be explained: the features that carried it, with
//! their weights.
//!
//! Deterministic: features are strings in ordered maps, the training order is fixed, the weights are integers.
//!
//!   world opchoice train <json> [epochs]   — learn from a book (SVAMP's json), keep the weights in <hot>/school/ops.tsv
//!   world opchoice test <json>             — right, answered, negative control (the answers of the next problem)
//!   world opchoice explain "<problem>"     — the choice and why

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;

use crate::mentions::{Mention, mentions_all, stem};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
    }
    fn apply(self, x: i64, y: i64) -> Option<i64> {
        match self {
            Op::Add => Some(x + y),
            Op::Sub => (x >= y).then_some(x - y),
            Op::Mul => Some(x * y),
            Op::Div => (y != 0 && x % y == 0).then(|| x / y),
        }
    }
}

const OPS: [Op; 4] = [Op::Add, Op::Sub, Op::Mul, Op::Div];

/// One candidate: the first mention, the operation, the second, the value.
#[derive(Clone, Debug)]
pub struct Cand {
    pub x: usize,
    pub op: Op,
    pub y: usize,
    /// a second step: (x op y) op2 z
    pub then: Option<(Op, usize)>,
    pub value: i64,
}

fn question_words(text: &str) -> BTreeSet<String> {
    let q = text.rsplit_once(['.', '!']).map(|x| x.1).unwrap_or(text);
    let q = crate::numbers::normalize(q).0;
    let mut set: BTreeSet<String> = q.split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 1 && w.parse::<i64>().is_err()).map(|w| stem(&w.to_lowercase())).filter(|w| !matches!(w.as_str(), "the" | "a" | "of" | "to" | "is" | "are" | "do" | "doe" | "did")).collect();
    // what the question's words mean by the dictionary: «#means +» for «altogether», «#means /» for «share» (off with
    // MOVA_NOMEANS=1)
    if std::env::var("MOVA_NOMEANS").is_err() {
        for w in q.split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 2) {
            if let Some(op) = crate::meanings::op_of(w) {
                set.insert(format!("#means {op}"));
            }
        }
    }
    set
}

/// The features of a candidate.
fn features(ms: &[Mention], q: &BTreeSet<String>, c: &Cand) -> Vec<String> {
    let (x, y, o) = (&ms[c.x], &ms[c.y], c.op.name());
    let mut f = vec![format!("op {o}")];
    for w in q {
        f.push(format!("q {w} {o}"));
    }
    let xin = q.contains(&x.thing);
    let yin = q.contains(&y.thing);
    f.push(format!("x thing asked {xin} {o}"));
    f.push(format!("y thing asked {yin} {o}"));
    f.push(format!("same thing {} {o}", x.thing == y.thing));
    f.push(format!("x first {} {o}", c.x < c.y));
    f.push(format!("x bigger {} {o}", x.value >= y.value));
    f.push(format!("x in question {} {o}", x.in_q));
    f.push(format!("y in question {} {o}", y.in_q));
    f.push(format!("same sentence {} {o}", x.sentence == y.sentence));
    f.push(format!("x verb {} {o}", x.verb));
    f.push(format!("y verb {} {o}", y.verb));
    f.push(format!("verbs {} {} {o}", x.verb, y.verb));
    for w in x.ctx.iter().chain(&x.adj) {
        f.push(format!("x ctx {w} {o}"));
    }
    for w in y.ctx.iter().chain(&y.adj) {
        f.push(format!("y ctx {w} {o}"));
    }
    // rates: a quantity «per X» joined with a quantity of X («$78 off each t-shirt» × «183 t-shirts»)
    f.push(format!("x rate {} {o}", x.per.is_some()));
    f.push(format!("y rate {} {o}", y.per.is_some()));
    f.push(format!("x per y {} {o}", x.per.as_ref().is_some_and(|p| *p == y.thing)));
    f.push(format!("y per x {} {o}", y.per.as_ref().is_some_and(|p| *p == x.thing)));
    // the kinds of the verbs and things (WordNet), so phrasings never seen share strength with known ones
    // («deleted» and «threw away» are both kinds of removing); off with MOVA_NOKINDS=1
    // what the two quantities' verbs mean by the dictionary
    if std::env::var("MOVA_NOMEANS").is_err() {
        for (tag, m) in [("x", x), ("y", y)] {
            if let Some(op) = crate::meanings::op_of(&m.verb_word) {
                f.push(format!("{tag} verb means {op} {o}"));
            }
        }
    }
    // the verb classes of the global level (get +, give ±, make +, lose −, have =, move =): the effect a verb has on a
    // quantity, the same for every phrasing of it; off with MOVA_NOCLASSES=1
    if std::env::var("MOVA_NOCLASSES").is_err() {
        for (tag, m) in [("x", x), ("y", y)] {
            if let Some((cl, e)) = verb_class(&m.verb_word) {
                f.push(format!("{tag} verb class {cl} {o}"));
                f.push(format!("{tag} verb effect {e} {o}"));
            }
        }
        // the verb's effect seen from the one the question asks about: «Sam gave 5» is less for Sam, more for the one
        // who got them (Hosseini et al. 2014; Roy & Roth 2018); off with MOVA_NOHOLDER=1
        if std::env::var("MOVA_NOHOLDER").is_err() {
            for (tag, m) in [("x", x), ("y", y)] {
                if let Some(h) = m.holder_asked {
                    if let Some((_, e)) = verb_class(&m.verb_word) {
                        f.push(format!("{tag} effect {e} holder asked {h} {o}"));
                    }
                }
            }
        }
        if let (Some(a), Some(b)) = (verb_class(&x.verb_word), verb_class(&y.verb_word)) {
            f.push(format!("verb classes {} {} {o}", a.0, b.0));
        }
    }
    if !no_kinds() {
        for (tag, m) in [("x", x), ("y", y)] {
            if let Some(k) = kind(&m.verb_word, "verb") {
                f.push(format!("{tag} verb kind {k} {o}"));
            }
            if let Some(k) = kind(&m.thing_word, "noun") {
                f.push(format!("{tag} thing kind {k} {o}"));
            }
        }
    }
    // what the tree says about the two quantities (`mentions::tree_feats`, on with MOVA_TREE=1)
    // MOVA_TREEF=role,case,comp,each,verb,holder,roles — the groups to use (default all; for the ablation)
    let groups: Option<Vec<String>> = std::env::var("MOVA_TREEF").ok().map(|v| v.split(',').map(String::from).collect());
    let on = |g: &str| groups.as_ref().is_none_or(|v| v.iter().any(|x| x == g));
    for (tag, m) in [("x", x), ("y", y)] {
        for t in &m.tree {
            if on(t.split(' ').next().unwrap_or("")) {
                f.push(format!("{tag} tree {t} {o}"));
            }
        }
    }
    let role = |m: &Mention| m.tree.iter().find_map(|t| t.strip_prefix("role ")).unwrap_or("-").to_string();
    if on("roles") && !x.tree.is_empty() && !y.tree.is_empty() {
        f.push(format!("tree roles {} {} {o}", role(x), role(y)));
    }
    // how many mentions the problem has: a distractor-free problem is joined differently
    f.push(format!("mentions {} {o}", ms.len().min(5)));
    match c.then {
        None => f.push(format!("steps 1 {o}")),
        Some((o2, zi)) => {
            let (z, o2) = (&ms[zi], o2.name());
            f.push(format!("steps 2 {o} {o2}"));
            f.push(format!("then {o2}"));
            for w in q {
                f.push(format!("q {w} then {o2}"));
            }
            f.push(format!("z thing asked {} {o2}", q.contains(&z.thing)));
            f.push(format!("z same thing as x {} {o2}", z.thing == x.thing));
            f.push(format!("z last {} {o2}", zi > c.x && zi > c.y));
            f.push(format!("z in question {} {o2}", z.in_q));
            f.push(format!("z verb {} {o2}", z.verb));
            f.push(format!("z rate {} {o2}", z.per.is_some()));
            f.push(format!("z per x {} {o2}", z.per.as_ref().is_some_and(|p| *p == x.thing || *p == y.thing)));
            for w in z.ctx.iter().chain(&z.adj) {
                f.push(format!("z ctx {w} {o2}"));
            }
        }
    }
    f
}

fn no_kinds() -> bool {
    static N: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *N.get_or_init(|| std::env::var("MOVA_NOKINDS").is_ok())
}

/// A verb's class on the global level, by its plain form (the endings taken off, irregular forms by the stems).
fn verb_class(word: &str) -> Option<(&'static str, char)> {
    if word.is_empty() {
        return None;
    }
    let w = word.to_lowercase();
    let mut tries = vec![w.clone(), crate::mentions::stem(&w)];
    for (suf, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ed", ""), ("ed", "e"), ("d", ""), ("ing", ""), ("ing", "e")] {
        if let Some(b) = w.strip_suffix(suf) {
            tries.push(format!("{b}{add}"));
        }
    }
    tries.into_iter().find_map(|t| global::verb_class(&t))
}

/// A word's kind, cached (the WordNet lookup is the slow part).
fn kind(word: &str, pos: &str) -> Option<String> {
    if word.is_empty() {
        return None;
    }
    static C: std::sync::Mutex<BTreeMap<(String, String), Option<String>>> = std::sync::Mutex::new(BTreeMap::new());
    let key = (word.to_string(), pos.to_string());
    if let Some(v) = C.lock().unwrap().get(&key) {
        return v.clone();
    }
    let v = crate::dictionary::kind_of(word, pos);
    C.lock().unwrap().insert(key, v.clone());
    v
}

/// All candidates of a problem.
pub fn candidates(ms: &[Mention]) -> Vec<Cand> {
    let mut v = Vec::new();
    for x in 0..ms.len() {
        for y in 0..ms.len() {
            if x == y {
                continue;
            }
            for op in OPS {
                // + and × are the same both ways: keep one order
                if matches!(op, Op::Add | Op::Mul) && x > y {
                    continue;
                }
                if let Some(value) = op.apply(ms[x].value, ms[y].value) {
                    v.push(Cand { x, op, y, then: None, value });
                    // a second step with a third quantity (problems of up to five quantities)
                    if ms.len() >= 3 && ms.len() <= 5 {
                        for z in (0..ms.len()).filter(|&z| z != x && z != y) {
                            for op2 in OPS {
                                if let Some(v2) = op2.apply(value, ms[z].value) {
                                    v.push(Cand { x, op, y, then: Some((op2, z)), value: v2 });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    v
}

/// The weights, by the features' names.
#[derive(Default, Clone)]
pub struct Weights {
    pub w: BTreeMap<String, i64>,
}

impl Weights {
    fn score(&self, f: &[String]) -> i64 {
        f.iter().map(|x| self.w.get(x).copied().unwrap_or(0)).sum()
    }
}

/// The best candidate of a problem by the weights (ties: the first in the candidates' order).
pub fn choose<'a>(w: &Weights, ms: &[Mention], q: &BTreeSet<String>, cs: &'a [Cand]) -> Option<&'a Cand> {
    let mut best: Option<(i64, &Cand)> = None;
    for c in cs {
        let s = w.score(&features(ms, q, c));
        if best.is_none_or(|b| s > b.0) {
            best = Some((s, c));
        }
    }
    best.map(|b| b.1)
}

/// How sure the choice is: the score of the best candidate over the best candidate with another value (0 on a tie;
/// a large margin when no other value is in sight).
pub fn margin(w: &Weights, text: &str) -> i64 {
    let ms = mentions_all(text);
    let q = question_words(text);
    let cs = candidates(&ms);
    let scored: Vec<(i64, i64)> = cs.iter().map(|c| (w.score(&features(&ms, &q, c)), c.value)).collect();
    let Some(&(top, v)) = scored.iter().max_by_key(|x| x.0) else { return 0 };
    // the value the choice gives is the first candidate with the top score
    let v = scored.iter().find(|x| x.0 == top).map(|x| x.1).unwrap_or(v);
    scored.iter().filter(|x| x.1 != v).map(|x| top - x.0).min().unwrap_or(1000)
}

/// The margin from which the learned choice calls itself sure: on the 5-fold check of SVAMP's train part, answers with
/// a margin of 8 or more were right 190 times of 257 (73%), all answers 360 of 700 (51%).
pub const SURE: i64 = 8;

/// Problems of a book: (text, answer).
pub fn book(path: &std::path::Path) -> Vec<(String, i64)> {
    let v: serde_json::Value = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| {
                    let ans = x["Answer"].as_f64()?;
                    if ans.fract() != 0.0 {
                        return None;
                    }
                    Some((format!("{} {}", x["Body"].as_str()?, x["Question"].as_str()?), ans as i64))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Learn from a book: `epochs` passes in the book's order; a problem none of whose candidates gives the answer
/// teaches nothing (it needs more steps or a number the mentions miss). The features are numbered once (in the order
/// they are first met) so the passes count with integers in vectors; the weights come back named.
pub fn train(items: &[(String, i64)], epochs: usize) -> Weights {
    // a bag (MOVA_BAG=k): k learners, each on the problems in its own drawn order, their weights summed — one order's
    // accidents average out
    let bag: usize = std::env::var("MOVA_BAG").ok().and_then(|x| x.parse().ok()).unwrap_or(1);
    let mut sum: BTreeMap<String, i64> = BTreeMap::new();
    for b in 0..bag {
        let mut order: Vec<(String, i64)> = items.to_vec();
        if b > 0 {
            let mut r = crate::lessons::Rng(0x9E37_79B9 ^ (b as u64) << 8 | 1);
            for i in (1..order.len()).rev() {
                let j = (r.next() % (i as u64 + 1)) as usize;
                order.swap(i, j);
            }
        }
        let mut l = Learner::default();
        let new = l.add(&order);
        l.run(new, epochs);
        for (k, v) in l.weights().w {
            *sum.entry(k).or_default() += v;
        }
    }
    sum.retain(|_, v| *v != 0);
    Weights { w: sum }
}

/// A learner that keeps what it learned: the features numbered once, the weights and their running sums; new problems
/// are added and learned without going over the old ones again (a lesson teaches its own exercises, in minutes, not
/// the whole school anew).
#[derive(Default)]
pub struct Learner {
    names: BTreeMap<String, u32>,
    order: Vec<String>,
    w: Vec<i64>,
    sum: Vec<i64>,
    stamp: Vec<i64>,
    step: i64,
    // per problem: the candidates' values, their numbered features, the answer
    prepared: Vec<(Vec<i64>, Vec<Vec<u32>>, i64)>,
}

impl Learner {
    fn id(&mut self, f: String) -> u32 {
        if let Some(&i) = self.names.get(&f) {
            return i;
        }
        let i = self.order.len() as u32;
        self.names.insert(f.clone(), i);
        self.order.push(f);
        self.w.push(0);
        self.sum.push(0);
        self.stamp.push(self.step);
        i
    }

    /// Add problems; returns the range of the new ones (those no candidate can answer teach nothing and are left out).
    pub fn add(&mut self, items: &[(String, i64)]) -> std::ops::Range<usize> {
        let from = self.prepared.len();
        for (t, a) in items {
            let ms = mentions_all(t);
            let q = question_words(t);
            let cs = candidates(&ms);
            if !cs.iter().any(|c| c.value == *a) {
                continue;
            }
            let fs: Vec<Vec<u32>> = cs.iter().map(|c| features(&ms, &q, c).into_iter().map(|f| self.id(f)).collect()).collect();
            self.prepared.push((cs.iter().map(|c| c.value).collect(), fs, *a));
        }
        from..self.prepared.len()
    }

    /// `epochs` passes over the given problems.
    pub fn run(&mut self, range: std::ops::Range<usize>, epochs: usize) {
        for _ in 0..epochs {
            for k in range.clone() {
                self.step += 1;
                let (best, g) = {
                    let (vals, fs, a) = &self.prepared[k];
                    let score = |f: &[u32]| -> i64 { f.iter().map(|&i| self.w[i as usize]).sum() };
                    // the chosen: the best score, the first on a tie
                    let mut best = 0;
                    let mut bs = i64::MIN;
                    for (j, f) in fs.iter().enumerate() {
                        let s = score(f);
                        if s > bs {
                            bs = s;
                            best = j;
                        }
                    }
                    if vals[best] == *a {
                        continue;
                    }
                    let mut g = usize::MAX;
                    let mut gs = i64::MIN;
                    for (j, f) in fs.iter().enumerate() {
                        if vals[j] == *a {
                            let s = score(f);
                            if s > gs {
                                gs = s;
                                g = j;
                            }
                        }
                    }
                    (best, g)
                };
                let fg = self.prepared[k].1[g].clone();
                let fb = self.prepared[k].1[best].clone();
                for (f, d) in [(&fg, 1i64), (&fb, -1i64)] {
                    for &i in f.iter() {
                        let i = i as usize;
                        // the lazy average: the sum catches up with the time the weight stood still
                        self.sum[i] += self.w[i] * (self.step - self.stamp[i]);
                        self.stamp[i] = self.step;
                        self.w[i] += d;
                    }
                }
            }
        }
    }

    /// The averaged weights, by the features' names.
    pub fn weights(&self) -> Weights {
        let mut out = Weights::default();
        for (i, f) in self.order.iter().enumerate() {
            let total = self.sum[i] + self.w[i] * (self.step - self.stamp[i]);
            let v = total / self.step.max(1);
            if v != 0 {
                out.w.insert(f.clone(), v);
            }
        }
        out
    }
}

fn path() -> std::path::PathBuf {
    crate::state::root().join("school").join("ops.tsv")
}

pub fn save(w: &Weights) -> Result<()> {
    let mut t = String::from("# the operation chooser's weights: feature, weight\n");
    for (f, v) in &w.w {
        t += &format!("{f}\t{v}\n");
    }
    std::fs::create_dir_all(path().parent().unwrap())?;
    crate::store::write_atomic(&path(), t.as_bytes())
}

static CURRENT: std::sync::Mutex<Option<std::sync::Arc<Weights>>> = std::sync::Mutex::new(None);

/// The weights in use (loaded once; `set` replaces them, as a lesson does after it learns) — shared, not copied: an
/// answer reads them, it does not clone tens of thousands of features.
pub fn current() -> std::sync::Arc<Weights> {
    let mut g = CURRENT.lock().unwrap();
    g.get_or_insert_with(|| std::sync::Arc::new(load())).clone()
}

pub fn set(w: Weights) {
    *CURRENT.lock().unwrap() = Some(std::sync::Arc::new(w));
}

pub fn load() -> Weights {
    let mut w = Weights::default();
    for l in std::fs::read_to_string(path()).unwrap_or_default().lines().filter(|l| !l.starts_with('#')) {
        if let Some((f, v)) = l.rsplit_once('\t') {
            w.w.insert(f.to_string(), v.parse().unwrap_or(0));
        }
    }
    w
}

/// The answer of a problem by the learned choice, with the reason: the two quantities, the operation, and the
/// features that carried it with their weights.
/// The k best answers the chooser sees, as (value, expression, score), best first, one per value.
pub fn top(w: &Weights, text: &str, k: usize) -> Vec<(i64, String, i64)> {
    let ms = mentions_all(text);
    let q = question_words(text);
    let mut v: Vec<(i64, String, i64)> = candidates(&ms).iter().map(|c| {
        let e = match c.then {
            None => format!("{} {} {}", ms[c.x].value, c.op.name(), ms[c.y].value),
            Some((o2, z)) => format!("({} {} {}) {} {}", ms[c.x].value, c.op.name(), ms[c.y].value, o2.name(), ms[z].value),
        };
        (c.value, e, w.score(&features(&ms, &q, c)))
    }).collect();
    v.sort_by(|a, b| b.2.cmp(&a.2).then(a.1.cmp(&b.1)));
    let mut seen = BTreeSet::new();
    v.retain(|x| seen.insert(x.0));
    v.truncate(k);
    v
}

pub fn answer(w: &Weights, text: &str) -> Option<(i64, String)> {
    let ms = mentions_all(text);
    let q = question_words(text);
    let cs = candidates(&ms);
    let c = choose(w, &ms, &q, &cs)?;
    let f = features(&ms, &q, c);
    let mut top: Vec<(i64, &String)> = f.iter().map(|x| (w.w.get(x).copied().unwrap_or(0), x)).filter(|x| x.0 != 0).collect();
    top.sort_by(|a, b| b.0.abs().cmp(&a.0.abs()).then(a.1.cmp(b.1)));
    let why = top.iter().take(4).map(|(v, x)| format!("{x} ({v:+})")).collect::<Vec<_>>().join(", ");
    let expr = match c.then {
        None => format!("{} {} {} {} {}", ms[c.x].value, ms[c.x].thing, c.op.name(), ms[c.y].value, ms[c.y].thing),
        Some((o2, z)) => format!("({} {} {} {} {}) {} {} {}", ms[c.x].value, ms[c.x].thing, c.op.name(), ms[c.y].value, ms[c.y].thing, o2.name(), ms[z].value, ms[z].thing),
    };
    Some((c.value, format!("{expr} = {} because {why}", c.value)))
}

/// Right, answered, negative control (the answer of the next problem).
pub fn test(w: &Weights, items: &[(String, i64)]) -> (usize, usize, usize) {
    let (mut right, mut answered, mut control) = (0, 0, 0);
    for (k, (t, a)) in items.iter().enumerate() {
        if let Some((v, _)) = answer(w, t) {
            answered += 1;
            right += (v == *a) as usize;
            control += (v == items[(k + 1) % items.len()].1) as usize;
        }
    }
    (right, answered, control)
}

pub fn main_args(args: &[String]) -> Result<()> {
    let default_train = format!("{}/research/raw/mwpdata-svamp/svamp-train.json", std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default())));
    let default_test = format!("{}/research/raw/mwpdata-svamp/svamp-test.json", std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default())));
    match args.get(1).map(String::as_str) {
        Some("train") => {
            let items = book(std::path::Path::new(args.get(2).filter(|x| !x.is_empty()).unwrap_or(&default_train)));
            let epochs = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(10);
            let w = train(&items, epochs);
            let (r, a, c) = test(&w, &items);
            save(&w)?;
            println!("learned from {} problems, {} weights; on them: {r} right, {a} answered, control {c}", items.len(), w.w.len());
        }
        Some("test") => {
            let items = book(std::path::Path::new(args.get(2).filter(|x| !x.is_empty()).unwrap_or(&default_test)));
            let (r, a, c) = test(&load(), &items);
            // the ceiling: problems one or two operations on the mentions can answer at all
            let reach = items.iter().filter(|(t, a)| candidates(&mentions_all(t)).iter().any(|c| c.value == *a)).count();
            println!("{} problems: {r} right ({}%), {a} answered, negative control {c}; one or two operations on the mentions can reach {reach}", items.len(), 100 * r / items.len().max(1));
        }
        Some("bench") => {
            // where an answer's time goes: the mentions, the candidates, their features, the whole choice
            let items = book(std::path::Path::new(args.get(2).filter(|x| !x.is_empty()).unwrap_or(&default_train)));
            let w = current();
            let t = std::time::Instant::now();
            let ms: Vec<Vec<Mention>> = items.iter().map(|(t, _)| mentions_all(t)).collect();
            let t_m = t.elapsed();
            let t = std::time::Instant::now();
            let cs: Vec<Vec<Cand>> = ms.iter().map(|m| candidates(m)).collect();
            let t_c = t.elapsed();
            let t = std::time::Instant::now();
            let mut nf = 0;
            for ((it, m), c) in items.iter().zip(&ms).zip(&cs) {
                let q = question_words(&it.0);
                for x in c {
                    nf += features(m, &q, x).len();
                }
            }
            let t_f = t.elapsed();
            let t = std::time::Instant::now();
            for (it, _) in &items {
                let _ = answer(&w, it);
            }
            let t_a = t.elapsed();
            println!("{} problems: mentions {t_m:?}, candidates {t_c:?} ({} in all), features {t_f:?} ({nf}), answers {t_a:?}", items.len(), cs.iter().map(|c| c.len()).sum::<usize>());
        }
        Some("explain") => {
            let text = args.iter().skip(2).cloned().collect::<Vec<_>>().join(" ");
            match answer(&load(), &text) {
                Some((v, why)) => println!("{v}: {why}"),
                None => println!("no answer"),
            }
        }
        _ => println!("usage: world opchoice train [json] [epochs] | test [json] | explain \"<problem>\""),
    }
    Ok(())
}
