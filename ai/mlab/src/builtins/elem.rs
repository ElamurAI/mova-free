//! Elementwise functions: abs sqrt exp log, trigonometry, rounding, mod/rem, number theory, complex parts.

use super::*;
use crate::ast::BinOp;
use crate::ops::{self, cexp, clog};

pub fn register(r: &mut Registry) {
    r.register("abs", |_, a, _| {
        let m = mat_arg(a, 0, "abs")?;
        if let Some(im) = &m.im {
            return one(Mat::new(m.rows, m.cols, m.re.iter().zip(im).map(|(x, y)| x.hypot(*y)).collect()));
        }
        one(map(m, f64::abs))
    });
    r.register("sqrt", |_, a, _| {
        let m = mat_arg(a, 0, "sqrt")?;
        if m.is_complex() || m.re.iter().any(|&x| x < 0.0) {
            return one(cmap(m, csqrt));
        }
        one(map(m, f64::sqrt))
    });
    r.register("exp", |_, a, _| {
        let m = mat_arg(a, 0, "exp")?;
        if m.is_complex() {
            return one(cmap(m, cexp));
        }
        one(map(m, f64::exp))
    });
    r.register("log", |_, a, _| {
        let m = mat_arg(a, 0, "log")?;
        if m.is_complex() || m.re.iter().any(|&x| x < 0.0) {
            return one(cmap(m, clog));
        }
        one(map(m, f64::ln))
    });
    r.register("log2", |_, a, _| {
        let m = mat_arg(a, 0, "log2")?;
        if m.is_complex() || m.re.iter().any(|&x| x < 0.0) {
            return one(cmap(m, |z| {
                let l = clog(z);
                (l.0 / std::f64::consts::LN_2, l.1 / std::f64::consts::LN_2)
            }));
        }
        one(map(m, f64::log2))
    });
    r.register("log10", |_, a, _| {
        let m = mat_arg(a, 0, "log10")?;
        if m.is_complex() || m.re.iter().any(|&x| x < 0.0) {
            return one(cmap(m, |z| {
                let l = clog(z);
                (l.0 / std::f64::consts::LN_10, l.1 / std::f64::consts::LN_10)
            }));
        }
        one(map(m, f64::log10))
    });
    r.register("log1p", |_, a, _| one(map(mat_arg(a, 0, "log1p")?, f64::ln_1p)));
    r.register("expm1", |_, a, _| one(map(mat_arg(a, 0, "expm1")?, f64::exp_m1)));
    r.register("sin", |_, a, _| {
        let m = mat_arg(a, 0, "sin")?;
        if m.is_complex() {
            return one(cmap(m, |(x, y)| (x.sin() * y.cosh(), x.cos() * y.sinh())));
        }
        one(map(m, f64::sin))
    });
    r.register("cos", |_, a, _| {
        let m = mat_arg(a, 0, "cos")?;
        if m.is_complex() {
            return one(cmap(m, |(x, y)| (x.cos() * y.cosh(), -x.sin() * y.sinh())));
        }
        one(map(m, f64::cos))
    });
    r.register("tan", |_, a, _| one(map(real_only(a, "tan")?, f64::tan)));
    r.register("asin", |_, a, _| one(map(real_only(a, "asin")?, f64::asin)));
    r.register("acos", |_, a, _| one(map(real_only(a, "acos")?, f64::acos)));
    r.register("atan", |_, a, _| one(map(real_only(a, "atan")?, f64::atan)));
    r.register("sinh", |_, a, _| one(map(real_only(a, "sinh")?, f64::sinh)));
    r.register("cosh", |_, a, _| one(map(real_only(a, "cosh")?, f64::cosh)));
    r.register("tanh", |_, a, _| one(map(real_only(a, "tanh")?, f64::tanh)));
    r.register("asinh", |_, a, _| one(map(real_only(a, "asinh")?, f64::asinh)));
    r.register("acosh", |_, a, _| one(map(real_only(a, "acosh")?, f64::acosh)));
    r.register("atanh", |_, a, _| one(map(real_only(a, "atanh")?, f64::atanh)));
    r.register("sec", |_, a, _| one(map(real_only(a, "sec")?, |x| 1.0 / x.cos())));
    r.register("csc", |_, a, _| one(map(real_only(a, "csc")?, |x| 1.0 / x.sin())));
    r.register("cot", |_, a, _| one(map(real_only(a, "cot")?, |x| 1.0 / x.tan())));
    r.register("deg2rad", |_, a, _| one(map(real_only(a, "deg2rad")?, f64::to_radians)));
    r.register("rad2deg", |_, a, _| one(map(real_only(a, "rad2deg")?, f64::to_degrees)));
    r.register("atan2", |_, a, _| bin(a, "atan2", f64::atan2));
    r.register("hypot", |_, a, _| bin(a, "hypot", f64::hypot));
    r.register("floor", |_, a, _| one(map_parts(mat_arg(a, 0, "floor")?, f64::floor)));
    r.register("ceil", |_, a, _| one(map_parts(mat_arg(a, 0, "ceil")?, f64::ceil)));
    r.register("round", |_, a, _| one(map_parts(mat_arg(a, 0, "round")?, f64::round)));
    r.register("fix", |_, a, _| one(map_parts(mat_arg(a, 0, "fix")?, f64::trunc)));
    r.register("mod", |_, a, _| {
        bin(a, "mod", |x, y| {
            if y == 0.0 {
                return x;
            }
            if !x.is_finite() || y.is_nan() {
                return f64::NAN;
            }
            if y.is_infinite() {
                return if x == 0.0 || x.signum() == y.signum() { x } else { y };
            }
            let r = x - (x / y).floor() * y;
            if r != 0.0 && (r - y).abs() < f64::EPSILON * y.abs() { 0.0 } else { r }
        })
    });
    r.register("rem", |_, a, _| {
        bin(a, "rem", |x, y| {
            if y == 0.0 {
                return x - y * (x / y).trunc();
            }
            x - (x / y).trunc() * y
        })
    });
    r.register("sign", |_, a, _| {
        let m = mat_arg(a, 0, "sign")?;
        if let Some(im) = &m.im {
            let (re, imv): (Vec<f64>, Vec<f64>) = m
                .re
                .iter()
                .zip(im)
                .map(|(&x, &y)| {
                    let r = x.hypot(y);
                    if r == 0.0 { (0.0, 0.0) } else { (x / r, y / r) }
                })
                .unzip();
            return one(Mat::complex(m.rows, m.cols, re, imv));
        }
        one(map(m, |x| if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else if x == 0.0 { 0.0 } else { f64::NAN }))
    });
    r.register("real", |_, a, _| {
        let m = mat_arg(a, 0, "real")?;
        one(Mat::new(m.rows, m.cols, m.re.clone()))
    });
    r.register("imag", |_, a, _| {
        let m = mat_arg(a, 0, "imag")?;
        one(Mat::new(m.rows, m.cols, m.im_or_zeros()))
    });
    r.register("conj", |_, a, _| {
        let m = mat_arg(a, 0, "conj")?;
        let mut out = m.clone();
        if let Some(im) = &mut out.im {
            for x in im.iter_mut() {
                *x = -*x;
            }
        }
        one(out)
    });
    for name in ["angle", "arg"] {
        r.register(name, |_, a, _| {
            let m = mat_arg(a, 0, "angle")?;
            let im = m.im_or_zeros();
            one(Mat::new(m.rows, m.cols, m.re.iter().zip(&im).map(|(x, y)| y.atan2(*x)).collect()))
        });
    }
    r.register("complex", |_, a, _| {
        let re = mat_arg(a, 0, "complex")?;
        let zero = Mat::scalar(0.0);
        let im = if a.len() > 1 { mat_arg(a, 1, "complex")? } else { &zero };
        let (r, c, rv) = ops::map2("complex", re, im, |x, _| x)?;
        let (_, _, iv) = ops::map2("complex", re, im, |_, y| y)?;
        // complex() does not narrow: complex(1, 0) is complex (isreal = 0), as in MATLAB (vmm-edge-027, tests3)
        let mut m = Mat::new(r, c, rv);
        m.im = Some(iv);
        one(m)
    });
    r.register("isnan", |_, a, _| pred(a, "isnan", |x, y| x.is_nan() || y.is_nan()));
    r.register("isinf", |_, a, _| pred(a, "isinf", |x, y| x.is_infinite() || y.is_infinite()));
    r.register("isfinite", |_, a, _| pred(a, "isfinite", |x, y| x.is_finite() && y.is_finite()));
    r.register("power", |_, a, _| Ok(vec![Value::Mat(ops::elementwise(BinOp::EPow, mat_arg(a, 0, "power")?, mat_arg(a, 1, "power")?)?)]));
    r.register("times", |_, a, _| Ok(vec![Value::Mat(ops::elementwise(BinOp::EMul, mat_arg(a, 0, "times")?, mat_arg(a, 1, "times")?)?)]));
    r.register("plus", |_, a, _| Ok(vec![Value::Mat(ops::elementwise(BinOp::Add, mat_arg(a, 0, "plus")?, mat_arg(a, 1, "plus")?)?)]));
    r.register("minus", |_, a, _| Ok(vec![Value::Mat(ops::elementwise(BinOp::Sub, mat_arg(a, 0, "minus")?, mat_arg(a, 1, "minus")?)?)]));
    r.register("rdivide", |_, a, _| Ok(vec![Value::Mat(ops::elementwise(BinOp::EDiv, mat_arg(a, 0, "rdivide")?, mat_arg(a, 1, "rdivide")?)?)]));
    r.register("mtimes", |_, a, _| Ok(vec![Value::Mat(ops::matmul(mat_arg(a, 0, "mtimes")?, mat_arg(a, 1, "mtimes")?)?)]));
    r.register("xor", |_, a, _| {
        let x = mat_arg(a, 0, "xor")?;
        let y = mat_arg(a, 1, "xor")?;
        let (r, c, v) = ops::map2("xor", x, y, |p, q| if (p != 0.0) != (q != 0.0) { 1.0 } else { 0.0 })?;
        one(Mat::new(r, c, v).with_class(Class::Logical))
    });
    r.register("not", |_, a, _| Ok(vec![ops::unary(crate::ast::UnOp::Not, arg(a, 0, "not")?)?]));
    r.register("gcd", |_, a, _| {
        bin(a, "gcd", |x, y| {
            let (mut p, mut q) = (x.abs(), y.abs());
            while q > 0.0 {
                let t = p % q;
                p = q;
                q = t;
            }
            p
        })
    });
    r.register("lcm", |_, a, _| {
        bin(a, "lcm", |x, y| {
            if x == 0.0 || y == 0.0 {
                return 0.0;
            }
            let (mut p, mut q) = (x.abs(), y.abs());
            while q > 0.0 {
                let t = p % q;
                p = q;
                q = t;
            }
            (x * y).abs() / p
        })
    });
    r.register("factorial", |_, a, _| {
        let m = mat_arg(a, 0, "factorial")?;
        if m.re.iter().any(|&x| x < 0.0 || x != x.trunc()) {
            return Err(MError::new("factorial: all N must be real non-negative integers"));
        }
        one(map(m, |x| if x > 170.0 { f64::INFINITY } else { (1..=x as u64).fold(1.0, |acc, k| acc * k as f64) }))
    });
    r.register("nchoosek", |_, a, _| {
        let v = mat_arg(a, 0, "nchoosek")?;
        let k = scalar_arg(a, 1, "nchoosek")?;
        if v.numel() > 1 {
            // nchoosek(v, k): all combinations of the elements of v as rows, in lexicographic order of positions
            // (documented form; vmm-red-030, tests3)
            let n = v.numel();
            if k < 0.0 || k != k.trunc() || k > n as f64 {
                return Err(MError::new("nchoosek: K must be an integer between 0 and numel(V)"));
            }
            let k = k as usize;
            let mut rows: Vec<Vec<f64>> = Vec::new();
            let mut idx: Vec<usize> = (0..k).collect();
            loop {
                rows.push(idx.iter().map(|&i| v.re[i]).collect());
                let Some(p) = (0..k).rev().find(|&p| idx[p] < n - k + p) else { break };
                idx[p] += 1;
                for q in p + 1..k {
                    idx[q] = idx[q - 1] + 1;
                }
            }
            let mut m = Mat::zeros(rows.len(), k);
            for (i, r) in rows.iter().enumerate() {
                for (j, x) in r.iter().enumerate() {
                    m.re[j * rows.len() + i] = *x;
                }
            }
            return one(m);
        }
        let n = scalar_arg(a, 0, "nchoosek")?;
        if k < 0.0 || k > n || n != n.trunc() || k != k.trunc() {
            return Err(MError::new("nchoosek: K must be an integer between 0 and N"));
        }
        let k = k.min(n - k) as u64;
        let mut acc = 1.0f64;
        for t in 1..=k {
            acc = acc * (n - k as f64 + t as f64) / t as f64;
        }
        num(acc.round())
    });
    r.register("primes", |_, a, _| {
        let n = scalar_arg(a, 0, "primes")?.floor();
        if !n.is_finite() {
            // primes(Inf) / primes(NaN): was — a panic on memory allocation (Octave tests, tests3)
            return Err(MError::new("primes: N must be finite"));
        }
        if n < 2.0 {
            return one(Mat::new(1, 0, vec![]));
        }
        let n = n as usize;
        let mut sieve = vec![true; n + 1];
        sieve[0] = false;
        sieve[1] = false;
        let mut p = 2;
        while p * p <= n {
            if sieve[p] {
                let mut q = p * p;
                while q <= n {
                    sieve[q] = false;
                    q += p;
                }
            }
            p += 1;
        }
        one(Mat::row((0..=n).filter(|&k| sieve[k]).map(|k| k as f64).collect()))
    });
    r.register("isprime", |_, a, _| {
        let m = mat_arg(a, 0, "isprime")?;
        let out: Vec<f64> = m
            .re
            .iter()
            .map(|&x| {
                if x < 2.0 || x != x.trunc() {
                    return 0.0;
                }
                let n = x as u64;
                let mut d = 2u64;
                while d * d <= n {
                    if n % d == 0 {
                        return 0.0;
                    }
                    d += 1;
                }
                1.0
            })
            .collect();
        one(Mat::new(m.rows, m.cols, out).with_class(Class::Logical))
    });
    r.register("nthroot", |_, a, _| {
        let x = mat_arg(a, 0, "nthroot")?;
        let n = scalar_arg(a, 1, "nthroot")?;
        if n == 0.0 || n != n.trunc() {
            return Err(MError::new("nthroot: N must be a nonzero scalar integer"));
        }
        let odd = (n as i64) % 2 != 0;
        if !odd && x.re.iter().any(|&v| v < 0.0) {
            return Err(MError::new("nthroot: N must be an odd integer if X contains negative values"));
        }
        one(map(x, |v| {
            let (s, av) = if v < 0.0 { (-1.0, -v) } else { (1.0, v) };
            let mut y = if n == 3.0 { av.cbrt() } else { av.powf(1.0 / n) };
            if y.is_finite() && y > 0.0 {
                // one Newton correction
                let yn1 = y.powf(n - 1.0);
                y -= (y * yn1 - av) / (n * yn1);
            }
            s * y
        }))
    });
    r.register("gamma", |_, a, _| one(map(real_only(a, "gamma")?, gamma)));
}

fn real_only<'a>(a: &'a [Value], name: &str) -> Result<&'a Mat, MError> {
    let m = mat_arg(a, 0, name)?;
    if m.is_complex() {
        return Err(MError::new(format!("{name}: complex arguments are not supported in mlab v1")));
    }
    Ok(m)
}

pub fn map(m: &Mat, f: impl Fn(f64) -> f64) -> Mat {
    Mat::new(m.rows, m.cols, m.re.iter().map(|&x| f(x)).collect())
}

/// Rounding etc. — separately for the real and imaginary parts.
fn map_parts(m: &Mat, f: impl Fn(f64) -> f64) -> Mat {
    let mut out = map(m, &f);
    if let Some(im) = &m.im {
        out.im = Some(im.iter().map(|&x| f(x)).collect());
        out.narrow();
    }
    out
}

pub fn cmap(m: &Mat, f: impl Fn((f64, f64)) -> (f64, f64)) -> Mat {
    let im = m.im_or_zeros();
    let (re, iv): (Vec<f64>, Vec<f64>) = m.re.iter().zip(&im).map(|(&x, &y)| f((x, y))).unzip();
    Mat::complex(m.rows, m.cols, re, iv)
}

pub fn csqrt(z: (f64, f64)) -> (f64, f64) {
    let (x, y) = z;
    if y == 0.0 {
        return if x >= 0.0 { (x.sqrt(), 0.0) } else { (0.0, (-x).sqrt()) };
    }
    let r = x.hypot(y);
    let re = ((r + x) / 2.0).sqrt();
    let im = ((r - x) / 2.0).sqrt();
    (re, if y < 0.0 { -im } else { im })
}

fn bin(a: &[Value], name: &str, f: impl Fn(f64, f64) -> f64) -> Result<Vec<Value>, MError> {
    let x = mat_arg(a, 0, name)?;
    let y = mat_arg(a, 1, name)?;
    let (r, c, v) = ops::map2(name, x, y, f)?;
    one(Mat::new(r, c, v))
}

fn pred(a: &[Value], name: &str, f: impl Fn(f64, f64) -> bool) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, name)?;
    let out: Vec<f64> = (0..m.numel()).map(|k| if f(m.re[k], m.im_at(k)) { 1.0 } else { 0.0 }).collect();
    one(Mat::new(m.rows, m.cols, out).with_class(Class::Logical))
}

/// Γ(x). Zero and negative integers — Inf (as in MATLAB). The rest — `libm::tgamma` (musl port, MIT): our own Lanczos
/// (g = 7, 9 terms) on Boost.Math data lost 2–3 digits for x ~ 100 (cancellation in the Lanczos sum) and 6–7
/// digits near negative integers (sin(πx) without argument reduction); tests3, `tests/ext_boost.rs`.
pub fn gamma(x: f64) -> f64 {
    if x == x.trunc() && x <= 0.0 {
        return f64::INFINITY;
    }
    libm::tgamma(x)
}

