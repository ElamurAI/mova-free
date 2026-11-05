//! Supplied-input joins; no corpus, parser acquisition, journal or adoption.
use en::annotate::Word;
use en::gram::{Feats, Rel, Tag, UPos};
use prag::{
    data, eval,
    feats::{self, Dict, SLOTS},
    frames, frames_md,
    model::Kind,
    schema::{Act, Field},
};
use std::collections::BTreeMap;
use world::{domains, tune};
#[test]
fn batched_items_keep_source_identity_through_classification() {
    let all: Vec<_> = (0..3)
        .map(|i| data::TaleSent {
            doc: "story".into(),
            title: "fixture".into(),
            para: 1,
            ord: i + 1,
            sent_id: format!("s{i}"),
            text: format!("sentence {i}"),
        })
        .collect();
    let items = data::make_items(
        &all,
        &[vec![vec![1, 2]]],
        &[("a".into(), "Hello".into()), ("b".into(), "Bye".into())],
        3,
        1,
    );
    assert_eq!(
        items.iter().map(|i| i.key.as_str()).collect::<Vec<_>>(),
        ["s1", "s2", "tatoeba:a", "tatoeba:b"]
    );
    assert_eq!(
        items.iter().map(|i| i.batch).collect::<Vec<_>>(),
        [0, 0, 1, 2]
    );
    assert_eq!(items[0].ctx, ["sentence 0"]);
    assert!(items[1..].iter().all(|i| i.ctx.is_empty()));
    let restored: Vec<data::Item> =
        serde_json::from_str(&serde_json::to_string(&items).unwrap()).unwrap();
    let prev = feats::prev_of(&restored);
    assert_eq!(prev, [None, Some(0), None, None]);
    let rows: Vec<Vec<String>> = restored
        .iter()
        .map(|i| {
            let mut r = vec!["-".into(); SLOTS.len()];
            r[0] = i.text.clone();
            r[feats::prev_slot()] = "start".into();
            r
        })
        .collect();
    let dict = Dict::build(&rows);
    let x = rows.iter().map(|r| dict.encode(r)).collect::<Vec<_>>();
    let expected = [0, Act::Request.idx(), 0, 0, 2, 2];
    let ys: Vec<_> = expected.iter().map(|&c| vec![c; 4]).collect();
    let keys = restored.iter().map(|i| i.key.clone()).collect::<Vec<_>>();
    let set = eval::fit_set(Kind::IgTree, &x, &ys, &keys, &dict, 1);
    let out = eval::predict_seq(&set, &rows, &prev, &[0, 1, 2, 3], &dict);
    for i in 0..4 {
        assert_eq!(out[&i].class, expected);
    }
    let actual = (0..4).map(|i| out[&i].class[1]).collect::<Vec<_>>();
    assert_eq!(
        eval::scores(
            &[Act::Request.idx(); 4],
            &actual,
            Field::Act.classes().len()
        ),
        (1.0, 1.0)
    );
}
#[test]
fn closed_class_metrics_reject_invalid_observations() {
    let n = Field::Act.classes().len();
    let (g, p) = (
        [Act::Request.idx(), Act::Assert.idx()],
        [Act::Request.idx(), Act::Request.idx()],
    );
    let (a, f) = eval::scores(&g, &p, n);
    assert_eq!(a, 0.5);
    assert!((f - 1.0 / 3.0).abs() < 1e-12);
    assert!(std::panic::catch_unwind(|| eval::scores(&g, &p[..1], n)).is_err());
    assert!(std::panic::catch_unwind(|| eval::scores(&[n], &[0], n)).is_err());
    assert!(std::panic::catch_unwind(|| eval::scores(&[0], &[n], n)).is_err());
    assert_eq!(eval::scores(&[], &[], 0), (0.0, 0.0));
}
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
#[test]
fn supplied_frame_identity_survives_markdown_projection() {
    let words = vec![
        word("Ada", "ada", UPos::PROPN, 2, Rel::Nsubj),
        word("carries", "carry", UPos::VERB, 0, Rel::Root),
        word("box", "box", UPos::NOUN, 2, Rel::Obj),
    ];
    let (fs, _, _) = frames::frames_of_tree(
        &frames::Tree::of_words(words.clone()),
        &mut frames::Ctx::default(),
        false,
    );
    assert_eq!(fs.len(), 1);
    assert_eq!(fs[0].agent.as_ref().unwrap().id, "ada");
    assert_eq!(fs[0].patient.as_ref().unwrap().id, "box");
    let table = frames_md::table(&fs);
    assert!(table.contains("**carry**"));
    assert!(table.contains("Ada"));
    assert!(table.contains("box"));
    assert_eq!(table.lines().filter(|l| l.starts_with("| 1 |")).count(), 1);
    assert!(prag::roles::roles(words).contains(&(1, 2, "agent")));
    assert!(frames_md::table(&[]).contains("no event"));
}
#[test]
fn hebrew_annotation_rows_keep_macula_participant_identity() {
    let text = "1\tv\tv\tVERB\t_\tHebBinyan=Qal\t0\troot\t_\tMId=o1|Frame=A0:2|Gloss=did.work|Type=finite\n2\tx\tx\tNOUN\t_\t_\t1\tnsubj\t_\tMId=o2|Gloss=worker\n";
    let words = frames_md::hwords(text);
    assert_eq!(words.len(), 2);
    assert_eq!(words[1].head, 1);
    let table = frames_md::hebrew_table(&words);
    assert!(table.contains("v *did work*"));
    assert!(table.contains("x *worker*"));
    assert!(table.contains("finite · qal"));
    assert_eq!(
        table,
        frames_md::hebrew_table(&frames_md::hwords(&text.replace('\n', "\r\n")))
    );
}
#[test]
fn domain_knobs_and_tuning_selection_preserve_candidate_identity() {
    let knobs = domains::knobs();
    let base: BTreeMap<_, _> = knobs.iter().map(|(k, v, _)| (k.to_string(), *v)).collect();
    let candidates = tune::neighboring_configs(&knobs, &base);
    assert!(candidates.len() > 2);
    for (_, cfg) in &candidates {
        assert_eq!(cfg.len(), base.len());
        assert_eq!(
            cfg.iter().filter(|(k, v)| base.get(*k) != Some(*v)).count(),
            1
        );
        assert!(cfg.values().all(|v| v.is_finite()));
    }
    let observations = [
        (2, 5, 0, 0.02, false),
        (0, 5, 0, 0.03, true),
        (1, 9, -1, 0.01, false),
    ];
    let winner = tune::select_candidate(&observations).unwrap();
    assert_eq!(winner.0, 0);
    assert_eq!(candidates[winner.0].1.len(), base.len());
    assert!(tune::select_candidate(&[(0, 1, 0, 0.05, false), (1, 0, 1, 0.01, false)]).is_none());
}
