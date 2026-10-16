//! LLM (Opus) confirmation of disputed seed candidates via `claude -p` — like `math/src/vmm.rs`:
//! empty directory, no tools, effort via `--settings` env (the box-wide `settings.json` overrides
//! `--effort`), JSON with the cost. The API key comes from `~/Mova/local.md` (`api_env`) and is never printed. The cap is the sum
//! of `cost_usd` from the log plus the call reserve; exceeding it means refusal without a call (fail-fast, no retries).
//! The LLM answers "yes / no / refine + why"; this is a second opinion, not truth: into the core only after a human.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

/// API key (line `ANTHROPIC_API_KEY=…` in `~/Mova/local.md`); if absent, the call goes through the subscription.
fn api_env() -> Vec<(String, String)> {
    if std::env::var("MOVA_SUBSCRIPTION").as_deref() == Ok("1") {
        return Vec::new();
    }
    let Ok(home) = std::env::var("HOME") else { return Vec::new() };
    let text = std::fs::read_to_string(Path::new(&home).join("Mova/local.md")).unwrap_or_default();
    text.lines()
        .find_map(|l| l.strip_prefix("ANTHROPIC_API_KEY="))
        .map(str::trim)
        .filter(|k| !k.is_empty())
        .map(|k| vec![("ANTHROPIC_API_KEY".to_string(), k.to_string())])
        .unwrap_or_default()
}

/// Spent according to the log (JSON lines with `cost_usd`).
pub fn spent(log: &Path) -> f64 {
    std::fs::read_to_string(log)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter_map(|v| v["cost_usd"].as_f64())
        .sum()
}

/// One LLM call; returns the answer text and the cost.
pub fn ask(cwd: &PathBuf, log: &Path, cap: f64, reserve: f64, prompt: &str) -> Result<(String, f64)> {
    let s = spent(log);
    if s + reserve > cap {
        bail!("cap ${cap:.2}: spent ${s:.2}, reserve ${reserve:.2} — stopping");
    }
    std::fs::create_dir_all(cwd)?;
    let effort = std::env::var("ONTO_EFFORT").unwrap_or_else(|_| "medium".into());
    let t0 = std::time::Instant::now();
    let mut child = Command::new("claude")
        .args(["-p", "--model", "claude-haiku-5-5", "--effort", &effort, "--output-format", "json"])
        .args(["--settings", &format!("{{\"env\":{{\"CLAUDE_CODE_EFFORT_LEVEL\":\"{effort}\"}}}}")])
        .args(["--disallowedTools", "Bash,Edit,Write,Read,Glob,Grep,WebFetch,WebSearch,Agent,NotebookEdit"])
        .envs(api_env())
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("claude -p")?;
    child.stdin.take().context("stdin")?.write_all(prompt.as_bytes())?;
    let out = child.wait_with_output()?;
    if !out.status.success() {
        bail!("claude -p: {}", String::from_utf8_lossy(&out.stderr).trim());
    }
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).context("JSON from claude -p")?;
    let cost = v["total_cost_usd"].as_f64().unwrap_or(0.0);
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(log)?;
    writeln!(f, "{}", serde_json::json!({"step": "wd-confirm", "secs": t0.elapsed().as_secs_f64(), "cost_usd": cost, "input_tokens": v["usage"]["input_tokens"], "output_tokens": v["usage"]["output_tokens"]}))?;
    if v["is_error"].as_bool() == Some(true) {
        bail!("claude -p: {}", v["result"]);
    }
    Ok((v["result"].as_str().unwrap_or_default().to_string(), cost))
}

/// Candidates from `wd-candidates.tsv` for the LLM: object values with confidence in [lo, hi) — disputed 0.45–0.8 (inherited or with
/// a non-strict anchor) — up to `n`, evenly across properties. Returns the file's lines.
pub fn pick_disputed(tsv: &str, n: usize, lo: f64, hi: f64) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = tsv.lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').map(String::from).collect()).filter(|f: &Vec<String>| f.len() >= 13 && f[7].is_empty()).collect();
    rows.retain(|f| f[6].parse::<f64>().map(|c| (lo..hi).contains(&c)).unwrap_or(false));
    // round-robin over properties — so that the sample is not all about parts
    let mut by: std::collections::BTreeMap<String, Vec<Vec<String>>> = Default::default();
    for f in rows {
        by.entry(f[3].clone()).or_default().push(f);
    }
    let mut out = Vec::new();
    let mut i = 0;
    while out.len() < n && by.values().any(|v| v.len() > i) {
        for v in by.values() {
            if let Some(f) = v.get(i) {
                if out.len() < n {
                    out.push(f.clone());
                }
            }
        }
        i += 1;
    }
    out
}

pub fn prompt(rows: &[Vec<String>]) -> String {
    let mut s = String::from(
        "You check candidate common-sense facts for a world model (real world, not fairy tales). Each line: id, concept (English lemma, WordNet sense), property, value (from Wikidata), where it came from.\n\
Properties: food = what it eats; habitat = where it lives; part = it has this part; no_part = it lacks this part; material = made of; color; use = what it is for.\n\
For EACH id answer one line exactly: id<TAB>yes|no|refine<TAB>why (max 12 words; for refine — the better value).\n\
Answer yes only if a typical instance of the concept has it (default knowledge); inherited from a too general class or wrong sense → no.\n\n",
    );
    for (i, f) in rows.iter().enumerate() {
        s.push_str(&format!("{}\t{} ({})\t{}\t{}\t{}\n", i + 1, f[0], f[2], f[3], f[4], f[5]));
    }
    s
}

/// Parse the answer: id → (verdict, why).
pub fn parse(ans: &str, n: usize) -> Vec<Option<(String, String)>> {
    let mut v = vec![None; n];
    for l in ans.lines() {
        let f: Vec<&str> = l.split('\t').map(str::trim).collect();
        if f.len() < 2 {
            continue;
        }
        let Ok(i) = f[0].trim_start_matches('#').parse::<usize>() else { continue };
        let verdict = match f[1].to_ascii_lowercase().as_str() {
            "yes" => "yes",
            "no" => "no",
            "refine" => "refine",
            _ => continue,
        };
        if (1..=n).contains(&i) {
            v[i - 1] = Some((verdict.to_string(), f.get(2).unwrap_or(&"").to_string()));
        }
    }
    v
}

#[cfg(test)]
mod t {
    #[test]
    fn parse_answer() {
        let v = super::parse("1\tyes\tcrows eat insects\n2\tNO\ttoo general\ngarbage\n3\trefine\tfeathers, not scales\n9\tyes\textra", 3);
        assert_eq!(v[0].as_ref().unwrap().0, "yes");
        assert_eq!(v[1].as_ref().unwrap().0, "no");
        assert_eq!(v[2].as_ref().unwrap().0, "refine");
    }
}
