//! One Sea Spider per host: an exclusive `flock` on the configured lock file
//! (`service.lock`, default `config::default_lock()`), held for the whole life of `serve`, `fetch`
//! or `robots`. The holder writes who it is (pid, what it runs, since when) into the file; a
//! second start reads that and reports the running Sea Spider instead of starting. The lock is
//! released by the kernel when the holder exits, so a crash leaves no stale lease.
//!
//! Inside the holder there may be several threads (the socket server, the single fetch worker),
//! but only one fetch stream: politeness (robots.txt cache, per-IP timers) lives in that one
//! process.
use serde_json::{json, Value};
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// Exit status of a start refused because another Sea Spider runs (as `crab-tmux-worker`).
pub const EXIT_RUNNING: u8 = 5;

/// The held lock; dropping it releases the lease.
#[derive(Debug)]
pub struct Singleton {
    _file: File,
    pub path: PathBuf,
}

/// A start that did not take the lease.
#[derive(Debug)]
pub enum Refused {
    /// Another Sea Spider holds it; what it recorded about itself.
    Running { path: PathBuf, holder: Value },
    Io(String),
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refused::Running { path, holder } => {
                let pid = holder["pid"].as_u64().map_or("an unknown pid".to_string(), |p| format!("pid {p}"));
                let what = holder["role"].as_str().unwrap_or("an unrecorded command");
                let since = holder["started_unix"].as_u64().map_or(String::new(), |t| format!(" since {}", crate::answer::utc(t)));
                write!(f, "Sea Spider is already running ({pid}, {what}{since}; lock {}). I did not start a second one.", path.display())
            }
            Refused::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Refused {}

/// Take the host-wide lease or report the running holder. `role` says what this process runs
/// ("serve /path/seaspider.sock", "fetch QUEUE").
pub fn acquire(path: &Path, role: &str) -> Result<Singleton, Refused> {
    let io = |e: std::io::Error| Refused::Io(format!("{}: {e}", path.display()));
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(io)?;
    }
    let mut file = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path).map_err(io)?;
    match file.try_lock() {
        Ok(()) => {}
        Err(TryLockError::WouldBlock) => {
            let mut text = String::new();
            let _ = file.read_to_string(&mut text);
            let holder = serde_json::from_str(text.trim()).unwrap_or(Value::Null);
            return Err(Refused::Running { path: path.to_path_buf(), holder });
        }
        Err(TryLockError::Error(e)) => return Err(io(e)),
    }
    let me = json!({"pid":std::process::id(),"role":role,"started_unix":crate::crawl::unix_now()});
    file.set_len(0).map_err(io)?;
    file.seek(SeekFrom::Start(0)).map_err(io)?;
    file.write_all(format!("{me}\n").as_bytes()).and_then(|_| file.sync_data()).map_err(io)?;
    Ok(Singleton { _file: file, path: path.to_path_buf() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_second_holder_is_refused_and_told_who_runs() {
        let d = std::env::temp_dir().join(format!("seaspider-singleton-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let lock = d.join("seaspider.lock");
        let first = acquire(&lock, "serve /x/seaspider.sock").unwrap();
        // flock is per open file description: a second open in the same process conflicts too.
        match acquire(&lock, "fetch q.txt") {
            Err(Refused::Running { holder, .. }) => {
                assert_eq!(holder["pid"], std::process::id());
                assert_eq!(holder["role"], "serve /x/seaspider.sock");
            }
            other => panic!("second start not refused: {other:?}"),
        }
        let msg = acquire(&lock, "fetch q.txt").unwrap_err().to_string();
        assert!(msg.starts_with(&format!("Sea Spider is already running (pid {}, serve /x/seaspider.sock since ", std::process::id())), "{msg}");
        drop(first);
        let again = acquire(&lock, "fetch q.txt").unwrap();
        assert_eq!(again.path, lock);
        let _ = std::fs::remove_dir_all(&d);
    }
}
