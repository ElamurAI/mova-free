//! Restartable local Codex batches; completed results are published atomically.
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};

struct Lock(PathBuf);
impl Drop for Lock {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

fn requests(out: &Path) -> Result<Vec<Value>> {
    let text = std::fs::read_to_string(out.join("batch-requests.jsonl"))?;
    let reqs: Vec<Value> = text.lines().filter(|l| !l.trim().is_empty())
        .map(serde_json::from_str).collect::<Result<_, _>>()?;
    let mut ids = BTreeSet::new();
    for r in &reqs {
        let id = r["custom_id"].as_str().context("request without custom_id")?;
        ensure!(!id.is_empty() && ids.insert(id), "empty or duplicate request id {id:?}");
        mova_teacher::validate_model(r["params"]["model"].as_str().context("request model")?)?;
        ensure!(r["params"]["messages"][0]["content"].is_array(), "request content");
    }
    Ok(reqs)
}

/// A successful return in async mode means launched/running; only batch-results.jsonl means complete.
pub fn run(out: &Path, asynchronous: bool) -> Result<()> {
    let out = std::fs::canonicalize(out)?;
    if out.join("batch-results.jsonl").exists() { return Ok(()); }
    if let Ok(error) = std::fs::read_to_string(out.join("codex-error.txt")) {
        bail!("Codex batch stopped: {error}. Fix the cause and explicitly remove codex-error.txt to resume.");
    }
    requests(&out)?; // reject legacy models before any process or result-file mutation
    if let Ok(id) = std::fs::read_to_string(out.join("batch-id.txt")) {
        ensure!(id.trim() == "codex-local", "legacy API batch is still pending; it will not be polled through Claude API");
    }
    let lock_path = out.join("codex-worker.lock");
    match std::fs::create_dir(&lock_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            let pid = std::fs::read_to_string(lock_path.join("pid")).context("Codex worker startup lock has no pid; inspect before explicitly removing codex-worker.lock")?;
            {
                let pid: u32 = pid.trim().parse().context("invalid worker pid")?;
                ensure!(Path::new(&format!("/proc/{pid}")).exists(),
                    "Codex batch worker {pid} is gone; inspect partial results before explicitly removing codex-worker.lock");
            }
            ensure!(asynchronous, "a Codex batch worker is already running");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    }
    let lock = Lock(lock_path);
    std::fs::write(out.join("batch-id.txt"), "codex-local\n")?;
    if asynchronous {
        let log = std::fs::OpenOptions::new().create(true).append(true).open(out.join("codex-worker.log"))?;
        let mut child = Command::new(std::env::current_exe()?).arg("codex-worker").arg(&out)
            .stdin(Stdio::null()).stdout(log.try_clone()?).stderr(log).spawn().context("start Codex batch worker")?;
        std::mem::forget(lock); // ownership transfers before waiting; an uncertain child keeps its lock
        await_worker_start(&out, &mut child, Duration::from_secs(5))
    } else {
        finish(&out, lock)
    }
}

pub fn worker(out: &Path) -> Result<()> {
    let lock = Lock(out.join("codex-worker.lock"));
    ensure!(lock.0.is_dir(), "Codex worker must be started by batch-run");
    finish(out, lock)
}

/// Publish a complete PID with one rename; readers never see a newly created empty PID file.
/// The temporary file is exclusive, and unexpected existing temporary state is not discarded.
fn publish_pid(lock: &Path) -> Result<()> {
    let temporary = lock.join(format!("pid.{}.new", std::process::id()));
    let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open(&temporary)?;
    file.write_all(std::process::id().to_string().as_bytes())?;
    file.sync_data()?;
    drop(file);
    std::fs::rename(temporary, lock.join("pid"))?;
    Ok(())
}

/// Await only this just-spawned child's startup, never an arbitrary preexisting lock.
/// Timeout retains the child-owned lock and output; it neither kills nor retries a teacher call.
fn await_worker_start(out: &Path, child: &mut Child, budget: Duration) -> Result<()> {
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            ensure!(status.success(), "Codex worker exited during startup ({status}); inspect codex-worker.log");
            ensure!(out.join("batch-results.jsonl").exists(),
                "Codex worker exited without publishing results; inspect before resuming");
            return Ok(());
        }
        match std::fs::read_to_string(out.join("codex-worker.lock/pid")) {
            Ok(text) => {
                let pid: u32 = text.trim().parse().context("invalid newly started worker pid")?;
                ensure!(pid == child.id(), "new worker pid {pid} differs from spawned child {}; inspect before resuming", child.id());
                return Ok(());
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e).context("read newly started Codex worker pid"),
        }
        ensure!(start.elapsed() < budget,
            "Codex worker startup timed out; child {} may still be running; inspect lock/results before resuming", child.id());
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn finish(out: &Path, lock: Lock) -> Result<()> {
    publish_pid(&lock.0)?;
    let result = execute(out);
    if let Err(e) = &result { std::fs::write(out.join("codex-error.txt"), format!("{e:#}"))?; }
    result
}

fn execute(out: &Path) -> Result<()> {
    let reqs = requests(out)?;
    let source = std::fs::read(out.join("batch-requests.jsonl"))?;
    let snapshot = out.join("codex-requests.jsonl");
    if snapshot.exists() {
        ensure!(std::fs::read(&snapshot)? == source, "requests changed since the Codex batch started; use a new output directory");
    } else {
        std::fs::write(&snapshot, &source)?;
    }
    let partial = out.join("codex-results.partial.jsonl");
    let mut done = BTreeSet::new();
    // A supplied partial file may end without LF after a complete last row (`lines()` accepts it).
    // Every row is validated first; only then is the missing record separator appended, so the next
    // success starts its own line instead of being glued onto the last prior record. A whitespace-only
    // tail is left alone: the next row is appended after it, as before, and no blank line is created.
    let mut missing_separator = false;
    if partial.exists() {
        let text = std::fs::read_to_string(&partial)?;
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let r: Value = serde_json::from_str(line).context("partial Codex result is incomplete; inspect before retrying")?;
            ensure!(r["provider"] == "codex" && r["result"]["type"] == "succeeded", "invalid partial Codex result");
            let id = r["custom_id"].as_str().context("partial result id")?.to_string();
            let request = reqs.iter().find(|q| q["custom_id"] == id).context("partial result has unknown id")?;
            ensure!(r["model"] == request["params"]["model"] && done.insert(id), "mismatched or duplicate partial result");
        }
        missing_separator = !text.rsplit('\n').next().unwrap_or("").trim().is_empty();
    }
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(&partial)?;
    if missing_separator {
        file.write_all(b"\n")?;
        file.sync_data()?;
    }
    for r in &reqs {
        let id = r["custom_id"].as_str().unwrap();
        if done.contains(id) { continue; }
        let p = &r["params"];
        let model = p["model"].as_str().unwrap();
        let effort = p["output_config"]["effort"].as_str().unwrap_or("medium");
        let prompt = p["messages"][0]["content"].as_array().unwrap().iter()
            .map(|b| b["text"].as_str().context("non-text teacher request block"))
            .collect::<Result<Vec<_>>>()?.join("\n\n");
        let v = mova_teacher::ask(&out.join("codex-teacher"), model, effort, &prompt)?;
        let line = json!({"custom_id":id, "provider":"codex", "model":model, "cost_reported":false,
            "result":{"type":"succeeded","message":{"content":[{"type":"text","text":v["result"]}],
                "stop_reason":"end_turn","usage":v["usage"]}}});
        writeln!(file, "{line}")?;
        file.sync_data()?;
    }
    drop(file);
    std::fs::rename(partial, out.join("batch-results.jsonl"))?;
    Ok(())
}


#[cfg(test)]
mod startup_tests {
    use super::*;
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            let p = std::env::temp_dir().join(format!("mova-batch-startup-{}-{stamp}", std::process::id()));
            std::fs::create_dir(&p).unwrap();
            std::fs::write(p.join("batch-requests.jsonl"), "").unwrap();
            std::fs::create_dir(p.join("codex-worker.lock")).unwrap();
            Self(p)
        }
        fn lock(&self) -> PathBuf { self.0.join("codex-worker.lock") }
    }
    impl Drop for Fixture { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); } }
    fn sleeping_child() -> Child { Command::new("sleep").arg("2").spawn().unwrap() }
    fn stop(child: &mut Child) { let _ = child.kill(); let _ = child.wait(); }

    #[test]
    fn pid_publication_is_complete_and_preserves_unexpected_temporary_state() {
        let f = Fixture::new();
        publish_pid(&f.lock()).unwrap();
        let expected = std::process::id().to_string();
        assert_eq!(std::fs::read_to_string(f.lock().join("pid")).unwrap(), expected);
        let temporary = f.lock().join(format!("pid.{}.new", std::process::id()));
        assert!(!temporary.exists());
        std::fs::write(&temporary, "foreign temporary bytes").unwrap();
        assert!(publish_pid(&f.lock()).is_err());
        assert_eq!(std::fs::read_to_string(f.lock().join("pid")).unwrap(), expected);
        assert_eq!(std::fs::read_to_string(temporary).unwrap(), "foreign temporary bytes");
    }

    #[test]
    fn startup_waits_only_for_its_known_child() {
        let f = Fixture::new();
        let mut child = sleeping_child();
        let pid = child.id();
        let path = f.lock().join("pid");
        let writer = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(25));
            std::fs::write(path, pid.to_string()).unwrap();
        });
        let result = await_worker_start(&f.0, &mut child, Duration::from_secs(1));
        stop(&mut child);
        writer.join().unwrap();
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(std::fs::read_to_string(f.lock().join("pid")).unwrap(), pid.to_string());
    }

    #[test]
    fn startup_rejects_empty_malformed_or_wrong_pid_without_rewriting_it() {
        for text in ["".to_string(), "not-a-pid".to_string(), std::process::id().to_string()] {
            let f = Fixture::new();
            let mut child = sleeping_child();
            std::fs::write(f.lock().join("pid"), &text).unwrap();
            let result = await_worker_start(&f.0, &mut child, Duration::from_secs(1));
            stop(&mut child);
            assert!(result.is_err(), "{text:?}");
            assert_eq!(std::fs::read_to_string(f.lock().join("pid")).unwrap(), text);
        }
    }

    #[test]
    fn startup_timeout_preserves_a_live_child_and_lock() {
        let f = Fixture::new();
        let mut child = sleeping_child();
        let result = await_worker_start(&f.0, &mut child, Duration::from_millis(10));
        let still_running = child.try_wait().unwrap().is_none();
        stop(&mut child);
        assert!(format!("{:#}", result.unwrap_err()).contains("startup timed out"));
        assert!(still_running && f.lock().is_dir());
        assert!(!f.lock().join("pid").exists());
    }

    #[test]
    fn exited_child_requires_success_and_published_results() {
        for (code, results, success) in [(0, false, false), (9, true, false), (0, true, true)] {
            let f = Fixture::new();
            if results { std::fs::write(f.0.join("batch-results.jsonl"), "").unwrap(); }
            let mut child = Command::new("sh").args(["-c", &format!("exit {code}")]).spawn().unwrap();
            child.wait().unwrap();
            assert_eq!(await_worker_start(&f.0, &mut child, Duration::ZERO).is_ok(), success);
            assert!(f.lock().is_dir());
        }
    }

    #[test]
    fn preexisting_missing_malformed_and_stale_pids_still_fail_closed() {
        for text in [None, Some(""), Some("bad"), Some("4294967295")] {
            let f = Fixture::new();
            if let Some(text) = text { std::fs::write(f.lock().join("pid"), text).unwrap(); }
            assert!(run(&f.0, true).is_err(), "{text:?}");
            assert!(f.lock().is_dir());
            assert!(!f.0.join("codex-requests.jsonl").exists());
            assert!(!f.0.join("codex-results.partial.jsonl").exists());
        }
    }
}
