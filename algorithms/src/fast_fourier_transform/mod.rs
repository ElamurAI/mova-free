//! Original radix-two FFT; Cooley and Tukey, Math. Comp. 19 (1965).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidLength,
    TooLarge,
    InvalidValue,
}
/// Forward uses negative phase; inverse divides by length. No relative error guarantee.
/// Allocation exhaustion follows the standard allocator policy.
pub fn transform(input: &[Complex], inverse: bool) -> Result<Vec<Complex>, Error> {
    let n = input.len();
    if n > 65536 {
        return Err(Error::TooLarge);
    }
    if !n.is_power_of_two() {
        return Err(Error::InvalidLength);
    }
    if input
        .iter()
        .any(|z| !z.re.is_finite() || !z.im.is_finite() || z.re.abs() > 1e100 || z.im.abs() > 1e100)
    {
        return Err(Error::InvalidValue);
    }
    let mut a = input.to_vec();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let phase = if inverse { 2.0 } else { -2.0 } * std::f64::consts::PI / len as f64;
        for start in (0..n).step_by(len) {
            for k in 0..len / 2 {
                let theta = phase * k as f64;
                let (s, c) = theta.sin_cos();
                let u = a[start + k];
                let z = a[start + k + len / 2];
                let v = Complex {
                    re: z.re * c - z.im * s,
                    im: z.re * s + z.im * c,
                };
                a[start + k] = Complex {
                    re: u.re + v.re,
                    im: u.im + v.im,
                };
                a[start + k + len / 2] = Complex {
                    re: u.re - v.re,
                    im: u.im - v.im,
                };
            }
        }
        len *= 2;
    }
    if inverse {
        for z in &mut a {
            z.re /= n as f64;
            z.im /= n as f64;
        }
    }
    Ok(a)
}
#[cfg(test)]
mod tests;
