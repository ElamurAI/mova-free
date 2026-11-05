//! Actual-source finite correspondence probes; no production substitution.
use math::big::{IBig, Q, q, to_f64};
use math::{
    linsys::{LinSol, gauss},
    poly::Poly,
};
use mova_algorithms::{
    functional_graph_decomposition as graph, gaussian_elimination_modular as modular, gcd, horner,
};
use serde_json::Value;

fn pairs(id: &str) -> Vec<[usize; 2]> {
    let c: Value =
        serde_json::from_str(include_str!("../../../models/correspondences.json")).unwrap();
    let f = c["formulas"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap();
    serde_json::from_value(f["expr"]["pairs"].clone()).unwrap()
}

fn observed(id: &str, inputs: &[&str], outputs: &[String]) {
    let c: Value =
        serde_json::from_str(include_str!("../../../models/correspondences.json")).unwrap();
    let f = c["formulas"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap();
    let names: Vec<String> =
        serde_json::from_value(f["contract"]["domain"]["values"].clone()).unwrap();
    assert_eq!(names, inputs);
    let codomain: Vec<String> =
        serde_json::from_value(f["contract"]["codomain"]["values"].clone()).unwrap();
    let actual: Vec<_> = outputs
        .iter()
        .enumerate()
        .map(|(i, o)| [i, codomain.iter().position(|v| v == o).unwrap()])
        .collect();
    assert_eq!(pairs(id), actual);
}

fn gate(heads: &[usize]) -> Result<(), String> {
    let text: String = heads
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let rel = if *h == 0 { "root" } else { "dep" };
            format!("{}\tx\tx\tNOUN\t_\t_\t{}\t{}\t_\t_\n", i + 1, h, rel)
        })
        .collect();
    let doc = en::conllu::Doc::parse(&text).unwrap();
    en::convert::tree_gate(&doc.sents[0])
}
fn basin(heads: &[usize]) -> Option<usize> {
    let s: Vec<_> = std::iter::once(0).chain(heads.iter().copied()).collect();
    let d = graph::decompose(&s).unwrap();
    (1..s.len()).find(|&i| d.cycle_id[i] != d.cycle_id[0])
}
#[test]
fn gcd_actual_implementations_and_divisor_oracle() {
    for a in 0..=64u64 {
        for b in 0..=64u64 {
            let actual = gcd::gcd(a, b);
            let oracle = if a == 0 && b == 0 {
                0
            } else {
                (1..=a.max(b))
                    .filter(|d| a % d == 0 && b % d == 0)
                    .max()
                    .unwrap()
            };
            assert_eq!(actual, oracle);
            assert_eq!(
                math::nt::gcd(&IBig::from(a), &IBig::from(b)),
                IBig::from(actual)
            );
        }
    }
    for (a, b) in [(u64::MAX, 0), (0, u64::MAX), (u64::MAX, u64::MAX)] {
        assert_eq!(
            math::nt::gcd(&IBig::from(a), &IBig::from(b)),
            IBig::from(gcd::gcd(a, b))
        );
    }
    let expected: Vec<_> = (0..9usize)
        .map(|i| [i, gcd::gcd((i / 3) as u64, (i % 3) as u64) as usize])
        .collect();
    assert_eq!(pairs("gcd.algorithm"), expected);
    let names: Vec<_> = (0..9).map(|i| format!("({},{})", i / 3, i % 3)).collect();
    let refs: Vec<_> = names.iter().map(String::as_str).collect();
    let av: Vec<_> = (0..9).map(|i| gcd::gcd(i / 3, i % 3).to_string()).collect();
    let ov: Vec<_> = (0..9u64)
        .map(|i| math::nt::gcd(&IBig::from(i / 3), &IBig::from(i % 3)).to_string())
        .collect();
    observed("gcd.algorithm", &refs, &av);
    observed("gcd.ordinary", &refs, &ov);
    assert_eq!(gcd::gcd(2, 0), gcd::gcd(2, 2)); // distinct compatible antecedents
    assert_eq!(gcd::gcd(0, 0), 0);
    assert_ne!(gcd::gcd(0, 0), 1);
    let av = if gcd::gcd(0, 0) == 0 {
        "returned_zero"
    } else {
        "wrong"
    };
    let ov = if math::nt::gcd(&IBig::ZERO, &IBig::ZERO) == IBig::ZERO {
        "returned_zero"
    } else {
        "wrong"
    };
    observed("gcd_zero.algorithm", &["(0,0)"], &[av.into()]);
    observed("gcd_zero.ordinary", &["(0,0)"], &[ov.into()]);
}
#[test]
fn horner_actual_value_and_derivative_on_small_exact_domain() {
    let mut checked = 0;
    for len in 0..=4u32 {
        for mut code in 0..5usize.pow(len) {
            let cs: Vec<_> = (0..len)
                .map(|_| {
                    let c = code % 5;
                    code /= 5;
                    c as i64 - 2
                })
                .collect();
            let p = Poly(cs.iter().copied().map(q).collect());
            let fs: Vec<_> = cs.iter().map(|&c| c as f64).collect();
            for x in -2..=2 {
                let (v, d) = horner::evaluate(&fs, x as f64).unwrap();
                assert_eq!(v, p.eval_f64(x as f64));
                assert_eq!(v, to_f64(&p.eval(&q(x))));
                assert_eq!(d, to_f64(&p.deriv().eval(&q(x))));
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 3905);
    let fixtures = [(vec![], 2), (vec![1, 2], 3), (vec![1, 0, -1], 1)];
    let av: Vec<_> = fixtures
        .iter()
        .map(|(c, x)| {
            horner::evaluate(&c.iter().map(|&v| v as f64).collect::<Vec<_>>(), *x as f64)
                .unwrap()
                .0
                .to_string()
        })
        .collect();
    let ov: Vec<_> = fixtures
        .iter()
        .map(|(c, x)| {
            Poly(c.iter().copied().map(q).collect())
                .eval_f64(*x as f64)
                .to_string()
        })
        .collect();
    observed(
        "horner.algorithm",
        &["[];x=2", "[1,2];x=3", "[1,0,-1];x=1"],
        &av,
    );
    observed(
        "horner.ordinary",
        &["[];x=2", "[1,2];x=3", "[1,0,-1];x=1"],
        &ov,
    );
    let (mut h, mut d) = (0.0, 0.0);
    for c in [2.0, 1.0] {
        h = h * 3.0 + c;
        d = d * 3.0 + h;
    }
    assert_eq!(horner::evaluate(&[1.0, 2.0], 3.0).unwrap(), (7.0, 2.0));
    assert_eq!(d, 13.0); // using the new value breaks the derivative recurrence
}
#[test]
fn finite_value_does_not_imply_algorithm_success() {
    let huge: Q = format!("1{}", "0".repeat(308)).parse().unwrap();
    let p = Poly(vec![q(0), q(0), huge]);
    let cs: Vec<_> = p.0.iter().map(to_f64).collect();
    assert!(p.eval_f64(1.0).is_finite());
    assert_eq!(
        horner::evaluate(&cs, 1.0),
        Err(horner::Error::NonFiniteArithmetic)
    );
    let av = if horner::evaluate(&cs, 1.0).is_err() {
        "arithmetic_rejection"
    } else {
        "finite_value"
    };
    let ov = if p.eval_f64(1.0).is_finite() {
        "finite_value"
    } else {
        "nonfinite"
    };
    observed(
        "horner_overflow.algorithm",
        &["[0,0,10^308];x=1"],
        &[av.into()],
    );
    observed(
        "horner_overflow.ordinary",
        &["[0,0,10^308];x=1"],
        &[ov.into()],
    );
}
#[test]
fn actual_gate_root_basin_and_first_failed_tail() {
    let mut checked = 0;
    for n in 1..=4u32 {
        for mut code in 0..(n as usize + 1).pow(n) {
            let heads: Vec<_> = (0..n)
                .map(|_| {
                    let h = code % (n as usize + 1);
                    code /= n as usize + 1;
                    h
                })
                .collect();
            if heads.iter().filter(|&&h| h == 0).count() != 1 {
                continue;
            }
            let expected = basin(&heads).map_or(Ok(()), |i| {
                Err(format!("tree gate: cycle through word {i}"))
            });
            assert_eq!(gate(&heads), expected, "{heads:?}");
            checked += 1;
        }
    }
    assert_eq!(checked, 288);
    assert_eq!(basin(&[3, 0, 4, 3]), Some(1));
    let d = graph::decompose(&[0, 3, 0, 4, 3]).unwrap();
    assert_eq!(
        d.cycles
            .iter()
            .filter(|c| !c.contains(&0))
            .flatten()
            .min()
            .copied(),
        Some(3)
    );
    for n in [63, 64, 65, 66, 128] {
        let mut heads: Vec<_> = (0..n).collect();
        assert_eq!(gate(&heads), Ok(()));
        heads[0] = n;
        heads[n - 1] = n;
        heads[1] = 0;
        assert_eq!(gate(&heads), Err("tree gate: cycle through word 1".into()));
        assert_eq!(basin(&heads), Some(1));
    }
    let fixtures = [vec![0, 1], vec![3, 0, 4, 3], vec![2, 0]];
    let av: Vec<_> = fixtures
        .iter()
        .map(|h| basin(h).map_or("accepted".into(), |i| format!("first_failed_start={i}")))
        .collect();
    let ov: Vec<_> = fixtures
        .iter()
        .map(|h| {
            gate(h).map_or_else(
                |s| {
                    format!(
                        "first_failed_start={}",
                        s.strip_prefix("tree gate: cycle through word ").unwrap()
                    )
                },
                |_| "accepted".into(),
            )
        })
        .collect();
    observed(
        "root_basin.algorithm",
        &["[0,1]", "[3,0,4,3]", "[2,0]"],
        &av,
    );
    observed("root_basin.ordinary", &["[0,1]", "[3,0,4,3]", "[2,0]"], &ov);
}
#[test]
fn field_domain_boundary_refutes_solver_equivalence() {
    let p = modular::MODULUS;
    assert!(modular::solve(&[vec![p]], &[1], 1).unwrap().is_none());
    let a = vec![vec![q(p as i64)]];
    let b = vec![q(1)];
    let LinSol::Unique(x) = gauss(&a, &b) else {
        panic!("unique rational solution expected")
    };
    assert_eq!(x, vec![q(1) / q(p as i64)]);
    assert!(math::linsys::check_solution(&a, &b, &x).ok);
    assert_eq!(
        modular::solve(&[vec![p]], &[0], 1)
            .unwrap()
            .unwrap()
            .nullspace
            .len(),
        1
    );
    assert!(matches!(gauss(&a, &[q(0)]), LinSol::Unique(_)));
    for k in 0..=8u64 {
        let m = modular::solve(&[vec![1, k], vec![0, 1]], &[2 * k + 3, 2], 2)
            .unwrap()
            .unwrap();
        let LinSol::Unique(r) = gauss(
            &[vec![q(1), q(k as i64)], vec![q(0), q(1)]],
            &[q((2 * k + 3) as i64), q(2)],
        ) else {
            panic!()
        };
        assert_eq!(m.particular, vec![3, 2]);
        assert_eq!(r, vec![q(3), q(2)]);
    }
    let av = if modular::solve(&[vec![p]], &[1], 1).unwrap().is_none() {
        "inconsistent"
    } else {
        "consistent"
    };
    let ov = if matches!(gauss(&a, &b), LinSol::Inconsistent(_)) {
        "inconsistent"
    } else {
        "consistent"
    };
    observed(
        "field_boundary.algorithm",
        &["A=[[998244353]];b=[1]"],
        &[av.into()],
    );
    observed(
        "field_boundary.ordinary",
        &["A=[[998244353]];b=[1]"],
        &[ov.into()],
    );
}

#[test]
fn typed_catalogue_admits_scoped_relations_and_rejects_broader_claims() {
    use mova_correspondence::{Catalog, compare, compose_summary, validate_formula};
    let c: Catalog =
        serde_json::from_str(include_str!("../../../models/correspondences.json")).unwrap();
    for f in &c.formulas {
        validate_formula(f).unwrap();
    }
    let get = |id: &str| c.formulas.iter().find(|f| f.id == id).unwrap();
    for family in ["gcd", "horner", "root_basin", "gcd_zero"] {
        let result = compare(
            get(&format!("{family}.algorithm")),
            get(&format!("{family}.ordinary")),
        )
        .unwrap();
        assert_eq!(result.status, "finite_coincidence");
        assert!(result.finite_equal);
        assert!(!result.structural_match);
    }
    for role in ["reverse", "complement"] {
        assert!(
            compare(
                get(&format!("gcd.algorithm.{role}")),
                get(&format!("gcd.ordinary.{role}"))
            )
            .unwrap()
            .finite_equal
        );
    }
    let overflow = compare(
        get("horner_overflow.algorithm"),
        get("horner_overflow.ordinary"),
    )
    .unwrap();
    assert_eq!(overflow.status, "finite_counterexample");
    assert!(overflow.counterexample.is_some());
    assert_eq!(
        compare(
            get("field_boundary.algorithm"),
            get("field_boundary.ordinary")
        )
        .unwrap()
        .status,
        "incompatible_contract"
    );
    let fiber = compose_summary(get("gcd.algorithm"), get("gcd.ordinary.reverse")).unwrap();
    assert_eq!(fiber.status, "finite_composition");
    assert!(!fiber.functional);
    assert!(!fiber.identity);
}

fn digit_fixtures() -> Vec<(Vec<i64>, i64)> {
    vec![
        (vec![], 10),
        (vec![0], 10),
        (vec![1, 2], 10),
        (vec![0, 1, 2], 10),
        (vec![-1, 2], 10),
        (vec![1, -2], 10),
        (vec![1, 2], 1),
        (vec![1, 2], 0),
        (vec![1, 2], -2),
        (vec![1, 2, 3], -2),
        (vec![2, 1, 0], 3),
        (vec![2, -1, 2], 2),
    ]
}

#[test]
fn digit_horner_value_transport_carry_fibers_and_catalogue() {
    let mut checked = 0;
    let mut carries = 0;
    let mut normalized_checked = 0;
    for len in 0..=4u32 {
        for mut code in 0..5usize.pow(len) {
            let ds: Vec<i64> = (0..len)
                .map(|_| {
                    let d = code % 5;
                    code /= 5;
                    d as i64 - 2
                })
                .collect();
            for base in -3i64..=3 {
                // Positional power sum is an independent value oracle, not another Horner fold.
                let oracle: i128 = ds
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(k, d)| *d as i128 * (base as i128).pow(k as u32))
                    .sum();
                let zs: Vec<_> = ds.iter().copied().map(IBig::from).collect();
                let exact = math::nt::from_digits(&zs, &IBig::from(base));
                let ascending: Vec<_> = ds.iter().rev().map(|d| *d as f64).collect();
                let (value, derivative) = horner::evaluate(&ascending, base as f64).unwrap();
                assert_eq!(exact, IBig::from(oracle));
                assert_eq!(value, oracle as f64);
                let derivative_oracle: i128 = ds
                    .iter()
                    .rev()
                    .enumerate()
                    .skip(1)
                    .map(|(k, d)| k as i128 * *d as i128 * (base as i128).pow(k as u32 - 1))
                    .sum();
                assert_eq!(derivative, derivative_oracle as f64);
                if base >= 2 {
                    normalized_checked += 1;
                    let normalized = math::nt::digits(&exact, base as u32).unwrap();
                    let nz: Vec<_> = normalized.iter().copied().map(IBig::from).collect();
                    let projected = math::nt::from_digits(&nz, &IBig::from(base));
                    assert_eq!(projected, IBig::from(oracle.abs()));
                    assert_eq!(
                        math::nt::digits(&projected, base as u32).unwrap(),
                        normalized
                    );
                }
                checked += 1;
                for i in 0..ds.len().saturating_sub(1) {
                    for t in -2i64..=2 {
                        let mut other = ds.clone();
                        other[i] += t;
                        other[i + 1] -= base * t;
                        let other: Vec<_> = other.into_iter().map(IBig::from).collect();
                        assert_eq!(math::nt::from_digits(&other, &IBig::from(base)), exact);
                        carries += 1;
                    }
                }
            }
        }
    }
    assert_eq!(checked, 5467);
    assert_eq!(carries, 75250);
    assert_eq!(normalized_checked, 1562);
    // Plausible but false adapter: both APIs accept the same coefficient order.
    assert_eq!(horner::evaluate(&[1.0, 2.0], 10.0).unwrap().0, 21.0);
    assert_eq!(
        math::nt::from_digits(&[1.into(), 2.into()], &10.into()),
        12.into()
    );
    let fixtures = digit_fixtures();
    let names: Vec<_> = fixtures
        .iter()
        .map(|(ds, b)| format!("ds={ds:?};b={b}"))
        .collect();
    let refs: Vec<_> = names.iter().map(String::as_str).collect();
    let exact: Vec<_> = fixtures
        .iter()
        .map(|(ds, b)| {
            math::nt::from_digits(
                &ds.iter().copied().map(IBig::from).collect::<Vec<_>>(),
                &IBig::from(*b),
            )
            .to_string()
        })
        .collect();
    let rounded: Vec<_> = fixtures
        .iter()
        .map(|(ds, b)| {
            horner::evaluate(
                &ds.iter().rev().map(|d| *d as f64).collect::<Vec<_>>(),
                *b as f64,
            )
            .unwrap()
            .0
            .to_string()
        })
        .collect();
    observed("digit_horner.algorithm", &refs, &rounded);
    observed("digit_horner.ordinary", &refs, &exact);
}

#[test]
fn digit_canonical_magnitude_inverse_and_admission_boundaries() {
    let mut checked = 0;
    for base in 2u32..=16 {
        for n in -512i64..=512 {
            let ds = math::nt::digits(&IBig::from(n), base).unwrap();
            assert!(!ds.is_empty());
            assert!(ds.iter().all(|d| *d < base));
            assert!(ds.len() == 1 || ds[0] != 0);
            let zs: Vec<_> = ds.iter().copied().map(IBig::from).collect();
            let magnitude = math::nt::from_digits(&zs, &IBig::from(base));
            assert_eq!(magnitude, IBig::from(n.abs()));
            assert!(IBig::from(n) == magnitude || IBig::from(n) == -magnitude);
            assert_eq!(math::nt::digits(&IBig::from(-n), base).unwrap(), ds);
            checked += 1;
        }
    }
    assert_eq!(checked, 15375);
    for base in [0, 1] {
        assert_eq!(math::nt::digits(&0.into(), base), Err("base < 2".into()));
    }
    assert_eq!(
        math::nt::from_digits(&[1.into(), 2.into()], &0.into()),
        2.into()
    );
    assert_eq!(
        math::nt::from_digits(&[1.into(), 2.into()], &1.into()),
        3.into()
    );
    assert_eq!(math::nt::from_digits(&[], &(-7).into()), IBig::ZERO);
    assert_eq!(math::nt::digits(&0.into(), 10).unwrap(), vec![0]);
    let noncanonical = [0.into(), 1.into(), 2.into()];
    let value = math::nt::from_digits(&noncanonical, &10.into());
    assert_eq!(math::nt::digits(&value, 10).unwrap(), vec![1, 2]);
    assert_ne!(math::nt::digits(&value, 10).unwrap(), vec![0, 1, 2]);
    let negative = math::nt::from_digits(&[1.into(), (-20).into()], &10.into());
    assert_eq!(negative, (-10).into());
    let normalized = math::nt::digits(&negative, 10).unwrap();
    let rebuilt = math::nt::from_digits(
        &normalized.into_iter().map(IBig::from).collect::<Vec<_>>(),
        &10.into(),
    );
    assert_ne!(rebuilt, negative); // digit normalization preserves magnitude, not arbitrary signed values.
    let huge: IBig = format!("1{}", "0".repeat(308)).parse().unwrap();
    assert_eq!(
        math::nt::from_digits(&[huge.clone(), 0.into(), 0.into()], &1.into()),
        huge
    );
    assert_eq!(
        horner::evaluate(&[0.0, 0.0, math::big::z_f64(&huge)], 1.0),
        Err(horner::Error::NonFiniteArithmetic)
    );

    // Exact inputs and exact final value do not guarantee exact floating intermediates.
    let radix = IBig::ONE << 53usize;
    let square = IBig::ONE << 106usize;
    let ds = [IBig::ONE, IBig::ONE, -square.clone()];
    assert_eq!(math::nt::from_digits(&ds, &radix), radix);
    assert_eq!(
        horner::evaluate(
            &[-math::big::z_f64(&square), 1.0, 1.0],
            math::big::z_f64(&radix)
        )
        .unwrap()
        .0,
        0.0
    );
    let aliased_radix = &radix + IBig::ONE;
    assert_eq!(
        math::nt::from_digits(&[IBig::ONE, -radix.clone()], &aliased_radix),
        IBig::ONE
    );
    assert_eq!(
        horner::evaluate(
            &[-math::big::z_f64(&radix), 1.0],
            math::big::z_f64(&aliased_radix)
        )
        .unwrap()
        .0,
        0.0
    );
    // Even exact bounded value intermediates need a separate derivative-success premise.
    let mut cancelled = vec![IBig::ONE, IBig::from(-2)];
    cancelled.resize(1026, IBig::ZERO);
    assert_eq!(math::nt::from_digits(&cancelled, &2.into()), IBig::ZERO);
    assert_eq!(
        horner::evaluate(
            &cancelled
                .iter()
                .rev()
                .map(math::big::z_f64)
                .collect::<Vec<_>>(),
            2.0
        ),
        Err(horner::Error::NonFiniteArithmetic)
    );
    // Fixed-length antecedents use last = target - base*H(prefix).
    let prefix_value = math::nt::from_digits(&[4.into(), 7.into()], &10.into());
    let last = IBig::from(123) - IBig::from(10) * &prefix_value;
    assert_eq!(last, (-347).into());
    assert_eq!(
        math::nt::from_digits(&[4.into(), 7.into(), last], &10.into()),
        123.into()
    );
    assert_ne!(
        math::nt::from_digits(
            &[4.into(), 7.into(), IBig::from(123) - prefix_value],
            &10.into()
        ),
        123.into()
    );

    use mova_correspondence::{Catalog, compare, compose_summary};
    let c: Catalog =
        serde_json::from_str(include_str!("../../../models/correspondences.json")).unwrap();
    let get = |id: &str| c.formulas.iter().find(|f| f.id == id).unwrap();
    for (a, b) in [
        ("digit_horner.algorithm", "digit_horner.ordinary"),
        (
            "digit_horner.algorithm.reverse",
            "digit_horner.ordinary.reverse",
        ),
        (
            "digit_horner.algorithm.complement",
            "digit_horner.ordinary.complement",
        ),
    ] {
        assert!(compare(get(a), get(b)).unwrap().finite_equal);
    }
    let fiber = compose_summary(
        get("digit_horner.algorithm"),
        get("digit_horner.ordinary.reverse"),
    )
    .unwrap();
    assert!(!fiber.identity);
    assert!(!fiber.functional);
    let image = compose_summary(
        get("digit_horner.algorithm.reverse"),
        get("digit_horner.ordinary"),
    )
    .unwrap();
    assert!(image.identity); // only the finite listed codomain, every value of which has a listed antecedent.
}
