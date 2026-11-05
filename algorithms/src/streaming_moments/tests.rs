use super::*;
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-10 * (1.0 + b.abs()),
        "moment oracle actual={a} expected={b}"
    );
}
#[test]
fn independent_integer_oracle_and_merge() {
    for code in 0..3125usize {
        let mut c = code;
        let mut values = Vec::new();
        let (mut sum, mut squares) = (0i128, 0i128);
        for _ in 0..5 {
            let x = (c % 5) as i128 - 2;
            c /= 5;
            sum += x;
            squares += x * x;
            values.push(x as f64);
        }
        let expected_mean = sum as f64 / 5.;
        let expected_var = (5 * squares - sum * sum) as f64 / 25.;
        let mut all = StreamingMoments::new();
        for &x in &values {
            all.update(x).unwrap();
        }
        near(all.mean().unwrap(), expected_mean);
        near(all.population_variance().unwrap(), expected_var);
        near(all.sample_variance().unwrap(), expected_var * 1.25);
        for split in 0..=5 {
            let mut a = StreamingMoments::new();
            let mut b = a.clone();
            for &x in &values[..split] {
                a.update(x).unwrap();
            }
            for &x in &values[split..] {
                b.update(x).unwrap();
            }
            a.merge(&b).unwrap();
            assert_eq!(a.count(), 5);
            near(a.mean().unwrap(), expected_mean);
            near(a.population_variance().unwrap(), expected_var);
        }
    }
}
#[test]
fn translation_and_constant_fiber() {
    let mut a = StreamingMoments::new();
    let mut b = a.clone();
    for x in [-2., -1., 0., 1., 2.] {
        a.update(x).unwrap();
        b.update(x + 1e12).unwrap();
    }
    near(a.population_variance().unwrap(), 2.);
    near(b.population_variance().unwrap(), 2.);
    let mut a = StreamingMoments::new();
    let mut b = a.clone();
    for x in [1., 3.] {
        a.update(x).unwrap();
    }
    for x in [3., 1.] {
        b.update(x).unwrap();
    }
    assert_eq!(a, b, "moment fibers discard order");
    let mut c = StreamingMoments::new();
    for _ in 0..100 {
        c.update(f64::MAX).unwrap();
    }
    assert_eq!(c.population_variance(), Some(0.));
}
#[test]
fn invalid_overflow_and_count_are_atomic() {
    let mut s = StreamingMoments::new();
    assert_eq!(s.mean(), None);
    assert_eq!(s.population_variance(), None);
    assert_eq!(s.sample_variance(), None);
    s.update(1.).unwrap();
    assert_eq!(s.sample_variance(), None);
    for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let old = s.clone();
        assert_eq!(s.update(x), Err(Error::NonFinite));
        assert_eq!(s, old);
    }
    let mut a = StreamingMoments::new();
    a.update(f64::MAX).unwrap();
    let old = a.clone();
    assert_eq!(a.update(-f64::MAX), Err(Error::Overflow));
    assert_eq!(a, old);
    let mut b = StreamingMoments::new();
    b.update(-f64::MAX).unwrap();
    assert_eq!(a.merge(&b), Err(Error::Overflow));
    assert_eq!(a, old);
    s.count = 1 << 53;
    let old = s.clone();
    assert_eq!(s.update(1.), Err(Error::CountLimit));
    assert_eq!(s, old);
    assert_eq!(s.merge(&b), Err(Error::CountLimit));
    assert_eq!(s, old);
}
#[test]
fn variance_mutation_guard() {
    let mut s = StreamingMoments::new();
    s.update(0.).unwrap();
    s.update(2.).unwrap();
    assert_eq!(
        s.population_variance(),
        Some(1.),
        "Welford residual yields population variance one"
    );
    assert_eq!(s.sample_variance(), Some(2.));
}
