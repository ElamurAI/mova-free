//! CUDA forward/backward checks against autodiff and independent f64 finite differences.
use candle_core::{DType, Device, Tensor, Var, D};
use candle_nn::ops;

fn max_abs(a: &Tensor, b: &Tensor) -> f32 {
    (a - b).unwrap().abs().unwrap().flatten_all().unwrap().max(0).unwrap().to_scalar().unwrap()
}

#[test]
#[ignore = "requires CUDA"]
fn fused_softmax_matches_reference_gradients_and_finite_differences() {
    let dev = Device::new_cuda(0).unwrap();
    for width in [1usize, 3, 17, 33, 257] {
        let values: Vec<f32> = (0..3 * width).map(|i| ((i * 17 % 43) as f32 - 21.) / 7.).collect();
        let weights: Vec<f32> = (0..3 * width).map(|i| ((i * 7 % 31) as f32 - 15.) / 11.).collect();
        let x = Var::from_vec(values.clone(), (3, width), &dev).unwrap();
        let w = Tensor::from_vec(weights.clone(), (3, width), &dev).unwrap();
        let fast = gparse::softmax::last_dim(&x).unwrap();
        let reference = ops::softmax(&x, D::Minus1).unwrap();
        assert!(max_abs(&fast, &reference) < 2e-6, "forward width={width}");
        let gf = (&fast * &w).unwrap().sum_all().unwrap().backward().unwrap();
        let gr = (&reference * &w).unwrap().sum_all().unwrap().backward().unwrap();
        let grad = gf.get(x.as_tensor()).unwrap();
        assert!(max_abs(grad, gr.get(x.as_tensor()).unwrap()) < 2e-6, "backward width={width}");
        let actual = grad.flatten_all().unwrap().to_vec1::<f32>().unwrap();
        let objective = |values: &[f64]| -> f64 {
            values.chunks(width).zip(weights.chunks(width)).map(|(row, w)| {
                let max = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let exp: Vec<_> = row.iter().map(|x| (x - max).exp()).collect();
                exp.iter().zip(w).map(|(p,w)| p * f64::from(*w)).sum::<f64>() / exp.iter().sum::<f64>()
            }).sum()
        };
        let base: Vec<_> = values.iter().map(|&x| f64::from(x)).collect();
        for i in [0, width - 1, width, 3 * width - 1] {
            let (mut lo, mut hi) = (base.clone(), base.clone());
            lo[i] -= 1e-4;
            hi[i] += 1e-4;
            let finite = (objective(&hi) - objective(&lo)) / 2e-4;
            assert!((f64::from(actual[i]) - finite).abs() < 2e-6,
                "width={width}, i={i}, gradient={}, finite={finite}", actual[i]);
        }
        // Contiguous slices with nonzero offsets and noncontiguous transposes.
        for view in [x.narrow(0, 1, 2).unwrap(), x.t().unwrap()] {
            let a = gparse::softmax::last_dim(&view).unwrap();
            let b = ops::softmax(&view, D::Minus1).unwrap();
            assert!(max_abs(&a, &b) < 2e-6);
            let ga = a.sqr().unwrap().sum_all().unwrap().backward().unwrap();
            let gb = b.sqr().unwrap().sum_all().unwrap().backward().unwrap();
            assert!(max_abs(ga.get(x.as_tensor()).unwrap(), gb.get(x.as_tensor()).unwrap()) < 2e-6);
        }
    }
    // Stable normalization and masked columns; a naive exp(1000) would overflow.
    let x = Var::from_vec(vec![1000f32, 999., -1e9, -1000., -1001., -1e9], (2,3), &dev).unwrap();
    let p = gparse::softmax::last_dim(&x).unwrap();
    let flat = p.flatten_all().unwrap().to_vec1::<f32>().unwrap();
    assert!(flat.iter().all(|x| x.is_finite()));
    assert_eq!((flat[2], flat[5]), (0., 0.));
    assert!(max_abs(&p, &ops::softmax(&x, D::Minus1).unwrap()) < 2e-6);
    // The upstream no-backward entry point is a negative control.
    let no_bwd = ops::softmax_last_dim(&x).unwrap().sqr().unwrap().sum_all().unwrap();
    assert!(no_bwd.backward().unwrap().get(x.as_tensor()).is_none());
}

#[test]
fn cpu_f64_uses_reference() {
    let x = Tensor::new(&[[1f64,2.,3.]], &Device::Cpu).unwrap();
    assert_eq!(x.dtype(), DType::F64);
    assert_eq!(gparse::softmax::last_dim(&x).unwrap().to_vec2::<f64>().unwrap(),
        ops::softmax(&x, D::Minus1).unwrap().to_vec2::<f64>().unwrap());
}
