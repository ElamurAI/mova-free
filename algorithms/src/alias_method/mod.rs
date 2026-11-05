//! Walker (1977), An efficient method for generating discrete random variables
//! with general distributions; Vose (1991), A linear algorithm for generating
//! random numbers with a given distribution. Original std-only implementation.
//! Draw coordinates are supplied by the caller; there is no hidden RNG.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Dimension,
    Weight,
    EmptyMass,
    Coordinate,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AliasTable {
    thresholds: Vec<f64>,
    aliases: Vec<usize>,
}
impl AliasTable {
    pub fn new(weights: &[f64]) -> Result<Self, Error> {
        let n = weights.len();
        if n == 0 || n > 4096 {
            return Err(Error::Dimension);
        }
        if weights.iter().any(|x| !x.is_finite() || *x < 0.0) {
            return Err(Error::Weight);
        }
        let max = weights.iter().copied().fold(0.0, f64::max);
        if max == 0.0 {
            return Err(Error::EmptyMass);
        }
        let total: f64 = weights.iter().map(|x| x / max).sum();
        let mut scaled: Vec<f64> = weights
            .iter()
            .map(|x| (x / max) / total * n as f64)
            .collect();
        let mut small = Vec::new();
        let mut large = Vec::new();
        for (i, &v) in scaled.iter().enumerate() {
            if v < 1.0 {
                small.push(i);
            } else {
                large.push(i);
            }
        }
        let mut result = Self {
            thresholds: vec![1.0; n],
            aliases: (0..n).collect(),
        };
        while !small.is_empty() && !large.is_empty() {
            let s = small.pop().unwrap();
            let l = large.pop().unwrap();
            result.thresholds[s] = scaled[s].clamp(0.0, 1.0);
            result.aliases[s] = l;
            scaled[l] = (scaled[l] - 1.0) + scaled[s];
            if scaled[l] < 1.0 {
                small.push(l);
            } else {
                large.push(l);
            }
        }
        Ok(result)
    }
    /// The half-open coin interval is [0,1). Threshold equality selects alias.
    pub fn draw(&self, column: usize, coin: f64) -> Result<usize, Error> {
        if column >= self.thresholds.len() || !coin.is_finite() || !(0.0..1.0).contains(&coin) {
            return Err(Error::Coordinate);
        }
        Ok(if coin < self.thresholds[column] {
            column
        } else {
            self.aliases[column]
        })
    }
    pub fn len(&self) -> usize {
        self.thresholds.len()
    }
    pub fn is_empty(&self) -> bool {
        self.thresholds.is_empty()
    }
}
#[cfg(test)]
mod tests;
