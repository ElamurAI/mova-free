//! State of the story world (, "Immutable states —
//! the alternative variant"). Each entity is an identity (like a branch in git) with a list of immutable states (like commits):
//! state `C1@n` internally is only a delta from `C1@n-1`, a cause and an anchor (shared memory, no full
//! copies), externally — a full snapshot (`Entity::snap`). Registries: locations, objects, characters, relations
//! (a separate class, also with states), events. The cursor — where and when the narration is. Mentions — append-only.
//!
//! Executor: a command is applied to a copy of the world; gates before and after (references, two-way
//! links, teleportation, nearness, silent change, continuity); a violation means refusal, the world does not
//! change.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use prag::schema::Act;

use crate::lang::{Cmd, Ev2, Line, Role, Speaker};
use crate::types::*;

/// Truth of a belief according to the story world (v2).
pub static TRUTH: Closed = Closed { what: "true", all: &["false", "true"] };

/// Change of one field in a state.
#[derive(Clone, Debug, PartialEq)]
pub enum Delta {
    Set(Field, Val),
    Add(SetF, Id),
    Del(SetF, Id),
}

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Delta::Set(k, v) => write!(f, "{}={v}", k.name()),
            Delta::Add(s, i) => write!(f, "{} +{i}", s.name()),
            Delta::Del(s, i) => write!(f, "{} −{i}", s.name()),
        }
    }
}

/// Immutable state: delta from the previous one, cause, which command created it and where in the text.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub ver: u32,
    pub delta: Vec<Delta>,
    pub cause: Option<Id>,
    /// the change-language command that created the state
    pub by: &'static str,
    /// line of the LLM output
    pub line: usize,
    pub pos: TextPos,
    /// explanation of the change (v2: `unset … why=`)
    pub note: Option<String>,
}

/// Full snapshot of a state (external view): fields and sets. An unknown field is absent.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snap {
    f: BTreeMap<Field, V>,
    s: BTreeMap<SetF, BTreeSet<Id>>,
}

impl Snap {
    pub fn get(&self, f: Field) -> Val {
        self.f.get(&f).cloned().map(Val::Known).unwrap_or_default()
    }

    pub fn id(&self, f: Field) -> Option<Id> {
        match self.f.get(&f) {
            Some(V::Id(i)) => Some(*i),
            _ => None,
        }
    }

    pub fn ids(&self, s: SetF) -> Vec<Id> {
        self.s.get(&s).map(|x| x.iter().copied().collect()).unwrap_or_default()
    }

    pub fn has(&self, s: SetF, id: Id) -> bool {
        self.s.get(&s).is_some_and(|x| x.contains(&id))
    }

    pub fn fields(&self) -> impl Iterator<Item = (Field, &V)> {
        self.f.iter().map(|(k, v)| (*k, v))
    }

    pub fn sets(&self) -> impl Iterator<Item = (SetF, &BTreeSet<Id>)> {
        self.s.iter().map(|(k, v)| (*k, v))
    }

    fn apply(&mut self, d: &Delta) {
        match d {
            Delta::Set(k, Val::Known(v)) => {
                self.f.insert(*k, v.clone());
            }
            Delta::Set(k, Val::Unknown) => {
                self.f.remove(k);
            }
            Delta::Add(s, i) => {
                self.s.entry(*s).or_default().insert(*i);
            }
            Delta::Del(s, i) => {
                if let Some(x) = self.s.get_mut(s) {
                    x.remove(i);
                    if x.is_empty() {
                        self.s.remove(s);
                    }
                }
            }
        }
    }
}

/// Identity: immutable states (append-only) and mentions (append-only).
#[derive(Clone, Debug)]
pub struct Entity {
    pub id: Id,
    pub(crate) states: Vec<State>,
    pub(crate) mentions: Vec<TextPos>,
}

impl Entity {
    pub fn states(&self) -> &[State] {
        &self.states
    }

    pub fn latest(&self) -> u32 {
        self.states.len() as u32
    }

    pub fn state(&self, ver: u32) -> Option<&State> {
        self.states.get((ver as usize).checked_sub(1)?)
    }

    /// Full snapshot `id@ver`: walking the chain of deltas from the first state.
    pub fn snap(&self, ver: u32) -> Snap {
        let mut s = Snap::default();
        for st in self.states.iter().take(ver as usize) {
            for d in &st.delta {
                s.apply(d);
            }
        }
        s
    }

    pub fn now(&self) -> Snap {
        self.snap(self.latest())
    }

    /// The last state in effect at the end of sentence `sent` (entity not yet created — `None`).
    pub fn ver_at(&self, sent: u16) -> Option<u32> {
        self.states.iter().rev().find(|s| s.pos.sent <= sent).map(|s| s.ver)
    }

    /// The state in which the field last changed up to and including state `ver`.
    pub fn set_by(&self, f: Field, ver: u32) -> Option<&State> {
        self.states.iter().take(ver as usize).rev().find(|s| s.delta.iter().any(|d| matches!(d, Delta::Set(k, _) if *k == f)))
    }

    pub fn mentions(&self) -> &[TextPos] {
        &self.mentions
    }

    pub fn sref(&self) -> SRef {
        SRef { id: self.id, ver: self.latest() }
    }
}

/// Speech act of a `say` event.
#[derive(Clone, Debug)]
pub struct Speech {
    pub narrator: bool,
    /// main act
    pub act: Act,
    /// further acts of the same utterance (v2: `act=request+promise`)
    pub also: Vec<Act>,
    pub indirect: bool,
    pub sincere: Option<bool>,
    pub means: Option<String>,
}

impl Speech {
    pub fn acts(&self) -> impl Iterator<Item = Act> + '_ {
        std::iter::once(self.act).chain(self.also.iter().copied())
    }

    /// `request+promise`
    pub fn acts_str(&self) -> String {
        self.acts().map(|a| a.name()).collect::<Vec<_>>().join("+")
    }
}

/// Event: participants are references to specific states (`C1@3`); one role may have several (v2).
#[derive(Clone, Debug)]
pub struct Event {
    pub id: Id,
    pub verb: String,
    pub roles: Vec<(Role, SRef)>,
    pub why: Option<String>,
    pub how: Option<String>,
    pub speech: Option<Speech>,
    /// v2: negation, cause event, what it is about, story time, flashback
    pub x: Ev2,
    pub anchors: Vec<u16>,
    pub pos: TextPos,
    pub line: usize,
    /// v3: the world (time axis) in which the event happened; 0 — the document world
    pub world: u32,
    /// v3: reality branch (`real` — really happened)
    pub branch: Tag,
}

impl Event {
    /// The event really happened (not `neg=1` and branch `real`).
    pub fn happened(&self) -> bool {
        !self.x.neg && self.branch.as_str() == "real"
    }
}

impl Event {
    pub fn role(&self, r: Role) -> Option<SRef> {
        self.roles.iter().find(|(x, _)| *x == r).map(|(_, s)| *s)
    }

    /// All participants of a role (joint agents).
    pub fn all(&self, r: Role) -> Vec<SRef> {
        self.roles.iter().filter(|(x, _)| *x == r).map(|(_, s)| *s).collect()
    }

    /// A participant in the role — any of the named ones.
    pub fn has(&self, r: Role, ids: &[Id]) -> bool {
        self.roles.iter().any(|(x, s)| *x == r && ids.contains(&s.id))
    }

    pub fn anchors(&self) -> String {
        self.anchors.iter().map(|a| format!("^s{a}")).collect::<Vec<_>>().join(" ")
    }
}

/// State of the main cursor: where in the text, where (location) and when (story time) the narration is.
#[derive(Clone, Debug, PartialEq)]
pub struct Cursor {
    pub pos: TextPos,
    pub at: Option<Id>,
    pub time: Option<String>,
    pub line: usize,
}

/// Document text: title and sentences (paragraph, text), sentence number is index + 1.
#[derive(Clone, Debug)]
pub struct Text {
    pub doc: i64,
    pub title: String,
    pub sents: Vec<(u16, String)>,
}

/// The gate that rejected a command.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gate {
    Anchor,
    Ref,
    Link,
    Teleport,
    Near,
    Silent,
    Continuity,
    Kind,
    // v3
    /// narrative frames: level, nesting, closing the wrong frame
    Frame,
    /// story time axis: a cycle in the partial order, absolute time not from the text, effect before cause
    Time,
    /// reality branch: the canonical world is changed only by what really happened
    Branch,
    /// world boundary: an event or cause from another world (story within a story)
    Scope,
    /// event identity: the same event twice (a second death), a reveal does not create events
    Identity,
}

impl Gate {
    pub fn name(self) -> &'static str {
        match self {
            Gate::Anchor => "anchor",
            Gate::Ref => "reference",
            Gate::Link => "two-way link",
            Gate::Teleport => "teleportation",
            Gate::Near => "nearness",
            Gate::Silent => "silent change",
            Gate::Continuity => "continuity",
            Gate::Kind => "kind",
            Gate::Frame => "frame",
            Gate::Time => "story time",
            Gate::Branch => "branch",
            Gate::Scope => "world boundary",
            Gate::Identity => "event identity",
        }
    }
}

#[derive(Clone, Debug)]
pub struct GateErr {
    pub gate: Gate,
    pub msg: String,
}

impl fmt::Display for GateErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.gate.name(), self.msg)
    }
}

pub(crate) fn gate<T>(g: Gate, msg: impl Into<String>) -> Result<T, GateErr> {
    Err(GateErr { gate: g, msg: msg.into() })
}

/// Records of one command: deltas per entity (each will get one new state).
#[derive(Default)]
struct Tx {
    d: BTreeMap<Id, Vec<Delta>>,
}

impl Tx {
    fn put(&mut self, id: Id, d: Delta) {
        self.d.entry(id).or_default().push(d);
    }
}

/// The world.
#[derive(Clone, Debug)]
pub struct World {
    pub text: Text,
    pub(crate) ents: BTreeMap<Id, Entity>,
    pub(crate) events: Vec<Event>,
    pub(crate) cursor: Vec<Cursor>,
    pub(crate) pos: TextPos,
    /// v3: gates in story time (teleportation on the story axis, dead agent, second death); v2 — as before
    pub v3: bool,
    /// v3: world identity (time axis): 0 — the document world; a story within a story gets a new one
    pub wid: u32,
    /// v3: world reality — `real` for the canonical one, a branch for a dream or lie world
    pub branch: Tag,
    /// v3: narrative layer (root only): frames, story axis, knowledge, reveals
    pub story: Option<Box<crate::story::Story>>,
}

impl World {
    /// World v1/v2: linear narration (v3 commands work too, gates as in v2).
    pub fn new(text: Text) -> World {
        World {
            text,
            ents: BTreeMap::new(),
            events: Vec::new(),
            cursor: Vec::new(),
            pos: TextPos { chapter: 1, para: 0, sent: 0 },
            v3: false,
            wid: 0,
            branch: BRANCH.tag("real").expect("real"),
            story: Some(Box::default()),
        }
    }

    /// World v3: gates in story time.
    pub fn new_v3(text: Text) -> World {
        World { v3: true, ..World::new(text) }
    }

    pub fn story(&self) -> Option<&crate::story::Story> {
        self.story.as_deref()
    }

    pub fn ent(&self, id: Id) -> Option<&Entity> {
        self.ents.get(&id)
    }

    pub fn ents(&self, reg: Reg) -> impl Iterator<Item = &Entity> {
        self.ents.values().filter(move |e| e.id.reg == reg)
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn event(&self, id: Id) -> Option<&Event> {
        self.events.iter().find(|e| e.id == id)
    }

    pub fn cursor(&self) -> &[Cursor] {
        &self.cursor
    }

    /// Snapshot of an entity at its last state.
    pub fn now(&self, id: Id) -> Option<Snap> {
        self.ents.get(&id).map(Entity::now)
    }

    /// Apply a command with gates. Refusal leaves the world unchanged.
    ///
    /// v3 pipeline: sentence cursor → narrative frame → event mention or claim →
    /// resolution into a canonical event or a new one → story time axis → reality branch → observer
    /// knowledge → gates decide whether the canonical world changes. Without frames and without explicit time this
    /// is the linear special case — exactly v2.
    pub fn apply(&mut self, l: &Line) -> Result<(), GateErr> {
        let pos = self.position(l)?;
        let mut w = self.clone();
        if w.story.is_some() {
            w.apply_root(l, pos)?;
        } else {
            w.apply_local(l, pos)?;
        }
        w.pos = pos;
        *self = w;
        Ok(())
    }

    /// Execute a world command (v2) on this world, without a copy: gates, links, mentions. Returns the new events.
    pub(crate) fn apply_local(&mut self, l: &Line, pos: TextPos) -> Result<Vec<Id>, GateErr> {
        let n0 = self.events.len();
        let touched = self.exec(l, pos)?;
        self.check_links()?;
        // mentions — on every anchor of the line (speech two sentences later mentions in both), append-only at the end
        let mut at: Vec<TextPos> =
            l.anchors.iter().map(|a| TextPos { chapter: pos.chapter, para: self.text.sents[*a as usize - 1].0, sent: *a }).collect();
        at.sort();
        for id in touched {
            if let Some(e) = self.ents.get_mut(&id) {
                for p in &at {
                    if e.mentions.last().is_none_or(|m| m < p) {
                        e.mentions.push(*p);
                    }
                }
            }
        }
        self.pos = pos;
        Ok(self.events[n0..].iter().map(|e| e.id).collect())
    }

    /// Position of a command in the text from its anchors (last anchor); anchors do not go backwards; the cursor is checked against the text.
    pub(crate) fn position(&self, l: &Line) -> Result<TextPos, GateErr> {
        let n = self.text.sents.len();
        for &a in &l.anchors {
            if a == 0 || a as usize > n {
                return gate(Gate::Anchor, format!("^s{a} outside the text (^s1–^s{n})"));
            }
        }
        let Some(&s) = l.anchors.iter().max() else { return gate(Gate::Anchor, "no anchor") };
        if s < self.pos.sent {
            return gate(Gate::Anchor, format!("^s{s} after ^s{} — anchors do not go backwards", self.pos.sent));
        }
        let para = self.text.sents[s as usize - 1].0;
        let mut chapter = self.pos.chapter;
        if let Cmd::Cursor { chapter: c, para: p, sent: q, .. } = &l.cmd {
            if let Some(q) = q
                && *q != s
            {
                return gate(Gate::Anchor, format!("cursor sent={q}, but anchor ^s{s}"));
            }
            if let Some(p) = p
                && *p != para
            {
                return gate(Gate::Anchor, format!("cursor para={p}, but ^s{s} is in paragraph {para}"));
            }
            if let Some(c) = c {
                if *c < chapter {
                    return gate(Gate::Anchor, format!("cursor chapter={c} after chapter {chapter}"));
                }
                chapter = *c;
            }
        }
        Ok(TextPos { chapter, para, sent: s })
    }

    /// The entity exists and is from the required registry.
    pub(crate) fn need(&self, id: Id, regs: &[Reg]) -> Result<&Entity, GateErr> {
        if !regs.contains(&id.reg) {
            return gate(Gate::Kind, format!("{id}: only {} allowed here", regs.iter().map(|r| r.letter().to_string()).collect::<Vec<_>>().join("/")));
        }
        match self.ents.get(&id) {
            Some(e) => Ok(e),
            None => gate(Gate::Ref, format!("{id} not in registry {}", id.reg.uk())),
        }
    }

    pub(crate) fn need_event(&self, id: Id) -> Result<&Event, GateErr> {
        match self.event(id) {
            Some(e) => Ok(e),
            None => gate(Gate::Ref, format!("cause {id} not in the event registry (write the event before referencing it)")),
        }
    }

    /// A cause that really happened: it is in the registry and not `neg=1` (v2: things do not move and goals are not
    /// achieved through something that did not happen).
    fn real_cause(&self, c: Option<Id>, what: &str) -> Result<(), GateErr> {
        if let Some(c) = c
            && self.need_event(c)?.x.neg
        {
            return gate(Gate::Continuity, format!("{what}: cause {c} is an event that did not happen (neg=1)"));
        }
        Ok(())
    }

    /// A state change that requires a cause (v2): no `cause=`/`by=` — silent change.
    fn need_cause(&self, c: Option<Id>, what: &str) -> Result<Id, GateErr> {
        match c {
            Some(c) => {
                self.need_event(c)?;
                Ok(c)
            }
            None => gate(Gate::Silent, format!("{what} without a cause (cause=E / by=E)")),
        }
    }

    /// Anything from the registries or events (`about=`, `mention`).
    fn need_any(&self, id: Id) -> Result<(), GateErr> {
        if id.reg == Reg::E {
            self.need_event(id).map(|_| ())
        } else {
            self.need(id, &[id.reg]).map(|_| ())
        }
    }

    /// Part of a whole: a thing is not moved separately until it is detached (v2).
    fn not_part(&self, o: Id) -> Result<(), GateErr> {
        if let Some(w) = self.now(o).and_then(|s| s.id(Field::PartOf)) {
            return gate(Gate::Link, format!("{o} is part of {w}: detach first"));
        }
        Ok(())
    }

    /// Next free registry numbers (for continuing the LLM in parts).
    pub fn next_free(&self) -> Vec<(Reg, u32)> {
        [Reg::L, Reg::O, Reg::C, Reg::R, Reg::E, Reg::P, Reg::B]
            .into_iter()
            .map(|r| {
                let m = if r == Reg::E { self.events.iter().map(|e| e.id.n).max() } else { self.ents(r).map(|e| e.id.n).max() };
                (r, m.unwrap_or(0) + 1)
            })
            .collect()
    }

    fn fresh(&self, id: Id) -> Result<(), GateErr> {
        if self.ents.contains_key(&id) || self.event(id).is_some() {
            return gate(Gate::Ref, format!("{id} already exists — ids are created once"));
        }
        Ok(())
    }

    /// Where the entity is now: location (for a character — its `at`, for an object in hand — the owner's place,
    /// for a part — the place of the whole).
    pub fn place(&self, id: Id) -> Option<Id> {
        self.place_n(id, 0)
    }

    fn place_n(&self, id: Id, depth: usize) -> Option<Id> {
        if depth > 16 {
            return None;
        }
        match id.reg {
            Reg::L => Some(id),
            Reg::C | Reg::O => {
                let s = self.now(id)?;
                if let Some(w) = s.id(Field::PartOf) {
                    return self.place_n(w, depth + 1);
                }
                let at = s.id(Field::At)?;
                if at.reg == Reg::C { self.place_n(at, depth + 1) } else { Some(at) }
            }
            _ => None,
        }
    }

    /// Ancestors of a location (the `in=` chain).
    pub fn ancestors(&self, l: Id) -> Vec<Id> {
        let mut out = Vec::new();
        let mut cur = l;
        while let Some(p) = self.now(cur).and_then(|s| s.id(Field::Parent)) {
            if out.contains(&p) || out.len() > 64 {
                break;
            }
            out.push(p);
            cur = p;
        }
        out
    }

    /// Nearness: the same location or one nested in the other. An unknown place is not a contradiction.
    pub fn near(&self, a: Option<Id>, b: Option<Id>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => a == b || self.ancestors(a).contains(&b) || self.ancestors(b).contains(&a),
            _ => true,
        }
    }

    fn show_place(&self, x: Option<Id>) -> String {
        x.map(|i| i.to_string()).unwrap_or_else(|| "?".into())
    }

    /// Execute a command on this (temporary) world; returns the entities the sentence mentions.
    pub(crate) fn exec(&mut self, l: &Line, pos: TextPos) -> Result<Vec<Id>, GateErr> {
        let mut tx = Tx::default();
        let mut touched = Vec::new();
        let mut cause_of_states = None;
        let mut note = None;
        match &l.cmd {
            Cmd::Cursor { at, time, .. } => {
                if let Some(a) = at {
                    self.need(*a, &[Reg::L])?;
                    touched.push(*a);
                }
                let prev = self.cursor.last().cloned();
                let at = at.or(prev.as_ref().and_then(|c| c.at));
                let time = time.clone().or(prev.and_then(|c| c.time));
                self.cursor.push(Cursor { pos, at, time, line: l.no });
            }
            Cmd::Loc { id, ty, class, name, parent, here } => {
                self.fresh(*id)?;
                tx.put(*id, Delta::Set(Field::Type, Val::Known(V::Tag(*ty))));
                if let Some(c) = class {
                    tx.put(*id, Delta::Set(Field::Class, Val::Known(V::Words(vec![c.clone()]))));
                }
                if let Some(n) = name {
                    tx.put(*id, Delta::Set(Field::Name, Val::Known(V::Text(n.clone()))));
                }
                if let Some(p) = parent {
                    self.need(*p, &[Reg::L])?;
                    tx.put(*id, Delta::Set(Field::Parent, Val::Known(V::Id(*p))));
                    tx.put(*p, Delta::Add(SetF::Children, *id));
                }
                touched.push(*id);
                // specification: a new location — the narration cursor moves there; v2: `here=0` — only a mention
                if *here {
                    let time = self.cursor.last().and_then(|c| c.time.clone());
                    self.cursor.push(Cursor { pos, at: Some(*id), time, line: l.no });
                }
            }
            Cmd::Obj { id, ty, class, name, at, part, num } => {
                self.fresh(*id)?;
                tx.put(*id, Delta::Set(Field::Type, Val::Known(V::Tag(*ty))));
                if let Some(c) = class {
                    tx.put(*id, Delta::Set(Field::Class, Val::Known(V::Words(vec![c.clone()]))));
                }
                if let Some(n) = name {
                    tx.put(*id, Delta::Set(Field::Name, Val::Known(V::Text(n.clone()))));
                }
                if let Some(h) = at {
                    self.need(*h, &[Reg::L, Reg::C])?;
                    tx.put(*id, Delta::Set(Field::At, Val::Known(V::Id(*h))));
                    tx.put(*h, Delta::Add(if h.reg == Reg::L { SetF::Objects } else { SetF::Items }, *id));
                    touched.push(*h);
                }
                if let Some(w) = part {
                    self.need(*w, &[Reg::C, Reg::O])?;
                    tx.put(*id, Delta::Set(Field::PartOf, Val::Known(V::Id(*w))));
                    tx.put(*w, Delta::Add(SetF::Parts, *id));
                    touched.push(*w);
                }
                if let Some(n) = num {
                    tx.put(*id, Delta::Set(Field::Num, Val::Known(n.clone())));
                }
                touched.push(*id);
            }
            Cmd::Char { id, ty, class, name, at, attrs } => {
                self.fresh(*id)?;
                tx.put(*id, Delta::Set(Field::Type, Val::Known(V::Tag(*ty))));
                if let Some(c) = class {
                    tx.put(*id, Delta::Set(Field::Class, Val::Known(V::Words(vec![c.clone()]))));
                }
                if let Some(n) = name {
                    tx.put(*id, Delta::Set(Field::Name, Val::Known(V::Text(n.clone()))));
                }
                if let Some(h) = at {
                    self.need(*h, &[Reg::L])?;
                    tx.put(*id, Delta::Set(Field::At, Val::Known(V::Id(*h))));
                    tx.put(*h, Delta::Add(SetF::Chars, *id));
                    touched.push(*h);
                }
                for (f, v) in attrs {
                    if *f == Field::At {
                        return gate(Gate::Teleport, "a character's place — only at= on creation or move");
                    }
                    self.check_value(*id, *f, v)?;
                    tx.put(*id, Delta::Set(*f, Val::Known(v.clone())));
                }
                touched.push(*id);
            }
            Cmd::Move { who, to, cause } => {
                let e = self.need(*who, &[Reg::C, Reg::O])?;
                let old = e.now().id(Field::At);
                if let Some(c) = cause {
                    self.need_event(*c)?;
                }
                self.real_cause(*cause, "move")?;
                if who.reg == Reg::O {
                    self.not_part(*who)?;
                }
                cause_of_states = *cause;
                if who.reg == Reg::C {
                    self.need(*to, &[Reg::L])?;
                    if old != Some(*to) {
                        tx.put(*who, Delta::Set(Field::At, Val::Known(V::Id(*to))));
                        if let Some(o) = old {
                            tx.put(o, Delta::Del(SetF::Chars, *who));
                        }
                        tx.put(*to, Delta::Add(SetF::Chars, *who));
                    }
                } else {
                    self.need(*to, &[Reg::L, Reg::C])?;
                    if let Some(o) = old
                        && o != *to
                    {
                        match (o.reg, to.reg) {
                            (Reg::C, Reg::C) => {
                                return gate(Gate::Teleport, format!("{who} is in {o} — from character to character only via give"));
                            }
                            (Reg::L, Reg::C) if !self.near(Some(o), self.place(*to)) => {
                                return gate(
                                    Gate::Near,
                                    format!("{who} lies in {o}, but {to} is in {} — it can be picked up only nearby", self.show_place(self.place(*to))),
                                );
                            }
                            (Reg::C, Reg::L) if !self.near(self.place(o), Some(*to)) => {
                                return gate(
                                    Gate::Near,
                                    format!("{who} is in {o}, and that one is in {} — it can be dropped or put only nearby, not into {to}", self.show_place(self.place(o))),
                                );
                            }
                            _ => {}
                        }
                    }
                    if old != Some(*to) {
                        tx.put(*who, Delta::Set(Field::At, Val::Known(V::Id(*to))));
                        if let Some(o) = old {
                            tx.put(o, Delta::Del(if o.reg == Reg::L { SetF::Objects } else { SetF::Items }, *who));
                        }
                        tx.put(*to, Delta::Add(if to.reg == Reg::L { SetF::Objects } else { SetF::Items }, *who));
                    }
                }
                touched.extend([*who, *to]);
            }
            Cmd::Have { who, what, cause } => {
                self.need(*who, &[Reg::C])?;
                let o = self.need(*what, &[Reg::O])?;
                if let Some(c) = cause {
                    self.need_event(*c)?;
                }
                self.real_cause(*cause, "have")?;
                self.not_part(*what)?;
                cause_of_states = *cause;
                // literally per the specification: C.items += O; O.at = C. The old side is not cleaned: a thing
                // that is already somewhere will produce a two-way link mismatch (handing over — give, picking up — move)
                if o.now().id(Field::At) != Some(*who) {
                    tx.put(*who, Delta::Add(SetF::Items, *what));
                    tx.put(*what, Delta::Set(Field::At, Val::Known(V::Id(*who))));
                }
                touched.extend([*who, *what]);
            }
            Cmd::Give { who, what, to, cause } => {
                self.need(*who, &[Reg::C])?;
                self.need(*to, &[Reg::C])?;
                let o = self.need(*what, &[Reg::O])?;
                if let Some(c) = cause {
                    self.need_event(*c)?;
                }
                self.real_cause(*cause, "give")?;
                self.not_part(*what)?;
                cause_of_states = *cause;
                let holder = o.now().id(Field::At);
                if holder != Some(*who) {
                    return gate(Gate::Continuity, format!("{who} does not have {what} ({what}.at = {})", self.show_place(holder)));
                }
                if who == to {
                    return gate(Gate::Continuity, format!("{who} gives {what} to himself"));
                }
                let (pa, pb) = (self.place(*who), self.place(*to));
                if !self.near(pa, pb) {
                    return gate(Gate::Near, format!("{who} is in {}, {to} is in {} — handing over is possible only nearby", self.show_place(pa), self.show_place(pb)));
                }
                tx.put(*who, Delta::Del(SetF::Items, *what));
                tx.put(*to, Delta::Add(SetF::Items, *what));
                tx.put(*what, Delta::Set(Field::At, Val::Known(V::Id(*to))));
                touched.extend([*who, *what, *to]);
            }
            Cmd::Set { who, attrs, cause } => {
                let e = self.need(*who, &[Reg::C, Reg::O, Reg::L])?;
                if let Some(c) = cause {
                    self.need_event(*c)?;
                }
                cause_of_states = *cause;
                let now = e.now();
                for (f, v) in attrs {
                    if *f == Field::At {
                        return gate(Gate::Teleport, format!("set {who} at= — the place is changed only by move (a thing — move or give)"));
                    }
                    if !key_ok(who.reg, *f) {
                        return gate(Gate::Kind, format!("{} — not an attribute of registry {}", f.name(), who.reg.uk()));
                    }
                    self.check_value(*who, *f, v)?;
                    // v3: a second death with a different cause — the same event twice (the solution does not create a new one)
                    if self.v3
                        && *f == Field::Life
                        && now.get(*f) == Val::Known(v.clone())
                        && v == &V::Tag(LIFE.tag("dead").expect("dead"))
                        && let Some(st) = e.set_by(Field::Life, e.latest())
                        && st.cause.is_some()
                        && *cause != st.cause
                    {
                        return gate(
                            Gate::Identity,
                            format!(
                                "{who} is already dead since {}@{} {} (because {}) — second death: the same event twice; write claim/reveal about {}",
                                who,
                                st.ver,
                                st.pos,
                                st.cause.map(|c| c.to_string()).unwrap_or_default(),
                                st.cause.map(|c| c.to_string()).unwrap_or_default()
                            ),
                        );
                    }
                    match now.get(*f) {
                        Val::Known(x) if x == *v => {}
                        Val::Known(x) if cause.is_none() => {
                            let st = e.set_by(*f, e.latest()).map(|s| format!("{who}@{} {}", s.ver, s.pos)).unwrap_or_default();
                            return gate(Gate::Silent, format!("{who}.{} = {x} ({st}) → {v} without cause=E", f.name()));
                        }
                        _ => tx.put(*who, Delta::Set(*f, Val::Known(v.clone()))),
                    }
                }
                touched.push(*who);
            }
            Cmd::Rel { id, kind, a, b, cause, fact } => {
                for x in [cause, fact].into_iter().flatten() {
                    self.need_event(*x)?;
                }
                cause_of_states = *cause;
                match self.ents.get(id) {
                    None => {
                        let (Some(k), Some(a), Some(b)) = (kind, a, b) else {
                            return gate(Gate::Ref, format!("{id} does not exist — a new relation needs: kind, a, b"));
                        };
                        self.need(*a, &[Reg::C])?;
                        self.need(*b, &[Reg::C])?;
                        if a == b {
                            return gate(Gate::Ref, format!("{id}: a = b = {a}"));
                        }
                        tx.put(*id, Delta::Set(Field::Kind, Val::Known(V::Tag(*k))));
                        tx.put(*id, Delta::Set(Field::A, Val::Known(V::Id(*a))));
                        tx.put(*id, Delta::Set(Field::B, Val::Known(V::Id(*b))));
                        tx.put(*a, Delta::Add(SetF::Relations, *id));
                        tx.put(*b, Delta::Add(SetF::Relations, *id));
                        touched.extend([*a, *b]);
                    }
                    Some(e) => {
                        let now = e.now();
                        if let Some(k) = kind
                            && now.get(Field::Kind) != Val::Known(V::Tag(*k))
                        {
                            if cause.is_none() {
                                return gate(Gate::Silent, format!("{id}.kind = {} → {k} without cause=E", now.get(Field::Kind)));
                            }
                            tx.put(*id, Delta::Set(Field::Kind, Val::Known(V::Tag(*k))));
                        }
                        // v2: the direction (a ↔ b between the same two) changes only with a cause
                        if let (Some(x), Some(y)) = (a, b)
                            && now.id(Field::A) == Some(*y)
                            && now.id(Field::B) == Some(*x)
                            && cause.is_none()
                        {
                            return gate(Gate::Silent, format!("{id}: direction {x} ↔ {y} changed without cause=E"));
                        }
                        // ends of the relation — literally: the new end is written, the old one is not cleaned → link gate
                        for (f, x) in [(Field::A, a), (Field::B, b)] {
                            if let Some(x) = x {
                                self.need(*x, &[Reg::C])?;
                                if now.id(f) != Some(*x) {
                                    tx.put(*id, Delta::Set(f, Val::Known(V::Id(*x))));
                                    tx.put(*x, Delta::Add(SetF::Relations, *id));
                                }
                            }
                        }
                    }
                }
                for x in [cause, fact].into_iter().flatten() {
                    if !self.ents.get(id).is_some_and(|e| e.now().has(SetF::Facts, *x)) {
                        tx.put(*id, Delta::Add(SetF::Facts, *x));
                    }
                }
            }
            Cmd::Event { id, verb, roles, why, how, x } => {
                self.fresh(*id)?;
                let mut refs = Vec::new();
                for (r, i) in roles {
                    let e = self.need(*i, r.regs())?;
                    refs.push((*r, e.sref()));
                    touched.push(*i);
                }
                self.check_ev2(*id, x)?;
                let at = roles.iter().find(|(r, _)| *r == Role::At).map(|(_, i)| *i);
                // v3: an event explicitly placed on the story axis (memory, flash-forward) does not stand next to the present
                // states — its place is checked in its own time (in a past frame — in the frame's world), we do not check here
                let placed = self.v3 && x.placement().is_some();
                // each of the joint agents is where the event is; v3: this is the teleportation gate on the story axis —
                // the hero's neighbouring state on the narration line is elsewhere, and there was no transition (move)
                if let Some(at) = at
                    && !placed
                {
                    for (_, ag) in roles.iter().filter(|(r, _)| *r == Role::Agent) {
                        let p = self.place(*ag);
                        if !self.near(Some(at), p) {
                            if self.v3 {
                                return gate(
                                    Gate::Teleport,
                                    format!("{id} at={at}, but agent {ag} was just in {} on the story axis — a memory without a time mark (frame … time=past or before=E)?", self.show_place(p)),
                                );
                            }
                            return gate(Gate::Continuity, format!("{id} at={at}, but agent {ag} is in {}", self.show_place(p)));
                        }
                    }
                }
                // v3: the dead do not act in the present (in a past frame the hero is still alive — that is its own world)
                if self.v3 && !placed && !x.neg {
                    for (_, ag) in roles.iter().filter(|(r, _)| *r == Role::Agent) {
                        if let Some(e) = self.ents.get(ag)
                            && e.now().get(Field::Life) == Val::Known(V::Tag(LIFE.tag("dead").expect("dead")))
                        {
                            let st = e.set_by(Field::Life, e.latest()).map(|s| format!(" since {}@{} {}", ag, s.ver, s.pos)).unwrap_or_default();
                            return gate(Gate::Continuity, format!("{id}: agent {ag} is dead{st} — the dead do not act in the present"));
                        }
                    }
                }
                self.events.push(Event {
                    id: *id,
                    verb: verb.clone(),
                    roles: refs,
                    why: why.clone(),
                    how: how.clone(),
                    speech: None,
                    x: x.clone(),
                    anchors: l.anchors.clone(),
                    pos,
                    line: l.no,
                    world: self.wid,
                    branch: self.branch,
                });
            }
            Cmd::Say { id, who, to, acts, indirect, sincere, means, why, x } => {
                let id = match id {
                    Some(i) => *i,
                    None => Id::new(Reg::E, self.events.iter().map(|e| e.id.n).max().unwrap_or(0) + 1),
                };
                self.fresh(id)?;
                let mut refs = Vec::new();
                if let Speaker::Chars(cs) = who {
                    for c in cs {
                        refs.push((Role::Agent, self.need(*c, &[Reg::C])?.sref()));
                        touched.push(*c);
                    }
                }
                for t in to {
                    refs.push((Role::To, self.need(*t, &[Reg::C])?.sref()));
                    touched.push(*t);
                }
                self.check_ev2(id, x)?;
                self.events.push(Event {
                    id,
                    verb: "say".into(),
                    roles: refs,
                    why: why.clone(),
                    how: None,
                    speech: Some(Speech {
                        narrator: *who == Speaker::Narrator,
                        act: acts[0],
                        also: acts[1..].to_vec(),
                        indirect: *indirect,
                        sincere: *sincere,
                        means: means.clone(),
                    }),
                    x: x.clone(),
                    anchors: l.anchors.clone(),
                    pos,
                    line: l.no,
                    world: self.wid,
                    branch: self.branch,
                });
            }
            Cmd::Plan { id, kind: Some(k), who, to, what, src, .. } => {
                self.fresh(*id)?;
                for c in who {
                    self.need(*c, &[Reg::C])?;
                    tx.put(*c, Delta::Add(SetF::Plans, *id));
                    tx.put(*id, Delta::Add(SetF::Owners, *c));
                    touched.push(*c);
                }
                tx.put(*id, Delta::Set(Field::Kind, Val::Known(V::Tag(*k))));
                tx.put(*id, Delta::Set(Field::Stage, Val::Known(V::Tag(PLAN_STATUS.tag("open").expect("open")))));
                if let Some(w) = what {
                    tx.put(*id, Delta::Set(Field::What, Val::Known(V::Text(w.clone()))));
                }
                if let Some(t) = to {
                    self.need(*t, &[Reg::C])?;
                    tx.put(*id, Delta::Set(Field::Target, Val::Known(V::Id(*t))));
                    touched.push(*t);
                }
                if let Some(s) = src {
                    self.need_event(*s)?;
                    tx.put(*id, Delta::Set(Field::Src, Val::Known(V::Id(*s))));
                }
                touched.push(*id);
            }
            Cmd::Plan { id, status: Some(s), by, .. } => {
                let p = self.need(*id, &[Reg::P])?.now();
                let by = self.need_cause(*by, &format!("{id} status={s}"))?;
                let cur = p.get(Field::Stage);
                if cur != Val::Known(V::Tag(PLAN_STATUS.tag("open").expect("open"))) {
                    return gate(Gate::Continuity, format!("{id} already closed ({cur}) — closed only once"));
                }
                if s.as_str() == "kept" && self.need_event(by)?.x.neg {
                    return gate(Gate::Continuity, format!("{id} kept by={by}, but {by} is an event that did not happen (neg=1)"));
                }
                tx.put(*id, Delta::Set(Field::Stage, Val::Known(V::Tag(*s))));
                tx.put(*id, Delta::Set(Field::By, Val::Known(V::Id(by))));
                cause_of_states = Some(by);
                touched.push(*id);
                touched.extend(p.ids(SetF::Owners));
            }
            Cmd::Plan { .. } => unreachable!("the parser passes only creation or closing"),
            Cmd::Believe { id, who: Some(c), claim, about, from, truth, .. } => {
                self.fresh(*id)?;
                self.need(*c, &[Reg::C])?;
                tx.put(*c, Delta::Add(SetF::Beliefs, *id));
                tx.put(*id, Delta::Add(SetF::Owners, *c));
                if let Some(t) = claim {
                    tx.put(*id, Delta::Set(Field::What, Val::Known(V::Text(t.clone()))));
                }
                tx.put(*id, Delta::Set(Field::Stage, Val::Known(V::Tag(BELIEF_STATUS.tag("held").expect("held")))));
                if let Some(a) = about {
                    self.need_any(*a)?;
                    tx.put(*id, Delta::Set(Field::About, Val::Known(V::Id(*a))));
                }
                if let Some(f) = from {
                    self.need_event(*f)?;
                    tx.put(*id, Delta::Set(Field::Src, Val::Known(V::Id(*f))));
                }
                if let Some(t) = truth {
                    tx.put(*id, Delta::Set(Field::Truth, Val::Known(V::Tag(TRUTH.tag(if *t { "true" } else { "false" }).expect("truth")))));
                }
                touched.extend([*c, *id]);
            }
            Cmd::Believe { id, truth, status, by, .. } => {
                let b = self.need(*id, &[Reg::B])?.now();
                let by = self.need_cause(*by, &format!("{id}: belief change"))?;
                if let Some(s) = status {
                    if b.get(Field::Stage) != Val::Known(V::Tag(BELIEF_STATUS.tag("held").expect("held"))) {
                        return gate(Gate::Continuity, format!("{id} already abandoned — abandoned only once"));
                    }
                    tx.put(*id, Delta::Set(Field::Stage, Val::Known(V::Tag(*s))));
                }
                if let Some(t) = truth {
                    let v = Val::Known(V::Tag(TRUTH.tag(if *t { "true" } else { "false" }).expect("truth")));
                    if b.get(Field::Truth) != v {
                        tx.put(*id, Delta::Set(Field::Truth, v));
                    }
                }
                tx.put(*id, Delta::Set(Field::By, Val::Known(V::Id(by))));
                cause_of_states = Some(by);
                touched.push(*id);
                touched.extend(b.ids(SetF::Owners));
            }
            Cmd::Done { who, cause } => {
                let s = self.need(*who, &[Reg::C])?.now();
                if s.get(Field::Goal) == Val::Unknown {
                    return gate(Gate::Continuity, format!("done {who}: no goal — nothing to achieve"));
                }
                let c = self.need_cause(*cause, &format!("done {who}"))?;
                self.real_cause(Some(c), &format!("done {who}"))?;
                tx.put(*who, Delta::Set(Field::Goal, Val::Unknown));
                cause_of_states = Some(c);
                touched.push(*who);
            }
            Cmd::Unset { who, field, cause, why } => {
                let e = self.need(*who, &[Reg::C])?;
                if e.now().get(*field) == Val::Unknown {
                    return gate(Gate::Continuity, format!("unset {who} {}: the value is unknown anyway", field.name()));
                }
                let c = self.need_cause(*cause, &format!("unset {who} {}", field.name()))?;
                tx.put(*who, Delta::Set(*field, Val::Unknown));
                cause_of_states = Some(c);
                note = why.clone();
                touched.push(*who);
            }
            Cmd::Feel { who, add, del, cause } => {
                let s = self.need(*who, &[Reg::C])?.now();
                if let Some(c) = cause {
                    self.need_event(*c)?;
                }
                let cur: Vec<Tag> = match s.get(Field::Feeling) {
                    Val::Known(v) => v.tag_list(),
                    Val::Unknown => Vec::new(),
                };
                for d in del {
                    if !cur.contains(d) {
                        return gate(Gate::Continuity, format!("feel {who} -{d}: this feeling is absent ({})", s.get(Field::Feeling)));
                    }
                }
                let mut new: Vec<Tag> = cur.iter().copied().filter(|t| !del.contains(t)).collect();
                new.extend(add.iter().copied().filter(|t| !cur.contains(t)));
                let newv = V::tags(new);
                let oldv = V::tags(cur.clone());
                // unknown → known is a discovery; changing a known value only with a cause
                if !cur.is_empty() && newv != oldv && cause.is_none() {
                    return gate(Gate::Silent, format!("feel {who}: {} → {} without cause=E", s.get(Field::Feeling), newv.map(|v| v.to_string()).unwrap_or("?".into())));
                }
                if newv != oldv {
                    tx.put(*who, Delta::Set(Field::Feeling, newv.map(Val::Known).unwrap_or_default()));
                }
                cause_of_states = *cause;
                touched.push(*who);
            }
            Cmd::Detach { what, to, cause } => {
                let s = self.need(*what, &[Reg::O])?.now();
                let Some(whole) = s.id(Field::PartOf) else {
                    return gate(Gate::Continuity, format!("detach {what}: this is not a part"));
                };
                self.need(*to, &[Reg::L, Reg::C])?;
                let c = self.need_cause(*cause, &format!("detach {what}"))?;
                self.real_cause(Some(c), &format!("detach {what}"))?;
                let (pw, pt) = (self.place(whole), self.place(*to));
                if !self.near(pw, pt) {
                    return gate(Gate::Near, format!("{what} is part of {whole} in {}, but {to} is in {} — detaching is possible only nearby", self.show_place(pw), self.show_place(pt)));
                }
                tx.put(*what, Delta::Set(Field::PartOf, Val::Unknown));
                tx.put(whole, Delta::Del(SetF::Parts, *what));
                tx.put(*what, Delta::Set(Field::At, Val::Known(V::Id(*to))));
                tx.put(*to, Delta::Add(if to.reg == Reg::L { SetF::Objects } else { SetF::Items }, *what));
                cause_of_states = Some(c);
                touched.extend([*what, whole, *to]);
            }
            Cmd::Attach { what, to, cause } => {
                let s = self.need(*what, &[Reg::O])?.now();
                if let Some(w) = s.id(Field::PartOf) {
                    return gate(Gate::Continuity, format!("attach {what}: already part of {w}"));
                }
                self.need(*to, &[Reg::C, Reg::O])?;
                let c = self.need_cause(*cause, &format!("attach {what}"))?;
                self.real_cause(Some(c), &format!("attach {what}"))?;
                let (pw, pt) = (self.place(*what), self.place(*to));
                if !self.near(pw, pt) {
                    return gate(Gate::Near, format!("{what} is in {}, but {to} is in {} — attaching is possible only nearby", self.show_place(pw), self.show_place(pt)));
                }
                if let Some(h) = s.id(Field::At) {
                    tx.put(h, Delta::Del(if h.reg == Reg::L { SetF::Objects } else { SetF::Items }, *what));
                    tx.put(*what, Delta::Set(Field::At, Val::Unknown));
                    touched.push(h);
                }
                tx.put(*what, Delta::Set(Field::PartOf, Val::Known(V::Id(*to))));
                tx.put(*to, Delta::Add(SetF::Parts, *what));
                cause_of_states = Some(c);
                touched.extend([*what, *to]);
            }
            Cmd::Mention { ids } => {
                for i in ids {
                    self.need_any(*i)?;
                    touched.push(*i);
                }
            }
            Cmd::Frame { .. } | Cmd::Claim { .. } | Cmd::Time { .. } | Cmd::Branch { .. } | Cmd::Know { .. } | Cmd::Reveal { .. } => {
                return gate(Gate::Frame, format!("{} is a narrative-layer command (root), not a frame-world command", l.cmd.name()));
            }
        }
        // v3: the canonical world is changed only by events that really happened in this world
        if !tx.d.is_empty()
            && let Some(c) = cause_of_states
            && let Some(ev) = self.event(c)
        {
            if ev.world != self.wid {
                return gate(Gate::Scope, format!("{} via {c} — an event of another world (a story within a story does not change the world listening to it)", l.cmd.name()));
            }
            if ev.branch != self.branch {
                return gate(Gate::Branch, format!("{} via {c} — an event of branch {} (the canonical world is changed only by events that really happened)", l.cmd.name(), ev.branch));
            }
        }
        for (id, delta) in tx.d {
            let e = self.ents.entry(id).or_insert_with(|| Entity { id, states: Vec::new(), mentions: Vec::new() });
            let ver = e.latest() + 1;
            e.states.push(State { ver, delta, cause: cause_of_states, by: l.cmd.name(), line: l.no, pos, note: note.clone() });
        }
        Ok(touched)
    }

    /// Event references v2: `cause=`, `before=` — existing events; `about=` — anything existing.
    fn check_ev2(&self, id: Id, x: &Ev2) -> Result<(), GateErr> {
        for e in [x.cause, x.before, x.after, x.during, x.same].into_iter().flatten() {
            if e == id {
                return gate(Gate::Ref, format!("{id} refers to itself"));
            }
            self.need_event(e)?;
        }
        // v3: placement in time — only on the same axis (in the same world)
        if let Some((r, e)) = x.placement()
            && let Some(ev) = self.event(e)
            && ev.world != self.wid
        {
            return gate(Gate::Scope, format!("{id} {}={e}: {e} is from another world, with its own time axis", r.name().to_lowercase()));
        }
        if let Some(a) = x.about {
            self.need_any(a)?;
        }
        Ok(())
    }

    /// Attribute value: a reference to a character — existing and not itself.
    fn check_value(&self, who: Id, f: Field, v: &V) -> Result<(), GateErr> {
        if let V::Id(x) = v
            && matches!(f, Field::Mother | Field::Father)
        {
            self.need(*x, &[Reg::C])?;
            if *x == who {
                return gate(Gate::Ref, format!("{who}.{} = itself", f.name()));
            }
        }
        Ok(())
    }

    /// Two-way links on the last states: `L.chars ∋ C ⇔ C.at = L`, `L.objects ∋ O ⇔ O.at = L`,
    /// `C.items ∋ O ⇔ O.at = C`, `L.places ∋ M ⇔ M.in = L`, `R.a/R.b = C ⇔ C.relations ∋ R`.
    pub fn check_links(&self) -> Result<(), GateErr> {
        let snap = |id: Id| self.now(id).unwrap_or_default();
        let at = |id: Id| self.show_place(snap(id).id(Field::At));
        for e in self.ents.values() {
            let id = e.id;
            let s = e.now();
            match id.reg {
                Reg::C => {
                    if let Some(l) = s.id(Field::At)
                        && !snap(l).has(SetF::Chars, id)
                    {
                        return gate(Gate::Link, format!("{id}.at = {l}, but {l}.chars ∌ {id}"));
                    }
                    for o in s.ids(SetF::Items) {
                        if snap(o).id(Field::At) != Some(id) {
                            return gate(Gate::Link, format!("{id}.items ∋ {o}, but {o}.at = {}", at(o)));
                        }
                    }
                    for r in s.ids(SetF::Relations) {
                        let rs = snap(r);
                        if rs.id(Field::A) != Some(id) && rs.id(Field::B) != Some(id) {
                            return gate(Gate::Link, format!("{id}.relations ∋ {r}, but {r} is between {} and {}", rs.get(Field::A), rs.get(Field::B)));
                        }
                    }
                    // v2: intentions and beliefs belong to their owners; parts — to the whole
                    for (set, what) in [(SetF::Plans, "plans"), (SetF::Beliefs, "beliefs")] {
                        for p in s.ids(set) {
                            if !snap(p).has(SetF::Owners, id) {
                                return gate(Gate::Link, format!("{id}.{what} ∋ {p}, but {p}.owners ∌ {id}"));
                            }
                        }
                    }
                    self.check_parts(id, &s)?;
                }
                Reg::O => {
                    if let Some(h) = s.id(Field::At) {
                        let set = if h.reg == Reg::L { SetF::Objects } else { SetF::Items };
                        if !snap(h).has(set, id) {
                            return gate(Gate::Link, format!("{id}.at = {h}, but {h}.{} ∌ {id}", set.name()));
                        }
                    }
                    if let Some(w) = s.id(Field::PartOf) {
                        if s.id(Field::At).is_some() {
                            return gate(Gate::Link, format!("{id} is both a part of {w} and lies separately in {}", at(id)));
                        }
                        if !snap(w).has(SetF::Parts, id) {
                            return gate(Gate::Link, format!("{id}.part_of = {w}, but {w}.parts ∌ {id}"));
                        }
                    }
                    self.check_parts(id, &s)?;
                }
                Reg::P | Reg::B => {
                    let set = if id.reg == Reg::P { SetF::Plans } else { SetF::Beliefs };
                    for c in s.ids(SetF::Owners) {
                        if !snap(c).has(set, id) {
                            return gate(Gate::Link, format!("{id}.owners ∋ {c}, but {c}.{} ∌ {id}", set.name()));
                        }
                    }
                }
                Reg::L => {
                    for c in s.ids(SetF::Chars) {
                        if snap(c).id(Field::At) != Some(id) {
                            return gate(Gate::Link, format!("{id}.chars ∋ {c}, but {c}.at = {}", at(c)));
                        }
                    }
                    for o in s.ids(SetF::Objects) {
                        if snap(o).id(Field::At) != Some(id) {
                            return gate(Gate::Link, format!("{id}.objects ∋ {o}, but {o}.at = {}", at(o)));
                        }
                    }
                    for m in s.ids(SetF::Children) {
                        if snap(m).id(Field::Parent) != Some(id) {
                            return gate(Gate::Link, format!("{id}.places ∋ {m}, but {m}.in = {}", snap(m).get(Field::Parent)));
                        }
                    }
                    if let Some(p) = s.id(Field::Parent)
                        && !snap(p).has(SetF::Children, id)
                    {
                        return gate(Gate::Link, format!("{id}.in = {p}, but {p}.places ∌ {id}"));
                    }
                }
                Reg::R => {
                    for f in [Field::A, Field::B] {
                        if let Some(c) = s.id(f)
                            && !snap(c).has(SetF::Relations, id)
                        {
                            return gate(Gate::Link, format!("{id}.{} = {c}, but {c}.relations ∌ {id}", f.name()));
                        }
                    }
                }
                Reg::E | Reg::F | Reg::K => {}
            }
        }
        Ok(())
    }

    /// `X.parts ∋ O ⇔ O.part_of = X` (v2).
    fn check_parts(&self, id: Id, s: &Snap) -> Result<(), GateErr> {
        for o in s.ids(SetF::Parts) {
            let w = self.now(o).and_then(|x| x.id(Field::PartOf));
            if w != Some(id) {
                return gate(Gate::Link, format!("{id}.parts ∋ {o}, but {o}.part_of = {}", self.show_place(w)));
            }
        }
        Ok(())
    }

    /// Tests only: append a state bypassing the executor (to show that the link gate sees the mismatch).
    #[cfg(test)]
    pub(crate) fn inject(&mut self, id: Id, delta: Vec<Delta>) {
        let e = self.ents.entry(id).or_insert_with(|| Entity { id, states: Vec::new(), mentions: Vec::new() });
        let ver = e.latest() + 1;
        e.states.push(State { ver, delta, cause: None, by: "inject", line: 0, pos: self.pos, note: None });
    }
}

/// Command run: accepted, gate refusals (line and reason), sentences without any command.
pub struct Run {
    pub world: World,
    pub accepted: usize,
    pub rejects: Vec<(Line, GateErr)>,
    pub uncovered: Vec<u16>,
}

/// Apply lines one by one; refusal of one does not stop the rest (the cascade is visible in the report).
pub fn run(text: Text, lines: &[Line]) -> Run {
    run_mode(text, lines, false)
}

/// Run in v3 mode (gates in story time): for linear narration gives the same as v2.
pub fn run_v3(text: Text, lines: &[Line]) -> Run {
    run_mode(text, lines, true)
}

pub fn run_mode(text: Text, lines: &[Line], v3: bool) -> Run {
    let mut w = if v3 { World::new_v3(text) } else { World::new(text) };
    let mut rejects = Vec::new();
    let mut accepted = 0;
    let mut covered = BTreeSet::new();
    for l in lines {
        covered.extend(l.anchors.iter().copied());
        match w.apply(l) {
            Ok(()) => accepted += 1,
            Err(e) => rejects.push((l.clone(), e)),
        }
    }
    let uncovered = (1..=w.text.sents.len() as u16).filter(|s| !covered.contains(s)).collect();
    Run { world: w, accepted, rejects, uncovered }
}
