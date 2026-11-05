use super::*;

// Independent Floyd-Warshall oracle, including a global diagonal cycle test.
fn oracle(g: &[Vec<(usize, i64)>]) -> Result<Vec<Vec<Option<i128>>>, Error> {
    let n = g.len();
    let mut d = vec![vec![None; n]; n];
    for u in 0..n {
        d[u][u] = Some(0);
        for &(v, w) in &g[u] {
            d[u][v] = Some(d[u][v].unwrap_or(i128::MAX).min(i128::from(w)));
        }
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                if let (Some(a), Some(b)) = (d[u][k], d[k][v]) {
                    d[u][v] = Some(d[u][v].unwrap_or(i128::MAX).min(a + b));
                }
            }
        }
    }
    if (0..n).any(|u| d[u][u].unwrap() < 0) {
        Err(Error::NegativeCycle)
    } else {
        Ok(d)
    }
}
#[test]
fn exhaustive_signed_graphs() {
    for n in 0..=3usize {
        for mut code in 0..3usize.pow((n * n) as u32) {
            let mut g = vec![vec![]; n];
            for row in &mut g {
                for v in 0..n {
                    match code % 3 {
                        1 => row.push((v, -1)),
                        2 => row.push((v, 2)),
                        _ => {}
                    }
                    code /= 3;
                }
            }
            assert_eq!(johnson(&g), oracle(&g), "{g:?}");
        }
    }
}
#[test]
fn global_validation_and_cycles() {
    assert_eq!(johnson(&[vec![], vec![(1, -1)]]), Err(Error::NegativeCycle));
    assert_eq!(
        johnson(&[vec![(0, -1)], vec![(2, 0)]]),
        Err(Error::InvalidEdge {
            from: 1,
            to: 2,
            vertices: 2
        })
    );
    assert_eq!(johnson(&vec![vec![]; 1025]), Err(Error::ResourceLimit));
    assert_eq!(
        johnson(&[vec![(0, 0); 1_000_001]]),
        Err(Error::ResourceLimit)
    );
}
#[test]
fn wide_reweighting_parallel_and_telescoping() {
    // The 0->4 reweighted edge exceeds u64::MAX.
    let g = vec![
        vec![(1, i64::MIN), (4, i64::MAX), (4, i64::MAX)],
        vec![(2, i64::MIN)],
        vec![(3, i64::MIN)],
        vec![(4, 0)],
        vec![],
    ];
    assert_eq!(johnson(&g), oracle(&g));
    assert_eq!(johnson(&g).unwrap()[0][4], Some(3 * i128::from(i64::MIN)));
    let mut reordered = g.clone();
    reordered[0].reverse();
    assert_eq!(johnson(&g), johnson(&reordered));
}
#[test]
fn potential_shift_and_information_loss() {
    let g = vec![vec![(1, 2), (2, 7)], vec![(2, 3)], vec![]];
    let p = [3i64, -4, 8];
    let shifted: Vec<_> = g
        .iter()
        .enumerate()
        .map(|(u, row)| row.iter().map(|&(v, w)| (v, w + p[v] - p[u])).collect())
        .collect();
    let a = johnson(&g).unwrap();
    let b = johnson(&shifted).unwrap();
    for u in 0..3 {
        for v in 0..3 {
            assert_eq!(b[u][v], a[u][v].map(|x| x + i128::from(p[v] - p[u])));
        }
    }
    let mut redundant = g.clone();
    redundant[0].push((2, 100));
    assert_eq!(johnson(&g), johnson(&redundant));
}
