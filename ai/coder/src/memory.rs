//! Memory of verified programs and BM25 search over our lemmas.
//!
//! Sources (read-only): `mlab/data/suite-v1.txt`, `suite-v2.txt` and 400 Opus scenarios
//! (`mlab/tests/data/vmm/*.txt`, branch tests3, merged into main on 26.09). "Verified" = the scenario passes
//! on this branch's mlab (run once, when building the memory — before training) and has no file calls.

use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::lemma;
use crate::util::crate_dir;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: String,
    pub source: String,
    pub title: String,
    pub code: String,
    /// lemmas of the title and category + code identifiers
    pub terms: Vec<String>,
    /// built-ins and other names occurring in the code
    pub idents: Vec<String>,
}

pub fn idents(code: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Ok(toks) = mlab::lexer::lex(code) {
        for t in toks {
            if let mlab::lexer::Tok::Ident(s) = t.tok {
                if !out.contains(&s) {
                    out.push(s);
                }
            }
        }
    }
    out
}

fn item(id: &str, source: &str, category: &str, title: &str, code: &str) -> Item {
    let idn = idents(code);
    let mut terms = lemma::lemmas(&format!("{category} {title}"));
    terms.extend(idn.iter().map(|s| s.to_lowercase()));
    Item { id: id.into(), source: source.into(), title: format!("{category}: {title}"), code: code.into(), terms, idents: idn }
}

/// Building the memory with a scenario run (verification). Returns (memory, report by source).
pub fn build() -> Result<(Vec<Item>, Vec<(String, usize, usize)>)> {
    let mut files: Vec<(String, std::path::PathBuf)> = vec![
        ("suite-v1".into(), crate_dir().join("../mlab/data/suite-v1.txt")),
        ("suite-v2".into(), crate_dir().join("../mlab/data/suite-v2.txt")),
    ];
    // 400 Opus scenarios: branch tests3 is already merged into main — take them from this branch
    let vdir = crate_dir().join("../mlab/tests/data/vmm");
    for area in ["ctl", "edge", "idx", "la", "ops", "poly", "red", "stat", "str", "tab"] {
        files.push((format!("vmm-{area}"), vdir.join(format!("{area}.txt"))));
    }
    let mut out = Vec::new();
    let mut rep = Vec::new();
    for (src, path) in files {
        let text = std::fs::read_to_string(&path)?;
        let sc = mlab::suite::parse(&text);
        let res = mlab::suite::run(&sc, None);
        let mut kept = 0;
        for (s, r) in sc.iter().zip(&res) {
            if !r.pass || !crate::sandbox::denied(&s.code).is_empty() {
                continue;
            }
            out.push(item(&s.id, &src, &s.category, &s.title, &s.code));
            kept += 1;
        }
        rep.push((src, sc.len(), kept));
    }
    Ok((out, rep))
}

/// BM25 (k1 = 1.2, b = 0.75) over memory terms.
pub struct Bm25 {
    pub docs: Vec<Item>,
    df: HashMap<String, usize>,
    tf: Vec<HashMap<String, usize>>,
    avgdl: f64,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub doc: usize,
    pub score: f64,
    /// contribution of each query term — for the explanation
    pub parts: Vec<(String, f64)>,
}

impl Bm25 {
    pub fn new(docs: Vec<Item>) -> Bm25 {
        let mut df: HashMap<String, usize> = HashMap::new();
        let mut tf = Vec::new();
        let mut total = 0usize;
        for d in &docs {
            let mut m: HashMap<String, usize> = HashMap::new();
            for t in &d.terms {
                *m.entry(t.clone()).or_default() += 1;
            }
            for k in m.keys() {
                *df.entry(k.clone()).or_default() += 1;
            }
            total += d.terms.len();
            tf.push(m);
        }
        let avgdl = total as f64 / docs.len().max(1) as f64;
        Bm25 { docs, df, tf, avgdl }
    }

    pub fn search(&self, query: &[String], k: usize) -> Vec<Hit> {
        let n = self.docs.len() as f64;
        let mut q: BTreeMap<&str, usize> = BTreeMap::new();
        for t in query {
            *q.entry(t.as_str()).or_default() += 1;
        }
        let mut hits: Vec<Hit> = Vec::new();
        for (i, tf) in self.tf.iter().enumerate() {
            let dl = self.docs[i].terms.len() as f64;
            let mut score = 0.0;
            let mut parts = Vec::new();
            for (t, _) in &q {
                let Some(&f) = tf.get(*t) else { continue };
                let df = *self.df.get(*t).unwrap_or(&0) as f64;
                let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();
                let f = f as f64;
                let s = idf * f * 2.2 / (f + 1.2 * (0.25 + 0.75 * dl / self.avgdl));
                score += s;
                parts.push((t.to_string(), s));
            }
            if score > 0.0 {
                parts.sort_by(|a, b| b.1.total_cmp(&a.1));
                hits.push(Hit { doc: i, score, parts });
            }
        }
        hits.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.doc.cmp(&b.doc)));
        hits.truncate(k);
        hits
    }
}
