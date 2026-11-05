use super::*;
use std::collections::BTreeSet;
fn invariant(t: &SplayTree, set: &BTreeSet<i64>) {
    assert_eq!(t.keys(), set.iter().copied().collect::<Vec<_>>());
    let mut pending = Vec::new();
    if let Some(r) = t.root {
        assert_eq!(t.nodes[r].parent, None);
        pending.push((r, None, None));
    }
    let mut seen = BTreeSet::new();
    while let Some((i, lo, hi)) = pending.pop() {
        assert!(seen.insert(i));
        let n = &t.nodes[i];
        assert!(lo.is_none_or(|v| v < n.key) && hi.is_none_or(|v| n.key < v));
        for (s, &child) in n.child.iter().enumerate() {
            if let Some(c) = child {
                assert_eq!(t.nodes[c].parent, Some(i));
                pending.push((
                    c,
                    if s == 1 { Some(n.key) } else { lo },
                    if s == 0 { Some(n.key) } else { hi },
                ));
            }
        }
    }
    assert_eq!(seen.len(), set.len());
}
fn last_key(t: &SplayTree, q: i64) -> Option<i64> {
    let mut at = t.root;
    let mut last = None;
    while let Some(i) = at {
        let n = &t.nodes[i];
        last = Some(n.key);
        if n.key == q {
            break;
        }
        at = if q < n.key { n.child[0] } else { n.child[1] };
    }
    last
}
#[test]
fn exhaustive_stateful_oracle() {
    for code in 0..6usize.pow(6) {
        let mut code = code;
        let mut t = SplayTree::default();
        let mut set = BTreeSet::new();
        for _ in 0..6 {
            let op = code % 6;
            code /= 6;
            let key = (op % 3) as i64 - 1;
            if op < 3 {
                assert_eq!(t.insert(key).unwrap(), set.insert(key));
                assert_eq!(t.root(), Some(key));
            } else {
                let last = last_key(&t, key);
                assert_eq!(t.access(key), set.contains(&key));
                assert_eq!(t.root(), last);
            }
            invariant(&t, &set);
        }
    }
}
#[test]
fn zig_zig_zag_and_missing_access() {
    let mut t = SplayTree::default();
    assert!(!t.access(9));
    assert_eq!(t.root(), None);
    for k in [5, 2, 8, 1, 3, 7, 9, 6, 4] {
        t.insert(k).unwrap();
    }
    let set = (1..=9).collect();
    for q in [1, 9, 2, 8, 3, 7, 4, 6, 5, 0, 10] {
        let last = last_key(&t, q);
        assert_eq!(t.access(q), (1..=9).contains(&q));
        assert_eq!(t.root(), last);
        invariant(&t, &set);
    }
}
#[test]
fn capacity_and_full_width() {
    let mut t = SplayTree::default();
    for k in 0..4096 {
        t.insert(k).unwrap();
    }
    let before = t.root();
    assert_eq!(t.insert(i64::MAX), Err("too many keys"));
    assert_eq!(t.root(), before);
    assert!(!t.insert(0).unwrap());
    assert_eq!(t.root(), Some(0));
    invariant(&t, &(0..4096).collect());
    let mut t = SplayTree::default();
    for k in [i64::MIN, i64::MAX, 0] {
        t.insert(k).unwrap();
    }
    for k in [i64::MIN, i64::MAX, 0] {
        assert!(t.access(k));
        assert_eq!(t.root(), Some(k));
    }
    invariant(&t, &BTreeSet::from([i64::MIN, 0, i64::MAX]));
}
