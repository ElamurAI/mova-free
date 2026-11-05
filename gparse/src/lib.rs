//! gparse as a library: the GPU parser for other crates (the GPU-enhanced mode of Mova's pipelines —
//! `world` parses through it with MOVA_EXEC=gpu). The command line is `main.rs`.

pub mod data;
pub mod eisner;
pub mod model;
pub mod softmax;

use std::fs;
use std::path::Path;

use candle_core::{DType, Device, Tensor, D};
use candle_nn::{ops, VarBuilder, VarMap};

use data::{Rng, Sent, Vocs};
use model::{Cfg, Net};

pub type R<T> = std::result::Result<T, String>;

pub fn e<E: std::fmt::Display>(x: E) -> String {
    x.to_string()
}

/// Weights from a file into the model; weights the file lacks (a part added later, e.g. the pragmatic inputs) keep
/// their init — an old model goes on working and learns the new part.
pub fn load_vars(vm: &VarMap, path: &Path, dev: &Device) -> R<()> {
    let ts = candle_core::safetensors::load(path, dev).map_err(e)?;
    for (name, var) in vm.data().lock().map_err(e)?.iter() {
        match ts.get(name) {
            Some(t) => var.set(t).map_err(e)?,
            None => eprintln!("{}: no {name}, starts fresh", path.display()),
        }
    }
    Ok(())
}

pub struct Loaded {
    pub net: Net,
    pub vocs: Vocs,
    pub dev: Device,
}

pub fn load(dir: &str) -> R<Loaded> {
    let p = Path::new(dir);
    let rd = |f: &str| fs::read_to_string(p.join(f)).map_err(|x| format!("{dir}/{f}: {x}"));
    let cfg = Cfg::load(&rd("cfg.txt")?).map_err(e)?;
    let vocs = Vocs { words: data::Voc::load(&rd("words.txt")?), xpos: data::Voc::load(&rd("xpos.txt")?), upos: data::Voc::load(&rd("upos.txt")?), rels: data::Voc::load(&rd("rels.txt")?) };
    let dev = if std::env::var("GP_CPU").is_ok() { Device::Cpu } else { Device::cuda_if_available(0).map_err(e)? };
    let vm = VarMap::new();
    let vb = VarBuilder::from_varmap(&vm, DType::F32, &dev);
    let net = Net::new(cfg, vocs.words.len(), vocs.xpos.len(), vocs.upos.len(), vocs.rels.len(), vb).map_err(e)?;
    load_vars(&vm, &p.join("model.safetensors"), &dev)?;
    Ok(Loaded { net, vocs, dev })
}

/// Parse sentences (gold tokens); returns predicted (head, rel, xpos, upos) per word.
/// The parse of every sentence. A sentence longer than the model's positions (`max_len`, the same bound training
/// uses) keeps the tree it came with, and the count is logged: an old book's 257-word sentence once ran the position
/// table out of range on the GPU (16.10).
pub fn predict(net: &Net, vocs: &Vocs, dev: &Device, sents: &[Sent]) -> R<Vec<Vec<(usize, String, String, String)>>> {
    predict_impl(net, vocs, dev, sents, false).map(|x| x.0)
}

/// `predict`, with each sentence's tree score: the sum over its words of the log-probabilities of the chosen head
/// and relation and half those of the two tags (the weights of the training loss). A sentence too long to parse
/// scores 0.
pub fn predict_scored(net: &Net, vocs: &Vocs, dev: &Device, sents: &[Sent]) -> R<(Vec<Vec<(usize, String, String, String)>>, Vec<f64>)> {
    predict_impl(net, vocs, dev, sents, true)
}

// Decoding is shared. Only callers that request scores pay for the extra GPU reductions and host copies.
fn predict_impl(net: &Net, vocs: &Vocs, dev: &Device, sents: &[Sent], scored: bool) -> R<(Vec<Vec<(usize, String, String, String)>>, Vec<f64>)> {
    let fits: Vec<usize> = (0..sents.len()).filter(|&i| sents[i].words.len() < net.cfg.max_len).collect();
    if fits.len() == sents.len() {
        return predict_fitting(net, vocs, dev, sents, scored);
    }
    eprintln!("gparse: {} sentences longer than {} words keep their input tree", sents.len() - fits.len(), net.cfg.max_len - 1);
    let short: Vec<Sent> = fits.iter().map(|&i| sents[i].clone()).collect();
    let (pred, sc) = predict_fitting(net, vocs, dev, &short, scored)?;
    let (mut pred, mut sc) = (pred.into_iter(), sc.into_iter());
    let mut at = fits.iter().peekable();
    let mut scores = Vec::with_capacity(sents.len());
    let out = (0..sents.len())
        .map(|i| {
            if at.peek() == Some(&&i) {
                at.next();
                scores.push(sc.next().unwrap_or(0.));
                pred.next().unwrap_or_default()
            } else {
                scores.push(0.);
                sents[i].words.iter().map(|w| (w.head, w.rel.clone(), w.xpos.clone(), w.upos.clone())).collect()
            }
        })
        .collect();
    Ok((out, scores))
}

#[allow(clippy::type_complexity)]
fn predict_fitting(net: &Net, vocs: &Vocs, dev: &Device, sents: &[Sent], scored: bool) -> R<(Vec<Vec<(usize, String, String, String)>>, Vec<f64>)> {
    let mut out: Vec<Vec<(usize, String, String, String)>> = vec![Vec::new(); sents.len()];
    let mut scores = vec![0f64; sents.len()];
    let mut rng = Rng(1);
    // GP_BATCH: words per batch at inference (6000 by default; larger batches move more at once)
    let per_batch: usize = std::env::var("GP_BATCH").ok().and_then(|v| v.parse().ok()).unwrap_or(6000);
    let timing = std::env::var("GP_TIMING").is_ok();
    let (mut t_fwd, mut t_dec, mut t_rel) = (0f64, 0f64, 0f64);
    // Empty sentences have no predictions or score. Keep their slots, but never send
    // a root-only batch to tag/relation reductions or the nonempty Eisner recurrence.
    let nonempty: Vec<_> = (0..sents.len()).filter(|&i| !sents[i].words.is_empty()).collect();
    let fused_softmax = match std::env::var("GP_FUSED_SOFTMAX").as_deref() {
        Ok("1") => true,
        Ok("0") => false,
        Err(std::env::VarError::NotPresent) => dev.is_cuda(),
        Err(err) => return Err(format!("GP_FUSED_SOFTMAX: {err}")),
        Ok(value) => return Err(format!("GP_FUSED_SOFTMAX must be 0 or 1, got {value:?}")),
    };
    for plan in data::plan_of(sents, &nonempty, per_batch) {
        let t0 = std::time::Instant::now();
        let refs: Vec<&Sent> = plan.iter().map(|&i| &sents[i]).collect();
        let x = data::batch_for(&refs, vocs, net.cfg.ngram, &mut rng);
        let o = net.forward_inference(&x, dev, fused_softmax).map_err(e)?;
        // the arc scores as one flat copy: [sentence][dependent][head], t × t per sentence
        let t = x.t;
        let lpf: Vec<f32> = ops::log_softmax(&o.arcs, D::Minus1).and_then(|z| z.flatten_all()).and_then(|z| z.to_vec1()).map_err(e)?;
        let lp = |k: usize, d: usize, h: usize| lpf[(k * t + d) * t + h];
        let rows = Tensor::from_slice(&x.rows, x.rows.len(), dev).map_err(e)?;
        let (xp, up) = net.tags(&o, &rows).map_err(e)?;
        let (lxp, lup): (Vec<f32>, Vec<f32>) = if scored {
            let (lxp, lup) = (ops::log_softmax(&xp, D::Minus1).map_err(e)?, ops::log_softmax(&up, D::Minus1).map_err(e)?);
            (lxp.max(D::Minus1).and_then(|z| z.to_vec1()).map_err(e)?,
             lup.max(D::Minus1).and_then(|z| z.to_vec1()).map_err(e)?)
        } else {
            (Vec::new(), Vec::new())
        };
        let xp: Vec<u32> = xp.argmax(D::Minus1).and_then(|z| z.to_vec1()).map_err(e)?;
        let up: Vec<u32> = up.argmax(D::Minus1).and_then(|z| z.to_vec1()).map_err(e)?;
        // heads by Eisner, then relations for those heads
        let mut heads_flat = Vec::with_capacity(x.rows.len());
        // Eisner per sentence, over threads (in order: the result does not depend on them)
        let decode = |k: usize| {
            let n = refs[k].words.len();
            let mut sc = vec![vec![0f32; n + 1]; n + 1];
            for d in 1..=n {
                for h in 0..=n {
                    sc[h][d] = lp(k, d, h);
                }
            }
            eisner::decode(&sc)
        };
        let t1 = std::time::Instant::now();
        t_fwd += (t1 - t0).as_secs_f64();
        let ks: Vec<usize> = (0..refs.len()).collect();
        let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(16);
        let chunk = ks.len().div_ceil(threads).max(1);
        let per: Vec<Vec<usize>> = std::thread::scope(|sc| {
            let hs: Vec<_> = ks.chunks(chunk).map(|c| sc.spawn(|| c.iter().map(|&k| decode(k)).collect::<Vec<_>>())).collect();
            hs.into_iter().flat_map(|h| h.join().expect("eisner thread")).collect()
        });
        for (k, hs) in per.iter().enumerate() {
            for &h in hs {
                heads_flat.push((k * x.t + h) as u32);
            }
        }
        let t2 = std::time::Instant::now();
        t_dec += (t2 - t1).as_secs_f64();
        let ht = Tensor::from_slice(&heads_flat, heads_flat.len(), dev).map_err(e)?;
        let rlogit = net.rels(&o, &rows, &ht, false).map_err(e)?;
        let rl: Vec<u32> = rlogit.argmax(D::Minus1).and_then(|z| z.to_vec1()).map_err(e)?;
        let lrl: Vec<f32> = if scored {
            ops::log_softmax(&rlogit, D::Minus1).and_then(|z| z.max(D::Minus1)).and_then(|z| z.to_vec1()).map_err(e)?
        } else {
            Vec::new()
        };
        let mut j = 0;
        for (k, &si) in plan.iter().enumerate() {
            for (w, &h) in per[k].iter().enumerate() {
                let rel = if h == 0 { "root".to_string() } else { vocs.rels.names[rl[j] as usize].clone() };
                let rel = if h != 0 && rel == "root" { "dep".to_string() } else { rel };
                out[si].push((h, rel, vocs.xpos.names[xp[j] as usize].clone(), vocs.upos.names[up[j] as usize].clone()));
                if scored {
                    scores[si] += (lp(k, w + 1, h) + lrl[j] + 0.5 * (lxp[j] + lup[j])) as f64;
                }
                j += 1;
            }
        }
        t_rel += t2.elapsed().as_secs_f64();
    }
    if timing {
        eprintln!("gparse predict: forward + copy {t_fwd:.2}s, eisner {t_dec:.2}s, relations {t_rel:.2}s");
    }
    Ok((out, scores))
}

/// Parse tokenized sentences: (head, relation, XPOS, UPOS) per word.
pub fn parse_forms(m: &Loaded, sents: &[Vec<String>]) -> R<Vec<Vec<(usize, String, String, String)>>> {
    let ss: Vec<Sent> = sents
        .iter()
        .map(|ws| Sent {
            comments: Vec::new(),
            words: ws.iter().map(|f| data::Word { form: f.clone(), lemma: "_".into(), feats: "_".into(), upos: "_".into(), xpos: "_".into(), head: 0, rel: "_".into(), misc: "_".into() }).collect(),
            roles: Vec::new(),
        })
        .collect();
    predict(&m.net, &m.vocs, &m.dev, &ss)
}
