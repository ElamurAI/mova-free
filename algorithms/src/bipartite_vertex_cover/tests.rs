use super::*;

fn brute_matching(g: &[Vec<usize>], row: usize, used: usize) -> usize {
    if row == g.len() {
        return 0;
    }
    let mut best = brute_matching(g, row + 1, used);
    for &r in &g[row] {
        if used & (1 << r) == 0 {
            best = best.max(1 + brute_matching(g, row + 1, used | (1 << r)));
        }
    }
    best
}

#[test]
fn exhaustive_cover_and_matching_oracles() {
    for l in 0..=3 {
        for r in 0..=3 {
            for edges in 0usize..(1 << (l * r)) {
                let g: Vec<Vec<_>> = (0..l)
                    .map(|u| {
                        (0..r)
                            .filter(|&v| edges & (1 << (u * r + v)) != 0)
                            .collect()
                    })
                    .collect();
                let c = minimum_vertex_cover(&g, r).unwrap();
                assert!(c.left.windows(2).all(|w| w[0] < w[1]));
                assert!(c.right.windows(2).all(|w| w[0] < w[1]));
                assert!(c.left.iter().all(|&u| u < l) && c.right.iter().all(|&v| v < r));
                assert!(
                    g.iter().enumerate().all(|(u, row)| row
                        .iter()
                        .all(|v| c.left.contains(&u) || c.right.contains(v))),
                    "uncovered edge"
                );
                let best = (0usize..(1 << (l + r)))
                    .filter(|&mask| {
                        g.iter().enumerate().all(|(u, row)| {
                            row.iter()
                                .all(|&v| mask & (1 << u) != 0 || mask & (1 << (l + v)) != 0)
                        })
                    })
                    .map(|m| m.count_ones() as usize)
                    .min()
                    .unwrap();
                assert_eq!(c.left.len() + c.right.len(), best, "nonminimum cover");
                assert_eq!(best, brute_matching(&g, 0, 0));
                assert_eq!(c, minimum_vertex_cover(&g, r).unwrap());
            }
        }
    }
}

#[test]
fn alternating_path_duplicate_and_disjoint_witnesses() {
    let g = vec![vec![0, 0], vec![0], vec![1, 2], vec![2], vec![]];
    assert_eq!(
        minimum_vertex_cover(&g, 3).unwrap(),
        Cover {
            left: vec![2, 3],
            right: vec![0]
        }
    );
    assert_eq!(
        minimum_vertex_cover(&[vec![], vec![]], 4).unwrap(),
        Cover {
            left: vec![],
            right: vec![]
        }
    );
}

#[test]
fn validation_and_budget_boundaries() {
    assert_eq!(
        minimum_vertex_cover(&[vec![1]], 1),
        Err(Error::InvalidEdge {
            left: 0,
            right: 1,
            right_vertices: 1
        })
    );
    assert_eq!(
        minimum_vertex_cover(&[vec![usize::MAX]], MAX_VERTICES),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        minimum_vertex_cover(&[vec![usize::MAX; MAX_EDGES + 1]], 0),
        Err(Error::LimitExceeded)
    );
    assert!(
        minimum_vertex_cover(&[], MAX_VERTICES)
            .unwrap()
            .right
            .is_empty()
    );
    assert_eq!(
        minimum_vertex_cover(&[], usize::MAX),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        minimum_vertex_cover(&[vec![0; MAX_EDGES]], 1).unwrap().left,
        vec![0]
    );
}
