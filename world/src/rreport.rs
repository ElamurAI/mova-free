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

/// Pair answers with questions in the same order used by reader::read_all.
/// Panics before scoring when cardinalities differ, preventing silent measurement loss.
/// Equal lengths cannot detect reordered answers; order is a caller precondition.
pub fn rows(own: &crate::world::World, qs: &[Qa], ans: Vec<RAnswer>) -> Vec<Row> {
    assert_eq!(qs.len(), ans.len(), "one answer per question is required");
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

#[cfg(test)]
mod wave10_models {
    use super::*;

    fn row(n: usize, state: bool, score: f64) -> Row {
        Row {
            q: Qa { story: "t".into(), n, attr: "setting".into(), ex: "explicit".into(), ex2: "".into(),
                local: true, secs: vec![1], question: "where?".into(), a1: "here".into(), a2: "".into() },
            a: RAnswer { text: "here".into(), src: RSrc::Sentence, state: state.then(|| "here".into()),
                conf: 1.0, fact: "".into(), reason: "".into(), frame: "where actor".into(), sent: None },
            r: score, r_state: score / 2.0, old: "".into(), r_old: 0.5, base: "".into(), r_base: 1.0,
        }
    }

    #[test]
    fn mismatched_answers_rejected_before_scoring() {
        let w = crate::world::World::new(crate::world::Text { doc: 0, title: String::new(), sents: vec![] });
        assert!(std::panic::catch_unwind(|| rows(&w, &[], vec![row(0, false, 0.0).a])).is_err(),
            "extra answers must not silently vanish");
        assert!(std::panic::catch_unwind(|| rows(&w, &[row(0, false, 0.0).q], vec![])).is_err(),
            "missing answers must not silently undercount");
        assert!(rows(&w, &[], vec![]).is_empty());
    }

    #[test]
    fn aggregation_independent_integer_oracle() {
        for mask in 0..8 {
            for code in 0..27 {
                let scores = [code % 3, code / 3 % 3, code / 9];
                let rs: Vec<_> = (0..3).map(|i| row(i, mask & (1 << i) != 0, scores[i] as f64 / 2.0)).collect();
                let mut a = Agg::default();
                for x in &rs { a.add(x); }
                let sum: usize = scores.iter().sum();
                let missing = 3 - (mask as u32).count_ones();
                assert_eq!((a.n, a.none), (3, missing as usize));
                assert_eq!((a.r, a.st, a.old, a.base), (sum as f64 / 2.0, sum as f64 / 4.0, 1.5, 3.0));
                assert_eq!(a.line("x"), format!("| x | 3 | 0.500 | {:.3} | {:.3} | 1.000 | {:.0}% |",
                    sum as f64 / 12.0, sum as f64 / 6.0, missing as f64 * 100.0 / 3.0));
            }
        }
        assert_eq!(Agg::default().line("empty"), "| empty | 0 | 0.000 | 0.000 | 0.000 | 0.000 | 0% |");
    }

    #[test]
    fn aggregation_fibers_and_identity_join() {
        let mut fibers: BTreeMap<String, usize> = BTreeMap::new();
        for x in 0..3 { for y in 0..3 {
            let mut a = Agg::default();
            a.add(&row(1, false, x as f64 / 2.0));
            a.add(&row(2, false, y as f64 / 2.0));
            *fibers.entry(a.line("pair")).or_default() += 1;
        }}
        assert_eq!(fibers.values().copied().collect::<Vec<_>>(), [1, 2, 3, 2, 1]);
        let mut rs = vec![row(7, false, 1.0), row(7, true, 0.0)];
        rs[1].q.story = "other".into();
        let verdicts = read_verdicts("t\t7\t1\t0\t0.5\t1\nother\t7\t0\t1\t0.5\t0");
        assert_eq!(verdicts.len(), 2);
        let m = manual(&rs.iter().collect::<Vec<_>>(), &verdicts);
        assert!(m.contains("| **all** | 2 | 50% | 50% | 50% | 2 | 50% |"));
        let excluded = BTreeSet::from([("t".to_string(), 7)]);
        let selected = sample(&rs, 5, "fixed", &excluded);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].q.story, "other");
        assert!(report(&rs).contains("| **all** | 2 | 0.500 | 0.250 | 0.500 | 1.000 | 50% |"));
        assert_eq!(tsv(&rs).lines().count(), 3);
    }

    #[test]
    fn sample_bounds_and_verdict_overwrite() {
        let rs: Vec<_> = (0..20).map(|i| row(i, false, 0.0)).collect();
        for k in 0..=22 {
            let chosen = sample(&rs, k, "s", &BTreeSet::new());
            assert_eq!(chosen.len(), k.min(20));
            let ids: BTreeSet<_> = chosen.iter().map(|r| r.q.n).collect();
            assert_eq!(ids.len(), chosen.len());
            assert_eq!(chosen.iter().map(|r| r.q.n).collect::<Vec<_>>(),
                sample(&rs, k, "s", &BTreeSet::new()).iter().map(|r| r.q.n).collect::<Vec<_>>());
        }
        let v = read_verdicts("# header\nbad\nt\t1\t0\t0\t0\t0\nt\t1\t1\t0.5\t0\t1");
        assert_eq!(v[&("t".into(), 1)], [1.0, 0.5, 0.0, 1.0]);
    }
}
