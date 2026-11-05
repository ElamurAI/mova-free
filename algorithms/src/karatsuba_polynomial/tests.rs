use super::*;
#[test]
fn direct_oracle() {
    for n in [0, 1, 15, 16, 17, 31, 32, 65, 129] {
        for m in [0, 1, 7, 17, 33] {
            let a: Vec<_> = (0..n).map(|i| u64::MAX - i as u64 * 37).collect();
            let b: Vec<_> = (0..m).map(|i| i as u64 * 111 + 5).collect();
            let mut want = vec![0; if n * m == 0 { 0 } else { n + m - 1 }];
            for i in 0..n {
                for j in 0..m {
                    want[i + j] = (want[i + j] + (a[i] % MODULUS) * (b[j] % MODULUS)) % MODULUS;
                }
            }
            assert_eq!(multiply(&a, &b).unwrap(), want, "schoolbook oracle");
        }
    }
}
#[test]
fn boundaries() {
    assert_eq!(multiply(&[], &vec![0; 4097]), Err(Error::TooLarge));
    assert_eq!(multiply(&[0, 0], &[0, 0]).unwrap(), vec![0; 3]);
    let a = vec![MODULUS - 1; 4096];
    assert_eq!(multiply(&a, &[1]).unwrap(), a);
}
