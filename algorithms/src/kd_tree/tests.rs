use super::*;
fn oracle(points: &[[i32; 2]], q: [i32; 2]) -> Option<(usize, u64)> {
    points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let d = (i128::from(q[0]) - i128::from(p[0])).pow(2)
                + (i128::from(q[1]) - i128::from(p[1])).pow(2);
            (u64::try_from(d).unwrap(), i)
        })
        .min()
        .map(|(d, i)| (i, d))
}
#[test]
fn exhaustive_grid_oracle() {
    let grid = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 0],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
    for mask in 0..512 {
        let points: Vec<_> = grid
            .iter()
            .enumerate()
            .filter(|&(i, _)| mask & (1 << i) != 0)
            .map(|(_, p)| *p)
            .collect();
        let tree = KdTree::new(&points).unwrap();
        for x in -2..=2 {
            for y in -2..=2 {
                assert_eq!(tree.nearest([x, y]), oracle(&points, [x, y]));
            }
        }
    }
}
#[test]
fn ties_duplicates_and_extremes() {
    for p in [
        vec![[1, 0], [-1, 0], [0, 1], [0, -1]],
        vec![[0, 0]; 17],
        vec![[-1_000_000, -1_000_000], [1_000_000, 1_000_000]],
    ] {
        let t = KdTree::new(&p).unwrap();
        for q in [
            [0, 0],
            [i32::MIN, i32::MIN],
            [i32::MAX, i32::MAX],
            [i32::MIN, i32::MAX],
        ] {
            assert_eq!(t.nearest(q), oracle(&p, q));
        }
    }
    assert_eq!(
        KdTree::new(&[[1, 0], [-1, 0]]).unwrap().nearest([0, 0]),
        Some((0, 1))
    );
}
#[test]
fn validation_and_capacity() {
    assert_eq!(KdTree::default().nearest([0, 0]), None);
    assert_eq!(
        KdTree::new(&[[i32::MAX, 0]; 4097]).unwrap_err().to_string(),
        "too many points"
    );
    assert_eq!(
        KdTree::new(&[[1_000_001, 0]]).unwrap_err().to_string(),
        "coordinate out of range"
    );
    assert_eq!(
        KdTree::new(&[[0, 0]; 4096]).unwrap().nearest([0, 0]),
        Some((0, 0))
    );
}
