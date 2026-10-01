//! Mentions from a UD tree. The head is nominal: NOUN, PROPN, PRON (except expl), and also DET and NUM when they
//! themselves stand in a nominal role (subject, object, oblique, nmod, appos…). The span is the head's subtree
//! (as in CorefUD), without punctuation at the edges; coordination gives two mentions with the same head: the whole group
//! ("John and Mary") and the first conjunct without conj/cc ("John") — the CRAC scorer tells them apart by span.
//!
//! Features come only from the tree and UD FEATS (Number, Gender, Person, PronType, Poss, Reflex) and closed classes:
//! a table of English pronouns and titles (Mr., Mrs.…) — our own, hand-made. There are no dictionaries of name gender or
//! animacy (licensing) — unknown stays empty and does not get in the way (like "unknown" in Lee et al. 2013).
//! We do not guess gender from a name.

use en::gram::{Feat, Rel, Tag, UPos};

use crate::corefud::Span;
use crate::doc::{Document, Sent};

/// Number, gender, animacy, person — bit sets; 0 means unknown (matches everything).
pub const SG: u8 = 1;
pub const PL: u8 = 2;
pub const MASC: u8 = 1;
pub const FEM: u8 = 2;
pub const NEUT: u8 = 4;
pub const ANIM: u8 = 1;
pub const INAN: u8 = 2;
pub const P1: u8 = 1;
pub const P2: u8 = 2;
pub const P3: u8 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Pronoun,
    Proper,
    Nominal,
    /// DET or NUM in a nominal role.
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pron {
    Personal,
    Possessive,
    Reflexive,
    Demonstrative,
    Relative,
    Interrogative,
    Indefinite,
    Reciprocal,
    Other,
}

#[derive(Clone, Debug)]
pub struct Mention {
    pub span: Span,
    pub head: usize,
    pub kind: Kind,
    pub pron: Option<Pron>,
    pub number: u8,
    pub gender: u8,
    pub animacy: u8,
    pub person: u8,
    /// The whole coordination group ("John and Mary").
    pub coord: bool,
    /// Starts with a/an, an indefinite pronoun or a bare plural — not linked as a new entity (Lee et al. 2013).
    pub indefinite: bool,
    /// Quantified group (every, each, no, any, either… or the quantifier itself as head): not an antecedent for definites.
    pub quantified: bool,
    /// Only clauses (acl) after the head — for relaxed: "Clinton, whose term ends…" ~ "Clinton".
    pub post_clausal_only: bool,
    /// Memory only: does not go into the GUM profile output (relative and interrogative pronouns).
    pub internal: bool,
    /// Span text, lowercased.
    pub text: String,
    /// Text from the start of the span up to and including the head, lowercased.
    pub pre_head: String,
    /// Head, lowercased (form).
    pub head_low: String,
    /// Head key for head-based sieves: in a proper name with flat — the last word of the name ("Barack Obama" → obama),
    /// because in UD the head of a flat name is the first word.
    pub head_key: String,
    /// Content words of the span (NOUN, PROPN, ADJ, NUM, VERB — lowercased forms, without titles).
    pub words: Vec<String>,
    /// Head modifiers: nouns and adjectives in the span outside subordinate clauses, appos and conj.
    pub mods: Vec<String>,
    /// Proper names and numbers among the modifiers (for proper_head: "southern Lebanon" ≠ "Lebanon").
    pub proper_mods: Vec<String>,
    /// Possessor (nmod:poss) — a word of the sentence.
    pub possessor: Option<usize>,
    /// Quote number of the head (0 — outside quotes).
    pub quote: u32,
    pub speaker: Option<String>,
    /// Head relation (without subtype).
    pub role: Rel,
    pub depth: u32,
}

impl Mention {
    pub fn is_pron(&self) -> bool {
        self.kind == Kind::Pronoun
    }
    /// Personal, possessive or reflexive pronoun.
    pub fn is_ppr(&self) -> bool {
        matches!(self.pron, Some(Pron::Personal | Pron::Possessive | Pron::Reflexive))
    }
    pub fn third(&self) -> bool {
        self.person == P3
    }
}

/// Nouns with gender in the word itself (kinship and gendered roles): a hand-made list, not people's names.
/// Registry: row `coref-gender-nouns` in en/data/train-licenses.tsv.
fn gender_noun(lemma: &str) -> Option<u8> {
    match lemma {
        "man" | "men" | "boy" | "father" | "dad" | "daddy" | "son" | "brother" | "husband" | "uncle" | "nephew" | "grandfather" | "grandson" | "king" | "prince" | "lord" | "gentleman" | "sir" | "mr" | "mr." | "guy" | "fiancé" | "boyfriend" | "stepfather" | "monk" | "priest" | "emperor" | "duke" | "baron" | "earl" => Some(MASC),
        "woman" | "women" | "girl" | "mother" | "mom" | "mum" | "mommy" | "daughter" | "sister" | "wife" | "aunt" | "niece" | "grandmother" | "granddaughter" | "queen" | "princess" | "lady" | "madam" | "mrs" | "mrs." | "ms" | "ms." | "miss" | "fiancée" | "girlfriend" | "stepmother" | "nun" | "actress" | "empress" | "duchess" | "baroness" | "countess" | "waitress" => Some(FEM),
        _ => None,
    }
}

/// Titles: gender (if the text states it) — Mr./Mrs./Ms.… Closed class only, not names.
fn title(low: &str) -> Option<u8> {
    match low.trim_end_matches('.') {
        "mr" | "mister" | "sir" => Some(MASC),
        "mrs" | "ms" | "miss" | "madam" | "mme" => Some(FEM),
        "dr" | "prof" | "professor" => Some(0),
        _ => None,
    }
}

struct PronInfo {
    pt: Pron,
    person: u8,
    number: u8,
    gender: u8,
    animacy: u8,
}

/// Table of English pronouns (closed class): type, person, number, gender, animacy.
fn pron_table(low: &str) -> Option<PronInfo> {
    use Pron::*;
    let p = |pt, person, number, gender, animacy| Some(PronInfo { pt, person, number, gender, animacy });
    match low {
        "i" | "me" => p(Personal, P1, SG, 0, ANIM),
        "we" | "us" => p(Personal, P1, PL, 0, ANIM),
        "you" | "ya" | "y'all" | "thee" | "thou" => p(Personal, P2, 0, 0, ANIM),
        "he" | "him" => p(Personal, P3, SG, MASC, ANIM),
        "she" => p(Personal, P3, SG, FEM, ANIM),
        "her" => p(Personal, P3, SG, FEM, ANIM),
        "it" => p(Personal, P3, SG, NEUT, INAN),
        "they" | "them" | "'em" => p(Personal, P3, PL, 0, 0),
        "my" | "mine" => p(Possessive, P1, SG, 0, ANIM),
        "our" | "ours" => p(Possessive, P1, PL, 0, ANIM),
        "your" | "yours" | "thy" => p(Possessive, P2, 0, 0, ANIM),
        "his" => p(Possessive, P3, SG, MASC, ANIM),
        "hers" => p(Possessive, P3, SG, FEM, ANIM),
        "its" => p(Possessive, P3, SG, NEUT, INAN),
        "their" | "theirs" => p(Possessive, P3, PL, 0, 0),
        "myself" => p(Reflexive, P1, SG, 0, ANIM),
        "ourselves" => p(Reflexive, P1, PL, 0, ANIM),
        "yourself" => p(Reflexive, P2, SG, 0, ANIM),
        "yourselves" => p(Reflexive, P2, PL, 0, ANIM),
        "himself" => p(Reflexive, P3, SG, MASC, ANIM),
        "herself" => p(Reflexive, P3, SG, FEM, ANIM),
        "itself" => p(Reflexive, P3, SG, NEUT, INAN),
        "themselves" | "themself" => p(Reflexive, P3, PL, 0, 0),
        "this" | "that" => p(Demonstrative, P3, SG, 0, 0),
        "these" | "those" => p(Demonstrative, P3, PL, 0, 0),
        "who" | "whom" | "whose" => p(Relative, P3, 0, 0, ANIM),
        "which" => p(Relative, P3, 0, 0, INAN),
        "what" | "whatever" | "whoever" | "whichever" => p(Interrogative, P3, 0, 0, 0),
        "someone" | "somebody" | "anyone" | "anybody" | "everyone" | "everybody" | "nobody" | "noone" => p(Indefinite, P3, SG, 0, ANIM),
        "something" | "anything" | "everything" | "nothing" => p(Indefinite, P3, SG, 0, INAN),
        "one" | "another" | "each" | "either" | "neither" => p(Indefinite, P3, SG, 0, 0),
        "all" | "some" | "any" | "none" | "both" | "many" | "few" | "several" | "others" | "most" | "more" | "less" | "much" => p(Indefinite, P3, 0, 0, 0),
        _ => None,
    }
}

/// Mention detection switches for experiments on dev (`COREF_MD=name,name`); the default is what was chosen on dev.
fn opt(name: &str) -> bool {
    std::env::var("COREF_MD").is_ok_and(|v| v.split(',').any(|x| x == name))
}

/// Non-nominal idioms with a noun (for example, in fact, a lot…): not mentions. Hand-made closed list.
fn idiom(s: &Sent, h: usize) -> bool {
    let w = s.low(h);
    let prev = if h > 0 { s.low(h - 1) } else { String::new() };
    let next = if h + 1 < s.len() { s.low(h + 1) } else { String::new() };
    match w.as_str() {
        "example" | "instance" => prev == "for",
        "fact" => prev == "in",
        "lot" | "lots" | "bit" | "couple" => (prev == "a" || w == "lots") && (next == "of" || s.rel(h).base() == Rel::Obl || s.rel(h).base() == Rel::Obj || s.rel(h).base() == Rel::Nmod),
        "kind" | "sort" => next == "of" && s.rel(h).base() != Rel::Nsubj,
        "terms" | "behalf" | "front" | "spite" | "order" | "case" => prev == "in" || prev == "on",
        "respect" => prev == "in" || prev == "with",
        "course" => prev == "of",
        "least" | "most" => prev == "at",
        "way" => prev == "by",
        _ => false,
    }
}

/// Roles in which DET or NUM is itself the noun.
fn nominal_role(r: Rel) -> bool {
    matches!(r.base(), Rel::Nsubj | Rel::Obj | Rel::Iobj | Rel::Obl | Rel::Nmod | Rel::Appos | Rel::Root | Rel::Conj | Rel::Dislocated | Rel::Vocative | Rel::Xcomp | Rel::List | Rel::Orphan)
}

/// Whether the word is a mention head.
fn is_head(s: &Sent, i: usize) -> Option<Kind> {
    let r = s.rel(i);
    let bad = |u: UPos| {
        title(&s.low(i)).is_some()
            || matches!(r.base(), Rel::Flat | Rel::Fixed | Rel::Goeswith | Rel::Reparandum)
            || r == Rel::NmodDesc && !opt("desc")
            || r.base() == Rel::Compound && !(opt("compound") || u == UPos::PROPN && !opt("nopcompound"))
            || idiom(s, i) && !opt("noidiom")
    };
    match s.upos(i) {
        Some(UPos::NOUN) => (!bad(UPos::NOUN)).then_some(Kind::Nominal),
        Some(UPos::PROPN) => (!bad(UPos::PROPN)).then_some(Kind::Proper),
        // "'s" in let's is not a mention (not annotated in GUM)
        Some(UPos::PRON) => (r != Rel::Expl && !matches!(r.base(), Rel::Fixed | Rel::Goeswith | Rel::Reparandum) && !matches!(s.low(i).as_str(), "'s" | "’s")).then_some(Kind::Pronoun),
        Some(UPos::DET) => nominal_role(r).then_some(Kind::Other),
        Some(UPos::NUM) => (nominal_role(r) || r == Rel::Parataxis).then_some(Kind::Other),
        Some(UPos::ADV) if opt("adv") => (matches!(s.low(i).as_str(), "home" | "here" | "there") && r.base() != Rel::Expl).then_some(Kind::Other),
        Some(UPos::ADV) => (s.low(i) == "home" && matches!(r.base(), Rel::Advmod | Rel::Obl | Rel::Root)).then_some(Kind::Other),
        _ => None,
    }
}

/// Words of the subtree of `h` without the subtrees of children for which `cut` is true; the span runs from first to last,
/// without punctuation at the edges.
/// Head children that belong to the nominal group (the rest is the clause around a predicative noun: nsubj, cop,
/// aux, mark, obl, advcl…, — and is not part of the mention).
fn np_internal(s: &Sent, h: usize, k: usize) -> bool {
    match s.rel(k).base() {
        Rel::Det | Rel::Amod | Rel::Nummod | Rel::Compound | Rel::Flat | Rel::Fixed | Rel::Nmod | Rel::Acl | Rel::Clf | Rel::Goeswith | Rel::Cc | Rel::Dep => true,
        // a conjunct clause ("a waiver, or is it a modification") is not part of the group
        Rel::Conj => is_head(s, k).is_some() && !s.kids[k].iter().any(|&x| matches!(s.rel(x).base(), Rel::Cop | Rel::Nsubj | Rel::Aux)),
        // a preposition before the head is not part of the mention (CorefUD); a possessive 's after it is
        Rel::Case => k > h,
        Rel::Advmod => opt("advmod") && k < h,
        Rel::Appos => opt("appos-in"),
        Rel::Punct => true,
        _ => false,
    }
}

/// Words of the subtree of `h` without subtrees of children outside the nominal group and those for which `cut` is true; the span
/// runs from first to last, without punctuation at the edges.
fn span_of(s: &Sent, h: usize, cut: &dyn Fn(usize) -> bool) -> (usize, usize) {
    let mut inside = vec![false; s.len()];
    let mut stack = vec![h];
    while let Some(x) = stack.pop() {
        inside[x] = true;
        for &k in &s.kids[x] {
            if x == h && (cut(k) || !np_internal(s, h, k) || (s.rel(k).base() == Rel::Cc && k < h)) {
                continue;
            }
            stack.push(k);
        }
    }
    // the span runs from the first to the last word of the group that is neither punctuation nor a symbol
    let edge = |i: usize| matches!(s.upos(i), Some(UPos::PUNCT | UPos::SYM));
    let words: Vec<usize> = (0..s.len()).filter(|&i| inside[i] && (!edge(i) || i == h)).collect();
    let (mut lo, mut hi) = (words[0], *words.last().unwrap());
    // a bracket or quote at the edge is kept if its pair is inside the span
    let opening = |f: &str| matches!(f, "(" | "[" | "{" | "\"" | "“" | "``");
    let closing = |f: &str| matches!(f, ")" | "]" | "}" | "\"" | "”" | "''");
    let pair = |o: &str, c: &str| matches!((o, c), ("(", ")") | ("[", "]") | ("{", "}") | ("\"", "\"") | ("“", "”") | ("``", "''"));
    if hi + 1 < s.len() && inside[hi + 1] && closing(&s.toks[hi + 1].form) && (lo..=hi).any(|i| pair(&s.toks[i].form, &s.toks[hi + 1].form)) {
        hi += 1;
    }
    if lo > 0 && inside[lo - 1] && opening(&s.toks[lo - 1].form) && (lo..=hi).any(|i| pair(&s.toks[lo - 1].form, &s.toks[i].form)) {
        lo -= 1;
    }
    (lo, hi)
}

fn content(s: &Sent, i: usize) -> bool {
    matches!(s.upos(i), Some(UPos::NOUN | UPos::PROPN | UPos::ADJ | UPos::NUM | UPos::VERB)) && title(&s.low(i)).is_none()
}

/// The path from the word to the head does not pass through a subordinate clause, appos or conj.
fn local_to(s: &Sent, mut i: usize, h: usize) -> bool {
    let mut guard = 0;
    while i != h {
        if matches!(s.rel(i).base(), Rel::Acl | Rel::Advcl | Rel::Appos | Rel::Conj | Rel::Parataxis | Rel::Ccomp) {
            return false;
        }
        match s.parent(i) {
            Some(p) => i = p,
            None => return false,
        }
        guard += 1;
        if guard > s.len() {
            return false;
        }
    }
    true
}

fn number_of(s: &Sent, h: usize) -> u8 {
    let t = &s.toks[h];
    if t.feats.has(Feat::NumberPlur) || t.feats.has(Feat::NumberPtan) {
        PL
    } else if t.feats.has(Feat::NumberSing) {
        SG
    } else {
        match t.tag {
            Some(Tag::NNS | Tag::NNPS) => PL,
            Some(Tag::NN | Tag::NNP) => SG,
            _ => 0,
        }
    }
}

/// Words of the head's flat name: the head itself and its flat descendants.
fn name_of(s: &Sent, h: usize) -> Vec<usize> {
    let mut out = vec![h];
    let mut i = 0;
    while i < out.len() {
        let x = out[i];
        for &k in &s.kids[x] {
            if s.rel(k).base() == Rel::Flat {
                out.push(k);
            }
        }
        i += 1;
    }
    out.sort_unstable();
    out
}

fn build(s: &Sent, si: usize, h: usize, kind: Kind, (a, b): (usize, usize), coord: bool) -> Mention {
    let low = s.low(h);
    let name = if kind == Kind::Proper { name_of(s, h) } else { vec![h] };
    let head_key = s.low(*name.last().unwrap());
    let mut m = Mention {
        span: Span { sent: si, start: a, end: b },
        head: h,
        kind,
        pron: None,
        number: 0,
        gender: 0,
        animacy: 0,
        person: P3,
        coord,
        indefinite: false,
        quantified: false,
        post_clausal_only: true,
        internal: false,
        text: s.text(a, b).to_lowercase(),
        pre_head: s.text(a, (*name.last().unwrap()).max(h).min(b)).to_lowercase(),
        head_low: low.clone(),
        head_key,
        words: (a..=b).filter(|&i| content(s, i)).map(|i| s.low(i)).collect(),
        mods: (a..=b).filter(|&i| !name.contains(&i) && matches!(s.upos(i), Some(UPos::NOUN | UPos::PROPN | UPos::ADJ)) && local_to(s, i, h)).map(|i| s.low(i)).collect(),
        proper_mods: (a..=b).filter(|&i| !name.contains(&i) && matches!(s.upos(i), Some(UPos::PROPN | UPos::NUM)) && local_to(s, i, h)).map(|i| s.low(i)).collect(),
        possessor: None,
        quote: s.quote[h],
        speaker: s.speaker.clone(),
        role: s.rel(h).base(),
        depth: s.depth[h],
    };
    if kind == Kind::Pronoun {
        let t = &s.toks[h];
        let info = pron_table(&low);
        let mut pt = info.as_ref().map_or(Pron::Other, |x| x.pt);
        // UD FEATS outweigh the table when present
        if t.feats.has(Feat::ReflexYes) {
            pt = Pron::Reflexive;
        } else if t.feats.has(Feat::PronTypeRel) {
            pt = Pron::Relative;
        } else if t.feats.has(Feat::PronTypeInt) || t.feats.has(Feat::PronTypeIntRel) {
            pt = if pt == Pron::Relative { Pron::Relative } else { Pron::Interrogative };
        } else if t.feats.has(Feat::PronTypeDem) {
            pt = Pron::Demonstrative;
        } else if t.feats.has(Feat::PronTypeInd) || t.feats.has(Feat::PronTypeTot) || t.feats.has(Feat::PronTypeNeg) {
            pt = Pron::Indefinite;
        } else if t.feats.has(Feat::PronTypeRcp) {
            pt = Pron::Reciprocal;
        } else if t.feats.has(Feat::PossYes) || s.rel(h) == Rel::NmodPoss {
            if matches!(pt, Pron::Personal | Pron::Possessive | Pron::Other) {
                pt = Pron::Possessive;
            }
        }
        // relative "that" in UD has PronType=Rel; "that" without Rel as subject of a subordinate clause is also relative
        m.pron = Some(pt);
        if let Some(x) = &info {
            m.person = x.person;
            m.number = x.number;
            m.gender = x.gender;
            m.animacy = x.animacy;
        }
        if t.feats.has(Feat::Person1) {
            m.person = P1;
        } else if t.feats.has(Feat::Person2) {
            m.person = P2;
        } else if t.feats.has(Feat::Person3) {
            m.person = P3;
        }
        if m.number == 0 {
            m.number = number_of(s, h);
        }
        if pt == Pron::Reciprocal {
            m.number = PL;
        }
        m.internal = matches!(pt, Pron::Relative | Pron::Interrogative);
        m.indefinite = pt == Pron::Indefinite;
        m.quantified = pt == Pron::Indefinite && !matches!(low.as_str(), "someone" | "somebody" | "something" | "one");
    } else {
        m.number = if coord { PL } else { number_of(s, h) };
        if !coord && let Some(g) = gender_noun(&s.lem(h)) {
            m.gender = g;
            m.animacy = ANIM;
        }
        // a title in the span: gender (if stated) and animacy
        for i in a..=b {
            if i < h
                && let Some(g) = title(&s.low(i))
            {
                m.gender |= g;
                m.animacy = ANIM;
            }
        }
        // subordinate clause with who / which at the head: animate or not — the text says it itself
        for &k in &s.kids[h] {
            if s.rel(k).base() == Rel::Acl {
                let (x, y) = s.sub[k];
                for i in x..=y {
                    if s.upos(i) == Some(UPos::PRON) && s.dominates(k, i) && s.parent(i).is_some() {
                        match s.low(i).as_str() {
                            "who" | "whom" | "whose" => m.animacy |= ANIM,
                            "which" => m.animacy |= INAN,
                            _ => {}
                        }
                    }
                }
            }
        }
        let first = s.low(a);
        let has_det = s.kids[h].iter().any(|&k| s.rel(k).base() == Rel::Det || s.rel(k) == Rel::NmodPoss);
        // quantifier determiners (some, any, no, another, each, every…) and numbers without a definite article — also a new entity
        const QUANT: &[&str] = &["some", "any", "no", "another", "each", "every", "most", "many", "several", "few", "other", "such", "all", "both", "either", "neither", "much", "more", "less", "enough", "what", "which", "whatever", "whichever"];
        const UNIV: &[&str] = &["every", "each", "no", "any", "either", "neither"];
        m.quantified = s.kids[h].iter().any(|&k| s.rel(k).base() == Rel::Det && UNIV.contains(&s.low(k).as_str()));
        m.post_clausal_only = s.kids[h].iter().all(|&k| k < h || matches!(s.rel(k).base(), Rel::Acl | Rel::Punct) || !(a..=b).contains(&k) || name.contains(&k));
        let quant = s.kids[h].iter().any(|&k| (s.rel(k).base() == Rel::Det || s.rel(k).base() == Rel::Amod) && QUANT.contains(&s.low(k).as_str()));
        let definite = s.kids[h].iter().any(|&k| s.rel(k).base() == Rel::Det && matches!(s.low(k).as_str(), "the" | "this" | "that" | "these" | "those") || s.rel(k) == Rel::NmodPoss);
        let numeral = s.kids[h].iter().any(|&k| s.rel(k).base() == Rel::Nummod);
        m.indefinite = first == "a" || first == "an" || quant || numeral && !definite || (kind == Kind::Nominal && m.number == PL && !has_det && !coord);
        // possessor (nmod:poss) — to check "my jeans" ≠ "your jeans"
        m.possessor = s.kids[h].iter().copied().find(|&k| s.rel(k) == Rel::NmodPoss);
    }
    m
}

/// All mentions of the document in text order: (sentence, start, longer first, head).
pub fn detect(doc: &Document) -> Vec<Mention> {
    let mut out: Vec<Mention> = Vec::new();
    for (si, s) in doc.sents.iter().enumerate() {
        for h in 0..s.len() {
            let Some(kind) = is_head(s, h) else { continue };
            let quant_head = kind == Kind::Other && matches!(s.low(h).as_str(), "either" | "neither" | "none" | "each" | "all" | "any" | "every");
            let has_conj = s.kids[h].iter().any(|&k| s.rel(k).base() == Rel::Conj && is_head(s, k).is_some());
            let own = span_of(s, h, &|k| has_conj && matches!(s.rel(k).base(), Rel::Conj | Rel::Cc));
            if has_conj {
                let all = span_of(s, h, &|_| false);
                if all != own {
                    out.push(build(s, si, h, kind, all, true));
                }
            }
            let mut m = build(s, si, h, kind, own, false);
            if quant_head {
                m.quantified = true;
                m.indefinite = true;
            }
            out.push(m);
        }
    }
    out.sort_by_key(|m| (m.span.sent, m.span.start, std::cmp::Reverse(m.span.end), m.head));
    out.dedup_by_key(|m| m.span);
    out
}
