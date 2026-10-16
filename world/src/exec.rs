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

use en::annotate::Word;

/// A map over threads, in order (the result does not depend on the threads).
pub fn par_map<T: Sync, U: Send>(xs: &[T], f: impl Fn(&T) -> U + Sync) -> Vec<U> {
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(16);
    let chunk = xs.len().div_ceil(threads).max(1);
    let f = &f;
    std::thread::scope(|s| {
        let hs: Vec<_> = xs.chunks(chunk).map(|c| s.spawn(move || c.iter().map(f).collect::<Vec<U>>())).collect();
        hs.into_iter().flat_map(|h| h.join().expect("a worker thread panicked")).collect()
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
pub fn parse_many(texts: &[String]) -> Vec<Vec<Word>> {
    match mode() {
        Mode::Cpu => cpu_parse_many(texts),
        Mode::Gpu => gpu_parse_many(texts),
    }
}

fn cpu_parse_many(texts: &[String]) -> Vec<Vec<Word>> {
    let _ = crate::tree::annotator();
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(12);
    let chunk = texts.len().div_ceil(threads).max(1);
    std::thread::scope(|s| {
        let hs: Vec<_> = texts.chunks(chunk).map(|c| s.spawn(move || c.iter().map(|u| crate::frames::parse_words(u)).collect::<Vec<_>>())).collect();
        hs.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    })
}

#[cfg(not(feature = "gpu"))]
fn gpu_parse_many(texts: &[String]) -> Vec<Vec<Word>> {
    cpu_parse_many(texts)
}

#[cfg(feature = "gpu")]
fn gpu_parse_many(texts: &[String]) -> Vec<Vec<Word>> {
    use en::gram::{Rel, Tag};
    static P: std::sync::OnceLock<Result<gparse::Loaded, String>> = std::sync::OnceLock::new();
    let Ok(a) = crate::tree::annotator() else { return vec![Vec::new(); texts.len()] };
    let t0 = std::time::Instant::now();
    let p = P.get_or_init(|| {
        let home = std::env::var("HOME").unwrap_or_default();
        let dir = std::env::var("MOVA_GPARSE_MODEL").unwrap_or(format!("{}/en/models/gparse", std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))));
        gparse::load(&dir)
    });
    let p = match p {
        Ok(p) => p,
        Err(e) => {
            eprintln!("MOVA_EXEC=gpu: the GPU parser: {e}");
            std::process::exit(2);
        }
    };
    let t1 = std::time::Instant::now();
    let forms: Vec<Vec<String>> = par_map(texts, |t| a.tokenize(t).into_iter().map(|x| x.form).collect());
    let nonempty: Vec<usize> = (0..forms.len()).filter(|&i| !forms[i].is_empty()).collect();
    let batch: Vec<Vec<String>> = nonempty.iter().map(|&i| forms[i].clone()).collect();
    let pred = match gparse::parse_forms(p, &batch) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("MOVA_EXEC=gpu: parsing: {e}");
            std::process::exit(2);
        }
    };
    let t2 = std::time::Instant::now();
    let mut out = vec![Vec::new(); texts.len()];
    let jobs: Vec<(usize, usize)> = nonempty.iter().copied().enumerate().collect();
    let done = par_map(&jobs, |&(k, i)| {
        let pr = &pred[k];
        let tags: Vec<Tag> = pr.iter().map(|x| Tag::parse(&x.2).unwrap_or(Tag::parse("NN").unwrap())).collect();
        let heads: Vec<usize> = pr.iter().map(|x| x.0).collect();
        let rels: Vec<Rel> = pr.iter().map(|x| Rel::parse(&x.1).or_else(|| Rel::parse(x.1.split(':').next().unwrap_or(""))).unwrap_or(Rel::Dep)).collect();
        finish(a.complete(&forms[i], &tags, &heads, &rels))
    });
    for ((_, i), w) in jobs.into_iter().zip(done) {
        out[i] = w;
    }
    if std::env::var("MOVA_TIMING").is_ok() {
        eprintln!("gpu parse: {} texts, load {:.1}s, tokenize+parse {:.1}s, complete {:.1}s", texts.len(), (t1 - t0).as_secs_f64(), (t2 - t1).as_secs_f64(), t2.elapsed().as_secs_f64());
    }
    out
}
