//! English morphology: lemmatization (form, PTB tag → lemma) and inflection (lemma, tag → form)
//! on the built-in dictionary (`dict`: full AGID form expansion + UD frequencies) — loads nothing
//! at run time. Priority: UD frequencies → AGID paradigms (Kevin Atkinson, permissive license) →
//! suffix rules. Lossless token encoding: lemma + case; if inflection does not reproduce the form —
//! escape (`Case::Raw`): the form is stored as is (as in an ordinary compressor).

use crate::dict::{self, Rec};
use crate::gram::{Case, Tag};

/// Paradigm cell.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Slot {
    Base,
    Past,
    PastPart,
    PresPart,
    ThirdSg,
    Plural,
    Comp,
    Super,
}

impl Slot {
    /// Paradigm cell from a PTB tag.
    pub fn of(t: Tag) -> Slot {
        match t {
            Tag::VBD => Slot::Past,
            Tag::VBN => Slot::PastPart,
            Tag::VBG => Slot::PresPart,
            Tag::VBZ => Slot::ThirdSg,
            Tag::NNS | Tag::NNPS => Slot::Plural,
            Tag::JJR | Tag::RBR => Slot::Comp,
            Tag::JJS | Tag::RBS => Slot::Super,
            _ => Slot::Base,
        }
    }

    /// Tag with which the cell is marked in the dictionary.
    pub fn tag(self) -> Tag {
        match self {
            Slot::Base => Tag::NN,
            Slot::Past => Tag::VBD,
            Slot::PastPart => Tag::VBN,
            Slot::PresPart => Tag::VBG,
            Slot::ThirdSg => Tag::VBZ,
            Slot::Plural => Tag::NNS,
            Slot::Comp => Tag::JJR,
            Slot::Super => Tag::JJS,
        }
    }
}

/// Stateless morphology: everything is in the built-in dictionary.
#[derive(Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Morph;

/// The most frequent UD entry for a tag (on ties — the later one by first occurrence).
fn ud_best(recs: impl Iterator<Item = Rec>, tag: Tag) -> Option<Rec> {
    recs.filter(|r| r.src & dict::UD != 0 && r.tag == tag).max_by_key(|r| r.count)
}

/// Lemma known to morphology (AGID without "?" or UD).
fn known(s: &str) -> bool {
    dict::lemma(s).is_some_and(dict::known)
}

impl Morph {
    /// Lemma of a form with a tag.
    ///
    /// For proper nouns, numbers with digits, addresses, symbols and foreign words the case of the form is kept
    /// if the lemma differs from the form only in case. This is how UD EWT does it:
    /// - "al" in "et al" stays "al", not "Al" from the dictionary;
    /// - "01-Feb-02" does not become "01-feb-02";
    /// - "One" at the start of a sentence — "one".
    ///
    /// A typographic apostrophe counts as a typewriter one: "’d" → would, "n’t" → not.
    pub fn lemmatize(&self, form: &str, tag: Tag) -> String {
        let typed;
        let form = if form.contains('\u{2019}') {
            typed = form.replace('\u{2019}', "'");
            typed.as_str()
        } else {
            form
        };
        let l = self.lemmatize_any_case(form, tag);
        // Form is all lowercase — case is taken from the dictionary: "dallas" → Dallas. This is how ATIS is annotated, with
        // queries without capitals; EWT loses 23 "et al" tokens on this (lemma "al", dictionary — Al).
        let keep = matches!(tag, Tag::NNP | Tag::ADD | Tag::SYM | Tag::LS | Tag::FW) || (tag == Tag::CD && form.chars().any(|c| c.is_ascii_digit()));
        if keep && form.chars().any(char::is_uppercase) && l.to_lowercase() == form.to_lowercase() {
            return form.to_string();
        }
        l
    }

    fn lemmatize_any_case(&self, form: &str, tag: Tag) -> String {
        let lower = form.to_lowercase();
        let f = dict::form(&lower);
        if let Some(r) = f.and_then(|f| ud_best(dict::analyses(f), tag)) {
            return dict::text(r.lemma()).to_string();
        }
        let sl = Slot::of(tag);
        if sl == Slot::Base {
            return if tag.is_proper() { form.to_string() } else { lower };
        }
        if let Some(f) = f {
            let rev: Vec<Rec> = dict::analyses(f).filter(|r| r.src & dict::REV != 0).collect();
            if let Some(r) = rev.iter().find(|r| Slot::of(r.tag) == sl).or_else(|| rev.first()) {
                return dict::text(r.lemma()).to_string();
            }
        }
        // candidates from rules: a known lemma from which inflection reproduces exactly this form > a known lemma > the first
        let cands = unsuffix(&lower, sl);
        if let Some(c) = cands.iter().find(|c| known(c) && self.inflect(c, tag) == lower) {
            return c.clone();
        }
        if let Some(c) = cands.iter().find(|c| self.inflect(c, tag) == lower) {
            return c.clone();
        }
        if let Some(c) = cands.iter().find(|c| known(c)) {
            return c.clone();
        }
        cands.into_iter().next().unwrap_or(lower)
    }

    /// Form from lemma and tag (lowercase, except proper nouns).
    pub fn inflect(&self, lemma: &str, tag: Tag) -> String {
        let key = lemma.to_lowercase();
        let l = dict::lemma(&key);
        if let Some(r) = l.and_then(|l| ud_best(dict::paradigm(l), tag)) {
            return dict::form_text(r.form()).to_string();
        }
        let sl = Slot::of(tag);
        if sl == Slot::Base {
            return key;
        }
        if let Some(r) = l.and_then(|l| dict::paradigm(l).find(|r| r.src & dict::AGID != 0 && r.tag == sl.tag())) {
            return dict::form_text(r.form()).to_string();
        }
        suffix(&key, sl)
    }

    pub fn encode(&self, form: &str, tag: Tag) -> (String, Case) {
        let lemma = self.lemmatize(form, tag);
        let made = self.inflect(&lemma, tag);
        let case = if made == form {
            Case::AsIs
        } else if title(&made) == form {
            Case::Title
        } else if made.to_uppercase() == form && form.chars().count() > 1 {
            Case::Upper
        } else {
            return (form.to_string(), Case::Raw);
        };
        (lemma, case)
    }

    pub fn decode(&self, lemma: &str, tag: Tag, case: Case) -> String {
        if case == Case::Raw {
            return lemma.to_string();
        }
        let f = self.inflect(lemma, tag);
        match case {
            Case::Title => title(&f),
            Case::Upper => f.to_uppercase(),
            _ => f,
        }
    }
}

fn title(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

/// Regular inflection (when neither UD nor AGID knows the word).
fn suffix(l: &str, sl: Slot) -> String {
    let ch: Vec<char> = l.chars().collect();
    let n = ch.len();
    let ends = |s: &str| l.ends_with(s);
    let cons_y = n >= 2 && ch[n - 1] == 'y' && !vowel(ch[n - 2]);
    match sl {
        Slot::Base => l.to_string(),
        Slot::Plural | Slot::ThirdSg => {
            if cons_y {
                format!("{}ies", &l[..l.len() - 1])
            } else if ends("s") || ends("x") || ends("z") || ends("ch") || ends("sh") {
                format!("{l}es")
            } else {
                format!("{l}s")
            }
        }
        Slot::Past | Slot::PastPart => {
            if ends("e") {
                format!("{l}d")
            } else if cons_y {
                format!("{}ied", &l[..l.len() - 1])
            } else {
                format!("{l}ed")
            }
        }
        Slot::PresPart => {
            if ends("ie") {
                format!("{}ying", &l[..l.len() - 2])
            } else if ends("e") && !ends("ee") && n > 2 {
                format!("{}ing", &l[..l.len() - 1])
            } else {
                format!("{l}ing")
            }
        }
        Slot::Comp | Slot::Super => {
            let s = if sl == Slot::Comp { "er" } else { "est" };
            if cons_y {
                format!("{}i{s}", &l[..l.len() - 1])
            } else if ends("e") {
                format!("{l}{}", &s[1..])
            } else {
                format!("{l}{s}")
            }
        }
    }
}

/// Candidate lemmas for a regular form (reverse rules), most probable first.
fn unsuffix(f: &str, sl: Slot) -> Vec<String> {
    let strip = |suf: &str| f.strip_suffix(suf).map(str::to_string);
    let mut v: Vec<String> = Vec::new();
    let mut push = |x: Option<String>| {
        if let Some(x) = x.filter(|x| !x.is_empty()) {
            v.push(x);
        }
    };
    match sl {
        Slot::Plural | Slot::ThirdSg => {
            push(strip("ies").map(|x| x + "y"));
            push(strip("es"));
            push(strip("s"));
        }
        Slot::Past | Slot::PastPart => {
            push(strip("ied").map(|x| x + "y"));
            push(strip("ed"));
            push(strip("d"));
            if let Some(x) = strip("ed") {
                let c: Vec<char> = x.chars().collect();
                if c.len() >= 2 && c[c.len() - 1] == c[c.len() - 2] {
                    push(Some(c[..c.len() - 1].iter().collect()));
                }
            }
        }
        Slot::PresPart => {
            push(strip("ying").map(|x| x + "ie"));
            push(strip("ing"));
            push(strip("ing").map(|x| x + "e"));
            if let Some(x) = strip("ing") {
                let c: Vec<char> = x.chars().collect();
                if c.len() >= 2 && c[c.len() - 1] == c[c.len() - 2] {
                    push(Some(c[..c.len() - 1].iter().collect()));
                }
            }
        }
        Slot::Comp | Slot::Super => {
            let s = if sl == Slot::Comp { "er" } else { "est" };
            push(strip(&format!("i{s}")).map(|x| x + "y"));
            push(strip(s));
            push(strip(&s[1..]));
        }
        Slot::Base => push(Some(f.to_string())),
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_rules() {
        assert_eq!(suffix("city", Slot::Plural), "cities");
        assert_eq!(suffix("box", Slot::Plural), "boxes");
        assert_eq!(suffix("make", Slot::PresPart), "making");
        assert_eq!(suffix("carry", Slot::Past), "carried");
        assert_eq!(suffix("large", Slot::Comp), "larger");
        assert!(unsuffix("carried", Slot::Past).contains(&"carry".to_string()));
        assert!(unsuffix("stopped", Slot::Past).contains(&"stop".to_string()));
    }

    #[test]
    fn dictionary() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        let m = Morph;
        for (f, t, l) in [("paced", Tag::VBN, "pace"), ("raises", Tag::VBZ, "raise"), ("went", Tag::VBD, "go"), ("mice", Tag::NNS, "mouse"), ("better", Tag::JJR, "good")] {
            assert_eq!(m.lemmatize(f, t), l, "{f}/{t}");
            assert_eq!(m.inflect(l, t), f, "{l}/{t}");
        }
        assert_eq!(m.encode("Went", Tag::VBD), ("go".to_string(), Case::Title));
    }
}
