//! The global graph of good ideas: chains of decisions that held, dormant hypotheses, principles of the gathered
//! worlds — shared by every state and every epoch (`<hot>/ideas.tsv`).
//!
//! A good chain of decisions should not stay in the epoch it was found in: when the circumstances change (a new state,
//! new code, new worlds), the ideas that held before are offered again first, as good thoughts carried over. Dormant
//! hypotheses wait in the same graph for their circumstances to change. The principles of the worlds are there to be
//! applied to what is new. Ideas are linked by the words they share (the same plain words as the nerve graph), so a
//! text calls up the ideas near it. Deterministic: ordered maps, a fixed order of ties.
//!
//!   world ideas                  — the graph: how many ideas of each kind
//!   world ideas "<text>"         — the ideas a text calls up

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;

/// One idea: (time, kind, text, evidence, circumstances, source).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Idea {
    pub time: u64,
    pub kind: String,
    pub text: String,
    pub evidence: String,
    pub ctx: String,
    pub source: String,
}

fn path() -> std::path::PathBuf {
    crate::state::root().join("ideas.tsv")
}

pub fn read() -> Vec<Idea> {
    std::fs::read_to_string(path())
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let c: Vec<&str> = l.splitn(6, '\t').collect();
            (c.len() == 6).then(|| Idea { time: c[0].parse().unwrap_or(0), kind: c[1].into(), text: c[2].into(), evidence: c[3].into(), ctx: c[4].into(), source: c[5].into() })
        })
        .collect()
}

/// Add an idea (once per kind and text; a known one gets its newest evidence and circumstances).
pub fn record(kind: &str, text: &str, evidence: &str, ctx: &str, source: &str) {
    let clean = |x: &str| x.replace(['\t', '\n'], " ");
    let mut v = read();
    match v.iter_mut().find(|i| i.kind == kind && i.text == clean(text)) {
        Some(i) => {
            i.evidence = clean(evidence);
            i.ctx = ctx.to_string();
            i.time = crate::talk::now();
        }
        None => v.push(Idea { time: crate::talk::now(), kind: kind.into(), text: clean(text), evidence: clean(evidence), ctx: ctx.into(), source: clean(source) }),
    }
    let body: String = std::iter::once("# time\tkind\ttext\tevidence\tcircumstances\tsource\n".to_string()).chain(v.iter().map(|i| format!("{}\t{}\t{}\t{}\t{}\t{}\n", i.time, i.kind, i.text, i.evidence, i.ctx, i.source))).collect();
    let _ = crate::store::write_atomic(&path(), body.as_bytes());
}

/// The principles of the gathered worlds as ideas (the best-supported of each part, as in the brains), recorded once.
pub fn seed_principles() -> usize {
    let have: BTreeSet<String> = read().into_iter().filter(|i| i.kind == "principle").map(|i| i.text).collect();
    let dir = crate::state::root().join("worlds").join("shared");
    let mut dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&dir).map(|r| r.flatten().map(|e| e.path()).collect()).unwrap_or_default();
    dirs.sort();
    let mut n = 0;
    for d in dirs {
        let topic = d.file_name().map(|x| x.to_string_lossy().split('-').next().unwrap_or("").to_string()).unwrap_or_default();
        let Ok(w) = std::fs::read_to_string(d.join(format!("{topic}.world.md"))) else { continue };
        let (body, _) = crate::worldbook::brain_body(&w);
        for l in body.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            let (text, src) = l.rsplit_once(" (").unwrap_or((l, ""));
            if !have.contains(text) {
                record("principle", text, src.trim_end_matches(')'), "", &format!("the world of {topic}"));
                n += 1;
            }
        }
    }
    n
}

fn words(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_alphanumeric()).filter_map(crate::nerves::norm).collect()
}

/// The ideas a text calls up: the shared words weighed by their rarity among the ideas (a word in every idea says
/// nothing, a rare one says much — integer weights), then the kind (held, principle, dormant), then the newest.
pub fn related(text: &str, k: usize) -> Vec<Idea> {
    let asked = words(text);
    let all = read();
    let mut df: BTreeMap<String, u64> = BTreeMap::new();
    for i in &all {
        for w in words(&i.text) {
            *df.entry(w).or_default() += 1;
        }
    }
    let rank = |kind: &str| match kind { "held" => 0, "principle" => 1, "derived" => 2, _ => 3 };
    let mut v: Vec<(u64, Idea)> = all.into_iter().map(|i| (words(&i.text).intersection(&asked).map(|w| 1_000_000 / df.get(w).copied().unwrap_or(1).max(1)).sum::<u64>(), i)).filter(|x| x.0 > 0).collect();
    v.sort_by(|a, b| b.0.cmp(&a.0).then(rank(&a.1.kind).cmp(&rank(&b.1.kind))).then(b.1.time.cmp(&a.1.time)).then(a.1.text.cmp(&b.1.text)));
    v.into_iter().take(k).map(|x| x.1).collect()
}

/// The good thoughts of earlier epochs: ideas that held in other circumstances than these.
pub fn carried_over(ctx: &str) -> Vec<Idea> {
    read().into_iter().filter(|i| i.kind == "held" && !i.ctx.is_empty() && i.ctx != ctx).collect()
}

/// `world ideas ["<text>"]`.
pub fn main_args(args: &[String]) -> Result<()> {
    let seeded = seed_principles();
    let v = read();
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for i in &v {
        *kinds.entry(i.kind.as_str()).or_default() += 1;
    }
    println!("ideas: {} ({}){}", v.len(), kinds.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", "), if seeded > 0 { format!("; {seeded} principles added now") } else { String::new() });
    let text = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    if !text.is_empty() {
        for i in related(&text, 8) {
            println!("  [{}] {} — {} ({})", i.kind, i.text, i.evidence, i.source);
        }
    }
    Ok(())
}
