//! mova-json: deterministic, layout-preserving JSON edits for Mova configs, registries and groupant manifests.
//!
//! The value model keeps object key order, number lexemes and the original escape text of every string it
//! did not change, so an unchanged document renders back byte for byte when its layout is one of the
//! supported layouts (pretty with N-space indent as written by Python `json.dumps(indent=N)` or
//! `serde_json::to_string_pretty`, or compact with or without spaces). Writes are refused for documents
//! whose bytes the renderer does not reproduce, unless the caller asks for a reformat.

use std::collections::BTreeMap;
use std::path::Path;

pub mod merge;
#[cfg(test)]
mod tests;

pub type Res<T> = Result<T, String>;

/// A JSON string: decoded text plus, when it differs from the canonical escape, the exact source escape.
#[derive(Clone, Debug)]
pub struct Str {
    pub s: String,
    raw: Option<String>,
}

impl Str {
    pub fn new(s: impl Into<String>) -> Str {
        Str { s: s.into(), raw: None }
    }
}

impl PartialEq for Str {
    fn eq(&self, o: &Str) -> bool {
        self.s == o.s
    }
}

/// Ordered JSON value. Numbers keep their lexeme; equality of numbers is lexeme equality.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Num(String),
    Str(Str),
    Arr(Vec<Value>),
    Obj(Vec<(Str, Value)>),
}

impl Value {
    pub fn string(s: impl Into<String>) -> Value {
        Value::Str(Str::new(s))
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(&s.s),
            _ => None,
        }
    }
    pub fn field(&self, k: &str) -> Option<&Value> {
        match self {
            Value::Obj(m) => m.iter().find(|(x, _)| x.s == k).map(|(_, v)| v),
            _ => None,
        }
    }
    pub fn field_mut(&mut self, k: &str) -> Option<&mut Value> {
        match self {
            Value::Obj(m) => m.iter_mut().find(|(x, _)| x.s == k).map(|(_, v)| v),
            _ => None,
        }
    }
    fn kind(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Num(_) => "number",
            Value::Str(_) => "string",
            Value::Arr(_) => "array",
            Value::Obj(_) => "object",
        }
    }
}

// ---------------------------------------------------------------- parsing

pub fn parse(text: &str) -> Res<Value> {
    let mut p = Parser { t: text, b: text.as_bytes(), i: 0, depth: 0 };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.b.len() {
        return Err(p.err("trailing data"));
    }
    Ok(v)
}

struct Parser<'a> {
    t: &'a str,
    b: &'a [u8],
    i: usize,
    depth: usize,
}

const MAX_DEPTH: usize = 512;

impl<'a> Parser<'a> {
    fn err(&self, m: &str) -> String {
        format!("JSON: {m} at byte {}", self.i)
    }
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn enter(&mut self) -> Res<()> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(self.err("nesting deeper than 512"));
        }
        self.i += 1;
        self.ws();
        Ok(())
    }
    fn value(&mut self) -> Res<Value> {
        match self.peek() {
            None => Err(self.err("unexpected end")),
            Some(b'{') => {
                self.enter()?;
                let mut m = Vec::new();
                if self.peek() == Some(b'}') {
                    self.i += 1;
                    self.depth -= 1;
                    return Ok(Value::Obj(m));
                }
                loop {
                    self.ws();
                    if self.peek() != Some(b'"') {
                        return Err(self.err("expected a key"));
                    }
                    let k = self.string()?;
                    self.ws();
                    if self.peek() != Some(b':') {
                        return Err(self.err("expected ':'"));
                    }
                    self.i += 1;
                    self.ws();
                    let v = self.value()?;
                    m.push((k, v));
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            break;
                        }
                        _ => return Err(self.err("expected ',' or '}'")),
                    }
                }
                self.depth -= 1;
                Ok(Value::Obj(m))
            }
            Some(b'[') => {
                self.enter()?;
                let mut a = Vec::new();
                if self.peek() == Some(b']') {
                    self.i += 1;
                    self.depth -= 1;
                    return Ok(Value::Arr(a));
                }
                loop {
                    self.ws();
                    a.push(self.value()?);
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            break;
                        }
                        _ => return Err(self.err("expected ',' or ']'")),
                    }
                }
                self.depth -= 1;
                Ok(Value::Arr(a))
            }
            Some(b'"') => Ok(Value::Str(self.string()?)),
            Some(b't') => self.lit("true", Value::Bool(true)),
            Some(b'f') => self.lit("false", Value::Bool(false)),
            Some(b'n') => self.lit("null", Value::Null),
            Some(c) if c == b'-' || c.is_ascii_digit() => self.number(),
            Some(_) => Err(self.err("unexpected character")),
        }
    }
    fn lit(&mut self, w: &str, v: Value) -> Res<Value> {
        if self.b[self.i..].starts_with(w.as_bytes()) {
            self.i += w.len();
            Ok(v)
        } else {
            Err(self.err("invalid literal"))
        }
    }
    fn digits(&mut self) -> usize {
        let s = self.i;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.i += 1;
        }
        self.i - s
    }
    fn number(&mut self) -> Res<Value> {
        let s = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        match self.peek() {
            Some(b'0') => self.i += 1,
            Some(c) if c.is_ascii_digit() => {
                self.digits();
            }
            _ => return Err(self.err("invalid number")),
        }
        if self.peek() == Some(b'.') {
            self.i += 1;
            if self.digits() == 0 {
                return Err(self.err("invalid fraction"));
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.i += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.i += 1;
            }
            if self.digits() == 0 {
                return Err(self.err("invalid exponent"));
            }
        }
        Ok(Value::Num(self.t[s..self.i].to_string()))
    }
    fn hex4(&mut self) -> Res<u32> {
        let h = self.t.get(self.i..self.i + 4).ok_or_else(|| self.err("short \\u escape"))?;
        let v = u32::from_str_radix(h, 16).map_err(|_| self.err("bad \\u escape"))?;
        self.i += 4;
        Ok(v)
    }
    fn string(&mut self) -> Res<Str> {
        self.i += 1;
        let start = self.i;
        let mut s = String::new();
        let mut run = self.i;
        loop {
            let c = self.peek().ok_or_else(|| self.err("unterminated string"))?;
            match c {
                b'"' => {
                    s.push_str(&self.t[run..self.i]);
                    let raw = &self.t[start..self.i];
                    self.i += 1;
                    let mut canon = String::new();
                    escape_into(&s, &mut canon);
                    let raw = if canon == raw { None } else { Some(raw.to_string()) };
                    return Ok(Str { s, raw });
                }
                b'\\' => {
                    s.push_str(&self.t[run..self.i]);
                    self.i += 1;
                    let e = self.peek().ok_or_else(|| self.err("unterminated escape"))?;
                    self.i += 1;
                    match e {
                        b'"' => s.push('"'),
                        b'\\' => s.push('\\'),
                        b'/' => s.push('/'),
                        b'b' => s.push('\u{8}'),
                        b'f' => s.push('\u{c}'),
                        b'n' => s.push('\n'),
                        b'r' => s.push('\r'),
                        b't' => s.push('\t'),
                        b'u' => {
                            let mut cp = self.hex4()?;
                            if (0xD800..0xDC00).contains(&cp) {
                                if !self.b[self.i..].starts_with(b"\\u") {
                                    return Err(self.err("lone high surrogate"));
                                }
                                self.i += 2;
                                let lo = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&lo) {
                                    return Err(self.err("bad low surrogate"));
                                }
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                            }
                            s.push(char::from_u32(cp).ok_or_else(|| self.err("lone surrogate"))?);
                        }
                        _ => return Err(self.err("bad escape")),
                    }
                    run = self.i;
                }
                c if c < 0x20 => return Err(self.err("control character in string")),
                _ => self.i += 1,
            }
        }
    }
}

// ---------------------------------------------------------------- rendering

/// Container layout. `Pretty(n)`: one item per line, n-space indent, `": "`; `Compact`: `,` and `:`;
/// `Spaced`: `, ` and `: ` (Python's default separators).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    Pretty(usize),
    Compact,
    Spaced,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Style {
    pub layout: Layout,
    pub newline: bool,
}

/// Canonical escape: `"`, `\`, \b \f \n \r \t, other C0 controls as lowercase \u00xx, everything else raw UTF-8.
pub fn escape_into(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
}

fn write_str(s: &Str, out: &mut String) {
    out.push('"');
    match &s.raw {
        Some(r) => out.push_str(r),
        None => escape_into(&s.s, out),
    }
    out.push('"');
}

fn indent(out: &mut String, n: usize) {
    out.extend(std::iter::repeat_n(' ', n));
}

fn write_value(v: &Value, lay: Layout, depth: usize, out: &mut String) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Num(n) => out.push_str(n),
        Value::Str(s) => write_str(s, out),
        Value::Arr(a) if a.is_empty() => out.push_str("[]"),
        Value::Obj(m) if m.is_empty() => out.push_str("{}"),
        Value::Arr(a) => {
            out.push('[');
            for (i, x) in a.iter().enumerate() {
                sep(out, lay, depth + 1, i > 0);
                write_value(x, lay, depth + 1, out);
            }
            close(out, lay, depth);
            out.push(']');
        }
        Value::Obj(m) => {
            out.push('{');
            for (i, (k, x)) in m.iter().enumerate() {
                sep(out, lay, depth + 1, i > 0);
                write_str(k, out);
                out.push_str(if lay == Layout::Compact { ":" } else { ": " });
                write_value(x, lay, depth + 1, out);
            }
            close(out, lay, depth);
            out.push('}');
        }
    }
}

fn sep(out: &mut String, lay: Layout, depth: usize, comma: bool) {
    match lay {
        Layout::Pretty(n) => {
            if comma {
                out.push(',');
            }
            out.push('\n');
            indent(out, n * depth);
        }
        Layout::Compact if comma => out.push(','),
        Layout::Spaced if comma => out.push_str(", "),
        _ => {}
    }
}

fn close(out: &mut String, lay: Layout, depth: usize) {
    if let Layout::Pretty(n) = lay {
        out.push('\n');
        indent(out, n * depth);
    }
}

pub fn render(v: &Value, st: Style) -> String {
    let mut out = String::new();
    write_value(v, st.layout, 0, &mut out);
    if st.newline {
        out.push('\n');
    }
    out
}

pub fn compact(v: &Value) -> String {
    render(v, Style { layout: Layout::Compact, newline: false })
}

// ---------------------------------------------------------------- documents

/// A parsed file: value, detected style and whether rendering reproduces the input bytes exactly.
#[derive(Clone, Debug)]
pub struct Doc {
    pub value: Value,
    pub style: Style,
    pub canonical: bool,
}

pub fn load_str(text: &str) -> Res<Doc> {
    let value = parse(text)?;
    let newline = text.ends_with('\n');
    let b = text.as_bytes();
    let mut cands = Vec::new();
    if b.len() > 2 && matches!(b[0], b'{' | b'[') && b[1] == b'\n' {
        let n = b[2..].iter().take_while(|c| **c == b' ').count();
        if n > 0 {
            cands.push(Layout::Pretty(n));
        }
    }
    for l in [Layout::Pretty(2), Layout::Pretty(1), Layout::Pretty(4), Layout::Compact, Layout::Spaced] {
        if !cands.contains(&l) {
            cands.push(l);
        }
    }
    for &layout in &cands {
        let style = Style { layout, newline };
        if render(&value, style) == text {
            return Ok(Doc { value, style, canonical: true });
        }
    }
    Ok(Doc { value, style: Style { layout: cands[0], newline }, canonical: false })
}

pub fn load_file(path: &Path) -> Res<Doc> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    load_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Load a document for editing: refuse one whose bytes would change on an unedited write.
pub fn load_for_edit(path: &Path, reformat: bool) -> Res<Doc> {
    let d = load_file(path)?;
    if !d.canonical && !reformat {
        return Err(format!(
            "{}: layout not reproduced byte for byte (detected {:?}); rerun with --reformat to rewrite it in that layout",
            path.display(),
            d.style.layout
        ));
    }
    Ok(d)
}

/// Write the document if its rendering differs from the file; atomic via a sibling temp file and rename,
/// keeping the original permissions. Returns whether bytes changed.
pub fn save(path: &Path, doc: &Doc) -> Res<bool> {
    let text = render(&doc.value, doc.style);
    let old = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if old == text.as_bytes() {
        return Ok(false);
    }
    write_atomic(path, text.as_bytes())?;
    Ok(true)
}

pub fn write_atomic(path: &Path, bytes: &[u8]) -> Res<()> {
    let name = path.file_name().ok_or_else(|| format!("{}: not a file path", path.display()))?;
    let tmp = path.with_file_name(format!(".{}.mova-json.tmp", name.to_string_lossy()));
    let e = |x: std::io::Error| format!("{}: {x}", tmp.display());
    std::fs::write(&tmp, bytes).map_err(e)?;
    if let Ok(m) = std::fs::metadata(path) {
        std::fs::set_permissions(&tmp, m.permissions()).map_err(e)?;
    }
    std::fs::rename(&tmp, path).map_err(|x| format!("{}: {x}", path.display()))
}

// ---------------------------------------------------------------- paths

/// One path step: object key, array index (negative counts from the end) or `[key=value]` selector
/// that must match exactly one object element of an array.
#[derive(Clone, Debug, PartialEq)]
pub enum Seg {
    Key(String),
    Index(i64),
    Select(String, String),
}

/// `a.b[2].c[name=x].d`; `\` escapes the next character; "" or "." is the root.
pub fn parse_path(p: &str) -> Res<Vec<Seg>> {
    let mut segs = Vec::new();
    if p.is_empty() || p == "." {
        return Ok(segs);
    }
    let cs: Vec<char> = p.chars().collect();
    let bad = |m: &str| format!("path {p:?}: {m}");
    let mut i = 0;
    loop {
        let mut key = String::new();
        let mut has_key = false;
        while i < cs.len() && cs[i] != '.' && cs[i] != '[' {
            if cs[i] == '\\' {
                i += 1;
                if i == cs.len() {
                    return Err(bad("dangling escape"));
                }
            }
            key.push(cs[i]);
            has_key = true;
            i += 1;
        }
        if has_key {
            segs.push(Seg::Key(key));
        }
        let mut sels = 0;
        while i < cs.len() && cs[i] == '[' {
            i += 1;
            let (mut body, mut eq) = (String::new(), None);
            while i < cs.len() && cs[i] != ']' {
                if cs[i] == '\\' {
                    i += 1;
                    if i == cs.len() {
                        return Err(bad("dangling escape"));
                    }
                } else if cs[i] == '=' && eq.is_none() {
                    eq = Some(body.len());
                    i += 1;
                    continue;
                }
                body.push(cs[i]);
                i += 1;
            }
            if i == cs.len() {
                return Err(bad("unclosed '['"));
            }
            i += 1;
            segs.push(match eq {
                Some(e) => Seg::Select(body[..e].to_string(), body[e..].to_string()),
                None => Seg::Index(body.trim().parse().map_err(|_| bad("index is not an integer"))?),
            });
            sels += 1;
        }
        if !has_key && sels == 0 {
            return Err(bad("empty segment"));
        }
        if i == cs.len() {
            return Ok(segs);
        }
        if cs[i] != '.' || i + 1 == cs.len() {
            return Err(bad("expected '.' between segments"));
        }
        i += 1;
    }
}

pub fn seg_text(s: &Seg) -> String {
    match s {
        Seg::Key(k) => format!(".{k}"),
        Seg::Index(n) => format!("[{n}]"),
        Seg::Select(k, v) => format!("[{k}={v}]"),
    }
}

/// Text a selector or filter compares with: a string's value, otherwise the scalar lexeme.
pub fn scalar_text(v: &Value) -> Option<String> {
    match v {
        Value::Str(s) => Some(s.s.clone()),
        Value::Num(n) => Some(n.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => Some("null".into()),
        _ => None,
    }
}

pub fn matches(v: &Value, want: &str) -> bool {
    scalar_text(v).as_deref() == Some(want)
}

fn position(v: &Value, seg: &Seg, at: &str) -> Res<usize> {
    match (seg, v) {
        (Seg::Key(k), Value::Obj(m)) => {
            m.iter().position(|(x, _)| x.s == *k).ok_or_else(|| format!("{at}: no key {k:?}"))
        }
        (Seg::Index(n), Value::Arr(a)) => {
            let len = a.len() as i64;
            let j = if *n < 0 { len + n } else { *n };
            if j < 0 || j >= len {
                Err(format!("{at}: index {n} out of range (length {len})"))
            } else {
                Ok(j as usize)
            }
        }
        (Seg::Select(k, want), Value::Arr(a)) => {
            let hits: Vec<usize> = (0..a.len()).filter(|&i| a[i].field(k).is_some_and(|f| matches(f, want))).collect();
            if hits.len() == 1 {
                Ok(hits[0])
            } else {
                Err(format!("{at}: selector [{k}={want}] matched {} elements, need exactly 1", hits.len()))
            }
        }
        _ => Err(format!("{at}: step {} does not apply to a {}", seg_text(seg), v.kind())),
    }
}

fn child(v: &Value, i: usize) -> &Value {
    match v {
        Value::Arr(a) => &a[i],
        Value::Obj(m) => &m[i].1,
        _ => unreachable!(),
    }
}

fn child_mut(v: &mut Value, i: usize) -> &mut Value {
    match v {
        Value::Arr(a) => &mut a[i],
        Value::Obj(m) => &mut m[i].1,
        _ => unreachable!(),
    }
}

fn prefix(path: &[Seg]) -> String {
    let s: String = path.iter().map(seg_text).collect();
    if s.is_empty() {
        "$".into()
    } else {
        format!("${s}")
    }
}

pub fn get<'a>(root: &'a Value, path: &[Seg]) -> Res<&'a Value> {
    let mut cur = root;
    for k in 0..path.len() {
        let i = position(cur, &path[k], &prefix(&path[..k]))?;
        cur = child(cur, i);
    }
    Ok(cur)
}

pub fn get_mut<'a>(root: &'a mut Value, path: &[Seg]) -> Res<&'a mut Value> {
    let mut cur = root;
    for k in 0..path.len() {
        let i = position(cur, &path[k], &prefix(&path[..k]))?;
        cur = child_mut(cur, i);
    }
    Ok(cur)
}

/// Insert a key: into its sorted place when the object's keys are already sorted, else at the end.
pub fn insert_key(m: &mut Vec<(Str, Value)>, k: Str, v: Value) {
    let sorted = m.windows(2).all(|w| w[0].0.s <= w[1].0.s);
    let at = if sorted { m.iter().position(|(x, _)| x.s > k.s).unwrap_or(m.len()) } else { m.len() };
    m.insert(at, (k, v));
}

/// Replace the value at `path`; a missing final object key is inserted (see `insert_key`).
pub fn set(root: &mut Value, path: &[Seg], val: Value) -> Res<()> {
    let Some((last, parent_path)) = path.split_last() else {
        *root = val;
        return Ok(());
    };
    let parent = get_mut(root, parent_path)?;
    if let (Seg::Key(k), Value::Obj(m)) = (last, &mut *parent) {
        match m.iter_mut().find(|(x, _)| x.s == *k) {
            Some(slot) => slot.1 = val,
            None => insert_key(m, Str::new(k.clone()), val),
        }
        return Ok(());
    }
    let i = position(parent, last, &prefix(parent_path))?;
    *child_mut(parent, i) = val;
    Ok(())
}

pub fn del(root: &mut Value, path: &[Seg]) -> Res<Value> {
    let (last, parent_path) = path.split_last().ok_or("cannot delete the root")?;
    let parent = get_mut(root, parent_path)?;
    let i = position(parent, last, &prefix(parent_path))?;
    Ok(match parent {
        Value::Arr(a) => a.remove(i),
        Value::Obj(m) => m.remove(i).1,
        _ => unreachable!(),
    })
}

fn array_at<'a>(root: &'a mut Value, path: &[Seg]) -> Res<&'a mut Vec<Value>> {
    match get_mut(root, path)? {
        Value::Arr(a) => Ok(a),
        v => Err(format!("{}: a {} is not an array", prefix(path), v.kind())),
    }
}

pub fn push(root: &mut Value, path: &[Seg], vals: Vec<Value>) -> Res<()> {
    array_at(root, path)?.extend(vals);
    Ok(())
}

/// Insert values before the single element equal to `anchor`.
pub fn insert_before(root: &mut Value, path: &[Seg], anchor: &Value, vals: Vec<Value>) -> Res<usize> {
    let at = prefix(path);
    let a = array_at(root, path)?;
    let hits: Vec<usize> = (0..a.len()).filter(|&i| a[i] == *anchor).collect();
    if hits.len() != 1 {
        return Err(format!("{at}: anchor {} occurs {} times, need exactly 1", compact(anchor), hits.len()));
    }
    let i = hits[0];
    a.splice(i..i, vals);
    Ok(i)
}

/// Remove every element equal to `val`; zero matches is an error.
pub fn remove_value(root: &mut Value, path: &[Seg], val: &Value) -> Res<usize> {
    let at = prefix(path);
    let a = array_at(root, path)?;
    let n = a.len();
    a.retain(|x| x != val);
    let removed = n - a.len();
    if removed == 0 {
        return Err(format!("{at}: no element equal to {}", compact(val)));
    }
    Ok(removed)
}

/// Replace `old` by `new` inside every string value (not keys). Returns (strings changed, occurrences).
pub fn replace_str(v: &mut Value, old: &str, new: &str) -> (usize, usize) {
    match v {
        Value::Str(s) if s.s.contains(old) => {
            let n = s.s.matches(old).count();
            *s = Str::new(s.s.replace(old, new));
            (1, n)
        }
        Value::Arr(a) => a.iter_mut().map(|x| replace_str(x, old, new)).fold((0, 0), |p, q| (p.0 + q.0, p.1 + q.1)),
        Value::Obj(m) => m.iter_mut().map(|(_, x)| replace_str(x, old, new)).fold((0, 0), |p, q| (p.0 + q.0, p.1 + q.1)),
        _ => (0, 0),
    }
}

// ---------------------------------------------------------------- JSONL tallies

pub const MISSING: &str = "<missing>";

fn label(v: &Value) -> String {
    scalar_text(v).unwrap_or_else(|| compact(v))
}

/// Tally the value at `path` over JSONL lines (blank lines skipped). Returns (counts, invalid line numbers).
pub fn jsonl_count(text: &str, path: &[Seg]) -> (BTreeMap<String, usize>, Vec<usize>) {
    let mut counts = BTreeMap::new();
    let mut invalid = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match parse(line) {
            Ok(v) => {
                let l = get(&v, path).map(label).unwrap_or_else(|_| MISSING.into());
                *counts.entry(l).or_insert(0) += 1;
            }
            Err(_) => invalid.push(n + 1),
        }
    }
    (counts, invalid)
}

/// Count rows sorted by count descending, then label.
pub fn sorted_counts(c: &BTreeMap<String, usize>) -> Vec<(usize, &str)> {
    let mut v: Vec<(usize, &str)> = c.iter().map(|(k, n)| (*n, k.as_str())).collect();
    v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
    v
}

/// Lines whose value at `path` compares equal (selector rule) to `want`.
pub fn jsonl_filter<'a>(text: &'a str, path: &[Seg], want: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|l| parse(l).ok().is_some_and(|v| get(&v, path).is_ok_and(|x| matches(x, want))))
        .collect()
}

/// Split `PATH=VALUE` at the first '=' outside brackets.
pub fn split_assignment(s: &str) -> Res<(&str, &str)> {
    let mut depth = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            '=' if depth == 0 => return Ok((&s[..i], &s[i + 1..])),
            _ => {}
        }
    }
    Err(format!("{s:?}: expected PATH=VALUE"))
}
