//! v2: tables — in-memory columns (each a column vector), display, keys for sorting, grouping and
//! joins, dates (datenum: 719529 = 1970-01-01, as in MATLAB).

use crate::display::Format;
use crate::interp::MError;
use crate::value::{Class, ColKind, Mat, StrArr, Table, Value};
use std::cmp::Ordering;
use std::rc::Rc;

pub const EPOCH_DATENUM: f64 = 719529.0;

// ---------- dates ----------

/// Days since 1970-01-01 (the «days from civil» algorithm, Hinnant, public domain).
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

pub fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn datenum_ymd(y: f64, m: f64, d: f64) -> f64 {
    // months outside 1..12 carry into years, days are simply added (as in MATLAB)
    let mm = m - 1.0;
    let y2 = y + (mm / 12.0).floor();
    let m2 = mm.rem_euclid(12.0) + 1.0;
    days_from_civil(y2 as i64, m2 as i64, 1) as f64 + (d - 1.0) + EPOCH_DATENUM
}

/// «YYYY-MM-DD» or «YYYY-MM-DD HH:MM[:SS]» / «…THH:MM…» → datenum.
pub fn parse_iso(s: &str) -> Option<f64> {
    let s = s.trim();
    let b = s.as_bytes();
    if b.len() < 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let num = |r: std::ops::Range<usize>| -> Option<i64> {
        let t = s.get(r)?;
        if t.bytes().all(|c| c.is_ascii_digit()) { t.parse().ok() } else { None }
    };
    let (y, m, d) = (num(0..4)?, num(5..7)?, num(8..10)?);
    if !(1..=12).contains(&m) || d < 1 || d > 31 {
        return None;
    }
    let mut frac = 0.0;
    if b.len() > 10 {
        if b[10] != b' ' && b[10] != b'T' {
            return None;
        }
        let rest = &s[11..];
        let parts: Vec<&str> = rest.split(':').collect();
        if parts.len() < 2 || parts.len() > 3 {
            return None;
        }
        let hh: f64 = parts[0].parse().ok()?;
        let mi: f64 = parts[1].parse().ok()?;
        let ss: f64 = if parts.len() == 3 { parts[2].parse().ok()? } else { 0.0 };
        frac = (hh * 3600.0 + mi * 60.0 + ss) / 86400.0;
    }
    let (cy, cm, cd) = civil_from_days(days_from_civil(y, m, d));
    if (cy, cm, cd) != (y, m, d) {
        return None; // 2026-02-30 etc.
    }
    Some(days_from_civil(y, m, d) as f64 + EPOCH_DATENUM + frac)
}

const MON: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/// datenum → text by pattern (yyyy, mmm, mm, dd, HH, MM, SS).
pub fn datestr(x: f64, fmt: &str) -> String {
    if !x.is_finite() {
        return "NaT".into();
    }
    let day = x.floor();
    let mut secs = ((x - day) * 86400.0).round() as i64;
    let mut dn = day as i64 - EPOCH_DATENUM as i64;
    if secs >= 86400 {
        secs -= 86400;
        dn += 1;
    }
    let (y, m, d) = civil_from_days(dn);
    let (hh, mi, ss) = (secs / 3600, (secs / 60) % 60, secs % 60);
    let mut out = String::new();
    let f: Vec<char> = fmt.chars().collect();
    let mut i = 0;
    let at = |i: usize, pat: &str| -> bool { pat.chars().enumerate().all(|(k, c)| f.get(i + k) == Some(&c)) };
    while i < f.len() {
        if at(i, "yyyy") {
            out.push_str(&format!("{y:04}"));
            i += 4;
        } else if at(i, "mmm") {
            out.push_str(MON[(m - 1) as usize]);
            i += 3;
        } else if at(i, "mm") {
            out.push_str(&format!("{m:02}"));
            i += 2;
        } else if at(i, "dd") {
            out.push_str(&format!("{d:02}"));
            i += 2;
        } else if at(i, "HH") {
            out.push_str(&format!("{hh:02}"));
            i += 2;
        } else if at(i, "MM") {
            out.push_str(&format!("{mi:02}"));
            i += 2;
        } else if at(i, "SS") {
            out.push_str(&format!("{ss:02}"));
            i += 2;
        } else {
            out.push(f[i]);
            i += 1;
        }
    }
    out
}

pub fn iso_date(x: f64) -> String {
    if !x.is_finite() {
        return "NaT".into();
    }
    if x == x.floor() { datestr(x, "yyyy-mm-dd") } else { datestr(x, "yyyy-mm-dd HH:MM:SS") }
}

// ---------- columns ----------

pub fn col_len(v: &Value) -> usize {
    match v {
        Value::Mat(m) => m.numel(),
        Value::Str(s) => s.data.len(),
        _ => 1,
    }
}

pub fn height(t: &Table) -> usize {
    t.columns.first().map_or(0, col_len)
}

fn table_err(msg: impl Into<String>) -> MError {
    MError::new(format!("table: {}", msg.into()))
}

/// Value → table column (column vector) and its type.
pub fn to_column(v: &Value, name: &str) -> Result<(Value, ColKind), MError> {
    match v {
        Value::Mat(m) => {
            if m.is_complex() {
                return Err(table_err(format!("variable '{name}': complex columns are not supported")));
            }
            if m.class == Class::Char {
                // char matrix: each row is one text
                let data = (0..m.rows).map(|i| Some(m.row_string(i))).collect();
                return Ok((Value::Str(Rc::new(StrArr::col(data))), ColKind::Text));
            }
            if m.rows > 1 && m.cols > 1 {
                return Err(table_err(format!("variable '{name}' must be a vector ({}x{} given)", m.rows, m.cols)));
            }
            let kind = if m.class == Class::Logical { ColKind::Bool } else { ColKind::Num };
            let mut c = m.clone();
            c.rows = m.numel();
            c.cols = 1;
            Ok((Value::Mat(c), kind))
        }
        Value::Str(s) => {
            if s.rows > 1 && s.cols > 1 {
                return Err(table_err(format!("variable '{name}' must be a vector")));
            }
            Ok((Value::Str(Rc::new(StrArr::col(s.data.clone()))), ColKind::Text))
        }
        other => Err(table_err(format!("variable '{name}': {} values cannot be table columns", other.class_name()))),
    }
}

pub fn valid_name(s: &str) -> bool {
    let mut ch = s.chars();
    matches!(ch.next(), Some(c) if c.is_alphabetic() || c == '_') && ch.all(|c| c.is_alphanumeric() || c == '_')
}

/// Name from a file → identifier (like makeValidName): spaces and other characters → «_», a non-letter at the start → «x».
pub fn make_valid_name(s: &str, k: usize) -> String {
    let t = s.trim();
    if t.is_empty() {
        return format!("Var{}", k + 1);
    }
    let mut out: String = t.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect();
    if !out.chars().next().is_some_and(|c| c.is_alphabetic()) {
        out = format!("x{out}");
    }
    out
}

pub fn dedupe(names: &mut [String]) {
    for i in 1..names.len() {
        let mut k = 1;
        let base = names[i].clone();
        while names[..i].contains(&names[i]) {
            names[i] = format!("{base}_{k}");
            k += 1;
        }
    }
}

pub fn make(names: Vec<String>, values: &[Value]) -> Result<Table, MError> {
    let mut t = Table { names: Vec::new(), columns: Vec::new(), kinds: Vec::new() };
    let mut h: Option<usize> = None;
    for (name, v) in names.into_iter().zip(values) {
        if !valid_name(&name) {
            return Err(table_err(format!("invalid variable name '{name}'")));
        }
        if t.names.contains(&name) {
            return Err(table_err(format!("duplicate variable name '{name}'")));
        }
        let (c, kind) = to_column(v, &name)?;
        let n = col_len(&c);
        if let Some(h0) = h {
            if n != h0 {
                return Err(table_err(format!("all variables must have the same number of rows ({h0} vs {n})")));
            }
        }
        h = Some(n);
        t.names.push(name);
        t.columns.push(c);
        t.kinds.push(kind);
    }
    Ok(t)
}

pub fn var_index(t: &Table, name: &str) -> Result<usize, MError> {
    t.names.iter().position(|n| n == name).ok_or_else(|| table_err(format!("unknown variable '{name}' (variables: {})", t.names.join(", "))))
}

/// Variable names from an argument: 'x', {'a','b'}, numbers, logical mask.
pub fn var_list(t: &Table, v: &Value) -> Result<Vec<usize>, MError> {
    match v {
        Value::Mat(m) if m.class == Class::Char => Ok(vec![var_index(t, &m.to_string_lossy())?]),
        Value::Str(s) => s.data.iter().map(|n| var_index(t, n.as_deref().unwrap_or(""))).collect(),
        Value::Mat(m) if m.class == Class::Logical => Ok((0..m.numel()).filter(|&k| m.re[k] != 0.0).collect()),
        Value::Mat(m) => m
            .re
            .iter()
            .map(|&x| {
                if x >= 1.0 && x == x.trunc() && (x as usize) <= t.names.len() {
                    Ok(x as usize - 1)
                } else {
                    Err(table_err(format!("variable index {x} out of bound {}", t.names.len())))
                }
            })
            .collect(),
        other => Err(table_err(format!("variables must be names or indices, not {}", other.class_name()))),
    }
}

pub fn take_rows_value(v: &Value, idx: &[usize]) -> Value {
    match v {
        Value::Mat(m) => {
            let re = idx.iter().map(|&k| m.re[k]).collect::<Vec<f64>>();
            Value::Mat(Mat { rows: idx.len(), cols: 1, re, im: None, class: m.class })
        }
        Value::Str(s) => Value::Str(Rc::new(StrArr::col(idx.iter().map(|&k| s.data[k].clone()).collect()))),
        other => other.clone(),
    }
}

pub fn take_rows(t: &Table, idx: &[usize]) -> Table {
    Table { names: t.names.clone(), columns: t.columns.iter().map(|c| take_rows_value(c, idx)).collect(), kinds: t.kinds.clone() }
}

pub fn select_vars(t: &Table, vars: &[usize]) -> Table {
    Table {
        names: vars.iter().map(|&k| t.names[k].clone()).collect(),
        columns: vars.iter().map(|&k| t.columns[k].clone()).collect(),
        kinds: vars.iter().map(|&k| t.kinds[k]).collect(),
    }
}

/// `T.name = v`: a new or replaced column; a scalar is broadcast to all rows.
pub fn set_var(t: &mut Table, name: &str, v: &Value) -> Result<(), MError> {
    if !valid_name(name) {
        return Err(table_err(format!("invalid variable name '{name}'")));
    }
    let (mut c, kind) = to_column(v, name)?;
    let h = height(t);
    let n = col_len(&c);
    if !t.columns.is_empty() && n != h {
        if n == 1 {
            c = take_rows_value(&c, &vec![0; h]);
        } else {
            let what = if t.names.iter().any(|x| x == name) { "variable" } else { "new variable" };
            return Err(table_err(format!("{what} '{name}' must have {h} rows, got {n}")));
        }
    }
    match t.names.iter().position(|x| x == name) {
        Some(k) => {
            // a date stays a date if numbers are assigned
            let keep_date = t.kinds[k] == ColKind::Date && kind == ColKind::Num;
            t.columns[k] = c;
            t.kinds[k] = if keep_date { ColKind::Date } else { kind };
        }
        None => {
            t.names.push(name.to_string());
            t.columns.push(c);
            t.kinds.push(kind);
        }
    }
    Ok(())
}

pub fn num_col(t: &Table, k: usize) -> Option<&Mat> {
    match &t.columns[k] {
        Value::Mat(m) => Some(m),
        _ => None,
    }
}

pub fn is_missing(v: &Value, i: usize) -> bool {
    match v {
        Value::Mat(m) => m.re[i].is_nan(),
        Value::Str(s) => s.data[i].is_none(),
        _ => false,
    }
}

// ---------- row comparison (sorting, groups, joins) ----------

fn cmp_f64(a: f64, b: f64) -> Ordering {
    match (a.is_nan(), b.is_nan()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Greater, // NaN — last in ascending order
        (false, true) => Ordering::Less,
        _ => a.partial_cmp(&b).unwrap(),
    }
}

pub fn cmp_cell(v: &Value, i: usize, j: usize) -> Ordering {
    match v {
        Value::Mat(m) => cmp_f64(m.re[i], m.re[j]),
        Value::Str(s) => match (&s.data[i], &s.data[j]) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(a), Some(b)) => a.cmp(b),
        },
        _ => Ordering::Equal,
    }
}

/// Stable sort of row indices by keys (column, descending?).
pub fn sorted_rows(cols: &[(&Value, bool)], n: usize) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..n).collect();
    // a single numeric key — fast path: sort (value, index) pairs
    if let [(Value::Mat(m), desc)] = cols {
        let mut pairs: Vec<(f64, usize)> = m.re.iter().copied().zip(0..n).collect();
        pairs.sort_by(|a, b| {
            let o = cmp_f64(a.0, b.0);
            let o = if *desc { o.reverse() } else { o };
            o.then(a.1.cmp(&b.1))
        });
        return pairs.into_iter().map(|p| p.1).collect();
    }
    idx.sort_by(|&i, &j| {
        for (c, desc) in cols {
            let o = cmp_cell(c, i, j);
            let o = if *desc { o.reverse() } else { o };
            if o != Ordering::Equal {
                return o;
            }
        }
        Ordering::Equal
    });
    idx
}

/// Groups by keys: (first row of the group, rows of the group), groups ordered by keys.
pub fn groups(keys: &[&Value], n: usize) -> Vec<Vec<usize>> {
    let cols: Vec<(&Value, bool)> = keys.iter().map(|k| (*k, false)).collect();
    let order = sorted_rows(&cols, n);
    let mut out: Vec<Vec<usize>> = Vec::new();
    for &r in &order {
        let same = match out.last() {
            Some(g) => keys.iter().all(|k| cmp_cell(k, g[0], r) == Ordering::Equal),
            None => false,
        };
        if same {
            out.last_mut().unwrap().push(r);
        } else {
            out.push(vec![r]);
        }
    }
    // within a group — order of appearance
    for g in &mut out {
        g.sort_unstable();
    }
    out
}

/// Row key for hashing in joins; None — has a missing value (missing values do not match).
pub fn row_key(keys: &[&Value], i: usize) -> Option<Vec<KeyAtom>> {
    let mut out = Vec::with_capacity(keys.len());
    for k in keys {
        match k {
            Value::Mat(m) => {
                let x = m.re[i];
                if x.is_nan() {
                    return None;
                }
                out.push(KeyAtom::Num(if x == 0.0 { 0u64 } else { x.to_bits() }));
            }
            Value::Str(s) => out.push(KeyAtom::Text(s.data[i].clone()?)),
            _ => return None,
        }
    }
    Some(out)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum KeyAtom {
    Num(u64),
    Text(String),
}

// ---------- display ----------

fn cells_of(t: &Table, k: usize, fmt: Format) -> Vec<String> {
    let v = &t.columns[k];
    match (t.kinds[k], v) {
        (ColKind::Text, Value::Str(s)) => s.data.iter().map(|x| x.clone().unwrap_or_else(|| "<missing>".into())).collect(),
        (ColKind::Bool, Value::Mat(m)) => m.re.iter().map(|&x| if x != 0.0 { "true".into() } else { "false".into() }).collect(),
        (ColKind::Date, Value::Mat(m)) => m.re.iter().map(|&x| iso_date(x)).collect(),
        (_, Value::Mat(m)) => {
            if m.numel() == 0 {
                return vec![];
            }
            let mut dm = m.clone();
            dm.class = Class::Double;
            let (cells, _) = crate::display::matrix_cells(&dm, fmt);
            cells.into_iter().map(|row| row[0].trim().to_string()).collect()
        }
        _ => vec![],
    }
}

const SHOW_HEAD: usize = 10;
const SHOW_TAIL: usize = 5;
const SHOW_ALL_UPTO: usize = 30;

/// Table body: header, underline, rows; numbers right-aligned, text left-aligned; 4 spaces of indentation,
/// 3 — between columns; trailing spaces stripped.
pub fn format_body(t: &Table, fmt: Format) -> String {
    let h = height(t);
    let rows: Vec<usize> =
        if h <= SHOW_ALL_UPTO { (0..h).collect() } else { (0..SHOW_HEAD).chain(h - SHOW_TAIL..h).collect() };
    let mut cols: Vec<(Vec<String>, usize, bool)> = Vec::new();
    for k in 0..t.names.len() {
        let all = cells_of(t, k, fmt);
        let shown: Vec<String> = rows.iter().map(|&r| all.get(r).cloned().unwrap_or_default()).collect();
        let w = shown.iter().map(|s| s.chars().count()).max().unwrap_or(0).max(t.names[k].chars().count());
        cols.push((shown, w, t.kinds[k] == ColKind::Text));
    }
    let pad = |s: &str, w: usize, left: bool| -> String {
        let n = s.chars().count();
        let fill = " ".repeat(w.saturating_sub(n));
        if left { format!("{s}{fill}") } else { format!("{fill}{s}") }
    };
    let line = |cells: Vec<String>| -> String { format!("    {}", cells.join("   ")).trim_end().to_string() + "\n" };
    let mut out = String::new();
    out.push_str(&line(cols.iter().enumerate().map(|(k, (_, w, l))| pad(&t.names[k], *w, *l)).collect()));
    out.push_str(&line(cols.iter().map(|(_, w, _)| "_".repeat(*w)).collect()));
    for (ri, _) in rows.iter().enumerate() {
        if h > SHOW_ALL_UPTO && ri == SHOW_HEAD {
            out.push_str(&format!("    … ({} rows skipped)\n", h - SHOW_HEAD - SHOW_TAIL));
        }
        out.push_str(&line(cols.iter().map(|(c, w, l)| pad(&c[ri], *w, *l)).collect()));
    }
    out
}

pub fn format_named(name: &str, t: &Table, fmt: Format) -> String {
    format!("{name} =\n\n  {}x{} table\n\n{}\n", height(t), t.names.len(), format_body(t, fmt))
}

pub fn format_str_arr(s: &StrArr) -> String {
    let mut out = String::new();
    for i in 0..s.rows {
        let row: Vec<String> = (0..s.cols).map(|j| s.data[j * s.rows + i].clone().unwrap_or_else(|| "<missing>".into())).collect();
        out.push_str(&row.join("  "));
        out.push('\n');
    }
    out
}
