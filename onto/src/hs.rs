//! hs: the HS22 goods classification as a world model — tree chapter → heading → subheading, parsing descriptions
//! with the UD tree (`en`), features: state, processing, material or source («of X»), form, purpose, parts, species
//! (Latin names), composition, exclusions, code references, thresholds — exact numbers with units via `math`.
//!
//! **Description texts belong to the WCO** (BACI carries them from UN Comtrade): they do not go into the repo. `data/hs/` has only derived data:
//! code → concept, features, labels. Codes are facts.
//!
//! Parsing is deterministic: `en` gives lemmas, parts of speech and the tree; the telegraphic HS style («Fruit, edible: apples,
//! fresh») is split into levels by colons (level 0 — the heading top, then refinements) and into chunks by commas. A chunk is
//! either a feature group (closed lists — enum) or a noun group (head from the UD tree: the word whose head is outside
//! the group; fallback — the last noun).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use en::gram::UPos;
use math::rat::Rat;
use math::units::{Mono, Unit};

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
}

/// The data directory: $MOVA_DATA, else ~/mova-data.
fn mova_data() -> PathBuf {
    std::env::var("MOVA_DATA").map(PathBuf::from).unwrap_or_else(|_| home().join("mova-data"))
}
/// HS22 codes and descriptions (BACI data, read-only).
/// HS22 codes and descriptions (external trade data, read-only).
pub fn csv_path() -> PathBuf {
    std::env::var("ONTO_HS_CSV").map(PathBuf::from).unwrap_or_else(|_| home().join("devData/BACI/product_codes_HS22_V202601.csv"))
}

/// The `en` model (full UD tagger).
pub fn model_path() -> PathBuf {
    std::env::var("ONTO_EN_MODEL").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("en/models/en.bin"))
}

/// Derived data in the repo: `onto/data/hs/`.
pub fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/hs")
}

// ── CSV and tree ─────────────────────────────────────────────────────────────────────────────────────

/// CSV row fields: comma separator, quotes with doubling.
pub fn csv_fields(line: &str) -> Vec<String> {
    let (mut out, mut cur, mut q) = (Vec::new(), String::new(), false);
    let mut it = line.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '"' if q && it.peek() == Some(&'"') => {
                cur.push('"');
                it.next();
            }
            '"' => q = !q,
            ',' if !q => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// Gates: a row is «code,description», the code is 6 digits, codes increase (unique).
pub fn read_codes(p: &Path) -> Result<Vec<(String, String)>> {
    let text = std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))?;
    let mut v: Vec<(String, String)> = Vec::new();
    for (i, l) in text.lines().enumerate().skip(1) {
        let f = csv_fields(l);
        if f.len() != 2 || f[0].len() != 6 || !f[0].bytes().all(|b| b.is_ascii_digit()) {
            bail!("CSV gate: row {} is not «code,description»", i + 1);
        }
        if let Some(last) = v.last() {
            if last.0 >= f[0] {
                bail!("CSV gate: codes do not increase: {} → {}", last.0, f[0]);
            }
        }
        v.push((f[0].clone(), f[1].clone()));
    }
    Ok(v)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Chapter,
    Heading,
    Sub,
}

impl Level {
    pub fn name(self) -> &'static str {
        match self {
            Level::Chapter => "chapter",
            Level::Heading => "heading",
            Level::Sub => "subheading",
        }
    }
}

/// Code tree: node — (code, level, parent).
pub struct Tree {
    pub nodes: Vec<(String, Level, Option<usize>)>,
    pub idx: HashMap<String, usize>,
}

impl Tree {
    pub fn new(codes: &[(String, String)]) -> Tree {
        let mut t = Tree { nodes: Vec::new(), idx: HashMap::new() };
        for (c, _) in codes {
            let ch = t.add(&c[..2], Level::Chapter, None);
            let hd = t.add(&c[..4], Level::Heading, Some(ch));
            t.add(c, Level::Sub, Some(hd));
        }
        t
    }
    fn add(&mut self, code: &str, l: Level, parent: Option<usize>) -> usize {
        if let Some(&i) = self.idx.get(code) {
            return i;
        }
        self.nodes.push((code.to_string(), l, parent));
        self.idx.insert(code.to_string(), self.nodes.len() - 1);
        self.nodes.len() - 1
    }
    pub fn count(&self, l: Level) -> usize {
        self.nodes.iter().filter(|n| n.1 == l).count()
    }
    /// Child codes of a node.
    pub fn children(&self, code: &str) -> Vec<&str> {
        let Some(&i) = self.idx.get(code) else { return Vec::new() };
        self.nodes.iter().filter(|n| n.2 == Some(i)).map(|n| n.0.as_str()).collect()
    }
}

/// «Other» by code: sixth digit 9 (…9 — «other» in a level-2 group) or ending 90 (level-1 «other»).
/// This is a structural WCO convention; textual «other» (n.e.c., other than) is separate, `Parsed::nec`.
pub fn residual_code(code: &str) -> bool {
    code.ends_with('9') || code.ends_with("90")
}

// ── tokens ────────────────────────────────────────────────────────────────────────────────────────────

/// A word from the `en` parse: form, lowercase form, lemma, UPOS, head (1-based, 0 — root).
#[derive(Clone, Debug)]
pub struct Tok {
    pub form: String,
    pub low: String,
    pub lemma: String,
    pub upos: UPos,
    pub head: usize,
}

pub fn annotate(a: &en::annotate::Annotator, text: &str) -> Vec<Tok> {
    let toks = a.tokenize(text);
    let forms: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
    a.annotate(&forms).into_iter().map(|w| Tok { low: w.form.to_lowercase(), lemma: w.lemma.to_lowercase(), form: w.form, upos: w.upos, head: w.head }).collect()
}

// ── closed feature lists ───────────────────────────────────────────────────────────────────────────

macro_rules! lexicon {
    ($(#[$m:meta])* $name:ident { $($v:ident = [$($w:literal),+]),+ $(,)? }) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
        pub enum $name { $($v),+ }
        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$v),+];
            /// Canonical word (the first in the list).
            pub fn name(self) -> &'static str {
                match self { $($name::$v => [$($w),+][0]),+ }
            }
            pub fn parse(w: &str) -> Option<$name> {
                match w { $($($w)|+ => Some($name::$v),)+ _ => None }
            }
        }
    };
}

lexicon! {
    /// Goods state.
    State {
        Live = ["live", "living", "alive"],
        Fresh = ["fresh"],
        Chilled = ["chilled"],
        Frozen = ["frozen"],
        Dried = ["dried", "dry", "desiccated", "dehydrated"],
        Raw = ["raw", "crude", "unwrought", "unworked", "untreated", "unprocessed", "rough"],
        Whole = ["whole"],
        Liquid = ["liquid", "fluid"],
        Solid = ["solid"],
        Concentrated = ["concentrated"],
        Sparkling = ["sparkling"],
        Still = ["still"],
        New = ["new"],
        Used = ["worn", "second-hand"],
        Edible = ["edible"],
        Inedible = ["inedible"],
        PureBred = ["pure-bred", "purebred"],
    }
}

lexicon! {
    /// Processing.
    Process {
        Roasted = ["roasted"],
        Ground = ["ground", "milled", "powdered", "pulverised", "pulverized", "grated"],
        Crushed = ["crushed", "broken", "kibbled"],
        Preserved = ["preserved"],
        Prepared = ["prepared"],
        Salted = ["salted", "brined"],
        Smoked = ["smoked"],
        Cooked = ["cooked", "boiled", "steamed", "baked", "fried", "parboiled"],
        Sweetened = ["sweetened", "sugared"],
        Flavoured = ["flavoured", "flavored"],
        Fermented = ["fermented"],
        Distilled = ["distilled", "distilling"],
        Refined = ["refined"],
        Denatured = ["denatured"],
        Decaffeinated = ["decaffeinated"],
        Shelled = ["shelled", "husked", "hulled", "peeled", "skinned", "blanched", "stripped", "stemmed", "pitted", "seeded"],
        Cut = ["cut", "sliced", "chopped", "sawn", "chipped", "split", "diced"],
        Worked = ["worked", "machined", "manufactured", "semi-manufactured", "fabricated"],
        Bleached = ["bleached"],
        Dyed = ["dyed", "coloured", "colored"],
        Printed = ["printed"],
        Coated = ["coated", "plated", "covered", "galvanised", "galvanized", "painted", "varnished", "lacquered", "impregnated", "laminated", "clad"],
        Woven = ["woven"],
        Knitted = ["knitted", "crocheted"],
        Combed = ["combed", "carded"],
        Rolled = ["rolled", "hot-rolled", "cold-rolled", "flat-rolled"],
        Drawn = ["drawn", "cold-drawn", "extruded"],
        Forged = ["forged"],
        Cast = ["cast", "moulded", "molded"],
        Tanned = ["tanned"],
        Polished = ["polished"],
        Mixed = ["mixed", "blended"],
        Assembled = ["assembled", "mounted"],
        Vulcanised = ["vulcanised", "vulcanized"],
        Reinforced = ["reinforced"],
        Pickled = ["pickled"],
        Glazed = ["glazed", "enamelled", "enameled"],
        Stuffed = ["stuffed"],
        Twisted = ["twisted"],
        Textured = ["textured"],
        Tufted = ["tufted", "flocked"],
        Welded = ["welded", "riveted", "stitched"],
        Sintered = ["sintered"],
        Hydrogenated = ["hydrogenated"],
        Condensed = ["condensed", "evaporated"],
        Skimmed = ["skimmed"],
        Carbonated = ["aerated", "carbonated"],
    }
}

/// «in X», where X is a preserving liquid: this is processing, not form.
const PRESERVE_IN: &[&str] = &["brine", "oil", "vinegar", "syrup", "water", "acid", "alcohol", "sugar", "spirit"];

/// Participle or gerund by form (en sometimes tags a plural noun as a verb: «lobsters»).
fn participle(t: &Tok) -> bool {
    t.upos == UPos::VERB && (t.low.ends_with("ed") || t.low.ends_with("ing") || t.low.ends_with("en") || t.low.ends_with("wn") || t.low.ends_with("lt") || State::parse(&t.low).is_some() || Process::parse(&t.low).is_some())
}

/// Who the user is: «men's or boys'» — purpose, not goods.
const USERS: &[&str] = &["man", "men", "woman", "women", "boy", "girl", "baby", "babies", "infant", "child", "children", "gentleman", "lady"];

/// Function words that carry neither a feature nor a concept.
const FILLER: &[&str] = &["or", "and", "not", "whether", "otherwise", "similarly", "partly", "wholly", "entirely", "but", "also", "only", "solely", "mainly", "either", "nor", "further", "simply", "merely", "put", "up", "the", "a", "an", "&", "/", "-", "both", "singly", "together", "thereof", "therefor", "whole", "but"];

/// Words that do not go into concept modifiers.
const NP_SKIP: &[&str] = &["other", "similar", "such", "like", "various", "certain", "same", "their", "its", "all", "any", "some", "kind", "type", "the", "a", "an", "etc", "thereof", "therefor", "specified", "n.e.c.", "n.e.c"];

// ── thresholds ────────────────────────────────────────────────────────────────────────────────────────────

lexicon! {
    /// What the threshold measures.
    Attr {
        Weight = ["weight", "weighing", "weigh", "weighs", "mass"],
        Width = ["width", "wide"],
        Thickness = ["thickness", "thick"],
        Length = ["length", "long"],
        Diameter = ["diameter", "bore"],
        Circumference = ["circumference"],
        Height = ["height", "high"],
        Dimension = ["dimension", "size"],
        Capacity = ["capacity", "holding", "content", "contents"],
        Power = ["power", "output", "rating"],
        Voltage = ["voltage"],
        Strength = ["strength"],
        Brix = ["brix"],
        Age = ["age"],
        Share = ["containing", "proportion"],
        LinearDensity = ["decitex", "dtex"],
        YarnCount = ["metric"],
        Twist = ["turns", "turn"],
        Frequency = ["frequency"],
        Pressure = ["pressure"],
        Speed = ["speed"],
    }
}

/// Threshold unit: from `math` (kg, g, m, cm, mm, %) or outside it — with an exact factor to the base unit.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum U {
    Math(Unit),
    /// g/m² — via `math::units::Mono` (G·M⁻²)
    GPerM2,
    Tonne,
    Litre,
    Cc,
    Watt,
    KiloWatt,
    Va,
    KiloVa,
    Volt,
    KiloVolt,
    Decitex,
    MetricNumber,
    Year,
    Turns,
    None,
}

impl U {
    pub fn parse(s: &str) -> Option<U> {
        Some(match s {
            "kg" | "kgs" | "kilogram" | "kilograms" => U::Math(Unit::Kg),
            "g" | "gram" | "grams" | "gm" => U::Math(Unit::G),
            "m" | "metre" | "metres" | "meter" | "meters" => U::Math(Unit::M),
            "cm" | "centimetre" | "centimetres" => U::Math(Unit::Cm),
            "mm" | "millimetre" | "millimetres" => U::Math(Unit::Mm),
            "km" => U::Math(Unit::Km),
            "%" | "percent" | "per cent" => U::Math(Unit::Pct),
            "g/m2" | "g/m²" | "g/sq.m" => U::GPerM2,
            "t" | "tonne" | "tonnes" | "ton" | "tons" => U::Tonne,
            "l" | "litre" | "litres" | "liter" | "liters" => U::Litre,
            "cc" | "cm3" | "cm³" | "cm.3" => U::Cc,
            "w" | "watt" | "watts" => U::Watt,
            "kw" | "kilowatt" | "kilowatts" => U::KiloWatt,
            "va" => U::Va,
            "kva" => U::KiloVa,
            "v" | "volt" | "volts" => U::Volt,
            "kv" => U::KiloVolt,
            "decitex" | "dtex" => U::Decitex,
            "year" | "years" => U::Year,
            _ => return None,
        })
    }
    /// Base unit and the exact factor to it (for kg, g, m… — from `math`).
    pub fn base(self) -> (&'static str, Rat) {
        let r = |n: i128, d: i128| Rat::new(n, d).expect("unit factor");
        match self {
            U::Math(Unit::Pct) => ("%", r(1, 1)),
            U::Math(u) => (u.dim_base(), u.scale()),
            U::GPerM2 => ("kg/m²", Mono(vec![(Unit::G, 1), (Unit::M, -2)]).scale().expect("g/m²")),
            U::Tonne => ("kg", r(1000, 1)),
            U::Litre => ("m³", r(1, 1000)),
            U::Cc => ("m³", Mono(vec![(Unit::Cm, 3)]).scale().expect("cm³")),
            U::Watt => ("W", r(1, 1)),
            U::KiloWatt => ("W", r(1000, 1)),
            U::Va => ("VA", r(1, 1)),
            U::KiloVa => ("VA", r(1000, 1)),
            U::Volt => ("V", r(1, 1)),
            U::KiloVolt => ("V", r(1000, 1)),
            U::Decitex => ("dtex", r(1, 1)),
            U::MetricNumber => ("Nm", r(1, 1)),
            U::Year => ("year", r(1, 1)),
            U::Turns => ("turns/m", r(1, 1)),
            U::None => ("", r(1, 1)),
        }
    }
    pub fn sym(self) -> &'static str {
        match self {
            U::Math(u) => u.sym(),
            U::GPerM2 => "g/m²",
            U::Tonne => "t",
            U::Litre => "l",
            U::Cc => "cm³",
            U::Watt => "W",
            U::KiloWatt => "kW",
            U::Va => "VA",
            U::KiloVa => "kVA",
            U::Volt => "V",
            U::KiloVolt => "kV",
            U::Decitex => "dtex",
            U::MetricNumber => "Nm",
            U::Year => "years",
            U::Turns => "turns/m",
            U::None => "",
        }
    }
    /// What a unit measures when there is no measure word.
    pub fn attr(self) -> Attr {
        match self {
            U::Math(Unit::Kg | Unit::G) | U::Tonne => Attr::Weight,
            U::Math(Unit::Pct) => Attr::Share,
            U::Math(_) => Attr::Dimension,
            U::GPerM2 => Attr::Weight,
            U::Litre | U::Cc => Attr::Capacity,
            U::Watt | U::KiloWatt | U::Va | U::KiloVa => Attr::Power,
            U::Volt | U::KiloVolt => Attr::Voltage,
            U::Decitex => Attr::LinearDensity,
            U::MetricNumber => Attr::YarnCount,
            U::Year => Attr::Age,
            U::Turns => Attr::Twist,
            U::None => Attr::Share,
        }
    }
}

trait DimBase {
    fn dim_base(self) -> &'static str;
}

impl DimBase for Unit {
    fn dim_base(self) -> &'static str {
        match self {
            Unit::Kg | Unit::G | Unit::Lb => "kg",
            Unit::M | Unit::Km | Unit::Cm | Unit::Mm | Unit::In | Unit::Ft | Unit::Mi => "m",
            Unit::S | Unit::Min | Unit::H | Unit::Day | Unit::Week => "s",
            _ => "",
        }
    }
}

/// Bound: value in the base unit (exact), whether inclusive.
pub type Edge = (Rat, bool);

/// Threshold: «weighing over 1 kg» → Weight, lo (1 kg, exclusive).
#[derive(Clone, Debug)]
pub struct Bound {
    pub attr: Attr,
    pub lo: Option<Edge>,
    pub hi: Option<Edge>,
    pub unit: U,
    /// «by weight» / «by volume» for shares
    pub basis: Option<&'static str>,
}

fn dec(r: Rat) -> String {
    r.exact_decimal(6).unwrap_or_else(|| r.round_decimal(6))
}

impl Bound {
    /// Human-readable form in the base unit: `Weight ∈ (1, ∞) kg`.
    pub fn show(&self) -> String {
        let base = self.unit.base().0;
        let lo = self.lo.map(|(v, inc)| format!("{}{}", if inc { "[" } else { "(" }, dec(v))).unwrap_or_else(|| "(−∞".into());
        let hi = self.hi.map(|(v, inc)| format!("{}{}", dec(v), if inc { "]" } else { ")" })).unwrap_or_else(|| "∞)".into());
        let b = self.basis.map(|b| format!(" {b}")).unwrap_or_default();
        format!("{:?} ∈ {lo}, {hi} {base}{b}", self.attr).trim_end().to_string()
    }
    /// The intervals of two thresholds overlap (for the neighbours gate).
    pub fn overlaps(&self, o: &Bound) -> bool {
        let below = |hi: Option<Edge>, lo: Option<Edge>| match (hi, lo) {
            (Some((h, hinc)), Some((l, linc))) => h < l || (h == l && !(hinc && linc)),
            _ => false,
        };
        !(below(self.hi, o.lo) || below(o.hi, self.lo))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Gt,
    Ge,
    Lt,
    Le,
}

/// Number with a unit from position `i`: «1000cc», «4.75mm», «200g/m2», «80 % vol», «2 litres». Returns (value,
/// unit, «by volume» base, how many tokens consumed).
fn num_unit(w: &[&str], i: usize) -> Option<(Rat, U, Option<&'static str>, usize)> {
    let t = w.get(i)?;
    let split = t.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == ',')).unwrap_or(t.len());
    if split == 0 {
        return None;
    }
    let v = Rat::parse_decimal(t[..split].trim_end_matches('.'))?;
    let suffix = &t[split..];
    let mut n = 1;
    let mut u = if suffix.is_empty() { U::None } else { U::parse(suffix)? };
    if u == U::None {
        if let Some(x) = w.get(i + 1).and_then(|s| U::parse(s)) {
            u = x;
            n += 1;
        } else if w.get(i + 1) == Some(&"metric") && w.get(i + 2) == Some(&"number") {
            u = U::MetricNumber;
            n += 2;
        } else if w.get(i + 1) == Some(&"turns") {
            u = U::Turns;
            n += 1;
        } else if w.get(i + 1) == Some(&"g") && w.get(i + 2) == Some(&"/") {
            u = U::GPerM2;
            n += 3;
        }
    }
    let mut basis = None;
    if u == U::Math(Unit::Pct) && w.get(i + n) == Some(&"vol") {
        basis = Some("by volume");
        n += 1;
    }
    Some((v, u, basis, n))
}

/// Thresholds in a chunk (lowercase words). Each comparator takes the nearest measure word before it, otherwise
/// the measure by unit. «over 1000 but not over 1500cc» — the unit of the first number comes from the second.
pub fn bounds(w: &[&str]) -> Vec<Bound> {
    let mut hits: Vec<(Op, Rat, U, Option<&'static str>, Option<Attr>, usize)> = Vec::new();
    let mut i = 0;
    let attr_before = |k: usize| -> Option<Attr> { (0..k).rev().find_map(|j| Attr::parse(w[j])) };
    while i < w.len() {
        let rest = &w[i..];
        let (op, skip) = match rest {
            ["not", "exceeding", ..] | ["not", "over", ..] | ["not", "more", "than", ..] | ["not", "above", ..] => (Some(Op::Le), if rest[1] == "more" { 3 } else { 2 }),
            ["not", "less", "than", ..] | ["not", "under", ..] | ["not", "below", ..] => (Some(Op::Ge), if rest[1] == "less" { 3 } else { 2 }),
            ["exceeding", ..] | ["over", ..] | ["above", ..] => (Some(Op::Gt), 1),
            ["more", "than", ..] => (Some(Op::Gt), 2),
            ["less", "than", ..] => (Some(Op::Lt), 2),
            ["under", ..] | ["below", ..] => (Some(Op::Lt), 1),
            ["at", "least", ..] | ["minimum", ..] => (Some(Op::Ge), if rest[0] == "at" { 2 } else { 1 }),
            ["up", "to", ..] | ["maximum", ..] => (Some(Op::Le), if rest[0] == "up" { 2 } else { 1 }),
            _ => (None, 0),
        };
        if let Some(op) = op {
            if let Some((v, u, b, n)) = num_unit(w, i + skip) {
                hits.push((op, v, u, b, attr_before(i), i));
                i += skip + n;
                continue;
            }
        }
        // «N or more» / «N or less» / «between N and M»
        if rest.first() == Some(&"between") {
            if let (Some((a, ua, ba, na)), true) = (num_unit(w, i + 1), true) {
                if w.get(i + 1 + na) == Some(&"and") {
                    if let Some((b, ub, bb, nb)) = num_unit(w, i + 2 + na) {
                        let at = attr_before(i);
                        hits.push((Op::Ge, a, ua, ba, at, i));
                        hits.push((Op::Le, b, ub, bb, at, i));
                        i += 2 + na + nb;
                        continue;
                    }
                }
            }
        }
        if let Some((v, u, b, n)) = num_unit(w, i) {
            let (a, c) = (w.get(i + n).copied(), w.get(i + n + 1).copied());
            if a == Some("or") && (c == Some("more") || c == Some("less") || c == Some("over") || c == Some("above")) {
                hits.push((if c == Some("less") { Op::Le } else { Op::Ge }, v, u, b, attr_before(i), i));
                i += n + 2;
                continue;
            }
        }
        i += 1;
    }
    // a unit-less number — the unit from a neighbour (the next one, otherwise the previous)
    for k in 0..hits.len() {
        if hits[k].2 == U::None {
            if let Some(u) = hits[k + 1..].iter().map(|h| h.2).find(|u| *u != U::None).or_else(|| hits[..k].iter().rev().map(|h| h.2).find(|u| *u != U::None)) {
                hits[k].2 = u;
            }
        }
    }
    // «by weight» / «by volume» after a share
    let basis_all = if w.windows(2).any(|x| x == ["by", "volume"]) {
        Some("by volume")
    } else if w.windows(2).any(|x| x == ["by", "weight"]) {
        Some("by weight")
    } else {
        None
    };
    let mut out: Vec<Bound> = Vec::new();
    for (op, v, u, b, at, _) in hits {
        let attr = match at {
            Some(Attr::Weight) if u == U::Math(Unit::Pct) => Attr::Share,
            Some(Attr::Capacity) if matches!(u, U::Math(Unit::Kg | Unit::G)) => Attr::Capacity,
            Some(a) => a,
            None => u.attr(),
        };
        let (_, f) = u.base();
        let Ok(x) = v.mul(f) else { continue };
        let edge = (x, matches!(op, Op::Ge | Op::Le));
        let basis = b.or(if u == U::Math(Unit::Pct) { basis_all } else { None });
        // same attribute and unit with no bound on this side — complete it («over 1000 but not over 1500cc»)
        if let Some(last) = out.last_mut().filter(|l| l.attr == attr && l.unit.base().0 == u.base().0) {
            let lower = matches!(op, Op::Gt | Op::Ge);
            if lower && last.lo.is_none() {
                last.lo = Some(edge);
                continue;
            }
            if !lower && last.hi.is_none() {
                last.hi = Some(edge);
                continue;
            }
        }
        let (lo, hi) = if matches!(op, Op::Gt | Op::Ge) { (Some(edge), None) } else { (None, Some(edge)) };
        out.push(Bound { attr, lo, hi, unit: u, basis });
    }
    out
}

// ── parsing a description ─────────────────────────────────────────────────────────────────────────────────────

/// Feature kind.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Kind {
    /// heading top (level 0, before the colon)
    Head,
    /// specific goods (levels ≥ 1)
    Item,
    State,
    Process,
    /// «of X»: material or source — decided by mapping to the world model
    Of,
    Form,
    Purpose,
    Part,
    Taxon,
    Bound,
    /// composition and components: containing, with, without
    Has,
    Exclude,
    Ref,
    /// unclassified label word (for measuring «unknown»)
    Other,
}

impl Kind {
    pub const ALL: [Kind; 14] = [Kind::Head, Kind::Item, Kind::State, Kind::Process, Kind::Of, Kind::Form, Kind::Purpose, Kind::Part, Kind::Taxon, Kind::Bound, Kind::Has, Kind::Exclude, Kind::Ref, Kind::Other];
    pub fn name(self) -> &'static str {
        match self {
            Kind::Head => "head",
            Kind::Item => "item",
            Kind::State => "state",
            Kind::Process => "process",
            Kind::Of => "of",
            Kind::Form => "form",
            Kind::Purpose => "purpose",
            Kind::Part => "part",
            Kind::Taxon => "taxon",
            Kind::Bound => "bound",
            Kind::Has => "has",
            Kind::Exclude => "exclude",
            Kind::Ref => "ref",
            Kind::Other => "other",
        }
    }
    pub fn parse(s: &str) -> Option<Kind> {
        Kind::ALL.iter().copied().find(|k| k.name() == s)
    }
    pub fn uk(self) -> &'static str {
        match self {
            Kind::Head => "heading top",
            Kind::Item => "goods",
            Kind::State => "state",
            Kind::Process => "processing",
            Kind::Of => "material / source",
            Kind::Form => "form",
            Kind::Purpose => "purpose",
            Kind::Part => "parts",
            Kind::Taxon => "species (taxon)",
            Kind::Bound => "threshold",
            Kind::Has => "composition",
            Kind::Exclude => "except",
            Kind::Ref => "reference",
            Kind::Other => "label",
        }
    }
}

/// Noun group: modifiers and head (lemma). `phrases()` — from the longest: «Atlantic salmon», «salmon».
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Np {
    pub mods: Vec<String>,
    pub head: String,
}

impl Np {
    pub fn phrases(&self) -> Vec<String> {
        let mut v = Vec::new();
        for k in 0..=self.mods.len() {
            let mut p: Vec<&str> = self.mods[k..].iter().map(|s| s.as_str()).collect();
            p.push(&self.head);
            v.push(p.join(" "));
        }
        v
    }
    pub fn show(&self) -> String {
        self.phrases()[0].clone()
    }
}

#[derive(Clone, Debug)]
pub struct Feat {
    pub kind: Kind,
    pub neg: bool,
    /// colon level (0 — heading top)
    pub level: u8,
    /// canonical value: list word, lemma or phrase
    pub val: String,
    pub np: Option<Np>,
    pub bound: Option<Bound>,
}

impl Feat {
    fn new(kind: Kind, level: u8, neg: bool, val: impl Into<String>) -> Feat {
        Feat { kind, neg, level, val: val.into(), np: None, bound: None }
    }
    fn np(kind: Kind, level: u8, neg: bool, np: Np) -> Feat {
        Feat { kind, neg, level, val: np.show(), np: Some(np), bound: None }
    }
}

/// Parsing one description.
#[derive(Clone, Debug, Default)]
pub struct Parsed {
    pub feats: Vec<Feat>,
    /// «n.e.c.» or «other than» in the text — textual «other»
    pub nec: bool,
    /// «whether or not …» — a feature that does not distinguish
    pub optional: usize,
    /// label words without a class / all content words
    pub unknown_words: usize,
    pub content_words: usize,
}

impl Parsed {
    pub fn of(&self, k: Kind) -> impl Iterator<Item = &Feat> {
        self.feats.iter().filter(move |f| f.kind == k)
    }
    /// Main concept: the deepest goods item, otherwise the heading top.
    pub fn main(&self) -> Option<&Feat> {
        let items: Vec<&Feat> = self.of(Kind::Item).collect();
        let deepest = items.iter().map(|f| f.level).max();
        items.into_iter().find(|f| Some(f.level) == deepest).or_else(|| self.of(Kind::Head).next())
    }
}

fn is_content(t: &Tok) -> bool {
    matches!(t.upos, UPos::NOUN | UPos::PROPN | UPos::ADJ | UPos::VERB | UPos::ADV | UPos::NUM)
}

/// Latin taxon name: «Salmo trutta», «Engraulis spp.», «Sparidae».
fn taxa(ts: &[&Tok]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    let cap = |t: &Tok| t.form.chars().next().is_some_and(|c| c.is_uppercase()) && t.form.chars().skip(1).all(|c| c.is_lowercase()) && t.form.len() > 2;
    while i < ts.len() {
        let t = ts[i];
        if cap(t) {
            if let Some(n) = ts.get(i + 1) {
                let sp = n.low.trim_end_matches('.');
                if sp == "spp" || sp == "sp" {
                    out.push(format!("{} spp.", t.form));
                    i += 2;
                    continue;
                }
                if n.form.chars().all(|c| c.is_lowercase()) && n.form.len() > 2 && !FILLER.contains(&n.low.as_str()) && n.upos != UPos::ADP {
                    out.push(format!("{} {}", t.form, n.form));
                    i += 2;
                    continue;
                }
            }
            if ["idae", "inae", "formes", "aceae", "oidea"].iter().any(|s| t.low.ends_with(s)) {
                out.push(t.form.clone());
            }
        }
        i += 1;
    }
    out
}

fn is_nec(t: &str) -> bool {
    t.starts_with("n.e.c") || t == "nec" || t == "nes" || t.starts_with("n.e.s")
}

/// Code from a reference: «0302.91» → 030291, «60.01» → 6001, «8702» → 8702, «0304» → 0304.
fn ref_code(t: &str) -> Option<String> {
    let d: String = t.chars().filter(|c| c.is_ascii_digit()).collect();
    let ok = t.chars().all(|c| c.is_ascii_digit() || c == '.') && matches!(d.len(), 4 | 5 | 6);
    let dotted = t.contains('.');
    let shape = if dotted { t.split('.').next().is_some_and(|a| a.len() == 2 || a.len() == 4) } else { d.len() == 4 };
    (ok && shape).then_some(d)
}

struct Parser<'a> {
    t: &'a [Tok],
    out: Parsed,
    exclude: Vec<bool>,
    /// list continuation through commas: «of sheep, goats, horses» — bare groups after «of X» are «of» too
    cont: Vec<Option<Kind>>,
    heads: usize,
}

impl<'a> Parser<'a> {
    /// Noun groups of a chunk up to the first preposition or participle outside the group; the rest is the tail.
    fn nps(&self, idx: &[usize]) -> (Vec<Np>, Vec<usize>) {
        let mut stop = idx.len();
        for (k, &i) in idx.iter().enumerate() {
            let t = &self.t[i];
            let is_adp = t.upos == UPos::ADP && !["-", "'s"].contains(&t.low.as_str());
            let verbal = participle(t) && State::parse(&t.low).is_none() && Process::parse(&t.low).is_none() && {
                // a participle followed by a noun is a modifier («breeding animals»), otherwise the tail
                let next = idx.get(k + 1).map(|&j| &self.t[j]);
                !next.is_some_and(|n| matches!(n.upos, UPos::NOUN | UPos::PROPN))
            };
            if k > 0 && (is_adp || verbal || t.low == "(") {
                stop = k;
                break;
            }
        }
        let (np_idx, tail) = idx.split_at(stop);
        // conjunctions and commas split into conjuncts; a conjunct without a noun is a distributive modifier of the next one
        let mut out = Vec::new();
        let mut carry: Vec<String> = Vec::new();
        let mut cur: Vec<usize> = Vec::new();
        let mut flush = |cur: &mut Vec<usize>, carry: &mut Vec<String>, out: &mut Vec<Np>, carry_next: bool| {
            let toks: Vec<&Tok> = cur.iter().map(|&i| &self.t[i]).filter(|t| !NP_SKIP.contains(&t.low.as_str()) && t.upos != UPos::DET && t.upos != UPos::NUM && t.upos != UPos::PUNCT).collect();
            let head = toks.iter().rposition(|t| matches!(t.upos, UPos::NOUN | UPos::PROPN) || (t.upos == UPos::VERB && !participle(t)));
            // «Atlantic and Pacific bluefin tunas»: a lone proper adjective is a distributive modifier
            let head = head.filter(|_| !(toks.len() == 1 && toks[0].upos == UPos::PROPN && carry_next));
            match head {
                Some(h) => {
                    let mut mods: Vec<String> = carry.drain(..).collect();
                    for t in &toks[..h] {
                        if State::parse(&t.low).is_none() && Process::parse(&t.low).is_none() {
                            mods.push(if t.upos == UPos::PROPN { t.form.clone() } else { t.low.clone() });
                        }
                    }
                    let ht = toks[h];
                    let head = if ht.lemma.is_empty() || ht.lemma == "_" { ht.low.clone() } else { ht.lemma.clone() };
                    out.push(Np { mods, head });
                }
                None => carry.extend(toks.iter().filter(|t| State::parse(&t.low).is_none() && Process::parse(&t.low).is_none()).map(|t| t.low.clone())),
            }
            cur.clear();
        };
        for (k, &i) in np_idx.iter().enumerate() {
            let t = &self.t[i];
            if t.upos == UPos::CCONJ || t.low == "," || t.low == ";" {
                // is there another conjunct of two or more words further on?
                let rest = np_idx[k + 1..].iter().take_while(|&&j| !(self.t[j].upos == UPos::CCONJ || self.t[j].low == ",")).count();
                flush(&mut cur, &mut carry, &mut out, rest >= 2);
            } else {
                cur.push(i);
            }
        }
        flush(&mut cur, &mut carry, &mut out, false);
        (out, tail.to_vec())
    }

    /// State and processing features among the chunk words; «not»/«un-»/«non-» — negation.
    fn lexical(&mut self, idx: &[usize], level: u8, neg0: bool) -> bool {
        let mut any = false;
        let mut neg = neg0;
        for &i in idx {
            let w = self.t[i].low.as_str();
            if w == "not" || w == "non" {
                neg = true;
                continue;
            }
            let (base, n) = match w.strip_prefix("un").or_else(|| w.strip_prefix("non-")) {
                Some(b) if State::parse(w).is_none() && Process::parse(w).is_none() && (State::parse(b).is_some() || Process::parse(b).is_some()) => (b, true),
                _ => (w, false),
            };
            if let Some(s) = State::parse(base) {
                self.out.feats.push(Feat::new(Kind::State, level, neg ^ n, s.name()));
                any = true;
            } else if let Some(p) = Process::parse(base) {
                self.out.feats.push(Feat::new(Kind::Process, level, neg ^ n, p.name()));
                any = true;
            }
        }
        any
    }

    fn only_lexical(&self, idx: &[usize]) -> bool {
        idx.iter().all(|&i| {
            let t = &self.t[i];
            let w = t.low.as_str();
            let b = w.strip_prefix("un").or_else(|| w.strip_prefix("non-")).unwrap_or(w);
            !is_content(t) && t.upos != UPos::CCONJ || FILLER.contains(&w) || State::parse(w).is_some() || Process::parse(w).is_some() || State::parse(b).is_some() || Process::parse(b).is_some() || t.upos == UPos::CCONJ
        })
    }

    /// Inversion at level 0: chunk adjectives modify the previous heading top («Nuts, edible» → edible nut).
    fn invert(&mut self, idx: &[usize], level: u8) {
        let adjs: Vec<String> = idx.iter().map(|&i| &self.t[i]).filter(|t| matches!(t.upos, UPos::ADJ | UPos::VERB) && !FILLER.contains(&t.low.as_str())).map(|t| t.low.clone()).collect();
        if let Some(last) = self.out.feats.iter_mut().rev().find(|f| f.kind == Kind::Head && f.level == level) {
            if let Some(np) = last.np.as_mut() {
                for a in adjs.iter().rev() {
                    if !np.mods.contains(a) {
                        np.mods.insert(0, a.clone());
                    }
                }
                last.val = np.show();
            }
        }
    }

    fn refs(&mut self, idx: &[usize], level: u8) {
        for &i in idx {
            if let Some(c) = ref_code(&self.t[i].low) {
                self.out.feats.push(Feat::new(Kind::Ref, level, false, c));
            }
        }
    }

    fn content_after(&self, idx: &[usize]) -> String {
        idx.iter().map(|&i| &self.t[i]).filter(|t| matches!(t.upos, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ) && !FILLER.contains(&t.low.as_str()) && !NP_SKIP.contains(&t.low.as_str())).take(3).map(|t| if t.upos == UPos::VERB || t.lemma.is_empty() { t.low.clone() } else { t.lemma.clone() }).collect::<Vec<_>>().join(" ")
    }

    /// A chunk between commas of one level.
    fn chunk(&mut self, idx0: &[usize], level: u8, neg0: bool) {
        let mut idx: Vec<usize> = idx0.iter().copied().filter(|&i| !matches!(self.t[i].low.as_str(), "," | ";" | "." | ")" | "(")).collect();
        while idx.first().is_some_and(|&i| matches!(self.t[i].low.as_str(), "and" | "or" | "but" | "&" | "-" | "including" | "incl" | "incl." | "i.e." | "e.g.")) {
            idx.remove(0);
        }
        if idx.is_empty() {
            return;
        }
        let w: Vec<&str> = idx.iter().map(|&i| self.t[i].low.as_str()).collect();
        let lv = level as usize;
        let prev_np = self.out.feats.last().is_some_and(|f| matches!(f.kind, Kind::Head | Kind::Item) && f.level == level);
        if self.exclude.len() <= lv {
            self.exclude.resize(lv + 1, false);
        }
        if self.cont.len() <= lv {
            self.cont.resize(lv + 1, None);
        }
        let cont = self.cont[lv];
        // whether or not …
        if w[0] == "whether" {
            self.out.optional += 1;
            return;
        }
        // n.e.c. … — textual «other»; code references
        if let Some(k) = w.iter().position(|x| is_nec(x)) {
            self.out.nec = true;
            self.refs(&idx[k..], level);
            if k > 0 {
                self.chunk(&idx[..k], level, neg0);
            }
            return;
        }
        // excluding …: up to the end of the level — exclusions
        if matches!(w[0], "excluding" | "except" | "excl" | "excl.") {
            self.exclude[lv] = true;
            self.cont[lv] = Some(Kind::Exclude);
            if idx.len() > 1 {
                self.chunk(&idx[1..], level, neg0);
            }
            return;
        }
        // negation: not X, other than X
        // «not fillets, meat of 0304, …» — an exclusion to the end of the level (BACI rewrites «excluding» this way); «other than X, Y» —
        // an exclusion over the list; «not smoked» — a negated feature
        // an exclusion if the rest has a noun (not only features): «not edible fish offal», «other than cattle»
        let np_first = |rest: &[usize]| rest.iter().take_while(|&&i| self.t[i].upos != UPos::ADP).any(|&i| matches!(self.t[i].upos, UPos::NOUN | UPos::PROPN) && State::parse(&self.t[i].low).is_none() && Process::parse(&self.t[i].low).is_none());
        if w[0] == "not" && idx.len() > 1 && !matches!(w[1], "exceeding" | "over" | "more" | "less" | "under" | "below" | "above") {
            if np_first(&idx[1..]) && !self.only_lexical(&idx[1..]) && !matches!(w[1], "for" | "containing" | "in" | "of" | "with" | "put" | "including") {
                self.exclude[lv] = true;
                self.cont[lv] = Some(Kind::Exclude);
                return self.chunk(&idx[1..], level, neg0);
            }
            return self.chunk(&idx[1..], level, !neg0);
        }
        if w.starts_with(&["other", "than"]) && idx.len() > 2 {
            self.out.nec = true;
            let breeding = w.contains(&"breeding");
            if np_first(&idx[2..]) && !self.only_lexical(&idx[2..]) && !breeding {
                self.cont[lv] = Some(Kind::Exclude);
                let (nps, tail) = self.nps(&idx[2..]);
                for np in nps {
                    self.out.feats.push(Feat::np(Kind::Exclude, level, neg0, np));
                }
                if !tail.is_empty() {
                    self.chunk(&tail, level, neg0);
                }
                return;
            }
            return self.chunk(&idx[2..], level, !neg0);
        }
        // parts
        if matches!(w[0], "parts" | "part") && (w.len() == 1 || matches!(w[1], "of" | "thereof" | "and" | "for" | "suitable" | "therefor")) {
            let of = w.iter().position(|x| *x == "of" || *x == "for").map(|k| self.nps(&idx[k + 1..]).0).unwrap_or_default();
            if of.is_empty() {
                self.out.feats.push(Feat::new(Kind::Part, level, false, "thereof"));
            }
            for np in of {
                self.out.feats.push(Feat::np(Kind::Part, level, false, np));
            }
            return;
        }
        // thresholds
        let bs = bounds(&w);
        if !bs.is_empty() {
            for b in bs {
                let mut f = Feat::new(Kind::Bound, level, neg0, format!("{:?}", b.attr));
                f.bound = Some(b);
                self.out.feats.push(f);
            }
            // «85% or more by weight of cotton» — the substance of the share
            let share_of = (2..w.len()).rev().find(|&k| w[k] == "of" && (w[k - 1].ends_with('%') || (matches!(w[k - 1], "weight" | "volume") && w[k - 2] == "by")));
            if let Some(k) = share_of {
                let (nps, _) = self.nps(&idx[k + 1..]);
                for np in nps {
                    self.out.feats.push(Feat::np(Kind::Has, level, neg0, np));
                }
            }
            return;
        }
        // purpose
        let purpose_at = if w[0] == "for" {
            Some(1)
        } else if w.len() > 2 && matches!(w[0], "used" | "suitable" | "designed" | "intended" | "specially") && w.contains(&"for") {
            w.iter().position(|x| *x == "for").map(|k| k + 1)
        } else if w.starts_with(&["of", "a", "kind"]) || w.starts_with(&["of", "the", "kind"]) || w.starts_with(&["of", "a", "type"]) || w.starts_with(&["of", "the", "type"]) {
            w.iter().position(|x| *x == "for").map(|k| k + 1)
        } else {
            None
        };
        if let Some(k) = purpose_at {
            let v = self.content_after(&idx[k..]);
            if !v.is_empty() {
                self.out.feats.push(Feat::new(Kind::Purpose, level, neg0, v));
                self.cont[lv] = Some(Kind::Purpose);
                return;
            }
        }
        // of X: material, source or reference
        if w[0] == "of" && idx.len() > 1 {
            let rest = &idx[1..];
            if rest.iter().any(|&i| matches!(self.t[i].low.as_str(), "heading" | "headings" | "subheading" | "subheadings" | "item" | "chapter" | "no." | "no")) || ref_code(&self.t[rest[0]].low).is_some() {
                self.refs(rest, level);
                return;
            }
            let (nps, tail) = self.nps(rest);
            let kind = if self.exclude[lv] || cont == Some(Kind::Exclude) { Kind::Exclude } else { Kind::Of };
            if kind == Kind::Of {
                self.cont[lv] = Some(Kind::Of);
            }
            for np in nps {
                self.out.feats.push(Feat::np(kind, level, neg0, np));
            }
            if !tail.is_empty() {
                self.chunk(&tail, level, neg0);
            }
            return;
        }
        // in X: form or preserving liquid
        if w[0] == "in" && idx.len() > 1 {
            let (nps, tail) = self.nps(&idx[1..]);
            for np in nps {
                if PRESERVE_IN.contains(&np.head.as_str()) {
                    self.out.feats.push(Feat::new(Kind::Process, level, neg0, format!("in {}", np.head)));
                } else {
                    self.out.feats.push(Feat::np(Kind::Form, level, neg0, np));
                }
            }
            if !tail.is_empty() {
                self.chunk(&tail, level, neg0);
            }
            return;
        }
        // composition: containing / with / without
        if matches!(w[0], "containing" | "with" | "without" | "incorporating" | "comprising") && idx.len() > 1 {
            let neg = neg0 ^ (w[0] == "without");
            let mut k = 1;
            while k < idx.len() && matches!(w[k], "added" | "only" | "both" | "a" | "an" | "the") {
                k += 1;
            }
            let (nps, tail) = self.nps(&idx[k.min(idx.len())..]);
            self.cont[lv] = Some(Kind::Has);
            for np in nps {
                self.out.feats.push(Feat::np(Kind::Has, level, neg, np));
            }
            if !tail.is_empty() {
                self.chunk(&tail, level, neg0);
            }
            return;
        }
        // only states and processing; at level 0 after the heading top — also a modifier of the top («Fruit, edible» → edible fruit)
        if self.only_lexical(&idx) {
            self.cont[lv] = None;
            if !self.exclude[lv] {
                self.lexical(&idx, level, neg0);
            }
            if prev_np && level == 0 {
                self.invert(&idx, level);
            }
            return;
        }
        // tail at a preposition not matched above («obtained by distilling …», «from …»)
        // «obtained by distilling …», «from …»; but «Tanning extracts», «Harrows» (en: verb) — a noun group
        let next_noun = idx.get(1).is_some_and(|&j| matches!(self.t[j].upos, UPos::NOUN | UPos::PROPN));
        if self.t[idx[0]].upos == UPos::ADP || (participle(&self.t[idx[0]]) && !next_noun && State::parse(w[0]).is_none() && Process::parse(w[0]).is_none()) {
            let lex = self.lexical(&idx, level, neg0);
            let k = idx.iter().position(|&i| matches!(self.t[i].upos, UPos::NOUN | UPos::PROPN));
            if let Some(k) = k {
                let (nps, tail) = self.nps(&idx[k..]);
                let kind = if w.contains(&"from") || w.iter().any(|x| matches!(*x, "obtained" | "derived" | "made" | "produced")) { Kind::Of } else { Kind::Other };
                for np in nps {
                    self.out.feats.push(Feat::np(if self.exclude[lv] { Kind::Exclude } else { kind }, level, neg0, np));
                }
                if !tail.is_empty() {
                    self.chunk(&tail, level, neg0);
                }
            } else if !lex {
                // «not filled» — an unknown feature still distinguishes neighbours: a label with negation
                for &i in idx.iter().filter(|&&i| is_content(&self.t[i]) && !FILLER.contains(&self.t[i].low.as_str()) && self.t[i].upos != UPos::ADP) {
                    self.out.feats.push(Feat::new(Kind::Other, level, neg0, self.t[i].low.clone()));
                    self.out.unknown_words += 1;
                }
            }
            return;
        }
        // noun group: heading top, goods, list continuation or exclusion
        let (nps, tail) = self.nps(&idx);
        let excl = self.exclude[lv] || cont == Some(Kind::Exclude);
        // state modifiers in the group («roasted coffee», «frozen fillets»)
        let np_len = idx.len() - tail.len();
        if !excl {
            self.lexical(&idx[..np_len], level, neg0);
        }
        if nps.is_empty() {
            // no noun: at level 0 after the heading top — inversion «Steel, stainless» → stainless steel; the rest — labels
            if prev_np && level == 0 {
                self.invert(&idx[..np_len], level);
            }
            for t in idx[..np_len].iter().map(|&i| &self.t[i]).filter(|t| matches!(t.upos, UPos::ADJ | UPos::VERB | UPos::NOUN) && !FILLER.contains(&t.low.as_str()) && State::parse(&t.low).is_none() && Process::parse(&t.low).is_none()) {
                self.out.feats.push(Feat::new(Kind::Other, level, neg0, t.low.clone()));
                self.out.unknown_words += 1;
            }
        }
        for np in nps {
            let kind = if np.head == "animal" && np.mods.iter().any(|m| m == "breeding") {
                // «pure-bred breeding animals» — purpose, not a new goods item
                self.out.feats.push(Feat::new(Kind::Purpose, level, neg0 || excl, "breeding"));
                continue;
            } else if excl || (neg0 && level > 0) {
                Kind::Exclude
            } else if level > 0 && USERS.contains(&np.head.as_str()) && np.mods.is_empty() {
                self.out.feats.push(Feat::new(Kind::Purpose, level, neg0, format!("for {}", np.head)));
                continue;
            } else if matches!(cont, Some(Kind::Of | Kind::Has)) {
                cont.expect("cont")
            } else if cont == Some(Kind::Purpose) {
                self.out.feats.push(Feat::new(Kind::Purpose, level, neg0, np.show()));
                continue;
            } else if level == 0 {
                self.heads += 1;
                Kind::Head
            } else if matches!(np.head.as_str(), "purpose" | "use" | "application" | "sale") {
                // «… dental or veterinary purposes» — purpose tail broken by commas
                let v = if np.mods.is_empty() { np.head.clone() } else { np.mods.join(" ") };
                self.out.feats.push(Feat::new(Kind::Purpose, level, neg0, v));
                continue;
            } else {
                Kind::Item
            };
            self.out.feats.push(Feat::np(kind, level, neg0, np));
        }
        if !tail.is_empty() {
            self.chunk(&tail, level, neg0);
        }
    }
}

/// Parsing a description: levels by colon, chunks by comma (outside parentheses), parentheses — taxa or nested chunks.
pub fn parse(t: &[Tok]) -> Parsed {
    let mut p = Parser { t, out: Parsed::default(), exclude: Vec::new(), cont: Vec::new(), heads: 0 };
    p.out.content_words = t.iter().filter(|x| is_content(x)).count();
    let mut level: u8 = 0;
    let mut depth = 0usize;
    let mut cur: Vec<usize> = Vec::new();
    let mut paren: Vec<usize> = Vec::new();
    let mut flush_paren = |p: &mut Parser, paren: &mut Vec<usize>, level: u8| {
        let toks: Vec<&Tok> = paren.iter().map(|&i| &t[i]).collect();
        let tx = taxa(&toks);
        if !tx.is_empty() {
            for x in tx {
                p.out.feats.push(Feat::new(Kind::Taxon, level, false, x));
            }
        } else if !toks.iter().any(|x| x.low == "iso" || x.low.starts_with("wco") || x.low == "inn") {
            // nested chunks: take the features, nouns as synonym labels («(trout)», «(endocarp)»)
            let mut part: Vec<usize> = Vec::new();
            let items_before = p.out.feats.len();
            for &i in paren.iter().chain(std::iter::once(&usize::MAX)) {
                if i == usize::MAX || t[i].low == "," || t[i].low == ";" {
                    let is_np = !part.is_empty() && !p.only_lexical(&part) && part.iter().all(|&j| !matches!(t[j].low.as_str(), "of" | "in" | "for" | "with" | "without" | "containing" | "excluding" | "other" | "not" | "including"));
                    if is_np {
                        let (nps, _) = p.nps(&part);
                        for np in nps {
                            p.out.feats.push(Feat::np(Kind::Other, level, false, np));
                        }
                    } else if !part.is_empty() {
                        // «(excluding X)» acts only inside the parentheses: the exclusion and level-list state is restored
                        let (ex, co) = (p.exclude.clone(), p.cont.clone());
                        p.chunk(&part, level, false);
                        p.exclude = ex;
                        p.cont = co;
                    }
                    part.clear();
                } else {
                    part.push(i);
                }
            }
            let _ = items_before;
        }
        paren.clear();
    };
    for i in 0..t.len() {
        let w = t[i].low.as_str();
        if w == "(" {
            depth += 1;
            if depth == 1 {
                continue;
            }
        }
        if w == ")" && depth > 0 {
            depth -= 1;
            if depth == 0 {
                flush_paren(&mut p, &mut paren, level);
                continue;
            }
        }
        if depth > 0 {
            paren.push(i);
            continue;
        }
        if w == ":" {
            chunks(&mut p, &cur, level);
            cur.clear();
            // the list «of X, Y: Z, W» extends across the colon, but not from the heading top («… of vegetable origin: tannins»)
            let inherit = p.cont.get(level as usize).copied().flatten().filter(|k| *k == Kind::Of && level > 0);
            level += 1;
            if p.cont.len() <= level as usize {
                p.cont.resize(level as usize + 1, None);
            }
            p.cont[level as usize] = inherit;
            continue;
        }
        cur.push(i);
    }
    if !paren.is_empty() {
        flush_paren(&mut p, &mut paren, level);
    }
    chunks(&mut p, &cur, level);
    p.out
}

fn chunks(p: &mut Parser, idx: &[usize], level: u8) {
    let mut part: Vec<usize> = Vec::new();
    for &i in idx {
        if p.t[i].low == "," || p.t[i].low == ";" {
            p.chunk(&part, level, false);
            part.clear();
        } else {
            part.push(i);
        }
    }
    p.chunk(&part, level, false);
}

// ── fallback tagger for tests (without the `en` model) ─────────────────────────────────────────────────

/// Rough tokens without a model: numbers, punctuation, prepositions, conjunctions, list words, the rest are nouns.
/// Only for unit tests of threshold and feature parsing.
pub fn naive_toks(text: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    let spaced = text.replace(':', " : ").replace(',', " , ").replace(';', " ; ").replace('(', " ( ").replace(')', " ) ");
    for w in spaced.split_whitespace() {
        let low = w.to_lowercase();
        let upos = if matches!(low.as_str(), ":" | "," | ";" | "(" | ")") {
            UPos::PUNCT
        } else if low.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            UPos::NUM
        } else if matches!(low.as_str(), "of" | "in" | "for" | "with" | "without" | "by" | "from" | "than" | "to" | "on") {
            UPos::ADP
        } else if matches!(low.as_str(), "and" | "or") {
            UPos::CCONJ
        } else if matches!(low.as_str(), "a" | "an" | "the") {
            UPos::DET
        } else if State::parse(&low).is_some() || Process::parse(&low).is_some() || low.starts_with("un") || matches!(low.as_str(), "not" | "exceeding" | "more" | "less" | "over" | "whether" | "other" | "excluding" | "containing" | "edible" | "stainless") {
            UPos::ADJ
        } else if w.chars().next().is_some_and(|c| c.is_uppercase()) && out.last().is_some_and(|t: &Tok| t.low == "(" || t.upos == UPos::PROPN || t.low == ",") {
            UPos::PROPN
        } else {
            UPos::NOUN
        };
        let lemma = if upos == UPos::NOUN && low.len() > 3 && low.ends_with('s') && !low.ends_with("ss") { low[..low.len() - 1].to_string() } else { low.clone() };
        out.push(Tok { form: w.to_string(), low, lemma, upos, head: 0 });
    }
    out
}

#[cfg(test)]
mod t {
    use super::*;

    fn b(s: &str) -> Vec<String> {
        let low = s.to_lowercase();
        let w: Vec<&str> = low.split_whitespace().collect();
        bounds(&w).iter().map(|b| b.show()).collect()
    }

    #[test]
    fn thresholds_exact() {
        // bounds with units — in base units, exactly (math)
        assert_eq!(b("cylinder capacity over 1000 but not over 1500cc"), vec!["Capacity ∈ (0.001, 0.0015] m³"]);
        assert_eq!(b("weighing more than 1 kg"), vec!["Weight ∈ (1, ∞) kg"]);
        assert_eq!(b("of an alcoholic strength by volume of less than 80 % vol"), vec!["Strength ∈ (−∞, 80) % by volume"]);
        assert_eq!(b("containing 85% or more by weight of cotton"), vec!["Share ∈ [85, ∞) % by weight"]);
        assert_eq!(b("weighing not more than 200g/m2"), vec!["Weight ∈ (−∞, 0.2] kg/m²"]);
        assert_eq!(b("in containers holding 2 litres or less"), vec!["Capacity ∈ (−∞, 0.002] m³"]);
        assert_eq!(b("of a width 600mm or more"), vec!["Width ∈ [0.6, ∞) m"]);
        assert_eq!(b("of a thickness of less than 3mm"), vec!["Thickness ∈ (−∞, 0.003) m"]);
        assert!(b("fresh or chilled").is_empty());
    }

    #[test]
    fn bound_overlap() {
        let low = |s: &str| bounds(&s.split_whitespace().collect::<Vec<_>>())[0].clone();
        let a = low("not over 1000cc");
        let c = low("over 1000 but not over 1500cc");
        let d = low("over 1500cc");
        assert!(!a.overlaps(&c) && !c.overlaps(&d) && !a.overlaps(&d));
        let bad = low("not over 1200cc");
        assert!(bad.overlaps(&c), "negative control: a shifted bound must overlap");
    }

    #[test]
    fn features_of_telegraphic_style() {
        // synthetic HS-style rows (not WCO quotes)
        let p = parse(&naive_toks("Pears, edible: dried, in packings for retail sale, of a net weight not exceeding 2 kg"));
        let kinds: Vec<(Kind, String)> = p.feats.iter().map(|f| (f.kind, f.val.clone())).collect();
        assert!(kinds.contains(&(Kind::Head, "edible pear".into())), "{kinds:?}");
        assert!(kinds.contains(&(Kind::State, "dried".into())));
        assert!(kinds.contains(&(Kind::State, "edible".into())) || kinds.contains(&(Kind::Other, "edible".into())));
        assert!(kinds.iter().any(|k| k.0 == Kind::Form && k.1 == "packing"), "{kinds:?}");
        assert!(kinds.contains(&(Kind::Purpose, "retail sale".into())), "{kinds:?}");
        assert!(p.feats.iter().any(|f| f.kind == Kind::Bound && f.bound.as_ref().unwrap().show() == "Weight ∈ (−∞, 2] kg"));
        let q = parse(&naive_toks("Trout: frozen, rainbow trout (Oncorhynchus mykiss), not smoked, of wood, excluding fillets"));
        let main = q.main().unwrap();
        assert_eq!((main.kind, main.val.as_str()), (Kind::Item, "rainbow trout"));
        assert!(q.feats.iter().any(|f| f.kind == Kind::Taxon && f.val == "Oncorhynchus mykiss"));
        assert!(q.feats.iter().any(|f| f.kind == Kind::Process && f.val == "smoked" && f.neg));
        assert!(q.feats.iter().any(|f| f.kind == Kind::Of && f.val == "wood"));
        assert!(q.feats.iter().any(|f| f.kind == Kind::Exclude && f.val == "fillet"));
    }

    #[test]
    fn csv_and_tree() {
        assert_eq!(csv_fields("010121,\"Horses: live, \"\"pure\"\"\""), vec!["010121", "Horses: live, \"pure\""]);
        let codes = vec![("010121".to_string(), String::new()), ("010129".into(), String::new()), ("020110".into(), String::new())];
        let t = Tree::new(&codes);
        assert_eq!((t.count(Level::Chapter), t.count(Level::Heading), t.count(Level::Sub)), (2, 2, 3));
        assert_eq!(t.children("0101"), vec!["010121", "010129"]);
        assert!(residual_code("010129") && residual_code("010190") && !residual_code("010121"));
    }
}
