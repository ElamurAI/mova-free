//! Shared bounded file, hash, lease, process-identity and storage helpers for every Whale module.
use crate::Result;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirBuilder, File, OpenOptions},
    io::{Read, Write},
    os::{
        fd::AsRawFd,
        unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt},
    },
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

/// Default free-space admission floor for new external calls and launches.
pub const DEFAULT_STORAGE_FLOOR: u64 = 10_000_000_000;

pub fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
pub fn hash(b: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(b.as_ref()))
}
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(i64::MAX as u128) as i64
}
/// Lowercase 64-digit SHA-256 hex.
pub fn valid_hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
/// Short label used for lane names and identifiers that become file names.
pub fn label(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
}
/// Repository-relative normal path: no empty, `.`, `..`, backslash or NUL components.
pub fn normal(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 1024
        && !s.contains(['\\', '\0', '\n', '\r'])
        && !s.starts_with('/')
        && s.split('/')
            .all(|x| !x.is_empty() && x != "." && x != "..")
}
pub fn file_hash(p: &Path) -> Result<String> {
    let mut r = File::open(p)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = r.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
pub fn read_bounded(p: &Path, max: usize) -> Result<Vec<u8>> {
    let mut b = Vec::new();
    File::open(p)?.take(max as u64 + 1).read_to_end(&mut b)?;
    if b.len() > max {
        return Err(error(format!("{} exceeds {max} bytes", p.display())));
    }
    Ok(b)
}
pub fn load(p: &Path, max: usize) -> Result<Value> {
    Ok(serde_json::from_slice(&read_bounded(p, max)?)?)
}
fn sync_parent(p: &Path) -> Result<()> {
    File::open(p.parent().ok_or("missing parent")?)?.sync_all()?;
    Ok(())
}
pub fn write_sync(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    Ok(())
}
/// Replace a JSON file through a synced temporary file and rename.
pub fn save(p: &Path, v: &Value) -> Result<()> {
    let tmp = p.with_extension("tmp");
    write_sync(&tmp, &serde_json::to_vec_pretty(v)?)?;
    fs::rename(tmp, p)?;
    sync_parent(p)
}
/// Create a new JSON file; an existing file is never overwritten.
pub fn create(p: &Path, v: &Value) -> Result<()> {
    create_bytes(p, &serde_json::to_vec_pretty(v)?)
}
pub fn create_bytes(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    sync_parent(p)
}
/// The value of `key`, or of the legacy key (`shark` for `whale`) that configurations and state
/// written before the Whale rename (2026-11-04) carry. Accepted for one release.
pub fn legacy<'a>(v: &'a Value, key: &str) -> &'a Value {
    match &v[key] {
        Value::Null => &v[key.replacen("whale", "shark", 1).as_str()],
        x => x,
    }
}
pub fn absolute(v: &Value, k: &str) -> Result<PathBuf> {
    let p = PathBuf::from(v[k].as_str().ok_or_else(|| error(format!("missing {k}")))?);
    if !p.is_absolute() {
        return Err(error(format!("{k} must be absolute")));
    }
    Ok(p)
}
/// Create when absent; require an owned 0700 directory that is not a symlink.
pub fn private_dir(p: &Path) -> Result<()> {
    if !p.is_absolute() {
        return Err(error("private directories must be absolute"));
    }
    if !p.exists() {
        DirBuilder::new().recursive(true).mode(0o700).create(p)?;
    }
    let m = fs::symlink_metadata(p)?;
    if !m.is_dir() || m.uid() != unsafe { libc::geteuid() } || m.mode() & 0o077 != 0 {
        return Err(error(format!(
            "{} must be an owned private 0700 directory",
            p.display()
        )));
    }
    Ok(())
}
/// Nonblocking exclusive flock. `None` means another open description holds it.
pub fn try_lease(p: &Path, create: bool) -> Result<Option<File>> {
    let f = OpenOptions::new()
        .create(create)
        .truncate(false)
        .read(true)
        .write(true)
        .mode(0o600)
        .open(p)?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
        return Ok(Some(f));
    }
    let e = std::io::Error::last_os_error();
    if e.raw_os_error() == Some(libc::EWOULDBLOCK) {
        Ok(None)
    } else {
        Err(e.into())
    }
}
/// Lifetime lease: released only when every inherited copy of the description closes.
pub fn lease(p: &Path) -> Result<File> {
    try_lease(p, true)?.ok_or_else(|| error(format!("lease held: {}", p.display())))
}
pub fn held(p: &Path) -> Result<bool> {
    if !p.exists() {
        return Ok(false);
    }
    Ok(try_lease(p, false)?.is_none())
}
/// Short management mutex that explicitly unlocks, so stray descriptor copies cannot keep it.
/// Never use it for worker or manager lifetime leases.
pub struct Operation(pub File);
impl Drop for Operation {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}
pub fn operation(p: &Path) -> Result<Operation> {
    Ok(Operation(lease(p)?))
}
/// (start time in clock ticks, state letter) from /proc; None when the PID is gone.
pub fn pid_identity(pid: u32) -> Option<(u64, char)> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let f: Vec<_> = s.rsplit_once(") ")?.1.split_whitespace().collect();
    Some((f.get(19)?.parse().ok()?, f.first()?.chars().next()?))
}
/// The recorded PID still names the same, non-zombie process.
pub fn alive(v: &Value) -> bool {
    let (Some(pid), Some(start)) = (v["pid"].as_u64(), v["start_ticks"].as_u64()) else {
        return false;
    };
    pid_identity(pid as u32).is_some_and(|(s, st)| s == start && st != 'Z' && st != 'X')
}
pub fn storage_available(root: &Path, floor: u64) -> Result<bool> {
    let raw = std::ffi::CString::new(root.as_os_str().as_encoded_bytes())?;
    let mut s: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(raw.as_ptr(), &mut s) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok((s.f_bavail as u64).saturating_mul(s.f_frsize as u64) >= floor)
}
pub fn storage_floor(v: &Value) -> Result<u64> {
    match &v["storage_floor_bytes"] {
        Value::Null => Ok(DEFAULT_STORAGE_FLOOR),
        x => x
            .as_u64()
            .ok_or_else(|| error("storage_floor_bytes must be a nonnegative integer")),
    }
}
/// UTF-8 prefix of at most `n` bytes ending on a character boundary.
pub fn prefix(s: &str, n: usize) -> &str {
    let mut end = s.len().min(n);
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}
/// A `{path, sha256}` pin. Relative paths resolve against `base`; bytes are read bounded and
/// must match the declared hash exactly.
pub fn pinned(v: &Value, base: Option<&Path>, max: usize) -> Result<(PathBuf, Vec<u8>)> {
    let raw = v["path"].as_str().ok_or("pin requires path")?;
    let expected = v["sha256"].as_str().ok_or("pin requires sha256")?;
    if !valid_hash(expected) {
        return Err(error("pin sha256 must be lowercase SHA-256 hex"));
    }
    let p = PathBuf::from(raw);
    let p = if p.is_absolute() {
        p
    } else if let Some(b) = base.filter(|_| normal(raw)) {
        b.join(p)
    } else {
        return Err(error("pin path must be absolute or normal relative to its file"));
    };
    let m = fs::symlink_metadata(&p)?;
    if !m.is_file() {
        return Err(error(format!("pinned path is not a regular file: {}", p.display())));
    }
    let b = read_bounded(&p, max)?;
    if hash(&b) != expected {
        return Err(error(format!("pinned file hash mismatch: {}", p.display())));
    }
    Ok((p, b))
}
