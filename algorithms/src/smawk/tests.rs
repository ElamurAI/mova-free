use super::*;
#[test]
fn lawful_matrices() {
    for rows in 1..20 {
        for cols in 1..20 {
            for slope in 0..4 {
                let f = |r: usize, c: usize| {
                    let d = 2 * c as i128 - (r * slope) as i128;
                    d * d
                };
                let expected = (0..rows)
                    .map(|r| (0..cols).min_by_key(|&c| f(r, c)).unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(row_minima(rows, cols, 10000, f), Ok(expected));
            }
        }
    }
}
#[test]
fn ties_budgets_and_linear_calls() {
    assert_eq!(row_minima(8, 9, 1000, |_, _| 0), Ok(vec![0; 8]));
    assert_eq!(row_minima(0, 0, 0, |_, _| panic!()), Ok(vec![]));
    assert_eq!(row_minima(1, 0, 0, |_, _| 0), Err(Error::EmptyColumns));
    let mut calls = 0;
    assert_eq!(
        row_minima(1, 1, 0, |_, _| {
            calls += 1;
            0
        }),
        Err(Error::BudgetExceeded)
    );
    assert_eq!(calls, 0);
    let mut calls = 0;
    let result = row_minima(128, 256, 10000, |r, c| {
        calls += 1;
        (r as i128 - c as i128).pow(2)
    })
    .unwrap();
    assert_eq!(result, (0..128).collect::<Vec<_>>());
    assert!(calls < 20 * (128 + 256));
}
