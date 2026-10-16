//! Graph snapshot: our own binary format, like `en/src/store.rs`, but without serde — the graph already lives
//! in flat arrays. Building from XML takes tens of seconds, while the snapshot loads in a fraction of a second.
//!
//! File = header + body:
//! - header: magic `MOVA-ONT`, format version, enum sizes (Rel, Pos, LEXFILES, Core, Src),
//!   body length and checksum;
//! - body: arrays one after another, each — length (u64) and little-endian bytes.
//!
//! Any mismatch is a load error, not a silently different graph: a truncated file, a foreign version,
//! different enums in the code, a corrupted byte, extra bytes at the end.

use anyhow::{bail, Context, Result};

use crate::model::*;

const MAGIC: &[u8; 8] = b"MOVA-ONT";
/// Format version: change it whenever any graph array changes.
pub const FORMAT: u32 = 1;
const HEADER: usize = 8 + 4 + 2 * 5 + 8 + 8;

/// Checksum: multiply-rotate over 8-byte words (like FxHash, the same as in `en::hash`) plus the length.
pub fn checksum(b: &[u8]) -> u64 {
    const K: u64 = 0x517c_c1b7_2722_0a95;
    let mut h = 0u64;
    let mut add = |x: u64| h = (h.rotate_left(5) ^ x).wrapping_mul(K);
    for ch in b.chunks(8) {
        let mut w = [0u8; 8];
        w[..ch.len()].copy_from_slice(ch);
        add(u64::from_le_bytes(w));
    }
    add(b.len() as u64);
    h
}

fn shape() -> [u16; 5] {
    [Rel::N as u16, Pos::N as u16, LEXFILES.len() as u16, Core::ALL.len() as u16, Src::ALL.len() as u16]
}

#[derive(Default)]
struct W(Vec<u8>);

impl W {
    fn len(&mut self, n: usize) {
        self.0.extend_from_slice(&(n as u64).to_le_bytes());
    }
    fn u8s(&mut self, v: &[u8]) {
        self.len(v.len());
        self.0.extend_from_slice(v);
    }
    fn u32s(&mut self, v: &[u32]) {
        self.len(v.len());
        for x in v {
            self.0.extend_from_slice(&x.to_le_bytes());
        }
    }
    fn str(&mut self, s: &str) {
        self.u8s(s.as_bytes());
    }
}

struct R<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> R<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.p + n > self.b.len() {
            bail!("snapshot truncated: need {n} bytes at offset {}", self.p);
        }
        let s = &self.b[self.p..self.p + n];
        self.p += n;
        Ok(s)
    }
    fn len(&mut self) -> Result<usize> {
        let n = u64::from_le_bytes(self.take(8)?.try_into().expect("8 bytes"));
        usize::try_from(n).context("array length")
    }
    fn u8s(&mut self) -> Result<Vec<u8>> {
        let n = self.len()?;
        Ok(self.take(n)?.to_vec())
    }
    fn u32s(&mut self) -> Result<Vec<u32>> {
        let n = self.len()?;
        let b = self.take(n.checked_mul(4).context("length")?)?;
        Ok(b.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
    }
    fn string(&mut self) -> Result<String> {
        String::from_utf8(self.u8s()?).context("snapshot string is not UTF-8")
    }
}

fn enums<T>(v: Vec<u8>, f: impl Fn(u8) -> Option<T>, what: &str) -> Result<Vec<T>> {
    v.into_iter().map(|b| f(b).with_context(|| format!("{what}: number {b} out of the enum"))).collect()
}

pub fn pack(g: &Graph) -> Vec<u8> {
    let mut w = W::default();
    w.u8s(&g.src.iter().map(|s| *s as u8).collect::<Vec<_>>());
    w.u32s(&g.key);
    w.u8s(&g.pos.iter().map(|p| *p as u8).collect::<Vec<_>>());
    w.u8s(&g.lexfile);
    w.u8s(&g.flags);
    w.u32s(&g.ili);
    w.u32s(&g.gloss_off);
    w.str(&g.gloss);
    w.u32s(&g.name_off);
    w.str(&g.names);
    w.u32s(&g.mem_off);
    w.u32s(&g.mem);
    w.u32s(&g.word_off);
    w.str(&g.words);
    w.u32s(&g.sense_off);
    w.u32s(&g.senses);
    for (off, es) in [(&g.out_off, &g.out), (&g.in_off, &g.inn)] {
        w.u32s(off);
        w.u8s(&es.iter().map(|e| e.rel as u8).collect::<Vec<_>>());
        w.u32s(&es.iter().map(|e| e.to).collect::<Vec<_>>());
    }
    w.u32s(&g.lus.iter().map(|l| l.word).collect::<Vec<_>>());
    w.u8s(&g.lus.iter().map(|l| l.pos as u8).collect::<Vec<_>>());
    w.u32s(&g.lus.iter().map(|l| l.frame).collect::<Vec<_>>());
    w.u32s(&g.lus.iter().map(|l| l.id).collect::<Vec<_>>());
    let body = w.0;
    let mut out = Vec::with_capacity(HEADER + body.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&FORMAT.to_le_bytes());
    for n in shape() {
        out.extend_from_slice(&n.to_le_bytes());
    }
    out.extend_from_slice(&(body.len() as u64).to_le_bytes());
    out.extend_from_slice(&checksum(&body).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

pub fn unpack(b: &[u8]) -> Result<Graph> {
    if b.len() < HEADER || &b[..8] != MAGIC {
        bail!("not an onto snapshot (no MOVA-ONT magic)");
    }
    let format = u32::from_le_bytes(b[8..12].try_into().expect("4 bytes"));
    if format != FORMAT {
        bail!("format version {format}, but this build reads {FORMAT} — rebuild the snapshot (`onto build`)");
    }
    let got: Vec<u16> = (0..5).map(|i| u16::from_le_bytes([b[12 + 2 * i], b[13 + 2 * i]])).collect();
    if got != shape() {
        bail!("enum sizes Rel/Pos/LEXFILES/Core/Src {got:?}, but this build has {:?} — rebuild the snapshot", shape());
    }
    let len = u64::from_le_bytes(b[22..30].try_into().expect("8 bytes"));
    let sum = u64::from_le_bytes(b[30..38].try_into().expect("8 bytes"));
    let body = &b[HEADER..];
    if body.len() as u64 != len {
        bail!("body is {} bytes, but the header says {len} — the file is truncated or appended to", body.len());
    }
    if checksum(body) != sum {
        bail!("checksum mismatch — the snapshot is corrupted");
    }
    let mut r = R { b: body, p: 0 };
    let mut g = Graph {
        src: enums(r.u8s()?, Src::from_u8, "src")?,
        key: r.u32s()?,
        pos: enums(r.u8s()?, Pos::from_u8, "pos")?,
        lexfile: r.u8s()?,
        flags: r.u8s()?,
        ili: r.u32s()?,
        gloss_off: r.u32s()?,
        gloss: r.string()?,
        name_off: r.u32s()?,
        names: r.string()?,
        mem_off: r.u32s()?,
        mem: r.u32s()?,
        word_off: r.u32s()?,
        words: r.string()?,
        sense_off: r.u32s()?,
        senses: r.u32s()?,
        ..Default::default()
    };
    for inbound in [false, true] {
        let off = r.u32s()?;
        let rels = enums(r.u8s()?, Rel::from_u8, "rel")?;
        let to = r.u32s()?;
        if rels.len() != to.len() {
            bail!("edges: {} relations and {} targets", rels.len(), to.len());
        }
        let es = rels.into_iter().zip(to).map(|(rel, to)| Edge { rel, to }).collect();
        if inbound {
            (g.in_off, g.inn) = (off, es);
        } else {
            (g.out_off, g.out) = (off, es);
        }
    }
    let (w, p, f, id) = (r.u32s()?, enums(r.u8s()?, Pos::from_u8, "lu pos")?, r.u32s()?, r.u32s()?);
    if !(w.len() == p.len() && p.len() == f.len() && f.len() == id.len()) {
        bail!("lexical units: array lengths differ");
    }
    g.lus = (0..w.len()).map(|i| Lu { word: w[i], pos: p[i], frame: f[i], id: id[i] }).collect();
    if r.p != body.len() {
        bail!("extra bytes at the end of the snapshot: {}", body.len() - r.p);
    }
    check(&g)?;
    Ok(g)
}

/// Array consistency: lengths, offsets, edge targets, string bounds — so that a corrupted snapshot cannot cause a panic.
pub fn check(g: &Graph) -> Result<()> {
    let n = g.src.len();
    for (what, l) in [("key", g.key.len()), ("pos", g.pos.len()), ("lexfile", g.lexfile.len()), ("flags", g.flags.len()), ("ili", g.ili.len())] {
        if l != n {
            bail!("array {what}: {l} instead of {n}");
        }
    }
    let offs = |what: &str, off: &[u32], rows: usize, total: usize| -> Result<()> {
        if off.len() != rows + 1 || off.first() != Some(&0) || *off.last().unwrap_or(&0) as usize != total || off.windows(2).any(|w| w[0] > w[1]) {
            bail!("offsets {what} are inconsistent");
        }
        Ok(())
    };
    offs("gloss", &g.gloss_off, n, g.gloss.len())?;
    offs("names", &g.name_off, n, g.names.len())?;
    offs("mem", &g.mem_off, n, g.mem.len())?;
    let nw = g.word_off.len().saturating_sub(1);
    offs("words", &g.word_off, nw, g.words.len())?;
    offs("senses", &g.sense_off, nw, g.senses.len())?;
    offs("out", &g.out_off, n, g.out.len())?;
    offs("in", &g.in_off, n, g.inn.len())?;
    for pool in [(&g.gloss, &g.gloss_off), (&g.names, &g.name_off), (&g.words, &g.word_off)] {
        if pool.1.iter().any(|&o| !pool.0.is_char_boundary(o as usize)) {
            bail!("string offset in the middle of a character");
        }
    }
    if g.mem.iter().any(|&w| w as usize >= nw) || g.senses.iter().any(|&c| c as usize >= n) {
        bail!("reference to a word or concept out of bounds");
    }
    if g.out.iter().chain(g.inn.iter()).any(|e| e.to as usize >= n) || g.lus.iter().any(|l| l.word as usize >= nw || l.frame as usize >= n) {
        bail!("edge or lexical unit out of bounds");
    }
    Ok(())
}

pub fn save(g: &Graph, path: &std::path::Path) -> Result<usize> {
    let b = pack(g);
    std::fs::write(path, &b).with_context(|| format!("cannot write {}", path.display()))?;
    Ok(b.len())
}

pub fn load(path: &std::path::Path) -> Result<Graph> {
    let b = std::fs::read(path).with_context(|| format!("cannot read snapshot {} — run `onto build` first", path.display()))?;
    unpack(&b)
}
