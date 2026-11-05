//! Min Cartesian tree with inorder indices and leftmost equal minima.
//! Original monotone-stack implementation; Cartesian trees: J. Vuillemin,
//! "A unifying look at data structures", Communications of the ACM 23(4), 1980.

pub const MAX_LEN: usize = 1 << 20;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooLong,
    AllocationFailed,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartesianTree {
    pub root: Option<usize>,
    pub parent: Vec<Option<usize>>,
    pub left: Vec<Option<usize>>,
    pub right: Vec<Option<usize>>,
}
fn links(n: usize) -> Result<Vec<Option<usize>>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(n, None);
    Ok(v)
}
/// O(n) comparisons/space; input is unchanged. Requires lawful Ord.
/// Length is checked before allocation. Empty input has no root.
/// Equal values stay on the stack, selecting the leftmost interval minimum.
pub fn min_cartesian_tree<T: Ord>(values: &[T]) -> Result<CartesianTree, Error> {
    let n = values.len();
    if n > MAX_LEN {
        return Err(Error::TooLong);
    }
    let mut tree = CartesianTree {
        root: None,
        parent: links(n)?,
        left: links(n)?,
        right: links(n)?,
    };
    let mut stack: Vec<usize> = Vec::new();
    stack
        .try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    for i in 0..n {
        let mut last = None;
        while stack.last().is_some_and(|&j| values[j] > values[i]) {
            last = stack.pop();
        }
        if let Some(&j) = stack.last() {
            tree.parent[i] = Some(j);
            tree.right[j] = Some(i);
        }
        if let Some(j) = last {
            tree.parent[j] = Some(i);
            tree.left[i] = Some(j);
        }
        stack.push(i);
    }
    tree.root = stack.first().copied();
    Ok(tree)
}
#[cfg(test)]
#[path = "tests.rs"]
mod wave14_models;
