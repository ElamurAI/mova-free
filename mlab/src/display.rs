//! Octave-style value display (format short — 5 significant digits, format long — 16).
//!
//! Our format specification (from documentation and output examples; Octave's code was not read):
//! - integer values — as integers; column width = digits of the largest + 1 (sign position), 2 spaces between columns;
//! - non-integers: d = number of digits before the decimal point (floor(log10|x|)+1 after rounding to prec significant digits);
//!   d ≥ 1 → prec−d decimals; d = 0 → prec−1; d < 0 → prec−d; a matrix — a common format
//!   by the largest and smallest nonzero magnitude; an exact zero is printed as «0»;
//! - e-format (prec−1 mantissa digits) when d_max ≥ prec, d_min ≤ −2 or the common format is too long;
//! - logical — width 1; char — as rows; empty — `[](RxC)`;
//! - complex — `re + imi` with a common format for the parts.

use crate::ast::{Expr, UnOp, PostOp};
use crate::value::{Class, Func, Mat, Value};

pub const TERM_WIDTH: usize = 80;

#[derive(Clone, Copy, Debug)]
pub struct Format {
    pub long: bool,
}

impl Format {
    pub fn prec(&self) -> usize {
        if self.long { 16 } else { 5 }
    }
}

#[derive(Clone, Copy, Debug)]
enum RealFmt {
    Int { fw: usize },
    Fixed { fw: usize, rd: usize },
    Exp { fw: usize, rd: usize },
}

fn int_digits(x: f64) -> usize {
    // x — integer ≥ 0
    if x < 1.0 {
        1
    } else {
        format!("{:.0}", x).len()
    }
}

fn round_sig(x: f64, p: usize) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    let s = format!("{:.*e}", p.saturating_sub(1), x);
    s.parse().unwrap_or(x)
}

/// Number of digits before the decimal point for x > 0 (may be ≤ 0 for x < 1).
fn digits_of(x: f64, prec: usize) -> i32 {
    let r = round_sig(x, prec);
    let s = format!("{:e}", r);
    let exp: i32 = s.split('e').nth(1).and_then(|e| e.parse().ok()).unwrap_or(0);
    exp + 1
}

fn ld_rd(d: i32, prec: usize) -> (usize, usize) {
    if d > 0 {
        let d = d as usize;
        (d, if d < prec { prec - d } else { 1 })
    } else if d == 0 {
        (1, prec - 1)
    } else {
        (1, prec + (-d) as usize)
    }
}

fn choose_fmt(values: &[f64], prec: usize) -> RealFmt {
    let mut has_nonfinite = false;
    let mut all_int = true;
    let mut maxabs: f64 = 0.0;
    let mut minabs_nz = f64::INFINITY;
    for &x in values {
        if !x.is_finite() {
            has_nonfinite = true;
            continue;
        }
        if x != x.trunc() {
            all_int = false;
        }
        let a = x.abs();
        if a > maxabs {
            maxabs = a;
        }
        if a > 0.0 && a < minabs_nz {
            minabs_nz = a;
        }
    }
    let exp_fmt = |maxabs: f64, minabs: f64| -> RealFmt {
        let rd = prec - 1;
        let big_exp = (maxabs > 0.0 && (maxabs >= 1e100 || (minabs.is_finite() && minabs < 1e-99))) as usize;
        RealFmt::Exp { fw: 1 + 1 + 1 + rd + 4 + big_exp, rd }
    };
    if all_int {
        let digits = int_digits(maxabs);
        if digits > 15 {
            return exp_fmt(maxabs, minabs_nz);
        }
        let mut fw = digits + 1;
        if has_nonfinite {
            fw = fw.max(4);
        }
        return RealFmt::Int { fw };
    }
    let dmax = digits_of(maxabs, prec);
    let dmin = digits_of(minabs_nz, prec);
    let (l1, r1) = ld_rd(dmax, prec);
    let (l2, r2) = ld_rd(dmin, prec);
    let ld = l1.max(l2);
    let rd = r1.max(r2);
    if dmax >= prec as i32 || dmin <= -2 || ld + rd > prec + 3 {
        return exp_fmt(maxabs, minabs_nz);
    }
    let mut fw = 1 + ld + 1 + rd;
    if has_nonfinite {
        fw = fw.max(4);
    }
    RealFmt::Fixed { fw, rd }
}

fn nonfinite_str(x: f64) -> Option<&'static str> {
    if x.is_nan() {
        Some("NaN")
    } else if x == f64::INFINITY {
        Some("Inf")
    } else if x == f64::NEG_INFINITY {
        Some("-Inf")
    } else {
        None
    }
}

pub fn exp_str(x: f64, rd: usize) -> String {
    let s = format!("{:.*e}", rd, x);
    let (m, e) = s.split_once('e').unwrap_or((&s, "0"));
    let ev: i32 = e.parse().unwrap_or(0);
    let sign = if ev < 0 { '-' } else { '+' };
    format!("{m}e{sign}{:02}", ev.abs())
}

fn fmt_real(x: f64, f: RealFmt) -> String {
    if let Some(s) = nonfinite_str(x) {
        return s.to_string();
    }
    match f {
        RealFmt::Int { .. } => {
            if x == 0.0 {
                "0".into()
            } else {
                format!("{:.0}", x)
            }
        }
        RealFmt::Fixed { rd, .. } => {
            if x == 0.0 {
                "0".into()
            } else {
                format!("{:.*}", rd, x)
            }
        }
        RealFmt::Exp { rd, .. } => {
            if x == 0.0 {
                "0".into()
            } else {
                exp_str(x, rd)
            }
        }
    }
}

fn fw_of(f: RealFmt) -> usize {
    match f {
        RealFmt::Int { fw } | RealFmt::Fixed { fw, .. } | RealFmt::Exp { fw, .. } => fw,
    }
}

/// Scalar without alignment (for `x = …` and disp).
pub fn scalar_str(m: &Mat, k: usize, fmt: Format) -> String {
    let x = m.re[k];
    if m.class == Class::Logical {
        return if x != 0.0 { "1".into() } else { "0".into() };
    }
    if m.class == Class::Char {
        return char::from_u32(x as u32).map(|c| c.to_string()).unwrap_or_default();
    }
    if let Some(im) = &m.im {
        let cf = choose_cfmt(&[x], &[im[k]], fmt.prec());
        let (r, i) = cfmt_parts(x, im[k], cf);
        let sep = if im[k] < 0.0 || (im[k] == 0.0 && im[k].is_sign_negative()) { " - " } else { " + " };
        return format!("{r}{sep}{i}i");
    }
    fmt_real(x, choose_fmt(&[x], fmt.prec()))
}

// ---------- complex ----------

#[derive(Clone, Copy)]
struct CFmt {
    f: RealFmt,
    fw_re: usize,
    fw_im: usize,
}

fn choose_cfmt(re: &[f64], im: &[f64], prec: usize) -> CFmt {
    let mut all: Vec<f64> = Vec::with_capacity(re.len() * 2);
    all.extend_from_slice(re);
    all.extend_from_slice(im);
    let f = choose_fmt(&all, prec);
    let (fw_re, fw_im) = match f {
        RealFmt::Int { fw } => (fw, fw - 1),
        RealFmt::Fixed { fw, .. } => (fw, fw - 1),
        RealFmt::Exp { fw, .. } => (fw, fw - 1),
    };
    CFmt { f, fw_re, fw_im }
}

fn cfmt_parts(re: f64, im: f64, cf: CFmt) -> (String, String) {
    let r = fmt_real(re, cf.f);
    let i = fmt_real(im.abs(), cf.f);
    (r, i)
}

// ---------- matrix rows ----------

/// Cells of matrix rows: each cell already includes leading separator spaces.
pub(crate) fn matrix_cells(m: &Mat, fmt: Format) -> (Vec<Vec<String>>, usize) {
    let (r, c) = (m.rows, m.cols);
    let mut cells = vec![Vec::with_capacity(c); r];
    let width;
    match m.class {
        Class::Logical => {
            width = 3;
            for i in 0..r {
                for j in 0..c {
                    cells[i].push(format!("  {}", if m.at(i, j) != 0.0 { 1 } else { 0 }));
                }
            }
        }
        _ => {
            if let Some(im) = &m.im {
                let cf = choose_cfmt(&m.re, im, fmt.prec());
                width = 2 + cf.fw_re + 3 + cf.fw_im + 1;
                for i in 0..r {
                    for j in 0..c {
                        let k = j * r + i;
                        let (rs, is) = cfmt_parts(m.re[k], im[k], cf);
                        let sep = if im[k] < 0.0 { " - " } else { " + " };
                        cells[i].push(format!("  {:>w1$}{sep}{:>w2$}i", rs, is, w1 = cf.fw_re, w2 = cf.fw_im));
                    }
                }
            } else {
                let f = choose_fmt(&m.re, fmt.prec());
                let fw = fw_of(f);
                width = 2 + fw;
                for i in 0..r {
                    for j in 0..c {
                        cells[i].push(format!("  {:>fw$}", fmt_real(m.at(i, j), f)));
                    }
                }
            }
        }
    }
    (cells, width)
}

/// Matrix rows split into column blocks if the width > 80.
fn matrix_body(m: &Mat, fmt: Format, for_disp: bool) -> String {
    let (cells, width) = matrix_cells(m, fmt);
    let c = m.cols;
    let per = (TERM_WIDTH / width.max(1)).max(1);
    let mut s = String::new();
    if c <= per {
        for row in &cells {
            s.push_str(&row.concat());
            s.push('\n');
        }
        return s;
    }
    let mut start = 0;
    while start < c {
        let end = (start + per).min(c);
        let hdr = if end - start == 1 {
            format!(" Column {}:", start + 1)
        } else if end == c && end - start == 2 {
            format!(" Columns {} and {}:", start + 1, end)
        } else {
            format!(" Columns {} through {}:", start + 1, end)
        };
        s.push_str(&hdr);
        s.push_str("\n\n");
        for row in &cells {
            s.push_str(&row[start..end].concat());
            s.push('\n');
        }
        // an empty line between blocks; after the last one it is added by the `name = …` output itself
        if end < c {
            s.push('\n');
        }
        let _ = for_disp;
        start = end;
    }
    s
}

pub fn unparse_func(f: &Func) -> String {
    match f {
        Func::Named(n) => format!("@{n}"),
        Func::Anon { params, body, .. } => format!("@({}) {}", params.join(", "), unparse(body)),
    }
}

/// `name = …` after an assignment or an expression without «;».
pub fn format_named(name: &str, v: &Value, fmt: Format) -> String {
    match v {
        Value::Mat(m) => {
            if m.class == Class::Char {
                if m.rows == 0 || m.cols == 0 {
                    return format!("{name} = \n");
                }
                if m.rows == 1 {
                    return format!("{name} = {}\n", m.row_string(0));
                }
                let mut s = format!("{name} =\n\n");
                for i in 0..m.rows {
                    s.push_str(&m.row_string(i));
                    s.push('\n');
                }
                s.push('\n');
                return s;
            }
            if m.is_empty() {
                return format!("{name} = []({})\n", m.dims_str());
            }
            if m.is_scalar() {
                return format!("{name} = {}\n", scalar_str(m, 0, fmt));
            }
            format!("{name} =\n\n{}\n", matrix_body(m, fmt, false))
        }
        Value::Func(f) => match &**f {
            Func::Named(n) => format!("{name} = @{n}\n"),
            Func::Anon { .. } => format!("{name} =\n\n{}\n\n", unparse_func(f)),
        },
        Value::Table(t) => crate::table::format_named(name, t, fmt),
        Value::Str(s) => {
            if s.data.len() == 1 {
                return format!("{name} = {}\n", s.data[0].as_deref().map_or("<missing>".to_string(), |x| format!("\"{x}\"")));
            }
            let q = crate::value::StrArr {
                rows: s.rows,
                cols: s.cols,
                data: s.data.iter().map(|x| Some(x.as_deref().map_or("<missing>".to_string(), |x| format!("\"{x}\"")))).collect(),
            };
            let body: String = crate::table::format_str_arr(&q).lines().map(|l| format!("  {l}\n")).collect();
            format!("{name} =\n\n{body}\n")
        }
        Value::Exact(e) => match *e {},
    }
}


/// disp(x)
pub fn format_disp(v: &Value, fmt: Format) -> String {
    match v {
        Value::Mat(m) => {
            if m.class == Class::Char {
                let mut s = String::new();
                for i in 0..m.rows {
                    s.push_str(&m.row_string(i));
                    s.push('\n');
                }
                if m.rows == 0 {
                    s.push('\n');
                }
                return s;
            }
            if m.is_empty() {
                return String::new();
            }
            if m.is_scalar() {
                return format!("{}\n", scalar_str(m, 0, fmt));
            }
            matrix_body(m, fmt, true)
        }
        Value::Func(f) => format!("{}\n", unparse_func(f)),
        Value::Table(t) => crate::table::format_body(t, fmt),
        Value::Str(s) => crate::table::format_str_arr(s),
        Value::Exact(e) => match *e {},
    }
}

/// Printing an anonymous function expression in Octave style: binary operators with spaces, `f (x)` with a space.
pub fn unparse(e: &Expr) -> String {
    match e {
        Expr::Num(_, t) | Expr::Imag(_, t) => t.clone(),
        Expr::Str(s, _) => format!("'{}'", s.replace('\'', "''")),
        Expr::Ident(s) => s.clone(),
        Expr::Colon => ":".into(),
        Expr::End => "end".into(),
        Expr::Paren(e) => format!("({})", unparse(e)),
        Expr::Unary(op, e) => {
            let o = match op {
                UnOp::Neg => "-",
                UnOp::Plus => "+",
                UnOp::Not => "!",
            };
            format!("{o}{}", unparse(e))
        }
        Expr::Postfix(op, e) => {
            let o = match op {
                PostOp::CTranspose => "'",
                PostOp::Transpose => ".'",
            };
            format!("{}{o}", unparse(e))
        }
        Expr::Binary(op, l, r) => format!("{} {} {}", unparse(l), op.text(), unparse(r)),
        Expr::Range(a, s, b) => match s {
            Some(s) => format!("{}:{}:{}", unparse(a), unparse(s), unparse(b)),
            None => format!("{}:{}", unparse(a), unparse(b)),
        },
        Expr::Index(f, args) => {
            let a: Vec<String> = args.iter().map(unparse).collect();
            format!("{} ({})", unparse(f), a.join(", "))
        }
        Expr::Matrix(rows) => {
            let r: Vec<String> = rows.iter().map(|row| row.iter().map(unparse).collect::<Vec<_>>().join(", ")).collect();
            format!("[{}]", r.join("; "))
        }
        Expr::CellList(items) => format!("{{{}}}", items.iter().map(unparse).collect::<Vec<_>>().join(", ")),
        Expr::AnonFn(ps, body) => format!("@({}) {}", ps.join(", "), unparse(body)),
        Expr::FuncHandle(n) => format!("@{n}"),
        Expr::Field(e, n) => format!("{}.{n}", unparse(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn f(x: f64) -> String {
        scalar_str(&Mat::scalar(x), 0, Format { long: false })
    }
    #[test]
    fn scalars_short() {
        assert_eq!(f(3.0), "3");
        assert_eq!(f(std::f64::consts::PI), "3.1416");
        assert_eq!(f(-0.5), "-0.5000");
        assert_eq!(f(12.5), "12.500");
        assert_eq!(f(100.5), "100.50");
        assert_eq!(f(1234.5), "1234.5");
        assert_eq!(f(12345.678), "1.2346e+04");
        assert_eq!(f(0.001), "1.0000e-03");
        assert_eq!(f(0.05), "0.050000");
        assert_eq!(f(f64::NAN), "NaN");
        assert_eq!(f(-f64::INFINITY), "-Inf");
        assert_eq!(f(-0.0), "0");
    }
    #[test]
    fn matrix_rows() {
        let m = Mat::new(2, 2, vec![1.5, -3.0, 2.25, 4.0]);
        assert_eq!(matrix_body(&m, Format { long: false }, true), "   1.5000   2.2500\n  -3.0000   4.0000\n");
        let m = Mat::new(1, 3, vec![0.0, 0.25, 0.5]);
        assert_eq!(matrix_body(&m, Format { long: false }, true), "        0   0.2500   0.5000\n");
        let m = Mat::new(1, 2, vec![100.5, 2.0]);
        assert_eq!(matrix_body(&m, Format { long: false }, true), "   100.5000     2.0000\n");
    }
}
