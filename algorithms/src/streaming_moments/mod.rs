//! Welford (1962), Note on a method for calculating corrected sums of
//! squares and products; Chan, Golub and LeVeque (1983), Algorithms for
//! computing the sample variance. IEEE f64 arithmetic, no universal relative
//! error certificate. Underflow may erase tiny variance. Updates are atomic
//! on reported errors; intermediate overflow is rejected even if a rearranged
//! computation could succeed. Count is limited to 2^53 for exact conversion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NonFinite,
    Overflow,
    CountLimit,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StreamingMoments {
    count: u64,
    mean: f64,
    m2: f64,
}
impl StreamingMoments {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, value: f64) -> Result<(), Error> {
        if !value.is_finite() {
            return Err(Error::NonFinite);
        }
        let n = self
            .count
            .checked_add(1)
            .filter(|&n| n <= 1 << 53)
            .ok_or(Error::CountLimit)?;
        let delta = value - self.mean;
        let mean = self.mean + delta / n as f64;
        let m2 = self.m2 + delta * (value - mean);
        if !mean.is_finite() || !m2.is_finite() || m2 < 0.0 {
            return Err(Error::Overflow);
        }
        self.count = n;
        self.mean = mean;
        self.m2 = m2;
        Ok(())
    }
    pub fn merge(&mut self, other: &Self) -> Result<(), Error> {
        let n = self
            .count
            .checked_add(other.count)
            .filter(|&n| n <= 1 << 53)
            .ok_or(Error::CountLimit)?;
        if other.count == 0 {
            return Ok(());
        }
        if self.count == 0 {
            *self = other.clone();
            return Ok(());
        }
        let delta = other.mean - self.mean;
        let mean = self.mean + delta * (other.count as f64 / n as f64);
        let m2 = self.m2
            + other.m2
            + delta * delta * (self.count as f64 * (other.count as f64 / n as f64));
        if !mean.is_finite() || !m2.is_finite() || m2 < 0.0 {
            return Err(Error::Overflow);
        }
        self.count = n;
        self.mean = mean;
        self.m2 = m2;
        Ok(())
    }
    pub fn count(&self) -> u64 {
        self.count
    }
    pub fn mean(&self) -> Option<f64> {
        (self.count > 0).then_some(self.mean)
    }
    pub fn population_variance(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.m2 / self.count as f64)
        }
    }
    pub fn sample_variance(&self) -> Option<f64> {
        if self.count < 2 {
            None
        } else {
            Some(self.m2 / (self.count - 1) as f64)
        }
    }
}
#[cfg(test)]
mod tests;
