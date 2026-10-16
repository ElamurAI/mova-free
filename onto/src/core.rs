//! v3: the compiled world-model core — Rust types that `build.rs` (the build script at the crate root) stitches from
//! `data/core.tsv` and `data/core-props.tsv` (`onto core-select`). Core concepts are created at compile time:
//! `CoreId::Fox`, `CORE[..]` are static records; nothing is built at run time.
//!
//! - Concept sets (food, habitat, parts, material, uses) — `Val`: Wikidata item, label, OEWN synset
//!   (if mapped), link to a core concept.
//! - Sizes, mass, lifespan, speed — distributions `Dist {mean, sd, min, max, n, unit}`.
//! - Risks — probabilities `Risk {act, harm, p}` (an order-of-magnitude estimate from the LLM).
//! - Every value has a `Source`: Q, P, rank, steps to the anchor, confidence, confirmation.
//! - Every field is `Known`, `Unknown(reason)` or `NotApplicable`: never empty (completeness gate in `build.rs`).
//!
//! **Query order — core first, then the graph.** A contradiction between core and graph is a gate (`check`): refusal at
//! load time showing both sides, not a silent choice.

use crate::kb::*;
use crate::model::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Domain {
    Animal,
    Food,
    Plant,
    Household,
    Family,
    Role,
    Magical,
}

impl Domain {
    pub fn name(self) -> &'static str {
        match self {
            Domain::Animal => "animal",
            Domain::Food => "food",
            Domain::Plant => "plant",
            Domain::Household => "household",
            Domain::Family => "family",
            Domain::Role => "role",
            Domain::Magical => "magical",
        }
    }
    /// OEWN lexicographer file of the domain.
    pub fn lexfile(self) -> &'static str {
        match self {
            Domain::Animal => "noun.animal",
            Domain::Food => "noun.food",
            Domain::Plant => "noun.plant",
            Domain::Household => "noun.artifact",
            Domain::Family | Domain::Role | Domain::Magical => "noun.person",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rank {
    Preferred,
    Normal,
    Deprecated,
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Confirm {
    /// the LLM said "yes"
    VmmYes,
    /// confidence ≥ 0.7, the LLM refined it (note)
    VmmRefine,
    /// confidence ≥ 0.7, the LLM did not look
    Confident,
    /// LLM estimate (risks)
    VmmEstimate,
}

impl Confirm {
    pub fn uk(self) -> &'static str {
        match self {
            Confirm::VmmYes => "LLM: yes",
            Confirm::VmmRefine => "LLM: refined",
            Confirm::Confident => "confident ≥ 0.7",
            Confirm::VmmEstimate => "LLM estimate",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Source {
    /// Wikidata item carrying the claim (0 — none)
    pub q: u32,
    /// Wikidata property (0 — none)
    pub p: u16,
    pub rank: Rank,
    /// steps from the anchor to the ancestor with the claim
    pub steps: u8,
    pub conf: f64,
    pub confirm: Confirm,
    pub note: &'static str,
}

impl Source {
    pub fn show(&self) -> String {
        let mut s = String::new();
        if self.q != 0 {
            s.push_str(&format!("Q{}", self.q));
        }
        if self.p != 0 {
            s.push_str(&format!(" P{}", self.p));
        }
        match self.rank {
            Rank::Preferred => s.push_str(" preferred"),
            Rank::Normal => s.push_str(" normal"),
            Rank::Deprecated => s.push_str(" deprecated"),
            Rank::None => {}
        }
        if self.steps > 0 {
            s.push_str(&format!(", {} steps from the anchor", self.steps));
        }
        if self.conf > 0.0 {
            s.push_str(&format!(", confidence {:.2}", self.conf));
        }
        s.push_str(&format!(", {}", self.confirm.uk()));
        if !self.note.is_empty() {
            s.push_str(&format!(" — {}", self.note));
        }
        s.trim_start_matches([' ', ',']).to_string()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Val {
    pub q: u32,
    pub label: &'static str,
    /// OEWN synset of the value (strict v2 mapping) or ""
    pub synset: &'static str,
    pub core: Option<CoreId>,
    pub src: Source,
}

#[derive(Clone, Copy, Debug)]
pub struct Dist {
    pub mean: f64,
    /// None — σ unknown (a single value)
    pub sd: Option<f64>,
    pub min: f64,
    pub max: f64,
    pub n: u32,
    pub unit: &'static str,
    pub src: Source,
}

impl Dist {
    pub fn show(&self) -> String {
        let sd = self.sd.map(|s| format!(" ± {}", sig(s))).unwrap_or_else(|| " (σ unknown)".into());
        format!("≈ {} {}{sd}, {}–{}, n = {}", sig(self.mean), self.unit, sig(self.min), sig(self.max), self.n)
    }
}

fn sig(x: f64) -> String {
    if x == 0.0 {
        return "0".into();
    }
    let d = (3 - x.abs().log10().floor() as i32 - 1).clamp(0, 6) as usize;
    let s = format!("{x:.d$}");
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    Touch,
    Approach,
    Eat,
    Use,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Harm {
    Bite,
    Scratch,
    Sting,
    Venom,
    Kick,
    Crush,
    Cut,
    Burn,
    Poison,
    Shock,
    Allergy,
    Fall,
    Drown,
}

#[derive(Clone, Copy, Debug)]
pub struct Risk {
    pub act: Act,
    pub harm: Harm,
    /// probability of harm per such contact (order of magnitude)
    pub p: f64,
    pub src: Source,
}

impl Risk {
    pub fn show(&self) -> String {
        format!("{:?} → {:?}: p ≈ {}", self.act, self.harm, sig(self.p)).to_lowercase()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Field<T: 'static> {
    Known(T),
    Unknown(&'static str),
    NotApplicable,
}

impl<T> Field<T> {
    pub fn known(&self) -> Option<&T> {
        match self {
            Field::Known(v) => Some(v),
            _ => None,
        }
    }
    pub fn is_known(&self) -> bool {
        matches!(self, Field::Known(_))
    }
    pub fn is_applicable(&self) -> bool {
        !matches!(self, Field::NotApplicable)
    }
}

pub struct CoreRec {
    pub id: CoreId,
    pub lemma: &'static str,
    pub domain: Domain,
    pub synset: &'static str,
    pub food: Field<&'static [Val]>,
    pub habitat: Field<&'static [Val]>,
    pub parts: Field<&'static [Val]>,
    pub material: Field<&'static [Val]>,
    pub uses: Field<&'static [Val]>,
    pub mass: Field<Dist>,
    pub height: Field<Dist>,
    pub length: Field<Dist>,
    pub lifespan: Field<Dist>,
    pub speed: Field<Dist>,
    pub risks: Field<&'static [Risk]>,
}

include!(concat!(env!("OUT_DIR"), "/core_gen.rs"));

/// Field names for humans and reports.
pub const FIELD_NAMES: [(&str, &str); 11] = [
    ("food", "food"),
    ("habitat", "habitat"),
    ("parts", "parts"),
    ("material", "material"),
    ("uses", "uses"),
    ("mass", "mass"),
    ("height", "height"),
    ("length", "length"),
    ("lifespan", "lifespan"),
    ("speed", "speed"),
    ("risks", "risks"),
];

impl CoreId {
    pub fn rec(self) -> &'static CoreRec {
        &CORE[self as usize]
    }
}

/// Core concepts by lemma (several — fish the animal and fish the food).
pub fn by_lemma(lemma: &str) -> Vec<CoreId> {
    let l = lemma.replace('_', " ").to_lowercase();
    CORE.iter().filter(|r| r.lemma == l).map(|r| r.id).collect()
}

pub fn by_synset(id: &str) -> Option<CoreId> {
    CORE.iter().find(|r| r.synset == id).map(|r| r.id)
}

/// Core concept by graph concept.
pub fn of_concept(g: &Graph, c: Cid) -> Option<CoreId> {
    (g.src[c as usize] == Src::Oewn).then(|| by_synset(&g.cid_str(c))).flatten()
}

/// Field state as (known, applicable) — for completeness.
pub fn field_state(r: &CoreRec, i: usize) -> (bool, bool) {
    let f = |x: &Field<&[Val]>| (x.is_known(), x.is_applicable());
    let d = |x: &Field<Dist>| (x.is_known(), x.is_applicable());
    match i {
        0 => f(&r.food),
        1 => f(&r.habitat),
        2 => f(&r.parts),
        3 => f(&r.material),
        4 => f(&r.uses),
        5 => d(&r.mass),
        6 => d(&r.height),
        7 => d(&r.length),
        8 => d(&r.lifespan),
        9 => d(&r.speed),
        _ => (r.risks.is_known(), r.risks.is_applicable()),
    }
}

/// Synset (key, part of speech) → concept: fast lookup instead of the linear `Graph::find_synset`.
pub type SynMap = std::collections::HashMap<(u32, u8), Cid>;

pub fn synset_map(g: &Graph) -> SynMap {
    (0..g.n_concepts()).filter(|&i| g.src[i] == Src::Oewn).map(|i| ((g.key[i], g.pos[i] as u8), i as Cid)).collect()
}

pub fn find(m: &SynMap, id: &str) -> Option<Cid> {
    let rest = id.strip_prefix("oewn-")?;
    let (num, p) = rest.split_once('-')?;
    m.get(&(num.parse().ok()?, Pos::from_wn(p)? as u8)).copied()
}

/// What the domain requires of graph seeds on the real layer: (property, forbidden value).
fn domain_expect(d: Domain) -> &'static [(Prop, bool)] {
    match d {
        Domain::Animal => &[(Prop::Animate, false), (Prop::Living, false), (Prop::Human, true)],
        Domain::Plant => &[(Prop::Living, false), (Prop::Animate, true)],
        Domain::Food => &[(Prop::Edible, false)],
        Domain::Household => &[(Prop::Living, true), (Prop::Animate, true)],
        Domain::Family | Domain::Role => &[(Prop::Human, false), (Prop::Animate, false)],
        Domain::Magical => &[],
    }
}

/// **"Core ↔ graph" gate.** For every core concept:
/// - the synset is in the graph and the lemma is its member (a lexfile outside the domain is only a warning, `lexfile_warnings`: coin in OEWN is
///   noun.possession, not an artifact, and that is not a factual contradiction);
/// - graph seeds on the real layer do not deny the domain: an animal is not `Animate=no`, food is not `Edible=no`, an object is not
///   `Living=yes`…;
/// - concept values of the core (part, material) do not contradict the graph: a part is not the concept itself or its ancestor.
///
/// Returns contradictions with both sides; empty means green.
pub fn check(kb: &Kb) -> Vec<String> {
    let g = &kb.g;
    let mut bad = Vec::new();
    let sm = synset_map(g);
    let idx: std::collections::HashMap<String, Cid> = CORE.iter().filter_map(|r| find(&sm, r.synset).map(|c| (r.synset.to_string(), c))).collect();
    for r in CORE.iter() {
        let Some(&c) = idx.get(r.synset) else {
            bad.push(format!("{:?}: synset {} is not in the graph", r.id, r.synset));
            continue;
        };
        if !g.members(c).iter().any(|w| g.word(*w).eq_ignore_ascii_case(r.lemma)) {
            bad.push(format!("{:?}: lemma \"{}\" is not a member of {} (graph: {})", r.id, r.lemma, r.synset, g.label(c)));
        }
        for &(p, forbidden) in domain_expect(r.domain) {
            if let PropAnswer::Known { value, win, .. } = kb.prop(c, p, 0) {
                if value == forbidden {
                    let a = &kb.asserts[win];
                    bad.push(format!("{:?}: core — domain {}, graph — seed {} {} {}={} ({})", r.id, r.domain.name(), a.id, g.show(a.concept), p.name(), if value { "yes" } else { "no" }, a.source));
                }
            }
        }
        let anc = kb.ancestors(c);
        for (name, f) in [("part", &r.parts), ("material", &r.material)] {
            if let Field::Known(vs) = f {
                for v in vs.iter() {
                    if let Some(vc) = find(&sm, v.synset) {
                        if anc.contains(vc) {
                            bad.push(format!("{:?}: core — {name} {} ({}), graph — it is the concept itself or its ancestor", r.id, v.label, v.src.show()));
                        }
                    }
                }
            }
        }
    }
    bad
}

/// Core concepts whose synset is not in the domain lexfile (a warning, not a gate).
pub fn lexfile_warnings(g: &Graph) -> Vec<String> {
    let sm = synset_map(g);
    CORE.iter().filter_map(|r| find(&sm, r.synset).filter(|&c| g.lexfile_name(c) != r.domain.lexfile()).map(|c| format!("{:?}: domain {} ({}), synset — {}", r.id, r.domain.name(), r.domain.lexfile(), g.lexfile_name(c)))).collect()
}
