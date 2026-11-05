use super::*;
fn oracle(values: &[i32]) -> CartesianTree {
    fn visit(
        v: &[i32],
        lo: usize,
        hi: usize,
        parent: Option<usize>,
        t: &mut CartesianTree,
    ) -> Option<usize> {
        if lo == hi {
            return None;
        }
        let root = (lo..hi).min_by_key(|&i| (v[i], i)).unwrap();
        t.parent[root] = parent;
        t.left[root] = visit(v, lo, root, Some(root), t);
        t.right[root] = visit(v, root + 1, hi, Some(root), t);
        Some(root)
    }
    let mut t = CartesianTree {
        root: None,
        parent: vec![None; values.len()],
        left: vec![None; values.len()],
        right: vec![None; values.len()],
    };
    t.root = visit(values, 0, values.len(), None, &mut t);
    t
}
#[test]
fn exhaustive_recursive_minima() {
    for n in 0..=8 {
        for mut code in 0..3usize.pow(n) {
            let v: Vec<i32> = (0..n)
                .map(|_| {
                    let x = (code % 3) as i32 - 1;
                    code /= 3;
                    x
                })
                .collect();
            assert_eq!(min_cartesian_tree(&v).unwrap(), oracle(&v), "{v:?}");
        }
    }
}
#[test]
fn literal_ties_and_non_clone_order() {
    assert_eq!(
        min_cartesian_tree(&[2, 1, 1, 0]).unwrap(),
        CartesianTree {
            root: Some(3),
            parent: vec![Some(1), Some(3), Some(1), None],
            left: vec![None, Some(0), None, Some(1)],
            right: vec![None, Some(2), None, None]
        }
    );
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Key(i32);
    assert_eq!(min_cartesian_tree(&[Key(2), Key(1)]).unwrap().root, Some(1));
}
#[test]
fn cap_and_linear_chain() {
    let v = vec![7; MAX_LEN];
    let tree = min_cartesian_tree(&v).unwrap();
    assert_eq!(tree.root, Some(0));
    assert_eq!(tree.parent[MAX_LEN - 1], Some(MAX_LEN - 2));
    assert_eq!(tree.right[MAX_LEN - 1], None);
    assert_eq!(
        min_cartesian_tree(&vec![0; MAX_LEN + 1]),
        Err(Error::TooLong)
    );
}
