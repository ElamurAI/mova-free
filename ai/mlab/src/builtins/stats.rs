//! v2: statistics — descriptive, distributions (statrs, MIT), tests with p-values, OLS regression.
//! Every test (ttest, ttest2, chi2test, anova1, corr, fitlm) prints a report by default: hypotheses,
//! assumptions (what is checked from the data and what is not), computation steps and conclusion; `'Display','off'` — silent.
//! Test cores are pure functions (the Monte Carlo gate, `crate::mcgate`, runs the same ones).
//! Negative control `fault = "p:<test>"` — wrong p (t → normal, F → χ²/df1, χ² → df + 1).

use super::table::{opt, opts};
use super::{Registry, arg, is_str, mat_arg, one, str_of};
use crate::interp::{Interp, MError};
use crate::table as tb;
use crate::value::{Class, ColKind, Mat, StrArr, Table, Value};
use statrs::distribution::{Binomial, ChiSquared, Continuous, ContinuousCDF, Discrete, DiscreteCDF, FisherSnedecor, Normal, Poisson, StudentsT};
use std::rc::Rc;

pub fn register(r: &mut Registry) {
    r.register("mode", mode);
    r.register("quantile", |it, a, n| quantile(it, a, n, 1.0, "quantile"));
    r.register("prctile", |it, a, n| quantile(it, a, n, 100.0, "prctile"));
    r.register("skewness", |_, a, _| moment_stat(a, "skewness"));
    r.register("kurtosis", |_, a, _| moment_stat(a, "kurtosis"));
    r.register("cov", cov);
    r.register("corrcoef", corrcoef);
    r.register("corr", corr);
    r.register("histcounts", histcounts);
    r.register("normpdf", |_, a, _| bcast(a, "normpdf", 3, &[0.0, 1.0], |v| Normal::new(v[1], v[2]).map_or(f64::NAN, |d| d.pdf(v[0]))));
    r.register("normcdf", |_, a, _| bcast(a, "normcdf", 3, &[0.0, 1.0], |v| if v[2] > 0.0 { phi((v[0] - v[1]) / v[2]) } else { f64::NAN }));
    r.register("norminv", |_, a, _| bcast(a, "norminv", 3, &[0.0, 1.0], |v| if v[2] > 0.0 { v[1] + v[2] * inv(v[0], phi_inv) } else { f64::NAN }));
    r.register("erf", |_, a, _| bcast(a, "erf", 1, &[], |v| libm::erf(v[0])));
    r.register("erfc", |_, a, _| bcast(a, "erfc", 1, &[], |v| erfc(v[0])));
    r.register("tpdf", |_, a, _| bcast(a, "tpdf", 2, &[], |v| tdist(v[1]).map_or(f64::NAN, |d| d.pdf(v[0]))));
    r.register("tcdf", |_, a, _| bcast(a, "tcdf", 2, &[], |v| tdist(v[1]).map_or(f64::NAN, |d| d.cdf(v[0]))));
    r.register("tinv", |_, a, _| bcast(a, "tinv", 2, &[], |v| tdist(v[1]).map_or(f64::NAN, |d| inv(v[0], |p| d.inverse_cdf(p)))));
    r.register("chi2pdf", |_, a, _| bcast(a, "chi2pdf", 2, &[], |v| ChiSquared::new(v[1]).map_or(f64::NAN, |d| if v[0] < 0.0 { 0.0 } else { d.pdf(v[0]) })));
    r.register("chi2cdf", |_, a, _| bcast(a, "chi2cdf", 2, &[], |v| ChiSquared::new(v[1]).map_or(f64::NAN, |d| if v[0] <= 0.0 { 0.0 } else { d.cdf(v[0]) })));
    r.register("chi2inv", |_, a, _| bcast(a, "chi2inv", 2, &[], |v| ChiSquared::new(v[1]).map_or(f64::NAN, |d| inv(v[0], |p| d.inverse_cdf(p)))));
    r.register("fpdf", |_, a, _| bcast(a, "fpdf", 3, &[], |v| FisherSnedecor::new(v[1], v[2]).map_or(f64::NAN, |d| if v[0] < 0.0 { 0.0 } else { d.pdf(v[0]) })));
    r.register("fcdf", |_, a, _| bcast(a, "fcdf", 3, &[], |v| FisherSnedecor::new(v[1], v[2]).map_or(f64::NAN, |d| if v[0] <= 0.0 { 0.0 } else { d.cdf(v[0]) })));
    r.register("finv", |_, a, _| bcast(a, "finv", 3, &[], |v| FisherSnedecor::new(v[1], v[2]).map_or(f64::NAN, |d| inv(v[0], |p| d.inverse_cdf(p)))));
    r.register("binopdf", |_, a, _| bcast(a, "binopdf", 3, &[], |v| binom(v[1], v[2]).map_or(f64::NAN, |d| if v[0] < 0.0 || v[0] != v[0].trunc() || v[0] > v[1] { 0.0 } else { d.pmf(v[0] as u64) })));
    r.register("binocdf", |_, a, _| bcast(a, "binocdf", 3, &[], |v| binom(v[1], v[2]).map_or(f64::NAN, |d| if v[0] < 0.0 { 0.0 } else if v[0] >= v[1] { 1.0 } else { d.cdf(v[0].floor() as u64) })));
    r.register("binoinv", |_, a, _| bcast(a, "binoinv", 3, &[], |v| binom(v[1], v[2]).map_or(f64::NAN, |d| dinv(v[0], v[1], |k| d.cdf(k)))));
    r.register("poisspdf", |_, a, _| bcast(a, "poisspdf", 2, &[], |v| pois(v[1]).map_or(f64::NAN, |d| if v[0] < 0.0 || v[0] != v[0].trunc() { 0.0 } else { d.pmf(v[0] as u64) })));
    r.register("poisscdf", |_, a, _| bcast(a, "poisscdf", 2, &[], |v| pois(v[1]).map_or(f64::NAN, |d| if v[0] < 0.0 { 0.0 } else { d.cdf(v[0].floor() as u64) })));
    r.register("poissinv", |_, a, _| bcast(a, "poissinv", 2, &[], |v| pois(v[1]).map_or(f64::NAN, |d| dinv(v[0], f64::INFINITY, |k| d.cdf(k)))));
    r.register("ttest", ttest);
    r.register("ttest2", ttest2);
    r.register("chi2test", chi2test);
    r.register("crosstab", crosstab);
    r.register("anova1", anova1);
    r.register("fitlm", fitlm);
}

fn g6(x: f64) -> String {
    super::io::c_g(x, 6, false, false)
}

// ---------- distributions ----------

// Normal distribution: statrs 0.19.1 gives erfc with relative error ~5e-11 (normcdf(−1.96) =
// 0.02499789514709759 vs 0.024997895148220435; found by scenario dist-01 of suite v2), so v2 had its own
// formulas: erf = 1 − erfc and erfc = 1 − series for x < 2. Boost.Math data (tests3, `tests/ext_boost.rs`) showed
// cancellation: erf(1.4e−45) = 0, erf near 0 — 15 eps, erfc near 2 — 410 eps. Now — `libm` (port of fdlibm/musl,
// MIT): < 1 ulp per the fdlibm documentation, on Boost data — see README.

pub fn erfc(x: f64) -> f64 {
    libm::erfc(x)
}

/// Φ(z) = erfc(−z/√2)/2.
pub fn phi(z: f64) -> f64 {
    if z == f64::INFINITY {
        return 1.0;
    }
    if z == f64::NEG_INFINITY {
        return 0.0;
    }
    0.5 * erfc(-z / std::f64::consts::SQRT_2)
}

/// Φ⁻¹: start from statrs, then two Newton steps on the exact Φ.
pub fn phi_inv(p: f64) -> f64 {
    if p <= 0.0 {
        return if p == 0.0 { f64::NEG_INFINITY } else { f64::NAN };
    }
    if p >= 1.0 {
        return if p == 1.0 { f64::INFINITY } else { f64::NAN };
    }
    let mut x = Normal::new(0.0, 1.0).unwrap().inverse_cdf(p);
    for _ in 0..2 {
        let dens = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();
        if dens > 0.0 {
            x -= (phi(x) - p) / dens;
        }
    }
    x
}

fn tdist(v: f64) -> Option<StudentsT> {
    StudentsT::new(0.0, 1.0, v).ok()
}
fn binom(n: f64, p: f64) -> Option<Binomial> {
    if n < 0.0 || n != n.trunc() { None } else { Binomial::new(p, n as u64).ok() }
}
fn pois(l: f64) -> Option<Poisson> {
    Poisson::new(l).ok()
}
fn inv(p: f64, f: impl Fn(f64) -> f64) -> f64 {
    if !(0.0..=1.0).contains(&p) || p.is_nan() { f64::NAN } else { f(p) }
}
/// Inverse of a discrete distribution: the smallest k with F(k) ≥ p.
fn dinv(p: f64, kmax: f64, cdf: impl Fn(u64) -> f64) -> f64 {
    if !(0.0..=1.0).contains(&p) || p.is_nan() {
        return f64::NAN;
    }
    if p == 1.0 {
        return kmax;
    }
    let mut k: u64 = 0;
    while cdf(k) < p {
        k += 1;
        if k as f64 > kmax {
            return kmax;
        }
    }
    k as f64
}

/// Elementwise application with scalar parameter broadcasting; missing parameters — standard ones.
fn bcast(a: &[Value], name: &str, n: usize, defaults: &[f64], f: impl Fn(&[f64]) -> f64) -> Result<Vec<Value>, MError> {
    let required = n - defaults.len();
    if a.len() < required || a.len() > n {
        return Err(super::invalid_call(name));
    }
    let mut ms: Vec<Mat> = Vec::with_capacity(n);
    for k in 0..n {
        ms.push(if k < a.len() { mat_arg(a, k, name)?.clone() } else { Mat::scalar(defaults[k - required]) });
    }
    let shape = ms.iter().find(|m| !m.is_scalar()).map_or((1, 1), |m| (m.rows, m.cols));
    for m in &ms {
        if !m.is_scalar() && (m.rows, m.cols) != shape {
            return Err(MError::new(format!("{name}: nonconformant arguments")));
        }
    }
    let len = shape.0 * shape.1;
    let mut buf = vec![0.0; n];
    let re = (0..len)
        .map(|i| {
            for (k, m) in ms.iter().enumerate() {
                buf[k] = if m.is_scalar() { m.re[0] } else { m.re[i] };
            }
            f(&buf)
        })
        .collect();
    one(Mat::new(shape.0, shape.1, re))
}

// ---------- descriptive ----------

fn cols_of(m: &Mat) -> Vec<Vec<f64>> {
    if m.rows == 1 || m.cols == 1 {
        return vec![m.re.clone()];
    }
    (0..m.cols).map(|j| m.re[j * m.rows..(j + 1) * m.rows].to_vec()).collect()
}

fn per_col(m: &Mat, f: impl Fn(&[f64]) -> f64) -> Mat {
    let cs = cols_of(m);
    if cs.len() == 1 { Mat::scalar(f(&cs[0])) } else { Mat::row(cs.iter().map(|c| f(c)).collect()) }
}

fn clean(x: &[f64]) -> Vec<f64> {
    x.iter().copied().filter(|v| !v.is_nan()).collect()
}

fn mode(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "mode")?;
    // mode(A, 2) — along rows (the dimension argument used to be ignored; vmm-stat-006, tests3)
    if a.len() > 1 && super::scalar_arg(a, 1, "mode")? == 2.0 && m.rows > 1 {
        let t = per_col(&m.transpose(), |c| super::table::agg("mode", c).unwrap_or(f64::NAN));
        return one(t.transpose());
    }
    one(per_col(m, |c| super::table::agg("mode", c).unwrap_or(f64::NAN)))
}

/// Quantile by the MATLAB method: sorted values are quantiles (k − 0.5)/n, linear in between, min/max outside.
pub fn quantile_sorted(s: &[f64], p: f64) -> f64 {
    let n = s.len();
    if n == 0 || p.is_nan() {
        return f64::NAN;
    }
    let pos = n as f64 * p + 0.5;
    if pos <= 1.0 {
        return s[0];
    }
    if pos >= n as f64 {
        return s[n - 1];
    }
    let lo = pos.floor() as usize;
    let fr = pos - lo as f64;
    s[lo - 1] + fr * (s[lo] - s[lo - 1])
}

fn quantile(_: &mut Interp, a: &[Value], _: usize, scale: f64, name: &str) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, name)?;
    let p = mat_arg(a, 1, name)?;
    if p.re.iter().any(|&x| !(0.0..=scale).contains(&x)) {
        return Err(MError::new(format!("{name}: probabilities must be in [0, {scale}]")));
    }
    let cs: Vec<Vec<f64>> = cols_of(m)
        .into_iter()
        .map(|c| {
            let mut s = clean(&c);
            s.sort_by(|x, y| x.partial_cmp(y).unwrap());
            s
        })
        .collect();
    if cs.len() == 1 {
        let re = p.re.iter().map(|&q| quantile_sorted(&cs[0], q / scale)).collect();
        return one(Mat::new(p.rows, p.cols, re));
    }
    let np = p.numel();
    let mut out = Mat::zeros(np, cs.len());
    for (j, c) in cs.iter().enumerate() {
        for (i, &q) in p.re.iter().enumerate() {
            out.re[j * np + i] = quantile_sorted(c, q / scale);
        }
    }
    one(out)
}

pub fn moments(x: &[f64]) -> (f64, f64, f64, f64, f64) {
    let x = clean(x);
    let n = x.len() as f64;
    let mean = x.iter().sum::<f64>() / n;
    let m = |k: i32| x.iter().map(|v| (v - mean).powi(k)).sum::<f64>() / n;
    (n, mean, m(2), m(3), m(4))
}

fn moment_stat(a: &[Value], name: &str) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, name)?;
    let flag = match a.get(1) {
        Some(Value::Mat(f)) if !f.is_empty() => f.re[0],
        _ => 1.0,
    };
    let skew = name == "skewness";
    one(per_col(m, |c| {
        let (n, _, m2, m3, m4) = moments(c);
        if skew {
            let s1 = m3 / m2.powf(1.5);
            if flag == 0.0 { if n < 3.0 { f64::NAN } else { s1 * (n * (n - 1.0)).sqrt() / (n - 2.0) } } else { s1 }
        } else {
            let k1 = m4 / (m2 * m2);
            if flag == 0.0 { if n < 4.0 { f64::NAN } else { 3.0 + (n - 1.0) / ((n - 2.0) * (n - 3.0)) * ((n + 1.0) * k1 - 3.0 * (n - 1.0)) } } else { k1 }
        }
    }))
}

/// Observation columns (x, y as vectors → two columns; matrix → its columns).
fn obs_cols(a: &[Value], name: &str) -> Result<Vec<Vec<f64>>, MError> {
    let x = mat_arg(a, 0, name)?;
    if let Some(Value::Mat(y)) = a.get(1).filter(|v| !is_str(v)) {
        if x.numel() != y.numel() {
            return Err(MError::new(format!("{name}: X and Y must have the same number of elements")));
        }
        return Ok(vec![x.re.clone(), y.re.clone()]);
    }
    if x.rows == 1 {
        return Ok(vec![x.re.clone()]);
    }
    Ok(cols_of(x))
}

fn cov_matrix(cs: &[Vec<f64>]) -> Mat {
    cov_matrix_w(cs, false)
}

/// `by_n` — normalize by N (cov(x, 1)), otherwise by N − 1.
fn cov_matrix_w(cs: &[Vec<f64>], by_n: bool) -> Mat {
    let p = cs.len();
    let n = cs[0].len() as f64;
    let means: Vec<f64> = cs.iter().map(|c| c.iter().sum::<f64>() / n).collect();
    let mut out = Mat::zeros(p, p);
    for i in 0..p {
        for j in 0..p {
            let s: f64 = (0..cs[0].len()).map(|k| (cs[i][k] - means[i]) * (cs[j][k] - means[j])).sum();
            out.re[j * p + i] = s / if by_n { n } else { n - 1.0 };
        }
    }
    out
}

fn cov(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    // cov(X, w), w = 0 | 1 — normalization (was: w taken as Y; vmm-stat-017, tests3)
    if let (Some(Value::Mat(w)), Some(Value::Mat(x))) = (a.get(1), a.first()) {
        if w.numel() == 1 && x.numel() > 1 && (w.re[0] == 0.0 || w.re[0] == 1.0) {
            return one(cov_matrix_w(&obs_cols(&a[..1], "cov")?, w.re[0] == 1.0));
        }
    }
    one(cov_matrix(&obs_cols(a, "cov")?))
}

/// p for H0: ρ = 0 (t with n − 2 degrees of freedom); fault — wrong p from the normal.
pub fn corr_p(r: f64, n: f64, fault: bool) -> (f64, f64) {
    let df = n - 2.0;
    let t = r * (df / (1.0 - r * r)).sqrt();
    let p = if r.abs() >= 1.0 { 0.0 } else { t_p(t, df, Tail::Both, fault) };
    (t, p)
}

fn corr_matrix(cs: &[Vec<f64>]) -> (Mat, Mat) {
    let c = cov_matrix(cs);
    let p = cs.len();
    let n = cs[0].len() as f64;
    let mut r = Mat::zeros(p, p);
    let mut pv = Mat::filled(p, p, 1.0);
    for i in 0..p {
        for j in 0..p {
            let v = if i == j { 1.0 } else { c.re[j * p + i] / (c.re[i * p + i] * c.re[j * p + j]).sqrt() };
            r.re[j * p + i] = v;
            if i != j {
                pv.re[j * p + i] = corr_p(v, n, false).1;
            }
        }
    }
    (r, pv)
}

fn corrcoef(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let cs = obs_cols(a, "corrcoef")?;
    if cs.is_empty() || cs[0].is_empty() {
        // corrcoef([]) — NaN (was: panic on cs[0]; Octave tests, tests3)
        return Ok(vec![Value::Mat(Mat::scalar(f64::NAN)); nargout.max(1)]);
    }
    let (r, p) = corr_matrix(&cs);
    let mut out = vec![Value::Mat(r)];
    if nargout > 1 {
        out.push(Value::Mat(p));
    }
    Ok(out)
}

fn histcounts(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let x = clean(&mat_arg(a, 0, "histcounts")?.re);
    let edges: Vec<f64> = match a.get(1) {
        Some(Value::Mat(e)) if e.numel() > 1 => e.re.clone(),
        other => {
            let nb = match other {
                Some(Value::Mat(e)) if e.numel() == 1 => e.re[0].max(1.0) as usize,
                _ => ((x.len().max(1) as f64).log2().ceil() as usize + 1).max(1),
            };
            let lo = x.iter().copied().fold(f64::INFINITY, f64::min);
            let hi = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let (lo, hi) = if x.is_empty() { (0.0, 1.0) } else if lo == hi { (lo - 0.5, hi + 0.5) } else { (lo, hi) };
            (0..=nb).map(|k| if k == nb { hi } else { lo + (hi - lo) * k as f64 / nb as f64 }).collect()
        }
    };
    let nb = edges.len() - 1;
    let mut n = vec![0.0; nb];
    for &v in &x {
        if v < edges[0] || v > edges[nb] {
            continue;
        }
        let k = if v == edges[nb] { nb - 1 } else { edges.partition_point(|&e| e <= v) - 1 };
        n[k] += 1.0;
    }
    let mut out = vec![Value::Mat(Mat::row(n))];
    if nargout > 1 {
        out.push(Value::Mat(Mat::row(edges)));
    }
    Ok(out)
}

// ---------- tests: common ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tail {
    Both,
    Right,
    Left,
}

/// p for a t statistic; `wrong` — negative control (normal instead of t).
pub fn t_p(t: f64, df: f64, tail: Tail, wrong: bool) -> f64 {
    // t = NaN (constant sample) or df ≤ 0: statrs panics in the beta function — found by Octave tests (tests3)
    if t.is_nan() || !(df > 0.0) {
        return f64::NAN;
    }
    let sf = |x: f64| -> f64 {
        if wrong {
            Normal::new(0.0, 1.0).unwrap().sf(x)
        } else {
            tdist(df).map_or(f64::NAN, |d| d.sf(x))
        }
    };
    match tail {
        Tail::Both => (2.0 * sf(t.abs())).min(1.0),
        Tail::Right => sf(t),
        Tail::Left => sf(-t),
    }
}

fn tcrit(p: f64, df: f64) -> f64 {
    tdist(df).map_or(f64::NAN, |d| d.inverse_cdf(p))
}

pub struct TestOpts {
    pub alpha: f64,
    pub tail: Tail,
    pub display: bool,
    pub equal_var: bool,
}

fn test_opts(o: &[(String, Value)], fname: &str) -> Result<TestOpts, MError> {
    let alpha = match opt(o, "alpha") {
        Some(Value::Mat(m)) if m.numel() == 1 && m.re[0] > 0.0 && m.re[0] < 1.0 => m.re[0],
        Some(_) => return Err(MError::new(format!("{fname}: 'Alpha' must be a scalar in (0, 1)"))),
        None => 0.05,
    };
    let tail = match opt(o, "tail").and_then(str_of).map(|s| s.to_ascii_lowercase()).as_deref() {
        None | Some("both") => Tail::Both,
        Some("right") => Tail::Right,
        Some("left") => Tail::Left,
        Some(s) => return Err(MError::new(format!("{fname}: unknown tail '{s}' (both, right, left)"))),
    };
    let display = !matches!(opt(o, "display").and_then(str_of).map(|s| s.to_ascii_lowercase()).as_deref(), Some("off"));
    let equal_var = !matches!(opt(o, "vartype").and_then(str_of).map(|s| s.to_ascii_lowercase()).as_deref(), Some("unequal"));
    Ok(TestOpts { alpha, tail, display, equal_var })
}

fn fault_is(it: &Interp, test: &str) -> bool {
    it.fault.as_deref().and_then(|f| f.strip_prefix("p:")) == Some(test)
}

fn verdict(p: f64, alpha: f64) -> (f64, String) {
    if p < alpha { (1.0, "p < α → reject H0 (h = 1)".into()) } else { (0.0, "p ≥ α → do not reject H0 (h = 0)".into()) }
}

fn h1_sign(tail: Tail) -> (&'static str, &'static str, &'static str) {
    match tail {
        Tail::Both => ("≠", "two-sided", "p = 2·P(T ≥ |t|)"),
        Tail::Right => (">", "right-tailed", "p = P(T ≥ t)"),
        Tail::Left => ("<", "left-tailed", "p = P(T ≤ t)"),
    }
}

fn stats_row(names: &[&str], vals: &[f64]) -> Value {
    let cols: Vec<Value> = vals.iter().map(|&v| Value::num(v)).collect();
    Value::Table(Rc::new(tb::make(names.iter().map(|s| s.to_string()).collect(), &cols).expect("stats row")))
}

fn ci_of(est: f64, se: f64, df: f64, o: &TestOpts) -> (f64, f64) {
    match o.tail {
        Tail::Both => {
            let c = tcrit(1.0 - o.alpha / 2.0, df) * se;
            (est - c, est + c)
        }
        Tail::Right => (est - tcrit(1.0 - o.alpha, df) * se, f64::INFINITY),
        Tail::Left => (f64::NEG_INFINITY, est + tcrit(1.0 - o.alpha, df) * se),
    }
}

// ---------- t-tests ----------

pub struct TOne {
    pub n: f64,
    pub mean: f64,
    pub sd: f64,
    pub se: f64,
    pub t: f64,
    pub df: f64,
    pub p: f64,
}

pub fn t_one(x: &[f64], m0: f64, tail: Tail, fault: bool) -> TOne {
    let x = clean(x);
    let n = x.len() as f64;
    let mean = x.iter().sum::<f64>() / n;
    let sd = (x.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / (n - 1.0)).sqrt();
    let se = sd / n.sqrt();
    let t = (mean - m0) / se;
    let df = n - 1.0;
    TOne { n, mean, sd, se, t, df, p: t_p(t, df, tail, fault) }
}

pub struct TTwo {
    pub nx: f64,
    pub ny: f64,
    pub mx: f64,
    pub my: f64,
    pub vx: f64,
    pub vy: f64,
    pub sp: f64,
    pub se: f64,
    pub t: f64,
    pub df: f64,
    pub p: f64,
}

pub fn t_two(x: &[f64], y: &[f64], equal: bool, tail: Tail, fault: bool) -> TTwo {
    let (x, y) = (clean(x), clean(y));
    let (nx, ny) = (x.len() as f64, y.len() as f64);
    let mx = x.iter().sum::<f64>() / nx;
    let my = y.iter().sum::<f64>() / ny;
    let vx = x.iter().map(|v| (v - mx) * (v - mx)).sum::<f64>() / (nx - 1.0);
    let vy = y.iter().map(|v| (v - my) * (v - my)).sum::<f64>() / (ny - 1.0);
    let sp = (((nx - 1.0) * vx + (ny - 1.0) * vy) / (nx + ny - 2.0)).sqrt();
    let (se, df) = if equal {
        (sp * (1.0 / nx + 1.0 / ny).sqrt(), nx + ny - 2.0)
    } else {
        let (a, b) = (vx / nx, vy / ny);
        ((a + b).sqrt(), (a + b) * (a + b) / (a * a / (nx - 1.0) + b * b / (ny - 1.0)))
    };
    let t = (mx - my) / se;
    TTwo { nx, ny, mx, my, vx, vy, sp, se, t, df, p: t_p(t, df, tail, fault) }
}

fn vec_arg(a: &[Value], k: usize, name: &str) -> Result<Vec<f64>, MError> {
    Ok(mat_arg(a, k, name)?.re.clone())
}

fn ttest(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let x = vec_arg(a, 0, "ttest")?;
    let (mut m0, mut paired, mut k) = (0.0, None, 1);
    if let Some(Value::Mat(m)) = a.get(1).filter(|v| !is_str(v)) {
        if m.numel() == 1 && x.len() != 1 {
            m0 = m.re[0];
        } else {
            if m.numel() != x.len() {
                return Err(MError::new("ttest: X and Y must have the same number of elements"));
            }
            paired = Some(m.re.clone());
        }
        k = 2;
    }
    let o = test_opts(&opts(a, k, "ttest")?, "ttest")?;
    let d: Vec<f64> = match &paired {
        Some(y) => x.iter().zip(y).map(|(a, b)| a - b).collect(),
        None => x.clone(),
    };
    if clean(&d).len() < 2 {
        return Err(MError::new("ttest: need at least 2 observations"));
    }
    let r = t_one(&d, m0, o.tail, fault_is(it, "ttest"));
    let ci = ci_of(r.mean, r.se, r.df, &o);
    let (h, concl) = verdict(r.p, o.alpha);
    if o.display {
        let (sign, side, pform) = h1_sign(o.tail);
        let (title, mu, bar) = if paired.is_some() { ("paired t-test (differences d = x − y)", "μd", "d̄") } else { ("one-sample t-test", "μ", "x̄") };
        let mut s = format!("ttest: {title}\n");
        s += &format!("  H0: {mu} = {}; H1: {mu} {sign} {} ({side}), α = {}\n", g6(m0), g6(m0), g6(o.alpha));
        s += "  assumptions:\n";
        s += "    independence of observations — not checked from the data\n";
        s += &format!("    normality of the {} (or large n) — n = {}\n", if paired.is_some() { "differences" } else { "population" }, g6(r.n));
        s += "  steps:\n";
        s += &format!("    n = {}, {bar} = {}, s = {}\n", g6(r.n), g6(r.mean), g6(r.sd));
        s += &format!("    SE = s/√n = {}\n", g6(r.se));
        s += &format!("    t = ({bar} − μ0)/SE = {}, df = n − 1 = {}\n", g6(r.t), g6(r.df));
        s += &format!("    {pform} = {}\n", g6(r.p));
        s += &format!("  conclusion: {concl}; {}% CI for {mu}: [{}, {}]\n", g6(100.0 * (1.0 - o.alpha)), g6(ci.0), g6(ci.1));
        it.out(&s);
    }
    Ok(vec![Value::num(h), Value::num(r.p), Value::Mat(Mat::row(vec![ci.0, ci.1])), stats_row(&["tstat", "df", "sd"], &[r.t, r.df, r.sd])])
}

fn ttest2(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let x = vec_arg(a, 0, "ttest2")?;
    let y = vec_arg(a, 1, "ttest2")?;
    let o = test_opts(&opts(a, 2, "ttest2")?, "ttest2")?;
    if clean(&x).len() < 2 || clean(&y).len() < 2 {
        return Err(MError::new("ttest2: need at least 2 observations in each sample"));
    }
    let r = t_two(&x, &y, o.equal_var, o.tail, fault_is(it, "ttest2"));
    let ci = ci_of(r.mx - r.my, r.se, r.df, &o);
    let (h, concl) = verdict(r.p, o.alpha);
    if o.display {
        let (sign, side, pform) = h1_sign(o.tail);
        let ratio = r.vx.max(r.vy) / r.vx.min(r.vy);
        let mut s = format!("ttest2: two-sample t-test for independent samples ({})\n", if o.equal_var { "equal variances" } else { "Welch" });
        s += &format!("  H0: μx = μy; H1: μx {sign} μy ({side}), α = {}\n", g6(o.alpha));
        s += "  assumptions:\n";
        s += "    independence of observations within and between samples — not checked from the data\n";
        s += &format!("    normality of both populations (or large n) — nx = {}, ny = {}\n", g6(r.nx), g6(r.ny));
        if o.equal_var {
            s += &format!("    equal variances: assumed (Vartype 'equal', as in MATLAB); max s²/min s² = {}{}\n", g6(ratio), if ratio > 4.0 { " — doubtful, prefer 'Vartype','unequal' (Welch)" } else { "" });
        } else {
            s += &format!("    equal variances not required (Welch); max s²/min s² = {}\n", g6(ratio));
        }
        s += "  steps:\n";
        s += &format!("    x̄ = {}, ȳ = {}, s²x = {}, s²y = {}\n", g6(r.mx), g6(r.my), g6(r.vx), g6(r.vy));
        if o.equal_var {
            s += &format!("    sp = √(((nx − 1)s²x + (ny − 1)s²y)/(nx + ny − 2)) = {}, SE = sp·√(1/nx + 1/ny) = {}\n", g6(r.sp), g6(r.se));
            s += &format!("    t = (x̄ − ȳ)/SE = {}, df = nx + ny − 2 = {}\n", g6(r.t), g6(r.df));
        } else {
            s += &format!("    SE = √(s²x/nx + s²y/ny) = {}\n", g6(r.se));
            s += &format!("    t = (x̄ − ȳ)/SE = {}, df (Welch — Satterthwaite) = {}\n", g6(r.t), g6(r.df));
        }
        s += &format!("    {pform} = {}\n", g6(r.p));
        s += &format!("  conclusion: {concl}; {}% CI for μx − μy: [{}, {}]\n", g6(100.0 * (1.0 - o.alpha)), g6(ci.0), g6(ci.1));
        it.out(&s);
    }
    let st = if o.equal_var {
        stats_row(&["tstat", "df", "sd"], &[r.t, r.df, r.sp])
    } else {
        stats_row(&["tstat", "df", "sdx", "sdy"], &[r.t, r.df, r.vx.sqrt(), r.vy.sqrt()])
    };
    Ok(vec![Value::num(h), Value::num(r.p), Value::Mat(Mat::row(vec![ci.0, ci.1])), st])
}

// ---------- χ² ----------

pub struct Chi2 {
    pub chi2: f64,
    pub df: f64,
    pub p: f64,
    pub n: f64,
    pub expected: Vec<Vec<f64>>,
}

pub fn chi2_ind(obs: &[Vec<f64>], fault: bool) -> Result<Chi2, MError> {
    let r = obs.len();
    let c = obs[0].len();
    let rs: Vec<f64> = obs.iter().map(|row| row.iter().sum()).collect();
    let cs: Vec<f64> = (0..c).map(|j| obs.iter().map(|row| row[j]).sum()).collect();
    let n: f64 = rs.iter().sum();
    if rs.iter().chain(&cs).any(|&s| s <= 0.0) {
        return Err(MError::new("chi2test: every row and column total must be positive"));
    }
    let mut e = vec![vec![0.0; c]; r];
    let mut chi2 = 0.0;
    for i in 0..r {
        for j in 0..c {
            e[i][j] = rs[i] * cs[j] / n;
            chi2 += (obs[i][j] - e[i][j]).powi(2) / e[i][j];
        }
    }
    let df = ((r - 1) * (c - 1)) as f64 + if fault { 1.0 } else { 0.0 };
    let p = ChiSquared::new(df).map_or(f64::NAN, |d| d.sf(chi2));
    Ok(Chi2 { chi2, df, p, n, expected: e })
}

fn rows_of(m: &Mat) -> Vec<Vec<f64>> {
    (0..m.rows).map(|i| (0..m.cols).map(|j| m.at(i, j)).collect()).collect()
}

fn chi2test(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "chi2test")?;
    if m.rows < 2 || m.cols < 2 {
        return Err(MError::new("chi2test: need a contingency table with at least 2 rows and 2 columns"));
    }
    let o = test_opts(&opts(a, 1, "chi2test")?, "chi2test")?;
    let r = chi2_ind(&rows_of(m), fault_is(it, "chi2test"))?;
    if o.display {
        let (h, concl) = verdict(r.p, o.alpha);
        let _ = h;
        let cells = m.rows * m.cols;
        let small = r.expected.iter().flatten().filter(|&&e| e < 5.0).count();
        let emin = r.expected.iter().flatten().copied().fold(f64::INFINITY, f64::min);
        let mut s = format!("chi2test: χ² test of independence ({}x{} table)\n", m.rows, m.cols);
        s += &format!("  H0: row and column variables are independent, α = {}\n", g6(o.alpha));
        s += "  assumptions:\n";
        s += "    independence of observations; each falls into exactly one cell\n";
        if small > 0 {
            s += &format!("    expected counts ≥ 5: violated — {small} of {cells} cells < 5 (χ² approximation unreliable)\n");
        } else {
            s += &format!("    expected counts ≥ 5: satisfied (minimum E = {})\n", g6(emin));
        }
        s += "  steps:\n";
        s += &format!("    E = (row sum)·(column sum)/n, n = {}\n", g6(r.n));
        s += &format!("    χ² = Σ (O − E)²/E = {}\n", g6(r.chi2));
        s += &format!("    df = (r − 1)(c − 1) = {}\n", g6(r.df));
        s += &format!("    p = P(χ²(df) ≥ χ²) = {}\n", g6(r.p));
        s += &format!("  conclusion: {concl}\n");
        it.out(&s);
    }
    let e = r.expected.clone();
    let em = Mat::new(m.rows, m.cols, (0..m.cols).flat_map(|j| e.iter().map(move |row| row[j])).collect());
    Ok(vec![Value::num(r.p), Value::num(r.chi2), Value::num(r.df), Value::Mat(em)])
}

/// Vector categories (numbers or text) → group numbers in ascending order.
fn categories(v: &Value, name: &str) -> Result<(Vec<usize>, usize), MError> {
    let col = match v {
        Value::Mat(m) => Value::Mat(Mat::col(m.re.clone())),
        Value::Str(s) => Value::Str(Rc::new(StrArr::col(s.data.clone()))),
        other => return Err(MError::new(format!("{name}: wrong type argument '{}'", other.class_name()))),
    };
    let n = tb::col_len(&col);
    let gs = tb::groups(&[&col], n);
    let mut id = vec![0; n];
    for (g, rows) in gs.iter().enumerate() {
        for &i in rows {
            id[i] = g;
        }
    }
    Ok((id, gs.len()))
}

fn crosstab(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let (x, nx) = categories(arg(a, 0, "crosstab")?, "crosstab")?;
    let (y, ny) = categories(arg(a, 1, "crosstab")?, "crosstab")?;
    if x.len() != y.len() {
        return Err(MError::new("crosstab: grouping variables must have the same length"));
    }
    let mut t = vec![vec![0.0; ny]; nx];
    for (&i, &j) in x.iter().zip(&y) {
        t[i][j] += 1.0;
    }
    let tm = Mat::new(nx, ny, (0..ny).flat_map(|j| t.iter().map(move |row| row[j])).collect());
    let (c2, p) = if nx >= 2 && ny >= 2 { chi2_ind(&t, false).map_or((f64::NAN, f64::NAN), |r| (r.chi2, r.p)) } else { (0.0, 1.0) };
    Ok(vec![Value::Mat(tm), Value::num(c2), Value::num(p)])
}

// ---------- ANOVA ----------

pub struct Anova {
    pub ssb: f64,
    pub ssw: f64,
    pub df1: f64,
    pub df2: f64,
    pub f: f64,
    pub p: f64,
    pub vratio: f64,
}

pub fn anova_one(groups: &[Vec<f64>], fault: bool) -> Anova {
    // SSB and SSW do not depend on a shift: subtract the first observation (exact for close values,
    // Sterbenz lemma), means — with a corrective second pass. Without this, data like 10^12 + 0.4
    // (NIST SmLs07–09) lost all significant digits in the sum.
    let c = groups.iter().flatten().next().copied().unwrap_or(0.0);
    let groups: Vec<Vec<f64>> = groups.iter().map(|g| g.iter().map(|v| v - c).collect()).collect();
    let mean2 = |v: &[f64]| -> f64 {
        let m = v.iter().sum::<f64>() / v.len() as f64;
        m + v.iter().map(|x| x - m).sum::<f64>() / v.len() as f64
    };
    let all: Vec<f64> = groups.iter().flatten().copied().collect();
    let n = all.len() as f64;
    let k = groups.len() as f64;
    let gm = mean2(&all);
    let (mut ssb, mut ssw) = (0.0, 0.0);
    let (mut vmin, mut vmax) = (f64::INFINITY, 0.0f64);
    for g in &groups {
        let ni = g.len() as f64;
        let mi = mean2(g);
        ssb += ni * (mi - gm) * (mi - gm);
        let ss: f64 = g.iter().map(|v| (v - mi) * (v - mi)).sum();
        ssw += ss;
        if ni > 1.0 {
            let v = ss / (ni - 1.0);
            vmin = vmin.min(v);
            vmax = vmax.max(v);
        }
    }
    let (df1, df2) = (k - 1.0, n - k);
    let f = (ssb / df1) / (ssw / df2);
    let p = if fault {
        ChiSquared::new(df1).map_or(f64::NAN, |d| d.sf(df1 * f))
    } else {
        FisherSnedecor::new(df1, df2).map_or(f64::NAN, |d| d.sf(f))
    };
    Anova { ssb, ssw, df1, df2, f, p, vratio: vmax / vmin }
}

fn anova1(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let y = mat_arg(a, 0, "anova1")?;
    let (mut groups, mut labels): (Vec<Vec<f64>>, Vec<String>) = (Vec::new(), Vec::new());
    let mut k = 1;
    match a.get(1).filter(|v| !is_str(v) || matches!(v, Value::Str(_))) {
        Some(g) if !(matches!(g, Value::Mat(m) if m.is_empty())) => {
            let (id, ng) = categories(g, "anova1")?;
            if id.len() != y.numel() {
                return Err(MError::new("anova1: Y and GROUP must have the same length"));
            }
            groups = vec![Vec::new(); ng];
            for (i, &gi) in id.iter().enumerate() {
                if !y.re[i].is_nan() {
                    groups[gi].push(y.re[i]);
                }
            }
            labels = (1..=ng).map(|g| g.to_string()).collect();
            k = 2;
        }
        Some(_) => k = 2,
        None => {}
    }
    if groups.is_empty() {
        groups = cols_of(y).iter().map(|c| clean(c)).collect();
        labels = (1..=groups.len()).map(|g| g.to_string()).collect();
    }
    let display = !matches!(a.get(k).and_then(str_of).map(|s| s.to_ascii_lowercase()).as_deref(), Some("off"));
    if groups.len() < 2 || groups.iter().any(|g| g.is_empty()) {
        return Err(MError::new("anova1: need at least 2 non-empty groups"));
    }
    let r = anova_one(&groups, fault_is(it, "anova1"));
    let _ = labels;
    if display {
        let n: usize = groups.iter().map(|g| g.len()).sum();
        let (_, concl) = verdict(r.p, 0.05);
        let mut s = format!("anova1: one-way analysis of variance, k = {} groups, N = {n}\n", groups.len());
        s += "  H0: all group means are equal, α = 0.05\n";
        s += "  assumptions:\n";
        s += "    independence of observations — not checked from the data\n";
        s += "    normality within each group\n";
        s += &format!("    equal group variances: max s²/min s² = {}{}\n", g6(r.vratio), if r.vratio > 4.0 { " — doubtful (> 4)" } else { " (< 4 — acceptable)" });
        s += "  steps:\n";
        s += &format!("    SSB = Σ nᵢ(x̄ᵢ − x̄)² = {}, df1 = k − 1 = {}\n", g6(r.ssb), g6(r.df1));
        s += &format!("    SSW = Σ Σ (x − x̄ᵢ)² = {}, df2 = N − k = {}\n", g6(r.ssw), g6(r.df2));
        s += &format!("    F = (SSB/df1)/(SSW/df2) = {}\n", g6(r.f));
        s += &format!("    p = P(F(df1, df2) ≥ F) = {}\n", g6(r.p));
        s += &format!("  conclusion: {concl}\n");
        it.out(&s);
    }
    let tbl = Table {
        names: ["Source", "SS", "df", "MS", "F", "p"].iter().map(|s| s.to_string()).collect(),
        columns: vec![
            Value::Str(Rc::new(StrArr::col(vec![Some("Groups".into()), Some("Error".into()), Some("Total".into())]))),
            Value::Mat(Mat::col(vec![r.ssb, r.ssw, r.ssb + r.ssw])),
            Value::Mat(Mat::col(vec![r.df1, r.df2, r.df1 + r.df2])),
            Value::Mat(Mat::col(vec![r.ssb / r.df1, r.ssw / r.df2, f64::NAN])),
            Value::Mat(Mat::col(vec![r.f, f64::NAN, f64::NAN])),
            Value::Mat(Mat::col(vec![r.p, f64::NAN, f64::NAN])),
        ],
        kinds: vec![ColKind::Text, ColKind::Num, ColKind::Num, ColKind::Num, ColKind::Num, ColKind::Num],
    };
    Ok(vec![Value::num(r.p), Value::Table(Rc::new(tbl))])
}

// ---------- correlation ----------

fn corr(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let x = mat_arg(a, 0, "corr")?;
    let (y, k) = match a.get(1).filter(|v| !is_str(v)) {
        Some(Value::Mat(y)) => (y.clone(), 2),
        _ => (x.clone(), 1),
    };
    let o = test_opts(&opts(a, k, "corr")?, "corr")?;
    if x.rows != y.rows {
        return Err(MError::new("corr: X and Y must have the same number of rows"));
    }
    let xc: Vec<Vec<f64>> = if x.rows == 1 { vec![x.re.clone()] } else { cols_of(x) };
    let yc: Vec<Vec<f64>> = if y.rows == 1 { vec![y.re.clone()] } else { cols_of(&y) };
    let n = xc[0].len() as f64;
    let fault = fault_is(it, "corr");
    let mut rho = Mat::zeros(xc.len(), yc.len());
    let mut pv = Mat::zeros(xc.len(), yc.len());
    let mut last = (0.0, 0.0, 0.0);
    for (i, xi) in xc.iter().enumerate() {
        for (j, yj) in yc.iter().enumerate() {
            let c = cov_matrix(&[xi.clone(), yj.clone()]);
            let r = c.re[2] / (c.re[0] * c.re[3]).sqrt();
            let (t, p) = corr_p(r, n, fault);
            rho.re[j * xc.len() + i] = r;
            pv.re[j * xc.len() + i] = p;
            last = (r, t, p);
        }
    }
    if o.display && xc.len() == 1 && yc.len() == 1 {
        let (r, t, p) = last;
        let (_, concl) = verdict(p, o.alpha);
        let mut s = "corr: Pearson correlation, test of H0: ρ = 0\n".to_string();
        s += &format!("  H0: ρ = 0; H1: ρ ≠ 0 (two-sided), α = {}\n", g6(o.alpha));
        s += "  assumptions:\n";
        s += "    independence of (x, y) pairs — not checked from the data\n";
        s += "    bivariate normality (for the exact p); linear relationship\n";
        s += "  steps:\n";
        s += &format!("    r = Σ(x − x̄)(y − ȳ)/√(Σ(x − x̄)²·Σ(y − ȳ)²) = {}, n = {}\n", g6(r), g6(n));
        s += &format!("    t = r·√((n − 2)/(1 − r²)) = {}, df = n − 2 = {}\n", g6(t), g6(n - 2.0));
        s += &format!("    p = 2·P(T ≥ |t|) = {}\n", g6(p));
        s += &format!("  conclusion: {concl}\n");
        it.out(&s);
    }
    Ok(vec![Value::Mat(rho), Value::Mat(pv)])
}

// ---------- OLS regression ----------

pub struct Ols {
    pub beta: Vec<f64>,
    pub se: Vec<f64>,
    pub t: Vec<f64>,
    pub p: Vec<f64>,
    pub sse: f64,
    pub sst: f64,
    pub r2: f64,
    pub adj: f64,
    pub rmse: f64,
    pub f: f64,
    pub pf: f64,
    pub dfe: f64,
    pub n: f64,
}

/// OLS via Householder QR. `x` — design columns (each of length n). `intercept` — whether there is a column of ones
/// (for R² and F relative to the constant model).
pub fn ols(x: &[Vec<f64>], y: &[f64], intercept: bool, fault: bool) -> Result<Ols, MError> {
    let n = y.len();
    let p = x.len();
    if n <= p {
        return Err(MError::new(format!("fitlm: need more observations ({n}) than coefficients ({p})")));
    }
    let mut a: Vec<Vec<f64>> = x.to_vec();
    let mut b = y.to_vec();
    let mut rdiag_max: f64 = 0.0;
    for k in 0..p {
        let norm = (k..n).map(|i| a[k][i] * a[k][i]).sum::<f64>().sqrt();
        rdiag_max = rdiag_max.max(norm);
        if norm <= 1e-12 * rdiag_max.max(1e-300) {
            return Err(MError::new("fitlm: design matrix is rank deficient"));
        }
        let alpha = if a[k][k] > 0.0 { -norm } else { norm };
        let mut v: Vec<f64> = (k..n).map(|i| a[k][i]).collect();
        v[0] -= alpha;
        let vv: f64 = v.iter().map(|z| z * z).sum();
        for col in a.iter_mut().skip(k) {
            let s: f64 = (k..n).map(|i| v[i - k] * col[i]).sum::<f64>() * 2.0 / vv;
            for i in k..n {
                col[i] -= s * v[i - k];
            }
        }
        let s: f64 = (k..n).map(|i| v[i - k] * b[i]).sum::<f64>() * 2.0 / vv;
        for i in k..n {
            b[i] -= s * v[i - k];
        }
    }
    // R — upper triangle: R[i][j] = a[j][i], i ≤ j
    let mut beta = vec![0.0; p];
    for i in (0..p).rev() {
        let s: f64 = (i + 1..p).map(|j| a[j][i] * beta[j]).sum();
        beta[i] = (b[i] - s) / a[i][i];
    }
    // R⁻¹ (upper triangle), diag((XᵀX)⁻¹) = Σ_k R⁻¹[j][k]²
    let mut rinv = vec![vec![0.0; p]; p];
    for j in 0..p {
        rinv[j][j] = 1.0 / a[j][j];
        for i in (0..j).rev() {
            let s: f64 = (i + 1..=j).map(|k| a[k][i] * rinv[k][j]).sum();
            rinv[i][j] = -s / a[i][i];
        }
    }
    let fitted: Vec<f64> = (0..n).map(|i| (0..p).map(|j| x[j][i] * beta[j]).sum()).collect();
    let sse: f64 = (0..n).map(|i| (y[i] - fitted[i]).powi(2)).sum();
    let ym = y.iter().sum::<f64>() / n as f64;
    let sst: f64 = if intercept { y.iter().map(|v| (v - ym).powi(2)).sum() } else { y.iter().map(|v| v * v).sum() };
    let dfe = (n - p) as f64;
    let s2 = sse / dfe;
    let se: Vec<f64> = (0..p).map(|j| (s2 * (j..p).map(|k| rinv[j][k] * rinv[j][k]).sum::<f64>()).sqrt()).collect();
    let t: Vec<f64> = beta.iter().zip(&se).map(|(b, s)| b / s).collect();
    let pv: Vec<f64> = t.iter().map(|&tt| t_p(tt, dfe, Tail::Both, fault)).collect();
    let dfm = if intercept { p as f64 - 1.0 } else { p as f64 };
    let r2 = 1.0 - sse / sst;
    let adj = 1.0 - (1.0 - r2) * (n as f64 - if intercept { 1.0 } else { 0.0 }) / dfe;
    let f = ((sst - sse) / dfm) / s2;
    let pf = if dfm > 0.0 { FisherSnedecor::new(dfm, dfe).map_or(f64::NAN, |d| d.sf(f)) } else { f64::NAN };
    Ok(Ols { beta, se, t, p: pv, sse, sst, r2, adj, rmse: s2.sqrt(), f, pf, dfe, n: n as f64 })
}

fn fitlm(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let (mut cols, mut names, y, yname, oidx, mut intercept): (Vec<Vec<f64>>, Vec<String>, Vec<f64>, String, usize, bool);
    match arg(a, 0, "fitlm")? {
        Value::Table(t) => {
            let formula = a.get(1).filter(|v| is_str(v)).and_then(str_of).filter(|s| s.contains('~'));
            let (resp, terms, icpt) = match &formula {
                Some(f) => parse_formula(f)?,
                None => {
                    let last = t.names.last().cloned().ok_or_else(|| MError::new("fitlm: empty table"))?;
                    (last.clone(), t.names.iter().filter(|n| **n != last).cloned().collect(), true)
                }
            };
            let col = |n: &str| -> Result<Vec<f64>, MError> {
                let k = tb::var_index(t, n)?;
                tb::num_col(t, k).map(|m| m.re.clone()).ok_or_else(|| MError::new(format!("fitlm: variable '{n}' is not numeric")))
            };
            y = col(&resp)?;
            cols = terms.iter().map(|n| col(n)).collect::<Result<_, _>>()?;
            names = terms;
            yname = resp;
            intercept = icpt;
            oidx = if formula.is_some() { 2 } else { 1 };
        }
        Value::Mat(xm) => {
            let ym = mat_arg(a, 1, "fitlm")?;
            if xm.rows != ym.numel() && !(xm.rows == 1 && xm.cols == ym.numel()) {
                return Err(MError::new("fitlm: X and y must have the same number of rows"));
            }
            let xm = if xm.rows == 1 && ym.numel() > 1 { xm.transpose() } else { xm.clone() };
            cols = cols_of(&xm);
            if xm.cols == 1 {
                cols = vec![xm.re.clone()];
            }
            names = (1..=cols.len()).map(|k| format!("x{k}")).collect();
            y = ym.re.clone();
            yname = "y".into();
            intercept = true;
            oidx = 2;
        }
        other => return Err(MError::new(format!("fitlm: wrong type argument '{}'", other.class_name()))),
    }
    let o = opts(a, oidx, "fitlm")?;
    if let Some(v) = opt(&o, "intercept") {
        intercept = matches!(v, Value::Mat(m) if m.numel() == 1 && m.re[0] != 0.0);
    }
    let display = !matches!(opt(&o, "display").and_then(str_of).map(|s| s.to_ascii_lowercase()).as_deref(), Some("off"));
    // rows with missing values are dropped (as in MATLAB)
    let n0 = y.len();
    let keep: Vec<usize> = (0..n0).filter(|&i| !y[i].is_nan() && cols.iter().all(|c| !c[i].is_nan())).collect();
    let y: Vec<f64> = keep.iter().map(|&i| y[i]).collect();
    for c in cols.iter_mut() {
        *c = keep.iter().map(|&i| c[i]).collect();
    }
    if intercept {
        cols.insert(0, vec![1.0; y.len()]);
        names.insert(0, "(Intercept)".into());
    }
    let r = ols(&cols, &y, intercept, fault_is(it, "fitlm"))?;
    // gate: check β against X \ y (the interpreter's solution, with its own gate)
    let mut beta = r.beta.clone();
    if it.fault.as_deref() == Some("gate:fitlm") {
        beta[0] += 1.0;
    }
    let n = y.len();
    let xm = Mat::new(n, cols.len(), cols.iter().flatten().copied().collect());
    let bslash = super::linalg::mldivide(it, &xm, &Mat::col(y.clone()))?;
    let nb = beta.iter().map(|b| b * b).sum::<f64>().sqrt().max(1e-300);
    let diff = beta.iter().zip(&bslash.re).map(|(a, b)| (a - b) * (a - b)).sum::<f64>().sqrt() / nb;
    let tol = 1e-8;
    if !(diff <= tol) {
        it.gate_warn(&format!("gate fitlm: |b - X\\y| / |b| = {diff:.1e} exceeds {tol:.1e}"));
    }
    if display {
        let terms: Vec<String> = names.iter().map(|n| if n == "(Intercept)" { "1".to_string() } else { n.clone() }).collect();
        let mut s = format!("fitlm: OLS linear regression, {yname} ~ {}\n", terms.join(" + "));
        s += "  assumptions:\n";
        s += &format!("    linear relationship of {yname} with the predictors\n");
        s += "    independence of observations — not checked from the data\n";
        s += "    homoscedasticity: constant residual variance\n";
        s += "    normality of residuals — for t and F tests at small n\n";
        s += "  steps:\n";
        s += "    β: OLS via QR (Rβ = Qᵀy), checked against X\\y\n";
        s += &format!("    s² = SSE/(n − p) = {}, SE = √diag(s²(XᵀX)⁻¹)\n", g6(r.sse / r.dfe));
        s += &format!("    t = β/SE, p = 2·P(T ≥ |t|), df = n − p = {}\n", g6(r.dfe));
        let w = names.iter().map(|n| n.chars().count()).max().unwrap_or(0).max(11);
        s += &format!("  coefficients:{}{:>12}{:>12}{:>12}{:>12}\n", " ".repeat(w + 4 - 14 + 2), "Estimate", "SE", "tStat", "pValue");
        for (k, name) in names.iter().enumerate() {
            s += &format!("    {name:<w$}  {:>12}{:>12}{:>12}{:>12}\n", g6(r.beta[k]), g6(r.se[k]), g6(r.t[k]), g6(r.p[k]));
        }
        s += &format!("  n = {n}, R² = {}, R²adj = {}, RMSE = {}, F = {} (p = {})\n", g6(r.r2), g6(r.adj), g6(r.rmse), g6(r.f), g6(r.pf));
        it.out(&s);
    }
    let coef = Table {
        names: ["Name", "Estimate", "SE", "tStat", "pValue"].iter().map(|s| s.to_string()).collect(),
        columns: vec![
            Value::Str(Rc::new(StrArr::col(names.iter().map(|n| Some(n.clone())).collect()))),
            Value::Mat(Mat::col(r.beta.clone())),
            Value::Mat(Mat::col(r.se.clone())),
            Value::Mat(Mat::col(r.t.clone())),
            Value::Mat(Mat::col(r.p.clone())),
        ],
        kinds: vec![ColKind::Text, ColKind::Num, ColKind::Num, ColKind::Num, ColKind::Num],
    };
    let summary = stats_row(&["N", "DFE", "R2", "AdjR2", "RMSE", "F", "pF", "SSE"], &[r.n, r.dfe, r.r2, r.adj, r.rmse, r.f, r.pf, r.sse]);
    Ok(vec![Value::Table(Rc::new(coef)), summary])
}

/// «y ~ a + b», «y ~ 1 + a», «y ~ a - 1» / «+ 0» (no intercept).
fn parse_formula(f: &str) -> Result<(String, Vec<String>, bool), MError> {
    let (lhs, rhs) = f.split_once('~').ok_or_else(|| MError::new("fitlm: formula must look like 'y ~ x1 + x2'"))?;
    let resp = lhs.trim().to_string();
    let mut intercept = true;
    let mut terms = Vec::new();
    let rhs = rhs.replace('-', "+-");
    for t in rhs.split('+').map(str::trim).filter(|t| !t.is_empty()) {
        match t.replace(' ', "").as_str() {
            "1" => intercept = true,
            "-1" | "0" => intercept = false,
            s if s.starts_with('-') => return Err(MError::new(format!("fitlm: cannot remove term '{s}'"))),
            s if tb::valid_name(s) => terms.push(s.to_string()),
            s => return Err(MError::new(format!("fitlm: unsupported term '{s}' (only 'y ~ a + b', '- 1')"))),
        }
    }
    if !tb::valid_name(&resp) {
        return Err(MError::new(format!("fitlm: bad response name '{resp}'")));
    }
    Ok((resp, terms, intercept))
}

#[allow(dead_code)]
fn is_class(v: &Value, c: Class) -> bool {
    matches!(v, Value::Mat(m) if m.class == c)
}
