//! Alignment of OEWN synsets ↔ Wikidata items with cross-checking of hierarchies.
//!
//! **Candidates** (synset ↔ Q pairs) from four sources:
//! - `lex` — senses of English lexemes with P5137 "item for this sense", tied to a synset
//!   (`wd-map.tsv`: unambiguous, by gloss, P8814);
//! - `ili` — P5063 on the item (Interlingual Index) = ILI of the OEWN synset;
//! - `wn` — P8814 on the item (WordNet 3.1 synset; the offset matches the OEWN id);
//! - `label` — English label or alias of the item = lemma of a noun synset (case-insensitive). The item is
//!   a class or a taxon (a node of the "is a kind of" graph) or has a Wikipedia article.
//!
//! **Rule:** a pair is accepted only when the ancestors are consistently aligned too — there is a support pair (s', q') among
//! the candidates, where s' is a proper ancestor of s in OEWN (IsA/InstanceOf, up to 10 steps, not too general), and q' is
//! a proper ancestor of q in Wikidata (P279/P171, a P31 step at the start, up to 20 steps). Among the accepted Qs of a synset
//! the best is: direct source, consistent glosses, shorter support, more popular.
//!
//! **Negative control:** shuffle Q among the candidates of each source (labels ↔ items) — accepted pairs
//! must drop by an order of magnitude.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};

use crate::model::{Cid, Graph, Pos, Rel, Src};
use crate::wdgraph::{for_each_row, parts, read_items, IsaGraph, ItemInfo, P171, P279};
use crate::wdx::thousands;

pub const S_LEX: u8 = 1;
pub const S_ILI: u8 = 2;
pub const S_WN: u8 = 4;
pub const S_LABEL: u8 = 8;
pub const DIRECT: u8 = S_LEX | S_ILI | S_WN;

pub fn src_names(m: u8) -> String {
    let mut v = Vec::new();
    for (b, n) in [(S_LEX, "lex"), (S_ILI, "ili"), (S_WN, "wn"), (S_LABEL, "label")] {
        if m & b != 0 {
            v.push(n);
        }
    }
    v.join("+")
}

/// Support search limits.
pub const OEWN_DEPTH: u16 = 10;
pub const WD_DEPTH: u16 = 20;

/// OEWN side: noun synsets, lemmas, ancestors, "too general" concepts.
pub struct Oewn<'a> {
    pub g: &'a Graph,
    /// lemma (lowercase) → noun synsets
    pub lemma: HashMap<String, Vec<Cid>>,
    pub by_key: HashMap<(u32, u8), Cid>,
    pub by_ili: HashMap<u32, Cid>,
    /// how many noun synsets have this concept as an ancestor
    pub desc: Vec<u32>,
    pub n_nouns: usize,
    /// "too general" threshold: an ancestor with more synsets beneath it will not be a support
    pub generic_limit: u32,
}

impl<'a> Oewn<'a> {
    pub fn new(g: &'a Graph, generic_share: f64) -> Oewn<'a> {
        let n = g.n_concepts();
        let mut lemma: HashMap<String, Vec<Cid>> = HashMap::new();
        let mut by_key = HashMap::new();
        let mut by_ili = HashMap::new();
        let mut desc = vec![0u32; n];
        let mut n_nouns = 0;
        for c in 0..n as Cid {
            if g.src[c as usize] != Src::Oewn {
                continue;
            }
            by_key.insert((g.key[c as usize], g.pos[c as usize].letter() as u8), c);
            if g.ili[c as usize] != 0 {
                by_ili.insert(g.ili[c as usize], c);
            }
            if g.pos[c as usize] != Pos::Noun {
                continue;
            }
            n_nouns += 1;
            for w in g.members(c) {
                let e = lemma.entry(g.word(*w).to_lowercase()).or_default();
                if !e.contains(&c) {
                    e.push(c);
                }
            }
            for (a, _) in oewn_ancestors(g, c, 64) {
                desc[a as usize] += 1;
            }
        }
        let generic_limit = (n_nouns as f64 * generic_share) as u32;
        Oewn { g, lemma, by_key, by_ili, desc, n_nouns, generic_limit }
    }

    pub fn find(&self, id: &str) -> Option<Cid> {
        // `oewn-05599617-n` or `05599617-n`
        let rest = id.strip_prefix("oewn-").unwrap_or(id);
        let (num, p) = rest.split_once('-')?;
        let key: u32 = num.parse().ok()?;
        let pos = p.bytes().next()?;
        self.by_key.get(&(key, pos)).copied().or_else(|| if pos == b'a' { self.by_key.get(&(key, b's')).copied() } else { None })
    }
}

/// Proper ancestors of a synset in OEWN (IsA, InstanceOf) with the shortest depth.
pub fn oewn_ancestors(g: &Graph, c: Cid, max: u16) -> Vec<(Cid, u16)> {
    let mut seen: HashMap<Cid, u16> = HashMap::new();
    seen.insert(c, 0);
    let mut q = std::collections::VecDeque::from([c]);
    let mut out = Vec::new();
    while let Some(x) = q.pop_front() {
        let d = seen[&x];
        if d >= max {
            continue;
        }
        for e in g.out_edges(x) {
            if !matches!(e.rel, Rel::IsA | Rel::InstanceOf) {
                continue;
            }
            if let std::collections::hash_map::Entry::Vacant(v) = seen.entry(e.to) {
                v.insert(d + 1);
                out.push((e.to, d + 1));
                q.push_back(e.to);
            }
        }
    }
    out
}

/// Wikidata side: the "is a kind of" graph and P31 of candidates.
pub struct Wd {
    pub g: IsaGraph,
    pub p31: HashMap<u32, Vec<u32>>,
    /// node taxon rank (P105) for the folk rule: P171 stops at class (`wdwalk`)
    pub rk: Vec<crate::wdwalk::RankKind>,
    /// true — folk ancestors (v2), false — full cladistics (v1)
    pub folk: bool,
}

impl Wd {
    /// Proper ancestors of an item: P279/P171 upward; P31 only as the first step (for instances).
    pub fn ancestors(&self, q: u32) -> HashMap<u32, u16> {
        let mut out: HashMap<u32, u16> = HashMap::new();
        let mut starts: Vec<(u32, u16)> = Vec::new();
        if let Some(v) = self.g.idx(q) {
            starts.push((v, 0));
        }
        if let Some(cs) = self.p31.get(&q) {
            for &c in cs {
                out.entry(c).or_insert(1);
                if let Some(v) = self.g.idx(c) {
                    starts.push((v, 1));
                }
            }
        }
        for (v, d0) in starts {
            for (a, d) in crate::wdwalk::folk_ancestors(&self.g, &self.rk, v, WD_DEPTH.saturating_sub(d0), self.folk) {
                let qa = self.g.nodes[a as usize];
                let dd = d + d0;
                let e = out.entry(qa).or_insert(dd);
                if dd < *e {
                    *e = dd;
                }
            }
        }
        out.remove(&q);
        out
    }
}

/// Candidate with evidence.
#[derive(Clone, Debug)]
pub struct Cand {
    pub s: Cid,
    pub q: u32,
    pub src: u8,
    /// support: (s', q', depth of s', depth of q')
    pub support: Option<(Cid, u32, u16, u16)>,
    pub status: Status,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Accepted,
    Also,
    /// ancestor support exists, but only one source (v2 requires two supports)
    Single,
    Unsupported,
}

impl Status {
    pub fn code(self) -> &'static str {
        match self {
            Status::Accepted => "accepted",
            Status::Also => "also",
            Status::Single => "single",
            Status::Unsupported => "unsupported",
        }
    }
}

/// Candidates from all sources: (s, q) → source mask.
pub fn gather(o: &Oewn, dir: &Path, wd_map: &Path, eligible: &(dyn Fn(u32) -> bool + Sync)) -> Result<(HashMap<(Cid, u32), u8>, HashMap<&'static str, u64>)> {
    let mut c: HashMap<(Cid, u32), u8> = HashMap::new();
    let mut n: HashMap<&'static str, u64> = HashMap::new();
    // lex: wd-map.tsv (Q, synset, lemma, method, …)
    let text = std::fs::read_to_string(wd_map).with_context(|| format!("cannot read {}", wd_map.display()))?;
    for l in text.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 4 {
            continue;
        }
        let (Some(q), Some(s)) = (crate::wdx::num_id(f[0], b'Q'), o.find(f[1])) else { continue };
        *c.entry((s, q)).or_default() |= S_LEX;
        *n.entry("lex").or_default() += 1;
    }
    // ili and wn: string claims of items
    for p in parts(dir, "strs")? {
        for_each_row(&p, |f| {
            if f.len() < 4 || f[3] == "0" {
                return Ok(());
            }
            let q: u32 = f[0].parse()?;
            match f[1] {
                "5063" => {
                    if let Some(s) = f[2].trim().strip_prefix('i').and_then(|x| x.parse::<u32>().ok()).and_then(|i| o.by_ili.get(&i).copied()) {
                        *c.entry((s, q)).or_default() |= S_ILI;
                        *n.entry("ili").or_default() += 1;
                    } else {
                        *n.entry("ili without OEWN synset").or_default() += 1;
                    }
                }
                "8814" => {
                    if let Some(s) = o.find(f[2].trim()) {
                        *c.entry((s, q)).or_default() |= S_WN;
                        *n.entry("wn").or_default() += 1;
                    } else {
                        *n.entry("wn without OEWN synset").or_default() += 1;
                    }
                }
                _ => {}
            }
            Ok(())
        })?;
    }
    // label: English labels and aliases (`labels`), and for items without exported labels — `items.label_en`
    let add_label = |q: u32, text: &str, c: &mut HashMap<(Cid, u32), u8>, n: &mut HashMap<&'static str, u64>| {
        if let Some(ss) = o.lemma.get(&text.to_lowercase()) {
            for &s in ss {
                *c.entry((s, q)).or_default() |= S_LABEL;
                *n.entry("label").or_default() += 1;
            }
        }
    };
    for p in parts(dir, "labels")? {
        for_each_row(&p, |f| {
            if f.len() < 4 || f[1] != "en" {
                return Ok(());
            }
            let q: u32 = f[0].parse()?;
            if eligible(q) {
                add_label(q, f[2], &mut c, &mut n);
            }
            Ok(())
        })?;
    }
    for p in parts(dir, "items")? {
        for_each_row(&p, |f| {
            if f.len() < 10 {
                return Ok(());
            }
            let flags: u32 = f[9].parse()?;
            // items with exported labels were handled above; here — the rest (en only)
            if flags & crate::wdx::F_LABELS != 0 || f[1].is_empty() {
                return Ok(());
            }
            let q: u32 = f[0].parse()?;
            if eligible(q) {
                add_label(q, f[1], &mut c, &mut n);
            }
            Ok(())
        })?;
    }
    Ok((c, n))
}

/// Source supports of a candidate: label, lexeme, direct link (ILI or P8814 count as one support: they are set together).
pub fn n_sources(m: u8) -> u8 {
    (m & S_LABEL != 0) as u8 + (m & S_LEX != 0) as u8 + (m & (S_ILI | S_WN) != 0) as u8
}

/// Support and choice. `cands` — candidates (s, q, sources); acceptance needs at least `min_src` source supports. `overlap` — words shared by the synset gloss and
/// the item label with description (only for choosing among supported Qs). Returns all of them, marked.
pub fn decide(o: &Oewn, wd: &Wd, info_wiki: &dyn Fn(u32) -> u16, overlap: &dyn Fn(Cid, u32) -> usize, cands: &[(Cid, u32, u8)], min_src: u8) -> Vec<Cand> {
    let mut anchors: HashMap<Cid, Vec<u32>> = HashMap::new();
    for &(s, q, _) in cands {
        anchors.entry(s).or_default().push(q);
    }
    let mut oanc_cache: HashMap<Cid, Vec<(Cid, u16)>> = HashMap::new();
    let mut idx: Vec<usize> = (0..cands.len()).collect();
    idx.sort_by_key(|&i| cands[i].1);
    let mut out: Vec<Cand> = cands.iter().map(|&(s, q, src)| Cand { s, q, src, support: None, status: Status::Unsupported }).collect();
    let mut i = 0;
    while i < idx.len() {
        let q = cands[idx[i]].1;
        let mut j = i;
        while j < idx.len() && cands[idx[j]].1 == q {
            j += 1;
        }
        let wanc = wd.ancestors(q);
        if !wanc.is_empty() {
            for &k in &idx[i..j] {
                let s = cands[k].0;
                let oanc = oanc_cache.entry(s).or_insert_with(|| {
                    oewn_ancestors(o.g, s, OEWN_DEPTH).into_iter().filter(|(a, _)| o.desc[*a as usize] <= o.generic_limit).collect()
                });
                let mut best: Option<(Cid, u32, u16, u16)> = None;
                for &(sa, ds) in oanc.iter() {
                    let Some(qs) = anchors.get(&sa) else { continue };
                    for &qa in qs {
                        if qa == q {
                            continue;
                        }
                        if let Some(&dq) = wanc.get(&qa) {
                            if best.map(|b| ds + dq < b.2 + b.3).unwrap_or(true) {
                                best = Some((sa, qa, ds, dq));
                            }
                        }
                    }
                }
                out[k].support = best;
            }
        }
        i = j;
    }
    // choice per synset: direct source > consistent glosses (≥ 2 shared words) > shorter support > more popular
    let mut by_s: HashMap<Cid, Vec<usize>> = HashMap::new();
    for (k, c) in out.iter_mut().enumerate() {
        if c.support.is_some() {
            if n_sources(c.src) >= min_src {
                by_s.entry(c.s).or_default().push(k);
            } else {
                c.status = Status::Single;
            }
        }
    }
    for (_, mut ks) in by_s {
        ks.sort_by_key(|&k| {
            let c = &out[k];
            let (_, _, ds, dq) = c.support.unwrap_or((0, 0, 99, 99));
            let gloss_agrees = ks_overlap(overlap, c.s, c.q) >= 2;
            (c.src & DIRECT == 0, !gloss_agrees, ds + dq, std::cmp::Reverse(info_wiki(c.q)), c.q)
        });
        for (r, k) in ks.into_iter().enumerate() {
            out[k].status = if r == 0 { Status::Accepted } else { Status::Also };
        }
    }
    out
}

fn ks_overlap(f: &dyn Fn(Cid, u32) -> usize, s: Cid, q: u32) -> usize {
    f(s, q)
}

/// Bitmap of Qs with an article in at least one Wikipedia (`items.n_wiki > 0`).
pub fn wiki_bits(dir: &Path) -> Result<Vec<u64>> {
    let files = parts(dir, "items")?;
    let res: Vec<Result<Vec<u32>>> = std::thread::scope(|sc| {
        let hs: Vec<_> = files
            .iter()
            .map(|p| {
                sc.spawn(move || -> Result<Vec<u32>> {
                    let mut v = Vec::new();
                    for_each_row(p, |f| {
                        if f.len() >= 10 && f[7] != "0" {
                            v.push(f[0].parse()?);
                        }
                        Ok(())
                    })?;
                    Ok(v)
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| Err(anyhow::anyhow!("thread panicked")))).collect()
    });
    let mut bits: Vec<u64> = Vec::new();
    for r in res {
        for q in r? {
            let w = (q >> 6) as usize;
            if bits.len() <= w {
                bits.resize(w + 1, 0);
            }
            bits[w] |= 1 << (q & 63);
        }
    }
    Ok(bits)
}

/// Deterministic generator (xorshift) for shuffling and sampling.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.max(1))
    }
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
}

/// Shuffle Q among the candidates of each source separately (labels ↔ items).
pub fn shuffled(cands: &[(Cid, u32, u8)], seed: u64) -> Vec<(Cid, u32, u8)> {
    let mut rng = Rng::new(seed);
    let mut out = cands.to_vec();
    for m in [S_LEX, S_ILI, S_WN, S_LABEL] {
        let ids: Vec<usize> = (0..out.len()).filter(|&i| out[i].2 & m != 0 && out[i].2 & !m & (S_LEX | S_ILI | S_WN | S_LABEL) == 0).collect();
        let mut qs: Vec<u32> = ids.iter().map(|&i| out[i].1).collect();
        for i in (1..qs.len()).rev() {
            let j = rng.below(i + 1);
            qs.swap(i, j);
        }
        for (k, &i) in ids.iter().enumerate() {
            out[i].1 = qs[k];
        }
    }
    // candidates with several sources also get a random Q from the whole pool
    let pool: Vec<u32> = cands.iter().map(|c| c.1).collect();
    for c in out.iter_mut() {
        if c.2.count_ones() > 1 {
            c.1 = pool[rng.below(pool.len())];
        }
    }
    out
}

pub struct AlignRun {
    pub report: String,
    pub rows: Vec<Cand>,
}

fn tally(v: &[Cand]) -> (usize, usize, usize) {
    let acc = v.iter().filter(|c| c.status == Status::Accepted).count();
    let also = v.iter().filter(|c| c.status == Status::Also).count();
    (acc, also, v.len())
}

/// Full alignment pass with control and sample.
#[allow(clippy::too_many_arguments)]
pub fn run(g: &Graph, dir: &Path, runs: &Path, wd_map: &Path, generic_share: f64, seed: u64, sample_n: usize, min_src: u8, folk: bool, gold: &Path) -> Result<AlignRun> {
    use std::fmt::Write as _;
    let t0 = Instant::now();
    let o = Oewn::new(g, generic_share);
    let t_oewn = t0.elapsed().as_secs_f64();
    // the "is a kind of" graph after breaking cycles
    let mut edges: Vec<(u32, u32, u16)> = Vec::new();
    for_each_row(&dir.join("isa_edges.tsv.zst"), |f| {
        edges.push((f[0].parse()?, f[1].parse()?, f[2].parse()?));
        Ok(())
    })?;
    let isa = IsaGraph::from_edges(edges);
    // "class or with an article": bitmap of items with a Wikipedia article (140 million bits — ~18 MB)
    let has_wiki = wiki_bits(dir)?;
    let eligible = |q: u32| isa.idx(q).is_some() || has_wiki.get((q >> 6) as usize).map(|w| w >> (q & 63) & 1 == 1).unwrap_or(false);
    let (cmap, counts) = gather(&o, dir, wd_map, &eligible)?;
    let t_gather = t0.elapsed().as_secs_f64();
    let qset: HashSet<u32> = cmap.keys().map(|k| k.1).collect();
    // popularity, label and description — only for candidates
    let info = read_items(dir, &|q| qset.contains(&q), &|q| qset.contains(&q))?;
    let wiki = |q: u32| info.n_wiki.get(&q).copied().unwrap_or(0);
    let mut otoks: HashMap<Cid, HashSet<String>> = HashMap::new();
    for &(s, _) in cmap.keys() {
        otoks.entry(s).or_insert_with(|| {
            let mut t = crate::wikidata::toks(g.gloss_of(s));
            for w in g.members(s) {
                t.extend(crate::wikidata::toks(g.word(*w)));
            }
            t
        });
    }
    let overlap = |s: Cid, q: u32| -> usize {
        let Some(ot) = otoks.get(&s) else { return 0 };
        let wt = crate::wikidata::toks(&format!("{} {}", info.label.get(&q).map(|x| x.as_str()).unwrap_or(""), info.desc.get(&q).map(|x| x.as_str()).unwrap_or("")));
        wt.intersection(ot).count()
    };
    // P31 of candidates and taxon ranks (P105) — one pass over the claims
    let cl = crate::wdwalk::scan_claims(dir, &|q| qset.contains(&q), &[], &|_| false)?;
    let rk = crate::wdwalk::rank_kinds(&isa, &cl.rank);
    let wd = Wd { g: isa, p31: cl.p31, rk, folk };
    let mut cands: Vec<(Cid, u32, u8)> = cmap.iter().map(|(&(s, q), &m)| (s, q, m)).collect();
    cands.sort_unstable();
    let rows = decide(&o, &wd, &wiki, &overlap, &cands, min_src);
    let t_decide = t0.elapsed().as_secs_f64();
    // negative control
    let sh = shuffled(&cands, seed);
    let rows_sh = decide(&o, &wd, &wiki, &overlap, &sh, min_src);
    // silver: without direct ili/wn — does the lex/label choice match the direct links
    let direct: HashMap<Cid, HashSet<u32>> = cands.iter().filter(|c| c.2 & (S_ILI | S_WN) != 0).fold(HashMap::new(), |mut m, c| {
        m.entry(c.0).or_default().insert(c.1);
        m
    });
    let indirect: Vec<(Cid, u32, u8)> = cands.iter().filter(|c| c.2 & (S_LEX | S_LABEL) != 0).map(|&(s, q, m)| (s, q, m & (S_LEX | S_LABEL))).collect();
    // silver measures the choice by ancestor support, so one source support is enough here (as in v1)
    let rows_ind = decide(&o, &wd, &wiki, &overlap, &indirect, 1);
    let (mut silver_ok, mut silver_n) = (0usize, 0usize);
    let mut silver_bad: Vec<String> = Vec::new();
    for c in rows_ind.iter().filter(|c| c.status == Status::Accepted) {
        if let Some(d) = direct.get(&c.s) {
            silver_n += 1;
            if d.contains(&c.q) {
                silver_ok += 1;
            } else if silver_bad.len() < 15 {
                silver_bad.push(format!("{} → Q{} (direct: {})", g.show(c.s), c.q, d.iter().map(|q| format!("Q{q}")).collect::<Vec<_>>().join(",")));
            }
        }
    }
    // labels for the report and the sample
    let acc: Vec<&Cand> = rows.iter().filter(|c| c.status == Status::Accepted).collect();
    let mut rng = Rng::new(seed ^ 0x9e37_79b9_7f4a_7c15);
    let mut pick: Vec<usize> = (0..acc.len()).collect();
    for i in (1..pick.len()).rev() {
        let j = rng.below(i + 1);
        pick.swap(i, j);
    }
    pick.truncate(sample_n);
    pick.sort_unstable();
    let mut need: HashSet<u32> = HashSet::new();
    for &i in &pick {
        need.insert(acc[i].q);
        if let Some(sp) = acc[i].support {
            need.insert(sp.1);
        }
    }
    let key_lemmas = ["apple", "fruit", "bird", "crow", "whale", "mammal", "fish", "lion", "cheese", "dog", "cat", "fox", "tree", "oak", "penguin", "mouse"];
    for c in rows.iter().filter(|c| c.status == Status::Accepted) {
        if o.g.members(c.s).iter().any(|w| key_lemmas.contains(&o.g.word(*w))) {
            need.insert(c.q);
        }
    }
    let texts: ItemInfo = read_items(dir, &|q| need.contains(&q), &|q| need.contains(&q))?;
    let lab = |q: u32| texts.label.get(&q).cloned().unwrap_or_default();
    let desc = |q: u32| texts.desc.get(&q).cloned().unwrap_or_default();
    // report
    let (a, al, n) = tally(&rows);
    let (a2, al2, n2) = tally(&rows_sh);
    let synsets_acc: HashSet<Cid> = rows.iter().filter(|c| c.status == Status::Accepted).map(|c| c.s).collect();
    let q_acc: HashSet<u32> = rows.iter().filter(|c| c.status == Status::Accepted).map(|c| c.q).collect();
    let mut s = String::from("# Alignment of OEWN synsets ↔ Wikidata items\n\n");
    let _ = writeln!(s, "Rule: a pair is accepted when the ancestors are consistently aligned too — there is a support pair (s′, q′) among the candidates, where s′ is a proper ancestor of s in OEWN (up to {OEWN_DEPTH} steps, with at most {} noun synsets beneath it — {:.0}% of {}), and q′ is a proper ancestor of q in Wikidata (P279/P171, P31 as the first step, up to {WD_DEPTH} steps{}).\n", o.generic_limit, generic_share * 100.0, o.n_nouns, if folk { "; P171 stops at the class rank — folk ancestors, `wdwalk`" } else { "; full cladistics" });
    let _ = writeln!(s, "Source supports required for acceptance: at least **{min_src}** (label, lexeme, direct link — ILI or P8814 count as one support). Candidates with ancestor support but a single source get status `single`.\n");
    let _ = writeln!(s, "- time: OEWN {t_oewn:.0} s, candidates {:.0} s, decision {:.0} s; total {:.0} s", t_gather - t_oewn, t_decide - t_gather, t0.elapsed().as_secs_f64());
    let mut cn: Vec<_> = counts.iter().collect();
    cn.sort();
    let _ = writeln!(s, "- source rows: {}", cn.iter().map(|(k, v)| format!("{k} {}", thousands(**v))).collect::<Vec<_>>().join(" · "));
    let single = rows.iter().filter(|c| c.status == Status::Single).count();
    let _ = writeln!(s, "- candidates (s, q) {}: accepted **{}** (best Q per synset) + also supported {}; ancestor support but a single source {}; unsupported {}", thousands(n as u64), thousands(a as u64), thousands(al as u64), thousands(single as u64), thousands((n - a - al - single) as u64));
    let _ = writeln!(s, "- synsets with an accepted Q: **{}** of {} nouns ({:.1}%); distinct Qs: {}\n", thousands(synsets_acc.len() as u64), thousands(o.n_nouns as u64), 100.0 * synsets_acc.len() as f64 / o.n_nouns.max(1) as f64, thousands(q_acc.len() as u64));
    let _ = writeln!(s, "| candidate source | candidates | accepted | supported (also) | supported share |\n|---|---:|---:|---:|---:|");
    let mut by_src: std::collections::BTreeMap<String, [usize; 3]> = Default::default();
    for c in &rows {
        let e = by_src.entry(src_names(c.src)).or_default();
        e[0] += 1;
        e[1] += (c.status == Status::Accepted) as usize;
        e[2] += (c.status == Status::Also) as usize;
    }
    for (k, v) in &by_src {
        let _ = writeln!(s, "| {k} | {} | {} | {} | {:.0}% |", thousands(v[0] as u64), thousands(v[1] as u64), thousands(v[2] as u64), 100.0 * (v[1] + v[2]) as f64 / v[0].max(1) as f64);
    }
    let rate = |a: usize, al: usize, n: usize| 100.0 * (a + al) as f64 / n.max(1) as f64;
    let control_ok = a2 * 10 <= a;
    let _ = writeln!(s, "\n## Negative control: shuffled labels\n\nQ shuffled among the candidates of each source (labels ↔ items), supports also from the shuffled ones. Real: accepted {} ({:.1}% of candidates supported). Shuffled: accepted **{}** ({:.1}% supported). Drop by {:.0}×{}. Control: **{}**.\n",
        thousands(a as u64), rate(a, al, n), thousands(a2 as u64), rate(a2, al2, n2), a as f64 / a2.max(1) as f64, if a2 == 0 { " (to zero)" } else { "" }, if control_ok { "yes — alignment drops" } else { "NO" });
    let _ = writeln!(s, "## Silver: direct links (ili, wn) versus the choice from lex and label\n\nFor synsets where Wikidata has a direct P5063/P8814 link, the choice from lex and label alone matched the direct one in **{silver_ok} of {silver_n}** ({:.1}%).\n", 100.0 * silver_ok as f64 / silver_n.max(1) as f64);
    for b in &silver_bad {
        let _ = writeln!(s, "- {b}");
    }
    // gold: manually checked pairs (v1 and v2 samples of 50 each)
    if let Ok(text) = std::fs::read_to_string(gold) {
        let st: HashMap<(Cid, u32), Status> = rows.iter().map(|c| ((c.s, c.q), c.status)).collect();
        let (mut n_g, mut acc_ok, mut acc_bad, mut lost_ok, mut rej_bad) = (0, 0, 0, 0, 0);
        let mut bad_kept: Vec<String> = Vec::new();
        for l in text.lines().filter(|l| !l.starts_with('#')) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() < 5 {
                continue;
            }
            let (Some(sy), Ok(q)) = (o.find(f[1]), f[2].parse::<u32>()) else { continue };
            n_g += 1;
            let ok = f[4] == "+";
            let accepted = st.get(&(sy, q)) == Some(&Status::Accepted);
            match (accepted, ok) {
                (true, true) => acc_ok += 1,
                (true, false) => {
                    acc_bad += 1;
                    bad_kept.push(format!("{} ↔ Q{q} ({})", g.show(sy), f.get(5).unwrap_or(&"")));
                }
                (false, true) => lost_ok += 1,
                (false, false) => rej_bad += 1,
            }
        }
        let _ = writeln!(s, "## Gold: manually checked pairs (`data/align-gold.tsv`)\n\nNow accepted {} of {n_g}: correct **{acc_ok}**, wrong **{acc_bad}** — precision {:.0}%. Correct ones that dropped out: {lost_ok}; wrong ones rejected: {rej_bad} of {}.\n", acc_ok + acc_bad, 100.0 * acc_ok as f64 / (acc_ok + acc_bad).max(1) as f64, rej_bad + acc_bad);
        for b in &bad_kept {
            let _ = writeln!(s, "- wrong but accepted: {b}");
        }
    }
    let _ = writeln!(s, "\n## Key words\n\n| synset | Q | label | description | sources | support |\n|---|---|---|---|---|---|");
    let mut keyrows: Vec<&Cand> = rows.iter().filter(|c| c.status == Status::Accepted && o.g.members(c.s).iter().any(|w| key_lemmas.contains(&o.g.word(*w)))).collect();
    keyrows.sort_by_key(|c| (o.g.label(c.s), c.s));
    for c in keyrows.iter().take(60) {
        let sp = c.support.map(|(sa, qa, ds, dq)| format!("{} ↔ Q{qa} ({ds}+{dq})", o.g.label(sa))).unwrap_or_default();
        let _ = writeln!(s, "| {} | Q{} | {} | {} | {} | {} |", g.show(c.s), c.q, lab(c.q), desc(c.q).chars().take(60).collect::<String>(), src_names(c.src), sp);
    }
    let _ = writeln!(s, "\n## Sample for manual checking ({} of {} accepted, seed {seed})\n\nThe \"verdict\" column is the manual check (filled in `align-sample.tsv`).\n", pick.len(), thousands(a as u64));
    let mut tsv = String::from("# n\tsynset\tlemmas\toewn_gloss\tQ\twd_label\twd_desc\tsources\tsupport\tverdict\n");
    for (k, &i) in pick.iter().enumerate() {
        let c = acc[i];
        let lem: Vec<&str> = o.g.members(c.s).iter().map(|w| o.g.word(*w)).collect();
        let sp = c.support.map(|(sa, qa, ds, dq)| format!("{} ↔ Q{qa} «{}» ({ds}+{dq})", o.g.label(sa), lab(qa))).unwrap_or_default();
        let _ = writeln!(s, "{}. {} [{}] «{}» ↔ Q{} «{}» — {} · {} · support {}", k + 1, o.g.cid_str(c.s), lem.join(", "), o.g.gloss_of(c.s).chars().take(90).collect::<String>(), c.q, lab(c.q), desc(c.q).chars().take(80).collect::<String>(), src_names(c.src), sp);
        let _ = writeln!(tsv, "{}\t{}\t{}\t{}\tQ{}\t{}\t{}\t{}\t{}\t", k + 1, o.g.cid_str(c.s), lem.join(", "), o.g.gloss_of(c.s).replace('\t', " "), c.q, lab(c.q), desc(c.q), src_names(c.src), sp);
    }
    std::fs::write(runs.join("align-sample.tsv"), tsv)?;
    std::fs::write(runs.join("align.md"), &s)?;
    Ok(AlignRun { report: s, rows })
}

/// Alignment table for the database: synset, q, status, sources, support.
pub fn write_rows(g: &Graph, rows: &[Cand], path: &Path) -> Result<usize> {
    use std::fmt::Write as _;
    let mut s = String::new();
    let mut n = 0;
    for c in rows {
        let (sa, qa, ds, dq) = c.support.map(|(a, b, x, y)| (g.cid_str(a), b.to_string(), x.to_string(), y.to_string())).unwrap_or_default();
        let _ = writeln!(s, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", g.cid_str(c.s), c.q, c.status.code(), src_names(c.src), sa, qa, ds, dq);
        n += 1;
    }
    std::fs::write(path, s)?;
    Ok(n)
}

// P279/P171 here only document the edge direction in `isa_edges`
const _: (u16, u16) = (P279, P171);

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn rng_and_shuffle() {
        let c: Vec<(Cid, u32, u8)> = (0..100).map(|i| (i, 1000 + i, S_LABEL)).collect();
        let s = shuffled(&c, 7);
        let moved = s.iter().zip(&c).filter(|(a, b)| a.1 != b.1).count();
        assert!(moved > 80, "shuffled {moved}");
        let mut qs: Vec<u32> = s.iter().map(|x| x.1).collect();
        qs.sort();
        assert_eq!(qs, (1000..1100).collect::<Vec<_>>(), "the same set of Qs");
        assert_eq!(src_names(S_LEX | S_LABEL), "lex+label");
    }
}
