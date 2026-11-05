//! Teacher calls through the shared Codex CLI transport (mova-teacher).
//! Existing result/accounting formats remain readable; subscription dollar cost is not reported.

use std::io::Write as _;
use std::path::{Path, PathBuf};
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
            model: mova_teacher::model("MATH_MODEL"),
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
        let v = mova_teacher::ask(&self.cwd, &self.model, &self.effort, prompt)?;
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

#[cfg(test)]
mod wave28 {
    use super::*;
    #[test]
    fn accounting_and_cap_before_effect() {
        let dir = std::env::temp_dir().join(format!("mova-wave28-vmm-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir(&dir).unwrap();
        let log = dir.join("calls");
        assert!(read_calls(&log).unwrap().is_empty());
        append_call(
            &log,
            &Call {
                cost_usd: 2.0,
                ..Default::default()
            },
        )
        .unwrap();
        let v = Vmm::new(dir.join("not-created"), log.clone(), 2.5).unwrap();
        assert_eq!(v.spent(), 2.0);
        assert!(
            v.ask("synthetic", 0, 0, "", 1.0)
                .unwrap_err()
                .to_string()
                .contains("cap")
        );
        assert!(!v.cwd.exists());
        assert_eq!(read_calls(&log).unwrap().len(), 1);
        std::fs::write(&log, "not json").unwrap();
        assert!(read_calls(&log).is_err());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
