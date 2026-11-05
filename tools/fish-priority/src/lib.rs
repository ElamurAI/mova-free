//! Cooperative priority dispatch; source admission belongs to the deployment.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
pub fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
#[derive(Clone, Debug)]
pub struct ReadyJob {
    pub id: String,
    pub sequence: u64,
    pub admission_sha256: String,
    pub payload: Value,
}
pub struct Scheduler {
    pub root: PathBuf,
    pub ordinary_roots: Vec<PathBuf>,
}
fn fresh(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    File::open(p.parent().ok_or("no parent")?)?.sync_all()?;
    Ok(())
}
impl Scheduler {
    fn marker(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(
            &json!({"owner":"mova-fish-priority-v1","scheduler_root":self.root}),
        )?)
    }
    fn pauses(&self, on: bool) -> Result<()> {
        let marker = self.marker()?;
        for r in &self.ordinary_roots {
            if !r.is_absolute() || !r.is_dir() {
                return Err("invalid ordinary worker root".into());
            }
            let p = r.join("PAUSE");
            if on {
                match fresh(&p, &marker) {
                    Ok(()) => {}
                    Err(e)
                        if e.downcast_ref::<std::io::Error>()
                            .is_some_and(|x| x.kind() == std::io::ErrorKind::AlreadyExists) => {}
                    Err(e) => return Err(e),
                }
            } else {
                match fs::read(&p) {
                    Ok(b) if b == marker => {
                        fs::remove_file(&p)?;
                        File::open(r)?.sync_all()?;
                    }
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(())
    }
    /// Caller supplies only independently admitted ready work, never raw candidates.
    /// Call between jobs; cooperative PAUSE never interrupts an in-flight child.
    pub fn dispatch_once<F>(
        &self,
        mut ready: Vec<ReadyJob>,
        ordinary_quiescent: bool,
        run: F,
    ) -> Result<Value>
    where
        F: FnOnce(&ReadyJob) -> Result<Value>,
    {
        if !self.root.is_absolute() {
            return Err("absolute scheduler root required".into());
        }
        fs::create_dir_all(&self.root)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(self.root.join("lock"))?;
        if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err("priority scheduler already active".into());
        }
        if self.root.join("STOP").exists() {
            return Ok(json!({"state":"stopped","automatic_retry":false}));
        }
        if self.root.join("PAUSE").exists() {
            return Ok(json!({"state":"paused","automatic_retry":false}));
        }
        // Uncertain reservations survive removal of an admission from the queue.
        for entry in fs::read_dir(&self.root)? {
            let p = entry?.path();
            if p.is_dir() && p.join("reserved.json").exists() && !p.join("receipt.json").exists() {
                return Err("uncertain retained priority reservation".into());
            }
        }
        ready.sort_by(|a, b| (a.sequence, &a.id).cmp(&(b.sequence, &b.id)));
        let mut seen = std::collections::BTreeSet::new();
        for j in &ready {
            if !digest(&j.id) || !digest(&j.admission_sha256) || !seen.insert(j.id.clone()) {
                return Err("invalid/duplicate priority identity".into());
            }
        }
        let mut chosen = None;
        for j in &ready {
            let root = self.root.join(&j.id);
            if root.join("receipt.json").exists() {
                let receipt: Value = serde_json::from_slice(&fs::read(root.join("receipt.json"))?)?;
                if receipt["admission_sha256"] != j.admission_sha256 {
                    return Err("terminal admission changed".into());
                }
                continue;
            }
            if root.join("reserved.json").exists() {
                return Err("uncertain priority reservation; inspect before any replay".into());
            }
            chosen = Some(j);
            break;
        }
        let Some(j) = chosen else {
            self.pauses(false)?;
            return Ok(json!({"state":"ordinary_available"}));
        };
        self.pauses(true)?;
        if !ordinary_quiescent {
            return Ok(json!({"state":"awaiting_ordinary_boundary"}));
        }
        let r = self.root.join(&j.id);
        fs::create_dir_all(&r)?;
        let reservation = json!({"schema":1,"id":j.id,"admission_sha256":j.admission_sha256,"payload":j.payload,"automatic_retry":false});
        fresh(
            &r.join("reserved.json"),
            &serde_json::to_vec_pretty(&reservation)?,
        )?;
        // An error deliberately retains both reservation and owned PAUSE markers.
        let result = run(j)?;
        let receipt = json!({"schema":1,"id":j.id,"admission_sha256":j.admission_sha256,"result":result,"automatic_retry":false});
        fresh(
            &r.join("receipt.json"),
            &serde_json::to_vec_pretty(&receipt)?,
        )?;
        // Keep ordinary work paused while any admitted ready item remains.
        let pending = ready
            .iter()
            .any(|x| !self.root.join(&x.id).join("receipt.json").exists());
        if !pending {
            self.pauses(false)?;
        }
        Ok(receipt)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Scheduler {
        let r = std::env::temp_dir().join(format!(
            "fish-priority-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(r.join("ordinary")).unwrap();
        Scheduler {
            root: r.join("priority"),
            ordinary_roots: vec![r.join("ordinary")],
        }
    }
    fn job(n: u64) -> ReadyJob {
        ReadyJob {
            id: hash(&n.to_le_bytes()),
            sequence: n,
            admission_sha256: hash(b"admission"),
            payload: json!({"annotation_only":true}),
        }
    }
    #[test]
    fn priority_order_and_no_replay() {
        let s = fixture();
        let b = job(2);
        let a = job(1);
        s.dispatch_once(vec![b.clone(), a.clone()], true, |j| {
            assert_eq!(j.sequence, 1);
            assert!(s.ordinary_roots[0].join("PAUSE").exists());
            Ok(json!("done"))
        })
        .unwrap();
        assert!(s.ordinary_roots[0].join("PAUSE").exists());
        s.dispatch_once(vec![b.clone(), a.clone()], true, |j| {
            assert_eq!(j.sequence, 2);
            Ok(json!("done"))
        })
        .unwrap();
        assert!(!s.ordinary_roots[0].join("PAUSE").exists());
        s.dispatch_once(vec![a, b], true, |_| panic!("replayed"))
            .unwrap();
    }
    #[test]
    fn foreign_pause_and_stop_preserved() {
        let s = fixture();
        fs::write(s.ordinary_roots[0].join("PAUSE"), b"operator").unwrap();
        s.dispatch_once(vec![job(1)], true, |_| Ok(json!("done")))
            .unwrap();
        assert_eq!(
            fs::read(s.ordinary_roots[0].join("PAUSE")).unwrap(),
            b"operator"
        );
        fs::write(s.root.join("STOP"), b"stop").unwrap();
        s.dispatch_once(vec![job(2)], true, |_| panic!("STOP bypass"))
            .unwrap();
    }
    #[test]
    fn uncertainty_not_replayed() {
        let s = fixture();
        assert!(s
            .dispatch_once(vec![job(1)], true, |_| Err("unknown child result".into()))
            .is_err());
        assert!(s
            .dispatch_once(vec![job(1)], true, |_| panic!("uncertain replay"))
            .is_err());
        assert!(s.ordinary_roots[0].join("PAUSE").exists());
    }
    #[test]
    fn active_job_finishes_before_priority_starts() {
        let s = fixture();
        let state = s
            .dispatch_once(vec![job(1)], false, |_| panic!("active job interrupted"))
            .unwrap();
        assert_eq!(state["state"], "awaiting_ordinary_boundary");
        assert!(!s.root.join(job(1).id).join("reserved.json").exists());
        assert!(s.ordinary_roots[0].join("PAUSE").exists());
        s.dispatch_once(vec![job(1)], true, |_| Ok(json!("done")))
            .unwrap();
    }
    #[test]
    fn uncertain_job_cannot_disappear_with_queue() {
        let s = fixture();
        assert!(s
            .dispatch_once(vec![job(1)], true, |_| Err("uncertain".into()))
            .is_err());
        assert!(s
            .dispatch_once(vec![], true, |_| panic!("empty queue executed"))
            .is_err());
        assert!(s.ordinary_roots[0].join("PAUSE").exists());
    }
    #[test]
    fn changed_admission_rejected() {
        let s = fixture();
        let mut j = job(1);
        s.dispatch_once(vec![j.clone()], true, |_| Ok(json!("done")))
            .unwrap();
        j.admission_sha256 = hash(b"changed");
        assert!(s
            .dispatch_once(vec![j], true, |_| panic!("changed replay"))
            .is_err());
    }
}
