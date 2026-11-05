//! Supplied-input Mova joins: no external parser, teacher, live courses or hot-state I/O.
use en::annotate::Word;
use en::gram::{Feats, Rel, Tag, UPos};
use prag::{agree, frames, invariants, lens, roles};
use std::collections::BTreeMap;
use world::{domains, lessons, primers, qa, qa3, school};

fn word(form: &str, lemma: &str, upos: UPos, head: usize, rel: Rel) -> Word {
    Word {
        form: form.into(),
        lemma: lemma.into(),
        upos,
        head,
        rel,
        tag: if upos == UPos::VERB { Tag::VB } else { Tag::NN },
        feats: Feats::default(),
    }
}
fn sentence(name: &str, object: &str) -> Vec<Word> {
    vec![
        word(name, &name.to_lowercase(), UPos::PROPN, 2, Rel::Nsubj),
        word("carries", "carry", UPos::VERB, 0, Rel::Root),
        word(object, object, UPos::NOUN, 2, Rel::Obj),
    ]
}
fn supplied_world(program: &str) -> world::world::World {
    let (lines, errors) = world::lang::parse_all(program);
    assert!(errors.is_empty(), "{errors:?}");
    let mut w = world::world::World::new_v3(world::world::Text {
        doc: 7,
        title: "fixture".into(),
        sents: (1..=6).map(|n| (1, format!("sentence {n}"))).collect(),
    });
    for line in lines {
        w.apply(&line)
            .unwrap_or_else(|e| panic!("{}: {e}", line.raw));
    }
    w
}

#[test]
fn bounded_lesson_expressions_agree_with_school_operations() {
    for a in 0_i64..=12 {
        for b in 0_i64..=12 {
            let env = BTreeMap::from([("a".into(), a), ("b".into(), b)]);
            for (symbol, op, expected) in [
                ("+", school::Op::Add, Some(a + b)),
                ("-", school::Op::Sub, (a >= b).then_some(a - b)),
                ("*", school::Op::Mul, Some(a * b)),
                (
                    "/",
                    school::Op::Div,
                    if b != 0 && a % b == 0 {
                        Some(a / b)
                    } else {
                        None
                    },
                ),
            ] {
                let Some(expected) = expected else { continue };
                let expression = format!("a {symbol} b");
                let p = school::Problem {
                    text: expression.clone(),
                    a,
                    b,
                    op,
                };
                assert_eq!(lessons::eval(&expression, &env), Some(expected));
                assert_eq!(p.answer(), expected);
            }
        }
    }
    // The lesson evaluator admits negative results; school subtraction does not.
    assert_eq!(lessons::eval("1 - 2", &BTreeMap::new()), Some(-1));
}

#[test]
fn generated_lesson_splits_retain_numeric_answer_identity() {
    let source = "lesson: 101\ntitle: Supplied addition\nvars: a=2..8 b=1..4\nanswer: a + b\nexercise: Exercise {a} apples and {b} apples.\ntest: Test {a} apples and {b} apples.\nexam: Exam {a} apples and {b} apples.\n";
    let lesson = lessons::parse(source, "fixture.lesson").unwrap();
    for (tag, draws) in [
        ("Exercise", lesson.exercises(32)),
        ("Test", lesson.test(32)),
        ("Exam", lesson.exam(32)),
    ] {
        assert_eq!(draws.len(), 32);
        for draw in draws {
            assert!(draw.text.starts_with(tag));
            let numbers = draw
                .text
                .split_whitespace()
                .filter_map(|w| w.parse::<i64>().ok())
                .collect::<Vec<_>>();
            assert_eq!(numbers.len(), 2);
            assert!((2..=8).contains(&numbers[0]));
            assert!((1..=4).contains(&numbers[1]));
            assert_eq!(draw.answer, numbers[0] + numbers[1]);
            assert!(primers::reachable(&draw.text, draw.answer));
        }
    }
    let one = lesson
        .exercises(32)
        .into_iter()
        .map(|x| (x.text, x.answer))
        .collect::<Vec<_>>();
    let two = lesson
        .exercises(32)
        .into_iter()
        .map(|x| (x.text, x.answer))
        .collect::<Vec<_>>();
    assert_eq!(one, two);
    // Sharing an arithmetic objective does not permit interchanging assessment phrasings.
    assert!(
        lesson
            .exercises(32)
            .iter()
            .all(|x| !x.text.starts_with("Test"))
    );
}

#[test]
fn state_answers_preserve_fallback_and_control_denominators() {
    let w = supplied_world("loc L1 type=room name=Hall ^s1\nchar C1 type=human name=Ada at=L1 ^s1");
    let text = "7\th\tsetting\tWhere?\tunknown || where C1 @last\thall\n7\tm\tsetting\tWhere?\twhere C1 @last\tmoon\n7\tu\tsetting\tWhere?\twhere C99 @last\thall\n7\tc\tcontrol\tUnknown?\twhere C99 @last\thall\n";
    let questions = qa::read_questions(text).unwrap();
    let before = w
        .history(
            world::types::Id::parse("C1").unwrap(),
            world::types::Field::At,
        )
        .len();
    let (_, score) = qa::answers_md(&w, &questions);
    assert_eq!(
        (score.hit, score.miss, score.none, score.ctrl_ok, score.ctrl),
        (1, 1, 1, 1, 1)
    );
    let direct = qa::answer(&w, "where C1 @last");
    let fallback = qa::answer(&w, &questions[0].query);
    assert!(direct.from_state && fallback.from_state);
    assert_eq!(direct.text, fallback.text);
    assert_eq!(qa::verdict(&fallback, "hall"), qa::Verdict::Hit);
    assert_eq!(
        w.history(
            world::types::Id::parse("C1").unwrap(),
            world::types::Field::At
        )
        .len(),
        before
    );
}

#[test]
fn narrative_order_survives_query_and_anchor_grading() {
    let w = supplied_world(
        "char C1 type=human name=Ada ^s1\nevent E1 verb=walk agent=C1 ^s1\nevent E2 verb=walk agent=C1 ^s2\nevent E3 verb=walk agent=C1 ^s3",
    );
    let answer = qa3::answer(&w, "unknown || sort E3 E1 E2");
    assert!(answer.from_state, "{}", answer.text);
    assert_eq!(qa3::verdict(&answer, "E1>E2>E3"), qa::Verdict::Hit);
    assert_eq!(qa3::verdict(&answer, "@s1-1>@s2-2>@s3-3"), qa::Verdict::Hit);
    assert_eq!(qa3::verdict(&answer, "E3>E2>E1"), qa::Verdict::Miss);
    assert_eq!(qa::verdict(&answer, "E1>E2>E3"), qa::Verdict::Hit);
    assert!(!qa3::answer(&w, "sort").from_state);
    assert!(qa3::answer(&w, "sort || frames").from_state);
    let absent = qa3::answer(&w, "order E1 E99");
    assert!(!absent.from_state);
    assert_eq!(qa3::verdict(&absent, "none"), qa::Verdict::Hit);
    assert_eq!(qa::verdict(&absent, "none"), qa::Verdict::NotInState);
    let qs = qa3::read_questions("fixture\tq\ttime\tOrder?\tsort E3 E1 E2\tE1>E2>E3\n").unwrap();
    assert_eq!(
        qa3::answers_md(&w, &qs).1,
        vec![("time".into(), qa::Verdict::Hit)]
    );
}

#[test]
fn frame_roles_and_invariant_hits_share_token_identity() {
    for (name, object) in [("Tom", "box"), ("Anna", "book")] {
        let words = sentence(name, object);
        let t = frames::Tree::of_words(words.clone());
        let mut context = frames::Ctx::default();
        let (mut fs, _, _) = frames::frames_of_tree(&t, &mut context, false);
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].agent.as_ref().unwrap().id, name.to_lowercase());
        assert_eq!(fs[0].patient.as_ref().unwrap().id, object);
        let actual = roles::roles(words);
        assert!(actual.contains(&(1, 2, "agent")), "{actual:?}");
        assert!(actual.contains(&(3, 2, "patient")), "{actual:?}");
        assert!(invariants::sentence(&t, &fs, &context.anchors).is_empty());
        fs[0].patient = fs[0].agent.clone();
        let hits = invariants::sentence(&t, &fs, &context.anchors);
        assert!(
            hits.iter()
                .any(|h| h.check == invariants::Check::WhoIsWhat && h.row == 1)
        );
    }
}

#[test]
fn lexical_means_reach_domain_report_with_original_forms() {
    for (first, second) in [("take", "place"), ("TAKE", "PLACE"), ("Take", "place")] {
        let words = vec![
            word(first, "take", UPos::VERB, 0, Rel::Root),
            word(second, "place", UPos::NOUN, 1, Rel::Obj),
        ];
        let view = lens::lens(&words);
        assert!(!view.readings.is_empty());
        let means = lens::means(&view);
        assert_ne!(means, "-");
        assert!(means.contains(&format!("{first} {second}")));
        let report = domains::domains_with(&words, "real", &BTreeMap::new());
        assert_eq!(report.means, means);
    }
}

#[test]
fn tree_form_agreement_roundtrip_preserves_sentence_and_voice() {
    let block = |id: &str| {
        format!(
            "# sent_id = {id}\n1\tLook\tlook\tVERB\tVB\t_\t0\troot\t_\t_\n2\t.\t.\tPUNCT\t.\t_\t1\tpunct\t_\t_\n"
        )
    };
    let labels = "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\nb\tdecl\tassert\t0\t0\tnone\tcharacter\na\tdecl\tassert\t0\t0\tnone\tnone\n";
    let trees = format!("{}\n{}", block("b"), block("a"));
    assert_eq!(agree::tree_form(&block("a")), "imp");
    let (output, _) = agree::run(labels, "sent_id\n", &trees);
    let crlf = agree::run(
        &labels.replace('\n', "\r\n"),
        "sent_id\n",
        &trees.replace('\n', "\r\n"),
    );
    assert_eq!(crlf.0, output);
    let table = agree::table(&output);
    assert_eq!(table.len(), 2);
    assert_eq!(
        (&table["a"]["form"], &table["a"]["act"]),
        (&"imp".to_owned(), &"request".to_owned())
    );
    assert_eq!(
        (&table["b"]["form"], &table["b"]["act"]),
        (&"decl".to_owned(), &"assert".to_owned())
    );
    assert_eq!(table["a"]["voice"], "none");
    assert_eq!(table["b"]["voice"], "character");
}

#[test]
fn document_role_and_context_projections_restore_input_order() {
    let sents = vec![
        ("school:fixture:a:2".to_owned(), sentence("Anna", "book")),
        ("school:fixture:b:1".to_owned(), sentence("Ben", "box")),
        ("school:fixture:a:1".to_owned(), sentence("Tom", "box")),
    ];
    let projected = roles::roles_docs(sents.clone());
    let contexts = roles::world_docs(sents.clone(), 1);
    for i in 0..sents.len() {
        assert_eq!(projected[i].0, sents[i].0);
        assert_eq!(contexts[i].0, sents[i].0);
        assert!(projected[i].1.contains(&(1, 2, "agent")));
    }
    assert!(contexts[1].1.ends_with("recent events: none yet"));
    assert!(contexts[2].1.ends_with("recent events: none yet"));
    assert!(contexts[0].1.to_lowercase().contains("tom"));
    assert!(!contexts[0].1.to_lowercase().contains("ben"));
    let reordered = vec![sents[2].clone(), sents[0].clone(), sents[1].clone()];
    let by_id = |v: Vec<(String, String)>| v.into_iter().collect::<BTreeMap<_, _>>();
    assert_eq!(by_id(contexts), by_id(roles::world_docs(reordered, 1)));
}

#[test]
fn retained_weight_codecs_preserve_activation_and_vote_tables() {
    for text in [
        "gain\t0\t9\t0\t0\n",
        "gain\t2\t1\t0\t0\t4\t3\t0\t0\n",
        "zero\t0\t0\t0\t0\t0\t0\t0\t0\ngain\t0\t9\t0\t0\n",
    ] {
        let state = school::Student::from_tsv(text);
        let encoded = state.to_tsv();
        let restored = school::Student::from_tsv(&encoded);
        assert_eq!(state.w, restored.w);
        assert_eq!(state.acc, restored.acc);
        assert_eq!(state.acc.is_empty(), restored.acc.is_empty());
        assert_eq!(restored.to_tsv(), encoded);
    }
    for first in -4_i64..=4 {
        for second in -4_i64..=4 {
            let text = format!("give\t-1\t1\t{first}\ngive\t-1\t1\t{second}\nkeep\t0\t0\t2\n");
            let state = school::Actors::from_tsv(&text);
            assert_eq!(state.votes["give"][&(-1, 1)], first + second);
            let encoded = state.to_tsv();
            let restored = school::Actors::from_tsv(&encoded);
            assert_eq!(state.votes, restored.votes);
            assert_eq!(restored.to_tsv(), encoded);
        }
    }
    let extreme = format!("give\t-1\t1\t{}\ngive\t-1\t1\t1\n", i64::MAX);
    let decoded = school::Actors::from_tsv(&extreme);
    assert_eq!(decoded.votes["give"][&(-1, 1)], i64::MAX);
    assert_eq!(
        school::Actors::from_tsv(&decoded.to_tsv()).votes,
        decoded.votes
    );
    // This is a lossy clipping policy for extreme duplicate totals, not an exact sum.
}

#[test]
fn grouped_assessment_folds_share_identity_and_bounded_indices() {
    assert_eq!(school::assessment_fold(0, None, 0, 0), None);
    assert_eq!(school::assessment_fold(0, Some(1), 0, 0), None);
    let primes = [1_u128, 7, 13, 31, 101];
    for folds in 1..=8 {
        for repeat in 0..8 {
            for index in 0..32 {
                let expected = ((index as u128 * primes[repeat % primes.len()] + repeat as u128)
                    % folds as u128) as usize;
                assert_eq!(
                    school::assessment_fold(index, None, repeat, folds),
                    Some(expected)
                );
            }
            for group in 0_u64..=8 {
                let observed = school::assessment_fold(0, Some(group), repeat, folds).unwrap();
                assert!(observed < folds);
                for index in 1..16 {
                    assert_eq!(
                        school::assessment_fold(index, Some(group), repeat, folds),
                        Some(observed)
                    );
                }
            }
        }
    }
    for group in [None, Some(u64::MAX)] {
        assert!(
            school::assessment_fold(usize::MAX, group, usize::MAX, usize::MAX).unwrap()
                < usize::MAX
        );
    }
}
