//! gparse — the SLM dependency parser trained by gradient descent on the GPU (RTX 4000, candle + CUDA).
//!
//! gparse train <model dir> <train.conllu>... -- <dev.conllu>...   AdamW, warmup + linear decay; the epoch with the best
//!                                                                  mean dev LAS is saved (GP_EPOCHS, GP_TOKENS, GP_LR, GP_SEED,
//!                                                                  GP_DROP, GP_LAYERS; GP_CPU=1 runs on the CPU)
//! gparse eval <model dir> <test.conllu>...                         UAS / LAS / XPOS / UPOS on gold tokens, then UAS / LAS
//!                                                                  by sentence length, seen vs unseen words, GUM genre
//! gparse annotate <model dir> <in.conllu> <out.conllu>             trees for gold tokens (GP_LEX=<corpus>: keep or
//!                                                                  refill LEMMA/FEATS, relations the corpus knows)
//!
//! Words come tokenized; the parser predicts XPOS, UPOS, heads and relations itself (no tagger in front).

use gparse::{data, e, load, load_vars, model, predict, predict_scored};

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
        Some("plan") if a.len() == 5 => plan_cost(&a[2], &a[3], &a[4]),
        _ => Err("usage: gparse plan <model dir> <in.conllu> <token budget> | train <model dir> <train.conllu>... -- <dev.conllu>... | eval <model dir> <test.conllu>... | annotate <model dir> <in.conllu> <out.conllu>".into()),
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
    let dev_every: usize = match std::env::var("GP_DEV_EVERY") {
        Ok(v) => v.parse().map_err(|_| "GP_DEV_EVERY must be a positive integer")?,
        Err(_) => 1,
    };
    let dev_quick: usize = match std::env::var("GP_DEV_QUICK") {
        Ok(v) => v.parse().map_err(|_| "GP_DEV_QUICK must be a positive integer")?,
        Err(_) => 256,
    };
    if dev_every == 0 || dev_quick == 0 { return Err("GP_DEV_EVERY and GP_DEV_QUICK must be positive".into()); }
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
    // Fixed, evenly spaced development probes; never mix their score with full-set checkpoint selection.
    let quick_devs: Vec<Vec<Sent>> = devs.iter().map(|(_,ds)| {
        let k = ds.len().min(dev_quick);
        (0..k).map(|i| ds[((2*i+1)*ds.len())/(2*k)].clone()).collect()
    }).collect();
    // GP_INIT=<model dir>: continue from that model (its configuration and vocabularies; new words are unknown)
    let init = std::env::var("GP_INIT").ok();
    // GP_SUBWORD=1: a new model reads every word also through its hashed character n-grams (GP_NGRAM_BUCKETS
    // buckets); a continued model keeps its own input, and asking for another one is an error
    let subword = match std::env::var("GP_SUBWORD").as_deref() {
        Ok("1") => Some(true),
        Ok("0") => Some(false),
        Err(_) => None,
        Ok(v) => return Err(format!("GP_SUBWORD must be 0 or 1, got {v:?}")),
    };
    let (cfg, vocs) = match &init {
        Some(d) => {
            let rd = |f: &str| fs::read_to_string(Path::new(d).join(f)).map_err(|x| format!("{d}/{f}: {x}"));
            let c = Cfg::load(&rd("cfg.txt")?).map_err(e)?;
            if subword.is_some_and(|w| w != (c.ngram > 0)) {
                return Err(format!("GP_SUBWORD={} disagrees with {d} (ngram={}); a continued model keeps its input", subword == Some(true), c.ngram));
            }
            (c, Vocs { words: data::Voc::load(&rd("words.txt")?), xpos: data::Voc::load(&rd("xpos.txt")?), upos: data::Voc::load(&rd("upos.txt")?), rels: data::Voc::load(&rd("rels.txt")?) })
        }
        None => {
            if subword == Some(true) {
                cfg.ngram = env("GP_NGRAM_BUCKETS", data::NGRAM_BUCKETS);
                if cfg.ngram < 2 {
                    return Err("GP_NGRAM_BUCKETS must be at least 2".into());
                }
            }
            (cfg, Vocs::build(&sents, 2))
        }
    };
    // GP_PA=<share>: partial annotation — a word whose head got less than this share of the agreement vote
    // (`VoteHead` in MISC) leaves the arc loss, one whose (head, relation) got less (`Vote`) leaves the relation loss;
    // words without a vote always stay. GP_PA=0 keeps every word (the reference). GP_SOFT=1: voted words learn the
    // vote's distribution over heads instead of the single winner. GP_PA_CONTROL=random (negative control): as many
    // words leave each loss, chosen at random.
    let pa: Option<f32> = match std::env::var("GP_PA") {
        Ok(v) => match v.parse::<f32>() {
            Ok(t) if (0. ..=1.).contains(&t) => Some(t),
            _ => return Err(format!("GP_PA must be a share in [0,1], got {v:?}")),
        },
        Err(_) => None,
    };
    let soft = match std::env::var("GP_SOFT").as_deref() {
        Ok("1") => true,
        Ok("0") | Err(_) => false,
        Ok(v) => return Err(format!("GP_SOFT must be 0 or 1, got {v:?}")),
    };
    let pa_random = match std::env::var("GP_PA_CONTROL").as_deref() {
        Ok("random") if pa.is_some() => true,
        Ok("random") => return Err("GP_PA_CONTROL=random needs GP_PA".into()),
        Ok(v) => return Err(format!("GP_PA_CONTROL must be random, got {v:?}")),
        Err(_) => false,
    };
    if pa.is_some() || soft {
        let voted = sents.iter().flat_map(|s| &s.words).filter(|w| data::vote(&w.misc).is_some()).count();
        println!("partial: {voted} voted words; threshold {pa:?}, soft targets {soft}, random control {pa_random}");
    }
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
    println!("model: {:?}, {:.1} M parameters, device {:?}", net.cfg, n_params as f64 / 1e6, dev.location());
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
    fs::write(p.join("cfg.txt"), net.cfg.save()).map_err(e)?;
    let mut rng = Rng(seed ^ 0x9e3779b97f4a7c15);
    let mut ctl = Rng(seed ^ 0x6a09e667f3bcc909);
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
        let (mut kept_a, mut kept_r, mut seen_n) = (0usize, 0usize, 0usize);
        for &bi in &order {
            step += 1.;
            let lr_now = lr * (step / warm).min(1.) * (1. - step / total).max(0.02);
            opt.set_learning_rate(lr_now);
            let refs: Vec<&Sent> = plans[bi].iter().map(|&i| &sents[i]).collect();
            let x = data::batch_full(&refs, &vocs, 0.15, prag_drop, net.cfg.ngram, &mut rng);
            let o = net.forward(&x, &dev, true).map_err(e)?;
            let n = x.rows.len();
            let rows = Tensor::from_slice(&x.rows, n, &dev).map_err(e)?;
            // the words each loss learns from (all of them without GP_PA)
            let (ka, kr) = keep(&x, pa, pa_random.then_some(&mut ctl));
            (kept_a, kept_r, seen_n) = (kept_a + ka.len(), kept_r + kr.len(), seen_n + n);
            let pick = |k: &[usize], v: &[u32]| -> Vec<u32> { k.iter().map(|&j| v[j]).collect() };
            let zero = || Tensor::zeros((), DType::F32, &dev);
            let la = if ka.is_empty() {
                zero().map_err(e)?
            } else {
                let rows_a = Tensor::from_slice(&pick(&ka, &x.rows), ka.len(), &dev).map_err(e)?;
                let arcs = o.arcs.reshape((x.b * x.t, x.t)).and_then(|z| z.index_select(&rows_a, 0)).map_err(e)?;
                if soft {
                    let q = Tensor::from_slice(&soft_targets(&x, &ka), (ka.len(), x.t), &dev).map_err(e)?;
                    candle_nn::ops::log_softmax(&arcs, candle_core::D::Minus1).and_then(|lp| (q * lp)?.sum(1)?.mean_all()?.neg()).map_err(e)?
                } else {
                    loss::cross_entropy(&arcs, &Tensor::from_slice(&pick(&ka, &x.heads), ka.len(), &dev).map_err(e)?).map_err(e)?
                }
            };
            let lr_ = if kr.is_empty() {
                zero().map_err(e)?
            } else {
                let rows_r = Tensor::from_slice(&pick(&kr, &x.rows), kr.len(), &dev).map_err(e)?;
                let hflat: Vec<u32> = kr.iter().map(|&j| x.rows[j] - x.rows[j] % x.t as u32 + x.heads[j]).collect();
                let ht = Tensor::from_slice(&hflat, kr.len(), &dev).map_err(e)?;
                loss::cross_entropy(&net.rels(&o, &rows_r, &ht, true).map_err(e)?, &Tensor::from_slice(&pick(&kr, &x.rels), kr.len(), &dev).map_err(e)?).map_err(e)?
            };
            let (xp, up) = net.tags(&o, &rows).map_err(e)?;
            let lx = loss::cross_entropy(&xp, &Tensor::from_slice(&x.xpos, n, &dev).map_err(e)?).map_err(e)?;
            let lu = loss::cross_entropy(&up, &Tensor::from_slice(&x.upos, n, &dev).map_err(e)?).map_err(e)?;
            let total_loss = (la + lr_).and_then(|z| z + ((lx + lu)? * 0.5)?).map_err(e)?;
            opt.backward_step(&total_loss).map_err(e)?;
            sum += total_loss.to_scalar::<f32>().map_err(e)? as f64;
            cnt += 1;
        }
        let train_s = t0.elapsed().as_secs_f64();
        if pa.is_some() || soft {
            println!("partial {ep}: arcs from {:.2}% of words, relations from {:.2}%", 100. * kept_a as f64 / seen_n.max(1) as f64, 100. * kept_r as f64 / seen_n.max(1) as f64);
        }
        let mut line = format!("epoch {ep:2}: loss {:.4}, {:.0} s", sum / cnt as f64, train_s);
        let full_dev = ep % dev_every == 0 || ep == epochs;
        let mut las_sum = 0.;
        for (i, (name, all)) in devs.iter().enumerate() {
            let ds = if full_dev { all } else { &quick_devs[i] };
            let pred = predict(&net, &vocs, &dev, ds)?;
            let (u, l, x, up, _) = score(ds, &pred);
            las_sum += l;
            let short = Path::new(name).file_name().map(|z| z.to_string_lossy().to_string()).unwrap_or_default();
            if !full_dev { line += &format!(" | proxy {}/{} sentences",ds.len(),all.len()); }
            line += &format!(" | {short} UAS {u:.2} LAS {l:.2} XPOS {x:.2} UPOS {up:.2}");
        }
        let mean = if devs.is_empty() { ep as f64 } else { las_sum / devs.len() as f64 };
        if (devs.is_empty() || full_dev) && mean > best {
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
        // GP_EVAL_SCRAMBLE=1 (negative control of the subword input): the letters of every word the model has not
        // seen are put in a fixed pseudo-random order, so its spelling no longer tells anything
        if std::env::var("GP_EVAL_SCRAMBLE").is_ok_and(|v| v == "1") {
            for w in ds.iter_mut().flat_map(|s| s.words.iter_mut()) {
                if m.vocs.words.get(&w.form.to_lowercase()).is_none() {
                    w.form = scramble(&w.form);
                }
            }
        }
        let t0 = std::time::Instant::now();
        let pred = predict(&m.net, &m.vocs, &m.dev, &ds)?;
        let (u, l, x, up, n) = score(&ds, &pred);
        println!("{t}: words {n}, UAS {u:.2}, LAS {l:.2}, XPOS {x:.2}, UPOS {up:.2}; {:.0} words/s", n as f64 / t0.elapsed().as_secs_f64());
        for (k, (n, u, l)) in breakdown(&ds, &pred, &m.vocs.words) {
            println!("  {k}: words {n}, UAS {:.2}, LAS {:.2}", 100. * u as f64 / n.max(1) as f64, 100. * l as f64 / n.max(1) as f64);
        }
    }
    Ok(())
}

/// Which words each loss learns from: (arc rows, relation rows) as indices into the batch's rows. Without a
/// threshold, all. With one, a word leaves the arc loss when its head's vote share is below it and the relation loss
/// when its (head, relation) share is; a word with no vote has share 1. With `random`, the same numbers of words
/// leave, drawn at random (nested: the relation loss loses the arc loss's words and more).
fn keep(x: &data::Batch, pa: Option<f32>, random: Option<&mut Rng>) -> (Vec<usize>, Vec<usize>) {
    let n = x.rows.len();
    let Some(t) = pa else { return ((0..n).collect(), (0..n).collect()) };
    let ka: Vec<usize> = (0..n).filter(|&j| x.arc_conf[j] >= t).collect();
    let kr: Vec<usize> = (0..n).filter(|&j| x.rel_conf[j] >= t).collect();
    let Some(r) = random else { return (ka, kr) };
    let mut perm: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        perm.swap(i, (r.next() % (i as u64 + 1)) as usize);
    }
    let rest = |drop: usize| -> Vec<usize> {
        let mut v = perm[drop.min(n)..].to_vec();
        v.sort_unstable();
        v
    };
    (rest(n - ka.len()), rest(n - kr.len()))
}

/// Soft arc targets (rows `k` × T): the vote's shares over heads (the gold head's share and the alternatives'),
/// normalized; a word with no vote gets its gold head with 1.
fn soft_targets(x: &data::Batch, k: &[usize]) -> Vec<f32> {
    let t = x.t;
    let mut q = vec![0f32; k.len() * t];
    for (i, &j) in k.iter().enumerate() {
        let row = &mut q[i * t..(i + 1) * t];
        row[x.heads[j] as usize] = if x.alts[j].is_empty() { 1. } else { x.arc_conf[j].max(1e-3) };
        for &(h, s) in &x.alts[j] {
            if (h as usize) < t {
                row[h as usize] += s.max(0.);
            }
        }
        let z: f32 = row.iter().sum();
        row.iter_mut().for_each(|v| *v /= z);
    }
    q
}

/// The word's letters in a pseudo-random order fixed by the word itself (Fisher–Yates over a seeded xorshift).
fn scramble(w: &str) -> String {
    let mut cs: Vec<char> = w.chars().collect();
    let mut r = Rng(w.bytes().fold(0x9e3779b97f4a7c15u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3)) | 1);
    for i in (1..cs.len()).rev() {
        cs.swap(i, (r.next() % (i as u64 + 1)) as usize);
    }
    cs.into_iter().collect()
}

/// The length bucket of a sentence of `n` words.
fn length_bucket(n: usize) -> &'static str {
    match n {
        0..=10 => "length 1-10",
        11..=20 => "length 11-20",
        21..=40 => "length 21-40",
        _ => "length 41+",
    }
}

/// UAS/LAS counts (words, right heads, right heads and relations) by sentence length, by whether the word is in the
/// model's vocabulary (`seen`; an unseen word reads as <unk>), and by GUM genre (`GUM_<genre>_…` sentence ids).
fn breakdown(sents: &[Sent], pred: &[Vec<(usize, String, String, String)>], words: &data::Voc) -> Vec<(String, (usize, usize, usize))> {
    let mut by: std::collections::BTreeMap<(u8, String), (usize, usize, usize)> = std::collections::BTreeMap::new();
    for (s, p) in sents.iter().zip(pred) {
        let len = (0u8, length_bucket(s.words.len()).to_string());
        let genre = s.id().strip_prefix("GUM_").and_then(|g| g.split('_').next()).map(|g| (2u8, format!("genre {g}")));
        for (w, q) in s.words.iter().zip(p) {
            let (u, l) = ((w.head == q.0) as usize, (w.head == q.0 && w.rel == q.1) as usize);
            let seen = (1u8, if words.get(&w.form.to_lowercase()).is_some() { "seen words" } else { "unseen words" }.to_string());
            for k in std::iter::once(&len).chain(std::iter::once(&seen)).chain(genre.as_ref()) {
                let c = by.entry(k.clone()).or_default();
                *c = (c.0 + 1, c.1 + u, c.2 + l);
            }
        }
    }
    // length buckets in their natural order (the string order puts 41+ before 11-20)
    let rank = |k: &(u8, String)| (k.0, ["length 1-10", "length 11-20", "length 21-40", "length 41+"].iter().position(|x| *x == k.1));
    let mut v: Vec<_> = by.into_iter().collect();
    v.sort_by(|a, b| rank(&a.0).cmp(&rank(&b.0)).then(a.0.cmp(&b.0)));
    v.into_iter().map(|((_, k), c)| (k, c)).collect()
}

fn annotate(dir: &str, inp: &str, out: &str) -> R<()> {
    let m = load(dir)?;
    let mut ds = read_all(&[inp.to_string()])?;
    with_roles(&mut ds)?;
    let (pred, scores) = predict_scored(&m.net, &m.vocs, &m.dev, &ds)?;
    // GP_SCORE=1: each sentence's tree score as a comment (`# gp_score = …`), for a reader that chooses between
    // readings of the same text (hbo's lattice)
    if std::env::var("GP_SCORE").is_ok() {
        for (s, sc) in ds.iter_mut().zip(&scores) {
            s.comments.retain(|c| !c.starts_with("# gp_score = "));
            s.comments.push(format!("# gp_score = {sc:.4}"));
        }
    }
    // GP_LEX=<corpus.conllu>: a full annotation out — LEMMA and FEATS kept where the tags did not change, refilled
    // from the corpus where they did; relations limited to the ones the corpus uses. The words of the EModE table
    // (`en::emode`) take LEMMA and FEATS from it in both cases, without GP_LEX too.
    let lex = match std::env::var("GP_LEX") {
        Ok(p) => data::Lex::learn(&read_all(&[p])?),
        Err(_) => data::Lex::default(),
    };
    for (s, p) in ds.iter_mut().zip(pred) {
        let forms: Vec<String> = s.words.iter().map(|w| w.form.clone()).collect();
        let forms: Vec<&str> = forms.iter().map(String::as_str).collect();
        for (i, (w, q)) in s.words.iter_mut().zip(p).enumerate() {
            if (w.upos.as_str(), w.xpos.as_str()) != (q.3.as_str(), q.2.as_str()) {
                let (f, l) = lex.fill(&forms, i, &w.lemma, &q.3, &q.2, &q.1);
                w.feats = f;
                w.lemma = l;
            } else if let Some((f, l)) = data::Lex::table(&forms, i, &q.3, &q.2, &q.1, &w.feats) {
                // the EModE table also for a word whose tags stay (F1)
                w.feats = f;
                w.lemma = l;
            }
            w.head = q.0;
            w.rel = if q.0 == 0 { "root".into() } else { lex.rel(&q.1) };
            w.xpos = q.2;
            w.upos = q.3;
        }
        postverbal_not(&mut s.words);
    }
    fs::write(out, data::write(&ds)).map_err(e)
}

/// S1 of the EModE manual (`en::emode::postverbal_not`): «I know not the day» — «not» on its lexical verb.
fn postverbal_not(ws: &mut [data::Word]) {
    use en::gram::{Rel, Tag, UPos};
    if !ws.iter().any(|w| w.form.eq_ignore_ascii_case("not")) {
        return;
    }
    let forms: Vec<&str> = ws.iter().map(|w| w.form.as_str()).collect();
    let upos: Vec<UPos> = ws.iter().map(|w| UPos::parse(&w.upos).unwrap_or(UPos::X)).collect();
    let tags: Vec<Tag> = ws.iter().map(|w| Tag::parse(&w.xpos).unwrap_or(Tag::FW)).collect();
    let mut heads: Vec<usize> = ws.iter().map(|w| w.head).collect();
    let mut rels: Vec<Rel> = ws.iter().map(|w| Rel::parse(&w.rel).unwrap_or(Rel::Dep)).collect();
    if en::emode::postverbal_not(&forms, &upos, &tags, &mut heads, &mut rels) == 0 {
        return;
    }
    for (i, w) in ws.iter_mut().enumerate() {
        if w.head != heads[i] {
            w.head = heads[i];
            w.rel = rels[i].to_string();
        }
    }
}

/// The actual inference batch planner supplies structural work, without loading CUDA/weights.
/// Linear encoder work scales with padded_tokens; attention/arc work with attention_cells.
/// Eisner is cubic in sentence length. These are cost features, not elapsed-time promises.
fn plan_cost(dir: &str, corpus: &str, budget: &str) -> R<()> {
    let tokens: usize = budget.parse().map_err(|_| "invalid token budget")?;
    if tokens == 0 { return Err("token budget must be positive".into()); }
    let cfg = Cfg::load(&fs::read_to_string(Path::new(dir).join("cfg.txt")).map_err(e)?).map_err(e)?;
    let sents = read_all(&[corpus.into()])?;
    let c = data::inference_cost(&sents, tokens, cfg.max_len);
    println!("{{\"sentences\":{},\"words\":{},\"skipped\":{},\"batches\":{},\"padded_tokens\":{},\"attention_cells\":{},\"decode_cubes\":{}}}",
        c.sentences,c.words,c.skipped,c.batches,c.padded_tokens,c.attention_cells,c.decode_cubes);
    Ok(())
}

#[cfg(test)]
mod wave30 {
    use super::*;
    #[test]
    fn breakdown_by_length_vocabulary_and_genre() {
        let mut s = data::read("# sent_id = GUM_bio_x-1\n1\tCat\tcat\tNOUN\tNN\t_\t0\troot\t_\t_\n2\tzork\tzork\tVERB\tVBZ\t_\t1\tdep\t_\t_\n\n");
        let long = data::read(&(1..=41).map(|i| format!("{i}\tcat\tcat\tNOUN\tNN\t_\t{}\tdep\t_\t_\n", i - 1)).collect::<String>());
        s.extend(long);
        let mut v = data::Voc::with(&["<pad>", "<unk>", "<root>"]);
        v.add("cat");
        let p0 = vec![(0, "root".to_string(), "NN".to_string(), "NOUN".to_string()), (2, "dep".into(), "VBZ".into(), "VERB".into())];
        let p1: Vec<_> = (1..=41).map(|i| (i - 1, if i == 1 { "root" } else { "dep" }.to_string(), "NN".to_string(), "NOUN".to_string())).collect();
        let b = breakdown(&s, &[p0, p1], &v);
        let get = |k: &str| b.iter().find(|x| x.0 == k).map(|x| x.1);
        assert_eq!(get("length 1-10"), Some((2, 1, 1)));
        assert_eq!(get("length 41+"), Some((41, 41, 40)), "the first word's relation is wrong (root vs dep)");
        assert_eq!(get("unseen words"), Some((1, 0, 0)));
        assert_eq!(get("seen words"), Some((42, 42, 41)));
        assert_eq!(get("genre bio"), Some((2, 1, 1)));
        assert_eq!(b.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), ["length 1-10", "length 41+", "seen words", "unseen words", "genre bio"]);
    }

    #[test]
    fn scramble_is_a_fixed_permutation() {
        let a = scramble("photosynthesis");
        assert_eq!(a, scramble("photosynthesis"));
        let (mut x, mut y): (Vec<char>, Vec<char>) = (a.chars().collect(), "photosynthesis".chars().collect());
        x.sort_unstable();
        y.sort_unstable();
        assert_eq!(x, y);
        assert_ne!(a, "photosynthesis");
        assert_eq!(scramble("a"), "a");
    }

    #[test]
    fn partial_annotation_keeps_and_drops() {
        let s = data::read("1\tA\ta\tX\tX\t_\t0\troot\t_\tVote=1\n2\tB\tb\tX\tX\t_\t1\tdep\t_\tVote=0.5|VoteHead=0.8|VoteAlt=0:0.2\n3\tC\tc\tX\tX\t_\t1\tdep\t_\tVote=0.4|VoteHead=0.4|VoteAlt=2:0.6\n4\tD\td\tX\tX\t_\t1\tdep\t_\t_\n\n");
        let v = Vocs::build(&s, 1);
        let x = data::batch(&[&s[0]], &v, 0., &mut Rng(1));
        // no threshold and a threshold of 0 keep everything (the reference); 0.75 drops C's arc and B's and C's relation
        let all = ((0..4).collect::<Vec<_>>(), (0..4).collect::<Vec<_>>());
        assert_eq!(keep(&x, None, None), all);
        assert_eq!(keep(&x, Some(0.), None), all);
        assert_eq!(keep(&x, Some(0.75), None), (vec![0, 1, 3], vec![0, 3]));
        // the random control drops as many, nested, and its draw depends only on its own generator
        let (a, r) = keep(&x, Some(0.75), Some(&mut Rng(7)));
        assert_eq!((a.len(), r.len()), (3, 2));
        assert!(r.iter().all(|j| a.contains(j)));
        assert_eq!(keep(&x, Some(0.75), Some(&mut Rng(7))), (a, r));
        // soft targets: B spreads over its gold head 1 and the root, C over 1 and 2; D and A stay one-hot
        let q = soft_targets(&x, &[0, 1, 2, 3]);
        let t = x.t;
        assert_eq!(&q[0..t], &[1., 0., 0., 0., 0.]);
        assert!((q[t] - 0.2).abs() < 1e-6 && (q[t + 1] - 0.8).abs() < 1e-6);
        assert!((q[2 * t + 1] - 0.4).abs() < 1e-6 && (q[2 * t + 2] - 0.6).abs() < 1e-6);
        assert_eq!(&q[3 * t..4 * t], &[0., 1., 0., 0., 0.]);
    }

    #[test]
    fn score_zip_and_plan_failure_precedence() {
        let s = data::read("1\tCat\tcat\tNOUN\tNN\t_\t0\troot\t_\t_\n2\tsleeps\tsleep\tVERB\tVBZ\t_\t1\tdep\t_\t_\n");
        let p = vec![vec![(1, "root".into(), "NN".into(), "VERB".into())]];
        assert_eq!(score(&s, &p), (0., 0., 100., 0., 1));
        assert_eq!(score(&[], &[]), (0., 0., 0., 0., 0));
        assert_eq!(
            plan_cost("absent", "absent", "0").unwrap_err(),
            "token budget must be positive"
        );
        assert_eq!(
            plan_cost("absent", "absent", "bad").unwrap_err(),
            "invalid token budget"
        );
    }
}
