//! LLM (Opus) via `claude -p` — the same approach as in: the call runs in an empty
//! directory (no project context), without tools, effort via `--settings` (the box-wide
//! `settings.json` sets `CLAUDE_CODE_EFFORT_LEVEL` and overrides `--effort`), output is JSON with
//! the result, time and tokens. Here it is a generic call not tied to UD: the prompt and answer parsing
//! are done by the caller. Accounting — one `calls.jsonl` line per call, fields as in `annot` plus batch and
//! cache write.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

/// Accounting of one call (like `annot::llm::Call`, plus `batch` and `cache_write_tokens`).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Call {
    pub step: String,
    #[serde(default)]
    pub batch: usize,
    pub sentences: usize,
    pub secs: f64,
    /// estimate from `total_cost_usd`; claude -p runs on the subscription — we report tokens and time
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    #[serde(default)]
    pub cache_write_tokens: u64,
}

pub struct Opus {
    pub model: String,
    pub effort: String,
    /// empty directory for the call
    pub cwd: PathBuf,
}

impl Opus {
    /// Model and effort from variables (`PRAG_MODEL`, `PRAG_EFFORT`); by default Opus at medium —
    /// Mova's sub-agents run at medium (AGENTS.md).
    pub fn from_env(cwd: PathBuf) -> Opus {
        Opus {
            model: std::env::var("PRAG_MODEL").unwrap_or_else(|_| "claude-opus-5-5".into()),
            effort: std::env::var("PRAG_EFFORT").unwrap_or_else(|_| "medium".into()),
            cwd,
        }
    }

    /// One call: prompt on stdin, the answer is the result text plus accounting. A process error or
    /// `is_error` is an error (fail-fast: the caller does not retry blindly).
    pub fn ask(&self, step: &str, batch: usize, n: usize, prompt: &str) -> Result<(String, Call)> {
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
        if v["is_error"].as_bool() == Some(true) {
            bail!("claude -p ({step} {batch}): {}", v["result"]);
        }
        let u = &v["usage"];
        let tok = |k: &str| u[k].as_u64().unwrap_or(0);
        let call = Call {
            step: step.to_string(),
            batch,
            sentences: n,
            secs: t0.elapsed().as_secs_f64(),
            cost_usd: v["total_cost_usd"].as_f64().unwrap_or(0.0),
            input_tokens: tok("input_tokens"),
            output_tokens: tok("output_tokens"),
            cache_read_tokens: tok("cache_read_input_tokens"),
            cache_write_tokens: tok("cache_creation_input_tokens"),
        };
        Ok((v["result"].as_str().unwrap_or_default().to_string(), call))
    }
}

/// Append the call accounting to `calls.jsonl` (right after the call — an interrupted run does not lose accounting).
pub fn append_call(path: &Path, c: &Call) -> Result<()> {
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path).with_context(|| path.display().to_string())?;
    writeln!(f, "{}", serde_json::to_string(c)?)?;
    Ok(())
}

/// Read the accounting.
pub fn read_calls(path: &Path) -> Result<Vec<Call>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    std::fs::read_to_string(path)?.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect()
}

/// API key for `claude -p` (design note: calls go through the API from now on): a line
/// `ANTHROPIC_API_KEY=…` in `local.md (or MOVA_LOCAL)` (outside git, under `secret-guard.sh`). No line, or
/// `MOVA_SUBSCRIPTION=1` — the call goes through the subscription, as before. The key is never printed.
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
