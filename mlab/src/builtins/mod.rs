//! Builtin function registry — extensible: each module registers its names via `Registry::register`.
//! v2 (tables, statistics, import/export) will add the modules `table`, `stats`, `io_files` the same way.

use crate::interp::{Interp, MError};
use crate::value::{Class, Mat, Value};
use std::collections::HashMap;

pub mod core;
pub mod elem;
pub mod fft;
pub mod files;
pub mod io;
pub mod linalg;
pub mod numeric;
pub mod poly;
pub mod reduce;
pub mod stats;
pub mod table;

/// Builtin signature: interpreter (output, generator, handle calls), arguments, number of requested results.
pub type BuiltinFn = fn(&mut Interp, &[Value], usize) -> Result<Vec<Value>, MError>;

#[derive(Default)]
pub struct Registry {
    map: HashMap<&'static str, BuiltinFn>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry { map: HashMap::new() }
    }
    pub fn register(&mut self, name: &'static str, f: BuiltinFn) {
        self.map.insert(name, f);
    }
    pub fn get(&self, name: &str) -> Option<BuiltinFn> {
        self.map.get(name).copied()
    }
    pub fn names(&self) -> Vec<&'static str> {
        let mut v: Vec<&'static str> = self.map.keys().copied().collect();
        v.sort();
        v
    }
    pub fn with_defaults() -> Registry {
        let mut r = Registry::new();
        core::register(&mut r);
        elem::register(&mut r);
        reduce::register(&mut r);
        io::register(&mut r);
        linalg::register(&mut r);
        poly::register(&mut r);
        fft::register(&mut r);
        numeric::register(&mut r);
        table::register(&mut r);
        stats::register(&mut r);
        files::register(&mut r);
        r
    }
}

// ---------- argument helpers ----------

pub fn invalid_call(fname: &str) -> MError {
    MError::new(format!("Invalid call to {fname}"))
}

pub fn arg<'a>(args: &'a [Value], k: usize, fname: &str) -> Result<&'a Value, MError> {
    args.get(k).ok_or_else(|| invalid_call(fname))
}

pub fn mat_arg<'a>(args: &'a [Value], k: usize, fname: &str) -> Result<&'a Mat, MError> {
    match arg(args, k, fname)? {
        Value::Mat(m) => Ok(m),
        other => Err(MError::new(format!("{fname}: wrong type argument '{}'", other.class_name()))),
    }
}

pub fn scalar_arg(args: &[Value], k: usize, fname: &str) -> Result<f64, MError> {
    let m = mat_arg(args, k, fname)?;
    if m.numel() < 1 {
        return Err(MError::new(format!("{fname}: argument {} must be a scalar", k + 1)));
    }
    Ok(m.re[0])
}

pub fn is_str(v: &Value) -> bool {
    matches!(v, Value::Mat(m) if m.class == Class::Char)
}

pub fn str_of(v: &Value) -> Option<String> {
    match v {
        Value::Mat(m) if m.class == Class::Char => Some(m.to_string_lossy()),
        _ => None,
    }
}

pub fn str_arg(args: &[Value], k: usize, fname: &str) -> Result<String, MError> {
    str_of(arg(args, k, fname)?).ok_or_else(|| MError::new(format!("{fname}: argument {} must be a string", k + 1)))
}

pub fn one(m: Mat) -> Result<Vec<Value>, MError> {
    Ok(vec![Value::Mat(m)])
}

pub fn num(x: f64) -> Result<Vec<Value>, MError> {
    Ok(vec![Value::num(x)])
}

pub fn boolv(b: bool) -> Result<Vec<Value>, MError> {
    Ok(vec![Value::boolean(b)])
}

/// Sizes for zeros/ones/rand/…: (), (n), (m, n), ([m n]); string arguments (class) are skipped.
pub fn dims_args(args: &[Value], fname: &str) -> Result<(usize, usize), MError> {
    let nums: Vec<&Value> = args.iter().filter(|a| !is_str(a)).collect();
    let to_dim = |x: f64| -> Result<usize, MError> {
        if x.is_nan() {
            return Err(MError::new(format!("{fname}: NaN is invalid as size specification")));
        }
        Ok(if x <= 0.0 { 0 } else { x.floor() as usize })
    };
    match nums.len() {
        0 => Ok((1, 1)),
        1 => {
            let m = match nums[0] {
                Value::Mat(m) => m,
                _ => return Err(invalid_call(fname)),
            };
            match m.numel() {
                1 => {
                    let n = to_dim(m.re[0])?;
                    Ok((n, n))
                }
                2 => Ok((to_dim(m.re[0])?, to_dim(m.re[1])?)),
                0 => Ok((0, 0)),
                _ => {
                    if m.re[2..].iter().all(|&x| x == 1.0) {
                        Ok((to_dim(m.re[0])?, to_dim(m.re[1])?))
                    } else {
                        Err(MError::new(format!("{fname}: N-dimensional arrays are not supported in mlab v1")))
                    }
                }
            }
        }
        _ => {
            let mut ds = Vec::new();
            for v in &nums {
                match v {
                    Value::Mat(m) if m.numel() == 1 => ds.push(to_dim(m.re[0])?),
                    _ => return Err(invalid_call(fname)),
                }
            }
            if ds[2..].iter().any(|&d| d != 1) {
                return Err(MError::new(format!("{fname}: N-dimensional arrays are not supported in mlab v1")));
            }
            Ok((ds[0], ds[1]))
        }
    }
}

/// First non-singleton dimension (1 — rows, 2 — columns).
pub fn default_dim(m: &Mat) -> usize {
    if m.rows != 1 { 1 } else { 2 }
}

pub fn dim_arg(args: &[Value], k: usize, fname: &str) -> Result<Option<usize>, MError> {
    match args.get(k) {
        None => Ok(None),
        Some(Value::Mat(m)) if m.is_empty() => Ok(None),
        Some(v) if is_str(v) => Ok(None),
        Some(_) => {
            let d = scalar_arg(args, k, fname)?;
            if d < 1.0 || d != d.trunc() {
                return Err(MError::new(format!("{fname}: DIM must be a valid dimension")));
            }
            Ok(Some(d as usize))
        }
    }
}

#[cfg(test)]
mod wave30_tests {
    use super::*;
    #[test]
    fn registry_and_dimensions() {
        let mut r = Registry::new();
        r.register("z", |_, _, _| num(1.0));
        r.register("a", |_, _, _| num(2.0));
        r.register("z", |_, _, _| num(3.0));
        assert_eq!(r.names(), vec!["a", "z"]);
        let mut it = Interp::capture();
        let out = r.get("z").unwrap()(&mut it, &[], 1).unwrap();
        assert_eq!(scalar_arg(&out, 0, "probe").unwrap(), 3.0);
        assert_eq!(dims_args(&[], "x").unwrap(), (1, 1));
        assert_eq!(dims_args(&[Value::num(-2.0)], "x").unwrap(), (0, 0));
        assert_eq!(dims_args(&[Value::num(2.9)], "x").unwrap(), (2, 2));
        assert!(dims_args(&[Value::num(f64::NAN)], "x").is_err());
        assert_eq!(
            scalar_arg(&[Value::Mat(Mat::col(vec![4.0, 8.0]))], 0, "x").unwrap(),
            4.0
        );
    }
}
