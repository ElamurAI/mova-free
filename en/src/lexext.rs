//! The lexicon extension: ordinary words the in-code dictionary (`dict`, AGID + UD) lacks, from Open English WordNet,
//! the SPECIALIST Lexicon and the 2010 U.S. Census surnames (`lexgrow.rs` makes `data/lexicon-ext.tsv` and
//! `data/acronyms.tsv`; `build.rs` compiles them into a blob of their own). The dictionary's blob, its fingerprint
//! and the lemma numbers stay as they are, so a trained model loads and reads as before; the extension only answers
//! for what the dictionary does not know.
//!
//! - `tags(form)`: the extension's tags of a lower-case form (open classes and proper nouns);
//! - `lemma(form, tag)`: its lemma under that tag (*vertices* NNS → vertex);
//! - `form(lemma, tag)`: the form of a lemma in that cell (vertex NNS → vertices);
//! - `expansions(acronym)`: an acronym's expansions with their sources (case as written: *GPU*, *LHCb*).
//!
//! `EN_LEXEXT=0` turns it off (the reference: Mova as before the extension).

use std::sync::OnceLock;

use crate::gram::Tag;
use crate::hash::str_key;

static BLOB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/lexext.bin"));
include!(concat!(env!("OUT_DIR"), "/lexext_layout.rs"));

/// Source bits of a row.
pub const OEWN: u8 = 1;
pub const SPECIALIST: u8 = 2;
pub const CENSUS: u8 = 4;
pub const MOVA: u8 = 8;
pub const NAMES: u8 = 16;

/// Whether the extension is on (`EN_LEXEXT=0`: off).
pub fn on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("EN_LEXEXT").map_or(true, |v| v != "0"))
}

#[inline]
fn u32_at(o: usize) -> u32 {
    u32::from_le_bytes([BLOB[o], BLOB[o + 1], BLOB[o + 2], BLOB[o + 3]])
}

fn bytes(str_sec: usize, off_sec: usize, i: usize) -> &'static [u8] {
    &BLOB[str_sec + u32_at(off_sec + 4 * i) as usize..str_sec + u32_at(off_sec + 4 * i + 4) as usize]
}

fn text(str_sec: usize, off_sec: usize, i: usize) -> &'static str {
    std::str::from_utf8(bytes(str_sec, off_sec, i)).unwrap_or("")
}

fn probe(s: &str, str_sec: usize, off_sec: usize, idx: usize, mask: usize, n: usize) -> Option<usize> {
    if n == 0 || !on() {
        return None;
    }
    let mut slot = str_key(s) as usize & mask;
    loop {
        let v = u32_at(idx + 4 * slot) as usize;
        if v == 0 {
            return None;
        }
        if bytes(str_sec, off_sec, v - 1) == s.as_bytes() {
            return Some(v - 1);
        }
        slot = (slot + 1) & mask;
    }
}

/// One record: a tag, its source bits and the id it points to (a lemma, a form or an expansion).
#[derive(Clone, Copy, Debug)]
pub struct Rec {
    pub tag: Tag,
    pub src: u8,
    id: u32,
}

fn recs(goff: usize, rec: usize, i: usize) -> impl Iterator<Item = Rec> {
    let (a, b) = (u32_at(goff + 4 * i) as usize, u32_at(goff + 4 * i + 4) as usize);
    (a..b).map(move |k| {
        let o = rec + 8 * k;
        Rec { tag: Tag::ALL.get(BLOB[o] as usize).copied().unwrap_or(Tag::X), src: BLOB[o + 1], id: u32_at(o + 4) }
    })
}

/// The extension's analyses of a lower-case form: tag, sources and lemma. The plural of an acronym it has is NNPS
/// (*gpus* → GPU), the only form made here rather than read: an acronym takes a bare -s.
pub fn analyses(lower: &str) -> Vec<(Tag, u8, &'static str)> {
    if let Some(f) = probe(lower, XF_STR, XF_OFF, XF_IDX, XF_MASK, N_XF) {
        return recs(XF_GOFF, XF_REC, f).map(|r| (r.tag, r.src, text(XL_STR, XL_OFF, r.id as usize))).collect();
    }
    acronym_plural(lower).into_iter().collect()
}

/// *gpus*: the stem is a form of the extension read as an acronym (NNP, its lemma all capitals: GPU, FPGA).
fn acronym_plural(lower: &str) -> Option<(Tag, u8, &'static str)> {
    let stem = lower.strip_suffix('s').filter(|s| s.chars().count() >= 2)?;
    let f = probe(stem, XF_STR, XF_OFF, XF_IDX, XF_MASK, N_XF)?;
    recs(XF_GOFF, XF_REC, f)
        .map(|r| (r.tag, r.src, text(XL_STR, XL_OFF, r.id as usize)))
        .find(|(t, _, l)| *t == Tag::NNP && crate::lexgrow::acronym_ok(l) && l.chars().all(|c| !c.is_lowercase()))
        .map(|(_, s, l)| (Tag::NNPS, s, l))
}

/// A form the extension has.
pub fn known(lower: &str) -> bool {
    probe(lower, XF_STR, XF_OFF, XF_IDX, XF_MASK, N_XF).is_some() || acronym_plural(lower).is_some()
}

/// The extension's tags of a lower-case form, sorted by tag index, without repeats.
pub fn tags(lower: &str) -> Vec<Tag> {
    let mut v: Vec<Tag> = analyses(lower).into_iter().map(|a| a.0).collect();
    v.sort_by_key(|t| t.idx());
    v.dedup();
    v
}

/// Tags for the taggers' dictionary hint on a word the dictionary does not know, by how it is written (`inside_cap`:
/// capitalised inside a sentence, where it is more likely a name):
/// - the rows of an acronym or a name spelt with two capitals (*GPU*, *IgH*) only for a word with two capitals or more
///   (not *amd* or *sry* in a chat);
/// - rows only from the name lists (Census, Names Corpus), and a name of three letters or fewer, only for a
///   capitalised word (*Zhang*; not *goin*, nor the spelling chunks *sh, kn* that SPECIALIST has as names);
/// - a capitalised word inside a sentence only when a row left is a proper noun (*Safed* is no form of *safe*;
///   *Stochastic* in a title keeps its old reading).
pub fn hint(word: &str, inside_cap: bool) -> Vec<Tag> {
    let low = word.to_lowercase();
    let caps = word.chars().filter(|c| c.is_uppercase()).count();
    let cap = word.chars().next().is_some_and(char::is_uppercase);
    let rows: Vec<(Tag, u8, &str)> = analyses(&low)
        .into_iter()
        .filter(|(t, src, l)| {
            let acronym = crate::lexgrow::acronym_ok(l) || (matches!(t, Tag::NNP | Tag::NNPS) && l.chars().filter(|c| c.is_uppercase()).count() >= 2);
            let names_only = src & !(CENSUS | NAMES) == 0;
            let short_name = matches!(t, Tag::NNP | Tag::NNPS) && low.chars().count() <= 3;
            (!acronym || caps >= 2) && (!(names_only || short_name) || cap)
        })
        .collect();
    if inside_cap && !rows.iter().any(|r| matches!(r.0, Tag::NNP | Tag::NNPS)) {
        return Vec::new();
    }
    let mut v: Vec<Tag> = rows.into_iter().map(|r| r.0).collect();
    v.sort_by_key(|t| t.idx());
    v.dedup();
    v
}

/// The lemma of a lower-case form under a tag (the first row with it).
pub fn lemma(lower: &str, tag: Tag) -> Option<&'static str> {
    analyses(lower).into_iter().find(|a| a.0 == tag).map(|a| a.2)
}

/// The form of a lemma (any case) in a tag's cell.
pub fn form(lemma: &str, tag: Tag) -> Option<&'static str> {
    let p = probe(&lemma.to_lowercase(), XP_STR, XP_OFF, XP_IDX, XP_MASK, N_XP)?;
    recs(XP_GOFF, XP_REC, p).find(|r| r.tag == tag).map(|r| text(XF_STR, XF_OFF, r.id as usize))
}

/// An acronym's expansions, each with its source (`s:<EUI>` SPECIALIST, `o:<synset>` OEWN), in file order.
pub fn expansions(acronym: &str) -> Vec<(&'static str, &'static str)> {
    match probe(acronym, XA_STR, XA_OFF, XA_IDX, XA_MASK, N_XA) {
        Some(a) => recs(XA_GOFF, XA_REC, a)
            .map(|r| {
                let t = text(XE_STR, XE_OFF, r.id as usize);
                t.split_once('\t').unwrap_or((t, ""))
            })
            .collect(),
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grown_words() {
        if N_XF == 0 {
            return;
        }
        // plain adjectives the dictionary lacked
        for w in ["grammatical", "lexical", "artificial", "stochastic", "contextual"] {
            assert!(tags(w).contains(&Tag::JJ), "{w}: {:?}", tags(w));
        }
        // no comparison where the source gives none
        assert!(!known("lexicaler"));
        // an irregular plural, both ways
        assert_eq!(lemma("vertices", Tag::NNS), Some("vertex"));
        assert_eq!(form("vertex", Tag::NNS), Some("vertices"));
        // an acronym's plural; not a lower-case word's
        assert_eq!(lemma("gpus", Tag::NNPS), Some("GPU"));
        assert!(!known("lexicals"));
        // the taggers' hint reads the case: an acronym in capitals only, a name-list surname capitalised only
        assert!(hint("GPU", true).contains(&Tag::NNP) && hint("gpu", false).is_empty());
        assert!(hint("Aaronson", true).contains(&Tag::NNP) && hint("aaronson", false).is_empty());
        assert!(hint("Stochastic", true).is_empty() && hint("stochastic", false).contains(&Tag::JJ));
        assert!(hint("sh", false).is_empty() && hint("kerala", false).contains(&Tag::NNP));
        // a class the dictionary lacked for a form it knows
        assert!(tags("directional").contains(&Tag::JJ));
        // negative control: misspellings and nonsense are not there (SPECIALIST's *noticable* is a variant of a
        // dictionary word, left out)
        for w in ["happyness", "qwzxv", "noticable", "occured", "arguement"] {
            assert!(!known(w), "{w}");
        }
    }

    #[test]
    fn acronyms() {
        if N_XA == 0 {
            return;
        }
        assert!(expansions("AI").iter().any(|(e, _)| *e == "artificial intelligence"), "{:?}", expansions("AI"));
        assert!(expansions("ai").is_empty(), "acronyms are case-sensitive");
    }
}
