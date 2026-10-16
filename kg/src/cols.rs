//! Edges as columns (structure of arrays): src, dst — u64; rel, gs, gd, flags — u32.
//! Sorted by (src, rel, dst): a vertex's adjacency is one contiguous slice (CSR), column filters
//! vectorize. Flags (design note: so that specific tasks need not crawl the whole graph but work on a
//! subgraph) — a mask of source, language, layer and check; subgraph = mask filter.
//! On disk: src — LEB128 deltas, dst — zigzag LEB128 deltas, u32 columns — byte planes;
//! everything zstd-compressed.

use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, bail};

/// Edge flags: bits 0–7 — source, 8–15 — language, 16–19 — layer, 20 — checked by synthesis.
pub mod flag {
    pub const SRC_TATOEBA: u32 = 1 << 0;
    pub const SRC_MANUAL: u32 = 1 << 1;
    pub const LANG_UK: u32 = 1 << 8;
    pub const LANG_EN: u32 = 1 << 9;
    pub const LAYER_FACT: u32 = 1 << 16;
    pub const LAYER_PROV: u32 = 1 << 17;
    pub const LAYER_TRANSLATION: u32 = 1 << 18;
    pub const CHECKED: u32 = 1 << 20;

    pub const NAMES: [(u32, &str); 8] = [
        (SRC_TATOEBA, "tatoeba"),
        (SRC_MANUAL, "manual"),
        (LANG_UK, "uk"),
        (LANG_EN, "en"),
        (LAYER_FACT, "fact"),
        (LAYER_PROV, "prov"),
        (LAYER_TRANSLATION, "translation"),
        (CHECKED, "checked"),
    ];

    /// `uk,fact,checked` → mask.
    pub fn parse(s: &str) -> Option<u32> {
        s.split(',').try_fold(0, |m, name| NAMES.iter().find(|(_, n)| *n == name).map(|(b, _)| m | b))
    }
}

#[derive(Default, Clone)]
pub struct Edges {
    pub src: Vec<u64>,
    pub dst: Vec<u64>,
    pub rel: Vec<u32>,
    pub gs: Vec<u32>,
    pub gd: Vec<u32>,
    pub flags: Vec<u32>,
}

impl Edges {
    pub fn len(&self) -> usize {
        self.src.len()
    }

    pub fn push(&mut self, src: u64, rel: u32, dst: u64, gs: u32, gd: u32, flags: u32) {
        self.src.push(src);
        self.dst.push(dst);
        self.rel.push(rel);
        self.gs.push(gs);
        self.gd.push(gd);
        self.flags.push(flags);
    }

    /// Sort by (src, rel, dst) via a permutation; columns are permuted together.
    pub fn sort(&mut self) {
        let mut idx: Vec<u32> = (0..self.len() as u32).collect();
        idx.sort_unstable_by_key(|&i| (self.src[i as usize], self.rel[i as usize], self.dst[i as usize]));
        fn take<T: Copy>(v: &[T], idx: &[u32]) -> Vec<T> {
            idx.iter().map(|&i| v[i as usize]).collect()
        }
        *self = Edges {
            src: take(&self.src, &idx),
            dst: take(&self.dst, &idx),
            rel: take(&self.rel, &idx),
            gs: take(&self.gs, &idx),
            gd: take(&self.gd, &idx),
            flags: take(&self.flags, &idx),
        };
    }

    /// Subgraph: edges that have all bits of the mask. A simple loop over a contiguous column — vectorizes.
    pub fn filter(&self, mask: u32) -> Edges {
        let keep: Vec<usize> = (0..self.len()).filter(|&i| self.flags[i] & mask == mask).collect();
        fn take<T: Copy>(v: &[T], keep: &[usize]) -> Vec<T> {
            keep.iter().map(|&i| v[i]).collect()
        }
        Edges {
            src: take(&self.src, &keep),
            dst: take(&self.dst, &keep),
            rel: take(&self.rel, &keep),
            gs: take(&self.gs, &keep),
            gd: take(&self.gd, &keep),
            flags: take(&self.flags, &keep),
        }
    }

    /// Range of edges of vertex `src` (the column is sorted).
    pub fn out(&self, src: u64) -> std::ops::Range<usize> {
        let a = self.src.partition_point(|&s| s < src);
        let b = self.src.partition_point(|&s| s <= src);
        a..b
    }

    /// Writes the columns to `dir`; returns (raw bytes, compressed bytes).
    pub fn write(&self, dir: &Path) -> Result<(usize, usize)> {
        fs::create_dir_all(dir)?;
        let mut raw = 0;
        let mut packed = 0;
        let mut put = |name: &str, bytes: Vec<u8>, raw_len: usize| -> Result<()> {
            let z = zstd::encode_all(bytes.as_slice(), 19)?;
            raw += raw_len;
            packed += z.len();
            fs::write(dir.join(format!("{name}.zst")), z)?;
            Ok(())
        };
        put("src", delta_u64(&self.src), self.len() * 8)?;
        put("dst", zigzag_u64(&self.dst), self.len() * 8)?;
        for (name, col) in [("rel", &self.rel), ("gs", &self.gs), ("gd", &self.gd), ("flags", &self.flags)] {
            put(name, planes_u32(col), self.len() * 4)?;
        }
        fs::write(dir.join("count"), self.len().to_string())?;
        Ok((raw, packed))
    }

    pub fn read(dir: &Path) -> Result<Edges> {
        let n: usize = fs::read_to_string(dir.join("count")).context("missing count")?.trim().parse()?;
        let get = |name: &str| -> Result<Vec<u8>> {
            let mut out = Vec::new();
            zstd::Decoder::new(fs::File::open(dir.join(format!("{name}.zst")))?)?.read_to_end(&mut out)?;
            Ok(out)
        };
        let e = Edges {
            src: undelta_u64(&get("src")?, n)?,
            dst: unzigzag_u64(&get("dst")?, n)?,
            rel: unplanes_u32(&get("rel")?, n)?,
            gs: unplanes_u32(&get("gs")?, n)?,
            gd: unplanes_u32(&get("gd")?, n)?,
            flags: unplanes_u32(&get("flags")?, n)?,
        };
        Ok(e)
    }
}

fn leb(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

fn unleb(bytes: &[u8], pos: &mut usize) -> Result<u64> {
    let (mut v, mut shift) = (0u64, 0);
    loop {
        let b = *bytes.get(*pos).context("truncated LEB128")?;
        *pos += 1;
        v |= u64::from(b & 0x7f) << shift;
        if b & 0x80 == 0 {
            return Ok(v);
        }
        shift += 7;
        if shift > 63 {
            bail!("LEB128 too long");
        }
    }
}

fn delta_u64(v: &[u64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len() * 2);
    let mut prev = 0u64;
    for &x in v {
        leb(&mut out, x.wrapping_sub(prev));
        prev = x;
    }
    out
}

fn undelta_u64(b: &[u8], n: usize) -> Result<Vec<u64>> {
    let (mut pos, mut prev) = (0, 0u64);
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        prev = prev.wrapping_add(unleb(b, &mut pos)?);
        out.push(prev);
    }
    Ok(out)
}

fn zigzag_u64(v: &[u64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len() * 3);
    let mut prev = 0u64;
    for &x in v {
        let d = x.wrapping_sub(prev) as i64;
        leb(&mut out, ((d << 1) ^ (d >> 63)) as u64);
        prev = x;
    }
    out
}

fn unzigzag_u64(b: &[u8], n: usize) -> Result<Vec<u64>> {
    let (mut pos, mut prev) = (0, 0u64);
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let z = unleb(b, &mut pos)?;
        let d = ((z >> 1) as i64) ^ -((z & 1) as i64);
        prev = prev.wrapping_add(d as u64);
        out.push(prev);
    }
    Ok(out)
}

/// u32 → four byte planes (all low bytes, then all second bytes…): identical high bytes compress to nothing.
fn planes_u32(v: &[u32]) -> Vec<u8> {
    let mut out = vec![0u8; v.len() * 4];
    for (i, &x) in v.iter().enumerate() {
        for (p, b) in x.to_le_bytes().into_iter().enumerate() {
            out[p * v.len() + i] = b;
        }
    }
    out
}

fn unplanes_u32(b: &[u8], n: usize) -> Result<Vec<u32>> {
    if b.len() != n * 4 {
        bail!("planes: {} bytes instead of {}", b.len(), n * 4);
    }
    Ok((0..n).map(|i| u32::from_le_bytes([b[i], b[n + i], b[2 * n + i], b[3 * n + i]])).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_filter() {
        let mut e = Edges::default();
        e.push(10 << 32 | 5, 2, 7 << 32 | 1, 11, 12, flag::LANG_UK | flag::CHECKED);
        e.push(10 << 32 | 3, 1, 7 << 32 | 9, 21, 22, flag::LANG_EN);
        e.push(10 << 32 | 5, 1, 3, 31, 32, flag::LANG_UK);
        e.sort();
        assert_eq!(e.src, vec![10 << 32 | 3, 10 << 32 | 5, 10 << 32 | 5]);
        assert_eq!(e.rel, vec![1, 1, 2]);
        let dir = std::env::temp_dir().join(format!("kg-cols-{}", std::process::id()));
        e.write(&dir).unwrap();
        let r = Edges::read(&dir).unwrap();
        assert_eq!((r.src, r.dst.clone(), r.rel, r.gs, r.gd, r.flags.clone()), (e.src.clone(), e.dst.clone(), e.rel.clone(), e.gs.clone(), e.gd.clone(), e.flags.clone()));
        assert_eq!(e.filter(flag::LANG_UK).len(), 2);
        assert_eq!(e.filter(flag::LANG_UK | flag::CHECKED).len(), 1);
        assert_eq!(e.out(10 << 32 | 5), 1..3);
        fs::remove_dir_all(dir).unwrap();
    }
}
