//! LLM run over batches: prag::opus uses Codex CLI through mova-teacher; accounting is
//! one `calls.jsonl` line per call. Raw outputs — `raw/bNNN.txt`; a batch format rejection gives exactly one
//! retry (`raw/bNNN.retry.txt`), other rejections go only to the log. A call error stops the run
//! (fail-fast, no retries); finished batches remain, a rerun takes the unfinished ones. Cap —
//! `max_calls` calls per run dir including retries (`--max-calls`, default `MAX_CALLS`).
//!
//! What to ask and how to check the format — `Job`: questions over fairy-tale paragraphs (`Tales`) or pilot corrections
//! (`fix::Fixes`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::{Context, Result, bail};

use prag::opus::{Opus, append_call, read_calls};

use crate::gate::{Level, gate};
use crate::prompt::prompt;
use crate::schema::Rule;
use crate::select::Para;

/// Default call cap (pilot: ~15 batches and a few format retries).
pub const MAX_CALLS: usize = 20;

/// LLM work over batches.
pub trait Job: Sync {
    /// step name in `calls.jsonl`; a retry — with "-retry"
    fn step(&self) -> &'static str;
    fn batches(&self) -> Vec<usize>;
    fn prompt(&self, b: usize) -> String;
    /// sentences in the batch — for accounting
    fn size(&self, b: usize) -> usize;
    /// format rejection (grounds for one retry) and the batch summary in one line
    fn check(&self, b: usize, text: &str) -> (Option<String>, String);
}

/// Batches by number, in paragraph order.
pub fn batches(paras: &[Para]) -> BTreeMap<usize, Vec<&Para>> {
    let mut b: BTreeMap<usize, Vec<&Para>> = BTreeMap::new();
    for p in paras {
        b.entry(p.batch).or_default().push(p);
    }
    b
}

/// Questions and answers over fairy-tale paragraphs.
pub struct Tales<'a> {
    pub all: BTreeMap<usize, Vec<&'a Para>>,
    pub rule: Rule,
}

impl<'a> Tales<'a> {
    pub fn new(paras: &'a [Para], rule: Rule) -> Tales<'a> {
        Tales {
            all: batches(paras),
            rule,
        }
    }
}

impl Job for Tales<'_> {
    fn step(&self) -> &'static str {
        "qagen"
    }

    fn batches(&self) -> Vec<usize> {
        self.all.keys().copied().collect()
    }

    fn prompt(&self, b: usize) -> String {
        prompt(&self.all[&b], self.rule)
    }

    fn size(&self, b: usize) -> usize {
        self.all[&b].iter().map(|p| p.sents.len()).sum()
    }

    fn check(&self, b: usize, text: &str) -> (Option<String>, String) {
        let its = &self.all[&b];
        let g = gate(text, its, self.rule);
        let rej = g.rej.iter().filter(|r| r.level != Level::Note).count();
        (
            g.format,
            format!(
                "paragraphs {}, pairs accepted {}, rejections {rej}",
                its.len(),
                g.ok.len()
            ),
        )
    }
}

pub fn raw_path(dir: &Path, b: usize) -> PathBuf {
    dir.join("raw").join(format!("b{b:03}.txt"))
}

pub fn retry_path(dir: &Path, b: usize) -> PathBuf {
    dir.join("raw").join(format!("b{b:03}.retry.txt"))
}

/// Compatibility lookup: retry first, then original; metadata errors count as absence.
/// Use try_final_raw when an unreadable path must stop processing.
pub fn final_raw(dir: &Path, b: usize) -> Option<PathBuf> {
    [retry_path(dir, b), raw_path(dir, b)]
        .into_iter()
        .find(|p| p.exists())
}

/// Strict final-output lookup. Only missing paths are skipped; metadata errors propagate.
/// Follows symlinks and treats dangling links as missing.
pub fn try_final_raw(dir: &Path, b: usize) -> Result<Option<PathBuf>> {
    for path in [retry_path(dir, b), raw_path(dir, b)] {
        if path
            .try_exists()
            .with_context(|| path.display().to_string())?
        {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

/// What a batch still needs: 0 — nothing, 1 — the first call, 2 — a retry after a format rejection.
fn need(dir: &Path, job: &dyn Job, b: usize) -> Result<u8> {
    let raw = raw_path(dir, b);
    if !raw
        .try_exists()
        .with_context(|| raw.display().to_string())?
    {
        return Ok(1);
    }
    let retry = retry_path(dir, b);
    if retry
        .try_exists()
        .with_context(|| retry.display().to_string())?
    {
        return Ok(0);
    }
    let text = std::fs::read_to_string(&raw).with_context(|| raw.display().to_string())?;
    Ok(if job.check(b, &text).0.is_some() {
        2
    } else {
        0
    })
}

/// Run over batches `which` (empty — all), `jobs` calls in parallel, at most `max_calls` per dir.
pub fn run(
    dir: &Path,
    job: &dyn Job,
    which: &[usize],
    jobs: usize,
    max_calls: usize,
) -> Result<()> {
    let mut todo: Vec<(usize, u8)> = Vec::new();
    for b in job.batches() {
        if which.is_empty() || which.contains(&b) {
            let n = need(dir, job, b)?;
            if n > 0 {
                todo.push((b, n));
            }
        }
    }
    std::fs::create_dir_all(dir.join("raw"))?;
    let calls_path = dir.join("calls.jsonl");
    let done_calls = read_calls(&calls_path)?.len();
    let opus = Opus::from_env(dir.join("cwd"));
    eprintln!(
        "LLM {} ({}): batches to call {}, parallel {jobs}; calls so far {done_calls}, cap {max_calls}",
        opus.model,
        opus.effort,
        todo.len()
    );
    let calls = AtomicUsize::new(done_calls);
    let next = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());
    let log = Mutex::new(());
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| {
                loop {
                    if stop.load(Ordering::SeqCst) {
                        break;
                    }
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(&(b, mut stage)) = todo.get(i) else {
                        break;
                    };
                    let p = job.prompt(b);
                    let r = (|| -> Result<()> {
                        std::fs::write(dir.join("raw").join(format!("b{b:03}.prompt.txt")), &p)?;
                        while stage > 0 {
                            if calls.fetch_add(1, Ordering::SeqCst) >= max_calls {
                                bail!("cap of {max_calls} calls");
                            }
                            let (step, out) = if stage == 1 {
                                (job.step().to_string(), raw_path(dir, b))
                            } else {
                                (format!("{}-retry", job.step()), retry_path(dir, b))
                            };
                            let (text, call) = opus.ask(&step, b, job.size(b), &p)?;
                            let tmp = out.with_extension("part");
                            std::fs::write(&tmp, &text)?;
                            std::fs::rename(&tmp, &out)?;
                            let _g = log.lock().unwrap();
                            append_call(&calls_path, &call)?;
                            let (format, summary) = job.check(b, &text);
                            eprintln!(
                                "batch {b}{}: {summary}; {:.0} s, output {} tok.{}",
                                if stage == 2 { " (retry)" } else { "" },
                                call.secs,
                                call.output_tokens,
                                format
                                    .as_deref()
                                    .map(|f| format!(" — FORMAT: {f}"))
                                    .unwrap_or_default()
                            );
                            // one retry — only on a format rejection of the first call
                            stage = if stage == 1 && format.is_some() { 2 } else { 0 };
                        }
                        Ok(())
                    })();
                    if let Err(e) = r {
                        stop.store(true, Ordering::SeqCst);
                        errors.lock().unwrap().push(format!("batch {b}: {e:#}"));
                        break;
                    }
                }
            });
        }
    });
    let errors = errors.into_inner().unwrap();
    if !errors.is_empty() {
        bail!("stopped (fail-fast, no retries): {}", errors.join(" | "));
    }
    Ok(())
}

#[cfg(test)]
mod wave18_models {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "mova-run-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(p.join("raw")).unwrap();
            Self(p)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    struct Fake;
    impl Job for Fake {
        fn step(&self) -> &'static str {
            "fixture"
        }
        fn batches(&self) -> Vec<usize> {
            vec![1]
        }
        fn prompt(&self, _: usize) -> String {
            panic!("fixture must never call teacher")
        }
        fn size(&self, _: usize) -> usize {
            0
        }
        fn check(&self, _: usize, text: &str) -> (Option<String>, String) {
            (
                if text == "valid" {
                    None
                } else {
                    Some("bad".into())
                },
                String::new(),
            )
        }
    }
    #[test]
    fn need_matches_raw_retry_state_table() {
        for raw in [None, Some("valid"), Some("invalid")] {
            for retry in [false, true] {
                let d = Scratch::new();
                if let Some(t) = raw {
                    std::fs::write(raw_path(&d.0, 1), t).unwrap();
                }
                if retry {
                    std::fs::write(retry_path(&d.0, 1), "anything").unwrap();
                }
                let expected = match (raw, retry) {
                    (None, _) => 1,
                    (_, true) => 0,
                    (Some("valid"), false) => 0,
                    _ => 2,
                };
                assert_eq!(need(&d.0, &Fake, 1).unwrap(), expected);
                let selected = final_raw(&d.0, 1);
                assert_eq!(try_final_raw(&d.0, 1).unwrap(), selected);
                assert_eq!(
                    selected,
                    if retry {
                        Some(retry_path(&d.0, 1))
                    } else {
                        raw.map(|_| raw_path(&d.0, 1))
                    }
                );
            }
        }
    }
    #[cfg(unix)]
    #[test]
    fn need_metadata_error_is_not_absence() {
        let d = Scratch::new();
        let raw = raw_path(&d.0, 1);
        std::os::unix::fs::symlink(&raw, &raw).unwrap();
        assert!(
            need(&d.0, &Fake, 1).is_err(),
            "symlink loop is not a missing raw file"
        );
    }
    #[cfg(unix)]
    #[test]
    fn need_retry_metadata_error_is_not_completion() {
        let d = Scratch::new();
        std::fs::write(raw_path(&d.0, 1), "valid").unwrap();
        let retry = retry_path(&d.0, 1);
        std::os::unix::fs::symlink(&retry, &retry).unwrap();
        assert!(
            need(&d.0, &Fake, 1).is_err(),
            "retry metadata failure must stop preflight"
        );
    }
}

#[cfg(test)]
mod wave18_strict_lookup {
    use super::*;
    #[cfg(unix)]
    #[test]
    fn retry_error_is_not_hidden_by_valid_original() {
        let dir = std::env::temp_dir().join(format!("mova-strict-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("raw")).unwrap();
        let retry = retry_path(&dir, 1);
        std::fs::write(raw_path(&dir, 1), "valid").unwrap();
        std::os::unix::fs::symlink(&retry, &retry).unwrap();
        assert!(try_final_raw(&dir, 1).is_err());
        assert_eq!(final_raw(&dir, 1), Some(raw_path(&dir, 1)));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
