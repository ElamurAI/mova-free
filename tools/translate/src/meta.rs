//! The meta-world and the readers into it.
//!
//! The meta-world of a verse is a graph of meta tokens: a concept (a word or a construction shared by the authors),
//! the coarse grammar the standard parse gives it (PTB tag), its head and relation. What is one author's own is
//! not in it: the spelling of a concept (thee, thou, ye and you are one concept), the construction an author prefers
//! (it came to pass / it happened), the capital of a sentence start. Each concept is read with the words it came from
//! (`span`), so an author's profile can learn how that author writes it.
//!
//! The concepts are learned once for all authors (the interlingua lexicon, `learn`): on aligned training verses
//! of pairs of authors, the places where two authors differ — the gaps between the longest common subsequence of
//! their reading keys — give word and phrase pairs; two words or phrases that are each other's most frequent partner
//! there, often enough (`LexParams`), join one concept. A reader is this lexicon read forwards: the longest phrase
//! first, then the word. The same reader serves every English author, so a verse read from two authors lands on the
//! same meta-world as far as the lexicon reaches.

use std::collections::{BTreeMap, BTreeSet};

use en::annotate::Word;
use en::gram::{Rel, Tag};
use roundtrip::tree::TNode;

use crate::bible::VKey;
use crate::corpus::Corpus;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MTok {
    pub concept: String,
    pub tag: Tag,
    /// 1-based, 0 — root
    pub head: usize,
    pub rel: Rel,
    /// the source tokens it was read from, [start, end)
    pub span: (usize, usize),
}

pub fn is_punct(w: &Word) -> bool {
    w.rel == Rel::Punct || roundtrip::class::is_punct_tag(w.tag)
}

/// The reading key of a word: the lemma in lower case; a punctuation mark as written; the divine names written in
/// capitals (LORD, GOD: the tetragrammaton, not «lord») in capitals.
pub fn rkey(w: &Word) -> String {
    if is_punct(w) {
        return w.form.clone();
    }
    let caps = w.form.chars().count() > 1 && w.form.chars().all(|c| !c.is_lowercase()) && w.form.chars().any(char::is_uppercase);
    if caps && (w.tag.is_proper() || matches!(w.lemma.to_lowercase().as_str(), "lord" | "god")) {
        return w.lemma.to_uppercase();
    }
    w.lemma.to_lowercase()
}

pub fn keys(ws: &[Word]) -> Vec<String> {
    ws.iter().map(rkey).collect()
}

/// The longest common subsequence of two key sequences, as index pairs.
pub fn lcs(a: &[String], b: &[String]) -> Vec<(usize, usize)> {
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![0u16; (n + 1) * (m + 1)];
    let at = |i: usize, j: usize| i * (m + 1) + j;
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[at(i, j)] = if a[i] == b[j] { dp[at(i + 1, j + 1)] + 1 } else { dp[at(i + 1, j)].max(dp[at(i, j + 1)]) };
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < n && j < m {
        if a[i] == b[j] {
            out.push((i, j));
            i += 1;
            j += 1;
        } else if dp[at(i + 1, j)] >= dp[at(i, j + 1)] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

/// The gaps between the anchors of an LCS: ([a0, a1), [b0, b1)) with something on at least one side.
pub fn gaps(n: usize, m: usize, anchors: &[(usize, usize)]) -> Vec<((usize, usize), (usize, usize))> {
    let mut out = Vec::new();
    let (mut pi, mut pj) = (0, 0);
    for &(i, j) in anchors.iter().chain(std::iter::once(&(n, m))) {
        if i > pi || j > pj {
            out.push(((pi, i), (pj, j)));
        }
        pi = i + 1;
        pj = j + 1;
    }
    out
}

#[derive(Clone, Copy, Debug)]
pub struct LexParams {
    /// a pair must be seen this often
    pub min_count: u32,
    /// and be this share of the word's (phrase's) places where the two authors differ
    pub share: f64,
    /// and this share of all its occurrences
    pub cover: f64,
    /// each must be the other's most frequent partner
    pub mutual: bool,
    /// how the run reads (the judge's measure reads the same way): pronoun features, coarse relations
    pub feats: bool,
    pub coarse: bool,
    pub fw: bool,
    pub clauses: bool,
    pub joined: bool,
    pub flat: bool,
    /// the per-pair judge measures each pair's saving on that pair's own samples
    pub own_samples: bool,
    /// the measured saving is its 95% lower bound, not its mean
    pub lb: bool,
    /// judge each merge per author pair: it must not cost any pair more than it saves that pair's reader
    pub per_pair: bool,
    /// a phrase pair whose two sides share a word is no candidate («LORD say» ~ «lord seide» is «say» ~ «seide»,
    /// which the single words give)
    pub distinct: bool,
    pub max_len: usize,
}

impl Default for LexParams {
    fn default() -> Self {
        LexParams { min_count: 5, share: 0.3, cover: 0.05, mutual: true, max_len: 4, feats: false, coarse: false, fw: false, clauses: false, joined: false, flat: false, own_samples: false, lb: false, per_pair: false, distinct: false }
    }
}

/// The interlingua lexicon: which words and phrases read into which concept, and why.
#[derive(Clone, Debug, Default)]
pub struct Lexicon {
    /// a word's key → its concept (only where they differ)
    pub single: BTreeMap<String, String>,
    /// a phrase's keys joined by spaces (two or more) → its concept
    pub phrases: BTreeMap<String, String>,
    pub max_len: usize,
    /// a concept → its members with their counts
    pub clusters: BTreeMap<String, Vec<(String, u32)>>,
    /// a member → why it joined (the pair that linked it)
    pub why: BTreeMap<String, String>,
    /// pronoun concepts carry number and person (`with_feats`)
    pub feats: bool,
    /// relations coarse as frame roles and punctuation on the word before it (`coarse`)
    pub coarse: bool,
    /// determiners and prepositions are features of the word they belong to (`absorb_function_words`)
    pub fw: bool,
    /// the clause level: joiners go with the next clause's first word, coordinated clauses and sentences stand side
    /// by side (`absorb_joiners`)
    pub clauses: bool,
    /// only the second half of that: coordinated clauses and sentences side by side, the joiners left as words
    pub flat: bool,
    /// the clause level with the joining kept as a feature (`#j:<relation>:<joiners>`), which a table per target
    /// author maps (`fwt::JoinTable`)
    pub joined: bool,
    /// the merges the logic gate refused (`learn_gated`)
    pub refused_logic: Vec<String>,
}

fn find(parent: &mut BTreeMap<String, String>, x: &str) -> String {
    let mut r = x.to_string();
    while let Some(p) = parent.get(&r) {
        if *p == r {
            break;
        }
        r = p.clone();
    }
    // path compression
    let mut c = x.to_string();
    while c != r {
        let p = parent.get(&c).cloned().unwrap_or(r.clone());
        parent.insert(c, r.clone());
        c = p;
    }
    r
}

fn ngrams(ks: &[String], max: usize, out: &mut BTreeMap<String, u32>) {
    for i in 0..ks.len() {
        for l in 1..=max.min(ks.len() - i) {
            *out.entry(ks[i..i + l].join(" ")).or_default() += 1;
        }
    }
}

/// A gap side worth a pair: one to `max` keys, not a word mixed with punctuation.
/// Modals: a word of their own in every meta-world, never part of a phrase («may Yahweh» is «may» and «Yahweh»).
pub const MODALS: [&str; 9] = ["may", "might", "shall", "will", "can", "could", "would", "should", "must"];

/// Two gap sides of which one is a phrase and they have a word in common (case aside).
fn shares_word(a: &[String], b: &[String]) -> bool {
    (a.len() > 1 || b.len() > 1) && a.iter().any(|x| b.iter().any(|y| x.to_lowercase() == y.to_lowercase()))
}

fn side_ok(ks: &[String], max: usize) -> bool {
    if ks.is_empty() || ks.len() > max {
        return false;
    }
    if ks.len() > 1 && ks.iter().any(|k| MODALS.contains(&k.as_str())) {
        return false;
    }
    let punct = |k: &String| !k.chars().any(char::is_alphanumeric);
    ks.iter().all(punct) || !ks.iter().any(punct)
}

/// Learn the lexicon from pairs of authors on the given verses. `shuffle`: a negative control — each verse of the
/// first author is paired with another verse of the second (a fixed rotation), so nothing true is left to learn.
///
/// `judge`: the residual-driven lexicon. The links above are only candidates; each is kept only if it lowers the
/// bits it moves: what it saves the reader (each differing place the merge makes equal, a lemma patch of 17 bits a
/// word) against what it costs the encoder (each occurrence, in all the judged corpora, whose member the context —
/// author, tag and relation; author alone for a phrase — no longer tells, 17 bits a word). Candidates go in order of
/// their counts; a cluster's cost is recomputed as it grows.
/// `measure`: calibrate the judge — what a merge saves the reader is measured, not assumed: on up to 12 sampled
/// training verse pairs where the two differ there, the reader's bits (`measure`) with the merge against without
/// it, times the merge's count.
pub fn learn(pairs: &[(&Corpus, &Corpus)], verses: &BTreeSet<VKey>, p: LexParams, shuffle: bool, judge: Option<&[&Corpus]>, measure: Option<&Measure>) -> Lexicon {
    learn_gated(pairs, verses, p, shuffle, judge, measure, None)
}

/// `learn` with the logic gate (`logic`, with `measure`): a merge is kept only if it does not raise the contradictions
/// of its sampled verse pairs (`logic_rise`); the refused merges are in `Lexicon::refused_logic`.
pub fn learn_gated(pairs: &[(&Corpus, &Corpus)], verses: &BTreeSet<VKey>, p: LexParams, shuffle: bool, judge: Option<&[&Corpus]>, measure: Option<&Measure>, logic: Option<&LogicFn>) -> Lexicon {
    let mut samples: Samples = BTreeMap::new();
    let mut links: Vec<(String, String, String)> = Vec::new();
    let mut tot: BTreeMap<String, u32> = BTreeMap::new();
    let mut gapc: BTreeMap<(String, String), u32> = BTreeMap::new();
    let mut gap_pp: Vec<BTreeMap<(String, String), u32>> = vec![BTreeMap::new(); pairs.len()];
    for (pi, (a, b)) in pairs.iter().enumerate() {
        let ks: Vec<VKey> = verses.iter().filter(|k| a.verses.contains_key(k) && b.verses.contains_key(k)).copied().collect();
        let n = ks.len();
        let (mut na, mut nb) = (BTreeMap::new(), BTreeMap::new());
        let mut c: BTreeMap<(String, String), u32> = BTreeMap::new();
        for (i, k) in ks.iter().enumerate() {
            let kb = if shuffle { ks[(i + 7919) % n.max(1)] } else { *k };
            let xa = keys(&a.verses[k].words);
            let xb = keys(&b.verses[&kb].words);
            ngrams(&xa, p.max_len, &mut na);
            ngrams(&xb, p.max_len, &mut nb);
            for ((a0, a1), (b0, b1)) in gaps(xa.len(), xb.len(), &lcs(&xa, &xb)) {
                if side_ok(&xa[a0..a1], p.max_len) && side_ok(&xb[b0..b1], p.max_len) && (!p.distinct || !shares_word(&xa[a0..a1], &xb[b0..b1])) {
                    let key = (xa[a0..a1].join(" "), xb[b0..b1].join(" "));
                    let sv = samples.entry(key.clone()).or_default();
                    if sv.len() < 12 {
                        sv.push((pi, *k, kb));
                    }
                    *c.entry(key).or_default() += 1;
                }
            }
        }
        for (x, v) in na.iter().chain(nb.iter()) {
            *tot.entry(x.clone()).or_default() += v;
        }
        for ((x, y), v) in &c {
            let k = if x < y { (x.clone(), y.clone()) } else { (y.clone(), x.clone()) };
            *gapc.entry(k.clone()).or_default() += v;
            *gap_pp[pi].entry(k).or_default() += v;
        }
        // the dominant partner of each side's word or phrase
        let mut best_a: BTreeMap<&str, (&str, u32)> = BTreeMap::new();
        let mut best_b: BTreeMap<&str, (&str, u32)> = BTreeMap::new();
        let (mut ga, mut gb): (BTreeMap<&str, u32>, BTreeMap<&str, u32>) = (BTreeMap::new(), BTreeMap::new());
        for ((x, y), &v) in &c {
            *ga.entry(x).or_default() += v;
            *gb.entry(y).or_default() += v;
            let e = best_a.entry(x).or_insert((y, 0));
            if v > e.1 {
                *e = (y, v);
            }
            let e = best_b.entry(y).or_insert((x, 0));
            if v > e.1 || (v == e.1 && x.as_str() < e.0) {
                *e = (x, v);
            }
        }
        let tag = format!("{}–{}", a.name, b.name);
        for (side, best, other, nn, gg) in [(&a.name, &best_a, &best_b, &na, &ga), (&b.name, &best_b, &best_a, &nb, &gb)] {
            for (x, (y, v)) in best.iter() {
                let occ = nn.get(*x).copied().unwrap_or(0).max(1);
                let g = gg.get(*x).copied().unwrap_or(0).max(1);
                // each must be the other's most frequent partner (no chains of one word drawing many)
                let mutual = !p.mutual || other.get(*y).is_some_and(|(z, _)| z == x);
                if mutual && *v >= p.min_count && (*v as f64) >= p.share * g as f64 && (*v as f64) >= p.cover * occ as f64 {
                    links.push((x.to_string(), y.to_string(), format!("«{x}» ({side}) stands for «{y}» in {v} of its {g} differing places, {occ} occurrences ({tag})")));
                }
            }
        }
    }
    let mut parent: BTreeMap<String, String> = BTreeMap::new();
    let mut lex = Lexicon { max_len: p.max_len, feats: p.feats, coarse: p.coarse, fw: p.fw, clauses: p.clauses, joined: p.joined, flat: p.flat, ..Default::default() };
    if let Some(cs) = judge {
        links = judged(&links, &gapc, cs, verses, p, measure.map(|m| (m, &samples, pairs)), (pairs, &gap_pp), logic, &mut lex.refused_logic);
    }
    for (x, y, why) in &links {
        parent.entry(x.clone()).or_insert(x.clone());
        parent.entry(y.clone()).or_insert(y.clone());
        let (rx, ry) = (find(&mut parent, x), find(&mut parent, y));
        if rx != ry {
            let (lo, hi) = if rx < ry { (rx, ry) } else { (ry, rx) };
            parent.insert(hi, lo);
        }
        lex.why.entry(x.clone()).or_insert(why.clone());
    }
    let members: Vec<String> = parent.keys().cloned().collect();
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for m in members {
        let r = find(&mut parent, &m);
        groups.entry(r).or_default().push(m);
    }
    for (_, ms) in groups {
        let canon = ms.iter().max_by(|x, y| tot.get(*x).cmp(&tot.get(*y)).then(y.cmp(x))).unwrap().clone();
        let concept = canon.replace(' ', "_");
        for m in &ms {
            if m.contains(' ') {
                lex.phrases.insert(m.clone(), concept.clone());
            } else if *m != concept {
                lex.single.insert(m.clone(), concept.clone());
            }
        }
        lex.clusters.insert(concept, ms.iter().map(|m| (m.clone(), tot.get(m).copied().unwrap_or(0))).collect());
    }
    lex
}

/// Occurrences of the candidate members in the judged corpora, by context: (corpus, tag and relation) for a word,
/// (corpus, "") for a phrase.
type Occ = BTreeMap<String, BTreeMap<(u8, String), u32>>;

fn occurrences(members: &BTreeSet<String>, cs: &[&Corpus], verses: &BTreeSet<VKey>, max_len: usize) -> Occ {
    let mut occ: Occ = BTreeMap::new();
    for (ci, c) in cs.iter().enumerate() {
        for (k, v) in &c.verses {
            if c.kind == crate::sources::Kind::Bible && !verses.contains(k) {
                continue;
            }
            if c.kind != crate::sources::Kind::Bible && c.kind.split(*k) != crate::bible::Split::Train {
                continue;
            }
            let ks = keys(&v.words);
            for i in 0..ks.len() {
                if members.contains(&ks[i]) {
                    *occ.entry(ks[i].clone()).or_default().entry((ci as u8, format!("{}|{}", v.words[i].tag, v.words[i].rel))).or_default() += 1;
                }
                for l in 2..=max_len.min(ks.len() - i) {
                    let ph = ks[i..i + l].join(" ");
                    if members.contains(&ph) {
                        *occ.entry(ph).or_default().entry((ci as u8, String::new())).or_default() += 1;
                    }
                }
            }
        }
    }
    occ
}

/// Occurrences of a cluster whose member its context does not tell (the context's minority members).
fn errors(ms: &[String], occ: &Occ) -> u64 {
    errors_in(ms, occ, None, false)
}

/// The same in some corpora only; `by_tag`: the context is the author and the tag (not the relation).
fn errors_in(ms: &[String], occ: &Occ, only: Option<&[u8]>, by_tag: bool) -> u64 {
    let phrase = ms.iter().any(|m| m.contains(' '));
    let mut by: BTreeMap<(u8, String), BTreeMap<&str, u32>> = BTreeMap::new();
    for m in ms {
        if let Some(o) = occ.get(m) {
            // only the given corpora's contexts: a range per corpus (the map is ordered by corpus first), which keeps
            // the per-pair judge cheap when there are many corpora
            let rows: Box<dyn Iterator<Item = (&(u8, String), &u32)>> = match only {
                Some(cs) => Box::new(cs.iter().flat_map(move |&c| o.range((c, String::new())..).take_while(move |((x, _), _)| *x == c))),
                None => Box::new(o.iter()),
            };
            for ((c, ctx), n) in rows {
                let ctx = if by_tag { ctx.split('|').next().unwrap_or("").to_string() } else { ctx.clone() };
                let key = if phrase { (*c, String::new()) } else { (*c, ctx) };
                *by.entry(key).or_default().entry(m.as_str()).or_default() += n;
            }
        }
    }
    by.values().map(|t| (t.values().sum::<u32>() - t.values().max().copied().unwrap_or(0)) as u64).sum()
}

fn words_of(m: &str) -> f64 {
    m.split(' ').count() as f64
}

/// The residual-driven choice among candidate links (see `learn`).
/// A word's anchors between two authors on the given verses: how often it is anchored to itself (the longest common
/// subsequence of reading keys), how often it stands in a place where they differ, and what the other writes there.
/// The test of false friends across centuries (postgate-1922): KJV's *suffer* (allow), *prevent* (go before), *meat*
/// (food), *quick* (alive), *conversation* (conduct), *let* (hinder) anchored to the same modern word is a false anchor.
pub fn anchor_stats(a: &Corpus, b: &Corpus, verses: &BTreeSet<VKey>, word: &str) -> (u32, u32, Vec<(String, u32)>) {
    let (mut same, mut differ) = (0u32, 0u32);
    let mut partners: BTreeMap<String, u32> = BTreeMap::new();
    for k in verses {
        let (Some(x), Some(y)) = (a.verses.get(k), b.verses.get(k)) else { continue };
        let (xa, xb) = (keys(&x.words), keys(&y.words));
        if !xa.iter().any(|w| w == word) {
            continue;
        }
        let an = lcs(&xa, &xb);
        same += an.iter().filter(|(i, _)| xa[*i] == word).count() as u32;
        for ((a0, a1), (b0, b1)) in gaps(xa.len(), xb.len(), &an) {
            let n = xa[a0..a1].iter().filter(|w| *w == word).count() as u32;
            if n > 0 {
                differ += n;
                *partners.entry(xb[b0..b1].join(" ")).or_default() += n;
            }
        }
    }
    let mut p: Vec<(String, u32)> = partners.into_iter().collect();
    p.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
    p.truncate(4);
    (same, differ, p)
}

/// The reader's bits for a pair of meta-worlds (`measure::reader_bits`).
pub type Measure = dyn Fn(&[MTok], &[MTok]) -> f64 + Sync;
/// Sampled places of each gap pair: (author pair, verse of the first, verse of the second).
type Samples = BTreeMap<(String, String), Vec<(usize, VKey, VKey)>>;

/// The logical distance between two readings of a verse through a lexicon: its contradictions (`logic::compare`).
pub type LogicFn = dyn Fn(&Lexicon, &crate::corpus::PVerse, &crate::corpus::PVerse) -> u64 + Sync;

/// What merging x and y does to the logic of the sampled verse pairs: the contradictions with the merge less those
/// without it. A merge that equates two words of different meaning makes facts that were apart look like the same
/// fact with different participants or polarity; that rise is what the logic gate refuses.
fn logic_rise(x: &str, y: &str, logic: &LogicFn, samples: &Samples, pairs: &[(&Corpus, &Corpus)]) -> i64 {
    if x.contains(' ') || y.contains(' ') {
        return 0;
    }
    let mut one = Lexicon::default();
    if x != y {
        one.single.insert(y.to_string(), x.to_string());
    }
    let none = Lexicon::default();
    let mut got: Vec<(usize, VKey, VKey)> = samples.get(&(x.to_string(), y.to_string())).cloned().unwrap_or_default();
    got.extend(samples.get(&(y.to_string(), x.to_string())).cloned().unwrap_or_default());
    got.iter()
        .map(|(pi, ka, kb)| {
            let (va, vb) = (&pairs[*pi].0.verses[ka], &pairs[*pi].1.verses[kb]);
            logic(&one, va, vb) as i64 - logic(&none, va, vb) as i64
        })
        .sum()
}

/// What merging x and y saves the reader per differing place, measured on the sampled verse pairs.
fn measured_saving(x: &str, y: &str, p: LexParams, m: &Measure, samples: &Samples, pairs: &[(&Corpus, &Corpus)], only_pair: Option<usize>) -> f64 {
    let mut one = Lexicon { max_len: p.max_len, feats: p.feats, coarse: p.coarse, fw: p.fw, clauses: p.clauses, joined: p.joined, flat: p.flat, ..Default::default() };
    for w in [x, y] {
        if w.contains(' ') {
            one.phrases.insert(w.to_string(), x.replace(' ', "_"));
        } else if w != x {
            one.single.insert(w.to_string(), x.replace(' ', "_"));
        }
    }
    let none = Lexicon { max_len: p.max_len, feats: p.feats, coarse: p.coarse, fw: p.fw, clauses: p.clauses, joined: p.joined, flat: p.flat, ..Default::default() };
    let mut got: Vec<(usize, VKey, VKey)> = samples.get(&(x.to_string(), y.to_string())).cloned().unwrap_or_default();
    // the other orientation: y on the first author's side
    for (pi, ka, kb) in samples.get(&(y.to_string(), x.to_string())).cloned().unwrap_or_default() {
        if got.len() < 12 {
            got.push((pi, ka, kb));
        }
    }
    // one author pair's own samples, when it has any
    if let Some(op) = only_pair {
        let own: Vec<(usize, VKey, VKey)> = got.iter().copied().filter(|s| s.0 == op).collect();
        if !own.is_empty() {
            got = own;
        }
    }
    if got.is_empty() {
        return 0.0;
    }
    let d: Vec<f64> = got
        .iter()
        .map(|(pi, ka, kb)| {
            let (wa, wb) = (&pairs[*pi].0.verses[ka].meta_words(), &pairs[*pi].1.verses[kb].meta_words());
            m(&none.read(wa), &none.read(wb)) - m(&one.read(wa), &one.read(wb))
        })
        .collect();
    let n = d.len() as f64;
    let mean = d.iter().sum::<f64>() / n;
    if !p.lb {
        return mean;
    }
    // the 95% lower bound of the mean saving (one-sided): a merge seen once, or with savings that scatter, is not
    // trusted with its mean
    if d.len() < 2 {
        return 0.0f64.min(mean);
    }
    let var = d.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (n - 1.0);
    mean - 1.645 * (var / n).sqrt()
}

#[allow(clippy::too_many_arguments)]
fn judged(links: &[(String, String, String)], gapc: &BTreeMap<(String, String), u32>, cs: &[&Corpus], verses: &BTreeSet<VKey>, p: LexParams, measure: Option<(&Measure, &Samples, &[(&Corpus, &Corpus)])>, pp: (&[(&Corpus, &Corpus)], &[BTreeMap<(String, String), u32>]), logic: Option<&LogicFn>, refused: &mut Vec<String>) -> Vec<(String, String, String)> {
    // each author pair's corpora among the judged ones
    let idx = |c: &Corpus| cs.iter().position(|x| x.name == c.name).map(|i| i as u8);
    let pair_cs: Vec<Vec<u8>> = pp.0.iter().map(|(a, b)| [idx(a), idx(b)].into_iter().flatten().collect()).collect();
    let members: BTreeSet<String> = links.iter().flat_map(|(x, y, _)| [x.clone(), y.clone()]).collect();
    let occ = occurrences(&members, cs, verses, p.max_len);
    let gap = |a: &str, b: &str| -> u32 {
        let k = if a < b { (a.to_string(), b.to_string()) } else { (b.to_string(), a.to_string()) };
        gapc.get(&k).copied().unwrap_or(0)
    };
    // candidates by their gap count, most first
    let mut cand: Vec<(u32, &String, &String)> = links.iter().map(|(x, y, _)| (gap(x, y), x, y)).collect();
    cand.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)).then(a.2.cmp(b.2)));
    cand.dedup_by(|a, b| (a.1 == b.1 && a.2 == b.2) || (a.1 == b.2 && a.2 == b.1));
    // a member pair's measured saving does not change as clusters grow: measured once
    let memo: std::cell::RefCell<BTreeMap<(String, String, Option<usize>), f64>> = Default::default();
    let memo_saving = |m1: &str, m2: &str, op: Option<usize>, f: &dyn Fn() -> f64| -> f64 {
        let k = (m1.to_string(), m2.to_string(), op);
        if let Some(v) = memo.borrow().get(&k) {
            return *v;
        }
        let v = f();
        memo.borrow_mut().insert(k, v);
        v
    };
    let mut cluster_of: BTreeMap<String, usize> = BTreeMap::new();
    let mut clusters: Vec<Vec<String>> = Vec::new();
    let mut out = Vec::new();
    for (_, x, y) in cand {
        let cx = *cluster_of.entry(x.clone()).or_insert_with(|| {
            clusters.push(vec![x.clone()]);
            clusters.len() - 1
        });
        let cy = *cluster_of.entry(y.clone()).or_insert_with(|| {
            clusters.push(vec![y.clone()]);
            clusters.len() - 1
        });
        if cx == cy {
            continue;
        }
        let (a, b) = (&clusters[cx], &clusters[cy]);
        let saved: f64 = a
            .iter()
            .flat_map(|m1| b.iter().map(move |m2| (m1, m2)))
            .map(|(m1, m2)| {
                let g = gap(m1, m2) as f64;
                if g == 0.0 {
                    return 0.0;
                }
                match measure {
                    Some((m, smp, prs)) => g * memo_saving(m1, m2, None, &|| measured_saving(m1, m2, p, m, smp, prs, None)),
                    None => g * 17.0 * words_of(m1).max(words_of(m2)),
                }
            })
            .sum();
        let mut u = a.clone();
        u.extend(b.iter().cloned());
        let wl = u.iter().map(|m| words_of(m)).sum::<f64>() / u.len() as f64;
        let cost = (errors(&u, &occ) as f64 - errors(a, &occ) as f64 - errors(b, &occ) as f64) * 17.0 * wl;
        // per author pair: what this pair's reader saves against what the pair's own authors' encoder pays (by tag)
        let mut pair_ok = true;
        if p.per_pair {
            for (pi, gp) in pp.1.iter().enumerate() {
                let g = |m1: &String, m2: &String| -> f64 {
                    let k = if m1 < m2 { (m1.clone(), m2.clone()) } else { (m2.clone(), m1.clone()) };
                    gp.get(&k).copied().unwrap_or(0) as f64
                };
                let sv: f64 = a
                    .iter()
                    .flat_map(|m1| b.iter().map(move |m2| (m1, m2)))
                    .map(|(m1, m2)| {
                        let n = g(m1, m2);
                        if n == 0.0 {
                            return 0.0;
                        }
                        match measure {
                            Some((m, smp, prs)) => {
                                let op = if p.own_samples { Some(pi) } else { None };
                                n * memo_saving(m1, m2, op, &|| measured_saving(m1, m2, p, m, smp, prs, op))
                            }
                            None => n * 17.0 * words_of(m1).max(words_of(m2)),
                        }
                    })
                    .sum();
                let only = Some(&pair_cs[pi][..]);
                let cp = (errors_in(&u, &occ, only, true) as f64 - errors_in(a, &occ, only, true) as f64 - errors_in(b, &occ, only, true) as f64) * 17.0 * wl;
                if sv - cp < 0.0 {
                    pair_ok = false;
                    break;
                }
            }
        }
        // the logic gate: the merge must not make the sampled verse pairs contradict each other more
        let mut logic_ok = true;
        // first the words themselves: a negation, a modal or a number never joins a word that is not the same
        if logic.is_some() && pair_ok && saved - cost >= 51.0 {
            let lw = |m: &str| crate::logic::logical_words(&m.split(' ').collect::<Vec<_>>());
            if let Some((m1, m2)) = a.iter().flat_map(|m1| b.iter().map(move |m2| (m1, m2))).find(|(m1, m2)| lw(m1) != lw(m2)) {
                logic_ok = false;
                refused.push(format!("«{x}» ~ «{y}»: saves {saved:.0} bits, costs {cost:.0}, but «{m1}» and «{m2}» differ in negation, modality or number"));
            }
        }
        if let (Some(lf), Some((_, smp, prs)), true) = (logic, measure, logic_ok && pair_ok && saved - cost >= 51.0) {
            let rise: i64 = a.iter().flat_map(|m1| b.iter().map(move |m2| (m1, m2))).filter(|(m1, m2)| gap(m1, m2) > 0).map(|(m1, m2)| logic_rise(m1, m2, lf, smp, prs)).sum();
            if rise > 0 {
                logic_ok = false;
                refused.push(format!("«{x}» ~ «{y}»: saves {saved:.0} bits, costs {cost:.0}, and {rise} more contradictions on its samples"));
            }
        }
        if pair_ok && logic_ok && saved - cost >= 51.0 {
            out.push((x.clone(), y.clone(), format!("«{x}» ~ «{y}»: saves the reader {saved:.0} bits, costs the encoder {cost:.0}")));
            let moved = std::mem::take(&mut clusters[cy]);
            for m in &moved {
                cluster_of.insert(m.clone(), cx);
            }
            clusters[cx].extend(moved);
        }
    }
    out
}

impl Lexicon {
    /// The lexicon as `lexicon.tsv`: concept, member, count, why (one row per member).
    pub fn to_tsv(&self) -> String {
        let mut lx = String::from("concept\tmember\tcount\twhy\n");
        for (c, ms) in &self.clusters {
            for (m, n) in ms {
                lx.push_str(&format!("{c}\t{m}\t{n}\t{}\n", self.why.get(m).cloned().unwrap_or_default()));
            }
        }
        lx
    }

    /// A lexicon back from its `lexicon.tsv` (a run's, as it is): a member with a space is a phrase, any other
    /// member that is not its concept's name reads as the concept. The reading switches are the caller's.
    pub fn from_tsv(text: &str, max_len: usize) -> Lexicon {
        let mut lex = Lexicon { max_len, ..Default::default() };
        for l in text.lines().skip(1) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() < 3 {
                continue;
            }
            let (c, m) = (f[0].to_string(), f[1].to_string());
            if m.contains(' ') {
                lex.phrases.insert(m.clone(), c.clone());
            } else if m != c {
                lex.single.insert(m.clone(), c.clone());
            }
            if let Some(w) = f.get(3).filter(|w| !w.is_empty()) {
                lex.why.insert(m.clone(), w.to_string());
            }
            lex.clusters.entry(c).or_default().push((m, f[2].parse().unwrap_or(0)));
        }
        lex
    }

    /// This lexicon with another's clusters added, those none of whose members is already here (the refused merges
    /// of both kept).
    pub fn extended(&self, o: &Lexicon) -> Lexicon {
        let mut l = self.clone();
        let mine: BTreeSet<&str> = self.clusters.values().flatten().map(|(m, _)| m.as_str()).collect();
        for (c, ms) in &o.clusters {
            if ms.iter().any(|(m, _)| mine.contains(m.as_str())) || l.clusters.contains_key(c) {
                continue;
            }
            for (m, _) in ms {
                if m.contains(' ') {
                    l.phrases.insert(m.clone(), c.clone());
                } else if m != c {
                    l.single.insert(m.clone(), c.clone());
                }
                if let Some(w) = o.why.get(m) {
                    l.why.insert(m.clone(), w.clone());
                }
            }
            l.clusters.insert(c.clone(), ms.clone());
        }
        l.refused_logic.extend(o.refused_logic.iter().cloned());
        l
    }

    /// The concept of a word's key.
    pub fn concept<'a>(&'a self, key: &'a str) -> &'a str {
        self.single.get(key).map(String::as_str).unwrap_or(key)
    }

    /// Read a verse's words (its standard tree) into the meta-world: the longest known phrase first, then the word.
    pub fn read(&self, ws: &[Word]) -> Vec<MTok> {
        let ks = keys(ws);
        let n = ws.len();
        let mut spans: Vec<((usize, usize), String)> = Vec::new();
        let mut i = 0;
        while i < n {
            let mut hit = None;
            for l in (2..=self.max_len.min(n - i)).rev() {
                if let Some(c) = self.phrases.get(&ks[i..i + l].join(" ")) {
                    hit = Some((l, c.clone()));
                    break;
                }
            }
            match hit {
                Some((l, c)) => {
                    spans.push(((i, i + l), c));
                    i += l;
                }
                None => {
                    spans.push(((i, i + 1), self.concept(&ks[i]).to_string()));
                    i += 1;
                }
            }
        }
        if self.fw {
            spans = absorb_function_words(ws, &ks, spans);
        }
        let joiner: Vec<bool> = (0..n).map(|t| self.clauses && is_joiner(&ws[t])).collect();
        let crel: Vec<Option<String>> = if self.clauses { clause_starts(ws, &joiner) } else { vec![None; n] };
        let starts: Vec<bool> = crel.iter().map(Option::is_some).collect();
        if self.clauses {
            spans = absorb_joiners(&joiner, &starts, spans);
        }
        let mut tok2m = vec![0usize; n];
        for (m, ((a, b), _)) in spans.iter().enumerate() {
            for t in *a..*b {
                tok2m[t] = m;
            }
        }
        spans
            .into_iter()
            .map(|((a, b), concept)| {
                let all_join = (a..b).all(|t| joiner[t]);
                let h = (a..b).find(|&t| (all_join || !joiner[t]) && (ws[t].head == 0 || ws[t].head - 1 < a || ws[t].head - 1 >= b)).or_else(|| (a..b).find(|&t| !joiner[t])).unwrap_or(a);
                let mut head = if ws[h].head == 0 || ws[h].head > n { 0 } else { tok2m[ws[h].head - 1] + 1 };
                let mut rel = ws[h].rel;
                // clause level: a coordinated or paratactic predicate and a sentence's root stand side by side at the
                // top of the verse; how they are joined is in the joiners the next clause's first word carries
                if (self.clauses || self.flat) && (rel == Rel::Root || matches!(format!("{}", rel.base()).as_str(), "conj" | "parataxis") && (ws[h].tag.is_verb() || ws[h].tag == Tag::MD || ws[h].upos == en::gram::UPos::VERB)) {
                    head = 0;
                    rel = Rel::Root;
                }
                let concept = if self.feats { with_feats(concept, &ws[h]) } else { concept };
                // a clause starts here (however the author joins it): a feature the encoder writes the joining for
                let concept = match (a..b).find_map(|t| crel[t].clone()) {
                    // joined: the clause's relation and the author's joining words are the feature
                    Some(r) if self.joined => format!("{concept}#j:{r}:{}", (a..b).filter(|&t| joiner[t]).map(|t| ks[t].clone()).collect::<Vec<_>>().join("_")),
                    Some(_) => format!("{concept}#c"),
                    None => concept,
                };
                MTok { concept, tag: ws[h].tag, head, rel, span: (a, b) }
            })
            .collect::<Vec<MTok>>()
            .pipe_coarse(self.coarse)
    }
}

/// A word that joins clauses or sentences: a mark of punctuation between them, a coordinating conjunction.
pub fn is_joiner(w: &Word) -> bool {
    let b = format!("{}", w.rel.base());
    (b == "punct" && matches!(w.form.as_str(), "," | ";" | ":" | "." | "!" | "?")) || (b == "cc" && matches!(w.lemma.to_lowercase().as_str(), "and" | "but" | "or" | "then" | "so" | "nor" | "yet" | "for"))
}

/// The first word (not a joiner) of every clause that stands at the top of the verse: a sentence's root, a
/// coordinated or paratactic predicate — the leftmost word of its own part of the tree (its subtree without the
/// coordinated clauses it heads).
fn clause_starts(ws: &[Word], joiner: &[bool]) -> Vec<Option<String>> {
    let n = ws.len();
    let top = |i: usize| {
        let b = format!("{}", ws[i].rel.base());
        ws[i].rel == Rel::Root || matches!(b.as_str(), "conj" | "parataxis") && (ws[i].tag.is_verb() || ws[i].tag == Tag::MD || ws[i].upos == en::gram::UPos::VERB)
    };
    // the clause a word belongs to: its nearest top ancestor (itself if top)
    let clause_of = |mut i: usize| -> Option<usize> {
        for _ in 0..=n {
            if top(i) {
                return Some(i);
            }
            let h = ws[i].head;
            if h == 0 || h > n {
                return None;
            }
            i = h - 1;
        }
        None
    };
    let mut first: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..n {
        if joiner[i] {
            continue;
        }
        if let Some(c) = clause_of(i) {
            first.entry(c).or_insert(i);
        }
    }
    let mut out = vec![None; n];
    for (c, f) in first {
        out[f] = Some(if ws[c].rel == Rel::Root { "root".to_string() } else { format!("{}", ws[c].rel.base()) });
    }
    out
}

/// The joiners before the first word of a clause go with it: «, and he said» and «. He said» are both a meta token
/// «he#c» whose chunk the author writes. Joiners inside a clause and at the end of the verse stay as they are.
fn absorb_joiners(joiner: &[bool], starts: &[bool], spans: Vec<((usize, usize), String)>) -> Vec<((usize, usize), String)> {
    let mut out: Vec<((usize, usize), String)> = Vec::new();
    let mut run: Option<usize> = None;
    let mut pending: Vec<((usize, usize), String)> = Vec::new();
    for (sp, c) in spans {
        let j = sp.1 == sp.0 + 1 && joiner[sp.0];
        if j {
            run.get_or_insert(sp.0);
            pending.push((sp, c));
        } else if (sp.0..sp.1).any(|t| starts[t]) {
            // a clause starts here: the joiners before it are its joining
            let start = run.take().unwrap_or(sp.0);
            pending.clear();
            out.push(((start, sp.1), c));
        } else {
            // joiners inside a clause (a comma before an apposition) stay words of their own
            run = None;
            out.append(&mut pending);
            out.push((sp, c));
        }
    }
    out.extend(pending);
    out
}

/// Determiners and prepositions as features: a span of one such word whose head is in the span right after it joins
/// that span, and the concept carries the words («in the house» → `house#in_the`). Two authors who differ in a
/// determiner then differ in one feature of one meta token, not in a word of their own; the encoder writes the
/// author's words for the whole.
fn absorb_function_words(ws: &[Word], ks: &[String], spans: Vec<((usize, usize), String)>) -> Vec<((usize, usize), String)> {
    let func = |t: usize| {
        let b = format!("{}", ws[t].rel.base());
        matches!(b.as_str(), "det" | "case") && !is_punct(&ws[t])
    };
    let mut out: Vec<((usize, usize), String, Vec<String>)> = Vec::new();
    for (sp, c) in spans.into_iter().rev() {
        let absorb = sp.1 == sp.0 + 1 && func(sp.0) && out.last().is_some_and(|((a, b), _, _)| *a == sp.1 && ws[sp.0].head >= a + 1 && ws[sp.0].head <= *b);
        if absorb {
            let last = out.last_mut().unwrap();
            last.0.0 = sp.0;
            last.2.insert(0, ks[sp.0].clone());
        } else {
            out.push((sp, c, Vec::new()));
        }
    }
    out.reverse();
    out.into_iter().map(|(sp, c, f)| (sp, if f.is_empty() { c } else { format!("{c}#{}", f.join("_")) })).collect()
}

/// A pronoun's concept with its number and person from the parse's FEATS («thou#S2», «ye#P2»): what the members of
/// a merged concept (thou, thee, ye, you) differ in and the context alone does not tell. Other words as they are.
pub fn with_feats(concept: String, w: &Word) -> String {
    if !matches!(w.tag, Tag::PRP | Tag::PRPS) {
        return concept;
    }
    let f = format!("{}", w.feats);
    let n = if f.contains("Number=Sing") { "S" } else if f.contains("Number=Plur") { "P" } else { "" };
    let p = ["1", "2", "3"].into_iter().find(|d| f.contains(&format!("Person={d}"))).unwrap_or("");
    if n.is_empty() && p.is_empty() { concept } else { format!("{concept}#{n}{p}") }
}

/// A concept without its features.
pub fn bare_concept(c: &str) -> &str {
    c.split('#').next().unwrap_or(c)
}

/// A relation as a frame role: subject, object, oblique, a noun's modifier, a verb's modifier, a clause link, a
/// function word, punctuation, root.
pub fn coarse_rel(r: Rel) -> Rel {
    let b = format!("{}", r.base());
    let to = match b.as_str() {
        "nsubj" | "csubj" | "expl" => "nsubj",
        "obj" | "iobj" => "obj",
        "obl" => "obl",
        "nmod" | "appos" | "nummod" | "amod" | "acl" | "compound" | "flat" | "fixed" | "goeswith" | "list" | "orphan" => "nmod",
        "advmod" | "discourse" | "vocative" | "dislocated" => "advmod",
        "conj" | "parataxis" | "ccomp" | "xcomp" | "advcl" => "conj",
        "det" | "case" | "mark" | "cc" | "aux" | "cop" | "clf" => "det",
        "punct" => "punct",
        "root" => "root",
        _ => "dep",
    };
    Rel::parse(to).unwrap_or(r)
}

trait PipeCoarse {
    fn pipe_coarse(self, on: bool) -> Vec<MTok>;
}

impl PipeCoarse for Vec<MTok> {
    /// Coarse relations; a punctuation mark hangs on the meta token before it (the first on the one after), where
    /// two parses of one verse agree more than on the parser's own choice of head.
    fn pipe_coarse(mut self, on: bool) -> Vec<MTok> {
        if !on {
            return self;
        }
        let n = self.len();
        for i in 0..n {
            self[i].rel = coarse_rel(self[i].rel);
            if self[i].rel == Rel::Punct {
                self[i].head = if i > 0 { i } else if n > 1 { 2 } else { 0 };
            }
        }
        self
    }
}

/// The meta-world as a tree the roundtrip machinery can align and patch.
pub fn tnodes(m: &[MTok]) -> Vec<TNode> {
    m.iter().map(|x| TNode { lemma: x.concept.clone(), tag: x.tag, head: x.head, rel: x.rel }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn the_logic_gate_on_merges() {
        use crate::corpus::PVerse;
        use crate::logic::Event;
        let ev = |v: &str, neg: bool| Event { verb: v.into(), neg, modal: None, roles: vec![(1, "moses".into(), crate::logic::NAME)] };
        let k = (5u8, 1u16, 1u16);
        let c = |name: &str, e: Event| Corpus { name: name.into(), kind: crate::sources::Kind::Bible, verses: [(k, PVerse { facts: vec![e], ..Default::default() })].into() };
        let (a, b) = (c("a", ev("come", true)), c("b", ev("go", false)));
        let pairs = [(&a, &b)];
        let samples: Samples = [(("come".to_string(), "go".to_string()), vec![(0, k, k)])].into();
        let lf = |l: &Lexicon, p: &crate::corpus::PVerse, q: &crate::corpus::PVerse| crate::logic::compare(l, &p.facts, &q.facts).contra;
        // merging «come» and «go» makes a negated coming and a going one event that contradicts itself
        assert_eq!(logic_rise("come", "go", &lf, &samples, &pairs), 1);
        // negative control: with the same polarity the merge only joins them
        let (a2, b2) = (c("a", ev("come", false)), c("b", ev("go", false)));
        assert_eq!(logic_rise("come", "go", &lf, &samples, &[(&a2, &b2)]), 0);
    }

    #[test]
    fn lexicon_tsv_back() {
        let mut l = Lexicon::default();
        l.single.insert("spake".into(), "speak".into());
        l.phrases.insert("come to pass".into(), "happen".into());
        l.clusters.insert("speak".into(), vec![("speak".into(), 9), ("spake".into(), 4)]);
        l.clusters.insert("happen".into(), vec![("come to pass".into(), 3), ("happen".into(), 5)]);
        l.why.insert("spake".into(), "a pair".into());
        let b = Lexicon::from_tsv(&l.to_tsv(), 4);
        assert_eq!((b.single.clone(), b.phrases.clone(), b.clusters.clone(), b.why.clone()), (l.single.clone(), l.phrases.clone(), l.clusters.clone(), l.why.clone()));
        // extending: a cluster that touches a member already there is not added, a new one is
        let mut o = Lexicon::default();
        o.single.insert("seide".into(), "say".into());
        o.clusters.insert("say".into(), vec![("say".into(), 9), ("seide".into(), 2)]);
        o.single.insert("speaketh".into(), "speak".into());
        o.clusters.insert("speak".into(), vec![("speak".into(), 9), ("speaketh".into(), 2)]);
        let e = b.extended(&o);
        assert_eq!(e.concept("seide"), "say");
        assert_eq!(e.concept("speaketh"), "speaketh");
        // a phrase pair that shares a word is the single words' business
        let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert!(shares_word(&s(&["LORD", "say"]), &s(&["lord", "seide"])));
        assert!(!shares_word(&s(&["come", "to", "pass"]), &s(&["happen"])));
        assert!(!shares_word(&s(&["say"]), &s(&["seide"])));
    }

    #[test]
    fn lcs_and_gaps() {
        let a = s(&["and", "it", "come", "to", "pass", "when", "he", "go"]);
        let b = s(&["it", "happen", "when", "he", "go"]);
        let an = lcs(&a, &b);
        assert_eq!(an, vec![(1, 0), (5, 2), (6, 3), (7, 4)]);
        let g = gaps(a.len(), b.len(), &an);
        assert_eq!(g, vec![((0, 1), (0, 0)), ((2, 5), (1, 2))]);
        assert!(side_ok(&a[2..5], 4));
        assert!(!side_ok(&s(&[",", "and"]), 4));
        // a modal is never glued to a concept: «may Yahweh» is no phrase
        assert!(!side_ok(&s(&["may", "yahweh"]), 4));
        assert!(side_ok(&s(&["may"]), 4));
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn metadata_loss_and_lcs_ties() {
        let mut lex = Lexicon::from_tsv("concept\tmember\tcount\twhy\nsay\tspake\t4\treason\n", 3);
        lex.feats = true;
        lex.coarse = true;
        lex.refused_logic.push("refused".into());
        let back = Lexicon::from_tsv(&lex.to_tsv(), 7);
        assert_eq!(back.concept("spake"), "say");
        assert!(!back.feats && !back.coarse && back.refused_logic.is_empty());
        assert_eq!(back.max_len, 7);
        let a = vec!["a".into(), "b".into()];
        let b = vec!["b".into(), "a".into()];
        assert_eq!(lcs(&a, &b), vec![(1, 0)]);
        assert!(lcs(&[], &b).is_empty());
        assert_eq!(gaps(0, 2, &[]), vec![((0, 0), (0, 2))]);
        assert_eq!(bare_concept("you#S2#j:root:and"), "you");
    }
}
