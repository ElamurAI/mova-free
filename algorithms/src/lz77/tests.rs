use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    (0..=max)
        .flat_map(|n| {
            (0..1usize << n).map(move |bits| (0..n).map(|i| ((bits >> i) & 1) as u8).collect())
        })
        .collect()
}
#[test]
fn exhaustive_roundtrip() {
    for s in words(9) {
        for w in [1, 3, 8] {
            let t = encode(&s, w, 8).unwrap();
            assert_eq!(decode(&t, s.len()), Ok(s.clone()));
        }
    }
}
#[test]
fn greedy_overlap_and_invalid() {
    assert_eq!(
        encode(b"aaaaaa", 4, 8),
        Ok(vec![
            Token::Literal(b'a'),
            Token::Match {
                distance: 1,
                length: 5
            }
        ])
    );
    assert_eq!(
        decode(
            &[Token::Match {
                distance: 1,
                length: 3
            }],
            99
        ),
        Err(Error::InvalidMatch)
    );
    assert_eq!(
        decode(
            &[
                Token::Literal(1),
                Token::Match {
                    distance: 0,
                    length: 3
                }
            ],
            99
        ),
        Err(Error::InvalidMatch)
    );
    assert_eq!(
        decode(
            &[
                Token::Literal(1),
                Token::Match {
                    distance: 1,
                    length: usize::MAX
                }
            ],
            99
        ),
        Err(Error::OutputLimit)
    );
    assert_eq!(
        decode(
            &[
                Token::Literal(1),
                Token::Match {
                    distance: 1,
                    length: 0
                }
            ],
            99
        ),
        Err(Error::InvalidMatch)
    );
    assert_eq!(encode(b"a", 0, 3), Err(Error::InvalidConfiguration));
}
#[test]
fn independent_greedy() {
    for s in words(8) {
        let t = encode(&s, 4, 5).unwrap();
        let mut i = 0;
        for token in t {
            let candidates: Vec<(usize, usize)> = (1..=i.min(4))
                .map(|d| {
                    let n = (0..5)
                        .take_while(|&k| i + k < s.len() && s[i + k] == s[i + k - d])
                        .count();
                    (d, n)
                })
                .collect();
            let max = candidates.iter().map(|p| p.1).max().unwrap_or(0);
            match token {
                Token::Literal(b) => {
                    assert!(max < 3);
                    assert_eq!(b, s[i]);
                    i += 1;
                }
                Token::Match { distance, length } => {
                    assert_eq!(length, max);
                    assert_eq!(distance, candidates.iter().find(|p| p.1 == max).unwrap().0);
                    i += length;
                }
            }
        }
        assert_eq!(i, s.len());
    }
}
