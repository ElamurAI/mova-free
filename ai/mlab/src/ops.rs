//! Operators: elementwise with implicit expansion (broadcasting), matrix multiplication, division, power,
//! comparison, logic, transposition, ranges, concatenation.

use crate::ast::{BinOp, UnOp};
use crate::builtins::linalg;
use crate::interp::{Interp, MError};
use crate::value::{Class, Mat, Value};

pub fn mat_of<'a>(v: &'a Value, op: &str) -> Result<&'a Mat, MError> {
    match v {
        Value::Mat(m) => Ok(m),
        other => Err(MError::new(format!("binary operator '{op}' not implemented for '{}' operations", other.class_name()))),
    }
}

fn nonconformant(op: &str, a: &Mat, b: &Mat) -> MError {
    MError::new(format!(
        "operator {op}: nonconformant arguments (op1 is {}x{}, op2 is {}x{})",
        a.rows, a.cols, b.rows, b.cols
    ))
}

pub fn bshape(op: &str, a: &Mat, b: &Mat) -> Result<(usize, usize), MError> {
    let r = if a.rows == b.rows {
        a.rows
    } else if a.rows == 1 {
        b.rows
    } else if b.rows == 1 {
        a.rows
    } else {
        return Err(nonconformant(op, a, b));
    };
    let c = if a.cols == b.cols {
        a.cols
    } else if a.cols == 1 {
        b.cols
    } else if b.cols == 1 {
        a.cols
    } else {
        return Err(nonconformant(op, a, b));
    };
    Ok((r, c))
}

#[inline]
fn bidx(m: &Mat, i: usize, j: usize) -> usize {
    (if m.cols == 1 { 0 } else { j }) * m.rows + if m.rows == 1 { 0 } else { i }
}

/// Elementwise real operation with broadcasting.
pub fn map2(op: &str, a: &Mat, b: &Mat, f: impl Fn(f64, f64) -> f64) -> Result<(usize, usize, Vec<f64>), MError> {
    if a.rows == b.rows && a.cols == b.cols {
        return Ok((a.rows, a.cols, a.re.iter().zip(&b.re).map(|(&x, &y)| f(x, y)).collect()));
    }
    if b.is_scalar() {
        let y = b.re[0];
        return Ok((a.rows, a.cols, a.re.iter().map(|&x| f(x, y)).collect()));
    }
    if a.is_scalar() {
        let x = a.re[0];
        return Ok((b.rows, b.cols, b.re.iter().map(|&y| f(x, y)).collect()));
    }
    let (r, c) = bshape(op, a, b)?;
    let mut out = Vec::with_capacity(r * c);
    for j in 0..c {
        for i in 0..r {
            out.push(f(a.re[bidx(a, i, j)], b.re[bidx(b, i, j)]));
        }
    }
    Ok((r, c, out))
}

/// Elementwise complex operation with broadcasting.
pub fn cmap2(
    op: &str,
    a: &Mat,
    b: &Mat,
    f: impl Fn((f64, f64), (f64, f64)) -> (f64, f64),
) -> Result<Mat, MError> {
    let (r, c) = if a.rows == b.rows && a.cols == b.cols {
        (a.rows, a.cols)
    } else if b.is_scalar() {
        (a.rows, a.cols)
    } else if a.is_scalar() {
        (b.rows, b.cols)
    } else {
        bshape(op, a, b)?
    };
    let mut re = Vec::with_capacity(r * c);
    let mut im = Vec::with_capacity(r * c);
    for j in 0..c {
        for i in 0..r {
            let ka = bidx(a, i.min(a.rows.saturating_sub(1)), j.min(a.cols.saturating_sub(1)));
            let kb = bidx(b, i.min(b.rows.saturating_sub(1)), j.min(b.cols.saturating_sub(1)));
            let (x, y) = f((a.re[ka], a.im_at(ka)), (b.re[kb], b.im_at(kb)));
            re.push(x);
            im.push(y);
        }
    }
    Ok(Mat::complex(r, c, re, im))
}

pub fn cmul(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}
pub fn cdiv(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    if b.1 == 0.0 {
        return (a.0 / b.0, a.1 / b.0);
    }
    // Smith: stable division
    if b.0.abs() >= b.1.abs() {
        let r = b.1 / b.0;
        let d = b.0 + b.1 * r;
        ((a.0 + a.1 * r) / d, (a.1 - a.0 * r) / d)
    } else {
        let r = b.0 / b.1;
        let d = b.0 * r + b.1;
        ((a.0 * r + a.1) / d, (a.1 * r - a.0) / d)
    }
}
pub fn cexp(z: (f64, f64)) -> (f64, f64) {
    let e = z.0.exp();
    if z.1 == 0.0 {
        return (e, 0.0);
    }
    (e * z.1.cos(), e * z.1.sin())
}
pub fn clog(z: (f64, f64)) -> (f64, f64) {
    (z.0.hypot(z.1).ln(), z.1.atan2(z.0))
}
pub fn cpow(z: (f64, f64), w: (f64, f64)) -> (f64, f64) {
    if w.1 == 0.0 && w.0 == w.0.trunc() && w.0.abs() <= 1024.0 {
        // integer power — by multiplication (more accurate for small integers)
        let mut n = w.0.abs() as u64;
        let mut base = z;
        let mut acc = (1.0, 0.0);
        while n > 0 {
            if n & 1 == 1 {
                acc = cmul(acc, base);
            }
            base = cmul(base, base);
            n >>= 1;
        }
        if w.0 < 0.0 {
            acc = cdiv((1.0, 0.0), acc);
        }
        return acc;
    }
    if z.0 == 0.0 && z.1 == 0.0 {
        return if w.0 > 0.0 { (0.0, 0.0) } else { (f64::INFINITY, 0.0) };
    }
    cexp(cmul(w, clog(z)))
}

fn arith_class(_a: &Mat, _b: &Mat) -> Class {
    Class::Double
}

pub fn elementwise(op: BinOp, a: &Mat, b: &Mat) -> Result<Mat, MError> {
    let name = op.text();
    let complex = a.is_complex() || b.is_complex();
    match op {
        BinOp::Add | BinOp::Sub | BinOp::EMul | BinOp::EDiv | BinOp::ELDiv => {
            if complex {
                let f = match op {
                    BinOp::Add => |x: (f64, f64), y: (f64, f64)| (x.0 + y.0, x.1 + y.1),
                    BinOp::Sub => |x: (f64, f64), y: (f64, f64)| (x.0 - y.0, x.1 - y.1),
                    BinOp::EMul => cmul,
                    BinOp::EDiv => cdiv,
                    _ => |x: (f64, f64), y: (f64, f64)| cdiv(y, x),
                };
                return cmap2(name, a, b, f);
            }
            let f: fn(f64, f64) -> f64 = match op {
                BinOp::Add => |x, y| x + y,
                BinOp::Sub => |x, y| x - y,
                BinOp::EMul => |x, y| x * y,
                BinOp::EDiv => |x, y| x / y,
                _ => |x, y| y / x,
            };
            let (r, c, v) = map2(name, a, b, f)?;
            Ok(Mat::new(r, c, v).with_class(arith_class(a, b)))
        }
        BinOp::EPow => {
            let need_complex = complex
                || a.re.iter().any(|&x| x < 0.0) && b.re.iter().any(|&y| y != y.trunc());
            if need_complex {
                return cmap2(name, a, b, cpow);
            }
            let (r, c, v) = map2(name, a, b, f64::powf)?;
            Ok(Mat::new(r, c, v))
        }
        BinOp::Eq | BinOp::Ne => {
            let eq = op == BinOp::Eq;
            if complex {
                let m = cmap2(name, a, b, |x, y| {
                    let e = x.0 == y.0 && x.1 == y.1;
                    (if e == eq { 1.0 } else { 0.0 }, 0.0)
                })?;
                return Ok(Mat::new(m.rows, m.cols, m.re).with_class(Class::Logical));
            }
            let (r, c, v) = map2(name, a, b, |x, y| if (x == y) == eq { 1.0 } else { 0.0 })?;
            Ok(Mat::new(r, c, v).with_class(Class::Logical))
        }
        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
            let f: fn(f64, f64) -> bool = match op {
                BinOp::Lt => |x, y| x < y,
                BinOp::Le => |x, y| x <= y,
                BinOp::Gt => |x, y| x > y,
                _ => |x, y| x >= y,
            };
            let (r, c, v) = map2(name, a, b, |x, y| if f(x, y) { 1.0 } else { 0.0 })?;
            Ok(Mat::new(r, c, v).with_class(Class::Logical))
        }
        BinOp::And | BinOp::Or => {
            let and = op == BinOp::And;
            let nz = |m: &Mat, k: usize| m.re[k] != 0.0 || m.im_at(k) != 0.0;
            // for simplicity — via real «nonzero» flags
            let am = Mat::new(a.rows, a.cols, (0..a.numel()).map(|k| if nz(a, k) { 1.0 } else { 0.0 }).collect());
            let bm = Mat::new(b.rows, b.cols, (0..b.numel()).map(|k| if nz(b, k) { 1.0 } else { 0.0 }).collect());
            let (r, c, v) = map2(name, &am, &bm, |x, y| {
                let t = if and { x != 0.0 && y != 0.0 } else { x != 0.0 || y != 0.0 };
                if t { 1.0 } else { 0.0 }
            })?;
            Ok(Mat::new(r, c, v).with_class(Class::Logical))
        }
        _ => Err(MError::new(format!("operator {name}: not elementwise"))),
    }
}

/// Matrix multiplication.
pub fn matmul(a: &Mat, b: &Mat) -> Result<Mat, MError> {
    if a.is_scalar() || b.is_scalar() {
        return elementwise(BinOp::EMul, a, b);
    }
    if a.cols != b.rows {
        return Err(nonconformant("*", a, b));
    }
    let (m, k, n) = (a.rows, a.cols, b.cols);
    if (a.is_complex() || b.is_complex()) && m * n * k >= 32 * 32 * 32 {
        // (Ar + iAi)(Br + iBi) — four real products on faer
        let ar = Mat::new(m, k, a.re.clone());
        let ai = Mat::new(m, k, a.im_or_zeros());
        let br = Mat::new(k, n, b.re.clone());
        let bi = Mat::new(k, n, b.im_or_zeros());
        let rr = matmul(&ar, &br)?;
        let ii = matmul(&ai, &bi)?;
        let ri = matmul(&ar, &bi)?;
        let ir = matmul(&ai, &br)?;
        let re: Vec<f64> = rr.re.iter().zip(&ii.re).map(|(x, y)| x - y).collect();
        let im: Vec<f64> = ri.re.iter().zip(&ir.re).map(|(x, y)| x + y).collect();
        return Ok(Mat::complex(m, n, re, im));
    }
    if a.is_complex() || b.is_complex() {
        let (ai, bi) = (a.im_or_zeros(), b.im_or_zeros());
        let mut re = vec![0.0; m * n];
        let mut im = vec![0.0; m * n];
        for j in 0..n {
            for p in 0..k {
                let (br, bim) = (b.re[j * k + p], bi[j * k + p]);
                for i in 0..m {
                    let (ar, aim) = (a.re[p * m + i], ai[p * m + i]);
                    re[j * m + i] += ar * br - aim * bim;
                    im[j * m + i] += ar * bim + aim * br;
                }
            }
        }
        return Ok(Mat::complex(m, n, re, im));
    }
    if m * n * k >= 32 * 32 * 32 {
        let c = linalg::view(a) * linalg::view(b);
        return Ok(linalg::from_faer(c.as_ref()));
    }
    let mut out = vec![0.0; m * n];
    for j in 0..n {
        for p in 0..k {
            let bv = b.re[j * k + p];
            if bv == 0.0 {
                continue;
            }
            let col = &a.re[p * m..(p + 1) * m];
            let o = &mut out[j * m..(j + 1) * m];
            for i in 0..m {
                o[i] += col[i] * bv;
            }
        }
    }
    Ok(Mat::new(m, n, out))
}

pub fn unary(op: UnOp, v: &Value) -> Result<Value, MError> {
    let m = match v {
        Value::Mat(m) => m,
        other => return Err(MError::new(format!("unary operator not implemented for '{}' operations", other.class_name()))),
    };
    Ok(Value::Mat(match op {
        UnOp::Neg => Mat {
            rows: m.rows,
            cols: m.cols,
            re: m.re.iter().map(|x| -x).collect(),
            im: m.im.as_ref().map(|v| v.iter().map(|x| -x).collect()),
            class: Class::Double,
        },
        UnOp::Plus => {
            let mut c = m.clone();
            c.class = Class::Double;
            c
        }
        UnOp::Not => Mat::new(
            m.rows,
            m.cols,
            (0..m.numel()).map(|k| if m.re[k] == 0.0 && m.im_at(k) == 0.0 { 1.0 } else { 0.0 }).collect(),
        )
        .with_class(Class::Logical),
    }))
}

/// v2: string arrays — only == and ~= (elementwise, a scalar is broadcast; a missing value equals nothing).
fn str_binary(op: BinOp, a: &Value, b: &Value) -> Result<Value, MError> {
    let parts = |v: &Value| -> Result<(usize, usize, Vec<Option<String>>), MError> {
        match v {
            Value::Str(s) => Ok((s.rows, s.cols, s.data.clone())),
            Value::Mat(m) if m.class == Class::Char && m.rows <= 1 => Ok((1, 1, vec![Some(m.to_string_lossy())])),
            other => Err(MError::new(format!("binary operator '{}' not implemented for 'string' by '{}' operations", op.text(), other.class_name()))),
        }
    };
    if !matches!(op, BinOp::Eq | BinOp::Ne) {
        return Err(MError::new(format!("binary operator '{}' not implemented for 'string' operations", op.text())));
    }
    let (ra, ca, da) = parts(a)?;
    let (rb, cb, db) = parts(b)?;
    let (r, c) = if da.len() == 1 { (rb, cb) } else { (ra, ca) };
    if da.len() != 1 && db.len() != 1 && (ra, ca) != (rb, cb) {
        return Err(MError::new(format!("nonconformant arguments (op1 is {ra}x{ca}, op2 is {rb}x{cb})")));
    }
    let n = r * c;
    let re = (0..n)
        .map(|k| {
            let x = &da[if da.len() == 1 { 0 } else { k }];
            let y = &db[if db.len() == 1 { 0 } else { k }];
            let eq = matches!((x, y), (Some(p), Some(q)) if p == q);
            let v = if op == BinOp::Eq { eq } else { !eq };
            if v { 1.0 } else { 0.0 }
        })
        .collect();
    Ok(Value::Mat(Mat { rows: r, cols: c, re, im: None, class: Class::Logical }))
}

pub fn transpose(v: &Value, conj: bool) -> Result<Value, MError> {
    if let Value::Str(s) = v {
        let mut data = Vec::with_capacity(s.data.len());
        for i in 0..s.rows {
            for j in 0..s.cols {
                data.push(s.data[j * s.rows + i].clone());
            }
        }
        return Ok(Value::Str(std::rc::Rc::new(crate::value::StrArr { rows: s.cols, cols: s.rows, data })));
    }
    match v {
        Value::Mat(m) => {
            let mut t = m.transpose();
            if conj {
                if let Some(im) = &mut t.im {
                    for x in im.iter_mut() {
                        *x = -*x;
                    }
                }
            }
            Ok(Value::Mat(t))
        }
        other => Err(MError::new(format!("transpose not defined for {}", other.class_name()))),
    }
}

/// Binary operator (except && and ||).
pub fn binary(interp: &mut Interp, op: BinOp, a: &Value, b: &Value) -> Result<Value, MError> {
    if matches!(a, Value::Str(_)) || matches!(b, Value::Str(_)) {
        return str_binary(op, a, b);
    }
    let am = mat_of(a, op.text())?;
    let bm = mat_of(b, op.text())?;
    match op {
        BinOp::Mul => matmul(am, bm).map(Value::Mat),
        BinOp::Div => {
            if bm.is_scalar() {
                return elementwise(BinOp::EDiv, am, bm).map(Value::Mat);
            }
            if am.cols != bm.cols {
                return Err(nonconformant("/", am, bm));
            }
            // A / B = (B' \ A')'
            let x = linalg::mldivide(interp, &bm.transpose(), &am.transpose())?;
            Ok(Value::Mat(x.transpose()))
        }
        BinOp::LDiv => {
            if am.is_scalar() {
                return elementwise(BinOp::ELDiv, am, bm).map(Value::Mat);
            }
            if am.rows != bm.rows {
                return Err(nonconformant("\\", am, bm));
            }
            linalg::mldivide(interp, am, bm).map(Value::Mat)
        }
        BinOp::Pow => mpower(interp, am, bm).map(Value::Mat),
        _ => elementwise(op, am, bm).map(Value::Mat),
    }
}

fn mpower(interp: &mut Interp, a: &Mat, b: &Mat) -> Result<Mat, MError> {
    if a.is_scalar() && b.is_scalar() {
        return elementwise(BinOp::EPow, a, b);
    }
    if b.is_scalar() && a.rows == a.cols && !b.is_complex() {
        let p = b.re[0];
        if p == p.trunc() {
            let mut base = if p < 0.0 { linalg::inv_checked(interp, a)? } else { a.clone() };
            let mut n = p.abs() as u64;
            let mut acc = linalg::eye(a.rows);
            while n > 0 {
                if n & 1 == 1 {
                    acc = matmul(&acc, &base)?;
                }
                n >>= 1;
                if n > 0 {
                    base = matmul(&base, &base)?;
                }
            }
            return Ok(acc);
        }
        return Err(MError::new("mpower: non-integer powers of matrices are not supported in mlab v1"));
    }
    Err(MError::new(
        "for x^y, only square matrix arguments are permitted and one argument must be scalar.  Use .^ for elementwise power.",
    ))
}

/// Range a:s:b without materialization: n elements, the k-th — `at(k)` (VM v1 runs a `for` loop over it lazily).
#[derive(Clone, Copy, Debug)]
pub struct RangeSpec {
    pub a: f64,
    pub s: f64,
    pub last: f64,
    pub n: usize,
}

impl RangeSpec {
    /// k-th element: «from both ends», so that the last one is exactly b.
    #[inline]
    pub fn at(&self, k: usize) -> f64 {
        if k < self.n / 2 { self.a + k as f64 * self.s } else { self.last - (self.n - 1 - k) as f64 * self.s }
    }
}

pub fn range_spec(a: f64, s: f64, b: f64) -> RangeSpec {
    if s == 0.0 || a.is_nan() || s.is_nan() || b.is_nan() || (s > 0.0 && a > b) || (s < 0.0 && a < b) {
        return RangeSpec { a, s, last: a, n: 0 };
    }
    let span = (b - a) / s;
    let tol = 3.0 * f64::EPSILON * span.abs().max(1.0);
    let n = (span + tol).floor() as usize + 1;
    let mut last = a + (n as f64 - 1.0) * s;
    if (last - b).abs() <= 3.0 * f64::EPSILON * a.abs().max(b.abs()) {
        last = b;
    }
    RangeSpec { a, s, last, n }
}

/// Value of the range a:s:b (the «from both ends» algorithm, so that the last element is exactly b).
pub fn range_values(a: f64, s: f64, b: f64) -> Vec<f64> {
    let r = range_spec(a, s, b);
    (0..r.n).map(|k| r.at(k)).collect()
}

pub fn range(a: &Value, s: Option<&Value>, b: &Value) -> Result<Value, MError> {
    let first = |v: &Value| -> Result<(f64, bool), MError> {
        let m = mat_of(v, ":")?;
        if m.is_empty() {
            return Ok((f64::NAN, false));
        }
        Ok((m.re[0], m.class == Class::Char))
    };
    let (av, ac) = first(a)?;
    let (bv, bc) = first(b)?;
    let sv = match s {
        Some(s) => first(s)?.0,
        None => 1.0,
    };
    let vals = range_values(av, sv, bv);
    let n = vals.len();
    let class = if ac && bc && s.is_none() { Class::Char } else { Class::Double };
    Ok(Value::Mat(Mat::new(if n == 0 { 1 } else { 1 }, n, vals).with_class(class)))
}

/// Concatenation of the rows of a matrix literal.
pub fn concat(rows: Vec<Vec<Value>>) -> Result<Value, MError> {
    let mut row_mats: Vec<Mat> = Vec::new();
    for row in rows {
        let mut mats: Vec<Mat> = Vec::new();
        for v in row {
            match v {
                Value::Mat(m) => mats.push(m),
                Value::Func(_) => return Err(MError::new("concatenation operator not implemented for 'function handle' by 'matrix' operations")),
                other => return Err(MError::new(format!("concatenation not implemented for '{}'", other.class_name()))),
            }
        }
        row_mats.push(hcat(mats)?);
    }
    Ok(Value::Mat(vcat(row_mats)?))
}

fn result_class(ms: &[Mat]) -> Class {
    if ms.iter().any(|m| m.class == Class::Char) {
        Class::Char
    } else if !ms.is_empty() && ms.iter().all(|m| m.class == Class::Logical) {
        Class::Logical
    } else {
        Class::Double
    }
}

pub fn hcat(mats: Vec<Mat>) -> Result<Mat, MError> {
    let class = result_class(&mats);
    let nonempty: Vec<Mat> = mats.iter().filter(|m| !m.is_empty()).cloned().collect();
    if nonempty.is_empty() {
        let mut m = mats.into_iter().find(|m| m.rows > 0 || m.cols > 0).unwrap_or_else(Mat::empty);
        m.class = class;
        return Ok(m);
    }
    let r = nonempty[0].rows;
    for m in &nonempty[1..] {
        if m.rows != r {
            return Err(MError::new(format!(
                "horizontal dimensions mismatch ({}x{} vs {}x{})",
                nonempty[0].rows, nonempty[0].cols, m.rows, m.cols
            )));
        }
    }
    let complex = nonempty.iter().any(|m| m.is_complex());
    let cols: usize = nonempty.iter().map(|m| m.cols).sum();
    let mut re = Vec::with_capacity(r * cols);
    let mut im = if complex { Some(Vec::with_capacity(r * cols)) } else { None };
    for m in &nonempty {
        re.extend_from_slice(&m.re);
        if let Some(im) = &mut im {
            im.extend(m.im_or_zeros());
        }
    }
    let mut out = Mat { rows: r, cols, re, im, class };
    out.narrow();
    Ok(out)
}

pub fn vcat(mats: Vec<Mat>) -> Result<Mat, MError> {
    let class = result_class(&mats);
    let nonempty: Vec<Mat> = mats.iter().filter(|m| !m.is_empty()).cloned().collect();
    if nonempty.is_empty() {
        let mut m = mats.into_iter().find(|m| m.rows > 0 || m.cols > 0).unwrap_or_else(Mat::empty);
        m.class = class;
        return Ok(m);
    }
    if nonempty.len() == 1 {
        let mut m = nonempty.into_iter().next().unwrap();
        m.class = class;
        return Ok(m);
    }
    let c = nonempty[0].cols;
    for m in &nonempty[1..] {
        if m.cols != c {
            return Err(MError::new(format!(
                "vertical dimensions mismatch ({}x{} vs {}x{})",
                nonempty[0].rows, nonempty[0].cols, m.rows, m.cols
            )));
        }
    }
    let complex = nonempty.iter().any(|m| m.is_complex());
    let rows: usize = nonempty.iter().map(|m| m.rows).sum();
    let mut re = vec![0.0; rows * c];
    let mut im = if complex { Some(vec![0.0; rows * c]) } else { None };
    let mut r0 = 0;
    for m in &nonempty {
        let mi = m.im_or_zeros();
        for j in 0..c {
            for i in 0..m.rows {
                re[j * rows + r0 + i] = m.re[j * m.rows + i];
                if let Some(im) = &mut im {
                    im[j * rows + r0 + i] = mi[j * m.rows + i];
                }
            }
        }
        r0 += m.rows;
    }
    let mut out = Mat { rows, cols: c, re, im, class };
    out.narrow();
    Ok(out)
}

/// Logical value of an if/while condition: nonempty and all elements nonzero.
pub fn truthy(v: &Value) -> Result<bool, MError> {
    match v {
        Value::Mat(m) => Ok(!m.is_empty() && (0..m.numel()).all(|k| m.re[k] != 0.0 || m.im_at(k) != 0.0)),
        other => Err(MError::new(format!("wrong type argument '{}' in condition", other.class_name()))),
    }
}
