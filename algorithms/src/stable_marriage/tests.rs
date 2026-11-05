use super::*;

fn permutations(n: usize) -> Vec<Vec<usize>> {
    fn visit(prefix: &mut Vec<usize>, n: usize, out: &mut Vec<Vec<usize>>) {
        if prefix.len() == n {
            out.push(prefix.clone());
            return;
        }
        for x in 0..n {
            if !prefix.contains(&x) {
                prefix.push(x);
                visit(prefix, n, out);
                prefix.pop();
            }
        }
    }
    let mut out = vec![];
    visit(&mut vec![], n, &mut out);
    out
}
fn position(row: &[usize], x: usize) -> usize {
    row.iter().position(|&v| v == x).unwrap()
}
fn stable(p: &[Vec<usize>], r: &[Vec<usize>], m: &[usize]) -> bool {
    for a in 0..p.len() {
        for (b, receiver_order) in r.iter().enumerate() {
            let incumbent = m.iter().position(|&v| v == b).unwrap();
            if position(&p[a], b) < position(&p[a], m[a])
                && position(receiver_order, a) < position(receiver_order, incumbent)
            {
                return false;
            }
        }
    }
    true
}
#[test]
fn exhaustive_stability_and_proposer_optimality() {
    for n in 0..=3 {
        let rows = permutations(n);
        let total = rows.len().pow((2 * n) as u32);
        for mut code in 0..total {
            let mut profile = Vec::new();
            for _ in 0..2 * n {
                profile.push(rows[code % rows.len()].clone());
                code /= rows.len();
            }
            let (p, r) = profile.split_at(n);
            let m = stable_matching(p, r).unwrap();
            assert!(rows.contains(&m), "not a permutation");
            assert!(stable(p, r, &m), "blocking pair");
            for alternative in &rows {
                if stable(p, r, alternative) {
                    for a in 0..n {
                        assert!(
                            position(&p[a], m[a]) <= position(&p[a], alternative[a]),
                            "not proposer optimal"
                        );
                    }
                }
            }
            assert_eq!(m, stable_matching(p, r).unwrap());
        }
    }
}
#[test]
fn receiver_replacement_witness() {
    let p = vec![vec![0, 1], vec![0, 1]];
    let r = vec![vec![1, 0], vec![0, 1]];
    assert_eq!(stable_matching(&p, &r).unwrap(), vec![1, 0]);
}
#[test]
fn validation_precedence_and_budget() {
    assert_eq!(stable_matching(&[vec![]], &[]), Err(Error::SizeMismatch));
    let too_many = vec![vec![]; 1001];
    assert_eq!(
        stable_matching(&too_many, &too_many),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        stable_matching(&[vec![]], &[vec![2]]),
        Err(Error::InvalidRowLength {
            side: Side::Proposer,
            row: 0
        })
    );
    assert_eq!(
        stable_matching(&[vec![2]], &[vec![]]),
        Err(Error::InvalidPreference {
            side: Side::Proposer,
            row: 0,
            position: 0,
            value: 2
        })
    );
    let p = vec![vec![0, 1], vec![0, 1]];
    assert_eq!(
        stable_matching(&p, &[vec![1, 1], vec![0, 1]]),
        Err(Error::InvalidPreference {
            side: Side::Receiver,
            row: 0,
            position: 1,
            value: 1
        })
    );
    assert_eq!(
        stable_matching(&[vec![0]], &[vec![]]),
        Err(Error::InvalidRowLength {
            side: Side::Receiver,
            row: 0
        })
    );
    let boundary = vec![(0..1000).collect::<Vec<_>>(); 1000];
    assert_eq!(
        stable_matching(&boundary, &boundary).unwrap(),
        (0..1000).collect::<Vec<_>>()
    );
}
