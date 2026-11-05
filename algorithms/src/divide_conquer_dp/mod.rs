//! Monotone divide-and-conquer DP; see Yao (1980), Efficient dynamic programming using quadrangle inequalities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DimensionOverflow,
    AllocationFailed,
    BudgetExceeded,
    ArithmeticOverflow,
}
struct Layer<'a, F> {
    prev: &'a [Option<u128>],
    next: &'a mut [Option<u128>],
    cost: &'a mut F,
    calls: &'a mut usize,
    budget: usize,
}
impl<F: FnMut(usize, usize) -> u128> Layer<'_, F> {
    fn fill(&mut self, lo: usize, hi: usize, opt_lo: usize, opt_hi: usize) -> Result<(), Error> {
        if lo > hi {
            return Ok(());
        }
        let mid = lo + (hi - lo) / 2;
        let mut best = None;
        let mut split = opt_lo;
        for k in opt_lo..=opt_hi.min(mid - 1) {
            if let Some(previous) = self.prev[k] {
                if *self.calls == self.budget {
                    return Err(Error::BudgetExceeded);
                }
                *self.calls += 1;
                let value = previous
                    .checked_add((self.cost)(k, mid))
                    .ok_or(Error::ArithmeticOverflow)?;
                if best.is_none_or(|b| value < b) {
                    best = Some(value);
                    split = k;
                }
            }
        }
        self.next[mid] = best;
        if mid > lo {
            self.fill(lo, mid - 1, opt_lo, split)?;
        }
        if mid < hi {
            self.fill(mid + 1, hi, split, opt_hi)?;
        }
        Ok(())
    }
}
/// Minimum cost partition of [0,n) into exactly layers nonempty intervals.
/// Recurrence D[g,j]=min_k D[g-1,k]+cost(k,j), g-1<=k<j.
/// Caller guarantees nondecreasing LEFTMOST optimal splits in j in every layer,
/// and deterministic callback values. The assumption is not verified. Panics propagate.
/// Impossible partitions return None; (0,0) returns Some(0), without callbacks.
/// O(layers*n*log(n)) evaluations, O(n) memory. All candidate sums checked;
/// overflow is an error even if another candidate could have won.
pub fn partition<F: FnMut(usize, usize) -> u128>(
    n: usize,
    layers: usize,
    max_evaluations: usize,
    mut cost: F,
) -> Result<Option<u128>, Error> {
    if layers == 0 {
        return Ok(if n == 0 { Some(0) } else { None });
    }
    if layers > n {
        return Ok(None);
    }
    let width = n.checked_add(1).ok_or(Error::DimensionOverflow)?;
    if width > isize::MAX as usize / std::mem::size_of::<Option<u128>>() {
        return Err(Error::DimensionOverflow);
    }
    let mut prev = Vec::new();
    prev.try_reserve_exact(width)
        .map_err(|_| Error::AllocationFailed)?;
    prev.resize(width, None);
    prev[0] = Some(0);
    let mut next = Vec::new();
    next.try_reserve_exact(width)
        .map_err(|_| Error::AllocationFailed)?;
    next.resize(width, None);
    let mut calls = 0;
    for layer in 1..=layers {
        next.fill(None);
        Layer {
            prev: &prev,
            next: &mut next,
            cost: &mut cost,
            calls: &mut calls,
            budget: max_evaluations,
        }
        .fill(layer, n, layer - 1, n - 1)?;
        std::mem::swap(&mut prev, &mut next);
    }
    Ok(prev[n])
}
#[cfg(test)]
mod tests;
