//! Queries over state and history, without the LLM: state of X at sentence N, where X is, who has O, difference `X@a`–`X@b`,
//! mentions, field history (walking the chain of states), event chronology, description resolution (`char:fox`),
//! working memory (view from the cursor: main and secondary characters, the speaker, what is happening and why).

use std::fmt::Write as _;

use crate::lang::Role;
use crate::types::*;
use crate::world::{Delta, Event, Snap, State, World};

/// Working memory at sentence N (, `Context`): values with links into the graph.
pub struct Context {
    pub main: Vec<(Id, usize)>,
    pub secondary: Vec<(Id, usize)>,
    pub speaker: Option<(Id, Id)>,
    pub addressee: Option<Id>,
    pub what: Option<Id>,
    pub why: Option<(Id, String)>,
    pub place: Option<Id>,
    pub time: Option<String>,
}

impl World {
    fn text_of(s: &Snap, f: Field) -> Option<String> {
        match s.get(f) {
            Val::Known(V::Text(t)) => Some(t),
            Val::Known(V::Words(w)) => Some(w.join(",")),
            Val::Known(V::Tag(t)) => Some(t.to_string()),
            _ => None,
        }
    }

    /// Human-readable label: `C1 Crow`, `L2 tree`, `O1 meat`, `E5 caw`, `R1 C1 Lion — C2 Mouse`.
    pub fn label(&self, id: Id) -> String {
        if id.reg == Reg::E {
            return match self.event(id) {
                Some(e) => match &e.speech {
                    Some(s) => format!("{id} say:{}", s.acts_str()),
                    None => format!("{id} {}", e.verb),
                },
                None => id.to_string(),
            };
        }
        let Some(s) = self.now(id) else { return id.to_string() };
        if id.reg == Reg::R {
            let end = |f| s.id(f).map(|c| self.label(c)).unwrap_or_else(|| "?".into());
            return format!("{id} {} — {}", end(Field::A), end(Field::B));
        }
        let name = Self::text_of(&s, Field::Name).or_else(|| Self::text_of(&s, Field::Class)).or_else(|| Self::text_of(&s, Field::Type));
        match name {
            Some(n) => format!("{id} {n}"),
            None => id.to_string(),
        }
    }

    /// Type/class: `animal/crow`.
    pub fn kind_of(&self, id: Id) -> String {
        let Some(s) = self.now(id) else { return String::new() };
        match (Self::text_of(&s, Field::Type), Self::text_of(&s, Field::Class)) {
            (Some(t), Some(c)) => format!("{t}/{c}"),
            (Some(t), None) => t,
            (None, Some(c)) => c,
            (None, None) => String::new(),
        }
    }

    /// Location with ancestors: `L2 tree ⊂ L1 forest`.
    pub fn place_chain(&self, l: Id) -> String {
        let mut out = self.label(l);
        for p in self.ancestors(l) {
            let _ = write!(out, " ⊂ {}", self.label(p));
        }
        out
    }

    /// State of an entity at the end of sentence N: version and snapshot.
    pub fn state_at(&self, id: Id, sent: u16) -> Option<(u32, Snap)> {
        let e = self.ent(id)?;
        let v = e.ver_at(sent)?;
        Some((v, e.snap(v)))
    }

    /// Field history: the states that changed it (like `Attr<T>` — walking the chain of states).
    pub fn history(&self, id: Id, f: Field) -> Vec<(&State, Val)> {
        let Some(e) = self.ent(id) else { return Vec::new() };
        let mut out = Vec::new();
        for st in e.states() {
            for d in &st.delta {
                if let Delta::Set(k, v) = d
                    && *k == f
                {
                    out.push((st, v.clone()));
                }
            }
        }
        out
    }

    /// Where the entity is at sentence N (human-readable, with the anchor of the place change).
    pub fn where_at(&self, id: Id, sent: u16) -> Option<String> {
        self.where_ver(id, self.ent(id)?.ver_at(sent)?)
    }

    /// Where the entity is in state `id@v`.
    pub fn where_ver(&self, id: Id, v: u32) -> Option<String> {
        let e = self.ent(id)?;
        let at = e.snap(v).id(Field::At)?;
        let st = e.set_by(Field::At, v);
        let since = st.map(|s| format!("since {} ({}{})", s.pos, s.by, s.cause.map(|c| format!(" because {}", self.label(c))).unwrap_or_default())).unwrap_or_default();
        let sent = e.state(v)?.pos.sent;
        let place = if at.reg == Reg::C {
            let holder_place = self.state_at(at, sent).and_then(|(_, s)| s.id(Field::At)).map(|l| format!(", which is in {}", self.place_chain(l))).unwrap_or_default();
            format!("in {}{holder_place}", self.label(at))
        } else {
            format!("in {}", self.place_chain(at))
        };
        Some(format!("{}@{v} {place} {since}", self.label(id)))
    }

    /// Who or what holds the object at sentence N: the owner (character or place) and the state where it happened.
    pub fn holder_at(&self, o: Id, sent: u16) -> Option<(Id, u32, &State)> {
        let e = self.ent(o)?;
        let v = e.ver_at(sent)?;
        let h = e.snap(v).id(Field::At)?;
        Some((h, v, e.set_by(Field::At, v)?))
    }

    /// Difference of two states of one entity: fields, sets and causes between them.
    pub fn diff(&self, id: Id, a: u32, b: u32) -> Option<String> {
        let e = self.ent(id)?;
        if a == 0 || b == 0 || a > e.latest() || b > e.latest() {
            return None;
        }
        let (sa, sb) = (e.snap(a), e.snap(b));
        let mut out = format!("{id}@{a} → {id}@{b}:");
        let mut keys: Vec<Field> = sa.fields().map(|(k, _)| k).chain(sb.fields().map(|(k, _)| k)).collect();
        keys.sort();
        keys.dedup();
        for k in keys {
            let (x, y) = (sa.get(k), sb.get(k));
            if x != y {
                let _ = write!(out, " {}: {x} → {y};", k.name());
            }
        }
        let mut sets: Vec<SetF> = sa.sets().map(|(k, _)| k).chain(sb.sets().map(|(k, _)| k)).collect();
        sets.sort();
        sets.dedup();
        for s in sets {
            let (x, y) = (sa.ids(s), sb.ids(s));
            let add: Vec<String> = y.iter().filter(|i| !x.contains(i)).map(|i| format!("+{i}")).collect();
            let del: Vec<String> = x.iter().filter(|i| !y.contains(i)).map(|i| format!("−{i}")).collect();
            if !add.is_empty() || !del.is_empty() {
                let _ = write!(out, " {}: {};", s.name(), add.into_iter().chain(del).collect::<Vec<_>>().join(" "));
            }
        }
        let (lo, hi) = if a < b { (a, b) } else { (b, a) };
        let steps: Vec<String> = e
            .states()
            .iter()
            .filter(|s| s.ver > lo && s.ver <= hi)
            .map(|s| format!("@{} {} {}{}", s.ver, s.pos, s.by, s.cause.map(|c| format!(" because {}", self.label(c))).unwrap_or_default()))
            .collect();
        let _ = write!(out, " steps: {}", if steps.is_empty() { "—".into() } else { steps.join(", ") });
        Some(out)
    }

    /// Resolve a description: `C1`, `char:fox`, `obj:meat|flesh`, `loc:tree` — by class, name words, type.
    pub fn resolve(&self, d: &str) -> Vec<Id> {
        if let Some(i) = Id::parse(d) {
            return if self.ent(i).is_some() || self.event(i).is_some() { vec![i] } else { Vec::new() };
        }
        let Some((r, alts)) = d.split_once(':') else { return Vec::new() };
        let reg = match r {
            "char" => Reg::C,
            "obj" => Reg::O,
            "loc" => Reg::L,
            _ => return Vec::new(),
        };
        let alts: Vec<String> = alts.split('|').map(|a| a.to_lowercase()).collect();
        let hit = |w: &str| alts.iter().any(|a| a == w || format!("{a}s") == w || format!("{w}s") == *a);
        self.ents(reg)
            .filter(|e| {
                let s = e.now();
                let mut words: Vec<String> = Vec::new();
                for f in [Field::Class, Field::Name, Field::Type] {
                    if let Some(t) = Self::text_of(&s, f) {
                        words.extend(t.to_lowercase().split(|c: char| !c.is_alphanumeric() && c != '_').filter(|w| !w.is_empty()).map(str::to_string));
                    }
                }
                words.iter().any(|w| hit(w))
            })
            .map(|e| e.id)
            .collect()
    }

    /// Attention weight up to sentence N: how many times the character was mentioned.
    pub fn attention(&self, upto: u16) -> Vec<(Id, usize)> {
        let mut v: Vec<(Id, usize)> = self.ents(Reg::C).map(|e| (e.id, e.mentions().iter().filter(|p| p.sent <= upto).count())).filter(|x| x.1 > 0).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v
    }

    /// Working memory at sentence N: main characters (attention ≥ half of the maximum) and secondary ones, the last speaker
    /// and addressee, the last event and the last motive, the cursor's place and time.
    pub fn context_at(&self, sent: u16) -> Context {
        let att = self.attention(sent);
        let top = att.first().map(|x| x.1).unwrap_or(0);
        let (main, secondary) = att.into_iter().partition(|x| x.1 * 2 >= top);
        let upto: Vec<&Event> = self.events().iter().filter(|e| e.pos.sent <= sent).collect();
        let last_say = upto.iter().rev().find(|e| e.speech.as_ref().is_some_and(|s| !s.narrator));
        let speaker = last_say.and_then(|e| e.role(Role::Agent).map(|a| (a.id, e.id)));
        let addressee = last_say.and_then(|e| e.role(Role::To).map(|a| a.id));
        let what = upto.last().map(|e| e.id);
        let why = upto.iter().rev().find_map(|e| e.why.clone().map(|w| (e.id, w)));
        let cur = self.cursor().iter().rev().find(|c| c.pos.sent <= sent);
        Context { main, secondary, speaker, addressee, what, why, place: cur.and_then(|c| c.at), time: cur.and_then(|c| c.time.clone()) }
    }

    /// Event participants, human-readable: `agent C1@2 Crow, patient O1@1 meat`.
    pub fn roles_str(&self, e: &Event) -> String {
        if e.roles.is_empty() {
            return if e.speech.as_ref().is_some_and(|s| s.narrator) { "narrator".into() } else { "—".into() };
        }
        e.roles
            .iter()
            .map(|(r, s)| {
                let name = self.label(s.id);
                let name = name.strip_prefix(&s.id.to_string()).unwrap_or(&name).trim().to_string();
                format!("{} {s} {name}", r.name())
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
