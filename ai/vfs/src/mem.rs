//! In-memory file system: a tree of immutable nodes under `Arc`, changes via path copying.
//!
//! - **O(1) snapshot:** `snapshot()` takes the root `Arc` and the counters; `restore()` puts them back. Shared nodes are not
//!   copied; the first write after a snapshot copies only the nodes on the path from the root to the changed one (and the file
//!   contents — only when appending, not when overwriting).
//! - **The root is the boundary:** `..` above the root, a symlink leading above the root — an `ENOTCAPABLE` error with
//!   an explanation; absolute paths are inside this tree (the real FS is not visible from here at all).
//! - **Limits** are checked before the change: a violation does not change state.
//! - **The clock is virtual:** every change advances it by `TICK_NS`; time goes into metadata and the journal.

use crate::error::{Errno, FsError};
use crate::journal::{Journal, Op};
use crate::path;
use crate::{DirEntry, FileType, Limits, Stat, Transfer, Usage};
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

/// Start of virtual time: 2026-09-26T00:00:00Z in nanoseconds.
pub const EPOCH_NS: u64 = 1_790_380_800_000_000_000;
/// Virtual clock step per change — 1 µs.
pub const TICK_NS: u64 = 1_000;
/// Max symlink hops in one path (like MAXSYMLINKS in Linux).
const MAX_HOPS: usize = 40;

#[derive(Clone, Debug)]
struct Meta {
    ino: u64,
    mode: u32,
    mtime: u64,
    ctime: u64,
}

#[derive(Clone, Debug)]
enum Kind {
    File(Arc<Vec<u8>>),
    /// Keys are `Arc<str>`: copying a directory on write is only reference counting, no memory allocation.
    Dir(BTreeMap<Arc<str>, Arc<Node>>),
    Link(String),
}

#[derive(Clone, Debug)]
struct Node {
    kind: Kind,
    meta: Meta,
}

impl Node {
    fn ftype(&self) -> FileType {
        match self.kind {
            Kind::File(_) => FileType::File,
            Kind::Dir(_) => FileType::Dir,
            Kind::Link(_) => FileType::Symlink,
        }
    }
    fn size(&self) -> u64 {
        match &self.kind {
            Kind::File(d) => d.len() as u64,
            Kind::Dir(e) => e.len() as u64,
            Kind::Link(t) => t.len() as u64,
        }
    }
    fn stat(&self) -> Stat {
        Stat {
            kind: self.ftype(),
            size: self.size(),
            ino: self.meta.ino,
            mode: self.meta.mode,
            mtime: self.meta.mtime,
            ctime: self.meta.ctime,
            nlink: 1,
        }
    }
    /// How many content bytes and nodes are in the subtree (including the node itself).
    fn weight(&self) -> (u64, u64) {
        match &self.kind {
            Kind::File(d) => (d.len() as u64, 1),
            Kind::Link(_) => (0, 1),
            Kind::Dir(e) => e.values().fold((0, 1), |(b, n), c| {
                let (cb, cn) = c.weight();
                (b + cb, n + cn)
            }),
        }
    }
}

/// State snapshot. Cheap (O(1)), immutable, `Send + Sync`: one snapshot can be restored in many
/// sandboxes in parallel — "a clean machine for every lesson".
#[derive(Clone, Debug)]
pub struct Snapshot {
    root: Arc<Node>,
    cwd: Vec<String>,
    clock: u64,
    next_ino: u64,
    usage: Usage,
    tmp: u64,
}

impl Snapshot {
    pub fn usage(&self) -> Usage {
        self.usage
    }
}

/// Deliberate faults for negative controls: a gate that cannot show red is not a gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// `..` above the root silently stays at the root (as in many file systems) — instead of a root-escape error.
    ClampDotDot,
    /// Limits are not checked.
    NoLimits,
    /// `restore` does not roll back the virtual clock.
    RestoreForgetsClock,
    /// `DirFs`: no canonical path check (links pointing outside pass through).
    NoConfine,
}

/// How to change file contents.
enum Edit<'a> {
    Replace(&'a [u8]),
    Append(&'a [u8]),
    At(u64, &'a [u8]),
    Truncate(u64),
    Touch,
}

/// In-memory file system.
pub struct Vfs {
    root: Arc<Node>,
    cwd: Vec<String>,
    clock: u64,
    next_ino: u64,
    usage: Usage,
    tmp: u64,
    pub limits: Limits,
    pub journal: Journal,
    /// Negative control for gates: a deliberate fault (see [`Fault`]). In production — `None`.
    pub fault: Option<Fault>,
}

impl Default for Vfs {
    fn default() -> Vfs {
        Vfs::new()
    }
}

impl Vfs {
    /// Root only.
    pub fn empty() -> Vfs {
        let meta = Meta { ino: 1, mode: 0o755, mtime: EPOCH_NS, ctime: EPOCH_NS };
        Vfs {
            root: Arc::new(Node { kind: Kind::Dir(BTreeMap::new()), meta }),
            cwd: Vec::new(),
            clock: EPOCH_NS,
            next_ino: 2,
            usage: Usage::default(),
            tmp: 0,
            limits: Limits::default(),
            journal: Journal::default(),
            fault: None,
        }
    }

    /// Root and `/tmp` (0o777) — as on a clean machine. The journal is empty.
    pub fn new() -> Vfs {
        let mut v = Vfs::empty();
        v.mkdir("/tmp").expect("vfs: /tmp");
        v.chmod("/tmp", 0o777).expect("vfs: chmod /tmp");
        v.journal.clear();
        v
    }

    pub fn with_limits(mut self, limits: Limits) -> Vfs {
        self.limits = limits;
        self
    }

    // ---------- clock, counters ----------

    /// Virtual time, ns since 1970-01-01.
    pub fn now(&self) -> u64 {
        self.clock
    }
    /// Advance the virtual clock (WASI `clock_time_get`, `poll_oneoff` with sleep).
    pub fn advance(&mut self, ns: u64) {
        self.clock = self.clock.saturating_add(ns);
    }
    fn tick(&mut self) -> u64 {
        self.clock = self.clock.saturating_add(TICK_NS);
        self.clock
    }
    fn alloc_ino(&mut self) -> u64 {
        let i = self.next_ino;
        self.next_ino += 1;
        i
    }
    pub fn usage(&self) -> Usage {
        self.usage
    }
    pub fn cwd(&self) -> String {
        path::join(&self.cwd)
    }

    fn log<T>(&mut self, op: Op, p: &str, bytes: u64, r: Result<T, FsError>) -> Result<T, FsError> {
        self.journal.record(self.clock, op, p, bytes, r.as_ref().err());
        r
    }

    // ---------- path resolution ----------

    fn lookup(&self, comps: &[String]) -> Option<&Arc<Node>> {
        let mut cur = &self.root;
        for c in comps {
            match &cur.kind {
                Kind::Dir(e) => cur = e.get(c.as_str())?,
                _ => return None,
            }
        }
        Some(cur)
    }

    /// Canonical path components: links in the middle are resolved; the last one — if `follow_last`. The last
    /// component may be absent (for creation). `..` above the root (including via a link) — root escape.
    fn resolve(&self, p: &str, follow_last: bool) -> Result<Vec<String>, FsError> {
        path::check(p)?;
        let mut stack: Vec<String> = Vec::new();
        let mut nodes: Vec<&Arc<Node>> = vec![&self.root];
        if !p.starts_with('/') {
            for c in &self.cwd {
                let Kind::Dir(e) = &nodes[nodes.len() - 1].kind else { return Err(FsError::new(Errno::NoEnt)) };
                let n = e.get(c.as_str()).ok_or_else(|| FsError::new(Errno::NoEnt))?;
                stack.push(c.clone());
                nodes.push(n);
            }
        }
        let mut pending: VecDeque<String> = path::parts(p).map(String::from).collect();
        let mut hops = 0;
        while let Some(c) = pending.pop_front() {
            if c == ".." {
                if stack.is_empty() {
                    if self.fault == Some(Fault::ClampDotDot) {
                        continue;
                    }
                    return Err(FsError::escape(p));
                }
                stack.pop();
                nodes.pop();
                continue;
            }
            path::check_name(&c)?;
            let Kind::Dir(entries) = &nodes[nodes.len() - 1].kind else { return Err(FsError::new(Errno::NotDir)) };
            let last = pending.is_empty();
            match entries.get(c.as_str()) {
                None if last => {
                    stack.push(c);
                    return Ok(stack);
                }
                None => return Err(FsError::new(Errno::NoEnt)),
                Some(child) => {
                    if let Kind::Link(t) = &child.kind {
                        if !last || follow_last {
                            hops += 1;
                            if hops > MAX_HOPS {
                                return Err(FsError::new(Errno::Loop));
                            }
                            if t.is_empty() {
                                return Err(FsError::new(Errno::NoEnt));
                            }
                            if t.starts_with('/') {
                                stack.clear();
                                nodes.truncate(1);
                            }
                            let tp: Vec<&str> = path::parts(t).collect();
                            for x in tp.into_iter().rev() {
                                pending.push_front(x.to_string());
                            }
                            continue;
                        }
                    }
                    stack.push(c);
                    nodes.push(child);
                }
            }
        }
        Ok(stack)
    }

    /// Canonical absolute path (for the WASI host and checks).
    pub fn resolve_path(&self, p: &str, follow_last: bool) -> Result<String, FsError> {
        self.resolve(p, follow_last).map(|c| path::join(&c))
    }

    /// Mutable access to a directory with path copying: every shared node on the path is copied (shallowly).
    fn dir_mut(&mut self, comps: &[String]) -> Result<(&mut BTreeMap<Arc<str>, Arc<Node>>, &mut Meta), FsError> {
        let mut cur: &mut Node = Arc::make_mut(&mut self.root);
        for c in comps {
            let Kind::Dir(entries) = &mut cur.kind else { return Err(FsError::new(Errno::NotDir)) };
            let child = entries.get_mut(c.as_str()).ok_or_else(|| FsError::new(Errno::NoEnt))?;
            cur = Arc::make_mut(child);
        }
        let Node { kind, meta } = cur;
        match kind {
            Kind::Dir(e) => Ok((e, meta)),
            _ => Err(FsError::new(Errno::NotDir)),
        }
    }

    fn node_mut(&mut self, comps: &[String]) -> Result<&mut Node, FsError> {
        let Some((name, parent)) = comps.split_last() else { return Ok(Arc::make_mut(&mut self.root)) };
        let (entries, _) = self.dir_mut(parent)?;
        let child = entries.get_mut(name.as_str()).ok_or_else(|| FsError::new(Errno::NoEnt))?;
        Ok(Arc::make_mut(child))
    }

    /// The parent directory exists, is a directory and is writable.
    fn check_parent(&self, parent: &[String]) -> Result<(), FsError> {
        let n = self.lookup(parent).ok_or_else(|| FsError::new(Errno::NoEnt))?;
        match n.kind {
            Kind::Dir(_) if n.meta.mode & 0o200 == 0 => Err(FsError::new(Errno::Acces)),
            Kind::Dir(_) => Ok(()),
            _ => Err(FsError::new(Errno::NotDir)),
        }
    }

    fn check_new_node(&self) -> Result<(), FsError> {
        let n = self.usage.files + 1;
        if n > self.limits.max_files && self.fault != Some(Fault::NoLimits) {
            return Err(FsError::with(
                Errno::DQuot,
                format!("vfs limit: {n} files would exceed max_files = {}", self.limits.max_files),
            ));
        }
        Ok(())
    }

    // ---------- files ----------

    fn edit_file(&mut self, p: &str, edit: Edit<'_>, create: bool) -> Result<(), FsError> {
        if p.ends_with('/') {
            return Err(FsError::new(Errno::IsDir));
        }
        let comps = self.resolve(p, true)?;
        let Some((name, parent)) = comps.split_last() else { return Err(FsError::new(Errno::IsDir)) };
        let old = match self.lookup(&comps) {
            Some(n) => match &n.kind {
                Kind::Dir(_) => return Err(FsError::new(Errno::IsDir)),
                Kind::Link(_) => return Err(FsError::new(Errno::Loop)),
                Kind::File(d) => {
                    if n.meta.mode & 0o200 == 0 && !matches!(edit, Edit::Touch) {
                        return Err(FsError::new(Errno::Acces));
                    }
                    Some(d.len() as u64)
                }
            },
            None => {
                if !create {
                    return Err(FsError::new(Errno::NoEnt));
                }
                self.check_parent(parent)?;
                self.check_new_node()?;
                None
            }
        };
        let old_size = old.unwrap_or(0);
        let new_size = match &edit {
            Edit::Replace(d) => Some(d.len() as u64),
            Edit::Append(d) => old_size.checked_add(d.len() as u64),
            Edit::At(off, d) => off.checked_add(d.len() as u64).map(|end| end.max(old_size)),
            Edit::Truncate(n) => Some(*n),
            Edit::Touch => Some(old_size),
        }
        .ok_or_else(|| FsError::new(Errno::FBig))?;
        let lim = if self.fault == Some(Fault::NoLimits) { Limits::UNLIMITED } else { self.limits };
        if new_size > lim.max_file_size {
            return Err(FsError::with(
                Errno::FBig,
                format!("vfs limit: file '{p}' would be {new_size} bytes, max_file_size = {}", self.limits.max_file_size),
            ));
        }
        let total = self.usage.bytes - old_size + new_size;
        if new_size > old_size && total > lim.max_total_bytes {
            return Err(FsError::with(
                Errno::NoSpc,
                format!("vfs limit: total size would be {total} bytes, max_total_bytes = {}", self.limits.max_total_bytes),
            ));
        }
        let now = self.tick();
        let ino = if old.is_none() { self.alloc_ino() } else { 0 };
        let name = name.clone();
        let parent = parent.to_vec();
        let (entries, pmeta) = self.dir_mut(&parent)?;
        match entries.get_mut(name.as_str()) {
            None => {
                let data = match edit {
                    Edit::Replace(d) | Edit::Append(d) => d.to_vec(),
                    Edit::At(off, d) => {
                        let mut v = vec![0u8; off as usize];
                        v.extend_from_slice(d);
                        v
                    }
                    Edit::Truncate(n) => vec![0u8; n as usize],
                    Edit::Touch => Vec::new(),
                };
                let meta = Meta { ino, mode: 0o644, mtime: now, ctime: now };
                entries.insert(Arc::from(name.as_str()), Arc::new(Node { kind: Kind::File(Arc::new(data)), meta }));
                pmeta.mtime = now;
                pmeta.ctime = now;
                self.usage.files += 1;
            }
            Some(child) => {
                let touch = matches!(edit, Edit::Touch);
                let node = Arc::make_mut(child);
                let Kind::File(d) = &mut node.kind else { return Err(FsError::new(Errno::IsDir)) };
                match edit {
                    Edit::Replace(x) => *d = Arc::new(x.to_vec()),
                    Edit::Append(x) => Arc::make_mut(d).extend_from_slice(x),
                    Edit::At(off, x) => {
                        let v = Arc::make_mut(d);
                        let (off, end) = (off as usize, off as usize + x.len());
                        if v.len() < end {
                            v.resize(end, 0);
                        }
                        v[off..end].copy_from_slice(x);
                    }
                    Edit::Truncate(n) => Arc::make_mut(d).resize(n as usize, 0),
                    Edit::Touch => {}
                }
                if !touch {
                    node.meta.mtime = now;
                    node.meta.ctime = now;
                }
            }
        }
        self.usage.bytes = total;
        Ok(())
    }

    /// Write a whole file (create or overwrite).
    pub fn write(&mut self, p: &str, data: &[u8]) -> Result<(), FsError> {
        let r = self.edit_file(p, Edit::Replace(data), true);
        self.log(Op::Write, p, data.len() as u64, r)
    }
    /// Append to the end (create if missing).
    pub fn append(&mut self, p: &str, data: &[u8]) -> Result<(), FsError> {
        let r = self.edit_file(p, Edit::Append(data), true);
        self.log(Op::Append, p, data.len() as u64, r)
    }
    /// Write at a position (the hole between the end and the position is zero-filled). The file must exist.
    pub fn write_at(&mut self, p: &str, offset: u64, data: &[u8]) -> Result<(), FsError> {
        let r = self.edit_file(p, Edit::At(offset, data), false);
        self.log(Op::WriteAt, p, data.len() as u64, r)
    }
    /// Truncate or extend with zeros. The file must exist.
    pub fn truncate(&mut self, p: &str, len: u64) -> Result<(), FsError> {
        let r = self.edit_file(p, Edit::Truncate(len), false);
        self.log(Op::Truncate, p, len, r)
    }
    /// Create an empty file if missing (`O_CREAT`); `excl` — error if it already exists (`O_EXCL`).
    pub fn create(&mut self, p: &str, excl: bool) -> Result<(), FsError> {
        let r = if excl && self.resolve(p, false).ok().and_then(|c| self.lookup(&c).map(|_| ())).is_some() {
            Err(FsError::new(Errno::Exist))
        } else {
            self.edit_file(p, Edit::Touch, true)
        };
        self.log(Op::Create, p, 0, r)
    }

    fn file_data(&self, p: &str) -> Result<Arc<Vec<u8>>, FsError> {
        let comps = self.resolve(p, true)?;
        let n = self.lookup(&comps).ok_or_else(|| FsError::new(Errno::NoEnt))?;
        match &n.kind {
            Kind::File(_) if p.ends_with('/') => Err(FsError::new(Errno::NotDir)),
            Kind::File(_) if n.meta.mode & 0o400 == 0 => Err(FsError::new(Errno::Acces)),
            Kind::File(d) => Ok(d.clone()),
            Kind::Dir(_) => Err(FsError::new(Errno::IsDir)),
            Kind::Link(_) => Err(FsError::new(Errno::Loop)),
        }
    }

    /// Read a whole file (a copy).
    pub fn read(&mut self, p: &str) -> Result<Vec<u8>, FsError> {
        let r = self.file_data(p).map(|d| d.to_vec());
        let n = r.as_ref().map_or(0, |d| d.len() as u64);
        self.log(Op::Read, p, n, r)
    }
    /// Read without copying: a shared buffer (lives as long as it is held, even after the file changes).
    pub fn read_shared(&mut self, p: &str) -> Result<Arc<Vec<u8>>, FsError> {
        let r = self.file_data(p);
        let n = r.as_ref().map_or(0, |d| d.len() as u64);
        self.log(Op::Read, p, n, r)
    }
    /// Read up to `len` bytes from position `offset` (past the end — empty).
    pub fn read_at(&mut self, p: &str, offset: u64, len: usize) -> Result<Vec<u8>, FsError> {
        let r = self.file_data(p).map(|d| {
            let start = (offset.min(d.len() as u64)) as usize;
            let end = start.saturating_add(len).min(d.len());
            d[start..end].to_vec()
        });
        let n = r.as_ref().map_or(0, |d| d.len() as u64);
        self.log(Op::Read, p, n, r)
    }

    // ---------- metadata, directories ----------

    fn stat_impl(&self, p: &str, follow: bool) -> Result<Stat, FsError> {
        let comps = self.resolve(p, follow)?;
        let n = self.lookup(&comps).ok_or_else(|| FsError::new(Errno::NoEnt))?;
        if p.ends_with('/') && !matches!(n.kind, Kind::Dir(_)) {
            return Err(FsError::new(Errno::NotDir));
        }
        Ok(n.stat())
    }
    /// Metadata, following links.
    pub fn stat(&mut self, p: &str) -> Result<Stat, FsError> {
        let r = self.stat_impl(p, true);
        self.log(Op::Stat, p, 0, r)
    }
    /// Metadata of the link itself.
    pub fn lstat(&mut self, p: &str) -> Result<Stat, FsError> {
        let r = self.stat_impl(p, false);
        self.log(Op::Stat, p, 0, r)
    }
    /// Metadata without journaling (for the host and checks); `follow` — resolve the last link.
    pub fn peek(&self, p: &str, follow: bool) -> Option<Stat> {
        self.stat_impl(p, follow).ok()
    }

    /// Directory contents by name (byte order, deterministic), without `.` and `..`.
    pub fn read_dir(&mut self, p: &str) -> Result<Vec<DirEntry>, FsError> {
        let r = (|| {
            let comps = self.resolve(p, true)?;
            let n = self.lookup(&comps).ok_or_else(|| FsError::new(Errno::NoEnt))?;
            match &n.kind {
                Kind::Dir(_) if n.meta.mode & 0o400 == 0 => Err(FsError::new(Errno::Acces)),
                Kind::Dir(e) => {
                    Ok(e.iter().map(|(name, c)| DirEntry { name: name.to_string(), kind: c.ftype(), ino: c.meta.ino }).collect())
                }
                _ => Err(FsError::new(Errno::NotDir)),
            }
        })();
        let n = r.as_ref().map_or(0, |v: &Vec<DirEntry>| v.len() as u64);
        self.log(Op::ReadDir, p, n, r)
    }

    fn mkdir_impl(&mut self, p: &str) -> Result<(), FsError> {
        let comps = self.resolve(p, false)?;
        let Some((name, parent)) = comps.split_last() else { return Err(FsError::new(Errno::Exist)) };
        if self.lookup(&comps).is_some() {
            return Err(FsError::new(Errno::Exist));
        }
        self.check_parent(parent)?;
        self.check_new_node()?;
        let now = self.tick();
        let ino = self.alloc_ino();
        let name = name.clone();
        let parent = parent.to_vec();
        let (entries, pmeta) = self.dir_mut(&parent)?;
        let meta = Meta { ino, mode: 0o755, mtime: now, ctime: now };
        entries.insert(Arc::from(name.as_str()), Arc::new(Node { kind: Kind::Dir(BTreeMap::new()), meta }));
        pmeta.mtime = now;
        pmeta.ctime = now;
        self.usage.files += 1;
        Ok(())
    }
    /// Create a directory (the parent must exist).
    pub fn mkdir(&mut self, p: &str) -> Result<(), FsError> {
        let r = self.mkdir_impl(p);
        self.log(Op::Mkdir, p, 0, r)
    }
    /// Create a directory with all parents (existing directories are not an error).
    pub fn mkdir_all(&mut self, p: &str) -> Result<(), FsError> {
        path::check(p)?;
        let mut prefix = if p.starts_with('/') { String::from("/") } else { String::new() };
        for c in p.split('/').filter(|c| !c.is_empty()) {
            if !prefix.is_empty() && !prefix.ends_with('/') {
                prefix.push('/');
            }
            prefix.push_str(c);
            match self.stat_impl(&prefix, true) {
                Ok(s) if s.kind == FileType::Dir => continue,
                Ok(_) => return self.log(Op::Mkdir, p, 0, Err(FsError::new(Errno::NotDir))),
                Err(e) if e.errno == Errno::NoEnt => {
                    let pre = prefix.clone();
                    self.mkdir(&pre)?;
                }
                Err(e) => return self.log(Op::Mkdir, p, 0, Err(e)),
            }
        }
        Ok(())
    }

    fn remove_impl(&mut self, p: &str, want_dir: bool, recursive: bool) -> Result<(), FsError> {
        let comps = self.resolve(p, false)?;
        let Some((name, parent)) = comps.split_last() else {
            return Err(FsError::with(Errno::Busy, "cannot remove the root"));
        };
        let n = self.lookup(&comps).ok_or_else(|| FsError::new(Errno::NoEnt))?;
        let (bytes, files) = match (&n.kind, want_dir) {
            (Kind::Dir(_), false) => return Err(FsError::new(Errno::IsDir)),
            (Kind::Dir(e), true) if !recursive && !e.is_empty() => return Err(FsError::new(Errno::NotEmpty)),
            (Kind::Dir(_), true) => n.weight(),
            (_, true) => return Err(FsError::new(Errno::NotDir)),
            (_, false) => n.weight(),
        };
        self.check_parent(parent)?;
        let now = self.tick();
        let name = name.clone();
        let parent = parent.to_vec();
        let (entries, pmeta) = self.dir_mut(&parent)?;
        entries.remove(name.as_str());
        pmeta.mtime = now;
        pmeta.ctime = now;
        self.usage.bytes -= bytes;
        self.usage.files -= files;
        Ok(())
    }
    /// Remove a file or link (not a directory).
    pub fn remove_file(&mut self, p: &str) -> Result<(), FsError> {
        let r = self.remove_impl(p, false, false);
        self.log(Op::RemoveFile, p, 0, r)
    }
    /// Remove an empty directory.
    pub fn remove_dir(&mut self, p: &str) -> Result<(), FsError> {
        let r = self.remove_impl(p, true, false);
        self.log(Op::RemoveDir, p, 0, r)
    }
    /// Remove a directory with all its contents.
    pub fn remove_dir_all(&mut self, p: &str) -> Result<(), FsError> {
        let r = self.remove_impl(p, true, true);
        self.log(Op::RemoveAll, p, 0, r)
    }

    fn rename_impl(&mut self, from: &str, to: &str) -> Result<(), FsError> {
        let fc = self.resolve(from, false)?;
        let tc = self.resolve(to, false)?;
        let (Some((fname, fparent)), Some((tname, tparent))) = (fc.split_last(), tc.split_last()) else {
            return Err(FsError::with(Errno::Busy, "cannot rename the root"));
        };
        let src = self.lookup(&fc).ok_or_else(|| FsError::new(Errno::NoEnt))?.clone();
        if fc == tc {
            return Ok(());
        }
        let src_dir = matches!(src.kind, Kind::Dir(_));
        if src_dir && tc.len() > fc.len() && tc.starts_with(&fc) {
            return Err(FsError::with(Errno::Inval, format!("cannot move '{from}' into itself")));
        }
        let (freed_bytes, freed_files) = match self.lookup(&tc) {
            None => (0, 0),
            Some(d) => match (&src.kind, &d.kind) {
                (Kind::Dir(_), Kind::Dir(e)) if !e.is_empty() => return Err(FsError::new(Errno::NotEmpty)),
                (Kind::Dir(_), Kind::Dir(_)) => (0, 1),
                (Kind::Dir(_), _) => return Err(FsError::new(Errno::NotDir)),
                (_, Kind::Dir(_)) => return Err(FsError::new(Errno::IsDir)),
                _ => d.weight(),
            },
        };
        self.check_parent(fparent)?;
        self.check_parent(tparent)?;
        let now = self.tick();
        let (fname, fparent, tname, tparent) = (fname.clone(), fparent.to_vec(), tname.clone(), tparent.to_vec());
        {
            let (e, m) = self.dir_mut(&fparent)?;
            e.remove(fname.as_str());
            m.mtime = now;
            m.ctime = now;
        }
        {
            let (e, m) = self.dir_mut(&tparent)?;
            e.insert(Arc::from(tname.as_str()), src);
            m.mtime = now;
            m.ctime = now;
        }
        self.usage.bytes -= freed_bytes;
        self.usage.files -= freed_files;
        Ok(())
    }
    /// Rename or move (replacing an existing one — per POSIX `rename` rules).
    pub fn rename(&mut self, from: &str, to: &str) -> Result<(), FsError> {
        let r = self.rename_impl(from, to);
        self.log(Op::Rename, &format!("{from} -> {to}"), 0, r)
    }

    fn symlink_impl(&mut self, target: &str, link: &str) -> Result<(), FsError> {
        path::check(target)?;
        let comps = self.resolve(link, false)?;
        let Some((name, parent)) = comps.split_last() else { return Err(FsError::new(Errno::Exist)) };
        if self.lookup(&comps).is_some() {
            return Err(FsError::new(Errno::Exist));
        }
        self.check_parent(parent)?;
        self.check_new_node()?;
        let now = self.tick();
        let ino = self.alloc_ino();
        let (name, parent) = (name.clone(), parent.to_vec());
        let (entries, pmeta) = self.dir_mut(&parent)?;
        let meta = Meta { ino, mode: 0o777, mtime: now, ctime: now };
        entries.insert(Arc::from(name.as_str()), Arc::new(Node { kind: Kind::Link(target.to_string()), meta }));
        pmeta.mtime = now;
        pmeta.ctime = now;
        self.usage.files += 1;
        Ok(())
    }
    /// Symlink `link → target`. The target is any text; it is resolved only inside this tree,
    /// and an attempt to go above the root is an error at access time.
    pub fn symlink(&mut self, target: &str, link: &str) -> Result<(), FsError> {
        let r = self.symlink_impl(target, link);
        self.log(Op::Symlink, &format!("{link} -> {target}"), 0, r)
    }
    pub fn read_link(&mut self, p: &str) -> Result<String, FsError> {
        let r = (|| {
            let comps = self.resolve(p, false)?;
            match &self.lookup(&comps).ok_or_else(|| FsError::new(Errno::NoEnt))?.kind {
                Kind::Link(t) => Ok(t.clone()),
                _ => Err(FsError::new(Errno::Inval)),
            }
        })();
        self.log(Op::ReadLink, p, 0, r)
    }

    pub fn chmod(&mut self, p: &str, mode: u32) -> Result<(), FsError> {
        let r = (|| {
            let comps = self.resolve(p, true)?;
            if self.lookup(&comps).is_none() {
                return Err(FsError::new(Errno::NoEnt));
            }
            let now = self.tick();
            let n = self.node_mut(&comps)?;
            n.meta.mode = mode & 0o7777;
            n.meta.ctime = now;
            Ok(())
        })();
        self.log(Op::Chmod, p, 0, r)
    }
    /// Set the modification time (ns since 1970-01-01).
    pub fn set_mtime(&mut self, p: &str, mtime: u64) -> Result<(), FsError> {
        let r = (|| {
            let comps = self.resolve(p, true)?;
            if self.lookup(&comps).is_none() {
                return Err(FsError::new(Errno::NoEnt));
            }
            let now = self.tick();
            let n = self.node_mut(&comps)?;
            n.meta.mtime = mtime;
            n.meta.ctime = now;
            Ok(())
        })();
        self.log(Op::SetTime, p, 0, r)
    }
    pub fn chdir(&mut self, p: &str) -> Result<(), FsError> {
        let r = (|| {
            let comps = self.resolve(p, true)?;
            match self.lookup(&comps).map(|n| n.ftype()) {
                Some(FileType::Dir) => {
                    self.cwd = comps;
                    Ok(())
                }
                Some(_) => Err(FsError::new(Errno::NotDir)),
                None => Err(FsError::new(Errno::NoEnt)),
            }
        })();
        self.log(Op::Chdir, p, 0, r)
    }
    /// Owner permissions: read (0o400) and write (0o200) — for `path_open` in the WASI host.
    pub fn check_access(&self, p: &str, read: bool, write: bool) -> Result<(), FsError> {
        let s = self.stat_impl(p, true)?;
        if (read && s.mode & 0o400 == 0) || (write && s.mode & 0o200 == 0) {
            return Err(FsError::new(Errno::Acces));
        }
        Ok(())
    }

    /// New temporary file name `/tmp/<prefix><k>`: deterministic, not taken.
    pub fn temp_name(&mut self, prefix: &str) -> String {
        loop {
            self.tmp += 1;
            let p = format!("/tmp/{prefix}{}", self.tmp);
            if self.stat_impl(&p, false).is_err() {
                return p;
            }
        }
    }

    // ---------- snapshots ----------

    /// O(1) snapshot.
    pub fn snapshot(&mut self) -> Snapshot {
        self.journal.record(self.clock, Op::Snapshot, "/", self.usage.bytes, None);
        Snapshot {
            root: self.root.clone(),
            cwd: self.cwd.clone(),
            clock: self.clock,
            next_ino: self.next_ino,
            usage: self.usage,
            tmp: self.tmp,
        }
    }
    /// Restore a snapshot in O(1): the state (tree, clock, counters) is byte for byte the same as at snapshot time.
    /// The journal is history, not state: it is not rolled back, it gets a `restore` entry instead.
    pub fn restore(&mut self, s: &Snapshot) {
        self.root = s.root.clone();
        self.cwd = s.cwd.clone();
        if self.fault != Some(Fault::RestoreForgetsClock) {
            self.clock = s.clock;
        }
        self.next_ino = s.next_ino;
        self.usage = s.usage;
        self.tmp = s.tmp;
        self.journal.record(self.clock, Op::Restore, "/", self.usage.bytes, None);
    }
    /// A new file system from this snapshot (for parallel sandboxes).
    pub fn from_snapshot(s: &Snapshot, limits: Limits) -> Vfs {
        let mut v = Vfs::empty().with_limits(limits);
        v.restore(s);
        v.journal.clear();
        v
    }

    /// Canonical byte-level description of the state: counters, then nodes in traversal order with metadata and contents.
    pub fn dump(&self) -> Vec<u8> {
        fn walk(n: &Node, p: &str, out: &mut Vec<u8>) {
            let k = match n.kind {
                Kind::File(_) => 'f',
                Kind::Dir(_) => 'd',
                Kind::Link(_) => 'l',
            };
            let m = &n.meta;
            out.extend_from_slice(
                format!("{k} {p} ino={} mode={:o} mtime={} ctime={} size={}\n", m.ino, m.mode, m.mtime, m.ctime, n.size())
                    .as_bytes(),
            );
            match &n.kind {
                Kind::File(d) => {
                    out.extend_from_slice(d);
                    out.push(b'\n');
                }
                Kind::Link(t) => {
                    out.extend_from_slice(t.as_bytes());
                    out.push(b'\n');
                }
                Kind::Dir(e) => {
                    for (name, c) in e {
                        walk(c, &path::child(p, name), out);
                    }
                }
            }
        }
        let mut out = format!(
            "vfs-dump v1\nclock {}\nnext_ino {}\nbytes {}\nfiles {}\ncwd {}\ntmp {}\n",
            self.clock,
            self.next_ino,
            self.usage.bytes,
            self.usage.files,
            self.cwd(),
            self.tmp
        )
        .into_bytes();
        walk(&self.root, "/", &mut out);
        out
    }
    /// FNV-1a 64 of `dump()`.
    pub fn fingerprint(&self) -> u64 {
        crate::fnv1a64(&self.dump())
    }

    // ---------- explicit import and export of a real directory ----------

    /// Copy a real directory `real` into `dest` (created). Links are carried over as links of this tree
    /// (real targets are not read); devices, pipes and non-UTF-8 names are skipped with a counter.
    /// Limits apply: a too-large file stops the import with a clear error before reading.
    pub fn import_dir(&mut self, real: &std::path::Path, dest: &str) -> Result<Transfer, FsError> {
        let r = (|| {
            if !std::fs::metadata(real)?.is_dir() {
                return Err(FsError::with(Errno::NotDir, format!("import: '{}' is not a directory", real.display())));
            }
            self.mkdir_all(dest)?;
            let mut t = Transfer::default();
            self.import_rec(real, dest, &mut t)?;
            Ok(t)
        })();
        let n = r.as_ref().map_or(0, |t| t.bytes);
        self.log(Op::Import, &format!("{} -> {dest}", real.display()), n, r)
    }

    fn import_rec(&mut self, real: &std::path::Path, dest: &str, t: &mut Transfer) -> Result<(), FsError> {
        let mut names: Vec<std::ffi::OsString> =
            std::fs::read_dir(real)?.map(|e| e.map(|e| e.file_name())).collect::<Result<_, _>>()?;
        names.sort();
        for name in names {
            let Some(s) = name.to_str() else {
                t.skipped += 1;
                continue;
            };
            let rp = real.join(&name);
            let vp = path::child(dest, s);
            let md = std::fs::symlink_metadata(&rp)?;
            let ft = md.file_type();
            if ft.is_symlink() {
                match std::fs::read_link(&rp)?.to_str() {
                    Some(target) => {
                        self.symlink(target, &vp)?;
                        t.links += 1;
                    }
                    None => t.skipped += 1,
                }
            } else if ft.is_dir() {
                self.mkdir_all(&vp)?;
                t.dirs += 1;
                self.import_rec(&rp, &vp, t)?;
            } else if ft.is_file() {
                if md.len() > self.limits.max_file_size {
                    return Err(FsError::with(
                        Errno::FBig,
                        format!(
                            "vfs limit: file '{}' is {} bytes, max_file_size = {}",
                            rp.display(),
                            md.len(),
                            self.limits.max_file_size
                        ),
                    ));
                }
                let d = std::fs::read(&rp)?;
                self.write(&vp, &d)?;
                t.files += 1;
                t.bytes += d.len() as u64;
            } else {
                t.skipped += 1;
            }
        }
        Ok(())
    }

    /// Write the subtree `src` into a real directory `real` (created). Links are not exported (counter);
    /// we do not write through an existing real link in `real` — error.
    pub fn export_dir(&mut self, src: &str, real: &std::path::Path) -> Result<Transfer, FsError> {
        let r = (|| {
            let comps = self.resolve(src, true)?;
            let node = self.lookup(&comps).ok_or_else(|| FsError::new(Errno::NoEnt))?.clone();
            if !matches!(node.kind, Kind::Dir(_)) {
                return Err(FsError::new(Errno::NotDir));
            }
            std::fs::create_dir_all(real)?;
            let mut t = Transfer::default();
            export_rec(&node, real, &mut t)?;
            Ok(t)
        })();
        let n = r.as_ref().map_or(0, |t| t.bytes);
        self.log(Op::Export, &format!("{src} -> {}", real.display()), n, r)
    }
}

fn export_rec(n: &Node, real: &std::path::Path, t: &mut Transfer) -> Result<(), FsError> {
    let Kind::Dir(e) = &n.kind else { return Ok(()) };
    for (name, c) in e {
        let rp = real.join(&**name);
        if let Ok(m) = std::fs::symlink_metadata(&rp) {
            if m.file_type().is_symlink() {
                return Err(FsError::with(
                    Errno::Perm,
                    format!("export: refusing to write through the real symlink '{}'", rp.display()),
                ));
            }
        }
        match &c.kind {
            Kind::Dir(_) => {
                if !rp.is_dir() {
                    std::fs::create_dir(&rp)?;
                }
                t.dirs += 1;
                export_rec(c, &rp, t)?;
            }
            Kind::File(d) => {
                std::fs::write(&rp, &d[..])?;
                t.files += 1;
                t.bytes += d.len() as u64;
            }
            Kind::Link(_) => t.links += 1,
        }
    }
    Ok(())
}
