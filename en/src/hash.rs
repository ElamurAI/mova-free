//! Hash for model tables: multiplication with rotation (like FxHash in rustc). Keys are numbers, enums and
//! structures of them; the hash is deterministic, so both map traversal and training are reproducible across runs.

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

const K: u64 = 0x517c_c1b7_2722_0a95;

#[derive(Default, Clone, Copy)]
pub struct Fx(u64);

impl Fx {
    #[inline]
    fn add(&mut self, x: u64) {
        self.0 = (self.0.rotate_left(5) ^ x).wrapping_mul(K);
    }
}

impl Hasher for Fx {
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut b = [0u8; 8];
            b[..chunk.len()].copy_from_slice(chunk);
            self.add(u64::from_le_bytes(b));
        }
    }
    fn write_u8(&mut self, i: u8) {
        self.add(i as u64);
    }
    fn write_u16(&mut self, i: u16) {
        self.add(i as u64);
    }
    fn write_u32(&mut self, i: u32) {
        self.add(i as u64);
    }
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

pub type FastMap<K, V> = HashMap<K, V, BuildHasherDefault<Fx>>;
pub type FastSet<K> = HashSet<K, BuildHasherDefault<Fx>>;

/// Feature key from several numbers (the first is the template number).
#[inline]
pub fn key(parts: &[u64]) -> u64 {
    let mut h = Fx(0x9e37_79b9_7f4a_7c15);
    for &p in parts {
        h.add(p);
    }
    h.0
}

/// String hash for static dictionary tables (the same in build.rs and at run time).
pub fn str_key(s: &str) -> u64 {
    let mut h = Fx(0x9e37_79b9_7f4a_7c15);
    h.write(s.as_bytes());
    h.write_usize(s.len());
    h.0
}
