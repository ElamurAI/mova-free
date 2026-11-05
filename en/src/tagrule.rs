//! Context tag rules of Modern English, given to the tagger before it tags (like `crate::emode::force`): a word whose
//! class the treebanks fix by a short context that the averaged perceptron does not weigh enough.
//!
//! R1 «so» of purpose (seed `en/seeds/verbal/so-purpose.md`): «so» right after a word (no comma, no coordinator before
//! it), then a pronoun subject or a determiner and a noun, then *can* or *could* not negated, introduces a clause of
//! purpose: *I left the key under the mat **so** she can get it*. UD EWT tags it SCONJ (IN) with `mark`; the tagger
//! gave RB and the parser `advmod`. In EWT train 2.18 the context fires 22 times: 18 SCONJ/mark, 4 ADV/advmod.
//! A comma before «so» (*it rained, so we stayed home*), a modal of result (*will, would, may*) or a negation leave
//! the word to the tagger.

use crate::gram::Tag;

/// Subject pronouns (lower case).
const SUBJECT: [&str; 7] = ["i", "you", "he", "she", "it", "we", "they"];
/// Determiners and possessives before a one-word noun subject.
const DET: [&str; 14] = ["the", "a", "an", "this", "that", "these", "those", "my", "your", "his", "her", "its", "our", "their"];

fn low(words: &[&str], k: usize) -> String {
    words.get(k).map(|w| w.to_lowercase()).unwrap_or_default()
}

fn punct_or_cc(w: &str) -> bool {
    w.chars().all(|c| !c.is_alphanumeric()) || matches!(w.to_lowercase().as_str(), "and" | "or" | "but" | "nor" | "yet")
}

/// R1: «so» at `i` introduces a purpose clause; the reason, or `None`.
pub fn so_purpose(words: &[&str], i: usize) -> Option<&'static str> {
    if i == 0 || low(words, i) != "so" || punct_or_cc(words[i - 1]) {
        return None;
    }
    let next = low(words, i + 1);
    if next == "that" {
        // «so that» is a fixed conjunction the tagger already knows
        return None;
    }
    let modal_at = if SUBJECT.contains(&next.as_str()) {
        i + 2
    } else if DET.contains(&next.as_str()) && words.get(i + 2).is_some_and(|w| w.chars().all(char::is_alphabetic)) {
        i + 3
    } else {
        return None;
    };
    let m = low(words, modal_at);
    let after = low(words, modal_at + 1);
    (matches!(m.as_str(), "can" | "could") && !matches!(after.as_str(), "n't" | "not" | "nt" | "n’t"))
        .then_some("«so» + subject + can/could after a word: a clause of purpose (SCONJ, mark)")
}

/// The tag each word must have where a rule knows it.
pub fn force(words: &[&str]) -> Vec<Option<Tag>> {
    (0..words.len()).map(|i| so_purpose(words, i).map(|_| Tag::IN)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str, i: usize) -> Option<Tag> {
        let w: Vec<&str> = s.split(' ').collect();
        force(&w)[i]
    }

    #[test]
    fn so_of_purpose() {
        assert_eq!(at("I left the key under the mat so she can get it .", 7), Some(Tag::IN));
        assert_eq!(at("I have a couple of questions so I can wrap up", 6), Some(Tag::IN));
        assert_eq!(at("place it in the paper so the bill could not be seen", 5), None, "negated");
        assert_eq!(at("get a box so the fry can hatch", 3), Some(Tag::IN));
        // negative controls: result, degree, a comma or a coordinator before, a modal of result, the start
        assert_eq!(at("It was raining , so we can stay home", 4), None);
        assert_eq!(at("and so we can go", 1), None);
        assert_eq!(at("She was so tired", 2), None);
        assert_eq!(at("I was busy so I would wait", 3), None);
        assert_eq!(at("So we can go", 0), None);
        assert_eq!(at("I think so", 2), None);
        assert_eq!(at("He saved so that he could buy it", 2), None, "«so that» is the tagger's");
    }
}
