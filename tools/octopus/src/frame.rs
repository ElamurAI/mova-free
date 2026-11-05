//! The deputy's own question frame, read from the native UD tree of one sentence.
//!
//! `world::ans::parse_question` reads story questions token by token and expects a wh-word first;
//! for a polar question ("Is Crab running normally?") or a modal request ("Could you start Eel?")
//! it puts the whole clause into the subject and leaves the verb slot empty. The deputy needs the
//! roles: who is asked about (subject), what is asked (verb or predicate adjective) and how the
//! sentence asks (mood). This reader takes them from the native parse and repairs one known
//! misparse of polar questions: the copula's subject read as the root noun with the real
//! predicate hanging below it ("Is the documenter running?" → root «documenter», «running» acl).
//!
//! Model: mood ∈ {imperative, modal_request, polar, wh, declarative}; subject and verb are node
//! indices (zero-based) or absent. Invariant: subject ≠ verb; a repaired frame names both. Cost:
//! O(n) over at most 160 words. Reverse: a frame does not determine the sentence (many sentences
//! share one frame); it only constrains the request head and the entity position.
use en::annotate::Word;
use en::gram::{Rel, Tag, UPos};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mood {
    /// "Start Eel." — a bare verb with no subject.
    Imperative,
    /// "Could you start Eel?" — a modal and «you» ask the addressee to act; read as imperative.
    ModalRequest,
    /// "Is Eel running?" — subject–auxiliary inversion asks to observe a state.
    Polar,
    /// "What is the status of Crab?" — a wh-word asks for a value.
    Wh,
    /// Anything else: a statement, a narrative or an unknown shape.
    Declarative,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub mood: Mood,
    /// Zero-based head node of the subject phrase.
    pub subject: Option<usize>,
    /// Zero-based node of the main verb or predicate adjective.
    pub verb: Option<usize>,
    /// True when the verb slot was recovered from the subject-as-root misparse.
    pub repaired: bool,
    /// Readable form: `polar | subj «the documenter» | verb run`.
    pub text: String,
}

const MODALS: &[&str] = &["can", "could", "would", "will"];
const WH: &[&str] = &["what", "which", "how", "who", "whom", "whose", "where", "when", "why"];

fn child(w: &[Word], head: usize, rels: &[Rel]) -> Option<usize> {
    (0..w.len()).find(|&i| w[i].head == head + 1 && rels.contains(&w[i].rel.base()))
}
fn predicate_like(x: &Word) -> bool {
    matches!(x.upos, UPos::VERB | UPos::ADJ) || (x.upos == UPos::NOUN && x.form.to_lowercase().ends_with("ing"))
}
/// The subject phrase in order: the head and its determiners, compounds, modifiers and possessors.
fn phrase(w: &[Word], head: usize) -> String {
    let mut nodes: Vec<usize> = (0..w.len())
        .filter(|&i| {
            i == head
                || (w[i].head == head + 1
                    && (w[i].rel == Rel::NmodPoss
                        || matches!(w[i].rel.base(), Rel::Det | Rel::Compound | Rel::Amod | Rel::Flat)))
        })
        .collect();
    nodes.sort_unstable();
    nodes.iter().map(|&i| w[i].form.to_lowercase()).collect::<Vec<_>>().join(" ")
}

pub fn read(w: &[Word]) -> Frame {
    let root = w.iter().position(|x| x.head == 0);
    let lower = |i: usize| w[i].form.to_lowercase();
    let mut f = Frame { mood: Mood::Declarative, subject: None, verb: None, repaired: false, text: String::new() };
    if let Some(r) = root {
        let first_aux = w.first().is_some_and(|x| x.upos == UPos::AUX);
        if w.iter().take(2).any(|x| WH.contains(&x.form.to_lowercase().as_str())) {
            f.mood = Mood::Wh;
            f.verb = Some(r);
            f.subject = child(w, r, &[Rel::Nsubj]);
        } else if first_aux && MODALS.contains(&lower(0).as_str()) && w.get(1).is_some_and(|x| x.form.eq_ignore_ascii_case("you")) {
            f.mood = Mood::ModalRequest;
            f.subject = Some(1);
            // «you» hangs on the verb it is asked to do; otherwise the root is that verb.
            let h = w[1].head;
            f.verb = Some(if h > 0 && w[h - 1].upos == UPos::VERB { h - 1 } else { r });
        } else if first_aux {
            f.mood = Mood::Polar;
            if predicate_like(&w[r]) && child(w, r, &[Rel::Nsubj]).is_some() {
                f.verb = Some(r);
                f.subject = child(w, r, &[Rel::Nsubj]);
            } else if matches!(w[r].upos, UPos::NOUN | UPos::PROPN) && w[0].tag != Tag::VBD {
                // Only present questions are repaired: a past one ("Was Crab running?") asks
                // about history, which the deputy does not admit, so its reading stays as parsed.
                // The subject was read as the root: the predicate is a later dependent of it
                // («running» acl, «busy» xcomp/amod), or the root itself is an -ing noun whose
                // compound is the subject («the Nautilus running» → compound + root).
                let later = (r + 1..w.len()).find(|&i| {
                    w[i].head == r + 1
                        && matches!(w[i].rel.base(), Rel::Acl | Rel::Amod | Rel::Xcomp)
                        && predicate_like(&w[i])
                });
                if let Some(v) = later {
                    f.subject = Some(r);
                    f.verb = Some(v);
                    f.repaired = true;
                } else if predicate_like(&w[r]) {
                    if let Some(s) = (0..r).rev().find(|&i| w[i].head == r + 1 && matches!(w[i].rel.base(), Rel::Compound | Rel::Nsubj)) {
                        f.subject = Some(s);
                        f.verb = Some(r);
                        f.repaired = true;
                    }
                }
            } else {
                f.verb = Some(r);
                // «Is Opus busy?» parsed with «opus» as a compound of «busy»: the name before the
                // predicate is its subject.
                f.subject = child(w, r, &[Rel::Nsubj])
                    .or_else(|| (0..r).rev().find(|&i| w[i].head == r + 1 && w[i].rel.base() == Rel::Compound));
            }
        } else if w[r].upos == UPos::VERB && w[r].tag == Tag::VB && child(w, r, &[Rel::Nsubj]).is_none() {
            f.mood = Mood::Imperative;
            f.verb = Some(r);
        } else {
            f.verb = Some(r);
            f.subject = child(w, r, &[Rel::Nsubj]);
        }
    }
    render(w, &mut f);
    f
}

/// The readable `text` of a frame over the words it was read from.
fn render(w: &[Word], f: &mut Frame) {
    let mood = serde_json::to_value(f.mood).ok().and_then(|v| v.as_str().map(str::to_owned)).unwrap_or_default();
    // A subject that is a compound under the predicate («the Sol Dolphin running») reads as the
    // whole name before the predicate, nested compounds included («is opus dolphin busy»: «opus» →
    // «dolphin» → «busy» in the 2026-11-05 en.bin).
    let premodifier = |r: Rel| matches!(r.base(), Rel::Det | Rel::Compound | Rel::Amod);
    let names = |i: usize, v: usize| {
        let mut at = i;
        while w[at].head != v + 1 {
            if w[at].head == 0 || !premodifier(w[at].rel) || w[at].head - 1 >= v {
                return false;
            }
            at = w[at].head - 1;
        }
        premodifier(w[at].rel)
    };
    let subject_text = match (f.subject, f.verb) {
        (Some(s), Some(v)) if w[s].head == v + 1 && s < v && w[s].rel.base() == Rel::Compound => (0..v)
            .filter(|&i| names(i, v))
            .map(|i| w[i].form.to_lowercase())
            .collect::<Vec<_>>()
            .join(" "),
        (Some(s), _) => phrase(w, s),
        _ => String::new(),
    };
    f.text = format!(
        "{mood}{} | subj «{}» | verb {}",
        if f.repaired { " repaired" } else { "" },
        subject_text,
        f.verb.map(|v| w[v].lemma.to_lowercase()).unwrap_or_else(|| "-".into())
    );
}

/// An imperative whose verb the parser read as a noun: "Show Crab's recent reports." parsed as one
/// noun phrase (both the 2026-11-05 en.bin and the one before it: «show» NOUN compound of «crab»,
/// «reports» root). The sentence has no verb or auxiliary at all, no subject, and its first word is
/// a bare singular common noun in base form whose lemma is one of `verbs` (the configured request
/// predicates) and that heads no determiner, possessor or modifier of its own. The repair reads the
/// first word as the imperative verb (VERB/VB, root) and the old root as its object; every other
/// attachment is kept. Returns None when any condition fails, so a real noun phrase ("The show
/// reports.") and any sentence with a verb stay as parsed. Invariant: the result is a tree with the
/// same words and one root. Reverse: the native words are kept by the caller for the record.
pub fn repair_imperative(w: &[Word], verbs: &std::collections::BTreeSet<String>) -> Option<Vec<Word>> {
    let first = w.first()?;
    let lemma = first.lemma.to_lowercase();
    let bare = first.upos == UPos::NOUN
        && first.tag == Tag::NN
        && first.form.eq_ignore_ascii_case(&lemma)
        && verbs.contains(&lemma);
    let own_phrase = w.iter().any(|x| {
        x.head == 1 && (x.rel == Rel::NmodPoss || matches!(x.rel.base(), Rel::Det | Rel::Amod | Rel::Nummod | Rel::Case))
    });
    let clause = w.iter().any(|x| matches!(x.upos, UPos::VERB | UPos::AUX) || x.rel.base() == Rel::Nsubj);
    let root = w.iter().position(|x| x.head == 0)?;
    if !bare || own_phrase || clause || w.len() < 2 {
        return None;
    }
    let mut out = w.to_vec();
    (out[0].upos, out[0].tag) = (UPos::VERB, Tag::VB);
    if root != 0 {
        (out[0].head, out[0].rel) = (0, Rel::Root);
        (out[root].head, out[root].rel) = (1, Rel::Obj);
    }
    Some(out)
}

/// `read` over the imperative repair when it applies; the frame is then marked repaired. Returns
/// the words the deputy reasons over (repaired or as parsed) and their frame.
pub fn read_repairing(w: &[Word], verbs: &std::collections::BTreeSet<String>) -> (Vec<Word>, Frame) {
    match repair_imperative(w, verbs) {
        Some(r) => {
            let mut f = read(&r);
            f.repaired = true;
            render(&r, &mut f);
            (r, f)
        }
        None => (w.to_vec(), read(w)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::Feats;
    fn w(spec: &[(&str, &str, UPos, Tag, usize, Rel)]) -> Vec<Word> {
        spec.iter()
            .map(|(form, lemma, upos, tag, head, rel)| Word {
                form: (*form).into(),
                lemma: (*lemma).into(),
                upos: *upos,
                tag: *tag,
                feats: Feats(0),
                head: *head,
                rel: *rel,
            })
            .collect()
    }
    // Parses below are the native en.bin (0.4) parses of refused or misread prompts (2026-11-04).
    #[test]
    fn polar_question_fills_the_verb_slot() {
        let t = w(&[
            ("is", "be", UPos::AUX, Tag::VBZ, 3, Rel::Aux),
            ("crab", "crab", UPos::PROPN, Tag::NNP, 3, Rel::Nsubj),
            ("running", "run", UPos::VERB, Tag::VBG, 0, Rel::Root),
            ("normally", "normally", UPos::ADV, Tag::RB, 3, Rel::Advmod),
        ]);
        let f = read(&t);
        assert_eq!((f.mood, f.subject, f.verb, f.repaired), (Mood::Polar, Some(1), Some(2), false));
        assert_eq!(f.text, "polar | subj «crab» | verb run");
    }
    #[test]
    fn subject_as_root_misparse_is_repaired() {
        let t = w(&[
            ("is", "be", UPos::AUX, Tag::VBZ, 3, Rel::Cop),
            ("the", "the", UPos::DET, Tag::DT, 3, Rel::Det),
            ("documenter", "documenter", UPos::NOUN, Tag::NN, 0, Rel::Root),
            ("running", "run", UPos::VERB, Tag::VBG, 3, Rel::Acl),
        ]);
        let f = read(&t);
        assert_eq!((f.mood, f.subject, f.verb, f.repaired), (Mood::Polar, Some(2), Some(3), true));
        assert_eq!(f.text, "polar repaired | subj «the documenter» | verb run");
        let busy = w(&[
            ("is", "be", UPos::AUX, Tag::VBZ, 2, Rel::Cop),
            ("eel", "eel", UPos::PROPN, Tag::NNP, 0, Rel::Root),
            ("busy", "busy", UPos::ADJ, Tag::JJ, 2, Rel::Xcomp),
        ]);
        assert_eq!(read(&busy).verb, Some(2));
        let compound = w(&[
            ("is", "be", UPos::AUX, Tag::VBZ, 4, Rel::Cop),
            ("the", "the", UPos::DET, Tag::DT, 4, Rel::Det),
            ("nautilus", "nautilus", UPos::PROPN, Tag::NNP, 4, Rel::Compound),
            ("running", "running", UPos::NOUN, Tag::NN, 0, Rel::Root),
        ]);
        let f = read(&compound);
        assert_eq!((f.subject, f.verb, f.repaired), (Some(2), Some(3), true));
        assert_eq!(f.text, "polar repaired | subj «the nautilus» | verb running");
        let name = w(&[
            ("is", "be", UPos::AUX, Tag::VBZ, 2, Rel::Cop),
            ("opus", "opus", UPos::PROPN, Tag::NNP, 3, Rel::Compound),
            ("busy", "busy", UPos::ADJ, Tag::JJ, 0, Rel::Root),
        ]);
        assert_eq!(read(&name).text, "polar | subj «opus» | verb busy");
        // en.bin 2483516a (2026-11-05): «opus» → «dolphin» → «busy»; the whole name is the subject.
        let nested = w(&[
            ("is", "be", UPos::AUX, Tag::VBZ, 4, Rel::Cop),
            ("opus", "opus", UPos::PROPN, Tag::NNP, 3, Rel::Compound),
            ("dolphin", "dolphin", UPos::PROPN, Tag::NNP, 4, Rel::Compound),
            ("busy", "busy", UPos::ADJ, Tag::JJ, 0, Rel::Root),
        ]);
        assert_eq!(read(&nested).text, "polar | subj «opus dolphin» | verb busy");
    }
    #[test]
    fn modal_request_puts_the_asked_verb_in_the_verb_slot() {
        let t = w(&[
            ("could", "could", UPos::AUX, Tag::MD, 3, Rel::Aux),
            ("you", "you", UPos::PRON, Tag::PRP, 3, Rel::Nsubj),
            ("start", "start", UPos::VERB, Tag::VB, 0, Rel::Root),
            ("eel", "eel", UPos::PROPN, Tag::NNP, 3, Rel::Obj),
        ]);
        let f = read(&t);
        assert_eq!((f.mood, f.subject, f.verb), (Mood::ModalRequest, Some(1), Some(2)));
        assert_eq!(f.text, "modal_request | subj «you» | verb start");
    }
    // «show crab 's recent reports» as the native en.bin parses it (2483516a and the model before
    // it, 2026-11-05): one noun phrase, «show» a compound of «crab».
    fn show_reports() -> Vec<Word> {
        w(&[
            ("show", "show", UPos::NOUN, Tag::NN, 2, Rel::Compound),
            ("crab", "crab", UPos::PROPN, Tag::NNP, 5, Rel::NmodPoss),
            ("'s", "'s", UPos::PART, Tag::POS, 2, Rel::Case),
            ("recent", "recent", UPos::ADJ, Tag::JJ, 5, Rel::Amod),
            ("reports", "report", UPos::NOUN, Tag::NNS, 0, Rel::Root),
        ])
    }
    fn verbs() -> std::collections::BTreeSet<String> {
        ["show", "check", "read"].map(str::to_string).into()
    }
    #[test]
    fn imperative_read_as_a_noun_phrase_is_repaired() {
        let (r, f) = read_repairing(&show_reports(), &verbs());
        assert_eq!((r[0].upos, r[0].tag, r[0].head, r[0].rel), (UPos::VERB, Tag::VB, 0, Rel::Root));
        assert_eq!((r[4].head, r[4].rel), (1, Rel::Obj));
        // Every other attachment is kept and the result is still one tree.
        assert_eq!((r[1].head, r[3].head), (5, 5));
        assert_eq!(r.iter().filter(|x| x.head == 0).count(), 1);
        assert_eq!((f.mood, f.verb, f.repaired), (Mood::Imperative, Some(0), true));
        assert_eq!(f.text, "imperative repaired | subj «» | verb show");
        // The first word as the root («show» root, «reports» nmod): only its tags change.
        let rooted = w(&[
            ("show", "show", UPos::NOUN, Tag::NN, 0, Rel::Root),
            ("reports", "report", UPos::NOUN, Tag::NNS, 1, Rel::Nmod),
        ]);
        let r = repair_imperative(&rooted, &verbs()).unwrap();
        assert_eq!((r[0].upos, r[0].head, r[1].head, r[1].rel), (UPos::VERB, 0, 1, Rel::Nmod));
    }
    #[test]
    fn noun_phrases_and_clauses_are_not_read_as_imperatives() {
        // Negative controls: a word that is not a request predicate, a determined noun, a plural,
        // a sentence with a verb, a subject.
        let mut t = show_reports();
        t[0].lemma = "stage".into();
        t[0].form = "stage".into();
        assert!(repair_imperative(&t, &verbs()).is_none());
        let mut det = show_reports();
        det.push(Word { form: "the".into(), lemma: "the".into(), upos: UPos::DET, tag: Tag::DT, feats: Feats(0), head: 1, rel: Rel::Det });
        assert!(repair_imperative(&det, &verbs()).is_none());
        let mut plural = show_reports();
        (plural[0].form, plural[0].tag) = ("shows".into(), Tag::NNS);
        assert!(repair_imperative(&plural, &verbs()).is_none());
        let mut verb = show_reports();
        verb[3].upos = UPos::VERB;
        assert!(repair_imperative(&verb, &verbs()).is_none());
        let mut subj = show_reports();
        subj[1].rel = Rel::Nsubj;
        assert!(repair_imperative(&subj, &verbs()).is_none());
        let (r, f) = read_repairing(&t, &verbs());
        assert_eq!((r, f.repaired), (t.clone(), false));
    }
    #[test]
    fn statements_and_noun_roots_without_predicates_are_not_repaired() {
        // Negative control: a polar copula with a plain noun predicate has nothing to repair.
        let t = w(&[
            ("is", "be", UPos::AUX, Tag::VBZ, 2, Rel::Cop),
            ("eel", "eel", UPos::PROPN, Tag::NNP, 0, Rel::Root),
        ]);
        let f = read(&t);
        assert_eq!((f.mood, f.verb, f.repaired), (Mood::Polar, None, false));
        let past = w(&[
            ("was", "be", UPos::AUX, Tag::VBD, 2, Rel::Cop),
            ("crab", "crab", UPos::NOUN, Tag::NN, 0, Rel::Root),
            ("running", "run", UPos::VERB, Tag::VBG, 2, Rel::Acl),
        ]);
        assert!(!read(&past).repaired);
        let s = w(&[
            ("crab", "crab", UPos::PROPN, Tag::NNP, 2, Rel::Nsubj),
            ("runs", "run", UPos::VERB, Tag::VBZ, 0, Rel::Root),
        ]);
        assert_eq!(read(&s).mood, Mood::Declarative);
        let i = w(&[("start", "start", UPos::VERB, Tag::VB, 0, Rel::Root), ("eel", "eel", UPos::PROPN, Tag::NNP, 1, Rel::Obj)]);
        assert_eq!(read(&i).mood, Mood::Imperative);
    }
}
