//! Second gate of rule v2 — "doubtful implicit": an implicit answer is almost entirely (≥ `DOUBT` of content
//! words) composed of the lemmas of a single anchor sentence. So it is a paraphrase of an explicit one, and the verbatim gate does not
//! catch it: "because he had tasted the king's white snake" vs "…had tasted the white snake". This is
//! a mark in the log (level `doubt`), not a rejection.
//!
//! Lemmas come from `en` (model `ud-ewt-eslspok.bin`: tokenizer, tagger, lemma dictionary in code; variable
//! `QAGEN_EN_MODEL`). Content words are determined by UPOS; function words (DET, ADP, AUX, PRON, CCONJ, SCONJ, PART, PUNCT,
//! SYM) are not counted. An answer word is covered if its lemma or form is among the sentence's lemmas or forms.

use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Result;

use en::annotate::Annotator;
use en::gram::UPos;

/// Threshold for the share of covered content words — "almost entirely".
pub const DOUBT: f64 = 0.8;

/// A word: form and lemma in lowercase, and whether it is a content word.
#[derive(Clone, Debug, PartialEq)]
pub struct Lemma {
    pub form: String,
    pub lemma: String,
    pub content: bool,
}

/// Lemma source: in a run — `en::annotate::Annotator`, in tests — a stub.
pub trait Lemmas {
    fn lemmas(&self, text: &str) -> Vec<Lemma>;
}

pub fn content(u: UPos) -> bool {
    matches!(u, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ | UPos::ADV | UPos::NUM | UPos::INTJ | UPos::X)
}

impl Lemmas for Annotator {
    fn lemmas(&self, text: &str) -> Vec<Lemma> {
        let toks = self.tokenize(text);
        let forms: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
        if forms.is_empty() {
            return Vec::new();
        }
        self.annotate(&forms)
            .into_iter()
            .map(|w| Lemma { form: w.form.to_lowercase(), lemma: w.lemma.to_lowercase(), content: content(w.upos) })
            .collect()
    }
}

/// The `en` model: `QAGEN_EN_MODEL` or `data/en/models/ud-ewt-eslspok.bin`.
pub fn model_path() -> PathBuf {
    std::env::var("QAGEN_EN_MODEL").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../en/models/ud-ewt-eslspok.bin"))
    })
}

pub fn load() -> Result<Annotator> {
    Annotator::load(&model_path())
}

/// Share of the answer's content words covered by the sentence's lemmas or forms; `None` — no content words.
pub fn cover(answer: &[Lemma], sent: &[Lemma]) -> Option<f64> {
    let pool: HashSet<&str> = sent.iter().flat_map(|w| [w.lemma.as_str(), w.form.as_str()]).collect();
    let words: Vec<&Lemma> = answer.iter().filter(|w| w.content).collect();
    if words.is_empty() {
        return None;
    }
    let hit = words.iter().filter(|w| pool.contains(w.lemma.as_str()) || pool.contains(w.form.as_str())).count();
    Some(hit as f64 / words.len() as f64)
}

/// "Doubtful implicit": the anchor sentence (from 1) with the highest coverage of the answer, if it is ≥ `DOUBT`.
/// `sents` — texts of the paragraph's sentences, `anchors` — from 1.
pub fn doubt(l: &dyn Lemmas, sents: &[&str], anchors: &[usize], answer: &str) -> Option<(usize, f64)> {
    let a = l.lemmas(answer);
    let mut best: Option<(usize, f64)> = None;
    for &k in anchors {
        if let Some(c) = cover(&a, &l.lemmas(sents[k - 1]))
            && c >= DOUBT
            && best.is_none_or(|b| c > b.1)
        {
            best = Some((k, c));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::words;

    /// Stub: lemmas from a small table, function words from a list.
    struct Stub;

    impl Lemmas for Stub {
        fn lemmas(&self, text: &str) -> Vec<Lemma> {
            const LEMMA: [(&str, &str); 7] = [("tasted", "taste"), ("had", "have"), ("ate", "eat"), ("eating", "eat"), ("snakes", "snake"), ("king's", "king"), ("was", "be")];
            const FUNC: [&str; 12] = ["the", "a", "he", "him", "had", "because", "of", "was", "and", "to", "his", "it"];
            words(text)
                .into_iter()
                .map(|f| {
                    let lemma = LEMMA.iter().find(|x| x.0 == f).map_or(f.clone(), |x| x.1.to_string());
                    Lemma { content: !FUNC.contains(&f.as_str()), form: f, lemma }
                })
                .collect()
        }
    }

    const SENTS: [&str; 3] = ["The servant was curious.", "He had tasted the white snake of the king.", "Now he understood the birds."];

    /// Negative controls: a genuine inference; a paraphrase present only in a non-anchor sentence; coverage
    /// below the threshold (3 of 4, 2 of 3); an answer without content words.
    #[test]
    fn doubt_flags_restated_implicit_answers() {
        // paraphrase of sentence 2 with other forms — all 4 content words from the anchor's lemmas
        assert_eq!(doubt(&Stub, &SENTS, &[2], "because he tasted the king's white snake"), Some((2, 1.0)));
        // from several anchors — the best one
        assert_eq!(doubt(&Stub, &SENTS, &[1, 2], "tasted the white snake"), Some((2, 1.0)));
        // 4 of 5 — the threshold; 3 of 4 — below
        assert_eq!(doubt(&Stub, &SENTS, &[2], "he tasted the white snake of the king quickly"), Some((2, 0.8)));
        assert_eq!(doubt(&Stub, &SENTS, &[2], "he tasted the white snake quickly"), None);
        assert_eq!(doubt(&Stub, &SENTS, &[2], "he was brave and wanted power"), None);
        assert_eq!(doubt(&Stub, &SENTS, &[1, 3], "because he tasted the king's white snake"), None);
        assert_eq!(doubt(&Stub, &SENTS, &[2], "he tasted a magic snake"), None);
        assert_eq!(doubt(&Stub, &SENTS, &[2], "because he had"), None);
        assert_eq!(cover(&Stub.lemmas("the white cat"), &Stub.lemmas(SENTS[1])), Some(0.5));
    }

    /// The real `en` model (present on the box): lemmas of forms and the same mark; negative control — an inference.
    #[test]
    fn doubt_with_en_model() {
        let path = model_path();
        if !path.exists() {
            eprintln!("SKIP: no model {}", path.display());
            return;
        }
        let a = load().unwrap();
        let l = a.lemmas("He had tasted the white snakes.");
        assert_eq!(l.iter().map(|w| w.lemma.as_str()).collect::<Vec<_>>(), ["he", "have", "taste", "the", "white", "snake", "."]);
        assert_eq!(l.iter().filter(|w| w.content).count(), 3);
        let sents = ["The servant was curious.", "Eating the snake had given him the power of understanding the language of animals.", "He heard the sparrows talk."];
        assert!(doubt(&a, &sents, &[2], "because eating the snake gave him that power").is_some());
        assert_eq!(doubt(&a, &sents, &[2], "he was curious and brave"), None);
        assert_eq!(doubt(&a, &sents, &[1, 3], "because eating the snake gave him that power"), None);
    }
}
