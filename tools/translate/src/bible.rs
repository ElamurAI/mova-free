//! Verse-aligned Bibles: the pools `school-sents usfm` cuts (`id<TAB>sentence` and `<pool>.verses`, `id<TAB>BOOK.C.V`),
//! read back into verses; the book split into train, dev and test.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};

/// The 39 books of the Hebrew Bible in the order of the English Old Testament (eBible's file order). The split is by
/// this order, the same for every translation.
pub const BOOKS: [&str; 39] = [
    "GEN", "EXO", "LEV", "NUM", "DEU", "JOS", "JDG", "RUT", "1SA", "2SA", "1KI", "2KI", "1CH", "2CH", "EZR", "NEH", "EST", "JOB", "PSA", "PRO", "ECC", "SNG", "ISA", "JER", "LAM", "EZK", "DAN", "HOS", "JOL", "AMO",
    "OBA", "JON", "MIC", "NAM", "HAB", "ZEP", "HAG", "ZEC", "MAL",
];

/// The 27 books of the New Testament in eBible's order. Their indices go on after the Hebrew Bible's (39 …), so
/// `split_of` splits them the same way: dev 1CO, TIT, REV; test ACT, EPH, 1TI, JAS, 3JN.
pub const NT: [&str; 27] = [
    "MAT", "MRK", "LUK", "JHN", "ACT", "ROM", "1CO", "2CO", "GAL", "EPH", "PHP", "COL", "1TH", "2TH", "1TI", "2TI", "TIT", "PHM", "HEB", "JAS", "1PE", "2PE", "1JN", "2JN", "3JN", "JUD", "REV",
];

/// The deuterocanonical and other books some canons have (the Catholic, the Orthodox; eBible's codes), after the New
/// Testament: Tobit, Judith, Greek Esther, Wisdom, Sirach, Baruch, the Letter of Jeremiah, the Song of the Three,
/// Susanna, Bel, 1–4 Maccabees, 1–2 Esdras, the Prayer of Manasseh, Psalm 151, Greek Daniel.
pub const DC: [&str; 19] = ["TOB", "JDT", "ESG", "WIS", "SIR", "BAR", "LJE", "S3Y", "SUS", "BEL", "1MA", "2MA", "3MA", "4MA", "1ES", "2ES", "MAN", "PS2", "DAG"];

/// A verse: book (index into `BOOKS`, then `NT`, then `DC`), chapter, verse.
pub type VKey = (u8, u16, u16);

/// A book's code by its index: the Hebrew Bible's, then the New Testament's.
pub fn code(b: u8) -> &'static str {
    let b = b as usize;
    BOOKS.iter().chain(NT.iter()).chain(DC.iter()).nth(b).copied().unwrap_or("?")
}

pub fn book_of(code: &str) -> Option<u8> {
    BOOKS.iter().chain(NT.iter()).chain(DC.iter()).position(|b| *b == code).map(|i| i as u8)
}

/// The USFM files of an eBible directory, one book a file: plain `*.usfm` and seekable-zstd `*.usfm.zst` (the
/// research library packs every file of 64 KiB or more), in file-name order.
pub fn usfm_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("{}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".usfm") || n.ends_with(".usfm.zst")))
        .collect();
    files.sort_by_key(|p| p.file_name().and_then(|n| n.to_str()).map(|n| n.trim_end_matches(".zst").to_string()));
    Ok(files)
}

/// A USFM file's text, decompressed when it is `.zst` (independent frames and a skippable seek table: the stream
/// decoder reads them all, as `zstdcat` does).
pub fn read_usfm(p: &Path) -> Result<String> {
    if p.extension().is_some_and(|x| x == "zst") {
        let f = std::fs::File::open(p).with_context(|| format!("{}", p.display()))?;
        let b = zstd::stream::decode_all(f).with_context(|| format!("{}", p.display()))?;
        Ok(String::from_utf8(b).with_context(|| format!("{}", p.display()))?)
    } else {
        std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))
    }
}

/// `GEN.1.1` → (0, 1, 1).
pub fn vkey(s: &str) -> Option<VKey> {
    let mut it = s.split('.');
    let b = book_of(it.next()?)?;
    Some((b, it.next()?.parse().ok()?, it.next()?.parse().ok()?))
}

pub fn show(k: VKey) -> String {
    format!("{}.{}.{}", code(k.0), k.1, k.2)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Split {
    Train,
    Dev,
    Test,
}

impl Split {
    pub fn name(self) -> &'static str {
        match self {
            Split::Train => "train",
            Split::Dev => "dev",
            Split::Test => "test",
        }
    }
    pub fn parse(s: &str) -> Option<Split> {
        match s {
            "train" => Some(Split::Train),
            "dev" => Some(Split::Dev),
            "test" => Some(Split::Test),
            _ => None,
        }
    }
}

/// By book: the books at places 3 and 8 of every ten are the test (NUM, 1SA, 2CH, PSA, JER, JOL, NAM, MAL), place 5
/// the dev (JOS, NEH, EZK, ZEP), the rest train. Prose and poetry fall on every side.
pub fn split_of(book: u8) -> Split {
    match book % 10 {
        3 | 8 => Split::Test,
        5 => Split::Dev,
        _ => Split::Train,
    }
}

/// A pool's verses: the sentences of each verse joined by one space (as the verse was before the cut).
pub fn read_pool(path: &Path) -> Result<BTreeMap<VKey, String>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let vpath = format!("{}.verses", path.display());
    let vtext = std::fs::read_to_string(&vpath).with_context(|| vpath.clone())?;
    let verse_of: BTreeMap<&str, &str> = vtext.lines().filter_map(|l| l.split_once('\t')).collect();
    let mut out: BTreeMap<VKey, String> = BTreeMap::new();
    for line in text.lines() {
        let Some((id, s)) = line.split_once('\t') else { continue };
        let Some(k) = verse_of.get(id).and_then(|v| vkey(v)) else { continue };
        let v = out.entry(k).or_default();
        if !v.is_empty() {
            v.push(' ');
        }
        v.push_str(s);
    }
    out.retain(|_, v| !v.trim().is_empty());
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_by_book() {
        let test: Vec<&str> = (0..39u8).filter(|&b| split_of(b) == Split::Test).map(|b| BOOKS[b as usize]).collect();
        assert_eq!(test, ["NUM", "1SA", "2CH", "PSA", "JER", "JOL", "NAM", "MAL"]);
        let dev: Vec<&str> = (0..39u8).filter(|&b| split_of(b) == Split::Dev).map(|b| BOOKS[b as usize]).collect();
        assert_eq!(dev, ["JOS", "NEH", "EZK", "ZEP"]);
        assert_eq!(vkey("1SA.3.10"), Some((8, 3, 10)));
        // the New Testament goes on after the Hebrew Bible, split by the same rule
        assert_eq!(vkey("REV.22.21"), Some((65, 22, 21)));
        let nt_dev: Vec<&str> = (39..66u8).filter(|&b| split_of(b) == Split::Dev).map(code).collect();
        assert_eq!(nt_dev, ["1CO", "TIT", "REV"]);
        assert_eq!(show((39, 1, 1)), "MAT.1.1");
        assert_eq!(vkey("TOB.1.1"), Some((66, 1, 1)));
    }
}
