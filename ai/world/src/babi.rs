//! bAbI (Weston et al. 2015, arXiv:1502.05698; CC BY 3.0) — a level-3 check: does the snake itself keep
//! the world of a story (who is where, who has what, what was given to whom, how things stand relative to each other) and answer from it, without training
//! on the questions and without trigger words in the code.
//!
//! - A sentence is a UD tree (`en`); what a verb does is a level-1 class (`global::verb_class`:
//!   move, get, give, lose); relations between things (north_of, left_of, bigger_than, fit_inside, parts
//!   of the day) are the level-1 `relation` block (`global/seeds/shortcuts/relations.md`): inverses, vectors,
//!   transitivity, order.
//! - The story world is temporary (level 3): facts only from the text, conclusions only through level-1
//!   knowledge. What is missing gives the answer «?» with a reason (gap = signal to extend level 1).

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

use anyhow::{Context, Result};
use en::annotate::Word;

use crate::tree::{Rel, UPos, annotator};

/// A sentence with its tree.
struct S {
    w: Vec<Word>,
    kids: Vec<Vec<usize>>,
}

fn parse(text: &str) -> Result<S> {
    let a = annotator()?;
    let forms: Vec<String> = a.tokenize(text).into_iter().map(|t| t.form).collect();
    let w = if forms.is_empty() { Vec::new() } else { a.annotate(&forms) };
    if std::env::var("BABI_DEBUG").is_ok() {
        eprintln!("{text}");
        for (i, x) in w.iter().enumerate() {
            eprintln!("  {} {} {} {:?} {} {:?}", i + 1, x.form, x.lemma, x.upos, x.head, x.rel);
        }
    }
    let mut kids = vec![Vec::new(); w.len()];
    for (i, x) in w.iter().enumerate() {
        if x.head > 0 && x.head <= w.len() {
            kids[x.head - 1].push(i);
        }
    }
    Ok(S { w, kids })
}

impl S {
    fn root(&self) -> Option<usize> {
        (0..self.w.len()).find(|&i| self.w[i].head == 0)
    }
    fn lem(&self, i: usize) -> String {
        let x = &self.w[i];
        // names: the form; a lemma with non-alphabetic characters (a name confused with an address from EWT) — also the form
        let cap = x.form.chars().next().is_some_and(char::is_uppercase);
        // «Taras» at the start of a sentence: not a word of the en lexicon — a name, without a lemma («tara»)
        let unknown = cap && en::dict::form(&x.form.to_lowercase()).is_none();
        if x.upos == UPos::PROPN || cap && (i > 0 || unknown) || !x.lemma.chars().all(|c| c.is_alphabetic() || c == '\'' || c == '-') {
            x.form.to_lowercase()
        } else {
            x.lemma.to_lowercase()
        }
    }
    fn kids_of(&self, i: usize, r: Rel) -> Vec<usize> {
        self.kids[i].iter().copied().filter(|&k| self.w[k].rel.base() == r).collect()
    }
    /// Noun by tag or by structure: with an article or a preposition («the attic» tagged ADJ is a place too).
    fn is_nominal(&self, i: usize) -> bool {
        matches!(self.w[i].upos, UPos::NOUN | UPos::PROPN | UPos::PRON)
            || matches!(self.w[i].rel.base(), Rel::Obl | Rel::Nmod | Rel::Obj | Rel::Nsubj | Rel::Root) && self.kids[i].iter().any(|&k| matches!(self.w[k].rel.base(), Rel::Det | Rel::Case))
    }
    /// Noun phrase as an entity name: modifier + head (+ «of …»): «the pink rectangle» → pink_rectangle,
    /// «the box of chocolates» → box_of_chocolate.
    fn np(&self, i: usize) -> String {
        self.np_cov(i, true).0
    }
    /// Noun phrase and the nodes it covers; `with_of` — with «of …».
    fn np_cov(&self, i: usize, with_of: bool) -> (String, Vec<usize>) {
        let mut parts: Vec<(usize, String)> = self.kids[i]
            .iter()
            .copied()
            .filter(|&k| matches!(self.w[k].rel, Rel::Amod | Rel::Compound) && global::relation(&self.lem(k), "order").is_none())
            .map(|k| (k, self.lem(k)))
            .collect();
        parts.push((i, self.lem(i)));
        parts.sort();
        let mut cov: Vec<usize> = parts.iter().map(|p| p.0).collect();
        let mut s: Vec<String> = parts.into_iter().map(|p| p.1).collect();
        if with_of {
            for k in self.kids_of(i, Rel::Nmod) {
                if self.kids_of(k, Rel::Case).iter().any(|&c| self.lem(c) == "of") {
                    s.push("of".into());
                    s.push(self.lem(k));
                    cov.push(k);
                }
            }
        }
        (s.join("_"), cov)
    }
    fn case(&self, i: usize) -> String {
        self.kids_of(i, Rel::Case).iter().map(|&c| self.lem(c)).collect::<Vec<_>>().join("_")
    }
    /// Group through coordination: «John and Mary» / «the school or the office» → [john, mary], flag «or».
    fn group(&self, i: usize) -> (Vec<usize>, bool) {
        let mut v = vec![i];
        let mut or = false;
        for k in self.kids_of(i, Rel::Conj) {
            v.push(k);
            or |= self.kids_of(k, Rel::Cc).iter().any(|&c| matches!(self.lem(c).as_str(), "or" | "nor"));
        }
        (v, or)
    }
    /// Negation at a node: «not», «no longer».
    fn negated(&self, i: usize) -> bool {
        self.kids[i].iter().any(|&k| {
            let l = self.lem(k);
            self.w[k].rel.base() == Rel::Advmod && (l == "not" || l == "n't" || l == "never" || self.kids[k].iter().any(|&c| self.lem(c) == "no"))
        })
    }
    fn words(&self) -> Vec<String> {
        (0..self.w.len()).map(|i| self.lem(i)).collect()
    }
}

/// Level-1 relation name from a word and its preposition: north + of → north_of,
/// bigger + than → bigger_than, fits + inside → fit_inside, above → above.
fn rel_name(s: &S, i: usize, prep: &str) -> Option<String> {
    let l = s.lem(i);
    let f = s.w[i].form.to_lowercase();
    let mut c = Vec::new();
    for b in [&l, &f] {
        if !prep.is_empty() {
            c.push(format!("{b}_{prep}"));
        }
        c.push(b.clone());
    }
    c.into_iter().find(|n| global::relation(n, "inverse").is_some() || global::relation(n, "same").is_some())
}

/// Normal form of a relation: same= expanded (fit_inside → smaller_than).
fn canon(r: &str) -> String {
    global::relation(r, "same").unwrap_or(r).to_string()
}

fn vec_of(r: &str) -> Option<(i32, i32)> {
    let v = global::relation(r, "vec")?;
    let (a, b) = v.split_once(',')?;
    Some((a.parse().ok()?, b.parse().ok()?))
}

/// What we know about a person's location.
#[derive(Clone, Debug)]
enum Pos {
    At(String),
    Not(String),
    Either(Vec<String>),
}

#[derive(Default)]
pub struct Story {
    t: usize,
    /// a person's location: current knowledge
    pos: BTreeMap<String, Pos>,
    /// history of a person's locations: (order of the part of day, sentence number, place)
    hist: BTreeMap<String, Vec<(u32, usize, String)>>,
    /// who holds a thing
    holder: BTreeMap<String, String>,
    /// what a person carries (order of taking)
    carry: BTreeMap<String, Vec<String>>,
    /// a thing's locations in order of change
    ohist: BTreeMap<String, Vec<String>>,
    /// transfers: (who, what, to whom)
    gives: Vec<(String, String, String)>,
    /// relation facts: (a, rel, b)
    rels: Vec<(String, String, String)>,
    /// «X is a Y»
    isa: BTreeMap<String, String>,
    /// «X is white / hungry»: attributes in text order
    attr: Vec<(String, String)>,
    /// «Mice are afraid of wolves»: (subject, predicate, object)
    preds: Vec<(String, String, String)>,
    last_subj: Vec<String>,
    /// names that were capitalized in the story (in questions they may be lowercase)
    names: BTreeSet<String>,
}

fn pronoun(l: &str) -> bool {
    matches!(l, "he" | "she" | "they" | "him" | "her" | "them" | "it")
}

impl Story {
    fn subjects(&mut self, s: &S, v: usize) -> Vec<String> {
        let time = |k: usize| global::relation(&s.lem(k), "order").is_some();
        let n = s.kids[v].iter().copied().find(|&k| matches!(s.w[k].rel, Rel::Nsubj | Rel::NsubjPass) && s.is_nominal(k) && !time(k)).or_else(|| {
            // the tree got a name wrong: the first noun without a preposition before the verb
            // (and «the suitcase fits» with «fits» as a noun: compound at the predicate itself)
            (0..v).find(|&k| matches!(s.w[k].upos, UPos::NOUN | UPos::PROPN | UPos::PRON | UPos::X) && s.case(k).is_empty() && !time(k) && (s.w[k].rel != Rel::Compound || s.w[k].head == v + 1))
        });
        let Some(n) = n else { return self.last_subj.clone() };
        if pronoun(&s.lem(n)) {
            return self.last_subj.clone();
        }
        let (g, _) = s.group(n);
        for &k in &g {
            let f = s.w[k].form.to_lowercase();
            if s.w[k].form.chars().next().is_some_and(char::is_uppercase) && (s.w[k].upos == UPos::PROPN || k > 0) {
                self.names.insert(f.clone());
            }
            // «Taras» at the start of a sentence got the lemma «tara»: once the name appears, fix the world
            let l = s.w[k].lemma.to_lowercase();
            if l != f && self.names.contains(&f) && self.entities().contains(&l) {
                self.rename(&l, &f);
            }
        }
        let g: Vec<usize> = g;
        let v: Vec<String> = g.into_iter().map(|k| if self.names.contains(&s.w[k].form.to_lowercase()) { s.w[k].form.to_lowercase() } else { s.np(k) }).collect();
        self.last_subj = v.clone();
        v
    }

    /// Rename an entity throughout the world (fixing a name spoiled by lemmatization).
    fn rename(&mut self, from: &str, to: &str) {
        let f = |x: &mut String| {
            if x == from {
                *x = to.to_string();
            }
        };
        let rk = |m: &mut BTreeMap<String, _>| {
            if let Some(v) = m.remove(from) {
                m.insert(to.to_string(), v);
            }
        };
        if let Some(v) = self.pos.remove(from) {
            self.pos.insert(to.into(), v);
        }
        if let Some(v) = self.hist.remove(from) {
            self.hist.insert(to.into(), v);
        }
        if let Some(v) = self.carry.remove(from) {
            self.carry.insert(to.into(), v);
        }
        rk(&mut self.isa);
        for v in self.holder.values_mut() {
            f(v);
        }
        for (a, _, c) in self.gives.iter_mut() {
            f(a);
            f(c);
        }
        for (a, _) in self.attr.iter_mut() {
            f(a);
        }
        for x in self.last_subj.iter_mut() {
            f(x);
        }
    }

    fn time_order(s: &S) -> Option<u32> {
        (0..s.w.len()).find_map(|i| global::relation(&s.lem(i), "order").and_then(|o| o.parse().ok()))
    }

    fn place_of(&self, p: &str) -> Option<String> {
        match self.pos.get(p) {
            Some(Pos::At(x)) => Some(x.clone()),
            _ => None,
        }
    }

    fn move_to(&mut self, who: &str, place: &str, order: u32) {
        self.pos.insert(who.into(), Pos::At(place.into()));
        self.hist.entry(who.into()).or_default().push((order, self.t, place.into()));
        for o in self.carry.get(who).cloned().unwrap_or_default() {
            let h = self.ohist.entry(o).or_default();
            if h.last().map(String::as_str) != Some(place) {
                h.push(place.into());
            }
        }
    }

    fn take(&mut self, who: &str, obj: &str) {
        if let Some(prev) = self.holder.insert(obj.into(), who.into()) {
            self.carry.entry(prev).or_default().retain(|x| x != obj);
        }
        self.carry.entry(who.into()).or_default().push(obj.into());
        if let Some(p) = self.place_of(who) {
            let h = self.ohist.entry(obj.into()).or_default();
            if h.last() != Some(&p) {
                h.push(p);
            }
        }
    }

    fn drop_(&mut self, who: &str, obj: &str) {
        if self.holder.get(obj).map(String::as_str) == Some(who) {
            self.holder.remove(obj);
        }
        self.carry.entry(who.into()).or_default().retain(|x| x != obj);
    }

    /// Reads one sentence into the world. Returns a reason if it did not understand.
    pub fn read(&mut self, text: &str) -> Result<Option<String>> {
        self.t += 1;
        let s = parse(text)?;
        let Some(r0) = s.root() else { return Ok(Some("empty".into())) };
        // «Daniel journeyed …» with a name as root and the verb as acl: the verb is the head
        let r = if s.is_nominal(r0) && s.kids_of(r0, Rel::Cop).is_empty() {
            s.kids[r0].iter().copied().find(|&k| s.w[k].upos == UPos::VERB && matches!(s.w[k].rel.base(), Rel::Acl | Rel::Advcl | Rel::Parataxis | Rel::Conj)).unwrap_or(r0)
        } else {
            r0
        };
        // «Following that she drove …»: a root without a class, and a verb with a class nearby — that is the head
        let vclass = |k: usize| global::verb_class(&s.lem(k)).is_some();
        let r = if s.w[r].upos == UPos::VERB && !vclass(r) && s.lem(r) != "be" {
            (0..s.w.len()).find(|&k| k != r && s.w[k].upos == UPos::VERB && vclass(k)).unwrap_or(r)
        } else {
            r
        };
        let order = Self::time_order(&s).unwrap_or(0);
        let has_cop = !s.kids_of(r, Rel::Cop).is_empty();
        if s.w[r].upos == UPos::VERB && !has_cop {
            let mut verb = s.lem(r);
            if let Some(&p) = s.kids[r].iter().find(|&&k| s.w[k].rel == Rel::CompoundPrt) {
                verb = format!("{verb}_{}", s.lem(p));
            }
            let subj = self.subjects(&s, r);
            let objs = s.kids_of(r, Rel::Obj);
            let mut obls = s.kids_of(r, Rel::Obl);
            for a in s.kids_of(r, Rel::Advmod) {
                obls.extend(s.kids_of(a, Rel::Obl));
            }
            obls.sort_by_key(|&k| (!matches!(s.case(k).as_str(), "to" | "into"), k));
            // «delivered the juice to Olena» with «to Olena» attached to the thing — also the recipient
            let obl_to = obls.iter().copied().chain(objs.iter().flat_map(|&o| s.kids_of(o, Rel::Nmod))).find(|&k| s.is_nominal(k) && s.case(k) == "to");
            let class = global::verb_class(&verb).map(|c| c.0);
            // relation verb: «The suitcase fits inside the box»
            if let Some(&o) = obls.iter().find(|&&k| rel_name(&s, r, &s.case(k)).is_some()) {
                let rn = canon(&rel_name(&s, r, &s.case(o)).unwrap());
                for a in &subj {
                    self.rels.push((a.clone(), rn.clone(), s.np(o)));
                }
                return Ok(None);
            }
            match class {
                Some("move") => {
                    let Some(&d) = obls.iter().find(|&&k| s.is_nominal(k)) else { return Ok(Some(format!("move without a place: {text}"))) };
                    let place = s.np(d);
                    for a in subj {
                        self.move_to(&a, &place, order);
                    }
                }
                // have/hold/take + thing: the thing is with them (possession state — same as received)
                Some("get") | Some("have") if !objs.is_empty() => {
                    let o = objs[0];
                    for a in subj {
                        self.take(&a, &s.np(o));
                    }
                }
                Some("lose") => {
                    let Some(&o) = objs.first() else { return Ok(Some(format!("lose without a thing: {text}"))) };
                    for a in subj {
                        self.drop_(&a, &s.np(o));
                    }
                }
                Some("give") => {
                    let Some(&o) = objs.first() else { return Ok(Some(format!("give without a thing: {text}"))) };
                    let recv = obl_to.or_else(|| s.kids_of(r, Rel::Iobj).first().copied());
                    let Some(rc) = recv else { return Ok(Some(format!("give without a recipient: {text}"))) };
                    let (obj, rcv) = (s.np(o), s.np(rc));
                    for a in subj {
                        self.drop_(&a, &obj);
                        self.take(&rcv, &obj);
                        self.gives.push((a, obj.clone(), rcv.clone()));
                    }
                }
                // «Taras is in the garden» with «is» as a verb: a place
                _ if verb == "be" => {
                    let Some(&d) = obls.iter().find(|&&k| s.is_nominal(k) && matches!(s.case(k).as_str(), "in" | "at")) else { return Ok(Some(format!("be without a place: {text}"))) };
                    let place = s.np(d);
                    let neg = s.negated(r);
                    for a in subj {
                        if neg {
                            self.pos.insert(a, Pos::Not(place.clone()));
                        } else {
                            self.move_to(&a, &place, order);
                        }
                    }
                }
                _ => return Ok(Some(format!("verb «{verb}» has no class at level 1"))),
            }
            return Ok(None);
        }
        // copula: the predicate is the root
        let subj = self.subjects(&s, r);
        let neg = s.negated(r);
        // relation: «The kitchen is north of the garden», «The chest is bigger than the box», «… to the left of …»
        let subj_nodes: Vec<usize> = s.kids[r].iter().copied().filter(|&k| matches!(s.w[k].rel, Rel::Nsubj | Rel::NsubjPass)).collect();
        let rel_target = |s: &S| -> Option<(String, usize)> {
            // the relation word is the root or its child; the object is a noun with a preposition among the root and its children
            let mut objs: Vec<usize> = std::iter::once(r).chain(s.kids[r].iter().copied()).filter(|&o| s.is_nominal(o) && !subj_nodes.contains(&o)).collect();
            for k in std::iter::once(r).chain(s.kids[r].iter().copied()).collect::<Vec<_>>() {
                objs.extend(s.kids[k].iter().copied().filter(|&o| s.is_nominal(o) && matches!(s.w[o].rel.base(), Rel::Obl | Rel::Nmod)));
            }
            for k in std::iter::once(r).chain(s.kids[r].iter().copied()) {
                for &o in &objs {
                    if o == k && s.case(o).is_empty() {
                        continue;
                    }
                    if let Some(n) = rel_name(s, k, &s.case(o)) {
                        return Some((canon(&n), o));
                    }
                }
            }
            None
        };
        if let Some((rn, o)) = rel_target(&s) {
            // tree without a subject («The box of chocolates fits …» with «fits» as a noun): the subject is the noun
            // phrase before the relation word (lemma, without articles, «of» stays)
            let has_nsubj = std::iter::once(r).chain(s.kids[r].iter().copied()).any(|k| s.kids[k].iter().any(|&c| matches!(s.w[c].rel, Rel::Nsubj | Rel::NsubjPass) && s.is_nominal(c)));
            let subj = if has_nsubj {
                subj
            } else {
                let k = s.w[o].head.saturating_sub(1);
                let span: Vec<String> = (0..k).filter(|&i| !matches!(s.w[i].upos, UPos::DET | UPos::PUNCT)).map(|i| s.lem(i)).collect();
                if span.is_empty() { subj } else { vec![span.join("_")] }
            };
            for a in &subj {
                self.rels.push((a.clone(), rn.clone(), s.np(o)));
            }
            return Ok(None);
        }
        match s.w[r].upos {
            // «Mary is in the school», «Bill is either in the school or the office», «Mary is no longer in the kitchen»
            UPos::NOUN | UPos::PROPN if s.case(r) == "in" || s.case(r) == "at" => {
                let (g, or) = s.group(r);
                let places: Vec<String> = g.into_iter().map(|k| s.np(k)).collect();
                for a in subj {
                    let p = if neg {
                        Pos::Not(places[0].clone())
                    } else if or && places.len() > 1 {
                        Pos::Either(places.clone())
                    } else {
                        self.hist.entry(a.clone()).or_default().push((order, self.t, places[0].clone()));
                        Pos::At(places[0].clone())
                    };
                    self.pos.insert(a, p);
                }
            }
            // «Gertrude is a mouse»
            UPos::NOUN => {
                for a in subj {
                    self.isa.insert(a, s.np(r));
                }
            }
            // «Lily is white», «Mice are afraid of wolves», «John is hungry»
            UPos::ADJ => {
                let obj = s.kids[r].iter().copied().find(|&k| s.is_nominal(k) && matches!(s.w[k].rel.base(), Rel::Obl | Rel::Nmod));
                for a in subj {
                    match obj {
                        Some(o) => self.preds.push((a, format!("{}_{}", s.lem(r), s.case(o)), s.np(o))),
                        None => self.attr.push((a, s.lem(r))),
                    }
                }
            }
            _ => return Ok(Some(format!("copula with «{}» ({:?})", s.lem(r), s.w[r].upos))),
        }
        Ok(None)
    }

    /// All story entities: people, places, things, classes.
    fn entities(&self) -> BTreeSet<String> {
        let mut e = BTreeSet::new();
        for (k, p) in &self.pos {
            e.insert(k.clone());
            match p {
                Pos::At(x) | Pos::Not(x) => {
                    e.insert(x.clone());
                }
                Pos::Either(v) => e.extend(v.iter().cloned()),
            }
        }
        for (k, h) in &self.hist {
            e.insert(k.clone());
            e.extend(h.iter().map(|x| x.2.clone()));
        }
        for (k, v) in &self.holder {
            e.insert(k.clone());
            e.insert(v.clone());
        }
        for (k, v) in &self.ohist {
            e.insert(k.clone());
            e.extend(v.iter().cloned());
        }
        for (a, _, c) in self.gives.iter().chain(self.rels.iter()).chain(self.preds.iter()) {
            e.insert(a.clone());
            e.insert(c.clone());
        }
        for (k, v) in &self.isa {
            e.insert(k.clone());
            e.insert(v.clone());
        }
        e.extend(self.attr.iter().map(|x| x.0.clone()));
        e.extend(self.carry.keys().cloned());
        e
    }

    fn class_of(&self, x: &str) -> Option<String> {
        self.isa.get(x).cloned()
    }

    /// Whether «a rel b» follows from the facts through level-1 knowledge (inverse, transitivity, vectors).
    fn holds(&self, a: &str, rel: &str, b: &str) -> Option<bool> {
        let rel = canon(rel);
        if let Some((dx, dy)) = vec_of(&rel) {
            let d = self.offset(a, b)?;
            // on the relation's axis: the sign matches; whatever is on the other axis (bAbI 17)
            let along = d.0 * dx + d.1 * dy;
            return Some(along > 0);
        }
        if global::relation(&rel, "transitive").is_some() {
            let inv = global::relation(&rel, "inverse").map(canon);
            if self.reach(a, &rel, b) {
                return Some(true);
            }
            if inv.as_deref().is_some_and(|i| self.reach(a, i, b)) {
                return Some(false);
            }
            return None;
        }
        None
    }

    /// Relation edges taking the inverse into account.
    fn edges(&self, rel: &str) -> Vec<(String, String)> {
        let inv = global::relation(rel, "inverse").map(canon);
        let mut e = Vec::new();
        for (a, r, b) in &self.rels {
            if r == rel {
                e.push((a.clone(), b.clone()));
            }
            if inv.as_deref() == Some(r.as_str()) {
                e.push((b.clone(), a.clone()));
            }
        }
        e
    }

    fn reach(&self, a: &str, rel: &str, b: &str) -> bool {
        let e = self.edges(rel);
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([a.to_string()]);
        while let Some(x) = q.pop_front() {
            for (u, v) in &e {
                if *u == x && seen.insert(v.clone()) {
                    if v == b {
                        return true;
                    }
                    q.push_back(v.clone());
                }
            }
        }
        false
    }

    /// Offset of a relative to b on the plane (sum of relation vectors along the fact path).
    fn offset(&self, a: &str, b: &str) -> Option<(i32, i32)> {
        let mut adj: BTreeMap<String, Vec<(String, (i32, i32))>> = BTreeMap::new();
        for (x, r, y) in &self.rels {
            if let Some((dx, dy)) = vec_of(r) {
                adj.entry(x.clone()).or_default().push((y.clone(), (dx, dy)));
                adj.entry(y.clone()).or_default().push((x.clone(), (-dx, -dy)));
            }
        }
        // x = y + v: from b to a
        let mut seen = BTreeMap::from([(b.to_string(), (0, 0))]);
        let mut q = VecDeque::from([b.to_string()]);
        while let Some(y) = q.pop_front() {
            let d = seen[&y];
            for (x, v) in adj.get(&y).cloned().unwrap_or_default() {
                // edge y→x with vector v means «y rel x» ⇒ y = x + v ⇒ x = y − v
                if !seen.contains_key(&x) {
                    seen.insert(x.clone(), (d.0 - v.0, d.1 - v.1));
                    q.push_back(x);
                }
            }
        }
        seen.get(a).copied()
    }

    /// Path in steps (n, s, e, w) from `from` to `to`.
    fn path(&self, from: &str, to: &str) -> Option<Vec<&'static str>> {
        // «A north_of B»: from B to A — step n; from A to B — the inverse step
        let mut adj: BTreeMap<String, Vec<(String, &'static str)>> = BTreeMap::new();
        for (a, r, b) in &self.rels {
            let (Some(st), Some(inv)) = (global::relation(r, "step"), global::relation(r, "inverse")) else { continue };
            let Some(ist) = global::relation(inv, "step") else { continue };
            adj.entry(b.clone()).or_default().push((a.clone(), st));
            adj.entry(a.clone()).or_default().push((b.clone(), ist));
        }
        let mut prev: BTreeMap<String, (String, &'static str)> = BTreeMap::new();
        let mut q = VecDeque::from([from.to_string()]);
        let mut seen = BTreeSet::from([from.to_string()]);
        while let Some(x) = q.pop_front() {
            if x == to {
                let mut out = Vec::new();
                let mut c = x;
                while let Some((p, st)) = prev.get(&c).cloned() {
                    out.push(st);
                    c = p;
                }
                out.reverse();
                return Some(out);
            }
            for (y, st) in adj.get(&x).cloned().unwrap_or_default() {
                if seen.insert(y.clone()) {
                    prev.insert(y.clone(), (x.clone(), st));
                    q.push_back(y);
                }
            }
        }
        None
    }

    /// Answer to a question with an explanation; «?» is a gap.
    pub fn ask(&self, q: &str) -> Result<(String, String)> {
        // story names in a question may be lowercase — restore the case so the tree is correct
        let recased: Vec<String> = q
            .split_whitespace()
            .map(|t| {
                let core = t.trim_matches(|c: char| !c.is_alphanumeric());
                if self.names.contains(core) { t.replacen(core, &format!("{}{}", core[..1].to_uppercase(), &core[1..]), 1) } else { t.to_string() }
            })
            .collect();
        let s = parse(&recased.join(" "))?;
        let ws = s.words();
        let first = ws.first().cloned().unwrap_or_default();
        let has = |w: &str| ws.iter().any(|x| x == w);
        let ents = self.entities();
        // relation in the question: word + preposition (ADP child, ADP to the right, the object's preposition)
        let qrel = (0..s.w.len()).find_map(|i| {
            let preps: Vec<String> = s.kids[i]
                .iter()
                .copied()
                .filter(|&k| s.w[k].rel.base() == Rel::Case || s.w[k].upos == UPos::ADP)
                .map(|k| s.lem(k))
                .chain((i + 1..s.w.len()).take(1).filter(|&k| s.w[k].upos == UPos::ADP).map(|k| s.lem(k)))
                .chain(s.kids[i].iter().flat_map(|&k| s.kids_of(k, Rel::Case)).map(|c| s.lem(c)))
                .collect();
            preps.iter().find_map(|p| rel_name(&s, i, p)).or_else(|| rel_name(&s, i, "")).map(|n| (i, canon(&n)))
        });
        // question entities: first those in the story (longest group), then unknown nouns
        let mut covered = BTreeSet::new();
        let mut nps: Vec<(usize, String)> = Vec::new();
        for i in 0..s.w.len() {
            if !matches!(s.w[i].upos, UPos::NOUN | UPos::PROPN | UPos::ADV | UPos::ADJ) || qrel.as_ref().is_some_and(|q| q.0 == i) || covered.contains(&i) {
                continue;
            }
            let full = s.np_cov(i, true);
            let noof = s.np_cov(i, false);
            // a name in the story may have gone in with a lemma («Taras» → tara) or the form — try both
            for (name, cov) in [full, noof, (s.lem(i), vec![i]), (s.w[i].lemma.to_lowercase(), vec![i]), (s.w[i].form.to_lowercase(), vec![i])] {
                if ents.contains(&name) {
                    covered.extend(cov);
                    nps.push((i, name));
                    break;
                }
            }
        }
        for i in 0..s.w.len() {
            if matches!(s.w[i].upos, UPos::NOUN | UPos::PROPN) && !covered.contains(&i) && !nps.iter().any(|x| x.0 == i) && qrel.as_ref().is_none_or(|q| q.0 != i) && !matches!(s.lem(i).as_str(), "object" | "color" | "colour") {
                nps.push((i, s.lem(i)));
            }
        }
        nps.sort();
        nps.dedup_by(|a, b| a.1 == b.1);
        // path: «How do you go from the kitchen to the garden?»
        if first == "how" && has("from") {
            let (Some(a), Some(b)) = (nps.first(), nps.get(1)) else { return Ok(("?".into(), "path without two places".into())) };
            return Ok(match self.path(&a.1, &b.1) {
                Some(p) => (p.join(","), format!("path by facts from {} to {}", a.1, b.1)),
                None => ("?".into(), format!("no path from {} to {}", a.1, b.1)),
            });
        }
        // how many they carry
        if first == "how" && has("many") {
            let Some(p) = nps.first() else { return Ok(("?".into(), "whom?".into())) };
            let n = self.carry.get(&p.1).map_or(0, Vec::len);
            const N: &[&str] = &["none", "one", "two", "three", "four", "five", "six"];
            return Ok((N.get(n).unwrap_or(&"many").to_string(), format!("{} carries {:?}", p.1, self.carry.get(&p.1))));
        }
        // yes/no: «Is John in the kitchen?», «Is the box bigger than …», «Does the box fit in …»
        if matches!(first.as_str(), "be" | "is" | "do" | "does") {
            if let Some((_, rn)) = &qrel {
                let (Some(a), Some(b)) = (nps.first(), nps.get(1)) else { return Ok(("?".into(), "relation without two things".into())) };
                return Ok(match self.holds(&a.1, rn, &b.1) {
                    Some(true) => ("yes".into(), format!("{} {rn} {} follows", a.1, b.1)),
                    Some(false) => ("no".into(), format!("{} {rn} {} — the opposite", a.1, b.1)),
                    None => ("?".into(), format!("{} {rn} {} does not follow", a.1, b.1)),
                });
            }
            let (Some(a), Some(b)) = (nps.first(), nps.get(1)) else { return Ok(("?".into(), "yes/no without two names".into())) };
            return Ok(match self.pos.get(&a.1) {
                Some(Pos::At(x)) => (if *x == b.1 { "yes" } else { "no" }.into(), format!("{} in {x}", a.1)),
                Some(Pos::Not(x)) => (if *x == b.1 { "no".into() } else { "maybe".into() }, format!("{} not in {x}", a.1)),
                Some(Pos::Either(v)) => (if v.contains(&b.1) { "maybe" } else { "no" }.into(), format!("{} in one of {v:?}", a.1)),
                None => ("?".into(), format!("nothing about {}", a.1)),
            });
        }
        if first == "why" {
            let Some(p) = nps.first() else { return Ok(("?".into(), "of what?".into())) };
            return Ok(match self.attr.iter().rev().find(|(x, _)| *x == p.1) {
                Some((_, st)) => (st.clone(), format!("{} — {st}", p.1)),
                None => ("?".into(), format!("state of {} unknown", p.1)),
            });
        }
        if first == "where" {
            let Some(p) = nps.first() else { return Ok(("?".into(), "where — who?".into())) };
            if has("will") {
                // motivation: state → where to go (level 1)
                let Some((_, st)) = self.attr.iter().rev().find(|(x, _)| *x == p.1) else { return Ok(("?".into(), format!("state of {} unknown", p.1))) };
                return Ok(match global::relation(st, "go") {
                    Some(g) => (g.into(), format!("{st} → {g} (level 1)")),
                    None => ("?".into(), format!("gap: where they go when {st}")),
                });
            }
            if has("before") {
                let Some(b) = nps.get(1) else { return Ok(("?".into(), "before what?".into())) };
                // person: history of locations over time; thing: history of locations
                let seq: Vec<String> = if let Some(h) = self.hist.get(&p.1) {
                    let mut h = h.clone();
                    h.sort();
                    h.into_iter().map(|x| x.2).collect()
                } else {
                    self.ohist.get(&p.1).cloned().unwrap_or_default()
                };
                let Some(k) = seq.iter().rposition(|x| *x == b.1) else { return Ok(("?".into(), format!("{} was not in {}", p.1, b.1))) };
                return Ok(match k.checked_sub(1).and_then(|j| seq.get(j)) {
                    Some(x) => (x.clone(), format!("{} places {seq:?}", p.1)),
                    None => ("?".into(), format!("nothing before {}: {seq:?}", b.1)),
                });
            }
            if let Some(Pos::At(x)) = self.pos.get(&p.1) {
                return Ok((x.clone(), format!("{} went to {x}", p.1)));
            }
            if let Some(h) = self.holder.get(&p.1) {
                if let Some(x) = self.place_of(h) {
                    return Ok((x, format!("{} in {h}", p.1)));
                }
            }
            return Ok(match self.ohist.get(&p.1).and_then(|h| h.last()) {
                Some(x) => (x.clone(), format!("{} stayed in {x}", p.1)),
                None => ("?".into(), format!("where {} is — unknown", p.1)),
            });
        }
        if first == "what" || first == "who" {
            // relation: «What is north of the kitchen?» / «What is the kitchen north of?»
            if let (Some((ri, rn)), Some(b)) = (&qrel, nps.first()) {
                let inv = global::relation(rn, "inverse").map(canon);
                let after = b.0 > *ri; // «north of the kitchen» — looking for a: a rel b
                for (x, r, y) in self.rels.iter().rev() {
                    if after && *r == *rn && *y == b.1 || !after && inv.as_deref() == Some(r.as_str()) && *y == b.1 {
                        return Ok((x.clone(), format!("{x} {r} {y}")));
                    }
                    if after && inv.as_deref() == Some(r.as_str()) && *x == b.1 || !after && *r == *rn && *x == b.1 {
                        return Ok((y.clone(), format!("{x} {r} {y}")));
                    }
                }
                return Ok(("?".into(), format!("no {rn} for {}", b.1)));
            }
            // what they carry
            if has("carry") {
                let Some(p) = nps.first() else { return Ok(("?".into(), "who carries?".into())) };
                let v = self.carry.get(&p.1).cloned().unwrap_or_default();
                return Ok((if v.is_empty() { "nothing".into() } else { v.join(",") }, format!("{} carries", p.1)));
            }
            // transfers
            let verb = (0..s.w.len()).find(|&i| s.w[i].upos == UPos::VERB).map(|i| s.lem(i)).unwrap_or_default();
            if matches!(global::verb_class(&verb).map(|c| c.0), Some("give") | Some("get")) && !self.gives.is_empty() {
                let recv_verb = global::verb_class(&verb).map(|c| c.0) == Some("get");
                let vi = (0..s.w.len()).find(|&i| s.w[i].upos == UPos::VERB).unwrap();
                let subj = nps.iter().find(|(i, _)| s.w[*i].head == vi + 1 && matches!(s.w[*i].rel, Rel::Nsubj | Rel::NsubjPass)).map(|x| x.1.clone());
                let objn = nps.iter().find(|(i, _)| s.w[*i].upos == UPos::NOUN).map(|x| x.1.clone());
                let to = nps.iter().find(|(i, _)| s.case(*i) == "to").map(|x| x.1.clone());
                let (giver, recv) = if recv_verb { (None, subj.clone()) } else { (subj.clone(), to.clone()) };
                for (g, o, r) in self.gives.iter().rev() {
                    let ok = giver.as_ref().is_none_or(|x| x == g) && recv.as_ref().is_none_or(|x| x == r) && objn.as_ref().is_none_or(|x| x == o);
                    if !ok {
                        continue;
                    }
                    let ans = if first == "what" {
                        o.clone()
                    } else if recv_verb || giver.is_some() {
                        r.clone()
                    } else {
                        g.clone()
                    };
                    return Ok((ans, format!("{g} gave {o} {r}")));
                }
                return Ok(("?".into(), "no such transfer happened".into()));
            }
            let Some(p) = nps.first() else { return Ok(("?".into(), "about whom?".into())) };
            // deduction: «What is Gertrude afraid of?» — a class predicate
            if let Some(ai) = (0..s.w.len()).find(|&i| self.preds.iter().any(|(_, pr, _)| pr.starts_with(&format!("{}_", s.lem(i))))) {
                let pred_pref = format!("{}_", s.lem(ai));
                let cls = self.class_of(&p.1);
                for (x, pr, y) in self.preds.iter().rev() {
                    if pr.starts_with(&pred_pref) && (*x == p.1 || cls.as_deref() == Some(x.as_str())) {
                        return Ok((y.clone(), format!("{} — {:?}; {x} {pr} {y}", p.1, cls)));
                    }
                }
                return Ok(("?".into(), format!("no {pred_pref}… for {} ({cls:?})", p.1)));
            }
            // induction: «What color is Brian?» — the attribute of another member of the same class
            if let Some(c) = self.class_of(&p.1) {
                if let Some((_, a)) = self.attr.iter().rev().find(|(x, _)| *x == p.1) {
                    return Ok((a.clone(), format!("{} itself {a}", p.1)));
                }
                // the attribute of the most recent other class member
                let mut votes: Vec<(String, usize, usize)> = Vec::new();
                for (t, (x, a)) in self.attr.iter().enumerate() {
                    if *x != p.1 && self.isa.get(x) == Some(&c) {
                        match votes.iter_mut().find(|v| v.0 == *a) {
                            Some(v) => {
                                v.1 += 1;
                                v.2 = t;
                            }
                            None => votes.push((a.clone(), 1, t)),
                        }
                    }
                }
                // measured: majority 97.6%, most recent 99.5% (test) — the bAbI generator takes the more recent example
                if let Some(v) = votes.iter().max_by_key(|v| v.2) {
                    return Ok((v.0.clone(), format!("{} — {c}; other {c}: {votes:?}", p.1)));
                }
                return Ok(("?".into(), format!("no {c} without an attribute")));
            }
            return Ok(("?".into(), format!("don't know what is asked about {}", p.1)));
        }
        Ok(("?".into(), format!("question type «{first}» unknown")))
    }
}

/// Run of one bAbI file: (correct, total, first errors).
pub fn eval_file(path: &Path, show: usize) -> Result<(usize, usize, Vec<String>)> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let mut st = Story::default();
    let (mut ok, mut n) = (0, 0);
    let mut errs = Vec::new();
    let mut ctx: Vec<String> = Vec::new();
    for l in text.lines() {
        let (num, rest) = l.split_once(' ').unwrap_or(("0", l));
        if num == "1" {
            st = Story::default();
            ctx.clear();
        }
        if let Some((q, tail)) = rest.split_once('\t') {
            let gold = tail.split('\t').next().unwrap_or("").trim().to_lowercase();
            let (ans, why) = st.ask(q.trim())?;
            let norm = |x: &str| {
                let mut v: Vec<&str> = x.split(',').collect();
                v.sort();
                v.join(",")
            };
            n += 1;
            if norm(&ans.to_lowercase()) == norm(&gold) {
                ok += 1;
            } else if errs.len() < show {
                errs.push(format!("{}\n  ? {q} → {ans} (expected {gold}) — {why}", ctx.join(" | ")));
            }
        } else {
            if let Some(why) = st.read(rest.trim())? {
                if errs.len() < show {
                    errs.push(format!("did not read: {why}"));
                }
            }
            ctx.push(rest.trim().to_string());
            if ctx.len() > 12 {
                ctx.remove(0);
            }
        }
    }
    Ok((ok, n, errs))
}

/// All 20 tasks (test) in the `en/` folder.
/// Debugging: trees and answers for story lines (questions — with «?»).
pub fn probe(lines: &[String]) -> Result<()> {
    let mut st = Story::default();
    for l in lines {
        if l.contains('?') {
            println!("? {l} → {:?}", st.ask(l)?);
        } else {
            println!("{l} → {:?}", st.read(l)?);
        }
    }
    Ok(())
}

/// Level-1 training from the training split (qa20): state → where they go. Prints a ```relation block
/// for the seed; the decision what to write in is up to a human and the gate (we do not see the test).
pub fn learn_motivations(path: &Path) -> Result<()> {
    let text = std::fs::read_to_string(path)?;
    let mut st = Story::default();
    let mut cnt: BTreeMap<(String, String), usize> = BTreeMap::new();
    for l in text.lines() {
        let (num, rest) = l.split_once(' ').unwrap_or(("0", l));
        if num == "1" {
            st = Story::default();
        }
        if let Some((q, tail)) = rest.split_once('\t') {
            let gold = tail.split('\t').next().unwrap_or("").trim().to_lowercase();
            let qs = q.trim().to_lowercase();
            if let Some(who) = qs.strip_prefix("where will ").and_then(|x| x.strip_suffix(" go?")) {
                if let Some((_, a)) = st.attr.iter().rev().find(|(x, _)| x == who) {
                    *cnt.entry((a.clone(), gold)).or_default() += 1;
                }
            }
        } else {
            st.read(rest.trim())?;
        }
    }
    let mut best: BTreeMap<String, (usize, String, usize)> = BTreeMap::new();
    for ((a, g), n) in &cnt {
        let e = best.entry(a.clone()).or_default();
        e.2 += n;
        if *n > e.0 {
            e.0 = *n;
            e.1 = g.clone();
        }
    }
    for (a, (n, g, all)) in best {
        println!("{a}: go={g}    # {n}/{all}");
    }
    Ok(())
}

/// Contrast variants (Gardner et al. 2020): the same stories with new names, places, things and
/// synonym verbs — some synonyms are in level-1 classes, some (hurried, abandoned) are not:
/// this shows the snake takes the principle, not the template, and honestly shows the gap.
const CONTRAST: &[(&str, &str)] = &[
    ("Mary", "Olena"), ("John", "Taras"), ("Daniel", "Bohdan"), ("Sandra", "Iryna"), ("Fred", "Ostap"), ("Bill", "Marko"), ("Jeff", "Yurko"),
    ("Julie", "Solomiia"), ("Jason", "Yarema"), ("jason", "yarema"), ("Antoine", "Dmytro"), ("antoine", "dmytro"), ("Sumit", "Nazar"), ("sumit", "nazar"),
    ("Yann", "Lesia"), ("yann", "lesia"), ("Emily", "Roksolana"), ("emily", "roksolana"), ("Gertrude", "Zoriana"), ("gertrude", "zoriana"),
    ("Winona", "Halyna"), ("winona", "halyna"), ("Jessica", "Myroslava"), ("jessica", "myroslava"), ("Lily", "Vira"), ("Greg", "Petro"),
    ("Bernhard", "Hnat"), ("Brian", "Stepan"), ("Julius", "Kyrylo"),
    ("hallway", "corridor"), ("bathroom", "attic"), ("office", "library"), ("school", "stadium"), ("park", "market"), ("cinema", "theatre"),
    ("football", "ball"), ("apple", "pear"), ("milk", "juice"),
    ("went", "walked"), ("journeyed", "ran"), ("travelled", "drove"), ("moved", "hurried"),
    ("grabbed", "collected"), ("got", "received"), ("took", "gathered"), ("dropped", "threw"), ("discarded", "abandoned"),
    ("gave", "sent"), ("handed", "delivered"), ("passed", "offered"), ("give", "send"),
    ("box", "crate"), ("chest", "trunk"), ("suitcase", "bag"), ("container", "barrel"), ("chocolates", "candies"), ("chocolate", "candy"),
    ("triangle", "circle"), ("square", "hexagon"), ("rectangle", "oval"),
    ("mouse", "rabbit"), ("mice", "rabbits"), ("Mice", "Rabbits"), ("wolf", "fox"), ("wolves", "foxes"), ("Wolves", "Foxes"), ("cat", "owl"), ("cats", "owls"),
    ("Cats", "Owls"), ("swan", "duck"), ("lion", "tiger"), ("frog", "toad"), ("rhino", "hippo"),
];

/// Writes contrast files (all tasks; in qa20 places are not changed — the motivations are about the kitchen and garden).
pub fn contrast(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    let places = ["hallway", "bathroom", "office", "school", "park", "cinema"];
    for e in std::fs::read_dir(src)?.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        let text = std::fs::read_to_string(e.path())?;
        let qa20 = name.starts_with("qa20_");
        let mut out = String::with_capacity(text.len());
        let mut word = String::new();
        let flush = |w: &mut String, out: &mut String| {
            if !w.is_empty() {
                let rep = CONTRAST.iter().find(|(a, _)| *a == w.as_str() && !(qa20 && places.contains(a))).map(|x| x.1);
                out.push_str(rep.unwrap_or(w));
                w.clear();
            }
        };
        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                word.push(c);
            } else {
                flush(&mut word, &mut out);
                out.push(c);
            }
        }
        flush(&mut word, &mut out);
        std::fs::write(dst.join(&name), out)?;
    }
    Ok(())
}

pub fn eval_dir(dir: &Path, which: &str, show: usize) -> Result<()> {
    let mut files: Vec<_> = std::fs::read_dir(dir)?.flatten().map(|e| e.path()).filter(|p| p.to_string_lossy().ends_with(&format!("_{which}.txt"))).collect();
    files.sort_by_key(|p| p.file_name().unwrap().to_string_lossy().trim_start_matches("qa").split('_').next().unwrap().parse::<u32>().unwrap_or(0));
    let (mut tok, mut tn, mut solved) = (0, 0, 0);
    for f in &files {
        let (ok, n, errs) = eval_file(f, show)?;
        let name = f.file_name().unwrap().to_string_lossy().replace(&format!("_{which}.txt"), "");
        let acc = 100.0 * ok as f64 / n.max(1) as f64;
        println!("{name:<32} {ok:>4}/{n:<4} {acc:5.1}%");
        for e in errs {
            println!("    {e}");
        }
        tok += ok;
        tn += n;
        solved += usize::from(acc >= 95.0);
    }
    println!("total {tok}/{tn} ({:.1}%), solved (≥95%) {solved}/{}", 100.0 * tok as f64 / tn.max(1) as f64, files.len());
    Ok(())
}
