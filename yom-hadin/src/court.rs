//! The Court: a case → its facts (frames) → the rules each jurisdiction has for it → a lattice of every way the case
//! can be judged. A path is one choice at every choice point:
//! - the reading of each term a rule needs only broadly («took» as «steal»): broad or narrow;
//! - each condition the case does not settle: met or not proven;
//! - each exception the case does not settle: applies or not;
//! - each dispute: whose word prevails;
//! - the principle that settles conflicting rules (the jurisdiction's own list);
//! - where the jurisdiction allows it, suspending the sentence.
//! The honest path takes the plain choice at every point: the terms as the jurisdiction reads them plainly, what is
//! not stated as not proven, the majority in a dispute, the declared precedence, no suspension.
//! Everything is counted exactly and in a fixed order; nothing is sampled.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::juris::{Cover, Jurisdiction, Principle};
use crate::rule::{Clause, Force, Rule};

/// One fact of the case, read from a frame.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fact {
    pub verb: String,
    pub neg: bool,
    pub agent: Option<String>,
    pub patient: Option<String>,
    /// other role fillers as `key:lemma` (`from:corner`, `when:night`, `place:field`)
    pub args: Vec<String>,
}

impl Fact {
    pub fn show(&self) -> String {
        let mut s = format!("{} {}{}", self.agent.as_deref().unwrap_or("_"), if self.neg { "NOT " } else { "" }, self.verb);
        if let Some(p) = &self.patient {
            s += &format!(" {p}");
        }
        for a in &self.args {
            s += &format!(" {a}");
        }
        s
    }
}

#[derive(Clone, Debug, Default)]
pub struct Case {
    pub text: String,
    pub facts: Vec<Fact>,
    /// places named in the case (for lex loci)
    pub places: Vec<String>,
    /// words of status for the parties (for the personal law)
    pub persons: Vec<String>,
}

/// Generic fillers that say nothing about which thing: they match anything.
const GENERIC: [&str; 12] = ["it", "thing", "anything", "something", "that", "this", "which", "what", "whatsoever", "aught", "ought", "whosoever"];
/// Words of status a rule may be addressed to.
const STATUS: [&str; 7] = ["jew", "israelite", "hebrew", "gentile", "priest", "levite", "nazirite"];
/// Pronouns of persons: they cover a case's word only on a broad reading («sell she» is not «sell ox»).
const PERSONAL: [&str; 7] = ["they", "them", "he", "him", "she", "her", "whoever"];

fn val(a: &str) -> &str {
    a.rsplit_once(':').map(|x| x.1).unwrap_or(a)
}

/// Read a case through en → prag → world.
pub fn read_case(text: &str) -> anyhow::Result<Case> {
    let mut ctx = prag::frames::Ctx::default();
    let r = world::frames::read(text, &mut ctx, false)?;
    Ok(case_of_frames(text, &r.frames))
}

pub fn case_of_frames(text: &str, frames: &[prag::frames::Frame]) -> Case {
    use prag::frames::Kind;
    let mut c = Case { text: text.to_string(), ..Default::default() };
    for f in frames {
        if f.kind == Kind::Bare || f.verb.is_empty() {
            continue;
        }
        let mut args: Vec<String> = Vec::new();
        for a in &f.attrs {
            if a.starts_with("when:") {
                args.push(a.clone());
            } else if !f.manner.contains(a) {
                args.push(format!("attr:{a}"));
            }
        }
        if let Some(p) = &f.place {
            args.push(format!("place:{}", p.id));
            c.places.push(p.id.clone());
            if p.named {
                c.places.push(p.text.to_lowercase());
            }
        }
        if let Some(r) = &f.recipient {
            args.push(format!("to:{}", r.id));
        }
        for (prep, e) in &f.obls {
            args.push(format!("{prep}:{}", e.id));
            if matches!(prep.as_str(), "in" | "at" | "within") {
                c.places.push(e.id.clone());
            }
        }
        for e in [&f.agent, &f.patient, &f.recipient].into_iter().flatten() {
            c.persons.push(e.id.clone());
            c.persons.extend(e.attrs.iter().cloned());
        }
        // a passive with its agent unsaid: «the ox was stolen» — someone stole it
        let agent = f.agent.as_ref().map(|e| e.id.clone()).filter(|a| a != "_");
        c.facts.push(Fact { verb: f.verb.clone(), neg: f.neg, agent, patient: f.patient.as_ref().map(|e| e.id.clone()), args });
    }
    c.places.sort();
    c.places.dedup();
    c.persons.sort();
    c.persons.dedup();
    c
}

/// The parties who did something: the agents of the facts, in order (`someone` for an unsaid agent).
pub fn defendants(c: &Case) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for f in &c.facts {
        let a = f.agent.clone().unwrap_or_else(|| "someone".into());
        if f.verb != "be" && !out.contains(&a) {
            out.push(a);
        }
    }
    out
}

/// How a clause of a rule stands against the facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Status {
    /// the case says the opposite, or says it did not happen
    Unmet,
    /// the case is silent: a choice point (met or not proven)
    Unknown,
    /// met only on the broad reading of a term: a choice point
    Broad,
    Met,
}

fn cover_word(j: &Jurisdiction, rule_w: &str, case_w: Option<&str>) -> Cover {
    if GENERIC.contains(&rule_w) {
        return Cover::Sure;
    }
    if PERSONAL.contains(&rule_w) {
        return Cover::Broad;
    }
    // alternatives: «ox~sheep» covers what either covers
    if rule_w.contains('~') {
        return rule_w.split('~').map(|w| cover_word(j, w, case_w)).max().unwrap_or(Cover::No);
    }
    match case_w {
        None => Cover::Broad,
        Some(c) if GENERIC.contains(&c) => Cover::Broad,
        Some(c) => j.cover(rule_w, c),
    }
}

/// A clause against one fact: the cover of the verb and patient; the args only as far as the fact says them.
fn clause_fact(j: &Jurisdiction, cl: &Clause, f: &Fact) -> (Cover, Option<String>) {
    let v = j.cover(&cl.verb, &f.verb);
    if v == Cover::No {
        return (Cover::No, None);
    }
    let p = match &cl.patient {
        None => Cover::Sure,
        Some(rp) => cover_word(j, rp, f.patient.as_deref()),
    };
    if p == Cover::No {
        return (Cover::No, None);
    }
    let mut why = None;
    let c = v.min(p);
    if c == Cover::Broad {
        let (rw, cw) = if v == Cover::Broad { (cl.verb.clone(), f.verb.clone()) } else { (cl.patient.clone().unwrap_or_default(), f.patient.clone().unwrap_or_else(|| "(unsaid)".into())) };
        why = Some(format!("«{cw}» as «{rw}»"));
    }
    (c, why)
}

/// A condition clause against all the facts.
fn clause_status(j: &Jurisdiction, cl: &Clause, facts: &[Fact]) -> (Status, String) {
    // the opposite said: «at night» against «if the sun be risen»
    for f in facts {
        let mut words: Vec<&str> = vec![f.verb.as_str()];
        words.extend(f.patient.as_deref());
        words.extend(f.args.iter().map(|a| val(a)));
        let mut rule_words: Vec<&str> = vec![cl.verb.as_str()];
        rule_words.extend(cl.args.iter().map(|a| val(a)));
        for rw in &rule_words {
            for w in &words {
                if j.opposite(rw, w) {
                    return (Status::Unmet, format!("the case says «{w}», the opposite of «{rw}»"));
                }
            }
        }
    }
    let mut best = (Status::Unknown, "the case does not say".to_string());
    for f in facts {
        let (c, why) = clause_fact(j, cl, f);
        if c == Cover::No {
            continue;
        }
        if f.neg != cl.neg && c == Cover::Sure {
            return (Status::Unmet, format!("the case says «{}»", f.show()));
        }
        if f.neg != cl.neg {
            continue;
        }
        // the clause's own args must be said by the fact (or another fact) to be sure
        let args_ok = cl.args.iter().all(|a| facts.iter().any(|g| g.args.iter().any(|b| j.cover(val(a), val(b)) == Cover::Sure) || g.patient.as_deref().is_some_and(|p| j.cover(val(a), p) == Cover::Sure)));
        let st = match (c, args_ok) {
            (Cover::Sure, true) => Status::Met,
            (Cover::Sure, false) => Status::Unknown,
            _ => Status::Broad,
        };
        let w = match st {
            Status::Met => format!("the case says «{}»", f.show()),
            Status::Broad => format!("{} (broad reading)", why.unwrap_or_default()),
            _ => format!("«{}» is said, its {} not", f.show(), cl.args.join(" ")),
        };
        if st > best.0 {
            best = (st, w);
        }
    }
    best
}

/// A choice point.
#[derive(Clone, Debug)]
pub struct Point {
    /// the matched rule it belongs to (index into `Lattice::rules`), none for the global points
    pub rule: Option<usize>,
    pub kind: PointKind,
    pub desc: String,
    pub options: Vec<String>,
    /// the honest choice
    pub plain: usize,
    /// for a rule's point: the option under which the rule's element holds
    pub holds: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointKind {
    Reading,
    Condition,
    Exception,
    Dispute,
    Precedence,
    Suspend,
}

impl PointKind {
    pub fn name(self) -> &'static str {
        match self {
            PointKind::Reading => "reading",
            PointKind::Condition => "condition",
            PointKind::Exception => "exception",
            PointKind::Dispute => "dispute",
            PointKind::Precedence => "precedence",
            PointKind::Suspend => "suspension",
        }
    }
}

/// A rule that matched the case, with what it rests on.
#[derive(Clone, Debug)]
pub struct Matched {
    pub rule: Rule,
    /// the order of the rule in its jurisdiction (for lex posterior)
    pub order: usize,
    pub anchor: String,
    /// the defendant did not do the act («did not pay»)
    pub neg_act: bool,
    /// for a duty: 0 done, 1 not said, 2 not done
    pub duty: Option<u8>,
    /// set aside whatever is chosen, and why
    pub blocked: Option<String>,
    /// what holds for certain
    pub sure: Vec<String>,
    pub specificity: usize,
    pub auth_rank: u8,
    pub auth_class: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Verdict {
    pub severity: u8,
    pub label: String,
}

impl Verdict {
    pub fn none() -> Verdict {
        Verdict { severity: 0, label: "no rule applies".into() }
    }
    pub fn free() -> Verdict {
        Verdict { severity: 0, label: "free: no rule fires".into() }
    }
    pub fn acquittal(&self) -> bool {
        self.severity == 0
    }
}

/// The lattice of one jurisdiction for one defendant.
#[derive(Clone, Debug)]
pub struct Lattice {
    pub jur: String,
    pub defendant: String,
    pub rules: Vec<Matched>,
    /// rules matched but left out by the cap (the weakest anchors, latest in the body)
    pub capped: usize,
    pub points: Vec<Point>,
    /// paths per verdict, and the first path (an assignment) that gives it
    pub counts: BTreeMap<Verdict, (u128, Vec<usize>)>,
    pub total: u128,
}

pub const MAX_RULES: usize = 16;

fn plain_verdict(j: &Jurisdiction, m: &Matched) -> Verdict {
    let r = &m.rule;
    let act = r.act_verb().unwrap_or("the act").to_string();
    match r.force {
        Force::Liability => {
            let (sev, _) = j.severity(&r.penalty).unwrap_or((2, String::new()));
            let p = if r.penalty.is_empty() { "liable".to_string() } else { r.penalty.clone() };
            Verdict { severity: sev.max(1), label: format!("liable: {p}") }
        }
        Force::Prohibition => Verdict { severity: 2, label: format!("guilty: forbidden «{act}»") },
        Force::Obligation => match m.duty.unwrap_or(1) {
            0 => Verdict { severity: 0, label: format!("duty «{act}» done") },
            2 => Verdict { severity: 2, label: format!("guilty: duty «{act}» not done") },
            _ => Verdict { severity: 1, label: format!("bound: duty «{act}»") },
        },
        Force::Permission => Verdict { severity: 0, label: format!("permitted: «{act}»") },
        Force::Exemption => Verdict { severity: 0, label: "exempt".into() },
    }
}

/// Build the lattice: match the rules, find the choice points, count every path.
pub fn lattice(j: &Jurisdiction, case: &Case, defendant: &str) -> Lattice {
    let mine: Vec<&Fact> = case.facts.iter().filter(|f| f.agent.clone().unwrap_or_else(|| "someone".into()) == defendant).collect();
    let mut found: Vec<(Cover, usize, usize, Matched, Vec<Point>)> = Vec::new();
    for (order, r) in j.rules.iter().enumerate() {
        // a rule addressed to a status binds only a defendant of that status («a Jew must not light a fire»)
        if let Some(who) = r.parties.iter().find_map(|p| p.strip_prefix("agent=")) {
            if STATUS.contains(&who) && who != defendant && j.cover(who, defendant) == Cover::No {
                continue;
            }
        }
        // the anchor: an act, or a clause of a condition, that the defendant's own facts cover
        let mut cands: Vec<(Clause, Option<usize>)> = r.acts.iter().map(|c| (c.clone(), None)).collect();
        for (gi, g) in r.conds.iter().enumerate() {
            cands.extend(g.iter().map(|c| (c.clone(), Some(gi))));
        }
        let mut best: Option<(Cover, Clause, Option<usize>, &Fact, Option<String>)> = None;
        for (cl, gi) in &cands {
            for f in &mine {
                let (c, why) = clause_fact(j, cl, f);
                let deontic_act = gi.is_none() && matches!(r.force, Force::Prohibition | Force::Obligation | Force::Permission);
                if c == Cover::No || (f.neg != cl.neg && !deontic_act) {
                    continue;
                }
                if best.as_ref().is_none_or(|b| c > b.0) {
                    best = Some((c, cl.clone(), *gi, f, why));
                }
            }
        }
        let Some((cov, cl, gi, f, why)) = best else { continue };
        let k = found.len();
        let mut pts: Vec<Point> = Vec::new();
        let mut sure: Vec<String> = Vec::new();
        let mut blocked = None;
        let neg_act = gi.is_none() && f.neg != cl.neg;
        if r.force == Force::Prohibition && neg_act {
            blocked = Some(format!("the case says «{}»: the forbidden act was not done", f.show()));
        }
        let anchor = format!("«{}» ~ «{}»", f.show(), cl.show());
        if cov == Cover::Broad {
            let term = j.term_of(&cl.verb, &f.verb).or_else(|| cl.patient.as_deref().and_then(|p| j.term_of(p, f.patient.as_deref().unwrap_or(""))));
            let plain_broad = term.is_some_and(|t| t.plain_broad);
            pts.push(Point { rule: Some(k), kind: PointKind::Reading, desc: format!("act {}", why.unwrap_or_default()), options: vec!["broad".into(), "narrow".into()], plain: if plain_broad { 0 } else { 1 }, holds: 0 });
        } else {
            sure.push(format!("act {anchor}"));
        }
        // the acts the anchor is not: what the rule speaks of must be shown too («thou shalt not wrong» when selling)
        // (a duty's act is not a condition: whether it was done decides the verdict, see `duty`)
        let mut groups: Vec<Vec<Clause>> = if r.force == Force::Obligation { Vec::new() } else { r.acts.iter().filter(|c| *c != &cl || gi.is_some()).map(|c| vec![c.clone()]).collect() };
        // the anchor's own role fillers the fact does not say: «sleep with:pledge» against «slept on the mat»
        for a in &cl.args {
            let said = f.args.iter().any(|b| j.cover(val(a), val(b)) == Cover::Sure);
            if !said {
                groups.push(vec![Clause { verb: cl.verb.clone(), neg: cl.neg, patient: None, args: vec![a.clone()] }]);
            }
        }
        let duty: Option<u8> = if r.force == Force::Obligation {
            if gi.is_none() {
                Some(if neg_act { 2 } else { 0 })
            } else {
                let st = r.acts.first().map(|c| clause_status(j, c, &case.facts).0).unwrap_or(Status::Unknown);
                Some(match st {
                    Status::Met => 0,
                    Status::Unmet => 2,
                    _ => 1,
                })
            }
        } else {
            None
        };
        for (g, grp) in r.conds.iter().enumerate() {
            if Some(g) != gi {
                groups.push(grp.clone());
            }
        }
        for grp in groups.iter() {
            let st: Vec<(Status, String)> = grp.iter().map(|c| clause_status(j, c, &case.facts)).collect();
            let shown = grp.iter().map(Clause::show).collect::<Vec<_>>().join(" or ");
            let top = st.iter().max_by_key(|s| s.0).cloned().unwrap_or((Status::Unknown, String::new()));
            match top.0 {
                Status::Met => sure.push(format!("if «{shown}»: {}", top.1)),
                Status::Unmet => {
                    if blocked.is_none() {
                        blocked = Some(format!("if «{shown}»: {}", top.1));
                    }
                }
                Status::Broad => pts.push(Point { rule: Some(k), kind: PointKind::Reading, desc: format!("if «{shown}»: {}", top.1), options: vec!["broad".into(), "narrow".into()], plain: 1, holds: 0 }),
                Status::Unknown => pts.push(Point { rule: Some(k), kind: PointKind::Condition, desc: format!("if «{shown}»: {}", top.1), options: vec!["met".into(), "not proven".into()], plain: 1, holds: 0 }),
            }
        }
        for grp in &r.excs {
            let st: Vec<(Status, String)> = grp.iter().map(|c| clause_status(j, c, &case.facts)).collect();
            let shown = grp.iter().map(Clause::show).collect::<Vec<_>>().join(" and ");
            let low = st.iter().min_by_key(|s| s.0).cloned().unwrap_or((Status::Unknown, String::new()));
            match low.0 {
                Status::Met => {
                    if blocked.is_none() {
                        blocked = Some(format!("except «{shown}»: it holds"));
                    }
                }
                Status::Unmet => sure.push(format!("except «{shown}»: {}", low.1)),
                _ => pts.push(Point { rule: Some(k), kind: PointKind::Exception, desc: format!("except «{shown}»: {}", low.1), options: vec!["does not apply".into(), "applies".into()], plain: 0, holds: 0 }),
            }
        }
        // a duty the defendant did is nothing to judge: the rule is not matched at all
        if duty == Some(0) {
            continue;
        }
        let (auth_class, auth_rank) = j.authority(&r.authority);
        let specificity = r.conds.len() + r.excs.len() + usize::from(!r.acts.is_empty());
        // how much of the defendant's conduct the rule speaks to, and whether it sanctions
        let reach = mine.iter().filter(|g| cands.iter().any(|(c, _)| j.cover(&c.verb, &g.verb) != Cover::No)).count();
        // and how closely its anchor fits: the same thing named, the same circumstances said
        let close = usize::from(cl.patient.as_deref().is_some_and(|p| !GENERIC.contains(&p) && f.patient.as_deref().is_some_and(|q| j.cover(p, q) == Cover::Sure))) + cl.args.iter().filter(|a| f.args.iter().any(|b| j.cover(val(a), val(b)) == Cover::Sure)).count();
        let weight = reach * 4 + close * 3 + usize::from(cov == Cover::Sure) * 2 + usize::from(matches!(r.force, Force::Liability | Force::Prohibition | Force::Exemption));
        found.push((cov, weight, order, Matched { rule: r.clone(), order, anchor, neg_act, duty, blocked, sure, specificity, auth_rank, auth_class }, pts));
    }
    // the cap: the rules that speak to most of the defendant's conduct, sure anchors and sanctions first, then the
    // body's order
    let n_found = found.len();
    let mut idx: Vec<usize> = (0..n_found).collect();
    idx.sort_by(|&a, &b| found[b].1.cmp(&found[a].1).then(found[b].0.cmp(&found[a].0)).then(found[a].2.cmp(&found[b].2)));
    idx.truncate(MAX_RULES);
    idx.sort_by_key(|&i| found[i].2);
    let mut rules = Vec::new();
    let mut points = Vec::new();
    for (newk, &i) in idx.iter().enumerate() {
        let (_, _, _, m, pts) = &found[i];
        rules.push(m.clone());
        for p in pts {
            let mut p = p.clone();
            p.rule = Some(newk);
            points.push(p);
        }
    }
    // disputes: rules of one dispute with more than one authority — whose word prevails
    let mut disputes: BTreeMap<String, Vec<(u8, String)>> = BTreeMap::new();
    for m in &rules {
        if !m.rule.dispute.is_empty() {
            let e = disputes.entry(m.rule.dispute.clone()).or_default();
            if !e.iter().any(|x| x.1 == m.rule.authority) {
                e.push((m.auth_rank, m.rule.authority.clone()));
            }
        }
    }
    for (d, mut auths) in disputes {
        if auths.len() < 2 {
            continue;
        }
        auths.sort();
        points.push(Point { rule: None, kind: PointKind::Dispute, desc: d, options: auths.into_iter().map(|a| a.1).collect(), plain: 0, holds: 0 });
    }
    if rules.len() > 1 && j.principles.len() > 1 {
        points.push(Point { rule: None, kind: PointKind::Precedence, desc: "conflicting rules settled by".into(), options: j.principles.iter().map(|p| p.name().to_string()).collect(), plain: 0, holds: 0 });
    }
    if let Some((max, why)) = &j.suspend {
        if rules.iter().any(|m| m.rule.force == Force::Liability) {
            points.push(Point { rule: None, kind: PointKind::Suspend, desc: format!("suspend a sentence up to severity {max} ({why})"), options: vec!["no".into(), "suspend".into()], plain: 0, holds: 0 });
        }
    }
    let mut l = Lattice { jur: j.id.clone(), defendant: defendant.to_string(), rules, capped: n_found - idx.len(), points, counts: BTreeMap::new(), total: 0 };
    count(j, &mut l);
    l
}

/// The verdict of one path (an option per point), with its chain of why.
pub fn evaluate(j: &Jurisdiction, l: &Lattice, a: &[usize]) -> (Verdict, Option<usize>, Vec<String>) {
    let mut why: Vec<String> = Vec::new();
    if l.rules.is_empty() {
        return (Verdict::none(), None, vec!["no rule of this jurisdiction covers any act of the defendant".into()]);
    }
    let mut fires: Vec<usize> = Vec::new();
    for (k, m) in l.rules.iter().enumerate() {
        let head = format!("{} [{}; {}]", m.rule.id, m.rule.source, m.rule.authority);
        if let Some(b) = &m.blocked {
            why.push(format!("{head}: set aside — {b}"));
            continue;
        }
        let mut fail: Option<String> = None;
        let mut chosen: Vec<String> = Vec::new();
        for (pi, p) in l.points.iter().enumerate() {
            if p.rule != Some(k) {
                continue;
            }
            let o = a[pi];
            let tag = if o == p.plain { "" } else { " (tipped)" };
            if o != p.holds {
                fail.get_or_insert(format!("{}: {}{}", p.desc, p.options[o], tag));
            } else {
                chosen.push(format!("{}: {}{}", p.desc, p.options[o], tag));
            }
        }
        match fail {
            Some(f) => why.push(format!("{head}: does not fire — {f}")),
            None => {
                let mut s = format!("{head}: fires ({}) — act {}", m.rule.force.name(), m.anchor);
                for c in m.sure.iter().filter(|c| !c.starts_with("act ")).chain(chosen.iter()) {
                    s += &format!("; {c}");
                }
                why.push(s);
                fires.push(k);
            }
        }
    }
    // disputes: the authority chosen prevails over the other sides that fire
    for (pi, p) in l.points.iter().enumerate() {
        if p.kind != PointKind::Dispute {
            continue;
        }
        let win = &p.options[a[pi]];
        let sides: Vec<usize> = fires.iter().copied().filter(|&k| l.rules[k].rule.dispute == p.desc).collect();
        if sides.iter().any(|&k| &l.rules[k].rule.authority == win) && sides.iter().any(|&k| &l.rules[k].rule.authority != win) {
            fires.retain(|&k| l.rules[k].rule.dispute != p.desc || &l.rules[k].rule.authority == win);
            let tag = if a[pi] == p.plain { "" } else { " (tipped)" };
            why.push(format!("dispute {}: the word of {win} prevails{tag}", p.desc));
        }
    }
    if fires.is_empty() {
        why.push("no rule fires: the defendant goes free".into());
        return (Verdict::free(), None, why);
    }
    let verdicts: Vec<Verdict> = fires.iter().map(|&k| plain_verdict(j, &l.rules[k])).collect();
    let mut win = 0usize;
    let distinct = verdicts.iter().any(|v| v != &verdicts[0]);
    if distinct {
        let pi = l.points.iter().position(|p| p.kind == PointKind::Precedence);
        let principle = pi.map(|pi| Principle::parse(&l.points[pi].options[a[pi]]).unwrap()).unwrap_or(j.principles.first().copied().unwrap_or(Principle::Specific));
        let key = |i: usize| -> i64 {
            let m = &l.rules[fires[i]];
            match principle {
                Principle::Specific => m.specificity as i64,
                Principle::Rank => -(m.auth_rank as i64),
                Principle::Later => m.order as i64,
                Principle::Strict => verdicts[i].severity as i64,
                Principle::Lenient => -(verdicts[i].severity as i64),
            }
        };
        // the best key wins; a tie goes to the earlier rule in the body
        for i in 1..fires.len() {
            if key(i) > key(win) {
                win = i;
            }
        }
        let tag = match pi {
            Some(pi) if a[pi] != l.points[pi].plain => " (tipped)",
            _ => "",
        };
        // a tie the principle cannot break between a guilty and a free verdict: the body's order decides
        // (on the same fact: the two anchors name the same act of the case)
        let fact_of = |k: usize| l.rules[fires[k]].anchor.split(" ~ ").next().unwrap_or("").to_string();
        if let Some(t) = (0..fires.len()).find(|&i| i != win && key(i) == key(win) && fact_of(i) == fact_of(win) && (verdicts[i].severity >= 2) != (verdicts[win].severity >= 2)) {
            why.push(format!("no principle decides between {} and {} under «{}»: the earlier in the body stands", l.rules[fires[win]].rule.id, l.rules[fires[t]].rule.id, principle.name()));
        }
        why.push(format!("{} rules fire with different verdicts; by «{}»{tag} {} prevails", fires.len(), principle.name(), l.rules[fires[win]].rule.id));
    }
    let mut v = verdicts[win].clone();
    if let Some(pi) = l.points.iter().position(|p| p.kind == PointKind::Suspend) {
        let max = j.suspend.as_ref().map(|s| s.0).unwrap_or(0);
        if a[pi] == 1 && v.severity >= 2 && v.severity <= max {
            why.push(format!("the sentence is suspended (tipped): {}", l.points[pi].desc));
            v = Verdict { severity: 1, label: format!("{} — suspended", v.label) };
        }
    }
    why.push(format!("verdict: {} (severity {})", v.label, v.severity));
    (v, Some(fires[win]), why)
}

/// The honest path: the plain choice at every point.
pub fn honest(l: &Lattice) -> Vec<usize> {
    l.points.iter().map(|p| p.plain).collect()
}

/// Count every path exactly. A rule fires only when every one of its points holds (one assignment of its 2^k);
/// so the paths are counted by which rules fire (2^rules patterns), times the global points enumerated in full.
fn count(j: &Jurisdiction, l: &mut Lattice) {
    let n = l.rules.len();
    let local: Vec<Vec<usize>> = (0..n).map(|k| l.points.iter().enumerate().filter(|(_, p)| p.rule == Some(k)).map(|(i, _)| i).collect()).collect();
    let global: Vec<usize> = l.points.iter().enumerate().filter(|(_, p)| p.rule.is_none()).map(|(i, _)| i).collect();
    let mut counts: BTreeMap<Verdict, (u128, Vec<usize>)> = BTreeMap::new();
    let mut total: u128 = 0;
    for pattern in 0u64..(1u64 << n) {
        let mut weight: u128 = 1;
        let mut a: Vec<usize> = l.points.iter().map(|p| p.holds).collect();
        let mut possible = true;
        for k in 0..n {
            let fire = pattern >> k & 1 == 1;
            let kk = local[k].len() as u32;
            if fire {
                if l.rules[k].blocked.is_some() {
                    possible = false;
                    break;
                }
            } else if l.rules[k].blocked.is_some() {
                weight *= 1u128 << kk;
            } else if kk == 0 {
                possible = false;
                break;
            } else {
                weight *= (1u128 << kk) - 1;
                // the representative: the first point not holding
                let p0 = local[k][0];
                a[p0] = 1 - l.points[p0].holds;
            }
        }
        if !possible {
            continue;
        }
        // every global combination
        let sizes: Vec<usize> = global.iter().map(|&g| l.points[g].options.len()).collect();
        let combos: usize = sizes.iter().product();
        for mut c in 0..combos {
            for (gi, &g) in global.iter().enumerate() {
                a[g] = c % sizes[gi];
                c /= sizes[gi];
            }
            let (v, _, _) = evaluate(j, l, &a);
            let e = counts.entry(v).or_insert((0, a.clone()));
            e.0 += weight;
            total += weight;
        }
    }
    l.counts = counts;
    l.total = total;
}

/// The admissible choices of law for a case: (jurisdiction index, principle).
pub fn choice_of_law(js: &[Jurisdiction], case: &Case) -> Vec<(usize, &'static str)> {
    let mut out = Vec::new();
    for (i, j) in js.iter().enumerate() {
        if j.rules.is_empty() {
            continue;
        }
        if case.places.iter().any(|p| j.places.contains(p)) {
            out.push((i, "lex loci (the place of the act)"));
        }
        if case.persons.iter().any(|p| j.persons.contains(p)) {
            out.push((i, "personal law (the parties' status)"));
        }
        out.push((i, "lex fori (the forum where it is brought)"));
    }
    out
}

/// One way to a verdict: the points tipped away from the honest path (point, option) and the rule that decides.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tip {
    pub points: Vec<(usize, usize)>,
    pub winner: Option<usize>,
}

/// Goal-directed search over the lattice: from the honest path, every set of up to `depth` tipped points, and for
/// each verdict reached the `keep` cheapest ways (fewest points, then the earliest points). The honest verdict is
/// reached with none.
pub fn tipping(j: &Jurisdiction, l: &Lattice, depth: usize, keep: usize) -> BTreeMap<Verdict, Vec<Tip>> {
    let base = honest(l);
    let mut out: BTreeMap<Verdict, Vec<Tip>> = BTreeMap::new();
    let (v0, w0, _) = evaluate(j, l, &base);
    out.insert(v0, vec![Tip { points: Vec::new(), winner: w0 }]);
    let alts: Vec<(usize, usize)> = l.points.iter().enumerate().flat_map(|(i, p)| (0..p.options.len()).filter(move |&o| o != p.plain).map(move |o| (i, o))).collect();
    // breadth first: all sets of size d before d + 1, each in a fixed order, so the first found are the cheapest
    let mut level: Vec<Vec<usize>> = vec![Vec::new()];
    for _ in 1..=depth {
        let mut next: Vec<Vec<usize>> = Vec::new();
        for set in &level {
            let from = set.last().map(|x| x + 1).unwrap_or(0);
            for x in from..alts.len() {
                if set.iter().any(|&y| alts[y].0 == alts[x].0) {
                    continue;
                }
                let mut s2 = set.clone();
                s2.push(x);
                let mut a = base.clone();
                for &y in &s2 {
                    a[alts[y].0] = alts[y].1;
                }
                let (v, w, _) = evaluate(j, l, &a);
                let e = out.entry(v).or_default();
                if e.len() < keep {
                    e.push(Tip { points: s2.iter().map(|&y| alts[y]).collect(), winner: w });
                }
                next.push(s2);
            }
        }
        level = next;
        if level.len() > 200_000 {
            break;
        }
    }
    out
}

/// The fewest points to each verdict.
pub fn mice(tips: &BTreeMap<Verdict, Vec<Tip>>) -> BTreeMap<Verdict, usize> {
    tips.iter().map(|(v, t)| (v.clone(), t.first().map(|t| t.points.len()).unwrap_or(usize::MAX))).collect()
}

/// Corruption sensitivity: the share of the lattice's other verdicts a judge reaches by tipping one point (0 when
/// the honest verdict is the only one, or there is no point to tip).
pub fn sensitivity(l: &Lattice, tips: &BTreeMap<Verdict, Vec<Tip>>) -> f64 {
    let m = mice(tips);
    let others: Vec<&Verdict> = l.counts.keys().filter(|v| m.get(*v).is_none_or(|&n| n != 0)).collect();
    if others.is_empty() || l.points.is_empty() {
        return 0.0;
    }
    let one = others.iter().filter(|v| m.get(**v).is_some_and(|&n| n == 1)).count();
    one as f64 / others.len() as f64
}

/// What a tipped point asks to be established or argued.
pub fn to_establish(l: &Lattice, p: usize, o: usize) -> String {
    let pt = &l.points[p];
    let opt = &pt.options[o];
    match pt.kind {
        PointKind::Condition if opt == "met" => format!("prove: {}", pt.desc.split(':').next().unwrap_or(&pt.desc)),
        PointKind::Condition => format!("leave unproven: {}", pt.desc.split(':').next().unwrap_or(&pt.desc)),
        PointKind::Exception if opt == "applies" => format!("prove the exception: {}", pt.desc.split(':').next().unwrap_or(&pt.desc)),
        PointKind::Exception => format!("disprove the exception: {}", pt.desc.split(':').next().unwrap_or(&pt.desc)),
        PointKind::Reading => format!("argue the {opt} reading: {}", pt.desc),
        PointKind::Dispute => format!("follow {opt} in {}", pt.desc),
        PointKind::Precedence => format!("settle the conflict by «{opt}»"),
        PointKind::Suspend => "ask for a suspended sentence".to_string(),
    }
}

pub fn show_tip(l: &Lattice, tip: &[(usize, usize)]) -> String {
    let mut s = String::new();
    for (k, &(p, o)) in tip.iter().enumerate() {
        let pt = &l.points[p];
        let rule = pt.rule.map(|r| format!(" of {} [{}]", l.rules[r].rule.id, l.rules[r].rule.source)).unwrap_or_default();
        if k > 0 {
            s += "; ";
        }
        let _ = write!(s, "{} «{}»{rule}: {} → {}", pt.kind.name(), pt.desc, pt.options[pt.plain], pt.options[o]);
    }
    s
}

/// Every word of a case: verbs, agents, patients, the values of the other roles, the parties' words.
pub fn case_words(c: &Case) -> Vec<String> {
    let mut w: Vec<String> = Vec::new();
    for f in &c.facts {
        w.push(f.verb.clone());
        w.extend(f.agent.iter().cloned());
        w.extend(f.patient.iter().cloned());
        w.extend(f.args.iter().map(|a| val(a).to_string()));
    }
    w.extend(c.persons.iter().cloned());
    w.extend(c.places.iter().cloned());
    w.sort();
    w.dedup();
    w
}

/// The case as a device reads it and as the substance is. A device of a place, a thing or a transaction renames
/// the substance's words into its form (the street becomes a courtyard); the case as read is the substance. A
/// device of an agent is already in the reading (the gentile lit the fire); the substance gives the act back to the
/// one it was done for.
pub fn device_views(j: &Jurisdiction, case: &Case) -> Vec<(crate::juris::Device, Case, Case)> {
    let words = case_words(case);
    let mut out = Vec::new();
    if !j.admits_devices() {
        return out;
    }
    for d in &j.devices {
        if d.trigger.is_empty() || !d.trigger.iter().all(|t| words.contains(t)) {
            continue;
        }
        let rename = |c: &Case, from: &[String], to: &str, agent: bool| -> Case {
            let mut c = c.clone();
            let swap = |w: &mut String| {
                if from.contains(w) {
                    *w = to.to_string();
                }
            };
            for f in &mut c.facts {
                if agent {
                    if let Some(a) = f.agent.as_mut() {
                        swap(a);
                    }
                    continue;
                }
                swap(&mut f.verb);
                if let Some(p) = f.patient.as_mut() {
                    swap(p);
                }
                for a in &mut f.args {
                    let (k, v) = a.rsplit_once(':').map(|(k, v)| (k.to_string(), v.to_string())).unwrap_or((String::new(), a.clone()));
                    if from.contains(&v) {
                        *a = if k.is_empty() { to.to_string() } else { format!("{k}:{to}") };
                    }
                }
            }
            c
        };
        // the form keeps the first form word for each substance word in order (interest → profit, loan → investment)
        let (with, without) = if d.field == "agent" {
            (case.clone(), rename(case, &d.form, d.substance.first().map(String::as_str).unwrap_or(""), true))
        } else {
            let mut c = case.clone();
            for (k, s) in d.substance.iter().enumerate() {
                let to = d.form.get(k).or(d.form.last()).cloned().unwrap_or_default();
                c = rename(&c, std::slice::from_ref(s), &to, false);
            }
            (c, case.clone())
        };
        out.push((d.clone(), with, without));
    }
    out
}
