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

pub(crate) struct Item {
    pub(crate) id: usize,
    pub(crate) story: Vec<String>,
    pub(crate) question: String,
    pub(crate) label: String,
}

fn load(path: &Path) -> Result<Vec<Item>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    if path.extension().is_some_and(|e| e == "txt") {
        return Ok(load_txt(&text));
    }
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

/// The bAbI-style text of the corrected StepGame (Li, Hogg & Cohn 2024): numbered sentences, a question line
/// "N question<TAB>answer<TAB>support"; a line numbered 1 starts a new example.
pub(crate) fn load_txt(text: &str) -> Vec<Item> {
    let mut out = Vec::new();
    let mut story: Vec<String> = Vec::new();
    for line in text.lines() {
        let Some((n, rest)) = line.split_once(' ') else { continue };
        if n == "1" {
            story.clear();
        }
        let cols: Vec<&str> = rest.split('\t').collect();
        if cols.len() >= 2 {
            out.push(Item { id: out.len(), story: story.clone(), question: cols[0].trim().to_string(), label: cols[1].trim().to_string() });
        } else {
            story.push(rest.trim().to_string());
        }
    }
    out
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
    /// phrasings seen in the training data (agents as A1/A2): the perceptron reads these, the lexical reader the rest
    seen: std::collections::BTreeSet<String>,
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

    /// Vector of A1 relative to A2: a phrasing seen in training is read by the perceptron (it knows the generator's
    /// templates, clock faces included); a new phrasing by the lexical reader, when it can; otherwise the perceptron.
    pub fn read(&self, s: &str, a1: char, a2: char) -> (i32, i32) {
        if !self.seen.contains(&phrasing(s)) && std::env::var("STEPGAME_NO_LEXICAL").is_err() {
            if let Some(v) = lexical(s, a1, a2) {
                return v;
            }
        }
        LABELS[self.score(&feats(s, a1, a2))].1
    }
}

/// Direction words of level 1 (English): the vector each one points to.
fn direction(w: &str) -> Option<(i32, i32)> {
    Some(match w {
        "left" | "west" | "western" | "westward" => (-1, 0),
        "right" | "east" | "eastern" | "eastward" => (1, 0),
        "up" | "above" | "over" | "atop" | "top" | "north" | "northern" | "northward" | "higher" | "upper" | "upward" | "upwards" => (0, 1),
        "down" | "below" | "under" | "beneath" | "underneath" | "bottom" | "south" | "southern" | "southward" | "lower" | "downward" | "downwards" => (0, -1),
        "northwest" | "northwestern" => (-1, 1),
        "northeast" | "northeastern" => (1, 1),
        "southwest" | "southwestern" => (-1, -1),
        "southeast" | "southeastern" => (1, -1),
        _ => return None,
    })
}

/// The lexical reader for phrasings the training data never had. Direction: the sum of the direction words (a
/// "right" before on/under/where/at… means "directly", not a side); none, but a sameness cue (same, coincide,
/// overlap, share, where) → overlap. Frame: the ground (reference) is the agent after of/from/than/with/as/under/
/// over/…, or the subject of have/keep ("B has A on its left"); without a cue the second agent is the ground.
/// Returns the vector of A1 relative to A2, or None when it cannot tell (clock faces, conflicting words).
pub(crate) fn lexical(s: &str, a1: char, a2: char) -> Option<(i32, i32)> {
    let toks: Vec<String> = s.split(|c: char| !c.is_alphanumeric() && c != '\'').filter(|t| !t.is_empty()).map(|t| t.to_string()).collect();
    let is_agent = |t: &str| t.len() == 1 && t.chars().next().is_some_and(|c| c == a1 || c == a2);
    let low: Vec<String> = toks.iter().map(|t| if is_agent(t) { t.clone() } else { t.to_lowercase() }).collect();
    if low.iter().any(|t| t.contains("clock")) {
        return None;
    }
    let intens = ["on", "under", "over", "above", "below", "where", "at", "next", "beside", "by", "in", "there", "here", "behind", "beneath", "underneath", "atop"];
    let (mut h, mut v) = (Vec::new(), Vec::new());
    for (i, t) in low.iter().enumerate() {
        if t == "right" && low.get(i + 1).is_some_and(|n| intens.contains(&n.as_str())) {
            continue;
        }
        if let Some(d) = direction(t) {
            if d.0 != 0 {
                h.push(d.0);
            }
            if d.1 != 0 {
                v.push(d.1);
            }
        }
    }
    let axis = |xs: &[i32]| -> Option<i32> {
        if xs.iter().any(|x| *x > 0) && xs.iter().any(|x| *x < 0) { None } else { Some(xs.first().copied().unwrap_or(0)) }
    };
    let d = (axis(&h)?, axis(&v)?);
    if d == (0, 0) {
        let same = low.iter().any(|t| t == "same" || t.starts_with("coincid") || t.starts_with("overlap") || t.starts_with("share") || t == "where" || t == "identical");
        if !same {
            return None;
        }
        return Some((0, 0));
    }
    // the frame: which agent is the ground
    let fillers = ["the", "a", "an", "object", "agent", "labeled", "labelled", "point", "of"];
    let cue = ["of", "from", "than", "with", "as", "under", "over", "above", "below", "beneath", "underneath", "atop", "where", "beside", "behind", "past"];
    let mut grounds: Vec<char> = Vec::new();
    for (i, t) in low.iter().enumerate() {
        if !is_agent(t) {
            continue;
        }
        // the previous word that is not a filler (but "of" itself counts as a cue)
        let mut j = i;
        while j > 0 {
            j -= 1;
            if cue.contains(&low[j].as_str()) {
                grounds.push(t.chars().next().unwrap());
                break;
            }
            if !fillers.contains(&low[j].as_str()) {
                break;
            }
        }
    }
    let ground = if grounds.len() == 1 {
        grounds[0]
    } else if let Some(i) = low.iter().position(|t| matches!(t.as_str(), "has" | "have" | "keeps" | "keep" | "holds")) {
        // "B has A on its left": the subject is the ground
        low[..i].iter().rev().find(|t| is_agent(t)).and_then(|t| t.chars().next()).unwrap_or(a2)
    } else {
        a2
    };
    Some(if ground == a2 { d } else { (-d.0, -d.1) })
}

/// A level-1 sentence as an example: (sentence, A1, A2, class of A1 relative to A2).
fn example(it: &Item) -> Option<(String, char, char, usize)> {
    let q = agents(&it.question);
    // a one-step story may carry distractor sentences (the noise variant): the sentence naming both queried agents
    let s = if it.story.len() == 1 {
        it.story[0].clone()
    } else {
        let mut hit = it.story.iter().filter(|x| q.len() == 2 && { let a = agents(x); a.len() == 2 && a.contains(&q[0]) && a.contains(&q[1]) });
        match (hit.next(), hit.next()) {
            (Some(x), None) => x.clone(),
            _ => it.story.join(" "),
        }
    };
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
    r.seen = ex.iter().map(|e| phrasing(&e.0)).collect();
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
    // the original release is JSON; the corrected one (Li, Hogg & Cohn 2024) is bAbI-style text
    let ext = if dir.join("qa1_train.txt").exists() { "txt" } else { "json" };
    let r = train(&dir.join(format!("qa1_train.{ext}")), epochs)?;
    if std::env::var("STEPGAME_ERR").is_ok() {
        // reader errors on level 1 of the test, by template (agents → A1/A2)
        let mut bad: BTreeMap<String, (usize, usize, String)> = BTreeMap::new();
        for it in load(&dir.join(format!("qa1_test.{ext}")))? {
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
    let mut per_k: Vec<f64> = Vec::new();
    // excluded by name with a reason (the Scientist's advice: so that "without faulty" does not read as cleaning up the test)
    let mut excl = String::from("# StepGame clean test: questions whose story contains a phrasing contradictory in the training data.\n# k\tid in file qa{k}_test.json\tphrasing numbers (see the end)\twhether the snake was correct\n");
    for k in 1..=10 {
        let items = load(&dir.join(format!("qa{k}_test.{ext}")))?;
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
        per_k.push(100.0 * ok as f64 / items.len().max(1) as f64);
    }
    // the corrected splits differ in size per k: the macro mean over k is the comparable number
    println!("total {}/{} ({:.1}%), macro mean over k {:.1}%", all.0, all.1, 100.0 * all.0 as f64 / all.1 as f64, per_k.iter().sum::<f64>() / per_k.len().max(1) as f64);
    for (i, t) in r.suspect.iter().enumerate() {
        excl.push_str(&format!("# phrasing {i}: «{t}» — in qa1_train.json the same sentence has different labels (a StepGame generator flaw)\n"));
    }
    if let Ok(p) = std::env::var("STEPGAME_EXCLUDED") {
        std::fs::write(&p, excl)?;
        println!("excluded → {p}");
    }
    Ok(())
}

/// A sentence with its agents replaced by A1/A2 in order of mention: the phrasing, for overlap checks.
pub(crate) fn phrasing(s: &str) -> String {
    let a = agents(s);
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| match t.chars().next().filter(|_| t.len() == 1).and_then(|c| a.iter().position(|x| *x == c)) {
            Some(i) => format!("A{}", i + 1),
            None => t.to_lowercase(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `world stepgame-contrast <train dir> [per k]`: a contrast set (Gardner 2020) for StepGame — the same task,
/// phrasings never produced by the StepGame generator (`data/stepgame-contrast.tsv`, including the reverse view
/// "B has A on its left"), chains of k = 1..10 steps, deterministic. The reader is trained on the original qa1_train
/// only. Gate first: none of the contrast phrasings may occur in the training data.
pub fn contrast(dir: &Path, per_k: usize, test: bool) -> Result<()> {
    let ext = if dir.join("qa1_train.txt").exists() { "txt" } else { "json" };
    let train_path = dir.join(format!("qa1_train.{ext}"));
    let src = if test { include_str!("../data/stepgame-contrast-test.tsv") } else { include_str!("../data/stepgame-contrast.tsv") };
    println!("contrast set: {}", if test { "TEST (never used to build the reader)" } else { "dev" });
    let tpl: Vec<(usize, String)> = src
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let (lab, t) = l.split_once('\t')?;
            Some((LABELS.iter().position(|x| x.0 == lab)?, t.to_string()))
        })
        .collect();
    // gate: the contrast phrasings are new
    let seen: std::collections::BTreeSet<String> = load(&train_path)?.iter().flat_map(|it| it.story.iter().map(|s| phrasing(s))).collect();
    let fill = |t: &str, a: char, b: char| t.replace("{A}", &a.to_string()).replace("{B}", &b.to_string());
    let old: Vec<&String> = tpl.iter().map(|x| &x.1).filter(|t| seen.contains(&phrasing(&fill(t, 'Q', 'Z')))).collect();
    println!("contrast phrasings {} ({} relations), seen in training: {}", tpl.len(), LABELS.len(), old.len());
    for t in &old {
        println!("  already in training, left out: {t}");
    }
    // the contrast uses only phrasings with zero overlap with the training data
    let old: Vec<String> = old.into_iter().cloned().collect();
    let tpl: Vec<(usize, String)> = tpl.into_iter().filter(|x| !old.contains(&x.1)).collect();
    println!("contrast phrasings used: {}", tpl.len());
    let r = train(&train_path, 10)?;
    // deterministic generator (LCG)
    let mut seed: u64 = 0x5eed_2026_1003;
    let mut next = |n: usize| -> usize {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as usize) % n
    };
    let letters: Vec<char> = ('A'..='Z').collect();
    let (mut macro_sum, mut all) = (0.0, (0, 0));
    let mut per_tpl: Vec<(usize, usize)> = vec![(0, 0); tpl.len()];
    for k in 1..=10 {
        let mut ok = 0;
        for _ in 0..per_k {
            // k+1 distinct agents on a chain; each step one relation with a phrasing chosen at random
            let mut ag: Vec<char> = Vec::new();
            while ag.len() < k + 1 {
                let c = letters[next(letters.len())];
                if !ag.contains(&c) {
                    ag.push(c);
                }
            }
            let mut story: Vec<(String, usize)> = Vec::new();
            let mut sum = (0, 0);
            for i in 0..k {
                let ti = next(tpl.len());
                let (lab, t) = &tpl[ti];
                // sentence: ag[i] relative to ag[i+1]
                let v = LABELS[*lab].1;
                sum = (sum.0 + v.0, sum.1 + v.1);
                story.push((fill(t, ag[i], ag[i + 1]), ti));
            }
            // shuffle the sentences (StepGame does too)
            for i in (1..story.len()).rev() {
                let j = next(i + 1);
                story.swap(i, j);
            }
            let it = Item { id: 0, story: story.iter().map(|x| x.0.clone()).collect(), question: format!("What is the relation of the agent {} to the agent {}?", ag[0], ag[k]), label: label_of(sum).to_string() };
            let got = answer(&r, &it);
            let right = got.as_ref().is_some_and(|x| x.0 == it.label);
            if std::env::var("STEPGAME_SHOW").is_ok() && k == 1 && !right {
                println!("  {:?} ? {} → {:?} (expected {})", it.story, it.question, got, it.label);
            }
            ok += usize::from(right);
            if k == 1 {
                per_tpl[story[0].1].1 += 1;
                per_tpl[story[0].1].0 += usize::from(right);
            }
        }
        println!("k={k:<2} {ok}/{per_k} ({:.1}%)", 100.0 * ok as f64 / per_k as f64);
        macro_sum += 100.0 * ok as f64 / per_k as f64;
        all.0 += ok;
        all.1 += per_k;
    }
    println!("contrast total {}/{} ({:.1}%), macro mean over k {:.1}%", all.0, all.1, 100.0 * all.0 as f64 / all.1 as f64, macro_sum / 10.0);
    println!("phrasings read wrong at k=1 (wrong/tried):");
    for (i, (o, n)) in per_tpl.iter().enumerate() {
        if *n > 0 && o < n {
            println!("  {}/{} {:<12} {}", n - o, n, LABELS[tpl[i].0].0, tpl[i].1);
        }
    }
    Ok(())
}
