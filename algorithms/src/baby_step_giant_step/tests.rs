use super::*;
#[test]
fn direct_oracle() {
    for g in [0, 1, MODULUS - 1, 3, 7] {
        let mut v = 1;
        let mut seen = BTreeMap::new();
        for e in 0..20 {
            seen.entry(v).or_insert(e);
            v = v * g % MODULUS;
        }
        for (t, e) in seen {
            assert_eq!(discrete_log(g, t), Ok(Some(e)), "least exponent oracle");
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(discrete_log(0, 0), Ok(Some(1)));
    assert_eq!(discrete_log(0, 1), Ok(Some(0)));
    assert_eq!(discrete_log(0, 2), Ok(None));
    assert_eq!(discrete_log(1, 2), Ok(None));
    assert_eq!(discrete_log(MODULUS - 1, 3), Ok(None));
    assert_eq!(discrete_log(3, 0), Ok(None));
    assert_eq!(discrete_log(MODULUS + 3, MODULUS + 3), Ok(Some(1)));
    let e = MODULUS - 2;
    assert_eq!(discrete_log(3, pow(3, e)), Ok(Some(e)));
}
