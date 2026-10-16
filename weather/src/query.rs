//! Input parser for weather queries: query graph (`en`) → typed query `WeatherQuery`
//! (place, time, condition, units). Spans are BIO labels on graph nodes from a "graph feature →
//! label weights" table (averaged perceptron, as in the `en` parser); features are numbers: lemma number, tag,
//! relation, head lemma, preposition attached to the word, neighbours, form. Units are a rule on word
//! constants (`w::CELSIUS`, `w::FAHRENHEIT`): integer comparison.
//! Training annotation: MASSIVE (CC BY 4.0) and SNIPS (CC0), `[label : words]`.

use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use en::dict::{self, w};
use en::gram::{Rel, Sym};
use en::hash::{FastMap, key, str_key};
use en::model::{Graph, Model};
use en::tok;

use crate::data::TempUnit;

/// Query slot type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Slot {
    Place,
    Time,
    Cond,
}

const SLOTS: [Slot; 3] = [Slot::Place, Slot::Time, Slot::Cond];

impl Slot {
    /// Unification of MASSIVE / SNIPS labels; the rest (spatial_relation, event_name…) is outside slots.
    pub fn from_label(label: &str) -> Option<Slot> {
        match label {
            "place_name" | "city" | "state" | "country" | "geographic_poi" | "current_location" => Some(Slot::Place),
            "date" | "time" | "timeofday" | "timeRange" => Some(Slot::Time),
            "weather_descriptor" | "condition_description" | "condition_temperature" => Some(Slot::Cond),
            _ => None,
        }
    }
}

/// BIO label on a node: outside a slot, beginning, continuation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Bio {
    O,
    B(Slot),
    I(Slot),
}

const NB: usize = 1 + 2 * SLOTS.len();

impl Bio {
    fn idx(self) -> usize {
        match self {
            Bio::O => 0,
            Bio::B(s) => 1 + 2 * s as usize,
            Bio::I(s) => 2 + 2 * s as usize,
        }
    }
    fn of(i: usize) -> Bio {
        match i {
            0 => Bio::O,
            i if i % 2 == 1 => Bio::B(SLOTS[(i - 1) / 2]),
            i => Bio::I(SLOTS[(i - 2) / 2]),
        }
    }
    /// I-slot only after B or I of the same slot.
    fn allowed(prev: Bio, cur: Bio) -> bool {
        match cur {
            Bio::I(s) => prev == Bio::B(s) || prev == Bio::I(s),
            _ => true,
        }
    }
}

/// Slot span over graph nodes: [start, end).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub slot: Slot,
}

/// Annotated utterance: text and slots as byte offsets in the text.
pub struct Utterance {
    pub text: String,
    pub spans: Vec<(usize, usize, Slot)>,
}

/// «is it [weather_descriptor : raining] in [place_name : boston]» → text and slots.
pub fn parse_annot(annot: &str) -> Utterance {
    let mut text = String::new();
    let mut spans = Vec::new();
    let mut rest = annot;
    while let Some(open) = rest.find('[') {
        text.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let Some(close) = after.find(']') else { break };
        let inner = &after[..close];
        if let Some((label, words)) = inner.split_once(" : ") {
            let start = text.len();
            text.push_str(words.trim());
            if let Some(s) = Slot::from_label(label.trim()) {
                spans.push((start, text.len(), s));
            }
        } else {
            text.push_str(inner);
        }
        rest = &after[close + 1..];
    }
    text.push_str(rest);
    Utterance { text, spans }
}

#[derive(Deserialize)]
struct MassiveRow {
    partition: String,
    annot_utt: String,
}

/// MASSIVE (JSONL, weather_query) and SNIPS (TSV: text \t annotation) → (train, test): MASSIVE
/// train+dev and the first 1,800 SNIPS are training; MASSIVE test and the rest of SNIPS are test.
pub fn load(massive: &Path, snips: &Path) -> Result<(Vec<Utterance>, Vec<Utterance>)> {
    let (mut train, mut test) = (Vec::new(), Vec::new());
    for line in std::fs::read_to_string(massive)?.lines().filter(|l| !l.trim().is_empty()) {
        let r: MassiveRow = serde_json::from_str(line)?;
        let u = parse_annot(&r.annot_utt);
        if r.partition == "test" { test.push(u) } else { train.push(u) }
    }
    for (i, line) in std::fs::read_to_string(snips)?.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let Some((_, annot)) = line.split_once('\t') else { continue };
        let u = parse_annot(annot);
        if i < 1800 { train.push(u) } else { test.push(u) }
    }
    Ok((train, test))
}

/// Query graph and the byte boundaries of its nodes in the text.
pub struct Parsed {
    pub graph: Graph,
    pub forms: Vec<String>,
    pub offs: Vec<(usize, usize)>,
}

pub fn prepare(model: &Model, text: &str) -> Parsed {
    let graph = model.encode(text);
    let forms: Vec<String> = tok::tokenize(text, &model.lx).into_iter().map(|t| t.form).collect();
    let mut pos = 0;
    let offs = forms
        .iter()
        .map(|f| match text[pos..].find(f.as_str()) {
            Some(i) => {
                let s = pos + i;
                pos = s + f.len();
                (s, s + f.len())
            }
            None => (pos, pos),
        })
        .collect();
    Parsed { graph, forms, offs }
}

/// Gold BIO from byte spans.
fn gold_bio(offs: &[(usize, usize)], spans: &[(usize, usize, Slot)]) -> Vec<Bio> {
    let mut out = vec![Bio::O; offs.len()];
    for &(s, e, slot) in spans {
        let mut first = true;
        for (i, &(a, b)) in offs.iter().enumerate() {
            if a >= s && b <= e && b > a {
                out[i] = if first { Bio::B(slot) } else { Bio::I(slot) };
                first = false;
            }
        }
    }
    out
}

/// Spans from BIO.
pub fn spans(bio: &[Bio]) -> Vec<Span> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bio.len() {
        if let Bio::B(s) = bio[i] {
            let mut j = i + 1;
            while j < bio.len() && bio[j] == Bio::I(s) {
                j += 1;
            }
            out.push(Span { start: i, end: j, slot: s });
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

/// Lemma key for features: lower-case number (dictionary heap) or string hash (local heap).
fn lex(g: &Graph, s: Sym) -> u64 {
    if s.is_local() {
        return str_key(&g.local[s.index()].to_lowercase()) | 1 << 63;
    }
    let l = dict::lower(s);
    if l == Sym::NONE { s.raw() as u64 } else { l.raw() as u64 }
}

fn feats(p: &Parsed, i: usize, prev: Bio) -> Vec<u64> {
    let g = &p.graph;
    let n = g.nodes.len();
    let node = g.nodes[i];
    let li = lex(g, node.lemma);
    let head = node.head as usize;
    let (hl, ht) = if head == 0 { (u64::MAX, u64::MAX) } else { (lex(g, g.nodes[head - 1].lemma), g.nodes[head - 1].tag as u64) };
    // preposition attached to the word: a dependent with case («in Boston»)
    let case = (0..n).find(|&j| g.nodes[j].head as usize == i + 1 && g.nodes[j].rel == Rel::Case).map_or(0, |j| lex(g, g.nodes[j].lemma));
    let f = &p.forms[i];
    let shape: u64 = if f.chars().all(|c| c.is_ascii_digit()) {
        1
    } else if f.chars().any(|c| c.is_ascii_digit()) {
        2
    } else if f.chars().next().is_some_and(char::is_uppercase) {
        3
    } else {
        4
    };
    let low = f.to_lowercase();
    let suf: String = low.chars().rev().take(3).collect();
    let (pl, pt) = if i > 0 { (lex(g, g.nodes[i - 1].lemma), g.nodes[i - 1].tag as u64) } else { (0, u64::MAX) };
    let (nl, nt) = if i + 1 < n { (lex(g, g.nodes[i + 1].lemma), g.nodes[i + 1].tag as u64) } else { (1, u64::MAX - 1) };
    let (t, r, pb) = (node.tag as u64, node.rel as u64, prev.idx() as u64);
    vec![
        key(&[1, li]),
        key(&[2, t]),
        key(&[3, r]),
        key(&[4, hl]),
        key(&[5, r, hl]),
        key(&[6, case]),
        key(&[7, case, t]),
        key(&[8, pl]),
        key(&[9, nl]),
        key(&[10, pt, t, nt]),
        key(&[11, shape]),
        key(&[12, str_key(&suf)]),
        key(&[13, pb]),
        key(&[14, pb, li]),
        key(&[15, pb, t]),
        key(&[16, pl, li]),
        key(&[17, li, nl]),
        key(&[18, r, ht]),
        key(&[19, pb, case]),
    ]
}

/// Slot table: averaged perceptron over BIO.
pub struct Slotter {
    w: FastMap<u64, [f32; NB]>,
}

impl Slotter {
    fn best(w: &FastMap<u64, [f32; NB]>, fs: &[u64], prev: Bio) -> Bio {
        let mut s = [0.0f32; NB];
        for f in fs {
            if let Some(x) = w.get(f) {
                for k in 0..NB {
                    s[k] += x[k];
                }
            }
        }
        (0..NB).map(Bio::of).filter(|&b| Bio::allowed(prev, b)).max_by(|a, b| s[a.idx()].total_cmp(&s[b.idx()])).unwrap_or(Bio::O)
    }

    pub fn train(data: &[(Parsed, Vec<Bio>)], epochs: usize) -> Slotter {
        let mut w: FastMap<u64, [f32; NB]> = FastMap::default();
        let mut u: FastMap<u64, [f32; NB]> = FastMap::default();
        let mut c = 1.0f32;
        let mut order: Vec<usize> = (0..data.len()).collect();
        let mut rng = 0x2545F4914F6CDD1Du64;
        for _ in 0..epochs {
            for i in (1..order.len()).rev() {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                order.swap(i, (rng % (i as u64 + 1)) as usize);
            }
            for &di in &order {
                let (p, gold) = &data[di];
                let mut prev = Bio::O;
                for i in 0..p.graph.nodes.len() {
                    let fs = feats(p, i, prev);
                    let pred = Self::best(&w, &fs, prev);
                    if pred != gold[i] {
                        for f in &fs {
                            let x = w.entry(*f).or_insert([0.0; NB]);
                            x[gold[i].idx()] += 1.0;
                            x[pred.idx()] -= 1.0;
                            let y = u.entry(*f).or_insert([0.0; NB]);
                            y[gold[i].idx()] += c;
                            y[pred.idx()] -= c;
                        }
                    }
                    prev = gold[i];
                    c += 1.0;
                }
            }
        }
        for (f, x) in w.iter_mut() {
            if let Some(y) = u.get(f) {
                for k in 0..NB {
                    x[k] -= y[k] / c;
                }
            }
        }
        Slotter { w }
    }

    pub fn tag(&self, p: &Parsed) -> Vec<Bio> {
        let mut out = Vec::with_capacity(p.graph.nodes.len());
        let mut prev = Bio::O;
        for i in 0..p.graph.nodes.len() {
            // rule on constants: a unit word is not a slot, it goes into the units field
            let b = if unit_of(p, i).is_some() { Bio::O } else { Self::best(&self.w, &feats(p, i, prev), prev) };
            out.push(b);
            prev = b;
        }
        out
    }
}

/// Typed query — a "command" for the weather code.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct WeatherQuery {
    pub place: Option<String>,
    pub time: Option<String>,
    pub condition: Option<String>,
    /// units, if the query names them (rule on word constants)
    pub unit: Option<TempUnit>,
}

/// Query from a graph: slots (first span of each type) + units.
pub fn query(p: &Parsed, bio: &[Bio]) -> WeatherQuery {
    let text = |s: &Span| -> String { p.forms[s.start..s.end].join(" ") };
    let first = |slot: Slot| spans(bio).into_iter().find(|s| s.slot == slot).map(|s| text(&s));
    let unit = (0..p.graph.nodes.len()).find_map(|i| unit_of(p, i));
    WeatherQuery { place: first(Slot::Place), time: first(Slot::Time), condition: first(Slot::Cond), unit }
}

/// Training and evaluation: span F1 per slot, exact match of all slots, query examples.
pub fn eval(model: &Model, train: &[Utterance], test: &[Utterance]) {
    let prep = |us: &[Utterance]| -> Vec<(Parsed, Vec<Bio>)> {
        us.iter()
            .map(|u| {
                let p = prepare(model, &u.text);
                let g = gold_bio(&p.offs, &u.spans);
                (p, g)
            })
            .collect()
    };
    let tr = prep(train);
    let te = prep(test);
    let t0 = std::time::Instant::now();
    let slotter = Slotter::train(&tr, 10);
    println!("slots: train {}, test {}, training {:.1} s", tr.len(), te.len(), t0.elapsed().as_secs_f64());
    let mut per: FastMap<Slot, (usize, usize, usize)> = FastMap::default(); // (match, predicted, gold)
    let mut exact = 0;
    let (mut unit_n, mut unit_ok) = (0, 0);
    let mut shown = 0;
    for (p, gold) in &te {
        let pred = slotter.tag(p);
        let (ps, gs) = (spans(&pred), spans(gold));
        for s in &ps {
            per.entry(s.slot).or_default().1 += 1;
            if gs.contains(s) {
                per.entry(s.slot).or_default().0 += 1;
            }
        }
        for s in &gs {
            per.entry(s.slot).or_default().2 += 1;
        }
        if ps == gs {
            exact += 1;
        } else if shown < 5 {
            shown += 1;
            println!("  {}\n    ours : {:?}\n    gold : {:?}", p.forms.join(" "), query(p, &pred), query(p, gold));
        }
        let q = query(p, &pred);
        let low = p.forms.join(" ").to_lowercase();
        if low.contains("celsius") || low.contains("fahrenheit") {
            unit_n += 1;
            let want = if low.contains("celsius") { TempUnit::C } else { TempUnit::F };
            if q.unit == Some(want) {
                unit_ok += 1;
            }
        }
    }
    let (mut hit, mut np, mut ng) = (0, 0, 0);
    for s in SLOTS {
        let (h, p_, g) = per.get(&s).copied().unwrap_or_default();
        let (pr, rc) = (h as f64 / p_.max(1) as f64, h as f64 / g.max(1) as f64);
        println!("  {s:?}: P {:.1}% R {:.1}% F1 {:.1}% (gold {g})", 100.0 * pr, 100.0 * rc, 200.0 * pr * rc / (pr + rc).max(1e-9));
        hit += h;
        np += p_;
        ng += g;
    }
    let (pr, rc) = (hit as f64 / np.max(1) as f64, hit as f64 / ng.max(1) as f64);
    println!(
        "  all slots: F1 {:.1}%; exact query (all spans) {:.1}%; units (celsius/fahrenheit) {unit_ok}/{unit_n}",
        200.0 * pr * rc / (pr + rc).max(1e-9),
        100.0 * exact as f64 / te.len().max(1) as f64
    );
    for text in ["should i bring an umbrella tomorrow in boston", "what is the temperature in celsius in Kyiv this weekend", "Will it snow in Denver tonight?"] {
        let p = prepare(model, text);
        println!("  «{text}» → {}", serde_json::to_string(&query(&p, &slotter.tag(&p))).unwrap_or_default());
    }
}

/// Units named by a node: comparing the lemma number with word constants.
fn unit_of(p: &Parsed, i: usize) -> Option<TempUnit> {
    let s = p.graph.nodes[i].lemma;
    let l = if s.is_local() { dict::lex(p.graph.str(s)) } else { dict::lower(s) };
    if l == w::CELSIUS {
        Some(TempUnit::C)
    } else if l == w::FAHRENHEIT {
        Some(TempUnit::F)
    } else {
        None
    }
}
