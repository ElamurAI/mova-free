//! The labelled pool: segments of every variety from open editions, split into train / dev / test by document.
//!
//! Sources (selected through the scripture registry `texts.tsv`; only rows with scope `open` are read, so no
//! non-commercial or unknown-licence edition enters the pool):
//! - `hbo`, `arc`: MACULA Hebrew lowfat (WLC; CC BY 4.0), every word marked `lang="H"` or `"A"`; a segment is a
//!   run of words of one language inside a verse. Document = book, chapter and block of ten verses.
//! - `arc-x-targum`: the targums (Onkelos, Jonathan, the Writings; PD and CC BY-SA). Document = work and chapter.
//! - `tmr`: the Bavli (Wikisource, CC BY-SA), the gemara between `גמ׳` and the next `מתני׳`; the Mishnah inside it
//!   goes to `hbo-x-mishnaic`. Document = tractate.
//! - `jpa`: the Yerushalmi (Guggenheimer, CC BY), the `הלכה:` sections; its `משנה:` sections go to `hbo-x-mishnaic`.
//! - `arc-x-zohar`: Zohar Chadash and the vocalised Tikkunei Zohar (PD), without Midrash ha-Ne'elam (Hebrew).
//! - `hbo-x-mishnaic`: the Mishnah (PD editions) and the Tosefta (CC BY), plus the two Talmud cases above.
//! - `he`: UD Hebrew IAHLTwiki and IAHLTknesset (CC BY-SA), their own train/dev/test split.
//! - `en`: Sefaria CC0 English translations of the Mishnah, only for the mixed-document measurement.
//!
//! The labels are by corpus, not by sentence: the Talmuds quote Hebrew baraitot inside Aramaic, the Yerushalmi's
//! halakha is half Hebrew. The measurements report this noise rather than hide it.

use crate::{fnv, Lang};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Example {
    pub id: String,
    pub lang: Lang,
    pub doc: String,
    pub licence: String,
    pub text: String,
}

/// Which split a document belongs to: 1 in 10 test, 1 in 10 dev, the rest train.
pub fn split_of(doc: &str) -> &'static str {
    match fnv(doc) % 10 {
        0 => "test",
        1 => "dev",
        _ => "train",
    }
}

const MAX_WORDS: usize = 25;
const MIN_WORDS: usize = 3;

/// Strip HTML tags, entities, bracketed references and Sefaria's section headings.
pub fn clean(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth = 0i32;
    for c in s.chars() {
        match c {
            '<' => depth += 1,
            '>' if depth > 0 => {
                depth -= 1;
                out.push(' ');
            }
            _ if depth > 0 => {}
            _ => out.push(c),
        }
    }
    let out = out.replace("&nbsp;", " ").replace("&thinsp;", " ").replace("&amp;", "&");
    // bracketed references and editorial notes: (בראשית א׳:ג׳), [ ... ]
    let mut res = String::with_capacity(out.len());
    let mut skip: Option<char> = None;
    let cs: Vec<char> = out.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if let Some(close) = skip {
            if c == close {
                skip = None;
            }
            i += 1;
            continue;
        }
        if c == '(' || c == '[' || c == '{' {
            let close = match c { '(' => ')', '[' => ']', _ => '}' };
            if cs[i..].iter().take(60).any(|&d| d == close) {
                skip = Some(close);
                i += 1;
                continue;
            }
        }
        res.push(c);
        i += 1;
    }
    res.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cut a cleaned text into chunks: sentences at . : ׃ ? ! ; and then at most MAX_WORDS words.
pub fn chunks(text: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    let flush = |cur: &mut Vec<&str>, out: &mut Vec<String>| {
        let heb = cur.iter().filter(|w| w.chars().any(crate::is_heb_letter) || w.chars().any(|c| c.is_alphabetic())).count();
        if heb >= MIN_WORDS {
            out.push(cur.join(" "));
        }
        cur.clear();
    };
    for w in text.split_whitespace() {
        cur.push(w);
        let end = w.ends_with(['.', ':', '׃', '?', '!', ';']);
        if end || cur.len() >= max {
            flush(&mut cur, &mut out);
        }
    }
    flush(&mut cur, &mut out);
    out
}

/// Every string in a Sefaria `text` value, with the top-level key (chapter, daf or parasha).
fn strings(v: &Value, top: &str, out: &mut Vec<(String, String)>) {
    match v {
        Value::String(s) => out.push((top.to_string(), s.clone())),
        Value::Array(a) => {
            for (i, x) in a.iter().enumerate() {
                let t = if top.is_empty() { i.to_string() } else { top.to_string() };
                strings(x, &t, out);
            }
        }
        Value::Object(m) => {
            for (k, x) in m {
                let t = if top.is_empty() { k.clone() } else { top.to_string() };
                strings(x, &t, out);
            }
        }
        _ => {}
    }
}

fn read_sefaria(path: &Path) -> Vec<(String, String)> {
    let Ok(s) = std::fs::read_to_string(path) else { return Vec::new() };
    let Ok(v) = serde_json::from_str::<Value>(&s) else { return Vec::new() };
    let mut out = Vec::new();
    if let Some(t) = v.get("text") {
        strings(t, "", &mut out);
    }
    out
}

/// The tractate name normalised across Mishnah, Tosefta, Bavli and Yerushalmi folders.
fn tractate(work: &str) -> String {
    let w = work.trim_start_matches("Mishnah ").trim_start_matches("Jerusalem Talmud ").trim_start_matches("Tosefta ");
    let w = w.split(" (").next().unwrap_or(w);
    w.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase()
}

/// The work folder: the parent of the `Hebrew` / `English` folder.
fn work_of(rel: &str) -> String {
    let parts: Vec<&str> = rel.split('/').collect();
    let n = parts.len();
    if n >= 3 { parts[n - 3].to_string() } else { rel.to_string() }
}

pub struct Builder {
    pub raw: PathBuf,
    pub out: Vec<Example>,
}

impl Builder {
    fn push(&mut self, lang: Lang, doc: &str, licence: &str, id_base: &str, text: &str) {
        for (i, c) in chunks(text, MAX_WORDS).into_iter().enumerate() {
            self.out.push(Example {
                id: format!("{id_base}.{i}"),
                lang,
                doc: doc.to_string(),
                licence: licence.to_string(),
                text: c,
            });
        }
    }

    /// The scripture registry rows: (relative path, lang, mixed, licence, scope).
    fn registry(&self) -> Vec<(String, String, String, String, String)> {
        let p = self.raw.join("scripture/texts.tsv");
        let s = std::fs::read_to_string(&p).unwrap_or_default();
        s.lines()
            .skip(1)
            .filter_map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                (f.len() > 15).then(|| (f[0].to_string(), f[9].to_string(), f[10].to_string(), f[14].to_string(), f[15].to_string()))
            })
            .collect()
    }

    pub fn build(raw: &Path) -> Builder {
        let mut b = Builder { raw: raw.to_path_buf(), out: Vec::new() };
        b.macula();
        let reg = b.registry();
        for (rel, lang, mixed, lic, scope) in &reg {
            if scope != "open" || lic.contains("NC") || lic == "unknown" || !rel.ends_with(".json") {
                continue;
            }
            let path = b.raw.join("scripture").join(rel);
            let work = work_of(rel);
            match (lang.as_str(), mixed.as_str()) {
                ("arc-x-targum", _) if rel.contains("/Targum/") => {
                    for (top, s) in read_sefaria(&path) {
                        let doc = format!("targum:{work}:{top}");
                        let id = format!("tg:{}:{top}", fnv(rel) % 100000);
                        b.push(Lang::ArcTargum, &doc, lic, &format!("{id}:{}", fnv(&s) % 1000000), &clean(&s));
                    }
                }
                ("arc-x-zohar", _) => {
                    for (top, s) in read_sefaria(&path) {
                        if top.contains("Ne'elam") || top.contains("Midrash Rut") {
                            continue;
                        }
                        let doc = format!("zohar:{work}:{top}");
                        b.push(Lang::ArcZohar, &doc, lic, &format!("zh:{}:{}", fnv(rel) % 100000, fnv(&s) % 10000000), &clean(&s));
                    }
                }
                ("hbo-x-mishnaic", "") if rel.contains("/Mishnah/") || rel.contains("/Tosefta/") => {
                    let doc = format!("tractate:{}", tractate(&work));
                    for (top, s) in read_sefaria(&path) {
                        b.push(Lang::HboMishnaic, &doc, lic, &format!("mh:{}:{top}:{}", fnv(rel) % 100000, fnv(&s) % 10000000), &clean(&s));
                    }
                }
                ("tmr", _) if rel.contains("/Talmud/Bavli/") => b.talmud(&path, rel, &work, lic, ["מתני׳", "מתני'"], ["גמ׳", "גמ'"], Lang::Tmr),
                ("jpa", _) if rel.contains("/Talmud/Yerushalmi/") => b.talmud(&path, rel, &work, lic, ["משנה:", "משנה"], ["הלכה:", "הלכה"], Lang::Jpa),
                ("en", _) if lic == "CC0" && rel.contains("/Mishnah/") && b.out.iter().filter(|e| e.lang == Lang::En).count() < 6000 => {
                    for (top, s) in read_sefaria(&path) {
                        let doc = format!("en:{work}:{top}");
                        b.push(Lang::En, &doc, lic, &format!("en:{}:{}", fnv(rel) % 100000, fnv(&s) % 10000000), &clean(&s));
                    }
                }
                _ => {}
            }
        }
        b.ud();
        // the same text in two editions makes the same id; keep one
        let mut seen = std::collections::BTreeSet::new();
        b.out.retain(|e| seen.insert((e.lang, e.text.clone())));
        b
    }

    /// A Talmud tractate: segments switch between Mishnah and gemara at the section markers.
    fn talmud(&mut self, path: &Path, rel: &str, work: &str, lic: &str, mishnah: [&str; 2], gemara: [&str; 2], lang: Lang) {
        let doc = format!("tractate:{}", tractate(work));
        let mut state = lang;
        // a Mishnah section opens only with the marker as the segment's first word (inside the gemara מתני׳
        // also means "our Mishnah"), and it is short: after 10 segments without a gemara marker the state goes
        // back to the gemara
        let mut since = 0;
        for (top, s) in read_sefaria(path) {
            let c = clean(&s);
            since += 1;
            if state == Lang::HboMishnaic && since > 10 {
                state = lang;
            }
            let mut pieces: Vec<(Lang, Vec<&str>)> = vec![(state, Vec::new())];
            for (i, w) in c.split_whitespace().enumerate() {
                let new = if i == 0 && mishnah.contains(&w) {
                    since = 0;
                    Some(Lang::HboMishnaic)
                } else if gemara.contains(&w) {
                    Some(lang)
                } else {
                    None
                };
                match new {
                    Some(l) => {
                        state = l;
                        pieces.push((l, Vec::new()));
                    }
                    None => pieces.last_mut().unwrap().1.push(w),
                }
            }
            for (l, ws) in pieces {
                if ws.is_empty() {
                    continue;
                }
                let t = ws.join(" ");
                let id = format!("{}:{}:{top}:{}", if lang == Lang::Tmr { "bavli" } else { "yeru" }, fnv(rel) % 100000, fnv(&t) % 10000000);
                self.push(l, &doc, lic, &id, &t);
            }
        }
    }

    /// MACULA Hebrew: words with `lang` H or A, grouped into same-language runs inside a verse.
    fn macula(&mut self) {
        use quick_xml::events::Event;
        let dir = self.raw.join("lexres/treebanks/macula-hebrew/WLC/lowfat");
        let Ok(rd) = std::fs::read_dir(&dir) else { return };
        let mut files: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "xml")).collect();
        files.sort();
        for f in files {
            let Ok(xml) = std::fs::read_to_string(&f) else { continue };
            let mut r = quick_xml::Reader::from_str(&xml);
            // verse -> words (surface, lang)
            let mut verses: BTreeMap<(u32, u32, String), Vec<(String, char)>> = BTreeMap::new();
            let mut order: Vec<(u32, u32, String)> = Vec::new();
            let mut cur: Option<(String, char, String)> = None; // ref, lang, after
            let mut word = String::new();
            let mut word_lang = 'H';
            let mut last_key: Option<(u32, u32, String)> = None;
            loop {
                match r.read_event() {
                    Ok(Event::Start(e)) if e.name().as_ref() == "w" => {
                        let mut rf = String::new();
                        let mut lg = 'H';
                        let mut after = String::new();
                        for a in e.attributes().flatten() {
                            let v = unescape(&a.value);
                            match a.key.as_ref() {
                                "ref" => rf = v,
                                "lang" => lg = v.chars().next().unwrap_or('H'),
                                "after" => after = v,
                                _ => {}
                            }
                        }
                        cur = Some((rf, lg, after));
                    }
                    Ok(Event::Text(t)) => {
                        if let Some((_, lg, _)) = &cur {
                            let txt = unescape(t.as_ref());
                            if word.is_empty() {
                                word_lang = *lg;
                            }
                            word.push_str(txt.trim());
                        }
                    }
                    Ok(Event::End(e)) if e.name().as_ref() == "w" => {
                        if let Some((rf, _, after)) = cur.take() {
                            // "GEN 1:1!3" -> book, chapter, verse
                            let v = rf.split('!').next().unwrap_or("");
                            let (book, cv) = v.split_once(' ').unwrap_or(("", ""));
                            let (ch, vs) = cv.split_once(':').unwrap_or(("0", "0"));
                            let key = (ch.parse().unwrap_or(0), vs.parse().unwrap_or(0), book.to_string());
                            if last_key.as_ref() != Some(&key) {
                                if !verses.contains_key(&key) {
                                    order.push(key.clone());
                                }
                                last_key = Some(key.clone());
                            }
                            let entry = verses.entry(key).or_default();
                            // an empty `after` joins the unit to the next one (prefixes, the article)
                            if !after.is_empty() {
                                entry.push((std::mem::take(&mut word), word_lang));
                            }
                        }
                    }
                    Ok(Event::Eof) | Err(_) => break,
                    _ => {}
                }
            }
            for key in order {
                let (ch, vs, book) = &key;
                let words = &verses[&key];
                // Biblical Aramaic is small (about 270 verses): its documents are single verses, so dev and test
                // get enough of it; Hebrew verses go in blocks of ten
                let aramaic = words.iter().any(|w| w.1 == 'A');
                let doc = if aramaic { format!("macula:{book}:{ch}:{vs}:v") } else { format!("macula:{book}:{ch}:{}", vs / 10) };
                let mut i = 0;
                while i < words.len() {
                    let lg = words[i].1;
                    let mut j = i + 1;
                    while j < words.len() && words[j].1 == lg {
                        j += 1;
                    }
                    let text: Vec<&str> = words[i..j].iter().map(|w| w.0.as_str()).filter(|w| !w.is_empty()).collect();
                    let lang = if lg == 'A' { Lang::Arc } else { Lang::Hbo };
                    let id = format!("wlc:{book}:{ch}:{vs}:{i}");
                    self.push(lang, &doc, "CC-BY-4.0", &id, &text.join(" "));
                    i = j;
                }
            }
        }
    }

    /// Modern Hebrew: the `# text =` lines of the UD IAHLT treebanks, in their own split.
    fn ud(&mut self) {
        for tb in ["UD_Hebrew-IAHLTwiki", "UD_Hebrew-IAHLTknesset"] {
            let dir = self.raw.join("lexres/treebanks/ud-2.18").join(tb);
            for split in ["train", "dev", "test"] {
                let name = format!("he_{}-ud-{split}.conllu", tb.trim_start_matches("UD_Hebrew-").to_lowercase());
                let Ok(s) = std::fs::read_to_string(dir.join(&name)) else { continue };
                let mut id = String::new();
                for l in s.lines() {
                    if let Some(x) = l.strip_prefix("# sent_id = ") {
                        id = x.trim().to_string();
                    }
                    if let Some(t) = l.strip_prefix("# text = ") {
                        // the document is the split itself, so UD's own split is kept
                        let doc = format!("ud:{split}");
                        self.push(Lang::He, &doc, "CC-BY-SA-4.0", &format!("ud:{tb}:{id}"), t);
                    }
                }
            }
        }
    }
}

/// The split of an example: UD Hebrew keeps its own split, everything else is split by document hash.
pub fn split_of_example(e: &Example) -> &'static str {
    match e.doc.strip_prefix("ud:") {
        Some("train") => "train",
        Some("dev") => "dev",
        Some("test") => "test",
        _ => split_of(&e.doc),
    }
}

fn unescape(s: &str) -> String {
    s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&")
}

pub fn write_pool(path: &Path, ex: &[&Example]) -> std::io::Result<()> {
    use std::io::Write;
    let mut w = std::io::BufWriter::new(std::fs::File::create(path)?);
    writeln!(w, "id\tlang\tdoc\tlicence\ttext")?;
    for e in ex {
        writeln!(w, "{}\t{}\t{}\t{}\t{}", e.id, e.lang.code(), e.doc, e.licence, e.text.replace('\t', " "))?;
    }
    Ok(())
}

pub fn read_pool(path: &Path) -> Result<Vec<Example>, String> {
    let s = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(s.lines()
        .skip(1)
        .filter_map(|l| {
            let f: Vec<&str> = l.splitn(5, '\t').collect();
            if f.len() != 5 {
                return None;
            }
            Some(Example { id: f[0].into(), lang: Lang::parse(f[1])?, doc: f[2].into(), licence: f[3].into(), text: f[4].into() })
        })
        .collect())
}

#[cfg(test)]
mod wave31 {
    use super::*;
    #[test]
    fn cleaning_chunk_and_split_boundaries() {
        assert_eq!(clean("a<b>x</b> (note) c &amp; d"), "a x c & d");
        assert_eq!(clean("a<broken"), "a");
        assert_eq!(clean("a [unclosed b"), "a [unclosed b");
        assert!(chunks("one two", 25).is_empty());
        assert_eq!(
            chunks("one two three. four five six", 25),
            ["one two three.", "four five six"]
        );
        assert!(chunks("one two three", 0).is_empty());
        let e = Example {
            id: "i".into(),
            lang: Lang::He,
            doc: "ud:dev".into(),
            licence: "synthetic".into(),
            text: "".into(),
        };
        assert_eq!(split_of_example(&e), "dev");
        assert_eq!(tractate("Jerusalem Talmud Test Name (edition)"), "testname");
        let mut out = Vec::new();
        strings(&serde_json::json!([["a", null], ["b"]]), "", &mut out);
        assert_eq!(out, [("0".into(), "a".into()), ("1".into(), "b".into())]);
    }
}
