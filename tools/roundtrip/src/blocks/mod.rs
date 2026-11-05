//! Blocks: the training unit of the round trip. A block is a whole lesson, a book chapter, or a run of whole chapters
//! up to `MAX_WORDS` words (about 4–5 A4 pages at 13 pt), cut only at chapter or lesson boundaries; a single chapter
//! longer than that stays whole. A block is one `vol::Volume`: one discourse context and one world for all of it,
//! with its headings, paragraphs and layout as pieces, so the whole block must come back byte for byte.
//!
//! The makers read each corpus of the training mix into chapters (`Chapter`), in the corpus's own order; `blocks`
//! groups them into runs. Library API (also for `tools/translate`):
//!
//! - `school(dir, stride)`, `book(path, id, caps)`, `jps(judaism, books)`, `talmud_en(judaism)`,
//!   `shakespeare(tsv)`, `grammars(corpora)`, `wlc(judaism, books)` → `Vec<Chapter>`;
//! - `blocks(corpus, &chapters, max_words)` → `Vec<Block>` (greedy runs inside a chapter's `group`);
//! - `like(corpus, &chapters, &blocks)` → the same chapter runs over another text of the same chapters (a
//!   translation: the WLC by the JPS blocks), so the two sides of a pair are the same block.

pub mod heb;
pub mod site_code;
pub mod site_heb;
pub mod site_learn;
pub mod site_md;
pub mod run;

use std::collections::BTreeMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::class::Class;
use crate::vol::{self, Block as VBlock, Piece, Volume};

/// The size of a block: about 4–5 A4 pages at 13 pt.
pub const MAX_WORDS: usize = 2500;
/// A "chapter" longer than this means the source has no chapter headings the maker can see (a dictionary, a
/// handbook numbered by sections only): it is cut at paragraph boundaries into runs of at most `MAX_WORDS`.
pub const HARD_MAX: usize = 10_000;

/// A chapter: the unit a block is cut at (a lesson, a Bible chapter, a halakhah, a scene, a book chapter).
#[derive(Clone, Debug)]
pub struct Chapter {
    /// `GEN.1`, `Berakhot.1.1`, `1H4.1.1`, `pg11#3`, `<course>/<lesson>.lesson`
    pub id: String,
    /// a run of chapters never crosses this (a Bible book, a Talmud chapter, a play, a book file); a lesson is its own
    pub group: String,
    /// the chapter's text as pieces: headings, ids and speakers as metadata, the text, the layout between
    pub vol: Volume,
    /// cut at paragraphs: no chapter heading within `HARD_MAX` words
    pub para_cut: bool,
}

impl Chapter {
    pub fn words(&self) -> usize {
        self.vol.blocks.iter().map(|b| b.text.split_whitespace().count()).sum()
    }
}

/// A block: a run of whole chapters, one volume.
#[derive(Clone, Debug)]
pub struct Block {
    pub corpus: String,
    /// the first chapter's id, or `<first>..<last>`
    pub id: String,
    pub chapters: Vec<String>,
    pub words: usize,
    /// one of its chapters was cut at paragraphs (`HARD_MAX`)
    pub para_cut: bool,
    pub vol: Volume,
}

impl Block {
    /// The block's text, byte for byte as in the source (with the maker's headings for sources kept as records).
    pub fn text(&self) -> String {
        self.vol.text()
    }
}

/// Greedy runs of chapters: a run stays inside one group and grows while it has at most `max` words; a chapter
/// longer than `max` is a run of its own.
pub fn group(chs: &[Chapter], max: usize) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut a = 0;
    let mut w = 0;
    for (i, c) in chs.iter().enumerate() {
        let n = c.words();
        if i > a && (c.group != chs[a].group || w + n > max) {
            out.push(a..i);
            a = i;
            w = 0;
        }
        w += n;
    }
    if a < chs.len() {
        out.push(a..chs.len());
    }
    out
}

/// One volume of a run of chapters: their pieces in order.
pub fn concat(name: &str, chs: &[Chapter]) -> Volume {
    let mut v = Volume { name: name.to_string(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: chs.first().map(|c| c.vol.canon_sep.clone()).unwrap_or_else(|| "\n".into()) };
    for c in chs {
        let off = v.blocks.len();
        v.blocks.extend(c.vol.blocks.iter().cloned());
        v.pieces.extend(c.vol.pieces.iter().map(|p| match p {
            Piece::Text(b) => Piece::Text(b + off),
            l => l.clone(),
        }));
    }
    v
}

fn block_of(corpus: &str, chs: &[Chapter]) -> Block {
    let id = if chs.len() == 1 { chs[0].id.clone() } else { format!("{}..{}", chs[0].id, chs[chs.len() - 1].id) };
    Block {
        corpus: corpus.to_string(),
        vol: concat(&format!("{corpus}:{id}"), chs),
        id,
        chapters: chs.iter().map(|c| c.id.clone()).collect(),
        words: chs.iter().map(Chapter::words).sum(),
        para_cut: chs.iter().any(|c| c.para_cut),
    }
}

/// The blocks of a corpus.
pub fn blocks(corpus: &str, chs: &[Chapter], max: usize) -> Vec<Block> {
    group(chs, max).into_iter().map(|r| block_of(corpus, &chs[r])).collect()
}

/// The blocks of `chs` cut as `like` is (by chapter ids): the other side of a translation in the same blocks. A
/// chapter of `chs` that no block of `like` names is left out.
pub fn like(corpus: &str, chs: &[Chapter], like: &[Block]) -> Vec<Block> {
    let by: BTreeMap<&str, usize> = chs.iter().enumerate().map(|(i, c)| (c.id.as_str(), i)).collect();
    like.iter()
        .filter_map(|b| {
            let run: Vec<Chapter> = b.chapters.iter().filter_map(|id| by.get(id.as_str()).map(|&i| chs[i].clone())).collect();
            (!run.is_empty()).then(|| block_of(corpus, &run))
        })
        .collect()
}

fn read(p: &Path) -> Result<String> {
    std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))
}

fn tsv_rows(p: &Path) -> Result<Vec<Vec<String>>> {
    Ok(read(p)?.lines().map(|l| l.split('\t').map(String::from).collect()).collect())
}

/// A chapter of records: a heading (metadata), then paragraphs of text, each with its own metadata line before it
/// (a speaker) or none, separated by blank lines.
fn records(id: &str, group: &str, heading: &str, paras: &[(Option<String>, String)]) -> Chapter {
    let mut v = Volume { name: id.to_string(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n\n".into() };
    v.pieces.push(Piece::Lit(format!("{heading}\n"), Class::Meta));
    for (k, (meta, text)) in paras.iter().enumerate() {
        if k > 0 {
            v.pieces.push(Piece::Lit("\n\n".into(), Class::Layout));
        }
        if let Some(m) = meta {
            v.pieces.push(Piece::Lit(format!("{m}\n"), Class::Meta));
        }
        v.blocks.push(VBlock { text: text.clone(), gold: None, whole: false, marks: Vec::new() });
        v.pieces.push(Piece::Text(v.blocks.len() - 1));
    }
    v.pieces.push(Piece::Lit("\n\n".into(), Class::Layout));
    Chapter { id: id.to_string(), group: group.to_string(), vol: v, para_cut: false }
}

// ---------------------------------------------------------------- the makers

/// The school: every `stride`-th lesson (`<course>/*.lesson`, sorted) is a chapter and a block of its own.
pub fn school(dir: &Path, stride: usize) -> Result<Vec<Chapter>> {
    Ok(vol::read_lessons(dir, stride)?
        .into_iter()
        .map(|v| Chapter { id: v.name.clone(), group: v.name.clone(), vol: v, para_cut: false })
        .collect())
}

const KEYWORDS: [&str; 8] = ["CHAPTER", "Chapter", "PART", "BOOK", "LECTURE", "Lecture", "STORY", "SECTION"];

fn keyword_heading(t: &str) -> bool {
    KEYWORDS.iter().any(|k| t.strip_prefix(k).is_some_and(|r| r.is_empty() || r.starts_with([' ', '.', ':'])))
}

/// A title line: alone between blank lines, at most ten words, letters and no lower-case letter (HOW THE WHALE GOT
/// HIS THROAT).
fn caps_heading(t: &str) -> bool {
    let letters = t.chars().filter(|c| c.is_alphabetic()).count();
    letters >= 3 && !t.chars().any(char::is_lowercase) && t.split_whitespace().count() <= 10
}

/// A plain-text book cut into chapters at its headings: a line starting with CHAPTER, PART, BOOK, LECTURE, STORY or
/// SECTION, and with `caps` also a title line in capitals alone between blank lines. The text before the first
/// heading is a chapter too (the front matter). A chapter longer than `HARD_MAX` words is cut at blank lines into
/// runs of at most `MAX_WORDS` (`para_cut`). The chapters together are the file byte for byte.
pub fn book(path: &Path, id: &str, caps: bool) -> Result<Vec<Chapter>> {
    let text = read(path)?;
    Ok(book_text(&text, id, caps))
}

pub fn book_text(text: &str, id: &str, caps: bool) -> Vec<Chapter> {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let blank = |i: usize| lines.get(i).is_none_or(|l| l.trim().is_empty());
    let mut cuts = vec![0usize];
    for (i, l) in lines.iter().enumerate() {
        let t = l.trim();
        if t.is_empty() || i == 0 {
            continue;
        }
        let head = keyword_heading(t) || caps && i > 0 && blank(i - 1) && blank(i + 1) && caps_heading(t);
        if head && blank(i - 1) {
            cuts.push(i);
        }
    }
    cuts.push(lines.len());
    cuts.dedup();
    let mut out = Vec::new();
    for w in cuts.windows(2) {
        let (a, b) = (w[0], w[1]);
        let words: usize = lines[a..b].iter().map(|l| l.split_whitespace().count()).sum();
        if words == 0 && a > 0 {
            // nothing but blank lines: they belong to the chapter before
            if let Some(last) = out.last_mut() {
                let c: &mut Chapter = last;
                let s = c.vol.text() + &lines[a..b].concat();
                c.vol = vol::txt_volume(&c.id, &s);
                continue;
            }
        }
        let pieces: Vec<Range<usize>> = if words > HARD_MAX { para_runs(&lines[a..b], MAX_WORDS).into_iter().map(|r| r.start + a..r.end + a).collect() } else { vec![a..b] };
        let cut = pieces.len() > 1;
        for r in pieces {
            let n = out.len();
            let cid = format!("{id}#{n}");
            out.push(Chapter { vol: vol::txt_volume(&cid, &lines[r].concat()), id: cid, group: id.to_string(), para_cut: cut });
        }
    }
    out
}

/// Runs of whole paragraphs (a run of lines ending after blank lines) of at most `max` words each.
fn para_runs(lines: &[&str], max: usize) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let (mut a, mut w) = (0usize, 0usize);
    for (i, l) in lines.iter().enumerate() {
        let n = l.split_whitespace().count();
        // a paragraph starts at a non-blank line after a blank one
        let starts = i > a && n > 0 && lines[i - 1].trim().is_empty();
        if starts && w + n > max && w > 0 {
            out.push(a..i);
            a = i;
            w = 0;
        }
        w += n;
    }
    if a < lines.len() {
        out.push(a..lines.len());
    }
    out
}

/// The book of a Bible verse id (`GEN.1.1`) and its chapter (`GEN.1`).
fn chapter_of(verse: &str) -> Option<(String, String)> {
    let mut it = verse.split('.');
    let (b, c) = (it.next()?, it.next()?);
    Some((b.to_string(), format!("{b}.{c}")))
}

/// JPS 1917 (`dict/tanakh-jps1917.tsv` with `.verses`): a chapter is the sentences whose first verse is in it, as
/// one paragraph (the sentences joined by a space), under the heading `GEN 1`. `books`: these books only (all if
/// empty), in the file's order.
pub fn jps(judaism: &Path, books: &[&str]) -> Result<Vec<Chapter>> {
    let d = judaism.join("dict");
    let verses: BTreeMap<String, String> = tsv_rows(&d.join("tanakh-jps1917.tsv.verses"))?.into_iter().filter(|r| r.len() >= 2).map(|r| (r[0].clone(), r[1].clone())).collect();
    let mut order: Vec<String> = Vec::new();
    let mut by: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    for r in tsv_rows(&d.join("tanakh-jps1917.tsv"))? {
        if r.len() < 2 {
            continue;
        }
        let Some((b, c)) = verses.get(&r[0]).and_then(|v| chapter_of(v.split_whitespace().next().unwrap_or(""))) else { continue };
        if !books.is_empty() && !books.contains(&b.as_str()) {
            continue;
        }
        if !by.contains_key(&c) {
            order.push(c.clone());
        }
        by.entry(c).or_insert_with(|| (b, Vec::new())).1.push(r[1].trim().to_string());
    }
    Ok(order
        .into_iter()
        .map(|c| {
            let (b, ss) = &by[&c];
            records(&c, b, &c.replacen('.', " ", 1), &[(None, ss.join(" "))])
        })
        .collect())
}

/// The Hebrew Bible (WLC, `dict/wlc-trees/<BOOK>.conllu`, `# text` and `# verse`): a chapter is its verses, one a
/// line, as written (accents and points). The ids are those of `jps`, so `like` gives the same blocks.
pub fn wlc(judaism: &Path, books: &[&str]) -> Result<Vec<Chapter>> {
    let mut out: Vec<Chapter> = Vec::new();
    for b in books {
        let mut verse = String::new();
        let mut cur: Option<(String, Vec<String>)> = None;
        let flush = |cur: &mut Option<(String, Vec<String>)>, out: &mut Vec<Chapter>| {
            if let Some((c, vs)) = cur.take() {
                let mut v = Volume { name: c.clone(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n".into() };
                for t in vs {
                    v.blocks.push(VBlock { text: t, gold: None, whole: true, marks: Vec::new() });
                    v.pieces.push(Piece::Text(v.blocks.len() - 1));
                    v.pieces.push(Piece::Lit("\n".into(), Class::Layout));
                }
                out.push(Chapter { id: c, group: b.to_string(), vol: v, para_cut: false });
            }
        };
        for l in read(&judaism.join(format!("dict/wlc-trees/{b}.conllu")))?.lines() {
            if let Some(x) = l.strip_prefix("# verse = ") {
                verse = x.split_whitespace().next().unwrap_or("").to_string();
            } else if let Some(x) = l.strip_prefix("# text = ") {
                let Some((_, c)) = chapter_of(&verse) else { continue };
                if cur.as_ref().is_none_or(|(k, _)| *k != c) {
                    flush(&mut cur, &mut out);
                    cur = Some((c, Vec::new()));
                }
                cur.as_mut().unwrap().1.push(x.to_string());
            }
        }
        flush(&mut cur, &mut out);
    }
    Ok(out)
}

/// The Yerushalmi in Guggenheimer's English (`text/yerushalmi-guggenheimer/<tractate>.en.tsv`): a chapter is a
/// halakhah (`Berakhot.1.1`), its segments paragraphs under the heading `Berakhot 1.1`; runs stay inside a chapter
/// of the tractate.
pub fn talmud_en(judaism: &Path) -> Result<Vec<Chapter>> {
    let y = judaism.join("text/yerushalmi-guggenheimer");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&y).with_context(|| format!("{}", y.display()))?.flatten().map(|e| e.path()).filter(|p| p.to_string_lossy().ends_with(".en.tsv")).collect();
    files.sort();
    let mut out = Vec::new();
    for f in files {
        let mut cur: Option<(String, Vec<(Option<String>, String)>)> = None;
        let flush = |cur: &mut Option<(String, Vec<(Option<String>, String)>)>, out: &mut Vec<Chapter>| {
            if let Some((h, ps)) = cur.take() {
                let g = h.rsplit_once('.').map(|x| x.0.to_string()).unwrap_or_default();
                out.push(records(&h, &g, &h.replacen('.', " ", 1), &ps));
            }
        };
        for r in tsv_rows(&f)? {
            if r.len() < 2 || r[1].trim().is_empty() {
                continue;
            }
            let Some((h, _)) = r[0].rsplit_once('.') else { continue };
            if cur.as_ref().is_none_or(|(k, _)| k != h) {
                flush(&mut cur, &mut out);
                cur = Some((h.to_string(), Vec::new()));
            }
            cur.as_mut().unwrap().1.push((None, r[1].trim().to_string()));
        }
        flush(&mut cur, &mut out);
    }
    Ok(out)
}

/// Shakespeare (`shakespeare.tsv`, `id<TAB>sentence`, with `.speakers`: `id<TAB>speaker<TAB>lines`): a chapter is a
/// scene (`1H4.1.1`), a speech a paragraph under its speaker; runs stay inside a play.
pub fn shakespeare(tsv: &Path) -> Result<Vec<Chapter>> {
    let sp_path = PathBuf::from(format!("{}.speakers", tsv.display()));
    let speakers: BTreeMap<String, (String, String)> = tsv_rows(&sp_path)?.into_iter().filter(|r| r.len() >= 3).map(|r| (r[0].clone(), (r[1].clone(), r[2].clone()))).collect();
    let mut out = Vec::new();
    let mut cur: Option<(String, String, Vec<(Option<String>, String)>, Option<(String, String)>)> = None;
    let flush = |cur: &mut Option<(String, String, Vec<(Option<String>, String)>, Option<(String, String)>)>, out: &mut Vec<Chapter>| {
        if let Some((scene, play, ps, _)) = cur.take() {
            out.push(records(&scene, &play, &scene.replacen('.', " ", 1), &ps));
        }
    };
    for r in tsv_rows(tsv)? {
        if r.len() < 2 {
            continue;
        }
        // shk:<PLAY>:<act.scene>:<n>
        let f: Vec<&str> = r[0].split(':').collect();
        if f.len() < 4 {
            continue;
        }
        let scene = format!("{}.{}", f[1], f[2]);
        if cur.as_ref().is_none_or(|c| c.0 != scene) {
            flush(&mut cur, &mut out);
            cur = Some((scene, f[1].to_string(), Vec::new(), None));
        }
        let c = cur.as_mut().unwrap();
        let who = speakers.get(&r[0]).cloned();
        let text = r[1].trim().to_string();
        if who.is_some() && who == c.3 && !c.2.is_empty() {
            let last = c.2.last_mut().unwrap();
            last.1.push(' ');
            last.1.push_str(&text);
        } else {
            c.2.push((who.as_ref().map(|w| w.0.clone()), text));
            c.3 = who;
        }
    }
    flush(&mut cur, &mut out);
    Ok(out)
}

/// The grammar corpora (`corpora/grammars-*`): our manuals (`text/manual/**.md`, a file a chapter, without
/// `sources*.md` and `README.md`, as their pools) and the English grammar books (`text/books-en/*.txt`, by
/// `book`), the books other languages and the read-only ones left out, as the pools leave them.
pub fn grammars(corpora: &Path) -> Result<Vec<Chapter>> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(corpora)?.flatten().map(|e| e.path()).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("grammars-"))).collect();
    dirs.sort();
    let mut out = Vec::new();
    for d in dirs {
        let g = d.file_name().unwrap().to_string_lossy().to_string();
        let mut mds = Vec::new();
        walk(&d.join("text/manual"), &mut mds);
        mds.retain(|p| {
            let n = p.file_name().unwrap().to_string_lossy();
            n.ends_with(".md") && !n.starts_with("sources") && n != "README.md"
        });
        mds.sort();
        for p in mds {
            let rel = p.strip_prefix(&d).unwrap_or(&p).display().to_string();
            let id = format!("{g}/{rel}");
            let group = format!("{g}/{}", p.parent().and_then(|x| x.strip_prefix(&d).ok()).map(|x| x.display().to_string()).unwrap_or_default());
            out.push(Chapter { vol: vol::txt_volume(&id, &read(&p)?), id, group, para_cut: false });
        }
        let mut books: Vec<PathBuf> = std::fs::read_dir(d.join("text/books-en")).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect()).unwrap_or_default();
        books.sort();
        for p in books {
            let id = format!("{g}/{}", p.file_stem().unwrap().to_string_lossy());
            out.extend(book(&p, &id, false)?);
        }
    }
    Ok(out)
}

pub(crate) fn walk(d: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(d) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// Plain-text books of a directory (`*.txt`, sorted), each by `book` with title lines.
pub fn books_dir(dir: &Path) -> Result<Vec<Chapter>> {
    let mut fs: Vec<PathBuf> = std::fs::read_dir(dir).with_context(|| format!("{}", dir.display()))?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
    fs.sort();
    let mut out = Vec::new();
    for p in fs {
        out.extend(book(&p, &p.file_stem().unwrap().to_string_lossy(), true)?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_chapters_give_the_file_back_and_runs_stay_whole() {
        let mut t = String::from("Front matter.\n\n\nCHAPTER I.\nDown the Rabbit-Hole\n\nAlice was tired.\n\n\nTHE WHALE\n\nA whale ate fish.\n\n");
        t += "CHAPTER II.\n\n";
        for _ in 0..30 {
            t += "Many words go here and then some more words follow them.\n\n";
        }
        let chs = book_text(&t, "b", true);
        assert_eq!(chs.iter().map(|c| c.vol.text()).collect::<String>(), t);
        let ids: Vec<&str> = chs.iter().map(|c| c.vol.blocks[0].text.as_str()).collect();
        assert_eq!(ids, ["Front matter.", "CHAPTER I.\nDown the Rabbit-Hole", "THE WHALE", "CHAPTER II."]);
        // a run grows to at most 30 words, a longer chapter is one block, never cut
        let bs = blocks("t", &chs, 30);
        assert_eq!(bs.iter().map(|b| b.text()).collect::<String>(), t);
        assert_eq!(bs.iter().map(|b| b.chapters.len()).collect::<Vec<_>>(), [3, 1]);
        assert!(bs[1].words > 30);
    }

    #[test]
    fn a_chapter_without_headings_is_cut_at_paragraphs() {
        let mut t = String::new();
        for _ in 0..(HARD_MAX / 10 + 50) {
            t += "one two three four five six seven eight nine ten\n\n";
        }
        let chs = book_text(&t, "d", false);
        assert!(chs.len() > 1 && chs.iter().all(|c| c.para_cut && c.words() <= MAX_WORDS));
        assert_eq!(chs.iter().map(|c| c.vol.text()).collect::<String>(), t);
    }
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn grouping_and_duplicate_id_projection() {
        let mk = |id: &str, group: &str, text: &str| Chapter {
            id: id.into(),
            group: group.into(),
            vol: vol::txt_volume(id, text),
            para_cut: false,
        };
        let chs = vec![
            mk("a", "g", "one two"),
            mk("b", "g", "three"),
            mk("c", "h", "four"),
        ];
        assert_eq!(group(&chs, 3), [0..2, 2..3]);
        assert_eq!(group(&chs, 2), [0..1, 1..2, 2..3]);
        assert!(group(&[], 0).is_empty());
        let bs = blocks("x", &chs, 3);
        assert_eq!(bs[0].text(), "one twothree");
        let translated = vec![mk("a", "g", "old"), mk("a", "g", "new")];
        let matched = like("y", &translated, &bs);
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].text(), "new");
        assert_eq!(matched[0].chapters, ["a"]);
    }
}
