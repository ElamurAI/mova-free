//! Ziv and Lempel, "A Universal Algorithm for Sequential Data Compression",
//! IEEE Transactions on Information Theory 23(3), 1977. Original bounded byte codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Literal(u8),
    Match { distance: usize, length: usize },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InputLimit,
    InvalidConfiguration,
    InvalidMatch,
    OutputLimit,
}
/// Greedy longest match; equal lengths prefer smallest distance. Overlap permitted.
/// n<=65536, window and max_match in 1..=4096. Matches of length >=3 only.
/// O(n*window*max_match), O(n) output; allocations may panic/abort.
pub fn encode(input: &[u8], window: usize, max_match: usize) -> Result<Vec<Token>, Error> {
    if input.len() > 65536 {
        return Err(Error::InputLimit);
    }
    if window == 0 || window > 4096 || max_match == 0 || max_match > 4096 {
        return Err(Error::InvalidConfiguration);
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < input.len() {
        let mut best = (0, 0);
        for d in 1..=i.min(window) {
            let mut len = 0;
            while len < max_match && i + len < input.len() && input[i + len] == input[i + len - d] {
                len += 1;
            }
            if len > best.1 {
                best = (d, len);
            }
        }
        if best.1 >= 3 {
            out.push(Token::Match {
                distance: best.0,
                length: best.1,
            });
            i += best.1;
        } else {
            out.push(Token::Literal(input[i]));
            i += 1;
        }
    }
    Ok(out)
}
/// Tokens cap first; validate each match before its budget check. Decodes overlap
/// one byte at a time. Arbitrary positive lengths accepted subject to output cap.
pub fn decode(tokens: &[Token], max_output: usize) -> Result<Vec<u8>, Error> {
    if tokens.len() > 65536 {
        return Err(Error::InputLimit);
    }
    let mut out = Vec::new();
    for &token in tokens {
        let length = match token {
            Token::Literal(_) => 1,
            Token::Match { distance, length } => {
                if distance == 0 || distance > out.len() || length == 0 {
                    return Err(Error::InvalidMatch);
                }
                length
            }
        };
        if length > max_output.min(65536) - out.len() {
            return Err(Error::OutputLimit);
        }
        match token {
            Token::Literal(b) => out.push(b),
            Token::Match { distance, length } => {
                for _ in 0..length {
                    let b = out[out.len() - distance];
                    out.push(b);
                }
            }
        }
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
