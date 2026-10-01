//! Measurement on FairytaleQA: MMM answers from the state, fallback anchor, baseline, ROUGE-L; slices by question
//! type and by explicit/inferred; share of "not in state"; a sample for manual checking and its summary.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::ans::{Answer, Src, answer, baseline, rouge_q};
use crate::ftqa::Qa;
use crate::world::World;

/// A measurement row.
#[derive(Clone, Debug)]
pub struct Row {
    pub q: Qa,
    pub a: Answer,
    /// ROUGE-L: MMM (not in state → 0), MMM+anchor, baseline
    pub r_mmm: f64,
    pub r_anchor: f64,
    pub base: String,
    pub r_base: f64,
}

pub fn eval_tale(w: &World, qs: &[Qa]) -> Vec<Row> {
    qs.iter()
        .map(|q| {
            let a = answer(w, q);
            let r_mmm = if a.src == Src::State { rouge_q(&a.text, q) } else { 0.0 };
            let fb = match (&a.src, &a.anchor) {
                (Src::State, _) => r_mmm,
                (Src::None, Some((_, s))) => rouge_q(s, q),
                (Src::None, None) => 0.0,
            };
            let (_, base) = baseline(w, q);
            let r_base = rouge_q(&base, q);
            Row { q: q.clone(), a, r_mmm, r_anchor: fb, base, r_base }
        })
        .collect()
}

fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ").replace('\t', " ")
}

/// FairytaleQA types in the paper's order.
pub const ATTRS: &[&str] = &["character", "setting", "action", "feeling", "causal relationship", "outcome resolution", "prediction"];

/// Aggregate: N, ROUGE-L MMM, MMM on answered only, MMM+anchor, baseline, share of "not in state".
#[derive(Default, Clone, Copy)]
pub struct Agg {
    pub n: usize,
    pub mmm: f64,
    pub answered: usize,
    pub mmm_answered: f64,
    pub anchor: f64,
    pub base: f64,
}

impl Agg {
    pub fn add(&mut self, r: &Row) {
        self.n += 1;
        self.mmm += r.r_mmm;
        self.anchor += r.r_anchor;
        self.base += r.r_base;
        if r.a.src == Src::State {
            self.answered += 1;
            self.mmm_answered += r.r_mmm;
        }
    }

    fn line(&self, name: &str) -> String {
        let n = self.n.max(1) as f64;
        let ans = self.answered.max(1) as f64;
        format!(
            "| {name} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.0}% |",
            self.n,
            self.mmm / n,
            self.mmm_answered / ans,
            self.anchor / n,
            self.base / n,
            100.0 * (self.n - self.answered) as f64 / n
        )
    }
}

const HEAD: &str = "| slice | N | ROUGE-L MMM | MMM, answered only | MMM + anchor | baseline | not in state |\n|---|---|---|---|---|---|---|\n";

/// Report: slices and tables.
pub fn report(rows: &[Row]) -> String {
    let mut o = String::new();
    let mut all = Agg::default();
    let mut by_attr: BTreeMap<&str, Agg> = BTreeMap::new();
    let mut by_ex: BTreeMap<String, Agg> = BTreeMap::new();
    let mut by_tale: BTreeMap<String, Agg> = BTreeMap::new();
    let mut by_intent: BTreeMap<String, Agg> = BTreeMap::new();
    for r in rows {
        all.add(r);
        by_attr.entry(ATTRS.iter().find(|a| **a == r.q.attr).copied().unwrap_or("?")).or_default().add(r);
        by_ex.entry(r.q.ex.clone()).or_default().add(r);
        by_tale.entry(r.q.story.clone()).or_default().add(r);
        by_intent.entry(r.a.frame.split(' ').next().unwrap_or("?").to_string()).or_default().add(r);
    }
    o.push_str("## By question type\n\n");
    o.push_str(HEAD);
    for a in ATTRS {
        if let Some(g) = by_attr.get(a) {
            let _ = writeln!(o, "{}", g.line(a));
        }
    }
    let _ = writeln!(o, "{}", all.line("**all**"));
    o.push_str("\n## Explicit and inferred\n\n");
    o.push_str(HEAD);
    for (k, g) in &by_ex {
        let _ = writeln!(o, "{}", g.line(k));
    }
    o.push_str("\n## By story\n\n");
    o.push_str(HEAD);
    for (k, g) in &by_tale {
        let _ = writeln!(o, "{}", g.line(k));
    }
    o.push_str("\n## By question parse intent (MMM)\n\n");
    o.push_str(HEAD);
    for (k, g) in &by_intent {
        let _ = writeln!(o, "{}", g.line(k));
    }
    o
}

/// All answers — TSV (for manual checking and analysis).
pub fn rows_tsv(rows: &[Row]) -> String {
    let mut o = String::from("story\tn\tattr\tex\tquestion\tanswer1\tanswer2\tsrc\tmmm\tr_mmm\tanchor\tr_anchor\tbase\tr_base\tframe\tnote\n");
    for r in rows {
        let _ = writeln!(
            o,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{}\t{:.3}\t{}\t{:.3}\t{}\t{}",
            r.q.story,
            r.q.n,
            r.q.attr,
            r.q.ex,
            cell(&r.q.question),
            cell(&r.q.a1),
            cell(&r.q.a2),
            if r.a.src == Src::State { "state" } else { "none" },
            cell(&r.a.text),
            r.r_mmm,
            cell(&r.a.anchor.as_ref().map(|x| format!("^s{} {}", x.0, x.1)).unwrap_or_default()),
            r.r_anchor,
            cell(&r.base),
            r.r_base,
            cell(&r.a.frame),
            cell(&r.a.note)
        );
    }
    o
}

/// A story's answers — md.
pub fn tale_md(name: &str, rows: &[Row]) -> String {
    let mut o = format!("# FairytaleQA: {name} — MMM answers from the state\n\n");
    o.push_str("| # | type | e/i | question | reference | MMM (state) | R | anchor | R | baseline | R |\n|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in rows {
        let refs = if r.q.a2.is_empty() { r.q.a1.clone() } else { format!("{} / {}", r.q.a1, r.q.a2) };
        let mmm = if r.a.src == Src::State { format!("{} ({})", r.a.text, r.a.note) } else { format!("not in state: {}", r.a.note) };
        let anc = if r.a.src == Src::None { r.a.anchor.as_ref().map(|x| format!("^s{} {}", x.0, x.1)).unwrap_or("—".into()) } else { "—".into() };
        let _ = writeln!(
            o,
            "| {} | {} | {} | {} | {} | {} | {:.2} | {} | {:.2} | {} | {:.2} |",
            r.q.n,
            r.q.attr,
            if r.q.ex == "explicit" { "e" } else { "i" },
            cell(&r.q.question),
            cell(&refs),
            cell(&mmm),
            r.r_mmm,
            cell(&anc),
            r.r_anchor,
            cell(&r.base),
            r.r_base
        );
    }
    o
}

/// Deterministic sample for manual checking: `k` questions of each type (FNV hash of the story and number).
pub fn sample<'a>(rows: &'a [Row], k: usize, salt: &str, exclude: &std::collections::BTreeSet<(String, usize)>) -> Vec<&'a Row> {
    // FNV-1a + splitmix64 finalizer (plain FNV clusters similar keys "story#n")
    let h = |s: &str| {
        let mut z = s.bytes().fold(0xcbf29ce484222325u64, |a, b| (a ^ b as u64).wrapping_mul(0x100000001b3));
        if salt == "fnv-plain" {
            // reproduction of the first (clustered) sample — only to exclude its questions
            return z;
        }
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    };
    let mut out = Vec::new();
    for a in ATTRS {
        let mut v: Vec<&Row> = rows.iter().filter(|r| r.q.attr == *a && !exclude.contains(&(r.q.story.clone(), r.q.n))).collect();
        let key = |r: &&Row| if salt == "fnv-plain" { h(&format!("{}#{}", r.q.story, r.q.n)) } else { h(&format!("{salt}{}#{}", r.q.story, r.q.n)) };
        v.sort_by_key(key);
        out.extend(v.into_iter().take(k));
    }
    out
}

/// Manual verdicts (`story<TAB>n<TAB>mmm<TAB>baseline<TAB>state<TAB>comment`; 1 — correct, 0.5 — partial,
/// 0 — no; "state" — whether the answer is in the world state at all).
pub fn read_verdicts(text: &str) -> BTreeMap<(String, usize), (f64, f64, f64, String)> {
    let mut m = BTreeMap::new();
    for l in text.lines() {
        if l.starts_with('#') || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 5 {
            continue;
        }
        let (Ok(n), Ok(a), Ok(b), Ok(s)) = (c[1].parse::<usize>(), c[2].parse::<f64>(), c[3].parse::<f64>(), c[4].parse::<f64>()) else { continue };
        m.insert((c[0].to_string(), n), (a, b, s, c.get(5).unwrap_or(&"").to_string()));
    }
    m
}

/// Accuracy on the sample by type: MMM (not in state — 0) and baseline.
pub fn manual_md(rows: &[&Row], v: &BTreeMap<(String, usize), (f64, f64, f64, String)>) -> String {
    let mut o = String::from("| type | N | MMM accuracy | of them not in state | answer is in state | baseline accuracy |\n|---|---|---|---|---|---|\n");
    let mut tot = (0usize, 0.0, 0usize, 0.0, 0.0);
    for a in ATTRS {
        let mut n = 0;
        let (mut m, mut b, mut s, mut none) = (0.0, 0.0, 0.0, 0);
        for r in rows.iter().filter(|r| r.q.attr == *a) {
            if let Some((x, y, z, _)) = v.get(&(r.q.story.clone(), r.q.n)) {
                n += 1;
                m += x;
                b += y;
                s += z;
                none += (r.a.src == Src::None) as usize;
            }
        }
        if n > 0 {
            let p = |x: f64| 100.0 * x / n as f64;
            let _ = writeln!(o, "| {a} | {n} | {:.0}% | {none} | {:.0}% | {:.0}% |", p(m), p(s), p(b));
            tot = (tot.0 + n, tot.1 + m, tot.2 + none, tot.3 + b, tot.4 + s);
        }
    }
    if tot.0 > 0 {
        let p = |x: f64| 100.0 * x / tot.0 as f64;
        let _ = writeln!(o, "| **all** | {} | {:.0}% | {} | {:.0}% | {:.0}% |", tot.0, p(tot.1), tot.2, p(tot.4), p(tot.3));
    }
    o
}
