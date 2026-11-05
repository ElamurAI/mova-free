//! Burkhard-Keller metric tree (Burkhard and Keller, CACM 1973).
use std::collections::BTreeMap;
#[derive(Debug)]
struct Node {
    word: Vec<u8>,
    indexes: Vec<usize>,
    children: BTreeMap<usize, usize>,
}
/// Immutable byte Levenshtein index: <=4096 words, <=256 bytes each.
/// Allocations follow standard library behavior; no callbacks.
#[derive(Debug, Default)]
pub struct BkTree {
    nodes: Vec<Node>,
}
impl BkTree {
    /// Count errors precede word-length errors; duplicates retain original indexes.
    /// Worst O(n² L²) construction for maximum word length L.
    pub fn new(words: &[Vec<u8>]) -> Result<Self, &'static str> {
        if words.len() > 4096 {
            return Err("too many words");
        }
        if words.iter().any(|w| w.len() > 256) {
            return Err("word too long");
        }
        let mut tree = Self::default();
        for (index, word) in words.iter().enumerate() {
            if tree.nodes.is_empty() {
                tree.nodes.push(Node {
                    word: word.clone(),
                    indexes: vec![index],
                    children: BTreeMap::new(),
                });
                continue;
            }
            let mut at = 0;
            loop {
                let d = crate::levenshtein::distance(word, &tree.nodes[at].word);
                if d == 0 {
                    tree.nodes[at].indexes.push(index);
                    break;
                }
                if let Some(&child) = tree.nodes[at].children.get(&d) {
                    at = child;
                } else {
                    let next = tree.nodes.len();
                    tree.nodes.push(Node {
                        word: word.clone(),
                        indexes: vec![index],
                        children: BTreeMap::new(),
                    });
                    tree.nodes[at].children.insert(d, next);
                    break;
                }
            }
        }
        Ok(tree)
    }
    /// (Original index, distance) sorted by index. Query <=256 bytes.
    /// Radius is any usize; empty returns empty. O(n L²+k log k) worst time,
    /// O(n+k+L) scratch. No guaranteed sublinear pruning.
    pub fn within(&self, query: &[u8], radius: usize) -> Result<Vec<(usize, usize)>, &'static str> {
        if query.len() > 256 {
            return Err("word too long");
        }
        let mut out = Vec::new();
        let mut pending = Vec::new();
        if !self.nodes.is_empty() {
            pending.push(0);
        }
        while let Some(at) = pending.pop() {
            let node = &self.nodes[at];
            let d = crate::levenshtein::distance(query, &node.word);
            if d <= radius {
                out.extend(node.indexes.iter().map(|&i| (i, d)));
            }
            pending.extend(
                node.children
                    .range(d.saturating_sub(radius)..=d.saturating_add(radius))
                    .map(|(_, &i)| i),
            );
        }
        out.sort_unstable_by_key(|&(i, _)| i);
        Ok(out)
    }
}
#[cfg(test)]
mod tests;
