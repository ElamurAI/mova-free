//! v3: word sense selection by context (WSD v1) — deterministic and transparent: every choice comes with a reason.
//!
//! Sense score = sum of four terms, each visible in the explanation:
//! 1. **Lesk** (extended, Banerjee & Pedersen 2003): context words shared with the sense signature — the gloss and
//!    lemmas of the synset itself (weight 1) and of neighbours: hypernyms up to 2 steps, hyponyms, parts, whole, substances (weight ½).
//!    Each shared word is weighted by IDF over all OEWN glosses.
//! 2. **Predominant sense from a corpus** (McCarthy et al. 2004, ACL P04-1036): distributional neighbours of the word from the
//!    Leipzig corpus (sentence co-occurrence, significance) vote for senses close to them in the graph (Wu — Palmer);
//!    the vote share is the prior weight of the sense. OEWN order plays no part here.
//! 3. **Core:** the sense is a concept of the compiled core (dolphin — mammal, oak — tree).
//! 4. **Domain:** share of context words whose senses lie in this sense's lexfile (animals, food, artifacts…).
//!
//! Ties — the first OEWN sense (as the baseline).

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result};

use crate::kb::*;
use crate::model::*;

pub const W_LESK: f64 = 1.0;
pub const W_PRIOR: f64 = 1.5;
pub const W_CORE: f64 = 0.5;
pub const W_DOMAIN: f64 = 1.0;

const STOP: &[&str] = &[
    "a", "an", "the", "and", "or", "of", "to", "in", "on", "at", "by", "for", "with", "from", "as", "is", "are", "was", "were", "be", "been", "it", "its", "this", "that", "these", "those", "his", "her", "their", "they", "he", "she", "we", "you", "i", "who", "which", "what", "not",
    "no", "but", "so", "into", "onto", "one", "some", "any", "all", "has", "have", "had", "do", "does", "did", "can", "may", "used", "use", "usually", "especially", "etc", "e", "g", "such", "other", "various", "several", "very", "more", "most", "than", "then", "there", "also", "while", "after", "before",
    "about", "over", "under", "up", "out", "down", "off", "our", "your", "my", "him", "them", "us", "if", "when", "where", "how", "both", "each", "every", "many", "much", "made", "make", "kind", "sort", "type", "part", "something", "someone", "thing", "things",
];

/// Word normalization: lowercase, simple stem (plural, -ing, -ed).
pub fn stem(w: &str) -> String {
    let w = w.to_lowercase();
    let w = w.trim_matches(|c: char| !c.is_alphanumeric()).trim_end_matches("'s").to_string();
    let n = w.chars().count();
    if n > 4 {
        if let Some(s) = w.strip_suffix("ies") {
            return format!("{s}y");
        }
        for suf in ["sses", "ches", "shes", "xes"] {
            if w.ends_with(suf) {
                return w[..w.len() - 2].to_string();
            }
        }
        if let Some(s) = w.strip_suffix("ing").filter(|s| s.len() >= 3) {
            return s.to_string();
        }
        if let Some(s) = w.strip_suffix("ed").filter(|s| s.len() >= 3) {
            return s.to_string();
        }
    }
    if n > 3 && w.ends_with('s') && !w.ends_with("ss") && !w.ends_with("us") {
        return w[..w.len() - 1].to_string();
    }
    w
}

/// Content words of a text (stems), without stop words.
pub fn content(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\'' || c == '-'))
        .flat_map(|t| t.split('-'))
        .filter(|t| t.chars().any(|c| c.is_alphabetic()))
        .map(|t| t.to_lowercase())
        .filter(|t| !STOP.contains(&t.as_str()) && t.len() > 1)
        .map(|t| stem(&t))
        .filter(|t| !STOP.contains(&t.as_str()))
        .collect()
}

/// IDF of stems over OEWN glosses and lemmas.
pub struct Idf {
    df: HashMap<String, u32>,
    n: f64,
}

impl Idf {
    pub fn new(g: &Graph) -> Idf {
        let mut df: HashMap<String, u32> = HashMap::new();
        let mut n = 0.0;
        for c in 0..g.n_concepts() as Cid {
            if g.src[c as usize] != Src::Oewn {
                continue;
            }
            n += 1.0;
            let mut seen: HashSet<String> = content(g.gloss_of(c)).into_iter().collect();
            for w in g.members(c) {
                seen.extend(content(g.word(*w)));
            }
            for s in seen {
                *df.entry(s).or_default() += 1;
            }
        }
        Idf { df, n }
    }
    pub fn w(&self, s: &str) -> f64 {
        (self.n / (1.0 + *self.df.get(s).unwrap_or(&0) as f64)).ln().max(0.0)
    }
}

/// Sense signature: stem → weight (1 — the synset itself, ½ — neighbours).
pub fn signature(kb: &Kb, c: Cid) -> HashMap<String, f64> {
    let g = &kb.g;
    let mut sig: HashMap<String, f64> = HashMap::new();
    let mut add = |x: Cid, w: f64, sig: &mut HashMap<String, f64>| {
        let mut words = content(g.gloss_of(x));
        for m in g.members(x) {
            words.extend(content(g.word(*m)));
        }
        for s in words {
            let e = sig.entry(s).or_insert(0.0);
            if *e < w {
                *e = w;
            }
        }
    };
    add(c, 1.0, &mut sig);
    // hypernyms up to 2 steps
    let mut frontier = vec![c];
    for _ in 0..2 {
        let mut next = Vec::new();
        for x in frontier {
            for st in kb.up(x) {
                add(st.to, 0.5, &mut sig);
                next.push(st.to);
            }
        }
        frontier = next;
    }
    // hyponyms, parts, whole, members, substances — one step
    for e in g.in_edges(c).iter().filter(|e| matches!(e.rel, Rel::IsA | Rel::InstanceOf | Rel::PartOf | Rel::MemberOf)).take(40) {
        add(e.to, 0.5, &mut sig);
    }
    for e in g.out_edges(c).iter().filter(|e| matches!(e.rel, Rel::PartOf | Rel::MemberOf | Rel::MadeOf)) {
        add(e.to, 0.5, &mut sig);
    }
    sig
}

/// Prior sense weights (McCarthy 2004) from `sense-prior.tsv`: synset → share; word → neighbours.
#[derive(Default)]
pub struct Prior {
    pub share: HashMap<String, f64>,
    pub neighbors: HashMap<String, String>,
}

impl Prior {
    pub fn load(path: &Path) -> Result<Prior> {
        let mut p = Prior::default();
        if !path.exists() {
            return Ok(p);
        }
        for l in std::fs::read_to_string(path)?.lines().filter(|l| !l.starts_with('#')) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() >= 4 {
                p.share.insert(f[1].to_string(), f[2].parse().unwrap_or(0.0));
                p.neighbors.entry(f[0].to_string()).or_insert_with(|| f[3].to_string());
            }
        }
        Ok(p)
    }
}

pub fn prior_path() -> std::path::PathBuf {
    std::env::var("ONTO_PRIOR").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("onto/sense-prior.tsv"))
}

/// Which terms to enable (for measurements and controls).
#[derive(Clone, Copy, Debug)]
pub struct Mix {
    pub lesk: bool,
    pub prior: bool,
    pub core: bool,
    pub domain: bool,
}

impl Mix {
    pub const ALL: Mix = Mix { lesk: true, prior: true, core: true, domain: true };
}

#[derive(Clone, Debug)]
pub struct Scored {
    pub sense: Cid,
    pub total: f64,
    pub lesk: f64,
    pub prior: f64,
    pub core: f64,
    pub domain: f64,
    pub overlap: Vec<String>,
}

pub struct Choice {
    pub word: String,
    pub scored: Vec<Scored>,
    pub best: usize,
}

impl Choice {
    pub fn sense(&self) -> Cid {
        self.scored[self.best].sense
    }
    /// Transparent explanation of the choice.
    pub fn explain(&self, g: &Graph, prior: &Prior) -> String {
        let mut s = String::new();
        let b = &self.scored[self.best];
        let _ = writeln!(s, "choice: {} → {} (score {:.2}) — \"{}\"", self.word, g.show(b.sense), b.total, g.gloss_of(b.sense).chars().take(90).collect::<String>());
        for (i, x) in self.scored.iter().enumerate() {
            let _ = writeln!(
                s,
                "  {}{}. {} {}: Lesk {:.2}{} + corpus {:.2} + core {:.2} + domain {:.2} = {:.2}",
                if i == self.best { "▶ " } else { "  " },
                i + 1,
                g.show(x.sense),
                g.lexfile_name(x.sense),
                x.lesk,
                if x.overlap.is_empty() { String::new() } else { format!(" [{}]", x.overlap.join(", ")) },
                x.prior,
                x.core,
                x.domain,
                x.total
            );
        }
        if let Some(n) = prior.neighbors.get(&self.word.to_lowercase()) {
            let _ = writeln!(s, "  corpus neighbours (McCarthy 2004): {n}");
        }
        s
    }
}

/// Share of context words by lexfile (each word splits its vote among its noun senses).
pub fn domain_votes(g: &Graph, ctx: &[String]) -> HashMap<u8, f64> {
    let mut v: HashMap<u8, f64> = HashMap::new();
    let mut total = 0.0;
    for w in ctx {
        if let Some(f) = crate::query::lookup(g, w, Pos::Noun) {
            // second round (NOT blind, 26.09): vote by sense rank 1, ½, ⅓ — the first OEWN sense is more frequent;
            // an equal vote over six senses gave "cat" five artifacts (CAT scan, Caterpillar…) against one animal
            let ws: Vec<f64> = (0..f.senses.len().min(3)).map(|i| 1.0 / (i + 1) as f64).collect();
            let z: f64 = ws.iter().sum();
            for (i, &c) in f.senses.iter().take(3).enumerate() {
                *v.entry(g.lexfile[c as usize]).or_default() += ws[i] / z;
            }
            total += 1.0;
        }
    }
    if total > 0.0 {
        for x in v.values_mut() {
            *x /= total;
        }
    }
    v
}

pub struct Wsd<'a> {
    pub kb: &'a Kb,
    pub idf: Idf,
    pub prior: Prior,
    pub mix: Mix,
    /// NEGATIVE CONTROL: the signature of sense c is taken from synset perm[c] — shuffled glosses (together with lemmas and
    /// neighbours, i.e. the whole synset description)
    pub perm: Option<Vec<Cid>>,
}

impl<'a> Wsd<'a> {
    pub fn new(kb: &'a Kb, prior: Prior) -> Wsd<'a> {
        Wsd { kb, idf: Idf::new(&kb.g), prior, mix: Mix::ALL, perm: None }
    }

    /// Choose the sense of noun `word` in sentence `sentence`.
    pub fn choose(&self, word: &str, sentence: &str) -> Option<Choice> {
        let g = &self.kb.g;
        let f = crate::query::lookup(g, word, Pos::Noun)?;
        let target = stem(word);
        let ctx: Vec<String> = content(sentence).into_iter().filter(|w| *w != target && *w != word.to_lowercase()).collect();
        let ctx_set: HashSet<&String> = ctx.iter().collect();
        let dom = if self.mix.domain { domain_votes(g, &ctx) } else { HashMap::new() };
        let mut scored = Vec::new();
        for &c in &f.senses {
            let (mut lesk, mut overlap) = (0.0, Vec::new());
            if self.mix.lesk {
                let sig = signature(self.kb, self.perm.as_ref().map(|p| p[c as usize]).unwrap_or(c));
                let mut hits: Vec<(&String, f64)> = ctx_set.iter().filter_map(|w| sig.get(*w).map(|wt| (*w, wt * self.idf.w(w)))).collect();
                hits.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(b.0)));
                lesk = hits.iter().map(|h| h.1).sum::<f64>() / 10.0;
                overlap = hits.iter().map(|h| h.0.clone()).collect();
            }
            let prior = if self.mix.prior { W_PRIOR * self.prior.share.get(&g.cid_str(c)).copied().unwrap_or(0.0) } else { 0.0 };
            let core = if self.mix.core && crate::core::of_concept(g, c).is_some() { W_CORE } else { 0.0 };
            let domain = W_DOMAIN * dom.get(&g.lexfile[c as usize]).copied().unwrap_or(0.0);
            let lesk = W_LESK * lesk;
            scored.push(Scored { sense: c, total: lesk + prior + core + domain, lesk, prior, core, domain, overlap });
        }
        // highest score; ties — earlier in OEWN order
        let best = (0..scored.len()).fold(0, |b, i| if scored[i].total > scored[b].total + 1e-9 { i } else { b });
        Some(Choice { word: word.to_string(), scored, best })
    }
}

// ── predominant sense from a corpus (McCarthy et al. 2004) ─────────────────────────────────────────────

/// Concept depth from the IsA root (root = 1), shortest.
fn root_depth(kb: &Kb, c: Cid, memo: &mut HashMap<Cid, u32>) -> u32 {
    if let Some(&d) = memo.get(&c) {
        return d;
    }
    memo.insert(c, 1); // guard against cycles (the gate rejects them anyway)
    let ups = kb.up(c);
    let d = if ups.is_empty() { 1 } else { 1 + ups.iter().map(|s| root_depth(kb, s.to, memo)).min().unwrap_or(0) };
    memo.insert(c, d);
    d
}

/// Wu — Palmer similarity: 2·depth(common ancestor) / (depth a + depth b).
pub fn wup(kb: &Kb, a: Cid, b: Cid, memo: &mut HashMap<Cid, u32>, anc: &mut HashMap<Cid, Anc>) -> f64 {
    let aa = anc.entry(a).or_insert_with(|| kb.ancestors(a)).map.keys().copied().collect::<Vec<_>>();
    let bb = anc.entry(b).or_insert_with(|| kb.ancestors(b));
    let common: Vec<Cid> = aa.into_iter().filter(|x| bb.contains(*x)).collect();
    let Some(l) = common.iter().map(|&x| (root_depth(kb, x, memo), x)).max() else { return 0.0 };
    let (da, db) = (root_depth(kb, a, memo), root_depth(kb, b, memo));
    2.0 * l.0 as f64 / (da + db) as f64
}

fn tar_member(archive: &Path, member: &str) -> Result<String> {
    let out = std::process::Command::new("tar").args(["xzf"]).arg(archive).args(["-O", member]).output().context("tar")?;
    anyhow::ensure!(out.status.success(), "tar {}: {}", archive.display(), String::from_utf8_lossy(&out.stderr));
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Predominant senses for `targets` from the Leipzig corpora (`<name>-words.txt`, `<name>-co_s.txt` in the archive).
pub fn build_prior(kb: &Kb, corpora: &[(std::path::PathBuf, String)], targets: &[String], k: usize, out: &Path) -> Result<String> {
    let g = &kb.g;
    let t0 = std::time::Instant::now();
    // vocabulary: string (lowercase) → number
    let mut ids: HashMap<String, u32> = HashMap::new();
    let mut names: Vec<String> = Vec::new();
    let mut freq: Vec<u64> = Vec::new();
    let mut vecs: Vec<HashMap<u32, f32>> = Vec::new();
    for (arch, name) in corpora {
        let words = tar_member(arch, &format!("{name}/{name}-words.txt"))?;
        let mut local: HashMap<u32, u32> = HashMap::new();
        for l in words.lines() {
            let mut f = l.split('\t');
            let (Some(id), Some(w), Some(n)) = (f.next(), f.next(), f.next()) else { continue };
            if !w.chars().all(|c| c.is_alphabetic()) || w.len() < 2 {
                continue;
            }
            let lw = w.to_lowercase();
            let gid = *ids.entry(lw.clone()).or_insert_with(|| {
                names.push(lw.clone());
                freq.push(0);
                vecs.push(HashMap::new());
                (names.len() - 1) as u32
            });
            freq[gid as usize] += n.parse::<u64>().unwrap_or(0);
            if let Ok(id) = id.parse::<u32>() {
                local.insert(id, gid);
            }
        }
        let co = tar_member(arch, &format!("{name}/{name}-co_s.txt"))?;
        for l in co.lines() {
            let mut f = l.split('\t');
            let (Some(a), Some(b), Some(_), Some(sig)) = (f.next(), f.next(), f.next(), f.next()) else { continue };
            let (Some(&a), Some(&b)) = (a.parse::<u32>().ok().and_then(|x| local.get(&x)), b.parse::<u32>().ok().and_then(|x| local.get(&x))) else { continue };
            if a == b || STOP.contains(&names[a as usize].as_str()) || STOP.contains(&names[b as usize].as_str()) {
                continue;
            }
            let s: f32 = sig.parse().unwrap_or(0.0);
            *vecs[a as usize].entry(b).or_default() += s;
            *vecs[b as usize].entry(a).or_default() += s;
        }
    }
    // trim to the 200 most significant contexts and normalize
    let mut norm: Vec<Vec<(u32, f32)>> = vecs
        .into_iter()
        .map(|m| {
            let mut v: Vec<(u32, f32)> = m.into_iter().collect();
            v.sort_by(|a, b| b.1.total_cmp(&a.1));
            v.truncate(200);
            let n = v.iter().map(|x| x.1 * x.1).sum::<f32>().sqrt().max(1e-9);
            v.iter_mut().for_each(|x| x.1 /= n);
            v
        })
        .collect();
    // neighbour candidates: OEWN nouns with frequency ≥ 50
    let is_noun = |w: &str| crate::query::lookup(g, w, Pos::Noun).is_some();
    let cand: Vec<bool> = names.iter().zip(&freq).map(|(w, &f)| f >= 50 && is_noun(w)).collect();
    let mut inv: HashMap<u32, Vec<(u32, f32)>> = HashMap::new();
    for (w, v) in norm.iter().enumerate() {
        if cand[w] {
            for &(c, x) in v {
                inv.entry(c).or_default().push((w as u32, x));
            }
        }
    }
    let mut memo = HashMap::new();
    let mut anc = HashMap::new();
    let mut s = String::from("# onto v3: predominant senses (McCarthy et al. 2004) from the Leipzig corpora — vote share of distributional neighbours per sense (Wu — Palmer⁸ on OEWN IsA).\n# word\tsynset\tshare\tneighbors (dss)\n");
    let (mut done, mut missing) = (0, 0);
    for t in targets {
        let tl = t.to_lowercase().replace('_', " ");
        let Some(f) = crate::query::lookup(g, &tl, Pos::Noun) else { continue };
        let senses: Vec<Cid> = f.senses.clone();
        if senses.len() < 2 {
            continue;
        }
        let Some(&tid) = ids.get(&tl) else {
            missing += 1;
            continue;
        };
        let mut acc: HashMap<u32, f32> = HashMap::new();
        for &(c, x) in &norm[tid as usize] {
            if let Some(ps) = inv.get(&c) {
                for &(w, y) in ps {
                    if w != tid {
                        *acc.entry(w).or_default() += x * y;
                    }
                }
            }
        }
        let mut nb: Vec<(u32, f32)> = acc.into_iter().collect();
        nb.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        nb.truncate(k);
        let mut prev = vec![0.0f64; senses.len()];
        for &(n, dss) in &nb {
            let Some(fnb) = crate::query::lookup(g, &names[n as usize], Pos::Noun) else { continue };
            // a word form of the target itself (dolphins for dolphin) is not a neighbour
            if stem(&names[n as usize]) == stem(&tl) {
                continue;
            }
            // Wu — Palmer to the 8th power: sharper similarity (McCarthy 2004 used jcn — also sharp)
            let sims: Vec<f64> = senses.iter().map(|&si| fnb.senses.iter().take(4).map(|&nj| wup(kb, si, nj, &mut memo, &mut anc)).fold(0.0, f64::max).powi(8)).collect();
            let z: f64 = sims.iter().sum();
            if z > 0.0 {
                for (i, x) in sims.iter().enumerate() {
                    prev[i] += dss as f64 * x / z;
                }
            }
        }
        let z: f64 = prev.iter().sum::<f64>().max(1e-12);
        let nbs: Vec<String> = nb.iter().take(8).map(|(n, d)| format!("{} {:.2}", names[*n as usize], d)).collect();
        for (i, &c) in senses.iter().enumerate() {
            let _ = writeln!(s, "{tl}\t{}\t{:.4}\t{}", g.cid_str(c), prev[i] / z, nbs.join(", "));
        }
        done += 1;
        norm[tid as usize].shrink_to_fit();
    }
    if let Some(p) = out.parent() {
        std::fs::create_dir_all(p)?;
    }
    std::fs::write(out, &s)?;
    Ok(format!("predominant senses: {done} polysemous words (missing from the corpus — {missing}), vocabulary {} words, {:.0} s → {}", names.len(), t0.elapsed().as_secs_f64(), out.display()))
}

/// Graph with shuffled glosses (Lesk negative control): glosses permuted among synsets.
pub fn shuffled_glosses(g: &Graph, seed: u64) -> Graph {
    let n = g.n_concepts();
    let mut order: Vec<usize> = (0..n).collect();
    let mut x = seed | 1;
    for i in (1..n).rev() {
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        let j = (x.wrapping_mul(0x2545_f491_4f6c_dd1d) % (i as u64 + 1)) as usize;
        order.swap(i, j);
    }
    let mut gloss = String::new();
    let mut off = vec![0u32];
    for &i in &order {
        gloss.push_str(g.gloss_of(i as Cid));
        off.push(gloss.len() as u32);
    }
    Graph {
        src: g.src.clone(),
        key: g.key.clone(),
        pos: g.pos.clone(),
        lexfile: g.lexfile.clone(),
        flags: g.flags.clone(),
        ili: g.ili.clone(),
        gloss_off: off,
        gloss,
        name_off: g.name_off.clone(),
        names: g.names.clone(),
        mem_off: g.mem_off.clone(),
        mem: g.mem.clone(),
        word_off: g.word_off.clone(),
        words: g.words.clone(),
        sense_off: g.sense_off.clone(),
        senses: g.senses.clone(),
        out_off: g.out_off.clone(),
        out: g.out.clone(),
        in_off: g.in_off.clone(),
        inn: g.inn.clone(),
        lus: g.lus.clone(),
    }
}

/// Permutation of OEWN synsets among themselves (for control): concept → another concept from the same source.
pub fn synset_perm(g: &Graph, seed: u64) -> Vec<Cid> {
    let ids: Vec<Cid> = (0..g.n_concepts() as Cid).filter(|&c| g.src[c as usize] == Src::Oewn).collect();
    let mut sh = ids.clone();
    let mut x = seed | 1;
    for i in (1..sh.len()).rev() {
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        let j = (x.wrapping_mul(0x2545_f491_4f6c_dd1d) % (i as u64 + 1)) as usize;
        sh.swap(i, j);
    }
    let mut p: Vec<Cid> = (0..g.n_concepts() as Cid).collect();
    for (a, b) in ids.iter().zip(sh) {
        p[*a as usize] = b;
    }
    p
}

/// Prior weights for queries without a sentence (`isa`, `prop`): loaded once per process from ONTO_PRIOR.
pub fn global_prior() -> &'static Prior {
    static P: std::sync::OnceLock<Prior> = std::sync::OnceLock::new();
    P.get_or_init(|| Prior::load(&prior_path()).unwrap_or_default())
}
