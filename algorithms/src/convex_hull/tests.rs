use super::*;

// Independent shoelace determinant, rather than production's difference product.
fn area(a: Point, b: Point, c: Point) -> i128 {
    i128::from(a.x) * i128::from(b.y)
        + i128::from(b.x) * i128::from(c.y)
        + i128::from(c.x) * i128::from(a.y)
        - i128::from(a.y) * i128::from(b.x)
        - i128::from(b.y) * i128::from(c.x)
        - i128::from(c.y) * i128::from(a.x)
}

fn check(points: &[Point]) {
    let hull = convex_hull(points);
    let mut distinct = points.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(hull.first(), distinct.first());
    assert!(hull.iter().all(|p| points.contains(p)));
    assert_eq!(
        hull.iter().collect::<std::collections::BTreeSet<_>>().len(),
        hull.len()
    );
    if hull.len() <= 1 {
        assert_eq!(hull, distinct);
    } else if hull.len() == 2 {
        assert_eq!(hull[1], *distinct.last().unwrap());
        assert!(points.iter().all(|&p| area(hull[0], hull[1], p) == 0));
    } else {
        for i in 0..hull.len() {
            let a = hull[i];
            let b = hull[(i + 1) % hull.len()];
            let c = hull[(i + 2) % hull.len()];
            assert!(area(a, b, c) > 0);
            assert!(points.iter().all(|&p| area(a, b, p) >= 0));
        }
    }
    // All support values must agree, including directions normal to every input pair.
    for a in &distinct {
        for b in &distinct {
            let dx = i128::from(a.y) - i128::from(b.y);
            let dy = i128::from(b.x) - i128::from(a.x);
            let support = |p: &Point| dx * i128::from(p.x) + dy * i128::from(p.y);
            assert_eq!(
                points.iter().map(support).max(),
                hull.iter().map(support).max()
            );
        }
    }
}

#[test]
fn every_grid_subset_has_support_and_strict_turns() {
    let grid: Vec<_> = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| Point { x, y }))
        .collect();
    for mask in 0..512 {
        let points: Vec<_> = grid
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, p)| *p)
            .collect();
        check(&points);
    }
}

#[test]
fn duplicates_order_and_collinear_fibers() {
    let corners = vec![
        Point { x: 0, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 2, y: 2 },
        Point { x: 0, y: 2 },
    ];
    let expected = convex_hull(&corners);
    for extra in [Point { x: 1, y: 1 }, Point { x: 1, y: 0 }, corners[0]] {
        let mut input = corners.clone();
        input.push(extra);
        input.reverse();
        assert_eq!(convex_hull(&input), expected);
        check(&input);
    }
    let line = vec![
        Point { x: -2, y: -2 },
        Point { x: 0, y: 0 },
        Point { x: 2, y: 2 },
        Point { x: 0, y: 0 },
    ];
    assert_eq!(convex_hull(&line), vec![line[0], line[2]]);
    check(&line);
}

#[test]
fn coordinate_limits_and_translation_scaling() {
    let extrema = [i32::MIN, 0, i32::MAX];
    let full: Vec<_> = extrema
        .iter()
        .flat_map(|&x| extrema.iter().map(move |&y| Point { x, y }))
        .collect();
    check(&full);
    let input = [
        Point { x: -3, y: 2 },
        Point { x: 1, y: 5 },
        Point { x: 5, y: -2 },
        Point { x: 0, y: 1 },
    ];
    let map = |p: Point| Point {
        x: 3 * p.x + 17,
        y: 3 * p.y - 11,
    };
    assert_eq!(
        convex_hull(&input.map(map)),
        convex_hull(&input).into_iter().map(map).collect::<Vec<_>>()
    );
    assert_eq!(convex_hull(&convex_hull(&input)), convex_hull(&input));
}
