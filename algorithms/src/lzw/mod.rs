//! Welch, "A Technique for High-Performance Data Compression", Computer 17(6), 1984.
//! Original byte LZW, 12-bit dictionary frozen at 4096; no clear/end codes or packing.
use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InputLimit,
    InvalidCode,
    OutputLimit,
}
/// At most 65536 input bytes; dictionary and output allocations may panic/abort.
/// O(n*phrase_length*log(4096)) time, bounded dictionary phrase storage.
pub fn encode(input: &[u8]) -> Result<Vec<u16>, Error> {
    if input.len() > 65536 {
        return Err(Error::InputLimit);
    }
    let mut dict: BTreeMap<Vec<u8>, u16> = (0..256).map(|i| (vec![i as u8], i)).collect();
    let mut out = Vec::new();
    let mut w = Vec::new();
    for &b in input {
        let mut wb = w.clone();
        wb.push(b);
        if dict.contains_key(&wb) {
            w = wb;
        } else {
            out.push(dict[&w]);
            if dict.len() < 4096 {
                dict.insert(wb, dict.len() as u16);
            }
            w = vec![b];
        }
    }
    if !w.is_empty() {
        out.push(dict[&w]);
    }
    Ok(out)
}
/// Input cap first; per code validate before budget, then dictionary update.
/// Accepts valid noncanonical streams. Output never exceeds min(max_output,65536).
pub fn decode(codes: &[u16], max_output: usize) -> Result<Vec<u8>, Error> {
    if codes.len() > 65536 {
        return Err(Error::InputLimit);
    }
    let mut dict: Vec<Vec<u8>> = (0..256).map(|i| vec![i as u8]).collect();
    let mut previous: Vec<u8> = Vec::new();
    let mut out = Vec::new();
    for &code in codes {
        let entry = if (code as usize) < dict.len() {
            dict[code as usize].clone()
        } else if code as usize == dict.len() && !previous.is_empty() && dict.len() < 4096 {
            let mut s = previous.clone();
            s.push(previous[0]);
            s
        } else {
            return Err(Error::InvalidCode);
        };
        if entry.len() > max_output.min(65536) - out.len() {
            return Err(Error::OutputLimit);
        }
        out.extend_from_slice(&entry);
        if !previous.is_empty() && dict.len() < 4096 {
            let mut next = previous;
            next.push(entry[0]);
            dict.push(next);
        }
        previous = entry;
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
