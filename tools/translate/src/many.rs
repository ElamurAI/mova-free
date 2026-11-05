//! Many English translations, each in its own world, linked through the meta-world (`translate many`).
//!
//!   translate many --pools <dir> --lexicon-from <lexicon.tsv> --out <dir> [--ebible <dir>] [--threads N]
//!                  [--train-every 4] [--near 3] [--hubs eng-kjv,eng-web] [--hold-every 5] [--reference eng-kjv]
//!                  [--max-words 2500] [--max-len 1] [--extend on|off] [--distinct on|off] [--learn on|off]
//!                  [--shuffled-lexicon on|off] [--canon on]
//!
//! Pools are `school-sents usfm` cuts of eBible's English translations (`<dir>/<name>.tsv` and `.verses`); the 66 books
//! of the Protestant canon are read (the Apocrypha and front matter are not). Verses line up by their English number;
//! a chapter whose numbers are not the reference's is left out for that translation (`versification`).
//!
//! **Blocks.** The unit is a block: a run of whole chapters up to 2 500 words, cut by `roundtrip blocks`' rule
//! (`blocks::make_rt`) on the reference, the same blocks for every translation. Each translation reads each block as
//! one discourse (the frames' context goes through the block) into **its own world**: the block's verses as its own
//! words read with no lexicon (`W_i`, each word its own concept). The **meta-world** links the worlds: the lexicon
//! reads `W_i` into `M_i`, and `W_i → M_i → M_j → W_j → j` is the cross translation. Measured per block (verse-level
//! numbers are its breakdown):
//! - the links: the bits of `W_i → M_i` (what the lexicon changes of a world on the way up) and of `M_j → W_j` (what the
//!   meta-world does not hold of a world, which the author's delta and the encoder pay on the way down), roundtrip's
//!   tree script on meta tokens (`measure::reader_bits`);
//! - between worlds in the meta-world: the reader script `M_i → M_j`, and the agreement — the share of concepts and
//!   of (head concept, relation, concept) edges two translations share;
//! - **logical closeness** (`logic`): the events, their order, roles, negation, modality and numbers of the two worlds,
//!   compared through the lexicon; contradictions per 1 000 words, the textual residual apart;
//! - the **cross residual**: the reader script plus the target's own encoder and stack (its autoencoder tail, which
//!   does not depend on the source: `measure`), per block (the verses' codes summed: verse-anchored) and per 1 000
//!   words. A sample (every translation into the reference and back) runs whole through `Run::verse` with the gate.
//!
//! **Training.** The baseline is the lexicon of a `translate eval` run as it is (`--lexicon-from`). Then a lexicon is
//! learned on many pairs (`meta::learn_gated`, the judged rule, the judge per pair, and the logic gate: a merge that
//! raises the contradictions of its sampled verse pairs is refused): each translation with the hubs and with its
//! `near` nearest by the baseline's agreement on the training blocks. Translations every `hold-every`-th by name are
//! held out of the lexicon (no pair, not in the judge's corpora). The standard is learned on the training blocks of
//! all the translations; each translation has a delta (its own words at 0.8) and its typography; with the learned
//! lexicon the deltas go through the logic gate (`Delta::logic_gate`).
//!
//! **Streaming.** The training blocks are parsed once and kept compact (trees and facts). The dev blocks are streamed:
//! a block is parsed for every translation, measured, and dropped; the lexicon, the standard and the deltas are the
//! state kept across blocks. The meta-world of a block is stored once for all translations: concepts and edges by
//! compact ids (`Interner`), each with the set of translations that hold it (a bit mask); its size is reported per block
//! and in total, with the peak memory. `--canon on` streams every block of the training and dev books (the test books
//! stay closed) through a given lexicon and reports only the meta-world's size and memory.
//!
//! Controls: verses shuffled inside each block (agreement falls, contradictions and cross bits rise), a damaged
//! residual (the gate red), and with `--shuffled-lexicon on` the lexicon learned on shuffled verse pairs.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use en::annotate::Annotator;
use en::gram::Rel;

use crate::bible::{self, Split, VKey};
use crate::blocks::{self, Block};
use crate::corpus::{self, Corpus, PVerse, par_map};
use crate::logic::{self, Logic};
use crate::measure::{self, Run, Src};
use crate::meta::{self, Lexicon, MTok};
use crate::profile::{self, Delta, Encoder, Seen, Standard, Typo};
use crate::rules;
use crate::sources::Kind;

struct Args {
    pools: PathBuf,
    ebible: Option<PathBuf>,
    lexicon_from: PathBuf,
    out: PathBuf,
    model: Option<PathBuf>,
    threads: usize,
    train_every: usize,
    near: usize,
    hubs: Vec<String>,
    hold_every: usize,
    reference: String,
    max_words: usize,
    learn: bool,
    shuffled_lexicon: bool,
    canon: bool,
    /// phrase pairs whose sides share a word are no candidates (`LexParams::distinct`)
    distinct: bool,
    /// the learned lexicon extends the baseline: its clusters that touch none of the baseline's members are added
    extend: bool,
    /// the longest phrase the learned lexicon takes (1: words only)
    max_len: usize,
    /// only the verses every measured translation has (`--parallel-only on`): the texts truly parallel, no canon or
    /// numbering differences in the comparison
    parallel_only: bool,
    /// residual rules for every ordered pair (`--rules on`, `rules`)
    rules: bool,
    /// only the first N ordered pairs (`--rules-limit N`; 0: all), for timing
    rules_limit: usize,
    /// the Hebrew source (`--wlc <dir> --prag <tsv> --jps-heb <pool>`): every translation measured against it
    wlc: Option<PathBuf>,
    prag: Option<PathBuf>,
    jps_heb: Option<PathBuf>,
    /// the four authors of `translate eval`, named as eBible's folders
    old: Vec<String>,
}

fn args(v: &[String]) -> Result<Args> {
    let mut m: BTreeMap<String, String> = BTreeMap::new();
    let mut it = v.iter();
    while let Some(x) = it.next() {
        let val = it.next().with_context(|| format!("{x} needs a value"))?;
        m.insert(x.trim_start_matches("--").to_string(), val.clone());
    }
    let p = |k: &str| m.get(k).map(PathBuf::from).with_context(|| format!("--{k}"));
    let num = |k: &str, d: usize| -> Result<usize> { Ok(m.get(k).map(|s| s.parse()).transpose()?.unwrap_or(d)) };
    Ok(Args {
        pools: p("pools")?,
        lexicon_from: p("lexicon-from")?,
        out: p("out")?,
        ebible: m.get("ebible").map(PathBuf::from),
        model: m.get("model").map(PathBuf::from),
        threads: num("threads", 8)?,
        train_every: num("train-every", 4)?.max(1),
        near: num("near", 3)?,
        hubs: m.get("hubs").map(String::as_str).unwrap_or("eng-kjv,eng-web").split(',').map(str::to_string).collect(),
        hold_every: num("hold-every", 5)?,
        reference: m.get("reference").cloned().unwrap_or_else(|| "eng-kjv".into()),
        max_words: num("max-words", 2500)?,
        learn: m.get("learn").is_none_or(|v| v == "on"),
        shuffled_lexicon: m.get("shuffled-lexicon").is_some_and(|v| v == "on"),
        canon: m.get("canon").is_some_and(|v| v == "on"),
        distinct: m.get("distinct").is_none_or(|v| v == "on"),
        extend: m.get("extend").is_none_or(|v| v == "on"),
        max_len: num("max-len", 1)?,
        parallel_only: m.get("parallel-only").is_some_and(|v| v == "on"),
        rules: m.get("rules").is_some_and(|v| v == "on"),
        rules_limit: num("rules-limit", 0)?,
        wlc: m.get("wlc").map(PathBuf::from),
        prag: m.get("prag").map(PathBuf::from),
        jps_heb: m.get("jps-heb").map(PathBuf::from),
        old: ["engjps", "eng-kjv", "eng-web", "eng-asv"].iter().map(|s| s.to_string()).collect(),
    })
}

/// A translation's licence from eBible's `copr.htm`.
fn licence(ebible: Option<&Path>, name: &str) -> String {
    let Some(t) = ebible.and_then(|d| std::fs::read_to_string(d.join(name).join("copr.htm")).ok()) else { return "?".into() };
    let t = t.to_lowercase();
    if t.contains("creativecommons.org/licenses/by-sa") {
        "CC BY-SA 4.0".into()
    } else if t.contains("creativecommons.org/licenses/by/") {
        "CC BY 4.0".into()
    } else if t.contains("public domain") {
        "public domain".into()
    } else {
        "other".into()
    }
}

/// The peak resident memory of this process so far, in GB (`VmHWM`).
fn peak_gb() -> f64 {
    std::fs::read_to_string("/proc/self/status").ok().and_then(|s| s.lines().find(|l| l.starts_with("VmHWM:")).and_then(|l| l.split_whitespace().nth(1)?.parse::<f64>().ok())).map_or(0.0, |kb| kb / 1048576.0)
}

fn lap(t0: &std::time::Instant, what: &str) {
    eprintln!("[{:>7.1}s {:>5.2} GB] {what}", t0.elapsed().as_secs_f64(), peak_gb());
}

/// The words of a verse a versification check compares: lower-case letters, four or more (the names and the content
/// words, which survive any wording).
fn vwords(s: &str) -> BTreeSet<String> {
    s.split(|c: char| !c.is_alphabetic()).filter(|w| w.chars().count() >= 4).map(str::to_lowercase).collect()
}

fn dice_set(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    2.0 * a.intersection(b).count() as f64 / (a.len() + b.len()) as f64
}

/// The chapters of a translation whose verse numbers are the reference's: in a chapter, a verse must share more words
/// with the reference's verse of the same number than with its neighbours (v−1, v+1), on average. A translation
/// numbered another way (the Septuagint's Psalms, the Vulgate's, a book in another order) falls out there.
#[allow(clippy::type_complexity)]
fn versification(t: &BTreeMap<VKey, String>, r: &BTreeMap<VKey, String>) -> (BTreeSet<(u8, u16)>, Vec<((u8, u16), f64, f64)>) {
    let rw: BTreeMap<VKey, BTreeSet<String>> = r.iter().map(|(k, s)| (*k, vwords(s))).collect();
    let mut by: BTreeMap<(u8, u16), (f64, f64, u32)> = BTreeMap::new();
    for (k, s) in t {
        let Some(same) = rw.get(k) else { continue };
        let w = vwords(s);
        let near = [k.2.checked_sub(1), k.2.checked_add(1)].into_iter().flatten().filter_map(|v| rw.get(&(k.0, k.1, v))).map(|x| dice_set(&w, x)).fold(0.0f64, f64::max);
        let e = by.entry((k.0, k.1)).or_default();
        e.0 += dice_set(&w, same);
        e.1 += near;
        e.2 += 1;
    }
    let mut keep = BTreeSet::new();
    let mut dropped = Vec::new();
    for (c, (s, n, cnt)) in by {
        let (s, n) = (s / cnt as f64, n / cnt as f64);
        if s > n {
            keep.insert(c);
        } else {
            dropped.push((c, s, n));
        }
    }
    (keep, dropped)
}

/// Agreement counts of two meta-worlds: (shared, |a|, |b|) for concepts and for edges, punctuation left out.
#[derive(Clone, Copy, Debug, Default)]
pub struct Agree {
    pub c: (u64, u64, u64),
    pub e: (u64, u64, u64),
}

impl Agree {
    fn add(&mut self, o: &Agree) {
        self.c = (self.c.0 + o.c.0, self.c.1 + o.c.1, self.c.2 + o.c.2);
        self.e = (self.e.0 + o.e.0, self.e.1 + o.e.1, self.e.2 + o.e.2);
    }
    pub fn concepts(&self) -> f64 {
        2.0 * self.c.0 as f64 / (self.c.1 + self.c.2).max(1) as f64
    }
    pub fn edges(&self) -> f64 {
        2.0 * self.e.0 as f64 / (self.e.1 + self.e.2).max(1) as f64
    }
}

fn bag(m: &[MTok]) -> (BTreeMap<&str, u32>, BTreeMap<(&str, Rel, &str), u32>) {
    let (mut c, mut e) = (BTreeMap::new(), BTreeMap::new());
    for x in m {
        if x.rel == Rel::Punct {
            continue;
        }
        *c.entry(x.concept.as_str()).or_default() += 1;
        let h = if x.head == 0 || x.head > m.len() { "ROOT" } else { m[x.head - 1].concept.as_str() };
        *e.entry((h, x.rel, x.concept.as_str())).or_default() += 1;
    }
    (c, e)
}

fn shared<K: Ord>(a: &BTreeMap<K, u32>, b: &BTreeMap<K, u32>) -> u64 {
    a.iter().map(|(k, n)| (*n).min(b.get(k).copied().unwrap_or(0)) as u64).sum()
}

pub fn agree(a: &[MTok], b: &[MTok]) -> Agree {
    let ((ca, ea), (cb, eb)) = (bag(a), bag(b));
    let n = |m: &BTreeMap<&str, u32>| m.values().map(|&x| x as u64).sum::<u64>();
    let ne = |m: &BTreeMap<(&str, Rel, &str), u32>| m.values().map(|&x| x as u64).sum::<u64>();
    Agree { c: (shared(&ca, &cb), n(&ca), n(&cb)), e: (shared(&ea, &eb), ne(&ea), ne(&eb)) }
}

/// The source verse each target verse is read from: itself, or (the control) another verse of the same block, half
/// the block's common verses on.
fn pairing(keys: &[VKey], shuffle: bool) -> Vec<(VKey, VKey)> {
    if !shuffle {
        return keys.iter().map(|k| (*k, *k)).collect();
    }
    let n = keys.len();
    keys.iter().enumerate().map(|(i, k)| (keys[(i + n / 2) % n], *k)).collect()
}

/// Concepts by compact ids, shared by every translation and block.
#[derive(Default)]
struct Interner {
    ids: BTreeMap<String, u32>,
}

impl Interner {
    fn id(&mut self, s: &str) -> u32 {
        if let Some(&i) = self.ids.get(s) {
            return i;
        }
        let i = self.ids.len() as u32;
        self.ids.insert(s.to_string(), i);
        i
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct MetaSize {
    blocks: u64,
    /// meta tokens of all the translations
    tokens: u64,
    /// distinct concepts and edges of each block, summed over the blocks
    concepts: u64,
    edges: u64,
    /// (translation, concept) and (translation, edge) memberships: the bits set in the masks
    c_members: u64,
    e_members: u64,
    /// the largest block
    max_concepts: u64,
    max_edges: u64,
}

impl MetaSize {
    /// Bytes of a block's meta-world as stored: a concept id and its mask (4 + 8), an edge (head id, relation,
    /// concept id: 4 + 1 + 4) and its mask (8), and each translation's tokens as (concept id, head, relation, tag)
    /// (4 + 2 + 1 + 1).
    fn bytes(&self) -> u64 {
        self.concepts * 12 + self.edges * 17 + self.tokens * 8
    }
    fn add(&mut self, o: &MetaSize) {
        self.blocks += o.blocks;
        self.tokens += o.tokens;
        self.concepts += o.concepts;
        self.edges += o.edges;
        self.c_members += o.c_members;
        self.e_members += o.e_members;
        self.max_concepts = self.max_concepts.max(o.concepts);
        self.max_edges = self.max_edges.max(o.edges);
    }
}

/// The meta-world of one block: every translation's meta tokens; each concept and each edge (head concept, relation,
/// concept) once, with the translations that hold it as a bit mask (up to 64 translations).
fn block_meta(it: &mut Interner, metas: &[(usize, &BTreeMap<VKey, Vec<MTok>>)]) -> MetaSize {
    let mut cs: BTreeMap<u32, u64> = BTreeMap::new();
    let mut es: BTreeMap<(u32, u8, u32), u64> = BTreeMap::new();
    let mut tokens = 0u64;
    for (t, vs) in metas {
        let bit = 1u64 << (*t % 64);
        for m in vs.values() {
            tokens += m.len() as u64;
            for x in m {
                let c = it.id(&x.concept);
                *cs.entry(c).or_default() |= bit;
                let h = if x.head == 0 || x.head > m.len() { u32::MAX } else { it.id(&m[x.head - 1].concept) };
                *es.entry((h, x.rel as u8, c)).or_default() |= bit;
            }
        }
    }
    let ones = |m: &mut dyn Iterator<Item = &u64>| m.map(|x| x.count_ones() as u64).sum::<u64>();
    MetaSize { blocks: 1, tokens, concepts: cs.len() as u64, edges: es.len() as u64, c_members: ones(&mut cs.values()), e_members: ones(&mut es.values()), max_concepts: cs.len() as u64, max_edges: es.len() as u64 }
}

/// One ordered pair over a set of blocks.
#[derive(Clone, Debug, Default)]
pub struct PairM {
    pub blocks: u64,
    pub verses: u64,
    pub words: u64,
    pub reader: f64,
    pub tail: f64,
    pub agree: Agree,
    /// logic through the meta-world, and of the worlds as they are (no lexicon)
    pub logic: Logic,
    pub logic_raw: Logic,
    /// the sensitivity control: one change of meaning put into the target (`logic::Sens`)
    pub sens: logic::Sens,
    /// the reader's bits by level (`LEVELS`)
    pub lv_bits: [f64; 4],
    /// «no match» marks by level: in the target (lexical, semantic: meta tokens nothing in the source stands for;
    /// structural, canonical: verses), and in the source (what the target leaves out)
    pub nomatch: [u64; 4],
    pub nomatch_src: [u64; 4],
    /// verse pairs of the same number the aligner refused to force (part of `nomatch[STRUCTURAL]`)
    pub unaligned: u64,
    /// the target's words in blocks the source lacks (the base of the canonical level's bits)
    pub absent_words: u64,
}

/// The levels of a difference between two worlds: lexical (other words for the same concept), semantic (another
/// reading of an event or of its roles), structural (numbering, order, how the text is divided), canonical (which
/// books and passages a tradition has).
pub const LEVELS: [&str; 4] = ["lexical", "semantic", "structural", "canonical"];
pub const LEXICAL: usize = 0;
pub const SEMANTIC: usize = 1;
pub const STRUCTURAL: usize = 2;
pub const CANONICAL: usize = 3;

/// The level of a residual class.
pub fn level_of(c: roundtrip::class::Class) -> usize {
    use roundtrip::class::Class as C;
    match c {
        C::Neg | C::Quantity | C::Tense | C::Attachment | C::Dropped | C::Links | C::Quote | C::Repetition => SEMANTIC,
        C::Order | C::Sentence | C::Punct | C::Layout | C::Spacing | C::Meta | C::Markup | C::Conllu | C::FrTable | C::Given | C::Repeat => STRUCTURAL,
        _ => LEXICAL,
    }
}

/// A content word (its absence is a matter of meaning): a noun, verb, adjective, adverb or number.
fn content(t: en::gram::Tag) -> bool {
    let s = format!("{t:?}");
    ["NN", "VB", "JJ", "RB", "CD"].iter().any(|p| s.starts_with(p))
}

impl PairM {
    fn add(&mut self, o: &PairM) {
        for l in 0..4 {
            self.lv_bits[l] += o.lv_bits[l];
            self.nomatch[l] += o.nomatch[l];
            self.nomatch_src[l] += o.nomatch_src[l];
        }
        self.unaligned += o.unaligned;
        self.absent_words += o.absent_words;
        self.blocks += o.blocks;
        self.verses += o.verses;
        self.words += o.words;
        self.reader += o.reader;
        self.tail += o.tail;
        self.agree.add(&o.agree);
        self.logic.add(&o.logic);
        self.logic_raw.add(&o.logic_raw);
        self.sens.add(&o.sens);
    }
    pub fn per_kw(&self) -> f64 {
        1000.0 * (self.reader + self.tail) / self.words.max(1) as f64
    }
    pub fn reader_kw(&self) -> f64 {
        1000.0 * self.reader / self.words.max(1) as f64
    }
    pub fn per_block(&self) -> f64 {
        (self.reader + self.tail) / self.blocks.max(1) as f64
    }
    pub fn contra_kw(&self) -> f64 {
        1000.0 * self.logic.contra as f64 / self.words.max(1) as f64
    }
    pub fn dist_kw(&self) -> f64 {
        1000.0 * self.logic.distance() / self.words.max(1) as f64
    }
    pub fn contra_raw_kw(&self) -> f64 {
        1000.0 * self.logic_raw.contra as f64 / self.words.max(1) as f64
    }
}

/// A translation's own figures over the dev blocks: its autoencoder and the links of its world to the meta-world.
#[derive(Clone, Debug, Default)]
struct Own {
    verses: u64,
    words: u64,
    tail: f64,
    gate: u64,
    up: f64,
    down: f64,
    overrides: usize,
}

/// One block's line: name, the reference's words, mean cross per 1 000 words, mean concept agreement, contradictions per
/// 1 000 words, the block's meta-world.
type BlockRow = (String, u64, f64, f64, f64, MetaSize);

/// What one lexicon gives over all the translations.
struct Measured {
    name: String,
    lex_size: (usize, usize),
    refused_merges: usize,
    refused_overrides: Vec<String>,
    std_tokens: u64,
    own: BTreeMap<String, Own>,
    pairs: BTreeMap<(String, String), PairM>,
    shuffled: BTreeMap<(String, String), PairM>,
    per_block: Vec<BlockRow>,
    size: MetaSize,
    /// the whole split code on a sample: (verses, gate ok, the largest gap to reader + tail); the gate control
    whole: (u64, u64, f64),
    control: (u64, u64),
    peak: f64,
    /// the reasons of the contradictions of one pair on the first dev block
    examples: Vec<String>,
    /// every measured translation's meta-world of the dev verses, with the block's index (kept for `--rules on`)
    kept: BTreeMap<usize, BTreeMap<VKey, (usize, Vec<MTok>)>>,
}

struct Ctx<'a> {
    a: &'a Annotator,
    names: &'a [String],
    pools: &'a BTreeMap<String, BTreeMap<VKey, String>>,
    /// translations measured (indices into `names`)
    measured: &'a [usize],
    train: &'a [Corpus],
    /// index into `names` → index into `train`
    train_of: &'a BTreeMap<usize, usize>,
    typos: &'a BTreeMap<usize, Typo>,
    dev_blocks: &'a [Block],
    group: &'a (dyn Fn(VKey) -> (u8, u16) + Sync),
    threads: usize,
    reference: usize,
    /// the level of a verse a translation does not have: canonical (its book, or the passage, is not in that
    /// tradition or edition) or structural (its chapter is numbered another way, or the verse is joined to another)
    absent: &'a (dyn Fn(usize, VKey) -> usize + Sync),
    parallel_only: bool,
    /// keep the dev meta-worlds for the residual rules and the Hebrew (`Measured::kept`)
    rules: bool,
}

/// Parse one block for one translation: its verses as one discourse.
fn parse_block(x: &Ctx, t: usize, b: &Block) -> Result<Corpus> {
    let name = &x.names[t];
    corpus::parse_grouped(x.a, name, Kind::Bible, &x.pools[name], &|k| b.contains(k), x.group, 1, false)
}

fn bits(o: &measure::Out) -> f64 {
    o.res.iter().map(|r| r.cells.values().map(|c| c.bits).sum::<f64>()).sum()
}

fn measure_lex(x: &Ctx, lex: &Lexicon, name: &str, gate_deltas: bool, t0: &std::time::Instant) -> Result<Measured> {
    // the standard on every translation's training blocks, a delta for each measured one
    let units: Vec<Vec<Seen>> = par_map(&(0..x.train.len()).collect::<Vec<_>>(), x.threads, &|&i| profile::seen_of(lex, &x.train[i], &|_| true));
    let std = Standard::train(&units.iter().flatten().collect::<Vec<_>>());
    lap(t0, &format!("{name}: standard on {} meta tokens", std.tokens));
    let trained: Vec<(Delta, Vec<String>)> = par_map(&x.measured.to_vec(), x.threads, &|&t| {
        let u: Vec<&Seen> = x.train_of.get(&t).map(|&i| units[i].iter().collect()).unwrap_or_default();
        let mut d = Delta::train(&x.names[t], &std, x.a, &u, false).with_own_words(&u, 0.8);
        let refused = if gate_deltas { d.logic_gate() } else { Vec::new() };
        (d, refused)
    });
    drop(units);
    let refused_overrides: Vec<String> = trained.iter().flat_map(|(_, r)| r.iter().cloned()).collect();
    let delta_of: BTreeMap<usize, &Delta> = x.measured.iter().copied().zip(trained.iter().map(|(d, _)| d)).collect();
    lap(t0, &format!("{name}: deltas ({} overrides refused by the logic gate)", refused_overrides.len()));
    let enc = |t: usize| Encoder { name: x.names[t].clone(), std: Some(&std), delta: Some(delta_of[&t]), typo: x.typos.get(&t) };
    let read = |ws: &[en::annotate::Word]| lex.read(ws);
    let own_lex = Lexicon { max_len: 1, coarse: lex.coarse, flat: lex.flat, ..Default::default() };
    let raw = Lexicon::default();
    let mut m = Measured {
        name: name.to_string(),
        lex_size: (lex.single.len(), lex.phrases.len()),
        refused_merges: lex.refused_logic.len(),
        refused_overrides,
        std_tokens: std.tokens,
        own: BTreeMap::new(),
        pairs: BTreeMap::new(),
        shuffled: BTreeMap::new(),
        per_block: Vec::new(),
        size: MetaSize::default(),
        whole: (0, 0, 0.0),
        control: (0, 0),
        peak: 0.0,
        examples: Vec::new(),
        kept: BTreeMap::new(),
    };
    let mut it = Interner::default();
    let ordered: Vec<(usize, usize)> = x.measured.iter().flat_map(|&i| x.measured.iter().filter(move |&&j| j != i).map(move |&j| (i, j))).collect();
    let r = x.reference;
    for (bi, b) in x.dev_blocks.iter().enumerate() {
        // the block in every translation: parsed, its world (own words) and its meta-world
        let parsed: Vec<Result<Corpus>> = par_map(&x.measured.to_vec(), x.threads, &|&t| parse_block(x, t, b));
        let mut cs: BTreeMap<usize, Corpus> = BTreeMap::new();
        for (&t, c) in x.measured.iter().zip(parsed) {
            let c = c?;
            if !c.verses.is_empty() {
                cs.insert(t, c);
            }
        }
        // parallel only: the verses every measured translation has; a block one of them lacks is left out
        if x.parallel_only {
            if cs.len() < x.measured.len() {
                continue;
            }
            let mut common: BTreeSet<VKey> = cs.values().next().map(|c| c.verses.keys().copied().collect()).unwrap_or_default();
            for c in cs.values() {
                common.retain(|k| c.verses.contains_key(k));
            }
            if common.is_empty() {
                continue;
            }
            for c in cs.values_mut() {
                c.verses.retain(|k, _| common.contains(k));
            }
        }
        let worlds: BTreeMap<usize, BTreeMap<VKey, Vec<MTok>>> = cs.iter().map(|(t, c)| (*t, c.verses.iter().map(|(k, v)| (*k, own_lex.read(v.meta_words()))).collect())).collect();
        let metas: BTreeMap<usize, BTreeMap<VKey, Vec<MTok>>> = cs.iter().map(|(t, c)| (*t, c.verses.iter().map(|(k, v)| (*k, lex.read(v.meta_words()))).collect())).collect();
        if x.rules && name == "many pairs" {
            for (t, v) in &metas {
                for (k, mt) in v {
                    m.kept.entry(*t).or_default().insert(*k, (bi, mt.clone()));
                }
            }
        }
        // each translation's autoencoder tail and its links to the meta-world, verse by verse
        let jobs: Vec<(usize, VKey)> = cs.iter().flat_map(|(t, c)| c.verses.keys().map(move |k| (*t, *k))).collect();
        let outs: Vec<(f64, bool, f64, f64)> = par_map(&jobs, x.threads, &|&(t, k)| {
            let e = enc(t);
            let run = Run { a: x.a, enc: &e, read: &read, corrupt: false, why: false, direct: false };
            let o = run.verse(&Src::Meta(&metas[&t][&k]), &cs[&t].verses[&k]);
            (bits(&o), o.gate_ok, measure::reader_bits(&worlds[&t][&k], &metas[&t][&k]), measure::reader_bits(&metas[&t][&k], &worlds[&t][&k]))
        });
        let mut tails: BTreeMap<(usize, VKey), f64> = BTreeMap::new();
        for (&(t, k), o) in jobs.iter().zip(&outs) {
            tails.insert((t, k), o.0);
            let e = m.own.entry(x.names[t].clone()).or_default();
            e.verses += 1;
            e.words += cs[&t].verses[&k].text.split_whitespace().count() as u64;
            e.tail += o.0;
            e.gate += o.1 as u64;
            e.up += o.2;
            e.down += o.3;
            e.overrides = delta_of[&t].size();
        }
        // every ordered pair over the block, and the same with the verses shuffled inside the block
        let present: Vec<(usize, usize)> = ordered.iter().copied().filter(|(i, j)| cs.contains_key(i) || cs.contains_key(j)).collect();
        let none: BTreeMap<VKey, Vec<MTok>> = BTreeMap::new();
        let one = |i: usize, j: usize, shuffle: bool| -> PairM {
            let (mi, mj) = (metas.get(&i).unwrap_or(&none), metas.get(&j).unwrap_or(&none));
            let mut p = PairM::default();
            // a whole block one side lacks: its verses are «no match» of the canon, kept out of the cross residual
            if mi.is_empty() || mj.is_empty() {
                for (k, mt) in mj {
                    p.nomatch[(x.absent)(i, *k)] += 1;
                    p.absent_words += cs[&j].verses[k].text.split_whitespace().count() as u64;
                    p.lv_bits[(x.absent)(i, *k)] += measure::reader_bits(&[], mt) + 1.0;
                }
                for k in mi.keys() {
                    p.nomatch_src[(x.absent)(j, *k)] += 1;
                }
                return p;
            }
            p.blocks = 1;
            let common: Vec<VKey> = mj.keys().copied().filter(|k| mi.contains_key(k)).collect();
            for (ks, kt) in pairing(&common, shuffle) {
                let (ms, mt) = (&mi[&ks], &mj[&kt]);
                let (vs, vt) = (&cs[&i].verses[&ks], &cs[&j].verses[&kt]);
                p.verses += 1;
                p.words += vt.text.split_whitespace().count() as u64;
                // the aligner never forces a match: where the script costs more than writing the target verse from
                // nothing, the pair is «no match» (structural: the same number, not the same content)
                let (res, ins, del) = measure::reader_res(ms, mt);
                let aligned: f64 = res.cells.values().map(|c| c.bits).sum();
                let alone = measure::reader_bits(&[], mt) + 1.0;
                if aligned > alone {
                    p.reader += alone;
                    p.lv_bits[STRUCTURAL] += alone;
                    p.nomatch[STRUCTURAL] += 1;
                    p.unaligned += 1;
                } else {
                    p.reader += aligned;
                    for ((_, c), cell) in &res.cells {
                        p.lv_bits[level_of(*c)] += cell.bits;
                    }
                    for t in ins {
                        p.nomatch[if content(mt[t].tag) { SEMANTIC } else { LEXICAL }] += 1;
                    }
                    for s in del {
                        p.nomatch_src[if content(ms[s].tag) { SEMANTIC } else { LEXICAL }] += 1;
                    }
                }
                p.tail += tails[&(j, kt)];
                p.agree.add(&agree(ms, mt));
                let base = logic::compare(lex, &vs.facts, &vt.facts);
                p.sens.probe(lex, &vs.facts, &vt.facts, &base);
                p.logic.add(&base);
                p.logic_raw.add(&logic::compare(&raw, &vs.facts, &vt.facts));
            }
            // verses the source does not have (by canon, or by its numbering), and those the target does not have
            for (k, mt) in mj.iter().filter(|(k, _)| !mi.contains_key(k)) {
                let lv = (x.absent)(i, *k);
                let b = measure::reader_bits(&[], mt) + 1.0;
                p.verses += 1;
                p.words += cs[&j].verses[k].text.split_whitespace().count() as u64;
                p.reader += b;
                p.lv_bits[lv] += b;
                p.nomatch[lv] += 1;
                p.tail += tails[&(j, *k)];
            }
            for k in mi.keys().filter(|k| !mj.contains_key(k)) {
                p.nomatch_src[(x.absent)(j, *k)] += 1;
            }
            p
        };
        let res: Vec<(PairM, PairM)> = par_map(&present, x.threads, &|&(i, j)| (one(i, j, false), one(i, j, true)));
        let (mut cross, mut conc, mut contra, mut n) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        for (&(i, j), (p, s)) in present.iter().zip(res) {
            if p.verses > 0 {
                cross += p.per_kw();
                conc += p.agree.concepts();
                contra += p.contra_kw();
                n += 1.0;
            }
            m.pairs.entry((x.names[i].clone(), x.names[j].clone())).or_default().add(&p);
            m.shuffled.entry((x.names[i].clone(), x.names[j].clone())).or_default().add(&s);
        }
        // the whole split code with its gate: every translation into the reference and back, every third verse
        let sample: Vec<(usize, usize, VKey)> = if cs.contains_key(&r) {
            cs.keys().copied().filter(|&i| i != r).flat_map(|i| [(i, r), (r, i)]).flat_map(|(i, j)| metas[&j].keys().copied().filter(|k| metas[&i].contains_key(k)).step_by(3).map(move |k| (i, j, k)).collect::<Vec<_>>()).collect()
        } else {
            Vec::new()
        };
        let whole: Vec<(bool, f64)> = par_map(&sample, x.threads, &|&(i, j, k)| {
            let e = enc(j);
            let run = Run { a: x.a, enc: &e, read: &read, corrupt: false, why: false, direct: false };
            let o = run.verse(&Src::Meta(&metas[&i][&k]), &cs[&j].verses[&k]);
            (o.gate_ok, (bits(&o) - measure::reader_bits(&metas[&i][&k], &metas[&j][&k]) - tails[&(j, k)]).abs())
        });
        m.whole.0 += whole.len() as u64;
        m.whole.1 += whole.iter().filter(|w| w.0).count() as u64;
        m.whole.2 = whole.iter().map(|w| w.1).fold(m.whole.2, f64::max);
        // the gate control on the first block: the residual damaged (one space flip) on the sample's first pair
        if m.control.0 == 0 && !sample.is_empty() {
            let ctl: Vec<(usize, usize, VKey)> = sample.iter().copied().filter(|s| (s.0, s.1) == (sample[0].0, sample[0].1)).collect();
            let bad: Vec<bool> = par_map(&ctl, x.threads, &|&(i, j, k)| {
                let e = enc(j);
                let run = Run { a: x.a, enc: &e, read: &read, corrupt: true, why: false, direct: false };
                run.verse(&Src::Meta(&metas[&i][&k]), &cs[&j].verses[&k]).gate_ok
            });
            m.control = (bad.len() as u64, bad.iter().filter(|b| **b).count() as u64);
        }
        // the reasons of the contradictions, on the first block, from the first measured translation into the reference
        if m.examples.is_empty() {
            if let Some(&i) = cs.keys().find(|&&i| i != r).filter(|_| cs.contains_key(&r)) {
                for (k, vt) in &cs[&r].verses {
                    let Some(vs) = cs[&i].verses.get(k) else { continue };
                    let mut why = Vec::new();
                    logic::compare_why(lex, &vs.facts, &vt.facts, Some(&mut why));
                    m.examples.extend(why.into_iter().map(|w| format!("{} {} → {}: {w}", bible::show(*k), x.names[i], x.names[r])));
                    if m.examples.len() >= 20 {
                        break;
                    }
                }
            }
        }
        // the block's meta-world, stored once for all the translations
        let ms: Vec<(usize, &BTreeMap<VKey, Vec<MTok>>)> = metas.iter().map(|(t, v)| (*t, v)).collect();
        let size = block_meta(&mut it, &ms);
        m.size.add(&size);
        let words = cs.get(&r).map_or(0, |c| c.verses.values().map(|v| v.text.split_whitespace().count() as u64).sum());
        let nn = n.max(1.0);
        m.per_block.push((b.name(), words, cross / nn, conc / nn, contra / nn, size));
        lap(t0, &format!("{name}: block {} ({}/{}): {} translations, {} verses, cross {:.0} bits / 1 000 words, concepts {:.3}, contradictions {:.2} / 1 000 words; meta-world {} concepts, {} edges", b.name(), bi + 1, x.dev_blocks.len(), cs.len(), jobs.len(), cross / nn, conc / nn, contra / nn, size.concepts, size.edges));
    }
    m.peak = peak_gb();
    lap(t0, &format!("{name}: whole split code on {} verses, gate {}, largest gap {:.6} bits; gate control {} of {} pass; interned concepts {}", m.whole.0, m.whole.1, m.whole.2, m.control.1, m.control.0, it.ids.len()));
    Ok(m)
}

/// Every translation against the Hebrew it translates (`--wlc`): the Leningrad Codex read into the same meta-world by
/// `hebrew::Reader`, learned on the training books against every measured translation alike (so that no one
/// translation is the reader's model), then on the development verses the bits to write each translation's
/// meta-world out of the Hebrew's, per 1 000 words, and back; the same with the Hebrew verse half a block away (the
/// control). Only the verses whose Hebrew and English numbers agree (the JPS text at both numbers is the same) are
/// used. Writes `hebrew.md`.
fn against_hebrew(ar: &Args, x: &Ctx, wlc: &Path, lex: &Lexicon, m: &Measured, t0: &std::time::Instant) -> Result<()> {
    let heb = crate::hebrew::read(wlc, ar.prag.as_deref())?;
    let jh = ar.jps_heb.as_deref().map(bible::read_pool).transpose()?.unwrap_or_default();
    let jps = x.names.iter().position(|n| n == "engjps").and_then(|i| x.pools.get(&x.names[i]));
    let same: BTreeSet<VKey> = match jps {
        Some(p) if !jh.is_empty() => p.iter().filter(|(k, v)| jh.get(k) == Some(v)).map(|(k, _)| *k).collect(),
        _ => heb.keys().copied().collect(),
    };
    let mut pairs: Vec<(&crate::hebrew::HVerse, Vec<MTok>, Vec<Option<u32>>)> = Vec::new();
    for &t in x.measured {
        let Some(c) = x.train_of.get(&t).map(|&i| &x.train[i]) else { continue };
        for (k, v) in &c.verses {
            if let (true, Some(h)) = (same.contains(k), heb.get(k)) {
                pairs.push((h, lex.read(v.meta_words()), Vec::new()));
            }
        }
    }
    let reader = crate::hebrew::Reader::train(lex, &pairs, false);
    lap(t0, &format!("the Hebrew reader: {} verse pairs over {} translations", pairs.len(), x.measured.len()));
    // the development verses every kept translation has, with their block
    let first = m.kept.values().next();
    let keys: Vec<(usize, VKey)> = first.map(|v| v.iter().filter(|(k, _)| same.contains(k) && heb.contains_key(k) && m.kept.values().all(|t| t.contains_key(k))).map(|(k, (b, _))| (*b, *k)).collect()).unwrap_or_default();
    let hm: BTreeMap<VKey, Vec<MTok>> = keys.iter().map(|(_, k)| (*k, reader.read(lex, &heb[k]).0)).collect();
    let mut by_block: BTreeMap<usize, Vec<VKey>> = BTreeMap::new();
    for (b, k) in &keys {
        by_block.entry(*b).or_default().push(*k);
    }
    let wrong: BTreeMap<VKey, VKey> = by_block.values().flat_map(|ks| { let n = ks.len(); ks.iter().enumerate().map(move |(q, k)| (*k, ks[(q + n / 2) % n])) }).collect();
    let ts: Vec<usize> = m.kept.keys().copied().collect();
    let rows: Vec<(usize, f64, f64, f64, u64)> = par_map(&ts, x.threads, &|&t| {
        let mut r = (t, 0.0, 0.0, 0.0, 0u64);
        for (_, k) in &keys {
            let mt = &m.kept[&t][k].1;
            r.1 += measure::reader_bits(&hm[k], mt);
            r.2 += measure::reader_bits(mt, &hm[k]);
            r.3 += measure::reader_bits(&hm[&wrong[k]], mt);
            r.4 += x.pools.get(&x.names[t]).and_then(|p| p.get(k)).map_or(0, |v| v.split_whitespace().count() as u64);
        }
        r
    });
    let mut rows = rows;
    rows.sort_by(|a, b| (a.1 / a.4.max(1) as f64).total_cmp(&(b.1 / b.4.max(1) as f64)));
    let mut md = format!("# Against the Hebrew\n\nThe Leningrad Codex (MACULA's trees) read into the meta-world by the Hebrew reader, learned on the training books against all {} translations alike; {} development verses whose Hebrew and English numbers agree. Bits per 1 000 words of the translation: from the Hebrew's meta-world to the translation's, back, and from the Hebrew verse half a block away (the control).\n\n| translation | Hebrew → translation | translation → Hebrew | control (wrong verse) |\n|---|---:|---:|---:|\n", x.measured.len(), keys.len());
    for (t, a, b, c, w) in &rows {
        let kw = |v: f64| 1000.0 * v / (*w).max(1) as f64;
        let _ = writeln!(md, "| {} | {:.0} | {:.0} | {:.0} |", x.names[*t], kw(*a), kw(*b), kw(*c));
    }
    std::fs::write(ar.out.join("hebrew.md"), md)?;
    lap(t0, &format!("against the Hebrew: {} translations on {} verses", rows.len(), keys.len()));
    Ok(())
}

/// The residual rules of every ordered pair of measured translations (`rules`): `rules.tsv` (each pair's rules in
/// greedy order) and `rules.md` (the rules that gain most over all the pairs, and per pair the share of its reader
/// bits the rules remove).
fn residual_rules(ar: &Args, names: &[String], m: &Measured, threads: usize, t0: &std::time::Instant) -> Result<()> {
    let ts: Vec<usize> = m.kept.keys().copied().collect();
    let mut pairs: Vec<(usize, usize)> = ts.iter().flat_map(|&i| ts.iter().filter(move |&&j| j != i).map(move |&j| (i, j))).collect();
    if ar.rules_limit > 0 {
        pairs.truncate(ar.rules_limit);
    }
    let done = std::sync::atomic::AtomicUsize::new(0);
    let out: Vec<(usize, usize, f64, Vec<rules::Found>)> = par_map(&pairs, threads, &|&(i, j)| {
        let k = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        if k % 50 == 0 {
            eprintln!("[{:>9.1}s] residual rules: {k} of {} pairs", t0.elapsed().as_secs_f64(), pairs.len());
        }
        let (a, b) = (&m.kept[&i], &m.kept[&j]);
        // the wrong pairing: inside each block, the verse half a block away (as the shuffled control of the squares)
        let mut by_block: BTreeMap<usize, Vec<VKey>> = BTreeMap::new();
        for (k, (bi, _)) in a.iter().filter(|(k, _)| b.contains_key(k)) {
            by_block.entry(*bi).or_default().push(*k);
        }
        let mut vs = Vec::new();
        let mut bits = 0.0;
        for (bi, ks) in &by_block {
            let n = ks.len();
            for (q, k) in ks.iter().enumerate() {
                let w = ks[(q + n / 2) % n];
                vs.push(rules::VPair { fold: bi % 2, src: &a[k].1, tgt: &b[k].1, wrong: &a[&w].1 });
                bits += measure::reader_bits(&a[k].1, &b[k].1);
            }
        }
        (i, j, bits, rules::learn(&vs, 3, 20))
    });
    let mut tsv = String::from("source\ttarget\trank\tkind\trule\tsupport\tgain held out\tgain wrong pairing\tstep\n");
    let mut total: BTreeMap<rules::Rule, (f64, f64, usize)> = BTreeMap::new();
    let mut share: Vec<(f64, String, String)> = Vec::new();
    for (i, j, bits, found) in &out {
        for (r, f) in found.iter().enumerate() {
            let _ = writeln!(tsv, "{}\t{}\t{}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{:.1}", names[*i], names[*j], r + 1, f.rule.kind(), f.rule.show(), f.support, f.gain, f.control, f.step);
            let e = total.entry(f.rule.clone()).or_default();
            e.0 += f.step;
            e.1 += f.control;
            e.2 += 1;
        }
        let saved: f64 = found.iter().map(|f| f.step).sum();
        share.push((100.0 * saved / bits.max(1.0), names[*i].clone(), names[*j].clone()));
    }
    std::fs::write(ar.out.join("rules.tsv"), tsv)?;
    let mut tops: Vec<(rules::Rule, (f64, f64, usize))> = total.into_iter().collect();
    tops.sort_by(|a, b| b.1 .0.total_cmp(&a.1 .0).then_with(|| a.0.cmp(&b.0)));
    let mut md = String::from("# Residual rules\n\nFor every ordered pair, the rules that bring the source's meta-world nearest the target's: learned on one half of the development blocks, measured on the other, joined greedily (`rules`). «Gain» is the reader's bits a rule saves on top of the rules before it; «wrong pairing» is the same rule with the verses paired by the wrong number inside their block: a rule that gains as much there is the target's habit, not a correspondence.\n\n## The rules that gain most over all the pairs\n\n| rule | kind | pairs | bits saved | bits saved, wrong pairing |\n|---|---|---:|---:|---:|\n");
    for (r, (g, c, n)) in tops.iter().take(60) {
        let _ = writeln!(md, "| {} | {} | {n} | {g:.0} | {c:.0} |", r.show(), r.kind());
    }
    share.sort_by(|a, b| b.0.total_cmp(&a.0));
    let _ = writeln!(md, "\n## The share of each pair's reader bits the rules remove\n\n| source | target | removed |\n|---|---|---:|");
    for (p, a, b) in &share {
        let _ = writeln!(md, "| {a} | {b} | {p:.1}% |");
    }
    std::fs::write(ar.out.join("rules.md"), md)?;
    lap(t0, &format!("residual rules: {} pairs, {} rules kept", out.len(), out.iter().map(|o| o.3.len()).sum::<usize>()));
    Ok(())
}

pub fn run(v: &[String]) -> Result<()> {
    let ar = args(v)?;
    std::fs::create_dir_all(&ar.out)?;
    let t0 = std::time::Instant::now();
    let a = Annotator::load(&ar.model.clone().unwrap_or_else(roundtrip::model_path))?;
    lap(&t0, "model loaded");
    // the pools, by name
    let mut names: Vec<String> = std::fs::read_dir(&ar.pools)?.flatten().filter_map(|e| e.file_name().to_str().and_then(|n| n.strip_suffix(".tsv")).map(str::to_string)).collect();
    names.sort();
    let mut pools: BTreeMap<String, BTreeMap<VKey, String>> = BTreeMap::new();
    for n in &names {
        pools.insert(n.clone(), bible::read_pool(&ar.pools.join(format!("{n}.tsv")))?);
    }
    let Some(mut refpool) = pools.get(&ar.reference).cloned() else { bail!("no pool of the reference {}", ar.reference) };
    // a book the reference does not have (the deuterocanon beyond its Apocrypha) takes the first translation by name
    // that has it as its reference: for the blocks and for the versification check
    let mut book_ref: BTreeMap<u8, String> = refpool.keys().map(|k| (k.0, ar.reference.clone())).collect();
    for (n, p) in &pools {
        let new: BTreeSet<u8> = p.keys().map(|k| k.0).filter(|b| !book_ref.contains_key(b)).collect();
        for b in new {
            book_ref.insert(b, n.clone());
            refpool.extend(p.iter().filter(|(k, _)| k.0 == b).map(|(k, v)| (*k, v.clone())));
        }
    }
    // what each translation has before the versification check: its books, and the chapters where it joins verses
    // (a range «\\v 1-2» is read as verse 0)
    let books_of: BTreeMap<String, BTreeSet<u8>> = pools.iter().map(|(n, p)| (n.clone(), p.keys().map(|k| k.0).collect())).collect();
    let joined: BTreeMap<String, BTreeSet<(u8, u16)>> = pools.iter().map(|(n, p)| (n.clone(), p.keys().filter(|k| k.2 == 0).map(|k| (k.0, k.1)).collect())).collect();
    // the versification check
    let mut dropped_ch: BTreeMap<String, Vec<((u8, u16), f64, f64)>> = BTreeMap::new();
    for (n, p) in pools.iter_mut() {
        let (keep, dropped) = if *n == ar.reference { (p.keys().map(|k| (k.0, k.1)).collect(), Vec::new()) } else { versification(p, &refpool) };
        p.retain(|k, _| keep.contains(&(k.0, k.1)));
        dropped_ch.insert(n.clone(), dropped);
    }
    // the blocks, cut on the reference by roundtrip blocks' rule; a verse's discourse is its block
    let refc = Corpus { name: ar.reference.clone(), kind: Kind::Bible, verses: refpool.iter().map(|(k, t)| (*k, PVerse { text: t.clone(), ..Default::default() })).collect() };
    let all_blocks = blocks::make_rt(&refc, &|_| true, ar.max_words);
    drop(refc);
    let first_of: BTreeMap<(u8, u16), u16> = all_blocks.iter().flat_map(|b| (b.first..=b.last).map(move |c| ((b.book, c), b.first))).collect();
    let group = |k: VKey| (k.0, first_of.get(&(k.0, k.1)).copied().unwrap_or(k.1));
    let train_blocks: Vec<Block> = all_blocks.iter().filter(|b| bible::split_of(b.book) == Split::Train).enumerate().filter(|(i, _)| i % ar.train_every == 0).map(|(_, b)| b.clone()).collect();
    // parallel only: the Tanakh's books (the Hebrew canon), the texts every translation of it shares
    let dev_blocks: Vec<Block> = all_blocks.iter().filter(|b| bible::split_of(b.book) == Split::Dev && (!ar.parallel_only || (b.book as usize) < bible::BOOKS.len())).cloned().collect();
    let in_train = |k: VKey| train_blocks.iter().any(|b| b.contains(k));
    let in_dev = |k: VKey| dev_blocks.iter().any(|b| b.contains(k));
    lap(&t0, &format!("blocks: {} in the canon, {} training blocks taken (one in {}), {} dev blocks", all_blocks.len(), train_blocks.len(), ar.train_every, dev_blocks.len()));
    if ar.canon {
        return canon(&ar, &a, &names, &pools, &all_blocks, &group, &t0);
    }
    // who takes part: a translation with dev verses is measured; one with training verses learns
    let mut skipped: Vec<(String, String)> = Vec::new();
    let mut measured: Vec<usize> = Vec::new();
    let mut learner_ok: BTreeSet<usize> = BTreeSet::new();
    for (t, n) in names.iter().enumerate() {
        let p = &pools[n];
        let (nt, nd) = (p.keys().filter(|k| in_train(**k)).count(), p.keys().filter(|k| in_dev(**k)).count());
        // parallel only: measured when it has every dev block of the Tanakh (a translation of the New Testament, or
        // of a few books, is left out rather than emptying the common verses)
        let full = !ar.parallel_only || dev_blocks.iter().all(|b| p.keys().any(|k| b.contains(*k)));
        if nd >= 100 && full {
            measured.push(t);
        } else if nd >= 100 {
            skipped.push((n.clone(), "parallel only: lacks some of the Tanakh's dev blocks, not measured".to_string()));
        }
        if nt >= 200 {
            learner_ok.insert(t);
        }
        if nd < 100 && nt < 200 {
            skipped.push((n.clone(), format!("{} verses of the canon; {nt} in the training blocks, {nd} in the dev blocks: too few to learn from or to measure", p.len())));
        } else if nd < 100 {
            skipped.push((n.clone(), format!("{} verses of the canon; {nd} in the dev blocks: learns, not measured", p.len())));
        }
    }
    // the training blocks, parsed once and kept compact
    let parts: Vec<usize> = (0..names.len()).filter(|t| learner_ok.contains(t) || measured.contains(t)).collect();
    let mut train: Vec<Corpus> = Vec::new();
    let mut train_of: BTreeMap<usize, usize> = BTreeMap::new();
    for &t in &parts {
        let mut c = corpus::parse_grouped(&a, &names[t], Kind::Bible, &pools[&names[t]], &in_train, &group, ar.threads, false)?;
        for v in c.verses.values_mut() {
            v.compact();
        }
        lap(&t0, &format!("{}: {} training verses parsed", names[t], c.verses.len()));
        train_of.insert(t, train.len());
        train.push(c);
    }
    let typos: BTreeMap<usize, Typo> = train_of.iter().map(|(&t, &i)| (t, Typo::learn(&train[i], &|_| true))).collect();
    let reference = names.iter().position(|n| *n == ar.reference).context("the reference")?;
    if !measured.contains(&reference) {
        bail!("the reference has no dev blocks");
    }
    let renumbered: BTreeMap<String, BTreeSet<(u8, u16)>> = dropped_ch.iter().map(|(n, d)| (n.clone(), d.iter().map(|x| x.0).collect())).collect();
    let absent = |t: usize, k: VKey| -> usize {
        let n = &names[t];
        if !books_of[n].contains(&k.0) {
            CANONICAL
        } else if renumbered[n].contains(&(k.0, k.1)) || joined[n].contains(&(k.0, k.1)) || k.2 == 0 {
            STRUCTURAL
        } else {
            CANONICAL
        }
    };
    let x = Ctx { a: &a, names: &names, pools: &pools, measured: &measured, train: &train, train_of: &train_of, typos: &typos, dev_blocks: &dev_blocks, group: &group, threads: ar.threads, reference, absent: &absent, parallel_only: ar.parallel_only, rules: ar.rules || ar.wlc.is_some() };
    // the baseline: the four authors' lexicon as it is
    let mut base = Lexicon::from_tsv(&std::fs::read_to_string(&ar.lexicon_from).with_context(|| format!("{}", ar.lexicon_from.display()))?, 4);
    base.coarse = true;
    base.flat = true;
    lap(&t0, &format!("baseline lexicon: {} words, {} phrases", base.single.len(), base.phrases.len()));
    let mut results: Vec<Measured> = vec![measure_lex(&x, &base, "baseline", false, &t0)?];
    // held out of the lexicon: every hold-every-th measured translation by name, not a hub, not one of the four
    let held: BTreeSet<usize> = measured.iter().enumerate().filter(|(i, t)| ar.hold_every > 0 && i % ar.hold_every == ar.hold_every / 2 && !ar.hubs.contains(&names[**t]) && !ar.old.contains(&names[**t])).map(|(_, t)| *t).collect();
    let learners: Vec<usize> = parts.iter().copied().filter(|t| learner_ok.contains(t) && !held.contains(t)).collect();
    // the pairs: each translation with the hubs and with its nearest by the baseline's agreement on the training blocks
    let train_meta: BTreeMap<usize, BTreeMap<VKey, Vec<MTok>>> = learners.iter().map(|&t| (t, train[train_of[&t]].verses.iter().map(|(k, v)| (*k, base.read(v.meta_words()))).collect())).collect();
    let upairs: Vec<(usize, usize)> = learners.iter().flat_map(|&i| learners.iter().filter(move |&&j| j > i).map(move |&j| (i, j))).collect();
    let tagree: Vec<Agree> = par_map(&upairs, ar.threads, &|&(i, j)| {
        let mut g = Agree::default();
        for (k, m) in &train_meta[&i] {
            if let Some(o) = train_meta[&j].get(k) {
                g.add(&agree(m, o));
            }
        }
        g
    });
    drop(train_meta);
    let tag: BTreeMap<(usize, usize), f64> = upairs.iter().zip(&tagree).filter(|(_, g)| g.c.1 > 0).flat_map(|(&(i, j), g)| [((i, j), g.concepts()), ((j, i), g.concepts())]).collect();
    let mut chosen: BTreeMap<(usize, usize), String> = BTreeMap::new();
    let key = |i: usize, j: usize| if i < j { (i, j) } else { (j, i) };
    for &i in &learners {
        for h in &ar.hubs {
            if let Some(j) = names.iter().position(|n| n == h).filter(|&j| j != i && learners.contains(&j) && tag.contains_key(&(i, j))) {
                chosen.entry(key(i, j)).or_insert_with(|| format!("hub {h}"));
            }
        }
        let mut near: Vec<(f64, usize)> = learners.iter().filter(|&&j| j != i).filter_map(|&j| tag.get(&(i, j)).map(|g| (*g, j))).collect();
        near.sort_by(|p, q| q.0.total_cmp(&p.0).then(p.1.cmp(&q.1)));
        for (g, j) in near.into_iter().take(ar.near) {
            chosen.entry(key(i, j)).or_insert_with(|| format!("near {} ({g:.3})", names[i]));
        }
    }
    lap(&t0, &format!("pairs: {} chosen of {} ({} translations learn, {} held out)", chosen.len(), upairs.len(), learners.len(), held.len()));
    if ar.learn {
        let lp = meta::LexParams { min_count: 3, share: 0.2, cover: 0.02, mutual: false, max_len: ar.max_len, feats: false, coarse: true, fw: false, clauses: false, joined: false, flat: true, own_samples: true, lb: true, per_pair: true, distinct: ar.distinct };
        let pairs: Vec<(&Corpus, &Corpus)> = chosen.keys().map(|&(i, j)| (&train[train_of[&i]], &train[train_of[&j]])).collect();
        let judge: Vec<&Corpus> = learners.iter().map(|&t| &train[train_of[&t]]).collect();
        let verses: BTreeSet<VKey> = train.iter().flat_map(|c| c.verses.keys().copied()).collect();
        let mfn = |p: &[MTok], q: &[MTok]| measure::reader_bits(p, q);
        let lfn = |l: &Lexicon, p: &PVerse, q: &PVerse| logic::compare(l, &p.facts, &q.facts).contra;
        let mut lex = meta::learn_gated(&pairs, &verses, lp, false, Some(&judge), Some(&mfn), Some(&lfn));
        if ar.extend {
            lex = base.extended(&lex);
        }
        lex.coarse = true;
        lex.flat = true;
        lap(&t0, &format!("learned lexicon: {} words, {} phrases; {} merges refused by the logic gate", lex.single.len(), lex.phrases.len(), lex.refused_logic.len()));
        std::fs::write(ar.out.join("lexicon.tsv"), lex.to_tsv())?;
        std::fs::write(ar.out.join("refused-merges.txt"), lex.refused_logic.join("\n") + "\n")?;
        results.push(measure_lex(&x, &lex, "many pairs", true, &t0)?);
        std::fs::write(ar.out.join("refused-overrides.txt"), results.last().map(|m| m.refused_overrides.join("\n")).unwrap_or_default() + "\n")?;
        if ar.rules {
            if let Some(m) = results.last() {
                residual_rules(&ar, &names, m, ar.threads, &t0)?;
            }
        }
        if let (Some(wlc), Some(m)) = (&ar.wlc, results.last()) {
            against_hebrew(&ar, &x, wlc, &lex, m, &t0)?;
        }
        if ar.shuffled_lexicon {
            let mut ls = meta::learn_gated(&pairs, &verses, lp, true, Some(&judge), Some(&mfn), Some(&lfn));
            if ar.extend {
                ls = base.extended(&ls);
            }
            ls.coarse = true;
            ls.flat = true;
            lap(&t0, &format!("shuffled lexicon: {} words, {} phrases", ls.single.len(), ls.phrases.len()));
            results.push(measure_lex(&x, &ls, "many pairs, shuffled verse pairs (control)", true, &t0)?);
        }
    }
    let lic: BTreeMap<String, String> = names.iter().map(|n| (n.clone(), licence(ar.ebible.as_deref(), n))).collect();
    let w = Written { ar: &ar, names: &names, measured: &measured, train_of: &train_of, train: &train, lic: &lic, skipped: &skipped, dropped: &dropped_ch, held: &held, chosen: &chosen, tag: &tag, pools: &pools, in_dev: &in_dev, train_blocks: train_blocks.len(), dev_blocks: dev_blocks.len(), books_of: &books_of };
    write(&w, &results)?;
    lap(&t0, "report written");
    Ok(())
}

/// The meta-world of the whole canon outside the test books: every block of every translation read through the given
/// lexicon, stored once (`block_meta`), streamed block by block. Only the size and the memory are reported.
fn canon(ar: &Args, a: &Annotator, names: &[String], pools: &BTreeMap<String, BTreeMap<VKey, String>>, all: &[Block], group: &(dyn Fn(VKey) -> (u8, u16) + Sync), t0: &std::time::Instant) -> Result<()> {
    let mut lex = Lexicon::from_tsv(&std::fs::read_to_string(&ar.lexicon_from)?, 4);
    lex.coarse = true;
    lex.flat = true;
    let blocks: Vec<&Block> = all.iter().filter(|b| bible::split_of(b.book) != Split::Test).collect();
    let mut it = Interner::default();
    let mut total = MetaSize::default();
    let mut rows = String::from("block\ttranslations\tverses\twords\ttokens\tconcepts\tedges\tconcept_members\tedge_members\tbytes\n");
    let ts: Vec<usize> = (0..names.len()).collect();
    for (bi, b) in blocks.iter().enumerate() {
        let parsed: Vec<Result<Corpus>> = par_map(&ts, ar.threads, &|&t| corpus::parse_grouped(a, &names[t], Kind::Bible, &pools[&names[t]], &|k| b.contains(k), group, 1, false));
        let mut metas: Vec<(usize, BTreeMap<VKey, Vec<MTok>>)> = Vec::new();
        let (mut verses, mut words) = (0u64, 0u64);
        for (t, c) in ts.iter().zip(parsed) {
            let c = c?;
            if c.verses.is_empty() {
                continue;
            }
            verses += c.verses.len() as u64;
            words += c.verses.values().map(|v| v.text.split_whitespace().count() as u64).sum::<u64>();
            metas.push((*t, c.verses.iter().map(|(k, v)| (*k, lex.read(v.meta_words()))).collect()));
        }
        let ms: Vec<(usize, &BTreeMap<VKey, Vec<MTok>>)> = metas.iter().map(|(t, v)| (*t, v)).collect();
        let s = block_meta(&mut it, &ms);
        let _ = writeln!(rows, "{}\t{}\t{verses}\t{words}\t{}\t{}\t{}\t{}\t{}\t{}", b.name(), metas.len(), s.tokens, s.concepts, s.edges, s.c_members, s.e_members, s.bytes());
        total.add(&s);
        if bi % 10 == 0 || bi + 1 == blocks.len() {
            lap(t0, &format!("canon: block {} of {} ({}): {} translations, {} concepts, {} edges; interned {}", bi + 1, blocks.len(), b.name(), metas.len(), s.concepts, s.edges, it.ids.len()));
        }
    }
    std::fs::write(ar.out.join("canon-blocks.tsv"), rows)?;
    let interned: u64 = it.ids.keys().map(|k| k.len() as u64 + 4 + 48).sum();
    let r = format!(
        "# The meta-world of the canon (training and dev books)\n\n- blocks: {} of {} (the test books are not read)\n- meta tokens of all translations: {}\n- distinct concepts per block, summed: {} (largest block {}); edges: {} (largest {})\n- translation memberships: concepts {}, edges {} (a concept is held by {:.1} translations on average, an edge by {:.1})\n- interned concept ids, all blocks: {} ({:.1} MB with their strings)\n- stored meta-world, all blocks: {:.1} MB; per block {:.2} MB\n- peak memory: {:.2} GB\n",
        total.blocks,
        all.len(),
        total.tokens,
        total.concepts,
        total.max_concepts,
        total.edges,
        total.max_edges,
        total.c_members,
        total.e_members,
        total.c_members as f64 / total.concepts.max(1) as f64,
        total.e_members as f64 / total.edges.max(1) as f64,
        it.ids.len(),
        interned as f64 / 1e6,
        total.bytes() as f64 / 1e6,
        total.bytes() as f64 / 1e6 / total.blocks.max(1) as f64,
        peak_gb()
    );
    std::fs::write(ar.out.join("canon.md"), r)?;
    lap(t0, "canon written");
    Ok(())
}

/// What the report says about the run besides the measurements.
struct Written<'a> {
    ar: &'a Args,
    names: &'a [String],
    measured: &'a [usize],
    train_of: &'a BTreeMap<usize, usize>,
    train: &'a [Corpus],
    lic: &'a BTreeMap<String, String>,
    skipped: &'a [(String, String)],
    dropped: &'a BTreeMap<String, Vec<((u8, u16), f64, f64)>>,
    held: &'a BTreeSet<usize>,
    chosen: &'a BTreeMap<(usize, usize), String>,
    tag: &'a BTreeMap<(usize, usize), f64>,
    pools: &'a BTreeMap<String, BTreeMap<VKey, String>>,
    in_dev: &'a dyn Fn(VKey) -> bool,
    train_blocks: usize,
    dev_blocks: usize,
    /// every translation's books, before the versification check
    books_of: &'a BTreeMap<String, BTreeSet<u8>>,
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 }
}

fn write(w: &Written, results: &[Measured]) -> Result<()> {
    let (ar, names, held) = (w.ar, w.names, w.held);
    let mut r = String::new();
    let role = |t: usize| -> String {
        let mut v = Vec::new();
        if ar.hubs.contains(&names[t]) {
            v.push("hub");
        }
        if held.contains(&t) {
            v.push("held out of the lexicon");
        }
        if ar.old.contains(&names[t]) {
            v.push("one of the four");
        }
        v.join(", ")
    };
    let _ = writeln!(r, "# Many translations, each in its own world, through the meta-world (dev)\n");
    let _ = writeln!(r, "Blocks of whole chapters up to {} words, cut on {}: {} training blocks (one in {} of the training books), {} dev blocks (JOS, NEH, EZK, ZEP; 1CO, TIT, REV). The test books are not read.\n", ar.max_words, ar.reference, w.train_blocks, ar.train_every, w.dev_blocks);
    let _ = writeln!(r, "## Translations\n\n| translation | licence | verses of the canon | in the training blocks | in the dev blocks | chapters left out | role |\n|---|---|---:|---:|---:|---:|---|");
    for t in w.measured.iter().copied().chain(w.train_of.keys().copied().filter(|t| !w.measured.contains(t))) {
        let n = &names[t];
        let _ = writeln!(r, "| {n} | {} | {} | {} | {} | {} | {} |", w.lic[n], w.pools[n].len(), w.train_of.get(&t).map_or(0, |&i| w.train[i].verses.len()), w.pools[n].keys().filter(|k| (w.in_dev)(**k)).count(), w.dropped.get(n).map_or(0, Vec::len), if w.measured.contains(&t) { role(t) } else { "learns only".into() });
    }
    let _ = writeln!(r, "\nSkipped or not measured:\n");
    for (n, why) in w.skipped {
        let _ = writeln!(r, "- {n} ({}): {why}", w.lic[n]);
    }
    let _ = writeln!(r, "\nChapters left out by the versification check (a verse shares more words with the reference's neighbours than with its own number):\n");
    for (n, d) in w.dropped {
        if d.is_empty() {
            continue;
        }
        let mut by: BTreeMap<&str, Vec<u16>> = BTreeMap::new();
        for ((b, c), _, _) in d {
            by.entry(bible::code(*b)).or_default().push(*c);
        }
        let _ = writeln!(r, "- {n} ({} chapters): {}", d.len(), by.iter().map(|(b, cs)| format!("{b} {}", cs.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join("; "));
    }
    let _ = writeln!(r, "\n## The lexicon's pairs\n\n{} pairs: each translation with the hubs ({}) and its {} nearest by the baseline's concept agreement on the training blocks.\n", w.chosen.len(), ar.hubs.join(", "), ar.near);
    for ((i, j), why) in w.chosen {
        let _ = writeln!(r, "- {} – {}: {why}; training agreement {:.3}", names[*i], names[*j], w.tag.get(&(*i, *j)).copied().unwrap_or(f64::NAN));
    }
    let in_held = |p: &(String, String)| held.iter().any(|&h| names[h] == p.0 || names[h] == p.1);
    let groups: [(&str, &dyn Fn(&(String, String)) -> bool); 4] = [
        ("all pairs", &|_| true),
        ("both in the lexicon", &|p| !in_held(p)),
        ("with a held-out translation", &|p| in_held(p)),
        ("the four authors", &|p| ar.old.contains(&p.0) && ar.old.contains(&p.1)),
    ];
    let _ = writeln!(r, "\n## Between worlds, per block (dev)\n\nMeans over the ordered pairs. *Concepts*, *edges*: the share two meta-worlds share. *Logic*: events matched, roles shared, events in the same order, contradictions per 1 000 words (through the lexicon; *raw*: the worlds compared word for word). *Cross*: the bits to give the target back from the source's meta-world (reader + the target's encoder and stack), per block and per 1 000 words; *reader*: the meta-world script alone. *Shuffled*: each target verse read against another verse of its block.\n");
    let _ = writeln!(r, "| lexicon | pairs | n | concepts | edges | events | roles | order | contradictions / 1 000 | raw contradictions / 1 000 | logical distance / 1 000 | cross / block | cross / 1 000 | reader / 1 000 | shuffled: concepts | shuffled: events | shuffled: contradictions / 1 000 | shuffled: logical distance / 1 000 | shuffled: cross / 1 000 |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for m in results {
        for (g, f) in &groups {
            let ps: Vec<&PairM> = m.pairs.iter().filter(|(k, p)| f(k) && p.verses > 0).map(|(_, p)| p).collect();
            let sh: Vec<&PairM> = m.shuffled.iter().filter(|(k, p)| f(k) && p.verses > 0).map(|(_, p)| p).collect();
            let c = |h: &dyn Fn(&PairM) -> f64| mean(&ps.iter().map(|p| h(p)).collect::<Vec<_>>());
            let s = |h: &dyn Fn(&PairM) -> f64| mean(&sh.iter().map(|p| h(p)).collect::<Vec<_>>());
            let _ = writeln!(
                r,
                "| {} | {g} | {} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.2} | {:.2} | {:.1} | {:.0} | {:.1} | {:.1} | {:.4} | {:.4} | {:.2} | {:.1} | {:.1} |",
                m.name,
                ps.len(),
                c(&|p| p.agree.concepts()),
                c(&|p| p.agree.edges()),
                c(&|p| p.logic.events()),
                c(&|p| p.logic.role_share()),
                c(&|p| p.logic.order()),
                c(&|p| p.contra_kw()),
                c(&|p| p.contra_raw_kw()),
                c(&|p| p.dist_kw()),
                c(&|p| p.per_block()),
                c(&|p| p.per_kw()),
                c(&|p| p.reader_kw()),
                s(&|p| p.agree.concepts()),
                s(&|p| p.logic.events()),
                s(&|p| p.contra_kw()),
                s(&|p| p.dist_kw()),
                s(&|p| p.per_kw()),
            );
        }
    }
    // before → after per pair
    if results.len() >= 2 {
        let (b, a) = (&results[0], &results[1]);
        let mut closer = (0, 0, 0, 0);
        for (k, pa) in &a.pairs {
            let Some(pb) = b.pairs.get(k) else { continue };
            if pa.verses == 0 {
                continue;
            }
            closer.0 += (pa.dist_kw() < pb.dist_kw()) as u32;
            closer.1 += (pa.dist_kw() > pb.dist_kw()) as u32;
            closer.2 += (pa.per_kw() < pb.per_kw()) as u32;
            closer.3 += (pa.agree.concepts() > pb.agree.concepts()) as u32;
        }
        let _ = writeln!(r, "\nBefore → after, per ordered pair ({} → {}): logically closer (distance) {} pairs, farther {}; fewer cross bits {}; more concepts shared {} (every pair in `pairs.tsv`).", b.name, a.name, closer.0, closer.1, closer.2, closer.3);
    }
    let _ = writeln!(r, "\n## Sensitivity of the logical distance (dev)\n\nA control: into every target verse of every ordered pair one change of meaning is put, of one kind at a time, and the comparison with the source verse is made again. «Contradiction» counts the changes the contradictions catch (what the logical distance counts); «any» the changes any part of the comparison catches (contradictions, gaps, shared roles, order). A verse with nothing of a kind to change is not counted for that kind.\n\n| lexicon | change | verses changed | caught as contradiction | caught by any part |\n|---|---|---:|---:|---:|");
    for m in results {
        let mut s = logic::Sens::default();
        for p in m.pairs.values() {
            s.add(&p.sens);
        }
        for (k, (n, c, a)) in s.0.iter().enumerate() {
            let pc = |x: u64| 100.0 * x as f64 / (*n).max(1) as f64;
            let _ = writeln!(r, "| {} | {} | {n} | {:.1}% | {:.1}% |", m.name, logic::PERTURB[k], pc(*c), pc(*a));
        }
    }
    let _ = writeln!(r, "\n## «No match» (dev)\n\nThe aligner never forces a match. A verse of the same number whose script would cost more than writing the target verse from nothing is «no match» (structural: not forced); a verse one side does not have is «no match» of its level (canonical: the book or passage is not in that canon or edition; structural: the chapter is numbered another way, or the verse is joined to another); inside a matched verse, a target meta token nothing in the source stands for is lexical (a function word, a mark) or semantic (a content word). Summed over all ordered pairs; the shuffled control pairs each verse with another of its block.\n");
    let _ = writeln!(r, "| lexicon | verse pairs | not forced | not forced, shuffled | no match: lexical | semantic | structural | canonical | shuffled: lexical | semantic | structural | canonical | source side: lexical | semantic | structural | canonical |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for m in results {
        let sum = |ps: &BTreeMap<(String, String), PairM>, f: &dyn Fn(&PairM) -> u64| ps.values().map(f).sum::<u64>();
        let vp = sum(&m.pairs, &|p| p.verses);
        let _ = writeln!(
            r,
            "| {} | {vp} | {} ({:.2}%) | {} ({:.2}%) | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            m.name,
            sum(&m.pairs, &|p| p.unaligned),
            100.0 * sum(&m.pairs, &|p| p.unaligned) as f64 / vp.max(1) as f64,
            sum(&m.shuffled, &|p| p.unaligned),
            100.0 * sum(&m.shuffled, &|p| p.unaligned) as f64 / sum(&m.shuffled, &|p| p.verses).max(1) as f64,
            sum(&m.pairs, &|p| p.nomatch[0]),
            sum(&m.pairs, &|p| p.nomatch[1]),
            sum(&m.pairs, &|p| p.nomatch[2]),
            sum(&m.pairs, &|p| p.nomatch[3]),
            sum(&m.shuffled, &|p| p.nomatch[0]),
            sum(&m.shuffled, &|p| p.nomatch[1]),
            sum(&m.shuffled, &|p| p.nomatch[2]),
            sum(&m.shuffled, &|p| p.nomatch[3]),
            sum(&m.pairs, &|p| p.nomatch_src[0]),
            sum(&m.pairs, &|p| p.nomatch_src[1]),
            sum(&m.pairs, &|p| p.nomatch_src[2]),
            sum(&m.pairs, &|p| p.nomatch_src[3]),
        );
    }
    let _ = writeln!(r, "\n## The links of each world to the meta-world, and the gate (dev)\n\n*Up*: the script from a translation's own world to its meta-world (what the lexicon changes); *down*: from the meta-world back to the world (what the meta-world does not hold of it); *autoencoder*: the encoder and stack of the translation's own verses; all per 1 000 words, mean over the translations.\n");
    let _ = writeln!(r, "| lexicon | words | phrases | merges refused by the logic gate | overrides refused by the logic gate | standard tokens | up / 1 000 | down / 1 000 | autoencoder / 1 000 | autoencoder gate | whole split code: gate | largest gap to reader + tail | gate control (damaged residual) | meta-world per block: concepts, edges | stored per block | peak memory |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---:|---|---|---:|---:|");
    for m in results {
        let kw = |f: &dyn Fn(&Own) -> f64| mean(&m.own.values().map(|o| 1000.0 * f(o) / o.words.max(1) as f64).collect::<Vec<_>>());
        let (v, ok) = m.own.values().fold((0, 0), |acc, o| (acc.0 + o.verses, acc.1 + o.gate));
        let nb = m.size.blocks.max(1) as f64;
        let _ = writeln!(
            r,
            "| {} | {} | {} | {} | {} | {} | {:.1} | {:.1} | {:.1} | {ok} of {v} | {} of {} | {:.6} | {} of {} pass | {:.0}, {:.0} | {:.2} MB | {:.2} GB |",
            m.name,
            m.lex_size.0,
            m.lex_size.1,
            m.refused_merges,
            m.refused_overrides.len(),
            m.std_tokens,
            kw(&|o| o.up),
            kw(&|o| o.down),
            kw(&|o| o.tail),
            m.whole.1,
            m.whole.0,
            m.whole.2,
            m.control.1,
            m.control.0,
            m.size.concepts as f64 / nb,
            m.size.edges as f64 / nb,
            m.size.bytes() as f64 / 1e6 / nb,
            m.peak
        );
    }
    for m in results {
        let _ = writeln!(r, "\nContradictions of {} on the first dev block (why):\n", m.name);
        for e in &m.examples {
            let _ = writeln!(r, "- {e}");
        }
    }
    traditions(&mut r, w, results);
    let _ = writeln!(r, "\n## Per translation (dev)\n\nMean over its pairs: concept agreement, contradictions per 1 000 words, cross bits per 1 000 words into it; its links and autoencoder per 1 000 words.\n");
    let mut head = String::from("| translation | role |");
    let mut line = String::from("|---|---|");
    for m in results {
        head.push_str(&format!(" {0}: concepts | {0}: contradictions | {0}: cross into | {0}: up | {0}: down | {0}: autoencoder |", m.name));
        line.push_str("---:|---:|---:|---:|---:|---:|");
    }
    let _ = writeln!(r, "{head}\n{line}");
    for &t in w.measured {
        let n = &names[t];
        let mut row = format!("| {n} | {} |", role(t));
        for m in results {
            let ps: Vec<&PairM> = m.pairs.iter().filter(|(k, p)| (k.0 == *n || k.1 == *n) && p.verses > 0).map(|(_, p)| p).collect();
            let into: Vec<f64> = m.pairs.iter().filter(|(k, p)| k.1 == *n && p.verses > 0).map(|(_, p)| p.per_kw()).collect();
            let o = m.own.get(n).cloned().unwrap_or_default();
            let kw = |x: f64| 1000.0 * x / o.words.max(1) as f64;
            row.push_str(&format!(" {:.4} | {:.2} | {:.1} | {:.1} | {:.1} | {:.1} |", mean(&ps.iter().map(|p| p.agree.concepts()).collect::<Vec<_>>()), mean(&ps.iter().map(|p| p.contra_kw()).collect::<Vec<_>>()), mean(&into), kw(o.up), kw(o.down), kw(o.tail)));
        }
        let _ = writeln!(r, "{row}");
    }
    let _ = writeln!(r, "\n## Per block (dev)\n\n| lexicon | block | reference words | cross / 1 000 | concepts | contradictions / 1 000 | meta-world concepts | edges | tokens of all translations | stored |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|");
    for m in results {
        for (b, wd, c, a, k, s) in &m.per_block {
            let _ = writeln!(r, "| {} | {b} | {wd} | {c:.1} | {a:.4} | {k:.2} | {} | {} | {} | {:.2} MB |", m.name, s.concepts, s.edges, s.tokens, s.bytes() as f64 / 1e6);
        }
    }
    std::fs::write(ar.out.join("report.md"), r)?;
    squares(w, results)?;
    // every ordered pair
    let mut t = String::from("lexicon\tsource\ttarget\tblocks\tverses\twords\treader_bits\ttail_bits\tcross_per_kw\tcross_per_block\tconcepts\tedges\tevents\troles\torder\tcontra_per_kw\traw_contra_per_kw\tdistance_per_kw\tshuffled_cross_per_kw\tshuffled_concepts\tshuffled_contra_per_kw\tshuffled_distance_per_kw\n");
    for m in results {
        for (k, p) in &m.pairs {
            let s = &m.shuffled[k];
            let _ = writeln!(
                t,
                "{}\t{}\t{}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{:.2}\t{:.1}\t{:.5}\t{:.5}\t{:.5}\t{:.5}\t{:.5}\t{:.3}\t{:.3}\t{:.3}\t{:.2}\t{:.5}\t{:.3}\t{:.3}",
                m.name,
                k.0,
                k.1,
                p.blocks,
                p.verses,
                p.words,
                p.reader,
                p.tail,
                p.per_kw(),
                p.per_block(),
                p.agree.concepts(),
                p.agree.edges(),
                p.logic.events(),
                p.logic.role_share(),
                p.logic.order(),
                p.contra_kw(),
                p.contra_raw_kw(),
                p.dist_kw(),
                s.per_kw(),
                s.agree.concepts(),
                s.contra_kw(),
                s.dist_kw()
            );
        }
    }
    std::fs::write(ar.out.join("pairs.tsv"), t)?;
    let mut t = String::from("lexicon\ttranslation\tverses\twords\ttail_bits\tgate_ok\tup_bits\tdown_bits\tdelta_overrides\n");
    for m in results {
        for (n, o) in &m.own {
            let _ = writeln!(t, "{}\t{n}\t{}\t{}\t{:.1}\t{}\t{:.1}\t{:.1}\t{}", m.name, o.verses, o.words, o.tail, o.gate, o.up, o.down, o.overrides);
        }
    }
    std::fs::write(ar.out.join("worlds.tsv"), t)?;
    Ok(())
}

/// The canonical tradition an edition serves, by eBible's description of it: the Jewish (JPS 1917, Leeser's), the
/// Catholic (Douay-Rheims, the WEB's Catholic edition), the Orthodox (the Septuagint's: Brenton, the LXX2012s, the
/// LXX Updated), and the Protestant (the rest; the Messianic editions keep the Protestant canon).
pub fn tradition(name: &str) -> &'static str {
    match name {
        "engjps" | "englee" => "Jewish",
        "engDRA" | "eng-web-c" => "Catholic",
        "eng-Brenton" | "eng-lxx2012" | "eng-uk-lxx2012" | "englxxup" => "Orthodox",
        _ => "Protestant",
    }
}

pub const TRADITIONS: [&str; 4] = ["Jewish", "Catholic", "Orthodox", "Protestant"];

/// A leaf order by average-linkage clustering (UPGMA) of a symmetric distance: the two nearest clusters join, left
/// before right, ties to the lower index. Similar translations stand side by side in the square.
pub fn upgma(items: &[usize], d: &dyn Fn(usize, usize) -> f64) -> Vec<usize> {
    let mut cl: Vec<Vec<usize>> = items.iter().map(|&i| vec![i]).collect();
    while cl.len() > 1 {
        let mut best = (f64::INFINITY, 0, 1);
        for a in 0..cl.len() {
            for b in a + 1..cl.len() {
                let s: f64 = cl[a].iter().flat_map(|&x| cl[b].iter().map(move |&y| (x, y))).map(|(x, y)| d(x, y)).sum::<f64>() / (cl[a].len() * cl[b].len()) as f64;
                if s < best.0 {
                    best = (s, a, b);
                }
            }
        }
        let right = cl.remove(best.2);
        cl[best.1].extend(right);
    }
    cl.pop().unwrap_or_default()
}

/// The measured translations in the square's order: by tradition, then clustered by the cross bits of the last
/// lexicon (both ways, averaged).
fn square_order(w: &Written, m: &Measured) -> Vec<usize> {
    let d = |i: usize, j: usize| {
        let g = |a: usize, b: usize| m.pairs.get(&(w.names[a].clone(), w.names[b].clone())).filter(|p| p.verses > 0).map(|p| p.per_kw());
        match (g(i, j), g(j, i)) {
            (Some(x), Some(y)) => (x + y) / 2.0,
            (Some(x), None) | (None, Some(x)) => x,
            _ => 1e9,
        }
    };
    TRADITIONS.iter().flat_map(|t| upgma(&w.measured.iter().copied().filter(|&i| tradition(&w.names[i]) == *t).collect::<Vec<_>>(), &d)).collect()
}

/// The canons and the traditions: the books each translation has, and every measure between traditions.
fn traditions(r: &mut String, w: &Written, results: &[Measured]) {
    let _ = writeln!(r, "\n## Canons (the books each translation has; read from the pools, no text measured)\n\n| tradition | translation | Hebrew Bible (of 39) | New Testament (of 27) | other books |\n|---|---|---:|---:|---|");
    for t in TRADITIONS {
        for n in w.names.iter().filter(|n| tradition(n) == t) {
            let b = &w.books_of[n];
            let ot = b.iter().filter(|&&x| (x as usize) < bible::BOOKS.len()).count();
            let nt = b.iter().filter(|&&x| (x as usize) >= bible::BOOKS.len() && (x as usize) < bible::BOOKS.len() + bible::NT.len()).count();
            let dc: Vec<&str> = b.iter().filter(|&&x| (x as usize) >= bible::BOOKS.len() + bible::NT.len()).map(|&x| bible::code(x)).collect();
            let _ = writeln!(r, "| {t} | {n} | {ot} | {nt} | {} |", dc.join(" "));
        }
    }
    let _ = writeln!(r, "\n## Within and between traditions (dev)\n\nOrdered pairs grouped by the source's and the target's tradition; means over the pairs. Bits by level per 1 000 target words; «no match» marks summed (lexical, semantic: meta tokens of the target nothing in the source stands for; structural, canonical: verses). Canonical bits and marks include whole blocks the source lacks, which the cross residual leaves out.\n");
    let _ = writeln!(r, "| lexicon | source → target | pairs | concepts | logical distance / 1 000 | cross / 1 000 | lexical | semantic | structural | canonical | no match: lexical | semantic | structural | canonical | verse pairs not forced |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for m in results {
        for a in TRADITIONS {
            for b in TRADITIONS {
                let ps: Vec<&PairM> = m.pairs.iter().filter(|(k, _)| tradition(&k.0) == a && tradition(&k.1) == b).map(|(_, p)| p).collect();
                if ps.is_empty() {
                    continue;
                }
                let have: Vec<&PairM> = ps.iter().copied().filter(|p| p.verses > 0).collect();
                let c = |f: &dyn Fn(&PairM) -> f64| mean(&have.iter().map(|p| f(p)).collect::<Vec<_>>());
                let lv = |l: usize| mean(&ps.iter().filter(|p| p.words + p.absent_words > 0).map(|p| 1000.0 * p.lv_bits[l] / (p.words + p.absent_words) as f64).collect::<Vec<_>>());
                let nm = |l: usize| ps.iter().map(|p| p.nomatch[l]).sum::<u64>();
                let _ = writeln!(r, "| {} | {a} → {b} | {} | {:.4} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {} | {} | {} | {} | {} |", m.name, ps.len(), c(&|p| p.agree.concepts()), c(&|p| p.dist_kw()), c(&|p| p.per_kw()), lv(0), lv(1), lv(2), lv(3), nm(0), nm(1), nm(2), nm(3), ps.iter().map(|p| p.unaligned).sum::<u64>());
            }
        }
    }
}

/// The all-to-all square: every translation against every other (row the source, column the target), for each
/// lexicon and measure, as TSV (`squares/`), Markdown (`square.md`) and a heat map (`square.html`).
fn squares(w: &Written, results: &[Measured]) -> Result<()> {
    let Some(last) = results.last() else { return Ok(()) };
    let order = square_order(w, last);
    let dir = w.ar.out.join("squares");
    std::fs::create_dir_all(&dir)?;
    type Measure = (&'static str, fn(&PairM) -> Option<f64>);
    let measures: Vec<Measure> = vec![
        ("cross", |p| (p.verses > 0).then(|| p.per_kw())),
        ("distance", |p| (p.verses > 0).then(|| p.dist_kw())),
        ("contradictions", |p| (p.verses > 0).then(|| p.contra_kw())),
        ("half-gaps", |p| (p.verses > 0).then(|| 500.0 * p.logic.gaps() as f64 / p.words.max(1) as f64)),
        ("concepts", |p| (p.verses > 0).then(|| p.agree.concepts())),
        ("lexical", |p| (p.words + p.absent_words > 0).then(|| 1000.0 * p.lv_bits[LEXICAL] / (p.words + p.absent_words) as f64)),
        ("semantic", |p| (p.words + p.absent_words > 0).then(|| 1000.0 * p.lv_bits[SEMANTIC] / (p.words + p.absent_words) as f64)),
        ("structural", |p| (p.words + p.absent_words > 0).then(|| 1000.0 * p.lv_bits[STRUCTURAL] / (p.words + p.absent_words) as f64)),
        ("canonical", |p| (p.words + p.absent_words > 0).then(|| 1000.0 * p.lv_bits[CANONICAL] / (p.words + p.absent_words) as f64)),
        ("nomatch-lexical", |p| Some(p.nomatch[LEXICAL] as f64)),
        ("nomatch-semantic", |p| Some(p.nomatch[SEMANTIC] as f64)),
        ("nomatch-structural", |p| Some(p.nomatch[STRUCTURAL] as f64)),
        ("nomatch-canonical", |p| Some(p.nomatch[CANONICAL] as f64)),
    ];
    let slug = |s: &str| s.split(|c: char| !c.is_alphanumeric()).filter(|x| !x.is_empty()).collect::<Vec<_>>().join("-");
    let cell = |m: &Measured, i: usize, j: usize, f: fn(&PairM) -> Option<f64>| -> Option<f64> { if i == j { None } else { m.pairs.get(&(w.names[i].clone(), w.names[j].clone())).and_then(f) } };
    for m in results {
        for (mn, f) in &measures {
            let mut t = String::from("source\\target");
            for &j in &order {
                t.push('\t');
                t.push_str(&w.names[j]);
            }
            t.push('\n');
            for &i in &order {
                t.push_str(&w.names[i]);
                for &j in &order {
                    t.push('\t');
                    if let Some(v) = cell(m, i, j, *f) {
                        let _ = write!(t, "{v:.3}");
                    }
                }
                t.push('\n');
            }
            std::fs::write(dir.join(format!("{}-{mn}.tsv", slug(&m.name))), t)?;
        }
    }
    // Markdown: cross / distance in each cell, for the last lexicon
    let mut md = format!("# All against all ({}, dev)\n\nRow: the source; column: the target. Each cell: the cross bits per 1 000 target words through the meta-world / the logical distance per 1 000 words. Ordered by tradition (J Jewish, C Catholic, O Orthodox, P Protestant), then clustered. Empty: no dev block in common.\n\n| source \\ target |", last.name);
    for &j in &order {
        let _ = write!(md, " {} ({}) |", w.names[j], &tradition(&w.names[j])[..1]);
    }
    md.push_str("\n|---|");
    md.push_str(&"---:|".repeat(order.len()));
    md.push('\n');
    for &i in &order {
        let _ = write!(md, "| **{}** ({}) |", w.names[i], &tradition(&w.names[i])[..1]);
        for &j in &order {
            match (cell(last, i, j, measures[0].1), cell(last, i, j, measures[1].1)) {
                (Some(c), Some(d)) => {
                    let _ = write!(md, " {c:.0} / {d:.0} |");
                }
                _ => md.push_str(" |"),
            }
        }
        md.push('\n');
    }
    std::fs::write(w.ar.out.join("square.md"), md)?;
    // the heat map: one square per measure of the last lexicon, the cell's colour by its rank among the cells
    let mut h = String::from("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>Translations square</title><style>:root{--bg:#fff;--fg:#1a1a1a;--line:#ddd}@media (prefers-color-scheme: dark){:root{--bg:#141414;--fg:#e8e8e8;--line:#333}}body{background:var(--bg);color:var(--fg);font:13px system-ui,sans-serif;margin:16px}table{border-collapse:collapse;margin:12px 0 28px}td,th{border:1px solid var(--line);padding:2px 4px;text-align:right;white-space:nowrap}th{position:sticky;top:0;background:var(--bg);font-weight:600}th.r{position:sticky;left:0;text-align:left}.wrap{overflow:auto;max-width:100%}</style></head><body>");
    let _ = write!(h, "<h1>All against all</h1><p>{} lexicon, dev blocks. Row: the source; column: the target. Ordered by tradition, then clustered. The colour is the cell's rank among the square's cells, light (near) to dark (far); hover a cell for all its measures.</p>", last.name);
    for (mn, f) in measures.iter().filter(|(n, _)| matches!(*n, "cross" | "distance" | "lexical" | "semantic" | "structural" | "canonical" | "nomatch-canonical")) {
        let mut vals: Vec<f64> = order.iter().flat_map(|&i| order.iter().filter_map(move |&j| cell(last, i, j, *f))).collect();
        vals.sort_by(f64::total_cmp);
        let rank = |v: f64| if vals.len() < 2 { 0.0 } else { vals.partition_point(|x| *x < v) as f64 / (vals.len() - 1) as f64 };
        let _ = write!(h, "<h2>{mn}</h2><div class=\"wrap\"><table><tr><th class=\"r\">source \\ target</th>");
        for &j in &order {
            let _ = write!(h, "<th title=\"{}\">{}</th>", tradition(&w.names[j]), w.names[j]);
        }
        h.push_str("</tr>");
        for &i in &order {
            let _ = write!(h, "<tr><th class=\"r\">{} ({})</th>", w.names[i], &tradition(&w.names[i])[..1]);
            for &j in &order {
                match cell(last, i, j, *f) {
                    Some(v) => {
                        let q = rank(v);
                        let all: Vec<String> = measures.iter().filter_map(|(n, g)| cell(last, i, j, *g).map(|x| format!("{n} {x:.1}"))).collect();
                        let _ = write!(h, "<td style=\"background:hsl(210 70% {:.0}%);color:{}\" title=\"{} → {}: {}\">{v:.0}</td>", 95.0 - 60.0 * q, if q > 0.6 { "#fff" } else { "#000" }, w.names[i], w.names[j], all.join(", "));
                    }
                    None => h.push_str("<td></td>"),
                }
            }
            h.push_str("</tr>");
        }
        h.push_str("</table></div>");
    }
    h.push_str("</body></html>\n");
    std::fs::write(w.ar.out.join("square.html"), h)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::Tag;

    #[test]
    fn clustering_puts_near_ones_together() {
        // 0 and 2 are near, 1 and 3 are near
        let d = |a: usize, b: usize| if (a % 2) == (b % 2) { 1.0 } else { 9.0 };
        let o = upgma(&[0, 1, 2, 3], &d);
        let at = |x: usize| o.iter().position(|&y| y == x).unwrap();
        assert_eq!(at(0).abs_diff(at(2)), 1);
        assert_eq!(at(1).abs_diff(at(3)), 1);
        assert_eq!(tradition("engDRA"), "Catholic");
        assert_eq!(tradition("eng-kjv"), "Protestant");
    }

    fn m(c: &[(&str, usize, &str)]) -> Vec<MTok> {
        c.iter().map(|(x, h, r)| MTok { concept: x.to_string(), tag: Tag::NN, head: *h, rel: Rel::parse(r).unwrap(), span: (0, 0) }).collect()
    }

    #[test]
    fn agreement_and_its_control() {
        let a = m(&[("god", 2, "nsubj"), ("say", 0, "root"), ("light", 2, "obj"), (".", 3, "punct")]);
        let b = m(&[("god", 2, "nsubj"), ("say", 0, "root"), ("dark", 2, "obj")]);
        let g = agree(&a, &a);
        assert_eq!((g.concepts(), g.edges()), (1.0, 1.0));
        let g = agree(&a, &b);
        // punctuation left out: 2 of 3 concepts, 2 of 3 edges
        assert!((g.concepts() - 2.0 / 3.0).abs() < 1e-9 && (g.edges() - 2.0 / 3.0).abs() < 1e-9);
        // the control pairs a verse with another of its block
        let ks = [(5, 1, 1), (5, 1, 2), (5, 1, 3), (5, 2, 1)];
        assert_eq!(pairing(&ks, true), vec![((5, 1, 3), (5, 1, 1)), ((5, 2, 1), (5, 1, 2)), ((5, 1, 1), (5, 1, 3)), ((5, 1, 2), (5, 2, 1))]);
        assert!(pairing(&ks, false).iter().all(|(x, y)| x == y));
    }

    #[test]
    fn a_block_meta_world_is_stored_once() {
        let (a, b) = (m(&[("god", 2, "nsubj"), ("say", 0, "root")]), m(&[("god", 2, "nsubj"), ("speak", 0, "root")]));
        let va: BTreeMap<VKey, Vec<MTok>> = [((5, 1, 1), a)].into();
        let vb: BTreeMap<VKey, Vec<MTok>> = [((5, 1, 1), b)].into();
        let mut it = Interner::default();
        // two translations with the same meta-world: each concept and edge once, held by both
        let s = block_meta(&mut it, &[(0, &va), (1, &va)]);
        assert_eq!((s.concepts, s.edges, s.c_members, s.tokens), (2, 2, 4, 4));
        // negative control: a different verb is a concept and two edges more
        let s = block_meta(&mut it, &[(0, &va), (1, &vb)]);
        assert_eq!((s.concepts, s.edges, s.c_members), (3, 4, 4));
    }

    #[test]
    fn versification_drops_a_shifted_chapter() {
        let r: BTreeMap<VKey, String> = [((18, 10, 1), "Lord stand afar"), ((18, 10, 2), "wicked pride persecute poor"), ((18, 10, 3), "wicked boasteth heart desire"), ((18, 11, 1), "trust Lord soul flee mountain"), ((18, 11, 2), "wicked bend bow arrow string")].iter().map(|(k, s)| (*k, s.to_string())).collect();
        // the same chapter 10, and chapter 11 numbered one on (as the Septuagint's Psalms are)
        let mut t = r.clone();
        t.insert((18, 11, 1), "Lord stand afar".into());
        t.insert((18, 11, 2), "trust Lord soul flee mountain".into());
        let (keep, dropped) = versification(&t, &r);
        assert!(keep.contains(&(18, 10)));
        assert!(!keep.contains(&(18, 11)) && dropped.len() == 1);
    }
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn empty_ties_and_accounting() {
        assert!(pairing(&[], true).is_empty());
        let k = [(0, 1, 1)];
        assert_eq!(pairing(&k, true), vec![(k[0], k[0])]);
        assert_eq!(upgma(&[3, 1, 2], &|_, _| 1.0), vec![3, 1, 2]);
        assert!(upgma(&[], &|_, _| 0.0).is_empty());
        assert!(mean(&[]).is_nan());
        assert_eq!(mean(&[2.0, 4.0]), 3.0);
        assert_eq!(Agree::default().concepts(), 0.0);
        let mut p = PairM {
            reader: 2.0,
            tail: 3.0,
            ..Default::default()
        };
        assert_eq!(p.per_kw(), 5000.0);
        p.add(&PairM {
            blocks: 2,
            words: 10,
            reader: 5.0,
            ..Default::default()
        });
        assert_eq!((p.per_kw(), p.per_block()), (1000.0, 5.0));
        assert_eq!(
            MetaSize {
                concepts: 2,
                edges: 3,
                tokens: 4,
                ..Default::default()
            }
            .bytes(),
            107
        );
        let pool = [((0, 1, 1), "short".into()), ((0, 1, 2), "short".into())].into();
        let (keep, drop) = versification(&pool, &pool);
        assert!(keep.is_empty());
        assert_eq!(drop.len(), 1);
    }
}
