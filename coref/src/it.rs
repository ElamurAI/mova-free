//! What a bare «it», «this» or «that» refers to, decided by tests on its predicate before the pronoun sieve runs.
//!
//! Three readings (Loáiciga et al. 2017, `papers/en/…loaiciga-2017-what-is-it…` I1; Eckert and Strube's context test
//! as surveyed in Kolhatkar et al. 2018, I1):
//! - **pleonastic** — refers to nothing: weather and time («it rained», «it was late»), extraposition («it is clear
//!   that…», «it seems that…», «it takes years to…»), an object before a small clause («made it clear that…»);
//! - **event** — refers to a clause, not to a noun: the subject of a predicate that only an event or a statement can
//!   have («it happened», «it means», «that is true», «it was a mistake», «it is because…»);
//! - **anaphoric** — the rest, left to the `pronoun` sieve.
//!
//! An event pronoun is kept from nominal antecedents. With the `deixis` sieve (not in the default sieve) it gets the
//! clause before it as its antecedent (the discourse-deixis baseline of the CODI-CRAC 2022
//! task, Yu et al. 2022, I1; the search order of Kolhatkar et al. 2018, I2): the clause to the left in the same
//! sentence, else the main clause of the previous sentence. The lists are hand-made closed classes (row
//! `coref-hand-lists` in en/data/train-licenses.tsv).

use en::gram::{Rel, UPos};

use crate::doc::Sent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reading {
    Pleonastic,
    Event,
    Anaphoric,
}

/// A step switched off (`PRAG_OFF=name,name`, the same switch as the frame reader's).
pub fn off(name: &str) -> bool {
    std::env::var("PRAG_OFF").is_ok_and(|v| v.split(',').any(|x| x == name))
}

/// Weather verbs: their subject is always pleonastic.
pub const WEATHER: [&str; 9] = ["rain", "snow", "drizzle", "pour", "hail", "thunder", "sleet", "freeze", "storm"];

/// Predicates of a copula whose «it» is the time or the weather («it was late», «it is cold»).
pub const TIME_PRED: [&str; 20] = ["late", "early", "dark", "light", "cold", "hot", "warm", "cool", "sunny", "cloudy", "windy", "rainy", "foggy", "noon", "midnight", "night", "morning", "evening", "time", "o'clock"];

/// Verbs that take a clause and leave their subject empty («it seems that», «it turned out that», «it takes a year to»).
pub const RAISING: [&str; 8] = ["seem", "appear", "happen", "turn", "take", "occur", "cost", "remain"];

/// Verbs whose «it» before a small clause is empty («made it clear that», «found it hard to»).
pub const SMALL_CLAUSE: [&str; 5] = ["make", "find", "consider", "think", "feel"];

/// Predicates only an event or a statement can be the subject of (Eckert and Strube's I-incompatible contexts).
pub const EVENT_VERB: [&str; 4] = ["happen", "occur", "mean", "matter"];

/// Copular predicates of an event or a statement («that is true», «it was a mistake»).
pub const EVENT_PRED: [&str; 22] = ["true", "false", "right", "wrong", "correct", "incorrect", "possible", "impossible", "pity", "shame", "problem", "reason", "mistake", "idea", "point", "fact", "case", "truth", "lie", "surprise", "joke", "why"];

fn is_pred(s: &Sent, i: usize) -> bool {
    matches!(s.upos(i), Some(UPos::VERB)) || s.kids_rel(i, Rel::Cop).next().is_some()
}

/// A clause child that is the real subject or content: csubj, ccomp, or an xcomp/advcl with «to», «that», «if» or
/// «whether».
fn clause_kid(s: &Sent, h: usize) -> bool {
    s.kids[h].iter().any(|&k| match s.rel(k).base() {
        Rel::Csubj | Rel::Ccomp => k > h || s.rel(k).base() == Rel::Csubj,
        Rel::Xcomp | Rel::Advcl => k > h && s.kids_rel(k, Rel::Mark).any(|m| matches!(s.low(m).as_str(), "to" | "that" | "if" | "whether")),
        _ => false,
    })
}

/// The reading of the pronoun at word `i` (only «it», «this», «that» are asked; anything else is anaphoric).
pub fn reading(s: &Sent, i: usize) -> Reading {
    let w = s.low(i);
    let dem = matches!(w.as_str(), "this" | "that") && s.upos(i) == Some(UPos::PRON);
    if w != "it" && !dem {
        return Reading::Anaphoric;
    }
    let Some(h) = s.parent(i) else { return Reading::Anaphoric };
    let hl = s.lem(h);
    let role = s.rel(i).base();
    let subj = matches!(role, Rel::Nsubj | Rel::Expl) || s.rel(i) == Rel::NsubjPass;
    let cop = s.kids_rel(h, Rel::Cop).next().is_some();
    if w == "it" && !off("it-pleonastic") {
        if subj && WEATHER.contains(&hl.as_str()) {
            return Reading::Pleonastic;
        }
        if subj && cop && TIME_PRED.contains(&hl.as_str()) && !clause_kid(s, h) {
            return Reading::Pleonastic;
        }
        if subj && (cop || RAISING.contains(&hl.as_str()) || s.rel(i) == Rel::NsubjPass) && clause_kid(s, h) {
            return Reading::Pleonastic;
        }
        if role == Rel::Obj && SMALL_CLAUSE.contains(&hl.as_str()) && s.kids_rel(h, Rel::Xcomp).any(|x| x > i) {
            return Reading::Pleonastic;
        }
    }
    if off("it-event") {
        return Reading::Anaphoric;
    }
    if subj && !cop && EVENT_VERB.contains(&hl.as_str()) && !off("it-ev-verb") {
        return Reading::Event;
    }
    if subj && cop && EVENT_PRED.contains(&hl.as_str()) && !off("it-ev-pred") {
        return Reading::Event;
    }
    // «it is because …»
    if subj && cop && s.kids_rel(h, Rel::Mark).any(|m| s.low(m) == "because") && !off("it-ev-because") {
        return Reading::Event;
    }
    // a bare demonstrative not before a noun points to the discourse (Yu et al. 2022's baseline)
    if dem && !off("dem-deixis") && !(i + 1 < s.len() && matches!(s.upos(i + 1), Some(UPos::NOUN | UPos::PROPN | UPos::ADJ))) {
        return Reading::Event;
    }
    Reading::Anaphoric
}

/// The clause an event pronoun at (`sents[si]`, `i`) refers to: a predicate before it in the same sentence that does not
/// contain it (the highest, then the nearest), else the root of the previous sentence when it is a predicate.
/// Returns (sentence, predicate, span start, span end).
pub fn antecedent(sents: &[Sent], si: usize, i: usize) -> Option<(usize, usize, usize, usize)> {
    let s = &sents[si];
    let left = (0..i).filter(|&p| is_pred(s, p) && !s.dominates(p, i) && s.sub[p].1 < i && matches!(s.rel(p).base(), Rel::Root | Rel::Conj | Rel::Parataxis | Rel::Advcl | Rel::Ccomp | Rel::Csubj)).min_by_key(|&p| (s.depth[p], std::cmp::Reverse(p)));
    // the root above the pronoun's clause, when the pronoun is in a later conjunct or parataxis of it
    let root_left = s.toks.iter().position(|t| t.head == 0).filter(|&r| r < i && is_pred(s, r)).and_then(|r| {
        let branch = s.kids[r].iter().copied().find(|&k| k > r && s.dominates(k, i) && matches!(s.rel(k).base(), Rel::Conj | Rel::Parataxis))?;
        Some((r, s.sub[branch].0))
    });
    let trim = |s: &Sent, a: usize, mut b: usize| {
        while b > a && (s.upos(b) == Some(UPos::PUNCT) || matches!(s.rel(b).base(), Rel::Cc)) {
            b -= 1;
        }
        let mut a = a;
        while a < b && s.upos(a) == Some(UPos::PUNCT) {
            a += 1;
        }
        (a, b)
    };
    if let Some((r, cut)) = root_left {
        let (a, b) = trim(s, s.sub[r].0, cut.saturating_sub(1));
        return Some((si, r, a, b));
    }
    if let Some(p) = left {
        let (a, b) = trim(s, s.sub[p].0, s.sub[p].1);
        return Some((si, p, a, b));
    }
    let pi = si.checked_sub(1)?;
    let ps = &sents[pi];
    let r = ps.toks.iter().position(|t| t.head == 0)?;
    if !is_pred(ps, r) {
        return None;
    }
    let (a, b) = trim(ps, ps.sub[r].0, ps.sub[r].1);
    Some((pi, r, a, b))
}
