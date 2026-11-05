//! CLI for bounded native jobs; concrete tasks and runtime state stay external.
use anyhow::{Context, Result, bail};
use std::{
    io::{self, Write},
    path::Path,
    thread,
    time::Duration,
};
#[derive(Default)]
struct WatchReporter {
    last_health: Option<String>,
}
impl WatchReporter {
    fn next(&mut self, report: &agent_runner::Summary) -> Result<Option<String>> {
        let mut health = serde_json::to_value(report)?;
        // Work-count decay on the next idle pass is not a health transition.
        health
            .as_object_mut()
            .context("watch summary must serialize to an object")?
            .remove("attempted_this_pass");
        let signature = serde_json::to_string(&health)?;
        if self.last_health.as_ref() != Some(&signature) || report.attempted_this_pass > 0 {
            let line = serde_json::to_string(report)?;
            self.last_health = Some(signature);
            Ok(Some(line))
        } else {
            Ok(None)
        }
    }
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "example") && args.len() == 3 {
        return agent_runner::example(Path::new(&args[1]), Path::new(&args[2]));
    }
    if args.len() != 2 {
        bail!(
            "agent-runner example CHECKPOINT_DIR NEW_DIR | once CONFIG | watch CONFIG | status CONFIG"
        );
    }
    let path = Path::new(&args[1]);
    match args[0].as_str() {
        "status" => println!("{}", serde_json::to_string(&agent_runner::status(path)?)?),
        "once" => {
            let runner = agent_runner::Runner::open(path)?;
            println!("{}", serde_json::to_string(&runner.pass()?)?);
        }
        "watch" => {
            let runner = agent_runner::Runner::open(path)?;
            let mut reporter = WatchReporter::default();
            loop {
                let report = runner.pass()?;
                if let Some(line) = reporter.next(&report)? {
                    let mut output = io::stdout().lock();
                    writeln!(output, "{line}")?;
                    output.flush()?;
                }
                if report.stopped {
                    break;
                }
                thread::sleep(Duration::from_millis(runner.poll_ms()));
            }
        }
        _ => bail!("unknown runner operation"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_runner::{
        Config, InputPin, Job, Operation, Queue, RecallIntent, RecallParent, Runner, State, Summary,
    };
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct CachedFixture {
        root: PathBuf,
        config: PathBuf,
        report: PathBuf,
    }
    impl CachedFixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "mova-watch-health-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            let job = Job {
                id: "cached".into(),
                operation: Operation::Recall {
                    intent: RecallIntent {
                        actor: "reader".into(),
                        subject: "apple".into(),
                        locations: vec![
                            "garden".into(),
                            "kitchen".into(),
                            "office".into(),
                            "hallway".into(),
                        ],
                        parent: RecallParent::PinnedCheckpoint {
                            state: InputPin {
                                path: "unused-state.json".into(),
                                sha256: "0".repeat(64),
                            },
                            report: InputPin {
                                path: "unused-report.json".into(),
                                sha256: "0".repeat(64),
                            },
                        },
                    },
                },
            };
            let job_hash = agent_mova::digest(&serde_json::to_vec(&job).unwrap());
            let queue = Queue {
                schema: 1,
                jobs: vec![job],
            };
            let queue_bytes = serde_json::to_vec_pretty(&queue).unwrap();
            let queue_hash = agent_mova::digest(&queue_bytes);
            let qp = root.join("queue.json");
            fs::write(&qp, queue_bytes).unwrap();
            let config = root.join("config.json");
            let c = Config {
                schema: 1,
                run_root: root.join("run"),
                queue: qp,
                queue_sha256: queue_hash.clone(),
                max_jobs_per_pass: 1,
                poll_ms: 100,
                allow_trace: false,
                input_root: Some(root.clone()),
            };
            fs::write(&config, serde_json::to_vec_pretty(&c).unwrap()).unwrap();
            drop(Runner::open(&config).unwrap());
            // Cached receipt metadata only: no native recall, trace, model or training is invoked.
            let dir = root.join("run/jobs/cached");
            fs::create_dir_all(dir.join("result")).unwrap();
            let report = dir.join("result/report.json");
            fs::write(&report, b"{}").unwrap();
            let reservation = serde_json::json!({"schema":1,"id":"cached","operation":"recall","job_sha256":job_hash,"queue_sha256":queue_hash,"worker_pid":0});
            let receipt = serde_json::json!({"schema":1,"id":"cached","operation":"recall","job_sha256":job_hash,"queue_sha256":queue_hash,"completed":true,"native_outcome":"uncertain","error":null,"partial_native_effects_possible":false,"artifacts":{"result/report.json":agent_mova::digest(b"{}")}});
            fs::write(
                dir.join("reservation.json"),
                serde_json::to_vec(&reservation).unwrap(),
            )
            .unwrap();
            fs::write(
                dir.join("receipt.json"),
                serde_json::to_vec(&receipt).unwrap(),
            )
            .unwrap();
            Self {
                root,
                config,
                report,
            }
        }
        fn pass(&self, runner: &Runner) -> Summary {
            let s = runner.pass().unwrap();
            assert_eq!(s.attempted_this_pass, 0);
            assert!(!self.root.join("run/jobs/cached/result/state.json").exists());
            s
        }
    }
    impl Drop for CachedFixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    #[test]
    fn watch_reports_actual_integrity_transition_and_recovery_once() {
        let f = CachedFixture::new();
        let runner = Runner::open(&f.config).unwrap();
        let mut gate = WatchReporter::default();
        let healthy = f.pass(&runner);
        assert_eq!(healthy.jobs[0].state, State::Completed);
        assert!(gate.next(&healthy).unwrap().is_some());
        assert!(gate.next(&f.pass(&runner)).unwrap().is_none());
        fs::write(&f.report, b"changed").unwrap();
        let bad = f.pass(&runner);
        assert_eq!(bad.jobs[0].state, State::IntegrityFailure);
        let line = gate
            .next(&bad)
            .unwrap()
            .expect("health transition must be emitted even with zero work");
        assert!(line.contains("integrity_failure"));
        for _ in 0..3 {
            assert!(gate.next(&f.pass(&runner)).unwrap().is_none());
        }
        fs::write(&f.report, b"{}").unwrap();
        let recovered = f.pass(&runner);
        assert_eq!(recovered.jobs[0].state, State::Completed);
        assert!(gate.next(&recovered).unwrap().is_some());
        assert!(gate.next(&f.pass(&runner)).unwrap().is_none());
        let mut restarted = WatchReporter::default();
        assert!(restarted.next(&recovered).unwrap().is_some());
    }
    #[test]
    fn watch_ignores_idle_counter_decay_but_keeps_work_events() {
        let f = CachedFixture::new();
        let runner = Runner::open(&f.config).unwrap();
        let mut s = f.pass(&runner);
        let mut gate = WatchReporter::default();
        s.attempted_this_pass = 1;
        assert!(gate.next(&s).unwrap().is_some());
        s.attempted_this_pass = 0;
        for _ in 0..3 {
            assert!(gate.next(&s).unwrap().is_none());
        }
        s.attempted_this_pass = 1;
        assert!(gate.next(&s).unwrap().is_some());
    }
    #[test]
    fn watch_preserves_other_meaningful_fields_and_stop() {
        let f = CachedFixture::new();
        let runner = Runner::open(&f.config).unwrap();
        let mut s = f.pass(&runner);
        let mut gate = WatchReporter::default();
        assert!(gate.next(&s).unwrap().is_some());
        s.jobs[0].native_outcome = Some(agent_mova::Outcome::Supported);
        assert!(gate.next(&s).unwrap().is_some());
        assert!(gate.next(&s).unwrap().is_none());
        s.jobs[0].detail = Some("changed diagnostic".into());
        assert!(gate.next(&s).unwrap().is_some());
        assert!(gate.next(&s).unwrap().is_none());
        s.semantic_admission = true;
        assert!(gate.next(&s).unwrap().is_some());
        s.stopped = true;
        assert!(gate.next(&s).unwrap().is_some());
        assert!(gate.next(&s).unwrap().is_none());
    }
}
