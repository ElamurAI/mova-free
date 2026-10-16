//! Treebank references ↔ `papers/` library ↔ ACL Anthology.
//!
//! Matching order for a reference item:
//! 1. Anthology id from a URL (`aclanthology.org/…`, `aclweb.org/anthology/…`) and from an ACL DOI (`10.18653/v1/…`):
//!    first the library (`ids.anthology`), then the ACL bibliography;
//! 2. other DOIs: library, then ACL;
//! 3. bibtex record: title equals the paper title after normalization + year + a shared author surname;
//! 4. free text: the normalized paper title (≥ 3 words, ≥ 20 chars) appears contiguously in the item, with title
//!    boundaries (punctuation or line edge on both sides, not a colon), the paper's year is in the item, and the
//!    surname of at least one author is outside the title.
//!
//! Any ambiguity or incomplete match is "not found", never someone else's paper. Partial matches
//! remain as a hint (`near`) for manual review.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;
use sources::bib;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

use crate::readme::year_of;

/// A normalized word with its boundaries in the raw text (bytes).
#[derive(Debug, Clone)]
pub struct Tok {
    pub w: String,
    pub start: usize,
    pub end: usize,
}

fn fold_into(c: char, out: &mut String) {
    for l in c.to_lowercase() {
        match l {
            'ł' => out.push('l'),
            'ø' => out.push('o'),
            'đ' | 'ð' => out.push('d'),
            'þ' => out.push_str("th"),
            'ß' => out.push_str("ss"),
            'æ' => out.push_str("ae"),
            'œ' => out.push_str("oe"),
            'ı' => out.push('i'),
            _ => {
                for d in std::iter::once(l).nfd() {
                    if !is_combining_mark(d) {
                        out.push(d);
                    }
                }
            }
        }
    }
}

/// Words of a text: letters and digits of any script, lowercase, without diacritics; everything else is a boundary.
pub fn toks(s: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut start = 0;
    for (i, c) in s.char_indices() {
        if c.is_alphanumeric() {
            if cur.is_empty() {
                start = i;
            }
            fold_into(c, &mut cur);
        } else if (is_combining_mark(c) || c == '{' || c == '}') && !cur.is_empty() {
            // diacritics as a separate char (NFD) and BibTeX braces ({F}risian) stay inside the word
        } else if !cur.is_empty() {
            out.push(Tok { w: std::mem::take(&mut cur), start, end: i });
        }
    }
    if !cur.is_empty() {
        out.push(Tok { w: cur, start, end: s.len() });
    }
    out
}

pub fn words(s: &str) -> Vec<String> {
    toks(s).into_iter().map(|t| t.w).collect()
}

/// Anthology id: old `W17-6514`, new `2020.lrec-1.497`, `2024.lrec-main.1471`.
pub fn is_acl_id(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() == 8 && b[0].is_ascii_uppercase() && b[1..3].iter().all(u8::is_ascii_digit) && b[3] == b'-' && b[4..].iter().all(u8::is_ascii_digit) {
        return true;
    }
    let p: Vec<&str> = s.split('.').collect();
    p.len() == 3
        && p[0].len() == 4
        && p[0].bytes().all(|c| c.is_ascii_digit())
        && p[1].contains('-')
        && !p[1].starts_with('-')
        && !p[1].ends_with('-')
        && p[1].bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
        && !p[2].is_empty()
        && p[2].bytes().all(|c| c.is_ascii_digit())
}

const URL_STOP: &str = " \t\n)]}>\"'<,;|`*";

/// Anthology ids from URLs in the text.
pub fn acl_ids(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let low = s.to_lowercase();
    for pre in ["aclanthology.org/", "aclweb.org/anthology/", "aclanthology.info/papers/", "aclanthology.coli.uni-saarland.de/papers/"] {
        let mut from = 0;
        while let Some(p) = low[from..].find(pre) {
            let a = from + p + pre.len();
            let b = s[a..].find(|c: char| URL_STOP.contains(c)).map_or(s.len(), |x| a + x);
            for seg in s[a..b].split('/') {
                let seg = seg.trim_end_matches(".pdf").trim_end_matches(".bib").trim_end_matches(".xml").trim_end_matches('.');
                if is_acl_id(seg) && !out.iter().any(|x| x == seg) {
                    out.push(seg.to_string());
                }
            }
            from = b.max(a);
        }
    }
    out
}

/// DOIs in the text (lowercase; without trailing punctuation and an unclosed parenthesis).
pub fn dois(s: &str, re: &Regex) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for m in re.find_iter(s) {
        let mut d = m.as_str().to_string();
        loop {
            let before = d.len();
            d = d.trim_end_matches(['.', ',', ';', ':']).to_string();
            if d.ends_with(')') && d.matches('(').count() < d.matches(')').count() {
                d.pop();
            }
            if d.len() == before {
                break;
            }
        }
        let d = d.to_lowercase();
        if !out.contains(&d) {
            out.push(d);
        }
    }
    out
}

pub fn doi_regex() -> Regex {
    Regex::new(r#"10\.\d{4,9}/[^\s"<>{}\[\]|\\^`']+"#).expect("regex DOI")
}

/// DOI ACL (`10.18653/v1/2020.lrec-1.497`) → id.
pub fn acl_id_of_doi(doi: &str) -> Option<String> {
    let rest = doi.strip_prefix("10.18653/v1/")?;
    let rest = rest.trim_end_matches(".pdf");
    // DOIs are lowercase, while old ids start with a capital letter
    let id = if rest.len() == 8 && rest.as_bytes()[0].is_ascii_lowercase() { format!("{}{}", rest[..1].to_uppercase(), &rest[1..]) } else { rest.to_string() };
    is_acl_id(&id).then_some(id)
}

const PARTICLES: &[&str] = &["de", "del", "della", "der", "den", "van", "von", "la", "le", "du", "da", "dos", "das", "di", "ten", "ter", "el", "al", "bin", "ibn", "mac", "st", "y"];

/// Surname → normalized words without particles (`de Marneffe` → `marneffe`).
fn surname_words(s: &str) -> Vec<String> {
    words(s).into_iter().filter(|w| w.chars().count() >= 2 && !PARTICLES.contains(&w.as_str())).collect()
}

/// A paper in the index (library or ACL).
#[derive(Debug, Clone, Default)]
pub struct Paper {
    pub title: String,
    pub year: u32,
    pub authors: Vec<String>,
    pub surnames: Vec<Vec<String>>,
    pub anthology: String,
    pub doi: String,
    /// folder in the library `<a>/<ab>/<paper>` (empty for ACL)
    pub folder: String,
    pub words: Vec<String>,
}

/// A title variant for search: the paper and how many leading words are skipped (0 — full title; 1 — without
/// the leading article: "The Stanford Typed Dependencies Representation" is also cited without "The").
type Variant = (usize, usize);

#[derive(Default)]
pub struct Index {
    pub papers: Vec<Paper>,
    pub by_id: HashMap<String, usize>,
    pub by_doi: HashMap<String, usize>,
    by_title: HashMap<String, Vec<Variant>>,
    by_tri: HashMap<String, Vec<Variant>>,
}

/// The title is suitable for search inside free text.
fn long_enough(w: &[String]) -> bool {
    w.len() >= 3 && w.join(" ").chars().count() >= 20
}

/// Words with letters (no numbers or versions): to allow "year ±1" the title must be long.
fn alpha_words(w: &[String]) -> usize {
    w.iter().filter(|x| x.chars().any(char::is_alphabetic)).count()
}

impl Index {
    pub fn new(papers: Vec<Paper>) -> Index {
        let mut ix = Index::default();
        for (i, p) in papers.iter().enumerate() {
            if !p.anthology.is_empty() {
                ix.by_id.entry(p.anthology.clone()).or_insert(i);
            }
            if !p.doi.is_empty() {
                ix.by_doi.entry(p.doi.clone()).or_insert(i);
            }
            let article = p.words.first().is_some_and(|w| matches!(w.as_str(), "the" | "a" | "an"));
            for skip in [0, 1] {
                if skip == 1 && !(article && p.words.len() >= 5) {
                    continue;
                }
                let w = &p.words[skip..];
                if w.len() >= 2 {
                    ix.by_title.entry(w.join(" ")).or_default().push((i, skip));
                }
                if long_enough(w) {
                    ix.by_tri.entry(w[..3].join(" ")).or_default().push((i, skip));
                }
            }
        }
        ix.papers = papers;
        ix
    }

    /// ACL bibliography: only records with an Anthology id.
    pub fn acl(entries: &[bib::Entry]) -> Index {
        let papers = entries
            .iter()
            .filter(|e| !e.anthology_id().is_empty())
            .map(|e| {
                let raw: Vec<&str> = e.get("author").split(" and ").map(str::trim).filter(|a| !a.is_empty()).collect();
                let authors = raw.iter().map(|a| a.split_once(", ").map_or(a.to_string(), |(l, f)| format!("{f} {l}"))).collect();
                let surnames = raw.iter().map(|a| surname_words(a.split_once(',').map_or_else(|| a.rsplit(' ').next().unwrap_or(a), |(l, _)| l))).collect();
                Paper {
                    title: detex(e.get("title")),
                    year: e.get("year").parse().unwrap_or(0),
                    authors,
                    surnames,
                    anthology: e.anthology_id().to_string(),
                    doi: e.get("doi").to_lowercase(),
                    folder: String::new(),
                    words: words(&detex(e.get("title"))),
                }
            })
            .collect();
        Index::new(papers)
    }

    /// Paper library: `<dir>/<a>/<ab>/<paper>/meta.yaml`, for folders with `SPLIT` — `<dir>/<a>/<ab>/<abc>/<paper>`.
    pub fn library(dir: &Path) -> Result<Index> {
        #[derive(Deserialize, Default)]
        struct Ids {
            #[serde(default)]
            anthology: Option<String>,
            #[serde(default)]
            doi: Option<String>,
        }
        #[derive(Deserialize)]
        struct M {
            #[serde(default)]
            title: String,
            #[serde(default)]
            authors: Vec<String>,
            #[serde(default)]
            year: u32,
            #[serde(default)]
            ids: Ids,
        }
        let mut papers = Vec::new();
        let mut dirs: Vec<std::path::PathBuf> = Vec::new();
        // a/ab/<paper> or a/ab/abc/<paper> (folders with papers/SPLIT have a third level)
        let mut stack: Vec<(std::path::PathBuf, usize)> = vec![(dir.to_path_buf(), 0)];
        while let Some((d, depth)) = stack.pop() {
            for e in fs::read_dir(&d).with_context(|| d.display().to_string())?.flatten().filter(|e| e.path().is_dir()) {
                let p = e.path();
                if p.join("meta.yaml").is_file() {
                    dirs.push(p);
                } else if depth < 3 && p.file_name().is_some_and(|n| n != "img") {
                    stack.push((p, depth + 1));
                }
            }
        }
        dirs.sort();
        for p in dirs {
            let f = p.join("meta.yaml");
            let m: M = serde_yaml::from_str(&fs::read_to_string(&f)?).with_context(|| f.display().to_string())?;
            let surnames = m.authors.iter().map(|a| surname_words(a.split_once(',').map_or_else(|| a.rsplit(' ').next().unwrap_or(a), |(l, _)| l))).collect();
            papers.push(Paper {
                words: words(&m.title),
                title: m.title,
                year: m.year,
                authors: m.authors,
                surnames,
                anthology: m.ids.anthology.unwrap_or_default(),
                doi: m.ids.doi.unwrap_or_default().to_lowercase(),
                folder: p.strip_prefix(dir).unwrap_or(&p).to_string_lossy().into_owned(),
            });
        }
        Ok(Index::new(papers))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Src {
    Lib,
    Acl,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum How {
    Url,
    Doi,
    Bibtex,
    Title,
}

impl How {
    pub fn label(self) -> &'static str {
        match self {
            How::Url => "by URL",
            How::Doi => "by DOI",
            How::Bibtex => "by bibtex",
            How::Title => "by title",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub src: Src,
    pub idx: usize,
    pub how: How,
    /// the year differs by 1 (long title, author matched) — shown with a mark
    pub fuzzy_year: bool,
    /// ACL ids of other records with the same title, year and author (duplicate records)
    pub alts: Vec<String>,
}

/// A partial title match without confirmation — a hint for review.
#[derive(Debug, Clone)]
pub struct Near {
    pub src: Src,
    pub idx: usize,
    pub why: &'static str,
}

#[derive(Default, Debug)]
pub struct Outcome {
    pub hits: Vec<Hit>,
    pub acl_ids: Vec<String>,
    pub dois: Vec<String>,
    pub unknown_ids: Vec<String>,
    pub near: Vec<Near>,
}

pub struct Matcher<'a> {
    pub lib: &'a Index,
    pub acl: &'a Index,
    pub doi_re: Regex,
}

/// Before the title: punctuation or edge. A colon is handled separately (`colon_ok`): only "Authors: Title" or "2010: Title".
const BEFORE: &str = ".,;\"“”„‘’'*_([{«‹>|)]";
/// After the title: punctuation or edge; a parenthesis too ("Title (Author et al., ACL 2022)"), but not a colon —
/// otherwise a short title would catch the start of a longer "Title: subtitle".
const AFTER: &str = ".,;\"“”‘’'*_()]}»›<|?!";

/// After the title: punctuation or edge, and without punctuation only "In Proc…", "in Proceeding…", "In: "
/// (start of the venue name).
pub fn after_ok(raw: &str, end: usize) -> bool {
    let rest = raw[end..].trim_start();
    match rest.chars().next() {
        None => true,
        Some(c) if AFTER.contains(c) => true,
        _ => ["In Proc", "in Proc", "In: ", "In the Proc", "in the Proc"].iter().any(|p| rest.starts_with(p)),
    }
}

fn prev_char(raw: &str, start: usize) -> Option<(usize, char)> {
    raw[..start].char_indices().rev().find(|(_, c)| !c.is_whitespace())
}

/// Before the title: punctuation or start of text.
pub fn before_ok(raw: &str, start: usize) -> bool {
    prev_char(raw, start).is_none_or(|(_, c)| BEFORE.contains(c))
}

/// The title ends a line and the next line is the paper's authors ("Title⏎Ryan McDonald, Joakim Nivre…⏎Proceedings").
/// A plain line break is not a boundary: breaks happen in the middle of a longer title.
pub fn line_then_authors(text: &str, end: usize, p: &Paper) -> bool {
    let Some(next) = text[end..].trim_start_matches([' ', '\t']).strip_prefix('\n') else { return false };
    let line: HashSet<String> = words(next.lines().next().unwrap_or("")).into_iter().collect();
    p.surnames.iter().any(|s| s.iter().any(|x| line.contains(x)))
}

/// A colon before the title is accepted only when it is preceded by the end of the paper's author list or a year:
/// "…, Albana Deda: Title", "Rushiti, R.: Title", "Csirik, János 2010: Title", but not "Title: Subtitle".
pub fn colon_ok(raw: &str, start: usize, p: &Paper) -> bool {
    let Some((colon, ':')) = prev_char(raw, start) else { return false };
    let t = toks(&raw[..colon]);
    t.iter().rev().take(2).any(|x| year_of(&x.w).is_some() || p.surnames.iter().any(|s| s.contains(&x.w)))
}

/// Text LaTeX macros that `bib::clean` does not expand (`{\guillemotleft}` → «).
pub fn detex(s: &str) -> String {
    if !s.contains('\\') {
        return s.to_string();
    }
    let mut out = s.to_string();
    for (m, u) in [
        ("\\guillemotleft", "«"),
        ("\\guillemotright", "»"),
        ("\\textquotedblleft", "“"),
        ("\\textquotedblright", "”"),
        ("\\textquoteleft", "‘"),
        ("\\textquoteright", "’"),
        ("\\textendash", "–"),
        ("\\textemdash", "—"),
        ("\\emph", ""),
        ("\\textit", ""),
        ("\\textbf", ""),
        ("\\textsc", ""),
        ("\\texttt", ""),
        ("\\mbox", ""),
    ] {
        out = out.replace(m, u);
    }
    out
}

impl<'a> Matcher<'a> {
    pub fn new(lib: &'a Index, acl: &'a Index) -> Matcher<'a> {
        Matcher { lib, acl, doi_re: doi_regex() }
    }

    fn ix(&self, s: Src) -> &Index {
        match s {
            Src::Lib => self.lib,
            Src::Acl => self.acl,
        }
    }

    pub fn paper(&self, s: Src, i: usize) -> &Paper {
        &self.ix(s).papers[i]
    }

    /// ACL paper → library paper, if it is already in the library.
    fn lift(&self, src: Src, idx: usize) -> (Src, usize) {
        if src == Src::Acl {
            if let Some(&j) = self.lib.by_id.get(&self.acl.papers[idx].anthology) {
                return (Src::Lib, j);
            }
        }
        (src, idx)
    }

    fn by_id(&self, id: &str) -> Option<(Src, usize)> {
        if let Some(&j) = self.lib.by_id.get(id) {
            return Some((Src::Lib, j));
        }
        self.acl.by_id.get(id).map(|&j| (Src::Acl, j))
    }

    fn by_doi(&self, doi: &str) -> Option<(Src, usize)> {
        if let Some(&j) = self.lib.by_doi.get(doi) {
            return Some((Src::Lib, j));
        }
        self.acl.by_doi.get(doi).map(|&j| self.lift(Src::Acl, j))
    }

    /// How many of the paper's authors have their surname among the words of `hay`.
    fn author_hits(p: &Paper, hay: &HashSet<&str>) -> usize {
        p.surnames.iter().filter(|s| s.iter().any(|w| hay.contains(w.as_str()))).count()
    }

    /// Best candidate: exact year, then more authors in the text, then the first in index order.
    /// The rest with the same score are duplicate records (ACL ids).
    fn best(ix: &Index, cands: &[(usize, usize, bool)]) -> (usize, bool, Vec<String>) {
        let score = |c: &(usize, usize, bool)| (!c.2, c.1);
        let top = cands.iter().map(score).max().unwrap_or((false, 0));
        let mut best: Vec<(usize, bool)> = cands.iter().filter(|c| score(c) == top).map(|c| (c.0, c.2)).collect();
        // among equals, the record with an ACL id comes first (the library may also hold an arXiv copy of the same paper)
        best.sort_by_key(|&(i, f)| (ix.papers[i].anthology.is_empty(), i, f));
        best.dedup();
        let (first, fuzzy) = best.remove(0);
        let alts = best.into_iter().map(|(i, _)| if ix.papers[i].anthology.is_empty() { ix.papers[i].folder.clone() } else { ix.papers[i].anthology.clone() }).filter(|a| !a.is_empty()).collect();
        (first, fuzzy, alts)
    }

    /// bibtex: title = paper title (or the same without the leading article), year (if present; for a title of ≥ 4 words —
    /// ±1, with a mark), a shared surname.
    fn bibtex(&self, e: &bib::Entry, src: Src) -> Option<Hit> {
        let ix = self.ix(src);
        let w = words(&detex(e.get("title")));
        let cands = ix.by_title.get(&w.join(" "))?;
        let year: Option<u32> = e.get("year").trim().parse().ok();
        let author_words: Vec<String> = words(e.get("author"));
        let hay: HashSet<&str> = author_words.iter().map(String::as_str).collect();
        let ok: Vec<(usize, usize, bool)> = cands
            .iter()
            .filter_map(|&(i, _)| {
                let p = &ix.papers[i];
                let fuzzy = match year {
                    None => false,
                    Some(y) if p.year == y => false,
                    Some(y) if p.year.abs_diff(y) == 1 && alpha_words(&w) >= 4 => true,
                    _ => return None,
                };
                let a = Self::author_hits(p, &hay);
                (a > 0 || (hay.is_empty() && year.is_some() && !fuzzy && long_enough(&p.words))).then_some((i, a, fuzzy))
            })
            .collect();
        if ok.is_empty() {
            return None;
        }
        let (idx, fuzzy_year, alts) = Self::best(ix, &ok);
        Some(Hit { src, idx, how: How::Bibtex, fuzzy_year, alts })
    }

    /// Titles of index papers inside free text. `used` — word spans already taken.
    fn contained(&self, src: Src, raw: &str, t: &[Tok], used: &mut Vec<(usize, usize)>, near: &mut Vec<Near>) -> Vec<Hit> {
        let ix = self.ix(src);
        let years: HashSet<u32> = t.iter().filter_map(|x| year_of(&x.w)).collect();
        // (span) → [(paper, authors in text, year ±1)]
        let mut spans: Vec<((usize, usize), Vec<(usize, usize, bool)>)> = Vec::new();
        for i in 0..t.len().saturating_sub(2) {
            let key = format!("{} {} {}", t[i].w, t[i + 1].w, t[i + 2].w);
            let Some(cands) = ix.by_tri.get(&key) else { continue };
            for &(c, skip) in cands {
                let p = &ix.papers[c];
                let w = &p.words[skip..];
                let l = w.len();
                if i + l > t.len() || t[i..i + l].iter().zip(w).any(|(a, b)| a.w != *b) {
                    continue;
                }
                let (s0, e0) = (t[i].start, t[i + l - 1].end);
                let b_ok = (before_ok(raw, s0) || colon_ok(raw, s0, p)) && (after_ok(raw, e0) || line_then_authors(raw, e0, p));
                let y_exact = years.contains(&p.year);
                // year ±1 (TLT16 volume — 2017, conference — January 2018) only for long titles, with a mark
                let y_fuzzy = !y_exact && alpha_words(w) >= 6 && years.iter().any(|y| y.abs_diff(p.year) == 1);
                let hay: HashSet<&str> = t.iter().enumerate().filter(|(k, _)| *k < i || *k >= i + l).map(|(_, x)| x.w.as_str()).collect();
                let a = Self::author_hits(p, &hay);
                // a hint only when exactly one check failed ("near match")
                match (b_ok, y_exact || y_fuzzy, a > 0) {
                    (true, true, true) => {}
                    (false, true, true) => {
                        near.push(Near { src, idx: c, why: "title boundaries" });
                        continue;
                    }
                    (true, false, true) => {
                        near.push(Near { src, idx: c, why: "year" });
                        continue;
                    }
                    (true, true, false) => {
                        near.push(Near { src, idx: c, why: "authors" });
                        continue;
                    }
                    _ => continue,
                }
                match spans.iter_mut().find(|s| s.0 == (i, i + l)) {
                    Some(s) => s.1.push((c, a, !y_exact)),
                    None => spans.push(((i, i + l), vec![(c, a, !y_exact)])),
                }
            }
        }
        // longer titles first; spans do not overlap
        spans.sort_by_key(|s| (std::cmp::Reverse(s.0.1 - s.0.0), s.0.0));
        let mut out = Vec::new();
        for (span, cands) in spans {
            if used.iter().any(|u| span.0 < u.1 && u.0 < span.1) {
                continue;
            }
            used.push(span);
            let (idx, fuzzy_year, alts) = Self::best(ix, &cands);
            out.push(Hit { src, idx, how: How::Title, fuzzy_year, alts });
        }
        out
    }

    /// Matching of one reference item.
    pub fn item(&self, raw: &str, bib_entry: Option<&bib::Entry>) -> Outcome {
        let mut o = Outcome::default();
        let push = |o: &mut Outcome, (src, idx): (Src, usize), how: How, fuzzy_year: bool, alts: Vec<String>| {
            if !o.hits.iter().any(|h| h.src == src && h.idx == idx) {
                o.hits.push(Hit { src, idx, how, fuzzy_year, alts });
            }
        };
        // 1–2. ids and DOIs
        let url_ids = acl_ids(raw);
        let mut ids = url_ids.clone();
        let all_dois = dois(raw, &self.doi_re);
        for d in &all_dois {
            if let Some(id) = acl_id_of_doi(d) {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        for id in &ids {
            let how = if url_ids.contains(id) { How::Url } else { How::Doi };
            match self.by_id(id) {
                Some(t) => push(&mut o, t, how, false, Vec::new()),
                None => o.unknown_ids.push(id.clone()),
            }
        }
        for d in all_dois.iter().filter(|d| acl_id_of_doi(d).is_none()) {
            if let Some(t) = self.by_doi(d) {
                push(&mut o, t, How::Doi, false, Vec::new());
            }
            o.dois.push(d.clone());
        }
        o.acl_ids = ids;
        // 3. bibtex
        if let Some(e) = bib_entry {
            for src in [Src::Lib, Src::Acl] {
                if let Some(h) = self.bibtex(e, src) {
                    let t = self.lift(h.src, h.idx);
                    push(&mut o, t, How::Bibtex, h.fuzzy_year, h.alts);
                    break;
                }
            }
        }
        // 4. titles in free text: library, then ACL on the rest of the text
        let t = toks(raw);
        let mut used = Vec::new();
        let mut near = Vec::new();
        for src in [Src::Lib, Src::Acl] {
            for h in self.contained(src, raw, &t, &mut used, &mut near) {
                let t = self.lift(h.src, h.idx);
                push(&mut o, t, How::Title, h.fuzzy_year, h.alts);
            }
        }
        // hints: only those that did not become a match
        for n in near {
            let (s, i) = self.lift(n.src, n.idx);
            if !o.hits.iter().any(|h| h.src == s && h.idx == i) && !o.near.iter().any(|x| x.src == s && x.idx == i) {
                o.near.push(Near { src: s, idx: i, why: n.why });
            }
        }
        o
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn paper(title: &str, year: u32, authors: &[&str], id: &str) -> Paper {
        Paper {
            title: title.into(),
            year,
            authors: authors.iter().map(|a| a.to_string()).collect(),
            surnames: authors.iter().map(|a| surname_words(a.rsplit(' ').next().unwrap())).collect(),
            anthology: id.into(),
            doi: String::new(),
            folder: String::new(),
            words: words(title),
        }
    }

    pub(super) fn acl() -> Index {
        Index::new(vec![
            paper("Universal Dependencies v1: A Multilingual Treebank Collection", 2016, &["Joakim Nivre", "Marie-Catherine de Marneffe"], "L16-1262"),
            paper("A Multilingual Treebank Collection", 2016, &["Joakim Nivre"], "X16-0001"),
            paper("Universal Dependencies", 2021, &["Marie-Catherine de Marneffe", "Joakim Nivre"], "2021.cl-2.11"),
            paper("The Prague Dependency Treebank", 2006, &["Jan Hajič"], "L06-9999"),
            paper("Parsing Universal Dependencies without training", 2017, &["Héctor Martínez Alonso"], "E17-1022"),
            paper("TwittIrish: A Universal Dependencies Treebank of Tweets in Modern Irish", 2022, &["Lauren Cassidy", "Teresa Lynn"], "2022.acl-long.473"),
            paper("Hungarian Dependency Treebank", 2010, &["Veronika Vincze", "János Csirik"], "L10-1321"),
            paper("Semgrex-Plus: a Tool for Automatic Dependency-Graph Rewriting", 2017, &["Fabio Tamburini"], "W17-6528"),
            paper("Challenges in Annotating and Parsing Spoken, Code-switched, Frisian-Dutch Data", 2021, &["Anouck Braggaar", "Rob van der Goot"], "2021.adaptnlp-1.6"),
            paper("UD Annotatrix: An annotation tool for Universal Dependencies", 2017, &["Francis M. Tyers", "Mariya Sheyanova"], "W17-7630"),
            paper("The Stanford Typed Dependencies Representation", 2008, &["Marie-Catherine de Marneffe", "Christopher D. Manning"], "W08-1301"),
            paper("Challenges in Converting the Index Thomisticus Treebank into Universal Dependencies", 2018, &["Flavio Massimiliano Cecchini", "Marco Passarotti"], "W18-6004"),
            paper("Universal Dependency Annotation for Multilingual Parsing", 2013, &["Ryan McDonald", "Joakim Nivre"], "P13-2017"),
        ])
    }

    #[test]
    fn normalization_and_ids() {
        assert_eq!(words("Hajič, J. «Česká» Straße—Łódź"), vec!["hajic", "j", "ceska", "strasse", "lodz"]);
        assert_eq!(words("Hajic\u{30C}ek"), vec!["hajicek"], "a separate diacritic char does not split the word");
        assert_eq!(
            acl_ids("see https://aclanthology.org/2020.lrec-1.497/ and [x](https://www.aclweb.org/anthology/W17-0401.pdf), http://aclweb.org/anthology/P/P13/P13-2017.pdf, https://aclanthology.org/2024.lrec-main.1471}"),
            vec!["2020.lrec-1.497", "2024.lrec-main.1471", "W17-0401", "P13-2017"]
        );
        // negative controls: a volume, not a paper; looks similar but is not an id
        assert!(acl_ids("https://aclanthology.org/volumes/2020.udw-1/ https://www.aclweb.org/anthology/W/W13/W13-37.pdf").is_empty());
        assert!(!is_acl_id("2020.lrec.497") && !is_acl_id("W17-040") && !is_acl_id("w17-0401"));
        let re = doi_regex();
        assert_eq!(dois("doi: 10.18653/v1/2020.acl-main.107. And (https://doi.org/10.1016/S0000(00)00000-0).", &re), vec!["10.18653/v1/2020.acl-main.107", "10.1016/s0000(00)00000-0"]);
        assert_eq!(acl_id_of_doi("10.18653/v1/w17-0401"), Some("W17-0401".into()));
        assert_eq!(acl_id_of_doi("10.18653/v1/2020.acl-main.107"), Some("2020.acl-main.107".into()));
        assert_eq!(acl_id_of_doi("10.1007/s10579-020-09492-z"), None);
    }

    fn one(m: &Matcher, raw: &str) -> Vec<String> {
        m.item(raw, None).hits.iter().map(|h| m.paper(h.src, h.idx).anthology.clone()).collect()
    }

    #[test]
    fn title_matching_positive() {
        let (lib, acl) = (Index::default(), acl());
        let m = Matcher::new(&lib, &acl);
        assert_eq!(one(&m, "Nivre, J., de Marneffe, M.-C., et al. (2016). Universal Dependencies v1: A Multilingual Treebank Collection. In Proceedings of LREC."), vec!["L16-1262"]);
        assert_eq!(one(&m, "Joakim Nivre et al. 2016. [Universal dependencies v1: a multilingual treebank collection](http://x). LREC."), vec!["L16-1262"]);
        assert_eq!(one(&m, "Martínez Alonso, H. 2017. “Parsing Universal Dependencies without Training.” EACL."), vec!["E17-1022"]);
        // "Title (Author et al., ACL 2022)", "Authors 2010: Title", "Title In Proc.", {F}risian in free text
        assert_eq!(one(&m, "TwittIrish: A Universal Dependencies Treebank of Tweets in Modern Irish (Cassidy et al., ACL 2022)"), vec!["2022.acl-long.473"]);
        assert_eq!(one(&m, "Vincze, Veronika; Csirik, János 2010: Hungarian Dependency Treebank. In: Proceedings of LREC"), vec!["L10-1321"]);
        assert_eq!(one(&m, "Tamburini F. (2017). Semgrex-Plus: a Tool for Automatic Dependency-Graph Rewriting In Proc. of Depling"), vec!["W17-6528"]);
        assert_eq!(one(&m, "Braggaar and Van der Goot (2021). Challenges in Annotating and Parsing Spoken, Code-switched, {F}risian-{D}utch Data. AdaptNLP"), vec!["2021.adaptnlp-1.6"]);
    }

    #[test]
    fn title_matching_negative_controls() {
        let (lib, acl) = (Index::default(), acl());
        let m = Matcher::new(&lib, &acl);
        // a different year
        let o = m.item("Nivre, J. (2018). Universal Dependencies v1: A Multilingual Treebank Collection. LREC.", None);
        assert!(o.hits.is_empty() && o.near.iter().any(|n| n.why == "year"));
        // none of the paper's authors present
        let o = m.item("Smith, J. (2016). Universal Dependencies v1: A Multilingual Treebank Collection. LREC.", None);
        assert!(o.hits.is_empty() && o.near.iter().any(|n| n.why == "authors"));
        // the short title "Universal Dependencies" is not searched in free text
        assert!(one(&m, "de Marneffe, M.-C., Nivre, J. (2021). Universal Dependencies. Computational Linguistics.").is_empty());
        // a short foreign title as the start of a longer one: "The Prague Dependency Treebank" ≠ "… Treebank 3.0"
        assert!(one(&m, "Hajič, J. et al. (2006). The Prague Dependency Treebank 3.0. LDC.").is_empty());
        // a subtitle after a colon is not a separate paper (the longer title wins, the short one is not added)
        assert_eq!(one(&m, "Nivre, J. (2016). Universal Dependencies v1: A Multilingual Treebank Collection. LREC."), vec!["L16-1262"]);
        assert!(one(&m, "Nivre, J. (2016). Something else: A Multilingual Treebank Collection. LREC.").is_empty());
        // a colon after a title word, not an author; a title without a boundary after it
        assert!(one(&m, "Csirik 2010. Parsing and the Hungarian Dependency Treebank project, notes").is_empty());
        assert!(one(&m, "Vincze 2010: Hungarian Dependency Treebank tools are described here").is_empty());
        // a title inside a sentence without punctuation
        assert!(one(&m, "In 2017 Martínez Alonso wrote that parsing universal dependencies without training is hard").is_empty());
    }

    #[test]
    fn ids_and_bibtex() {
        let lib = Index::new(vec![Paper { folder: "n/ni/nivre-2016".into(), ..paper("Universal Dependencies v1: A Multilingual Treebank Collection", 2016, &["Joakim Nivre"], "L16-1262") }]);
        let acl = acl();
        let m = Matcher::new(&lib, &acl);
        // id from URL → library, not ACL
        let o = m.item("See https://aclanthology.org/L16-1262/", None);
        assert_eq!((o.hits[0].src, o.hits[0].how), (Src::Lib, How::Url));
        // an id that exists nowhere is reported separately
        let o = m.item("https://aclanthology.org/2099.xx-1.1", None);
        assert!(o.hits.is_empty() && o.unknown_ids == vec!["2099.xx-1.1".to_string()]);
        // bibtex: exact title + year + author; with a different year — no
        let e = &bib::parse("@inproceedings{k, title = {{P}arsing {U}niversal {D}ependencies without Training}, author = {Mart{\\'i}nez Alonso, H{\\'e}ctor}, year = {2017}}")[0];
        let o = m.item("", Some(e));
        assert_eq!(o.hits.len(), 1);
        assert_eq!(m.paper(o.hits[0].src, o.hits[0].idx).anthology, "E17-1022");
        let e2 = &bib::parse("@inproceedings{k, title = {Parsing Universal Dependencies without Training}, author = {Martinez Alonso, Hector}, year = {2019}}")[0];
        assert!(m.item("", Some(e2)).hits.is_empty());
    }
}

#[cfg(test)]
mod year_tests {
    use super::tests::acl;
    use super::*;

    #[test]
    fn fuzzy_year_article_and_detex() {
        let (lib, acl) = (Index::default(), acl());
        let m = Matcher::new(&lib, &acl);
        // year ±1 for a long title is a match with a mark
        let o = m.item("Tyers, F. M., Sheyanova, M. (2018). UD Annotatrix: An annotation tool for Universal Dependencies. In Proceedings of TLT16.", None);
        assert_eq!(o.hits.len(), 1);
        assert!(o.hits[0].fuzzy_year);
        // negative controls: ±2 — no; short title ±1 — no
        assert!(m.item("Tyers, F. M. (2019). UD Annotatrix: An annotation tool for Universal Dependencies. TLT.", None).hits.is_empty());
        assert!(m.item("Vincze, Veronika; Csirik, János 2011: Hungarian Dependency Treebank. LREC.", None).hits.is_empty());
        // a title with an article is cited without it
        let o = m.item("De Marneffe, Marie-Catherine, and Christopher D. Manning. 2008. Stanford Typed Dependencies Representation. In Proceedings of the COLING'08 Workshop.", None);
        assert_eq!(o.hits.len(), 1);
        assert!(!o.hits[0].fuzzy_year);
        // \emph{…} in a bibtex title
        let e = &bib::parse("@article{lait, author = {Cecchini, Flavio Massimiliano and Passarotti, Marco}, title = {Challenges in Converting the \\emph{Index Thomisticus} treebank into Universal Dependencies}, year = {2018}}")[0];
        let o = m.item("", Some(e));
        assert_eq!(o.hits.len(), 1, "{}", detex(e.get("title")));
    }
}

#[cfg(test)]
mod line_tests {
    use super::tests::acl;
    use super::*;

    #[test]
    fn title_line_then_authors_line() {
        let (lib, acl) = (Index::default(), acl());
        let m = Matcher::new(&lib, &acl);
        let o = m.item("Universal Dependency Annotation for Multilingual Parsing\nRyan McDonald, Joakim Nivre, Yoav Goldberg\nProceedings of ACL 2013", None);
        assert_eq!(o.hits.len(), 1);
        // negative controls: a line break in the middle of a longer title (the next line is not authors); the same text on one line
        assert!(m.item("Hajič, J. (2006). The Prague Dependency Treebank\n3.0 released by LDC.", None).hits.is_empty());
        assert!(m.item("Universal Dependency Annotation for Multilingual Parsing Ryan McDonald, Joakim Nivre, Proceedings of ACL 2013", None).hits.is_empty());
    }
}
