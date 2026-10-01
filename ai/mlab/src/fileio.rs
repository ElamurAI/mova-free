//! v2: table import and export — CSV/TSV (RFC 4180: quotes, doubled quotes, line breaks inside quotes) and JSON
//! (array of records). Our own parsers: an error names the file's row and column.

use crate::interp::MError;
use crate::table::{self, iso_date, make_valid_name, parse_iso};
use crate::value::{Class, ColKind, Mat, StrArr, Table, Value};
use std::rc::Rc;

pub struct Field {
    pub text: String,
    pub quoted: bool,
    pub line: usize,
    pub col: usize,
}

pub struct Record {
    pub line: usize,
    pub fields: Vec<Field>,
    /// end-of-line position (for «too few fields»)
    pub end_col: usize,
}

fn at(fname: &str, line: usize, col: usize, msg: &str) -> MError {
    MError::new(format!("{fname}: line {line}, column {col}: {msg}"))
}

/// CSV/TSV parsing. Empty lines are skipped.
pub fn parse_csv(text: &str, delim: char, fname: &str) -> Result<Vec<Record>, MError> {
    let chars: Vec<char> = text.trim_start_matches('\u{feff}').chars().collect();
    let n = chars.len();
    let (mut i, mut line, mut col) = (0usize, 1usize, 1usize);
    let mut out = Vec::new();
    while i < n {
        if chars[i] == '\n' || (chars[i] == '\r' && i + 1 < n && chars[i + 1] == '\n') {
            i += if chars[i] == '\r' { 2 } else { 1 };
            line += 1;
            col = 1;
            continue;
        }
        let rec_line = line;
        let mut fields = Vec::new();
        loop {
            let (fl, fc) = (line, col);
            let mut buf = String::new();
            let quoted = i < n && chars[i] == '"';
            if quoted {
                i += 1;
                col += 1;
                loop {
                    if i >= n {
                        return Err(at(fname, fl, fc, "unterminated quoted field"));
                    }
                    let c = chars[i];
                    if c == '"' {
                        if i + 1 < n && chars[i + 1] == '"' {
                            buf.push('"');
                            i += 2;
                            col += 2;
                            continue;
                        }
                        i += 1;
                        col += 1;
                        break;
                    }
                    buf.push(c);
                    i += 1;
                    if c == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                }
                if i < n && chars[i] != delim && chars[i] != '\n' && chars[i] != '\r' {
                    return Err(at(fname, line, col, "unexpected character after closing quote"));
                }
            } else {
                while i < n && chars[i] != delim && chars[i] != '\n' && !(chars[i] == '\r' && i + 1 < n && chars[i + 1] == '\n') {
                    buf.push(chars[i]);
                    i += 1;
                    col += 1;
                }
            }
            fields.push(Field { text: buf, quoted, line: fl, col: fc });
            if i < n && chars[i] == delim {
                i += 1;
                col += 1;
                continue;
            }
            let end_col = col;
            if i < n {
                i += if chars[i] == '\r' { 2 } else { 1 };
            }
            line += 1;
            col = 1;
            out.push(Record { line: rec_line, fields, end_col });
            break;
        }
    }
    Ok(out)
}

fn is_bool(s: &str) -> Option<bool> {
    match s.trim().to_ascii_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

pub fn parse_num(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    match t {
        "NaN" | "nan" | "NA" => return Some(f64::NAN),
        "Inf" | "inf" | "+Inf" => return Some(f64::INFINITY),
        "-Inf" | "-inf" => return Some(f64::NEG_INFINITY),
        _ => {}
    }
    if t.chars().any(|c| c.is_alphabetic() && c != 'e' && c != 'E') {
        return None;
    }
    t.parse::<f64>().ok()
}

/// Cell: text or missing (empty without quotes).
pub struct Cell {
    pub text: Option<String>,
    pub quoted: bool,
}

/// Columns of cells → typed table columns.
pub fn infer_column(cells: &[Cell]) -> (Value, ColKind) {
    let present: Vec<&Cell> = cells.iter().filter(|c| c.text.is_some()).collect();
    let any_quoted = present.iter().any(|c| c.quoted);
    let n = cells.len();
    if !any_quoted {
        if present.iter().all(|c| parse_num(c.text.as_deref().unwrap()).is_some()) {
            let re = cells.iter().map(|c| c.text.as_deref().and_then(parse_num).unwrap_or(f64::NAN)).collect();
            return (Value::Mat(Mat::col(re)), ColKind::Num);
        }
        if present.iter().all(|c| is_bool(c.text.as_deref().unwrap()).is_some()) {
            let vals: Vec<Option<bool>> = cells.iter().map(|c| c.text.as_deref().and_then(is_bool)).collect();
            if vals.iter().all(|v| v.is_some()) {
                let re = vals.iter().map(|v| if v.unwrap() { 1.0 } else { 0.0 }).collect();
                return (Value::Mat(Mat::col(re).with_class(Class::Logical)), ColKind::Bool);
            }
            let re = vals.iter().map(|v| v.map_or(f64::NAN, |b| if b { 1.0 } else { 0.0 })).collect();
            return (Value::Mat(Mat::col(re)), ColKind::Num);
        }
        if present.iter().all(|c| parse_iso(c.text.as_deref().unwrap()).is_some()) {
            let re = cells.iter().map(|c| c.text.as_deref().and_then(parse_iso).unwrap_or(f64::NAN)).collect();
            return (Value::Mat(Mat::col(re)), ColKind::Date);
        }
    }
    let _ = n;
    (Value::Str(Rc::new(StrArr::col(cells.iter().map(|c| c.text.clone()).collect()))), ColKind::Text)
}

fn build(names: Vec<String>, cols: Vec<Vec<Cell>>) -> Table {
    let mut t = Table { names: Vec::new(), columns: Vec::new(), kinds: Vec::new() };
    for (name, cells) in names.into_iter().zip(cols) {
        let (v, k) = infer_column(&cells);
        t.names.push(name);
        t.columns.push(v);
        t.kinds.push(k);
    }
    t
}

pub fn read_csv_table(text: &str, delim: char, fname: &str) -> Result<Table, MError> {
    let recs = parse_csv(text, delim, fname)?;
    let Some(head) = recs.first() else { return Ok(Table { names: vec![], columns: vec![], kinds: vec![] }) };
    let mut names: Vec<String> = head.fields.iter().enumerate().map(|(k, f)| make_valid_name(&f.text, k)).collect();
    table::dedupe(&mut names);
    let w = names.len();
    let mut cols: Vec<Vec<Cell>> = (0..w).map(|_| Vec::with_capacity(recs.len())).collect();
    for r in &recs[1..] {
        if r.fields.len() != w {
            let (l, c) = if r.fields.len() > w { (r.fields[w].line, r.fields[w].col) } else { (r.line, r.end_col) };
            return Err(at(fname, l, c, &format!("expected {w} fields (as in header), found {}", r.fields.len())));
        }
        for (k, f) in r.fields.iter().enumerate() {
            let text = if f.text.is_empty() && !f.quoted { None } else { Some(f.text.clone()) };
            cols[k].push(Cell { text, quoted: f.quoted });
        }
    }
    Ok(build(names, cols))
}

/// Number → text without loss: an integer — as an integer, otherwise — the shortest exact form.
pub fn num_text(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Inf".into() } else { "-Inf".into() };
    }
    if x == x.trunc() && x.abs() < 1e15 {
        return format!("{}", x as i64);
    }
    let s = format!("{x}");
    if s.len() > 20 { format!("{x:e}") } else { s }
}

fn csv_quote(s: &str, delim: char) -> String {
    let looks_typed = parse_num(s).is_some() || is_bool(s).is_some() || parse_iso(s).is_some();
    if s.is_empty() || looks_typed || s.contains(delim) || s.contains('"') || s.contains('\n') || s.contains('\r') || s.trim() != s {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn cell_text(t: &Table, k: usize, i: usize, delim: char) -> String {
    match (&t.columns[k], t.kinds[k]) {
        (Value::Mat(m), ColKind::Bool) => (if m.re[i] != 0.0 { "true" } else { "false" }).into(),
        (Value::Mat(m), ColKind::Date) => if m.re[i].is_nan() { String::new() } else { iso_date(m.re[i]) },
        (Value::Mat(m), _) => if m.re[i].is_nan() { String::new() } else { num_text(m.re[i]) },
        (Value::Str(s), _) => s.data[i].as_deref().map_or(String::new(), |x| csv_quote(x, delim)),
        _ => String::new(),
    }
}

pub fn write_csv_table(t: &Table, delim: char) -> String {
    let d = delim.to_string();
    let mut out = t.names.iter().map(|n| csv_quote(n, delim)).collect::<Vec<_>>().join(&d);
    out.push('\n');
    for i in 0..table::height(t) {
        let row: Vec<String> = (0..t.names.len()).map(|k| cell_text(t, k, i, delim)).collect();
        out.push_str(&row.join(&d));
        out.push('\n');
    }
    out
}

pub fn read_matrix(text: &str, delim: char, fname: &str) -> Result<Mat, MError> {
    let recs = parse_csv(text, delim, fname)?;
    let numeric = |r: &Record| r.fields.iter().all(|f| f.text.trim().is_empty() || parse_num(&f.text).is_some());
    let skip = usize::from(recs.first().is_some_and(|r| !numeric(r)));
    let rows: Vec<&Record> = recs[skip..].iter().collect();
    let w = rows.iter().map(|r| r.fields.len()).max().unwrap_or(0);
    let h = rows.len();
    let mut m = Mat::filled(h, w, f64::NAN);
    for (i, r) in rows.iter().enumerate() {
        for (j, f) in r.fields.iter().enumerate() {
            m.re[j * h + i] = parse_num(&f.text).unwrap_or(f64::NAN);
        }
    }
    Ok(m)
}

pub fn write_matrix(m: &Mat, delim: char) -> String {
    let mut out = String::new();
    for i in 0..m.rows {
        let row: Vec<String> = (0..m.cols).map(|j| num_text(m.at(i, j))).collect();
        out.push_str(&row.join(&delim.to_string()));
        out.push('\n');
    }
    out
}

// ---------- JSON ----------

#[derive(Debug, Clone)]
pub enum J {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

struct JP<'a> {
    c: Vec<char>,
    i: usize,
    fname: &'a str,
}

impl JP<'_> {
    fn pos(&self) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for &ch in &self.c[..self.i.min(self.c.len())] {
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
    fn err(&self, msg: &str) -> MError {
        let (l, c) = self.pos();
        at(self.fname, l, c, msg)
    }
    fn ws(&mut self) {
        while self.i < self.c.len() && self.c[self.i].is_whitespace() {
            self.i += 1;
        }
    }
    fn lit(&mut self, s: &str, v: J) -> Result<J, MError> {
        let w: Vec<char> = s.chars().collect();
        if self.c.len() >= self.i + w.len() && self.c[self.i..self.i + w.len()] == w[..] {
            self.i += w.len();
            Ok(v)
        } else {
            Err(self.err("invalid JSON value"))
        }
    }
    fn value(&mut self) -> Result<J, MError> {
        self.ws();
        let Some(&c) = self.c.get(self.i) else { return Err(self.err("unexpected end of JSON")) };
        match c {
            '{' => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.c.get(self.i) == Some(&'}') {
                    self.i += 1;
                    return Ok(J::Obj(out));
                }
                loop {
                    self.ws();
                    if self.c.get(self.i) != Some(&'"') {
                        return Err(self.err("expected a string key"));
                    }
                    let k = self.string()?;
                    self.ws();
                    if self.c.get(self.i) != Some(&':') {
                        return Err(self.err("expected ':'"));
                    }
                    self.i += 1;
                    let v = self.value()?;
                    out.push((k, v));
                    self.ws();
                    match self.c.get(self.i) {
                        Some(',') => self.i += 1,
                        Some('}') => {
                            self.i += 1;
                            return Ok(J::Obj(out));
                        }
                        _ => return Err(self.err("expected ',' or '}'")),
                    }
                }
            }
            '[' => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.c.get(self.i) == Some(&']') {
                    self.i += 1;
                    return Ok(J::Arr(out));
                }
                loop {
                    out.push(self.value()?);
                    self.ws();
                    match self.c.get(self.i) {
                        Some(',') => self.i += 1,
                        Some(']') => {
                            self.i += 1;
                            return Ok(J::Arr(out));
                        }
                        _ => return Err(self.err("expected ',' or ']'")),
                    }
                }
            }
            '"' => Ok(J::Str(self.string()?)),
            't' => self.lit("true", J::Bool(true)),
            'f' => self.lit("false", J::Bool(false)),
            'n' => self.lit("null", J::Null),
            _ => {
                let st = self.i;
                while self.i < self.c.len() && matches!(self.c[self.i], '0'..='9' | '-' | '+' | '.' | 'e' | 'E') {
                    self.i += 1;
                }
                let s: String = self.c[st..self.i].iter().collect();
                match s.parse::<f64>() {
                    Ok(x) if !s.is_empty() => Ok(J::Num(x)),
                    _ => {
                        self.i = st;
                        Err(self.err("invalid JSON value"))
                    }
                }
            }
        }
    }
    fn hex4(&mut self) -> Result<u32, MError> {
        let s: String = self.c.get(self.i..self.i + 4).map(|x| x.iter().collect()).unwrap_or_default();
        let v = u32::from_str_radix(&s, 16).map_err(|_| self.err("invalid \\u escape"))?;
        self.i += 4;
        Ok(v)
    }
    fn string(&mut self) -> Result<String, MError> {
        let start = self.i;
        self.i += 1;
        let mut out = String::new();
        loop {
            let Some(&c) = self.c.get(self.i) else {
                self.i = start;
                return Err(self.err("unterminated string"));
            };
            self.i += 1;
            match c {
                '"' => return Ok(out),
                '\\' => {
                    let Some(&e) = self.c.get(self.i) else { return Err(self.err("unterminated string")) };
                    self.i += 1;
                    match e {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\u{8}'),
                        'f' => out.push('\u{c}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'u' => {
                            let mut v = self.hex4()?;
                            if (0xD800..0xDC00).contains(&v) && self.c.get(self.i) == Some(&'\\') && self.c.get(self.i + 1) == Some(&'u') {
                                self.i += 2;
                                let lo = self.hex4()?;
                                v = 0x10000 + ((v - 0xD800) << 10) + (lo.wrapping_sub(0xDC00) & 0x3FF);
                            }
                            out.push(char::from_u32(v).unwrap_or('\u{FFFD}'));
                        }
                        _ => return Err(self.err("invalid escape")),
                    }
                }
                _ => out.push(c),
            }
        }
    }
}

pub fn parse_json(text: &str, fname: &str) -> Result<J, MError> {
    let mut p = JP { c: text.chars().collect(), i: 0, fname };
    let v = p.value()?;
    p.ws();
    if p.i < p.c.len() {
        return Err(p.err("trailing characters after JSON value"));
    }
    Ok(v)
}

pub fn json_to_table(j: &J, fname: &str) -> Result<Table, MError> {
    let J::Arr(items) = j else { return Err(MError::new(format!("{fname}: expected a JSON array of objects"))) };
    let mut names: Vec<String> = Vec::new();
    for it in items {
        let J::Obj(kv) = it else { return Err(MError::new(format!("{fname}: expected a JSON array of objects"))) };
        for (k, _) in kv {
            if !names.contains(k) {
                names.push(k.clone());
            }
        }
    }
    let mut t = Table { names: Vec::new(), columns: Vec::new(), kinds: Vec::new() };
    let mut valid: Vec<String> = names.iter().enumerate().map(|(k, n)| make_valid_name(n, k)).collect();
    table::dedupe(&mut valid);
    for (name, vname) in names.iter().zip(valid) {
        let vals: Vec<&J> = items
            .iter()
            .map(|it| match it {
                J::Obj(kv) => kv.iter().find(|(k, _)| k == name).map_or(&J::Null, |(_, v)| v),
                _ => &J::Null,
            })
            .collect();
        let present: Vec<&&J> = vals.iter().filter(|v| !matches!(v, J::Null)).collect();
        let (col, kind) = if present.iter().all(|v| matches!(v, J::Num(_))) {
            (Value::Mat(Mat::col(vals.iter().map(|v| if let J::Num(x) = v { *x } else { f64::NAN }).collect())), ColKind::Num)
        } else if present.iter().all(|v| matches!(v, J::Bool(_))) {
            let all = vals.iter().all(|v| matches!(v, J::Bool(_)));
            let re: Vec<f64> = vals.iter().map(|v| if let J::Bool(b) = v { f64::from(u8::from(*b)) } else { f64::NAN }).collect();
            if all { (Value::Mat(Mat::col(re).with_class(Class::Logical)), ColKind::Bool) } else { (Value::Mat(Mat::col(re)), ColKind::Num) }
        } else if present.iter().all(|v| matches!(v, J::Str(s) if parse_iso(s).is_some())) {
            (Value::Mat(Mat::col(vals.iter().map(|v| if let J::Str(s) = v { parse_iso(s).unwrap() } else { f64::NAN }).collect())), ColKind::Date)
        } else {
            let data = vals
                .iter()
                .map(|v| match v {
                    J::Null => None,
                    J::Str(s) => Some(s.clone()),
                    J::Num(x) => Some(num_text(*x)),
                    J::Bool(b) => Some(b.to_string()),
                    other => Some(json_text(other, false)),
                })
                .collect();
            (Value::Str(Rc::new(StrArr::col(data))), ColKind::Text)
        };
        t.names.push(vname);
        t.columns.push(col);
        t.kinds.push(kind);
    }
    Ok(t)
}

pub fn json_escape(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub fn json_text(j: &J, pretty: bool) -> String {
    let (kv, sep) = if pretty { (": ", ", ") } else { (":", ",") };
    match j {
        J::Null => "null".into(),
        J::Bool(b) => b.to_string(),
        J::Num(x) => if x.is_finite() { num_text(*x) } else { "null".into() },
        J::Str(s) => json_escape(s),
        J::Arr(a) => format!("[{}]", a.iter().map(|x| json_text(x, pretty)).collect::<Vec<_>>().join(sep)),
        J::Obj(o) => format!("{{{}}}", o.iter().map(|(k, v)| format!("{}{kv}{}", json_escape(k), json_text(v, pretty))).collect::<Vec<_>>().join(sep)),
    }
}

pub fn table_records(t: &Table) -> Vec<J> {
    (0..table::height(t))
        .map(|i| {
            J::Obj(
                (0..t.names.len())
                    .map(|k| {
                        let v = match (&t.columns[k], t.kinds[k]) {
                            (Value::Mat(m), ColKind::Bool) => J::Bool(m.re[i] != 0.0),
                            (Value::Mat(m), ColKind::Date) => if m.re[i].is_nan() { J::Null } else { J::Str(iso_date(m.re[i])) },
                            (Value::Mat(m), _) => if m.re[i].is_finite() { J::Num(m.re[i]) } else { J::Null },
                            (Value::Str(s), _) => s.data[i].clone().map_or(J::Null, J::Str),
                            _ => J::Null,
                        };
                        (t.names[k].clone(), v)
                    })
                    .collect(),
            )
        })
        .collect()
}

/// JSON file: array of records, one record per line.
pub fn write_json_table(t: &Table) -> String {
    let recs = table_records(t);
    if recs.is_empty() {
        return "[]\n".into();
    }
    let body: Vec<String> = recs.iter().map(|r| format!("  {}", json_text(r, true))).collect();
    format!("[\n{}\n]\n", body.join(",\n"))
}

pub fn value_to_json(v: &Value) -> J {
    match v {
        Value::Table(t) => J::Arr(table_records(t)),
        Value::Str(s) => {
            let items: Vec<J> = s.data.iter().map(|x| x.clone().map_or(J::Null, J::Str)).collect();
            if items.len() == 1 { items.into_iter().next().unwrap() } else { J::Arr(items) }
        }
        Value::Mat(m) if m.class == Class::Char => J::Str(m.to_string_lossy()),
        Value::Mat(m) => {
            let cell = |x: f64| if m.class == Class::Logical { J::Bool(x != 0.0) } else if x.is_finite() { J::Num(x) } else { J::Null };
            if m.is_scalar() {
                cell(m.re[0])
            } else if m.rows == 1 || m.cols == 1 {
                J::Arr(m.re.iter().map(|&x| cell(x)).collect())
            } else {
                J::Arr((0..m.rows).map(|i| J::Arr((0..m.cols).map(|j| cell(m.at(i, j))).collect())).collect())
            }
        }
        _ => J::Null,
    }
}
