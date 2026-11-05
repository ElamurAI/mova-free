use super::*;
use std::collections::{BTreeSet, VecDeque};
fn oracle(n: usize, ops: &[Operation]) -> Vec<Answer> {
    let mut edges = BTreeSet::new();
    let mut out = Vec::new();
    for &op in ops {
        match op {
            Operation::Add(a, b) => {
                edges.insert((a.min(b), a.max(b)));
            }
            Operation::Remove(a, b) => {
                edges.remove(&(a.min(b), a.max(b)));
            }
            _ => {
                let mut labels = vec![usize::MAX; n];
                let mut count = 0;
                for start in 0..n {
                    if labels[start] != usize::MAX {
                        continue;
                    }
                    labels[start] = count;
                    let mut queue = VecDeque::from([start]);
                    while let Some(v) = queue.pop_front() {
                        for &(a, b) in &edges {
                            let next = if a == v {
                                b
                            } else if b == v {
                                a
                            } else {
                                continue;
                            };
                            if labels[next] == usize::MAX {
                                labels[next] = count;
                                queue.push_back(next);
                            }
                        }
                    }
                    count += 1;
                }
                out.push(match op {
                    Operation::Connected(a, b) => Answer::Connected(labels[a] == labels[b]),
                    _ => Answer::Components(count),
                });
            }
        }
    }
    out
}
#[test]
fn enumerated_timelines_match_bfs() {
    let edges = [(0, 0), (0, 1), (1, 2), (0, 2)];
    for code in 0..4096usize {
        let mut state = 0usize;
        let mut digits = code;
        let mut ops = vec![Operation::Components];
        for _ in 0..6 {
            let index = digits % 4;
            digits /= 4;
            let (a, b) = edges[index];
            ops.push(if state & (1 << index) == 0 {
                Operation::Add(b, a)
            } else {
                Operation::Remove(a, b)
            });
            state ^= 1 << index;
            for a in 0..3 {
                for b in 0..3 {
                    ops.push(Operation::Connected(a, b));
                }
            }
            ops.push(Operation::Components);
        }
        assert_eq!(solve(3, &ops).unwrap(), oracle(3, &ops), "code={code}");
    }
}
#[test]
fn rollback_and_lifecycle() {
    use Operation::*;
    let ops = [
        Add(0, 1),
        Add(1, 2),
        Connected(0, 2),
        Remove(1, 0),
        Connected(0, 2),
        Components,
        Add(0, 1),
        Connected(0, 2),
        Remove(2, 1),
        Connected(0, 2),
    ];
    assert_eq!(
        solve(3, &ops),
        Ok(vec![
            Answer::Connected(true),
            Answer::Connected(false),
            Answer::Components(2),
            Answer::Connected(true),
            Answer::Connected(false)
        ])
    );
    assert_eq!(
        solve(2, &[Add(0, 1), Add(1, 0)]),
        Err(Error::DuplicateEdge { time: 1 })
    );
    assert_eq!(
        solve(2, &[Remove(1, 0)]),
        Err(Error::AbsentEdge { time: 0 })
    );
    assert_eq!(
        solve(0, &[Connected(0, 0)]),
        Err(Error::InvalidVertex { time: 0, vertex: 0 })
    );
    assert_eq!(solve(0, &[Components]), Ok(vec![Answer::Components(0)]));
    assert_eq!(solve(0, &[]), Ok(vec![]));
    assert_eq!(
        solve(1, &[Add(0, 0), Components, Remove(0, 0), Connected(0, 0)]),
        Ok(vec![Answer::Components(1), Answer::Connected(true)])
    );
}
#[test]
fn resource_caps() {
    assert_eq!(solve(MAX_VERTICES + 1, &[]), Err(Error::TooManyVertices));
    assert_eq!(
        solve(0, &vec![Operation::Components; MAX_OPERATIONS + 1]),
        Err(Error::TooManyOperations)
    );
    assert_eq!(
        solve(MAX_VERTICES, &[Operation::Components]),
        Ok(vec![Answer::Components(MAX_VERTICES)])
    );
    assert_eq!(
        solve(0, &vec![Operation::Components; MAX_OPERATIONS]).unwrap(),
        vec![Answer::Components(0); MAX_OPERATIONS]
    );
}
