// Independent cross-groupant fixtures, 2026-10-30. No live models or data.
#[allow(dead_code)]
#[path = "../src/ann.rs"]
mod ann;
use en::{
    conllu::Doc,
    convert::tree_gate,
    gram::{Feats, Rel, Tag, UPos},
    udreg::Registry,
};
fn fixture(heads: &[usize]) -> (ann::Ann, en::conllu::FullSentence) {
    let a: ann::Ann = heads
        .iter()
        .map(|&head| ann::Tok {
            form: "word".into(),
            lemma: "word".into(),
            upos: UPos::NOUN,
            tag: Tag::NN,
            feats: Feats::default(),
            head,
            rel: if head == 0 { Rel::Root } else { Rel::Dep },
        })
        .collect();
    let text = if heads.is_empty() {
        "1\tword\tword\tNOUN\tNN\t_\t0\troot\t_\t_\n\n".into()
    } else {
        ann::write_block("transfer", &a)
    };
    let mut s = Doc::parse(&text).unwrap().sents.remove(0);
    if heads.is_empty() {
        s.words.clear();
        s.rows.clear();
    }
    (a, s)
}
#[test]
fn groupant_common_tree_and_rejected_cycle_only_projection() {
    let reg = Registry::load();
    // Four valid trees and three tails/cycles; all avoid self edges.
    let cases: [(&[usize], Option<usize>); 7] = [
        (&[0], None),
        (&[0, 1, 2], None),
        (&[3, 3, 0], None),
        (&[2, 0, 2, 3], None),
        (&[3, 0, 4, 3], Some(1)),
        (&[0, 3, 2], Some(2)),
        (&[2, 3, 2, 0], Some(1)),
    ];
    let mut cases: Vec<_> = cases.into_iter().map(|(h, b)| (h.to_vec(), b)).collect();
    // Exact small-loop boundary and its next size; chains and stars take different paths.
    for n in [64usize, 65] {
        cases.push(((0..n).collect(), None));
        let mut star = vec![1; n];
        star[0] = 0;
        cases.push((star, None));
    }
    // The rooted prefix alone costs 1+...+128 > 4*132 head steps.
    // Thus this must exhaust the budget before reaching tail 129 and cycle {131,132}.
    let mut late_cycle: Vec<_> = (0..132).collect();
    late_cycle[128..].copy_from_slice(&[130, 131, 132, 131]);
    cases.push((late_cycle, Some(129)));
    for (heads, bad) in cases {
        let (a, s) = fixture(&heads);
        let forms = vec!["word".to_string(); heads.len()];
        let mut rows = vec![vec![]; heads.len() + 1];
        for (i, &h) in heads.iter().enumerate() {
            rows[h].push(i + 1);
        }
        let reached = mova_algorithms::bfs::bfs(&rows, 0).unwrap();
        assert_eq!((1..=heads.len()).find(|&i| reached[i].is_none()), bad);
        assert_eq!(
            ann::check(&a, &forms, &reg),
            bad.map_or(Ok(()), |i| Err(ann::Bad::Cycle(i)))
        );
        assert_eq!(
            tree_gate(&s),
            bad.map_or(Ok(()), |i| Err(format!(
                "tree gate: cycle through word {i}"
            )))
        );
    }
    // In [3,0,4,3], word 1 is a tail, while the cycle itself is {3,4}.
    // Projecting the first cyclic vertex (3) would change the real error witness (1).
    let (a, s) = fixture(&[3, 0, 4, 3]);
    assert_ne!(
        ann::check(&a, &vec!["word".into(); 4], &reg),
        Err(ann::Bad::Cycle(3))
    );
    assert_ne!(tree_gate(&s), Err("tree gate: cycle through word 3".into()));
}
#[test]
fn groupant_distinct_empty_self_head_and_error_policies() {
    let reg = Registry::load();
    let (a, s) = fixture(&[]);
    assert_eq!(tree_gate(&s), Ok(()));
    assert_eq!(ann::check(&a, &[], &reg), Err(ann::Bad::Roots(0)));
    let (a, s) = fixture(&[1, 0]);
    let forms = vec!["word".into(); 2];
    assert_eq!(tree_gate(&s), Err("tree gate: cycle through word 1".into()));
    assert_eq!(ann::check(&a, &forms, &reg), Err(ann::Bad::Head(1)));
    let (a, s) = fixture(&[3, 0]);
    assert_eq!(ann::check(&a, &forms, &reg), Err(ann::Bad::Head(1)));
    assert_eq!(
        tree_gate(&s),
        Err("tree gate: word 1: head 3 outside the sentence (2 words)".into())
    );
    // Annotation count/form preflight is outside the convert tree gate's observation.
    let (a, s) = fixture(&[0]);
    assert_eq!(tree_gate(&s), Ok(()));
    assert_eq!(ann::check(&a, &[], &reg), Err(ann::Bad::Count(1, 0)));
    assert_eq!(
        ann::check(&a, &["other".into()], &reg),
        Err(ann::Bad::Form(1))
    );
}
