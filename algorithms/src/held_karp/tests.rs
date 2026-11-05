use super::*;

fn permutations(
    arcs: &[Vec<Option<u64>>],
    remaining: &mut [usize],
    at: usize,
    best: &mut Option<u128>,
) {
    if at == remaining.len() {
        let mut last = 0;
        let mut sum = 0;
        for next in remaining.iter().copied().chain(std::iter::once(0)) {
            let Some(cost) = arcs[last][next] else {
                return;
            };
            sum += u128::from(cost);
            last = next;
        }
        if best.is_none_or(|value| sum < value) {
            *best = Some(sum);
        }
        return;
    }
    for i in at..remaining.len() {
        remaining.swap(at, i);
        permutations(arcs, remaining, at + 1, best);
        remaining.swap(at, i);
    }
}

fn check(arcs: &[Vec<Option<u64>>]) {
    let result = held_karp(arcs).unwrap();
    let mut expected = None;
    if arcs.len() <= 1 {
        expected = Some(0);
    } else {
        permutations(
            arcs,
            &mut (1..arcs.len()).collect::<Vec<_>>(),
            0,
            &mut expected,
        );
    }
    assert_eq!(result.as_ref().map(|tour| tour.cost), expected);
    if let Some(tour) = &result {
        if arcs.is_empty() {
            assert!(tour.vertices.is_empty());
        } else {
            assert_eq!(tour.vertices.len(), arcs.len() + 1);
            assert_eq!(tour.vertices.first(), Some(&0));
            assert_eq!(tour.vertices.last(), Some(&0));
            let mut inner = tour.vertices[..arcs.len()].to_vec();
            inner.sort();
            assert_eq!(inner, (0..arcs.len()).collect::<Vec<_>>());
            if arcs.len() > 1 {
                let sum: u128 = tour
                    .vertices
                    .windows(2)
                    .map(|edge| u128::from(arcs[edge[0]][edge[1]].unwrap()))
                    .sum();
                assert_eq!(sum, tour.cost);
            }
        }
    }
    assert_eq!(result, held_karp(arcs).unwrap());
}

#[test]
fn exhaustive_directed_triangles() {
    for mut code in 0..3usize.pow(6) {
        let mut arcs = vec![vec![None; 3]; 3];
        for (i, row) in arcs.iter_mut().enumerate() {
            for (j, edge) in row.iter_mut().enumerate() {
                if i != j {
                    *edge = match code % 3 {
                        0 => None,
                        1 => Some(0),
                        _ => Some(7),
                    };
                    code /= 3;
                }
            }
        }
        check(&arcs);
    }
    for n in 2..=7 {
        for seed in 0..12 {
            let arcs: Vec<_> = (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| {
                            if (i * 7 + j * 11 + seed) % 5 == 0 {
                                None
                            } else {
                                Some(((i * 19 + j * 3 + seed * 13) % 29) as u64)
                            }
                        })
                        .collect()
                })
                .collect();
            check(&arcs);
        }
    }
}

#[test]
fn empty_singleton_ties_and_wide_cost() {
    check(&[]);
    check(&[vec![None]]);
    check(&[vec![Some(u64::MAX)]]);
    check(&vec![vec![Some(u64::MAX); 4]; 4]);
    let tie = held_karp(&vec![vec![Some(0); 4]; 4]).unwrap().unwrap();
    assert_eq!(tie.vertices, vec![0, 3, 2, 1, 0]);
    check(&[vec![None, Some(2)], vec![None, None]]);
}

#[test]
fn shape_cap_and_cap_boundary() {
    assert_eq!(
        held_karp(&vec![vec![]; MAX_VERTICES + 1]),
        Err(Error::TooManyVertices)
    );
    assert_eq!(held_karp(&[vec![]]), Err(Error::NonSquareMatrix));
    assert_eq!(
        held_karp(&[vec![None; 2], vec![]]),
        Err(Error::NonSquareMatrix)
    );
    let mut cycle = vec![vec![None; MAX_VERTICES]; MAX_VERTICES];
    for (i, row) in cycle.iter_mut().enumerate() {
        row[(i + 1) % MAX_VERTICES] = Some(1);
    }
    let result = held_karp(&cycle).unwrap().unwrap();
    assert_eq!(result.cost, MAX_VERTICES as u128);
    assert_eq!(
        result.vertices,
        (0..MAX_VERTICES)
            .chain(std::iter::once(0))
            .collect::<Vec<_>>()
    );
}
