//! v2: table builtins — creation, selection, sortrows, groupsummary, grouptransform, pivot, joins,
//! missing values, moving windows, dates, import and export (CSV/TSV/JSON).

use super::{Registry, arg, invalid_call, is_str, mat_arg, num, one, str_arg, str_of};
use crate::fileio;
use crate::interp::{Interp, MError};
use crate::table::{self as tb, height};
use crate::value::{Class, ColKind, Mat, StrArr, Table, Value};
use std::collections::HashMap;
use std::rc::Rc;

pub fn register(r: &mut Registry) {
    r.register("table", table);
    r.register("height", height_b);
    r.register("width", width_b);
    r.register("summary", summary);
    r.register("sortrows", sortrows);
    r.register("groupsummary", groupsummary);
    r.register("grouptransform", grouptransform);
    r.register("pivot", pivot);
    r.register("innerjoin", innerjoin);
    r.register("join", join);
    r.register("ismissing", ismissing);
    r.register("rmmissing", rmmissing);
    r.register("movmean", |it, a, n| moving(it, a, n, "movmean"));
    r.register("movsum", |it, a, n| moving(it, a, n, "movsum"));
    r.register("movmax", |it, a, n| moving(it, a, n, "movmax"));
    r.register("movmin", |it, a, n| moving(it, a, n, "movmin"));
    r.register("movmedian", |it, a, n| moving(it, a, n, "movmedian"));
    r.register("tempname", tempname);
    r.register("fileread", fileread);
    r.register("writelines", writelines);
    r.register("datenum", datenum);
    r.register("datestr", datestr);
    r.register("readtable", readtable);
    r.register("writetable", writetable);
    r.register("readmatrix", readmatrix);
    r.register("writematrix", writematrix);
    r.register("jsonencode", jsonencode);
    r.register("string", string_b);
}

fn table_arg<'a>(a: &'a [Value], k: usize, fname: &str) -> Result<&'a Rc<Table>, MError> {
    match arg(a, k, fname)? {
        Value::Table(t) => Ok(t),
        other => Err(MError::new(format!("{fname}: argument {} must be a table, not {}", k + 1, other.class_name()))),
    }
}

fn tval(t: Table) -> Value {
    Value::Table(Rc::new(t))
}

/// «name, value» pairs from position `start`; names are case-insensitive.
pub fn opts(a: &[Value], start: usize, fname: &str) -> Result<Vec<(String, Value)>, MError> {
    let mut out = Vec::new();
    let mut k = start;
    while k < a.len() {
        let name = str_of(&a[k]).ok_or_else(|| MError::new(format!("{fname}: expected an option name at argument {}", k + 1)))?;
        let v = a.get(k + 1).ok_or_else(|| MError::new(format!("{fname}: option '{name}' needs a value")))?.clone();
        out.push((name.to_ascii_lowercase(), v));
        k += 2;
    }
    Ok(out)
}

pub fn opt<'a>(o: &'a [(String, Value)], name: &str) -> Option<&'a Value> {
    o.iter().rev().find(|(n, _)| n == name).map(|(_, v)| v)
}

pub fn strs_of(v: &Value) -> Option<Vec<String>> {
    match v {
        Value::Str(s) => Some(s.data.iter().map(|x| x.clone().unwrap_or_default()).collect()),
        Value::Mat(m) if m.class == Class::Char => Some(vec![m.to_string_lossy()]),
        _ => None,
    }
}

fn table(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let arg_names = std::mem::take(&mut it.arg_names);
    let pos = a.iter().position(|v| str_of(v).is_some_and(|s| s.eq_ignore_ascii_case("VariableNames"))).unwrap_or(a.len());
    let vals = &a[..pos];
    let names: Vec<String> = if pos < a.len() {
        let names = a.get(pos + 1).and_then(strs_of).ok_or_else(|| MError::new("table: 'VariableNames' needs a list of names"))?;
        if names.len() != vals.len() {
            return Err(MError::new(format!("table: {} names for {} variables", names.len(), vals.len())));
        }
        names
    } else {
        (0..vals.len()).map(|k| arg_names.get(k).cloned().flatten().unwrap_or_else(|| format!("Var{}", k + 1))).collect()
    };
    Ok(vec![tval(tb::make(names, vals)?)])
}

fn height_b(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    match arg(a, 0, "height")? {
        Value::Table(t) => num(height(t) as f64),
        Value::Mat(m) => num(m.rows as f64),
        Value::Str(s) => num(s.rows as f64),
        _ => num(1.0),
    }
}

fn width_b(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    match arg(a, 0, "width")? {
        Value::Table(t) => num(t.names.len() as f64),
        Value::Mat(m) => num(m.cols as f64),
        Value::Str(s) => num(s.cols as f64),
        _ => num(1.0),
    }
}

fn g6(x: f64) -> String {
    super::io::c_g(x, 6, false, false)
}

fn present(m: &Mat) -> Vec<f64> {
    m.re.iter().copied().filter(|x| !x.is_nan()).collect()
}

fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

fn median_sorted(v: &[f64]) -> f64 {
    let n = v.len();
    if n == 0 {
        f64::NAN
    } else if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn summary(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let t = table_arg(a, 0, "summary")?;
    let h = height(t);
    let mut s = format!("summary: {}x{} table\n", h, t.names.len());
    for k in 0..t.names.len() {
        let c = &t.columns[k];
        let miss = (0..h).filter(|&i| tb::is_missing(c, i)).count();
        let ty = match t.kinds[k] {
            ColKind::Num => "double",
            ColKind::Bool => "logical",
            ColKind::Text => "string",
            ColKind::Date => "date",
        };
        s.push_str(&format!("  {}: {ty}, {h} rows, missing {miss}\n", t.names[k]));
        match (t.kinds[k], c) {
            (ColKind::Text, Value::Str(st)) => {
                let mut u: Vec<&String> = st.data.iter().flatten().collect();
                u.sort();
                u.dedup();
                s.push_str(&format!("    unique {}\n", u.len()));
            }
            (ColKind::Bool, Value::Mat(m)) => {
                let tr = m.re.iter().filter(|&&x| x != 0.0).count();
                s.push_str(&format!("    true {tr}, false {}\n", h - tr));
            }
            (ColKind::Date, Value::Mat(m)) => {
                let v = sorted(present(m));
                let (lo, hi) = (v.first().copied().unwrap_or(f64::NAN), v.last().copied().unwrap_or(f64::NAN));
                s.push_str(&format!("    min {}, max {}\n", tb::iso_date(lo), tb::iso_date(hi)));
            }
            (_, Value::Mat(m)) => {
                let v = sorted(present(m));
                let n = v.len() as f64;
                let mean = v.iter().sum::<f64>() / n;
                let (lo, hi) = (v.first().copied().unwrap_or(f64::NAN), v.last().copied().unwrap_or(f64::NAN));
                s.push_str(&format!("    min {}, median {}, max {}, mean {}\n", g6(lo), g6(median_sorted(&v)), g6(hi), g6(mean)));
            }
            _ => {}
        }
    }
    it.out(&s);
    Ok(vec![])
}

// ---------- sortrows ----------

fn sortrows(_: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let dir_desc = |v: Option<&Value>| -> Result<Option<Vec<bool>>, MError> {
        match v.and_then(strs_of) {
            Some(ds) => ds
                .iter()
                .map(|d| match d.to_ascii_lowercase().as_str() {
                    "ascend" => Ok(false),
                    "descend" => Ok(true),
                    _ => Err(MError::new(format!("sortrows: unknown direction '{d}'"))),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Some),
            None => Ok(None),
        }
    };
    match arg(a, 0, "sortrows")? {
        Value::Table(t) => {
            let (vars, dirs) = match a.get(1) {
                None => ((0..t.names.len()).collect::<Vec<_>>(), dir_desc(None)?),
                Some(v) if strs_of(v).is_some_and(|s| s.len() == 1 && matches!(s[0].to_ascii_lowercase().as_str(), "ascend" | "descend")) => {
                    ((0..t.names.len()).collect(), dir_desc(Some(v))?)
                }
                Some(v) => (tb::var_list(t, v)?, dir_desc(a.get(2))?),
            };
            let keys: Vec<(&Value, bool)> = vars
                .iter()
                .enumerate()
                .map(|(k, &c)| (&t.columns[c], dirs.as_ref().map_or(false, |d| if d.len() == 1 { d[0] } else { d.get(k).copied().unwrap_or(false) })))
                .collect();
            let idx = tb::sorted_rows(&keys, height(t));
            let out = tb::take_rows(t, &idx);
            let mut res = vec![tval(out)];
            if nargout > 1 {
                res.push(Value::Mat(Mat::col(idx.iter().map(|&i| (i + 1) as f64).collect())));
            }
            Ok(res)
        }
        Value::Mat(m) => {
            let cols: Vec<(usize, bool)> = match a.get(1) {
                Some(Value::Mat(c)) if c.class != Class::Char => c
                    .re
                    .iter()
                    .map(|&x| {
                        let j = x.abs() as usize;
                        if j < 1 || j > m.cols {
                            Err(MError::new(format!("sortrows: column {x} out of bound {}", m.cols)))
                        } else {
                            Ok((j - 1, x < 0.0))
                        }
                    })
                    .collect::<Result<_, _>>()?,
                other => {
                    let desc = dir_desc(other)?.map_or(false, |d| d[0]);
                    (0..m.cols).map(|j| (j, desc)).collect()
                }
            };
            let colvals: Vec<Value> = cols.iter().map(|&(j, _)| Value::Mat(Mat::col((0..m.rows).map(|i| m.at(i, j)).collect()))).collect();
            let keys: Vec<(&Value, bool)> = colvals.iter().zip(&cols).map(|(v, &(_, d))| (v, d)).collect();
            let idx = tb::sorted_rows(&keys, m.rows);
            let mut out = Mat::zeros(m.rows, m.cols);
            out.class = m.class;
            for (ni, &oi) in idx.iter().enumerate() {
                for j in 0..m.cols {
                    out.re[j * m.rows + ni] = m.at(oi, j);
                }
            }
            let mut res = vec![Value::Mat(out)];
            if nargout > 1 {
                res.push(Value::Mat(Mat::col(idx.iter().map(|&i| (i + 1) as f64).collect())));
            }
            Ok(res)
        }
        other => Err(MError::new(format!("sortrows: wrong type argument '{}'", other.class_name()))),
    }
}

// ---------- grouping ----------

/// Aggregate over the group's values; missing values (NaN) are ignored, except for nummissing.
pub fn agg(method: &str, vals: &[f64]) -> Result<f64, MError> {
    let p: Vec<f64> = vals.iter().copied().filter(|x| !x.is_nan()).collect();
    let n = p.len() as f64;
    let mean = || p.iter().sum::<f64>() / n;
    let var = || {
        if p.len() < 2 {
            return if p.len() == 1 { 0.0 } else { f64::NAN };
        }
        let m = mean();
        p.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n - 1.0)
    };
    Ok(match method {
        "sum" => p.iter().sum(),
        "mean" => mean(),
        "median" => median_sorted(&sorted(p.clone())),
        "mode" => {
            let s = sorted(p.clone());
            let (mut best, mut bc, mut k) = (f64::NAN, 0, 0);
            while k < s.len() {
                let mut e = k;
                while e < s.len() && s[e] == s[k] {
                    e += 1;
                }
                if e - k > bc {
                    bc = e - k;
                    best = s[k];
                }
                k = e;
            }
            best
        }
        "var" => var(),
        "std" => var().sqrt(),
        "min" => p.iter().copied().fold(f64::NAN, f64::min),
        "max" => p.iter().copied().fold(f64::NAN, f64::max),
        "range" => p.iter().copied().fold(f64::NAN, f64::max) - p.iter().copied().fold(f64::NAN, f64::min),
        "nummissing" => (vals.len() - p.len()) as f64,
        "nnz" => p.iter().filter(|&&x| x != 0.0).count() as f64,
        "numunique" => {
            let mut s = sorted(p.clone());
            s.dedup();
            s.len() as f64
        }
        "count" | "numel" => vals.len() as f64,
        _ => return Err(MError::new(format!("groupsummary: unknown method '{method}' (sum mean median mode var std min max range nummissing nnz numunique)"))),
    })
}

fn group_keys<'a>(t: &'a Table, v: &Value) -> Result<(Vec<usize>, Vec<&'a Value>), MError> {
    let gv = tb::var_list(t, v)?;
    let keys = gv.iter().map(|&k| &t.columns[k]).collect();
    Ok((gv, keys))
}

fn groupsummary(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let t = table_arg(a, 0, "groupsummary")?;
    let (gv, keys) = group_keys(t, arg(a, 1, "groupsummary")?)?;
    let methods: Vec<String> = match a.get(2) {
        Some(v) => strs_of(v).ok_or_else(|| MError::new("groupsummary: methods must be names"))?.iter().map(|s| s.to_ascii_lowercase()).collect(),
        None => vec![],
    };
    let dv: Vec<usize> = match a.get(3) {
        Some(v) => tb::var_list(t, v)?,
        None => (0..t.names.len()).filter(|k| !gv.contains(k) && matches!(t.kinds[*k], ColKind::Num | ColKind::Bool)).collect(),
    };
    let gs = tb::groups(&keys, height(t));
    let firsts: Vec<usize> = gs.iter().map(|g| g[0]).collect();
    let mut out = tb::select_vars(&tb::take_rows(t, &firsts), &gv);
    out.names.push("GroupCount".into());
    out.columns.push(Value::Mat(Mat::col(gs.iter().map(|g| g.len() as f64).collect())));
    out.kinds.push(ColKind::Num);
    for &k in &dv {
        let m = tb::num_col(t, k).ok_or_else(|| MError::new(format!("groupsummary: variable '{}' is not numeric", t.names[k])))?;
        for meth in &methods {
            let mut col = Vec::with_capacity(gs.len());
            let mut buf = Vec::new();
            for g in &gs {
                buf.clear();
                buf.extend(g.iter().map(|&i| m.re[i]));
                col.push(agg(meth, &buf)?);
            }
            out.names.push(format!("{meth}_{}", t.names[k]));
            out.columns.push(Value::Mat(Mat::col(col)));
            out.kinds.push(ColKind::Num);
        }
    }
    Ok(vec![tval(out)])
}

fn grouptransform(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let t = table_arg(a, 0, "grouptransform")?.clone();
    let (gv, keys) = group_keys(&t, arg(a, 1, "grouptransform")?)?;
    let method = arg(a, 2, "grouptransform")?.clone();
    let dv: Vec<usize> = match a.get(3) {
        Some(v) => tb::var_list(&t, v)?,
        None => (0..t.names.len()).filter(|k| !gv.contains(k) && t.kinds[*k] == ColKind::Num).collect(),
    };
    let gs = tb::groups(&keys, height(&t));
    let mut out = (*t).clone();
    for &k in &dv {
        let m = tb::num_col(&t, k).ok_or_else(|| MError::new(format!("grouptransform: variable '{}' is not numeric", t.names[k])))?;
        let mut res = m.re.clone();
        for g in &gs {
            let x: Vec<f64> = g.iter().map(|&i| m.re[i]).collect();
            let y: Vec<f64> = match str_of(&method).map(|s| s.to_ascii_lowercase()) {
                Some(s) if s == "meancenter" => {
                    let mu = agg("mean", &x)?;
                    x.iter().map(|v| v - mu).collect()
                }
                Some(s) if s == "zscore" => {
                    let (mu, sd) = (agg("mean", &x)?, agg("std", &x)?);
                    x.iter().map(|v| (v - mu) / sd).collect()
                }
                Some(s) => return Err(MError::new(format!("grouptransform: unknown method '{s}' (meancenter, zscore or a function handle)"))),
                None => match it.call1(&method, vec![Value::Mat(Mat::col(x.clone()))])? {
                    Value::Mat(r) if r.numel() == x.len() => r.re,
                    _ => return Err(MError::new("grouptransform: the function must return one value per row of the group")),
                },
            };
            for (&i, v) in g.iter().zip(y) {
                res[i] = v;
            }
        }
        out.columns[k] = Value::Mat(Mat::col(res));
    }
    Ok(vec![tval(out)])
}

/// pivot(T, 'Rows', r, 'Columns', c, 'DataVariable', v, 'Method', 'sum'): pivot table; without Columns — like groupsummary.
fn pivot(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let t = table_arg(a, 0, "pivot")?;
    let o = opts(a, 1, "pivot")?;
    let rows = opt(&o, "rows").ok_or_else(|| MError::new("pivot: 'Rows' is required"))?;
    let (rv, rkeys) = group_keys(t, rows)?;
    let method = opt(&o, "method").and_then(str_of).unwrap_or_else(|| "count".into()).to_ascii_lowercase();
    let data = match opt(&o, "datavariable") {
        Some(v) => Some(tb::var_list(t, v)?[0]),
        None => None,
    };
    let h = height(t);
    let rg = tb::groups(&rkeys, h);
    let mut row_of = vec![0usize; h];
    for (gi, g) in rg.iter().enumerate() {
        for &i in g {
            row_of[i] = gi;
        }
    }
    let firsts: Vec<usize> = rg.iter().map(|g| g[0]).collect();
    let mut out = tb::select_vars(&tb::take_rows(t, &firsts), &rv);
    let (cgroups, cnames): (Vec<Vec<usize>>, Vec<String>) = match opt(&o, "columns") {
        Some(c) => {
            let ck = tb::var_list(t, c)?[0];
            let cg = tb::groups(&[&t.columns[ck]], h);
            let names = cg
                .iter()
                .map(|g| {
                    let cell = match (&t.columns[ck], t.kinds[ck]) {
                        (Value::Str(s), _) => s.data[g[0]].clone().unwrap_or_else(|| "missing".into()),
                        (Value::Mat(m), ColKind::Date) => tb::iso_date(m.re[g[0]]),
                        (Value::Mat(m), ColKind::Bool) => (m.re[g[0]] != 0.0).to_string(),
                        (Value::Mat(m), _) => fileio::num_text(m.re[g[0]]),
                        _ => String::new(),
                    };
                    tb::make_valid_name(&cell, 0)
                })
                .collect();
            (cg, names)
        }
        None => (vec![(0..h).collect()], vec![data.map_or("count".into(), |d| format!("{method}_{}", t.names[d]))]),
    };
    let mut names = cnames;
    tb::dedupe(&mut names);
    for (cg, name) in cgroups.iter().zip(names) {
        let mut cells: Vec<Vec<f64>> = vec![Vec::new(); rg.len()];
        for &i in cg {
            cells[row_of[i]].push(data.and_then(|d| tb::num_col(t, d)).map_or(1.0, |m| m.re[i]));
        }
        let col: Vec<f64> = cells
            .iter()
            .map(|c| if method == "count" { Ok(c.len() as f64) } else if c.is_empty() { Ok(f64::NAN) } else { agg(&method, c) })
            .collect::<Result<_, _>>()?;
        out.names.push(name);
        out.columns.push(Value::Mat(Mat::col(col)));
        out.kinds.push(ColKind::Num);
    }
    Ok(vec![tval(out)])
}

// ---------- joins ----------

fn join_keys(l: &Table, r: &Table, o: &[(String, Value)], fname: &str) -> Result<(Vec<usize>, Vec<usize>), MError> {
    let names: Vec<String> = match opt(o, "keys") {
        Some(v) => strs_of(v).ok_or_else(|| MError::new(format!("{fname}: 'Keys' must be names")))?,
        None => l.names.iter().filter(|n| r.names.contains(n)).cloned().collect(),
    };
    if names.is_empty() {
        return Err(MError::new(format!("{fname}: no common variables to use as keys")));
    }
    let lk = names.iter().map(|n| tb::var_index(l, n)).collect::<Result<Vec<_>, _>>()?;
    let rk = names.iter().map(|n| tb::var_index(r, n)).collect::<Result<Vec<_>, _>>()?;
    Ok((lk, rk))
}

fn combine(l: &Table, r: &Table, li: &[usize], ri: &[usize], rk: &[usize]) -> Table {
    let mut out = tb::take_rows(l, li);
    let rt = tb::take_rows(r, ri);
    for k in 0..r.names.len() {
        if rk.contains(&k) {
            continue;
        }
        let mut name = r.names[k].clone();
        if let Some(p) = out.names.iter().position(|n| *n == name) {
            out.names[p] = format!("{name}_left");
            name = format!("{name}_right");
        }
        out.names.push(name);
        out.columns.push(rt.columns[k].clone());
        out.kinds.push(rt.kinds[k]);
    }
    out
}

fn right_index(r: &Table, rk: &[usize]) -> HashMap<Vec<tb::KeyAtom>, Vec<usize>> {
    let keys: Vec<&Value> = rk.iter().map(|&k| &r.columns[k]).collect();
    let mut map: HashMap<Vec<tb::KeyAtom>, Vec<usize>> = HashMap::new();
    for j in 0..height(r) {
        if let Some(key) = tb::row_key(&keys, j) {
            map.entry(key).or_default().push(j);
        }
    }
    map
}

fn innerjoin(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let (l, r) = (table_arg(a, 0, "innerjoin")?, table_arg(a, 1, "innerjoin")?);
    let o = opts(a, 2, "innerjoin")?;
    let (lk, rk) = join_keys(l, r, &o, "innerjoin")?;
    let map = right_index(r, &rk);
    let lkeys: Vec<&Value> = lk.iter().map(|&k| &l.columns[k]).collect();
    let (mut li, mut ri) = (Vec::new(), Vec::new());
    for i in 0..height(l) {
        if let Some(js) = tb::row_key(&lkeys, i).and_then(|k| map.get(&k)) {
            for &j in js {
                li.push(i);
                ri.push(j);
            }
        }
    }
    // sort by keys (stable)
    let sortcols: Vec<Value> = lk.iter().map(|&k| tb::take_rows_value(&l.columns[k], &li)).collect();
    let keys: Vec<(&Value, bool)> = sortcols.iter().map(|v| (v, false)).collect();
    let ord = tb::sorted_rows(&keys, li.len());
    let li: Vec<usize> = ord.iter().map(|&p| li[p]).collect();
    let ri: Vec<usize> = ord.iter().map(|&p| ri[p]).collect();
    Ok(vec![tval(combine(l, r, &li, &ri, &rk))])
}

fn join(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let (l, r) = (table_arg(a, 0, "join")?, table_arg(a, 1, "join")?);
    let o = opts(a, 2, "join")?;
    let (lk, rk) = join_keys(l, r, &o, "join")?;
    let map = right_index(r, &rk);
    if map.values().any(|v| v.len() > 1) {
        return Err(MError::new("join: key values in the right table must be unique"));
    }
    let lkeys: Vec<&Value> = lk.iter().map(|&k| &l.columns[k]).collect();
    let mut ri = Vec::with_capacity(height(l));
    for i in 0..height(l) {
        match tb::row_key(&lkeys, i).and_then(|k| map.get(&k)) {
            Some(js) => ri.push(js[0]),
            None => return Err(MError::new(format!("join: the key in row {} of the left table has no match in the right table", i + 1))),
        }
    }
    let li: Vec<usize> = (0..height(l)).collect();
    Ok(vec![tval(combine(l, r, &li, &ri, &rk))])
}

// ---------- missing values ----------

fn ismissing(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    match arg(a, 0, "ismissing")? {
        Value::Mat(m) => {
            let re = m.re.iter().map(|x| f64::from(u8::from(x.is_nan() && m.class != Class::Char))).collect();
            one(Mat { rows: m.rows, cols: m.cols, re, im: None, class: Class::Logical })
        }
        Value::Str(s) => one(Mat { rows: s.rows, cols: s.cols, re: s.data.iter().map(|x| f64::from(u8::from(x.is_none()))).collect(), im: None, class: Class::Logical }),
        Value::Table(t) => {
            let h = height(t);
            let w = t.names.len();
            let mut m = Mat::zeros(h, w).with_class(Class::Logical);
            for k in 0..w {
                for i in 0..h {
                    m.re[k * h + i] = f64::from(u8::from(tb::is_missing(&t.columns[k], i)));
                }
            }
            one(m)
        }
        other => Err(MError::new(format!("ismissing: wrong type argument '{}'", other.class_name()))),
    }
}

fn rmmissing(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    match arg(a, 0, "rmmissing")? {
        Value::Mat(m) if m.rows == 1 || m.cols == 1 => {
            let v: Vec<f64> = m.re.iter().copied().filter(|x| !x.is_nan()).collect();
            one(if m.rows == 1 { Mat::row(v) } else { Mat::col(v) })
        }
        Value::Mat(m) => {
            let keep: Vec<usize> = (0..m.rows).filter(|&i| (0..m.cols).all(|j| !m.at(i, j).is_nan())).collect();
            let mut out = Mat::zeros(keep.len(), m.cols);
            for (ni, &i) in keep.iter().enumerate() {
                for j in 0..m.cols {
                    out.re[j * keep.len() + ni] = m.at(i, j);
                }
            }
            one(out)
        }
        Value::Str(s) => Ok(vec![Value::Str(Rc::new(StrArr::col(s.data.iter().filter(|x| x.is_some()).cloned().collect())))]),
        Value::Table(t) => {
            let keep: Vec<usize> = (0..height(t)).filter(|&i| t.columns.iter().all(|c| !tb::is_missing(c, i))).collect();
            Ok(vec![tval(tb::take_rows(t, &keep))])
        }
        other => Err(MError::new(format!("rmmissing: wrong type argument '{}'", other.class_name()))),
    }
}

// ---------- moving windows ----------

fn moving(_: &mut Interp, a: &[Value], _: usize, name: &str) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, name)?;
    let w = mat_arg(a, 1, name)?;
    let (kb, kf) = match w.numel() {
        1 => {
            let k = w.re[0];
            if k < 1.0 || k != k.trunc() {
                return Err(MError::new(format!("{name}: window length must be a positive integer")));
            }
            let k = k as usize;
            if k % 2 == 1 { ((k - 1) / 2, (k - 1) / 2) } else { (k / 2, k / 2 - 1) }
        }
        2 => (w.re[0].max(0.0) as usize, w.re[1].max(0.0) as usize),
        _ => return Err(MError::new(format!("{name}: window must be a scalar or [kb kf]"))),
    };
    let (n, ncols) = if m.rows == 1 { (m.cols, 1) } else { (m.rows, m.cols) };
    let mut out = m.clone();
    out.class = Class::Double;
    for c in 0..ncols {
        let x: Vec<f64> = (0..n).map(|i| if m.rows == 1 { m.re[i] } else { m.re[c * n + i] }).collect();
        for i in 0..n {
            let lo = i.saturating_sub(kb);
            let hi = (i + kf).min(n - 1);
            let win = &x[lo..=hi];
            let v = match name {
                "movsum" => win.iter().sum(),
                "movmean" => win.iter().sum::<f64>() / win.len() as f64,
                "movmax" => win.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                "movmin" => win.iter().copied().fold(f64::INFINITY, f64::min),
                _ => median_sorted(&sorted(win.to_vec())),
            };
            let k = if m.rows == 1 { i } else { c * n + i };
            out.re[k] = v;
        }
    }
    one(out)
}

// ---------- files, dates ----------

fn tempname(it: &mut Interp, _: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    Ok(vec![Value::str(&it.fs.temp_name("mlab_"))])
}

/// File builtins go through `it.fs` (the `vfs::FileSystem` trait): in memory, in a root-confined directory, or the real FS.
pub(crate) fn read_file(it: &mut Interp, path: &str, fname: &str) -> Result<String, MError> {
    it.fs.read_to_string(path).map_err(|e| MError::new(format!("{fname}: cannot open '{path}': {e}")))
}

pub(crate) fn write_file(it: &mut Interp, path: &str, text: &str, fname: &str) -> Result<(), MError> {
    it.fs.write(path, text.as_bytes()).map_err(|e| MError::new(format!("{fname}: cannot write '{path}': {e}")))
}

fn fileread(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let p = str_arg(a, 0, "fileread")?;
    Ok(vec![Value::str(&read_file(it, &p, "fileread")?)])
}

fn writelines(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let lines = arg(a, 0, "writelines").map(strs_of)?.ok_or_else(|| invalid_call("writelines"))?;
    let p = str_arg(a, 1, "writelines")?;
    let text: String = lines.iter().map(|l| format!("{l}\n")).collect();
    write_file(it, &p, &text, "writelines")?;
    Ok(vec![])
}

fn datenum(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    if let Some(ss) = arg(a, 0, "datenum").map(|v| if is_str(v) || matches!(v, Value::Str(_)) { strs_of(v) } else { None })? {
        let v: Vec<f64> = ss.iter().map(|s| tb::parse_iso(s).ok_or_else(|| MError::new(format!("datenum: cannot parse '{s}' (expected YYYY-MM-DD)")))).collect::<Result<_, _>>()?;
        return one(Mat::col(v));
    }
    let m = mat_arg(a, 0, "datenum")?;
    if a.len() == 1 && m.cols >= 3 {
        let v = (0..m.rows).map(|i| tb::datenum_ymd(m.at(i, 0), m.at(i, 1), m.at(i, 2))).collect();
        return one(Mat::col(v));
    }
    let y = super::scalar_arg(a, 0, "datenum")?;
    let mo = if a.len() > 1 { super::scalar_arg(a, 1, "datenum")? } else { 1.0 };
    let d = if a.len() > 2 { super::scalar_arg(a, 2, "datenum")? } else { 1.0 };
    let hh = if a.len() > 3 { super::scalar_arg(a, 3, "datenum")? } else { 0.0 };
    let mi = if a.len() > 4 { super::scalar_arg(a, 4, "datenum")? } else { 0.0 };
    let s = if a.len() > 5 { super::scalar_arg(a, 5, "datenum")? } else { 0.0 };
    num(tb::datenum_ymd(y, mo, d) + (hh * 3600.0 + mi * 60.0 + s) / 86400.0)
}

fn datestr(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "datestr")?;
    let fmt = match a.get(1) {
        Some(v) => str_of(v).ok_or_else(|| invalid_call("datestr"))?,
        None => {
            if m.re.iter().all(|x| *x == x.floor()) { "dd-mmm-yyyy".into() } else { "dd-mmm-yyyy HH:MM:SS".into() }
        }
    };
    let lines: Vec<String> = m.re.iter().map(|&x| tb::datestr(x, &fmt)).collect();
    if lines.len() == 1 {
        return Ok(vec![Value::str(&lines[0])]);
    }
    Ok(vec![Value::Str(Rc::new(StrArr::col(lines.into_iter().map(Some).collect())))])
}

fn ext_of(path: &str) -> String {
    std::path::Path::new(path).extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default()
}

fn delim_for(path: &str, o: &[(String, Value)]) -> Result<char, MError> {
    if let Some(d) = opt(o, "delimiter").and_then(str_of) {
        return Ok(match d.as_str() {
            "tab" | "\\t" | "\t" => '\t',
            "comma" | "," => ',',
            "semi" | "semicolon" | ";" => ';',
            "space" | " " => ' ',
            "bar" | "|" => '|',
            other => other.chars().next().ok_or_else(|| MError::new("empty delimiter"))?,
        });
    }
    Ok(if matches!(ext_of(path).as_str(), "tsv" | "tab") { '\t' } else { ',' })
}

fn file_kind(path: &str, o: &[(String, Value)], fname: &str) -> Result<&'static str, MError> {
    let ft = opt(o, "filetype").and_then(str_of).map(|s| s.to_ascii_lowercase());
    let e = ext_of(path);
    match ft.as_deref().unwrap_or(e.as_str()) {
        "json" => Ok("json"),
        "xlsx" | "xls" | "ods" | "spreadsheet" => Err(MError::new(format!("{fname}: spreadsheet files are not supported yet (planned: calamine / rust_xlsxwriter); use CSV or JSON"))),
        _ => Ok("text"),
    }
}

fn readtable(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let p = str_arg(a, 0, "readtable")?;
    let o = opts(a, 1, "readtable")?;
    let kind = file_kind(&p, &o, "readtable")?;
    let text = read_file(it, &p, "readtable")?;
    let t = if kind == "json" {
        fileio::json_to_table(&fileio::parse_json(&text, "readtable")?, "readtable")?
    } else {
        fileio::read_csv_table(&text, delim_for(&p, &o)?, "readtable")?
    };
    Ok(vec![tval(t)])
}

fn writetable(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let t = table_arg(a, 0, "writetable")?;
    let p = str_arg(a, 1, "writetable")?;
    let o = opts(a, 2, "writetable")?;
    let text = if file_kind(&p, &o, "writetable")? == "json" { fileio::write_json_table(t) } else { fileio::write_csv_table(t, delim_for(&p, &o)?) };
    write_file(it, &p, &text, "writetable")?;
    Ok(vec![])
}

fn readmatrix(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let p = str_arg(a, 0, "readmatrix")?;
    let o = opts(a, 1, "readmatrix")?;
    let text = read_file(it, &p, "readmatrix")?;
    one(fileio::read_matrix(&text, delim_for(&p, &o)?, "readmatrix")?)
}

fn writematrix(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "writematrix")?;
    let p = str_arg(a, 1, "writematrix")?;
    let o = opts(a, 2, "writematrix")?;
    write_file(it, &p, &fileio::write_matrix(m, delim_for(&p, &o)?), "writematrix")?;
    Ok(vec![])
}

fn jsonencode(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let v = arg(a, 0, "jsonencode")?;
    Ok(vec![Value::str(&fileio::json_text(&fileio::value_to_json(v), false))])
}

/// string(x): text → string array; numbers → their textual form.
fn string_b(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let v = arg(a, 0, "string")?;
    let s = match v {
        Value::Str(s) => (**s).clone(),
        Value::Mat(m) if m.class == Class::Char => StrArr::row(vec![Some(m.to_string_lossy())]),
        Value::Mat(m) => StrArr { rows: m.rows, cols: m.cols, data: m.re.iter().map(|&x| if x.is_nan() { None } else { Some(fileio::num_text(x)) }).collect() },
        other => return Err(MError::new(format!("string: wrong type argument '{}'", other.class_name()))),
    };
    Ok(vec![Value::Str(Rc::new(s))])
}
