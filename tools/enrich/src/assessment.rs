//! Assessment-binding model and a composition contract for study comparisons.
//! Forward: bind a result to the exact scored inputs and learner, then compare.
//! Inverse: equal totals/learner hashes cannot authorize a different assessment.
//! formal/StudyComparison.lean models the gates and score relation; tests compare
//! the Rust policy to its finite table. SHA collision resistance is assumed.
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use world::knowledge::{Lesson, retention::assessment_fingerprint};

struct Expected {
    kind: &'static str,
    index: usize,
    question: String,
    right: String,
    options: Vec<String>,
}
pub struct AssessmentContract {
    identity: String,
    rows: Vec<Expected>,
}
#[derive(Debug)]
pub struct BoundGrade {
    model: String,
    assessment: String,
    right: u64,
    total: u64,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Comparison {
    pub gain: i64,
    pub regression_preserved: bool,
    pub recommend: bool,
}
impl AssessmentContract {
    pub fn new(l: &Lesson) -> Result<Self> {
        ensure!(
            l.explain.is_empty() && l.facts.is_empty() && l.exercises.is_empty(),
            "assessment contract requires question-only input"
        );
        let mut rows = Vec::new();
        for (kind, qs) in [("test", &l.tests), ("exam", &l.exams)] {
            for (index, (q, right, wrong)) in qs.iter().enumerate() {
                rows.push(Expected {
                    kind,
                    index,
                    question: q.clone(),
                    right: right.clone(),
                    options: std::iter::once(right.clone())
                        .chain(wrong.iter().cloned())
                        .collect(),
                });
            }
        }
        ensure!(!rows.is_empty(), "empty assessment contract");
        Ok(Self {
            identity: assessment_fingerprint(l)?,
            rows,
        })
    }
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn bind(&self, v: &Value, model: &str) -> Result<BoundGrade> {
        ensure!(
            !model.is_empty() && v["state_sha256"] == model,
            "assessment learner identity mismatch"
        );
        ensure!(
            v["assessment_sha256"] == self.identity,
            "assessment identity mismatch (legacy results need a fresh bound exam)"
        );
        ensure!(
            v["lesson_reads"] == 0 && v["exercise_updates"] == 0,
            "assessment performed learning"
        );
        let rows = v["rows"].as_array().context("assessment rows missing")?;
        let total = v["total"].as_u64().context("assessment total missing")?;
        let right = v["right"].as_u64().context("assessment score missing")?;
        ensure!(
            rows.len() == self.rows.len() && total == self.rows.len() as u64 && right <= total,
            "assessment cardinality or score invalid"
        );
        let mut counted = 0;
        for (row, e) in rows.iter().zip(&self.rows) {
            ensure!(
                row["kind"] == e.kind
                    && row["index"] == e.index
                    && row["question"] == e.question
                    && row["right"] == e.right,
                "assessment row belongs to different scored input"
            );
            let choice = row["choice"]
                .as_str()
                .context("assessment choice missing")?;
            ensure!(
                e.options.iter().any(|o| o == choice),
                "assessment choice outside options"
            );
            let correct = choice == e.right;
            ensure!(
                row["correct"] == correct,
                "assessment correctness flag disagrees with choice"
            );
            counted += u64::from(correct);
        }
        ensure!(
            counted == right,
            "assessment aggregate score disagrees with rows"
        );
        Ok(BoundGrade {
            model: model.into(),
            assessment: self.identity.clone(),
            right,
            total,
        })
    }
}
pub fn compare(
    new_before: &BoundGrade,
    new_after: &BoundGrade,
    old_before: &BoundGrade,
    old_after: &BoundGrade,
) -> Result<Comparison> {
    ensure!(
        new_before.assessment == new_after.assessment && new_before.total == new_after.total,
        "new score pair uses different assessments"
    );
    ensure!(
        old_before.assessment == old_after.assessment && old_before.total == old_after.total,
        "regression score pair uses different assessments"
    );
    ensure!(
        new_before.model == old_before.model && new_after.model == old_after.model,
        "comparison mixes learner pairs"
    );
    let gain = i64::try_from(new_after.right)? - i64::try_from(new_before.right)?;
    let regression_preserved = old_after.right >= old_before.right;
    Ok(Comparison {
        gain,
        regression_preserved,
        recommend: gain > 0 && regression_preserved,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn lesson(question: &str) -> Lesson {
        Lesson {
            course: "books".into(),
            n: 1,
            tests: vec![(
                question.into(),
                "Alice".into(),
                vec!["Bob".into(), "Clara".into(), "Dora".into()],
            )],
            ..Default::default()
        }
    }
    fn report(c: &AssessmentContract, model: &str) -> Value {
        json!({"state_sha256":model,"assessment_sha256":c.identity(),"total":1,"right":1,
            "lesson_reads":0,"exercise_updates":0,
            "rows":[{"kind":"test","index":0,"question":c.rows[0].question,
                "right":"Alice","choice":"Alice","correct":true}]})
    }
    // Literal report metadata deliberately does not reuse the contract's rows:
    // construction order and per-kind indices are part of this correspondence.
    fn two_row_fixture() -> (AssessmentContract, Value) {
        let l = Lesson {
            tests: vec![(
                "First?".into(),
                "A".into(),
                vec!["A".into(), "B".into(), "B".into()],
            )],
            exams: vec![("Second?".into(), "X".into(), vec!["Y".into(), "Y".into()])],
            ..Default::default()
        };
        let c = AssessmentContract::new(&l).unwrap();
        let v = json!({"state_sha256":"learner", "assessment_sha256":c.identity(),
        "total":2, "right":2, "lesson_reads":0, "exercise_updates":0,
        "rows":[
            {"kind":"test","index":0,"question":"First?","right":"A","choice":"A","correct":true},
            {"kind":"exam","index":0,"question":"Second?","right":"X","choice":"X","correct":true}
        ]});
        (c, v)
    }

    #[test]
    fn catalog_choice_and_score_preimages_match_independent_sets() {
        use std::collections::BTreeSet;
        let (c, template) = two_row_fixture();
        let mut accepted = BTreeSet::new();
        // Include outside labels, incompatible flag types, and every declared score.
        // This is a finite universe of reports, not a reimplementation of bind.
        for a in ["A", "B", "outside"] {
            for b in ["X", "Y", "outside"] {
                for af in [json!(false), json!(true), json!(1), Value::Null] {
                    for bf in [json!(false), json!(true), json!("true"), Value::Null] {
                        for score in 0..=3u64 {
                            let mut v = template.clone();
                            v["rows"][0]["choice"] = json!(a);
                            v["rows"][1]["choice"] = json!(b);
                            v["rows"][0]["correct"] = af.clone();
                            v["rows"][1]["correct"] = bf.clone();
                            v["right"] = json!(score);
                            if let Ok(bound) = c.bind(&v, "learner") {
                                assert_eq!((bound.right, bound.total), (score, 2));
                                assert_eq!(bound.assessment, c.identity());
                                assert_eq!(bound.model, "learner");
                                accepted.insert((a, b, af.to_string(), bf.to_string(), score));
                            }
                        }
                    }
                }
            }
        }
        let expected = BTreeSet::from([
            ("A", "X", "true".into(), "true".into(), 2),
            ("A", "Y", "true".into(), "false".into(), 1),
            ("B", "X", "false".into(), "true".into(), 1),
            ("B", "Y", "false".into(), "false".into(), 0),
        ]);
        assert_eq!(
            accepted, expected,
            "duplicate option positions cannot add label outcomes"
        );
        let fibers: Vec<_> = (0..=2)
            .map(|score| accepted.iter().filter(|row| row.4 == score).count())
            .collect();
        assert_eq!(fibers, [1, 2, 1]);
    }

    #[test]
    fn catalog_unknown_fields_preserve_binding_but_consulted_mutations_reject() {
        let (c, template) = two_row_fixture();
        for extension in [
            Value::Null,
            json!(false),
            json!(7),
            json!("text"),
            json!([1, {"nested":true}]),
            json!({"choice":"outside","right":999}),
        ] {
            let mut v = template.clone();
            v["extra"] = extension.clone();
            v["rows"][0]["extra"] = extension;
            let bound = c.bind(&v, "learner").unwrap();
            assert_eq!((bound.right, bound.total), (2, 2));
            assert_eq!(bound.model, "learner");
            assert_eq!(bound.assessment, c.identity());
            // Unconsulted data must not rescue an invalid consulted value.
            v["rows"][1]["correct"] = json!(false);
            assert!(c.bind(&v, "learner").is_err());
        }
        for (pointer, value) in [
            ("/total", json!(1)),
            ("/right", json!(3)),
            ("/rows", json!([])),
            ("/total", json!(-1)),
            ("/right", json!(1.0)),
            ("/rows/0/choice", Value::Null),
            ("/rows/0/index", json!(1)),
            ("/rows/1/kind", json!("test")),
        ] {
            let mut v = template.clone();
            *v.pointer_mut(pointer).unwrap() = value;
            assert!(c.bind(&v, "learner").is_err(), "{pointer}");
        }
        let mut reversed = template;
        reversed["rows"].as_array_mut().unwrap().reverse();
        assert!(c.bind(&reversed, "learner").is_err());
    }

    #[test]
    fn catalog_first_error_partitions_multifault_reports() {
        let c = AssessmentContract::new(&lesson("Who carried it?")).unwrap();
        let faults = [
            (
                "/state_sha256",
                json!("other"),
                "assessment learner identity mismatch",
            ),
            (
                "/assessment_sha256",
                json!("other"),
                "assessment identity mismatch (legacy results need a fresh bound exam)",
            ),
            ("/lesson_reads", json!(1), "assessment performed learning"),
            ("/total", json!(-1), "assessment total missing"),
            ("/right", json!(-1), "assessment score missing"),
            (
                "/rows/0/question",
                json!("other"),
                "assessment row belongs to different scored input",
            ),
            (
                "/rows/0/choice",
                json!("outside"),
                "assessment choice outside options",
            ),
            (
                "/rows/0/correct",
                json!(false),
                "assessment correctness flag disagrees with choice",
            ),
        ];
        for mask in 1..(1usize << faults.len()) {
            let mut v = report(&c, "learner");
            for (i, (pointer, value, _)) in faults.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    *v.pointer_mut(pointer).unwrap() = value.clone();
                }
            }
            let first = mask.trailing_zeros() as usize;
            assert_eq!(
                c.bind(&v, "learner").unwrap_err().to_string(),
                faults[first].2,
                "fault subset {mask}"
            );
        }
    }

    #[test]
    fn catalog_signed_edges_reject_unrepresentable_new_scores_only() {
        let max = i64::MAX as u64;
        let grade = |right, assessment: &str| BoundGrade {
            model: "same".into(),
            assessment: assessment.into(),
            right,
            total: u64::MAX,
        };
        // Explicit boundary results; old unsigned scores do not undergo signed conversion.
        for (before, after, gain) in [
            (0, 0, 0),
            (0, max, i64::MAX),
            (max, 0, -i64::MAX),
            (max, max, 0),
            (max - 1, max, 1),
            (max, max - 1, -1),
        ] {
            let result = compare(
                &grade(before, "new"),
                &grade(after, "new"),
                &grade(u64::MAX, "old"),
                &grade(u64::MAX, "old"),
            )
            .unwrap();
            assert_eq!(
                result,
                Comparison {
                    gain,
                    regression_preserved: true,
                    recommend: gain > 0
                }
            );
        }
        for invalid in [max + 1, u64::MAX] {
            for (before, after) in [(invalid, 0), (0, invalid), (invalid, invalid)] {
                assert!(
                    compare(
                        &grade(before, "new"),
                        &grade(after, "new"),
                        &grade(0, "old"),
                        &grade(0, "old")
                    )
                    .is_err()
                );
            }
        }
        let result = compare(
            &grade(0, "new"),
            &grade(1, "new"),
            &grade(u64::MAX, "old"),
            &grade(max + 1, "old"),
        )
        .unwrap();
        assert_eq!(
            result,
            Comparison {
                gain: 1,
                regression_preserved: false,
                recommend: false
            }
        );
    }

    #[test]
    fn catalog_constructor_excludes_empty_and_teaching_inputs() {
        // Cross question presence with all subsets of teaching fields.
        for questions in 0..4 {
            for teaching in 0..8 {
                let mut l = Lesson::default();
                let q = ("Question?".into(), "yes".into(), vec!["no".into()]);
                if questions & 1 != 0 {
                    l.tests.push(q.clone());
                }
                if questions & 2 != 0 {
                    l.exams.push(q.clone());
                }
                if teaching & 1 != 0 {
                    l.explain.push("Explanation".into());
                }
                if teaching & 2 != 0 {
                    l.facts.push("Fact".into());
                }
                if teaching & 4 != 0 {
                    l.exercises.push(q);
                }
                let result = AssessmentContract::new(&l);
                match (questions, teaching) {
                    (_, 1..=7) => assert_eq!(
                        result.err().unwrap().to_string(),
                        "assessment contract requires question-only input"
                    ),
                    (0, 0) => assert_eq!(
                        result.err().unwrap().to_string(),
                        "empty assessment contract"
                    ),
                    _ => assert!(result.is_ok()),
                }
            }
        }
    }

    #[test]
    fn same_count_and_model_cannot_bind_a_different_assessment() {
        let a = AssessmentContract::new(&lesson("Who carried it?")).unwrap();
        let b = AssessmentContract::new(&lesson("Who dropped it?")).unwrap();
        let v = report(&a, "learner");
        assert!(a.bind(&v, "learner").is_ok());
        assert!(b.bind(&v, "learner").is_err());
        let mut renamed = v;
        renamed["assessment_sha256"] = b.identity().into();
        assert!(
            b.bind(&renamed, "learner").is_err(),
            "rows also bind the actual question"
        );
    }
    #[test]
    fn bounded_inverse_model_requires_every_binding_guard() {
        let c = AssessmentContract::new(&lesson("Who carried it?")).unwrap();
        for bits in 0..64 {
            let mut v = report(&c, "learner");
            if bits & 1 == 0 {
                v["state_sha256"] = "other".into();
            }
            if bits & 2 == 0 {
                v["assessment_sha256"] = "other".into();
            }
            if bits & 4 == 0 {
                v["total"] = 2.into();
            }
            if bits & 8 == 0 {
                v["lesson_reads"] = 1.into();
            }
            if bits & 16 == 0 {
                v["exercise_updates"] = 1.into();
            }
            if bits & 32 == 0 {
                v["rows"][0]["question"] = "different".into();
            }
            assert_eq!(c.bind(&v, "learner").is_ok(), bits == 63, "case {bits}");
        }
        let mut v = report(&c, "learner");
        v["right"] = 0.into();
        assert!(
            c.bind(&v, "learner").is_err(),
            "aggregate must agree with rows"
        );
    }
    #[test]
    fn comparison_matches_lean_finite_table_and_rejects_mixed_pairs() {
        let table = include_str!("../../../formal/StudyComparison.table");
        let mut count = 0;
        for row in table
            .lines()
            .filter(|s| !s.starts_with('#') && !s.trim().is_empty())
        {
            let c: Vec<_> = row.split('\t').collect();
            let v: Vec<u64> = c[..4].iter().map(|s| s.parse().unwrap()).collect();
            let grade = |model: &str, assessment: &str, right: u64| BoundGrade {
                model: model.into(),
                assessment: assessment.into(),
                right,
                total: 3,
            };
            let before = grade("before", "new", v[0]);
            let after = grade("after", "new", v[1]);
            let old = grade("before", "old", v[2]);
            let regression = grade("after", "old", v[3]);
            let comparison = compare(&before, &after, &old, &regression).unwrap();
            assert_eq!(comparison.recommend, c[4] == "true", "{row}");
            count += 1;
        }
        assert_eq!(count, 256);
        let a = BoundGrade {
            model: "before".into(),
            assessment: "a".into(),
            right: 1,
            total: 3,
        };
        let b = BoundGrade {
            model: "after".into(),
            assessment: "b".into(),
            right: 2,
            total: 3,
        };
        assert!(
            compare(&a, &b, &a, &b).is_err(),
            "a gain cannot mix question sets"
        );
    }

    // Build actual reports from independently declared row metadata. All four
    // comparator arguments pass the public constructor/binder path.
    fn scored_contract(n: usize, tag: &str, gold: &str, wrong: &str) -> AssessmentContract {
        AssessmentContract::new(&Lesson {
            tests: (0..n)
                .map(|i| (format!("{tag} {i}?"), gold.into(), vec![wrong.into()]))
                .collect(),
            ..Default::default()
        })
        .unwrap()
    }
    #[allow(clippy::too_many_arguments)]
    fn scored_report(
        c: &AssessmentContract,
        n: usize,
        tag: &str,
        model: &str,
        score: usize,
        gold: &str,
        wrong: &str,
    ) -> Value {
        json!({"state_sha256":model,"assessment_sha256":c.identity(),
            "lesson_reads":0,"exercise_updates":0,"total":n,"right":score,
            "rows":(0..n).map(|i| json!({"kind":"test","index":i,
                "question":format!("{tag} {i}?"),"right":gold,
                "choice":if i<score {gold} else {wrong},"correct":i<score})).collect::<Vec<_>>()})
    }
    fn scored_grade(
        c: &AssessmentContract,
        n: usize,
        tag: &str,
        model: &str,
        score: usize,
    ) -> BoundGrade {
        c.bind(&scored_report(c, n, tag, model, score, "yes", "no"), model)
            .unwrap()
    }

    #[test]
    fn deep_full_output_preimages_and_complements_use_public_bound_grades() {
        use std::collections::BTreeMap;
        for n in 1..=4usize {
            for m in 1..=3usize {
                let new = scored_contract(n, "new", "yes", "no");
                let old = scored_contract(m, "old", "yes", "no");
                let mut fibers = BTreeMap::new();
                for b in 0..=n {
                    for a in 0..=n {
                        for u in 0..=m {
                            for v in 0..=m {
                                let output = compare(
                                    &scored_grade(&new, n, "new", "before", b),
                                    &scored_grade(&new, n, "new", "after", a),
                                    &scored_grade(&old, m, "old", "before", u),
                                    &scored_grade(&old, m, "old", "after", v),
                                )
                                .unwrap();
                                *fibers
                                    .entry((
                                        output.gain,
                                        output.regression_preserved,
                                        output.recommend,
                                    ))
                                    .or_insert(0usize) += 1;
                            }
                        }
                    }
                }
                // Closed-form inverse cardinalities, not replayed branch logic:
                // a-b=d has n+1-|d| solutions; u<=v is a triangular lattice.
                let mut expected = BTreeMap::new();
                for d in -(n as i64)..=n as i64 {
                    let new_count = n + 1 - d.unsigned_abs() as usize;
                    expected.insert((d, false, false), new_count * m * (m + 1) / 2);
                    expected.insert((d, true, d > 0), new_count * (m + 1) * (m + 2) / 2);
                }
                assert_eq!(fibers, expected, "denominators {n},{m}");
                let recommended: usize = fibers.iter().filter(|(k, _)| k.2).map(|(_, v)| v).sum();
                assert_eq!(recommended, n * (n + 1) * (m + 1) * (m + 2) / 4);
                // The other candidate triples are the behavior complement in
                // this finite output universe; impossible recommendation flags
                // and out-of-denominator gains must have empty preimages.
                for d in -(n as i64) - 1..=n as i64 + 1 {
                    for preserved in [false, true] {
                        for recommend in [false, true] {
                            assert_eq!(
                                fibers.contains_key(&(d, preserved, recommend)),
                                expected.contains_key(&(d, preserved, recommend))
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn deep_four_corner_identity_constraints_and_renamings() {
        let new = scored_contract(2, "new", "yes", "no");
        let old = scored_contract(3, "old", "yes", "no");
        // Every learner assignment on two identities, including same-snapshot
        // pairs: compare checks matching labels, not actual execution provenance.
        for mask in 0..16usize {
            let ids: Vec<_> = (0..4)
                .map(|i| {
                    if mask & (1 << i) == 0 {
                        "alpha"
                    } else {
                        "beta"
                    }
                })
                .collect();
            let grades = [
                scored_grade(&new, 2, "new", ids[0], 0),
                scored_grade(&new, 2, "new", ids[1], 1),
                scored_grade(&old, 3, "old", ids[2], 2),
                scored_grade(&old, 3, "old", ids[3], 2),
            ];
            let out = compare(&grades[0], &grades[1], &grades[2], &grades[3]);
            assert_eq!(out.is_ok(), [0, 5, 10, 15].contains(&mask));
        }
        let baseline = [
            scored_grade(&new, 2, "new", "before", 0),
            scored_grade(&new, 2, "new", "after", 1),
            scored_grade(&old, 3, "old", "before", 2),
            scored_grade(&old, 3, "old", "after", 2),
        ];
        let expected = compare(&baseline[0], &baseline[1], &baseline[2], &baseline[3]).unwrap();
        for slot in 0..4 {
            // Same denominator and score; only the assessment metadata differs.
            let n = if slot < 2 { 2 } else { 3 };
            let foreign = scored_contract(n, "foreign", "yes", "no");
            let mut grades = [
                scored_grade(&new, 2, "new", "before", 0),
                scored_grade(&new, 2, "new", "after", 1),
                scored_grade(&old, 3, "old", "before", 2),
                scored_grade(&old, 3, "old", "after", 2),
            ];
            grades[slot] = scored_grade(
                &foreign,
                n,
                "foreign",
                if slot % 2 == 0 { "before" } else { "after" },
                [0, 1, 2, 2][slot],
            );
            assert!(compare(&grades[0], &grades[1], &grades[2], &grades[3]).is_err());
        }
        // Simultaneous bijections of questions, answer labels and learner IDs,
        // with fresh assessment identities, preserve the observable comparison.
        let renamed_new = scored_contract(2, "renamed-new", "oui", "non");
        let renamed_old = scored_contract(3, "renamed-old", "oui", "non");
        let renamed: Vec<_> = [
            (&renamed_new, 2, "renamed-new", "left", 0),
            (&renamed_new, 2, "renamed-new", "right", 1),
            (&renamed_old, 3, "renamed-old", "left", 2),
            (&renamed_old, 3, "renamed-old", "right", 2),
        ]
        .into_iter()
        .map(|(c, n, tag, id, score)| {
            c.bind(&scored_report(c, n, tag, id, score, "oui", "non"), id)
                .unwrap()
        })
        .collect();
        assert_eq!(
            compare(&renamed[0], &renamed[1], &renamed[2], &renamed[3]).unwrap(),
            expected
        );
        // Noninjective label maps do not preserve false answers.
        let collapsed = scored_contract(1, "collapsed", "same", "same");
        assert!(
            collapsed
                .bind(
                    &scored_report(&collapsed, 1, "collapsed", "left", 0, "same", "same"),
                    "left"
                )
                .is_err()
        );
    }

    #[test]
    fn deep_source_identity_is_not_review_authentication() {
        let original = lesson("Who?");
        let c = AssessmentContract::new(&original).unwrap();
        let template = report(&c, "learner");
        for variant in 0..4 {
            let mut l = original.clone();
            match variant {
                0 => l.course = "other-course".into(),
                1 => l.n += 1,
                2 => l.tests[0].2.reverse(),
                _ => l.tests[0].2.push("extra-option".into()),
            }
            let changed = AssessmentContract::new(&l).unwrap();
            assert_ne!(changed.identity(), c.identity());
            assert!(changed.bind(&template, "learner").is_err());
            // These provenance/option edits leave consulted report-row fields
            // unchanged. A consistently retagged report passes: no signature.
            let mut retagged = template.clone();
            retagged["assessment_sha256"] = json!(changed.identity());
            assert!(changed.bind(&retagged, "learner").is_ok());
        }
        for review in [Value::Null, json!("rejected"), json!({"approved":false})] {
            let mut report = template.clone();
            report["review"] = review;
            assert!(c.bind(&report, "learner").is_ok());
        }
    }
}
