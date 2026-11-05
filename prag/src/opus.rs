//! Teacher calls through the shared Codex CLI transport (mova-teacher).
//! Existing result/accounting formats remain readable; subscription dollar cost is not reported.
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Accounting of one call; historical dollar estimates remain readable.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Call {
    pub step: String,
    #[serde(default)]
    pub batch: usize,
    pub sentences: usize,
    pub secs: f64,
    /// Historical estimate; Codex subscription calls report zero, not a dollar price.
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    #[serde(default)]
    pub cache_write_tokens: u64,
}

/// Compatibility name for the shared Codex teacher configuration.
pub struct Opus {
    pub model: String,
    pub effort: String,
    /// Directory containing isolated teacher calls.
    pub cwd: PathBuf,
}
impl Opus {
    /// Shared Codex model selection; PRAG_EFFORT defaults to medium.
    pub fn from_env(cwd: PathBuf) -> Opus {
        Opus {
            model: mova_teacher::model("PRAG_MODEL"),
            effort: std::env::var("PRAG_EFFORT").unwrap_or_else(|_| "medium".into()),
            cwd,
        }
    }
    /// The shared transport validates process success and completed Codex events.
    /// No retry is performed here.
    pub fn ask(&self, step: &str, batch: usize, n: usize, prompt: &str) -> Result<(String, Call)> {
        std::fs::create_dir_all(&self.cwd)?;
        let t0 = Instant::now();
        let v = mova_teacher::ask(&self.cwd, &self.model, &self.effort, prompt)?;
        Ok(interpret_reply(
            step,
            batch,
            n,
            t0.elapsed().as_secs_f64(),
            &v,
        ))
    }
}

/// Pure accounting projection of a supplied, already validated teacher reply.
/// This does not validate event lifecycle or execute a teacher. Missing or wrongly
/// typed compatibility fields retain their historical zero/empty defaults.
pub fn interpret_reply(
    step: &str,
    batch: usize,
    n: usize,
    secs: f64,
    v: &serde_json::Value,
) -> (String, Call) {
    let u = &v["usage"];
    let tok = |k: &str| u[k].as_u64().unwrap_or(0);
    let call = Call {
        step: step.to_string(),
        batch,
        sentences: n,
        secs,
        cost_usd: v["total_cost_usd"].as_f64().unwrap_or(0.0),
        input_tokens: tok("input_tokens"),
        output_tokens: tok("output_tokens"),
        cache_read_tokens: tok("cache_read_input_tokens"),
        cache_write_tokens: tok("cache_creation_input_tokens"),
    };
    (v["result"].as_str().unwrap_or_default().to_string(), call)
}

/// Append one JSONL record. Nonfinite floats cannot round-trip through JSON.
/// This is not an atomic transaction or an fsync durability guarantee.
pub fn append_call(path: &Path, c: &Call) -> Result<()> {
    ensure!(
        c.secs.is_finite() && c.cost_usd.is_finite(),
        "call accounting requires finite seconds and cost"
    );
    let line = serde_json::to_string(c)?;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| path.display().to_string())?;
    writeln!(f, "{line}").with_context(|| path.display().to_string())?;
    Ok(())
}

/// Read accounting in order, ignoring blank lines. Only absence means empty;
/// unreadable files and malformed records fail without returning a partial prefix.
pub fn read_calls(path: &Path) -> Result<Vec<Call>> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e).with_context(|| path.display().to_string()),
    };
    text.lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
        .map(|(i, l)| {
            serde_json::from_str(l).with_context(|| format!("{}: line {}", path.display(), i + 1))
        })
        .collect()
}

#[cfg(test)]
mod wave17_models {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "mova-opus-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn canonical_decode_accounting_join() {
        let events = "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"1 OK\"}}\n{\"type\":\"turn.completed\",\"usage\":{\"input_tokens\":10,\"cached_input_tokens\":3,\"output_tokens\":2}}\n";
        let v = mova_teacher::decode_events(events).unwrap();
        let (answer, c) = interpret_reply("silver", 4, 7, 1.25, &v);
        assert_eq!(answer, "1 OK");
        assert_eq!(
            (
                c.input_tokens,
                c.output_tokens,
                c.cache_read_tokens,
                c.cache_write_tokens
            ),
            (10, 2, 3, 0)
        );
        assert_eq!(
            (c.step.as_str(), c.batch, c.sentences, c.secs, c.cost_usd),
            ("silver", 4, 7, 1.25, 0.0)
        );
    }
    #[test]
    fn supplied_projection_default_fibers() {
        for value in [
            json!(null),
            json!({}),
            json!({"result":42,"usage":{"input_tokens":-1,"output_tokens":"9"}}),
        ] {
            let (s, c) = interpret_reply("s", 0, 0, 0.0, &value);
            assert_eq!(s, "");
            assert_eq!(
                (
                    c.input_tokens,
                    c.output_tokens,
                    c.cache_read_tokens,
                    c.cache_write_tokens
                ),
                (0, 0, 0, 0)
            );
        }
        let (_, c) = interpret_reply(
            "s",
            2,
            3,
            4.0,
            &json!({"usage":{"input_tokens":u64::MAX,"output_tokens":9,"cache_read_input_tokens":8,"cache_creation_input_tokens":7},"total_cost_usd":0.125}),
        );
        assert_eq!(
            (
                c.input_tokens,
                c.output_tokens,
                c.cache_read_tokens,
                c.cache_write_tokens,
                c.cost_usd
            ),
            (u64::MAX, 9, 8, 7, 0.125)
        );
    }
    #[test]
    fn append_read_order_and_legacy_defaults() {
        let tmp = Scratch::new();
        let path = tmp.0.join("calls");
        assert!(read_calls(&path).unwrap().is_empty());
        std::fs::write(&path, "\n {\"step\":\"old\",\"sentences\":2,\"secs\":0.5,\"cost_usd\":0,\"input_tokens\":1,\"output_tokens\":2,\"cache_read_tokens\":3}\n \n").unwrap();
        let c = Call {
            step: "new\nquoted".into(),
            batch: 9,
            secs: 1.5,
            ..Call::default()
        };
        append_call(&path, &c).unwrap();
        let calls = read_calls(&path).unwrap();
        assert_eq!(calls.len(), 2);
        assert_eq!(
            (
                calls[0].step.as_str(),
                calls[0].batch,
                calls[0].cache_write_tokens
            ),
            ("old", 0, 0)
        );
        assert_eq!(
            serde_json::to_value(&calls[1]).unwrap(),
            serde_json::to_value(&c).unwrap()
        );
    }
    #[test]
    fn nonfinite_append_leaves_file_untouched() {
        let tmp = Scratch::new();
        let path = tmp.0.join("calls");
        for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for c in [
                Call {
                    secs: x,
                    ..Call::default()
                },
                Call {
                    cost_usd: x,
                    ..Call::default()
                },
            ] {
                assert!(
                    append_call(&path, &c).is_err(),
                    "nonfinite accounting accepted"
                );
                assert!(!path.exists());
            }
        }
        std::fs::write(&path, "sentinel").unwrap();
        assert!(
            append_call(
                &path,
                &Call {
                    secs: f64::NAN,
                    ..Call::default()
                }
            )
            .is_err()
        );
        assert_eq!(std::fs::read_to_string(path).unwrap(), "sentinel");
    }
    #[test]
    fn malformed_and_io_failures_are_not_empty() {
        let tmp = Scratch::new();
        let path = tmp.0.join("calls");
        append_call(&path, &Call::default()).unwrap();
        std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap()
            .write_all(b"bad\n")
            .unwrap();
        assert!(
            read_calls(&path)
                .unwrap_err()
                .to_string()
                .contains("line 2")
        );
        assert!(read_calls(&tmp.0).is_err());
        assert!(append_call(&tmp.0, &Call::default()).is_err());
        std::fs::write(&path, [0xff]).unwrap();
        assert!(read_calls(&path).is_err());
    }
    #[cfg(unix)]
    #[test]
    fn symlink_loop_is_read_error() {
        let tmp = Scratch::new();
        let path = tmp.0.join("loop");
        std::os::unix::fs::symlink("loop", &path).unwrap();
        assert!(
            read_calls(&path).is_err(),
            "filesystem error incorrectly treated as absence"
        );
    }
}
