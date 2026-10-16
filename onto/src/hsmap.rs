//! hsmap (26.09): HS22 → world model. Word → OEWN synset (WSD v1 with the heading context; lexfile preference by
//! the goods' state; the chapter domain — in a second pass) → core concept → Wikidata Q (strict v2 alignment), every choice
//! with a reason. Gates: IsA contradiction (HS "goods ⊂ heading head" vs our graph), feature vs core, thresholds
//! of neighbouring subheadings. New concepts are candidates. `onto hs describe <code>`. A blind Opus judge.
//!
//! Only derivatives go into `data/hs/`: codes, lemmas and concept name phrases, synsets, Qs, features. No WCO description texts there.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::hs::{self, Kind, Np, Parsed};
use crate::kb::*;
use crate::model::*;
use crate::wsd::Wsd;

/// Bonus for a sense in the lexfile required by the goods' state (live → animal or plant; fresh, frozen… → food).
pub const W_HS: f64 = 1.0;
/// Bonus × share of the lexfile among the chapter's head concepts (second pass) — "one sense per text".
pub const W_CHAP: f64 = 1.0;

pub struct Row {
    pub code: String,
    pub desc: String,
    pub p: Parsed,
}

pub fn load_rows() -> Result<Vec<Row>> {
    let codes = hs::read_codes(&hs::csv_path())?;
    let a = en::annotate::Annotator::load(&hs::model_path())?;
    Ok(codes
        .into_iter()
        .map(|(code, desc)| {
            let t = hs::annotate(&a, &desc);
            let p = hs::parse(&t);
            Row { code, desc, p }
        })
        .collect())
}

/// British spelling → American (OEWN is mostly American): fallback variants of a phrase.
fn us_variants(p: &str) -> Vec<String> {
    const R: &[(&str, &str)] = &[
        ("aluminium", "aluminum"), ("sulph", "sulf"), ("colour", "color"), ("fibre", "fiber"), ("mould", "mold"), ("tyre", "tire"), ("mollusc", "mollusk"),
        ("isation", "ization"), ("ised", "ized"), ("yse", "yze"), ("metre", "meter"), ("litre", "liter"), ("centre", "center"), ("jewellery", "jewelry"),
        ("grey", "gray"), ("plough", "plow"), ("oesophag", "esophag"), ("haem", "hem"), ("vapour", "vapor"), ("odour", "odor"), ("programme", "program"),
        ("catalogue", "catalog"), ("pyjama", "pajama"), ("waggon", "wagon"), ("anaesthe", "anesthe"), ("caesium", "cesium"), ("oestr", "estr"),
        ("foetus", "fetus"), ("woollen", "woolen"), ("defence", "defense"), ("licence", "license"), ("moulding", "molding"), ("kerosene", "kerosine"),
        ("liquorice", "licorice"), ("yoghurt", "yogurt"), ("sulphur", "sulfur"), ("coloured", "colored"), ("savoury", "savory"), ("flavour", "flavor"),
    ];
    let mut v: Vec<String> = Vec::new();
    for (a, b) in R {
        if p.contains(a) {
            let x = p.replace(a, b);
            if !v.contains(&x) {
                v.push(x);
            }
        }
    }
    // «cuttle fish» → «cuttlefish»
    if p.matches(' ').count() == 1 {
        v.push(p.replace(' ', ""));
    }
    // «seabass» → «sea bass», «seabream» → «sea bream»
    for pre in ["sea", "fish", "cod"] {
        if let Some(rest) = p.strip_prefix(pre).filter(|r| r.len() > 3 && !r.starts_with(' ')) {
            v.push(format!("{pre} {rest}"));
        }
    }
    v
}

/// Sense candidates of a phrase: WSD scores with an explanation of the terms.
#[derive(Clone)]
pub struct Cands {
    pub phrase: String,
    pub scored: Vec<(Cid, f64, String)>,
}

/// The longest phrase present in OEWN; if the goods' state requires a lexfile (food, animal) and the longer phrase has no such sense,
/// a shorter one that does ("fatty liver" is a disease, "liver" is food).
fn candidates(w: &Wsd, np: &Np, ctx: &str, prefer: &[u8]) -> Option<Cands> {
    let g = &w.kb.g;
    let mut first: Option<Cands> = None;
    for ph in np.phrases() {
        let mut vars = vec![ph.clone()];
        vars.extend(us_variants(&ph));
        for v in vars {
            if crate::query::lookup(g, &v, Pos::Noun).is_some() {
                let ch = w.choose(&v, ctx)?;
                let scored = ch
                    .scored
                    .iter()
                    .map(|s| {
                        let ov = if s.overlap.is_empty() { String::new() } else { format!(" [{}]", s.overlap.iter().take(4).cloned().collect::<Vec<_>>().join(", ")) };
                        (s.sense, s.total, format!("Lesk {:.2}{ov} + corpus {:.2} + core {:.2} + domain {:.2}", s.lesk, s.prior, s.core, s.domain))
                    })
                    .collect::<Vec<_>>();
                // "food" also accepts an animal or plant (sea cucumber is an echinoderm, not a cucumber)
                let food = lexfile_id("noun.food").expect("lexfile");
                let org = [lexfile_id("noun.animal").expect("lexfile"), lexfile_id("noun.plant").expect("lexfile")];
                let fits = prefer.is_empty() || scored.iter().any(|x| { let l = g.lexfile[x.0 as usize]; prefer.contains(&l) || (prefer.contains(&food) && org.contains(&l)) });
                let cs = Cands { phrase: v, scored };
                if fits {
                    return Some(cs);
                }
                if first.is_none() {
                    first = Some(cs);
                }
                break;
            }
        }
    }
    first
}

#[derive(Clone, Debug)]
pub struct Pick {
    pub phrase: String,
    pub c: Cid,
    pub margin: f64,
    pub second: Option<Cid>,
    pub n: usize,
    pub why: String,
}

fn pick(g: &Graph, cs: &Cands, prefer: &[u8], chap: Option<&HashMap<u8, f64>>) -> Pick {
    let v: Vec<(Cid, f64, String)> = cs
        .scored
        .iter()
        .map(|(c, t, e)| {
            let lf = g.lexfile[*c as usize];
            let hb = if prefer.contains(&lf) { W_HS } else { 0.0 };
            let cb = chap.and_then(|m| m.get(&lf)).map(|s| W_CHAP * s).unwrap_or(0.0);
            let mut e = e.clone();
            if hb > 0.0 {
                e.push_str(&format!(" + HS state {hb:.2}"));
            }
            if cb > 0.0 {
                e.push_str(&format!(" + chapter {cb:.2}"));
            }
            (*c, t + hb + cb, e)
        })
        .collect();
    let best = (0..v.len()).fold(0, |b, i| if v[i].1 > v[b].1 + 1e-9 { i } else { b });
    let second = (0..v.len()).filter(|&i| i != best).max_by(|&a, &b| v[a].1.total_cmp(&v[b].1).then(b.cmp(&a)));
    let margin = second.map(|s| v[best].1 - v[s].1).unwrap_or(9.99);
    let why = format!("{} = {:.2}; margin {:.2}{}", v[best].2, v[best].1, margin.min(9.99), second.map(|s| format!(" over «{}» ({})", g.label(v[s].0), g.lexfile_name(v[s].0))).unwrap_or_else(|| ", the only sense".into()));
    Pick { phrase: cs.phrase.clone(), c: v[best].0, margin, second: second.map(|s| v[s].0), n: v.len(), why }
}

/// Lexfiles required by the goods' state.
fn prefer(p: &Parsed) -> Vec<u8> {
    let lf = |s: &str| lexfile_id(s).expect("lexfile");
    let pos: Vec<&str> = p.feats.iter().filter(|f| !f.neg && matches!(f.kind, Kind::State | Kind::Process)).map(|f| f.val.as_str()).collect();
    if pos.contains(&"live") {
        return vec![lf("noun.animal"), lf("noun.plant")];
    }
    const FOOD: &[&str] = &["fresh", "chilled", "frozen", "dried", "edible", "roasted", "salted", "smoked", "cooked", "preserved", "prepared", "sweetened", "in brine", "in vinegar", "in syrup", "in oil", "decaffeinated", "skimmed", "condensed"];
    if pos.iter().any(|x| FOOD.contains(x)) {
        return vec![lf("noun.food")];
    }
    Vec::new()
}

/// LLM confirmation key: chapter and state class ("03/live", "03/food"): live and frozen fish are not merged.
fn ckey(code: &str, p: &Parsed) -> String {
    let pf = prefer(p);
    let tag = if pf.contains(&lexfile_id("noun.animal").expect("lexfile")) {
        "/live"
    } else if pf.contains(&lexfile_id("noun.food").expect("lexfile")) {
        "/food"
    } else {
        ""
    };
    format!("{}{tag}", &code[..2])
}

/// "of X": material (substance, fibre, metal, fabric…), source (organism, food) or other.
pub fn of_class(kb: &Kb, c: Cid) -> &'static str {
    let g = &kb.g;
    let lf = g.lexfile_name(c);
    if lf == "noun.substance" {
        return "material";
    }
    const MAT: &[&str] = &["material", "substance", "matter", "fabric", "textile", "fiber", "fibre", "metal", "plastic", "wood", "alloy", "mineral", "paper", "glass", "rubber", "leather", "yarn", "thread", "stone", "chemical compound"];
    const SRC: &[&str] = &["organism", "animal", "plant", "person", "fish", "bird", "mammal", "fruit", "vegetable", "food", "foodstuff"];
    let anc = kb.ancestors(c);
    let labels: Vec<String> = anc.map.keys().map(|&a| g.label(a)).collect();
    if labels.iter().any(|l| MAT.contains(&l.as_str())) {
        return "material";
    }
    if matches!(lf, "noun.animal" | "noun.plant" | "noun.food") || labels.iter().any(|l| SRC.contains(&l.as_str())) {
        return "source";
    }
    "other"
}

/// Wikidata Q of a synset: only those accepted by the strict v2 alignment (`accepted`).
pub fn load_q(path: &Path) -> HashMap<String, u32> {
    let mut m = HashMap::new();
    for l in std::fs::read_to_string(path).unwrap_or_default().lines() {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() >= 3 && f[2] == "accepted" {
            if let Ok(q) = f[1].parse() {
                m.entry(f[0].to_string()).or_insert(q);
            }
        }
    }
    m
}

pub fn q_path() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/onto2-2026-09-26/synset_q-v2.tsv")
}

/// Verdict of the IsA gate: HS says "goods ⊂ head".
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum IsaV {
    Yes,
    /// yes, but via a different sense of the goods than WSD chose
    OtherSense,
    /// the goods are a part of the head (fish liver), not a kind
    Part,
    /// the goods are a taxon group (order, family): its members are kinds of the head (parrot ∈ Psittaciformes, parrot ⊂ bird)
    Taxon,
    /// sisters: the common ancestor is the head's immediate parent (tongue and offal under "variety meat")
    Near,
    /// the head is a material, the goods are made of it ("Copper: rivets" — copper rivets): not IsA but "made of"
    Made,
    /// the goods are a kind of the head's modifier ("Mollusc preparations: mussels" — mussel ⊂ mollusc)
    Modifier,
    /// the goods are the source organism of the head's product ("Meat: … mammals" — meat of mammals)
    Source,
    No,
    Unknown,
}

impl IsaV {
    pub fn name(self) -> &'static str {
        match self {
            IsaV::Yes => "yes",
            IsaV::OtherSense => "other-sense",
            IsaV::Part => "part",
            IsaV::Taxon => "taxon",
            IsaV::Near => "near",
            IsaV::Made => "made-of",
            IsaV::Modifier => "modifier",
            IsaV::Source => "source",
            IsaV::No => "no",
            IsaV::Unknown => "unknown",
        }
    }
}

fn label_chain(kb: &Kb, ch: &Chain) -> String {
    let g = &kb.g;
    let mut v = vec![g.label(ch.start)];
    v.extend(ch.steps.iter().map(|s| g.label(s.to)));
    v.join(" → ")
}

/// IsA gate: the chosen goods sense xc, all senses of the goods phrase xs, the heads' senses (all) and the chosen ones.
pub(crate) fn isa_verdict(kb: &Kb, xc: Cid, xs: &[Cid], heads_all: &[Cid], heads_chosen: &[Cid], mods: &[Cid]) -> (IsaV, String) {
    let g = &kb.g;
    let anc = kb.ancestors(xc);
    let lf = |c: Cid| g.lexfile_name(c);
    for &h in heads_all {
        if let Some(ch) = anc.chain(h) {
            return (IsaV::Yes, label_chain(kb, &ch));
        }
    }
    for &x in xs.iter().filter(|x| **x != xc) {
        let a = kb.ancestors(x);
        for &h in heads_all {
            if let Some(ch) = a.chain(h) {
                return (IsaV::OtherSense, format!("other sense: {} ({}): {}", g.label(x), g.lexfile_name(x), label_chain(kb, &ch)));
            }
        }
    }
    for &h in heads_chosen {
        for (p, rel, owner, _) in crate::query::parts_of(kb, h) {
            if p == xc || anc.contains(p) {
                return (IsaV::Part, format!("{} —{}→ {} (from {})", g.label(xc), rel.name(), g.label(h), g.label(owner.end())));
            }
        }
    }
    // taxon group: members of the group (MemberOf) are kinds of the head
    for e in g.in_edges(xc).iter().filter(|e| e.rel == Rel::MemberOf).take(60) {
        let a = kb.ancestors(e.to);
        for &h in heads_all {
            if let Some(ch) = a.chain(h) {
                return (IsaV::Taxon, format!("taxon group: {} ∈ {}; {}", g.label(e.to), g.label(xc), label_chain(kb, &ch)));
            }
        }
    }
    // modifier of the head: "Mollusc preparations: mussels" → mussel ⊂ mollusc
    for &mc in mods {
        if let Some(ch) = anc.chain(mc) {
            return (IsaV::Modifier, format!("kind of head modifier: {}", label_chain(kb, &ch)));
        }
    }
    let Some(&h0) = heads_chosen.first() else { return (IsaV::Unknown, "head not mapped".into()) };
    // the head is a material, the goods are a product, form or derivative ("Copper: rivets", "Aluminium: powders", "Antibiotics: salts")
    const GENERIC: &[&str] = &["article", "waste", "scrap", "powder", "flake", "product", "good", "ware", "residue", "salt", "ester", "derivative", "compound", "mixture", "preparation"];
    let generic = xs.iter().chain(std::iter::once(&xc)).any(|&x| g.members(x).iter().any(|w| GENERIC.contains(&g.word(*w)))) && GENERIC.iter().any(|w| g.label(xc).starts_with(w));
    let artifact = lf(xc) == "noun.artifact" || kb.ancestors(xc).map.keys().any(|&a| g.label(a) == "artifact");
    let head_material = heads_chosen.iter().any(|&h| of_class(kb, h) == "material");
    if head_material && (generic || artifact) {
        return (IsaV::Made, format!("{} (product) made of material «{}»", g.label(xc), g.label(h0)));
    }
    // the goods are an organism, the head is a product (food, substance, article)
    if xs.iter().chain(std::iter::once(&xc)).any(|&x| matches!(lf(x), "noun.animal" | "noun.plant")) && heads_chosen.iter().any(|&h| matches!(lf(h), "noun.food" | "noun.substance")) {
        return (IsaV::Source, format!("{} ({}) — source of product «{}» ({})", g.label(xc), lf(xc), g.label(h0), lf(h0)));
    }
    let ah = kb.ancestors(h0);
    let lca = anc.map.keys().filter(|c| ah.contains(**c)).min_by_key(|c| (anc.depth(**c).unwrap_or(0) + ah.depth(**c).unwrap_or(0), **c)).copied();
    match lca {
        Some(l) if ah.depth(l) == Some(1) => (IsaV::Near, format!("sisters: {} ({}) and {} under «{}»", g.label(xc), g.lexfile_name(xc), g.label(h0), g.label(l))),
        Some(l) => (IsaV::No, format!("{} ({}) and {} ({}) diverge under «{}»", g.label(xc), g.lexfile_name(xc), g.label(h0), g.lexfile_name(h0), g.label(l))),
        None => (IsaV::No, "no common ancestor".into()),
    }
}

/// A mapped noun group of a feature.
#[derive(Clone)]
pub struct MNp {
    pub kind: Kind,
    pub level: u8,
    pub neg: bool,
    pub np: Np,
    /// the group from parsing (before compounding with the head) — the key for matching with features
    pub orig: Np,
    pub pick: Option<Pick>,
    pub all: Vec<Cid>,
}

/// Build result.
pub struct Built {
    pub rows: Vec<Row>,
    pub m: Vec<Vec<MNp>>,
    pub main: Vec<Option<usize>>,
    pub isa: Vec<Option<(IsaV, String, String, String)>>,
    pub gates: Vec<(String, &'static str, String, String)>,
    pub report: String,
}

fn fnv(s: &str) -> u64 {
    s.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3))
}

fn np_kinds(k: Kind) -> bool {
    matches!(k, Kind::Head | Kind::Item | Kind::Of | Kind::Part | Kind::Has)
}

/// Build: parsing, mapping, gates, controls; `data/hs/*.tsv` and a report in `runs`.
pub fn build(kb: &Kb, out: &Path, runs: &Path, confirm: &HashMap<(String, String), String>) -> Result<Built> {
    let g = &kb.g;
    let t0 = std::time::Instant::now();
    let rows = load_rows()?;
    let t_parse = t0.elapsed().as_secs_f64();
    let tree = hs::Tree::new(&rows.iter().map(|r| (r.code.clone(), String::new())).collect::<Vec<_>>());
    // heading context: content words of its subheadings' descriptions (in memory, not in the repo)
    let mut hctx: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for r in &rows {
        let e = hctx.entry(r.code[..4].to_string()).or_default();
        for w in crate::wsd::content(&r.desc) {
            if !e.contains(&w) {
                e.push(w);
            }
        }
    }
    let hctx: HashMap<String, String> = hctx.into_iter().map(|(k, v)| (k, v.join(" "))).collect();
    let w = Wsd::new(kb, crate::wsd::Prior::load(&crate::wsd::prior_path())?);
    let qmap = load_q(&q_path());
    // candidates: (heading phrase, heading) → Cands
    let mut cache: HashMap<(String, String, Vec<u8>), Option<Cands>> = HashMap::new();
    let mut m: Vec<Vec<MNp>> = Vec::new();
    for r in &rows {
        let hd = r.code[..4].to_string();
        let mut v = Vec::new();
        let head0 = r.p.of(Kind::Head).next().and_then(|h| h.np.as_ref()).map(|n| n.head.clone());
        for f in r.p.feats.iter().filter(|f| np_kinds(f.kind) && f.np.is_some()) {
            let mut np = f.np.clone().expect("np");
            // "Lighters: cigarette" — cigarette lighter, if such a compound exists in OEWN
            if f.kind == Kind::Item {
                if let Some(h) = &head0 {
                    let comp = format!("{} {h}", np.phrases()[0]);
                    if np.head != *h && crate::query::lookup(g, &comp, Pos::Noun).is_some() {
                        let mut mods = np.mods.clone();
                        mods.push(np.head.clone());
                        np = Np { mods, head: h.clone() };
                    }
                }
            }
            let pf = prefer(&r.p);
            let key = (np.show(), hd.clone(), pf.clone());
            let cs = cache.entry(key).or_insert_with(|| candidates(&w, &np, &hctx[&hd], &pf)).clone();
            let all = cs.as_ref().map(|c| c.scored.iter().map(|x| x.0).collect()).unwrap_or_default();
            v.push(MNp { kind: f.kind, level: f.level, neg: f.neg, np, orig: f.np.clone().expect("np"), pick: None, all });
        }
        m.push(v);
    }
    let main_of = |p: &Parsed, v: &[MNp]| -> Option<usize> {
        let mf = p.main()?;
        v.iter().position(|x| x.kind == mf.kind && x.level == mf.level && Some(&x.orig) == mf.np.as_ref())
    };
    let main: Vec<Option<usize>> = rows.iter().zip(&m).map(|(r, v)| main_of(&r.p, v)).collect();
    let prefs: Vec<Vec<u8>> = rows.iter().map(|r| prefer(&r.p)).collect();
    // pass 1: without the chapter domain → distribution of lexfiles of head concepts and heads in the chapter
    let mut chap: HashMap<String, HashMap<u8, f64>> = HashMap::new();
    for (i, r) in rows.iter().enumerate() {
        for (j, x) in m[i].iter().enumerate() {
            if Some(j) != main[i] && x.kind != Kind::Head {
                continue;
            }
            if let Some(cs) = cache.get(&(x.np.show(), r.code[..4].to_string(), prefs[i].clone())).and_then(|c| c.as_ref()) {
                let pk = pick(g, cs, &prefs[i], None);
                *chap.entry(r.code[..2].to_string()).or_default().entry(g.lexfile[pk.c as usize]).or_default() += 1.0;
            }
        }
    }
    for d in chap.values_mut() {
        let z: f64 = d.values().sum();
        for x in d.values_mut() {
            *x /= z;
        }
    }
    // pass 2: with the chapter domain; LLM confirmation overrides (phrase, heading)
    let smap = crate::core::synset_map(g);
    let mut n_confirmed = 0usize;
    for (i, r) in rows.iter().enumerate() {
        let hd = r.code[..4].to_string();
        for x in m[i].iter_mut() {
            if let Some(cs) = cache.get(&(x.np.show(), hd.clone(), prefs[i].clone())).and_then(|c| c.as_ref()) {
                let mut pk = pick(g, cs, &prefs[i], chap.get(&r.code[..2]));
                if let Some(syn) = confirm.get(&(cs.phrase.clone(), ckey(&r.code, &r.p))) {
                    if let Some(c) = crate::core::find(&smap, syn).filter(|c| cs.scored.iter().any(|s| s.0 == *c)) {
                        if c != pk.c {
                            pk.why = format!("LLM (confirmation of a disputed one): {} instead of {}; WSD: {}", g.label(c), g.label(pk.c), pk.why);
                            pk.c = c;
                        } else {
                            pk.why = format!("{}; LLM: yes", pk.why);
                        }
                        n_confirmed += 1;
                    }
                }
                x.pick = Some(pk);
            }
        }
    }
    let t_mapping = t0.elapsed().as_secs_f64();

    // ── IsA gate: HS "goods ⊂ parent" (parent — the goods of the previous level, otherwise the heads) ────────────
    let mut isa: Vec<Option<(IsaV, String, String, String)>> = vec![None; rows.len()];
    let mut isa_pairs: Vec<(usize, Cid, Vec<Cid>, Vec<Cid>, Vec<Cid>, Vec<Cid>)> = Vec::new();
    for i in 0..rows.len() {
        let Some(mi) = main[i] else { continue };
        let x = &m[i][mi];
        if x.kind != Kind::Item {
            continue;
        }
        // parent — the heading's heads (level 0): deeper BACI colons are sometimes a subkind, sometimes a list ("ostriches: emus")
        let parents: Vec<&MNp> = m[i].iter().filter(|y| y.kind == Kind::Head).collect();
        let (Some(xp), false) = (x.pick.as_ref(), parents.is_empty()) else {
            isa[i] = Some((IsaV::Unknown, x.np.show(), parents.iter().map(|p| p.np.show()).collect::<Vec<_>>().join(" | "), "item or parent not mapped".into()));
            continue;
        };
        let hall: Vec<Cid> = parents.iter().flat_map(|p| p.all.iter().copied()).collect();
        let hch: Vec<Cid> = parents.iter().filter_map(|p| p.pick.as_ref().map(|k| k.c)).collect();
        if hch.is_empty() || parents.iter().any(|p| p.np == x.np) {
            isa[i] = Some((IsaV::Unknown, x.np.show(), parents.iter().map(|p| p.np.show()).collect::<Vec<_>>().join(" | "), if hch.is_empty() { "parent not mapped".into() } else { "item = parent".into() }));
            continue;
        }
        // noun modifiers of the head (all senses): "mollusc" in "mollusc preparation"
        let mods: Vec<Cid> = parents.iter().flat_map(|p| p.np.mods.iter()).filter_map(|mo| crate::query::lookup(g, mo, Pos::Noun)).flat_map(|f| f.senses.into_iter().take(3)).collect();
        let (v, why) = isa_verdict(kb, xp.c, &x.all, &hall, &hch, &mods);
        isa[i] = Some((v, x.np.show(), parents.iter().map(|p| p.np.show()).collect::<Vec<_>>().join(" | "), why));
        isa_pairs.push((i, xp.c, x.all.clone(), hall, hch, mods));
    }
    // negative control of the IsA gate: goods shuffled between headings (shift by half the list)
    let mut ctl_n: BTreeMap<IsaV, usize> = BTreeMap::new();
    let n_pairs = isa_pairs.len();
    for k in 0..n_pairs {
        let (_, xc, ref xs, _, _, _) = isa_pairs[(k + n_pairs / 2) % n_pairs.max(1)];
        let (_, _, _, ref hall, ref hch, ref mods) = isa_pairs[k];
        *ctl_n.entry(isa_verdict(kb, xc, xs, hall, hch, mods).0).or_default() += 1;
    }
    let ctl_no = *ctl_n.get(&IsaV::No).unwrap_or(&0);

    // ── gate "feature vs core and seeds" ──────────────────────────────────────────────────────────
    let mut gates: Vec<(String, &'static str, String, String)> = Vec::new();
    let artifact = lexfile_id("noun.artifact").expect("lexfile");
    for (i, r) in rows.iter().enumerate() {
        let Some(mi) = main[i] else { continue };
        let Some(pk) = m[i][mi].pick.as_ref() else { continue };
        let c = pk.c;
        let st = |v: &str, neg: bool| r.p.feats.iter().any(|f| f.kind == Kind::State && f.val == v && f.neg == neg);
        let purpose_hc = r.p.feats.iter().any(|f| f.kind == Kind::Purpose && f.val.contains("human consumption"));
        let mut check = |p: Prop, want: bool, hs_side: &str| {
            if let PropAnswer::Known { value, win, .. } = kb.prop(c, p, 0) {
                if value != want {
                    let a = &kb.asserts[win];
                    gates.push((r.code.clone(), "prop", format!("{hs_side} ({})", g.label(c)), format!("seed {} {}: {}={} — {}", a.id, g.label(a.concept), p.name(), value, a.source)));
                }
            }
        };
        if st("live", false) {
            check(Prop::Living, true, "HS: live");
        }
        if st("edible", false) || (purpose_hc && !r.p.feats.iter().any(|f| f.kind == Kind::Purpose && f.neg)) {
            check(Prop::Edible, true, "HS: edible");
        }
        if st("inedible", false) || st("edible", true) {
            check(Prop::Edible, false, "HS: inedible");
        }
        let core = crate::core::of_concept(g, c);
        if let Some(id) = core {
            let rec = id.rec();
            for x in m[i].iter().filter(|x| x.kind == Kind::Of && !x.neg) {
                if let Some(xp) = &x.pick {
                    if of_class(kb, xp.c) == "material" && !rec.material.is_applicable() {
                        gates.push((r.code.clone(), "core-material", format!("HS: {} of {}", g.label(c), g.label(xp.c)), format!("core {:?}: material does not apply to domain {}", id, rec.domain.name())));
                    }
                }
            }
            if st("live", false) && matches!(rec.domain, crate::core::Domain::Household | crate::core::Domain::Food) {
                gates.push((r.code.clone(), "core-live", format!("HS: live {}", g.label(c)), format!("core {:?}: domain {}", id, rec.domain.name())));
            }
        }
        if g.lexfile[c as usize] == artifact && ["fresh", "chilled", "frozen", "live"].iter().any(|s| st(s, false)) {
            gates.push((r.code.clone(), "state-artifact", format!("HS: food or live state on {}", g.label(c)), "sense is an artifact (noun.artifact)".into()));
        }
    }

    // ── threshold gate: neighbours with the same other features do not overlap ────────────────────────────
    let sig = |r: &Row| -> String {
        let mut v: Vec<String> = r.p.feats.iter().filter(|f| !matches!(f.kind, Kind::Bound | Kind::Ref)).map(|f| format!("{}:{}:{}", f.kind.name(), f.neg, f.val)).collect();
        v.sort();
        v.dedup();
        v.join("|")
    };
    let mut by_hd: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, r) in rows.iter().enumerate() {
        if r.p.feats.iter().any(|f| f.kind == Kind::Bound) {
            by_hd.entry(&r.code[..4]).or_default().push(i);
        }
    }
    let (mut bound_pairs, mut bound_bad, mut bound_ctl) = (0usize, Vec::new(), 0usize);
    for ids in by_hd.values() {
        for a in 0..ids.len() {
            for b in a + 1..ids.len() {
                let (ra, rb) = (&rows[ids[a]], &rows[ids[b]]);
                if sig(ra) != sig(rb) {
                    continue;
                }
                let ba: Vec<&hs::Bound> = ra.p.feats.iter().filter_map(|f| f.bound.as_ref()).collect();
                let bb: Vec<&hs::Bound> = rb.p.feats.iter().filter_map(|f| f.bound.as_ref()).collect();
                let shared: Vec<(&hs::Bound, &hs::Bound)> = ba.iter().flat_map(|x| bb.iter().filter(move |y| y.attr == x.attr && y.unit.base().0 == x.unit.base().0).map(move |y| (*x, *y))).collect();
                if shared.is_empty() {
                    continue;
                }
                bound_pairs += 1;
                // different subheadings with the same other features must differ in at least one threshold
                if shared.iter().all(|(x, y)| x.overlaps(y)) {
                    bound_bad.push((ra.code.clone(), rb.code.clone(), shared.iter().map(|(x, y)| format!("{} ~ {}", x.show(), y.show())).collect::<Vec<_>>().join("; ")));
                }
                // negative control: a copy of the first neighbour's threshold — must overlap
                if shared.iter().all(|(x, _)| x.overlaps(x)) {
                    bound_ctl += 1;
                }
            }
        }
    }
    for (a, b, why) in &bound_bad {
        gates.push((format!("{a}~{b}"), "bounds", "HS: neighbours with the same features".into(), why.clone()));
    }

    // ── new concepts: phrases that are neither in the core nor in the graph ─────────────────────────────────────────────────
    let mut newc: BTreeMap<String, (usize, BTreeMap<&'static str, usize>, Vec<String>)> = BTreeMap::new();
    for (i, r) in rows.iter().enumerate() {
        for x in &m[i] {
            if x.pick.is_none() {
                let e = newc.entry(x.np.show()).or_default();
                e.0 += 1;
                *e.1.entry(x.kind.name()).or_default() += 1;
                if e.2.len() < 5 && !e.2.contains(&r.code) {
                    e.2.push(r.code.clone());
                }
            }
        }
    }
    let t_gates = t0.elapsed().as_secs_f64();

    // ── output files ─────────────────────────────────────────────────────────────────────────────────
    std::fs::create_dir_all(out)?;
    let q_of = |c: Cid| qmap.get(&g.cid_str(c)).map(|q| format!("Q{q}")).unwrap_or_else(|| "-".into());
    let core_of = |c: Cid| crate::core::of_concept(g, c).map(|id| format!("{id:?}")).unwrap_or_else(|| "-".into());
    // tree.tsv: nodes, parents, derived labels (most frequent head concepts)
    let mut labels: HashMap<String, BTreeMap<String, usize>> = HashMap::new();
    for (i, r) in rows.iter().enumerate() {
        if let Some(mi) = main[i] {
            let l = m[i][mi].np.show();
            for k in [&r.code[..2], &r.code[..4]] {
                *labels.entry(k.to_string()).or_default().entry(l.clone()).or_default() += 1;
            }
        }
    }
    let top = |k: &str| -> String {
        let mut v: Vec<(&String, &usize)> = labels.get(k).map(|m| m.iter().collect()).unwrap_or_default();
        v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        v.iter().take(4).map(|x| x.0.as_str()).collect::<Vec<_>>().join(", ")
    };
    let mut s = String::from("# HS22: code tree (section → heading → subheading). Codes are facts; the label is our derivative (most frequent head concepts), not WCO text.\n# code\tlevel\tparent\tchildren\tresidual_code\tlabel\n");
    for (code, l, parent) in &tree.nodes {
        let par = parent.map(|p| tree.nodes[p].0.clone()).unwrap_or_else(|| "-".into());
        let res = if *l == hs::Level::Sub && hs::residual_code(code) { "1" } else { "0" };
        let _ = writeln!(s, "{code}\t{}\t{par}\t{}\t{res}\t{}", l.name(), tree.children(code).len(), top(code));
    }
    std::fs::write(out.join("tree.tsv"), s)?;
    // codes.tsv
    let mut s = String::from("# HS22 → world model: the subheading's head concept (the deepest goods, otherwise the heading head), head, IsA gate.\n# code\tnec\tmain_kind\tmain\tsynset\tlexfile\tcore\tq\thead\thead_synset\tisa\tisa_why\n");
    for (i, r) in rows.iter().enumerate() {
        let (mk, mp, syn, lf, core, q) = match main[i].map(|mi| &m[i][mi]) {
            Some(x) => match &x.pick {
                Some(p) => (x.kind.name(), x.np.show(), g.cid_str(p.c), g.lexfile_name(p.c).to_string(), core_of(p.c), q_of(p.c)),
                None => (x.kind.name(), x.np.show(), "-".into(), "-".into(), "-".into(), "-".into()),
            },
            None => ("-", "-".into(), "-".into(), "-".into(), "-".into(), "-".into()),
        };
        let head = m[i].iter().find(|x| x.kind == Kind::Head);
        let (hp, hs_) = head.map(|h| (h.np.show(), h.pick.as_ref().map(|p| g.cid_str(p.c)).unwrap_or_else(|| "-".into()))).unwrap_or(("-".into(), "-".into()));
        let (iv, iw) = isa[i].as_ref().map(|x| (x.0.name(), x.3.clone())).unwrap_or(("-", String::new()));
        let _ = writeln!(s, "{}\t{}\t{mk}\t{mp}\t{syn}\t{lf}\t{core}\t{q}\t{hp}\t{hs_}\t{iv}\t{iw}", r.code, r.p.nec as u8);
    }
    std::fs::write(out.join("codes.tsv"), s)?;
    // feats.tsv
    let mut s = String::from("# HS22: subheading features. value — an enum word, lemma or name phrase; extra — threshold in base units (exact, math), class \"of X\" and synset.\n# code\tkind\tneg\tlevel\tvalue\textra\n");
    let mut kind_n: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    for (i, r) in rows.iter().enumerate() {
        let mut seen: HashSet<(Kind, bool, String)> = HashSet::new();
        for f in &r.p.feats {
            if !seen.insert((f.kind, f.neg, f.val.clone())) {
                continue;
            }
            let e = kind_n.entry(f.kind.name()).or_default();
            e.0 += 1;
            e.1.insert(r.code.clone());
            let extra = if let Some(b) = &f.bound {
                b.show()
            } else if let Some(x) = m[i].iter().find(|x| x.kind == f.kind && Some(&x.orig) == f.np.as_ref()) {
                match &x.pick {
                    Some(p) if f.kind == Kind::Of => format!("{} {}", of_class(kb, p.c), g.cid_str(p.c)),
                    Some(p) => g.cid_str(p.c),
                    None => "unknown".into(),
                }
            } else {
                String::new()
            };
            let _ = writeln!(s, "{}\t{}\t{}\t{}\t{}\t{}", r.code, f.kind.name(), f.neg as u8, f.level, f.val, extra);
        }
    }
    std::fs::write(out.join("feats.tsv"), s)?;
    // terms.tsv: phrase → sense (most frequent), how many times, reason
    let mut terms: BTreeMap<String, (usize, BTreeMap<&'static str, usize>, BTreeMap<Cid, usize>, Option<Pick>)> = BTreeMap::new();
    for v in &m {
        for x in v {
            let e = terms.entry(x.np.show()).or_default();
            e.0 += 1;
            *e.1.entry(x.kind.name()).or_default() += 1;
            if let Some(p) = &x.pick {
                *e.2.entry(p.c).or_default() += 1;
                if e.3.as_ref().is_none_or(|q| q.margin > p.margin) {
                    e.3 = Some(p.clone());
                }
            }
        }
    }
    let mut s = String::from("# HS22: word → world-model concept. synset — the phrase's most frequent sense (WSD by heading context); senses — how many different senses in different headings; why — score terms for the most disputed use.\n# phrase\tcount\tkinds\tfound_as\tsynset\tlexfile\tcore\tq\tsenses\tmin_margin\twhy\n");
    let (mut t_all, mut t_map, mut occ_all, mut occ_map) = (0usize, 0usize, 0usize, 0usize);
    for (ph, (n, kinds, senses, pk)) in &terms {
        t_all += 1;
        occ_all += n;
        let kinds = kinds.iter().map(|(k, n)| format!("{k}:{n}")).collect::<Vec<_>>().join(",");
        match (senses.iter().max_by_key(|x| (x.1, std::cmp::Reverse(*x.0))), pk) {
            (Some((&c, _)), Some(p)) => {
                t_map += 1;
                occ_map += n;
                let _ = writeln!(s, "{ph}\t{n}\t{kinds}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{}", p.phrase, g.cid_str(c), g.lexfile_name(c), core_of(c), q_of(c), senses.len(), p.margin.min(9.99), p.why);
            }
            _ => {
                let _ = writeln!(s, "{ph}\t{n}\t{kinds}\t-\t-\t-\t-\t-\t0\t-\tneither in the core nor in the graph");
            }
        }
    }
    std::fs::write(out.join("terms.tsv"), s)?;
    let mut s = String::from("# HS22: new candidate concepts — phrases that are neither in the core nor in the OEWN 2025+ graph (with British variants). Not truth: candidates.\n# phrase\tcount\tkinds\tcodes\n");
    let mut nv: Vec<(&String, &(usize, BTreeMap<&'static str, usize>, Vec<String>))> = newc.iter().collect();
    nv.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(b.0)));
    for (ph, (n, kinds, codes)) in &nv {
        let _ = writeln!(s, "{ph}\t{n}\t{}\t{}", kinds.iter().map(|(k, n)| format!("{k}:{n}")).collect::<Vec<_>>().join(","), codes.join(" "));
    }
    std::fs::write(out.join("new-concepts.tsv"), s)?;
    let mut s = String::from("# HS22: contradictions with the world model — IsA (HS \"goods ⊂ head\", different in ours) and features vs core and seeds.\n# code\tgate\ths\tours\n");
    for i in 0..rows.len() {
        if let Some((IsaV::No, x, p, why)) = &isa[i] {
            let _ = writeln!(s, "{}\tisa\t{x} ⊂ {p}\t{why}", rows[i].code);
        }
    }
    for (c, gname, h, o) in &gates {
        let _ = writeln!(s, "{c}\t{gname}\t{h}\t{o}");
    }
    std::fs::write(out.join("contradictions.tsv"), s)?;

    // ── report ──────────────────────────────────────────────────────────────────────────────────────────
    let n = rows.len();
    let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
    let with_main = main.iter().filter(|x| x.is_some()).count();
    let main_mapped = (0..n).filter(|&i| main[i].is_some_and(|mi| m[i][mi].pick.is_some())).count();
    let main_item = (0..n).filter(|&i| main[i].is_some_and(|mi| m[i][mi].kind == Kind::Item)).count();
    let main_core = (0..n).filter(|&i| main[i].and_then(|mi| m[i][mi].pick.as_ref()).is_some_and(|p| crate::core::of_concept(g, p.c).is_some())).count();
    let main_q = (0..n).filter(|&i| main[i].and_then(|mi| m[i][mi].pick.as_ref()).is_some_and(|p| qmap.contains_key(&g.cid_str(p.c)))).count();
    let nec = rows.iter().filter(|r| r.p.nec).count();
    let rescode = rows.iter().filter(|r| hs::residual_code(&r.code)).count();
    let both = rows.iter().filter(|r| r.p.nec && hs::residual_code(&r.code)).count();
    let other_words: usize = rows.iter().map(|r| r.p.unknown_words).sum();
    let content: usize = rows.iter().map(|r| r.p.content_words).sum();
    let mut isa_n: BTreeMap<IsaV, usize> = BTreeMap::new();
    for x in isa.iter().flatten() {
        *isa_n.entry(x.0).or_default() += 1;
    }
    let isa_checked: usize = isa_n.iter().filter(|(k, _)| **k != IsaV::Unknown).map(|x| x.1).sum();
    let mut gate_n: BTreeMap<&'static str, usize> = BTreeMap::new();
    for gt in &gates {
        *gate_n.entry(gt.1).or_default() += 1;
    }
    let mut rep = String::new();
    let _ = writeln!(rep, "# HS22 → world model (onto hs build)\n");
    let _ = writeln!(rep, "Time: `en` parsing {t_parse:.1} s, mapping {:.1} s, gates {:.1} s.\n", t_mapping - t_parse, t_gates - t_mapping);
    let _ = writeln!(rep, "## Tree\n\nchapters {}, headings {}, subheadings {}. \"Other\" by code (…9, …90): {rescode}; textual (n.e.c., other than): {nec}; both: {both}.\n", tree.count(hs::Level::Chapter), tree.count(hs::Level::Heading), tree.count(hs::Level::Sub));
    let _ = writeln!(rep, "## Coverage\n\n| measure | value |\n|---|---:|");
    let _ = writeln!(rep, "| subheadings with a head concept | {with_main} / {n} ({:.1}%) |", pct(with_main, n));
    let _ = writeln!(rep, "| — of them goods (deeper than the head) | {main_item} |");
    let _ = writeln!(rep, "| **head concept mapped to a synset** | **{main_mapped} / {n} ({:.1}%)** |", pct(main_mapped, n));
    let _ = writeln!(rep, "| — core concept | {main_core} ({:.1}%) |", pct(main_core, n));
    let _ = writeln!(rep, "| — with Q (strict v2 alignment) | {main_q} ({:.1}%) |", pct(main_q, n));
    let _ = writeln!(rep, "| terms (distinct phrases) mapped | {t_map} / {t_all} ({:.1}%) |", pct(t_map, t_all));
    let _ = writeln!(rep, "| term uses mapped | {occ_map} / {occ_all} ({:.1}%) |", pct(occ_map, occ_all));
    let _ = writeln!(rep, "| \"unknown\": head concept not mapped | {} ({:.1}%) |", n - main_mapped, pct(n - main_mapped, n));
    let _ = writeln!(rep, "| label words without a class / content words | {other_words} / {content} ({:.1}%) |", pct(other_words, content));
    let _ = writeln!(rep, "| LLM confirmations applied | {n_confirmed} |\n");
    let _ = writeln!(rep, "## Features\n\n| kind | features | subheadings |\n|---|---:|---:|");
    for k in Kind::ALL {
        if let Some((a, b)) = kind_n.get(k.name()) {
            let _ = writeln!(rep, "| {} ({}) | {a} | {} |", k.name(), k.uk(), b.len());
        }
    }
    let mut vals: BTreeMap<(&'static str, String), usize> = BTreeMap::new();
    for r in &rows {
        for f in r.p.feats.iter().filter(|f| matches!(f.kind, Kind::State | Kind::Process | Kind::Form | Kind::Purpose)) {
            *vals.entry((f.kind.name(), format!("{}{}", if f.neg { "¬" } else { "" }, f.val))).or_default() += 1;
        }
    }
    for k in ["state", "process", "form", "purpose"] {
        let mut v: Vec<(&String, &usize)> = vals.iter().filter(|x| x.0 .0 == k).map(|x| (&x.0 .1, x.1)).collect();
        v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        let _ = writeln!(rep, "\n- **{k}:** {}", v.iter().take(14).map(|x| format!("{} {}", x.0, x.1)).collect::<Vec<_>>().join(", "));
    }
    let mut ofc: BTreeMap<&'static str, usize> = BTreeMap::new();
    for v in &m {
        for x in v.iter().filter(|x| x.kind == Kind::Of) {
            *ofc.entry(x.pick.as_ref().map(|p| of_class(kb, p.c)).unwrap_or("unknown")).or_default() += 1;
        }
    }
    let _ = writeln!(rep, "- **of X:** {}", ofc.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", "));
    let mut battr: BTreeMap<String, usize> = BTreeMap::new();
    for r in &rows {
        for b in r.p.feats.iter().filter_map(|f| f.bound.as_ref()) {
            *battr.entry(format!("{:?} ({})", b.attr, b.unit.base().0)).or_default() += 1;
        }
    }
    let mut bv: Vec<(&String, &usize)> = battr.iter().collect();
    bv.sort_by(|a, b| b.1.cmp(a.1));
    let _ = writeln!(rep, "- **thresholds:** {}", bv.iter().take(16).map(|x| format!("{} {}", x.0, x.1)).collect::<Vec<_>>().join(", "));
    let _ = writeln!(rep, "\n## Gates\n\n- **IsA** (HS \"goods ⊂ parent\"), checked {isa_checked}: {}.", isa_n.iter().map(|(k, v)| format!("{} {v}", k.name())).collect::<Vec<_>>().join(", "));
    let _ = writeln!(rep, "  - negative control (goods shuffled between headings): \"no\" {ctl_no} / {n_pairs} ({:.1}%) vs {} / {n_pairs} ({:.1}%) on the real ones; control breakdown: {}.", pct(ctl_no, n_pairs), isa_n.get(&IsaV::No).unwrap_or(&0), pct(*isa_n.get(&IsaV::No).unwrap_or(&0), n_pairs), ctl_n.iter().map(|(k, v)| format!("{} {v}", k.name())).collect::<Vec<_>>().join(", "));
    let _ = writeln!(rep, "- **feature vs core and seeds:** {}.", if gate_n.is_empty() { "0".into() } else { gate_n.iter().filter(|x| *x.0 != "bounds").map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", ") });
    let _ = writeln!(rep, "- **neighbour thresholds** (same other features): pairs {bound_pairs}, not distinguished by a threshold {}; the control (a copy of the neighbour's threshold) goes red on {bound_ctl} / {bound_pairs}.", bound_bad.len());
    let _ = writeln!(rep, "\n## New concepts (candidates)\n\nphrases without a concept: {} (uses {}). Most frequent: {}.", nv.len(), nv.iter().map(|x| x.1 .0).sum::<usize>(), nv.iter().take(25).map(|x| format!("{} {}", x.0, x.1 .0)).collect::<Vec<_>>().join(", "));
    let _ = writeln!(rep, "\n## IsA contradictions (first 25)\n");
    for i in (0..n).filter(|&i| matches!(isa[i], Some((IsaV::No, ..)))).take(25) {
        let x = isa[i].as_ref().expect("isa");
        let _ = writeln!(rep, "- {}: {} ⊂ {} — {}", rows[i].code, x.1, x.2, x.3);
    }
    let _ = writeln!(rep, "\n## Features vs core (first 15)\n");
    for gt in gates.iter().filter(|x| x.1 != "bounds").take(15) {
        let _ = writeln!(rep, "- {} [{}]: {} — {}", gt.0, gt.1, gt.2, gt.3);
    }
    let _ = writeln!(rep, "\n## Thresholds: undistinguished neighbours (first 10)\n");
    for (a, b, why) in bound_bad.iter().take(10) {
        let _ = writeln!(rep, "- {a} ~ {b}: {why}");
    }
    std::fs::create_dir_all(runs)?;
    Ok(Built { rows, m, main, isa, gates, report: rep })
}

// ── goods description ───────────────────────────────────────────────────────────────────────────────────────

fn tsv_rows(p: &Path) -> Result<Vec<Vec<String>>> {
    Ok(std::fs::read_to_string(p).with_context(|| format!("{} — run `onto hs build` first", p.display()))?.lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').map(String::from).collect()).collect())
}

/// `onto hs describe <code>`: tree, head concept (sense, why, core, Q), HS IsA and our verdict, OEWN IsA,
/// features, core fields (`describe` v3), sources. The WCO description text — only with `--text` and only from a local file.
pub fn describe(kb: &Kb, code: &str, text: bool) -> Result<String> {
    let g = &kb.g;
    let d = hs::data_dir();
    let codes = tsv_rows(&d.join("codes.tsv"))?;
    let feats = tsv_rows(&d.join("feats.tsv"))?;
    let terms = tsv_rows(&d.join("terms.tsv"))?;
    let tree = tsv_rows(&d.join("tree.tsv"))?;
    let smap = crate::core::synset_map(g);
    let Some(row) = codes.iter().find(|r| r[0] == code) else { bail!("code {code} is not among the HS22 subheadings") };
    let mut s = String::new();
    let lab = |c: &str| tree.iter().find(|r| r[0] == c).map(|r| r[5].clone()).unwrap_or_default();
    let (ch, hd) = (&code[..2], &code[..4]);
    let sibs: Vec<String> = codes.iter().filter(|r| r[0].starts_with(hd) && r[0] != code).map(|r| format!("{} {}", r[0], r[3])).take(6).collect();
    let _ = writeln!(s, "{code} — subheading of heading {hd} [{}] of chapter {ch} [{}]{}", lab(hd), lab(ch), if hs::residual_code(code) { "; \"other\" by code" } else { "" });
    if !sibs.is_empty() {
        let _ = writeln!(s, "  neighbours: {}", sibs.join("; "));
    }
    if text {
        if let Ok(v) = hs::read_codes(&hs::csv_path()) {
            if let Some((_, t)) = v.iter().find(|x| x.0 == code) {
                let _ = writeln!(s, "  description (BACI/UN Comtrade, © WCO; local only): {t}");
            }
        }
    }
    let why_of = |ph: &str| terms.iter().find(|t| t[0] == ph).map(|t| t[10].clone()).unwrap_or_default();
    let main = &row[3];
    let _ = writeln!(s, "head concept ({}): {main}", if row[2] == "item" { "goods" } else { "heading head" });
    match crate::core::find(&smap, &row[4]) {
        Some(c) => {
            let _ = writeln!(s, "  → {} ({}; core: {}; {}) — «{}»", g.show(c), row[5], row[6], if row[7] == "-" { "no Q" } else { &row[7] }, g.gloss_of(c).chars().take(100).collect::<String>());
            let _ = writeln!(s, "  why (WSD v1 + HS features; the phrase's most disputed use): {}", why_of(main));
            let anc = kb.ancestors(c);
            let mut chain = vec![g.label(c)];
            let mut cur = c;
            for _ in 0..12 {
                let Some(st) = kb.up(cur).into_iter().next() else { break };
                cur = st.to;
                chain.push(g.label(cur));
            }
            let _ = anc;
            let _ = writeln!(s, "  is a kind of (OEWN): {}", chain.join(" → "));
        }
        None => {
            let _ = writeln!(s, "  unknown: the phrase is neither in the core nor in the graph — a candidate for new concepts");
        }
    }
    if row[8] != "-" {
        let _ = writeln!(s, "heading head: {} → {}", row[8], crate::core::find(&smap, &row[9]).map(|c| g.show(c)).unwrap_or_else(|| "unknown".into()));
    }
    if row[10] != "-" {
        let v = match row[10].as_str() {
            "yes" => "yes",
            "other-sense" => "yes, but via a different sense",
            "part" => "it is a part, not a kind",
            "no" => "NO — contradiction",
            _ => "unknown",
        };
        let _ = writeln!(s, "HS: {main} ⊂ {} — ours: {v} ({})", row[8], row[11]);
    }
    let mine: Vec<&Vec<String>> = feats.iter().filter(|f| f[0] == code && !matches!(f[1].as_str(), "head" | "item")).collect();
    if !mine.is_empty() {
        let _ = writeln!(s, "features:");
        let mut by: BTreeMap<&str, Vec<String>> = BTreeMap::new();
        for f in mine {
            let v = format!("{}{}{}", if f[2] == "1" { "not " } else { "" }, f[4], if f[5].is_empty() { String::new() } else { format!(" [{}]", f[5]) });
            by.entry(f[1].as_str()).or_default().push(v);
        }
        for (k, v) in by {
            let _ = writeln!(s, "  {}: {}", Kind::parse(k).map(|x| x.uk()).unwrap_or(k), v.join("; "));
        }
    }
    if let Some(dsc) = crate::core::find(&smap, &row[4]).and_then(|c| crate::describe::describe(kb, &g.cid_str(c), None, 0, true)) {
        let lines: Vec<&str> = dsc.text.lines().skip(2).filter(|l| !["unknown", "not apply", "not applicable", "n/a"].iter().any(|w| l.contains(w))).collect();
        if !lines.is_empty() {
            let _ = writeln!(s, "world model (core, then graph; describe v3):");
            for l in lines {
                let _ = writeln!(s, "  {l}");
            }
        }
    }
    let _ = writeln!(s, "sources: code and tree — BACI HS22 V202601 (external trade data); parsing — en (UD, ud-ewt-eslspok); senses — WSD v1 onto; core — onto v3; Q — wd-align v2 (accepted); thresholds — math (exact rationals).");
    Ok(s)
}

// ── blind Opus judge and confirmation of disputed senses ───────────────────────────────────────────────

/// Output for the judge: head concept, sense with a gloss, HS IsA, features — without our scores and verdicts.
pub fn render(kb: &Kb, b: &Built, i: usize) -> String {
    let g = &kb.g;
    let mut s = String::new();
    match b.main[i].map(|mi| &b.m[i][mi]) {
        Some(x) => match &x.pick {
            Some(p) => {
                let _ = write!(s, "main: {} = WordNet «{}» ({}: {})", x.np.show(), g.label(p.c), g.lexfile_name(p.c), g.gloss_of(p.c).chars().take(80).collect::<String>());
            }
            None => {
                let _ = write!(s, "main: {} = (not in WordNet)", x.np.show());
            }
        },
        None => s.push_str("main: (none)"),
    }
    if let Some(h) = b.m[i].iter().find(|x| x.kind == Kind::Head) {
        let _ = write!(s, "; class: {}", h.np.show());
    }
    let mut by: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    let mut seen = HashSet::new();
    for f in &b.rows[i].p.feats {
        if matches!(f.kind, Kind::Head | Kind::Item | Kind::Ref) || !seen.insert((f.kind, f.neg, f.val.clone())) {
            continue;
        }
        let v = match (&f.bound, f.kind) {
            (Some(bd), _) => bd.show(),
            (None, Kind::Of) => {
                let cls = b.m[i].iter().find(|x| x.kind == Kind::Of && Some(&x.orig) == f.np.as_ref()).and_then(|x| x.pick.as_ref()).map(|p| of_class(kb, p.c)).unwrap_or("?");
                format!("{} ({cls})", f.val)
            }
            _ => f.val.clone(),
        };
        by.entry(f.kind.name()).or_default().push(format!("{}{v}", if f.neg { "not " } else { "" }));
    }
    for (k, v) in by {
        let _ = write!(s, "; {k}: {}", v.join(", "));
    }
    s
}

pub fn judge(kb: &Kb, b: &Built, runs: &Path, n: usize, decoys: usize, batch: usize, cap: f64) -> Result<String> {
    let log = runs.join("hs-judge-cost.jsonl");
    let cwd = runs.join("vmm-cwd");
    // sample: 150 codes by hash (spread over chapters); decoys — another code's output paired with a foreign description
    let mut idx: Vec<usize> = (0..b.rows.len()).collect();
    idx.sort_by_key(|&i| fnv(&format!("hs-judge:{}", b.rows[i].code)));
    let real: Vec<usize> = idx[..n.min(idx.len())].to_vec();
    let mut items: Vec<(String, usize, usize, bool)> = real.iter().map(|&i| (String::new(), i, i, false)).collect();
    for k in 0..decoys.min(real.len()) {
        let i = idx[n + k];
        let j = idx[n + decoys + k];
        items.push((String::new(), i, j, true));
    }
    items.sort_by_key(|x| fnv(&format!("order:{}:{}", b.rows[x.1].code, x.3)));
    for (k, it) in items.iter_mut().enumerate() {
        it.0 = format!("{}", 101 + k);
    }
    let head = "You evaluate automatic extraction of an ontology from customs product descriptions (Harmonized System 2022). Each item: id, the description, and the extraction: main = the product concept mapped to a WordNet sense (lemma, lexicographer file, gloss); class = the heading concept above it; then features by type (state, process, of = material or source, form, purpose, part, taxon, bound = numeric threshold in base units, has = composition, exclude, other = unclassified label).\n\
Score each item 1-5 on: M = main concept and its WordNet sense are right for this product; F = completeness: the distinguishing properties in the description are captured; P = precision: 5 = nothing wrong (no wrong sense, feature, or threshold).\n\
Answer one line per item exactly: id<TAB>M<TAB>F<TAB>P<TAB>note (max 12 words). No other text.\n\n";
    let mut out: Vec<(String, u8, u8, u8, String)> = Vec::new();
    let mut spent = 0.0;
    for ch in items.chunks(batch) {
        let mut p = String::from(head);
        for (id, i, j, _) in ch {
            let _ = writeln!(p, "{id}\tdescription: {}\n\textraction: {}\n", b.rows[*i].desc, render(kb, b, *j));
        }
        let (ans, cost) = crate::wdvmm::ask(&cwd, &log, cap, 0.6, &p)?;
        spent += cost;
        for l in ans.lines() {
            let f: Vec<&str> = l.split('\t').map(str::trim).collect();
            if f.len() >= 4 && ch.iter().any(|x| x.0 == f[0]) {
                let sc = |x: &str| x.trim().parse::<u8>().ok().filter(|v| (1..=5).contains(v));
                if let (Some(a), Some(bb), Some(c)) = (sc(f[1]), sc(f[2]), sc(f[3])) {
                    out.push((f[0].to_string(), a, bb, c, f.get(4).unwrap_or(&"").to_string()));
                }
            }
        }
    }
    let mut tsv = String::from("# id\tcode_desc\tcode_extraction\tdecoy\tM\tF\tP\tnote\n");
    let (mut r_sum, mut d_sum, mut r_n, mut d_n) = ([0f64; 3], [0f64; 3], 0usize, 0usize);
    let mut worst: Vec<(f64, String)> = Vec::new();
    for (id, a, bb, c, note) in &out {
        let Some(it) = items.iter().find(|x| &x.0 == id) else { continue };
        let _ = writeln!(tsv, "{id}\t{}\t{}\t{}\t{a}\t{bb}\t{c}\t{note}", b.rows[it.1].code, b.rows[it.2].code, it.3 as u8);
        let v = [*a as f64, *bb as f64, *c as f64];
        if it.3 {
            d_n += 1;
            for k in 0..3 {
                d_sum[k] += v[k];
            }
        } else {
            r_n += 1;
            for k in 0..3 {
                r_sum[k] += v[k];
            }
            worst.push(((v[0] + v[1] + v[2]) / 3.0, format!("{} ({a}/{bb}/{c}): {note}", b.rows[it.1].code)));
        }
    }
    std::fs::write(runs.join("hs-judge.tsv"), tsv)?;
    worst.sort_by(|x, y| x.0.total_cmp(&y.0));
    let avg = |s: [f64; 3], n: usize| if n == 0 { "-".into() } else { format!("M {:.2}, F {:.2}, P {:.2}, mean {:.2}", s[0] / n as f64, s[1] / n as f64, s[2] / n as f64, (s[0] + s[1] + s[2]) / (3.0 * n as f64)) };
    let mut rep = format!("## Blind Opus judge\n\nScores: real {r_n} of {}, decoys {d_n} of {} (a decoy is another code's output paired with a foreign description: a negative control of the rubric). Cost ${spent:.2}.\n\n- **real:** {}\n- **decoys:** {}\n\nWorst real ones:\n", real.len(), decoys, avg(r_sum, r_n), avg(d_sum, d_n));
    for w in worst.iter().take(8) {
        let _ = writeln!(rep, "- {}", w.1);
    }
    Ok(rep)
}

/// Disputed senses: (phrase, heading) with a small margin and several senses — the LLM chooses the sense number.
pub fn confirm(kb: &Kb, b: &Built, runs: &Path, out: &Path, n: usize, margin: f64, batch: usize, cap: f64) -> Result<String> {
    let g = &kb.g;
    let log = runs.join("hs-judge-cost.jsonl");
    let cwd = runs.join("vmm-cwd");
    let mut disputed: BTreeMap<(String, String), (usize, Pick, Vec<Cid>, String)> = BTreeMap::new();
    for (i, r) in b.rows.iter().enumerate() {
        for x in &b.m[i] {
            if let Some(p) = &x.pick {
                if p.n > 1 && matches!(x.kind, Kind::Head | Kind::Item | Kind::Of) {
                    let e = disputed.entry((p.phrase.clone(), ckey(&r.code, &r.p))).or_insert_with(|| (0, p.clone(), x.all.clone(), r.desc.clone()));
                    e.0 += 1;
                    if p.margin < e.1.margin {
                        e.1 = p.clone();
                    }
                }
            }
        }
    }
    // disputed (small margin) and the most frequent (an error in a frequent word is costly: "rubber" as a condom in chapter 40)
    let mut v: Vec<((String, String), (usize, Pick, Vec<Cid>, String))> = disputed.into_iter().collect();
    v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
    let (mut low, mut freq): (Vec<_>, Vec<_>) = v.into_iter().partition(|x| x.1 .1.margin < margin);
    let n_freq = n / 3;
    freq.truncate(n_freq);
    low.truncate(n - freq.len().min(n));
    let mut v = low;
    v.extend(freq);
    let head = "Choose the WordNet sense of a word as used in a customs product description (Harmonized System). Each item: id, the word, one description where it occurs, and numbered candidate senses (lexicographer file: gloss).\n\
Answer one line per item exactly: id<TAB>k<TAB>why (max 10 words), where k is the number of the right sense, or 0 if none fits. No other text.\n\n";
    let mut res = String::from("# HS22: sense confirmations by the LLM (Opus): disputed ones (small WSD margin) and the most frequent. phrase, chapter — the key; synset — the LLM's choice; wsd — the WSD choice.\n# phrase\tchapter\tsynset\twsd\tagree\twhy\n");
    let (mut agree, mut changed, mut none) = (0, 0, 0);
    let mut spent = 0.0;
    for ch in v.chunks(batch) {
        let mut p = String::from(head);
        for (k, ((ph, _), (_, _, all, desc))) in ch.iter().enumerate() {
            let _ = writeln!(p, "{}\tword: {ph}\n\tdescription: {desc}", k + 1);
            for (j, &c) in all.iter().enumerate().take(12) {
                let _ = writeln!(p, "\t{}. {} ({}): {}", j + 1, g.label(c), g.lexfile_name(c), g.gloss_of(c).chars().take(90).collect::<String>());
            }
            p.push('\n');
        }
        let (ans, cost) = crate::wdvmm::ask(&cwd, &log, cap, 0.5, &p)?;
        spent += cost;
        for l in ans.lines() {
            let f: Vec<&str> = l.split('\t').map(str::trim).collect();
            let (Some(Ok(id)), Some(Ok(k))) = (f.first().map(|x| x.parse::<usize>()), f.get(1).map(|x| x.parse::<usize>())) else { continue };
            let Some(((ph, hd), (_, pk, all, _))) = id.checked_sub(1).and_then(|i| ch.get(i)) else { continue };
            if k == 0 || k > all.len() {
                none += 1;
                continue;
            }
            let c = all[k - 1];
            if c == pk.c {
                agree += 1;
            } else {
                changed += 1;
            }
            let _ = writeln!(res, "{ph}\t{hd}\t{}\t{}\t{}\t{}", g.cid_str(c), g.cid_str(pk.c), (c == pk.c) as u8, f.get(2).unwrap_or(&""));
        }
    }
    std::fs::write(out.join("sense-confirm.tsv"), res)?;
    Ok(format!("## Confirmation of disputed senses (LLM)\n\nKey — (phrase, chapter). Disputed (margin < {margin}) and the most frequent: {} taken. LLM: agrees with WSD {agree}, a different sense {changed}, none fits {none}. Cost ${spent:.2}.\n", v.len()))
}

pub fn load_confirm(p: &Path) -> HashMap<(String, String), String> {
    let mut m = HashMap::new();
    for l in std::fs::read_to_string(p).unwrap_or_default().lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() >= 3 {
            m.insert((f[0].to_string(), f[1].to_string()), f[2].to_string());
        }
    }
    m
}

// ── labels in other languages (Wikidata) vs external HS translations ─────────────────────────────────

/// Label stem for searching in a translation: the longest word without its last 1–2 letters (cases, plural).
fn label_stem(l: &str) -> String {
    let w = l.to_lowercase().split(|c: char| !c.is_alphanumeric()).max_by_key(|w| w.chars().count()).unwrap_or_default().to_string();
    let n = w.chars().count();
    let keep = if n > 5 { n - 2 } else if n > 4 { n - 1 } else { n };
    w.chars().take(keep).collect()
}

/// `onto hs labels`: labels of head concepts (Q) in eight languages from `hs-labels.tsv` (`sql/hs-labels.sql`, Wikidata CC0)
/// vs external HS translations (`$MOVA_DATA/hs-translations/*.properties`, read-only):
/// whether our concept's name occurs in the subheading's translation. A mismatch means our mapping, the translation, or a synonym.
pub fn labels(runs: &Path, out: &Path) -> Result<String> {
    let codes = tsv_rows(&hs::data_dir().join("codes.tsv"))?;
    let mut lab: HashMap<u32, HashMap<String, Vec<(String, bool)>>> = HashMap::new();
    for l in std::fs::read_to_string(runs.join("hs-labels.tsv")).context("hs-labels.tsv — run sql/hs-labels.sql first")?.lines() {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() >= 4 {
            if let Ok(q) = f[0].parse() {
                lab.entry(q).or_default().entry(f[1].to_string()).or_default().push((f[2].to_string(), f[3] == "true"));
            }
        }
    }
    let tr_dir = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("hs-translations");
    let mut rep = String::from("## Labels in other languages (Wikidata) vs external HS translations\n\nA subheading counts if its head concept has a Q, a label in the language and a translated name. A match means the stem of the label or an alias (the longest word without its ending) occurs in the translation.\n\n| language | subheadings | match | share |\n|---|---:|---:|---:|\n");
    let mut tsv = String::from("# HS22: labels of head concepts in other languages — Wikidata (CC0), main labels only (not aliases).\n# q\tlang\tlabel\n");
    let mut qs: Vec<&u32> = lab.keys().collect();
    qs.sort();
    for q in qs {
        let mut langs: Vec<(&String, &Vec<(String, bool)>)> = lab[q].iter().collect();
        langs.sort();
        for (lang, v) in langs {
            if let Some((l, _)) = v.iter().find(|x| !x.1) {
                let _ = writeln!(tsv, "Q{q}\t{lang}\t{l}");
            }
        }
    }
    std::fs::write(out.join("labels.tsv"), tsv)?;
    let mut misses = String::new();
    for lang in ["uk", "de", "fr", "es", "pl", "ru", "it", "nl"] {
        let file = if lang == "es" { "es_ES" } else { lang };
        let Ok(text) = std::fs::read_to_string(tr_dir.join(format!("{file}.properties"))) else { continue };
        let names: HashMap<&str, String> = text.lines().filter(|l| !l.starts_with('#') && !l.starts_with('!')).filter_map(|l| l.split_once('=')).filter_map(|(k, v)| k.trim().strip_suffix(".name").map(|c| (c, v.trim().to_lowercase()))).collect();
        let (mut n, mut hit) = (0usize, 0usize);
        let mut miss_ex: Vec<String> = Vec::new();
        for r in &codes {
            let Some(q) = r[7].strip_prefix('Q').and_then(|x| x.parse::<u32>().ok()) else { continue };
            let (Some(ls), Some(name)) = (lab.get(&q).and_then(|m| m.get(lang)), names.get(r[0].as_str())) else { continue };
            n += 1;
            if ls.iter().any(|(l, _)| { let s = label_stem(l); s.chars().count() >= 3 && name.contains(&s) }) {
                hit += 1;
            } else if miss_ex.len() < 6 && fnv(&format!("{lang}{}", r[0])) % 7 == 0 {
                miss_ex.push(format!("{} {} → «{}»", r[0], r[3], ls.iter().find(|x| !x.1).map(|x| x.0.as_str()).unwrap_or("-")));
            }
        }
        let _ = writeln!(rep, "| {lang} | {n} | {hit} | {:.1}% |", if n == 0 { 0.0 } else { 100.0 * hit as f64 / n as f64 });
        let _ = writeln!(misses, "- {lang}: {}", miss_ex.join("; "));
    }
    let _ = writeln!(rep, "\nMismatches (examples; our concept → label):\n{misses}");
    Ok(rep)
}
