//! Bridge to the story world (v1 — read-only): object classes from fable and fairy-tale worlds (`cmds-*.txt`
//! of the `world` crate) → world-model concepts; questions about fables via inheritance.
//!
//! A story individual (Crow1) does not copy the concept but refers to it: class `crow` and type `animal`
//! give context — the word sense from the lexicographer file `noun.animal`. The bridge does not change the
//! files of `world/`.

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::kb::*;
use crate::model::*;
use crate::query::{self, Answer, Verdict};

#[derive(Clone, Debug)]
pub struct Ent {
    pub tale: String,
    pub reg: char,
    pub id: String,
    pub ty: String,
    pub class: Option<String>,
    pub name: Option<String>,
}

/// Value of a key in a line of the change language: `key=value` or `key="…"`.
fn field(line: &str, key: &str) -> Option<String> {
    let pat = format!(" {key}=");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    if let Some(r) = rest.strip_prefix('"') {
        Some(r[..r.find('"')?].to_string())
    } else {
        Some(rest.split_whitespace().next()?.to_string())
    }
}

/// Tale name from the command file name.
pub fn tale_of(file: &str) -> String {
    let stem = file.trim_start_matches("cmds-").trim_end_matches(".txt");
    match stem {
        "v2-wen" => "how-an-old-man-lost-his-wen".into(),
        "v3-merchant" => "arabian-nights".into(),
        s if s.starts_with("2451") => "2451".into(),
        s => s.to_string(),
    }
}

/// Entities from command files (files duplicated by content — once).
pub fn read_world(files: &[PathBuf]) -> Result<(Vec<Ent>, Vec<String>)> {
    let mut seen = HashSet::new();
    let mut ents = Vec::new();
    let mut used = Vec::new();
    for f in files {
        let text = std::fs::read_to_string(f).with_context(|| format!("cannot read {}", f.display()))?;
        if !seen.insert(crate::eval::sha256_hex(text.as_bytes())) {
            continue;
        }
        let tale = tale_of(&f.file_name().unwrap_or_default().to_string_lossy());
        used.push(format!("{} → tale:{tale}", f.display()));
        for l in text.lines() {
            let reg = match l.split_whitespace().next() {
                Some("char") => 'C',
                Some("obj") => 'O',
                Some("loc") => 'L',
                _ => continue,
            };
            let Some(ty) = field(l, "type") else { continue };
            let id = l.split_whitespace().nth(1).unwrap_or("").to_string();
            ents.push(Ent { tale: tale.clone(), reg, id, ty, class: field(l, "class"), name: field(l, "name") });
        }
    }
    Ok((ents, used))
}

/// World object type → OEWN lexicographer file as context for sense choice.
pub fn type_lexfile(reg: char, ty: &str) -> Option<&'static str> {
    Some(match (reg, ty) {
        ('C', "human") | ('C', "deity") => "noun.person",
        ('C', "animal") | ('O', "animal") => "noun.animal",
        ('O', "food") | ('O', "drink") => "noun.food",
        ('O', "plant") | ('O', "tree") | ('L', "tree") => "noun.plant",
        ('O', "body_part") => "noun.body",
        ('O', "stone") => "noun.object",
        ('O', "water") => "noun.substance",
        ('O', "money") => "noun.possession",
        ('O', "tool" | "rope" | "net" | "weapon" | "container" | "treasure" | "clothing" | "trap" | "vehicle" | "jewel" | "ring" | "key" | "letter" | "book" | "instrument" | "furniture" | "building") => "noun.artifact",
        ('L', "house" | "palace" | "castle" | "mill" | "church" | "barn" | "stable" | "inn" | "shop" | "hut" | "room" | "hall" | "kitchen" | "bridge" | "ship" | "boat" | "well") => "noun.artifact",
        ('L', "mountain" | "hill" | "river" | "lake" | "sea" | "shore" | "island" | "cave") => "noun.object",
        _ => return None,
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Tier {
    /// class as is (underscores → spaces)
    Exact,
    /// spelling variant: hyphen → space or joined
    Variant,
    /// head word only (the last one): `demon_chief` → `chief`
    Head,
    Missing,
}

/// Entity class → concept: match tier, concept, whether the context lexfile matched.
pub fn map_class(g: &Graph, class: &str, ctx: Option<&str>) -> (Tier, Option<Cid>, bool) {
    let pick = |f: &query::Found| -> (Cid, bool) {
        if let Some(lf) = ctx.and_then(lexfile_id) {
            if let Some(&c) = f.senses.iter().find(|&&c| g.lexfile[c as usize] == lf) {
                return (c, true);
            }
        }
        (f.senses[0], false)
    };
    if let Some(f) = query::lookup(g, class, Pos::Noun) {
        let (c, m) = pick(&f);
        return (Tier::Exact, Some(c), m);
    }
    for v in [class.replace('-', " "), class.replace(['-', '_'], "")] {
        if let Some(f) = query::lookup(g, &v, Pos::Noun) {
            let (c, m) = pick(&f);
            return (Tier::Variant, Some(c), m);
        }
    }
    if let Some(head) = class.rsplit(['_', '-', ' ']).next().filter(|h| *h != class) {
        if let Some(f) = query::lookup(g, head, Pos::Noun) {
            let (c, m) = pick(&f);
            return (Tier::Head, Some(c), m);
        }
    }
    (Tier::Missing, None, false)
}

/// Class coverage: md report.
pub fn coverage(kb: &Kb, ents: &[Ent], used: &[String]) -> String {
    let g = &kb.g;
    let mut classes: BTreeMap<String, (char, String, usize)> = BTreeMap::new();
    let (mut with_class, mut without) = (0usize, 0usize);
    for e in ents {
        match &e.class {
            Some(c) => {
                with_class += 1;
                classes.entry(c.clone()).or_insert((e.reg, e.ty.clone(), 0)).2 += 1;
            }
            None => without += 1,
        }
    }
    let mut tiers: BTreeMap<Tier, Vec<String>> = BTreeMap::new();
    let (mut ctx_total, mut ctx_match) = (0usize, 0usize);
    let mut sample = Vec::new();
    for (c, (reg, ty, n)) in &classes {
        let ctx = type_lexfile(*reg, ty);
        let (t, cid, m) = map_class(g, c, ctx);
        if ctx.is_some() && cid.is_some() {
            ctx_total += 1;
            ctx_match += m as usize;
        }
        tiers.entry(t).or_default().push(c.clone());
        if let Some(cid) = cid {
            if sample.len() < 400 {
                sample.push(format!("| {c} | {reg} {ty} | {n} | {:?} | {} | {} |", t, g.show(cid), g.lexfile_name(cid)));
            }
        }
    }
    let total = classes.len();
    let found = total - tiers.get(&Tier::Missing).map(|v| v.len()).unwrap_or(0);
    let mut s = String::from("## Coverage of story-world classes\n\nCommand files (duplicates by content — once):\n\n");
    for u in used {
        let _ = writeln!(s, "- `{u}`");
    }
    let _ = writeln!(
        s,
        "\nEntities (`char`/`obj`/`loc`): {} — with a class {with_class}, with a type only {without}.\n\n**Distinct classes {total}, found {found} ({:.0}%):**\n",
        ents.len(),
        100.0 * found as f64 / total.max(1) as f64
    );
    for (t, v) in &tiers {
        let _ = writeln!(s, "- {:?}: {} ({:.0}%)", t, v.len(), 100.0 * v.len() as f64 / total.max(1) as f64);
    }
    let _ = writeln!(s, "\nThe type context (`type=animal` → `noun.animal` etc.) gave a sense in its own lexfile for {ctx_match} of {ctx_total} found classes that have context.\n");
    if let Some(m) = tiers.get(&Tier::Missing) {
        let _ = writeln!(s, "Not found: {}\n", m.join(", "));
    }
    if let Some(h) = tiers.get(&Tier::Head) {
        let _ = writeln!(s, "Head word only: {}\n", h.join(", "));
    }
    s.push_str("| class | type | entities | tier | concept | lexfile |\n|---|---|---:|---|---|---|\n");
    for l in sample {
        s.push_str(&l);
        s.push('\n');
    }
    s
}

/// Question about a fable: entity from the world → concept → query on the tale layer.
pub fn answer_question(kb: &Kb, ents: &[Ent], tale: &str, q: &str) -> Answer {
    let g = &kb.g;
    let t = q.trim().trim_end_matches('?').to_string();
    let words: Vec<&str> = t.split_whitespace().collect();
    let bad = |m: &str| Answer { verdict: Verdict::Unknown, values: vec![], lines: vec![m.to_string()] };
    // «Can the X V» · «Is the X a Y» · «Is the X ADJ» · «Does the X have a Y» · «Does the X exist»
    let (x, rest): (&str, Vec<&str>) = match words.as_slice() {
        [_, "the", x, rest @ ..] => (x, rest.to_vec()),
        _ => return bad("question form not recognized"),
    };
    let Some(e) = ents.iter().find(|e| e.tale == tale && (e.class.as_deref() == Some(x) || e.name.as_deref().is_some_and(|n| n.to_lowercase().contains(x)))) else {
        return bad(&format!("the world of tale {tale} has no entity «{x}»"));
    };
    let class = e.class.clone().unwrap_or_else(|| e.ty.clone());
    let ctx = type_lexfile(e.reg, &e.ty);
    let (tier, cid, m) = map_class(g, &class, ctx);
    let Some(mut c) = cid else { return bad(&format!("class «{class}» not found in the world model")) };
    let layer = kb.layer_id(&format!("tale:{tale}")).unwrap_or(0);
    // one sense per text: if a seed of this tale's layers (tale, collection, genre) is on another sense
    // of the same word in the same lexfile — take it (second round)
    let mut seed_note = String::new();
    if let Some(f) = query::lookup(g, &class.replace('_', " "), Pos::Noun) {
        let chain: Vec<LayerId> = kb.layer_chain(layer).into_iter().filter(|l| *l != 0).collect();
        let same_lf = |x: Cid| g.lexfile[x as usize] == g.lexfile[c as usize];
        if let Some(a) = kb.asserts.iter().find(|a| chain.contains(&a.layer) && a.concept != c && f.senses.contains(&a.concept) && same_lf(a.concept)) {
            seed_note = format!("; sense from seed {} of layer {} (one sense per text), not {}", a.id, kb.layers[a.layer as usize].name, g.show(c));
            c = a.concept;
        }
    }
    let head = format!(
        "tale world {tale}: {} {} type={} class={class} → {} ({:?}{})",
        e.id,
        e.name.as_deref().unwrap_or(""),
        e.ty,
        g.show(c),
        tier,
        if m { format!(", sense from {} by type", ctx.unwrap_or("")) } else { ", first sense".into() }
    ) + &seed_note;
    let sid = g.cid_str(c);
    let mut a = match (words[0].to_lowercase().as_str(), rest.as_slice()) {
        ("can", [v]) => match Prop::parse(v) {
            Some(p) => query::prop(kb, &sid, p, layer),
            None => bad(&format!("verb «{v}» is not a property")),
        },
        ("is", [_art, y]) => query::isa(kb, &sid, y),
        // «Is the meat food?» — no article: a property if the word is a property, otherwise IsA
        ("is", [adj]) => match Prop::parse(adj) {
            Some(p) => query::prop(kb, &sid, p, layer),
            None => query::isa(kb, &sid, adj),
        },
        ("does", ["have", _art, y]) => query::has_part(kb, &sid, y, Rel::PartOf),
        ("does", ["exist"]) => query::prop(kb, &sid, Prop::Exists, layer),
        _ => bad("question form not recognized"),
    };
    a.lines.insert(0, head);
    a
}

pub fn world_files(repo: &Path, runs: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    let mut add = |dir: PathBuf| {
        if let Ok(rd) = std::fs::read_dir(&dir) {
            let mut fs: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("cmds-") && n.to_string_lossy().ends_with(".txt"))).collect();
            fs.sort();
            v.extend(fs);
        }
    };
    add(repo.join("world/data"));
    for d in ["world-2026-09-26", "world-v2-2026-09-26", "world-v3-2026-09-26"] {
        add(runs.join(d));
    }
    v
}
