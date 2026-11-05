//! The `==English==` section of one English Wiktionary page → dictionary entries, one per part-of-speech section
//! (under each etymology): senses with their labels, examples and in-sense nyms, the head template's forms, the
//! section-level synonyms and antonyms, and the templates no handler knew (gaps). Then one TSV row per entry, in
//! the columns of Mova's dictionary layers (`world/src/dictstore.rs`) with extra columns after them.

use crate::wikitext::{self, Out, calls, nym_terms, render, term};
use std::collections::BTreeMap;

/// Part-of-speech headings of the English Wiktionary (its «POS headers»), as written.
pub const POS: &[&str] = &[
    "Noun", "Verb", "Adjective", "Adverb", "Pronoun", "Preposition", "Conjunction", "Interjection", "Proper noun", "Determiner", "Article", "Numeral",
    "Particle", "Prefix", "Suffix", "Infix", "Interfix", "Circumfix", "Affix", "Phrase", "Prepositional phrase", "Proverb", "Idiom", "Contraction", "Symbol",
    "Letter", "Number", "Punctuation mark", "Diacritical mark", "Clitic", "Abbreviation", "Initialism", "Acronym", "Participle", "Postposition",
];

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sense {
    /// 1 for `#`, 2 for `##` …
    pub depth: u8,
    pub text: String,
    pub labels: Vec<String>,
    pub examples: Vec<String>,
    /// (relation, term) from `#: {{syn|en|…}}` lines and inline nym templates.
    pub nyms: Vec<(String, String)>,
    pub form_of: Option<(String, String)>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Entry {
    pub title: String,
    pub pos: String,
    /// The etymology number (0 when the page has one etymology section without a number).
    pub etym: u32,
    /// The head template's name and its forms (`plural=cats`), or `head=…` when no forms are derivable.
    pub head: String,
    pub forms: Vec<(String, String)>,
    /// Labels of the head line (`{{tlb|en|UK}}`): they apply to every sense.
    pub head_labels: Vec<String>,
    pub senses: Vec<Sense>,
    /// Section-level synonyms/antonyms (`=====Synonyms=====`), (relation, term).
    pub nyms: Vec<(String, String)>,
    pub quotes: u32,
    pub gaps: BTreeMap<String, u32>,
    pub dropped: BTreeMap<String, u32>,
}

/// The heading of a line: (level, text) for `===Noun===`.
pub fn heading(line: &str) -> Option<(usize, &str)> {
    let l = line.trim_end();
    let lv = l.bytes().take_while(|&b| b == b'=').count();
    if lv < 2 || l.len() < 2 * lv + 1 || !l.ends_with(&"=".repeat(lv)) {
        return None;
    }
    let inner = l[lv..l.len() - lv].trim();
    (!inner.is_empty() && !inner.starts_with('=')).then_some((lv, inner))
}

/// The English section of a page, or None.
pub fn english(text: &str) -> Option<&str> {
    let mut start = None;
    let mut pos = 0;
    for line in text.split_inclusive('\n') {
        if let Some((2, h)) = heading(line) {
            if let Some(s) = start {
                return Some(&text[s..pos]);
            }
            if h == "English" {
                start = Some(pos + line.len());
            }
        } else if start.is_some() && line.trim_end() == "----" {
            return Some(&text[start.unwrap()..pos]);
        }
        pos += line.len();
    }
    start.map(|s| &text[s..])
}

/// Logical lines: a line whose templates are not closed is joined with the following ones (`{{col|en\n|a\n|b}}`).
pub fn logical_lines(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth: i64 = 0;
    for line in s.lines() {
        if depth > 0 {
            cur.push(' ');
        }
        cur.push_str(line);
        depth += line.matches("{{").count() as i64 - line.matches("}}").count() as i64;
        if depth <= 0 || cur.len() > 200_000 {
            out.push(std::mem::take(&mut cur));
            depth = 0;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn merge(into: &mut BTreeMap<String, u32>, from: &BTreeMap<String, u32>) {
    for (k, v) in from {
        *into.entry(k.clone()).or_default() += v;
    }
}

/// The entries of a page's English section.
pub fn entries(title: &str, section: &str) -> Vec<Entry> {
    let mut out: Vec<Entry> = Vec::new();
    let mut etym = 0u32;
    // the heading context: the current POS entry's level, and the kind of the current non-POS subsection
    let mut pos_level = 0usize;
    let mut sub: Option<String> = None;
    let mut in_pos = false;
    let mut seen_head = false;
    for line in logical_lines(section) {
        if let Some((lv, h)) = heading(&line) {
            if let Some(n) = h.strip_prefix("Etymology") {
                etym = n.trim().parse().unwrap_or(0);
                in_pos = false;
                sub = None;
                continue;
            }
            if POS.contains(&h) {
                out.push(Entry { title: title.to_string(), pos: h.to_lowercase(), etym, ..Entry::default() });
                pos_level = lv;
                in_pos = true;
                seen_head = false;
                sub = None;
                continue;
            }
            // a subsection under the POS (deeper), or at its level right after it (synonyms of the whole entry)
            if in_pos && lv > pos_level || !out.is_empty() && lv >= pos_level && matches!(h, "Synonyms" | "Antonyms") {
                sub = Some(h.to_string());
            } else {
                in_pos = false;
                sub = None;
            }
            continue;
        }
        let Some(e) = out.last_mut() else { continue };
        if let Some(s) = &sub {
            let rel = match s.as_str() {
                "Synonyms" => "syn",
                "Antonyms" => "ant",
                _ => continue,
            };
            if !line.starts_with('*') && !line.starts_with("{{col") && !line.starts_with("{{der") && !line.starts_with("{{rel") {
                continue;
            }
            for c in calls(&line) {
                let n = c.name.as_str();
                if matches!(n, "l" | "l-self" | "ll" | "link") {
                    let t = wikitext::plain(&term(c.p(1)));
                    if !t.is_empty() {
                        e.nyms.push((rel.into(), t));
                    }
                } else if n.starts_with("col") || n.starts_with("der") || n.starts_with("rel") || matches!(n, "syn" | "ant") {
                    e.nyms.extend(nym_terms(&c).into_iter().map(|t| (rel.to_string(), t)));
                }
            }
            // plain links on a list line: * [[word]], [[other]]
            if line.starts_with('*') && !line.contains("{{") {
                let mut o = Out::default();
                for part in render(line.trim_start_matches('*'), &mut o).split([',', ';']) {
                    let t = part.trim();
                    if !t.is_empty() && !t.contains(':') && t.split_whitespace().count() <= 4 {
                        e.nyms.push((rel.into(), t.to_string()));
                    }
                }
            }
            continue;
        }
        if !in_pos {
            continue;
        }
        let t = line.trim_start();
        if !seen_head && t.starts_with("{{") && !t.starts_with('#') {
            seen_head = true;
            head(e, t);
            continue;
        }
        if !t.starts_with('#') {
            continue;
        }
        let marks: String = t.chars().take_while(|c| matches!(c, '#' | '*' | ':')).collect();
        let body = &t[marks.len()..];
        if marks.contains('*') {
            // a quotation (#*, #*:): counted, its text not taken (quoted works keep their own provenance)
            if marks.ends_with('*') {
                e.quotes += 1;
            }
            continue;
        }
        if marks.ends_with(':') {
            // an example or an in-sense nym line under the last sense
            let mut o = Out::default();
            let text = render(body, &mut o);
            merge(&mut e.gaps, &o.gaps);
            merge(&mut e.dropped, &o.dropped);
            if let Some(s) = e.senses.last_mut() {
                let had = !o.examples.is_empty();
                s.nyms.extend(o.nyms);
                s.examples.extend(o.examples);
                // a bare example line: #: ''An example.''
                if !had &&!text.is_empty() && calls(body).iter().all(|c| !matches!(c.name.as_str(), "syn" | "ant" | "hyper" | "hypo" | "coord" | "synonyms" | "antonyms")) {
                    s.examples.push(text);
                }
            }
            continue;
        }
        let depth = marks.len() as u8;
        let mut o = Out::default();
        let text = render(body, &mut o);
        merge(&mut e.gaps, &o.gaps);
        merge(&mut e.dropped, &o.dropped);
        if text.is_empty() && o.labels.is_empty() && o.form_of.is_none() {
            continue;
        }
        e.senses.push(Sense { depth, text, labels: o.labels, examples: o.examples, nyms: o.nyms, form_of: o.form_of });
    }
    out.retain(|e| !e.senses.is_empty());
    out
}

/// The head line (`{{en-noun|es}} {{tlb|en|UK}}`): the head template's forms and the head labels.
fn head(e: &mut Entry, line: &str) {
    let mut o = Out::default();
    for c in calls(line) {
        match c.name.as_str() {
            "tlb" | "lb" => e.head_labels.extend(wikitext::label_values(&c)),
            "en-noun" | "en-proper noun" | "en-prop" | "en-verb" | "en-adj" | "en-adv" | "head" => {
                e.head = c.name.clone();
                e.forms = forms(&e.title, &c);
            }
            n if n.starts_with("en-") => e.head = n.to_string(),
            _ => {
                wikitext::template(&c, &mut o);
            }
        }
    }
    merge(&mut e.gaps, &o.gaps);
    merge(&mut e.dropped, &o.dropped);
}

fn ends_sibilant(w: &str) -> bool {
    w.ends_with('s') || w.ends_with('x') || w.ends_with('z') || w.ends_with("ch") || w.ends_with("sh")
}

/// A one-syllable word ending in one vowel and one consonant (cat, stop, big): the final consonant doubles before
/// -ing, -ed, -er, -est (an approximation of `Module:en-headword`; w, x, y never double).
pub fn cvc(w: &str) -> bool {
    let b = w.as_bytes();
    let vowel = |c: u8| b"aeiou".contains(&c);
    let groups = b.iter().enumerate().filter(|&(i, &c)| vowel(c) && (i == 0 || !vowel(b[i - 1]))).count();
    b.len() >= 3 && b.iter().all(|c| c.is_ascii_lowercase()) && groups == 1 && !vowel(b[b.len() - 1]) && !b"wxy".contains(&b[b.len() - 1]) && vowel(b[b.len() - 2]) && !vowel(b[b.len() - 3])
}

fn cons_y(w: &str) -> bool {
    let b = w.as_bytes();
    b.len() >= 2 && b[b.len() - 1] == b'y' && !b"aeiou".contains(&b[b.len() - 2])
}

/// The default plural of English nouns as `Module:en-headword` forms it: -es after a sibilant, consonant+y → -ies,
/// else -s.
pub fn plural(w: &str) -> String {
    if ends_sibilant(w) {
        format!("{w}es")
    } else if cons_y(w) {
        format!("{}ies", &w[..w.len() - 1])
    } else {
        format!("{w}s")
    }
}

/// The default verb forms (third person singular, present participle, simple past) as `Module:en-headword` forms
/// them; `double` doubles the final consonant (the `++` spec).
pub fn verb_default(w: &str, double: bool) -> [String; 3] {
    let s3 = if ends_sibilant(w) || w.ends_with('o') && !w.ends_with("oo") { format!("{w}es") } else if cons_y(w) { format!("{}ies", &w[..w.len() - 1]) } else { format!("{w}s") };
    let stem = if double { format!("{w}{}", w.chars().last().unwrap_or_default()) } else { w.to_string() };
    let ing = if w.ends_with("ie") { format!("{}ying", &w[..w.len() - 2]) } else if !double && w.ends_with('e') && !w.ends_with("ee") && !w.ends_with("ye") && !w.ends_with("oe") && w.len() > 2 { format!("{}ing", &w[..w.len() - 1]) } else if w.ends_with("ie") { format!("{}ying", &w[..w.len() - 2]) } else { format!("{stem}ing") };
    let past = if w.ends_with('e') { format!("{w}d") } else if cons_y(w) { format!("{}ied", &w[..w.len() - 1]) } else { format!("{stem}ed") };
    [s3, ing, past]
}

/// The forms a head template's parameters give, without the Lua module: `en-noun` plurals (`+`, `s`, `es`, `~`, `-`,
/// full forms), `en-verb` (`+`, `++`, full forms; positions 1–4), `en-adj`/`en-adv` comparatives (`er`, `more`,
/// `-`, full forms). A parameter with a spec the rules do not cover (`<…>`, `[[…]]`) is kept raw as `raw=…`.
pub fn forms(w: &str, c: &wikitext::Call) -> Vec<(String, String)> {
    let mut f: Vec<(String, String)> = Vec::new();
    let raw = |p: &str| p.contains('<') || p.contains("[[") || p.contains("{{");
    let base = if c.n("head").is_empty() { w.to_string() } else { wikitext::plain(c.n("head")) };
    match c.name.as_str() {
        "en-noun" => {
            let ps: Vec<&str> = (0..c.pos.len()).map(|i| c.p(i)).filter(|p| !p.is_empty()).collect();
            if ps.is_empty() {
                f.push(("plural".into(), plural(&base)));
            }
            for p in ps {
                match p {
                    "~" => f.push(("countable".into(), "both".into())),
                    "-" => f.push(("countable".into(), "no".into())),
                    "!" | "?" => f.push(("plural".into(), p.into())),
                    "+" => f.push(("plural".into(), plural(&base))),
                    "s" | "es" => f.push(("plural".into(), format!("{base}{p}"))),
                    p if raw(p) => f.push(("raw".into(), p.to_string())),
                    p => f.push(("plural".into(), p.to_string())),
                }
            }
            if f.iter().all(|x| x.0 == "countable") && f.iter().any(|x| x.1 == "both") {
                f.push(("plural".into(), plural(&base)));
            }
        }
        "en-proper noun" | "en-prop" => {
            for i in 0..c.pos.len() {
                match c.p(i) {
                    "" => {}
                    "s" | "es" => f.push(("plural".into(), format!("{base}{}", c.p(i)))),
                    "+" => f.push(("plural".into(), plural(&base))),
                    p if raw(p) => f.push(("raw".into(), p.to_string())),
                    p => f.push(("plural".into(), p.to_string())),
                }
            }
        }
        "en-verb" => {
            let p: Vec<&str> = (0..4).map(|i| c.p(i)).collect();
            if p.iter().any(|x| raw(x)) {
                f.push(("raw".into(), c.pos.join("|")));
            } else {
                let double = p[0] == "++" || p.iter().all(|x| x.is_empty()) && cvc(&base);
                let d = verb_default(&base, double);
                let pick = |x: &str, i: usize| if x.is_empty() || x == "+" || x == "++" { d[i].clone() } else { x.to_string() };
                let s3 = if double { d[0].clone() } else { pick(p[0], 0) };
                let ing = pick(p[1], 1);
                let past = if double && p[2].is_empty() { d[2].clone() } else { pick(p[2], 2) };
                let pp = if p[3].is_empty() { past.clone() } else { p[3].to_string() };
                f.extend([("3sg".into(), s3), ("ing".into(), ing), ("past".into(), past), ("pp".into(), pp)]);
            }
        }
        "en-adj" | "en-adv" => {
            let ps: Vec<&str> = (0..c.pos.len()).map(|i| c.p(i)).filter(|p| !p.is_empty()).collect();
            if ps.is_empty() && c.name == "en-adj" {
                f.push(("comparative".into(), format!("more {base}")));
            }
            for p in ps {
                match p {
                    "-" => f.push(("comparable".into(), "no".into())),
                    "more" => f.push(("comparative".into(), format!("more {base}"))),
                    "further" => f.push(("comparative".into(), format!("further {base}"))),
                    "er" => {
                        let st = if cvc(&base) { format!("{base}{}", base.chars().last().unwrap_or_default()) } else if base.ends_with('e') { base[..base.len() - 1].to_string() } else if cons_y(&base) { format!("{}i", &base[..base.len() - 1]) } else { base.clone() };
                        f.push(("comparative".into(), format!("{st}er")));
                        f.push(("superlative".into(), format!("{st}est")));
                    }
                    p if raw(p) => f.push(("raw".into(), p.to_string())),
                    p => f.push(("comparative".into(), p.to_string())),
                }
            }
            for k in ["sup", "sup2"] {
                if !c.n(k).is_empty() {
                    f.push(("superlative".into(), c.n(k).to_string()));
                }
            }
        }
        _ => {
            // head|en|noun|plural|cats|…: label/form pairs after the POS
            let mut i = 2;
            while i + 1 < c.pos.len() {
                let (k, v) = (wikitext::plain(c.p(i)), wikitext::plain(c.p(i + 1)));
                if !k.is_empty() && !v.is_empty() {
                    f.push((k, v));
                }
                i += 2;
            }
        }
    }
    f
}

/// One cell: tabs and newlines become spaces.
fn cell(s: &str) -> String {
    s.chars().map(|c| if matches!(c, '\t' | '\n' | '\r') { ' ' } else { c }).collect()
}

/// The sense numbers: 1), 2) for top-level senses, 1.1) for their subsenses.
fn numbers(senses: &[Sense]) -> Vec<String> {
    let mut out = Vec::new();
    let (mut top, mut subn) = (0u32, 0u32);
    for s in senses {
        if s.depth <= 1 || top == 0 {
            top += 1;
            subn = 0;
            out.push(top.to_string());
        } else {
            subn += 1;
            out.push(format!("{top}.{subn}"));
        }
    }
    out
}

/// Provenance of a row: page id, revision id, dump file and the byte offset of its bz2 stream.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Prov {
    pub page_id: u64,
    pub rev: u64,
    pub file: String,
    pub offset: u64,
}

/// The column names of a row, as the layer's header writes them (the first six are dictstore's).
pub const COLUMNS: [&str; 13] = [
    "word", "part of speech", "meaning", "sense in the lessons", "examples", "note", "labels", "synonyms", "antonyms", "forms", "form of", "gaps",
    "provenance",
];

/// A layer row of an entry (see `COLUMNS`). Meaning: «1) (labels) text. 2) …»; examples, labels and nyms prefixed
/// by their sense number (`*` for the whole entry).
pub fn row(e: &Entry, p: &Prov) -> String {
    let nums = numbers(&e.senses);
    let mut meaning = Vec::new();
    let mut examples = Vec::new();
    let mut labels = Vec::new();
    let mut syn: Vec<String> = Vec::new();
    let mut ant: Vec<String> = Vec::new();
    let mut form_of = Vec::new();
    if !e.head_labels.is_empty() {
        labels.push(format!("*:{}", e.head_labels.join(",")));
    }
    let add_nyms = |n: &str, nyms: &[(String, String)], syn: &mut Vec<String>, ant: &mut Vec<String>| {
        let s: Vec<&str> = nyms.iter().filter(|x| x.0 == "syn").map(|x| x.1.as_str()).collect();
        let a: Vec<&str> = nyms.iter().filter(|x| x.0 == "ant").map(|x| x.1.as_str()).collect();
        if !s.is_empty() {
            syn.push(format!("{n}:{}", s.join(",")));
        }
        if !a.is_empty() {
            ant.push(format!("{n}:{}", a.join(",")));
        }
    };
    for (s, n) in e.senses.iter().zip(&nums) {
        let lab = if s.labels.is_empty() { String::new() } else { format!("({}) ", s.labels.join(", ")) };
        let mut t = s.text.trim_end().to_string();
        if !t.is_empty() && !t.ends_with(['.', '!', '?', '…', ')', ':']) {
            t.push('.');
        }
        meaning.push(format!("{n}) {lab}{t}"));
        for x in &s.examples {
            examples.push(format!("{n}: {x}"));
        }
        if !s.labels.is_empty() {
            labels.push(format!("{n}:{}", s.labels.join(",")));
        }
        add_nyms(n, &s.nyms, &mut syn, &mut ant);
        if let Some((k, l)) = &s.form_of {
            form_of.push(format!("{n}:{k}:{l}"));
        }
    }
    add_nyms("*", &e.nyms, &mut syn, &mut ant);
    let forms: Vec<String> = e.forms.iter().map(|(k, v)| format!("{k}={v}")).collect();
    let gaps: Vec<String> = e.gaps.iter().map(|(k, v)| format!("{k}×{v}")).collect();
    let note = if e.etym > 0 { format!("etymology {}", e.etym) } else { String::new() };
    let prov = format!("{} rev {} {}@{}", p.page_id, p.rev, p.file, p.offset);
    [
        e.title.clone(), e.pos.clone(), meaning.join(" "), String::new(), examples.join(" | "), note, labels.join("; "), syn.join("; "), ant.join("; "),
        forms.join("; "), form_of.join("; "), gaps.join(","), prov,
    ]
    .iter()
    .map(|c| cell(c))
    .collect::<Vec<_>>()
    .join("\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TELLY: &str = "==English==\n\n===Etymology 1===\nShortened.\n\n====Noun====\n{{en-noun|~|+|s<l:nonstandard>}} {{tlb|en|UK|colloquial}}\n\n# [[television|Television]].\n#: {{ux|en|Not much on '''telly''' tonight.}}\n#* '''2007''', ''Show'':\n#*: You're from '''telly'''!\n# A [[television set]].\n## {{lb|en|rare}} A small one.\n# {{zz-unknown|x}} Telegraph.\n\n=====Synonyms=====\n* {{l|en|tube}} (US), {{l|en|TV}}\n\n=====Translations=====\n* French: {{t+|fr|télé|f}}\n\n===Etymology 2===\n\n====Verb====\n{{en-verb}}\n\n# To [[watch]] television.\n#: {{syn|en|view}}\n\n==French==\n\n===Noun===\n# Not English.\n";

    #[test]
    fn telly_entries() {
        let sec = english(TELLY).unwrap();
        assert!(!sec.contains("Not English"));
        let es = entries("telly", sec);
        assert_eq!(es.len(), 2);
        let n = &es[0];
        assert_eq!((n.pos.as_str(), n.etym, n.head_labels.clone()), ("noun", 1, vec!["UK".to_string(), "colloquial".to_string()]));
        assert_eq!(n.senses.len(), 4);
        assert_eq!(n.senses[0].examples, vec!["Not much on telly tonight.".to_string()]);
        assert_eq!(n.quotes, 1);
        assert_eq!(n.nyms, vec![("syn".into(), "tube".into()), ("syn".into(), "TV".into())]);
        assert_eq!(n.gaps.get("zz-unknown"), Some(&1));
        assert_eq!(n.forms, vec![("countable".into(), "both".into()), ("plural".into(), "tellies".into()), ("raw".into(), "s<l:nonstandard>".into())]);
        let r = row(n, &Prov { page_id: 74428, rev: 91706010, file: "f.bz2".into(), offset: 42 });
        let c: Vec<&str> = r.split('\t').collect();
        assert_eq!(c.len(), COLUMNS.len());
        assert_eq!(c[2], "1) Television. 2) A television set. 2.1) (rare) A small one. 3) Telegraph.");
        assert_eq!(c[4], "1: Not much on telly tonight.");
        assert_eq!(c[6], "*:UK,colloquial; 2.1:rare");
        assert_eq!(c[7], "*:tube,TV");
        assert_eq!(c[11], "zz-unknown×1");
        assert_eq!(c[12], "74428 rev 91706010 f.bz2@42");
        let v = &es[1];
        assert_eq!((v.pos.as_str(), v.etym), ("verb", 2));
        assert_eq!(v.forms.iter().map(|x| x.1.as_str()).collect::<Vec<_>>(), vec!["tellies", "tellying", "tellied", "tellied"]);
        assert_eq!(v.senses[0].nyms, vec![("syn".into(), "view".into())]);
    }

    #[test]
    fn default_forms() {
        assert_eq!(plural("box"), "boxes");
        assert_eq!(plural("city"), "cities");
        assert_eq!(plural("day"), "days");
        assert_eq!(verb_default("make", false), ["makes", "making", "maked"].map(String::from));
        assert_eq!(verb_default("stop", true), ["stops", "stopping", "stopped"].map(String::from));
        assert_eq!(verb_default("lie", false)[1], "lying");
        let c = wikitext::parse_call("en-adj|er");
        assert_eq!(forms("big", &c)[0].1, "bigger");
        assert_eq!(forms("nice", &wikitext::parse_call("en-adj|er")), vec![("comparative".into(), "nicer".into()), ("superlative".into(), "nicest".into())]);
    }

    #[test]
    fn no_english_section() {
        assert!(english("==French==\n# chat\n").is_none());
        assert_eq!(heading("====Noun===="), Some((4, "Noun")));
        assert_eq!(heading("=bad"), None);
    }
}
