//! Undated main text for groupant `model.md` files. The main blocks state what is now;
//! dated material (epoch headings, dated paragraphs and list items, provenance such as
//! hashes and `$MOVA_DATA` run roots) moves verbatim into one final `## History`
//! section, grouped as `### <date> · <origin section>` in chronological order.
//!
//! Mechanics only, deterministic and idempotent: no line is deleted or reworded. A moved
//! heading block keeps its text; only its ATX level is raised to sit under a History
//! group. The first dated heading of a section that has no undated prose before it is
//! the section's lead (its current body): it stays in place under its undated name and a
//! History note keeps the original heading line. Dated H1 titles are renamed the same
//! way. H2 sections that some JSON record selects by name never move. Fenced code is
//! never inspected for dates. Fresh hash-bound JSON references to rewritten files are
//! rebound (model.md sections, whole files, then JSON chains) and reported.
use crate::consolidate::{git_files, join, json_literals, parent};
use crate::sections::{self, demote, heading_level, split_reference};
use crate::sha;
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const HISTORY: &str = "History";
/// ATX level of the top heading of a moved block inside a History group.
const BASE_LEVEL: usize = 4;
const UNDATED: &str = "Undated";
const NOTE_PREFIX: &str = "Heading ";
const GROUP_SEP: &str = " · ";

/// Epoch dates (`YYYY-MM-DD`, years 2025 to 2029, not adjacent to digits) in a line.
/// Not epoch dates: a toolchain build date `(<hex commit> DATE)` and a data snapshot
/// date followed by ` run`, ` dump`, ` English dump` or ` snapshot`.
pub fn epoch_dates(s: &str) -> Vec<String> {
    date_starts(s)
        .into_iter()
        .map(|i| s[i..i + 10].to_string())
        .collect()
}
/// Byte offsets of the epoch dates of `epoch_dates`.
fn date_starts(s: &str) -> Vec<usize> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 10 <= b.len() {
        if !is_date_at(b, i) || (i > 0 && b[i - 1].is_ascii_digit()) {
            i += 1;
            continue;
        }
        if b.get(i + 10).is_some_and(u8::is_ascii_digit) || excluded(s, i) {
            i += 10;
            continue;
        }
        out.push(i);
        i += 10;
    }
    out
}
fn is_date_at(b: &[u8], i: usize) -> bool {
    let d = |k: usize| b[i + k].is_ascii_digit();
    if !(d(0) && d(1) && d(2) && d(3) && b[i + 4] == b'-' && d(5) && d(6) && b[i + 7] == b'-')
        || !(d(8) && d(9))
    {
        return false;
    }
    let n = |a: usize, z: usize| -> u32 {
        b[i + a..i + z]
            .iter()
            .fold(0, |v, c| v * 10 + u32::from(c - b'0'))
    };
    (2025..=2029).contains(&n(0, 4)) && (1..=12).contains(&n(5, 7)) && (1..=31).contains(&n(8, 10))
}
fn excluded(s: &str, i: usize) -> bool {
    let after = &s[i + 10..];
    if [" run", " dump", " English dump", " snapshot"]
        .iter()
        .any(|w| after.starts_with(w))
    {
        return true;
    }
    // `(b940084d7 2026-09-28)`: a compiler or tool build date.
    if after.starts_with(')') {
        let before = &s[..i];
        if let Some(rest) = before.strip_suffix(' ')
            && let Some(open) = rest.rfind('(')
        {
            let hex = &rest[open + 1..];
            return (7..=40).contains(&hex.len()) && hex.bytes().all(|c| c.is_ascii_hexdigit());
        }
    }
    false
}

/// Provenance that is not main text: concrete `$MOVA_DATA/…` paths (not the bare
/// variable name, not a template path holding a `<placeholder>`), full hex digests (40 or
/// more lowercase hex digits, not glued to a lowercase word; `SHA<digest>` counts) and
/// abbreviated digests (seven or more hex digits then `…`).
pub fn provenance(s: &str) -> bool {
    !provenance_spans(s).is_empty()
}
const DATA_VARS: [&str; 2] = ["$MOVA_DATA/", "${MOVA_DATA}/"];
/// Byte spans of the provenance tokens of `provenance`, sorted and disjoint.
fn provenance_spans(s: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for var in DATA_VARS {
        let mut from = 0;
        while let Some(p) = s[from..].find(var).map(|p| p + from) {
            let len = s[p..]
                .find(|c: char| {
                    c.is_whitespace() || matches!(c, '`' | '"' | '\'' | ',' | ';' | ')')
                })
                .unwrap_or(s.len() - p);
            let mut end = p + len;
            while s[..end].ends_with(['.', ':']) && end > p + var.len() {
                end -= 1;
            }
            if !s[p..end].contains('<') {
                out.push((p, end));
            }
            from = p + var.len();
        }
    }
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if !matches!(b[i], b'0'..=b'9' | b'a'..=b'f')
            || (i > 0 && (b[i - 1].is_ascii_lowercase() || b[i - 1].is_ascii_digit()))
        {
            i += 1;
            continue;
        }
        let run = b[i..]
            .iter()
            .take_while(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
            .count();
        let next = &s[i + run..];
        let bounded = !next
            .bytes()
            .next()
            .is_some_and(|c| c.is_ascii_alphanumeric());
        let digits = b[i..i + run].iter().any(u8::is_ascii_digit);
        if digits && run >= 40 && bounded {
            out.push((i, i + run));
        } else if digits && run >= 7 && next.starts_with('…') {
            out.push((i, i + run + '…'.len_utf8()));
        }
        i += run.max(1);
    }
    out.sort_unstable();
    out.dedup_by(|b, a| b.0 < a.1);
    out
}

/// Mechanical undating of one main-text line for a selected section (see `strip_line`).
#[derive(Debug, PartialEq)]
pub enum Strip {
    /// The line stays, possibly rewritten.
    Keep(String),
    /// The line carries only dated or provenance material; it lives in History only.
    Drop,
    /// The date or provenance is woven into an identifier; no mechanical rewrite.
    Fail,
}
/// What stands in main text for a provenance token or a dated code span.
pub const IN_HISTORY: &str = "(see History)";
/// Record labels whose line is provenance as a whole (`Date: …`, `Run root: …`).
const RECORD_LABELS: [&str; 12] = [
    "date",
    "dates",
    "run",
    "runs",
    "run root",
    "run roots",
    "receipt",
    "receipts",
    "commit",
    "revision",
    "subject/revision",
    "source sha-256",
];
fn list_marker_len(line: &str) -> usize {
    let indent = line.bytes().take_while(|b| *b == b' ').count();
    let r = &line[indent..];
    for m in ["- ", "* ", "+ "] {
        if r.starts_with(m) {
            return indent + 2;
        }
    }
    let digits = r.bytes().take_while(u8::is_ascii_digit).count();
    if digits > 0 && (r[digits..].starts_with(". ") || r[digits..].starts_with(") ")) {
        return indent + digits + 2;
    }
    indent
}
/// Rewrites one line so it carries no epoch date and no provenance, deterministically:
/// a dated heading takes its `undated_name` (Drop when nothing remains); a code span
/// holding a date or provenance and a bare provenance token become `(see History)`;
/// a date is removed with its frame: `(D)`, `; D)`, `(D, `, a leading `Epoch D …:` or
/// `D, `, a preposition (`as of`, `on`, `since`, …) or a separator (`, D`, ` — D`, ` D`).
/// A record line labelled `Date:`, `Run root:`, … and a History note Drop; a date glued
/// to an identifier Fails. Fenced lines and lines without dates or provenance Keep as is.
pub fn strip_line(line: &str, fenced: bool) -> Strip {
    if fenced || (date_starts(line).is_empty() && provenance_spans(line).is_empty()) {
        return Strip::Keep(line.to_string());
    }
    if heading_level(line).is_some() {
        if provenance(heading_text(line)) {
            return Strip::Fail;
        }
        let name = undated_name(heading_text(line), "");
        if name.is_empty() {
            return Strip::Drop;
        }
        return Strip::Keep(renamed_line(line, &name));
    }
    let m = list_marker_len(line);
    let (marker, body) = line.split_at(m);
    if body.starts_with(NOTE_PREFIX) && body.contains(" stays in ") {
        return Strip::Drop;
    }
    if let Some((label, _)) = body.split_once(':') {
        let l = label.trim_matches(['*', '_', ' ']).to_lowercase();
        if RECORD_LABELS.contains(&l.as_str()) {
            return Strip::Drop;
        }
    }
    let mut s = replace_code_spans(body);
    let spans = provenance_spans(&s);
    for (a, z) in spans.into_iter().rev() {
        s.replace_range(a..z, IN_HISTORY);
    }
    let mut guard = 0;
    while let Some(&p) = date_starts(&s).first() {
        guard += 1;
        if guard > 64 || !remove_date(&mut s, p) {
            return Strip::Fail;
        }
    }
    let s = tidy(&s);
    if !date_starts(&s).is_empty() || provenance(&s) {
        return Strip::Fail;
    }
    let substance = s.replace(IN_HISTORY, "");
    if !substance.chars().any(char::is_alphanumeric) {
        return Strip::Drop;
    }
    Strip::Keep(format!("{marker}{s}"))
}
/// Replaces every inline code span that holds a date or provenance by `IN_HISTORY`.
fn replace_code_spans(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(open) = rest.find('`') {
        let ticks = rest[open..].bytes().take_while(|b| *b == b'`').count();
        let fence = &rest[open..open + ticks];
        let after = &rest[open + ticks..];
        let Some(close) = after.find(fence) else {
            break;
        };
        let inner = &after[..close];
        out.push_str(&rest[..open]);
        if date_starts(inner).is_empty() && provenance_spans(inner).is_empty() {
            out.push_str(&rest[open..open + ticks + close + ticks]);
        } else {
            out.push_str(IN_HISTORY);
        }
        rest = &after[close + ticks..];
    }
    out.push_str(rest);
    out
}
const DATE_WORDS: [&str; 12] = [
    "as of ", "on ", "since ", "from ", "dated ", "until ", "through ", "at ", "in ", "by ",
    "after ", "before ",
];
/// Removes the date at byte `p` with its frame; false when the date is glued to an
/// identifier (a letter, digit, `-`, `_`, `/` or `.` beside it).
fn remove_date(s: &mut String, p: usize) -> bool {
    let q = p + 10;
    let prev = s[..p].chars().next_back();
    let next = s[q..].chars().next();
    let glued =
        |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '/'));
    if glued(prev) || glued(next) || prev == Some('.') {
        // A run key or path holding the date: the whole token is provenance.
        let a = s[..p]
            .rfind(|c: char| c.is_whitespace() || c == '(')
            .map_or(0, |i| i + 1);
        let mut z = s[q..]
            .find(|c: char| c.is_whitespace())
            .map_or(s.len(), |i| q + i);
        while z > q && s[..z].ends_with(['.', ',', ';', ':', ')']) {
            z -= 1;
        }
        if s[a..z].contains(IN_HISTORY) {
            return false;
        }
        s.replace_range(a..z, IN_HISTORY);
        return true;
    }
    let before = &s[..p];
    let after = &s[q..];
    // Parenthetical frames.
    if before.ends_with('(') && after.starts_with(')') {
        let a = if before.ends_with(" (") { p - 2 } else { p - 1 };
        s.replace_range(a..q + 1, "");
        return true;
    }
    if before.ends_with('(') && (after.starts_with(", ") || after.starts_with("; ")) {
        s.replace_range(p..q + 2, "");
        return true;
    }
    for sep in ["; ", ", "] {
        if before.ends_with(sep) && after.starts_with(')') {
            s.replace_range(p - sep.len()..q, "");
            return true;
        }
    }
    // A sentence-leading `Epoch D (…): ` / `D, ` / `D: ` / `**Epoch D.**`.
    let boundary = [". ", "! ", "? "]
        .iter()
        .filter_map(|b| before.rfind(b).map(|i| i + b.len()))
        .max()
        .unwrap_or(0);
    let (kept, before) = before.split_at(boundary);
    let lead = before.trim_start_matches(['*', '_']);
    if lead.trim().is_empty() || ["Epoch ", "epoch ", "Epoch: "].contains(&lead) {
        let mut r = after;
        if r.starts_with(" (")
            && let Some(c) = r.find(')')
        {
            r = &r[c + 1..];
        }
        let r = r
            .trim_start_matches(['*', '_'])
            .trim_start_matches([':', ',', ';', '.', ' ', '—', '–'])
            .trim_start_matches(['*', '_'])
            .trim_start();
        let r = if before.starts_with("**") {
            r.trim_start_matches("**").trim_start()
        } else {
            r
        };
        let mut c = r.chars();
        let cap: String = match c.next() {
            Some(f) => f.to_uppercase().chain(c).collect(),
            None => String::new(),
        };
        *s = format!("{kept}{cap}");
        return true;
    }
    let lower = before.to_lowercase();
    for w in DATE_WORDS {
        if lower.ends_with(&format!(" {w}")) || lower == w {
            let a = p - w.len() - usize::from(lower != w);
            s.replace_range(a..q, "");
            capitalize_after(s, a);
            return true;
        }
    }
    for sep in [", ", " — ", " – ", " - ", ": "] {
        if before.ends_with(sep) {
            s.replace_range(p - sep.len()..q, "");
            return true;
        }
    }
    if after.starts_with(':') && before.ends_with(' ') {
        s.replace_range(p - 1..q, "");
        return true;
    }
    for sep in [", ", ": ", "; "] {
        if after.starts_with(sep) {
            s.replace_range(p..q + sep.len(), "");
            return true;
        }
    }
    if before.ends_with(' ') {
        s.replace_range(p - 1..q, "");
    } else {
        s.replace_range(p..q, "");
    }
    true
}
/// Uppercases the first letter at or after byte `a` when a sentence starts there.
fn capitalize_after(s: &mut String, a: usize) {
    let Some(off) = s[a..].find(|c: char| c != ' ' && c != ',') else {
        return;
    };
    let at = a + off;
    let head = s[..a].trim_end();
    let starts = head.is_empty() || head.ends_with(['.', '!', '?']);
    let c = s[at..].chars().next().expect("char");
    if starts && c.is_lowercase() {
        let up: String = c.to_uppercase().collect();
        s.replace_range(
            a..at + c.len_utf8(),
            &format!("{}{up}", if head.is_empty() { "" } else { " " }),
        );
    }
}
/// Spacing and punctuation left by removals.
fn tidy(s: &str) -> String {
    let mut t = s.to_string();
    loop {
        let before = t.clone();
        for (a, z) in [
            (" ()", ""),
            ("()", ""),
            ("(; ", "("),
            ("(, ", "("),
            ("( ", "("),
            (" )", ")"),
            (" ,", ","),
            (" .", "."),
            (" ;", ";"),
            (",,", ","),
            (",.", "."),
            (";.", "."),
            ("  ", " "),
        ] {
            t = t.replace(a, z);
        }
        if t == before {
            break;
        }
    }
    let t = t.trim_end().trim_start_matches([',', ';', ' ']);
    t.to_string()
}

/// Undated heading name: drops the date with its separators, then a trailing or
/// leading `epoch`. An empty remainder takes `fallback`; a lone parenthetical is unwrapped.
pub fn undated_name(name: &str, fallback: &str) -> String {
    let mut s = name.to_string();
    for d in epoch_dates(name) {
        for pat in [
            format!(" ({d})"),
            format!("({d})"),
            format!(" — {d}"),
            format!(" – {d}"),
            format!(" - {d}"),
            format!(", {d}"),
            format!(": {d}"),
            format!(" {d}"),
            format!("{d} "),
            d.clone(),
        ] {
            if let Some(p) = s.find(&pat) {
                s.replace_range(p..p + pat.len(), "");
                break;
            }
        }
    }
    let mut s = s
        .trim_matches(|c: char| c.is_whitespace() || matches!(c, '—' | '–' | '-' | ',' | ':'))
        .to_string();
    for (pre, suf) in [
        ("Epoch ", ""),
        ("epoch ", ""),
        ("", " epoch"),
        ("", " Epoch"),
    ] {
        if !pre.is_empty() && s.starts_with(pre) {
            s = s[pre.len()..].to_string();
        }
        if !suf.is_empty() && s.ends_with(suf) {
            s.truncate(s.len() - suf.len());
        }
    }
    if s == "Epoch" || s == "epoch" {
        s.clear();
    }
    if s.starts_with('(') && s.ends_with(')') && s.matches('(').count() == 1 {
        s = s[1..s.len() - 1].to_string();
    }
    let s = s.trim().to_string();
    let s = if s.is_empty() {
        fallback.to_string()
    } else {
        s
    };
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => s,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Blank,
    Heading(usize),
    Fenced,
    Text,
}
enum Item {
    Unit {
        start: usize,
        end: usize,
        fenced: bool,
    },
    Title {
        line: usize,
    },
    Block {
        line: usize,
        level: usize,
        end: usize,
        children: Vec<Item>,
    },
}
fn kinds(text: &str) -> (Vec<sections::Line<'_>>, Vec<Kind>) {
    let all = sections::lines(text);
    let k = all
        .iter()
        .map(|l| {
            if l.fenced {
                Kind::Fenced
            } else if let Some(h) = heading_level(l.text) {
                Kind::Heading(h)
            } else if l.text.trim().is_empty() {
                Kind::Blank
            } else {
                Kind::Text
            }
        })
        .collect();
    (all, k)
}
fn list_start(line: &str) -> bool {
    let indent = line.bytes().take_while(|b| *b == b' ').count();
    if indent > 1 {
        return false;
    }
    let r = &line[indent..];
    if r.starts_with("- ") || r.starts_with("* ") || r.starts_with("+ ") {
        return true;
    }
    let digits = r.bytes().take_while(u8::is_ascii_digit).count();
    digits > 0 && (r[digits..].starts_with(". ") || r[digits..].starts_with(") "))
}
/// Names of core model blocks (descriptant, domain, forward, reverse, invariant, cost,
/// checks, limits, composition, approximant); such a heading never moves whole.
fn core_heading(name: &str) -> bool {
    let n = name.to_lowercase();
    [
        "descriptant",
        "domain",
        "forward",
        "reverse",
        "invariant",
        "cost",
        "checks",
        "tests",
        "limits",
        "limitations",
        "composition",
        "approximant",
    ]
    .iter()
    .any(|w| n.split(|c: char| !c.is_alphanumeric()).any(|t| t == *w))
}
fn ends_clause(line: &str) -> bool {
    line.trim_end()
        .ends_with(['.', ':', ';', '!', '?', ')', '`', '|'])
}
fn table(line: &str) -> bool {
    line.trim_start().starts_with('|')
}
fn parse(lines: &[sections::Line], k: &[Kind], from: usize, to: usize) -> Vec<Item> {
    let mut out = Vec::new();
    let mut i = from;
    while i < to {
        match k[i] {
            Kind::Blank => i += 1,
            Kind::Heading(1) => {
                out.push(Item::Title { line: i });
                i += 1;
            }
            Kind::Heading(level) => {
                let end = (i + 1..to)
                    .find(|j| matches!(k[*j], Kind::Heading(h) if h <= level))
                    .unwrap_or(to);
                let children = parse(lines, k, i + 1, end);
                out.push(Item::Block {
                    line: i,
                    level,
                    end,
                    children,
                });
                i = end;
            }
            Kind::Fenced => {
                let end = (i..to).find(|j| k[*j] != Kind::Fenced).unwrap_or(to);
                out.push(Item::Unit {
                    start: i,
                    end,
                    fenced: true,
                });
                i = end;
            }
            Kind::Text => {
                let tab = table(lines[i].text);
                let mut j = i + 1;
                while j < to
                    && k[j] == Kind::Text
                    && !list_start(lines[j].text)
                    && table(lines[j].text) == tab
                {
                    j += 1;
                }
                // A run of record lines (each but the last ends a sentence or clause, as
                // `Type: …` / `Evidence: …` records do) splits per line; wrapped prose
                // stays one unit.
                let records = !tab
                    && j - i >= 2
                    && !list_start(lines[i].text)
                    && lines[i..j - 1].iter().all(|l| ends_clause(l.text));
                if records {
                    out.extend((i..j).map(|s| Item::Unit {
                        start: s,
                        end: s + 1,
                        fenced: false,
                    }));
                } else {
                    out.push(Item::Unit {
                        start: i,
                        end: j,
                        fenced: false,
                    });
                }
                i = j;
            }
        }
    }
    out
}

struct Section {
    name: String,
    prose_seen: bool,
    lead_used: bool,
    last_date: Option<String>,
    /// Some JSON record selects this H2: dated units stay rewritten (`strip_line`).
    selected: bool,
}
/// A History entry in document order: moved lines (with a heading shift) or a note.
struct Entry {
    order: usize,
    key: String,
    origin: String,
    body: Body,
}
enum Body {
    Lines {
        start: usize,
        end: usize,
        shift: usize,
    },
    Note(String),
}
struct Plan {
    entries: Vec<Entry>,
    renames: BTreeMap<usize, String>,
    protected_dated: usize,
    /// Lines of copied units that stay in main text: rewritten, or dropped (None).
    kept: BTreeMap<usize, Option<String>>,
}

fn code(s: &str) -> String {
    if s.contains('`') {
        format!("`` {s} ``")
    } else {
        format!("`{s}`")
    }
}
fn heading_text(line: &str) -> &str {
    let t = line.trim_start_matches(' ');
    t.trim_start_matches('#').trim_start_matches([' ', '\t'])
}
fn renamed_line(line: &str, new_name: &str) -> String {
    let indent = line.bytes().take_while(|b| *b == b' ').count();
    let hashes = line[indent..].bytes().take_while(|b| *b == b'#').count();
    format!(
        "{}{} {new_name}",
        &line[..indent],
        &line[indent..indent + hashes]
    )
}

struct Walker<'a> {
    lines: &'a [sections::Line<'a>],
    plan: Plan,
}
impl Walker<'_> {
    fn unit_text(&self, start: usize, end: usize) -> String {
        self.lines[start..end]
            .iter()
            .map(|l| l.text)
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Per-line rewrites of a dated unit, or None when a line Fails or nothing stays.
    fn strip_unit(&self, start: usize, end: usize) -> Option<Vec<(usize, Option<String>)>> {
        let mut out = Vec::new();
        let mut any = false;
        for i in start..end {
            match strip_line(self.lines[i].text, self.lines[i].fenced) {
                Strip::Fail => return None,
                Strip::Drop => out.push((i, None)),
                Strip::Keep(s) => {
                    any = true;
                    out.push((i, Some(s)));
                }
            }
        }
        any.then_some(out)
    }
    fn key(own: Option<String>, enclosing: Option<&str>, sec: &Section) -> String {
        own.or(enclosing.map(str::to_string))
            .or(sec.last_date.clone())
            .unwrap_or_else(|| UNDATED.to_string())
    }
    /// Walks the children of a kept heading; returns whether anything stays and the
    /// moves made below it (so a fully historical undated block can move whole).
    fn children(
        &mut self,
        items: &[Item],
        sec: &mut Section,
        enclosing: Option<&str>,
    ) -> (bool, Vec<Entry>) {
        let mut stayed = false;
        let mut moves = Vec::new();
        for item in items {
            match item {
                Item::Title { .. } => stayed = true,
                Item::Unit { start, end, fenced } => {
                    if *fenced {
                        // Code is neutral: it stays with prose that stays and moves with a block that moves.
                        continue;
                    }
                    let text = self.unit_text(*start, *end);
                    let own = epoch_dates(&text).into_iter().next();
                    if own.is_some() {
                        sec.last_date.clone_from(&own);
                    }
                    if (own.is_some() || provenance(&text))
                        && sec.selected
                        && let Some(kept) = self.strip_unit(*start, *end)
                    {
                        // Selected sections keep their substance: the unit stays
                        // rewritten and History holds it verbatim.
                        moves.push(Entry {
                            order: *start,
                            key: Self::key(own, enclosing, sec),
                            origin: sec.name.clone(),
                            body: Body::Lines {
                                start: *start,
                                end: *end,
                                shift: 0,
                            },
                        });
                        self.plan.kept.extend(kept);
                        stayed = true;
                        sec.prose_seen = true;
                    } else if own.is_some() || provenance(&text) {
                        moves.push(Entry {
                            order: *start,
                            key: Self::key(own, enclosing, sec),
                            origin: sec.name.clone(),
                            body: Body::Lines {
                                start: *start,
                                end: *end,
                                shift: 0,
                            },
                        });
                    } else {
                        stayed = true;
                        sec.prose_seen = true;
                    }
                }
                Item::Block {
                    line,
                    level,
                    end,
                    children,
                } => {
                    let raw = self.lines[*line].text;
                    let own = epoch_dates(heading_text(raw)).into_iter().next();
                    if let Some(d) = own {
                        sec.last_date = Some(d.clone());
                        if !sec.prose_seen && !sec.lead_used {
                            sec.lead_used = true;
                            let name = undated_name(heading_text(raw), &sec.name);
                            let new = renamed_line(raw, &name);
                            self.plan.entries.push(Entry {
                                order: *line,
                                key: d.clone(),
                                origin: sec.name.clone(),
                                body: Body::Note(format!(
                                    "{NOTE_PREFIX}{} stays in {} as {}.",
                                    code(raw),
                                    sec.name,
                                    code(&new)
                                )),
                            });
                            self.plan.renames.insert(*line, new);
                            let (_, sub) = self.children(children, sec, Some(&d));
                            moves.extend(sub);
                            stayed = true;
                        } else {
                            moves.push(Entry {
                                order: *line,
                                key: d,
                                origin: sec.name.clone(),
                                body: Body::Lines {
                                    start: *line,
                                    end: *end,
                                    shift: BASE_LEVEL.saturating_sub(*level),
                                },
                            });
                        }
                    } else if children.is_empty() {
                        stayed = true;
                    } else {
                        let (sub_stayed, sub) = self.children(children, sec, enclosing);
                        let canonical = core_heading(heading_text(raw));
                        if canonical && !sub_stayed {
                            // A core block heading stays even when all its text moved:
                            // the gap in the current model remains visible.
                            stayed = true;
                            moves.extend(sub);
                        } else if !sub_stayed && !sub.is_empty() {
                            moves.push(Entry {
                                order: *line,
                                key: sub[0].key.clone(),
                                origin: sec.name.clone(),
                                body: Body::Lines {
                                    start: *line,
                                    end: *end,
                                    shift: BASE_LEVEL.saturating_sub(*level),
                                },
                            });
                        } else {
                            stayed |= sub_stayed;
                            moves.extend(sub);
                        }
                    }
                }
            }
        }
        (stayed, moves)
    }
}

/// Counts of one transformation.
#[derive(Default, Debug, PartialEq)]
pub struct Outcome {
    pub text: String,
    pub moved_blocks: usize,
    pub moved_units: usize,
    pub renamed: usize,
    pub protected_dated: usize,
    pub heading_overflow: usize,
    /// Dated units of selected sections that stay rewritten (History holds them verbatim).
    pub stripped_units: usize,
    /// Selected sections refilled from their newest History group.
    pub refilled: Vec<String>,
    /// Selected sections still heading-only (no History group yields a current statement).
    pub residue: Vec<String>,
}

/// Undates one model.md text. `protected` holds H2 names that JSON records select:
/// their dated units stay rewritten by `strip_line`, and when such a section is left
/// heading-only its current body is restored from its newest History group (`refill`).
pub fn undate_text(
    text: &str,
    protected: &BTreeSet<String>,
    selected: &BTreeSet<String>,
) -> Result<Outcome> {
    let mut o = move_pass(text, protected, selected)?;
    let (t, refilled, residue) = refill(&o.text, selected)?;
    o.text = t;
    o.refilled = refilled;
    o.residue = residue;
    Ok(o)
}
fn move_pass(
    text: &str,
    protected: &BTreeSet<String>,
    selected: &BTreeSet<String>,
) -> Result<Outcome> {
    ensure!(
        !sections::unclosed_fence(text),
        "text ends inside a code fence"
    );
    let (lines, k) = kinds(text);
    let n = lines.len();
    let items = parse(&lines, &k, 0, n);
    let mut w = Walker {
        lines: &lines,
        plan: Plan {
            entries: Vec::new(),
            renames: BTreeMap::new(),
            protected_dated: 0,
            kept: BTreeMap::new(),
        },
    };
    let mut history: Option<(usize, usize)> = None;
    let mut doc = Section {
        name: "document".into(),
        prose_seen: true,
        lead_used: true,
        last_date: None,
        selected: false,
    };
    for item in &items {
        match item {
            Item::Title { line } => {
                let raw = lines[*line].text;
                if let Some(d) = epoch_dates(heading_text(raw)).into_iter().next() {
                    let name = undated_name(heading_text(raw), "Model");
                    let new = renamed_line(raw, &name);
                    w.plan.entries.push(Entry {
                        order: *line,
                        key: d,
                        origin: "document".into(),
                        body: Body::Note(format!(
                            "{NOTE_PREFIX}{} stays in document as {}.",
                            code(raw),
                            code(&new)
                        )),
                    });
                    w.plan.renames.insert(*line, new);
                }
            }
            Item::Unit { .. } => {
                let (_, moves) = w.children(std::slice::from_ref(item), &mut doc, None);
                w.plan.entries.extend(moves);
            }
            Item::Block {
                line,
                level,
                end,
                children,
            } => {
                debug_assert_eq!(*level, 2);
                let name = heading_text(lines[*line].text).to_string();
                if lines[*line].text == format!("## {HISTORY}") {
                    ensure!(history.is_none(), "duplicate ## {HISTORY}");
                    history = Some((*line, *end));
                    continue;
                }
                ensure!(history.is_none(), "## {HISTORY} is not the last H2 section");
                let own = epoch_dates(&name).into_iter().next();
                if let Some(d) = own.clone()
                    && !protected.contains(&name)
                {
                    w.plan.entries.push(Entry {
                        order: *line,
                        key: d,
                        origin: "document".into(),
                        body: Body::Lines {
                            start: *line,
                            end: *end,
                            shift: BASE_LEVEL - 2,
                        },
                    });
                    continue;
                }
                if own.is_some() {
                    w.plan.protected_dated += 1;
                }
                let mut sec = Section {
                    selected: selected.contains(&name),
                    name,
                    prose_seen: false,
                    lead_used: false,
                    last_date: own.clone(),
                };
                let (_, moves) = w.children(children, &mut sec, own.as_deref());
                w.plan.entries.extend(moves);
            }
        }
    }
    let plan = w.plan;
    let protected_dated = plan.protected_dated;
    if plan.entries.is_empty() {
        return Ok(Outcome {
            text: text.to_string(),
            protected_dated,
            ..Outcome::default()
        });
    }
    // Main text: drop moved lines, apply renames, collapse blank runs left by removals.
    let mut removed = vec![false; n];
    let mut groups: Vec<(String, String, String)> = Vec::new();
    let (mut moved_blocks, mut moved_units, mut overflow, mut stripped_units) = (0, 0, 0, 0);
    let mut entries = plan.entries;
    entries.sort_by_key(|e| e.order);
    for e in &entries {
        let body = match &e.body {
            Body::Note(s) => s.clone(),
            Body::Lines { start, end, shift } => {
                removed[*start..*end].iter_mut().for_each(|r| *r = true);
                let last = (*start..*end)
                    .rev()
                    .find(|j| k[*j] != Kind::Blank)
                    .unwrap_or(*start);
                let raw = &text[lines[*start].start..lines[last].start + lines[last].text.len()];
                if matches!(k[*start], Kind::Heading(_)) {
                    moved_blocks += 1;
                    let (d, o) = demote(raw, *shift);
                    overflow += o;
                    d
                } else if plan.kept.contains_key(start) {
                    stripped_units += 1;
                    raw.to_string()
                } else {
                    moved_units += 1;
                    raw.to_string()
                }
            }
        };
        groups.push((e.key.clone(), e.origin.clone(), body));
    }
    let mut main = String::new();
    let (mut last_blank, mut after_removal) = (true, false);
    let main_end = history.map_or(n, |h| h.0);
    for i in 0..main_end {
        if let Some(kept) = plan.kept.get(&i) {
            match kept {
                Some(new) => {
                    main.push_str(new);
                    main.push_str(&text[lines[i].start + lines[i].text.len()..lines[i].end]);
                    last_blank = false;
                    after_removal = false;
                }
                None => after_removal = true,
            }
            continue;
        }
        if removed[i] {
            after_removal = true;
            continue;
        }
        let blank = k[i] == Kind::Blank;
        if blank && after_removal && last_blank {
            continue;
        }
        match plan.renames.get(&i) {
            Some(new) => {
                main.push_str(new);
                main.push_str(&text[lines[i].start + lines[i].text.len()..lines[i].end]);
            }
            None => main.push_str(&text[lines[i].start..lines[i].end]),
        }
        last_blank = blank;
        if !blank {
            after_removal = false;
        }
    }
    // Existing History groups come first, so a stable date sort appends new ones.
    let mut all: Vec<(String, String, String)> = Vec::new();
    if let Some((h, end)) = history {
        all = parse_history(&lines, &k, text, h, end)?;
    }
    all.extend(groups);
    all.sort_by(|a, b| sort_key(&a.0).cmp(sort_key(&b.0)));
    let mut out = main.trim_end().to_string();
    out.push_str(&format!("\n\n## {HISTORY}\n"));
    let mut prev: Option<(&str, &str)> = None;
    for (key, origin, body) in &all {
        if prev != Some((key.as_str(), origin.as_str())) {
            out.push_str(&format!("\n### {key}{GROUP_SEP}{origin}\n"));
        }
        out.push('\n');
        out.push_str(body.trim_end());
        out.push('\n');
        prev = Some((key, origin));
    }
    Ok(Outcome {
        text: out,
        moved_blocks,
        moved_units,
        renamed: plan.renames.len(),
        protected_dated,
        heading_overflow: overflow,
        stripped_units,
        ..Outcome::default()
    })
}
/// True when lines `from..to` hold anything but blank lines and headings.
fn has_body(k: &[Kind], from: usize, to: usize) -> bool {
    k[from..to]
        .iter()
        .any(|x| matches!(x, Kind::Text | Kind::Fenced))
}
/// Restores the current body of every selected H2 section that is heading-only (no
/// line but blank lines and headings) from History: the groups of that origin are
/// tried newest first (dated keys descending, then `Undated`) and the first whose
/// `restate` is nonempty becomes the body, inserted right after the H2 heading.
/// History is not changed. Returns the text, the refilled and the residue sections.
pub fn refill(
    text: &str,
    selected: &BTreeSet<String>,
) -> Result<(String, Vec<String>, Vec<String>)> {
    let (lines, k) = kinds(text);
    let n = lines.len();
    let h2: Vec<usize> = (0..n).filter(|i| k[*i] == Kind::Heading(2)).collect();
    let history = h2
        .iter()
        .position(|i| lines[*i].text == format!("## {HISTORY}"));
    let groups = match history {
        Some(p) => {
            let end = h2.get(p + 1).copied().unwrap_or(n);
            parse_history(&lines, &k, text, h2[p], end)?
        }
        None => Vec::new(),
    };
    let main_h2 = &h2[..history.unwrap_or(h2.len())];
    let main_end = history.map_or(n, |p| h2[p]);
    let (mut refilled, mut residue) = (Vec::new(), Vec::new());
    let mut inserts: Vec<(usize, String)> = Vec::new();
    for (j, &i) in main_h2.iter().enumerate() {
        let name = heading_text(lines[i].text);
        let end = main_h2.get(j + 1).copied().unwrap_or(main_end);
        if !selected.contains(name) || has_body(&k, i + 1, end) {
            continue;
        }
        let mut keys: Vec<&str> = groups
            .iter()
            .filter(|g| g.1 == name)
            .map(|g| g.0.as_str())
            .collect();
        keys.sort_by(|a, b| sort_key(b).cmp(sort_key(a)));
        keys.dedup();
        // Undated sorts as newest; it is tried only after every dated group.
        keys.sort_by_key(|key| *key == UNDATED);
        let body = keys.iter().find_map(|key| {
            let joined = groups
                .iter()
                .filter(|g| g.0 == *key && g.1 == name)
                .map(|g| g.2.as_str())
                .collect::<Vec<_>>()
                .join("\n\n");
            let r = restate(&joined);
            (!r.is_empty()).then_some(r)
        });
        match body {
            Some(b) => {
                refilled.push(name.to_string());
                inserts.push((lines[i].end, b));
            }
            None => residue.push(name.to_string()),
        }
    }
    let mut out = text.to_string();
    for (at, body) in inserts.into_iter().rev() {
        let sep = if text[..at].ends_with('\n') { "" } else { "\n" };
        out.insert_str(at, &format!("{sep}\n{body}\n"));
    }
    Ok((out, refilled, residue))
}
/// The current statement a History group body yields: every line through
/// `strip_line`; a paragraph or list item with a Failing line, or with nothing kept,
/// is left out; a fence follows only a kept paragraph; a heading only kept content.
pub fn restate(body: &str) -> String {
    let (lines, k) = kinds(body);
    let items = parse(&lines, &k, 0, lines.len());
    let mut pieces: Vec<(usize, usize, String)> = Vec::new();
    restate_items(&lines, &items, &mut pieces);
    let mut out = String::new();
    let mut prev_end: Option<usize> = None;
    for (start, end, s) in pieces {
        if let Some(p) = prev_end {
            out.push_str(if p == start { "\n" } else { "\n\n" });
        }
        out.push_str(&s);
        prev_end = Some(end);
    }
    out
}
fn restate_items(
    lines: &[sections::Line],
    items: &[Item],
    out: &mut Vec<(usize, usize, String)>,
) -> bool {
    let mut any = false;
    let mut last_kept = false;
    for item in items {
        match item {
            Item::Title { .. } => last_kept = false,
            Item::Unit {
                start,
                end,
                fenced: true,
            } => {
                if last_kept {
                    let s = lines[*start..*end]
                        .iter()
                        .map(|l| l.text)
                        .collect::<Vec<_>>();
                    out.push((*start, *end, s.join("\n")));
                }
            }
            Item::Unit { start, end, .. } => {
                let mut kept = Vec::new();
                let mut fail = false;
                for l in &lines[*start..*end] {
                    match strip_line(l.text, false) {
                        Strip::Keep(s) => kept.push(s),
                        Strip::Drop => {}
                        Strip::Fail => fail = true,
                    }
                }
                last_kept = !fail && !kept.is_empty();
                if last_kept {
                    any = true;
                    out.push((*start, *end, kept.join("\n")));
                }
            }
            Item::Block { line, children, .. } => {
                last_kept = false;
                let mut sub = Vec::new();
                match strip_line(lines[*line].text, false) {
                    Strip::Fail => {}
                    Strip::Drop => {
                        if restate_items(lines, children, &mut sub) {
                            any = true;
                            out.extend(sub);
                        }
                    }
                    Strip::Keep(h) => {
                        if restate_items(lines, children, &mut sub) {
                            any = true;
                            out.push((*line, *line + 1, h));
                            out.extend(sub);
                        }
                    }
                }
            }
        }
    }
    any
}
fn sort_key(key: &str) -> &str {
    if key == UNDATED { "9999" } else { key }
}
fn parse_group_header(text: &str) -> Option<(String, String)> {
    let rest = text.strip_prefix("### ")?;
    let (key, origin) = rest.split_once(GROUP_SEP)?;
    let dated = key.len() == 10 && epoch_dates(key) == [key.to_string()];
    (dated || key == UNDATED).then(|| (key.to_string(), origin.to_string()))
}
/// Splits an existing History section into (key, origin, body) groups; each body
/// paragraph-run keeps its bytes.
fn parse_history(
    lines: &[sections::Line],
    k: &[Kind],
    text: &str,
    h: usize,
    end: usize,
) -> Result<Vec<(String, String, String)>> {
    let mut out: Vec<(String, String, String)> = Vec::new();
    let mut current: Option<(String, String, usize)> = None;
    let flush = |cur: &Option<(String, String, usize)>, stop: usize, out: &mut Vec<_>| {
        if let Some((key, origin, from)) = cur {
            let body_lines: Vec<usize> = (*from..stop).filter(|j| k[*j] != Kind::Blank).collect();
            if let (Some(a), Some(b)) = (body_lines.first(), body_lines.last()) {
                let body =
                    text[lines[*a].start..lines[*b].start + lines[*b].text.len()].to_string();
                out.push((key.clone(), origin.clone(), body));
            }
        }
    };
    for i in h + 1..end {
        if k[i] == Kind::Heading(3) {
            let Some((key, origin)) = parse_group_header(lines[i].text) else {
                bail!(
                    "History group heading is not `### <date> · <origin>`: {}",
                    lines[i].text
                );
            };
            flush(&current, i, &mut out);
            current = Some((key, origin, i + 1));
        } else if current.is_none() && k[i] != Kind::Blank {
            bail!("History text before its first group heading");
        }
    }
    flush(&current, end, &mut out);
    Ok(out)
}

/// One nonblank line: key (a heading without its level), raw text, whether it lies in
/// History, its main-text H2 section and whether it is fenced.
struct Row {
    key: String,
    raw: String,
    hist: bool,
    section: String,
    fenced: bool,
}
fn row_key(raw: &str, fenced: bool) -> String {
    match (!fenced).then(|| heading_level(raw)).flatten() {
        Some(_) => format!("H:{}", heading_text(raw)),
        None => raw.to_string(),
    }
}
fn normalized(text: &str) -> Vec<Row> {
    let (lines, k) = kinds(text);
    let mut in_history = false;
    let mut section = String::new();
    let mut out = Vec::new();
    for (l, kind) in lines.iter().zip(&k) {
        if *kind == Kind::Blank {
            continue;
        }
        if *kind == Kind::Heading(2) {
            in_history = l.text == format!("## {HISTORY}");
            section = heading_text(l.text).to_string();
        }
        let fenced = *kind == Kind::Fenced;
        out.push(Row {
            key: row_key(l.text, fenced),
            raw: l.text.to_string(),
            hist: in_history,
            section: section.clone(),
            fenced,
        });
    }
    out
}
/// The heading a History note says a lost heading stays as: `… as `NEW`.`
fn note_target(note: &str) -> Option<&str> {
    let t = note.rsplit_once(" as ")?.1.strip_suffix('.')?;
    t.strip_prefix("`` ")
        .and_then(|t| t.strip_suffix(" ``"))
        .or_else(|| t.strip_prefix('`').and_then(|t| t.strip_suffix('`')))
}

/// Independent check of one transformation (the negative-control gate): nonblank
/// lines are preserved as a multiset (headings compared without their `#` level);
/// every missing line is a dated heading renamed in place with a History note quoting
/// it; additions are only the History heading, group headings, notes, renamed
/// headings, and — inside protected (selected) H2 sections of the main text — lines
/// that are the `strip_line` image of a History line (stripped units and refilled
/// bodies, whose verbatim source History holds); no epoch date or provenance remains
/// outside History and fences except on protected H2 headings; History, if present,
/// is the single last H2.
pub fn verify(
    old: &str,
    new: &str,
    protected: &BTreeSet<String>,
    selected: &BTreeSet<String>,
) -> Result<()> {
    let count = |v: &[Row]| {
        let mut m: BTreeMap<String, i64> = BTreeMap::new();
        v.iter()
            .for_each(|r| *m.entry(r.key.clone()).or_default() += 1);
        m
    };
    let (o, nw) = (normalized(old), normalized(new));
    let (mo, mn) = (count(&o), count(&nw));
    let mut missing: Vec<String> = Vec::new();
    for (key, c) in &mo {
        for _ in 0..(c - mn.get(key).copied().unwrap_or(0)).max(0) {
            missing.push(key.clone());
        }
    }
    let note = |r: &Row| r.hist && r.raw.starts_with(NOTE_PREFIX);
    let new_notes: Vec<&str> = nw
        .iter()
        .filter(|r| note(r))
        .map(|r| r.raw.as_str())
        .collect();
    let old_raw: BTreeMap<&str, Vec<&str>> = o.iter().fold(BTreeMap::new(), |mut m, r| {
        m.entry(r.key.as_str())
            .or_insert_with(Vec::new)
            .push(r.raw.as_str());
        m
    });
    let mut targets: BTreeMap<String, i64> = BTreeMap::new();
    for m in &missing {
        ensure!(
            m.starts_with("H:") && !epoch_dates(m).is_empty(),
            "line lost: {m}"
        );
        let quoting: Vec<&&str> = new_notes
            .iter()
            .filter(|n| old_raw[m.as_str()].iter().any(|raw| n.contains(&code(raw))))
            .collect();
        ensure!(
            !quoting.is_empty(),
            "renamed heading without a History note: {m}"
        );
        if let Some(t) = quoting.iter().find_map(|n| note_target(n)) {
            *targets.entry(row_key(t, false)).or_default() += 1;
        }
    }
    let derivable: BTreeSet<String> = nw
        .iter()
        .filter(|r| r.hist && !note(r) && parse_group_header(&r.raw).is_none())
        .filter(|r| heading_level(&r.raw) != Some(2))
        .filter_map(|r| match strip_line(&r.raw, r.fenced) {
            Strip::Keep(s) => Some(row_key(&s, r.fenced)),
            _ => None,
        })
        .collect();
    let (mut renamed, mut notes, mut histories) = (0i64, 0usize, 0usize);
    for (key, c) in &mn {
        let extra = c - mo.get(key).copied().unwrap_or(0);
        if extra <= 0 {
            continue;
        }
        let rows: Vec<&Row> = nw.iter().filter(|r| &r.key == key).collect();
        let last = rows[rows.len() - 1];
        if last.raw == format!("## {HISTORY}") {
            histories += extra as usize;
        } else if note(last) {
            notes += extra as usize;
        } else if last.hist && parse_group_header(&last.raw).is_some() {
        } else {
            let r = extra.min(targets.get(key).copied().unwrap_or(0));
            if r > 0 {
                ensure!(
                    rows.iter().all(|x| !x.hist) && epoch_dates(key).is_empty(),
                    "renamed heading misplaced: {}",
                    last.raw
                );
            }
            renamed += r;
            if extra > r {
                let in_selected = rows
                    .iter()
                    .any(|x| !x.hist && selected.contains(&x.section));
                ensure!(
                    derivable.contains(key) && in_selected,
                    "invented line: {}",
                    last.raw
                );
            }
        }
    }
    ensure!(histories <= 1, "more than one History heading added");
    ensure!(
        renamed == missing.len() as i64 && notes >= missing.len(),
        "renames ({renamed}), notes ({notes}) and lost headings ({}) disagree",
        missing.len()
    );
    let (lines, k) = kinds(new);
    let h2: Vec<usize> = (0..lines.len())
        .filter(|i| k[*i] == Kind::Heading(2))
        .collect();
    let hist: Vec<usize> = h2
        .iter()
        .copied()
        .filter(|i| lines[*i].text == format!("## {HISTORY}"))
        .collect();
    ensure!(hist.len() <= 1, "duplicate History sections");
    if let Some(h) = hist.first() {
        ensure!(h2.last() == Some(h), "History is not the last H2 section");
    }
    let stop = hist.first().copied().unwrap_or(lines.len());
    for i in 0..stop {
        if matches!(k[i], Kind::Fenced | Kind::Blank) {
            continue;
        }
        let t = lines[i].text;
        if k[i] == Kind::Heading(2) && protected.contains(heading_text(t)) {
            continue;
        }
        ensure!(epoch_dates(t).is_empty(), "dated line outside History: {t}");
        if !matches!(k[i], Kind::Heading(_)) {
            ensure!(!provenance(t), "provenance outside History: {t}");
        }
    }
    Ok(())
}

/// Sections that JSON records select, per target file: `path` plus `section` in one
/// object, or a `file#Section` value (`path`, `description`, `evidence`, …), resolved
/// against the JSON file's directory.
fn selections(texts: &BTreeMap<String, String>) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (f, text) in texts {
        let lits = json_literals(text).with_context(|| f.clone())?;
        let mut fields: BTreeMap<usize, BTreeMap<&str, &str>> = BTreeMap::new();
        for l in &lits {
            if let (Some(o), Some(key)) = (l.object, l.key.as_deref()) {
                fields.entry(o).or_default().insert(key, l.value.as_str());
            }
            if let (file, Some(section)) = split_reference(&l.value)
                && file.ends_with(".md")
                && let Some(target) = join(parent(f), file)
            {
                out.entry(target).or_default().insert(section.to_string());
            }
        }
        for keys in fields.values() {
            if let (Some(p), Some(s)) = (keys.get("path"), keys.get("section"))
                && let Some(target) = join(parent(f), split_reference(p).0)
            {
                out.entry(target).or_default().insert(s.to_string());
            }
        }
    }
    Ok(out)
}

/// H2 names that tracked JSON selects: `section` values and `model.md#Name` suffixes.
fn protected_names(texts: &BTreeMap<String, String>) -> Result<BTreeSet<String>> {
    let mut out = BTreeSet::new();
    for (f, text) in texts {
        for l in json_literals(text).with_context(|| f.clone())? {
            if l.key.as_deref() == Some("section") {
                out.insert(l.value.clone());
            }
            if let Some((_, s)) = l.value.split_once("model.md#") {
                out.insert(s.to_string());
            }
        }
    }
    Ok(out)
}

fn bound<'t>(text: &'t str, section: Option<&str>) -> Option<&'t str> {
    match section {
        Some(s) => sections::section_bytes(text, s),
        None => Some(text),
    }
}

/// Rebinds fresh hash-bound references (`path` plus `sha256` in one JSON object) to
/// files whose text changed, to fixpoint: JSON files changed by a rebind are targets
/// of the next round. Returns (rebinds, already-stale bindings left untouched).
pub(crate) fn rebind(
    changed: &mut BTreeMap<String, Vec<String>>,
    json_files: &[String],
    json_texts: &mut BTreeMap<String, String>,
) -> Result<(Vec<Value>, Vec<Value>)> {
    let mut rebinds = Vec::new();
    let mut stale = BTreeSet::new();
    for round in 0..16 {
        let mut any = false;
        for f in json_files {
            let text = json_texts[f].clone();
            let lits = json_literals(&text).with_context(|| f.clone())?;
            let mut fields: BTreeMap<usize, BTreeMap<&str, usize>> = BTreeMap::new();
            for (i, l) in lits.iter().enumerate() {
                if let (Some(o), Some(key)) = (l.object, l.key.as_deref()) {
                    fields.entry(o).or_default().insert(key, i);
                }
            }
            let mut edits: BTreeMap<usize, String> = BTreeMap::new();
            for keys in fields.values() {
                let (Some(p), Some(h)) = (keys.get("path"), keys.get("sha256")) else {
                    continue;
                };
                let reference = &lits[*p].value;
                let (file, section) = split_reference(reference);
                let Some(target) = [join(parent(f), file), join("", file)]
                    .into_iter()
                    .flatten()
                    .find(|c| changed.contains_key(c))
                else {
                    continue;
                };
                let versions = &changed[&target];
                let current = versions.last().expect("versions");
                let new_sha = bound(current, section)
                    .map(|b| sha(b.as_bytes()))
                    .with_context(|| format!("{f}: section {reference} missing after undating"))?;
                let have = &lits[*h].value;
                if *have == new_sha {
                    continue;
                }
                let fresh = versions[..versions.len() - 1]
                    .iter()
                    .any(|v| bound(v, section).is_some_and(|b| sha(b.as_bytes()) == *have));
                if fresh {
                    edits.insert(*h, new_sha.clone());
                    rebinds.push(json!({"file": f, "reference": reference, "target": target,
                        "old_sha256": have, "new_sha256": new_sha, "round": round}));
                } else {
                    stale.insert((f.clone(), reference.clone()));
                }
            }
            if edits.is_empty() {
                continue;
            }
            any = true;
            let mut out = String::with_capacity(text.len());
            let mut last = 0;
            for (i, v) in &edits {
                out.push_str(&text[last..lits[*i].start]);
                out.push_str(&serde_json::to_string(v)?);
                last = lits[*i].end;
            }
            out.push_str(&text[last..]);
            serde_json::from_str::<Value>(&out)
                .with_context(|| format!("rebound JSON {f} does not parse"))?;
            changed
                .entry(f.clone())
                .or_insert_with(|| vec![text.clone()])
                .push(out.clone());
            json_texts.insert(f.clone(), out);
        }
        if !any {
            let stale = stale
                .into_iter()
                .map(
                    |(f, r)| json!({"file": f, "reference": r, "note": "already stale; untouched"}),
                )
                .collect();
            return Ok((rebinds, stale));
        }
    }
    bail!("hash rebinding did not reach a fixpoint")
}

/// Undates every tracked `*.groupant/**/model.md`, verifies each result and its
/// idempotence, rebinds fresh hash-bound JSON references and (unless `dry_run`)
/// writes the files. Returns the report.
pub fn run(repo: &Path, dry_run: bool) -> Result<Value> {
    let files = git_files(repo)?;
    let json_files: Vec<String> = files
        .iter()
        .filter(|f| f.ends_with(".json"))
        .cloned()
        .collect();
    let mut json_texts = BTreeMap::new();
    for f in &json_files {
        json_texts.insert(
            f.clone(),
            fs::read_to_string(repo.join(f)).with_context(|| f.clone())?,
        );
    }
    let protected = protected_names(&json_texts)?;
    let selected = selections(&json_texts)?;
    let no_selection = BTreeSet::new();
    let models: Vec<&String> = files
        .iter()
        .filter(|f| {
            (f.as_str() == "model.md" || f.ends_with("/model.md")) && f.contains(".groupant/")
        })
        .collect();
    let mut changed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut per_file = Vec::new();
    let (mut blocks, mut units, mut renamed, mut protected_dated, mut overflow) = (0, 0, 0, 0, 0);
    let (mut stripped, mut refilled, mut residue) = (0, Vec::new(), Vec::new());
    for f in &models {
        let old = fs::read_to_string(repo.join(f)).with_context(|| f.to_string())?;
        let sel = selected.get(f.as_str()).unwrap_or(&no_selection);
        let o = undate_text(&old, &protected, sel).with_context(|| f.to_string())?;
        protected_dated += o.protected_dated;
        residue.extend(o.residue.iter().map(|s| json!({"file": f, "section": s})));
        if o.text == old {
            continue;
        }
        verify(&old, &o.text, &protected, sel).with_context(|| format!("verify {f}"))?;
        let again = undate_text(&o.text, &protected, sel)?;
        ensure!(again.text == o.text, "{f}: second run changes the text");
        blocks += o.moved_blocks;
        units += o.moved_units;
        renamed += o.renamed;
        overflow += o.heading_overflow;
        stripped += o.stripped_units;
        refilled.extend(o.refilled.iter().map(|s| json!({"file": f, "section": s})));
        per_file.push(
            json!({"file": f, "moved_blocks": o.moved_blocks, "moved_units": o.moved_units,
            "renamed": o.renamed, "stripped_units": o.stripped_units, "refilled": o.refilled, "bytes_before": old.len(), "bytes_after": o.text.len()}),
        );
        changed.insert(f.to_string(), vec![old, o.text]);
    }
    let changed_models: Vec<String> = changed.keys().cloned().collect();
    let (rebinds, stale) = rebind(&mut changed, &json_files, &mut json_texts)?;
    let json_changed: Vec<&String> = changed.keys().filter(|f| f.ends_with(".json")).collect();
    if !dry_run {
        for (f, versions) in &changed {
            fs::write(repo.join(f), versions.last().expect("versions"))?;
        }
    }
    Ok(json!({
        "schema": 1,
        "kind": "groupant_undate",
        "dry_run": dry_run,
        "model_files": models.len(),
        "changed_model_files": changed_models.len(),
        "moved_blocks": blocks,
        "moved_units": units,
        "renamed_headings": renamed,
        "protected_dated_h2": protected_dated,
        "heading_overflow": overflow,
        "stripped_units": stripped,
        "refilled_sections": refilled,
        "residue_sections": residue,
        "json_files_changed": json_changed,
        "hash_rebinds": rebinds,
        "stale_bindings_untouched": stale,
        "files": per_file,
        "semantics_checked": false
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const A_STAR: &str = "# algorithm-a-star\n\n## descriptant\n\n### algorithm-a-star descriptant\n\n#### Design epoch 2026-10-30\n\nValidate start then goal (Hart, Nilsson and Raphael 1968).\n\n#### Prospective predictions\n\nExact agreement.\n\n#### Review epoch 2026-11-01\n\nA diamond fixture.\n\n## Forward\n\nOrder by (g+h,g,vertex); (Kahn 1962).\n\n## evidence\n\n### Finite evidence\n\nDate: 2026-10-30. Source SHA-256: `76a320c9ade011864661dd9edcbad7ca6d200346cbdea407d626eaedf024670d`.\n\nThe DFS enumerates 729 graphs.\n\n#### Executed validation\n\nAll pass; SHA-256 `f9d87c0f117ccf985e0cc849c0a0ed47d85c1acf513a9db7c52c1f16edc01337`.\n\n```sh\nrun 2026-10-30\n```\n\n#### Heuristic projection review — 2026-11-01\n\nFor the diamond.\n";

    fn none() -> BTreeSet<String> {
        BTreeSet::new()
    }

    #[test]
    fn dates_skip_paper_years_versions_seeds_and_toolchain_builds() {
        assert_eq!(
            epoch_dates("Epoch 2026-10-30 (wave37), 2026-11-01."),
            ["2026-10-30", "2026-11-01"]
        );
        assert_eq!(
            epoch_dates("run root octopus-wave7-2026-10-30/"),
            ["2026-10-30"]
        );
        assert_eq!(epoch_dates("Epoch2026-11-02"), ["2026-11-02"]);
        for s in [
            "(Kahn 1962), Hart et al. 1968, arXiv 2410.12345",
            "rustc 1.99.0 (b940084d7 2026-09-28)",
            "seed 0x6b7520261029, v0.4, SPDX 2.3",
            "dump export format (2026-09-01 run)",
            "the 2026-09-01 English dump",
            "1969-12-31 and 2000-03-01 and 12026-10-301",
            "2026-13-01 2026-00-10 2026-10-32",
        ] {
            assert!(epoch_dates(s).is_empty(), "{s}");
        }
    }

    #[test]
    fn provenance_finds_digests_and_run_roots_only() {
        assert!(provenance("root `$MOVA_DATA/runs/x/`") && provenance("${MOVA_DATA}/runs"));
        assert!(!provenance(
            "raw outputs stay in `$MOVA_DATA` and are referenced"
        ));
        assert!(provenance(
            "SHA-256 `76a320c9ade011864661dd9edcbad7ca6d200346cbdea407d626eaedf024670d`."
        ));
        assert!(provenance("from `3c03bb63…` to"));
        assert!(provenance(
            "at source SHA502037c74e4cf1b88674de6594fe4a611e92eacc9d13ee96db6aac1b5192 x"
        ));
        assert!(!provenance(
            "id sha502037c74e4cf1b88674de6594fe4a611e92eacc9d13ee96db6aac1b5192x"
        ));
        assert!(!provenance(
            "commit 8cff45c64 renamed; SHA-256 digests are compared"
        ));
        assert!(!provenance(
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdead is letters only? no: abcdefabcdefabcdefabcdefabcdefabcdefabcdef"
        ));
    }

    #[test]
    fn undated_names() {
        assert_eq!(undated_name("Design epoch 2026-10-30", "x"), "Design");
        assert_eq!(
            undated_name("Heuristic projection review — 2026-11-01", "x"),
            "Heuristic projection review"
        );
        assert_eq!(undated_name("Evidence, 2026-10-31", "x"), "Evidence");
        assert_eq!(undated_name("Checks (2026-10-31)", "x"), "Checks");
        assert_eq!(undated_name("Epoch 2026-11-04 (senses)", "x"), "Senses");
        assert_eq!(undated_name("Epoch 2026-11-04", "tests"), "Tests");
        assert_eq!(
            undated_name("Epoch2026-11-04", "descriptant"),
            "Descriptant"
        );
        assert_eq!(
            undated_name("Lesson model, 2026-10-29", "Model"),
            "Lesson model"
        );
    }

    #[test]
    fn a_star_moves_history_keeps_lead_and_verifies() {
        let o = undate_text(A_STAR, &none(), &none()).unwrap();
        let t = &o.text;
        assert!(t.contains("#### Design\n\nValidate start"), "{t}");
        assert!(t.contains("(Hart, Nilsson and Raphael 1968)") && t.contains("(Kahn 1962)"));
        let (main, hist) = t.split_once("\n## History\n").unwrap();
        assert!(epoch_dates(main).is_empty() && !provenance(main), "{main}");
        assert!(
            main.contains("The DFS enumerates") && main.contains("#### Prospective predictions")
        );
        assert!(hist.contains("### 2026-10-30 · descriptant\n\nHeading `#### Design epoch 2026-10-30` stays in descriptant as `#### Design`."));
        assert!(hist.contains(
            "### 2026-11-01 · descriptant\n\n#### Review epoch 2026-11-01\n\nA diamond fixture."
        ));
        // An undated block whose content is all provenance moves whole, fence included.
        assert!(hist.contains("#### Executed validation\n\nAll pass;"));
        assert!(hist.contains("run 2026-10-30\n```"));
        // Chronological: every 10-30 group precedes every 11-01 group.
        assert!(hist.rfind("### 2026-10-30").unwrap() < hist.find("### 2026-11-01").unwrap());
        assert_eq!((o.moved_blocks, o.moved_units, o.renamed), (3, 1, 1));
        verify(A_STAR, t, &none(), &none()).unwrap();
        assert_eq!(
            undate_text(t, &none(), &none()).unwrap().text,
            *t,
            "idempotent"
        );
    }

    #[test]
    fn negative_controls_fail_verification() {
        let good = undate_text(A_STAR, &none(), &none()).unwrap().text;
        let cases = [
            ("dropped line", good.replace("The DFS enumerates 729 graphs.\n", "")),
            ("changed word", good.replace("A diamond fixture.", "A square fixture.")),
            ("invented line", good.replace("Exact agreement.", "Exact agreement.\n\nInvented claim.")),
            ("date left in main", good.replace("Exact agreement.", "Exact agreement on 2026-11-01.")),
            ("hash left in main", good.replace("The DFS", "`f9d87c0f117ccf985e0cc849c0a0ed47d85c1acf513a9db7c52c1f16edc01337` The DFS")),
            ("rename without note", good.replace("Heading `#### Design epoch 2026-10-30` stays in descriptant as `#### Design`.\n", "")),
            ("History not last", format!("{good}\n## after\n\nx\n")),
        ];
        for (name, bad) in cases {
            assert!(
                verify(A_STAR, &bad, &none(), &none()).is_err(),
                "{name} passed"
            );
        }
        assert!(
            verify(A_STAR, A_STAR, &none(), &none()).is_err(),
            "untransformed dated text passes"
        );
    }

    #[test]
    fn protected_h2_stays_and_unprotected_dated_h2_moves() {
        let text = "# x\n\n## Forward\n\nf\n\n## Epoch 2026-11-04 (pool)\n\nPool text.\n\n### Sub\n\ns\n\n## Source survey (2026-11-01)\n\nKept.\n\n## tests\n\n### Checks\n\nc\n";
        let protected: BTreeSet<String> = ["Source survey (2026-11-01)".to_string()].into();
        let o = undate_text(text, &protected, &protected).unwrap();
        assert!(o.text.contains("## Source survey (2026-11-01)\n\nKept."));
        assert!(o.text.contains(
            "### 2026-11-04 · document\n\n#### Epoch 2026-11-04 (pool)\n\nPool text.\n\n##### Sub"
        ));
        assert_eq!(
            sections::section_bytes(&o.text, "tests"),
            Some("## tests\n\n### Checks\n\nc")
        );
        assert_eq!(o.protected_dated, 1);
        verify(text, &o.text, &protected, &protected).unwrap();
        assert!(
            verify(text, &o.text, &none(), &none()).is_err(),
            "protection is needed for the date to stay"
        );
    }

    #[test]
    fn new_material_merges_into_existing_history() {
        let first = undate_text(A_STAR, &none(), &none()).unwrap().text;
        let (main, hist) = first.split_once("\n## History\n").unwrap();
        let grown = format!(
            "{main}\n\n## more\n\nLater note, 2026-11-05.\n\nUndated now.\n\n## History\n{hist}"
        );
        let o = undate_text(&grown, &none(), &none()).unwrap();
        assert!(
            o.text
                .ends_with("### 2026-11-05 · more\n\nLater note, 2026-11-05.\n"),
            "{}",
            o.text
        );
        assert!(o.text.contains("## more\n\nUndated now.\n\n## History"));
        verify(&grown, &o.text, &none(), &none()).unwrap();
        assert_eq!(undate_text(&o.text, &none(), &none()).unwrap().text, o.text);
    }

    #[test]
    fn lists_and_tables_move_per_item() {
        let text = "# x\n\n## tests\n\nChecks:\n- a passes\n- b passes, 2026-10-31\n  continued\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
        let o = undate_text(text, &none(), &none()).unwrap();
        assert!(
            o.text.contains("Checks:\n- a passes\n\n| a | b |"),
            "{}",
            o.text
        );
        assert!(
            o.text
                .contains("### 2026-10-31 · tests\n\n- b passes, 2026-10-31\n  continued\n")
        );
        verify(text, &o.text, &none(), &none()).unwrap();
    }

    #[test]
    fn record_lines_move_alone_and_wrapped_prose_moves_whole() {
        let digest = "222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6";
        let text = format!(
            "# x\n\n## behavior-complement\n\nSubject/revision: mod.rs sha256 {digest}.\nDomain: vertex counts.\nStatus: partial.\n\n## Forward\n\nA wrapped sentence that\ncontinues on 2026-10-30 and\nends here.\n"
        );
        let o = undate_text(&text, &none(), &none()).unwrap();
        assert!(
            o.text
                .contains("## behavior-complement\n\nDomain: vertex counts.\nStatus: partial.\n"),
            "{}",
            o.text
        );
        assert!(o.text.contains("### 2026-10-30 · Forward\n\nA wrapped sentence that\ncontinues on 2026-10-30 and\nends here.\n"));
        assert!(o.text.contains(&format!(
            "### Undated · behavior-complement\n\nSubject/revision: mod.rs sha256 {digest}.\n"
        )));
        verify(&text, &o.text, &none(), &none()).unwrap();
    }

    #[test]
    fn core_heading_stays_when_its_dated_text_moves() {
        let text = "# x\n\n## invariant-and-cost\n\n#### Invariant\n\nRows are sorted.\n\n#### Cost\n\nTime 63 s on a shared machine (2026-11-04).\n\n#### Run notes\n\nMeasured 2026-11-04.\n";
        let o = undate_text(text, &none(), &none()).unwrap();
        assert!(o.text.contains("#### Cost\n\n## History"), "{}", o.text);
        assert!(o.text.contains("### 2026-11-04 · invariant-and-cost\n\nTime 63 s on a shared machine (2026-11-04).\n\n#### Run notes\n\nMeasured 2026-11-04.\n"));
        verify(text, &o.text, &none(), &none()).unwrap();
    }

    #[test]
    fn rebind_follows_fresh_section_and_chained_json_bindings() {
        let old = "## F\n\nf, 2026-10-30\n\n## G\n\ng".to_string();
        let new = undate_text(&old, &none(), &none()).unwrap().text;
        let mut changed: BTreeMap<String, Vec<String>> = [(
            "a/m.groupant/model.md".to_string(),
            vec![old.clone(), new.clone()],
        )]
        .into();
        let s = |t: &str| sha(t.as_bytes());
        let f_old = s(sections::section_bytes(&old, "F").unwrap());
        let g = s(sections::section_bytes(&old, "G").unwrap());
        let man = format!(
            "{{\"a\":[{{\"path\":\"model.md#F\",\"sha256\":\"{f_old}\"}},{{\"path\":\"model.md#G\",\"sha256\":\"{g}\"}},{{\"path\":\"model.md\",\"sha256\":\"{}\"}}]}}",
            "0".repeat(64)
        );
        let ev = format!(
            "{{\"path\":\"a/m.groupant/manifest.json\",\"sha256\":\"{}\"}}",
            s(&man)
        );
        let files = vec![
            "a/m.groupant/manifest.json".to_string(),
            "ev.json".to_string(),
        ];
        let mut texts: BTreeMap<String, String> =
            [(files[0].clone(), man.clone()), (files[1].clone(), ev)].into();
        let (rebinds, stale) = rebind(&mut changed, &files, &mut texts).unwrap();
        assert_eq!(rebinds.len(), 2, "{rebinds:?}");
        let m = &texts[&files[0]];
        assert!(m.contains(&s(sections::section_bytes(&new, "F").unwrap())) && m.contains(&g));
        assert!(
            texts[&files[1]].contains(&s(m)),
            "chained JSON binding follows"
        );
        assert_eq!(
            stale.len(),
            1,
            "an already stale binding is reported, not refreshed"
        );
    }

    fn keep(line: &str) -> String {
        match strip_line(line, false) {
            Strip::Keep(s) => s,
            other => panic!("{line}: {other:?}"),
        }
    }

    #[test]
    fn strip_line_removes_dates_with_their_frames() {
        let d = "76a320c9ade011864661dd9edcbad7ca6d200346cbdea407d626eaedf024670d";
        for (line, want) in [
            ("Plain text stays.", "Plain text stays."),
            ("Cost is linear (2026-11-04).", "Cost is linear."),
            (
                "Licences (data, not code; 2026-11-05) apply.",
                "Licences (data, not code) apply.",
            ),
            ("Rows (2026-11-05, wave 3) sorted.", "Rows (wave 3) sorted."),
            (
                "Epoch 2026-11-05 (review round 2): a cached copy.",
                "A cached copy.",
            ),
            (
                "2026-11-05, first epoch. Live run.",
                "First epoch. Live run.",
            ),
            (
                "Pass. Epoch 2026-11-05: the gate flipped.",
                "Pass. The gate flipped.",
            ),
            (
                "Manual mutations 2026-11-05: one cell.",
                "Manual mutations: one cell.",
            ),
            ("Measured as of 2026-11-01, 12 rows.", "Measured, 12 rows."),
            (
                "Empty. On 2026-11-04 the tables left.",
                "Empty. The tables left.",
            ),
            ("- b passes, 2026-10-31", "- b passes"),
            ("Review — 2026-11-01", "Review"),
            (
                "Source SHA-256 `76a320c9ade0…`, rows sorted.",
                "Source SHA-256 (see History), rows sorted.",
            ),
            (&format!("Bound to {d}."), "Bound to (see History)."),
            ("Raw in `$MOVA_DATA/runs/x/`.", "Raw in (see History)."),
            (
                "Part file `$MOVA_DATA/release/<r>/<name>.part`.",
                "Part file `$MOVA_DATA/release/<r>/<name>.part`.",
            ),
            ("#### Review epoch 2026-11-01", "#### Review"),
        ] {
            assert_eq!(keep(line), want, "{line}");
        }
        assert_eq!(strip_line("#### Epoch 2026-11-01", false), Strip::Drop);
        assert_eq!(
            strip_line("Date: 2026-10-30. Source `abc1234…`.", false),
            Strip::Drop
        );
        assert_eq!(strip_line("(2026-11-04)", false), Strip::Drop);
        assert_eq!(
            strip_line("Heading `## A 2026-11-01` stays in x as `## A`.", false),
            Strip::Drop
        );
        // A date inside a run key or path: the whole token is provenance.
        assert_eq!(
            keep("root octopus-wave7-2026-10-30/ kept."),
            "root (see History) kept."
        );
        assert_eq!(
            keep("Logs: external x-2026-10-31."),
            "Logs: external (see History)."
        );
        assert_eq!(keep("(2026-11-02 dry run)"), "(dry run)");
        // Negative control: a heading holding provenance is not rewritten.
        assert_eq!(strip_line("### Run 3c03bb63…", false), Strip::Fail);
        // Fenced lines are never inspected.
        assert_eq!(
            strip_line("run 2026-10-30", true),
            Strip::Keep("run 2026-10-30".into())
        );
        let k = keep("Cost is linear (2026-11-04).");
        assert!(epoch_dates(&k).is_empty() && !provenance(&k));
    }

    #[test]
    fn selected_sections_keep_rewritten_substance_and_history_keeps_verbatim() {
        let text = "# x\n\n## checks\n\n`cargo test -p a` passes. Manual mutations 2026-11-05: one row turns red.\n\nDate: 2026-11-05.\n\n#### Review epoch 2026-11-06\n\nLater review.\n\n## notes\n\nMeasured 2026-11-05.\n";
        let sel: BTreeSet<String> = ["checks".to_string()].into();
        let o = undate_text(text, &none(), &sel).unwrap();
        let (main, hist) = o.text.split_once("\n## History\n").unwrap();
        assert!(
            main.contains(
                "## checks\n\n`cargo test -p a` passes. Manual mutations: one row turns red.\n"
            ),
            "{main}"
        );
        assert!(
            !main.contains("Date:") && !main.contains("Review"),
            "{main}"
        );
        assert!(
            main.ends_with("## notes\n") && !main.contains("Measured"),
            "unselected sections move as before"
        );
        assert!(hist.contains("Manual mutations 2026-11-05: one row turns red."));
        assert!(hist.contains("Date: 2026-11-05.") && hist.contains("Measured 2026-11-05."));
        assert_eq!(o.stripped_units, 1);
        verify(text, &o.text, &none(), &sel).unwrap();
        assert_eq!(
            undate_text(&o.text, &none(), &sel).unwrap().text,
            o.text,
            "idempotent"
        );
        // Negative control: the same rewrite in an unselected section is an invented line.
        assert!(verify(text, &o.text, &none(), &none()).is_err());
    }

    const EMPTIED: &str = "# x\n\n## forward\n\n## reverse\n\n## cost\n\n#### Cost\n\n## extra\n\n## History\n\n### 2026-10-30 · forward\n\nOld rule, 2026-10-30.\n\n### 2026-11-05 · forward\n\nNew rule (2026-11-05): rows sorted.\n- item one, 2026-11-05\n\n```sh\nrun 2026-11-05\n```\n\n#### Review epoch 2026-11-05\n\nReviewed; fine.\n\n### Undated · reverse\n\nFrom a stamp at `$MOVA_DATA/<top>.fetched` the SHA-256 is recovered.\n\n### 2026-11-05 · cost\n\nDate: 2026-11-05.\n\n### 2026-11-05 · extra\n\nNot selected, 2026-11-05.\n";

    #[test]
    fn heading_only_selected_sections_are_refilled_from_the_newest_group() {
        let sel: BTreeSet<String> = ["forward", "reverse", "cost"].map(String::from).into();
        let o = undate_text(EMPTIED, &none(), &sel).unwrap();
        let t = &o.text;
        assert!(t.contains("## forward\n\nNew rule: rows sorted.\n- item one\n\n```sh\nrun 2026-11-05\n```\n\n#### Review\n\nReviewed; fine.\n\n## reverse"), "{t}");
        assert!(
            !t.split_once("## reverse").unwrap().0.contains("Old rule"),
            "only the newest group"
        );
        assert!(t.contains("## reverse\n\nFrom a stamp at `$MOVA_DATA/<top>.fetched` the SHA-256 is recovered.\n\n## cost"));
        assert!(
            t.contains("## cost\n\n#### Cost\n\n## extra\n\n## History"),
            "nothing current: residue"
        );
        assert_eq!(o.refilled, ["forward", "reverse"]);
        assert_eq!(o.residue, ["cost"]);
        assert!(
            t.ends_with(EMPTIED.split_once("## History\n").unwrap().1),
            "History unchanged"
        );
        verify(EMPTIED, t, &none(), &sel).unwrap();
        assert_eq!(
            undate_text(t, &none(), &sel).unwrap().text,
            *t,
            "idempotent"
        );
        let invented = t.replace("New rule: rows sorted.", "New rule: rows shuffled.");
        assert!(
            verify(EMPTIED, &invented, &none(), &sel).is_err(),
            "invented refill passes"
        );
        let dated = t.replace(
            "New rule: rows sorted.",
            "New rule (2026-11-05): rows sorted.",
        );
        assert!(
            verify(EMPTIED, &dated, &none(), &sel).is_err(),
            "dated refill passes"
        );
        assert!(
            verify(EMPTIED, t, &none(), &none()).is_err(),
            "refill outside a selected section passes"
        );
    }
}
