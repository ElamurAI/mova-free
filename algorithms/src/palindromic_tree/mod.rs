//! Eertree representation of distinct byte palindromes.
//! Rubinchik and Shur, "EERTREE: An Efficient Data Structure for Processing
//! Palindromes in Strings" (2015). Original implementation with sparse edges.
use std::collections::BTreeMap;
use std::ops::Range;

pub const MAX_LEN: usize = 1_000_000;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    LengthLimit,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Palindrome {
    /// First occurrence of this distinct palindrome.
    pub range: Range<usize>,
    /// Includes overlapping occurrences.
    pub occurrences: usize,
    /// Index of longest proper nonempty palindromic suffix, if any.
    pub suffix_link: Option<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PalindromicTree {
    palindromes: Vec<Palindrome>,
    longest: Option<Range<usize>>,
}
impl PalindromicTree {
    pub fn palindromes(&self) -> &[Palindrome] {
        &self.palindromes
    }
    pub fn distinct_count(&self) -> usize {
        self.palindromes.len()
    }
    /// Longest palindrome; ties choose earliest starting position.
    pub fn longest(&self) -> Option<Range<usize>> {
        self.longest.clone()
    }
}
struct Node {
    length: isize,
    link: usize,
    edges: BTreeMap<u8, usize>,
    end: usize,
    count: usize,
}
fn extendable(nodes: &[Node], bytes: &[u8], position: usize, mut node: usize) -> usize {
    loop {
        let left = position as isize - 1 - nodes[node].length;
        if left >= 0 && bytes[left as usize] == bytes[position] {
            return node;
        }
        node = nodes[node].link;
    }
}
/// Builds all distinct nonempty palindromes in first-discovery order.
/// O(n log 256) time and O(n) space with sparse ordered transitions. Each
/// individual extension may traverse many suffix links; total traversal is
/// amortized linear. The length cap precedes allocation; allocation follows Vec.
pub fn build(bytes: &[u8]) -> Result<PalindromicTree, Error> {
    if bytes.len() > MAX_LEN {
        return Err(Error::LengthLimit);
    }
    let mut nodes = vec![
        Node {
            length: -1,
            link: 0,
            edges: BTreeMap::new(),
            end: 0,
            count: 0,
        },
        Node {
            length: 0,
            link: 0,
            edges: BTreeMap::new(),
            end: 0,
            count: 0,
        },
    ];
    let mut suffix = 1;
    let mut longest = None;
    for position in 0..bytes.len() {
        let parent = extendable(&nodes, bytes, position, suffix);
        suffix = if let Some(&existing) = nodes[parent].edges.get(&bytes[position]) {
            existing
        } else {
            let length = nodes[parent].length + 2;
            let link = if length == 1 {
                1
            } else {
                let candidate = extendable(&nodes, bytes, position, nodes[parent].link);
                nodes[candidate].edges[&bytes[position]]
            };
            let index = nodes.len();
            nodes.push(Node {
                length,
                link,
                edges: BTreeMap::new(),
                end: position + 1,
                count: 0,
            });
            nodes[parent].edges.insert(bytes[position], index);
            index
        };
        nodes[suffix].count += 1;
        let length = nodes[suffix].length as usize;
        if longest
            .as_ref()
            .is_none_or(|r: &Range<usize>| length > r.len())
        {
            longest = Some(position + 1 - length..position + 1);
        }
    }
    // Proper suffix nodes predate their children; reverse discovery order
    // therefore propagates every terminal-suffix occurrence exactly once.
    for index in (2..nodes.len()).rev() {
        let link = nodes[index].link;
        nodes[link].count += nodes[index].count;
    }
    let palindromes = nodes
        .iter()
        .skip(2)
        .map(|node| Palindrome {
            range: node.end - node.length as usize..node.end,
            occurrences: node.count,
            suffix_link: node.link.checked_sub(2),
        })
        .collect();
    Ok(PalindromicTree {
        palindromes,
        longest,
    })
}
#[cfg(test)]
mod tests;
