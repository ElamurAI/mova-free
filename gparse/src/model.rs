//! The network: summed embeddings (word, prefix-3, suffix-3, shape, position) → pre-LN transformer encoder →
//! biaffine arc scorer (Dozat & Manning 2017) + bilinear relation scorer + XPOS/UPOS classifiers.
//! Everything learns from scratch on our trees: no pretrained weights, no foreign licences inside.

use candle_core::{Device, Result, Tensor, D};
use candle_nn::{linear, ops, Embedding, Linear, Module, VarBuilder};

use crate::data::{Batch, AFFIX};

#[derive(Clone, Copy, Debug)]
pub struct Cfg {
    pub d: usize,
    pub layers: usize,
    pub heads: usize,
    pub ff: usize,
    pub arc: usize,
    pub rel: usize,
    pub max_len: usize,
    pub dropout: f32,
}

impl Cfg {
    pub const DEFAULT: Cfg = Cfg { d: 256, layers: 4, heads: 8, ff: 1024, arc: 384, rel: 128, max_len: 256, dropout: 0.2 };
    pub fn save(&self) -> String {
        format!("d={}\nlayers={}\nheads={}\nff={}\narc={}\nrel={}\nmax_len={}\ndropout={}", self.d, self.layers, self.heads, self.ff, self.arc, self.rel, self.max_len, self.dropout)
    }
    pub fn load(s: &str) -> Cfg {
        let mut c = Cfg::DEFAULT;
        for l in s.lines() {
            let Some((k, v)) = l.split_once('=') else { continue };
            match k {
                "d" => c.d = v.parse().unwrap_or(c.d),
                "layers" => c.layers = v.parse().unwrap_or(c.layers),
                "heads" => c.heads = v.parse().unwrap_or(c.heads),
                "ff" => c.ff = v.parse().unwrap_or(c.ff),
                "arc" => c.arc = v.parse().unwrap_or(c.arc),
                "rel" => c.rel = v.parse().unwrap_or(c.rel),
                "max_len" => c.max_len = v.parse().unwrap_or(c.max_len),
                "dropout" => c.dropout = v.parse().unwrap_or(c.dropout),
                _ => {}
            }
        }
        c
    }
}

struct Block {
    ln1: LayerNorm,
    qkv: Linear,
    out: Linear,
    ln2: LayerNorm,
    ff1: Linear,
    ff2: Linear,
}

pub struct Net {
    pub cfg: Cfg,
    word: Embedding,
    pre: Embedding,
    suf: Embedding,
    shape: Embedding,
    pos: Embedding,
    /// pragmatics read back: the token's role / predicate flag, and a learned pull of each role on its arc
    prag_tok: Embedding,
    prag_arc: Embedding,
    blocks: Vec<Block>,
    ln: LayerNorm,
    arc_d: Linear,
    arc_h: Linear,
    arc_w: Tensor,
    arc_u: Tensor,
    rel_d: Linear,
    rel_h: Linear,
    rel_w: Tensor,
    rel_lin: Linear,
    xpos: Linear,
    upos: Linear,
    n_rel: usize,
}

/// Outputs for one batch: arc scores (B,T,T) and the encoder states.
pub struct Out {
    pub arcs: Tensor,
    pub h: Tensor,
}

/// Layer norm from plain tensor ops: candle's fused `LayerNorm` kernel passes no gradient back (measured 15.10:
/// embeddings and blocks stayed at their random init, only the heads learned).
pub struct LayerNorm {
    w: Tensor,
    b: Tensor,
}

impl LayerNorm {
    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let mean = x.mean_keepdim(D::Minus1)?;
        let xc = x.broadcast_sub(&mean)?;
        let var = xc.sqr()?.mean_keepdim(D::Minus1)?;
        xc.broadcast_div(&(var + 1e-5)?.sqrt()?)?.broadcast_mul(&self.w)?.broadcast_add(&self.b)
    }
}

fn layer_norm(d: usize, vb: VarBuilder) -> Result<LayerNorm> {
    Ok(LayerNorm { w: vb.get_with_hints(d, "weight", candle_nn::Init::Const(1.))?, b: vb.get_with_hints(d, "bias", candle_nn::Init::Const(0.))? })
}

/// Embedding tables start small (σ 0.02): with σ 1 a word seen a few times never moves away from its random
/// vector under Adam's small steps, and the tagger stalls at 75% (measured 15.10).
fn emb(n: usize, d: usize, vb: VarBuilder) -> Result<Embedding> {
    let w = vb.get_with_hints((n, d), "weight", candle_nn::Init::Randn { mean: 0., stdev: 0.02 })?;
    Ok(Embedding::new(w, d))
}

fn drop(x: &Tensor, p: f32, train: bool) -> Result<Tensor> {
    if train && p > 0. { ops::dropout(x, p) } else { Ok(x.clone()) }
}

impl Net {
    pub fn new(cfg: Cfg, n_words: usize, n_xpos: usize, n_upos: usize, n_rel: usize, vb: VarBuilder) -> Result<Net> {
        let d = cfg.d;
        let mut blocks = Vec::new();
        for l in 0..cfg.layers {
            let b = vb.pp(format!("block{l}"));
            blocks.push(Block {
                ln1: layer_norm(d, b.pp("ln1"))?,
                qkv: linear(d, 3 * d, b.pp("qkv"))?,
                out: linear(d, d, b.pp("out"))?,
                ln2: layer_norm(d, b.pp("ln2"))?,
                ff1: linear(d, cfg.ff, b.pp("ff1"))?,
                ff2: linear(cfg.ff, d, b.pp("ff2"))?,
            });
        }
        let init = candle_nn::Init::Randn { mean: 0., stdev: 0.02 };
        Ok(Net {
            cfg,
            word: emb(n_words, d, vb.pp("word"))?,
            pre: emb(AFFIX, d, vb.pp("pre"))?,
            suf: emb(AFFIX, d, vb.pp("suf"))?,
            shape: emb(9, d, vb.pp("shape"))?,
            pos: emb(cfg.max_len + 1, d, vb.pp("pos"))?,
            prag_tok: emb(16, d, vb.pp("prag_tok"))?,
            prag_arc: Embedding::new(vb.get_with_hints((8, 1), "prag_arc.weight", candle_nn::Init::Const(0.))?, 1),
            blocks,
            ln: layer_norm(d, vb.pp("ln"))?,
            arc_d: linear(d, cfg.arc, vb.pp("arc_d"))?,
            arc_h: linear(d, cfg.arc, vb.pp("arc_h"))?,
            arc_w: vb.get_with_hints((cfg.arc, cfg.arc), "arc_w", init)?,
            arc_u: vb.get_with_hints((cfg.arc, 1), "arc_u", init)?,
            rel_d: linear(d, cfg.rel, vb.pp("rel_d"))?,
            rel_h: linear(d, cfg.rel, vb.pp("rel_h"))?,
            rel_w: vb.get_with_hints((cfg.rel, n_rel * cfg.rel), "rel_w", init)?,
            rel_lin: linear(2 * cfg.rel, n_rel, vb.pp("rel_lin"))?,
            xpos: linear(d, n_xpos, vb.pp("xpos"))?,
            upos: linear(d, n_upos, vb.pp("upos"))?,
            n_rel,
        })
    }

    pub fn forward(&self, x: &Batch, dev: &Device, train: bool) -> Result<Out> {
        let (b, t, c) = (x.b, x.t, self.cfg);
        let inp = Tensor::from_slice(&x.inp, (b, t, 4), dev)?;
        let col = |k: usize| inp.narrow(2, k, 1).and_then(|z| z.squeeze(2)).and_then(|z| z.contiguous());
        let pos = Tensor::arange(0u32, t as u32, dev)?.unsqueeze(0)?.broadcast_as((b, t))?.contiguous()?;
        let mut h = (self.word.forward(&col(0)?)? + self.pre.forward(&col(1)?)?)?;
        h = (h + self.suf.forward(&col(2)?)?)?;
        h = (h + self.shape.forward(&col(3)?)?)?;
        h = (h + self.pos.forward(&pos)?)?;
        h = (h + self.prag_tok.forward(&Tensor::from_slice(&x.role_tok, (b, t), dev)?)?)?;
        h = drop(&h, c.dropout, train)?;
        // key mask: −1e9 on padding columns, shape (B,1,1,T)
        let keys = Tensor::from_slice(&x.keys, (b, 1, 1, t), dev)?;
        let kbias = ((keys - 1.0)? * 1e9)?;
        let (nh, dh) = (c.heads, c.d / c.heads);
        for bl in &self.blocks {
            let z = bl.ln1.forward(&h)?;
            let qkv = bl.qkv.forward(&z)?.reshape((b, t, 3, nh, dh))?;
            let q = qkv.narrow(2, 0, 1)?.squeeze(2)?.transpose(1, 2)?.contiguous()?;
            let k = qkv.narrow(2, 1, 1)?.squeeze(2)?.transpose(1, 2)?.contiguous()?;
            let v = qkv.narrow(2, 2, 1)?.squeeze(2)?.transpose(1, 2)?.contiguous()?;
            let att = (q.matmul(&k.t()?)? / (dh as f64).sqrt())?.broadcast_add(&kbias)?;
            // ops::softmax, not softmax_last_dim: the fused kernel passes no gradient (test `which_ops_backprop`)
            let att = drop(&ops::softmax(&att, D::Minus1)?, c.dropout, train)?;
            let y = att.matmul(&v)?.transpose(1, 2)?.contiguous()?.reshape((b, t, c.d))?;
            h = (h + drop(&bl.out.forward(&y)?, c.dropout, train)?)?;
            let z = bl.ln2.forward(&h)?;
            let y = bl.ff2.forward(&drop(&bl.ff1.forward(&z)?.gelu_erf()?, c.dropout, train)?)?;
            h = (h + drop(&y, c.dropout, train)?)?;
        }
        let h = self.ln.forward(&h)?;
        let dep = drop(&self.arc_d.forward(&h)?.gelu_erf()?, c.dropout, train)?;
        let hed = drop(&self.arc_h.forward(&h)?.gelu_erf()?, c.dropout, train)?;
        // S[b,i,j] = dep_i W hed_j + hed_j u
        let s = dep.broadcast_matmul(&self.arc_w)?.matmul(&hed.t()?)?;
        let bias = hed.broadcast_matmul(&self.arc_u)?.transpose(1, 2)?; // (B,1,T)
        let s = s.broadcast_add(&bias)?;
        let cbias = ((Tensor::from_slice(&x.keys, (b, 1, t), dev)? - 1.0)? * 1e9)?;
        // the reader's role links pull their arcs: one learned score per role (0 for "none")
        let pull = self.prag_arc.forward(&Tensor::from_slice(&x.role_arc, (b, t, t), dev)?)?.squeeze(3)?;
        let arcs = (s.broadcast_add(&cbias)? + pull)?;
        Ok(Out { arcs, h })
    }

    /// Relation scores (N, L) for words `rows` (flat b*T+i) with heads `heads` (flat b*T+h).
    pub fn rels(&self, out: &Out, rows: &Tensor, heads: &Tensor, train: bool) -> Result<Tensor> {
        let (b, t, d) = out.h.dims3()?;
        let flat = out.h.reshape((b * t, d))?;
        let dd = drop(&self.rel_d.forward(&flat.index_select(rows, 0)?)?.gelu_erf()?, self.cfg.dropout, train)?;
        let hh = drop(&self.rel_h.forward(&flat.index_select(heads, 0)?)?.gelu_erf()?, self.cfg.dropout, train)?;
        let n = dd.dim(0)?;
        let r = self.cfg.rel;
        let bil = dd.matmul(&self.rel_w)?.reshape((n, self.n_rel, r))?.broadcast_mul(&hh.unsqueeze(1)?)?.sum(D::Minus1)?;
        bil + self.rel_lin.forward(&Tensor::cat(&[&dd, &hh], 1)?)?
    }

    /// Tag scores for words `rows`: (XPOS, UPOS).
    pub fn tags(&self, out: &Out, rows: &Tensor) -> Result<(Tensor, Tensor)> {
        let (b, t, d) = out.h.dims3()?;
        let z = out.h.reshape((b * t, d))?.index_select(rows, 0)?;
        Ok((self.xpos.forward(&z)?, self.upos.forward(&z)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::DType;
    use candle_nn::VarMap;

    #[test]
    fn embeddings_get_gradients() {
        let dev = Device::Cpu;
        let vm = VarMap::new();
        let vb = VarBuilder::from_varmap(&vm, DType::F32, &dev);
        let e = candle_nn::embedding(10, 4, vb.pp("e")).unwrap();
        let ids = Tensor::new(&[[1u32, 2, 3]], &dev).unwrap();
        let col = ids.unsqueeze(2).unwrap().narrow(2, 0, 1).unwrap().squeeze(2).unwrap().contiguous().unwrap();
        let y = e.forward(&col).unwrap().sum_all().unwrap();
        let g = y.backward().unwrap();
        let names: Vec<String> = vm.data().lock().unwrap().keys().cloned().collect();
        let w = vm.data().lock().unwrap().get("e.weight").cloned().unwrap();
        assert!(g.get(w.as_tensor()).is_some(), "no gradient for the embedding; vars {names:?}");
        // through our layer norm the gradient reaches the embedding; negative control: candle's fused one
        let ln = layer_norm(4, vb.pp("ln")).unwrap();
        let y = ln.forward(&e.forward(&col).unwrap()).unwrap().sqr().unwrap().sum_all().unwrap();
        let g = y.backward().unwrap();
        let gw = g.get(w.as_tensor()).expect("gradient through our layer norm");
        assert!(gw.abs().unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap() > 0.);
        let fused = candle_nn::layer_norm(4, 1e-5, vb.pp("fused")).unwrap();
        let y = fused.forward(&e.forward(&col).unwrap()).unwrap().sqr().unwrap().sum_all().unwrap();
        let fused_reaches = y.backward().ok().and_then(|g| g.get(w.as_tensor()).map(|t| t.abs().unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap()));
        assert_eq!(fused_reaches, None, "fused layer norm learned to backprop: it may be used again");
    }
}

#[cfg(test)]
mod fused {
    use candle_core::{DType, Device, Tensor, D};
    use candle_nn::{ops, VarBuilder, VarMap};

    /// Which candle ops pass a gradient back. The fused `softmax_last_dim` (and the fused layer norm, see
    /// `embeddings_get_gradients`) silently return none — the model must not use them.
    #[test]
    fn which_ops_backprop() {
        let dev = Device::Cpu;
        let vm = VarMap::new();
        let vb = VarBuilder::from_varmap(&vm, DType::F32, &dev);
        let w = vb.get_with_hints((2, 3), "w", candle_nn::Init::Randn { mean: 0., stdev: 1. }).unwrap();
        let x = Tensor::new(&[[1f32, 2., 3.], [0., 1., -1.]], &dev).unwrap();
        let probe = |name: &str, y: candle_core::Result<Tensor>| {
            let y = y.unwrap().sqr().unwrap().sum_all().unwrap();
            let g = y.backward().ok().and_then(|g| g.get(&w).map(|t| t.abs().unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap()));
            println!("{name}: {g:?}");
            g.is_some_and(|v| v > 0.)
        };
        let z = (&x * &w).unwrap();
        assert!(!probe("softmax_last_dim", ops::softmax_last_dim(&z)), "fused softmax learned to backprop: it may be used again");
        for (name, y) in [("softmax", ops::softmax(&z, D::Minus1)), ("log_softmax", ops::log_softmax(&z, D::Minus1)), ("gelu_erf", z.gelu_erf()), ("dropout", ops::dropout(&z, 0.1))] {
            assert!(probe(name, y), "{name} passes no gradient");
        }
    }
}
