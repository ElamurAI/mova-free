//! Part-of-speech tagger (PTB) — averaged perceptron (Collins 2002) with a greedy left-to-right
//! pass (Honnibal 2013). Features are numbers (hash of parts), weights are per tag; the hint of the in-code dictionary
//! is a feature too (mask of tags the dictionary gives the form), so the model learns how much to trust it.
//!
//! Compared to TnT (`tag.rs`) this has neighbouring words, prefixes, word shape and the "previous tag +
//! word" pair: the Markov model sees only the tag chain and the word itself. For unknown words TnT remains
//! a prior: its P(tag | suffix, dictionary) is added with a weight to the perceptron score.
//!
//! EWT test:
//! - TnT — 93.12%;
//! - greedy perceptron — 93.74%;
//! - plus TnT prior — 94.13%;
//! - plus neighbour shape and beam 8 — 94.35%.
//!
//! Parser on these tags: LAS 77.27 → 79.00.
//! Did not help (25.09): a separate feature per tag of the dictionary mask (unknown 71 → 65%) and
//! training rare words without features of the word itself (±0.1).

use crate::conllu::Sentence;
use crate::ctx::{Codes, Ctx, G_ENT};
use crate::dict;
use crate::gram::Tag;
use crate::hash::{FastMap, key, str_key};

const NT: usize = Tag::N;
/// Unambiguous frequent words are tagged directly by the training dictionary.
const DICT_FREQ: u32 = 20;
const DICT_SHARE: f64 = 0.97;
/// Weight of the TnT prior ln P(tag) for unknown words (EWT dev: 0 → 93.19%, 1 → 93.65%,
/// 4 → 93.73%, 8 → 93.68%).
const ALPHA: f32 = 4.0;
/// Epoch from which words that are wrong in every epoch no longer update weights; `EN_ROBUST_FROM` overrides
/// (0 — off). Principle: examples are not truth.
///
/// Measured 25.09 on EWT test:
/// - on clean data — ±0.1;
/// - with 5% corrupted tags — 94.00 → 94.10;
/// - suspicions find corrupted labels with 95.8% precision and 63% recall;
/// - on clean EWT — 185 suspicions: some are real gold errors, some are borderline cases.
fn robust_from() -> usize {
    std::env::var("EN_ROBUST_FROM").ok().and_then(|x| x.parse().ok()).unwrap_or(3)
}

/// Default beam width for tagging (the `beam` field changes it without retraining). Measured
/// 25.09 (EWT / PUD / GUM): 1 → 94.34 / 94.03 / 93.72; 2 → 94.38 / 94.09 / 93.77; 8 → 94.35 / 94.04 /
/// 93.82 — wider than 2 gives nothing, only slower.
const BEAM: usize = 2;

/// Normalized word for features: lowercase, digits → 0 (numbers generalize by shape).
fn norm(w: &str) -> String {
    w.chars().map(|c| if c.is_ascii_digit() { '0' } else { c }).flat_map(char::to_lowercase).collect()
}

/// Word shape: uppercase → X, lowercase → x, digits → d, the rest as is; repeats are collapsed.
fn shape(w: &str) -> u64 {
    let mut s = String::new();
    for c in w.chars() {
        let k = if c.is_uppercase() {
            'X'
        } else if c.is_lowercase() {
            'x'
        } else if c.is_ascii_digit() {
            'd'
        } else {
            c
        };
        if !s.ends_with(k) {
            s.push(k);
        }
    }
    str_key(&s)
}

/// Mask of tags the in-code dictionary gives the form (0 — form not in the dictionary).
fn dict_mask(w: &str) -> u64 {
    let mut m = 0u64;
    if let Some(f) = dict::form(&w.to_lowercase()) {
        for r in dict::analyses(f) {
            m |= 1 << r.tag.idx();
        }
    }
    m
}

/// Word in a sentence: what features take from neighbours.
struct W {
    norm: u64,
    suf3: u64,
    mask: u64,
    shape: u64,
}

fn chars_key(kind: u64, cs: &[char]) -> u64 {
    let mut parts = Vec::with_capacity(cs.len() + 1);
    parts.push(kind);
    parts.extend(cs.iter().map(|&c| c as u64));
    key(&parts)
}

fn prep(words: &[&str]) -> Vec<W> {
    words
        .iter()
        .map(|w| {
            let n = norm(w);
            let cs: Vec<char> = n.chars().collect();
            W { norm: str_key(&n), suf3: chars_key(3, &cs[cs.len().saturating_sub(3)..]), mask: dict_mask(w), shape: shape(w) }
        })
        .collect()
}

/// Features of position `i` given already assigned tags `p1` (i−1) and `p2` (i−2); `NT` is the sentence boundary.
fn features(words: &[&str], ws: &[W], i: usize, p1: usize, p2: usize, out: &mut Vec<u64>) {
    out.clear();
    let w = words[i];
    let n = norm(w);
    let cs: Vec<char> = n.chars().collect();
    let len = cs.len();
    let cur = &ws[i];
    let at = |j: isize| -> Option<&W> { if j < 0 { None } else { ws.get(j as usize) } };
    const B: u64 = u64::MAX; // sentence boundary
    let nb = |x: Option<&W>| x.map_or(B, |x| x.norm);
    let sb = |x: Option<&W>| x.map_or(B, |x| x.suf3);
    let mb = |x: Option<&W>| x.map_or(B, |x| x.mask);
    let hb = |x: Option<&W>| x.map_or(B, |x| x.shape);
    let i_ = i as isize;
    let first_upper = w.chars().next().is_some_and(char::is_uppercase);
    out.extend([
        key(&[0]),
        key(&[1, cur.norm]),
        key(&[2, p1 as u64]),
        key(&[3, p1 as u64, p2 as u64]),
        key(&[4, p1 as u64, cur.norm]),
        key(&[5, nb(at(i_ - 1))]),
        key(&[6, nb(at(i_ + 1))]),
        key(&[7, nb(at(i_ - 2))]),
        key(&[8, nb(at(i_ + 2))]),
        key(&[9, sb(at(i_ - 1))]),
        key(&[10, sb(at(i_ + 1))]),
        key(&[11, cur.shape]),
        key(&[20, hb(at(i_ + 1))]),
        key(&[21, hb(at(i_ - 1))]),
        key(&[12, first_upper as u64, (i == 0) as u64]),
        key(&[13, cur.mask]),
        key(&[14, cur.mask, p1 as u64]),
        key(&[15, mb(at(i_ + 1))]),
        key(&[16, nb(at(i_ - 1)), cur.norm]),
        key(&[17, cur.norm, nb(at(i_ + 1))]),
    ]);
    for k in 1..=4.min(len) {
        out.push(chars_key(100 + k as u64, &cs[len - k..]));
    }
    for k in 1..=3.min(len) {
        out.push(chars_key(200 + k as u64, &cs[..k]));
    }
    if w.contains('-') {
        out.push(key(&[18]));
    }
}

/// Document-context features of position `i` (`crate::ctx`) — for each group: class offset, × word
/// shape, × first word (capitalized, or first in the sentence); entity — code of token `i`. With
/// `EN_CTX_WORD=1` — also × the word itself. Empty context — no features.
fn ctx_features(cx: &Codes, words: &[&str], ws: &[W], i: usize, out: &mut Vec<u64>) {
    let first_upper = words[i].chars().next().is_some_and(char::is_uppercase) as u64;
    let first = (i == 0) as u64;
    let e = cx.ent(i);
    for (g, v) in cx.sent.iter().copied().chain((e != 0).then_some((G_ENT, e))) {
        out.push(key(&[300, g, v]));
        out.push(key(&[301, g, v, ws[i].shape]));
        out.push(key(&[302, g, v, first_upper, first]));
        if cx.word {
            out.push(key(&[303, g, v, ws[i].norm]));
        }
    }
}

/// Feature weight for one tag; during training — with a sum for averaging.
#[derive(Clone, Copy, Default)]
struct Cell {
    w: f32,
    total: f64,
    stamp: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PTagger {
    /// TnT on the same data: for unknown words its P(tag | suffix, dictionary) is the prior
    tnt: crate::tag::Tagger,
    /// weight of the prior ln P for unknown words (0 — off)
    pub alpha: f32,
    /// beam width for tagging (1 — greedy)
    pub beam: usize,
    /// suspicious labels of the training corpus: (sentence among tagged, word) — the model disagreed
    /// with them in every epoch; candidate annotation errors for the report
    pub suspects: Vec<(u32, u32)>,
    /// feature → [(tag, weight)] non-zero only
    weights: FastMap<u64, Vec<(u8, f32)>>,
    /// frequent unambiguous words (as is) → tag
    known: FastMap<String, u8>,
    /// tags seen in training
    #[serde(with = "crate::store::arr")]
    seen: [bool; NT],
}

fn scores(weights: &FastMap<u64, Vec<(u8, f32)>>, feats: &[u64]) -> [f32; NT] {
    let mut s = [0.0f32; NT];
    for f in feats {
        if let Some(v) = weights.get(f) {
            for &(t, w) in v {
                s[t as usize] += w;
            }
        }
    }
    s
}

fn best(s: &[f32; NT], seen: &[bool; NT]) -> usize {
    let mut b = usize::MAX;
    for t in 0..NT {
        if seen[t] && (b == usize::MAX || s[t] > s[b]) {
            b = t;
        }
    }
    b
}

impl PTagger {
    pub fn train(sents: &[Sentence], epochs: usize) -> PTagger {
        Self::train_ctx(sents, None, epochs)
    }

    /// Training with document context: `ctx` is the context of each sentence of `sents` (same
    /// order). `None` — same as `train`, bit for bit.
    pub fn train_ctx(sents: &[Sentence], ctx: Option<&[Ctx]>, epochs: usize) -> PTagger {
        if let Some(c) = ctx {
            assert_eq!(c.len(), sents.len(), "context for every sentence");
        }
        let kept: Vec<usize> = (0..sents.len()).filter(|&k| sents[k].tagged()).collect();
        let data: Vec<(Vec<&str>, Vec<usize>)> = kept
            .iter()
            .map(|&k| &sents[k])
            .map(|s| (s.tokens.iter().map(|t| t.form.as_str()).collect(), s.tokens.iter().map(|t| t.tag.expect("tagged").idx()).collect()))
            .collect();
        let cxs: Vec<Codes> = kept.iter().map(|&k| ctx.map_or_else(Codes::default, |c| Codes::of(&c[k], sents[k].tokens.len()))).collect();
        let mut seen = [false; NT];
        let mut freq: FastMap<&str, [u32; NT]> = FastMap::default();
        for (ws, ts) in &data {
            for (w, &t) in ws.iter().zip(ts) {
                seen[t] = true;
                freq.entry(w).or_insert([0; NT])[t] += 1;
            }
        }

        let known: FastMap<String, u8> = freq
            .iter()
            .filter_map(|(w, c)| {
                let n: u32 = c.iter().sum();
                let (t, &m) = c.iter().enumerate().max_by_key(|x| x.1)?;
                (n >= DICT_FREQ && m as f64 / n as f64 >= DICT_SHARE).then(|| (w.to_string(), t as u8))
            })
            .collect();
        let mut cells: FastMap<u64, Vec<(u8, Cell)>> = FastMap::default();
        let mut now: u32 = 0;
        let mut order: Vec<usize> = (0..data.len()).collect();
        let mut rng: u64 = crate::ctx::seeded(0x9E37_79B9_7F4A_7C15);
        let mut feats = Vec::with_capacity(32);
        // "examples are not truth": how many epochs in a row the model disagrees with
        // a word's label. From epoch ROBUST_FROM a word that is wrong in every epoch gets no updates:
        // it is a suspected annotation error, not something to memorize.
        let robust_from = robust_from();
        let mut wrong: Vec<Vec<u8>> = data.iter().map(|(w, _)| vec![0u8; w.len()]).collect();
        for epoch in 0..epochs {
            // deterministic shuffle (xorshift)
            for i in (1..order.len()).rev() {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                order.swap(i, (rng % (i as u64 + 1)) as usize);
            }
            for &k in &order {
                let (words, gold) = &data[k];
                let ws = prep(words);
                let (mut p1, mut p2) = (NT, NT);
                for i in 0..words.len() {
                    let g = gold[i];
                    let guess = if let Some(&t) = known.get(words[i]) {
                        t as usize
                    } else {
                        features(words, &ws, i, p1, p2, &mut feats);
                        if !cxs[k].is_empty() {
                            ctx_features(&cxs[k], words, &ws, i, &mut feats);
                        }
                        let mut s = [0.0f32; NT];
                        for f in &feats {
                            if let Some(v) = cells.get(f) {
                                for (t, c) in v {
                                    s[*t as usize] += c.w;
                                }
                            }
                        }
                        let guess = best(&s, &seen);
                        now += 1;
                        if guess != g {
                            wrong[k][i] = wrong[k][i].saturating_add(1);
                        }
                        let suspect = robust_from > 0 && epoch >= robust_from && wrong[k][i] as usize == epoch + 1;
                        if guess != g && !suspect {
                            for &f in &feats {
                                let v = cells.entry(f).or_default();
                                for (t, d) in [(g, 1.0f32), (guess, -1.0)] {
                                    let pos = match v.iter().position(|(x, _)| *x as usize == t) {
                                        Some(p) => p,
                                        None => {
                                            v.push((t as u8, Cell::default()));
                                            v.len() - 1
                                        }
                                    };
                                    let c = &mut v[pos].1;
                                    c.total += (now - c.stamp) as f64 * c.w as f64;
                                    c.stamp = now;
                                    c.w += d;
                                }
                            }
                        }
                        guess
                    };
                    p2 = p1;
                    p1 = guess;
                }
            }
        }
        // averaging
        let weights = cells
            .into_iter()
            .filter_map(|(f, v)| {
                let v: Vec<(u8, f32)> = v
                    .into_iter()
                    .filter_map(|(t, c)| {
                        let avg = (c.total + (now - c.stamp) as f64 * c.w as f64) / now.max(1) as f64;
                        (avg != 0.0).then_some((t, avg as f32))
                    })
                    .collect();
                (!v.is_empty()).then_some((f, v))
            })
            .collect();
        // suspicious labels: a word the model got wrong in every epoch
        let suspects: Vec<(u32, u32)> = if robust_from > 0 {
            wrong.iter().enumerate().flat_map(|(k, v)| v.iter().enumerate().filter(|(_, c)| **c as usize == epochs).map(move |(i, _)| (k as u32, i as u32))).collect()
        } else {
            Vec::new()
        };
        PTagger { tnt: crate::tag::Tagger::train(sents), alpha: ALPHA, beam: BEAM, weights, known, seen, suspects }
    }

    /// Tag scores at position `i` given the two previous tags (with the TnT prior for unknown words).
    fn local(&self, words: &[&str], ws: &[W], i: usize, p1: usize, p2: usize, cx: &Codes, feats: &mut Vec<u64>) -> [f32; NT] {
        features(words, ws, i, p1, p2, feats);
        if !cx.is_empty() {
            ctx_features(cx, words, ws, i, feats);
        }
        let mut s = scores(&self.weights, feats);
        if self.alpha > 0.0 {
            let (known, probs) = self.tnt.word_classes(words[i]);
            if !known {
                let mut prior = [f32::NEG_INFINITY; NT];
                for (t, p) in probs {
                    if p > 0.0 {
                        prior[t.idx()] = self.alpha * (p as f32).ln();
                    }
                }
                for t in 0..NT {
                    s[t] += prior[t];
                }
            }
        }
        for t in 0..NT {
            if !self.seen[t] {
                s[t] = f32::NEG_INFINITY;
            }
        }
        s
    }

    /// Beam of width `BEAM` with state merging (last two tags): a later word can fix
    /// an earlier choice — the greedy pass cannot do that ("a high near/NN 73").
    pub fn tag(&self, words: &[&str]) -> Vec<Tag> {
        self.tag_ctx(words, None)
    }

    /// The same with document context (`None` — same as `tag`).
    pub fn tag_ctx(&self, words: &[&str], ctx: Option<&Ctx>) -> Vec<Tag> {
        let cx = ctx.map_or_else(Codes::default, |c| Codes::of(c, words.len()));
        let ws = prep(words);
        let mut feats = Vec::with_capacity(40);
        // (score, tag i−1, tag i−2, index in history)
        let mut beam: Vec<(f32, usize, usize, usize)> = vec![(0.0, NT, NT, usize::MAX)];
        let mut hist: Vec<(u8, usize)> = Vec::new(); // (tag, previous node)
        for i in 0..words.len() {
            let mut next: Vec<(f32, usize, usize, usize)> = Vec::new();
            for &(sc, p1, p2, h) in &beam {
                let cands: Vec<(usize, f32)> = match self.known.get(words[i]) {
                    Some(&t) => vec![(t as usize, 0.0)],
                    None => {
                        let s = self.local(words, &ws, i, p1, p2, &cx, &mut feats);
                        let mut v: Vec<(usize, f32)> = (0..NT).filter(|&t| s[t] > f32::NEG_INFINITY).map(|t| (t, s[t])).collect();
                        v.sort_by(|a, b| b.1.total_cmp(&a.1));
                        v.truncate(self.beam.max(1));
                        v
                    }
                };
                for (t, x) in cands {
                    hist.push((t as u8, h));
                    next.push((sc + x, t, p1, hist.len() - 1));
                }
            }
            next.sort_by(|a, b| b.0.total_cmp(&a.0));
            let mut seen_state: Vec<(usize, usize)> = Vec::new();
            beam.clear();
            for x in next {
                if beam.len() == self.beam.max(1) {
                    break;
                }
                if !seen_state.contains(&(x.1, x.2)) {
                    seen_state.push((x.1, x.2));
                    beam.push(x);
                }
            }
        }
        let mut out = vec![Tag::ALL[0]; words.len()];
        let mut h = beam.first().map_or(usize::MAX, |b| b.3);
        for i in (0..words.len()).rev() {
            out[i] = Tag::ALL[hist[h].0 as usize];
            h = hist[h].1;
        }
        out
    }

    /// The inner TnT — for explaining word classes.
    pub fn tnt(&self) -> &crate::tag::Tagger {
        &self.tnt
    }

    /// Number of features with non-zero weights.
    pub fn size(&self) -> usize {
        self.weights.len()
    }
}

/// Jackknife: tags of the training corpus assigned by a tagger that has not seen the sentence (k folds).
/// A parser trained on such tags gets used to the tagger errors it will live with on new text.
pub fn jackknife(sents: &[Sentence], k: usize, epochs: usize) -> Vec<Sentence> {
    let mut out: Vec<Sentence> = sents.to_vec();
    for f in 0..k {
        let rest: Vec<Sentence> = sents.iter().enumerate().filter(|(i, _)| i % k != f).map(|(_, s)| s.clone()).collect();
        let t = PTagger::train(&rest, epochs);
        for s in out.iter_mut().skip(f).step_by(k) {
            let words: Vec<String> = s.tokens.iter().map(|x| x.form.clone()).collect();
            let refs: Vec<&str> = words.iter().map(String::as_str).collect();
            for (tok, tag) in s.tokens.iter_mut().zip(t.tag(&refs)) {
                tok.tag = Some(tag);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conllu::Token;
    use crate::gram::Rel;

    fn sent(words: &[(&str, Tag)]) -> Sentence {
        let tokens = words
            .iter()
            .map(|&(w, t)| Token { form: w.into(), lemma: w.to_lowercase(), tag: Some(t), upos: None, feats: Default::default(), head: 0, rel: Rel::Dep, space_after: true })
            .collect();
        Sentence { id: String::new(), text: String::new(), tokens, has_feats: false, has_lemmas: true, has_forms: true }
    }

    #[test]
    fn learns_context_and_guesses_unknown_words() {
        use Tag::*;
        let data: Vec<Sentence> = (0..30)
            .flat_map(|_| {
                [
                    sent(&[("The", DT), ("dog", NN), ("runs", VBZ), (".", Stop)]),
                    sent(&[("The", DT), ("dogs", NNS), ("run", VBP), (".", Stop)]),
                    sent(&[("They", PRP), ("run", VBP), ("home", RB), (".", Stop)]),
                ]
            })
            .collect();
        let t = PTagger::train(&data, 5);
        // "run" — VBP by context; unknown "cats" — NNS from suffix, dictionary and the TnT prior
        assert_eq!(t.tag(&["They", "run", "home", "."]), vec![PRP, VBP, RB, Stop]);
        assert_eq!(t.tag(&["The", "cats", "run", "."])[1], NNS);
        // negative control: training on wrong tags gives wrong tags
        let bad: Vec<Sentence> = (0..30).map(|_| sent(&[("The", NN), ("dog", DT), ("runs", Stop), (".", VBZ)])).collect();
        assert_ne!(PTagger::train(&bad, 5).tag(&["The", "dog", "runs", "."]), vec![DT, NN, VBZ, Stop]);
    }
}
