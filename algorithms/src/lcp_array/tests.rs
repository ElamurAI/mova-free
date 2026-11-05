use super::lcp_array;
fn words(n: usize) -> impl Iterator<Item = Vec<u8>> {
    (0..3usize.pow(n as u32)).map(move |mut c| {
        (0..n)
            .map(|_| {
                let b = (c % 3) as u8;
                c /= 3;
                b
            })
            .collect()
    })
}
fn oracle(t: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let mut sa: Vec<_> = (0..t.len()).collect();
    sa.sort_by(|&a, &b| t[a..].cmp(&t[b..]));
    let mut lcp = vec![0; t.len()];
    for r in 1..sa.len() {
        lcp[r] = t[sa[r - 1]..]
            .iter()
            .zip(&t[sa[r]..])
            .take_while(|(a, b)| a == b)
            .count();
    }
    (sa, lcp)
}
#[test]
fn exhaustive_direct_prefix_oracle() {
    for n in 0..=8 {
        for text in words(n) {
            assert_eq!(lcp_array(&text), oracle(&text), "{text:?}");
        }
    }
}
#[test]
fn boundaries_and_carried_prefix() {
    assert_eq!(lcp_array(b""), (vec![], vec![]));
    assert_eq!(lcp_array(b"x"), (vec![0], vec![0]));
    assert_eq!(
        lcp_array(b"banana"),
        (vec![5, 3, 1, 0, 4, 2], vec![0, 1, 3, 0, 0, 2])
    );
    assert_eq!(lcp_array(&vec![255; 1025]).1, (0..1025).collect::<Vec<_>>());
    let all: Vec<u8> = (0..=255).collect();
    assert_eq!(lcp_array(&all), oracle(&all));
}
#[test]
fn interval_minimum_recovers_nonadjacent_lcp() {
    for t in words(6) {
        let (sa, lcp) = lcp_array(&t);
        for a in 0..sa.len() {
            for b in a + 1..sa.len() {
                let common = t[sa[a]..]
                    .iter()
                    .zip(&t[sa[b]..])
                    .take_while(|(x, y)| x == y)
                    .count();
                assert_eq!(*lcp[a + 1..=b].iter().min().unwrap(), common);
            }
        }
    }
}
#[test]
fn inverse_fiber_and_relabel() {
    let mut count = 0;
    for t in words(2) {
        let compatible = lcp_array(&t) == (vec![1, 0], vec![0, 1]);
        assert_eq!(compatible, t[0] == t[1]);
        count += usize::from(compatible);
    }
    assert_eq!(count, 3);
    for t in words(5) {
        let mapped: Vec<_> = t.iter().map(|b| 10 + b * 7).collect();
        assert_eq!(lcp_array(&t), lcp_array(&mapped));
    }
}
#[test]
fn distinct_substrings_from_lcp() {
    for t in words(5) {
        let mut substrings = std::collections::BTreeSet::new();
        for a in 0..t.len() {
            for b in a + 1..=t.len() {
                substrings.insert(&t[a..b]);
            }
        }
        let (_, lcp) = lcp_array(&t);
        assert_eq!(
            substrings.len(),
            t.len() * (t.len() + 1) / 2 - lcp.iter().sum::<usize>()
        );
    }
}
