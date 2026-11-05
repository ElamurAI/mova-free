//! Fixed-width histogram inversion; cf. Cormode et al. (2012),
//! Synopses for Massive Data: Samples, Histograms, Wavelets, Sketches.
//! Exact bucket counts yield honest value/rank intervals; no relative-value
//! or normalized-rank error tier is promised.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidDomain,
    TooManyBuckets,
    Allocation,
    OutOfDomain,
    Overflow,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuantileInterval {
    pub value_min: u64,
    pub value_max: u64,
    pub rank_start: u64,
    pub rank_end: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistogramQuantiles {
    min: u64,
    max: u64,
    width: u64,
    total: u64,
    buckets: Vec<u64>,
}
impl HistogramQuantiles {
    /// At most one million buckets; inclusive domain, positive width.
    pub fn new(min: u64, max: u64, width: u64) -> Result<Self, Error> {
        if min > max || width == 0 {
            return Err(Error::InvalidDomain);
        }
        let n = (max as u128 - min as u128) / width as u128 + 1;
        if n > 1_000_000 {
            return Err(Error::TooManyBuckets);
        }
        let mut buckets = Vec::new();
        buckets
            .try_reserve_exact(n as usize)
            .map_err(|_| Error::Allocation)?;
        buckets.resize(n as usize, 0);
        Ok(Self {
            min,
            max,
            width,
            total: 0,
            buckets,
        })
    }
    pub fn update(&mut self, value: u64) -> Result<(), Error> {
        if value < self.min || value > self.max {
            return Err(Error::OutOfDomain);
        }
        let total = self.total.checked_add(1).ok_or(Error::Overflow)?;
        let index = ((value - self.min) / self.width) as usize;
        self.buckets[index] += 1;
        self.total = total;
        Ok(())
    }
    /// Zero-based requested rank; rank_end is exclusive. Empty/out-of-range is None.
    pub fn quantile(&self, rank: u64) -> Option<QuantileInterval> {
        if rank >= self.total {
            return None;
        }
        let mut start = 0;
        for (i, &count) in self.buckets.iter().enumerate() {
            let end = start + count;
            if rank < end {
                let low = self.min as u128 + i as u128 * self.width as u128;
                let high = (low + self.width as u128 - 1).min(self.max as u128);
                return Some(QuantileInterval {
                    value_min: low as u64,
                    value_max: high as u64,
                    rank_start: start,
                    rank_end: end,
                });
            }
            start = end;
        }
        None
    }
    pub fn len(&self) -> u64 {
        self.total
    }
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
}
#[cfg(test)]
mod tests;
