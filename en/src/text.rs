//! The boundary with text: stitching words into a string and text comparison metrics.

use crate::hash::FastMap;

/// Spaces in the generated order: a pair that stood adjacent in the original — the "space after" bit;
/// otherwise rules (no space before , . ! ? ; : ) clitics and % and after ( [ $).
pub fn detok(forms: &[String], order: &[usize], space_after: &[bool]) -> String {
    let mut s = String::new();
    for (k, &i) in order.iter().enumerate() {
        if k > 0 {
            let prev = order[k - 1];
            let f = forms[i - 1].as_str();
            let pf = forms[prev - 1].as_str();
            let space = if prev + 1 == i {
                space_after[prev - 1]
            } else {
                !(matches!(f, "," | "." | "!" | "?" | ";" | ":" | ")" | "]" | "}" | "%" | "..." | "n't" | "'s" | "'re" | "'ve" | "'ll" | "'d" | "'m")
                    || matches!(pf, "(" | "[" | "{" | "$" | "#"))
            };
            if space {
                s.push(' ');
            }
        }
        s.push_str(&forms[i - 1]);
    }
    s
}

/// chrF (Popović 2015): character n-grams 1..6, β = 2; strings without spaces.
pub fn chrf(hyp: &str, reference: &str) -> f64 {
    let h: Vec<char> = hyp.chars().filter(|c| !c.is_whitespace()).collect();
    let r: Vec<char> = reference.chars().filter(|c| !c.is_whitespace()).collect();
    let (mut p_sum, mut r_sum, mut k) = (0.0, 0.0, 0.0);
    for n in 1..=6usize {
        if h.len() < n || r.len() < n {
            continue;
        }
        let mut cnt: FastMap<&[char], i32> = FastMap::default();
        for g in r.windows(n) {
            *cnt.entry(g).or_default() += 1;
        }
        let mut hit = 0;
        for g in h.windows(n) {
            if let Some(c) = cnt.get_mut(g) {
                if *c > 0 {
                    *c -= 1;
                    hit += 1;
                }
            }
        }
        p_sum += hit as f64 / (h.len() - n + 1) as f64;
        r_sum += hit as f64 / (r.len() - n + 1) as f64;
        k += 1.0;
    }
    if k == 0.0 {
        return 0.0;
    }
    let (p, r) = (p_sum / k, r_sum / k);
    if p + r == 0.0 { 0.0 } else { 5.0 * p * r / (4.0 * p + r) }
}

/// Length of the longest common subsequence.
/// Two rolling rows are reused and swapped after each left item (two allocations for
/// nonempty inputs, none for an empty side); the comparison order `x == y` over `a` then `b`
/// is the classical row recurrence and is unchanged. Elements are compared as given:
/// callers choose tokens or characters, never raw UTF-8 bytes on their behalf.
pub fn lcs<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let mut prev = vec![0usize; b.len() + 1];
    let mut cur = vec![0usize; b.len() + 1];
    for x in a {
        cur[0] = 0;
        for (j, y) in b.iter().enumerate() {
            cur[j + 1] = if x == y { prev[j] + 1 } else { prev[j + 1].max(cur[j]) };
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

/// Share of adjacent output pairs that are adjacent in the same order in the original (a BLEU-2 substitute).
pub fn bigram_precision(out: &[usize]) -> (usize, usize) {
    let hit = out.windows(2).filter(|w| w[1] == w[0] + 1).count();
    (hit, out.len().saturating_sub(1))
}

/// First letter uppercase.
pub fn title(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod wave27_tests {
    use super::*;
    #[test]
    fn metrics_and_adjacency_priority() {
        assert_eq!(chrf("", ""), 0.0);
        assert_eq!(chrf("a b", "ab"), 1.0);
        assert_eq!(chrf("aaa", "bbb"), 0.0);
        assert_eq!(lcs(&[1, 2, 3], &[2, 1, 3]), 2);
        assert_eq!(lcs::<u8>(&[], &[1]), 0);
        assert_eq!(bigram_precision(&[2, 3, 1]), (1, 2));
        assert_eq!(bigram_precision(&[]), (0, 0));
        let f = vec!["a".into(), ",".into(), "b".into()];
        assert_eq!(detok(&f, &[1, 2], &[true, false, false]), "a ,");
        assert_eq!(detok(&f, &[3, 2], &[true, false, false]), "b,");
        assert_eq!(title("ßa"), "SSa");
    }

    /// Empty shapes, the swapped-row recurrence against a brute-force subsequence oracle,
    /// and the token/character distinction: one multibyte character is one element.
    #[test]
    fn lcs_rows_empty_shapes_and_oracle() {
        assert_eq!(lcs::<u8>(&[], &[]), 0);
        assert_eq!(lcs(&[1], &[]), 0);
        assert_eq!(lcs(&[7], &[7]), 1);
        assert_eq!(lcs(&[1, 2], &[2, 1]), 1);
        fn oracle(a: &[u8], b: &[u8]) -> usize {
            (0..1u32 << a.len())
                .filter(|m| {
                    let mut k = 0;
                    for (i, x) in a.iter().enumerate() {
                        if m >> i & 1 == 1 {
                            match b[k..].iter().position(|y| y == x) {
                                Some(p) => k += p + 1,
                                None => return false,
                            }
                        }
                    }
                    true
                })
                .map(|m| m.count_ones() as usize)
                .max()
                .unwrap()
        }
        let mut s = 0x5eed_u32;
        for _ in 0..400 {
            s = s.wrapping_mul(1103515245).wrapping_add(12345);
            let la = (s >> 8) as usize % 7;
            let lb = (s >> 12) as usize % 7;
            let a: Vec<u8> = (0..la).map(|i| ((s >> (i * 2)) & 3) as u8).collect();
            let b: Vec<u8> = (0..lb).map(|i| ((s >> (i * 3 + 1)) & 3) as u8).collect();
            assert_eq!(lcs(&a, &b), oracle(&a, &b), "{a:?} {b:?}");
        }
        let (x, y): (Vec<char>, Vec<char>) = ("éa".chars().collect(), "aé".chars().collect());
        assert_eq!(lcs(&x, &y), 1);
        assert_eq!(lcs("éa".as_bytes(), "aé".as_bytes()), 2, "raw bytes are not characters");
    }
}
