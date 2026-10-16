//! Morphology from VESUM (`dict_corp_vis.txt.bz2`: a lemma line, its forms indented below it).
//! Only what is needed is loaded: pass 1 — analyses of word forms from the sentence vocabulary; pass 2 — all forms
//! of the lemmas found, to synthesize a form back from (lemma, grammar) when checking the graph.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;
use bzip2::read::MultiBzDecoder;

use crate::gram::{CASE_NAMES, Gram, POS_NAMES};

#[derive(Clone, Debug)]
pub struct Analysis {
    /// Lemma key: `lemma|part_of_speech`.
    pub lemma: String,
    pub gram: Gram,
    /// Government from VESUM (`# rv_zna:rv_mis` on prepositions, `# rv_rod:rv_dav` on verbs):
    /// bit = case number, bit 0 — infinitive (`rv_inf`). 0 — government unknown.
    pub gov: u8,
}

#[derive(Default)]
pub struct Morph {
    pub forms: HashMap<String, Vec<Analysis>>,
    /// (lemma, grammar) → forms as in VESUM (with case), the main one first.
    pub synth: HashMap<(String, Gram), Vec<String>>,
}

/// Invisible and stress marks that occur inside words (stress U+0301, zero-width, soft hyphen).
pub fn invisible(c: char) -> bool {
    matches!(c, '\u{0300}' | '\u{0301}' | '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' | '\u{00AD}')
}

/// Normalization for lookup: lowercase, apostrophes → `'`, no stress marks or invisible characters.
pub fn norm(s: &str) -> String {
    s.chars()
        .filter(|&c| !invisible(c))
        .map(|c| if matches!(c, '’' | 'ʼ' | '`' | '‘') { '\'' } else { c })
        .collect::<String>()
        .to_lowercase()
}

/// VESUM line → (form, tags, comment); `lemma_line` — whether this is a lemma line.
fn split_line(line: &str) -> Option<(bool, &str, &str, &str)> {
    let lemma_line = !line.starts_with(' ');
    let body = line.trim_start();
    let (body, comment) = body.split_once('#').map_or((body, ""), |(b, c)| (b.trim_end(), c.trim()));
    let (form, tags) = body.split_once(' ')?;
    Some((lemma_line, form, tags.trim(), comment))
}

/// Lemma tags plus the form's paradigm marker (in VESUM `xpN` is on forms, not always on the lemma).
fn with_xp(lemma_tags: &str, form_tags: &str) -> String {
    match form_tags.split(':').find(|t| t.starts_with("xp") && t.len() > 2) {
        Some(xp) if !lemma_tags.split(':').any(|t| t == xp) => format!("{lemma_tags}:{xp}"),
        _ => lemma_tags.to_string(),
    }
}

fn gov_mask(comment: &str) -> u8 {
    let mut m = 0u8;
    for t in comment.split(|c: char| c == ':' || c.is_whitespace()) {
        if let Some(c) = t.strip_prefix("rv_") {
            if c == "inf" {
                m |= 1;
            } else if let Some(i) = CASE_NAMES.iter().position(|n| n.strip_prefix("v_") == Some(c)) {
                m |= 1 << i;
            }
        }
    }
    m
}

/// Lemma key `lemma|part_of_speech`, plus `#xpN` for homonymous VESUM paradigms ("боліти" 1 and 2).
fn lemma_key(lemma: &str, tags: &str) -> String {
    let pos = tags.split(':').next().unwrap_or("");
    let pos = if POS_NAMES.contains(&pos) { pos } else { "other" };
    match tags.split(':').find(|t| t.starts_with("xp") && t.len() > 2) {
        Some(xp) => format!("{lemma}|{pos}#{xp}"),
        None => format!("{lemma}|{pos}"),
    }
}

/// Lemma from a key: `боліти|verb#xp2` → `боліти`.
pub fn lemma_of(key: &str) -> &str {
    key.split('|').next().unwrap_or(key)
}

fn for_each_entry(path: &Path, mut f: impl FnMut(&str, &str, &str, &str)) -> Result<()> {
    let reader = BufReader::with_capacity(1 << 20, MultiBzDecoder::new(File::open(path)?));
    let (mut lemma, mut lemma_tags) = (String::new(), String::new());
    for line in reader.lines() {
        let line = line?;
        let Some((is_lemma, form, tags, comment)) = split_line(&line) else { continue };
        if is_lemma {
            lemma = form.to_string();
            lemma_tags = tags.to_string();
        }
        let _ = comment;
        // the lemma key uses the part of speech of the lemma line
        f(&lemma, &lemma_tags, form, tags);
    }
    Ok(())
}

impl Morph {
    pub fn load(vesum: &Path, vocab: &HashSet<String>) -> Result<Morph> {
        let mut m = Morph::default();
        let mut lemmas = HashSet::new();
        // pass 1: analyses of the needed forms (the comment is needed for preposition government)
        let reader = BufReader::with_capacity(1 << 20, MultiBzDecoder::new(File::open(vesum)?));
        let (mut lemma, mut lemma_tags, mut lemma_gov) = (String::new(), String::new(), 0u8);
        for line in reader.lines() {
            let line = line?;
            let Some((is_lemma, form, tags, comment)) = split_line(&line) else { continue };
            if is_lemma {
                lemma = form.to_string();
                lemma_tags = tags.to_string();
                lemma_gov = gov_mask(comment);
            }
            let n = norm(form);
            if !vocab.contains(&n) {
                continue;
            }
            let key = lemma_key(&lemma, &with_xp(&lemma_tags, tags));
            // government comes from the lemma line; verb forms and prepositions inherit it
            let gov = if tags.starts_with("prep") || tags.starts_with("verb") { lemma_gov } else { 0 };
            m.forms.entry(n).or_default().push(Analysis { lemma: key.clone(), gram: Gram::from_vesum(tags), gov });
            lemmas.insert(key);
        }
        // pass 2: all forms of the lemmas found — for synthesis
        for_each_entry(vesum, |lemma, lemma_tags, form, tags| {
            let key = lemma_key(lemma, &with_xp(lemma_tags, tags));
            if lemmas.contains(&key) {
                // the form as in the dictionary (proper-name case: "Нью-Йорка"); compared via norm()
                m.synth.entry((key, Gram::from_vesum(tags))).or_default().push(form.to_string());
            }
        })?;
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_and_government() {
        assert_eq!(split_line("в prep    # rv_zna:rv_mis:rv_rod"), Some((true, "в", "prep", "rv_zna:rv_mis:rv_rod")));
        assert_eq!(split_line("  кота noun:anim:m:v_rod"), Some((false, "кота", "noun:anim:m:v_rod", "")));
        let m = gov_mask("rv_zna:rv_mis");
        assert_eq!(m, (1 << 4) | (1 << 6));
        assert_eq!(norm("М’ЯСО"), "м'ясо");
    }
}
