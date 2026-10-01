//! v2: learning — a memory of verified solutions, not weights in a black box. Each solution:
//! problem features, method, plan, trace. The method is chosen by an averaged perceptron over problem features —
//! transparent: you can see which features and with what weight won. A plan template is a verified plan in which
//! the numbers from the problem are replaced by slots `#k`; for a similar problem the MMM substitutes its numbers and executes the plan
//! by itself, without the LLM — and the answer passes the same gate.

use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::big::*;
use crate::curric::Problem;

const STOP: &[&str] = &[
    "the", "and", "for", "are", "was", "were", "has", "have", "had", "his", "her", "she", "him", "they", "them", "their", "that", "this", "with", "what", "which", "how",
    "many", "much", "does", "did", "each", "from", "into", "than", "then", "there", "will", "would", "can", "could", "you", "your", "its", "all", "any", "but", "not",
    "our", "who", "whom", "also", "some", "more", "most", "such", "only", "same", "other", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn stem(w: &str) -> String {
    for suf in ["ing", "ed", "es", "s"] {
        if w.len() > suf.len() + 3 && w.ends_with(suf) {
            return w[..w.len() - suf.len()].to_string();
        }
    }
    w.to_string()
}

pub const NUM_WORDS: &[(&str, i64)] = &[
    ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("ten", 10), ("eleven", 11), ("twelve", 12),
    ("fifteen", 15), ("twenty", 20), ("thirty", 30), ("forty", 40), ("fifty", 50), ("hundred", 100), ("dozen", 12), ("twice", 2), ("half", 2), ("thrice", 3), ("double", 2),
    ("triple", 3), ("quarter", 4),
];

/// Numbers from the problem in order of appearance (as digits and as words).
pub fn numbers(text: &str) -> Vec<Q> {
    let mut out = Vec::new();
    let cs: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_ascii_digit() {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || ((cs[i] == ',' || cs[i] == '.') && i + 1 < cs.len() && cs[i + 1].is_ascii_digit())) {
                i += 1;
            }
            let s: String = cs[st..i].iter().collect();
            let s = if s.contains(',') && s.split(',').skip(1).all(|g| g.len() == 3) { s.replace(',', "") } else { s.split(',').next().unwrap_or("").to_string() };
            if let Some(x) = parse_q(&s) {
                out.push(x);
            }
            continue;
        }
        if c.is_alphabetic() {
            let st = i;
            while i < cs.len() && cs[i].is_alphabetic() {
                i += 1;
            }
            let w: String = cs[st..i].iter().collect::<String>().to_lowercase();
            if let Some((_, v)) = NUM_WORDS.iter().find(|(k, _)| *k == w) {
                out.push(q(*v));
            }
            continue;
        }
        i += 1;
    }
    out
}

/// Problem features for the perceptron.
pub fn features(p: &Problem) -> Vec<String> {
    let mut f: Vec<String> = Vec::new();
    let text = p.question.to_lowercase();
    let mut seen = HashSet::new();
    for w in text.split(|c: char| !c.is_alphabetic()) {
        if w.len() >= 3 && !STOP.contains(&w) {
            let s = stem(w);
            if seen.insert(s.clone()) {
                f.push(format!("w:{s}"));
            }
        }
    }
    let nums = numbers(&p.question);
    f.push(format!("n:{}", nums.len().min(8)));
    let mag = nums.iter().map(|x| to_f64(x).abs()).fold(0.0f64, f64::max);
    f.push(format!("mag:{}", if mag < 1.0 { 0 } else { mag.log10() as i64 + 1 }.min(12)));
    for (k, pat) in [("pct", "%"), ("pct", "percent"), ("usd", "$"), ("frac", "/"), ("eq", "="), ("pow", "^"), ("x", " x "), ("latex", "\\")] {
        if text.contains(pat) {
            f.push(format!("has:{k}"));
        }
    }
    if !p.options.is_empty() {
        f.push("has:options".into());
    }
    f.push(format!("lvl:{}", p.level));
    f.dedup();
    f
}

/// Memory record: a verified solution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Memo {
    pub id: String,
    pub level: u8,
    pub split: String,
    pub source: String,
    pub question: String,
    pub feats: Vec<String>,
    pub method: String,
    pub plan: Value,
    pub answer: String,
    /// match with the reference (known only in measurement; not used in learning)
    pub correct: bool,
    pub checks_ok: usize,
    pub checks_total: usize,
    /// plan template with slots #k (if the plan contains numbers from the problem)
    #[serde(default)]
    pub template: Option<Value>,
    #[serde(default)]
    pub nums: usize,
}

// ---------- perceptron ----------

#[derive(Default, Serialize, Deserialize)]
pub struct Perceptron {
    /// class → feature → weight (averaged)
    pub w: BTreeMap<String, HashMap<String, f64>>,
    pub classes: Vec<String>,
    pub trained_on: usize,
}

pub struct Guess {
    pub method: String,
    pub margin: f64,
    /// the features that weighed most, with their weights
    pub why: Vec<(String, f64)>,
}

impl Perceptron {
    pub fn train(memos: &[&Memo], epochs: usize) -> Perceptron {
        let mut classes: Vec<String> = memos.iter().map(|m| m.method.clone()).collect::<HashSet<_>>().into_iter().collect();
        classes.sort();
        let mut w: HashMap<(String, String), f64> = HashMap::new();
        let mut acc: HashMap<(String, String), f64> = HashMap::new();
        let mut stamp: HashMap<(String, String), usize> = HashMap::new();
        let mut t = 1usize;
        let mut order: Vec<&&Memo> = memos.iter().collect();
        order.sort_by(|a, b| a.id.cmp(&b.id));
        for _ in 0..epochs {
            for m in &order {
                let score = |c: &String, w: &HashMap<(String, String), f64>| m.feats.iter().map(|f| w.get(&(c.clone(), f.clone())).copied().unwrap_or(0.0)).sum::<f64>();
                let pred = classes.iter().max_by(|a, b| score(a, &w).partial_cmp(&score(b, &w)).unwrap().then(b.cmp(a))).cloned().unwrap_or_default();
                if pred != m.method {
                    for f in &m.feats {
                        for (c, d) in [(&m.method, 1.0), (&pred, -1.0)] {
                            let k = (c.clone(), f.clone());
                            let last = stamp.get(&k).copied().unwrap_or(0);
                            let cur = w.get(&k).copied().unwrap_or(0.0);
                            *acc.entry(k.clone()).or_insert(0.0) += cur * (t - last) as f64;
                            stamp.insert(k.clone(), t);
                            w.insert(k, cur + d);
                        }
                    }
                }
                t += 1;
            }
        }
        let mut out: BTreeMap<String, HashMap<String, f64>> = BTreeMap::new();
        for (k, cur) in &w {
            let last = stamp.get(k).copied().unwrap_or(0);
            let total = acc.get(k).copied().unwrap_or(0.0) + cur * (t - last) as f64;
            let avg = total / t as f64;
            if avg.abs() > 1e-9 {
                out.entry(k.0.clone()).or_default().insert(k.1.clone(), avg);
            }
        }
        Perceptron { w: out, classes, trained_on: memos.len() }
    }

    pub fn guess(&self, feats: &[String]) -> Option<Guess> {
        if self.classes.is_empty() {
            return None;
        }
        let score = |c: &String| feats.iter().map(|f| self.w.get(c).and_then(|m| m.get(f)).copied().unwrap_or(0.0)).sum::<f64>();
        let mut scored: Vec<(String, f64)> = self.classes.iter().map(|c| (c.clone(), score(c))).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
        let (best, s1) = scored[0].clone();
        let s2 = scored.get(1).map(|x| x.1).unwrap_or(0.0);
        let mut why: Vec<(String, f64)> = feats.iter().filter_map(|f| self.w.get(&best).and_then(|m| m.get(f)).map(|v| (f.clone(), *v))).collect();
        why.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        why.truncate(5);
        Some(Guess { method: best, margin: s1 - s2, why })
    }
}

// ---------- plan templates ----------

fn scan_numbers(s: &str) -> Vec<(usize, usize, String)> {
    let cs: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let prev_ident = i > 0 && (cs[i - 1].is_alphanumeric() || cs[i - 1] == '_' || cs[i - 1] == '#');
        if cs[i].is_ascii_digit() && !prev_ident {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || (cs[i] == '.' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit())) {
                i += 1;
            }
            if i < cs.len() && (cs[i].is_alphabetic() || cs[i] == '_') {
                continue; // part of an identifier
            }
            out.push((st, i, cs[st..i].iter().collect()));
            continue;
        }
        i += 1;
    }
    out
}

fn map_strings(v: &Value, f: &mut dyn FnMut(&str) -> String) -> Value {
    match v {
        Value::String(s) => Value::String(f(s)),
        Value::Number(n) => Value::String(f(&n.to_string())),
        Value::Array(xs) => Value::Array(xs.iter().map(|x| map_strings(x, f)).collect()),
        Value::Object(m) => Value::Object(m.iter().map(|(k, x)| (k.clone(), if matches!(k.as_str(), "id" | "op" | "method" | "type" | "option" | "domain" | "why") { x.clone() } else { map_strings(x, f) })).collect()),
        o => o.clone(),
    }
}

/// Problem skeleton: lowercased words, numbers → "#", no markup. Problems of the same structure have
/// almost the same skeleton.
pub fn skeleton(question: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let flush = |cur: &mut String, out: &mut Vec<String>| {
        if !cur.is_empty() {
            let w = std::mem::take(cur);
            if w.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ',') || NUM_WORDS.iter().any(|(k, _)| *k == w) {
                out.push("#".into());
            } else {
                out.push(w);
            }
        }
    };
    for c in question.to_lowercase().chars() {
        if c.is_alphanumeric() || ((c == '.' || c == ',') && cur.chars().last().is_some_and(|d| d.is_ascii_digit())) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out);
        }
    }
    flush(&mut cur, &mut out);
    out.dedup_by(|a, b| a == "#" && b == "#");
    out
}

/// Skeleton similarity: 2·LCS / (|a| + |b|) (longest common subsequence of words).
pub fn skeleton_sim(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() || b.is_empty() || a.len() > 400 || b.len() > 400 {
        return 0.0;
    }
    let mut dp = vec![vec![0u16; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            dp[i][j] = if a[i - 1] == b[j - 1] { dp[i - 1][j - 1] + 1 } else { dp[i - 1][j].max(dp[i][j - 1]) };
        }
    }
    2.0 * dp[a.len()][b.len()] as f64 / (a.len() + b.len()) as f64
}

/// Template: plan numbers that match numbers of the problem → slots `#k` (k — index of the number in the problem).
/// A template is usable only if complete and unambiguous: every problem number became a slot, and among the problem numbers
/// there is no 0 or 1 (they cannot be told apart from plan constants — `n + 1`).
pub fn make_template(plan: &Value, question: &str) -> Option<(Value, usize)> {
    let nums = numbers(question);
    if nums.iter().any(|x| *x == q(0) || *x == q(1)) {
        return None;
    }
    let mut used: HashSet<usize> = HashSet::new();
    let mut slots = 0;
    let mut t = plan.clone();
    if let Value::Object(m) = &mut t {
        m.remove("id");
        m.remove("option");
    }
    let t = map_strings(&t, &mut |s: &str| {
        let mut out = String::new();
        let mut last = 0;
        let cs: Vec<char> = s.chars().collect();
        for (a, b, lit) in scan_numbers(s) {
            out.extend(&cs[last..a]);
            let v = parse_q(&lit);
            match v.and_then(|v| nums.iter().position(|x| *x == v)) {
                Some(k) if !(lit == "0" || lit == "1") => {
                    out.push_str(&format!("#{k}"));
                    used.insert(k);
                    slots += 1;
                }
                _ => out.push_str(&lit),
            }
            last = b;
        }
        out.extend(&cs[last..]);
        out
    });
    (slots > 0 && used.len() == nums.len()).then_some((t, nums.len()))
}

pub fn instantiate(template: &Value, question: &str) -> Option<Value> {
    let nums = numbers(question);
    let mut ok = true;
    let v = map_strings(template, &mut |s: &str| {
        let mut out = String::new();
        let cs: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < cs.len() {
            if cs[i] == '#' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit() {
                let st = i + 1;
                i += 1;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
                let k: usize = cs[st..i].iter().collect::<String>().parse().unwrap_or(usize::MAX);
                match nums.get(k) {
                    Some(x) => out.push_str(&format!("({})", show(x))),
                    None => {
                        ok = false;
                    }
                }
                continue;
            }
            out.push(cs[i]);
            i += 1;
        }
        out
    });
    ok.then_some(v)
}

/// Problem similarity (Jaccard over words).
pub fn similarity(a: &[String], b: &[String]) -> f64 {
    let wa: HashSet<&String> = a.iter().filter(|f| f.starts_with("w:")).collect();
    let wb: HashSet<&String> = b.iter().filter(|f| f.starts_with("w:")).collect();
    let inter = wa.intersection(&wb).count() as f64;
    let uni = wa.union(&wb).count() as f64;
    if uni == 0.0 { 0.0 } else { inter / uni }
}

pub fn load(path: &std::path::Path) -> Vec<Memo> {
    std::fs::read_to_string(path).unwrap_or_default().lines().filter_map(|l| serde_json::from_str(l).ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates_roundtrip() {
        let q1 = "Janet has 16 eggs. She eats 3 and bakes with 4. She sells the rest for $2 each.";
        let plan: Value = serde_json::from_str(r#"{"id":"a","method":"arith","steps":[{"id":"left","op":"compute","expr":"16 - 3 - 4"},{"id":"money","op":"compute","expr":"left * 2"}],"answer":"money"}"#).unwrap();
        let (t, n) = make_template(&plan, q1).unwrap();
        assert_eq!(n, 4);
        assert!(t.to_string().contains("#0 - #1 - #2"), "{t}");
        let q2 = "Ann has 20 eggs. She eats 2 and bakes with 5. She sells the rest for $3 each.";
        let p2 = instantiate(&t, q2).unwrap();
        assert!(p2.to_string().contains("(20) - (2) - (5)"), "{p2}");
    }

    #[test]
    fn template_policy_precision_first() {
        // "1" in the problem cannot be told apart from a plan constant — the template is rejected (this is how the CRT problem broke)
        let q = "Find the smallest positive integer that leaves a remainder of 5 when divided by 8, a remainder of 1 when divided by 3, and a remainder of 7 when divided by 11.";
        let plan: Value = serde_json::from_str(r#"{"steps":[{"id":"x","op":"compute","expr":"crt([5,1,7],[8,3,11])"}],"answer":"x"}"#).unwrap();
        assert!(make_template(&plan, q).is_none());
        // incomplete template (not all problem numbers are in the plan) — rejected
        let plan: Value = serde_json::from_str(r#"{"steps":[{"id":"x","op":"compute","expr":"16 - 3"}],"answer":"x"}"#).unwrap();
        assert!(make_template(&plan, "Janet has 16 eggs. She eats 3 and bakes with 4.").is_none());
        // skeleton: the same structure — high similarity; different wording — low
        let a = skeleton("Find the smallest positive integer that leaves a remainder of 3 when divided by 5, a remainder of 4 when divided by 7, and a remainder of 2 when divided by 9.");
        let b = skeleton(q);
        assert!(skeleton_sim(&a, &b) > 0.95, "{}", skeleton_sim(&a, &b));
        let c = skeleton("How many ways are there to divide a set of 5 elements into 2 non-empty ordered subsets?");
        let d = skeleton("In how many ways can a group of 7 people be divided into 2 non-empty subsets?");
        assert!(skeleton_sim(&c, &d) < 0.9, "{}", skeleton_sim(&c, &d));
    }

    #[test]
    fn perceptron_learns() {
        let mk = |id: &str, q: &str, m: &str| Memo { id: id.into(), level: 0, split: "dev".into(), source: String::new(), question: q.into(), feats: features(&Problem { id: id.into(), level: 0, source: String::new(), license: String::new(), split: String::new(), question: q.into(), options: vec![], gold: String::new(), gold_kind: "int".into(), topic: String::new() }), method: m.into(), plan: Value::Null, answer: String::new(), correct: true, checks_ok: 0, checks_total: 0, template: None, nums: 0 };
        let ms = [mk("1", "What percent of the apples are red?", "percent"), mk("2", "The price increased by 20 percent.", "percent"), mk("3", "How many ways to choose 3 books from 10?", "counting"), mk("4", "In how many ways can 5 people sit?", "counting")];
        let refs: Vec<&Memo> = ms.iter().collect();
        let p = Perceptron::train(&refs, 10);
        let g = p.guess(&features(&Problem { id: "x".into(), level: 0, source: String::new(), license: String::new(), split: String::new(), question: "How many ways to arrange 4 books?".into(), options: vec![], gold: String::new(), gold_kind: "int".into(), topic: String::new() })).unwrap();
        assert_eq!(g.method, "counting");
        assert!(!g.why.is_empty());
    }
}
