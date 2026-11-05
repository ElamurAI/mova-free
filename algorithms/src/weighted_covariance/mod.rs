//! West (1979), Updating mean and variance estimates: an improved method;
//! Chan, Golub and LeVeque (1983), Algorithms for computing the sample variance.
//! Population covariance, zero-weight neutral observations, atomic failures.
//! Floating results depend on grouping; no universal error certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Dimension,
    Value,
    Weight,
    Arithmetic,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WeightedCovariance {
    weight: f64,
    mean: Vec<f64>,
    m2: Vec<Vec<f64>>,
}
impl WeightedCovariance {
    pub fn new(dimension: usize) -> Result<Self, Error> {
        if dimension == 0 || dimension > 64 {
            return Err(Error::Dimension);
        }
        Ok(Self {
            weight: 0.0,
            mean: vec![0.0; dimension],
            m2: vec![vec![0.0; dimension]; dimension],
        })
    }
    pub fn push(&mut self, point: &[f64], weight: f64) -> Result<(), Error> {
        if point.len() != self.mean.len() {
            return Err(Error::Dimension);
        }
        if point.iter().any(|x| !x.is_finite() || x.abs() > 1e100) {
            return Err(Error::Value);
        }
        if !weight.is_finite() || !(0.0..=1e100).contains(&weight) {
            return Err(Error::Weight);
        }
        if weight == 0.0 {
            return Ok(());
        }
        let mut singleton = Self::new(point.len())?;
        singleton.weight = weight;
        singleton.mean.copy_from_slice(point);
        self.merge(&singleton)
    }
    pub fn merge(&mut self, other: &Self) -> Result<(), Error> {
        if self.mean.len() != other.mean.len() {
            return Err(Error::Dimension);
        }
        if other.weight == 0.0 {
            return Ok(());
        }
        if self.weight == 0.0 {
            *self = other.clone();
            return Ok(());
        }
        let total = self.weight + other.weight;
        if !total.is_finite() {
            return Err(Error::Arithmetic);
        }
        let ratio = other.weight / total;
        let factor = self.weight * ratio;
        let delta: Vec<f64> = other
            .mean
            .iter()
            .zip(&self.mean)
            .map(|(b, a)| b - a)
            .collect();
        let mean: Vec<f64> = self
            .mean
            .iter()
            .zip(&delta)
            .map(|(a, d)| a + d * ratio)
            .collect();
        let mut m2 = self.m2.clone();
        for (i, row) in m2.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value += other.m2[i][j] + (delta[i] * delta[j]) * factor;
            }
        }
        if mean.iter().any(|x| !x.is_finite()) || m2.iter().flatten().any(|x| !x.is_finite()) {
            return Err(Error::Arithmetic);
        }
        self.weight = total;
        self.mean = mean;
        self.m2 = m2;
        Ok(())
    }
    pub fn weight(&self) -> f64 {
        self.weight
    }
    pub fn mean(&self) -> Option<&[f64]> {
        (self.weight > 0.0).then_some(&self.mean)
    }
    pub fn covariance(&self) -> Option<Vec<Vec<f64>>> {
        (self.weight > 0.0).then(|| {
            self.m2
                .iter()
                .map(|row| row.iter().map(|x| x / self.weight).collect())
                .collect()
        })
    }
}
#[cfg(test)]
mod tests;
