//! What a text implies beyond what it says, deterministic and explained (docs/pragmatics-algorithms.md).
//!
//! - **Scalar implicatures** (the Quantity axiom of Tatu & Moldovan 2012; Horn scales): a weaker item of a scale
//!   implicates that the stronger ones do not hold — "some of the guests left" → not all of them. Not under
//!   negation, not when a stronger item of the same scale is in the sentence, not when the speaker cancels it ("if
//!   not all", "in fact all", "or even").
//! - **Presuppositions** of triggers (Cianflone et al. 2018 for the adverbs): again, still (the event or state held
//!   before), also, too (someone or something else did the same), stop, quit (it was going on), continue, keep (it
//!   was going on), know, realize, regret, forget that (the complement is true). A presupposition is *licensed* when
//!   the earlier text (up to 50 tokens back) has the same predicate; an unlicensed one is new information the hearer
//!   must accommodate — the explanation says which.
//! - **Quality and Manner** (more maxim axioms of Tatu & Moldovan 2012): a question implicates that the speaker does
//!   not know the questioned part (a wh-question: what the wh-word stands for; a yes/no question with an inverted
//!   auxiliary: whether the rest holds); an exclamation implicates a strong feeling ("be truthful" — the speaker shows
//!   what he really knows and feels). "Be orderly": two event verbs coordinated by "and" are told in the order they
//!   happened — "She opened the door and walked in" → «opened» before «walked». Not with a stative verb (be, have,
//!   know, like, love, own: states do not follow each other) and not when a word that reorders or overlaps the events
//!   is in the sentence (after, before, while, meanwhile, simultaneously; "then" anywhere but on the second verb).

use en::annotate::Word;
use en::gram::Rel;

/// Horn scales, weakest first. Lemmas.
pub const SCALES: &[&[&str]] = &[
    &["some", "many", "most", "all"],
    &["some", "most", "all"],
    &["or", "and"],
    &["possible", "likely", "certain"],
    &["may", "must"],
    &["might", "must"],
    &["possibly", "probably", "certainly"],
    &["sometimes", "often", "always"],
    &["occasionally", "usually", "always"],
    &["warm", "hot"],
    &["cool", "cold"],
    &["good", "excellent"],
    &["like", "love"],
    &["try", "manage"],
    &["start", "finish"],
    &["believe", "know"],
    &["few", "none"],
];

/// One inference with its trigger and why.
#[derive(Clone, Debug, PartialEq)]
pub struct Inference {
    pub kind: &'static str,
    /// the word that triggers it
    pub trigger: String,
    pub content: String,
    pub why: String,
    /// presuppositions: found in the earlier text (true) or to be accommodated (false); implicatures: true
    pub licensed: bool,
}

pub(crate) fn negated(words: &[Word], i: usize) -> bool {
    // the word or its head has a negation child; or a negation stands right before it
    let h = words[i].head;
    words.iter().enumerate().any(|(j, w)| {
        let neg = matches!(w.lemma.to_lowercase().as_str(), "not" | "n't" | "never" | "no");
        neg && (w.head == i + 1 || (h > 0 && w.head == h) || j + 1 == i)
    })
}

/// Scalar implicatures of one sentence.
pub fn implicatures(words: &[Word]) -> Vec<Inference> {
    let lemmas: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    let low: String = words.iter().map(|w| w.form.to_lowercase()).collect::<Vec<_>>().join(" ");
    let cancelled = ["if not all", "in fact all", "or even", "if not", "indeed all", "and maybe all"].iter().any(|c| low.contains(c));
    let mut out: Vec<Inference> = Vec::new();
    for (i, l) in lemmas.iter().enumerate() {
        for scale in SCALES {
            let Some(pos) = scale.iter().position(|x| x == l) else { continue };
            if pos + 1 == scale.len() {
                continue;
            }
            let stronger = &scale[pos + 1..];
            if stronger.iter().any(|s| lemmas.contains(&s.to_string())) || negated(words, i) || cancelled {
                continue;
            }
            // "or" only between two conjuncts; a scale item must be the word itself, not a part of a name
            if l == "or" && words[i].rel != Rel::Cc {
                continue;
            }
            if out.iter().any(|x| x.trigger == words[i].form && x.content.contains(stronger[stronger.len() - 1])) {
                continue;
            }
            let strongest = stronger[stronger.len() - 1];
            out.push(Inference {
                kind: "scalar implicature",
                trigger: words[i].form.clone(),
                content: format!("not «{strongest}»{}", if stronger.len() > 1 { format!(" (nor «{}»)", stronger[..stronger.len() - 1].join("», «")) } else { String::new() }),
                why: format!("«{}» is weaker than «{}» on the scale {}: a speaker who could say the stronger would have (Quantity)", words[i].form, strongest, scale.join(" < ")),
                licensed: true,
            });
        }
    }
    out
}

/// Presuppositions of one sentence, licensed against the earlier sentences of the text.
pub fn presuppositions(words: &[Word], earlier: &[Vec<Word>]) -> Vec<Inference> {
    let back: Vec<&Word> = earlier.iter().flatten().collect();
    let back: Vec<&Word> = back.iter().rev().take(50).rev().copied().collect();
    let seen = |lemma: &str| back.iter().any(|w| w.lemma.to_lowercase() == lemma);
    let mut out = Vec::new();
    for (i, w) in words.iter().enumerate() {
        let l = w.lemma.to_lowercase();
        let head = if w.head > 0 { Some(&words[w.head - 1]) } else { None };
        let pred = head.map(|h| h.lemma.to_lowercase()).unwrap_or_default();
        let (content, key): (String, String) = match l.as_str() {
            "again" if !pred.is_empty() => (format!("«{pred}» happened before"), pred.clone()),
            "still" if !pred.is_empty() => (format!("«{pred}» held before"), pred.clone()),
            "also" | "too" if !pred.is_empty() && !(l == "too" && head.is_some_and(|h| matches!(h.upos, en::gram::UPos::ADJ | en::gram::UPos::ADV))) => (format!("someone or something else «{pred}» too"), pred.clone()),
            "stop" | "quit" | "continue" | "keep" if w.upos == en::gram::UPos::VERB => {
                let Some((_, c)) = words.iter().enumerate().find(|(_, x)| x.head == i + 1 && matches!(x.rel, Rel::Xcomp | Rel::Ccomp | Rel::Obj)) else { continue };
                let v = c.lemma.to_lowercase();
                (format!("«{v}» was going on before"), v)
            }
            "know" | "realize" | "realise" | "regret" | "forget" if w.upos == en::gram::UPos::VERB => {
                // the complement clause: ccomp, or an xcomp that has its own subject or a "that" (a parser slip)
                let clause_of = |j: usize| words.iter().any(|y| y.head == j + 1 && matches!(y.rel, Rel::Nsubj | Rel::Mark));
                let Some((j, _)) = words.iter().enumerate().find(|(j, x)| x.head == i + 1 && (x.rel == Rel::Ccomp || (x.rel == Rel::Xcomp && clause_of(*j)))) else { continue };
                // the whole subtree of the clause head, without its "that"
                let under = |k: usize| {
                    let mut x = k;
                    for _ in 0..words.len() {
                        if x == j {
                            return true;
                        }
                        match words[x].head {
                            0 => return false,
                            h => x = h - 1,
                        }
                    }
                    false
                };
                let clause: Vec<&str> = words.iter().enumerate().filter(|(k, x)| under(*k) && !(x.rel == Rel::Mark && x.head == j + 1)).map(|(_, x)| x.form.as_str()).collect();
                (format!("it is true that «{}»", clause.join(" ")), words[j].lemma.to_lowercase())
            }
            _ => continue,
        };
        let licensed = seen(&key);
        out.push(Inference {
            kind: "presupposition",
            trigger: w.form.clone(),
            why: if licensed { format!("«{}» needs it, and the earlier text has «{key}»", w.form) } else { format!("«{}» needs it, but the earlier text has no «{key}»: the hearer must accommodate it as new", w.form) },
            content,
            licensed,
        });
    }
    out
}

/// Wh-words that open a content question.
pub const WH: &[&str] = &["what", "who", "whom", "whose", "which", "where", "when", "why", "how"];
/// Auxiliaries that open a yes/no question by inversion.
pub const AUX_INV: &[&str] = &["do", "does", "did", "is", "are", "was", "were", "am", "can", "could", "will", "would", "shall", "should", "may", "might", "must", "have", "has", "had"];
/// Stative verbs: states, not events — no order between them (Manner).
pub const STATIVE: &[&str] = &["be", "have", "know", "like", "love", "own", "believe", "want", "need", "seem", "belong", "contain"];
/// Words that reorder or overlap coordinated events, so the told order is not the order of events.
pub const REORDER: &[&str] = &["after", "before", "while", "meanwhile", "simultaneously", "afterwards", "beforehand", "previously", "earlier"];

/// Quality and Manner implicatures of one sentence (`sentence` is its text, for the final punctuation).
pub fn maxims(words: &[Word], sentence: &str) -> Vec<Inference> {
    let mut out = Vec::new();
    let end = sentence.trim_end();
    let body: Vec<&Word> = words.iter().filter(|w| w.upos != en::gram::UPos::PUNCT).collect();
    let first = body.first().map(|w| w.form.to_lowercase()).unwrap_or_default();
    // Quality: a question — the speaker does not know the questioned part
    if end.ends_with('?') || end.ends_with("?!") || end.ends_with("!?") {
        let rest = |from: usize| body[from.min(body.len())..].iter().map(|w| w.form.as_str()).collect::<Vec<_>>().join(" ");
        let (content, why) = if WH.contains(&first.as_str()) {
            (format!("the speaker does not know {}", rest(0).to_lowercase()), format!("a question opened by «{first}» asks for what «{first}» stands for: a truthful speaker asks only what he does not know (Quality)"))
        } else if AUX_INV.contains(&first.as_str()) {
            (format!("the speaker does not know whether «{}»", rest(1)), format!("a yes/no question with the inverted «{first}»: a truthful speaker asks only what he does not know (Quality)"))
        } else {
            (format!("the speaker does not know whether «{}»", rest(0)), "a declarative sentence with a question mark asks to confirm it: the speaker is not sure (Quality)".to_string())
        };
        out.push(Inference { kind: "quality implicature", trigger: "?".into(), content, why, licensed: true });
    }
    // Quality: an exclamation — a strong feeling
    if end.ends_with('!') || end.ends_with("!?") {
        out.push(Inference {
            kind: "quality implicature",
            trigger: "!".into(),
            content: "the speaker feels strongly about it".into(),
            why: "the exclamation mark shows the speaker's real attitude, not only the content (Quality)".into(),
            licensed: true,
        });
    }
    // Manner: "e1 and e2" — e1 happened before e2
    let lemmas: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    for (i, w) in words.iter().enumerate() {
        if w.rel != Rel::Conj || w.upos != en::gram::UPos::VERB || w.head == 0 {
            continue;
        }
        let h = w.head - 1;
        let e1 = &words[h];
        if e1.upos != en::gram::UPos::VERB {
            continue;
        }
        if !words.iter().any(|x| x.head == i + 1 && x.rel == Rel::Cc && x.lemma.to_lowercase() == "and") {
            continue;
        }
        if STATIVE.contains(&lemmas[h].as_str()) || STATIVE.contains(&lemmas[i].as_str()) {
            continue;
        }
        let reorder = lemmas.iter().enumerate().find(|(j, l)| REORDER.contains(&l.as_str()) || (l.as_str() == "then" && words[*j].head != i + 1));
        if reorder.is_some() {
            continue;
        }
        out.push(Inference {
            kind: "manner implicature",
            trigger: "and".into(),
            content: format!("«{}» happened before «{}»", e1.form, w.form),
            why: format!("two events «{}» and «{}» joined by «and» are told in the order they happened (Manner: be orderly); no stative verb, no reordering word", e1.form, w.form),
            licensed: true,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::{Feats, Tag, UPos};

    fn w(form: &str, lemma: &str, upos: UPos, head: usize, rel: Rel) -> Word {
        Word { form: form.into(), lemma: lemma.into(), upos, tag: Tag::NN, feats: Feats::default(), head, rel }
    }

    fn some_guests_left() -> Vec<Word> {
        vec![w("Some", "some", UPos::DET, 2, Rel::Det), w("guests", "guest", UPos::NOUN, 3, Rel::Nsubj), w("left", "leave", UPos::VERB, 0, Rel::Root)]
    }

    #[test]
    fn scalar_implicature_and_its_blocks() {
        let i = implicatures(&some_guests_left());
        assert_eq!(i.len(), 1);
        assert!(i[0].content.starts_with("not «all»"));
        // negative controls: a stronger item present, negation, cancellation
        let mut all = some_guests_left();
        all.push(w("all", "all", UPos::DET, 2, Rel::Det));
        assert!(implicatures(&all).is_empty());
        let not_warm = vec![w("It", "it", UPos::PRON, 3, Rel::Nsubj), w("not", "not", UPos::PART, 3, Rel::Advmod), w("warm", "warm", UPos::ADJ, 0, Rel::Root)];
        assert!(implicatures(&not_warm).is_empty());
    }

    #[test]
    fn presuppositions_licensed_or_new() {
        let ran = vec![w("The", "the", UPos::DET, 2, Rel::Det), w("hare", "hare", UPos::NOUN, 3, Rel::Nsubj), w("ran", "run", UPos::VERB, 0, Rel::Root)];
        let too = vec![w("The", "the", UPos::DET, 2, Rel::Det), w("tortoise", "tortoise", UPos::NOUN, 3, Rel::Nsubj), w("ran", "run", UPos::VERB, 0, Rel::Root), w("too", "too", UPos::ADV, 3, Rel::Advmod)];
        let p = presuppositions(&too, &[ran.clone()]);
        assert_eq!(p.len(), 1);
        assert!(p[0].licensed);
        let p0 = presuppositions(&too, &[]);
        assert!(!p0[0].licensed, "without the earlier sentence it is new information");
        // "too heavy": the excess sense is no trigger
        let heavy = vec![w("too", "too", UPos::ADV, 2, Rel::Advmod), w("heavy", "heavy", UPos::ADJ, 0, Rel::Root)];
        assert!(presuppositions(&heavy, &[]).is_empty());
    }

    fn fox_eat_grapes() -> Vec<Word> {
        vec![
            w("Did", "do", UPos::AUX, 3, Rel::Aux),
            w("the", "the", UPos::DET, 2, Rel::Det),
            w("fox", "fox", UPos::NOUN, 3, Rel::Nsubj),
            w("eat", "eat", UPos::VERB, 0, Rel::Root),
            w("the", "the", UPos::DET, 5, Rel::Det),
            w("grapes", "grape", UPos::NOUN, 3, Rel::Obj),
            w("?", "?", UPos::PUNCT, 3, Rel::Punct),
        ]
    }

    #[test]
    fn quality_question_and_exclamation() {
        let q = maxims(&fox_eat_grapes(), "Did the fox eat the grapes?");
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].content, "the speaker does not know whether «the fox eat the grapes»");
        let who = vec![w("Who", "who", UPos::PRON, 2, Rel::Nsubj), w("ate", "eat", UPos::VERB, 0, Rel::Root), w("?", "?", UPos::PUNCT, 2, Rel::Punct)];
        assert!(maxims(&who, "Who ate?")[0].content.contains("who ate"));
        let mut ex = some_guests_left();
        ex.push(w("!", "!", UPos::PUNCT, 3, Rel::Punct));
        let e = maxims(&ex, "Some guests left!");
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].content, "the speaker feels strongly about it");
        // negative control: a plain statement implicates neither
        assert!(maxims(&some_guests_left(), "Some guests left.").is_empty());
    }

    fn opened_and_walked(second: &str, extra: Option<Word>) -> Vec<Word> {
        let mut s = vec![
            w("She", "she", UPos::PRON, 2, Rel::Nsubj),
            w("opened", "open", UPos::VERB, 0, Rel::Root),
            w("the", "the", UPos::DET, 4, Rel::Det),
            w("door", "door", UPos::NOUN, 2, Rel::Obj),
            w("and", "and", UPos::CCONJ, 6, Rel::Cc),
            w(second, second, UPos::VERB, 2, Rel::Conj),
        ];
        s.extend(extra);
        s
    }

    #[test]
    fn manner_order_and_its_blocks() {
        let m = maxims(&opened_and_walked("walked", None), "She opened the door and walked.");
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].content, "«opened» happened before «walked»");
        // "then" on the second verb keeps the order
        assert_eq!(maxims(&opened_and_walked("walked", Some(w("then", "then", UPos::ADV, 6, Rel::Advmod))), "").len(), 1);
        // negative controls: a reordering word, "then" elsewhere, a stative verb, "or" instead of "and"
        assert!(maxims(&opened_and_walked("walked", Some(w("meanwhile", "meanwhile", UPos::ADV, 6, Rel::Advmod))), "").is_empty());
        assert!(maxims(&opened_and_walked("walked", Some(w("then", "then", UPos::ADV, 2, Rel::Advmod))), "").is_empty());
        assert!(maxims(&opened_and_walked("own", None), "").is_empty());
        let mut or = opened_and_walked("walked", None);
        or[4] = w("or", "or", UPos::CCONJ, 6, Rel::Cc);
        assert!(maxims(&or, "").is_empty());
    }
}
