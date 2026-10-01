//! LLM run over batches: the call is `prag::opus` (`claude -p`, Opus on high via `--settings`), accounting is
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
        Tales { all: batches(paras), rule }
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
        (g.format, format!("paragraphs {}, pairs accepted {}, rejections {rej}", its.len(), g.ok.len()))
    }
}

pub fn raw_path(dir: &Path, b: usize) -> PathBuf {
    dir.join("raw").join(format!("b{b:03}.txt"))
}

pub fn retry_path(dir: &Path, b: usize) -> PathBuf {
    dir.join("raw").join(format!("b{b:03}.retry.txt"))
}

/// Final raw output of a batch: the retry if there was one, otherwise the first.
pub fn final_raw(dir: &Path, b: usize) -> Option<PathBuf> {
    [retry_path(dir, b), raw_path(dir, b)].into_iter().find(|p| p.exists())
}

/// What a batch still needs: 0 — nothing, 1 — the first call, 2 — a retry after a format rejection.
fn need(dir: &Path, job: &dyn Job, b: usize) -> Result<u8> {
    let raw = raw_path(dir, b);
    if !raw.exists() {
        return Ok(1);
    }
    if retry_path(dir, b).exists() {
        return Ok(0);
    }
    let text = std::fs::read_to_string(&raw).with_context(|| raw.display().to_string())?;
    Ok(if job.check(b, &text).0.is_some() { 2 } else { 0 })
}

/// Run over batches `which` (empty — all), `jobs` calls in parallel, at most `max_calls` per dir.
pub fn run(dir: &Path, job: &dyn Job, which: &[usize], jobs: usize, max_calls: usize) -> Result<()> {
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
    eprintln!("LLM {} ({}): batches to call {}, parallel {jobs}; calls so far {done_calls}, cap {max_calls}", opus.model, opus.effort, todo.len());
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
                    let Some(&(b, mut stage)) = todo.get(i) else { break };
                    let p = job.prompt(b);
                    let r = (|| -> Result<()> {
                        std::fs::write(dir.join("raw").join(format!("b{b:03}.prompt.txt")), &p)?;
                        while stage > 0 {
                            if calls.fetch_add(1, Ordering::SeqCst) >= max_calls {
                                bail!("cap of {max_calls} calls");
                            }
                            let (step, out) = if stage == 1 { (job.step().to_string(), raw_path(dir, b)) } else { (format!("{}-retry", job.step()), retry_path(dir, b)) };
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
                                format.as_deref().map(|f| format!(" — FORMAT: {f}")).unwrap_or_default()
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
