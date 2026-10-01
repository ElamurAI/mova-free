//! Questions of FairytaleQA types (character, setting, action, feeling, causal, outcome, prediction) —
//! answers from the world state and history, without the LLM, with anchors. The question is pre-formalized into a query
//! (question understanding is not solved here); `a || b` — second reading if the first gave no answer.
//! Where the state cannot answer — honestly "not in state" with a reason.
//!
//! Queries:
//!   main                                    main and secondary characters (attention = mentions)
//!   where <desc> <time>                     where the entity is (time: ^sN, @first — sentence of appearance, @last — end)
//!   holder <desc> <time> [<time>…]          who/what holds the object
//!   holders <desc>                          the whole location history of the object
//!   attr <desc> <key> <time>                attribute value and the state that set it
//!   cause <desc> <key>=<value>              when and why the attribute got the value (causing event)
//!   say <desc>|narrator [to=<desc>] [act=a|b]   speech acts: act, indirectness, sincerity, means, why
//!   why verb=a|b [agent=<desc>] [patient=<desc>]  motives of events
//!   who verb=a|b [agent=…] [patient=…] [to=…]     who performed the event
//!   events agent=<desc> [from=^sN] [to=^sM]       events of a character
//!   rel <desc> <desc>                        history of a relationship
//!   predict-rel <desc> <desc>                prediction from the last state of the relationship
//! Description: `C1`, `char:fox`, `obj:meat|flesh`, `loc:tree`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::lang::Role;
use crate::types::*;
use crate::world::{Event, World};

/// Question: document, number, FairytaleQA type, text, formal query, expected (comma-separated words —
/// all; alternatives via `|` — any; case-insensitive comparison).
#[derive(Clone, Debug)]
pub struct Question {
    pub doc: i64,
    pub id: String,
    pub kind: String,
    pub text: String,
    pub query: String,
    pub expect: String,
}

/// Read questions (TSV: doc, id, type, question, query, expected; `#` — comment).
pub fn read_questions(text: &str) -> Result<Vec<Question>, String> {
    let mut out = Vec::new();
    for (i, l) in text.lines().enumerate() {
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 6 {
            return Err(format!("questions, line {}: {} columns instead of 6", i + 1, c.len()));
        }
        let doc = c[0].trim().parse().map_err(|_| format!("questions, line {}: doc {:?}", i + 1, c[0]))?;
        out.push(Question { doc, id: c[1].into(), kind: c[2].into(), text: c[3].into(), query: c[4].into(), expect: c[5].into() });
    }
    Ok(out)
}

/// MMM answer: text with anchors or "not in state" with a reason.
#[derive(Clone, Debug)]
pub struct Answer {
    pub text: String,
    pub from_state: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    Hit,
    Miss,
    NotInState,
}

impl Verdict {
    pub fn name(self) -> &'static str {
        match self {
            Verdict::Hit => "hit",
            Verdict::Miss => "miss",
            Verdict::NotInState => "not in state",
        }
    }
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

/// Scoring: expected — comma-separated groups (all), within a group steps via `>` (in order in the answer text),
/// within a step alternatives via `|` (any).
pub fn verdict(a: &Answer, expect: &str) -> Verdict {
    if !a.from_state {
        return Verdict::NotInState;
    }
    let t = a.text.to_lowercase();
    let group = |g: &str| {
        let mut at = 0usize;
        for step in g.split('>') {
            let found = step
                .split('|')
                .map(|alt| alt.trim().to_lowercase())
                .filter_map(|alt| t[at..].find(&alt).map(|i| at + i + alt.len()))
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

type R = Result<String, String>;

struct Q<'a> {
    pos: Vec<&'a str>,
    kv: BTreeMap<&'a str, &'a str>,
}

fn parse_q(q: &str) -> Result<(&str, Q<'_>), String> {
    let mut it = q.split_whitespace();
    let op = it.next().ok_or("empty query")?;
    let mut pos = Vec::new();
    let mut kv = BTreeMap::new();
    // options are only these keys; the rest (`char:fox`, `^s3`, `sleep=awake` in cause) are positional
    const OPT: &[&str] = &["to", "act", "verb", "agent", "patient", "from"];
    for t in it {
        match t.split_once('=') {
            Some((k, v)) if OPT.contains(&k) => {
                kv.insert(k, v);
            }
            _ => pos.push(t),
        }
    }
    Ok((op, Q { pos, kv }))
}

fn one(w: &World, d: &str) -> Result<Id, String> {
    w.resolve(d).into_iter().next().ok_or_else(|| format!("no entity {d}"))
}

fn sent_of(w: &World, id: Id, t: &str) -> Result<u16, String> {
    let n = w.text.sents.len() as u16;
    match t {
        "@last" => Ok(n),
        "@first" => w.ent(id).and_then(|e| e.states().first()).map(|s| s.pos.sent).ok_or_else(|| format!("{id} has no states")),
        _ => t.strip_prefix("^s").and_then(|x| x.parse().ok()).filter(|x| *x >= 1 && *x <= n).ok_or_else(|| format!("time {t:?}")),
    }
}

fn alts(v: &str) -> Vec<String> {
    v.split('|').map(str::to_string).collect()
}

fn why_str(w: &World, e: &Event) -> String {
    let mut s = format!("{} {} ({})", w.label(e.id), e.anchors(), w.roles_str(e));
    if let Some(y) = &e.why {
        let _ = write!(s, ": why «{y}»");
    }
    if let Some(h) = &e.how {
        let _ = write!(s, "; how «{h}»");
    }
    s
}

/// Events matching verb= and roles (description → id; unknown description is an error).
fn matching<'w>(w: &'w World, q: &Q) -> Result<Vec<&'w Event>, String> {
    let verbs = q.kv.get("verb").map(|v| alts(v));
    let mut need: Vec<(Role, Vec<Id>)> = Vec::new();
    for (k, r) in [("agent", Role::Agent), ("patient", Role::Patient), ("to", Role::To)] {
        if let Some(d) = q.kv.get(k) {
            let ids = w.resolve(d);
            if ids.is_empty() {
                return Err(format!("no entity {d}"));
            }
            need.push((r, ids));
        }
    }
    Ok(w.events()
        .iter()
        .filter(|e| verbs.as_ref().is_none_or(|v| v.iter().any(|x| *x == e.verb)))
        .filter(|e| need.iter().all(|(r, ids)| e.has(*r, ids)))
        .collect())
}

fn run1(w: &World, q: &str) -> R {
    let (op, q) = parse_q(q)?;
    match op {
        "main" => {
            let n = w.text.sents.len() as u16;
            let c = w.context_at(n);
            if c.main.is_empty() {
                return Err("no characters".into());
            }
            let list = |v: &[(Id, usize)]| {
                if v.is_empty() {
                    return "—".to_string();
                }
                v.iter()
                    .map(|(i, k)| {
                        let m = w.ent(*i).map(|e| e.mentions().to_vec()).unwrap_or_default();
                        let span = match (m.first(), m.last()) {
                            (Some(a), Some(b)) => format!("{a}–{b}"),
                            _ => String::new(),
                        };
                        format!("{} ({k} mentions, {span})", w.label(*i))
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            Ok(format!("main: {}; secondary: {}", list(&c.main), list(&c.secondary)))
        }
        "where" => {
            let [d, t] = q.pos[..] else { return Err("where <desc> <time>".into()) };
            let id = one(w, d)?;
            if t == "@first" {
                let (st, _) = w.history(id, Field::At).into_iter().next().ok_or_else(|| format!("{}: location unknown", w.label(id)))?;
                return w.where_ver(id, st.ver).ok_or_else(|| format!("{}: location unknown", w.label(id)));
            }
            let s = sent_of(w, id, t)?;
            w.where_at(id, s).ok_or_else(|| format!("{}: location unknown at ^s{s}", w.label(id)))
        }
        "holder" => {
            let (d, times) = q.pos.split_first().ok_or("holder <desc> <time>…")?;
            let o = one(w, d)?;
            let mut out = Vec::new();
            for t in times {
                // @first — first known location (in the same sentence the item may have already moved on)
                let found = if *t == "@first" {
                    w.history(o, Field::At).into_iter().next().and_then(|(st, v)| match v {
                        Val::Known(V::Id(h)) => Some((h, st.ver, st)),
                        _ => None,
                    })
                } else {
                    w.holder_at(o, sent_of(w, o, t)?)
                };
                let (h, v, st) = found.ok_or_else(|| format!("{}: holder unknown at {t}", w.label(o)))?;
                let how = if h.reg == Reg::L { format!("lies in {}", w.place_chain(h)) } else { w.label(h) };
                let cause = st.cause.map(|c| format!(" because {}", w.label(c))).unwrap_or_default();
                out.push(format!("{t}: {how} ({o}@{v}, since {} {}{cause})", st.pos, st.by));
            }
            Ok(out.join("; "))
        }
        "holders" => {
            let [d] = q.pos[..] else { return Err("holders <desc>".into()) };
            let o = one(w, d)?;
            let h: Vec<String> = w
                .history(o, Field::At)
                .into_iter()
                .map(|(st, v)| {
                    let at = match v {
                        Val::Known(V::Id(i)) => w.label(i),
                        _ => "?".into(),
                    };
                    let cause = st.cause.map(|c| format!(" because {}", w.label(c))).unwrap_or_default();
                    format!("{at} {} ({}{cause})", st.pos, st.by)
                })
                .collect();
            if h.is_empty() {
                return Err(format!("{}: location unknown", w.label(o)));
            }
            Ok(format!("{}: {}", w.label(o), h.join(" → ")))
        }
        "attr" => {
            let [d, k, t] = q.pos[..] else { return Err("attr <desc> <key> <time>".into()) };
            let (f, _) = key(k).ok_or_else(|| format!("key {k}"))?;
            let id = one(w, d)?;
            let e = w.ent(id).ok_or("no entity")?;
            let (v, s) = if t == "@first" {
                // first known value
                let (st, _) = w.history(id, f).into_iter().find(|(_, v)| *v != Val::Unknown).ok_or_else(|| format!("{}: {k} unknown", w.label(id)))?;
                (st.ver, st.pos.sent)
            } else {
                let s = sent_of(w, id, t)?;
                (e.ver_at(s).ok_or_else(|| format!("{} does not exist yet at ^s{s}", w.label(id)))?, s)
            };
            match e.snap(v).get(f) {
                Val::Unknown => Err(format!("{}: {k} unknown at ^s{s}", w.label(id))),
                Val::Known(x) => {
                    let st = e.set_by(f, v);
                    let since = st.map(|st| format!(" (since {} {}{})", st.pos, st.by, st.cause.map(|c| format!(" because {}", w.label(c))).unwrap_or_default())).unwrap_or_default();
                    Ok(format!("{}@{v}: {k}={x}{since}", w.label(id)))
                }
            }
        }
        "cause" => {
            let [d, kv] = q.pos[..] else { return Err("cause <desc> <key>=<value>".into()) };
            let (k, val) = kv.split_once('=').ok_or("cause: key=value")?;
            let (f, _) = key(k).ok_or_else(|| format!("key {k}"))?;
            let id = one(w, d)?;
            let hit = w.history(id, f).into_iter().find(|(_, v)| match v {
                Val::Known(V::Tag(t)) => t.as_str() == val,
                Val::Known(V::Tags(ts)) => ts.iter().any(|t| t.as_str() == val),
                Val::Known(V::Words(ws)) => ws.iter().any(|x| x == val),
                Val::Known(V::Text(t)) => t.to_lowercase().contains(&val.to_lowercase()),
                Val::Known(V::Id(i)) => i.to_string() == val,
                Val::Unknown => false,
            });
            let (st, _) = hit.ok_or_else(|| format!("{}: {k}={val} not in history", w.label(id)))?;
            let c = st.cause.ok_or_else(|| format!("{}: {k}={val} since {}, cause not recorded", w.label(id), st.pos))?;
            let ev = w.event(c).map(|e| why_str(w, e)).unwrap_or_else(|| c.to_string());
            Ok(format!("{}@{}: {k}={val} since {} because {ev}", w.label(id), st.ver, st.pos))
        }
        "say" => {
            let [d] = q.pos[..] else { return Err("say <desc>|narrator …".into()) };
            let who: Option<Vec<Id>> = if d == "narrator" { None } else { Some(w.resolve(d)) };
            if who.as_ref().is_some_and(|v| v.is_empty()) {
                return Err(format!("no entity {d}"));
            }
            let to = match q.kv.get("to") {
                Some(t) => {
                    let v = w.resolve(t);
                    if v.is_empty() {
                        return Err(format!("no entity {t}"));
                    }
                    Some(v)
                }
                None => None,
            };
            let acts = q.kv.get("act").map(|v| alts(v));
            let hits: Vec<String> = w
                .events()
                .iter()
                .filter_map(|e| e.speech.as_ref().map(|s| (e, s)))
                .filter(|(e, s)| match &who {
                    None => s.narrator,
                    Some(ids) => e.has(Role::Agent, ids),
                })
                .filter(|(e, _)| to.as_ref().is_none_or(|ids| e.has(Role::To, ids)))
                .filter(|(_, s)| acts.as_ref().is_none_or(|v| s.acts().any(|a| v.iter().any(|x| x == a.name()))))
                .map(|(e, s)| {
                    let mut x = format!("{} {} ({}): act={}", e.id, e.anchors(), w.roles_str(e), s.acts_str());
                    if s.indirect {
                        x.push_str(", indirect");
                    }
                    if s.sincere == Some(false) {
                        x.push_str(", insincere");
                    }
                    if let Some(m) = &s.means {
                        let _ = write!(x, "; means «{m}»");
                    }
                    if let Some(y) = &e.why {
                        let _ = write!(x, "; why «{y}»");
                    }
                    x
                })
                .collect();
            if hits.is_empty() {
                return Err(format!("no utterances by {d}"));
            }
            Ok(hits.join(" | "))
        }
        "why" => {
            let ev = matching(w, &q)?;
            if ev.is_empty() {
                return Err(format!("no events {}", q.kv.get("verb").unwrap_or(&"?")));
            }
            let with: Vec<String> = ev.iter().filter(|e| e.why.is_some()).map(|e| why_str(w, e)).collect();
            if with.is_empty() {
                return Err(format!("{} exist, no motive", ev.iter().map(|e| w.label(e.id)).collect::<Vec<_>>().join(", ")));
            }
            Ok(with.join(" | "))
        }
        "who" => {
            let ev = matching(w, &q)?;
            let with: Vec<String> = ev
                .iter()
                .filter_map(|e| {
                    let ag = e.all(Role::Agent);
                    if ag.is_empty() {
                        return None;
                    }
                    let names = ag.iter().map(|a| w.label(a.id)).collect::<Vec<_>>().join(" + ");
                    let mut s = format!("{names} ({} {}", w.label(e.id), e.anchors());
                    if let Some(y) = &e.why {
                        let _ = write!(s, "; why «{y}»");
                    }
                    s.push(')');
                    Some(s)
                })
                .collect();
            if with.is_empty() {
                return Err(format!("no events with a performer {}", q.kv.get("verb").unwrap_or(&"?")));
            }
            Ok(with.join("; "))
        }
        "events" => {
            let from = q.kv.get("from").and_then(|x| x.strip_prefix("^s")).and_then(|x| x.parse::<u16>().ok()).unwrap_or(1);
            let to_s = q.kv.get("to").and_then(|x| x.strip_prefix("^s")).and_then(|x| x.parse::<u16>().ok()).unwrap_or(u16::MAX);
            let mut q2 = Q { pos: Vec::new(), kv: q.kv.clone() };
            q2.kv.remove("from");
            q2.kv.remove("to");
            let ev: Vec<String> = matching(w, &q2)?.into_iter().filter(|e| e.pos.sent >= from && e.pos.sent <= to_s).map(|e| why_str(w, e)).collect();
            if ev.is_empty() {
                return Err("no events in the interval".into());
            }
            Ok(ev.join("; "))
        }
        "rel" | "predict-rel" => {
            let [a, b] = q.pos[..] else { return Err(format!("{op} <desc> <desc>")) };
            let (a, b) = (one(w, a)?, one(w, b)?);
            let rels: Vec<Id> = w
                .ents(Reg::R)
                .filter(|e| {
                    let s = e.now();
                    let (x, y) = (s.id(Field::A), s.id(Field::B));
                    (x == Some(a) && y == Some(b)) || (x == Some(b) && y == Some(a))
                })
                .map(|e| e.id)
                .collect();
            let r = *rels.first().ok_or_else(|| format!("no relationship {} — {}", w.label(a), w.label(b)))?;
            let hist: Vec<String> = w
                .history(r, Field::Kind)
                .into_iter()
                .map(|(st, v)| format!("{v} {}{}", st.pos, st.cause.map(|c| format!(" (because {})", w.label(c))).unwrap_or_default()))
                .collect();
            let e = w.ent(r).ok_or("no entity")?;
            let facts: Vec<String> = e.now().ids(SetF::Facts).iter().map(|f| w.label(*f)).collect();
            let head = format!("{}: {}", w.label(r), hist.join(" → "));
            if op == "rel" {
                let facts = if facts.is_empty() { String::new() } else { format!("; facts: {}", facts.join(", ")) };
                return Ok(format!("{head}{facts}"));
            }
            let kind = match e.now().get(Field::Kind) {
                Val::Known(V::Tag(t)) => t.as_str(),
                _ => "",
            };
            let guess = match kind {
                "friend" | "ally" => "probably will help each other",
                "adversary" => "probably will be wary of each other and feud",
                "rival" => "probably will keep competing",
                "kin" | "romantic" => "probably will stick together",
                _ => return Err(format!("{head} — relationship {kind} gives no prediction")),
            };
            Ok(format!("{guess}: {head}"))
        }
        "state" => {
            let [d, t] = q.pos[..] else { return Err("state <desc> <time>".into()) };
            let id = one(w, d)?;
            let s = sent_of(w, id, t)?;
            let e = w.ent(id).ok_or("no entity")?;
            let v = e.ver_at(s).ok_or_else(|| format!("{} does not exist yet at ^s{s}", w.label(id)))?;
            Ok(format!("{}@{v} at ^s{s}: {}", w.label(id), crate::md::snap_str(w, id, v)))
        }
        "diff" => {
            let [d, a, b] = q.pos[..] else { return Err("diff <desc> <version> <version>".into()) };
            let id = one(w, d)?;
            let num = |x: &str| x.trim_start_matches('@').parse::<u32>().map_err(|_| format!("version {x:?}"));
            w.diff(id, num(a)?, num(b)?).ok_or_else(|| format!("{}: no such versions", w.label(id)))
        }
        "mentions" => {
            let [d] = q.pos[..] else { return Err("mentions <desc>".into()) };
            let id = one(w, d)?;
            let m: Vec<String> = w.ent(id).map(|e| e.mentions().iter().map(|p| format!("{p} ({})", p.long())).collect()).unwrap_or_default();
            if m.is_empty() {
                return Err(format!("{}: no mentions", w.label(id)));
            }
            Ok(format!("{}: {}", w.label(id), m.join(", ")))
        }
        "chrono" => {
            if w.events().is_empty() {
                return Err("no events".into());
            }
            Ok(w.events().iter().map(|e| why_str(w, e)).collect::<Vec<_>>().join("\n"))
        }
        _ => Err(format!("unknown query {op}")),
    }
}

/// Answer summary: FairytaleQA questions (hit, miss, not in state) and the honesty control.
#[derive(Clone, Copy, Debug, Default)]
pub struct Score {
    pub hit: usize,
    pub miss: usize,
    pub none: usize,
    pub ctrl_ok: usize,
    pub ctrl: usize,
}

/// Control question type: by construction the answer is not in the state; correct is "not in state".
/// The value is a data marker: the type column of data/questions.tsv.
pub const CONTROL: &str = "control";

/// Answers to a document's questions: md table, separately — the honesty control.
pub fn answers_md(w: &World, qs: &[Question]) -> (String, Score) {
    let mut o = format!("# Answers from the state: {} (doc {})\n\n", w.text.title, w.text.doc);
    o.push_str("The MMM answers from the world state and history, without the LLM. The query is a pre-formalized question.\n\n");
    o.push_str("| # | type | question | query | MMM answer | expected | verdict |\n|---|---|---|---|---|---|---|\n");
    let mut n = Score::default();
    let cell = |s: &str| s.replace('|', "\\|");
    let mut ctrl = String::new();
    for q in qs {
        let a = answer(w, &q.query);
        if q.kind == CONTROL {
            n.ctrl += 1;
            let ok = !a.from_state;
            n.ctrl_ok += ok as usize;
            let _ = writeln!(ctrl, "| {} | {} | `{}` | {} | {} |", q.id, cell(&q.text), cell(&q.query), cell(&a.text), if ok { "honest" } else { "made up" });
            continue;
        }
        let v = verdict(&a, &q.expect);
        match v {
            Verdict::Hit => n.hit += 1,
            Verdict::Miss => n.miss += 1,
            Verdict::NotInState => n.none += 1,
        }
        let _ = writeln!(o, "| {} | {} | {} | `{}` | {} | {} | {} |", q.id, q.kind, cell(&q.text), cell(&q.query), cell(&a.text), cell(&q.expect), v.name());
    }
    let _ = writeln!(o, "\nHit {}, miss {}, not in state {} (of {}).", n.hit, n.miss, n.none, n.hit + n.miss + n.none);
    if n.ctrl > 0 {
        o.push_str("\n## Honesty control\n\nQuestions whose answers are not in the state by construction (added after the run as a negative control of the answerer): correct is \"not in state\", not a made-up answer.\n\n");
        o.push_str("| # | question | query | MMM answer | result |\n|---|---|---|---|---|\n");
        o.push_str(&ctrl);
        let _ = writeln!(o, "\nHonest {} of {}.",
 n.ctrl_ok, n.ctrl);
    }
    (o, n)
}
