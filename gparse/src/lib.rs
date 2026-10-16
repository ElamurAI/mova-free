//! gparse as a library: the GPU parser for other crates (the GPU-enhanced mode of Mova's pipelines —
//! `world` parses through it with MOVA_EXEC=gpu). The command line is `main.rs`.

pub mod data;
pub mod eisner;
pub mod model;

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
    let cfg = Cfg::load(&rd("cfg.txt")?);
    let vocs = Vocs { words: data::Voc::load(&rd("words.txt")?), xpos: data::Voc::load(&rd("xpos.txt")?), upos: data::Voc::load(&rd("upos.txt")?), rels: data::Voc::load(&rd("rels.txt")?) };
    let dev = if std::env::var("GP_CPU").is_ok() { Device::Cpu } else { Device::cuda_if_available(0).map_err(e)? };
    let vm = VarMap::new();
    let vb = VarBuilder::from_varmap(&vm, DType::F32, &dev);
    let net = Net::new(cfg, vocs.words.len(), vocs.xpos.len(), vocs.upos.len(), vocs.rels.len(), vb).map_err(e)?;
    load_vars(&vm, &p.join("model.safetensors"), &dev)?;
    Ok(Loaded { net, vocs, dev })
}

/// Parse sentences (gold tokens); returns predicted (head, rel, xpos, upos) per word.
pub fn predict(net: &Net, vocs: &Vocs, dev: &Device, sents: &[Sent]) -> R<Vec<Vec<(usize, String, String, String)>>> {
    let mut out: Vec<Vec<(usize, String, String, String)>> = vec![Vec::new(); sents.len()];
    let mut rng = Rng(1);
    // GP_BATCH: words per batch at inference (6000 by default; larger batches move more at once)
    let per_batch: usize = std::env::var("GP_BATCH").ok().and_then(|v| v.parse().ok()).unwrap_or(6000);
    let timing = std::env::var("GP_TIMING").is_ok();
    let (mut t_fwd, mut t_dec, mut t_rel) = (0f64, 0f64, 0f64);
    for plan in data::plan(sents, per_batch) {
        let t0 = std::time::Instant::now();
        let refs: Vec<&Sent> = plan.iter().map(|&i| &sents[i]).collect();
        let x = data::batch(&refs, vocs, 0., &mut rng);
        let o = net.forward(&x, dev, false).map_err(e)?;
        // the arc scores as one flat copy: [sentence][dependent][head], t × t per sentence
        let t = x.t;
        let lpf: Vec<f32> = ops::log_softmax(&o.arcs, D::Minus1).and_then(|z| z.flatten_all()).and_then(|z| z.to_vec1()).map_err(e)?;
        let lp = |k: usize, d: usize, h: usize| lpf[(k * t + d) * t + h];
        let rows = Tensor::from_slice(&x.rows, x.rows.len(), dev).map_err(e)?;
        let (xp, up) = net.tags(&o, &rows).map_err(e)?;
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
        let rl: Vec<u32> = net.rels(&o, &rows, &ht, false).and_then(|z| z.argmax(D::Minus1)).and_then(|z| z.to_vec1()).map_err(e)?;
        let mut j = 0;
        for (k, &si) in plan.iter().enumerate() {
            for (w, &h) in per[k].iter().enumerate() {
                let _ = w;
                let rel = if h == 0 { "root".to_string() } else { vocs.rels.names[rl[j] as usize].clone() };
                let rel = if h != 0 && rel == "root" { "dep".to_string() } else { rel };
                out[si].push((h, rel, vocs.xpos.names[xp[j] as usize].clone(), vocs.upos.names[up[j] as usize].clone()));
                j += 1;
            }
        }
        t_rel += t2.elapsed().as_secs_f64();
    }
    if timing {
        eprintln!("gparse predict: forward + copy {t_fwd:.2}s, eisner {t_dec:.2}s, relations {t_rel:.2}s");
    }
    Ok(out)
}

/// Parse tokenized sentences: (head, relation, XPOS, UPOS) per word.
pub fn parse_forms(m: &Loaded, sents: &[Vec<String>]) -> R<Vec<Vec<(usize, String, String, String)>>> {
    let ss: Vec<Sent> = sents
        .iter()
        .map(|ws| Sent {
            comments: Vec::new(),
            words: ws.iter().map(|f| data::Word { form: f.clone(), lemma: "_".into(), feats: "_".into(), upos: "_".into(), xpos: "_".into(), head: 0, rel: "_".into() }).collect(),
            roles: Vec::new(),
        })
        .collect();
    predict(&m.net, &m.vocs, &m.dev, &ss)
}
