//! Natural cubic spline, Thomas tridiagonal solve; de Boor, A Practical Guide to Splines.
//! O(n) build/storage and O(log n) query. No extrapolation or f64 accuracy guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}
#[derive(Debug, Clone)]
pub struct NaturalCubicSpline {
    xs: Vec<f64>,
    ys: Vec<f64>,
    second: Vec<f64>,
}
impl NaturalCubicSpline {
    pub fn new(xs: &[f64], ys: &[f64]) -> Result<Self, Error> {
        let n = xs.len();
        if n < 2
            || n != ys.len()
            || xs.iter().chain(ys).any(|v| !v.is_finite())
            || xs.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(Error::InvalidInput);
        }
        let mut h = Vec::with_capacity(n - 1);
        let mut slopes = Vec::with_capacity(n - 1);
        for i in 0..n - 1 {
            let d = xs[i + 1] - xs[i];
            let dy = ys[i + 1] - ys[i];
            let s = dy / d;
            if !d.is_finite() || !dy.is_finite() || !s.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
            h.push(d);
            slopes.push(s);
        }
        let mut upper = vec![0.0; n];
        let mut second = vec![0.0; n];
        for i in 1..n - 1 {
            let diag = 2.0 * (h[i - 1] + h[i]);
            let pivot = diag - h[i - 1] * upper[i - 1];
            let rhs = 6.0 * (slopes[i] - slopes[i - 1]) - h[i - 1] * second[i - 1];
            upper[i] = h[i] / pivot;
            second[i] = rhs / pivot;
            if [diag, pivot, rhs, upper[i], second[i]]
                .iter()
                .any(|v| !v.is_finite())
                || pivot <= 0.0
            {
                return Err(Error::NonFiniteArithmetic);
            }
        }
        for i in (1..n - 1).rev() {
            second[i] -= upper[i] * second[i + 1];
            if !second[i].is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
        Ok(Self {
            xs: xs.to_vec(),
            ys: ys.to_vec(),
            second,
        })
    }
    pub fn evaluate(&self, x: f64) -> Result<f64, Error> {
        if !x.is_finite() || x < self.xs[0] || x > *self.xs.last().unwrap() {
            return Err(Error::InvalidInput);
        }
        if let Ok(i) = self.xs.binary_search_by(|v| v.partial_cmp(&x).unwrap()) {
            return Ok(self.ys[i]);
        }
        let i = self.xs.partition_point(|v| *v < x) - 1;
        let h = self.xs[i + 1] - self.xs[i];
        let a = (self.xs[i + 1] - x) / h;
        let b = (x - self.xs[i]) / h;
        let correction =
            ((a * a * a - a) * self.second[i] + (b * b * b - b) * self.second[i + 1]) * h * h / 6.0;
        let value = a * self.ys[i] + b * self.ys[i + 1] + correction;
        if !correction.is_finite() || !value.is_finite() {
            Err(Error::NonFiniteArithmetic)
        } else {
            Ok(value)
        }
    }
}
#[cfg(test)]
mod tests;
