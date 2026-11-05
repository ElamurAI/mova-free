use super::*;
fn lit(variable: usize, positive: bool) -> Literal {
    Literal { variable, positive }
}
fn satisfies(clauses: &[(Literal, Literal)], values: &[bool]) -> bool {
    clauses
        .iter()
        .all(|&(a, b)| values[a.variable] == a.positive || values[b.variable] == b.positive)
}
#[test]
fn exhaustive_truth_table_oracle() {
    let literals = [lit(0, false), lit(0, true), lit(1, false), lit(1, true)];
    let mut universe = Vec::new();
    for i in 0..4 {
        for j in i..4 {
            universe.push((literals[i], literals[j]));
        }
    }
    for mask in 0..1024 {
        let clauses: Vec<_> = universe
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &c)| c)
            .collect();
        let expected = (0..4).any(|bits| satisfies(&clauses, &[bits & 1 != 0, bits & 2 != 0]));
        let result = solve(2, &clauses).unwrap();
        assert_eq!(result.is_some(), expected, "mask={mask}");
        if let Some(values) = result {
            assert_eq!(values.len(), 2);
            assert!(satisfies(&clauses, &values));
        }
    }
}
#[test]
fn empty_unit_contradiction_and_tautology() {
    assert_eq!(solve(0, &[]), Ok(Some(vec![])));
    assert_eq!(solve(2, &[]), Ok(Some(vec![true, true])));
    let x = lit(0, true);
    let nx = lit(0, false);
    assert_eq!(solve(1, &[(x, x)]), Ok(Some(vec![true])));
    assert_eq!(solve(1, &[(nx, nx)]), Ok(Some(vec![false])));
    assert_eq!(solve(1, &[(x, x), (nx, nx)]), Ok(None));
    assert!(solve(1, &[(x, nx)]).unwrap().is_some());
}
#[test]
fn validates_before_graph_allocation() {
    assert_eq!(solve(usize::MAX, &[]), Err(Error::TooManyVariables));
    assert_eq!(
        solve(0, &[(lit(0, true), lit(1, false))]),
        Err(Error::InvalidVariable {
            clause: 0,
            variable: 0,
            variables: 0
        })
    );
    assert_eq!(
        solve(
            2,
            &[(lit(0, true), lit(1, false)), (lit(1, true), lit(4, false))]
        ),
        Err(Error::InvalidVariable {
            clause: 1,
            variable: 4,
            variables: 2
        })
    );
}
#[test]
fn canonical_components_require_topological_ranks() {
    let clauses = [
        (lit(0, false), lit(0, false)),
        (lit(0, true), lit(1, false)),
        (lit(1, true), lit(2, true)),
    ];
    assert_eq!(solve(3, &clauses), Ok(Some(vec![false, false, true])));
}
#[test]
fn clause_permutation_duplication_and_negation() {
    let clauses = vec![
        (lit(0, true), lit(1, true)),
        (lit(0, false), lit(1, true)),
        (lit(1, false), lit(2, true)),
    ];
    let original = solve(3, &clauses).unwrap().unwrap();
    let mut reordered = clauses.clone();
    reordered.reverse();
    reordered.extend(clauses.clone());
    assert_eq!(solve(3, &reordered), Ok(Some(original)));
    let flipped: Vec<_> = clauses
        .iter()
        .map(|&(a, b)| (lit(a.variable, !a.positive), lit(b.variable, !b.positive)))
        .collect();
    let result = solve(3, &flipped).unwrap().unwrap();
    let unflipped: Vec<_> = result.into_iter().map(|b| !b).collect();
    assert!(satisfies(&clauses, &unflipped));
}
#[test]
fn iterative_long_implication_chain() {
    let n = 2000;
    let mut clauses = vec![(lit(0, true), lit(0, true))];
    clauses.extend((1..n).map(|v| (lit(v - 1, false), lit(v, true))));
    assert_eq!(solve(n, &clauses), Ok(Some(vec![true; n])));
    clauses.push((lit(n - 1, false), lit(n - 1, false)));
    assert_eq!(solve(n, &clauses), Ok(None));
}
