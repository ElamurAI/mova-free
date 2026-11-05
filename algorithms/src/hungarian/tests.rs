use super::*;

fn enumerate(costs: &[Vec<i64>], used: &mut [bool], row: usize, sum: i128) -> i128 {
    if row == costs.len() {
        return sum;
    }
    let mut best = i128::MAX;
    for col in 0..used.len() {
        if !used[col] {
            used[col] = true;
            best = best.min(enumerate(
                costs,
                used,
                row + 1,
                sum + i128::from(costs[row][col]),
            ));
            used[col] = false;
        }
    }
    best
}

fn check(costs: &[Vec<i64>]) {
    let result = hungarian(costs).unwrap();
    let cols = costs.first().map_or(0, Vec::len);
    assert_eq!(result.cost, enumerate(costs, &mut vec![false; cols], 0, 0));
    assert_eq!(result.columns.len(), costs.len());
    let mut seen = vec![false; cols];
    let mut sum = 0;
    for (row, &col) in result.columns.iter().enumerate() {
        assert!(!seen[col]);
        seen[col] = true;
        sum += i128::from(costs[row][col]);
        assert_eq!(
            result.row_potentials[row] + result.column_potentials[col],
            i128::from(costs[row][col])
        );
    }
    assert_eq!(sum, result.cost);
    for (row, costs) in costs.iter().enumerate() {
        for (col, &cost) in costs.iter().enumerate() {
            assert!(result.row_potentials[row] + result.column_potentials[col] <= i128::from(cost));
        }
    }
    for (col, &potential) in result.column_potentials.iter().enumerate() {
        assert!(potential <= 0);
        if !seen[col] {
            assert_eq!(potential, 0);
        }
    }
    assert_eq!(
        result.cost,
        result.row_potentials.iter().sum::<i128>() + result.column_potentials.iter().sum::<i128>()
    );
    assert_eq!(result, hungarian(costs).unwrap());
}

#[test]
fn exhaustive_small_assignments() {
    for rows in 1..=2 {
        for cols in rows..=3 {
            for mut code in 0..3usize.pow((rows * cols) as u32) {
                let mut costs = vec![vec![0; cols]; rows];
                for row in &mut costs {
                    for cost in row {
                        *cost = (code % 3) as i64 - 1;
                        code /= 3;
                    }
                }
                check(&costs);
            }
        }
    }
    for seed in 0..64 {
        let costs: Vec<_> = (0..4)
            .map(|r| {
                (0..5)
                    .map(|c| ((seed * 13 + r * 7 + c * c * 3 + r * c) % 23) as i64 - 11)
                    .collect()
            })
            .collect();
        check(&costs);
    }
}

#[test]
fn boundaries_ties_and_extremes() {
    check(&[]);
    check(&[vec![i64::MIN, i64::MAX], vec![i64::MAX, i64::MIN]]);
    check(&[vec![i64::MAX; 3], vec![i64::MAX; 3]]);
    assert_eq!(
        hungarian(&[vec![0; 3], vec![0; 3]]).unwrap().columns,
        vec![0, 1]
    );
    check(&[vec![10, 1, 2], vec![0, 8, 9], vec![3, 4, 0]]);
}

#[test]
fn error_precedence_and_work_bound() {
    assert_eq!(
        hungarian(&vec![vec![]; MAX_ROWS + 1]),
        Err(Error::TooManyRows)
    );
    assert_eq!(
        hungarian(&[vec![0; MAX_COLUMNS + 1], vec![]]),
        Err(Error::TooManyColumns)
    );
    assert_eq!(hungarian(&[vec![0], vec![]]), Err(Error::RaggedMatrix));
    assert_eq!(hungarian(&[vec![]]), Err(Error::TooFewColumns));
    assert_eq!(
        hungarian(&vec![vec![0; MAX_ROWS]; MAX_ROWS]),
        Err(Error::WorkLimit)
    );
}
