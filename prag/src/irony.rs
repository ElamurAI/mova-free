//! Sarcasm and irony candidates by incongruity, deterministic and explained (docs/pragmatics-algorithms.md,
//! "Sarcasm by incongruity").
//!
//! Method (Joshi, Sharma & Bhattacharyya 2015; the negative situations after Riloff et al. 2013): sarcasm says
//! something positive about something bad. Three groups of integer features:
//!
//! - **Explicit incongruity**: polar words of both signs from a small built-in lexicon of content-word lemmas
//!   ([`POSITIVE`], [`NEGATIVE`]; a negated polar word flips its sign; a negative situation counts as negative). The
//!   counters of Joshi et al.: positive and negative words, sentiment flips in word order, the longest same-polarity
//!   run, overall polarity. It fires when there is at least one flip and a positive word governs or precedes a
//!   negative one ("I love being ignored", "Great, my tooth hurts") — unless a contrastive connector (but, although,
//!   though, however, yet) stands between them: "the food was good but the service was bad" is a sincere mixed
//!   opinion.
//! - **Implicit incongruity** (Riloff et al. 2013): a positive verb (love, enjoy, adore, like, appreciate, "can't
//!   wait") whose complement — its dependency subtree after it — holds a negative situation from [`SITUATIONS`]
//!   ("I love waiting in line for hours"). A situation that is ended ("the rain stopped") or absent ("no traffic") is
//!   not negative. Riloff bootstraps the situations from tweets; here they are a hand-written seed list.
//! - **Pragmatic cues**: emoticons, laughter, "!!" / "?!", ALL-CAPS words, "yeah right", "oh great", "just what I
//!   needed".
//!
//! Score (integers): explicit incongruity 2, implicit incongruity 3, each pragmatic cue kind 1. **Threshold:** a
//! candidate needs at least one incongruity (cues alone are only emphasis — "lol that was fun" is not irony) and a
//! score of at least [`THRESHOLD`] = 2. The incongruent pair (positive part, negative part) is the explanation.
//!
//! In Mova this meets the absurdity matrix's humor trigger in `world` (`world/src/domains.rs`, absurdity that
//! survives the domain → humor): absurdity catches odd doers and objects, incongruity catches praise of the bad; both
//! switch on the humor/hidden-meaning reading. `world` is not changed here.

use en::annotate::Word;
use en::gram::UPos;

use crate::infer::negated;

/// Positive content-word lemmas (and a few frequent inflected forms the lemmatizer may keep).
pub const POSITIVE: &[&str] = &[
    "love", "like", "enjoy", "adore", "appreciate", "great", "good", "wonderful", "fantastic", "amazing", "awesome",
    "excellent", "perfect", "nice", "lovely", "brilliant", "happy", "glad", "delighted", "thrilled", "excited",
    "exciting", "fun", "beautiful", "best", "better", "fabulous", "superb", "terrific", "marvelous", "splendid",
    "pleasant", "delightful", "charming", "thank", "thanks", "grateful", "thankful", "win", "success", "successful",
    "joy", "enjoyable", "favorite", "favourite", "kind", "sweet", "smart", "clever", "impressive", "incredible",
    "outstanding", "glorious", "magnificent", "pleased", "cheerful", "lucky", "fortunate", "admire", "relaxing",
    "comfortable", "peaceful", "gorgeous", "stunning", "adorable",
];

/// Negative content-word lemmas.
pub const NEGATIVE: &[&str] = &[
    "hate", "dislike", "ignore", "hurt", "pain", "painful", "bad", "terrible", "awful", "horrible", "worst", "worse",
    "sad", "angry", "annoy", "annoying", "annoyed", "boring", "bored", "tired", "sick", "ill", "broken", "break",
    "fail", "failure", "lose", "lost", "problem", "trouble", "disaster", "mess", "ugly", "stupid", "rude", "cry",
    "suffer", "miserable", "upset", "frustrate", "frustrating", "stress", "stressful", "nightmare", "crash", "wrong",
    "late", "slow", "noisy", "dirty", "cancel", "reject", "steal", "stolen", "die", "dead", "sore", "ache", "poor",
    "lonely", "scared", "afraid", "worry", "disappoint", "disappointing", "useless", "waste", "blame", "insult",
];

/// Positive verbs that take a situation as their complement (implicit incongruity); "can't wait" is matched apart.
pub const POSITIVE_VERBS: &[&str] = &["love", "enjoy", "adore", "like", "appreciate"];

/// Negative situations as lemma sequences (seed list after Riloff et al. 2013).
pub const SITUATIONS: &[&[&str]] = &[
    &["wait", "in", "line"], &["wait", "in", "queue"], &["stand", "in", "line"], &["be", "on", "hold"], &["wait", "on", "hold"],
    &["be", "ignore"], &["get", "ignore"], &["get", "up", "early"], &["wake", "up", "early"], &["work", "late"],
    &["work", "overtime"], &["work", "on", "weekend"], &["traffic"], &["homework"], &["monday"], &["rain"],
    &["delay"], &["toothache"], &["headache"], &["cleaning"], &["clean", "the", "house"], &["tax"], &["taxes"],
    &["meeting"], &["exam"], &["dentist"], &["paperwork"], &["commute"], &["laundry"], &["do", "the", "dish"],
    &["flat", "tire"], &["power", "outage"], &["deadline"], &["snow"], &["get", "stuck"], &["spam"], &["jury", "duty"],
    &["hangover"], &["insomnia"], &["bill"], &["chore"], &["be", "sick"], &["get", "sick"], &["construction", "noise"],
    &["alarm", "clock"],
];

/// Emoticons and emoji that mark irony.
pub const EMOTICONS: &[&str] = &[":)", ";)", ":D", ":P", ":-)", ";-)", "🙃", "😒", "🙄", "😏", "😂"];
/// Laughter tokens.
pub const LAUGHTER: &[&str] = &["lol", "haha", "hahaha", "hehe", "lmao", "rofl"];
/// Fixed ironic phrases.
pub const PHRASES: &[(&str, &str)] = &[("yeah right", "yeah right"), ("oh great", "oh great"), ("just what i needed", "just what I needed")];
/// Contrastive connectors: between the pair they make a sincere mixed opinion.
pub const CONTRAST: &[&str] = &["but", "although", "though", "however", "yet"];
/// Verbs and words that end or deny a situation.
pub const ENDED: &[&str] = &["stop", "end", "finish", "cease", "over", "no", "without"];

/// Minimal score of a candidate.
pub const THRESHOLD: i32 = 2;

/// A sarcasm / irony candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct Irony {
    pub score: i32,
    pub features: Vec<&'static str>,
    /// the incongruent pair: (positive part, negative part)
    pub pair: (String, String),
    pub positive: usize,
    pub negative: usize,
    pub flips: usize,
    pub longest_run: usize,
    /// positive minus negative
    pub polarity: i32,
    pub why: String,
}

fn under(words: &[Word], k: usize, top: usize) -> bool {
    let mut x = k;
    for _ in 0..=words.len() {
        if x == top {
            return true;
        }
        match words[x].head {
            0 => return false,
            h => x = h - 1,
        }
    }
    false
}

/// The negative situations of the sentence: (first index, last index).
fn situations(words: &[Word]) -> Vec<(usize, usize)> {
    let lemmas: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    let forms: Vec<String> = words.iter().map(|w| w.form.to_lowercase()).collect();
    let mut out: Vec<(usize, usize)> = Vec::new();
    for i in 0..words.len() {
        for s in SITUATIONS {
            let n = s.len();
            if i + n > words.len() || !(0..n).all(|k| lemmas[i + k] == s[k] || forms[i + k] == s[k]) {
                continue;
            }
            // ended or denied: the head word or a word right before is in ENDED
            let last = i + n - 1;
            let ended = (i..=last).any(|k| words[k].head > 0 && ENDED.contains(&lemmas[words[k].head - 1].as_str()))
                || (i > 0 && ENDED.contains(&lemmas[i - 1].as_str()))
                || words.iter().enumerate().any(|(j, w)| ENDED.contains(&lemmas[j].as_str()) && (i..=last).any(|k| w.head == k + 1));
            if !ended && !out.iter().any(|&(a, b)| a <= last && i <= b) {
                out.push((i, last));
            }
        }
    }
    out
}

/// "can't wait": the index of "wait" after can + not / n't, or "cannot".
fn cant_wait(words: &[Word], i: usize) -> bool {
    let f = |k: usize| words[k].form.to_lowercase();
    words[i].lemma.to_lowercase() == "wait"
        && ((i >= 2 && matches!(f(i - 2).as_str(), "ca" | "can") && matches!(f(i - 1).as_str(), "n't" | "not")) || (i >= 1 && matches!(f(i - 1).as_str(), "cannot" | "can't")))
}

/// Pragmatic cues of the raw text.
fn cues(text: &str) -> Vec<&'static str> {
    let mut out = Vec::new();
    let low = text.to_lowercase();
    if EMOTICONS.iter().any(|e| text.contains(e)) {
        out.push("emoticon");
    }
    let toks: Vec<&str> = text.split(|c: char| !c.is_alphanumeric() && c != '\'').filter(|t| !t.is_empty()).collect();
    if toks.iter().any(|t| LAUGHTER.contains(&t.to_lowercase().as_str()) || t.to_lowercase().starts_with("haha")) {
        out.push("laughter");
    }
    if text.contains("!!") {
        out.push("!!");
    }
    if text.contains("?!") || text.contains("!?") {
        out.push("?!");
    }
    if toks.iter().any(|t| t.chars().count() >= 3 && t.chars().all(|c| c.is_alphabetic() && c.is_uppercase())) {
        out.push("all caps");
    }
    for (p, name) in PHRASES {
        if low.contains(p) {
            out.push(*name);
        }
    }
    out
}

/// Sarcasm / irony candidate of one sentence (`text` is its raw text, for the cues), or None.
pub fn irony(words: &[Word], text: &str) -> Option<Irony> {
    let lemmas: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    let sits = situations(words);
    let in_sit = |k: usize| sits.iter().any(|&(a, b)| a <= k && k <= b);
    // polar sequence in word order: (index, +1/-1); a situation is one negative item at its first word
    let mut polar: Vec<(usize, i32)> = Vec::new();
    for (k, l) in lemmas.iter().enumerate() {
        if let Some(&(a, _)) = sits.iter().find(|&&(a, _)| a == k) {
            polar.push((a, -1));
            continue;
        }
        if in_sit(k) || words[k].upos == UPos::ADP || words[k].upos == UPos::SCONJ {
            continue; // "like" as a preposition is no sentiment
        }
        let base = if cant_wait(words, k) {
            1
        } else if POSITIVE.contains(&l.as_str()) || POSITIVE.contains(&words[k].form.to_lowercase().as_str()) {
            1
        } else if NEGATIVE.contains(&l.as_str()) || NEGATIVE.contains(&words[k].form.to_lowercase().as_str()) {
            -1
        } else {
            continue;
        };
        let sign = if cant_wait(words, k) || !negated(words, k) { base } else { -base };
        polar.push((k, sign));
    }
    let positive = polar.iter().filter(|p| p.1 > 0).count();
    let negative = polar.len() - positive;
    let flips = polar.windows(2).filter(|w| w[0].1 != w[1].1).count();
    let mut longest_run = 0;
    let mut run = 0;
    for (n, p) in polar.iter().enumerate() {
        run = if n > 0 && polar[n - 1].1 == p.1 { run + 1 } else { 1 };
        longest_run = longest_run.max(run);
    }
    let polarity = positive as i32 - negative as i32;
    let span = |k: usize| sits.iter().find(|&&(a, _)| a == k).map(|&(a, b)| words[a..=b].iter().map(|w| w.form.as_str()).collect::<Vec<_>>().join(" ")).unwrap_or_else(|| words[k].form.clone());
    let pos_form = |k: usize| if cant_wait(words, k) { "can't wait".to_string() } else { words[k].form.clone() };
    let mut features: Vec<&'static str> = Vec::new();
    let mut score = 0;
    let mut pair: Option<(String, String)> = None;
    let mut why: Vec<String> = Vec::new();
    // implicit: a positive verb with a negative situation in its complement
    for &(p, s) in polar.iter() {
        if s <= 0 || !(POSITIVE_VERBS.contains(&lemmas[p].as_str()) || cant_wait(words, p)) || words[p].upos == UPos::ADJ {
            continue;
        }
        if let Some(&(a, _)) = sits.iter().find(|&&(a, b)| a > p && (a..=b).any(|k| under(words, k, p))) {
            features.push("implicit incongruity");
            score += 3;
            pair = Some((pos_form(p), span(a)));
            why.push(format!("the positive verb «{}» takes the negative situation «{}» as its complement (implicit incongruity, +3)", pos_form(p), span(a)));
            break;
        }
    }
    // explicit: a flip with a positive word governing or preceding a negative one, no contrast between
    if flips > 0 {
        let mut found = None;
        'outer: for &(p, sp) in &polar {
            if sp <= 0 {
                continue;
            }
            for &(n, sn) in &polar {
                if sn >= 0 || !(p < n || under(words, n, p)) {
                    continue;
                }
                let (lo, hi) = (p.min(n), p.max(n));
                if (lo + 1..hi).any(|k| CONTRAST.contains(&lemmas[k].as_str())) {
                    continue;
                }
                found = Some((p, n));
                break 'outer;
            }
        }
        if let Some((p, n)) = found {
            features.push("explicit incongruity");
            score += 2;
            let how = if under(words, n, p) { "governs" } else { "precedes" };
            if pair.is_none() {
                pair = Some((pos_form(p), span(n)));
            }
            why.push(format!("the positive «{}» {how} the negative «{}» ({positive} positive, {negative} negative, {flips} flip(s), longest run {longest_run}; explicit incongruity, +2)", pos_form(p), span(n)));
        }
    }
    let pair = pair?;
    let c = cues(text);
    if !c.is_empty() {
        score += c.len() as i32;
        why.push(format!("pragmatic cues: {} (+{})", c.join(", "), c.len()));
        features.extend(c);
    }
    if score < THRESHOLD {
        return None;
    }
    Some(Irony { score, features, pair, positive, negative, flips, longest_run, polarity, why: format!("{} — score {score} ≥ {THRESHOLD}", why.join("; ")) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::{Feats, Rel, Tag};

    fn w(form: &str, lemma: &str, upos: UPos, head: usize, rel: Rel) -> Word {
        Word { form: form.into(), lemma: lemma.into(), upos, tag: Tag::NN, feats: Feats::default(), head, rel }
    }
    use Rel::*;
    use UPos::*;

    /// A hand-made parse: (form, lemma, upos, head, rel) per word.
    fn s(spec: &[(&str, &str, UPos, usize, Rel)]) -> Vec<Word> {
        spec.iter().map(|&(f, l, u, h, r)| w(f, l, u, h, r)).collect()
    }

    #[test]
    fn sarcastic_sentences() {
        let ignored = s(&[("I", "I", PRON, 2, Nsubj), ("love", "love", VERB, 0, Root), ("being", "be", AUX, 4, AuxPass), ("ignored", "ignore", VERB, 2, Xcomp), (".", ".", PUNCT, 2, Punct)]);
        let r = irony(&ignored, "I love being ignored.").expect("ignored");
        assert!(r.features.contains(&"implicit incongruity") && r.features.contains(&"explicit incongruity"));
        assert_eq!(r.pair, ("love".to_string(), "being ignored".to_string()));

        let tooth = s(&[("Great", "great", ADJ, 5, Discourse), (",", ",", PUNCT, 1, Punct), ("my", "my", PRON, 4, NmodPoss), ("tooth", "tooth", NOUN, 5, Nsubj), ("hurts", "hurt", VERB, 0, Root)]);
        let r = irony(&tooth, "Great, my tooth hurts").expect("tooth");
        assert_eq!(r.features, vec!["explicit incongruity"]);
        assert_eq!((r.positive, r.negative, r.flips, r.longest_run, r.polarity), (1, 1, 1, 1, 0));

        let line = s(&[("I", "I", PRON, 2, Nsubj), ("love", "love", VERB, 0, Root), ("waiting", "wait", VERB, 2, Xcomp), ("in", "in", ADP, 5, Case), ("line", "line", NOUN, 3, Obl), ("for", "for", ADP, 7, Case), ("hours", "hour", NOUN, 3, Obl)]);
        let r = irony(&line, "I love waiting in line for hours").expect("line");
        assert_eq!(r.pair.1, "waiting in line");
        assert!(r.score >= 3);

        let monday = s(&[("I", "I", PRON, 4, Nsubj), ("ca", "can", AUX, 4, Aux), ("n't", "not", PART, 4, Advmod), ("wait", "wait", VERB, 0, Root), ("for", "for", ADP, 6, Case), ("Monday", "Monday", PROPN, 4, Obl), ("🙄", "🙄", SYM, 4, Discourse)]);
        let r = irony(&monday, "I can't wait for Monday 🙄").expect("monday");
        assert!(r.features.contains(&"implicit incongruity") && r.features.contains(&"emoticon"));
        assert_eq!(r.pair.0, "can't wait");

        let rains = s(&[("Oh", "oh", INTJ, 5, Discourse), ("great", "great", ADJ, 5, Discourse), (",", ",", PUNCT, 2, Punct), ("it", "it", PRON, 5, Expl), ("rains", "rain", VERB, 0, Root), ("again", "again", ADV, 5, Advmod), ("!", "!", PUNCT, 5, Punct)]);
        let r = irony(&rains, "Oh great, it rains again!").expect("rains");
        assert!(r.features.contains(&"oh great"));

        let enjoy = s(&[("I", "I", PRON, 3, Nsubj), ("just", "just", ADV, 3, Advmod), ("enjoy", "enjoy", VERB, 0, Root), ("doing", "do", VERB, 3, Xcomp), ("my", "my", PRON, 6, NmodPoss), ("taxes", "tax", NOUN, 4, Obj)]);
        assert!(irony(&enjoy, "I just enjoy doing my taxes").is_some());

        let perfect = s(&[("PERFECT", "perfect", ADJ, 0, Root), ("weather", "weather", NOUN, 1, Nsubj), ("for", "for", ADP, 4, Case), ("traffic", "traffic", NOUN, 1, Obl), ("!!", "!", PUNCT, 1, Punct)]);
        let r = irony(&perfect, "PERFECT weather for traffic!!").expect("perfect");
        assert!(r.features.contains(&"all caps") && r.features.contains(&"!!"));
    }

    #[test]
    fn sincere_sentences_give_none() {
        let mother = s(&[("I", "I", PRON, 2, Nsubj), ("love", "love", VERB, 0, Root), ("my", "my", PRON, 4, NmodPoss), ("mother", "mother", NOUN, 2, Obj)]);
        assert_eq!(irony(&mother, "I love my mother"), None);
        let stopped = s(&[("The", "the", DET, 2, Det), ("rain", "rain", NOUN, 3, Nsubj), ("stopped", "stop", VERB, 0, Root)]);
        assert_eq!(irony(&stopped, "The rain stopped"), None);
        // ended situation after a positive word: no incongruity
        let great_stop = s(&[("Great", "great", ADJ, 4, Discourse), (",", ",", PUNCT, 1, Punct), ("rain", "rain", NOUN, 4, Nsubj), ("stopped", "stop", VERB, 0, Root)]);
        assert_eq!(irony(&great_stop, "Great, rain stopped"), None);
        // negated positive verb: "I don't like being ignored" — sincere
        let dont = s(&[("I", "I", PRON, 4, Nsubj), ("do", "do", AUX, 4, Aux), ("n't", "not", PART, 4, Advmod), ("like", "like", VERB, 0, Root), ("being", "be", AUX, 6, AuxPass), ("ignored", "ignore", VERB, 4, Xcomp)]);
        assert_eq!(irony(&dont, "I don't like being ignored"), None);
        // a mixed opinion with "but"
        let mixed = s(&[("The", "the", DET, 2, Det), ("food", "food", NOUN, 4, Nsubj), ("was", "be", AUX, 4, Cop), ("good", "good", ADJ, 0, Root), ("but", "but", CCONJ, 9, Cc), ("the", "the", DET, 7, Det), ("service", "service", NOUN, 9, Nsubj), ("was", "be", AUX, 9, Cop), ("bad", "bad", ADJ, 4, Conj)]);
        assert_eq!(irony(&mixed, "The food was good but the service was bad"), None);
        // negative first, then positive: complaint and relief, not praise of the bad
        let relief = s(&[("My", "my", PRON, 2, NmodPoss), ("tooth", "tooth", NOUN, 3, Nsubj), ("hurt", "hurt", VERB, 0, Root), (",", ",", PUNCT, 6, Punct), ("now", "now", ADV, 6, Advmod), ("good", "good", ADJ, 3, Parataxis)]);
        assert_eq!(irony(&relief, "My tooth hurt, now good"), None);
        // cues alone are emphasis, not irony
        let fun = s(&[("lol", "lol", INTJ, 4, Discourse), ("that", "that", PRON, 4, Nsubj), ("was", "be", AUX, 4, Cop), ("fun", "fun", ADJ, 0, Root)]);
        assert_eq!(irony(&fun, "lol that was fun"), None);
        // "like" as a preposition is no sentiment
        let like = s(&[("It", "it", PRON, 2, Nsubj), ("looks", "look", VERB, 0, Root), ("like", "like", ADP, 4, Case), ("rain", "rain", NOUN, 2, Obl)]);
        assert_eq!(irony(&like, "It looks like rain"), None);
    }
}
