//! The court's report on a case. Never a single number: the report is layered.
//! (a) the forum: the admissible space under the law of the chosen jurisdiction;
//! (b) the modern reference: the distribution of honest verdicts across modern systems, weights printed;
//! (c) the historical alternatives: counterfactual judges and a source of arguments, weighted apart, never mixed in;
//! (d) slices by principle — proportionality, harm, intent, restitution, rehabilitation — views of the same lattice;
//! (e) the strictest and the most lenient plausible positions;
//! (f) the facts that flip the conclusion;
//! (g) the value-choice points the law does not decide.
//! Then the symmetric goals (convict, acquit, reduce, reclassify, elsewhere), the corrupt judge's counterfactual, and,
//! for a given verdict, where it is atypical and what one must accept to reach it.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::court::{self, Case, Lattice, PointKind, Tip, Verdict};
use crate::juris::Jurisdiction;

pub const PRINCIPLE: &str = "The machine does not sentence. It makes arbitrary sentencing harder to hide.";

/// The shared scale of penalties (`_common/penalties.tsv`).
pub const CLASSES: [&str; 8] = ["none", "restitution or a duty", "fine or an unstated sanction", "fine as a multiple of the harm", "corporal", "liberty", "exclusion", "capital"];

#[derive(Clone, Debug)]
pub struct Opts {
    /// all, or one section: compare, forum, modern, history, slices, bounds, flips, values, goals, corrupt, outlier,
    /// fiction;
    /// detail lenses: paths, honest (majority), strict, lenient, loopholes
    pub lens: String,
    pub max: usize,
    /// points tipped at most in the goal search
    pub depth: usize,
    pub only: Vec<String>,
    /// the forum of layer (a); by default the jurisdictions the place of the act points to
    pub forum: Option<String>,
    /// a court's verdict on the shared scale, to be placed against the reference
    pub given: Option<u8>,
    /// include flagged rules in the reference
    pub flagged: bool,
    /// weights per jurisdiction in the references (1 by default)
    pub weights: BTreeMap<String, f64>,
}

impl Default for Opts {
    fn default() -> Opts {
        Opts { lens: "all".into(), max: 5, depth: 3, only: Vec::new(), forum: None, given: None, flagged: false, weights: BTreeMap::new() }
    }
}

impl Opts {
    pub fn weight(&self, id: &str) -> f64 {
        self.weights.get(id).copied().unwrap_or(1.0)
    }
    /// The options as one canonical line (for the protocol).
    pub fn canon(&self) -> String {
        format!(
            "lens={} max={} depth={} only={} forum={} given={} flagged={} weights={}",
            self.lens,
            self.max,
            self.depth,
            self.only.join(","),
            self.forum.as_deref().unwrap_or("-"),
            self.given.map(|g| g.to_string()).unwrap_or_else(|| "-".into()),
            self.flagged,
            self.weights.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(",")
        )
    }
}

/// One jurisdiction's lattice for one defendant, with its search.
pub struct Judged {
    pub j: Jurisdiction,
    pub law: Vec<&'static str>,
    pub l: Lattice,
    pub honest: (Verdict, Option<usize>, Vec<String>),
    pub tips: BTreeMap<Verdict, Vec<Tip>>,
}

/// Judge in every admissible jurisdiction. `clean`: without the flagged rules.
pub fn judge_all(js: &[Jurisdiction], case: &Case, defendant: &str, opts: &Opts, clean: bool) -> Vec<Judged> {
    let mut by: BTreeMap<usize, Vec<&'static str>> = BTreeMap::new();
    for (i, why) in court::choice_of_law(js, case) {
        by.entry(i).or_default().push(why);
    }
    let mut out = Vec::new();
    for (i, law) in by {
        if !opts.only.is_empty() && !opts.only.contains(&js[i].id) {
            continue;
        }
        let j = if clean { js[i].unflagged() } else { js[i].clone() };
        let l = court::lattice(&j, case, defendant);
        let honest = court::evaluate(&j, &l, &court::honest(&l));
        let tips = court::tipping(&j, &l, opts.depth, opts.max.max(1));
        out.push(Judged { j, law, l, honest, tips });
    }
    out
}

fn short(s: &str, n: usize) -> String {
    if s.chars().count() <= n { s.to_string() } else { format!("{}…", s.chars().take(n - 1).collect::<String>()) }
}

fn rule_ref(g: &Judged, k: Option<usize>) -> String {
    k.map(|k| format!("{} [{}]", g.l.rules[k].rule.id, g.l.rules[k].rule.source)).unwrap_or_else(|| "—".into())
}

fn class(sev: u8) -> String {
    format!("{sev} {}", CLASSES.get(sev as usize).unwrap_or(&"?"))
}

/// The goals of the symmetric report.
pub const GOALS: [&str; 4] = ["convict", "acquit", "reduce", "reclassify"];

fn act_of(g: &Judged, k: Option<usize>) -> Option<(String, crate::rule::Force)> {
    k.map(|k| (g.l.rules[k].rule.act_verb().unwrap_or("").to_string(), g.l.rules[k].rule.force))
}

/// Does a verdict reached by `tip` serve the goal, against the honest verdict of its jurisdiction?
pub fn serves(goal: &str, g: &Judged, v: &Verdict, tip: &Tip) -> bool {
    let h = &g.honest.0;
    match goal {
        "convict" => v.severity >= 2,
        "acquit" => v.severity == 0,
        "reduce" => v.severity > 0 && v.severity < h.severity,
        "reclassify" => v != h && act_of(g, tip.winner) != act_of(g, g.honest.1),
        _ => false,
    }
}

/// Rows for a goal across the jurisdictions: (points, jurisdiction, verdict, tip), cheapest first.
pub fn goal_rows<'b>(goal: &str, gs: &'b [Judged]) -> Vec<(usize, usize, &'b Verdict, &'b Tip)> {
    let mut rows = Vec::new();
    for (gi, g) in gs.iter().enumerate() {
        for (v, tips) in &g.tips {
            for t in tips {
                if serves(goal, g, v, t) {
                    rows.push((t.points.len(), gi, v, t));
                }
            }
        }
    }
    rows.sort_by(|a, b| {
        let sev = |v: &Verdict| if goal == "convict" { -(v.severity as i32) } else { v.severity as i32 };
        a.0.cmp(&b.0).then(sev(a.2).cmp(&sev(b.2))).then(a.1.cmp(&b.1)).then(a.2.cmp(b.2))
    });
    rows
}

/// Weighted statistics of severities: (modal class, median, p25, p75, min, max, total weight).
pub fn stats(xs: &[(u8, f64)]) -> Option<(u8, u8, u8, u8, u8, u8, f64)> {
    let total: f64 = xs.iter().map(|x| x.1).sum();
    if xs.is_empty() || total <= 0.0 {
        return None;
    }
    let mut by: BTreeMap<u8, f64> = BTreeMap::new();
    for &(s, w) in xs {
        *by.entry(s).or_default() += w;
    }
    // the modal class: the heaviest; a tie goes to the milder
    let modal = by.iter().fold((0u8, -1.0f64), |acc, (&s, &w)| if w > acc.1 { (s, w) } else { acc }).0;
    let q = |p: f64| -> u8 {
        let mut c = 0.0;
        for (&s, &w) in &by {
            c += w;
            if c >= p * total - 1e-9 {
                return s;
            }
        }
        *by.keys().last().unwrap()
    };
    Some((modal, q(0.5), q(0.25), q(0.75), *by.keys().next().unwrap(), *by.keys().last().unwrap(), total))
}

/// The weighted percentile of a severity among the others: those below, plus half of those equal.
pub fn percentile(xs: &[(u8, f64)], s: u8) -> f64 {
    let total: f64 = xs.iter().map(|x| x.1).sum();
    if total <= 0.0 {
        return 0.0;
    }
    let below: f64 = xs.iter().filter(|x| x.0 < s).map(|x| x.1).sum();
    let eq: f64 = xs.iter().filter(|x| x.0 == s).map(|x| x.1).sum();
    100.0 * (below + eq / 2.0) / total
}

const HARM_DEATH: [&str; 5] = ["die", "kill", "slay", "murder", "perish"];
const HARM_BODY: [&str; 7] = ["smite", "strike", "wound", "gore", "hurt", "beat", "injure"];
const HARM_PROPERTY: [&str; 10] = ["steal", "take", "rob", "burn", "damage", "destroy", "break", "sell", "keep", "eat"];
const INTENT: [&str; 14] = ["intent", "intentionally", "wilfully", "willfully", "malice", "wait", "presumptuously", "knowingly", "negligence", "unwittingly", "unawares", "error", "deliberately", "accident"];

/// The harm the case tells of, on the shared scale (for proportionality): a death 7, a body hurt 4, property 3 (the
/// classical proportionate answer to a loss of property is to pay it back several times), else 1.
pub fn harm_of(case: &Case) -> u8 {
    let vs: Vec<&str> = case.facts.iter().filter(|f| !f.neg).map(|f| f.verb.as_str()).collect();
    if vs.iter().any(|v| HARM_DEATH.contains(v)) {
        7
    } else if vs.iter().any(|v| HARM_BODY.contains(v)) {
        4
    } else if vs.iter().any(|v| HARM_PROPERTY.contains(v)) {
        3
    } else {
        1
    }
}

fn rule_words(g: &Judged, k: Option<usize>) -> String {
    k.map(|k| format!(" {} ", g.l.rules[k].rule.text.to_lowercase())).unwrap_or_default()
}

/// The slices: each picks, in each jurisdiction, the plausible verdict (reachable within the depth) that best fits
/// one principle; a tie goes to fewer tipped points, then the milder verdict.
pub fn slice<'a>(name: &str, g: &'a Judged, harm: u8) -> Option<(&'a Verdict, &'a Tip)> {
    let mut best: Option<(i64, usize, u8, &'a Verdict, &'a Tip)> = None;
    for (v, tips) in &g.tips {
        for t in tips {
            let text = rule_words(g, t.winner);
            let score: Option<i64> = match name {
                "proportionality" => Some(-((v.severity as i64 - harm as i64).abs())),
                "harm" => t.winner.and_then(|k| {
                    let r = &g.l.rules[k].rule;
                    let hit = r.conds.iter().flatten().chain(r.acts.iter()).any(|c| HARM_DEATH.contains(&c.verb.as_str()) || HARM_BODY.contains(&c.verb.as_str()) || matches!(c.verb.as_str(), "damage" | "burn" | "destroy" | "lose" | "ruin"));
                    hit.then_some(v.severity as i64)
                }),
                "intent" => INTENT.iter().any(|w| text.contains(&format!(" {w}"))).then_some(0),
                "restitution" => (1..=3).contains(&v.severity).then_some(-(v.severity as i64)),
                "rehabilitation" => (v.label.ends_with("suspended") || (v.severity <= 2 && t.winner.is_some())).then_some(if v.label.ends_with("suspended") { 1 } else { 0 }),
                _ => None,
            };
            if let Some(sc) = score {
                let key = (sc, t.points.len(), v.severity);
                if best.as_ref().is_none_or(|b| key.0 > b.0 || (key.0 == b.0 && (key.1, key.2) < (b.1, b.2))) {
                    best = Some((key.0, key.1, key.2, v, t));
                }
            }
        }
    }
    best.map(|b| (b.3, b.4))
}

pub const SLICES: [&str; 5] = ["proportionality", "harm", "intent", "restitution", "rehabilitation"];

fn reference_block(o: &mut String, title: &str, gs: &[Judged], opts: &Opts, all: &[Jurisdiction]) -> Vec<(u8, f64)> {
    let _ = writeln!(o, "{title}");
    let mut xs: Vec<(u8, f64)> = Vec::new();
    if gs.is_empty() {
        let _ = writeln!(o, "  no jurisdiction of this kind has rules yet");
        return xs;
    }
    let _ = writeln!(o, "  {:<12} {:>6}  {:<34} {:<34} {:<34} {:<7} {}", "jurisdiction", "weight", "honest verdict", "class", "decisive rule", "source", "flagged rules left out");
    for g in gs {
        let w = opts.weight(&g.j.id);
        let full = all.iter().find(|j| j.id == g.j.id);
        let left_out = full.map(|f| f.rules.len() - g.j.rules.len()).unwrap_or(0);
        let _ = writeln!(o, "  {:<12} {:>6.2}  {:<34} {:<34} {:<34} {:<7} {}", g.j.id, w, short(&g.honest.0.label, 34), class(g.honest.0.severity), short(&rule_ref(g, g.honest.1), 34), g.j.provenance().0, if opts.flagged { "none (--flagged)".to_string() } else { left_out.to_string() });
        if w > 0.0 {
            xs.push((g.honest.0.severity, w));
        }
    }
    match stats(&xs) {
        Some((modal, med, p25, p75, min, max, total)) => {
            let _ = writeln!(o, "  modal class {}; median {}; spread {}–{} (quartiles {}–{}); total weight {total:.2}", class(modal), class(med), min, max, p25, p75);
        }
        None => {
            let _ = writeln!(o, "  no weight");
        }
    }
    xs
}

pub fn report(js: &[Jurisdiction], case: &Case, opts: &Opts) -> String {
    let mut o = String::new();
    let lens = opts.lens.as_str();
    let want = |x: &str| {
        let detail = matches!(x, "paths" | "honest" | "strict" | "lenient" | "loopholes");
        (lens == "all" && !detail) || lens == x || (lens == "bribed" && x == "corrupt") || (lens == "majority" && x == "honest") || (lens == "detail" && detail) || (x == "outlier" && opts.given.is_some() && lens == "all")
    };
    let _ = writeln!(o, "Yom HaDin — {PRINCIPLE}\nA study of reasoning, not legal advice; it judges no one.\n");
    let _ = writeln!(o, "Case: {}", case.text);
    let _ = writeln!(o, "Facts (frames):");
    for (i, f) in case.facts.iter().enumerate() {
        let _ = writeln!(o, "  #{i} {}", f.show());
    }
    let harm = harm_of(case);
    let _ = writeln!(o, "Places named: {}; parties' words: {}; harm told: {}", if case.places.is_empty() { "none".into() } else { case.places.join(", ") }, case.persons.join(", "), class(harm));
    for d in court::defendants(case) {
        let gs = judge_all(js, case, &d, opts, false);
        let clean = if opts.flagged { judge_all(js, case, &d, opts, false) } else { judge_all(js, case, &d, opts, true) };
        let _ = writeln!(o, "\n=== Defendant: {d}");
        if gs.is_empty() {
            let _ = writeln!(o, "no jurisdiction with rules");
            continue;
        }
        let total: u128 = gs.iter().map(|g| g.l.total * g.law.len() as u128).sum();
        let _ = writeln!(o, "Lattice: {total} paths over {} jurisdictions (each jurisdiction's paths × its admissible choices of law)", gs.len());
        if want("compare") {
            let _ = writeln!(o, "\n-- Side by side (law: loci/pers/fori; honest verdict; the decisive rule; max and min severity in the lattice; corruption sensitivity; mice to acquittal; ⚑ flagged rules among the matched)");
            let _ = writeln!(o, "{:<12} {:<4} {:<9} {:>5} {:>6} {:>9}  {:<40} {:<34} {:>3} {:>3} {:>5} {:>3} {:>2}", "jurisdiction", "kind", "law", "rules", "points", "paths", "honest verdict", "decisive rule", "max", "min", "sens", "🐭0", "⚑");
            for g in &gs {
                let lawtag = g.law.iter().map(|w| if w.contains("loci") { "loci" } else if w.contains("personal") { "pers" } else { "fori" }).collect::<Vec<_>>().join("+");
                let max = g.l.counts.keys().map(|v| v.severity).max().unwrap_or(0);
                let min = g.l.counts.keys().map(|v| v.severity).min().unwrap_or(0);
                let m = court::mice(&g.tips);
                let to_acq = m.iter().filter(|(v, _)| v.acquittal()).map(|(_, &n)| n).min();
                let flagged = g.l.rules.iter().filter(|m| !g.j.flags_of(&m.rule).is_empty()).count();
                let _ = writeln!(
                    o,
                    "{:<12} {:<4} {:<9} {:>5} {:>6} {:>9}  {:<40} {:<34} {:>3} {:>3} {:>5.2} {:>3} {:>2}",
                    g.j.id,
                    if g.j.kind() == "modern" { "mod" } else { "hist" },
                    lawtag,
                    g.l.rules.len(),
                    g.l.points.len(),
                    g.l.total,
                    short(&format!("{} [{}]", g.honest.0.label, g.honest.0.severity), 40),
                    short(&rule_ref(g, g.honest.1), 34),
                    max,
                    min,
                    court::sensitivity(&g.l, &g.tips),
                    to_acq.map(|n| n.to_string()).unwrap_or_else(|| format!(">{}", opts.depth)),
                    flagged
                );
            }
        }
        // (a) the forum
        let forum: Vec<&Judged> = match &opts.forum {
            Some(f) => gs.iter().filter(|g| &g.j.id == f).collect(),
            None => gs.iter().filter(|g| g.law.iter().any(|w| w.contains("loci"))).collect(),
        };
        if want("forum") {
            let _ = writeln!(o, "\n-- (a) The forum: the admissible space under the law in force there");
            if forum.is_empty() {
                let _ = writeln!(o, "  no forum: the case names no place a jurisdiction claims; choose one with --forum <id>");
            }
            for g in &forum {
                let _ = writeln!(o, "  {} — {} ({}; {}): {} rules, {} choice points, {} paths", g.j.id, g.j.name(), g.j.kind(), g.j.status(), g.l.rules.len(), g.l.points.len(), g.l.total);
                let mut vs: Vec<(&Verdict, &(u128, Vec<usize>))> = g.l.counts.iter().collect();
                vs.sort_by(|a, b| b.0.severity.cmp(&a.0.severity).then(a.0.cmp(b.0)));
                for (v, (n, _)) in vs {
                    let _ = writeln!(o, "    {n:>8} paths  {} [{}]{}", v.label, class(v.severity), if *v == g.honest.0 { "  ← honest" } else { "" });
                }
                for w in &g.honest.2 {
                    let _ = writeln!(o, "    · {w}");
                }
            }
        }
        // (b), (c) the references
        let modern: Vec<Judged> = clean.iter().filter(|g| g.j.kind() == "modern").map(|g| Judged { j: g.j.clone(), law: g.law.clone(), l: g.l.clone(), honest: g.honest.clone(), tips: g.tips.clone() }).collect();
        let hist: Vec<Judged> = clean.iter().filter(|g| g.j.kind() != "modern").map(|g| Judged { j: g.j.clone(), law: g.law.clone(), l: g.l.clone(), honest: g.honest.clone(), tips: g.tips.clone() }).collect();
        let mut modern_xs = Vec::new();
        if want("modern") {
            let _ = writeln!(o, "\n-- (b) The modern reference: honest verdicts across modern systems{} (a second opinion, not a court)", if opts.flagged { ", flagged rules included" } else { ", flagged rules left out" });
            modern_xs = reference_block(&mut o, "", &modern, opts, js);
        } else {
            modern_xs = modern.iter().filter(|g| opts.weight(&g.j.id) > 0.0).map(|g| (g.honest.0.severity, opts.weight(&g.j.id))).collect();
        }
        if want("history") {
            let _ = writeln!(o, "\n-- (c) Historical and religious traditions: alternatives and a source of arguments, weighted apart and never mixed into (b)");
            reference_block(&mut o, "", &hist, opts, js);
        }
        if want("slices") {
            let _ = writeln!(o, "\n-- (d) Slices by principle (each a view of the same lattices; plausible = within {} tipped points)", opts.depth);
            for name in SLICES {
                let _ = writeln!(o, "  [{name}]{}", if name == "proportionality" { format!(" the verdict nearest the harm told ({})", class(harm)) } else { String::new() });
                for g in &gs {
                    match slice(name, g, harm) {
                        Some((v, t)) => {
                            let _ = writeln!(o, "    {:<12} {} [{}] — {}{}", g.j.id, v.label, v.severity, rule_ref(g, t.winner), if t.points.is_empty() { " (honest path)".to_string() } else { format!("; {}", t.points.iter().map(|&(p, x)| court::to_establish(&g.l, p, x)).collect::<Vec<_>>().join("; ")) });
                        }
                        None => {
                            let _ = writeln!(o, "    {:<12} no plausible path in this slice", g.j.id);
                        }
                    }
                }
            }
        }
        if want("bounds") {
            let _ = writeln!(o, "\n-- (e) The strictest and the most lenient plausible positions (within {} tipped points)", opts.depth);
            for (name, layer) in [("modern", gs.iter().filter(|g| g.j.kind() == "modern").collect::<Vec<_>>()), ("historical", gs.iter().filter(|g| g.j.kind() != "modern").collect::<Vec<_>>())] {
                let mut all: Vec<(u8, usize, &Judged, &Verdict, &Tip)> = Vec::new();
                for g in &layer {
                    for (v, ts) in &g.tips {
                        if let Some(t) = ts.first() {
                            all.push((v.severity, t.points.len(), g, v, t));
                        }
                    }
                }
                let hi = all.iter().max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)).then(b.2.j.id.cmp(&a.2.j.id)));
                let lo = all.iter().min_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.j.id.cmp(&b.2.j.id)));
                for (tag, x) in [("strictest", hi), ("most lenient", lo)] {
                    if let Some((_, n, g, v, t)) = x {
                        let _ = writeln!(o, "  {name} {tag}: {} — {} [{}], {} points{}", g.j.id, v.label, v.severity, n, if t.points.is_empty() { String::new() } else { format!(": {}", court::show_tip(&g.l, &t.points)) });
                    }
                }
            }
        }
        if want("flips") {
            let _ = writeln!(o, "\n-- (f) The facts that flip the conclusion (one fact established or left unproven)");
            let mut any = false;
            for g in &gs {
                for (v, ts) in &g.tips {
                    for t in ts.iter().filter(|t| t.points.len() == 1) {
                        let (p, x) = t.points[0];
                        if matches!(g.l.points[p].kind, PointKind::Condition | PointKind::Exception) {
                            let _ = writeln!(o, "  {:<12} {} → {} [{}]", g.j.id, court::to_establish(&g.l, p, x), v.label, v.severity);
                            any = true;
                        }
                    }
                }
            }
            if !any {
                let _ = writeln!(o, "  none: no single fact changes an honest verdict");
            }
        }
        if want("values") {
            let _ = writeln!(o, "\n-- (g) The value-choice points: where a judge must choose and the law does not decide");
            let mut any = false;
            let mut quiet: BTreeMap<String, usize> = BTreeMap::new();
            for g in &gs {
                let base = court::honest(&g.l);
                for (pi, p) in g.l.points.iter().enumerate() {
                    if !matches!(p.kind, PointKind::Reading | PointKind::Dispute | PointKind::Precedence | PointKind::Suspend) {
                        continue;
                    }
                    let mut outs: Vec<String> = Vec::new();
                    let mut seen: Vec<Verdict> = Vec::new();
                    for x in 0..p.options.len() {
                        let mut a = base.clone();
                        a[pi] = x;
                        let (v, _, _) = court::evaluate(&g.j, &g.l, &a);
                        outs.push(format!("{}{} → {} [{}]", p.options[x], if x == p.plain { " (plain)" } else { "" }, v.label, v.severity));
                        if !seen.contains(&v) {
                            seen.push(v);
                        }
                    }
                    // only the points that decide alone; the others are counted
                    if seen.len() < 2 {
                        *quiet.entry(g.j.id.clone()).or_default() += 1;
                        continue;
                    }
                    any = true;
                    let rule = p.rule.map(|r| format!(" of {}", g.l.rules[r].rule.id)).unwrap_or_default();
                    let _ = writeln!(o, "  {:<12} {} «{}»{rule}: {}", g.j.id, p.kind.name(), p.desc, outs.join("; "));
                }
            }
            if !any {
                let _ = writeln!(o, "  none decides alone");
            }
            if !quiet.is_empty() {
                let _ = writeln!(o, "  points that change nothing alone (they count in combination): {}", quiet.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
            }
        }
        for g in &gs {
            if !(want("paths") || want("honest") || want("strict") || want("lenient") || want("loopholes")) {
                break;
            }
            let _ = writeln!(o, "\n-- {} — {} ({}): {} rules matched{}, {} choice points, {} paths; law by {}", g.j.id, g.j.name(), g.j.status(), g.l.rules.len(), if g.l.capped > 0 { format!(" ({} more left out by the cap)", g.l.capped) } else { String::new() }, g.l.points.len(), g.l.total, g.law.join(", "));
            for m in &g.l.rules {
                let f = g.j.flags_of(&m.rule);
                if !f.is_empty() {
                    let _ = writeln!(o, "  ⚑ {}: {}", m.rule.id, f.join("; "));
                }
            }
            if want("paths") {
                let mut vs: Vec<(&Verdict, &(u128, Vec<usize>))> = g.l.counts.iter().collect();
                vs.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
                for (v, (n, a)) in vs.iter().take(opts.max * 2) {
                    let _ = writeln!(o, "  {n:>8}  {} [severity {}] — first path:", v.label, v.severity);
                    for w in court::evaluate(&g.j, &g.l, a).2 {
                        let _ = writeln!(o, "            · {w}");
                    }
                }
            }
            if want("honest") {
                let _ = writeln!(o, "honest path:");
                for w in &g.honest.2 {
                    let _ = writeln!(o, "  · {w}");
                }
            }
            for (name, pick) in [("strict", true), ("lenient", false)] {
                if !want(name) {
                    continue;
                }
                let v = if pick { g.l.counts.iter().max_by_key(|(v, _)| (v.severity, std::cmp::Reverse((*v).clone()))) } else { g.l.counts.iter().min_by_key(|(v, _)| (v.severity, (*v).clone())) };
                if let Some((v, (n, a))) = v {
                    let _ = writeln!(o, "{}: {} [severity {}], {n} paths; the first:", if pick { "strictest (convict by the harshest reading, forgetting the others)" } else { "most lenient" }, v.label, v.severity);
                    for w in court::evaluate(&g.j, &g.l, a).2 {
                        let _ = writeln!(o, "  · {w}");
                    }
                }
            }
            if want("loopholes") {
                let _ = writeln!(o, "loopholes (one point from the honest path, the defendant goes free):");
                let mut found = 0;
                for (v, tips) in &g.tips {
                    if !v.acquittal() || v == &g.honest.0 {
                        continue;
                    }
                    for t in tips.iter().filter(|t| t.points.len() == 1) {
                        let _ = writeln!(o, "  · {} → {}", court::show_tip(&g.l, &t.points), v.label);
                        found += 1;
                    }
                }
                if found == 0 {
                    let _ = writeln!(o, "  none{}", if g.honest.0.acquittal() { ": the honest verdict already frees" } else { "" });
                }
            }
        }
        if want("goals") {
            let _ = writeln!(o, "\n-- Symmetric goals (rows ranked by contested points 🐭; jurisdiction; verdict; the rule it rests on; what must be established)");
            for goal in GOALS {
                let rows = goal_rows(goal, &gs);
                let _ = writeln!(o, "[{goal}]");
                if rows.is_empty() {
                    let hs: Vec<String> = gs.iter().map(|g| format!("{}: {}", g.j.id, g.honest.0.label)).collect();
                    let _ = writeln!(o, "  unreachable within {} points under these rules — the honest verdicts stand ({})", opts.depth, hs.join("; "));
                    continue;
                }
                for (n, gi, v, t) in rows.iter().take(opts.max) {
                    let g = &gs[*gi];
                    let est: Vec<String> = t.points.iter().map(|&(p, x)| court::to_establish(&g.l, p, x)).collect();
                    let _ = writeln!(o, "  {n} {:<6} {:<12} {} — {}{}", "🐭".repeat(*n), g.j.id, v.label, rule_ref(g, t.winner), if est.is_empty() { " (the honest path)".into() } else { format!("; {}", est.join("; ")) });
                }
            }
            let _ = writeln!(o, "[elsewhere] (each jurisdiction's honest verdict, and the choice of law that brings the case there)");
            for g in &gs {
                let _ = writeln!(o, "  {:<12} {} — {} — by {}", g.j.id, g.honest.0.label, rule_ref(g, g.honest.1), g.law.join(", "));
            }
        }
        if want("corrupt") {
            let _ = writeln!(o, "\n-- Corrupt judge counterfactual (how much the outcome rests on an honest procedure; not advice). The bribe is counted in mice 🐭, one per tipped point.");
            for g in &gs {
                let m = court::mice(&g.tips);
                let sens = court::sensitivity(&g.l, &g.tips);
                let _ = writeln!(o, "  {} — sensitivity {sens:.2}: {}", g.j.id, if g.l.points.is_empty() { "no discretionary point, nothing to tip" } else if sens >= 0.5 { "fragile" } else if sens > 0.0 { "pliable" } else { "robust within the search" });
                let targets: Vec<(&str, Option<&Verdict>)> = vec![
                    ("acquittal", m.keys().filter(|v| v.acquittal() && **v != g.honest.0).min_by_key(|v| (m[*v], (*v).clone()))),
                    ("suspended", m.keys().filter(|v| v.label.ends_with("suspended")).min_by_key(|v| (m[*v], (*v).clone()))),
                    ("harshest", m.keys().filter(|v| **v != g.honest.0 && v.severity > g.honest.0.severity).max_by_key(|v| (v.severity, std::cmp::Reverse(m[*v])))),
                ];
                for (name, v) in targets {
                    match v {
                        Some(v) => {
                            let t = &g.tips[v][0];
                            let _ = writeln!(o, "    · {name}: {} — {} {}: {}", v.label, t.points.len(), "🐭".repeat(t.points.len()), court::show_tip(&g.l, &t.points));
                        }
                        None => {
                            let _ = writeln!(o, "    · {name}: unreachable within {} points", opts.depth);
                        }
                    }
                }
                if m.keys().any(|v| v.label.ends_with("suspended")) {
                    let _ = writeln!(o, "    (with mice as a bribe, the snake finds the probation clause and gives a suspended sentence)");
                }
            }
        }
        if want("outlier") {
            if let Some(s) = opts.given {
                let _ = writeln!(o, "\n-- A given verdict: {} — where it stands, what one must accept to reach it, where else the chains lead", class(s));
                if modern_xs.is_empty() {
                    let _ = writeln!(o, "  no modern reference to place it against");
                } else {
                    let p = percentile(&modern_xs, s);
                    let (_, _, p25, p75, min, max, _) = stats(&modern_xs).unwrap();
                    let atyp = s < min || s > max;
                    let _ = writeln!(o, "  against the modern reference: percentile {p:.0}; {}", if atyp { format!("atypical: outside the reference's whole range {min}–{max}") } else if s < p25 || s > p75 { format!("at the edge: outside the quartiles {p25}–{p75}") } else { "within the quartiles".to_string() });
                }
                let targets: Vec<&Judged> = if forum.is_empty() { gs.iter().filter(|g| g.j.kind() == "modern").collect() } else { forum.clone() };
                for g in targets {
                    let reach: Vec<(&Verdict, &Tip)> = g.tips.iter().filter(|(v, _)| v.severity == s).filter_map(|(v, ts)| ts.first().map(|t| (v, t))).collect();
                    match reach.iter().min_by_key(|x| x.1.points.len()) {
                        Some((v, t)) => {
                            let _ = writeln!(o, "  {}: reachable as «{}» with {} contested points; assumptions to accept: {}", g.j.id, v.label, t.points.len(), if t.points.is_empty() { "none (it is the honest verdict)".into() } else { t.points.iter().map(|&(p, x)| court::to_establish(&g.l, p, x)).collect::<Vec<_>>().join("; ") });
                        }
                        None => {
                            let _ = writeln!(o, "  {}: not reachable within {} points — outside the space this law allows here", g.j.id, opts.depth);
                        }
                    }
                    let mut alts: Vec<(usize, &Verdict, &Tip)> = g.tips.iter().filter(|(v, _)| v.severity != s).filter_map(|(v, ts)| ts.first().map(|t| (t.points.len(), v, t))).collect();
                    alts.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(b.1)));
                    for (n, v, t) in alts.iter().take(opts.max) {
                        let _ = writeln!(o, "    elsewhere with {n}: {} [{}] — {}", v.label, v.severity, if t.points.is_empty() { "the honest path".to_string() } else { court::show_tip(&g.l, &t.points) });
                    }
                }
            }
        }
    }
    if lens == "absurd" {
        o += &crate::absurd::lens(js, case, &opts.only, opts.depth, opts.max);
    }
    if lens == "fiction" || lens == "all" {
        fiction(&mut o, js, case, opts, lens == "fiction");
    }
    o
}

/// The fiction lens: the verdicts that rest on a legal device. For each jurisdiction whose device is in the case,
/// the honest path with the device and without it side by side; the steps that differ rest on the device (⟐).
pub fn fiction(o: &mut String, js: &[Jurisdiction], case: &Case, opts: &Opts, always: bool) {
    let mut views: Vec<(&Jurisdiction, crate::juris::Device, Case, Case)> = Vec::new();
    for j in js {
        if !opts.only.is_empty() && !opts.only.contains(&j.id) {
            continue;
        }
        for (d, with, without) in court::device_views(j, case) {
            views.push((j, d, with, without));
        }
    }
    if views.is_empty() {
        if always {
            let _ = writeln!(o, "\n-- Legal fictions: none — no device of any jurisdiction is in this case");
        }
        return;
    }
    let _ = writeln!(o, "\n-- Legal fictions (ha'aramah): verdicts that rest on a formal device; ⟐ marks a step that rests on it");
    for (j, d, with, without) in views {
        let _ = writeln!(o, "  {} — {} ({}; {}): {}", j.id, d.name, d.field, d.source, d.note);
        let mut ds: Vec<String> = court::defendants(&with);
        for x in court::defendants(&without) {
            if !ds.contains(&x) {
                ds.push(x);
            }
        }
        for def in ds {
            let lw = court::lattice(j, &with, &def);
            let lo = court::lattice(j, &without, &def);
            let hw = court::evaluate(j, &lw, &court::honest(&lw));
            let ho = court::evaluate(j, &lo, &court::honest(&lo));
            let _ = writeln!(o, "   defendant {def}: with the device {} [{}] ({} paths); without it {} [{}] ({} paths){}", hw.0.label, hw.0.severity, lw.total, ho.0.label, ho.0.severity, lo.total, if hw.0 == ho.0 || (hw.0.severity == ho.0.severity && hw.1.is_none() && ho.1.is_none()) { " — the device changes nothing here" } else { " — the verdict rests on the device" });
            // a step rests on the device when its rule's standing (fires, does not fire, set aside) changes with it,
            // or when the settling of the conflict or the verdict does
            let key = |w: &str| -> String {
                for st in [": fires", ": does not fire", ": set aside"] {
                    if let Some(i) = w.find(st) {
                        return w[..i + st.len()].to_string();
                    }
                }
                w.to_string()
            };
            let kw: Vec<String> = hw.2.iter().map(|w| key(w)).collect();
            let ko: Vec<String> = ho.2.iter().map(|w| key(w)).collect();
            let _ = writeln!(o, "     with the device:");
            for w in &hw.2 {
                let _ = writeln!(o, "       {} {w}", if ko.contains(&key(w)) { " " } else { "⟐" });
            }
            let _ = writeln!(o, "     without it:");
            for w in &ho.2 {
                let _ = writeln!(o, "       {} {w}", if kw.contains(&key(w)) { " " } else { "·" });
            }
        }
    }
}

/// FNV-1a, 64 bits: the protocol's name and the fingerprint of each rules file.
pub fn fnv(s: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// The protocol of a run: the case, the options and weights, every jurisdiction's rules file fingerprint and
/// provenance, the report, and each lattice's points and counts. No time and no machine path: the same input gives
/// the same protocol, byte for byte.
pub fn protocol(js: &[Jurisdiction], case: &Case, opts: &Opts, rep: &str) -> String {
    let mut o = String::new();
    let _ = writeln!(o, "# Yom HaDin protocol\nversion\t{}\ncase\t{}\noptions\t{}", env!("CARGO_PKG_VERSION"), case.text, opts.canon());
    for j in js {
        let rules = std::fs::read(j.dir.join("rules.tsv")).unwrap_or_default();
        let (pk, pd) = j.provenance();
        let _ = writeln!(o, "jurisdiction\t{}\t{}\t{}\tweight={}\trules={}\tfnv={:016x}\tprovenance={pk}: {pd}", j.id, j.kind(), j.status(), opts.weight(&j.id), j.rules.len(), fnv(&rules));
    }
    let _ = writeln!(o, "\n# report\n{rep}\n# lattices");
    for d in court::defendants(case) {
        for g in judge_all(js, case, &d, opts, false) {
            let _ = writeln!(o, "lattice\t{d}\t{}\trules={}\tpoints={}\tpaths={}", g.j.id, g.l.rules.len(), g.l.points.len(), g.l.total);
            for p in &g.l.points {
                let _ = writeln!(o, "point\t{}\t{}\t{}\tplain={}\t{}", p.kind.name(), p.rule.map(|r| g.l.rules[r].rule.id.clone()).unwrap_or_else(|| "-".into()), p.desc, p.options[p.plain], p.options.join("|"));
            }
            for (v, (n, _)) in &g.l.counts {
                let _ = writeln!(o, "count\t{n}\t{}\t{}", v.severity, v.label);
            }
        }
    }
    o
}
