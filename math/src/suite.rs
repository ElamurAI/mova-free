//! Run of the test suite: `id <TAB> question <TAB> expected`. Verdict — hit, honest «don't understand»,
//! wrong (wrong answer, wrong unit, or an answer where an error or «don't understand» was expected).

use std::collections::BTreeMap;

use anyhow::{Context, Result, bail};

use crate::err::Error;
use crate::eval::{Num, answer, fmt_q};
use crate::rat::Rat;
use crate::understand::Understander;
use crate::units::{Mono, Scale, Unit};

/// Expected answer.
pub enum Want {
    Value { approx: bool, val: Rat, unit: WantUnit },
    /// NU, ERR:dim, ERR:temp, ERR:overflow, ERR:div0.
    Code(String),
}

#[derive(PartialEq, Debug)]
pub enum WantUnit {
    Plain,
    Mono(Mono),
    Point(Scale),
}

pub fn parse_want(s: &str) -> Result<Want> {
    let s = s.trim();
    if s == "NU" || s.starts_with("ERR:") {
        return Ok(Want::Code(s.to_string()));
    }
    let (approx, rest) = match s.strip_prefix('≈') {
        Some(r) => (true, r.trim()),
        None => (false, s),
    };
    let (n, unit) = match rest.split_once(' ') {
        Some((a, b)) => (a, b.trim()),
        None => (rest, ""),
    };
    let val = Rat::parse(n).with_context(|| format!("number «{n}»"))?;
    let unit = if unit.is_empty() {
        WantUnit::Plain
    } else if let Some(sc) = Scale::parse_sym(unit) {
        WantUnit::Point(sc)
    } else {
        WantUnit::Mono(Mono::parse_sym(unit).with_context(|| format!("unit «{unit}»"))?)
    };
    Ok(Want::Value { approx, val, unit })
}

fn sorted(m: &Mono) -> Vec<(Unit, i8)> {
    let mut v = m.0.clone();
    v.sort();
    v
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Verdict {
    Hit,
    HonestNu,
    Wrong,
}

pub fn judge(u: &Understander, q: &str, want: &Want) -> (Verdict, String) {
    let res = u.understand(q).and_then(|p| answer(&p.expr).map(|a| (p, a)));
    match (res, want) {
        (Ok((p, a)), Want::Value { approx, val, unit }) => {
            let unit_ok = match unit {
                WantUnit::Plain => a.q.point.is_none() && a.q.u.is_one(),
                WantUnit::Point(s) => a.q.point == Some(*s),
                WantUnit::Mono(m) => a.q.point.is_none() && sorted(&a.q.u) == sorted(m),
            };
            let val_ok = match (a.q.n, approx) {
                (Num::Exact(r), false) => r == *val,
                (n, _) => (n.to_f64() - val.to_f64()).abs() <= 1e-6 * val.to_f64().abs().max(1e-12),
            };
            let got = format!("{}  ⟵ {}", fmt_q(&a.q), p.expr);
            if unit_ok && val_ok { (Verdict::Hit, got) } else { (Verdict::Wrong, got) }
        }
        (Ok((p, a)), Want::Code(c)) => (Verdict::Wrong, format!("{} instead of {c}  ⟵ {}", fmt_q(&a.q), p.expr)),
        (Err(e @ Error::NotUnderstood(_)), Want::Code(c)) if c == "NU" => (Verdict::Hit, e.to_string()),
        (Err(e @ Error::NotUnderstood(_)), _) => (Verdict::HonestNu, e.to_string()),
        (Err(e), Want::Code(c)) if e.code() == c => (Verdict::Hit, e.to_string()),
        (Err(e), _) => (Verdict::Wrong, format!("{}: {e}", e.code())),
    }
}

/// A report row.
pub struct Row {
    pub id: String,
    pub question: String,
    pub want: String,
    pub verdict: Verdict,
    pub got: String,
}

/// Summary: rows and counters per category (id prefix before «-»): [hit, NU, wrong].
pub struct Report {
    pub rows: Vec<Row>,
    pub by_cat: BTreeMap<String, [usize; 3]>,
    pub total: [usize; 3],
}

pub fn run(u: &Understander, text: &str) -> Result<Report> {
    let mut rep = Report { rows: Vec::new(), by_cat: BTreeMap::new(), total: [0; 3] };
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() != 3 {
            bail!("suite row without three columns: {line}");
        }
        let want = parse_want(c[2])?;
        let (verdict, got) = judge(u, c[1], &want);
        let slot = verdict as usize;
        rep.by_cat.entry(c[0].split('-').next().unwrap_or("").to_string()).or_default()[slot] += 1;
        rep.total[slot] += 1;
        rep.rows.push(Row { id: c[0].into(), question: c[1].into(), want: c[2].into(), verdict, got });
    }
    Ok(rep)
}
