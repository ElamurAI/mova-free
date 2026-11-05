//! The topic corpus in the learning loop (optional, by configuration): a Whale's paper gets a closed
//! topic corpus of its neighbour papers and a small paper world when it is assigned, the corpus
//! explains the paper's terms natively before the Whale asks its terminal (seeding its knowledge
//! between rounds), and every round's description is also written with citations and a
//! bibliography.
//!
//! The mechanism is the public `mova-topic-corpus` (`tools/topic-corpus`), which depends on this
//! crate; so the Whale runs its frozen binary as a child process (`build`, `world`, `seed`, `cite`)
//! with the deployment's configuration, never links it. A child's failure is journaled and never
//! stops learning; nothing is retried within a round. `seed` writes the Whale's knowledge store
//! and refuses while `learner.lock` is held, so the learner releases its lock for the seed and
//! takes it again (a lost lock stops the learner: `lock_lost`).
use crate::util::error;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TopicCorpus {
    /// The frozen `mova-topic-corpus` binary (absolute).
    pub executable: PathBuf,
    /// Its configuration (`mova-topic-corpus-v1`, absolute).
    pub config: PathBuf,
    /// Build the corpus and the paper world when a batch assigns the paper.
    #[serde(default = "d_on")]
    pub build_on_assign: bool,
    /// Seed term facts from the corpus before each round's questions.
    #[serde(default = "d_on")]
    pub seed: bool,
    /// Write `cited.md` (description with citations and a bibliography) for every round.
    #[serde(default = "d_on")]
    pub cite: bool,
    /// Cite in public mode (refuses web pages, drafts and closed papers as sources).
    #[serde(default)]
    pub public: bool,
    /// A child running longer is killed and journaled as a timeout.
    #[serde(default = "d_timeout")]
    pub timeout_seconds: u64,
}
fn d_on() -> bool {
    true
}
fn d_timeout() -> u64 {
    1800
}

impl TopicCorpus {
    pub fn validate(&self) -> Result<()> {
        if !self.executable.is_absolute() || !self.config.is_absolute() || !(10..=86_400).contains(&self.timeout_seconds) {
            return Err(error("topic_corpus: absolute executable and config, timeout_seconds 10..86400"));
        }
        Ok(())
    }

    /// Run one `mova-topic-corpus` command; its JSON output, or the failure, as a journal value.
    pub fn run(&self, cmd: &str, args: &[&str]) -> Value {
        let started = Instant::now();
        let mut child = match Command::new(&self.executable).arg(cmd).arg(&self.config).args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
            Ok(c) => c,
            Err(e) => return json!({"command":cmd,"ok":false,"error":format!("cannot start {}: {e}", self.executable.display())}),
        };
        // Drain both pipes on threads so a large output cannot block the child.
        let mut out = child.stdout.take();
        let mut err = child.stderr.take();
        let o = std::thread::spawn(move || {
            let mut s = String::new();
            if let Some(p) = out.as_mut() {
                let _ = p.read_to_string(&mut s);
            }
            s
        });
        let e = std::thread::spawn(move || {
            let mut s = String::new();
            if let Some(p) = err.as_mut() {
                let _ = p.read_to_string(&mut s);
            }
            s
        });
        let limit = Duration::from_secs(self.timeout_seconds);
        let status = loop {
            match child.try_wait() {
                Ok(Some(s)) => break Some(s),
                Ok(None) if started.elapsed() > limit => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break None;
                }
                Ok(None) => std::thread::sleep(Duration::from_millis(100)),
                Err(_) => break None,
            }
        };
        let stdout = o.join().unwrap_or_default();
        let stderr = e.join().unwrap_or_default();
        let tail: String = stderr.chars().rev().take(600).collect::<Vec<_>>().into_iter().rev().collect();
        let ms = started.elapsed().as_millis() as u64;
        match status {
            Some(s) if s.success() => json!({"command":cmd,"ok":true,"ms":ms,"result":serde_json::from_str::<Value>(&stdout).unwrap_or(json!(stdout.chars().take(2000).collect::<String>()))}),
            Some(s) => json!({"command":cmd,"ok":false,"ms":ms,"exit":s.code(),"stderr":tail}),
            None => json!({"command":cmd,"ok":false,"ms":ms,"timeout":true,"stderr":tail}),
        }
    }

    /// The corpus and the paper world of a newly assigned paper.
    pub fn build(&self, paper: &str) -> Value {
        let b = self.run("build", &[paper]);
        let w = if b["ok"] == true { self.run("world", &[paper]) } else { json!({"command":"world","ok":false,"skipped":"build failed"}) };
        json!({"paper":paper,"build":b,"world":w})
    }

    /// Term facts from the corpus into the Whale's knowledge (the caller has released its lock).
    pub fn seed(&self, paper: &str, whale: &str, state_root: &Path) -> Value {
        self.run("seed", &[paper, whale, &state_root.display().to_string()])
    }

    /// The round's description with citations and a bibliography, written to `out`.
    pub fn cite(&self, paper: &str, round: &Path, state_root: &Path, whale: &str, out: &Path) -> Value {
        let (r, s, o) = (round.display().to_string(), state_root.display().to_string(), out.display().to_string());
        let mut args = vec![paper, r.as_str(), s.as_str(), whale, o.as_str()];
        if self.public {
            args.push("--public");
        }
        self.run("cite", &args)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    /// A fake `mova-topic-corpus`: a shell script that logs its arguments, answers JSON, and for
    /// `seed` checks that the learner's lock is free (`flock -n`), as the real seed does.
    pub fn fake(dir: &Path) -> TopicCorpus {
        let exe = dir.join("fake-topic-corpus.sh");
        let log = dir.join("topic-calls.log");
        let script = format!(
            "#!/bin/bash\necho \"$*\" >> '{}'\ncase \"$1\" in\n  seed) exec 9>\"$5/learner.lock\"; flock -n 9 || {{ echo held >&2; exit 3; }}; echo '{{\"seeded\":[]}}';;\n  cite) echo '# cited' > \"$7\"; echo '{{\"sentences\":1}}';;\n  fail) exit 2;;\n  *) echo '{{\"ok\":1}}';;\nesac\n",
            log.display()
        );
        std::fs::write(&exe, script).unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&exe, std::fs::Permissions::from_mode(0o755)).unwrap();
        let cfg = dir.join("topic.json");
        std::fs::write(&cfg, "{}").unwrap();
        TopicCorpus { executable: exe, config: cfg, build_on_assign: true, seed: true, cite: true, public: false, timeout_seconds: 60 }
    }

    #[test]
    fn children_report_success_failure_and_timeouts() {
        let d = crate::learn::tests::tmp("topic-run");
        let t = fake(&d);
        let b = t.build("p-1");
        assert_eq!((b["build"]["ok"].clone(), b["world"]["ok"].clone()), (json!(true), json!(true)), "{b}");
        let f = t.run("fail", &[]);
        assert_eq!((f["ok"].clone(), f["exit"].clone()), (json!(false), json!(2)));
        let mut slow = t.clone();
        slow.executable = PathBuf::from("/bin/sleep");
        slow.config = PathBuf::from("30");
        slow.timeout_seconds = 10;
        assert!(slow.validate().is_err(), "a relative config is refused");
        let _ = std::fs::remove_dir_all(d);
    }
}
