//! The logical facts of a verse's world, and how far two readings of one verse are apart logically.
//!
//! Two translations of a verse may word it as they like, but their worlds should hold the same logic: the same
//! events, in the same order, with the same participants in the same roles (who did what to whom), the same negation,
//! modality and quantities. A verse's facts are read from its frames (`prag::frames`, through roundtrip's `F`): per
//! event its verb, negation, modal and roles (agent, patient, recipient: the entity's head lemma and whether it is a
//! name, a common noun or a pronoun), and per sentence the numbers it says. Two readings are compared through a
//! lexicon (the same concept is the same fact):
//! - events matched by verb concept along the longest common subsequence (the shared order);
//! - a **contradiction** is a matched event whose negation differs, whose modals both stand and differ, or whose role
//!   is filled in both by two different names («Moses» against «Joshua»; a pronoun against a name is a matter of
//!   reference, and two common nouns are as often two words for one thing or the parser's choice of head: they count
//!   in the role agreement, not as contradictions); in which one entity, a name or a noun, is the agent on one side and the patient on
//!   the other («the dog bit the man» against «the man bit the dog», or against «the boy bit the dog»); and a sentence's numbers that differ where both say numbers;
//! - an event one side has and the other not is a **gap**, not a contradiction;
//! - the **logical distance** is the contradictions plus half the gaps: two readings of one verse are near, two
//!   different verses far.
//!
//! The contradictions are what a training step must not raise (the lexicon's logic gate, `meta::learn_gated`; an
//! author's delta, `profile::Delta::logic_gate`). The idea: two texts that hold the same facts under different
//! wording, as in recognising textual entailment (dagan-2006); an event a translation leaves out as an omission, not an
//! error of meaning (melamed-1996, the detection of omissions in translations).

use std::collections::BTreeMap;

use en::annotate::Word;
use en::gram::Tag;
use roundtrip::frames::F;

use crate::meta::Lexicon;

/// What an entity is: a name, a common noun, a pronoun.
pub const NAME: u8 = 1;
pub const NOUN: u8 = 0;
pub const PRONOUN: u8 = 2;

/// One event of a verse's world: its verb, negation, modal and roles (slot 1 agent, 2 patient, 3 recipient; the
/// entity; its kind), or (verb `#n`) the numbers its sentence says, as slot 0.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Event {
    pub verb: String,
    pub neg: bool,
    pub modal: Option<String>,
    pub roles: Vec<(u8, String, u8)>,
}

pub const NUMBERS: &str = "#n";

/// Entity ids that are pronouns even where the frames did not resolve them.
pub const PRONOUNS: [&str; 32] = ["i", "me", "we", "us", "thou", "thee", "ye", "you", "he", "him", "she", "her", "it", "they", "them", "who", "whom", "which", "that", "this", "these", "those", "one", "himself", "herself", "itself", "themselves", "myself", "ourselves", "yourself", "yourselves", "thyself"];

/// A sentence's events from its frames, and its numbers.
pub fn events(fs: &[F], w: &[Word]) -> Vec<Event> {
    let mut out: Vec<Event> = Vec::new();
    for f in fs {
        if f.kind == prag::frames::Kind::Bare || f.verb.is_empty() {
            continue;
        }
        let mut roles = Vec::new();
        for (s, e) in [(1u8, &f.agent), (2, &f.patient), (3, &f.recipient)] {
            if let Some(e) = e {
                let kind = if e.pron.is_some() || PRONOUNS.contains(&e.id.to_lowercase().as_str()) { PRONOUN } else if e.named { NAME } else { NOUN };
                roles.push((s, e.id.to_lowercase(), kind));
            }
        }
        out.push(Event { verb: f.verb.to_lowercase(), neg: f.neg, modal: f.tam.modal.map(|m| format!("{m:?}").to_lowercase()), roles });
    }
    let nums: Vec<(u8, String, u8)> = w.iter().filter(|x| x.tag == Tag::CD).map(|x| (0u8, number(&x.lemma), NOUN)).collect();
    if !nums.is_empty() {
        out.push(Event { verb: NUMBERS.into(), neg: false, modal: None, roles: nums });
    }
    out
}

/// A number word's value, so that «threescore» and «sixty», «hundreds» and «hundred» are one number.
pub fn number(w: &str) -> String {
    let w = w.to_lowercase().replace(',', "");
    let v = match w.trim_end_matches('s') {
        "one" => "1",
        "two" | "twain" | "both" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "ten" => "10",
        "eleven" => "11",
        "twelve" => "12",
        "score" => "20",
        "twenty" => "20",
        "thirty" => "30",
        "forty" => "40",
        "fifty" => "50",
        "threescore" | "sixty" => "60",
        "seventy" | "threescore and ten" => "70",
        "fourscore" | "eighty" => "80",
        "ninety" => "90",
        "hundred" => "100",
        "thousand" => "1000",
        _ => return w,
    };
    v.to_string()
}

/// How far two readings of a verse are apart logically.
#[derive(Clone, Copy, Debug, Default)]
pub struct Logic {
    /// events: matched (as a multiset), on each side
    pub ev: (u64, u64, u64),
    /// (verb, role, entity) facts: shared, on each side
    pub roles: (u64, u64, u64),
    /// events matched in the same order (the longest common subsequence)
    pub ordered: u64,
    pub contra: u64,
}

impl Logic {
    pub fn add(&mut self, o: &Logic) {
        self.ev = (self.ev.0 + o.ev.0, self.ev.1 + o.ev.1, self.ev.2 + o.ev.2);
        self.roles = (self.roles.0 + o.roles.0, self.roles.1 + o.roles.1, self.roles.2 + o.roles.2);
        self.ordered += o.ordered;
        self.contra += o.contra;
    }
    pub fn events(&self) -> f64 {
        2.0 * self.ev.0 as f64 / (self.ev.1 + self.ev.2).max(1) as f64
    }
    pub fn role_share(&self) -> f64 {
        2.0 * self.roles.0 as f64 / (self.roles.1 + self.roles.2).max(1) as f64
    }
    /// the share of matched events that stand in the same order
    pub fn order(&self) -> f64 {
        self.ordered as f64 / self.ev.0.max(1) as f64
    }
    /// the events one side has and the other not
    pub fn gaps(&self) -> u64 {
        (self.ev.1 + self.ev.2).saturating_sub(2 * self.ev.0)
    }
    /// contradictions plus half the gaps
    pub fn distance(&self) -> f64 {
        self.contra as f64 + self.gaps() as f64 / 2.0
    }
}

fn mapped(lex: &Lexicon, v: &[Event]) -> Vec<Event> {
    v.iter()
        .map(|e| Event {
            verb: if e.verb == NUMBERS { e.verb.clone() } else { lex.concept(&e.verb).to_string() },
            neg: e.neg,
            modal: e.modal.clone(),
            roles: e.roles.iter().map(|(s, x, k)| (*s, if *s == 0 { x.clone() } else { lex.concept(x).to_string() }, *k)).collect(),
        })
        .collect()
}

fn multiset<K: Ord + Clone>(a: &[K], b: &[K]) -> u64 {
    let mut cb: BTreeMap<&K, i64> = BTreeMap::new();
    for x in b {
        *cb.entry(x).or_default() += 1;
    }
    let mut m = 0;
    for x in a {
        if let Some(c) = cb.get_mut(x).filter(|c| **c > 0) {
            *c -= 1;
            m += 1;
        }
    }
    m
}

/// Compare two readings' facts through a lexicon.
pub fn compare(lex: &Lexicon, a: &[Event], b: &[Event]) -> Logic {
    compare_why(lex, a, b, None)
}

/// The same, with the reason of every contradiction.
pub fn compare_why(lex: &Lexicon, a: &[Event], b: &[Event], mut why: Option<&mut Vec<String>>) -> Logic {
    let (a, b) = (mapped(lex, a), mapped(lex, b));
    let va: Vec<String> = a.iter().map(|e| e.verb.clone()).collect();
    let vb: Vec<String> = b.iter().map(|e| e.verb.clone()).collect();
    let an = crate::meta::lcs(&va, &vb);
    let mut l = Logic { ev: (multiset(&va, &vb), va.len() as u64, vb.len() as u64), ordered: an.len() as u64, ..Default::default() };
    let facts = |v: &[Event]| -> Vec<(String, u8, String)> { v.iter().filter(|e| e.verb != NUMBERS).flat_map(|e| e.roles.iter().map(move |(s, x, _)| (e.verb.clone(), *s, x.clone()))).collect() };
    let (fa, fb) = (facts(&a), facts(&b));
    l.roles = (multiset(&fa, &fb), fa.len() as u64, fb.len() as u64);
    let mut say = |s: String| {
        if let Some(w) = why.as_deref_mut() {
            w.push(s);
        }
    };
    for (i, j) in an {
        let (x, y) = (&a[i], &b[j]);
        if x.verb == NUMBERS {
            let (nx, ny): (Vec<&String>, Vec<&String>) = (x.roles.iter().map(|r| &r.1).collect(), y.roles.iter().map(|r| &r.1).collect());
            let d = (nx.len().max(ny.len()) as u64).saturating_sub(multiset(&nx, &ny));
            if d > 0 {
                l.contra += d;
                say(format!("numbers {nx:?} against {ny:?}"));
            }
            continue;
        }
        if x.neg != y.neg {
            l.contra += 1;
            say(format!("«{}» negated on one side only", x.verb));
        }
        if let (Some(p), Some(q)) = (&x.modal, &y.modal) {
            if p != q {
                l.contra += 1;
                say(format!("«{}»: modal {p} against {q}", x.verb));
            }
        }
        // one entity is the agent on one side and the patient on the other: who did what to whom is reversed, even
        // where the other role is worded differently (a name or a noun; a pronoun moving between roles is reference)
        let role = |v: &Event, s: u8| v.roles.iter().find(|r| r.0 == s && r.2 != PRONOUN).map(|r| r.1.clone());
        let (xa, xp, ya, yp) = (role(x, 1), role(x, 2), role(y, 1), role(y, 2));
        let crossed = |from: &Option<String>, to: &Option<String>, stays: &Option<String>| from.is_some() && from == to && from != stays;
        // only where both sides are transitive (an agent and a patient each): the one argument of an intransitive verb
        // is the agent or the patient as the reader takes the verb, which is not a difference of the texts
        let both = |v: &Event| v.roles.iter().any(|r| r.0 == 1) && v.roles.iter().any(|r| r.0 == 2);
        if both(x) && both(y) && (crossed(&xa, &yp, &ya) || crossed(&xp, &ya, &yp)) {
            l.contra += 1;
            say(format!("«{}»: «{}» changes between agent and patient", x.verb, xa.as_ref().filter(|a| Some(*a) == yp.as_ref()).or(xp.as_ref()).cloned().unwrap_or_default()));
            continue;
        }
        for (s, e, k) in &x.roles {
            if let Some((_, f, kf)) = y.roles.iter().find(|(t, _, _)| t == s) {
                if e != f && *k == NAME && *kf == NAME {
                    l.contra += 1;
                    say(format!("«{}»: {} «{e}» against «{f}»", x.verb, ["", "agent", "patient", "recipient"][*s as usize]));
                }
            }
        }
    }
    l
}

/// The sensitivity control: one change of meaning put into a reading, by kind. A logical distance that does not move
/// when the meaning changes is no gate; this control says, per kind, how often it moves.
pub const PERTURB: [&str; 4] = ["negation flipped", "agent and patient swapped", "a number changed", "two events reordered"];

/// The reading `v` with one change of meaning of the given kind, made on the events `only` (those the other reading
/// matches: a change on an event the other side lacks cannot be seen), or `None` where there is nothing to change.
pub fn perturb(v: &[Event], kind: usize, only: &[usize]) -> Option<Vec<Event>> {
    let mut w = v.to_vec();
    match kind {
        0 => w.iter_mut().enumerate().filter(|(i, _)| only.contains(i)).map(|(_, e)| e).find(|e| e.verb != NUMBERS).map(|e| e.neg = !e.neg)?,
        1 => {
            let e = w.iter_mut().enumerate().filter(|(i, _)| only.contains(i)).map(|(_, e)| e).find(|e| {
                let (a, p) = (e.roles.iter().find(|r| r.0 == 1), e.roles.iter().find(|r| r.0 == 2));
                e.verb != NUMBERS && matches!((a, p), (Some(a), Some(p)) if a.1 != p.1)
            })?;
            for r in e.roles.iter_mut() {
                r.0 = match r.0 {
                    1 => 2,
                    2 => 1,
                    s => s,
                };
            }
            e.roles.sort();
        }
        2 => {
            let r = w.iter_mut().enumerate().filter(|(i, _)| only.contains(i)).map(|(_, e)| e).find(|e| e.verb == NUMBERS)?.roles.first_mut()?;
            r.1 = r.1.parse::<u64>().map(|n| (n + 1).to_string()).unwrap_or_else(|_| format!("{}1", r.1));
        }
        _ => {
            let i = (1..w.len()).find(|&i| only.contains(&(i - 1)) && only.contains(&i) && w[i - 1].verb != NUMBERS && w[i].verb != NUMBERS && w[i - 1].verb != w[i].verb)?;
            w.swap(i - 1, i);
        }
    }
    Some(w)
}

/// The sensitivity control's counts by kind: changes made; changes the contradictions catch; changes any part of
/// the comparison catches (contradictions, gaps, shared roles, order).
#[derive(Clone, Copy, Debug, Default)]
pub struct Sens(pub [(u64, u64, u64); 4]);

impl Sens {
    pub fn add(&mut self, o: &Sens) {
        for k in 0..4 {
            self.0[k] = (self.0[k].0 + o.0[k].0, self.0[k].1 + o.0[k].1, self.0[k].2 + o.0[k].2);
        }
    }
    /// Put each kind of change into `b`, on an event `a` matches, and see whether the comparison with `a` moves.
    pub fn probe(&mut self, lex: &Lexicon, a: &[Event], b: &[Event], base: &Logic) {
        let (ma, mb) = (mapped(lex, a), mapped(lex, b));
        let va: Vec<String> = ma.iter().map(|e| e.verb.clone()).collect();
        let vb: Vec<String> = mb.iter().map(|e| e.verb.clone()).collect();
        let only: Vec<usize> = crate::meta::lcs(&va, &vb).into_iter().map(|(_, j)| j).collect();
        for k in 0..4 {
            if let Some(c) = perturb(b, k, &only) {
                let l = compare(lex, a, &c);
                self.0[k].0 += 1;
                let contra = l.contra > base.contra;
                self.0[k].1 += contra as u64;
                self.0[k].2 += (contra || l.gaps() > base.gaps() || l.roles.0 < base.roles.0 || l.ordered < base.ordered) as u64;
            }
        }
    }
}

/// The logical words of a chunk or a concept: negations, modals, numbers (by value). A merge of the lexicon and an
/// author's override may change a word's spelling, never these (`meta::learn_gated`, `profile::Delta::logic_gate`).
pub fn logical_words(words: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = words
        .iter()
        .map(|w| w.to_lowercase())
        .map(|w| if w == "n't" { "not".to_string() } else { w })
        .filter_map(|w| {
            if matches!(w.as_str(), "not" | "no" | "never" | "nor" | "neither" | "none" | "nothing") || crate::meta::MODALS.contains(&w.as_str()) {
                return Some(w);
            }
            let n = number(&w);
            (n.chars().all(|c| c.is_ascii_digit()) && !n.is_empty()).then_some(n)
        })
        .collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(v: &str, neg: bool, roles: &[(u8, &str, u8)]) -> Event {
        Event { verb: v.into(), neg, modal: None, roles: roles.iter().map(|(s, x, k)| (*s, x.to_string(), *k)).collect() }
    }

    #[test]
    fn every_kind_of_change_is_seen() {
        let lex = Lexicon::default();
        let a = vec![
            ev("smite", false, &[(1, "moses", NAME), (2, "egyptian", NAME)]),
            ev("hide", false, &[(1, "moses", NAME)]),
            Event { verb: NUMBERS.into(), neg: false, modal: None, roles: vec![(0, "40".into(), NOUN)] },
        ];
        let base = compare(&lex, &a, &a);
        assert_eq!(base.contra, 0);
        let mut s = Sens::default();
        s.probe(&lex, &a, &a, &base);
        for k in 0..4 {
            assert_eq!(s.0[k].0, 1, "{}", PERTURB[k]);
            assert_eq!(s.0[k].2, 1, "{} is not seen", PERTURB[k]);
        }
        // negation, named roles and numbers are contradictions; the order is not
        assert_eq!([s.0[0].1, s.0[1].1, s.0[2].1, s.0[3].1], [1, 1, 1, 0]);
        // an event the other side lacks is not changed: nothing of «hide» in the source, no negation to flip there
        let mut g = Sens::default();
        g.probe(&lex, &a[2..], &a[1..2], &compare(&lex, &a[2..], &a[1..2]));
        assert_eq!(g.0[0].0, 0);
        // a reversal of common nouns is a contradiction too; the same roles are none
        let dog = vec![ev("bite", false, &[(1, "dog", NOUN), (2, "man", NOUN)])];
        let man = vec![ev("bite", false, &[(1, "man", NOUN), (2, "dog", NOUN)])];
        assert_eq!(compare(&lex, &dog, &man).contra, 1);
        assert_eq!(compare(&lex, &dog, &dog).contra, 0);
        // one entity crossing is enough, the other role worded differently
        let boy = vec![ev("bite", false, &[(1, "boy", NOUN), (2, "dog", NOUN)])];
        assert_eq!(compare(&lex, &dog, &boy).contra, 1);
        // a pronoun moving between roles is reference, not a reversal
        let he = vec![ev("bite", false, &[(1, "him", PRONOUN), (2, "man", NOUN)])];
        let him = vec![ev("bite", false, &[(1, "dog", NOUN), (2, "him", PRONOUN)])];
        assert_eq!(compare(&lex, &he, &him).contra, 0);
        // an intransitive reading on one side is no reversal
        let came = vec![ev("come", false, &[(2, "man", NOUN)])];
        let went = vec![ev("come", false, &[(1, "man", NOUN), (2, "city", NOUN)])];
        assert_eq!(compare(&lex, &came, &went).contra, 0);
        // negative control: nothing to change, nothing counted
        let mut z = Sens::default();
        z.probe(&lex, &[], &[], &Logic::default());
        assert_eq!(z.0.iter().map(|x| x.0).sum::<u64>(), 0);
    }

    #[test]
    fn contradictions_and_their_control() {
        let mut lex = Lexicon::default();
        lex.single.insert("spake".into(), "speak".into());
        let a = vec![ev("speak", false, &[(1, "lord", NAME), (3, "joshua", NAME)]), ev("go", false, &[(1, "joshua", NAME)])];
        let b = vec![ev("spake", false, &[(1, "lord", NAME), (3, "joshua", NAME)]), ev("go", false, &[(1, "joshua", NAME)])];
        let l = compare(&lex, &a, &b);
        assert_eq!((l.contra, l.ev.0, l.ordered, l.gaps()), (0, 2, 2, 0));
        assert_eq!(l.role_share(), 1.0);
        // negative control: the other one goes, and he does not speak — two contradictions
        let c = vec![ev("speak", true, &[(1, "lord", NAME), (3, "joshua", NAME)]), ev("go", false, &[(1, "moses", NAME)])];
        let mut why = Vec::new();
        assert_eq!(compare_why(&lex, &a, &c, Some(&mut why)).contra, 2);
        assert_eq!(why.len(), 2);
        // a pronoun against a name is reference, not logic
        let p = vec![ev("speak", false, &[(1, "lord", NAME), (3, "joshua", NAME)]), ev("go", false, &[(1, "he", PRONOUN)])];
        assert_eq!(compare(&lex, &a, &p).contra, 0);
        // an event one side lacks is a gap, not a contradiction
        let g = compare(&lex, &a, &a[..1]);
        assert_eq!((g.contra, g.gaps(), g.distance()), (0, 1, 0.5));
        // numbers by value
        let n = |x: &str| vec![Event { verb: NUMBERS.into(), neg: false, modal: None, roles: vec![(0, number(x), NOUN)] }];
        assert_eq!(compare(&lex, &n("threescore"), &n("sixty")).contra, 0);
        assert_eq!(compare(&lex, &n("seven"), &n("three")).contra, 1);
        assert_eq!(logical_words(&["do", "n't", "go"]), vec!["not"]);
        assert_eq!(logical_words(&["hundreds"]), logical_words(&["hundred"]));
        assert!(logical_words(&["went"]).is_empty());
        assert_ne!(logical_words(&["may"]), logical_words(&["and"]));
    }
}
