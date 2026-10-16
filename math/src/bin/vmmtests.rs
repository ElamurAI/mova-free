//! Independent tests from the LLM (Opus): the LLM does not see our code, it writes scenarios from the language
//! documentation and the mathematics; the answer is stored as is and frozen by a hash before the first run.
//!
//!   vmmtests ask <jobs.tsv> --log <calls.jsonl> --cap <USD> [--par 2] [--reserve 1.5] [--cwd DIR]
//!
//! `jobs.tsv`: `name <TAB> prompt file <TAB> answer file` (paths relative to jobs.tsv). An existing non-empty
//! answer is not re-asked (we do not pay twice). Cost cap — `math::vmm::Vmm`: before each call,
//! spent from the log + reserve ≤ cap, otherwise stop without calling (fail-fast, no retries).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result, bail};
use math::vmm::Vmm;

fn opt<'a>(args: &'a [String], k: &str) -> Option<&'a str> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).map(|s| s.as_str())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) != Some("ask") || args.len() < 2 {
        bail!("usage: vmmtests ask <jobs.tsv> --log <calls.jsonl> --cap <USD> [--par 2] [--reserve 1.5] [--cwd DIR]");
    }
    let jobs_path = PathBuf::from(&args[1]);
    let base = jobs_path.parent().unwrap_or(Path::new(".")).to_path_buf();
    let log = PathBuf::from(opt(&args, "--log").context("--log")?);
    let cap: f64 = opt(&args, "--cap").context("--cap")?.parse()?;
    let par: usize = opt(&args, "--par").unwrap_or("2").parse()?;
    let reserve: f64 = opt(&args, "--reserve").unwrap_or("1.5").parse()?;
    let cwd = opt(&args, "--cwd").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join("vmmtests-empty"));
    let text = std::fs::read_to_string(&jobs_path).with_context(|| jobs_path.display().to_string())?;
    let mut jobs = Vec::new();
    for l in text.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() != 3 {
            bail!("jobs.tsv: 3 fields required: {l}");
        }
        jobs.push((f[0].to_string(), base.join(f[1]), base.join(f[2])));
    }
    let vmm = Vmm::new(cwd, log, cap)?;
    eprintln!("already spent ${:.2} of the ${cap:.2} cap; jobs {}", vmm.spent(), jobs.len());
    let next = AtomicUsize::new(0);
    let failed = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..par.clamp(1, 2) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some((name, prompt, out)) = jobs.get(i) else { break };
                    if std::fs::metadata(out).map(|m| m.len() > 0).unwrap_or(false) {
                        eprintln!("{name}: answer already exists — skipping");
                        continue;
                    }
                    let res = std::fs::read_to_string(prompt)
                        .with_context(|| prompt.display().to_string())
                        .and_then(|p| vmm.ask(name, i, 0, &p, reserve));
                    match res {
                        Ok((text, call)) => {
                            if let Err(e) = std::fs::write(out, &text) {
                                eprintln!("{name}: write {}: {e}", out.display());
                                failed.fetch_add(1, Ordering::SeqCst);
                            }
                            eprintln!(
                                "{name}: {:.0} s, ${:.3}, out {} tok.; total ${:.2}",
                                call.secs,
                                call.cost_usd,
                                call.output_tokens,
                                vmm.spent()
                            );
                        }
                        Err(e) => {
                            // fail-fast: cap or failure — do not run the rest
                            eprintln!("{name}: ERROR {e:#}");
                            failed.fetch_add(1, Ordering::SeqCst);
                            next.store(usize::MAX / 2, Ordering::SeqCst);
                        }
                    }
                }
            });
        }
    });
    let f = failed.load(Ordering::SeqCst);
    eprintln!("done: spent ${:.2}; errors {f}", vmm.spent());
    if f > 0 {
        bail!("{f} jobs failed");
    }
    Ok(())
}
