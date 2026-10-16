//! A single index of state facts for the v2 question reader — everything a question can rely on, in one list:
//! - events (agent, object, addressee, place, instrument, manner `how`, reason `why`, cause event `cause`,
//!   time `when`, topic `about`, negation);
//! - utterances (speaker, addressee, acts, `means`, `why`);
//! - intentions (kind, owners, text, to whom, state and fulfilment event);
//! - beliefs (owners, claim, topic, truthfulness);
//! - attributes of characters, items and places by state: traits, status, goal, feeling (added tags), age, sex,
//!   sleep, freedom, life, number, name, place — with the reason and the `unset … why=` explanation;
//! - items in hand and parts;
//! - names of characters and places (with `in=` ancestors), relations.
//!
//! Every fact has sentence anchors. Search is by lemmas of the `en` dictionary, with field weights and a preference for the
//! question's section (as in FairytaleQA, where the model sees the section).

use std::collections::HashSet;
use std::fmt::Write as _;

use crate::ans::{lemmas, name_of, toks};
use crate::lang::Role;
use crate::types::*;
use crate::world::{Delta, World};

/// Fact kind.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Event,
    Speech,
    Plan,
    Belief,
    /// entity attribute by state (the key is the field name: feeling, goal, trait, status, at…)
    Attr,
    /// item in hand (`items`)
    Have,
    /// part of a whole (`parts`)
    Part,
    /// character: name, class, type
    Char,
    /// place: name and ancestors
    Place,
    Rel,
}

/// Fact field.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Slot {
    /// event verb, intention kind, attribute key
    Pred,
    Agent,
    Patient,
    To,
    At,
    Instr,
    How,
    Why,
    Means,
    /// intention text or belief claim
    What,
    /// attribute value, name of an item or a part
    Value,
    When,
    About,
}

impl Slot {
    /// Weight of a match between a question word and the field (predicate and texts matter most).
    pub fn weight(self) -> f64 {
        match self {
            Slot::Pred => 1.5,
            Slot::Why | Slot::How | Slot::Means | Slot::What | Slot::Value => 1.0,
            Slot::Agent | Slot::Patient | Slot::To | Slot::At | Slot::Instr => 1.0,
            Slot::When | Slot::About => 0.5,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Slot::Pred => "pred",
            Slot::Agent => "agent",
            Slot::Patient => "patient",
            Slot::To => "to",
            Slot::At => "at",
            Slot::Instr => "instr",
            Slot::How => "how",
            Slot::Why => "why",
            Slot::Means => "means",
            Slot::What => "what",
            Slot::Value => "value",
            Slot::When => "when",
            Slot::About => "about",
        }
    }
}

/// Fact field: text (words of the tale or the LLM), entities, lemmas for search.
#[derive(Clone, Debug)]
pub struct FField {
    pub slot: Slot,
    pub text: String,
    pub ids: Vec<Id>,
    pub lem: HashSet<String>,
}

/// State fact with anchors.
#[derive(Clone, Debug)]
pub struct Fact {
    pub n: usize,
    pub kind: Kind,
    /// event, intention, belief, entity or relation
    pub src: Id,
    /// verb; attribute key; intention kind; believe; have; part; char; place; rel
    pub key: String,
    /// agents, owners, attribute's entity
    pub who: Vec<Id>,
    pub fields: Vec<FField>,
    pub anchors: Vec<u16>,
    pub sent: u16,
    /// LLM command line — story order
    pub line: usize,
    pub cause: Option<Id>,
    /// intention: fulfilment or breach event; belief: change event
    pub by: Option<Id>,
    pub neg: bool,
    /// attribute removed (`unset`), intention closed
    pub end: bool,
}

impl Fact {
    pub fn get(&self, s: Slot) -> Option<&FField> {
        self.fields.iter().find(|f| f.slot == s)
    }

    pub fn text(&self, s: Slot) -> Option<&str> {
        self.get(s).map(|f| f.text.as_str()).filter(|t| !t.is_empty())
    }

    pub fn ids(&self, s: Slot) -> Vec<Id> {
        self.fields.iter().filter(|f| f.slot == s).flat_map(|f| f.ids.iter().copied()).collect()
    }

    /// Entity in any field.
    pub fn has_id(&self, id: Id) -> bool {
        self.who.contains(&id) || self.fields.iter().any(|f| f.ids.contains(&id))
    }

    /// Human-readable label: `E12 give ^s23`, `C2 feeling=angry ^s5 ← E4`.
    pub fn label(&self) -> String {
        let mut s = match self.kind {
            Kind::Attr | Kind::Have | Kind::Part => format!("{} {}={}", self.src, self.key, self.text(Slot::Value).unwrap_or("?")),
            _ => format!("{} {}", self.src, self.key),
        };
        for a in &self.anchors {
            let _ = write!(s, " ^s{a}");
        }
        if let Some(c) = self.cause {
            let _ = write!(s, " ← {c}");
        }
        s
    }
}

/// Function words (not content) — for lemma search.
const STOP: &[&str] = &[
    "a", "an", "the", "of", "to", "in", "on", "at", "for", "with", "by", "from", "and", "or", "but", "is", "was", "were", "are", "be", "been",
    "being", "am", "did", "do", "does", "had", "has", "have", "will", "would", "could", "should", "can", "may", "might", "must", "shall", "it",
    "its", "he", "she", "they", "him", "her", "them", "his", "their", "this", "that", "these", "those", "what", "who", "whom", "whose", "why",
    "how", "where", "when", "which", "if", "so", "as", "not", "s", "t", "there", "then", "than", "i", "you", "we", "me", "my", "your", "our",
    "us", "into", "onto", "upon", "about", "very", "too", "also", "just", "only", "himself", "herself", "themselves", "itself", "no", "nor",
    "ever", "still", "yet", "some", "any", "all", "one", "someone", "something", "thing", "things",
];

pub fn stop(w: &str) -> bool {
    STOP.contains(&w)
}

/// Lemmas of all content words of a text (the word itself, dictionary analyses, suffix fallback).
pub fn lemset(text: &str) -> HashSet<String> {
    toks(text).iter().filter(|w| !stop(w)).flat_map(|w| lemmas(w)).collect()
}

/// Entity words: name, class, type.
pub fn ent_text(w: &World, id: Id) -> String {
    let Some(s) = w.now(id) else { return String::new() };
    let mut out = Vec::new();
    for f in [Field::Name, Field::Class, Field::Type] {
        match s.get(f) {
            Val::Known(V::Text(t)) => out.push(t),
            Val::Known(V::Words(ws)) => out.push(ws.join(" ")),
            Val::Known(V::Tag(t)) => out.push(t.as_str().replace('_', " ")),
            _ => {}
        }
    }
    out.join(" ").replace('_', " ")
}

/// Place in human form: name and up to two ancestors (`in=`).
pub fn place_str(w: &World, l: Id) -> String {
    let mut s = name_of(w, l);
    for p in w.ancestors(l).into_iter().take(2) {
        let _ = write!(s, " in {}", name_of(w, p));
    }
    s
}

fn val_text(w: &World, v: &V) -> String {
    match v {
        V::Id(i) if i.reg == Reg::L => place_str(w, *i),
        V::Id(i) => name_of(w, *i),
        V::Tag(t) => t.as_str().replace('_', " "),
        V::Tags(ts) => ts.iter().map(|t| t.as_str().replace('_', " ")).collect::<Vec<_>>().join(" and "),
        V::Words(ws) => ws.join(" ").replace('_', " "),
        V::Text(t) => t.clone(),
    }
}

/// Predicate words for attributes and other non-events (what to match the question verb against).
fn attr_pred(key: &str) -> &'static str {
    match key {
        "feeling" => "feel",
        "goal" => "want wish goal hope aim",
        "trait" | "status" | "age" | "gender" => "be",
        "at" => "be live stay dwell",
        "have" => "have own carry hold keep possess",
        "part" => "have",
        "name" => "name call",
        "num" => "many much number",
        "life" => "die live dead alive kill",
        "sleep" => "sleep asleep awake wake",
        "freedom" => "free captive release catch",
        "believe" => "believe think know suppose",
        "char" => "be",
        "place" => "be lie stand",
        _ => "",
    }
}

fn plan_pred(kind: &str) -> String {
    let extra = match kind {
        "promise" => "promise vow swear",
        "plan" => "plan intend decide mean",
        "agreement" | "bargain" => "agree agreement bargain promise",
        "wish" => "wish want hope desire",
        "threat" => "threaten threat",
        "order" => "order command tell bid",
        "task" => "task tell order",
        "curse" => "curse",
        "prophecy" => "prophesy foretell predict",
        "bet" => "bet wager",
        _ => "",
    };
    format!("{kind} {extra}")
}

/// Index of world facts.
#[derive(Clone, Debug, Default)]
pub struct Index {
    pub facts: Vec<Fact>,
}

struct B<'w> {
    w: &'w World,
    facts: Vec<Fact>,
}

impl B<'_> {
    fn field(&self, slot: Slot, text: String, ids: Vec<Id>) -> FField {
        let mut lem = lemset(&text);
        for id in &ids {
            lem.extend(lemset(&ent_text(self.w, *id)));
        }
        FField { slot, text, ids, lem }
    }

    fn ent_field(&self, slot: Slot, ids: Vec<Id>) -> Option<FField> {
        if ids.is_empty() {
            return None;
        }
        let text = ids.iter().map(|i| name_of(self.w, *i)).collect::<Vec<_>>().join(" and ");
        Some(self.field(slot, text, ids))
    }

    #[allow(clippy::too_many_arguments)]
    fn push(&mut self, kind: Kind, src: Id, key: &str, who: Vec<Id>, fields: Vec<FField>, anchors: Vec<u16>, line: usize, cause: Option<Id>) -> &mut Fact {
        let n = self.facts.len();
        let sent = anchors.first().copied().unwrap_or(1);
        self.facts.push(Fact { n, kind, src, key: key.to_string(), who, fields, anchors, sent, line, cause, by: None, neg: false, end: false });
        self.facts.last_mut().unwrap()
    }
}

impl Index {
    pub fn empty() -> Index {
        Index::default()
    }

    /// All facts of the world; order is the order of LLM commands (story order).
    pub fn build(w: &World) -> Index {
        let mut b = B { w, facts: Vec::new() };
        // events and utterances
        for e in w.events() {
            let roles = |r: Role| e.all(r).iter().map(|s| s.id).collect::<Vec<Id>>();
            let mut fs = Vec::new();
            let (kind, key, pred) = match &e.speech {
                Some(sp) => (Kind::Speech, "say".to_string(), format!("say tell speak {}", sp.acts().map(|a| a.name()).collect::<Vec<_>>().join(" "))),
                None => (Kind::Event, e.verb.clone(), e.verb.replace(['_', '-'], " ")),
            };
            fs.push(b.field(Slot::Pred, pred, Vec::new()));
            for (r, s) in [(Role::Agent, Slot::Agent), (Role::Patient, Slot::Patient), (Role::To, Slot::To), (Role::Instr, Slot::Instr)] {
                if let Some(f) = b.ent_field(s, roles(r)) {
                    fs.push(f);
                }
            }
            if let Some(l) = e.role(Role::At) {
                fs.push(b.field(Slot::At, place_str(w, l.id), vec![l.id]));
            }
            for (t, s) in [(&e.how, Slot::How), (&e.why, Slot::Why)] {
                if let Some(t) = t {
                    fs.push(b.field(s, t.clone(), Vec::new()));
                }
            }
            if let Some(m) = e.speech.as_ref().and_then(|s| s.means.clone()) {
                fs.push(b.field(Slot::Means, m, Vec::new()));
            }
            if let Some(t) = &e.x.when {
                fs.push(b.field(Slot::When, t.replace('_', " "), Vec::new()));
            }
            if let Some(a) = e.x.about {
                let text = if a.reg == Reg::E { w.event(a).map(|x| x.verb.replace('_', " ")).unwrap_or_default() } else { name_of(w, a) };
                fs.push(b.field(Slot::About, text, vec![a]));
            }
            let who = roles(Role::Agent);
            let f = b.push(kind, e.id, &key, who, fs, e.anchors.clone(), e.line, e.x.cause);
            f.neg = e.x.neg;
            f.sent = e.pos.sent;
        }
        // intentions
        for x in w.ents(Reg::P) {
            let st0 = &x.states()[0];
            let s = x.now();
            let kind = s.get(Field::Kind).to_string();
            let mut fs = vec![b.field(Slot::Pred, plan_pred(&kind), Vec::new())];
            if let Val::Known(V::Text(t)) = s.get(Field::What) {
                fs.push(b.field(Slot::What, t, Vec::new()));
            }
            if let Some(t) = s.id(Field::Target)
                && let Some(f) = b.ent_field(Slot::To, vec![t])
            {
                fs.push(f);
            }
            let stage = s.get(Field::Stage).to_string();
            fs.push(b.field(Slot::Value, stage.clone(), Vec::new()));
            let (src, by) = (s.id(Field::Src), s.id(Field::By));
            let f = b.push(Kind::Plan, x.id, &kind, s.ids(SetF::Owners), fs, vec![st0.pos.sent], st0.line, src);
            f.by = by;
            f.end = stage != "open";
        }
        // beliefs
        for x in w.ents(Reg::B) {
            let st0 = &x.states()[0];
            let s = x.now();
            let mut fs = vec![b.field(Slot::Pred, attr_pred("believe").to_string(), Vec::new())];
            if let Val::Known(V::Text(t)) = s.get(Field::What) {
                fs.push(b.field(Slot::What, t, Vec::new()));
            }
            if let Some(a) = s.id(Field::About) {
                let text = if a.reg == Reg::E { w.event(a).map(|x| x.verb.replace('_', " ")).unwrap_or_default() } else { name_of(w, a) };
                fs.push(b.field(Slot::About, text, vec![a]));
            }
            fs.push(b.field(Slot::Value, s.get(Field::Truth).to_string(), Vec::new()));
            let (src, by) = (s.id(Field::Src), s.id(Field::By));
            let dropped = s.get(Field::Stage).to_string() == "dropped";
            let f = b.push(Kind::Belief, x.id, "believe", s.ids(SetF::Owners), fs, vec![st0.pos.sent], st0.line, src);
            f.by = by;
            f.end = dropped;
        }
        // attributes, items, parts — by states of characters, items and places
        for reg in [Reg::C, Reg::O, Reg::L] {
            for x in w.ents(reg) {
                let mut prev_feel: Vec<Tag> = Vec::new();
                for st in x.states() {
                    for d in &st.delta {
                        match d {
                            Delta::Set(Field::Feeling, v) => {
                                let now = match v {
                                    Val::Known(v) => v.tag_list(),
                                    Val::Unknown => Vec::new(),
                                };
                                let added: Vec<Tag> = now.iter().copied().filter(|t| !prev_feel.contains(t)).collect();
                                if !added.is_empty() {
                                    let text = added.iter().map(|t| t.as_str().replace('_', " ")).collect::<Vec<_>>().join(" and ");
                                    let fs = vec![b.field(Slot::Pred, "feel".into(), Vec::new()), b.field(Slot::Value, text, Vec::new())];
                                    b.push(Kind::Attr, x.id, "feeling", vec![x.id], fs, vec![st.pos.sent], st.line, st.cause);
                                } else if now.is_empty() && !prev_feel.is_empty() {
                                    let text = st.note.clone().unwrap_or_default();
                                    let fs = vec![b.field(Slot::Pred, "feel".into(), Vec::new()), b.field(Slot::Value, text, Vec::new())];
                                    let f = b.push(Kind::Attr, x.id, "feeling", vec![x.id], fs, vec![st.pos.sent], st.line, st.cause);
                                    f.end = true;
                                }
                                prev_feel = now;
                            }
                            Delta::Set(fl, v) => {
                                let key = fl.name();
                                if matches!(fl, Field::Type | Field::Class | Field::Parent | Field::A | Field::B | Field::Mother | Field::Father | Field::PartOf) {
                                    continue;
                                }
                                let (text, ids, end) = match v {
                                    Val::Known(v) => (val_text(w, v), if let V::Id(i) = v { vec![*i] } else { Vec::new() }, false),
                                    Val::Unknown => (st.note.clone().unwrap_or_default(), Vec::new(), true),
                                };
                                let fs = vec![b.field(Slot::Pred, attr_pred(key).to_string(), Vec::new()), b.field(Slot::Value, text, ids)];
                                let f = b.push(Kind::Attr, x.id, key, vec![x.id], fs, vec![st.pos.sent], st.line, st.cause);
                                f.end = end;
                            }
                            Delta::Add(SetF::Items, o) => {
                                let fs = vec![b.field(Slot::Pred, attr_pred("have").to_string(), Vec::new()), b.field(Slot::Value, name_of(w, *o), vec![*o])];
                                b.push(Kind::Have, x.id, "have", vec![x.id], fs, vec![st.pos.sent], st.line, st.cause);
                            }
                            Delta::Add(SetF::Parts, o) => {
                                let fs = vec![b.field(Slot::Pred, attr_pred("part").to_string(), Vec::new()), b.field(Slot::Value, name_of(w, *o), vec![*o])];
                                b.push(Kind::Part, x.id, "part", vec![x.id], fs, vec![st.pos.sent], st.line, st.cause);
                            }
                            _ => {}
                        }
                    }
                }
                // name of a character or place (from the first state)
                let st0 = &x.states()[0];
                match reg {
                    Reg::C => {
                        let fs = vec![b.field(Slot::Pred, "be".into(), Vec::new()), b.field(Slot::Value, name_of(w, x.id), vec![x.id])];
                        b.push(Kind::Char, x.id, "char", vec![x.id], fs, vec![st0.pos.sent], st0.line, None);
                    }
                    Reg::L => {
                        let fs = vec![b.field(Slot::Pred, attr_pred("place").to_string(), Vec::new()), b.field(Slot::Value, place_str(w, x.id), vec![x.id])];
                        b.push(Kind::Place, x.id, "place", vec![x.id], fs, vec![st0.pos.sent], st0.line, None);
                    }
                    _ => {}
                }
            }
        }
        // relations
        for x in w.ents(Reg::R) {
            for st in x.states() {
                for d in &st.delta {
                    if let Delta::Set(Field::Kind, Val::Known(v)) = d {
                        let s = x.snap(st.ver);
                        let who: Vec<Id> = [s.id(Field::A), s.id(Field::B)].into_iter().flatten().collect();
                        let fs = vec![b.field(Slot::Pred, "be".into(), Vec::new()), b.field(Slot::Value, val_text(w, v), Vec::new())];
                        b.push(Kind::Rel, x.id, "rel", who, fs, vec![st.pos.sent], st.line, st.cause);
                    }
                }
            }
        }
        let mut facts = b.facts;
        facts.sort_by_key(|f| (f.line, f.n));
        for (i, f) in facts.iter_mut().enumerate() {
            f.n = i;
        }
        Index { facts }
    }

    /// Fact of event `e` (event or utterance).
    pub fn event(&self, e: Id) -> Option<&Fact> {
        self.facts.iter().find(|f| f.src == e && matches!(f.kind, Kind::Event | Kind::Speech))
    }
}

/// Query to the index.
#[derive(Clone, Debug, Default)]
pub struct Query {
    /// predicate lemmas (with xcomp)
    pub preds: Vec<Vec<String>>,
    /// question subject and the role in which to expect it
    pub who: Vec<Id>,
    pub role: Option<Slot>,
    /// other participants (object, adverbials)
    pub others: Vec<Id>,
    /// the rest of the content words (lemmas of each word)
    pub words: Vec<Vec<String>>,
    pub secs: Vec<u16>,
    /// prediction ("what will …"): the answer is in later sections, so the next section weighs almost as much as its own
    pub fut: bool,
}

/// Fact score and its components (for a transparent log).
#[derive(Clone, Debug)]
pub struct Scored {
    pub n: usize,
    pub score: f64,
    pub pred: bool,
    pub who: bool,
    pub words: f64,
    pub insec: bool,
    pub why: String,
}

impl Index {
    /// Fact score for a query: predicate +3; subject in role +3, elsewhere +1, absent −2; other participant +1.5;
    /// content word — field weight (1.5 predicate; 1 texts and participants; 0.5 time and "about"); question section
    /// +3, adjacent +0.5, further −1.5.
    pub fn score(&self, w: &World, f: &Fact, q: &Query) -> Scored {
        let mut s = 0.0;
        let mut why = String::new();
        let pl = f.get(Slot::Pred).map(|x| &x.lem);
        let pred = !q.preds.is_empty() && pl.is_some_and(|pl| q.preds.iter().any(|alts| alts.iter().any(|a| pl.contains(a))));
        if pred {
            s += 3.0;
            why.push_str("predicate+3 ");
        }
        let mut who_hit = false;
        if !q.who.is_empty() {
            let in_role = match q.role {
                Some(Slot::Agent) => f.who.iter().any(|x| q.who.contains(x)) && matches!(f.kind, Kind::Event | Kind::Speech | Kind::Plan | Kind::Belief | Kind::Attr | Kind::Have | Kind::Part | Kind::Char | Kind::Place | Kind::Rel),
                Some(r) => f.ids(r).iter().any(|x| q.who.contains(x)),
                None => f.who.iter().any(|x| q.who.contains(x)),
            };
            let any = q.who.iter().any(|x| f.has_id(*x));
            if in_role {
                s += 3.0;
                who_hit = true;
                why.push_str("subject+3 ");
            } else if any {
                s += 1.0;
                who_hit = true;
                why.push_str("subject(other role)+1 ");
            } else {
                s -= 2.0;
                why.push_str("no subject−2 ");
            }
        }
        for o in &q.others {
            if f.has_id(*o) {
                s += 1.5;
                let _ = write!(why, "{}+1.5 ", o);
            }
        }
        let mut ws = 0.0;
        for alts in &q.words {
            let mut best = 0.0f64;
            for fl in &f.fields {
                if alts.iter().any(|a| fl.lem.contains(a)) {
                    best = best.max(fl.slot.weight());
                }
            }
            ws += best;
        }
        if ws > 0.0 {
            s += ws;
            let _ = write!(why, "words+{ws:.1} ");
        }
        let sec = w.text.sents.get(f.sent as usize - 1).map(|x| x.0).unwrap_or(0);
        let insec = q.secs.contains(&sec);
        if q.secs.is_empty() {
        } else if insec {
            s += 3.0;
            why.push_str("section+3");
        } else if q.fut && q.secs.iter().any(|x| sec > *x && sec - *x <= 1) {
            s += 2.5;
            why.push_str("next section (prediction)+2.5");
        } else if q.secs.iter().any(|x| x.abs_diff(sec) == 1) {
            s += 0.5;
            why.push_str("adjacent section+0.5");
        } else {
            s -= 1.5;
            why.push_str("outside sections−1.5");
        }
        Scored { n: f.n, score: s, pred, who: who_hit, words: ws, insec, why }
    }

    /// Best facts for a query among those that passed the filter; ties — earlier in the story wins.
    pub fn search(&self, w: &World, q: &Query, keep: impl Fn(&Fact) -> bool) -> Vec<Scored> {
        let mut out: Vec<Scored> = self.facts.iter().filter(|f| keep(f)).map(|f| self.score(w, f, q)).collect();
        out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap().then(a.n.cmp(&b.n)));
        out
    }
}
