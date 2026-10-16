//! v3 tests (non-linear narrative): the prompt example passes all gates (positive control) and gives
//! the intended state; linear narrative v1/v2 is a special case (the same world); negative controls
//! of the specification (a flashback without a time marker, a claim does not change the canonical world, a reveal does not
//! create a second event); every new command turns red with its own gate; strict format; the time axis is
//! a partial order.

use crate::lang::{Obs, parse_all, parse_line};
use crate::qa::Verdict;
use crate::qa3::{answer, verdict};
use crate::story::Order;
use crate::types::*;
use crate::vmm::{EXAMPLE_CMDS, EXAMPLE_TEXT, EXAMPLE2_CMDS, EXAMPLE2_TEXT, EXAMPLE3_CMDS, EXAMPLE3_TEXT};
use crate::world::{Gate, Text, World, run, run_v3};

fn text_of(t: &[(u16, &str)]) -> Text {
    Text { doc: 0, title: "t".into(), sents: t.iter().map(|(p, s)| (*p, s.to_string())).collect() }
}

fn id(s: &str) -> Id {
    Id::parse(s).unwrap()
}

pub(crate) fn example3() -> World {
    let (lines, errs) = parse_all(EXAMPLE3_CMDS);
    assert!(errs.is_empty(), "example v3, format: {errs:?}");
    let r = run_v3(text_of(EXAMPLE3_TEXT), &lines);
    assert!(r.rejects.is_empty(), "example v3, gates: {:?}", r.rejects.iter().map(|(l, e)| format!("{} — {e}", l.raw)).collect::<Vec<_>>());
    assert!(r.uncovered.is_empty(), "example v3, sentences without commands: {:?}", r.uncovered);
    r.world
}

/// A v3 world from lines (each must pass) on a text of n sentences.
fn world3(n: usize, cmds: &str) -> World {
    let (lines, errs) = parse_all(cmds);
    assert!(errs.is_empty(), "format: {errs:?}");
    let mut w = World::new_v3(Text { doc: 0, title: "t".into(), sents: (1..=n).map(|i| (1, format!("sentence {i} two years ago"))).collect() });
    for l in &lines {
        w.apply(l).unwrap_or_else(|e| panic!("{}: {e}", l.raw));
    }
    w
}

fn gate_of(w: &mut World, s: &str) -> Gate {
    let l = parse_line(1, s).expect("format").expect("line");
    match w.apply(&l) {
        Ok(()) => panic!("gate is silent: {s}"),
        Err(e) => e.gate,
    }
}

fn ask(w: &World, q: &str) -> String {
    let a = answer(w, q);
    assert!(a.from_state, "{q}: {}", a.text);
    a.text
}

#[test]
fn prompt_example_v3_passes_all_gates() {
    let w = example3();
    let st = w.story().unwrap();
    assert!(st.warnings.is_empty(), "warnings: {:?}", st.warnings);
    // frame: Tom tells Mary, level 1, its own world; frame events are level 1
    assert_eq!(st.frames.len(), 2);
    assert_eq!(st.frames[1].level, 1);
    assert_eq!(st.frames[1].narrator, Obs::C(id("C1")));
    assert_eq!(st.level_of(id("E8")), Some(1));
    assert_eq!(st.level_of(id("E14")), Some(0));
    // story axis: the broken vase (told at ^s2 after Mary arrives) is earlier than the arrival; the cat's jump is
    // after Tom left (past frame) and before Mary arrived; Tom left earlier than Mary came
    assert_eq!(st.time.order(id("E2"), id("E1")), Order::Before);
    assert_eq!(st.time.order(id("E15"), id("E1")), Order::Before);
    assert_eq!(st.time.order(id("E12"), id("E15")), Order::Before);
    assert_eq!(st.time.order(id("E12"), id("E1")), Order::Before, "inferred: E12 < E15 < E1");
    assert_eq!(st.time.order(id("E8"), id("E7")), Order::Before, "the past frame is before the telling");
    assert_eq!(st.time.order(id("E18"), id("E14")), Order::After);
    // the present is not overwritten by the past: Tom is in the shop (in the frame he went to the market), the vase is broken on the floor
    assert_eq!(w.now(id("C1")).unwrap().id(Field::At), Some(id("L2")));
    assert_eq!(w.now(id("O1")).unwrap().id(Field::At), Some(id("L2")));
    // what is new from the frame carried over: the milk is with the cat, the shelf is in the shop
    assert_eq!(w.now(id("O3")).unwrap().id(Field::At), Some(id("C2")));
    assert_eq!(w.now(id("L4")).unwrap().id(Field::Parent), Some(id("L2")));
    // Mary's claim: believes → doubts (E13) → is mistaken (reveal)
    let t = ask(&w, "claims who=char:mary about=E2");
    assert!(t.contains("believes ^s3") && t.contains("doubts ^s8") && t.contains("wrong ^s10 (reveal)"), "{t}");
    let t = ask(&w, "claims who=char:mary about=E2 at=^s8");
    assert!(t.contains("at ^s8: doubts"), "{t}");
    // reveal: the agent is the cat, the cause is the jump; the event is the same one (not a second one)
    let t = ask(&w, "truth ev:break");
    assert!(t.contains("agent=C2 Ginger") && t.contains("cause=E15"), "{t}");
    assert_eq!(w.events().iter().filter(|e| e.verb == "break").count(), 1);
    let t = ask(&w, "revealed E2");
    assert!(t.contains("^s10") && t.contains("K1"), "{t}");
    // knowledge: Mary knows since ^s10 (reveal), at ^s5 not yet; Tom since ^s9 (inferred); the reader since the telling ^s2
    assert!(ask(&w, "knows char:mary E2 at=^s5").contains("does not yet know"));
    assert!(ask(&w, "knows char:mary E2").contains("since ^s10"));
    assert!(ask(&w, "knows char:tom E2").contains("since ^s9"));
    assert!(ask(&w, "knows reader E2").contains("since ^s2"));
    // Mary's lie: an event in the lie branch, outside the narrative line; the baker believes it is true
    assert!(!st.is_real(id("E17")));
    assert!(!st.ev[&id("E17")].main);
    assert!(ask(&w, "branch E17").contains("branch lie"));
    // absolute time — only from the text
    assert!(ask(&w, "abs E8").contains("«At dawn» ^s6"));
    assert!(!answer(&w, "abs E2").from_state);
    // the frame's own cursor: the morning and the shelf are in the frame world's cursor, the root cursor stayed in the evening at the shop
    assert_eq!(st.frames[1].world.as_ref().unwrap().cursor().last().and_then(|c| c.time.clone()).as_deref(), Some("morning"));
    assert!(w.cursor().iter().all(|c| c.time.as_deref() != Some("morning")));
    // frames and levels
    assert!(ask(&w, "levels").contains("levels: 2"));
    assert!(ask(&w, "teller 1").contains("Old Tom → C3 Mary"));
    assert!(ask(&w, "level E10").contains("level 1"));
    // anachronies: the broken vase is told after Mary arrives, but happened earlier
    assert!(ask(&w, "analepses 10").contains("E2 break"));
    let v = verdict(&answer(&w, "order ev:break ev:come/char:mary"), "before");
    assert_eq!(v, Verdict::Hit);
}

/// Linear narrative is a special case: examples v1 and v2 on the v3 executor give the same md world.
#[test]
fn linear_v1_v2_is_special_case_of_v3() {
    for (t, c) in [(EXAMPLE_TEXT, EXAMPLE_CMDS), (EXAMPLE2_TEXT, EXAMPLE2_CMDS)] {
        let (lines, errs) = parse_all(c);
        assert!(errs.is_empty());
        let (a, b) = (run(text_of(t), &lines), run_v3(text_of(t), &lines));
        assert!(a.rejects.is_empty() && b.rejects.is_empty());
        assert_eq!(crate::md::world_md(&a, &[], lines.len()), crate::md::world_md(&b, &[], lines.len()));
        // on the line — all events that happened; told in sequence — happened in sequence
        let st = b.world.story().unwrap();
        let main: Vec<Id> = b.world.events().iter().filter(|e| e.happened() && e.x.placement().is_none()).map(|e| e.id).collect();
        assert_eq!(st.frames[0].main, main);
        for p in main.windows(2) {
            assert_eq!(st.time.order(p[0], p[1]), Order::Before);
        }
    }
}

const BAKER: &str = "loc L1 type=house name=\"Baker Street\" ^s1
loc L2 type=house name=\"Stoke Moran\" here=0 ^s1
char C1 type=human class=lady name=\"Helen\" at=L1 gender=female ^s1
char C2 type=human class=doctor name=\"Watson\" at=L1 gender=male ^s1
char C3 type=human class=lady name=\"Julia\" gender=female ^s1
event E1 verb=arrive agent=C1 to=L1 ^s1
say C1 to=C2 act=narrate means=\"my sister died\" id=E2 ^s2";

/// Negative control of the specification: a flashback without a time marker — the teleportation gate on the story axis.
#[test]
fn flashback_without_time_mark_is_teleport() {
    // without a marker: Helen is on Baker Street, and a past event «at Stoke Moran» becomes adjacent to the present
    let mut w = world3(6, BAKER);
    assert_eq!(gate_of(&mut w, "event E3 verb=hear agent=C1 at=L2 ^s3"), Gate::Teleport);
    // with a past frame: the adjacent states are in the frame world, not the present ones
    let ok = world3(6, &format!("{BAKER}\nframe open F1 narrator=C1 to=C2 level=1 src=E2 ^s3\nmove C1 to=L2 ^s3\nevent E3 verb=hear agent=C1 at=L2 ^s3\nset C3 life=dead cause=E3 ^s4\nframe close F1 ^s4"));
    let st = ok.story().unwrap();
    assert_eq!(st.time.order(id("E3"), id("E1")), Order::Unknown, "the testimony is before the telling, but not necessarily before the arrival");
    assert_eq!(st.time.order(id("E3"), id("E2")), Order::Before);
    assert_eq!(ok.now(id("C1")).unwrap().id(Field::At), Some(id("L1")), "the present location is not overwritten by a memory");
    assert_eq!(ok.now(id("C3")).unwrap().get(Field::Life), Val::Known(V::Tag(LIFE.tag("dead").unwrap())), "the permanent (death) carried over from the past");
    // with explicit placement before= — also not teleportation
    world3(6, &format!("{BAKER}\nevent E3 verb=hear agent=C1 at=L2 before=E1 ^s3"));
    // a neighbour on the story axis inside a frame: in the frame Helen is already at Stoke Moran — an event on Baker Street without move is red
    let mut w = world3(6, &format!("{BAKER}\nframe open F1 narrator=C1 to=C2 level=1 src=E2 ^s3\nmove C1 to=L2 ^s3"));
    assert_eq!(gate_of(&mut w, "event E3 verb=sleep agent=C1 at=L1 ^s4"), Gate::Teleport);
}

/// Negative control of the specification: Watson's claim does not change the canonical world.
#[test]
fn claim_does_not_change_canonical_world() {
    let base = format!("{BAKER}\nevent E3 verb=die patient=C3 at=L2 before=E1 ^s2\nset C3 life=dead cause=E3 ^s2");
    let w0 = world3(6, &base);
    let w1 = world3(6, &format!("{base}\nclaim K1 who=C2 about=E3 status=believes interp=\"she died of fear\" ^s3"));
    for reg in [Reg::L, Reg::O, Reg::C, Reg::R, Reg::P, Reg::B] {
        for e in w0.ents(reg) {
            assert_eq!(e.now(), w1.now(e.id).unwrap(), "{} changed by a claim", e.id);
            assert_eq!(e.states().len(), w1.ent(e.id).unwrap().states().len());
        }
    }
    assert_eq!(w0.events().len(), w1.events().len());
    // a claim recorded as an event in a belief branch also does not change the world
    let mut w = world3(6, &format!("{base}\nevent E4 verb=kill agent=C2 patient=C3 before=E1 ^s3\nbranch E4 belief ^s3"));
    assert_eq!(gate_of(&mut w, "set C1 feeling=afraid cause=E4 ^s4"), Gate::Branch);
    // and an event that already changed the world cannot be declared a dream
    let mut w = world3(6, &base);
    assert_eq!(gate_of(&mut w, "branch E3 dream ^s3"), Gate::Branch);
}

/// Negative control of the specification: a reveal does not create a second event.
#[test]
fn reveal_does_not_create_second_event() {
    let base = format!(
        "{BAKER}\nchar C4 type=human class=doctor name=\"Roylott\" ^s2\nevent E3 verb=die patient=C3 at=L2 before=E1 ^s2\nset C3 life=dead cause=E3 ^s2\nclaim K1 who=C1 about=E3 status=believes interp=\"she died of fear\" ^s2"
    );
    let w0 = world3(6, &base);
    let w1 = world3(6, &format!("{base}\nreveal E3 to=C2 agent=C4 wrong=K1 interp=\"killed by a snake\" ^s5"));
    assert_eq!(w0.events().len(), w1.events().len(), "reveal does not add events");
    assert_eq!(w0.story().unwrap().ev.len(), w1.story().unwrap().ev.len());
    assert!(ask(&w1, "claims who=char:helen").contains("wrong ^s5 (reveal)"));
    // revealing a non-existent event — reference; a second death — identity
    let mut w = world3(6, &base);
    assert_eq!(gate_of(&mut w, "reveal E9 interp=\"murder\" ^s5"), Gate::Ref);
    // mutation: the solution recorded as a new «murder» event — a second death of the same heroine
    let mut w = world3(6, &format!("{base}\nevent E4 verb=kill agent=C4 patient=C3 at=L2 before=E1 ^s5"));
    assert_eq!(gate_of(&mut w, "set C3 life=dead cause=E4 ^s5"), Gate::Identity);
    // a reveal does not overwrite a role that is already in the event
    let mut w = world3(6, &format!("{base}\nreveal E3 agent=C4 ^s5"));
    assert_eq!(gate_of(&mut w, "reveal E3 agent=C2 ^s6"), Gate::Identity);
}

/// Every new v3 command turns red with its own gate.
#[test]
fn v3_gates_are_not_mute() {
    let base = format!("{BAKER}\nevent E3 verb=die patient=C3 at=L2 before=E1 ^s2\nclaim K1 who=C1 about=E3 status=believes interp=\"fear\" ^s2");
    let w = || world3(8, &base);
    // frames: level, closing the wrong one, the root, continuation with another narrator, a non-existent narrator
    assert_eq!(gate_of(&mut w(), "frame open F1 narrator=C1 level=2 ^s3"), Gate::Frame);
    assert_eq!(gate_of(&mut w(), "frame open F1 narrator=C9 level=1 ^s3"), Gate::Ref);
    let mut x = world3(8, &format!("{base}\nframe open F1 narrator=C1 level=1 ^s3\nframe open F2 narrator=C2 level=2 ^s3"));
    assert_eq!(gate_of(&mut x, "frame close F1 ^s4"), Gate::Frame);
    let mut x = world3(8, &format!("{base}\nframe open F1 narrator=C1 level=0 ^s3"));
    assert_eq!(gate_of(&mut x, "frame close F1 ^s4"), Gate::Frame);
    assert_eq!(gate_of(&mut x, "frame open F2 narrator=C2 level=0 ^s4"), Gate::Frame);
    let mut x = world3(8, &format!("{base}\nframe open F1 narrator=C1 level=1 ^s3\nframe close F1 ^s3"));
    assert_eq!(gate_of(&mut x, "frame open F1 narrator=C2 level=1 ^s4"), Gate::Frame);
    assert_eq!(gate_of(&mut w(), "frame close F7 ^s4"), Gate::Ref);
    // claims: about a non-existent event; change without by=; a non-existent observer
    assert_eq!(gate_of(&mut w(), "claim K2 who=C2 about=E9 status=doubts interp=\"x\" ^s3"), Gate::Ref);
    assert_eq!(gate_of(&mut w(), "claim K1 status=wrong ^s3"), Gate::Silent);
    assert_eq!(gate_of(&mut w(), "claim K2 who=C9 about=E3 status=doubts interp=\"x\" ^s3"), Gate::Ref);
    assert_eq!(gate_of(&mut w(), "claim K1 who=C2 about=E3 status=doubts interp=\"x\" ^s3"), Gate::Ref);
    // time: a cycle in the partial order; absolute time not from the text; different worlds
    assert_eq!(gate_of(&mut w(), "time E1 BEFORE E3 ^s3"), Gate::Time);
    let mut x = world3(8, &format!("{base}\nevent E4 verb=wait agent=C2 during=E1 ^s3"));
    assert_eq!(gate_of(&mut x, "time E4 BEFORE E1 ^s4"), Gate::Time);
    assert_eq!(gate_of(&mut w(), "time E3 at=\"in 1881\" ^s3"), Gate::Time);
    world3(8, &format!("{base}\ntime E3 at=\"two years ago\" ^s3"));
    let mut x = world3(8, &format!("{base}\nframe open F1 narrator=C1 level=1 world=new ^s3\nchar C5 type=human class=merchant ^s3\nevent E4 verb=travel agent=C5 ^s3"));
    assert_eq!(gate_of(&mut x, "time E4 BEFORE E1 ^s4"), Gate::Scope);
    // a story within a story does not change the listener's world: a character of the tale is not a participant in the parent frame's events
    let mut x = world3(8, &format!("{base}\nframe open F1 narrator=C1 level=1 world=new ^s3\nchar C5 type=human class=merchant ^s3\nevent E4 verb=travel agent=C5 ^s3\nframe close F1 ^s4"));
    assert_eq!(gate_of(&mut x, "event E5 verb=greet agent=C5 ^s5"), Gate::Ref);
    assert_eq!(gate_of(&mut x, "set C1 feeling=happy cause=E4 ^s5"), Gate::Scope);
    // branch: a non-existent event; knowledge from a future sentence; knowledge of a lie as a fact
    assert_eq!(gate_of(&mut w(), "branch E9 dream ^s3"), Gate::Ref);
    assert_eq!(gate_of(&mut w(), "know who=C2 fact=E3 from=^s7 ^s3"), Gate::Anchor);
    let mut x = world3(8, &format!("{base}\nevent E4 verb=poison agent=C2 patient=C3 before=E1 ^s3\nbranch E4 lie ^s3"));
    assert_eq!(gate_of(&mut x, "know who=C1 fact=E4 from=^s3 ^s4"), Gate::Branch);
    // reveal: a claim about another event
    let mut x = world3(8, &format!("{base}\nevent E4 verb=wait agent=C2 ^s3\nclaim K2 who=C2 about=E4 status=doubts interp=\"x\" ^s3"));
    assert_eq!(gate_of(&mut x, "reveal E3 wrong=K2 ^s4"), Gate::Ref);
    // revealing a time that contradicts what is known
    assert_eq!(gate_of(&mut w(), "reveal E3 after=E1 ^s4"), Gate::Time);
    // the dead do not act in the present
    let mut x = world3(8, &format!("{base}\nset C3 life=dead cause=E3 ^s3"));
    assert_eq!(gate_of(&mut x, "event E4 verb=speak agent=C3 ^s4"), Gate::Continuity);
}

/// Format v3 is strict.
#[test]
fn v3_format_is_strict() {
    for bad in [
        "frame open F1 narrator=C1 ^s1",
        "frame open F1 level=1 ^s1",
        "frame shut F1 ^s1",
        "frame close F1 level=1 ^s1",
        "frame open F1 narrator=reader level=1 ^s1",
        "frame open F1 narrator=C1 level=1 world=new time=past ^s1",
        "frame open F1 narrator=C1 level=1 branch=real ^s1",
        "claim K1 who=C1 about=E1 status=thinks interp=\"x\" ^s1",
        "claim K1 who=C1 about=E1 status=believes ^s1",
        "claim K1 who=C1 status=believes interp=\"x\" ^s1",
        "claim K1 status=wrong interp=\"x\" by=E2 ^s1",
        "claim C1 who=C1 about=E1 status=believes interp=\"x\" ^s1",
        "time E1 LATER E2 ^s1",
        "time E1 BEFORE E1 ^s1",
        "time E1 ^s1",
        "time E1 BEFORE E2 at=\"x\" ^s1",
        "branch E1 fantasy ^s1",
        "branch C1 lie ^s1",
        "know who=C1 fact=E1 ^s1",
        "know who=C1 fact=E1 from=12 ^s1",
        "know who=narrator fact=E1 from=^s1 ^s1",
        "know who=C1 fact=E1 from=^s1 how=dreamt ^s1",
        "reveal E1 ^s1",
        "reveal E1 wrong=K1 right=K1 ^s1",
        "reveal E1 cause=E1 ^s1",
        "reveal E1 before=E2 after=E3 ^s1",
        "reveal E1 to=narrator interp=\"x\" ^s1",
        "event E1 verb=go before=E2 after=E3 ^s1",
        "mention E1 ^s1",
    ] {
        assert!(parse_line(1, bad).is_err(), "format let through: {bad}");
    }
    for good in [
        "frame open F1 narrator=C1 to=C2+reader level=1 world=same time=past before=E3 src=E2 ^s1",
        "frame open F2 narrator=narrator level=1 world=new ^s1",
        "frame open F3 narrator=C1 level=2 branch=dream ^s1",
        "frame close F1 ^s2",
        "claim K1 who=reader about=E1 status=doubts interp=\"an accident\" from=E2 ^s1",
        "claim K1 status=wrong by=E5 ^s1",
        "time E1 SAME E2 ^s1",
        "time E1 at=\"early in April\" ^s1",
        "branch E1 if by=E2 ^s1",
        "know who=reader fact=K1 from=^s1 how=revealed by=E2 ^s1",
        "reveal E1 to=C1+reader by=E2 interp=\"x\" wrong=K1+K2 right=K3 agent=C4 instr=O2 cause=E3 before=E4 ^s1",
        "event E1 verb=go agent=C1 during=E2 ^s1",
    ] {
        assert!(parse_line(1, good).is_ok(), "format rejected: {good}: {:?}", parse_line(1, good).err());
    }
}

/// The time axis is a partial order: transitivity, «during», «≈ simultaneously», unknown, closure.
#[test]
fn time_is_a_partial_order() {
    let w = world3(
        8,
        "char C1 type=human class=man ^s1
event E1 verb=wake agent=C1 ^s1
event E2 verb=eat agent=C1 ^s2
event E3 verb=dream agent=C1 before=E1 ^s3
event E4 verb=sing agent=C1 during=E2 ^s4
event E5 verb=rain same=E2 ^s5
event E6 verb=build agent=C1 after=E1 ^s6",
    );
    let st = w.story().unwrap();
    assert_eq!(st.time.order(id("E1"), id("E2")), Order::Before);
    assert_eq!(st.time.order(id("E3"), id("E2")), Order::Before, "transitively: E3 < E1 < E2");
    assert_eq!(st.time.order(id("E4"), id("E2")), Order::During);
    assert_eq!(st.time.order(id("E2"), id("E4")), Order::Contains);
    assert_eq!(st.time.order(id("E5"), id("E2")), Order::Overlap);
    assert_eq!(st.time.order(id("E6"), id("E2")), Order::Unknown, "after E1 — but relative to E2 unknown");
    assert_eq!(st.time.order(id("E3"), id("E4")), Order::Before);
    let cl = st.time.closure();
    let n = |p| st.time.get(p).unwrap();
    use crate::story::Pt;
    assert!(cl.lt(n(Pt::E(id("E3"))), n(Pt::S(id("E2")))));
    assert!(!cl.lt(n(Pt::E(id("E6"))), n(Pt::S(id("E2")))) && !cl.lt(n(Pt::E(id("E2"))), n(Pt::S(id("E6")))));
    assert!(ask(&w, "sort E2 E1 E3").starts_with("E3 dream"));
}

/// Real LLM v3 output (Scheherazade's frame and the beginning of the tale of the merchant and the genie, «The Arabian Nights»,
/// retold by Andrew Lang, public domain) is clean; mutations turn red with their own gates.
#[test]
fn real_vmm_v3_output_and_its_mutations() {
    let cmds = include_str!("../data/cmds-v3-merchant.txt");
    let story = include_str!("../data/v3-merchant.tsv");
    let text = || Text {
        doc: 128,
        title: "merchant".into(),
        sents: story.lines().filter(|l| !l.starts_with('#')).map(|l| {
            let c: Vec<&str> = l.splitn(3, '\t').collect();
            (c[0].parse().unwrap(), c[2].to_string())
        }).collect(),
    };
    let (lines, errs) = parse_all(cmds);
    assert!(errs.is_empty(), "{errs:?}");
    let clean = run_v3(text(), &lines);
    assert!(clean.rejects.is_empty() && clean.uncovered.is_empty(), "{:?} {:?}", clean.rejects.first().map(|x| x.1.to_string()), clean.uncovered);
    let w = &clean.world;
    let st = w.story().unwrap();
    // Scheherazade's frame — level 1, a new world: the merchant and the genie live only there
    assert_eq!(st.frames[1].level, 1);
    assert_eq!(st.frames[1].kind, crate::story::FWorld::New);
    assert!(w.ent(id("C10")).is_none() && st.frames[1].world.as_ref().unwrap().ent(id("C10")).is_some());
    // the son's murder is one event: earlier than the genie's appearance, revealed (a date stone, during the meal)
    assert_eq!(st.time.order(id("E80"), id("E79")), Order::Before);
    assert_eq!(w.events().iter().filter(|e| e.verb == "kill").count(), 1);
    assert!(ask(w, "truth E80").contains("instr=O9"));
    assert!(ask(w, "claims who=char:merchant about=E80").contains("wrong ^s85 (reveal)"));
    assert!(ask(w, "learns char:merchant E80 at=^s80").contains("does not yet know the truth"));
    // the murder of the genie's son and Scheherazade's wedding are in different worlds: there is no order between them
    assert_eq!(st.time.order(id("E80"), id("E68")), Order::After, "in the tale's world: during the meal (E77) — after the start of the journey (E68)");
    let wedding = w.events().iter().find(|e| e.anchors.contains(&56)).unwrap().id;
    assert_eq!(st.time.order(id("E80"), wedding), Order::OtherWorld);

    let first_gate = |edit: &dyn Fn(&str) -> Vec<String>| {
        let src: Vec<String> = cmds.lines().flat_map(edit).collect();
        let (lines, errs) = parse_all(&src.join("\n"));
        assert!(errs.is_empty(), "{errs:?}");
        let r = run_v3(text(), &lines);
        r.rejects.first().map(|(l, e)| (l.raw.clone(), e.gate))
    };
    let keep = |l: &str| vec![l.to_string()];
    // the claim removed — the canonical world is the same (claims do not change it)
    let src: Vec<String> = cmds.lines().filter(|l| !l.starts_with("claim ")).map(|l| l.replace(" right=K1", "").replace(" wrong=K2", "")).collect();
    let (lines2, _) = parse_all(&src.join("\n"));
    let noclaims = run_v3(text(), &lines2);
    assert!(noclaims.rejects.is_empty() && noclaims.world.ents(Reg::K).count() == 0 && w.ents(Reg::K).count() == 3);
    let fw = |x: &World| x.story().unwrap().frames[1].world.as_ref().unwrap().ents(Reg::C).map(|e| (e.id, e.now())).collect::<Vec<_>>();
    assert_eq!(fw(w), fw(&noclaims.world));
    assert_eq!(w.ents(Reg::C).map(|e| (e.id, e.now())).collect::<Vec<_>>(), noclaims.world.ents(Reg::C).map(|e| (e.id, e.now())).collect::<Vec<_>>());
    // the solution recorded as a new «murder» event — a second death of the son: identity
    let (raw, g) = first_gate(&|l| {
        if l.starts_with("reveal E80 ") {
            vec!["event E999 verb=kill agent=C10 patient=C12 instr=O9 before=E79 ^s85".into(), "set C12 life=dead cause=E999 ^s85".into()]
        } else if l.starts_with("claim K3 ") {
            vec![]
        } else {
            keep(l)
        }
    })
    .unwrap();
    assert_eq!(g, Gate::Identity, "{raw}");
    // time that contradicts what is known: the murder after the genie's appearance — a cycle
    let (_, g) = first_gate(&|l| if l.starts_with("reveal E80 ") { vec![l.to_string(), "time E80 AFTER E79 ^s85".into()] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Time);
    // an event of the tale is a cause of changes in the listener's world: world boundary
    let (_, g) = first_gate(&|l| if l.starts_with("frame close F1") { vec![l.to_string(), "feel C2 +horrified cause=E80 ^s93".into()] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Scope);
    // a frame at the wrong level
    let (_, g) = first_gate(&|l| if l.starts_with("frame open F1 ") { vec![l.replace("level=1", "level=2")] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Frame);
    // a murder that already changed the world declared a lie — branch
    let (_, g) = first_gate(&|l| if l.starts_with("frame close F1") { vec!["branch E80 lie ^s92".into(), l.to_string()] } else { keep(l) }).unwrap();
    assert_eq!(g, Gate::Branch);
}

/// Narrative level — by narrators (Genette): a memory of the same narrator does not open a new level.
#[test]
fn narrative_level_counts_narrators_not_frames() {
    let w = world3(
        8,
        "char C1 type=human class=doctor name=\"Watson\" ^s1
char C2 type=human class=lady name=\"Helen\" ^s1
frame open F1 narrator=C1 level=0 ^s1
say C1 act=narrate id=E1 ^s1
frame open F2 narrator=C1 level=1 before=E1 ^s2
event E2 verb=arrive agent=C2 ^s2
say C2 to=C1 act=narrate id=E3 ^s3
frame open F3 narrator=C2 to=C1 level=2 before=E2 src=E3 ^s3
event E4 verb=die patient=C2 neg=1 ^s4
frame close F3 ^s5",
    );
    let st = w.story().unwrap();
    assert_eq!((st.depth(), st.narr_depth()), (3, 2));
    assert_eq!((0..st.frames.len()).map(|i| st.narr_level(i)).collect::<Vec<_>>(), vec![0, 0, 1]);
    assert_eq!(st.time.order(id("E2"), id("E1")), Order::Before, "the memoir is before the writing");
}

/// Frame-branch (dream): its own copy world, events in the `dream` branch, outside the line; the canonical world does not change.
#[test]
fn dream_frame_does_not_change_canonical_world() {
    let base = "loc L1 type=house ^s1
loc L2 type=sky here=0 ^s1
char C1 type=human class=man name=\"Tom\" at=L1 ^s1
event E1 verb=sleep agent=C1 at=L1 ^s1
frame open F1 narrator=C1 level=1 branch=dream ^s2
event E2 verb=fly agent=C1 to=L2 ^s2
move C1 to=L2 cause=E2 ^s2
frame close F1 ^s3";
    let mut w = world3(6, base);
    let st = w.story().unwrap();
    assert_eq!(st.ev[&id("E2")].branch.as_str(), "dream");
    assert!(!st.ev[&id("E2")].main && st.frames[0].main == vec![id("E1")]);
    assert_eq!(w.now(id("C1")).unwrap().id(Field::At), Some(id("L1")), "flew in the dream, but actually at home");
    assert_eq!(st.frames[1].world.as_ref().unwrap().now(id("C1")).unwrap().id(Field::At), Some(id("L2")));
    // a dream is not a cause of changes in the canonical world
    assert_eq!(gate_of(&mut w, "move C1 to=L2 cause=E2 ^s4"), Gate::Scope);
}
