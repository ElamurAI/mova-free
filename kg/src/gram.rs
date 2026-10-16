//! Compact word-form grammar in 4 bytes (bit fields) and parsing of VESUM tags.
//! The layout is arbitrary (design note: density and speed come first): part of speech 5 bits, case 3,
//! number 2, gender 2, person 2, tense/form 3, aspect 2, animacy 2, degree 2, flags pronoun,
//! proper name, negation, implicit sentence member; 5 bits spare.

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, PartialOrd, Ord)]
pub struct Gram(pub u32);

type Field = (u32, u32); // (shift, width)
const POS: Field = (0, 5);
const CASE: Field = (5, 3);
const NUM: Field = (8, 2);
const GEN: Field = (10, 2);
const PERS: Field = (12, 2);
const TENSE: Field = (14, 3);
const ASPECT: Field = (17, 2);
const ANIM: Field = (19, 2);
const DEGREE: Field = (21, 2);
pub const PRON: u32 = 23;
pub const PROP: u32 = 24;
pub const NEG: u32 = 25;
pub const IMPLICIT: u32 = 26;

pub const POS_NAMES: [&str; 14] =
    ["", "noun", "verb", "adj", "adv", "prep", "conj", "part", "intj", "numr", "noninfl", "advp", "onomat", "other"];
pub const NOUN: u32 = 1;
pub const VERB: u32 = 2;
pub const ADJ: u32 = 3;
pub const ADV: u32 = 4;
pub const PREP: u32 = 5;
pub const PART: u32 = 7;

pub const CASE_NAMES: [&str; 8] = ["", "v_naz", "v_rod", "v_dav", "v_zna", "v_oru", "v_mis", "v_kly"];
pub const NAZ: u32 = 1;
pub const ROD: u32 = 2;
pub const DAV: u32 = 3;
pub const ZNA: u32 = 4;
pub const ORU: u32 = 5;
pub const MIS: u32 = 6;

pub const SG: u32 = 1;
pub const PL: u32 = 2;
const NUM_NAMES: [&str; 3] = ["", "s", "p"];
const GEN_NAMES: [&str; 4] = ["", "m", "f", "n"];
pub const TENSE_NAMES: [&str; 7] = ["", "inf", "pres", "futr", "past", "impr", "impers"];
pub const PRES: u32 = 2;
pub const FUTR: u32 = 3;
pub const PAST: u32 = 4;
pub const IMPR: u32 = 5;
const ASPECT_NAMES: [&str; 3] = ["", "imperf", "perf"];
const ANIM_NAMES: [&str; 4] = ["", "anim", "inanim", "unanim"];
const DEGREE_NAMES: [&str; 4] = ["", "compb", "compr", "super"];

impl Gram {
    fn get(self, (s, w): Field) -> u32 {
        (self.0 >> s) & ((1 << w) - 1)
    }
    fn set(&mut self, (s, w): Field, v: u32) {
        let m = ((1 << w) - 1) << s;
        self.0 = (self.0 & !m) | ((v << s) & m);
    }
    pub fn flag(self, bit: u32) -> bool {
        self.0 >> bit & 1 == 1
    }
    pub fn with(mut self, bit: u32) -> Self {
        self.0 |= 1 << bit;
        self
    }
    pub fn pos(self) -> u32 {
        self.get(POS)
    }
    pub fn case(self) -> u32 {
        self.get(CASE)
    }
    pub fn num(self) -> u32 {
        self.get(NUM)
    }
    pub fn gender(self) -> u32 {
        self.get(GEN)
    }
    pub fn person(self) -> u32 {
        self.get(PERS)
    }
    pub fn tense(self) -> u32 {
        self.get(TENSE)
    }
    pub fn anim(self) -> u32 {
        self.get(ANIM)
    }
    /// Grammar of the implicit subject from the predicate's grammar: person, number, gender.
    pub fn implicit_subject(pred: Gram) -> Gram {
        let mut g = Gram(0);
        g.set(POS, NOUN);
        g.set(CASE, NAZ);
        g.set(NUM, pred.num());
        g.set(GEN, pred.gender());
        g.set(PERS, pred.person());
        g.with(PRON).with(IMPLICIT)
    }

    /// VESUM tags (`noun:anim:m:v_naz`, `verb:imperf:pres:s:3`, …) → Gram; unknown tags are skipped.
    pub fn from_vesum(tags: &str) -> Gram {
        let mut g = Gram(0);
        let mut it = tags.split(':');
        let pos = it.next().unwrap_or("");
        g.set(POS, POS_NAMES.iter().position(|n| *n == pos).unwrap_or(13) as u32);
        for t in it {
            if let Some(c) = CASE_NAMES.iter().skip(1).position(|n| *n == t) {
                g.set(CASE, c as u32 + 1);
                continue;
            }
            match t {
                "s" => g.set(NUM, SG),
                "p" => g.set(NUM, PL),
                "m" | "f" | "n" => {
                    g.set(GEN, GEN_NAMES.iter().position(|n| *n == t).unwrap_or(0) as u32);
                    if g.num() == 0 {
                        g.set(NUM, SG);
                    }
                }
                "1" => g.set(PERS, 1),
                "2" => g.set(PERS, 2),
                "3" => g.set(PERS, 3),
                "inf" => g.set(TENSE, 1),
                "pres" => g.set(TENSE, PRES),
                "futr" => g.set(TENSE, FUTR),
                "past" => g.set(TENSE, PAST),
                "impr" => g.set(TENSE, IMPR),
                "impers" => g.set(TENSE, 6),
                "imperf" => g.set(ASPECT, 1),
                "perf" => g.set(ASPECT, 2),
                "anim" | "ranim" => g.set(ANIM, 1),
                "inanim" | "rinanim" => g.set(ANIM, 2),
                "unanim" => g.set(ANIM, 3),
                "compb" => g.set(DEGREE, 1),
                "compr" => g.set(DEGREE, 2),
                "super" => g.set(DEGREE, 3),
                "pron" => g.0 |= 1 << PRON,
                "prop" => g.0 |= 1 << PROP,
                _ => {}
            }
        }
        g
    }
}

impl fmt::Display for Gram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts: Vec<&str> = vec![POS_NAMES[self.pos() as usize]];
        for (v, names) in [
            (self.case(), &CASE_NAMES[..]),
            (self.num(), &NUM_NAMES[..]),
            (self.gender(), &GEN_NAMES[..]),
            (self.tense(), &TENSE_NAMES[..]),
            (self.get(ASPECT), &ASPECT_NAMES[..]),
            (self.anim(), &ANIM_NAMES[..]),
            (self.get(DEGREE), &DEGREE_NAMES[..]),
        ] {
            if v > 0 {
                parts.push(names[v as usize]);
            }
        }
        let pers = ["", "1", "2", "3"][self.person() as usize];
        if !pers.is_empty() {
            parts.push(pers);
        }
        for (bit, name) in [(PRON, "pron"), (PROP, "prop"), (NEG, "neg"), (IMPLICIT, "implicit")] {
            if self.flag(bit) {
                parts.push(name);
            }
        }
        write!(f, "{}", parts.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vesum_tags() {
        let g = Gram::from_vesum("noun:anim:m:v_zna");
        assert_eq!((g.pos(), g.case(), g.num(), g.gender(), g.anim()), (NOUN, ZNA, SG, 1, 1));
        let v = Gram::from_vesum("verb:imperf:pres:s:3");
        assert_eq!((v.pos(), v.tense(), v.num(), v.person()), (VERB, PRES, SG, 3));
        let p = Gram::from_vesum("noun:anim:p:v_naz:pron:pers:1");
        assert!(p.flag(PRON) && p.num() == PL && p.person() == 1);
        assert_eq!(Gram::from_vesum("verb:perf:past:f").to_string(), "verb.s.f.past.perf");
    }
}
