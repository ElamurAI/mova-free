//! Cooperative idle-GPU continuation through Mova's native parser engine.
//! Deployment data and policy are injected; no private module is required.
pub mod evidence;
pub mod trainer;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Versioned, immutable annotation snapshots supplied by the deployment.
/// Status is a poll hint: export may observe a newer generation. Both identities
/// must be lowercase SHA-256; export must never invent independent witnesses.
#[derive(Clone, Debug)]
pub enum PriorityWork {
    None,
    /// Reviewed annotation has not yet been admitted; ordinary work must wait.
    AwaitingReview {
        diagnostic_id: String,
    },
    /// A deployment-verified immutable export, never a raw prompt or failure.
    Reviewed {
        diagnostic_id: String,
        export: serde_json::Value,
    },
}
pub trait TrainingData {
    /// Called at a round boundary before ordinary polling/export. Backends must
    /// rank ready work deterministically and preserve pending admission failures.
    fn priority(&mut self, _config: &serde_json::Value) -> Result<PriorityWork> {
        Ok(PriorityWork::None)
    }
    /// Durably acknowledge terminal priority work. On error the controller
    /// pauses rather than replaying an already-trained uncertain round.
    fn priority_terminal(
        &mut self,
        _config: &serde_json::Value,
        _diagnostic_id: &str,
        _receipt: &serde_json::Value,
    ) -> Result<()> {
        Err("eel: priority backend must implement durable terminal acknowledgement".into())
    }
    fn status(&mut self, config: &serde_json::Value) -> Result<serde_json::Value>;
    fn export(
        &mut self,
        config: &serde_json::Value,
        destination: &std::path::Path,
    ) -> Result<serde_json::Value>;
}
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    os::{fd::AsRawFd, unix::process::CommandExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
// Historical on-disk wire tag: retain ownership/recovery/retention compatibility.
const OWNER: &str = "peta-eel-v1";
/// Status identity checked by read-only Crab status observers.
pub const KIND: &str = "mova_eel";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
/// Run identity: SHA-256 of the serialized parsed controller configuration, as bound in `identity.json`.
pub fn config_identity(c: &Value) -> Result<String> {
    Ok(hash(&serde_json::to_vec(c)?))
}
fn field<'a>(c: &'a Value, k: &str) -> Result<&'a str> {
    c[k].as_str()
        .ok_or_else(|| format!("eel: missing string {k}").into())
}
fn path(c: &Value, k: &str) -> Result<PathBuf> {
    let p = PathBuf::from(field(c, k)?);
    if !p.is_absolute() {
        return Err(format!("eel: {k} must be absolute").into());
    }
    Ok(p)
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn load(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn atomic(p: &Path, v: &Value) -> Result<()> {
    let tmp = p.with_extension("eel-tmp");
    let mut f = OpenOptions::new().create_new(true).write(true).open(&tmp)?;
    f.write_all(&serde_json::to_vec_pretty(v)?)?;
    f.sync_all()?;
    drop(f);
    fs::rename(tmp, p)?;
    File::open(p.parent().ok_or("eel: no parent")?)?.sync_all()?;
    Ok(())
}
fn status(r: &Path, c: &Value, state: &str, detail: &str, round: Option<u64>) -> Result<()> {
    atomic(
        &r.join("status.json"),
        &json!({"kind":KIND,"config_sha256":config_identity(c)?,"state":state,"detail":detail,"round":round,"heartbeat_unix":now(),"low_priority":true,"instant_preemption":false}),
    )
}
fn lease(p: &Path) -> Result<Option<File>> {
    fs::create_dir_all(p.parent().ok_or("eel: lease parent")?)?;
    let f = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(p)?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        let e = std::io::Error::last_os_error();
        if e.kind() == std::io::ErrorKind::WouldBlock {
            return Ok(None);
        }
        return Err(e.into());
    }
    Ok(Some(f))
}
fn bytes(p: &Path) -> Result<u64> {
    let mut n = 0u64;
    for e in fs::read_dir(p)? {
        let e = e?;
        let t = match e.file_type() {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(e.into()),
        };
        if t.is_dir() {
            match bytes(&e.path()) {
                Ok(k) => n = n.saturating_add(k),
                Err(e)
                    if e.downcast_ref::<std::io::Error>()
                        .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) => {}
                Err(e) => return Err(e),
            }
        } else if t.is_file() {
            match e.metadata() {
                Ok(m) => n = n.saturating_add(m.len()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            }
        } else {
            return Err("eel: special/symlink file in owned run storage".into());
        }
    }
    Ok(n)
}
fn storage(c: &Value) -> Result<bool> {
    let root = path(c, "run_root")?;
    let p = std::ffi::CString::new(root.as_os_str().as_encoded_bytes())?;
    let mut s: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(p.as_ptr(), &mut s) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(
        (s.f_bavail as u64).saturating_mul(s.f_frsize as u64) >= 10_000_000_000
            && bytes(&root)? < c["eel_max_run_bytes"].as_u64().unwrap_or(50_000_000_000),
    )
}
fn query(c: &Value, args: &[&str], out: &Path) -> Result<String> {
    let executable = c["nvidia_smi"].as_str().unwrap_or("/usr/bin/nvidia-smi");
    let f = OpenOptions::new().create_new(true).write(true).open(out)?;
    let mut cmd = Command::new(executable);
    cmd.args(args)
        .process_group(0)
        .stdin(Stdio::null())
        .stdout(Stdio::from(f.try_clone()?))
        .stderr(Stdio::from(f));
    let mut child = cmd.spawn()?;
    let start = Instant::now();
    loop {
        if let Some(s) = child.try_wait()? {
            if !s.success() {
                return Err("eel: nvidia-smi failed; GPU idleness unknown".into());
            }
            break;
        }
        if start.elapsed() > Duration::from_secs(3) {
            unsafe {
                libc::kill(-(child.id() as i32), libc::SIGKILL);
            }
            let _ = child.wait();
            return Err("eel: GPU query timeout; idleness unknown".into());
        }
        thread::sleep(Duration::from_millis(20));
    }
    Ok(fs::read_to_string(out)?)
}
fn gpu_idle(c: &Value, r: &Path, probe: u64) -> Result<bool> {
    let Some(_gpu) = lease(&path(c, "gpu_lock")?)? else {
        return Ok(false);
    };
    let appfile = r.join(format!("probe-{probe}-apps.tmp"));
    let utilfile = r.join(format!("probe-{probe}-util.tmp"));
    let apps = query(
        c,
        &[
            "-i",
            "0",
            "--query-compute-apps=pid",
            "--format=csv,noheader,nounits",
        ],
        &appfile,
    );
    let utils = if apps.is_ok() {
        query(
            c,
            &[
                "-i",
                "0",
                "--query-gpu=utilization.gpu",
                "--format=csv,noheader,nounits",
            ],
            &utilfile,
        )
    } else {
        Err("eel: app query failed".into())
    };
    let _ = fs::remove_file(&appfile);
    let _ = fs::remove_file(&utilfile);
    let apps = apps?;
    let utils = utils?;
    if !apps.trim().is_empty() {
        if apps.lines().any(|l| l.trim().parse::<u32>().is_err()) {
            return Err("eel: unrecognized GPU process output".into());
        }
        return Ok(false);
    }
    let vals: Vec<_> = utils.lines().filter(|l| !l.trim().is_empty()).collect();
    if vals.len() != 1 {
        return Err("eel: expected exactly GPU0 utilization".into());
    }
    let utilization = vals[0].trim().parse::<u32>()?;
    if utilization > 100 {
        return Err("eel: invalid GPU utilization".into());
    }
    Ok(utilization <= c["eel_max_gpu_utilization"].as_u64().unwrap_or(5).min(100) as u32)
}
fn anchor(c: &Value) -> Result<f64> {
    let f = c["ewt_las_floor"]
        .as_f64()
        .ok_or("eel: explicit fixed ewt_las_floor required")?;
    if !f.is_finite() || !(0.0..=100.0).contains(&f) {
        return Err("eel: invalid EWT LAS floor".into());
    }
    Ok(f)
}
fn paused(r: &Path, c: &Value, detail: &str, round: Option<u64>) -> Result<()> {
    if !r.join("PAUSE").exists() {
        atomic(
            &r.join("PAUSE"),
            &json!({"owner":OWNER,"reason":detail,"round":round}),
        )?;
    }
    status(r, c, "paused", detail, round)
}
fn round_config(c: &Value, round: &Path, index: u64) -> Result<Value> {
    anchor(c)?;
    let mut v = c.clone();
    v["run_root"] = json!(round);
    v["epochs"] = json!(1);
    v["lr"] = json!(0.00003);
    let fraction = if c["eel_gp_sample"].is_null() {
        0.15
    } else {
        c["eel_gp_sample"]
            .as_f64()
            .ok_or("eel: sample fraction must be numeric")?
    };
    if !fraction.is_finite() || !(0.01..=0.25).contains(&fraction) {
        return Err("eel: idle GP_SAMPLE must be in [0.01,0.25]".into());
    }
    v["gp_sample"] = json!(fraction);
    v["seed"] = json!(c["seed"]
        .as_u64()
        .unwrap_or(42)
        .checked_add(index)
        .ok_or("eel: experiment seed overflow")?);
    v["force_train_once"] = json!(true);
    v["min_new_train"] = json!(0);
    v["train_timeout_seconds"] = json!(c["eel_slice_seconds"].as_u64().unwrap_or(20).clamp(10, 20));
    Ok(v)
}
fn complete_receipt(round: &Path) -> Result<Value> {
    let trainer = round.join("trainer");
    let sp = trainer.join("status.json");
    if !sp.exists() {
        return Err("eel: trainer returned without durable status".into());
    }
    let s = load(&sp)?;
    if s["state"] == "waiting_for_gpu" {
        return Ok(json!({"outcome":"deferred","reason":"lease changed after idle probe"}));
    }
    if s["state"] == "waiting_for_review" {
        return Ok(
            json!({"outcome":"deferred","reason":"priority work awaiting reviewed annotation",
            "priority_diagnostic_id":s["priority_diagnostic_id"]}),
        );
    }
    let g = s["generation"]
        .as_str()
        .ok_or("eel: missing terminal trainer generation")?;
    if g.len() != 64
        || !g
            .bytes()
            .all(|x| x.is_ascii_hexdigit() && !x.is_ascii_uppercase())
    {
        return Err("eel: invalid trainer generation".into());
    }
    let receipt = load(&trainer.join("generations").join(g).join("receipt.json"))?;
    if receipt["outcome"] == "rejected" {
        if receipt["model_unchanged"] != true
            || !matches!(
                receipt["reason_kind"].as_str(),
                Some("development_las_gate" | "absolute_anchor_gate")
            )
        {
            return Err("eel: malformed quality rejection receipt".into());
        }
        return Ok(receipt);
    }
    if receipt["outcome"] != "installed" && receipt["outcome"] != "below_threshold" {
        return Err(format!(
            "eel: trainer generation did not complete successfully: {}",
            receipt["error"]
        )
        .into());
    }
    Ok(receipt)
}
fn safe_owned(round: &Path, c: &Value) -> Result<bool> {
    if !fs::symlink_metadata(round)?.is_dir() {
        return Ok(false);
    }
    let owner = load(&round.join("owner.json"))?;
    if owner["owner"] != OWNER {
        return Ok(false);
    }
    let receipt = load(&round.join("receipt.json"))?;
    if !matches!(
        receipt["outcome"].as_str(),
        Some("installed" | "deferred" | "below_threshold" | "rejected")
    ) {
        return Ok(false);
    }
    let canonical = round.canonicalize()?;
    fn referenced(v: &Value, root: &Path) -> Result<bool> {
        match v {
            Value::String(s) if Path::new(s).is_absolute() => {
                let p = Path::new(s);
                Ok(p.starts_with(root) || (p.exists() && p.canonicalize()?.starts_with(root)))
            }
            Value::Array(a) => {
                for x in a {
                    if referenced(x, root)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            Value::Object(o) => {
                for x in o.values() {
                    if referenced(x, root)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }
    // Protect every referenced path, including source paths nested in campaign objects.
    if referenced(c, &canonical)? {
        return Ok(false);
    }
    let _ = bytes(round)?;
    Ok(true)
}
fn retain(c: &Value, r: &Path) -> Result<usize> {
    let mut completed = Vec::new();
    for e in fs::read_dir(r.join("rounds"))? {
        let e = e?;
        if !e.file_name().to_string_lossy().starts_with("round-") {
            continue;
        }
        if e.path().join("receipt.json").exists() && safe_owned(&e.path(), c).unwrap_or(false) {
            completed.push(e.path());
        }
    }
    completed.sort();
    let remove = completed.len().saturating_sub(8);
    for p in completed.iter().take(remove) {
        fs::remove_dir_all(p)?;
    }
    if remove > 0 {
        File::open(r.join("rounds"))?.sync_all()?;
    }
    Ok(remove)
}
fn next_counter(c: &Value, n: u64, previous: &Value, receipt: &Value) -> Result<Value> {
    let prior = previous["reject_streak"].as_u64().unwrap_or(0);
    let streak =
        receipt["reject_streak"]
            .as_u64()
            .unwrap_or_else(|| match receipt["outcome"].as_str() {
                Some("rejected") => prior.saturating_add(1),
                Some("deferred") => prior,
                _ => 0,
            });
    let base = c["eel_cooldown_seconds"].as_u64().unwrap_or(5).clamp(2, 60);
    let cooldown = if streak >= 3 { base.max(30) } else { base };
    Ok(
        json!({"next_round":n.checked_add(1).ok_or("eel: round counter overflow")?,"yield_until_unix":now()+cooldown,"last_round":n,"reject_streak":streak}),
    )
}
#[derive(Debug, PartialEq)]
enum Step {
    Busy,
    Yielded,
    Completed,
    Paused,
}
fn step<F: FnMut(&Value) -> Result<()>>(
    c: &Value,
    r: &Path,
    probe: u64,
    train: &mut F,
) -> Result<Step> {
    let statepath = r.join("counter.json");
    let state = if statepath.exists() {
        load(&statepath)?
    } else {
        json!({"next_round":0,"yield_until_unix":0})
    };
    let n = state["next_round"]
        .as_u64()
        .ok_or("eel: invalid round counter")?;
    let round = r.join("rounds").join(format!("round-{n:08}"));
    if round.exists() {
        if round.join("receipt.json").exists() && safe_owned(&round, c).unwrap_or(false) {
            atomic(
                &statepath,
                &next_counter(c, n, &state, &load(&round.join("receipt.json"))?)?,
            )?;
            status(
                r, c,
                "yielding",
                "recovered terminal receipt without replay",
                Some(n),
            )?;
            return Ok(Step::Yielded);
        }
        paused(
            r, c,
            "uncertain existing round; inspect parent hash and trainer evidence",
            Some(n),
        )?;
        return Ok(Step::Paused);
    }
    if state["yield_until_unix"].as_u64().unwrap_or(0) > now() {
        status(
            r, c,
            "yielding",
            "durable cooldown gives foreground GPU work an opportunity",
            None,
        )?;
        return Ok(Step::Yielded);
    }
    let _ = retain(c, r)?;
    if !storage(c)? {
        paused(
            r, c,
            "10 GB free floor or global run cap; retained evidence requires review",
            None,
        )?;
        return Ok(Step::Paused);
    }
    match gpu_idle(c, r, probe) {
        Ok(true) => {}
        Ok(false) => {
            status(
                r, c,
                "waiting_for_gpu",
                "shared lease, compute app or utilization indicates busy",
                None,
            )?;
            return Ok(Step::Busy);
        }
        Err(e) => {
            status(r, c, "waiting_for_gpu", &e.to_string(), None)?;
            return Ok(Step::Busy);
        }
    }
    // The probe lease has been dropped: trainer::run acquires its own inherited GPU lease.
    let parent = hash(&fs::read(path(c, "model_dir")?.join("model.safetensors"))?);
    fs::create_dir(&round)?;
    let roundcfg = round_config(c, &round, n)?;
    atomic(&round.join("owner.json"), &json!({"owner":OWNER,"round":n}))?;
    atomic(
        &round.join("reservation.json"),
        &json!({"owner":OWNER,"round":n,"proposed_parent_weights_sha256":parent,"config_sha256":hash(&serde_json::to_vec(&roundcfg)?),"experiment":{"seed":roundcfg["seed"],"gp_sample":roundcfg["gp_sample"],"epochs":1,"lr":0.00003},"reserved_unix":now()}),
    )?;
    status(
        r, c,
        "round_active",
        "one bounded one-epoch continuation; no instantaneous preemption",
        Some(n),
    )?;
    let start = Instant::now();
    let result = train(&roundcfg).and_then(|_| complete_receipt(&round));
    match result {
        Ok(receipt) => {
            let outcome = receipt["outcome"].as_str().ok_or("eel: missing outcome")?;
            let next = next_counter(c, n, &state, &receipt)?;
            let mut summary = json!({"experiment":{"seed":roundcfg["seed"],"gp_sample":roundcfg["gp_sample"],"epochs":1,"lr":0.00003},"reject_streak":next["reject_streak"],"reason_kind":receipt["reason_kind"],"candidate_weights_sha256":receipt["candidate_weights_sha256"],"owner":OWNER,"round":n,"outcome":outcome,"proposed_parent_weights_sha256":parent,"actual_parent_weights_sha256":receipt["old_weights_sha256"],"installed_weights_sha256":receipt["installed_weights_sha256"],"trainer_generation":receipt["generation"],"wall_seconds":start.elapsed().as_secs_f64(),"ewt_las_floor":anchor(c)?,"checkpoint_interpretation":"selection-biased developer regression, not heldout generalization"});
            summary["priority_diagnostic_id"] = receipt["priority_diagnostic_id"].clone();
            summary["evidence"] = serde_json::to_value(evidence::inspect(&summary)?)?;
            atomic(&round.join("receipt.json"), &summary)?;
            atomic(&r.join("receipts").join(format!("{n:08}.json")), &summary)?;
            atomic(&statepath, &next)?;
            let removed = retain(c, r)?;
            status(
                r, c,
                "yielding",
                &format!(
                    "terminal {outcome} round; bounded cooldown; pruned {removed} owned terminal artifacts"
                ),
                Some(n),
            )?;
            Ok(Step::Completed)
        }
        Err(e) => {
            atomic(
                &round.join("failure.json"),
                &json!({"owner":OWNER,"round":n,"error":e.to_string(),"automatic_retry":false}),
            )?;
            paused(r, c, &e.to_string(), Some(n))?;
            Ok(Step::Paused)
        }
    }
}
/// Continue only after a cooperative lease and driver observation find GPU0 idle.
pub fn run(c: &Value, once: bool, data: &mut impl TrainingData) -> Result<()> {
    anchor(c)?;
    let root = path(c, "run_root")?;
    let r = root.join("eel");
    fs::create_dir_all(r.join("rounds"))?;
    fs::create_dir_all(r.join("receipts"))?;
    let _owner = lease(&r.join("controller.lock"))?.ok_or("eel: another controller active")?;
    let identity = config_identity(c)?;
    let idfile = r.join("identity.json");
    if idfile.exists() {
        if load(&idfile)?["config_sha256"] != identity {
            return Err("eel: config changed; use a new run root".into());
        }
    } else {
        atomic(&idfile, &json!({"owner":OWNER,"config_sha256":identity}))?;
    }
    let mut probe = 0;
    loop {
        if root.join("STOP").exists() || r.join("STOP").exists() {
            status(&r, c, "stopped", "STOP present", None)?;
            return Ok(());
        }
        if root.join("PAUSE").exists() || r.join("PAUSE").exists() {
            status(&r, c, "paused", "PAUSE present", None)?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(5));
            continue;
        }
        let mut train = |roundcfg: &Value| trainer::run(roundcfg, true, data);
        let _ = step(c, &r, probe, &mut train)?;
        probe += 1;
        if once {
            return Ok(());
        }
        thread::sleep(Duration::from_secs(2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        os::unix::fs::PermissionsExt,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        dir: PathBuf,
        r: PathBuf,
        c: Value,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    fn fixture(mode: &str) -> Fixture {
        let base = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&base).unwrap();
        let dir = base.join(format!(
            "peta-eel-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let r = dir.join("eel");
        fs::create_dir_all(r.join("rounds")).unwrap();
        fs::create_dir_all(r.join("receipts")).unwrap();
        let m = dir.join("model");
        fs::create_dir(&m).unwrap();
        fs::write(m.join("model.safetensors"), b"oldweights").unwrap();
        let smi = dir.join("nvidia-smi-fake");
        let script=format!("#!/bin/sh\ncase \"$*\" in *query-compute-apps*) test '{mode}' != apps || echo 123;; *) if test '{mode}' = util; then echo 90; else echo 0; fi;; esac\n");
        fs::write(&smi, script).unwrap();
        fs::set_permissions(&smi, fs::Permissions::from_mode(0o700)).unwrap();
        let c = json!({"run_root":dir,"model_dir":m,"gpu_lock":dir.join("gpu.lock"),"nvidia_smi":smi,"ewt_las_floor":84.94,"eel_cooldown_seconds":5});
        Fixture { dir, r, c }
    }
    fn successful(cfg: &Value) -> Result<()> {
        assert_eq!(cfg["epochs"], 1);
        assert_eq!(cfg["lr"], 0.00003);
        assert_eq!(cfg["force_train_once"], true);
        assert_eq!(cfg["gp_sample"], 0.15);
        assert_eq!(cfg["train_timeout_seconds"], 20);
        let _probe = lease(&path(cfg, "gpu_lock")?)?.ok_or("probe lease leaked into trainer")?;
        let r = path(cfg, "run_root")?.join("trainer");
        let g = "a".repeat(64);
        let dir = r.join("generations").join(&g);
        fs::create_dir_all(&dir)?;
        atomic(
            &dir.join("receipt.json"),
            &json!({"outcome":"installed","generation":g,"old_weights_sha256":"old","installed_weights_sha256":"new"}),
        )?;
        atomic(
            &r.join("status.json"),
            &json!({"state":"installed","generation":g}),
        )?;
        Ok(())
    }
    #[test]
    fn planned_rounds_have_new_seeds_and_bounded_idle_samples() {
        let mut f = fixture("idle");
        let a = round_config(&f.c, &f.r.join("a"), 0).unwrap();
        let b = round_config(&f.c, &f.r.join("b"), 1).unwrap();
        assert_eq!(a["seed"], 42);
        assert_eq!(b["seed"], 43);
        assert_eq!(a["gp_sample"], 0.15);
        for v in [0.0, 0.001, 0.26, 1.0] {
            f.c["eel_gp_sample"] = json!(v);
            assert!(round_config(&f.c, &f.r.join("bad"), 0).is_err());
        }
    }
    #[test]
    fn rejected_candidate_is_terminal_then_new_experiment_can_run() {
        let f = fixture("idle");
        let mut rejected = |cfg: &Value| -> Result<()> {
            successful(cfg)?;
            let r = path(cfg, "run_root")?.join("trainer");
            let g = "a".repeat(64);
            atomic(
                &r.join("generations").join(&g).join("receipt.json"),
                &json!({"outcome":"rejected","reason_kind":"development_las_gate","model_unchanged":true,"generation":g,"old_weights_sha256":"old","candidate_weights_sha256":"candidate"}),
            )?;
            Ok(())
        };
        assert_eq!(step(&f.c, &f.r, 0, &mut rejected).unwrap(), Step::Completed);
        assert!(!f.r.join("PAUSE").exists());
        let mut s = load(&f.r.join("counter.json")).unwrap();
        assert_eq!(s["reject_streak"], 1);
        s["yield_until_unix"] = json!(0);
        atomic(&f.r.join("counter.json"), &s).unwrap();
        assert_eq!(
            step(&f.c, &f.r, 1, &mut successful).unwrap(),
            Step::Completed
        );
        let first = load(&f.r.join("receipts/00000000.json")).unwrap();
        let second = load(&f.r.join("receipts/00000001.json")).unwrap();
        assert_eq!(first["outcome"], "rejected");
        assert_ne!(first["experiment"]["seed"], second["experiment"]["seed"]);
        assert_eq!(
            fs::read(f.dir.join("model/model.safetensors")).unwrap(),
            b"oldweights"
        );
        let backoff = next_counter(
            &f.c,
            2,
            &json!({"reject_streak":2}),
            &json!({"outcome":"rejected"}),
        )
        .unwrap();
        assert!(backoff["yield_until_unix"].as_u64().unwrap() >= now() + 30);
    }
    #[test]
    fn busy_gpu_never_invokes_trainer() {
        for mode in ["apps", "util"] {
            let f = fixture(mode);
            let mut calls = 0;
            let s = step(&f.c, &f.r, 0, &mut |_| {
                calls += 1;
                Ok(())
            })
            .unwrap();
            assert_eq!(s, Step::Busy);
            assert_eq!(calls, 0);
            assert!(!f.r.join("counter.json").exists());
        }
        let f = fixture("idle");
        let _l = lease(&path(&f.c, "gpu_lock").unwrap()).unwrap().unwrap();
        let mut calls = 0;
        assert_eq!(
            step(&f.c, &f.r, 0, &mut |_| {
                calls += 1;
                Ok(())
            })
            .unwrap(),
            Step::Busy
        );
        assert_eq!(calls, 0);
    }
    #[test]
    fn one_bounded_round_has_durable_yield_and_releases_probe() {
        let f = fixture("idle");
        let mut calls = 0;
        let mut train = |cfg: &Value| {
            calls += 1;
            successful(cfg)
        };
        assert_eq!(step(&f.c, &f.r, 0, &mut train).unwrap(), Step::Completed);
        assert_eq!(step(&f.c, &f.r, 1, &mut train).unwrap(), Step::Yielded);
        assert_eq!(calls, 1);
        let s = load(&f.r.join("counter.json")).unwrap();
        assert_eq!(s["next_round"], 1);
        assert!(s["yield_until_unix"].as_u64().unwrap() > now());
        assert!(f.r.join("receipts/00000000.json").exists());
    }
    #[test]
    fn uncertain_and_failed_rounds_pause_without_retry() {
        let f = fixture("idle");
        fs::create_dir(f.r.join("rounds/round-00000000")).unwrap();
        let mut calls = 0;
        assert_eq!(
            step(&f.c, &f.r, 0, &mut |_| {
                calls += 1;
                Ok(())
            })
            .unwrap(),
            Step::Paused
        );
        assert_eq!(calls, 0);
        assert!(f.r.join("PAUSE").exists());
        let f = fixture("idle");
        assert_eq!(
            step(&f.c, &f.r, 0, &mut |_| Err("controlled failure".into())).unwrap(),
            Step::Paused
        );
        assert!(f.r.join("rounds/round-00000000/failure.json").exists());
        assert!(!f.r.join("rounds/round-00000000/receipt.json").exists());
    }
    #[test]
    fn retention_keeps_eight_and_preserves_unknown_failed_and_sources() {
        let mut f = fixture("idle");
        for n in 0..10 {
            let p = f.r.join("rounds").join(format!("round-{n:08}"));
            fs::create_dir(&p).unwrap();
            atomic(&p.join("owner.json"), &json!({"owner":OWNER,"round":n})).unwrap();
            atomic(&p.join("receipt.json"), &json!({"outcome":"installed"})).unwrap();
            fs::write(p.join("artifact"), b"owned completed").unwrap();
        }
        let protected = f.r.join("rounds/round-00000000/source-db");
        fs::write(&protected, b"never delete DB").unwrap();
        f.c["db"] = json!(protected);
        let failed = f.r.join("rounds/round-00000010");
        fs::create_dir(&failed).unwrap();
        atomic(&failed.join("owner.json"), &json!({"owner":OWNER})).unwrap();
        atomic(&failed.join("receipt.json"), &json!({"outcome":"failed"})).unwrap();
        let uncertain = f.r.join("rounds/round-00000011");
        fs::create_dir(&uncertain).unwrap();
        assert_eq!(retain(&f.c, &f.r).unwrap(), 1);
        assert!(f.r.join("rounds/round-00000000/source-db").exists());
        assert!(!f.r.join("rounds/round-00000001").exists());
        assert!(failed.exists());
        assert!(uncertain.exists());
    }
    #[test]
    fn storage_cap_and_missing_anchor_do_not_start_work() {
        let mut f = fixture("idle");
        f.c["eel_max_run_bytes"] = json!(0);
        let mut calls = 0;
        assert_eq!(
            step(&f.c, &f.r, 0, &mut |_| {
                calls += 1;
                Ok(())
            })
            .unwrap(),
            Step::Paused
        );
        assert_eq!(calls, 0);
        f.c["ewt_las_floor"] = Value::Null;
        assert!(round_config(&f.c, &f.r.join("round"), 0).is_err());
    }
    struct NoTraining;
    impl TrainingData for NoTraining {
        fn status(&mut self, _: &Value) -> Result<Value> {
            panic!("paused controller must not query training data")
        }
        fn export(&mut self, _: &Value, _: &Path) -> Result<Value> {
            panic!("paused controller must not export training data")
        }
    }
    #[test]
    fn paused_status_carries_run_identity_without_training() {
        for local in [false, true] {
            let f = fixture("idle");
            let marker = if local { f.r.join("PAUSE") } else { f.dir.join("PAUSE") };
            fs::write(&marker, b"operator pause").unwrap();
            run(&f.c, true, &mut NoTraining).unwrap();
            assert_eq!(fs::read(&marker).unwrap(), b"operator pause");
            let s = load(&f.r.join("status.json")).unwrap();
            let id = load(&f.r.join("identity.json")).unwrap();
            assert_eq!(s["kind"], "mova_eel");
            assert_eq!(s["state"], "paused");
            assert_eq!(s["config_sha256"], id["config_sha256"]);
            assert_eq!(s["config_sha256"], config_identity(&f.c).unwrap());
            assert!(fs::read_dir(f.r.join("rounds")).unwrap().next().is_none());
        }
    }
    #[test]
    fn status_identity_wire_contract_is_pinned_independently() {
        assert_eq!(KIND, "mova_eel");
        let c = json!({"run_root":"/r","model_dir":"/m"});
        assert_eq!(
            config_identity(&c).unwrap(),
            "7c7294c642657722e4a76df5378dcf5054d33db35b3de1102ba3d60975e04bf8"
        );
    }
    #[test]
    fn status_identity_follows_configuration_not_a_constant() {
        let mut f = fixture("idle");
        f.c["eel_max_run_bytes"] = json!(0);
        assert_eq!(step(&f.c, &f.r, 0, &mut |_| Ok(())).unwrap(), Step::Paused);
        let a = load(&f.r.join("status.json")).unwrap()["config_sha256"].clone();
        f.c["eel_cooldown_seconds"] = json!(6);
        assert_eq!(step(&f.c, &f.r, 0, &mut |_| Ok(())).unwrap(), Step::Paused);
        let b = load(&f.r.join("status.json")).unwrap()["config_sha256"].clone();
        assert_ne!(a, b);
        assert_eq!(b, config_identity(&f.c).unwrap());
    }
}
