//! gparse — the MMM dependency parser trained by gradient descent on the GPU (RTX 4000, candle + CUDA).
//!
//! gparse train <model dir> <train.conllu>... -- <dev.conllu>...   AdamW, warmup + linear decay; the epoch with the best
//!                                                                  mean dev LAS is saved (GP_EPOCHS, GP_TOKENS, GP_LR, GP_SEED,
//!                                                                  GP_DROP, GP_LAYERS; GP_CPU=1 runs on the CPU)
//! gparse eval <model dir> <test.conllu>...                         UAS / LAS / XPOS / UPOS on gold tokens
//! gparse annotate <model dir> <in.conllu> <out.conllu>             trees for gold tokens (GP_LEX=<corpus>: keep or
//!                                                                  refill LEMMA/FEATS, relations the corpus knows)
//!
//! Words come tokenized; the parser predicts XPOS, UPOS, heads and relations itself (no tagger in front).

use gparse::{data, e, load, load_vars, model, predict};

use std::fs;
use std::path::Path;

use candle_core::{DType, Device, Tensor};
use candle_nn::{loss, AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap};

use data::{Rng, Sent, Vocs};
use model::{Cfg, Net};

type R<T> = std::result::Result<T, String>;

fn env<T: std::str::FromStr>(k: &str, d: T) -> T {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn read_all(paths: &[String]) -> R<Vec<Sent>> {
    let mut out = Vec::new();
    for p in paths {
        out.extend(data::read(&fs::read_to_string(p).map_err(|x| format!("{p}: {x}"))?));
    }
    Ok(out)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let r = match a.get(1).map(String::as_str) {
        Some("train") if a.len() > 3 => {
            let sep = a.iter().position(|x| x == "--").unwrap_or(a.len());
            train(&a[2], &a[3..sep], if sep < a.len() { &a[sep + 1..] } else { &[] })
        }
        Some("eval") if a.len() > 3 => eval(&a[2], &a[3..]),
        Some("annotate") if a.len() == 5 => annotate(&a[2], &a[3], &a[4]),
        _ => Err("usage: gparse train <model dir> <train.conllu>... -- <dev.conllu>... | eval <model dir> <test.conllu>... | annotate <model dir> <in.conllu> <out.conllu>".into()),
    };
    if let Err(x) = r {
        eprintln!("gparse: {x}");
        std::process::exit(1);
    }
}

/// GP_PRAG=<roles.tsv> (`prag roles`): the reader's roles for these sentences, read back as input.
fn with_roles(sents: &mut [Sent]) -> R<()> {
    if let Ok(f) = std::env::var("GP_PRAG") {
        let n = data::attach_roles(sents, &fs::read_to_string(&f).map_err(|x| format!("{f}: {x}"))?);
        eprintln!("pragmatics: {n} roles from {f}");
    }
    Ok(())
}

/// (UAS, LAS, XPOS, UPOS, words)
fn score(sents: &[Sent], pred: &[Vec<(usize, String, String, String)>]) -> (f64, f64, f64, f64, usize) {
    let (mut n, mut u, mut l, mut x, mut up) = (0usize, 0usize, 0usize, 0usize, 0usize);
    for (s, p) in sents.iter().zip(pred) {
        for (w, q) in s.words.iter().zip(p) {
            n += 1;
            if w.head == q.0 {
                u += 1;
                l += (w.rel == q.1) as usize;
            }
            x += (w.xpos == q.2) as usize;
            up += (w.upos == q.3) as usize;
        }
    }
    let f = |a: usize| 100. * a as f64 / n.max(1) as f64;
    (f(u), f(l), f(x), f(up), n)
}

fn train(dir: &str, train_files: &[String], dev_files: &[String]) -> R<()> {
    let mut cfg = Cfg::DEFAULT;
    cfg.dropout = env("GP_DROP", cfg.dropout);
    cfg.layers = env("GP_LAYERS", cfg.layers);
    let epochs: usize = env("GP_EPOCHS", 30);
    let tokens: usize = env("GP_TOKENS", 4000);
    let lr: f64 = env("GP_LR", 1e-3);
    let seed: u64 = env("GP_SEED", 42);
    // GP_ALWAYS=<file,file>: sentences that go into every epoch whole (a cycle round's new trees); the rest is sampled
    let always_files: Vec<String> = std::env::var("GP_ALWAYS").map(|v| v.split(',').filter(|x| !x.is_empty()).map(String::from).collect()).unwrap_or_default();
    let all: Vec<(Sent, bool)> = read_all(train_files)?.into_iter().map(|s| (s, false)).chain(read_all(&always_files)?.into_iter().map(|s| (s, true))).collect();
    let before = all.len();
    let (mut sents, keep_always): (Vec<Sent>, Vec<bool>) = all.into_iter().filter(|(s, _)| s.words.len() < cfg.max_len).unzip();
    // GP_PRAG_TRAIN=<roles.tsv>: the reader's roles over the training trees; GP_PRAG_DROP (0.5) of the sentences
    // see none, so the first pass (no pragmatics yet) is learned too
    let prag_drop: f32 = env("GP_PRAG_DROP", 0.5);
    if let Ok(f) = std::env::var("GP_PRAG_TRAIN") {
        let n = data::attach_roles(&mut sents, &fs::read_to_string(&f).map_err(|x| format!("{f}: {x}"))?);
        println!("pragmatics: {n} roles on the training trees, hidden for {prag_drop} of sentences per batch");
    }
    // GP_SAMPLE: the fraction of the other sentences each epoch sees (a fresh random subset per epoch)
    let sample: f64 = env("GP_SAMPLE", 1.0);
    let devs: Vec<(String, Vec<Sent>)> = dev_files.iter().map(|p| read_all(std::slice::from_ref(p)).map(|s| (p.clone(), s))).collect::<R<_>>()?;
    // GP_INIT=<model dir>: continue from that model (its configuration and vocabularies; new words are unknown)
    let init = std::env::var("GP_INIT").ok();
    let (cfg, vocs) = match &init {
        Some(d) => {
            let rd = |f: &str| fs::read_to_string(Path::new(d).join(f)).map_err(|x| format!("{d}/{f}: {x}"));
            (Cfg::load(&rd("cfg.txt")?), Vocs { words: data::Voc::load(&rd("words.txt")?), xpos: data::Voc::load(&rd("xpos.txt")?), upos: data::Voc::load(&rd("upos.txt")?), rels: data::Voc::load(&rd("rels.txt")?) })
        }
        None => (cfg, Vocs::build(&sents, 2)),
    };
    println!("train: {} sentences ({} longer than {} left out; {} always in, the rest sampled at {sample}), {} words; vocab {} words, {} XPOS, {} UPOS, {} relations{}",
        sents.len(), before - sents.len(), cfg.max_len, keep_always.iter().filter(|x| **x).count(), sents.iter().map(|s| s.words.len()).sum::<usize>(), vocs.words.len(), vocs.xpos.len(), vocs.upos.len(), vocs.rels.len(),
        init.as_ref().map(|d| format!("; continuing from {d}")).unwrap_or_default());
    let dev = if std::env::var("GP_CPU").is_ok() { Device::Cpu } else { Device::cuda_if_available(0).map_err(e)? };
    if dev.is_cuda() {
        dev.set_seed(seed).map_err(e)?;
    }
    let vm = VarMap::new();
    let vb = VarBuilder::from_varmap(&vm, DType::F32, &dev);
    let net = Net::new(cfg, vocs.words.len(), vocs.xpos.len(), vocs.upos.len(), vocs.rels.len(), vb).map_err(e)?;
    if let Some(d) = &init {
        load_vars(&vm, &Path::new(d).join("model.safetensors"), &dev)?;
    }
    let n_params: usize = vm.all_vars().iter().map(|v| v.elem_count()).sum();
    println!("model: {:?}, {:.1} M parameters, device {:?}", cfg, n_params as f64 / 1e6, dev.location());
    let mut opt = AdamW::new(vm.all_vars(), ParamsAdamW { lr, beta1: 0.9, beta2: 0.98, eps: 1e-8, weight_decay: 0.01 }).map_err(e)?;
    let mut rng0 = Rng(seed ^ 0x2545f4914f6cdd1d);
    // one epoch's sentences: all of `always` plus a random `sample` share of the rest
    let pick = |rng: &mut Rng| -> Vec<usize> { (0..sents.len()).filter(|&i| keep_always[i] || sample >= 1.0 || (rng.unit() as f64) < sample).collect() };
    let first = pick(&mut rng0);
    let est = data::plan_of(&sents, &first, tokens).len();
    let total = (est * epochs) as f64;
    let warm = if init.is_some() { (est as f64).min(100.) } else { (est as f64).min(2000.) };
    let p = Path::new(dir);
    fs::create_dir_all(p).map_err(e)?;
    for (f, v) in [("words.txt", &vocs.words), ("xpos.txt", &vocs.xpos), ("upos.txt", &vocs.upos), ("rels.txt", &vocs.rels)] {
        fs::write(p.join(f), v.save()).map_err(e)?;
    }
    fs::write(p.join("cfg.txt"), cfg.save()).map_err(e)?;
    let mut rng = Rng(seed ^ 0x9e3779b97f4a7c15);
    let mut step = 0f64;
    let mut best = f64::MIN;
    let mut log = String::new();
    for ep in 1..=epochs {
        let t0 = std::time::Instant::now();
        let chosen = if ep == 1 { first.clone() } else { pick(&mut rng0) };
        let plans = data::plan_of(&sents, &chosen, tokens);
        let mut order: Vec<usize> = (0..plans.len()).collect();
        for i in (1..order.len()).rev() {
            order.swap(i, (rng.next() % (i as u64 + 1)) as usize);
        }
        let (mut sum, mut cnt) = (0f64, 0usize);
        for &bi in &order {
            step += 1.;
            let lr_now = lr * (step / warm).min(1.) * (1. - step / total).max(0.02);
            opt.set_learning_rate(lr_now);
            let refs: Vec<&Sent> = plans[bi].iter().map(|&i| &sents[i]).collect();
            let x = data::batch_prag(&refs, &vocs, 0.15, prag_drop, &mut rng);
            let o = net.forward(&x, &dev, true).map_err(e)?;
            let n = x.rows.len();
            let rows = Tensor::from_slice(&x.rows, n, &dev).map_err(e)?;
            let arcs = o.arcs.reshape((x.b * x.t, x.t)).and_then(|z| z.index_select(&rows, 0)).map_err(e)?;
            let heads = Tensor::from_slice(&x.heads, n, &dev).map_err(e)?;
            let la = loss::cross_entropy(&arcs, &heads).map_err(e)?;
            let hflat: Vec<u32> = x.rows.iter().zip(&x.heads).map(|(&r, &h)| r - r % x.t as u32 + h).collect();
            let ht = Tensor::from_slice(&hflat, n, &dev).map_err(e)?;
            let lr_ = loss::cross_entropy(&net.rels(&o, &rows, &ht, true).map_err(e)?, &Tensor::from_slice(&x.rels, n, &dev).map_err(e)?).map_err(e)?;
            let (xp, up) = net.tags(&o, &rows).map_err(e)?;
            let lx = loss::cross_entropy(&xp, &Tensor::from_slice(&x.xpos, n, &dev).map_err(e)?).map_err(e)?;
            let lu = loss::cross_entropy(&up, &Tensor::from_slice(&x.upos, n, &dev).map_err(e)?).map_err(e)?;
            let total_loss = (la + lr_).and_then(|z| z + ((lx + lu)? * 0.5)?).map_err(e)?;
            opt.backward_step(&total_loss).map_err(e)?;
            sum += total_loss.to_scalar::<f32>().map_err(e)? as f64;
            cnt += 1;
        }
        let train_s = t0.elapsed().as_secs_f64();
        let mut line = format!("epoch {ep:2}: loss {:.4}, {:.0} s", sum / cnt as f64, train_s);
        let mut las_sum = 0.;
        for (name, ds) in &devs {
            let pred = predict(&net, &vocs, &dev, ds)?;
            let (u, l, x, up, _) = score(ds, &pred);
            las_sum += l;
            let short = Path::new(name).file_name().map(|z| z.to_string_lossy().to_string()).unwrap_or_default();
            line += &format!(" | {short} UAS {u:.2} LAS {l:.2} XPOS {x:.2} UPOS {up:.2}");
        }
        let mean = if devs.is_empty() { ep as f64 } else { las_sum / devs.len() as f64 };
        if mean > best {
            best = mean;
            vm.save(p.join("model.safetensors")).map_err(e)?;
            line += " *";
        }
        println!("{line}");
        log += &line;
        log.push('\n');
        fs::write(p.join("train.log"), &log).map_err(e)?;
    }
    Ok(())
}

fn eval(dir: &str, tests: &[String]) -> R<()> {
    let m = load(dir)?;
    for t in tests {
        let mut ds = read_all(std::slice::from_ref(t))?;
        with_roles(&mut ds)?;
        let t0 = std::time::Instant::now();
        let pred = predict(&m.net, &m.vocs, &m.dev, &ds)?;
        let (u, l, x, up, n) = score(&ds, &pred);
        println!("{t}: words {n}, UAS {u:.2}, LAS {l:.2}, XPOS {x:.2}, UPOS {up:.2}; {:.0} words/s", n as f64 / t0.elapsed().as_secs_f64());
    }
    Ok(())
}

fn annotate(dir: &str, inp: &str, out: &str) -> R<()> {
    let m = load(dir)?;
    let mut ds = read_all(&[inp.to_string()])?;
    with_roles(&mut ds)?;
    let pred = predict(&m.net, &m.vocs, &m.dev, &ds)?;
    // GP_LEX=<corpus.conllu>: a full annotation out — LEMMA and FEATS kept where the tags did not change, refilled
    // from the corpus where they did; relations limited to the ones the corpus uses
    let lex = match std::env::var("GP_LEX") {
        Ok(p) => data::Lex::learn(&read_all(&[p])?),
        Err(_) => data::Lex::default(),
    };
    for (s, p) in ds.iter_mut().zip(pred) {
        for (w, q) in s.words.iter_mut().zip(p) {
            if (w.upos.as_str(), w.xpos.as_str()) != (q.3.as_str(), q.2.as_str()) {
                let (f, l) = lex.fill(&w.form, &w.lemma, &q.3, &q.2);
                w.feats = f;
                w.lemma = l;
            }
            w.head = q.0;
            w.rel = if q.0 == 0 { "root".into() } else { lex.rel(&q.1) };
            w.xpos = q.2;
            w.upos = q.3;
        }
    }
    fs::write(out, data::write(&ds)).map_err(e)
}
