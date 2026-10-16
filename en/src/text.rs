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
pub fn lcs<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let mut prev = vec![0usize; b.len() + 1];
    for x in a {
        let mut cur = vec![0usize; b.len() + 1];
        for (j, y) in b.iter().enumerate() {
            cur[j + 1] = if x == y { prev[j] + 1 } else { prev[j + 1].max(cur[j]) };
        }
        prev = cur;
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
