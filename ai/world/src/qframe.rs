//! Parsing a question from a UD tree (`en`) for the reader: question word, question predicate, participants (subject,
//! object, adverbials with a preposition), subordinate part ("what happened **when** …" — a separate clause whose
//! event the reader looks for first), negation, future, passive. From this follows the question type — which index field
//! to look at (`QKind`) — and the role of the question word (`Focus`).
//!
//! FairytaleQA questions are lowercase, so the tagger sometimes takes a verb for a noun ("the princess
//! *return* to"). The repair uses the `en` dictionary in code: the predicate is the first word after the auxiliary that can be
//! a verb (as in the v1 reader rules), and only when the tree did not give a verbal predicate.

use crate::ans::{lemmas, toks};
use crate::tree::{PTag, Rel, Tree, UPos, annotator};

/// Question word.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wh {
    Who,
    Whose,
    What,
    Where,
    When,
    Why,
    How,
    HowMany,
    None,
}

/// Question type for the reader — which index field to look at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QKind {
    /// feeling: feeling (tag set) at the anchor time
    Feel,
    /// why: event's why, cause event, goal, intention, belief
    Why,
    /// what happened: consequences (cause=) or following events
    Happen,
    /// what happened to X
    HappenTo,
    /// what X did: an event with agent X
    Do,
    /// what X said: the utterance's means
    Say,
    /// what X wanted: goal, intention
    Want,
    /// what X thought: belief
    Think,
    /// what (object): patient / instr / how; "have" — items and parts
    Obj,
    /// who (subject): agent
    WhoSubj,
    /// whom / to whom: patient / to
    WhoObj,
    /// who is X: character's name
    WhoIs,
    /// what (subject): agent or cause event
    WhatSubj,
    /// what X is like: traits, status, parts, name
    Describe,
    /// where: event's at/to, character's place, place name and ancestors
    Where,
    /// when: when, cursor, cause event
    When,
    /// how: how, instr, cause event
    How,
    /// how many: num
    Num,
    Generic,
}

impl QKind {
    pub fn name(self) -> &'static str {
        match self {
            QKind::Feel => "feel",
            QKind::Why => "why",
            QKind::Happen => "happen",
            QKind::HappenTo => "happen-to",
            QKind::Do => "do",
            QKind::Say => "say",
            QKind::Want => "want",
            QKind::Think => "think",
            QKind::Obj => "obj",
            QKind::WhoSubj => "who",
            QKind::WhoObj => "who-obj",
            QKind::WhoIs => "who-is",
            QKind::WhatSubj => "what-subj",
            QKind::Describe => "describe",
            QKind::Where => "where",
            QKind::When => "when",
            QKind::How => "how",
            QKind::Num => "num",
            QKind::Generic => "generic",
        }
    }
}

/// Role of the question word relative to the predicate.
#[derive(Clone, Debug, PartialEq)]
pub enum Focus {
    Subj,
    Obj,
    /// adverbial with a preposition ("to whom", "return to ?")
    Obl(String),
    /// where / when / why / how
    Adv,
    /// nominal part ("who was the youngest son")
    Attr,
    None,
}

/// Clause: predicate, participants, content words.
#[derive(Clone, Debug, Default)]
pub struct Clause {
    pub t: Tree,
    pub pred: Option<usize>,
    /// predicate lemma ("be" — for a nominal one without another verb)
    pub lemma: String,
    /// nominal predicate: be + noun/adjective (lemma is the nominal part)
    pub cop: bool,
    /// lemmas of xcomp/ccomp verbs of the predicate ("tried to spin" → spin)
    pub xcomp: Vec<String>,
    pub subj: Vec<String>,
    pub obj: Vec<String>,
    /// preposition and phrase
    pub obl: Vec<(String, Vec<String>)>,
    pub neg: bool,
    /// content words of the clause (forms), without the question word, auxiliaries and the subject
    pub content: Vec<String>,
}

impl Clause {
    /// Predicate lemmas with xcomp: for matching against the event verb.
    pub fn preds(&self) -> Vec<String> {
        let mut v = Vec::new();
        if !self.lemma.is_empty() && self.lemma != "be" && self.lemma != "do" && self.lemma != "have" {
            v.push(self.lemma.clone());
        }
        for x in &self.xcomp {
            if !v.contains(x) {
                v.push(x.clone());
            }
        }
        if v.is_empty() && !self.lemma.is_empty() {
            v.push(self.lemma.clone());
        }
        v
    }
}

/// Parsed question.
#[derive(Clone, Debug)]
pub struct QFrame {
    pub q: String,
    pub wh: Wh,
    pub kind: QKind,
    pub focus: Focus,
    pub main: Clause,
    /// subordinating conjunction and the subordinate clause ("time anchor")
    pub mark: Option<String>,
    pub sub: Option<Clause>,
    pub fut: bool,
    pub passive: bool,
    /// preposition at the end ("return to ?", "left with ?")
    pub prep: Option<String>,
    /// «about X» / «towards X»
    pub about: Vec<String>,
    /// «happen to X»
    pub target: Vec<String>,
    /// noun in the question-word phrase ("how many *fairies*", "what *job* did …")
    pub whnp: Vec<String>,
}

const SUBS: &[&str] = &["when", "after", "before", "because", "while", "if", "since", "until", "till", "once", "whenever", "though", "although", "as"];
pub const FEEL_L: &[&str] = &["feel", "react", "respond"];
pub const SAY_L: &[&str] = &[
    "say", "tell", "ask", "answer", "reply", "shout", "whisper", "promise", "order", "command", "beg", "warn", "suggest", "advise", "announce",
    "exclaim", "request", "explain", "declare", "cry", "call", "sing", "speak", "offer", "threaten", "propose", "demand", "scream",
];
pub const WANT_L: &[&str] = &["want", "wish", "need", "hope", "plan", "decide", "intend", "desire", "try", "long", "mean", "resolve", "determine"];
pub const THINK_L: &[&str] = &[
    "think", "believe", "know", "realize", "realise", "learn", "discover", "notice", "understand", "suppose", "suspect", "guess", "remember",
    "forget", "imagine", "expect", "assume", "conclude", "figure", "see",
];
const AUXES: &[&str] = &["do", "be", "have", "will", "would", "shall", "should", "can", "could", "may", "might", "must"];
const DETS: &[&str] = &["the", "a", "an", "his", "her", "their", "its", "my", "your", "our", "this", "that", "these", "those", "some", "every", "each", "all", "both", "no", "any", "'s", "s"];

/// Can be a verb according to the `en` dictionary (share of VB* tags ≥ 5% or ≥ 3 occurrences).
pub fn can_verb(w: &str) -> bool {
    let Some(f) = en::dict::form(w) else { return w.ends_with("ed") && w.len() > 4 };
    let (mut tot, mut hit) = (0u32, 0u32);
    for r in en::dict::analyses(f) {
        tot += r.count;
        if r.tag.is_verb() {
            hit += r.count.max(1);
        }
    }
    hit > 0 && (tot == 0 || hit * 20 >= tot || hit >= 3)
}

fn wh_of(w: &str) -> Option<Wh> {
    Some(match w {
        "who" | "whom" => Wh::Who,
        "whose" => Wh::Whose,
        "what" | "which" => Wh::What,
        "where" => Wh::Where,
        "when" => Wh::When,
        "why" => Wh::Why,
        "how" => Wh::How,
        _ => return None,
    })
}

/// Question tokens: `en::tok`, lowercase, without the final "?".
fn tokens(q: &str) -> Vec<String> {
    let Ok(a) = annotator() else { return toks(q) };
    let mut v: Vec<String> = a.tokenize(q).into_iter().map(|t| t.form.to_lowercase()).collect();
    while v.last().is_some_and(|x| x == "?" || x == "." || x == "!") {
        v.pop();
    }
    v
}

/// The question's auxiliary verb and the forms of the main verb it requires.
fn aux_want(a: &str) -> Option<&'static [PTag]> {
    Some(match a {
        "did" | "do" | "does" | "will" | "would" | "shall" | "should" | "can" | "could" | "may" | "might" | "must" => &[PTag::VB],
        "was" | "were" | "is" | "are" | "am" | "been" => &[PTag::VBG, PTag::VBN],
        "had" | "has" | "have" => &[PTag::VBN],
        _ => return None,
    })
}

/// The word can be a verb in one of the forms in `want` according to the `en` dictionary (share ≥ 5% or ≥ 3 occurrences).
pub fn verb_form(w: &str, want: &[PTag]) -> bool {
    let Some(f) = en::dict::form(w) else { return want.contains(&PTag::VBN) && w.ends_with("ed") && w.len() > 4 };
    let (mut tot, mut hit) = (0u32, 0u32);
    for r in en::dict::analyses(f) {
        tot += r.count;
        if want.contains(&r.tag) {
            hit += r.count.max(1);
        }
    }
    hit > 0 && (tot == 0 || hit * 20 >= tot || hit >= 3)
}

/// Share of verbal analyses of a form in the `en` dictionary.
fn verb_share(w: &str) -> f64 {
    let Some(f) = en::dict::form(w) else { return 0.5 };
    let (mut tot, mut hit) = (0u32, 0u32);
    for r in en::dict::analyses(f) {
        tot += r.count;
        if r.tag.is_verb() {
            hit += r.count;
        }
    }
    if tot == 0 { 0.5 } else { hit as f64 / tot as f64 }
}

/// Parse a question. The `en` model is required (`tree::annotator`); without it — an empty frame with `Generic`.
///
/// 1. Subordinate clause ("when/after/because/… X") — separately, with its own tree.
/// 2. Main part: question word, auxiliary (did/was/had/will…), predicate — the root of the tree, and if
///    the tree did not give a verb — the first word after the subject that can be a verb of the required form.
/// 3. Participants of the main part — from the tree of the **declarative** sentence "subject + predicate + rest"
///    ("where did X return to" → "X return to"): the tagger knows such sentences better than questions.
pub fn parse(q: &str) -> QFrame {
    let t = tokens(q);
    let mut cut = None;
    for i in 2..t.len() {
        let w = t[i].as_str();
        if !SUBS.contains(&w) || (w == "once" && t.get(i + 1).is_some_and(|x| x == "upon")) {
            continue;
        }
        if w == "as" && !(t.get(i + 1).is_some_and(|x| x == "soon" || x == "long") && t.get(i + 2).is_some_and(|x| x == "as")) {
            continue;
        }
        cut = Some(i);
        break;
    }
    let (m, mark, sub_t) = match cut {
        Some(i) => {
            let (mk, rest) = if t[i] == "as" { ("as soon as".to_string(), i + 3) } else { (t[i].clone(), i + 1) };
            (t[..i].to_vec(), Some(mk), t[rest.min(t.len())..].to_vec())
        }
        None => (t.clone(), None, Vec::new()),
    };
    let empty = |mark| QFrame { q: q.into(), wh: Wh::None, kind: QKind::Generic, focus: Focus::None, main: Clause::default(), mark, sub: None, fut: false, passive: false, prep: None, about: Vec::new(), target: Vec::new(), whnp: Vec::new() };
    let Ok(a) = annotator() else { return empty(mark) };
    let sub = (!sub_t.is_empty()).then(|| {
        let mut f = sub_t.clone();
        f.push(".".into());
        let tr = Tree::of_forms(a, &f);
        let r = tr.root();
        decl_clause(tr, r)
    });
    // question word: one of the first three ("to whom", "for what reason")
    let Some(wi) = (0..m.len().min(3)).find(|i| wh_of(&m[*i]).is_some()) else {
        let mut f = m.clone();
        f.push("?".into());
        let tr = Tree::of_forms(a, &f);
        let r = tr.root();
        let mut fr = empty(mark);
        fr.main = decl_clause(tr, r);
        fr.sub = sub;
        return fr;
    };
    let mut wh = wh_of(&m[wi]).unwrap();
    let mut wend = wi + 1;
    if wh == Wh::How && m.get(wi + 1).is_some_and(|x| x == "many" || x == "much") {
        wh = Wh::HowMany;
        wend = wi + 2;
    }
    // "what job / what kind of / which prince did …": noun in the question-word phrase
    let mut ai = wend;
    if matches!(wh, Wh::What | Wh::Whose | Wh::HowMany) {
        if let Some(k) = (wend..m.len().min(wend + 4)).find(|k| aux_want(&m[*k]).is_some()) {
            ai = k;
        }
    }
    let aux = m.get(ai).filter(|x| aux_want(x).is_some()).cloned();
    let be = aux.as_deref().is_some_and(|x| matches!(x, "was" | "were" | "is" | "are" | "am"));
    let fut = aux.as_deref().is_some_and(|x| matches!(x, "will" | "shall" | "would")) || m.windows(2).any(|w| w[0] == "going" && w[1] == "to");
    let neg = m.iter().any(|x| matches!(x.as_str(), "not" | "n't" | "never"));
    // predicate: candidates are words after the auxiliary that can be a verb of the required form (dictionary
    // `en`); check — the tree of the declarative "subject + predicate + rest": there the candidate must be VERB
    let mut p: Option<usize> = None;
    if let Some(want) = aux.as_deref().and_then(aux_want) {
        let do_aux = matches!(aux.as_deref(), Some("did" | "do" | "does"));
        let mut cands = Vec::new();
        for j in ai + 1..m.len() {
            let f = m[j].as_str();
            if matches!(f, "who" | "which" | "that" | "whom" | "whose") && j > ai + 1 {
                break; // relative clause — the predicate is not there
            }
            if DETS.contains(&f) || matches!(f, "not" | "n't" | "never" | "he" | "she" | "they" | "it" | "i" | "you" | "we" | "him" | "them" | "there" | "also" | "just" | "ever" | "still" | "really" | "only" | "then" | "so") {
                continue;
            }
            if be && ADJ_PRED.contains(&f) {
                break; // "was X able/ready to V" — nominal predicate
            }
            if do_aux && j == ai + 1 {
                continue; // did + subject + verb: right after did comes the subject
            }
            let after_det = j > ai + 1 && DETS.contains(&m[j - 1].as_str());
            if !after_det && verb_form(f, want) {
                cands.push(j);
            }
        }
        // "plan to *do*", "tell X to *buy*": the verb after "to" is the xcomp of the previous candidate
        let first = cands.first().copied();
        cands.retain(|j| Some(*j) == first || m[*j - 1] != "to");
        let subj_of = |j: usize| -> Vec<String> { m[ai + 1..j].to_vec() };
        // checked candidates; among them — the first that is more often a verb (≥ 30% of analyses), otherwise the first
        let ok: Vec<usize> = cands
            .iter()
            .copied()
            .filter(|j| {
                let (tr, v) = decl_tree(a, &subj_of(*j), aux.as_deref(), be, &m[*j..]);
                tr.len() > v && (tr.upos(v) == UPos::VERB || (tr.upos(v) == UPos::AUX && !matches!(tr.lemma(v), "be" | "have" | "do")))
            })
            .collect();
        // verbal context: followed by "to", a determiner, an object pronoun, a preposition (plan *to*, tell *the* king)
        let verb_ctx = |j: usize| m.get(j + 1).is_some_and(|x| DETS.contains(&x.as_str()) || matches!(x.as_str(), "to" | "him" | "them" | "it" | "me" | "us" | "up" | "out" | "down" | "off" | "away" | "back" | "into" | "on" | "in" | "at" | "for" | "with" | "about" | "over" | "through"));
        p = ok.iter().copied().find(|j| verb_share(&m[*j]) >= 0.3 || verb_ctx(*j)).or_else(|| ok.first().copied()).or_else(|| cands.first().copied());
        // "who had seven sons", "what did the trick": the auxiliary is the predicate itself, the subject is the question word
        if p.is_none() && matches!(wh, Wh::Who | Wh::What) && !be && matches!(aux.as_deref(), Some("had" | "has" | "have" | "did" | "does" | "do")) && ai == wend {
            p = Some(ai);
        }
    } else if m.len() > wend {
        // no auxiliary: "who appeared", "what happened", "who owned …"
        p = Some(wend);
    }
    let prep_before = wi == 1 && matches!(m[0].as_str(), "to" | "for" | "with" | "from" | "by" | "in" | "at" | "on" | "of");
    let lastw = m.last().cloned().unwrap_or_default();
    let prep = (m.len() > ai + 1 && matches!(lastw.as_str(), "to" | "for" | "with" | "from" | "by" | "in" | "at" | "on" | "of" | "into" | "about" | "after" | "like" | "through" | "under" | "upon" | "onto" | "towards" | "toward"))
        .then(|| lastw.clone());
    // nominal question: be without a verb ("who was the youngest son", "why was the princess ready …")
    let (main, focus, passive) = match p {
        Some(p) if p == ai && aux.is_some() => {
            // auxiliary as predicate: "who had seven sons" → "someone had seven sons"
            let mut f = vec!["someone".to_string()];
            f.extend(m[ai..].iter().cloned());
            f.push(".".into());
            let tr = Tree::of_forms(a, &f);
            let r = tr.root();
            let mut c = decl_clause(tr, r);
            c.subj.clear();
            (c, Focus::Subj, false)
        }
        Some(p) => {
            let subj: Vec<String> = if aux.is_some() { m[ai + 1..p].iter().filter(|x| !matches!(x.as_str(), "not" | "n't" | "never")).cloned().collect() } else { Vec::new() };
            let focus = if matches!(wh, Wh::Where | Wh::When | Wh::Why | Wh::How | Wh::HowMany) {
                Focus::Adv
            } else if prep_before {
                Focus::Obl(m[0].clone())
            } else if subj.is_empty() {
                Focus::Subj
            } else if let Some(pp) = &prep {
                Focus::Obl(pp.clone())
            } else {
                Focus::Obj
            };
            let passive = be && verb_form(&m[p], &[PTag::VBN]) && !verb_form(&m[p], &[PTag::VBG]);
            // declarative: subject (or "someone") + predicate + rest; the predicate is the same token
            let (tr, vpos) = decl_tree(a, &subj, aux.as_deref(), be, &m[p..]);
            let mut c = decl_clause(tr, Some(vpos));
            // subject — the words between the auxiliary and the predicate (more reliable than the tagger's nsubj)
            c.subj = subj.clone();
            let ns = if subj.is_empty() { 1 } else { subj.len() };
            c.content = c.t.w.iter().enumerate().filter(|(i, _)| *i >= ns).map(|(_, x)| x.form.clone()).filter(|x| c.content.contains(x)).collect();
            (c, focus, passive)
        }
        None => {
            // be + noun phrase (+ adjective/preposition): "who was X", "where was X", "why was X ready to V"
            let rest: Vec<String> = m[(ai + 1).min(m.len())..].iter().filter(|x| !matches!(x.as_str(), "not" | "n't" | "never")).cloned().collect();
            // "X was …": the auxiliary is inserted after the subject phrase — the first position where the tree gives
            // a nominal predicate with cop and nsubj
            let mut best: Option<Tree> = None;
            if let Some(x) = aux.as_deref().filter(|_| be) {
                for k in 1..rest.len() {
                    let mut f: Vec<String> = rest[..k].to_vec();
                    f.push(x.to_string());
                    f.extend(rest[k..].iter().cloned());
                    f.push(".".into());
                    let tr = Tree::of_forms(a, &f);
                    if let Some(r) = tr.root()
                        && r > k
                        && tr.kids(r).iter().any(|c| *c == k && tr.base(*c) == Rel::Cop)
                        && tr.kids(r).iter().any(|c| tr.base(*c) == Rel::Nsubj)
                    {
                        best = Some(tr);
                        break;
                    }
                }
            }
            let tr = best.unwrap_or_else(|| {
                let mut f = rest.clone();
                f.push(".".into());
                Tree::of_forms(a, &f)
            });
            let r = tr.root();
            let mut c = decl_clause(tr, r);
            c.cop = true;
            if !c.content.is_empty() && c.subj.is_empty() {
                // "who was the youngest son": the whole phrase is the subject-description
                c.subj = rest.clone();
            }
            let focus = if matches!(wh, Wh::Who | Wh::What | Wh::Whose) { Focus::Attr } else { Focus::Adv };
            (c, focus, false)
        }
    };
    let mut main = main;
    main.neg = neg;
    let mut fr = frame(q, wh, main, focus, fut, passive, prep, mark, sub, &m);
    fr.whnp = if ai > wend { m[wend..ai].iter().filter(|x| !matches!(x.as_str(), "of" | "kind" | "type" | "sort")).cloned().collect() } else if wh == Wh::HowMany && aux.is_none() { m[wend..].iter().take_while(|x| !can_verb(x)).cloned().collect() } else { Vec::new() };
    fr
}

/// Clause of a declarative sentence: predicate `p` (or the root), subject, object, adverbials, xcomp.
fn decl_clause(t: Tree, p: Option<usize>) -> Clause {
    let mut c = Clause { t: t.clone(), ..Default::default() };
    let Some(p) = p.or_else(|| t.root()) else { return c };
    if p >= t.len() {
        return c;
    }
    c.pred = Some(p);
    c.cop = !t.kids_of(p, Rel::Cop).is_empty();
    c.lemma = if c.cop || matches!(t.upos(p), UPos::ADJ | UPos::NOUN | UPos::PROPN) && !can_verb(t.form(p)) { t.lemma(p).to_string() } else if matches!(t.upos(p), UPos::VERB | UPos::AUX) { t.lemma(p).to_string() } else { first_lemma(t.form(p)) };
    for k in t.kids(p) {
        if matches!(t.base(*k), Rel::Xcomp | Rel::Ccomp) && matches!(t.upos(*k), UPos::VERB | UPos::ADJ) {
            c.xcomp.push(t.lemma(*k).to_string());
        }
    }
    let subj_ix: Vec<usize> = t.kids(p).iter().copied().find(|k| t.base(*k) == Rel::Nsubj).map(|s| t.sub(s, &[])).unwrap_or_default();
    c.subj = subj_ix.iter().map(|i| t.form(*i).to_string()).collect();
    if let Some(o) = t.kids(p).iter().copied().find(|k| t.base(*k) == Rel::Obj) {
        c.obj = t.sub(o, &[]).into_iter().map(|i| t.form(i).to_string()).collect();
    }
    for k in t.kids(p) {
        if matches!(t.base(*k), Rel::Obl | Rel::Iobj) {
            let case = t.kids_of(*k, Rel::Case).first().map(|x| t.form(*x).to_string()).unwrap_or_default();
            let words: Vec<String> = t.sub(*k, &[]).into_iter().filter(|i| t.base(*i) != Rel::Case).map(|i| t.form(i).to_string()).collect();
            c.obl.push((case, words));
        }
    }
    c.neg = t.w.iter().any(|x| matches!(x.lemma.as_str(), "not" | "never"));
    for i in 0..t.len() {
        if subj_ix.contains(&i) || !(t.content(i) || i == p) {
            continue;
        }
        let f = t.form(i);
        if matches!(t.upos(i), UPos::AUX) || matches!(f, "?" | "." | "not" | "n't" | "never" | "someone" | "something") || (i != p && AUXES.contains(&t.lemma(i)) && t.upos(i) != UPos::NOUN) {
            continue;
        }
        c.content.push(f.to_string());
    }
    c
}

/// Verb lemma according to the `en` dictionary: the most frequent verbal analysis of the form; if none — the first lemma.
fn first_lemma(w: &str) -> String {
    if let Some(f) = en::dict::form(w)
        && let Some(r) = en::dict::analyses(f).filter(|r| r.tag.is_verb()).max_by_key(|r| r.count)
    {
        return en::dict::text(r.lemma()).to_lowercase();
    }
    lemmas(w).into_iter().nth(1).unwrap_or_else(|| w.to_string())
}

/// Tree of a declarative sentence: subject (or "someone") + auxiliary (be, modals, had) + predicate and the rest.
/// The second value is the position of the predicate.
fn decl_tree(a: &en::annotate::Annotator, subj: &[String], aux: Option<&str>, be: bool, rest: &[String]) -> (Tree, usize) {
    let mut f: Vec<String> = if subj.is_empty() { vec!["someone".into()] } else { subj.to_vec() };
    if let Some(x) = aux
        && (be || matches!(x, "will" | "would" | "could" | "can" | "should" | "must" | "might" | "may" | "had" | "has" | "have" | "shall" | "did" | "does" | "do"))
    {
        f.push(x.to_string());
    }
    let v = f.len();
    f.extend(rest.iter().cloned());
    f.push(".".into());
    (Tree::of_forms(a, &f), v)
}

/// Predicative adjectives after be ("was X able to …").
const ADJ_PRED: &[&str] = &[
    "able", "unable", "ready", "willing", "afraid", "glad", "happy", "sad", "angry", "sure", "eager", "anxious", "worried", "surprised", "sorry",
    "jealous", "proud", "scared", "upset", "pleased", "interested",
];

#[allow(clippy::too_many_arguments)]
fn frame(q: &str, wh: Wh, mut main: Clause, focus: Focus, fut: bool, passive: bool, prep: Option<String>, mark: Option<String>, sub: Option<Clause>, m: &[String]) -> QFrame {
    let has = |xs: &[&str]| m.iter().any(|f| xs.contains(&f.as_str()));
    let lemma = main.lemma.clone();
    let about = m.iter().position(|w| w == "about" || w == "towards" || w == "toward").map(|i| m[i + 1..].to_vec()).unwrap_or_default();
    let target = if lemma == "happen" { m.iter().position(|w| w == "to").map(|i| m[i + 1..].to_vec()).unwrap_or_default() } else { Vec::new() };
    let kind_of = m.windows(3).any(|w| (w[0] == "what" || w[0] == "which") && matches!(w[1].as_str(), "kind" | "type" | "sort") && w[2] == "of");
    let special = has(&["special"]) || m.windows(2).any(|w| w[1] == "like" && matches!(w[0].as_str(), "look" | "looked" | "was" | "were" | "is" | "are" | "seem" | "seemed"));
    let feel = FEEL_L.contains(&lemma.as_str()) || has(&["feeling", "feelings", "felt"]);
    // "what made/caused X V" — why X V: the event is the xcomp, the subject is the object of make
    if wh == Wh::What && focus == Focus::Subj && matches!(lemma.as_str(), "make" | "cause" | "lead" | "force" | "drive") {
        let t = main.t.clone();
        if let Some(p) = main.pred
            && let Some(x) = t.kids(p).iter().copied().find(|k| matches!(t.base(*k), Rel::Xcomp | Rel::Ccomp))
        {
            let obj = main.obj.clone();
            let inner = decl_clause(t.clone(), Some(x));
            main = Clause { subj: obj, neg: main.neg, ..inner };
            let kind = if FEEL_L.contains(&main.lemma.as_str()) { QKind::Feel } else { QKind::Why };
            return QFrame { q: q.into(), wh, kind, focus: Focus::Adv, main, mark, sub, fut, passive: false, prep, about, target, whnp: Vec::new() };
        }
    }
    let kind = match wh {
        Wh::Why => QKind::Why,
        Wh::HowMany => QKind::Num,
        Wh::How if feel => QKind::Feel,
        Wh::What if feel && focus != Focus::Subj => QKind::Feel,
        Wh::What | Wh::Who | Wh::Whose if kind_of || special => QKind::Describe,
        Wh::What if lemma == "happen" => {
            if target.is_empty() {
                QKind::Happen
            } else {
                QKind::HappenTo
            }
        }
        Wh::What if lemma == "do" && focus != Focus::Subj => QKind::Do,
        // «what was X able to do»
        Wh::What if main.cop && main.xcomp.iter().any(|x| x == "do") => QKind::Do,
        Wh::What if SAY_L.contains(&lemma.as_str()) && focus != Focus::Subj => QKind::Say,
        Wh::What if WANT_L.contains(&lemma.as_str()) && focus != Focus::Subj => QKind::Want,
        Wh::What if THINK_L.contains(&lemma.as_str()) && focus != Focus::Subj && lemma != "see" => QKind::Think,
        Wh::Who | Wh::Whose => match focus {
            Focus::Attr => QKind::WhoIs,
            Focus::Subj => QKind::WhoSubj,
            _ => QKind::WhoObj,
        },
        Wh::What => match focus {
            Focus::Subj => QKind::WhatSubj,
            Focus::Attr => QKind::Describe,
            _ => QKind::Obj,
        },
        Wh::Where => QKind::Where,
        Wh::When => QKind::When,
        Wh::How => {
            if main.cop && main.pred.is_some_and(|p| main.t.upos(p) != UPos::VERB) && !has(&["able", "unable"]) {
                QKind::Describe
            } else {
                QKind::How
            }
        }
        Wh::None => QKind::Generic,
    };
    QFrame { q: q.into(), wh, kind, focus, main, mark, sub, fut, passive, prep, about, target, whnp: Vec::new() }
}

/// Frame in one line (for development and the answer log).
pub fn show(f: &QFrame) -> String {
    let mut s = format!("{} {:?}", f.kind.name(), f.focus);
    if f.fut {
        s.push_str(" fut");
    }
    if f.main.neg {
        s.push_str(" neg");
    }
    if f.passive {
        s.push_str(" pass");
    }
    s.push_str(&format!(" | pred {}{} | subj «{}» | obj «{}»", f.main.lemma, if f.main.cop { "(cop)" } else { "" }, f.main.subj.join(" "), f.main.obj.join(" ")));
    for (c, w) in &f.main.obl {
        s.push_str(&format!(" | {c} «{}»", w.join(" ")));
    }
    if !f.main.xcomp.is_empty() {
        s.push_str(&format!(" | x {}", f.main.xcomp.join(",")));
    }
    s.push_str(&format!(" | c «{}»", f.main.content.join(" ")));
    if let Some(p) = &f.prep {
        s.push_str(&format!(" | prep {p}"));
    }
    if !f.whnp.is_empty() {
        s.push_str(&format!(" | wh-np «{}»", f.whnp.join(" ")));
    }
    if let (Some(m), Some(c)) = (&f.mark, &f.sub) {
        s.push_str(&format!(" || {m}: pred {} | subj «{}» | obj «{}» | c «{}»", c.lemma, c.subj.join(" "), c.obj.join(" "), c.content.join(" ")));
    }
    s
}
