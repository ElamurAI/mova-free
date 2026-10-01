//! The v3 pipeline at the root of the world (, section "Pipeline v3"): sentence cursor → narrative frame → mention
//! of an event or claim → resolution into a canonical event or a new one → story time axis → branch of reality →
//! observer knowledge → only then do the gates decide whether the canonical world changes.
//!
//! World commands (v1/v2) run in the world of the current frame: at the root — in the document's own world, in a past
//! frame — in its own world (a projection of the parent's world without places and transient states), in a story within
//! a story — in a new world. New events are registered on the story time axis: without explicit placement — on the
//! frame's narrative line (told in sequence — happened in sequence), with `before=`/`after=`/`during=`/`same=` — off
//! the line. Claims (`K`), knowledge, reveals, branches — in the root's narrative layer.

use std::collections::{BTreeMap, BTreeSet};

use crate::lang::{Cmd, Line, Obs, TRel};
use crate::story::*;
use crate::types::*;
use crate::world::{Delta, Entity, Event, Gate, GateErr, State, World, gate};

/// Observers other than characters.
pub static OBSERVER: Closed = Closed { what: "observer", all: &["reader", "narrator"] };

fn obs_val(o: Obs) -> V {
    match o {
        Obs::C(c) => V::Id(c),
        Obs::Reader => V::Tag(OBSERVER.tag("reader").expect("reader")),
        Obs::Narrator => V::Tag(OBSERVER.tag("narrator").expect("narrator")),
    }
}

/// Observer from the value of a claim's `who` field.
pub fn val_obs(v: &Val) -> Option<Obs> {
    match v {
        Val::Known(V::Id(c)) => Some(Obs::C(*c)),
        Val::Known(V::Tag(t)) if t.as_str() == "reader" => Some(Obs::Reader),
        Val::Known(V::Tag(t)) if t.as_str() == "narrator" => Some(Obs::Narrator),
        _ => None,
    }
}

/// The frame whose world applies to frame `fi`: the nearest one with its own world (root — 0).
pub fn world_frame(st: &Story, mut fi: usize) -> usize {
    while fi != 0 && st.frames[fi].world.is_none() {
        fi = st.frames[fi].parent.unwrap_or(0);
    }
    fi
}

fn tag(set: &'static Closed, s: &str) -> Tag {
    set.tag(s).expect("closed set")
}

impl World {
    /// All worlds: the root and the frames' worlds (for queries after building).
    pub fn worlds(&self) -> Vec<&World> {
        let mut v = vec![self];
        if let Some(st) = self.story() {
            for f in &st.frames {
                if let Some(w) = f.world.as_deref() {
                    v.push(w);
                }
            }
        }
        v
    }

    /// Event from any world (first found — root, then frames).
    pub fn event_any(&self, id: Id) -> Option<&Event> {
        self.worlds().into_iter().find_map(|w| w.event(id))
    }

    /// Entity from any world: the world where it has the most states (fullest history).
    pub fn ent_any(&self, id: Id) -> Option<(&World, &Entity)> {
        self.worlds().into_iter().filter_map(|w| w.ent(id).map(|e| (w, e))).max_by_key(|(_, e)| e.states().iter().filter(|s| s.by != "project").count())
    }

    /// Human-readable label from any world.
    pub fn label_any(&self, id: Id) -> String {
        if id.reg == Reg::E {
            for w in self.worlds() {
                if w.event(id).is_some() {
                    return w.label(id);
                }
            }
        }
        match self.ent_any(id) {
            Some((w, _)) => w.label(id),
            None => id.to_string(),
        }
    }

    /// Root: a v3 command — narrative layer; a world command — in the world of the current frame, then event registration.
    pub(crate) fn apply_root(&mut self, l: &Line, pos: TextPos) -> Result<(), GateErr> {
        let mut st = self.story.take().expect("root narrative layer");
        let r = self.step(&mut st, l, pos);
        self.story = Some(st);
        r
    }

    fn step(&mut self, st: &mut Story, l: &Line, pos: TextPos) -> Result<(), GateErr> {
        if l.cmd.is_v3() {
            return self.exec_v3(st, l, pos);
        }
        let line = l.clone();
        let fi = st.top();
        let wi = world_frame(st, fi);
        causal_stamp(st, fi, &l.cmd, l.no);
        if wi == 0 {
            let new = self.apply_local(&line, pos)?;
            for e in new {
                let ev = self.event(e).expect("just created").clone();
                register(st, fi, &ev, l.no)?;
            }
        } else {
            let mut fw = st.frames[wi].world.take().expect("frame world");
            let r = (|| -> Result<(), GateErr> {
                let p = fw.position(&line)?;
                let new = fw.apply_local(&line, p)?;
                for e in new {
                    let ev = fw.event(e).expect("just created").clone();
                    register(st, fi, &ev, l.no)?;
                }
                Ok(())
            })();
            st.frames[wi].world = Some(fw);
            r?;
        }
        Ok(())
    }

    /// Frame world for reading (root — the world itself).
    fn wref<'a>(&'a self, st: &'a Story, wi: usize) -> &'a World {
        if wi == 0 { self } else { st.frames[wi].world.as_deref().expect("frame world") }
    }

    /// Event from any world during execution (the narrative layer is taken out of the root).
    fn ev_in<'a>(&'a self, st: &'a Story, id: Id) -> Option<&'a Event> {
        self.event(id).or_else(|| st.frames.iter().filter_map(|f| f.world.as_deref()).find_map(|w| w.event(id)))
    }

    fn ent_in<'a>(&'a self, st: &'a Story, id: Id) -> bool {
        self.ent(id).is_some() || st.frames.iter().filter_map(|f| f.world.as_deref()).any(|w| w.ent(id).is_some())
    }

    /// The character observer exists in the current world.
    fn need_obs(&self, st: &Story, o: Obs) -> Result<(), GateErr> {
        if let Obs::C(c) = o {
            let wi = world_frame(st, st.top());
            self.wref(st, wi).need(c, &[Reg::C])?;
        }
        Ok(())
    }

    fn exec_v3(&mut self, st: &mut Story, l: &Line, pos: TextPos) -> Result<(), GateErr> {
        match &l.cmd {
            Cmd::Frame { id, open: true, narrator, to, level, world, time, before, src, branch } => {
                let parent = st.top();
                let (narrator, level) = (narrator.expect("parsed"), level.expect("parsed"));
                // continuation of an interrupted frame: same parent, level, narrator
                if let Some(fi) = st.frame_by_id(*id) {
                    let f = &st.frames[fi];
                    if f.is_open() {
                        return gate(Gate::Frame, format!("{id} already open"));
                    }
                    if f.parent != Some(parent) || f.level != level || f.narrator != narrator {
                        return gate(
                            Gate::Frame,
                            format!("{id} is continued only from the same frame, with level {} and narrator {} (now {} level {level} narrator {narrator})", f.level, f.narrator, st.frames[parent].name()),
                        );
                    }
                    if f.kind == FWorld::Past {
                        let pw = world_frame(st, parent);
                        let fresh = self.wref(st, pw).project_missing(st.frames[fi].world.as_deref().expect("frame world"));
                        let fw = st.frames[fi].world.as_deref_mut().expect("frame world");
                        fw.absorb(fresh);
                    }
                    st.frames[fi].spans.push((pos.sent, None));
                    st.stack.push(fi);
                    return Ok(());
                }
                if level == 0 {
                    if st.root_named || st.stack.len() > 1 || !st.frames[0].main.is_empty() && st.frames[0].id.is_some() {
                        return gate(Gate::Frame, format!("{id} level=0: the root is named once, as the first frame"));
                    }
                    if world.is_some() || time.is_some() || before.is_some() || branch.is_some() {
                        return gate(Gate::Frame, "level=0 — root: only narrator= and to=");
                    }
                    self.need_obs(st, narrator)?;
                    for o in to {
                        self.need_obs(st, *o)?;
                    }
                    let r = &mut st.frames[0];
                    r.id = Some(*id);
                    r.narrator = narrator;
                    if !to.is_empty() {
                        r.to = to.clone();
                    }
                    st.root_named = true;
                    return Ok(());
                }
                let pl = st.frames[parent].level;
                if level != pl + 1 {
                    return gate(Gate::Frame, format!("{id} level={level}, but frame {} has level {pl} — a nested frame is one level deeper", st.frames[parent].name()));
                }
                self.need_obs(st, narrator)?;
                for o in to {
                    self.need_obs(st, *o)?;
                }
                let pw = world_frame(st, parent);
                for e in [*src, *before].into_iter().flatten() {
                    let ev = self.wref(st, pw).need_event(e)?;
                    if !ev.happened() {
                        return gate(Gate::Branch, format!("{id}: {e} did not happen — a past frame only precedes what happened"));
                    }
                }
                let kind = if branch.is_some() {
                    FWorld::Branch
                } else if world.is_some_and(|w| w.as_str() == "new") {
                    FWorld::New
                } else if time.is_some_and(|t| t.as_str() == "now") {
                    FWorld::Now
                } else {
                    FWorld::Past
                };
                let fi = st.frames.len();
                let (base_wid, built) = {
                    let base = self.wref(st, pw);
                    let built = match kind {
                        FWorld::Past => Some(base.project_past()),
                        FWorld::Now => None,
                        FWorld::New => {
                            let mut nw = World::new(base.text.clone());
                            nw.story = None;
                            nw.v3 = base.v3;
                            nw.pos = base.pos;
                            Some(nw)
                        }
                        FWorld::Branch | FWorld::Root => {
                            let mut bw = base.clone();
                            bw.story = None;
                            bw.branch = branch.expect("branch");
                            bw.cursor.clear();
                            Some(bw)
                        }
                    };
                    (base.wid, built)
                };
                let (wid, fworld, line) = match kind {
                    FWorld::Past => (base_wid, built.map(Box::new), fi),
                    FWorld::Now => (base_wid, None, st.frames[parent].line),
                    _ => {
                        let w = st.next_wid;
                        st.next_wid += 1;
                        let mut b = built.expect("frame world");
                        b.wid = w;
                        (w, Some(Box::new(b)), fi)
                    }
                };
                let fb = branch.unwrap_or_else(|| tag(&BRANCH, "real"));
                // boundary of a past frame: everything in it precedes the parent's event (before=, src= or the last on the line)
                let anchor = if kind == FWorld::Past { before.or(*src).or_else(|| st.frames[st.frames[parent].line].main.last().copied()) } else { None };
                st.frames.push(Frame {
                    id: Some(*id),
                    narrator,
                    to: to.clone(),
                    level,
                    parent: Some(parent),
                    kind,
                    branch: fb,
                    wid,
                    line,
                    anchor,
                    src: *src,
                    spans: vec![(pos.sent, None)],
                    world: fworld,
                    main: Vec::new(),
                });
                if kind == FWorld::Past {
                    let fs = st.time.node(Pt::FS(fi), wid);
                    let fe = st.time.node(Pt::FE(fi), wid);
                    st.time.add(fs, fe, false, Why::Span).map_err(|_| GateErr { gate: Gate::Time, msg: "frame bounds".into() })?;
                    if let Some(a) = anchor
                        && let Some(sa) = st.time.get(Pt::S(a))
                        && let Err(back) = st.time.add(fe, sa, true, Why::FrameBound(fi))
                    {
                        return gate(Gate::Time, format!("{id} before {a}: contradiction — {}", st.time.explain(&back, &st.frames)));
                    }
                }
                st.stack.push(fi);
                Ok(())
            }
            Cmd::Frame { id, open: false, .. } => {
                let Some(fi) = st.frame_by_id(*id) else { return gate(Gate::Ref, format!("{id} not in the frame registry")) };
                if fi == 0 {
                    return gate(Gate::Frame, format!("{id} is the root, it is not closed"));
                }
                if st.top() != fi {
                    return gate(Gate::Frame, format!("frame close {id}, but the deepest open one is {} — frames are closed in order, like brackets", st.frames[st.top()].name()));
                }
                st.stack.pop();
                if let Some(s) = st.frames[fi].spans.last_mut() {
                    s.1 = Some(pos.sent);
                }
                let parent = st.frames[fi].parent.unwrap_or(0);
                let pw = world_frame(st, parent);
                let kind = st.frames[fi].kind;
                if matches!(kind, FWorld::Past | FWorld::New | FWorld::Branch) {
                    let child = st.frames[fi].world.take().expect("frame world");
                    let warns = if pw == 0 {
                        self.merge_from(&child, kind, pos, l.no, &st.frames[fi].name())
                    } else {
                        let mut p = st.frames[pw].world.take().expect("parent world");
                        let w = p.merge_from(&child, kind, pos, l.no, &st.frames[fi].name());
                        st.frames[pw].world = Some(p);
                        w
                    };
                    st.frames[fi].world = Some(child);
                    st.warnings.extend(warns);
                }
                Ok(())
            }
            Cmd::Claim { id, who: Some(who), about: Some(about), status, interp, from, .. } => {
                if self.ent(*id).is_some() {
                    return gate(Gate::Ref, format!("{id} already exists — an id is created once"));
                }
                self.need_obs(st, *who)?;
                if !st.ev.contains_key(about) {
                    return gate(Gate::Ref, format!("claim {id} about={about}: {about} not in the event registry (the event is written before the claim)"));
                }
                if let Some(f) = from
                    && self.ev_in(st, *f).is_none()
                {
                    return gate(Gate::Ref, format!("claim {id} from={f}: no such event"));
                }
                let mut d = vec![
                    Delta::Set(Field::Who, Val::Known(obs_val(*who))),
                    Delta::Set(Field::About, Val::Known(V::Id(*about))),
                    Delta::Set(Field::Stage, Val::Known(V::Tag(*status))),
                ];
                if let Some(t) = interp {
                    d.push(Delta::Set(Field::What, Val::Known(V::Text(t.clone()))));
                }
                if let Some(f) = from {
                    d.push(Delta::Set(Field::Src, Val::Known(V::Id(*f))));
                }
                self.ents.insert(
                    *id,
                    Entity { id: *id, states: vec![State { ver: 1, delta: d, cause: None, by: "claim", line: l.no, pos, note: None }], mentions: vec![pos] },
                );
                Ok(())
            }
            Cmd::Claim { id, status, by, .. } => {
                let now = self.need(*id, &[Reg::K])?.now();
                let Some(by) = by else { return gate(Gate::Silent, format!("claim {id} status={status} without by=E — a claim is changed by an event")) };
                if self.ev_in(st, *by).is_none() {
                    return gate(Gate::Ref, format!("claim {id} by={by}: no such event"));
                }
                if now.get(Field::Stage) == Val::Known(V::Tag(*status)) {
                    return gate(Gate::Continuity, format!("claim {id}: status already {status}"));
                }
                self.claim_state(*id, *status, Some(*by), l.no, pos, "claim");
                Ok(())
            }
            Cmd::Time { a, rel: Some((r, b)), .. } => {
                for e in [a, b] {
                    if !st.ev.contains_key(e) {
                        return gate(Gate::Ref, format!("time: {e} not in the event registry"));
                    }
                }
                if st.ev[a].wid != st.ev[b].wid {
                    return gate(Gate::Scope, format!("time {a} {} {b}: different worlds — different time axes (story within a story)", r.name()));
                }
                relate(st, *a, *r, *b, Why::Time(l.no))
            }
            Cmd::Time { a, at: Some(t), .. } => {
                if !st.ev.contains_key(a) {
                    return gate(Gate::Ref, format!("time: {a} not in the event registry"));
                }
                let norm = |s: &str| s.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ").replace(['’', '‘'], "'");
                let want = norm(t);
                let found = l.anchors.iter().any(|s| self.text.sents.get(*s as usize - 1).is_some_and(|(_, x)| norm(x).contains(&want)));
                if !found {
                    return gate(Gate::Time, format!("time {a} at=«{t}»: these words are not in sentence {} — absolute time only from the text", l.anchors.iter().map(|s| format!("^s{s}")).collect::<Vec<_>>().join(" ")));
                }
                st.ev.get_mut(a).expect("exists").abs.push((t.clone(), pos.sent));
                Ok(())
            }
            Cmd::Time { .. } => unreachable!("the parser passes either a relation or at="),
            Cmd::Branch { ev, branch, by } => {
                let Some(m) = st.ev.get(ev).cloned() else { return gate(Gate::Ref, format!("branch: {ev} not in the event registry")) };
                if let Some(b) = by
                    && self.ev_in(st, *b).is_none()
                {
                    return gate(Gate::Ref, format!("branch {ev} by={b}: no such event"));
                }
                if m.branch == *branch {
                    return gate(Gate::Continuity, format!("{ev} already in branch {branch}"));
                }
                if branch.as_str() != "real" && m.branch.as_str() == "real" {
                    // the canonical world has already been changed by this event — it cannot "not happen"
                    if let Some(why) = self.consequences(st, *ev) {
                        return gate(Gate::Branch, format!("{ev} → {branch}: the event already changed the canonical world ({why}) — a lie or a dream does not change the world"));
                    }
                    unchain(st, *ev);
                }
                let e = st.ev.get_mut(ev).expect("exists");
                e.branch = *branch;
                e.branch_log.push((*branch, l.no, pos.sent, *by));
                self.set_branch_everywhere(st, *ev, *branch);
                Ok(())
            }
            Cmd::Know { who, fact, from, how, by } => {
                self.need_obs(st, *who)?;
                let exists = match fact.reg {
                    Reg::E => self.ev_in(st, *fact).is_some(),
                    _ => self.ent_in(st, *fact),
                };
                if !exists {
                    return gate(Gate::Ref, format!("know fact={fact}: no such thing"));
                }
                let max = l.anchors.iter().max().copied().unwrap_or(0);
                if *from > max {
                    return gate(Gate::Anchor, format!("know from=^s{from}, but the command is at ^s{max} — knowledge from a future sentence"));
                }
                if fact.reg == Reg::E && !st.is_real(*fact) {
                    return gate(Gate::Branch, format!("know fact={fact}: {fact} in branch {} — one can know what happened; for a lie or a dream — a claim", st.ev[fact].branch));
                }
                if let Some(b) = by
                    && self.ev_in(st, *b).is_none()
                {
                    return gate(Gate::Ref, format!("know by={b}: no such event"));
                }
                st.knows.push(Know { who: *who, fact: *fact, from: *from, how: *how, by: *by, line: l.no, sent: pos.sent });
                Ok(())
            }
            Cmd::Reveal { ev, to, by, interp, wrong, right, roles, cause, rel } => {
                let Some(m) = st.ev.get(ev).cloned() else {
                    return gate(Gate::Ref, format!("reveal {ev}: not in the event registry — a reveal does not create events, it is about an existing one"));
                };
                for o in to {
                    self.need_obs(st, *o)?;
                }
                if let Some(b) = by
                    && self.ev_in(st, *b).is_none()
                {
                    return gate(Gate::Ref, format!("reveal by={b}: no such event"));
                }
                for k in wrong.iter().chain(right) {
                    let s = self.need(*k, &[Reg::K])?.now();
                    if s.id(Field::About) != Some(*ev) {
                        return gate(Gate::Ref, format!("reveal {ev}: {k} — a claim about {}, not about {ev}", s.get(Field::About)));
                    }
                }
                // a revealed role: an entity from the event's world; an existing role is not overwritten
                let rec = self.ev_in(st, *ev).cloned().expect("event exists");
                let evw = st.frames.iter().enumerate().find(|(_, f)| f.world.as_deref().is_some_and(|w| w.event(*ev).is_some())).map(|(i, _)| i);
                for (r, x) in roles {
                    let has = self.ent(*x).is_some() || evw.is_some_and(|i| st.frames[i].world.as_deref().is_some_and(|w| w.ent(*x).is_some()));
                    if !has {
                        return gate(Gate::Ref, format!("reveal {ev} {}={x}: {x} not in the event's world", r.name()));
                    }
                    let before = rec.role(*r).map(|s| s.id).or_else(|| st.reveals.iter().filter(|v| v.ev == *ev).find_map(|v| v.roles.iter().find(|(rr, _)| rr == r).map(|(_, i)| *i)));
                    if let Some(old) = before
                        && old != *x
                    {
                        return gate(
                            Gate::Identity,
                            format!("reveal {ev} {}={x}: the event already has {}={old} — a reveal does not rewrite the past; another interpretation — claim", r.name(), r.name()),
                        );
                    }
                }
                if let Some(c) = cause {
                    let Some(cm) = st.ev.get(c) else { return gate(Gate::Ref, format!("reveal cause={c}: not in the event registry")) };
                    if cm.wid != m.wid {
                        return gate(Gate::Scope, format!("reveal {ev} cause={c}: different worlds"));
                    }
                    let (Some(ec), Some(se)) = (st.time.get(Pt::E(*c)), st.time.get(Pt::S(*ev))) else { return gate(Gate::Ref, "time axis") };
                    if let Err(back) = st.time.add(ec, se, false, Why::Reveal(l.no)) {
                        return gate(Gate::Time, format!("reveal {ev} cause={c}: cause later than effect — {}", st.time.explain(&back, &st.frames)));
                    }
                }
                if let Some((r, x)) = rel {
                    let Some(xm) = st.ev.get(x) else { return gate(Gate::Ref, format!("reveal: {x} not in the event registry")) };
                    if xm.wid != m.wid {
                        return gate(Gate::Scope, format!("reveal {ev} {} {x}: different worlds", r.name()));
                    }
                    relate(st, *ev, *r, *x, Why::Reveal(l.no))?;
                }
                for k in wrong {
                    self.claim_state(*k, tag(&CLAIM_STATUS, "wrong"), *by, l.no, pos, "reveal");
                }
                for k in right {
                    self.claim_state(*k, tag(&CLAIM_STATUS, "knows"), *by, l.no, pos, "reveal");
                }
                let meta = st.ev.get_mut(ev).expect("exists");
                if let Some(t) = interp {
                    meta.revealed.push(Revealed { what: format!("«{t}»"), sent: pos.sent, line: l.no });
                }
                for (r, x) in roles {
                    meta.revealed.push(Revealed { what: format!("{}={x}", r.name()), sent: pos.sent, line: l.no });
                }
                if let Some(c) = cause {
                    meta.revealed.push(Revealed { what: format!("cause={c}"), sent: pos.sent, line: l.no });
                }
                if let Some((r, x)) = rel {
                    meta.revealed.push(Revealed { what: format!("{} {x}", r.name()), sent: pos.sent, line: l.no });
                }
                st.reveals.push(Reveal {
                    ev: *ev,
                    to: to.clone(),
                    by: *by,
                    interp: interp.clone(),
                    wrong: wrong.clone(),
                    right: right.clone(),
                    roles: roles.clone(),
                    cause: *cause,
                    rel: *rel,
                    sent: pos.sent,
                    line: l.no,
                    frame: st.top(),
                });
                Ok(())
            }
            _ => unreachable!("exec_v3 only for v3 commands"),
        }
    }

    /// New claim state: status and the event that changed it.
    fn claim_state(&mut self, k: Id, status: Tag, by: Option<Id>, line: usize, pos: TextPos, cmd: &'static str) {
        let e = self.ents.get_mut(&k).expect("claim exists");
        let mut d = vec![Delta::Set(Field::Stage, Val::Known(V::Tag(status)))];
        if let Some(b) = by {
            d.push(Delta::Set(Field::By, Val::Known(V::Id(b))));
        }
        let ver = e.latest() + 1;
        e.states.push(State { ver, delta: d, cause: by, by: cmd, line, pos, note: None });
        if e.mentions.last().is_none_or(|m| *m < pos) {
            e.mentions.push(pos);
        }
    }

    /// Whether the event has already changed the canonical world: states with `cause=`, plans and beliefs closed by it,
    /// events that resulted from it.
    fn consequences(&self, st: &Story, ev: Id) -> Option<String> {
        let mut ws: Vec<&World> = vec![self];
        ws.extend(st.frames.iter().filter_map(|f| f.world.as_deref()));
        for w in ws {
            for e in w.ents.values() {
                if let Some(s) = e.states.iter().find(|s| s.cause == Some(ev) && s.by != "project") {
                    return Some(format!("{}@{} {} {}", e.id, s.ver, s.by, s.pos));
                }
            }
            if let Some(x) = w.events().iter().find(|x| x.x.cause == Some(ev) && x.happened()) {
                return Some(format!("{} cause={ev}", x.id));
            }
        }
        None
    }

    fn set_branch_everywhere(&mut self, st: &mut Story, ev: Id, b: Tag) {
        for e in self.events.iter_mut().filter(|e| e.id == ev) {
            e.branch = b;
        }
        for f in st.frames.iter_mut() {
            if let Some(w) = f.world.as_deref_mut() {
                for e in w.events.iter_mut().filter(|e| e.id == ev) {
                    e.branch = b;
                }
            }
        }
    }

    /// Projection into the past: identity and structure of entities without place and transient states; events — the same.
    pub(crate) fn project_past(&self) -> World {
        let mut w = World {
            text: self.text.clone(),
            ents: BTreeMap::new(),
            events: self.events.clone(),
            cursor: Vec::new(),
            pos: self.pos,
            v3: self.v3,
            wid: self.wid,
            branch: self.branch,
            story: None,
        };
        for e in self.ents.values() {
            w.ents.insert(e.id, self.projected(e));
        }
        w
    }

    fn projected(&self, e: &Entity) -> Entity {
        let s = e.now();
        let mut delta = Vec::new();
        for (f, v) in s.fields() {
            if !DYNAMIC.contains(&f) {
                delta.push(Delta::Set(f, Val::Known(v.clone())));
            }
        }
        for (sf, set) in s.sets() {
            if !PLACE_SETS.contains(&sf) {
                for i in set {
                    delta.push(Delta::Add(sf, *i));
                }
            }
        }
        Entity { id: e.id, states: vec![State { ver: 1, delta, cause: None, by: "project", line: 0, pos: self.pos, note: None }], mentions: Vec::new() }
    }

    /// What the frame world lacks from this world (for continuing an interrupted past frame).
    fn project_missing(&self, child: &World) -> (Vec<Entity>, Vec<Event>) {
        let ents = self.ents.values().filter(|e| child.ent(e.id).is_none()).map(|e| self.projected(e)).collect();
        let evs = self.events.iter().filter(|e| child.event(e.id).is_none()).cloned().collect();
        (ents, evs)
    }

    fn absorb(&mut self, (ents, evs): (Vec<Entity>, Vec<Event>)) {
        for e in ents {
            self.ents.insert(e.id, e);
        }
        self.events.extend(evs);
    }

    /// Closing a frame: what passes into the parent's world.
    /// - The frame's events — into the parent's registry (story within a story and branches — as foreign: they can be
    ///   talked about, but they are not a cause of changes in the parent's world).
    /// - Past frame: new entities — with identity, structure and stable fields (life, status, traits);
    ///   for existing ones — only what the parent does not know (the past does not rewrite the present); transient
    ///   states do not pass. Recreation — by change-language commands through the parent's gates; a rejection is a
    ///   warning, not a stop.
    fn merge_from(&mut self, child: &World, kind: FWorld, pos: TextPos, no: usize, fname: &str) -> Vec<String> {
        let mut warns = Vec::new();
        for e in child.events() {
            if self.event(e.id).is_none() {
                self.events.push(e.clone());
            }
        }
        if kind != FWorld::Past {
            return warns;
        }
        let fresh: BTreeSet<Id> = child.ents.keys().copied().filter(|i| self.ent(*i).is_none() && !matches!(i.reg, Reg::K | Reg::F | Reg::E)).collect();
        let mut cmds: Vec<Cmd> = Vec::new();
        for id in creation_order(child, &fresh) {
            cmds.extend(self.recreate(child, id, &fresh));
        }
        // existing: what the parent does not know — from the past
        for (id, e) in &child.ents {
            if fresh.contains(id) || !matches!(id.reg, Reg::C | Reg::O | Reg::L) {
                continue;
            }
            let (cs, ps) = (e.now(), self.now(*id).unwrap_or_default());
            let mut attrs = Vec::new();
            for (f, v) in cs.fields() {
                if TRANSIENT.contains(&f) || matches!(f, Field::Type | Field::Class | Field::At | Field::Parent | Field::PartOf) || !key_ok(id.reg, f) {
                    continue;
                }
                if ps.get(f) == Val::Unknown {
                    attrs.push((f, v.clone()));
                }
            }
            if !attrs.is_empty() {
                cmds.push(Cmd::Set { who: *id, attrs, cause: None });
            }
            if matches!(id.reg, Reg::C | Reg::O)
                && ps.id(Field::At).is_none()
                && ps.id(Field::PartOf).is_none()
                && let Some(to) = cs.id(Field::At)
                && self.ent(to).is_some()
            {
                cmds.push(Cmd::Move { who: *id, to, cause: None });
            }
        }
        for cmd in cmds {
            let name = cmd.name();
            let line = Line { no, raw: format!("(closing {fname}) {name}"), cmd, anchors: vec![pos.sent] };
            let saved = self.clone();
            let r = self.exec(&line, pos).and_then(|_| self.check_links());
            if let Err(e) = r {
                warns.push(format!("closing {fname}: {} — {e}", describe(&line.cmd)));
                *self = saved;
            }
        }
        warns
    }

    /// Commands that recreate a new entity of a past frame in the parent's world.
    fn recreate(&self, child: &World, id: Id, fresh: &BTreeSet<Id>) -> Vec<Cmd> {
        let s = child.now(id).unwrap_or_default();
        let exists = |x: Id| self.ent(x).is_some() || fresh.contains(&x);
        let word = |f: Field| match s.get(f) {
            Val::Known(V::Words(w)) => w.first().cloned(),
            _ => None,
        };
        let text = |f: Field| match s.get(f) {
            Val::Known(V::Text(t)) => Some(t),
            _ => None,
        };
        let ty = match s.get(Field::Type) {
            Val::Known(V::Tag(t)) => Some(t),
            _ => None,
        };
        let mut out = Vec::new();
        let keep = |f: Field| !TRANSIENT.contains(&f) && !matches!(f, Field::Type | Field::Class | Field::Name | Field::At | Field::Parent | Field::PartOf | Field::Num);
        let attrs: Vec<(Field, V)> = s
            .fields()
            .filter(|(f, v)| keep(*f) && key_ok(id.reg, *f) && !matches!(v, V::Id(x) if !exists(*x)))
            .map(|(f, v)| (f, v.clone()))
            .collect();
        match id.reg {
            Reg::L => {
                let Some(ty) = ty else { return out };
                out.push(Cmd::Loc { id, ty, class: word(Field::Class), name: text(Field::Name), parent: s.id(Field::Parent).filter(|p| exists(*p)), here: false });
                if !attrs.is_empty() {
                    out.push(Cmd::Set { who: id, attrs, cause: None });
                }
            }
            Reg::C => {
                let Some(ty) = ty else { return out };
                out.push(Cmd::Char { id, ty, class: word(Field::Class), name: text(Field::Name), at: None, attrs });
                if let Val::Known(n) = s.get(Field::Num) {
                    out.push(Cmd::Set { who: id, attrs: vec![(Field::Num, n)], cause: None });
                }
                if let Some(to) = s.id(Field::At).filter(|x| exists(*x) && x.reg == Reg::L) {
                    out.push(Cmd::Move { who: id, to, cause: None });
                }
            }
            Reg::O => {
                let Some(ty) = ty else { return out };
                let part = s.id(Field::PartOf).filter(|x| exists(*x));
                let num = match s.get(Field::Num) {
                    Val::Known(v) => Some(v),
                    _ => None,
                };
                out.push(Cmd::Obj { id, ty, class: word(Field::Class), name: text(Field::Name), at: None, part, num });
                if !attrs.is_empty() {
                    out.push(Cmd::Set { who: id, attrs, cause: None });
                }
                if part.is_none()
                    && let Some(to) = s.id(Field::At).filter(|x| exists(*x))
                {
                    out.push(Cmd::Move { who: id, to, cause: None });
                }
            }
            Reg::R => {
                let (Some(a), Some(b)) = (s.id(Field::A), s.id(Field::B)) else { return out };
                let Val::Known(V::Tag(k)) = s.get(Field::Kind) else { return out };
                if exists(a) && exists(b) {
                    out.push(Cmd::Rel { id, kind: Some(k), a: Some(a), b: Some(b), cause: None, fact: None });
                    for f in s.ids(SetF::Facts) {
                        if self.event(f).is_some() || child.event(f).is_some() {
                            out.push(Cmd::Rel { id, kind: None, a: None, b: None, cause: None, fact: Some(f) });
                        }
                    }
                }
            }
            Reg::P => {
                let Val::Known(V::Tag(k)) = s.get(Field::Kind) else { return out };
                let who: Vec<Id> = s.ids(SetF::Owners).into_iter().filter(|c| exists(*c)).collect();
                if who.is_empty() {
                    return out;
                }
                out.push(Cmd::Plan {
                    id,
                    kind: Some(k),
                    who,
                    to: s.id(Field::Target).filter(|x| exists(*x)),
                    what: text(Field::What),
                    src: s.id(Field::Src),
                    status: None,
                    by: None,
                });
                if let Val::Known(V::Tag(stage)) = s.get(Field::Stage)
                    && stage.as_str() != "open"
                {
                    out.push(Cmd::Plan { id, kind: None, who: Vec::new(), to: None, what: None, src: None, status: Some(stage), by: s.id(Field::By) });
                }
            }
            Reg::B => {
                let Some(who) = s.ids(SetF::Owners).into_iter().find(|c| exists(*c)) else { return out };
                let truth = match s.get(Field::Truth) {
                    Val::Known(V::Tag(t)) => Some(t.as_str() == "true"),
                    _ => None,
                };
                out.push(Cmd::Believe { id, who: Some(who), claim: text(Field::What), about: s.id(Field::About), from: s.id(Field::Src), truth, status: None, by: None });
                if let Val::Known(V::Tag(stage)) = s.get(Field::Stage)
                    && stage.as_str() == "dropped"
                {
                    out.push(Cmd::Believe { id, who: None, claim: None, about: None, from: None, truth: None, status: Some(stage), by: s.id(Field::By) });
                }
            }
            _ => {}
        }
        out
    }
}

fn describe(c: &Cmd) -> String {
    match c {
        Cmd::Loc { id, .. } | Cmd::Obj { id, .. } | Cmd::Char { id, .. } | Cmd::Rel { id, .. } | Cmd::Plan { id, .. } | Cmd::Believe { id, .. } => format!("{} {id}", c.name()),
        Cmd::Set { who, .. } | Cmd::Move { who, .. } => format!("{} {who}", c.name()),
        _ => c.name().to_string(),
    }
}

/// Relation of two events on the story axis (edges between endpoints).
pub(crate) fn relate(st: &mut Story, a: Id, r: TRel, b: Id, why: Why) -> Result<(), GateErr> {
    let p = |st: &Story, x: Pt| st.time.get(x).expect("event nodes exist");
    let (sa, ea, sb, eb) = (p(st, Pt::S(a)), p(st, Pt::E(a)), p(st, Pt::S(b)), p(st, Pt::E(b)));
    let edges: Vec<(usize, usize, bool)> = match r {
        TRel::Before => vec![(ea, sb, true)],
        TRel::After => vec![(eb, sa, true)],
        TRel::During => vec![(sb, sa, false), (ea, eb, false)],
        TRel::Same => vec![(sa, eb, false), (sb, ea, false)],
    };
    let saved = st.time.clone();
    for (u, v, s) in edges {
        if let Err(back) = st.time.add(u, v, s, why.clone()) {
            let msg = format!("{a} {} {b}: contradiction in the partial order — already {}", r.name(), st.time.explain(&back, &st.frames));
            st.time = saved;
            return gate(Gate::Time, msg);
        }
    }
    Ok(())
}

/// Remove an event from the narrative line (it became unreal): drop its line and frame edges, stitch the neighbours.
fn unchain(st: &mut Story, ev: Id) {
    let Some(m) = st.ev.get(&ev).cloned() else { return };
    if !m.main {
        return;
    }
    let line = st.frames[m.frame].line;
    let (Some(s), Some(e)) = (st.time.get(Pt::S(ev)), st.time.get(Pt::E(ev))) else { return };
    st.time.kill(|x| (x.u == s || x.v == s || x.u == e || x.v == e) && matches!(x.why, Why::Chain(_) | Why::InFrame(_)));
    let main = &mut st.frames[line].main;
    if let Some(k) = main.iter().position(|x| *x == ev) {
        main.remove(k);
        if k > 0 && k < main.len() {
            let (prev, next) = (main[k - 1], main[k]);
            if let (Some(pe), Some(ns)) = (st.time.get(Pt::E(prev)), st.time.get(Pt::S(next))) {
                let _ = st.time.add(pe, ns, true, Why::Chain(line));
            }
        }
    }
    st.ev.get_mut(&ev).expect("exists").main = false;
}

/// Registering a new event on the story time axis: nodes, narrative line or explicit placement, bounds of a past
/// frame, cause (soft: a contradiction is a warning, because v2 did not check this).
pub(crate) fn register(st: &mut Story, fi: usize, ev: &Event, line: usize) -> Result<(), GateErr> {
    let wid = ev.world;
    let s = st.time.node(Pt::S(ev.id), wid);
    let e = st.time.node(Pt::E(ev.id), wid);
    let _ = st.time.add(s, e, false, Why::Span);
    let placed = ev.x.placement();
    let sent = ev.anchors.iter().max().copied().unwrap_or(ev.pos.sent);
    let mut meta = EvMeta {
        frame: fi,
        wid,
        branch: ev.branch,
        branch_log: vec![(ev.branch, line, sent, None)],
        main: false,
        placed,
        sent,
        line,
        revealed: Vec::new(),
        abs: Vec::new(),
    };
    let happened = ev.happened();
    if let Some((r, x)) = placed {
        if st.ev.get(&x).is_some_and(|xm| xm.wid == wid) {
            relate(st, ev.id, r, x, Why::Place(line))?;
        }
    } else if happened {
        let lf = st.frames[fi].line;
        if let Some(prev) = st.frames[lf].main.last().copied()
            && let (Some(pe), Some(sn)) = (st.time.get(Pt::E(prev)), Some(s))
            && let Err(back) = st.time.add(pe, sn, true, Why::Chain(lf))
        {
            return gate(Gate::Time, format!("{} on the line after {prev}: contradiction — {}", ev.id, st.time.explain(&back, &st.frames)));
        }
        st.frames[lf].main.push(ev.id);
        meta.main = true;
        // causes of state changes recorded before this line event are not later than it
        for (c, ln) in st.pending.remove(&lf).unwrap_or_default() {
            if let Some(ce) = st.time.get(Pt::E(c))
                && st.time.wid[ce] == wid
                && st.time.add(ce, s, false, Why::Cause(ln)).is_err()
            {
                st.warnings.push(format!("line {ln}: effect via {c} recorded on the line before {}, but on the story axis {c} is later — edge not added", ev.id));
            }
        }
    }
    // bounds of a past frame: the event is inside the frame
    let lf = st.frames[fi].line;
    if happened && st.frames[lf].kind == FWorld::Past {
        let fs = st.time.node(Pt::FS(lf), wid);
        let fe = st.time.node(Pt::FE(lf), wid);
        for (u, v) in [(fs, s), (e, fe)] {
            if let Err(back) = st.time.add(u, v, false, Why::InFrame(lf)) {
                return gate(Gate::Time, format!("{} in frame {}: contradiction — {}", ev.id, st.frames[lf].name(), st.time.explain(&back, &st.frames)));
            }
        }
    }
    // cause earlier than effect — soft
    if happened
        && let Some(c) = ev.x.cause
        && let Some(ce) = st.time.get(Pt::E(c))
        && st.time.wid[ce] == wid
        && st.time.add(ce, s, false, Why::Cause(line)).is_err()
    {
        st.warnings.push(format!("line {line}: {} cause={c} — on the story axis the cause is not earlier (edge not added)", ev.id));
    }
    st.ev.insert(ev.id, meta);
    Ok(())
}

/// Cause of a state change (not an event): `cause=` for changes, `by=` for closing plans and beliefs.
fn state_cause(c: &Cmd) -> Option<Id> {
    match c {
        Cmd::Move { cause, .. }
        | Cmd::Have { cause, .. }
        | Cmd::Give { cause, .. }
        | Cmd::Set { cause, .. }
        | Cmd::Rel { cause, .. }
        | Cmd::Done { cause, .. }
        | Cmd::Unset { cause, .. }
        | Cmd::Feel { cause, .. }
        | Cmd::Detach { cause, .. }
        | Cmd::Attach { cause, .. } => *cause,
        Cmd::Plan { by, .. } | Cmd::Believe { by, .. } => *by,
        _ => None,
    }
}

/// Cause not later than effect: a state change is recorded on the frame's line when the narration reaches it,
/// so its off-line cause (placed or with unknown time) happened earlier than everything the line tells
/// next. Edge E(cause) ≤ S(next line event) — when that event appears (`register`). Soft:
/// a contradiction is a warning, the edge is not added (same as for event causes).
fn causal_stamp(st: &mut Story, fi: usize, cmd: &Cmd, line: usize) {
    let Some(c) = state_cause(cmd) else { return };
    let Some(m) = st.ev.get(&c) else { return };
    if m.main || m.branch.as_str() != "real" {
        return;
    }
    let lf = st.frames[fi].line;
    let v = st.pending.entry(lf).or_default();
    if !v.iter().any(|(x, _)| *x == c) {
        v.push((c, line));
    }
}
