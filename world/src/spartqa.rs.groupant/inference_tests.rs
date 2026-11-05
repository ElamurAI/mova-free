use super::*;
impl Scene {
    fn reference_breach(&self, a: char, r: R, b: char) -> bool {
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([a]);
        while let Some(x) = q.pop_front() {
            for &(u, rr, v) in &self.bfacts {
                let step = if u == x && rr == r {
                    Some(v)
                } else if v == x && rr.inv() == r && rr.directional() {
                    Some(u)
                } else {
                    None
                };
                if let Some(y) = step {
                    if y == b {
                        return true;
                    }
                    if seen.insert(y) {
                        q.push_back(y);
                    }
                }
            }
        }
        false
    }

    /// Whether «x r y» is inferred.
    pub fn reference_holds(&self, x: usize, r: R, y: usize) -> bool {
        if x == y {
            return false;
        }
        let (bx, by) = (self.objs[x].block, self.objs[y].block);
        if !r.directional() {
            if self.facts.contains(&(x, r, y)) || self.facts.contains(&(y, r, x)) {
                return true;
            }
            // in different blocks — far
            return r == R::Far && bx.is_some() && by.is_some() && bx != by;
        }
        if let (Some(a), Some(b)) = (bx, by) {
            if a != b {
                return self.reference_breach(a, r, b);
            }
        }
        // transitively within the facts
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([x]);
        while let Some(u) = q.pop_front() {
            for &(a, rr, b) in &self.facts {
                let step = if a == u && rr == r {
                    Some(b)
                } else if b == u && rr.directional() && rr.inv() == r {
                    Some(a)
                } else {
                    None
                };
                if let Some(v) = step {
                    if v == y {
                        return true;
                    }
                    if seen.insert(v) {
                        q.push_back(v);
                    }
                }
            }
        }
        false
    }

    /// Path of direction r from x to y; Some(whether the path implies «far»), None — no path. Far: a far step, or two
    /// steps or more whose distances were all qualified (near / far) — a thing in between at the narrator's scale.
    fn reference_dir_path(&self, x: usize, r: R, y: usize) -> Option<bool> {
        let dist = |u: usize, v: usize| {
            [R::Near, R::Far, R::Touch]
                .iter()
                .any(|&d| self.facts.contains(&(u, d, v)) || self.facts.contains(&(v, d, u)))
        };
        // state: (thing, far so far, steps (capped at 2), all steps qualified)
        let mut seen: BTreeSet<(usize, bool, u8, bool)> = BTreeSet::new();
        let mut q = VecDeque::from([(x, false, 0u8, true)]);
        let mut best: Option<bool> = None;
        while let Some((u, far, steps, qual)) = q.pop_front() {
            for &(a, rr, b) in &self.facts {
                let v = if a == u && rr == r {
                    b
                } else if b == u && rr.directional() && rr.inv() == r {
                    a
                } else {
                    continue;
                };
                let (steps, qual) = ((steps + 1).min(2), qual && dist(u, v));
                let f = far
                    || self.facts.contains(&(u, R::Far, v))
                    || self.facts.contains(&(v, R::Far, u));
                if v == y {
                    best = Some(best.unwrap_or(false) || f || steps >= 2 && qual);
                }
                if seen.insert((v, f, steps, qual)) {
                    q.push_back((v, f, steps, qual));
                }
            }
        }
        best
    }
}

fn scene(n: usize) -> Scene {
    Scene {
        objs: (0..n)
            .map(|_| Obj {
                size: None,
                color: None,
                shape: None,
                block: None,
            })
            .collect(),
        ..Scene::default()
    }
}

// Independent fixed-point semantics: no production rows or FIFO traversal.
fn closure(sc: &Scene, r: R, source: usize, target: usize) -> Option<bool> {
    let mut reached = BTreeSet::from([(source, false, 0u8, true)]);
    let mut best = None;
    loop {
        let before = reached.len();
        for (u, far, steps, qualified) in reached.clone() {
            for v in 0..sc.objs.len() {
                let edge = sc.facts.contains(&(u, r, v))
                    || r.directional() && sc.facts.contains(&(v, r.inv(), u));
                if !edge {
                    continue;
                }
                let marked =
                    |kind| sc.facts.contains(&(u, kind, v)) || sc.facts.contains(&(v, kind, u));
                let f = far || marked(R::Far);
                let q = qualified && [R::Near, R::Far, R::Touch].into_iter().any(marked);
                let k = (steps + 1).min(2);
                if v == target {
                    best = Some(best.unwrap_or(false) || f || k >= 2 && q);
                }
                reached.insert((v, f, k, q));
            }
        }
        if reached.len() == before {
            return best;
        }
    }
}

#[test]
fn exhaustive_small_labeled_graphs() {
    let edges: Vec<_> = (0..3)
        .flat_map(|a| (0..3).filter(move |&b| b != a).map(move |b| (a, b)))
        .collect();
    for mask in 0..64usize {
        for quals in 0..16usize {
            let mut sc = scene(3);
            for (i, &(a, b)) in edges.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    sc.facts.insert(if i % 2 == 0 {
                        (a, R::Left, b)
                    } else {
                        (b, R::Right, a)
                    });
                    match (quals >> (i % 4)) & 3 {
                        1 => {
                            sc.facts.insert((a, R::Near, b));
                        }
                        2 => {
                            sc.facts.insert((b, R::Far, a));
                        }
                        3 => {
                            sc.facts.insert((a, R::Touch, b));
                        }
                        _ => {}
                    }
                }
            }
            sc.facts.insert((0, R::Above, 2));
            for r in [
                R::Left,
                R::Right,
                R::Above,
                R::Below,
                R::Near,
                R::Far,
                R::Touch,
            ] {
                for x in 0..3 {
                    for y in 0..3 {
                        assert_eq!(
                            sc.holds(x, r, y),
                            sc.reference_holds(x, r, y),
                            "mask={mask},quals={quals},x={x},r={r:?},y={y}"
                        );
                        if r.directional() {
                            let expected = closure(&sc, r, x, y);
                            assert_eq!(
                                sc.dir_path(x, r, y),
                                expected,
                                "mask={mask},quals={quals},x={x},r={r:?},y={y}"
                            );
                            assert_eq!(sc.dir_path(x, r, y), sc.reference_dir_path(x, r, y));
                            if x != y {
                                assert_eq!(sc.holds(x, r, y), expected.is_some());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn blocks_cycles_and_nondirectional_semantics() {
    for mask in 0..64usize {
        let mut sc = scene(3);
        for i in 0..3 {
            sc.objs[i].block = Some((b'A' + i as u8) as char);
        }
        let mut bit = 0;
        for a in ['A', 'B', 'C'] {
            for b in ['A', 'B', 'C'] {
                if a != b {
                    if mask & (1 << bit) != 0 {
                        sc.bfacts.insert((a, R::Left, b));
                    }
                    bit += 1;
                }
            }
        }
        for r in [R::Left, R::Right, R::Above, R::Near] {
            for x in 0..3 {
                for y in 0..3 {
                    assert_eq!(
                        sc.breach((b'A' + x as u8) as char, r, (b'A' + y as u8) as char),
                        sc.reference_breach((b'A' + x as u8) as char, r, (b'A' + y as u8) as char)
                    );
                    assert_eq!(sc.holds(x, r, y), sc.reference_holds(x, r, y));
                }
            }
        }
    }
    let mut sc = scene(2);
    sc.bfacts.insert(('A', R::Near, 'B'));
    assert!(sc.breach('A', R::Near, 'B'));
    assert!(!sc.breach('B', R::Near, 'A'));
    sc.facts.insert((0, R::Near, 1));
    assert!(sc.holds(1, R::Near, 0));
    assert!(!sc.holds(0, R::Near, 0));
}

#[test]
fn inverse_and_qualifier_controls() {
    let mut sc = scene(4);
    sc.facts.extend([
        (1, R::Right, 0),
        (2, R::Right, 1),
        (0, R::Near, 1),
        (1, R::Touch, 2),
        (0, R::Left, 2),
    ]);
    assert!(sc.holds(0, R::Left, 2));
    assert_eq!(sc.dir_path(0, R::Left, 2), Some(true));
    sc.facts.remove(&(1, R::Touch, 2));
    assert_eq!(sc.dir_path(0, R::Left, 2), Some(false));
    sc.facts.insert((1, R::Far, 2));
    assert_eq!(sc.dir_path(0, R::Left, 2), Some(true));
    assert_eq!(sc.dir_path(3, R::Left, 0), None);
    sc.bfacts
        .extend([('B', R::Right, 'A'), ('C', R::Right, 'B')]);
    assert!(sc.breach('A', R::Left, 'C'));
}

#[test]
fn adjacency_preserves_original_row_order() {
    let facts = BTreeSet::from([
        (0, R::Left, 2),
        (0, R::Left, 1),
        (1, R::Right, 0),
        (2, R::Above, 0),
        (2, R::Right, 0),
    ]);
    assert_eq!(
        directional_rows(&facts, R::Left).get(&0),
        Some(&vec![1, 2, 1, 2])
    );
}

#[test]
#[ignore = "Run only in an isolated release timing window"]
fn benchmark_transfer() {
    use std::{hint::black_box, time::Instant};
    let mut workloads = Vec::new();
    for n in [3usize, 32, 256] {
        let mut sc = scene(n);
        for i in 0..n - 1 {
            sc.facts.insert((i, R::Left, i + 1));
            sc.facts.insert((i, R::Near, i + 1));
        }
        workloads.push((
            format!("chain-{n}"),
            sc,
            0,
            n - 1,
            if n == 256 { 50 } else { 500 },
        ));
    }
    for count in [64usize, 65] {
        let mut sc = scene(4);
        sc.facts.extend([
            (0, R::Left, 1),
            (1, R::Left, 2),
            (0, R::Near, 1),
            (1, R::Near, 2),
        ]);
        pad_facts(&mut sc, count);
        workloads.push((format!("boundary-{count}"), sc, 0, 2, 500));
    }
    let mut dense = scene(48);
    for a in 0..48 {
        for b in a + 1..48 {
            dense.facts.insert((a, R::Left, b));
        }
    }
    workloads.push(("dense-direct".into(), dense, 0, 47, 30));
    let mut direct = scene(256);
    for a in 0..255 {
        direct.facts.insert((a, R::Left, a + 1));
    }
    workloads.push(("large-direct".into(), direct, 0, 1, 300));
    let mut missing = scene(256);
    for a in 1..255 {
        missing.facts.insert((a, R::Left, a + 1));
    }
    workloads.push(("missing-source".into(), missing, 0, 255, 300));
    for (name, sc, x, y, reps) in workloads {
        assert_eq!(sc.holds(x, R::Left, y), sc.reference_holds(x, R::Left, y));
        assert_eq!(
            sc.dir_path(x, R::Left, y),
            sc.reference_dir_path(x, R::Left, y)
        );
        for product in [false, true] {
            let run = |old: bool| {
                let mut checksum = 0u64;
                for _ in 0..reps {
                    let sc = black_box(&sc);
                    let x = black_box(x);
                    let y = black_box(y);
                    let result = if product {
                        if old {
                            sc.reference_dir_path(x, R::Left, y)
                        } else {
                            sc.dir_path(x, R::Left, y)
                        }
                    } else {
                        Some(if old {
                            sc.reference_holds(x, R::Left, y)
                        } else {
                            sc.holds(x, R::Left, y)
                        })
                    };
                    checksum += black_box(result.map_or(0, |b| 1 + u64::from(b)));
                }
                checksum
            };
            for _ in 0..3 {
                assert_eq!(run(true), run(false));
            }
            for sample in 0..7 {
                let mut sums = [0u64; 2];
                let mut times = [0u128; 2];
                let order = if sample % 2 == 0 {
                    [true, false]
                } else {
                    [false, true]
                };
                for old in order {
                    let start = Instant::now();
                    let slot = usize::from(old);
                    sums[slot] = run(old);
                    times[slot] = start.elapsed().as_nanos();
                }
                assert_eq!(sums[0], sums[1]);
                println!(
                    "{name} product={product} sample={sample} old_first={} reps={reps} baseline_ns={} production_ns={} checksum={}",
                    sample % 2 == 0,
                    times[1],
                    times[0],
                    sums[0]
                );
            }
        }
    }
}

#[test]
fn intermediate_ids_need_not_have_objects() {
    let mut sc = scene(2);
    sc.facts.extend([
        (0, R::Left, usize::MAX),
        (usize::MAX, R::Left, 1),
        (0, R::Near, usize::MAX),
        (usize::MAX, R::Touch, 1),
    ]);
    assert!(sc.holds(0, R::Left, 1));
    assert_eq!(sc.holds(0, R::Left, 1), sc.reference_holds(0, R::Left, 1));
    assert_eq!(sc.dir_path(0, R::Left, 1), Some(true));
    assert_eq!(
        sc.dir_path(0, R::Left, 1),
        sc.reference_dir_path(0, R::Left, 1)
    );
}

fn pad_facts(sc: &mut Scene, count: usize) {
    let mut i = 100usize;
    while sc.facts.len() < count {
        sc.facts.insert((i, R::Above, i + 1));
        i += 1;
    }
}

#[test]
fn hybrid_boundary_and_indexed_controls() {
    for count in [64, 65, 96] {
        let mut sc = scene(4);
        sc.facts.extend([
            (1, R::Right, 0),
            (2, R::Right, 1),
            (0, R::Near, 1),
            (1, R::Touch, 2),
            (0, R::Left, 2),
        ]);
        pad_facts(&mut sc, count);
        assert_eq!(sc.facts.len(), count);
        assert!(sc.holds(0, R::Left, 2));
        assert_eq!(sc.dir_path(0, R::Left, 2), Some(true));
        assert_eq!(
            sc.dir_path(0, R::Left, 2),
            sc.reference_dir_path(0, R::Left, 2)
        );
        sc.facts.remove(&(1, R::Touch, 2));
        pad_facts(&mut sc, count);
        assert_eq!(sc.dir_path(0, R::Left, 2), Some(false));
        sc.facts.insert((1, R::Far, 2));
        assert_eq!(sc.dir_path(0, R::Left, 2), Some(true));
        assert_eq!(sc.dir_path(3, R::Left, 0), None);
        assert!(!sc.holds(3, R::Left, 0));
        sc.bfacts
            .extend([('B', R::Right, 'A'), ('C', R::Right, 'B')]);
        for i in 0..count - 2 {
            sc.bfacts
                .insert((char::from_u32(1000 + i as u32).unwrap(), R::Above, 'Z'));
        }
        assert_eq!(sc.bfacts.len(), count);
        assert!(sc.breach('A', R::Left, 'C'));
        assert_eq!(
            sc.breach('A', R::Left, 'C'),
            sc.reference_breach('A', R::Left, 'C')
        );
        assert!(!sc.breach('Q', R::Left, 'C'));
    }
}
