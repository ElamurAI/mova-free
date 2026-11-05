//! Myers, "A Fast Bit-Vector Algorithm for Approximate String Matching", JACM 46 (1999).
//! Original single-word global Levenshtein distance (not substring matching).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    PatternTooLong,
    TextTooLong,
}
/// Pattern cap is checked first; empty pattern has distance text.len().
/// O(pattern.len()+text.len()+256), constant storage, no allocation.
pub fn distance(pattern: &[u8], text: &[u8]) -> Result<usize, Error> {
    if pattern.len() > 64 {
        return Err(Error::PatternTooLong);
    }
    if text.len() > 65536 {
        return Err(Error::TextTooLong);
    }
    if pattern.is_empty() {
        return Ok(text.len());
    }
    let mut eq = [0u64; 256];
    for (i, &b) in pattern.iter().enumerate() {
        eq[b as usize] |= 1u64 << i;
    }
    let mut pv = !0u64;
    let mut mv = 0u64;
    let mut score = pattern.len();
    let high = 1u64 << (pattern.len() - 1);
    for &b in text {
        let e = eq[b as usize];
        let xv = e | mv;
        let xh = ((e & pv).wrapping_add(pv) ^ pv) | e;
        let mut ph = mv | !(xh | pv);
        let mut mh = pv & xh;
        if ph & high != 0 {
            score += 1;
        }
        if mh & high != 0 {
            score -= 1;
        }
        ph = (ph << 1) | 1;
        mh <<= 1;
        pv = mh | !(xv | ph);
        mv = ph & xv;
    }
    Ok(score)
}
#[cfg(test)]
mod tests;
