//! Exact linear recurrence evaluation by polynomial reduction (Kitamasa method).
//! Original implementation. Polynomial recurrence background: J. von zur Gathen
//! and J. Gerhard, Modern Computer Algebra, 3rd edition, 2013.

/// Explicit resource policy; work remains quadratic in this order.
pub const MAX_ORDER: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ZeroModulus,
    EmptyOrder,
    DimensionMismatch,
    OrderTooLarge,
    AllocationFailed,
}

fn zeros(len: usize) -> Result<Vec<u64>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(len, 0);
    Ok(v)
}

fn add_product(acc: u64, a: u64, b: u64, modulus: u64) -> u64 {
    ((u128::from(acc) + u128::from(a) * u128::from(b)) % u128::from(modulus)) as u64
}

fn combine(a: &[u64], b: &[u64], coefficients: &[u64], modulus: u64) -> Result<Vec<u64>, Error> {
    let k = coefficients.len();
    let mut product = zeros(2 * k - 1)?;
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            product[i + j] = add_product(product[i + j], x, y, modulus);
        }
    }
    // x^d = sum_j c[j] x^(d-1-j), applied from highest degree down.
    for degree in (k..product.len()).rev() {
        for (j, &c) in coefficients.iter().enumerate() {
            let target = degree - 1 - j;
            product[target] = add_product(product[target], product[degree], c % modulus, modulus);
        }
    }
    product.truncate(k);
    Ok(product)
}

/// Return a[n] modulo modulus for a[t] = sum_j coefficients[j] * a[t-1-j].
/// initial contains a[0..k]. Inputs are reduced modulo any nonzero u64 modulus.
/// Validation precedence: zero modulus, empty order, dimension mismatch, order
/// cap. Validation applies even when n < k. Allocation refusal is reported.
/// O(k² log(n+1)) word operations and O(k) auxiliary words for n >= k;
/// n < k returns directly after validation. No field/invertibility is required.
pub fn nth_term(
    initial: &[u64],
    coefficients: &[u64],
    mut n: u64,
    modulus: u64,
) -> Result<u64, Error> {
    if modulus == 0 {
        return Err(Error::ZeroModulus);
    }
    let k = coefficients.len();
    if k == 0 {
        return Err(Error::EmptyOrder);
    }
    if initial.len() != k {
        return Err(Error::DimensionMismatch);
    }
    if k > MAX_ORDER {
        return Err(Error::OrderTooLarge);
    }
    if n < k as u64 {
        return Ok(initial[n as usize] % modulus);
    }
    let mut result = zeros(k)?;
    result[0] = 1 % modulus;
    let mut power = zeros(k)?;
    if k == 1 {
        power[0] = coefficients[0] % modulus;
    } else {
        power[1] = 1 % modulus;
    }
    while n != 0 {
        if n & 1 != 0 {
            result = combine(&result, &power, coefficients, modulus)?;
        }
        n >>= 1;
        if n != 0 {
            power = combine(&power, &power, coefficients, modulus)?;
        }
    }
    Ok(result
        .iter()
        .zip(initial)
        .fold(0, |acc, (&weight, &value)| {
            add_product(acc, weight, value % modulus, modulus)
        }))
}

#[cfg(test)]
mod tests;
