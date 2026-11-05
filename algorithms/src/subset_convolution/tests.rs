use super::*;
#[test]
fn direct_oracle() {
    for d in 0..=7 {
        let n = 1 << d;
        let a: Vec<_> = (0..n)
            .map(|i| (i as u64).wrapping_mul(171717171717))
            .collect();
        let b: Vec<_> = (0..n).map(|i| u64::MAX - i as u64).collect();
        let got = convolve(&a, &b).unwrap();
        for (s, &v) in got.iter().enumerate() {
            let mut want = 0;
            for t in 0..n {
                if t & s == t {
                    want = (want + (a[t] % MODULUS) * (b[s ^ t] % MODULUS)) % MODULUS
                }
            }
            assert_eq!(v, want, "subset oracle");
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(convolve(&[], &[]), Err(Error::Shape));
    assert_eq!(convolve(&[1], &[1, 2]), Err(Error::Shape));
    assert_eq!(convolve(&[1; 3], &[1; 3]), Err(Error::Shape));
    assert_eq!(convolve(&vec![0; 4097], &[]), Err(Error::TooLarge));
    let mut unit = vec![0; 4096];
    unit[0] = 1;
    assert_eq!(convolve(&unit, &unit).unwrap(), unit);
}
