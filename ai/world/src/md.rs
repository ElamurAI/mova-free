//! Human view of the world — an md graph with anchors: text, cursor, registries (locations, objects, characters with
//! state deltas, relations), event chronology, working memory at the end, gate rejections.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use crate::lang::FormatErr;
use crate::types::*;
use crate::world::{Delta, Run, World};

fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn mentions(w: &World, id: Id) -> String {
    let m = w.ent(id).map(|e| e.mentions().to_vec()).unwrap_or_default();
    if m.is_empty() {
        return "—".into();
    }
    m.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" ")
}

fn ids(w: &World, v: &[Id]) -> String {
    if v.is_empty() {
        return "—".into();
    }
    v.iter().map(|i| w.label(*i)).collect::<Vec<_>>().join(", ")
}

fn delta_str(w: &World, d: &[Delta]) -> String {
    d.iter()
        .map(|x| match x {
            Delta::Set(Field::At | Field::Parent | Field::Mother | Field::Father | Field::A | Field::B, Val::Known(V::Id(i))) => {
                let Delta::Set(k, _) = x else { unreachable!() };
                format!("{}={}", k.name(), w.label(*i))
            }
            _ => x.to_string(),
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// Snapshot in human form: known fields (for a character also unknown base ones: sex, age) and sets.
pub fn snap_str(w: &World, id: Id, ver: u32) -> String {
    let Some(e) = w.ent(id) else { return String::new() };
    let s = e.snap(ver);
    let mut parts = Vec::new();
    for (k, v) in s.fields() {
        if matches!(k, Field::Type | Field::Class) {
            continue;
        }
        let v = match (k, v) {
            (Field::At | Field::Parent, V::Id(i)) if i.reg == Reg::L => w.place_chain(*i),
            (_, V::Id(i)) => w.label(*i),
            _ => v.to_string(),
        };
        parts.push(format!("{}={v}", k.name()));
    }
    if id.reg == Reg::C {
        for k in [Field::Gender, Field::Age] {
            if s.get(k) == Val::Unknown {
                parts.push(format!("{}=?", k.name()));
            }
        }
    }
    for (k, set) in s.sets() {
        let v: Vec<Id> = set.iter().copied().collect();
        parts.push(format!("{}={{{}}}", k.name(), ids(w, &v)));
    }
    parts.join("; ")
}

/// md world of a document.
pub fn world_md(r: &Run, fmt_errs: &[FormatErr], lines_total: usize) -> String {
    let w = &r.world;
    let t = &w.text;
    let mut o = String::new();
    let _ = writeln!(o, "# Tale world: {} (doc {})\n", t.title, t.doc);
    let _ = writeln!(
        o,
        "Sentences {}; LLM lines {lines_total}: accepted {}, rejected by format {}, by gates {}; sentences without commands {}.\n",
        t.sents.len(),
        r.accepted,
        fmt_errs.len(),
        r.rejects.len(),
        r.uncovered.len()
    );

    o.push_str("## Text\n\n");
    for (i, (p, s)) in t.sents.iter().enumerate() {
        let _ = writeln!(o, "- `^s{}` (par. {p}) {s}", i + 1);
    }

    // the cursor moves with the narration: for each sentence — place in the text, location and time (last cursor state)
    o.push_str("\n## Cursor\n\n| sentence | place in text | at | time |\n|---|---|---|---|\n");
    for i in 1..=t.sents.len() as u16 {
        let c = w.cursor().iter().rev().find(|c| c.pos.sent <= i);
        let pos = TextPos { chapter: c.map(|c| c.pos.chapter).unwrap_or(1), para: t.sents[i as usize - 1].0, sent: i };
        let at = c.and_then(|c| c.at).map(|l| w.place_chain(l)).unwrap_or_else(|| "?".into());
        let time = c.and_then(|c| c.time.clone()).unwrap_or_else(|| "—".into());
        let _ = writeln!(o, "| {pos} | {} | {} | {time} |", pos.long(), cell(&at));
    }

    o.push_str("\n## Location registry\n\n| id | type/class | in | contents at the end | mentions |\n|---|---|---|---|---|\n");
    for e in w.ents(Reg::L) {
        let s = e.now();
        let parent = s.id(Field::Parent).map(|p| w.label(p)).unwrap_or_else(|| "—".into());
        let mut inside = Vec::new();
        for (k, set) in [("characters", SetF::Chars), ("objects", SetF::Objects), ("sub-places", SetF::Children)] {
            let v = s.ids(set);
            if !v.is_empty() {
                inside.push(format!("{k}: {}", ids(w, &v)));
            }
        }
        let inside = if inside.is_empty() { "—".into() } else { inside.join("; ") };
        let _ = writeln!(o, "| {} | {} | {} | {} | {} |", w.label(e.id), w.kind_of(e.id), cell(&parent), cell(&inside), mentions(w, e.id));
    }

    o.push_str("\n## Object registry\n\n| id | type/class | place: history | at the end | mentions |\n|---|---|---|---|---|\n");
    for e in w.ents(Reg::O) {
        let hist: Vec<String> = w
            .history(e.id, Field::At)
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
        let hist = if hist.is_empty() { "?".into() } else { hist.join(" → ") };
        let _ = writeln!(o, "| {} | {} | {} | {} | {} |", w.label(e.id), w.kind_of(e.id), cell(&hist), cell(&snap_str(w, e.id, e.latest())), mentions(w, e.id));
    }

    o.push_str("\n## Characters: immutable states\n\nEach row is a new state `C@n`: delta from the previous one, cause, anchor. Snapshot is the full state.\n");
    for e in w.ents(Reg::C) {
        let _ = writeln!(o, "\n### {} — {}\n", w.label(e.id), w.kind_of(e.id));
        o.push_str("| state | anchor | command | delta | cause |\n|---|---|---|---|---|\n");
        for st in e.states() {
            let cause = st.cause.map(|c| w.label(c)).unwrap_or_else(|| "—".into());
            let _ = writeln!(o, "| {}@{} | {} | {} | {} | {} |", e.id, st.ver, st.pos, st.by, cell(&delta_str(w, &st.delta)), cell(&cause));
        }
        let _ = writeln!(o, "\nSnapshot at the end `{}@{}`: {}", e.id, e.latest(), snap_str(w, e.id, e.latest()));
        let m = e.mentions();
        let _ = writeln!(
            o,
            "\nMentions ({}): {}{}",
            m.len(),
            mentions(w, e.id),
            match (m.first(), m.last()) {
                (Some(a), Some(b)) => format!(" — first {} ({}), last {} ({})", a, a.long(), b, b.long()),
                _ => String::new(),
            }
        );
    }

    o.push_str("\n## Relations\n");
    if w.ents(Reg::R).next().is_none() {
        o.push_str("\n—\n");
    }
    for e in w.ents(Reg::R) {
        let _ = writeln!(o, "\n### {}\n", w.label(e.id));
        o.push_str("| state | anchor | change | cause |\n|---|---|---|---|\n");
        for st in e.states() {
            let cause = st.cause.map(|c| w.label(c)).unwrap_or_else(|| "—".into());
            let _ = writeln!(o, "| {}@{} | {} | {} | {} |", e.id, st.ver, st.pos, cell(&delta_str(w, &st.delta)), cell(&cause));
        }
    }

    // v2: intentions and beliefs — separate classes with states
    for (reg, title) in [(Reg::P, "Intentions (promises, plans, deals…)"), (Reg::B, "Beliefs")] {
        if w.ents(reg).next().is_none() {
            continue;
        }
        let _ = writeln!(o, "\n## {title}\n\n| id | whose | kind / claim | state history |\n|---|---|---|---|");
        for e in w.ents(reg) {
            let s = e.now();
            let owners = ids(w, &s.ids(SetF::Owners));
            let what = match (s.get(Field::Kind), s.get(Field::What)) {
                (Val::Known(k), Val::Known(t)) => format!("{k} {t}"),
                (_, t) => t.to_string(),
            };
            let mut extra = Vec::new();
            for (f, name) in [(Field::Target, "to whom"), (Field::About, "about"), (Field::Src, "from")] {
                if let Some(i) = s.id(f) {
                    extra.push(format!("{name} {}", w.label(i)));
                }
            }
            if let Val::Known(t) = s.get(Field::Truth) {
                extra.push(format!("truth: {t}"));
            }
            let hist: Vec<String> = w
                .history(e.id, Field::Stage)
                .into_iter()
                .map(|(st, v)| format!("{v} {}{}", st.pos, st.cause.map(|c| format!(" (by {})", w.label(c))).unwrap_or_default()))
                .collect();
            let what = if extra.is_empty() { what } else { format!("{what}; {}", extra.join("; ")) };
            let _ = writeln!(o, "| {} | {} | {} | {} |", e.id, cell(&owners), cell(&what), cell(&hist.join(" → ")));
        }
    }

    o.push_str("\n## Event chronology\n\n| id | anchor | event | participants (states) | motive / sense |\n|---|---|---|---|---|\n");
    for ev in w.events() {
        let what = match &ev.speech {
            Some(s) => {
                let mut x = format!("say {}", s.acts_str());
                if s.indirect {
                    x.push_str(", indirect");
                }
                match s.sincere {
                    Some(false) => x.push_str(", insincere"),
                    Some(true) => x.push_str(", sincere"),
                    None => {}
                }
                if s.narrator {
                    x.push_str(" (narrator)");
                }
                x
            }
            None => ev.verb.clone(),
        };
        let mut sense = Vec::new();
        // v2: negation, cause event, about what, story time, recollection
        if ev.x.neg {
            sense.push("**did not happen** (neg)".to_string());
        }
        if let Some(c) = ev.x.cause {
            sense.push(format!("because {}", w.label(c)));
        }
        if let Some(a) = ev.x.about {
            sense.push(format!("about {}", w.label(a)));
        }
        if let Some(t) = &ev.x.when {
            sense.push(format!("when: {t}"));
        }
        if let Some(b) = ev.x.before {
            sense.push(format!("before {b}"));
        }
        if let Some(m) = ev.speech.as_ref().and_then(|s| s.means.clone()) {
            sense.push(format!("means «{m}»"));
        }
        if let Some(y) = &ev.why {
            sense.push(format!("why «{y}»"));
        }
        if let Some(h) = &ev.how {
            sense.push(format!("how «{h}»"));
        }
        let sense = if sense.is_empty() { "—".into() } else { sense.join("; ") };
        let _ = writeln!(o, "| {} | {} | {} | {} | {} |", ev.id, ev.anchors(), cell(&what), cell(&w.roles_str(ev)), cell(&sense));
    }

    let n = t.sents.len() as u16;
    let c = w.context_at(n);
    let _ = writeln!(o, "\n## Working memory at the end (^s{n})\n");
    let list = |v: &[(Id, usize)]| if v.is_empty() { "—".into() } else { v.iter().map(|(i, k)| format!("{} ({k})", w.label(*i))).collect::<Vec<_>>().join(", ") };
    let _ = writeln!(o, "- main (attention = mentions): {}", list(&c.main));
    let _ = writeln!(o, "- secondary: {}", list(&c.secondary));
    let _ = writeln!(o, "- speaker: {}", c.speaker.map(|(s, e)| format!("{} ({e})", w.label(s))).unwrap_or_else(|| "—".into()));
    let _ = writeln!(o, "- addressee: {}", c.addressee.map(|a| w.label(a)).unwrap_or_else(|| "—".into()));
    let _ = writeln!(o, "- what: {}", c.what.map(|e| w.label(e)).unwrap_or_else(|| "—".into()));
    let _ = writeln!(o, "- why: {}", c.why.map(|(e, y)| format!("«{y}» ({e})")).unwrap_or_else(|| "—".into()));
    let _ = writeln!(o, "- where: {}", c.place.map(|l| w.place_chain(l)).unwrap_or_else(|| "—".into()));
    let _ = writeln!(o, "- when: {}", c.time.unwrap_or_else(|| "—".into()));

    o.push_str("\n## Gates\n\n");
    if fmt_errs.is_empty() && r.rejects.is_empty() && r.uncovered.is_empty() {
        o.push_str("All lines accepted, every sentence has a command.\n");
    }
    for e in fmt_errs {
        let _ = writeln!(o, "- format, line {}: `{}` — {}", e.no, e.raw, e.msg);
    }
    for (l, e) in &r.rejects {
        let _ = writeln!(o, "- line {}: `{}` — {e}", l.no, l.raw);
    }
    if !r.uncovered.is_empty() {
        let _ = writeln!(o, "- sentences without commands: {}", r.uncovered.iter().map(|s| format!("^s{s}")).collect::<Vec<_>>().join(" "));
    }
    o
}

/// md of the v3 narration layer: frames, story axis (explicit relations, anachronies), branches, claims with status
/// history, knowledge, reveals, warnings; below — the md world of the root (v2) and the frame worlds briefly.
pub fn story_md(r: &Run, fmt_errs: &[FormatErr], lines_total: usize) -> String {
    use crate::story::{FWorld, Why};
    let w = &r.world;
    let Some(st) = w.story() else { return world_md(r, fmt_errs, lines_total) };
    let mut o = format!("# World v3: {}\n\n", w.text.title);
    let _ = writeln!(
        o,
        "Sentences {}, LLM lines {lines_total}, accepted {}, rejected by format {}, rejected by gates {}, sentences without commands {}. Events {} (happened {}, in branches {}), frames {}, narration levels {}, claims {}, knowledge {}, reveals {}.\n",
        w.text.sents.len(),
        r.accepted,
        fmt_errs.len(),
        r.rejects.len(),
        r.uncovered.len(),
        st.ev.len(),
        st.ev.values().filter(|m| m.branch.as_str() == "real").count(),
        st.ev.values().filter(|m| m.branch.as_str() != "real").count(),
        st.frames.len(),
        st.narr_depth(),
        w.ents(Reg::K).count(),
        st.knows.len(),
        st.reveals.len()
    );
    o.push_str("## Narration frames\n\n| frame | narration level | nesting | narrator → listeners | world | sentences | events |\n|---|---|---|---|---|---|---|\n");
    for (i, f) in st.frames.iter().enumerate() {
        let kind = match f.kind {
            FWorld::Root => "root".to_string(),
            FWorld::Past => format!("same, past{}", f.anchor.map(|a| format!(" (before {})", w.label_any(a))).unwrap_or_default()),
            FWorld::Now => "same, present".into(),
            FWorld::New => format!("new (axis {})", f.wid),
            FWorld::Branch => format!("branch {}", f.branch),
        };
        let who = |x: crate::lang::Obs| match x {
            crate::lang::Obs::C(c) => w.label_any(c),
            crate::lang::Obs::Reader => "reader".into(),
            crate::lang::Obs::Narrator => "narrator".into(),
        };
        let _ = writeln!(
            o,
            "| {} | {} | {} | {} → {} | {kind} | {} | {} |",
            f.name(),
            st.narr_level(i),
            f.level,
            cell(&who(f.narrator)),
            cell(&f.to.iter().map(|x| who(*x)).collect::<Vec<_>>().join(", ")),
            f.spans.iter().map(|(a, b)| format!("^s{a}–{}", b.map(|x| format!("^s{x}")).unwrap_or("…".into()))).collect::<Vec<_>>().join(", "),
            st.ev.values().filter(|m| m.frame == i).count()
        );
    }
    let open: Vec<String> = st.stack.iter().skip(1).map(|i| st.frames[*i].name()).collect();
    if !open.is_empty() {
        let _ = writeln!(o, "\nNot closed by the end of the text: {}.", open.join(", "));
    }
    o.push_str("\n## Story time axis\n\n");
    let expl: Vec<String> = st
        .time
        .edges
        .iter()
        .filter(|e| e.alive && matches!(e.why, Why::Time(_) | Why::Place(_) | Why::Reveal(_) | Why::FrameBound(_)))
        .map(|e| {
            let p = |i: usize| match st.time.pts[i] {
                crate::story::Pt::S(x) | crate::story::Pt::E(x) => w.label_any(x),
                crate::story::Pt::FS(f) | crate::story::Pt::FE(f) => format!("frame {}", st.frames[f].name()),
            };
            format!("- {} {} {} ({})", p(e.u), if e.strict { "<" } else { "≤" }, p(e.v), e.why.show(&st.frames))
        })
        .collect();
    let chain = st.time.edges.iter().filter(|e| e.alive && matches!(e.why, Why::Chain(_))).count();
    let _ = writeln!(o, "Edges: narration line {chain}, explicit (placement, time, reveal, frame bounds) {}.\n", expl.len());
    for l in expl.iter().take(80) {
        let _ = writeln!(o, "{l}");
    }
    for (title, q) in [("Prolepses (told earlier, happened later)", "prolepses 8"), ("Analepses (told later, happened earlier)", "analepses 8")] {
        let a = crate::qa3::answer(w, q);
        let _ = writeln!(o, "\n**{title}:**\n");
        for l in a.text.lines() {
            let _ = writeln!(o, "- {l}");
        }
    }
    let abs: Vec<String> = st.ev.iter().filter(|(_, m)| !m.abs.is_empty()).map(|(id, m)| format!("{}: {}", w.label_any(*id), m.abs.iter().map(|(t, s)| format!("«{t}» ^s{s}")).collect::<Vec<_>>().join(", "))).collect();
    if !abs.is_empty() {
        let _ = writeln!(o, "\n**Time from the text:** {}", abs.join("; "));
    }
    o.push_str("\n## Reality branches (did not happen)\n\n");
    let br: Vec<String> = st.ev.iter().filter(|(_, m)| m.branch.as_str() != "real").map(|(id, _)| format!("- {}", crate::qa3::ev_str(w, *id))).collect();
    o.push_str(if br.is_empty() { "—\n" } else { "" });
    for l in br {
        let _ = writeln!(o, "{l}");
    }
    o.push_str("\n## Observer claims\n\n");
    let a = crate::qa3::answer(w, "claims");
    for l in a.text.lines() {
        let _ = writeln!(o, "- {}", l.trim_start_matches("not in state: "));
    }
    o.push_str("\n## Knowledge (know)\n\n");
    if st.knows.is_empty() {
        o.push_str("—\n");
    }
    for k in &st.knows {
        let who = match k.who {
            crate::lang::Obs::C(c) => w.label_any(c),
            crate::lang::Obs::Reader => "reader".into(),
            crate::lang::Obs::Narrator => "narrator".into(),
        };
        let _ = writeln!(o, "- {who} knows {} since ^s{}{} (line {})", w.label_any(k.fact), k.from, k.how.map(|h| format!(", {h}")).unwrap_or_default(), k.line);
    }
    o.push_str("\n## Reveals\n\n");
    let ids: BTreeSet<Id> = st.reveals.iter().map(|r| r.ev).collect();
    if ids.is_empty() {
        o.push_str("—\n");
    }
    for id in ids {
        let a = crate::qa3::answer(w, &format!("revealed {id}"));
        let _ = writeln!(o, "- {}", a.text);
    }
    if !st.warnings.is_empty() {
        o.push_str("\n## Narration layer warnings\n\n");
        for x in &st.warnings {
            let _ = writeln!(o, "- {}", cell(x));
        }
    }
    if !r.rejects.is_empty() || !fmt_errs.is_empty() {
        o.push_str("\n## Rejections\n\n| line | command | gate | reason |\n|---|---|---|---|\n");
        for e in fmt_errs {
            let _ = writeln!(o, "| {} | `{}` | format | {} |", e.no, cell(&e.raw), cell(&e.msg));
        }
        for (l, e) in &r.rejects {
            let _ = writeln!(o, "| {} | `{}` | {} | {} |", l.no, cell(&l.raw), e.gate.name(), cell(&e.msg));
        }
    }
    o.push_str("\n## Frame worlds\n\n");
    for f in &st.frames {
        if let Some(fw) = f.world.as_deref() {
            let own: Vec<String> = fw.ents(Reg::C).filter(|e| e.states().iter().any(|s| s.by != "project")).map(|e| fw.label(e.id)).collect();
            let _ = writeln!(o, "- {} (level {}): characters with states in the frame {}: {}", f.name(), f.level, own.len(), own.join(", "));
        }
    }
    o.push_str("\n---\n\n");
    o.push_str(&world_md(r, fmt_errs, lines_total));
    o
}
