//! GELU approximation: measured errors, finite differences and parser throughput.
use candle_core::{Device, Var};
use gparse::model::{Cfg, Gelu};
use std::time::Instant;

#[test]
fn activation_configuration_roundtrips() {
    assert_eq!(Cfg::load("d=256").unwrap().gelu, Gelu::Erf);
    assert!(Cfg::load("gelu=typo").is_err());
    let mut cfg = Cfg::DEFAULT;
    cfg.gelu = Gelu::Tanh;
    assert_eq!(Cfg::load(&cfg.save()).unwrap().gelu, Gelu::Tanh);
}

#[test]
#[ignore = "requires CUDA"]
fn cuda_values_and_derivatives() {
    let dev = Device::new_cuda(0).unwrap();
    let values: Vec<f32> = (0..=16000).map(|i| i as f32 * 0.001 - 8.).collect();
    let x = Var::from_vec(values.clone(), values.len(), &dev).unwrap();
    let erf = Gelu::Erf.forward(&x).unwrap();
    let tanh = Gelu::Tanh.forward(&x).unwrap();
    let a = erf.to_vec1::<f32>().unwrap();
    let b = tanh.to_vec1::<f32>().unwrap();
    let ga = erf.sum_all().unwrap().backward().unwrap();
    let gb = tanh.sum_all().unwrap().backward().unwrap();
    let da = ga.get(x.as_tensor()).unwrap().to_vec1::<f32>().unwrap();
    let db = gb.get(x.as_tensor()).unwrap().to_vec1::<f32>().unwrap();
    let max = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(a,b)|(a-b).abs()).fold(0f32,f32::max);
    let (value_error, derivative_error) = (max(&a,&b),max(&da,&db));
    assert!(a.iter().chain(&b).chain(&da).chain(&db).all(|x|x.is_finite()));
    assert!(value_error < 0.0005 && derivative_error < 0.001);
    let approx = |x: f64| 0.5*x*(1.+((2./std::f64::consts::PI).sqrt()*(x+0.044715*x*x*x)).tanh());
    let mut finite_error = 0f64;
    for (&x,&grad) in values.iter().zip(&db) {
        let x=f64::from(x);
        let finite=(approx(x+1e-4)-approx(x-1e-4))/2e-4;
        finite_error=finite_error.max((finite-f64::from(grad)).abs());
    }
    assert!(finite_error < 1e-5, "bad approximate derivative: {finite_error}");
    println!("grid=[-8,8] spacing=0.001 samples={} value_max_abs={value_error} derivative_max_abs={derivative_error} finite_difference_max_abs={finite_error}",values.len());
}

#[test]
#[ignore = "requires GP_TEST_MODEL, GP_TEST_CORPUS and CUDA"]
fn parser_agreement_and_throughput() {
    let mut m = gparse::load(&std::env::var("GP_TEST_MODEL").unwrap()).unwrap();
    assert!(m.dev.is_cuda());
    let src=std::fs::read_to_string(std::env::var("GP_TEST_CORPUS").unwrap()).unwrap();
    let sents:Vec<_>=gparse::data::read(&src).into_iter().filter(|s|!s.words.is_empty()&&s.words.len()<m.net.cfg.max_len).take(2048).collect();
    assert_eq!(sents.len(),2048);
    m.net.cfg.gelu=Gelu::Erf;
    let reference=gparse::predict(&m.net,&m.vocs,&m.dev,&sents).unwrap();
    m.net.cfg.gelu=Gelu::Tanh;
    let candidate=gparse::predict(&m.net,&m.vocs,&m.dev,&sents).unwrap();
    let (mut words,mut changed,mut changed_heads,mut changed_relations,mut changed_tags)=(0,0,0,0,0);
    let (mut reference_las,mut candidate_las)=(0,0);
    for ((s,a),b) in sents.iter().zip(&reference).zip(&candidate) {
        assert_eq!(a.len(),b.len()); assert_eq!(a.len(),s.words.len());
        for ((gold,a),b) in s.words.iter().zip(a).zip(b) {
            words+=1; changed+=usize::from(a!=b);
            changed_heads+=usize::from(a.0!=b.0);
            changed_relations+=usize::from(a.1!=b.1);
            changed_tags+=usize::from(a.2!=b.2||a.3!=b.3);
            reference_las+=usize::from(a.0==gold.head&&a.1==gold.rel);
            candidate_las+=usize::from(b.0==gold.head&&b.1==gold.rel);
        }
    }
    let (mut rtimes,mut ctimes)=(vec![],vec![]);
    for round in 0..8 {
        for fast in if round%2==0 {[false,true]} else {[true,false]} {
            m.net.cfg.gelu=if fast {Gelu::Tanh} else {Gelu::Erf};
            m.dev.synchronize().unwrap();
            let t=Instant::now();
            let result=gparse::predict(&m.net,&m.vocs,&m.dev,&sents).unwrap();
            m.dev.synchronize().unwrap();
            let elapsed=t.elapsed().as_secs_f64();
            assert_eq!(result,if fast {&candidate} else {&reference}.clone());
            if fast {ctimes.push(elapsed)} else {rtimes.push(elapsed)}
        }
    }
    let median=|v:&[f64]|{let mut s=v.to_vec();s.sort_by(f64::total_cmp);(s[3]+s[4])/2.};
    let r=median(&rtimes);let c=median(&ctimes);
    println!("words={words} changed_tokens={changed} changed_heads={changed_heads} changed_relations={changed_relations} changed_tags={changed_tags} reference_train_las_count={reference_las} tanh_train_las_count={candidate_las} reference_median_s={r:.6} tanh_median_s={c:.6} speedup={:.4}",r/c);
    println!("reference_seconds={rtimes:?}\ntanh_seconds={ctimes:?}");
}
