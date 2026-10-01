//! Numerical methods: integral (adaptive Gauss — Kronrod 7–15), fzero (Brent), fminsearch (Nelder — Mead),
//! ode45 (Dormand — Prince 5(4)), trapz. Algorithms — from published papers; constants — mathematical facts.

use super::*;

pub fn register(r: &mut Registry) {
    r.register("integral", integral);
    r.register("quad", integral);
    r.register("quadgk", integral);
    r.register("fzero", fzero);
    r.register("fminsearch", fminsearch);
    r.register("ode45", ode45);
    r.register("trapz", trapz);
}

fn call_scalar(it: &mut Interp, f: &Value, x: f64) -> Result<f64, MError> {
    match it.call1(f, vec![Value::num(x)])? {
        Value::Mat(m) if m.numel() >= 1 => Ok(m.re[0]),
        _ => Err(MError::new("function must return a numeric scalar")),
    }
}

/// Values of f at the nodes; vectorized first, otherwise pointwise.
fn eval_nodes(it: &mut Interp, f: &Value, xs: &[f64]) -> Result<Vec<f64>, MError> {
    let arg = Value::Mat(Mat::row(xs.to_vec()));
    if let Ok(Value::Mat(m)) = it.call1(f, vec![arg]) {
        if m.numel() == xs.len() && !m.is_complex() {
            return Ok(m.re);
        }
    }
    xs.iter().map(|&x| call_scalar(it, f, x)).collect()
}

const XGK: [f64; 8] = [
    0.991_455_371_120_812_639_206_854_697_526_329,
    0.949_107_912_342_758_524_526_189_684_047_851,
    0.864_864_423_359_769_072_789_712_788_640_926,
    0.741_531_185_599_394_439_863_864_773_280_788,
    0.586_087_235_467_691_130_294_144_845_693_013,
    0.405_845_151_377_397_166_906_606_412_076_961,
    0.207_784_955_007_898_467_600_689_403_773_245,
    0.0,
];
const WGK: [f64; 8] = [
    0.022_935_322_010_529_224_963_732_008_058_970,
    0.063_092_092_629_978_553_290_700_663_189_204,
    0.104_790_010_322_250_183_839_876_322_541_518,
    0.140_653_259_715_525_918_745_189_590_510_238,
    0.169_004_726_639_267_902_826_583_426_598_550,
    0.190_350_578_064_785_409_913_256_402_421_014,
    0.204_432_940_075_298_892_414_161_999_234_649,
    0.209_482_141_084_727_828_012_999_174_891_714,
];
const WG: [f64; 4] = [
    0.129_484_966_168_869_693_270_611_432_679_082,
    0.279_705_391_489_276_667_901_467_771_423_780,
    0.381_830_050_505_118_944_950_369_775_488_975,
    0.417_959_183_673_469_387_755_102_040_816_327,
];

#[derive(Clone, Copy)]
enum Map {
    /// finite [a, b]: x = (b−a)/4 · t(3 − t²) + (b+a)/2, t ∈ [−1, 1] — softens endpoint singularities
    /// (transformation from Shampine's paper on vectorized adaptive quadrature, 2008)
    Finite(f64, f64),
    /// [a, ∞): x = a + t/(1−t)
    Right(f64),
    /// (−∞, b]: x = b − t/(1−t)
    Left(f64),
    /// (−∞, ∞): x = t/(1−t²)
    Both,
}

fn map_point(m: Map, t: f64) -> (f64, f64) {
    match m {
        Map::Finite(a, b) => {
            let q = 0.25 * (b - a);
            (q * t * (3.0 - t * t) + 0.5 * (a + b), 3.0 * q * (1.0 - t * t))
        }
        Map::Right(a) => (a + t / (1.0 - t), 1.0 / ((1.0 - t) * (1.0 - t))),
        Map::Left(b) => (b - t / (1.0 - t), 1.0 / ((1.0 - t) * (1.0 - t))),
        Map::Both => {
            let d = 1.0 - t * t;
            (t / d, (1.0 + t * t) / (d * d))
        }
    }
}

/// One G7–K15 rule on [lo, hi] in the variable t: (integral, error estimate).
fn gk15(it: &mut Interp, f: &Value, m: Map, lo: f64, hi: f64) -> Result<(f64, f64), MError> {
    let c = 0.5 * (lo + hi);
    let h = 0.5 * (hi - lo);
    let mut ts = Vec::with_capacity(15);
    for j in 0..7 {
        ts.push(c - h * XGK[j]);
        ts.push(c + h * XGK[j]);
    }
    ts.push(c);
    let mut xs = Vec::with_capacity(15);
    let mut ws = Vec::with_capacity(15);
    for &t in &ts {
        let (x, w) = map_point(m, t);
        xs.push(x);
        ws.push(w);
    }
    let fx = eval_nodes(it, f, &xs)?;
    let g: Vec<f64> = fx.iter().zip(&ws).map(|(a, b)| a * b).collect();
    let mut k = WGK[7] * g[14];
    let mut gs = WG[3] * g[14];
    for j in 0..7 {
        let pair = g[2 * j] + g[2 * j + 1];
        k += WGK[j] * pair;
        if j % 2 == 1 {
            gs += WG[j / 2] * pair;
        }
    }
    let res = h * k;
    let err = (h * (k - gs)).abs();
    Ok((res, err))
}

fn integral(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let f = arg(a, 0, "integral")?.clone();
    let mut lo = scalar_arg(a, 1, "integral")?;
    let mut hi = scalar_arg(a, 2, "integral")?;
    let mut abstol = 1e-10;
    let mut reltol = 1e-6;
    let mut k = 3;
    while k + 1 < a.len() {
        let key = str_of(&a[k]).unwrap_or_default().to_lowercase();
        let v = scalar_arg(a, k + 1, "integral")?;
        match key.as_str() {
            "abstol" => abstol = v,
            "reltol" => reltol = v,
            _ => return Err(MError::new(format!("integral: unsupported option '{key}' in mlab v1"))),
        }
        k += 2;
    }
    if lo == hi {
        return num(0.0);
    }
    let mut sign = 1.0;
    if lo > hi {
        std::mem::swap(&mut lo, &mut hi);
        sign = -1.0;
    }
    let (m, t0, t1) = match (lo.is_infinite(), hi.is_infinite()) {
        (false, false) => (Map::Finite(lo, hi), -1.0, 1.0),
        (false, true) => (Map::Right(lo), 0.0, 1.0),
        (true, false) => (Map::Left(hi), 0.0, 1.0),
        (true, true) => (Map::Both, -1.0, 1.0),
    };
    // (−∞, b]: t from 0 to 1 maps to x from b to −∞, the integral from −∞ to b is positive
    let mut ivs: Vec<(f64, f64, f64, f64)> = Vec::new();
    let pieces = 10;
    for p in 0..pieces {
        let a0 = t0 + (t1 - t0) * p as f64 / pieces as f64;
        let b0 = t0 + (t1 - t0) * (p + 1) as f64 / pieces as f64;
        let (q, e) = gk15(it, &f, m, a0, b0)?;
        ivs.push((a0, b0, q, e));
    }
    let max_intervals = 650;
    loop {
        let q: f64 = ivs.iter().map(|v| v.2).sum();
        let e: f64 = ivs.iter().map(|v| v.3).sum();
        if !q.is_finite() {
            return num(sign * q);
        }
        if e <= abstol.max(reltol * q.abs()) {
            return num(sign * q);
        }
        if ivs.len() >= max_intervals {
            it.warn(&format!(
                "integral: maximum number of intervals reached; approximate bound on error is {:.1e}. The integral may not exist, or it may be difficult to approximate numerically to the requested accuracy.",
                e
            ));
            return num(sign * q);
        }
        let (idx, _) = ivs.iter().enumerate().max_by(|x, y| x.1.3.partial_cmp(&y.1.3).unwrap()).unwrap();
        let (a0, b0, _, _) = ivs.swap_remove(idx);
        let mid = 0.5 * (a0 + b0);
        let (q1, e1) = gk15(it, &f, m, a0, mid)?;
        let (q2, e2) = gk15(it, &f, m, mid, b0)?;
        ivs.push((a0, mid, q1, e1));
        ivs.push((mid, b0, q2, e2));
    }
}

fn sgn(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { 0.0 }
}

fn brent(it: &mut Interp, f: &Value, mut a: f64, mut b: f64, mut fa: f64, mut fb: f64, tolx: f64) -> Result<(f64, f64), MError> {
    let mut c = a;
    let mut fc = fa;
    let mut d = b - a;
    let mut e = d;
    for _ in 0..1000 {
        if fb * fc > 0.0 {
            c = a;
            fc = fa;
            d = b - a;
            e = d;
        }
        if fc.abs() < fb.abs() {
            a = b;
            b = c;
            c = a;
            fa = fb;
            fb = fc;
            fc = fa;
        }
        let tol1 = 2.0 * f64::EPSILON * b.abs() + 0.5 * tolx;
        let xm = 0.5 * (c - b);
        if xm.abs() <= tol1 || fb == 0.0 {
            return Ok((b, fb));
        }
        if e.abs() >= tol1 && fa.abs() > fb.abs() {
            let s = fb / fa;
            let (mut p, mut q);
            if a == c {
                p = 2.0 * xm * s;
                q = 1.0 - s;
            } else {
                let qq = fa / fc;
                let r = fb / fc;
                p = s * (2.0 * xm * qq * (qq - r) - (b - a) * (r - 1.0));
                q = (qq - 1.0) * (r - 1.0) * (s - 1.0);
            }
            if p > 0.0 {
                q = -q;
            } else {
                p = -p;
            }
            if 2.0 * p < (3.0 * xm * q - (tol1 * q).abs()).min((e * q).abs()) {
                e = d;
                d = p / q;
            } else {
                d = xm;
                e = d;
            }
        } else {
            d = xm;
            e = d;
        }
        a = b;
        fa = fb;
        b += if d.abs() > tol1 { d } else if xm >= 0.0 { tol1 } else { -tol1 };
        fb = call_scalar(it, f, b)?;
        if fb.is_nan() {
            return Err(MError::new("fzero: function returned NaN"));
        }
    }
    Ok((b, fb))
}

fn fzero(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let f = arg(a, 0, "fzero")?.clone();
    let x0 = mat_arg(a, 1, "fzero")?.clone();
    let tolx = f64::EPSILON;
    let (lo, hi, flo, fhi) = if x0.numel() >= 2 {
        let (lo, hi) = (x0.re[0], x0.re[1]);
        let flo = call_scalar(it, &f, lo)?;
        let fhi = call_scalar(it, &f, hi)?;
        if !(sgn(flo) * sgn(fhi) <= 0.0) {
            return Err(MError::new("fzero: not a valid initial bracketing"));
        }
        (lo, hi, flo, fhi)
    } else {
        let x = x0.re.first().copied().ok_or_else(|| invalid_call("fzero"))?;
        let fx = call_scalar(it, &f, x)?;
        if fx == 0.0 {
            let mut out = vec![Value::num(x)];
            if nargout > 1 {
                out.push(Value::num(0.0));
            }
            if nargout > 2 {
                out.push(Value::num(1.0));
            }
            return Ok(out);
        }
        let mut dx = if x == 0.0 { 1.0 / 50.0 } else { x.abs() / 50.0 };
        let mut found = None;
        for _ in 0..200 {
            dx *= std::f64::consts::SQRT_2;
            let (l, r) = (x - dx, x + dx);
            let fl = call_scalar(it, &f, l)?;
            let fr = call_scalar(it, &f, r)?;
            if !fl.is_finite() || !fr.is_finite() {
                break;
            }
            if sgn(fl) * sgn(fx) <= 0.0 {
                found = Some((l, x, fl, fx));
                break;
            }
            if sgn(fr) * sgn(fx) <= 0.0 {
                found = Some((x, r, fx, fr));
                break;
            }
        }
        found.ok_or_else(|| MError::new("fzero: unable to find a valid initial bracketing"))?
    };
    let (x, fx) = if flo == 0.0 {
        (lo, 0.0)
    } else if fhi == 0.0 {
        (hi, 0.0)
    } else {
        brent(it, &f, lo, hi, flo, fhi, tolx)?
    };
    let mut out = vec![Value::num(x)];
    if nargout > 1 {
        out.push(Value::num(fx));
    }
    if nargout > 2 {
        out.push(Value::num(1.0));
    }
    Ok(out)
}

fn fminsearch(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let f = arg(a, 0, "fminsearch")?.clone();
    let x0 = mat_arg(a, 1, "fminsearch")?.clone();
    let n = x0.numel();
    if n == 0 {
        return Err(MError::new("fminsearch: X0 must not be empty"));
    }
    let (tolx, tolf) = (1e-4, 1e-4);
    let max_iter = 200 * n;
    let max_fev = 200 * n;
    let shape = |v: &[f64]| -> Value {
        let mut m = x0.clone();
        m.re = v.to_vec();
        m.im = None;
        m.class = Class::Double;
        Value::Mat(m)
    };
    let mut fev = 0usize;
    let mut feval = |it: &mut Interp, v: &[f64]| -> Result<f64, MError> {
        fev += 1;
        match it.call1(&f, vec![shape(v)])? {
            Value::Mat(m) if m.numel() >= 1 => Ok(m.re[0]),
            _ => Err(MError::new("fminsearch: function must return a numeric scalar")),
        }
    };
    let mut simplex: Vec<Vec<f64>> = vec![x0.re.clone()];
    for i in 0..n {
        let mut v = x0.re.clone();
        v[i] = if v[i] != 0.0 { 1.05 * v[i] } else { 0.00025 };
        simplex.push(v);
    }
    let mut fv: Vec<f64> = Vec::with_capacity(n + 1);
    for v in &simplex {
        fv.push(feval(it, v)?);
    }
    let sort = |s: &mut Vec<Vec<f64>>, fv: &mut Vec<f64>| {
        let mut idx: Vec<usize> = (0..fv.len()).collect();
        idx.sort_by(|&p, &q| fv[p].partial_cmp(&fv[q]).unwrap_or(std::cmp::Ordering::Equal));
        let s2: Vec<Vec<f64>> = idx.iter().map(|&k| s[k].clone()).collect();
        let f2: Vec<f64> = idx.iter().map(|&k| fv[k]).collect();
        *s = s2;
        *fv = f2;
    };
    sort(&mut simplex, &mut fv);
    let mut iter = 0;
    let mut fev_count = n + 1;
    while iter < max_iter && fev_count < max_fev {
        let fspread = fv.iter().skip(1).map(|x| (x - fv[0]).abs()).fold(0.0, f64::max);
        let xspread = simplex.iter().skip(1).map(|v| v.iter().zip(&simplex[0]).map(|(p, q)| (p - q).abs()).fold(0.0, f64::max)).fold(0.0, f64::max);
        if fspread <= tolf && xspread <= tolx {
            break;
        }
        iter += 1;
        let mut xbar = vec![0.0; n];
        for v in &simplex[..n] {
            for k in 0..n {
                xbar[k] += v[k] / n as f64;
            }
        }
        let worst = simplex[n].clone();
        let xr: Vec<f64> = (0..n).map(|k| 2.0 * xbar[k] - worst[k]).collect();
        let fr = feval(it, &xr)?;
        fev_count += 1;
        if fr < fv[0] {
            let xe: Vec<f64> = (0..n).map(|k| 3.0 * xbar[k] - 2.0 * worst[k]).collect();
            let fe = feval(it, &xe)?;
            fev_count += 1;
            if fe < fr {
                simplex[n] = xe;
                fv[n] = fe;
            } else {
                simplex[n] = xr;
                fv[n] = fr;
            }
        } else if fr < fv[n - 1] {
            simplex[n] = xr;
            fv[n] = fr;
        } else {
            let mut shrink = false;
            if fr < fv[n] {
                let xc: Vec<f64> = (0..n).map(|k| 1.5 * xbar[k] - 0.5 * worst[k]).collect();
                let fc = feval(it, &xc)?;
                fev_count += 1;
                if fc <= fr {
                    simplex[n] = xc;
                    fv[n] = fc;
                } else {
                    shrink = true;
                }
            } else {
                let xcc: Vec<f64> = (0..n).map(|k| 0.5 * xbar[k] + 0.5 * worst[k]).collect();
                let fcc = feval(it, &xcc)?;
                fev_count += 1;
                if fcc < fv[n] {
                    simplex[n] = xcc;
                    fv[n] = fcc;
                } else {
                    shrink = true;
                }
            }
            if shrink {
                let best = simplex[0].clone();
                for j in 1..=n {
                    let v: Vec<f64> = (0..n).map(|k| best[k] + 0.5 * (simplex[j][k] - best[k])).collect();
                    fv[j] = feval(it, &v)?;
                    fev_count += 1;
                    simplex[j] = v;
                }
            }
        }
        sort(&mut simplex, &mut fv);
    }
    let _ = fev;
    if iter >= max_iter || fev_count >= max_fev {
        it.warn("fminsearch: exiting: maximum number of iterations or function evaluations reached");
    }
    let mut out = vec![shape(&simplex[0])];
    if nargout > 1 {
        out.push(Value::num(fv[0]));
    }
    Ok(out)
}

fn eval_rhs(it: &mut Interp, f: &Value, t: f64, y: &[f64]) -> Result<Vec<f64>, MError> {
    let yv = Value::Mat(Mat::col(y.to_vec()));
    match it.call1(f, vec![Value::num(t), yv])? {
        Value::Mat(m) if m.numel() == y.len() && !m.is_complex() => Ok(m.re),
        Value::Mat(m) => Err(MError::new(format!(
            "ode45: function must return a vector of length {} (got {}x{})",
            y.len(),
            m.rows,
            m.cols
        ))),
        _ => Err(MError::new("ode45: function must return a numeric vector")),
    }
}

fn ode45(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    if nargout < 2 {
        return Err(MError::new("ode45: the struct output needs structs, which are not supported in mlab v1; use [t, y] = ode45(...)"));
    }
    if a.len() > 3 {
        return Err(MError::new("ode45: options (odeset) need structs, which are not supported in mlab v1"));
    }
    let f = arg(a, 0, "ode45")?.clone();
    let tspan = mat_arg(a, 1, "ode45")?.re.clone();
    let y0 = mat_arg(a, 2, "ode45")?.re.clone();
    if tspan.len() < 2 {
        return Err(MError::new("ode45: TRANGE must contain at least 2 elements"));
    }
    let (rtol, atol) = (1e-3, 1e-6);
    let t0 = tspan[0];
    let tf = *tspan.last().unwrap();
    let dir = if tf >= t0 { 1.0 } else { -1.0 };
    let neq = y0.len();
    // Dormand — Prince
    const C: [f64; 7] = [0.0, 0.2, 0.3, 0.8, 8.0 / 9.0, 1.0, 1.0];
    let aa: [[f64; 6]; 7] = [
        [0.0; 6],
        [0.2, 0.0, 0.0, 0.0, 0.0, 0.0],
        [3.0 / 40.0, 9.0 / 40.0, 0.0, 0.0, 0.0, 0.0],
        [44.0 / 45.0, -56.0 / 15.0, 32.0 / 9.0, 0.0, 0.0, 0.0],
        [19372.0 / 6561.0, -25360.0 / 2187.0, 64448.0 / 6561.0, -212.0 / 729.0, 0.0, 0.0],
        [9017.0 / 3168.0, -355.0 / 33.0, 46732.0 / 5247.0, 49.0 / 176.0, -5103.0 / 18656.0, 0.0],
        [35.0 / 384.0, 0.0, 500.0 / 1113.0, 125.0 / 192.0, -2187.0 / 6784.0, 11.0 / 84.0],
    ];
    const E: [f64; 7] = [
        71.0 / 57600.0,
        0.0,
        -71.0 / 16695.0,
        71.0 / 1920.0,
        -17253.0 / 339200.0,
        22.0 / 525.0,
        -1.0 / 40.0,
    ];
    let mut t = t0;
    let mut y = y0.clone();
    let mut k1 = eval_rhs(it, &f, t, &y)?;
    let hmax = 0.1 * (tf - t0).abs();
    // initial step
    let threshold = atol / rtol;
    let rh = (0..neq)
        .map(|i| k1[i].abs() / y[i].abs().max(threshold))
        .fold(0.0, f64::max)
        / (0.8 * rtol.powf(0.2));
    let mut h = if rh * hmax > 1.0 { 1.0 / rh } else { hmax };
    h = h.max(16.0 * f64::EPSILON * t.abs().max(1.0));
    let refine_all = tspan.len() == 2;
    let mut ts = vec![t0];
    let mut ys = vec![y0.clone()];
    let mut next_out = 1usize;
    let mut steps = 0usize;
    while dir * (tf - t) > 0.0 {
        steps += 1;
        if steps > 1_000_000 {
            return Err(MError::new("ode45: too many steps"));
        }
        let hmin = 16.0 * f64::EPSILON * t.abs().max(1.0);
        h = h.min(hmax).max(hmin);
        // up to the end of the interval or to the next output point
        let target = if refine_all { tf } else { tspan[next_out] };
        let mut last = false;
        if 1.1 * h >= (target - t).abs() {
            h = (target - t).abs();
            last = true;
        }
        let hs = dir * h;
        let mut k = vec![k1.clone()];
        for s in 1..7 {
            let yt: Vec<f64> = (0..neq).map(|i| y[i] + hs * (0..s).map(|j| aa[s][j] * k[j][i]).sum::<f64>()).collect();
            k.push(eval_rhs(it, &f, t + C[s] * hs, &yt)?);
        }
        // 5th order = row a7 (FSAL): y_new = y + h Σ b_j k_j
        let ynew: Vec<f64> = (0..neq).map(|i| y[i] + hs * (0..6).map(|j| aa[6][j] * k[j][i]).sum::<f64>()).collect();
        let knew = eval_rhs(it, &f, t + hs, &ynew)?;
        let mut err: f64 = 0.0;
        for i in 0..neq {
            let ei = hs * ((0..6).map(|j| E[j] * k[j][i]).sum::<f64>() + E[6] * knew[i]);
            let sc = atol.max(rtol * y[i].abs().max(ynew[i].abs()));
            err = err.max(ei.abs() / sc);
        }
        if !err.is_finite() {
            return Err(MError::new("ode45: solution became non-finite"));
        }
        if err <= 1.0 {
            t = if last { target } else { t + hs };
            y = ynew;
            k1 = knew;
            if refine_all {
                ts.push(t);
                ys.push(y.clone());
            } else if last {
                ts.push(t);
                ys.push(y.clone());
                next_out += 1;
                if next_out >= tspan.len() {
                    break;
                }
            }
            let fac = if err == 0.0 { 5.0 } else { (0.9 * err.powf(-0.2)).clamp(0.2, 5.0) };
            h *= fac;
        } else {
            let fac = (0.9 * err.powf(-0.2)).clamp(0.1, 1.0);
            h *= fac;
            if h < hmin {
                return Err(MError::new(format!("ode45: step size became too small at t = {t}")));
            }
        }
    }
    let n = ts.len();
    let mut ym = Mat::zeros(n, neq);
    for (r, yv) in ys.iter().enumerate() {
        for c in 0..neq {
            ym.re[c * n + r] = yv[c];
        }
    }
    Ok(vec![Value::Mat(Mat::col(ts)), Value::Mat(ym)])
}

fn trapz(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let (x, y) = if a.len() >= 2 { (Some(mat_arg(a, 0, "trapz")?), mat_arg(a, 1, "trapz")?) } else { (None, mat_arg(a, 0, "trapz")?) };
    let colwise = !(y.rows == 1);
    let (len, count) = if colwise { (y.rows, y.cols) } else { (y.cols, 1) };
    // trapz(h, Y) with a scalar step h; length of X ≠ length of Y — an error (was: index panic, tests
    // Octave, tests3)
    let step = x.filter(|x| x.numel() == 1).map(|x| x.re[0]);
    let x = x.filter(|x| x.numel() != 1);
    if let Some(x) = x {
        if x.numel() != len {
            return Err(MError::new("trapz: length of X and length of Y along DIM must match"));
        }
    }
    let mut out = Vec::with_capacity(count);
    for s in 0..count {
        let get = |t: usize| if colwise { y.re[s * y.rows + t] } else { y.re[t] };
        let mut acc = 0.0;
        for t in 1..len {
            let dx = match x {
                Some(x) => x.re[t] - x.re[t - 1],
                None => step.unwrap_or(1.0),
            };
            acc += dx * (get(t) + get(t - 1)) / 2.0;
        }
        out.push(acc);
    }
    if out.len() == 1 {
        return num(out[0]);
    }
    one(Mat::row(out))
}
