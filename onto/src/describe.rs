//! v3: `onto describe <word> [--layer fable]` — concept description with proofs: IsA chain, parts, material, food,
//! habitat, uses, sizes and mass as distributions, risks, seed properties, and on the fable layer — character.
//! Every line carries its source.
//!
//! **Core first, then the graph.** With the core (`use_core`) fields come from `core::CORE`; if a core field is "unknown" — then the graph
//! (OEWN meronymy for parts and material). Without the core — only the graph and seeds: how v2 would describe it (the "before" measurement).

use std::fmt::Write as _;

use crate::core::{self, Field};
use crate::kb::*;
use crate::model::*;
use crate::query;

/// Description fields for completeness (report order).
pub const DESC_FIELDS: [&str; 13] = ["isa", "parts", "material", "food", "habitat", "uses", "mass", "height", "length", "lifespan", "speed", "risks", "props"];

pub struct Desc {
    pub text: String,
    pub category: &'static str,
    pub domain: &'static str,
    /// (field, applicable, known)
    pub fields: Vec<(&'static str, bool, bool)>,
    pub concept: Cid,
    pub core: Option<core::CoreId>,
}

impl Desc {
    pub fn filled(&self) -> usize {
        self.fields.iter().filter(|f| f.1 && f.2).count()
    }
    pub fn applicable(&self) -> usize {
        self.fields.iter().filter(|f| f.1).count()
    }
}

/// Lexfile → report category and core domain.
pub fn category(lexfile: &str) -> (&'static str, &'static str) {
    match lexfile {
        "noun.animal" => ("animals", "animal"),
        "noun.plant" => ("plants", "plant"),
        "noun.food" => ("food", "food"),
        "noun.artifact" => ("artifacts", "household"),
        "noun.person" => ("people", "role"),
        _ => ("other", "other"),
    }
}

/// Whether a field applies to the domain (like `coresel::applicable`; `isa` and `props` — always).
fn applicable(domain: &str, f: &str) -> bool {
    match f {
        "isa" | "props" => true,
        "parts" => crate::coresel::applicable(domain, "part"),
        "uses" => crate::coresel::applicable(domain, "use"),
        "risks" => crate::coresel::applicable(domain, "risk"),
        x => crate::coresel::applicable(domain, x),
    }
}

fn vals_line(vs: &[core::Val]) -> String {
    vs.iter().map(|v| format!("{} [{}]", if v.label.is_empty() { format!("Q{}", v.q) } else { v.label.to_string() }, v.src.show())).collect::<Vec<_>>().join("; ")
}

/// Word sense for the description: with the core — a core concept (domain by the lexfile context), otherwise — as bridge v1.
pub fn resolve(kb: &Kb, word: &str, ctx: Option<&str>, use_core: bool) -> Option<(Cid, String)> {
    let g = &kb.g;
    if let Some(c) = g.find_synset(word) {
        return Some((c, "synset id".into()));
    }
    if use_core {
        let ids = core::by_lemma(word);
        let pick = ids.iter().find(|id| ctx == Some(id.rec().domain.lexfile())).or_else(|| (ctx.is_none() || ids.len() == 1).then(|| ids.first()).flatten());
        if let Some(id) = pick {
            if let Some(c) = g.find_synset(id.rec().synset) {
                return Some((c, format!("core concept {:?}", id)));
            }
        }
    }
    let (_, c, m) = crate::bridge::map_class(g, word, ctx);
    c.map(|c| (c, if m { "first sense in the context lexfile".into() } else { "first sense".into() }))
}

pub fn describe(kb: &Kb, word: &str, ctx: Option<&str>, layer: LayerId, use_core: bool) -> Option<Desc> {
    let g = &kb.g;
    let (c, how) = resolve(kb, word, ctx, use_core)?;
    let cid = if use_core { core::of_concept(g, c) } else { None };
    let rec = cid.map(|x| x.rec());
    let (category, dom0) = category(g.lexfile_name(c));
    let domain = rec.map(|r| r.domain.name()).unwrap_or(dom0);
    let mut s = String::new();
    let mut fields: Vec<(&'static str, bool, bool)> = Vec::new();
    let _ = writeln!(s, "{word} → {} ({}; {how}) — «{}»", g.show(c), g.lexfile_name(c), g.gloss_of(c));
    // IsA: first ancestor up to the root
    let mut chain = vec![g.label(c)];
    let mut cur = c;
    for _ in 0..12 {
        let up = kb.up(cur);
        let Some(st) = up.first() else { break };
        cur = st.to;
        chain.push(g.label(cur));
    }
    let isa_ok = chain.len() > 1;
    let _ = writeln!(s, "is a kind of: {} [OEWN IsA]", chain.join(" → "));
    fields.push(("isa", true, isa_ok));
    // parts and material from the graph (as a fallback and for "before")
    let gparts = query::parts_of(kb, c);
    let graph_list = |rel: Rel| -> Vec<String> {
        gparts.iter().filter(|x| x.1 == rel && x.3.len() == 1).map(|(p, _, owner, _)| {
            let from = if owner.steps.is_empty() { String::new() } else { format!(" from {}", g.label(owner.end())) };
            format!("{}{from}", g.label(*p))
        }).take(12).collect()
    };
    let set_field = |s: &mut String, fields: &mut Vec<(&'static str, bool, bool)>, name: &'static str, uk: &str, f: Option<&Field<&'static [core::Val]>>, graph: Vec<String>| {
        let app = applicable(domain, name);
        let core_known = f.and_then(|f| f.known()).filter(|v| !v.is_empty());
        if let Some(vs) = core_known {
            let _ = writeln!(s, "{uk}: {} [core]", vals_line(vs));
            fields.push((name, app, true));
        } else if !graph.is_empty() {
            let _ = writeln!(s, "{uk}: {} [OEWN graph{}]", graph.join("; "), if f.is_some() { ", core — unknown" } else { "" });
            fields.push((name, app, true));
        } else if !app {
            let _ = writeln!(s, "{uk}: not applicable ({domain})");
            fields.push((name, false, false));
        } else {
            let why = match f {
                Some(Field::Unknown(w)) => format!("core: {w}"),
                _ => "neither in the core nor in the graph".into(),
            };
            let _ = writeln!(s, "{uk}: unknown ({why})");
            fields.push((name, true, false));
        }
    };
    set_field(&mut s, &mut fields, "parts", "parts", rec.map(|r| &r.parts), [graph_list(Rel::PartOf), graph_list(Rel::MemberOf)].concat());
    set_field(&mut s, &mut fields, "material", "material", rec.map(|r| &r.material), graph_list(Rel::MadeOf));
    set_field(&mut s, &mut fields, "food", "food", rec.map(|r| &r.food), vec![]);
    set_field(&mut s, &mut fields, "habitat", "habitat", rec.map(|r| &r.habitat), vec![]);
    set_field(&mut s, &mut fields, "uses", "uses", rec.map(|r| &r.uses), vec![]);
    for (name, uk) in [("mass", "mass"), ("height", "height"), ("length", "length"), ("lifespan", "lifespan"), ("speed", "speed")] {
        let app = applicable(domain, name);
        let f = rec.map(|r| match name {
            "mass" => &r.mass,
            "height" => &r.height,
            "length" => &r.length,
            "lifespan" => &r.lifespan,
            _ => &r.speed,
        });
        match f {
            Some(Field::Known(d)) => {
                let _ = writeln!(s, "{uk}: {} [core: {}]", d.show(), d.src.show());
                fields.push((name, app, true));
            }
            _ if !app => fields.push((name, false, false)),
            Some(Field::Unknown(w)) => {
                let _ = writeln!(s, "{uk}: unknown (core: {w})");
                fields.push((name, true, false));
            }
            _ => {
                let _ = writeln!(s, "{uk}: unknown");
                fields.push((name, true, false));
            }
        }
    }
    let app = applicable(domain, "risks");
    match rec.map(|r| &r.risks) {
        Some(Field::Known(rs)) if rs.is_empty() => {
            let _ = writeln!(s, "risks: none notable [core: LLM assessment]");
            fields.push(("risks", app, true));
        }
        Some(Field::Known(rs)) => {
            let v: Vec<String> = rs.iter().map(|r| format!("{} [{}]", r.show(), r.src.show())).collect();
            let _ = writeln!(s, "risks: {}", v.join("; "));
            fields.push(("risks", app, true));
        }
        _ => {
            let _ = writeln!(s, "risks: unknown");
            fields.push(("risks", app, false));
        }
    }
    // seed properties on the layer
    let mut props = Vec::new();
    let mut character = Vec::new();
    for p in Prop::ALL {
        if let PropAnswer::Known { value, win, layer: l, .. } = kb.prop(c, p, layer) {
            let a = &kb.asserts[win];
            let line = format!("{}={} ({} {} on {}, source: {})", p.name(), if value { "yes" } else { "no" }, a.id, g.label(a.concept), kb.layers[l as usize].name, a.source.chars().take(60).collect::<String>());
            if matches!(p, Prop::Cunning | Prop::Vain | Prop::Greedy | Prop::Industrious | Prop::Wise) {
                character.push(line);
            } else {
                props.push(line);
            }
        }
    }
    if props.is_empty() {
        let _ = writeln!(s, "properties: no seed applies");
    } else {
        let _ = writeln!(s, "properties: {}", props.join("; "));
    }
    fields.push(("props", true, !props.is_empty()));
    if layer != 0 {
        if character.is_empty() {
            let _ = writeln!(s, "character (layer {}): no seeds", kb.layers[layer as usize].name);
        } else {
            let _ = writeln!(s, "character (layer {}): {}", kb.layers[layer as usize].name, character.join("; "));
        }
    }
    Some(Desc { text: s, category, domain, fields, concept: c, core: cid })
}

/// Classes from fairy-tale and fable worlds: (class, type context lexfile), without duplicates.
pub fn world_classes(ents: &[crate::bridge::Ent]) -> Vec<(String, Option<&'static str>)> {
    let mut m: std::collections::BTreeMap<String, Option<&'static str>> = Default::default();
    for e in ents {
        if let Some(c) = &e.class {
            m.entry(c.clone()).or_insert_with(|| crate::bridge::type_lexfile(e.reg, &e.ty));
        }
    }
    m.into_iter().collect()
}

/// Completeness by category: category → (classes, filled, applicable, unknown).
pub fn completeness(ds: &[(String, Desc)]) -> String {
    let mut m: std::collections::BTreeMap<&str, [usize; 4]> = Default::default();
    let mut per_field: std::collections::BTreeMap<&str, [usize; 2]> = Default::default();
    for (_, d) in ds {
        for key in [d.category, "total"] {
            let e = m.entry(key).or_default();
            e[0] += 1;
            e[1] += d.filled();
            e[2] += d.applicable();
            e[3] += d.applicable() - d.filled();
        }
        for (f, app, known) in &d.fields {
            if *app {
                let e = per_field.entry(f).or_default();
                e[0] += *known as usize;
                e[1] += 1;
            }
        }
    }
    let mut s = String::from("| category | classes | fields filled per class | \"unknown\" share |\n|---|---:|---:|---:|\n");
    for (k, v) in &m {
        let _ = writeln!(s, "| {k} | {} | {:.1} of {:.1} | {:.0}% |", v[0], v[1] as f64 / v[0].max(1) as f64, v[2] as f64 / v[0].max(1) as f64, 100.0 * v[3] as f64 / v[2].max(1) as f64);
    }
    s.push_str("\n| field | filled / applicable |\n|---|---:|\n");
    for f in DESC_FIELDS {
        if let Some(v) = per_field.get(f) {
            let _ = writeln!(s, "| {f} | {}/{} ({:.0}%) |", v[0], v[1], 100.0 * v[0] as f64 / v[1].max(1) as f64);
        }
    }
    s
}
