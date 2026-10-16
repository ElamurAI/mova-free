//! Questions under discussion (QUD) for stories and dialogues: which question does each sentence answer.
//!
//! - **Stack** (Roberts 2012): explicit questions ("…?") are pushed; q-alt is approximated by the wh-type (who,
//!   what, where, when, why, how, which, or yes/no) and the question's content lemmas (NOUN, PROPN, VERB, ADJ, NUM,
//!   without auxiliaries and light words).
//! - **Congruence**: an assertion attaches to the deepest (most recent) open question with (a) at least one shared
//!   content lemma and (b) an answer of the right type — who: an animate or proper subject/object; where: an oblique
//!   with a place preposition or a place adverb; when: a time word or obl:tmod/obl:unmarked; why: "because/since/so"
//!   or an advcl; how: a manner adverb or "by/with"; what/which: a new content argument; yes/no: the same predicate.
//!   A complete answer (the slot filled for the question's own predicate; the predicate asserted or negated for
//!   yes/no) pops the question; a partial one (another predicate, an indefinite filler, a hedge) pops nothing.
//! - **Implicit QUD** (Riester et al. 2021): with no congruent open question, a subquestion is accommodated from
//!   the given lemmas and the shape of the sentence's predicate ("What did the fox do?", "What was the fox like?",
//!   "What happened next?"). It is answered by the sentence itself and does not stay on the stack.
//! - **Givenness** (Wu et al. 2023, QUDeval): a question may use only lemmas of earlier sentences; a lemma new in
//!   the answer is answer leakage, reported (e.g. "Who is Reynard?" when Reynard first appears in the answer).
//! - Pronouns he/she/it/they resolve to the last subject entity (recency only).
//!
//! Deterministic (Vec, BTreeSet, fixed order); the stack and the reasons are the explanation.

use std::collections::BTreeSet;

use anyhow::Result;

use crate::tree::{Rel, UPos};

/// A pre-annotated token: form, lemma (lowercase), UPOS, relation and head (1-based, 0 — root).
#[derive(Clone, Debug)]
pub struct Tok {
    pub form: String,
    pub lemma: String,
    pub upos: UPos,
    pub rel: Rel,
    pub head: usize,
}

impl Tok {
    pub fn new(form: &str, lemma: &str, upos: UPos, rel: Rel, head: usize) -> Tok {
        Tok { form: form.into(), lemma: lemma.to_lowercase(), upos, rel, head }
    }
}

/// The wh-type of a question.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wh {
    Who,
    What,
    Where,
    When,
    Why,
    How,
    Which,
    YesNo,
}

impl Wh {
    pub fn name(self) -> &'static str {
        match self {
            Wh::Who => "who",
            Wh::What => "what",
            Wh::Where => "where",
            Wh::When => "when",
            Wh::Why => "why",
            Wh::How => "how",
            Wh::Which => "which",
            Wh::YesNo => "yes/no",
        }
    }

    fn of(lemma: &str) -> Option<Wh> {
        Some(match lemma {
            "who" | "whom" | "whose" => Wh::Who,
            "what" => Wh::What,
            "where" => Wh::Where,
            "when" => Wh::When,
            "why" => Wh::Why,
            "how" => Wh::How,
            "which" => Wh::Which,
            _ => return None,
        })
    }
}

/// An explicit question on the stack: its sentence number (1-based), wh-type, content lemmas and predicate.
#[derive(Clone, Debug)]
pub struct Question {
    pub id: usize,
    pub wh: Wh,
    pub lemmas: BTreeSet<String>,
    pub pred: Option<String>,
    words: BTreeSet<String>,
}

/// What a sentence attaches to: pushed as a question, an explicit open question (its number), or an implicit one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attach {
    Pushed(usize),
    Explicit(usize),
    Implicit(String),
}

/// Answer status of a sentence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Question,
    Complete,
    Partial,
}

/// One sentence of the text with its QUD, answer status, the stack after it (question numbers) and the reasons.
#[derive(Clone, Debug)]
pub struct Step {
    pub n: usize,
    pub text: String,
    pub attach: Attach,
    pub answer: Answer,
    pub stack: Vec<usize>,
    pub why: Vec<String>,
    pub leaks: Vec<String>,
}

const LIGHT: &[&str] = &["be", "have", "do", "get", "make", "let", "thing", "one", "way", "lot", "kind"];
const PRONOUNS: &[&str] = &["he", "she", "it", "they", "him", "her", "them"];
const PLACE_PREP: &[&str] = &["in", "at", "on", "to", "into", "under", "near", "behind"];
const PLACE_ADV: &[&str] = &["there", "here", "home", "away", "inside", "outside", "upstairs", "downstairs", "abroad", "back", "out", "off"];
const TIME_WORDS: &[&str] = &[
    "day", "night", "morning", "evening", "afternoon", "noon", "midnight", "dawn", "dusk", "week", "month", "year", "hour",
    "minute", "moment", "time", "today", "yesterday", "tomorrow", "tonight", "now", "then", "later", "soon", "once",
    "ago", "before", "after", "early", "late", "monday", "tuesday", "wednesday", "thursday", "friday", "saturday",
    "sunday", "spring", "summer", "autumn", "winter",
];
const REASON: &[&str] = &["because", "since", "so"];
const MANNER_CASE: &[&str] = &["by", "with"];
const NOT_MANNER: &[&str] = &["not", "n't", "never", "very", "too", "so", "also", "just", "only", "even", "still", "again", "really"];
const HEDGE: &[&str] = &["maybe", "perhaps", "probably", "possibly", "might", "may"];
const INDEFINITE: &[&str] = &["someone", "somebody", "something", "somewhere", "sometime", "someday", "somehow", "anyone", "anything"];

/// A sentence prepared for tracking: content lemmas (resolved pronouns included), subject entity and predicate.
struct Sent<'a> {
    toks: &'a [Tok],
    lemmas: BTreeSet<String>,
    words: BTreeSet<String>,
    subj: Option<(String, bool)>,
    pred: Option<String>,
    state: bool,
    resolved: Vec<(String, String)>,
}

fn content(t: &Tok) -> bool {
    matches!(t.upos, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ | UPos::NUM)
        && !matches!(t.rel.base(), Rel::Aux | Rel::Cop)
        && !LIGHT.contains(&t.lemma.as_str())
        && Wh::of(&t.lemma).is_none()
}

fn kids(toks: &[Tok], i: usize) -> impl Iterator<Item = usize> + '_ {
    toks.iter().enumerate().filter(move |(_, t)| t.head == i + 1).map(|(k, _)| k)
}

fn prepare<'a>(toks: &'a [Tok], last: &Option<(String, bool)>) -> Sent<'a> {
    let mut lemmas = BTreeSet::new();
    let mut resolved = Vec::new();
    for t in toks {
        if content(t) {
            lemmas.insert(t.lemma.clone());
        } else if t.upos == UPos::PRON && PRONOUNS.contains(&t.lemma.as_str()) {
            if let Some((e, _)) = last {
                lemmas.insert(e.clone());
                resolved.push((t.form.clone(), e.clone()));
            }
        }
    }
    let words = toks.iter().map(|t| t.lemma.clone()).collect();
    let root = toks.iter().position(|t| t.head == 0);
    let (mut subj, mut pred, mut state) = (None, None, false);
    if let Some(r) = root {
        let rt = &toks[r];
        state = kids(toks, r).any(|k| toks[k].rel.base() == Rel::Cop);
        if content(rt) || LIGHT.contains(&rt.lemma.as_str()) {
            pred = Some(rt.lemma.clone());
        }
        if let Some(s) = kids(toks, r).find(|k| toks[*k].rel.base() == Rel::Nsubj) {
            let st = &toks[s];
            subj = match st.upos {
                UPos::NOUN | UPos::PROPN => Some((st.lemma.clone(), st.upos == UPos::PROPN)),
                UPos::PRON if PRONOUNS.contains(&st.lemma.as_str()) => last.clone(),
                _ => None,
            };
        }
    }
    Sent { toks, lemmas, words, subj, pred, state, resolved }
}

fn question_of(id: usize, s: &Sent) -> Question {
    let wh = s.toks.iter().find_map(|t| Wh::of(&t.lemma)).unwrap_or(Wh::YesNo);
    let pred = s.pred.clone().filter(|p| !LIGHT.contains(&p.as_str()) || wh == Wh::YesNo);
    Question { id, wh, lemmas: s.lemmas.clone(), pred, words: s.words.clone() }
}

fn animate(t: &Tok) -> bool {
    t.upos == UPos::PROPN || t.upos == UPos::NOUN && matches!(crate::sense::animacy(&t.lemma), crate::sense::Anim::Animate(_))
}

/// The wh-slot filler in sentence `s` for question `q`: (description, partial) or None.
fn filler(q: &Question, s: &Sent) -> Option<(String, bool)> {
    let toks = s.toks;
    let new = |t: &Tok| !q.words.contains(&t.lemma);
    let case_of = |i: usize| kids(toks, i).find(|k| toks[*k].rel.base() == Rel::Case).map(|k| toks[k].lemma.clone());
    let indef = |want: &[&str]| toks.iter().find(|t| want.contains(&t.lemma.as_str())).map(|t| (format!("indefinite \"{}\"", t.form), true));
    match q.wh {
        Wh::Who => toks
            .iter()
            .find(|t| matches!(t.rel.base(), Rel::Nsubj | Rel::Obj | Rel::Iobj) && animate(t) && new(t) && !s.resolved.iter().any(|r| r.0 == t.form))
            .map(|t| (format!("{} \"{}\" ({}, animate)", t.rel, t.form, t.upos), false))
            .or_else(|| indef(&["someone", "somebody", "anyone"])),
        Wh::Where => toks
            .iter()
            .enumerate()
            .find_map(|(i, t)| {
                let c = case_of(i)?;
                (t.rel.base() == Rel::Obl && PLACE_PREP.contains(&c.as_str()) && new(t)).then(|| (format!("obl \"{c} {}\"", t.form), false))
            })
            .or_else(|| toks.iter().find(|t| t.upos == UPos::ADV && PLACE_ADV.contains(&t.lemma.as_str()) && new(t)).map(|t| (format!("place adverb \"{}\"", t.form), false)))
            .or_else(|| indef(&["somewhere"])),
        Wh::When => toks
            .iter()
            .find(|t| matches!(t.rel, Rel::OblTmod | Rel::OblUnmarked | Rel::NmodTmod) && new(t))
            .map(|t| (format!("{} \"{}\"", t.rel, t.form), false))
            .or_else(|| toks.iter().find(|t| TIME_WORDS.contains(&t.lemma.as_str()) && new(t)).map(|t| (format!("time word \"{}\"", t.form), false)))
            .or_else(|| indef(&["sometime", "someday"])),
        Wh::Why => toks
            .iter()
            .find(|t| REASON.contains(&t.lemma.as_str()) && matches!(t.rel.base(), Rel::Mark | Rel::Cc | Rel::Advmod))
            .map(|t| (format!("reason \"{}\" ({})", t.form, t.rel), false))
            .or_else(|| toks.iter().find(|t| t.rel.base() == Rel::Advcl).map(|t| (format!("advcl \"{}\"", t.form), false))),
        Wh::How => toks
            .iter()
            .enumerate()
            .find_map(|(i, t)| {
                let c = case_of(i)?;
                (t.rel.base() == Rel::Obl && MANNER_CASE.contains(&c.as_str())).then(|| (format!("obl \"{c} {}\"", t.form), false))
            })
            .or_else(|| {
                toks.iter()
                    .find(|t| {
                        t.rel.base() == Rel::Advmod
                            && t.upos == UPos::ADV
                            && new(t)
                            && ![PLACE_ADV, TIME_WORDS, NOT_MANNER, HEDGE, INDEFINITE].iter().any(|l| l.contains(&t.lemma.as_str()))
                    })
                    .map(|t| (format!("manner adverb \"{}\"", t.form), false))
            })
            .or_else(|| indef(&["somehow"])),
        Wh::What | Wh::Which => toks
            .iter()
            .find(|t| matches!(t.rel.base(), Rel::Obj | Rel::Nsubj | Rel::Obl | Rel::Xcomp | Rel::Ccomp | Rel::Root) && content(t) && new(t))
            .map(|t| (format!("{} \"{}\"", t.rel, t.form), false))
            .or_else(|| indef(&["something", "anything"])),
        Wh::YesNo => {
            let p = q.pred.as_ref()?;
            if s.pred.as_ref() != Some(p) && !s.lemmas.contains(p) {
                return None;
            }
            let neg = toks.iter().any(|t| matches!(t.lemma.as_str(), "not" | "n't" | "never" | "no"));
            let hedge = toks.iter().find(|t| HEDGE.contains(&t.lemma.as_str()));
            Some(match hedge {
                Some(h) => (format!("predicate \"{p}\" hedged by \"{}\"", h.form), true),
                None => (format!("predicate \"{p}\" {}", if neg { "negated" } else { "asserted" }), false),
            })
        }
    }
}

/// Congruence with an open question: (complete, reasons) or Err(reason it does not fit).
fn congruent(q: &Question, s: &Sent) -> std::result::Result<(bool, Vec<String>), String> {
    let shared: Vec<&String> = q.lemmas.intersection(&s.lemmas).collect();
    if shared.is_empty() && !q.lemmas.is_empty() {
        return Err(format!("Q{}: no shared lemma", q.id));
    }
    let shared_s = shared.iter().map(|x| x.as_str()).collect::<Vec<_>>().join(", ");
    let Some((fill, partial)) = filler(q, s) else {
        return Err(format!("Q{}: shared {shared_s}, but no {} answer", q.id, q.wh.name()));
    };
    let mut why = vec![format!("Q{} {}: shared {}", q.id, q.wh.name(), if shared.is_empty() { "nothing (question has no content lemmas)".into() } else { shared_s })];
    why.push(format!("slot: {fill}"));
    let pred_ok = match &q.pred {
        Some(p) => s.lemmas.contains(p) || s.pred.as_ref() == Some(p),
        None => true,
    };
    if !pred_ok && q.wh != Wh::YesNo {
        why.push(format!("question predicate \"{}\" not asserted", q.pred.as_deref().unwrap_or("")));
    }
    Ok((!partial && (pred_ok || q.wh == Wh::YesNo), why))
}

/// Givenness (QUDeval): lemmas of a question that are not given; new in the answer = leakage, else hallucination.
pub fn givenness(qlemmas: &[&str], given: &BTreeSet<String>, answer: &BTreeSet<String>) -> Vec<String> {
    qlemmas
        .iter()
        .filter(|l| !given.contains(**l))
        .map(|l| if answer.contains(*l) { format!("\"{l}\" occurs only in the answer (answer leakage)") } else { format!("\"{l}\" occurs nowhere before (hallucination)") })
        .collect()
}

fn name_of((lemma, propn): &(String, bool)) -> String {
    if *propn {
        let mut c = lemma.chars();
        c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
    } else {
        format!("the {lemma}")
    }
}

/// An implicit QUD for sentence `s` from given lemmas: (question, its lemmas, reasons, rejected candidates).
fn implicit(s: &Sent, given: &BTreeSet<String>, first: bool) -> (String, Vec<String>, Vec<String>, Vec<String>) {
    let mut why = Vec::new();
    let mut leaks = Vec::new();
    // a new name first: "Who is X?" would need the new lemma — QUDeval answer leakage, rejected
    for t in s.toks.iter().filter(|t| t.upos == UPos::PROPN && !given.contains(&t.lemma)) {
        let cand = format!("Who is {}?", name_of(&(t.lemma.clone(), true)));
        for l in givenness(&[t.lemma.as_str()], given, &s.lemmas) {
            leaks.push(format!("\"{cand}\" rejected: {l}"));
        }
    }
    let pred = s.pred.clone().unwrap_or_default();
    let pred_given = given.contains(&pred);
    let (q, ql): (String, Vec<String>) = match &s.subj {
        Some(e) if given.contains(&e.0) => {
            let kind = if pred.is_empty() || LIGHT.contains(&pred.as_str()) { "light" } else if pred_given { "given" } else { "new" };
            why.push(format!("subject \"{}\" given, predicate \"{pred}\" {kind}{}", e.0, if s.state { " (state)" } else { "" }));
            if pred.is_empty() || LIGHT.contains(&pred.as_str()) {
                (format!("What about {}?", name_of(e)), vec![e.0.clone()])
            } else if s.state {
                (format!("What was {} like?", name_of(e)), vec![e.0.clone()])
            } else {
                (format!("What did {} do?", name_of(e)), vec![e.0.clone()])
            }
        }
        Some(e) => {
            why.push(format!("subject \"{}\" new", e.0));
            (if first { "What happened?" } else { "What happened next?" }.into(), vec![])
        }
        None => {
            why.push("no subject entity".into());
            (if first { "What happened?" } else { "What happened next?" }.into(), vec![])
        }
    };
    let refs: Vec<&str> = ql.iter().map(String::as_str).collect();
    leaks.extend(givenness(&refs, given, &s.lemmas).into_iter().map(|l| format!("\"{q}\": {l}")));
    (q, ql, why, leaks)
}

/// The tracker over pre-annotated sentences (text, tokens).
pub fn track_toks(sents: &[(String, Vec<Tok>)]) -> Vec<Step> {
    let mut stack: Vec<Question> = Vec::new();
    let mut given: BTreeSet<String> = BTreeSet::new();
    let mut last: Option<(String, bool)> = None;
    let mut out = Vec::new();
    for (k, (text, toks)) in sents.iter().enumerate() {
        let n = k + 1;
        let s = prepare(toks, &last);
        let mut why: Vec<String> = s.resolved.iter().map(|(p, e)| format!("\"{p}\" → {e}")).collect();
        let mut leaks = Vec::new();
        let (attach, answer) = if text.trim_end().ends_with('?') {
            let q = question_of(n, &s);
            why.push(format!("{} question; lemmas {}", q.wh.name(), q.lemmas.iter().cloned().collect::<Vec<_>>().join(", ")));
            stack.push(q);
            (Attach::Pushed(n), Answer::Question)
        } else {
            let mut found = None;
            for (i, q) in stack.iter().enumerate().rev() {
                match congruent(q, &s) {
                    Ok(r) => {
                        found = Some((i, r));
                        break;
                    }
                    Err(e) => why.push(e),
                }
            }
            match found {
                Some((i, (complete, w))) => {
                    why.extend(w);
                    let id = stack[i].id;
                    if complete {
                        stack.remove(i);
                    }
                    (Attach::Explicit(id), if complete { Answer::Complete } else { Answer::Partial })
                }
                None => {
                    let (q, _, w, l) = implicit(&s, &given, k == 0);
                    why.extend(w);
                    leaks = l;
                    (Attach::Implicit(q), Answer::Complete)
                }
            }
        };
        given.extend(s.lemmas.iter().cloned());
        if s.subj.is_some() {
            last = s.subj.clone();
        }
        out.push(Step { n, text: text.clone(), attach, answer, stack: stack.iter().map(|q| q.id).collect(), why, leaks });
    }
    out
}

/// Tokens of a parsed sentence.
fn toks_of(t: &crate::tree::Tree) -> Vec<Tok> {
    t.w.iter().map(|w| Tok { form: w.form.clone(), lemma: w.lemma.clone(), upos: w.upos, rel: w.rel, head: w.head }).collect()
}

/// Splits and annotates the text, then tracks its QUDs.
pub fn track(text: &str) -> Result<Vec<Step>> {
    let a = crate::tree::annotator()?;
    let sents: Vec<(String, Vec<Tok>)> = crate::babi::split_sentences(text).into_iter().map(|s| (s.clone(), toks_of(&crate::tree::Tree::parse(a, &s)))).collect();
    Ok(track_toks(&sents))
}

/// Prints the QUD of every sentence, the stack after it and the reasons.
pub fn run(text: &str) -> Result<()> {
    for s in track(text)? {
        let att = match &s.attach {
            Attach::Pushed(id) => format!("push Q{id}"),
            Attach::Explicit(id) => format!("answers Q{id} ({})", if s.answer == Answer::Complete { "complete, popped" } else { "partial" }),
            Attach::Implicit(q) => format!("implicit \"{q}\""),
        };
        let st = s.stack.iter().map(|i| format!("Q{i}")).collect::<Vec<_>>().join(" ");
        println!("{:>2}  {}", s.n, s.text);
        println!("    {att} · stack [{st}]");
        if !s.why.is_empty() {
            println!("    why: {}", s.why.join("; "));
        }
        for l in &s.leaks {
            println!("    givenness: {l}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use Rel::*;
    use UPos::*;

    fn t(form: &str, lemma: &str, upos: UPos, rel: Rel, head: usize) -> Tok {
        Tok::new(form, lemma, upos, rel, head)
    }

    fn s(text: &str, toks: Vec<Tok>) -> (String, Vec<Tok>) {
        (text.into(), toks)
    }

    fn who_did_fox_meet() -> (String, Vec<Tok>) {
        s("Who did the fox meet?", vec![t("Who", "who", PRON, Obj, 4), t("did", "do", AUX, Aux, 4), t("the", "the", DET, Det, 3), t("fox", "fox", NOUN, Nsubj, 4), t("meet", "meet", VERB, Root, 0), t("?", "?", PUNCT, Punct, 4)])
    }

    #[test]
    fn who_question_answered_later_pops() {
        let sents = vec![
            who_did_fox_meet(),
            s("The fox met a crow.", vec![t("The", "the", DET, Det, 2), t("fox", "fox", NOUN, Nsubj, 3), t("met", "meet", VERB, Root, 0), t("a", "a", DET, Det, 5), t("crow", "crow", NOUN, Obj, 3), t(".", ".", PUNCT, Punct, 3)]),
        ];
        let st = track_toks(&sents);
        assert_eq!(st[0].attach, Attach::Pushed(1));
        assert_eq!(st[0].stack, vec![1]);
        assert_eq!(st[1].attach, Attach::Explicit(1));
        assert_eq!(st[1].answer, Answer::Complete);
        assert!(st[1].stack.is_empty(), "{:?}", st[1]);
        assert!(st[1].why.iter().any(|w| w.contains("crow")), "{:?}", st[1].why);
    }

    #[test]
    fn unrelated_assertion_does_not_attach() {
        // negative control: no shared lemma, no animate filler — the question stays open
        let sents = vec![
            who_did_fox_meet(),
            s("The sun was hot.", vec![t("The", "the", DET, Det, 2), t("sun", "sun", NOUN, Nsubj, 4), t("was", "be", AUX, Cop, 4), t("hot", "hot", ADJ, Root, 0), t(".", ".", PUNCT, Punct, 4)]),
        ];
        let st = track_toks(&sents);
        assert!(matches!(st[1].attach, Attach::Implicit(_)), "{:?}", st[1]);
        assert_eq!(st[1].stack, vec![1]);
        assert!(st[1].why.iter().any(|w| w.contains("no shared lemma")));
    }

    #[test]
    fn partial_answer_pops_nothing() {
        let sents = vec![
            who_did_fox_meet(),
            s("The fox met someone.", vec![t("The", "the", DET, Det, 2), t("fox", "fox", NOUN, Nsubj, 3), t("met", "meet", VERB, Root, 0), t("someone", "someone", PRON, Obj, 3), t(".", ".", PUNCT, Punct, 3)]),
        ];
        let st = track_toks(&sents);
        assert_eq!(st[1].attach, Attach::Explicit(1));
        assert_eq!(st[1].answer, Answer::Partial);
        assert_eq!(st[1].stack, vec![1]);
    }

    #[test]
    fn implicit_question_from_given_lemmas() {
        let sents = vec![
            s("The fox was hungry.", vec![t("The", "the", DET, Det, 2), t("fox", "fox", NOUN, Nsubj, 4), t("was", "be", AUX, Cop, 4), t("hungry", "hungry", ADJ, Root, 0), t(".", ".", PUNCT, Punct, 4)]),
            s("He jumped.", vec![t("He", "he", PRON, Nsubj, 2), t("jumped", "jump", VERB, Root, 0), t(".", ".", PUNCT, Punct, 2)]),
        ];
        let st = track_toks(&sents);
        assert_eq!(st[0].attach, Attach::Implicit("What happened?".into()));
        assert_eq!(st[1].attach, Attach::Implicit("What did the fox do?".into()));
        assert!(st[1].leaks.is_empty(), "{:?}", st[1].leaks);
        assert!(st[1].why.iter().any(|w| w.contains("\"He\" → fox")));
    }

    #[test]
    fn givenness_violation_reported() {
        let sents = vec![
            s("The fox slept.", vec![t("The", "the", DET, Det, 2), t("fox", "fox", NOUN, Nsubj, 3), t("slept", "sleep", VERB, Root, 0), t(".", ".", PUNCT, Punct, 3)]),
            s("Reynard came.", vec![t("Reynard", "reynard", PROPN, Nsubj, 2), t("came", "come", VERB, Root, 0), t(".", ".", PUNCT, Punct, 2)]),
        ];
        let st = track_toks(&sents);
        assert_eq!(st[1].attach, Attach::Implicit("What happened next?".into()));
        assert!(st[1].leaks.iter().any(|l| l.contains("Who is Reynard?") && l.contains("answer leakage")), "{:?}", st[1].leaks);
        // the rule itself: a lemma nowhere before and not in the answer is a hallucination
        let given: BTreeSet<String> = ["fox".to_string()].into();
        let ans: BTreeSet<String> = ["crow".to_string()].into();
        assert!(givenness(&["fox"], &given, &ans).is_empty());
        assert!(givenness(&["crow"], &given, &ans)[0].contains("leakage"));
        assert!(givenness(&["wolf"], &given, &ans)[0].contains("hallucination"));
    }
}
