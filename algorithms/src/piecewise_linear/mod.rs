//! Exact rational linear interpolation and conditional curvature certificate.
//! See Atkinson, An Introduction to Numerical Analysis, interpolation remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Knot {
    pub x: i64,
    pub y: i64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TooFewKnots,
    Unordered,
    OutsideDomain,
    Overflow,
    WorkBudget,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Estimate {
    pub numerator: i128,
    pub denominator: u64,
    /// Absolute error <= error_numerator/8 under the documented premises.
    pub error_numerator: u128,
}
/// Interpolate at an integer x, without floating rounding or extrapolation.
/// Caller asserts exact samples of a C² function on the full knot domain
/// and |f''|<=curvature there. Samples alone cannot establish this premise.
/// Bound is absolute M*h²/8, NOT relative. Exact knot queries have zero error.
/// O(knots.len()) validation, O(log n) lookup, O(1) space.
/// Work budget bounds knot validation; no allocations.
pub fn interpolate(
    knots: &[Knot],
    x: i64,
    curvature: u64,
    work_budget: usize,
) -> Result<Estimate, Error> {
    if knots.len() > work_budget {
        return Err(Error::WorkBudget);
    }
    if knots.len() < 2 {
        return Err(Error::TooFewKnots);
    }
    if knots.windows(2).any(|w| w[0].x >= w[1].x) {
        return Err(Error::Unordered);
    }
    if x < knots[0].x || x > knots[knots.len() - 1].x {
        return Err(Error::OutsideDomain);
    }
    match knots.binary_search_by_key(&x, |p| p.x) {
        Ok(i) => Ok(Estimate {
            numerator: i128::from(knots[i].y),
            denominator: 1,
            error_numerator: 0,
        }),
        Err(i) => {
            let a = knots[i - 1];
            let b = knots[i];
            let h = (i128::from(b.x) - i128::from(a.x)) as u64;
            let left = i128::from(b.x) - i128::from(x);
            let right = i128::from(x) - i128::from(a.x);
            let numerator = i128::from(a.y)
                .checked_mul(left)
                .and_then(|v| {
                    i128::from(b.y)
                        .checked_mul(right)
                        .and_then(|w| v.checked_add(w))
                })
                .ok_or(Error::Overflow)?;
            let error_numerator = u128::from(curvature)
                .checked_mul(u128::from(h))
                .and_then(|v| v.checked_mul(u128::from(h)))
                .ok_or(Error::Overflow)?;
            Ok(Estimate {
                numerator,
                denominator: h,
                error_numerator,
            })
        }
    }
}
#[cfg(test)]
mod tests;
