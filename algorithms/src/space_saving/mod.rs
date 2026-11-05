//! Space-Saving: Metwally, Agrawal and El Abbadi (2005),
//! Efficient computation of frequent and top-k elements in data streams.
//! Error/N is additive; no relative guarantee for a rare key.
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    UnsupportedPercent,
    Overflow,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceSaving {
    capacity: usize,
    total: u64,
    counters: BTreeMap<u64, (u64, u64)>,
}
impl SpaceSaving {
    pub fn new(error_percent: u8) -> Result<Self, Error> {
        if !matches!(error_percent, 1 | 2 | 5 | 10) {
            return Err(Error::UnsupportedPercent);
        }
        Ok(Self {
            capacity: 100 / error_percent as usize,
            total: 0,
            counters: BTreeMap::new(),
        })
    }
    pub fn update(&mut self, key: u64) -> Result<(), Error> {
        let total = self.total.checked_add(1).ok_or(Error::Overflow)?;
        if let Some((c, _)) = self.counters.get_mut(&key) {
            *c += 1;
        } else if self.counters.len() < self.capacity {
            self.counters.insert(key, (1, 0));
        } else {
            let (&victim, &(minimum, _)) = self
                .counters
                .iter()
                .min_by_key(|(k, (c, _))| (*c, **k))
                .unwrap();
            self.counters.remove(&victim);
            self.counters.insert(key, (minimum + 1, minimum));
        }
        self.total = total;
        Ok(())
    }
    pub fn bounds(&self, key: u64) -> (u64, u64) {
        match self.counters.get(&key) {
            Some(&(count, error)) => (count - error, count),
            None => (
                0,
                if self.counters.len() < self.capacity {
                    0
                } else {
                    self.counters.values().map(|x| x.0).min().unwrap_or(0)
                },
            ),
        }
    }
    pub fn len(&self) -> u64 {
        self.total
    }
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
    /// Values are (upper estimate, inherited error).
    pub fn counters(&self) -> &BTreeMap<u64, (u64, u64)> {
        &self.counters
    }
}
#[cfg(test)]
mod tests;
