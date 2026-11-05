//! Blocks: whole chapters, up to about 2 500 words (four or five pages; e.g. Genesis 1–3), cut only at chapter
//! boundaries, the unit a translation must give back byte for byte.
//!
//! A block's text is its chapters in order: a chapter line (`\c N`), then for each verse the layout the author puts
//! before it — paragraph and poetry marks (`\p`, `\q1`, `\b` …) and headings (`\s1 …`, `\d …`, `\ms1 …`), each on a
//! line — and the verse line `\v N <text>`, the verse text as the verse pools have it. The layout is read from the
//! author's USFM; marks inside a verse (a poetry line that goes on the same verse) are not kept.
//!
//! The block's code is the verse codes of its target verses (the encoder and the stack, verse by verse), one reader
//! script over the whole block (the meta-worlds of its verses read as one, a boundary token between verses, so content
//! may move between verses), and the layout's residual: the target's verse numbers that the source does not have and
//! the reverse, and every verse whose layout differs from the source's (its marks as literals).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::Result;
use roundtrip::class::{Class, Layer, lit_bits, pos_bits};
use roundtrip::tally::Residual;

use crate::bible::{VKey, book_of};
use crate::corpus::Corpus;

/// A block: a book's chapters [first, last] and the verse keys a reference author has in them.
#[derive(Clone, Debug)]
pub struct Block {
    pub book: u8,
    pub first: u16,
    pub last: u16,
}

impl Block {
    pub fn contains(&self, k: VKey) -> bool {
        k.0 == self.book && k.1 >= self.first && k.1 <= self.last
    }
    pub fn name(&self) -> String {
        format!("{} {}–{}", crate::bible::code(self.book), self.first, self.last)
    }
}

/// Cut the books of a reference author into blocks: whole chapters, added while the block stays within `max` words
/// (a chapter longer than that is a block of its own).
pub fn make(reference: &Corpus, keep: &dyn Fn(VKey) -> bool, max: usize) -> Vec<Block> {
    let mut words: BTreeMap<(u8, u16), usize> = BTreeMap::new();
    for (k, v) in &reference.verses {
        if keep(*k) {
            *words.entry((k.0, k.1)).or_default() += v.text.split_whitespace().count();
        }
    }
    let mut out: Vec<Block> = Vec::new();
    let mut cur: Option<(Block, usize)> = None;
    for ((b, c), w) in words {
        match cur.as_mut() {
            Some((blk, n)) if blk.book == b && *n + w <= max && c == blk.last + 1 => {
                blk.last = c;
                *n += w;
            }
            _ => {
                if let Some((blk, _)) = cur.take() {
                    out.push(blk);
                }
                cur = Some((Block { book: b, first: c, last: c }, w));
            }
        }
    }
    if let Some((blk, _)) = cur {
        out.push(blk);
    }
    out
}

/// The same cut by `roundtrip blocks`' own rule (`roundtrip::blocks::group`): the reference author's chapters as
/// roundtrip chapters (id `BOOK.C`, group the book, the verses as the chapter's text).
pub fn make_rt(reference: &Corpus, keep: &dyn Fn(VKey) -> bool, max: usize) -> Vec<Block> {
    use roundtrip::vol::{Block as VBlock, Piece, Volume};
    let mut by: BTreeMap<(u8, u16), Vec<String>> = BTreeMap::new();
    for (k, v) in &reference.verses {
        if keep(*k) {
            by.entry((k.0, k.1)).or_default().push(v.text.clone());
        }
    }
    let keys: Vec<(u8, u16)> = by.keys().copied().collect();
    let chs: Vec<roundtrip::blocks::Chapter> = by
        .into_iter()
        .map(|((b, c), vs)| {
            let id = format!("{}.{c}", crate::bible::code(b));
            let mut vol = Volume { name: id.clone(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n".into() };
            for t in vs {
                vol.blocks.push(VBlock { text: t, gold: None, whole: true, marks: Vec::new() });
                vol.pieces.push(Piece::Text(vol.blocks.len() - 1));
            }
            roundtrip::blocks::Chapter { id, group: crate::bible::code(b).to_string(), vol, para_cut: false }
        })
        .collect();
    roundtrip::blocks::group(&chs, max).into_iter().map(|r| Block { book: keys[r.start].0, first: keys[r.start].1, last: keys[r.end - 1].1 }).collect()
}

/// The chapter runs of `roundtrip blocks` itself for JPS 1917 (its Tanakh pool, Hebrew chapter numbers), as
/// (book, first, last): to check that the cut here is the same.
pub fn rt_jps(judaism: &Path, books: &[&str], max: usize) -> Result<Vec<(u8, u16, u16)>> {
    let chs = roundtrip::blocks::jps(judaism, books)?;
    Ok(roundtrip::blocks::blocks("jps", &chs, max)
        .iter()
        .filter_map(|b| {
            let f = b.chapters.first()?;
            let l = b.chapters.last()?;
            let (bk, c1) = f.split_once('.')?;
            let (_, c2) = l.split_once('.')?;
            Some((book_of(bk)?, c1.parse().ok()?, c2.parse().ok()?))
        })
        .collect())
}

fn clean(text: &str) -> String {
    // character marks off, as `school-sents usfm` takes them off
    let mut s = String::new();
    let mut it = text.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '|' => {
                while it.peek().is_some_and(|&x| x != '\\') {
                    it.next();
                }
            }
            '\\' => {
                let mut closing = false;
                while let Some(&x) = it.peek() {
                    if x.is_ascii_alphanumeric() || x == '+' {
                        it.next();
                    } else {
                        if x == '*' {
                            it.next();
                            closing = true;
                        }
                        break;
                    }
                }
                if !closing && it.peek() == Some(&' ') {
                    it.next();
                }
            }
            c => s.push(c),
        }
    }
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

const HEADINGS: [&str; 10] = ["\\s", "\\s1", "\\s2", "\\s3", "\\d", "\\ms", "\\ms1", "\\ms2", "\\r", "\\mr"];

/// The layout before every verse of an author's USFM (English verse numbers): paragraph and poetry marks and
/// headings, in order, each as its line will read.
pub fn read_layout(dir: &Path) -> Result<BTreeMap<VKey, Vec<String>>> {
    let files = crate::bible::usfm_files(dir)?;
    let mut out: BTreeMap<VKey, Vec<String>> = BTreeMap::new();
    for f in files {
        let text = crate::bible::read_usfm(&f)?;
        let (mut book, mut c) = (None, 0u16);
        let mut pending: Vec<String> = Vec::new();
        for line in text.lines() {
            let l = line.trim();
            // a mark, then possibly a verse on the same line («\q1 \v 2 …», «\p \v 1 …»)
            let mut rest = l;
            loop {
                let (mark, body) = rest.split_once(' ').unwrap_or((rest, ""));
                match mark {
                    "\\id" => {
                        book = body.split_whitespace().next().and_then(book_of);
                        break;
                    }
                    "\\c" => {
                        c = body.trim().parse().unwrap_or(c);
                        pending.clear();
                        break;
                    }
                    "\\v" => {
                        let n = body.trim().split(' ').next().and_then(|x| x.parse::<u16>().ok());
                        if let (Some(b), Some(n)) = (book, n) {
                            out.insert((b, c, n), std::mem::take(&mut pending));
                        }
                        break;
                    }
                    m if HEADINGS.contains(&m) => {
                        // the heading's own text, with its footnotes off
                        let mut t = body.to_string();
                        while let (Some(a), Some(z)) = (t.find("\\f "), t.find("\\f*")) {
                            if z < a {
                                break;
                            }
                            t.replace_range(a..z + 3, "");
                        }
                        pending.push(format!("{m} {}", clean(&t)).trim_end().to_string());
                        break;
                    }
                    m if m.starts_with('\\') && !m.starts_with("\\h") && !m.starts_with("\\toc") && !m.starts_with("\\mt") && !m.starts_with("\\id") && !m.starts_with("\\c") && !m.starts_with("\\rem") => {
                        // a paragraph or poetry mark: it lays out the verse that follows on its line, or the next one;
                        // with text of its own it goes on a verse begun before (not kept)
                        let b = body.trim_start();
                        if b.is_empty() {
                            pending.push(m.to_string());
                            break;
                        }
                        if b.starts_with("\\v ") {
                            pending.push(m.to_string());
                            rest = b;
                            continue;
                        }
                        break;
                    }
                    _ => break,
                }
            }
        }
    }
    Ok(out)
}

/// A block's text for an author: its chapters, each verse's layout and verse line.
pub fn text(blk: &Block, c: &Corpus, layout: &BTreeMap<VKey, Vec<String>>, verse_text: &dyn Fn(VKey) -> Option<String>) -> String {
    let mut s = String::new();
    let mut chapter = 0u16;
    for k in c.verses.keys().filter(|k| blk.contains(**k)) {
        if k.1 != chapter {
            chapter = k.1;
            s.push_str(&format!("\\c {chapter}\n"));
        }
        for l in layout.get(k).map(Vec::as_slice).unwrap_or(&[]) {
            s.push_str(l);
            s.push('\n');
        }
        s.push_str(&format!("\\v {} {}\n", k.2, verse_text(*k).unwrap_or_default()));
    }
    s
}

/// The layout residual of a block: the verse numbers of the target the source lacks and those the source has and the
/// target lacks (a position each), and every target verse whose layout is not the source's (its marks, literal).
/// Returns the residual and the decoded target layout.
pub fn layout_code(blk: &Block, src_keys: &BTreeSet<VKey>, tgt_keys: &BTreeSet<VKey>, src: &BTreeMap<VKey, Vec<String>>, tgt: &BTreeMap<VKey, Vec<String>>, res: &mut Residual) -> BTreeMap<VKey, Vec<String>> {
    let n = tgt_keys.len().max(src_keys.len());
    for k in tgt_keys.symmetric_difference(src_keys).filter(|k| blk.contains(**k)) {
        let kk = *k;
        res.add(Layer::Tokens, Class::Layout, pos_bits(n), || format!("verse {} {}", crate::bible::show(kk), if tgt_keys.contains(&kk) { "only in the target" } else { "only in the source" }));
    }
    let mut stored: BTreeMap<VKey, Vec<String>> = BTreeMap::new();
    for k in tgt_keys.iter().filter(|k| blk.contains(**k)) {
        let want = tgt.get(k).cloned().unwrap_or_default();
        let have = if src_keys.contains(k) { src.get(k).cloned().unwrap_or_default() } else { Vec::new() };
        if want != have {
            let l = want.join("\n");
            let kk = *k;
            res.add(Layer::Tokens, Class::Layout, lit_bits(&l) + pos_bits(n) + 1.0, || format!("{}: {:?} for {:?}", crate::bible::show(kk), want, have));
            stored.insert(*k, want);
        }
    }
    // decode: the source's layout where nothing is stored
    tgt_keys
        .iter()
        .filter(|k| blk.contains(**k))
        .map(|k| (*k, stored.get(k).cloned().unwrap_or_else(|| if src_keys.contains(k) { src.get(k).cloned().unwrap_or_default() } else { Vec::new() })))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_of_a_verse_line() {
        let dir = std::env::temp_dir().join(format!("translate-blocks-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("02-GEN.usfm"), "\\id GEN test\n\\c 1\n\\s1 The creation\n\\p\n\\v 1 In the beginning\n\\q1 \\v 2 And the earth\n\\q2 was waste\n\\b\n\\v 3 And God\n").unwrap();
        let l = read_layout(&dir).unwrap();
        assert_eq!(l[&(0, 1, 1)], vec!["\\s1 The creation".to_string(), "\\p".to_string()]);
        assert_eq!(l[&(0, 1, 2)], vec!["\\q1".to_string()]);
        assert_eq!(l[&(0, 1, 3)], vec!["\\b".to_string()]);
        std::fs::remove_dir_all(&dir).ok();
    }
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn layout_reconstruction_and_ranges() {
        let b = Block {
            book: 0,
            first: 1,
            last: 2,
        };
        assert!(b.contains((0, 2, 0)));
        assert!(!b.contains((1, 2, 1)));
        let a = (0, 1, 1);
        let z = (0, 2, 1);
        let src = [(a, vec!["p".into()])].into();
        let tgt = [(a, vec![]), (z, vec!["q".into()])].into();
        let mut r = Residual::default();
        assert_eq!(
            layout_code(&b, &[a].into(), &[a, z].into(), &src, &tgt, &mut r),
            tgt
        );
        assert!(r.cells.values().map(|c| c.bits).sum::<f64>() > 0.0);
    }
}
