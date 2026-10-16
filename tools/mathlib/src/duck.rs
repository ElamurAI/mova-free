//! DuckDB via the CLI (`~/bin/duckdb`, variable `DUCKDB`), as in `tools/db`: script on stdin with `-bail`,
//! an error is a non-zero exit code plus stderr text. The math catalog is a separate database `math.duckdb`.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

pub fn run(db: &Path, sql: &str, mode: Option<&str>) -> Result<String> {
    let bin = std::env::var("DUCKDB").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(std::env::var("HOME").unwrap_or_default()).join("bin/duckdb")
    });
    let mut cmd = Command::new(&bin);
    cmd.arg("-bail").arg("-batch");
    if let Some(m) = mode {
        cmd.arg(format!("-{m}"));
    }
    cmd.arg(db).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().with_context(|| format!("cannot start {}", bin.display()))?;
    child.stdin.take().context("stdin")?.write_all(sql.as_bytes())?;
    let out = child.wait_with_output()?;
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() || stderr.contains("Error:") {
        bail!("duckdb {}: {}", db.display(), stderr.trim());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// SQL template: `{name}` → value (quotes in paths are escaped).
pub fn fill(sql: &str, vars: &[(&str, &str)]) -> String {
    vars.iter().fold(sql.to_string(), |s, (k, v)| s.replace(&format!("{{{k}}}"), &v.replace('\'', "''")))
}
