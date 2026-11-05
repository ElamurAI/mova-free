//! The Tanakh in three languages, verse by verse (operator, 18.10: «the whole Tanakh, and the Talmud later, in three
//! languages: Hebrew, Aramaic, English»).
//!
//!   macula targums <sefaria Tanakh/Targum dir> <text-dir>
//!
//! The Targums as text layers keyed by verse, from Sefaria's export: Targum Onkelos on the Torah, Targum Jonathan on
//! the Prophets, and the later Targums of the Writings (Psalms, Proverbs, Job, the Song, Ruth, Lamentations,
//! Ecclesiastes, Esther, Chronicles). Only editions whose `license` is «Public Domain» are copied; any other is left
//! out with a note (CC BY-SA and NC editions are referenced in the corpus README, never carried). Out:
//! `<text-dir>/<layer>/<BOOK>.tsv` (`BOOK.C.V` → text, markup taken off, NFC) and `<layer>/SOURCES.tsv`.
//!
//!   macula tanakh3 <text-dir> <en-pool.tsv> <out-dir>
//!
//! The verse-aligned index: one TSV per book, a row per verse (the Hebrew numbering) with the Hebrew sentence ids
//! (`tanakh-wlc-macula`), the Aramaic (Biblical Aramaic sentence ids from `tanakh-arc-macula`, and the Targum verse
//! as `<layer>:BOOK.C.V`) and the English sentence ids (the pool's `.verses` map from `school-sents usfm`).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

/// The books in the order of the Tanakh: Torah, Prophets, Writings.
pub const BOOKS: [&str; 39] = [
    "GEN", "EXO", "LEV", "NUM", "DEU", "JOS", "JDG", "1SA", "2SA", "1KI", "2KI", "ISA", "JER", "EZK", "HOS", "JOL", "AMO", "OBA", "JON", "MIC", "NAM", "HAB", "ZEP", "HAG", "ZEC", "MAL", "PSA", "PRO", "JOB", "SNG", "RUT", "LAM", "ECC", "EST", "DAN", "EZR",
    "NEH", "1CH", "2CH",
];

/// (layer, book, Sefaria title): the editions read. The file is `<title dir>/Hebrew/<edition>.json` under the layer's
/// Sefaria folder.
const TARGUMS: [(&str, &str, &str, &str); 36] = [
    ("onkelos", "GEN", "Onkelos/Torah/Onkelos Genesis", "Onkelos Genesis"),
    ("onkelos", "EXO", "Onkelos/Torah/Onkelos Exodus", "Onkelos Exodus"),
    ("onkelos", "LEV", "Onkelos/Torah/Onkelos Leviticus", "Onkelos Leviticus"),
    ("onkelos", "NUM", "Onkelos/Torah/Onkelos Numbers", "Onkelos Numbers"),
    ("onkelos", "DEU", "Onkelos/Torah/Onkelos Deuteronomy", "Onkelos Deuteronomy"),
    ("jonathan", "JOS", "Targum Jonathan/Prophets/Targum Jonathan on Joshua", "Mikraot Gedolot"),
    ("jonathan", "JDG", "Targum Jonathan/Prophets/Targum Jonathan on Judges", "Mikraot Gedolot"),
    ("jonathan", "1SA", "Targum Jonathan/Prophets/Targum Jonathan on I Samuel", "Mikraot Gedolot"),
    ("jonathan", "2SA", "Targum Jonathan/Prophets/Targum Jonathan on II Samuel", "Mikraot Gedolot"),
    ("jonathan", "1KI", "Targum Jonathan/Prophets/Targum Jonathan on I Kings", "Mikraot Gedolot"),
    ("jonathan", "2KI", "Targum Jonathan/Prophets/Targum Jonathan on II Kings", "Mikraot Gedolot"),
    ("jonathan", "ISA", "Targum Jonathan/Prophets/Targum Jonathan on Isaiah", "Mikraot Gedolot"),
    ("jonathan", "JER", "Targum Jonathan/Prophets/Targum Jonathan on Jeremiah", "Mikraot Gedolot"),
    ("jonathan", "EZK", "Targum Jonathan/Prophets/Targum Jonathan on Ezekiel", "Mikraot Gedolot"),
    ("jonathan", "HOS", "Targum Jonathan/Prophets/Targum Jonathan on Hosea", "Mikraot Gedolot"),
    ("jonathan", "JOL", "Targum Jonathan/Prophets/Targum Jonathan on Joel", "Mikraot Gedolot"),
    ("jonathan", "AMO", "Targum Jonathan/Prophets/Targum Jonathan on Amos", "Mikraot Gedolot"),
    ("jonathan", "OBA", "Targum Jonathan/Prophets/Targum Jonathan on Obadiah", "Mikraot Gedolot"),
    ("jonathan", "JON", "Targum Jonathan/Prophets/Targum Jonathan on Jonah", "Mikraot Gedolot"),
    ("jonathan", "MIC", "Targum Jonathan/Prophets/Targum Jonathan on Micah", "Mikraot Gedolot"),
    ("jonathan", "NAM", "Targum Jonathan/Prophets/Targum Jonathan on Nahum", "Mikraot Gedolot"),
    ("jonathan", "HAB", "Targum Jonathan/Prophets/Targum Jonathan on Habakkuk", "Mikraot Gedolot"),
    ("jonathan", "ZEP", "Targum Jonathan/Prophets/Targum Jonathan on Zephaniah", "Mikraot Gedolot"),
    ("jonathan", "HAG", "Targum Jonathan/Prophets/Targum Jonathan on Haggai", "Mikraot Gedolot"),
    ("jonathan", "ZEC", "Targum Jonathan/Prophets/Targum Jonathan on Zechariah", "Mikraot Gedolot"),
    ("jonathan", "MAL", "Targum Jonathan/Prophets/Targum Jonathan on Malachi", "Mikraot Gedolot"),
    ("writings", "PSA", "Aramaic Targum/Writings/Aramaic Targum to Psalms", "Mikraot Gedolot"),
    ("writings", "PRO", "Aramaic Targum/Writings/Aramaic Targum to Proverbs", "Mikraot Gedolot"),
    ("writings", "JOB", "Aramaic Targum/Writings/Aramaic Targum to Job", "Mikraot Gedolot"),
    ("writings", "SNG", "Aramaic Targum/Writings/Aramaic Targum to Song of Songs", "Mikraot Gedolot"),
    ("writings", "RUT", "Aramaic Targum/Writings/Aramaic Targum to Ruth", "Mikraot Gedolot"),
    ("writings", "LAM", "Aramaic Targum/Writings/Aramaic Targum to Lamentations", "Mikraot Gedolot"),
    ("writings", "ECC", "Aramaic Targum/Writings/Aramaic Targum to Ecclesiastes", "Mikraot Gedolot"),
    ("writings", "EST", "Aramaic Targum/Writings/Aramaic Targum to Esther", "Mikraot Gedolot"),
    ("writings", "1CH", "Targum Jerusalem/Targum of I Chronicles", "Wikisource Aramaic Targum to Chronicles"),
    ("writings", "2CH", "Targum Jerusalem/Targum of II Chronicles", "Wikisource Aramaic Targum to Chronicles"),
];

/// The folder of a Targum layer in the corpus.
pub fn layer_dir(layer: &str) -> String {
    match layer {
        "writings" => "targum-writings".into(),
        l => format!("targum-{l}"),
    }
}

/// The verses of one Sefaria edition: (chapter, verse, text), markup off, NFC; empty verses left out. An edition
/// whose licence is not «Public Domain» gives an error: it is never copied.
fn edition(json: &str, book: &str) -> Result<(Vec<(String, String)>, String, String), String> {
    let d: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let licence = d["license"].as_str().unwrap_or("").to_string();
    if licence != "Public Domain" {
        return Err(format!("licence «{licence}», not public domain"));
    }
    let mut out = Vec::new();
    for (c, ch) in d["text"].as_array().ok_or("no text")?.iter().enumerate() {
        for (v, t) in ch.as_array().map(|a| a.as_slice()).unwrap_or(&[]).iter().enumerate() {
            let t = macula::html_text(t.as_str().unwrap_or(""), "strong").0.nfc().collect::<String>();
            if !t.is_empty() {
                out.push((format!("{book}.{}.{}", c + 1, v + 1), t.replace('\t', " ")));
            }
        }
    }
    Ok((out, d["versionTitle"].as_str().unwrap_or("").into(), d["versionSource"].as_str().unwrap_or("").into()))
}

pub fn targums(src: &str, text_dir: &str) {
    let mut sources: BTreeMap<String, String> = BTreeMap::new();
    let mut count: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for (layer, book, title, ed) in TARGUMS.iter() {
        let path = Path::new(src).join(title).join("Hebrew").join(format!("{ed}.json"));
        let json = match fs::read_to_string(&path) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("{book}: {}: {e} — left out", path.display());
                continue;
            }
        };
        let (verses, vt, vs) = match edition(&json, book) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{book}: {} — {e}; left out, only referenced", path.display());
                continue;
            }
        };
        let dir = Path::new(text_dir).join(layer_dir(layer));
        fs::create_dir_all(&dir).expect("layer dir");
        let body: String = verses.iter().map(|(v, t)| format!("{v}\t{t}\n")).collect();
        fs::write(dir.join(format!("{book}.tsv")), body).expect("write targum");
        let rel = path.strip_prefix(src).unwrap_or(&path).display().to_string();
        sources.entry(layer.to_string()).or_insert_with(|| "book\tverses\tedition\tsource\tlicence\tfile\n".into()).push_str(&format!("{book}\t{}\t{vt}\t{vs}\tPublic Domain\t{rel}\n", verses.len()));
        let e = count.entry(layer.to_string()).or_default();
        e.0 += 1;
        e.1 += verses.len();
    }
    for (layer, t) in &sources {
        fs::write(Path::new(text_dir).join(layer_dir(layer)).join("SOURCES.tsv"), t).expect("write sources");
    }
    for (layer, (b, v)) in &count {
        println!("{}: {b} books, {v} verses", layer_dir(layer));
    }
}

/// Book, chapter, verse as a sortable key in the order of the Tanakh: GEN.1.1 → (0, 1, 1).
pub fn key(v: &str) -> (usize, u32, u32) {
    let mut p = v.split('.');
    let b = p.next().and_then(|b| BOOKS.iter().position(|x| *x == b)).unwrap_or(BOOKS.len());
    (b, p.next().and_then(|x| x.parse().ok()).unwrap_or(0), p.next().and_then(|x| x.parse().ok()).unwrap_or(0))
}

/// `id\tverse verse …` → verse → ids, in the file's order.
fn by_verse(map: &str, into: &mut BTreeMap<String, Vec<String>>) {
    for l in map.lines() {
        if let Some((id, vs)) = l.split_once('\t') {
            for v in vs.split_whitespace() {
                into.entry(v.to_string()).or_default().push(id.to_string());
            }
        }
    }
}

pub fn index(text_dir: &str, pool: &str, out: &str) {
    let t = Path::new(text_dir);
    let read = |p: &Path| fs::read_to_string(p).unwrap_or_default();
    let (mut hbo, mut arc, mut en) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
    by_verse(&read(&t.join("tanakh-wlc-macula/verses.tsv")), &mut hbo);
    by_verse(&read(&t.join("tanakh-arc-macula/verses.tsv")), &mut arc);
    by_verse(&read(Path::new(&format!("{pool}.verses"))), &mut en);
    for layer in ["onkelos", "jonathan", "writings"] {
        let short = if layer == "writings" { "targum" } else { layer };
        let Ok(files) = fs::read_dir(t.join(layer_dir(layer))) else { continue };
        let mut files: Vec<_> = files.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "tsv") && !p.ends_with("SOURCES.tsv")).collect();
        files.sort();
        for f in files {
            for l in read(&f).lines() {
                if let Some((v, _)) = l.split_once('\t') {
                    arc.entry(v.to_string()).or_insert_with(Vec::new).push(format!("{short}:{v}"));
                }
            }
        }
    }
    let mut verses: BTreeSet<((usize, u32, u32), String)> = BTreeSet::new();
    for v in hbo.keys().chain(arc.keys()).chain(en.keys()) {
        verses.insert((key(v), v.clone()));
    }
    fs::create_dir_all(out).expect("out dir");
    let mut books: BTreeMap<usize, String> = BTreeMap::new();
    // per book group: verses, with Hebrew, with Aramaic, with English, with all three
    let mut stats: BTreeMap<&str, [usize; 5]> = BTreeMap::new();
    let get = |m: &BTreeMap<String, Vec<String>>, v: &str| m.get(v).map(|x| x.join(" ")).unwrap_or_default();
    for ((k, _, _), v) in &verses {
        if *k >= BOOKS.len() {
            eprintln!("verse {v}: unknown book, left out");
            continue;
        }
        let (h, a, e) = (get(&hbo, v), get(&arc, v), get(&en, v));
        books.entry(*k).or_insert_with(|| "verse\thbo\tarc\ten\n".into()).push_str(&format!("{v}\t{h}\t{a}\t{e}\n"));
        let group = if *k < 5 { "1 Torah" } else if *k < 26 { "2 Prophets" } else { "3 Writings" };
        let s = stats.entry(group).or_default();
        s[0] += 1;
        s[1] += usize::from(!h.is_empty());
        s[2] += usize::from(!a.is_empty());
        s[3] += usize::from(!e.is_empty());
        s[4] += usize::from(!h.is_empty() && !a.is_empty() && !e.is_empty());
    }
    for (k, body) in &books {
        fs::write(Path::new(out).join(format!("{}.tsv", BOOKS[*k])), body).expect("write index");
    }
    println!("{} books → {out}", books.len());
    println!("group\tverses\thbo\tarc\ten\tall three");
    for (g, s) in &stats {
        println!("{g}\t{}\t{}\t{}\t{}\t{}", s[0], s[1], s[2], s[3], s[4]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_domain_only() {
        let pd = r#"{"license": "Public Domain", "versionTitle": "T", "versionSource": "S", "text": [["<small>א</small> ב", ""], ["ג"]]}"#;
        let (v, t, s) = edition(pd, "GEN").unwrap();
        assert_eq!(v, vec![("GEN.1.1".to_string(), "א ב".to_string()), ("GEN.2.1".to_string(), "ג".to_string())]);
        assert_eq!((t.as_str(), s.as_str()), ("T", "S"));
        // negative control: a CC BY-SA or NC edition is refused
        assert!(edition(&pd.replace("Public Domain", "CC-BY-SA"), "GEN").is_err());
        assert!(edition(&pd.replace("Public Domain", "CC-BY-NC"), "GEN").is_err());
    }

    #[test]
    fn tanakh_order() {
        assert!(key("DEU.34.12") < key("JOS.1.1"));
        assert!(key("MAL.3.24") < key("PSA.1.1"));
        assert!(key("PSA.9.1") < key("PSA.10.1"));
        assert!(key("2CH.36.23") < key("XYZ.1.1"));
    }
}
