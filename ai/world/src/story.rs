//! Narration layer v3 (, sections "Non-linear narrative", "Hierarchical and global worlds"): the world
//! separates what happened (canonical events with identity), what is thought about it (observer claims and
//! knowledge) and in what order it is told (the text cursor versus the story time axis).
//!
//! - **The story time axis is a partial order**, not a label: each event has two ends (start and end),
//!   relations BEFORE / AFTER / DURING / SAME (≈) are edges `<` or `≤` between ends (point algebra,
//!   the tractable fragment of Allen's interval algebra, as in TimeML). A contradiction — a cycle with at least one strict
//!   edge — is a time-gate rejection. Absolute time comes only from the text.
//! - **Narration frames are a stack**: who tells, to whom, level, own cursor and own world. A past frame
//!   (testimony, recollection) is a projection of the parent world without dynamic fields (place, feelings…): the gates in it
//!   check a hero's neighbouring states on the story axis inside the frame, not neighbouring sentences. A `world=new` frame
//!   is a made-up story within the story: its own world and its own time axis.
//! - **Narration line** of a frame: events told without explicit placement go in sequence (the linear special
//!   case is v1/v2). An event with `before=`/`after=`/`during=`/`same=` does not join the line.
//! - **Reality branches**, **observer knowledge**, **reveals** live here, while entity states live in the worlds.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::lang::{Obs, Role, TRel};
use crate::types::*;
use crate::world::World;

// ── Story time axis ───────────────────────────────────────────────────────────────────────────────

/// A point of the story axis: start or end of an event, frame bound.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Pt {
    S(Id),
    E(Id),
    /// frame start (frame index)
    FS(usize),
    /// frame end
    FE(usize),
}

/// Where the edge comes from (transparency: every «before» can be traced to its cause).
#[derive(Clone, Debug, PartialEq)]
pub enum Why {
    /// an event has duration: start ≤ end
    Span,
    /// frame narration line: told in sequence — happened in sequence
    Chain(usize),
    /// explicit placement on creation (`before=` etc.), line
    Place(usize),
    /// `time` command, line
    Time(usize),
    /// `reveal`, line
    Reveal(usize),
    /// cause before effect (`cause=`), line
    Cause(usize),
    /// past frame: everything in it is before the parent's event
    FrameBound(usize),
    /// event inside a past frame
    InFrame(usize),
}

impl Why {
    pub fn show(&self, frames: &[Frame]) -> String {
        match self {
            Why::Span => "duration".into(),
            Why::Chain(f) => format!("line of frame {}", frames.get(*f).map(|x| x.name()).unwrap_or_default()),
            Why::Place(l) => format!("placement, line {l}"),
            Why::Time(l) => format!("time, line {l}"),
            Why::Reveal(l) => format!("reveal, line {l}"),
            Why::Cause(l) => format!("cause=, line {l}"),
            Why::FrameBound(f) => format!("frame {} — past", frames.get(*f).map(|x| x.name()).unwrap_or_default()),
            Why::InFrame(f) => format!("in frame {}", frames.get(*f).map(|x| x.name()).unwrap_or_default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TEdge {
    pub u: usize,
    pub v: usize,
    pub strict: bool,
    pub why: Why,
    pub alive: bool,
}

/// Story time graph: nodes are points, edges `u < v` (strict) or `u ≤ v`.
#[derive(Clone, Debug, Default)]
pub struct TimeGraph {
    idx: HashMap<Pt, usize>,
    pub pts: Vec<Pt>,
    /// world of the node (different worlds have different axes)
    pub wid: Vec<u32>,
    pub edges: Vec<TEdge>,
    out: Vec<Vec<usize>>,
}

/// Relation of two events on the story axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Order {
    Before,
    /// end of A ≤ start of B (non-strict: «not later»)
    NotAfter,
    After,
    NotBefore,
    During,
    Contains,
    /// ≈ simultaneous (intervals overlap)
    Overlap,
    Unknown,
    /// different worlds — different time axes
    OtherWorld,
}

impl Order {
    pub fn uk(self) -> &'static str {
        match self {
            Order::Before => "before",
            Order::NotAfter => "not later than",
            Order::After => "after",
            Order::NotBefore => "not earlier than",
            Order::During => "during",
            Order::Contains => "contains",
            Order::Overlap => "≈ simultaneous with",
            Order::Unknown => "unknown relative to",
            Order::OtherWorld => "in another world than",
        }
    }
}

impl TimeGraph {
    pub fn node(&mut self, p: Pt, wid: u32) -> usize {
        if let Some(i) = self.idx.get(&p) {
            return *i;
        }
        let i = self.pts.len();
        self.idx.insert(p, i);
        self.pts.push(p);
        self.wid.push(wid);
        self.out.push(Vec::new());
        i
    }

    pub fn get(&self, p: Pt) -> Option<usize> {
        self.idx.get(&p).copied()
    }

    /// Path u →* v: None — none; Some(true) — there is a path with a strict edge; Some(false) — only non-strict ones.
    pub fn reach(&self, u: usize, v: usize) -> Option<bool> {
        self.path(u, v).map(|(s, _)| s)
    }

    /// The path with the greatest strictness and its edges (for explanation).
    pub fn path(&self, u: usize, v: usize) -> Option<(bool, Vec<usize>)> {
        if u == v {
            return Some((false, Vec::new()));
        }
        // state: (node, whether a strict edge was already seen); predecessor for path reconstruction
        let n = self.pts.len();
        let mut prev: Vec<[Option<(usize, bool, usize)>; 2]> = vec![[None, None]; n];
        let mut seen = vec![[false, false]; n];
        let mut q = VecDeque::new();
        seen[u][0] = true;
        q.push_back((u, false));
        while let Some((x, s)) = q.pop_front() {
            for &ei in &self.out[x] {
                let e = &self.edges[ei];
                if !e.alive {
                    continue;
                }
                let ns = s || e.strict;
                let k = ns as usize;
                if !seen[e.v][k] {
                    seen[e.v][k] = true;
                    prev[e.v][k] = Some((x, s, ei));
                    q.push_back((e.v, ns));
                }
            }
        }
        let k = if seen[v][1] {
            1
        } else if seen[v][0] {
            0
        } else {
            return None;
        };
        let mut edges = Vec::new();
        let (mut x, mut s) = (v, k == 1);
        while let Some((p, ps, ei)) = prev[x][s as usize] {
            edges.push(ei);
            x = p;
            s = ps;
            if x == u && !s {
                break;
            }
        }
        edges.reverse();
        Some((k == 1, edges))
    }

    /// Add an edge with a check: a cycle with a strict edge is a contradiction (returns the path back).
    pub fn add(&mut self, u: usize, v: usize, strict: bool, why: Why) -> Result<(), Vec<usize>> {
        if u == v {
            return if strict { Err(Vec::new()) } else { Ok(()) };
        }
        if let Some((s, back)) = self.path(v, u)
            && (s || strict)
        {
            return Err(back);
        }
        let i = self.edges.len();
        self.edges.push(TEdge { u, v, strict, why, alive: true });
        self.out[u].push(i);
        Ok(())
    }

    /// Remove live edges touching a node, by a condition.
    pub fn kill(&mut self, f: impl Fn(&TEdge) -> bool) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, e) in self.edges.iter_mut().enumerate() {
            if e.alive && f(e) {
                e.alive = false;
                out.push(i);
            }
        }
        out
    }

    /// Relation of two events.
    pub fn order(&self, a: Id, b: Id) -> Order {
        let (Some(sa), Some(ea), Some(sb), Some(eb)) = (self.get(Pt::S(a)), self.get(Pt::E(a)), self.get(Pt::S(b)), self.get(Pt::E(b))) else {
            return Order::Unknown;
        };
        if self.wid[sa] != self.wid[sb] {
            return Order::OtherWorld;
        }
        match self.reach(ea, sb) {
            Some(true) => return Order::Before,
            Some(false) if self.reach(eb, sa).is_none() => return Order::NotAfter,
            _ => {}
        }
        match self.reach(eb, sa) {
            Some(true) => return Order::After,
            Some(false) => return Order::NotBefore,
            None => {}
        }
        let (d1, d2) = (self.reach(sb, sa).is_some(), self.reach(ea, eb).is_some());
        if d1 && d2 {
            return Order::During;
        }
        if self.reach(sa, sb).is_some() && self.reach(eb, ea).is_some() {
            return Order::Contains;
        }
        if self.reach(sa, eb).is_some() && self.reach(sb, ea).is_some() {
            return Order::Overlap;
        }
        Order::Unknown
    }

    /// Path explanation: causes of the edges.
    pub fn explain(&self, edges: &[usize], frames: &[Frame]) -> String {
        let mut parts: Vec<String> = Vec::new();
        for &ei in edges {
            let e = &self.edges[ei];
            if e.why == Why::Span {
                continue;
            }
            let p = |i: usize| match self.pts[i] {
                Pt::S(x) | Pt::E(x) => x.to_string(),
                Pt::FS(f) | Pt::FE(f) => frames.get(f).map(|x| x.name()).unwrap_or_default(),
            };
            parts.push(format!("{} {} {} ({})", p(e.u), if e.strict { "<" } else { "≤" }, p(e.v), e.why.show(frames)));
        }
        parts.dedup();
        parts.join("; ")
    }

    /// Closure for bulk queries: for each node — reachable (≤*) and strictly reachable (<*).
    pub fn closure(&self) -> Closure {
        let n = self.pts.len();
        let words = n.div_ceil(64).max(1);
        // order: reverse DFS finish (the graph may have non-strict cycles — equalities; iterate to a fixpoint)
        let mut any = vec![vec![0u64; words]; n];
        let mut strict = vec![vec![0u64; words]; n];
        let set = |v: &mut Vec<u64>, i: usize| v[i / 64] |= 1 << (i % 64);
        let mut changed = true;
        let mut rounds = 0;
        while changed && rounds < n + 2 {
            changed = false;
            rounds += 1;
            for u in (0..n).rev() {
                for &ei in &self.out[u] {
                    let e = &self.edges[ei];
                    if !e.alive {
                        continue;
                    }
                    let v = e.v;
                    let (mut na, mut ns) = (any[u].clone(), strict[u].clone());
                    set(&mut na, v);
                    for w in 0..words {
                        na[w] |= any[v][w];
                        ns[w] |= strict[v][w];
                        if e.strict {
                            ns[w] |= any[v][w];
                        }
                    }
                    if e.strict {
                        set(&mut ns, v);
                    }
                    if na != any[u] || ns != strict[u] {
                        any[u] = na;
                        strict[u] = ns;
                        changed = true;
                    }
                }
            }
        }
        Closure { any, strict }
    }
}

/// Closure of the time graph.
pub struct Closure {
    any: Vec<Vec<u64>>,
    strict: Vec<Vec<u64>>,
}

impl Closure {
    pub fn lt(&self, u: usize, v: usize) -> bool {
        self.strict[u][v / 64] >> (v % 64) & 1 == 1
    }

    pub fn le(&self, u: usize, v: usize) -> bool {
        u == v || self.any[u][v / 64] >> (v % 64) & 1 == 1
    }
}

// ── Narration frames ──────────────────────────────────────────────────────────────────────────────

/// World of a frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FWorld {
    /// root: the document world itself
    Root,
    /// the same world in the past (testimony, recollection): own world — a projection of the parent world
    Past,
    /// the same world and the same line (parallel line, another narrator)
    Now,
    /// a made-up story within the story: new world, own axis
    New,
    /// branch: dream, lie, plan, «what if» — a copy of the parent state, changes do not flow back
    Branch,
}

/// Frame: who tells, to whom, level, own cursor and own world.
#[derive(Clone, Debug)]
pub struct Frame {
    pub id: Option<Id>,
    pub narrator: Obs,
    pub to: Vec<Obs>,
    pub level: u8,
    pub parent: Option<usize>,
    pub kind: FWorld,
    /// event branch of the frame (`real` — the ordinary one)
    pub branch: Tag,
    /// world identity (time axis): `same` inherits from the parent
    pub wid: u32,
    /// the frame whose narration line the events continue (for `time=now` — the parent's line)
    pub line: usize,
    /// past frame bound: everything in it is before this parent event
    pub anchor: Option<Id>,
    pub src: Option<Id>,
    /// opening and closing (an interrupted frame can be resumed): (sentence, line)
    pub spans: Vec<(u16, Option<u16>)>,
    /// own world (for Past, New, Branch); for Root and Now — the parent's world
    pub world: Option<Box<World>>,
    /// narration line: events in sequence
    pub main: Vec<Id>,
}

impl Frame {
    pub fn name(&self) -> String {
        self.id.map(|i| i.to_string()).unwrap_or_else(|| "F·root".into())
    }

    pub fn is_open(&self) -> bool {
        self.spans.last().is_some_and(|s| s.1.is_none())
    }
}

// ── Events, knowledge, reveals ────────────────────────────────────────────────────────────────────

/// Metadata of a canonical event (v3): in which frame and world it is told, branch, on the line or placed explicitly.
#[derive(Clone, Debug)]
pub struct EvMeta {
    pub frame: usize,
    pub wid: u32,
    pub branch: Tag,
    /// branch history: (branch, line, sentence, by)
    pub branch_log: Vec<(Tag, usize, u16, Option<Id>)>,
    /// on the frame's narration line
    pub main: bool,
    pub placed: Option<(TRel, Id)>,
    pub sent: u16,
    pub line: usize,
    /// roles, causes, interpretations revealed later
    pub revealed: Vec<Revealed>,
    /// absolute time from the text: (text, sentence)
    pub abs: Vec<(String, u16)>,
}

#[derive(Clone, Debug)]
pub struct Revealed {
    pub what: String,
    pub sent: u16,
    pub line: usize,
}

/// Knowledge: who knows a fact since sentence N (explicit `know`).
#[derive(Clone, Debug)]
pub struct Know {
    pub who: Obs,
    pub fact: Id,
    pub from: u16,
    pub how: Option<Tag>,
    pub by: Option<Id>,
    pub line: usize,
    pub sent: u16,
}

/// Reveal: an event, what was revealed, to whom, by what; which claims became false or true.
#[derive(Clone, Debug)]
pub struct Reveal {
    pub ev: Id,
    pub to: Vec<Obs>,
    pub by: Option<Id>,
    pub interp: Option<String>,
    pub wrong: Vec<Id>,
    pub right: Vec<Id>,
    pub roles: Vec<(Role, Id)>,
    pub cause: Option<Id>,
    pub rel: Option<(TRel, Id)>,
    pub sent: u16,
    pub line: usize,
    pub frame: usize,
}

/// Narration layer of the root world.
#[derive(Clone, Debug)]
pub struct Story {
    pub frames: Vec<Frame>,
    /// open frames (indices), the root at the bottom
    pub stack: Vec<usize>,
    pub time: TimeGraph,
    pub ev: BTreeMap<Id, EvMeta>,
    pub knows: Vec<Know>,
    pub reveals: Vec<Reveal>,
    /// soft warnings (a cause edge that would contradict the order; frame synchronisation)
    pub warnings: Vec<String>,
    pub next_wid: u32,
    /// the root is already named (`frame open … level=0`)
    pub root_named: bool,
    /// causes of state changes off the line waiting for the next line event: (cause, line) per frame line
    pub pending: BTreeMap<usize, Vec<(Id, usize)>>,
}

impl Story {
    pub fn new() -> Story {
        let root = Frame {
            id: None,
            narrator: Obs::Narrator,
            to: vec![Obs::Reader],
            level: 0,
            parent: None,
            kind: FWorld::Root,
            branch: BRANCH.tag("real").expect("real"),
            wid: 0,
            line: 0,
            anchor: None,
            src: None,
            spans: vec![(1, None)],
            world: None,
            main: Vec::new(),
        };
        Story {
            frames: vec![root],
            stack: vec![0],
            time: TimeGraph::default(),
            ev: BTreeMap::new(),
            knows: Vec::new(),
            reveals: Vec::new(),
            warnings: Vec::new(),
            next_wid: 1,
            root_named: false,
            pending: BTreeMap::new(),
        }
    }

    pub fn top(&self) -> usize {
        *self.stack.last().expect("root is always on the stack")
    }

    pub fn frame_by_id(&self, id: Id) -> Option<usize> {
        self.frames.iter().position(|f| f.id == Some(id))
    }

    /// Frame level of an event.
    pub fn level_of(&self, e: Id) -> Option<u8> {
        self.ev.get(&e).map(|m| self.frames[m.frame].level)
    }

    /// Whether the event canonically happened (branch real).
    pub fn is_real(&self, e: Id) -> bool {
        self.ev.get(&e).is_none_or(|m| m.branch.as_str() == "real")
    }

    /// The deepest number of frame nesting levels.
    pub fn depth(&self) -> u8 {
        self.frames.iter().map(|f| f.level).max().unwrap_or(0) + 1
    }

    /// Narration level of a frame (Genette): only another narrator opens a new level. A recollection by the same
    /// narrator (Watson writing memoirs about the past) is a time frame, not a nested story: the level stays the same.
    pub fn narr_level(&self, fi: usize) -> u8 {
        let f = &self.frames[fi];
        match f.parent {
            None => 0,
            Some(p) => self.narr_level(p) + u8::from(self.frames[p].narrator != f.narrator),
        }
    }

    /// Number of narration levels (by narrators).
    pub fn narr_depth(&self) -> u8 {
        (0..self.frames.len()).map(|i| self.narr_level(i)).max().unwrap_or(0) + 1
    }

    /// Who knows a fact at sentence N: explicit `know`, reveals (to whom), the reader — from the telling anchor, frame
    /// listeners — from the telling anchor, the frame narrator — from its opening. Returns (sentence, how).
    pub fn known_since(&self, who: Obs, fact: Id, w: &World) -> Option<(u16, String)> {
        let mut best: Option<(u16, String)> = None;
        let mut take = |s: u16, how: String| {
            if best.as_ref().is_none_or(|b| s < b.0) {
                best = Some((s, how));
            }
        };
        for k in &self.knows {
            if k.who == who && k.fact == fact {
                take(k.from, format!("know ^s{} {}", k.sent, k.how.map(|h| h.to_string()).unwrap_or_default()).trim().to_string());
            }
        }
        for r in &self.reveals {
            if r.ev == fact && (r.to.contains(&who) || who == Obs::Reader) {
                take(r.sent, format!("reveal ^s{}{}", r.sent, r.by.map(|b| format!(" ({})", w.label_any(b))).unwrap_or_default()));
            }
        }
        if fact.reg == Reg::E
            && let Some(m) = self.ev.get(&fact)
        {
            let f = &self.frames[m.frame];
            if who == Obs::Reader {
                take(m.sent, format!("told ^s{} (frame {}, level {})", m.sent, f.name(), f.level));
            }
            if f.to.contains(&who) {
                take(m.sent, format!("listener of frame {} ^s{}", f.name(), m.sent));
            }
            if f.narrator == who {
                take(f.spans.first().map(|s| s.0).unwrap_or(m.sent), format!("narrator of frame {}", f.name()));
            }
        }
        best
    }
}

impl Default for Story {
    fn default() -> Self {
        Story::new()
    }
}

/// Fields unknown in a past frame: place and transient states (feelings, sleep, freedom, goal, status,
/// life). The rest is identity and structure (name, sex, parents, traits, parts, relations, intentions).
pub const DYNAMIC: &[Field] = &[Field::At, Field::Feeling, Field::Sleep, Field::Freedom, Field::Goal, Field::Status, Field::Life, Field::Belief];

/// Fields that do not carry over from the past into the present when a frame closes (they pass).
pub const TRANSIENT: &[Field] = &[Field::Feeling, Field::Sleep, Field::Freedom, Field::Goal, Field::Belief];

/// Sets derived from place (absent from the past projection).
pub const PLACE_SETS: &[SetF] = &[SetF::Chars, SetF::Objects, SetF::Items];

/// Order entities for reproduction in another world: locations by nesting, characters, objects
/// (the whole before the part), relations, intentions, beliefs.
pub fn creation_order(w: &World, ids: &BTreeSet<Id>) -> Vec<Id> {
    let mut out: Vec<Id> = Vec::new();
    let mut placed: BTreeSet<Id> = BTreeSet::new();
    for reg in [Reg::L, Reg::C, Reg::O, Reg::R, Reg::P, Reg::B] {
        let mut left: Vec<Id> = ids.iter().copied().filter(|i| i.reg == reg).collect();
        let mut guard = 0;
        while !left.is_empty() && guard < 64 {
            guard += 1;
            let mut rest = Vec::new();
            for i in left {
                let dep = w.now(i).and_then(|s| if reg == Reg::L { s.id(Field::Parent) } else if reg == Reg::O { s.id(Field::PartOf) } else { None });
                match dep {
                    Some(d) if ids.contains(&d) && !placed.contains(&d) && d.reg == reg => rest.push(i),
                    _ => {
                        placed.insert(i);
                        out.push(i);
                    }
                }
            }
            left = rest;
        }
        out.extend(left);
    }
    out
}
