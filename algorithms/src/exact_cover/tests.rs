use super::*;
#[test]
fn exhaustive_row_families() {
    for columns in 0..=3 {
        let types = 1usize << columns;
        for family in 0..1usize << types {
            let rows: Vec<Vec<usize>> = (0..types)
                .filter(|&m| family & (1 << m) != 0)
                .map(|m| (0..columns).filter(|&c| m & (1 << c) != 0).collect())
                .collect();
            let mut feasible = false;
            for subset in 0..1usize << rows.len() {
                let mut counts = vec![0; columns];
                for (r, row) in rows.iter().enumerate() {
                    if subset & (1 << r) != 0 {
                        for &c in row {
                            counts[c] += 1;
                        }
                    }
                }
                if counts.iter().all(|&x| x == 1) {
                    feasible = true;
                    break;
                }
            }
            let got = solve(columns, &rows, 100_000).unwrap();
            assert_eq!(got.is_some(), feasible);
            if let Some(chosen) = got {
                let mut counts = vec![0; columns];
                for r in chosen {
                    for &c in &rows[r] {
                        counts[c] += 1;
                    }
                }
                assert!(counts.iter().all(|&x| x == 1));
            }
        }
    }
}
#[test]
fn budget_ties_and_boundaries() {
    assert_eq!(
        solve(2, &[vec![0], vec![1], vec![0, 1]], 3),
        Ok(Some(vec![0, 1]))
    );
    assert_eq!(solve(0, &[], 0), Err(Error::ResourceExhausted));
    assert_eq!(solve(0, &[], 1), Ok(Some(vec![])));
    assert_eq!(solve(1, &[], 1), Ok(None));
    assert_eq!(solve(1, &[vec![0]], 1), Err(Error::ResourceExhausted));
    assert_eq!(solve(64, &[(0..64).collect()], 2), Ok(Some(vec![0])));
    assert_eq!(solve(65, &[], 0), Err(Error::LimitExceeded));
    assert_eq!(
        solve(1, &[vec![0, 0]], 0),
        Err(Error::DuplicateColumn { row: 0, column: 0 })
    );
    assert_eq!(
        solve(0, &[vec![0]], 0),
        Err(Error::InvalidColumn { row: 0, column: 0 })
    );
}
