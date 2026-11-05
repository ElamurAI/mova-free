use super::*;
fn baseline(a: &Ann, forms: &[String], reg: &Registry) -> Result<(), Bad> {
    if a.len() != forms.len() {
        return Err(Bad::Count(a.len(), forms.len()));
    }
    for (i, t) in a.iter().enumerate() {
        if t.form != forms[i] {
            return Err(Bad::Form(i + 1));
        }
        if t.head > a.len() || t.head == i + 1 {
            return Err(Bad::Head(i + 1));
        }
        if let Some(m) = reg
            .check(t.upos, t.feats, t.rel, &t.lemma)
            .into_iter()
            .next()
        {
            return Err(Bad::Registry(i + 1, m));
        }
    }
    let roots = a.iter().filter(|t| t.head == 0).count();
    if roots != 1 {
        return Err(Bad::Roots(roots));
    }
    for i in 1..=a.len() {
        let mut j = i;
        for _ in 0..=a.len() {
            j = a[j - 1].head;
            if j == 0 {
                break;
            }
        }
        if j != 0 {
            return Err(Bad::Cycle(i));
        }
    }
    Ok(())
}

fn fixture(heads: &[usize]) -> Ann {
    heads
        .iter()
        .map(|&head| Tok {
            form: "word".into(),
            lemma: "word".into(),
            upos: UPos::NOUN,
            tag: Tag::NN,
            feats: Feats::default(),
            head,
            rel: if head == 0 { Rel::Root } else { Rel::Dep },
        })
        .collect()
}
fn compare(heads: &[usize]) {
    let a = fixture(heads);
    let forms = a.iter().map(|t| t.form.clone()).collect::<Vec<_>>();
    static REGISTRY: std::sync::OnceLock<Registry> = std::sync::OnceLock::new();
    let reg = REGISTRY.get_or_init(Registry::load);
    assert_eq!(
        check(&a, &forms, &reg),
        baseline(&a, &forms, &reg),
        "heads={heads:?}"
    );
}

#[test]
fn bfs_transfer_precedence() {
    let reg = Registry::load();
    let forms = vec!["word".to_string(); 4];
    let mut a = fixture(&[3, 0, 4, 3]);
    assert_eq!(check(&a, &forms, &reg), Err(Bad::Cycle(1)));
    a[0].head = 1;
    assert_eq!(check(&a, &forms, &reg), Err(Bad::Head(1)));
    a[0].head = 5;
    assert_eq!(check(&a, &forms, &reg), Err(Bad::Head(1)));
    a[0].form = "wrong".into();
    assert_eq!(check(&a, &forms, &reg), Err(Bad::Form(1)));
    assert_eq!(check(&a, &[], &reg), Err(Bad::Count(4, 0)));
    assert_eq!(check(&vec![], &[], &reg), Err(Bad::Roots(0)));
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
    for n in [8, 64, 1024] {
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
    let a = fixture(&heads);
    let forms = a.iter().map(|t| t.form.clone()).collect::<Vec<_>>();
    assert_eq!(check(&a, &forms, &Registry::load()), Err(Bad::Cycle(100)));
}

#[test]
#[ignore = "manual serial timing only; synthetic inputs, no external state"]
fn bfs_transfer_benchmark() {
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
            let forms = input.iter().map(|t| t.form.clone()).collect::<Vec<_>>();
            let reg = Registry::load();
            assert_eq!(check(&input, &forms, &reg), baseline(&input, &forms, &reg));
            let old_run = || {
                let start = Instant::now();
                for _ in 0..iterations {
                    black_box(baseline(black_box(&input), &forms, &reg)).unwrap();
                }
                start.elapsed()
            };
            let new_run = || {
                let start = Instant::now();
                for _ in 0..iterations {
                    black_box(check(black_box(&input), &forms, &reg)).unwrap();
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
