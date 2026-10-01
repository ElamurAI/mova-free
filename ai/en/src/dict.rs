//! Built-in dictionary: a lemma heap (frozen numbers) and the full English form expansion (AGID + UD),
//! assembled by `build.rs` from `data/` into static hash tables inside the binary: at run time nothing
//! is built or read from disk. Words the rules rely on are constants (`w::THE`,
//! `w::QMARK`…); checking a rule is an integer comparison.

use crate::gram::{Sym, Tag};
use crate::hash::str_key;

static BLOB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/lexicon.bin"));
include!(concat!(env!("OUT_DIR"), "/lexicon_layout.rs"));

/// Lemma numbers as constants in code.
pub mod w {
    #[allow(unused_imports)]
    use crate::gram::Sym;
    include!(concat!(env!("OUT_DIR"), "/words.rs"));
}

/// Sources of a form entry: UD (with frequency), the main form of an AGID cell, a clean AGID form for
/// reverse lookup, an AGID base (word class for the tagger).
pub const UD: u8 = 1;
pub const AGID: u8 = 2;
pub const REV: u8 = 4;
pub const BASE: u8 = 8;

#[inline]
fn u32_at(o: usize) -> u32 {
    u32::from_le_bytes([BLOB[o], BLOB[o + 1], BLOB[o + 2], BLOB[o + 3]])
}

fn lemma_bytes(i: usize) -> &'static [u8] {
    &BLOB[L_STR + u32_at(L_OFF + 4 * i) as usize..L_STR + u32_at(L_OFF + 4 * i + 4) as usize]
}

fn form_bytes(i: usize) -> &'static [u8] {
    &BLOB[F_STR + u32_at(F_OFF + 4 * i) as usize..F_STR + u32_at(F_OFF + 4 * i + 4) as usize]
}

/// Lookup in an open-addressing table.
fn probe(s: &str, idx: usize, mask: usize, n: usize, bytes: fn(usize) -> &'static [u8]) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let mut slot = str_key(s) as usize & mask;
    loop {
        let v = u32_at(idx + 4 * slot) as usize;
        if v == 0 {
            return None;
        }
        if bytes(v - 1) == s.as_bytes() {
            return Some(v - 1);
        }
        slot = (slot + 1) & mask;
    }
}

/// Fingerprint of the built-in dictionary (hash of the whole blob). Lemma numbers and dictionary masks are baked into a trained
/// model, so a model file with a different dictionary does not load (`store::unpack`).
pub fn fingerprint() -> u64 {
    static FP: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *FP.get_or_init(|| crate::store::checksum(BLOB))
}

/// Lemma number (string as is).
pub fn lemma(s: &str) -> Option<Sym> {
    probe(s, L_IDX, L_MASK, N_LEMMAS, lemma_bytes).map(|i| Sym::global(i as u32))
}

/// Lemma string from the heap (for local numbers — empty: they live in the graph).
pub fn text(s: Sym) -> &'static str {
    if s.is_local() || s.index() >= N_LEMMAS {
        return "";
    }
    std::str::from_utf8(lemma_bytes(s.index())).unwrap_or("")
}

/// Number of the lowercase variant (or `NONE`).
pub fn lower(s: Sym) -> Sym {
    if s.is_local() || s.index() >= N_LEMMAS {
        return Sym::NONE;
    }
    match u32_at(L_LOWER + 4 * s.index()) {
        u32::MAX => Sym::NONE,
        l => Sym::global(l),
    }
}

/// Closed-class key from a string: number of the lowercase variant or `NONE`.
pub fn lex(s: &str) -> Sym {
    lemma(&s.to_lowercase()).unwrap_or(Sym::NONE)
}

/// Lemma known to morphology (AGID without "?" or UD).
pub fn known(s: Sym) -> bool {
    !s.is_local() && s.index() < N_LEMMAS && BLOB[L_FLAGS + s.index()] & 1 != 0
}

/// Name of the code constant for a lemma (`w::THE`), if any.
pub fn const_name(s: Sym) -> Option<&'static str> {
    if s.is_local() {
        return None;
    }
    w::NAMES.binary_search_by_key(&s.raw(), |x| x.0).ok().map(|i| w::NAMES[i].1)
}

/// Form number in the full expansion.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Form(u32);

pub fn form(s: &str) -> Option<Form> {
    probe(s, F_IDX, F_MASK, N_FORMS, form_bytes).map(|i| Form(i as u32))
}

pub fn form_text(f: Form) -> &'static str {
    std::str::from_utf8(form_bytes(f.0 as usize)).unwrap_or("")
}

/// Dictionary entry: tag, sources, number (of the lemma for analyses, of the form for paradigms), UD frequency.
#[derive(Clone, Copy, Debug)]
pub struct Rec {
    pub tag: Tag,
    pub src: u8,
    pub id: u32,
    pub count: u32,
}

impl Rec {
    /// Lemma number (for form analyses).
    pub fn lemma(&self) -> Sym {
        Sym::global(self.id)
    }
    /// Form number (for lemma paradigms).
    pub fn form(&self) -> Form {
        Form(self.id)
    }
}

fn recs(off: usize, rec: usize, i: usize) -> impl Iterator<Item = Rec> {
    let (a, b) = (u32_at(off + 4 * i) as usize, u32_at(off + 4 * i + 4) as usize);
    (a..b).map(move |k| {
        let o = rec + 12 * k;
        Rec { tag: Tag::ALL[BLOB[o] as usize], src: BLOB[o + 1], id: u32_at(o + 4), count: u32_at(o + 8) }
    })
}

/// Form analyses: (tag, lemma) in dictionary order — UD by first occurrence, then AGID by file.
pub fn analyses(f: Form) -> impl Iterator<Item = Rec> {
    recs(A_OFF, A_REC, f.0 as usize)
}

/// Lemma paradigm: (tag, form): UD (under the lowercase lemma) and the main AGID forms.
pub fn paradigm(l: Sym) -> Box<dyn Iterator<Item = Rec>> {
    if l.is_local() || l.index() >= N_LEMMAS {
        return Box::new(std::iter::empty());
    }
    Box::new(recs(P_OFF, P_REC, l.index()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_match_build() {
        assert_eq!(TAG_NAMES.len(), Tag::N);
        for (i, t) in Tag::ALL.iter().enumerate() {
            assert_eq!(TAG_NAMES[i], t.name());
        }
    }

    #[test]
    fn lookups() {
        if N_LEMMAS == 0 {
            return; // first build without the dictionary
        }
        let the = lemma("the").expect("the");
        assert_eq!(text(the), "the");
        assert_eq!(lower(lemma("The").unwrap_or(the)), the);
        let went = form("went").expect("went");
        assert!(analyses(went).any(|r| r.tag == Tag::VBD && text(Sym::global(r.id)) == "go"));
        let go = lemma("go").expect("go");
        assert!(paradigm(go).any(|r| r.tag == Tag::VBD && form_text(Form(r.id)) == "went"));
        assert!(known(go));
        assert_eq!(lemma("zzzqqq"), None);
    }
}
