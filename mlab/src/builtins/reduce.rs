//! Reductions along a dimension: sum prod cumsum cumprod max min mean median std var sort find any all unique diff nnz.

use super::*;
use crate::ops;

pub fn register(r: &mut Registry) {
    r.register("sum", |_, a, _| {
        let m = mat_arg(a, 0, "sum")?;
        reduce_c(m, dim_arg(a, 1, "sum")?, 0.0, |acc, x| (acc.0 + x.0, acc.1 + x.1))
    });
    r.register("prod", |_, a, _| {
        let m = mat_arg(a, 0, "prod")?;
        reduce_c(m, dim_arg(a, 1, "prod")?, 1.0, ops::cmul)
    });
    r.register("mean", |_, a, _| {
        let m = mat_arg(a, 0, "mean")?;
        let d = dim_arg(a, 1, "mean")?;
        if m.rows == 0 && m.cols == 0 && d.is_none() {
            return num(f64::NAN);
        }
        let dim = d.unwrap_or(default_dim(m));
        let n = if dim == 1 { m.rows } else if dim == 2 { m.cols } else { 1 } as f64;
        let s = reduce_c(m, Some(dim), 0.0, |acc, x| (acc.0 + x.0, acc.1 + x.1))?;
        match s.into_iter().next() {
            Some(Value::Mat(mut s)) => {
                for x in s.re.iter_mut() {
                    *x /= n;
                }
                if let Some(im) = &mut s.im {
                    for x in im.iter_mut() {
                        *x /= n;
                    }
                }
                one(s)
            }
            _ => unreachable!(),
        }
    });
    r.register("cumsum", |_, a, _| {
        let m = mat_arg(a, 0, "cumsum")?;
        cumulative(m, dim_arg(a, 1, "cumsum")?.unwrap_or(default_dim(m)), |acc, x| acc + x, 0.0)
    });
    r.register("cumprod", |_, a, _| {
        let m = mat_arg(a, 0, "cumprod")?;
        cumulative(m, dim_arg(a, 1, "cumprod")?.unwrap_or(default_dim(m)), |acc, x| acc * x, 1.0)
    });
    r.register("max", |_, a, n| minmax(a, n, true));
    r.register("min", |_, a, n| minmax(a, n, false));
    r.register("median", |_, a, _| {
        let m = mat_arg(a, 0, "median")?;
        if m.rows == 0 && m.cols == 0 {
            return num(f64::NAN);
        }
        reduce_slices(m, dim_arg(a, 1, "median")?, |v| {
            if v.is_empty() || v.iter().any(|x| x.is_nan()) {
                return f64::NAN;
            }
            let mut s = v.to_vec();
            s.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let n = s.len();
            if n % 2 == 1 { s[n / 2] } else { (s[n / 2 - 1] + s[n / 2]) / 2.0 }
        })
    });
    r.register("var", |_, a, _| var_std(a, "var", false));
    r.register("std", |_, a, _| var_std(a, "std", true));
    r.register("any", |_, a, _| {
        let m = mat_arg(a, 0, "any")?;
        if m.rows == 0 && m.cols == 0 {
            return boolv(false);
        }
        let mut out = reduce_slices_m(m, dim_arg(a, 1, "any")?, |v| if v.iter().any(|&x| x != 0.0 && !x.is_nan()) { 1.0 } else { 0.0 })?;
        out.class = Class::Logical;
        one(out)
    });
    r.register("all", |_, a, _| {
        let m = mat_arg(a, 0, "all")?;
        if m.rows == 0 && m.cols == 0 {
            return boolv(true);
        }
        let mut out = reduce_slices_m(m, dim_arg(a, 1, "all")?, |v| if v.iter().all(|&x| x != 0.0) { 1.0 } else { 0.0 })?;
        out.class = Class::Logical;
        one(out)
    });
    r.register("nnz", |_, a, _| {
        let m = mat_arg(a, 0, "nnz")?;
        num((0..m.numel()).filter(|&k| m.re[k] != 0.0 || m.im_at(k) != 0.0).count() as f64)
    });
    r.register("sort", sort);
    r.register("find", find);
    r.register("unique", unique);
    r.register("diff", |_, a, _| {
        let mut m = mat_arg(a, 0, "diff")?.clone();
        let order = if a.len() > 1 { scalar_arg(a, 1, "diff")? as usize } else { 1 };
        let dim = dim_arg(a, 2, "diff")?.unwrap_or(default_dim(&m));
        for _ in 0..order {
            m = diff1(&m, dim);
        }
        one(m)
    });
}

/// Slice along a dimension: for dim=1 — columns, for dim=2 — rows.
fn slices(m: &Mat, dim: usize) -> (usize, usize, Vec<Vec<usize>>) {
    match dim {
        1 => (1, m.cols, (0..m.cols).map(|j| (0..m.rows).map(|i| j * m.rows + i).collect()).collect()),
        2 => (m.rows, 1, (0..m.rows).map(|i| (0..m.cols).map(|j| j * m.rows + i).collect()).collect()),
        _ => (m.rows, m.cols, (0..m.numel()).map(|k| vec![k]).collect()),
    }
}

fn reduce_c(m: &Mat, dim: Option<usize>, init: f64, f: impl Fn((f64, f64), (f64, f64)) -> (f64, f64)) -> Result<Vec<Value>, MError> {
    if m.rows == 0 && m.cols == 0 && dim.is_none() {
        return num(init);
    }
    let dim = dim.unwrap_or(default_dim(m));
    let (r, c, sl) = slices(m, dim);
    let mut re = Vec::with_capacity(sl.len());
    let mut im = Vec::with_capacity(sl.len());
    for s in &sl {
        let mut acc = (init, 0.0);
        for &k in s {
            acc = f(acc, (m.re[k], m.im_at(k)));
        }
        re.push(acc.0);
        im.push(acc.1);
    }
    let out = if m.is_complex() { Mat::complex(r, c, re, im) } else { Mat::new(r, c, re) };
    one(out)
}

fn reduce_slices_m(m: &Mat, dim: Option<usize>, f: impl Fn(&[f64]) -> f64) -> Result<Mat, MError> {
    let dim = dim.unwrap_or(default_dim(m));
    let (r, c, sl) = slices(m, dim);
    let vals: Vec<f64> = sl.iter().map(|s| f(&s.iter().map(|&k| m.re[k]).collect::<Vec<_>>())).collect();
    Ok(Mat::new(r, c, vals))
}

fn reduce_slices(m: &Mat, dim: Option<usize>, f: impl Fn(&[f64]) -> f64) -> Result<Vec<Value>, MError> {
    one(reduce_slices_m(m, dim, f)?)
}

fn cumulative(m: &Mat, dim: usize, f: impl Fn(f64, f64) -> f64, init: f64) -> Result<Vec<Value>, MError> {
    let mut out = Mat::new(m.rows, m.cols, m.re.clone());
    let (_, _, sl) = slices(m, dim);
    for s in &sl {
        let mut acc = init;
        for &k in s {
            acc = f(acc, m.re[k]);
            out.re[k] = acc;
        }
    }
    one(out)
}

fn var_std(a: &[Value], name: &str, sqrt: bool) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, name)?;
    let w = match a.get(1) {
        Some(Value::Mat(x)) if !x.is_empty() => x.re[0],
        _ => 0.0,
    };
    if m.rows == 0 && m.cols == 0 {
        return num(f64::NAN);
    }
    reduce_slices(m, dim_arg(a, 2, name)?, |v| {
        let n = v.len();
        if n == 0 {
            return f64::NAN;
        }
        if n == 1 {
            return 0.0;
        }
        let mean = v.iter().sum::<f64>() / n as f64;
        let ss: f64 = v.iter().map(|x| (x - mean) * (x - mean)).sum();
        let d = if w == 1.0 { n as f64 } else { (n - 1) as f64 };
        let var = ss / d;
        if sqrt { var.sqrt() } else { var }
    })
}

fn better(x: f64, best: f64, is_max: bool) -> bool {
    if best.is_nan() {
        return !x.is_nan();
    }
    if is_max { x > best } else { x < best }
}

fn minmax(a: &[Value], nargout: usize, is_max: bool) -> Result<Vec<Value>, MError> {
    let name = if is_max { "max" } else { "min" };
    let m = mat_arg(a, 0, name)?;
    // max(A, B) — elementwise
    if a.len() >= 2 {
        if let Value::Mat(b) = &a[1] {
            if !b.is_empty() || a.len() == 2 {
                if a.len() == 2 {
                    let (r, c, v) = ops::map2(name, m, b, |x, y| {
                        if x.is_nan() {
                            y
                        } else if y.is_nan() {
                            x
                        } else if is_max {
                            x.max(y)
                        } else {
                            x.min(y)
                        }
                    })?;
                    return one(Mat::new(r, c, v));
                }
            }
        }
    }
    if m.rows == 0 && m.cols == 0 {
        return Ok(vec![Value::Mat(Mat::empty()), Value::Mat(Mat::empty())]);
    }
    let dim = dim_arg(a, 2, name)?.unwrap_or(default_dim(m));
    let (r, c, sl) = slices(m, dim);
    let key = |k: usize| -> f64 { if m.is_complex() { m.re[k].hypot(m.im_at(k)) } else { m.re[k] } };
    let mut vals = Vec::with_capacity(sl.len());
    let mut ims = Vec::with_capacity(sl.len());
    let mut idx = Vec::with_capacity(sl.len());
    for s in &sl {
        let mut best_k = s[0];
        let mut best = key(s[0]);
        let mut pos = 0;
        for (p, &k) in s.iter().enumerate().skip(1) {
            let x = key(k);
            if better(x, best, is_max) {
                best = x;
                best_k = k;
                pos = p;
            }
        }
        vals.push(m.re[best_k]);
        ims.push(m.im_at(best_k));
        idx.push((pos + 1) as f64);
    }
    let mut out = if m.is_complex() { Mat::complex(r, c, vals, ims) } else { Mat::new(r, c, vals) };
    if m.class == Class::Char || m.class == Class::Logical {
        out.class = Class::Double;
    }
    let mut res = vec![Value::Mat(out)];
    if nargout > 1 {
        res.push(Value::Mat(Mat::new(r, c, idx)));
    }
    Ok(res)
}

fn sort(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "sort")?;
    let mut descend = false;
    let mut dim = None;
    for v in &a[1..] {
        if let Some(s) = str_of(v) {
            descend = s.eq_ignore_ascii_case("descend");
        } else if let Value::Mat(d) = v {
            if !d.is_empty() {
                dim = Some(d.re[0] as usize);
            }
        }
    }
    let dim = dim.unwrap_or(default_dim(m));
    let mut out = m.clone();
    let mut idx = Mat::zeros(m.rows, m.cols);
    let (_, _, sl) = slices(m, dim);
    let key = |k: usize| -> f64 { if m.is_complex() { m.re[k].hypot(m.im_at(k)) } else { m.re[k] } };
    for s in &sl {
        let mut order: Vec<usize> = (0..s.len()).collect();
        order.sort_by(|&p, &q| {
            let (x, y) = (key(s[p]), key(s[q]));
            let o = match (x.is_nan(), y.is_nan()) {
                (true, true) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => x.partial_cmp(&y).unwrap(),
            };
            if descend {
                match (x.is_nan(), y.is_nan()) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => o.reverse(),
                }
            } else {
                o
            }
        });
        for (t, &p) in order.iter().enumerate() {
            out.re[s[t]] = m.re[s[p]];
            if let Some(im) = &mut out.im {
                im[s[t]] = m.im_at(s[p]);
            }
            idx.re[s[t]] = (p + 1) as f64;
        }
    }
    let mut res = vec![Value::Mat(out)];
    if nargout > 1 {
        res.push(Value::Mat(idx));
    }
    Ok(res)
}

fn find(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "find")?;
    let limit = if a.len() > 1 { Some(scalar_arg(a, 1, "find")? as usize) } else { None };
    let from_end = a.get(2).and_then(str_of).map_or(false, |s| s == "last");
    let mut ks: Vec<usize> = (0..m.numel()).filter(|&k| m.re[k] != 0.0 || m.im_at(k) != 0.0).collect();
    if let Some(n) = limit {
        if from_end {
            let start = ks.len().saturating_sub(n);
            ks = ks[start..].to_vec();
        } else {
            ks.truncate(n);
        }
    }
    let n = ks.len();
    let shape = |n: usize| -> (usize, usize) {
        if m.rows == 1 && m.cols != 1 {
            (1, n)
        } else if m.rows == 0 && m.cols == 0 {
            (0, 0)
        } else {
            (n, 1)
        }
    };
    let (r, c) = shape(n);
    if nargout <= 1 {
        return one(Mat::new(r, c, ks.iter().map(|&k| (k + 1) as f64).collect()));
    }
    let rows: Vec<f64> = ks.iter().map(|&k| (k % m.rows.max(1) + 1) as f64).collect();
    let cols: Vec<f64> = ks.iter().map(|&k| (k / m.rows.max(1) + 1) as f64).collect();
    let mut out = vec![Value::Mat(Mat::new(r, c, rows)), Value::Mat(Mat::new(r, c, cols))];
    if nargout > 2 {
        let mut v = Mat::new(r, c, ks.iter().map(|&k| m.re[k]).collect());
        v.class = m.class;
        out.push(Value::Mat(v));
    }
    Ok(out)
}

fn unique(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "unique")?;
    let n = m.numel();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&p, &q| {
        let (x, y) = (m.re[p], m.re[q]);
        match (x.is_nan(), y.is_nan()) {
            (true, true) => p.cmp(&q),
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            _ => x.partial_cmp(&y).unwrap().then(p.cmp(&q)),
        }
    });
    let mut vals = Vec::new();
    let mut first = Vec::new();
    let mut j_map = vec![0.0; n];
    for &k in &order {
        let x = m.re[k];
        let is_new = match vals.last() {
            None => true,
            Some(&last) => x != last || x.is_nan(),
        };
        if is_new {
            vals.push(x);
            first.push((k + 1) as f64);
        }
        j_map[k] = vals.len() as f64;
    }
    let cnt = vals.len();
    let row = m.rows == 1 && m.cols >= 1;
    let mk = |v: Vec<f64>| if row { Mat::row(v) } else { Mat::col(v) };
    let mut u = mk(vals);
    u.class = m.class;
    let _ = cnt;
    let mut out = vec![Value::Mat(u)];
    if nargout > 1 {
        out.push(Value::Mat(mk(first)));
    }
    if nargout > 2 {
        out.push(Value::Mat(mk(j_map)));
    }
    Ok(out)
}

fn diff1(m: &Mat, dim: usize) -> Mat {
    if dim == 1 {
        if m.rows == 0 {
            return m.clone();
        }
        let r = m.rows - 1;
        let mut out = Mat::zeros(r, m.cols);
        for j in 0..m.cols {
            for i in 0..r {
                out.re[j * r + i] = m.re[j * m.rows + i + 1] - m.re[j * m.rows + i];
            }
        }
        out
    } else {
        if m.cols == 0 {
            return m.clone();
        }
        let c = m.cols - 1;
        let mut out = Mat::zeros(m.rows, c);
        for j in 0..c {
            for i in 0..m.rows {
                out.re[j * m.rows + i] = m.re[(j + 1) * m.rows + i] - m.re[j * m.rows + i];
            }
        }
        out
    }
}
