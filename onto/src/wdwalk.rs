//! Wikidata walker for the world model (onto v2).
//!
//! **Folk «is a kind of».** Wikidata cladistics leads the whale into fish: whale —P279→ Cetacea —P171→ Mammalia
//! —P171×22→ Sarcopterygii → Osteichthyes —P279→ fish. Rule:
//! - a P171 chain does not climb from a taxon of rank «class» or above (P105);
//! - exception — one step class → superclass (a grade: Actinopterygii → Osteichthyes —P279→ fish), otherwise a pike is not a fish;
//! - bypass clades (P171 past the class) are dropped: from the P171 region only nodes from which the class is reachable remain;
//! - P279 — unrestricted.
//!
//! Negative control — the same with the rule switched off: the whale is a fish again.
//!
//! **Core walker.** Core concept (`data/core.tsv`) → OEWN synset → anchor Q (strict v2 mapping; for animals
//! and plants without a taxon — also a taxon anchor) → statements of the anchor and its folk ancestors → seed candidates with
//! a source (Q, P, rank), confidence, and numbers as a distribution (mean, σ, min, max, n, unit).
//! These are candidates, not the truth: they go into the core only after confirmation.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write as _;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};

use crate::model::{Cid, Graph};
use crate::wdgraph::{for_each_row, parts, read_items, IsaGraph, P171, P279};

pub const P31: u16 = 31;
pub const P105: u16 = 105;
/// Rank «class» (Q37517) and «superclass» (Q3504061).
pub const CLASS: u32 = 37517;
pub const SUPERCLASS: u32 = 3504061;

/// Ranks above class (P105): a P171 chain stops at them.
pub const HIGH_RANKS: &[u32] = &[
    60922428, // megaclass
    2361851,  // infraphylum
    1153785,  // subphylum
    38348,    // phylum
    2111790,  // superphylum
    334460,   // division (botany — phylum)
    30093070, // division
    3491997,  // subdivision (botany — above class)
    30093105, // subdivision
    23760204, // superdivision
    36732,    // kingdom
    2752679,  // subkingdom
    3150876,  // infrakingdom
    19858692, // superkingdom
    146481,   // domain
    62075839, // realm
];

/// Kind of a node's rank for the stop rule.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RankKind {
    /// below class or no rank — P171 upward freely
    Low,
    /// class — P171 only up to a superclass (grade)
    Class,
    /// superclass — stop
    Super,
    /// above — stop
    High,
}

pub fn rank_kind(r: Option<u32>) -> RankKind {
    match r {
        Some(CLASS) => RankKind::Class,
        Some(SUPERCLASS) => RankKind::Super,
        Some(x) if HIGH_RANKS.contains(&x) => RankKind::High,
        _ => RankKind::Low,
    }
}

pub fn rank_kinds(g: &IsaGraph, rank: &HashMap<u32, u32>) -> Vec<RankKind> {
    g.nodes.iter().map(|q| rank_kind(rank.get(q).copied())).collect()
}

/// P171 region of a node (graph indices, P171 steps). Stops at class and above, the step class → superclass is allowed.
/// If the region has a class, only nodes from which the class is reachable remain (bypass clades are dropped), plus
/// superclasses reached from the class; no class — the target is any stop; no stops — the whole region (as in v1).
pub fn p171_region(g: &IsaGraph, rk: &[RankKind], v: u32) -> Vec<(u32, u16)> {
    let climbs = |x: u32, w: u32| match rk[x as usize] {
        RankKind::Low => true,
        RankKind::Class => rk[w as usize] == RankKind::Super,
        _ => false,
    };
    let mut order: Vec<(u32, u16)> = vec![(v, 0)];
    let mut edges: Vec<(u32, u32)> = Vec::new();
    let mut i = 0;
    while i < order.len() {
        let (x, d) = order[i];
        i += 1;
        for k in g.ups(x) {
            let w = g.up[k];
            if g.kind[k] != P171 || !climbs(x, w) {
                continue;
            }
            edges.push((x, w));
            if !order.iter().any(|&(y, _)| y == w) {
                order.push((w, d + 1));
            }
        }
    }
    let mut classes: Vec<u32> = order.iter().map(|x| x.0).filter(|&x| rk[x as usize] == RankKind::Class).collect();
    if classes.len() > 1 {
        // only the lowest classes: a class that in the full cladistics is above another class of the region (Sarcopterygii over
        // Mammalia) is reached by a bypass clade — it is not a target
        let above: Vec<Vec<u32>> = classes.iter().map(|&c| p171_all(g, c)).collect();
        let low: Vec<u32> = classes.iter().copied().filter(|c| !above.iter().any(|a| a.contains(c))).collect();
        classes = low;
    }
    let targets: Vec<u32> = if classes.is_empty() { order.iter().map(|x| x.0).filter(|&x| rk[x as usize] != RankKind::Low).collect() } else { classes };
    if targets.is_empty() {
        return order;
    }
    let mut keep: Vec<u32> = targets.clone();
    let mut st = targets;
    while let Some(w) = st.pop() {
        for &(x, y) in &edges {
            if y == w && !keep.contains(&x) {
                keep.push(x);
                st.push(x);
            }
        }
    }
    // superclasses reached from the class (grade)
    for &(x, w) in &edges {
        if rk[x as usize] == RankKind::Class && keep.contains(&x) && !keep.contains(&w) {
            keep.push(w);
        }
    }
    order.into_iter().filter(|(x, _)| *x == v || keep.contains(x)).collect()
}

/// All ancestors via P171 without restrictions (indices, without the node itself).
fn p171_all(g: &IsaGraph, v: u32) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();
    let mut st = vec![v];
    while let Some(x) = st.pop() {
        for k in g.ups(x) {
            let w = g.up[k];
            if g.kind[k] == P171 && w != v && !out.contains(&w) {
                out.push(w);
                st.push(w);
            }
        }
    }
    out
}

/// Folk ancestors of a node (graph indices, without the node itself): P171 — within the P171 region, P279 — freely.
/// `class_stop = false` — full cladistics (negative control, v1 behaviour).
pub fn folk_ancestors(g: &IsaGraph, rk: &[RankKind], v: u32, max_depth: u16, class_stop: bool) -> Vec<(u32, u16)> {
    if !class_stop {
        return g.ancestors(v, max_depth);
    }
    // relaxation: a node is expanded again only if reached by a shorter path (whale —P279→ Cetacea
    // —P279→ mammal in 2 steps, though the P171 chain from Cetacea to Mammalia is 16)
    let mut best: HashMap<u32, u16> = HashMap::new();
    let mut q: VecDeque<(u32, u16)> = VecDeque::from([(v, 0)]);
    while let Some((x, d)) = q.pop_front() {
        if best.get(&x).is_some_and(|&e| e < d) {
            continue;
        }
        for (u, dr) in p171_region(g, rk, x) {
            let du = d + dr;
            if du > max_depth || (u != x && best.get(&u).is_some_and(|&e| e <= du)) {
                continue;
            }
            best.insert(u, du);
            if du < max_depth {
                for k in g.ups(u) {
                    let w = g.up[k];
                    if g.kind[k] == P279 && best.get(&w).is_none_or(|&e| e > du + 1) {
                        best.insert(w, du + 1);
                        q.push_back((w, du + 1));
                    }
                }
            }
        }
    }
    best.remove(&v);
    let mut out: Vec<(u32, u16)> = best.into_iter().collect();
    out.sort_by_key(|&(a, d)| (d, a));
    out
}

/// Whether x is a kind of y (Q numbers) by the folk rule (or by full cladistics).
#[cfg_attr(not(test), allow(dead_code))]
pub fn folk_isa(g: &IsaGraph, rk: &[RankKind], x: u32, y: u32, class_stop: bool) -> Option<u16> {
    let (vx, vy) = (g.idx(x)?, g.idx(y)?);
    folk_ancestors(g, rk, vx, 64, class_stop).into_iter().find(|&(a, _)| a == vy).map(|(_, d)| d)
}

/// Path x → y (Q numbers) by the folk rule — for the proof in the report (BFS with parents).
pub fn folk_path(g: &IsaGraph, rk: &[RankKind], x: u32, y: u32) -> Option<Vec<u32>> {
    let (vx, vy) = (g.idx(x)?, g.idx(y)?);
    let allowed: HashSet<u32> = folk_ancestors(g, rk, vx, 64, true).into_iter().map(|(a, _)| a).chain([vx]).collect();
    if !allowed.contains(&vy) {
        return None;
    }
    let mut par: HashMap<u32, u32> = HashMap::new();
    let mut q = VecDeque::from([vx]);
    while let Some(a) = q.pop_front() {
        if a == vy {
            let mut p = vec![g.nodes[a as usize]];
            let mut c = a;
            while let Some(&b) = par.get(&c) {
                p.push(g.nodes[b as usize]);
                c = b;
            }
            p.reverse();
            return Some(p);
        }
        for k in g.ups(a) {
            let w = g.up[k];
            if allowed.contains(&w) && w != vx && !par.contains_key(&w) {
                par.insert(w, a);
                q.push_back(w);
            }
        }
    }
    None
}

/// Statements from `claims-*.tsv.zst` in one pass (files in parallel), rank not deprecated.
#[derive(Default)]
pub struct Claims {
    /// P105: taxon → rank
    pub rank: HashMap<u32, u32>,
    /// P31 for the selected items
    pub p31: HashMap<u32, Vec<u32>>,
    /// other properties: (s, p, o, rank)
    pub rows: Vec<(u32, u16, u32, u8)>,
}

pub fn scan_claims(dir: &Path, p31_for: &(dyn Fn(u32) -> bool + Sync), props: &[u16], subj: &(dyn Fn(u32) -> bool + Sync)) -> Result<Claims> {
    let files = parts(dir, "claims")?;
    let res: Vec<Result<Claims>> = std::thread::scope(|sc| {
        let hs: Vec<_> = files
            .iter()
            .map(|p| {
                sc.spawn(move || -> Result<Claims> {
                    let mut c = Claims::default();
                    for_each_row(p, |f| {
                        if f.len() < 4 || f[3] == "0" {
                            return Ok(());
                        }
                        let pn: u16 = f[1].parse()?;
                        if pn == P105 {
                            let s: u32 = f[0].parse()?;
                            let r: u32 = f[2].parse()?;
                            // the better statement rank wins: preferred (2) over normal (1)
                            if f[3] == "2" {
                                c.rank.insert(s, r);
                            } else {
                                c.rank.entry(s).or_insert(r);
                            }
                        } else if pn == P31 {
                            let s: u32 = f[0].parse()?;
                            if p31_for(s) {
                                c.p31.entry(s).or_default().push(f[2].parse()?);
                            }
                        } else if props.contains(&pn) {
                            let s: u32 = f[0].parse()?;
                            if subj(s) {
                                c.rows.push((s, pn, f[2].parse()?, f[3].parse()?));
                            }
                        }
                        Ok(())
                    })?;
                    Ok(c)
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| Err(anyhow::anyhow!("thread panicked")))).collect()
    });
    let mut out = Claims::default();
    for r in res {
        let r = r?;
        for (s, x) in r.rank {
            out.rank.entry(s).or_insert(x);
        }
        for (s, v) in r.p31 {
            out.p31.entry(s).or_default().extend(v);
        }
        out.rows.extend(r.rows);
    }
    Ok(out)
}

// ── core walker ──────────────────────────────────────────────────────────────────────────────────────

/// Item-valued properties we collect: (P, name). Sound (P4733) is not collected (it was a joke).
pub const ITEM_PROPS: &[(u16, &str)] = &[
    (1034, "food"),
    (2974, "habitat"),
    (527, "part"),
    (2670, "part"),
    (3113, "no_part"),
    (186, "material"),
    (462, "color"),
    (366, "use"),
];

/// Numeric properties: (P, name, SI unit).
pub const NUM_PROPS: &[(u16, &str, &str)] = &[(2067, "mass", "kg"), (2043, "length", "m"), (2048, "height", "m"), (2250, "lifespan", "year"), (2052, "speed", "km/h")];

/// Wikidata unit → factor to the property's SI unit; unknown unit — `None` (the value is skipped).
pub fn unit_factor(p: u16, unit: u32) -> Option<f64> {
    match (p, unit) {
        (2067, 11570) => Some(1.0),         // kilogram
        (2067, 41803) => Some(1e-3),        // gram
        (2067, 191118) => Some(1e3),        // tonne
        (2067, 100995) => Some(0.453_592_37), // pound
        (2067, 48013) => Some(0.028_349_523), // ounce
        (2067, 3241121) => Some(1e-6),      // milligram
        (2043 | 2048, 11573) => Some(1.0),  // metre
        (2043 | 2048, 174728) => Some(1e-2), // centimetre
        (2043 | 2048, 174789) => Some(1e-3), // millimetre
        (2043 | 2048, 828224) => Some(1e3), // kilometre
        (2043 | 2048, 218593) => Some(0.0254), // inch
        (2043 | 2048, 3710) => Some(0.3048), // foot
        (2250, 577) => Some(1.0),           // year
        (2250, 5151) => Some(1.0 / 12.0),   // month
        (2250, 23387) => Some(7.0 / 365.25), // week
        (2250, 573) => Some(1.0 / 365.25),  // day
        (2052, 180154) => Some(1.0),        // km/h
        (2052, 182429) => Some(3.6),        // m/s
        (2052, 211256) => Some(1.609_344),  // mph
        (2052, 128822) => Some(1.852),      // knot
        _ => None,
    }
}

/// Core concept domain → OEWN lexicographer file for sense choice.
pub fn domain_lexfile(d: &str) -> Option<&'static str> {
    match d {
        "animal" => Some("noun.animal"),
        "food" => Some("noun.food"),
        "plant" => Some("noun.plant"),
        "household" => Some("noun.artifact"),
        "family" | "role" | "magical" => Some("noun.person"),
        _ => None,
    }
}

pub struct CoreConcept {
    pub lemma: String,
    pub domain: String,
    pub synset: Option<Cid>,
    /// sense found in the domain's lexicographer file
    pub lf_ok: bool,
}

/// `data/core.tsv`: lemma, domain. Gates: unknown domain, duplicate.
pub fn read_core(g: &Graph, path: &Path) -> Result<Vec<CoreConcept>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    let mut v = Vec::new();
    let mut seen = HashSet::new();
    for l in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let mut f = l.split('\t');
        let lemma = f.next().unwrap_or("").trim().to_string();
        let domain = f.next().unwrap_or("").trim().to_string();
        anyhow::ensure!(domain_lexfile(&domain).is_some(), "unknown domain «{domain}» in line «{l}»");
        anyhow::ensure!(seen.insert((lemma.clone(), domain.clone())), "duplicate «{lemma}» ({domain})");
        // v3: the third column is an explicit synset, when the first sense in the domain lexfile is wrong (dolphin — not
        // dolphinfish, oak — not wood); gate: the lemma is a synset member, the synset is in the domain lexfile
        if let Some(id) = f.next().map(str::trim).filter(|s| !s.is_empty() && !s.starts_with('#')) {
            let c = g.find_synset(id).with_context(|| format!("synset {id} is not in OEWN (core, «{lemma}»)"))?;
            let name = lemma.replace('_', " ");
            anyhow::ensure!(g.members(c).iter().any(|w| g.word(*w).eq_ignore_ascii_case(&name)), "lemma «{lemma}» is not a member of synset {id}");
            let lf_ok = domain_lexfile(&domain) == Some(g.lexfile_name(c));
            anyhow::ensure!(lf_ok, "synset {id} for «{lemma}» is not in the lexfile of domain {domain}");
            v.push(CoreConcept { lemma, domain, synset: Some(c), lf_ok });
            continue;
        }
        let (_, c, m) = crate::bridge::map_class(g, &lemma, domain_lexfile(&domain));
        v.push(CoreConcept { lemma, domain, synset: c, lf_ok: m });
    }
    Ok(v)
}

/// v2 mapping row (`synset_q`): Q, status, number of source supports, length of the ancestor support.
#[derive(Clone, Debug)]
pub struct AlignRow {
    pub q: u32,
    pub status: String,
    #[allow(dead_code)]
    pub sources: String,
    pub n_src: u8,
    pub d: u16,
}

/// Source supports: label, lexeme, direct link (ILI or P8814 — one support).
pub fn n_sources_str(s: &str) -> u8 {
    let has = |x: &str| s.split('+').any(|p| p == x);
    has("label") as u8 + has("lex") as u8 + (has("ili") || has("wn")) as u8
}

pub fn read_align(path: &Path, want: &HashSet<String>) -> Result<HashMap<String, Vec<AlignRow>>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    let mut m: HashMap<String, Vec<AlignRow>> = HashMap::new();
    for l in text.lines() {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 8 || !want.contains(f[0]) {
            continue;
        }
        let d = f[6].parse::<u16>().unwrap_or(99) + f[7].parse::<u16>().unwrap_or(99);
        m.entry(f[0].to_string()).or_default().push(AlignRow { q: f[1].parse()?, status: f[2].to_string(), sources: f[3].to_string(), n_src: n_sources_str(f[3]), d });
    }
    Ok(m)
}

/// Concept anchor: Q, anchor confidence, how it was found.
#[derive(Clone, Debug)]
pub struct Anchor {
    pub q: u32,
    pub conf: f64,
    pub how: &'static str,
}

/// Rank preference for a taxon anchor: genus, species, family, other.
fn taxon_rank_pref(r: Option<u32>) -> u8 {
    match r {
        Some(34740) => 0, // genus
        Some(7432) => 1,  // species
        Some(35409) => 2, // family
        Some(_) => 3,
        None => 4,
    }
}

/// Anchors from mapping rows: the main one (strict → two supports → one support with ancestors) and, for
/// animals without a class among folk ancestors (a common name: crow —P279→ omnivore), a taxon anchor.
pub fn pick_anchors(rows: &[AlignRow], want_taxon: bool, is_taxon: &dyn Fn(u32) -> bool, reaches_taxon: &dyn Fn(u32) -> bool, rank: &dyn Fn(u32) -> Option<u32>) -> Vec<Anchor> {
    let best = |f: &dyn Fn(&AlignRow) -> bool| rows.iter().filter(|r| f(r)).min_by_key(|r| (std::cmp::Reverse(r.n_src), r.d, r.q)).cloned();
    let main = if let Some(r) = rows.iter().find(|r| r.status == "accepted") {
        Some(Anchor { q: r.q, conf: 0.9, how: "strict mapping" })
    } else if let Some(r) = best(&|r| r.n_src >= 2) {
        Some(Anchor { q: r.q, conf: 0.7, how: "two supports, ancestors disagree" })
    } else {
        best(&|r| r.status == "single").map(|r| Anchor { q: r.q, conf: 0.6, how: "ancestor support, one source" })
    };
    let mut out: Vec<Anchor> = main.into_iter().collect();
    if want_taxon && !out.iter().any(|a| is_taxon(a.q) || reaches_taxon(a.q)) {
        if let Some(t) = rows.iter().filter(|r| r.status != "unsupported" && is_taxon(r.q)).min_by_key(|r| (taxon_rank_pref(rank(r.q)), r.d, r.q)) {
            out.push(Anchor { q: t.q, conf: 0.6, how: "taxon anchor" });
        }
    }
    out
}

/// Distribution of numeric values.
#[derive(Clone, Debug, Default)]
pub struct Dist {
    pub n: usize,
    pub mean: f64,
    pub sd: Option<f64>,
    pub min: f64,
    pub max: f64,
}

pub fn dist(vals: &[f64], bounds: &[f64]) -> Option<Dist> {
    if vals.is_empty() {
        return None;
    }
    let n = vals.len();
    let mean = vals.iter().sum::<f64>() / n as f64;
    let sd = (n >= 2).then(|| (vals.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt());
    let all = vals.iter().chain(bounds.iter()).copied();
    let min = all.clone().fold(f64::INFINITY, f64::min);
    let max = all.fold(f64::NEG_INFINITY, f64::max);
    Some(Dist { n, mean, sd, min, max })
}

/// Number with three significant digits.
pub fn sig(x: f64) -> String {
    if x == 0.0 || !x.is_finite() {
        return format!("{x}");
    }
    let mag = x.abs().log10().floor() as i32;
    let dec = (2 - mag).clamp(0, 9) as usize;
    let s = format!("{x:.dec$}");
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s }
}

/// Seed candidate.
#[derive(Clone, Debug)]
pub struct Cand {
    pub ci: usize,
    pub prop: &'static str,
    pub p: u16,
    /// item value (0 — a number)
    pub o: u32,
    pub src: u32,
    pub rank: u8,
    pub d: u16,
    pub anchor: u32,
    pub conf: f64,
    pub dist: Option<(Dist, &'static str, Vec<u32>, bool)>,
}

/// Descent: descendants of a node via P279/P171 up to `depth` steps, at most `cap`.
pub fn descendants(down_off: &[u32], down: &[u32], v: u32, depth: u16, cap: usize) -> Vec<u32> {
    let mut seen: HashSet<u32> = HashSet::from([v]);
    let mut q = VecDeque::from([(v, 0u16)]);
    let mut out = Vec::new();
    while let Some((x, d)) = q.pop_front() {
        if d >= depth {
            continue;
        }
        for &w in &down[down_off[x as usize] as usize..down_off[x as usize + 1] as usize] {
            if seen.insert(w) {
                out.push(w);
                if out.len() >= cap {
                    return out;
                }
                q.push_back((w, d + 1));
            }
        }
    }
    out
}

/// Control query: concept (animal lemma) or Q → class Q; expectation.
pub struct Control {
    pub what: &'static str,
    pub x: &'static str,
    pub y: u32,
    pub y_name: &'static str,
    pub expect: bool,
}

pub const CONTROLS: &[Control] = &[
    Control { what: "whale → mammals", x: "whale", y: 7377, y_name: "mammal Q7377", expect: true },
    Control { what: "whale → not fish", x: "whale", y: 152, y_name: "fish Q152", expect: false },
    Control { what: "crow → birds", x: "crow", y: 5113, y_name: "bird Q5113", expect: true },
    Control { what: "shark → fish", x: "shark", y: 152, y_name: "fish Q152", expect: true },
    Control { what: "bat → mammals", x: "bat", y: 7377, y_name: "mammal Q7377", expect: true },
    Control { what: "bat → not bird", x: "bat", y: 5113, y_name: "bird Q5113", expect: false },
    Control { what: "pike → fish (grade class → superclass)", x: "pike", y: 152, y_name: "fish Q152", expect: true },
    Control { what: "crow → not mammal", x: "crow", y: 7377, y_name: "mammal Q7377", expect: false },
];

pub struct WalkOut {
    pub report: String,
    #[allow(dead_code)]
    pub cands: Vec<Cand>,
    pub controls_ok: bool,
}

/// Full walker pass: cycle gate, control queries, core candidates.
pub fn run(g: &Graph, dir: &Path, align: &Path, core_path: &Path, out_tsv: &Path) -> Result<WalkOut> {
    let t0 = Instant::now();
    let mut rep = String::from("# onto v2: Wikidata walker\n\n");
    // «is a kind of» graph (after breaking cycles) and the cycle gate
    let mut edges: Vec<(u32, u32, u16)> = Vec::new();
    for_each_row(&dir.join("isa_edges.tsv.zst"), |f| {
        edges.push((f[0].parse()?, f[1].parse()?, f[2].parse()?));
        Ok(())
    })?;
    let isa = IsaGraph::from_edges(edges.clone());
    let cyc = crate::wdgraph::acyclic(&isa);
    // negative control of the gate: the edge fruit (Q3314483) ⊂ apple (Q89) closes a cycle
    edges.push((3314483, 89, P279));
    let bad = crate::wdgraph::acyclic(&IsaGraph::from_edges(edges));
    let gate_ok = cyc.is_none() && bad.is_some();
    let _ = writeln!(rep, "## Cycle gate\n\n- «is a kind of» graph: {} nodes, {} edges; acyclic: **{}**", isa.n(), isa.n_edges(), if cyc.is_none() { "yes" } else { "NO" });
    let _ = writeln!(rep, "- negative control: + edge fruit Q3314483 ⊂ apple Q89 → {} nodes in cycles — gate **{}**\n", bad.unwrap_or(0), if bad.is_some() { "turns red" } else { "does NOT turn red" });
    let t_graph = t0.elapsed().as_secs_f64();
    // core → synsets → mapping rows
    let mut core = read_core(g, core_path)?;
    let controls_extra: Vec<&str> = CONTROLS.iter().map(|c| c.x).filter(|x| !core.iter().any(|k| k.lemma == *x && k.domain == "animal")).collect();
    for x in controls_extra {
        let (_, c, m) = crate::bridge::map_class(g, x, Some("noun.animal"));
        core.push(CoreConcept { lemma: x.to_string(), domain: "control".into(), synset: c, lf_ok: m });
    }
    let want: HashSet<String> = core.iter().filter_map(|k| k.synset.map(|c| g.cid_str(c))).collect();
    let arows = read_align(align, &want)?;
    let qcands: HashSet<u32> = arows.values().flatten().map(|r| r.q).collect();
    // one pass over statements: ranks, P31 of candidates, properties for graph nodes and candidates
    let props: Vec<u16> = ITEM_PROPS.iter().map(|x| x.0).collect();
    let cl = scan_claims(dir, &|q| qcands.contains(&q), &props, &|q| qcands.contains(&q) || isa.idx(q).is_some())?;
    let rk = rank_kinds(&isa, &cl.rank);
    let t_scan = t0.elapsed().as_secs_f64();
    let is_taxon = |q: u32| cl.rank.contains_key(&q) || isa.idx(q).map(|v| isa.ups(v).any(|k| isa.kind[k] == P171)).unwrap_or(false);
    // «tied to the taxonomy»: a class (P105 = class) is among the folk ancestors. Kingdom Animalia (Q729
    // «animal») does not count — any common name reaches it (crow —P279→ omnivore → animal).
    let reaches_taxon = |q: u32| isa.idx(q).map(|v| rk[v as usize] == RankKind::Class || folk_ancestors(&isa, &rk, v, 24, true).iter().any(|&(a, _)| rk[a as usize] == RankKind::Class)).unwrap_or(false);
    let rank_of = |q: u32| cl.rank.get(&q).copied();
    let mut anchors: Vec<Vec<Anchor>> = Vec::new();
    for k in &core {
        let rows = k.synset.and_then(|c| arows.get(&g.cid_str(c))).map(|v| v.as_slice()).unwrap_or(&[]);
        // taxon anchor — only for animals: «tree» is a life form, not a taxon
        let want_taxon = matches!(k.domain.as_str(), "animal" | "control");
        anchors.push(pick_anchors(rows, want_taxon, &is_taxon, &reaches_taxon, &rank_of));
    }
    // folk ancestors of anchors: (node Q, distance); P31 — as the first step for instances outside the graph
    let anc_of = |q: u32, class_stop: bool| -> Vec<(u32, u16)> {
        let mut v: Vec<(u32, u16)> = Vec::new();
        let mut starts: Vec<(u32, u16)> = Vec::new();
        if let Some(x) = isa.idx(q) {
            starts.push((x, 0));
        } else {
            for &c in cl.p31.get(&q).map(|v| v.as_slice()).unwrap_or(&[]) {
                if let Some(x) = isa.idx(c) {
                    v.push((c, 1));
                    starts.push((x, 1));
                }
            }
        }
        for (x, d0) in starts {
            for (a, d) in folk_ancestors(&isa, &rk, x, 64 - d0, class_stop) {
                v.push((isa.nodes[a as usize], d + d0));
            }
        }
        v.sort_by_key(|&(a, d)| (a, d));
        v.dedup_by_key(|x| x.0);
        v
    };
    // control queries: concept → anchors → folk ancestors; and with the rule switched off (negative control)
    let _ = writeln!(rep, "## Control queries: P171 stops at the class rank\n\n| query | anchors | folk rule | full cladistics (control) | how |\n|---|---|---|---|---|");
    let mut controls_ok = true;
    let mut rule_matters = false;
    let mut ctl_lines: Vec<(usize, u32, u32)> = Vec::new();
    for c in CONTROLS {
        let Some(ci) = core.iter().position(|k| k.lemma == c.x && (k.domain == "animal" || k.domain == "control")) else {
            let _ = writeln!(rep, "| {} | — | no such word | | |", c.what);
            controls_ok = false;
            continue;
        };
        let an = &anchors[ci];
        let hit = |stop: bool| an.iter().find_map(|a| if a.q == c.y { Some((a.q, 0)) } else { anc_of(a.q, stop).into_iter().find(|&(x, _)| x == c.y).map(|(_, d)| (a.q, d)) });
        let folk = hit(true);
        let full = hit(false);
        let ok = folk.is_some() == c.expect;
        controls_ok &= ok;
        rule_matters |= folk.is_some() != full.is_some();
        if let Some((a, _)) = folk {
            ctl_lines.push((ci, a, c.y));
        }
        let _ = writeln!(
            rep,
            "| {} | {} | {} {} | {} | {} |",
            c.what,
            an.iter().map(|a| format!("Q{} ({})", a.q, a.how)).collect::<Vec<_>>().join(", "),
            if ok { "✓" } else { "✗" },
            folk.map(|(_, d)| format!("yes, {d} steps")).unwrap_or("no".into()),
            full.map(|(_, d)| format!("yes, {d} steps")).unwrap_or("no".into()),
            c.y_name
        );
    }
    let _ = writeln!(rep, "\nControls: **{}**. The rule changes the answer in at least one query (negative control — without the rule the whale is a fish): **{}**.\n", if controls_ok { "all green" } else { "SOME RED" }, if rule_matters { "yes" } else { "NO" });
    // candidates: anchor ancestors up to 3 steps and the class (P105 = class) at any depth
    let mut cands: Vec<Cand> = Vec::new();
    let mut need_sub: HashSet<u32> = HashSet::new();
    let mut walk: Vec<Vec<(u32, u16, u32, f64)>> = Vec::new(); // per concept: (node, distance, anchor, anchor confidence)
    let is_class_q = |x: u32| isa.idx(x).map(|v| rk[v as usize] == RankKind::Class).unwrap_or(false);
    for an in &anchors {
        let mut w: Vec<(u32, u16, u32, f64)> = Vec::new();
        for a in an {
            w.push((a.q, 0, a.q, a.conf));
            for (x, d) in anc_of(a.q, true) {
                if d <= 3 || is_class_q(x) {
                    w.push((x, d, a.q, a.conf));
                }
            }
        }
        walk.push(w);
    }
    // descent (P279/P171 downward): for ancestor generality and numeric distributions
    let generic_cap: usize = 20_000;
    let n = isa.n();
    let mut down_off = vec![0u32; n + 1];
    for &w in &isa.up {
        down_off[w as usize + 1] += 1;
    }
    for i in 0..n {
        down_off[i + 1] += down_off[i];
    }
    let mut down = vec![0u32; isa.up.len()];
    let mut fill = down_off.clone();
    for s in 0..n as u32 {
        for k in isa.ups(s) {
            let w = isa.up[k] as usize;
            down[fill[w] as usize] = s;
            fill[w] += 1;
        }
    }
    // too general ancestors — over 20 thousand descendants in the «is a kind of» graph (food, organism, physical object…):
    // we do not inherit from them; a class (bird, mammal, fish, insect) is always inherited
    let n_with = anchors.iter().filter(|a| !a.is_empty()).count().max(1);
    let mut cand_anc: HashSet<u32> = HashSet::new();
    for w in &walk {
        cand_anc.extend(w.iter().filter(|e| e.1 > 0 && !is_class_q(e.0)).map(|e| e.0));
    }
    let mut generic: Vec<(u32, usize)> = cand_anc
        .iter()
        .filter_map(|&x| {
            let v = isa.idx(x)?;
            let k = descendants(&down_off, &down, v, 64, generic_cap).len();
            (k >= generic_cap).then_some((x, walk.iter().filter(|w| w.iter().any(|e| e.0 == x)).count()))
        })
        .collect();
    generic.sort_by_key(|&(x, c)| (std::cmp::Reverse(c), x));
    let generic_set: HashSet<u32> = generic.iter().map(|g| g.0).collect();
    for w in walk.iter_mut() {
        w.retain(|e| e.1 == 0 || !generic_set.contains(&e.0));
        for &(x, ..) in w.iter() {
            need_sub.insert(x);
        }
    }
    let mut by_sub: HashMap<u32, Vec<(u16, u32, u8)>> = HashMap::new();
    for &(s, p, o, r) in &cl.rows {
        if need_sub.contains(&s) {
            by_sub.entry(s).or_default().push((p, o, r));
        }
    }
    let (mut dropped_taxon_part, mut dropped_low) = (0usize, 0usize);
    for (ci, w) in walk.iter().enumerate() {
        let mut best: HashMap<(u16, u32), Cand> = HashMap::new();
        for &(x, d, a, ac) in w {
            for &(p, o, r) in by_sub.get(&x).map(|v| v.as_slice()).unwrap_or(&[]) {
                let Some(&(_, name)) = ITEM_PROPS.iter().find(|z| z.0 == p) else { continue };
                // a «part» that is a taxon: these are members or subspecies (fox P527 red fox), not body parts
                if matches!(p, 527 | 2670 | 3113) && (is_taxon(o) || o == a || o == x) {
                    dropped_taxon_part += 1;
                    continue;
                }
                let conf = ac * if r == 2 { 1.0 } else { 0.9 } * 0.85f64.powi(d as i32);
                if conf < 0.4 {
                    dropped_low += 1;
                    continue;
                }
                let c = Cand { ci, prop: name, p, o, src: x, rank: r, d, anchor: a, conf, dist: None };
                let key = (if p == 2670 { 527 } else { p }, o);
                if best.get(&key).map(|b| conf > b.conf).unwrap_or(true) {
                    best.insert(key, c);
                }
            }
        }
        let mut v: Vec<Cand> = best.into_values().collect();
        v.sort_by(|a, b| a.prop.cmp(b.prop).then(b.conf.total_cmp(&a.conf)).then(a.o.cmp(&b.o)));
        cands.extend(v);
    }
    // numbers: the anchor's own values, otherwise a distribution over descendants (P279/P171 downward, up to 3 steps, ≤ 400)
    let mut desc_of: HashMap<u32, Vec<u32>> = HashMap::new();
    for an in &anchors {
        for a in an {
            if let Some(v) = isa.idx(a.q) {
                desc_of.entry(a.q).or_insert_with(|| descendants(&down_off, &down, v, 3, 400).into_iter().map(|x| isa.nodes[x as usize]).collect());
            }
        }
    }
    drop(down);
    let qneed: HashSet<u32> = anchors.iter().flatten().map(|a| a.q).chain(desc_of.values().flatten().copied()).collect();
    let mut quant: HashMap<(u32, u16), (Vec<f64>, Vec<f64>, u8)> = HashMap::new();
    let (mut q_rows, mut q_skip, mut q_birth) = (0usize, 0usize, 0usize);
    for p in parts(dir, "quant")? {
        for_each_row(&p, |f| {
            if f.len() < 7 || f[6] == "0" {
                return Ok(());
            }
            let s: u32 = f[0].parse()?;
            if !qneed.contains(&s) {
                return Ok(());
            }
            let pn: u16 = f[1].parse()?;
            if !NUM_PROPS.iter().any(|x| x.0 == pn) {
                return Ok(());
            }
            // birth weight (P3831 = Q4128476) is not about a typical adult: lion 1.65 kg
            if f.get(7).is_some_and(|x| x.contains("P3831=Q4128476")) {
                q_birth += 1;
                return Ok(());
            }
            let Some(k) = unit_factor(pn, f[3].parse().unwrap_or(0)) else {
                q_skip += 1;
                return Ok(());
            };
            q_rows += 1;
            let e = quant.entry((s, pn)).or_default();
            e.0.push(f[2].parse::<f64>()? * k);
            for b in [f[4], f[5]] {
                if let Ok(x) = b.parse::<f64>() {
                    e.1.push(x * k);
                }
            }
            e.2 = e.2.max(f[6].parse()?);
            Ok(())
        })?;
    }
    for (ci, an) in anchors.iter().enumerate() {
        for &(p, name, unit) in NUM_PROPS {
            let own = an.iter().find_map(|a| quant.get(&(a.q, p)).map(|x| (a, x)));
            if let Some((a, (vals, bounds, r))) = own {
                if let Some(d) = dist(vals, bounds) {
                    cands.push(Cand { ci, prop: name, p, o: 0, src: a.q, rank: *r, d: 0, anchor: a.q, conf: a.conf * 0.95, dist: Some((d, unit, vec![a.q], false)) });
                }
                continue;
            }
            // distribution over descendants of the first anchor that has them
            for a in an {
                let subs = desc_of.get(&a.q).map(|v| v.as_slice()).unwrap_or(&[]);
                let (mut vals, mut bounds, mut from) = (Vec::new(), Vec::new(), Vec::new());
                for s in subs {
                    if let Some((v, b, _)) = quant.get(&(*s, p)) {
                        vals.extend(v);
                        bounds.extend(b);
                        from.push(*s);
                    }
                }
                if let Some(d) = dist(&vals, &bounds) {
                    cands.push(Cand { ci, prop: name, p, o: 0, src: a.q, rank: 1, d: 0, anchor: a.q, conf: a.conf * 0.7, dist: Some((d, unit, from, true)) });
                    break;
                }
            }
        }
    }
    let t_walk = t0.elapsed().as_secs_f64();
    // labels
    let mut lab_need: HashSet<u32> = cands.iter().flat_map(|c| [c.o, c.src, c.anchor]).collect();
    lab_need.extend(anchors.iter().flatten().map(|a| a.q));
    for &(_, a, y) in &ctl_lines {
        lab_need.insert(a);
        lab_need.insert(y);
        if let Some(p) = folk_path(&isa, &rk, a, y) {
            lab_need.extend(p);
        }
    }
    lab_need.extend(generic.iter().map(|g| g.0));
    lab_need.remove(&0);
    let info = read_items(dir, &|q| lab_need.contains(&q), &|q| lab_need.contains(&q))?;
    let lab = |q: u32| info.label.get(&q).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| format!("Q{q}"));
    // proofs of the control queries
    let _ = writeln!(rep, "Proofs (folk rule):\n");
    for &(ci, a, y) in &ctl_lines {
        if let Some(p) = folk_path(&isa, &rk, a, y) {
            let _ = writeln!(rep, "- {}: {}", core[ci].lemma, p.iter().map(|q| format!("{} Q{q}", lab(*q))).collect::<Vec<_>>().join(" → "));
        }
    }
    // output TSV
    let mut tsv = String::from("# onto v2: seed candidates from Wikidata (CANDIDATES, not the truth; into the core only after confirmation). Created by `onto wd-walk`.\n");
    tsv.push_str("# confidence = anchor (strict 0.9 · two supports 0.7 · one support or taxon anchor 0.6) × rank (preferred 1 · normal 0.9) × 0.85^steps to the ancestor; numbers: own ×0.95, distribution over descendants ×0.7\n");
    tsv.push_str("# concept\tdomain\tsynset\tprop\tvalue\tsource\tconfidence\tmean\tsd\tmin\tmax\tn\tunit\n");
    let rank_s = |r: u8| match r {
        2 => "preferred",
        1 => "normal",
        _ => "deprecated",
    };
    for c in &cands {
        let k = &core[c.ci];
        if k.domain == "control" {
            continue;
        }
        let syn = k.synset.map(|s| g.cid_str(s)).unwrap_or_default();
        let (value, source, nums) = match &c.dist {
            None => (
                format!("Q{} {}", c.o, lab(c.o)),
                if c.d == 0 { format!("Q{} {} P{} {}", c.src, lab(c.src), c.p, rank_s(c.rank)) } else { format!("Q{} {} P{} {} ← anchor Q{} {} ({} steps)", c.src, lab(c.src), c.p, rank_s(c.rank), c.anchor, lab(c.anchor), c.d) },
                String::from("\t\t\t\t\t"),
            ),
            Some((d, unit, from, sub)) => {
                let v = if d.n == 1 { format!("point estimate {} {unit}, σ unknown", sig(d.mean)) } else { format!("≈ {} {unit} ± {} (n={}, {}–{})", sig(d.mean), sig(d.sd.unwrap_or(0.0)), d.n, sig(d.min), sig(d.max)) };
                let src = if *sub {
                    let mut s = format!("distribution over {} descendants of Q{} {} (P{}): ", from.len(), c.anchor, lab(c.anchor), c.p);
                    s.push_str(&from.iter().take(4).map(|q| format!("Q{q}")).collect::<Vec<_>>().join(", "));
                    if from.len() > 4 {
                        s.push_str(&format!(" +{}", from.len() - 4));
                    }
                    s
                } else {
                    format!("Q{} {} P{} {}", c.src, lab(c.src), c.p, rank_s(c.rank))
                };
                (v, src, format!("{}\t{}\t{}\t{}\t{}\t{unit}", sig(d.mean), d.sd.map(sig).unwrap_or_default(), sig(d.min), sig(d.max), d.n))
            }
        };
        let _ = writeln!(tsv, "{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{}", k.lemma, k.domain, syn, c.prop, value.replace('\t', " "), source.replace('\t', " "), c.conf, nums);
    }
    std::fs::write(out_tsv, &tsv)?;
    // report: coverage, properties, examples
    let real: Vec<usize> = (0..core.len()).filter(|&i| core[i].domain != "control").collect();
    let with_syn = real.iter().filter(|&&i| core[i].synset.is_some()).count();
    let with_anchor = real.iter().filter(|&&i| !anchors[i].is_empty()).count();
    let _ = writeln!(rep, "\n## Core: concepts → anchors\n\n- concepts in `data/core.tsv`: {}; synset found: {} (in the domain lexfile — {}); with anchor Q: **{}**", real.len(), with_syn, real.iter().filter(|&&i| core[i].lf_ok).count(), with_anchor);
    let mut how: std::collections::BTreeMap<&str, usize> = Default::default();
    for &i in &real {
        for a in &anchors[i] {
            *how.entry(a.how).or_default() += 1;
        }
    }
    let _ = writeln!(rep, "- anchors: {}", how.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(" · "));
    let _ = writeln!(rep, "- without an anchor: {}\n", real.iter().filter(|&&i| anchors[i].is_empty()).map(|&i| core[i].lemma.as_str()).collect::<Vec<_>>().join(", "));
    let _ = writeln!(rep, "- too general ancestors (≥ {generic_cap} descendants; not inherited from; in parentheses — how many of {n_with} anchored concepts have them): {}", generic.iter().take(30).map(|&(x, c)| format!("{} Q{x} ({c})", lab(x))).collect::<Vec<_>>().join(", ") + &if generic.len() > 30 { format!(" … {} more", generic.len() - 30) } else { String::new() });
    let _ = writeln!(rep, "- dropped: «part» that is a taxon or the concept itself (members, subspecies: fox P527 red fox; Cetacea P527 whale) — {dropped_taxon_part}; confidence < 0.4 — {dropped_low}\n");
    let _ = writeln!(rep, "## Candidates by property\n\n| property | candidates | concepts | confidence ≥ 0.7 |\n|---|---:|---:|---:|");
    let mut by_prop: std::collections::BTreeMap<&str, (usize, HashSet<usize>, usize)> = Default::default();
    for c in cands.iter().filter(|c| core[c.ci].domain != "control") {
        let e = by_prop.entry(c.prop).or_default();
        e.0 += 1;
        e.1.insert(c.ci);
        e.2 += (c.conf >= 0.7) as usize;
    }
    for (k, (n, cs, hi)) in &by_prop {
        let _ = writeln!(rep, "| {k} | {n} | {} | {hi} |", cs.len());
    }
    let total = cands.iter().filter(|c| core[c.ci].domain != "control").count();
    let _ = writeln!(rep, "| **total** | **{total}** | {} | |\n", cands.iter().filter(|c| core[c.ci].domain != "control").map(|c| c.ci).collect::<HashSet<_>>().len());
    let _ = writeln!(rep, "- numbers: {q_rows} values in known units; skipped in unknown units — {q_skip}, birth weight — {q_birth}");
    let _ = writeln!(rep, "- time: graph and gate {t_graph:.0} s, statements {:.0} s, walker and numbers {:.0} s, labels {:.0} s\n", t_scan - t_graph, t_walk - t_scan, t0.elapsed().as_secs_f64() - t_walk);
    let _ = writeln!(rep, "Output: `{}`", out_tsv.display());
    Ok(WalkOut { report: rep, cands, controls_ok: controls_ok && gate_ok })
}

#[cfg(test)]
mod t {
    use super::*;

    /// Synthetic graph of whale and fish: whale —P279→ Cetacea —P171→ Whippomorpha —P171→ Mammalia[class]
    /// —P171→ Mammaliaformes —P171→ Sarcopterygii[class] —P171→ Osteichthyes[superclass] —P279→ fish[class];
    /// bypass: Whippomorpha —P171→ Mammaliaformes; Mammalia —P279→ Vertebrata[subphylum];
    /// pike —P171→ Actinopterygii[class] —P171→ Osteichthyes; shark —P279→ fish, —P171→ Chondrichthyes[class]
    /// —P171→ Gnathostomata; bat —P171→ Mammalia.
    fn world() -> (IsaGraph, HashMap<u32, u32>) {
        let e = |s: u32, o: u32, p: u16| (s, o, p);
        let edges = vec![
            e(1, 2, P279),
            e(2, 8, P171),
            e(8, 3, P171),
            e(8, 4, P171),
            e(3, 4, P171),
            e(4, 5, P171),
            e(5, 6, P171),
            e(6, 7, P279),
            e(3, 9, P279),
            e(10, 11, P171),
            e(11, 6, P171),
            e(12, 7, P279),
            e(12, 13, P171),
            e(13, 14, P171),
            e(15, 3, P171),
            e(7, 9, P279),
            // Cetacea —P279→ Mammalia directly (shorter than the P171 chain via Whippomorpha)
            e(2, 3, P279),
        ];
        let rank = HashMap::from([(3, CLASS), (5, CLASS), (6, SUPERCLASS), (7, CLASS), (9, 1153785), (11, CLASS), (13, CLASS), (14, 2361851)]);
        (IsaGraph::from_edges(edges), rank)
    }

    #[test]
    fn whale_is_mammal_not_fish() {
        let (g, rank) = world();
        let rk = rank_kinds(&g, &rank);
        assert_eq!(folk_isa(&g, &rk, 1, 3, true), Some(2), "whale is a mammal, by the shortest path (relaxation)");
        assert!(folk_isa(&g, &rk, 1, 9, true).is_some(), "whale is a vertebrate (P279 from the class)");
        assert!(folk_isa(&g, &rk, 1, 7, true).is_none(), "whale is not a fish");
        assert!(folk_isa(&g, &rk, 1, 4, true).is_none(), "bypass clade above the class is dropped");
        // negative control: without the rule, cladistics leads the whale into fish
        assert!(folk_isa(&g, &rk, 1, 7, false).is_some(), "the control must turn red");
    }

    #[test]
    fn fish_grade_and_shark_and_bat() {
        let (g, rank) = world();
        let rk = rank_kinds(&g, &rank);
        assert!(folk_isa(&g, &rk, 10, 7, true).is_some(), "pike is a fish via the grade class → superclass");
        assert!(folk_isa(&g, &rk, 12, 7, true).is_some(), "shark is a fish (P279)");
        assert!(folk_isa(&g, &rk, 12, 14, true).is_none(), "shark does not climb from class Chondrichthyes");
        assert!(folk_isa(&g, &rk, 15, 3, true).is_some(), "bat is a mammal");
        assert!(folk_isa(&g, &rk, 15, 7, true).is_none(), "bat is not a fish");
        assert!(folk_isa(&g, &rk, 3, 7, true).is_none(), "mammals are not fish");
        assert_eq!(folk_path(&g, &rk, 10, 7), Some(vec![10, 11, 6, 7]));
    }

    #[test]
    fn anchors_and_units() {
        let r = |q: u32, st: &str, src: &str| AlignRow { q, status: st.into(), sources: src.into(), n_src: n_sources_str(src), d: 3 };
        assert_eq!(n_sources_str("ili+wn"), 1);
        assert_eq!(n_sources_str("wn+label"), 2);
        assert_eq!(n_sources_str("lex+ili+wn+label"), 3);
        // crow: a common name without a taxon → also a taxon anchor (genus is better than species)
        let rows = vec![r(388450, "accepted", "ili+wn+label"), r(26198, "single", "label"), r(43365, "single", "label")];
        let taxa = [26198u32, 43365];
        let rank = |q: u32| match q {
            26198 => Some(7432),
            43365 => Some(34740),
            _ => None,
        };
        let a = pick_anchors(&rows, true, &|q| taxa.contains(&q), &|_| false, &rank);
        assert_eq!(a.iter().map(|x| x.q).collect::<Vec<_>>(), vec![388450, 43365]);
        // without strict — two supports
        let rows2 = vec![r(5, "unsupported", "wn+label"), r(6, "single", "label")];
        assert_eq!(pick_anchors(&rows2, false, &|_| false, &|_| false, &|_| None)[0].q, 5);
        assert_eq!(unit_factor(2067, 41803), Some(1e-3));
        assert_eq!(unit_factor(2067, 483261), None, "dalton is not for the core");
        let d = dist(&[100.0, 200.0], &[50.0]).unwrap();
        assert_eq!((d.n, d.mean, d.min, d.max), (2, 150.0, 50.0, 200.0));
        assert!(dist(&[3.0], &[]).unwrap().sd.is_none(), "one value — σ unknown");
        assert_eq!(sig(188.4), "188");
        assert_eq!(sig(0.01234), "0.0123");
    }
}
