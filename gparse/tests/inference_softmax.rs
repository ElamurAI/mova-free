//! Explicit CUDA experiment; only a training corpus is used, never a release test set.
//! GP_TEST_MODEL=<dir> GP_TEST_CORPUS=<train.conllu> cargo test --release -p gparse
//! --test inference_softmax -- --ignored --nocapture --test-threads=1
use std::time::Instant;

fn compare(a: &[Vec<(usize, String, String, String)>], b: &[Vec<(usize, String, String, String)>]) -> (usize, usize) {
    assert_eq!(a.len(), b.len());
    let (mut sentences, mut tokens) = (0, 0);
    for (a, b) in a.iter().zip(b) {
        assert_eq!(a.len(), b.len());
        sentences += usize::from(a != b);
        tokens += a.iter().zip(b).filter(|(a, b)| a != b).count();
    }
    (sentences, tokens)
}

#[test]
#[ignore = "requires GP_TEST_MODEL, GP_TEST_CORPUS and CUDA; serial environment changes"]
fn fused_softmax_agreement_repeatability_and_speed() {
    let m = gparse::load(&std::env::var("GP_TEST_MODEL").expect("GP_TEST_MODEL")).unwrap();
    assert!(m.dev.is_cuda());
    let corpus = std::fs::read_to_string(std::env::var("GP_TEST_CORPUS").expect("GP_TEST_CORPUS")).unwrap();
    let limit = std::env::var("GP_TEST_SENTENCES").unwrap_or_else(|_| "2048".into()).parse::<usize>().unwrap();
    let sents: Vec<_> = gparse::data::read(&corpus).into_iter()
        .filter(|s| !s.words.is_empty() && s.words.len() < m.net.cfg.max_len).take(limit).collect();
    assert!(!sents.is_empty());
    let words: usize = sents.iter().map(|s| s.words.len()).sum();
    let predict = |fused: bool| {
        std::env::set_var("GP_FUSED_SOFTMAX", if fused { "1" } else { "0" });
        gparse::predict(&m.net, &m.vocs, &m.dev, &sents).unwrap()
    };
    // Warm both implementations; include device completion in every timing interval.
    let reference = predict(false);
    let candidate = predict(true);
    let delta = compare(&reference, &candidate);
    if let Ok(prefix) = std::env::var("GP_TEST_OUTPUT") {
        std::fs::write(format!("{prefix}-reference.txt"), format!("{reference:?}")).unwrap();
        std::fs::write(format!("{prefix}-fused.txt"), format!("{candidate:?}")).unwrap();
    }
    // Negative control: the agreement counter must detect an altered prediction.
    let mut wrong = reference.clone();
    wrong[0][0].1.push_str("-control");
    assert_eq!(compare(&reference, &wrong), (1, 1));
    let (mut reference_times, mut fused_times) = (vec![], vec![]);
    for round in 0..8 {
        for fused in if round % 2 == 0 { [false, true] } else { [true, false] } {
            m.dev.synchronize().unwrap();
            let t = Instant::now();
            let result = predict(fused);
            m.dev.synchronize().unwrap();
            let elapsed = t.elapsed().as_secs_f64();
            assert_eq!(result, if fused { &candidate } else { &reference }.clone(),
                "nondeterministic output, round={round}, fused={fused}");
            if fused { fused_times.push(elapsed); } else { reference_times.push(elapsed); }
        }
    }
    let median = |times: &[f64]| {
        let mut sorted = times.to_vec();
        sorted.sort_by(f64::total_cmp);
        (sorted[3] + sorted[4]) / 2.
    };
    let (r, f) = (median(&reference_times), median(&fused_times));
    println!("sentences={} words={words} changed_sentences={} changed_tokens={} reference_median_s={r:.6} fused_median_s={f:.6} speedup={:.4}",
        sents.len(), delta.0, delta.1, r / f);
    println!("reference_seconds={reference_times:?}\nfused_seconds={fused_times:?}");
    // Compare attention implementations at the model boundary, including padded rows.
    let refs: Vec<_> = sents.iter().take(8).collect();
    let x = gparse::data::batch(&refs, &m.vocs, 0., &mut gparse::data::Rng(1));
    let a = m.net.forward(&x, &m.dev, false).unwrap();
    let b = m.net.forward_inference(&x, &m.dev, true).unwrap();
    let max_delta = |a: &candle_core::Tensor, b: &candle_core::Tensor| {
        (a - b).unwrap().abs().unwrap().flatten_all().unwrap()
            .max(0).unwrap().to_scalar::<f32>().unwrap()
    };
    let h_delta = max_delta(&a.h, &b.h);
    let arc_delta = max_delta(&a.arcs, &b.arcs);
    assert!(h_delta.is_finite() && h_delta < 0.01);
    assert!(arc_delta.is_finite() && arc_delta < 0.01);
    println!("encoder_max_abs_delta={h_delta} arcs_max_abs_delta={arc_delta}");
    // Scored and ordinary inference must still return exactly the same discrete predictions.
    let scored = gparse::predict_scored(&m.net, &m.vocs, &m.dev, &sents).unwrap();
    assert_eq!(scored.0, candidate);
    assert!(scored.1.iter().all(|s| s.is_finite()));
    std::env::remove_var("GP_FUSED_SOFTMAX");
}

#[test]
fn compare_counts_tuple_changes_and_shape_errors() {
    let a = vec![vec![(0, "root".into(), "NN".into(), "NOUN".into())], vec![]];
    assert_eq!(compare(&a, &a), (0, 0));
    let mut b = a.clone();
    b[0][0].2 = "VB".into();
    assert_eq!(compare(&a, &b), (1, 1));
    assert!(std::panic::catch_unwind(|| compare(&a, &[])).is_err());
    b[0].clear();
    assert!(std::panic::catch_unwind(|| compare(&a, &b)).is_err());
}
