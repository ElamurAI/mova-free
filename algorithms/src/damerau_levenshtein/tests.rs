use super::*;
use std::collections::{BTreeMap, VecDeque};

fn words(max: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n).map(move |mask| (0..n).map(|bit| ((mask >> bit) & 1) as u8).collect())
        })
        .collect()
}
// Every script of cost <=budget stays within start.len()+budget bytes.
// A shortest script needs no fresh alphabet symbols: replace a fresh symbol
// consistently by a member of the input/output alphabet, removing null edits.
// BFS includes every single insertion/deletion/substitution/adjacent swap.
fn graph(start: &[u8], alphabet: &[u8], budget: usize) -> BTreeMap<Vec<u8>, usize> {
    let mut seen = BTreeMap::from([(start.to_vec(), 0)]);
    let mut queue = VecDeque::from([start.to_vec()]);
    while let Some(word) = queue.pop_front() {
        let depth = seen[&word];
        if depth == budget {
            continue;
        }
        let mut next = Vec::new();
        for i in 0..word.len() {
            let mut v = word.clone();
            v.remove(i);
            next.push(v);
            for &c in alphabet {
                let mut v = word.clone();
                v[i] = c;
                next.push(v);
            }
            if i + 1 < word.len() {
                let mut v = word.clone();
                v.swap(i, i + 1);
                next.push(v);
            }
        }
        for i in 0..=word.len() {
            for &c in alphabet {
                let mut v = word.clone();
                v.insert(i, c);
                next.push(v);
            }
        }
        for v in next {
            if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(v.clone()) {
                e.insert(depth + 1);
                queue.push_back(v);
            }
        }
    }
    seen
}
#[test]
fn exhaustive_edit_graph() {
    let all = words(3);
    for a in &all {
        let oracle = graph(a, &[0, 1], 3);
        for b in &all {
            let actual = distance(a, b).unwrap();
            assert_eq!(actual, oracle[b], "{a:?} -> {b:?}");
            assert_eq!(actual, distance(b, a).unwrap());
            assert!(actual >= a.len().abs_diff(b.len()));
        }
    }
}
#[test]
fn unrestricted_witness() {
    let oracle = graph(b"CA", b"ABC", 3);
    assert_eq!(oracle[b"ABC".as_slice()], 2);
    assert_eq!(distance(b"CA", b"ABC"), Ok(2));
    assert_eq!(crate::optimal_string_alignment::distance(b"CA", b"ABC"), 3);
}
#[test]
fn byte_and_resource_boundaries() {
    assert_eq!(distance(&[0, 255], &[255, 0]), Ok(1));
    assert_eq!(distance(&vec![0; 998], &vec![0; 998]), Ok(0));
    assert_eq!(
        distance(&vec![0; 999], &vec![0; 998]),
        Err(Error::CellLimit)
    );
    assert_eq!(distance(&[], &vec![0; MAX_CELLS]), Ok(MAX_CELLS));
}
#[test]
fn triangle_and_zero_fibers() {
    let all = words(2);
    for a in &all {
        for b in &all {
            assert_eq!(distance(a, b).unwrap() == 0, a == b);
            for c in &all {
                assert!(
                    distance(a, c).unwrap() <= distance(a, b).unwrap() + distance(b, c).unwrap()
                );
            }
        }
    }
}
