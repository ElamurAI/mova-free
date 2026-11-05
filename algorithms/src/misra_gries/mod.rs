//! Deterministic Misra–Gries (1982), Finding repeated elements.
//! Tiers bound additive frequency error divided by stream length, not relative
//! error of a rare key. Allocation failure follows standard BTreeMap behavior.
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    UnsupportedPercent,
    Overflow,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MisraGries {
    capacity: usize,
    total: u64,
    cancellations: u64,
    counters: BTreeMap<u64, u64>,
}
impl MisraGries {
    pub fn new(error_percent: u8) -> Result<Self, Error> {
        if !matches!(error_percent, 1 | 2 | 5 | 10) {
            return Err(Error::UnsupportedPercent);
        }
        Ok(Self {
            capacity: 100 / error_percent as usize,
            total: 0,
            cancellations: 0,
            counters: BTreeMap::new(),
        })
    }
    pub fn update(&mut self, key: u64) -> Result<(), Error> {
        let total = self.total.checked_add(1).ok_or(Error::Overflow)?;
        if let Some(c) = self.counters.get_mut(&key) {
            *c += 1;
        } else if self.counters.len() < self.capacity {
            self.counters.insert(key, 1);
        } else {
            self.counters.retain(|_, c| {
                *c -= 1;
                *c > 0
            });
            self.cancellations += 1;
        }
        self.total = total;
        Ok(())
    }
    /// Inclusive lower/upper frequency bounds, including untracked keys.
    pub fn bounds(&self, key: u64) -> (u64, u64) {
        let low = self.counters.get(&key).copied().unwrap_or(0);
        (low, low + self.cancellations)
    }
    pub fn len(&self) -> u64 {
        self.total
    }
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
    pub fn counters(&self) -> &BTreeMap<u64, u64> {
        &self.counters
    }
}
#[cfg(test)]
mod tests;
