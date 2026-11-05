use super::*;
use std::collections::BTreeSet;
fn invariant(t: &Treap, set: &BTreeSet<i64>) {
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
                assert!(t.rank(i) < t.rank(c));
                pending.push((
                    c,
                    if s == 1 { Some(n.key) } else { lo },
                    if s == 0 { Some(n.key) } else { hi },
                ));
            }
        }
    }
    assert_eq!(seen.len(), set.len());
    assert_eq!(t.nodes.len() - t.free.len(), set.len());
    assert!(t.free.iter().all(|i| !seen.contains(i)));
}
#[test]
fn exhaustive_operation_oracle() {
    for code in 0..6usize.pow(6) {
        let mut code = code;
        let mut t = Treap::default();
        let mut set = BTreeSet::new();
        for step in 0..6 {
            let op = code % 6;
            code /= 6;
            let key = (op % 3) as i64 - 1;
            if op < 3 {
                assert_eq!(t.insert(key, (step % 2) as u64).unwrap(), set.insert(key));
            } else {
                assert_eq!(t.remove(key), set.remove(&key));
            }
            invariant(&t, &set);
            for q in -2..=2 {
                assert_eq!(t.contains(q), set.contains(&q));
            }
        }
    }
}
#[test]
fn priorities_capacity_and_reuse() {
    let mut t = Treap::default();
    let mut set = BTreeSet::new();
    for k in 0..4096 {
        assert!(t.insert(k, k as u64).unwrap());
        set.insert(k);
    }
    invariant(&t, &set);
    assert_eq!(t.insert(4096, 0), Err("too many keys"));
    assert!(!t.insert(0, u64::MAX).unwrap());
    assert_eq!(t.nodes[t.root.unwrap()].priority, 0);
    for k in (0..4096).step_by(2) {
        assert!(t.remove(k));
        set.remove(&k);
    }
    for k in 4096..6144 {
        assert!(t.insert(k, u64::MAX).unwrap());
        set.insert(k);
    }
    invariant(&t, &set);
    assert_eq!(t.nodes.len(), 4096);
    for k in set.clone() {
        assert!(t.remove(k));
    }
    assert!(t.keys().is_empty());
    assert!(!t.remove(0));
    for k in [i64::MIN, i64::MAX, 0] {
        t.insert(k, 7).unwrap();
    }
    invariant(&t, &BTreeSet::from([i64::MIN, 0, i64::MAX]));
}
