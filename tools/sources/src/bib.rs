//! ACL Anthology (unpacked `anthology+abstracts.bib`) → paper manifests and abstracts by topic.
//!
//!   sources bib <anthology.bib> <rules.tsv> <out-dir>
//!
//! rules.tsv — lines `topic \t slug-prefix \t field \t pattern`:
//! - field: `title`, `any` (title or abstract) or `booktitle`;
//! - pattern: case-insensitive substrings joined by " & " — all must match.
//!
//! A paper goes into a topic if at least one rule fires. Priority:
//! 1 — match in the title or the proceedings title (e.g. UDW papers); 2 — only in the abstract.
//!
//! Output per topic:
//! - `<topic>.jsonl` — manifest for `sources plan` (PDF from aclanthology.org; license: from 2016 CC BY 4.0,
//!   earlier CC BY-NC-SA 3.0);
//! - `<topic>-abstracts.md` — all found papers with abstracts, by year: this is what agents read.
//!
//! A paper may belong to several topics; its slug then takes the prefix of the first topic in the rules.
//!
//!   sources bib-ids <anthology.bib> <ids.txt> <out.jsonl>
//!
//! Papers from an explicit list of Anthology ids (line: `id`, `id \t topic` or `id \t topic \t priority`) →
//! a manifest of the same format. The default topic is the output file name without extension
//! (`treebanks.jsonl` → `treebanks`), which is also the slug prefix. Priority is 1…3, default 1:
//! `papers summarize` uses it to choose what to summarize first. An id missing from the bibliography is an error,
//! and the manifest is not written.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

/// Bibliography entry: type (`inproceedings`…), key and fields (field names lowercased).
pub struct Entry {
    pub kind: String,
    pub key: String,
    pub fields: BTreeMap<String, String>,
}

impl Entry {
    pub fn get(&self, k: &str) -> &str {
        self.fields.get(k).map_or("", String::as_str)
    }

    /// Anthology id from the `url` field (`https://aclanthology.org/2020.lrec-1.497/` → `2020.lrec-1.497`);
    /// empty if the URL is not from aclanthology.org.
    pub fn anthology_id(&self) -> &str {
        let url = self.get("url").trim_end_matches('/');
        if !url.contains("aclanthology.org") {
            return "";
        }
        url.rsplit('/').next().unwrap_or("")
    }

    /// First author's surname, as in BibTeX before the comma (`Nivre, Joakim and …` → `Nivre`).
    pub fn first_surname(&self) -> &str {
        let a = self.get("author").split(" and ").next().unwrap_or("");
        a.split(',').next().unwrap_or(a).trim()
    }
}

/// Parses BibTeX in the form ACL Anthology writes it: `@type{key,` and fields `name = "…"`
/// or `name = {…}` or a bare value (`month = may`), one per line or spanning several lines.
pub fn parse(text: &str) -> Vec<Entry> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut i = 0;
    while let Some(at) = text.get(i..).and_then(|t| t.find('@')) {
        i += at + 1;
        let Some(open) = text[i..].find('{') else { break };
        let kind = text[i..i + open].trim().to_lowercase();
        i += open + 1;
        let Some(comma) = text[i..].find(',') else { break };
        let key = text[i..i + comma].trim().to_string();
        i += comma + 1;
        let mut fields = BTreeMap::new();
        loop {
            while i < b.len() && (b[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= b.len() || b[i] == b'}' {
                i += 1;
                break;
            }
            let Some(eq) = text.get(i..).and_then(|t| t.find('=')) else { break };
            let name = text[i..i + eq].trim().to_lowercase();
            i += eq + 1;
            while i < b.len() && b[i] == b' ' {
                i += 1;
            }
            let value;
            match b.get(i) {
                Some(b'"') => {
                    let start = i + 1;
                    let mut depth = 0i32;
                    i += 1;
                    while i < b.len() {
                        match b[i] {
                            b'\\' => i += 1,
                            b'{' => depth += 1,
                            b'}' => depth -= 1,
                            b'"' if depth == 0 => break,
                            _ => {}
                        }
                        i += 1;
                    }
                    value = text[start..i.min(b.len())].to_string();
                    i += 1;
                }
                Some(b'{') => {
                    let start = i + 1;
                    let mut depth = 1i32;
                    i += 1;
                    while i < b.len() && depth > 0 {
                        match b[i] {
                            b'{' => depth += 1,
                            b'}' => depth -= 1,
                            _ => {}
                        }
                        i += 1;
                    }
                    // unclosed brace (bibtex snippet in a README) — value runs to the end of the text
                    value = if depth > 0 { text[start..].to_string() } else { text[start..i - 1].to_string() };
                }
                _ => {
                    let start = i;
                    while i < b.len() && b[i] != b',' && b[i] != b'\n' && b[i] != b'}' {
                        i += 1;
                    }
                    value = text[start..i].trim().to_string();
                }
            }
            // BibTeX concatenation `"7-8 " # jun`: tails after `#` are appended to the value, otherwise
            // `# jun` would swallow the next field (this is how `year` got lost in *SEM 2012)
            let mut value = value;
            loop {
                let mut j = i;
                while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
                    j += 1;
                }
                if j >= b.len() || b[j] != b'#' {
                    break;
                }
                j += 1;
                while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
                    j += 1;
                }
                let start = j;
                if b.get(j) == Some(&b'"') {
                    j += 1;
                    while j < b.len() && b[j] != b'"' {
                        j += 1;
                    }
                    value.push_str(&text[start + 1..j.min(b.len())]);
                    j += 1;
                } else {
                    while j < b.len() && (b[j] as char).is_ascii_alphanumeric() {
                        j += 1;
                    }
                    value.push_str(&text[start..j]);
                }
                i = j.min(b.len());
            }
            // comma after the value
            while i < b.len() && (b[i] == b',' || b[i] == b' ') {
                i += 1;
            }
            fields.insert(name, clean(&value));
        }
        out.push(Entry { kind, key, fields });
    }
    out
}

/// LaTeX accents → Unicode: `{\\"u}` → ü, `{\\c{s}}` → ş, `\\v{z}` → ž, `\\l` → ł.
fn accents(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    const MARK: &[(char, char)] = &[
        ('\'', '\u{301}'), ('`', '\u{300}'), ('^', '\u{302}'), ('"', '\u{308}'), ('~', '\u{303}'), ('=', '\u{304}'),
        ('.', '\u{307}'), ('u', '\u{306}'), ('v', '\u{30C}'), ('H', '\u{30B}'), ('c', '\u{327}'), ('k', '\u{328}'),
        ('r', '\u{30A}'), ('d', '\u{323}'), ('b', '\u{331}'),
    ];
    const LETTER: &[(&str, &str)] = &[
        ("ss", "ß"), ("ae", "æ"), ("AE", "Æ"), ("oe", "œ"), ("OE", "Œ"), ("aa", "å"), ("AA", "Å"),
        ("l", "ł"), ("L", "Ł"), ("o", "ø"), ("O", "Ø"), ("i", "ı"), ("j", "ȷ"),
    ];
    let c: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < c.len() {
        if c[i] == '\\' && i + 1 < c.len() {
            if let Some(&(_, mark)) = MARK.iter().find(|(a, _)| *a == c[i + 1]) {
                // \'a, \'{a}, \c{s}, \c s
                let mut j = i + 2;
                while j < c.len() && (c[j] == '{' || c[j] == ' ') {
                    j += 1;
                }
                if j < c.len() && c[j].is_alphabetic() && !(c[i + 1].is_alphabetic() && j == i + 2) {
                    out.push(c[j]);
                    out.push(mark);
                    i = j + 1;
                    while i < c.len() && c[i] == '}' {
                        i += 1;
                    }
                    continue;
                }
            }
            let rest: String = c[i + 1..].iter().take_while(|x| x.is_ascii_alphabetic()).collect();
            if let Some(&(_, u)) = LETTER.iter().find(|(k, _)| *k == rest) {
                out.push_str(u);
                i += 1 + rest.chars().count();
                continue;
            }
        }
        out.push(c[i]);
        i += 1;
    }
    out.nfc().collect()
}

/// Value without BibTeX markup: accents to Unicode, braces removed, whitespace collapsed,
/// simple escapes expanded.
pub fn clean(s: &str) -> String {
    let s = accents(s).replace(['{', '}'], "").replace("\\&", "&").replace("\\%", "%").replace("\\_", "_").replace("--", "–");
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latex_accents_become_unicode() {
        assert_eq!(clean("Eryi{\\u{g}}it, G{\\\"u}l{\\c{s}}en"), "Eryiğit, Gülşen");
        assert_eq!(clean("Samard{\\v{z}}i{\\'c} and Wr{\\'o}blewska and Ma{\\l}ecki"), "Samardžić and Wróblewska and Małecki");
        // negative control: plain text is unchanged
        assert_eq!(clean("Universal Dependencies v2"), "Universal Dependencies v2");
    }

    #[test]
    fn concatenated_month_keeps_following_fields() {
        let e = &parse("@inproceedings{k,\n    title = \"T\",\n    month = \"7-8 \" # jun,\n    year = \"2012\",\n    url = \"https://aclanthology.org/S12-1025/\",\n}\n")[0];
        assert_eq!(e.get("year"), "2012");
        assert_eq!(e.get("month"), "7-8 jun");
        assert_eq!(e.anthology_id(), "S12-1025");
        // negative control: an ordinary field without concatenation stays as is
        let e = &parse("@misc{k, month = jun, year = \"2012\"}")[0];
        assert_eq!((e.get("month"), e.get("year")), ("jun", "2012"));
    }

    #[test]
    fn malformed_snippets_do_not_panic() {
        // bibtex snippets from treebank READMEs can be truncated or broken — parsing does not fail
        assert_eq!(parse("@misc{k, title = {Unclosed Łódź")[0].get("title"), "Unclosed Łódź");
        assert_eq!(parse("@misc{k, title = \"No end quote")[0].get("title"), "No end quote");
        for bad in ["@misc{k, title = {x}}\n}", "@misc{k,", "@", "@misc{k, title = {x}, }", "@misc{k, title}", "@misc{k, a = \\"] {
            let _ = parse(bad);
        }
    }
}

struct Rule {
    topic: String,
    prefix: String,
    field: String,
    parts: Vec<String>,
}

fn rules(path: &Path) -> Result<Vec<Rule>> {
    let mut out = Vec::new();
    for (n, line) in fs::read_to_string(path)?.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() != 4 || !matches!(c[2], "title" | "any" | "booktitle") {
            bail!("{}:{}: expected \"topic \\t prefix \\t title|any|booktitle \\t pattern\"", path.display(), n + 1);
        }
        out.push(Rule {
            topic: c[0].into(),
            prefix: c[1].into(),
            field: c[2].into(),
            parts: c[3].split(" & ").map(|p| p.trim().to_lowercase()).collect(),
        });
    }
    Ok(out)
}

/// ASCII words for a slug: Latin letters and digits, everything else is a separator. An accented capital
/// is lowercased first (`Álvez` → `alvez`, not `lvez`).
fn ascii_words(s: &str) -> Vec<String> {
    s.chars()
        .flat_map(char::to_lowercase)
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' | 'ā' | 'ă' | 'ą' => 'a',
            'é' | 'è' | 'ê' | 'ë' | 'ě' | 'ē' | 'ę' => 'e',
            'í' | 'ì' | 'î' | 'ï' | 'ı' | 'ī' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ø' | 'ő' | 'ō' => 'o',
            'ú' | 'ù' | 'û' | 'ü' | 'ů' | 'ű' | 'ū' => 'u',
            'č' | 'ç' | 'ć' => 'c',
            'š' | 'ś' | 'ş' | 'ș' => 's',
            'ž' | 'ź' | 'ż' => 'z',
            'ř' => 'r',
            'ň' | 'ñ' | 'ń' => 'n',
            'ý' => 'y',
            'ł' => 'l',
            'ğ' => 'g',
            'ţ' | 'ț' => 't',
            'đ' | 'ď' => 'd',
            c => c.to_ascii_lowercase(),
        })
        .collect::<String>()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect()
}

const STOP: &[&str] = &["a", "an", "the", "of", "for", "and", "in", "on", "to", "with", "from", "by", "at", "towards", "toward", "using", "via"];

/// Paper slug: `<prefix>-<year>-<surname>-<up to 5 title words without function words>`; if it is already
/// in `used`, words of the Anthology id are appended.
pub fn slug_for(e: &Entry, prefix: &str, used: &mut HashSet<String>) -> String {
    let mut words: Vec<String> = ascii_words(e.get("title")).into_iter().filter(|w| !STOP.contains(&w.as_str())).take(5).collect();
    let last = ascii_words(e.first_surname()).last().cloned().unwrap_or_else(|| "anon".into());
    words.insert(0, last);
    let mut s = format!("{prefix}-{}-{}", e.get("year"), words.join("-"));
    if !used.insert(s.clone()) {
        s = format!("{s}-{}", ascii_words(e.anthology_id()).join("-"));
        used.insert(s.clone());
    }
    s
}

/// Manifest row for `sources plan` and `papers import`: PDF from aclanthology.org; license by year
/// (from 2016 CC BY 4.0, earlier CC BY-NC-SA 3.0).
pub fn row(e: &Entry, slug: &str, topic: &str, priority: u8) -> Value {
    let url = e.get("url").trim_end_matches('/');
    let y: u32 = e.get("year").parse().unwrap_or(0);
    let (status, license, license_url) = if y >= 2016 {
        ("open", "CC BY 4.0 (ACL Anthology)", "https://creativecommons.org/licenses/by/4.0/")
    } else {
        ("open-nc", "CC BY-NC-SA 3.0 (ACL Anthology, before 2016)", "https://creativecommons.org/licenses/by-nc-sa/3.0/")
    };
    json!({
        "slug": slug, "name": e.get("title"), "authors": e.get("author"), "langs": [], "kind": "paper",
        "year": y, "status": status, "license": license, "license_url": license_url,
        "source_page": format!("{url}/"), "format": "pdf",
        "files": [{"url": format!("{url}.pdf"), "size": null, "md5": null, "sha1": null}],
        "priority": priority, "notes": format!("{topic}; {}", e.get("booktitle")),
        "topics": [topic], "venue": e.get("booktitle"), "anthology_id": e.anthology_id(), "bibkey": e.key,
        "doi": e.get("doi"), "pages": e.get("pages"), "publisher": e.get("publisher"), "abstract": e.get("abstract"),
    })
}

/// Ids from a list (line: `id` or `id \t topic`; empty lines and `#…` are skipped) → manifest.
pub fn run_ids(bib: &Path, ids: &Path, out: &Path) -> Result<()> {
    let default_topic: String = out
        .file_stem()
        .map(|s| s.to_string_lossy().chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase())
        .unwrap_or_default();
    if default_topic.is_empty() {
        bail!("topic from file name {} is empty — name the file after the topic, e.g. treebanks.jsonl", out.display());
    }
    let wanted = read_ids(&fs::read_to_string(ids).with_context(|| format!("{}", ids.display()))?, &default_topic)?;
    let text = fs::read_to_string(bib).with_context(|| format!("{}", bib.display()))?;
    let entries = parse(&text);
    let (manifest, missing) = manifest_for_ids(&entries, &wanted);
    if !missing.is_empty() {
        for m in &missing {
            eprintln!("MISS {m} — not in the bibliography");
        }
        bail!("{} ids not found in {} — manifest not written", missing.len(), bib.display());
    }
    fs::write(out, &manifest)?;
    let nc = manifest.lines().filter(|l| l.contains("\"status\":\"open-nc\"")).count();
    println!("bib-ids: {} papers → {} (of them before 2016, NC: {nc}); bibliography entries {}", wanted.len(), out.display(), entries.len());
    Ok(())
}

/// Id list lines → (id, topic, priority) without duplicates, in file order.
fn read_ids(text: &str, default_topic: &str) -> Result<Vec<(String, String, u8)>> {
    let mut out: Vec<(String, String, u8)> = Vec::new();
    let mut seen = HashSet::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut cols = line.split('\t');
        let id = cols.next().unwrap_or("").trim().to_string();
        let topic = cols.next().map(str::trim).filter(|t| !t.is_empty()).unwrap_or(default_topic).to_string();
        let priority = match cols.next().map(str::trim).filter(|p| !p.is_empty()) {
            None => 1,
            Some(p) => match p.parse::<u8>() {
                Ok(v @ 1..=3) => v,
                _ => bail!("line {}: priority \"{p}\" — expected 1, 2 or 3", n + 1),
            },
        };
        if id.contains(char::is_whitespace) || topic.contains('-') || cols.next().is_some() {
            bail!("line {}: expected \"id\", \"id \\t topic\" or \"id \\t topic \\t priority\" (topic without a hyphen — it is also the slug prefix)", n + 1);
        }
        if seen.insert(id.clone()) {
            out.push((id, topic, priority));
        }
    }
    Ok(out)
}

/// Manifest (JSONL) for a list of (id, topic, priority), plus the ids missing from the bibliography.
pub fn manifest_for_ids(entries: &[Entry], wanted: &[(String, String, u8)]) -> (String, Vec<String>) {
    let by_id: HashMap<&str, &Entry> = entries.iter().filter(|e| !e.anthology_id().is_empty()).map(|e| (e.anthology_id(), e)).collect();
    let mut used = HashSet::new();
    let (mut manifest, mut missing) = (String::new(), Vec::new());
    for (id, topic, priority) in wanted {
        match by_id.get(id.as_str()) {
            Some(e) => {
                let slug = slug_for(e, topic, &mut used);
                let _ = writeln!(manifest, "{}", row(e, &slug, topic, *priority));
            }
            None => missing.push(id.clone()),
        }
    }
    (manifest, missing)
}

#[cfg(test)]
mod ids_tests {
    use super::*;

    const BIB: &str = r#"@inproceedings{nivre-etal-2016-universal,
    title = "{U}niversal {D}ependencies v1: A Multilingual Treebank Collection",
    author = "Nivre, Joakim  and
      de Marneffe, Marie-Catherine",
    year = "2016",
    url = "https://aclanthology.org/L16-1262/",
}
@proceedings{udw-2020-universal,
    title = "Proceedings of the Fourth Workshop on Universal Dependencies (UDW 2020)",
    editor = "de Marneffe, Marie-Catherine",
    year = "2020",
    url = "https://aclanthology.org/2020.udw-1/",
}
@inproceedings{zeman-etal-2018-conll,
    title = "{C}o{NLL} 2018 Shared Task: Multilingual Parsing from Raw Text to Universal Dependencies",
    author = "Zeman, Daniel",
    year = "2018",
    url = "https://aclanthology.org/K18-2001/",
}
"#;

    #[test]
    fn ids_to_manifest() {
        let entries = parse(BIB);
        assert_eq!(entries[1].kind, "proceedings");
        assert_eq!(entries[0].anthology_id(), "L16-1262");
        let wanted = read_ids("K18-2001\nL16-1262\tud\t2\n# comment\nK18-2001\n", "treebanks").unwrap();
        assert_eq!(wanted.len(), 2, "duplicate id is merged");
        let (m, missing) = manifest_for_ids(&entries, &wanted);
        assert!(missing.is_empty());
        let rows: Vec<Value> = m.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
        assert_eq!(rows[0]["slug"], "treebanks-2018-zeman-conll-2018-shared-task-multilingual");
        assert_eq!(rows[0]["status"], "open");
        assert_eq!(rows[0]["priority"], 1, "default priority");
        assert_eq!(rows[1]["slug"], "ud-2016-nivre-universal-dependencies-v1-multilingual-treebank");
        assert_eq!(rows[1]["status"], "open");
        assert_eq!(rows[1]["files"][0]["url"], "https://aclanthology.org/L16-1262.pdf");
        assert_eq!(rows[1]["topics"][0], "ud");
        assert_eq!(rows[1]["priority"], 2);
    }

    #[test]
    fn slug_words_keep_accented_capitals() {
        // an accented capital is not lost: previously "Álvez" gave "lvez", "Doğan" gave "do an"
        assert_eq!(ascii_words("Álvez"), vec!["alvez"]);
        assert_eq!(ascii_words("Doğan"), vec!["dogan"]);
        assert_eq!(ascii_words("Ó Séaghdha"), vec!["o", "seaghdha"]);
        // negative control: ASCII and already-lowercase letters stay as is
        assert_eq!(ascii_words("Zeman, Daniel"), vec!["zeman", "daniel"]);
        assert_eq!(ascii_words("Samardžić"), vec!["samardzic"]);
    }

    #[test]
    fn unknown_id_and_bad_topic_are_red() {
        // negative controls: a missing id is not silent; a topic with a hyphen would break the paper folder
        let entries = parse(BIB);
        let (_, missing) = manifest_for_ids(&entries, &[("2099.xx-1.1".into(), "treebanks".into(), 1)]);
        assert_eq!(missing, vec!["2099.xx-1.1".to_string()]);
        assert!(read_ids("L16-1262\tud-treebanks\n", "treebanks").is_err());
        assert!(read_ids("L16-1262 K18-2001\n", "treebanks").is_err());
        // priority outside 1…3, not a number, or an extra column — rejected, not a silent priority 1
        for bad in ["L16-1262\tud\t0\n", "L16-1262\tud\t4\n", "L16-1262\tud\tP1\n", "L16-1262\tud\t1\textra\n"] {
            assert!(read_ids(bad, "treebanks").is_err(), "{bad:?}");
        }
    }
}

pub fn run(bib: &Path, rules_path: &Path, out: &Path) -> Result<()> {
    let text = fs::read_to_string(bib).with_context(|| format!("{}", bib.display()))?;
    let entries = parse(&text);
    let rules = rules(rules_path)?;
    fs::create_dir_all(out)?;
    // topic → (papers: (priority, entry)); topic order as in the rules
    let mut topics: Vec<String> = Vec::new();
    for r in &rules {
        if !topics.contains(&r.topic) {
            topics.push(r.topic.clone());
        }
    }
    let mut found: BTreeMap<String, Vec<(u8, usize)>> = BTreeMap::new();
    let mut prefix_of: BTreeMap<usize, String> = BTreeMap::new();
    for (k, e) in entries.iter().enumerate() {
        let title = e.get("title").to_lowercase();
        let abs = e.get("abstract").to_lowercase();
        let book = e.get("booktitle").to_lowercase();
        let mut hit: BTreeMap<&str, u8> = BTreeMap::new();
        for r in &rules {
            let hay: [(&str, u8); 2] = match r.field.as_str() {
                "title" => [(&title, 1), ("", 0)],
                "booktitle" => [(&book, 1), ("", 0)],
                _ => [(&title, 1), (&abs, 2)],
            };
            for (h, p) in hay {
                if p > 0 && r.parts.iter().all(|x| h.contains(x.as_str())) {
                    let e = hit.entry(&r.topic).or_insert(p);
                    *e = (*e).min(p);
                    prefix_of.entry(k).or_insert_with(|| r.prefix.clone());
                }
            }
        }
        for (t, p) in hit {
            found.entry(t.to_string()).or_default().push((p, k));
        }
    }
    let mut used_slugs: HashSet<String> = HashSet::new();
    let mut slug_of: BTreeMap<usize, String> = BTreeMap::new();
    for topic in &topics {
        let Some(list) = found.get(topic) else {
            println!("{topic}: 0");
            continue;
        };
        let (mut manifest, mut md) = (String::new(), String::new());
        let mut by_year: BTreeMap<String, Vec<(u8, usize)>> = BTreeMap::new();
        let mut no_url = 0usize;
        for &(p, k) in list {
            by_year.entry(entries[k].get("year").to_string()).or_default().push((p, k));
        }
        let _ = writeln!(md, "# {topic}: ACL Anthology papers with abstracts ({} papers)\n", list.len());
        let _ = writeln!(md, "Priority: [1] — match in the paper or proceedings title, [2] — only in the abstract.\n");
        for (year, v) in by_year.iter().rev() {
            let _ = writeln!(md, "## {year}\n");
            for &(p, k) in v {
                let e = &entries[k];
                let url = e.get("url").trim_end_matches('/').to_string();
                let id = url.rsplit('/').next().unwrap_or("").to_string();
                let authors: Vec<&str> = e.get("author").split(" and ").collect();
                let first = authors.first().map_or("", |a| a.split(',').next().unwrap_or(a).trim());
                let who = if authors.len() > 2 { format!("{first} et al.") } else { e.get("author").replace(" and ", "; ") };
                let _ = writeln!(md, "- [{p}] **{}** — {who}. {} ({id})", e.get("title"), e.get("booktitle"));
                let abs = e.get("abstract");
                if !abs.is_empty() {
                    let _ = writeln!(md, "  {abs}");
                }
                if url.is_empty() || !url.contains("aclanthology.org") {
                    no_url += 1;
                    continue;
                }
                let slug = slug_of.entry(k).or_insert_with(|| slug_for(e, &prefix_of[&k], &mut used_slugs)).clone();
                let _ = writeln!(manifest, "{}", row(e, &slug, topic, p));
            }
            md.push('\n');
        }
        fs::write(out.join(format!("{topic}.jsonl")), &manifest)?;
        fs::write(out.join(format!("{topic}-abstracts.md")), &md)?;
        let p1 = list.iter().filter(|x| x.0 == 1).count();
        println!("{topic}: {} papers ({p1} with priority 1), without PDF on aclanthology {no_url}", list.len());
    }
    println!("bibliography entries: {}", entries.len());
    Ok(())
}
