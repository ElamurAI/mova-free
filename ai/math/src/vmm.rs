//! v2: LLM (Opus) via `claude -p` — the same approach as in and `prag/src/opus.rs`:
//! an empty directory (no project context), no tools, effort via `--settings` (the box-wide
//! `settings.json` sets `CLAUDE_CODE_EFFORT_LEVEL` and overrides `--effort`), output — JSON with
//! the result, time, tokens and cost. API key — from `local.md (or MOVA_LOCAL)` (`api_env`), never printed.
//! Spending cap: before each call — the sum of `cost_usd` from the wave log plus reserve ≤ cap.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Call {
    pub step: String,
    #[serde(default)]
    pub batch: usize,
    /// problems in the call
    pub problems: usize,
    pub secs: f64,
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    #[serde(default)]
    pub cache_write_tokens: u64,
}

pub struct Vmm {
    pub model: String,
    pub effort: String,
    pub cwd: PathBuf,
    /// call log of the whole wave (for the cap)
    pub log: PathBuf,
    pub cap_usd: f64,
    spent: Mutex<f64>,
}

impl Vmm {
    pub fn new(cwd: PathBuf, log: PathBuf, cap_usd: f64) -> Result<Vmm> {
        let spent = read_calls(&log)?.iter().map(|c| c.cost_usd).sum();
        Ok(Vmm {
            model: std::env::var("MATH_MODEL").unwrap_or_else(|_| "claude-opus-5-5".into()),
            effort: std::env::var("MATH_EFFORT").unwrap_or_else(|_| "medium".into()),
            cwd,
            log,
            cap_usd,
            spent: Mutex::new(spent),
        })
    }

    pub fn spent(&self) -> f64 {
        *self.spent.lock().unwrap()
    }

    /// One call. `reserve` — estimate of the most expensive call: if spent + reserve > cap — refuse
    /// without calling (fail-fast, no retries).
    pub fn ask(&self, step: &str, batch: usize, n: usize, prompt: &str, reserve: f64) -> Result<(String, Call)> {
        {
            let s = self.spent.lock().unwrap();
            if *s + reserve > self.cap_usd {
                bail!("cap ${:.2}: spent ${:.2}, call reserve ${:.2} — stopping", self.cap_usd, *s, reserve);
            }
        }
        std::fs::create_dir_all(&self.cwd)?;
        let t0 = Instant::now();
        let mut child = Command::new("claude")
            .args(["-p", "--model", &self.model, "--effort", &self.effort, "--output-format", "json"])
            .args(["--settings", &format!("{{\"env\":{{\"CLAUDE_CODE_EFFORT_LEVEL\":\"{}\"}}}}", self.effort)])
            .args(["--disallowedTools", "Bash,Edit,Write,Read,Glob,Grep,WebFetch,WebSearch,Agent,NotebookEdit"])
            .envs(api_env())
            .current_dir(&self.cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("claude -p")?;
        child.stdin.take().context("stdin")?.write_all(prompt.as_bytes())?;
        let out = child.wait_with_output()?;
        if !out.status.success() {
            bail!("claude -p ({step} {batch}): {} {}", String::from_utf8_lossy(&out.stderr).trim(), String::from_utf8_lossy(&out.stdout).chars().take(400).collect::<String>());
        }
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).context("JSON from claude -p")?;
        let u = &v["usage"];
        let tok = |k: &str| u[k].as_u64().unwrap_or(0);
        let call = Call {
            step: step.to_string(),
            batch,
            problems: n,
            secs: t0.elapsed().as_secs_f64(),
            cost_usd: v["total_cost_usd"].as_f64().unwrap_or(0.0),
            input_tokens: tok("input_tokens"),
            output_tokens: tok("output_tokens"),
            cache_read_tokens: tok("cache_read_input_tokens"),
            cache_write_tokens: tok("cache_creation_input_tokens"),
        };
        *self.spent.lock().unwrap() += call.cost_usd;
        append_call(&self.log, &call)?;
        if v["is_error"].as_bool() == Some(true) {
            bail!("claude -p ({step} {batch}): {}", v["result"]);
        }
        Ok((v["result"].as_str().unwrap_or_default().to_string(), call))
    }
}

pub fn append_call(path: &Path, c: &Call) -> Result<()> {
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path).with_context(|| path.display().to_string())?;
    writeln!(f, "{}", serde_json::to_string(c)?)?;
    Ok(())
}

pub fn read_calls(path: &Path) -> Result<Vec<Call>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    std::fs::read_to_string(path)?.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect()
}

/// API key for `claude -p` ("let them work through the API from now on"): the line
/// `ANTHROPIC_API_KEY=…` in `local.md (or MOVA_LOCAL)` (outside git, guarded by `secret-guard.sh`). No line or
/// `MOVA_SUBSCRIPTION=1` — the call goes through the subscription. The key is never printed.
fn api_env() -> Vec<(String, String)> {
    if std::env::var("MOVA_SUBSCRIPTION").as_deref() == Ok("1") {
        return Vec::new();
    }
    let text = std::fs::read_to_string(std::env::var("MOVA_LOCAL").unwrap_or_else(|_| "local.md".into())).unwrap_or_default();
    text.lines()
        .find_map(|l| l.strip_prefix("ANTHROPIC_API_KEY="))
        .map(str::trim)
        .filter(|k| !k.is_empty())
        .map(|k| vec![("ANTHROPIC_API_KEY".to_string(), k.to_string())])
        .unwrap_or_default()
}
