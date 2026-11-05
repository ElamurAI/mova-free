//! The court's gates, with their negative controls. These tests build jurisdictions and cases by hand, so they need
//! neither the parser model nor the corpora.

use yomhadin::court::{self, Case, Fact};
use yomhadin::juris::{Jurisdiction, Principle, Term};
use yomhadin::report::{self, Opts};
use yomhadin::rule::{Clause, Force, Rule};

fn cl(verb: &str, patient: Option<&str>) -> Clause {
    Clause { verb: verb.into(), neg: false, patient: patient.map(str::to_string), args: Vec::new() }
}

fn rule(id: &str, force: Force, penalty: &str, acts: Vec<Clause>, conds: Vec<Vec<Clause>>) -> Rule {
    Rule { id: id.into(), jur: "test".into(), source: id.into(), sent: id.into(), authority: "Lawgiver".into(), force, penalty: penalty.into(), acts, conds, excs: Vec::new(), parties: Vec::new(), dispute: String::new(), tree: "hand".into(), reason: String::new(), text: format!("rule {id}") }
}

fn jur(rules: Vec<Rule>) -> Jurisdiction {
    let mut j = Jurisdiction::default();
    j.id = "test".into();
    j.principles = vec![Principle::Specific, Principle::Strict, Principle::Lenient];
    j.terms = vec![Term { head: "steal".into(), narrow: vec!["steal".into()], broad: vec!["take".into()], plain_broad: false, opposite: Vec::new(), note: String::new() }];
    j.penalties = vec![yomhadin::juris::Penalty { pattern: "death".into(), rank: 7, gloss: "capital".into() }, yomhadin::juris::Penalty { pattern: "pay".into(), rank: 2, gloss: "payment".into() }];
    j.rules = rules;
    j
}

fn fact(agent: &str, verb: &str, patient: Option<&str>) -> Fact {
    Fact { verb: verb.into(), neg: false, agent: Some(agent.into()), patient: patient.map(str::to_string), args: Vec::new() }
}

fn case(facts: Vec<Fact>) -> Case {
    Case { text: "a case".into(), facts, places: Vec::new(), persons: Vec::new() }
}

#[test]
fn no_rule_applies_is_not_a_verdict() {
    let j = jur(vec![rule("r1", Force::Liability, "pay double", Vec::new(), vec![vec![cl("steal", Some("ox"))]])]);
    let c = case(vec![fact("river", "freeze", None)]);
    let l = court::lattice(&j, &c, "river");
    let (v, w, _) = court::evaluate(&j, &l, &court::honest(&l));
    assert_eq!(v.label, "no rule applies");
    assert!(w.is_none());
    assert_eq!(l.total, 1);
    assert_eq!(l.counts.len(), 1);
    // control: a case the rule covers gives a verdict
    let c2 = case(vec![fact("man", "steal", Some("ox"))]);
    let l2 = court::lattice(&j, &c2, "man");
    assert_eq!(court::evaluate(&j, &l2, &court::honest(&l2)).0.label, "liable: pay double");
}

#[test]
fn removing_a_condition_changes_the_path_counts() {
    let two = vec![vec![cl("steal", Some("ox"))], vec![cl("sell", None)], vec![cl("kill", None)]];
    let one = vec![vec![cl("steal", Some("ox"))], vec![cl("sell", None)]];
    let c = case(vec![fact("man", "steal", Some("ox"))]);
    let a = court::lattice(&jur(vec![rule("r", Force::Liability, "pay five", Vec::new(), two)]), &c, "man");
    let b = court::lattice(&jur(vec![rule("r", Force::Liability, "pay five", Vec::new(), one)]), &c, "man");
    assert_eq!(a.points.len(), 2);
    assert_eq!(b.points.len(), 1);
    assert_eq!(a.total, 4);
    assert_eq!(b.total, 2);
    assert_ne!(a.total, b.total);
}

#[test]
fn shuffled_rule_sources_lose_the_verdict() {
    let rules = vec![
        rule("theft", Force::Liability, "pay five ox for ox", Vec::new(), vec![vec![cl("steal", Some("ox"))]]),
        rule("vow", Force::Obligation, "", vec![cl("pay", Some("vow"))], vec![vec![cl("vow", None)]]),
    ];
    let c = case(vec![fact("man", "steal", Some("ox"))]);
    let j = jur(rules.clone());
    let l = court::lattice(&j, &c, "man");
    let real = court::evaluate(&j, &l, &court::honest(&l)).0;
    assert_eq!(real.label, "liable: pay five ox for ox");
    // control: each rule's text (its acts and conditions) paired with another rule's force and penalty
    let mut shuffled = rules.clone();
    let (a0, c0) = (shuffled[0].acts.clone(), shuffled[0].conds.clone());
    shuffled[0].acts = shuffled[1].acts.clone();
    shuffled[0].conds = shuffled[1].conds.clone();
    shuffled[1].acts = a0;
    shuffled[1].conds = c0;
    let js = jur(shuffled);
    let ls = court::lattice(&js, &c, "man");
    let fake = court::evaluate(&js, &ls, &court::honest(&ls)).0;
    assert_ne!(fake, real);
}

#[test]
fn a_broad_term_is_a_choice_point_and_the_plain_reading_is_narrow() {
    let j = jur(vec![rule("r", Force::Prohibition, "", vec![cl("steal", None)], Vec::new())]);
    let c = case(vec![fact("man", "take", Some("sheaf"))]);
    let l = court::lattice(&j, &c, "man");
    assert_eq!(l.points.len(), 1);
    assert_eq!(l.total, 2);
    assert_eq!(court::evaluate(&j, &l, &court::honest(&l)).0.label, "free: no rule fires");
    let tips = court::tipping(&j, &l, 2, 3);
    let guilty = tips.keys().find(|v| v.severity == 2).expect("the broad reading convicts");
    assert_eq!(tips[guilty][0].points.len(), 1);
}

#[test]
fn no_discretion_means_no_sensitivity_and_the_goals_collapse() {
    let j = jur(vec![rule("r", Force::Liability, "be put to death", Vec::new(), vec![vec![cl("steal", Some("ox"))]])]);
    let c = case(vec![fact("man", "steal", Some("ox"))]);
    let l = court::lattice(&j, &c, "man");
    assert!(l.points.is_empty());
    let tips = court::tipping(&j, &l, 3, 3);
    assert_eq!(court::sensitivity(&l, &tips), 0.0);
    let gs = report::judge_all(std::slice::from_ref(&j), &c, "man", &Opts::default(), false);
    // the honest verdict serves «convict» with no point; the other goals are unreachable, not forced
    let convict = report::goal_rows("convict", &gs);
    assert_eq!(convict.len(), 1);
    assert_eq!(convict[0].0, 0);
    for goal in ["acquit", "reduce", "reclassify"] {
        assert!(report::goal_rows(goal, &gs).is_empty(), "{goal} must be unreachable");
    }
    // control: with a point to tip, the sensitivity is above zero
    let j2 = jur(vec![rule("r", Force::Liability, "be put to death", Vec::new(), vec![vec![cl("steal", Some("ox"))], vec![cl("sell", None)]])]);
    let l2 = court::lattice(&j2, &c, "man");
    let t2 = court::tipping(&j2, &l2, 3, 3);
    assert!(court::sensitivity(&l2, &t2) > 0.0);
}

#[test]
fn conflicting_rules_are_settled_by_the_chosen_principle() {
    let j = jur(vec![
        rule("general", Force::Liability, "be put to death", Vec::new(), vec![vec![cl("kill", None)]]),
        rule("special", Force::Exemption, "", Vec::new(), vec![vec![cl("kill", None)], vec![cl("break", None)]]),
    ]);
    let c = case(vec![fact("owner", "kill", Some("thief")), fact("thief", "break", None)]);
    let l = court::lattice(&j, &c, "owner");
    // honest: lex specialis, the exemption; tipping the precedence to «strict» gives death
    assert_eq!(court::evaluate(&j, &l, &court::honest(&l)).0.label, "exempt");
    let tips = court::tipping(&j, &l, 1, 3);
    assert!(tips.keys().any(|v| v.severity == 7));
}

#[test]
fn counting_is_deterministic() {
    let j = jur(vec![
        rule("a", Force::Liability, "pay double", Vec::new(), vec![vec![cl("steal", None)], vec![cl("sell", None)]]),
        rule("b", Force::Prohibition, "", vec![cl("steal", None)], vec![vec![cl("night", None)]]),
    ]);
    let c = case(vec![fact("man", "take", Some("ox"))]);
    let a = court::lattice(&j, &c, "man");
    let b = court::lattice(&j, &c, "man");
    assert_eq!(a.total, b.total);
    assert_eq!(format!("{:?}", a.counts), format!("{:?}", b.counts));
    let opts = Opts::default();
    assert_eq!(report::report(std::slice::from_ref(&j), &c, &opts), report::report(std::slice::from_ref(&j), &c, &opts));
}

#[test]
fn flagged_rules_leave_the_reference() {
    let mut j = jur(vec![rule("r", Force::Liability, "be put to death", Vec::new(), vec![vec![cl("steal", None)]]), rule("s", Force::Liability, "pay", Vec::new(), vec![vec![cl("steal", None)]])]);
    j.flags = vec![yomhadin::juris::Flag { pattern: "death".into(), kind: "penalty".into(), field: "penalty".into(), reason: "the death penalty".into() }];
    assert_eq!(j.flags_of(&j.rules[0]).len(), 1);
    assert!(j.flags_of(&j.rules[1]).is_empty());
    assert_eq!(j.unflagged().rules.len(), 1);
}

#[test]
fn reference_statistics() {
    let xs = [(0u8, 1.0), (2, 1.0), (2, 1.0), (5, 1.0)];
    let (modal, med, _, _, min, max, total) = report::stats(&xs).unwrap();
    assert_eq!((modal, med, min, max), (2, 2, 0, 5));
    assert_eq!(total, 4.0);
    assert_eq!(report::percentile(&xs, 5), 87.5);
    assert!(report::stats(&[]).is_none());
}

fn device_jur() -> Jurisdiction {
    let mut j = jur(vec![
        rule("carry-street", Force::Prohibition, "", vec![Clause { verb: "carry".into(), neg: false, patient: None, args: vec!["in:street".into()] }], Vec::new()),
        rule("carry-courtyard", Force::Permission, "", vec![Clause { verb: "carry".into(), neg: false, patient: None, args: vec!["in:courtyard".into()] }], Vec::new()),
    ]);
    j.devices = vec![yomhadin::juris::Device { id: "eruv".into(), name: "the eruv".into(), trigger: vec!["eruv".into()], field: "place".into(), substance: vec!["street".into()], form: vec!["courtyard".into()], source: "tractate Eruvin".into(), note: String::new() }];
    j
}

#[test]
fn a_device_changes_the_verdict_and_a_case_without_one_has_none() {
    let j = device_jur();
    let mut f = fact("man", "carry", Some("book"));
    f.args = vec!["in:street".into(), "inside:eruv".into()];
    let c = case(vec![f]);
    let views = court::device_views(&j, &c);
    assert_eq!(views.len(), 1);
    let (_, with, without) = &views[0];
    let lw = court::lattice(&j, with, "man");
    let lo = court::lattice(&j, without, "man");
    assert_eq!(court::evaluate(&j, &lw, &court::honest(&lw)).0.severity, 0, "the eruv permits carrying");
    assert_eq!(court::evaluate(&j, &lo, &court::honest(&lo)).0.severity, 2, "without it carrying in the street is forbidden");
    // negative control: no eruv in the case, no device, and the lens says none
    let mut g = fact("man", "carry", Some("book"));
    g.args = vec!["in:street".into()];
    let plain = case(vec![g]);
    assert!(court::device_views(&j, &plain).is_empty());
    let mut o = String::new();
    report::fiction(&mut o, std::slice::from_ref(&j), &plain, &Opts::default(), true);
    assert!(o.contains("none"), "{o}");
}

#[test]
fn an_ordinary_proportionate_case_has_no_absurdity_and_a_disproportionate_one_has() {
    let c = case(vec![fact("man", "steal", Some("ox"))]);
    // paying for a theft: one class from the harm of property, no point to tip, no status: no flag
    let fair = jur(vec![rule("r", Force::Liability, "pay double", Vec::new(), vec![vec![cl("steal", Some("ox"))]])]);
    assert!(yomhadin::absurd::flags(&fair, &c, "man", 3).is_empty());
    // control: death for the same theft is four classes from the harm
    let harsh = jur(vec![rule("r", Force::Liability, "be put to death", Vec::new(), vec![vec![cl("steal", Some("ox"))]])]);
    let f = yomhadin::absurd::flags(&harsh, &c, "man", 3);
    assert!(f.iter().any(|x| x.kind == "disproportion" && x.score == 3), "{f:?}");
}

#[test]
fn a_verdict_that_turns_on_status_alone_is_flagged() {
    let mut r = rule("fire", Force::Prohibition, "", vec![cl("light", Some("fire"))], Vec::new());
    r.parties = vec!["agent=jew".into()];
    let j = jur(vec![r]);
    let c = case(vec![fact("jew", "light", Some("fire"))]);
    let f = yomhadin::absurd::flags(&j, &c, "jew", 3);
    assert!(f.iter().any(|x| x.kind == "status-only"), "{f:?}");
    // control: the same rule addressed to anyone does not turn on status
    let j2 = jur(vec![rule("fire", Force::Prohibition, "", vec![cl("light", Some("fire"))], Vec::new())]);
    assert!(!yomhadin::absurd::flags(&j2, &c, "jew", 3).iter().any(|x| x.kind == "status-only"));
}

#[test]
fn wave31_count_matches_explicit_assignments_and_ties_keep_first() {
    let j = jur(vec![
        rule(
            "first",
            Force::Prohibition,
            "",
            vec![cl("steal", None)],
            vec![vec![cl("sell", None)]],
        ),
        rule(
            "second",
            Force::Permission,
            "",
            vec![cl("steal", None)],
            vec![vec![cl("kill", None)]],
        ),
    ]);
    let c = case(vec![fact("actor", "take", None)]);
    let l = court::lattice(&j, &c, "actor");
    let sizes: Vec<_> = l.points.iter().map(|p| p.options.len()).collect();
    let total: usize = sizes.iter().product();
    assert!(total <= 128);
    let mut explicit = std::collections::BTreeMap::new();
    for mut code in 0..total {
        let a: Vec<_> = sizes
            .iter()
            .map(|&n| {
                let x = code % n;
                code /= n;
                x
            })
            .collect();
        *explicit
            .entry(court::evaluate(&j, &l, &a).0)
            .or_insert(0u128) += 1;
    }
    assert_eq!(l.total, total as u128);
    assert_eq!(
        l.counts
            .iter()
            .map(|(v, x)| (v.clone(), x.0))
            .collect::<std::collections::BTreeMap<_, _>>(),
        explicit
    );
    let a: Vec<_> = l.points.iter().map(|p| p.holds).collect();
    assert_eq!(court::evaluate(&j, &l, &a).1, Some(0));
    let zero = court::tipping(&j, &l, 0, 0);
    assert_eq!(zero.len(), 1);
    assert_eq!(zero.values().next().unwrap()[0].points.len(), 0);
}

#[test]
fn wave31_extraction_adjacency_disputes_and_metric_denominators() {
    use yomhadin::{extract, measure};
    let mut a = rule("a", Force::Prohibition, "", vec![cl("carry", None)], vec![]);
    a.source = "S.1.2.1".into();
    let mut b = rule(
        "b",
        Force::Permission,
        "",
        vec![cl("carry", None)],
        vec![vec![cl("open", None), cl("close", None)]],
    );
    b.source = a.source.clone();
    b.authority = "Other".into();
    b.text = "But if open".into();
    let mut rs = vec![a, b];
    extract::attach_but_if(&mut rs);
    assert_eq!(rs[0].excs, vec![vec![cl("open", None)]]);
    extract::mark_disputes(&mut rs);
    assert_eq!(rs[0].dispute, "dispute:S.1.2:carry");
    assert_eq!(rs[0].dispute, rs[1].dispute);
    assert!(extract::in_filter("S.2.4", "S.1-2"));
    assert!(!extract::in_filter("S.3.4", "S.1-2"));
    let gold = std::collections::BTreeMap::from([
        (
            "a".into(),
            measure::Gold {
                rules: vec![(Force::Prohibition, vec!["carry".into()])],
                authority: "?".into(),
            },
        ),
        (
            "b".into(),
            measure::Gold {
                rules: vec![(Force::Permission, vec!["carry".into()])],
                authority: "OTHER".into(),
            },
        ),
    ]);
    let js = vec![jur(rs)];
    let sample = measure::sample(&js, &gold);
    assert_eq!(sample.len(), 2);
    let order = vec!["a".into(), "b".into()];
    assert_eq!(
        measure::precision(&sample, &gold, &order, 0),
        "2 rules of 2 judged: force+act 2 (100%), authority 1 of 1 (100%), both 2 (100%)"
    );
    assert!(measure::precision(&sample, &gold, &order, 1).contains("force+act 0 (0%)"));
    assert_eq!(
        measure::recall(&sample, &gold),
        "2 of 2 gold rules found (100%)"
    );
    assert!(measure::precision(&sample, &gold, &[], 0).starts_with("0 rules of 2 judged"));
}

#[test]
fn wave31_codec_statistics_and_status_fibers() {
    let c = Clause {
        verb: "put down".into(),
        neg: true,
        patient: None,
        args: vec!["at:home".into()],
    };
    assert_eq!(Clause::dec(&c.enc()).unwrap(), c);
    assert_eq!(
        Clause::dec("put_down/-/at:home/ignored").unwrap().verb,
        "put down"
    );
    assert!(Clause::dec("").is_err());
    let xs = [(2, 1.0), (5, 1.0)];
    assert_eq!(report::stats(&xs).unwrap(), (2, 2, 2, 5, 2, 5, 2.0));
    assert_eq!(report::percentile(&xs, 2), 25.0);
    assert_eq!(report::percentile(&[], 2), 0.0);
    assert!(report::stats(&[(2, 0.0)]).is_none());
    let mut f = fact("jew", "carry", Some("gentile"));
    f.args = vec!["from:jew".into(), "gentile".into()];
    let mut c = case(vec![f]);
    c.persons = vec!["jew".into()];
    c.places = vec!["jew".into()];
    let s = yomhadin::absurd::swap_status(&c, "jew", "gentile");
    assert_eq!(s.facts[0].agent.as_deref(), Some("gentile"));
    assert_eq!(s.facts[0].args, ["from:gentile", "gentile"]);
    assert_eq!(s.places, c.places);
    let back = yomhadin::absurd::swap_status(&s, "jew", "gentile");
    assert_eq!(back.facts, c.facts);
    assert_eq!(back.persons, c.persons);
}

#[test]
fn wave31_jurisdiction_cover_direction_and_lookup_order() {
    use yomhadin::juris::{Authority, Cover, Penalty};
    let mut j = jur(vec![]);
    j.terms[0].opposite = vec!["give".into()];
    assert_eq!(j.cover("take", "steal"), Cover::Broad);
    assert_eq!(j.cover("steal", "steal"), Cover::Sure);
    assert_eq!(j.cover("missing", "other"), Cover::No);
    assert!(j.opposite("steal", "give"));
    assert!(!j.opposite("give", "steal"));
    j.authorities = vec![
        Authority {
            pattern: "a".into(),
            class: "first".into(),
            rank: 9,
        },
        Authority {
            pattern: "a".into(),
            class: "second".into(),
            rank: 1,
        },
    ];
    assert_eq!(j.authority("A"), ("first".into(), 9));
    j.penalties = vec![
        Penalty {
            pattern: "pay".into(),
            rank: 2,
            gloss: "first".into(),
        },
        Penalty {
            pattern: "pay".into(),
            rank: 2,
            gloss: "last".into(),
        },
    ];
    assert_eq!(j.severity("PAY"), Some((2, "last".into())));
    assert_eq!(j.severity("nothing"), None);
}
