//! Bounded training-step experiment, not a release accuracy evaluation.
//! GP_TEST_MODEL=<dir> GP_TEST_CORPUS=<train.conllu> cargo test --release -p gparse
//! --test training_speed -- --ignored --nocapture --test-threads=1
use candle_core::{DType, Device, Tensor};
use candle_nn::{loss, AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use gparse::{data, model::{Cfg, Net}};
use std::{path::Path, time::Instant};

fn step(net: &Net, vm: &VarMap, x: &data::Batch, dev: &Device, opt: &mut AdamW, inspect: bool) -> f32 {
    let out = net.forward(x, dev, true).unwrap();
    let n = x.rows.len();
    let rows = Tensor::from_slice(&x.rows, n, dev).unwrap();
    let arcs = out.arcs.reshape((x.b * x.t, x.t)).unwrap().index_select(&rows, 0).unwrap();
    let heads = Tensor::from_slice(&x.heads, n, dev).unwrap();
    let arc_loss = loss::cross_entropy(&arcs, &heads).unwrap();
    let hflat: Vec<u32> = x.rows.iter().zip(&x.heads).map(|(&r,&h)| r-r % x.t as u32+h).collect();
    let ht = Tensor::from_slice(&hflat, n, dev).unwrap();
    let rel_loss = loss::cross_entropy(&net.rels(&out,&rows,&ht,true).unwrap(),
        &Tensor::from_slice(&x.rels,n,dev).unwrap()).unwrap();
    let (xp,up) = net.tags(&out,&rows).unwrap();
    let xp_loss = loss::cross_entropy(&xp,&Tensor::from_slice(&x.xpos,n,dev).unwrap()).unwrap();
    let up_loss = loss::cross_entropy(&up,&Tensor::from_slice(&x.upos,n,dev).unwrap()).unwrap();
    let total = ((arc_loss+rel_loss).unwrap()+((xp_loss+up_loss).unwrap()*0.5).unwrap()).unwrap();
    let gradients = total.backward().unwrap();
    if inspect {
        let vars = vm.data().lock().unwrap();
        for name in ["word.weight", "block0.qkv.weight", "arc_w", "rel_w"] {
            let g = gradients.get(vars[name].as_tensor()).unwrap();
            let norm = g.abs().unwrap().sum_all().unwrap().to_scalar::<f32>().unwrap();
            assert!(norm.is_finite() && norm > 0., "{name}: {norm}");
        }
    }
    opt.step(&gradients).unwrap();
    total.to_scalar::<f32>().unwrap()
}

#[test]
#[ignore = "requires GP_TEST_MODEL, GP_TEST_CORPUS and CUDA; serial environment changes"]
fn measured_optimizer_steps() {
    let compare_gelu = std::env::var("GP_TRAIN_TEST_COMPARE").as_deref() == Ok("gelu");
    println!("comparison={}",if compare_gelu {"erf-vs-tanh-gelu"} else {"reference-vs-fused-softmax"});
    let dev = Device::new_cuda(0).unwrap();
    let dir = std::env::var("GP_TEST_MODEL").expect("GP_TEST_MODEL");
    let read = |f: &str| std::fs::read_to_string(Path::new(&dir).join(f)).unwrap();
    let cfg = Cfg::load(&read("cfg.txt")).unwrap();
    let v = data::Vocs { words:data::Voc::load(&read("words.txt")), xpos:data::Voc::load(&read("xpos.txt")),
        upos:data::Voc::load(&read("upos.txt")), rels:data::Voc::load(&read("rels.txt")) };
    let src = std::fs::read_to_string(std::env::var("GP_TEST_CORPUS").expect("GP_TEST_CORPUS")).unwrap();
    let count: usize = std::env::var("GP_TRAIN_TEST_SENTENCES").unwrap_or_else(|_| "32".into()).parse().unwrap();
    assert!(count > 0 && count <= 128, "this bounded probe supports 1..=128 sentences");
    let sentences: Vec<_> = data::read(&src).into_iter()
        .filter(|s| (24..=40).contains(&s.words.len()) && s.words.len() < cfg.max_len).take(count).collect();
    assert_eq!(sentences.len(),count);
    if let Ok(path) = std::env::var("GP_TRAIN_TEST_CORPUS_OUT") {
        std::fs::write(path,data::write(&sentences)).unwrap();
    }
    let batch = data::batch(&sentences.iter().collect::<Vec<_>>(),&v,0.,&mut data::Rng(1));
    let words = batch.rows.len();
    let run = |fused: bool| {
        std::env::set_var("GP_TRAIN_FUSED_SOFTMAX",if fused || compare_gelu {"1"} else {"0"});
        if compare_gelu { std::env::set_var("GP_GELU",if fused {"tanh"} else {"erf"}); }
        dev.set_seed(42).unwrap();
        let vm = VarMap::new();
        let vb = VarBuilder::from_varmap(&vm,DType::F32,&dev);
        let net = Net::new(cfg,v.words.len(),v.xpos.len(),v.upos.len(),v.rels.len(),vb).unwrap();
        gparse::load_vars(&vm,&Path::new(&dir).join("model.safetensors"),&dev).unwrap();
        let mut opt = AdamW::new(vm.all_vars(),ParamsAdamW {lr:1e-4,beta1:0.9,beta2:0.98,eps:1e-8,weight_decay:0.01}).unwrap();
        let mut losses = vec![];
        for k in 0..3 { losses.push(step(&net,&vm,&batch,&dev,&mut opt,k==0)); }
        dev.synchronize().unwrap();
        let start = Instant::now();
        for _ in 0..8 { losses.push(step(&net,&vm,&batch,&dev,&mut opt,false)); }
        dev.synchronize().unwrap();
        (start.elapsed().as_secs_f64()/8.,losses)
    };
    let (mut reference_times,mut fused_times) = (vec![],vec![]);
    let (mut reference_losses,mut fused_losses): (Option<Vec<f32>>,Option<Vec<f32>>) = (None,None);
    let mut max_repeat_delta = 0f32;
    for round in 0..4 {
        for fused in if round % 2 == 0 { [false,true] } else { [true,false] } {
            let (seconds,losses) = run(fused);
            assert!(losses.iter().all(|x|x.is_finite()));
            let saved = if fused {&mut fused_losses} else {&mut reference_losses};
            if let Some(previous) = saved {
                let delta = losses.iter().zip(previous.iter()).map(|(x,y)|(x-y).abs()).fold(0f32,f32::max);
                max_repeat_delta = max_repeat_delta.max(delta);
                assert!(delta < 1e-5,"repeat drift exceeds tolerance: {delta}");
            }
            else { *saved = Some(losses.clone()); }
            println!("round={round} fused={fused} step_seconds={seconds:.6} losses={losses:?}");
            if fused {fused_times.push(seconds)} else {reference_times.push(seconds)}
        }
    }
    let a = reference_losses.unwrap();
    let b = fused_losses.unwrap();
    let max_loss_delta = a.iter().zip(&b).map(|(x,y)|(x-y).abs()).fold(0f32,f32::max);
    let scale = a.iter().copied().fold(1f32,f32::max);
    assert!(max_loss_delta < 0.005*scale,"training trajectories diverged: {max_loss_delta}");
    let median = |v: &mut Vec<f64>| {v.sort_by(f64::total_cmp);(v[1]+v[2])/2.};
    let r = median(&mut reference_times); let f = median(&mut fused_times);
    println!("sentences={count} words={words} padded_positions={} dropout={} reference_step_s={r:.6} fused_step_s={f:.6} speedup={:.4} max_loss_delta={max_loss_delta} max_repeat_delta={max_repeat_delta}",batch.b*batch.t,cfg.dropout,r/f);
    std::env::remove_var("GP_TRAIN_FUSED_SOFTMAX");
    if compare_gelu { std::env::remove_var("GP_GELU"); }
}

#[test]
#[ignore = "requires CUDA; synthetic single optimizer step"]
fn synthetic_single_optimizer_step() {
    let dev = Device::new_cuda(0).unwrap();
    dev.set_seed(42).unwrap();
    let s = data::read("1\tCats\tcat\tNOUN\tNNS\t_\t2\tnsubj\t_\t_\n2\tsleep\tsleep\tVERB\tVBP\t_\t0\troot\t_\t_\n");
    let v = data::Vocs::build(&s, 1);
    let batch = data::batch(&s.iter().collect::<Vec<_>>(), &v, 0., &mut data::Rng(1));
    let cfg = Cfg {
        d: 16,
        layers: 1,
        heads: 2,
        ff: 32,
        arc: 8,
        rel: 8,
        max_len: 16,
        dropout: 0.,
        gelu: gparse::model::Gelu::Erf,
        ngram: 0,
    };
    let vm = VarMap::new();
    let net = Net::new(
        cfg,
        v.words.len(),
        v.xpos.len(),
        v.upos.len(),
        v.rels.len(),
        VarBuilder::from_varmap(&vm, DType::F32, &dev),
    )
    .unwrap();
    let var = vm.data().lock().unwrap()["arc_w"].clone();
    let before = var.flatten_all().unwrap().to_vec1::<f32>().unwrap();
    let mut opt = AdamW::new(vm.all_vars(), ParamsAdamW::default()).unwrap();
    let loss = step(&net, &vm, &batch, &dev, &mut opt, true);
    assert!(loss.is_finite() && loss > 0.);
    let after = var.flatten_all().unwrap().to_vec1::<f32>().unwrap();
    assert!(before.iter().zip(&after).any(|(a, b)| a != b));
    assert!(after.iter().all(|x| x.is_finite()));
}
