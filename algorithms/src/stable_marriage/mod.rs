//! Proposer-optimal stable marriage using deferred acceptance.
//! Attribution: D. Gale and L. S. Shapley, "College Admissions and the
//! Stability of Marriage", American Mathematical Monthly 69(1), 1962, 9–15.
//! Original implementation and fixtures; complete strict preferences only.
use std::collections::VecDeque;

/// Total entries across both n-by-n preference matrices.
pub const MAX_PREFERENCE_ENTRIES: usize = 2_000_000;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Proposer,
    Receiver,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    SizeMismatch,
    LimitExceeded,
    InvalidRowLength {
        side: Side,
        row: usize,
    },
    InvalidPreference {
        side: Side,
        row: usize,
        position: usize,
        value: usize,
    },
}

/// Return receiver index per proposer; each row lists best to worst.
/// Validation: equal population sizes, checked 2*n*n budget, then proposer
/// rows followed by receiver rows; row length precedes its first invalid or
/// duplicate entry. Empty input returns empty. FIFO free proposers and strict
/// ranks make the witness deterministic and proposer-optimal.
/// O(n²) time/auxiliary space. Allocation failure is outside typed errors.
pub fn stable_matching(
    proposers: &[Vec<usize>],
    receivers: &[Vec<usize>],
) -> Result<Vec<usize>, Error> {
    let n = proposers.len();
    if receivers.len() != n {
        return Err(Error::SizeMismatch);
    }
    if n.checked_mul(n)
        .and_then(|v| v.checked_mul(2))
        .is_none_or(|v| v > MAX_PREFERENCE_ENTRIES)
    {
        return Err(Error::LimitExceeded);
    }
    for (side, rows) in [(Side::Proposer, proposers), (Side::Receiver, receivers)] {
        for (row, values) in rows.iter().enumerate() {
            if values.len() != n {
                return Err(Error::InvalidRowLength { side, row });
            }
            let mut seen = vec![false; n];
            for (position, &value) in values.iter().enumerate() {
                if value >= n || seen[value] {
                    return Err(Error::InvalidPreference {
                        side,
                        row,
                        position,
                        value,
                    });
                }
                seen[value] = true;
            }
        }
    }
    let mut rank = vec![vec![0; n]; n];
    for (receiver, order) in receivers.iter().enumerate() {
        for (r, &proposer) in order.iter().enumerate() {
            rank[receiver][proposer] = r;
        }
    }
    let mut free: VecDeque<_> = (0..n).collect();
    let mut next = vec![0; n];
    let mut held: Vec<Option<usize>> = vec![None; n];
    let mut answer = vec![0; n];
    while let Some(proposer) = free.pop_front() {
        let receiver = proposers[proposer][next[proposer]];
        next[proposer] += 1;
        if let Some(old) = held[receiver] {
            if rank[receiver][proposer] < rank[receiver][old] {
                free.push_back(old);
            } else {
                free.push_back(proposer);
                continue;
            }
        }
        held[receiver] = Some(proposer);
        answer[proposer] = receiver;
    }
    Ok(answer)
}

#[cfg(test)]
mod tests;
