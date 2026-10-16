//! Span of the anchor sentence for a question. FairytaleQA answers to explicit questions are text spans, so for
//! explicit questions the reader takes not only the fact field but also the span of the sentence the fact is bound to:
//! UD tree of the sentence → predicate node (lemma from the question or from the fact) → subtree of the question word's role
//! (subject, object, place, time, reason, manner, feeling, consequence, words).
//!
//! The span rule is a name in the log (`subj`, `obl:loc`, `advcl:because`…), so it is visible why exactly this span.

use std::collections::HashSet;

use crate::ans::lemmas;
use crate::qframe::{QFrame, QKind};
use crate::tree::{PTag, Rel, Tree, UPos};

/// Which part of the sentence to take.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Want {
    Subj,
    Obj,
    Where,
    When,
    Why,
    How,
    Feel,
    /// predicate clause without the subject (what they did)
    Do,
    /// predicate clause with the subject (what happened)
    Clause,
    /// main part or the next clause after the anchor subordinate clause (what happened when …)
    After,
    Say,
    Num,
    Describe,
}

impl Want {
    pub fn name(self) -> &'static str {
        match self {
            Want::Subj => "subj",
            Want::Obj => "obj",
            Want::Where => "where",
            Want::When => "when",
            Want::Why => "why",
            Want::How => "how",
            Want::Feel => "feel",
            Want::Do => "do",
            Want::Clause => "clause",
            Want::After => "after",
            Want::Say => "say",
            Want::Num => "num",
            Want::Describe => "describe",
        }
    }
}

/// Sentence part by question type.
pub fn want_of(fr: &QFrame) -> Want {
    match fr.kind {
        QKind::Feel => Want::Feel,
        QKind::Why => Want::Why,
        QKind::Happen | QKind::HappenTo => {
            if fr.sub.is_some() {
                Want::After
            } else {
                Want::Clause
            }
        }
        QKind::Do => Want::Do,
        QKind::Say | QKind::Think => Want::Say,
        QKind::Want | QKind::Obj | QKind::WhoObj => Want::Obj,
        QKind::WhoSubj | QKind::WhatSubj | QKind::WhoIs => Want::Subj,
        QKind::Describe => Want::Describe,
        QKind::Where => Want::Where,
        QKind::When => Want::When,
        QKind::How => Want::How,
        QKind::Num => Want::Num,
        QKind::Generic => Want::Clause,
    }
}

const LOC_CASE: &[&str] = &[
    "in", "at", "to", "into", "on", "under", "near", "by", "from", "over", "through", "across", "behind", "beside", "inside", "towards",
    "toward", "upon", "onto", "round", "around", "along", "among", "amongst", "beneath", "below", "above", "within", "outside", "between",
    "past", "down", "up", "out", "off", "beyond", "against",
];
const LOC_ADV: &[&str] = &["home", "there", "here", "away", "abroad", "out", "back", "down", "up", "inside", "outside", "ashore", "aboard", "far", "off", "downstairs", "upstairs", "everywhere", "somewhere", "nearby", "underground", "overboard"];
const TIME_N: &[&str] = &[
    "day", "night", "morning", "evening", "year", "time", "hour", "week", "month", "spring", "summer", "winter", "autumn", "noon", "midnight",
    "dawn", "sunrise", "sunset", "moment", "minute", "age", "youth", "childhood", "eve", "birthday", "christmas", "while", "season",
    "afternoon", "daybreak", "dusk", "yesterday", "today", "tomorrow", "sunday", "monday", "saturday", "harvest", "festival",
];
const TIME_MARK: &[&str] = &["when", "after", "before", "until", "till", "while", "once", "since", "whenever", "as"];
const WHY_MARK: &[&str] = &["because", "as", "since", "for", "so", "lest", "cause"];
const QUANT: &[&str] = &["many", "much", "few", "several", "some", "all", "every", "each", "both", "no", "any", "more", "most", "other", "such"];
const FEEL_V: &[&str] = &["feel", "look", "become", "grow", "seem", "get", "turn", "appear", "remain", "be", "go", "fall", "find", "make"];
const EMO_N: &[&str] = &[
    "joy", "fear", "anger", "grief", "sorrow", "surprise", "delight", "terror", "rage", "fury", "despair", "shame", "pity", "horror", "wonder",
    "astonishment", "amazement", "dismay", "gladness", "happiness", "sadness", "love", "jealousy", "envy", "pride", "gratitude", "relief",
    "alarm", "fright", "dread", "distress", "disappointment", "satisfaction", "admiration", "curiosity", "excitement", "anxiety", "misery",
];
/// Feeling words (adjectives and participles), besides the closed set `FEELING`: the «feeling» span
/// takes only them — «ill», «old», «done» are not feelings.
const EMO_ADJ: &[&str] = &[
    "glad", "merry", "joyful", "joyous", "cheerful", "delighted", "pleased", "overjoyed", "contented", "satisfied", "thankful", "unhappy",
    "miserable", "sorrowful", "grieved", "heartbroken", "downcast", "gloomy", "melancholy", "mournful", "troubled", "distressed", "anxious",
    "uneasy", "nervous", "alarmed", "frightened", "terrified", "scared", "fearful", "dismayed", "furious", "enraged", "cross", "mad", "indignant",
    "vexed", "irritated", "astonished", "amazed", "startled", "bewildered", "perplexed", "confused", "discouraged", "depressed", "dejected",
    "hopeless", "envious", "sympathetic", "moved", "touched", "charmed", "enchanted", "flattered", "insulted", "humiliated", "frustrated",
    "comforted", "grateful", "impressed", "interested", "fond", "happy", "sad", "wretched", "weary", "exhausted", "sleepy", "hungry",
    "thirsty", "lonely", "homesick", "restless", "nervous", "calm", "brave", "bold", "timid", "shy", "sorry", "grave", "serious", "stunned",
    "aghast", "dumbfounded", "petrified", "horror-stricken", "panic-stricken", "thunderstruck", "awed", "ecstatic", "jubilant", "elated",
];

fn emo(t: &Tree, i: usize) -> bool {
    let w = t.form(i);
    let l = t.lemma(i);
    EMO_ADJ.contains(&w) || EMO_ADJ.contains(&l) || crate::types::FEELING.tag(w).is_ok() || crate::types::FEELING.tag(l).is_ok()
}

/// Extra words at the span edges.
const EDGE: &[&str] = &["and", "but", "or", "so", "then", "that", "because", ",", ";", ":", "\"", "'", "``", "''", "-", "--", ".", "!", "?", "yet", "for", "as", "since", "while", "when"];

/// Predicate node: a lemma from `preds` (order — priority); a verb takes priority over a noun.
pub fn find_pred(t: &Tree, preds: &[String]) -> Option<usize> {
    for p in preds {
        let mut best: Option<(u8, usize)> = None;
        for i in 0..t.len() {
            let hit = t.lemma(i) == p || t.form(i) == p || lemmas(t.form(i)).iter().any(|l| l == p);
            if !hit {
                continue;
            }
            let rank = match t.upos(i) {
                UPos::VERB => 0,
                UPos::ADJ | UPos::AUX => 1,
                UPos::NOUN => 2,
                _ => 3,
            };
            if best.is_none_or(|b| rank < b.0) {
                best = Some((rank, i));
            }
        }
        if let Some((r, i)) = best
            && r < 3
        {
            return Some(i);
        }
    }
    None
}

fn is_pron(t: &Tree, ix: &[usize]) -> bool {
    ix.iter().all(|i| matches!(t.upos(*i), UPos::PRON | UPos::DET | UPos::PUNCT))
}

/// Trim the edges (conjunctions, punctuation); `None` — empty or just an echo of the question.
fn clean(t: &Tree, mut ix: Vec<usize>, qlem: &HashSet<String>) -> Option<Vec<usize>> {
    ix.sort_unstable();
    ix.dedup();
    while ix.first().is_some_and(|i| EDGE.contains(&t.form(*i)) || matches!(t.base(*i), Rel::Cc | Rel::Punct | Rel::Mark)) {
        ix.remove(0);
    }
    while ix.last().is_some_and(|i| EDGE.contains(&t.form(*i)) || matches!(t.base(*i), Rel::Cc | Rel::Punct)) {
        ix.pop();
    }
    let content: Vec<usize> = ix.iter().copied().filter(|i| t.content(*i)).collect();
    if content.is_empty() {
        return None;
    }
    let echo = content.iter().filter(|i| qlem.contains(t.lemma(**i)) || qlem.contains(t.form(**i))).count();
    // echo: the span is mostly the question's own words
    if echo * 2 > content.len() || echo == content.len() {
        return None;
    }
    Some(ix)
}

fn case_of(t: &Tree, i: usize) -> String {
    t.kids_of(i, Rel::Case).first().map(|c| t.form(*c).to_string()).unwrap_or_default()
}

fn mark_of(t: &Tree, i: usize) -> String {
    let ms: Vec<&str> = t.kids(i).iter().filter(|k| t.base(**k) == Rel::Mark).map(|k| t.form(*k)).collect();
    ms.join(" ")
}

/// Children of the predicate and children of its xcomp/conj verbs with the same subject (one «action»).
fn verb_kids(t: &Tree, p: usize) -> Vec<usize> {
    let mut out: Vec<usize> = t.kids(p).to_vec();
    for k in t.kids(p) {
        if matches!(t.base(*k), Rel::Xcomp) {
            out.extend(t.kids(*k).iter().copied());
        }
    }
    out
}

/// Span of sentence `t` for part `want`; `preds` — predicate lemmas (of the question or the fact), `qlem` — lemmas
/// of the question words (for the «echo»). The second value is the rule name.
pub fn extract(t: &Tree, want: Want, preds: &[String], sub_preds: &[String], fr: &QFrame, qlem: &HashSet<String>) -> Option<(String, String)> {
    if t.is_empty() {
        return None;
    }
    let p = find_pred(t, preds);
    let out = |ix: Vec<usize>, rule: &str| -> Option<(String, String)> { clean(t, ix, qlem).map(|ix| (t.text(&ix), rule.to_string())) };
    match want {
        Want::Subj => {
            let p = p?;
            // «there appeared a fellow» / predicate in a relative clause — the head of the modifier
            if t.base(p) == Rel::Acl && t.kids(p).iter().any(|k| t.base(*k) == Rel::Nsubj && matches!(t.form(*k), "who" | "that" | "which")) {
                let h = t.head(p)?;
                return out(t.sub(h, &[p]), "subj:relcl");
            }
            // passive in the question («who was blamed»), active in the sentence («blamed him») — the object
            if fr.passive && !t.kids(p).iter().any(|k| t.w[*k].rel == Rel::AuxPass) {
                let o = t.kids(p).iter().copied().find(|k| t.base(*k) == Rel::Obj)?;
                let ix = t.sub(o, &[]);
                return if is_pron(t, &ix) { None } else { out(ix, "obj(question passive)") };
            }
            let s = t.kids(p).iter().copied().find(|k| t.base(*k) == Rel::Nsubj);
            let ix = match s {
                Some(s) => t.sub(s, &[]),
                // tree without a subject: words from the clause start up to the predicate
                None => {
                    let st = (0..p).rev().find(|i| matches!(t.form(*i), ";" | ":" | "\"")).map(|i| i + 1).unwrap_or(0);
                    (st..p).collect()
                }
            };
            if is_pron(t, &ix) || ix.is_empty() {
                return None;
            }
            out(ix, if s.is_some() { "subj" } else { "subj(words before predicate)" })
        }
        Want::Obj => {
            let p = p?;
            let ks = verb_kids(t, p);
            if let Some(pp) = &fr.prep
                && let Some(o) = ks.iter().copied().find(|k| t.base(*k) == Rel::Obl && case_of(t, *k) == *pp)
            {
                let ix: Vec<usize> = t.sub(o, &[]).into_iter().filter(|i| !(t.base(*i) == Rel::Case && t.head(*i) == Some(o))).collect();
                return out(ix, "obl:prep");
            }
            for r in [Rel::Obj, Rel::Ccomp, Rel::Xcomp, Rel::Iobj] {
                if let Some(o) = ks.iter().copied().find(|k| t.base(*k) == r) {
                    let ix = t.sub(o, &[]);
                    if is_pron(t, &ix) {
                        continue;
                    }
                    return out(ix, r.name());
                }
            }
            // «pride herself on his good looks»: an adverbial that is not in the question
            for k in ks.iter().copied().filter(|k| t.base(*k) == Rel::Obl) {
                let ix: Vec<usize> = t.sub(k, &[]).into_iter().filter(|i| !(t.base(*i) == Rel::Case && t.head(*i) == Some(k))).collect();
                if let Some(r) = out(ix, "obl") {
                    return Some(r);
                }
            }
            None
        }
        Want::Where => {
            let cand = |i: usize| -> bool {
                (t.base(i) == Rel::Obl && LOC_CASE.contains(&case_of(t, i).as_str()) && !TIME_N.contains(&t.lemma(i))) || (t.base(i) == Rel::Advmod && LOC_ADV.contains(&t.form(i)))
            };
            if let Some(p) = p {
                let ks = verb_kids(t, p);
                if let Some(o) = ks.iter().copied().find(|k| cand(*k)) {
                    return out(t.sub(o, &[]), "obl:loc");
                }
                // «where was X»: place inside a modifier of the noun X (the woods up among the hills)
                if matches!(t.upos(p), UPos::NOUN | UPos::PROPN) {
                    let mods: Vec<usize> = t.kids(p).iter().copied().filter(|k| matches!(t.base(*k), Rel::Nmod | Rel::Acl | Rel::Advmod) && (LOC_CASE.contains(&case_of(t, *k).as_str()) || t.base(*k) != Rel::Nmod)).collect();
                    if !mods.is_empty() {
                        let mut ix = Vec::new();
                        for m in &mods {
                            ix.extend(t.sub(*m, &[]));
                        }
                        return out(ix, "nmod:loc");
                    }
                    // the parser attached the place to the verb: place adverbials right after X (set out for
                    // the woods *up among the hills*)
                    if let Some(h) = t.head(p) {
                        let sib: Vec<usize> = t.kids(h).iter().copied().filter(|k| *k > p && cand(*k)).collect();
                        if let (Some(a), Some(b)) = (sib.first(), sib.last()) {
                            let lo = t.sub(*a, &[])[0];
                            let hi = *t.sub(*b, &[]).last().unwrap();
                            if lo == p + 1 || lo <= p + 2 {
                                return out((lo..=hi).collect(), "obl:loc-after-X");
                            }
                        }
                    }
                }
            }
            let o = (0..t.len()).find(|i| t.base(*i) == Rel::Obl && LOC_CASE.contains(&case_of(t, *i).as_str()) && !TIME_N.contains(&t.lemma(*i)))?;
            out(t.sub(o, &[]), "obl:loc(any)")
        }
        Want::When => {
            let scan: Vec<usize> = match p {
                Some(p) => verb_kids(t, p),
                None => (0..t.len()).collect(),
            };
            for k in &scan {
                if t.base(*k) == Rel::Advcl && TIME_MARK.contains(&mark_of(t, *k).split(' ').next().unwrap_or("")) {
                    return out(t.sub(*k, &[]), "advcl:time");
                }
            }
            for k in &scan {
                if t.base(*k) == Rel::Obl && (t.w[*k].rel == Rel::OblTmod || TIME_N.contains(&t.lemma(*k))) {
                    return out(t.sub(*k, &[]), "obl:time");
                }
            }
            None
        }
        Want::Why => {
            // the question's predicate, then its heads (xcomp/obj/conj) — up to two levels up
            let mut chain = Vec::new();
            if let Some(p) = p {
                let mut x = p;
                chain.push(x);
                for _ in 0..2 {
                    match t.head(x) {
                        Some(h) if matches!(t.base(x), Rel::Xcomp | Rel::Ccomp | Rel::Obj | Rel::Conj | Rel::Advcl | Rel::Nsubj | Rel::Obl) => {
                            chain.push(h);
                            x = h;
                        }
                        _ => break,
                    }
                }
            }
            let strip_mark = |k: usize| -> Vec<usize> { t.sub(k, &[]).into_iter().filter(|i| !(matches!(t.base(*i), Rel::Mark | Rel::Cc) && t.head(*i) == Some(k))).collect() };
            for &x in &chain {
                let ks = verb_kids(t, x);
                for k in &ks {
                    let m = mark_of(t, *k);
                    if matches!(t.base(*k), Rel::Advcl) && WHY_MARK.iter().any(|w| m.split(' ').any(|y| y == *w)) && !chain.contains(k) {
                        return out(strip_mark(*k), "advcl:because");
                    }
                }
                // «…, for he had the heart of a hero» — a coordinate clause with «for»
                for k in &ks {
                    if t.base(*k) == Rel::Conj && *k > x && t.kids(*k).iter().any(|c| t.base(*c) == Rel::Cc && matches!(t.form(*c), "for" | "because")) {
                        return out(strip_mark(*k), "conj:for");
                    }
                }
                for k in &ks {
                    if matches!(t.base(*k), Rel::Advcl | Rel::Xcomp) && t.kids(*k).iter().any(|m| t.form(*m) == "to" && t.base(*m) == Rel::Mark) && *k > x && !chain.contains(k) {
                        return out(t.sub(*k, &[]), "advcl:to");
                    }
                }
                for k in &ks {
                    if t.base(*k) == Rel::Obl && matches!(case_of(t, *k).as_str(), "because" | "for" | "from" | "out" | "through") {
                        return out(t.sub(*k, &[]), "obl:because");
                    }
                }
                // «…, so he crept in»: the reason is the preceding clause
                let so = t.kids(x).iter().any(|k| matches!(t.base(*k), Rel::Cc | Rel::Advmod | Rel::Mark) && matches!(t.form(*k), "so" | "therefore" | "thus" | "hence") && *k < x);
                if so && matches!(t.base(x), Rel::Conj | Rel::Parataxis | Rel::Advcl | Rel::Ccomp)
                    && let Some(h) = t.head(x)
                {
                    return out(t.sub(h, &[x]), "conj:so");
                }
                // «X was so ADJ / of such beauty that they V»: the reason is the main part
                if matches!(t.base(x), Rel::Ccomp | Rel::Advcl) && t.kids(x).iter().any(|k| t.form(*k) == "that" && t.base(*k) == Rel::Mark)
                    && let Some(h) = t.head(x)
                {
                    return out(t.sub(h, &[x]), "so-that");
                }
            }
            // the tree gave nothing: words — «X, so <predicate>» (reason before «so») and «<predicate> … because Y»
            let p = p?;
            let bound = |i: usize| matches!(t.form(i), ";" | ":" | "\"" | "." | "!" | "?");
            if let Some(k) = (p.saturating_sub(5)..p).rev().find(|k| matches!(t.form(*k), "so" | "therefore" | "thus") && *k > 0 && matches!(t.form(*k - 1), "," | ";")) {
                let st = (0..k - 1).rev().find(|i| bound(*i)).map(|i| i + 1).unwrap_or(0);
                return out((st..k - 1).collect(), "words: X, so");
            }
            if let Some(k) = (p + 1..t.len()).find(|k| matches!(t.form(*k), "because" | "for" | "since") && t.upos(*k) != UPos::ADP) {
                let en = (k + 1..t.len()).find(|i| bound(*i)).unwrap_or(t.len());
                return out((k + 1..en).collect(), "words: because Y");
            }
            None
        }
        Want::How => {
            let p = p?;
            let ks = verb_kids(t, p);
            for k in &ks {
                if t.base(*k) == Rel::Obl && matches!(case_of(t, *k).as_str(), "with" | "by" | "through" | "in" | "without" | "like") && !TIME_N.contains(&t.lemma(*k)) {
                    return out(t.sub(*k, &[]), "obl:manner");
                }
            }
            for k in &ks {
                if t.base(*k) == Rel::Advcl && (mark_of(t, *k) == "by" || t.tag(*k) == PTag::VBG) {
                    return out(t.sub(*k, &[]), "advcl:by");
                }
            }
            for k in &ks {
                if t.base(*k) == Rel::Advmod && t.upos(*k) == UPos::ADV && !matches!(t.form(*k), "not" | "never" | "then" | "so" | "also" | "too" | "very" | "once" | "again" | "now" | "there" | "here" | "soon" | "just" | "still") {
                    return out(t.sub(*k, &[]), "advmod");
                }
            }
            None
        }
        Want::Feel => {
            // adjectival predicate (was troubled, felt sad, found his wife very anxious) or «with joy»
            let adj = |i: usize| -> bool { (matches!(t.upos(i), UPos::ADJ) || (t.tag(i) == PTag::VBN && t.kids(i).iter().any(|k| matches!(t.base(*k), Rel::Cop | Rel::Aux)))) && emo(t, i) };
            let mut cands = Vec::new();
            for i in 0..t.len() {
                if !adj(i) {
                    continue;
                }
                let r = t.base(i);
                let pred_like = t.kids(i).iter().any(|k| t.base(*k) == Rel::Cop) || t.w[i].rel == Rel::AuxPass;
                let comp = matches!(r, Rel::Xcomp | Rel::Ccomp) && t.head(i).is_some_and(|h| FEEL_V.contains(&t.lemma(h)));
                let root_adj = matches!(r, Rel::Root | Rel::Conj | Rel::Advcl) && (pred_like || t.kids(i).iter().any(|k| t.base(*k) == Rel::Nsubj));
                if comp || root_adj || (pred_like && t.tag(i) == PTag::VBN) {
                    cands.push(i);
                }
            }
            if let Some(i) = cands.first().copied() {
                // the adjective, its intensifier directly before it, and coordinated adjectives (the words themselves)
                let intens = |k: usize| k + 1 == i || matches!(t.form(k), "very" | "so" | "sorely" | "much" | "quite" | "greatly" | "deeply" | "too" | "most" | "rather" | "extremely" | "terribly" | "exceedingly" | "highly");
                let mut ix = vec![i];
                for k in t.kids(i) {
                    if t.base(*k) == Rel::Advmod && *k < i && intens(*k) {
                        ix.push(*k);
                    }
                    if t.base(*k) == Rel::Conj && adj(*k) && !t.kids(*k).iter().any(|c| matches!(t.base(*c), Rel::Nsubj | Rel::Cop)) {
                        ix.push(*k);
                        ix.extend(t.kids(*k).iter().copied().filter(|c| t.base(*c) == Rel::Cc));
                    }
                }
                // intensifier right before the adjective, wherever the parser attached it (sorely troubled)
                if i > 0 && matches!(t.upos(i - 1), UPos::ADV | UPos::ADJ) && intens(i - 1) && !matches!(t.form(i - 1), "not" | "never") && !emo(t, i - 1) {
                    ix.push(i - 1);
                }
                ix.sort_unstable();
                // contiguous span from the first to the last
                let (a, b) = (ix[0], *ix.last().unwrap());
                return out((a..=b).collect(), "adj");
            }
            let e = (0..t.len()).find(|i| t.upos(*i) == UPos::NOUN && EMO_N.contains(&t.lemma(*i)))?;
            out(vec![e], "emotion-noun")
        }
        Want::Do | Want::Clause => {
            // there is an anchor subordinate clause and it is in this sentence: the main part after it (without the subject for «do»)
            if !sub_preds.is_empty()
                && find_pred(t, sub_preds).is_some()
                && let Some((s, r)) = extract(t, Want::After, preds, sub_preds, fr, qlem)
            {
                if want == Want::Do {
                    let a = find_pred(t, sub_preds).unwrap();
                    let m = if t.base(a) == Rel::Advcl { t.head(a) } else { None };
                    if let Some(m) = m
                        && let Some(ns) = t.kids(m).iter().copied().find(|k| t.base(*k) == Rel::Nsubj)
                    {
                        let mut cut = vec![a, ns];
                        cut.extend(t.kids(m).iter().copied().filter(|k| t.base(*k) == Rel::Parataxis));
                        return out(t.sub(m, &cut), "main-of-advcl-subj");
                    }
                }
                return Some((s, r));
            }
            let mut p = p.or_else(|| t.root())?;
            // the predicate is coordinated (sit … and *cry*): the whole chain from the first
            if t.base(p) == Rel::Conj
                && let Some(h) = t.head(p)
                && t.upos(h) == UPos::VERB
                && h < p
            {
                p = h;
            }
            let mut cut: Vec<usize> = Vec::new();
            for k in t.kids(p) {
                let r = t.base(*k);
                if want == Want::Do && r == Rel::Nsubj {
                    cut.push(*k);
                }
                // a subordinate clause repeating the question's anchor (when he saw …) — dropped
                if r == Rel::Advcl && !sub_preds.is_empty() && t.sub(*k, &[]).iter().any(|i| sub_preds.iter().any(|s| s == t.lemma(*i))) {
                    cut.push(*k);
                }
                if matches!(r, Rel::Parataxis) || (r == Rel::Conj && matches!(t.upos(*k), UPos::VERB) && t.kids(*k).iter().any(|c| t.base(*c) == Rel::Nsubj)) {
                    cut.push(*k);
                }
                // coordinated nominal predicate (were happy and contented) — a state, not an action
                if r == Rel::Conj && !matches!(t.upos(*k), UPos::VERB) {
                    cut.push(*k);
                }
                // an adverbial repeating the question (in the summer mornings)
                if matches!(r, Rel::Obl | Rel::Advmod) {
                    let c: Vec<usize> = t.sub(*k, &[]).into_iter().filter(|i| t.content(*i)).collect();
                    if !c.is_empty() && c.iter().all(|i| qlem.contains(t.lemma(*i)) || qlem.contains(t.form(*i))) {
                        cut.push(*k);
                    }
                }
            }
            out(t.sub(p, &cut), if want == Want::Do { "clause-subj" } else { "clause" })
        }
        Want::After => {
            let a = find_pred(t, sub_preds)?;
            // the anchor is a subordinate clause: the answer is the main part without it
            if matches!(t.base(a), Rel::Advcl) {
                let m = t.head(a)?;
                let mut cut = vec![a];
                cut.extend(t.kids(m).iter().copied().filter(|k| t.base(*k) == Rel::Parataxis));
                return out(t.sub(m, &cut), "main-of-advcl");
            }
            // «so surprised that he lost …»
            if let Some(k) = t.kids(a).iter().copied().find(|k| matches!(t.base(*k), Rel::Ccomp | Rel::Advcl) && t.kids(*k).iter().any(|m| t.form(*m) == "that") && *k > a) {
                return out(t.sub(k, &[]), "that-clause");
            }
            // «… made a false stroke, and cut his own arm» — the following coordinated predicates
            let conj: Vec<usize> = t.kids(a).iter().copied().filter(|k| t.base(*k) == Rel::Conj && *k > a && matches!(t.upos(*k), UPos::VERB | UPos::ADJ)).collect();
            if !conj.is_empty() {
                let mut ix = Vec::new();
                for c in conj {
                    ix.extend(t.sub(c, &[]));
                }
                return out(ix, "conj-after");
            }
            // the anchor is a coordinated predicate: the coordinated heads after it
            if t.base(a) == Rel::Conj
                && let Some(h) = t.head(a)
            {
                let rest: Vec<usize> = t.kids(h).iter().copied().filter(|k| t.base(*k) == Rel::Conj && *k > a).collect();
                if !rest.is_empty() {
                    let mut ix = Vec::new();
                    for c in rest {
                        ix.extend(t.sub(c, &[]));
                    }
                    return out(ix, "conj-after");
                }
            }
            None
        }
        Want::Say => {
            let p = p?;
            // direct speech: between quotes
            let q: Vec<usize> = (0..t.len()).filter(|i| matches!(t.form(*i), "\"" | "``" | "''" | "'")).collect();
            if q.len() >= 2 {
                let ix: Vec<usize> = (q[0] + 1..q[1]).collect();
                if ix.len() >= 2 {
                    return out(ix, "quote");
                }
            }
            let ks = verb_kids(t, p);
            for r in [Rel::Ccomp, Rel::Xcomp, Rel::Obj] {
                if let Some(o) = ks.iter().copied().find(|k| t.base(*k) == r) {
                    let ix = t.sub(o, &[]);
                    if is_pron(t, &ix) {
                        continue;
                    }
                    return out(ix, r.name());
                }
            }
            None
        }
        Want::Num => {
            let n = (0..t.len()).find(|i| t.base(*i) == Rel::Nummod || t.upos(*i) == UPos::NUM)?;
            Some((t.form(n).to_string(), "nummod".into()))
        }
        Want::Describe => {
            // modifier of the question's subject noun: amod, acl, nmod, appos
            let p = p?;
            if matches!(t.upos(p), UPos::NOUN | UPos::PROPN) {
                let mods: Vec<usize> = t.kids(p).iter().copied().filter(|k| matches!(t.base(*k), Rel::Amod | Rel::Acl | Rel::Nmod | Rel::Appos) && !QUANT.contains(&t.form(*k)) && t.upos(*k) != UPos::NUM).collect();
                if !mods.is_empty() {
                    let mut ix = Vec::new();
                    for m in mods {
                        ix.extend(t.sub(m, &[]));
                    }
                    return out(ix, "modifiers");
                }
            }
            // «X was ADJ …» — descriptive predicate
            let h = t.head(p).filter(|h| t.kids(*h).iter().any(|k| t.base(*k) == Rel::Cop))?;
            out(t.sub(h, &[p]), "cop-pred")
        }
    }
}

/// Fallback span: the longest contiguous piece of the sentence without question words (at least 2 content words).
pub fn minus_question(t: &Tree, qlem: &HashSet<String>) -> Option<String> {
    let mut best: (usize, usize, usize) = (0, 0, 0);
    let mut i = 0;
    while i < t.len() {
        let inq = |k: usize| t.content(k) && (qlem.contains(t.lemma(k)) || qlem.contains(t.form(k)));
        if inq(i) {
            i += 1;
            continue;
        }
        let s = i;
        let mut c = 0;
        while i < t.len() && !inq(i) {
            c += t.content(i) as usize;
            i += 1;
        }
        if c > best.2 {
            best = (s, i, c);
        }
    }
    if best.2 < 2 {
        return None;
    }
    clean(t, (best.0..best.1).collect(), qlem).map(|ix| t.text(&ix))
}
