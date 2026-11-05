//! Measures: how many rules, their precision on a hand-checked sample, the lattices of the example cases, and the
//! controls that must go red.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result};

use crate::juris::Jurisdiction;
use crate::rule::{Force, Rule};

/// The sample for the gold: every rule read from one of the gold's sentences (the gold fixes the sentences, so a
/// change of the extractor is measured on the same text).
pub fn sample<'a>(js: &'a [Jurisdiction], gold: &BTreeMap<String, Gold>) -> Vec<&'a Rule> {
    let mut out = Vec::new();
    for j in js {
        out.extend(j.rules.iter().filter(|r| r.jur == j.id && gold.contains_key(&r.sent)));
    }
    out
}

/// The 25 + 25 sentences the gold was written for, chosen once (2026-10-18) as every n-th rule of the first
/// extraction: kept here as the gold's own order.
pub fn gold_order(path: &Path) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)?.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#') && !l.starts_with("sent\t")).map(|l| l.split('\t').next().unwrap_or("").to_string()).collect())
}

/// Recall: the share of the gold's rules that some extracted rule of the same sentence matches (force and an act).
pub fn recall(sample: &[&Rule], gold: &BTreeMap<String, Gold>) -> String {
    let (mut n, mut hit) = (0usize, 0usize);
    for (sent, g) in gold {
        for (f, vs) in &g.rules {
            n += 1;
            let found = sample.iter().filter(|r| &r.sent == sent).any(|r| {
                let verbs: Vec<&str> = r.acts.iter().map(|c| c.verb.as_str()).chain(r.conds.iter().flatten().map(|c| c.verb.as_str())).collect();
                r.force == *f && vs.iter().any(|v| verbs.contains(&v.as_str()))
            });
            hit += found as usize;
        }
    }
    format!("{hit} of {n} gold rules found ({:.0}%)", if n == 0 { 0.0 } else { 100.0 * hit as f64 / n as f64 })
}

/// A gold row: the rules a sentence truly holds (force and act verb), and who says them.
#[derive(Clone, Debug)]
pub struct Gold {
    pub rules: Vec<(Force, Vec<String>)>,
    pub authority: String,
}

/// `gold.tsv`: `sent<TAB>force:verb|verb;force:verb<TAB>authority<TAB>note`; `none` when the sentence holds no rule.
pub fn read_gold(path: &Path) -> Result<BTreeMap<String, Gold>> {
    let mut out = BTreeMap::new();
    for l in std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?.lines() {
        if l.trim().is_empty() || l.starts_with('#') || l.starts_with("sent\t") {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        let mut rules = Vec::new();
        if c.get(1).is_some_and(|x| *x != "none") {
            for r in c[1].split(';') {
                let (f, v) = r.split_once(':').with_context(|| format!("bad gold {r}"))?;
                rules.push((Force::parse(f).with_context(|| format!("bad force {f}"))?, v.split('|').map(str::to_string).collect()));
            }
        }
        out.insert(c[0].to_string(), Gold { rules, authority: c.get(2).unwrap_or(&"").to_string() });
    }
    Ok(out)
}

/// Is a rule right by the gold: its force with its act verb among the gold's; its authority.
pub fn judge_rule(r: &Rule, g: &Gold) -> (bool, bool) {
    let verbs: Vec<&str> = r.acts.iter().map(|c| c.verb.as_str()).chain(r.conds.iter().flatten().map(|c| c.verb.as_str())).collect();
    let fa = g.rules.iter().any(|(f, vs)| *f == r.force && vs.iter().any(|v| verbs.contains(&v.as_str())));
    let au = g.authority.eq_ignore_ascii_case(&r.authority);
    (fa, au)
}

/// The authority of a gold row is scored only when the text says who speaks.
pub fn authority_known(g: &Gold) -> bool {
    g.authority != "?"
}

/// Precision of the rules against the gold of their own sentence; with `shift`, against the gold of the sentence
/// `shift` places later in the gold's order (the control).
pub fn precision(sample: &[&Rule], gold: &BTreeMap<String, Gold>, order: &[String], shift: usize) -> String {
    let golds: Vec<Option<&Gold>> = sample
        .iter()
        .map(|r| {
            let k = order.iter().position(|s| s == &r.sent)?;
            gold.get(&order[(k + shift) % order.len()])
        })
        .collect();
    let n = sample.len();
    let (mut judged, mut fa, mut au, mut both, mut au_n) = (0, 0, 0, 0, 0);
    for (i, r) in sample.iter().enumerate() {
        let Some(g) = golds[i] else { continue };
        judged += 1;
        let (a, b) = judge_rule(r, g);
        let known = authority_known(g);
        fa += a as usize;
        au_n += known as usize;
        au += (b && known) as usize;
        both += (a && (b || !known)) as usize;
    }
    let pct = |x: usize, n: usize| if n == 0 { 0.0 } else { 100.0 * x as f64 / n as f64 };
    format!("{judged} rules of {n} judged: force+act {fa} ({:.0}%), authority {au} of {au_n} ({:.0}%), both {both} ({:.0}%)", pct(fa, judged), pct(au, au_n), pct(both, judged))
}

/// The rule counts of every jurisdiction.
pub fn counts(js: &[Jurisdiction]) -> String {
    let mut o = String::new();
    for j in js {
        let own: Vec<&Rule> = j.rules.iter().filter(|r| r.jur == j.id).collect();
        if j.rules.is_empty() {
            let _ = writeln!(o, "{:<14} {:<7} no rules yet ({})", j.id, j.status(), j.kind());
            continue;
        }
        let mut by_force: BTreeMap<&str, usize> = BTreeMap::new();
        let mut by_auth: BTreeMap<String, usize> = BTreeMap::new();
        let mut disputes: BTreeMap<&str, usize> = BTreeMap::new();
        let (mut conds, mut excs, mut pen) = (0, 0, 0);
        for r in &own {
            *by_force.entry(r.force.name()).or_default() += 1;
            *by_auth.entry(j.authority(&r.authority).0).or_default() += 1;
            if !r.dispute.is_empty() {
                *disputes.entry(r.dispute.as_str()).or_default() += 1;
            }
            conds += (!r.conds.is_empty()) as usize;
            excs += (!r.excs.is_empty()) as usize;
            pen += (!r.penalty.is_empty()) as usize;
        }
        let _ = writeln!(o, "{:<14} {:<7} {:>5} rules (+{} inherited); with conditions {conds}, exceptions {excs}, a penalty {pen}; disputes {} ({} rules)", j.id, j.status(), own.len(), j.rules.len() - own.len(), disputes.len(), disputes.values().sum::<usize>());
        let _ = writeln!(o, "    force: {}", by_force.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
        let _ = writeln!(o, "    authority class: {}", by_auth.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
    }
    o
}
