//! Explicit-priority treap; Seidel and Aragon, Randomized Search Trees (1996).
#[derive(Debug)]
struct Node {
    key: i64,
    priority: u64,
    parent: Option<usize>,
    child: [Option<usize>; 2],
}
/// Unique keys in a BST and min heap ordered by (priority,key).
/// <=4096 live keys; freed arena slots are reused. Allocator failure follows std.
/// Arbitrary explicit priorities may yield linear height. Expected logarithmic
/// height requires independent random priorities and is not promised by this API.
/// No callbacks; insertion/search/removal cost O(height).
#[derive(Debug, Default)]
pub struct Treap {
    root: Option<usize>,
    nodes: Vec<Node>,
    free: Vec<usize>,
}
impl Treap {
    fn find(&self, key: i64) -> Option<usize> {
        let mut at = self.root;
        while let Some(i) = at {
            if self.nodes[i].key == key {
                return Some(i);
            }
            at = self.nodes[i].child[usize::from(key > self.nodes[i].key)];
        }
        None
    }
    /// Observational search; does not alter priorities or shape.
    pub fn contains(&self, key: i64) -> bool {
        self.find(key).is_some()
    }
    fn rank(&self, i: usize) -> (u64, i64) {
        (self.nodes[i].priority, self.nodes[i].key)
    }
    /// Duplicate returns Ok(false) without changing priority, even at capacity.
    /// A new key at capacity errors without changing state.
    pub fn insert(&mut self, key: i64, priority: u64) -> Result<bool, &'static str> {
        let mut parent = None;
        let mut at = self.root;
        while let Some(i) = at {
            if self.nodes[i].key == key {
                return Ok(false);
            }
            parent = Some(i);
            at = self.nodes[i].child[usize::from(key > self.nodes[i].key)];
        }
        if self.nodes.len() - self.free.len() == 4096 {
            return Err("too many keys");
        }
        let node = Node {
            key,
            priority,
            parent,
            child: [None, None],
        };
        let x = if let Some(i) = self.free.pop() {
            self.nodes[i] = node;
            i
        } else {
            self.nodes.push(node);
            self.nodes.len() - 1
        };
        if let Some(p) = parent {
            let s = usize::from(key > self.nodes[p].key);
            self.nodes[p].child[s] = Some(x);
        } else {
            self.root = Some(x);
        }
        while let Some(p) = self.nodes[x].parent {
            if self.rank(p) <= self.rank(x) {
                break;
            }
            self.rotate(x);
        }
        Ok(true)
    }
    /// Remove if present; missing keys leave state unchanged.
    pub fn remove(&mut self, key: i64) -> bool {
        let Some(x) = self.find(key) else {
            return false;
        };
        loop {
            let next = match self.nodes[x].child {
                [None, None] => break,
                [Some(a), None] | [None, Some(a)] => a,
                [Some(a), Some(b)] => {
                    if self.rank(a) < self.rank(b) {
                        a
                    } else {
                        b
                    }
                }
            };
            self.rotate(next);
        }
        if let Some(p) = self.nodes[x].parent {
            let s = usize::from(self.nodes[p].child[1] == Some(x));
            self.nodes[p].child[s] = None;
        } else {
            self.root = None;
        }
        self.free.push(x);
        true
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
