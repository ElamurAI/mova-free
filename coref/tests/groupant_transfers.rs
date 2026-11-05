// Independent cross-groupant fixtures, 2026-10-30.
// External integration test using the actual coref public APIs.
use coref::corefud::Span;
use coref::score::{MSpan, Matching, assign_max, match_mentions};
fn mention(start: usize, end: usize) -> MSpan {
    MSpan {
        span: Span {
            sent: 0,
            start,
            end,
        },
        head: 0,
    }
}
#[test]
fn groupant_reject_global_assignment_as_greedy_replacement() {
    // Residual overlap matrix starts [[5,4],[4,0]] (prediction rows).
    // All spans differ; 126 zero-overlap fillers activate the 128-side seam.
    let mut gold = vec![mention(0, 4), mention(4, 7)];
    let mut pred = vec![mention(0, 7), mention(0, 3)];
    for i in 0..126 {
        gold.push(mention(100 + i * 2, 100 + i * 2));
        pred.push(mention(1000 + i * 2, 1000 + i * 2));
    }
    let got = match_mentions(&gold, &pred, Matching::Head);
    assert_eq!(got, (0..128).map(Some).collect::<Vec<_>>());
    assert_eq!(assign_max(&[vec![5.0, 4.0], vec![4.0, 0.0]]), 8.0);
    // The greedy mapping has total 5; maximizing globally would change identities.
    assert_eq!(got[0], Some(0));
    assert_eq!(got[1], Some(1));
}
#[test]
fn groupant_exact_match_priority_survives_large_residual_group() {
    let mut gold: Vec<_> = (0..129).map(|i| mention(i * 10, i * 10 + 2)).collect();
    let mut pred: Vec<_> = (0..129).map(|i| mention(i * 10 + 1, i * 10 + 3)).collect();
    gold.insert(4, mention(5000, 5005));
    gold.insert(7, mention(5000, 5005));
    pred.insert(2, mention(5000, 5005));
    pred.insert(9, mention(5000, 5005));
    let got = match_mentions(&gold, &pred, Matching::Head);
    assert_eq!(got[2], Some(4));
    assert_eq!(got[9], Some(7));
    let used: std::collections::BTreeSet<_> = got.iter().flatten().copied().collect();
    assert_eq!(used.len(), 131);
    assert_eq!(
        match_mentions(&gold, &pred, Matching::Exact)
            .iter()
            .flatten()
            .count(),
        2
    );
}
#[test]
fn groupant_assignment_transpose_negative_and_empty_boundaries() {
    assert_eq!(assign_max(&[]), 0.0);
    assert_eq!(assign_max(&[vec![]]), 0.0);
    assert_eq!(
        assign_max(&[vec![-8.0, -2.0, -5.0], vec![-3.0, -9.0, -4.0]]),
        -5.0
    );
    assert_eq!(
        assign_max(&[vec![-8.0, -3.0], vec![-2.0, -9.0], vec![-5.0, -4.0]]),
        -5.0
    );
}

#[test]
fn groupant_total_key_zero_ties_join_heap_sort() {
    use std::cmp::Reverse;
    let gold: Vec<_> = (0..128).map(|i| mention(i * 2, i * 2)).collect();
    let pred: Vec<_> = (0..129)
        .map(|i| mention(1000 + i * 2, 1000 + i * 2))
        .collect();
    let mut keys: Vec<_> = (0..129)
        .rev()
        .flat_map(|p| (0..128).rev().map(move |g| (Reverse(0usize), p, g)))
        .collect();
    mova_algorithms::heap_sort::heap_sort(&mut keys);
    let expected_keys: Vec<_> = (0..129)
        .flat_map(|p| (0..128).map(move |g| (Reverse(0usize), p, g)))
        .collect();
    assert_eq!(keys, expected_keys);
    assert_eq!(
        match_mentions(&gold, &pred, Matching::Head),
        (0..129).map(|i| (i < 128).then_some(i)).collect::<Vec<_>>()
    );
    // Explicitly below the side threshold: no exact spans or positive overlaps.
    assert_eq!(
        match_mentions(&gold[..127], &pred, Matching::Head),
        (0..129).map(|i| (i < 127).then_some(i)).collect::<Vec<_>>()
    );
}
#[test]
fn groupant_integer_assignment_relation_and_noninteger_rejection() {
    // All 3^4 matrices over {-1,0,1}: exact integer objective, not witness equality.
    for mut code in 0..81usize {
        let mut w = vec![vec![0.0; 2]; 2];
        for row in &mut w {
            for x in row {
                *x = (code % 3) as f64 - 1.0;
                code /= 3;
            }
        }
        let costs: Vec<Vec<i64>> = w
            .iter()
            .map(|r| r.iter().map(|x| -(*x as i64)).collect())
            .collect();
        let exact = mova_algorithms::hungarian::hungarian(&costs).unwrap();
        let independent = (w[0][0] + w[1][1]).max(w[0][1] + w[1][0]);
        assert_eq!(assign_max(&w), independent);
        assert_eq!(-(exact.cost as f64), independent);
    }
    let fractional = vec![vec![0.75, 0.5], vec![0.5, 0.0]];
    assert_eq!(assign_max(&fractional), 1.0);
    let coerced: Vec<Vec<i64>> = fractional
        .iter()
        .map(|r| r.iter().map(|x| -(*x as i64)).collect())
        .collect();
    assert_eq!(
        mova_algorithms::hungarian::hungarian(&coerced)
            .unwrap()
            .cost,
        0
    );
    assert_ne!(assign_max(&fractional), 0.0); // lossy coercion is not an admissible alpha
}
