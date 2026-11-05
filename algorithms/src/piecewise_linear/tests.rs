use super::*;
#[test]
fn affine_exact() {
    let k = [
        Knot { x: -10, y: -27 },
        Knot { x: 0, y: 3 },
        Knot { x: 10, y: 33 },
    ];
    for x in -10..=10 {
        let c = interpolate(&k, x, 0, 3).unwrap();
        assert_eq!(
            c.numerator,
            i128::from(3 * x + 3) * i128::from(c.denominator),
            "affine reproduction"
        );
        assert_eq!(c.error_numerator, 0);
    }
}
#[test]
fn quadratic_bound() {
    let k = [
        Knot { x: -10, y: 100 },
        Knot { x: 0, y: 0 },
        Knot { x: 10, y: 100 },
    ];
    for x in -10..=10 {
        let c = interpolate(&k, x, 2, 3).unwrap();
        let error = (c.numerator - i128::from(x * x) * i128::from(c.denominator)).unsigned_abs();
        assert!(
            error * 8 <= c.error_numerator * u128::from(c.denominator),
            "curvature bound"
        );
        if x == 5 {
            assert_eq!(
                error * 8,
                c.error_numerator * u128::from(c.denominator),
                "sharp midpoint"
            );
        }
    }
}
#[test]
fn validation_and_extremes() {
    let k = [Knot { x: 0, y: 0 }, Knot { x: 4, y: 1 }];
    assert_eq!(
        interpolate(&k, 1, 0, 2).unwrap().numerator,
        1,
        "rational numerator"
    );
    assert_eq!(interpolate(&k, 1, 0, 2).unwrap().denominator, 4);
    assert_eq!(interpolate(&k, 5, 0, 2), Err(Error::OutsideDomain));
    assert_eq!(interpolate(&k, 1, 0, 1), Err(Error::WorkBudget));
    assert_eq!(interpolate(&k[..1], 0, 0, 2), Err(Error::TooFewKnots));
    assert_eq!(interpolate(&[k[0], k[0]], 0, 0, 2), Err(Error::Unordered));
    let wide = [
        Knot {
            x: i64::MIN,
            y: i64::MIN,
        },
        Knot {
            x: i64::MAX,
            y: i64::MAX,
        },
    ];
    assert_eq!(interpolate(&wide, 0, u64::MAX, 2), Err(Error::Overflow));
    let c = interpolate(&wide, 0, 0, 2).unwrap();
    assert_eq!(c.numerator, 0);
    assert_eq!(
        interpolate(&wide, i64::MAX, u64::MAX, 2)
            .unwrap()
            .error_numerator,
        0
    );
}
