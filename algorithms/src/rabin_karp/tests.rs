use super::*;
fn words(max: usize) -> Vec<Vec<u8>> {
    let mut all = vec![vec![]];
    for len in 1..=max {
        for bits in 0..1usize << len {
            all.push((0..len).map(|i| ((bits >> i) & 1) as u8).collect());
        }
    }
    all
}

fn oracle(t: &[u8], p: &[u8]) -> Vec<usize> {
    (0..=t.len())
        .filter(|&i| t.get(i..i.saturating_add(p.len())) == Some(p))
        .collect()
}
#[test]
fn bounded_oracle() {
    let texts = words(7);
    let patterns = words(4);
    for t in &texts {
        for p in &patterns {
            assert_eq!(find_all(t, p), oracle(t, p));
        }
    }
}
#[test]
fn boundaries_overlap_prefix_bytes() {
    assert_eq!(find_all(b"aaaaa", b"aaa"), vec![0, 1, 2]);
    assert_eq!(find_all(b"abababa", b"aba"), vec![0, 2, 4]);
    assert_eq!(find_all(b"", b""), vec![0]);
    assert_eq!(find_all(b"a", b""), vec![0, 1]);
    assert!(find_all(b"a", b"ab").is_empty());
    assert_eq!(find_all(&[255, 0, 255], &[255]), vec![0, 2]);
    assert_eq!(find_all("éé".as_bytes(), "é".as_bytes()), vec![0, 2]);
}
#[test]
fn compatible_preimages_and_relabeling() {
    for t in words(5) {
        for p in words(3) {
            let y = find_all(&t, &p);
            let fiber: Vec<_> = words(5)
                .into_iter()
                .filter(|x| oracle(x, &p) == y)
                .collect();
            assert!(fiber.contains(&t));
            let flip = |v: &[u8]| v.iter().map(|b| b ^ 1).collect::<Vec<_>>();
            assert_eq!(find_all(&flip(&t), &flip(&p)), y);
            for i in 0..=t.len() {
                assert_eq!(y.contains(&i), t.get(i..i + p.len()) == Some(p.as_slice()));
            }
        }
    }
}

#[test]
fn actual_wrapping_collision_is_rejected() {
    // This base-257 polynomial equals 2^64, hence hashes to zero.
    let collision = [249, 27, 201, 69, 201, 27, 249, 1];
    assert!(find_all(&collision, &[0; 8]).is_empty());
    let mut t = collision.to_vec();
    t.extend_from_slice(&[0; 8]);
    assert_eq!(find_all(&t, &[0; 8]), vec![8]);
}
