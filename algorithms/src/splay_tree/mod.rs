//! Iterative bottom-up splay tree; Sleator and Tarjan, JACM 1985.
#[derive(Debug)]
struct Node {
    key: i64,
    parent: Option<usize>,
    child: [Option<usize>; 2],
}
/// Unique i64 keys, <=4096 entries. Allocations follow std behavior; no callbacks.
/// Individual operations can be linear. Standard potential accounting gives
/// amortized O(log n) access/insertion with the initial-potential term included.
/// The observation methods root/keys do not splay.
#[derive(Debug, Default)]
pub struct SplayTree {
    root: Option<usize>,
    nodes: Vec<Node>,
}
impl SplayTree {
    /// Current root key; empty tree returns None.
    pub fn root(&self) -> Option<i64> {
        self.root.map(|i| self.nodes[i].key)
    }
    fn locate(&self, key: i64) -> (bool, Option<usize>) {
        let mut at = self.root;
        let mut last = None;
        while let Some(i) = at {
            last = Some(i);
            if self.nodes[i].key == key {
                return (true, last);
            }
            at = self.nodes[i].child[usize::from(key > self.nodes[i].key)];
        }
        (false, last)
    }
    /// Hit splays the key; miss splays the last visited key. Empty => false.
    pub fn access(&mut self, key: i64) -> bool {
        let (found, last) = self.locate(key);
        if let Some(i) = last {
            self.splay(i);
        }
        found
    }
    /// New insertion splays the new key; duplicate splays the existing key and
    /// returns Ok(false), even at capacity. New-key capacity error is state-neutral.
    pub fn insert(&mut self, key: i64) -> Result<bool, &'static str> {
        let (found, parent) = self.locate(key);
        if found {
            self.splay(parent.unwrap());
            return Ok(false);
        }
        if self.nodes.len() == 4096 {
            return Err("too many keys");
        }
        let x = self.nodes.len();
        self.nodes.push(Node {
            key,
            parent,
            child: [None, None],
        });
        if let Some(p) = parent {
            let s = usize::from(key > self.nodes[p].key);
            self.nodes[p].child[s] = Some(x);
        } else {
            self.root = Some(x);
        }
        self.splay(x);
        Ok(true)
    }
    fn splay(&mut self, x: usize) {
        while let Some(p) = self.nodes[x].parent {
            if let Some(g) = self.nodes[p].parent {
                let xp = self.nodes[p].child[1] == Some(x);
                let pg = self.nodes[g].child[1] == Some(p);
                if xp == pg {
                    self.rotate(p);
                } else {
                    self.rotate(x);
                }
            }
            self.rotate(x);
        }
    }
    fn rotate(&mut self, x: usize) {
        let p = self.nodes[x].parent.unwrap();
        let g = self.nodes[p].parent;
        let side = usize::from(self.nodes[p].child[1] == Some(x));
        let middle = self.nodes[x].child[1 - side];
        self.nodes[p].child[side] = middle;
        if let Some(m) = middle {
            self.nodes[m].parent = Some(p);
        }
        self.nodes[x].child[1 - side] = Some(p);
        self.nodes[p].parent = Some(x);
        self.nodes[x].parent = g;
        if let Some(g) = g {
            let s = usize::from(self.nodes[g].child[1] == Some(p));
            self.nodes[g].child[s] = Some(x);
        } else {
            self.root = Some(x);
        }
    }
    /// Sorted keys in O(n) time and O(height) traversal scratch, plus returned storage.
    pub fn keys(&self) -> Vec<i64> {
        let mut out = Vec::new();
        let mut stack = Vec::new();
        let mut at = self.root;
        loop {
            while let Some(i) = at {
                stack.push(i);
                at = self.nodes[i].child[0];
            }
            let Some(i) = stack.pop() else {
                break;
            };
            out.push(self.nodes[i].key);
            at = self.nodes[i].child[1];
        }
        out
    }
}
#[cfg(test)]
mod tests;
