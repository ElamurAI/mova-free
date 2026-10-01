//! Measuring two code representations (text, AST, both) on the memory of verified programs — no Opus and no execution:
//! (a) example search: given a program's code, find the nearest other program of the same area (leave-one-out, P@1, MRR@10);
//! (b) move choice: given a code prefix, predict the move of the next statement (perceptron, 80/20 by programs);
//! both — also on invalid fragments (truncated code) with partial parsing.

use std::collections::HashMap;

use crate::memory::Item;
use crate::motiv::Perceptron;
use crate::repr::{self, Mode};
use crate::util::sha256_hex;

fn label(it: &Item) -> String {
    if it.source.starts_with("vmm-") { it.source.clone() } else { format!("{}:{}", it.source, it.title.split(':').next().unwrap_or("")) }
}

/// Invalid fragment: the first 60% of lines, the last kept line cut in half.
pub fn corrupt(code: &str) -> String {
    let lines: Vec<&str> = code.lines().filter(|l| !l.trim().is_empty()).collect();
    let k = ((lines.len() as f64 * 0.6).ceil() as usize).max(1).min(lines.len());
    let mut out: Vec<String> = lines[..k].iter().map(|s| s.to_string()).collect();
    if let Some(last) = out.last_mut() {
        let n = last.chars().count();
        *last = last.chars().take((n / 2).max(1)).collect();
    }
    out.join("\n") + "\n"
}

type Vecx = std::collections::BTreeMap<String, f64>;

fn tfidf(docs: &[Vec<String>]) -> (Vec<Vecx>, HashMap<String, f64>) {
    let n = docs.len() as f64;
    let mut df: HashMap<String, f64> = HashMap::new();
    for d in docs {
        let mut seen = std::collections::HashSet::new();
        for f in d {
            if seen.insert(f) {
                *df.entry(f.clone()).or_default() += 1.0;
            }
        }
    }
    let idf: HashMap<String, f64> = df.into_iter().map(|(k, v)| (k, (n / v).ln() + 1.0)).collect();
    let vecs = docs.iter().map(|d| vec_of(d, &idf)).collect();
    (vecs, idf)
}

fn vec_of(d: &[String], idf: &HashMap<String, f64>) -> Vecx {
    let mut m: Vecx = Vecx::new();
    for f in d {
        *m.entry(f.clone()).or_default() += 1.0;
    }
    for (k, v) in m.iter_mut() {
        *v = (1.0 + v.ln()) * idf.get(k).copied().unwrap_or(0.0);
    }
    let norm = m.values().map(|x| x * x).sum::<f64>().sqrt().max(1e-12);
    for v in m.values_mut() {
        *v /= norm;
    }
    m
}

fn cos(a: &Vecx, b: &Vecx) -> f64 {
    let (s, l) = if a.len() < b.len() { (a, b) } else { (b, a) };
    s.iter().map(|(k, v)| v * l.get(k).copied().unwrap_or(0.0)).sum()
}

pub struct RetrievalRow {
    pub mode: &'static str,
    pub query: &'static str,
    pub p1: f64,
    pub mrr: f64,
    pub n: usize,
}

pub fn retrieval(items: &[Item]) -> (Vec<RetrievalRow>, (usize, f64, usize)) {
    let labels: Vec<String> = items.iter().map(label).collect();
    let mut rows = Vec::new();
    // how many invalid fragments still parse partially
    let mut full = 0usize;
    let mut frac_sum = 0.0;
    for it in items {
        let c = corrupt(&it.code);
        let (_, _, frac, ok) = repr::parse_partial(&c);
        if ok {
            full += 1;
        }
        frac_sum += frac;
    }
    for (mode, mname) in [(Mode::Text, "text"), (Mode::Ast, "AST"), (Mode::Both, "both")] {
        let docs: Vec<Vec<String>> = items.iter().map(|it| repr::feats(&it.code, mode)).collect();
        let (vecs, idf) = tfidf(&docs);
        for (qname, corrupted) in [("valid code", false), ("invalid fragment", true)] {
            let (mut p1, mut mrr) = (0.0, 0.0);
            for i in 0..items.len() {
                let q = if corrupted { vec_of(&repr::feats(&corrupt(&items[i].code), mode), &idf) } else { vecs[i].clone() };
                let mut sc: Vec<(usize, f64)> = (0..items.len()).filter(|&j| j != i).map(|j| (j, cos(&q, &vecs[j]))).collect();
                sc.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
                if labels[sc[0].0] == labels[i] {
                    p1 += 1.0;
                }
                if let Some(r) = sc.iter().take(10).position(|(j, _)| labels[*j] == labels[i]) {
                    mrr += 1.0 / (r + 1) as f64;
                }
            }
            let n = items.len() as f64;
            rows.push(RetrievalRow { mode: mname, query: qname, p1: p1 / n, mrr: mrr / n, n: items.len() });
        }
    }
    (rows, (full, frac_sum / items.len() as f64, items.len()))
}

pub struct MoveRow {
    pub mode: &'static str,
    pub query: &'static str,
    pub acc: f64,
    pub n: usize,
}

/// Move choice: program prefix → move of the next top-level statement.
pub fn moves(items: &[Item], skip_output: bool) -> (Vec<MoveRow>, f64, Vec<(String, usize)>) {
    // (prefix, label, test?)
    let mut data: Vec<(String, &'static str, bool)> = Vec::new();
    for it in items {
        let Some(p) = repr::program(&it.code) else { continue };
        if p.body.len() < 2 {
            continue;
        }
        let test = u8::from_str_radix(&sha256_hex(it.id.as_bytes())[..2], 16).unwrap_or(0) < 51; // ~20%
        let lines: Vec<&str> = it.code.lines().collect();
        for s in &p.body[1..] {
            let cut = s.line.saturating_sub(1).min(lines.len());
            if cut == 0 {
                continue;
            }
            let prefix = lines[..cut].join("\n") + "\n";
            let m = repr::stmt_move(s);
            if skip_output && m == "Output" {
                continue;
            }
            data.push((prefix, m, test));
        }
    }
    let classes: Vec<&str> = {
        let mut c: Vec<&str> = data.iter().map(|d| d.1).collect();
        c.sort();
        c.dedup();
        c
    };
    let mut dist: HashMap<&str, usize> = HashMap::new();
    for d in data.iter().filter(|d| d.2) {
        *dist.entry(d.1).or_default() += 1;
    }
    let n_test = data.iter().filter(|d| d.2).count();
    let base = dist.values().copied().max().unwrap_or(0) as f64 / n_test.max(1) as f64;
    let mut rows = Vec::new();
    for (mode, mname) in [(Mode::Text, "text"), (Mode::Ast, "AST"), (Mode::Both, "both")] {
        let feats: Vec<Vec<String>> = data.iter().map(|d| {
            let mut f = repr::feats(&d.0, mode);
            f.sort();
            f.dedup();
            f.push("bias".into());
            f
        }).collect();
        let mut p = Perceptron::default();
        for _ in 0..8 {
            for (i, d) in data.iter().enumerate() {
                if !d.2 {
                    p.train(&feats[i], d.1, &classes);
                }
            }
        }
        p.average();
        for (qname, corrupted) in [("valid code", false), ("invalid fragment", true)] {
            let mut hit = 0;
            for (i, d) in data.iter().enumerate().filter(|(_, d)| d.2) {
                let f = if corrupted {
                    let mut f = repr::feats(&corrupt_last(&d.0), mode);
                    f.sort();
                    f.dedup();
                    f.push("bias".into());
                    f
                } else {
                    feats[i].clone()
                };
                if p.predict(&f, &classes) == d.1 {
                    hit += 1;
                }
            }
            rows.push(MoveRow { mode: mname, query: qname, acc: hit as f64 / n_test.max(1) as f64, n: n_test });
        }
    }
    let mut dv: Vec<(String, usize)> = dist.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    dv.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    (rows, base, dv)
}

/// Prefix with the last line cut in half (unfinished code).
fn corrupt_last(prefix: &str) -> String {
    let mut lines: Vec<String> = prefix.lines().map(str::to_string).collect();
    if let Some(l) = lines.iter_mut().rev().find(|l| !l.trim().is_empty()) {
        let n = l.chars().count();
        *l = l.chars().take((n / 2).max(1)).collect();
    }
    lines.join("\n") + "\n"
}
