//! v2: Monte Carlo gate for p-values. Under H0 a correct p is uniform on [0, 1] (exactly for exact tests,
//! approximately for Welch and χ²), so the fraction of p ≤ α over K simulations must be α ± 4.5·√(α(1 − α)/K) (+ a margin for
//! the approximation). The seed is fixed — the run is reproducible. Negative control: `fault = "p:<test>"` (wrong p) → red.

use crate::builtins::stats::{self, Tail};
use crate::rng::Rng;

pub struct Row {
    pub test: &'static str,
    pub alpha: f64,
    pub frac: f64,
    pub tol: f64,
    pub ok: bool,
}

pub const ALPHAS: [f64; 4] = [0.01, 0.05, 0.1, 0.5];

pub fn run(k: usize, seed: u64, fault: Option<&str>) -> Vec<Row> {
    let bad = |name: &str| fault.and_then(|f| f.strip_prefix("p:")) == Some(name);
    let mut rng = Rng::new(seed);
    let mut out = Vec::new();
    let mut check = |test: &'static str, approx: f64, ps: &[f64]| {
        let kk = ps.len() as f64;
        for &a in &ALPHAS {
            let frac = ps.iter().filter(|&&p| p <= a).count() as f64 / kk;
            let tol = 4.5 * (a * (1.0 - a) / kk).sqrt() + approx;
            out.push(Row { test, alpha: a, frac, tol, ok: (frac - a).abs() <= tol });
        }
    };
    let norm = |n: usize, s: f64, rng: &mut Rng| -> Vec<f64> { (0..n).map(|_| s * rng.normal()).collect() };

    let ps: Vec<f64> = (0..k).map(|_| stats::t_one(&norm(5, 1.0, &mut rng), 0.0, Tail::Both, bad("ttest")).p).collect();
    check("ttest, n = 5", 0.0, &ps);
    let ps: Vec<f64> = (0..k).map(|_| stats::t_one(&norm(6, 2.0, &mut rng), 0.0, Tail::Right, bad("ttest")).p).collect();
    check("ttest right-tailed, n = 6", 0.0, &ps);
    let ps: Vec<f64> = (0..k)
        .map(|_| {
            let (x, y) = (norm(4, 1.0, &mut rng), norm(7, 1.0, &mut rng));
            stats::t_two(&x, &y, true, Tail::Both, bad("ttest2")).p
        })
        .collect();
    check("ttest2 equal variances, 4 and 7", 0.0, &ps);
    let ps: Vec<f64> = (0..k)
        .map(|_| {
            let (x, y) = (norm(6, 1.0, &mut rng), norm(9, 2.0, &mut rng));
            stats::t_two(&x, &y, false, Tail::Both, bad("ttest2")).p
        })
        .collect();
    check("ttest2 Welch, 6 and 9, σ 1 and 2", 0.004, &ps);
    let ps: Vec<f64> = (0..k)
        .map(|_| {
            let g: Vec<Vec<f64>> = (0..3).map(|_| norm(4, 1.0, &mut rng)).collect();
            stats::anova_one(&g, bad("anova1")).p
        })
        .collect();
    check("anova1, 3 groups of 4", 0.0, &ps);
    let ps: Vec<f64> = (0..k)
        .map(|_| {
            let (x, y) = (norm(6, 1.0, &mut rng), norm(6, 1.0, &mut rng));
            let (mx, my) = (x.iter().sum::<f64>() / 6.0, y.iter().sum::<f64>() / 6.0);
            let sxy: f64 = x.iter().zip(&y).map(|(a, b)| (a - mx) * (b - my)).sum();
            let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
            let syy: f64 = y.iter().map(|b| (b - my) * (b - my)).sum();
            stats::corr_p(sxy / (sxx * syy).sqrt(), 6.0, bad("corr")).1
        })
        .collect();
    check("corr, n = 6", 0.0, &ps);
    let xs: Vec<f64> = (1..=7).map(|v| v as f64).collect();
    let ps: Vec<f64> = (0..k)
        .map(|_| {
            let y = norm(7, 1.0, &mut rng);
            stats::ols(&[vec![1.0; 7], xs.clone()], &y, true, bad("fitlm")).map_or(f64::NAN, |r| r.p[1])
        })
        .collect();
    check("fitlm, slope p, n = 7", 0.0, &ps);
    let ps: Vec<f64> = (0..k)
        .filter_map(|_| {
            let mut t = vec![vec![0.0; 3]; 2];
            for _ in 0..300 {
                let r = usize::from(rng.uniform() >= 0.3);
                let u = rng.uniform();
                let c = if u < 0.2 { 0 } else if u < 0.5 { 1 } else { 2 };
                t[r][c] += 1.0;
            }
            stats::chi2_ind(&t, bad("chi2test")).ok().map(|r| r.p)
        })
        .collect();
    check("chi2test 2x3, n = 300", 0.004, &ps);
    out
}
