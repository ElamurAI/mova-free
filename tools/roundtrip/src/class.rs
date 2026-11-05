//! The layers and the classes of loss: every residual item and every divergence is put in one class.

use en::gram::{Rel, Tag};

/// A layer boundary: the representation above it is decoded into the one below.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Layer {
    /// a mixed text ↔ its language pieces (langseg; back: the pieces joined with one space)
    Route,
    /// text ↔ tokens (`en::tok`): forms and the space-after bit
    Tokens,
    /// tokens ↔ UD tree (`en` annotator; back: `lin.rs` order + `morph.rs` forms + spacing rules)
    Tree,
    /// UD tree ↔ frames (`prag::frames`; back: the realizer of this crate, then the tree decoder)
    Frames,
    /// frames ↔ world (the story's state: entity registry + event line; back: one frame per event)
    World,
}

pub const LAYERS: [Layer; 4] = [Layer::Tokens, Layer::Tree, Layer::Frames, Layer::World];

impl Layer {
    pub fn name(self) -> &'static str {
        match self {
            Layer::Route => "route",
            Layer::Tokens => "tokens",
            Layer::Tree => "tree",
            Layer::Frames => "frames",
            Layer::World => "world",
        }
    }
    pub fn boundary(self) -> &'static str {
        match self {
            Layer::Route => "text ↔ pieces",
            Layer::Tokens => "text ↔ tokens",
            Layer::Tree => "tokens ↔ tree",
            Layer::Frames => "tree ↔ frames",
            Layer::World => "frames ↔ world",
        }
    }
}

/// A class of loss.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Class {
    /// ids, lesson keys, lines that are not text
    Meta,
    /// line wraps and paragraph breaks
    Layout,
    /// spacing and tokenization: SpaceAfter, double spaces, characters the tokenizer dropped
    Spacing,
    Punct,
    /// capitalization
    Case,
    /// word form / inflection (number, degree, pronoun case) with the same lemma
    Form,
    /// lemma → form choice: the form is a known word, but not the one `morph` makes from lemma and tag
    Choice,
    /// spelling of the source: the form is no known word form (archaic, dialect, misspelt, foreign)
    Spelling,
    /// typographic marks: ’ “ ” where the dictionary and the generator have ' and "
    Typography,
    /// word order
    Order,
    /// function words: determiners, prepositions, conjunctions, particles, markers
    Function,
    /// a pronoun where the other side has a name (or the reverse)
    PronName,
    /// tense, aspect, modality: auxiliaries, modals, the copula, verb tense
    Tense,
    Neg,
    /// numbers, quantifiers, grammatical number
    Quantity,
    /// content words and modifiers dropped (or added)
    Dropped,
    /// another lemma in the same place: a synonym, a wrong lemma
    Lexical,
    /// the same words, attached differently (head or relation)
    Attachment,
    /// sentence boundaries
    Sentence,
    /// discourse links between frames (next, cause, what was said)
    Links,
    /// direct speech versus reported
    Quote,
    /// a repeated statement the world's state does not keep (it changes nothing)
    Repetition,
    /// cantillation accents (teʿamim), meteg, rafe: marks of the written Hebrew text the generator does not write
    Accent,
    /// text with no parser yet (Aramaic): kept whole at the token layer, nothing above it holds it
    Unparsed,
    /// the markup of a Markdown page: front matter, `<!-- … -->` markers, heading and list marks, table pipes, link
    /// targets, emphasis, code fences
    Markup,
    /// what a page's CoNLL-U block needs beyond the tree the layers give for its sentence (`blocks::site_md`)
    Conllu,
    /// what a page's frames table needs beyond the table rebuilt from that tree (`blocks::site_md`)
    FrTable,
    /// a text the volume is given from elsewhere (a translation's output, its source): kept, not counted
    Given,
    /// prose a page repeats from the prose before it in the block (a template's sentence with other names): copied
    /// with an edit instead of going up the layers (`blocks::site_md::repeat`)
    Repeat,
}

pub const CLASSES: [Class; 29] = [
    Class::Meta,
    Class::Layout,
    Class::Spacing,
    Class::Punct,
    Class::Case,
    Class::Form,
    Class::Choice,
    Class::Spelling,
    Class::Typography,
    Class::Order,
    Class::Function,
    Class::PronName,
    Class::Tense,
    Class::Neg,
    Class::Quantity,
    Class::Dropped,
    Class::Lexical,
    Class::Attachment,
    Class::Sentence,
    Class::Links,
    Class::Quote,
    Class::Repetition,
    Class::Accent,
    Class::Unparsed,
    Class::Markup,
    Class::Conllu,
    Class::FrTable,
    Class::Given,
    Class::Repeat,
];

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::Meta => "meta",
            Class::Layout => "layout",
            Class::Spacing => "spacing",
            Class::Punct => "punct",
            Class::Case => "case",
            Class::Form => "form",
            Class::Choice => "lemma→form",
            Class::Spelling => "src-spelling",
            Class::Typography => "typography",
            Class::Order => "order",
            Class::Function => "function",
            Class::PronName => "pron/name",
            Class::Tense => "tense/mod",
            Class::Neg => "negation",
            Class::Quantity => "quantity",
            Class::Dropped => "dropped",
            Class::Lexical => "lexical",
            Class::Attachment => "attachment",
            Class::Sentence => "sentence",
            Class::Links => "links",
            Class::Quote => "quote",
            Class::Repetition => "repetition",
            Class::Accent => "accent",
            Class::Unparsed => "unparsed",
            Class::Markup => "markup",
            Class::Conllu => "conllu",
            Class::FrTable => "frames-table",
            Class::Given => "given",
            Class::Repeat => "repeat",
        }
    }
}

const NEG: [&str; 11] = ["not", "n't", "never", "no", "nor", "neither", "nobody", "nothing", "none", "nowhere", "cannot"];
const QUANT: [&str; 15] = ["all", "some", "many", "much", "few", "several", "every", "each", "both", "any", "more", "most", "less", "half", "enough"];

pub fn is_punct_tag(t: Tag) -> bool {
    matches!(t, Tag::Comma | Tag::Stop | Tag::Colon | Tag::LRB | Tag::RRB | Tag::LQuote | Tag::RQuote | Tag::DQuote | Tag::HYPH | Tag::NFP)
}

/// The class of a word that is missing on one side (or added): what kind of word it is.
pub fn of_word(tag: Tag, rel: Rel, lemma: &str) -> Class {
    let l = lemma.to_lowercase();
    if rel == Rel::Punct || is_punct_tag(tag) {
        Class::Punct
    } else if NEG.contains(&l.as_str()) {
        Class::Neg
    } else if tag == Tag::CD || rel == Rel::Nummod || QUANT.contains(&l.as_str()) {
        Class::Quantity
    } else if tag == Tag::MD || matches!(rel, Rel::Aux | Rel::AuxPass | Rel::Cop) {
        Class::Tense
    } else if matches!(tag, Tag::DT | Tag::PDT | Tag::IN | Tag::TO | Tag::CC | Tag::RP | Tag::POS | Tag::EX | Tag::WDT | Tag::WP | Tag::WPS | Tag::PRPS)
        || matches!(rel, Rel::Det | Rel::Case | Rel::Mark | Rel::Cc | Rel::Expl | Rel::CompoundPrt | Rel::Fixed)
    {
        Class::Function
    } else {
        Class::Dropped
    }
}

/// Coarse part of speech for alignment.
pub fn coarse(t: Tag) -> u8 {
    if t.is_noun() {
        1
    } else if t.is_verb() || t == Tag::MD {
        2
    } else if matches!(t, Tag::JJ | Tag::JJR | Tag::JJS) {
        3
    } else if matches!(t, Tag::RB | Tag::RBR | Tag::RBS | Tag::WRB) {
        4
    } else if matches!(t, Tag::PRP | Tag::PRPS | Tag::WP | Tag::WPS | Tag::EX) {
        5
    } else if is_punct_tag(t) {
        6
    } else {
        7
    }
}

/// The class of a tag change on the same word.
pub fn of_tag_change(a: Tag, b: Tag) -> Class {
    if (a.is_verb() || a == Tag::MD) && (b.is_verb() || b == Tag::MD) {
        Class::Tense
    } else if a.is_noun() && b.is_noun() && a.is_proper() == b.is_proper() {
        Class::Quantity
    } else if coarse(a) == coarse(b) {
        Class::Form
    } else {
        Class::Attachment
    }
}

/// The class of a lemma change in the same place: a pronoun for a name, another function word (or quantifier,
/// negation, auxiliary) in the class of the word the text has, another content word.
pub fn of_lemma_change(a_tag: Tag, b: &crate::tree::TNode) -> Class {
    let pron = |t: Tag| matches!(t, Tag::PRP | Tag::PRPS);
    if (pron(a_tag) && b.tag.is_noun()) || (pron(b.tag) && a_tag.is_noun()) {
        return Class::PronName;
    }
    match of_word(b.tag, b.rel, &b.lemma) {
        Class::Dropped => Class::Lexical,
        c => c,
    }
}

/// Bits of a literal string: 8 a byte; a character of the Hebrew script (a letter, a point, an accent) 8, as an
/// English letter — UTF-8 spends two bytes on it.
pub fn lit_bits(s: &str) -> f64 {
    8.0 * s.chars().map(|c| if matches!(c, '\u{0590}'..='\u{05FF}' | '\u{FB1D}'..='\u{FB4F}') { 1 } else { c.len_utf8() }).sum::<usize>() as f64
}

/// Bits of a position among n.
pub fn pos_bits(n: usize) -> f64 {
    ((n + 1) as f64).log2()
}

/// The lemmas of the Hebrew parser's training trees (set once by a mixed run): a Hebrew lemma among them costs
/// log2 of their number, as an English lemma costs log2 of the dictionary's.
pub static HEB_LEMMAS: std::sync::OnceLock<std::collections::BTreeSet<String>> = std::sync::OnceLock::new();

/// Bits of a lemma: a number in the dictionary heap, or the string itself.
pub fn lemma_bits(l: &str) -> f64 {
    if l.chars().next().is_some_and(langseg::is_heb_letter) {
        // a Hebrew lemma: a number among the lemmas of the Hebrew parser's training trees, when it is one of them
        if let Some(lex) = HEB_LEMMAS.get() {
            if lex.contains(l) {
                return (lex.len() as f64).log2();
            }
        }
        return lit_bits(l);
    }
    if en::dict::lemma(l).is_some() || en::dict::lemma(&l.to_lowercase()).is_some() { 17.0 } else { 8.0 * l.len() as f64 }
}

/// Bits of a word of a class: a punctuation mark is one of a few dozen (5 bits), a function word, negation or
/// auxiliary one of a few hundred (8 bits), any other word a number in the dictionary or its string.
pub fn word_bits(c: Class, lemma: &str) -> f64 {
    match c {
        Class::Punct => 5.0,
        Class::Function | Class::Neg | Class::Tense => 8.0,
        _ => lemma_bits(lemma),
    }
}

/// log2(k!)
pub fn perm_bits(k: usize) -> f64 {
    (2..=k).map(|x| (x as f64).log2()).sum()
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn precedence_and_bits() {
        assert_eq!(of_word(Tag::CD, Rel::Punct, "not"), Class::Punct);
        assert_eq!(of_word(Tag::CD, Rel::Dep, "NOT"), Class::Neg);
        assert_eq!(of_word(Tag::MD, Rel::Nummod, "all"), Class::Quantity);
        assert_eq!(of_word(Tag::MD, Rel::Det, "can"), Class::Tense);
        assert_eq!(of_word(Tag::DT, Rel::Dep, "the"), Class::Function);
        assert_eq!(of_word(Tag::NN, Rel::Dep, "cat"), Class::Dropped);
        assert_eq!(lit_bits(""), 0.0);
        assert_eq!(lit_bits("שa😀"), 48.0);
        assert_eq!(pos_bits(0), 0.0);
        assert_eq!(pos_bits(3), 2.0);
        assert_eq!(perm_bits(0), 0.0);
        assert_eq!(perm_bits(3), 6f64.log2());
        assert_eq!(of_tag_change(Tag::NN, Tag::NNS), Class::Quantity);
        assert_eq!(of_tag_change(Tag::VBD, Tag::VB), Class::Tense);
        assert_eq!(
            CLASSES
                .iter()
                .map(|c| c.name())
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            29
        );
    }
}
