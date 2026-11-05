//! English morphology: lemmatization (form, PTB tag → lemma) and inflection (lemma, tag → form)
//! on the built-in dictionary (`dict`: full AGID form expansion + UD frequencies) — loads nothing
//! at run time. Priority: UD frequencies → AGID paradigms (Kevin Atkinson, permissive license) →
//! suffix rules. Lossless token encoding: lemma + case; if inflection does not reproduce the form —
//! escape (`Case::Raw`): the form is stored as is (as in an ordinary compressor).

use crate::dict::{self, Rec};
use crate::gram::{Case, Feat, Feats, Rel, Tag};

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

/// Early Modern English verb forms, as in the King James and JPS 1917 Bibles (17.10: the Torah's drafts lost a third of
/// their sentences to «AUX with lemma 'shalt'»): only for a verb or auxiliary tag, so «art» the noun stays art.
/// The second-person «-est»/«-st» and third-person «-eth» endings go to the plain verb when it is known (`ending_verb`).
pub fn archaic(lower: &str, tag: Tag) -> Option<String> {
    // forms that are never anything but a verb: whatever the tagger said (the small model tags «shalt» a noun, and a
    // later parser that makes it AUX keeps the lemma it was given)
    const ONLY_VERBS: [(&str, &str); 23] = [
        ("shalt", "shall"), ("hath", "have"), ("hast", "have"), ("hadst", "have"), ("doth", "do"), ("dost", "do"), ("didst", "do"),
        ("wast", "be"), ("wert", "be"), ("canst", "can"), ("couldst", "could"), ("couldest", "could"), ("shouldst", "should"),
        ("shouldest", "should"), ("wouldst", "would"), ("wouldest", "would"), ("mayest", "may"), ("mayst", "may"),
        ("mightest", "might"), ("saith", "say"), ("sayest", "say"), ("spake", "speak"), ("badest", "bid"),
    ];
    if let Some((_, l)) = ONLY_VERBS.iter().find(|(f, _)| *f == lower) {
        return Some(l.to_string());
    }
    let verb = matches!(tag, Tag::VB | Tag::VBD | Tag::VBG | Tag::VBN | Tag::VBP | Tag::VBZ | Tag::MD);
    if !verb {
        return None;
    }
    const IRREGULAR: [(&str, &str); 6] = [("wilt", "will"), ("art", "be"), ("brake", "break"), ("gavest", "give"), ("camest", "come"), ("wentest", "go")];
    if let Some((_, l)) = IRREGULAR.iter().find(|(f, _)| *f == lower) {
        return Some(l.to_string());
    }
    // a form the dictionary knows is no archaic ending: «digest», «interest», «harvest»
    if known_form(lower) {
        return None;
    }
    ending_verb(lower).map(|(l, _)| l)
}

/// A form Mova knows: one the dictionary really knows (`derive::in_dictionary`: any analysis except an AGID
/// superlative or comparative that is neither in UD nor clean for reverse lookup — AGID lists «goest», «doest»,
/// «seest», «liest», «shouldest» as superlatives of «go», «doe», «see», «lie», «should», the Bible's second-person
/// verbs, not adjectives), or a word derived from a known one by the seeds' productive affixes (`crate::derive`:
/// *unsupervised*, *tokenised*, *incrementally*).
pub fn known_form(lower: &str) -> bool {
    crate::derive::in_dictionary(lower) || crate::lexext::known(lower) || !crate::derive::analyses(lower).is_empty()
}

/// A known lemma with a verb paradigm (a third-person, past or participle form in UD or AGID).
fn verb_lemma(c: &str) -> Option<crate::gram::Sym> {
    dict::lemma(c).filter(|&l| dict::known(l) && dict::paradigm(l).any(|r| matches!(r.tag, Tag::VBZ | Tag::VBD | Tag::VBG | Tag::VBN)))
}

/// Spellings of a stem before «-eth»/«-est»: as is, with «e» («cometh» → come), undoubled («biddeth» → bid), «i» → «y»
/// («crieth» → cry). Of those that are verbs, the most frequent in UD wins (the lemma heap is ordered by UD frequency):
/// «cometh» → come, not «com»; «hateth» → hate, not «hat»; «putteth» → put, not «putt».
fn stem_verb(stem: &str) -> Option<String> {
    let mut cands = vec![stem.to_string(), format!("{stem}e")];
    let b = stem.as_bytes();
    if b.len() >= 3 && b[b.len() - 1] == b[b.len() - 2] {
        cands.push(stem[..stem.len() - 1].to_string());
    }
    if let Some(s) = stem.strip_suffix('i') {
        cands.push(format!("{s}y"));
    }
    // a stem ending consonant–vowel–consonant would have doubled its consonant before the ending («riddeth», «bitteth»),
    // so the «e» spelling comes first: «rideth» → ride, «biteth» → bite, «hateth» → hate, «cometh» → come
    let c: Vec<char> = stem.chars().collect();
    let n = c.len();
    let cons = |x: char| x.is_ascii_lowercase() && !vowel(x);
    if n >= 3 && cons(c[n - 1]) && !matches!(c[n - 1], 'w' | 'x' | 'y') && vowel(c[n - 2]) && cons(c[n - 3]) {
        if let Some(l) = verb_lemma(&cands[1]) {
            return Some(dict::text(l).to_string());
        }
    }
    // else the most frequent verb in UD, then the most frequent lemma
    let verb_n = |l: crate::gram::Sym| -> u32 { dict::paradigm(l).filter(|r| r.src & dict::UD != 0 && r.tag.name().starts_with("VB")).map(|r| r.count).sum() };
    cands.iter().filter_map(|c| verb_lemma(c)).max_by(|a, b| verb_n(*a).cmp(&verb_n(*b)).then(b.index().cmp(&a.index()))).map(|l| dict::text(l).to_string())
}

/// A past form under the second-person ending («saidst», «broughtest», «sawest»): the lemma of its VBD reading, when
/// that reading is at least as frequent in UD as the form's own present reading («sawest» → see; «layest» → lay).
fn past_verb(stem: &str) -> Option<String> {
    let f = dict::form(stem)?;
    let past = dict::analyses(f)
        .filter(|r| r.tag == Tag::VBD && r.src & (dict::UD | dict::REV) != 0 && dict::text(r.lemma()) != stem)
        .max_by(|a, b| a.count.cmp(&b.count).then(b.id.cmp(&a.id)))?;
    let present: u32 = dict::analyses(f).filter(|r| matches!(r.tag, Tag::VB | Tag::VBP) && dict::text(r.lemma()) == stem).map(|r| r.count).sum();
    let past_n: u32 = dict::analyses(f).filter(|r| r.tag == Tag::VBD && r.lemma() == past.lemma()).map(|r| r.count).sum();
    (past_n >= present).then(|| dict::text(past.lemma()).to_string())
}

/// The verb under an EModE ending, with the tag the ending gives: «-eth» → VBZ (3rd singular present); «-est» → VBP
/// (2nd singular present) or VBD after a past form; «-st» only after a past form («saidst», «defiledst»). The form is
/// not checked against the dictionary here (`archaic` and `crate::emode` do that).
pub fn ending_verb(lower: &str) -> Option<(String, Tag)> {
    if let Some(stem) = lower.strip_suffix("eth").filter(|s| s.len() >= 2) {
        return stem_verb(stem).map(|l| (l, Tag::VBZ));
    }
    if let Some(stem) = lower.strip_suffix("est").filter(|s| s.len() >= 2) {
        if let Some(l) = [stem.to_string(), format!("{stem}e")].iter().find_map(|c| past_verb(c)) {
            return Some((l, Tag::VBD));
        }
        if let Some(l) = stem_verb(stem) {
            return Some((l, Tag::VBP));
        }
    }
    if let Some(stem) = lower.strip_suffix("st").filter(|s| s.len() >= 3) {
        return past_verb(stem).map(|l| (l, Tag::VBD));
    }
    None
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
        // a plural proper noun keeps the capital of its form, as UD EWT: «Tuesdays» → Tuesday, «Americans» → American
        if tag == Tag::NNPS && form.chars().next().is_some_and(char::is_uppercase) && l.chars().next().is_some_and(char::is_lowercase) {
            let mut c = l.chars();
            return c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or(l);
        }
        l
    }

    fn lemmatize_any_case(&self, form: &str, tag: Tag) -> String {
        let lower = form.to_lowercase();
        if let Some(l) = archaic(&lower, tag) {
            return l;
        }
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
        // a form of the lexicon extension: its lemma in this cell (*vertices* NNS → vertex)
        if f.is_none()
            && let Some(l) = crate::lexext::lemma(&lower, tag)
        {
            return if tag.is_proper() { l.to_string() } else { l.to_lowercase() };
        }
        // candidates from rules: a known lemma from which inflection reproduces exactly this form > a known lemma > the first
        let cands = unsuffix(&lower, sl);
        if let Some(c) = cands.iter().find(|c| (known(c) || crate::lexext::known(c)) && self.inflect(c, tag) == lower) {
            return c.clone();
        }
        // a lemma derived from a known word (`crate::derive`): «unsupervised» VBN → unsupervise, «tokenizers» → tokenizer
        let cls = if matches!(sl, Slot::Plural) { crate::derive::N } else { crate::derive::V | crate::derive::A };
        if let Some(c) = cands.iter().find(|c| crate::derive::derived_as(c, cls) && self.inflect(c, tag) == lower) {
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
        // a lemma the dictionary does not inflect: the lexicon extension's form (vertex NNS → vertices is the
        // dictionary's vertexes; a new lemma's irregular form comes from here)
        if l.is_none_or(|l| dict::paradigm(l).next().is_none())
            && let Some(x) = crate::lexext::form(&key, sl.tag()).or_else(|| crate::lexext::form(&key, tag))
        {
            return x.to_string();
        }
        suffix(&key, sl)
    }

    /// Form from lemma, tag and FEATS: a personal pronoun in Case=Acc (me, us, him, her, them, thee), «be» by
    /// Person and Number (am; are and were for a plural or the second person), «this» and «that» with Number=Plur
    /// (these, those); everything else as `inflect`. The FEATS may come from the annotator or from the tree
    /// (`tree_feats`).
    pub fn inflect_feats(&self, lemma: &str, tag: Tag, feats: Feats) -> String {
        let key = lemma.to_lowercase();
        if tag == Tag::PRP && feats.has(Feat::CaseAcc) {
            if let Some((_, acc)) = PRON_ACC.iter().find(|(n, _)| *n == key) {
                return acc.to_string();
            }
        }
        if key == "be" {
            let plur = feats.has(Feat::NumberPlur) || feats.has(Feat::Person2);
            match tag {
                Tag::VBD if plur => return "were".into(),
                Tag::VBP if feats.has(Feat::Person1) && !plur => return "am".into(),
                Tag::VBP => return "are".into(),
                _ => {}
            }
        }
        if tag == Tag::DT && feats.has(Feat::NumberPlur) {
            match key.as_str() {
                "this" => return "these".into(),
                "that" => return "those".into(),
                _ => {}
            }
        }
        self.inflect(lemma, tag)
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

/// Personal pronouns (lemma = the nominative) and their accusative.
const PRON_ACC: [(&str, &str); 7] = [("i", "me"), ("we", "us"), ("he", "him"), ("she", "her"), ("they", "them"), ("thou", "thee"), ("ye", "you")];

/// The FEATS inflection needs, read from an unordered tree (lemma, tag, head 1-based, relation) by agreement, for a
/// generator that has no form to read them from: a personal pronoun is Case=Nom as a subject (or a conjunct of
/// one), Case=Acc elsewhere; «be» (verb, auxiliary or copula) takes Person and Number from its clause's subject
/// (I → 1 Sing, we → 1 Plur, you → 2, a plural noun or a coordination → Plur), the subject of an expletive «there»
/// clause included; «this» and «that» as determiners take the Number of their noun.
pub fn tree_feats(lemmas: &[&str], tags: &[Tag], heads: &[usize], rels: &[Rel]) -> Vec<Feats> {
    let n = lemmas.len();
    let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        if heads[i] >= 1 && heads[i] <= n {
            kids[heads[i] - 1].push(i);
        }
    }
    let set = |f: &mut Feats, x: Feat| f.0 |= 1 << x.idx();
    let subj = |i: usize| -> bool { rels[i].base() == Rel::Nsubj };
    let mut out = vec![Feats::default(); n];
    for i in 0..n {
        let l = lemmas[i].to_lowercase();
        if tags[i] == Tag::PRP && PRON_ACC.iter().any(|(p, _)| *p == l) {
            // a conjunct takes the case of its first conjunct
            let mut j = i;
            while rels[j] == Rel::Conj && heads[j] >= 1 && heads[j] <= n && heads[j] - 1 != j {
                j = heads[j] - 1;
            }
            set(&mut out[i], if subj(j) { Feat::CaseNom } else { Feat::CaseAcc });
        }
        if l == "be" && matches!(tags[i], Tag::VBD | Tag::VBP | Tag::VBZ) {
            // the clause: the verb itself, or the head of an auxiliary or copula
            let c = if matches!(rels[i].base(), Rel::Aux | Rel::Cop) && heads[i] >= 1 && heads[i] <= n { heads[i] - 1 } else { i };
            if let Some(&s) = kids[c].iter().find(|&&k| subj(k)) {
                let sl = lemmas[s].to_lowercase();
                let coord = kids[s].iter().any(|&k| rels[k] == Rel::Conj);
                match sl.as_str() {
                    "i" if tags[s] == Tag::PRP => {
                        set(&mut out[i], Feat::Person1);
                        set(&mut out[i], Feat::NumberSing);
                    }
                    "we" if tags[s] == Tag::PRP => {
                        set(&mut out[i], Feat::Person1);
                        set(&mut out[i], Feat::NumberPlur);
                    }
                    "you" | "ye" | "thou" if tags[s] == Tag::PRP => set(&mut out[i], Feat::Person2),
                    "they" if tags[s] == Tag::PRP => set(&mut out[i], Feat::NumberPlur),
                    _ if coord || matches!(tags[s], Tag::NNS | Tag::NNPS) => set(&mut out[i], Feat::NumberPlur),
                    _ => set(&mut out[i], Feat::NumberSing),
                }
            }
        }
        if tags[i] == Tag::DT && matches!(l.as_str(), "this" | "that") && rels[i] == Rel::Det && heads[i] >= 1 && heads[i] <= n && matches!(tags[heads[i] - 1], Tag::NNS | Tag::NNPS) {
            set(&mut out[i], Feat::NumberPlur);
        }
    }
    out
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
    fn archaic_verbs() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        let a = |f: &str, t: Tag| archaic(f, t);
        assert_eq!(a("shalt", Tag::MD).as_deref(), Some("shall"));
        assert_eq!(a("hath", Tag::VBZ).as_deref(), Some("have"));
        assert_eq!(a("art", Tag::VBP).as_deref(), Some("be"));
        assert_eq!(a("knoweth", Tag::VBZ).as_deref(), Some("know"));
        assert_eq!(a("maketh", Tag::VBZ).as_deref(), Some("make"));
        assert_eq!(a("crieth", Tag::VBZ).as_deref(), Some("cry"));
        assert_eq!(a("saith", Tag::VBZ).as_deref(), Some("say"));
        // negative control: a noun «art», a superlative, an ordinary verb form are left to the usual rules
        assert_eq!(a("art", Tag::NN), None);
        // a form that is only ever a verb, whatever the tag
        assert_eq!(a("shalt", Tag::NN).as_deref(), Some("shall"));
        assert_eq!(a("wilt", Tag::NN), None);
        assert_eq!(a("greatest", Tag::JJS), None);
        assert_eq!(a("rest", Tag::VB), None);
        assert_eq!(a("makes", Tag::VBZ), None);
        assert_eq!(a("digest", Tag::VB), None);
        assert_eq!(a("interest", Tag::VB), None);
    }

    /// L1–L3 of `languages/en-x-emode/13-improvements.md`, every form from the Torah (JPS 1917).
    #[test]
    fn archaic_endings() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        // L1: the verb among the spellings of the stem, the most frequent first — not «com», «hat», «li», «putt», «marri»
        for (f, l) in [
            ("cometh", "come"), ("seeth", "see"), ("hateth", "hate"), ("lieth", "lie"), ("dieth", "die"), ("putteth", "put"),
            ("marrieth", "marry"), ("biteth", "bite"), ("rideth", "ride"), ("writeth", "write"), ("giveth", "give"),
        ] {
            assert_eq!(archaic(f, Tag::VBZ).as_deref(), Some(l), "{f}");
        }
        // L2: AGID's «superlatives» of verbs are no dictionary forms
        for (f, l) in [("goest", "go"), ("doest", "do"), ("seest", "see"), ("shouldest", "should"), ("liest", "lie"), ("knowest", "know")] {
            assert_eq!(archaic(f, Tag::VBP).as_deref(), Some(l), "{f}");
        }
        // L3: the second-person past
        for (f, l) in [
            ("saidst", "say"), ("broughtest", "bring"), ("stoodest", "stand"), ("knewest", "know"), ("sentest", "send"), ("badest", "bid"),
            ("sawest", "see"), ("smotest", "smite"), ("drankest", "drink"), ("defiledst", "defile"), ("stretchedst", "stretch"),
        ] {
            assert_eq!(archaic(f, Tag::VBD).as_deref(), Some(l), "{f}");
        }
        assert_eq!(ending_verb("saidst"), Some(("say".to_string(), Tag::VBD)));
        assert_eq!(ending_verb("goest"), Some(("go".to_string(), Tag::VBP)));
        assert_eq!(ending_verb("goeth"), Some(("go".to_string(), Tag::VBZ)));
        // negative controls: modern words keep their usual analysis
        for f in ["digest", "interest", "harvest", "greatest", "rest", "best", "lest", "honest", "earnest", "priest", "forest", "longest", "fastest"] {
            assert!(known_form(f), "{f} is a dictionary form");
            assert_eq!(archaic(f, Tag::VB), None, "{f}");
        }
        for f in ["goest", "doest", "seest", "liest", "shouldest"] {
            assert!(!known_form(f), "{f}");
        }
        let m = Morph;
        assert_eq!(m.lemmatize("hat", Tag::NN), "hat");
        assert_eq!(m.lemmatize("putt", Tag::NN), "putt");
        assert_eq!(m.lemmatize("com", Tag::NN), "com");
        assert_eq!(m.lemmatize("greatest", Tag::JJS), "great");
        assert_eq!(m.lemmatize("Tuesdays", Tag::NNPS), "Tuesday");
        // negative control: a common noun stays lower case, and so does a plural proper noun written lower case
        assert_eq!(m.lemmatize("Days", Tag::NNS), "day");
        assert_eq!(m.lemmatize("tuesdays", Tag::NNPS), "tuesday");
        // no ending at all, or a stem that is no verb: nothing
        // «teeth» has a verb under it («tee»), but the dictionary knows the form
        assert!(known_form("teeth"));
        assert_eq!(archaic("teeth", Tag::VBZ), None);
        assert_eq!(ending_verb("japheth"), None);
        assert_eq!(ending_verb("first"), None);
        assert_eq!(ending_verb("must"), None);
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

#[cfg(test)]
mod feats_tests {
    use super::*;

    /// Inflection with the FEATS agreement reads from the tree; the negative control — lemma and tag alone — gives
    /// the nominative and the singular back.
    #[test]
    fn pronoun_case_and_be_agreement_from_the_tree() {
        let m = Morph;
        // I(1) saw(2) them(3) and(4) they(5) were(6) here(7) — «they» heads «were» as subject
        let lemmas = ["i", "see", "they", "and", "they", "be", "this", "apple"];
        let tags = [Tag::PRP, Tag::VBD, Tag::PRP, Tag::CC, Tag::PRP, Tag::VBD, Tag::DT, Tag::NNS];
        let heads = [2, 0, 2, 6, 6, 2, 8, 2];
        let rels = [Rel::Nsubj, Rel::Root, Rel::Obj, Rel::Cc, Rel::Nsubj, Rel::Conj, Rel::Det, Rel::Obj];
        let f = tree_feats(&lemmas, &tags, &heads, &rels);
        let forms: Vec<String> = (0..lemmas.len()).map(|i| m.inflect_feats(lemmas[i], tags[i], f[i])).collect();
        assert_eq!(forms, ["i", "saw", "them", "and", "they", "were", "these", "apples"]);
        let plain: Vec<String> = (0..lemmas.len()).map(|i| m.inflect(lemmas[i], tags[i])).collect();
        assert_eq!(plain[2], "they");
        assert_eq!(plain[5], "was");
        assert_eq!(plain[6], "this");
        // «I am», «you are»
        let f = tree_feats(&["i", "be"], &[Tag::PRP, Tag::VBP], &[2, 0], &[Rel::Nsubj, Rel::Root]);
        assert_eq!(m.inflect_feats("be", Tag::VBP, f[1]), "am");
        let f = tree_feats(&["she", "see", "i"], &[Tag::PRP, Tag::VBZ, Tag::PRP], &[2, 0, 2], &[Rel::Nsubj, Rel::Root, Rel::Obj]);
        assert_eq!(m.inflect_feats("i", Tag::PRP, f[2]), "me");
        assert_eq!(m.inflect_feats("she", Tag::PRP, f[0]), "she");
    }
}

#[cfg(test)]
mod wave27_tests {
    use super::*;
    #[test]
    fn lossless_escape_and_features() {
        let m = Morph;
        for (f, t) in [
            ("", Tag::NN),
            ("MiXeD?", Tag::NN),
            ("Qzxv", Tag::NN),
            ("n’t", Tag::RB),
            ("ß", Tag::NN),
        ] {
            let (l, c) = m.encode(f, t);
            assert_eq!(m.decode(&l, t, c), f);
        }
        assert_eq!(
            m.inflect_feats("i", Tag::PRP, Feats::parse("Case=Acc").0),
            "me"
        );
        assert_eq!(
            m.inflect_feats("be", Tag::VBD, Feats::parse("Person=2").0),
            "were"
        );
        assert_eq!(m.decode("MiXeD?", Tag::NN, Case::Raw), "MiXeD?");
        assert_eq!(
            ud_best(
                vec![
                    Rec {
                        tag: Tag::NN,
                        src: dict::UD,
                        id: 1,
                        count: 3
                    },
                    Rec {
                        tag: Tag::NN,
                        src: dict::UD,
                        id: 2,
                        count: 3
                    }
                ]
                .into_iter(),
                Tag::NN
            )
            .unwrap()
            .id,
            2
        );
    }
}
