//! The third layer, UD tree ↔ frames.
//!
//! Up: `prag::frames::frames_of_tree`, the discourse carried through the volume. The frame's content is what this
//! crate compares and decodes from (`F`): kind, verb, negation, `tam`, the entities of the roles (head lemma,
//! pronoun, number, name, modifiers, coordinated phrases, and what the mention says: determiner, preposition,
//! possessor, prepositional modifiers, appositions, relative word), the frame's attributes, obliques, words,
//! role-less noun phrases, markers and conjunction, its links (what was said, cause, the frame above with the
//! clause's relation) and flags. The phrase as written (`Ent::text`) and the rule trace (`how`) are not content: a
//! decoder may not read them.
//!
//! Down: a realizer that says what the frames keep (README, «The realizer»): the verb group of the frame's `tam`
//! (`group_tam`) or of its kind (`group_old`), then the roles with the mentions' own function words, the clause's
//! words, markers and conjunction; clauses hang where their links say; punctuation from `en::lin::punctuate`. Each
//! step has a `PRAG_OFF=rt-…` switch back to the old defaults. The tree goes down through the tree decoder (`lin.rs`
//! and `morph.rs`).
//!
//! The residual is an edit script from the realized tree to the parsed one: words to insert, and the lemma, tag,
//! head or relation of a realized word to patch.

use std::collections::BTreeSet;

use en::gram::{Rel, Tag};
use prag::frames::{Ent, Frame, Kind, Link, Slot, Tam, Tense, VForm};

use crate::class::{self, Class, Layer, pos_bits, word_bits};
use crate::tally::Residual;
use crate::tree::TNode;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct E {
    pub id: String,
    pub pron: Option<String>,
    pub plural: bool,
    pub named: bool,
    pub attrs: Vec<String>,
    pub and: Vec<String>,
    pub poss: Option<Box<E>>,
    pub mods: Vec<(String, E)>,
    pub appos: Vec<E>,
    pub words: Vec<(Rel, String, Tag)>,
    pub conj: Vec<E>,
    pub det: Option<(String, Tag)>,
    pub case: Option<String>,
    pub cc: Option<String>,
    pub rel_word: Option<(String, Tag)>,
}

impl E {
    pub fn of(e: &Ent) -> E {
        E {
            id: e.id.clone(),
            pron: e.pron.clone(),
            plural: e.plural,
            named: e.named,
            attrs: e.attrs.clone(),
            and: e.and.clone(),
            poss: e.poss.as_ref().map(|p| Box::new(E::of(p))),
            mods: e.mods.iter().map(|(c, x)| (c.clone(), E::of(x))).collect(),
            appos: e.appos.iter().map(E::of).collect(),
            words: e.words.clone(),
            conj: e.conj.iter().map(E::of).collect(),
            det: e.det.clone(),
            case: e.case.clone(),
            cc: e.cc.clone(),
            rel_word: e.rel_word.clone(),
        }
    }
}

/// A frame's content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F {
    /// roles filled from elsewhere, not said in the clause
    pub unsaid: Vec<Slot>,
    /// the clause's other one-word dependents
    pub words: Vec<(Rel, String, Tag)>,
    /// noun phrases on the clause in no role, with their relation
    pub nps: Vec<(Rel, E)>,
    /// the attributes that are manner adverbs
    pub manner: Vec<String>,
    pub mark: Vec<(String, Tag)>,
    pub cc: Option<String>,
    /// the frame above (an index into the sentence's frames), the clause's relation, the noun it belongs to
    pub link: Option<(usize, Rel, Option<String>)>,
    pub kind: Kind,
    pub verb: String,
    pub neg: bool,
    pub agent: Option<E>,
    pub patient: Option<E>,
    pub recipient: Option<E>,
    pub place: Option<E>,
    pub attrs: Vec<String>,
    pub obls: Vec<(String, E)>,
    /// indices into the sentence's frames
    pub quote: Vec<usize>,
    pub quoted: bool,
    pub cause: Option<usize>,
    pub next: Option<usize>,
    pub cond: bool,
    /// tense, aspect, voice and modality (`read` false: the realizer falls back on the kind)
    pub tam: Tam,
    /// the event a noun names («the arrival of the Queen», prag's `By::Nominal`): the noun's mention says its words,
    /// so the realizer does not say it as a clause
    pub nominal: bool,
}

impl Default for F {
    fn default() -> F {
        F { unsaid: vec![], words: vec![], nps: vec![], manner: vec![], mark: vec![], cc: None, link: None, tam: Default::default(), kind: Kind::Bare, verb: String::new(), neg: false, agent: None, patient: None, recipient: None, place: None, attrs: vec![], obls: vec![], quote: vec![], quoted: false, cause: None, next: None, cond: false, nominal: false }
    }
}

impl F {
    pub fn of(f: &Frame) -> F {
        F {
            unsaid: f.unsaid.clone(),
            words: f.words.clone(),
            nps: f.nps.iter().map(|(r, e)| (*r, E::of(e))).collect(),
            manner: f.manner.clone(),
            mark: f.mark.clone(),
            cc: f.cc.clone(),
            link: f.link.clone(),
            kind: f.kind,
            verb: f.verb.clone(),
            neg: f.neg,
            agent: f.agent.as_ref().map(E::of),
            patient: f.patient.as_ref().map(E::of),
            recipient: f.recipient.as_ref().map(E::of),
            place: f.place.as_ref().map(E::of),
            attrs: f.attrs.clone(),
            obls: f.obls.iter().map(|(p, e)| (p.clone(), E::of(e))).collect(),
            quote: f.quote.clone(),
            quoted: f.quoted,
            cause: f.cause,
            next: f.next,
            cond: f.cond,
            tam: f.tam,
            nominal: f.how.by == prag::frames::By::Nominal,
        }
    }

    pub fn ents(&self) -> Vec<&E> {
        let mut v: Vec<&E> = [&self.agent, &self.patient, &self.recipient, &self.place].into_iter().flatten().collect();
        v.extend(self.obls.iter().map(|(_, e)| e));
        v
    }

    pub fn show(&self) -> String {
        let e = |x: &Option<E>| x.as_ref().map(|e| e.pron.clone().map(|p| format!("{}({p})", e.id)).unwrap_or(e.id.clone())).unwrap_or("_".into());
        let mut s = format!("{:?} {}{} {}", self.kind, e(&self.agent), if self.neg { " NOT" } else { "" }, self.verb);
        if self.patient.is_some() {
            s += &format!(" patient={}", e(&self.patient));
        }
        if self.recipient.is_some() {
            s += &format!(" to={}", e(&self.recipient));
        }
        if self.place.is_some() {
            s += &format!(" place={}", e(&self.place));
        }
        for (p, x) in &self.obls {
            s += &format!(" {p}={}", x.id);
        }
        if !self.attrs.is_empty() {
            s += &format!(" attrs={}", self.attrs.join(","));
        }
        s
    }
}

/// The realizer: frames of a volume's sentences → trees, carrying which entities were mentioned.
pub struct Realizer {
    seen: BTreeSet<String>,
    /// the realizer's steps switched off (`PRAG_OFF`, read once; tests set it directly)
    pub off: BTreeSet<String>,
}

impl Default for Realizer {
    fn default() -> Realizer {
        let off = std::env::var("PRAG_OFF").map(|v| v.split(',').filter(|x| x.starts_with("rt-")).map(str::to_string).collect()).unwrap_or_default();
        Realizer { seen: BTreeSet::new(), off }
    }
}

impl Realizer {
    /// A realizer with exactly these steps off (for tests: the environment is not read).
    #[cfg(test)]
    pub fn with_off(off: &[&str]) -> Realizer {
        Realizer { seen: BTreeSet::new(), off: off.iter().map(|x| x.to_string()).collect() }
    }
}

struct Build<'a> {
    fs: &'a [F],
    nodes: Vec<TNode>,
    done: BTreeSet<usize>,
    seen: &'a mut BTreeSet<String>,
    off: &'a BTreeSet<String>,
}

/// A clause's verb group as built: its head node, and which roles it has said already.
struct Group {
    h: usize,
    patient_done: bool,
    place_done: bool,
    attrs_left: Vec<String>,
    subject_done: bool,
    obls_done: bool,
}

fn add(nodes: &mut Vec<TNode>, lemma: &str, tag: Tag, head: usize, rel: Rel) -> usize {
    nodes.push(TNode { lemma: lemma.to_string(), tag, head, rel });
    nodes.len()
}

impl Build<'_> {
    /// The function words the frames keep (determiners, prepositions, conjunctions, markers) are said as kept;
    /// PRAG_OFF=rt-function says the old defaults («a» then «the», «to», «in», «and», «because», «if»).
    fn fun(&self) -> bool {
        !self.off.contains("rt-function")
    }

    /// Clauses hang where the frames' links put them (relation and noun); PRAG_OFF=rt-links joins them as before
    /// (conjuncts of the first, what was said as a ccomp, a cause as an advcl).
    fn links(&self) -> bool {
        !self.off.contains("rt-links")
    }

    fn np(&mut self, e: &E, head: usize, rel: Rel, prep: Option<&str>) -> usize {
        if let (Some((w, t)), true) = (&e.rel_word, self.links()) {
            // a relative word stands for the entity in its clause
            let h = add(&mut self.nodes, w, *t, head, rel);
            if let Some(p) = prep {
                for x in p.split(' ').filter(|x| !x.is_empty()) {
                    add(&mut self.nodes, x, Tag::IN, h, Rel::Case);
                }
            }
            return h;
        }
        let h = if let Some(p) = &e.pron {
            add(&mut self.nodes, &p.to_lowercase(), Tag::PRP, head, rel)
        } else {
            let tag = match (e.named, e.plural) {
                (true, false) => Tag::NNP,
                (true, true) => Tag::NNPS,
                (false, false) => Tag::NN,
                (false, true) => Tag::NNS,
            };
            let h = add(&mut self.nodes, &e.id, tag, head, rel);
            if let Some(p) = &e.poss {
                // a possessor instead of an article: a pronoun as its possessive form, a noun with «'s»
                let personal = match p.id.as_str() {
                    "i" => Some("my"),
                    "you" => Some("your"),
                    "we" => Some("our"),
                    _ => None,
                };
                if let Some(pr) = p.pron.as_ref().map(|x| x.to_lowercase()).or(personal.map(str::to_string)) {
                    add(&mut self.nodes, &pr, Tag::PRPS, h, Rel::NmodPoss);
                } else {
                    let ph = self.np(&E { poss: None, ..(**p).clone() }, h, Rel::NmodPoss, None);
                    add(&mut self.nodes, "'s", Tag::POS, ph, Rel::Case);
                }
            } else if self.fun() {
                self.seen.insert(e.id.to_lowercase());
                if let Some((l, t)) = &e.det {
                    add(&mut self.nodes, l, *t, h, Rel::Det);
                }
            } else if !e.named {
                let first = self.seen.insert(e.id.to_lowercase());
                if !first {
                    add(&mut self.nodes, "the", Tag::DT, h, Rel::Det);
                } else if !e.plural {
                    add(&mut self.nodes, "a", Tag::DT, h, Rel::Det);
                }
            }
            for a in &e.attrs {
                add(&mut self.nodes, a, Tag::JJ, h, Rel::Amod);
            }
            for (r, l, t) in &e.words {
                add(&mut self.nodes, l, *t, h, *r);
            }
            for (c, m) in &e.mods {
                self.np(m, h, Rel::Nmod, (!c.is_empty()).then_some(c.as_str()));
            }
            for a in &e.appos {
                self.np(a, h, Rel::Appos, None);
            }
            h
        };
        if let Some(p) = prep {
            for w in p.split(' ').filter(|w| !w.is_empty()) {
                add(&mut self.nodes, w, Tag::IN, h, Rel::Case);
            }
        }
        if e.conj.is_empty() || e.pron.is_some() {
            for x in &e.and {
                let tag = if x.chars().next().is_some_and(char::is_uppercase) { Tag::NNP } else { Tag::NN };
                let c = add(&mut self.nodes, x, tag, h, Rel::Conj);
                add(&mut self.nodes, "and", Tag::CC, c, Rel::Cc);
            }
        } else {
            // the coordinated phrases in full
            for x in &e.conj {
                let c = self.np(x, h, Rel::Conj, None);
                let cc = if self.fun() { x.cc.as_deref() } else { Some("and") };
                if let Some(cc) = cc {
                    add(&mut self.nodes, cc, Tag::CC, c, Rel::Cc);
                }
            }
        }
        h
    }

    fn clause(&mut self, i: usize, head: usize, rel: Rel, mark: Option<&str>) -> usize {
        self.clause_with(i, head, rel, mark, None)
    }

    /// A clause; `shared` is the subject of the clause it is coordinated with: the same agent, when it is no pronoun,
    /// is not said again («the dog barked and ran»; PRAG_OFF=rt-shared-subject says it again). A pronoun subject
    /// is said again: the frames do not say whether the text repeated it, and «he … and he …» is common.
    fn clause_with(&mut self, i: usize, head: usize, rel: Rel, mark: Option<&str>, shared: Option<&E>) -> usize {
        self.done.insert(i);
        let start = self.nodes.len();
        let f = self.fs[i].clone();
        let g = if f.kind == Kind::Bare { self.fragment(&f, head, rel) } else if f.tam.read { self.group_tam(&f, head, rel) } else { self.group_old(&f, head, rel) };
        let (h, patient_done, place_done, attrs_left) = (g.h, g.patient_done, g.place_done, g.attrs_left);
        let you = |a: &E| a.id == "you" && a.pron.is_none();
        if let (Some(a), false) = (&f.agent, g.subject_done) {
            let unsaid = f.unsaid.contains(&Slot::Agent) && !self.off.contains("rt-unsaid");
            let imperative = f.tam.read && f.tam.imperative && you(a);
            if !unsaid && !imperative && (shared != Some(a) || a.pron.is_some() || self.off.contains("rt-shared-subject")) {
                self.np(a, h, Rel::Nsubj, None);
            }
        }
        if let (Some(p), false) = (&f.patient, patient_done) {
            if !f.unsaid.contains(&Slot::Patient) || self.off.contains("rt-unsaid") {
                self.np(p, h, Rel::Obj, None);
            }
        }
        if let Some(r) = &f.recipient {
            // a recipient without a preposition is an indirect object («gave her a cake»)
            let (prep, rr) = if !self.fun() { (Some("to"), Rel::Obl) } else if r.case.is_some() { (r.case.as_deref(), Rel::Obl) } else { (None, Rel::Iobj) };
            self.np(r, h, rr, prep);
        }
        if let (Some(p), false) = (&f.place, place_done) {
            let prep = if self.fun() { p.case.as_deref() } else { Some("in") };
            self.np(p, h, Rel::Obl, prep);
        }
        if !g.obls_done {
            for (p, e) in &f.obls {
                self.np(e, h, Rel::Obl, Some(p));
            }
        }
        for a in &attrs_left {
            if let Some(w) = a.strip_prefix("when:").filter(|_| !self.off.contains("rt-when")) {
                // a time the reader kept as an attribute
                self.np(&E { id: w.to_string(), ..Default::default() }, h, Rel::OblTmod, None);
            } else if a.ends_with("ly") || f.manner.contains(a) {
                add(&mut self.nodes, a, Tag::RB, h, Rel::Advmod);
            } else {
                add(&mut self.nodes, a, Tag::JJ, h, Rel::Xcomp);
            }
        }
        for (r, l, t) in &f.words {
            // (a fragment's own word is its head, said by `fragment`)
            if !(f.kind == Kind::Bare && *r == Rel::Root) {
                add(&mut self.nodes, l, *t, h, *r);
            }
        }
        for (r, e) in &f.nps {
            // (with the mention's own preposition: «parsed by Mova», a phrase no role took)
            let prep = if self.fun() { e.case.as_deref() } else { None };
            self.np(e, h, *r, prep);
        }
        if self.fun() {
            for (l, t) in &f.mark {
                add(&mut self.nodes, l, *t, h, Rel::Mark);
            }
            if let Some(cc) = &f.cc {
                add(&mut self.nodes, cc, Tag::CC, h, Rel::Cc);
            }
        } else {
            if f.cond {
                add(&mut self.nodes, "if", Tag::IN, h, Rel::Mark);
            }
            if let Some(m) = mark {
                add(&mut self.nodes, m, Tag::IN, h, Rel::Mark);
            }
        }
        let linked = |b: &Self, j: usize| b.links() && b.fs[j].link.as_ref().is_some_and(|l| l.0 < b.fs.len() && l.0 != j);
        for &q in &f.quote {
            if q < self.fs.len() && !self.done.contains(&q) && !linked(self, q) {
                self.clause(q, h, Rel::Ccomp, None);
            }
        }
        if let Some(c) = f.cause {
            if c < self.fs.len() && !self.done.contains(&c) && !linked(self, c) {
                self.clause(c, h, Rel::Advcl, Some("because"));
            }
        }
        // the clauses that hang on this one, under the noun they belong to or under its predicate
        if self.links() {
            for j in 0..self.fs.len() {
                let Some((up, r, of)) = self.fs[j].link.clone() else { continue };
                if up != i || j == i || self.done.contains(&j) {
                    continue;
                }
                let at = of.and_then(|w| (start..self.nodes.len()).find(|&k| self.nodes[k].lemma.eq_ignore_ascii_case(&w)).map(|k| k + 1)).unwrap_or(h);
                let sh = (r.base() == Rel::Conj).then(|| f.agent.clone()).flatten();
                self.clause_with(j, at, r, None, sh.as_ref());
            }
        }
        h
    }

    /// The head of a frame without a predicate (`prag::frames::Kind::Bare`: an option, a heading, a noun phrase): its
    /// own word when it has one (the word with the relation root: an adjective, an adverb, an interjection), else
    /// its entity — the patient, the place or the first oblique — with the mention's own preposition. The other
    /// obliques are the head's, with their prepositions. The attributes and the negation of a bare frame sum up its
    /// words for the readers and are not said again.
    fn fragment(&mut self, f: &F, head: usize, rel: Rel) -> Group {
        let fun = self.fun();
        let case = |e: &E, p: Option<&str>| -> Option<String> { if fun { p.filter(|p| !p.is_empty()).map(str::to_string).or(e.case.clone()) } else { p.filter(|p| !p.is_empty()).map(str::to_string) } };
        let own = f.words.iter().find(|w| w.0 == Rel::Root);
        let mut obls: Vec<&(String, E)> = f.obls.iter().collect();
        let h = if let Some((_, l, t)) = own {
            add(&mut self.nodes, l, *t, head, rel)
        } else if let Some(p) = &f.patient {
            self.np(p, head, rel, case(p, None).as_deref())
        } else if let Some(p) = &f.place {
            let prep = if fun { p.case.clone() } else { Some("in".to_string()) };
            self.np(p, head, rel, prep.as_deref())
        } else if !obls.is_empty() {
            let (c, e) = obls.remove(0);
            self.np(e, head, rel, case(e, Some(c)).as_deref())
        } else if let Some(a) = &f.agent {
            self.np(a, head, rel, None)
        } else {
            add(&mut self.nodes, if f.verb.is_empty() { "thing" } else { &f.verb }, Tag::NN, head, rel)
        };
        for (c, e) in obls {
            self.np(e, h, Rel::Obl, case(e, Some(c)).as_deref());
        }
        Group { h, patient_done: true, place_done: true, attrs_left: Vec::new(), subject_done: true, obls_done: true }
    }

    /// The verb group of a frame without `tam`: past tense for events and speech, present for general statements,
    /// «do not» for a negation, «can» for Fact(can); a copula with an adjective, a noun or a place.
    fn group_old(&mut self, f: &F, head: usize, rel: Rel) -> Group {
        let past = matches!(f.kind, Kind::Event | Kind::Say(_));
        let plural = f.agent.as_ref().is_some_and(|a| a.plural || a.pron.as_deref().is_some_and(|p| matches!(p.to_lowercase().as_str(), "i" | "we" | "you" | "they")));
        let fin = if past { Tag::VBD } else if plural { Tag::VBP } else { Tag::VBZ };
        let mut words = f.verb.split(' ').filter(|w| !w.is_empty());
        let verb = words.next().unwrap_or("be").to_string();
        let prt: Vec<String> = words.map(str::to_string).collect();
        // (an attribute that repeats the verb's particle — «run away» with attrs=away — is said once)
        let mut attrs_left: Vec<String> = f.attrs.iter().filter(|a| self.off.contains("rt-prt-once") || !prt.contains(a)).cloned().collect();
        let copular = verb == "be" && (f.patient.is_some() || !attrs_left.is_empty() || f.place.is_some());
        let h;
        let mut place_done = false;
        let mut patient_done = false;
        if copular {
            h = if let Some(p) = &f.patient {
                patient_done = true;
                self.np(p, head, rel, None)
            } else if !attrs_left.is_empty() {
                let a = attrs_left.remove(0);
                add(&mut self.nodes, &a, Tag::JJ, head, rel)
            } else {
                place_done = true;
                let pl = f.place.clone().unwrap_or_default();
                let prep = if self.fun() { pl.case.clone() } else { Some("in".to_string()) };
                self.np(&pl, head, rel, prep.as_deref())
            };
            add(&mut self.nodes, "be", fin, h, Rel::Cop);
            if f.neg {
                add(&mut self.nodes, "not", Tag::RB, h, Rel::Advmod);
            }
        } else {
            let can = f.kind == Kind::Fact(Link::Can);
            let vt = if can || f.neg { Tag::VB } else { fin };
            h = add(&mut self.nodes, &verb, vt, head, rel);
            if can {
                add(&mut self.nodes, "can", Tag::MD, h, Rel::Aux);
            } else if f.neg {
                add(&mut self.nodes, "do", fin, h, Rel::Aux);
            }
            if f.neg {
                add(&mut self.nodes, "not", Tag::RB, h, Rel::Advmod);
            }
            for p in &prt {
                add(&mut self.nodes, p, Tag::RP, h, Rel::CompoundPrt);
            }
        }
        Group { h, patient_done, place_done, attrs_left, subject_done: false, obls_done: false }
    }

    /// The verb group of a frame with its own tense, aspect, voice and modality (`prag::frames::Tam`): the auxiliary
    /// chain modal or «will» or «do», «have» (perfect), «be» (progressive), «be» (passive), then the verb or the
    /// copula; the first word is finite by the tense (or the clause's form: «to», -ing, participle), each next word
    /// takes the form its auxiliary asks for. A passive says the patient as subject and the agent with «by» (its
    /// own preposition when the frames keep it); the caller says an active subject.
    fn group_tam(&mut self, f: &F, head: usize, rel: Rel) -> Group {
        let tam = f.tam;
        let subj = if tam.passive { f.patient.as_ref() } else { f.agent.as_ref() };
        let plural = subj.is_some_and(|a| a.plural || !a.and.is_empty() || a.pron.as_deref().is_some_and(|p| matches!(p.to_lowercase().as_str(), "we" | "you" | "they")) || a.id == "you");
        let mut words = f.verb.split(' ').filter(|w| !w.is_empty());
        let verb = words.next().unwrap_or("be").to_string();
        let prt: Vec<String> = words.map(str::to_string).collect();
        // (an attribute that repeats the verb's particle — «run away» with attrs=away — is said once)
        let mut attrs_left: Vec<String> = f.attrs.iter().filter(|a| self.off.contains("rt-prt-once") || !prt.contains(a)).cloned().collect();
        let copular = verb == "be" && (f.patient.is_some() && !tam.passive || !attrs_left.is_empty() || f.place.is_some());
        // the verb words in order: (lemma, what it asks of the next word: 0 base, 1 participle, 2 -ing)
        let mut chain: Vec<(String, u8)> = Vec::new();
        if let Some(m) = tam.modal {
            chain.push((m.lemma().to_string(), 0));
        } else if tam.tense == Tense::Fut {
            chain.push(("will".into(), 0));
        } else if tam.dosupport {
            chain.push(("do".into(), 0));
        }
        if tam.perfect {
            chain.push(("have".into(), 1));
        }
        if tam.progressive {
            chain.push(("be".into(), 2));
        }
        if tam.passive && !copular {
            chain.push(("be".into(), 1));
        }
        // the tag of each verb word: the first by the tense or form, the next by what the word before asks
        let fin = match (tam.tense, tam.form) {
            (Tense::Past, _) => Tag::VBD,
            (Tense::Pres | Tense::Fut, _) | (Tense::None, VForm::Fin) => {
                if plural || subj.is_some_and(|a| a.id == "i" || a.pron.as_deref().is_some_and(|p| p.eq_ignore_ascii_case("i"))) {
                    Tag::VBP
                } else {
                    Tag::VBZ
                }
            }
            (_, VForm::Inf | VForm::ToInf) => Tag::VB,
            (_, VForm::Ger) => Tag::VBG,
            (_, VForm::Part) => Tag::VBN,
        };
        let mut tags: Vec<Tag> = Vec::with_capacity(chain.len() + 1);
        for k in 0..=chain.len() {
            let t = if k == 0 {
                if chain.first().is_some_and(|(l, _)| l != "do" && l != "have" && l != "be") { Tag::MD } else { fin }
            } else {
                match chain[k - 1].1 {
                    1 => Tag::VBN,
                    2 => Tag::VBG,
                    _ => Tag::VB,
                }
            };
            tags.push(t);
        }
        let last = *tags.last().unwrap_or(&fin);
        let h;
        let mut place_done = false;
        let mut patient_done = false;
        if copular {
            h = if let (Some(p), false) = (&f.patient, tam.passive) {
                patient_done = true;
                self.np(p, head, rel, None)
            } else if !attrs_left.is_empty() {
                let a = attrs_left.remove(0);
                add(&mut self.nodes, &a, Tag::JJ, head, rel)
            } else {
                place_done = true;
                let pl = f.place.clone().unwrap_or_default();
                let prep = if self.fun() { pl.case.clone() } else { Some("in".to_string()) };
                self.np(&pl, head, rel, prep.as_deref())
            };
            add(&mut self.nodes, "be", last, h, Rel::Cop);
        } else {
            h = add(&mut self.nodes, &verb, last, head, rel);
            for p in &prt {
                add(&mut self.nodes, p, Tag::RP, h, Rel::CompoundPrt);
            }
        }
        for (k, (l, _)) in chain.iter().enumerate() {
            let r = if tam.passive && !copular && k + 1 == chain.len() { Rel::AuxPass } else { Rel::Aux };
            add(&mut self.nodes, l, tags[k], h, r);
        }
        if f.neg {
            add(&mut self.nodes, "not", Tag::RB, h, Rel::Advmod);
        }
        // «to» of an infinitive, unless the frame's own markers say it
        if tam.form == VForm::ToInf && !(self.fun() && f.mark.iter().any(|(m, _)| m == "to")) {
            add(&mut self.nodes, "to", Tag::TO, h, Rel::Mark);
        }
        let mut subject_done = false;
        if tam.passive {
            subject_done = true;
            if let (Some(p), false) = (&f.patient, patient_done) {
                if !f.unsaid.contains(&Slot::Patient) || self.off.contains("rt-unsaid") {
                    self.np(p, h, Rel::NsubjPass, None);
                }
                patient_done = true;
            }
            if let Some(a) = &f.agent {
                if !f.unsaid.contains(&Slot::Agent) || self.off.contains("rt-unsaid") {
                    let by = if self.fun() { a.case.clone() } else { Some("by".to_string()) };
                    self.np(a, h, Rel::OblAgent, by.as_deref());
                }
            }
        }
        Group { h, patient_done, place_done, attrs_left, subject_done, obls_done: false }
    }
}

impl Realizer {
    /// The tree of one sentence's frames.
    pub fn sentence(&mut self, fs: &[F]) -> Vec<TNode> {
        let mut b = Build { fs, nodes: Vec::new(), done: BTreeSet::new(), seen: &mut self.seen, off: &self.off };
        if fs.is_empty() {
            return b.nodes;
        }
        let inner: BTreeSet<usize> = fs.iter().flat_map(|f| f.quote.iter().copied().chain(f.cause)).collect();
        let mut root = 0;
        // the subject of the clause before: a conjunct with the same one does not say it again
        let mut last_agent: Option<E> = None;
        let linked = |i: usize| b.links() && fs[i].link.as_ref().is_some_and(|l| l.0 < fs.len() && l.0 != i);
        let linked: Vec<bool> = (0..fs.len()).map(linked).collect();
        // the events nouns name are said by their nouns (PRAG_OFF=rt-nominal says them as clauses too)
        if !b.off.contains("rt-nominal") && fs.iter().any(|f| !f.nominal) {
            b.done.extend((0..fs.len()).filter(|&i| fs[i].nominal));
        }
        for pass in 0..2 {
            for i in 0..fs.len() {
                if b.done.contains(&i) || pass == 0 && (inner.contains(&i) && i != 0 || linked[i]) {
                    continue;
                }
                if root == 0 {
                    root = b.clause(i, 0, Rel::Root, None);
                } else {
                    let c = b.clause_with(i, root, Rel::Conj, None, last_agent.as_ref());
                    if !b.fun() {
                        add(&mut b.nodes, "and", Tag::CC, c, Rel::Cc);
                    }
                }
                last_agent = fs[i].agent.clone();
            }
        }
        if b.off.contains("rt-punct") {
            add(&mut b.nodes, ".", Tag::Stop, root, Rel::Punct);
        } else {
            // punctuation from the clause structure (en::lin::punctuate): the frames never had the text's marks
            let tags: Vec<Tag> = b.nodes.iter().map(|x| x.tag).collect();
            let rels: Vec<Rel> = b.nodes.iter().map(|x| x.rel).collect();
            let heads: Vec<usize> = b.nodes.iter().map(|x| x.head).collect();
            for (h, m, t) in en::lin::punctuate(&tags, &rels, &heads) {
                add(&mut b.nodes, &m, t, h, Rel::Punct);
            }
        }
        b.nodes
    }
}

/// One step of the edit script for a word of the parsed tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    /// a realized word, with what to patch
    Take { from: usize, lemma: Option<String>, tag: Option<Tag>, rel: Option<Rel>, head: Option<usize> },
    Insert(TNode),
}

/// The edit script realized tree → parsed tree (one step per parsed word, in text order).
pub type R3 = Vec<Step>;

fn low(s: &str) -> String {
    s.to_lowercase()
}

/// Align the parsed words with the realized ones: same lemma and coarse part of speech; then same lemma; then the
/// same place in the tree (aligned heads, same relation) — a pronoun for a name, another word for this one.
fn align(t: &[TNode], r: &[TNode], pre: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut a: Vec<Option<usize>> = vec![None; t.len()];
    let mut used = vec![false; r.len()];
    // pairs given by the caller first (e.g. a sequence alignment of two texts in their order)
    for (i, p) in pre.iter().enumerate().take(t.len()) {
        if let Some(j) = *p {
            if j < r.len() && !used[j] {
                a[i] = Some(j);
                used[j] = true;
            }
        }
    }
    for pass in 0..2 {
        for i in 0..t.len() {
            if a[i].is_some() {
                continue;
            }
            let l = low(&t[i].lemma);
            if let Some(j) = (0..r.len()).find(|&j| !used[j] && low(&r[j].lemma) == l && (pass == 1 || class::coarse(r[j].tag) == class::coarse(t[i].tag))) {
                a[i] = Some(j);
                used[j] = true;
            }
        }
    }
    // structural: the heads are aligned with each other and the relation is the same
    for _ in 0..2 {
        for i in 0..t.len() {
            if a[i].is_some() || class::of_word(t[i].tag, t[i].rel, &t[i].lemma) == Class::Punct {
                continue;
            }
            let th = t[i].head;
            let rh = if th == 0 { Some(0) } else { a[th - 1].map(|j| j + 1) };
            let Some(rh) = rh else { continue };
            if let Some(j) = (0..r.len()).find(|&j| !used[j] && r[j].head == rh && r[j].rel.base() == t[i].rel.base()) {
                a[i] = Some(j);
                used[j] = true;
            }
        }
    }
    a
}

/// The residual of one sentence: the script, with every step counted in its class.
pub fn encode(t: &[TNode], r: &[TNode], forms: &[String], res: &mut Residual) -> R3 {
    encode_with(t, r, forms, &[], res)
}

/// `encode` with pairs (parsed word → realized word) the caller already knows; the alignment fills in the rest.
pub fn encode_with(t: &[TNode], r: &[TNode], forms: &[String], pre: &[Option<usize>], res: &mut Residual) -> R3 {
    let a = align(t, r, pre);
    let n = t.len();
    let mut inv = vec![None; r.len()];
    for (i, x) in a.iter().enumerate() {
        if let Some(j) = x {
            inv[*j] = Some(i + 1);
        }
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let w = &t[i];
        let f = forms.get(i).cloned().unwrap_or_default();
        match a[i] {
            None => {
                let c = class::of_word(w.tag, w.rel, &w.lemma);
                // a punctuation mark's kind names the mark: which marks the punctuation generator misses
                let kind = if c == Class::Punct { format!("missing «{f}»") } else { format!("missing {}", w.rel) };
                res.add_k(Layer::Frames, c, word_bits(c, &w.lemma) + 12.0 + pos_bits(n), kind, || format!("«{f}» {}/{} not in the frames", w.tag, w.rel));
                out.push(Step::Insert(w.clone()));
            }
            Some(j) => {
                let x = &r[j];
                let lemma = (x.lemma != w.lemma).then(|| w.lemma.clone());
                let mut word_class = None;
                if let Some(l) = &lemma {
                    let c = if low(l) == low(&x.lemma) { Class::Case } else { class::of_lemma_change(x.tag, w) };
                    if c != Class::Case {
                        word_class = Some(c);
                    }
                    let xl = x.lemma.clone();
                    res.add(Layer::Frames, c, word_bits(c, l), || format!("«{f}»: realized lemma «{xl}», parsed «{l}»"));
                }
                let tag = (x.tag != w.tag).then_some(w.tag);
                if let Some(tg) = tag {
                    // another word: its tag goes with it
                    let c = word_class.unwrap_or_else(|| class::of_tag_change(x.tag, tg));
                    let xt = x.tag;
                    res.add(Layer::Frames, c, 6.0, || format!("«{f}»: realized {xt}, parsed {tg}"));
                }
                let mapped = if x.head == 0 { Some(0) } else { inv[x.head - 1] };
                let head = (mapped != Some(w.head)).then_some(w.head);
                let rel = (x.rel != w.rel).then_some(w.rel);
                if head.is_some() || rel.is_some() {
                    let bits = if head.is_some() { pos_bits(n) } else { 0.0 } + if rel.is_some() { 6.0 } else { 0.0 };
                    let (xr, wr) = (x.rel, w.rel);
                    res.add_k(Layer::Frames, Class::Attachment, bits, format!("{xr}→{wr}"), || format!("«{f}»: realized {xr}, parsed {wr}{}", if head.is_some() { " (other head)" } else { "" }));
                }
                out.push(Step::Take { from: j, lemma, tag, rel, head });
            }
        }
    }
    // realized words the parsed tree does not have: the script drops them
    for (j, x) in r.iter().enumerate() {
        if inv[j].is_none() {
            let c = class::of_word(x.tag, x.rel, &x.lemma);
            let l = x.lemma.clone();
            res.add_k(Layer::Frames, c, pos_bits(r.len()), format!("realized {}", x.rel), || format!("realized «{l}» {}/{} has no word in the text", x.tag, x.rel));
        }
    }
    out
}

/// Apply the script to the realized tree.
pub fn apply(r: &[TNode], script: &R3) -> Vec<TNode> {
    let mut inv = vec![None; r.len()];
    for (i, s) in script.iter().enumerate() {
        if let Step::Take { from, .. } = s {
            if let Some(x) = inv.get_mut(*from) {
                *x = Some(i + 1);
            }
        }
    }
    script
        .iter()
        .map(|s| match s {
            Step::Insert(n) => n.clone(),
            Step::Take { from, lemma, tag, rel, head } => {
                // (a script for another realized tree — a damaged residual upstream — takes a word it does not have:
                // an empty word, and the gate is red)
                let Some(x) = r.get(*from) else { return TNode { lemma: String::new(), tag: Tag::NN, head: 0, rel: Rel::Dep } };
                let mapped = if x.head == 0 { 0 } else { inv[x.head - 1].unwrap_or(0) };
                TNode { lemma: lemma.clone().unwrap_or(x.lemma.clone()), tag: tag.unwrap_or(x.tag), head: head.unwrap_or(mapped), rel: rel.unwrap_or(x.rel) }
            }
        })
        .collect()
}

/// Frames of a sentence by `prag`, with their content.
pub fn read(words: &[en::annotate::Word], ctx: &mut prag::frames::Ctx) -> Vec<F> {
    let t = prag::frames::Tree::of_words(words.to_vec());
    if t.is_empty() {
        ctx.new_line();
        return Vec::new();
    }
    let (fs, _, _) = prag::frames::frames_of_tree(&t, ctx, false);
    ctx.new_line();
    fs.iter().map(F::of).collect()
}
