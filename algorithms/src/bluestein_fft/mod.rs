//! Chirp-convolution DFT; Bluestein, "A linear filtering approach to the computation of discrete Fourier transform" (1970).
use crate::fast_fourier_transform::{self as fft, Complex};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    Kernel(fft::Error),
    NonFiniteArithmetic,
}
fn multiply(a: Complex, b: Complex) -> Complex {
    Complex {
        re: a.re * b.re - a.im * b.im,
        im: a.re * b.im + a.im * b.re,
    }
}
fn chirp(j: usize, n: usize, sign: f64) -> Complex {
    // Reduce j² modulo 2n before conversion to keep phase bounded.
    let angle = sign * std::f64::consts::PI * ((j * j) % (2 * n)) as f64 / n as f64;
    let (im, re) = angle.sin_cos();
    Complex { re, im }
}
/// Any length 1..=16384, finite components |x|<=1e50; periodic DFT grid.
/// Negative forward phase, positive inverse phase with 1/n normalization.
/// Reuses radix-two FFT at next_power_of_two(2n-1). O(n log n)/O(n).
/// Standard allocator policy, no callbacks, no universal relative error guarantee.
pub fn transform(input: &[Complex], inverse: bool) -> Result<Vec<Complex>, Error> {
    let n = input.len();
    if n == 0
        || n > 16384
        || input.iter().any(|z| {
            !z.re.is_finite() || !z.im.is_finite() || z.re.abs() > 1e50 || z.im.abs() > 1e50
        })
    {
        return Err(Error::InvalidInput);
    }
    let m = (2 * n - 1).next_power_of_two();
    let zero = Complex { re: 0.0, im: 0.0 };
    let mut a = vec![zero; m];
    let mut b = vec![zero; m];
    let sign = if inverse { 1.0 } else { -1.0 };
    for j in 0..n {
        a[j] = multiply(input[j], chirp(j, n, sign));
        b[j] = chirp(j, n, -sign);
        if j > 0 {
            b[m - j] = b[j];
        }
    }
    let mut fa = fft::transform(&a, false).map_err(Error::Kernel)?;
    let fb = fft::transform(&b, false).map_err(Error::Kernel)?;
    for (x, y) in fa.iter_mut().zip(fb) {
        *x = multiply(*x, y);
    }
    let conv = fft::transform(&fa, true).map_err(Error::Kernel)?;
    let mut out = Vec::with_capacity(n);
    for (k, &value) in conv.iter().take(n).enumerate() {
        let mut z = multiply(value, chirp(k, n, sign));
        if inverse {
            z.re /= n as f64;
            z.im /= n as f64;
        }
        if !z.re.is_finite() || !z.im.is_finite() {
            return Err(Error::NonFiniteArithmetic);
        }
        out.push(z);
    }
    Ok(out)
}
#[cfg(test)]
mod tests;
