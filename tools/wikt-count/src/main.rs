//! wikt-count — one-off streaming count over a Wiktionary XML dump (stdin).
//!
//!   lbzip2 -dc …/<db>/20260901/*-multistream*.xml*.bz2 | wikt-count <db> summary|en|uk <out-prefix>
//!
//! Always writes `<out-prefix>.summary.json`: per namespace — pages, redirects,
//! "with a [[link]]" (how MediaWiki counts articles, the `link` method), text bytes; for ns0 without
//! redirects — year of last edit, last editors (top 100 and how many distinct), anonymous ones.
//! Modes `en`/`uk` also parse the language sections of ns0 and write `.langs.tsv` (counters per language),
//! `.templates.tsv` (templates in the focus languages), and for `uk` also `.morph.tsv` and `.sumcite.tsv`.
//!
//! Dump format export-0.11: page tags one per line, text is escaped, so `</text>`
//! cannot occur inside the content — parsing is line-based, without an XML parser.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

use anyhow::{Context, Result, bail};
use memchr::memmem;
use serde_json::json;

#[derive(Default)]
struct NsStat {
    pages: u64,
    redirects: u64,
    linked: u64,
    bytes: u64,
}

#[derive(Default)]
struct Page {
    title: String,
    ns: i64,
    redirect: bool,
    timestamp: String,
    user: Option<String>,
    ip: bool,
    bytes: u64,
    text: Vec<u8>,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        bail!("usage: wikt-count <db> summary|en|uk <out-prefix>  < pages-articles.xml");
    }
    let (db, mode, out) = (args[0].as_str(), args[1].as_str(), args[2].as_str());
    if !["summary", "en", "uk"].contains(&mode) {
        bail!("mode: summary|en|uk");
    }

    let mut ns: BTreeMap<i64, NsStat> = BTreeMap::new();
    let mut years: BTreeMap<String, u64> = BTreeMap::new();
    let mut editors: HashMap<String, u64> = HashMap::new();
    let (mut anon, mut botlike, mut hidden, mut temp) = (0u64, 0u64, 0u64, 0u64);
    let mut en = EnAgg::default();
    let mut uk = UkAgg::default();

    let stdin = io::stdin();
    let mut rd = BufReader::with_capacity(1 << 24, stdin.lock());
    let mut line: Vec<u8> = Vec::with_capacity(1 << 16);
    let mut p = Page::default();
    let (mut in_text, mut in_page) = (false, false);
    let mut total = 0u64;
    loop {
        line.clear();
        if rd.read_until(b'\n', &mut line)? == 0 {
            break;
        }
        let mut l: &[u8] = &line;
        if l.last() == Some(&b'\n') {
            l = &l[..l.len() - 1];
        }
        if in_text {
            if let Some(body) = l.strip_suffix(b"</text>") {
                p.text.extend_from_slice(body);
                in_text = false;
            } else {
                p.text.extend_from_slice(l);
                p.text.push(b'\n');
            }
            continue;
        }
        let t = trim_start(l);
        if t.starts_with(b"<page>") {
            p = Page::default();
            in_page = true;
        } else if !in_page {
            continue;
        } else if let Some(v) = tag(t, b"<title>", b"</title>") {
            p.title = unescape(v);
        } else if let Some(v) = tag(t, b"<ns>", b"</ns>") {
            p.ns = std::str::from_utf8(v)?.trim().parse().context("ns")?;
        } else if t.starts_with(b"<redirect") {
            p.redirect = true;
        } else if let Some(v) = tag(t, b"<timestamp>", b"</timestamp>") {
            p.timestamp = String::from_utf8_lossy(v).into_owned();
        } else if let Some(v) = tag(t, b"<username>", b"</username>") {
            p.user = Some(unescape(v));
        } else if t.starts_with(b"<ip>") {
            p.ip = true;
        } else if t.starts_with(b"<text") {
            let gt = t.iter().position(|&c| c == b'>').context("<text> tag without '>'")?;
            p.bytes = attr_u64(&t[..gt], b"bytes=\"").unwrap_or(0);
            if !t[..=gt].ends_with(b"/>") {
                let rest = &t[gt + 1..];
                if let Some(body) = rest.strip_suffix(b"</text>") {
                    p.text.extend_from_slice(body);
                } else {
                    p.text.extend_from_slice(rest);
                    p.text.push(b'\n');
                    in_text = true;
                }
            }
        } else if t.starts_with(b"</page>") {
            in_page = false;
            total += 1;
            let s = ns.entry(p.ns).or_default();
            s.pages += 1;
            if p.redirect {
                s.redirects += 1;
            } else {
                s.bytes += p.bytes;
                if memmem::find(&p.text, b"[[").is_some() {
                    s.linked += 1;
                }
                if p.ns == 0 {
                    *years.entry(p.timestamp.get(..4).unwrap_or("????").to_string()).or_default() += 1;
                    match (&p.user, p.ip) {
                        (Some(u), _) => {
                            *editors.entry(u.clone()).or_default() += 1;
                            if is_botlike(u) {
                                botlike += 1;
                            }
                            // temporary accounts (instead of IPs, since 2025): "~2026-17637-29"
                            if u.starts_with('~') && u.chars().nth(1).is_some_and(|c| c.is_ascii_digit()) {
                                temp += 1;
                            }
                        }
                        (None, true) => anon += 1,
                        _ => hidden += 1,
                    }
                    match mode {
                        "en" => en.page(&p.title, &unescape(&p.text)),
                        "uk" => uk.page(&p.title, &unescape(&p.text)),
                        _ => {}
                    }
                }
            }
            if mode == "uk" && p.ns == 10 {
                if let Some((_, name)) = p.title.split_once(':') {
                    uk.tpl_pages.insert(name.to_string());
                }
            }
        }
    }
    if in_text || in_page {
        bail!("stream cut off at page \"{}\"", p.title);
    }
    if total == 0 {
        bail!("no pages in the stream");
    }

    let mut top: Vec<(&String, &u64)> = editors.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let top: Vec<_> = top.iter().take(100).map(|(u, n)| json!([u, n])).collect();
    let ns_json: serde_json::Map<String, serde_json::Value> = ns
        .iter()
        .map(|(k, v)| {
            (k.to_string(), json!({"pages": v.pages, "redirects": v.redirects, "linked": v.linked, "bytes": v.bytes}))
        })
        .collect();
    let mut extra = json!({});
    match mode {
        "en" => extra = en.write(out)?,
        "uk" => extra = uk.write(out)?,
        _ => {}
    }
    let summary = json!({
        "db": db, "mode": mode, "pages_total": total, "ns": ns_json,
        "ns0_last_year": years, "ns0_editors_distinct": editors.len(), "ns0_anon": anon,
        "ns0_hidden_user": hidden, "ns0_botlike": botlike, "ns0_temp_accounts": temp, "ns0_top_editors": top, "extra": extra,
    });
    let mut f = BufWriter::new(File::create(format!("{out}.summary.json"))?);
    serde_json::to_writer(&mut f, &summary)?;
    writeln!(f)?;
    Ok(())
}

// ---------- general helpers ----------

fn trim_start(l: &[u8]) -> &[u8] {
    let i = l.iter().position(|&c| c != b' ' && c != b'\t').unwrap_or(l.len());
    &l[i..]
}

fn tag<'a>(t: &'a [u8], open: &[u8], close: &[u8]) -> Option<&'a [u8]> {
    t.strip_prefix(open)?.strip_suffix(close)
}

fn attr_u64(t: &[u8], key: &[u8]) -> Option<u64> {
    let i = memmem::find(t, key)? + key.len();
    let j = t[i..].iter().position(|&c| c == b'"')? + i;
    std::str::from_utf8(&t[i..j]).ok()?.parse().ok()
}

fn unescape(v: &[u8]) -> String {
    if !v.contains(&b'&') {
        return String::from_utf8_lossy(v).into_owned();
    }
    let mut out: Vec<u8> = Vec::with_capacity(v.len());
    let mut i = 0;
    while i < v.len() {
        if v[i] == b'&' {
            if let Some(end) = v[i + 1..].iter().take(10).position(|&c| c == b';') {
                let ent = &v[i + 1..i + 1 + end];
                let rep: Option<char> = match ent {
                    b"lt" => Some('<'),
                    b"gt" => Some('>'),
                    b"amp" => Some('&'),
                    b"quot" => Some('"'),
                    b"apos" => Some('\''),
                    _ if ent.first() == Some(&b'#') => {
                        let num = std::str::from_utf8(&ent[1..]).ok();
                        let code = match num {
                            Some(n) if n.starts_with('x') || n.starts_with('X') => u32::from_str_radix(&n[1..], 16).ok(),
                            Some(n) => n.parse().ok(),
                            None => None,
                        };
                        code.and_then(char::from_u32)
                    }
                    _ => None,
                };
                if let Some(c) = rep {
                    let mut buf = [0u8; 4];
                    out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                    i += end + 2;
                    continue;
                }
            }
        }
        out.push(v[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn is_botlike(u: &str) -> bool {
    let l = u.to_lowercase();
    l.ends_with("bot")
        || l.starts_with("bot")
        // the last pattern is "bot" in Cyrillic
        || ["bot ", " bot", "-bot", "bot-", "_bot", "bot_", "\u{431}\u{43e}\u{442}"].iter().any(|p| l.contains(p))
}

/// All `{{name…` in a line: (name trimmed of surrounding spaces, rest of the line from the separator).
/// `{{{parameter}}}` is not a template, skipped.
fn tpl_iter(s: &str) -> Vec<(&str, &str)> {
    let b = s.as_bytes();
    let mut v = Vec::new();
    let mut i = 0;
    while let Some(off) = memmem::find(&b[i..], b"{{") {
        let start = i + off + 2;
        if b.get(start) == Some(&b'{') {
            i = start + 1;
            continue;
        }
        let mut j = start;
        while j < b.len() && !matches!(b[j], b'|' | b'}' | b'\n' | b'{') {
            j += 1;
        }
        v.push((s[start..j].trim(), &s[j..]));
        i = j.max(start);
    }
    v
}

/// First template parameter from the rest of the line (`|…` up to the next `|` or `}}`).
fn first_param(rest: &str) -> &str {
    let Some(r) = rest.strip_prefix('|') else { return "" };
    let end = r.find(['|', '}']).unwrap_or(r.len());
    &r[..end]
}

fn letters(s: &str) -> usize {
    s.chars().filter(|c| c.is_alphabetic()).count()
}

/// Text without `{{…}}` (nesting-aware) and without `<br>`.
fn strip_templates(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let b = s.as_bytes();
    let (mut depth, mut i, mut last) = (0usize, 0usize, 0usize);
    while i < b.len() {
        if b[i..].starts_with(b"{{") {
            if depth == 0 {
                out.push_str(&s[last..i]);
            }
            depth += 1;
            i += 2;
            continue;
        }
        if b[i..].starts_with(b"}}") && depth > 0 {
            depth -= 1;
            i += 2;
            if depth == 0 {
                last = i;
            }
            continue;
        }
        i += 1;
    }
    if depth == 0 {
        out.push_str(&s[last..]);
    }
    out.replace("<br>", " ").replace("<br />", " ").replace("<br/>", " ")
}

/// Heading `==…==`: (level, text).
fn heading(line: &str) -> Option<(usize, &str)> {
    let t = line.trim_end();
    let a = t.bytes().take_while(|&c| c == b'=').count();
    let z = t.bytes().rev().take_while(|&c| c == b'=').count();
    if a == 0 || z == 0 || a + z >= t.len() {
        return None;
    }
    Some((a.min(z), t[a..t.len() - z].trim()))
}

// ---------- en.wiktionary ----------

const EN_POS: &[&str] = &[
    "Noun", "Verb", "Adjective", "Adverb", "Pronoun", "Preposition", "Conjunction", "Interjection", "Numeral",
    "Particle", "Proper noun", "Phrase", "Proverb", "Prefix", "Suffix", "Infix", "Interfix", "Circumfix", "Affix",
    "Determiner", "Article", "Participle", "Contraction", "Symbol", "Letter", "Abbreviation", "Initialism",
    "Acronym", "Idiom", "Prepositional phrase", "Postposition", "Classifier", "Counter", "Root", "Combining form",
    "Diacritical mark", "Punctuation mark", "Syllable", "Ideophone", "Adnominal", "Ambiposition", "Circumposition",
    "Clitic", "Han character", "Romanization", "Number", "Converb", "Gerund", "Predicative", "Relative",
];

const EN_UX: &[&str] = &["ux", "uxi", "usex", "ja-usex", "zh-x", "ko-usex", "th-x", "km-x", "lo-x", "ja-x"];
const EN_T: &[&str] = &["t", "t+", "tt", "tt+", "t-check", "t+check"];

fn en_formof(name: &str) -> bool {
    matches!(name, "inflection of" | "infl of" | "plural of" | "verb form of" | "noun form of" | "adj form of" | "gerund of")
        || ["participle of", "tense of", "past of", "singular of", "plural of", "comparative of", "superlative of", "ing form of", "case form of"]
            .iter()
            .any(|s| name.ends_with(s))
}

#[derive(Default)]
struct EnSec {
    bytes: u64,
    pos: u64,
    defs: u64,
    defs_formof: u64,
    ex: u64,
    quotes: u64,
    pron: bool,
    audio: bool,
    etym: bool,
    rfe: bool,
    trans: u64,
    trans_uk: u64,
    syn: bool,
    rf: bool,
    infl: bool,
    refs: bool,
}

#[derive(Default)]
struct EnLang {
    sections: u64,
    bytes: u64,
    pos: u64,
    defs: u64,
    defs_formof: u64,
    sec_nodefs: u64,
    sec_formonly: u64,
    ex: u64,
    sec_ex: u64,
    quotes: u64,
    sec_quotes: u64,
    sec_pron: u64,
    sec_audio: u64,
    sec_etym: u64,
    sec_rfe: u64,
    trans: u64,
    trans_uk: u64,
    sec_trans: u64,
    sec_syn: u64,
    sec_rf: u64,
    sec_infl: u64,
    sec_refs: u64,
}

#[derive(Default)]
struct EnAgg {
    langs: HashMap<String, EnLang>,
    tpl: HashMap<(String, String), (u64, u64)>,
    pages: u64,
    pages_nolang: u64,
    trans_pages: u64,
    trans_items: u64,
    trans_items_uk: u64,
    nolang_sample: Vec<String>,
}

fn count_t(line: &str) -> (u64, u64) {
    let (mut a, mut b) = (0, 0);
    for (name, rest) in tpl_iter(line) {
        if EN_T.contains(&name) {
            a += 1;
            if rest.starts_with("|uk|") {
                b += 1;
            }
        }
    }
    (a, b)
}

impl EnAgg {
    fn page(&mut self, title: &str, text: &str) {
        self.pages += 1;
        if title.ends_with("/translations") {
            self.trans_pages += 1;
            for line in text.lines() {
                let (a, b) = count_t(line);
                self.trans_items += a;
                self.trans_items_uk += b;
            }
            return;
        }
        let mut cur: Option<(String, usize)> = None;
        let mut sec = EnSec::default();
        let mut any = false;
        let mut off = 0usize;
        for line in text.split('\n') {
            let start = off;
            off += line.len() + 1;
            if let Some((2, name)) = heading(line) {
                if let Some((lang, s0)) = cur.take() {
                    self.flush(lang, std::mem::take(&mut sec), &text[s0..start.min(text.len())]);
                }
                cur = Some((name.to_string(), start));
                any = true;
                continue;
            }
            if cur.is_some() {
                en_line(&mut sec, line);
            }
        }
        if let Some((lang, s0)) = cur.take() {
            self.flush(lang, sec, &text[s0.min(text.len())..]);
        }
        if !any {
            self.pages_nolang += 1;
            if self.nolang_sample.len() < 30 {
                self.nolang_sample.push(title.to_string());
            }
        }
    }

    fn flush(&mut self, lang: String, mut sec: EnSec, body: &str) {
        sec.bytes = body.len() as u64;
        if lang == "English" || lang == "Ukrainian" {
            let mut local: HashMap<&str, u64> = HashMap::new();
            for (name, _) in tpl_iter(body) {
                *local.entry(name).or_default() += 1;
            }
            for (name, n) in local {
                let e = self.tpl.entry((lang.clone(), name.to_string())).or_default();
                e.0 += n;
                e.1 += 1;
            }
        }
        let l = self.langs.entry(lang).or_default();
        l.sections += 1;
        l.bytes += sec.bytes;
        l.pos += sec.pos;
        l.defs += sec.defs;
        l.defs_formof += sec.defs_formof;
        if sec.defs == 0 {
            l.sec_nodefs += 1;
        } else if sec.defs_formof == sec.defs {
            l.sec_formonly += 1;
        }
        l.ex += sec.ex;
        l.sec_ex += (sec.ex > 0) as u64;
        l.quotes += sec.quotes;
        l.sec_quotes += (sec.quotes > 0) as u64;
        l.sec_pron += sec.pron as u64;
        l.sec_audio += sec.audio as u64;
        l.sec_etym += sec.etym as u64;
        l.sec_rfe += sec.rfe as u64;
        l.trans += sec.trans;
        l.trans_uk += sec.trans_uk;
        l.sec_trans += (sec.trans > 0) as u64;
        l.sec_syn += sec.syn as u64;
        l.sec_rf += sec.rf as u64;
        l.sec_infl += sec.infl as u64;
        l.sec_refs += sec.refs as u64;
    }

    fn write(&self, out: &str) -> Result<serde_json::Value> {
        let mut f = BufWriter::new(File::create(format!("{out}.langs.tsv"))?);
        writeln!(
            f,
            "lang\tsections\tbytes\tpos\tdefs\tdefs_formof\tsec_nodefs\tsec_formonly\tex\tsec_ex\tquotes\tsec_quotes\tsec_pron\tsec_audio\tsec_etym\tsec_rfe\ttrans\ttrans_uk\tsec_trans\tsec_syn\tsec_rf\tsec_infl\tsec_refs"
        )?;
        let mut langs: Vec<_> = self.langs.iter().collect();
        langs.sort_by(|a, b| b.1.sections.cmp(&a.1.sections).then(a.0.cmp(b.0)));
        for (k, l) in langs {
            writeln!(
                f,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                k.replace('\t', " "), l.sections, l.bytes, l.pos, l.defs, l.defs_formof, l.sec_nodefs, l.sec_formonly,
                l.ex, l.sec_ex, l.quotes, l.sec_quotes, l.sec_pron, l.sec_audio, l.sec_etym, l.sec_rfe, l.trans,
                l.trans_uk, l.sec_trans, l.sec_syn, l.sec_rf, l.sec_infl, l.sec_refs
            )?;
        }
        write_tpl(out, &self.tpl)?;
        Ok(json!({
            "ns0_pages_parsed": self.pages, "pages_without_L2": self.pages_nolang,
            "translation_subpages": self.trans_pages, "translation_subpage_items": self.trans_items,
            "translation_subpage_items_uk": self.trans_items_uk, "languages": self.langs.len(),
            "nolang_sample": self.nolang_sample,
        }))
    }
}

fn en_line(sec: &mut EnSec, line: &str) {
    if line.starts_with('=') {
        if let Some((lvl, name)) = heading(line) {
            let base = name.trim_end_matches(|c: char| c.is_ascii_digit() || c == ' ');
            if lvl >= 3 {
                if EN_POS.contains(&base) {
                    sec.pos += 1;
                }
                if base.starts_with("Etymology") {
                    sec.etym = true;
                }
                match base {
                    "Synonyms" => sec.syn = true,
                    "Declension" | "Conjugation" | "Inflection" => sec.infl = true,
                    "References" => sec.refs = true,
                    _ => {}
                }
            }
            return;
        }
    }
    if let Some(r) = line.strip_prefix('#') {
        let rest = r.trim_start_matches('#');
        match rest.chars().next() {
            Some(':') => {
                if rest[1..].trim_start().starts_with("''") {
                    sec.ex += 1;
                }
            }
            Some('*') => {
                if !rest[1..].starts_with(':') {
                    sec.quotes += 1;
                }
            }
            Some(_) if !rest.trim().is_empty() => {
                sec.defs += 1;
                if tpl_iter(rest).iter().any(|(n, _)| en_formof(n)) {
                    sec.defs_formof += 1;
                }
            }
            _ => {}
        }
    }
    if !line.contains("{{") {
        return;
    }
    for (name, rest) in tpl_iter(line) {
        if EN_UX.contains(&name) {
            sec.ex += 1;
        } else if EN_T.contains(&name) {
            sec.trans += 1;
            if rest.starts_with("|uk|") {
                sec.trans_uk += 1;
            }
        } else if name == "IPA" || name.ends_with("-IPA") || name == "en-pr" || name.ends_with("-pron") {
            sec.pron = true;
        } else if name == "audio" || name == "audios" {
            sec.audio = true;
        } else if name == "syn" || name == "synonyms" {
            sec.syn = true;
        }
        if name.starts_with("rf") {
            sec.rf = true;
            if name == "rfe" {
                sec.rfe = true;
            }
        }
    }
}

fn write_tpl(out: &str, tpl: &HashMap<(String, String), (u64, u64)>) -> Result<()> {
    let mut f = BufWriter::new(File::create(format!("{out}.templates.tsv"))?);
    writeln!(f, "lang\ttemplate\toccurrences\tsections")?;
    let mut v: Vec<_> = tpl.iter().collect();
    v.sort_by(|a, b| a.0.0.cmp(&b.0.0).then(b.1.1.cmp(&a.1.1)).then(a.0.1.cmp(&b.0.1)));
    for ((lang, name), (occ, secs)) in v {
        writeln!(f, "{}\t{}\t{}\t{}", lang, name.replace('\t', " "), occ, secs)?;
    }
    Ok(())
}

// ---------- uk.wiktionary ----------

#[derive(Default, Clone, Copy, PartialEq)]
enum Block {
    #[default]
    None,
    Morph,
    Pron,
    Meaning,
    Syn,
    Etym,
    Trans,
    Other,
}

#[derive(Default)]
struct UkSec {
    block: Block,
    defs: u64,
    defs_filled: u64,
    ex: u64,
    ex_filled: u64,
    sumcite: u64,
    pron: bool,
    audio: bool,
    etym_buf: String,
    etym_text: bool,
    etym_tpls: Vec<String>,
    trans_slots: u64,
    trans_filled: u64,
    syn: bool,
    morph_pos: Option<String>,
    morpho: bool,
    audio_files: Vec<String>,
    emptydef_tpls: Vec<String>,
    plain_defs: u64,
}

#[derive(Default)]
struct UkLang {
    sections: u64,
    bytes: u64,
    defs: u64,
    defs_filled: u64,
    sec_nodefs: u64,
    ex: u64,
    ex_filled: u64,
    sec_ex: u64,
    sumcite: u64,
    sec_sumcite: u64,
    sec_pron: u64,
    sec_audio: u64,
    sec_etym_text: u64,
    sec_etym_tpl: u64,
    sec_etym_tpl_ok: u64,
    sec_etym_any: u64,
    trans_slots: u64,
    trans_filled: u64,
    sec_trans: u64,
    sec_syn: u64,
    sec_morph: u64,
    sec_morpho: u64,
}

#[derive(Default)]
struct UkAgg {
    langs: HashMap<String, UkLang>,
    tpl: HashMap<(String, String), (u64, u64)>,
    morph: HashMap<(String, String), u64>,
    sumcite_years: BTreeMap<String, u64>,
    pending_etym: Vec<(String, bool, Vec<String>)>,
    tpl_pages: HashSet<String>,
    pages: u64,
    pages_nolang: u64,
    nolang_sample: Vec<String>,
    sections_per_page: BTreeMap<u64, u64>,
    audio: Vec<(String, String)>,
    emptydef: HashMap<(String, String), u64>,
}

/// Ukrainian Wiktionary templates that are themselves a definition ("same as…", word form, "relating to…",
/// cross-reference gloss). Names are genuine ukwiktionary template names, written as \u escapes.
const UK_DEF_TPL: &[&str] = &[
    "\u{442}\u{435} \u{441}\u{430}\u{43c}\u{435}", "\u{424}\u{43e}\u{440}\u{43c}\u{430}-\u{456}\u{43c}", "\u{424}\u{43e}\u{440}\u{43c}\u{430}-\u{434}\u{456}\u{454}\u{441}\u{43b}", "\u{424}\u{43e}\u{440}\u{43c}\u{430}-\u{43f}\u{440}\u{438}\u{43a}\u{43c}", "\u{424}\u{43e}\u{440}\u{43c}\u{430}-\u{437}\u{430}\u{439}\u{43c}", "\u{424}\u{43e}\u{440}\u{43c}\u{430}-\u{447}\u{438}\u{441}\u{43b}", "\u{441}\u{442}\u{43e}\u{441}.", "\u{434}\u{456}\u{44f}", "\u{437}\u{43c}\u{43f}", "\u{436}\u{456}\u{43d}. \u{456}\u{43c}.",
    "\u{430}\u{431}\u{441}\u{442}\u{440}. \u{456}\u{43c}.", "\u{432}\u{43b}\u{430}\u{441}\u{442}\u{438}\u{432}\u{456}\u{441}\u{442}\u{44c}", "\u{434}\u{456}\u{454}\u{43f}\u{440}\u{438}\u{43a}\u{43c}.", "\u{43f}\u{440}\u{438}\u{441}\u{43b}\u{456}\u{432}\u{43d}\u{438}\u{43a}", "\u{441}\u{43a}\u{43e}\u{440}.", "\u{434}\u{456}\u{44f}\u{441}\u{442}\u{430}\u{43d}", "as uk", "aslinks", "\u{441}\u{442}\u{430}\u{43d}", "\u{434}\u{43e}\u{43a}\u{43e}\u{43d}\u{430}\u{442}\u{438}",
    "\u{43c}\u{43e}\u{440}\u{444}\u{435}\u{43c}\u{430}", "\u{437}\u{431}", "\u{437}\u{431}\u{456}\u{440}\u{43d}.",
];

/// Ukrainian Wiktionary non-language level-1 headings: See also, Sources, Bibliography, Literature, Links, Notes.
const UK_NONLANG: &[&str] = &["\u{414}\u{438}\u{432}. \u{442}\u{430}\u{43a}\u{43e}\u{436}", "\u{414}\u{436}\u{435}\u{440}\u{435}\u{43b}\u{430}", "\u{411}\u{456}\u{431}\u{43b}\u{456}\u{43e}\u{433}\u{440}\u{430}\u{444}\u{456}\u{44f}", "\u{41b}\u{456}\u{442}\u{435}\u{440}\u{430}\u{442}\u{443}\u{440}\u{430}", "\u{41f}\u{43e}\u{441}\u{438}\u{43b}\u{430}\u{43d}\u{43d}\u{44f}", "\u{41f}\u{440}\u{438}\u{43c}\u{456}\u{442}\u{43a}\u{438}"];

/// Language heading of the Ukrainian edition: `{{=uk=}}`, `= {{-uk-}} =`, `={{-ru-|{{PAGENAME}}}}=`, `= {{=uk=}} =`.
fn uk_lang_header(line: &str) -> Option<String> {
    let t = line.trim();
    if let Some(rest) = t.strip_prefix("{{=") {
        let end = rest.find("=}}").or_else(|| rest.find("=|"))?;
        return Some(rest[..end].trim().to_string());
    }
    if t.starts_with('=') && !t.starts_with("==") {
        if let Some(p) = t.find("{{=") {
            let rest = &t[p + 3..];
            if let Some(end) = rest.find("=}}").or_else(|| rest.find("=|")) {
                return Some(rest[..end].trim().to_string());
            }
        }
        if let Some(p) = t.find("{{-") {
            let rest = &t[p + 3..];
            if let Some(end) = rest.find("-}}").or_else(|| rest.find("-|")) {
                return Some(rest[..end].trim().to_string());
            }
        }
        let inner = t.trim_matches('=').trim();
        if UK_NONLANG.contains(&inner) {
            return None;
        }
        return Some(format!("?{inner}"));
    }
    None
}

/// Citation in SUM-11 (Ukrainian academic dictionary) style: "…, [volume,] year, page" — `, 1952, 75` before `}}`/`|`/`.`/`)`/`;`/end.
fn sumcite(s: &str, years: &mut BTreeMap<String, u64>) -> u64 {
    let b = s.as_bytes();
    let (mut n, mut i) = (0, 0);
    while let Some(off) = memmem::find(&b[i..], b", ") {
        let y0 = i + off + 2;
        i = y0;
        let mut j = y0;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        if j - y0 != 4 || !(b[y0] == b'1' && (b[y0 + 1] == b'8' || b[y0 + 1] == b'9') || b[y0] == b'2' && b[y0 + 1] == b'0') {
            continue;
        }
        if !b[j..].starts_with(b", ") {
            continue;
        }
        let p0 = j + 2;
        let mut k = p0;
        while k < b.len() && b[k].is_ascii_digit() {
            k += 1;
        }
        if k == p0 {
            continue;
        }
        if k == b.len() || matches!(b[k], b'}' | b'|' | b'.' | b')' | b';' | b' ' | b'-' | b'<') {
            n += 1;
            *years.entry(s[y0..y0 + 4].to_string()).or_default() += 1;
        }
    }
    n
}

impl UkAgg {
    fn page(&mut self, title: &str, text: &str) {
        self.pages += 1;
        let mut cur: Option<(String, usize)> = None;
        let mut sec = UkSec::default();
        let mut nsec = 0u64;
        let mut off = 0usize;
        for line in text.split('\n') {
            let start = off;
            off += line.len() + 1;
            if let Some(code) = uk_lang_header(line) {
                if let Some((lang, s0)) = cur.take() {
                    self.flush(lang, std::mem::take(&mut sec), &text[s0..start.min(text.len())]);
                }
                cur = Some((code, start));
                nsec += 1;
                continue;
            }
            if cur.is_some() {
                self.line(&mut sec, line);
            }
        }
        if let Some((lang, s0)) = cur.take() {
            self.flush(lang, sec, &text[s0.min(text.len())..]);
        }
        *self.sections_per_page.entry(nsec).or_default() += 1;
        if nsec == 0 {
            self.pages_nolang += 1;
            if self.nolang_sample.len() < 40 {
                self.nolang_sample.push(title.to_string());
            }
        }
    }

    fn close_block(sec: &mut UkSec) {
        if sec.block == Block::Etym {
            let stripped = strip_templates(&sec.etym_buf);
            // drop the word "from" (Ukrainian, both cases) before counting etymology letters
            let stripped = stripped.replace("\u{412}\u{456}\u{434}", "").replace("\u{432}\u{456}\u{434}", "");
            if letters(&stripped) >= 6 {
                sec.etym_text = true;
            }
            sec.etym_buf.clear();
        }
    }

    fn line(&mut self, sec: &mut UkSec, line: &str) {
        if let Some((_, name)) = heading(line) {
            Self::close_block(sec);
            let n = name.to_lowercase();
            // ukwiktionary section headings: morphology, pronunciation, meaning, synonyms, etymology, translation
            sec.block = if n.contains("\u{43c}\u{43e}\u{440}\u{444}\u{43e}") {
                Block::Morph
            } else if n.contains("\u{432}\u{438}\u{43c}\u{43e}\u{432}\u{430}") {
                Block::Pron
            } else if n.contains("\u{437}\u{43d}\u{430}\u{447}\u{435}\u{43d}\u{43d}\u{44f}") {
                Block::Meaning
            } else if n.contains("\u{441}\u{438}\u{43d}\u{43e}\u{43d}\u{456}\u{43c}") {
                Block::Syn
            } else if n.contains("\u{435}\u{442}\u{438}\u{43c}\u{43e}\u{43b}\u{43e}\u{433}") {
                Block::Etym
            } else if n.contains("\u{43f}\u{435}\u{440}\u{435}\u{43a}\u{43b}\u{430}\u{434}") {
                Block::Trans
            } else {
                Block::Other
            };
            return;
        }
        match sec.block {
            Block::Meaning => {
                if line.chars().next().is_some_and(|c| c.is_alphabetic()) && letters(&strip_templates(line)) >= 1 {
                    sec.plain_defs += 1;
                }
                if let Some(r) = line.strip_prefix('#') {
                    let rest = r.trim_start_matches('#');
                    if !rest.starts_with(':') && !rest.starts_with('*') {
                        sec.defs += 1;
                        if letters(&strip_templates(rest)) >= 2
                            || tpl_iter(rest).iter().any(|(n, _)| UK_DEF_TPL.contains(n))
                        {
                            sec.defs_filled += 1;
                        } else {
                            let first = tpl_iter(rest)
                                .into_iter()
                                .map(|(n, _)| n)
                                .find(|n| !n.starts_with("\u{43f}\u{440}\u{438}\u{43a}\u{43b}\u{430}\u{434}")) // skip "example" templates
                                .unwrap_or("(no template)");
                            sec.emptydef_tpls.push(first.to_string());
                        }
                    }
                }
            }
            Block::Syn => {
                if let Some(r) = line.strip_prefix('#') {
                    if letters(&strip_templates(r)) >= 2 {
                        sec.syn = true;
                    }
                }
            }
            Block::Etym => {
                sec.etym_buf.push_str(line);
                sec.etym_buf.push('\n');
            }
            Block::Trans => {
                if let Some(r) = line.trim_start().strip_prefix('|') {
                    if let Some((code, val)) = r.split_once('=') {
                        let code = code.trim();
                        if !code.is_empty()
                            && code.len() <= 12
                            && code.chars().all(|c| c.is_ascii_lowercase() || c == '-' || c == '.' || c.is_ascii_digit())
                        {
                            sec.trans_slots += 1;
                            if letters(val) >= 1 {
                                sec.trans_filled += 1;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        let low = line.to_lowercase();
        if [".ogg", ".oga", ".wav", ".mp3", ".flac"].iter().any(|e| low.contains(e)) {
            sec.audio = true;
            for tok in line.split(['|', '[', ']', '{', '}', '=', '\n']) {
                let tk = tok.trim();
                let tl = tk.to_lowercase();
                if [".ogg", ".oga", ".wav", ".mp3", ".flac"].iter().any(|e| tl.ends_with(e)) {
                    // "File:" namespace prefix (local ukwiktionary spelling, with and without a space)
                    let tk = tk.trim_start_matches("\u{424}\u{430}\u{439}\u{43b}:").trim_start_matches("File:").trim_start_matches("\u{424}\u{430}\u{439}\u{43b} :");
                    sec.audio_files.push(tk.to_string());
                }
            }
        }
        sec.sumcite += sumcite(line, &mut self.sumcite_years);
        if !line.contains("{{") {
            return;
        }
        for (name, rest) in tpl_iter(line) {
            // ukwiktionary template names: example (uk/ru), transcription, IPA, morphology, semantics/synonyms=, etymology:
            if name.starts_with("\u{43f}\u{440}\u{438}\u{43a}\u{43b}\u{430}\u{434}") || name == "\u{43f}\u{440}\u{438}\u{43c}\u{435}\u{440}" {
                sec.ex += 1;
                if letters(first_param(rest)) >= 2 {
                    sec.ex_filled += 1;
                }
            } else if name.starts_with("transcription") || name.starts_with("\u{442}\u{440}\u{430}\u{43d}\u{441}\u{43a}\u{440}\u{438}\u{43f}\u{446}") || name == "uk-IPA" || name == "\u{41c}\u{424}\u{410}" {
                if letters(first_param(rest)) >= 1 {
                    sec.pron = true;
                }
            } else if name.starts_with("\u{43c}\u{43e}\u{440}\u{444}\u{43e}") {
                sec.morpho = true;
            } else if name == "\u{441}\u{435}\u{43c}\u{430}\u{43d}\u{442}\u{438}\u{43a}\u{430}" {
                if let Some(p) = rest.find("\u{441}\u{438}\u{43d}\u{43e}\u{43d}\u{456}\u{43c}\u{438}=") {
                    let v = &rest[p + "\u{441}\u{438}\u{43d}\u{43e}\u{43d}\u{456}\u{43c}\u{438}=".len()..];
                    let end = v.find(['|', '}']).unwrap_or(v.len());
                    if letters(&v[..end]) >= 2 {
                        sec.syn = true;
                    }
                }
            } else if let Some(stem) = name.strip_prefix("\u{435}\u{442}\u{438}\u{43c}\u{43e}\u{43b}\u{43e}\u{433}\u{456}\u{44f}:") {
                sec.etym_tpls.push(format!("\u{435}\u{442}\u{438}\u{43c}\u{43e}\u{43b}\u{43e}\u{433}\u{456}\u{44f}:{stem}"));
            }
            if sec.block == Block::Morph && sec.morph_pos.is_none() && !name.is_empty() && !name.starts_with("\u{43c}\u{43e}\u{440}\u{444}\u{43e}") {
                sec.morph_pos = Some(name.split_whitespace().collect::<Vec<_>>().join(" "));
            }
        }
    }

    fn flush(&mut self, lang: String, mut sec: UkSec, body: &str) {
        Self::close_block(&mut sec);
        if sec.defs == 0 && sec.plain_defs > 0 {
            sec.defs = sec.plain_defs;
            sec.defs_filled = sec.plain_defs;
        }
        if lang == "uk" {
            let mut local: HashMap<&str, u64> = HashMap::new();
            for (name, _) in tpl_iter(body) {
                *local.entry(name).or_default() += 1;
            }
            for (name, n) in local {
                let e = self.tpl.entry((lang.clone(), name.to_string())).or_default();
                e.0 += n;
                e.1 += 1;
            }
        }
        if let Some(pos) = &sec.morph_pos {
            *self.morph.entry((lang.clone(), pos.clone())).or_default() += 1;
        }
        for a in std::mem::take(&mut sec.audio_files) {
            self.audio.push((lang.clone(), a));
        }
        for t in std::mem::take(&mut sec.emptydef_tpls) {
            *self.emptydef.entry((lang.clone(), t)).or_default() += 1;
        }
        if !sec.etym_tpls.is_empty() {
            self.pending_etym.push((lang.clone(), sec.etym_text, std::mem::take(&mut sec.etym_tpls)));
        }
        let l = self.langs.entry(lang).or_default();
        l.sections += 1;
        l.bytes += body.len() as u64;
        l.defs += sec.defs;
        l.defs_filled += sec.defs_filled;
        l.sec_nodefs += (sec.defs_filled == 0) as u64;
        l.ex += sec.ex;
        l.ex_filled += sec.ex_filled;
        l.sec_ex += (sec.ex_filled > 0) as u64;
        l.sumcite += sec.sumcite;
        l.sec_sumcite += (sec.sumcite > 0) as u64;
        l.sec_pron += sec.pron as u64;
        l.sec_audio += sec.audio as u64;
        l.sec_etym_text += sec.etym_text as u64;
        l.trans_slots += sec.trans_slots;
        l.trans_filled += sec.trans_filled;
        l.sec_trans += (sec.trans_filled > 0) as u64;
        l.sec_syn += sec.syn as u64;
        l.sec_morph += sec.morph_pos.is_some() as u64;
        l.sec_morpho += sec.morpho as u64;
        // sec_etym_any without template ones is counted here; template ones are added in write()
        if sec.etym_text {
            l.sec_etym_any += 1;
        }
    }

    fn write(&mut self, out: &str) -> Result<serde_json::Value> {
        for (lang, text, names) in std::mem::take(&mut self.pending_etym) {
            let ok = names.iter().any(|n| self.tpl_pages.contains(n));
            let l = self.langs.entry(lang).or_default();
            l.sec_etym_tpl += 1;
            l.sec_etym_tpl_ok += ok as u64;
            if ok && !text {
                l.sec_etym_any += 1;
            }
        }
        let mut f = BufWriter::new(File::create(format!("{out}.langs.tsv"))?);
        writeln!(
            f,
            "lang\tsections\tbytes\tdefs\tdefs_filled\tsec_nodefs\tex\tex_filled\tsec_ex\tsumcite\tsec_sumcite\tsec_pron\tsec_audio\tsec_etym_text\tsec_etym_tpl\tsec_etym_tpl_ok\tsec_etym_any\ttrans_slots\ttrans_filled\tsec_trans\tsec_syn\tsec_morph\tsec_morpho"
        )?;
        let mut langs: Vec<_> = self.langs.iter().collect();
        langs.sort_by(|a, b| b.1.sections.cmp(&a.1.sections).then(a.0.cmp(b.0)));
        for (k, l) in langs {
            writeln!(
                f,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                k.replace('\t', " "), l.sections, l.bytes, l.defs, l.defs_filled, l.sec_nodefs, l.ex, l.ex_filled,
                l.sec_ex, l.sumcite, l.sec_sumcite, l.sec_pron, l.sec_audio, l.sec_etym_text, l.sec_etym_tpl,
                l.sec_etym_tpl_ok, l.sec_etym_any, l.trans_slots, l.trans_filled, l.sec_trans, l.sec_syn, l.sec_morph,
                l.sec_morpho
            )?;
        }
        write_tpl(out, &self.tpl)?;
        let mut m = BufWriter::new(File::create(format!("{out}.morph.tsv"))?);
        writeln!(m, "lang\tmorph_template\tsections")?;
        let mut v: Vec<_> = self.morph.iter().collect();
        v.sort_by(|a, b| a.0.0.cmp(&b.0.0).then(b.1.cmp(a.1)));
        for ((lang, w), n) in v {
            writeln!(m, "{lang}\t{w}\t{n}")?;
        }
        let mut a = BufWriter::new(File::create(format!("{out}.audio.tsv"))?);
        writeln!(a, "lang\tfile")?;
        for (lang, file) in &self.audio {
            writeln!(a, "{lang}\t{}", file.replace('\t', " "))?;
        }
        let mut e = BufWriter::new(File::create(format!("{out}.emptydef.tsv"))?);
        writeln!(e, "lang\tfirst_template\tdef_lines")?;
        let mut v: Vec<_> = self.emptydef.iter().collect();
        v.sort_by(|a, b| a.0.0.cmp(&b.0.0).then(b.1.cmp(a.1)));
        for ((lang, t), n) in v {
            writeln!(e, "{lang}\t{}\t{n}", t.replace('\t', " "))?;
        }
        let mut y = BufWriter::new(File::create(format!("{out}.sumcite.tsv"))?);
        writeln!(y, "year\tcitations")?;
        for (yr, n) in &self.sumcite_years {
            writeln!(y, "{yr}\t{n}")?;
        }
        Ok(json!({
            "ns0_pages_parsed": self.pages, "pages_without_lang_header": self.pages_nolang,
            "languages": self.langs.len(), "template_pages": self.tpl_pages.len(),
            "sections_per_page": self.sections_per_page, "nolang_sample": self.nolang_sample,
        }))
    }
}
