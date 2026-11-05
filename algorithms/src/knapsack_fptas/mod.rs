//! Integer profit-scaling FPTAS (Ibarra and Kim, JACM 22(4), 1975).
use crate::knapsack::Item;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnsupportedPercent,
    Overflow,
    WorkBudget,
    Allocation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Certificate {
    /// Feasible objective, without a selected-item witness.
    pub value: u128,
    /// OPT-value <= loss_numerator/100 <= percent*OPT/100.
    pub loss_numerator: u128,
    pub percent: u8,
    pub work: usize,
}
/// Exact rational scaling, tiers 1/2/5/10%, zero optimum exact.
/// O(n^3/epsilon) time, O(n^2/epsilon) space; work counts DP cells + input.
pub fn max_value(
    items: &[Item],
    capacity: usize,
    percent: u8,
    work_budget: usize,
) -> Result<Certificate, Error> {
    if !matches!(percent, 1 | 2 | 5 | 10) {
        return Err(Error::UnsupportedPercent);
    }
    if items.len() > work_budget {
        return Err(Error::WorkBudget);
    }
    let n = items
        .iter()
        .filter(|x| x.weight <= capacity && x.value > 0)
        .count();
    let vmax = items
        .iter()
        .filter(|x| x.weight <= capacity)
        .map(|x| x.value)
        .max()
        .unwrap_or(0);
    if vmax == 0 {
        return Ok(Certificate {
            value: 0,
            loss_numerator: 0,
            percent,
            work: items.len(),
        });
    }
    let factor = (n as u128).checked_mul(100).ok_or(Error::Overflow)?;
    let divisor = u128::from(percent) * u128::from(vmax);
    let scaled = |v: u64| -> Result<usize, Error> {
        usize::try_from(u128::from(v).checked_mul(factor).ok_or(Error::Overflow)? / divisor)
            .map_err(|_| Error::Overflow)
    };
    let mut total = 0usize;
    for x in items.iter().filter(|x| x.weight <= capacity && x.value > 0) {
        total = total.checked_add(scaled(x.value)?).ok_or(Error::Overflow)?;
    }
    let width = total.checked_add(1).ok_or(Error::Overflow)?;
    let work = n
        .checked_mul(width)
        .and_then(|w| w.checked_add(items.len()))
        .ok_or(Error::Overflow)?;
    if work > work_budget {
        return Err(Error::WorkBudget);
    }
    let mut dp: Vec<Option<(usize, u128)>> = Vec::new();
    dp.try_reserve_exact(width).map_err(|_| Error::Allocation)?;
    dp.resize(width, None);
    dp[0] = Some((0, 0));
    for x in items.iter().filter(|x| x.weight <= capacity && x.value > 0) {
        let p = scaled(x.value)?;
        for j in (p..width).rev() {
            if let Some((w, v)) = dp[j - p] {
                if x.weight > capacity - w {
                    continue;
                }
                let candidate = (
                    w + x.weight,
                    v.checked_add(u128::from(x.value)).ok_or(Error::Overflow)?,
                );
                if dp[j].is_none_or(|old| {
                    candidate.0 < old.0 || (candidate.0 == old.0 && candidate.1 > old.1)
                }) {
                    dp[j] = Some(candidate);
                }
            }
        }
    }
    let value = dp.iter().flatten().map(|x| x.1).max().unwrap_or(0);
    Ok(Certificate {
        value,
        loss_numerator: divisor,
        percent,
        work,
    })
}
#[cfg(test)]
mod tests;
