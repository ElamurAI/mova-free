//! Deterministic substring index over bytes.
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
struct State {
    longest: usize,
    link: Option<usize>,
    edges: BTreeMap<u8, usize>,
}
/// All reachable states accept, including the empty path.
#[derive(Clone, Debug)]
pub struct SuffixAutomaton {
    states: Vec<State>,
}
impl SuffixAutomaton {
    /// Indexes every contiguous byte substring.
    pub fn new(input: &[u8]) -> Self {
        let mut states = vec![State {
            longest: 0,
            link: None,
            edges: BTreeMap::new(),
        }];
        let mut last = 0;
        for &byte in input {
            let current = states.len();
            states.push(State {
                longest: states[last].longest + 1,
                link: Some(0),
                edges: BTreeMap::new(),
            });
            let mut cursor = Some(last);
            while let Some(p) = cursor {
                if states[p].edges.contains_key(&byte) {
                    break;
                }
                states[p].edges.insert(byte, current);
                cursor = states[p].link;
            }
            if let Some(p) = cursor {
                let q = states[p].edges[&byte];
                if states[p].longest + 1 == states[q].longest {
                    states[current].link = Some(q);
                } else {
                    let clone = states.len();
                    let mut copy = states[q].clone();
                    copy.longest = states[p].longest + 1;
                    states.push(copy);
                    while let Some(p) = cursor {
                        if states[p].edges.get(&byte) != Some(&q) {
                            break;
                        }
                        states[p].edges.insert(byte, clone);
                        cursor = states[p].link;
                    }
                    states[q].link = Some(clone);
                    states[current].link = Some(clone);
                }
            }
            last = current;
        }
        Self { states }
    }
    /// Empty queries are always contained.
    pub fn contains(&self, query: &[u8]) -> bool {
        let mut state = 0;
        for byte in query {
            let Some(&next) = self.states[state].edges.get(byte) else {
                return false;
            };
            state = next;
        }
        true
    }
    /// Counts distinct nonempty substrings.
    pub fn distinct_substrings(&self) -> u128 {
        self.states
            .iter()
            .skip(1)
            .map(|s| (s.longest - self.states[s.link.expect("nonroot link")].longest) as u128)
            .sum()
    }
}
#[cfg(test)]
mod tests;
