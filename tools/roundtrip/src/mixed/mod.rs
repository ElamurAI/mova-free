//! `roundtrip mixed`: the meta layer test. Many languages and styles of speech in one stream — one sentence a line
//! shuffled, and documents where English quotes Hebrew and Aramaic — cut into language pieces by `langseg`, each piece
//! read by its own route (English: `en` and `prag`; Hebrew: `hbo` and `gparse`, `macula prag`; Aramaic: tokens only,
//! no parser yet), one world for all of them, and back up to the text, byte for byte in the exact mode.
//!
//!   roundtrip mixed --out <dir> [--n 120] [--docs 300] [--seed N] [--volume 25] [--threads 8] [--model <en.bin>]
//!                   [--school <dir>] [--tales <conllu>] [--judaism <dir>] [--shakespeare <tsv>] [--ud <dir>]
//!                   [--hbo <dir>] [--langseg <model.tsv>] [--gparse <bin>] [--damage]
//!
//! `--damage` is the gate's negative control: the exact decode drops the Hebrew tree residual and the joints, and
//! the volumes that hold either must fail.

pub mod heb;
pub mod link;
pub mod report;
pub mod route;
pub mod sample;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result, anyhow};
use en::annotate::{Annotator, Word};
use hbo::conllu::Sent as HSent;
use langseg::{Lang, Rng, Segmenter, fnv};

use crate::class::{Class, Layer, lit_bits};
use crate::diff::{self, Tk};
use crate::frames::{self, F, Realizer};
use crate::tally::{Diverge, Residual, Start, Tally};
use crate::tree::{self, Dec, TNode};
use crate::vol::{self, Block, Piece as VPiece, Volume};
use crate::world;

use link::{LinkMode, PieceInfo};
use route::{Mode, Split};
use sample::{Item, Paths, Span, Stream, VARIETIES};

/// Which reader a piece goes to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Route {
    /// English of every style: `en` (tokens, tree) and `prag` (frames)
    En,
    /// Hebrew: `hbo` (segmenter, gparse) and `macula prag`
    Heb,
    /// no parser yet (Aramaic, other scripts): the tokens are kept as they are
    Text,
}

impl Route {
    pub fn name(self) -> &'static str {
        match self {
            Route::En => "en",
            Route::Heb => "hebrew",
            Route::Text => "text",
        }
    }
}

/// Everything one run (a stream in a mode) measured.
#[derive(Clone, Debug, Default)]
pub struct Run {
    pub volumes: u64,
    pub exact_ok: u64,
    pub lines: u64,
    pub fails: Vec<String>,
    /// per gold variety (and `glue`): residual, normal mode, counts
    pub by: BTreeMap<String, Tally>,
    /// gold items per variety
    pub items: BTreeMap<String, u64>,
    /// langseg tokens per (gold variety, predicted language)
    pub conf: BTreeMap<(String, String), u64>,
    /// tokens per (gold variety, route right?)
    pub route_ok: BTreeMap<String, (u64, u64)>,
    pub links: link::Stats,
    /// per gold item: how many times it was read, and its residual bits by layer and class
    pub item: BTreeMap<usize, (u64, BTreeMap<(Layer, Class), f64>)>,
}

impl Run {
    fn merge(&mut self, o: &Run) {
        self.volumes += o.volumes;
        self.exact_ok += o.exact_ok;
        self.lines += o.lines;
        for f in &o.fails {
            if self.fails.len() < 4 {
                self.fails.push(f.clone());
            }
        }
        for (k, t) in &o.by {
            self.by.entry(k.clone()).or_default().merge(t);
        }
        for (k, n) in &o.items {
            *self.items.entry(k.clone()).or_default() += n;
        }
        for (k, n) in &o.conf {
            *self.conf.entry(k.clone()).or_default() += n;
        }
        for (k, (a, b)) in &o.route_ok {
            let e = self.route_ok.entry(k.clone()).or_default();
            e.0 += a;
            e.1 += b;
        }
        self.links.merge(&o.links);
        for (k, (n, cells)) in &o.item {
            let e = self.item.entry(*k).or_default();
            e.0 += n;
            for (key, b) in cells {
                *e.1.entry(*key).or_default() += b;
            }
        }
    }
}

/// What the runs share.
pub struct Ctx<'a> {
    pub a: &'a Annotator,
    pub sg: &'a Segmenter,
    pub models: BTreeMap<PathBuf, heb::HModel>,
    pub hbo_dir: PathBuf,
    pub cache: BTreeMap<String, Vec<HSent>>,
    pub lex: link::Lex,
    pub items: Vec<Item>,
    pub rules: macula::prag::Rules,
    /// cut langseg's segments at a document's quote marks and brackets (off: the ablation)
    pub quote_cut: bool,
    /// the gate's negative control (`--damage`): the exact decode drops the Hebrew tree residual and the joints
    pub damage: bool,
}

impl Ctx<'_> {
    fn model(&self, core: &str) -> &heb::HModel {
        &self.models[&hbo::model_dir(&self.hbo_dir, core)]
    }
    fn var_of(&self, k: usize) -> &'static str {
        VARIETIES[self.items[k].var].name
    }
}

/// The gold item that holds most of the bytes of a span.
fn owner(gold: &[Span], a: usize, b: usize) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None;
    for sp in gold {
        let Some(k) = sp.item else { continue };
        let o = sp.end.min(b).saturating_sub(sp.start.max(a));
        if o > 0 && best.is_none_or(|x| o > x.1) {
            best = Some((k, o));
        }
    }
    best.map(|x| x.0)
}

/// The segments of every line of a volume in a mode.
fn volume_segments(ctx: &Ctx, st: &Stream, lines: std::ops::Range<usize>, mode: Mode, vol_index: usize) -> Vec<Vec<(usize, usize, Lang)>> {
    let lang_of = |k: usize| VARIETIES[ctx.items[k].var].lang;
    let mut segs: Vec<Vec<(usize, usize, Lang)>> = lines
        .clone()
        .map(|i| {
            let s = route::segments(ctx.sg, &st.lines[i], &st.gold[i], &lang_of, mode);
            if ctx.quote_cut && matches!(mode, Mode::Langseg | Mode::Shuffle) { route::cut_at_quotes(&st.lines[i], &s) } else { s }
        })
        .collect();
    if mode == Mode::Shuffle {
        route::shuffle(&mut segs, &mut Rng(0x5eed ^ fnv(&st.name) ^ vol_index as u64));
    }
    segs
}

// ---------------------------------------------------------------- one volume

struct EnSent {
    forms: Vec<String>,
    space: Vec<bool>,
    words: Vec<Word>,
    tree: Vec<TNode>,
    r2: tree::R2,
    realized: Vec<TNode>,
    r3: frames::R3,
    /// the residual bucket of the sentence's item
    bucket: usize,
}

struct HSentX {
    tree: heb::HTree,
    toks: Vec<String>,
    r2: Option<heb::R2>,
    realized: Vec<heb::HW>,
    r3: heb::R3,
}

enum Body {
    En { r1: vol::R1, sents: Vec<EnSent> },
    Heb { toks: heb::Toks, r1: heb::R1, sents: Vec<HSentX> },
    Text { toks: heb::Toks, r1: heb::R1 },
}

struct PieceX {
    line: usize,
    text: String,
    route: Route,
    bucket: usize,
    gold_verse: Option<String>,
    body: Body,
    /// indices of this piece's sentences in the volume's frame sentences
    fsents: Vec<usize>,
}

fn chrf_add(d: &mut Diverge, out: &str, orig: &str, ntok: usize) {
    d.sents += 1;
    d.tokens += ntok as u64;
    d.chrf += en::text::chrf(out, orig);
    if out == orig {
        d.exact += 1;
    }
}

fn tks_of(t: &[TNode], order: &[usize], forms: &[String], space: &[bool]) -> Vec<Tk> {
    order.iter().enumerate().map(|(k, &i)| Tk { form: forms[k].clone(), lemma: t[i - 1].lemma.clone(), tag: t[i - 1].tag, rel: t[i - 1].rel, space: space[k] }).collect()
}

fn measure(dec: &Dec, t: &[TNode], orig: &[Tk], orig_s: &str, d: &mut Diverge) {
    if t.is_empty() {
        diff::compare(orig, &[], orig_s, "", d);
        return;
    }
    let (order, forms, space) = dec.decode(t);
    let out = tks_of(t, &order, &forms, &space);
    diff::compare(orig, &out, orig_s, &vol::join(&forms, &space), d);
}

pub(crate) fn run_volume(ctx: &Ctx, st: &Stream, lines: std::ops::Range<usize>, mode: Mode, lmode: LinkMode, vol_index: usize) -> Result<Run> {
    let mut run = Run { volumes: 1, lines: lines.len() as u64, ..Default::default() };
    let segs = volume_segments(ctx, st, lines.clone(), mode, vol_index);
    // routing, measured on langseg's tokens
    for (li, i) in lines.clone().enumerate() {
        let line = &st.lines[i];
        let its: BTreeSet<usize> = st.gold[i].iter().filter_map(|sp| sp.item).collect();
        for k in its {
            *run.items.entry(ctx.var_of(k).to_string()).or_default() += 1;
        }
        for t in langseg::tokenize(line) {
            let Some(k) = st.gold[i].iter().find(|sp| sp.item.is_some() && sp.start <= t.start && t.end <= sp.end).and_then(|sp| sp.item) else { continue };
            let var = ctx.var_of(k);
            let pred = segs[li].iter().find(|s| s.0 <= t.start && t.end <= s.1).map(|s| s.2).unwrap_or(Lang::Other);
            *run.conf.entry((var.to_string(), pred.code().to_string())).or_default() += 1;
            let e = run.route_ok.entry(var.to_string()).or_default();
            e.1 += 1;
            if route::route_of(pred) == VARIETIES[ctx.items[k].var].route {
                e.0 += 1;
            }
        }
    }
    let splits: Vec<Split> = lines.clone().enumerate().map(|(li, i)| route::split(&st.lines[i], &segs[li])).collect();
    // buckets: one residual per gold item, and glue
    let mut bucket_names: Vec<Option<usize>> = Vec::new();
    let mut bidx: BTreeMap<Option<usize>, usize> = BTreeMap::new();
    let mut res: Vec<Residual> = Vec::new();
    let mut tal: Vec<Tally> = Vec::new();
    let mut bucket = |name: Option<usize>, bucket_names: &mut Vec<Option<usize>>, res: &mut Vec<Residual>, tal: &mut Vec<Tally>| -> usize {
        *bidx.entry(name).or_insert_with(|| {
            bucket_names.push(name);
            res.push(Residual::default());
            tal.push(Tally::default());
            bucket_names.len() - 1
        })
    };
    let glue = bucket(None, &mut bucket_names, &mut res, &mut tal);
    // up
    let dec = Dec { a: ctx.a };
    let mut pctx = prag::frames::Ctx::default();
    let mut real = Realizer::default();
    let mut open = false;
    let mut pieces: Vec<PieceX> = Vec::new();
    let mut fsents: Vec<Vec<F>> = Vec::new();
    let mut fowner: Vec<usize> = Vec::new();
    let mut fheb: Vec<bool> = Vec::new();
    let mut joints: Vec<Vec<Option<String>>> = Vec::new();
    for (li, i) in lines.clone().enumerate() {
        let line = &st.lines[i];
        let sp = &splits[li];
        let mut js = Vec::new();
        for (k, p) in sp.pieces.iter().enumerate() {
            if k == 0 {
                js.push(None);
            } else {
                let j = route::joint(line, sp, k);
                if j == " " {
                    js.push(None);
                } else {
                    let j2 = j.to_string();
                    res[glue].add(Layer::Route, Class::Spacing, lit_bits(j) + 1.0, || format!("joint {j2:?}"));
                    js.push(Some(j.to_string()));
                }
            }
            let text = line[p.start..p.end].to_string();
            let own = owner(&st.gold[i], p.start, p.end);
            let b = bucket(own, &mut bucket_names, &mut res, &mut tal);
            let gold_verse = own.and_then(|k| ctx.items[k].verse.clone()).filter(|_| own.is_some_and(|k| VARIETIES[ctx.items[k].var].route == Route::Heb));
            let mut px = PieceX { line: li, text: text.clone(), route: p.route, bucket: b, gold_verse, body: Body::Text { toks: heb::Toks::default(), r1: heb::R1::default() }, fsents: Vec::new() };
            match p.route {
                Route::En => {
                    let seg = vol::segment(ctx.a, &text, 0, false, None)?;
                    let v = Volume { name: String::new(), pieces: vec![VPiece::Text(0)], blocks: vec![Block { text: text.clone(), gold: None, whole: false, marks: Vec::new() }], canon_sep: "\n".into() };
                    let r1 = vol::residual(&v, std::slice::from_ref(&seg), &mut res[b]);
                    let mut sents = Vec::new();
                    let mut pos = 0;
                    for s in &seg.sents {
                        // the sentence's own item: a piece of English can hold several (the sentences of a verse)
                        let a0 = pos + s.gaps.first().map_or(0, String::len);
                        for (g, f) in s.gaps.iter().zip(&s.forms) {
                            pos += g.len() + f.len();
                        }
                        let b = bucket(owner(&st.gold[i], p.start + a0, p.start + pos).or(own), &mut bucket_names, &mut res, &mut tal);
                        let words = ctx.a.annotate(&s.forms);
                        let t = tree::of_words(&words);
                        let space = s.space();
                        let r2 = dec.encode(&t, &s.forms, &space, &mut res[b]);
                        let fs = frames::read(&words, &mut pctx);
                        let realized = real.sentence(&fs);
                        let r3 = frames::encode(&t, &realized, &s.forms, &mut res[b]);
                        tal[b].sents += 1;
                        tal[b].tokens += s.forms.len() as u64;
                        tal[b].frames += fs.len() as u64;
                        px.fsents.push(fsents.len());
                        fsents.push(fs);
                        fowner.push(b);
                        fheb.push(false);
                        sents.push(EnSent { forms: s.forms.clone(), space, words, tree: t, r2, realized, r3, bucket: b });
                    }
                    px.body = Body::En { r1, sents };
                }
                Route::Heb => {
                    let (toks, r1) = heb::tokens_up(&text, &mut res[b]);
                    let (_, _, core) = heb::core(&text);
                    let m = ctx.model(&core);
                    let parsed = ctx.cache.get(&core).ok_or_else(|| anyhow!("no parse for a Hebrew piece: «{core}»"))?;
                    if parsed.len() != toks.sents.len() {
                        return Err(anyhow!("{} sentences parsed, {} tokenized: «{core}»", parsed.len(), toks.sents.len()));
                    }
                    let mut sents = Vec::new();
                    for (s, ps) in parsed.iter().enumerate() {
                        let t = heb::tree_of(ps);
                        let tk = toks.sents[s].clone();
                        let r2 = heb::tree_encode(&m.lookup, &t, &tk, &mut res[b]);
                        let fs = heb::read(&t, &mut open, &ctx.rules);
                        let realized = heb::realize(&fs);
                        let r3 = heb::frames_encode(&t.words, &t.forms, &realized, &mut res[b]);
                        tal[b].sents += 1;
                        tal[b].tokens += tk.len() as u64;
                        tal[b].frames += fs.len() as u64;
                        px.fsents.push(fsents.len());
                        fsents.push(fs);
                        fowner.push(b);
                        fheb.push(true);
                        sents.push(HSentX { tree: t, toks: tk, r2: Some(r2), realized, r3 });
                    }
                    px.body = Body::Heb { toks, r1, sents };
                }
                Route::Text => {
                    let (toks, r1) = heb::tokens_up(&text, &mut res[b]);
                    for s in &toks.sents {
                        heb::unparsed(s, &mut res[b]);
                        tal[b].sents += 1;
                        tal[b].tokens += s.len() as u64;
                    }
                    px.body = Body::Text { toks, r1 };
                }
            }
            pieces.push(px);
        }
        joints.push(js);
    }
    // one world: entities of all languages, a Hebrew and an English one joined only by the verse or the name
    let infos: Vec<PieceInfo> = pieces
        .iter()
        .filter(|p| p.route != Route::Text)
        .map(|p| {
            let heb = p.route == Route::Heb;
            let verses: BTreeSet<String> = if heb {
                match &p.body {
                    Body::Heb { toks, .. } => toks.sents.iter().filter_map(|s| ctx.lex.wlc_verse(&s.join(" ")).cloned()).collect(),
                    _ => BTreeSet::new(),
                }
            } else {
                sample::sentences(&p.text).iter().filter_map(|s| ctx.lex.jps_verse(s).cloned()).collect()
            };
            PieceInfo { heb, verses, gold_verse: p.gold_verse.clone(), frames: p.fsents.iter().flat_map(|&s| fsents[s].iter()).collect() }
        })
        .collect();
    let (lmap, lstats) = link::link(&ctx.lex, &infos, lmode);
    run.links = lstats;
    let back = link::inverse(&lmap);
    let wsents: Vec<Vec<F>> = fsents.iter().zip(&fheb).map(|(fs, &h)| if h { link::rename(fs, &lmap) } else { fs.clone() }).collect();
    let (w, r4) = world::encode_by(&wsents, &fowner, &mut res);
    tal[glue].events += w.line.len() as u64;
    // down, exact: world → frames → tree → tokens → pieces → lines
    let fs2: Vec<Vec<F>> = world::decode_exact(&w, &r4, wsents.len()).into_iter().zip(&fheb).map(|(fs, &h)| if h { link::rename(&fs, &back) } else { fs }).collect();
    let mut bad: Option<String> = (0..fsents.len()).find(|&s| fs2.get(s) != fsents.get(s)).map(|s| format!("{}: frames differ at frame sentence {s}", st.name));
    let mut real2 = Realizer::default();
    let mut texts: Vec<Vec<String>> = vec![Vec::new(); lines.len()];
    for p in &pieces {
        let text = match &p.body {
            Body::En { r1, sents } => {
                let mut toks2 = Vec::new();
                for (k, s) in sents.iter().enumerate() {
                    let rt = real2.sentence(&fs2[p.fsents[k]]);
                    let t2 = frames::apply(&rt, &s.r3);
                    if bad.is_none() && t2 != s.tree {
                        bad = Some(format!("{}: English tree differs: «{}»", st.name, p.text));
                    }
                    let (f2, s2) = dec.decode_exact(&t2, &s.r2);
                    if bad.is_none() && (f2 != s.forms || s2 != s.space) {
                        bad = Some(format!("{}: English tokens differ: «{}»", st.name, p.text));
                    }
                    toks2.push((f2, s2));
                }
                vol::decode_blocks(1, &[toks2], r1).remove(0)
            }
            Body::Heb { toks, r1, sents } => {
                let (_, _, core) = heb::core(&p.text);
                let m = ctx.model(&core);
                let mut out = Vec::new();
                for (k, s) in sents.iter().enumerate() {
                    let r = heb::realize(&fs2[p.fsents[k]]);
                    let words = heb::frames_apply(&r, &s.r3);
                    if bad.is_none() && words != s.tree.words {
                        bad = Some(format!("{}: Hebrew tree differs: «{}»", st.name, p.text));
                    }
                    let (_, _, tk) = heb::tree_decode(&m.lookup, &words, s.r2.as_ref().filter(|_| !ctx.damage));
                    if bad.is_none() && tk != s.toks {
                        bad = Some(format!("{}: Hebrew tokens differ: «{}» vs «{}»", st.name, tk.join(" "), s.toks.join(" ")));
                    }
                    out.push(tk);
                }
                let _ = toks;
                heb::tokens_down(&heb::Toks { sents: out }, r1)
            }
            Body::Text { toks, r1 } => heb::tokens_down(toks, r1),
        };
        texts[p.line].push(text);
    }
    let mut ok = bad.is_none();
    for (li, i) in lines.clone().enumerate() {
        let l = if ctx.damage { route::join(&texts[li], &vec![None; joints[li].len()]) } else { route::join(&texts[li], &joints[li]) };
        if l != st.lines[i] {
            ok = false;
            if bad.is_none() {
                bad = Some(format!("{}: line {i} differs: «{l}»", st.name));
            }
        }
    }
    if ok {
        run.exact_ok = 1;
    } else {
        run.fails.push(bad.unwrap_or_default());
    }
    // down, normal: from every start, no residual
    let wfs: Vec<Vec<F>> = world::decode_in(&w, &wsents).into_iter().zip(&fheb).map(|(fs, &h)| if h { link::rename(&fs, &back) } else { fs }).collect();
    let mut rw = Realizer::default();
    for p in &pieces {
        let b = p.bucket;
        match &p.body {
            Body::En { sents, .. } => {
                for (k, s) in sents.iter().enumerate() {
                    let d = &mut tal[s.bucket].normal;
                    let orig: Vec<Tk> = s.words.iter().enumerate().map(|(j, x)| Tk { form: x.form.clone(), lemma: x.lemma.clone(), tag: x.tag, rel: x.rel, space: s.space[j] }).collect();
                    let tok_s = vol::join(&s.forms, &s.space);
                    diff::compare(&orig, &orig, &tok_s, &tok_s, d.entry(Start::Tokens).or_default());
                    measure(&dec, &s.tree, &orig, &tok_s, d.entry(Start::Tree).or_default());
                    measure(&dec, &s.realized, &orig, &tok_s, d.entry(Start::Frames).or_default());
                    let tw = rw.sentence(&wfs[p.fsents[k]]);
                    measure(&dec, &tw, &orig, &tok_s, d.entry(Start::World).or_default());
                }
            }
            Body::Heb { sents, toks, .. } => {
                let (_, _, core) = heb::core(&p.text);
                let m = ctx.model(&core);
                let _ = toks;
                let d = &mut tal[b].normal;
                for (k, s) in sents.iter().enumerate() {
                    let orig = heb::join_tokens(std::slice::from_ref(&s.toks));
                    let n = s.toks.len();
                    chrf_add(d.entry(Start::Tokens).or_default(), &orig, &orig, n);
                    let from = |words: &[heb::HW]| heb::join_tokens(&[heb::tree_decode(&m.lookup, words, None).2]);
                    chrf_add(d.entry(Start::Tree).or_default(), &from(&s.tree.words), &orig, n);
                    chrf_add(d.entry(Start::Frames).or_default(), &from(&s.realized), &orig, n);
                    chrf_add(d.entry(Start::World).or_default(), &from(&heb::realize(&wfs[p.fsents[k]])), &orig, n);
                }
            }
            Body::Text { toks, .. } => {
                let d = &mut tal[b].normal;
                for s in &toks.sents {
                    let orig = heb::join_tokens(std::slice::from_ref(s));
                    chrf_add(d.entry(Start::Tokens).or_default(), &orig, &orig, s.len());
                }
            }
        }
    }
    for (b, name) in bucket_names.iter().enumerate() {
        let mut t = std::mem::take(&mut tal[b]);
        t.residual = std::mem::take(&mut res[b]);
        if let Some(k) = name {
            let e = run.item.entry(*k).or_default();
            e.0 += 1;
            for (key, c) in &t.residual.cells {
                *e.1.entry(*key).or_default() += c.bits;
            }
        }
        let v = name.map_or("glue", |k| ctx.var_of(k)).to_string();
        run.by.entry(v).or_default().merge(&t);
    }
    Ok(run)
}

// ---------------------------------------------------------------- runs

/// The Hebrew pieces of a stream in a mode: their cores.
pub(crate) fn hebrew_cores(ctx: &Ctx, st: &Stream, mode: Mode, vol: usize) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (vi, a) in (0..st.lines.len()).step_by(vol.max(1)).enumerate() {
        let r = a..(a + vol).min(st.lines.len());
        let segs = volume_segments(ctx, st, r.clone(), mode, vi);
        for (li, i) in r.enumerate() {
            let line = &st.lines[i];
            for p in route::split(line, &segs[li]).pieces {
                if p.route == Route::Heb {
                    out.insert(heb::core(&line[p.start..p.end]).2);
                }
            }
        }
    }
    out
}

/// Parse the cores not in the cache, one `gparse` call per model; on a failure (the shared GPU out of memory) wait ten
/// minutes and try again, three times at most.
pub(crate) fn parse_cores(ctx: &mut Ctx, cores: &BTreeSet<String>, gparse: &str, out: &Path) -> Result<()> {
    let mut by: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    for c in cores {
        if !ctx.cache.contains_key(c) {
            by.entry(hbo::model_dir(&ctx.hbo_dir, c)).or_default().push(c.clone());
        }
    }
    for (dir, cs) in by {
        let m = &ctx.models[&dir];
        let mut flat: Vec<HSent> = Vec::new();
        let mut ranges = Vec::new();
        for c in &cs {
            let a = flat.len();
            flat.extend(hbo::segment_line(&m.seg, c, a + 1));
            ranges.push(a..flat.len());
        }
        let name = dir.file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        let tmp = out.join(format!("hbo-{name}-{}", ctx.cache.len()));
        let mut tries = 0;
        loop {
            match hbo::annotate(&dir, &m.seg, &mut flat, gparse, &tmp.to_string_lossy()) {
                Ok(()) => break,
                Err(e) if tries < 3 => {
                    tries += 1;
                    eprintln!("gparse failed ({e}); waiting ten minutes, try {tries} of 3");
                    std::thread::sleep(std::time::Duration::from_secs(600));
                }
                Err(e) => return Err(anyhow!(e)),
            }
        }
        let mut dump = String::new();
        for (c, r) in cs.iter().zip(ranges) {
            dump += &format!("# core = {c}\n{}", hbo::conllu::write(&flat[r.clone()]));
            ctx.cache.insert(c.clone(), flat[r].to_vec());
        }
        std::fs::write(out.join(format!("hbo-{name}.conllu")), dump)?;
        eprintln!("parsed {} Hebrew pieces with {name}", cs.len());
    }
    Ok(())
}

fn run_stream(ctx: &Ctx, st: &Stream, mode: Mode, lmode: LinkMode, vol: usize, threads: usize) -> Result<Run> {
    let ranges: Vec<std::ops::Range<usize>> = (0..st.lines.len()).step_by(vol.max(1)).map(|a| a..(a + vol).min(st.lines.len())).collect();
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<Result<Run>>>> = Mutex::new((0..ranges.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    if i >= ranges.len() {
                        break;
                    }
                    let r = run_volume(ctx, st, ranges[i].clone(), mode, lmode, i);
                    out.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    let mut run = Run::default();
    for r in out.into_inner().unwrap() {
        run.merge(&r.unwrap()?);
    }
    Ok(run)
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned()
}

pub fn main(args: &[String], model: &Path) -> Result<()> {
    let out = PathBuf::from(flag(args, "--out").context("--out <dir>")?);
    let md = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default())));
    let p = |name: &str, def: PathBuf| flag(args, name).map(PathBuf::from).unwrap_or(def);
    let paths = Paths {
        school: p("--school", md.join("school/courses")),
        tales: p("--tales", md.join("corpus-0.3/treebanks/mova-tales-silver-en.conllu")),
        judaism: p("--judaism", md.join("corpora/judaism")),
        shakespeare: p("--shakespeare", md.join("corpora/emode/text/shakespeare.tsv")),
        ud: p("--ud", md.join("treebanks/ud-2.18")),
        hbo: p("--hbo", md.join("models/hbo")),
        langseg: p("--langseg", md.join("langseg/model.tsv")),
        gparse: flag(args, "--gparse").unwrap_or_else(|| "gparse".into()),
    };
    let n: usize = flag(args, "--n").map(|x| x.parse()).transpose()?.unwrap_or(120);
    let ndocs: usize = flag(args, "--docs").map(|x| x.parse()).transpose()?.unwrap_or(300);
    let seed: u64 = flag(args, "--seed").map(|x| x.parse()).transpose()?.unwrap_or(20261018);
    let vol: usize = flag(args, "--volume").map(|x| x.parse()).transpose()?.unwrap_or(25);
    let threads: usize = flag(args, "--threads").map(|x| x.parse()).transpose()?.unwrap_or(8);
    std::fs::create_dir_all(&out)?;
    let t0 = std::time::Instant::now();
    let items = sample::sample(&paths, n, seed)?;
    let streams = sample::streams(&items, seed, ndocs);
    sample::write(&out.join("streams"), &items, &streams)?;
    eprintln!("{} items, {} streams in {:.1}s", items.len(), streams.len(), t0.elapsed().as_secs_f64());
    let a = Annotator::load(model)?;
    let lm = langseg::Model::load(&paths.langseg).map_err(|e| anyhow!(e))?;
    let sg = Segmenter::new(lm, 16.0);
    let mut models = BTreeMap::new();
    let mut lemmas = BTreeSet::new();
    for lvl in ["pointed", "unpointed"] {
        let m = heb::HModel::load(&paths.hbo.join(lvl))?;
        lemmas.extend(m.lemmas.iter().cloned());
        models.insert(paths.hbo.join(lvl), m);
    }
    let _ = crate::class::HEB_LEMMAS.set(lemmas);
    let lex = link::Lex::load(&paths.judaism)?;
    eprintln!("models loaded in {:.1}s", t0.elapsed().as_secs_f64());
    let mut ctx = Ctx { a: &a, sg: &sg, models, hbo_dir: paths.hbo.clone(), cache: BTreeMap::new(), lex, items, rules: macula::prag::Rules::new(&[], &[]), quote_cut: true, damage: args.iter().any(|a| a == "--damage") };
    // the runs: every stream with langseg and the oracle; the mixed streams with the two controls too
    let mut plan: Vec<(usize, Mode, LinkMode)> = Vec::new();
    for (si, s) in streams.iter().enumerate() {
        let modes: &[Mode] = if s.name.starts_with("mono-") { &[Mode::Langseg, Mode::Oracle] } else { &[Mode::Langseg, Mode::Oracle, Mode::AllEn, Mode::Shuffle] };
        for &m in modes {
            plan.push((si, m, LinkMode::Align));
        }
        if !s.name.starts_with("mono-") {
            plan.push((si, Mode::Langseg, LinkMode::Any));
        }
    }
    let docs = streams.iter().position(|s| s.name == "docs");
    let mut cores = BTreeSet::new();
    for &(si, m, _) in &plan {
        cores.extend(hebrew_cores(&ctx, &streams[si], m, vol));
    }
    ctx.quote_cut = false;
    if let Some(d) = docs {
        cores.extend(hebrew_cores(&ctx, &streams[d], Mode::Langseg, vol));
    }
    ctx.quote_cut = true;
    parse_cores(&mut ctx, &cores, &paths.gparse, &out)?;
    let mut runs: Vec<(String, Mode, LinkMode, Run)> = Vec::new();
    for &(si, m, lmode) in &plan {
        let t1 = std::time::Instant::now();
        let r = run_stream(&ctx, &streams[si], m, lmode, vol, threads)?;
        eprintln!("{} / {}{}: {} lines, exact {}/{} volumes in {:.1}s", streams[si].name, m.name(), if lmode == LinkMode::Any { " / link-any" } else { "" }, r.lines, r.exact_ok, r.volumes, t1.elapsed().as_secs_f64());
        runs.push((streams[si].name.clone(), m, lmode, r));
    }
    // the ablation: langseg's segments not cut at the quote marks
    if let Some(d) = docs {
        ctx.quote_cut = false;
        let r = run_stream(&ctx, &streams[d], Mode::Langseg, LinkMode::Align, vol, threads)?;
        eprintln!("docs / langseg, segments not cut at quotes: exact {}/{} volumes", r.exact_ok, r.volumes);
        runs.push(("docs:no-quote-cut".into(), Mode::Langseg, LinkMode::Align, r));
        ctx.quote_cut = true;
    }
    let md = report::report(&ctx, &streams, &runs, n, seed, vol);
    std::fs::write(out.join("report.md"), &md)?;
    std::fs::write(out.join("residual.tsv"), report::tsv(&runs))?;
    std::fs::write(out.join("items.tsv"), report::items_tsv(&ctx, &runs))?;
    let links: String = runs.iter().filter(|r| r.0 == "docs" && r.1 == Mode::Langseg && r.2 == LinkMode::Align).flat_map(|r| r.3.links.examples.iter().map(|x| format!("{x}\n"))).collect();
    std::fs::write(out.join("links.tsv"), links)?;
    println!("{md}");
    Ok(())
}
#[cfg(test)]
mod wave32_tests {
    use super::*;
    #[test]
    fn owner_and_run_merge() {
        let spans = vec![
            Span {
                start: 0,
                end: 2,
                item: Some(7),
            },
            Span {
                start: 2,
                end: 4,
                item: Some(9),
            },
        ];
        assert_eq!(owner(&spans, 0, 4), Some(7));
        assert_eq!(owner(&spans, 4, 4), None);
        let mut a = Run::default();
        let mut b = Run::default();
        b.lines = 2;
        b.route_ok.insert("x".into(), (1, 2));
        b.fails = (0..6).map(|x| x.to_string()).collect();
        a.merge(&b);
        a.merge(&b);
        assert_eq!(a.lines, 4);
        assert_eq!(a.route_ok["x"], (2, 4));
        assert_eq!(a.fails, ["0", "1", "2", "3"]);
        let mut d = Diverge::default();
        chrf_add(&mut d, "", "", 0);
        assert_eq!((d.sents, d.exact, d.tokens), (1, 1, 0));
    }
}
