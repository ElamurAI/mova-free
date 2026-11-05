//! translate — Mova's deterministic translator through a meta-world, and its byte-for-byte test.
//!
//!   translate eval --pools <dir> --wlc <dir> --prag <prag.tsv> --school <dir> --tales <conllu> --talmud <pool.tsv>
//!                  --out <dir> [--split dev|test] [--threads N] [--sample N]
//!   translate many --pools <dir> --lexicon-from <lexicon.tsv> --out <dir> …   (many translations: `many.rs`)
//!
//! language → tree → pragmatics → world (the standard stack, tools/roundtrip) → reader into the meta-world →
//! meta-world → the standard English encoder with an author's delta → the target's words → the standard stack down
//! to the text. Pools are `school-sents usfm` cuts: `<dir>/{jps,kjv,web,asv}.tsv` (English verse numbers) and
//! `<dir>/jps-heb.tsv` (JPS cut at the Hebrew verse numbers). Writes report.md, examples.md, numbers.tsv, lexicon.tsv,
//! deltas.tsv to `<out>`.

mod bible;
mod blocks;
mod corpus;
mod fwt;
mod heb_enc;
mod hebrew;
mod ibm;
mod logic;
mod many;
mod measure;
mod rules;
mod meta;
mod pages;
mod profile;
mod report;
mod sources;
mod strong;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use en::annotate::Annotator;
use roundtrip::tally::{Diverge, Residual};

use crate::bible::{Split, VKey};
use crate::corpus::{Corpus, PVerse, par_map};
use crate::hebrew::{HVerse, Reader};
use crate::measure::{Out, Run, Src};
use crate::meta::{Lexicon, MTok};
use crate::profile::{Delta, Encoder, Seen, Standard};
use crate::sources::{Kind, Register};

/// The measured outcome of one system over its verses.
#[derive(Default)]
pub struct Agg {
    pub name: String,
    pub group: String,
    pub note: String,
    pub verses: u64,
    pub tokens: u64,
    pub gate_ok: u64,
    pub exact: u64,
    pub chrf: f64,
    pub res: [Residual; 3],
    /// the direct code (meta systems): the target from the normal-mode output
    pub direct: Residual,
    pub direct_ok: u64,
    pub div: Diverge,
    pub fails: Vec<String>,
    pub examples: Vec<(String, String, String, String, Vec<String>)>,
    /// every target verse, for the blocks
    pub recs: BTreeMap<VKey, VRec>,
}

impl Agg {
    fn add(&mut self, key: VKey, src: &str, tgt: &PVerse, o: Out) {
        self.verses += 1;
        self.tokens += tgt.forms.len() as u64;
        self.gate_ok += o.gate_ok as u64;
        self.exact += o.exact as u64;
        self.chrf += o.chrf;
        for s in 0..3 {
            self.res[s].merge(&o.res[s]);
        }
        self.div.merge(&o.div);
        if let Some(d) = &o.direct {
            self.direct.merge(d);
            self.direct_ok += o.direct_ok as u64;
        }
        if let Some(f) = o.fail {
            if self.fails.len() < 4 {
                self.fails.push(format!("{}: {f}", bible::show(key)));
            }
        }
        if self.examples.len() < 6 && (self.verses % 97 == 1) {
            self.examples.push((bible::show(key), src.to_string(), o.text, tgt.text.clone(), o.why));
        }
    }
}

enum From<'a> {
    Meta(&'a Corpus),
    Heb(&'a BTreeMap<VKey, HVerse>, &'a Reader),
    Copy(&'a Corpus),
    Wbw(&'a Corpus, &'a BTreeMap<String, String>),
    Gloss(&'a BTreeMap<VKey, HVerse>),
    /// the target's own text (an autoencoder)
    Itself,
}

struct Sys<'a> {
    name: String,
    group: &'static str,
    note: String,
    from: From<'a>,
    tgt: &'a Corpus,
    enc: Encoder<'a>,
    lex: &'a Lexicon,
    /// pair each target verse with another verse of the source (a control)
    rotate: bool,
    corrupt: bool,
    /// measure only these verses (None — all)
    only: Option<&'a BTreeSet<VKey>>,
    /// the pair's function-word table, applied to the source's meta-world
    fwt: Option<&'a fwt::FwTable>,
    /// the target author's joining table, applied to the source's meta-world
    jt: Option<&'a fwt::JoinTable>,
    /// the target author's table of where clause parts go, applied to the source's meta-world
    rt: Option<&'a fwt::ReorderTable>,
}

/// The pair table of the word-by-word baseline: each source word form (lower case) → the target form most often
/// in its place (LCS anchors, and gaps of equal length position by position), on the training verses of the pair.
fn wbw_table(a: &Corpus, b: &Corpus, keys: &BTreeSet<VKey>) -> BTreeMap<String, String> {
    let mut c: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
    for k in keys {
        let (Some(x), Some(y)) = (a.verses.get(k), b.verses.get(k)) else { continue };
        let (kx, ky) = (meta::keys(&x.words), meta::keys(&y.words));
        let fy = profile::decap(&y.words);
        let an = meta::lcs(&kx, &ky);
        let mut pairs: Vec<(usize, usize)> = an.clone();
        for ((a0, a1), (b0, b1)) in meta::gaps(kx.len(), ky.len(), &an) {
            if a1 - a0 == b1 - b0 {
                pairs.extend((0..a1 - a0).map(|d| (a0 + d, b0 + d)));
            }
        }
        for (i, j) in pairs {
            *c.entry(x.words[i].form.to_lowercase()).or_default().entry(fy[j].clone()).or_default() += 1;
        }
    }
    c.into_iter().map(|(k, t)| (k, t.iter().fold((String::new(), 0u32), |acc, (f, &v)| if v > acc.1 { (f.clone(), v) } else { acc }).0)).collect()
}

fn run_sys(a: &Annotator, s: &Sys, eval: &[VKey], threads: usize) -> Agg {
    let has_src = |k: &VKey| match &s.from {
        From::Meta(c) | From::Copy(c) | From::Wbw(c, _) => c.verses.contains_key(k),
        From::Heb(h, _) | From::Gloss(h) => h.contains_key(k),
        From::Itself => true,
    };
    // every target verse runs (a block needs them all); the verse tables count those the source has too
    let keys: Vec<VKey> = eval.iter().copied().filter(|k| s.only.is_none_or(|o| o.contains(k)) && s.tgt.verses.contains_key(k)).collect();
    let with_src: Vec<VKey> = keys.iter().copied().filter(|k| has_src(k)).collect();
    let n = with_src.len();
    let read = |ws: &[en::annotate::Word]| s.lex.read(ws);
    let run = Run { a, enc: &s.enc, read: &read, corrupt: s.corrupt, why: true, direct: true };
    let outs: Vec<(String, Option<Vec<MTok>>, Out)> = par_map(&keys, threads, &|k: &VKey| {
        let sk = if s.rotate && has_src(k) { with_src[(with_src.iter().position(|x| x == k).unwrap() + 101) % n] } else { *k };
        let tgt = &s.tgt.verses[k];
        if !has_src(&sk) {
            // a verse the source does not have: nothing to read
            return match &s.from {
                From::Copy(_) | From::Wbw(_, _) | From::Gloss(_) => (String::new(), None, run.verse(&Src::Words(Vec::new()), tgt)),
                _ => (String::new(), Some(Vec::new()), run.verse(&Src::Meta(&[]), tgt)),
            };
        }
        let (src, m, o) = match &s.from {
            From::Meta(c) => {
                let v = &c.verses[&sk];
                let m = s.lex.read(&v.meta_words());
                let m = match s.fwt {
                    Some(t) => t.apply(&m),
                    None => m,
                };
                let m = match s.jt {
                    Some(t) => t.apply(&m),
                    None => m,
                };
                let m = match s.rt {
                    Some(t) => t.apply(&m),
                    None => m,
                };
                let o = run.verse(&Src::Meta(&m), tgt);
                (v.text.clone(), Some(m), o)
            }
            From::Itself => {
                let m = s.lex.read(&tgt.meta_words());
                let o = run.verse(&Src::Meta(&m), tgt);
                (tgt.text.clone(), Some(m), o)
            }
            From::Heb(h, r) => {
                let (m, _) = r.read(s.lex, &h[&sk]);
                let src: String = m.iter().map(|x| x.concept.as_str()).collect::<Vec<_>>().join(" ");
                let o = run.verse(&Src::Meta(&m), tgt);
                (src, Some(m), o)
            }
            From::Copy(c) => {
                let v = &c.verses[&sk];
                (v.text.clone(), None, run.verse(&Src::Words(v.forms.clone()), tgt))
            }
            From::Wbw(c, t) => {
                let v = &c.verses[&sk];
                let fx = profile::decap(&v.words);
                let mut f: Vec<String> = v.words.iter().enumerate().map(|(i, w)| t.get(&w.form.to_lowercase()).cloned().unwrap_or(fx[i].clone())).collect();
                profile::recap(&mut f);
                (v.text.clone(), None, run.verse(&Src::Words(f), tgt))
            }
            From::Gloss(h) => {
                let hv = &h[&sk];
                let mut f: Vec<String> = hv.toks.iter().flat_map(|t| t.gloss.split('.').filter(|w| !w.is_empty() && !w.starts_with('(')).map(str::to_string).collect::<Vec<_>>()).collect();
                profile::recap(&mut f);
                (f.join(" "), None, run.verse(&Src::Words(f), tgt))
            }
        };
        (src, m, o)
    });
    let mut g = Agg { name: s.name.clone(), group: s.group.to_string(), note: s.note.clone(), ..Default::default() };
    for (k, (src, m, o)) in keys.iter().zip(outs) {
        let tmeta = s.lex.read(&s.tgt.verses[k].meta_words());
        let mut tail = Residual::default();
        tail.merge(&o.res[1]);
        tail.merge(&o.res[2]);
        let mut all = tail.clone();
        all.merge(&o.res[0]);
        g.recs.insert(*k, VRec { stages: o.res.clone(), chrf: o.chrf, src: m.filter(|_| has_src(k)), tgt: tmeta, tail, all, gate: o.gate_ok, out: o.text.clone(), has_src: has_src(k) });
        if has_src(k) {
            g.add(*k, &src, &s.tgt.verses[k], o);
        }
    }
    g
}

/// A system over blocks.
#[derive(Default)]
pub struct BAgg {
    pub name: String,
    pub blocks: u64,
    pub gate_ok: u64,
    pub words: u64,
    pub exact: u64,
    pub chrf: f64,
    /// the reader over each whole block (meta systems), the verses' encoder and stack, the layout; the baselines'
    /// verse residuals all in `tail`
    pub reader: Residual,
    pub tail: Residual,
    pub layout: Residual,
    /// the same block with the reader verse by verse (each verse's own script, positions within the verse)
    pub anchored: Residual,
    pub fails: Vec<String>,
}

impl BAgg {
    pub fn bits(&self) -> f64 {
        [&self.reader, &self.tail, &self.layout].iter().map(|r| r.cells.values().map(|c| c.bits).sum::<f64>()).sum()
    }
}

/// One system over the blocks: the reader over the whole block, the verse residuals of its target verses, the
/// layout; the gate of every part; the normal-mode block text against the target's.
#[allow(clippy::too_many_arguments)]
fn block_measure(name: &str, recs: &BTreeMap<VKey, VRec>, blks: &[blocks::Block], tgt: &Corpus, src_lay: &BTreeMap<VKey, Vec<String>>, tgt_lay: &BTreeMap<VKey, Vec<String>>, shuffle: bool, corrupt: bool) -> BAgg {
    let mut b = BAgg { name: name.to_string(), ..Default::default() };
    for blk in blks {
        let tk: Vec<VKey> = recs.keys().copied().filter(|k| blk.contains(*k)).collect();
        if tk.is_empty() {
            continue;
        }
        b.blocks += 1;
        let meta = recs[&tk[0]].src.is_some() || tk.iter().any(|k| recs[k].src.is_some());
        let mut ok = tk.iter().all(|k| recs[k].gate) && !corrupt;
        if meta {
            let mut src: Vec<&[MTok]> = tk.iter().filter(|k| recs[k].has_src).filter_map(|k| recs[k].src.as_deref()).collect();
            if shuffle && src.len() > 1 {
                let h = src.len() / 2;
                src.rotate_left(h);
            }
            let tg: Vec<&[MTok]> = tk.iter().map(|k| recs[k].tgt.as_slice()).collect();
            let (r, rok) = measure::reader_block(&src, &tg);
            b.reader.merge(&r);
            ok &= rok;
            for k in &tk {
                b.tail.merge(&recs[k].tail);
            }
            if shuffle {
                // the verse-by-verse reader of shuffled verses: each target verse read against another source verse
                let n = src.len();
                for (i, k) in tk.iter().enumerate() {
                    let s_i: &[MTok] = if n > 0 { src[i % n] } else { &[] };
                    let (r, _) = measure::reader_block(&[s_i], &[recs[k].tgt.as_slice()]);
                    b.anchored.merge(&r);
                    b.anchored.merge(&recs[k].tail);
                }
            } else {
                for k in &tk {
                    b.anchored.merge(&recs[k].all);
                }
            }
        } else {
            for k in &tk {
                b.anchored.merge(&recs[k].all);
            }
            for k in &tk {
                b.tail.merge(&recs[k].all);
            }
        }
        let src_keys: BTreeSet<VKey> = tk.iter().copied().filter(|k| recs[k].has_src).collect();
        let tgt_keys: BTreeSet<VKey> = tk.iter().copied().collect();
        let decoded = blocks::layout_code(blk, &src_keys, &tgt_keys, src_lay, tgt_lay, &mut b.layout);
        let want: BTreeMap<VKey, Vec<String>> = tk.iter().map(|k| (*k, tgt_lay.get(k).cloned().unwrap_or_default())).collect();
        ok &= decoded == want;
        // the block's text, the target's and the translation's (the source's layout, the verses as written in normal mode)
        let only = |c: &Corpus| -> Corpus { Corpus { name: c.name.clone(), kind: c.kind, verses: c.verses.iter().filter(|(k, _)| tgt_keys.contains(k)).map(|(k, v)| (*k, v.clone())).collect() } };
        let tc = only(tgt);
        let target = blocks::text(blk, &tc, tgt_lay, &|k| tgt.verses.get(&k).map(|v| v.text.clone()));
        let src_lay_here: BTreeMap<VKey, Vec<String>> = src_keys.iter().filter_map(|k| src_lay.get(k).map(|l| (*k, l.clone()))).collect();
        let output = blocks::text(blk, &tc, &src_lay_here, &|k| recs.get(&k).map(|r| r.out.clone()));
        b.words += tk.iter().map(|k| tgt.verses[k].text.split_whitespace().count() as u64).sum::<u64>();
        b.exact += (output == target) as u64;
        b.chrf += en::text::chrf(&output, &target);
        if ok {
            b.gate_ok += 1;
        } else if b.fails.len() < 4 {
            b.fails.push(blk.name());
        }
    }
    b
}

/// What a verse leaves for its block: the meta-worlds read from the source (None for the baselines that give words)
/// and from the target, the encoder's and the stack's residual, the whole verse's residual, the verse gate, the
/// normal-mode output.
pub struct VRec {
    /// the verse's residual by stage, and its chrF in normal mode
    pub stages: [Residual; 3],
    pub chrf: f64,
    pub src: Option<Vec<MTok>>,
    pub tgt: Vec<MTok>,
    pub tail: Residual,
    pub all: Residual,
    pub gate: bool,
    pub out: String,
    pub has_src: bool,
}

/// World agreement of two readings of the same verses: events and entities as meta concepts (multiset matches),
/// and whether a verse has a negated event.
#[derive(Default, Clone, Debug)]
pub struct WorldAgree {
    pub name: String,
    pub verses: u64,
    pub ev: (u64, u64, u64),
    pub en: (u64, u64, u64),
    pub neg_same: u64,
}

fn multiset_match(a: &[String], b: &[String]) -> u64 {
    let mut cb: BTreeMap<&str, i64> = BTreeMap::new();
    for x in b {
        *cb.entry(x).or_default() += 1;
    }
    let mut m = 0;
    for x in a {
        if let Some(c) = cb.get_mut(x.as_str()) {
            if *c > 0 {
                *c -= 1;
                m += 1;
            }
        }
    }
    m
}

fn en_world(lex: &Lexicon, v: &PVerse) -> (Vec<String>, Vec<String>, bool) {
    let c = |s: &str| lex.concept(&s.to_lowercase()).to_string();
    (v.events.iter().map(|(x, _)| c(x)).collect(), v.ents.iter().map(|x| c(x)).collect(), v.events.iter().any(|x| x.1))
}

fn agree(name: &str, eval: &[VKey], f: &dyn Fn(VKey) -> Option<((Vec<String>, Vec<String>, bool), (Vec<String>, Vec<String>, bool))>) -> WorldAgree {
    let mut w = WorldAgree { name: name.to_string(), ..Default::default() };
    for k in eval {
        let Some((x, y)) = f(*k) else { continue };
        w.verses += 1;
        w.ev.0 += multiset_match(&x.0, &y.0);
        w.ev.1 += x.0.len() as u64;
        w.ev.2 += y.0.len() as u64;
        w.en.0 += multiset_match(&x.1, &y.1);
        w.en.1 += x.1.len() as u64;
        w.en.2 += y.1.len() as u64;
        w.neg_same += (x.2 == y.2) as u64;
    }
    w
}

struct Args {
    pools: PathBuf,
    wlc: PathBuf,
    prag: PathBuf,
    school: PathBuf,
    tales: PathBuf,
    talmud: PathBuf,
    out: PathBuf,
    split: Split,
    threads: usize,
    sample: u16,
    lexicon: String,
    /// a second hub (ASV) in the lexicon's pairs, and per-pair savings from each pair's own samples (--hub2 on)
    hub2: bool,
    /// the author's own words in its delta (--own-words <share>)
    own_words: Option<f64>,
    /// each author's typography in its encoder (--typo on)
    typo: bool,
    /// where clause parts go, a table per target author (--reorder on)
    reorder: bool,
    /// one vote per corpus in the standard (--std-vote corpus)
    std_vote: String,
    /// the Hebrew reader's grammar: construct «of»/«the», possessive suffixes, subject before verb (--heb-grammar on)
    heb_grammar: bool,
    /// the lexicon's judge takes the 95% lower bound of the measured saving (--lb on)
    lb: bool,
    /// the standard learns on the verse lattice's backbone (--lattice on)
    lattice: bool,
    /// a function-word insert/drop table per trained author pair (--fwt on)
    fwt: bool,
    /// the clause-level meta-world (--clauses on), or its tree half only (--clauses flat)
    clauses: String,
    /// determiners and prepositions as features of meta tokens (--fw on)
    fw: bool,
    /// judge each lexicon merge per author pair (--judge-pairs on)
    per_pair: bool,
    /// the meta-world's tree through the frames' anchors (--meta-tree frames)
    frames_tree: bool,
    /// coarse relations in the meta-world (--meta-rel coarse)
    coarse: bool,
    /// the lexicon's judge measures what a merge saves the reader (--judge measured)
    judge_measured: bool,
    /// pronoun concepts with number and person (off by default: measured to cost more than it saves)
    feats: bool,
    /// eBible's directory (`engjps`, `eng-kjv`, `eng-web`, `eng-asv`): the layout of the blocks
    usfm: Option<PathBuf>,
    /// eBible's Strong-tagged KJV (the `eng-kjv` USFM directory)
    strong: Option<PathBuf>,
    /// the site's Markdown root (`free/en`): the parallel Tanakh as pages, both ways (`pages.rs`)
    pages: Option<PathBuf>,
    /// the `en` model (default `$MOVA_DATA/en/models/en.bin`)
    model: Option<PathBuf>,
    /// the site's markup dictionary and Hebrew trees' lexicon (`roundtrip site-learn`), for the pages
    site_dict: Option<PathBuf>,
    site_heb: Option<PathBuf>,
    /// the site's train pages' English (`roundtrip site-learn`'s english.tsv), for the standard encoder
    site_english: Option<PathBuf>,
    /// the Hebrew stack's forms with the word before them in their written word (--heb-forms ctx)
    heb_forms_ctx: bool,
}

fn args() -> Result<Args> {
    let v: Vec<String> = std::env::args().skip(1).collect();
    if v.first().map(String::as_str) != Some("eval") {
        bail!("usage: translate eval --pools <dir> --wlc <dir> --prag <prag.tsv> --school <dir> --tales <conllu> --talmud <pool.tsv> --out <dir> [--split dev|test] [--threads N] [--sample N]");
    }
    let mut m: BTreeMap<String, String> = BTreeMap::new();
    let mut it = v.iter().skip(1);
    while let Some(x) = it.next() {
        let val = it.next().with_context(|| format!("{x} needs a value"))?;
        m.insert(x.trim_start_matches("--").to_string(), val.clone());
    }
    let p = |k: &str| m.get(k).map(PathBuf::from).with_context(|| format!("--{k}"));
    Ok(Args {
        pools: p("pools")?,
        wlc: p("wlc")?,
        prag: p("prag")?,
        school: p("school")?,
        tales: p("tales")?,
        talmud: p("talmud")?,
        out: p("out")?,
        split: m.get("split").map(|s| Split::parse(s).context("--split dev|test")).transpose()?.unwrap_or(Split::Dev),
        threads: m.get("threads").map(|s| s.parse()).transpose()?.unwrap_or(8),
        sample: m.get("sample").map(|s| s.parse()).transpose()?.unwrap_or(5),
        lexicon: m.get("lexicon").cloned().unwrap_or_else(|| "judged".into()),
        strong: m.get("strong").map(PathBuf::from),
        usfm: m.get("usfm").map(PathBuf::from),
        pages: m.get("pages").map(PathBuf::from),
        model: m.get("model").map(PathBuf::from),
        site_dict: m.get("site-dict").map(PathBuf::from),
        site_heb: m.get("site-heb").map(PathBuf::from),
        site_english: m.get("site-english").map(PathBuf::from),
        heb_forms_ctx: m.get("heb-forms").is_some_and(|v| v == "ctx"),
        feats: m.get("feats").is_some_and(|v| v == "on"),
        coarse: m.get("meta-rel").is_some_and(|v| v == "coarse"),
        frames_tree: m.get("meta-tree").is_some_and(|v| v == "frames"),
        per_pair: m.get("judge-pairs").is_some_and(|v| v == "on"),
        fw: m.get("fw").is_some_and(|v| v == "on"),
        clauses: m.get("clauses").cloned().unwrap_or_default(),
        fwt: m.get("fwt").is_some_and(|v| v == "on"),
        lb: m.get("lb").is_some_and(|v| v == "on"),
        std_vote: m.get("std-vote").cloned().unwrap_or_default(),
        reorder: m.get("reorder").is_some_and(|v| v == "on"),
        own_words: m.get("own-words").map(|x| x.parse()).transpose()?,
        typo: m.get("typo").is_some_and(|v| v == "on"),
        heb_grammar: m.get("heb-grammar").is_some_and(|v| v == "on"),
        lattice: m.get("lattice").is_some_and(|v| v == "on"),
        hub2: m.get("hub2").is_some_and(|v| v == "on"),
        judge_measured: m.get("judge").is_some_and(|v| v == "measured"),
    })
}

fn main() -> Result<()> {
    let v: Vec<String> = std::env::args().skip(1).collect();
    if v.first().map(String::as_str) == Some("many") {
        return many::run(&v[1..]);
    }
    let ar = args()?;
    if ar.split == Split::Train {
        bail!("--split is dev or test");
    }
    std::fs::create_dir_all(&ar.out)?;
    let t0 = std::time::Instant::now();
    let a = Annotator::load(&ar.model.clone().unwrap_or_else(roundtrip::model_path))?;
    let lap = |what: &str| eprintln!("[{:>6.1}s] {what}", t0.elapsed().as_secs_f64());
    lap("model loaded");
    // texts, parsed: the training part and the measured part (the school's held-out lessons sampled)
    let split = ar.split;
    let sample = ar.sample;
    let load = |name: &str, kind: Kind, pool: &BTreeMap<VKey, String>| -> Result<Corpus> {
        // the school is large: a fifth of its training lessons and one in `sample` of its held-out ones
        let school = name.starts_with("school");
        let keep = |k: VKey| {
            let s = kind.split(k);
            s == Split::Train && (!school || k.1 % 10 == 0 || k.1 % 10 == 6) || (s == split && (!school || kind.sampled(k, sample)))
        };
        let c = corpus::parse(&a, name, kind, pool, &keep, ar.threads, ar.frames_tree)?;
        lap(&format!("{name}: {} units parsed", c.verses.len()));
        Ok(c)
    };
    let pool = |n: &str| bible::read_pool(&ar.pools.join(format!("{n}.tsv")));
    let jps = load("jps", Kind::Bible, &pool("jps")?)?;
    let kjv = load("kjv", Kind::Bible, &pool("kjv")?)?;
    let web = load("web", Kind::Bible, &pool("web")?)?;
    let asv = load("asv", Kind::Bible, &pool("asv")?)?;
    let jpsh = load("jps-heb", Kind::Bible, &pool("jps-heb")?)?;
    let plain = load("school-plain", Kind::Docs, &sources::lessons(&ar.school, Register::Plain)?)?;
    let talk = load("school-talk", Kind::Docs, &sources::lessons(&ar.school, Register::Talk)?)?;
    let exam = load("school-exam", Kind::Docs, &sources::lessons(&ar.school, Register::Exam)?)?;
    let tales = load("tales", Kind::Docs, &sources::treebank_texts(&ar.tales)?)?;
    let talmud = load("talmud", Kind::TrainOnly, &sources::pool_texts(&ar.talmud)?)?;
    let heb = hebrew::read(&ar.wlc, Some(&ar.prag))?;
    lap(&format!("Hebrew: {} verses", heb.len()));
    let bibles: [&Corpus; 4] = [&jps, &kjv, &web, &asv];
    let styles: [&Corpus; 8] = [&jps, &kjv, &web, &asv, &plain, &talk, &exam, &tales];
    let train_of = |c: &Corpus| -> BTreeSet<VKey> { c.verses.keys().filter(|k| c.kind.split(**k) == Split::Train).copied().collect() };
    let eval_of = |c: &Corpus| -> Vec<VKey> { c.verses.keys().filter(|k| c.kind.split(**k) == split).copied().collect() };
    let bible_train: BTreeSet<VKey> = train_of(&kjv);
    // the interlingua lexicon: pairs around KJV only, so JPS↔WEB, JPS↔ASV and WEB↔ASV are never paired
    // with --hub2 on, also around ASV (JPS–ASV, ASV–WEB): JPS↔WEB alone is never paired
    let mut lex_pairs: Vec<(&Corpus, &Corpus)> = vec![(&jps, &kjv), (&kjv, &web), (&kjv, &asv)];
    if ar.hub2 {
        lex_pairs.push((&jps, &asv));
        lex_pairs.push((&asv, &web));
    }
    let paired: BTreeSet<(String, String)> = lex_pairs.iter().flat_map(|(a, b)| [(a.name.clone(), b.name.clone()), (b.name.clone(), a.name.clone())]).collect();
    let all: [&Corpus; 9] = [&jps, &kjv, &web, &asv, &plain, &talk, &exam, &tales, &talmud];
    // judged (default): loose candidates, each kept only if it lowers the bits it moves; mutual: the plain rule
    let (lp, judge): (meta::LexParams, Option<&[&Corpus]>) = match ar.lexicon.as_str() {
        "mutual" => (meta::LexParams::default(), None),
        _ => (meta::LexParams { min_count: 3, share: 0.2, cover: 0.02, mutual: false, max_len: 4, feats: false, coarse: false, fw: false, clauses: false, joined: false, flat: false, own_samples: false, lb: false, per_pair: false, distinct: false }, Some(&all[..])),
    };
    let lp = meta::LexParams { feats: ar.feats, coarse: ar.coarse, fw: ar.fw, clauses: ar.clauses == "on" || ar.clauses == "joined", joined: ar.clauses == "joined", flat: ar.clauses == "flat", own_samples: ar.hub2, lb: ar.lb, per_pair: ar.per_pair, ..lp };
    // the judge's savings measured on sampled verse pairs (--judge measured), read as the run reads
    let mfn = |a: &[MTok], b: &[MTok]| measure::reader_bits(a, b);
    let measure: Option<&meta::Measure> = if ar.judge_measured { Some(&mfn) } else { None };
    let mut lex = meta::learn(&lex_pairs, &bible_train, lp, false, judge, measure);
    let mut lex_shuf = meta::learn(&lex_pairs, &bible_train, lp, true, judge, measure);
    lex.feats = ar.feats;
    lex_shuf.feats = ar.feats;
    lex.coarse = ar.coarse;
    lex_shuf.coarse = ar.coarse;
    lex.fw = ar.fw;
    lex_shuf.fw = ar.fw;
    lex.clauses = ar.clauses == "on" || ar.clauses == "joined";
    lex_shuf.clauses = lex.clauses;
    lex.joined = ar.clauses == "joined";
    lex_shuf.joined = lex.joined;
    lex.flat = ar.clauses == "flat";
    lex_shuf.flat = ar.clauses == "flat";
    lap(&format!("lexicon: {} words, {} phrases ({} with shuffled verses)", lex.single.len(), lex.phrases.len(), lex_shuf.single.len() + lex_shuf.phrases.len()));
    // the standard and the deltas
    let train_units = |lx: &Lexicon, c: &Corpus| -> Vec<Seen> { profile::seen_of(lx, c, &|k| c.kind.split(k) == Split::Train) };
    // the standard learns on all of them, and on the site's train pages' English when given (`--site-english`)
    let site = match &ar.site_english {
        Some(p) => Some(load("site", Kind::TrainOnly, &sources::pool_texts(p)?)?),
        None => None,
    };
    let mut std_corpora: Vec<&Corpus> = all.to_vec();
    if let Some(sc) = &site {
        std_corpora.push(sc);
    }
    let mut units: Vec<Vec<Seen>> = std_corpora.iter().map(|c| train_units(&lex, c)).collect();
    // the verse lattice: a Bible's token is on the backbone when every other rendering of the verse has it (the
    // longest common subsequence of concepts); the standard learns from the backbone only, the deltas from all
    let mut backbone = (0u64, 0u64);
    if ar.lattice {
        let concepts = |c: &Corpus, k: &VKey| -> Vec<String> { lex.read(&c.verses[k].meta_words()).iter().map(|x| x.concept.clone()).collect() };
        for (bi, c) in bibles.iter().enumerate() {
            let keys: Vec<VKey> = c.verses.keys().filter(|k| c.kind.split(**k) == Split::Train).copied().collect();
            for (ui, k) in keys.iter().enumerate() {
                let mine = concepts(c, k);
                let mut keep = vec![true; mine.len()];
                for o in bibles.iter().filter(|o| o.name != c.name) {
                    let Some(_) = o.verses.get(k) else { continue };
                    let theirs = concepts(o, k);
                    let on: BTreeSet<usize> = meta::lcs(&mine, &theirs).into_iter().map(|x| x.0).collect();
                    for (i, kp) in keep.iter_mut().enumerate() {
                        *kp &= on.contains(&i);
                    }
                }
                backbone.0 += keep.iter().filter(|&&x| x).count() as u64;
                backbone.1 += keep.len() as u64;
                if let Some(u) = units[bi].get_mut(ui) {
                    u.keep = keep;
                }
            }
        }
        lap(&format!("verse lattice: {} of {} Bible tokens on the backbone", backbone.0, backbone.1));
    }
    let std = if ar.std_vote == "context" {
        Standard::train_voted(&units.iter().map(|u| u.iter().collect::<Vec<_>>()).collect::<Vec<_>>())
    } else if ar.std_vote == "corpus" {
        Standard::train_weighted(&units.iter().map(|u| (u.iter().collect::<Vec<_>>(), 1e6 / u.iter().map(|x| x.m.len()).sum::<usize>().max(1) as f64)).collect::<Vec<_>>())
    } else {
        Standard::train(&units.iter().flatten().collect::<Vec<_>>())
    };
    lap(&format!("standard: {} meta tokens", std.tokens));
    let deltas: Vec<Delta> = par_map(&(0..8).collect::<Vec<usize>>(), ar.threads, &|&i| {
        let u: Vec<&Seen> = units[i].iter().collect();
        let d = Delta::train(&styles[i].name, &std, &a, &u, false);
        // the author's own words (--own-words <share>)
        match ar.own_words {
            Some(sh) => d.with_own_words(&u, sh),
            None => d,
        }
    });
    // each author's typography (--typo on)
    let typos: BTreeMap<String, profile::Typo> = if ar.typo { styles.iter().map(|c| (c.name.clone(), profile::Typo::learn(c, &|k| c.kind.split(k) == Split::Train))).collect() } else { BTreeMap::new() };
    for (n, t) in &typos {
        lap(&format!("typography of {n}: quotes {:?} {:?}, possessive «{}», bare after s: {}", t.open, t.close, t.mark, t.bare_after_s));
    }
    let deltas_shuf: Vec<Delta> = par_map(&(0..8).collect::<Vec<usize>>(), ar.threads, &|&i| Delta::train(&format!("{} shuffled", styles[i].name), &std, &a, &units[i].iter().collect::<Vec<_>>(), true));
    lap("deltas");
    // the world with the shuffled lexicon (a control): the standard and the KJV, WEB deltas read through it
    let units_s: Vec<Vec<Seen>> = std_corpora.iter().map(|c| train_units(&lex_shuf, c)).collect();
    let std_s = if ar.std_vote == "context" {
        Standard::train_voted(&units_s.iter().map(|u| u.iter().collect::<Vec<_>>()).collect::<Vec<_>>())
    } else if ar.std_vote == "corpus" {
        Standard::train_weighted(&units_s.iter().map(|u| (u.iter().collect::<Vec<_>>(), 1e6 / u.iter().map(|x| x.m.len()).sum::<usize>().max(1) as f64)).collect::<Vec<_>>())
    } else {
        Standard::train(&units_s.iter().flatten().collect::<Vec<_>>())
    };
    let d_kjv_s = Delta::train("kjv (shuffled lexicon)", &std_s, &a, &units_s[1].iter().collect::<Vec<_>>(), false);
    let d_web_s = Delta::train("web (shuffled lexicon)", &std_s, &a, &units_s[2].iter().collect::<Vec<_>>(), false);
    lap("shuffled-lexicon standard");
    // verses whose English and Hebrew numbers agree: JPS cut both ways gives the same text
    let same_num: BTreeSet<VKey> = jps.verses.iter().filter(|(k, v)| jpsh.verses.get(k).is_some_and(|h| h.text == v.text)).map(|(k, _)| *k).collect();
    // the Hebrew reader, on the meta-world read from JPS at the Hebrew verse numbers
    // and on the meta-world read from KJV where its numbers are the Hebrew ones, aligned by Strong's numbers
    let mut hpairs: Vec<(&HVerse, Vec<MTok>, Vec<Option<u32>>)> = bible_train.iter().filter_map(|k| Some((heb.get(k)?, lex.read(&jpsh.verses.get(k)?.meta_words()), Vec::new()))).collect();
    if let Some(dir) = &ar.strong {
        let sw = strong::read(dir)?;
        for k in bible_train.iter().filter(|k| same_num.contains(k)) {
            let (Some(h), Some(v), Some(ws)) = (heb.get(k), kjv.verses.get(k), sw.get(k)) else { continue };
            let m = lex.read(&v.meta_words());
            let on = strong::on_tokens(&v.forms, ws);
            let sn = m.iter().map(|x| (x.span.0..x.span.1).find_map(|t| on[t])).collect();
            hpairs.push((h, m, sn));
        }
    }
    let jps_pairs = hpairs.len();
    let mut hreader = Reader::train(&lex, &hpairs, false);
    let mut hreader_s = Reader::train(&lex, &hpairs, true);
    hreader.grammar = ar.heb_grammar;
    hreader_s.grammar = ar.heb_grammar;
    lap(&format!("Hebrew reader: {} verse pairs, {} words aligned, {} without a chunk, {} meta tokens aligned by Strong's number", hpairs.len(), hreader.aligned, hreader.null, hreader.strong));
    // the function-word tables of the trained pairs (both ways), and two learned on shuffled verses (a control)
    let mut tables: BTreeMap<(String, String), fwt::FwTable> = BTreeMap::new();
    let mut tables_shuf: BTreeMap<(String, String), fwt::FwTable> = BTreeMap::new();
    if ar.fwt {
        for (a, b) in &lex_pairs {
            for (x, y) in [(*a, *b), (*b, *a)] {
                let t = fwt::FwTable::train(&format!("{}→{}", x.name, y.name), &lex, x, y, &bible_train, false);
                tables.insert((x.name.clone(), y.name.clone()), t);
            }
        }
        for (x, y) in [(&jps, &kjv), (&kjv, &web)] {
            tables_shuf.insert((x.name.clone(), y.name.clone()), fwt::FwTable::train(&format!("{}→{} shuffled", x.name, y.name), &lex, x, y, &bible_train, true));
        }
        lap(&format!("function-word tables: {}", tables.values().map(|t| format!("{} {}", t.name, t.size())).collect::<Vec<_>>().join(", ")));
    }
    let table = |x: &Corpus, y: &Corpus| tables.get(&(x.name.clone(), y.name.clone()));
    // systems
    let idx = |c: &Corpus| styles.iter().position(|x| x.name == c.name).unwrap();
    let typo = |c: &Corpus| typos.get(&c.name);
    let with = |c: &Corpus| Encoder { name: format!("standard + {} delta", c.name), std: Some(&std), delta: Some(&deltas[idx(c)]), typo: typo(c) };
    let std_only = || Encoder { name: "standard".into(), std: Some(&std), delta: None, typo: None };
    let mut syss: Vec<Sys> = Vec::new();
    // 1. every style back through the meta-world: standard alone, standard + delta, the wrong delta, a shuffled delta, bare forms
    let wrong = |c: &Corpus| -> &Corpus {
        match c.name.as_str() {
            "jps" => &web,
            "kjv" => &web,
            "web" => &kjv,
            "asv" => &web,
            "school-plain" => &talk,
            "school-talk" => &plain,
            "school-exam" => &talk,
            _ => &kjv,
        }
    };
    for c in styles {
        let i = idx(c);
        for (tag, e) in [
            ("standard", std_only()),
            ("standard + delta", with(c)),
            ("standard + wrong delta", Encoder { name: format!("standard + {} delta", wrong(c).name), std: Some(&std), delta: Some(&deltas[idx(wrong(c))]), typo: typo(wrong(c)) }),
            ("standard + shuffled delta", Encoder { name: "standard + shuffled delta".into(), std: Some(&std), delta: Some(&deltas_shuf[i]), typo: typo(c) }),
            ("bare forms", Encoder { name: "bare forms".into(), std: None, delta: None, typo: None }),
        ] {
            syss.push(Sys { name: format!("{} → meta → {}: {tag}", c.name, c.name), group: "autoencoder", note: e.name.clone(), from: From::Itself, tgt: c, enc: e, lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
        }
    }
    // 2. any → any among the Bibles, standard + the target's delta; the lexicon saw only pairs with KJV
    for x in bibles {
        for y in bibles {
            if x.name == y.name {
                continue;
            }
            let seen = paired.contains(&(x.name.clone(), y.name.clone()));
            syss.push(Sys { name: format!("{} → {}", x.name, y.name), group: "any→any", note: if seen { "pair in the lexicon".into() } else { "pair never trained".into() }, from: From::Meta(x), tgt: y, enc: with(y), lex: &lex, rotate: false, corrupt: false, only: None, fwt: table(x, y), jt: None, rt: None });
        }
    }
    // 3. baselines and controls on three pairs
    for (x, y) in [(&jps, &kjv), (&kjv, &web), (&jps, &web)] {
        let p = format!("{} → {}", x.name, y.name);
        syss.push(Sys { name: format!("{p}: standard alone"), group: "control", note: "null delta".into(), from: From::Meta(x), tgt: y, enc: std_only(), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
        syss.push(Sys { name: format!("{p}: copy the source"), group: "baseline", note: String::new(), from: From::Copy(x), tgt: y, enc: std_only(), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    }
    let wbw: Vec<(String, BTreeMap<String, String>)> = [(&jps, &kjv), (&kjv, &web), (&jps, &web)].iter().map(|(x, y)| (format!("{} → {}", x.name, y.name), wbw_table(x, y, &bible_train))).collect();
    for (i, (x, y)) in [(&jps, &kjv), (&kjv, &web), (&jps, &web)].into_iter().enumerate() {
        syss.push(Sys { name: format!("{}: word by word", wbw[i].0), group: "baseline", note: "a table trained on this pair".into(), from: From::Wbw(x, &wbw[i].1), tgt: y, enc: std_only(), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    }
    syss.push(Sys { name: "jps → kjv: WEB's delta".into(), group: "control", note: "wrong author".into(), from: From::Meta(&jps), tgt: &kjv, enc: with(&web), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "kjv → web: JPS's delta".into(), group: "control", note: "wrong author".into(), from: From::Meta(&kjv), tgt: &web, enc: with(&jps), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    for (x, y) in [(&jps, &kjv), (&kjv, &web)] {
        if let Some(t) = tables_shuf.get(&(x.name.clone(), y.name.clone())) {
            syss.push(Sys { name: format!("{} → {}: shuffled function-word table", x.name, y.name), group: "control", note: format!("table learned on shuffled verse pairs ({} rules)", t.size()), from: From::Meta(x), tgt: y, enc: with(y), lex: &lex, rotate: false, corrupt: false, only: None, fwt: Some(t), jt: None, rt: None });
        }
    }
    syss.push(Sys { name: "jps → kjv: shuffled lexicon".into(), group: "control", note: "lexicon from shuffled verse pairs".into(), from: From::Meta(&jps), tgt: &kjv, enc: Encoder { name: "standard + kjv delta (shuffled lexicon)".into(), std: Some(&std_s), delta: Some(&d_kjv_s), typo: typo(&kjv) }, lex: &lex_shuf, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "kjv → web: shuffled lexicon".into(), group: "control", note: "lexicon from shuffled verse pairs".into(), from: From::Meta(&kjv), tgt: &web, enc: Encoder { name: "standard + web delta (shuffled lexicon)".into(), std: Some(&std_s), delta: Some(&d_web_s), typo: typo(&web) }, lex: &lex_shuf, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "jps → web: shuffled lexicon".into(), group: "control", note: "lexicon from shuffled verse pairs".into(), from: From::Meta(&jps), tgt: &web, enc: Encoder { name: "standard + web delta (shuffled lexicon)".into(), std: Some(&std_s), delta: Some(&d_web_s), typo: typo(&web) }, lex: &lex_shuf, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "jps → kjv: verses misaligned".into(), group: "control", note: "each KJV verse from another JPS verse".into(), from: From::Meta(&jps), tgt: &kjv, enc: with(&kjv), lex: &lex, rotate: true, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "jps → kjv: gate control".into(), group: "control", note: "one space flip dropped from the residual".into(), from: From::Meta(&jps), tgt: &kjv, enc: with(&kjv), lex: &lex, rotate: false, corrupt: true, only: None, fwt: None, jt: None, rt: None });
    // 4. Hebrew → English
    syss.push(Sys { name: "wlc → jps".into(), group: "hebrew", note: "standard + jps delta".into(), from: From::Heb(&heb, &hreader), tgt: &jpsh, enc: with(&jps), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "wlc → jps: standard alone".into(), group: "hebrew", note: "null delta".into(), from: From::Heb(&heb, &hreader), tgt: &jpsh, enc: std_only(), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "wlc → kjv".into(), group: "hebrew", note: "standard + kjv delta; verses whose English and Hebrew numbers agree".into(), from: From::Heb(&heb, &hreader), tgt: &kjv, enc: with(&kjv), lex: &lex, rotate: false, corrupt: false, only: Some(&same_num), fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "wlc → web".into(), group: "hebrew", note: "standard + web delta; verses whose English and Hebrew numbers agree".into(), from: From::Heb(&heb, &hreader), tgt: &web, enc: with(&web), lex: &lex, rotate: false, corrupt: false, only: Some(&same_num), fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "wlc → jps: MACULA glosses word by word".into(), group: "hebrew", note: "baseline".into(), from: From::Gloss(&heb), tgt: &jpsh, enc: std_only(), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "wlc → jps: shuffled reader".into(), group: "hebrew", note: "the Hebrew reader learned on shuffled verse pairs".into(), from: From::Heb(&heb, &hreader_s), tgt: &jpsh, enc: with(&jps), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    syss.push(Sys { name: "jps-heb → meta → jps-heb".into(), group: "hebrew", note: "the same verses' autoencoder".into(), from: From::Itself, tgt: &jpsh, enc: with(&jps), lex: &lex, rotate: false, corrupt: false, only: None, fwt: None, jt: None, rt: None });
    // the false friends across centuries, KJV against WEB and against JPS on the training verses
    let mut ff: Vec<String> = Vec::new();
    for w in ["let", "prevent", "conversation", "quick", "meat", "suffer"] {
        let concept = lex.concept(w).to_string();
        let members = lex.clusters.get(&concept).map(|m| m.iter().map(|x| x.0.clone()).collect::<Vec<_>>().join(", ")).unwrap_or_default();
        for o in [&web, &jps] {
            let (same, differ, p) = meta::anchor_stats(&kjv, o, &bible_train, w);
            ff.push(format!("| {w} | {} | {} | {same} | {differ} | {} |", o.name, if members.is_empty() { "—".to_string() } else { format!("{concept}: {members}") }, p.iter().map(|(x, n)| format!("{x} ({n})")).collect::<Vec<_>>().join(", ")));
        }
    }
    // the archaism dial: KJV → WEB with only WEB's k strongest overrides
    let dials: Vec<(usize, Delta)> = [0usize, 10, 50, 200].iter().map(|&k| (k, deltas[idx(&web)].top(k))).collect();
    for (k, d) in &dials {
        syss.push(Sys { name: format!("kjv → web: WEB's top {k} overrides"), group: "dial", note: format!("archaism dial, {} of {} overrides", d.size(), deltas[idx(&web)].size()), from: From::Meta(&kjv), tgt: &web, enc: Encoder { name: format!("standard + web delta top {k}"), std: Some(&std), delta: Some(d), typo: typo(&web) }, lex: &lex, rotate: false, corrupt: false, only: None, fwt: table(&kjv, &web), jt: None, rt: None });
    }
    // the joining tables of the target authors (--clauses joined), and the KJV one learned on shuffled verses
    let mut jtables: BTreeMap<String, fwt::JoinTable> = BTreeMap::new();
    let mut jt_shuf: Option<fwt::JoinTable> = None;
    if lex.joined {
        for y in bibles {
            let into: Vec<(&Corpus, &Corpus)> = lex_pairs.iter().flat_map(|(a, b)| [(*a, *b), (*b, *a)]).filter(|(_, b)| b.name == y.name).collect();
            let t = fwt::JoinTable::train(&format!("joining into {}", y.name), &lex, &into, &bible_train, false);
            if y.name == "kjv" {
                jt_shuf = Some(fwt::JoinTable::train("joining into kjv, shuffled", &lex, &into, &bible_train, true));
            }
            jtables.insert(y.name.clone(), t);
        }
        lap(&format!("joining tables: {}", jtables.values().map(|t| format!("{} {}", t.name, t.map.len())).collect::<Vec<_>>().join(", ")));
        for s in syss.iter_mut() {
            if matches!(s.from, From::Meta(_)) {
                s.jt = jtables.get(&s.tgt.name);
            }
        }
        if let Some(t) = &jt_shuf {
            syss.push(Sys { name: "jps → kjv: shuffled joining table".into(), group: "control", note: format!("{} mappings", t.map.len()), from: From::Meta(&jps), tgt: &kjv, enc: with(&kjv), lex: &lex, rotate: false, corrupt: false, only: None, fwt: table(&jps, &kjv), jt: Some(t), rt: None });
            syss.push(Sys { name: "jps → kjv: no joining table".into(), group: "control", note: "the source's joining kept".into(), from: From::Meta(&jps), tgt: &kjv, enc: with(&kjv), lex: &lex, rotate: false, corrupt: false, only: None, fwt: table(&jps, &kjv), jt: None, rt: None });
        }
    }
    // where clause parts go, per target author (--reorder on), and a KJV one learned on shuffled verses
    let mut rtables: BTreeMap<String, fwt::ReorderTable> = BTreeMap::new();
    let mut rt_shuf: Option<fwt::ReorderTable> = None;
    if ar.reorder {
        for y in bibles {
            let into: Vec<(&Corpus, &Corpus)> = lex_pairs.iter().flat_map(|(a, b)| [(*a, *b), (*b, *a)]).filter(|(_, b)| b.name == y.name).collect();
            rtables.insert(y.name.clone(), fwt::ReorderTable::train(&format!("parts into {}", y.name), &lex, &into, &bible_train, false));
            if y.name == "kjv" {
                rt_shuf = Some(fwt::ReorderTable::train("parts into kjv, shuffled", &lex, &into, &bible_train, true));
            }
        }
        lap(&format!("reorder tables: {}", rtables.values().map(|t| format!("{} {} [{}]", t.name, t.moves.len(), t.top.join("; "))).collect::<Vec<_>>().join(", ")));
        for s in syss.iter_mut() {
            if matches!(s.from, From::Meta(_)) && s.group != "dial" {
                s.rt = rtables.get(&s.tgt.name);
            }
        }
        if let Some(t) = &rt_shuf {
            syss.push(Sys { name: "jps → kjv: shuffled reorder table".into(), group: "control", note: format!("{} moves", t.moves.len()), from: From::Meta(&jps), tgt: &kjv, enc: with(&kjv), lex: &lex, rotate: false, corrupt: false, only: None, fwt: table(&jps, &kjv), jt: None, rt: Some(t) });
            syss.push(Sys { name: "jps → kjv: no reorder table".into(), group: "control", note: "the source's order".into(), from: From::Meta(&jps), tgt: &kjv, enc: with(&kjv), lex: &lex, rotate: false, corrupt: false, only: None, fwt: table(&jps, &kjv), jt: None, rt: None });
        }
    }
    let mut aggs: Vec<Agg> = Vec::new();
    for s in &syss {
        let ev = eval_of(s.tgt);
        let g = run_sys(&a, s, &ev, ar.threads);
        lap(&format!("{}: {} verses, gate {}, exact {}, chrF {:.1}", g.name, g.verses, g.gate_ok, g.exact, g.chrf / g.verses.max(1) as f64));
        aggs.push(g);
    }
    // blocks: whole chapters up to 2 500 words, the English ones by JPS's chapters, the Hebrew-numbered by JPS's cut at
    // the Hebrew numbers; the layout of each author from its USFM (JPS's for its Hebrew numbering where the numbers agree)
    let mut bagg: Vec<BAgg> = Vec::new();
    if let Some(dir) = &ar.usfm {
        let mut lay: BTreeMap<String, BTreeMap<VKey, Vec<String>>> = BTreeMap::new();
        for (n, d) in [("jps", "engjps"), ("kjv", "eng-kjv"), ("web", "eng-web"), ("asv", "eng-asv")] {
            lay.insert(n.to_string(), blocks::read_layout(&dir.join(d))?);
        }
        let jl = lay["jps"].clone();
        lay.insert("jps-heb".into(), jl.into_iter().filter(|(k, _)| same_num.contains(k)).collect());
        // cut by roundtrip blocks' rule (`roundtrip::blocks::group`); the cut of this crate's own maker must agree
        let blk_en = blocks::make_rt(&jps, &|k| jps.kind.split(k) == split, 2500);
        let blk_he = blocks::make_rt(&jpsh, &|k| jpsh.kind.split(k) == split, 2500);
        let same = |x: &[blocks::Block], y: &[blocks::Block]| x.len() == y.len() && x.iter().zip(y).all(|(a, b)| (a.book, a.first, a.last) == (b.book, b.first, b.last));
        let own_en = blocks::make(&jps, &|k| jps.kind.split(k) == split, 2500);
        let own_he = blocks::make(&jpsh, &|k| jpsh.kind.split(k) == split, 2500);
        let dev_books: Vec<&str> = bible::BOOKS.iter().enumerate().filter(|(i, _)| bible::split_of(*i as u8) == split).map(|(_, b)| *b).collect();
        let judaism = ar.wlc.parent().and_then(|p| p.parent()).map(PathBuf::from).unwrap_or_default();
        let rt = blocks::rt_jps(&judaism, &dev_books, 2500).unwrap_or_default();
        let rt_same = rt.len() == blk_he.len() && rt.iter().zip(&blk_he).all(|(r, b)| *r == (b.book, b.first, b.last));
        lap(&format!(
            "blocks: {} English, {} at the Hebrew numbers; this crate's maker gives the same: {}, {}; roundtrip blocks' JPS (its own pool) gives the same Hebrew-numbered runs: {} ({} runs)",
            blk_en.len(),
            blk_he.len(),
            same(&blk_en, &own_en),
            same(&blk_he, &own_he),
            rt_same,
            rt.len()
        ));
        let none: BTreeMap<VKey, Vec<String>> = BTreeMap::new();
        for (si, s) in syss.iter().enumerate() {
            let g = &aggs[si];
            let keep = matches!(s.group, "any→any" | "baseline" | "hebrew") || (s.group == "autoencoder" && s.name.ends_with(": standard + delta")) || s.name.contains("shuffled lexicon") || s.name.ends_with("gate control");
            if !keep || s.only.is_some() || s.tgt.kind != Kind::Bible {
                continue;
            }
            let src_name = match &s.from {
                From::Meta(c) | From::Copy(c) | From::Wbw(c, _) => Some(c.name.clone()),
                From::Itself => Some(s.tgt.name.clone()),
                _ => None,
            };
            let src_lay = src_name.as_ref().and_then(|n| lay.get(n)).unwrap_or(&none);
            let tgt_lay = lay.get(&s.tgt.name).unwrap_or(&none);
            let blks = if s.tgt.name == "jps-heb" { &blk_he } else { &blk_en };
            let mut variants: Vec<(String, bool)> = vec![(g.name.clone(), false)];
            if g.name == "jps → kjv" {
                variants.push(("jps → kjv: verses shuffled inside each block".into(), true));
            }
            for (name, shuffle) in variants {
                let b = block_measure(&name, &g.recs, blks, s.tgt, src_lay, tgt_lay, shuffle, s.corrupt);
                lap(&format!("{name}: {} blocks, gate {}, {:.0} bits a block", b.blocks, b.gate_ok, b.bits() / b.blocks.max(1) as f64));
                bagg.push(b);
            }
        }
    }
    // meta-world agreement at the world level
    let mut worlds = Vec::new();
    for (x, y) in [(&jps, &kjv), (&kjv, &web), (&jps, &web), (&kjv, &asv), (&web, &asv)] {
        let ev = eval_of(y);
        worlds.push(agree(&format!("{} ~ {}", x.name, y.name), &ev, &|k| Some((en_world(&lex, x.verses.get(&k)?), en_world(&lex, y.verses.get(&k)?)))));
    }
    let evh = eval_of(&jpsh);
    worlds.push(agree("wlc ~ jps", &evh, &|k| {
        let h = heb.get(&k)?;
        let (e, n) = hreader.world(&lex, h);
        Some(((e, n, h.neg), en_world(&lex, jpsh.verses.get(&k)?)))
    }));
    worlds.push(agree("wlc ~ jps (shuffled reader)", &evh, &|k| {
        let h = heb.get(&k)?;
        let (e, n) = hreader_s.world(&lex, h);
        Some(((e, n, h.neg), en_world(&lex, jpsh.verses.get(&k)?)))
    }));
    let info = report::Info {
        split: split.name().to_string(),
        lex: &lex,
        std: &std,
        deltas: &deltas,
        deltas_shuf: &deltas_shuf,
        units: styles.iter().map(|c| (c.name.clone(), c.verses.keys().filter(|k| c.kind.split(**k) == Split::Train).count(), eval_of(c).len())).collect(),
        hreader: (hpairs.len(), hreader.aligned, hreader.null),
        false_friends: ff,
    };
    report::write(&ar.out, &info, &aggs, &worlds, &bagg)?;
    lap("report written");
    // the parallel Tanakh as pages, both ways
    if let Some(root) = &ar.pages {
        // the tables the site's training pages taught (`roundtrip site-learn`)
        if let Some(d) = &ar.site_dict {
            roundtrip::blocks::site_learn::load_dict(d)?;
        }
        if let Some(h) = &ar.site_heb {
            let _ = roundtrip::blocks::site_heb::HEB_LEX.set(roundtrip::blocks::site_heb::read(&std::fs::read_to_string(h)?));
        }
        // the Hebrew encoder, on the reader's alignments of the JPS pairs (training books), and its control
        let epairs: Vec<(&HVerse, &[MTok], &[Option<usize>])> = (0..jps_pairs).map(|i| (hpairs[i].0, hpairs[i].1.as_slice(), hreader.owners[i].as_slice())).collect();
        let henc = heb_enc::HebEncoder::train(&epairs, false);
        let henc_shuf = heb_enc::HebEncoder::train(&epairs, true);
        // the Hebrew stack's lookup of forms, from the training books' trees
        let mut lex_text = String::new();
        for (i, b) in bible::BOOKS.iter().enumerate() {
            if bible::split_of(i as u8) == Split::Train {
                if let Ok(t) = std::fs::read_to_string(ar.wlc.join(format!("{b}.conllu"))) {
                    lex_text.push_str(&t);
                }
            }
        }
        // with the word before each word in its written word (--heb-forms ctx), or without
        let lookup = if ar.heb_forms_ctx { roundtrip::mixed::heb::Lookup::from_conllu_ctx(&lex_text, hbo::text::Level::Pointed) } else { roundtrip::mixed::heb::Lookup::from_conllu(&lex_text, hbo::text::Level::Pointed) };
        let _ = roundtrip::class::HEB_LEMMAS.set(lookup.lemmas.clone());
        lap(&format!("Hebrew encoder: {} meta tokens; lookup of {} lemmas", henc.tokens, lookup.lemmas.len()));
        let judaism = ar.wlc.parent().and_then(|p| p.parent()).map(PathBuf::from).unwrap_or_default();
        let vtext = std::fs::read_to_string(judaism.join("dict/tanakh-jps1917.tsv.verses"))?;
        let verse_of: BTreeMap<String, String> = vtext.lines().filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.split_whitespace().next().unwrap_or("").to_string())).collect();
        let pool = std::fs::read_to_string(ar.pools.join("jps-heb.tsv"))?;
        let pv = std::fs::read_to_string(ar.pools.join("jps-heb.tsv.verses"))?;
        let pverse: BTreeMap<&str, &str> = pv.lines().filter_map(|l| l.split_once('\t')).collect();
        let mut sents: BTreeMap<VKey, Vec<String>> = BTreeMap::new();
        for l in pool.lines() {
            let Some((id, t)) = l.split_once('\t') else { continue };
            if let Some(k) = pverse.get(id).and_then(|v| bible::vkey(v)) {
                sents.entry(k).or_default().push(t.to_string());
            }
        }
        let wlc = aggs.iter().find(|g| g.name == "wlc → jps").context("the wlc → jps system")?;
        let wlc_jps = wlc.recs.iter().filter(|(_, r)| r.has_src).map(|(k, r)| (*k, (r.stages.clone(), r.gate, r.chrf))).collect();
        let books: Vec<&'static str> = bible::BOOKS.iter().enumerate().filter(|(i, _)| bible::split_of(*i as u8) == split).map(|(_, b)| *b).collect();
        let pc = pages::Ctx { a: &a, root: root.clone(), judaism, books, verse_of, lex: &lex, hreader: &hreader, heb: &heb, jpsh: &jpsh, sents, wlc_jps, henc: &henc, henc_shuf: &henc_shuf, lookup: &lookup, threads: ar.threads };
        let (sec, rows) = pages::run(&pc)?;
        let mut rep = std::fs::read_to_string(ar.out.join("report.md"))?;
        rep.push('\n');
        rep.push_str(&sec);
        std::fs::write(ar.out.join("report.md"), rep)?;
        std::fs::write(ar.out.join("pages.tsv"), rows)?;
        lap("pages");
    }
    Ok(())
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn aggregate_and_multiset() {
        assert_eq!(multiset_match(&["x".into(), "x".into()], &["x".into()]), 1);
        assert_eq!(multiset_match(&[], &[]), 0);
        let mut g = Agg::default();
        let t = PVerse {
            forms: vec!["x".into()],
            ..Default::default()
        };
        for i in 0..8 {
            g.add(
                (0, 1, i),
                "",
                &t,
                Out {
                    gate_ok: true,
                    exact: false,
                    chrf: 0.25,
                    fail: Some("fixture".into()),
                    direct_ok: true,
                    ..Default::default()
                },
            );
        }
        assert_eq!((g.verses, g.tokens, g.gate_ok, g.direct_ok), (8, 8, 8, 0));
        assert_eq!((g.fails.len(), g.examples.len()), (4, 1));
        assert_eq!(g.chrf, 2.0);
    }
}
