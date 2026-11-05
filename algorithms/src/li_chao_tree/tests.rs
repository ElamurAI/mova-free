use super::*;
#[test]
fn exhaustive_envelopes() {
    let xs = [-7, -2, 0, 1, 9];
    let candidates: Vec<_> = (-2..=2)
        .flat_map(|slope| (-2..=2).map(move |intercept| Line { slope, intercept }))
        .collect();
    for &a in &candidates {
        for &b in &candidates {
            for &c in &candidates {
                let mut tree = LiChaoTree::new(&xs).unwrap();
                let lines = [a, b, c];
                for end in 0..3 {
                    assert_eq!(tree.insert(lines[end]), Ok(end));
                    for &x in &xs {
                        let expected = lines[..=end]
                            .iter()
                            .enumerate()
                            .map(|(line_id, l)| Minimum {
                                value: i128::from(l.slope) * i128::from(x)
                                    + i128::from(l.intercept),
                                line_id,
                            })
                            .min()
                            .unwrap();
                        assert_eq!(tree.query(x), Ok(expected), "lines={lines:?}, x={x}");
                    }
                }
            }
        }
    }
}
#[test]
fn extremes_ties_and_errors() {
    assert!(matches!(LiChaoTree::new(&[]), Err(Error::EmptyCoordinates)));
    assert!(matches!(
        LiChaoTree::new(&vec![0; MAX_COORDINATES + 1]),
        Err(Error::TooManyCoordinates)
    ));
    let mut tree = LiChaoTree::new(&[i64::MAX, 0, i64::MIN, 0]).unwrap();
    assert_eq!(tree.query(1), Err(Error::UnknownCoordinate));
    assert_eq!(tree.query(0), Err(Error::NoLines));
    let lines = [
        Line {
            slope: i64::MIN,
            intercept: i64::MIN,
        },
        Line {
            slope: i64::MAX,
            intercept: i64::MAX,
        },
        Line {
            slope: 0,
            intercept: 0,
        },
    ];
    for line in lines {
        tree.insert(line).unwrap();
    }
    for x in [i64::MIN, 0, i64::MAX] {
        let expected = lines
            .iter()
            .enumerate()
            .map(|(line_id, l)| Minimum {
                value: i128::from(l.slope) * i128::from(x) + i128::from(l.intercept),
                line_id,
            })
            .min()
            .unwrap();
        assert_eq!(tree.query(x), Ok(expected));
    }
    let mut tie = LiChaoTree::new(&[0]).unwrap();
    for _ in 0..MAX_LINES {
        tie.insert(Line {
            slope: 1,
            intercept: 2,
        })
        .unwrap();
    }
    assert_eq!(
        tie.query(0),
        Ok(Minimum {
            value: 2,
            line_id: 0
        })
    );
    assert_eq!(
        tie.insert(Line {
            slope: 0,
            intercept: -1
        }),
        Err(Error::TooManyLines)
    );
    assert_eq!(
        tie.query(0),
        Ok(Minimum {
            value: 2,
            line_id: 0
        })
    );
}
#[test]
fn full_coordinate_cap() {
    let xs: Vec<_> = (0..MAX_COORDINATES as i64).rev().collect();
    let mut tree = LiChaoTree::new(&xs).unwrap();
    tree.insert(Line {
        slope: -2,
        intercept: 3,
    })
    .unwrap();
    for x in [0, 1, MAX_COORDINATES as i64 - 1] {
        assert_eq!(tree.query(x).unwrap().value, 3 - 2 * i128::from(x));
    }
}
