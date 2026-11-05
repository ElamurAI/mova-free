//! Hebrew text: levels of pointing, letter units, raw text into tokens.

use unicode_normalization::UnicodeNormalization;

/// How much of the pointing a form keeps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// letters and vowel points (dagesh, shin/sin dots); the accents (teʿamim), meteg and rafe are dropped
    Pointed,
    /// letters only
    Unpointed,
}

impl Level {
    pub fn parse(s: &str) -> Option<Level> {
        match s {
            "pointed" => Some(Level::Pointed),
            "unpointed" | "none" => Some(Level::Unpointed),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Level::Pointed => "pointed",
            Level::Unpointed => "unpointed",
        }
    }
}

pub const MAQAF: char = '\u{05BE}';
pub const PASEQ: char = '\u{05C0}';
pub const SOF_PASUQ: char = '\u{05C3}';

/// An accent, meteg, rafe, the extraordinary dots, or a joiner: never kept.
fn accent(c: char) -> bool {
    matches!(c, '\u{0591}'..='\u{05AF}' | '\u{05BD}' | '\u{05BF}' | '\u{05C4}' | '\u{05C5}' | '\u{200C}' | '\u{200D}' | '\u{034F}')
}

/// A vowel point, dagesh or mapiq, shin or sin dot, qamats qatan.
fn point(c: char) -> bool {
    matches!(c, '\u{05B0}'..='\u{05BC}' | '\u{05C1}' | '\u{05C2}' | '\u{05C7}')
}

/// A mark that sits on the letter before it (not maqaf, paseq or sof pasuq, which are signs of their own).
pub fn combining(c: char) -> bool {
    accent(c) || point(c)
}

/// Does the text carry vowel points?
pub fn is_pointed(s: &str) -> bool {
    s.chars().any(point)
}

pub fn norm(s: &str, level: Level) -> String {
    let kept: String = s.chars().filter(|&c| !accent(c) && !(level == Level::Unpointed && point(c))).collect();
    kept.nfc().collect()
}

/// A partly pointed form: each vowel point, dagesh or dot dropped with the given share, by a hash of the form, the
/// seed and the mark's place (the same input always thins the same way). Accents go as in `norm`.
pub fn thin(s: &str, share: f64, seed: u64) -> String {
    let mut h: u64 = 0xcbf29ce484222325 ^ seed.wrapping_mul(0x100000001b3);
    for b in s.bytes() {
        h = (h ^ b as u64).wrapping_mul(0x100000001b3);
    }
    let kept: String = s.chars().enumerate().filter(|&(i, c)| {
        if accent(c) {
            return false;
        }
        if !point(c) {
            return true;
        }
        let mut x = h ^ (i as u64).wrapping_mul(0x9e3779b97f4a7c15);
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51afd7ed558ccd);
        x ^= x >> 33;
        ((x >> 11) as f64 / (1u64 << 53) as f64) >= share
    }).map(|(_, c)| c).collect();
    kept.nfc().collect()
}

/// Letter units: a letter (or any other sign) with the marks that sit on it. Prefixes and suffixes are cut between
/// units.
pub fn units(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in s.char_indices() {
        if combining(c) && start.is_some() {
            continue;
        }
        if let Some(st) = start {
            out.push(&s[st..i]);
        }
        start = Some(i);
    }
    if let Some(st) = start {
        out.push(&s[st..]);
    }
    out
}

/// A paragraph mark (petuḥah פ, setumah ס) or a sign of punctuation.
pub fn is_mark(tok: &str) -> bool {
    matches!(tok, "פ" | "ס" | "׃" | "׀" | ":" | "." | "," | ";" | "?" | "!" | "|" | "(" | ")" | "[" | "]" | "\"" | "'" | "״" | "׳")
}

/// One raw line into sentences of tokens. Tokens are cut at spaces and after a maqaf (which stays on the word before
/// it, as in MACULA); sof pasuq, paseq and ASCII punctuation are tokens of their own; a sentence ends after sof
/// pasuq (with the paragraph marks after it), «.», «?» or «!».
pub fn tokenize(line: &str) -> Vec<Vec<String>> {
    let line: String = line.nfc().collect();
    let mut toks: Vec<String> = Vec::new();
    for chunk in line.split_whitespace() {
        let mut cur = String::new();
        for c in chunk.chars() {
            match c {
                MAQAF => {
                    cur.push(c);
                    toks.push(std::mem::take(&mut cur));
                }
                SOF_PASUQ | PASEQ | ':' | '.' | ',' | ';' | '?' | '!' | '|' | '(' | ')' | '[' | ']' | '"' | '״' => {
                    if !cur.is_empty() {
                        toks.push(std::mem::take(&mut cur));
                    }
                    toks.push(c.to_string());
                }
                _ => cur.push(c),
            }
        }
        if !cur.is_empty() {
            toks.push(cur);
        }
    }
    // a paragraph mark written close after sof pasuq («׃פ») is cut above; then sentences
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut cur: Vec<String> = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        let t = toks[i].clone();
        let ends = matches!(t.as_str(), "׃" | ":" | "." | "?" | "!");
        cur.push(t);
        if ends {
            while i + 1 < toks.len() && matches!(toks[i + 1].as_str(), "פ" | "ס") {
                i += 1;
                cur.push(toks[i].clone());
            }
            out.push(std::mem::take(&mut cur));
        }
        i += 1;
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels() {
        let w = "בָּאָ֑רֶץ";
        assert_eq!(norm(w, Level::Pointed), "בָּאָרֶץ");
        assert_eq!(norm(w, Level::Unpointed), "בארץ");
        assert!(is_pointed(w) && !is_pointed("בארץ"));
        // the units do not change with the level: the cuts carry over
        assert_eq!(units(w).len(), units("בארץ").len());
        assert_eq!(units("וּבְ"), ["וּ", "בְ"]);
    }

    #[test]
    fn tokens_and_sentences() {
        let s = tokenize("וַיְהִי־כֵן׃ פ וַיַּרְא אֱלֹהִים׃פ");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], ["וַיְהִי־", "כֵן", "׃", "פ"]);
        assert_eq!(s[1], ["וַיַּרְא", "אֱלֹהִים", "׃", "פ"]);
        // negative control: a פ that is a word's letter stays in the word
        assert_eq!(tokenize("פֶּה")[0], ["פֶּה"]);
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn coordinates_thinning_and_sentence_edges() {
        let raw = "בָּ֑א";
        assert_eq!(units(raw), ["בָּ֑", "א"]);
        assert_eq!(units(raw).concat(), raw);
        assert!(units("").is_empty());
        assert_eq!(norm(raw, Level::Unpointed), "בא");
        assert_eq!(thin(raw, 0.0, 9), norm(raw, Level::Pointed));
        assert_eq!(thin(raw, 1.0, 9), "בא");
        assert_eq!(thin(raw, f64::NAN, 9), "בא");
        assert_eq!(
            tokenize("אב: ס גד־ה!"),
            [vec!["אב", ":", "ס"], vec!["גד־", "ה", "!"]]
        );
        assert!(tokenize(" \t").is_empty());
        assert_eq!(Level::parse("none"), Some(Level::Unpointed));
        assert_eq!(Level::parse("POINTED"), None);
    }
}
