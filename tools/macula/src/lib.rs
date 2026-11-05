//! The parts of `macula` other crates read: the pragmatic rules for Hebrew trees (`prag`), the helpers they use, and the
//! readers of the verses and dictionary entries the dictionary builder shares.
//! The converter itself is the binary (`main.rs`).

pub mod prag;

/// Verbs whose next clause, with no conjunction between, is what was said (UD: `ccomp` of the verb).
pub const SAYING: [&str; 6] = ["אמר", "דבר", "צוה", "ענה", "שאל", "קרא"];

/// The consonants of a pointed word: lemmas compare by letters, since the order of points varies between sources.
pub fn consonants(s: &str) -> String {
    s.chars().filter(|c| ('\u{05D0}'..='\u{05EA}').contains(c)).collect()
}

// ---- reading the sources, shared with the dictionary builder ----------------------------------------------------

use std::collections::BTreeMap;
use std::fs;

pub fn unescape(s: &str) -> String {
    s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&")
}

/// The books of the Hebrew Bible in MACULA's order, with their names in Sefaria's references («Genesis 24:12»).
pub const BOOKS: [(&str, &str); 39] = [
    ("GEN", "Genesis"), ("EXO", "Exodus"), ("LEV", "Leviticus"), ("NUM", "Numbers"), ("DEU", "Deuteronomy"), ("JOS", "Joshua"),
    ("JDG", "Judges"), ("RUT", "Ruth"), ("1SA", "I Samuel"), ("2SA", "II Samuel"), ("1KI", "I Kings"), ("2KI", "II Kings"),
    ("1CH", "I Chronicles"), ("2CH", "II Chronicles"), ("EZR", "Ezra"), ("NEH", "Nehemiah"), ("EST", "Esther"), ("JOB", "Job"),
    ("PSA", "Psalms"), ("PRO", "Proverbs"), ("ECC", "Ecclesiastes"), ("SNG", "Song of Songs"), ("ISA", "Isaiah"), ("JER", "Jeremiah"),
    ("LAM", "Lamentations"), ("EZK", "Ezekiel"), ("DAN", "Daniel"), ("HOS", "Hosea"), ("JOL", "Joel"), ("AMO", "Amos"),
    ("OBA", "Obadiah"), ("JON", "Jonah"), ("MIC", "Micah"), ("NAM", "Nahum"), ("HAB", "Habakkuk"), ("ZEP", "Zephaniah"),
    ("HAG", "Haggai"), ("ZEC", "Zechariah"), ("MAL", "Malachi"),
];

/// Book order and chapter.verse as a sortable key: GEN.1.1 → (0, 1, 1).
pub fn verse_key(v: &str) -> (usize, u32, u32) {
    let mut p = v.split('.');
    let book = p.next();
    let b = BOOKS.iter().position(|x| Some(x.0) == book).unwrap_or(BOOKS.len());
    let c = p.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let n = p.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    (b, c, n)
}

/// The English verses of a pool cut by `school-sents usfm`: verse → text (its sentences joined).
pub fn english_verses(pool: &str) -> BTreeMap<String, String> {
    let text: BTreeMap<String, String> = fs::read_to_string(pool).expect("pool").lines().filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
    let mut v: BTreeMap<String, String> = BTreeMap::new();
    for l in fs::read_to_string(format!("{pool}.verses")).expect("verses map").lines() {
        if let Some((id, verse)) = l.split_once('\t') {
            if let Some(s) = text.get(id) {
                let e = v.entry(verse.to_string()).or_default();
                if !e.is_empty() {
                    e.push(' ');
                }
                e.push_str(s);
            }
        }
    }
    v
}

/// The Hebrew verses of our trees: verse → (pointed text, word for word); a sentence over several verses is left out.
pub fn hebrew_verses(trees: &str) -> BTreeMap<String, (String, String)> {
    let mut out = BTreeMap::new();
    let mut files: Vec<_> = fs::read_dir(trees).expect("trees").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "conllu")).collect();
    files.sort();
    for f in files {
        for block in fs::read_to_string(f).expect("ours").split("\n\n") {
            let meta = |k: &str| block.lines().find_map(|l| l.strip_prefix(k)).unwrap_or("").to_string();
            let (verse, heb) = (meta("# verse = "), meta("# text = "));
            if verse.is_empty() || verse.contains(' ') {
                continue;
            }
            let literal = block.lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.split('\t').nth(9)).filter_map(|m| m.split('|').find_map(|kv| kv.strip_prefix("Gloss="))).map(|g| g.replace('_', " ")).filter(|g| !g.is_empty()).collect::<Vec<_>>().join(" ");
            out.insert(verse, (heb, literal));
        }
    }
    out
}


/// Text of an HTML fragment: tags off, entities read; and the text inside `<tag>` (BDB's bold `strong`, Jastrow's
/// italic `i`) on its own.
pub fn html_text(h: &str, tag_name: &str) -> (String, String) {
    let (mut all, mut marked, mut tag, mut depth) = (String::new(), String::new(), String::new(), 0i32);
    let (open, close) = (tag_name.to_string(), format!("/{tag_name}"));
    let mut in_tag = false;
    for c in h.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
                let t = tag.trim().to_lowercase();
                if t == open || t.starts_with(&format!("{open} ")) {
                    depth += 1;
                } else if t == close {
                    depth -= 1;
                    marked.push(' ');
                }
                tag.clear();
            } else {
                tag.push(c);
            }
        } else if c == '<' {
            in_tag = true;
        } else {
            all.push(c);
            if depth > 0 {
                marked.push(c);
            }
        }
    }
    let clean = |s: &str| unescape(&s.replace("&nbsp;", " ")).split_whitespace().collect::<Vec<_>>().join(" ");
    (clean(&all), clean(&marked))
}

#[cfg(test)]
mod wave31 {
    use super::*;
    #[test]
    fn text_and_coordinate_fibers() {
        assert_eq!(consonants("a מֶלֶךְ 1"), "מלך");
        assert_eq!(unescape("&amp;lt;"), "&lt;");
        assert_eq!(verse_key("XYZ.bad.4294967296"), (39, 0, 0));
        assert_eq!(verse_key("GEN.1.2.extra"), (0, 1, 2));
        assert_eq!(
            html_text("a<strong x='y'>b<i>c</i></strong>d", "strong"),
            ("abcd".into(), "bc".into())
        );
        assert_eq!(html_text("a<broken", "i"), ("a".into(), "".into()));
        assert_eq!(html_text("", "i"), ("".into(), "".into()));
    }
}
