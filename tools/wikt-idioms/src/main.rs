//! wikt-idioms — multiword English expressions from an English Wiktionary XML dump (stdin), line-based like
//! `wikt-count` (export-0.11: one tag per line, escaped text).
//!
//!   lbzip2 -dc …/enwiktionary/20260901/*-multistream*.xml*.bz2 | wikt-idioms > idioms.tsv
//!
//! A page counts if its title has a space (or a hyphen between letters) and its `==English==` section has a
//! multiword part of speech — Phrase, Proverb, Idiom, Verb (phrasal and multiword verbs), Noun, Adjective,
//! Adverb, Interjection, Prepositional phrase — and either an idiomatic label (`{{lb|en|…idiomatic…}}`), an
//! idiom/phrasal-verb category, or the POS Phrase/Proverb/Idiom, or a Verb title (verb + particle/object).
//! Single words count only with a register label (slang, informal, formal, humorous, euphemistic, archaic …).
//! Output TSV: phrase, kind (idiom | phrasal-verb | verb-phrase | phrase | proverb | multiword | word), POS, first
//! gloss, register labels with the gloss of the labelled sense ("slang: …; humorous: …"), regional varieties with the
//! gloss of the labelled sense ("UK: …; Australia: …"; single words also count with a variety label).
//! License of the output: CC BY-SA (Wiktionary) — attribution and share-alike for the derived list.

use std::io::{BufRead, Write};

fn unescape(s: &str) -> String {
    s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&#039;", "'").replace("&amp;", "&")
}

/// Crude wikitext → plain text for a gloss: [[a|b]] → b, [[a]] → a, templates dropped except {{w|x}}/{{l|en|x}}.
fn plain(s: &str) -> String {
    let mut out = String::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '{' && cs.get(i + 1) == Some(&'{') {
            let mut d = 0;
            let mut j = i;
            while j < cs.len() {
                if cs[j] == '{' && cs.get(j + 1) == Some(&'{') {
                    d += 1;
                    j += 2;
                    continue;
                }
                if cs[j] == '}' && cs.get(j + 1) == Some(&'}') {
                    d -= 1;
                    j += 2;
                    if d == 0 {
                        break;
                    }
                    continue;
                }
                j += 1;
            }
            let inner: String = cs[i + 2..j.saturating_sub(2).max(i + 2)].iter().collect();
            let parts: Vec<&str> = inner.split('|').collect();
            match parts.first().map(|x| x.trim()) {
                Some("l") | Some("m") | Some("link") if parts.len() >= 3 => out.push_str(parts[2]),
                Some("w") if parts.len() >= 2 => out.push_str(parts.last().unwrap()),
                Some("gloss") | Some("gl") if parts.len() >= 2 => out.push_str(&format!("({})", parts[1])),
                _ => {}
            }
            i = j;
            continue;
        }
        if cs[i] == '[' && cs.get(i + 1) == Some(&'[') {
            let rest: String = cs[i + 2..].iter().collect();
            if let Some(e) = rest.find("]]") {
                let link = &rest[..e];
                out.push_str(link.rsplit('|').next().unwrap_or(link));
                i += 2 + rest[..e].chars().count() + 2;
                continue;
            }
        }
        if cs[i] != '\'' {
            out.push(cs[i]);
        }
        i += 1;
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

const POS: [&str; 10] = ["Phrase", "Proverb", "Idiom", "Verb", "Noun", "Adjective", "Adverb", "Interjection", "Prepositional phrase", "Preposition"];

/// Register and meaning labels that define the domains (slang, formal language, humour, hidden meanings).
const REGISTER: [&str; 26] = [
    "slang", "informal", "colloquial", "formal", "humorous", "jocular", "euphemistic", "vulgar", "derogatory", "archaic", "dated", "figurative", "ironic", "sarcastic", "literary", "offensive",
    "internet slang", "legal", "law", "nonstandard", "idiomatic", "figuratively", "poetic", "officialese", "bureaucratese", "euphemism",
];

/// Regional varieties (labels → a canonical variety name).
const REGIONS: [(&str, &str); 24] = [
    ("us", "US"), ("american", "US"), ("north america", "North America"), ("canada", "Canada"), ("canadian", "Canada"),
    ("uk", "UK"), ("british", "UK"), ("england", "UK"), ("scotland", "Scotland"), ("scottish", "Scotland"), ("ireland", "Ireland"), ("irish", "Ireland"),
    ("wales", "Wales"), ("australia", "Australia"), ("au", "Australia"), ("australian", "Australia"), ("new zealand", "New Zealand"), ("nz", "New Zealand"),
    ("india", "India"), ("indian english", "India"), ("south africa", "South Africa"), ("singapore", "Singapore"), ("philippines", "Philippines"), ("nigeria", "Nigeria"),
];

/// Regional varieties of a definition line.
fn regions(line: &str) -> Vec<&'static str> {
    let low = line.to_lowercase();
    let mut out = Vec::new();
    for key in ["{{lb|en|", "{{label|en|", "{{lbl|en|"] {
        let mut rest = low.as_str();
        while let Some(a) = rest.find(key) {
            let after = &rest[a + key.len()..];
            let end = after.find("}}").unwrap_or(after.len());
            for part in after[..end].split('|') {
                if let Some((_, v)) = REGIONS.iter().find(|(k, _)| part.trim() == *k) {
                    if !out.contains(v) {
                        out.push(*v);
                    }
                }
            }
            rest = &after[end..];
        }
    }
    out
}

/// Register labels of a definition line (`# {{lb|en|slang|humorous}} …`).
fn labels(line: &str) -> Vec<&'static str> {
    let low = line.to_lowercase();
    let mut out = Vec::new();
    for key in ["{{lb|en|", "{{label|en|", "{{lbl|en|"] {
        let mut rest = low.as_str();
        while let Some(a) = rest.find(key) {
            let after = &rest[a + key.len()..];
            let end = after.find("}}").unwrap_or(after.len());
            for part in after[..end].split('|') {
                if let Some(r) = REGISTER.iter().find(|r| part.trim() == **r) {
                    if !out.contains(r) {
                        out.push(*r);
                    }
                }
            }
            rest = &after[end..];
        }
    }
    out
}

fn emit(out: &mut impl Write, title: &str, text: &str) -> std::io::Result<()> {
    let multi = title.contains(' ') || title.split('-').filter(|p| !p.is_empty() && p.chars().all(char::is_alphabetic)).count() >= 2;
    if title.contains(':') || title.chars().count() > 60 {
        return Ok(());
    }
    let Some(a) = text.find("==English==") else { return Ok(()) };
    let rest = &text[a + 11..];
    let en = match rest.find("\n==") {
        Some(b) if !rest[b + 3..].starts_with('=') => &rest[..b],
        _ => rest,
    };
    let mut pos = String::new();
    let mut gloss = String::new();
    let mut idiomatic = false;
    // register label → the gloss of the first sense that carries it
    let mut reg: Vec<(&'static str, String)> = Vec::new();
    let mut regs_of_region: Vec<(&'static str, String)> = Vec::new();
    for l in en.lines() {
        let t = l.trim();
        if t.starts_with("===") {
            let h = t.trim_matches('=').trim();
            if pos.is_empty() && POS.contains(&h) {
                pos = h.to_string();
            }
        } else if t.starts_with("# ") && !pos.is_empty() {
            let short = |g: String| if g.chars().count() > 160 { g.chars().take(160).collect::<String>() + "…" } else { g };
            if gloss.is_empty() {
                gloss = short(plain(&t[2..]));
            }
            for l in labels(t) {
                if !reg.iter().any(|x| x.0 == l) {
                    reg.push((l, short(plain(&t[2..]))));
                }
            }
            for v in regions(t) {
                if !regs_of_region.iter().any(|x| x.0 == v) {
                    regs_of_region.push((v, short(plain(&t[2..]))));
                }
            }
        }
        let low = t.to_lowercase();
        if (low.contains("{{lb|en|") || low.contains("{{label|en|")) && low.contains("idiomatic") {
            idiomatic = true;
        }
        if low.contains("category:english idioms") || low.contains("{{c|en|idioms") {
            idiomatic = true;
        }
    }
    if pos.is_empty() || (!multi && reg.is_empty() && regs_of_region.is_empty()) {
        return Ok(());
    }
    let regs = reg.iter().map(|(l, g)| format!("{l}: {}", g.replace(';', ","))).collect::<Vec<_>>().join("; ");
    let vars = regs_of_region.iter().map(|(l, g)| format!("{l}: {}", g.replace(';', ","))).collect::<Vec<_>>().join("; ");
    if !multi {
        return writeln!(out, "{title}\tword\t{pos}\t{}\t{}\t{}", gloss.replace('\t', " "), regs.replace('\t', " "), vars.replace('\t', " "));
    }
    let phrasal = en.to_lowercase().contains("phrasal verb");
    let kind = match (pos.as_str(), idiomatic, phrasal) {
        ("Proverb", _, _) => "proverb",
        ("Verb", _, true) => "phrasal-verb",
        (_, true, _) => "idiom",
        ("Idiom", _, _) => "idiom",
        ("Verb", _, _) => "verb-phrase",
        ("Phrase", _, _) => "phrase",
        _ => "multiword",
    };
    writeln!(out, "{title}\t{kind}\t{pos}\t{}\t{}\t{}", gloss.replace('\t', " "), regs.replace('\t', " "), vars.replace('\t', " "))
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut out = std::io::BufWriter::new(std::io::stdout());
    writeln!(out, "phrase\tkind\tpos\tgloss\tregister\tvariety")?;
    let (mut title, mut ns0, mut in_text, mut text) = (String::new(), false, false, String::new());
    for line in stdin.lock().lines() {
        let line = line?;
        let t = line.trim_start();
        if let Some(x) = t.strip_prefix("<title>") {
            title = unescape(x.trim_end_matches("</title>"));
        } else if t.starts_with("<ns>") {
            ns0 = t == "<ns>0</ns>";
        } else if t.starts_with("<text") {
            text.clear();
            let body = t.splitn(2, '>').nth(1).unwrap_or("");
            if let Some(b) = body.strip_suffix("</text>") {
                text.push_str(b);
                if ns0 {
                    emit(&mut out, &title, &unescape(&text))?;
                }
            } else {
                text.push_str(body);
                text.push('\n');
                in_text = true;
            }
        } else if in_text {
            if let Some(b) = line.strip_suffix("</text>") {
                text.push_str(b);
                in_text = false;
                if ns0 {
                    emit(&mut out, &title, &unescape(&text))?;
                }
            } else {
                text.push_str(&line);
                text.push('\n');
            }
        }
    }
    out.flush()?;
    Ok(())
}
