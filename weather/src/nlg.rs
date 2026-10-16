//! "Data → text" forecast generator: `Period` → text in the style of the variant (US / international).
//!
//! Training is on "period data → text" pairs (NWS, ECCC):
//! 1. The text is split into sentences. Each sentence is aligned with the data: numbers become
//!    slots by context (temperature, probability, speeds, gusts, UV), wind direction becomes a slot, the remaining
//!    words become form numbers from the in-code dictionary. This turns the sentence into a **template**.
//! 2. The template gets a **component**: sky, phenomenon, sky+temperature, temperature, wind,
//!    probability, UV, humidex.
//! 3. Tables per variant (keys are data features, with backoff from exact to coarse):
//!    - whether the component is present;
//!    - which template;
//!    - component order — the mean position in the texts.
//!
//! **Honesty.** Sentences with a number or direction not in the data, and with time («after 1pm»,
//! «developing overnight») do not become templates: the generator does not invent facts. Every generated
//! sentence is checked by the same alignment (gate), and the negative control is alignment with
//! other data.

use std::collections::BTreeMap;

use en::dict::{self, Form};
use en::gram::Case;
use en::hash::FastMap;
use en::text::{chrf, title};
use en::tok::{self, Lexicon};

use crate::data::{Dir, Period, SpeedUnit, Variant, Wx, sentences};

/// Component of a forecast text.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Comp {
    Wx,
    Sky,
    SkyTemp,
    Temp,
    Wind,
    Pop,
    Uv,
    Humidex,
}

const COMPS: [Comp; 8] = [Comp::Wx, Comp::Sky, Comp::SkyTemp, Comp::Temp, Comp::Wind, Comp::Pop, Comp::Uv, Comp::Humidex];

/// Template slot — a place for a value from the data.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Slot {
    Temp,
    Pop,
    Lo,
    Hi,
    Gust,
    Dir,
    Uv,
    UvCat,
    Humidex,
}

/// Template piece: a form from the in-code dictionary (number + case), a string outside the dictionary, a slot.
/// `space` — whether a space follows it.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Piece {
    Word { form: Form, case: Case, space: bool },
    Lit { text: Box<str>, space: bool },
    Slot { slot: Slot, case: Case, space: bool },
}

pub type Template = Vec<Piece>;

/// Table key: component, backoff level, data features.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Key {
    comp: Comp,
    level: u8,
    f: [u8; 6],
}

/// Feature words (form numbers in the in-code dictionary): sky, phenomena, wind, time, directions.
struct Words {
    sky: Vec<Form>,
    wx: Vec<Form>,
    wind: Vec<Form>,
    time: Vec<Form>,
    dir: Vec<Form>,
    precipitation: Option<Form>,
    /// which phenomenon words are allowed for the phenomenon in the data
    allow: FastMap<Wx, Vec<Form>>,
}

fn forms(ws: &[&str]) -> Vec<Form> {
    ws.iter().filter_map(|w| dict::form(w)).collect()
}

impl Words {
    fn new() -> Words {
        Words {
            sky: forms(&["sunny", "cloudy", "clear", "cloud", "clouds", "sun", "overcast", "fair", "cloudiness", "clearing", "partly", "mostly", "mainly"]),
            wx: forms(&[
                "rain", "showers", "shower", "thunderstorms", "thunderstorm", "thundershowers", "snow", "drizzle", "fog", "sleet", "flurries", "hail", "freezing",
                "smoke", "haze", "dust", "precipitation", "storms", "blizzard", "mist",
            ]),
            wind: forms(&["wind", "winds", "windy", "breezy", "gusts", "gusting", "calm"]),
            time: forms(&[
                "morning", "afternoon", "evening", "overnight", "midnight", "noon", "am", "pm", "late", "early", "beginning", "ending", "becoming", "developing",
                "dissipating", "then", "after", "before", "until", "tonight", "today", "tomorrow", "daybreak", "sunrise", "sunset",
            ]),
            dir: forms(&["north", "south", "east", "west", "northeast", "northwest", "southeast", "southwest"]),
            precipitation: dict::form("precipitation"),
            allow: [
                (Wx::Showers, &["showers", "shower", "rain"][..]),
                (Wx::Rain, &["rain", "showers", "shower", "drizzle"]),
                (Wx::Drizzle, &["drizzle", "rain"]),
                (Wx::Thunder, &["thunderstorms", "thunderstorm", "thundershowers", "storms", "showers", "shower", "rain"]),
                (Wx::FreezingRain, &["freezing", "rain", "drizzle"]),
                (Wx::RainSnow, &["rain", "snow", "showers", "shower", "flurries"]),
                (Wx::Flurries, &["flurries", "snow"]),
                (Wx::Snow, &["snow", "flurries"]),
                (Wx::Blizzard, &["blizzard", "snow"]),
                (Wx::Sleet, &["sleet", "snow", "rain", "freezing"]),
                (Wx::Fog, &["fog", "mist"]),
                (Wx::Haze, &["haze"]),
                (Wx::Smoke, &["smoke", "haze"]),
                (Wx::Dust, &["dust"]),
            ]
            .into_iter()
            .map(|(wx, ws)| (wx, forms(ws)))
            .collect(),
        }
    }
    fn has(set: &[Form], f: Option<Form>) -> bool {
        f.is_some_and(|f| set.contains(&f))
    }
}

/// Direction name (lower case): 16 compass points.
fn dir_phrase(d: Dir) -> Option<&'static str> {
    Some(match d {
        Dir::N => "north",
        Dir::NNE => "north northeast",
        Dir::NE => "northeast",
        Dir::ENE => "east northeast",
        Dir::E => "east",
        Dir::ESE => "east southeast",
        Dir::SE => "southeast",
        Dir::SSE => "south southeast",
        Dir::S => "south",
        Dir::SSW => "south southwest",
        Dir::SW => "southwest",
        Dir::WSW => "west southwest",
        Dir::W => "west",
        Dir::WNW => "west northwest",
        Dir::NW => "northwest",
        Dir::NNW => "north northwest",
        Dir::Var => return None,
    })
}

/// UV index category (WHO scale).
fn uv_cat(i: u8) -> &'static str {
    match i {
        0..=2 => "low",
        3..=5 => "moderate",
        6..=7 => "high",
        8..=10 => "very high",
        _ => "extreme",
    }
}

fn case_of(tok: &str, lower: &str) -> Option<Case> {
    if tok == lower {
        Some(Case::AsIs)
    } else if tok == title(lower) {
        Some(Case::Title)
    } else if tok == lower.to_uppercase() {
        Some(Case::Upper)
    } else {
        None
    }
}

/// Data features for keys.
struct Feat {
    lead: u8,
    sky: u8,
    wx: u8,
    popb: u8,
    day: u8,
    windy: u8,
    tclass: u8,
    dirp: u8,
    same: u8,
    gust: u8,
    spb: u8,
    uvc: u8,
}

fn pop_bucket(p: Option<u8>) -> u8 {
    match p {
        None => 0,
        Some(0..=19) => 1,
        Some(20..=29) => 2,
        Some(30..=50) => 3,
        Some(51..=79) => 4,
        Some(_) => 5,
    }
}

fn feat(p: &Period) -> Feat {
    let t = p.temp.map(|t| t.value);
    let w = p.wind;
    Feat {
        lead: match p.lead {
            0..=2 => 1,
            3..=5 => 2,
            _ => 3,
        },
        sky: p.cond.sky as u8,
        wx: p.cond.wx as u8,
        popb: pop_bucket(p.pop),
        day: p.day as u8,
        windy: p.cond.windy as u8,
        tclass: match t {
            None => 0,
            Some(v) if v < 0 => 1,
            Some(0) => 2,
            Some(1..=5) => 3,
            Some(_) => 4,
        },
        dirp: w.is_some_and(|w| w.dir.and_then(dir_phrase).is_some()) as u8,
        same: w.is_some_and(|w| w.lo == w.hi) as u8,
        gust: w.is_some_and(|w| w.gust.is_some()) as u8,
        spb: w.map_or(0, |w| match w.hi {
            0 => 1,
            1..=9 => 2,
            10..=19 => 3,
            20..=29 => 4,
            30..=39 => 5,
            _ => 6,
        }),
        uvc: p.uv.map_or(0, |u| match uv_cat(u) {
            "low" => 1,
            "moderate" => 2,
            "high" => 3,
            "very high" => 4,
            _ => 5,
        }),
    }
}

/// Component keys from exact to coarse.
fn keys(comp: Comp, f: &Feat) -> Vec<Key> {
    const X: u8 = 0xFF;
    let lv: Vec<[u8; 6]> = match comp {
        // sky and phenomenon never back off beyond the sky or phenomenon itself: better to stay silent than to name the wrong one
        Comp::Sky => vec![
            [f.sky, f.wx, f.popb, f.day, f.windy, f.lead],
            [f.sky, f.wx, f.popb, f.day, f.windy, X],
            [f.sky, f.wx, f.popb, f.day, X, X],
            [f.sky, f.wx, X, f.day, X, X],
            [f.sky, f.wx, X, X, X, X],
            [f.sky, X, X, X, X, X],
        ],
        Comp::Wx => vec![[f.wx, f.popb, f.day, f.lead, X, X], [f.wx, f.popb, f.day, X, X, X], [f.wx, f.popb, X, X, X, X], [f.wx, X, X, X, X, X]],
        Comp::SkyTemp => vec![[f.sky, f.wx, f.popb, f.day, f.tclass, X], [f.sky, f.wx, X, f.day, f.tclass, X], [f.sky, X, X, f.day, f.tclass, X], [f.sky, X, X, f.day, X, X]],
        Comp::Temp => vec![[f.day, f.tclass, X, X, X, X], [f.day, X, X, X, X, X], [X; 6]],
        Comp::Wind => vec![[f.lead, f.dirp, f.same, f.gust, f.spb, X], [f.lead, f.dirp, f.same, f.gust, X, X], [f.lead, f.gust, X, X, X, X], [f.gust, X, X, X, X, X], [X; 6]],
        Comp::Pop => vec![[f.lead, f.popb, X, X, X, X], [f.popb, X, X, X, X, X], [X; 6]],
        Comp::Uv => vec![[f.uvc, X, X, X, X, X], [X; 6]],
        Comp::Humidex => vec![[X; 6]],
    };
    lv.into_iter().enumerate().map(|(i, f)| Key { comp, level: i as u8, f }).collect()
}

/// Result of aligning a sentence with the data.
pub struct Delex {
    pub comp: Option<Comp>,
    /// template — if everything that looks like data fits the data
    pub template: Option<Template>,
    /// why it is not a template (for the report)
    pub why: Option<&'static str>,
}

/// Generator model for one variant.
pub struct Nlg {
    pub variant: Variant,
    lx: Lexicon,
    words: Words,
    /// key → (component present, absent)
    pres: FastMap<Key, (u32, u32)>,
    /// key → template → count
    tmpl: FastMap<Key, Vec<(Template, u32)>>,
    /// component order
    order: Vec<Comp>,
    /// only the coarsest keys (baseline without conditions on the data)
    pub baseline: bool,
}

const MIN_PRES: u32 = 3;

impl Nlg {
    /// Aligning a sentence with the period data.
    pub fn delex(&self, sentence: &str, p: &Period) -> Delex {
        let toks = tok::tokenize(sentence, &self.lx);
        let low: Vec<String> = toks.iter().map(|t| t.form.to_lowercase()).collect();
        let fs: Vec<Option<Form>> = low.iter().map(|l| dict::form(l)).collect();
        let w = &self.words;
        let has = |set: &[Form]| fs.iter().any(|&f| Words::has(set, f));
        // component from words and numbers, even if there will be no template
        let unit = low.iter().any(|l| l == "mph" || l == "km/h");
        let pct = low.iter().any(|l| l == "%" || l == "percent");
        let is_uv = low.windows(2).any(|x| x[0] == "uv" && x[1] == "index");
        let is_hx = low.iter().any(|l| l == "humidex");
        let temp_word = low.iter().any(|l| matches!(l.as_str(), "high" | "low" | "highs" | "lows" | "temperature" | "temperatures"));
        let digits = low.iter().any(|l| l.chars().all(|c| c.is_ascii_digit()));
        let comp = if is_uv {
            Some(Comp::Uv)
        } else if is_hx {
            Some(Comp::Humidex)
        } else if pct && fs.iter().any(|&f| f.is_some() && f == w.precipitation) && !has(&w.sky) {
            Some(Comp::Pop)
        } else if unit || has(&w.wind) {
            Some(Comp::Wind)
        } else if temp_word && (digits || low.iter().any(|l| l == "zero")) {
            if has(&w.sky) || has(&w.wx) { Some(Comp::SkyTemp) } else { Some(Comp::Temp) }
        } else if has(&w.wx) {
            Some(Comp::Wx)
        } else if has(&w.sky) {
            Some(Comp::Sky)
        } else {
            None
        };
        let fail = |why| Delex { comp, template: None, why: Some(why) };
        if comp.is_none() {
            return fail("component not determined");
        }
        if has(&w.time) {
            return fail("time not in the data");
        }
        // phenomenon words: only those allowed for the phenomenon in the data («precipitation» always: it is about probability)
        let allowed = w.allow.get(&p.cond.wx);
        for &f in fs.iter().flatten() {
            if w.wx.contains(&f) && Some(f) != w.precipitation && !allowed.is_some_and(|a| a.contains(&f)) {
                return fail("phenomenon not in the data");
            }
        }
        // wind direction: only the one in the data
        let mut slot_at: Vec<Option<(Slot, Case)>> = vec![None; toks.len()];
        let mut skip = vec![false; toks.len()];
        if let Some(ph) = p.wind.and_then(|x| x.dir).and_then(dir_phrase) {
            let parts: Vec<&str> = ph.split(' ').collect();
            for i in 0..toks.len() {
                if i + parts.len() <= toks.len() && (0..parts.len()).all(|k| low[i + k] == parts[k]) {
                    let case = if toks[i].form.starts_with(char::is_uppercase) { Case::Title } else { Case::AsIs };
                    slot_at[i] = Some((Slot::Dir, case));
                    for k in 1..parts.len() {
                        skip[i + k] = true;
                    }
                    break;
                }
            }
        }
        for i in 0..toks.len() {
            if slot_at[i].is_none() && !skip[i] && Words::has(&w.dir, fs[i]) {
                return fail("direction not in the data");
            }
        }
        // numbers
        let wind = p.wind;
        let (mut lo_used, mut hi_used) = (false, false);
        for i in 0..toks.len() {
            let Ok(v) = low[i].parse::<i32>() else { continue };
            let prev = if i > 0 { low[i - 1].as_str() } else { "" };
            let prev2 = if i > 1 { low[i - 2].as_str() } else { "" };
            let next = low.get(i + 1).map(String::as_str).unwrap_or("");
            let s = if unit && wind.is_some() {
                let wd = wind.expect("wind");
                let gust_ctx = (prev == "as" && prev2 == "high") || (prev == "to" && prev2 == "gusting") || prev == "gusts";
                if gust_ctx && wd.gust == Some(v as u16) {
                    Some(Slot::Gust)
                } else if !gust_ctx && !lo_used && v as u16 == wd.lo {
                    lo_used = true;
                    Some(Slot::Lo)
                } else if !gust_ctx && !hi_used && lo_used && v as u16 == wd.hi {
                    hi_used = true;
                    Some(Slot::Hi)
                } else {
                    None
                }
            } else if (next == "%" || next == "percent") && p.pop == Some(v as u8) {
                Some(Slot::Pop)
            } else if prev == "index" && p.uv == Some(v as u8) {
                Some(Slot::Uv)
            } else if prev == "humidex" && p.humidex == Some(v as i16) {
                Some(Slot::Humidex)
            } else if let Some(t) = p.temp {
                let ok = match prev {
                    "minus" => t.value == -v as i16,
                    "plus" => t.value == v as i16 && v > 0,
                    _ => t.value == v as i16,
                };
                (ok && matches!(prev, "near" | "around" | "high" | "low" | "plus" | "minus" | "highs" | "lows")).then_some(Slot::Temp)
            } else {
                None
            };
            match s {
                Some(s) => slot_at[i] = Some((s, Case::AsIs)),
                None => return fail("number not in the data"),
            }
        }
        // UV category
        if comp == Some(Comp::Uv) {
            let Some(cat) = p.uv.map(uv_cat) else { return fail("UV without data") };
            let parts: Vec<&str> = cat.split(' ').collect();
            let Some(or) = low.iter().position(|l| l == "or") else { return fail("UV without category") };
            let at = or + 1;
            if at + parts.len() <= toks.len() && (0..parts.len()).all(|k| low[at + k] == parts[k]) {
                slot_at[at] = Some((Slot::UvCat, Case::AsIs));
                for k in 1..parts.len() {
                    skip[at + k] = true;
                }
            } else {
                return fail("UV category does not match the data");
            }
        }
        // template
        let mut t: Template = Vec::new();
        for i in 0..toks.len() {
            if skip[i] {
                // the space after a skipped slot piece belongs to the slot
                if let Some(Piece::Slot { space, .. }) = t.last_mut() {
                    *space = toks[i].space_after;
                }
                continue;
            }
            let space = toks[i].space_after;
            let piece = match slot_at[i] {
                Some((slot, case)) => Piece::Slot { slot, case, space },
                None => match (fs[i], case_of(&toks[i].form, &low[i])) {
                    (Some(form), Some(case)) => Piece::Word { form, case, space },
                    _ => Piece::Lit { text: toks[i].form.clone().into(), space },
                },
            };
            t.push(piece);
        }
        Delex { comp, template: Some(t), why: None }
    }

    /// Training on the periods of one variant.
    pub fn train(variant: Variant, lx: Lexicon, periods: &[&Period]) -> Nlg {
        let mut m = Nlg { variant, lx, words: Words::new(), pres: FastMap::default(), tmpl: FastMap::default(), order: Vec::new(), baseline: false };
        let mut pos: BTreeMap<Comp, (f64, f64)> = BTreeMap::new();
        for p in periods {
            let ss = sentences(&p.text);
            let f = feat(p);
            let mut seen: Vec<Comp> = Vec::new();
            let mut tpl: Vec<(Comp, Template)> = Vec::new();
            for (k, s) in ss.iter().enumerate() {
                let d = m.delex(s, p);
                let Some(c) = d.comp else { continue };
                if !seen.contains(&c) {
                    seen.push(c);
                    let e = pos.entry(c).or_default();
                    e.0 += k as f64 / ss.len().max(1) as f64;
                    e.1 += 1.0;
                    if let Some(t) = d.template {
                        tpl.push((c, t));
                    }
                }
            }
            for c in COMPS {
                let present = seen.contains(&c);
                for key in keys(c, &f) {
                    let e = m.pres.entry(key).or_default();
                    if present { e.0 += 1 } else { e.1 += 1 }
                }
            }
            for (c, t) in tpl {
                for key in keys(c, &f) {
                    let v = m.tmpl.entry(key).or_default();
                    match v.iter_mut().find(|(x, _)| *x == t) {
                        Some(e) => e.1 += 1,
                        None => v.push((t.clone(), 1)),
                    }
                }
            }
        }
        let mut order: Vec<(Comp, f64)> = pos.into_iter().map(|(c, (s, n))| (c, s / n)).collect();
        order.sort_by(|a, b| a.1.total_cmp(&b.1));
        m.order = order.into_iter().map(|x| x.0).collect();
        m
    }

    fn keys_for(&self, c: Comp, f: &Feat) -> Vec<Key> {
        let ks = keys(c, f);
        if self.baseline { ks.into_iter().last().into_iter().collect() } else { ks }
    }

    /// Whether the component is present: the first key level with enough examples.
    fn present(&self, c: Comp, f: &Feat) -> bool {
        for k in self.keys_for(c, f) {
            if let Some(&(y, n)) = self.pres.get(&k) {
                if y + n >= MIN_PRES || k.f == [0xFF; 6] {
                    return y > n;
                }
            }
        }
        false
    }

    /// The template's phenomenon words are compatible with the phenomenon in the data.
    fn compatible(&self, t: &Template, p: &Period) -> bool {
        let allowed = self.words.allow.get(&p.cond.wx);
        t.iter().all(|x| match x {
            Piece::Word { form, .. } => !self.words.wx.contains(form) || Some(*form) == self.words.precipitation || allowed.is_some_and(|a| a.contains(form)),
            _ => true,
        })
    }

    /// Component template: the most frequent one at the first level that has a template fillable from the data.
    fn template(&self, c: Comp, f: &Feat, p: &Period) -> Option<&Template> {
        for k in self.keys_for(c, f) {
            let Some(v) = self.tmpl.get(&k) else { continue };
            let mut v: Vec<&(Template, u32)> = v.iter().collect();
            v.sort_by(|a, b| b.1.cmp(&a.1));
            if let Some((t, _)) = v.into_iter().find(|(t, _)| fillable(t, p) && self.compatible(t, p)) {
                return Some(t);
            }
        }
        None
    }

    /// Component presence statistics by key level (for debugging).
    pub fn pres_stats(&self, c: Comp, p: &Period) -> Vec<(u8, u32, u32)> {
        self.keys_for(c, &feat(p)).into_iter().filter_map(|k| self.pres.get(&k).map(|&(y, n)| (k.level, y, n))).collect()
    }

    /// Forecast text from data (data units are the variant's units).
    pub fn generate(&self, p: &Period) -> String {
        let f = feat(p);
        let skytemp = self.present(Comp::SkyTemp, &f) && p.temp.is_some() && self.template(Comp::SkyTemp, &f, p).is_some();
        // a phenomenon sentence that already describes the sky («Cloudy with 60 percent chance of showers») replaces the separate sky sentence
        let wx_has_sky = p.cond.wx != Wx::None
            && self.present(Comp::Wx, &f)
            && self.template(Comp::Wx, &f, p).is_some_and(|t| t.iter().any(|x| matches!(x, Piece::Word { form, .. } if self.words.sky.contains(form))));
        let mut out: Vec<String> = Vec::new();
        for &c in &self.order {
            let need = match c {
                Comp::SkyTemp => skytemp,
                Comp::Sky => !skytemp && !wx_has_sky,
                Comp::Temp => !skytemp && p.temp.is_some(),
                Comp::Wind => p.wind.is_some(),
                Comp::Pop => p.pop.is_some(),
                Comp::Uv => p.uv.is_some(),
                Comp::Humidex => p.humidex.is_some(),
                Comp::Wx => p.cond.wx != Wx::None,
            };
            // temperature is mandatory: if it is in the data, it will be in the text (as a separate sentence unless combined with sky)
            let forced = c == Comp::Temp && need;
            if !need || (c != Comp::SkyTemp && !forced && !self.present(c, &f)) {
                continue;
            }
            if let Some(t) = self.template(c, &f, p) {
                out.push(fill(t, p));
            }
        }
        out.join(" ")
    }
}

/// Whether the data suffices for all template slots.
fn fillable(t: &Template, p: &Period) -> bool {
    t.iter().all(|x| match x {
        Piece::Slot { slot, .. } => match slot {
            Slot::Temp => p.temp.is_some(),
            Slot::Pop => p.pop.is_some(),
            Slot::Lo => p.wind.is_some(),
            Slot::Hi => p.wind.is_some_and(|w| w.hi != w.lo),
            Slot::Gust => p.wind.is_some_and(|w| w.gust.is_some()),
            Slot::Dir => p.wind.and_then(|w| w.dir).and_then(dir_phrase).is_some(),
            Slot::Uv | Slot::UvCat => p.uv.is_some(),
            Slot::Humidex => p.humidex.is_some(),
        },
        _ => true,
    }) && {
        // "from" without "to" only when the data has a single speed (otherwise the text would lie)
        let has = |s: Slot| t.iter().any(|x| matches!(x, Piece::Slot { slot, .. } if *slot == s));
        !(has(Slot::Lo) && !has(Slot::Hi) && p.wind.is_some_and(|w| w.lo != w.hi))
    }
}

fn apply_case(s: &str, c: Case) -> String {
    match c {
        Case::Title => title(s),
        Case::Upper => s.to_uppercase(),
        _ => s.to_string(),
    }
}

/// Filling a template with data.
pub fn fill(t: &Template, p: &Period) -> String {
    let mut s = String::new();
    for x in t {
        let (text, space) = match x {
            Piece::Word { form, case, space } => (apply_case(dict::form_text(*form), *case), *space),
            Piece::Lit { text, space } => (text.to_string(), *space),
            Piece::Slot { slot, case, space } => {
                let v = match slot {
                    Slot::Temp => {
                        let t = p.temp.expect("temperature").value;
                        // in the international style the sign comes from template words («minus», «plus»)
                        if p.variant == Variant::Intl { t.abs().to_string() } else { t.to_string() }
                    }
                    Slot::Pop => p.pop.unwrap_or(0).to_string(),
                    Slot::Lo => p.wind.map_or(0, |w| w.lo).to_string(),
                    Slot::Hi => p.wind.map_or(0, |w| w.hi).to_string(),
                    Slot::Gust => p.wind.and_then(|w| w.gust).unwrap_or(0).to_string(),
                    Slot::Dir => p.wind.and_then(|w| w.dir).and_then(dir_phrase).map(|d| apply_case(d, *case)).unwrap_or_default(),
                    Slot::Uv => p.uv.unwrap_or(0).to_string(),
                    Slot::UvCat => uv_cat(p.uv.unwrap_or(0)).to_string(),
                    Slot::Humidex => p.humidex.unwrap_or(0).to_string(),
                };
                (v, *space)
            }
        };
        s.push_str(&text);
        if space {
            s.push(' ');
        }
    }
    s.trim_end().to_string()
}

/// Evaluation on test periods: exact, chrF, components, coverage, fact gate and negative control.
pub struct Report {
    pub n: usize,
    pub exact: usize,
    pub chrf: f64,
    pub comp_hit: BTreeMap<Comp, (usize, usize)>,
    pub ref_sent: usize,
    pub ref_ok: usize,
    pub gen_sent: usize,
    pub gen_bad: usize,
    pub neg_sent: usize,
    pub neg_bad: usize,
    /// why reference sentences did not become templates
    pub why: BTreeMap<&'static str, usize>,
}

pub fn evaluate(m: &Nlg, test: &[&Period], show: usize) -> Report {
    let mut r = Report { n: test.len(), exact: 0, chrf: 0.0, comp_hit: BTreeMap::new(), ref_sent: 0, ref_ok: 0, gen_sent: 0, gen_bad: 0, neg_sent: 0, neg_bad: 0, why: BTreeMap::new() };
    let mut shown = 0;
    for (i, p) in test.iter().enumerate() {
        let g = m.generate(p);
        if g == p.text {
            r.exact += 1;
        }
        r.chrf += chrf(&g, &p.text);
        // components: reference sentences that became templates vs generator sentences
        let gen_s = sentences(&g);
        let gen_by: Vec<(Option<Comp>, &String)> = gen_s.iter().map(|s| (m.delex(s, p).comp, s)).collect();
        for s in sentences(&p.text) {
            r.ref_sent += 1;
            let d = m.delex(&s, p);
            if let Some(w) = d.why {
                *r.why.entry(w).or_default() += 1;
            }
            if d.template.is_some() {
                r.ref_ok += 1;
                let e = r.comp_hit.entry(d.comp.expect("component")).or_default();
                e.1 += 1;
                if gen_by.iter().any(|(c, g)| *c == d.comp && **g == s) {
                    e.0 += 1;
                } else if let (Ok(want), false) = (std::env::var("NLG_COMP"), m.baseline) {
                    if format!("{:?}", d.comp.expect("component")) == want {
                        let ours = gen_by.iter().find(|(c, _)| *c == d.comp).map(|x| x.1.as_str()).unwrap_or("—");
                        println!("      [{want}] reference: {s}  |  ours: {ours}  |  wind={:?} lead={} cond={:?} pop={:?} {:?}", p.wind.map(|w| (w.dir, w.lo, w.hi)), p.lead, p.cond, p.pop, m.pres_stats(Comp::Wx, p));
                    }
                }
            }
        }
        // fact gate: every generated sentence must fit its own data
        for s in &gen_s {
            r.gen_sent += 1;
            if m.delex(s, p).template.is_none() {
                r.gen_bad += 1;
            }
        }
        // negative control: text generated for this period vs the neighbouring period's data
        let q = test[(i + 1) % test.len()];
        for s in &gen_s {
            r.neg_sent += 1;
            if m.delex(s, q).template.is_none() {
                r.neg_bad += 1;
            }
        }
        if shown < show && g != p.text {
            shown += 1;
            println!("    data : {} · t={:?} pop={:?} wind={:?} sky={:?}/{:?}", p.name, p.temp.map(|t| t.value), p.pop, p.wind.map(|w| (w.dir, w.lo, w.hi, w.gust)), p.cond.sky, p.cond.wx);
            println!("    ours : {g}\n    NWS/ECCC: {}", p.text);
            if std::env::var("NLG_DEBUG").is_ok() {
                for c in COMPS {
                    println!("      {c:?}: {:?}", m.pres_stats(c, p));
                }
            }
        }
    }
    r
}

/// Converting period data to the units of the other variant (speed rounded to a multiple of 5).
pub fn convert(p: &Period, to: Variant) -> Period {
    let mut q = p.clone();
    q.variant = to;
    q.temp = p.temp.map(|t| t.to(to.temp_unit()));
    q.wind = p.wind.map(|w| {
        let k = match (w.unit, to.speed_unit()) {
            (SpeedUnit::Mph, SpeedUnit::Kmh) => 1.609_344,
            (SpeedUnit::Kmh, SpeedUnit::Mph) => 1.0 / 1.609_344,
            _ => 1.0,
        };
        let r = |v: u16| ((v as f64 * k / 5.0).round() * 5.0) as u16;
        crate::data::Wind { dir: w.dir, lo: r(w.lo), hi: r(w.hi), gust: w.gust.map(r), unit: to.speed_unit() }
    });
    q
}
