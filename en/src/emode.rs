//! Early Modern English (EModE), as in the King James and JPS 1917 Bibles: words whose analysis does not depend on
//! the context in that register, given to the tagger before it tags and to the annotation after it.
//!
//! The small model learns on UD English, which has 6 archaic pronouns in all; the Torah has one in 32 words. A closed
//! table says what the model cannot learn: «thee» is a second person pronoun, «shalt» is *shall*, «hath» is *have*
//! (`languages/en-x-emode/13-improvements.md`, T1). Each entry can say why it fired (`Entry::why`).
//!
//! Two steps, both deterministic:
//! - `force` — the PTB tag of each word the table knows, in its context; the tagger keeps it (`PTagger::tag_forced`), so
//!   the parser and the tags of the neighbours see the right class;
//! - `fix` — lemma, UPOS and FEATS of those words after the parse; FEATS the registry does not allow for the final UPOS
//!   are dropped (`Style=Arch` is allowed on AUX, VERB, PRON, ADV, ADP, not on SCONJ or INTJ).

use std::sync::OnceLock;

use crate::gram::{Feats, Rel, Tag, UPos};
use crate::udreg::Registry;

/// The context an entry needs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum When {
    Always,
    /// «art», «wilt»: a noun and a modern verb elsewhere; the archaic auxiliary only with «thou» up to two words before
    /// or one after («thou art», «art thou», «thou also wilt»).
    NearThou,
    /// «behold», «lo» as an interjection: first in the sentence, after a punctuation mark or «and», or before one.
    Interjection,
    /// «ye»: not «Ye» before a capitalised word («Ye Olde Shoppe»).
    NotOlde,
    /// «nigh» the adverb («drew nigh»); after *be* it is the adjective («the day is nigh»), left to the tagger.
    NotAfterBe,
    /// «LORD», «GOD» in small capitals (the tetragrammaton in the JPS and King James Bibles): written all in capitals in
    /// a sentence that is not («the LORD said»; not «THE LORD OF THE RINGS», not «the lord of the manor»).
    Capitals,
}

/// A table entry: form (lower case), PTB tag, UPOS (`None` — AUX or VERB from the parse, as for *have*, *do*), lemma
/// and FEATS.
#[derive(Debug)]
pub struct Entry {
    pub form: &'static str,
    pub tag: Tag,
    pub upos: Option<UPos>,
    pub lemma: &'static str,
    pub feats: &'static str,
    pub when: When,
    pub why: &'static str,
}

macro_rules! e {
    ($f:literal, $t:ident, $u:expr, $l:literal, $x:expr, $w:ident, $why:literal) => {
        Entry { form: $f, tag: Tag::$t, upos: $u, lemma: $l, feats: $x, when: When::$w, why: $why }
    };
}

const PRON: Option<UPos> = Some(UPos::PRON);
const AUX: Option<UPos> = Some(UPos::AUX);
const VERB: Option<UPos> = Some(UPos::VERB);
const EITHER: Option<UPos> = None;
const MODAL: &str = "Style=Arch|VerbForm=Fin";
const ADV: Option<UPos> = Some(UPos::ADV);
/// FEATS from the annotation, not from the table.
const KEEP: &str = "";

/// T1: pronouns, auxiliaries, and other closed words of the biblical register.
pub const CLOSED: &[Entry] = &[
    e!("thou", PRP, PRON, "thou", "Case=Nom|Number=Sing|Person=2|PronType=Prs|Style=Arch", Always, "second person singular pronoun, subject"),
    e!("thee", PRP, PRON, "thou", "Case=Acc|Number=Sing|Person=2|PronType=Prs|Style=Arch", Always, "second person singular pronoun, object"),
    e!("thy", PRPS, PRON, "thy", "Case=Gen|Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch", Always, "second person singular possessive"),
    e!("thine", PRPS, PRON, "thy", "Case=Gen|Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch", Always, "second person singular possessive"),
    e!("thyself", PRP, PRON, "thyself", "Case=Acc|Number=Sing|Person=2|PronType=Prs|Reflex=Yes|Style=Arch", Always, "second person singular reflexive"),
    e!("ye", PRP, PRON, "ye", "Case=Nom|Number=Plur|Person=2|PronType=Prs|Style=Arch", NotOlde, "second person plural pronoun, subject"),
    e!("shalt", MD, AUX, "shall", MODAL, Always, "shall, second person singular"),
    e!("wilt", MD, AUX, "will", MODAL, NearThou, "will, second person singular"),
    e!("canst", MD, AUX, "can", MODAL, Always, "can, second person singular"),
    e!("couldest", MD, AUX, "could", MODAL, Always, "could, second person singular"),
    e!("couldst", MD, AUX, "could", MODAL, Always, "could, second person singular"),
    e!("mayest", MD, AUX, "may", MODAL, Always, "may, second person singular"),
    e!("mayst", MD, AUX, "may", MODAL, Always, "may, second person singular"),
    e!("mightest", MD, AUX, "might", MODAL, Always, "might, second person singular"),
    e!("shouldest", MD, AUX, "should", MODAL, Always, "should, second person singular"),
    e!("shouldst", MD, AUX, "should", MODAL, Always, "should, second person singular"),
    e!("wouldest", MD, AUX, "would", MODAL, Always, "would, second person singular"),
    e!("wouldst", MD, AUX, "would", MODAL, Always, "would, second person singular"),
    e!("hast", VBP, EITHER, "have", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Pres|VerbForm=Fin", Always, "have, second person singular present"),
    e!("hath", VBZ, EITHER, "have", "Mood=Ind|Number=Sing|Person=3|Style=Arch|Tense=Pres|VerbForm=Fin", Always, "have, third person singular present"),
    e!("hadst", VBD, EITHER, "have", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Past|VerbForm=Fin", Always, "have, second person singular past"),
    e!("dost", VBP, EITHER, "do", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Pres|VerbForm=Fin", Always, "do, second person singular present"),
    e!("doth", VBZ, EITHER, "do", "Mood=Ind|Number=Sing|Person=3|Style=Arch|Tense=Pres|VerbForm=Fin", Always, "do, third person singular present"),
    e!("didst", VBD, EITHER, "do", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Past|VerbForm=Fin", Always, "do, second person singular past"),
    e!("art", VBP, AUX, "be", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Pres|VerbForm=Fin", NearThou, "be, second person singular present"),
    e!("wast", VBD, AUX, "be", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Past|VerbForm=Fin", Always, "be, second person singular past"),
    e!("wert", VBD, AUX, "be", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Past|VerbForm=Fin", Always, "be, second person singular past"),
    e!("saith", VBZ, VERB, "say", "Mood=Ind|Number=Sing|Person=3|Style=Arch|Tense=Pres|VerbForm=Fin", Always, "say, third person singular present"),
    e!("sayest", VBP, VERB, "say", "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Pres|VerbForm=Fin", Always, "say, second person singular present"),
    e!("lest", IN, Some(UPos::SCONJ), "lest", "_", Always, "conjunction: for fear that"),
    e!("unto", IN, Some(UPos::ADP), "unto", "_", Always, "preposition: to"),
    e!("behold", UH, Some(UPos::INTJ), "behold", "_", Interjection, "interjection: look"),
    e!("lo", UH, Some(UPos::INTJ), "lo", "_", Interjection, "interjection: look"),
    e!("aught", NN, PRON, "aught", "PronType=Ind", Always, "pronoun: anything"),
    e!("nought", NN, PRON, "nought", "PronType=Neg", Always, "pronoun: nothing"),
    e!("naught", NN, PRON, "naught", "PronType=Neg", Always, "pronoun: nothing"),
    e!("whosoever", WP, PRON, "whosoever", "PronType=Rel", Always, "relative pronoun: whoever"),
    e!("whoso", WP, PRON, "whoso", "PronType=Rel", Always, "relative pronoun: whoever"),
    e!("yea", UH, Some(UPos::INTJ), "yea", "_", Always, "interjection: yes, indeed"),
    e!("nay", UH, Some(UPos::INTJ), "nay", "_", Always, "interjection: no"),
    // T2: pro-adverbs and other old adverbs; the «where-» words take their PronType (Int or Rel) from the annotation
    e!("thereof", RB, ADV, "thereof", "_", Always, "adverb: of it, of that"),
    e!("therein", RB, ADV, "therein", "_", Always, "adverb: in it"),
    e!("thereon", RB, ADV, "thereon", "_", Always, "adverb: on it"),
    e!("thereby", RB, ADV, "thereby", "_", Always, "adverb: by that"),
    e!("thereto", RB, ADV, "thereto", "_", Always, "adverb: to it"),
    e!("therewith", RB, ADV, "therewith", "_", Always, "adverb: with it"),
    e!("therefrom", RB, ADV, "therefrom", "_", Always, "adverb: from it"),
    e!("thereat", RB, ADV, "thereat", "_", Always, "adverb: at it"),
    e!("thereunto", RB, ADV, "thereunto", "_", Always, "adverb: to it"),
    e!("thereupon", RB, ADV, "thereupon", "_", Always, "adverb: upon that"),
    e!("thither", RB, ADV, "thither", "_", Always, "adverb: to that place"),
    e!("hither", RB, ADV, "hither", "_", Always, "adverb: to this place"),
    e!("thence", RB, ADV, "thence", "_", Always, "adverb: from that place"),
    e!("hence", RB, ADV, "hence", "_", Always, "adverb: from here; therefore"),
    e!("peradventure", RB, ADV, "peradventure", "_", Always, "adverb: perhaps"),
    e!("howbeit", RB, ADV, "howbeit", "_", Always, "adverb: nevertheless"),
    e!("yonder", RB, ADV, "yonder", "_", Always, "adverb: over there"),
    e!("nigh", RB, ADV, "nigh", "_", NotAfterBe, "adverb: near"),
    e!("betimes", RB, ADV, "betimes", "_", Always, "adverb: early"),
    e!("hitherto", RB, ADV, "hitherto", "_", Always, "adverb: until now"),
    e!("henceforth", RB, ADV, "henceforth", "_", Always, "adverb: from now on"),
    e!("thenceforth", RB, ADV, "thenceforth", "_", Always, "adverb: from then on"),
    e!("aforetime", RB, ADV, "aforetime", "_", Always, "adverb: formerly"),
    e!("verily", RB, ADV, "verily", "_", Always, "adverb: truly"),
    e!("whereof", WRB, ADV, "whereof", KEEP, Always, "adverb: of which, of what"),
    e!("wherein", WRB, ADV, "wherein", KEEP, Always, "adverb: in which, in what"),
    e!("whereon", WRB, ADV, "whereon", KEEP, Always, "adverb: on which"),
    e!("wherewith", WRB, ADV, "wherewith", KEEP, Always, "adverb: with which, with what"),
    e!("whereby", WRB, ADV, "whereby", KEEP, Always, "adverb: by which"),
    e!("whereunto", WRB, ADV, "whereunto", KEEP, Always, "adverb: to which"),
    e!("whereupon", WRB, ADV, "whereupon", KEEP, Always, "adverb: upon which"),
    e!("whereat", WRB, ADV, "whereat", KEEP, Always, "adverb: at which"),
    e!("whither", WRB, ADV, "whither", KEEP, Always, "adverb: to which place, where to"),
    e!("whence", WRB, ADV, "whence", KEEP, Always, "adverb: from which place, where from"),
    e!("wherefore", WRB, ADV, "wherefore", KEEP, Always, "adverb: why; therefore"),
    // T4: the divine name in capitals
    e!("lord", NNP, Some(UPos::PROPN), "LORD", "Number=Sing", Capitals, "the divine name, printed in capitals"),
    e!("god", NNP, Some(UPos::PROPN), "GOD", "Number=Sing", Capitals, "the divine name, printed in capitals"),
    // second readings, for a tag given elsewhere (the graph parser): never given to the tagger
    e!("thine", PRP, PRON, "thy", "Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch", Always, "second person singular possessive, alone"),
];

/// The entry of a form (any case).
pub fn entry(form: &str) -> Option<&'static Entry> {
    let low = form.to_lowercase();
    CLOSED.iter().find(|e| e.form == low)
}

fn punct(w: &str) -> bool {
    !w.is_empty() && w.chars().all(|c| c.is_ascii_punctuation() || matches!(c, '‘' | '’' | '“' | '”' | '—' | '–'))
}

fn applies(e: &Entry, words: &[&str], i: usize) -> bool {
    let low = |k: usize| words.get(k).map(|w| w.to_lowercase());
    match e.when {
        When::Always => true,
        When::NearThou => [i.wrapping_sub(2), i.wrapping_sub(1), i + 1].iter().any(|&k| low(k).as_deref() == Some("thou")),
        When::Interjection => {
            i == 0
                || punct(words[i - 1])
                || low(i - 1).as_deref() == Some("and")
                || words.get(i + 1).is_some_and(|w| punct(w))
        }
        When::NotOlde => !(words[i] == "Ye" && words.get(i + 1).is_some_and(|w| w.chars().next().is_some_and(char::is_uppercase))),
        When::Capitals => {
            let caps = |w: &str| w.chars().all(|c| !c.is_lowercase()) && w.chars().filter(|c| c.is_alphabetic()).count() > 1;
            caps(words[i]) && words.iter().any(|w| w.chars().any(char::is_lowercase))
        }
        When::NotAfterBe => {
            !(i > 0 && matches!(low(i - 1).as_deref(), Some("be" | "is" | "was" | "are" | "were" | "been" | "art" | "am" | "wast")))
        }
    }
}

/// The entry for word `i` if its context allows it (the first of its form: the one the tagger is given).
pub fn closed(words: &[&str], i: usize) -> Option<&'static Entry> {
    entry(words[i]).filter(|e| applies(e, words, i))
}

/// The entry of word `i` for a given tag, if its context allows it: the first entry, or another reading of the same
/// form the table also knows («thine» PRP$ before a noun, PRP alone: «the land is thine»).
pub fn tagged(words: &[&str], i: usize, tag: Tag) -> Option<&'static Entry> {
    let low = words[i].to_lowercase();
    CLOSED.iter().find(|e| e.form == low && e.tag == tag && applies(e, words, i))
}

/// T3: a word the dictionary does not know, ending in «-eth», «-est» or «-st», whose stem is a known verb: lemma and
/// tag of that verb (`morph::ending_verb`: «giveth» → give VBZ, «goest» → go VBP, «saidst» → say VBD). Words the
/// dictionary knows keep their analysis: *priest*, *harvest*, *greatest*, *earth*, *teeth*, *twentieth*; so do
/// stems that are no verb (*Japheth*) and the proper nouns of the text (a capital inside the sentence, not after a
/// punctuation mark: «said: 'Knowest thou…»).
pub fn ending(words: &[&str], i: usize) -> Option<(String, Tag)> {
    let w = words[i];
    if !w.chars().all(|c| c.is_ascii_alphabetic()) || (i > 0 && !punct(words[i - 1]) && w.chars().next().is_some_and(char::is_uppercase)) {
        return None;
    }
    let low = w.to_lowercase();
    if entry(&low).is_some() || crate::morph::known_form(&low) {
        return None;
    }
    crate::morph::ending_verb(&low)
}

/// FEATS the ending gives: «-eth» third person, «-est»/«-st» second, singular; present or past.
fn ending_feats(tag: Tag) -> &'static str {
    match tag {
        Tag::VBZ => "Mood=Ind|Number=Sing|Person=3|Style=Arch|Tense=Pres|VerbForm=Fin",
        Tag::VBD => "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Past|VerbForm=Fin",
        _ => "Mood=Ind|Number=Sing|Person=2|Style=Arch|Tense=Pres|VerbForm=Fin",
    }
}

/// The tag each word must have, where the table (T1) or the ending rule (T3) knows it.
pub fn force(words: &[&str]) -> Vec<Option<Tag>> {
    (0..words.len()).map(|i| closed(words, i).map(|e| e.tag).or_else(|| ending(words, i).map(|x| x.1))).collect()
}

fn registry() -> &'static Registry {
    static R: OnceLock<Registry> = OnceLock::new();
    R.get_or_init(Registry::load)
}

/// FEATS pairs the registry allows for a UPOS.
fn allowed(u: UPos, f: Feats) -> Feats {
    let ok = registry().allowed(u);
    Feats(f.iter().filter(|x| ok.contains(x)).fold(0, |a, x| a | 1 << x.idx()))
}

/// Lemma, UPOS, FEATS and relation of a word the table knows, after the parse (`tag`, `upos` and `rel` are the
/// annotation's; a word whose tag is not the table's — tagged elsewhere — is left alone). Only *be* can be a copula:
/// «hath» the parser took for one («the God who hath been my shepherd») becomes `aux`.
pub fn fix(words: &[&str], i: usize, tag: Tag, upos: UPos, rel: Rel, feats: Feats) -> Option<(String, UPos, Feats, Rel)> {
    let Some(e) = tagged(words, i, tag) else {
        // T3: a lexical verb
        let (lemma, t) = ending(words, i).filter(|x| x.1 == tag)?;
        let u = if upos == UPos::AUX && registry().auxiliaries().iter().any(|a| *a == lemma) { UPos::AUX } else { UPos::VERB };
        return Some((lemma, u, allowed(u, Feats::parse(ending_feats(t)).0), if rel == Rel::Cop { Rel::Aux } else { rel }));
    };
    let u = e.upos.unwrap_or(if matches!(upos, UPos::AUX | UPos::VERB) {
        upos
    } else if matches!(rel.base(), Rel::Aux | Rel::Cop) {
        UPos::AUX
    } else {
        UPos::VERB
    });
    let rel = if rel == Rel::Cop && e.lemma != "be" { Rel::Aux } else { rel };
    let f = if e.feats == KEEP { feats } else { Feats::parse(e.feats).0 };
    Some((e.lemma.to_string(), u, allowed(u, f), rel))
}

/// S1: «not» right after a lexical verb, or after that verb's postposed pronoun («I know not», «fear ye
/// not»), is the verb's `advmod` (EModE negates the lexical verb itself, without *do*). A parser trained on modern
/// English attaches it to the next word («not the day»). Not after *let* («let not the Lord be angry»: the
/// negation is the lower clause's), nor after an auxiliary («I will not eat») or a form of *be*, *have*, *do*, nor
/// before a non-finite verb, an adjective or «only» (modern English: «recommend not using»). Returns how many were moved;
/// a move that would make a cycle is not made.
pub fn postverbal_not(forms: &[&str], upos: &[UPos], tags: &[Tag], heads: &mut [usize], rels: &mut [Rel]) -> usize {
    let mut moved = 0;
    for i in 1..forms.len() {
        if !forms[i].eq_ignore_ascii_case("not") {
            continue;
        }
        // modern «not» before a non-finite or a predicate: «recommend not using», «know not to eat», «make them not
        // noticeable», «not only»
        if tags.get(i + 1).is_some_and(|t| matches!(t, Tag::VBG | Tag::VBN | Tag::TO | Tag::JJ | Tag::JJR | Tag::JJS))
            || forms.get(i + 1).is_some_and(|w| w.eq_ignore_ascii_case("only"))
        {
            continue;
        }
        let pron = |k: usize| upos[k] == UPos::PRON && tags[k] == Tag::PRP;
        let v = if pron(i - 1) && i >= 2 && upos[i - 2] == UPos::VERB { i - 2 } else { i - 1 };
        let low = forms[v].to_lowercase();
        // *let*, and *be*, *have*, *do* (auxiliaries even when the parse makes them heads: «has not a Tom among them»)
        const SKIP: [&str; 21] = [
            "let", "be", "is", "am", "are", "was", "were", "been", "art", "wast", "have", "has", "had", "hath", "hast", "hadst", "do",
            "does", "did", "doth", "dost",
        ];
        if upos[v] != UPos::VERB || !matches!(tags[v], Tag::VB | Tag::VBD | Tag::VBP | Tag::VBZ) || SKIP.contains(&low.as_str()) {
            continue;
        }
        if heads[i] == v + 1 {
            continue;
        }
        // the verb must not hang below «not»
        let (mut h, mut steps) = (heads[v], 0);
        while h != 0 && h != i + 1 && steps <= forms.len() {
            h = heads[h - 1];
            steps += 1;
        }
        if h == i + 1 {
            continue;
        }
        heads[i] = v + 1;
        rels[i] = Rel::Advmod;
        moved += 1;
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_words_in_context() {
        let s = ["And", "thou", "shalt", "love", "thy", "neighbour", "as", "thyself", "."];
        let f = force(&s);
        assert_eq!(f, vec![None, Some(Tag::PRP), Some(Tag::MD), None, Some(Tag::PRPS), None, None, Some(Tag::PRP), None]);
        // «art» with «thou», and the noun without it
        assert_eq!(force(&["Where", "art", "thou", "?"])[1], Some(Tag::VBP));
        assert_eq!(force(&["thou", "also", "art", "a", "man"])[2], Some(Tag::VBP));
        assert_eq!(force(&["the", "art", "of", "the", "perfumer"])[1], None);
        assert_eq!(force(&["modern", "art", "is", "here"])[1], None);
        // «wilt» the verb (to droop)
        assert_eq!(force(&["the", "flowers", "wilt", "in", "heat"])[2], None);
        assert_eq!(force(&["wilt", "thou", "go"])[0], Some(Tag::MD));
        // «Ye Olde» is no pronoun; «Ye shall» is
        assert_eq!(force(&["Ye", "Olde", "Shoppe"])[0], None);
        assert_eq!(force(&["Ye", "shall", "not", "eat"])[0], Some(Tag::PRP));
        // «behold» the verb and the interjection
        assert_eq!(force(&["And", ",", "behold", ",", "a", "ladder"])[2], Some(Tag::UH));
        assert_eq!(force(&["Behold", "the", "man"])[0], Some(Tag::UH));
        assert_eq!(force(&["they", "shall", "behold", "the", "land"])[2], None);
        // T2: the old adverbs; «nigh» after «be» is the adjective
        assert_eq!(force(&["the", "bones", "thereof"])[2], Some(Tag::RB));
        assert_eq!(force(&["whither", "goest", "thou"])[0], Some(Tag::WRB));
        assert_eq!(force(&["they", "drew", "nigh"])[2], Some(Tag::RB));
        assert_eq!(force(&["the", "day", "is", "nigh"])[3], None);
        // T4: the divine name in capitals; a lord in lower case, a title in capitals are left to the tagger
        assert_eq!(force(&["And", "the", "LORD", "said"])[2], Some(Tag::NNP));
        assert_eq!(closed(&["O", "Lord", "GOD", ",", "remember", "me"], 2).map(|e| e.lemma), Some("GOD"));
        assert_eq!(force(&["the", "lord", "of", "the", "manor"])[1], None);
        assert_eq!(force(&["the", "Lord", "Mayor"])[1], None);
        assert_eq!(force(&["THE", "LORD", "OF", "THE", "RINGS"])[1], None);
        assert_eq!(force(&["my", "god", "!"])[1], None);
        // modern words are not in the table
        for w in ["digest", "interest", "best", "you", "your", "has", "have", "does", "be", "to", "nothing", "whoever", "where", "there", "therefore", "near"] {
            assert!(entry(w).is_none(), "{w}");
        }
        for w in ["lord", "Lord", "god", "God"] {
            assert!(closed(&["the", w, "is", "here"], 1).is_none(), "{w}");
        }
    }

    #[test]
    fn ending_rule() {
        if crate::dict::N_LEMMAS == 0 {
            return;
        }
        let s = ["He", "that", "giveth", "and", "thou", "that", "goest", ",", "as", "thou", "saidst", "."];
        let f = force(&s);
        assert_eq!((f[2], f[6], f[10]), (Some(Tag::VBZ), Some(Tag::VBP), Some(Tag::VBD)));
        let (l, u, x, _) = fix(&s, 2, Tag::VBZ, UPos::NOUN, Rel::Nsubj, Feats::default()).unwrap();
        assert_eq!((l.as_str(), u), ("give", UPos::VERB));
        assert_eq!(x.to_string(), "Mood=Ind|Number=Sing|Person=3|Style=Arch|Tense=Pres|VerbForm=Fin");
        assert_eq!(fix(&s, 10, Tag::VBD, UPos::VERB, Rel::Advcl, Feats::default()).unwrap().0, "say");
        // a lexical verb the parser took for an auxiliary is a verb: only the registry's auxiliaries are AUX
        assert_eq!(fix(&["thou", "knowest", "not"], 1, Tag::VBP, UPos::AUX, Rel::Aux, Feats::default()).unwrap().1, UPos::VERB);
        // negative controls from the Torah: nouns, adjectives, numerals and names stay with the tagger
        let neg = [
            "priest", "against", "beast", "feast", "harvest", "interest", "youngest", "greatest", "earth", "death", "teeth", "twentieth",
            "digest", "best", "lest", "rest", "honest", "forest", "first", "must", "west", "breast", "dust", "east", "Beth", "Heth", "Japheth",
        ];
        for w in neg {
            let s = ["the", w, "of", "it"];
            assert_eq!(force(&s)[1], entry(w).map(|e| e.tag), "{w}");
            assert!(ending(&s, 1).is_none(), "{w}");
        }
        // a capital inside the sentence is a name
        assert!(ending(&["unto", "Goeth"], 1).is_none());
        assert_eq!(ending(&["said", ":", "'", "Knowest", "thou"], 3), Some(("know".to_string(), Tag::VBP)));
    }

    #[test]
    fn not_after_the_verb() {
        use UPos::*;
        // «I know not the day of my death»: «not» on «day» → on «know»
        let f = ["I", "know", "not", "the", "day"];
        let u = [PRON, VERB, PART, DET, NOUN];
        let t = [Tag::PRP, Tag::VBP, Tag::RB, Tag::DT, Tag::NN];
        let (mut h, mut r) = (vec![2, 0, 5, 5, 2], vec![Rel::Nsubj, Rel::Root, Rel::Advmod, Rel::Det, Rel::Obj]);
        assert_eq!(postverbal_not(&f, &u, &t, &mut h, &mut r), 1);
        assert_eq!((h[2], r[2]), (2, Rel::Advmod));
        // «fear ye not»: over the postposed pronoun
        let f = ["fear", "ye", "not"];
        let (mut h, mut r) = (vec![0, 1, 2], vec![Rel::Root, Rel::Nsubj, Rel::Advmod]);
        assert_eq!(postverbal_not(&f, &[VERB, PRON, PART], &[Tag::VB, Tag::PRP, Tag::RB], &mut h, &mut r), 1);
        assert_eq!(h[2], 1);
        // negative controls: an auxiliary before «not», «let not», no verb, already on the verb
        let f = ["I", "will", "not", "eat"];
        let (mut h, mut r) = (vec![4, 4, 4, 0], vec![Rel::Nsubj, Rel::Aux, Rel::Advmod, Rel::Root]);
        assert_eq!(postverbal_not(&f, &[PRON, AUX, PART, VERB], &[Tag::PRP, Tag::MD, Tag::RB, Tag::VB], &mut h, &mut r), 0);
        let f = ["let", "not", "the", "Lord", "be", "angry"];
        let (mut h, mut r) = (vec![0, 6, 4, 6, 6, 1], vec![Rel::Root, Rel::Advmod, Rel::Det, Rel::Nsubj, Rel::Cop, Rel::Ccomp]);
        assert_eq!(postverbal_not(&f, &[VERB, PART, DET, PROPN, AUX, ADJ], &[Tag::VB, Tag::RB, Tag::DT, Tag::NNP, Tag::VB, Tag::JJ], &mut h, &mut r), 0);
        let f = ["not", "the", "sinew", "but", "the", "bone"];
        let (mut h, mut r) = (vec![3, 3, 0, 6, 6, 3], vec![Rel::Advmod, Rel::Det, Rel::Root, Rel::Cc, Rel::Det, Rel::Conj]);
        assert_eq!(postverbal_not(&f, &[PART, DET, NOUN, CCONJ, DET, NOUN], &[Tag::RB, Tag::DT, Tag::NN, Tag::CC, Tag::DT, Tag::NN], &mut h, &mut r), 0);
        // modern English: «recommend not using», «know not to eat», «has not a Tom»
        let f = ["I", "recommend", "not", "using", "it"];
        let (mut h, mut r) = (vec![2, 0, 4, 2, 4], vec![Rel::Nsubj, Rel::Root, Rel::Advmod, Rel::Xcomp, Rel::Obj]);
        assert_eq!(postverbal_not(&f, &[PRON, VERB, PART, VERB, PRON], &[Tag::PRP, Tag::VBP, Tag::RB, Tag::VBG, Tag::PRP], &mut h, &mut r), 0);
        let f = ["would", "know", "not", "to", "eat"];
        let (mut h, mut r) = (vec![2, 0, 5, 5, 2], vec![Rel::Aux, Rel::Root, Rel::Advmod, Rel::Mark, Rel::Xcomp]);
        assert_eq!(postverbal_not(&f, &[AUX, VERB, PART, PART, VERB], &[Tag::MD, Tag::VB, Tag::RB, Tag::TO, Tag::VB], &mut h, &mut r), 0);
        let f = ["it", "has", "not", "a", "Tom"];
        let (mut h, mut r) = (vec![2, 0, 5, 5, 2], vec![Rel::Nsubj, Rel::Root, Rel::Advmod, Rel::Det, Rel::Obj]);
        assert_eq!(postverbal_not(&f, &[PRON, VERB, PART, DET, PROPN], &[Tag::PRP, Tag::VBZ, Tag::RB, Tag::DT, Tag::NNP], &mut h, &mut r), 0);
        // a verb below «not» is not moved over (no cycle)
        let f = ["he", "knew", "not"];
        let (mut h, mut r) = (vec![2, 3, 0], vec![Rel::Nsubj, Rel::Ccomp, Rel::Root]);
        assert_eq!(postverbal_not(&f, &[PRON, VERB, PART], &[Tag::PRP, Tag::VBD, Tag::RB], &mut h, &mut r), 0);
    }

    #[test]
    fn fix_follows_the_registry() {
        if crate::dict::N_LEMMAS == 0 {
            return;
        }
        let s = ["he", "hath", "a", "son"];
        // «hath» as a main verb and as an auxiliary: UPOS from the parse, lemma and FEATS from the table
        let (l, u, f, _) = fix(&s, 1, Tag::VBZ, UPos::VERB, Rel::Root, Feats::default()).unwrap();
        assert_eq!((l.as_str(), u), ("have", UPos::VERB));
        assert_eq!(f.to_string(), "Mood=Ind|Number=Sing|Person=3|Style=Arch|Tense=Pres|VerbForm=Fin");
        let (_, u, _, r) = fix(&["he", "hath", "said"], 1, Tag::VBZ, UPos::NOUN, Rel::Aux, Feats::default()).unwrap();
        assert_eq!((u, r), (UPos::AUX, Rel::Aux));
        // only «be» is a copula
        let (_, u, _, r) = fix(&["who", "hath", "been", "my", "shepherd"], 1, Tag::VBZ, UPos::AUX, Rel::Cop, Feats::default()).unwrap();
        assert_eq!((u, r), (UPos::AUX, Rel::Aux));
        let (_, _, _, r) = fix(&["thou", "art", "a", "man"], 1, Tag::VBP, UPos::AUX, Rel::Cop, Feats::default()).unwrap();
        assert_eq!(r, Rel::Cop);
        // a pronoun's features come from the table, never from the most frequent PRP («it»)
        let (l, u, f, _) = fix(&["I", "love", "thee"], 2, Tag::PRP, UPos::NOUN, Rel::Obj, Feats::default()).unwrap();
        assert_eq!((l.as_str(), u), ("thou", UPos::PRON));
        assert_eq!(f.to_string(), "Case=Acc|Number=Sing|Person=2|PronType=Prs|Style=Arch");
        assert!(fix(&["the", "art"], 1, Tag::NN, UPos::NOUN, Rel::Root, Feats::default()).is_none());
        // the second reading of «thine»: alone, as EWT's «mine»
        let (l, _, f, _) = fix(&["it", "is", "thine"], 2, Tag::PRP, UPos::PRON, Rel::Root, Feats::default()).unwrap();
        assert_eq!((l.as_str(), f.to_string().as_str()), ("thy", "Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch"));
        assert_eq!(force(&["it", "is", "thine"])[2], Some(Tag::PRPS));
        // tagged elsewhere as something else: left alone
        assert!(fix(&["I", "love", "thee"], 2, Tag::NN, UPos::NOUN, Rel::Obj, Feats::default()).is_none());
        // the registry filter: Style=Arch is not a feature of an interjection or a subordinator
        let arch = Feats::parse("Style=Arch").0;
        assert!(allowed(UPos::INTJ, arch).is_empty());
        assert!(allowed(UPos::SCONJ, arch).is_empty());
        assert_eq!(allowed(UPos::PRON, arch), arch);
    }
}
