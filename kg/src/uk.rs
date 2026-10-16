//! Simple Ukrainian sentence → fact: predicate, subject, objects, attributes, adverbials.
//! v2 is conservative: declarative or exclamatory sentences of up to 8 words without commas, conjunctions or digits.
//! Ambiguity the rules cannot resolve is a rejection with a reason. We do not guess.
//! Quality is measured against a hand-made gold standard (`examples/gold-*.tsv`), not by synthesis: synthesis checks only
//! completeness (graph + grammar reproduce all words), not the correctness of roles.

use std::collections::HashSet;

use crate::gram::*;
use crate::morph::{Analysis, Morph, invisible, norm};

pub struct Token {
    pub text: String,
    pub norm: String,
    pub word: bool,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    for (i, &c) in chars.iter().enumerate() {
        let joiner = matches!(c, '-' | '\'' | '’' | 'ʼ')
            && !cur.is_empty()
            && chars.get(i + 1).is_some_and(|n| n.is_alphabetic());
        if c.is_alphanumeric() || joiner || (invisible(c) && !cur.is_empty()) {
            cur.push(c);
            continue;
        }
        if invisible(c) {
            continue;
        }
        if !cur.is_empty() {
            let t = std::mem::take(&mut cur);
            out.push(Token { norm: norm(&t), text: t, word: true });
        }
        if !c.is_whitespace() {
            out.push(Token { text: c.to_string(), norm: c.to_string(), word: false });
        }
    }
    if !cur.is_empty() {
        out.push(Token { norm: norm(&cur), text: cur, word: true });
    }
    out
}

/// Conjunctions and connective words: v2 skips sentences containing them (complex or with coordinated members).
const CONJ: &[&str] = &[
    "і", "й", "та", "а", "але", "або", "чи", "що", "щоб", "коли", "якщо", "бо", "ніж", "ані", "зате", "проте",
    "адже", "хоча", "хоч", "поки", "доки", "як", "наче", "ніби", "мов", "неначе", "тому", "аніж", "де", "куди",
    // connective and interrogative words: in a declarative sentence they introduce a subordinate clause ("Ось чому…")
    "чому", "навіщо", "звідки", "хто", "який", "яка", "яке", "які", "чий", "чия", "чиє", "чиї", "котрий",
    // impersonal predicative words with an infinitive ("Треба було піти")
    "треба", "потрібно", "можна", "слід", "варто", "необхідно", "неможливо",
];
/// Copular verbs: with them an adjective or noun in the nominative/instrumental is the nominal part of the predicate.
const COPULA: &[&str] =
    &["бути", "стати", "ставати", "здаватися", "залишатися", "лишатися", "виявитися", "виявлятися", "вважатися", "бувати"];
/// Modal and phasal verbs after which an infinitive is part of the predicate.
const MODAL: &[&str] = &[
    "хотіти", "могти", "мусити", "вміти", "уміти", "мати", "бажати", "любити", "почати", "починати",
    "перестати", "переставати", "припинити", "припиняти", "збиратися", "намагатися", "старатися", "стати",
    "продовжувати", "кинути", "вирішити", "вирішувати", "планувати", "забути", "встигнути", "наважитися",
];
/// Quantity words taking the genitive: "багато років".
const QUANT: &[&str] = &[
    "багато", "мало", "кілька", "декілька", "чимало", "небагато", "скільки", "стільки", "трохи", "достатньо",
    "недостатньо", "більше", "менше", "повно", "обмаль", "замало", "забагато",
];

#[derive(Debug)]
pub enum Reject {
    Question,
    Punct(String),
    Digit,
    Conj(String),
    TooLong,
    Unknown(String),
    NoParse,
    Ambiguous,
}

impl Reject {
    /// Category for statistics.
    pub fn code(&self) -> String {
        match self {
            Reject::Question => "question".into(),
            Reject::Punct(p) => format!("punctuation {p}"),
            Reject::Digit => "digits".into(),
            Reject::Conj(_) => "conjunction".into(),
            Reject::TooLong => "over 8 words".into(),
            Reject::Unknown(_) => "word not in VESUM".into(),
            Reject::NoParse => "not covered by rules".into(),
            Reject::Ambiguous => "ambiguous".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    /// Relation from the fact: `predicate`, `subject`, `object:v_zna`, `adverbial:в+v_mis`,
    /// `quantity:багато`, `predicate:nominal`, `attribute:<relation of the modified word>`…
    pub rel: String,
    /// Index of the word in the sentence (none for an implicit subject).
    pub word: Option<usize>,
    /// Lemma key `lemma|part_of_speech`.
    pub lemma: String,
    pub gram: Gram,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fact {
    /// Grammar of the fact = grammar of the predicate (with NEG if "не" is present).
    pub gram: Gram,
    pub members: Vec<Member>,
}

fn finite(a: &Analysis) -> bool {
    a.gram.pos() == VERB && matches!(a.gram.tense(), PRES | FUTR | PAST | IMPR)
}

fn nominal(a: &Analysis) -> bool {
    a.gram.pos() == NOUN
}

fn adjectival(a: &Analysis) -> bool {
    a.gram.pos() == ADJ
}

/// Subject–predicate agreement: present/future — person and number; past — gender and number.
fn agrees(pred: Gram, subj: Gram) -> bool {
    if subj.case() != NAZ {
        return false;
    }
    match pred.tense() {
        PRES | FUTR => {
            let person = if subj.person() > 0 { subj.person() } else { 3 };
            person == pred.person() && (subj.num() == 0 || subj.num() == pred.num())
        }
        PAST if pred.num() == PL => subj.num() == PL,
        PAST => subj.num() != PL && (subj.gender() == 0 || pred.gender() == 0 || subj.gender() == pred.gender()),
        _ => false,
    }
}

/// Attribute–head agreement: case, number, gender in the singular, animacy in the accusative.
fn adj_agrees(adj: Gram, noun: Gram) -> bool {
    if adj.case() != noun.case() {
        return false;
    }
    let number = if noun.num() == PL {
        adj.num() == PL
    } else {
        adj.num() != PL && (adj.gender() == 0 || noun.gender() == 0 || adj.gender() == noun.gender())
    };
    let animacy =
        adj.case() != ZNA || adj.anim() == 0 || noun.anim() == 0 || noun.anim() == 3 || adj.anim() == noun.anim();
    number && animacy
}

/// Whether the verb's government allows an object in case `case`. Unknown government (0) allows everything;
/// under negation a transitive verb also takes the genitive ("не бачу книжки").
fn governs(gov: u8, case: u32, neg: bool) -> bool {
    let known = gov & !1;
    known == 0 || known & (1 << case) != 0 || neg && case == ROD && known & (1 << ZNA) != 0
}

/// Case preference when a preposition allows several ("у парку" — locative, not genitive).
const PP_CASE_ORDER: [u32; 5] = [MIS, ZNA, ORU, ROD, DAV];
/// "у мене", "в нього" — genitive.
const PP_PRON_CASE_ORDER: [u32; 5] = [ROD, MIS, ZNA, ORU, DAV];
/// Preference for objects without a preposition.
const OBJ_CASE_ORDER: [u32; 4] = [ZNA, DAV, ORU, ROD];

struct Ctx {
    an: Vec<Vec<Analysis>>,
    norms: Vec<String>,
}

impl Ctx {
    fn has(&self, w: usize, f: impl Fn(&Analysis) -> bool) -> bool {
        self.an[w].iter().any(f)
    }
    fn pick(&self, w: usize, f: impl Fn(&Analysis) -> bool) -> Option<&Analysis> {
        self.an[w].iter().find(|a| f(a))
    }
}

/// `names` — forms that occur capitalized mid-sentence in the corpus (proper names).
pub fn parse(text: &str, m: &Morph, names: &HashSet<String>) -> Result<Fact, Reject> {
    let toks = tokenize(text);
    for t in toks.iter().filter(|t| !t.word) {
        match t.text.as_str() {
            "?" => return Err(Reject::Question),
            "." | "!" | "—" | "–" | "-" | "…" => {}
            other => return Err(Reject::Punct(other.to_string())),
        }
    }
    let words: Vec<&Token> = toks.iter().filter(|t| t.word).collect();
    if words.is_empty() {
        return Err(Reject::NoParse);
    }
    if words.len() > 8 {
        return Err(Reject::TooLong);
    }
    if words.iter().any(|w| w.text.chars().any(|c| c.is_ascii_digit())) {
        return Err(Reject::Digit);
    }
    if let Some(w) = words.iter().find(|w| CONJ.contains(&w.norm.as_str())) {
        return Err(Reject::Conj(w.norm.clone()));
    }
    let mut an = Vec::with_capacity(words.len());
    for (i, w) in words.iter().enumerate() {
        let all = m.forms.get(&w.norm).ok_or_else(|| Reject::Unknown(w.norm.clone()))?;
        an.push(proper_filter(all, w, i, names));
    }
    let cx = Ctx { an, norms: words.iter().map(|w| w.norm.clone()).collect() };
    let dash = toks.iter().position(|t| matches!(t.text.as_str(), "—" | "–" | "-"));

    let mut found: Vec<Fact> = Vec::new();
    let preds: Vec<usize> = (0..words.len()).filter(|&w| cx.has(w, finite)).collect();
    // a dash alongside a verb means two parts ("У нього прізвище — язик зламаєш"), no longer a simple sentence
    if dash.is_some() && !preds.is_empty() {
        return Err(Reject::Punct("dash with verb".into()));
    }
    for &p in &preds {
        for a in cx.an[p].iter().filter(|a| finite(a)) {
            if let Some(f) = verbal(&cx, p, a) {
                if !found.contains(&f) {
                    found.push(f);
                }
            }
        }
    }
    if preds.is_empty() {
        let dash_word = dash.map(|d| toks[..d].iter().filter(|t| t.word).count());
        if let Some(f) = nominal_sentence(&cx, dash_word) {
            found.push(f);
        }
    }
    match found.len() {
        0 => Err(Reject::NoParse),
        1 => Ok(found.pop().expect("one")),
        _ => Err(Reject::Ambiguous),
    }
}

/// Proper names: a lowercase word is not a proper name; a capitalized word mid-sentence is a proper name;
/// a capitalized word at the start is a proper name only if the corpus knows the form as a name (capitalized mid-sentence).
fn proper_filter(all: &[Analysis], w: &Token, i: usize, names: &HashSet<String>) -> Vec<Analysis> {
    let (prop, common): (Vec<Analysis>, Vec<Analysis>) = all.iter().cloned().partition(|a| a.gram.flag(PROP));
    if prop.is_empty() || common.is_empty() {
        return all.to_vec();
    }
    let cap = w.text.chars().next().is_some_and(char::is_uppercase);
    if !cap {
        common
    } else if i > 0 || names.contains(&w.norm) {
        prop
    } else {
        common
    }
}

/// Sentence with verbal predicate `p` (analysis `pa`).
fn verbal(cx: &Ctx, p: usize, pa: &Analysis) -> Option<Fact> {
    let n = cx.an.len();
    let mut taken = vec![false; n];
    let mut members = Vec::new();
    taken[p] = true;
    let mut pred = pa.gram;
    if p > 0 && cx.norms[p - 1] == "не" {
        taken[p - 1] = true;
        pred = pred.with(NEG);
    }
    members.push(Member { rel: "predicate".into(), word: Some(p), lemma: pa.lemma.clone(), gram: pred });
    let copula = COPULA.contains(&crate::morph::lemma_of(&pa.lemma));
    // infinitive after the predicate: after a modal/phasal verb it forms a compound verbal predicate
    // ("хочу піти", "маєш кинути"); after other verbs ("попросив мене дати…") it is a second clause core, skipped
    let is_inf = |a: &Analysis| a.gram.pos() == VERB && a.gram.tense() == 1;
    let infs: Vec<usize> = (p + 1..n).filter(|&w| cx.has(w, is_inf)).collect();
    let mut obj_gov = pa.gov;
    if !infs.is_empty() {
        let modal = MODAL.contains(&crate::morph::lemma_of(&pa.lemma));
        if !modal || infs.len() > 1 {
            return None;
        }
        let w = infs[0];
        let a = cx.pick(w, is_inf)?;
        members.push(Member { rel: "predicate:infinitive".into(), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
        taken[w] = true;
        // objects after the infinitive are governed by it ("хочу пити воду")
        obj_gov = a.gov;
    }

    // prepositional phrases: preposition, [attributes…], noun in the governed case
    let mut heads: Vec<(usize, String, Gram)> = Vec::new();
    let mut w = 0;
    while w < n {
        if taken[w] || !cx.has(w, |a| a.gram.pos() == PREP) {
            w += 1;
            continue;
        }
        let gov = cx.an[w].iter().filter(|a| a.gram.pos() == PREP).fold(0u8, |m, a| m | a.gov);
        let prep = crate::morph::lemma_of(&cx.pick(w, |a| a.gram.pos() == PREP)?.lemma).to_string();
        let mut j = w + 1;
        while j < n && !taken[j] && !cx.has(j, nominal) && cx.has(j, adjectival) {
            j += 1;
        }
        if j >= n || taken[j] {
            return None;
        }
        let adjs: Vec<usize> = (w + 1..j).collect();
        // a pronoun after a preposition ("у мене", "з нього") and "з/із/зі" ("зі стола") — genitive first
        // "у друга", "в сестри" — an animate noun after "у/в" with identical genitive and accusative: genitive
        let anim_uv = matches!(prep.as_str(), "у" | "в")
            && cx.has(j, |a| nominal(a) && a.gram.case() == ROD && a.gram.anim() == 1)
            && cx.has(j, |a| nominal(a) && a.gram.case() == ZNA && a.gram.anim() == 1);
        let order = if anim_uv || cx.has(j, |a| a.gram.flag(PRON)) || matches!(prep.as_str(), "з" | "із" | "зі" | "зо") {
            PP_PRON_CASE_ORDER
        } else {
            PP_CASE_ORDER
        };
        let noun = order.iter().filter(|&&c| gov & (1 << c) != 0).find_map(|&c| {
            cx.an[j].iter().find(|a| {
                nominal(a)
                    && a.gram.case() == c
                    && adjs.iter().all(|&d| cx.has(d, |x| adjectival(x) && adj_agrees(x.gram, a.gram)))
            })
        })?;
        let rel = format!("adverbial:{prep}+{}", CASE_NAMES[noun.gram.case() as usize]);
        for &d in &adjs {
            let x = cx.pick(d, |x| adjectival(x) && adj_agrees(x.gram, noun.gram))?;
            members.push(Member { rel: format!("attribute:{rel}"), word: Some(d), lemma: x.lemma.clone(), gram: x.gram });
            taken[d] = true;
        }
        taken[w] = true;
        taken[j] = true;
        members.push(Member { rel: rel.clone(), word: Some(j), lemma: noun.lemma.clone(), gram: noun.gram });
        heads.push((j, rel, noun.gram));
        w = j + 1;
    }

    // quantity phrases: "багато років" — the noun in the genitive, the quantity word goes into the relation
    for w in 0..n.saturating_sub(1) {
        if !taken[w] && !taken[w + 1] && QUANT.contains(&cx.norms[w].as_str()) {
            if let Some(a) = cx.pick(w + 1, |a| nominal(a) && a.gram.case() == ROD) {
                let rel = format!("quantity:{}", cx.norms[w]);
                members.push(Member { rel: rel.clone(), word: Some(w + 1), lemma: a.lemma.clone(), gram: a.gram });
                heads.push((w + 1, rel, a.gram));
                taken[w] = true;
                taken[w + 1] = true;
            }
        }
    }

    // pre-modifiers: an adjective (or chain of them) before a noun it can agree with
    let mut premod: Vec<Option<usize>> = vec![None; n];
    for w in (0..n.saturating_sub(1)).rev() {
        if taken[w] || !cx.has(w, adjectival) {
            continue;
        }
        let head = if !taken[w + 1] && cx.has(w + 1, |a| nominal(a) && !a.gram.flag(PRON)) {
            Some(w + 1)
        } else {
            premod[w + 1]
        };
        if let Some(h) = head {
            if cx.an[h].iter().any(|na| nominal(na) && cx.has(w, |x| adjectival(x) && adj_agrees(x.gram, na.gram))) {
                premod[w] = Some(h);
            }
        }
    }
    // a noun analysis compatible with all its pre-modifiers
    let fits = |h: usize, a: &Analysis| -> bool {
        (0..n)
            .filter(|&d| premod[d] == Some(h))
            .all(|d| cx.has(d, |x| adjectival(x) && adj_agrees(x.gram, a.gram)))
    };

    // subject: a free noun/pronoun in the nominative agreeing with the predicate
    if pred.tense() != IMPR {
        let mut cands: Vec<usize> = (0..n)
            .filter(|&w| !taken[w] && premod[w].is_none() && cx.has(w, |a| nominal(a) && agrees(pred, a.gram) && fits(w, a)))
            .collect();
        // "Це ви порушили…": "це" alongside another candidate is a particle
        if cands.len() > 1 {
            if let Some(ce) = cands.iter().position(|&w| cx.norms[w] == "це" && cx.has(w, |a| a.gram.pos() == PART)) {
                let w = cands.remove(ce);
                let a = cx.pick(w, |a| a.gram.pos() == PART)?;
                members.push(Member { rel: "particle".into(), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
                taken[w] = true;
            }
        }
        let subj = match cands.len() {
            0 => None,
            1 => Some(cands[0]),
            _ => {
                let only_naz: Vec<usize> = cands
                    .iter()
                    .copied()
                    .filter(|&w| cx.an[w].iter().filter(|a| nominal(a)).all(|a| a.gram.case() == NAZ))
                    .collect();
                let before: Vec<usize> = cands.iter().copied().filter(|&w| w < p).collect();
                if only_naz.len() == 1 {
                    Some(only_naz[0])
                } else if before.len() == 1 {
                    Some(before[0])
                } else {
                    return None;
                }
            }
        };
        match subj {
            Some(s) => {
                let a = cx.pick(s, |a| nominal(a) && agrees(pred, a.gram) && fits(s, a))?;
                taken[s] = true;
                members.push(Member { rel: "subject".into(), word: Some(s), lemma: a.lemma.clone(), gram: a.gram });
                heads.push((s, "subject".into(), a.gram));
            }
            None => members.push(Member {
                rel: "subject".into(),
                word: None,
                lemma: "ø|noun".into(),
                gram: Gram::implicit_subject(pred),
            }),
        }
    }

    // nominal part of the predicate with a copula: an adjective or noun in the nominative/instrumental
    if copula {
        if let Some(w) = (0..n).find(|&w| {
            !taken[w] && premod[w].is_none() && cx.has(w, |a| (adjectival(a) || nominal(a)) && matches!(a.gram.case(), NAZ | ORU))
        }) {
            let a = cx.pick(w, |a| (adjectival(a) || nominal(a)) && matches!(a.gram.case(), NAZ | ORU) && (adjectival(a) || fits(w, a)))?;
            members.push(Member { rel: "predicate:nominal".into(), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
            taken[w] = true;
            heads.push((w, "predicate:nominal".into(), a.gram));
        }
    }

    // remaining nouns: a non-personal genitive right after a noun is an attribute; otherwise an object
    for w in 0..n {
        if taken[w] || premod[w].is_some() || !cx.has(w, nominal) {
            continue;
        }
        // "Це ви порушили…": "це" before the predicate that did not become the subject is a particle
        if cx.norms[w] == "це" && w < p {
            if let Some(a) = cx.pick(w, |a| a.gram.pos() == PART) {
                members.push(Member { rel: "particle".into(), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
                taken[w] = true;
                continue;
            }
        }
        if let Some((_, hrel, _)) = heads.iter().find(|(h, _, g)| *h + 1 == w && !g.flag(PRON)) {
            if let Some(a) = cx.pick(w, |a| nominal(a) && a.gram.case() == ROD && !a.gram.flag(PRON) && fits(w, a)) {
                members.push(Member { rel: format!("attribute:{hrel}"), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
                taken[w] = true;
                heads.push((w, format!("attribute:{hrel}"), a.gram));
                continue;
            }
        }
        // under negation the genitive comes before the dative: "не мечіть бісеру"
        let order: &[u32] = if pred.flag(NEG) { &[ZNA, ROD, DAV, ORU] } else { &OBJ_CASE_ORDER };
        // "попросити допомоги", "випити води": genitive singular, not the homonymous accusative plural
        let gen_sg = governs(obj_gov, ROD, pred.flag(NEG))
            && cx.has(w, |a| nominal(a) && a.gram.case() == ZNA && a.gram.num() == PL)
            && cx.has(w, |a| nominal(a) && a.gram.case() == ROD && a.gram.num() == SG);
        let pick_gen = if gen_sg { cx.pick(w, |a| nominal(a) && a.gram.case() == ROD && a.gram.num() == SG && fits(w, a)) } else { None };
        let Some(a) = pick_gen.or_else(|| {
            order
                .iter()
                .filter(|&&c| governs(obj_gov, c, pred.flag(NEG)))
                .find_map(|&c| cx.pick(w, |a| nominal(a) && a.gram.case() == c && fits(w, a)))
        }) else {
            if cx.has(w, adjectival) {
                continue; // pronominal adjective without a head — handled below
            }
            return None;
        };
        let rel = format!("object:{}", CASE_NAMES[a.gram.case() as usize]);
        if members.iter().any(|m| m.rel == rel) {
            return None; // two objects in the same case ("тебе … цілу вічність") — we do not guess
        }
        members.push(Member { rel: rel.clone(), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
        taken[w] = true;
        heads.push((w, rel, a.gram));
    }

    // pre-modifiers go to their nouns; remaining adjectives go to the nearest agreeing head
    for w in 0..n {
        if taken[w] || !cx.has(w, adjectival) {
            continue;
        }
        let mut hs: Vec<&(usize, String, Gram)> = heads.iter().filter(|(h, _, g)| *h != w && !g.flag(PRON)).collect();
        hs.sort_by_key(|(h, _, _)| (premod[w] != Some(*h), h.abs_diff(w)));
        let Some((a, rel)) = hs.iter().find_map(|(_, rel, g)| {
            cx.pick(w, |a| adjectival(a) && adj_agrees(a.gram, *g)).map(|a| (a, rel.clone()))
        }) else {
            if cx.has(w, |a| matches!(a.gram.pos(), ADV | PART)) {
                continue; // "дуже" is an adverb, not "дужий": handled below
            }
            return None;
        };
        members.push(Member { rel: format!("attribute:{rel}"), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
        taken[w] = true;
    }

    // adverbs and particles
    for w in 0..n {
        if taken[w] {
            continue;
        }
        let (rel, a) = if let Some(a) = cx.pick(w, |a| a.gram.pos() == ADV) {
            ("adverbial", a)
        } else if let Some(a) = cx.pick(w, |a| a.gram.pos() == PART) {
            ("particle", a)
        } else {
            return None;
        };
        members.push(Member { rel: rel.into(), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
        taken[w] = true;
    }
    Some(Fact { gram: pred, members })
}

/// Verbless sentence: "Київ — столиця України.", "Це кіт.", "Кіт білий.".
fn nominal_sentence(cx: &Ctx, dash_word: Option<usize>) -> Option<Fact> {
    let n = cx.an.len();
    let naz = |w: usize| cx.pick(w, |a| (nominal(a) || adjectival(a)) && a.gram.case() == NAZ);
    let (subj, pred) = match (dash_word, n) {
        (Some(1), _) if n >= 2 => (0, 1),
        (None, _) if cx.norms[0] == "це" && n >= 2 => (0, n - 1),
        (None, 2) => (0, 1),
        // "Ти мій друг.", "Ви огидна свиня.": subject, attribute, nominal predicate without a dash
        (None, _) if n >= 3 && cx.has(n - 1, |a| nominal(a) && a.gram.case() == NAZ) => (0, n - 1),
        _ => return None,
    };
    let s = cx.pick(subj, |a| nominal(a) && a.gram.case() == NAZ)?;
    let pa = naz(pred)?;
    if adjectival(pa) && !adj_agrees(pa.gram, s.gram) {
        return None;
    }
    let mut members = vec![
        Member { rel: "predicate".into(), word: Some(pred), lemma: pa.lemma.clone(), gram: pa.gram },
        Member { rel: "subject".into(), word: Some(subj), lemma: s.lemma.clone(), gram: s.gram },
    ];
    // between subject and predicate (after "це") — attributes of the predicate; after the predicate — genitive
    for w in 1..n {
        if w == pred {
            continue;
        }
        let a = if w > pred {
            cx.pick(w, |a| nominal(a) && a.gram.case() == ROD)?
        } else {
            cx.pick(w, |a| adjectival(a) && adj_agrees(a.gram, pa.gram))?
        };
        members.push(Member { rel: "attribute:predicate".into(), word: Some(w), lemma: a.lemma.clone(), gram: a.gram });
    }
    Some(Fact { gram: Gram(0), members })
}

/// Completeness check (not correctness!): from each member's (lemma, grammar) we synthesize the form via
/// VESUM and compare it with the sentence; prepositions, quantity words and "не" are restored from the relation and
/// the flag. The multiset of graph words must equal the multiset of sentence words.
pub fn render_check(text: &str, f: &Fact, m: &Morph) -> Result<(), String> {
    let words: Vec<String> = tokenize(text).into_iter().filter(|t| t.word).map(|t| t.norm).collect();
    let mut want = words.clone();
    want.sort();
    let mut got = Vec::new();
    for mem in &f.members {
        if mem.gram.flag(IMPLICIT) {
            continue;
        }
        let key_gram = Gram(mem.gram.0 & !(1 << NEG));
        let forms = m.synth.get(&(mem.lemma.clone(), key_gram)).ok_or_else(|| format!("no-synthesis {} {key_gram}", mem.lemma))?;
        let original = words.get(mem.word.ok_or("member-without-word")?).cloned().unwrap_or_default();
        if !forms.iter().any(|f| norm(f) == original) {
            return Err(format!("{} {key_gram} → {forms:?}, but the sentence has «{original}»", mem.lemma));
        }
        got.push(original);
        if let Some(prep) = mem.rel.strip_prefix("adverbial:").and_then(|r| r.split_once('+')).map(|(p, _)| p) {
            got.push(norm(prep));
        }
        if let Some(q) = mem.rel.strip_prefix("quantity:") {
            got.push(q.to_string());
        }
        if mem.gram.flag(NEG) {
            got.push("не".into());
        }
    }
    got.sort();
    if got != want {
        return Err(format!("graph-words {got:?} ≠ sentence {want:?}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_strip_stress_and_keep_apostrophes() {
        let t: Vec<String> = tokenize("Ма\u{301}ма їсть м’ясо.").into_iter().filter(|t| t.word).map(|t| t.norm).collect();
        assert_eq!(t, vec!["мама", "їсть", "м'ясо"]);
    }

    /// Negative control of the completeness check: a damaged fact must fail.
    #[test]
    fn render_check_catches_damage() {
        let mut m = Morph::default();
        let g_n = Gram::from_vesum("noun:anim:m:v_naz");
        let g_v = Gram::from_vesum("verb:imperf:pres:s:3");
        m.synth.insert(("кіт|noun".into(), g_n), vec!["кіт".into()]);
        m.synth.insert(("спати|verb".into(), g_v), vec!["спить".into()]);
        let good = Fact {
            gram: g_v,
            members: vec![
                Member { rel: "predicate".into(), word: Some(1), lemma: "спати|verb".into(), gram: g_v },
                Member { rel: "subject".into(), word: Some(0), lemma: "кіт|noun".into(), gram: g_n },
            ],
        };
        assert!(render_check("Кіт спить.", &good, &m).is_ok());
        let mut lost = good.clone();
        lost.members.pop();
        assert!(render_check("Кіт спить.", &lost, &m).is_err(), "lost member");
        let mut wrong = good.clone();
        wrong.members[1].gram = Gram::from_vesum("noun:anim:m:v_zna");
        assert!(render_check("Кіт спить.", &wrong, &m).is_err(), "different grammar");
        let mut neg = good.clone();
        neg.members[0].gram = g_v.with(NEG);
        assert!(render_check("Кіт спить.", &neg, &m).is_err(), "extra «не»");
    }
}
