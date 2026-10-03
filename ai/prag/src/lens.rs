//! The pragmatics lens over expressions: what the words really mean, not literally. The expression base is
//! level-1 knowledge (`global::expressions_in`, English Wiktionary, CC BY-SA): idioms, proverbs, phrases, phrasal
//! and verb phrases with their meanings, and register labels (slang, formal, humorous, euphemistic, archaic …)
//! that switch on a domain — slang, formal language, humour, hidden meaning.
//!
//! Tried as classifier features (2026-10-02: expression kind and domains as two slots of `feats`): no clear gain
//! on the silver fields (cross-validated macro-F1: act 33.3 → 35.7, indirect 59.3 → 60.8, form 69.5 → 65.5 —
//! within noise); the fields describe form and act, not meaning. The lens feeds "what was meant" instead.

use en::annotate::Word;

/// A found expression with its real meaning.
#[derive(Clone, Debug)]
pub struct Reading {
    /// the words of the sentence it covers
    pub text: String,
    pub phrase: &'static str,
    pub kind: &'static str,
    pub meaning: &'static str,
    pub register: &'static str,
}

/// The lens of a sentence: expressions with meanings, and the domains to switch on.
#[derive(Clone, Debug, Default)]
pub struct Lens {
    pub readings: Vec<Reading>,
    pub domains: Vec<&'static str>,
}

/// Expressions and domains of an annotated sentence (lemmas from `en`).
pub fn lens(words: &[Word]) -> Lens {
    let lemmas: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    let forms: Vec<String> = words.iter().map(|w| w.form.to_lowercase()).collect();
    let refs: Vec<&str> = lemmas.iter().map(String::as_str).collect();
    let frefs: Vec<&str> = forms.iter().map(String::as_str).collect();
    let mut out = Lens::default();
    for m in global::expressions_in_sentence(&frefs, &refs) {
        out.domains.extend(m.domains.iter().copied());
        out.readings.push(Reading {
            text: words[m.start..m.end].iter().map(|w| w.form.as_str()).collect::<Vec<_>>().join(" "),
            phrase: m.entry.phrase,
            kind: m.entry.kind,
            meaning: m.entry.meaning,
            register: m.entry.register,
        });
    }
    out.domains.sort();
    out.domains.dedup();
    out
}

/// "What was meant" from the lens: the meanings of the multiword expressions, `-` if none.
pub fn means(l: &Lens) -> String {
    let ms: Vec<String> = l.readings.iter().filter(|r| r.phrase.contains(' ')).map(|r| format!("{} = {}", r.text, r.meaning.trim_end_matches('.'))).collect();
    if ms.is_empty() { "-".into() } else { ms.join("; ") }
}
