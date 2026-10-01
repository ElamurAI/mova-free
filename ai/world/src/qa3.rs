//! v3 questions to the world of a non-linear narrative — answers from the state with anchors, no LLM. A question is
//! formalized in advance as a query (as in `qa.rs`); `a || b` — a second reading. Where the state is silent — "not in state".
//!
//! Story time:
//!   order <event> <event>                   which is earlier on the story axis (and how it is told), with explanation
//!   sort <event> <event> …                  order of events on the story axis (unknown — "?")
//!   prolepses [N]                           told earlier, happened later (flash-forwards)
//!   analepses [N]                           told later, happened earlier (flashbacks)
//!   abs <event>                             absolute time — only from the text
//! Knowledge:
//!   claims [who=<obs.>] [about=<event>] [at=^sN]  observers' claims and their status at ^sN
//!   truth <event>                           what really happened: event, frame, branch, revealed, claims
//!   revealed <event>                        when and how it was revealed
//!   knows <obs.> <event|K…> [at=^sN]        whether the observer knows about the event (since which sentence, from where)
//!   learns <obs.> <event> [at=^sN]          when the observer learned the truth (reveal, claim knows)
//!   branch <event>                          branch of reality
//! Frames:
//!   frames                                  all frames: level, narrator → listeners, world, sentences
//!   teller <N>                              who tells whom at level N
//!   levels                                  how many levels
//!   level <event>                           level and frame of the event
//! Event description: `E7` or `ev:<verbs separated by |>[/<entity description>][@sA-B]`; entities — as in `qa.rs`
//! (`char:julia`); observer — `reader`, `narrator` or an entity description.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use crate::lang::{Obs, Role};
use crate::narr::val_obs;
use crate::qa::Answer;
use crate::story::{FWorld, Order, Pt, Story, Why};
use crate::types::*;
use crate::world::{Event, World};

type R = Result<String, String>;

/// v3 question: testbed, id, kind, text, query, expected.
#[derive(Clone, Debug)]
pub struct Q3 {
    pub poly: String,
    pub id: String,
    pub kind: String,
    pub text: String,
    pub query: String,
    pub expect: String,
}

pub fn read_questions(text: &str) -> Result<Vec<Q3>, String> {
    let mut out = Vec::new();
    for (i, l) in text.lines().enumerate() {
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 6 {
            return Err(format!("v3 questions, line {}: {} columns instead of 6", i + 1, c.len()));
        }
        out.push(Q3 { poly: c[0].into(), id: c[1].into(), kind: c[2].into(), text: c[3].into(), query: c[4].into(), expect: c[5].into() });
    }
    Ok(out)
}

pub fn answer(w: &World, query: &str) -> Answer {
    let mut why_not = Vec::new();
    for q in query.split("||") {
        match run1(w, q.trim()) {
            Ok(t) => return Answer { text: t, from_state: true },
            Err(e) => why_not.push(e),
        }
    }
    Answer { text: format!("not in state: {}", why_not.join("; ")), from_state: false }
}

/// Scoring as in `qa::verdict` (groups separated by comma — all; steps separated by `>` — in order; alternatives by `|`),
/// plus a step `@sA-B` — anchor ^sN within A…B. Expected `none` — the correct answer is "not in state".
pub fn verdict(a: &Answer, expect: &str) -> crate::qa::Verdict {
    use crate::qa::Verdict;
    if expect.trim() == "none" {
        return if a.from_state { Verdict::Miss } else { Verdict::Hit };
    }
    if !a.from_state {
        return Verdict::NotInState;
    }
    let t = a.text.to_lowercase();
    let anchor_after = |from: usize, lo: u32, hi: u32| -> Option<usize> {
        let mut i = from;
        while let Some(k) = t[i..].find("^s") {
            let st = i + k + 2;
            let num: String = t[st..].chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = num.parse::<u32>()
                && n >= lo
                && n <= hi
            {
                return Some(st + num.len());
            }
            i = st;
        }
        None
    };
    let group = |g: &str| {
        let mut at = 0usize;
        for step in g.split('>') {
            let found = step
                .split('|')
                .map(|alt| alt.trim().to_lowercase())
                .filter_map(|alt| {
                    if let Some(r) = alt.strip_prefix("@s")
                        && let Some((lo, hi)) = r.split_once('-')
                        && let (Ok(lo), Ok(hi)) = (lo.parse::<u32>(), hi.parse::<u32>())
                    {
                        return anchor_after(at, lo, hi);
                    }
                    t[at..].find(&alt).map(|i| at + i + alt.len())
                })
                .min();
            match found {
                Some(end) => at = end,
                None => return false,
            }
        }
        true
    };
    if expect.split(',').all(group) { Verdict::Hit } else { Verdict::Miss }
}

fn story(w: &World) -> Result<&Story, String> {
    w.story().ok_or_else(|| "no narrative layer".to_string())
}

/// Entities matching a description, from all worlds.
fn resolve_any(w: &World, d: &str) -> Vec<Id> {
    let mut out = BTreeSet::new();
    for x in w.worlds() {
        out.extend(x.resolve(d));
    }
    out.into_iter().collect()
}

/// Event participants in human-readable form, from any world.
pub fn roles_any(w: &World, e: &Event) -> String {
    if e.roles.is_empty() {
        return if e.speech.as_ref().is_some_and(|s| s.narrator) { "narrator".into() } else { "—".into() };
    }
    e.roles
        .iter()
        .map(|(r, s)| {
            let name = w.label_any(s.id);
            let name = name.strip_prefix(&s.id.to_string()).unwrap_or(&name).trim().to_string();
            format!("{} {} {name}", r.name(), s.id)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Event in human-readable form: `E7 break (patient O1 the blue vase) ^s2 [F1, level 1]`.
pub fn ev_str(w: &World, id: Id) -> String {
    let Some(e) = w.event_any(id) else { return id.to_string() };
    let mut s = format!("{} ({}) {}", w.label_any(id), roles_any(w, e), e.anchors());
    if let Some(sp) = &e.speech
        && let Some(m) = &sp.means
    {
        let _ = write!(s, " «{m}»");
    }
    if let Some(st) = w.story()
        && let Some(m) = st.ev.get(&id)
    {
        let f = &st.frames[m.frame];
        let _ = write!(s, " [frame {}, level {}", f.name(), st.narr_level(m.frame));
        if m.branch.as_str() != "real" {
            let _ = write!(s, ", branch {}", m.branch);
        }
        s.push(']');
    }
    s
}

/// Events matching a description: `E7` or `ev:verb|verb[/description][@sA-B]`; order — by text.
fn events(w: &World, d: &str) -> Result<Vec<Id>, String> {
    let st = story(w)?;
    if let Some(i) = Id::parse(d) {
        return if st.ev.contains_key(&i) { Ok(vec![i]) } else { Err(format!("no event {d}")) };
    }
    let body = d.strip_prefix("ev:").ok_or_else(|| format!("event description {d:?}"))?;
    let (body, range) = match body.split_once('@') {
        Some((b, r)) => {
            let r = r.strip_prefix('s').ok_or_else(|| format!("range {r:?}"))?;
            let (lo, hi) = r.split_once('-').ok_or_else(|| format!("range {r:?}"))?;
            (b, Some((lo.parse::<u16>().map_err(|_| "range")?, hi.parse::<u16>().map_err(|_| "range")?)))
        }
        None => (body, None),
    };
    let (verbs, who) = match body.split_once('/') {
        Some((v, e)) => (v, Some(e)),
        None => (body, None),
    };
    let verbs: Vec<&str> = verbs.split('|').filter(|x| !x.is_empty()).collect();
    let ents = match who {
        Some(e) => {
            let ids = resolve_any(w, e);
            if ids.is_empty() {
                return Err(format!("no entity {e}"));
            }
            Some(ids)
        }
        None => None,
    };
    let mut found: Vec<(u16, usize, Id)> = Vec::new();
    for (id, m) in &st.ev {
        let Some(e) = w.event_any(*id) else { continue };
        let verb_ok = verbs.is_empty()
            || verbs.iter().any(|v| {
                *v == e.verb || e.speech.as_ref().is_some_and(|s| s.acts().any(|a| a.name() == *v) || *v == "say") || e.verb.split('_').next() == Some(v)
            });
        let who_ok = ents.as_ref().is_none_or(|ids| e.roles.iter().any(|(_, s)| ids.contains(&s.id)) || e.x.about.is_some_and(|a| ids.contains(&a)));
        let range_ok = range.is_none_or(|(lo, hi)| e.anchors.iter().any(|a| *a >= lo && *a <= hi));
        if verb_ok && who_ok && range_ok {
            found.push((m.sent, m.line, *id));
        }
    }
    found.sort();
    if found.is_empty() {
        return Err(format!("no event {d}"));
    }
    Ok(found.into_iter().map(|x| x.2).collect())
}

fn one_ev(w: &World, d: &str) -> Result<Id, String> {
    events(w, d).map(|v| v[0])
}

fn observer(w: &World, d: &str) -> Result<Obs, String> {
    match d {
        "reader" => Ok(Obs::Reader),
        "narrator" => Ok(Obs::Narrator),
        _ => {
            let ids: Vec<Id> = resolve_any(w, d).into_iter().filter(|i| i.reg == Reg::C).collect();
            ids.first().map(|c| Obs::C(*c)).ok_or_else(|| format!("no observer {d}"))
        }
    }
}

fn obs_str(w: &World, o: Obs) -> String {
    match o {
        Obs::C(c) => w.label_any(c),
        Obs::Reader => "reader".into(),
        Obs::Narrator => "narrator".into(),
    }
}

fn sent_arg(t: &str) -> Result<u16, String> {
    t.strip_prefix("at=^s").or_else(|| t.strip_prefix("^s")).and_then(|x| x.parse().ok()).ok_or_else(|| format!("sentence {t:?}"))
}

fn order_str(w: &World, a: Id, b: Id) -> Result<String, String> {
    let st = story(w)?;
    let o = st.time.order(a, b);
    let (ma, mb) = (&st.ev[&a], &st.ev[&b]);
    let mut s = format!("{} — {} — {}", ev_str(w, a), o.uk(), ev_str(w, b));
    // explanation of the path
    let path = match o {
        Order::Before | Order::NotAfter => st.time.path(st.time.get(Pt::E(a)).unwrap(), st.time.get(Pt::S(b)).unwrap()),
        Order::After | Order::NotBefore => st.time.path(st.time.get(Pt::E(b)).unwrap(), st.time.get(Pt::S(a)).unwrap()),
        _ => None,
    };
    if let Some((_, edges)) = path {
        let ex = st.time.explain(&edges, &st.frames);
        if !ex.is_empty() {
            let _ = write!(s, ". Why: {ex}");
        }
    }
    let told = if (ma.sent, ma.line) < (mb.sent, mb.line) { format!("{a} told earlier (^s{} vs ^s{})", ma.sent, mb.sent) } else { format!("{b} told earlier (^s{} vs ^s{})", mb.sent, ma.sent) };
    let _ = write!(s, ". {told}");
    if o == Order::Unknown {
        return Err(format!("order of {a} and {b} on the story axis is unknown. {told}"));
    }
    Ok(s)
}

/// Anachronies: pairs (A told earlier than B, but A happened later than B), grouped by A.
fn anachronies(w: &World, pro: bool, n: usize) -> R {
    let st = story(w)?;
    let cl = st.time.closure();
    let evs: Vec<(Id, (u16, usize), usize, usize)> = st
        .ev
        .iter()
        .filter(|(_, m)| m.branch.as_str() == "real")
        .filter_map(|(id, m)| Some((*id, (m.sent, m.line), st.time.get(Pt::S(*id))?, st.time.get(Pt::E(*id))?)))
        .collect();
    let mut rows: Vec<(usize, Id, Vec<Id>)> = Vec::new();
    // flash-forward and flashback — an event whose time is announced explicitly: its own placement, `time` or `reveal` put it
    // later (flash-forward) or earlier (flashback, content of a past frame) than others. Without this every event of the line
    // told before a flashback would be a "flash-forward" merely because of the flashback itself.
    // the event's own announcement: its placement (creation line), `time` with it, a reveal about it; for a flashback —
    // also the content of a past frame
    let own = |a: Id, why: &Why| match why {
        Why::Place(l) => st.ev[&a].line == *l,
        Why::Time(_) | Why::InFrame(_) => true,
        Why::Reveal(l) => st.reveals.iter().any(|r| r.line == *l && r.ev == a),
        _ => false,
    };
    let announced = |a: Id, x: usize, later: bool| {
        st.time.edges.iter().any(|e| {
            e.alive
                && if later { e.v == x && !matches!(e.why, Why::InFrame(_)) && own(a, &e.why) } else { e.u == x && own(a, &e.why) }
        })
    };
    for (a, ta, sa, ea) in &evs {
        if !announced(*a, if pro { *sa } else { *ea }, pro) {
            continue;
        }
        let mut hits = Vec::new();
        for (b, tb, sb, eb) in &evs {
            if a == b || st.time.wid[*sa] != st.time.wid[*sb] {
                continue;
            }
            // pro: A told earlier, happened later (B < A); ana: A told later, happened earlier
            let hit = if pro { ta < tb && cl.lt(*eb, *sa) } else { ta > tb && cl.lt(*ea, *sb) };
            if hit {
                hits.push(*b);
            }
        }
        if !hits.is_empty() {
            rows.push((hits.len(), *a, hits));
        }
    }
    if rows.is_empty() {
        return Err(if pro { "no flash-forwards" } else { "no flashbacks (told later, happened earlier)" }.into());
    }
    rows.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));
    let mut o = String::new();
    for (k, a, hits) in rows.iter().take(n) {
        let m = &st.ev[a];
        let ex: Vec<String> = hits.iter().take(3).map(|b| format!("{} ^s{}", w.label_any(*b), st.ev[b].sent)).collect();
        let _ = writeln!(
            o,
            "{} — told ^s{}, {} {k} events told {}: {}{}",
            ev_str(w, *a),
            m.sent,
            if pro { "but happened later than" } else { "but happened earlier than" },
            if pro { "later" } else { "earlier" },
            ex.join("; "),
            if hits.len() > 3 { " …" } else { "" }
        );
    }
    Ok(o.trim_end().to_string())
}

/// Claim status at sentence N (state history).
fn claim_at(w: &World, k: Id, at: u16) -> Option<(String, String)> {
    let e = w.ent(k)?;
    let v = e.ver_at(at)?;
    let s = e.snap(v);
    let st = s.get(Field::Stage).to_string();
    let hist: Vec<String> = e
        .states()
        .iter()
        .map(|x| {
            let stage = x.delta.iter().find_map(|d| match d {
                crate::world::Delta::Set(Field::Stage, v) => Some(v.to_string()),
                _ => None,
            });
            format!("{} {}{}{}", stage.unwrap_or_default(), x.pos, if x.by == "reveal" { " (reveal)" } else { "" }, x.cause.map(|c| format!(" because {}", w.label_any(c))).unwrap_or_default())
        })
        .collect();
    Some((st, hist.join(" → ")))
}

fn claims(w: &World, who: Option<Obs>, about: Option<Id>, at: Option<u16>) -> R {
    let n = w.text.sents.len() as u16;
    let at = at.unwrap_or(n);
    let mut o = String::new();
    for e in w.ents(Reg::K) {
        let s = e.now();
        let (Some(o2), Some(ab)) = (val_obs(&s.get(Field::Who)), s.id(Field::About)) else { continue };
        if who.is_some_and(|x| x != o2) || about.is_some_and(|x| x != ab) {
            continue;
        }
        let Some((stage, hist)) = claim_at(w, e.id, at) else { continue };
        let _ = writeln!(o, "{} {}: about {} — «{}»; at ^s{at}: {stage}; history: {hist}", e.id, obs_str(w, o2), ev_str(w, ab), s.get(Field::What).to_string().trim_matches(['«', '»']), );
    }
    if o.is_empty() {
        return Err(format!("no claims{}", if at < n { format!(" before ^s{at}") } else { String::new() }));
    }
    Ok(o.trim_end().to_string())
}

fn truth(w: &World, id: Id) -> R {
    let st = story(w)?;
    let m = &st.ev[&id];
    let e = w.event_any(id).ok_or("no event record")?;
    let mut o = format!("{}; branch: {}", ev_str(w, id), m.branch);
    if e.x.neg {
        o.push_str("; did not happen (neg=1)");
    }
    if let Some(y) = &e.why {
        let _ = write!(o, "; why «{y}»");
    }
    if let Some(c) = e.x.cause {
        let _ = write!(o, "; cause {}", w.label_any(c));
    }
    for r in &m.revealed {
        let what = r.what.clone();
        let what = match what.split_once('=') {
            Some((k, v)) if Id::parse(v).is_some() => format!("{k}={}", w.label_any(Id::parse(v).unwrap())),
            _ => what,
        };
        let _ = write!(o, "; revealed ^s{}: {what}", r.sent);
    }
    if !m.abs.is_empty() {
        let _ = write!(o, "; time from the text: {}", m.abs.iter().map(|(t, s)| format!("«{t}» ^s{s}")).collect::<Vec<_>>().join(", "));
    }
    let cl: Vec<String> = w
        .ents(Reg::K)
        .filter(|k| k.now().id(Field::About) == Some(id))
        .map(|k| {
            let s = k.now();
            format!("{} {} «{}» → {}", k.id, val_obs(&s.get(Field::Who)).map(|x| obs_str(w, x)).unwrap_or_default(), s.get(Field::What).to_string().trim_matches(['«', '»']), s.get(Field::Stage))
        })
        .collect();
    if !cl.is_empty() {
        let _ = write!(o, "; claims: {}", cl.join("; "));
    }
    Ok(o)
}

fn revealed(w: &World, id: Id) -> R {
    let st = story(w)?;
    let rs: Vec<String> = st
        .reveals
        .iter()
        .filter(|r| r.ev == id)
        .map(|r| {
            let mut s = format!("^s{} (frame {}, level {})", r.sent, st.frames[r.frame].name(), st.narr_level(r.frame));
            if let Some(b) = r.by {
                let _ = write!(s, " via {}", ev_str(w, b));
            }
            if !r.to.is_empty() {
                let _ = write!(s, " — to whom: {}", r.to.iter().map(|o| obs_str(w, *o)).collect::<Vec<_>>().join(", "));
            }
            if let Some(t) = &r.interp {
                let _ = write!(s, "; what: «{t}»");
            }
            for (ro, x) in &r.roles {
                let _ = write!(s, "; {}={}", ro.name(), w.label_any(*x));
            }
            if let Some(c) = r.cause {
                let _ = write!(s, "; cause {}", ev_str(w, c));
            }
            if let Some((rel, x)) = r.rel {
                let _ = write!(s, "; time: {id} {} {}", rel.name(), w.label_any(x));
            }
            if !r.wrong.is_empty() {
                let _ = write!(s, "; false: {}", r.wrong.iter().map(|k| claim_line(w, *k)).collect::<Vec<_>>().join(", "));
            }
            if !r.right.is_empty() {
                let _ = write!(s, "; true: {}", r.right.iter().map(|k| claim_line(w, *k)).collect::<Vec<_>>().join(", "));
            }
            s
        })
        .collect();
    if rs.is_empty() {
        return Err(format!("no reveals about {id}"));
    }
    Ok(format!("{}: {}", ev_str(w, id), rs.join(" | ")))
}

fn claim_line(w: &World, k: Id) -> String {
    let Some(e) = w.ent(k) else { return k.to_string() };
    let s = e.now();
    format!("{k} {} «{}»", val_obs(&s.get(Field::Who)).map(|x| obs_str(w, x)).unwrap_or_default(), s.get(Field::What).to_string().trim_matches(['«', '»']))
}

fn knows(w: &World, who: Obs, fact: Id, at: Option<u16>) -> R {
    let st = story(w)?;
    match st.known_since(who, fact, w) {
        Some((s, how)) => match at {
            Some(n) if n < s => Ok(format!("{} at ^s{n} does not yet know {} — learns since ^s{s} ({how})", obs_str(w, who), ev_str(w, fact))),
            _ => Ok(format!("{} knows {} since ^s{s} ({how})", obs_str(w, who), ev_str(w, fact))),
        },
        None => Ok(format!("{} does not know {} — the state has neither a telling to them, nor know, nor reveal", obs_str(w, who), ev_str(w, fact))),
    }
}

/// When the observer learned the truth about an event: a reveal to them (for the reader — any reveal) or their own
/// claim with status `knows`. Knowing that the event took place (from a telling) is not yet knowing the truth.
fn learns(w: &World, who: Obs, ev: Id, at: Option<u16>) -> R {
    let st = story(w)?;
    let mut best: Option<(u16, String)> = None;
    let mut take = |s: u16, how: String| {
        if best.as_ref().is_none_or(|b| s < b.0) {
            best = Some((s, how));
        }
    };
    for r in st.reveals.iter().filter(|r| r.ev == ev) {
        if r.to.contains(&who) || who == Obs::Reader {
            take(r.sent, format!("reveal ^s{}{}{}", r.sent, r.by.map(|b| format!(" via {}", ev_str(w, b))).unwrap_or_default(), r.interp.as_ref().map(|t| format!(": «{t}»")).unwrap_or_default()));
        }
    }
    for k in w.ents(Reg::K) {
        let s = k.now();
        if s.id(Field::About) != Some(ev) || val_obs(&s.get(Field::Who)) != Some(who) {
            continue;
        }
        for x in k.states() {
            let knows = x.delta.iter().any(|d| matches!(d, crate::world::Delta::Set(Field::Stage, Val::Known(V::Tag(t))) if t.as_str() == "knows"));
            if knows {
                take(x.pos.sent, format!("claim {} knows ^s{}: «{}»", k.id, x.pos.sent, s.get(Field::What).to_string().trim_matches(['«', '»'])));
            }
        }
    }
    let w_ = obs_str(w, who);
    match (best, at) {
        (Some((s, how)), Some(n)) if n < s => Ok(format!("{w_} at ^s{n} does not yet know the truth about {} — learns since ^s{s} ({how})", ev_str(w, ev))),
        (Some((s, how)), _) => Ok(format!("{w_} knows the truth about {} since ^s{s} ({how})", ev_str(w, ev))),
        (None, _) => Ok(format!("{w_} does not know the truth about {} — the state has no reveal to them or claim knows", ev_str(w, ev))),
    }
}

fn frames_str(w: &World) -> R {
    let st = story(w)?;
    let mut o = String::new();
    for (i, f) in st.frames.iter().enumerate() {
        let n = st.ev.values().filter(|m| m.frame == i).count();
        let spans: Vec<String> = f.spans.iter().map(|(a, b)| format!("^s{a}–{}", b.map(|x| format!("^s{x}")).unwrap_or("…".into()))).collect();
        let kind = match f.kind {
            FWorld::Root => "root",
            FWorld::Past => "same world, past",
            FWorld::Now => "same world, present",
            FWorld::New => "new world (story within a story)",
            FWorld::Branch => "branch",
        };
        let _ = writeln!(
            o,
            "{} level {}: {} → {}; {kind}{}; sentences {}; events {n}",
            f.name(),
            f.level,
            obs_str(w, f.narrator),
            if f.to.is_empty() { "—".into() } else { f.to.iter().map(|x| obs_str(w, *x)).collect::<Vec<_>>().join(", ") },
            if f.branch.as_str() != "real" { format!(" ({})", f.branch) } else { String::new() },
            spans.join(", ")
        );
    }
    Ok(o.trim_end().to_string())
}

fn run1(w: &World, q: &str) -> R {
    let toks: Vec<&str> = q.split_whitespace().collect();
    let op = *toks.first().ok_or("empty query")?;
    let st = story(w)?;
    match op {
        "order" => {
            let (a, b) = (toks.get(1).ok_or("order: events missing")?, toks.get(2).ok_or("order: second event missing")?);
            order_str(w, one_ev(w, a)?, one_ev(w, b)?)
        }
        "sort" => {
            let ids: Vec<Id> = toks[1..].iter().map(|d| one_ev(w, d)).collect::<Result<_, _>>()?;
            let cl = st.time.closure();
            let node = |x: Id, e: bool| st.time.get(if e { Pt::E(x) } else { Pt::S(x) }).unwrap();
            let before = |a: Id, b: Id| cl.lt(node(a, true), node(b, false));
            // not later: end of a ≤ start of b (non-strict edge, e.g. cause ≤ effect)
            let not_after = |a: Id, b: Id| a != b && cl.le(node(a, true), node(b, false));
            let mut v = ids.clone();
            // topologically: how many other events of the set are earlier (or not later) than this one
            v.sort_by_key(|x| (ids.iter().filter(|y| before(**y, *x) || (not_after(**y, *x) && !not_after(*x, **y))).count(), st.ev[x].sent));
            let mut o = ev_str(w, v[0]);
            for p in v.windows(2) {
                let sep = if before(p[0], p[1]) {
                    "<"
                } else if not_after(p[0], p[1]) {
                    "≤"
                } else {
                    "?"
                };
                let _ = write!(o, " {sep} {}", ev_str(w, p[1]));
            }
            Ok(o)
        }
        "prolepses" | "analepses" => anachronies(w, op == "prolepses", toks.get(1).and_then(|x| x.parse().ok()).unwrap_or(5)),
        "abs" => {
            let e = one_ev(w, toks.get(1).ok_or("abs: event missing")?)?;
            let m = &st.ev[&e];
            if m.abs.is_empty() {
                return Err(format!("{e}: no absolute time in the text"));
            }
            Ok(format!("{}: {}", ev_str(w, e), m.abs.iter().map(|(t, s)| format!("«{t}» ^s{s}")).collect::<Vec<_>>().join(", ")))
        }
        "claims" => {
            let mut who = None;
            let mut about = None;
            let mut at = None;
            for t in &toks[1..] {
                if let Some(x) = t.strip_prefix("who=") {
                    who = Some(observer(w, x)?);
                } else if let Some(x) = t.strip_prefix("about=") {
                    about = Some(one_ev(w, x)?);
                } else {
                    at = Some(sent_arg(t)?);
                }
            }
            claims(w, who, about, at)
        }
        "truth" => truth(w, one_ev(w, toks.get(1).ok_or("truth: event missing")?)?),
        "revealed" => revealed(w, one_ev(w, toks.get(1).ok_or("revealed: event missing")?)?),
        "knows" => {
            let who = observer(w, toks.get(1).ok_or("knows: observer missing")?)?;
            let f = toks.get(2).ok_or("knows: fact missing")?;
            let fact = if f.starts_with('K') { Id::parse(f).ok_or("fact")? } else { one_ev(w, f)? };
            knows(w, who, fact, toks.get(3).map(|t| sent_arg(t)).transpose()?)
        }
        "learns" => {
            let who = observer(w, toks.get(1).ok_or("learns: observer missing")?)?;
            let e = one_ev(w, toks.get(2).ok_or("learns: event missing")?)?;
            learns(w, who, e, toks.get(3).map(|t| sent_arg(t)).transpose()?)
        }
        "branch" => {
            let e = one_ev(w, toks.get(1).ok_or("branch: event missing")?)?;
            let m = &st.ev[&e];
            let log: Vec<String> = m.branch_log.iter().map(|(b, _, s, by)| format!("{b} ^s{s}{}", by.map(|x| format!(" because {}", w.label_any(x))).unwrap_or_default())).collect();
            Ok(format!("{}: branch {} (history: {})", ev_str(w, e), m.branch, log.join(" → ")))
        }
        "frames" => frames_str(w),
        "teller" => {
            let n: u8 = toks.get(1).and_then(|x| x.parse().ok()).ok_or("teller: level")?;
            let v: Vec<String> = st
                .frames
                .iter()
                .enumerate()
                .filter(|(i, _)| st.narr_level(*i) == n)
                .map(|(_, f)| {
                    format!(
                        "{}: {} → {} (sentences {})",
                        f.name(),
                        obs_str(w, f.narrator),
                        if f.to.is_empty() { "—".into() } else { f.to.iter().map(|x| obs_str(w, *x)).collect::<Vec<_>>().join(", ") },
                        f.spans.iter().map(|(a, b)| format!("^s{a}–{}", b.map(|x| format!("^s{x}")).unwrap_or("…".into()))).collect::<Vec<_>>().join(", ")
                    )
                })
                .collect();
            if v.is_empty() {
                return Err(format!("no frames at level {n}"));
            }
            Ok(format!("level {n} ({} frames): {}", v.len(), v.join("; ")))
        }
        "levels" => {
            let d = st.narr_depth();
            let per: Vec<String> = (0..d).map(|l| format!("level {l}: {} frames", (0..st.frames.len()).filter(|i| st.narr_level(*i) == l).count())).collect();
            Ok(format!("levels: {d} (0–{}) by narrators; frame nesting {}; {}", d - 1, st.depth(), per.join(", ")))
        }
        "level" => {
            let e = one_ev(w, toks.get(1).ok_or("level: event missing")?)?;
            let m = &st.ev[&e];
            let f = &st.frames[m.frame];
            Ok(format!("{}: level {} (frame {}, nesting {}, narrator {})", ev_str(w, e), st.narr_level(m.frame), f.name(), f.level, obs_str(w, f.narrator)))
        }
        _ => Err(format!("unknown query {op}")),
    }
}

/// Answers to the questions of one testbed in md: a table with verdicts and a summary by kind.
pub fn answers_md(w: &World, qs: &[Q3]) -> (String, Vec<(String, crate::qa::Verdict)>) {
    let mut o = String::from("| id | kind | question | answer from state | expected | verdict |\n|---|---|---|---|---|---|\n");
    let mut v = Vec::new();
    for q in qs {
        let a = answer(w, &q.query);
        let vd = verdict(&a, &q.expect);
        let cell = |s: &str| s.replace('|', "\\|").replace('\n', "<br>");
        let _ = writeln!(o, "| {} | {} | {} | {} | `{}` | {} |", q.id, q.kind, cell(&q.text), cell(&a.text), cell(&q.expect), vd.name());
        v.push((q.kind.clone(), vd));
    }
    (o, v)
}

/// Role name for a role label (for tests and md).
pub fn role_name(r: Role) -> &'static str {
    r.name()
}
