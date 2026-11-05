use super::*;
#[test]
fn exhaustive_canonical_roundtrips() {
    for n in 0..=8 {
        for mut code in 0..3usize.pow(n) {
            let input: Vec<u8> = (0..n)
                .map(|_| {
                    let b = (code % 3) as u8;
                    code /= 3;
                    b
                })
                .collect();
            let runs = encode(&input);
            assert_eq!(decode(&runs, input.len()), Ok(input.clone()));
            assert!(runs.iter().all(|r| r.count > 0));
            assert!(runs.windows(2).all(|w| w[0].byte != w[1].byte));
            let expected = if input.is_empty() {
                0
            } else {
                1 + input.windows(2).filter(|w| w[0] != w[1]).count()
            };
            assert_eq!(runs.len(), expected);
            assert_eq!(runs.iter().map(|r| r.count).sum::<usize>(), input.len());
            assert_eq!(encode(&decode(&runs, input.len()).unwrap()), runs);
        }
    }
}
#[test]
fn validation_and_limits() {
    let r = |byte, count| Run { byte, count };
    assert_eq!(decode(&[r(0, 0)], 0), Err(DecodeError::ZeroCount));
    assert_eq!(
        decode(&[r(0, 1), r(0, 1)], 0),
        Err(DecodeError::AdjacentEqual)
    );
    assert_eq!(
        decode(&[r(0, usize::MAX), r(1, 1)], 0),
        Err(DecodeError::LengthOverflow)
    );
    assert_eq!(decode(&[r(0, 2)], 1), Err(DecodeError::OutputLimit));
    assert_eq!(
        decode(&[r(0, usize::MAX)], usize::MAX),
        Err(DecodeError::Allocation)
    );
    assert_eq!(decode(&[], 0), Ok(vec![]));
    assert_eq!(decode(&[r(255, 3)], 3), Ok(vec![255; 3]));
}
#[test]
fn all_bytes_and_long_run() {
    let bytes: Vec<u8> = (0..=255).collect();
    assert_eq!(
        encode(&bytes),
        bytes
            .iter()
            .map(|&byte| Run { byte, count: 1 })
            .collect::<Vec<_>>()
    );
    assert_eq!(
        encode(&vec![42; 10000]),
        vec![Run {
            byte: 42,
            count: 10000
        }]
    );
}
