//! File system errors: the error number (errno) in three numberings — ours, Linux and WASI preview1.
//!
//! Text without details — as in `std::io::Error` on Linux: "No such file or directory (os error 2)". This way the output
//! of programs in memory, in a real directory and in a WASM guest matches byte for byte (the "as if" rule).

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Errno {
    Perm,
    NoEnt,
    Io,
    BadF,
    Acces,
    Busy,
    Exist,
    XDev,
    NotDir,
    IsDir,
    Inval,
    MFile,
    FBig,
    NoSpc,
    SPipe,
    RoFs,
    NameTooLong,
    NoSys,
    NotEmpty,
    Loop,
    NotSup,
    DQuot,
    /// Escaping the sandbox root. For Linux — EPERM, for WASI — ENOTCAPABLE (as for pre-opened directories).
    NotCapable,
}

use Errno::*;

impl Errno {
    pub const ALL: [Errno; 23] = [
        Perm, NoEnt, Io, BadF, Acces, Busy, Exist, XDev, NotDir, IsDir, Inval, MFile, FBig, NoSpc, SPipe, RoFs,
        NameTooLong, NoSys, NotEmpty, Loop, NotSup, DQuot, NotCapable,
    ];

    /// Number in Linux (x86-64, asm-generic).
    pub fn linux(self) -> i32 {
        match self {
            Perm | NotCapable => 1,
            NoEnt => 2,
            Io => 5,
            BadF => 9,
            Acces => 13,
            Busy => 16,
            Exist => 17,
            XDev => 18,
            NotDir => 20,
            IsDir => 21,
            Inval => 22,
            MFile => 24,
            FBig => 27,
            NoSpc => 28,
            SPipe => 29,
            RoFs => 30,
            NameTooLong => 36,
            NoSys => 38,
            NotEmpty => 39,
            Loop => 40,
            NotSup => 95,
            DQuot => 122,
        }
    }

    /// Number in WASI preview1 (`wasi_snapshot_preview1`, type `errno`).
    pub fn wasi(self) -> u16 {
        match self {
            Perm => 63,
            NoEnt => 44,
            Io => 29,
            BadF => 8,
            Acces => 2,
            Busy => 10,
            Exist => 20,
            XDev => 75,
            NotDir => 54,
            IsDir => 31,
            Inval => 28,
            MFile => 33,
            FBig => 22,
            NoSpc => 51,
            SPipe => 70,
            RoFs => 69,
            NameTooLong => 37,
            NoSys => 52,
            NotEmpty => 55,
            Loop => 32,
            NotSup => 58,
            DQuot => 19,
            NotCapable => 76,
        }
    }

    /// Text as in glibc `strerror`.
    pub fn text(self) -> &'static str {
        match self {
            Perm | NotCapable => "Operation not permitted",
            NoEnt => "No such file or directory",
            Io => "Input/output error",
            BadF => "Bad file descriptor",
            Acces => "Permission denied",
            Busy => "Device or resource busy",
            Exist => "File exists",
            XDev => "Invalid cross-device link",
            NotDir => "Not a directory",
            IsDir => "Is a directory",
            Inval => "Invalid argument",
            MFile => "Too many open files",
            FBig => "File too large",
            NoSpc => "No space left on device",
            SPipe => "Illegal seek",
            RoFs => "Read-only file system",
            NameTooLong => "File name too long",
            NoSys => "Function not implemented",
            NotEmpty => "Directory not empty",
            Loop => "Too many levels of symbolic links",
            NotSup => "Operation not supported",
            DQuot => "Disk quota exceeded",
        }
    }

    /// Short name for the journal: `ENOENT`.
    pub fn name(self) -> &'static str {
        match self {
            Perm => "EPERM",
            NoEnt => "ENOENT",
            Io => "EIO",
            BadF => "EBADF",
            Acces => "EACCES",
            Busy => "EBUSY",
            Exist => "EEXIST",
            XDev => "EXDEV",
            NotDir => "ENOTDIR",
            IsDir => "EISDIR",
            Inval => "EINVAL",
            MFile => "EMFILE",
            FBig => "EFBIG",
            NoSpc => "ENOSPC",
            SPipe => "ESPIPE",
            RoFs => "EROFS",
            NameTooLong => "ENAMETOOLONG",
            NoSys => "ENOSYS",
            NotEmpty => "ENOTEMPTY",
            Loop => "ELOOP",
            NotSup => "ENOTSUP",
            DQuot => "EDQUOT",
            NotCapable => "ENOTCAPABLE",
        }
    }

    pub fn from_linux(code: i32) -> Option<Errno> {
        Self::ALL.iter().copied().find(|e| *e != NotCapable && e.linux() == code)
    }

    pub fn from_wasi(code: u16) -> Option<Errno> {
        Self::ALL.iter().copied().find(|e| e.wasi() == code)
    }

    /// Whether this is hitting a limit (file size, total volume, number of files).
    pub fn is_limit(self) -> bool {
        matches!(self, FBig | NoSpc | DQuot)
    }
}

/// Error: a number plus, when needed, a human-readable explanation (limit, root escape).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FsError {
    pub errno: Errno,
    pub detail: Option<String>,
}

impl FsError {
    pub fn new(errno: Errno) -> FsError {
        FsError { errno, detail: None }
    }
    pub fn with(errno: Errno, detail: impl Into<String>) -> FsError {
        FsError { errno, detail: Some(detail.into()) }
    }
    /// Attempt to escape the root: `..` above the root, a symlink pointing outside, a path outside the root.
    pub fn escape(path: &str) -> FsError {
        FsError::with(NotCapable, format!("path escapes the sandbox root: '{path}'"))
    }
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.detail {
            Some(d) => f.write_str(d),
            None => write!(f, "{} (os error {})", self.errno.text(), self.errno.linux()),
        }
    }
}

impl std::error::Error for FsError {}

impl From<Errno> for FsError {
    fn from(e: Errno) -> FsError {
        FsError::new(e)
    }
}

/// OS error → ours. OS number: in a WASI guest — the WASI number, on Linux — the Linux number. Unknown — text as is.
impl From<std::io::Error> for FsError {
    fn from(e: std::io::Error) -> FsError {
        let code = e.raw_os_error();
        #[cfg(target_os = "wasi")]
        let errno = code.and_then(|c| u16::try_from(c).ok()).and_then(Errno::from_wasi);
        #[cfg(not(target_os = "wasi"))]
        let errno = code.and_then(Errno::from_linux);
        match errno {
            Some(n) => FsError::new(n),
            None => FsError::with(Io, e.to_string()),
        }
    }
}
