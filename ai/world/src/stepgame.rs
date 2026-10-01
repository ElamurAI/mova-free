//! StepGame (Shi, Zhang, Lipani 2022; MIT) — relative placement of agents on a grid, 1…10 steps.
//! Tests level 3 on livelier language than bAbI: 418 different phrasings per relation.
//!
//! - **Sentence reader** (the snake learns): a sentence with two agents → vector of the first mentioned agent
//!   relative to the second (9 classes: 8 directions + overlap). Averaged perceptron on word n-grams in
//!   which the agents are replaced with A1/A2; integer weights — deterministic. Learns only on level 1 (one sentence =
//!   one relation, the question label is the sentence's direct label).
//! - **World** (level 3): vectors are edges of the agent graph; the answer is the sum of vectors along the path from
//!   the queried agent to the reference one, sign per axis → class. The knowledge "how directions compose" is the vectors.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::path::Path;

use anyhow::{Context, Result};

pub const LABELS: &[(&str, (i32, i32))] = &[
    ("left", (-1, 0)),
    ("right", (1, 0)),
    ("above", (0, 1)),
    ("below", (0, -1)),
    ("upper-left", (-1, 1)),
    ("upper-right", (1, 1)),
    ("lower-left", (-1, -1)),
    ("lower-right", (1, -1)),
    ("overlap", (0, 0)),
];

fn label_of(v: (i32, i32)) -> &'static str {
    let s = (v.0.signum(), v.1.signum());
    LABELS.iter().find(|(_, x)| *x == s).unwrap().0
}

fn vec_of(l: &str) -> Option<(i32, i32)> {
    LABELS.iter().find(|(n, _)| *n == l).map(|x| x.1)
}

struct Item {
    id: usize,
    story: Vec<String>,
    question: String,
    label: String,
}

fn load(path: &Path) -> Result<Vec<Item>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let m: BTreeMap<String, serde_json::Value> = serde_json::from_str(&text)?;
    // keys are numbers as strings; order is numeric
    let mut v: Vec<(usize, Item)> = m
        .into_iter()
        .map(|(k, it)| {
            let story = it["story"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
            let question = it["question"].as_str().unwrap_or("").to_string();
            let label = it["label"].as_str().unwrap_or("").to_string();
            let id = k.parse().unwrap_or(0);
            (id, Item { id, story, question, label })
        })
        .collect();
    v.sort_by_key(|x| x.0);
    Ok(v.into_iter().map(|x| x.1).collect())
}

/// Agents are single capital letters.
fn agents(s: &str) -> Vec<char> {
    let mut out = Vec::new();
    for t in s.split(|c: char| !c.is_alphanumeric()) {
        let mut ch = t.chars();
        if let (Some(c), None) = (ch.next(), ch.next()) {
            if c.is_ascii_uppercase() && !out.contains(&c) {
                out.push(c);
            }
        }
    }
    out
}

/// Sentence features: word n-grams (1–3) with agents A1/A2.
fn feats(s: &str, a1: char, a2: char) -> Vec<String> {
    let toks: Vec<String> = s
        .split(|c: char| c.is_whitespace() || c == ',' || c == '.')
        .filter(|t| !t.is_empty())
        .map(|t| {
            let mut ch = t.chars();
            match (ch.next(), ch.next()) {
                (Some(c), None) if c == a1 => "A1".to_string(),
                (Some(c), None) if c == a2 => "A2".to_string(),
                _ => t.to_lowercase(),
            }
        })
        .collect();
    let mut f = vec!["bias".to_string()];
    for n in 1..=3 {
        for w in toks.windows(n) {
            f.push(w.join(" "));
        }
    }
    f
}

/// Averaged perceptron over 9 classes, integer weights.
#[derive(Default)]
pub struct Reader {
    w: HashMap<String, [i64; 9]>,
    acc: HashMap<String, [i64; 9]>,
    last: HashMap<String, [i64; 9]>,
    t: i64,
    avg: bool,
    /// phrasings that are contradictory in the training data
    pub suspect: Vec<String>,
}

impl Reader {
    fn score(&self, f: &[String]) -> usize {
        let mut s = [0i64; 9];
        for x in f {
            if let Some(w) = self.w.get(x) {
                for k in 0..9 {
                    s[k] += w[k];
                }
            }
        }
        (0..9).max_by_key(|&k| (s[k], std::cmp::Reverse(k))).unwrap()
    }

    fn update(&mut self, f: &[String], gold: usize, pred: usize) {
        self.t += 1;
        if gold == pred {
            return;
        }
        for x in f {
            let t = self.t;
            let w = self.w.entry(x.clone()).or_insert([0; 9]);
            let a = self.acc.entry(x.clone()).or_insert([0; 9]);
            let l = self.last.entry(x.clone()).or_insert([0; 9]);
            for k in [gold, pred] {
                a[k] += (t - l[k]) * w[k];
                l[k] = t;
            }
            w[gold] += 1;
            w[pred] -= 1;
        }
    }

    fn finish(&mut self) {
        let t = self.t;
        for (x, w) in self.w.iter_mut() {
            let a = self.acc.get_mut(x).unwrap();
            let l = &self.last[x];
            for k in 0..9 {
                a[k] += (t - l[k]) * w[k];
                // average, scaled (integer): we only compare relatively
                w[k] = a[k];
            }
        }
        self.avg = true;
    }

    /// Vector of A1 relative to A2.
    pub fn read(&self, s: &str, a1: char, a2: char) -> (i32, i32) {
        LABELS[self.score(&feats(s, a1, a2))].1
    }
}

/// A level-1 sentence as an example: (sentence, A1, A2, class of A1 relative to A2).
fn example(it: &Item) -> Option<(String, char, char, usize)> {
    let s = it.story.join(" ");
    let q = agents(&it.question);
    let a = agents(&s);
    if q.len() != 2 || a.len() != 2 {
        return None;
    }
    let v = vec_of(&it.label)?;
    // question: q0 relative to q1; A1 is the first mentioned in the sentence
    let (a1, a2) = (a[0], a[1]);
    let v = if a1 == q[0] { v } else { (-v.0, -v.1) };
    let k = LABELS.iter().position(|x| x.1 == v)?;
    Some((s, a1, a2, k))
}

pub fn train(path: &Path, epochs: usize) -> Result<Reader> {
    let items = load(path)?;
    let ex: Vec<_> = items.iter().filter_map(example).collect();
    let fx: Vec<Vec<String>> = ex.iter().map(|e| feats(&e.0, e.1, e.2)).collect();
    let mut r = Reader::default();
    for ep in 0..epochs {
        let mut ok = 0;
        // deterministic order with a stride (no random generator)
        let n = ex.len();
        for j in 0..n {
            let i = (j * 7919 + ep * 104729) % n;
            let p = r.score(&fx[i]);
            ok += usize::from(p == ex[i].3);
            r.update(&fx[i], ex[i].3, p);
        }
        eprintln!("epoch {}: correct on training {ok}/{n}", ep + 1);
    }
    r.finish();
    // suspicions (examples are not truth): the same phrasing with different labels in the training data
    let mut by: BTreeMap<String, [usize; 9]> = BTreeMap::new();
    for e in &ex {
        let t = feats(&e.0, e.1, e.2).into_iter().filter(|f| !f.contains(' ') && f != "bias").collect::<Vec<_>>().join(" ");
        by.entry(t).or_insert([0; 9])[e.3] += 1;
    }
    for (t, c) in &by {
        let n: usize = c.iter().sum();
        let top = *c.iter().max().unwrap();
        if n >= 10 && top * 10 < n * 6 {
            let labs: Vec<String> = (0..9).filter(|&k| c[k] > 0).map(|k| format!("{} {}", LABELS[k].0, c[k])).collect();
            eprintln!("suspicion: phrasing contradictory in the data ({n} examples: {}) — «{t}»", labs.join(", "));
            r.suspect.push(t.clone());
        }
    }
    Ok(r)
}

/// Numbers of contradictory phrasings (`Reader::suspect`) present in the story.
fn suspects_in(r: &Reader, it: &Item) -> Vec<usize> {
    let mut v: Vec<usize> = it
        .story
        .iter()
        .filter_map(|s| {
            let a = agents(s);
            if a.len() != 2 {
                return None;
            }
            let t = feats(s, a[0], a[1]).into_iter().filter(|f| !f.contains(' ') && f != "bias").collect::<Vec<_>>().join(" ");
            r.suspect.iter().position(|x| *x == t)
        })
        .collect();
    v.sort();
    v.dedup();
    v
}

/// Answer: a world of vectors, sum along the path.
pub fn answer(r: &Reader, it: &Item) -> Option<(&'static str, String)> {
    let q = agents(&it.question);
    if q.len() != 2 {
        return None;
    }
    let mut adj: BTreeMap<char, Vec<(char, (i32, i32))>> = BTreeMap::new();
    for s in &it.story {
        let a = agents(s);
        if a.len() != 2 {
            continue;
        }
        let v = r.read(s, a[0], a[1]);
        // a0 = a1 + v
        adj.entry(a[1]).or_default().push((a[0], v));
        adj.entry(a[0]).or_default().push((a[1], (-v.0, -v.1)));
    }
    // positions relative to q1
    let mut pos = BTreeMap::from([(q[1], (0, 0))]);
    let mut dq = VecDeque::from([q[1]]);
    while let Some(x) = dq.pop_front() {
        let p = pos[&x];
        for (y, v) in adj.get(&x).cloned().unwrap_or_default() {
            if let std::collections::btree_map::Entry::Vacant(e) = pos.entry(y) {
                e.insert((p.0 + v.0, p.1 + v.1));
                dq.push_back(y);
            }
        }
    }
    let d = *pos.get(&q[0])?;
    let sus = suspects_in(r, it).len();
    let note = if sus > 0 { format!("; {sus} sentences from phrasings contradictory in the data — answer in doubt") } else { String::new() };
    Some((label_of(d), format!("{} relative to {}: offset {d:?}{note}", q[0], q[1])))
}

pub fn eval(dir: &Path, epochs: usize, show: usize) -> Result<()> {
    let r = train(&dir.join("qa1_train.json"), epochs)?;
    if std::env::var("STEPGAME_ERR").is_ok() {
        // reader errors on level 1 of the test, by template (agents → A1/A2)
        let mut bad: BTreeMap<String, (usize, usize, String)> = BTreeMap::new();
        for it in load(&dir.join("qa1_test.json"))? {
            let Some((s, a1, a2, k)) = example(&it) else { continue };
            let t = feats(&s, a1, a2).into_iter().filter(|f| f.split(' ').count() == 1 && f != "bias").collect::<Vec<_>>().join(" ");
            let p = r.score(&feats(&s, a1, a2));
            let e = bad.entry(t).or_insert((0, 0, LABELS[k].0.to_string()));
            e.1 += 1;
            if p != k {
                e.0 += 1;
            }
        }
        let mut v: Vec<_> = bad.into_iter().filter(|x| x.1 .0 > 0).collect();
        v.sort_by_key(|x| std::cmp::Reverse(x.1 .0));
        for (t, (b, n, g)) in v.iter().take(25) {
            println!("{b}/{n} {g:<12} {t}");
        }
    }
    let mut all = (0, 0);
    // excluded by name with a reason (the Scientist's advice: so that "without faulty" does not read as cleaning up the test)
    let mut excl = String::from("# StepGame clean test: questions whose story contains a phrasing contradictory in the training data.\n# k\tid in file qa{k}_test.json\tphrasing numbers (see the end)\twhether the snake was correct\n");
    for k in 1..=10 {
        let items = load(&dir.join(format!("qa{k}_test.json")))?;
        let mut ok = 0;
        let (mut sus, mut sus_ok) = (0, 0);
        let mut shown = 0;
        for it in &items {
            let a = answer(&r, it);
            if a.as_ref().is_some_and(|x| x.1.contains("in doubt")) {
                let ids: Vec<String> = suspects_in(&r, it).iter().map(|x| x.to_string()).collect();
                excl.push_str(&format!("{k}\t{}\t{}\t{}\n", it.id, ids.join(","), u8::from(a.as_ref().is_some_and(|x| x.0 == it.label))));
                sus += 1;
                sus_ok += usize::from(a.as_ref().is_some_and(|x| x.0 == it.label));
            }
            if a.as_ref().is_some_and(|x| x.0 == it.label) {
                ok += 1;
            } else if shown < show {
                shown += 1;
                println!("  {:?}\n  ? {} → {a:?} (expected {})", it.story, it.question, it.label);
            }
        }
        let clean = items.len() - sus;
        println!(
            "k={k:<2} {ok}/{} ({:.1}%); without doubtful {}/{clean} ({:.1}%), doubtful {sus}",
            items.len(),
            100.0 * ok as f64 / items.len() as f64,
            ok - sus_ok,
            100.0 * (ok - sus_ok) as f64 / clean.max(1) as f64
        );
        all.0 += ok;
        all.1 += items.len();
    }
    println!("total {}/{} ({:.1}%)", all.0, all.1, 100.0 * all.0 as f64 / all.1 as f64);
    for (i, t) in r.suspect.iter().enumerate() {
        excl.push_str(&format!("# phrasing {i}: «{t}» — in qa1_train.json the same sentence has different labels (a StepGame generator flaw)\n"));
    }
    if let Ok(p) = std::env::var("STEPGAME_EXCLUDED") {
        std::fs::write(&p, excl)?;
        println!("excluded → {p}");
    }
    Ok(())
}
