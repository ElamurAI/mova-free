//! Verbal multiword expressions found by the SLM, as a tunable component: contiguous matches of the expression
//! base (`global::expressions_in_sentence`) and pairs over dependency arcs (verb + object or particle in the idiom
//! base `global::idiom`) — the second finds discontinuous ones ("took an important decision"). Measured on PARSEME
//! 1.3 English (verbal idioms, light-verb constructions, verb-particle constructions; annotation CC BY 4.0, UD text
//! CC BY-SA 4.0) with the gold trees of the corpus.
//!
//! A predicted expression is right if it shares two or more tokens with a gold one (the verb among them); per
//! sentence score = right − wrong predictions (for the tuning loop); precision and recall are printed too.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

pub struct Sent {
    pub form: Vec<String>,
    pub lemma: Vec<String>,
    pub upos: Vec<String>,
    pub head: Vec<usize>,
    pub rel: Vec<String>,
    /// gold expressions: token index sets with their category
    pub gold: Vec<(Vec<usize>, String)>,
}

/// Read a `.cupt` file (CoNLL-U Plus with the PARSEME:MWE column).
pub fn read_cupt(path: &Path) -> Result<Vec<Sent>> {
    let mut out = Vec::new();
    let mut cur: Vec<Vec<String>> = Vec::new();
    let flush = |cur: &mut Vec<Vec<String>>, out: &mut Vec<Sent>| {
        if cur.is_empty() {
            return;
        }
        let mut groups: BTreeMap<String, (Vec<usize>, String)> = BTreeMap::new();
        for (i, c) in cur.iter().enumerate() {
            for part in c[10].split(';').filter(|p| *p != "*" && *p != "_") {
                let (id, cat) = part.split_once(':').map_or((part, ""), |(a, b)| (a, b));
                let e = groups.entry(id.to_string()).or_insert((Vec::new(), String::new()));
                e.0.push(i);
                if !cat.is_empty() {
                    e.1 = cat.to_string();
                }
            }
        }
        out.push(Sent {
            form: cur.iter().map(|c| c[1].clone()).collect(),
            lemma: cur.iter().map(|c| c[2].to_lowercase()).collect(),
            upos: cur.iter().map(|c| c[3].clone()).collect(),
            head: cur.iter().map(|c| c[6].parse().unwrap_or(0)).collect(),
            rel: cur.iter().map(|c| c[7].clone()).collect(),
            gold: groups.into_values().collect(),
        });
        cur.clear();
    };
    for l in std::fs::read_to_string(path)?.lines() {
        if l.trim().is_empty() {
            flush(&mut cur, &mut out);
        } else if !l.starts_with('#') {
            let c: Vec<String> = l.split('\t').map(String::from).collect();
            if c.len() >= 11 && c[0].chars().all(|x| x.is_ascii_digit()) {
                cur.push(c);
            }
        }
    }
    flush(&mut cur, &mut out);
    Ok(out)
}

pub fn knobs() -> Vec<(&'static str, f64, Vec<f64>)> {
    vec![
        ("contiguous", 1.0, vec![0.0, 1.0]),
        ("kind_idiom", 1.0, vec![0.0, 1.0]),
        ("kind_verb_phrase", 1.0, vec![0.0, 1.0]),
        ("kind_phrasal_verb", 1.0, vec![0.0, 1.0]),
        ("kind_phrase", 0.0, vec![0.0, 1.0]),
        ("min_content", 2.0, vec![1.0, 2.0, 3.0]),
        ("arcs", 1.0, vec![0.0, 1.0]),
        ("arc_idiom", 1.0, vec![0.0, 1.0]),
        ("arc_light_verb", 1.0, vec![0.0, 1.0]),
        ("arc_collocation", 0.0, vec![0.0, 1.0]),
        ("arc_phrasal_verb", 1.0, vec![0.0, 1.0]),
        ("arc_candidate", 0.0, vec![0.0, 1.0]),
    ]
}

fn content(u: &str) -> bool {
    matches!(u, "NOUN" | "VERB" | "ADJ" | "ADV" | "PROPN")
}

/// Predicted expressions of a sentence under a configuration (token index sets).
pub fn predict(s: &Sent, cfg: &BTreeMap<String, f64>) -> Vec<Vec<usize>> {
    let k = |n: &str| cfg.get(n).copied().unwrap_or(0.0) > 0.5;
    let num = |n: &str| cfg.get(n).copied().unwrap_or(0.0);
    let mut out: Vec<Vec<usize>> = Vec::new();
    if k("contiguous") {
        let fr: Vec<String> = s.form.iter().map(|f| f.to_lowercase()).collect();
        let f: Vec<&str> = fr.iter().map(String::as_str).collect();
        let l: Vec<&str> = s.lemma.iter().map(String::as_str).collect();
        for m in global::expressions_in_sentence(&f, &l) {
            if m.end - m.start < 2 {
                continue;
            }
            let kind_ok = match m.entry.kind {
                "idiom" => k("kind_idiom"),
                "verb-phrase" => k("kind_verb_phrase"),
                "phrasal-verb" => k("kind_phrasal_verb"),
                "phrase" => k("kind_phrase"),
                _ => false,
            };
            let span: Vec<usize> = (m.start..m.end).collect();
            let has_verb = span.iter().any(|&i| s.upos[i] == "VERB");
            if kind_ok && has_verb && span.iter().filter(|&&i| content(&s.upos[i])).count() as f64 >= num("min_content").min(span.len() as f64) {
                out.push(span);
            }
        }
    }
    if k("arcs") {
        for i in 0..s.form.len() {
            let h = s.head[i];
            if h == 0 || !matches!(s.rel[i].as_str(), "obj" | "compound:prt" | "iobj") || s.upos[h - 1] != "VERB" {
                continue;
            }
            let ok = match global::idiom(&s.lemma[h - 1], &s.lemma[i]) {
                Some("idiom") => k("arc_idiom"),
                Some("light-verb") => k("arc_light_verb"),
                Some("collocation") => k("arc_collocation"),
                Some("phrasal-verb") => k("arc_phrasal_verb"),
                Some("candidate") => k("arc_candidate"),
                _ => false,
            };
            let mut pair = vec![h - 1, i];
            pair.sort();
            if ok && !out.iter().any(|o| pair.iter().all(|x| o.contains(x))) {
                out.push(pair);
            }
        }
    }
    out
}

/// (right, wrong, gold) for a sentence.
pub fn judge(s: &Sent, pred: &[Vec<usize>]) -> (usize, usize, usize) {
    let mut used = vec![false; s.gold.len()];
    let (mut right, mut wrong) = (0, 0);
    for p in pred {
        let hit = s.gold.iter().enumerate().find(|(gi, (g, _))| !used[*gi] && p.iter().filter(|x| g.contains(x)).count() >= 2);
        match hit {
            Some((gi, _)) => {
                used[gi] = true;
                right += 1;
            }
            None => wrong += 1,
        }
    }
    (right, wrong, s.gold.len())
}

pub fn score(data: &[Sent], cfg: &BTreeMap<String, f64>) -> Vec<i64> {
    data.iter().map(|s| {
        let (r, w, _) = judge(s, &predict(s, cfg));
        r as i64 - w as i64
    }).collect()
}

/// Precision and recall under a configuration.
pub fn pr(data: &[Sent], cfg: &BTreeMap<String, f64>) -> (f64, f64) {
    let (mut r, mut w, mut g) = (0usize, 0usize, 0usize);
    for s in data {
        let (a, b, c) = judge(s, &predict(s, cfg));
        r += a;
        w += b;
        g += c;
    }
    (100.0 * r as f64 / (r + w).max(1) as f64, 100.0 * r as f64 / g.max(1) as f64)
}
