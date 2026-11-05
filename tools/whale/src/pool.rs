//! Crab-owned launch and observation of one prepared Whale worker cohort. No teacher logic.
//! Prior launch intents are observed, never replayed or restarted.
use crate::{
    util::{alive, error, file_hash, hash, held, load, now, operation, pid_identity, private_dir, read_bounded, save, try_lease},
    Result,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::Write,
    os::{
        fd::AsRawFd,
        unix::{
            fs::{MetadataExt, OpenOptionsExt},
            process::CommandExt,
        },
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const LIMIT: usize = 1 << 20;
pub const LEASE_ENV: &str = "MOVA_WHALE_POOL_LEASE_FD";
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    crate::util::legacy(v, k).as_str().ok_or_else(|| error(format!("missing string {k}")))
}
fn absolute(p: &Path) -> Result<()> {
    if !p.is_absolute() {
        return Err(error("trusted Whale paths must be absolute"));
    }
    Ok(())
}
struct Context {
    pool: PathBuf,
    master_path: PathBuf,
    master: Value,
    root: PathBuf,
    executable: PathBuf,
    workers: usize,
    base: Value,
    owner: String,
}
impl Context {
    /// Crab config keys: run_root (Crab), whale_config, whale_executable.
    fn new(c: &Value) -> Result<Self> {
        let crab = PathBuf::from(field(c, "run_root")?);
        let master_path = PathBuf::from(field(c, "whale_config")?);
        let executable = PathBuf::from(field(c, "whale_executable")?);
        for p in [&crab, &master_path, &executable] {
            absolute(p)?;
        }
        let bytes = read_bounded(&master_path, LIMIT)?;
        let master: Value = serde_json::from_slice(&bytes)?;
        let root = PathBuf::from(field(&master, "run_root")?);
        absolute(&root)?;
        let workers = master["workers"].as_u64().unwrap_or(2);
        if !(1..=8).contains(&workers) {
            return Err(error("Crab Whale pool requires 1..8 workers"));
        }
        let executable = executable.canonicalize()?;
        let m = fs::metadata(&executable)?;
        if !m.is_file() || m.mode() & 0o111 == 0 {
            return Err(error("trusted Whale executable is not executable"));
        }
        let base = json!({"schema":"mova-shark-pool-v1","crab_root":crab,"master_config":master_path,"master_sha256":hash(&bytes),"shark_root":root,"executable":executable,"executable_sha256":file_hash(&executable)?});
        let owner = format!("mova-shark-pool:{}\n", hash(serde_json::to_vec(&base)?));
        Ok(Self { pool: crab.join("shark-pools").join(hash(&bytes)), master_path, master, root, executable, workers: workers as usize, base, owner })
    }
    fn manifest(&self) -> PathBuf {
        self.pool.join("pool.json")
    }
    fn journal(&self) -> PathBuf {
        self.pool.join("launch.json")
    }
    fn child(&self, i: usize) -> PathBuf {
        self.pool.join(format!("child-{i}.json"))
    }
    fn worker_root(&self, i: usize) -> PathBuf {
        self.root.join("workers").join(format!("{i:02}"))
    }
}
fn verify(c: &Context, m: &Value) -> Result<()> {
    if m["base"] != c.base || m["owner"] != c.owner {
        return Err(error("Whale trusted config/executable identity changed"));
    }
    let prepared = load(&c.root.join("manifest.json"), 64 << 20)?;
    if m["prepared_result_sha256"] != hash(serde_json::to_vec(&prepared)?) {
        return Err(error("prepared Whale manifest changed"));
    }
    let workers = m["worker_configs"].as_array().ok_or("missing prepared workers")?;
    if workers.len() != c.workers {
        return Err(error("prepared pool worker count changed"));
    }
    let mut seen = BTreeSet::new();
    for w in workers {
        let i = w["worker_index"].as_u64().ok_or("worker index")? as usize;
        if i >= c.workers || !seen.insert(i) {
            return Err(error("duplicate/invalid worker index"));
        }
        let path = PathBuf::from(field(w, "config")?);
        if path != c.worker_root(i).join("config.json") || file_hash(&path)? != field(w, "sha256")? {
            return Err(error("prepared worker config hash/path changed"));
        }
        let cfg = load(&path, LIMIT)?;
        if cfg["worker_index"] != i || cfg["run_root"] != json!(c.root) || cfg["workers"].as_u64().unwrap_or(2) as usize != c.workers {
            return Err(error("worker index/root binding mismatch"));
        }
    }
    Ok(())
}
fn prepare(c: &Context) -> Result<Value> {
    let p = crate::whale::prepare(&c.master)?;
    let mut workers = p["worker_configs"].as_array().ok_or("Whale prepare omitted worker_configs")?.clone();
    for w in &mut workers {
        let path = PathBuf::from(field(w, "config")?);
        w["sha256"] = json!(file_hash(&path)?);
    }
    let m = json!({"base":c.base,"owner":c.owner,"worker_configs":workers,"prepared_result_sha256":hash(serde_json::to_vec(&p)?)});
    verify(c, &m)?;
    save(&c.manifest(), &m)?;
    Ok(m)
}
fn worker_observation(c: &Context, i: usize) -> Result<Value> {
    let launch = if c.child(i).exists() { load(&c.child(i), LIMIT)? } else { json!({"worker_index":i,"phase":"not_recorded"}) };
    let path = c.worker_root(i).join("status.json");
    let worker = if path.exists() { Some(load(&path, LIMIT)?) } else { None };
    Ok(json!({"worker_index":i,"pid_alive":alive(&launch),"launch":launch,"worker_status":worker,"pause_present":c.worker_root(i).join("PAUSE").exists(),"stop_present":c.worker_root(i).join("STOP").exists()}))
}
fn receipt_observation(c: &Context) -> Result<Value> {
    let mut files = Vec::new();
    let jobs = c.root.join("jobs");
    if jobs.is_dir() {
        for e in fs::read_dir(jobs)? {
            let e = e?;
            if e.file_type()?.is_dir() {
                let p = e.path().join("receipt.json");
                if p.is_file() {
                    files.push(p);
                }
            }
        }
    }
    files.sort();
    let total = files.len();
    let mut states = BTreeMap::<String, usize>::new();
    let mut invalid = 0;
    for p in files.iter().take(128) {
        match load(p, LIMIT) {
            Ok(v) => *states.entry(v["state"].as_str().or_else(|| v["outcome"].as_str()).unwrap_or("unclassified").into()).or_default() += 1,
            Err(_) => invalid += 1,
        }
    }
    Ok(json!({"receipt_json_files":total,"inspected":total.min(128),"truncated":total>128,"states":states,"invalid_receipts":invalid,"scope":"first 128 receipts sorted by task path; worker_status carries current progress"}))
}
fn observe(c: &Context, m: &Value) -> Result<Value> {
    verify(c, m)?;
    let children = (0..c.workers).map(|i| worker_observation(c, i)).collect::<Result<Vec<_>>>()?;
    let n = children.iter().filter(|w| w["pid_alive"] == true).count();
    let exited = children.iter().filter(|w| w["launch"]["phase"] == "exited").count();
    let journal = if c.journal().exists() { Some(load(&c.journal(), LIMIT)?) } else { None };
    let lease_held = held(&c.pool.join("workers.lease"))?;
    let uncertain = (journal.is_some() && children.iter().any(|w| w["launch"]["phase"] != "exited" && w["pid_alive"] != true)) || (n == 0 && lease_held);
    let (stop, pause) = (c.root.join("STOP").exists(), c.root.join("PAUSE").exists());
    let state = if n > 0 {
        if stop {
            "stop_requested"
        } else if pause {
            "pause_requested"
        } else if uncertain {
            "running_partial_uncertain"
        } else {
            "running"
        }
    } else if uncertain {
        "uncertain"
    } else if exited == c.workers {
        "exited"
    } else if journal.is_some() {
        "uncertain"
    } else {
        "prepared"
    };
    Ok(json!({"state":state,"workers":children,"alive_workers":n,"uncertain":uncertain,"auto_restart":false,"lifetime_lease_held":lease_held,"shared_pause_present":pause,"shared_stop_present":stop,"launch_journal":journal,"receipts":receipt_observation(c)?,"pool":c.pool,"master_config":c.master_path,"observation_scope":"PID identity, raw worker status and bounded receipt metadata"}))
}
/// Initial launch only. Prior launch intents are observed, never replayed or restarted.
pub fn start(config: &Value) -> Result<Value> {
    let c = Context::new(config)?;
    private_dir(c.pool.parent().ok_or("Crab root")?)?;
    private_dir(&c.pool)?;
    let _op = operation(&c.pool.join("operation.lock")).map_err(|_| error("another Whale pool operation owns the lease"))?;
    let m = if c.manifest().exists() {
        let m = load(&c.manifest(), LIMIT)?;
        verify(&c, &m)?;
        m
    } else {
        prepare(&c)?
    };
    if c.journal().exists() || (0..c.workers).any(|i| c.child(i).exists()) {
        return observe(&c, &m);
    }
    for name in ["STOP", "PAUSE"] {
        if c.root.join(name).exists() {
            return Err(error(format!("shared Whale {name} prevents initial launch")));
        }
    }
    let Some(lease) = try_lease(&c.pool.join("workers.lease"), true)? else {
        return Err(error("worker lifetime lease held without matching launch journal; inspect before recovery"));
    };
    let identity = hash(serde_json::to_vec(&m)?);
    save(&c.journal(), &json!({"state":"launching","at":now(),"workers":c.workers,"identity":identity,"no_auto_restart":true}))?;
    for w in m["worker_configs"].as_array().ok_or("worker configs")? {
        let i = w["worker_index"].as_u64().ok_or("worker index")? as usize;
        let cfg = PathBuf::from(field(w, "config")?);
        let mut record = json!({"worker_index":i,"phase":"reserved","at":now(),"config":cfg,"config_sha256":w["sha256"],"executable":c.executable});
        save(&c.child(i), &record)?;
        let out = OpenOptions::new().create_new(true).write(true).mode(0o600).open(c.pool.join(format!("worker-{i}.log")))?;
        let fd = lease.as_raw_fd();
        let mut command = Command::new(&c.executable);
        command.arg("whale").arg(&cfg).stdin(Stdio::null()).stdout(Stdio::from(out.try_clone()?)).stderr(Stdio::from(out)).env(LEASE_ENV, fd.to_string());
        unsafe {
            command.pre_exec(move || {
                if libc::setsid() < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                let flags = libc::fcntl(fd, libc::F_GETFD);
                if flags < 0 || libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(e) => {
                record["phase"] = json!("spawn_failed");
                record["error"] = json!(e.to_string());
                save(&c.child(i), &record)?;
                return Err(error(format!("worker {i} spawn failed; reservation retained: {e}")));
            }
        };
        let pid = child.id();
        record["pid"] = json!(pid);
        if let Some((ticks, _)) = pid_identity(pid) {
            record["start_ticks"] = json!(ticks);
        } else if let Some(exit) = child.try_wait()? {
            record["phase"] = json!("exited");
            record["exit_code"] = json!(exit.code());
            save(&c.child(i), &record)?;
            continue;
        } else {
            record["phase"] = json!("spawned_identity_uncertain");
            save(&c.child(i), &record)?;
            return Err(error("spawned worker identity unavailable; inspect without relaunch"));
        }
        record["phase"] = json!("spawned");
        save(&c.child(i), &record)?;
        let path = c.child(i);
        std::thread::spawn(move || {
            match child.wait() {
                Ok(exit) => {
                    record["phase"] = json!("exited");
                    record["exit_code"] = json!(exit.code());
                    record["exited_at"] = json!(now());
                }
                Err(e) => {
                    record["phase"] = json!("wait_uncertain");
                    record["error"] = json!(e.to_string());
                }
            }
            if let Err(e) = save(&path, &record) {
                eprintln!("Whale child exit receipt failed: {e}");
            }
        });
    }
    save(&c.journal(), &json!({"state":"launched","at":now(),"workers":c.workers,"identity":identity,"no_auto_restart":true}))?;
    observe(&c, &m)
}
/// Observe actual config pins, recorded PID identities, worker status and bounded receipt counts.
pub fn status(config: &Value) -> Result<Value> {
    let c = Context::new(config)?;
    if !c.manifest().exists() {
        return Ok(json!({"state":"not_prepared","auto_restart":false,"pool":c.pool}));
    }
    let m = load(&c.manifest(), LIMIT)?;
    observe(&c, &m)
}
/// Shared trusted-root markers only; caller-supplied paths are never admitted.
pub fn marker(config: &Value, op: &str) -> Result<Value> {
    let c = Context::new(config)?;
    private_dir(&c.pool)?;
    let _op = operation(&c.pool.join("operation.lock")).map_err(|_| error("another Whale pool operation owns the lease"))?;
    let m = load(&c.manifest(), LIMIT)?;
    verify(&c, &m)?;
    let pause = c.root.join("PAUSE");
    let stop = c.root.join("STOP");
    match op {
        "pause" | "stop" => {
            let path = if op == "pause" { &pause } else { &stop };
            if path.exists() {
                if fs::read_to_string(path)? != c.owner {
                    return Err(error("foreign Whale marker; refusing overwrite"));
                }
            } else {
                let mut f = OpenOptions::new().create_new(true).write(true).mode(0o600).open(path)?;
                f.write_all(c.owner.as_bytes())?;
                f.sync_all()?;
                File::open(&c.root)?.sync_all()?;
            }
        }
        "resume" => {
            if stop.exists() || (0..c.workers).any(|i| c.worker_root(i).join("STOP").exists()) {
                return Err(error("STOP is terminal for this Whale pool; resume refused"));
            }
            if pause.exists() {
                if fs::read_to_string(&pause)? != c.owner {
                    return Err(error("foreign Whale PAUSE; refusing removal"));
                }
                fs::remove_file(pause)?;
                File::open(&c.root)?.sync_all()?;
            }
        }
        _ => return Err(error("Whale marker operation must be pause, resume or stop")),
    }
    observe(&c, &m)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::tests::fixture_spec;
    use std::{
        os::unix::fs::PermissionsExt,
        sync::atomic::{AtomicU64, Ordering},
        time::{Duration, Instant},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        root: PathBuf,
        crab: PathBuf,
        whale: PathBuf,
        config: Value,
    }
    impl Fixture {
        fn new() -> Self {
            let base = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
            fs::create_dir_all(&base).unwrap();
            let root = base.join(format!("whale-pool-test-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::SeqCst)));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir(&root).unwrap();
            let crab = root.join("crab");
            private_dir(&crab).unwrap();
            let whale = root.join("whale");
            let source = root.join("fixture.txt");
            fs::write(&source, "The counter increases by one.\n").unwrap();
            let spec = fixture_spec(&root, "words", 40, 50);
            let item = |id: &str, b: &str| json!({"kind":"note","id":id,"binding":b,"path":source,"sha256":file_hash(&source).unwrap(),"format":"plain","licence":"open"});
            let master = json!({"spec":spec,"teacher":{"model":"gpt-fixture","effort":"medium"},"run_root":whale,"store":root.join("library"),"workers":2,"items":[item("fixture-a","notes/a.txt"),item("fixture-b","notes/b.txt")]});
            let cfg = root.join("whales.json");
            fs::write(&cfg, master.to_string()).unwrap();
            let exe = root.join("fake-whale");
            fs::write(&exe, "#!/bin/sh\n[ \"$1\" = whale ] || exit 52\nworker_dir=$(dirname \"$2\")\nidx=$(basename \"$worker_dir\")\nroot=$(dirname \"$(dirname \"$worker_dir\")\")\nprintf '%s\\n' \"$idx\" >> \"$root/calls-$idx\"\nwhile [ ! -e \"$root/STOP\" ] && [ ! -e \"$worker_dir/STOP\" ]; do\n  if [ -e \"$root/PAUSE\" ] || [ -e \"$worker_dir/PAUSE\" ]; then state=paused; else state=running; fi\n  printf '{\"state\":\"%s\",\"worker_index\":\"%s\"}' \"$state\" \"$idx\" > \"$worker_dir/status.tmp\"\n  mv \"$worker_dir/status.tmp\" \"$worker_dir/status.json\"\n  sleep 0.02\ndone\nprintf '{\"state\":\"stopped\",\"worker_index\":\"%s\"}' \"$idx\" > \"$worker_dir/status.tmp\"\nmv \"$worker_dir/status.tmp\" \"$worker_dir/status.json\"\n").unwrap();
            fs::set_permissions(&exe, fs::Permissions::from_mode(0o700)).unwrap();
            Self { root, crab: crab.clone(), whale, config: json!({"run_root":crab,"whale_config":cfg,"whale_executable":exe}) }
        }
        fn wait(&self, predicate: impl Fn() -> bool) {
            let end = Instant::now() + Duration::from_secs(5);
            while !predicate() {
                assert!(Instant::now() < end, "fixture child deadline");
                std::thread::sleep(Duration::from_millis(10));
            }
        }
        fn prepared(&self) -> (Context, Value) {
            let c = Context::new(&self.config).unwrap();
            private_dir(&c.pool).unwrap();
            let m = prepare(&c).unwrap();
            (c, m)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            if self.whale.is_dir() {
                let _ = fs::write(self.whale.join("STOP"), "fixture cleanup\n");
                let deadline = Instant::now() + Duration::from_secs(3);
                while Instant::now() < deadline {
                    let lease = Context::new(&self.config).map(|c| c.pool.join("workers.lease")).unwrap_or_else(|_| self.crab.join("missing-lease"));
                    if !held(&lease).unwrap_or(true) {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    #[test]
    fn two_disjoint_workers_exact_repeat_and_receipts() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let first = start(&f.config).unwrap();
        assert_eq!(first["workers"].as_array().unwrap().len(), 2);
        f.wait(|| f.whale.join("calls-00").exists() && f.whale.join("calls-01").exists());
        assert!(held(&Context::new(&f.config).unwrap().pool.join("workers.lease")).unwrap());
        assert_eq!(start(&f.config).unwrap()["auto_restart"], false);
        for idx in ["00", "01"] {
            assert_eq!(fs::read_to_string(f.whale.join(format!("calls-{idx}"))).unwrap().lines().count(), 1);
        }
        let m = load(&f.whale.join("manifest.json"), LIMIT).unwrap();
        let tasks = m["tasks"].as_array().unwrap();
        assert_eq!((tasks.len(), tasks[0]["worker_index"].clone(), tasks[1]["worker_index"].clone()), (2, json!(0), json!(1)));
        let receipt = f.whale.join("jobs").join(tasks[0]["key"].as_str().unwrap()).join("receipt.json");
        save(&receipt, &json!({"state":"completed","worker_index":0})).unwrap();
        assert_eq!(status(&f.config).unwrap()["receipts"]["states"]["completed"], 1);
        marker(&f.config, "stop").unwrap();
        f.wait(|| status(&f.config).unwrap()["state"] == "exited");
        assert!(marker(&f.config, "resume").is_err());
        assert_eq!(start(&f.config).unwrap()["state"], "exited");
    }
    #[test]
    fn uncertain_launch_is_never_replayed() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let (c, _) = f.prepared();
        save(&c.journal(), &json!({"state":"launching","no_auto_restart":true})).unwrap();
        save(&c.child(0), &json!({"worker_index":0,"phase":"reserved"})).unwrap();
        let state = start(&f.config).unwrap();
        assert_eq!((state["state"].clone(), state["uncertain"].clone()), (json!("uncertain"), json!(true)));
        assert!(!f.whale.join("calls-00").exists());
    }
    #[test]
    fn owned_markers_and_foreign_controls_are_preserved() {
        let _serial = crate::serial();
        let f = Fixture::new();
        f.prepared();
        let pause = f.whale.join("PAUSE");
        fs::write(&pause, "foreign-owner\n").unwrap();
        assert!(marker(&f.config, "pause").is_err());
        assert!(marker(&f.config, "resume").is_err());
        assert!(start(&f.config).is_err());
        assert_eq!(fs::read_to_string(&pause).unwrap(), "foreign-owner\n");
        fs::remove_file(&pause).unwrap();
        marker(&f.config, "pause").unwrap();
        assert!(start(&f.config).is_err());
        marker(&f.config, "resume").unwrap();
        assert!(!pause.exists());
        marker(&f.config, "stop").unwrap();
        assert!(marker(&f.config, "resume").is_err());
        assert!(start(&f.config).is_err());
        assert!(marker(&f.config, "restart").is_err());
    }
    #[test]
    fn changed_master_worker_or_executable_identity_refuses() {
        let _serial = crate::serial();
        let f = Fixture::new();
        f.prepared();
        let p = f.whale.join("workers/00/config.json");
        let mut b = fs::read(&p).unwrap();
        b.push(b' ');
        fs::write(&p, b).unwrap();
        assert!(start(&f.config).is_err());
        assert!(status(&f.config).is_err());
        assert!(!f.whale.join("calls-00").exists());
        let f = Fixture::new();
        f.prepared();
        let p = Path::new(f.config["whale_config"].as_str().unwrap());
        let mut master = load(p, LIMIT).unwrap();
        master["store"] = json!(f.root.join("changed-library"));
        save(p, &master).unwrap();
        assert!(start(&f.config).is_err());
        let f = Fixture::new();
        f.prepared();
        let p = Path::new(f.config["whale_executable"].as_str().unwrap());
        let mut b = fs::read(p).unwrap();
        b.push(b'\n');
        fs::write(p, b).unwrap();
        assert!(start(&f.config).is_err());
    }
    #[test]
    fn short_operation_releases_even_with_a_duplicate_open_description() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let (c, _) = f.prepared();
        let path = c.pool.join("operation.lock");
        let held_op = operation(&path).unwrap();
        let unrelated_copy = held_op.0.try_clone().unwrap();
        assert!(try_lease(&path, false).unwrap().is_none());
        drop(held_op);
        assert!(try_lease(&path, false).unwrap().is_some());
        drop(unrelated_copy);
        let path = c.pool.join("workers.lease");
        let lifetime = try_lease(&path, true).unwrap().unwrap();
        let child_copy = lifetime.try_clone().unwrap();
        drop(lifetime);
        assert!(try_lease(&path, false).unwrap().is_none());
        drop(child_copy);
    }
    #[test]
    fn manifest_tamper_and_surviving_descendant_lease_remain_uncertain() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let (c, _) = f.prepared();
        save(&c.journal(), &json!({"state":"launched"})).unwrap();
        for i in 0..2 {
            save(&c.child(i), &json!({"worker_index":i,"phase":"exited","exit_code":0})).unwrap();
        }
        let descendant = try_lease(&c.pool.join("workers.lease"), true).unwrap().unwrap();
        let s = status(&f.config).unwrap();
        assert_eq!((s["uncertain"].clone(), s["state"].clone()), (json!(true), json!("uncertain")));
        drop(descendant);
        assert_eq!(status(&f.config).unwrap()["state"], "exited");
        let path = f.whale.join("manifest.json");
        let mut m = load(&path, LIMIT).unwrap();
        m["tasks"][0]["worker_index"] = json!(1);
        save(&path, &m).unwrap();
        assert!(start(&f.config).is_err());
        assert!(status(&f.config).is_err());
    }
    #[test]
    fn cohorts_preserve_prior_pool_journals() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let (old, _) = f.prepared();
        save(&old.journal(), &json!({"state":"launching"})).unwrap();
        let before = fs::read(old.journal()).unwrap();
        let mut master = load(Path::new(f.config["whale_config"].as_str().unwrap()), LIMIT).unwrap();
        let next_root = f.root.join("next-whale");
        master["run_root"] = json!(next_root);
        let path = f.root.join("next.json");
        save(&path, &master).unwrap();
        let mut cfg = f.config.clone();
        cfg["whale_config"] = json!(path);
        let next = Context::new(&cfg).unwrap();
        assert_ne!(old.pool, next.pool);
        assert_eq!(next.pool.file_name().unwrap().to_string_lossy(), hash(read_bounded(&path, LIMIT).unwrap()));
        start(&cfg).unwrap();
        f.wait(|| next_root.join("calls-00").exists() && next_root.join("calls-01").exists());
        assert_eq!(fs::read(old.journal()).unwrap(), before);
        assert_eq!(status(&f.config).unwrap()["state"], "uncertain");
        marker(&cfg, "stop").unwrap();
        f.wait(|| status(&cfg).unwrap()["state"] == "exited");
        assert!(!f.whale.join("STOP").exists());
    }
}
