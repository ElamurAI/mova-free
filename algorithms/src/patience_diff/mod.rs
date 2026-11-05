//! Patience diff: Bram Cohen, "Patience Diff Advantages" (2007).
//! Original byte specialization: unique anchors, increasing subsequence,
//! recursive gaps, delete/insert fallback. Correct patches, not minimal scripts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edit {
    Equal(u8),
    Delete(u8),
    Insert(u8),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InputLimit,
    InvalidPatch,
    OutputLimit,
}
fn segment(a: &[u8], b: &[u8], out: &mut Vec<Edit>) {
    let prefix = a.iter().zip(b).take_while(|(x, y)| x == y).count();
    out.extend(a[..prefix].iter().map(|&x| Edit::Equal(x)));
    let a = &a[prefix..];
    let b = &b[prefix..];
    let suffix = a
        .iter()
        .rev()
        .zip(b.iter().rev())
        .take_while(|(x, y)| x == y)
        .count();
    let aa = &a[..a.len() - suffix];
    let bb = &b[..b.len() - suffix];
    let mut ca = [0usize; 256];
    let mut cb = [0usize; 256];
    let mut pb = [0usize; 256];
    for &x in aa {
        ca[x as usize] += 1;
    }
    for (i, &x) in bb.iter().enumerate() {
        cb[x as usize] += 1;
        pb[x as usize] = i;
    }
    let pairs: Vec<(usize, usize)> = aa
        .iter()
        .enumerate()
        .filter(|&(_, x)| ca[*x as usize] == 1 && cb[*x as usize] == 1)
        .map(|(i, &x)| (i, pb[x as usize]))
        .collect();
    let mut lengths = vec![1; pairs.len()];
    let mut prev = vec![None; pairs.len()];
    for i in 0..pairs.len() {
        for j in 0..i {
            if pairs[j].1 < pairs[i].1 && lengths[j] + 1 > lengths[i] {
                lengths[i] = lengths[j] + 1;
                prev[i] = Some(j);
            }
        }
    }
    let mut anchors = Vec::new();
    if !pairs.is_empty() {
        let mut best = 0;
        for i in 1..pairs.len() {
            if lengths[i] > lengths[best] {
                best = i;
            }
        }
        let mut p = Some(best);
        while let Some(i) = p {
            anchors.push(pairs[i]);
            p = prev[i];
        }
        anchors.reverse();
    }
    if anchors.is_empty() {
        out.extend(aa.iter().map(|&x| Edit::Delete(x)));
        out.extend(bb.iter().map(|&x| Edit::Insert(x)));
    } else {
        let (mut ai, mut bi) = (0, 0);
        for (i, j) in anchors {
            segment(&aa[ai..i], &bb[bi..j], out);
            out.push(Edit::Equal(aa[i]));
            ai = i + 1;
            bi = j + 1;
        }
        segment(&aa[ai..], &bb[bi..], out);
    }
    out.extend(a[a.len() - suffix..].iter().map(|&x| Edit::Equal(x)));
}
/// Each input <=4096. Prefix/suffix stripped first; LIS ties keep earliest
/// predecessor/endpoint. O(n^3) conservative time, O(n^2) stack/storage bound.
/// Allocation may panic/abort; no callback and no edit-minimality guarantee.
pub fn diff(old: &[u8], new: &[u8]) -> Result<Vec<Edit>, Error> {
    if old.len() > 4096 || new.len() > 4096 {
        return Err(Error::InputLimit);
    }
    let mut out = Vec::new();
    segment(old, new, &mut out);
    Ok(out)
}
/// Validate full source consumption and byte identity before output budget.
/// Edits <=8192, source <=4096; result <=min(max_output,65536).
pub fn apply(old: &[u8], edits: &[Edit], max_output: usize) -> Result<Vec<u8>, Error> {
    if old.len() > 4096 || edits.len() > 8192 {
        return Err(Error::InputLimit);
    }
    let mut i = 0;
    let mut size = 0;
    for &e in edits {
        match e {
            Edit::Equal(x) | Edit::Delete(x) => {
                if old.get(i) != Some(&x) {
                    return Err(Error::InvalidPatch);
                }
                i += 1;
            }
            Edit::Insert(_) => {}
        }
        if !matches!(e, Edit::Delete(_)) {
            size += 1;
        }
    }
    if i != old.len() {
        return Err(Error::InvalidPatch);
    }
    if size > max_output.min(65536) {
        return Err(Error::OutputLimit);
    }
    Ok(edits
        .iter()
        .filter_map(|&e| match e {
            Edit::Equal(x) | Edit::Insert(x) => Some(x),
            Edit::Delete(_) => None,
        })
        .collect())
}
#[cfg(test)]
mod tests;
