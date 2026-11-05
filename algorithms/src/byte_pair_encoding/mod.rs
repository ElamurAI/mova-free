//! Gage, "A New Algorithm for Data Compression", C Users Journal 12(2), 1994.
//! Original deterministic byte-pair training and model-bound tokenization.
use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InputLimit,
    MergeLimit,
    InvalidToken,
    OutputLimit,
}
/// Token IDs 0..255 are bytes; 256+i expands merge i. Tokens must travel with
/// this exact ordered model: decoding against another model is not authenticated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    merges: Vec<(u16, u16)>,
}
fn replace(input: &[u16], pair: (u16, u16), id: u16) -> Vec<u16> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < input.len() {
        if i + 1 < input.len() && (input[i], input[i + 1]) == pair {
            out.push(id);
            i += 2;
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    out
}
impl Model {
    /// Count overlapping pairs; highest frequency >=2 wins, lexicographically
    /// smallest pair breaks ties. Replacement is nonoverlapping left-to-right.
    /// <=65536 bytes, <=3840 merges. O(merges*n*log n), O(n+merges) storage.
    pub fn train(input: &[u8], merges: usize) -> Result<Self, Error> {
        if input.len() > 65536 {
            return Err(Error::InputLimit);
        }
        if merges > 3840 {
            return Err(Error::MergeLimit);
        }
        let mut model = Self { merges: Vec::new() };
        let mut seq: Vec<u16> = input.iter().map(|&b| b as u16).collect();
        for _ in 0..merges {
            let mut counts = BTreeMap::new();
            for w in seq.windows(2) {
                *counts.entry((w[0], w[1])).or_insert(0usize) += 1;
            }
            let mut best = None;
            let mut count = 1;
            for (pair, c) in counts {
                if c > count {
                    best = Some(pair);
                    count = c;
                }
            }
            let Some(pair) = best else { break };
            seq = replace(&seq, pair, 256 + model.merges.len() as u16);
            model.merges.push(pair);
        }
        Ok(model)
    }
    pub fn merges(&self) -> &[(u16, u16)] {
        &self.merges
    }
    /// Apply exactly the learned ordered substitutions; no retraining.
    pub fn encode(&self, input: &[u8]) -> Result<Vec<u16>, Error> {
        if input.len() > 65536 {
            return Err(Error::InputLimit);
        }
        let mut seq: Vec<u16> = input.iter().map(|&b| b as u16).collect();
        for (i, &pair) in self.merges.iter().enumerate() {
            seq = replace(&seq, pair, 256 + i as u16);
        }
        Ok(seq)
    }
    /// Validate all token IDs, then expanded size, then expand iteratively.
    /// O(model size + output + tokens), O(model size + output) memory.
    /// Output bounded by min(max_output,65536); allocation may panic/abort.
    pub fn decode(&self, tokens: &[u16], max_output: usize) -> Result<Vec<u8>, Error> {
        if tokens.len() > 65536 {
            return Err(Error::InputLimit);
        }
        if tokens
            .iter()
            .any(|&t| t as usize >= 256 + self.merges.len())
        {
            return Err(Error::InvalidToken);
        }
        let cap = max_output.min(65536);
        let mut sizes = vec![1usize; 256];
        for &(a, b) in &self.merges {
            sizes.push((sizes[a as usize] + sizes[b as usize]).min(65537));
        }
        let mut total = 0;
        for &t in tokens {
            if sizes[t as usize] > cap - total {
                return Err(Error::OutputLimit);
            }
            total += sizes[t as usize];
        }
        let mut out = Vec::with_capacity(total);
        let mut stack = Vec::new();
        for &t in tokens {
            stack.push(t);
            while let Some(x) = stack.pop() {
                if x < 256 {
                    out.push(x as u8);
                } else {
                    let (a, b) = self.merges[x as usize - 256];
                    stack.push(b);
                    stack.push(a);
                }
            }
        }
        Ok(out)
    }
}
#[cfg(test)]
mod tests;
