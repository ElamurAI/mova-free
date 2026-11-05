//! Stoer-Wagner global minimum cut of an undirected nonnegative graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    NonSquare {
        row: usize,
        expected: usize,
        actual: usize,
    },
    Asymmetric {
        from: usize,
        to: usize,
    },
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub weight: u128,
    /// Sorted nonempty proper side, oriented to contain vertex zero.
    pub side: Vec<usize>,
}

/// Symmetric u64 adjacency matrix; zero means no capacity, diagonal is ignored.
/// Validate all row lengths, then upper-triangle symmetry. Fewer than two
/// vertices return None. Disconnected graphs have a zero cut. Phase selection
/// breaks ties by smallest surviving vertex id. Equal phase cuts use the
/// lexicographically smaller oriented side; global lexicographic optimality
/// among all minimum cuts is not promised. All sums use checked u128 arithmetic.
/// O(V^3) time and O(V^2) auxiliary space; input is unchanged. Allocation failure
/// is outside the returned errors.
pub fn minimum_cut(capacity: &[Vec<u64>]) -> Result<Option<Cut>, Error> {
    let n = capacity.len();
    for (row, values) in capacity.iter().enumerate() {
        if values.len() != n {
            return Err(Error::NonSquare {
                row,
                expected: n,
                actual: values.len(),
            });
        }
    }
    for (from, row) in capacity.iter().enumerate() {
        for (to, other) in capacity.iter().enumerate().skip(from + 1) {
            if row[to] != other[from] {
                return Err(Error::Asymmetric { from, to });
            }
        }
    }
    if n < 2 {
        return Ok(None);
    }
    let mut weights: Vec<Vec<u128>> = capacity
        .iter()
        .map(|row| row.iter().map(|&x| u128::from(x)).collect())
        .collect();
    let mut active: Vec<usize> = (0..n).collect();
    let mut groups: Vec<Vec<usize>> = (0..n).map(|v| vec![v]).collect();
    let mut best: Option<Cut> = None;
    while active.len() > 1 {
        let mut added = vec![false; n];
        let mut connection = vec![0u128; n];
        let mut previous = active[0];
        for step in 0..active.len() {
            let vertex = active
                .iter()
                .copied()
                .filter(|&v| !added[v])
                .max_by_key(|&v| (connection[v], std::cmp::Reverse(v)))
                .expect("phase has an unselected vertex");
            if step + 1 == active.len() {
                let mut side = groups[vertex].clone();
                side.sort_unstable();
                if side.binary_search(&0).is_err() {
                    side = (0..n).filter(|v| side.binary_search(v).is_err()).collect();
                }
                let candidate = Cut {
                    weight: connection[vertex],
                    side,
                };
                if best
                    .as_ref()
                    .is_none_or(|old| (candidate.weight, &candidate.side) < (old.weight, &old.side))
                {
                    best = Some(candidate);
                }
                for &other in &active {
                    if other != previous && other != vertex {
                        let sum = weights[previous][other]
                            .checked_add(weights[vertex][other])
                            .ok_or(Error::ArithmeticOverflow)?;
                        weights[previous][other] = sum;
                        weights[other][previous] = sum;
                    }
                }
                let merged = std::mem::take(&mut groups[vertex]);
                groups[previous].extend(merged);
                active.retain(|&v| v != vertex);
                break;
            }
            added[vertex] = true;
            for &other in &active {
                if !added[other] {
                    connection[other] = connection[other]
                        .checked_add(weights[vertex][other])
                        .ok_or(Error::ArithmeticOverflow)?;
                }
            }
            previous = vertex;
        }
    }
    Ok(best)
}

#[cfg(test)]
mod tests;
