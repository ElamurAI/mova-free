//! The site's Markdown as a corpus: the Markdown twin of mova.elamur.ai (`free/en/**.md`, read only). A page is a
//! chapter; a block is a page or a run of sibling pages up to `MAX_WORDS` words of prose, cut only at page
//! boundaries. The prose — paragraphs, list items, table cells, headings, link texts — goes up the layers as English
//! text. The markup is layout: front matter, `<!-- … -->` markers, heading and list marks, table pipes, link
//! targets, emphasis, escapes and code fences are literals (`Class::Markup`; inline marks as `vol::Block::marks`). A
//! page's ```` ```conllu ```` block and the frames table under it are the tree and the frames of the sentence above
//! them: they are pieces the site decoder rebuilds from the layers (`site_code`), and only what the rebuild misses
//! is stored. A paragraph a marker gives another language (`lang=hbo`, `dir=rtl`) is kept whole (`Class::Unparsed`).
//!
//! Sampling and split: pages are listed per section (`corpora/torah`, `corpora/tales`, `docs`, …), cut into blocks in
//! the order of their paths (numbers compared as numbers), every N-th block of a section is taken, and a block goes
//! to train, dev or test by the hash of its first page (`split_of`: test 1 in 10, dev 1 in 10).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::{Block, Chapter, MAX_WORDS};
use crate::class::Class;
use crate::vol::{Block as VBlock, Piece, Volume};

/// The split of a block, by the hash of its first page: test 1 in 10, dev 1 in 10, train the rest. Fixed: the
/// held-out pages never move.
pub fn split_of(first_page: &str) -> &'static str {
    match langseg::fnv(first_page) % 10 {
        0 => "test",
        1 => "dev",
        _ => "train",
    }
}

/// A page's section: `corpora/<x>` and `dictionaries/<x>`, else the first folder (`docs`, `school-run`, …).
pub fn section_of(id: &str) -> String {
    let parts: Vec<&str> = id.split('/').collect();
    match parts.as_slice() {
        [a @ ("corpora" | "dictionaries"), b, _, ..] => format!("{a}/{b}"),
        [a, _, ..] => a.to_string(),
        _ => "index".to_string(),
    }
}

/// The run boundary of a page: its folder, and for a Bible id (`DEU.355`, `DEU.12.5`) also its book.
pub fn group_of(id: &str) -> String {
    let (dir, name) = id.rsplit_once('/').unwrap_or(("", id));
    let book = name.split_once('.').filter(|(b, rest)| b.len() == 3 && b.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) && rest.starts_with(|c: char| c.is_ascii_digit())).map(|x| x.0);
    match book {
        Some(b) => format!("{dir}/{b}"),
        None => dir.to_string(),
    }
}

/// A sort key that compares numbers as numbers: `GEN.2` before `GEN.10`.
pub fn natural_key(s: &str) -> Vec<(u8, u64, String)> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut dig = false;
    let flush = |cur: &mut String, dig: bool, out: &mut Vec<(u8, u64, String)>| {
        if cur.is_empty() {
            return;
        }
        if dig {
            out.push((0, cur.parse().unwrap_or(u64::MAX), String::new()));
        } else {
            out.push((1, 0, std::mem::take(cur)));
        }
        cur.clear();
    };
    for c in s.chars() {
        let d = c.is_ascii_digit();
        if d != dig {
            flush(&mut cur, dig, &mut out);
            dig = d;
        }
        cur.push(c);
    }
    flush(&mut cur, dig, &mut out);
    out
}

/// The page id of a Markdown file under the root: `corpora/torah/DEU.12.5` for `…/DEU.12.5/index.md`.
pub fn page_id(root: &Path, f: &Path) -> String {
    let rel = f.strip_prefix(root).unwrap_or(f).to_string_lossy().to_string();
    match rel.strip_suffix("/index.md") {
        Some(x) => x.to_string(),
        None if rel == "index.md" => String::new(),
        None => rel.strip_suffix(".md").unwrap_or(&rel).to_string(),
    }
}

/// Every Markdown file under `root`, sorted by the natural order of its page id.
pub fn list(root: &Path) -> Vec<(String, PathBuf)> {
    let mut fs = Vec::new();
    super::walk(root, &mut fs);
    let mut out: Vec<(String, PathBuf)> = fs.into_iter().filter(|p| p.extension().is_some_and(|x| x == "md")).map(|p| (page_id(root, &p), p)).collect();
    out.sort_by_cached_key(|(id, _)| (natural_key(id), id.clone()));
    out
}

// ---------------------------------------------------------------- the page as pieces

struct Builder {
    v: Volume,
    lit: String,
    lit_class: Class,
}

impl Builder {
    fn new(name: &str) -> Builder {
        Builder { v: Volume { name: name.to_string(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n".into() }, lit: String::new(), lit_class: Class::Layout }
    }
    /// A literal; neighbours of one class are one piece.
    fn lit(&mut self, s: &str, c: Class) {
        if s.is_empty() {
            return;
        }
        if c != self.lit_class && !self.lit.is_empty() {
            self.flush();
        }
        self.lit_class = c;
        self.lit.push_str(s);
    }
    fn flush(&mut self) {
        if !self.lit.is_empty() {
            self.v.pieces.push(Piece::Lit(std::mem::take(&mut self.lit), self.lit_class));
        }
    }
    /// A piece the site decoder rebuilds, alone in its piece.
    fn derived(&mut self, s: String, c: Class) {
        self.flush();
        self.v.pieces.push(Piece::Lit(s, c));
    }
    /// Inline text: its prose a text block with its marks, or markup when it holds no letter.
    fn text(&mut self, raw: &str) {
        let (text, marks) = inline(raw);
        if !text.chars().any(char::is_alphabetic) {
            self.lit(raw, Class::Markup);
            return;
        }
        self.flush();
        self.v.blocks.push(VBlock { text, gold: None, whole: false, marks });
        self.v.pieces.push(Piece::Text(self.v.blocks.len() - 1));
    }
    fn done(mut self) -> Volume {
        self.flush();
        self.v
    }
}

const ESCAPABLE: &str = "\\`*_[]{}()#+-.!|<>~";

/// The inline markup of a line of Markdown: the prose with every mark taken out, and the marks at their offsets in
/// the prose. Marks: a backslash before an escaped character, `[` and `](target)` of a link (`![` of an image),
/// `*` and `**` runs, the backticks of code (its content stays prose), inline HTML tags and autolinks.
pub fn inline(s: &str) -> (String, Vec<(usize, String)>) {
    let mut text = String::with_capacity(s.len());
    let mut marks: Vec<(usize, String)> = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    // the closing «](…)» of the links open now
    let mut open_links: Vec<usize> = Vec::new();
    while i < b.len() {
        let c = b[i];
        if c == b'\\' && i + 1 < b.len() && ESCAPABLE.as_bytes().contains(&b[i + 1]) {
            marks.push((text.len(), "\\".into()));
            text.push(b[i + 1] as char);
            i += 2;
            continue;
        }
        if c == b'[' || (c == b'!' && b.get(i + 1) == Some(&b'[')) {
            // a link: «[text](target)» with the closing bracket on this line
            let start = if c == b'!' { i + 1 } else { i };
            if let Some(close) = link_close(s, start) {
                marks.push((text.len(), s[i..start + 1].to_string()));
                open_links.push(close);
                i = start + 1;
                continue;
            }
        }
        if let Some(&close) = open_links.last() {
            if i == close {
                // «](target)»
                let end = s[close..].find(')').map(|k| close + k + 1).unwrap_or(s.len());
                marks.push((text.len(), s[close..end].to_string()));
                open_links.pop();
                i = end;
                continue;
            }
        }
        if c == b'*' {
            let mut j = i;
            while j < b.len() && b[j] == b'*' {
                j += 1;
            }
            marks.push((text.len(), s[i..j].to_string()));
            i = j;
            continue;
        }
        if c == b'`' {
            let mut j = i;
            while j < b.len() && b[j] == b'`' {
                j += 1;
            }
            marks.push((text.len(), s[i..j].to_string()));
            i = j;
            continue;
        }
        if c == b'<' {
            if let Some(k) = s[i..].find('>') {
                let tag = &s[i..i + k + 1];
                let inner = &tag[1..tag.len() - 1];
                let html = inner.chars().next().is_some_and(|x| x.is_ascii_alphabetic() || x == '/' || x == '!') && !inner.contains(' ') || inner.starts_with("http") || inner.contains('=');
                if html && !inner.is_empty() {
                    marks.push((text.len(), tag.to_string()));
                    i += k + 1;
                    continue;
                }
            }
        }
        let ch = s[i..].chars().next().unwrap();
        text.push(ch);
        i += ch.len_utf8();
    }
    (text, marks)
}

/// The byte offset of the «](» that closes the link opening at `open` («[»), if the line has one.
fn link_close(s: &str, open: usize) -> Option<usize> {
    let b = s.as_bytes();
    let mut depth = 0;
    let mut i = open;
    while i < b.len() {
        match b[i] {
            b'\\' => {
                i += 2;
                continue;
            }
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return (b.get(i + 1) == Some(&b'(') && s[i..].contains(')')).then_some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// The frames table's note, which `prag::frames_md::table` writes under it.
const FR_NOTE: &str = "One row per clause the reader finds.";
const FR_NONE: &str = "The frames reader finds no event or fact in this sentence.";
/// The Hebrew verbs' table's note, and the note without a table (`prag::frames_md::hebrew_table`).
const HE_NOTE: &str = "MACULA's semantic frames:";
const HE_NONE: &str = "MACULA marks no verb frame in this verse.";

/// The markers of the page's frame, written by the site around every page: what follows each is markup, not prose.
const CHROME: [&str; 3] = ["<!-- md -->", "<!-- crumbs -->", "<!-- pn -->"];
/// Whether the frame is split off as markup (off: see the note where it is used).
const CHROME_AS_MARKUP: bool = false;

fn is_table_row(l: &str) -> bool {
    l.trim_start().starts_with('|')
}

/// A marker comment's language: not English when it says `lang=` another or `dir=rtl`.
fn foreign(marker: &str) -> bool {
    marker.contains("dir=rtl") || marker.split_whitespace().any(|w| w.strip_prefix("lang=").is_some_and(|l| l != "en" && !l.starts_with("en-")))
}

/// A Markdown page as pieces.
pub fn page_volume(name: &str, md: &str) -> Volume {
    let lines: Vec<&str> = md.split_inclusive('\n').collect();
    let mut b = Builder::new(name);
    let mut i = 0;
    // front matter
    if lines.first().is_some_and(|l| l.trim_end() == "---") {
        if let Some(k) = lines.iter().skip(1).position(|l| l.trim_end() == "---") {
            b.lit(&lines[..k + 2].concat(), Class::Markup);
            i = k + 2;
        }
    }
    let (mut next_foreign, mut next_chrome) = (false, false);
    while i < lines.len() {
        let l = lines[i];
        let body = l.strip_suffix('\n').unwrap_or(l);
        let nl = &l[body.len()..];
        let t = body.trim();
        if t.is_empty() {
            b.lit(l, Class::Layout);
            i += 1;
            continue;
        }
        // a code fence: an English CoNLL-U is rebuilt from the tree, any other fence is markup
        if t.starts_with("```") {
            let mut k = i + 1;
            while k < lines.len() && !lines[k].trim_start().starts_with("```") {
                k += 1;
            }
            let end = (k + 1).min(lines.len());
            let chunk = lines[i..end].concat();
            if t.starts_with("```conllu") {
                b.derived(chunk, Class::Conllu);
            } else {
                b.lit(&chunk, Class::Markup);
            }
            i = end;
            continue;
        }
        // a frames table (an English sentence's, with its note when the page gives it; or the Hebrew verbs' frames)
        if t == "<!-- .fr -->" && lines.get(i + 1).is_some_and(|x| is_table_row(x)) {
            let mut k = i + 1;
            while k < lines.len() && is_table_row(lines[k]) {
                k += 1;
            }
            if lines.get(k).is_some_and(|x| x.trim().is_empty()) && lines.get(k + 1).is_some_and(|x| x.trim() == "<!-- .note -->") && lines.get(k + 2).is_some_and(|x| x.starts_with(FR_NOTE) || x.starts_with(HE_NOTE)) {
                k += 3;
            }
            if lines.get(k).is_some_and(|x| x.trim().is_empty()) {
                k += 1;
            }
            b.derived(lines[i..k].concat(), Class::FrTable);
            i = k;
            continue;
        }
        if t == "<!-- .note -->" && lines.get(i + 1).is_some_and(|x| x.trim_end() == FR_NONE || x.trim_end() == HE_NONE) {
            let mut k = i + 2;
            if lines.get(k).is_some_and(|x| x.trim().is_empty()) {
                k += 1;
            }
            b.derived(lines[i..k].concat(), Class::FrTable);
            i = k;
            continue;
        }
        // a marker for the renderer
        if t.starts_with("<!--") && t.ends_with("-->") {
            next_foreign = foreign(t);
            // the frame as prose for now: stored as markup it cost more (dev 22.10: 49.5 → 62.6 Mbit); it is to be derived
            // from the page's address instead (`CHROME`)
            next_chrome = (CHROME_AS_MARKUP || std::env::var_os("MOVA_RT_CHROME").is_some()) && CHROME.contains(&t);
            b.lit(l, Class::Markup);
            i += 1;
            continue;
        }
        // the page's frame (the twin's link, the breadcrumbs, previous and next): the site's, not text
        if next_chrome {
            let mut k = i;
            while k < lines.len() && !lines[k].trim().is_empty() {
                k += 1;
            }
            b.lit(&lines[i..k].concat(), Class::Markup);
            next_chrome = false;
            i = k;
            continue;
        }
        // a paragraph in another language: kept as written
        if next_foreign {
            let mut k = i;
            while k < lines.len() && !lines[k].trim().is_empty() {
                k += 1;
            }
            b.lit(&lines[i..k].concat(), Class::Unparsed);
            next_foreign = false;
            i = k;
            continue;
        }
        // a heading: its marks, its text, its anchor
        if let Some(h) = heading(body) {
            let (marks, rest) = body.split_at(h);
            b.lit(marks, Class::Markup);
            let (txt, anchor) = match rest.rfind(" {#") {
                Some(k) if rest.ends_with('}') => (&rest[..k], &rest[k..]),
                _ => (rest, ""),
            };
            b.text(txt);
            b.lit(anchor, Class::Markup);
            b.lit(nl, Class::Layout);
            i += 1;
            continue;
        }
        // a table row: pipes are markup, a cell with letters prose
        if is_table_row(body) {
            row(&mut b, body);
            b.lit(nl, Class::Layout);
            i += 1;
            continue;
        }
        // a list item: its mark, then its text
        if let Some(k) = list_mark(body) {
            b.lit(&body[..k], Class::Markup);
            b.text(&body[k..]);
            b.lit(nl, Class::Layout);
            i += 1;
            continue;
        }
        // a paragraph: lines up to a blank line or a line of another kind, one text block
        let mut k = i + 1;
        while k < lines.len() {
            let x = lines[k].trim();
            if x.is_empty() || x.starts_with("```") || x.starts_with("<!--") || heading(lines[k]).is_some() || is_table_row(lines[k]) || list_mark(lines[k]).is_some() {
                break;
            }
            k += 1;
        }
        let para = lines[i..k].concat();
        let pbody = para.strip_suffix('\n').unwrap_or(&para);
        b.text(pbody);
        b.lit(&para[pbody.len()..], Class::Layout);
        i = k;
    }
    b.done()
}

/// The length of a heading's marks («## »), if the line is a heading.
fn heading(l: &str) -> Option<usize> {
    let h = l.bytes().take_while(|&c| c == b'#').count();
    ((1..=6).contains(&h) && l.as_bytes().get(h) == Some(&b' ')).then_some(h + 1)
}

/// The length of a list item's mark («- », «  1. »), if the line is a list item.
fn list_mark(l: &str) -> Option<usize> {
    let ind = l.len() - l.trim_start_matches(' ').len();
    let r = &l[ind..];
    if r.starts_with("- ") || r.starts_with("* ") || r.starts_with("+ ") {
        return Some(ind + 2);
    }
    let d = r.bytes().take_while(u8::is_ascii_digit).count();
    (d > 0 && d < 5 && r[d..].starts_with(". ")).then_some(ind + d + 2)
}

/// A table row: the pipes and the spaces next to them are markup, each cell prose or markup.
fn row(b: &mut Builder, body: &str) {
    // the separator row
    if body.trim().chars().all(|c| matches!(c, '|' | '-' | ':' | ' ')) {
        b.lit(body, Class::Markup);
        return;
    }
    let bytes = body.as_bytes();
    let mut cells: Vec<(usize, usize)> = Vec::new();
    let mut start = None;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                i += 2;
                continue;
            }
            b'|' => {
                if let Some(s) = start {
                    cells.push((s, i));
                }
                start = Some(i + 1);
            }
            _ => {}
        }
        i += 1;
    }
    let mut at = 0;
    for (s, e) in cells {
        let cell = &body[s..e];
        let lead = cell.len() - cell.trim_start().len();
        let trail = cell.len() - cell.trim_end().len();
        b.lit(&body[at..s + lead], Class::Markup);
        if e - trail > s + lead {
            b.text(&body[s + lead..e - trail]);
        }
        at = e - trail;
    }
    b.lit(&body[at..], Class::Markup);
}

/// The texts of a volume given from elsewhere (a translation's output, the source of a translation): a prose block
/// whose text is one of them (marks only around it) and a paragraph kept as written (another language) that is one of
/// them become `Class::Given` pieces, which the residual does not count; their marks stay markup.
pub fn give(v: &Volume, given: &std::collections::BTreeSet<String>) -> Volume {
    let mut out = Volume { name: v.name.clone(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: v.canon_sep.clone() };
    for p in &v.pieces {
        match p {
            Piece::Text(b) => {
                let blk = &v.blocks[*b];
                let around = blk.marks.iter().all(|(k, _)| *k == 0 || *k == blk.text.len());
                if around && given.contains(&blk.text) {
                    let pre: String = blk.marks.iter().filter(|m| m.0 == 0).map(|m| m.1.as_str()).collect();
                    let post: String = blk.marks.iter().filter(|m| m.0 != 0).map(|m| m.1.as_str()).collect();
                    if !pre.is_empty() {
                        out.pieces.push(Piece::Lit(pre, Class::Markup));
                    }
                    out.pieces.push(Piece::Lit(blk.text.clone(), Class::Given));
                    if !post.is_empty() {
                        out.pieces.push(Piece::Lit(post, Class::Markup));
                    }
                } else {
                    out.blocks.push(blk.clone());
                    out.pieces.push(Piece::Text(out.blocks.len() - 1));
                }
            }
            Piece::Lit(x, Class::Unparsed) => {
                let body = x.trim_end_matches('\n');
                if given.contains(body) {
                    out.pieces.push(Piece::Lit(body.to_string(), Class::Given));
                    if body.len() < x.len() {
                        out.pieces.push(Piece::Lit(x[body.len()..].to_string(), Class::Layout));
                    }
                } else {
                    out.pieces.push(p.clone());
                }
            }
            l => out.pieces.push(l.clone()),
        }
    }
    out
}

/// A prose block cheaper as a copy of the prose before it than this share of its bytes stored whole goes to
/// `Class::Repeat`.
pub const REPEAT_SHARE: f64 = 0.25;

/// The prose a block's pages repeat from the prose before them (a template's sentences with other names and
/// numbers): a text block whose copy from the earlier prose (`vol::Copier`) costs at most `REPEAT_SHARE` of its
/// bytes becomes a `Class::Repeat` piece, coded as that copy, and does not go up the layers. `ROUNDTRIP_OFF=site-repeat`
/// keeps every block.
pub fn repeat(v: &Volume) -> Volume {
    if std::env::var("ROUNDTRIP_OFF").is_ok_and(|x| x.split(',').any(|y| y == "site-repeat")) {
        return v.clone();
    }
    let mut cp = crate::vol::Copier::default();
    let mut out = Volume { name: v.name.clone(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: v.canon_sep.clone() };
    for p in &v.pieces {
        match p {
            Piece::Text(b) => {
                let blk = &v.blocks[*b];
                let w = blk.written();
                // the same decision the residual's coder makes, on a copy of its state
                let mut trial = cp.clone();
                let (bits, _) = trial.code(&w);
                if w.len() >= 24 && bits <= REPEAT_SHARE * crate::class::lit_bits(&w) {
                    cp = trial;
                    out.pieces.push(Piece::Lit(w, Class::Repeat));
                } else {
                    cp.remember(&w);
                    out.blocks.push(blk.clone());
                    out.pieces.push(Piece::Text(out.blocks.len() - 1));
                }
            }
            l => out.pieces.push(l.clone()),
        }
    }
    out
}

// ---------------------------------------------------------------- the corpus

/// A page listed for blocks: its id, file, group and section, and its words of prose.
#[derive(Clone, Debug)]
pub struct Listed {
    pub id: String,
    pub path: PathBuf,
    pub group: String,
    pub section: String,
    pub words: usize,
}

/// A page as a chapter.
pub fn chapter(id: &str, md: &str) -> Chapter {
    let v = page_volume(id, md);
    Chapter { id: id.to_string(), group: group_of(id), vol: v, para_cut: false }
}

/// Every page under `root` with its words (pages read in parallel, in order).
pub fn listed(root: &Path, threads: usize) -> Result<Vec<Listed>> {
    let fs = list(root);
    let out = super::heb::par_each(&fs, threads, |_, (id, p)| {
        let md = std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))?;
        let w = chapter(id, &md).words();
        Ok(Listed { id: id.clone(), path: p.clone(), group: group_of(id), section: section_of(id), words: w })
    });
    out.into_iter().collect()
}

/// The runs of pages of a section, by the rule of `blocks::group` (inside a group, up to `max` words).
pub fn runs(pages: &[&Listed], max: usize) -> Vec<std::ops::Range<usize>> {
    let mut out = Vec::new();
    let (mut a, mut w) = (0usize, 0usize);
    for (i, p) in pages.iter().enumerate() {
        if i > a && (p.group != pages[a].group || w + p.words > max) {
            out.push(a..i);
            a = i;
            w = 0;
        }
        w += p.words;
    }
    if a < pages.len() {
        out.push(a..pages.len());
    }
    out
}

/// What to take: every `stride`-th block of a section, or (`target`) so many blocks of each section spread evenly;
/// the split (`train`, `dev`, `test`, or all).
#[derive(Clone, Debug)]
pub struct Sample {
    pub target: usize,
    pub split: Option<String>,
    pub max: usize,
    pub only: Option<Vec<String>>,
}

impl Default for Sample {
    fn default() -> Sample {
        Sample { target: 200, split: Some("dev".into()), max: MAX_WORDS, only: None }
    }
}

/// The site's blocks, per section (corpus `site/<section>`), sampled and split; each block knows its pages.
pub fn blocks(root: &Path, s: &Sample, threads: usize) -> Result<Vec<(String, Vec<Block>)>> {
    let all = listed(root, threads)?;
    let mut by: BTreeMap<String, Vec<&Listed>> = BTreeMap::new();
    for p in &all {
        if s.only.as_ref().is_none_or(|o| o.iter().any(|x| *x == p.section)) {
            by.entry(p.section.clone()).or_default().push(p);
        }
    }
    let mut out = Vec::new();
    for (sec, pages) in by {
        let rs = runs(&pages, s.max);
        let stride = rs.len().div_ceil(s.target.max(1)).max(1);
        let mut bs = Vec::new();
        for (k, r) in rs.iter().enumerate() {
            if k % stride != 0 || s.split.as_deref().is_some_and(|sp| split_of(&pages[r.start].id) != sp) {
                continue;
            }
            let mut chs = Vec::new();
            for p in &pages[r.clone()] {
                let md = std::fs::read_to_string(&p.path).with_context(|| format!("{}", p.path.display()))?;
                chs.push(chapter(&p.id, &md));
            }
            for mut b in super::blocks(&format!("site/{sec}"), &chs, usize::MAX) {
                b.vol = repeat(&b.vol);
                bs.push(b);
            }
        }
        eprintln!("site/{sec}: {} pages, {} runs, every {stride}th taken, {} in the split", pages.len(), rs.len(), bs.len());
        out.push((format!("site/{sec}"), bs));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "---\ntitle: A — b\n---\n\n<!-- md -->\n[Mova](https://x/) · [this page in HTML](https://y/)\n\n# Deuteronomy 12:5 דְּבָרִים\n\n<!-- .vhe dir=rtl lang=hbo -->\nכִּ֠י אִֽם־אֶל\n\n<!-- .ven lang=en -->\n[But unto the place, \\*even\\* thither.](https://z/)\n\n**Who acts:** LORD · `code` here\n\n## The tree {#tree}\n\n```conllu\n# text = But unto the place.\n1\tBut\tbut\tCCONJ\tCC\t_\t0\troot\t_\t_\n```\n\n| # | verb | who |\n| --- | --- | --- |\n| 1 | **choose out** | the LORD *not said here* |\n\n- form: statement\n1. [x](https://w/) — two\n";

    #[test]
    fn a_page_gives_itself_back_and_its_prose_goes_up() {
        let v = page_volume("p", PAGE);
        assert_eq!(v.text(), PAGE);
        let texts: Vec<&str> = v.blocks.iter().map(|b| b.text.as_str()).collect();
        assert!(texts.contains(&"But unto the place, *even* thither."), "{texts:?}");
        assert!(texts.contains(&"Who acts: LORD · code here"), "{texts:?}");
        assert!(texts.contains(&"choose out") && texts.contains(&"the LORD not said here") && texts.contains(&"The tree"), "{texts:?}");
        assert!(!texts.iter().any(|t| t.contains("https") || t.contains('|') || t.contains("כִּ")), "{texts:?}");
        // the page's frame is markup only when it is split off
        assert_eq!(texts.iter().any(|t| t.contains("this page in HTML")), !CHROME_AS_MARKUP, "{texts:?}");
        // the CoNLL-U is a piece of its own, the Hebrew kept whole, the numbers of a table markup
        assert!(v.pieces.iter().any(|p| matches!(p, Piece::Lit(x, Class::Conllu) if x.starts_with("```conllu\n") && x.ends_with("```\n"))));
        assert!(v.pieces.iter().any(|p| matches!(p, Piece::Lit(x, Class::Unparsed) if x.starts_with("כִּ"))));
        assert!(!texts.contains(&"1"));
    }

    #[test]
    fn inline_marks_and_order() {
        let (t, m) = inline("See [the test](https://a/#e2) and **bold** \\| x `y`.");
        assert_eq!(t, "See the test and bold | x y.");
        assert_eq!(crate::vol::with_marks(&t, &m), "See [the test](https://a/#e2) and **bold** \\| x `y`.");
        assert!(natural_key("GEN.2") < natural_key("GEN.10"));
        assert_eq!(group_of("corpora/torah/jps/DEU.355"), "corpora/torah/jps/DEU");
        assert_eq!(group_of("corpora/tales/pg1/17/117"), "corpora/tales/pg1/17");
        assert_eq!(section_of("corpora/torah/DEU.1.1"), "corpora/torah");
        assert_eq!(section_of("docs/papers/x"), "docs");
    }
}
