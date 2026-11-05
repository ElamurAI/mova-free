use super::*;
fn graph(n: usize, mask: usize) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; n];
    let mut bit = 0;
    for u in 0..n {
        for v in u + 1..n {
            if mask >> bit & 1 != 0 {
                g[u].push(v);
                g[v].push(u);
            }
            bit += 1;
        }
    }
    g
}
#[test]
fn validation() {
    assert_eq!(matrix(&vec![vec![]; 257], true), Err(Error::LimitExceeded));
    assert_eq!(matrix(&[vec![1]], true), Err(Error::InvalidVertex));
    for g in [
        vec![vec![0]],
        vec![vec![1], vec![]],
        vec![vec![1, 1], vec![0]],
    ] {
        assert_eq!(matrix(&g, true), Err(Error::NotSimpleUndirected));
    }
}

#[test]
fn exhaustive_subsets() {
    for mask in 0..1024 {
        let g = graph(5, mask);
        for k in 0..=6 {
            let actual = k_core(&g, k).unwrap();
            let mut union = 0usize;
            for subset in 0usize..32 {
                if (0..5).all(|v| {
                    subset >> v & 1 == 0
                        || g[v].iter().filter(|&&w| subset >> w & 1 != 0).count() >= k
                }) {
                    union |= subset;
                }
            }
            assert_eq!(
                actual,
                (0..5).filter(|&v| union >> v & 1 != 0).collect::<Vec<_>>(),
                "maximal core"
            );
        }
    }
    assert_eq!(k_core(&[vec![]], usize::MAX).unwrap(), Vec::<usize>::new());
    assert_eq!(k_core(&[], 0).unwrap(), Vec::<usize>::new());
}
