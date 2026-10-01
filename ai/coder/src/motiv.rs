//! Motivated steps (architecture v1, design idea): mathematics first.
//!
//! 1. Opus annotates every step of the verified math v2 solutions: goal, motive, move, check and a
//!    "difference → operator" pair (`diff`, `kind`). Annotations are data tagged with the source `vmm`.
//! 2. The MMM learns transparently:
//!    - a "difference between state and goal → move" table (means-ends analysis, GPS of Newell and Simon);
//!    - an averaged perceptron choosing the next move from state and goal features;
//!    - subgoal order — bigrams of differences from the annotated solutions.

use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::lemma;
use crate::util::{home, read_jsonl, run_dir};

pub const DIFFS: &[&str] = &[
    "given", "combine", "candidates", "select", "transform", "aggregate", "order", "locate", "repeat", "unknown", "parts", "reuse",
    "represent", "verify", "present",
];
pub const KINDS: &[&str] = &[
    "Bind", "Arith", "Range", "Filter", "Map", "Reduce", "Sort", "Search", "Iterate", "Solve", "Decompose", "Define", "Convert", "Check",
    "Output",
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dm {
    pub diff: String,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AStep {
    pub n: usize,
    pub goal: String,
    pub motive: String,
    #[serde(rename = "move")]
    pub mv: String,
    pub check: String,
    pub moves: Vec<Dm>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ann {
    pub id: String,
    pub goal: String,
    #[serde(default)]
    pub subgoals: Vec<String>,
    pub steps: Vec<AStep>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub split: String,
    #[serde(default)]
    pub question: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MathSol {
    pub id: String,
    pub split: String,
    pub question: String,
    pub plan: serde_json::Value,
}

pub fn math_memory() -> Result<Vec<MathSol>> {
    read_jsonl(&home().join("runs/math2-2026-09-26/memory.jsonl"))
}

pub fn ann_dir() -> std::path::PathBuf {
    run_dir().join("math-ann")
}

/// Compact plan notation for the prompt: step = op + the main field.
pub fn plan_brief(p: &serde_json::Value) -> String {
    let mut out = String::new();
    if let Some(steps) = p["steps"].as_array() {
        for (i, s) in steps.iter().enumerate() {
            let mut o = s.clone();
            if let Some(m) = o.as_object_mut() {
                m.remove("id");
            }
            out.push_str(&format!("  {}. {}\n", i + 1, serde_json::to_string(&o).unwrap_or_default()));
        }
    }
    out
}

pub fn n_steps(p: &serde_json::Value) -> usize {
    p["steps"].as_array().map(|a| a.len()).unwrap_or(0)
}

/// Parsing annotator responses: one JSON per line; the step count must match the plan, labels must come from the lists.
pub fn parse_ann(text: &str, sols: &HashMap<String, MathSol>) -> (Vec<Ann>, Vec<String>) {
    let mut ok = Vec::new();
    let mut bad = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        if !l.starts_with('{') {
            continue;
        }
        let Ok(mut a) = serde_json::from_str::<Ann>(l) else {
            bad.push(format!("not JSON: {}", l.chars().take(60).collect::<String>()));
            continue;
        };
        let Some(s) = sols.get(&a.id) else {
            bad.push(format!("{}: no such solution", a.id));
            continue;
        };
        if a.steps.len() != n_steps(&s.plan) {
            bad.push(format!("{}: {} steps instead of {}", a.id, a.steps.len(), n_steps(&s.plan)));
            continue;
        }
        let labels_ok = a.steps.iter().all(|st| !st.moves.is_empty() && st.moves.iter().all(|m| DIFFS.contains(&m.diff.as_str()) && KINDS.contains(&m.kind.as_str())));
        if !labels_ok {
            bad.push(format!("{}: label not in the list", a.id));
            continue;
        }
        a.source = "vmm".into();
        a.split = s.split.clone();
        a.question = s.question.clone();
        ok.push(a);
    }
    (ok, bad)
}

/// Averaged (multiclass) perceptron with transparent weights.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Perceptron {
    pub w: HashMap<String, HashMap<String, f64>>,
    acc: HashMap<String, HashMap<String, f64>>,
    stamp: HashMap<String, HashMap<String, usize>>,
    t: usize,
}

impl Perceptron {
    pub fn score(&self, feats: &[String], class: &str) -> f64 {
        feats.iter().map(|f| self.w.get(f).and_then(|m| m.get(class)).copied().unwrap_or(0.0)).sum()
    }
    pub fn predict<'a>(&self, feats: &[String], classes: &[&'a str]) -> &'a str {
        let mut best = classes[0];
        let mut bs = f64::MIN;
        for c in classes {
            let s = self.score(feats, c);
            if s > bs {
                bs = s;
                best = c;
            }
        }
        best
    }
    fn upd(&mut self, f: &str, c: &str, d: f64) {
        let t = self.t;
        let w = self.w.entry(f.into()).or_default().entry(c.into()).or_default();
        let acc = self.acc.entry(f.into()).or_default().entry(c.into()).or_default();
        let st = self.stamp.entry(f.into()).or_default().entry(c.into()).or_default();
        *acc += (t - *st) as f64 * *w;
        *st = t;
        *w += d;
    }
    pub fn train(&mut self, feats: &[String], gold: &str, classes: &[&str]) {
        self.t += 1;
        let p = self.predict(feats, classes).to_string();
        if p != gold {
            for f in feats {
                self.upd(f, gold, 1.0);
                self.upd(f, &p, -1.0);
            }
        }
    }
    pub fn average(&mut self) {
        let t = self.t as f64;
        let keys: Vec<(String, String)> = self.w.iter().flat_map(|(f, m)| m.keys().map(move |c| (f.clone(), c.clone()))).collect();
        for (f, c) in keys {
            let w = self.w[&f][&c];
            let st = self.stamp[&f][&c];
            let acc = self.acc[&f][&c] + (self.t - st) as f64 * w;
            self.w.get_mut(&f).unwrap().insert(c, acc / t.max(1.0));
        }
    }
    /// The heaviest features in favour of a class (for the explanation).
    pub fn top(&self, feats: &[String], class: &str, k: usize) -> Vec<(String, f64)> {
        let mut v: Vec<(String, f64)> =
            feats.iter().filter_map(|f| self.w.get(f).and_then(|m| m.get(class)).map(|w| (f.clone(), *w))).filter(|(_, w)| *w != 0.0).collect();
        v.sort_by(|a, b| b.1.abs().total_cmp(&a.1.abs()));
        v.truncate(k);
        v
    }
}

/// Cue lemmas (for perceptron features and the difference detector) — from the shared dictionary.
pub const CUE_LEMMAS: &[&str] = &[
    "total", "sum", "each", "every", "many", "how", "much", "first", "last", "large", "small", "most", "least", "average", "remain",
    "left", "more", "less", "than", "time", "per", "percent", "half", "twice", "double", "digit", "prime", "divisible", "even", "odd",
    "remainder", "sort", "order", "until", "after", "each", "number", "count", "product", "difference", "ratio", "equal", "find",
    "smallest", "largest", "maximum", "minimum", "all", "some", "both", "between", "from", "to", "multiple", "factor", "square",
];

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    /// the "difference → move" table: counters
    pub table: BTreeMap<String, BTreeMap<String, usize>>,
    /// medoid motive per cell (diff, kind): (text, solution id, step)
    pub motive: BTreeMap<String, (String, String, usize, usize)>,
    /// bigrams of differences (subgoal order), "^" — start
    pub diff_next: BTreeMap<String, BTreeMap<String, usize>>,
    /// move choice: state and goal features → operator
    pub percep: Perceptron,
    /// measurement on held-out math solutions: (perceptron hit, table hit, total)
    pub eval: (usize, usize, usize),
    pub n_ann: usize,
    pub n_moves: usize,
}

fn feats(prev: &str, prev2: &str, diff: &str, cues: &[String]) -> Vec<String> {
    let mut f = vec![format!("d:{diff}"), format!("p:{prev}"), format!("p2:{prev2}"), format!("dp:{diff}|{prev}"), "bias".into()];
    for c in cues {
        f.push(format!("q:{c}|{diff}"));
    }
    f
}

pub fn question_cues(q: &str) -> Vec<String> {
    let mut v: Vec<String> = lemma::cues(q).into_iter().filter(|l| CUE_LEMMAS.contains(&l.as_str())).collect();
    v.sort();
    v.dedup();
    v
}

impl Model {
    pub fn learn(anns: &[Ann]) -> Model {
        let mut m = Model { n_ann: anns.len(), ..Default::default() };
        // table and motives
        let mut cell_motives: BTreeMap<String, Vec<(String, String, usize)>> = BTreeMap::new();
        for a in anns {
            let mut prev_d = "^".to_string();
            for st in &a.steps {
                for mv in &st.moves {
                    *m.table.entry(mv.diff.clone()).or_default().entry(mv.kind.clone()).or_default() += 1;
                    cell_motives.entry(format!("{}→{}", mv.diff, mv.kind)).or_default().push((st.motive.clone(), a.id.clone(), st.n));
                    *m.diff_next.entry(prev_d.clone()).or_default().entry(mv.diff.clone()).or_default() += 1;
                    prev_d = mv.diff.clone();
                    m.n_moves += 1;
                }
            }
        }
        // medoid: the motive sharing the most lemmas with the other motives of the cell
        for (cell, ms) in cell_motives {
            let lem: Vec<Vec<String>> = ms.iter().map(|(t, _, _)| lemma::lemmas(t)).collect();
            let mut best = 0;
            let mut bs = -1i64;
            for i in 0..ms.len() {
                let s: i64 = (0..ms.len()).filter(|&j| j != i).map(|j| lem[i].iter().filter(|w| lem[j].contains(w)).count() as i64).sum();
                if s > bs {
                    bs = s;
                    best = i;
                }
            }
            let (t, id, n) = ms[best].clone();
            m.motive.insert(cell, (t, id, n, ms.len()));
        }
        // perceptron: training on dev, measurement on held
        let seqs = |split: &str| -> Vec<(Vec<String>, Vec<(String, String)>)> {
            anns.iter()
                .filter(|a| (split == "held") == (a.split == "held"))
                .map(|a| (question_cues(&a.question), a.steps.iter().flat_map(|s| s.moves.iter().map(|m| (m.diff.clone(), m.kind.clone()))).collect()))
                .collect()
        };
        let train = seqs("dev");
        for _ in 0..10 {
            for (cues, seq) in &train {
                let (mut p, mut p2) = ("^".to_string(), "^".to_string());
                for (d, k) in seq {
                    m.percep.train(&feats(&p, &p2, d, cues), k, KINDS);
                    p2 = p;
                    p = k.clone();
                }
            }
        }
        m.percep.average();
        let (mut hit_p, mut hit_t, mut tot) = (0, 0, 0);
        for (cues, seq) in seqs("held") {
            let (mut p, mut p2) = ("^".to_string(), "^".to_string());
            for (d, k) in &seq {
                if m.percep.predict(&feats(&p, &p2, d, &cues), KINDS) == k {
                    hit_p += 1;
                }
                if m.table_best(d).as_deref() == Some(k.as_str()) {
                    hit_t += 1;
                }
                tot += 1;
                p2 = p;
                p = k.clone();
            }
        }
        m.eval = (hit_p, hit_t, tot);
        m
    }

    pub fn table_best(&self, diff: &str) -> Option<String> {
        self.table.get(diff).and_then(|r| r.iter().max_by_key(|(_, c)| **c).map(|(k, _)| k.clone()))
    }

    /// Move for a difference: perceptron by state (previous move) and cues; the explanation is the weights.
    pub fn choose(&self, diff: &str, prev: &str, prev2: &str, cues: &[String], allowed: &[&str]) -> (String, String) {
        let f = feats(prev, prev2, diff, cues);
        let allowed: Vec<&str> = if allowed.is_empty() { KINDS.to_vec() } else { allowed.to_vec() };
        let k = self.percep.predict(&f, &allowed).to_string();
        let top = self.percep.top(&f, &k, 3).iter().map(|(f, w)| format!("{f} {w:+.2}")).collect::<Vec<_>>().join(", ");
        let n = self.table.get(diff).and_then(|r| r.get(&k)).copied().unwrap_or(0);
        (k, format!("perceptron [{top}]; table {diff}→ this move seen {n}×"))
    }

    /// Subgoal order: hard dependencies (data → selection/transformation → reduction/order/search → presentation),
    /// among the allowed ones — bigrams of differences from mathematics, ties — natural order.
    pub fn order(&self, mut diffs: Vec<String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut prev = "^".to_string();
        let tier = |d: &str| match d {
            "given" | "candidates" | "reuse" => 0,
            "parts" | "repeat" | "unknown" | "combine" => 1,
            "select" | "transform" | "represent" => 2,
            "order" | "aggregate" | "locate" => 3,
            "verify" => 4,
            _ => 5,
        };
        while !diffs.is_empty() {
            let row = self.diff_next.get(&prev);
            let min_tier = diffs.iter().map(|d| tier(d)).min().unwrap_or(0);
            let (i, _) = diffs
                .iter()
                .enumerate()
                .filter(|(_, d)| tier(d) == min_tier)
                .map(|(i, d)| (i, row.and_then(|r| r.get(d)).copied().unwrap_or(0) as i64 * 1000 - canon(d) as i64))
                .max_by_key(|(_, s)| *s)
                .unwrap();
            let d = diffs.remove(i);
            prev = d.clone();
            out.push(d);
        }
        out
    }

    pub fn motive_of(&self, diff: &str, kind: &str) -> Option<&(String, String, usize, usize)> {
        self.motive.get(&format!("{diff}→{kind}"))
    }
}

/// Natural order (when there are no bigrams): data → selection → transformation → order → reduction → search → presentation.
pub fn canon(d: &str) -> usize {
    DIFFS.iter().position(|x| *x == d).unwrap_or(99)
}
