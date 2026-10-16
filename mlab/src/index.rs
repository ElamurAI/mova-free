//! Indexing: reading `A(I)`, `A(I,J)`, assignment with matrix growth, deletion `A(I) = []`.
//! 1-based indices; logical masks; `:`; result shape per the language rules
//! (vector by vector — orientation of the source, otherwise — shape of the index; `A(:)` — a column).

use crate::interp::MError;
use crate::value::{Class, Mat};

#[derive(Clone, Debug)]
pub enum IdxArg {
    All,
    Vals(Mat),
}

fn fmt_num(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}

/// Index position in a message: `v(1.5)`, `A(1.5,_)`, `A(_,3)`.
fn pos_str(name: &str, pos: usize, nargs: usize, val: &str) -> String {
    if nargs <= 1 {
        return format!("{name}({val})");
    }
    let parts: Vec<String> = (0..nargs).map(|k| if k == pos { val.to_string() } else { "_".to_string() }).collect();
    format!("{name}({})", parts.join(","))
}

/// Converts an argument into a list of 0-based indices (without an upper-bound check).
/// For a logical mask — positions of true elements; also returns the orientation (true = row).
pub fn to_indices(arg: &IdxArg, extent: usize, name: &str, pos: usize, nargs: usize) -> Result<(Vec<usize>, Option<(usize, usize)>), MError> {
    match arg {
        IdxArg::All => Ok(((0..extent).collect(), None)),
        IdxArg::Vals(m) => {
            if m.class == Class::Logical {
                let idx: Vec<usize> = (0..m.numel()).filter(|&k| m.re[k] != 0.0).collect();
                let n = idx.len();
                let shape = if m.rows == 1 { (1, n) } else { (n, 1) };
                return Ok((idx, Some(shape)));
            }
            let mut out = Vec::with_capacity(m.numel());
            for &v in &m.re {
                if v != v.trunc() || v < 1.0 || !v.is_finite() {
                    let at = pos_str(name, pos, nargs, &fmt_num(v));
                    let _ = extent;
                    return Err(MError::new(format!("{at}: subscripts must be either integers 1 to (2^63)-1 or logicals")));
                }
                out.push(v as usize - 1);
            }
            Ok((out, Some((m.rows, m.cols))))
        }
    }
}

fn oob(name: &str, pos: usize, nargs: usize, idx1: usize, bound: usize, m: &Mat) -> MError {
    let at = pos_str(name, pos, nargs, &idx1.to_string());
    MError::new(format!("{at}: out of bound {bound} (dimensions are {}x{})", m.rows, m.cols))
}

fn gather(m: &Mat, idx: &[usize], rows: usize, cols: usize) -> Mat {
    let re: Vec<f64> = idx.iter().map(|&k| m.re[k]).collect();
    let im = m.im.as_ref().map(|v| idx.iter().map(|&k| v[k]).collect());
    let mut out = Mat { rows, cols, re, im, class: m.class };
    out.narrow();
    out
}

pub fn read(m: &Mat, args: &[IdxArg], name: &str) -> Result<Mat, MError> {
    match args.len() {
        0 => Ok(m.clone()),
        1 => {
            let n = m.numel();
            let (idx, shape) = to_indices(&args[0], n, name, 0, 1)?;
            for &k in &idx {
                if k >= n {
                    return Err(oob(name, 0, 1, k + 1, n, m));
                }
            }
            let cnt = idx.len();
            let (r, c) = match (&args[0], shape) {
                (IdxArg::All, _) => (n, 1),
                (_, Some((ir, ic))) => {
                    let idx_vec = ir == 1 || ic == 1;
                    let src_vec = m.rows == 1 || m.cols == 1;
                    if idx_vec && src_vec && !m.is_scalar() {
                        if m.rows == 1 { (1, cnt) } else { (cnt, 1) }
                    } else {
                        (ir, ic)
                    }
                }
                _ => (cnt, 1),
            };
            Ok(gather(m, &idx, r, c))
        }
        _ => {
            let nargs = args.len();
            let (ri, _) = to_indices(&args[0], m.rows, name, 0, nargs)?;
            let last_extent = if nargs == 2 { m.cols } else { m.cols };
            let (ci, _) = to_indices(&args[1], last_extent, name, 1, nargs)?;
            for &i in &ri {
                if i >= m.rows {
                    return Err(oob(name, 0, nargs, i + 1, m.rows, m));
                }
            }
            for &j in &ci {
                if j >= m.cols {
                    return Err(oob(name, 1, nargs, j + 1, m.cols, m));
                }
            }
            for (p, a) in args.iter().enumerate().skip(2) {
                let (ks, _) = to_indices(a, 1, name, p, nargs)?;
                if let Some(&k) = ks.iter().find(|&&k| k >= 1) {
                    return Err(oob(name, p, nargs, k + 1, 1, m));
                }
            }
            let mut idx = Vec::with_capacity(ri.len() * ci.len());
            for &j in &ci {
                for &i in &ri {
                    idx.push(j * m.rows + i);
                }
            }
            Ok(gather(m, &idx, ri.len(), ci.len()))
        }
    }
}

fn merged_class(target: &Mat, target_was_empty: bool, v: &Mat) -> Class {
    if target_was_empty {
        return v.class;
    }
    match (target.class, v.class) {
        (Class::Char, _) => Class::Char,
        (Class::Logical, Class::Logical) => Class::Logical,
        _ => Class::Double,
    }
}

fn resize(m: &Mat, rows: usize, cols: usize) -> Mat {
    if rows == m.rows && cols == m.cols {
        return m.clone();
    }
    let mut re = vec![0.0; rows * cols];
    let mut im = m.im.as_ref().map(|_| vec![0.0; rows * cols]);
    for j in 0..m.cols.min(cols) {
        for i in 0..m.rows.min(rows) {
            re[j * rows + i] = m.re[j * m.rows + i];
            if let (Some(dst), Some(src)) = (&mut im, &m.im) {
                dst[j * rows + i] = src[j * m.rows + i];
            }
        }
    }
    Mat { rows, cols, re, im, class: m.class }
}

fn is_delete(v: &Mat) -> bool {
    v.rows == 0 && v.cols == 0
}

/// In-place indexed assignment. Check first, then modify: on error `target` is unchanged.
/// `defined` — whether the variable already existed (an undefined one behaves as `[]`).
pub fn assign(target: &mut Mat, defined: bool, args: &[IdxArg], v: &Mat, name: &str) -> Result<(), MError> {
    let was_empty = !defined || target.numel() == 0;
    if is_delete(v) {
        let out = delete(target, args, name)?;
        *target = out;
        return Ok(());
    }
    match args.len() {
        0 => {
            *target = v.clone();
            Ok(())
        }
        1 => assign_linear(target, was_empty, &args[0], v, name),
        _ => {
            let nargs = args.len();
            for (p, a) in args.iter().enumerate().skip(2) {
                let (ks, _) = to_indices(a, 1, name, p, nargs)?;
                if ks.iter().any(|&k| k >= 1) {
                    return Err(MError::new("N-dimensional arrays are not supported in mlab v1"));
                }
            }
            assign_2d(target, was_empty, &args[0], &args[1], v, name)
        }
    }
}

fn assign_linear(m: &mut Mat, was_empty: bool, arg: &IdxArg, v: &Mat, name: &str) -> Result<(), MError> {
    let n = m.numel();
    let (idx, _) = to_indices(arg, n, name, 0, 1)?;
    let cnt = idx.len();
    if !(v.is_scalar() || v.numel() == cnt) {
        return Err(MError::new(format!(
            "=: nonconformant arguments (op1 is 1x{cnt}, op2 is {}x{})",
            v.rows, v.cols
        )));
    }
    if let Some(mx) = idx.iter().copied().max() {
        if mx >= n {
            let need = mx + 1;
            let grown = if n == 0 {
                if m.cols == 1 && m.rows == 0 {
                    resize(m, need, 1)
                } else {
                    let row0 = Mat { rows: 1, cols: 0, re: vec![], im: m.im.as_ref().map(|_| vec![]), class: m.class };
                    resize(&row0, 1, need)
                }
            } else if m.rows == 1 {
                resize(m, 1, need)
            } else if m.cols == 1 {
                resize(m, need, 1)
            } else {
                return Err(MError::new(format!(
                    "{name}({need}): cannot resize a {}x{} matrix by a linear index (out of bound {n})",
                    m.rows, m.cols
                )));
            };
            *m = grown;
        }
    }
    m.class = merged_class(m, was_empty, v);
    write(m, &idx, v);
    Ok(())
}

fn write(m: &mut Mat, idx: &[usize], v: &Mat) {
    if v.is_complex() && m.im.is_none() {
        m.im = Some(vec![0.0; m.re.len()]);
    }
    for (p, &k) in idx.iter().enumerate() {
        let s = if v.is_scalar() { 0 } else { p };
        m.re[k] = v.re[s];
        if let Some(im) = &mut m.im {
            im[k] = v.im_at(s);
        }
    }
    m.narrow();
}

fn assign_2d(m: &mut Mat, was_empty: bool, a0: &IdxArg, a1: &IdxArg, v: &Mat, name: &str) -> Result<(), MError> {
    // «:» on an empty matrix takes its size from the right-hand side
    let rows_extent = if matches!(a0, IdxArg::All) && m.numel() == 0 {
        if v.is_scalar() { m.rows.max(1) } else { v.rows }
    } else {
        m.rows
    };
    let cols_extent = if matches!(a1, IdxArg::All) && m.numel() == 0 {
        if v.is_scalar() { m.cols.max(1) } else { v.cols }
    } else {
        m.cols
    };
    let (ri, _) = to_indices(a0, rows_extent, name, 0, 2)?;
    let (ci, _) = to_indices(a1, cols_extent, name, 1, 2)?;
    let (ni, nj) = (ri.len(), ci.len());
    let ok = v.is_scalar()
        || (v.rows == ni && v.cols == nj)
        || (v.numel() == ni * nj && (ni == 1 || nj == 1) && (v.rows == 1 || v.cols == 1));
    if !ok {
        return Err(MError::new(format!(
            "=: nonconformant arguments (op1 is {ni}x{nj}, op2 is {}x{})",
            v.rows, v.cols
        )));
    }
    let new_r = ri.iter().map(|&i| i + 1).max().unwrap_or(0).max(m.rows);
    let new_c = ci.iter().map(|&j| j + 1).max().unwrap_or(0).max(m.cols);
    let class = merged_class(m, was_empty, v);
    if new_r != m.rows || new_c != m.cols {
        *m = resize(m, new_r, new_c);
    }
    m.class = class;
    let mut idx = Vec::with_capacity(ni * nj);
    for &j in &ci {
        for &i in &ri {
            idx.push(j * new_r + i);
        }
    }
    write(m, &idx, v);
    Ok(())
}


fn delete(m: &Mat, args: &[IdxArg], name: &str) -> Result<Mat, MError> {
    match args.len() {
        1 => {
            let n = m.numel();
            if matches!(args[0], IdxArg::All) {
                return Ok(Mat { rows: 0, cols: 0, re: vec![], im: None, class: m.class });
            }
            let (idx, _) = to_indices(&args[0], n, name, 0, 1)?;
            let mut del = vec![false; n];
            for &k in &idx {
                if k >= n {
                    return Err(oob(name, 0, 1, k + 1, n, m));
                }
                del[k] = true;
            }
            let keep: Vec<usize> = (0..n).filter(|&k| !del[k]).collect();
            let cnt = keep.len();
            let (r, c) = if m.cols == 1 && m.rows != 1 { (cnt, 1) } else { (1, cnt) };
            Ok(gather(m, &keep, r, c))
        }
        _ => {
            let nargs = args.len();
            let (ri, _) = to_indices(&args[0], m.rows, name, 0, nargs)?;
            let (ci, _) = to_indices(&args[1], m.cols, name, 1, nargs)?;
            let all_rows = matches!(args[0], IdxArg::All) || {
                let mut s = ri.clone();
                s.sort_unstable();
                s.dedup();
                s.len() == m.rows && s.iter().enumerate().all(|(a, &b)| a == b)
            };
            let all_cols = matches!(args[1], IdxArg::All) || {
                let mut s = ci.clone();
                s.sort_unstable();
                s.dedup();
                s.len() == m.cols && s.iter().enumerate().all(|(a, &b)| a == b)
            };
            if all_rows {
                let mut del = vec![false; m.cols];
                for &j in &ci {
                    if j >= m.cols {
                        return Err(oob(name, 1, nargs, j + 1, m.cols, m));
                    }
                    del[j] = true;
                }
                let keep: Vec<usize> = (0..m.cols).filter(|&j| !del[j]).collect();
                let mut idx = Vec::new();
                for &j in &keep {
                    for i in 0..m.rows {
                        idx.push(j * m.rows + i);
                    }
                }
                Ok(gather(m, &idx, m.rows, keep.len()))
            } else if all_cols {
                let mut del = vec![false; m.rows];
                for &i in &ri {
                    if i >= m.rows {
                        return Err(oob(name, 0, nargs, i + 1, m.rows, m));
                    }
                    del[i] = true;
                }
                let keep: Vec<usize> = (0..m.rows).filter(|&i| !del[i]).collect();
                let mut idx = Vec::new();
                for j in 0..m.cols {
                    for &i in &keep {
                        idx.push(j * m.rows + i);
                    }
                }
                Ok(gather(m, &idx, keep.len(), m.cols))
            } else {
                Err(MError::new("a null assignment can only have one non-colon index"))
            }
        }
    }
}
