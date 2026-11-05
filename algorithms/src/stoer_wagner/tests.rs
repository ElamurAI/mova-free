use super::*;
fn weight(g: &[Vec<u64>], side: &[usize]) -> u128 {
    let mut sum = 0;
    for &from in side {
        for (to, &capacity) in g[from].iter().enumerate() {
            if !side.contains(&to) {
                sum += u128::from(capacity);
            }
        }
    }
    sum
}
fn oracle(g: &[Vec<u64>]) -> u128 {
    (1usize..(1 << g.len()) - 1)
        .filter(|mask| mask & 1 != 0)
        .map(|mask| {
            let side: Vec<_> = (0..g.len()).filter(|v| mask & (1 << v) != 0).collect();
            weight(g, &side)
        })
        .min()
        .unwrap()
}
fn check(g: &[Vec<u64>]) {
    let cut = minimum_cut(g).unwrap().unwrap();
    assert_eq!(cut.weight, oracle(g), "graph={g:?}");
    assert!(!cut.side.is_empty() && cut.side.len() < g.len());
    assert_eq!(cut.side[0], 0);
    assert!(cut.side.windows(2).all(|e| e[0] < e[1]));
    assert_eq!(weight(g, &cut.side), cut.weight);
}
#[test]
fn exhaustive_partition_oracle() {
    let mut cases = 0;
    for (n, radix, count) in [(4, 3u64, 729), (5, 2u64, 1024)] {
        for code in 0..count {
            let mut digits = code;
            let mut g = vec![vec![0; n]; n];
            for from in 0..n {
                let (prefix, suffix) = g.split_at_mut(from + 1);
                let row = &mut prefix[from];
                for (offset, other) in suffix.iter_mut().enumerate() {
                    let w = digits % radix;
                    digits /= radix;
                    row[from + 1 + offset] = w;
                    other[from] = w;
                }
            }
            check(&g);
            cases += 1;
        }
    }
    assert_eq!(cases, 1753);
}
#[test]
fn empty_singleton_and_invalid_matrices() {
    assert_eq!(minimum_cut(&[]), Ok(None));
    assert_eq!(minimum_cut(&[vec![u64::MAX]]), Ok(None));
    assert_eq!(
        minimum_cut(&[vec![]]),
        Err(Error::NonSquare {
            row: 0,
            expected: 1,
            actual: 0
        })
    );
    assert_eq!(
        minimum_cut(&[vec![0, 4], vec![3]]),
        Err(Error::NonSquare {
            row: 1,
            expected: 2,
            actual: 1
        })
    );
    assert_eq!(
        minimum_cut(&[vec![0, 4], vec![3, 0]]),
        Err(Error::Asymmetric { from: 0, to: 1 })
    );
}
#[test]
fn wide_totals_and_ignored_diagonals() {
    let g = vec![vec![u64::MAX; 3]; 3];
    let cut = minimum_cut(&g).unwrap().unwrap();
    assert_eq!(cut.weight, 2 * u128::from(u64::MAX));
    check(&g);
    let mut without = g.clone();
    for (i, row) in without.iter_mut().enumerate() {
        row[i] = 0;
    }
    assert_eq!(minimum_cut(&g), minimum_cut(&without));
}
#[test]
fn disconnected_ties_and_contraction_witness() {
    assert_eq!(
        minimum_cut(&[vec![0, 0], vec![0, 0]]),
        Ok(Some(Cut {
            weight: 0,
            side: vec![0]
        }))
    );
    let g = vec![
        vec![0, 8, 1, 0],
        vec![8, 0, 0, 1],
        vec![1, 0, 0, 9],
        vec![0, 1, 9, 0],
    ];
    assert_eq!(
        minimum_cut(&g),
        Ok(Some(Cut {
            weight: 2,
            side: vec![0, 1]
        }))
    );
    let triangle = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    assert_eq!(
        minimum_cut(&triangle),
        Ok(Some(Cut {
            weight: 2,
            side: vec![0]
        }))
    );
}
#[test]
fn scaling_monotonicity_and_relabeling_objective() {
    let g = vec![vec![0, 2, 3], vec![2, 0, 4], vec![3, 4, 0]];
    let base = minimum_cut(&g).unwrap().unwrap();
    let scaled: Vec<Vec<_>> = g
        .iter()
        .map(|row| row.iter().map(|w| 3 * w).collect())
        .collect();
    assert_eq!(
        minimum_cut(&scaled).unwrap().unwrap().weight,
        3 * base.weight
    );
    let mut raised = g.clone();
    raised[0][1] += 5;
    raised[1][0] += 5;
    assert!(minimum_cut(&raised).unwrap().unwrap().weight >= base.weight);
    let p = [2, 0, 1];
    let permuted: Vec<Vec<_>> = p
        .iter()
        .map(|&i| p.iter().map(|&j| g[i][j]).collect())
        .collect();
    assert_eq!(minimum_cut(&permuted).unwrap().unwrap().weight, base.weight);
}
#[test]
fn output_fibers_discard_loop_capacity() {
    let a = vec![vec![0, 7], vec![7, 0]];
    let b = vec![vec![8, 7], vec![7, 9]];
    assert_ne!(a, b);
    assert_eq!(minimum_cut(&a), minimum_cut(&b));
    assert_eq!(minimum_cut(&a).unwrap().unwrap().weight, 7);
}
