//! Reader v2 measurement on FairytaleQA: answers (merge «state + sentence»), state only, reader v1 (`ans.rs`,
//! freeze-2) and the baseline; ROUGE-L by type, explicit and inferred, by tale, by answer source; «not in state»;
//! negative controls (facts of another tale; no index); a sample for manual checking and its summary.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::ans::{Src, answer, rouge_q};
use crate::ftqa::Qa;
use crate::reader::{RAnswer, RSrc};
use crate::report::ATTRS;

/// Measurement row.
#[derive(Clone, Debug)]
pub struct Row {
    pub q: Qa,
    pub a: RAnswer,
    /// ROUGE-L: merge, state only (not in state → 0), reader v1, baseline
    pub r: f64,
    pub r_state: f64,
    pub old: String,
    pub r_old: f64,
    pub base: String,
    pub r_base: f64,
}

pub fn rows(own: &crate::world::World, qs: &[Qa], ans: Vec<RAnswer>) -> Vec<Row> {
    qs.iter()
        .zip(ans)
        .map(|(q, a)| {
            let o = answer(own, q);
            let r_old = if o.src == Src::State { rouge_q(&o.text, q) } else { 0.0 };
            let (_, base) = crate::ans::baseline(own, q);
            let r_base = rouge_q(&base, q);
            let r = rouge_q(&a.text, q);
            let r_state = a.state.as_deref().map(|s| rouge_q(s, q)).unwrap_or(0.0);
            Row { q: q.clone(), a, r, r_state, old: if o.src == Src::State { o.text } else { String::new() }, r_old, base, r_base }
        })
        .collect()
}

#[derive(Default, Clone, Copy)]
struct Agg {
    n: usize,
    r: f64,
    st: f64,
    old: f64,
    base: f64,
    none: usize,
}

impl Agg {
    fn add(&mut self, x: &Row) {
        self.n += 1;
        self.r += x.r;
        self.st += x.r_state;
        self.old += x.r_old;
        self.base += x.r_base;
        self.none += x.a.state.is_none() as usize;
    }

    fn line(&self, name: &str) -> String {
        let n = self.n.max(1) as f64;
        format!("| {name} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.0}% |", self.n, self.old / n, self.st / n, self.r / n, self.base / n, 100.0 * self.none as f64 / n)
    }
}

const HEAD: &str = "| slice | N | before: reader v1 (freeze-2) | reader v2: state only | **reader v2: state + sentence** | baseline | not in state (v2) |\n|---|---|---|---|---|---|---|\n";

/// ROUGE-L tables.
pub fn report(rs: &[Row]) -> String {
    let mut o = String::new();
    let mut all = Agg::default();
    let mut by_attr: BTreeMap<String, Agg> = BTreeMap::new();
    let mut by_ex: BTreeMap<String, Agg> = BTreeMap::new();
    let mut by_tale: BTreeMap<String, Agg> = BTreeMap::new();
    let mut by_src: BTreeMap<String, Agg> = BTreeMap::new();
    let mut by_kind: BTreeMap<String, Agg> = BTreeMap::new();
    for x in rs {
        all.add(x);
        by_attr.entry(x.q.attr.clone()).or_default().add(x);
        by_ex.entry(x.q.ex.clone()).or_default().add(x);
        by_tale.entry(x.q.story.clone()).or_default().add(x);
        by_src.entry(x.a.src.name().to_string()).or_default().add(x);
        by_kind.entry(x.a.frame.split(' ').next().unwrap_or("?").to_string()).or_default().add(x);
    }
    o.push_str("## ROUGE-L by question type\n\n");
    o.push_str(HEAD);
    for a in ATTRS {
        if let Some(g) = by_attr.get(*a) {
            let _ = writeln!(o, "{}", g.line(a));
        }
    }
    let _ = writeln!(o, "{}", all.line("**all**"));
    o.push_str("\n## Explicit and inferred\n\n");
    o.push_str(HEAD);
    for (k, g) in &by_ex {
        let _ = writeln!(o, "{}", g.line(k));
    }
    o.push_str("\n## By tale\n\n");
    o.push_str(HEAD);
    for (k, g) in &by_tale {
        let _ = writeln!(o, "{}", g.line(k));
    }
    o.push_str("\n## By v2 answer source\n\n");
    o.push_str(HEAD);
    for (k, g) in &by_src {
        let _ = writeln!(o, "{}", g.line(k));
    }
    o.push_str("\n## By parse type (UD)\n\n");
    o.push_str(HEAD);
    for (k, g) in &by_kind {
        let _ = writeln!(o, "{}", g.line(k));
    }
    o
}

fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace(['\n', '\t'], " ")
}

/// All answers — TSV.
pub fn tsv(rs: &[Row]) -> String {
    let mut o = String::from("story\tn\tattr\tex\tquestion\tanswer1\tanswer2\tsrc\tv2\tr_v2\tstate\tr_state\tv1\tr_v1\tbase\tr_base\tconf\tfact\treason\tframe\n");
    for x in rs {
        let _ = writeln!(
            o,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{}\t{:.3}\t{}\t{:.3}\t{}\t{:.3}\t{:.1}\t{}\t{}\t{}",
            x.q.story,
            x.q.n,
            x.q.attr,
            x.q.ex,
            cell(&x.q.question),
            cell(&x.q.a1),
            cell(&x.q.a2),
            x.a.src.name(),
            cell(&x.a.text),
            x.r,
            cell(x.a.state.as_deref().unwrap_or("")),
            x.r_state,
            cell(&x.old),
            x.r_old,
            cell(&x.base),
            x.r_base,
            x.a.conf,
            cell(&x.a.fact),
            cell(&x.a.reason),
            cell(&x.a.frame)
        );
    }
    o
}

/// Detailed — for development (only on allowed questions).
pub fn dev(rs: &[Row]) -> String {
    let mut o = String::new();
    for x in rs {
        let _ = writeln!(
            o,
            "{}#{} [{}/{}] {}\n  G: {} | {}\n  v2 {:.2} [{}] {}\n     {} | {}\n     {}\n  v1 {:.2} {}\n  B  {:.2} {}",
            x.q.story,
            x.q.n,
            x.q.attr,
            &x.q.ex[..2.min(x.q.ex.len())],
            x.q.question,
            x.q.a1,
            x.q.a2,
            x.r,
            x.a.src.name(),
            x.a.text,
            x.a.reason,
            x.a.fact,
            x.a.frame,
            x.r_old,
            x.old,
            x.r_base,
            x.base
        );
    }
    o
}

/// Sample for manual checking: `k` per type, deterministic (FNV + splitmix from the salt, tale and number),
/// excluding excluded questions.
pub fn sample<'a>(rs: &'a [Row], k: usize, salt: &str, exclude: &BTreeSet<(String, usize)>) -> Vec<&'a Row> {
    let h = |s: &str| {
        let mut z = s.bytes().fold(0xcbf29ce484222325u64, |a, b| (a ^ b as u64).wrapping_mul(0x100000001b3));
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    };
    let mut out = Vec::new();
    for a in ATTRS {
        let mut v: Vec<&Row> = rs.iter().filter(|x| x.q.attr == *a && !exclude.contains(&(x.q.story.clone(), x.q.n))).collect();
        v.sort_by_key(|x| h(&format!("{salt}{}#{}", x.q.story, x.q.n)));
        out.extend(v.into_iter().take(k));
    }
    out
}

/// Manual verdicts: `story<TAB>n<TAB>v2<TAB>v1<TAB>baseline<TAB>state<TAB>comment` (1 / 0.5 / 0; «state» — whether the
/// answer is in the world state at all).
pub fn read_verdicts(text: &str) -> BTreeMap<(String, usize), [f64; 4]> {
    let mut m = BTreeMap::new();
    for l in text.lines() {
        if l.starts_with('#') || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 6 {
            continue;
        }
        let p = |i: usize| c[i].trim().parse::<f64>().ok();
        let (Ok(n), Some(a), Some(b), Some(d), Some(s)) = (c[1].parse::<usize>(), p(2), p(3), p(4), p(5)) else { continue };
        m.insert((c[0].to_string(), n), [a, b, d, s]);
    }
    m
}

/// Manual accuracy by type and explicit/inferred: v2, v1, baseline; how often v2 took the sentence; whether the answer is in the state.
pub fn manual(rs: &[&Row], v: &BTreeMap<(String, usize), [f64; 4]>) -> String {
    let mut o = String::from("| slice | N | before: v1 | **v2** | baseline | v2 took sentence | answer in state |\n|---|---|---|---|---|---|---|\n");
    let mut groups: Vec<(String, Vec<&Row>)> = ATTRS.iter().map(|a| (a.to_string(), rs.iter().copied().filter(|x| x.q.attr == *a).collect())).collect();
    groups.push(("**all**".into(), rs.to_vec()));
    groups.push(("explicit".into(), rs.iter().copied().filter(|x| x.q.ex == "explicit").collect()));
    groups.push(("inferred".into(), rs.iter().copied().filter(|x| x.q.ex != "explicit").collect()));
    for (name, g) in groups {
        let mut n = 0usize;
        let mut s = [0.0f64; 4];
        let mut sent = 0usize;
        for x in g {
            if let Some(vv) = v.get(&(x.q.story.clone(), x.q.n)) {
                n += 1;
                for i in 0..4 {
                    s[i] += vv[i];
                }
                sent += (x.a.src == RSrc::Sentence) as usize;
            }
        }
        if n == 0 {
            continue;
        }
        let p = |x: f64| 100.0 * x / n as f64;
        let _ = writeln!(o, "| {name} | {n} | {:.0}% | {:.0}% | {:.0}% | {sent} | {:.0}% |", p(s[1]), p(s[0]), p(s[2]), p(s[3]));
    }
    o
}
