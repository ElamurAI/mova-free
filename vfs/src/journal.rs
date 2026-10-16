//! Operation journal: what was read and written, what failed, attempts to escape the root, hitting limits.
//! Errors never vanish silently: above the cap, ordinary entries are dropped with a counter, errors still have headroom.

use crate::error::{Errno, FsError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Read,
    Write,
    Append,
    WriteAt,
    Truncate,
    Create,
    Mkdir,
    RemoveFile,
    RemoveDir,
    RemoveAll,
    Rename,
    Symlink,
    ReadLink,
    Chmod,
    SetTime,
    Stat,
    ReadDir,
    Chdir,
    Open,
    Snapshot,
    Restore,
    Import,
    Export,
}

impl Op {
    pub fn name(self) -> &'static str {
        match self {
            Op::Read => "read",
            Op::Write => "write",
            Op::Append => "append",
            Op::WriteAt => "write_at",
            Op::Truncate => "truncate",
            Op::Create => "create",
            Op::Mkdir => "mkdir",
            Op::RemoveFile => "remove_file",
            Op::RemoveDir => "remove_dir",
            Op::RemoveAll => "remove_all",
            Op::Rename => "rename",
            Op::Symlink => "symlink",
            Op::ReadLink => "read_link",
            Op::Chmod => "chmod",
            Op::SetTime => "set_time",
            Op::Stat => "stat",
            Op::ReadDir => "read_dir",
            Op::Chdir => "chdir",
            Op::Open => "open",
            Op::Snapshot => "snapshot",
            Op::Restore => "restore",
            Op::Import => "import",
            Op::Export => "export",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub seq: u64,
    /// Virtual time (ns since 1970-01-01) for the in-memory file system; for a real directory — the entry number.
    pub time: u64,
    pub op: Op,
    pub path: String,
    pub bytes: u64,
    pub err: Option<Errno>,
}

#[derive(Clone, Debug)]
pub struct Journal {
    entries: Vec<Entry>,
    /// Cap on ordinary entries; errors get the same amount of extra headroom.
    pub cap: usize,
    seq: u64,
    dropped: u64,
    errors: u64,
    denied: u64,
    limited: u64,
}

impl Default for Journal {
    fn default() -> Journal {
        Journal { entries: Vec::new(), cap: 100_000, seq: 0, dropped: 0, errors: 0, denied: 0, limited: 0 }
    }
}

impl Journal {
    pub fn record(&mut self, time: u64, op: Op, path: &str, bytes: u64, err: Option<&FsError>) {
        self.seq += 1;
        let errno = err.map(|e| e.errno);
        if let Some(e) = errno {
            self.errors += 1;
            if e == Errno::NotCapable {
                self.denied += 1;
            }
            if e.is_limit() {
                self.limited += 1;
            }
        }
        let room = if errno.is_some() { self.cap.saturating_mul(2) } else { self.cap };
        if self.entries.len() >= room {
            self.dropped += 1;
            return;
        }
        self.entries.push(Entry { seq: self.seq, time, op, path: path.to_string(), bytes, err: errno });
    }
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    /// Total operations (including those dropped above the cap).
    pub fn total(&self) -> u64 {
        self.seq
    }
    pub fn errors(&self) -> u64 {
        self.errors
    }
    /// Attempts to escape the root.
    pub fn denied(&self) -> u64 {
        self.denied
    }
    /// Times a limit was hit.
    pub fn limited(&self) -> u64 {
        self.limited
    }
    pub fn dropped(&self) -> u64 {
        self.dropped
    }
    pub fn clear(&mut self) {
        *self = Journal { cap: self.cap, ..Journal::default() };
    }
    /// TSV: `seq  time  op  path  bytes  result`.
    pub fn tsv(&self) -> String {
        let mut s = String::from("seq\ttime\top\tpath\tbytes\tresult\n");
        for e in &self.entries {
            let path = e.path.replace(['\t', '\n'], " ");
            let res = e.err.map_or("ok", |x| x.name());
            s.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", e.seq, e.time, e.op.name(), path, e.bytes, res));
        }
        s
    }
    pub fn summary(&self) -> String {
        format!(
            "vfs journal: {} operations, {} errors, {} root-escape attempts, {} limit hits, {} not recorded (cap {})",
            self.seq, self.errors, self.denied, self.limited, self.dropped, self.cap
        )
    }
}
