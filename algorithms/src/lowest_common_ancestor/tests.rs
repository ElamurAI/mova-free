use super::*;

fn chain(parents: &[Option<usize>], mut vertex: usize) -> Option<Vec<usize>> {
    let mut result = Vec::new();
    loop {
        if result.contains(&vertex) {
            return None;
        }
        result.push(vertex);
        match parents[vertex] {
            Some(parent) => vertex = parent,
            None => return Some(result),
        }
    }
}

fn oracle(parents: &[Option<usize>], a: usize, b: usize) -> Option<usize> {
    let left = chain(parents, a).unwrap();
    let right = chain(parents, b).unwrap();
    left.into_iter().find(|v| right.contains(v))
}

#[test]
fn exhaustive_parent_functions_and_ancestor_oracle() {
    for n in 1usize..=5 {
        for mut code in 0..(n + 1).pow(n as u32) {
            let mut parents = vec![None; n];
            for parent in &mut parents {
                let digit = code % (n + 1);
                code /= n + 1;
                *parent = (digit < n).then_some(digit);
            }
            let valid = (0..n).all(|v| chain(&parents, v).is_some());
            let result = Lca::new(&parents);
            assert_eq!(result.is_ok(), valid);
            if let Ok(lca) = result {
                for a in 0..n {
                    for b in 0..n {
                        let answer = lca.query(a, b).unwrap();
                        assert_eq!(answer, oracle(&parents, a, b));
                        assert_eq!(answer, lca.query(b, a).unwrap());
                        if let Some(c) = answer {
                            assert_eq!(lca.query(a, c).unwrap(), Some(c));
                            assert_eq!(lca.query(c, b).unwrap(), Some(c));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn long_chain_and_power_boundaries() {
    for n in [1usize, 2, 3, 7, 8, 9, 1023, 1024, 1025, 100_000] {
        let parents: Vec<_> = (0..n).map(|v| v.checked_sub(1)).collect();
        let lca = Lca::new(&parents).unwrap();
        assert_eq!(lca.query(n - 1, n / 2).unwrap(), Some(n / 2));
        assert_eq!(lca.query(0, n - 1).unwrap(), Some(0));
        assert_eq!(lca.query(n - 1, n - 1).unwrap(), Some(n - 1));
    }
}

#[test]
fn validation_and_forest_boundaries() {
    assert_eq!(
        Lca::new(&[Some(3), Some(1)]),
        Err(Error::InvalidParent {
            vertex: 0,
            parent: 3
        })
    );
    assert_eq!(
        Lca::new(&[Some(1), Some(0)]),
        Err(Error::Cycle { vertex: 0 })
    );
    assert_eq!(Lca::new(&[None, Some(1)]), Err(Error::Cycle { vertex: 1 }));
    assert_eq!(
        Lca::new(&[]).unwrap().query(0, 0),
        Err(Error::InvalidVertex {
            vertex: 0,
            vertices: 0
        })
    );
    let lca = Lca::new(&[Some(2), None, None, Some(1)]).unwrap();
    assert_eq!(lca.query(0, 3).unwrap(), None);
    assert_eq!(lca.query(0, 2).unwrap(), Some(2));
    assert_eq!(
        lca.query(1, 4),
        Err(Error::InvalidVertex {
            vertex: 4,
            vertices: 4
        })
    );
}

#[test]
fn relabeling_extension_and_ambiguous_preimage() {
    let parents = [None, Some(0), Some(0), Some(1), None];
    let permutation = [4, 2, 0, 3, 1];
    let mut relabeled = vec![None; parents.len()];
    for (v, p) in parents.iter().enumerate() {
        relabeled[permutation[v]] = p.map(|p| permutation[p]);
    }
    let original = Lca::new(&parents).unwrap();
    let transformed = Lca::new(&relabeled).unwrap();
    let extended = Lca::new(&[None, Some(0), Some(0), Some(1), None, Some(3)]).unwrap();
    for a in 0..parents.len() {
        for b in 0..parents.len() {
            assert_eq!(
                transformed.query(permutation[a], permutation[b]).unwrap(),
                original.query(a, b).unwrap().map(|v| permutation[v])
            );
            assert_eq!(extended.query(a, b), original.query(a, b));
        }
    }
    // Same observed LCA has multiple compatible parent forests.
    assert_eq!(
        Lca::new(&[None, Some(0), Some(0)]).unwrap().query(1, 2),
        Ok(Some(0))
    );
    assert_eq!(
        Lca::new(&[None, Some(0), Some(0), Some(2)])
            .unwrap()
            .query(1, 2),
        Ok(Some(0))
    );
    assert_ne!(
        Lca::new(&[None, Some(0), Some(1)]).unwrap().query(1, 2),
        Ok(Some(0))
    );
}
