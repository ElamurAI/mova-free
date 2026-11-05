use super::suffix_array;

fn oracle(text: &[u8]) -> Vec<usize> {
    // Direct suffix comparison, independent of rank doubling.
    let mut remaining: Vec<usize> = (0..text.len()).collect();
    let mut output = Vec::new();
    while !remaining.is_empty() {
        let minimum = (0..remaining.len())
            .min_by(|&a, &b| text[remaining[a]..].cmp(&text[remaining[b]..]))
            .unwrap();
        output.push(remaining.remove(minimum));
    }
    output
}

fn words(n: usize) -> impl Iterator<Item = Vec<u8>> {
    (0..3usize.pow(n as u32)).map(move |mut code| {
        (0..n)
            .map(|_| {
                let byte = (code % 3) as u8;
                code /= 3;
                byte
            })
            .collect()
    })
}

#[test]
fn exhaustive_against_direct_suffix_oracle() {
    for n in 0..=8 {
        for text in words(n) {
            assert_eq!(suffix_array(&text), oracle(&text));
        }
    }
}

#[test]
fn boundaries_and_long_common_prefixes() {
    assert_eq!(suffix_array(b""), []);
    assert_eq!(suffix_array(b"x"), [0]);
    assert_eq!(suffix_array(b"banana"), [5, 3, 1, 0, 4, 2]);
    let repeated = vec![255; 1025];
    assert_eq!(suffix_array(&repeated), (0..1025).rev().collect::<Vec<_>>());
    let all_bytes: Vec<u8> = (0..=255).rev().collect();
    assert_eq!(suffix_array(&all_bytes), oracle(&all_bytes));
    let periodic: Vec<u8> = (0..1025).map(|i| (i % 7) as u8).collect();
    assert_eq!(suffix_array(&periodic), oracle(&periodic));
}

#[test]
fn increasing_relabel_and_unique_minimum_prefix() {
    for text in words(6) {
        let mapped: Vec<u8> = text.iter().map(|b| 9 + 7 * b).collect();
        assert_eq!(suffix_array(&text), suffix_array(&mapped));
        let mut prefixed = vec![0];
        prefixed.extend(&mapped);
        let mut expected = vec![0];
        expected.extend(suffix_array(&mapped).into_iter().map(|i| i + 1));
        assert_eq!(suffix_array(&prefixed), expected);
    }
    // Injective but decreasing maps do not preserve lexicographic order.
    assert_ne!(suffix_array(&[0, 1]), suffix_array(&[255, 254]));
}

#[test]
fn compatible_order_fiber_retains_constraints_not_bytes() {
    let target = vec![2, 1, 0];
    let mut predecessors = 0;
    for text in words(3) {
        // This order holds precisely for nonincreasing three-byte strings.
        let compatible = text[0] >= text[1] && text[1] >= text[2];
        assert_eq!(suffix_array(&text) == target, compatible);
        predecessors += usize::from(compatible);
    }
    assert_eq!(predecessors, 10);
    assert_eq!(suffix_array(&[2, 1, 0]), suffix_array(&[7, 7, 7]));
    assert_ne!(suffix_array(&[0, 1, 2]), target);
}
