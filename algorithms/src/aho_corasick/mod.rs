//! Multiple exact byte-pattern matching with overlapping matches.
//!
//! Original implementation of trie/failure-link ideas in A. V. Aho and
//! M. J. Corasick, "Efficient String Matching: An Aid to Bibliographic Search",
//! Communications of the ACM 18(6), 333–340 (1975), doi:10.1145/360825.360855.
//! Sparse edges and copied output lists deliberately trade compact transitions
//! for simple deterministic storage. Empty patterns match every byte boundary.
use std::collections::{BTreeMap, VecDeque};

/// One occurrence, with a half-open byte range and original pattern index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Match {
    pub pattern: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, Default)]
struct Node {
    edges: BTreeMap<u8, usize>,
    failure: usize,
    outputs: Vec<usize>,
}

/// An owned automaton. Duplicate patterns keep distinct original indices.
#[derive(Clone, Debug)]
pub struct AhoCorasick {
    nodes: Vec<Node>,
    lengths: Vec<usize>,
}

impl AhoCorasick {
    /// Compile arbitrary byte patterns, including duplicates and empty patterns.
    ///
    /// With M total pattern bytes, L maximum length and H total stored outputs,
    /// a conservative construction bound is O(pattern_count + M L log 256 + H +
    /// sum_s o_s log o_s), where o_s counts outputs at state s.
    /// Storage is O(M + H + pattern count). Allocation failure may panic/abort.
    pub fn new(patterns: &[&[u8]]) -> Self {
        let mut nodes = vec![Node::default()];
        let lengths = patterns.iter().map(|p| p.len()).collect();
        for (pattern, bytes) in patterns.iter().enumerate() {
            let mut state = 0;
            for byte in *bytes {
                state = if let Some(&next) = nodes[state].edges.get(byte) {
                    next
                } else {
                    let next = nodes.len();
                    nodes.push(Node::default());
                    nodes[state].edges.insert(*byte, next);
                    next
                };
            }
            nodes[state].outputs.push(pattern);
        }
        let mut queue = VecDeque::new();
        let roots: Vec<usize> = nodes[0].edges.values().copied().collect();
        for child in roots {
            let inherited = nodes[0].outputs.clone();
            nodes[child].outputs.extend(inherited);
            nodes[child].outputs.sort_unstable();
            queue.push_back(child);
        }
        while let Some(state) = queue.pop_front() {
            let edges: Vec<(u8, usize)> =
                nodes[state].edges.iter().map(|(&b, &s)| (b, s)).collect();
            for (byte, child) in edges {
                let mut fallback = nodes[state].failure;
                while fallback != 0 && !nodes[fallback].edges.contains_key(&byte) {
                    fallback = nodes[fallback].failure;
                }
                let failure = nodes[fallback].edges.get(&byte).copied().unwrap_or(0);
                nodes[child].failure = failure;
                let inherited = nodes[failure].outputs.clone();
                nodes[child].outputs.extend(inherited);
                nodes[child].outputs.sort_unstable();
                queue.push_back(child);
            }
        }
        Self { nodes, lengths }
    }

    /// Return all matches in increasing (end byte offset, pattern index) order.
    ///
    /// Overlaps are retained. Empty patterns match offsets 0 through text.len().
    /// For z reported matches, O(text.len() log 256 + z) time and O(z) output.
    pub fn find_all(&self, text: &[u8]) -> Vec<Match> {
        let mut result = Vec::new();
        let mut state = 0;
        for end in 0..=text.len() {
            if end != 0 {
                let byte = text[end - 1];
                while state != 0 && !self.nodes[state].edges.contains_key(&byte) {
                    state = self.nodes[state].failure;
                }
                state = self.nodes[state].edges.get(&byte).copied().unwrap_or(0);
            }
            for &pattern in &self.nodes[state].outputs {
                result.push(Match {
                    pattern,
                    start: end - self.lengths[pattern],
                    end,
                });
            }
        }
        result
    }
}

#[cfg(test)]
mod tests;
