//! Wikidata: one streaming pass over the full dump → TSV (zstd) for the world model and for our own translation.
//!
//! ```text
//! dump .bz2 ─(copier thread, counts compressed bytes)→ lbzip2 -dc -n K ─→ 8 MiB chunks up to '\n'
//!          ─→ W parser threads ─→ each with its own set of `<table>-<w>.tsv.zst`
//! ```
//!
//! - Parsing borrows strings from the buffer (`Cow`) and skips the remaining fields (`IgnoredAny`). Labels and aliases are first
//!   kept only "raw" (`RawValue`): they are fully parsed only for items that pass the label filter.
//! - Empty maps in the Wikidata dump are written as `[]` (a PHP legacy) — every map visitor also accepts an empty
//!   array.
//! - The dump is read-only; nothing is written into the geography project's directory.
//! - TSV fields without quotes: tab, newline and other control bytes (< 0x20) → space.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use serde::de::{self, Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::value::RawValue;

// ── what we take ───────────────────────────────────────────────────────────────────────────────────

/// Property value type → output table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// item → `claims(s, p, o, rank)`
    Item,
    /// string or URL → `strs(q, p, value)`
    Str,
    /// text with a language → `mono(q, p, lang, text)`
    Mono,
    /// quantity with a unit → `quant(q, p, amount, unit, lower, upper, rank, quals)`
    Quant,
}

pub struct PropSpec {
    pub p: u16,
    pub kind: Kind,
    pub name: &'static str,
    /// why: from the assignment or added by us (with a reason)
    pub why: &'static str,
}

const fn ps(p: u16, kind: Kind, name: &'static str, why: &'static str) -> PropSpec {
    PropSpec { p, kind, name, why }
}

/// Properties of the pass. The order is the number in the statistics; new ones only at the end.
pub const PROPS: &[PropSpec] = &[
    ps(31, Kind::Item, "instance of", "assigned"),
    ps(279, Kind::Item, "subclass of", "assigned; class hierarchy"),
    ps(361, Kind::Item, "part of", "assigned"),
    ps(527, Kind::Item, "has part(s)", "assigned"),
    ps(186, Kind::Item, "made from material", "assigned"),
    ps(171, Kind::Item, "parent taxon", "assigned; taxa do not go through P279 — without P171 a whale is not a mammal"),
    ps(1034, Kind::Item, "main food source", "assigned"),
    ps(4733, Kind::Item, "produced sound", "assigned"),
    ps(1582, Kind::Item, "natural product of taxon", "assigned"),
    ps(105, Kind::Item, "taxon rank", "added: species, genus, family — to tell \"lion\" (species) from \"Felidae\" (family)"),
    ps(1672, Kind::Item, "this taxon is source of", "added: inverse of P1582 (apple tree → apple); both directions are filled incompletely"),
    ps(2670, Kind::Item, "has part(s) of the class", "added: parts of a class (a bird has a beak, a wing); P527 is mostly about instances"),
    ps(3113, Kind::Item, "does not have part", "added: an exception for seeds (a snake has no legs)"),
    ps(462, Kind::Item, "color", "added: colour — common knowledge about things"),
    ps(1419, Kind::Item, "shape", "added: shape — common knowledge about things"),
    ps(2974, Kind::Item, "habitat", "added: where an animal lives (a fish — in water)"),
    ps(366, Kind::Item, "has use", "added: what a thing is for (an apple — food)"),
    ps(1552, Kind::Item, "has characteristic", "added: characteristic properties of a class"),
    ps(3094, Kind::Item, "develops from", "added: life cycle (tadpole → frog)"),
    ps(225, Kind::Str, "taxon name", "assigned"),
    ps(2888, Kind::Str, "exact match", "assigned; URL — in a separate strings table"),
    ps(1709, Kind::Str, "equivalent class", "added: like P2888 — links to external ontologies"),
    ps(5063, Kind::Str, "Interlingual Index ID", "added: ILI — a direct key to OEWN synsets (the `onto` snapshot has ILI)"),
    ps(8814, Kind::Str, "WordNet 3.1 Synset ID", "added: a WordNet synset directly on the item, if present"),
    ps(1843, Kind::Mono, "taxon common name", "assigned; with a language"),
    ps(2067, Kind::Quant, "mass", "assigned"),
    ps(2043, Kind::Quant, "length", "assigned"),
    ps(2048, Kind::Quant, "height", "assigned"),
    ps(2052, Kind::Quant, "speed", "added: \"a hare is faster than a tortoise\" (onto README, \"What next\", item 7)"),
    ps(2250, Kind::Quant, "life expectancy", "added: how long it lives — comparing animals"),
];

const PIDX_N: usize = 16384;

const fn build_pidx() -> [u8; PIDX_N] {
    let mut t = [u8::MAX; PIDX_N];
    let mut i = 0;
    while i < PROPS.len() {
        t[PROPS[i].p as usize] = i as u8;
        i += 1;
    }
    t
}

/// Property number → index in `PROPS` (255 — not taken).
static PIDX: [u8; PIDX_N] = build_pidx();

/// Taxon classes for P31 (besides P171/P225, which also make an item a taxon).
pub const TAXON_CLASSES: &[u32] = &[16521, 310890, 23038290, 713623, 47487597, 98961713];

/// Wiki maintenance pages (category, template, disambiguation, module, project page, portal) — about the wiki,
/// not about the world. Marked with a flag; whether to take their labels and sitelinks — `Cfg::skip_wm`.
pub const WM_CLASSES: &[u32] = &[4167836, 11266439, 4167410, 15184295, 14204246, 4663903];

/// Scholarly article.
pub const SCHOLARLY: u32 = 13442814;

/// Sites with the `wiki` suffix that are not Wikipedias.
pub const NOT_WIKIPEDIA: &[&str] = &[
    "commonswiki",
    "specieswiki",
    "metawiki",
    "mediawikiwiki",
    "wikidatawiki",
    "sourceswiki",
    "outreachwiki",
    "wikimaniawiki",
    "foundationwiki",
    "incubatorwiki",
    "wikifunctionswiki",
    "testwiki",
    "test2wiki",
    "testwikidatawiki",
    "testcommonswiki",
    "strategywiki",
    "officewiki",
    // Abstract Wikipedia: page titles are Q-ids, not language articles
    "abstractwiki",
];

/// The site is a language Wikipedia: `enwiki`, `ukwiki`, `be_x_oldwiki`, `simplewiki`; without Commons, Species, Meta,
/// Wikisource (`enwikisource`), Wikiquote etc.
pub fn is_wikipedia(site: &str) -> bool {
    site.len() > 4 && site.ends_with("wiki") && !NOT_WIKIPEDIA.contains(&site)
}

// ── item flags (the `flags` column of the `items` table) ─────────────────────────────────────────

pub const F_P279: u32 = 1;
pub const F_TAXON: u32 = 2;
pub const F_WM: u32 = 4;
pub const F_LABELS: u32 = 8;
pub const F_SCHOLARLY: u32 = 16;
pub const F_P31: u32 = 32;

// ── parsing ───────────────────────────────────────────────────────────────────────────────────────

/// `Q123` → 123 (with the required letter).
pub fn num_id(s: &str, letter: u8) -> Option<u32> {
    let b = s.as_bytes();
    if b.len() < 2 || b[0] != letter {
        return None;
    }
    let mut n: u32 = 0;
    for &c in &b[1..] {
        if !c.is_ascii_digit() {
            return None;
        }
        n = n.checked_mul(10)?.checked_add((c - b'0') as u32)?;
    }
    Some(n)
}

#[derive(Deserialize)]
pub(crate) struct Text<'a> {
    #[serde(borrow)]
    pub(crate) value: Cow<'a, str>,
}

/// Descriptions: only en and uk.
#[derive(Deserialize, Default)]
struct EnUk<'a> {
    #[serde(borrow, default)]
    en: Option<Text<'a>>,
    #[serde(borrow, default)]
    uk: Option<Text<'a>>,
}

/// Accepts a JSON object as a structure, and an empty array `[]` as the default value.
pub(crate) fn map_or_seq<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    struct V<T>(PhantomData<T>);
    impl<'de, T: Deserialize<'de> + Default> Visitor<'de> for V<T> {
        type Value = T;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an object or an empty array")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<T, A::Error> {
            while a.next_element::<IgnoredAny>()?.is_some() {}
            Ok(T::default())
        }
        fn visit_map<A: MapAccess<'de>>(self, a: A) -> Result<T, A::Error> {
            T::deserialize(de::value::MapAccessDeserializer::new(a))
        }
    }
    d.deserialize_any(V(PhantomData))
}

/// Labels, of which the `items` table needs only en, uk, mul — and the number of languages.
#[derive(Default)]
struct LabelPick<'a> {
    en: Option<Cow<'a, str>>,
    uk: Option<Cow<'a, str>>,
    mul: Option<Cow<'a, str>>,
    n: u32,
}

impl<'de: 'a, 'a> Deserialize<'de> for LabelPick<'a> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<'a>(PhantomData<&'a ()>);
        impl<'de: 'a, 'a> Visitor<'de> for V<'a> {
            type Value = LabelPick<'a>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("labels")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                while a.next_element::<IgnoredAny>()?.is_some() {}
                Ok(LabelPick::default())
            }
            fn visit_map<A: MapAccess<'de>>(self, mut m: A) -> Result<Self::Value, A::Error> {
                let mut p = LabelPick::default();
                while let Some(k) = m.next_key::<&'a str>()? {
                    p.n += 1;
                    match k {
                        "en" => p.en = Some(m.next_value::<Text<'a>>()?.value),
                        "uk" => p.uk = Some(m.next_value::<Text<'a>>()?.value),
                        "mul" => p.mul = Some(m.next_value::<Text<'a>>()?.value),
                        _ => {
                            m.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                Ok(p)
            }
        }
        d.deserialize_any(V(PhantomData))
    }
}

/// All labels or aliases: (language, text). `aliases` is a map language → array.
#[derive(Default)]
pub(crate) struct Terms<'a>(pub(crate) Vec<(&'a str, Cow<'a, str>)>);

/// Map language → text (lemmas, form representations, lexeme sense glosses).
impl<'de: 'a, 'a> Deserialize<'de> for Terms<'a> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(TermsV { aliases: false, _p: PhantomData })
    }
}

struct TermsV<'a> {
    aliases: bool,
    _p: PhantomData<&'a ()>,
}

impl<'de: 'a, 'a> Visitor<'de> for TermsV<'a> {
    type Value = Terms<'a>;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("labels or aliases")
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
        while a.next_element::<IgnoredAny>()?.is_some() {}
        Ok(Terms::default())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut m: A) -> Result<Self::Value, A::Error> {
        let mut v = Vec::new();
        while let Some(k) = m.next_key::<&'a str>()? {
            if self.aliases {
                for t in m.next_value::<Vec<Text<'a>>>()? {
                    v.push((k, t.value));
                }
            } else {
                v.push((k, m.next_value::<Text<'a>>()?.value));
            }
        }
        Ok(Terms(v))
    }
}

fn parse_terms<'a>(raw: &'a RawValue, aliases: bool) -> Result<Terms<'a>, serde_json::Error> {
    let mut d = serde_json::Deserializer::from_str(raw.get());
    d.deserialize_any(TermsV { aliases, _p: PhantomData })
}

/// Sitelinks: the count of all and the article titles of language Wikipedias.
#[derive(Default)]
struct Sitelinks<'a> {
    n: u32,
    wp: Vec<(&'a str, Cow<'a, str>)>,
}

#[derive(Deserialize)]
struct SiteTitle<'a> {
    #[serde(borrow)]
    title: Cow<'a, str>,
}

impl<'de: 'a, 'a> Deserialize<'de> for Sitelinks<'a> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<'a>(PhantomData<&'a ()>);
        impl<'de: 'a, 'a> Visitor<'de> for V<'a> {
            type Value = Sitelinks<'a>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("sitelinks")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                while a.next_element::<IgnoredAny>()?.is_some() {}
                Ok(Sitelinks::default())
            }
            fn visit_map<A: MapAccess<'de>>(self, mut m: A) -> Result<Self::Value, A::Error> {
                let mut s = Sitelinks::default();
                while let Some(k) = m.next_key::<&'a str>()? {
                    s.n += 1;
                    if is_wikipedia(k) {
                        s.wp.push((k, m.next_value::<SiteTitle<'a>>()?.title));
                    } else {
                        m.next_value::<IgnoredAny>()?;
                    }
                }
                Ok(s)
            }
        }
        d.deserialize_any(V(PhantomData))
    }
}

/// Snak value: a string (string, url, external-id) or an object (item, quantity, text with a language).
pub(crate) enum Val<'a> {
    Str(Cow<'a, str>),
    Obj(Obj<'a>),
    Other,
}

#[derive(Deserialize, Default)]
pub(crate) struct Obj<'a> {
    #[serde(borrow, default)]
    pub(crate) id: Option<Cow<'a, str>>,
    #[serde(rename = "numeric-id", default)]
    numeric_id: Option<u64>,
    #[serde(borrow, default)]
    amount: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    unit: Option<Cow<'a, str>>,
    #[serde(rename = "upperBound", borrow, default)]
    upper: Option<Cow<'a, str>>,
    #[serde(rename = "lowerBound", borrow, default)]
    lower: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    text: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    language: Option<Cow<'a, str>>,
}

impl<'de: 'a, 'a> Deserialize<'de> for Val<'a> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<'a>(PhantomData<&'a ()>);
        impl<'de: 'a, 'a> Visitor<'de> for V<'a> {
            type Value = Val<'a>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("snak value")
            }
            fn visit_borrowed_str<E: de::Error>(self, v: &'de str) -> Result<Val<'a>, E> {
                Ok(Val::Str(Cow::Borrowed(v)))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Val<'a>, E> {
                Ok(Val::Str(Cow::Owned(v.to_owned())))
            }
            fn visit_string<E: de::Error>(self, v: String) -> Result<Val<'a>, E> {
                Ok(Val::Str(Cow::Owned(v)))
            }
            fn visit_map<A: MapAccess<'de>>(self, a: A) -> Result<Val<'a>, A::Error> {
                Obj::deserialize(de::value::MapAccessDeserializer::new(a)).map(Val::Obj)
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Val<'a>, A::Error> {
                while a.next_element::<IgnoredAny>()?.is_some() {}
                Ok(Val::Other)
            }
            fn visit_bool<E: de::Error>(self, _: bool) -> Result<Val<'a>, E> {
                Ok(Val::Other)
            }
            fn visit_i64<E: de::Error>(self, _: i64) -> Result<Val<'a>, E> {
                Ok(Val::Other)
            }
            fn visit_u64<E: de::Error>(self, _: u64) -> Result<Val<'a>, E> {
                Ok(Val::Other)
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> Result<Val<'a>, E> {
                Ok(Val::Other)
            }
            fn visit_unit<E: de::Error>(self) -> Result<Val<'a>, E> {
                Ok(Val::Other)
            }
        }
        d.deserialize_any(V(PhantomData))
    }
}

impl Val<'_> {
    /// Item number: `id` ("Q123"), and in old dumps — `numeric-id`.
    pub(crate) fn item(&self) -> Option<u32> {
        match self {
            Val::Obj(o) => match &o.id {
                Some(id) => num_id(id, b'Q'),
                None => o.numeric_id.and_then(|n| u32::try_from(n).ok()),
            },
            _ => None,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Dv<'a> {
    #[serde(borrow)]
    pub(crate) value: Val<'a>,
}

#[derive(Deserialize)]
pub(crate) struct Snak<'a> {
    #[serde(borrow, default)]
    pub(crate) datavalue: Option<Dv<'a>>,
}

#[derive(Deserialize)]
pub(crate) struct Stmt<'a> {
    #[serde(borrow)]
    pub(crate) mainsnak: Snak<'a>,
    #[serde(borrow, default)]
    pub(crate) rank: Option<Cow<'a, str>>,
}

/// A statement with qualifiers — only for quantities (sex, role, age: mass of an adult male lion).
#[derive(Deserialize)]
struct StmtQ<'a> {
    #[serde(borrow)]
    mainsnak: Snak<'a>,
    #[serde(borrow, default)]
    rank: Option<Cow<'a, str>>,
    #[serde(borrow, default, deserialize_with = "map_or_seq")]
    qualifiers: HashMap<&'a str, Vec<Snak<'a>>>,
}

pub(crate) fn rank_code(r: &Option<Cow<str>>) -> u8 {
    match r.as_deref() {
        Some("deprecated") => 0,
        Some("preferred") => 2,
        _ => 1,
    }
}

/// Quantity: "+1.65" → "1.65"; unit `http://www.wikidata.org/entity/Q11570` → 11570, "1" → 0.
fn amount(s: &str) -> &str {
    s.strip_prefix('+').unwrap_or(s)
}

fn unit_q(u: &str) -> u32 {
    u.rsplit('/').next().and_then(|t| num_id(t, b'Q')).unwrap_or(0)
}

/// Qualifiers in one line: `P21=Q6581097;P3831=Q30`.
fn quals_str(q: &HashMap<&str, Vec<Snak>>) -> String {
    let mut keys: Vec<&&str> = q.keys().collect();
    keys.sort();
    let mut s = String::new();
    for k in keys {
        for sn in &q[*k] {
            let Some(dv) = &sn.datavalue else { continue };
            let v = match &dv.value {
                Val::Str(x) => x.to_string(),
                Val::Obj(o) => {
                    if let Some(id) = &o.id {
                        id.to_string()
                    } else if let Some(a) = &o.amount {
                        let u = o.unit.as_deref().map(unit_q).unwrap_or(0);
                        if u == 0 { amount(a).to_string() } else { format!("{}Q{u}", amount(a)) }
                    } else if let (Some(t), Some(l)) = (&o.text, &o.language) {
                        format!("{t}@{l}")
                    } else {
                        continue;
                    }
                }
                Val::Other => continue,
            };
            if !s.is_empty() {
                s.push(';');
            }
            s.push_str(k);
            s.push('=');
            s.push_str(&v.replace([';', '='], " "));
        }
    }
    s
}

/// A quantity row.
struct QuantRow<'a> {
    pi: u8,
    amount: Cow<'a, str>,
    unit: u32,
    lower: Option<Cow<'a, str>>,
    upper: Option<Cow<'a, str>>,
    rank: u8,
    quals: String,
}

/// Statements of the needed properties.
#[derive(Default)]
struct Claims<'a> {
    /// (index in PROPS, item, rank)
    items: Vec<(u8, u32, u8)>,
    strs: Vec<(u8, Cow<'a, str>, u8)>,
    monos: Vec<(u8, Cow<'a, str>, Cow<'a, str>, u8)>,
    quants: Vec<QuantRow<'a>>,
    /// statements without a value (somevalue/novalue) — counted only
    novalue: u32,
    /// how many distinct properties the item has (for statistics)
    nprops: u32,
}

impl<'de: 'a, 'a> Deserialize<'de> for Claims<'a> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<'a>(PhantomData<&'a ()>);
        impl<'de: 'a, 'a> Visitor<'de> for V<'a> {
            type Value = Claims<'a>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("statements")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                while a.next_element::<IgnoredAny>()?.is_some() {}
                Ok(Claims::default())
            }
            fn visit_map<A: MapAccess<'de>>(self, mut m: A) -> Result<Self::Value, A::Error> {
                let mut c = Claims::default();
                while let Some(k) = m.next_key::<&'a str>()? {
                    c.nprops += 1;
                    let pi = num_id(k, b'P').filter(|&p| (p as usize) < PIDX_N).map(|p| PIDX[p as usize]).unwrap_or(u8::MAX);
                    if pi == u8::MAX {
                        m.next_value::<IgnoredAny>()?;
                        continue;
                    }
                    match PROPS[pi as usize].kind {
                        Kind::Quant => {
                            for s in m.next_value::<Vec<StmtQ<'a>>>()? {
                                let r = rank_code(&s.rank);
                                match s.mainsnak.datavalue {
                                    Some(Dv { value: Val::Obj(o) }) if o.amount.is_some() => c.quants.push(QuantRow {
                                        pi,
                                        amount: o.amount.unwrap_or_default(),
                                        unit: o.unit.as_deref().map(unit_q).unwrap_or(0),
                                        lower: o.lower,
                                        upper: o.upper,
                                        rank: r,
                                        quals: quals_str(&s.qualifiers),
                                    }),
                                    _ => c.novalue += 1,
                                }
                            }
                        }
                        kind => {
                            for s in m.next_value::<Vec<Stmt<'a>>>()? {
                                let r = rank_code(&s.rank);
                                let Some(dv) = s.mainsnak.datavalue else {
                                    c.novalue += 1;
                                    continue;
                                };
                                match (kind, dv.value) {
                                    (Kind::Item, v) => match v.item() {
                                        Some(o) => c.items.push((pi, o, r)),
                                        None => c.novalue += 1,
                                    },
                                    (Kind::Str, Val::Str(x)) => c.strs.push((pi, x, r)),
                                    (Kind::Mono, Val::Obj(o)) => match (o.language, o.text) {
                                        (Some(l), Some(t)) => c.monos.push((pi, l, t, r)),
                                        _ => c.novalue += 1,
                                    },
                                    _ => c.novalue += 1,
                                }
                            }
                        }
                    }
                }
                Ok(c)
            }
        }
        d.deserialize_any(V(PhantomData))
    }
}

/// A dump entity: an item (`item`) or a property (`property`).
#[derive(Deserialize)]
struct Ent<'a> {
    #[serde(rename = "type", borrow)]
    typ: Cow<'a, str>,
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow, default)]
    datatype: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    labels: Option<&'a RawValue>,
    #[serde(borrow, default, deserialize_with = "map_or_seq")]
    descriptions: EnUk<'a>,
    #[serde(borrow, default)]
    aliases: Option<&'a RawValue>,
    #[serde(borrow, default)]
    claims: Claims<'a>,
    #[serde(borrow, default)]
    sitelinks: Sitelinks<'a>,
}

// ── output ────────────────────────────────────────────────────────────────────────────────────────

/// Output tables and their columns — checked against `sql/wikidata-load.sql` by a test.
#[cfg_attr(not(test), allow(dead_code))]
pub const COLUMNS: &[(&str, &str)] = &[
    ("items", "q, label_en, label_mul, label_uk, desc_en, desc_uk, n_sitelinks, n_wiki, n_labels, flags"),
    ("labels", "q, lang, label, is_alias"),
    ("sitelinks", "q, site, title"),
    ("claims", "s, p, o, rank"),
    ("strs", "q, p, value, rank"),
    ("mono", "q, p, lang, text, rank"),
    ("quant", "q, p, amount, unit, lower, upper, rank, quals"),
    ("props", "p, datatype, label_en, label_uk, desc_en"),
];

/// One TSV file, compressed with zstd.
pub struct Tsv {
    enc: zstd::stream::write::Encoder<'static, BufWriter<File>>,
    buf: Vec<u8>,
    pub rows: u64,
    pub raw_bytes: u64,
    path: PathBuf,
}

impl Tsv {
    pub fn create(path: PathBuf, level: i32) -> Result<Tsv> {
        let f = File::create(&path).with_context(|| format!("cannot create {}", path.display()))?;
        let enc = zstd::stream::write::Encoder::new(BufWriter::with_capacity(1 << 20, f), level)?;
        Ok(Tsv { enc, buf: Vec::with_capacity(1 << 19), rows: 0, raw_bytes: 0, path })
    }

    /// A string field: control bytes (< 0x20: tab, newline) → space.
    #[inline]
    pub fn s(&mut self, x: &str) -> &mut Self {
        let b = x.as_bytes();
        if b.iter().any(|&c| c < 0x20) {
            self.buf.extend(b.iter().map(|&c| if c < 0x20 { b' ' } else { c }));
        } else {
            self.buf.extend_from_slice(b);
        }
        self
    }

    #[inline]
    pub fn n(&mut self, v: u64) -> &mut Self {
        let mut b = itoa::Buffer::new();
        self.buf.extend_from_slice(b.format(v).as_bytes());
        self
    }

    #[inline]
    pub fn t(&mut self) -> &mut Self {
        self.buf.push(b'\t');
        self
    }

    #[inline]
    pub fn end(&mut self) -> Result<()> {
        self.buf.push(b'\n');
        self.rows += 1;
        if self.buf.len() >= 1 << 18 {
            self.flush_buf()?;
        }
        Ok(())
    }

    fn flush_buf(&mut self) -> Result<()> {
        self.raw_bytes += self.buf.len() as u64;
        self.enc.write_all(&self.buf)?;
        self.buf.clear();
        Ok(())
    }

    /// Finish and close: (path, rows, TSV bytes, compressed bytes).
    pub fn finish(mut self) -> Result<(PathBuf, u64, u64, u64)> {
        self.flush_buf()?;
        let mut w = self.enc.finish()?;
        w.flush()?;
        drop(w);
        let sz = std::fs::metadata(&self.path)?.len();
        Ok((self.path, self.rows, self.raw_bytes, sz))
    }
}

/// The set of files of one parser thread.
pub struct Outs {
    items: Tsv,
    labels: Tsv,
    sitelinks: Tsv,
    claims: Tsv,
    strs: Tsv,
    mono: Tsv,
    quant: Tsv,
    props: Tsv,
}

impl Outs {
    pub fn create(dir: &Path, part: &str, level: i32) -> Result<Outs> {
        let f = |t: &str| Tsv::create(dir.join(format!("{t}-{part}.tsv.zst")), level);
        Ok(Outs { items: f("items")?, labels: f("labels")?, sitelinks: f("sitelinks")?, claims: f("claims")?, strs: f("strs")?, mono: f("mono")?, quant: f("quant")?, props: f("props")? })
    }

    pub fn finish(self) -> Result<Vec<(String, PathBuf, u64, u64, u64)>> {
        let mut v = Vec::new();
        for (name, t) in [("items", self.items), ("labels", self.labels), ("sitelinks", self.sitelinks), ("claims", self.claims), ("strs", self.strs), ("mono", self.mono), ("quant", self.quant), ("props", self.props)] {
            let (p, r, raw, z) = t.finish()?;
            v.push((name.to_string(), p, r, raw, z));
        }
        Ok(v)
    }
}

// ── settings and statistics ───────────────────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct Cfg {
    /// wiki maintenance pages: without labels in all languages and without sitelinks
    pub skip_wm: bool,
    /// only these label languages (None — all)
    pub langs: Option<Arc<HashSet<String>>>,
}

/// Item categories for volume estimation: 0 — the rest, 1 — wiki maintenance pages, 2 — scholarly articles.
pub const CATS: [&str; 3] = ["the rest", "wiki maintenance pages", "scholarly articles"];

#[derive(Default, Clone)]
pub struct Stats {
    pub lines: u64,
    pub bad: u64,
    pub bad_examples: Vec<String>,
    pub items: u64,
    pub props: u64,
    pub other: u64,
    pub saw_open: bool,
    pub saw_close: bool,
    pub max_q: u32,
    pub label_en: u64,
    pub label_mul: u64,
    pub label_uk: u64,
    pub desc_en: u64,
    pub desc_uk: u64,
    pub p279: u64,
    pub p31: u64,
    pub taxa: u64,
    pub wm: u64,
    pub scholarly: u64,
    pub with_sitelinks: u64,
    pub with_wiki: u64,
    pub exported: u64,
    pub label_rows: u64,
    pub alias_rows: u64,
    pub sitelinks_all: u64,
    pub sitelink_rows: u64,
    pub novalue: u64,
    /// rows per property (PROPS index) × rank (0 — deprecated, 1 — normal, 2 — preferred)
    pub per_prop: Vec<[u64; 3]>,
    /// language → (labels, aliases)
    pub langs: HashMap<String, [u64; 2]>,
    pub sites: HashMap<String, u64>,
    /// per category: label+alias rows, bytes of their text, bytes of `items` text, sitelink rows
    pub cat_label_rows: [u64; 3],
    pub cat_label_bytes: [u64; 3],
    pub cat_item_bytes: [u64; 3],
    pub cat_sitelinks: [u64; 3],
    pub cat_items: [u64; 3],
}

impl Stats {
    pub fn new() -> Stats {
        Stats { per_prop: vec![[0; 3]; PROPS.len()], ..Default::default() }
    }

    pub fn merge(&mut self, o: &Stats) {
        macro_rules! add {
            ($($f:ident),*) => { $( self.$f += o.$f; )* };
        }
        add!(lines, bad, items, props, other, label_en, label_mul, label_uk, desc_en, desc_uk, p279, p31, taxa, wm, scholarly, with_sitelinks, with_wiki, exported, label_rows, alias_rows, sitelinks_all, sitelink_rows, novalue);
        self.saw_open |= o.saw_open;
        self.saw_close |= o.saw_close;
        self.max_q = self.max_q.max(o.max_q);
        for e in &o.bad_examples {
            if self.bad_examples.len() < 20 {
                self.bad_examples.push(e.clone());
            }
        }
        for (i, r) in o.per_prop.iter().enumerate() {
            for k in 0..3 {
                self.per_prop[i][k] += r[k];
            }
        }
        for (l, c) in &o.langs {
            let e = self.langs.entry(l.clone()).or_default();
            e[0] += c[0];
            e[1] += c[1];
        }
        for (s, c) in &o.sites {
            *self.sites.entry(s.clone()).or_default() += c;
        }
        for k in 0..3 {
            self.cat_label_rows[k] += o.cat_label_rows[k];
            self.cat_label_bytes[k] += o.cat_label_bytes[k];
            self.cat_item_bytes[k] += o.cat_item_bytes[k];
            self.cat_sitelinks[k] += o.cat_sitelinks[k];
            self.cat_items[k] += o.cat_items[k];
        }
    }
}

// ── processing one line ─────────────────────────────────────────────────────────────────────────

/// A dump line → records in the tables. An error is only I/O (fail-fast); broken JSON is counted.
pub fn process_line(line: &[u8], o: &mut Outs, st: &mut Stats, cfg: &Cfg) -> Result<()> {
    let mut l = line;
    while let Some((&last, rest)) = l.split_last() {
        if last == b'\r' || last == b',' || last == b' ' {
            l = rest;
        } else {
            break;
        }
    }
    match l {
        b"" => return Ok(()),
        b"[" => {
            st.saw_open = true;
            return Ok(());
        }
        b"]" => {
            st.saw_close = true;
            return Ok(());
        }
        _ => {}
    }
    st.lines += 1;
    let e: Ent = match serde_json::from_slice(l) {
        Ok(e) => e,
        Err(err) => {
            st.bad += 1;
            if st.bad_examples.len() < 20 {
                let head: String = String::from_utf8_lossy(&l[..l.len().min(60)]).into_owned();
                st.bad_examples.push(format!("{err} — {head}"));
            }
            return Ok(());
        }
    };
    if let Err(err) = emit(&e, o, st, cfg) {
        // secondary parsing of raw labels: same as broken JSON; I/O — refusal
        if err.downcast_ref::<serde_json::Error>().is_some() {
            st.bad += 1;
            if st.bad_examples.len() < 20 {
                st.bad_examples.push(format!("{err} — {}", e.id));
            }
            return Ok(());
        }
        return Err(err);
    }
    Ok(())
}

fn emit(e: &Ent, o: &mut Outs, st: &mut Stats, cfg: &Cfg) -> Result<()> {
    match &*e.typ {
        "item" => {}
        "property" => {
            st.props += 1;
            let Some(p) = num_id(&e.id, b'P') else {
                st.bad += 1;
                return Ok(());
            };
            let lab: LabelPick = match e.labels {
                Some(r) => serde_json::from_str(r.get())?,
                None => LabelPick::default(),
            };
            let de = e.descriptions.en.as_ref().map(|t| t.value.as_ref()).unwrap_or("");
            o.props.n(p as u64).t().s(e.datatype.as_deref().unwrap_or("")).t().s(lab.en.as_deref().unwrap_or("")).t().s(lab.uk.as_deref().unwrap_or("")).t().s(de).end()?;
            return Ok(());
        }
        _ => {
            st.other += 1;
            return Ok(());
        }
    }
    let Some(q) = num_id(&e.id, b'Q') else {
        st.bad += 1;
        if st.bad_examples.len() < 20 {
            st.bad_examples.push(format!("not a Q-id: {}", e.id));
        }
        return Ok(());
    };
    st.items += 1;
    st.max_q = st.max_q.max(q);
    let c = &e.claims;
    let (i31, i279, i171) = (PIDX[31], PIDX[279], PIDX[171]);
    let live = |pi: u8| c.items.iter().any(|&(p, _, r)| p == pi && r > 0);
    let p31_has = |set: &[u32]| c.items.iter().any(|&(p, o, r)| p == i31 && r > 0 && set.contains(&o));
    let has279 = live(i279);
    let has31 = live(i31);
    let taxon = live(i171) || c.strs.iter().any(|(p, _, r)| PROPS[*p as usize].p == 225 && *r > 0) || p31_has(TAXON_CLASSES);
    let wm = p31_has(WM_CLASSES);
    let scholarly = p31_has(&[SCHOLARLY]);
    let cat = if scholarly { 2 } else if wm { 1 } else { 0 };
    let sl = &e.sitelinks;
    let skip_wm = cfg.skip_wm && wm;
    let export = (has279 || sl.n > 0 || taxon) && !skip_wm;
    let lab: LabelPick = match e.labels {
        Some(r) => serde_json::from_str(r.get())?,
        None => LabelPick::default(),
    };
    let de = e.descriptions.en.as_ref().map(|t| t.value.as_ref()).unwrap_or("");
    let du = e.descriptions.uk.as_ref().map(|t| t.value.as_ref()).unwrap_or("");
    let (le, lm, lu) = (lab.en.as_deref().unwrap_or(""), lab.mul.as_deref().unwrap_or(""), lab.uk.as_deref().unwrap_or(""));
    let mut flags = 0u32;
    flags |= if has279 { F_P279 } else { 0 };
    flags |= if taxon { F_TAXON } else { 0 };
    flags |= if wm { F_WM } else { 0 };
    flags |= if export { F_LABELS } else { 0 };
    flags |= if scholarly { F_SCHOLARLY } else { 0 };
    flags |= if has31 { F_P31 } else { 0 };
    let n_wiki = sl.wp.len() as u64;
    o.items.n(q as u64).t().s(le).t().s(lm).t().s(lu).t().s(de).t().s(du).t().n(sl.n as u64).t().n(n_wiki).t().n(lab.n as u64).t().n(flags as u64).end()?;
    st.cat_items[cat] += 1;
    st.cat_item_bytes[cat] += (le.len() + lm.len() + lu.len() + de.len() + du.len() + 24) as u64;
    st.label_en += !le.is_empty() as u64;
    st.label_mul += !lm.is_empty() as u64;
    st.label_uk += !lu.is_empty() as u64;
    st.desc_en += !de.is_empty() as u64;
    st.desc_uk += !du.is_empty() as u64;
    st.p279 += has279 as u64;
    st.p31 += has31 as u64;
    st.taxa += taxon as u64;
    st.wm += wm as u64;
    st.scholarly += scholarly as u64;
    st.with_sitelinks += (sl.n > 0) as u64;
    st.with_wiki += (n_wiki > 0) as u64;
    st.sitelinks_all += sl.n as u64;
    st.novalue += c.novalue as u64;
    if export {
        st.exported += 1;
        for (raw, alias) in [(e.labels, false), (e.aliases, true)] {
            let Some(raw) = raw else { continue };
            let t = parse_terms(raw, alias)?;
            for (lang, v) in &t.0 {
                if let Some(ls) = &cfg.langs {
                    if !ls.contains(*lang) {
                        continue;
                    }
                }
                o.labels.n(q as u64).t().s(lang).t().s(v).t().n(alias as u64).end()?;
                if alias {
                    st.alias_rows += 1;
                } else {
                    st.label_rows += 1;
                }
                st.cat_label_rows[cat] += 1;
                st.cat_label_bytes[cat] += (v.len() + lang.len() + 12) as u64;
                match st.langs.get_mut(*lang) {
                    Some(x) => x[alias as usize] += 1,
                    None => {
                        let mut x = [0u64; 2];
                        x[alias as usize] += 1;
                        st.langs.insert(lang.to_string(), x);
                    }
                }
            }
        }
    }
    if !skip_wm {
        for (site, title) in &sl.wp {
            o.sitelinks.n(q as u64).t().s(site).t().s(title).end()?;
            st.sitelink_rows += 1;
            st.cat_sitelinks[cat] += 1;
            match st.sites.get_mut(*site) {
                Some(x) => *x += 1,
                None => {
                    st.sites.insert(site.to_string(), 1);
                }
            }
        }
    }
    for &(pi, obj, r) in &c.items {
        o.claims.n(q as u64).t().n(PROPS[pi as usize].p as u64).t().n(obj as u64).t().n(r as u64).end()?;
        st.per_prop[pi as usize][r as usize] += 1;
    }
    for (pi, v, r) in &c.strs {
        o.strs.n(q as u64).t().n(PROPS[*pi as usize].p as u64).t().s(v).t().n(*r as u64).end()?;
        st.per_prop[*pi as usize][*r as usize] += 1;
    }
    for (pi, lang, text, r) in &c.monos {
        o.mono.n(q as u64).t().n(PROPS[*pi as usize].p as u64).t().s(lang).t().s(text).t().n(*r as u64).end()?;
        st.per_prop[*pi as usize][*r as usize] += 1;
    }
    for x in &c.quants {
        o.quant.n(q as u64).t().n(PROPS[x.pi as usize].p as u64).t().s(amount(&x.amount)).t().n(x.unit as u64).t();
        o.quant.s(x.lower.as_deref().map(amount).unwrap_or("")).t().s(x.upper.as_deref().map(amount).unwrap_or("")).t().n(x.rank as u64).t().s(&x.quals).end()?;
        st.per_prop[x.pi as usize][x.rank as usize] += 1;
    }
    Ok(())
}

// ── parallel pass ───────────────────────────────────────────────────────────────────────────

/// Counters for the progress log.
#[derive(Default)]
pub struct Progress {
    pub bz_bytes: AtomicU64,
    pub json_bytes: AtomicU64,
    pub ents: AtomicU64,
    pub abort: AtomicBool,
}

const CHUNK: usize = 8 << 20;

/// A stream of JSON lines → W parser threads; each has its own files `<table>-<prefix><w>.tsv.zst`.
pub fn run_stream<R: Read>(input: R, out: &Path, prefix: &str, workers: usize, level: i32, cfg: &Cfg, pg: &Arc<Progress>) -> Result<(Stats, Vec<(String, PathBuf, u64, u64, u64)>)> {
    run_stream_chunked(input, out, prefix, workers, level, cfg, pg, CHUNK)
}

/// `run_stream` with a chunk size (tests use tiny chunks to check tails and long lines).
pub fn run_stream_chunked<R: Read>(mut input: R, out: &Path, prefix: &str, workers: usize, level: i32, cfg: &Cfg, pg: &Arc<Progress>, chunk: usize) -> Result<(Stats, Vec<(String, PathBuf, u64, u64, u64)>)> {
    let (tx, rx) = mpsc::sync_channel::<Vec<u8>>(workers * 2);
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = Vec::new();
    for w in 0..workers {
        let rx = Arc::clone(&rx);
        let pg = Arc::clone(pg);
        let cfg = cfg.clone();
        let mut outs = Outs::create(out, &format!("{prefix}{w}"), level)?;
        handles.push(std::thread::Builder::new().name(format!("wd-parse-{w}")).spawn(move || -> Result<(Stats, Vec<(String, PathBuf, u64, u64, u64)>)> {
            let mut st = Stats::new();
            loop {
                let msg = {
                    let g = rx.lock().map_err(|_| anyhow::anyhow!("poisoned mutex"))?;
                    g.recv()
                };
                let Ok(buf) = msg else { break };
                let before = st.lines;
                let mut start = 0;
                for nl in memchr::memchr_iter(b'\n', &buf) {
                    if let Err(e) = process_line(&buf[start..nl], &mut outs, &mut st, &cfg) {
                        pg.abort.store(true, Relaxed);
                        return Err(e);
                    }
                    start = nl + 1;
                }
                if start < buf.len() {
                    process_line(&buf[start..], &mut outs, &mut st, &cfg)?;
                }
                pg.json_bytes.fetch_add(buf.len() as u64, Relaxed);
                pg.ents.fetch_add(st.lines - before, Relaxed);
                if st.bad > 1000 {
                    pg.abort.store(true, Relaxed);
                    bail!("over 1000 unparsed lines — stopping (fail-fast); examples: {:?}", &st.bad_examples[..st.bad_examples.len().min(5)]);
                }
            }
            let files = outs.finish()?;
            Ok((st, files))
        })?);
    }
    // reader: chunks up to the last '\n', the tail carries over to the next chunk
    let mut carry: Vec<u8> = Vec::new();
    let mut read_err: Option<anyhow::Error> = None;
    loop {
        if pg.abort.load(Relaxed) {
            break;
        }
        // a line longer than a chunk stays in the tail whole — read at least one more chunk
        let target = chunk.max(carry.len() + chunk);
        let mut buf = Vec::with_capacity(target + (1 << 16));
        buf.extend_from_slice(&carry);
        carry.clear();
        let mut eof = false;
        while buf.len() < target {
            let old = buf.len();
            buf.resize(old + (target - old).min(1 << 20), 0);
            match input.read(&mut buf[old..]) {
                Ok(0) => {
                    buf.truncate(old);
                    eof = true;
                    break;
                }
                Ok(n) => buf.truncate(old + n),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => buf.truncate(old),
                Err(e) => {
                    buf.truncate(old);
                    read_err = Some(e.into());
                    eof = true;
                    break;
                }
            }
        }
        if !eof {
            match memchr::memrchr(b'\n', &buf) {
                Some(i) => {
                    carry.extend_from_slice(&buf[i + 1..]);
                    buf.truncate(i + 1);
                }
                None => {
                    // the line is longer than a chunk — keep reading together
                    carry = buf;
                    continue;
                }
            }
        }
        if !buf.is_empty() && tx.send(buf).is_err() {
            break;
        }
        if eof {
            break;
        }
    }
    drop(tx);
    let mut st = Stats::new();
    let mut files = Vec::new();
    let mut errs = Vec::new();
    for h in handles {
        match h.join() {
            Ok(Ok((s, f))) => {
                st.merge(&s);
                files.extend(f);
            }
            Ok(Err(e)) => errs.push(e.to_string()),
            Err(_) => errs.push("parser thread crashed".into()),
        }
    }
    if !errs.is_empty() {
        bail!("parser thread errors: {}", errs.join("; "));
    }
    if let Some(e) = read_err {
        return Err(e.context("reading the JSON stream"));
    }
    Ok((st, files))
}

pub(crate) fn log_line(log: &Mutex<File>, s: &str) {
    eprintln!("{s}");
    if let Ok(mut f) = log.lock() {
        let _ = writeln!(f, "{s}");
        let _ = f.flush();
    }
}

pub(crate) fn hms(s: f64) -> String {
    let s = s.max(0.0) as u64;
    format!("{}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60)
}

pub(crate) fn utc_now() -> String {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    format!("{:02}:{:02}:{:02}Z", t / 3600 % 24, t / 60 % 60, t % 60)
}

fn dir_bytes(dir: &Path, prefix: &str) -> u64 {
    std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| {
                    let n = e.file_name();
                    let n = n.to_string_lossy();
                    n.ends_with(".tsv.zst") && n.rsplit_once('-').map(|(_, t)| t.starts_with(prefix)).unwrap_or(false)
                })
                .filter_map(|e| e.metadata().ok())
                .map(|m| m.len())
                .sum()
        })
        .unwrap_or(0)
}

/// Pass summary.
pub struct RunSummary {
    pub stats: Stats,
    pub files: Vec<(String, PathBuf, u64, u64, u64)>,
    pub wall: f64,
    pub bz_read: u64,
    pub bz_total: u64,
    pub json_bytes: u64,
    pub lbzip2_status: String,
    pub lbzip2_stderr: String,
}

/// A pass over the dump: copier (counts compressed bytes) → `lbzip2 -dc -n K` → `run_stream`.
/// `max_bytes` — a pilot on the first bytes (a truncated end is an expected lbzip2 error).
pub fn extract(dump: &Path, lbzip2: &Path, bz_threads: usize, workers: usize, max_bytes: Option<u64>, out: &Path, prefix: &str, level: i32, cfg: &Cfg, log_path: &Path) -> Result<RunSummary> {
    std::fs::create_dir_all(out)?;
    let log = Arc::new(Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(log_path)?));
    let total = std::fs::metadata(dump).with_context(|| format!("no dump {}", dump.display()))?.len();
    let limit = max_bytes.unwrap_or(total).min(total);
    log_line(&log, &format!("{} start: dump {} ({:.2} GB, reading {:.2} GB), lbzip2 -n {bz_threads}, parser threads {workers}, zstd {level}, output {} (parts «{prefix}»), skip_wm={}, langs={}", utc_now(), dump.display(), total as f64 / 1e9, limit as f64 / 1e9, out.display(), cfg.skip_wm, cfg.langs.as_ref().map(|l| l.len().to_string()).unwrap_or("all".into())));
    let mut child = Command::new(lbzip2)
        .args(["-dc", "-n", &bz_threads.to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("cannot start {}", lbzip2.display()))?;
    let pg = Arc::new(Progress::default());
    let t0 = Instant::now();
    // copier: dump (read-only) → lbzip2 stdin
    let mut stdin = child.stdin.take().context("stdin lbzip2")?;
    let dump_p = dump.to_path_buf();
    let pgc = Arc::clone(&pg);
    let copier = std::thread::spawn(move || -> Result<()> {
        let f = File::open(&dump_p)?;
        let mut r = f.take(limit);
        let mut buf = vec![0u8; 1 << 20];
        loop {
            if pgc.abort.load(Relaxed) {
                break;
            }
            let n = r.read(&mut buf)?;
            if n == 0 {
                break;
            }
            stdin.write_all(&buf[..n])?;
            pgc.bz_bytes.fetch_add(n as u64, Relaxed);
        }
        drop(stdin);
        Ok(())
    });
    let mut stderr = child.stderr.take().context("stderr lbzip2")?;
    let err_reader = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = stderr.read_to_string(&mut s);
        s
    });
    // progress log every minute
    let done = Arc::new(AtomicBool::new(false));
    let (pgl, logl, donel, outl, prefl) = (Arc::clone(&pg), Arc::clone(&log), Arc::clone(&done), out.to_path_buf(), prefix.to_string());
    let logger = std::thread::spawn(move || {
        let mut last = Instant::now();
        let mut last_bz = 0u64;
        while !donel.load(Relaxed) {
            std::thread::sleep(Duration::from_millis(500));
            if last.elapsed() < Duration::from_secs(60) {
                continue;
            }
            last = Instant::now();
            let el = t0.elapsed().as_secs_f64();
            let bz = pgl.bz_bytes.load(Relaxed);
            let js = pgl.json_bytes.load(Relaxed);
            let n = pgl.ents.load(Relaxed);
            let frac = bz as f64 / limit.max(1) as f64;
            let eta = if frac > 0.0 { el / frac - el } else { 0.0 };
            let inst = (bz - last_bz) as f64 / 60.0 / 1e6;
            last_bz = bz;
            log_line(
                &logl,
                &format!(
                    "{} +{}  bz {:.2} GB ({:.1}%, {:.1} MB/s over the minute)  JSON {:.1} GB  entities {:.2} M ({:.0} k/s)  output {:.2} GB  left ~{}",
                    utc_now(),
                    hms(el),
                    bz as f64 / 1e9,
                    frac * 100.0,
                    inst,
                    js as f64 / 1e9,
                    n as f64 / 1e6,
                    n as f64 / el / 1e3,
                    dir_bytes(&outl, &prefl) as f64 / 1e9,
                    hms(eta)
                ),
            );
        }
    });
    let stdout = child.stdout.take().context("stdout lbzip2")?;
    let res = run_stream(stdout, out, prefix, workers, level, cfg, &pg);
    if res.is_err() {
        pg.abort.store(true, Relaxed);
        let _ = child.kill();
    }
    let status = child.wait()?;
    let lb_err = err_reader.join().unwrap_or_default();
    let copy_res = copier.join().map_err(|_| anyhow::anyhow!("copier crashed"))?;
    done.store(true, Relaxed);
    let _ = logger.join();
    let (stats, files) = res?;
    let truncated = limit < total;
    if !truncated {
        copy_res.context("copying the dump into lbzip2")?;
        if !status.success() {
            bail!("lbzip2 exited with error {status}: {lb_err}");
        }
    }
    let wall = t0.elapsed().as_secs_f64();
    let s = RunSummary {
        json_bytes: pg.json_bytes.load(Relaxed),
        bz_read: pg.bz_bytes.load(Relaxed),
        bz_total: total,
        stats,
        files,
        wall,
        lbzip2_status: status.to_string(),
        lbzip2_stderr: lb_err.trim().to_string(),
    };
    log_line(&log, &format!("{} end: {} in {}; entities {}, items {}, properties {}, unparsed {}; lbzip2 {}", utc_now(), if truncated { "pilot" } else { "full pass" }, hms(wall), s.stats.lines, s.stats.items, s.stats.props, s.stats.bad, s.lbzip2_status));
    Ok(s)
}

// ── sample from the middle of the dump: bzip2 blocks from an arbitrary bit offset ───────────────────────────

/// bzip2 block magic (48 bits, π): blocks inside a stream are bit-aligned, not byte-aligned.
const BZ_BLOCK_MAGIC: u64 = 0x3141_5926_5359;

/// The first bit offset of the block magic in `buf`.
pub fn find_block_bit(buf: &[u8]) -> Option<usize> {
    let mut acc: u64 = 0;
    for (i, &byte) in buf.iter().enumerate() {
        for k in 0..8 {
            acc = ((acc << 1) | ((byte >> (7 - k)) & 1) as u64) & 0xFFFF_FFFF_FFFF;
            let bit = i * 8 + k + 1;
            if bit >= 48 && acc == BZ_BLOCK_MAGIC {
                return Some(bit - 48);
            }
        }
    }
    None
}

/// A new bzip2 stream from a block at a bit offset: header `BZh9` + bits from the magic, shifted to a byte boundary.
pub fn restream(buf: &[u8], bit: usize) -> Vec<u8> {
    let (byte, sh) = (bit >> 3, (bit & 7) as u32);
    let mut out = Vec::with_capacity(buf.len() - byte + 4);
    out.extend_from_slice(b"BZh9");
    if sh == 0 {
        out.extend_from_slice(&buf[byte..]);
    } else {
        for i in byte..buf.len() - 1 {
            out.push((buf[i] << sh) | (buf[i + 1] >> (8 - sh)));
        }
    }
    out
}

/// Decompress one chunk from the middle of the dump: (JSON from the first complete line, compressed bytes used).
pub fn sample_chunk(dump: &Path, offset: u64, len: usize) -> Result<(Vec<u8>, u64)> {
    use std::io::Seek;
    let mut f = File::open(dump)?;
    f.seek(std::io::SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; len];
    let mut got = 0;
    while got < len {
        let n = f.read(&mut buf[got..])?;
        if n == 0 {
            break;
        }
        got += n;
    }
    buf.truncate(got);
    let bit = find_block_bit(&buf).context("no bzip2 block magic in the chunk")?;
    let st = restream(&buf, bit);
    let mut dec = bzip2::read::BzDecoder::new(&st[..]);
    let mut json = Vec::new();
    let mut tmp = vec![0u8; 1 << 20];
    loop {
        match dec.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => json.extend_from_slice(&tmp[..n]),
            Err(_) => break, // truncated last block — expected
        }
    }
    // the first and last lines are incomplete
    let a = memchr::memchr(b'\n', &json).map(|i| i + 1).unwrap_or(json.len());
    let b = memchr::memrchr(b'\n', &json).map(|i| i + 1).unwrap_or(a);
    let used = (got - bit / 8) as u64;
    Ok((json[a..b.max(a)].to_vec(), used))
}

/// Stratified sample: `k` chunks of `mb` MB, evenly across the file; `threads` in parallel.
pub fn sample(dump: &Path, k: usize, mb: usize, threads: usize, out: &Path, level: i32, cfg: &Cfg) -> Result<(Stats, Vec<(String, PathBuf, u64, u64, u64)>, u64, u64, f64)> {
    std::fs::create_dir_all(out)?;
    let total = std::fs::metadata(dump)?.len();
    let t0 = Instant::now();
    let next = Arc::new(AtomicU64::new(0));
    let used = Arc::new(AtomicU64::new(0));
    let jsonb = Arc::new(AtomicU64::new(0));
    let mut hs = Vec::new();
    for w in 0..threads {
        let (next, used, jsonb) = (Arc::clone(&next), Arc::clone(&used), Arc::clone(&jsonb));
        let (dump, out, cfg) = (dump.to_path_buf(), out.to_path_buf(), cfg.clone());
        hs.push(std::thread::spawn(move || -> Result<(Stats, Vec<(String, PathBuf, u64, u64, u64)>)> {
            let mut outs = Outs::create(&out, &format!("s{w}"), level)?;
            let mut st = Stats::new();
            loop {
                let i = next.fetch_add(1, Relaxed) as usize;
                if i >= k {
                    break;
                }
                let off = ((i as f64 + 0.5) / k as f64 * total as f64) as u64;
                let off = off.min(total.saturating_sub((mb << 20) as u64));
                let (json, u) = sample_chunk(&dump, off, mb << 20)?;
                used.fetch_add(u, Relaxed);
                jsonb.fetch_add(json.len() as u64, Relaxed);
                let mut start = 0;
                for nl in memchr::memchr_iter(b'\n', &json) {
                    process_line(&json[start..nl], &mut outs, &mut st, &cfg)?;
                    start = nl + 1;
                }
            }
            Ok((st, outs.finish()?))
        }));
    }
    let mut st = Stats::new();
    let mut files = Vec::new();
    for h in hs {
        let (s, f) = h.join().map_err(|_| anyhow::anyhow!("sample thread crashed"))??;
        st.merge(&s);
        files.extend(f);
    }
    Ok((st, files, used.load(Relaxed), jsonb.load(Relaxed), t0.elapsed().as_secs_f64()))
}

// ── report ─────────────────────────────────────────────────────────────────────────────────────────

fn gb(b: u64) -> String {
    format!("{:.2}", b as f64 / 1e9)
}

pub(crate) fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push('\u{202f}');
        }
        out.push(c);
    }
    out
}

/// Report on a pass or a sample. `scale` — the factor for estimating the full dump (1 — full pass).
pub fn report(title: &str, st: &Stats, files: &[(String, PathBuf, u64, u64, u64)], scale: f64, extra: &str) -> String {
    use std::fmt::Write as _;
    let mut s = format!("# {title}\n\n{extra}\n");
    let _ = writeln!(s, "## Entities\n");
    let _ = writeln!(
        s,
        "- entity lines {}, unparsed {}; items {}, properties {}, other {}; `[` {} and `]` {}; largest Q{}",
        thousands(st.lines),
        st.bad,
        thousands(st.items),
        thousands(st.props),
        st.other,
        if st.saw_open { "yes" } else { "no" },
        if st.saw_close { "yes" } else { "no" },
        st.max_q
    );
    let _ = writeln!(s, "- with P31 {}, with P279 (class as subject) {}, taxa {}, wiki maintenance pages {}, scholarly articles {}", thousands(st.p31), thousands(st.p279), thousands(st.taxa), thousands(st.wm), thousands(st.scholarly));
    let _ = writeln!(s, "- label en {}, mul {}, uk {}; description en {}, uk {}", thousands(st.label_en), thousands(st.label_mul), thousands(st.label_uk), thousands(st.desc_en), thousands(st.desc_uk));
    let _ = writeln!(s, "- with sitelinks {} (all sitelinks {}), with Wikipedia {}; Wikipedia sitelink rows {} ({} sites)", thousands(st.with_sitelinks), thousands(st.sitelinks_all), thousands(st.with_wiki), thousands(st.sitelink_rows), st.sites.len());
    let _ = writeln!(s, "- labels in all languages for {} items: labels {}, aliases {}, languages {}", thousands(st.exported), thousands(st.label_rows), thousands(st.alias_rows), st.langs.len());
    let _ = writeln!(s, "- statements without a value (somevalue/novalue) in the taken properties: {}\n", thousands(st.novalue));
    let _ = writeln!(s, "## By category (for budgeting)\n\n| category | items | `items` text bytes | labels+aliases | their bytes | sitelinks |\n|---|---:|---:|---:|---:|---:|");
    for k in 0..3 {
        let _ = writeln!(s, "| {} | {} | {} | {} | {} | {} |", CATS[k], thousands(st.cat_items[k]), thousands(st.cat_item_bytes[k]), thousands(st.cat_label_rows[k]), thousands(st.cat_label_bytes[k]), thousands(st.cat_sitelinks[k]));
    }
    let _ = writeln!(s, "\n## Statements by property\n\n| P | name | type | rows | deprecated | preferred | why |\n|---|---|---|---:|---:|---:|---|");
    for (i, p) in PROPS.iter().enumerate() {
        let r = st.per_prop[i];
        let _ = writeln!(s, "| P{} | {} | {:?} | {} | {} | {} | {} |", p.p, p.name, p.kind, thousands(r[0] + r[1] + r[2]), thousands(r[0]), thousands(r[2]), p.why);
    }
    let _ = writeln!(s, "\n## Files\n\n| table | rows | TSV, GB | zstd, GB |{}\n|---|---:|---:|---:|{}", if scale != 1.0 { " zstd estimate for the whole dump, GB |" } else { "" }, if scale != 1.0 { "---:|" } else { "" });
    let mut by: Vec<(String, u64, u64, u64)> = Vec::new();
    for (t, _, r, raw, z) in files {
        match by.iter_mut().find(|x| &x.0 == t) {
            Some(x) => {
                x.1 += r;
                x.2 += raw;
                x.3 += z;
            }
            None => by.push((t.clone(), *r, *raw, *z)),
        }
    }
    let (mut tr, mut tz) = (0u64, 0u64);
    for (t, r, raw, z) in &by {
        tr += raw;
        tz += z;
        let est = if scale != 1.0 { format!(" {:.2} |", *z as f64 * scale / 1e9) } else { String::new() };
        let _ = writeln!(s, "| {t} | {} | {} | {} |{est}", thousands(*r), gb(*raw), gb(*z));
    }
    let est = if scale != 1.0 { format!(" **{:.2}** |", tz as f64 * scale / 1e9) } else { String::new() };
    let _ = writeln!(s, "| **total** | | {} | {} |{est}", gb(tr), gb(tz));
    let mut langs: Vec<(&String, &[u64; 2])> = st.langs.iter().collect();
    langs.sort_by(|a, b| (b.1[0] + b.1[1]).cmp(&(a.1[0] + a.1[1])));
    let _ = writeln!(s, "\n## Label languages (first 40 of {})\n\n{}", langs.len(), langs.iter().take(40).map(|(l, c)| format!("{l} {}+{}", c[0], c[1])).collect::<Vec<_>>().join(" · "));
    let mut sites: Vec<(&String, &u64)> = st.sites.iter().collect();
    sites.sort_by(|a, b| b.1.cmp(a.1));
    let _ = writeln!(s, "\n## Wikipedia sites (first 30 of {})\n\n{}", sites.len(), sites.iter().take(30).map(|(l, c)| format!("{l} {c}")).collect::<Vec<_>>().join(" · "));
    if !st.bad_examples.is_empty() {
        let _ = writeln!(s, "\n## Unparsed lines (examples)\n");
        for b in &st.bad_examples {
            let _ = writeln!(s, "- {b}");
        }
    }
    s
}

/// Language and site counters — as separate TSVs (for decisions about languages and cross-checks).
pub fn write_counts(st: &Stats, dir: &Path, tag: &str) -> Result<()> {
    let mut langs: Vec<(&String, &[u64; 2])> = st.langs.iter().collect();
    langs.sort_by(|a, b| (b.1[0] + b.1[1]).cmp(&(a.1[0] + a.1[1])).then(a.0.cmp(b.0)));
    let mut s = String::from("# lang\tlabels\taliases\n");
    for (l, c) in langs {
        s.push_str(&format!("{l}\t{}\t{}\n", c[0], c[1]));
    }
    std::fs::write(dir.join(format!("{tag}-langs.tsv")), s)?;
    let mut sites: Vec<(&String, &u64)> = st.sites.iter().collect();
    sites.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let mut s = String::from("# site\tsitelinks\n");
    for (l, c) in sites {
        s.push_str(&format!("{l}\t{c}\n"));
    }
    std::fs::write(dir.join(format!("{tag}-sites.tsv")), s)?;
    Ok(())
}

#[cfg(test)]
mod t {
    use super::*;

    fn run_lines(js: &str, cfg: &Cfg) -> (Stats, Vec<(String, String)>) {
        let dir = std::env::temp_dir().join(format!("onto-wdx-{}-{}", std::process::id(), js.len()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut o = Outs::create(&dir, "t", 3).unwrap();
        let mut st = Stats::new();
        for l in js.lines() {
            process_line(l.as_bytes(), &mut o, &mut st, cfg).unwrap();
        }
        let files = o.finish().unwrap();
        let mut out = Vec::new();
        for (t, p, _, _, _) in files {
            let b = zstd::stream::decode_all(File::open(&p).unwrap()).unwrap();
            out.push((t, String::from_utf8(b).unwrap()));
        }
        let _ = std::fs::remove_dir_all(&dir);
        (st, out)
    }

    fn table<'a>(v: &'a [(String, String)], t: &str) -> &'a str {
        &v.iter().find(|x| x.0 == t).unwrap().1
    }

    /// An item with labels, descriptions, aliases, sitelinks and statements of different types; `[]` as empty
    /// maps; deprecated is kept with rank 0 and does not make the item a class; property.
    #[test]
    fn entity_rows() {
        let js = r#"[
{"type":"item","id":"Q89","labels":{"en":{"language":"en","value":"apple"},"uk":{"language":"uk","value":"yabluko"},"de":{"language":"de","value":"Apfel"}},"descriptions":{"en":{"language":"en","value":"fruit of the apple tree"},"fr":{"language":"fr","value":"fruit"}},"aliases":{"en":[{"language":"en","value":"apples"}]},"claims":{"P279":[{"mainsnak":{"snaktype":"value","property":"P279","datavalue":{"value":{"entity-type":"item","numeric-id":3314483,"id":"Q3314483"},"type":"wikibase-entityid"}},"rank":"normal"}],"P2067":[{"mainsnak":{"datavalue":{"value":{"amount":"+0.2","unit":"http://www.wikidata.org/entity/Q11570","upperBound":"+0.25"},"type":"quantity"}},"qualifiers":{"P21":[{"datavalue":{"value":{"id":"Q44148"}}}]},"rank":"preferred"}],"P18":[{"mainsnak":{"datavalue":{"value":"Apple.jpg","type":"string"}}}],"P1843":[{"mainsnak":{"datavalue":{"value":{"text":"Apfel","language":"de"},"type":"monolingualtext"}},"rank":"normal"}],"P31":[{"mainsnak":{"snaktype":"somevalue"},"rank":"normal"}]},"sitelinks":{"enwiki":{"site":"enwiki","title":"Apple","badges":[]},"commonswiki":{"site":"commonswiki","title":"Apples"},"enwikiquote":{"site":"enwikiquote","title":"Apples"}}},
{"type":"item","id":"Q2","labels":[],"descriptions":[],"aliases":[],"claims":{"P279":[{"mainsnak":{"datavalue":{"value":{"id":"Q3"}}},"rank":"deprecated"}]},"sitelinks":[]},
{"type":"property","id":"P279","datatype":"wikibase-item","labels":{"en":{"language":"en","value":"subclass of"}},"descriptions":{"en":{"language":"en","value":"next higher class"}},"claims":[]},
{"type":"item","id":"Q5", broken
]"#;
        let (st, out) = run_lines(js, &Cfg::default());
        assert!(st.saw_open && st.saw_close);
        assert_eq!((st.items, st.props, st.bad), (2, 1, 1));
        let items = table(&out, "items");
        assert_eq!(items.lines().next().unwrap(), "89\tapple\t\tyabluko\tfruit of the apple tree\t\t3\t1\t3\t9");
        // Q2: deprecated P279 — not a class; no sitelinks and not a taxon — labels are not exported
        assert_eq!(items.lines().nth(1).unwrap(), "2\t\t\t\t\t\t0\t0\t0\t0");
        let labels = table(&out, "labels");
        assert_eq!(labels, "89\ten\tapple\t0\n89\tuk\tyabluko\t0\n89\tde\tApfel\t0\n89\ten\tapples\t1\n");
        assert_eq!(table(&out, "sitelinks"), "89\tenwiki\tApple\n");
        assert_eq!(table(&out, "claims"), "89\t279\t3314483\t1\n2\t279\t3\t0\n");
        assert_eq!(table(&out, "mono"), "89\t1843\tde\tApfel\t1\n");
        assert_eq!(table(&out, "quant"), "89\t2067\t0.2\t11570\t\t0.25\t2\tP21=Q44148\n");
        assert_eq!(table(&out, "strs"), "");
        assert_eq!(table(&out, "props"), "279\twikibase-item\tsubclass of\t\tnext higher class\n");
        assert_eq!(st.novalue, 1);
        assert_eq!(st.sitelinks_all, 3);
    }

    /// Language restriction and skipping wiki maintenance pages.
    #[test]
    fn filters() {
        let js = r#"{"type":"item","id":"Q10","labels":{"en":{"language":"en","value":"Category:Apples"},"xx":{"language":"xx","value":"X"}},"claims":{"P31":[{"mainsnak":{"datavalue":{"value":{"id":"Q4167836"}}},"rank":"normal"}]},"sitelinks":{"enwiki":{"title":"Category:Apples"}}}
{"type":"item","id":"Q11","labels":{"en":{"language":"en","value":"lion"},"xx":{"language":"xx","value":"X"}},"claims":{"P225":[{"mainsnak":{"datavalue":{"value":"Panthera leo","type":"string"}},"rank":"normal"}]}}"#;
        let cfg = Cfg { skip_wm: true, langs: Some(Arc::new(["en".to_string()].into_iter().collect())) };
        let (st, out) = run_lines(js, &cfg);
        assert_eq!((st.wm, st.taxa, st.exported), (1, 1, 1));
        assert_eq!(table(&out, "labels"), "11\ten\tlion\t0\n");
        assert_eq!(table(&out, "sitelinks"), "");
        assert_eq!(table(&out, "strs"), "11\t225\tPanthera leo\t1\n");
    }

    /// A stream with tiny chunks: line tails carry over to the next chunk, a line longer than a chunk
    /// is not lost and does not loop the reader; a final line without '\n' is parsed too.
    #[test]
    fn stream_tails() {
        let long = "x".repeat(5000);
        let mut js = String::from("[\n");
        for i in 1..=50u32 {
            let lab = if i == 7 { long.clone() } else { format!("item {i}") };
            js.push_str(&format!("{{\"type\":\"item\",\"id\":\"Q{i}\",\"labels\":{{\"en\":{{\"language\":\"en\",\"value\":\"{lab}\"}}}},\"claims\":{{\"P279\":[{{\"mainsnak\":{{\"datavalue\":{{\"value\":{{\"id\":\"Q{}\"}}}}}},\"rank\":\"normal\"}}]}}}},\n", i + 1));
        }
        js.push(']');
        let dir = std::env::temp_dir().join(format!("onto-wdx-stream-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let pg = Arc::new(Progress::default());
        let (st, files) = run_stream_chunked(js.as_bytes(), &dir, "t", 3, 3, &Cfg::default(), &pg, 300).unwrap();
        assert_eq!((st.items, st.bad, st.p279, st.label_rows), (50, 0, 50, 50));
        assert!(st.saw_open && st.saw_close);
        let claims: usize = files.iter().filter(|f| f.0 == "claims").map(|f| f.2 as usize).sum();
        assert_eq!(claims, 50);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn wikipedia_sites() {
        for s in ["enwiki", "ukwiki", "be_x_oldwiki", "simplewiki", "zh_min_nanwiki"] {
            assert!(is_wikipedia(s), "{s}");
        }
        for s in ["commonswiki", "specieswiki", "enwikisource", "enwikiquote", "wikidatawiki", "metawiki", "wiki", "abstractwiki"] {
            assert!(!is_wikipedia(s), "{s}");
        }
    }

    /// A bzip2 block from an arbitrary bit offset: find the magic and decompress as a new stream.
    #[test]
    fn bit_aligned_block() {
        let mut text = Vec::new();
        for i in 0..400_000u32 {
            text.extend_from_slice(format!("{{\"id\":\"Q{i}\"}},\n").as_bytes());
        }
        let mut enc = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::best());
        enc.write_all(&text).unwrap();
        let bz = enc.finish().unwrap();
        // from the very start: the magic right after "BZh9"
        assert_eq!(find_block_bit(&bz), Some(32));
        // from the middle: the next block does not start on a byte boundary
        let mid = bz.len() / 2;
        let bit = find_block_bit(&bz[mid..]).expect("second block");
        let s = restream(&bz[mid..], bit);
        let mut dec = bzip2::read::BzDecoder::new(&s[..]);
        let mut out = Vec::new();
        let mut tmp = vec![0u8; 1 << 16];
        while let Ok(n) = dec.read(&mut tmp) {
            if n == 0 {
                break;
            }
            out.extend_from_slice(&tmp[..n]);
        }
        assert!(out.len() > 100_000);
        // the decompressed data is a contiguous piece of the original (the end of a stream with a foreign combined CRC may not arrive — it
        // is not in the sample anyway: the chunk breaks off earlier)
        let at = memchr::memmem::find(&text, &out[..4096]).expect("start found in the original");
        assert_eq!(&text[at..at + out.len()], &out[..]);
    }

    /// TSV columns match what the loading SQL reads (a gate for code/SQL divergence).
    #[test]
    fn sql_columns_match() {
        let sql = include_str!("../sql/wikidata-load.sql");
        for (t, cols) in COLUMNS.iter().chain(crate::wdlex::LEX_COLUMNS) {
            let file = if crate::wdlex::LEX_COLUMNS.iter().any(|x| x.0 == *t) { format!("'lex-{t}.tsv.zst'") } else { format!("'{t}-w*.tsv.zst'") };
            let at = sql.find(&file).unwrap_or_else(|| panic!("SQL does not read {file}"));
            let rest = &sql[at..];
            let block = &rest[..rest.find(")").unwrap_or(rest.len())];
            let names: Vec<&str> = block.split('\'').enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, x)| x).filter(|x| !x.contains(".tsv") && !x.is_empty() && *x != "\\t").collect();
            let want: Vec<&str> = cols.split(", ").collect();
            let got: Vec<&str> = names.iter().copied().filter(|x| want.contains(x) || !["INTEGER", "VARCHAR", "SMALLINT", "UTINYINT", "BOOLEAN", "DOUBLE"].contains(x)).filter(|x| !["INTEGER", "VARCHAR", "SMALLINT", "UTINYINT", "BOOLEAN", "DOUBLE"].contains(x)).collect();
            assert_eq!(got, want, "columns {t}");
        }
    }

    #[test]
    fn ids() {
        assert_eq!(num_id("Q89", b'Q'), Some(89));
        assert_eq!(num_id("P279", b'P'), Some(279));
        assert_eq!(num_id("Q", b'Q'), None);
        assert_eq!(num_id("L1-S1", b'L'), None);
        assert_eq!(unit_q("http://www.wikidata.org/entity/Q11570"), 11570);
        assert_eq!(unit_q("1"), 0);
        assert_eq!(PIDX[279] as usize, PROPS.iter().position(|p| p.p == 279).unwrap());
        assert_eq!(PIDX[18], u8::MAX);
    }
}
