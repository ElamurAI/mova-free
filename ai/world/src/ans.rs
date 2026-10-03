//! Natural-language answers to FairytaleQA questions — from the world state, without the LLM (SLM).
//!
//! 1. **Question parsing** (deterministic): wh-word, auxiliary verb, subject, main verb,
//!    the rest; the subordinate clause after `when/after/before/because/if…` is the "time anchor"; negation; future.
//!    From this, the intent: feeling, why, what happened, what did, what said, what wanted, what thought, object of action,
//!    who, where, how, when, how many, description.
//! 2. **Fact search in the state**: events (verb, participants, why/how/means), character states (feelings,
//!    goal, status, location), intents, beliefs. Question and fact words go through lemmas of the `en` lexicon.
//!    The question's sections (as in FairytaleQA, where the model sees the section) are a preference, not a limit.
//! 3. **Answer from state slots**: the event's why, causing or resulting event, feelings at the anchor time, goal,
//!    belief statement, intent text, participant, location. An event is rendered from the agent label, the verb
//!    in the past tense (`en::morph`), the object label and `how`.
//! 4. No slot — "not in state" with a reason. Separately, marked "anchor", the anchor sentence of the best
//!    fact found (a fallback, not a state answer).
//!
//! Also here: the baseline (most similar sentence of the tale's sections), ROUGE-L F1 (max over two references, as in the paper).

use std::collections::BTreeSet;
use std::fmt::Write as _;

use en::gram::Tag as PTag;

use crate::ftqa::Qa;
use crate::lang::Role;
use crate::types::*;
use crate::world::{Event, Snap, World};

// ── words ───────────────────────────────────────────────────────────────────────────────────────────

/// Tokens for ROUGE and matching: lowercase Latin and digits; the rest are boundaries (`king's` → king, s).
pub fn toks(s: &str) -> Vec<String> {
    s.to_lowercase().split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_string).collect()
}

/// Function words (not content).
const STOP: &[&str] = &[
    "a", "an", "the", "of", "to", "in", "on", "at", "for", "with", "by", "from", "and", "or", "but", "is", "was", "were", "are", "be", "been",
    "being", "am", "did", "do", "does", "done", "had", "has", "have", "having", "will", "would", "could", "should", "can", "may", "might",
    "must", "shall", "it", "its", "he", "she", "they", "him", "her", "them", "his", "their", "theirs", "this", "that", "these", "those",
    "what", "who", "whom", "whose", "why", "how", "where", "when", "which", "after", "before", "because", "while", "if", "so", "as", "not",
    "s", "t", "there", "then", "than", "i", "you", "we", "me", "my", "your", "our", "us", "one", "into", "onto", "upon", "about", "all",
    "some", "any", "very", "too", "also", "just", "only", "own", "himself", "herself", "themselves", "itself", "such", "no", "nor",
    "again", "ever", "never", "still", "yet", "once", "since", "until", "till", "whom", "whose", "each", "other", "else", "much", "many",
    "more", "most", "get", "got", "make", "made", "happen", "happened", "happens", "happening", "thing", "things", "something", "anything",
    "kind", "way", "time", "first", "last", "story", "like", "going", "go", "went",
];

fn stop(w: &str) -> bool {
    STOP.contains(&w)
}

/// Word lemmas: the word itself, analyses from the `en` lexicon (all tags) and a suffix fallback.
pub fn lemmas(w: &str) -> Vec<String> {
    let mut out = vec![w.to_string()];
    let mut add = |x: String| {
        if !x.is_empty() && !out.contains(&x) {
            out.push(x);
        }
    };
    if let Some(f) = en::dict::form(w) {
        for r in en::dict::analyses(f) {
            add(en::dict::text(r.lemma()).to_lowercase());
        }
    }
    for (suf, rep) in [("ies", "y"), ("ied", "y"), ("es", ""), ("s", ""), ("ed", ""), ("ed", "e"), ("ing", ""), ("ing", "e"), ("ly", "")] {
        if let Some(stem) = w.strip_suffix(suf)
            && stem.len() >= 3
        {
            add(format!("{stem}{rep}"));
        }
    }
    out
}

/// UD tag frequencies of a word (from the lexicon analyses).
fn tag_counts(w: &str) -> Vec<(PTag, u32)> {
    en::dict::form(w).map(|f| en::dict::analyses(f).map(|r| (r.tag, r.count)).collect()).unwrap_or_default()
}

/// The word can be a verb in the `want` form (UD share ≥ 5%, or the word is known only to AGID).
fn verb_form(w: &str, want: &[PTag]) -> bool {
    if stop(w) {
        return false;
    }
    let tc = tag_counts(w);
    let total: u32 = tc.iter().map(|x| x.1).sum();
    let hit: u32 = tc.iter().filter(|(t, _)| want.contains(t)).map(|x| x.1).sum();
    if tc.iter().any(|(t, _)| want.contains(t)) {
        return total == 0 || hit * 20 >= total || hit >= 3;
    }
    false
}

/// Past tense of a verb (`give_up` → gave up) via `en::morph`.
pub fn past(verb: &str) -> String {
    let mut parts = verb.split(['_', '-']);
    let head = parts.next().unwrap_or(verb);
    let m = en::morph::Morph;
    let mut s = m.inflect(head, PTag::VBD);
    for p in parts {
        s.push(' ');
        s.push_str(p);
    }
    s
}

/// Feeling synonyms in questions → tags of the closed set (SLM question understanding).
const FEEL_SYN: &[(&str, &str)] = &[
    ("delighted", "happy"), ("glad", "happy"), ("joyful", "happy"), ("overjoyed", "happy"), ("pleased", "happy"), ("cheerful", "happy"),
    ("merry", "happy"), ("joy", "happy"), ("happiness", "happy"), ("unhappy", "sad"), ("sorrowful", "sad"), ("miserable", "sad"),
    ("grieved", "sad"), ("heartbroken", "sad"), ("sorrow", "sad"), ("grief", "sad"), ("scared", "afraid"), ("frightened", "afraid"),
    ("terrified", "afraid"), ("fearful", "afraid"), ("fear", "afraid"), ("furious", "angry"), ("mad", "angry"), ("cross", "angry"),
    ("enraged", "angry"), ("anger", "angry"), ("anxious", "worried"), ("nervous", "worried"), ("thankful", "grateful"),
    ("astonished", "surprised"), ("amazed", "surprised"), ("envious", "jealous"), ("frustrated", "annoyed"), ("irritated", "annoyed"),
    ("hopeless", "desperate"), ("confused", "puzzled"), ("sympathy", "sympathetic"), ("pity", "pitying"), ("love", "loving"),
];

fn feel_tag(w: &str) -> Option<&'static str> {
    if let Ok(t) = FEELING.tag(w) {
        return Some(t.as_str());
    }
    FEEL_SYN.iter().find(|(a, _)| *a == w).map(|(_, b)| *b)
}

// ── question parsing ────────────────────────────────────────────────────────────────────────────────────

/// What is asked.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Intent {
    Feel,
    Why,
    Happen,
    HappenTo,
    Do,
    Say,
    Want,
    Think,
    Obj,
    WhoSubj,
    WhoObj,
    Describe,
    Where,
    How,
    When,
    Num,
    Generic,
}

impl Intent {
    pub fn name(self) -> &'static str {
        match self {
            Intent::Feel => "feel",
            Intent::Why => "why",
            Intent::Happen => "happen",
            Intent::HappenTo => "happen-to",
            Intent::Do => "do",
            Intent::Say => "say",
            Intent::Want => "want",
            Intent::Think => "think",
            Intent::Obj => "obj",
            Intent::WhoSubj => "who",
            Intent::WhoObj => "who-obj",
            Intent::Describe => "describe",
            Intent::Where => "where",
            Intent::How => "how",
            Intent::When => "when",
            Intent::Num => "num",
            Intent::Generic => "generic",
        }
    }
}

/// A parsed question.
#[derive(Clone, Debug)]
pub struct Frame {
    pub wh: String,
    pub intent: Intent,
    pub fut: bool,
    pub neg: bool,
    pub passive: bool,
    /// subject (tokens, with function words)
    pub subj: Vec<String>,
    /// main verb (as in the text)
    pub verb: Option<String>,
    /// content of the main clause after the verb
    pub rest: Vec<String>,
    /// conjunction and content of the subordinate clause (time anchor)
    pub rel: Option<String>,
    pub anchor: Vec<String>,
    /// «about X» / «to X» after a feeling verb or happen
    pub about: Vec<String>,
}

const AUX: &[&str] = &["did", "do", "does", "will", "would", "was", "were", "is", "are", "could", "can", "should", "had", "has", "have", "might", "may", "must", "shall"];
const SUB: &[&str] = &["when", "after", "before", "because", "while", "if", "since", "until", "till", "once", "whenever"];
const FEEL_V: &[&str] = &["feel", "felt", "feeling", "feels", "react", "reacted", "react", "respond", "responded"];
const SAY_V: &[&str] = &[
    "say", "said", "tell", "told", "ask", "asked", "answer", "answered", "reply", "replied", "shout", "shouted", "cry", "cried", "call", "called",
    "whisper", "whispered", "promise", "promised", "order", "ordered", "command", "commanded", "beg", "begged", "warn", "warned", "suggest",
    "suggested", "advise", "advised", "announce", "announced", "exclaim", "exclaimed", "sing", "sang", "request", "requested",
];
const WANT_V: &[&str] = &["want", "wanted", "wish", "wished", "need", "needed", "hope", "hoped", "plan", "planned", "decide", "decided", "intend", "intended", "desire", "desired", "like", "liked", "try", "tried"];
const THINK_V: &[&str] = &[
    "think", "thought", "believe", "believed", "know", "knew", "realize", "realized", "learn", "learned", "learnt", "discover", "discovered",
    "notice", "noticed", "understand", "understood", "suppose", "supposed", "suspect", "suspected", "guess", "guessed", "remember",
    "remembered", "forget", "forgot", "imagine", "imagined", "expect", "expected", "fear", "feared", "find", "found",
];

/// Parse a question (lowercase tokens, as in FairytaleQA).
pub fn parse_question(q: &str) -> Frame {
    let norm = q.to_lowercase().replace("n't", " not").replace("can not", "can not").replace("won not", "will not").replace("'s", " s");
    let mut t = toks(&norm);
    // «what kind of / what type of» → description
    let kind_of = t.windows(3).any(|w| (w[0] == "what" || w[0] == "which") && (w[1] == "kind" || w[1] == "type" || w[1] == "sort") && w[2] == "of");
    let special = t.windows(2).any(|w| w == ["was", "special"] || w == ["look", "like"] || w == ["looked", "like"]);
    // subordinate clause
    let cut = t.iter().enumerate().skip(1).find(|(i, w)| SUB.contains(&w.as_str()) && !(*w == "once" && *i + 1 < t.len() && t[i + 1] == "upon")).map(|(i, _)| i);
    let (mut rel, mut anchor) = (None, Vec::new());
    if let Some(i) = cut {
        rel = Some(t[i].clone());
        anchor = t[i + 1..].to_vec();
        t.truncate(i);
    }
    let neg = t.iter().any(|w| w == "not" || w == "never");
    // «to whom / for what …» — preposition before wh
    if t.len() > 1 && ["to", "for", "with", "from", "by", "in", "at", "on", "of"].contains(&t[0].as_str()) && ["who", "whom", "what", "which", "whose"].contains(&t[1].as_str()) {
        t.remove(0);
    }
    let wh = t.first().cloned().unwrap_or_default();
    let mut i = 1;
    let mut aux = None;
    if t.get(1).is_some_and(|w| AUX.contains(&w.as_str())) {
        aux = t.get(1).cloned();
        i = 2;
    } else if t.get(2).is_some_and(|w| AUX.contains(&w.as_str())) && ["what", "which", "whose"].contains(&wh.as_str()) {
        // «what job did X give …» — a noun in the wh-group
        aux = t.get(2).cloned();
        i = 3;
    }
    // «who had / who did …» without another verb: the auxiliary is the main verb
    let subj_wh = ["who", "what", "which"].contains(&wh.as_str());
    // «how many / how much»
    if wh == "how" && t.get(1).is_some_and(|w| w == "many" || w == "much") {
        let np: Vec<String> = t[2..].iter().take_while(|w| !AUX.contains(&w.as_str())).cloned().collect();
        return Frame { wh: wh.clone(), intent: Intent::Num, fut: false, neg, passive: false, subj: np, verb: None, rest: Vec::new(), rel, anchor, about: Vec::new() };
    }
    let fut = aux.as_deref().is_some_and(|a| matches!(a, "will" | "would" | "shall" | "might" | "may"));
    let be = aux.as_deref().is_some_and(|a| matches!(a, "was" | "were" | "is" | "are"));
    // main verb: after the auxiliary — the first word of the needed form, not right after a determiner
    let want: &[PTag] = if be {
        &[PTag::VBG, PTag::VBN]
    } else if aux.as_deref().is_some_and(|a| matches!(a, "had" | "has" | "have")) {
        &[PTag::VBN]
    } else if aux.is_some() {
        &[PTag::VB]
    } else {
        &[PTag::VBD, PTag::VBZ, PTag::VBP, PTag::VB, PTag::VBN]
    };
    let dets = ["the", "a", "an", "his", "her", "their", "its", "my", "your", "our", "s", "this", "that", "some", "every", "each", "not", "never"];
    // function words that can be the main verb (did X *do*, what *happened*, X *had*)
    const MAINV: &[&str] = &["do", "doing", "did", "done", "have", "had", "having", "get", "got", "make", "made", "go", "went", "going", "happen", "happened", "happens", "like", "liked", "be"];
    let skip = ["he", "she", "they", "it", "i", "you", "we", "him", "them", "there", "all", "both", "who", "what", "which", "that", "then", "so", "also", "just", "ever", "still", "really", "only"];
    let mut vpos = None;
    for j in i..t.len() {
        let w = &t[j];
        let mainv = MAINV.contains(&w.as_str());
        if !mainv && (dets.contains(&w.as_str()) || skip.contains(&w.as_str()) || stop(w) && !FEEL_V.contains(&w.as_str()) && !SAY_V.contains(&w.as_str())) {
            continue;
        }
        let after_det = j > i && dets.contains(&t[j - 1].as_str()) && t[j - 1] != "not" && t[j - 1] != "never";
        // without an auxiliary (who/what + verb) the verb comes right after wh; an unknown word in -ed too
        if aux.is_none() && j == i && (mainv || verb_form(w, want) || FEEL_V.contains(&w.as_str()) || (w.ends_with("ed") && tag_counts(w).is_empty())) {
            vpos = Some(j);
            break;
        }
        // «who was standing …» — the subject is wh itself: a participle right after be
        if be && subj_wh && j == i && verb_form(w, &[PTag::VBG]) {
            vpos = Some(j);
            break;
        }
        if aux.is_some() && j > i && !after_det && (mainv && !be || verb_form(w, want) || FEEL_V.contains(&w.as_str())) {
            vpos = Some(j);
            break;
        }
        if aux.is_none() {
            break;
        }
    }
    // «who had seven sons», «what did the trick» — auxiliary as main verb, subject is wh
    if vpos.is_none() && subj_wh && i == 2 && aux.as_deref().is_some_and(|a| matches!(a, "had" | "has" | "have" | "did" | "does" | "do")) {
        vpos = Some(1);
        i = 1;
        aux = None;
    }
    let (subj, verb, rest) = match vpos {
        Some(v) => (t[i..v].to_vec(), Some(t[v].clone()), t[v + 1..].to_vec()),
        None => (t[i..].to_vec(), None, Vec::new()),
    };
    let passive = be && verb.as_deref().is_some_and(|v| verb_form(v, &[PTag::VBN]) && !verb_form(v, &[PTag::VBG]));
    let vb = verb.clone().unwrap_or_default();
    // «what made/caused X <verb> …» — a question about the cause: X is the subject, then its verb
    if (wh == "what" || wh == "which") && aux.is_none() && matches!(vb.as_str(), "made" | "make" | "makes" | "caused" | "cause" | "causes" | "led") {
        let k = rest.iter().enumerate().skip(1).find(|(_, w)| !stop(w) && verb_form(w, &[PTag::VB])).map(|(k, _)| k);
        let (s2, v2, r2) = match k {
            Some(k) => (rest[..k].to_vec(), Some(rest[k].clone()), rest[k + 1..].to_vec()),
            None => (rest.clone(), None, Vec::new()),
        };
        return Frame { wh: wh.clone(), intent: Intent::Why, fut, neg, passive: false, subj: s2, verb: v2, rest: r2, rel, anchor, about: Vec::new() };
    }
    let has = |set: &[&str]| t.iter().any(|w| set.contains(&w.as_str()));
    // «about X» after a feeling
    let about = rest.iter().position(|w| w == "about" || w == "towards" || w == "toward").map(|p| rest[p + 1..].to_vec()).unwrap_or_default();
    let intent = if (wh == "how" || wh == "what") && (has(FEEL_V) || t.windows(2).any(|w| w[1] == "feeling" || w[1] == "feelings")) {
        Intent::Feel
    } else if wh == "why" {
        Intent::Why
    } else if kind_of || special {
        Intent::Describe
    } else if (wh == "what" || wh == "which") && t.iter().any(|w| w == "happen" || w == "happened" || w == "happens" || w == "happening") {
        if t.iter().skip_while(|w| !w.starts_with("happen")).nth(1).is_some_and(|w| w == "to") { Intent::HappenTo } else { Intent::Happen }
    } else if wh == "what" && (vb == "do" || vb == "doing" || vb == "did") {
        Intent::Do
    } else if wh == "what" && SAY_V.contains(&vb.as_str()) {
        Intent::Say
    } else if wh == "what" && WANT_V.contains(&vb.as_str()) {
        Intent::Want
    } else if wh == "what" && THINK_V.contains(&vb.as_str()) && vb != "find" && vb != "found" {
        Intent::Think
    } else if wh == "who" || wh == "whom" || wh == "whose" {
        if verb.is_none() {
            Intent::Describe
        } else if aux.is_none() || (be && !passive) {
            Intent::WhoSubj
        } else {
            Intent::WhoObj
        }
    } else if wh == "where" {
        Intent::Where
    } else if wh == "when" {
        Intent::When
    } else if wh == "how" {
        if verb.is_none() { Intent::Describe } else { Intent::How }
    } else if wh == "what" || wh == "which" {
        if verb.is_none() && be {
            Intent::Describe
        } else if aux.is_none() {
            // «what made X …», «what saved X» — subject question: cause or agent
            Intent::Generic
        } else {
            Intent::Obj
        }
    } else {
        Intent::Generic
    };
    Frame { wh, intent, fut, neg, passive, subj, verb, rest, rel, anchor, about }
}

/// Content words (no function words or pure punctuation).
fn content(ws: &[String]) -> Vec<String> {
    ws.iter().filter(|w| !stop(w) && w.len() > 1).cloned().collect()
}

// ── labels and rendering ────────────────────────────────────────────────────────────────────────────────────

fn text_field(s: &Snap, f: Field) -> Option<String> {
    match s.get(f) {
        Val::Known(V::Text(t)) => Some(t),
        Val::Known(V::Words(w)) => Some(w.join(" ").replace('_', " ")),
        Val::Known(V::Tag(t)) => Some(t.as_str().replace('_', " ")),
        _ => None,
    }
}

/// Entity name for the answer: `name`, else «the <class>», else the type.
pub fn name_of(w: &World, id: Id) -> String {
    let Some(s) = w.now(id) else { return id.to_string() };
    if let Some(n) = text_field(&s, Field::Name) {
        return n;
    }
    if let Some(c) = text_field(&s, Field::Class) {
        return format!("the {c}");
    }
    text_field(&s, Field::Type).map(|t| format!("the {t}")).unwrap_or_else(|| id.to_string())
}

/// Entity words for matching the question: name, class, type.
fn ent_words(w: &World, id: Id) -> Vec<String> {
    let Some(s) = w.now(id) else { return Vec::new() };
    let mut out = Vec::new();
    for f in [Field::Name, Field::Class, Field::Type] {
        if let Some(t) = text_field(&s, f) {
            out.extend(content(&toks(&t)));
        }
    }
    out
}

/// A place in human terms: name and up to two ancestors.
fn place_str(w: &World, l: Id) -> String {
    let mut s = name_of(w, l);
    for p in w.ancestors(l).into_iter().take(2) {
        let _ = write!(s, " in {}", name_of(w, p));
    }
    s
}

/// An event in human terms: agents, verb in past tense, object, addressee, how.
pub fn render(w: &World, e: &Event) -> String {
    let names = |r: Role| e.all(r).iter().map(|s| name_of(w, s.id)).collect::<Vec<_>>().join(" and ");
    if let Some(sp) = &e.speech {
        let who = if sp.narrator { "the narrator".to_string() } else { names(Role::Agent) };
        let to = names(Role::To);
        let mut s = format!("{who} said");
        if !to.is_empty() {
            let _ = write!(s, " to {to}");
        }
        if let Some(m) = &sp.means {
            let _ = write!(s, " {m}");
        }
        return s;
    }
    let mut s = names(Role::Agent);
    let verb = if e.x.neg { format!("did not {}", e.verb.replace('_', " ")) } else { past(&e.verb) };
    if !s.is_empty() {
        s.push(' ');
    }
    s.push_str(&verb);
    let p = names(Role::Patient);
    if !p.is_empty() {
        let _ = write!(s, " {p}");
    }
    let to = names(Role::To);
    if !to.is_empty() {
        let _ = write!(s, " to {to}");
    }
    if let Some(h) = &e.how {
        let _ = write!(s, " {h}");
    }
    s
}

// ── fact search ──────────────────────────────────────────────────────────────────────────────────────

/// Event words: verb, participants, texts.
struct EvWords {
    verb: Vec<Vec<String>>,
    who: Vec<Vec<String>>,
    text: Vec<Vec<String>>,
}

fn lem_all(ws: &[String]) -> Vec<Vec<String>> {
    ws.iter().map(|w| lemmas(w)).collect()
}

fn ev_words(w: &World, e: &Event) -> EvWords {
    let verb: Vec<String> = e.verb.split(['_', '-']).map(str::to_string).collect();
    let mut who = Vec::new();
    for (_, s) in &e.roles {
        who.extend(ent_words(w, s.id));
    }
    let mut text = Vec::new();
    for t in [&e.why, &e.how].into_iter().flatten() {
        text.extend(content(&toks(t)));
    }
    if let Some(sp) = &e.speech
        && let Some(m) = &sp.means
    {
        text.extend(content(&toks(m)));
    }
    // reader v2: story time, aboutness, and texts of intents or beliefs born from this event
    if let Some(t) = &e.x.when {
        text.extend(content(&toks(t)));
    }
    if let Some(a) = e.x.about {
        who.extend(ent_words(w, a));
    }
    for reg in [Reg::P, Reg::B] {
        for x in w.ents(reg) {
            let s = x.now();
            if s.id(Field::Src) == Some(e.id)
                && let Some(t) = text_field(&s, Field::What)
            {
                text.extend(content(&toks(&t)));
            }
        }
    }
    EvWords { verb: lem_all(&verb), who: lem_all(&who), text: lem_all(&text) }
}

fn meets(a: &[String], b: &[Vec<String>]) -> bool {
    b.iter().any(|x| x.iter().any(|y| a.contains(y)))
}

/// Match of question words with an event: (content, whether the verb matched).
fn score_words(q: &[Vec<String>], ew: &EvWords) -> (f64, bool) {
    let mut s = 0.0;
    let mut vhit = false;
    for ql in q {
        if meets(ql, &ew.verb) {
            s += 3.0;
            vhit = true;
        } else if meets(ql, &ew.who) {
            s += 1.5;
        } else if meets(ql, &ew.text) {
            s += 1.0;
        }
    }
    (s, vhit)
}

/// Resolve a word group (subject, «about X») into characters/objects/places: the head word is the last content word.
pub fn resolve_np(w: &World, np: &[String], regs: &[Reg]) -> Vec<Id> {
    // possessive: «the king s daughter» — the owner (before `s`) is never the head word
    let own = np.iter().rposition(|x| x == "s").map(|p| content(&np[..p]).len()).unwrap_or(0);
    let c = content(np);
    // head word is the last content word; if not found — the previous one (the princess *sad*)
    for k in (own..c.len()).rev().take(3) {
        let r = resolve_head(w, &c[..=k], regs);
        if !r.is_empty() {
            return r;
        }
    }
    Vec::new()
}

fn resolve_head(w: &World, c: &[String], regs: &[Reg]) -> Vec<Id> {
    let Some(head) = c.last() else { return Vec::new() };
    let head_l = lemmas(head);
    let mods: Vec<Vec<String>> = c[..c.len() - 1].iter().map(|x| lemmas(x)).collect();
    let mut best: Vec<(f64, usize, Id)> = Vec::new();
    for reg in regs {
        for e in w.ents(*reg) {
            let ws = ent_words(w, e.id);
            let wl: Vec<Vec<String>> = ws.iter().map(|x| lemmas(x)).collect();
            if !wl.iter().any(|x| x.iter().any(|y| head_l.contains(y))) {
                continue;
            }
            let m = mods.iter().filter(|ml| wl.iter().any(|x| x.iter().any(|y| ml.contains(y)))).count() as f64;
            // a modifier absent from the name (the *old* woman vs the young woman), — a minus
            let miss = mods.iter().filter(|ml| !wl.iter().any(|x| x.iter().any(|y| ml.contains(y)))).count() as f64;
            best.push((2.0 + m - 0.3 * miss, e.mentions().len(), e.id));
        }
    }
    best.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then(b.1.cmp(&a.1)));
    let top = best.first().map(|x| x.0).unwrap_or(0.0);
    best.into_iter().filter(|x| x.0 >= top - 1e-9).map(|x| x.2).collect()
}

/// Pronoun subject: «he/she/they» → the most mentioned character of the sections (rough).
fn pronoun(np: &[String]) -> bool {
    content(np).is_empty() && np.iter().any(|w| matches!(w.as_str(), "he" | "she" | "they" | "him" | "her" | "them" | "i" | "you" | "we"))
}

/// A found event and its score.
#[derive(Clone, Copy)]
struct Hit<'w> {
    e: &'w Event,
    score: f64,
}

/// Best events for the words (preferring the question's sections and participant `who` in role `role`).
fn find_events<'w>(w: &'w World, words: &[String], who: &[Id], role: Option<Role>, secs: &[u16], need_verb: bool) -> Vec<Hit<'w>> {
    // reader v2: first with the question's verb; if none — without it, but with at least two content matches
    let out = find_events_min(w, words, who, role, secs, need_verb, 1.0);
    if !out.is_empty() || !need_verb {
        return out;
    }
    find_events_min(w, words, who, role, secs, false, 2.0)
}

fn find_events_min<'w>(w: &'w World, words: &[String], who: &[Id], role: Option<Role>, secs: &[u16], need_verb: bool, min: f64) -> Vec<Hit<'w>> {
    let ql = lem_all(&content(words));
    let mut out = Vec::new();
    for e in w.events() {
        let ew = ev_words(w, e);
        let (mut s, vhit) = score_words(&ql, &ew);
        let content_s = s;
        if need_verb && !vhit {
            continue;
        }
        if !who.is_empty() {
            let inrole = role.is_some_and(|r| e.has(r, who));
            let any = e.roles.iter().any(|(_, x)| who.contains(&x.id));
            s += if inrole { 3.0 } else if any { 1.0 } else { -2.0 };
        }
        // question sections (as in FairytaleQA): strong preference, far away — a minus
        let sec = w.text.sents[e.pos.sent as usize - 1].0;
        if secs.contains(&sec) {
            s += 3.0;
        } else if secs.iter().any(|x| x.abs_diff(sec) == 1) {
            s += 0.5;
        } else {
            s -= 1.5;
        }
        if content_s < min && who.is_empty() {
            continue;
        }
        if content_s < min && s < 4.0 {
            continue;
        }
        out.push(Hit { e, score: s });
    }
    out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap().then(a.e.pos.sent.cmp(&b.e.pos.sent)));
    out
}

/// Beliefs and intents of a character (or anyone's, if `who` is empty) with the largest word overlap;
/// preference for those created in the question's sections. (score, sentence, text, id)
fn find_minds(w: &World, words: &[String], who: &[Id], secs: &[u16], regs: &[Reg]) -> Vec<(f64, u16, String, Id)> {
    let ql = lem_all(&content(words));
    let mut out = Vec::new();
    for reg in regs {
        for x in w.ents(*reg) {
            let s = x.now();
            if !who.is_empty() && !s.ids(SetF::Owners).iter().any(|c| who.contains(c)) {
                continue;
            }
            let Some(t) = text_field(&s, Field::What) else { continue };
            let tl = lem_all(&content(&toks(&t)));
            let m = ql.iter().filter(|q| meets(q, &tl)).count() as f64;
            let at = x.states()[0].pos.sent;
            let sec = w.text.sents[at as usize - 1].0;
            let bonus = if secs.contains(&sec) { 2.0 } else if secs.iter().any(|y| y.abs_diff(sec) == 1) { 0.5 } else { 0.0 };
            if m + bonus >= 2.0 && m >= 1.0 {
                out.push((m + bonus, at, t, x.id));
            }
        }
    }
    out.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    out
}

/// Characters whose traits, status or name match the question words (≥ 2 words; for a single word — that
/// word). (score, id)
fn find_by_traits(w: &World, words: &[String]) -> Vec<(f64, Id)> {
    let ql = lem_all(&content(words));
    let mut out = Vec::new();
    for e in w.ents(Reg::C) {
        let s = e.now();
        let mut ws = Vec::new();
        for f in [Field::Trait, Field::Status] {
            for (_, v) in w.history(e.id, f) {
                if let Val::Known(x) = v {
                    ws.extend(content(&toks(&x.to_string())));
                }
            }
        }
        if let Some(n) = text_field(&s, Field::Name) {
            ws.extend(content(&toks(&n)));
        }
        let wl = lem_all(&ws);
        let m = ql.iter().filter(|q| meets(q, &wl)).count() as f64;
        if m >= 2.0 || (ql.len() == 1 && m >= 1.0) {
            out.push((m, e.id));
        }
    }
    out.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    out
}

/// First and last sentence of the question's sections.
fn sec_span(w: &World, secs: &[u16]) -> (u16, u16) {
    let mut lo = u16::MAX;
    let mut hi = 0;
    for (i, (p, _)) in w.text.sents.iter().enumerate() {
        if secs.contains(p) {
            lo = lo.min(i as u16 + 1);
            hi = hi.max(i as u16 + 1);
        }
    }
    if hi == 0 { (1, w.text.sents.len() as u16) } else { (lo, hi) }
}

// ── answer ─────────────────────────────────────────────────────────────────────────────────────────

/// Where the answer comes from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Src {
    /// from state slots
    State,
    /// not in state
    None,
}

#[derive(Clone, Debug)]
pub struct Answer {
    pub frame: String,
    pub src: Src,
    pub text: String,
    /// state facts the answer comes from (ids, anchors), or the "not in state" reason
    pub note: String,
    /// fallback: anchor sentence of the best fact found (marked separately)
    pub anchor: Option<(u16, String)>,
}

fn ans(frame: &Frame, text: String, note: String, anchor: Option<(u16, String)>) -> Answer {
    Answer { frame: frame_str(frame), src: Src::State, text: num_words(&text), note, anchor }
}

/// Numbers 1–20 as words (tales and references use «seven», not «7»).
fn num_words(s: &str) -> String {
    const W: [&str; 21] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen", "twenty",
    ];
    s.split(' ').map(|t| t.parse::<usize>().ok().filter(|n| *n <= 20).map(|n| W[n].to_string()).unwrap_or_else(|| t.to_string())).collect::<Vec<_>>().join(" ")
}

fn none(frame: &Frame, note: impl Into<String>, anchor: Option<(u16, String)>) -> Answer {
    Answer { frame: frame_str(frame), src: Src::None, text: String::new(), note: note.into(), anchor }
}

pub fn frame_str(f: &Frame) -> String {
    let mut s = f.intent.name().to_string();
    if f.fut {
        s.push_str(" fut");
    }
    if f.neg {
        s.push_str(" neg");
    }
    if f.passive {
        s.push_str(" pass");
    }
    let _ = write!(s, " | subj «{}» | verb {} | rest «{}»", f.subj.join(" "), f.verb.clone().unwrap_or("-".into()), f.rest.join(" "));
    if let Some(r) = &f.rel {
        let _ = write!(s, " | {r} «{}»", f.anchor.join(" "));
    }
    s
}

fn sent_text(w: &World, n: u16) -> (u16, String) {
    (n, w.text.sents[n as usize - 1].1.clone())
}

/// A character's feelings: the tag set in the snapshot.
fn feelings(s: &Snap) -> Vec<Tag> {
    match s.get(Field::Feeling) {
        Val::Known(v) => v.tag_list(),
        Val::Unknown => Vec::new(),
    }
}

/// Feelings that appeared in a character within sentences [a, b] (states `feel`/`set`), with causes.
fn feelings_in(w: &World, c: Id, a: u16, b: u16) -> Vec<(u16, Vec<Tag>, Option<Id>)> {
    let Some(e) = w.ent(c) else { return Vec::new() };
    let mut out = Vec::new();
    let mut prev: Vec<Tag> = Vec::new();
    for st in e.states() {
        let now = feelings(&e.snap(st.ver));
        let added: Vec<Tag> = now.iter().copied().filter(|t| !prev.contains(t)).collect();
        if st.pos.sent >= a && st.pos.sent <= b && !added.is_empty() {
            out.push((st.pos.sent, added, st.cause));
        }
        prev = now;
    }
    out
}

fn join_tags(v: &[Tag]) -> String {
    // reader v2: at most two — a long list is not an answer
    let s: Vec<&str> = v.iter().take(2).map(|t| t.as_str()).collect();
    match s.len() {
        0 => String::new(),
        1 => s[0].to_string(),
        _ => format!("{} and {}", s[..s.len() - 1].join(", "), s[s.len() - 1]),
    }
}

/// Answer a FairytaleQA question from the state.
pub fn answer(w: &World, q: &Qa) -> Answer {
    let f = parse_question(&q.question);
    let secs = &q.secs;
    let (lo, hi) = sec_span(w, secs);
    let chars = [Reg::C];
    // subject: character (or object, place)
    let mut subj = resolve_np(w, &f.subj, &[Reg::C]);
    if subj.is_empty() {
        subj = resolve_np(w, &f.subj, &[Reg::O, Reg::L]);
    }
    if subj.is_empty() && pronoun(&f.subj) {
        // pronoun: the most frequent character of the question's sections
        let mut best: Vec<(usize, Id)> = w.ents(Reg::C).map(|e| (e.mentions().iter().filter(|m| m.sent >= lo && m.sent <= hi).count(), e.id)).collect();
        best.sort_by(|a, b| b.0.cmp(&a.0));
        subj = best.into_iter().take(1).filter(|x| x.0 > 0).map(|x| x.1).collect();
    }
    // time anchor: the subordinate clause's event
    let anchor_ev: Option<Hit> = if f.anchor.is_empty() {
        None
    } else {
        let an_subj = {
            // subject of the subordinate clause — the first group before the verb (roughly: the first 1–4 content words that form a name)
            let ids = resolve_np(w, &f.anchor.iter().take(4).cloned().collect::<Vec<_>>(), &chars);
            if ids.len() == 1 { ids } else { Vec::new() }
        };
        find_events(w, &f.anchor, &an_subj, Some(Role::Agent), secs, false).into_iter().next()
    };
    let main_words: Vec<String> = f.verb.iter().cloned().chain(f.rest.iter().cloned()).collect();
    let all_words: Vec<String> = f.subj.iter().chain(main_words.iter()).cloned().collect();
    let fallback = |hits: &[Hit]| -> Option<(u16, String)> { hits.first().map(|h| sent_text(w, h.e.pos.sent)) };
    let t_anchor = anchor_ev.map(|h| h.e.pos.sent);

    match f.intent {
        Intent::Feel => {
            let who = if subj.is_empty() { Vec::new() } else { subj.clone() };
            let Some(c) = who.first().copied().filter(|c| c.reg == Reg::C) else {
                return none(&f, "feeling: question character not found", anchor_ev.map(|h| sent_text(w, h.e.pos.sent)));
            };
            // 1) feelings caused by the anchor event; 2) appeared from the anchor to the end of the sections; 3) in the sections
            if let Some(h) = anchor_ev {
                let t = h.e.pos.sent;
                let caused: Vec<Tag> = feelings_in(w, c, t, u16::MAX).into_iter().filter(|x| x.2 == Some(h.e.id)).flat_map(|x| x.1).collect();
                if !caused.is_empty() {
                    return ans(&f, join_tags(&caused), format!("{} feel because {} {}", w.label(c), w.label(h.e.id), h.e.anchors()), Some(sent_text(w, t)));
                }
                let after = feelings_in(w, c, t, hi.max(t + 2));
                if let Some((s, tags, cause)) = after.first() {
                    return ans(&f, join_tags(tags), format!("{} feel ^s{s}{}", w.label(c), cause.map(|x| format!(" because {}", w.label(x))).unwrap_or_default()), Some(sent_text(w, *s)));
                }
                // reader v2: the last added feelings before the anchor (not the whole set)
                if let Some((s, tags, _)) = feelings_in(w, c, lo.min(t), t).last() {
                    return ans(&f, join_tags(tags), format!("{} feel ^s{s} (latest before anchor ^s{t})", w.label(c)), Some(sent_text(w, *s)));
                }
            }
            // «about Y»: feelings caused by events involving Y
            let about = resolve_np(w, &f.about, &[Reg::C, Reg::O, Reg::L]);
            let ins = feelings_in(w, c, lo, hi);
            if !about.is_empty()
                && let Some((s, tags, _)) = ins.iter().find(|x| x.2.and_then(|e| w.event(e)).is_some_and(|e| e.roles.iter().any(|(_, r)| about.contains(&r.id))))
            {
                return ans(&f, join_tags(tags), format!("{} feel ^s{s} (about {})", w.label(c), w.label(about[0])), Some(sent_text(w, *s)));
            }
            // best match with the remaining words among the causing events of feelings in the sections
            if let Some((s, tags, _)) = ins.first() {
                return ans(&f, join_tags(tags), format!("{} feel ^s{s} (in the sections)", w.label(c)), Some(sent_text(w, *s)));
            }
            // forecast: feelings after the sections
            if f.fut
                && let Some((s, tags, _)) = feelings_in(w, c, hi, hi.saturating_add(15)).first()
            {
                return ans(&f, join_tags(tags), format!("{} feel ^s{s} (later in the tale)", w.label(c)), Some(sent_text(w, *s)));
            }
            // reader v2: the last added feelings up to the end of the sections
            if let Some((s, tags, _)) = feelings_in(w, c, 1, hi).last() {
                return ans(&f, join_tags(tags), format!("{} feel ^s{s} (latest up to the end of the sections)", w.label(c)), Some(sent_text(w, *s)));
            }
            none(&f, format!("no feelings of {} in the sections", w.label(c)), anchor_ev.map(|h| sent_text(w, h.e.pos.sent)))
        }
        Intent::Why => why(w, &f, &subj, &main_words, secs, anchor_ev),
        Intent::Happen | Intent::HappenTo => {
            let target = if f.intent == Intent::HappenTo {
                let p = f.rest.iter().position(|x| x == "to").map(|p| f.rest[p + 1..].to_vec()).unwrap_or_default();
                resolve_np(w, &p, &[Reg::C, Reg::O, Reg::L])
            } else {
                Vec::new()
            };
            let base = match anchor_ev {
                Some(h) => Some(h),
                None if f.intent == Intent::HappenTo => None,
                None => find_events(w, &all_words, &subj, None, secs, false).into_iter().next(),
            };
            let from = base.map(|h| h.e.pos.sent).unwrap_or(lo);
            // outcomes: events with cause=base; then the following events (for target — with it in a role)
            if let Some(b) = base
                && target.is_empty()
            {
                let caused: Vec<&Event> = w.events().iter().filter(|e| e.x.cause == Some(b.e.id) && e.speech.as_ref().is_none_or(|s| !s.narrator)).take(2).collect();
                if !caused.is_empty() {
                    let t = caused.iter().map(|e| render(w, e)).collect::<Vec<_>>().join(", and ");
                    return ans(&f, t, format!("outcomes {} ← {}", caused.iter().map(|e| e.id.to_string()).collect::<Vec<_>>().join(" "), b.e.id), Some(sent_text(w, caused[0].pos.sent)));
                }
                // reader v2: an outcome is a state change (feeling, status) with cause=base
                for c in w.ents(Reg::C) {
                    for (s, tags, cause) in feelings_in(w, c.id, b.e.pos.sent, u16::MAX) {
                        if cause == Some(b.e.id) {
                            return ans(&f, format!("{} felt {}", name_of(w, c.id), join_tags(&tags)), format!("{} feel ← {}", w.label(c.id), b.e.id), Some(sent_text(w, s)));
                        }
                    }
                }
            }
            let next: Vec<&Event> = w
                .events()
                .iter()
                .filter(|e| e.pos.sent >= from && base.is_none_or(|b| e.id != b.e.id && (e.pos.sent > b.e.pos.sent || e.line > b.e.line)))
                .filter(|e| target.is_empty() || e.roles.iter().any(|(_, s)| target.contains(&s.id)))
                .filter(|e| e.speech.is_none())
                .take(2)
                .collect();
            if next.is_empty() {
                return none(&f, "no next event in the state", base.map(|b| sent_text(w, b.e.pos.sent)));
            }
            let t = next.iter().map(|e| render(w, e)).collect::<Vec<_>>().join(", and ");
            ans(&f, t, format!("next {} after {}", next.iter().map(|e| e.id.to_string()).collect::<Vec<_>>().join(" "), base.map(|b| b.e.id.to_string()).unwrap_or("the start of the sections".into())), Some(sent_text(w, next[0].pos.sent)))
        }
        Intent::Do => {
            if subj.is_empty() {
                return none(&f, "what did: subject not found", None);
            }
            let from = t_anchor.unwrap_or(lo);
            let evs: Vec<&Event> = w
                .events()
                .iter()
                .filter(|e| e.has(Role::Agent, &subj) && e.speech.is_none() && !e.x.neg)
                .filter(|e| e.pos.sent >= from && anchor_ev.is_none_or(|a| e.id != a.e.id))
                .filter(|e| if f.fut || anchor_ev.is_some() { true } else { e.pos.sent <= hi })
                .collect();
            // among the agent's events — the best by the remaining words, else the first
            let rest_l = lem_all(&content(&f.rest));
            let best = evs.iter().max_by(|a, b| {
                let sa = score_words(&rest_l, &ev_words(w, a)).0 - a.pos.sent.abs_diff(from) as f64 * 0.01;
                let sb = score_words(&rest_l, &ev_words(w, b)).0 - b.pos.sent.abs_diff(from) as f64 * 0.01;
                sa.partial_cmp(&sb).unwrap()
            });
            match best {
                Some(e) => ans(&f, render(w, e), format!("{} {}", w.label(e.id), e.anchors()), Some(sent_text(w, e.pos.sent))),
                None => none(&f, format!("no events of agent {} after ^s{from}", w.label(subj[0])), None),
            }
        }
        Intent::Say => {
            let hits: Vec<Hit> = find_events(w, &all_words.iter().chain(f.anchor.iter()).cloned().collect::<Vec<_>>(), &subj, Some(Role::Agent), secs, false)
                .into_iter()
                .filter(|h| h.e.speech.is_some() && (subj.is_empty() || h.e.has(Role::Agent, &subj)))
                .filter(|h| t_anchor.is_none_or(|t| h.e.pos.sent >= t))
                .collect();
            if let Some(h) = hits.first() {
                let sp = h.e.speech.as_ref().unwrap();
                // promise: the intent text from this utterance
                if f.verb.as_deref().is_some_and(|v| v.starts_with("promis"))
                    && let Some(p) = w.ents(Reg::P).find(|p| p.now().id(Field::Src) == Some(h.e.id))
                    && let Some(t) = text_field(&p.now(), Field::What)
                {
                    return ans(&f, t, format!("{} ← {}", p.id, h.e.id), Some(sent_text(w, h.e.pos.sent)));
                }
                return match &sp.means {
                    Some(m) => ans(&f, m.clone(), format!("{} means {}", h.e.id, h.e.anchors()), Some(sent_text(w, h.e.pos.sent))),
                    None => none(&f, format!("{} has no means", h.e.id), Some(sent_text(w, h.e.pos.sent))),
                };
            }
            none(&f, "utterance not found", None)
        }
        Intent::Want => {
            let Some(c) = subj.first().copied().filter(|c| c.reg == Reg::C) else { return none(&f, "what wanted: character not found", None) };
            let t = t_anchor.unwrap_or(hi);
            // goal in the state at the time (or the first set in the sections), the character's wish/plan intents
            let rest_l = lem_all(&content(&f.rest));
            let mut cands: Vec<(f64, u16, String, String)> = Vec::new();
            for (st, v) in w.history(c, Field::Goal) {
                if let Val::Known(V::Text(g)) = v {
                    let near = if st.pos.sent >= lo && st.pos.sent <= hi.max(t) { 2.0 } else { 0.0 };
                    let m = rest_l.iter().filter(|ql| toks(&g).iter().any(|x| ql.contains(x))).count() as f64;
                    cands.push((near + m - (st.pos.sent.abs_diff(t) as f64) * 0.02, st.pos.sent, g, format!("{}@{} goal", w.label(c), st.ver)));
                }
            }
            for p in w.ents(Reg::P) {
                let s = p.now();
                if !s.has(SetF::Owners, c) {
                    continue;
                }
                let kind = text_field(&s, Field::Kind).unwrap_or_default();
                if !matches!(kind.as_str(), "wish" | "plan" | "bargain" | "task" | "order") {
                    continue;
                }
                let Some(g) = text_field(&s, Field::What) else { continue };
                let at = p.states()[0].pos.sent;
                let near = if at >= lo && at <= hi.max(t) { 2.0 } else { 0.0 };
                let m = rest_l.iter().filter(|ql| toks(&g).iter().any(|x| ql.contains(x))).count() as f64;
                cands.push((near + m - (at.abs_diff(t) as f64) * 0.02, at, g, format!("{} {kind}", p.id)));
            }
            cands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            match cands.into_iter().next() {
                Some((_, s, g, note)) => ans(&f, g, note, Some(sent_text(w, s))),
                None => none(&f, format!("no goal or wish of {}", w.label(c)), None),
            }
        }
        Intent::Think => {
            let Some(c) = subj.first().copied().filter(|c| c.reg == Reg::C) else { return none(&f, "what thought: character not found", None) };
            let rest_l = lem_all(&content(&f.rest));
            let mut cands: Vec<(f64, u16, String, Id)> = Vec::new();
            for b in w.ents(Reg::B) {
                let s = b.now();
                if !s.has(SetF::Owners, c) {
                    continue;
                }
                let Some(claim) = text_field(&s, Field::What) else { continue };
                let at = b.states()[0].pos.sent;
                let near = if at >= lo && at <= hi { 2.0 } else { 0.0 };
                let m = rest_l.iter().filter(|ql| toks(&claim).iter().any(|x| ql.contains(x))).count() as f64;
                cands.push((near + m, at, claim, b.id));
            }
            // plain attribute belief (v1)
            for (st, v) in w.history(c, Field::Belief) {
                if let Val::Known(V::Text(g)) = v {
                    let near = if st.pos.sent >= lo && st.pos.sent <= hi { 2.0 } else { 0.0 };
                    cands.push((near, st.pos.sent, g, c));
                }
            }
            cands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            match cands.into_iter().next() {
                Some((_, s, g, id)) => ans(&f, g, format!("{id} believe"), Some(sent_text(w, s))),
                None => none(&f, format!("no beliefs of {}", w.label(c)), None),
            }
        }
        Intent::Obj | Intent::WhoObj | Intent::WhoSubj | Intent::Generic => {
            // «what did X have/carry/hold» — possession: items in the state at the time
            if f.intent == Intent::Obj
                && f.verb.as_deref().is_some_and(|v| matches!(v, "have" | "had" | "own" | "owned" | "carry" | "carried" | "hold" | "held" | "keep" | "kept" | "possess" | "possessed"))
                && let Some(c) = subj.first().copied()
            {
                let t = t_anchor.unwrap_or(hi);
                if let Some((v, s)) = w.state_at(c, t) {
                    let items = s.ids(SetF::Items);
                    if !items.is_empty() {
                        let names: Vec<String> = items.iter().map(|o| name_of(w, *o)).collect();
                        return ans(&f, names.join(" and "), format!("{}@{v}.items at ^s{t}", w.label(c)), None);
                    }
                }
            }
            // reader v2: «who did X think …» — X's beliefs
            if matches!(f.intent, Intent::WhoObj | Intent::Obj)
                && f.verb.as_deref().is_some_and(|v| THINK_V.contains(&v))
                && !subj.is_empty()
                && let Some((_, s, t, id)) = find_minds(w, &main_words, &subj, secs, &[Reg::B]).into_iter().next()
            {
                return ans(&f, t, format!("{id} believe"), Some(sent_text(w, s)));
            }
            let (role, words) = match f.intent {
                Intent::WhoSubj | Intent::Generic => (Some(Role::Patient), main_words.clone()),
                _ => (Some(Role::Agent), main_words.clone()),
            };
            let who_ids = if f.intent == Intent::WhoSubj || f.intent == Intent::Generic { resolve_np(w, &f.rest, &[Reg::C, Reg::O]) } else { subj.clone() };
            let who_role = if f.passive { Some(Role::Patient) } else { role };
            let hits = find_events(w, &words, &who_ids, who_role, secs, f.verb.is_some());
            let hits: Vec<Hit> = hits.into_iter().filter(|h| t_anchor.is_none_or(|t| h.e.pos.sent + 3 >= t)).collect();
            let Some(h) = hits.first().copied() else {
                // reader v2: «who was a tall, handsome man …» — a character's traits or status
                if f.intent == Intent::WhoSubj
                    && let Some((_, c)) = find_by_traits(w, &main_words).into_iter().next()
                {
                    return ans(&f, name_of(w, c), format!("{} trait/status", w.label(c)), None);
                }
                return none(&f, "event not found", None);
            };
            let e = h.e;
            let pick = |r: Role| -> Option<String> {
                let v: Vec<String> = e.all(r).iter().filter(|s| !who_ids.contains(&s.id)).map(|s| name_of(w, s.id)).collect();
                (!v.is_empty()).then(|| v.join(" and "))
            };
            let text = match f.intent {
                Intent::WhoSubj => {
                    if f.passive { pick(Role::Patient) } else { pick(Role::Agent) }
                }
                Intent::WhoObj => pick(Role::Patient).or_else(|| pick(Role::To)),
                Intent::Obj if f.verb.as_deref().is_some_and(|v| v == "use" || v == "used") => pick(Role::Instr).or_else(|| pick(Role::Patient)),
                Intent::Obj => pick(Role::Patient).or_else(|| pick(Role::Instr)).or_else(|| e.how.clone()),
                _ => pick(Role::Agent).map(|a| format!("{a}: {}", render(w, e))).or_else(|| Some(render(w, e))),
            };
            match text {
                Some(t) => ans(&f, t, format!("{} {}", w.label(e.id), e.anchors()), Some(sent_text(w, e.pos.sent))),
                None => none(&f, format!("{}: needed role missing", w.label(e.id)), fallback(&hits)),
            }
        }
        Intent::Where => {
            // event with a verb → at/to; else the subject's location at the anchor time or in the sections
            if f.verb.as_deref().is_some_and(|v| !matches!(v, "live" | "lived" | "be" | "stay" | "stayed")) {
                let hits = find_events(w, &main_words, &subj, Some(Role::Agent), secs, true);
                if let Some(h) = hits.first() {
                    let e = h.e;
                    if let Some(l) = e.role(Role::At).or_else(|| e.all(Role::To).into_iter().find(|s| s.id.reg == Reg::L)) {
                        return ans(&f, place_str(w, l.id), format!("{} at/to {}", w.label(e.id), e.anchors()), Some(sent_text(w, e.pos.sent)));
                    }
                    // the agent's location after the event
                    if let Some(a) = e.role(Role::Agent)
                        && let Some((_, s)) = w.state_at(a.id, e.pos.sent)
                        && let Some(l) = s.id(Field::At)
                    {
                        return ans(&f, place_str(w, l), format!("{} in {} at {}", w.label(a.id), l, e.anchors()), Some(sent_text(w, e.pos.sent)));
                    }
                }
            }
            let Some(c) = subj.first().copied() else { return none(&f, "where: subject not found", None) };
            // reader v2: «where was the village» — the subject is itself a place: its name and ancestors
            if c.reg == Reg::L {
                return ans(&f, place_str(w, c), format!("{} name and in=", w.label(c)), None);
            }
            let t = t_anchor.unwrap_or(lo);
            // the first location after t (for «go») or the location at t
            let hist = w.history(c, Field::At);
            let went = hist.iter().find(|(st, _)| st.pos.sent >= t && st.pos.sent <= hi.max(t + 3));
            let at = went.map(|(st, v)| (st.pos.sent, v.clone())).or_else(|| w.state_at(c, t).map(|(_, s)| (t, s.get(Field::At))));
            match at {
                Some((s, Val::Known(V::Id(l)))) if l.reg == Reg::L => ans(&f, place_str(w, l), format!("{}.at since ^s{s}", w.label(c)), Some(sent_text(w, s))),
                Some((s, Val::Known(V::Id(h)))) => ans(&f, format!("with {}", name_of(w, h)), format!("{}.at = {h} since ^s{s}", w.label(c)), Some(sent_text(w, s))),
                _ => {
                    // the first known location at all
                    match hist.first() {
                        Some((st, Val::Known(V::Id(l)))) => ans(&f, place_str(w, *l), format!("{}.at (first) since {}", w.label(c), st.pos), Some(sent_text(w, st.pos.sent))),
                        _ => none(&f, format!("location of {} unknown", w.label(c)), None),
                    }
                }
            }
        }
        Intent::How => {
            let hits = find_events(w, &main_words, &subj, Some(Role::Agent), secs, true);
            let Some(h) = hits.first().copied() else { return none(&f, "event not found", None) };
            let e = h.e;
            if let Some(hw) = &e.how {
                return ans(&f, hw.clone(), format!("{} how {}", w.label(e.id), e.anchors()), Some(sent_text(w, e.pos.sent)));
            }
            if let Some(i) = e.role(Role::Instr) {
                return ans(&f, format!("with {}", name_of(w, i.id)), format!("{} instr", w.label(e.id)), Some(sent_text(w, e.pos.sent)));
            }
            // manner — causing events
            if let Some(c) = e.x.cause.and_then(|c| w.event(c)) {
                return ans(&f, render(w, c), format!("{} ← cause {}", e.id, c.id), Some(sent_text(w, e.pos.sent)));
            }
            none(&f, format!("{}: no how", w.label(e.id)), Some(sent_text(w, e.pos.sent)))
        }
        Intent::When => {
            let hits = find_events(w, &all_words, &subj, Some(Role::Agent), secs, false);
            let Some(h) = hits.first().copied() else { return none(&f, "event not found", None) };
            if let Some(t) = &h.e.x.when {
                return ans(&f, t.replace('_', " "), format!("{} when", h.e.id), Some(sent_text(w, h.e.pos.sent)));
            }
            if let Some(c) = h.e.x.cause.and_then(|c| w.event(c)) {
                return ans(&f, format!("after {}", render(w, c)), format!("{} ← cause {}", h.e.id, c.id), Some(sent_text(w, h.e.pos.sent)));
            }
            let cur = w.cursor().iter().rev().find(|c| c.pos.sent <= h.e.pos.sent).and_then(|c| c.time.clone());
            match cur {
                Some(t) => ans(&f, t.replace('_', " "), format!("time cursor at {}", h.e.anchors()), Some(sent_text(w, h.e.pos.sent))),
                None => none(&f, "no time", Some(sent_text(w, h.e.pos.sent))),
            }
        }
        Intent::Num => {
            let ids = resolve_np(w, &f.subj, &[Reg::O, Reg::C]);
            for id in &ids {
                if let Some(s) = w.now(*id)
                    && let Some(n) = text_field(&s, Field::Num)
                {
                    return ans(&f, n, format!("{}.num", w.label(*id)), None);
                }
            }
            none(&f, "no number", None)
        }
        Intent::Describe => {
            let target = if subj.is_empty() { resolve_np(w, &f.rest, &[Reg::C, Reg::O, Reg::L]) } else { subj.clone() };
            // «who was deeply in love with …» — a trait, not a name: the agent of an event with these words
            if (f.wh == "who" || f.wh == "whom") && (target.is_empty() || target.first().is_some_and(|c| c.reg != Reg::C)) {
                let hits = find_events(w, &f.subj, &[], None, secs, false);
                if let Some(h) = hits.first()
                    && let Some(a) = h.e.role(Role::Agent)
                {
                    return ans(&f, name_of(w, a.id), format!("agent {} {}", w.label(h.e.id), h.e.anchors()), Some(sent_text(w, h.e.pos.sent)));
                }
            }
            let Some(c) = target.first().copied() else { return none(&f, "description: entity not found", None) };
            let s = w.now(c).unwrap_or_default();
            let mut parts = Vec::new();
            for fl in [Field::Trait, Field::Status] {
                if let Some(t) = text_field(&s, fl) {
                    parts.push(t);
                }
            }
            // status and traits present in the sections (the first state in the interval)
            if let Some((_, sn)) = w.state_at(c, hi) {
                for fl in [Field::Trait, Field::Status, Field::Age] {
                    if let Some(t) = text_field(&sn, fl)
                        && !parts.contains(&t)
                    {
                        parts.push(t);
                    }
                }
            }
            // reader v2: parts (scales of glittering green) are a description too
            for p in s.ids(SetF::Parts) {
                parts.push(name_of(w, p));
            }
            let name = name_of(w, c);
            if parts.is_empty() {
                return ans(&f, name.clone(), format!("{} (name only)", w.label(c)), None);
            }
            ans(&f, format!("{} {name}", parts.join(", ")), format!("{} trait/status/parts", w.label(c)), None)
        }
    }
}

/// "Why": the event's why; causing event; cause of a state change (feeling, status); the agent's goal; intent.
fn why(w: &World, f: &Frame, subj: &[Id], main_words: &[String], secs: &[u16], anchor_ev: Option<Hit>) -> Answer {
    // «why was X <feeling>» — the cause of the feeling
    let fw: Vec<&str> = f.subj.iter().chain(f.rest.iter()).chain(f.verb.iter()).filter_map(|x| feel_tag(x)).collect();
    if let (Some(c), Some(tag)) = (subj.first().copied(), fw.first()) {
        let (lo, hi) = sec_span(w, secs);
        let hitf = feelings_in(w, c, 1, u16::MAX)
            .into_iter()
            .filter(|x| x.1.iter().any(|t| t.as_str() == *tag))
            .min_by_key(|x| if x.0 >= lo && x.0 <= hi { 0 } else { 1 + x.0.abs_diff(lo) as u32 });
        if let Some((s, _, Some(cause))) = hitf
            && let Some(e) = w.event(cause)
        {
            let t = e.why.clone().unwrap_or_else(|| render(w, e));
            return ans(f, t, format!("{} {tag} ^s{s} because {}", w.label(c), w.label(cause)), Some(sent_text(w, s)));
        }
    }
    let hits = find_events(w, main_words, subj, Some(if f.passive { Role::Patient } else { Role::Agent }), secs, f.verb.is_some());
    let hits: Vec<Hit> = if f.neg { hits.iter().copied().filter(|h| h.e.x.neg).chain(hits.iter().copied().filter(|h| !h.e.x.neg)).collect() } else { hits };
    let Some(h) = hits.first().copied() else {
        // state: «why was X <status>» — the cause of the attribute
        if let Some(c) = subj.first().copied() {
            let rest_l = lem_all(&content(&f.rest));
            for fl in [Field::Status, Field::Trait, Field::Goal, Field::Life, Field::Freedom, Field::Sleep] {
                for (st, v) in w.history(c, fl) {
                    let vs = match &v {
                        Val::Known(x) => x.to_string(),
                        Val::Unknown => continue,
                    };
                    if rest_l.iter().any(|ql| toks(&vs).iter().any(|x| ql.contains(x)))
                        && let Some(e) = st.cause.and_then(|c| w.event(c))
                    {
                        let t = e.why.clone().unwrap_or_else(|| render(w, e));
                        return ans(f, t, format!("{}.{} ← {}", w.label(c), fl.name(), e.id), Some(sent_text(w, st.pos.sent)));
                    }
                }
            }
        }
        // reader v2: the subject's beliefs with the question words («why was harold jealous» → «the people liked…»)
        let words: Vec<String> = f.subj.iter().chain(main_words.iter()).cloned().collect();
        if let Some((_, s, t, id)) = find_minds(w, &words, subj, secs, &[Reg::B]).into_iter().next() {
            return ans(f, t, format!("{id} believe"), Some(sent_text(w, s)));
        }
        // reader v2: «why did X love/marry/choose Y» — Y's traits
        if f.verb.as_deref().is_some_and(|v| matches!(lemmas(v).first().map(String::as_str), Some("love" | "loved" | "like" | "liked" | "admire" | "admired" | "marry" | "married" | "choose" | "chose" | "prefer" | "preferred" | "fall" | "fell")))
            && let Some(y) = resolve_np(w, &f.rest, &[Reg::C]).first().copied()
            && let Some(t) = w.now(y).and_then(|s| text_field(&s, Field::Trait))
        {
            return ans(f, format!("{} was {t}", name_of(w, y)), format!("{}.trait", w.label(y)), None);
        }
        return none(f, "question event not found", anchor_ev.map(|h| sent_text(w, h.e.pos.sent)));
    };
    let e = h.e;
    if let Some(y) = &e.why {
        return ans(f, y.clone(), format!("{} why {}", w.label(e.id), e.anchors()), Some(sent_text(w, e.pos.sent)));
    }
    // reader v2: how with a causal word («easily, the hole was near the ground»)
    if let Some(hw) = &e.how
        && toks(hw).iter().any(|t| matches!(t.as_str(), "because" | "since" | "so" | "as" | "for"))
    {
        return ans(f, hw.clone(), format!("{} how {}", w.label(e.id), e.anchors()), Some(sent_text(w, e.pos.sent)));
    }
    if let Some(c) = e.x.cause.and_then(|c| w.event(c)) {
        let t = c.why.clone().map(|y| format!("{}, {y}", render(w, c))).unwrap_or_else(|| render(w, c));
        return ans(f, t, format!("{} ← cause {}", e.id, c.id), Some(sent_text(w, e.pos.sent)));
    }
    // reader v2: the agent's beliefs in the sections with the question words
    if let Some(a) = e.role(Role::Agent)
        && let Some((_, s, t, id)) = find_minds(w, main_words, &[a.id], secs, &[Reg::B]).into_iter().next()
    {
        return ans(f, t, format!("{id} believe of agent {}", w.label(a.id)), Some(sent_text(w, s)));
    }
    // the agent's goal at that time
    if let Some(a) = e.role(Role::Agent)
        && let Some((v, s)) = w.state_at(a.id, e.pos.sent)
        && let Some(g) = text_field(&s, Field::Goal)
    {
        return ans(f, g, format!("{}@{v} goal at {}", w.label(a.id), e.anchors()), Some(sent_text(w, e.pos.sent)));
    }
    // the intent fulfilled by the event
    if let Some(p) = w.ents(Reg::P).find(|p| p.now().id(Field::By) == Some(e.id))
        && let Some(t) = text_field(&p.now(), Field::What)
    {
        return ans(f, t, format!("{} fulfils {}", e.id, p.id), Some(sent_text(w, e.pos.sent)));
    }
    none(f, format!("{}: no why, cause or goal", w.label(e.id)), Some(sent_text(w, e.pos.sent)))
}

// ── baseline and metric ───────────────────────────────────────────────────────────────────────────────────────

/// Baseline: the sentence of the question's sections with the largest content-lemma overlap with the question (tie — the earlier one).
pub fn baseline(w: &World, q: &Qa) -> (u16, String) {
    let ql: Vec<Vec<String>> = content(&toks(&q.question)).iter().map(|x| lemmas(x)).collect();
    let mut best = (0.0f64, 0u16);
    for (i, (p, s)) in w.text.sents.iter().enumerate() {
        if !q.secs.contains(p) {
            continue;
        }
        let sl: BTreeSet<String> = content(&toks(s)).iter().flat_map(|x| lemmas(x)).collect();
        let m = ql.iter().filter(|l| l.iter().any(|x| sl.contains(x))).count() as f64;
        if m > best.0 || best.1 == 0 {
            best = (m, i as u16 + 1);
        }
    }
    if best.1 == 0 {
        return (0, String::new());
    }
    sent_text(w, best.1)
}

fn lcs(a: &[String], b: &[String]) -> usize {
    let mut dp = vec![0usize; b.len() + 1];
    for x in a {
        let mut prev = 0;
        for (j, y) in b.iter().enumerate() {
            let tmp = dp[j + 1];
            dp[j + 1] = if x == y { prev + 1 } else { dp[j + 1].max(dp[j]) };
            prev = tmp;
        }
    }
    dp[b.len()]
}

/// ROUGE-L F1 (β = 1) on `toks` tokens; an empty hypothesis scores 0.
pub fn rouge_l(hyp: &str, reference: &str) -> f64 {
    let (h, r) = (toks(hyp), toks(reference));
    if h.is_empty() || r.is_empty() {
        return 0.0;
    }
    let l = lcs(&h, &r) as f64;
    if l == 0.0 {
        return 0.0;
    }
    let (p, rc) = (l / h.len() as f64, l / r.len() as f64);
    2.0 * p * rc / (p + rc)
}

/// Best ROUGE-L over the question's references (answer1, answer2), as in the paper.
pub fn rouge_q(hyp: &str, q: &Qa) -> f64 {
    [&q.a1, &q.a2].iter().filter(|r| !r.trim().is_empty()).map(|r| rouge_l(hyp, r)).fold(0.0, f64::max)
}
