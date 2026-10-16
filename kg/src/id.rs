//! Vertex identifiers: u64 = heap (high 32 bits) | number within the heap (low 32).
//! Heaps: [0, 2^26) — service, [2^26, 2^27) — local, [2^27, 2^32) — global.
//! Numbers within a heap are consecutive from 1 (0 means "none"); key → number lives in `keys/<heap>.tsv`
//! and is append-only, so ids are stable across runs.

use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct Id(pub u64);

impl Id {
    pub const fn new(heap: u32, local: u32) -> Self {
        Id(((heap as u64) << 32) | local as u64)
    }
    pub const fn heap(self) -> u32 {
        (self.0 >> 32) as u32
    }
    pub const fn local(self) -> u32 {
        self.0 as u32
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.heap(), self.local())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Service,
    Local,
    Global,
}

impl Kind {
    /// Inclusive range of heap numbers for this kind.
    fn range(self) -> (u32, u32) {
        match self {
            Kind::Service => (1, (1 << 26) - 1),
            Kind::Local => (1 << 26, (1 << 27) - 1),
            Kind::Global => (1 << 27, u32::MAX),
        }
    }
    fn name(self) -> &'static str {
        match self {
            Kind::Service => "service",
            Kind::Local => "local",
            Kind::Global => "global",
        }
    }
    fn parse(s: &str) -> Result<Self> {
        Ok(match s {
            "service" => Kind::Service,
            "local" => Kind::Local,
            "global" => Kind::Global,
            _ => bail!("unknown heap kind {s}"),
        })
    }
}

struct Keys {
    map: HashMap<String, u32>,
    names: Vec<String>,
    saved: usize,
}

pub struct Registry {
    dir: PathBuf,
    heaps: Vec<(u32, Kind, String)>,
    saved_heaps: usize,
    keys: HashMap<u32, Keys>,
}

impl Registry {
    pub fn open(dir: &Path) -> Result<Self> {
        fs::create_dir_all(dir.join("keys"))?;
        let mut heaps = Vec::new();
        let path = dir.join("heaps.tsv");
        if path.exists() {
            for line in BufReader::new(File::open(&path)?).lines() {
                let line = line?;
                let mut p = line.splitn(3, '\t');
                let (Some(h), Some(k), Some(n)) = (p.next(), p.next(), p.next()) else {
                    bail!("malformed heaps.tsv line: {line}");
                };
                heaps.push((h.parse()?, Kind::parse(k)?, n.to_string()));
            }
        }
        let saved_heaps = heaps.len();
        Ok(Registry { dir: dir.to_path_buf(), heaps, saved_heaps, keys: HashMap::new() })
    }

    /// Heap by name; a new heap gets the next free number in its kind's range.
    pub fn heap(&mut self, kind: Kind, name: &str) -> Result<u32> {
        if let Some(&(h, k, _)) = self.heaps.iter().find(|(_, _, n)| n == name) {
            if k != kind {
                bail!("heap {name} already exists as {}", k.name());
            }
            return Ok(h);
        }
        let (start, end) = kind.range();
        let next = self.heaps.iter().filter(|(_, k, _)| *k == kind).map(|(h, _, _)| h + 1).max().unwrap_or(start);
        if next > end {
            bail!("heaps of kind {} exhausted", kind.name());
        }
        self.heaps.push((next, kind, name.to_string()));
        Ok(next)
    }

    pub fn heap_name(&self, heap: u32) -> &str {
        self.heaps.iter().find(|(h, _, _)| *h == heap).map_or("?", |(_, _, n)| n.as_str())
    }

    /// Vertex by key within a heap; a new key gets the next number.
    pub fn id(&mut self, heap: u32, key: &str) -> Result<Id> {
        let keys = self.keys_of(heap)?;
        if let Some(&l) = keys.map.get(key) {
            return Ok(Id::new(heap, l));
        }
        if key.contains(['\t', '\n']) {
            bail!("key contains a tab or newline: {key:?}");
        }
        let l = u32::try_from(keys.names.len()).context("heap overflow")?;
        keys.map.insert(key.to_string(), l);
        keys.names.push(key.to_string());
        Ok(Id::new(heap, l))
    }

    /// Vertex by key without creating it.
    pub fn find(&mut self, heap: u32, key: &str) -> Result<Option<Id>> {
        Ok(self.keys_of(heap)?.map.get(key).map(|&l| Id::new(heap, l)))
    }

    /// Heap by name without creating it.
    pub fn heap_by_name(&self, name: &str) -> Option<u32> {
        self.heaps.iter().find(|(_, _, n)| n == name).map(|(h, _, _)| *h)
    }

    pub fn key(&mut self, id: Id) -> Result<String> {
        let keys = self.keys_of(id.heap())?;
        Ok(keys.names.get(id.local() as usize).cloned().unwrap_or_default())
    }

    fn keys_of(&mut self, heap: u32) -> Result<&mut Keys> {
        if !self.keys.contains_key(&heap) {
            let mut names = vec![String::new()];
            let path = self.dir.join("keys").join(format!("{heap}.tsv"));
            if path.exists() {
                for line in BufReader::new(File::open(&path)?).lines() {
                    let line = line?;
                    let (l, k) = line.split_once('\t').context("malformed keys line")?;
                    if l.parse::<usize>()? != names.len() {
                        bail!("keys of heap {heap} are not consecutive at {l}");
                    }
                    names.push(k.to_string());
                }
            }
            let map = names.iter().enumerate().skip(1).map(|(i, n)| (n.clone(), i as u32)).collect();
            let saved = names.len();
            self.keys.insert(heap, Keys { map, names, saved });
        }
        Ok(self.keys.get_mut(&heap).expect("just inserted"))
    }

    /// Appends new heaps and keys (append-only).
    pub fn save(&mut self) -> Result<()> {
        let mut f = OpenOptions::new().create(true).append(true).open(self.dir.join("heaps.tsv"))?;
        for (h, k, n) in &self.heaps[self.saved_heaps..] {
            writeln!(f, "{h}\t{}\t{n}", k.name())?;
        }
        self.saved_heaps = self.heaps.len();
        for (heap, keys) in &mut self.keys {
            if keys.saved == keys.names.len() {
                continue;
            }
            let path = self.dir.join("keys").join(format!("{heap}.tsv"));
            let mut f = std::io::BufWriter::new(OpenOptions::new().create(true).append(true).open(path)?);
            for (i, n) in keys.names.iter().enumerate().skip(keys.saved) {
                writeln!(f, "{i}\t{n}")?;
            }
            keys.saved = keys.names.len();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_parts() {
        let id = Id::new((1 << 27) + 5, 42);
        assert_eq!(id.heap(), (1 << 27) + 5);
        assert_eq!(id.local(), 42);
        assert_eq!(id.to_string(), "134217733:42");
    }
}
