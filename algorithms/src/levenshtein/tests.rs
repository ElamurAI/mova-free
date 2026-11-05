use super::*;

fn table(a: &[u8], b: &[u8]) -> usize {
    let mut d = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in d[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]));
        }
    }
    d[a.len()][b.len()]
}

#[test]
fn exhaustive_table() {
    let words: Vec<Vec<u8>> = (0..=5)
        .flat_map(|n| {
            (0..1usize << n).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect();
    for a in &words {
        for b in &words {
            let d = distance(a, b);
            assert_eq!(d, table(a, b), "{a:?} {b:?}");
            assert_eq!(d, distance(b, a));
            assert!(a.len().abs_diff(b.len()) <= d && d <= a.len().max(b.len()));
        }
    }
}

#[test]
#[should_panic(expected = "edit-distance dimension overflow")]
fn maximal_zst_dimension_is_rejected() {
    distance(&[(); usize::MAX], &[()]);
}

#[test]
fn caller_chooses_unicode_units() {
    let a: Vec<char> = "café🙂".chars().collect();
    let b: Vec<char> = "cafe🙂".chars().collect();
    assert_eq!(distance(&a, &b), 1);
    assert_eq!(distance("café🙂".as_bytes(), "cafe🙂".as_bytes()), 2);
    assert_eq!(distance(&[10, 20, 30], &[10, 30]), 1);
    assert_eq!(distance::<char>(&[], &a), 5);
}

#[test]
fn wider_alphabet_longer_inputs_match_table() {
    let alphabet = [0, 127, 128, 255];
    let mut seed = 0xd1b54a32d192ed03u64;
    for case in 0..512 {
        let mut make = |len| -> Vec<u8> {
            (0..len)
                .map(|_| {
                    seed = seed
                        .wrapping_mul(2862933555777941757)
                        .wrapping_add(3037000493);
                    alphabet[(seed >> 32) as usize % alphabet.len()]
                })
                .collect()
        };
        let a = make(case % 17);
        let b = make((case * 7) % 19);
        assert_eq!(
            distance(&a, &b),
            table(&a, &b),
            "case={case}, a={a:?}, b={b:?}"
        );
    }
}

#[test]
fn unicode_metric_triangle_and_independent_encoding_oracle() {
    // Mapping is injective on this alphabet, so Eq and unit edit costs are preserved.
    let alphabet = ['é', '🙂', '\u{301}'];
    let mut words = vec![vec![]];
    for &a in &alphabet {
        words.push(vec![a]);
        for &b in &alphabet {
            words.push(vec![a, b]);
        }
    }
    let encode = |s: &[char]| -> Vec<u8> {
        s.iter()
            .map(|c| alphabet.iter().position(|x| x == c).unwrap() as u8)
            .collect()
    };
    for a in &words {
        for b in &words {
            let ab = distance(a, b);
            assert_eq!(ab, table(&encode(a), &encode(b)), "a={a:?}, b={b:?}");
            assert_eq!(ab == 0, a == b, "a={a:?}, b={b:?}");
            assert_eq!(ab, distance(b, a));
            for c in &words {
                assert!(
                    ab <= distance(a, c) + distance(c, b),
                    "triangle a={a:?}, b={b:?}, c={c:?}"
                );
            }
        }
    }
    assert_eq!(distance(&['é'], &['e', '\u{301}']), 2);
}
