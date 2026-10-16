//! Measurement: a frozen question set (hash checked before the run) and negative controls.
//!
//! Answer scoring: correct / "unknown" / wrong. For gold `unknown` only "unknown" is correct;
//! for a frame gold — correct if the frame is among the word's frames.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

use crate::kb::*;
use crate::model::*;
use crate::query::{self, Answer, Verdict};

#[derive(Clone, Debug)]
pub struct Q {
    pub id: String,
    pub group: String,
    pub query: String,
    pub gold: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Correct,
    Unknown,
    Wrong,
}

pub fn sha256_hex(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

/// Question set; if `<file>.sha256` (in `sha256sum` format) is next to it, the hash must match.
pub fn read_frozen(path: &Path) -> Result<(Vec<Q>, String)> {
    let bytes = std::fs::read(path).with_context(|| format!("cannot read {}", path.display()))?;
    let hash = sha256_hex(&bytes);
    let hp = path.with_extension("sha256");
    if hp.exists() {
        let want = std::fs::read_to_string(&hp)?.split_whitespace().next().unwrap_or("").to_string();
        if want != hash {
            bail!("gate: hash of {} {hash} ≠ frozen {want} — the set was changed after freezing", path.display());
        }
    }
    let text = String::from_utf8(bytes)?;
    let mut qs = Vec::new();
    for l in text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 4 {
            bail!("set line without 4 fields: {l}");
        }
        qs.push(Q { id: f[0].into(), group: f[1].into(), query: f[2].into(), gold: f[3].into() });
    }
    Ok((qs, hash))
}

pub fn score(a: &Answer, gold: &str) -> Status {
    match gold {
        "yes" | "no" | "undetermined" => {
            if a.verdict.code() == gold {
                Status::Correct
            } else if a.verdict == Verdict::Unknown {
                Status::Unknown
            } else {
                Status::Wrong
            }
        }
        "unknown" => {
            if a.verdict == Verdict::Unknown {
                Status::Correct
            } else {
                Status::Wrong
            }
        }
        frame => {
            if a.values.iter().any(|v| v.eq_ignore_ascii_case(frame)) {
                Status::Correct
            } else if a.verdict == Verdict::Unknown {
                Status::Unknown
            } else {
                Status::Wrong
            }
        }
    }
}

pub fn run(kb: &Kb, qs: &[Q]) -> Vec<(Answer, Status)> {
    qs.iter()
        .map(|q| {
            let a = query::ask(kb, &q.query).unwrap_or_else(|e| Answer { verdict: Verdict::Unknown, values: vec![], lines: vec![format!("query error: {e}")] });
            let s = score(&a, &q.gold);
            (a, s)
        })
        .collect()
}

/// Summary: group → (correct, unknown, wrong).
pub fn tally(qs: &[Q], rs: &[(Answer, Status)]) -> Vec<(String, [usize; 3])> {
    let mut m: Vec<(String, [usize; 3])> = Vec::new();
    for (q, (_, s)) in qs.iter().zip(rs) {
        let i = match m.iter().position(|x| x.0 == q.group) {
            Some(i) => i,
            None => {
                m.push((q.group.clone(), [0; 3]));
                m.len() - 1
            }
        };
        m[i].1[*s as usize] += 1;
    }
    let total = m.iter().fold([0; 3], |acc, (_, v)| [acc[0] + v[0], acc[1] + v[1], acc[2] + v[2]]);
    m.push(("total".into(), total));
    m
}

pub fn tally_md(t: &[(String, [usize; 3])]) -> String {
    let mut s = String::from("| group | questions | correct | \"unknown\" | wrong |\n|---|---:|---:|---:|---:|\n");
    for (g, v) in t {
        let n = v.iter().sum::<usize>();
        let _ = writeln!(s, "| {g} | {n} | {} ({:.0}%) | {} | {} |", v[0], 100.0 * v[0] as f64 / n.max(1) as f64, v[1], v[2]);
    }
    s
}

fn status_uk(s: Status) -> &'static str {
    match s {
        Status::Correct => "correct",
        Status::Unknown => "\"unknown\"",
        Status::Wrong => "WRONG",
    }
}

/// Frozen-set report: summary, each question with its answer and proof.
pub fn report(qs: &[Q], rs: &[(Answer, Status)], hash: &str, title: &str) -> String {
    let mut s = format!("# {title}\n\nSet `onto/data/frozen.tsv`, sha256 `{hash}` (checked against `frozen.sha256` before the run).\n\n");
    s.push_str(&tally_md(&tally(qs, rs)));
    s.push_str("\n## Questions\n\n");
    for (q, (a, st)) in qs.iter().zip(rs) {
        let _ = writeln!(s, "### {} `{}` — gold {}, answer {} → {}\n", q.id, q.query, q.gold, a.verdict.code(), status_uk(*st));
        s.push_str("```\n");
        s.push_str(&a.text());
        s.push_str("\n```\n\n");
    }
    s
}

// ── negative controls ───────────────────────────────────────────────────────────────────────────

/// Simple reproducible generator (xorshift64*).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Graph with a shuffled hierarchy: the endpoints of IsA/InstanceOf edges are randomly permuted among themselves
/// (same in-degree distribution, different structure).
pub fn shuffled(g: &Graph, seed: u64) -> Graph {
    let n = g.n_concepts();
    let mut edges: Vec<(Cid, Rel, Cid)> = Vec::with_capacity(g.out.len());
    for c in 0..n {
        for e in g.out_edges(c as Cid) {
            edges.push((c as Cid, e.rel, e.to));
        }
    }
    let idx: Vec<usize> = (0..edges.len()).filter(|&i| edges[i].1.is_taxonomic()).collect();
    let mut targets: Vec<Cid> = idx.iter().map(|&i| edges[i].2).collect();
    let mut rng = Rng(seed | 1);
    for i in (1..targets.len()).rev() {
        let j = rng.below(i + 1);
        targets.swap(i, j);
    }
    for (k, &i) in idx.iter().enumerate() {
        edges[i].2 = targets[k];
    }
    let mut out = Graph {
        src: g.src.clone(),
        key: g.key.clone(),
        pos: g.pos.clone(),
        lexfile: g.lexfile.clone(),
        flags: g.flags.clone(),
        ili: g.ili.clone(),
        gloss_off: g.gloss_off.clone(),
        gloss: g.gloss.clone(),
        name_off: g.name_off.clone(),
        names: g.names.clone(),
        mem_off: g.mem_off.clone(),
        mem: g.mem.clone(),
        word_off: g.word_off.clone(),
        words: g.words.clone(),
        sense_off: g.sense_off.clone(),
        senses: g.senses.clone(),
        lus: g.lus.clone(),
        ..Default::default()
    };
    let csr = |v: &mut Vec<(Cid, Edge)>| -> (Vec<u32>, Vec<Edge>) {
        v.sort_by_key(|(s, e)| (*s, e.rel as u8, e.to));
        let mut off = vec![0u32; n + 1];
        for (s, _) in v.iter() {
            off[*s as usize + 1] += 1;
        }
        for i in 0..n {
            off[i + 1] += off[i];
        }
        (off, v.iter().map(|(_, e)| *e).collect())
    };
    (out.out_off, out.out) = csr(&mut edges.iter().map(|&(s, r, t)| (s, Edge { rel: r, to: t })).collect());
    (out.in_off, out.inn) = csr(&mut edges.iter().map(|&(s, r, t)| (t, Edge { rel: r, to: s })).collect());
    out
}

/// Check "a lower-layer exception does not leak into the real layer": answers on the real layer with all
/// seeds must match the answers when only the real-layer seeds are kept.
pub fn leak_check(kb: &Kb, real_only: &Kb, probes: &[(&str, Prop)]) -> Vec<String> {
    let mut bad = Vec::new();
    for (w, p) in probes {
        let a = query::prop(kb, w, *p, 0);
        let b = query::prop(real_only, w, *p, 0);
        if a.verdict != b.verdict {
            bad.push(format!("{w} {}: with all layers {} ≠ real only {}", p.name(), a.verdict.code(), b.verdict.code()));
        }
    }
    bad
}

/// Check "unknown word — unknown": every query with these words must yield `unknown`.
pub fn unknown_check(kb: &Kb, words: &[&str]) -> Vec<String> {
    let mut bad = Vec::new();
    for w in words {
        for q in [
            format!("isa {w} animal"),
            format!("isa animal {w}"),
            format!("can {w} fly"),
            format!("can {w} speak --layer fable"),
            format!("haspart {w} wing"),
            format!("frame {w}"),
            format!("role {w} Buyer"),
            format!("entails {w} sleep"),
        ] {
            match query::ask(kb, &q) {
                Ok(a) if a.verdict == Verdict::Unknown => {}
                Ok(a) => bad.push(format!("`{q}` → {}", a.verdict.code())),
                Err(e) => bad.push(format!("`{q}` → error {e}")),
            }
        }
    }
    bad
}

pub fn probes() -> Vec<(&'static str, Prop)> {
    let words = ["crow", "fox", "lion", "mouse", "hare", "tortoise", "stone", "tree", "genie", "jellyfish", "oewn-01913388-n", "fairy", "dragon", "unicorn", "penguin", "whale"];
    let mut v = Vec::new();
    for w in words {
        for p in Prop::ALL {
            v.push((w, p));
        }
    }
    v
}

/// Kb with the same graph and only the real-layer seeds.
pub fn real_only(kb: &Kb, g: Graph) -> Result<Kb> {
    let mut k = Kb::new(g);
    k.layers = kb.layers.clone();
    for e in &kb.extra {
        k.add_extra(e.clone())?;
    }
    for a in kb.asserts.iter().filter(|a| a.layer == 0) {
        k.add_assert(a.clone())?;
    }
    Ok(k)
}

pub fn group_acc(t: &[(String, [usize; 3])], g: &str) -> String {
    t.iter()
        .find(|x| x.0 == g)
        .map(|(_, v)| {
            let n: usize = v.iter().sum();
            format!("{}/{} ({:.0}%)", v[0], n, 100.0 * v[0] as f64 / n.max(1) as f64)
        })
        .unwrap_or_default()
}

/// Share of yes/no answers that match the trivial "always no" baseline — the chance level for
/// a set where answers are mostly "no" without a hierarchy.
pub fn majority_baseline(qs: &[Q], group: &str) -> String {
    let v: Vec<&Q> = qs.iter().filter(|q| q.group == group).collect();
    let mut c: HashMap<&str, usize> = HashMap::new();
    for q in &v {
        *c.entry(q.gold.as_str()).or_default() += 1;
    }
    let (best, n) = c.iter().max_by_key(|x| (*x.1, *x.0)).map(|(k, v)| (*k, *v)).unwrap_or(("", 0));
    format!("\"always {best}\" {n}/{} ({:.0}%)", v.len(), 100.0 * n as f64 / v.len().max(1) as f64)
}
