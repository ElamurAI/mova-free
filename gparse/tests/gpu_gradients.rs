//! CUDA backprop guard for the attention optimization. No trained model required.
use candle_core::{DType, Device};
use candle_nn::{VarBuilder, VarMap};
use gparse::{data, model::{Cfg, Net}};

#[test]
#[ignore = "requires CUDA"]
fn training_attention_keeps_gradients_and_inference_control_breaks_them() {
    let dev = Device::new_cuda(0).unwrap();
    dev.set_seed(42).unwrap();
    let vm = VarMap::new();
    let vb = VarBuilder::from_varmap(&vm, DType::F32, &dev);
    let sents = data::read("1\tCats\tcat\tNOUN\tNNS\t_\t2\tnsubj\t_\t_\n2\tsleep\tsleep\tVERB\tVBP\t_\t0\troot\t_\t_\n\n");
    let v = data::Vocs::build(&sents, 1);
    let x = data::batch(&sents.iter().collect::<Vec<_>>(), &v, 0., &mut data::Rng(1));
    let cfg = Cfg { d: 16, layers: 1, heads: 2, ff: 32, arc: 8, rel: 8, max_len: 16, dropout: 0., gelu: gparse::model::Gelu::Erf, ngram: 0 };
    let net = Net::new(cfg, v.words.len(), v.xpos.len(), v.upos.len(), v.rels.len(), vb).unwrap();
    let vars = vm.data().lock().unwrap().clone();
    let qkv = vars.get("block0.qkv.weight").unwrap();
    // The public training entry point always chooses differentiable attention.
    let out = net.forward(&x, &dev, true).unwrap();
    let gradients = out.arcs.sqr().unwrap().sum_all().unwrap().backward().unwrap();
    for name in ["word.weight", "block0.qkv.weight", "block0.ln1.weight", "arc_w"] {
        let g = gradients.get(vars.get(name).unwrap().as_tensor()).expect(name);
        let norm = g.abs().unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap();
        assert!(norm.is_finite() && norm > 0., "{name}: gradient norm={norm}");
    }
    let qkv_grad = gradients.get(qkv.as_tensor()).unwrap().to_vec2::<f32>().unwrap();
    for (name, rows) in ["query", "key", "value"].into_iter().zip(qkv_grad.chunks(cfg.d)) {
        let norm: f32 = rows.iter().flatten().map(|x| x.abs()).sum();
        assert!(norm.is_finite() && norm > 0., "{name}: gradient norm={norm}");
    }
    // Negative control: fusion cuts the query/key route. The value slice can still
    // learn via AV, so checking only whether *any* QKV gradient exists is insufficient.
    let out = net.forward_inference(&x, &dev, true).unwrap();
    let gradients = out.arcs.sqr().unwrap().sum_all().unwrap().backward().unwrap();
    let g = gradients.get(qkv.as_tensor()).unwrap();
    let rows = g.to_vec2::<f32>().unwrap();
    assert!(rows[..2 * cfg.d].iter().flatten().all(|&x| x == 0.));
    assert!(rows[2 * cfg.d..].iter().flatten().any(|&x| x != 0.));
}
