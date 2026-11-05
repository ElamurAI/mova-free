//! Bounded argv-only tmux adapter. It never interprets model output as shell commands.
use crate::{Config, Fish, Screen, Terminal, read_limited};
use anyhow::{Context, Result, ensure};
use std::{
    fs::{self, OpenOptions},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
pub struct Tmux {
    pub binary: PathBuf,
    pub temporary: PathBuf,
    pub timeout: Duration,
}
impl Tmux {
    pub fn new(c: &Config) -> Self {
        Self {
            binary: c.tmux.clone(),
            temporary: c.root.join("tmp"),
            timeout: Duration::from_secs(5),
        }
    }
    pub fn run(&self, args: &[&str]) -> Result<String> {
        let tag = format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        );
        let out = self.temporary.join(format!("{tag}.out"));
        let err = self.temporary.join(format!("{tag}.err"));
        let mut paths = Cleanup(vec![out.clone(), err.clone()]);
        let a = OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(&out)?;
        let b = OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(&err)?;
        let mut child = Command::new(&self.binary)
            .args(args)
            .stdin(Stdio::null())
            .stdout(a)
            .stderr(b)
            .spawn()
            .context("start tmux client")?;
        let start = Instant::now();
        let status = loop {
            if let Some(s) = child.try_wait()? {
                break s;
            }
            if start.elapsed() >= self.timeout {
                let _ = child.kill();
                let _ = child.wait();
                anyhow::bail!("tmux client timed out; terminal effects may be uncertain");
            }
            thread::sleep(Duration::from_millis(5));
        };
        let bytes = read_limited(&out, 32768)?;
        let diagnostic = read_limited(&err, 2048)?;
        ensure!(
            status.success(),
            "tmux client failed (stderr SHA-256 {}; content withheld)",
            crate::hash(&diagnostic)
        );
        paths.0.clear();
        let _ = fs::remove_file(out);
        let _ = fs::remove_file(err);
        Ok(String::from_utf8(bytes)?)
    }
    fn text<'a>(p: &'a Path) -> Result<&'a str> {
        p.to_str().context("UTF-8 path")
    }
}
struct Cleanup(Vec<PathBuf>);
impl Drop for Cleanup {
    fn drop(&mut self) {
        for p in &self.0 {
            let _ = fs::remove_file(p);
        }
    }
}
// "=" disables tmux prefix matching. The colon selects a pane in that exact session.
pub(crate) fn session_target(f: &Fish) -> String {
    format!("={}", f.session)
}
pub(crate) fn pane_target(f: &Fish) -> String {
    format!("={}:", f.session)
}
impl Terminal for Tmux {
    fn launch(&mut self, c: &Config, f: &Fish, owner: &str) -> Result<()> {
        // Exact session names are new. A refusal preserves the launch reservation for inspection.
        self.run(&[
            "new-session",
            "-d",
            "-s",
            &f.session,
            "-c",
            Self::text(&f.workdir)?,
            "-x",
            "120",
            "-y",
            "40",
            Self::text(&c.codex)?,
            "--no-alt-screen",
            "--no-daemon",
            "-m",
            &c.model,
            "-c",
            "model_reasoning_effort=\"medium\"",
            "-s",
            "workspace-write",
            "-a",
            "never",
            "-C",
            Self::text(&f.workdir)?,
        ])?;
        self.run(&[
            "set-option",
            "-t",
            &session_target(f),
            "@peta-fish-owner",
            owner,
        ])?;
        Ok(())
    }
    fn owned(&mut self, f: &Fish, owner: &str) -> Result<bool> {
        let s = self.run(&[
            "show-options",
            "-v",
            "-t",
            &session_target(f),
            "@peta-fish-owner",
        ])?;
        let panes = self.run(&[
            "list-panes",
            "-s",
            "-t",
            &session_target(f),
            "-F",
            "#{pane_id}",
        ])?;
        Ok(s.trim() == owner && panes.lines().filter(|l| !l.trim().is_empty()).count() == 1)
    }
    fn capture(&mut self, f: &Fish) -> Result<Screen> {
        let meta = self.run(&[
            "display-message",
            "-p",
            "-t",
            &pane_target(f),
            "#{pane_current_command}|#{pane_dead}",
        ])?;
        let (command, dead) = meta.trim().split_once('|').context("pane metadata")?;
        ensure!(matches!(dead, "0" | "1"), "unknown pane state");
        let text = self.run(&["capture-pane", "-p", "-t", &pane_target(f), "-S", "-35"])?;
        Ok(Screen {
            command: command.into(),
            dead: dead == "1",
            text,
        })
    }
    fn load(&mut self, buffer: &str, p: &Path) -> Result<()> {
        self.run(&["load-buffer", "-b", buffer, Self::text(p)?])?;
        Ok(())
    }
    fn paste(&mut self, f: &Fish, buffer: &str) -> Result<()> {
        self.run(&[
            "paste-buffer",
            "-p",
            "-d",
            "-b",
            buffer,
            "-t",
            &pane_target(f),
        ])?;
        Ok(())
    }
    fn enter(&mut self, f: &Fish) -> Result<()> {
        self.run(&["send-keys", "-t", &pane_target(f), "Enter"])?;
        Ok(())
    }
}
