//! Real file system: [`DirFs`] — a directory with a confined root, [`RealFs`] — unrestricted (explicit only).

use crate::error::{Errno, FsError};
use crate::journal::{Journal, Op};
use crate::path;
use crate::{DirEntry, Fault, FileSystem, FileType, Stat};
use std::path::{Path, PathBuf};

fn stat_of(m: &std::fs::Metadata) -> Stat {
    let kind = if m.file_type().is_symlink() {
        FileType::Symlink
    } else if m.is_dir() {
        FileType::Dir
    } else {
        FileType::File
    };
    #[cfg(unix)]
    let (ino, mode) = {
        use std::os::unix::fs::MetadataExt;
        (m.ino(), m.mode() & 0o7777)
    };
    #[cfg(not(unix))]
    let (ino, mode) = (0, if m.permissions().readonly() { 0o444 } else { 0o644 });
    let mtime = m
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos() as u64);
    Stat { kind, size: m.len(), ino, mode, mtime, ctime: mtime, nlink: 1 }
}

fn read_dir_real(p: &Path) -> Result<Vec<DirEntry>, FsError> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(p)? {
        let e = e?;
        let Ok(name) = e.file_name().into_string() else { continue };
        let ft = e.file_type()?;
        let kind = if ft.is_symlink() {
            FileType::Symlink
        } else if ft.is_dir() {
            FileType::Dir
        } else {
            FileType::File
        };
        out.push(DirEntry { name, kind, ino: 0 });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

/// Shared by real file systems: a journal with an entry number instead of time.
fn log<T>(j: &mut Journal, op: Op, p: &str, bytes: u64, r: Result<T, FsError>) -> Result<T, FsError> {
    let t = j.total() + 1;
    j.record(t, op, p, bytes, r.as_ref().err());
    r
}

/// A real directory as the root. Program paths are virtual: `/` is the directory root (like chroot), relative ones are from
/// the current directory (initially the root). `..` above the root is an error; links leading outside the root (including
/// dangling last components) are an error; the check uses the canonical path of the deepest existing ancestor.
///
/// v1 caveat: there is a window between check and access (TOCTOU) — another process may swap a directory for a link.
/// For untrusted code — use the `mem` mode.
pub struct DirFs {
    root: PathBuf,
    cwd: Vec<String>,
    tmp: u64,
    pub journal: Journal,
    /// Negative control: `ClampDotDot`, `NoConfine`.
    pub fault: Option<Fault>,
}

impl DirFs {
    pub fn new(root: impl AsRef<Path>) -> Result<DirFs, FsError> {
        let root = std::fs::canonicalize(root.as_ref())?;
        if !root.is_dir() {
            return Err(FsError::new(Errno::NotDir));
        }
        Ok(DirFs { root, cwd: Vec::new(), tmp: 0, journal: Journal::default(), fault: None })
    }
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Virtual path → real path inside the root (or a root-escape error).
    pub fn real_path(&self, p: &str) -> Result<PathBuf, FsError> {
        let comps = path::normalize_with(&self.cwd, p, self.fault == Some(Fault::ClampDotDot))?;
        let mut rp = self.root.clone();
        for c in &comps {
            rp.push(c);
        }
        if self.fault != Some(Fault::NoConfine) {
            self.confine(&rp, p)?;
        }
        Ok(rp)
    }

    fn confine(&self, rp: &Path, p: &str) -> Result<(), FsError> {
        if let Ok(m) = std::fs::symlink_metadata(rp) {
            if m.file_type().is_symlink() {
                return match std::fs::canonicalize(rp) {
                    Ok(c) if c.starts_with(&self.root) => Ok(()),
                    _ => Err(FsError::escape(p)),
                };
            }
        }
        let mut probe = rp.to_path_buf();
        loop {
            if let Ok(c) = std::fs::canonicalize(&probe) {
                return if c.starts_with(&self.root) { Ok(()) } else { Err(FsError::escape(p)) };
            }
            if !probe.pop() {
                return Err(FsError::escape(p));
            }
        }
    }

    fn with<T>(&mut self, op: Op, p: &str, f: impl FnOnce(&Path) -> Result<T, FsError>) -> Result<T, FsError> {
        let r = self.real_path(p).and_then(|rp| f(&rp));
        log(&mut self.journal, op, p, 0, r)
    }
}

impl FileSystem for DirFs {
    fn describe(&self) -> String {
        format!("dir:{}", self.root.display())
    }
    fn read(&mut self, p: &str) -> Result<Vec<u8>, FsError> {
        self.with(Op::Read, p, |rp| Ok(std::fs::read(rp)?))
    }
    fn write(&mut self, p: &str, data: &[u8]) -> Result<(), FsError> {
        self.with(Op::Write, p, |rp| Ok(std::fs::write(rp, data)?))
    }
    fn append(&mut self, p: &str, data: &[u8]) -> Result<(), FsError> {
        self.with(Op::Append, p, |rp| {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(rp)?;
            Ok(f.write_all(data)?)
        })
    }
    fn stat(&mut self, p: &str) -> Result<Stat, FsError> {
        self.with(Op::Stat, p, |rp| Ok(stat_of(&std::fs::metadata(rp)?)))
    }
    fn read_dir(&mut self, p: &str) -> Result<Vec<DirEntry>, FsError> {
        self.with(Op::ReadDir, p, read_dir_real)
    }
    fn create_dir_all(&mut self, p: &str) -> Result<(), FsError> {
        self.with(Op::Mkdir, p, |rp| Ok(std::fs::create_dir_all(rp)?))
    }
    fn remove_file(&mut self, p: &str) -> Result<(), FsError> {
        self.with(Op::RemoveFile, p, |rp| Ok(std::fs::remove_file(rp)?))
    }
    fn remove_dir(&mut self, p: &str) -> Result<(), FsError> {
        self.with(Op::RemoveDir, p, |rp| Ok(std::fs::remove_dir(rp)?))
    }
    fn remove_dir_all(&mut self, p: &str) -> Result<(), FsError> {
        let root = self.root.clone();
        self.with(Op::RemoveAll, p, |rp| {
            if rp == root {
                return Err(FsError::with(Errno::Busy, "cannot remove the root"));
            }
            Ok(std::fs::remove_dir_all(rp)?)
        })
    }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), FsError> {
        let r = self.real_path(from).and_then(|a| self.real_path(to).map(|b| (a, b)));
        let r = r.and_then(|(a, b)| Ok(std::fs::rename(a, b)?));
        log(&mut self.journal, Op::Rename, &format!("{from} -> {to}"), 0, r)
    }
    fn temp_name(&mut self, prefix: &str) -> String {
        let _ = std::fs::create_dir_all(self.root.join("tmp"));
        loop {
            self.tmp += 1;
            let p = format!("/tmp/{prefix}{}", self.tmp);
            if !self.root.join("tmp").join(format!("{prefix}{}", self.tmp)).exists() {
                return p;
            }
        }
    }
    fn journal(&self) -> Option<&Journal> {
        Some(&self.journal)
    }
}

/// Unrestricted real file system — explicit only (`--fs real`), for backward compatibility. In a WASM guest these are
/// WASI calls, i.e. the host file system (in our host — the same `Vfs`).
#[derive(Default)]
pub struct RealFs {
    #[allow(dead_code)]
    tmp: u64,
    pub journal: Journal,
}

impl RealFs {
    pub fn new() -> RealFs {
        RealFs::default()
    }
    fn with<T>(&mut self, op: Op, p: &str, f: impl FnOnce(&Path) -> Result<T, FsError>) -> Result<T, FsError> {
        let r = f(Path::new(p));
        log(&mut self.journal, op, p, 0, r)
    }
}

impl FileSystem for RealFs {
    fn describe(&self) -> String {
        "real".into()
    }
    fn read(&mut self, p: &str) -> Result<Vec<u8>, FsError> {
        self.with(Op::Read, p, |rp| Ok(std::fs::read(rp)?))
    }
    fn write(&mut self, p: &str, data: &[u8]) -> Result<(), FsError> {
        self.with(Op::Write, p, |rp| Ok(std::fs::write(rp, data)?))
    }
    fn append(&mut self, p: &str, data: &[u8]) -> Result<(), FsError> {
        self.with(Op::Append, p, |rp| {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(rp)?;
            Ok(f.write_all(data)?)
        })
    }
    fn stat(&mut self, p: &str) -> Result<Stat, FsError> {
        self.with(Op::Stat, p, |rp| Ok(stat_of(&std::fs::metadata(rp)?)))
    }
    fn read_dir(&mut self, p: &str) -> Result<Vec<DirEntry>, FsError> {
        self.with(Op::ReadDir, p, read_dir_real)
    }
    fn create_dir_all(&mut self, p: &str) -> Result<(), FsError> {
        self.with(Op::Mkdir, p, |rp| Ok(std::fs::create_dir_all(rp)?))
    }
    fn remove_file(&mut self, p: &str) -> Result<(), FsError> {
        self.with(Op::RemoveFile, p, |rp| Ok(std::fs::remove_file(rp)?))
    }
    fn remove_dir(&mut self, p: &str) -> Result<(), FsError> {
        self.with(Op::RemoveDir, p, |rp| Ok(std::fs::remove_dir(rp)?))
    }
    fn remove_dir_all(&mut self, p: &str) -> Result<(), FsError> {
        self.with(Op::RemoveAll, p, |rp| Ok(std::fs::remove_dir_all(rp)?))
    }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), FsError> {
        let r = std::fs::rename(from, to).map_err(FsError::from);
        log(&mut self.journal, Op::Rename, &format!("{from} -> {to}"), 0, r)
    }
    /// Native build — as before: `temp_dir()/<prefix><pid>_<k>_<ns>`. WASI guest — `/tmp/<prefix><k>`, as in memory:
    /// then the same script gives the same output natively with `--fs mem` and in the guest.
    fn temp_name(&mut self, prefix: &str) -> String {
        #[cfg(not(target_os = "wasi"))]
        {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static N: AtomicUsize = AtomicUsize::new(0);
            let k = N.fetch_add(1, Ordering::Relaxed);
            let nanos =
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.subsec_nanos());
            let p = std::env::temp_dir().join(format!("{prefix}{}_{k}_{nanos}", std::process::id()));
            p.to_string_lossy().into_owned()
        }
        #[cfg(target_os = "wasi")]
        loop {
            self.tmp += 1;
            let p = format!("/tmp/{prefix}{}", self.tmp);
            if std::fs::symlink_metadata(&p).is_err() {
                return p;
            }
        }
    }
    fn journal(&self) -> Option<&Journal> {
        Some(&self.journal)
    }
}
