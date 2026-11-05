use super::*;
fn baseline(s: &FullSentence) -> Result<(), String> {
    let n = s.words.len();
    let mut heads = Vec::with_capacity(n);
    for k in 0..n {
        let r = s.word(k);
        let h: usize = r.get(Col::Head).parse().map_err(|_| {
            format!(
                "tree gate: word {}: head '{}' is not a number",
                k + 1,
                r.get(Col::Head)
            )
        })?;
        if h > n {
            return Err(format!(
                "tree gate: word {}: head {h} outside the sentence ({n} words)",
                k + 1
            ));
        }
        if (h == 0) != (r.get(Col::Deprel) == "root") {
            return Err(format!(
                "tree gate: word {}: head {h}, relation '{}' — root only on the root",
                k + 1,
                r.get(Col::Deprel)
            ));
        }
        heads.push(h);
    }
    let roots = heads.iter().filter(|&&h| h == 0).count();
    if n > 0 && roots != 1 {
        return Err(format!("tree gate: roots {roots}, expected one"));
    }
    for k in 0..n {
        let (mut x, mut steps) = (k + 1, 0);
        while x != 0 {
            x = heads[x - 1];
            steps += 1;
            if steps > n {
                return Err(format!("tree gate: cycle through word {}", k + 1));
            }
        }
    }
    Ok(())
}

fn fixture(heads: &[usize]) -> FullSentence {
    let text = if heads.is_empty() {
        "1\tword\tword\tNOUN\tNN\t_\t0\troot\t_\t_\n\n".to_string()
    } else {
        heads
            .iter()
            .enumerate()
            .map(|(k, h)| {
                format!(
                    "{}\tword\tword\tNOUN\tNN\t_\t{}\t{}\t_\t_\n",
                    k + 1,
                    h,
                    if *h == 0 { "root" } else { "dep" }
                )
            })
            .collect::<String>()
            + "\n"
    };
    let mut s = crate::conllu::Doc::parse(&text).unwrap().sents.remove(0);
    if heads.is_empty() {
        s.words.clear();
        s.rows.clear();
    }
    s
}
fn compare(heads: &[usize]) {
    let s = fixture(heads);
    assert_eq!(tree_gate(&s), baseline(&s), "heads={heads:?}");
    assert_eq!(tree_gate(&s), parent_gate(&s), "parent heads={heads:?}");
}

#[test]
fn bfs_transfer_precedence() {
    let mut s = fixture(&[3, 0, 4, 3]);
    assert_eq!(tree_gate(&s), Err("tree gate: cycle through word 1".into()));
    s.word_mut(0).set(Col::Head, "1");
    assert_eq!(tree_gate(&s), Err("tree gate: cycle through word 1".into()));
    s.word_mut(0).set(Col::Head, "9");
    assert_eq!(tree_gate(&s), baseline(&s));
    s.word_mut(0).set(Col::Head, "x");
    assert_eq!(tree_gate(&s), baseline(&s));
    s.word_mut(0).set(Col::Head, "0");
    assert_eq!(tree_gate(&s), baseline(&s));
    assert_eq!(tree_gate(&fixture(&[])), Ok(()));
}

#[test]
fn bfs_transfer_exhaustive_heads() {
    let mut cases = 0usize;
    for n in 0..=6u32 {
        let radix = n as usize + 1;
        for mut code in 0..radix.pow(n) {
            let heads = (0..n)
                .map(|_| {
                    let h = code % radix;
                    code /= radix;
                    h
                })
                .collect::<Vec<_>>();
            compare(&heads);
            cases += 1;
        }
    }
    assert_eq!(cases, 126126);
}
#[test]
fn bfs_transfer_long_shapes() {
    for n in [8, 63, 64, 65, 66, 128, 1024] {
        let mut h = (0..n).collect::<Vec<_>>();
        compare(&h);
        h.fill(1);
        h[0] = 0;
        compare(&h);
        h[0] = 2;
        h[1] = 3;
        h[2] = 2;
        h[n - 1] = 0;
        compare(&h);
    }
}

#[test]
fn bfs_transfer_active_fallback() {
    for n in [64, 65, 129] {
        let mut heads = (0..n).collect::<Vec<_>>();
        compare(&heads);
        heads.fill(1);
        heads[0] = 0;
        compare(&heads);
    }
    // The first 99 rooted words exhaust 4*n before the cycle's first tail.
    // Thus a reverse projection mutant must be caught on the active BFS path.
    let mut heads = (0..129).collect::<Vec<_>>();
    heads[99] = 101;
    heads[100] = 102;
    heads[101] = 101;
    compare(&heads);
    assert_eq!(
        tree_gate(&fixture(&heads)),
        Err("tree gate: cycle through word 100".into())
    );
}

#[test]
#[ignore = "manual serial timing only; synthetic inputs, no external state"]
fn bfs_transfer_benchmark() {
    benchmark_gate(baseline);
}

#[test]
#[ignore = "manual serial parent timing"]
fn wave25_parent_benchmark() {
    benchmark_gate(parent_gate);
}

fn benchmark_gate(baseline: impl Fn(&FullSentence) -> Result<(), String>) {
    use std::{hint::black_box, time::Instant};
    for n in [8, 32, 64, 65, 128, 512, 2048] {
        for shape in ["chain", "star"] {
            let mut heads = (0..n).collect::<Vec<_>>();
            if shape == "star" {
                heads.fill(1);
                heads[0] = 0;
            }
            let iterations = if n < 128 { 1000 } else { 32 };
            let input = fixture(&heads);
            assert_eq!(tree_gate(&input), baseline(&input));
            let old_run = || {
                let start = Instant::now();
                for _ in 0..iterations {
                    black_box(baseline(black_box(&input))).unwrap();
                }
                start.elapsed()
            };
            let new_run = || {
                let start = Instant::now();
                for _ in 0..iterations {
                    black_box(tree_gate(black_box(&input))).unwrap();
                }
                start.elapsed()
            };
            for _ in 0..3 {
                black_box(old_run());
                black_box(new_run());
            }
            for sample in 0..7 {
                let (old, new) = if sample % 2 == 0 {
                    let old = old_run();
                    (old, new_run())
                } else {
                    let new = new_run();
                    (old_run(), new)
                };
                println!(
                    "n={n} shape={shape} iterations={iterations} sample={sample} baseline_first={} baseline_ns={} hybrid_ns={}",
                    sample % 2 == 0,
                    old.as_nanos(),
                    new.as_nanos()
                );
            }
        }
    }
}
fn parent_gate(s: &FullSentence) -> Result<(), String> {
    let n = s.words.len();
    let mut heads = Vec::with_capacity(n);
    for k in 0..n {
        let r = s.word(k);
        let h: usize = r.get(Col::Head).parse().map_err(|_| {
            format!(
                "tree gate: word {}: head '{}' is not a number",
                k + 1,
                r.get(Col::Head)
            )
        })?;
        if h > n {
            return Err(format!(
                "tree gate: word {}: head {h} outside the sentence ({n} words)",
                k + 1
            ));
        }
        if (h == 0) != (r.get(Col::Deprel) == "root") {
            return Err(format!(
                "tree gate: word {}: head {h}, relation '{}' — root only on the root",
                k + 1,
                r.get(Col::Deprel)
            ));
        }
        heads.push(h);
    }
    let roots = heads.iter().filter(|&&h| h == 0).count();
    if n > 0 && roots != 1 {
        return Err(format!("tree gate: roots {roots}, expected one"));
    }
    if n <= 64 {
        for k in 0..n {
            let (mut x, mut steps) = (k + 1, 0);
            while x != 0 {
                x = heads[x - 1];
                steps += 1;
                if steps > n {
                    return Err(format!("tree gate: cycle through word {}", k + 1));
                }
            }
        }
        return Ok(());
    }
    // Preserve cheap ordered walks, but cap their combined work before BFS.
    let mut budget = n.saturating_mul(4);
    'walks: {
        for k in 0..n {
            let (mut x, mut steps) = (k + 1, 0);
            while x != 0 {
                if budget == 0 {
                    break 'walks;
                }
                budget -= 1;
                x = heads[x - 1];
                steps += 1;
                if steps > n {
                    return Err(format!("tree gate: cycle through word {}", k + 1));
                }
            }
        }
        return Ok(());
    }
    // Include artificial root 0: its reverse-edge reachable set is exactly
    // the words whose original head chain terminates, rather than cycling.
    let mut children = vec![Vec::new(); n + 1];
    for (k, &head) in heads.iter().enumerate() {
        children[head].push(k + 1);
    }
    let reachable = mova_algorithms::bfs::bfs(&children, 0)
        .expect("validated heads produce valid reverse edges");
    if let Some(k) = (1..=n).find(|&k| reachable[k].is_none()) {
        return Err(format!("tree gate: cycle through word {k}"));
    }
    Ok(())
}

#[test]
fn wave25_color_paths_and_error_precedence() {
    for n in [65, 66, 129] {
        let mut heads = vec![n; n];
        heads[n - 1] = 0;
        compare(&heads); // Shared completed suffix is not a current-path cycle.
        heads[0] = 2;
        heads[1] = 3;
        heads[2] = 2;
        compare(&heads); // Report tail 1, not the first cycle member 2.
        assert_eq!(
            tree_gate(&fixture(&heads)),
            Err("tree gate: cycle through word 1".into())
        );
        let mut s = fixture(&heads);
        s.word_mut(n - 1).set(Col::Head, "bad");
        assert_eq!(tree_gate(&s), baseline(&s)); // All row checks precede cycles.
    }
}
