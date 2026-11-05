//! The two execution modes, always both: CPU only and GPU enhanced (docs/gpu-exec.md).
//!
//! - **CPU only** (the default, and the only one a build without `--features gpu` has): everything as it was, exact.
//! - **GPU enhanced** (`MOVA_EXEC=gpu`, a build with `--features gpu`): batch stages run on the GPU where it pays. A
//!   stage is either *exact* there (the same result as on the CPU, checked) or an *approximant* (a GPU model standing
//!   in for a CPU one, with its agreement measured); approximants are marked as such.
//!
//! Stages so far:
//! - parse ahead (`parse_many`, approximant): the GPU parser (`gparse`, MOVA_GPARSE_MODEL, default
//!   $MOVA_DATA/en/models/gparse) gives the tags and the tree, the `en` annotator the tokens, UPOS, FEATS and lemmas.
//!
//! Asking for the GPU from a CPU-only build is an error (fail-fast), not a silent fallback.

use anyhow::{Result, anyhow};
use en::annotate::Word;

/// Maps in input order; results are schedule-independent for pure callbacks.
pub fn par_map<T: Sync, U: Send>(xs: &[T], f: impl Fn(&T) -> U + Sync) -> Vec<U> {
    try_par_map(xs, 16, f).expect("a worker thread panicked")
}

/// Join every worker before returning an error; never return a truncated batch.
fn try_par_map<T: Sync, U: Send>(xs: &[T], max_threads: usize, f: impl Fn(&T) -> U + Sync) -> Result<Vec<U>> {
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(max_threads);
    let chunk = xs.len().div_ceil(threads).max(1);
    let f = &f;
    std::thread::scope(|s| {
        let hs: Vec<_> = xs.chunks(chunk).map(|c| s.spawn(move || c.iter().map(f).collect::<Vec<U>>())).collect();
        // Collect all joins first, including after a panic, so scope does not re-panic on an unjoined worker.
        let joined: Vec<_> = hs.into_iter().map(|h| h.join().map_err(|_| anyhow!("parser batch worker panicked"))).collect();
        let batches: Result<Vec<_>> = joined.into_iter().collect();
        Ok(batches?.into_iter().flatten().collect())
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Cpu,
    Gpu,
}

/// The mode asked for (MOVA_EXEC=cpu|gpu; cpu by default).
pub fn mode() -> Mode {
    static M: std::sync::OnceLock<Mode> = std::sync::OnceLock::new();
    *M.get_or_init(|| match std::env::var("MOVA_EXEC").as_deref() {
        Ok("gpu") => {
            if !cfg!(feature = "gpu") {
                eprintln!("MOVA_EXEC=gpu: this build is CPU only (build with --features gpu)");
                std::process::exit(2);
            }
            Mode::Gpu
        }
        Ok("cpu") | Err(_) => Mode::Cpu,
        Ok(x) => {
            eprintln!("MOVA_EXEC={x}: cpu or gpu");
            std::process::exit(2);
        }
    })
}

/// The `en` reading of a cased text's words after the tree: the matrix repairs and lower case, as `Tree::parse_cased`.
#[cfg(feature = "gpu")]
fn finish(mut w: Vec<Word>) -> Vec<Word> {
    if std::env::var("WORLD_NO_REPAIR").is_err() {
        crate::rerank::repair_words(&mut w);
    }
    for x in &mut w {
        x.form = x.form.to_lowercase();
        x.lemma = x.lemma.to_lowercase();
    }
    w
}

/// Parse many cased texts: on the CPU (threads; exactly `frames::parse_words`) or on the GPU (batches).
/// Model and worker failures are errors, never empty or shortened successful batches.
pub fn parse_many(texts: &[String]) -> Result<Vec<Vec<Word>>> {
    match mode() {
        Mode::Cpu => cpu_parse_many(texts),
        Mode::Gpu => gpu_parse_many(texts),
    }
}

fn cpu_parse_many(texts: &[String]) -> Result<Vec<Vec<Word>>> {
    let a = crate::tree::annotator()?;
    try_par_map(texts, 12, |u| crate::tree::Tree::parse_cased(a, u).w)
}

#[cfg(not(feature = "gpu"))]
fn gpu_parse_many(texts: &[String]) -> Result<Vec<Vec<Word>>> {
    let _ = texts;
    anyhow::bail!("MOVA_EXEC=gpu: this build is CPU only (build with --features gpu)")
}

#[cfg(feature = "gpu")]
fn gpu_parse_many(texts: &[String]) -> Result<Vec<Vec<Word>>> {
    use en::gram::{Rel, Tag};
    static P: std::sync::OnceLock<Result<gparse::Loaded, String>> = std::sync::OnceLock::new();
    let a = crate::tree::annotator()?;
    let t0 = std::time::Instant::now();
    let p = P.get_or_init(|| {
        let dir = std::env::var("MOVA_GPARSE_MODEL").unwrap_or(format!("{}/en/models/gparse", std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))));
        gparse::load(&dir)
    });
    let p = p.as_ref().map_err(|e| anyhow!("MOVA_EXEC=gpu: the GPU parser: {e}"))?;
    let t1 = std::time::Instant::now();
    let forms: Vec<Vec<String>> = try_par_map(texts, 16, |t| a.tokenize(t).into_iter().map(|x| x.form).collect())?;
    let nonempty: Vec<usize> = (0..forms.len()).filter(|&i| !forms[i].is_empty()).collect();
    let batch: Vec<Vec<String>> = nonempty.iter().map(|&i| forms[i].clone()).collect();
    let pred = gparse::parse_forms(p, &batch).map_err(|e| anyhow!("MOVA_EXEC=gpu: parsing: {e}"))?;
    anyhow::ensure!(pred.len() == batch.len(), "GPU parser returned {} sentences for {} inputs", pred.len(), batch.len());
    for (k, (words, prediction)) in batch.iter().zip(&pred).enumerate() {
        anyhow::ensure!(words.len() == prediction.len(), "GPU parser sentence {k}: {} predictions for {} tokens", prediction.len(), words.len());
    }
    let t2 = std::time::Instant::now();
    let mut out = vec![Vec::new(); texts.len()];
    let jobs: Vec<(usize, usize)> = nonempty.iter().copied().enumerate().collect();
    let done = try_par_map(&jobs, 16, |&(k, i)| {
        let pr = &pred[k];
        let tags: Vec<Tag> = pr.iter().map(|x| Tag::parse(&x.2).unwrap_or(Tag::parse("NN").unwrap())).collect();
        let heads: Vec<usize> = pr.iter().map(|x| x.0).collect();
        let rels: Vec<Rel> = pr.iter().map(|x| Rel::parse(&x.1).or_else(|| Rel::parse(x.1.split(':').next().unwrap_or(""))).unwrap_or(Rel::Dep)).collect();
        finish(a.complete(&forms[i], &tags, &heads, &rels))
    })?;
    for ((_, i), w) in jobs.into_iter().zip(done) {
        out[i] = w;
    }
    if std::env::var("MOVA_TIMING").is_ok() {
        eprintln!("gpu parse: {} texts, load {:.1}s, tokenize+parse {:.1}s, complete {:.1}s", texts.len(), (t1 - t0).as_secs_f64(), (t2 - t1).as_secs_f64(), t2.elapsed().as_secs_f64());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::try_par_map;

    #[test]
    fn batches_preserve_order_and_cardinality() {
        let xs: Vec<_> = (0..101).collect();
        assert_eq!(try_par_map(&xs, 4, |x| x * 2).unwrap(), xs.iter().map(|x| x * 2).collect::<Vec<_>>());
        assert!(try_par_map::<i32, i32>(&[], 4, |x| *x).unwrap().is_empty());
    }

    #[test]
    fn worker_panics_are_errors_instead_of_partial_results() {
        let result = try_par_map(&[0, 1, 2, 3], 4, |_| -> usize { panic!("broken worker") });
        assert!(result.unwrap_err().to_string().contains("worker panicked"));
    }
}
#[cfg(test)]
mod correspondence_tests {
    use super::{par_map, try_par_map};
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    #[test]
    fn finite_output_fibers_are_complete_cartesian_products() {
        for length in 0..=3u32 {
            for cap in [1, 3] {
                let mut fibers: BTreeMap<Vec<usize>, Vec<Vec<usize>>> = BTreeMap::new();
                for mut code in 0..4usize.pow(length) {
                    let input: Vec<_> = (0..length)
                        .map(|_| {
                            let x = code % 4;
                            code /= 4;
                            x
                        })
                        .collect();
                    let output = try_par_map(&input, cap, |x| x % 2).unwrap();
                    fibers.entry(output).or_default().push(input);
                }
                for mut code in 0..3usize.pow(length) {
                    let output: Vec<_> = (0..length)
                        .map(|_| {
                            let x = code % 3;
                            code /= 3;
                            x
                        })
                        .collect();
                    let mut expected = Vec::new();
                    if output.iter().all(|&x| x < 2) {
                        for bits in 0..(1usize << length) {
                            expected.push(
                                output
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &x)| x + 2 * ((bits >> i) & 1))
                                    .collect::<Vec<_>>(),
                            );
                        }
                    }
                    expected.sort();
                    let mut actual = fibers.get(&output).cloned().unwrap_or_default();
                    actual.sort();
                    assert_eq!(
                        actual, expected,
                        "length={length}, cap={cap}, output={output:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn every_worker_finishes_its_prefix_before_panic_error_returns() {
        let threads = std::thread::available_parallelism()
            .map(|x| x.get())
            .unwrap_or(4)
            .min(3);
        let n = threads * 4;
        let input: Vec<_> = (0..n).collect();
        for failures in [vec![1], vec![1, (threads - 1) * 4 + 2]] {
            let visits: Vec<_> = (0..n).map(|_| AtomicUsize::new(0)).collect();
            let outcome = std::panic::catch_unwind(|| {
                try_par_map(&input, 3, |&i| {
                    visits[i].fetch_add(1, Ordering::SeqCst);
                    assert!(!failures.contains(&i), "injected worker panic");
                    i
                })
            });
            let error = outcome
                .expect("joined worker panics must not escape")
                .unwrap_err();
            assert_eq!(error.to_string(), "parser batch worker panicked");
            for (i, visit) in visits.iter().enumerate() {
                let start = (i / 4) * 4;
                let first = failures
                    .iter()
                    .copied()
                    .filter(|&f| (start..start + 4).contains(&f))
                    .min();
                let expected = usize::from(first.is_none_or(|f| i <= f));
                assert_eq!(
                    visit.load(Ordering::SeqCst),
                    expected,
                    "i={i}, failures={failures:?}"
                );
            }
        }
    }
    #[test]
    fn failed_batches_drop_all_successfully_constructed_outputs() {
        struct Output(Arc<AtomicUsize>);
        impl Drop for Output {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let created = AtomicUsize::new(0);
        let dropped = Arc::new(AtomicUsize::new(0));
        let threads = std::thread::available_parallelism()
            .map(|x| x.get())
            .unwrap_or(4)
            .min(3);
        let n = threads * 4;
        let input: Vec<_> = (0..n).collect();
        let result = try_par_map(&input, 3, |&i| {
            assert_ne!(i, 1, "injected worker panic");
            created.fetch_add(1, Ordering::SeqCst);
            Output(dropped.clone())
        });
        assert!(result.is_err());
        assert_eq!(created.load(Ordering::SeqCst), n - 3);
        assert_eq!(dropped.load(Ordering::SeqCst), n - 3);
    }
    #[test]
    fn ordered_output_preserves_positions_but_not_callback_state() {
        let input: Vec<_> = (0..31).collect();
        for base in [0, 100] {
            let ticket = AtomicUsize::new(base);
            let result = par_map(&input, |&i| (i, ticket.fetch_add(1, Ordering::SeqCst)));
            assert_eq!(result.iter().map(|&(i, _)| i).collect::<Vec<_>>(), input);
            let mut tickets: Vec<_> = result.iter().map(|&(_, v)| v).collect();
            tickets.sort_unstable();
            assert_eq!(tickets, (base..base + input.len()).collect::<Vec<_>>());
        }
    }
}
