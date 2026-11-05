//! Subset zeta transform over Z/modulus Z, original implementation.
//! Fast subset transforms: Yates, The Design and Analysis of Factorial
//! Experiments (1937); subset-lattice treatment in Bjorklund et al.,
//! "Fourier meets Mobius: fast subset convolution", STOC 2007.
pub const MAX_LEN: usize = 1 << 20;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    ZeroModulus,
    Empty,
    NotPowerOfTwo,
    TooLong,
}
/// Replace f[S] with sum of original f[T] over T subset of S, modulo modulus.
/// Validate zero modulus, empty, power-of-two, then cap before any mutation.
/// Normalize all entries, including n=1. O(n log n) time, O(1) extra space.
/// Arbitrary nonzero u64 moduli work; additions use u128 before reduction.
pub fn transform(values: &mut [u64], modulus: u64) -> Result<(), Error> {
    if modulus == 0 {
        return Err(Error::ZeroModulus);
    }
    if values.is_empty() {
        return Err(Error::Empty);
    }
    if !values.len().is_power_of_two() {
        return Err(Error::NotPowerOfTwo);
    }
    if values.len() > MAX_LEN {
        return Err(Error::TooLong);
    }
    for value in values.iter_mut() {
        *value %= modulus;
    }
    let mut bit = 1;
    while bit < values.len() {
        for mask in 0..values.len() {
            if mask & bit != 0 {
                values[mask] = ((u128::from(values[mask]) + u128::from(values[mask ^ bit]))
                    % u128::from(modulus)) as u64;
            }
        }
        bit <<= 1;
    }
    Ok(())
}
#[cfg(test)]
#[path = "tests.rs"]
mod wave14_models;
