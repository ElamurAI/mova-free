use super::*;

fn brute(points: &[Point]) -> Option<ClosestPair> {
    let mut candidates = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dx = (i64::from(points[i].x) - i64::from(points[j].x)).unsigned_abs();
            let dy = (i64::from(points[i].y) - i64::from(points[j].y)).unsigned_abs();
            candidates.push((u128::from(dx).pow(2) + u128::from(dy).pow(2), (i, j)));
        }
    }
    candidates
        .into_iter()
        .min()
        .map(|(distance_squared, indices)| ClosestPair {
            indices,
            distance_squared,
        })
}

#[test]
fn exhaustive_sequences_match_independent_pairs() {
    let alphabet = [
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 2, y: 2 },
    ];
    for len in 0..=6u32 {
        for mut code in 0..4usize.pow(len) {
            let points: Vec<_> = (0..len)
                .map(|_| {
                    let p = alphabet[code % 4];
                    code /= 4;
                    p
                })
                .collect();
            assert_eq!(closest_pair(&points), brute(&points), "{points:?}");
        }
    }
}

#[test]
fn full_coordinates_and_cross_split_ties() {
    let values = [i32::MIN, -1, 0, 1, i32::MAX];
    let all: Vec<_> = values
        .iter()
        .flat_map(|&x| values.iter().map(move |&y| Point { x, y }))
        .collect();
    for shift in 0..all.len() {
        let mut points = all.clone();
        points.rotate_left(shift);
        assert_eq!(closest_pair(&points), brute(&points));
    }
    let corners = [
        Point {
            x: i32::MIN,
            y: i32::MIN,
        },
        Point {
            x: i32::MAX,
            y: i32::MAX,
        },
    ];
    assert_eq!(
        closest_pair(&corners).unwrap().distance_squared,
        2 * u128::from(u32::MAX).pow(2)
    );
    let ties = [
        Point { x: 0, y: 0 },
        Point { x: 0, y: 2 },
        Point { x: -2, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: -2 },
    ];
    assert_eq!(closest_pair(&ties), brute(&ties));
}

#[test]
fn duplicate_runs_choose_original_indices() {
    let points = [
        Point { x: 9, y: 9 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 9, y: 9 },
        Point { x: 9, y: 9 },
    ];
    assert_eq!(closest_pair(&points).unwrap().indices, (0, 3));
    assert_eq!(
        closest_pair(&vec![Point { x: 7, y: 7 }; 10000])
            .unwrap()
            .indices,
        (0, 1)
    );
}

#[test]
fn isometries_scaling_and_ambiguous_preimages() {
    let points = [
        Point { x: 0, y: 0 },
        Point { x: 3, y: 4 },
        Point { x: 20, y: 20 },
        Point { x: -20, y: 30 },
    ];
    let result = closest_pair(&points).unwrap();
    let shifted = points.map(|p| Point {
        x: 50 - p.y,
        y: 30 + p.x,
    });
    assert_eq!(closest_pair(&shifted), Some(result));
    let scaled = points.map(|p| Point {
        x: 3 * p.x,
        y: 3 * p.y,
    });
    assert_eq!(
        closest_pair(&scaled),
        Some(ClosestPair {
            distance_squared: 9 * result.distance_squared,
            ..result
        })
    );
    let other = [
        Point { x: 0, y: 0 },
        Point { x: 0, y: 5 },
        Point { x: 90, y: 90 },
    ];
    assert_eq!(closest_pair(&other), Some(result));
    assert_ne!(points.to_vec(), other.to_vec());
}

#[test]
fn deterministic_clouds_and_vertical_strip() {
    let mut state = 917u32;
    for len in [4, 7, 16, 31, 64, 129] {
        let points: Vec<_> = (0..len)
            .map(|_| {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                let x = state as i32;
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                Point { x, y: state as i32 }
            })
            .collect();
        assert_eq!(closest_pair(&points), brute(&points));
    }
    let vertical: Vec<_> = (0..257).rev().map(|y| Point { x: 0, y: 2 * y }).collect();
    assert_eq!(closest_pair(&vertical), brute(&vertical));
}
