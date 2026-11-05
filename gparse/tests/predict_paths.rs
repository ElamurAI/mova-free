//! Explicit GPU regression and throughput probe using external model and corpus files.
//! GP_TEST_MODEL=<dir> GP_TEST_CORPUS=<train.conllu> cargo test --release -p gparse --test predict_paths -- --ignored --nocapture
use std::time::{Duration, Instant};

#[test]
#[ignore = "requires GP_TEST_MODEL, GP_TEST_CORPUS and CUDA"]
fn unscored_matches_scored_and_repeated_batches() {
    let model = gparse::load(&std::env::var("GP_TEST_MODEL").expect("GP_TEST_MODEL")).unwrap();
    assert!(model.dev.is_cuda(), "this probe must run on the GPU");
    let corpus = std::fs::read_to_string(std::env::var("GP_TEST_CORPUS").expect("GP_TEST_CORPUS")).unwrap();
    let limit: usize = std::env::var("GP_TEST_SENTENCES").unwrap_or_else(|_| "1024".into()).parse().unwrap();
    let sents: Vec<_> = gparse::data::read(&corpus).into_iter()
        .filter(|s| !s.words.is_empty() && s.words.len() < model.net.cfg.max_len).take(limit).collect();
    assert!(!sents.is_empty(), "the corpus must contain fitting sentences");
    let words: usize = sents.iter().map(|s| s.words.len()).sum();
    let fast = || gparse::predict(&model.net, &model.vocs, &model.dev, &sents).unwrap();
    let scored = || gparse::predict_scored(&model.net, &model.vocs, &model.dev, &sents).unwrap().0;
    let reference = scored();
    assert_eq!(fast(), reference);
    let (mut fast_times, mut scored_times) = (Vec::new(), Vec::new());
    for round in 0..6 {
        // Alternate the order to reduce warmup and clock bias.
        for use_fast in if round % 2 == 0 { [true, false] } else { [false, true] } {
            let start = Instant::now();
            let output = if use_fast { fast() } else { scored() };
            let elapsed = start.elapsed();
            assert_eq!(output, reference, "tree or tag changed on round {round}");
            if use_fast { fast_times.push(elapsed); } else { scored_times.push(elapsed); }
        }
    }
    let median = |v: &mut Vec<Duration>| {
        v.sort();
        (v[2].as_secs_f64() + v[3].as_secs_f64()) / 2.
    };
    let (fast_s, scored_s) = (median(&mut fast_times), median(&mut scored_times));
    println!("sentences={} words={words} unscored_median_s={fast_s:.6} scored_median_s={scored_s:.6} speedup={:.3} unscored_words_per_s={:.0}",
        sents.len(), scored_s / fast_s, words as f64 / fast_s);
    // The public API also preserves empty batches and the existing overlength fallback.
    let mut long = sents[0].clone();
    long.words = std::iter::repeat_n(long.words[0].clone(), model.net.cfg.max_len).collect();
    let edge = vec![sents[0].clone(), long];
    assert_eq!(
        gparse::predict(&model.net, &model.vocs, &model.dev, &edge).unwrap(),
        gparse::predict_scored(&model.net, &model.vocs, &model.dev, &edge).unwrap().0,
    );
    assert!(gparse::predict(&model.net, &model.vocs, &model.dev, &[]).unwrap().is_empty());
}

#[test]
fn synthetic_cpu_prediction_preserves_slots_and_fallback() {
    use candle_core::{DType, Device};
    use candle_nn::{VarBuilder, VarMap};
    use gparse::{
        data,
        model::{Cfg, Net},
    };
    let dev = Device::Cpu;
    let mut s = data::read("1\tCat\tcat\tNOUN\tNN\t_\t0\troot\t_\t_\n");
    let v = data::Vocs::build(&s, 1);
    let cfg = Cfg {
        d: 8,
        layers: 1,
        heads: 2,
        ff: 16,
        arc: 4,
        rel: 4,
        max_len: 4,
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
    let mut long = s[0].clone();
    long.words = vec![long.words[0].clone(); cfg.max_len];
    let expected: Vec<_> = long
        .words
        .iter()
        .map(|w| (w.head, w.rel.clone(), w.xpos.clone(), w.upos.clone()))
        .collect();
    s.insert(
        0,
        data::Sent {
            comments: vec![],
            words: vec![],
            roles: vec![],
        },
    );
    s.push(long);
    let a = gparse::predict(&net, &v, &dev, &s).unwrap();
    let (b, scores) = gparse::predict_scored(&net, &v, &dev, &s).unwrap();
    assert_eq!(a, b);
    assert!(a[0].is_empty());
    assert_eq!(a[1][0].0, 0);
    assert_eq!(a[1][0].1, "root");
    assert_eq!(a[2], expected);
    assert_eq!((scores[0], scores[2]), (0., 0.));
    assert!(scores[1].is_finite());
}
