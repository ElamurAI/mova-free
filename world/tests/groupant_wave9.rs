//! Supplied-input joins; external parser, teacher and live courses are not invoked.
use std::collections::BTreeSet;
use world::story::{Order, Pt};
use world::types::{Id, Reg};
use world::{ans, ftqa, knowledge, report, schoolbench, schooltools};

fn lesson(n: usize, stars: usize, options: &str) -> String {
    format!(
        "course: fixture\nlesson: {n}\nstars: {stars}\nfact: A fixture exists.\nexercise: Which? | {options}\ntest: Which? | {options}\n"
    )
}
fn world(program: &str) -> world::world::World {
    let (lines, errors) = world::lang::parse_all(program);
    assert!(errors.is_empty(), "{errors:?}");
    let text = world::world::Text {
        doc: 7,
        title: "fixture".into(),
        sents: (1..=6).map(|n| (1, format!("sentence {n}"))).collect(),
    };
    let mut w = world::world::World::new_v3(text);
    for line in lines {
        w.apply(&line)
            .unwrap_or_else(|e| panic!("{}: {e}", line.raw));
    }
    w
}
fn ordered_events(w: &world::world::World, a: Id, b: Id) -> Order {
    w.story().unwrap().time.order(a, b)
}

#[test]
fn marked_option_identity_survives_tools_and_lesson_parser() {
    for right in ["a", "medium", "long answer"] {
        for position in 0..3 {
            let mut choices = vec!["wrong".to_owned(), "other wrong".to_owned()];
            choices.insert(position, format!("*{right}"));
            let text = lesson(1, 2, &choices.join(" | "));
            let l = knowledge::parse(&text, "fixture.lesson").unwrap();
            let line = text.lines().find(|s| s.starts_with("exercise:")).unwrap();
            let (_, fields) = schooltools::option_line(line).unwrap();
            let canonical = schooltools::canon(&fields);
            assert_eq!(l.exercises[0].0, canonical[0]);
            assert_eq!(l.exercises[0].1, right);
            assert_eq!(l.exercises[0].1, canonical[1]);
            assert_eq!(l.exercises[0].2, canonical[2..]);
            let tally = schooltools::tally(&text);
            assert_eq!(tally.n, l.exercises.len() + l.tests.len());
            assert_eq!(tally.l, if right.len() > 11 { 2 } else { 0 });
            assert_eq!(tally.s, if right.len() < 5 { 2 } else { 0 });
        }
    }
    let bad = lesson(1, 1, "*a | *b");
    assert!(knowledge::parse(&bad, "fixture.lesson").is_err());
    // Tools still recognize the line: recognition is not lesson validation.
    assert!(schooltools::option_line("exercise: Which?|*a|*b").is_some());
}

#[test]
fn parsed_lesson_strata_join_nested_chronological_selection() {
    let lessons = (1..=27)
        .map(|n| knowledge::parse(&lesson(n, n % 3, "*yes | no"), "fixture.lesson").unwrap())
        .collect::<Vec<_>>();
    for seed in 0..8 {
        let quick = schoolbench::selection(&lessons, schoolbench::Tier::Quick, seed);
        let medium = schoolbench::selection(&lessons, schoolbench::Tier::Medium, seed);
        let full = schoolbench::selection(&lessons, schoolbench::Tier::Full, seed);
        assert_eq!(full, (0..27).collect::<Vec<_>>());
        for (selected, per_stratum) in [(&quick, 2), (&medium, 3)] {
            assert!(selected.windows(2).all(|w| w[0] < w[1]));
            for stars in 0..3 {
                assert_eq!(
                    selected
                        .iter()
                        .filter(|&&i| lessons[i].stars == stars)
                        .count(),
                    per_stratum
                );
            }
            for &i in selected {
                assert_eq!(lessons[i].n, i + 1);
                assert_eq!(lessons[i].exercises[0].1, "yes");
            }
        }
        assert!(quick.iter().all(|i| medium.contains(i)));
    }
    assert!(schoolbench::selection(&[], schoolbench::Tier::Quick, 0).is_empty());
}

#[test]
fn narrative_registration_joins_time_queries_and_atomic_rejection() {
    let mut w = world(
        "char C1 type=human name=Ada ^s1\nevent E1 verb=walk agent=C1 ^s1\nevent E2 verb=walk agent=C1 ^s2\nevent E3 verb=walk agent=C1 ^s3",
    );
    let ids = (1..=3).map(|n| Id::new(Reg::E, n)).collect::<Vec<_>>();
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                assert_eq!(
                    ordered_events(&w, ids[i], ids[j]),
                    if i < j { Order::Before } else { Order::After }
                );
            }
        }
    }
    let before = w.story().unwrap().time.edges.len();
    let (lines, errors) = world::lang::parse_all("time E3 BEFORE E1 ^s4");
    assert!(errors.is_empty());
    assert_eq!(
        w.apply(&lines[0]).unwrap_err().gate,
        world::world::Gate::Time
    );
    assert_eq!(w.story().unwrap().time.edges.len(), before);
    assert_eq!(ordered_events(&w, ids[0], ids[2]), Order::Before);
    let st = w.story().unwrap();
    let closure = st.time.closure();
    assert!(closure.lt(
        st.time.get(Pt::E(ids[0])).unwrap(),
        st.time.get(Pt::S(ids[2])).unwrap()
    ));
    assert_eq!(st.frames[0].main, ids);
}

#[test]
fn nested_new_world_preserves_lookup_and_axis_separation() {
    let w = world(
        "char C1 type=human name=Ada ^s1\nevent E1 verb=walk agent=C1 ^s1\nframe open F1 narrator=C1 level=1 world=new ^s2\nchar C2 type=human name=Bea ^s2\nevent E2 verb=walk agent=C2 ^s2\nframe close F1 ^s3",
    );
    let st = w.story().unwrap();
    assert_eq!(world::narr::world_frame(st, 1), 1);
    assert_eq!(w.worlds().len(), 2);
    assert_eq!(w.event(Id::new(Reg::E, 2)).unwrap().world, st.frames[1].wid);
    assert!(w.ent(Id::new(Reg::C, 2)).is_none());
    assert!(w.ent_any(Id::new(Reg::C, 2)).is_some());
    assert!(w.event_any(Id::new(Reg::E, 2)).is_some());
    assert_eq!(
        ordered_events(&w, Id::new(Reg::E, 1), Id::new(Reg::E, 2)),
        Order::OtherWorld
    );
    assert_eq!(st.stack, vec![0]);
    assert!(!st.frames[1].is_open());
}

#[test]
fn fairytale_question_identity_joins_report_and_manual_verdicts() {
    let dir = std::env::temp_dir().join(format!("mova-wave9-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("fixture.tsv");
    let qs = (1..=4)
        .map(|n| ftqa::Qa {
            story: "fixture".into(),
            n,
            attr: "character".into(),
            ex: "explicit".into(),
            ex2: "".into(),
            local: true,
            secs: vec![1],
            question: format!("Question {n}?"),
            a1: "Ada".into(),
            a2: "".into(),
        })
        .collect::<Vec<_>>();
    std::fs::write(&path, ftqa::qa_tsv(&qs.iter().collect::<Vec<_>>())).unwrap();
    let loaded = ftqa::read_qa(&path).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    let rows = loaded
        .into_iter()
        .map(|q| report::Row {
            a: ans::Answer {
                frame: "who".into(),
                src: ans::Src::State,
                text: q.a1.clone(),
                note: "supplied".into(),
                anchor: None,
            },
            q,
            r_slm: 1.0,
            r_anchor: 1.0,
            base: "Ada".into(),
            r_base: 1.0,
        })
        .collect::<Vec<_>>();
    let excluded = BTreeSet::from([("fixture".to_owned(), 2)]);
    let selected = report::sample(&rows, 10, "fixture", &excluded);
    assert_eq!(
        selected.iter().map(|r| r.q.n).collect::<BTreeSet<_>>(),
        BTreeSet::from([1, 3, 4])
    );
    let verdicts = report::read_verdicts(
        "fixture\t1\t1\t0\t1\tone\nfixture\t3\t0\t1\t1\tthree\nwrong-story\t4\t1\t1\t1\twrong identity",
    );
    let md = report::manual_md(&selected, &verdicts);
    assert!(
        md.contains("| character | 2 | 50% | 0 | 100% | 50% |"),
        "{md}"
    );
    let all = report::report(&rows);
    assert!(
        all.contains("| **all** | 4 | 1.000 | 1.000 | 1.000 | 1.000 | 0% |"),
        "{all}"
    );
    assert_eq!(report::rows_tsv(&rows).lines().count(), 5);
}
