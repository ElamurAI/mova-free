//! The absurd lens: the moments where reasoning turns absurd, found by measurable criteria only. Each flag carries
//! its chain and sources and a score 0–4 by a fixed rule, on the scale of Mova's absurdity matrix (0 none … 4 the
//! most absurd). It judges paths, never people.
//!
//! 1. purpose defeated: the rule's own stated reason is there in the case, and the path frees the defendant from
//!    the rule all the same — by a device (3) or by the narrow reading of a word (2);
//! 2. disproportion: the verdict's class on the shared scale is 3 or more steps from the harm the case tells of
//!    (3 steps: 2, 4 steps: 3, 5 or more: 4);
//! 3. contradiction: on the honest path, two rules of the same jurisdiction give a guilty and a free verdict on the
//!    same facts and the declared principle cannot decide between them (the body's order does) (3);
//! 4. literalism: the honest verdict turns between free and guilty on the width of one word alone (2; 3 when the
//!    two verdicts are 5 or more classes apart);
//! 5. status-only: the facts unchanged and only a party's status changed (Jew/gentile, slave/free, man/woman,
//!    citizen/foreigner, Israelite/stranger), the honest verdict changes (3; 4 when 3 or more classes apart).
//!
//! A path's score is the highest of its flags; a case with no flag scores 0.

use std::fmt::Write as _;

use crate::court::{self, Case, PointKind};
use crate::juris::{Cover, Jurisdiction};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flag {
    pub kind: &'static str,
    pub score: u8,
    pub what: String,
    /// the chain of the path flagged
    pub chain: Vec<String>,
}

/// The pairs of status words the status test swaps, both ways.
pub const STATUS_PAIRS: [(&str, &str); 6] = [("jew", "gentile"), ("israelite", "stranger"), ("slave", "freeman"), ("man", "woman"), ("citizen", "foreigner"), ("hebrew", "stranger")];

fn guilty(sev: u8) -> bool {
    sev >= 2
}

/// The case with every status word swapped for its pair (the facts otherwise the same).
pub fn swap_status(case: &Case, a: &str, b: &str) -> Case {
    let mut c = case.clone();
    let sw = |w: &mut String| {
        if w == a {
            *w = b.to_string();
        } else if w == b {
            *w = a.to_string();
        }
    };
    for f in &mut c.facts {
        if let Some(x) = f.agent.as_mut() {
            sw(x);
        }
        if let Some(x) = f.patient.as_mut() {
            sw(x);
        }
        for arg in &mut f.args {
            if let Some((k, v)) = arg.rsplit_once(':') {
                let mut v = v.to_string();
                sw(&mut v);
                *arg = format!("{k}:{v}");
            }
        }
    }
    for p in &mut c.persons {
        sw(p);
    }
    c
}

/// Every flag of one jurisdiction for one defendant.
pub fn flags(j: &Jurisdiction, case: &Case, defendant: &str, depth: usize) -> Vec<Flag> {
    let mut out = Vec::new();
    let l = court::lattice(j, case, defendant);
    let base = court::honest(&l);
    let (hv, _hw, hchain) = court::evaluate(j, &l, &base);
    let harm = crate::report::harm_of(case);
    let src = |k: usize| format!("{} [{}; {}]", l.rules[k].rule.id, l.rules[k].rule.source, l.rules[k].rule.authority);

    // 1. purpose defeated, by a narrow reading: a rule whose stated reason the case shows does not fire on the
    //    honest path only because a word is read narrowly
    for (k, m) in l.rules.iter().enumerate() {
        if m.rule.reason.is_empty() {
            continue;
        }
        let shown = reason_in_case(j, &m.rule.reason, case);
        if !shown {
            continue;
        }
        for (pi, p) in l.points.iter().enumerate() {
            if p.rule == Some(k) && p.kind == PointKind::Reading && base[pi] != p.holds {
                let mut a = base.clone();
                a[pi] = p.holds;
                let (v2, _, ch2) = court::evaluate(j, &l, &a);
                if v2.severity > hv.severity {
                    out.push(Flag { kind: "purpose defeated", score: 2, what: format!("{} gives its reason «{}», which the case shows; the narrow reading «{}» frees the defendant ({} instead of {})", src(k), m.rule.reason, p.desc, hv.label, v2.label), chain: ch2 });
                }
            }
        }
    }
    // 1. purpose defeated, by a device: a rule that fires without the device and not with it, whose stated reason
    //    the case shows, while the verdict gets milder
    let firing = |chain: &[String]| -> Vec<String> { chain.iter().filter(|w| w.contains(": fires (")).filter_map(|w| w.split(" [").next().map(str::to_string)).collect() };
    for (d, with, without) in court::device_views(j, case) {
        let lw = court::lattice(j, &with, defendant);
        let lo = court::lattice(j, &without, defendant);
        let (vw, _, cw) = court::evaluate(j, &lw, &court::honest(&lw));
        let (vo, _, co) = court::evaluate(j, &lo, &court::honest(&lo));
        if vw.severity >= vo.severity {
            continue;
        }
        let (fw, fo) = (firing(&cw), firing(&co));
        for m in &lo.rules {
            let r = &m.rule;
            if fo.contains(&r.id) && !fw.contains(&r.id) && !r.reason.is_empty() && reason_in_case(j, &r.reason, &without) {
                out.push(Flag { kind: "purpose defeated", score: 3, what: format!("{} [{}] gives its reason «{}», which the case shows; {} keeps its letter and the verdict becomes {} instead of {}", r.id, r.source, r.reason, d.name, vw.label, vo.label), chain: cw.clone() });
            }
        }
    }
    // 2. disproportion: the honest verdict, and each verdict a judge reaches within the depth
    let tips = court::tipping(j, &l, depth, 1);
    for (v, ts) in &tips {
        let gap = (v.severity as i32 - harm as i32).unsigned_abs();
        // a penalty imposed (an exemption or freedom is not a penalty)
        if gap >= 3 && v.severity >= 1 && ts[0].winner.is_some() {
            let a: Vec<usize> = {
                let mut a = base.clone();
                for &(p, o) in &ts[0].points {
                    a[p] = o;
                }
                a
            };
            let score = match gap {
                3 => 2,
                4 => 3,
                _ => 4,
            };
            out.push(Flag { kind: "disproportion", score, what: format!("{} (class {}) for a harm of class {} — {} classes apart, {} tipped points away from the honest path", v.label, v.severity, harm, gap, ts[0].points.len()), chain: court::evaluate(j, &l, &a).2 });
        }
    }
    // 3. contradiction the declared principle cannot settle
    if hchain.iter().any(|w| w.starts_with("no principle decides")) {
        out.push(Flag { kind: "contradiction", score: 3, what: "rules of this jurisdiction give a guilty and a free verdict on the same facts, and its declared principle ties: the order of the body decides".into(), chain: hchain.clone() });
    }
    // 4. literalism: one word's width turns the honest verdict between free and guilty
    for (pi, p) in l.points.iter().enumerate() {
        if p.kind != PointKind::Reading {
            continue;
        }
        let mut a = base.clone();
        a[pi] = 1 - base[pi].min(1);
        let (v2, _, ch2) = court::evaluate(j, &l, &a);
        if guilty(v2.severity) != guilty(hv.severity) {
            let gap = (v2.severity as i32 - hv.severity as i32).unsigned_abs();
            let rule = p.rule.map(src).unwrap_or_default();
            out.push(Flag { kind: "literalism", score: if gap >= 5 { 3 } else { 2 }, what: format!("{rule}: «{}» read {} gives {}, read {} gives {}", p.desc, p.options[base[pi]], hv.label, p.options[a[pi]], v2.label), chain: ch2 });
        }
    }
    // 5. status-only
    let words = court::case_words(case);
    for (a, b) in STATUS_PAIRS {
        if !words.iter().any(|w| w == a || w == b) {
            continue;
        }
        let c2 = swap_status(case, a, b);
        let d2 = if defendant == a { b } else if defendant == b { a } else { defendant };
        let l2 = court::lattice(j, &c2, d2);
        let (v2, _, ch2) = court::evaluate(j, &l2, &court::honest(&l2));
        if v2.severity != hv.severity {
            let gap = (v2.severity as i32 - hv.severity as i32).unsigned_abs();
            out.push(Flag { kind: "status-only", score: if gap >= 3 { 4 } else { 3 }, what: format!("with «{a}» and «{b}» swapped and the facts the same, the verdict for {d2} is {} instead of {}", v2.label, hv.label), chain: ch2 });
        }
    }
    out.sort_by(|x, y| y.score.cmp(&x.score).then(x.kind.cmp(y.kind)).then(x.what.cmp(&y.what)));
    out.dedup();
    out
}

/// Is a rule's stated reason there in the case: its verb, or the thing it names, said by some fact?
fn reason_in_case(j: &Jurisdiction, reason: &str, case: &Case) -> bool {
    let body = reason.split(';').flat_map(|r| r.split_once(": ").map(|x| x.1)).collect::<Vec<_>>();
    body.iter().any(|r| {
        let ws: Vec<&str> = r.split_whitespace().map(|w| w.rsplit_once(':').map(|x| x.1).unwrap_or(w)).filter(|w| !matches!(*w, "be" | "he" | "they" | "ye" | "thou" | "i" | "you" | "it" | "not")).collect();
        ws.iter().any(|w| case.facts.iter().any(|f| j.cover(w, &f.verb) == Cover::Sure || f.patient.as_deref().is_some_and(|p| j.cover(w, p) == Cover::Sure) || f.args.iter().any(|a| j.cover(w, a.rsplit_once(':').map(|x| x.1).unwrap_or(a)) == Cover::Sure)))
    })
}

/// The lens as text.
pub fn lens(js: &[Jurisdiction], case: &Case, only: &[String], depth: usize, max: usize) -> String {
    let mut o = String::new();
    let _ = writeln!(o, "\n-- Absurdity (measurable criteria only; a score 0–4 on the absurdity matrix's scale; it judges paths, not people)");
    let mut any = false;
    for d in court::defendants(case) {
        for j in js {
            if j.rules.is_empty() || (!only.is_empty() && !only.contains(&j.id)) {
                continue;
            }
            let fl = flags(j, case, &d, depth);
            if fl.is_empty() {
                continue;
            }
            any = true;
            let _ = writeln!(o, "  {} / {d}: score {} — {} flags", j.id, fl[0].score, fl.len());
            for f in fl.iter().take(max) {
                let _ = writeln!(o, "    [{}] {}: {}", f.score, f.kind, f.what);
                for w in f.chain.iter().filter(|w| w.contains("fires (") || w.starts_with("verdict") || w.contains("prevails") || w.starts_with("no principle") || w.starts_with("no rule")) {
                    let _ = writeln!(o, "        · {w}");
                }
            }
            if fl.len() > max {
                let _ = writeln!(o, "    … {} more", fl.len() - max);
            }
        }
    }
    if !any {
        let _ = writeln!(o, "  none: score 0 — no path of this case meets a criterion");
    }
    o
}
