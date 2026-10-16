//! Minimal zip reader: central directory + deflate (flate2). The `zip` crate is not available offline, and FrameNet
//! ships as a zip archive — no need to unpack it to disk.
//!
//! Only what `framenet_v17.zip` uses is supported: methods 0 (stored) and 8 (deflate), no ZIP64 and no
//! encryption. Anything else is an error, not a silent skip.

use std::io::Read;

use anyhow::{bail, Context, Result};

pub struct Zip {
    data: Vec<u8>,
    /// name, method, compressed size, size, local header offset
    entries: Vec<(String, u16, u32, u32, u32)>,
}

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

impl Zip {
    pub fn open(path: &std::path::Path) -> Result<Zip> {
        let data = std::fs::read(path).with_context(|| format!("cannot read {}", path.display()))?;
        // end of central directory: signature 0x06054b50, searched from the end (comment up to 64 KB)
        let lo = data.len().saturating_sub(22 + 65535);
        let eocd = (lo..data.len().saturating_sub(21))
            .rev()
            .find(|&i| u32_at(&data, i) == 0x0605_4b50)
            .context("not a zip: no end of central directory")?;
        let n = u16_at(&data, eocd + 10) as usize;
        let mut p = u32_at(&data, eocd + 16) as usize;
        if n == 0xffff || p == 0xffff_ffff as usize {
            bail!("ZIP64 is not supported");
        }
        let mut entries = Vec::with_capacity(n);
        for _ in 0..n {
            if p + 46 > data.len() || u32_at(&data, p) != 0x0201_4b50 {
                bail!("corrupt central directory at offset {p}");
            }
            let flags = u16_at(&data, p + 8);
            let method = u16_at(&data, p + 10);
            let csize = u32_at(&data, p + 20);
            let size = u32_at(&data, p + 24);
            let (nl, el, cl) = (u16_at(&data, p + 28) as usize, u16_at(&data, p + 30) as usize, u16_at(&data, p + 32) as usize);
            let off = u32_at(&data, p + 42);
            let name = String::from_utf8_lossy(&data[p + 46..p + 46 + nl]).into_owned();
            if flags & 1 != 0 {
                bail!("{name}: encryption is not supported");
            }
            entries.push((name, method, csize, size, off));
            p += 46 + nl + el + cl;
        }
        Ok(Zip { data, entries })
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.0.as_str())
    }

    pub fn read(&self, name: &str) -> Result<Vec<u8>> {
        let (_, method, csize, size, off) = self.entries.iter().find(|e| e.0 == name).with_context(|| format!("{name} not in zip"))?.clone();
        let o = off as usize;
        if u32_at(&self.data, o) != 0x0403_4b50 {
            bail!("{name}: corrupt local header");
        }
        let start = o + 30 + u16_at(&self.data, o + 26) as usize + u16_at(&self.data, o + 28) as usize;
        let raw = &self.data[start..start + csize as usize];
        let out = match method {
            0 => raw.to_vec(),
            8 => {
                let mut v = Vec::with_capacity(size as usize);
                flate2::read::DeflateDecoder::new(raw).read_to_end(&mut v).with_context(|| format!("{name}: deflate"))?;
                v
            }
            m => bail!("{name}: compression method {m} is not supported"),
        };
        if out.len() != size as usize {
            bail!("{name}: size {} instead of {size}", out.len());
        }
        Ok(out)
    }
}
