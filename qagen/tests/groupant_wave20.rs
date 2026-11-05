//! Synthetic composition only: no model, teacher, corpus or database.
use qagen::{
    fix, gate,
    lemma::{self, Lemma, Lemmas},
    schema::ExIm,
    select::{Para, Sent},
};
use std::cell::Cell;
#[derive(Default)]
struct Stub {
    calls: Cell<usize>,
}
impl Lemmas for Stub {
    fn lemmas(&self, s: &str) -> Vec<Lemma> {
        self.calls.set(self.calls.get() + 1);
        gate::words(s)
            .into_iter()
            .map(|form| {
                let lemma = if form == "cats" {
                    "cat".into()
                } else {
                    form.clone()
                };
                Lemma {
                    content: form != "the",
                    form,
                    lemma,
                }
            })
            .collect()
    }
}
fn fixture() -> (Vec<Para>, Vec<fix::Item>) {
    let p = Para {
        doc: "synthetic".into(),
        book: "synthetic".into(),
        title: "Fixture".into(),
        author: String::new(),
        para: 1,
        block: "p".into(),
        sents: vec![
            Sent {
                sent_id: "s1".into(),
                text: "The cat sleeps.".into(),
            },
            Sent {
                sent_id: "s2".into(),
                text: "The dog runs.".into(),
            },
        ],
        dialogue: false,
        words: 6,
        ctx: vec![],
        batch: 0,
    };
    let i = fix::Item {
        doc: p.doc.clone(),
        para: 1,
        k: 1,
        qtype: "action".into(),
        question: "What happens?".into(),
        answer: "cats sleeps".into(),
        anchors: vec![1],
        reason: "explicit: not a span".into(),
        batch: 0,
    };
    (vec![p], vec![i])
}
#[test]
fn validated_fix_prompt_gate_and_lemma_join() {
    let (p, i) = fixture();
    let f = fix::Fixes::try_new(&p, &i).unwrap();
    let (ps, its) = &f.all[&0];
    assert!(fix::prompt(ps, its).contains("Item 1 (action; anchors 1)"));
    let g = fix::gate("1\t-\timplicit\n", ps, its);
    assert!(g.format.is_none());
    assert_eq!(g.ok.len(), 1);
    assert_eq!(g.ok[0].exim, ExIm::Implicit);
    assert_eq!(g.ok[0].answer, i[0].answer);
    let stub = Stub::default();
    let texts: Vec<&str> = p[0].sents.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(
        lemma::try_doubt(&stub, &texts, &g.ok[0].anchors, &g.ok[0].answer).unwrap(),
        Some((1, 1.))
    );
    assert_eq!(stub.calls.get(), 2);
}
#[test]
fn explicit_fix_anchor_ids_join_verbatim_and_lemma() {
    let (p, i) = fixture();
    let f = fix::Fixes::try_new(&p, &i).unwrap();
    let (ps, its) = &f.all[&0];
    let g = fix::gate("Item 1\t2\tthe dog runs\n", ps, its);
    assert!(g.format.is_none());
    assert_eq!(g.ok.len(), 1);
    let out = &g.ok[0];
    assert_eq!(out.exim, ExIm::Explicit);
    assert_eq!(p[0].sents[out.anchors[0] - 1].sent_id, "s2");
    let s = gate::Sents::of(&p[0]);
    assert!(gate::span_reason(&out.answer, &s.norm, &s.words, &out.anchors).is_none());
    let stub = Stub::default();
    assert_eq!(
        lemma::try_doubt(
            &stub,
            &["The cat sleeps.", "The dog runs."],
            &out.anchors,
            &out.answer
        )
        .unwrap(),
        Some((2, 1.))
    );
}
#[test]
fn malformed_contexts_are_rejected_before_lemma_callbacks() {
    let (p, mut i) = fixture();
    i[0].anchors = vec![0];
    assert!(fix::Fixes::try_new(&p, &i).is_err());
    let stub = Stub::default();
    assert!(lemma::try_doubt(&stub, &["cat"], &[1, 0], "cat").is_err());
    assert_eq!(stub.calls.get(), 0);
    assert_eq!(lemma::doubt(&stub, &["cat"], &[2], "cat"), None);
    assert_eq!(stub.calls.get(), 0);
    let (p, i) = fixture();
    let f = fix::Fixes::try_new(&p, &i).unwrap();
    let (ps, its) = &f.all[&0];
    let g = fix::gate("é🙂\t1\tcat\n", ps, its);
    assert!(g.format.is_some());
    assert!(g.ok.is_empty());
}
