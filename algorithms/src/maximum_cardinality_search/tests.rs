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
fn exhaustive_priority() {
    assert!(maximum_cardinality_search(&[]).unwrap().is_empty());
    for mask in 0..1024 {
        let g = graph(5, mask);
        let order = maximum_cardinality_search(&g).unwrap();
        let mut prefix: Vec<usize> = vec![];
        for &v in &order {
            let count = |w: usize| g[w].iter().filter(|x| prefix.contains(x)).count();
            for w in 0..5 {
                if !prefix.contains(&w) {
                    assert!(
                        count(v) > count(w) || (count(v) == count(w) && v <= w),
                        "priority"
                    );
                }
            }
            prefix.push(v);
        }
        assert_eq!(prefix.len(), 5);
        let mut reversed = g.clone();
        for row in &mut reversed {
            row.reverse();
        }
        assert_eq!(maximum_cardinality_search(&reversed).unwrap(), order);
    }
    assert_eq!(
        maximum_cardinality_search(&vec![vec![]; 256]).unwrap(),
        (0..256).collect::<Vec<_>>()
    );
}
