//! Matrix creation, sizes, types, strings, control (error, warning, feval, arrayfun…), constants.

use super::*;
use crate::display::unparse_func;
use crate::ops;
use crate::value::Func;
use std::rc::Rc;

pub fn register(r: &mut Registry) {
    // constants
    r.register("pi", |_, a, _| fill_const(a, std::f64::consts::PI, "pi"));
    r.register("e", |_, a, _| fill_const(a, std::f64::consts::E, "e"));
    r.register("Inf", |_, a, _| fill_const(a, f64::INFINITY, "Inf"));
    r.register("inf", |_, a, _| fill_const(a, f64::INFINITY, "inf"));
    r.register("NaN", |_, a, _| fill_const(a, f64::NAN, "NaN"));
    r.register("nan", |_, a, _| fill_const(a, f64::NAN, "nan"));
    r.register("NA", |_, a, _| fill_const(a, f64::NAN, "NA"));
    r.register("eps", eps);
    r.register("realmax", |_, _, _| num(f64::MAX));
    r.register("realmin", |_, _, _| num(f64::MIN_POSITIVE));
    r.register("flintmax", |_, _, _| num(9007199254740992.0));
    for name in ["i", "j", "I", "J"] {
        r.register(name, |_, _, _| one(Mat::cscalar(0.0, 1.0)));
    }
    r.register("true", |_, a, _| {
        let (m, n) = dims_args(a, "true")?;
        one(Mat::filled(m, n, 1.0).with_class(Class::Logical))
    });
    r.register("false", |_, a, _| {
        let (m, n) = dims_args(a, "false")?;
        one(Mat::filled(m, n, 0.0).with_class(Class::Logical))
    });
    // creation
    r.register("zeros", |_, a, _| {
        let (m, n) = dims_args(a, "zeros")?;
        one(Mat::zeros(m, n))
    });
    r.register("ones", |_, a, _| {
        let (m, n) = dims_args(a, "ones")?;
        one(Mat::filled(m, n, 1.0))
    });
    r.register("eye", |_, a, _| {
        let (m, n) = dims_args(a, "eye")?;
        let mut z = Mat::zeros(m, n);
        for k in 0..m.min(n) {
            z.re[k * m + k] = 1.0;
        }
        one(z)
    });
    r.register("rand", rand);
    r.register("randn", randn);
    r.register("randi", randi);
    r.register("randperm", randperm);
    r.register("rng", rng);
    r.register("linspace", linspace);
    r.register("logspace", logspace);
    r.register("colon", |_, a, _| {
        let s = if a.len() > 2 { Some(arg(a, 1, "colon")?) } else { None };
        let b = arg(a, a.len().max(2) - 1, "colon")?;
        Ok(vec![ops::range(arg(a, 0, "colon")?, s, b)?])
    });
    r.register("magic", magic);
    r.register("diag", diag);
    r.register("triu", |_, a, _| tri(a, true));
    r.register("tril", |_, a, _| tri(a, false));
    r.register("repmat", repmat);
    r.register("reshape", reshape);
    r.register("cat", cat);
    r.register("horzcat", |_, a, _| Ok(vec![Value::Mat(ops::hcat(mats(a, "horzcat")?)?)]));
    r.register("vertcat", |_, a, _| Ok(vec![Value::Mat(ops::vcat(mats(a, "vertcat")?)?)]));
    r.register("meshgrid", meshgrid);
    r.register("fliplr", |_, a, _| flip(mat_arg(a, 0, "fliplr")?, 2));
    r.register("flipud", |_, a, _| flip(mat_arg(a, 0, "flipud")?, 1));
    r.register("flip", |_, a, _| {
        let m = mat_arg(a, 0, "flip")?;
        let d = dim_arg(a, 1, "flip")?.unwrap_or(default_dim(m));
        flip(m, d)
    });
    // sizes
    r.register("size", size);
    r.register("numel", |_, a, _| num(mat_like_numel(arg(a, 0, "numel")?) as f64));
    r.register("length", |_, a, _| {
        let (r, c) = dims_of(arg(a, 0, "length")?);
        num(if r == 0 || c == 0 { 0.0 } else { r.max(c) as f64 })
    });
    r.register("ndims", |_, _, _| num(2.0));
    r.register("rows", |_, a, _| num(dims_of(arg(a, 0, "rows")?).0 as f64));
    r.register("columns", |_, a, _| num(dims_of(arg(a, 0, "columns")?).1 as f64));
    r.register("isempty", |_, a, _| {
        let (r, c) = dims_of(arg(a, 0, "isempty")?);
        boolv(r == 0 || c == 0)
    });
    r.register("isscalar", |_, a, _| boolv(dims_of(arg(a, 0, "isscalar")?) == (1, 1)));
    r.register("isvector", |_, a, _| {
        let (r, c) = dims_of(arg(a, 0, "isvector")?);
        boolv((r == 1 || c == 1) && r * c >= 1)
    });
    r.register("isrow", |_, a, _| boolv(dims_of(arg(a, 0, "isrow")?).0 == 1));
    r.register("iscolumn", |_, a, _| boolv(dims_of(arg(a, 0, "iscolumn")?).1 == 1));
    r.register("ismatrix", |_, _, _| boolv(true));
    r.register("issquare", |_, a, _| {
        let (r, c) = dims_of(arg(a, 0, "issquare")?);
        boolv(r == c)
    });
    // types
    r.register("double", |_, a, _| {
        let mut m = mat_arg(a, 0, "double")?.clone();
        m.class = Class::Double;
        one(m)
    });
    r.register("single", |_, a, _| {
        let mut m = mat_arg(a, 0, "single")?.clone();
        m.class = Class::Double;
        one(m)
    });
    r.register("logical", |_, a, _| {
        let m = mat_arg(a, 0, "logical")?;
        if m.re.iter().any(|x| x.is_nan()) {
            return Err(MError::new("logical: NaN can't be converted to logical value"));
        }
        one(Mat::new(m.rows, m.cols, m.re.iter().map(|&x| if x != 0.0 { 1.0 } else { 0.0 }).collect()).with_class(Class::Logical))
    });
    r.register("char", |_, a, _| {
        if a.len() > 1 {
            // char('ab', 'cde') — rows padded with spaces
            let strs: Vec<String> = a.iter().map(|v| str_of(v).unwrap_or_default()).collect();
            let w = strs.iter().map(|s| s.chars().count()).max().unwrap_or(0);
            let rows: Vec<Mat> = strs.iter().map(|s| Mat::str(&format!("{s:<w$}"))).collect();
            return Ok(vec![Value::Mat(ops::vcat(rows)?)]);
        }
        let mut m = mat_arg(a, 0, "char")?.clone();
        m.class = Class::Char;
        one(m)
    });
    r.register("class", |_, a, _| Ok(vec![Value::str(arg(a, 0, "class")?.class_name())]));
    r.register("isa", |_, a, _| {
        let v = arg(a, 0, "isa")?;
        let c = str_arg(a, 1, "isa")?;
        let cn = v.class_name();
        boolv(cn == c || (c == "numeric" && cn == "double") || (c == "float" && cn == "double"))
    });
    r.register("isnumeric", |_, a, _| boolv(matches!(arg(a, 0, "isnumeric")?, Value::Mat(m) if m.class == Class::Double)));
    r.register("isfloat", |_, a, _| boolv(matches!(arg(a, 0, "isfloat")?, Value::Mat(m) if m.class == Class::Double)));
    r.register("ischar", |_, a, _| boolv(is_str(arg(a, 0, "ischar")?)));
    r.register("islogical", |_, a, _| boolv(matches!(arg(a, 0, "islogical")?, Value::Mat(m) if m.class == Class::Logical)));
    r.register("isbool", |_, a, _| boolv(matches!(arg(a, 0, "isbool")?, Value::Mat(m) if m.class == Class::Logical)));
    r.register("isreal", |_, a, _| boolv(matches!(arg(a, 0, "isreal")?, Value::Mat(m) if !m.is_complex())));
    r.register("iscomplex", |_, a, _| boolv(matches!(arg(a, 0, "iscomplex")?, Value::Mat(m) if m.is_complex())));
    r.register("is_function_handle", |_, a, _| boolv(matches!(arg(a, 0, "is_function_handle")?, Value::Func(_))));
    r.register("isequal", isequal);
    // strings
    r.register("strcmp", |_, a, _| boolv(str_of(arg(a, 0, "strcmp")?).is_some() && str_of(arg(a, 0, "strcmp")?) == str_of(arg(a, 1, "strcmp")?)));
    r.register("strcmpi", |_, a, _| {
        let x = str_of(arg(a, 0, "strcmpi")?).map(|s| s.to_lowercase());
        let y = str_of(arg(a, 1, "strcmpi")?).map(|s| s.to_lowercase());
        boolv(x.is_some() && x == y)
    });
    r.register("upper", |_, a, _| case_map(a, "upper", true));
    r.register("toupper", |_, a, _| case_map(a, "toupper", true));
    r.register("lower", |_, a, _| case_map(a, "lower", false));
    r.register("tolower", |_, a, _| case_map(a, "tolower", false));
    r.register("strtrim", |_, a, _| Ok(vec![Value::str(str_arg(a, 0, "strtrim")?.trim_matches(|c: char| c.is_whitespace() || c == '\0'))]));
    r.register("deblank", |_, a, _| Ok(vec![Value::str(str_arg(a, 0, "deblank")?.trim_end())]));
    r.register("blanks", |_, a, _| Ok(vec![Value::str(&" ".repeat(scalar_arg(a, 0, "blanks")?.max(0.0) as usize))]));
    r.register("strrep", |_, a, _| {
        let s = str_arg(a, 0, "strrep")?;
        let p = str_arg(a, 1, "strrep")?;
        let q = str_arg(a, 2, "strrep")?;
        Ok(vec![Value::str(&if p.is_empty() { s } else { s.replace(&p, &q) })])
    });
    r.register("strcat", |_, a, _| {
        let mut s = String::new();
        for v in a {
            s.push_str(str_of(v).as_deref().map(|x| x.trim_end()).unwrap_or(""));
        }
        Ok(vec![Value::str(&s)])
    });
    r.register("strfind", |_, a, _| {
        let s: Vec<char> = str_arg(a, 0, "strfind")?.chars().collect();
        let p: Vec<char> = str_arg(a, 1, "strfind")?.chars().collect();
        let mut out = Vec::new();
        if !p.is_empty() && p.len() <= s.len() {
            for k in 0..=s.len() - p.len() {
                if s[k..k + p.len()] == p[..] {
                    out.push((k + 1) as f64);
                }
            }
        }
        let n = out.len();
        one(Mat::new(if n == 0 { 1 } else { 1 }, n, out))
    });
    r.register("str2double", |_, a, _| {
        let s = str_of(arg(a, 0, "str2double")?).unwrap_or_default();
        num(parse_double(&s).unwrap_or(f64::NAN))
    });
    r.register("str2num", |it, a, _| {
        let s = str_arg(a, 0, "str2num")?;
        let prog = match crate::parser::parse_program(&format!("__str2num__ = [{s}];")) {
            Ok(p) => p,
            Err(_) => return one(Mat::empty()),
        };
        let _ = prog;
        let mut sub = Interp::capture();
        sub.run(&format!("__str2num__ = [{s}];"));
        let v = sub.get_var("__str2num__").cloned().unwrap_or(Value::Mat(Mat::empty()));
        let _ = it;
        Ok(vec![v])
    });
    r.register("isspace", |_, a, _| {
        let m = mat_arg(a, 0, "isspace")?;
        one(Mat::new(m.rows, m.cols, m.re.iter().map(|&x| if matches!(x as u32, 9..=13 | 32) { 1.0 } else { 0.0 }).collect()).with_class(Class::Logical))
    });
    r.register("isdigit", |_, a, _| {
        let m = mat_arg(a, 0, "isdigit")?;
        one(Mat::new(m.rows, m.cols, m.re.iter().map(|&x| if (48.0..=57.0).contains(&x) { 1.0 } else { 0.0 }).collect()).with_class(Class::Logical))
    });
    // functions
    r.register("feval", |it, a, nargout| {
        let f = arg(a, 0, "feval")?.clone();
        it.call_value(&f, a[1..].to_vec(), nargout)
    });
    r.register("func2str", |_, a, _| match arg(a, 0, "func2str")? {
        Value::Func(f) => Ok(vec![Value::str(&unparse_func(f))]),
        _ => Err(MError::new("func2str: FCN_HANDLE argument must be a valid function handle")),
    });
    r.register("str2func", |_, a, _| {
        let s = str_arg(a, 0, "str2func")?;
        if s.starts_with('@') {
            let mut sub = Interp::capture();
            sub.run(&format!("__f__ = {s};"));
            return match sub.get_var("__f__") {
                Some(v) => Ok(vec![v.clone()]),
                None => Err(MError::new(format!("str2func: invalid function string '{s}'"))),
            };
        }
        Ok(vec![Value::Func(Rc::new(Func::Named(s)))])
    });
    r.register("arrayfun", arrayfun);
    r.register("exist", |it, a, _| {
        let s = str_arg(a, 0, "exist")?;
        // exist(name[, 'var'|'builtin'|'file'|'dir']): 1 — variable, 5 — function, 2 — file, 7 — directory (via it.fs)
        let kind = a.get(1).and_then(str_of).map(|k| k.to_lowercase());
        let var = || if it.get_var(&s).is_some() { 1.0 } else { 0.0 };
        let v = match kind.as_deref() {
            Some("var") => var(),
            Some("builtin") => if it.is_function_name(&s) { 5.0 } else { 0.0 },
            Some("file") => super::files::exist_path(it, &s),
            Some("dir") => if super::files::exist_path(it, &s) == 7.0 { 7.0 } else { 0.0 },
            _ => {
                if it.get_var(&s).is_some() {
                    1.0
                } else if it.is_function_name(&s) {
                    5.0
                } else {
                    super::files::exist_path(it, &s)
                }
            }
        };
        num(v)
    });
    // control
    r.register("error", error);
    r.register("warning", warning);
    r.register("lasterr", |it, _, _| Ok(vec![Value::str(&it.last_error.clone())]));
    r.register("assert", assert);
    r.register("format", |it, a, _| {
        let mode = a.first().and_then(str_of).unwrap_or_else(|| "short".into()).to_lowercase();
        match mode.as_str() {
            "long" => it.fmt.long = true,
            "short" => it.fmt.long = false,
            _ => {}
        }
        Ok(vec![])
    });
    r.register("clear", |it, a, _| {
        let names: Vec<String> = a.iter().filter_map(str_of).collect();
        it.clear_vars(&names);
        Ok(vec![])
    });
    r.register("clearvars", |it, a, _| {
        let names: Vec<String> = a.iter().filter_map(str_of).collect();
        it.clear_vars(&names);
        Ok(vec![])
    });
    for name in ["clc", "close", "more", "pkg", "hold", "figure", "drawnow", "beep"] {
        r.register(name, |_, _, _| Ok(vec![]));
    }
    r.register("tic", |it, _, nargout| {
        it.tic = Some(std::time::Instant::now());
        if nargout > 0 { num(0.0) } else { Ok(vec![]) }
    });
    r.register("toc", |it, _, nargout| {
        let t = it.tic.map(|t| t.elapsed().as_secs_f64()).ok_or_else(|| MError::new("toc: function called before timer initialization with tic()"))?;
        if nargout > 0 {
            num(t)
        } else {
            it.out(&format!("Elapsed time is {t:.6} seconds.\n"));
            Ok(vec![])
        }
    });
}

fn fill_const(a: &[Value], x: f64, name: &str) -> Result<Vec<Value>, MError> {
    if a.is_empty() {
        return num(x);
    }
    let (m, n) = dims_args(a, name)?;
    one(Mat::filled(m, n, x))
}

fn eps(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    if let Some(Value::Mat(m)) = a.first() {
        if m.class != Class::Char {
            let out: Vec<f64> = m
                .re
                .iter()
                .map(|&x| {
                    let x = x.abs();
                    if !x.is_finite() {
                        f64::NAN
                    } else if x < f64::MIN_POSITIVE {
                        f64::from_bits(1)
                    } else {
                        let e = x.log2().floor();
                        let mut v = 2f64.powf(e - 52.0);
                        // correction for log2 inaccuracy near powers of two
                        if 2f64.powf(e) > x {
                            v /= 2.0;
                        }
                        v
                    }
                })
                .collect();
            return one(Mat::new(m.rows, m.cols, out));
        }
    }
    fill_const(&[], f64::EPSILON, "eps")
}

fn seed_from(v: &Value) -> u64 {
    match v {
        Value::Mat(m) if m.class != Class::Char && m.numel() >= 1 => m.re[0].abs() as u64,
        _ => crate::rng::DEFAULT_SEED,
    }
}

/// rand('seed'|'state'|'twister', s) — seeding; returns true if it was a seeding call.
fn maybe_seed(it: &mut Interp, a: &[Value]) -> bool {
    if let Some(s) = a.first().and_then(str_of) {
        if matches!(s.as_str(), "seed" | "state" | "twister") {
            let seed = a.get(1).map(seed_from).unwrap_or(crate::rng::DEFAULT_SEED);
            it.rng.reseed(seed);
            return true;
        }
    }
    false
}

fn rand(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    if maybe_seed(it, a) {
        return Ok(vec![]);
    }
    let (m, n) = dims_args(a, "rand")?;
    let v: Vec<f64> = (0..m * n).map(|_| it.rng.uniform()).collect();
    one(Mat::new(m, n, v))
}

fn randn(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    if maybe_seed(it, a) {
        return Ok(vec![]);
    }
    let (m, n) = dims_args(a, "randn")?;
    let v: Vec<f64> = (0..m * n).map(|_| it.rng.normal()).collect();
    one(Mat::new(m, n, v))
}

fn randi(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let lim = mat_arg(a, 0, "randi")?;
    let (lo, hi) = if lim.numel() >= 2 { (lim.re[0], lim.re[1]) } else { (1.0, lim.re[0]) };
    if hi < lo {
        return Err(MError::new("randi: require IMIN <= IMAX"));
    }
    let (m, n) = dims_args(&a[1..], "randi")?;
    let span = (hi - lo + 1.0).floor();
    let v: Vec<f64> = (0..m * n).map(|_| lo + (it.rng.uniform() * span).floor()).collect();
    one(Mat::new(m, n, v))
}

fn randperm(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let n = scalar_arg(a, 0, "randperm")?.max(0.0) as usize;
    let mut p: Vec<f64> = (1..=n).map(|x| x as f64).collect();
    for i in (1..n).rev() {
        let j = (it.rng.uniform() * (i + 1) as f64) as usize;
        p.swap(i, j.min(i));
    }
    let k = if a.len() > 1 { scalar_arg(a, 1, "randperm")? as usize } else { n };
    p.truncate(k);
    one(Mat::row(p))
}

fn rng(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    match a.first() {
        None => Ok(vec![]),
        Some(v) => {
            let seed = match str_of(v) {
                Some(s) if s == "default" => crate::rng::DEFAULT_SEED,
                Some(s) if s == "shuffle" => std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0),
                Some(_) => crate::rng::DEFAULT_SEED,
                None => seed_from(v),
            };
            it.rng.reseed(seed);
            Ok(vec![])
        }
    }
}

fn linspace(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let lo = scalar_arg(a, 0, "linspace")?;
    let hi = scalar_arg(a, 1, "linspace")?;
    let n = if a.len() > 2 { scalar_arg(a, 2, "linspace")?.floor() } else { 100.0 };
    if !(n >= 1.0) {
        // linspace(a, b, 0) — empty 1×0, as in MATLAB (vmm-red-036, tests3)
        return one(Mat::new(1, 0, vec![]));
    }
    let n = n as usize;
    if n == 1 {
        return one(Mat::row(vec![hi]));
    }
    let mut v: Vec<f64> = (0..n).map(|k| lo + (hi - lo) * (k as f64) / ((n - 1) as f64)).collect();
    v[n - 1] = hi;
    one(Mat::row(v))
}

fn logspace(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let lo = scalar_arg(a, 0, "logspace")?;
    let hi = scalar_arg(a, 1, "logspace")?;
    let n = if a.len() > 2 { scalar_arg(a, 2, "logspace")?.max(1.0) as usize } else { 50 };
    let v: Vec<f64> = (0..n)
        .map(|k| if n == 1 { 10f64.powf(hi) } else { 10f64.powf(lo + (hi - lo) * (k as f64) / ((n - 1) as f64)) })
        .collect();
    one(Mat::row(v))
}

/// Magic squares: classical constructions (odd — the Siamese method in its formula variant,
/// doubly even — swapping on the diagonals, singly even — LUX-like construction from four odd ones).
pub fn magic_square(n: usize) -> Mat {
    let mut m = Mat::zeros(n, n);
    if n == 0 {
        return Mat::empty();
    }
    if n == 1 {
        return Mat::scalar(1.0);
    }
    if n == 2 {
        return Mat::new(2, 2, vec![4.0, 1.0, 3.0, 2.0]);
    }
    let set = |m: &mut Mat, i: usize, j: usize, v: f64| m.re[j * n + i] = v;
    if n % 2 == 1 {
        // M(i,j) = n*mod(i+j-(n+3)/2, n) + mod(i+2j-2, n) + 1  (1-based indices)
        for i in 1..=n {
            for j in 1..=n {
                let a = ((i + j) as i64 - ((n + 3) / 2) as i64).rem_euclid(n as i64);
                let b = ((i + 2 * j) as i64 - 2).rem_euclid(n as i64);
                set(&mut m, i - 1, j - 1, (n as i64 * a + b + 1) as f64);
            }
        }
    } else if n % 4 == 0 {
        for i in 1..=n {
            for j in 1..=n {
                let v = ((i - 1) * n + j) as f64;
                let keep = ((i % 4) / 2) == ((j % 4) / 2);
                set(&mut m, i - 1, j - 1, if keep { (n * n + 1) as f64 - v } else { v });
            }
        }
    } else {
        let p = n / 2;
        let a = magic_square(p);
        let pp = (p * p) as f64;
        for i in 0..p {
            for j in 0..p {
                let v = a.re[j * p + i];
                set(&mut m, i, j, v);
                set(&mut m, i + p, j + p, v + pp);
                set(&mut m, i, j + p, v + 2.0 * pp);
                set(&mut m, i + p, j, v + 3.0 * pp);
            }
        }
        let k = (n - 2) / 4;
        // swap columns between the upper and lower halves
        let mut cols: Vec<usize> = (0..k).collect();
        cols.extend((n - k + 1)..n);
        for i in 0..p {
            for &j in &cols {
                let t = m.re[j * n + i];
                m.re[j * n + i] = m.re[j * n + i + p];
                m.re[j * n + i + p] = t;
            }
        }
        for &(i, j) in &[(k, 0usize), (k, k)] {
            let t = m.re[j * n + i];
            m.re[j * n + i] = m.re[j * n + i + p];
            m.re[j * n + i + p] = t;
        }
    }
    m
}

fn magic(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let n = scalar_arg(a, 0, "magic")?.max(0.0) as usize;
    one(magic_square(n))
}

fn diag(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "diag")?;
    let k = if a.len() > 1 { scalar_arg(a, 1, "diag")? as i64 } else { 0 };
    if m.rows == 1 || m.cols == 1 {
        let n = m.numel();
        let size = n + k.unsigned_abs() as usize;
        let mut out = Mat::zeros(size, size);
        let mut im = m.im.as_ref().map(|_| vec![0.0; size * size]);
        for t in 0..n {
            let (i, j) = if k >= 0 { (t, t + k as usize) } else { (t + (-k) as usize, t) };
            out.re[j * size + i] = m.re[t];
            if let Some(im) = &mut im {
                im[j * size + i] = m.im_at(t);
            }
        }
        out.im = im;
        out.class = if m.class == Class::Logical { Class::Double } else { m.class };
        return one(out);
    }
    let mut vals = Vec::new();
    let mut ims = Vec::new();
    let (mut i, mut j) = if k >= 0 { (0usize, k as usize) } else { ((-k) as usize, 0usize) };
    while i < m.rows && j < m.cols {
        vals.push(m.at(i, j));
        ims.push(m.im_at(j * m.rows + i));
        i += 1;
        j += 1;
    }
    let n = vals.len();
    let mut out = if m.is_complex() { Mat::complex(n, 1, vals, ims) } else { Mat::col(vals) };
    out.class = m.class;
    one(out)
}

fn tri(a: &[Value], upper: bool) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, if upper { "triu" } else { "tril" })?;
    let k = if a.len() > 1 { scalar_arg(a, 1, "triu")? as i64 } else { 0 };
    let mut out = m.clone();
    for j in 0..m.cols {
        for i in 0..m.rows {
            let d = j as i64 - i as i64;
            let keep = if upper { d >= k } else { d <= k };
            if !keep {
                out.re[j * m.rows + i] = 0.0;
                if let Some(im) = &mut out.im {
                    im[j * m.rows + i] = 0.0;
                }
            }
        }
    }
    out.narrow();
    one(out)
}

fn repmat(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "repmat")?;
    let (rm, rn) = dims_args(&a[1..], "repmat")?;
    let (r, c) = (m.rows * rm, m.cols * rn);
    let mut re = vec![0.0; r * c];
    let mut im = m.im.as_ref().map(|_| vec![0.0; r * c]);
    for bj in 0..rn {
        for bi in 0..rm {
            for j in 0..m.cols {
                for i in 0..m.rows {
                    let dst = (bj * m.cols + j) * r + bi * m.rows + i;
                    re[dst] = m.re[j * m.rows + i];
                    if let Some(im) = &mut im {
                        im[dst] = m.im_at(j * m.rows + i);
                    }
                }
            }
        }
    }
    one(Mat { rows: r, cols: c, re, im, class: m.class })
}

fn reshape(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "reshape")?;
    let n = m.numel();
    let mut dims: Vec<Option<usize>> = Vec::new();
    if a.len() == 2 {
        let d = mat_arg(a, 1, "reshape")?;
        for &x in &d.re {
            dims.push(Some(x as usize));
        }
    } else {
        for v in &a[1..] {
            match v {
                Value::Mat(d) if d.is_empty() => dims.push(None),
                Value::Mat(d) => dims.push(Some(d.re[0] as usize)),
                _ => return Err(invalid_call("reshape")),
            }
        }
    }
    let known: usize = dims.iter().flatten().product();
    let holes = dims.iter().filter(|d| d.is_none()).count();
    if holes > 1 {
        return Err(MError::new("reshape: only a single dimension can be unknown"));
    }
    if holes == 1 {
        if known == 0 || n % known != 0 {
            return Err(MError::new(format!("reshape: SIZE is not divisible by the product of known dimensions (= {known})")));
        }
        let fill = n / known;
        for d in dims.iter_mut() {
            if d.is_none() {
                *d = Some(fill);
            }
        }
    }
    let ds: Vec<usize> = dims.into_iter().flatten().collect();
    if ds.len() < 2 || ds[2..].iter().any(|&d| d != 1) {
        return Err(MError::new("reshape: N-dimensional arrays are not supported in mlab v1"));
    }
    let (r, c) = (ds[0], ds[1]);
    if r * c != n {
        return Err(MError::new(format!("reshape: can't reshape {}x{} array to {}x{} array", m.rows, m.cols, r, c)));
    }
    let mut out = m.clone();
    out.rows = r;
    out.cols = c;
    one(out)
}

fn mats(a: &[Value], fname: &str) -> Result<Vec<Mat>, MError> {
    a.iter()
        .map(|v| match v {
            Value::Mat(m) => Ok(m.clone()),
            _ => Err(MError::new(format!("{fname}: wrong type argument"))),
        })
        .collect()
}

fn cat(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let d = scalar_arg(a, 0, "cat")? as usize;
    let ms = mats(&a[1..], "cat")?;
    match d {
        1 => Ok(vec![Value::Mat(ops::vcat(ms)?)]),
        2 => Ok(vec![Value::Mat(ops::hcat(ms)?)]),
        _ => Err(MError::new("cat: N-dimensional arrays are not supported in mlab v1")),
    }
}

fn meshgrid(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let x = mat_arg(a, 0, "meshgrid")?;
    let y = if a.len() > 1 { mat_arg(a, 1, "meshgrid")? } else { x };
    let (nx, ny) = (x.numel(), y.numel());
    let mut xx = Mat::zeros(ny, nx);
    let mut yy = Mat::zeros(ny, nx);
    for j in 0..nx {
        for i in 0..ny {
            xx.re[j * ny + i] = x.re[j];
            yy.re[j * ny + i] = y.re[i];
        }
    }
    let mut out = vec![Value::Mat(xx)];
    if nargout > 1 {
        out.push(Value::Mat(yy));
    }
    Ok(out)
}

fn flip(m: &Mat, dim: usize) -> Result<Vec<Value>, MError> {
    let mut out = m.clone();
    for j in 0..m.cols {
        for i in 0..m.rows {
            let (si, sj) = if dim == 1 { (m.rows - 1 - i, j) } else { (i, m.cols - 1 - j) };
            out.re[j * m.rows + i] = m.re[sj * m.rows + si];
            if let (Some(dst), Some(src)) = (&mut out.im, &m.im) {
                dst[j * m.rows + i] = src[sj * m.rows + si];
            }
        }
    }
    one(out)
}

fn dims_of(v: &Value) -> (usize, usize) {
    match v {
        Value::Mat(m) => (m.rows, m.cols),
        Value::Table(t) => (crate::table::height(t), t.names.len()),
        Value::Str(s) => (s.rows, s.cols),
        _ => (1, 1),
    }
}

fn mat_like_numel(v: &Value) -> usize {
    let (r, c) = dims_of(v);
    r * c
}

fn size(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let (r, c) = dims_of(arg(a, 0, "size")?);
    if a.len() > 1 {
        let d = scalar_arg(a, 1, "size")? as usize;
        return num(match d {
            1 => r as f64,
            2 => c as f64,
            0 => return Err(MError::new("size: requested dimension DIM (= 0) out of range")),
            _ => 1.0,
        });
    }
    if nargout <= 1 {
        return one(Mat::row(vec![r as f64, c as f64]));
    }
    let mut out = vec![Value::num(r as f64), Value::num(c as f64)];
    while out.len() < nargout {
        out.push(Value::num(1.0));
    }
    Ok(out)
}

fn isequal(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let first = arg(a, 0, "isequal")?;
    for other in &a[1..] {
        let eq = match (first, other) {
            (Value::Mat(x), Value::Mat(y)) => {
                x.rows == y.rows && x.cols == y.cols && x.re == y.re && x.im_or_zeros() == y.im_or_zeros()
            }
            (Value::Func(x), Value::Func(y)) => Rc::ptr_eq(x, y),
            _ => false,
        };
        if !eq {
            return boolv(false);
        }
    }
    boolv(true)
}

fn arrayfun(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let f = arg(a, 0, "arrayfun")?.clone();
    let mut inputs: Vec<&Mat> = Vec::new();
    let mut uniform = true;
    let mut k = 1;
    while k < a.len() {
        if let Some(s) = str_of(&a[k]) {
            if s == "UniformOutput" && k + 1 < a.len() {
                uniform = super::scalar_arg(a, k + 1, "arrayfun")? != 0.0;
                k += 2;
                continue;
            }
        }
        inputs.push(mat_arg(a, k, "arrayfun")?);
        k += 1;
    }
    if !uniform {
        return Err(MError::new("arrayfun: UniformOutput=false needs cell arrays, which are not supported in mlab v1"));
    }
    let first = inputs.first().ok_or_else(|| invalid_call("arrayfun"))?;
    let (r, c) = (first.rows, first.cols);
    for m in &inputs {
        if m.rows != r || m.cols != c {
            return Err(MError::new("arrayfun: all the input arguments must have the same size and shape"));
        }
    }
    let nout = nargout.max(1);
    let mut outs: Vec<(Vec<f64>, Vec<f64>, Class)> = vec![(Vec::with_capacity(r * c), Vec::new(), Class::Double); nout];
    for idx in 0..r * c {
        let args: Vec<Value> = inputs
            .iter()
            .map(|m| {
                let mut e = Mat { rows: 1, cols: 1, re: vec![m.re[idx]], im: m.im.as_ref().map(|v| vec![v[idx]]), class: m.class };
                e.narrow();
                Value::Mat(e)
            })
            .collect();
        let res = it.call_value(&f, args, nout)?;
        for (o, slot) in outs.iter_mut().enumerate() {
            match res.get(o) {
                Some(Value::Mat(m)) if m.numel() == 1 => {
                    slot.0.push(m.re[0]);
                    slot.1.push(m.im_at(0));
                    slot.2 = m.class;
                }
                _ => {
                    return Err(MError::new(
                        "arrayfun: all values must be scalars when UniformOutput = true; use the 'UniformOutput', false options",
                    ));
                }
            }
        }
    }
    Ok(outs
        .into_iter()
        .map(|(re, im, class)| {
            let mut m = Mat::complex(r, c, re, im);
            m.class = class;
            Value::Mat(m)
        })
        .collect())
}

fn is_identifier_like(s: &str) -> bool {
    // «Pkg:id» — no spaces, contains a colon, does not end with a colon
    !s.contains(char::is_whitespace) && s.contains(':') && !s.ends_with(':') && !s.starts_with(':') && !s.contains('%')
}

fn format_message(it: &mut Interp, a: &[Value], fname: &str) -> Result<(String, String), MError> {
    let first = str_arg(a, 0, fname)?;
    let (id, fmt_idx) = if a.len() > 1 && is_identifier_like(&first) { (first.clone(), 1) } else { (String::new(), 0) };
    let fmt = str_arg(a, fmt_idx, fname)?;
    let msg = if a.len() > fmt_idx + 1 || fmt.contains('%') || fmt.contains('\\') {
        super::io::sprintf_impl(it, &fmt, &a[fmt_idx + 1..])?
    } else {
        fmt
    };
    Ok((id, msg))
}

fn error(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let (id, msg) = format_message(it, a, "error")?;
    Err(MError::new(msg).with_id(id))
}

fn warning(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    if let Some(s) = a.first().and_then(str_of) {
        match s.as_str() {
            "off" => {
                if a.len() == 1 || a.get(1).and_then(str_of).as_deref() == Some("all") {
                    it.warnings_on = false;
                }
                return Ok(vec![]);
            }
            "on" => {
                if a.len() == 1 || a.get(1).and_then(str_of).as_deref() == Some("all") {
                    it.warnings_on = true;
                }
                return Ok(vec![]);
            }
            "query" | "error" => return Ok(vec![]),
            _ => {}
        }
    }
    let (_, msg) = format_message(it, a, "warning")?;
    it.warn(&msg);
    Ok(vec![])
}

fn assert(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    match a.len() {
        0 => Err(invalid_call("assert")),
        1 => {
            if ops::truthy(&a[0])? {
                Ok(vec![])
            } else {
                Err(MError::new("assert (cond) failed"))
            }
        }
        _ => {
            if is_str(&a[1]) {
                if ops::truthy(&a[0])? {
                    return Ok(vec![]);
                }
                let (_, msg) = format_message(it, &a[1..], "assert")?;
                return Err(MError::new(msg));
            }
            let x = mat_arg(a, 0, "assert")?;
            let y = mat_arg(a, 1, "assert")?;
            let tol = if a.len() > 2 { scalar_arg(a, 2, "assert")? } else { 0.0 };
            if x.rows != y.rows || x.cols != y.cols {
                if !(y.is_scalar()) {
                    return Err(MError::new(format!(
                        "ASSERT errors for:  assert (cond,expected,tol)\n\n  Location  |  Observed  |  Expected  |  Reason\n     .          O({}x{})       E({}x{})      Dimensions don't match",
                        x.rows, x.cols, y.rows, y.cols
                    )));
                }
            }
            for k in 0..x.numel() {
                let yv = if y.is_scalar() { y.re[0] } else { y.re[k] };
                let xv = x.re[k];
                let ok = if xv.is_nan() && yv.is_nan() {
                    true
                } else if tol == 0.0 {
                    xv == yv
                } else if tol > 0.0 {
                    (xv - yv).abs() <= tol
                } else {
                    (xv - yv).abs() <= tol.abs() * yv.abs()
                };
                if !ok {
                    return Err(MError::new(format!(
                        "ASSERT errors for:  assert (cond,expected,tol)\n\n  Location  |  Observed  |  Expected  |  Reason\n    ({})          {}          {}          Abs err {} exceeds tol {}",
                        k + 1,
                        xv,
                        yv,
                        (xv - yv).abs(),
                        tol
                    )));
                }
            }
            Ok(vec![])
        }
    }
}

pub fn parse_double(s: &str) -> Option<f64> {
    let t = s.trim();
    match t.to_ascii_lowercase().as_str() {
        "inf" | "+inf" => return Some(f64::INFINITY),
        "-inf" => return Some(f64::NEG_INFINITY),
        "nan" => return Some(f64::NAN),
        _ => {}
    }
    t.replace(['d', 'D'], "e").parse::<f64>().ok()
}

/// upper/lower elementwise: a char matrix keeps its shape (upper(['ab'; 'cd']) — 2×2; previously
/// it was flattened into a row — vmm-str-010, tests3); non-numeric and non-char — unchanged.
fn case_map(a: &[Value], name: &str, up: bool) -> Result<Vec<Value>, MError> {
    if let Some(Value::Mat(m)) = a.first() {
        if m.class == Class::Char {
            let mut out = m.clone();
            for c in out.re.iter_mut() {
                if let Some(ch) = char::from_u32(*c as u32) {
                    let mapped = if up { ch.to_uppercase().next() } else { ch.to_lowercase().next() };
                    *c = mapped.unwrap_or(ch) as u32 as f64;
                }
            }
            return Ok(vec![Value::Mat(out)]);
        }
    }
    let s = str_arg(a, 0, name)?;
    Ok(vec![Value::str(&if up { s.to_uppercase() } else { s.to_lowercase() })])
}
