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
    /// Names that heaps.tsv cannot frame exactly are rejected before lookup or change:
    /// LF splits the row and BufRead::lines strips a CR before LF. Tabs and other CRs
    /// round-trip, since the name is the third splitn field.
    pub fn heap(&mut self, kind: Kind, name: &str) -> Result<u32> {
        if name.contains('\n') || name.ends_with('\r') {
            bail!("heap name contains a newline or ends with a carriage return: {name:?}");
        }
        if let Some(&(h, k, _)) = self.heaps.iter().find(|(_, _, n)| n == name) {
            if k != kind {
                bail!("heap {name} already exists as {}", k.name());
            }
            return Ok(h);
        }
        let (start, end) = kind.range();
        let next = match self.heaps.iter().filter(|(_, k, _)| *k == kind).map(|(h, _, _)| *h).max() {
            Some(last) => last.checked_add(1).with_context(|| format!("heaps of kind {} exhausted", kind.name()))?,
            None => start,
        };
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
    /// Reject TSV separators before loading or changing registry state.
    pub fn id(&mut self, heap: u32, key: &str) -> Result<Id> {
        // A trailing CR would be stripped by BufRead::lines after save appends LF.
        if key.contains(['\t', '\n', '\r']) {
            bail!("key contains a tab, newline or carriage return: {key:?}");
        }
        let keys = self.keys_of(heap)?;
        if let Some(&l) = keys.map.get(key) {
            return Ok(Id::new(heap, l));
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

#[cfg(test)]
mod wave29 {
    use super::*;
    #[test]
    fn global_heap_exhaustion_preserves_state() {
        let dir = std::env::temp_dir().join(format!("kg-wave29-full-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("heaps.tsv"),
            format!("{}\tglobal\tlast\n", u32::MAX),
        )
        .unwrap();
        let mut r = Registry::open(&dir).unwrap();
        assert!(r.heap(Kind::Global, "overflow").is_err());
        assert_eq!(r.heap_by_name("overflow"), None);
        assert_eq!(r.heaps.len(), 1);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn registry_persistence_and_corruption() {
        let dir = std::env::temp_dir().join(format!("kg-wave29-id-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let mut r = Registry::open(&dir).unwrap();
        let h = r.heap(Kind::Global, "words").unwrap();
        assert_eq!(h, 1 << 27);
        assert!(r.heap(Kind::Local, "words").is_err());
        let a = r.id(h, "alpha").unwrap();
        assert_eq!(a.local(), 1);
        assert_eq!(r.id(h, "alpha").unwrap(), a);
        assert!(r.id(h, "bad\tkey").is_err());
        assert_eq!(r.find(h, "missing").unwrap(), None);
        assert_eq!(r.key(Id::new(h, 0)).unwrap(), "");
        r.save().unwrap();
        let before = fs::read(dir.join("keys").join(format!("{h}.tsv"))).unwrap();
        r.save().unwrap();
        assert_eq!(
            fs::read(dir.join("keys").join(format!("{h}.tsv"))).unwrap(),
            before
        );
        let mut reopened = Registry::open(&dir).unwrap();
        assert_eq!(reopened.find(h, "alpha").unwrap(), Some(a));
        assert_eq!(reopened.id(h, "beta").unwrap().local(), 2);
        fs::write(dir.join("keys/7.tsv"), "2\tbroken\n").unwrap();
        assert!(reopened.find(7, "broken").is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg(test)]
mod wave37 {
    use super::*;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("kg-wave37-{label}-{}", std::process::id()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn rejected_key_separators_preserve_registry_state() {
        let dir = TempDir::new("rejection");
        let mut registry = Registry::open(&dir.0).unwrap();
        let heap = registry.heap(Kind::Global, "words").unwrap();
        registry.save().unwrap();
        let heaps = registry.heaps.clone();
        let saved_heaps = registry.saved_heaps;
        let heap_bytes = fs::read(dir.0.join("heaps.tsv")).unwrap();
        let path = dir.0.join("keys").join(format!("{heap}.tsv"));
        let rejected = [
            "alpha\r",
            "\ralpha",
            "al\rpha",
            "\r",
            "alpha\r\n",
            "bad\tkey",
            "bad\nkey",
        ];
        for key in rejected {
            assert!(registry.id(heap, key).is_err(), "accepted {key:?}");
            assert!(registry.keys.is_empty(), "loaded keys for {key:?}");
            assert_eq!(registry.heaps, heaps);
            assert_eq!(registry.saved_heaps, saved_heaps);
            assert_eq!(fs::read(dir.0.join("heaps.tsv")).unwrap(), heap_bytes);
            assert!(!path.exists());
        }
        let first = registry.id(heap, "alpha").unwrap();
        assert_eq!(first.local(), 1);
        registry.save().unwrap();
        let names = registry.keys[&heap].names.clone();
        let map = registry.keys[&heap].map.clone();
        let saved = registry.keys[&heap].saved;
        let bytes = fs::read(&path).unwrap();
        for key in rejected {
            assert!(registry.id(heap, key).is_err(), "accepted {key:?}");
            assert_eq!(registry.keys[&heap].names, names);
            assert_eq!(registry.keys[&heap].map, map);
            assert_eq!(registry.keys[&heap].saved, saved);
            assert_eq!(registry.heaps, heaps);
            assert_eq!(registry.saved_heaps, saved_heaps);
        }
        registry.save().unwrap();
        assert_eq!(fs::read(path).unwrap(), bytes);
        assert_eq!(registry.id(heap, "alpha").unwrap(), first);
        assert_eq!(registry.id(heap, "beta").unwrap().local(), 2);
    }

    #[test]
    fn rejected_keys_precede_lazy_io_and_legacy_lookup() {
        let dir = TempDir::new("precedence");
        let mut registry = Registry::open(&dir.0).unwrap();
        let heap = registry.heap(Kind::Global, "words").unwrap();
        let path = dir.0.join("keys").join(format!("{heap}.tsv"));
        fs::write(&path, "2\tbroken\n").unwrap();
        for key in ["bad\rkey", "bad\tkey", "bad\nkey"] {
            let error = registry.id(heap, key).unwrap_err().to_string();
            assert!(error.starts_with("key contains"), "{error}");
            assert!(registry.keys.is_empty());
        }
        assert!(registry.id(heap, "valid").is_err());
        assert!(registry.keys.is_empty());
        let legacy_bytes = b"1\told\rinside\n2\told\tinside\n";
        fs::write(&path, legacy_bytes).unwrap();
        for (local, key) in [(1, "old\rinside"), (2, "old\tinside")] {
            assert_eq!(
                registry.find(heap, key).unwrap(),
                Some(Id::new(heap, local))
            );
            assert!(registry.id(heap, key).is_err());
            assert_eq!(registry.key(Id::new(heap, local)).unwrap(), key);
        }
        assert_eq!(registry.keys[&heap].names.len(), 3);
        assert_eq!(registry.keys[&heap].map.len(), 2);
        assert_eq!(registry.keys[&heap].saved, 3);
        registry.save().unwrap();
        assert_eq!(fs::read(path).unwrap(), legacy_bytes);
    }

    #[test]
    fn admitted_unicode_keys_reopen_with_exact_ids() {
        let dir = TempDir::new("unicode");
        let mut registry = Registry::open(&dir.0).unwrap();
        let heap = registry.heap(Kind::Global, "words").unwrap();
        let keys = [
            "alpha",
            "",
            "Знання 🐙",
            "café",
            "cafe\u{301}",
            "  padded  ",
            "line\u{2028}separator",
            "paragraph\u{2029}separator",
            "\0key",
        ];
        for (index, key) in keys.iter().enumerate() {
            assert_eq!(
                registry.id(heap, key).unwrap(),
                Id::new(heap, index as u32 + 1)
            );
        }
        registry.save().unwrap();
        let path = dir.0.join("keys").join(format!("{heap}.tsv"));
        let bytes = fs::read(&path).unwrap();
        drop(registry);
        let mut reopened = Registry::open(&dir.0).unwrap();
        assert_eq!(reopened.heap_by_name("words"), Some(heap));
        for (index, key) in keys.iter().enumerate() {
            let expected = Id::new(heap, index as u32 + 1);
            assert_eq!(reopened.find(heap, key).unwrap(), Some(expected));
            assert_eq!(reopened.id(heap, key).unwrap(), expected);
            assert_eq!(reopened.key(expected).unwrap(), *key);
        }
        reopened.save().unwrap();
        assert_eq!(fs::read(&path).unwrap(), bytes);
        let next = reopened.id(heap, "next").unwrap();
        assert_eq!(next.local(), keys.len() as u32 + 1);
        reopened.save().unwrap();
        drop(reopened);
        let mut reopened = Registry::open(&dir.0).unwrap();
        assert_eq!(reopened.find(heap, "next").unwrap(), Some(next));
        for (index, key) in keys.iter().enumerate() {
            assert_eq!(
                reopened.find(heap, key).unwrap(),
                Some(Id::new(heap, index as u32 + 1))
            );
        }
    }

    #[test]
    fn legacy_trailing_carriage_return_remains_decode_ambiguous() {
        let dir = TempDir::new("legacy-crlf");
        let mut registry = Registry::open(&dir.0).unwrap();
        let heap = registry.heap(Kind::Global, "words").unwrap();
        registry.save().unwrap();
        let path = dir.0.join("keys").join(format!("{heap}.tsv"));
        let legacy_bytes = b"1\talpha\r\n";
        fs::write(&path, legacy_bytes).unwrap();
        let mut reopened = Registry::open(&dir.0).unwrap();
        let id = Id::new(heap, 1);
        assert_eq!(reopened.find(heap, "alpha").unwrap(), Some(id));
        assert_eq!(reopened.find(heap, "alpha\r").unwrap(), None);
        assert_eq!(reopened.key(id).unwrap(), "alpha");
        assert!(reopened.id(heap, "alpha\r").is_err());
        assert_eq!(reopened.id(heap, "alpha").unwrap(), id);
        reopened.save().unwrap();
        assert_eq!(fs::read(path).unwrap(), legacy_bytes);
    }
}

#[cfg(test)]
mod wave38 {
    use super::*;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("kg-wave38-{label}-{}", std::process::id()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    const LOSSY: [&str; 6] = [
        "lf\ninside",
        "\n",
        "trail\r",
        "\r",
        "crlf\r\n",
        "x\n134217728\tglobal\tforged",
    ];

    #[test]
    fn lossy_heap_names_are_rejected_without_state_or_file_change() {
        let dir = TempDir::new("lossy");
        let mut registry = Registry::open(&dir.0).unwrap();
        let words = registry.heap(Kind::Global, "words").unwrap();
        registry.save().unwrap();
        let heaps = registry.heaps.clone();
        let bytes = fs::read(dir.0.join("heaps.tsv")).unwrap();
        for name in LOSSY {
            let error = registry.heap(Kind::Global, name).unwrap_err().to_string();
            assert!(error.starts_with("heap name contains"), "{name:?}: {error}");
            assert_eq!(registry.heaps, heaps);
            assert_eq!(registry.saved_heaps, heaps.len());
            assert_eq!(registry.heap_by_name(name), None);
        }
        registry.save().unwrap();
        assert_eq!(fs::read(dir.0.join("heaps.tsv")).unwrap(), bytes);
        let mut reopened = Registry::open(&dir.0).unwrap();
        assert_eq!(reopened.heaps, heaps);
        assert_eq!(reopened.heap(Kind::Global, "next").unwrap(), words + 1);
    }

    #[test]
    fn heap_name_admission_precedes_kind_conflict_and_exhaustion() {
        let dir = TempDir::new("precedence");
        fs::write(
            dir.0.join("heaps.tsv"),
            format!("{}\tglobal\tlast\n{}\tlocal\tplace\n", u32::MAX, 1u32 << 26),
        )
        .unwrap();
        let mut registry = Registry::open(&dir.0).unwrap();
        let heaps = registry.heaps.clone();
        // Exhausted kind: admission error, not exhaustion.
        let error = registry.heap(Kind::Global, "new\n").unwrap_err().to_string();
        assert!(error.starts_with("heap name contains"), "{error}");
        // Admitted name keeps the existing paths and messages.
        let error = registry.heap(Kind::Global, "fresh").unwrap_err().to_string();
        assert!(error.contains("exhausted"), "{error}");
        let error = registry.heap(Kind::Global, "place").unwrap_err().to_string();
        assert!(error.contains("already exists as local"), "{error}");
        assert_eq!(registry.heap(Kind::Local, "place").unwrap(), 1 << 26);
        assert_eq!(registry.heap(Kind::Global, "last").unwrap(), u32::MAX);
        assert_eq!(registry.heaps, heaps);
    }

    #[test]
    fn unusual_exact_heap_names_reopen_with_stable_numbers() {
        let dir = TempDir::new("exact");
        let mut registry = Registry::open(&dir.0).unwrap();
        let names = [
            "words",
            "tab\tinside",
            "two\ttabs\t",
            "cr\rinside",
            "\rleading",
            "",
            "Знання 🐙",
            "cafe\u{301}",
            "  padded  ",
            "line\u{2028}separator",
            "nul\0",
        ];
        let numbers: Vec<u32> = names
            .iter()
            .map(|n| registry.heap(Kind::Global, n).unwrap())
            .collect();
        assert_eq!(numbers, (0..names.len() as u32).map(|i| (1 << 27) + i).collect::<Vec<_>>());
        registry.save().unwrap();
        let bytes = fs::read(dir.0.join("heaps.tsv")).unwrap();
        drop(registry);
        let mut reopened = Registry::open(&dir.0).unwrap();
        for (name, number) in names.iter().zip(&numbers) {
            assert_eq!(reopened.heap_by_name(name), Some(*number), "{name:?}");
            assert_eq!(reopened.heap_name(*number), *name);
            assert_eq!(reopened.heap(Kind::Global, name).unwrap(), *number);
        }
        reopened.save().unwrap();
        assert_eq!(fs::read(dir.0.join("heaps.tsv")).unwrap(), bytes);
    }

    #[test]
    fn legacy_heap_files_decode_unchanged_and_ambiguous_forms_stay_rejected() {
        let dir = TempDir::new("legacy");
        // CRLF-formatted heap rows decode without the CR, as before.
        let legacy = b"134217728\tglobal\twords\r\n134217729\tglobal\ttab\tname\r\n";
        fs::write(dir.0.join("heaps.tsv"), legacy).unwrap();
        let mut registry = Registry::open(&dir.0).unwrap();
        assert_eq!(registry.heap_by_name("words"), Some(1 << 27));
        assert_eq!(registry.heap_by_name("tab\tname"), Some((1 << 27) + 1));
        assert_eq!(registry.heap(Kind::Global, "words").unwrap(), 1 << 27);
        // The CR-terminated form cannot claim a second number for the decoded name.
        assert!(registry.heap(Kind::Global, "words\r").is_err());
        registry.save().unwrap();
        assert_eq!(fs::read(dir.0.join("heaps.tsv")).unwrap(), legacy);
        // A historical LF-split row remains a hard open error, not a silent repair.
        fs::write(dir.0.join("heaps.tsv"), b"134217728\tglobal\tlf\ninside\n").unwrap();
        let error = Registry::open(&dir.0).err().unwrap().to_string();
        assert!(error.contains("malformed heaps.tsv line"), "{error}");
    }
}
