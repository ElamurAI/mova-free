//! Newton divided differences; Burden and Faires, Numerical Analysis, interpolation chapter.
//! Construction O(n^2), evaluation O(n), storage O(n); rounded results have no accuracy bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    NonFiniteArithmetic,
}
#[derive(Debug, Clone)]
pub struct NewtonPolynomial {
    nodes: Vec<f64>,
    coefficients: Vec<f64>,
}
impl NewtonPolynomial {
    pub fn new(xs: &[f64], ys: &[f64]) -> Result<Self, Error> {
        if xs.is_empty()
            || xs.len() != ys.len()
            || xs.iter().chain(ys).any(|v| !v.is_finite())
            || xs.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(Error::InvalidInput);
        }
        let mut c = ys.to_vec();
        for j in 1..xs.len() {
            for i in (j..xs.len()).rev() {
                let d = xs[i] - xs[i - j];
                let n = c[i] - c[i - 1];
                if !d.is_finite() || !n.is_finite() {
                    return Err(Error::NonFiniteArithmetic);
                }
                c[i] = n / d;
                if !c[i].is_finite() {
                    return Err(Error::NonFiniteArithmetic);
                }
            }
        }
        Ok(Self {
            nodes: xs.to_vec(),
            coefficients: c,
        })
    }
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }
    pub fn evaluate(&self, x: f64) -> Result<f64, Error> {
        if !x.is_finite() {
            return Err(Error::InvalidInput);
        }
        let mut v = *self.coefficients.last().unwrap();
        for i in (0..self.nodes.len() - 1).rev() {
            let d = x - self.nodes[i];
            let p = v * d;
            v = p + self.coefficients[i];
            if !d.is_finite() || !p.is_finite() || !v.is_finite() {
                return Err(Error::NonFiniteArithmetic);
            }
        }
        Ok(v)
    }
}
#[cfg(test)]
mod tests;
