use super::{Congruence, Error, solve};

fn c(residue: u64, modulus: u64) -> Congruence {
    Congruence { residue, modulus }
}

fn brute(input: &[Congruence]) -> Option<Congruence> {
    let product: u64 = input.iter().map(|c| c.modulus).product();
    let period = (1..=product)
        .find(|x| input.iter().all(|c| x % c.modulus == 0))
        .unwrap();
    (0..period)
        .find(|x| input.iter().all(|c| x % c.modulus == c.residue % c.modulus))
        .map(|residue| c(residue, period))
}

#[test]
fn exhaustive_small_residue_scan() {
    for m in 1..=10 {
        for n in 1..=10 {
            for a in 0..=m {
                for b in 0..=n {
                    let input = [c(a, m), c(b, n)];
                    assert_eq!(solve(&input), Ok(brute(&input)), "{input:?}");
                }
            }
        }
    }
}

#[test]
fn neutral_normalization_and_non_coprime() {
    assert_eq!(solve(&[]), Ok(Some(c(0, 1))));
    assert_eq!(solve(&[c(u64::MAX, 1)]), Ok(Some(c(0, 1))));
    assert_eq!(solve(&[c(14, 6), c(8, 9)]), Ok(Some(c(8, 18))));
    assert_eq!(solve(&[c(1, 6), c(2, 9)]), Ok(None));
    assert_eq!(solve(&[c(20, 7)]), Ok(Some(c(6, 7))));
}

#[test]
fn full_width_arithmetic_and_error_order() {
    let max = u64::MAX;
    assert_eq!(
        solve(&[c(max - 1, max), c(2, 3)]),
        Ok(Some(c(max - 1, max)))
    );
    assert_eq!(
        solve(&[c(max - 1, max), c(max - 1, max)]),
        Ok(Some(c(max - 1, max)))
    );
    assert_eq!(
        solve(&[c(0, max), c(0, max - 1)]),
        Err(Error::ModulusOverflow)
    );
    assert_eq!(
        solve(&[c(0, 2), c(1, 2), c(0, 0)]),
        Err(Error::ZeroModulus { index: 2 })
    );
    assert_eq!(
        solve(&[c(0, max), c(0, max - 1), c(0, 0)]),
        Err(Error::ZeroModulus { index: 2 })
    );
    assert_eq!(
        solve(&[c(0, max), c(0, max - 1), c(1, 2)]),
        Err(Error::ModulusOverflow)
    );
    assert_eq!(solve(&[c(0, 2), c(1, 2), c(0, max)]), Ok(None));
    assert_eq!(solve(&[c(0, max - 1), c(1, max - 3)]), Ok(None));
    // Large signed delta and inverse residues, but representable total period.
    let m = 4_294_967_291;
    let n = 4_294_967_279;
    let result = solve(&[c(m - 1, m), c(n - 2, n)]).unwrap().unwrap();
    assert_eq!(result.modulus, m * n);
    assert_eq!(result.residue % m, m - 1);
    assert_eq!(result.residue % n, n - 2);
}

#[test]
fn permutations_redundancy_and_solution_fibers() {
    for a in 0..6 {
        for b in 0..8 {
            for d in 0..9 {
                let input = [c(a, 6), c(b, 8), c(d, 9)];
                let expected = brute(&input);
                for perm in [
                    [0, 1, 2],
                    [0, 2, 1],
                    [1, 0, 2],
                    [1, 2, 0],
                    [2, 0, 1],
                    [2, 1, 0],
                ] {
                    assert_eq!(solve(&perm.map(|i| input[i])), Ok(expected));
                }
                assert_eq!(
                    solve(&[input[0], input[1], input[2], input[0], c(0, 1)]),
                    Ok(expected)
                );
                if let Some(output) = expected {
                    for k in 0..4 {
                        let x = output.residue + k * output.modulus;
                        assert!(input.iter().all(|c| x % c.modulus == c.residue));
                    }
                }
            }
        }
    }
}

#[test]
fn reverse_input_fibers_are_ambiguous() {
    let output = Some(c(8, 18));
    assert_eq!(solve(&[c(8, 18)]), Ok(output));
    assert_eq!(solve(&[c(2, 6), c(8, 9)]), Ok(output));
    assert_eq!(solve(&[c(20, 6), c(26, 9)]), Ok(output));
    assert_eq!(solve(&[c(8, 9)]), Ok(Some(c(8, 9))));
}
