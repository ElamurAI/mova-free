//! vfs — file system for the execution sandbox.
//!
//! Three implementations of one trait [`FileSystem`]:
//! - [`Vfs`] — in memory: copy-on-write tree, O(1) snapshots, limits, journal, virtual
//!   clock, no escaping the root; explicit import and export of a real directory;
//! - [`DirFs`] — a real directory whose root is the boundary (`..`, links pointing outside — error);
//! - [`RealFs`] — unrestricted real FS, explicit only, for backward compatibility.
//!
//! No dependencies — std only. The WASI host (`wasi-host`) routes `path_open`, `fd_read`… calls into the same [`Vfs`].

pub mod error;
mod host;
pub mod journal;
mod mem;
pub mod path;

pub use error::{Errno, FsError};
pub use host::{DirFs, RealFs};
pub use journal::{Entry, Journal, Op};
pub use mem::{EPOCH_NS, Fault, Snapshot, TICK_NS, Vfs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    File,
    Dir,
    Symlink,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stat {
    pub kind: FileType,
    pub size: u64,
    pub ino: u64,
    /// Permissions (`0o644`); in memory the owner bits are checked: read 0o400, write 0o200.
    pub mode: u32,
    /// Ns since 1970-01-01 (in memory — virtual time).
    pub mtime: u64,
    pub ctime: u64,
    pub nlink: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirEntry {
    pub name: String,
    pub kind: FileType,
    pub ino: u64,
}

/// In-memory file system limits. A violation is a clear error (`EFBIG`, `ENOSPC`, `EDQUOT`), state is unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Total size of file contents, bytes.
    pub max_total_bytes: u64,
    /// Number of nodes: files, directories and links (excluding the root).
    pub max_files: u64,
    /// Size of a single file, bytes.
    pub max_file_size: u64,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits { max_total_bytes: 256 << 20, max_files: 65_536, max_file_size: 64 << 20 }
    }
}

impl Limits {
    pub const UNLIMITED: Limits = Limits { max_total_bytes: u64::MAX, max_files: u64::MAX, max_file_size: u64::MAX };

    /// Parses `total=256M,file=64M,files=65536` (suffixes K, M, G — powers of 1024); omitted — default.
    pub fn parse(s: &str) -> Result<Limits, String> {
        let mut l = Limits::default();
        for part in s.split(',').map(str::trim).filter(|p| !p.is_empty()) {
            let (k, v) = part.split_once('=').ok_or_else(|| format!("limit \"{part}\": expected key=value"))?;
            let n = parse_size(v)?;
            match k.trim() {
                "total" => l.max_total_bytes = n,
                "file" => l.max_file_size = n,
                "files" => l.max_files = n,
                other => return Err(format!("unknown limit \"{other}\" (available: total, file, files)")),
            }
        }
        Ok(l)
    }
}

/// `64M` → 67108864; suffixes K, M, G — powers of 1024.
pub fn parse_size(s: &str) -> Result<u64, String> {
    let s = s.trim();
    let (num, mul) = match s.chars().last() {
        Some('K' | 'k') => (&s[..s.len() - 1], 1u64 << 10),
        Some('M' | 'm') => (&s[..s.len() - 1], 1 << 20),
        Some('G' | 'g') => (&s[..s.len() - 1], 1 << 30),
        _ => (s, 1),
    };
    num.parse::<u64>().ok().and_then(|n| n.checked_mul(mul)).ok_or_else(|| format!("not a number: \"{s}\""))
}

/// Usage: content bytes and nodes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    pub bytes: u64,
    pub files: u64,
}

/// Import or export summary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Transfer {
    pub files: u64,
    pub dirs: u64,
    pub bytes: u64,
    /// Import — links carried over; export — links skipped.
    pub links: u64,
    /// Skipped: devices, pipes, non-UTF-8 names.
    pub skipped: u64,
}

/// File system for language built-ins (mlab and others). Paths are strings, as in the guest program.
pub trait FileSystem {
    /// Mode: `mem`, `dir:<dir>`, `real`.
    fn describe(&self) -> String;
    fn read(&mut self, path: &str) -> Result<Vec<u8>, FsError>;
    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), FsError>;
    fn append(&mut self, path: &str, data: &[u8]) -> Result<(), FsError>;
    /// Metadata, following links.
    fn stat(&mut self, path: &str) -> Result<Stat, FsError>;
    /// Names in a directory, byte order, without `.` and `..`.
    fn read_dir(&mut self, path: &str) -> Result<Vec<DirEntry>, FsError>;
    fn create_dir_all(&mut self, path: &str) -> Result<(), FsError>;
    fn remove_file(&mut self, path: &str) -> Result<(), FsError>;
    fn remove_dir(&mut self, path: &str) -> Result<(), FsError>;
    fn remove_dir_all(&mut self, path: &str) -> Result<(), FsError>;
    fn rename(&mut self, from: &str, to: &str) -> Result<(), FsError>;
    /// Name for a temporary file (in memory and in a directory — `/tmp/<prefix><k>`, deterministic).
    fn temp_name(&mut self, prefix: &str) -> String;
    fn journal(&self) -> Option<&Journal> {
        None
    }
    /// Access to the in-memory file system (import, export, snapshots), if that is what it is.
    fn as_mem(&mut self) -> Option<&mut Vfs> {
        None
    }

    fn read_to_string(&mut self, path: &str) -> Result<String, FsError> {
        String::from_utf8(self.read(path)?).map_err(|_| FsError::with(Errno::Inval, "stream did not contain valid UTF-8"))
    }
    fn exists(&mut self, path: &str) -> bool {
        self.stat(path).is_ok()
    }
    fn copy(&mut self, from: &str, to: &str) -> Result<u64, FsError> {
        let d = self.read(from)?;
        self.write(to, &d)?;
        Ok(d.len() as u64)
    }
}

impl FileSystem for Vfs {
    fn describe(&self) -> String {
        "mem".into()
    }
    fn read(&mut self, path: &str) -> Result<Vec<u8>, FsError> {
        Vfs::read(self, path)
    }
    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), FsError> {
        Vfs::write(self, path, data)
    }
    fn append(&mut self, path: &str, data: &[u8]) -> Result<(), FsError> {
        Vfs::append(self, path, data)
    }
    fn stat(&mut self, path: &str) -> Result<Stat, FsError> {
        Vfs::stat(self, path)
    }
    fn read_dir(&mut self, path: &str) -> Result<Vec<DirEntry>, FsError> {
        Vfs::read_dir(self, path)
    }
    fn create_dir_all(&mut self, path: &str) -> Result<(), FsError> {
        Vfs::mkdir_all(self, path)
    }
    fn remove_file(&mut self, path: &str) -> Result<(), FsError> {
        Vfs::remove_file(self, path)
    }
    fn remove_dir(&mut self, path: &str) -> Result<(), FsError> {
        Vfs::remove_dir(self, path)
    }
    fn remove_dir_all(&mut self, path: &str) -> Result<(), FsError> {
        Vfs::remove_dir_all(self, path)
    }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), FsError> {
        Vfs::rename(self, from, to)
    }
    fn temp_name(&mut self, prefix: &str) -> String {
        Vfs::temp_name(self, prefix)
    }
    fn journal(&self) -> Option<&Journal> {
        Some(&self.journal)
    }
    fn as_mem(&mut self) -> Option<&mut Vfs> {
        Some(self)
    }
}

/// Mode from a CLI string: `mem`, `dir:<dir>`, `real`.
pub fn open(spec: &str, limits: Limits) -> Result<Box<dyn FileSystem>, String> {
    match spec {
        "mem" => Ok(Box::new(Vfs::new().with_limits(limits))),
        "real" => Ok(Box::new(RealFs::new())),
        s => match s.strip_prefix("dir:") {
            Some(d) if !d.is_empty() => DirFs::new(d).map(|f| Box::new(f) as Box<dyn FileSystem>).map_err(|e| format!("--fs {s}: {e}")),
            _ => Err(format!("--fs: unknown mode \"{s}\" (available: mem, dir:<dir>, real)")),
        },
    }
}

/// FNV-1a, 64 bits.
pub fn fnv1a64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}
