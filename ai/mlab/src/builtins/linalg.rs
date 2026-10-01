//! Linear algebra on faer: \ / inv det lu qr chol eig svd norm rank trace kron dot cross pinv cond.
//!
//! Result-checking gates (a warning, not silence):
//! - `\`: backward error ‖A·x − b‖ / (‖A‖‖x‖ + ‖b‖); for least squares — ‖Aᵀ(A·x − b)‖ / (‖A‖₁(‖A‖‖x‖ + ‖b‖));
//! - `inv`: ‖A·X − I‖ / (‖A‖‖X‖);
//! - `eig`: ‖A·V − V·D‖ / (‖A‖‖V‖), without vectors — Σλ against the trace;
//! - `svd`: ‖U·S·Vᵀ − A‖ / ‖A‖, without vectors — Σσ² against ‖A‖²_F;
//! - `lu`, `qr`, `chol` — reconstruction from the factors.
//! Threshold — 1000 · n · eps; stable algorithms sit orders of magnitude below it.

use super::*;
use crate::ops;
use faer::linalg::solvers::{DenseSolveCore, Solve};
use faer::{MatRef, Side, c64};

type FMat = faer::Mat<f64>;

pub fn register(r: &mut Registry) {
    r.register("inv", |it, a, _| one(inv_checked(it, mat_arg(a, 0, "inv")?)?));
    r.register("inverse", |it, a, _| one(inv_checked(it, mat_arg(a, 0, "inverse")?)?));
    r.register("det", det);
    r.register("mldivide", |it, a, _| {
        let x = mat_arg(a, 0, "mldivide")?;
        let y = mat_arg(a, 1, "mldivide")?;
        Ok(vec![ops::binary(it, crate::ast::BinOp::LDiv, &Value::Mat(x.clone()), &Value::Mat(y.clone()))?])
    });
    r.register("mrdivide", |it, a, _| {
        let x = mat_arg(a, 0, "mrdivide")?;
        let y = mat_arg(a, 1, "mrdivide")?;
        Ok(vec![ops::binary(it, crate::ast::BinOp::Div, &Value::Mat(x.clone()), &Value::Mat(y.clone()))?])
    });
    r.register("lu", lu);
    r.register("qr", qr);
    r.register("chol", chol);
    r.register("eig", eig);
    r.register("svd", svd);
    r.register("norm", norm);
    r.register("rank", rank);
    r.register("trace", |_, a, _| {
        let m = real_mat(a, 0, "trace")?;
        if m.rows != m.cols {
            return Err(MError::new("trace: only valid on square matrix"));
        }
        num((0..m.rows).map(|i| m.at(i, i)).sum())
    });
    r.register("kron", |_, a, _| {
        let x = real_mat(a, 0, "kron")?;
        let y = real_mat(a, 1, "kron")?;
        let (r, c) = (x.rows * y.rows, x.cols * y.cols);
        let mut out = Mat::zeros(r, c);
        for j1 in 0..x.cols {
            for i1 in 0..x.rows {
                let s = x.at(i1, j1);
                for j2 in 0..y.cols {
                    for i2 in 0..y.rows {
                        out.re[(j1 * y.cols + j2) * r + i1 * y.rows + i2] = s * y.at(i2, j2);
                    }
                }
            }
        }
        one(out)
    });
    r.register("dot", |_, a, _| {
        let (xc, yc) = (mat_arg(a, 0, "dot")?, mat_arg(a, 1, "dot")?);
        if (xc.is_complex() || yc.is_complex()) && xc.numel() == yc.numel() && (xc.rows == 1 || xc.cols == 1) {
            // complex vectors: Σ conj(x)·y (vmm-la-033, tests3)
            let (xi, yi) = (xc.im.clone().unwrap_or(vec![0.0; xc.numel()]), yc.im.clone().unwrap_or(vec![0.0; yc.numel()]));
            let (mut re, mut im) = (0.0, 0.0);
            for k in 0..xc.numel() {
                re += xc.re[k] * yc.re[k] + xi[k] * yi[k];
                im += xc.re[k] * yi[k] - xi[k] * yc.re[k];
            }
            return one(Mat::complex(1, 1, vec![re], vec![im]));
        }
        let x = real_mat(a, 0, "dot")?;
        let y = real_mat(a, 1, "dot")?;
        if x.numel() != y.numel() {
            return Err(MError::new("dot: sizes of X and Y must match"));
        }
        if (x.rows == 1 || x.cols == 1) && (y.rows == 1 || y.cols == 1) {
            return num(x.re.iter().zip(&y.re).map(|(p, q)| p * q).sum());
        }
        let v: Vec<f64> = (0..x.cols).map(|j| (0..x.rows).map(|i| x.at(i, j) * y.at(i, j)).sum()).collect();
        one(Mat::row(v))
    });
    r.register("cross", |_, a, _| {
        let x = real_mat(a, 0, "cross")?;
        let y = real_mat(a, 1, "cross")?;
        if x.numel() != 3 || y.numel() != 3 {
            return Err(MError::new("cross: both X and Y must have 3 elements"));
        }
        let (p, q) = (&x.re, &y.re);
        let v = vec![p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]];
        one(if x.cols == 1 { Mat::col(v) } else { Mat::row(v) })
    });
    r.register("pinv", |_, a, _| {
        let m = real_mat(a, 0, "pinv")?;
        let eye_m = eye(m.rows);
        one(lstsq_svd(m, &eye_m)?)
    });
    r.register("cond", |_, a, _| {
        let m = real_mat(a, 0, "cond")?;
        if m.is_empty() {
            return num(0.0);
        }
        let s = singular_values(m)?;
        let mn = *s.last().unwrap();
        num(if mn == 0.0 { f64::INFINITY } else { s[0] / mn })
    });
}

fn real_mat<'a>(a: &'a [Value], k: usize, name: &str) -> Result<&'a Mat, MError> {
    let m = mat_arg(a, k, name)?;
    if m.is_complex() {
        return Err(MError::new(format!("{name}: complex matrices are not supported in mlab v1")));
    }
    Ok(m)
}

/// Zero-copy view: mlab data is already column-major.
pub fn view(m: &Mat) -> MatRef<'_, f64> {
    MatRef::from_column_major_slice(&m.re, m.rows, m.cols)
}

pub fn to_faer(m: &Mat) -> FMat {
    view(m).to_owned()
}

pub fn from_faer(f: MatRef<'_, f64>) -> Mat {
    let (r, c) = (f.nrows(), f.ncols());
    let mut re = Vec::with_capacity(r * c);
    for j in 0..c {
        re.extend(f.col(j).iter().copied());
    }
    Mat::new(r, c, re)
}

fn from_faer_c(f: MatRef<'_, c64>) -> Mat {
    let (r, c) = (f.nrows(), f.ncols());
    let mut re = Vec::with_capacity(r * c);
    let mut im = Vec::with_capacity(r * c);
    for j in 0..c {
        for i in 0..r {
            re.push(f[(i, j)].re);
            im.push(f[(i, j)].im);
        }
    }
    Mat::complex(r, c, re, im)
}

pub fn eye(n: usize) -> Mat {
    let mut m = Mat::zeros(n, n);
    for k in 0..n {
        m.re[k * n + k] = 1.0;
    }
    m
}

fn abs_at(m: &Mat, k: usize) -> f64 {
    match &m.im {
        Some(im) => m.re[k].hypot(im[k]),
        None => m.re[k].abs(),
    }
}

pub fn norm_inf(m: &Mat) -> f64 {
    let mut rows = vec![0.0f64; m.rows];
    for j in 0..m.cols {
        for i in 0..m.rows {
            rows[i] += abs_at(m, j * m.rows + i);
        }
    }
    rows.into_iter().fold(0.0, f64::max)
}

pub fn norm_1(m: &Mat) -> f64 {
    (0..m.cols).map(|j| (0..m.rows).map(|i| abs_at(m, j * m.rows + i)).sum::<f64>()).fold(0.0, f64::max)
}

fn e2(x: f64) -> String {
    crate::display::exp_str(x, 1)
}

fn tol_for(n: usize) -> f64 {
    1e3 * (n.max(1) as f64) * f64::EPSILON
}

fn all_finite(ms: &[&Mat]) -> bool {
    ms.iter().all(|m| m.re.iter().all(|x| x.is_finite()) && m.im.as_ref().map_or(true, |v| v.iter().all(|x| x.is_finite())))
}

/// Estimate of the reciprocal condition number in the 1-norm (Hager's method with LU).
fn rcond_est(a: &Mat, lu: &faer::linalg::solvers::PartialPivLu<f64>) -> f64 {
    let n = a.rows;
    if n == 0 {
        return f64::INFINITY;
    }
    let anorm = norm_1(a);
    if anorm == 0.0 {
        return 0.0;
    }
    let u = lu.U();
    for i in 0..n {
        let d = u[(i, i)];
        if d == 0.0 || !d.is_finite() {
            return 0.0;
        }
    }
    let mut x = FMat::from_fn(n, 1, |_, _| 1.0 / n as f64);
    let mut est: f64 = 0.0;
    for _ in 0..5 {
        let mut y = x.clone();
        lu.solve_in_place(&mut y);
        let y1: f64 = (0..n).map(|i| y[(i, 0)].abs()).sum();
        est = est.max(y1);
        let mut z = FMat::from_fn(n, 1, |i, _| if y[(i, 0)] >= 0.0 { 1.0 } else { -1.0 });
        lu.solve_transpose_in_place(&mut z);
        let (mut jmax, mut zmax) = (0usize, 0.0f64);
        for i in 0..n {
            if z[(i, 0)].abs() > zmax {
                zmax = z[(i, 0)].abs();
                jmax = i;
            }
        }
        let ztx: f64 = (0..n).map(|i| z[(i, 0)] * x[(i, 0)]).sum();
        if zmax <= ztx {
            break;
        }
        x = FMat::from_fn(n, 1, |i, _| if i == jmax { 1.0 } else { 0.0 });
    }
    // Higham's fallback estimate: x_i = (−1)^i (1 + i/(n−1))
    let mut b = FMat::from_fn(n, 1, |i, _| {
        let s = if i % 2 == 0 { 1.0 } else { -1.0 };
        s * (1.0 + if n > 1 { i as f64 / (n - 1) as f64 } else { 0.0 })
    });
    lu.solve_in_place(&mut b);
    let alt = 2.0 * (0..n).map(|i| b[(i, 0)].abs()).sum::<f64>() / (3.0 * n as f64);
    est = est.max(alt);
    if !est.is_finite() {
        return 0.0;
    }
    1.0 / (anorm * est)
}

/// Minimum-norm least-squares solution via SVD (like xGELSD).
pub fn lstsq_svd(a: &Mat, b: &Mat) -> Result<Mat, MError> {
    let (m, n) = (a.rows, a.cols);
    if m == 0 || n == 0 {
        return Ok(Mat::zeros(n, b.cols));
    }
    let svd = to_faer(a).thin_svd().map_err(|_| MError::new("svd: failed to converge"))?;
    let (u, s, v) = (svd.U(), svd.S(), svd.V());
    let k = m.min(n);
    let smax = if k > 0 { s[0] } else { 0.0 };
    let tol = (m.max(n) as f64) * smax * f64::EPSILON;
    let mut x = Mat::zeros(n, b.cols);
    for col in 0..b.cols {
        for t in 0..k {
            let sv = s[t];
            if sv <= tol {
                continue;
            }
            let mut dotp = 0.0;
            for i in 0..m {
                dotp += u[(i, t)] * b.re[col * m + i];
            }
            let coef = dotp / sv;
            for i in 0..n {
                x.re[col * n + i] += v[(i, t)] * coef;
            }
        }
    }
    Ok(x)
}

/// Least squares for m > n: Householder QR with column pivoting (like MATLAB `\` for rectangular matrices). QR accuracy does not
/// depend on column scaling (unlike SVD), so polynomials on raw powers (NIST Pontius,
/// Wampler) are computed with full accuracy. Rank — from the diagonal of R: |R_kk| > max(m, n)·eps·|R_11|.
/// Returns `None` if the matrix is rank deficient (then — minimum-norm SVD with a warning).
pub fn lstsq_qr(a: &Mat, b: &Mat) -> Option<Mat> {
    let (m, n) = (a.rows, a.cols);
    if m < n || n == 0 {
        return None;
    }
    let mut cols: Vec<Vec<f64>> = (0..n).map(|j| a.re[j * m..(j + 1) * m].to_vec()).collect();
    let mut rhs: Vec<Vec<f64>> = (0..b.cols).map(|j| b.re[j * m..(j + 1) * m].to_vec()).collect();
    let mut perm: Vec<usize> = (0..n).collect();
    let mut r11 = 0.0f64;
    let tol_rel = (m.max(n) as f64) * f64::EPSILON;
    for k in 0..n {
        // column with the largest residual norm
        let norm_of = |c: &Vec<f64>| -> f64 {
            let mx = c[k..].iter().fold(0.0f64, |s, v| s.max(v.abs()));
            if mx == 0.0 { 0.0 } else { mx * c[k..].iter().map(|v| (v / mx) * (v / mx)).sum::<f64>().sqrt() }
        };
        let (mut best, mut bn) = (k, norm_of(&cols[k]));
        for j in k + 1..n {
            let nj = norm_of(&cols[j]);
            if nj > bn {
                (best, bn) = (j, nj);
            }
        }
        cols.swap(k, best);
        perm.swap(k, best);
        if k == 0 {
            r11 = bn;
        }
        if bn <= tol_rel * r11 || bn == 0.0 {
            return None;
        }
        let alpha = if cols[k][k] > 0.0 { -bn } else { bn };
        let mut v: Vec<f64> = cols[k][k..].to_vec();
        v[0] -= alpha;
        let vv: f64 = v.iter().map(|z| z * z).sum();
        if vv == 0.0 {
            continue;
        }
        let reflect = |c: &mut Vec<f64>| {
            let s: f64 = v.iter().zip(&c[k..]).map(|(p, q)| p * q).sum::<f64>() * 2.0 / vv;
            for (ci, vi) in c[k..].iter_mut().zip(&v) {
                *ci -= s * vi;
            }
        };
        for c in cols.iter_mut().skip(k) {
            reflect(c);
        }
        for c in rhs.iter_mut() {
            reflect(c);
        }
    }
    let mut x = Mat::zeros(n, b.cols);
    for (col, c) in rhs.iter().enumerate() {
        let mut z = vec![0.0; n];
        for i in (0..n).rev() {
            let s: f64 = (i + 1..n).map(|j| cols[j][i] * z[j]).sum();
            z[i] = (c[i] - s) / cols[i][i];
        }
        for (k, &p) in perm.iter().enumerate() {
            x.re[col * n + p] = z[k];
        }
    }
    Some(x)
}

fn residual(a: &Mat, x: &Mat, b: &Mat) -> Result<Mat, MError> {
    let ax = ops::matmul(a, x)?;
    ops::elementwise(crate::ast::BinOp::Sub, &ax, b)
}

fn gate_solve(it: &mut Interp, a: &Mat, b: &Mat, x: &Mat, least_squares: bool) {
    if !all_finite(&[a, b, x]) {
        if all_finite(&[a, b]) {
            it.gate_warn("gate mldivide: solution contains Inf or NaN");
        }
        return;
    }
    let r = match residual(a, x, b) {
        Ok(r) => r,
        Err(_) => return,
    };
    let n = a.rows.max(a.cols);
    let tol = tol_for(n);
    let an = norm_inf(a);
    let denom = an * norm_inf(x) + norm_inf(b);
    if !least_squares {
        let eta = if denom == 0.0 { 0.0 } else { norm_inf(&r) / denom };
        if eta > tol {
            it.gate_warn(&format!("gate mldivide: |A*x - b| / (|A|*|x| + |b|) = {} exceeds {}", e2(eta), e2(tol)));
        }
    } else {
        let g = match ops::matmul(&a.transpose(), &r) {
            Ok(g) => g,
            Err(_) => return,
        };
        let d = norm_1(a) * denom;
        let eta = if d == 0.0 { 0.0 } else { norm_inf(&g) / d };
        if eta > tol {
            it.gate_warn(&format!("gate mldivide: least squares |A'*(A*x - b)| / (|A|*(|A|*|x| + |b|)) = {} exceeds {}", e2(eta), e2(tol)));
        }
    }
}

/// A \ B with a condition check and a gate.
pub fn mldivide(it: &mut Interp, a: &Mat, b: &Mat) -> Result<Mat, MError> {
    if a.is_complex() || b.is_complex() {
        return complex_solve(a, b);
    }
    let (m, n) = (a.rows, a.cols);
    if m == 0 || n == 0 || b.cols == 0 {
        return Ok(Mat::zeros(n, b.cols));
    }
    let finite = all_finite(&[a, b]);
    let mut least_squares = false;
    let mut x = if m == n {
        let lu = to_faer(a).partial_piv_lu();
        let rc = if finite { rcond_est(a, &lu) } else { f64::NAN };
        if finite && rc < f64::EPSILON {
            it.warn("matrix singular to machine precision");
            least_squares = true;
            lstsq_svd(a, b)?
        } else {
            from_faer(lu.solve(&to_faer(b)).as_ref())
        }
    } else {
        least_squares = true;
        // m > n: QR with column pivoting; rank deficient (or m < n) — minimum-norm SVD
        match lstsq_qr(a, b) {
            Some(x) => x,
            None => {
                if m > n && finite {
                    it.warn("rank deficient least squares; minimum-norm solution");
                }
                lstsq_svd(a, b)?
            }
        }
    };
    if it.fault.as_deref() == Some("gate:mldivide") && !x.re.is_empty() {
        x.re[0] += 1.0 + x.re[0].abs();
    }
    if finite {
        gate_solve(it, a, b, &x, least_squares);
    }
    Ok(x)
}

/// Complex square solve — Gaussian elimination with partial pivoting (minimal v1 support).
fn complex_solve(a: &Mat, b: &Mat) -> Result<Mat, MError> {
    let n = a.rows;
    if a.cols != n {
        return Err(MError::new("mldivide: complex least squares is not supported in mlab v1"));
    }
    let (ai, bi) = (a.im_or_zeros(), b.im_or_zeros());
    let mut m: Vec<Vec<(f64, f64)>> = (0..n).map(|i| (0..n).map(|j| (a.re[j * n + i], ai[j * n + i])).collect()).collect();
    let mut rhs: Vec<Vec<(f64, f64)>> = (0..n).map(|i| (0..b.cols).map(|j| (b.re[j * n + i], bi[j * n + i])).collect()).collect();
    for k in 0..n {
        let p = (k..n).max_by(|&x, &y| m[x][k].0.hypot(m[x][k].1).partial_cmp(&m[y][k].0.hypot(m[y][k].1)).unwrap()).unwrap();
        m.swap(k, p);
        rhs.swap(k, p);
        let piv = m[k][k];
        if piv.0 == 0.0 && piv.1 == 0.0 {
            return Err(MError::new("mldivide: matrix singular to machine precision"));
        }
        for i in k + 1..n {
            let f = ops::cdiv(m[i][k], piv);
            for j in k..n {
                let t = ops::cmul(f, m[k][j]);
                m[i][j] = (m[i][j].0 - t.0, m[i][j].1 - t.1);
            }
            for j in 0..b.cols {
                let t = ops::cmul(f, rhs[k][j]);
                rhs[i][j] = (rhs[i][j].0 - t.0, rhs[i][j].1 - t.1);
            }
        }
    }
    let mut x = vec![vec![(0.0, 0.0); b.cols]; n];
    for i in (0..n).rev() {
        for j in 0..b.cols {
            let mut s = rhs[i][j];
            for k in i + 1..n {
                let t = ops::cmul(m[i][k], x[k][j]);
                s = (s.0 - t.0, s.1 - t.1);
            }
            x[i][j] = ops::cdiv(s, m[i][i]);
        }
    }
    let mut re = Vec::with_capacity(n * b.cols);
    let mut im = Vec::with_capacity(n * b.cols);
    for j in 0..b.cols {
        for row in x.iter() {
            re.push(row[j].0);
            im.push(row[j].1);
        }
    }
    Ok(Mat::complex(n, b.cols, re, im))
}

pub fn inv_checked(it: &mut Interp, a: &Mat) -> Result<Mat, MError> {
    if a.rows != a.cols {
        return Err(MError::new("inverse: argument must be a square matrix"));
    }
    if a.is_complex() {
        return Err(MError::new("inverse: complex matrices are not supported in mlab v1"));
    }
    let n = a.rows;
    if n == 0 {
        return Ok(Mat::empty());
    }
    let lu = to_faer(a).partial_piv_lu();
    let u = lu.U();
    if (0..n).any(|i| u[(i, i)] == 0.0) {
        it.warn("matrix singular to machine precision");
        return Ok(Mat::filled(n, n, f64::INFINITY));
    }
    let finite = all_finite(&[a]);
    if finite && rcond_est(a, &lu) < f64::EPSILON {
        it.warn("matrix singular to machine precision");
    }
    let mut x = from_faer(lu.inverse().as_ref());
    if it.fault.as_deref() == Some("gate:inv") {
        x.re[0] += 1.0 + x.re[0].abs();
    }
    if finite && all_finite(&[&x]) {
        let ax = ops::matmul(a, &x)?;
        let r = ops::elementwise(crate::ast::BinOp::Sub, &ax, &eye(n))?;
        let d = norm_inf(a) * norm_inf(&x);
        let eta = if d == 0.0 { 0.0 } else { norm_inf(&r) / d };
        let tol = tol_for(n);
        if eta > tol {
            it.gate_warn(&format!("gate inv: |A*inv(A) - I| / (|A|*|inv(A)|) = {} exceeds {}", e2(eta), e2(tol)));
        }
    }
    Ok(x)
}

/// Parity of a permutation (−1 or 1).
fn perm_sign(p: &[usize]) -> f64 {
    let n = p.len();
    let mut seen = vec![false; n];
    let mut sign = 1.0;
    for s in 0..n {
        if seen[s] {
            continue;
        }
        let mut len = 0;
        let mut k = s;
        while !seen[k] {
            seen[k] = true;
            k = p[k];
            len += 1;
        }
        if len % 2 == 0 {
            sign = -sign;
        }
    }
    sign
}

fn det(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = real_mat(a, 0, "det")?;
    if m.rows != m.cols {
        return Err(MError::new("det: A must be a square matrix"));
    }
    let n = m.rows;
    if n == 0 {
        return num(1.0);
    }
    let lu = to_faer(m).partial_piv_lu();
    let (fwd, _) = lu.P().arrays();
    let u = lu.U();
    let mut d = perm_sign(fwd);
    for i in 0..n {
        // zero pivot — the determinant is exactly 0 (otherwise faer divides 0/0 and gives NaN: det(zeros(3)) was NaN;
        // found by LLM scenario vmm-la-003, tests3)
        if u[(i, i)] == 0.0 {
            return num(0.0);
        }
        d *= u[(i, i)];
    }
    num(d)
}

/// faer row permutation as a vector «row i of the result = row p[i] of the input»: P·A = L·U.
fn perm_rows(lu: &faer::linalg::solvers::PartialPivLu<f64>) -> Vec<usize> {
    lu.P().arrays().0.to_vec()
}

fn frob_rel(x: &Mat, y: &Mat) -> f64 {
    let d: f64 = x.re.iter().zip(&y.re).map(|(p, q)| (p - q) * (p - q)).sum::<f64>().sqrt();
    let s: f64 = y.re.iter().map(|q| q * q).sum::<f64>().sqrt();
    if s == 0.0 { d } else { d / s }
}

fn lu(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = real_mat(a, 0, "lu")?;
    let (r, c) = (m.rows, m.cols);
    let f = to_faer(m).partial_piv_lu();
    let l = from_faer(f.L());
    let u = from_faer(f.U());
    let p = perm_rows(&f);
    let mut pm = Mat::zeros(r, r);
    for (i, &pi) in p.iter().enumerate() {
        pm.re[pi * r + i] = 1.0;
    }
    // gate: P·A = L·U
    if all_finite(&[m]) && !m.is_empty() {
        let pa = ops::matmul(&pm, m)?;
        let lu_ = ops::matmul(&l, &u)?;
        let e = frob_rel(&lu_, &pa);
        let tol = tol_for(r.max(c));
        if e > tol {
            it.gate_warn(&format!("gate lu: |P*A - L*U| / |A| = {} exceeds {}", e2(e), e2(tol)));
        }
    }
    match nargout {
        0 | 1 => {
            // Y = L + U − I (in the square part)
            let k = r.min(c);
            let mut y = Mat::zeros(r, c);
            for j in 0..c {
                for i in 0..r {
                    let lv = if j < k && i > j { l.re[j * r + i] } else { 0.0 };
                    let uv = if i < k && i <= j { u.re[j * k + i] } else { 0.0 };
                    y.re[j * r + i] = lv + uv;
                }
            }
            one(y)
        }
        2 => {
            let pl = ops::matmul(&pm.transpose(), &l)?;
            Ok(vec![Value::Mat(pl), Value::Mat(u)])
        }
        _ => Ok(vec![Value::Mat(l), Value::Mat(u), Value::Mat(pm)]),
    }
}

fn qr(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = real_mat(a, 0, "qr")?;
    let econ = a.len() > 1;
    let (r, c) = (m.rows, m.cols);
    let f = to_faer(m).qr();
    let k = r.min(c);
    let (q, rr) = if econ {
        (from_faer(f.compute_thin_Q().as_ref()), from_faer(f.thin_R()))
    } else {
        let q = from_faer(f.compute_Q().as_ref());
        let rt = from_faer(f.R());
        // full-size R r×c: pad with zeros
        let mut full = Mat::zeros(r, c);
        for j in 0..c {
            for i in 0..rt.rows.min(r) {
                full.re[j * r + i] = if i <= j || i < k { rt.re[j * rt.rows + i] } else { 0.0 };
            }
        }
        (q, full)
    };
    // zeros below the diagonal of R are exact
    let mut rr = rr;
    for j in 0..rr.cols {
        for i in (j + 1)..rr.rows {
            rr.re[j * rr.rows + i] = 0.0;
        }
    }
    if all_finite(&[m]) && !m.is_empty() {
        let qr_ = ops::matmul(&q, &rr)?;
        let e = frob_rel(&qr_, m);
        let tol = tol_for(r.max(c));
        if e > tol {
            it.gate_warn(&format!("gate qr: |Q*R - A| / |A| = {} exceeds {}", e2(e), e2(tol)));
        }
    }
    if nargout <= 1 {
        return one(rr);
    }
    Ok(vec![Value::Mat(q), Value::Mat(rr)])
}

fn chol(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = real_mat(a, 0, "chol")?;
    if m.rows != m.cols {
        return Err(MError::new("chol: A must be a square matrix"));
    }
    let n = m.rows;
    // the upper triangle is used
    let sym = FMat::from_fn(n, n, |i, j| {
        let (p, q) = if i <= j { (i, j) } else { (j, i) };
        m.re[q * n + p]
    });
    match sym.llt(Side::Lower) {
        Ok(f) => {
            let l = from_faer(f.L());
            let mut rr = l.transpose();
            for j in 0..n {
                for i in (j + 1)..n {
                    rr.re[j * n + i] = 0.0;
                }
            }
            if all_finite(&[m]) && n > 0 {
                let rtr = ops::matmul(&rr.transpose(), &rr)?;
                let upper = Mat::new(n, n, (0..n * n).map(|k| sym[(k % n, k / n)]).collect());
                let e = frob_rel(&rtr, &upper);
                let tol = tol_for(n);
                if e > tol {
                    it.gate_warn(&format!("gate chol: |R'*R - A| / |A| = {} exceeds {}", e2(e), e2(tol)));
                }
            }
            if nargout > 1 {
                return Ok(vec![Value::Mat(rr), Value::num(0.0)]);
            }
            one(rr)
        }
        Err(_) => {
            if nargout > 1 {
                return Ok(vec![Value::Mat(Mat::empty()), Value::num(1.0)]);
            }
            Err(MError::new("chol: input matrix must be positive definite"))
        }
    }
}

fn eig(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "eig")?;
    if m.rows != m.cols {
        return Err(MError::new("eig: argument must be a square matrix"));
    }
    if m.is_complex() {
        return Err(MError::new("eig: complex matrices are not supported in mlab v1"));
    }
    if !all_finite(&[m]) {
        return Err(MError::new("eig: EIG: matrix contains Inf or NaN values"));
    }
    let n = m.rows;
    if n == 0 {
        return if nargout > 1 { Ok(vec![Value::Mat(Mat::empty()), Value::Mat(Mat::empty())]) } else { one(Mat::empty()) };
    }
    let symmetric = (0..n).all(|j| (0..j).all(|i| m.re[j * n + i] == m.re[i * n + j]));
    let fa = to_faer(m);
    let tol = tol_for(n);
    let an = norm_inf(m).max(f64::MIN_POSITIVE);
    let fault = it.fault.as_deref() == Some("gate:eig");
    if nargout <= 1 {
        let mut vals = if symmetric {
            Mat::col(fa.self_adjoint_eigenvalues(Side::Lower).map_err(|_| MError::new("eig: failed to converge"))?)
        } else {
            let v = fa.eigenvalues().map_err(|_| MError::new("eig: failed to converge"))?;
            Mat::complex(n, 1, v.iter().map(|z| z.re).collect(), v.iter().map(|z| z.im).collect())
        };
        if fault {
            vals.re[0] += 1.0 + vals.re[0].abs();
        }
        // gate: Σλ = trace
        let tr: f64 = (0..n).map(|i| m.at(i, i)).sum();
        let s: f64 = vals.re.iter().sum();
        let e = (s - tr).abs() / (an * n as f64);
        if e > tol {
            it.gate_warn(&format!("gate eig: |sum(eig(A)) - trace(A)| / (n*|A|) = {} exceeds {}", e2(e), e2(tol)));
        }
        return one(vals);
    }
    let (v, mut d) = if symmetric {
        let ev = fa.self_adjoint_eigen(Side::Lower).map_err(|_| MError::new("eig: failed to converge"))?;
        let v = from_faer(ev.U());
        let s = ev.S();
        let mut d = Mat::zeros(n, n);
        for k in 0..n {
            d.re[k * n + k] = s[k];
        }
        (v, d)
    } else {
        let ev = fa.eigen().map_err(|_| MError::new("eig: failed to converge"))?;
        let mut v = from_faer_c(ev.U());
        // unit column norms
        let vi = v.im_or_zeros();
        let mut vim = vi.clone();
        for j in 0..n {
            let nr: f64 = (0..n).map(|i| v.re[j * n + i].powi(2) + vi[j * n + i].powi(2)).sum::<f64>().sqrt();
            if nr > 0.0 {
                for i in 0..n {
                    v.re[j * n + i] /= nr;
                    vim[j * n + i] /= nr;
                }
            }
        }
        v.im = Some(vim);
        v.narrow();
        let s = ev.S();
        let mut dre = vec![0.0; n * n];
        let mut dim = vec![0.0; n * n];
        for k in 0..n {
            dre[k * n + k] = s[k].re;
            dim[k * n + k] = s[k].im;
        }
        (v, Mat::complex(n, n, dre, dim))
    };
    if fault {
        d.re[0] += 1.0 + d.re[0].abs();
    }
    // gate: A·V = V·D
    let av = ops::matmul(m, &v)?;
    let vd = scale_cols(&v, &d);
    let r = ops::elementwise(crate::ast::BinOp::Sub, &av, &vd)?;
    let e = norm_inf(&r) / (an * norm_inf(&v).max(f64::MIN_POSITIVE));
    if e > tol {
        it.gate_warn(&format!("gate eig: |A*V - V*D| / (|A|*|V|) = {} exceeds {}", e2(e), e2(tol)));
    }
    Ok(vec![Value::Mat(v), Value::Mat(d)])
}

/// V·D for a diagonal D — column scaling (O(n²) instead of O(n³)).
fn scale_cols(v: &Mat, d: &Mat) -> Mat {
    let n = v.rows;
    let c = v.cols;
    let vi = v.im_or_zeros();
    let di = d.im_or_zeros();
    let mut re = vec![0.0; n * c];
    let mut im = vec![0.0; n * c];
    for j in 0..c {
        let dj = (d.re[j * d.rows + j], di[j * d.rows + j]);
        for i in 0..n {
            let z = ops::cmul((v.re[j * n + i], vi[j * n + i]), dj);
            re[j * n + i] = z.0;
            im[j * n + i] = z.1;
        }
    }
    Mat::complex(n, c, re, im)
}

fn singular_values(m: &Mat) -> Result<Vec<f64>, MError> {
    to_faer(m).singular_values().map_err(|_| MError::new("svd: failed to converge"))
}

fn svd(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let m = real_mat(a, 0, "svd")?;
    if !all_finite(&[m]) {
        return Err(MError::new("svd: cannot take SVD of matrix containing Inf or NaN values"));
    }
    let (r, c) = (m.rows, m.cols);
    let econ = a.len() > 1;
    let tol = tol_for(r.max(c));
    let fault = it.fault.as_deref() == Some("gate:svd");
    let fro2: f64 = m.re.iter().map(|x| x * x).sum();
    if nargout <= 1 {
        let mut s = singular_values(m)?;
        if fault && !s.is_empty() {
            s[0] += 1.0 + s[0];
        }
        let s2: f64 = s.iter().map(|x| x * x).sum();
        let e = if fro2 == 0.0 { s2.sqrt() } else { (s2 - fro2).abs() / fro2 };
        if e > tol {
            it.gate_warn(&format!("gate svd: |sum(s.^2) - |A|_F^2| / |A|_F^2 = {} exceeds {}", e2(e), e2(tol)));
        }
        return one(Mat::col(s));
    }
    let f = if econ { to_faer(m).thin_svd() } else { to_faer(m).svd() }.map_err(|_| MError::new("svd: failed to converge"))?;
    let u = from_faer(f.U());
    let v = from_faer(f.V());
    let sd = f.S();
    let k = r.min(c);
    let (sr, sc) = if econ { (k, k) } else { (r, c) };
    let mut s = Mat::zeros(sr, sc);
    for t in 0..k {
        s.re[t * sr + t] = sd[t];
    }
    if fault && k > 0 {
        s.re[0] += 1.0 + s.re[0];
    }
    if !m.is_empty() {
        let usv = ops::matmul(&ops::matmul(&u, &s)?, &v.transpose())?;
        let e = frob_rel(&usv, m);
        if e > tol {
            it.gate_warn(&format!("gate svd: |U*S*V' - A| / |A| = {} exceeds {}", e2(e), e2(tol)));
        }
    }
    Ok(vec![Value::Mat(u), Value::Mat(s), Value::Mat(v)])
}

fn vec_norm(v: &[f64], p: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    if p == 2.0 {
        let mx = v.iter().fold(0.0f64, |a, &x| a.max(x.abs()));
        if mx == 0.0 || !mx.is_finite() {
            return mx;
        }
        if mx > 1e150 || mx < 1e-150 {
            return mx * v.iter().map(|x| (x / mx) * (x / mx)).sum::<f64>().sqrt();
        }
        return v.iter().map(|x| x * x).sum::<f64>().sqrt();
    }
    if p == 1.0 {
        return v.iter().map(|x| x.abs()).sum();
    }
    if p == f64::INFINITY {
        return v.iter().fold(0.0f64, |a, &x| a.max(x.abs()));
    }
    if p == f64::NEG_INFINITY {
        return v.iter().fold(f64::INFINITY, |a, &x| a.min(x.abs()));
    }
    if p == 0.0 {
        return v.iter().filter(|&&x| x != 0.0).count() as f64;
    }
    v.iter().map(|x| x.abs().powf(p)).sum::<f64>().powf(1.0 / p)
}

fn norm(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "norm")?;
    let abs_vals: Vec<f64> = (0..m.numel()).map(|k| m.re[k].hypot(m.im_at(k))).collect();
    let p = match a.get(1) {
        None => 2.0,
        Some(v) => match str_of(v) {
            Some(s) if s == "fro" => -99.0,
            Some(s) if s.eq_ignore_ascii_case("inf") => f64::INFINITY,
            Some(_) => return Err(MError::new("norm: unrecognized option")),
            None => scalar_arg(a, 1, "norm")?,
        },
    };
    if m.is_empty() {
        return num(0.0);
    }
    if p == -99.0 {
        return num(vec_norm(&abs_vals, 2.0));
    }
    if m.rows == 1 || m.cols == 1 {
        return num(vec_norm(&abs_vals, p));
    }
    let am = Mat::new(m.rows, m.cols, abs_vals);
    if p == 1.0 {
        return num(norm_1(&am));
    }
    if p == f64::INFINITY {
        return num(norm_inf(&am));
    }
    if p == 2.0 {
        if m.is_complex() {
            return Err(MError::new("norm: 2-norm of complex matrices is not supported in mlab v1"));
        }
        let s = singular_values(m)?;
        return num(s.first().copied().unwrap_or(0.0));
    }
    Err(MError::new("norm: only 1, 2, Inf and 'fro' norms are supported for matrices"))
}

fn rank(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = real_mat(a, 0, "rank")?;
    if m.is_empty() {
        return num(0.0);
    }
    let s = singular_values(m)?;
    let smax = s.first().copied().unwrap_or(0.0);
    let tol = if a.len() > 1 {
        scalar_arg(a, 1, "rank")?
    } else {
        let e = if smax > 0.0 { 2f64.powf(smax.log2().floor() - 52.0) } else { 0.0 };
        (m.rows.max(m.cols) as f64) * e
    };
    num(s.iter().filter(|&&x| x > tol).count() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lu_perm_convention() {
        // P·A = L·U with the faer permutation, as we interpret it
        let a = Mat::new(3, 3, vec![1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 10.0]);
        let f = to_faer(&a).partial_piv_lu();
        let l = from_faer(f.L());
        let u = from_faer(f.U());
        let p = perm_rows(&f);
        let mut pm = Mat::zeros(3, 3);
        for (i, &pi) in p.iter().enumerate() {
            pm.re[pi * 3 + i] = 1.0;
        }
        let pa = ops::matmul(&pm, &a).unwrap();
        let lu = ops::matmul(&l, &u).unwrap();
        assert!(frob_rel(&lu, &pa) < 1e-14, "P·A ≠ L·U: {:?} vs {:?}", pa.re, lu.re);
    }
}
