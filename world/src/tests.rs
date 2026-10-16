//! Tests: the prompt example passes the gates (positive control), negative controls of every gate
//! (teleportation, reference to a non-existent entity, silent change, two-way link mismatch, nearby,
//! anchors, closed sets), immutability of old states, rollback of a rejected command, queries from the state.

use crate::lang::{Line, parse_all, parse_line};
use crate::qa::{Verdict, answer, verdict};
use crate::types::*;
use crate::vmm::{EXAMPLE_CMDS, EXAMPLE_TEXT};
use crate::world::{Delta, Gate, Text, World, run};

fn text(n: usize) -> Text {
    Text { doc: 0, title: "test".into(), sents: (1..=n).map(|i| (1, format!("sentence {i}"))).collect() }
}

fn line(s: &str) -> Line {
    parse_line(1, s).expect("format").expect("not empty")
}

/// A world from lines, each must pass.
fn world(n: usize, cmds: &str) -> World {
    let (lines, errs) = parse_all(cmds);
    assert!(errs.is_empty(), "format: {errs:?}");
    let mut w = World::new(text(n));
    for l in &lines {
        w.apply(l).unwrap_or_else(|e| panic!("{}: {e}", l.raw));
    }
    w
}

fn gate_of(w: &mut World, s: &str) -> Gate {
    match w.apply(&line(s)) {
        Ok(()) => panic!("gate is silent: {s}"),
        Err(e) => e.gate,
    }
}

fn id(s: &str) -> Id {
    Id::parse(s).unwrap()
}

const BASE: &str = "loc L1 type=forest ^s1
loc L2 type=tree in=L1 ^s1
loc L3 type=river ^s1
char C1 type=animal class=crow name=\"Crow\" at=L2 gender=female ^s1
char C2 type=animal class=fox name=\"Fox\" at=L1 ^s1
char C3 type=animal class=frog at=L3 ^s1
obj O1 type=food class=meat at=C1 ^s1
obj O2 type=stone at=L3 ^s1
event E1 verb=steal agent=C1 patient=O1 ^s1";

fn example() -> World {
    let t = Text { doc: 0, title: "The Girl and the Dog".into(), sents: EXAMPLE_TEXT.iter().map(|(p, s)| (*p, s.to_string())).collect() };
    let (lines, errs) = parse_all(EXAMPLE_CMDS);
    assert!(errs.is_empty(), "example, format: {errs:?}");
    let r = run(t, &lines);
    assert!(r.rejects.is_empty(), "example, gates: {:?}", r.rejects.iter().map(|(l, e)| format!("{} — {e}", l.raw)).collect::<Vec<_>>());
    assert!(r.uncovered.is_empty(), "example, sentences without commands: {:?}", r.uncovered);
    r.world
}

#[test]
fn prompt_example_passes_all_gates() {
    let w = example();
    assert_eq!(w.ents(Reg::C).count(), 3);
    assert_eq!(w.events().len(), 9);
    assert!(w.check_links().is_ok());
    // the apple: with the girl → in the grass (because of E3 slip) → with the dog (because of E4 snatch)
    let hist: Vec<String> = w.history(id("O1"), Field::At).iter().map(|(st, v)| format!("{v}{}", st.cause.map(|c| format!("/{c}")).unwrap_or_default())).collect();
    assert_eq!(hist, ["C1", "L3/E3", "C3/E4"]);
}

#[test]
fn teleport_is_rejected() {
    let mut w = world(3, BASE);
    // only move changes location
    assert_eq!(gate_of(&mut w, "set C2 at=L3 cause=E1 ^s2"), Gate::Teleport);
    assert_eq!(gate_of(&mut w, "set O1 at=C2 ^s2"), Gate::Teleport);
    // re-creating a character in another place — also not allowed
    assert_eq!(gate_of(&mut w, "char C2 type=animal class=fox at=L3 ^s2"), Gate::Ref);
    // from character to character — only give
    assert_eq!(gate_of(&mut w, "move O1 to=C2 ^s2"), Gate::Teleport);
    // picking a stone up from the river while in the forest — not nearby
    assert_eq!(gate_of(&mut w, "move O2 to=C2 ^s2"), Gate::Near);
    // handing to the frog that is in the river — not nearby
    assert_eq!(gate_of(&mut w, "give C1 O1 to=C3 ^s2"), Gate::Near);
    // dropping from the tree into the river — not nearby; into the forest under the tree — allowed
    assert_eq!(gate_of(&mut w, "move O1 to=L3 ^s2"), Gate::Near);
    w.apply(&line("move O1 to=L1 ^s2")).unwrap();
    // an event where the agent is not
    assert_eq!(gate_of(&mut w, "event E2 verb=swim agent=C2 at=L3 ^s2"), Gate::Continuity);
    // and move is the legal way
    w.apply(&line("move C2 to=L3 ^s3")).unwrap();
    assert_eq!(w.now(id("C2")).unwrap().id(Field::At), Some(id("L3")));
}

#[test]
fn reference_to_missing_is_rejected() {
    let mut w = world(2, BASE);
    assert_eq!(gate_of(&mut w, "move C9 to=L1 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "move C1 to=L9 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "char C4 type=human class=hunter at=L7 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "event E2 verb=bite agent=C1 patient=O9 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "set C1 feeling=proud cause=E7 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "rel R1 kind=friend a=C1 b=C8 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "say C1 to=C9 act=greet id=E2 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "event E1 verb=again agent=C1 ^s2"), Gate::Ref);
    // wrong registry
    assert_eq!(gate_of(&mut w, "move C1 to=C2 ^s2"), Gate::Kind);
}

#[test]
fn silent_change_is_rejected() {
    let mut w = world(4, BASE);
    // unknown → known without a cause — allowed (the text reveals, not changes)
    w.apply(&line("set C1 feeling=calm ^s2")).unwrap();
    // known → other without a cause — silent change
    assert_eq!(gate_of(&mut w, "set C1 feeling=proud ^s2"), Gate::Silent);
    assert_eq!(gate_of(&mut w, "set C1 gender=male ^s2"), Gate::Silent);
    // with a cause — allowed
    w.apply(&line("say C2 to=C1 act=evaluate indirect=1 sincere=0 means=\"wants the meat\" id=E2 ^s3")).unwrap();
    w.apply(&line("set C1 feeling=proud cause=E2 ^s3")).unwrap();
    assert_eq!(w.now(id("C1")).unwrap().get(Field::Feeling), Val::Known(V::Tag(FEELING.tag("proud").unwrap())));
    // a relationship changes kind only with a cause
    w.apply(&line("rel R1 kind=acquaintance a=C1 b=C2 ^s3")).unwrap();
    assert_eq!(gate_of(&mut w, "rel R1 kind=adversary ^s4"), Gate::Silent);
    w.apply(&line("rel R1 kind=adversary cause=E2 ^s4")).unwrap();
}

#[test]
fn two_way_mismatch_is_rejected() {
    let mut w = world(3, BASE);
    // have is literally C.items += O, O.at = C; the crow still holds the meat → mismatch
    assert_eq!(gate_of(&mut w, "have C2 O1 ^s2"), Gate::Link);
    // the stone lies in the river (L3.objects ∋ O2) — have does not remove it from there
    assert_eq!(gate_of(&mut w, "have C3 O2 ^s2"), Gate::Link);
    // the other end of a relationship — the old end would keep a reference
    w.apply(&line("rel R1 kind=rival a=C1 b=C2 ^s2")).unwrap();
    assert_eq!(gate_of(&mut w, "rel R1 b=C3 ^s3"), Gate::Link);
    // legal ways: give between neighbours, move O to=C for a lying object
    w.apply(&line("move C2 to=L2 ^s3")).unwrap();
    w.apply(&line("give C1 O1 to=C2 ^s3")).unwrap();
    w.apply(&line("move O2 to=C3 ^s3")).unwrap();
    assert!(w.check_links().is_ok());
    // the check sees a mismatch even if it was introduced bypassing the executor
    w.inject(id("L1"), vec![Delta::Add(SetF::Chars, id("C3"))]);
    let e = w.check_links().unwrap_err();
    assert_eq!(e.gate, Gate::Link, "{e}");
}

#[test]
fn old_states_stay_immutable() {
    let mut w = world(4, BASE);
    let c1 = id("C1");
    let v1 = w.ent(c1).unwrap().latest();
    let before = w.ent(c1).unwrap().snap(v1);
    let st_before = w.ent(c1).unwrap().state(v1).unwrap().clone();
    for s in [
        "set C1 feeling=proud ^s2",
        "say C2 to=C1 act=evaluate id=E2 ^s2",
        "set C1 feeling=offended cause=E2 ^s3",
        "move O1 to=L1 cause=E2 ^s3",
        "move C1 to=L1 ^s4",
        "rel R1 kind=adversary a=C1 b=C2 cause=E2 ^s4",
    ] {
        w.apply(&line(s)).unwrap();
    }
    let e = w.ent(c1).unwrap();
    assert!(e.latest() > v1);
    // old state — the same snapshot and the same delta
    assert_eq!(e.snap(v1), before);
    assert_eq!(e.state(v1).unwrap(), &st_before);
    // new state — another snapshot; inside only the delta (no type/name field — they are not copied)
    assert_ne!(e.now(), before);
    let last = e.state(e.latest()).unwrap();
    assert!(last.delta.iter().all(|d| !matches!(d, Delta::Set(Field::Type | Field::Name, _))), "{:?}", last.delta);
    // the difference between states is visible at once
    let d = w.diff(c1, v1, e.latest()).unwrap();
    assert!(d.contains("feeling: ? → offended") && d.contains("at: L2 → L1"), "{d}");
}

#[test]
fn rejected_command_leaves_world_unchanged() {
    let mut w = world(2, BASE);
    let n: Vec<u32> = w.ents(Reg::C).map(|e| e.latest()).collect();
    let ev = w.events().len();
    // have writes two entities, the link gate after the write rejects — rollback, neither changes
    assert!(w.apply(&line("have C2 O1 ^s2")).is_err());
    assert!(w.apply(&line("give C1 O1 to=C3 ^s2")).is_err());
    assert_eq!(w.ents(Reg::C).map(|e| e.latest()).collect::<Vec<_>>(), n);
    assert_eq!(w.events().len(), ev);
    assert!(w.check_links().is_ok());
}

#[test]
fn anchors_are_checked() {
    let mut w = world(3, BASE);
    w.apply(&line("move C2 to=L2 ^s2")).unwrap();
    assert_eq!(gate_of(&mut w, "move C2 to=L1 ^s1"), Gate::Anchor);
    assert_eq!(gate_of(&mut w, "move C2 to=L1 ^s9"), Gate::Anchor);
    assert_eq!(gate_of(&mut w, "cursor para=2 ^s3"), Gate::Anchor);
    w.apply(&line("cursor chapter=1 para=1 sent=3")).unwrap();
}

#[test]
fn format_is_strict() {
    for bad in [
        "loc L1 type=spaceship ^s1",
        "set C1 feeling=hangry ^s1",
        "say C1 act=threaten ^s1",
        "jump C1 ^s1",
        "move C1 to=L2",
        "move C1 to=L2 speed=fast ^s1",
        "move C1 to=L2 to=L3 ^s1",
        "char C1 type=animal class=Fox ^s1",
        "char C1 type=animal class=fox name=\"Fox ^s1",
        "event E1 verb=run why=\"a b c d e f g h i j k l m n o p q r s t u v w x y z\" ^s1",
        "have C1 L1 ^s1",
        "rel R1 kind=enemy a=C1 b=C2 ^s1",
        "set C1 ^s1",
        "^s1 move C1 to=L2",
    ] {
        assert!(parse_line(1, bad).is_err(), "format let through: {bad}");
    }
    assert!(parse_line(1, "   ").unwrap().is_none());
    let l = line("say C2 to=C1 act=evaluate indirect=1 sincere=0 means=\"wants the meat\" why=\"to get the meat\" id=E3 ^s3 ^s4");
    assert_eq!(l.anchors, [3, 4]);
}

#[test]
fn answers_come_from_state() {
    let w = example();
    let a = |q: &str| answer(&w, q);
    let hit = |q: &str, e: &str| {
        let x = a(q);
        assert_eq!(verdict(&x, e), Verdict::Hit, "{q} → {}", x.text);
    };
    hit("holder obj:apple @first @last", "girl,dog");
    hit("holders obj:apple", "girl,grass|garden,dog,snatch");
    hit("where char:girl ^s2", "tree");
    hit("attr char:girl feeling ^s4", "angry");
    hit("attr char:girl feeling @last", "happy,feed");
    hit("cause char:dog status=fed", "eat");
    hit("say char:grandmother to=char:girl", "suggest,protect");
    hit("why verb=snatch agent=char:dog", "hungry");
    hit("who verb=feed patient=char:dog", "girl");
    hit("rel char:girl char:dog", "adversary,friend");
    hit("predict-rel char:girl char:dog", "help");
    hit("main", "girl");
    hit("say narrator", "kind");
    hit("state char:dog ^s3", "status=fed");
    hit("mentions char:dog", "^s3");
    hit("diff char:girl 1 3", "at");
    hit("verb=nonsense || who verb=climb", "girl");
    // honest «not in state»
    for q in ["attr char:dog age @last", "who verb=fly", "where char:unicorn ^s1", "why verb=climb", "predict-rel char:girl char:grandmother"] {
        let x = a(q);
        if q.starts_with("predict") {
            // kin gives a prediction — that is an answer
            assert!(x.from_state, "{q}");
            continue;
        }
        assert_eq!(verdict(&x, "x"), Verdict::NotInState, "{q} → {}", x.text);
    }
}

#[test]
fn questions_file_is_well_formed() {
    let qs = crate::qa::read_questions(include_str!("../data/questions.tsv")).unwrap();
    assert_eq!(qs.len(), 27);
    for doc in [2451, 2355, 2366] {
        let n = qs.iter().filter(|q| q.doc == doc && q.kind != crate::qa::CONTROL).count();
        assert!((6..=8).contains(&n), "doc {doc}: {n} questions");
    }
    // every query reading is a known query (on an empty world: «not in state», not «unknown query»)
    let w = World::new(text(8));
    for q in &qs {
        for alt in q.query.split("||") {
            let a = answer(&w, alt.trim());
            assert!(!a.text.contains("unknown query"), "{}: {}", q.id, a.text);
        }
    }
}

#[test]
fn verdict_respects_order_and_alternatives() {
    use crate::qa::Answer;
    let a = Answer { text: "@first: C1 Crow (^s1); @last: C2 Fox (^s6)".into(), from_state: true };
    assert_eq!(verdict(&a, "crow>fox"), Verdict::Hit);
    assert_eq!(verdict(&a, "fox>crow"), Verdict::Miss);
    assert_eq!(verdict(&a, "crow>^s6|^s9,fox"), Verdict::Hit);
    assert_eq!(verdict(&a, "crow,wolf|bear"), Verdict::Miss);
    let none = Answer { text: "not in state: x".into(), from_state: false };
    assert_eq!(verdict(&none, "x"), Verdict::NotInState);
}

/// Real LLM output for «The Fox and the Crow» (run 2026-09-26) and its mutations: gates that are silent on
/// the clean output must turn red on the spoiled one.
#[test]
fn real_vmm_output_and_its_mutations() {
    let cmds = include_str!("../data/cmds-2451-fox-crow.txt");
    let (lines, errs) = parse_all(cmds);
    assert!(errs.is_empty());
    let clean = run(text(6), &lines);
    assert!(clean.rejects.is_empty() && clean.uncovered.is_empty());
    // the first rejected line after the mutation and its gate
    let first_gate = |edit: &dyn Fn(&str) -> Option<String>| {
        let src: Vec<String> = cmds.lines().filter_map(edit).collect();
        let (lines, errs) = parse_all(&src.join("\n"));
        assert!(errs.is_empty(), "{errs:?}");
        let r = run(text(6), &lines);
        r.rejects.first().map(|(l, e)| (l.raw.clone(), e.gate))
    };
    // the Crow did not drop the meat — the Fox «picked it up» straight from the beak
    let (raw, g) = first_gate(&|l| (!l.starts_with("move O1 to=L1")).then(|| l.to_string())).unwrap();
    assert_eq!((raw.starts_with("move O1 to=C2"), g), (true, Gate::Teleport));
    // the Fox «has» meat lying on the ground (without move)
    let (_, g) = first_gate(&|l| Some(if l.starts_with("move O1 to=C2") { "have C2 O1 ^s6".into() } else { l.to_string() })).unwrap();
    assert_eq!(g, Gate::Link);
    // a feeling changed without a cause
    let (_, g) = first_gate(&|l| Some(l.replace("feeling=ashamed cause=E10", "feeling=ashamed"))).unwrap();
    assert_eq!(g, Gate::Silent);
    // reference to a non-existent character
    let (_, g) = first_gate(&|l| Some(l.replace("agent=C2 patient=O1", "agent=C7 patient=O1"))).unwrap();
    assert_eq!(g, Gate::Ref);
    // an event «on the ground» while the Crow is in the tree — ok (the tree is in the ground); in the river — not
    let (_, g) = first_gate(&|l| {
        Some(if l.starts_with("loc L2") { format!("{l}\nloc L3 type=river ^s1") } else { l.replace("verb=caw agent=C1 at=L2", "verb=caw agent=C1 at=L3") })
    })
    .unwrap();
    assert_eq!(g, Gate::Continuity);
}

// ── v2 ────────────────────────────────────────────────────────────────────────────────────────────

use crate::vmm::{EXAMPLE2_CMDS, EXAMPLE2_TEXT};

fn example2() -> World {
    let t = Text { doc: 0, title: "The Miller's Goose".into(), sents: EXAMPLE2_TEXT.iter().map(|(p, s)| (*p, s.to_string())).collect() };
    let (lines, errs) = parse_all(EXAMPLE2_CMDS);
    assert!(errs.is_empty(), "example v2, format: {errs:?}");
    let r = run(t, &lines);
    assert!(r.rejects.is_empty(), "example v2, gates: {:?}", r.rejects.iter().map(|(l, e)| format!("{} — {e}", l.raw)).collect::<Vec<_>>());
    assert!(r.uncovered.is_empty(), "example v2, sentences without commands: {:?}", r.uncovered);
    r.world
}

fn tag(set: &'static Closed, s: &str) -> Tag {
    set.tag(s).unwrap()
}

/// Positive control v2: the prompt example passes all gates, the state is as intended.
#[test]
fn prompt_example_v2_passes_all_gates() {
    let w = example2();
    assert!(w.check_links().is_ok());
    let snap = |s: &str| w.now(id(s)).unwrap();
    // the promise is kept by event E5, the wife's plan abandoned at E7
    assert_eq!(snap("P2").get(Field::Stage), Val::Known(V::Tag(tag(&PLAN_STATUS, "kept"))));
    assert_eq!(snap("P2").id(Field::By), Some(id("E5")));
    assert_eq!(snap("P1").get(Field::Stage), Val::Known(V::Tag(tag(&PLAN_STATUS, "dropped"))));
    // several feelings at once
    assert_eq!(snap("C2").get(Field::Feeling), Val::Known(V::Tags(vec![tag(&FEELING, "happy"), tag(&FEELING, "ashamed")].into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect())));
    // goal reached: goal unknown, the state created done with a cause
    let g = w.history(id("C3"), Field::Goal);
    assert_eq!(g.len(), 2);
    assert_eq!((g[1].0.by, g[1].0.cause, g[1].1.clone()), ("done", Some(id("E7")), Val::Unknown));
    // a part: the wing is detached and with the fox
    assert_eq!(snap("O1").id(Field::At), Some(id("C4")));
    assert_eq!(snap("O1").id(Field::PartOf), None);
    assert!(!snap("C3").has(SetF::Parts, id("O1")));
    // shared agents and several acts
    let e10 = w.event(id("E10")).unwrap();
    assert_eq!(e10.all(crate::lang::Role::Agent).iter().map(|s| s.id).collect::<Vec<_>>(), [id("C1"), id("C2")]);
    let e1 = w.event(id("E1")).unwrap().speech.as_ref().unwrap();
    assert_eq!(e1.acts_str(), "request+promise");
    // negation, memory, story time, about what
    assert!(w.event(id("E3")).unwrap().x.neg);
    assert_eq!(w.event(id("E5")).unwrap().x.before, Some(id("E4")));
    assert_eq!(w.event(id("E8")).unwrap().x.when.as_deref(), Some("night"));
    assert_eq!(w.event(id("E7")).unwrap().x.about, Some(id("P1")));
    // beliefs: false, abandoned
    assert_eq!(snap("B1").get(Field::Truth), Val::Known(V::Tag(crate::world::TRUTH.tag("false").unwrap())));
    assert_eq!(snap("B1").get(Field::Stage), Val::Known(V::Tag(tag(&BELIEF_STATUS, "dropped"))));
    // here=0 and mention do not move the cursor: the last cursor is at the mill (next_morning)
    let c = w.cursor().last().unwrap();
    assert_eq!((c.at, c.time.as_deref()), (Some(id("L2")), Some("next_morning")));
    assert!(w.ent(id("L1")).unwrap().mentions().iter().any(|p| p.sent == 9));
    // directed relationship
    assert_eq!(snap("R2").get(Field::Kind), Val::Known(V::Tag(tag(&REL_KIND, "benefactor"))));
    // a number
    assert_eq!(snap("O3").get(Field::Num), Val::Known(V::Words(vec!["many".into()])));
}

const BASE2: &str = "loc L1 type=house ^s1
loc L2 type=river ^s1
char C1 type=human class=girl at=L1 ^s1
char C2 type=human class=witch at=L1 ^s1
char C3 type=human class=boy at=L2 ^s1
obj O1 type=body_part class=nose part=C2 ^s1
obj O2 type=food class=cake at=L1 num=2 ^s1
say C2 to=C1 act=promise means=\"I will give you a cake\" id=E1 ^s1
plan P1 kind=promise who=C2 to=C1 what=\"give a cake\" src=E1 ^s1
believe B1 who=C1 claim=\"the witch is kind\" about=C2 from=E1 ^s1
event E2 verb=give agent=C2 patient=O2 to=C1 neg=1 ^s1
event E3 verb=laugh agent=C2 ^s1";

/// Negative controls v2: every new command turns red with its own gate.
#[test]
fn v2_gates_are_not_mute() {
    let mut w = world(4, BASE2);
    // intention: closing without an event — silent change; kept by an event that did not happen — continuity
    assert_eq!(gate_of(&mut w, "plan P1 status=kept ^s2"), Gate::Silent);
    assert_eq!(gate_of(&mut w, "plan P1 status=kept by=E2 ^s2"), Gate::Continuity);
    assert_eq!(gate_of(&mut w, "plan P1 status=kept by=E9 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "plan P7 status=broken by=E3 ^s2"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "plan P2 kind=wish who=C9 what=\"fly\" ^s2"), Gate::Ref);
    w.apply(&line("plan P1 status=broken by=E3 ^s2")).unwrap();
    // closed once
    assert_eq!(gate_of(&mut w, "plan P1 status=kept by=E3 ^s2"), Gate::Continuity);
    // beliefs: change without an event — silent; abandoning twice — continuity
    assert_eq!(gate_of(&mut w, "believe B1 true=0 ^s2"), Gate::Silent);
    assert_eq!(gate_of(&mut w, "believe B2 who=C1 claim=\"x\" from=E8 ^s2"), Gate::Ref);
    w.apply(&line("believe B1 status=dropped true=0 by=E3 ^s2")).unwrap();
    assert_eq!(gate_of(&mut w, "believe B1 status=dropped by=E3 ^s2"), Gate::Continuity);
    // goal: done without a goal — continuity; without a cause — silent; a cause that did not happen — continuity
    assert_eq!(gate_of(&mut w, "done C1 cause=E3 ^s2"), Gate::Continuity);
    w.apply(&line("set C1 goal=\"get a cake\" ^s2")).unwrap();
    assert_eq!(gate_of(&mut w, "done C1 ^s2"), Gate::Silent);
    assert_eq!(gate_of(&mut w, "done C1 cause=E2 ^s2"), Gate::Continuity);
    assert_eq!(gate_of(&mut w, "unset C1 goal ^s2"), Gate::Silent);
    assert_eq!(gate_of(&mut w, "unset C1 status cause=E3 ^s2"), Gate::Continuity);
    w.apply(&line("unset C1 goal cause=E3 why=\"the witch lied\" ^s2")).unwrap();
    assert_eq!(w.history(id("C1"), Field::Goal).last().unwrap().0.note.as_deref(), Some("the witch lied"));
    // feelings: revealing without a cause — allowed; changing a known one without a cause — not; removing an absent one — not
    w.apply(&line("feel C1 +hopeful ^s2")).unwrap();
    assert_eq!(gate_of(&mut w, "feel C1 +sad ^s3"), Gate::Silent);
    assert_eq!(gate_of(&mut w, "feel C1 -angry cause=E3 ^s3"), Gate::Continuity);
    w.apply(&line("feel C1 +sad +angry -hopeful cause=E3 ^s3")).unwrap();
    assert_eq!(w.now(id("C1")).unwrap().get(Field::Feeling), Val::Known(V::tags(vec![tag(&FEELING, "sad"), tag(&FEELING, "angry")]).unwrap()));
    // a part: does not move and is not transferred separately; detach — only with a cause and nearby
    assert_eq!(gate_of(&mut w, "move O1 to=L1 ^s3"), Gate::Link);
    assert_eq!(gate_of(&mut w, "give C2 O1 to=C1 ^s3"), Gate::Link);
    assert_eq!(gate_of(&mut w, "have C1 O1 ^s3"), Gate::Link);
    assert_eq!(gate_of(&mut w, "detach O1 to=C1 ^s3"), Gate::Silent);
    assert_eq!(gate_of(&mut w, "detach O1 to=L2 cause=E3 ^s3"), Gate::Near);
    assert_eq!(gate_of(&mut w, "detach O2 to=L1 cause=E3 ^s3"), Gate::Continuity);
    w.apply(&line("event E4 verb=cut_off agent=C1 patient=O1 at=L1 ^s3")).unwrap();
    w.apply(&line("detach O1 to=C1 cause=E4 ^s3")).unwrap();
    assert_eq!(w.now(id("O1")).unwrap().id(Field::At), Some(id("C1")));
    assert_eq!(gate_of(&mut w, "attach O1 to=C3 cause=E4 ^s3"), Gate::Near);
    // transfer through an event that did not happen — continuity
    assert_eq!(gate_of(&mut w, "move O2 to=C1 cause=E2 ^s3"), Gate::Continuity);
    // a number: a known one changes only with a cause
    assert_eq!(gate_of(&mut w, "set O2 num=1 ^s3"), Gate::Silent);
    w.apply(&line("set O2 num=1 cause=E3 ^s3")).unwrap();
    // shared agents: each must be where the event is; a non-existent one — reference
    assert_eq!(gate_of(&mut w, "event E5 verb=dance agent=C1+C3 at=L1 ^s4"), Gate::Continuity);
    assert_eq!(gate_of(&mut w, "event E5 verb=dance agent=C1+C9 ^s4"), Gate::Ref);
    // references to the past: about/before/cause — only to existing ones
    assert_eq!(gate_of(&mut w, "say C1 to=C2 act=ask about=E9 id=E5 ^s4"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "event E5 verb=remember agent=C1 before=E9 ^s4"), Gate::Ref);
    assert_eq!(gate_of(&mut w, "event E5 verb=cry agent=C1 cause=E5 ^s4"), Gate::Ref);
    // mention: only existing ones; the cursor does not move
    assert_eq!(gate_of(&mut w, "mention L9 ^s4"), Gate::Ref);
    let before = w.cursor().len();
    w.apply(&line("mention L2 C3 ^s4")).unwrap();
    w.apply(&line("loc L3 type=kingdom here=0 ^s4")).unwrap();
    assert_eq!(w.cursor().len(), before);
    // directed relationship: changing the direction without a cause — silent
    w.apply(&line("rel R1 kind=benefactor a=C2 b=C1 ^s4")).unwrap();
    assert_eq!(gate_of(&mut w, "rel R1 a=C1 b=C2 ^s4"), Gate::Silent);
    w.apply(&line("rel R1 a=C1 b=C2 cause=E4 ^s4")).unwrap();
    assert!(w.check_links().is_ok());
}

/// Two-way links v2 see a mismatch introduced bypassing the executor.
#[test]
fn v2_links_catch_injected_mismatch() {
    let base = world(2, BASE2);
    let mut w = base.clone();
    w.inject(id("C1"), vec![Delta::Add(SetF::Plans, id("P1"))]);
    assert_eq!(w.check_links().unwrap_err().gate, Gate::Link);
    let mut w = base.clone();
    w.inject(id("O1"), vec![Delta::Set(Field::At, Val::Known(V::Id(id("L1"))))]);
    w.inject(id("L1"), vec![Delta::Add(SetF::Objects, id("O1"))]);
    assert_eq!(w.check_links().unwrap_err().gate, Gate::Link, "a part, and lies separately");
    let mut w = base.clone();
    w.inject(id("C3"), vec![Delta::Add(SetF::Parts, id("O1"))]);
    assert_eq!(w.check_links().unwrap_err().gate, Gate::Link);
    let mut w = base;
    w.inject(id("B1"), vec![Delta::Add(SetF::Owners, id("C3"))]);
    assert_eq!(w.check_links().unwrap_err().gate, Gate::Link);
}

/// Format v2 is strict: several acts without repeats, `+` without repeats, a part without a place, closing an intention.
#[test]
fn v2_format_is_strict() {
    for bad in [
        "say C1 act=request+request ^s1",
        "say C1 act=request+threaten ^s1",
        "say C1 act=ask+agree+thank+greet ^s1",
        "event E1 verb=go agent=C1+C1 ^s1",
        "event E1 verb=go agent=C1+L1 ^s1",
        "event E1 verb=go neg=2 ^s1",
        "event E1 verb=go when=Night ^s1",
        "obj O1 type=body_part part=C1 at=L1 ^s1",
        "obj O1 type=food num=lots ^s1",
        "plan P1 kind=promise what=\"x\" ^s1",
        "plan P1 kind=vow who=C1 what=\"x\" ^s1",
        "plan P1 status=open by=E1 ^s1",
        "plan P1 status=kept who=C1 by=E1 ^s1",
        "plan P1 kind=promise status=kept who=C1 what=\"x\" ^s1",
        "believe B1 who=C1 ^s1",
        "believe B1 claim=\"x\" ^s1",
        "believe B1 by=E1 ^s1",
        "feel C1 ^s1",
        "feel C1 +hangry ^s1",
        "feel C1 happy ^s1",
        "feel C1 +sad -sad ^s1",
        "unset C1 name cause=E1 ^s1",
        "unset C1 at cause=E1 ^s1",
        "detach O1 to=O2 cause=E1 ^s1",
        "attach O1 to=L1 cause=E1 ^s1",
        "mention ^s1",
        "mention C1 C1 ^s1",
        "mention E1 ^s1",
        "set C1 feeling=happy,happy ^s1",
        "loc L1 type=room here=maybe ^s1",
    ] {
        assert!(parse_line(1, bad).is_err(), "format let through: {bad}");
    }
    let l = line("say C1+C2 to=C3+C4 act=request+promise cause=E1 about=P1 when=morning id=E2 ^s3");
    assert!(matches!(l.cmd, crate::lang::Cmd::Say { ref acts, ref to, .. } if acts.len() == 2 && to.len() == 2));
}

/// Natural-language answerer (ans.rs) on the v2 example: FairytaleQA-style questions (written here, not from
/// FairytaleQA) → answers from the state; a question whose answer is not in the state — «not in state».
#[test]
fn natural_questions_are_answered_from_state() {
    use crate::ans::{Src, answer};
    use crate::ftqa::Qa;
    let w = example2();
    let q = |text: &str, secs: &[u16]| Qa {
        story: "goose".into(),
        n: 0,
        attr: String::new(),
        ex: String::new(),
        ex2: String::new(),
        local: true,
        secs: secs.to_vec(),
        question: text.into(),
        a1: String::new(),
        a2: String::new(),
    };
    let hit = |text: &str, secs: &[u16], want: &[&str]| {
        let a = answer(&w, &q(text, secs));
        assert_eq!(a.src, Src::State, "{text} → none: {} [{}]", a.note, a.frame);
        for x in want {
            assert!(a.text.contains(x), "{text} → «{}» without «{x}» [{}; {}]", a.text, a.frame, a.note);
        }
    };
    hit("what did the goose promise ?", &[1], &["golden egg"]);
    hit("how did the wife feel when the miller found the golden egg ?", &[2], &["ashamed", "happy"]);
    hit("who drove the fox away ?", &[3], &["the miller", "his wife"]);
    hit("what did the fox bite off ?", &[3], &["lame wing"]);
    hit("what happened after the fox bit off the goose's wing ?", &[3], &["drove away"]);
    hit("what did the wife believe ?", &[1], &["will not bring a golden egg"]);
    hit("why did the wife give up her plan ?", &[2], &["kept its promise"]);
    hit("what did the wife plan to do ?", &[1], &["cook the goose"]);
    hit("what did the goose want ?", &[1], &["not be killed"]);
    hit("how many golden eggs did the goose lay ?", &[3], &["many"]);
    hit("where did the miller live ?", &[1], &["mill"]);
    // negative control: this is not in the state
    let a = answer(&w, &q("what color was the fox's tail ?", &[3]));
    assert!(a.src == Src::None || !a.text.contains("red"), "{}", a.text);
    let a = answer(&w, &q("how did the fox feel when the miller sang ?", &[3]));
    assert_eq!(a.src, Src::None, "the fox has no feelings: {} [{}]", a.text, a.note);
}

/// Real LLM v2 output («How an Old Man Lost His Wen», FairytaleQA test, run 2026-09-26: the wen
/// detached from the old man and attached to the neighbour) and its mutations: new gates are silent on the clean output and
/// turn red on the spoiled one.
#[test]
fn real_vmm_v2_output_and_its_mutations() {
    let cmds = include_str!("../data/cmds-v2-wen.txt");
    let n = 89;
    let (lines, errs) = parse_all(cmds);
    assert!(errs.is_empty(), "{errs:?}");
    let clean = run(text(n), &lines);
    assert!(clean.rejects.is_empty() && clean.uncovered.is_empty(), "{:?}", clean.rejects.first().map(|x| x.1.to_string()));
    // the wen: part of C1 → with the demon C5 (detach) → part of C8 (attach)
    let hist: Vec<String> = clean.world.history(id("O1"), Field::PartOf).iter().map(|(st, v)| format!("{v}/{}", st.by)).collect();
    assert_eq!(hist, ["C1/obj", "?/detach", "C8/attach"]);
    let first_gate = |edit: &dyn Fn(&str) -> Vec<String>| {
        let src: Vec<String> = cmds.lines().flat_map(edit).collect();
        let (lines, errs) = parse_all(&src.join("\n"));
        assert!(errs.is_empty(), "{errs:?}");
        let r = run(text(n), &lines);
        r.rejects.first().map(|(l, e)| (l.raw.clone(), e.gate))
    };
    let keep = |l: &str| vec![l.to_string()];
    // the wen was not detached — it cannot be attached to the neighbour: it is still part of the old man
    let (raw, g) = first_gate(&|l| if l.starts_with("detach O1 ") { vec![] } else { keep(l) }).unwrap();
    assert_eq!((raw.starts_with("attach O1"), g), (true, Gate::Continuity), "{raw}");
    // the part was «moved» with move instead of detach — link
    let (_, g) = first_gate(&|l| if l.starts_with("detach O1 ") { vec![l.replacen("detach", "move", 1)] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Link);
    // attached without a cause — silent change
    let (_, g) = first_gate(&|l| if l.starts_with("attach O1 ") { vec!["attach O1 to=C8 ^s84".into()] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Silent);
    // intention closed twice — continuity
    let (_, g) = first_gate(&|l| if l.starts_with("plan P6 status=kept") { vec![l.to_string(), l.to_string()] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Continuity);
    // «kept» by an event that did not happen (E21 neg=1) — continuity
    let (_, g) = first_gate(&|l| if l.starts_with("plan P6 status=kept") { vec![l.replace("by=E70", "by=E21")] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Continuity);
    // a known feeling changed without a cause — silent change
    let (_, g) = first_gate(&|l| if l.starts_with("feel C1 -afraid +hopeful") { vec![l.replace(" cause=E17", "")] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Silent);
    // an event with a cause that does not exist yet — reference
    let (_, g) = first_gate(&|l| if l.starts_with("event E4 ") { vec![l.replace("cause=E3", "cause=E99")] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Ref);
}

// ── question reader v2 (fact index, UD parsing, span, merging) ──────────────────────────────────

fn qa2(text: &str, secs: &[u16]) -> crate::ftqa::Qa {
    crate::ftqa::Qa {
        story: "goose".into(),
        n: 0,
        attr: String::new(),
        ex: String::new(),
        ex2: String::new(),
        local: true,
        secs: secs.to_vec(),
        question: text.into(),
        a1: String::new(),
        a2: String::new(),
    }
}

/// Question parsing with a UD tree: type, predicate, subject, anchor clause.
#[test]
fn reader_parses_questions_with_ud() {
    use crate::qframe::{QKind, parse};
    let f = parse("how did the wife feel when the miller found the golden egg ?");
    assert_eq!(f.kind, QKind::Feel);
    assert_eq!(f.main.subj.join(" "), "the wife");
    assert_eq!(f.mark.as_deref(), Some("when"));
    let s = f.sub.as_ref().expect("subordinate clause");
    assert_eq!((s.lemma.as_str(), s.subj.join(" ")), ("find", "the miller".to_string()));
    // a predicate the tagger does not see in lowercase («the princess *return* to»)
    let f = parse("where did assipattle and the princess return to ?");
    assert_eq!((f.kind, f.main.lemma.as_str()), (QKind::Where, "return"));
    assert_eq!(f.main.subj.join(" "), "assipattle and the princess");
    let f = parse("who was the youngest son ?");
    assert_eq!(f.kind, QKind::WhoIs);
    let f = parse("what happened to the eldest son because he made a false stroke with his axe ?");
    assert_eq!((f.kind, f.target.join(" ")), (QKind::HappenTo, "the eldest son".to_string()));
    assert_eq!(f.sub.as_ref().map(|c| c.lemma.as_str()), Some("make"));
    let f = parse("how many fairies did the king and queen find ?");
    assert_eq!((f.kind, f.whnp.join(" ")), (QKind::Num, "fairies".to_string()));
    let f = parse("what made the queen confused ?");
    assert_eq!(f.kind, QKind::Why);
}

/// Sentence span by the role of the question word (UD tree of the sentence).
#[test]
fn reader_cuts_spans_from_anchor_sentences() {
    use crate::span::{Want, extract};
    let a = crate::tree::annotator().expect("en model");
    let t = |s: &str| crate::tree::Tree::parse(a, s);
    let fr = crate::qframe::parse("where did the fox fling herself to take a little rest ?");
    let ql = crate::reader::qlemmas(&fr);
    let tr = t("and as she spoke she reached a little plot of grass , and flung herself under a tree to take a little rest .");
    let (s, _) = extract(&tr, Want::Where, &["fling".into()], &[], &fr, &ql).expect("place");
    assert_eq!(s, "under a tree");
    let fr = crate::qframe::parse("why was the man able to creep in easily ?");
    let ql = crate::reader::qlemmas(&fr);
    let tr = t("the hole was near the ground , so he crept in easily , and sat down .");
    let (s, _) = extract(&tr, Want::Why, &["creep".into()], &[], &fr, &ql).expect("cause");
    assert_eq!(s, "the hole was near the ground");
    let fr = crate::qframe::parse("how did the king feel when his wife became ill ?");
    let ql = crate::reader::qlemmas(&fr);
    let tr = t("the king was sorely troubled when he saw his precious bride so ill .");
    let (s, r) = extract(&tr, Want::Feel, &["feel".into()], &[], &fr, &ql).expect("feeling");
    assert_eq!((s.as_str(), r.as_str()), ("sorely troubled", "adj"));
    // an echo of the question is not an answer
    let fr = crate::qframe::parse("who drove the fox away ?");
    let ql = crate::reader::qlemmas(&fr);
    let tr = t("the fox was driven away .");
    assert!(extract(&tr, Want::Subj, &["drive".into()], &[], &fr, &ql).is_none());
}

/// Reader v2 on the v2 example (not FairytaleQA): the state when confident; otherwise the baseline sentence — with a reason.
/// Negative controls: without the index — exactly the baseline; the index of another story — the state answers less often and wrongly.
#[test]
fn reader_v2_answers_from_state_and_controls() {
    use crate::index::Index;
    use crate::reader::{RSrc, Tale, read, trees};
    let w = example2();
    let ix = Index::build(&w);
    let tr = trees(&w.text).expect("en model");
    let tale = Tale { fw: &w, ix: &ix, own: &w, tr: &tr };
    let hits: &[(&str, &[u16], &[&str])] = &[
        ("how did the wife feel when the miller found the golden egg ?", &[2], &["ashamed"]),
        ("who drove the fox away ?", &[3], &["miller"]),
        ("what did the fox bite off ?", &[3], &["lame wing"]),
        ("why did the wife give up her plan ?", &[2], &["kept its promise"]),
        ("how many golden eggs did the goose lay ?", &[3], &["many"]),
        ("where did the miller and his wife live ?", &[1], &["mill"]),
        ("what did the wife plan to do ?", &[1], &["cook"]),
    ];
    let mut n_state = 0;
    for (q, secs, want) in hits {
        let a = read(&tale, &qa2(q, secs));
        assert_ne!(a.src, RSrc::Sentence, "{q} → sentence: {} [{}] {}", a.reason, a.frame, a.fact);
        n_state += 1;
        for x in *want {
            assert!(a.text.contains(x), "{q} → «{}» without «{x}» [{}; {}]", a.text, a.reason, a.fact);
        }
        assert!(a.reason.starts_with("state"), "{}", a.reason);
    }
    // not in state: the answer is the baseline sentence, and the reason says so
    let a = read(&tale, &qa2("how did the fox feel when the miller sang ?", &[3]));
    assert_eq!(a.src, RSrc::Sentence, "{} {}", a.text, a.reason);
    assert!(a.reason.starts_with("sentence"));
    // control 1: without the index — exactly the baseline on every question
    let empty = Index::empty();
    let bare = Tale { fw: &w, ix: &empty, own: &w, tr: &tr };
    for (q, secs, _) in hits {
        let a = read(&bare, &qa2(q, secs));
        let (_, base) = crate::ans::baseline(&w, &qa2(q, secs));
        assert_eq!((a.src, a.text.as_str()), (RSrc::Sentence, base.as_str()), "{q}");
    }
    // control 2: facts of another story (the girl and the dog) — fewer state hits
    let other = example();
    let oix = Index::build(&other);
    let mixed = Tale { fw: &other, ix: &oix, own: &w, tr: &tr };
    let ok = hits.iter().filter(|(q, secs, want)| {
        let a = read(&mixed, &qa2(q, secs));
        a.src != RSrc::Sentence && want.iter().all(|x| a.text.contains(x))
    });
    assert!(ok.count() < n_state / 2, "foreign facts give the same answers — the reader does not take them from the state");
}

