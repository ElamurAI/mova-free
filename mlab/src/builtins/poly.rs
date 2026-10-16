//! Polynomials: polyval polyfit roots conv deconv poly polyder polyint. Coefficients — from the highest power.

use super::*;
use crate::ops::cmul;

pub fn register(r: &mut Registry) {
    r.register("polyval", |_, a, _| {
        let p = mat_arg(a, 0, "polyval")?;
        let x = mat_arg(a, 1, "polyval")?;
        if p.is_complex() || x.is_complex() {
            let (pr, pi) = (p.re.clone(), p.im_or_zeros());
            let xi = x.im_or_zeros();
            let (re, im): (Vec<f64>, Vec<f64>) = (0..x.numel())
                .map(|k| {
                    let z = (x.re[k], xi[k]);
                    let mut acc = (0.0, 0.0);
                    for t in 0..pr.len() {
                        acc = cmul(acc, z);
                        acc = (acc.0 + pr[t], acc.1 + pi[t]);
                    }
                    acc
                })
                .unzip();
            return one(Mat::complex(x.rows, x.cols, re, im));
        }
        let out: Vec<f64> = x.re.iter().map(|&xv| p.re.iter().fold(0.0, |acc, &c| acc * xv + c)).collect();
        one(Mat::new(x.rows, x.cols, out))
    });
    r.register("polyfit", |_, a, nargout| {
        let x = mat_arg(a, 0, "polyfit")?;
        let y = mat_arg(a, 1, "polyfit")?;
        let n = scalar_arg(a, 2, "polyfit")?;
        if nargout > 1 {
            return Err(MError::new("polyfit: the S and MU outputs need structs, which are not supported in mlab v1"));
        }
        if x.numel() != y.numel() {
            return Err(MError::new("polyfit: X and Y must be vectors of the same size"));
        }
        if !n.is_finite() || n < 0.0 || n != n.trunc() {
            return Err(MError::new("polyfit: N must be a non-negative integer"));
        }
        let n = n as usize;
        let m = x.numel();
        // Vandermonde matrix m×(n+1)
        let mut v = Mat::zeros(m, n + 1);
        for i in 0..m {
            for j in 0..=n {
                v.re[j * m + i] = x.re[i].powi((n - j) as i32);
            }
        }
        let p = super::linalg::lstsq_svd(&v, &Mat::col(y.re.clone()))?;
        one(Mat::row(p.re))
    });
    r.register("roots", |it, a, _| {
        let p = mat_arg(a, 0, "roots")?;
        if p.is_complex() {
            return Err(MError::new("roots: complex coefficients are not supported in mlab v1"));
        }
        let mut c: Vec<f64> = p.re.clone();
        if c.iter().any(|x| !x.is_finite()) {
            return Err(MError::new("roots: inputs must not contain Inf or NaN"));
        }
        while c.first() == Some(&0.0) {
            c.remove(0);
        }
        let mut zeros = 0;
        while c.len() > 1 && c.last() == Some(&0.0) {
            c.pop();
            zeros += 1;
        }
        let n = c.len().saturating_sub(1);
        let mut re = Vec::new();
        let mut im = Vec::new();
        if n >= 1 {
            // companion matrix
            let mut comp = Mat::zeros(n, n);
            for j in 0..n {
                comp.re[j * n] = -c[j + 1] / c[0];
            }
            for i in 1..n {
                comp.re[(i - 1) * n + i] = 1.0;
            }
            let ev = it.call_function("eig", vec![Value::Mat(comp)], 1).map_err(|f| match f {
                crate::interp::Flow::Err(e) => e,
                _ => MError::new("roots: eig failed"),
            })?;
            if let Some(Value::Mat(e)) = ev.first() {
                re.extend_from_slice(&e.re);
                im.extend(e.im_or_zeros());
            }
        }
        for _ in 0..zeros {
            re.push(0.0);
            im.push(0.0);
        }
        let k = re.len();
        if k == 0 {
            return one(Mat::zeros(0, 0));
        }
        one(Mat::complex(k, 1, re, im))
    });
    r.register("conv", |_, a, _| {
        let x = mat_arg(a, 0, "conv")?;
        let y = mat_arg(a, 1, "conv")?;
        if x.is_empty() || y.is_empty() {
            return Err(MError::new("conv: both arguments A and B must be vectors"));
        }
        let complex = x.is_complex() || y.is_complex();
        let (xi, yi) = (x.im_or_zeros(), y.im_or_zeros());
        let n = x.numel() + y.numel() - 1;
        let mut re = vec![0.0; n];
        let mut im = vec![0.0; n];
        for i in 0..x.numel() {
            for j in 0..y.numel() {
                let t = cmul((x.re[i], xi[i]), (y.re[j], yi[j]));
                re[i + j] += t.0;
                im[i + j] += t.1;
            }
        }
        let column = x.cols == 1 && x.rows > 1;
        let (r, c) = if column { (n, 1) } else { (1, n) };
        one(if complex { Mat::complex(r, c, re, im) } else { Mat::new(r, c, re) })
    });
    r.register("deconv", |_, a, nargout| {
        let y = mat_arg(a, 0, "deconv")?;
        let d = mat_arg(a, 1, "deconv")?;
        if d.is_empty() || d.re[0] == 0.0 {
            return Err(MError::new("deconv: divisor cannot be zero"));
        }
        let (ly, ld) = (y.numel(), d.numel());
        if ly < ld {
            let mut out = vec![Value::num(0.0)];
            if nargout > 1 {
                out.push(Value::Mat(y.clone()));
            }
            return Ok(out);
        }
        let nq = ly - ld + 1;
        let mut rem = y.re.clone();
        let mut q = vec![0.0; nq];
        for k in 0..nq {
            let coef = rem[k] / d.re[0];
            q[k] = coef;
            for j in 0..ld {
                rem[k + j] -= coef * d.re[j];
            }
            rem[k] = 0.0;
        }
        let column = y.cols == 1 && y.rows > 1;
        let mk = |v: Vec<f64>| if column { Mat::col(v) } else { Mat::row(v) };
        let mut out = vec![Value::Mat(mk(q))];
        if nargout > 1 {
            out.push(Value::Mat(mk(rem)));
        }
        Ok(out)
    });
    r.register("poly", |it, a, _| {
        let m = mat_arg(a, 0, "poly")?;
        let roots: Mat = if m.rows == m.cols && m.rows > 1 {
            match it.call_function("eig", vec![Value::Mat(m.clone())], 1) {
                Ok(v) => match v.into_iter().next() {
                    Some(Value::Mat(e)) => e,
                    _ => return Err(MError::new("poly: eig failed")),
                },
                Err(crate::interp::Flow::Err(e)) => return Err(e),
                Err(_) => return Err(MError::new("poly: eig failed")),
            }
        } else {
            m.clone()
        };
        let ri = roots.im_or_zeros();
        let mut c: Vec<(f64, f64)> = vec![(1.0, 0.0)];
        for k in 0..roots.numel() {
            let z = (roots.re[k], ri[k]);
            let mut next = vec![(0.0, 0.0); c.len() + 1];
            for (t, &ct) in c.iter().enumerate() {
                next[t] = (next[t].0 + ct.0, next[t].1 + ct.1);
                let p = cmul(ct, z);
                next[t + 1] = (next[t + 1].0 - p.0, next[t + 1].1 - p.1);
            }
            c = next;
        }
        let maxre = c.iter().fold(0.0f64, |acc, z| acc.max(z.0.abs())).max(1.0);
        let maxim = c.iter().fold(0.0f64, |acc, z| acc.max(z.1.abs()));
        let n = c.len();
        if maxim <= 1e-10 * maxre {
            return one(Mat::row(c.iter().map(|z| z.0).collect()));
        }
        one(Mat::complex(1, n, c.iter().map(|z| z.0).collect(), c.iter().map(|z| z.1).collect()))
    });
    r.register("polyder", |_, a, _| {
        let p0 = mat_arg(a, 0, "polyder")?;
        // polyder(a, b) — derivative of the product conv(a, b) (documented form; vmm-poly-018, tests3)
        let pv: Vec<f64> = match a.get(1) {
            Some(_) => {
                let q = mat_arg(a, 1, "polyder")?;
                let mut c = vec![0.0; (p0.numel() + q.numel()).max(1) - 1];
                for (i, x) in p0.re.iter().enumerate() {
                    for (j, y) in q.re.iter().enumerate() {
                        c[i + j] += x * y;
                    }
                }
                c
            }
            None => p0.re.clone(),
        };
        let n = pv.len();
        if n <= 1 {
            return num(0.0);
        }
        let out: Vec<f64> = (0..n - 1).map(|k| pv[k] * (n - 1 - k) as f64).collect();
        one(Mat::row(out))
    });
    r.register("polyint", |_, a, _| {
        let p = mat_arg(a, 0, "polyint")?;
        let k = if a.len() > 1 { scalar_arg(a, 1, "polyint")? } else { 0.0 };
        let n = p.numel();
        let mut out: Vec<f64> = (0..n).map(|t| p.re[t] / (n - t) as f64).collect();
        out.push(k);
        one(Mat::row(out))
    });
}

