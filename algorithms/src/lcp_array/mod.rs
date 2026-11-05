//! Adjacent suffix common-prefix lengths by Kasai's carried-prefix method.
//!
//! Original implementation of the method of T. Kasai et al., "Linear-Time
//! Longest-Common-Prefix Computation in Suffix Arrays and Its Applications",
//! CPM 2001, LNCS 2089, pp. 181–192.

/// Return (suffix order, adjacent LCP lengths) for arbitrary unsigned bytes.
///
/// Both vectors have text.len() entries; lcp[0] is zero when nonempty, and
/// lcp[r] compares suffixes sa[r-1] and sa[r]. No empty suffix is included.
/// Existing comparison-sort suffix construction costs O(n log² n); the Kasai
/// stage costs O(n) time and space. Allocation failure may panic/abort.
pub fn lcp_array(text: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let order = crate::suffix_array::suffix_array(text);
    let n = text.len();
    let mut rank = vec![0; n];
    for (r, &start) in order.iter().enumerate() {
        rank[start] = r;
    }
    let mut lcp = vec![0; n];
    let mut common = 0;
    for start in 0..n {
        let r = rank[start];
        if r == 0 {
            common = 0;
            continue;
        }
        let previous = order[r - 1];
        while common < n - start
            && common < n - previous
            && text[start + common] == text[previous + common]
        {
            common += 1;
        }
        lcp[r] = common;
        common = common.saturating_sub(1);
    }
    (order, lcp)
}

#[cfg(test)]
mod tests;
