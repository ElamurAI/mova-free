//! DuckDB through the CLI (`~/bin/duckdb`, variable `DUCKDB`). The SQL script goes to stdin with `-bail`: the first error
//! stops the script, exit code ≠ 0 — an error with the stderr text. Data travels as TSV files (COPY) in the stage
//! directory (`Stage`), not through stdout; stdout is only for human reports (`db stats`, `db sql`).

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

pub struct Duck {
    pub bin: PathBuf,
    pub db: PathBuf,
}

impl Duck {
    pub fn open(db: &Path) -> Duck {
        let bin = std::env::var("DUCKDB").map(PathBuf::from).unwrap_or_else(|_| crate::util::home().join("bin/duckdb"));
        Duck { bin, db: db.to_path_buf() }
    }

    /// Run a script; `readonly` — without write access (several readers can work at once).
    /// `mode` — CLI output mode (`markdown`, `box`…) for human reports.
    pub fn run(&self, sql: &str, readonly: bool, mode: Option<&str>) -> Result<String> {
        let mut cmd = Command::new(&self.bin);
        cmd.arg("-bail").arg("-batch");
        if readonly {
            cmd.arg("-readonly");
        }
        if let Some(m) = mode {
            cmd.arg(format!("-{m}"));
        }
        cmd.arg(&self.db).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = cmd.spawn().with_context(|| format!("cannot start {} (DUCKDB)", self.bin.display()))?;
        child.stdin.take().expect("stdin").write_all(sql.as_bytes())?;
        let out = child.wait_with_output()?;
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("Error:") {
            bail!("duckdb {}: {}", self.db.display(), stderr.trim());
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }

    pub fn exec(&self, sql: &str) -> Result<()> {
        self.run(sql, false, None).map(|_| ())
    }

    pub fn read(&self, sql: &str) -> Result<()> {
        self.run(sql, true, None).map(|_| ())
    }
}

/// Temporary stage directory: TSV for COPY in both directions. Removed after the work; `DB_KEEP=1` keeps
/// it for inspection.
pub struct Stage {
    pub dir: PathBuf,
}

impl Stage {
    pub fn new(tag: &str) -> Result<Stage> {
        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("mova-db-{}-{tag}-{n}", std::process::id()));
        if dir.exists() {
            std::fs::remove_dir_all(&dir)?;
        }
        std::fs::create_dir_all(&dir).with_context(|| format!("{}", dir.display()))?;
        Ok(Stage { dir })
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    /// Substitute the stage directory into SQL: `{dir}` / `{q}` → path (no quotes in the path — otherwise an error).
    pub fn sql(&self, template: &str, key: &str) -> Result<String> {
        let d = self.dir.to_string_lossy();
        if d.contains('\'') {
            bail!("stage path contains a quote: {d}");
        }
        Ok(template.replace(&format!("{{{key}}}"), &d))
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        if std::env::var("DB_KEEP").is_err() {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
}

/// SQL string in quotes: "'" → "''".
pub fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}
