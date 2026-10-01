//! Part-of-speech tagger (PTB) — a trigram Markov model over classes (TnT, Brants 2000):
//! a dense transition table "class, class → class" (ln P with deleted interpolation smoothing),
//! a "word → classes" lexicon for known words and a suffix model for unknown ones (separately for capitalized and
//! lowercase). The best class chain — exact Viterbi over class pairs.

use crate::conllu::Sentence;
use crate::dict;
use crate::gram::Tag;
use crate::hash::FastMap;

/// Class in the tables: 0 — sentence boundary, 1 + tag number.
const BOS: usize = 0;
const NT: usize = Tag::N + 1;
const MAX_SUFFIX: usize = 10;
/// Words with frequency ≤ RARE train the suffix model.
const RARE: u32 = 10;

fn code(t: Tag) -> usize {
    t.idx() + 1
}

fn tag_of(c: usize) -> Tag {
    Tag::ALL[c - 1]
}

fn cap(w: &str) -> bool {
    w.chars().next().is_some_and(char::is_uppercase)
}

/// Suffix key (last `chars`, word case mark) — a number, no string.
fn suffix_key(cap: bool, chars: &[char]) -> u64 {
    let mut parts = Vec::with_capacity(chars.len() + 1);
    parts.push(cap as u64);
    parts.extend(chars.iter().map(|&c| c as u64));
    crate::hash::key(&parts)
}

fn add(v: &mut Vec<(u8, u32)>, c: usize) {
    match v.iter_mut().find(|(x, _)| *x as usize == c) {
        Some(e) => e.1 += 1,
        None => v.push((c as u8, 1)),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Tagger {
    /// ln P(c | a, b), index (a·NT + b)·NT + c
    trans: Vec<f64>,
    /// word → [(class, ln P(word | class))]
    lex: FastMap<String, Vec<(u8, f64)>>,
    /// suffix → [(class, count)]
    suffix: FastMap<u64, Vec<(u8, u32)>>,
    /// token count per class and total count (with sentence boundaries)
    #[serde(with = "crate::store::arr")]
    tag_count: [f64; NT],
    total: f64,
    theta: f64,
    /// hint from the built-in dictionary for unknown words
    pub dict: DictHint,
}

/// How the built-in dictionary (full AGID form expansion) helps with unknown words: it does not,
/// softly (dictionary classes × multiplier) or hard (only dictionary classes + capitalized proper nouns).
#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum DictHint {
    Off,
    Soft(f64),
    Hard,
}

impl Tagger {
    pub fn train(sents: &[Sentence]) -> Tagger {
        let mut uni = [0.0f64; NT];
        let mut bi = vec![0.0f64; NT * NT];
        let mut tri = vec![0.0f64; NT * NT * NT];
        let mut counts: FastMap<String, Vec<(u8, u32)>> = FastMap::default();
        let mut word_freq: FastMap<String, u32> = FastMap::default();
        let mut tag_count = [0.0f64; NT];
        for s in sents.iter().filter(|s| s.tagged()) {
            let mut seq = vec![BOS, BOS];
            for t in &s.tokens {
                let c = code(t.tag.expect("tagged"));
                seq.push(c);
                add(counts.entry(t.form.clone()).or_default(), c);
                *word_freq.entry(t.form.clone()).or_default() += 1;
                tag_count[c] += 1.0;
            }
            seq.push(BOS);
            for w in seq.windows(3) {
                uni[w[2]] += 1.0;
                bi[w[1] * NT + w[2]] += 1.0;
                tri[(w[0] * NT + w[1]) * NT + w[2]] += 1.0;
            }
        }
        let total: f64 = uni.iter().sum();
        let mut bi_ctx = [0.0f64; NT];
        let mut tri_ctx = vec![0.0f64; NT * NT];
        for a in 0..NT {
            for b in 0..NT {
                bi_ctx[a] += bi[a * NT + b];
                for c in 0..NT {
                    tri_ctx[a * NT + b] += tri[(a * NT + b) * NT + c];
                }
            }
        }
        // λ: deleted interpolation (Brants 2000)
        let mut l = [0.0f64; 3];
        for a in 0..NT {
            for b in 0..NT {
                for c in 0..NT {
                    let f = tri[(a * NT + b) * NT + c];
                    if f == 0.0 {
                        continue;
                    }
                    let ab = tri_ctx[a * NT + b];
                    let q3 = if ab > 1.0 { (f - 1.0) / (ab - 1.0) } else { 0.0 };
                    let q2 = if bi_ctx[b] > 1.0 { (bi[b * NT + c] - 1.0) / (bi_ctx[b] - 1.0) } else { 0.0 };
                    let q1 = (uni[c] - 1.0) / (total - 1.0);
                    if q3 >= q2 && q3 >= q1 {
                        l[2] += f;
                    } else if q2 >= q1 {
                        l[1] += f;
                    } else {
                        l[0] += f;
                    }
                }
            }
        }
        let s: f64 = l.iter().sum();
        let lambda = [l[0] / s, l[1] / s, l[2] / s];
        let mut trans = vec![f64::NEG_INFINITY; NT * NT * NT];
        for a in 0..NT {
            for b in 0..NT {
                let ab = tri_ctx[a * NT + b];
                for c in 0..NT {
                    let p1 = uni[c] / total;
                    let p2 = if bi_ctx[b] > 0.0 { bi[b * NT + c] / bi_ctx[b] } else { 0.0 };
                    let p3 = if ab > 0.0 { tri[(a * NT + b) * NT + c] / ab } else { 0.0 };
                    trans[(a * NT + b) * NT + c] = (lambda[0] * p1 + lambda[1] * p2 + lambda[2] * p3).ln();
                }
            }
        }
        // suffix model from rare words
        let mut suffix: FastMap<u64, Vec<(u8, u32)>> = FastMap::default();
        for (w, &f) in &word_freq {
            if f > RARE {
                continue;
            }
            let chars: Vec<char> = w.to_lowercase().chars().collect();
            for k in 0..=MAX_SUFFIX.min(chars.len()) {
                let e = suffix.entry(suffix_key(cap(w), &chars[chars.len() - k..])).or_default();
                for &(t, c) in &counts[w] {
                    for _ in 0..c {
                        add(e, t as usize);
                    }
                }
            }
        }
        // θ — standard deviation of unconditional probabilities of tags that occurred
        let p: Vec<f64> = (1..NT).filter(|&t| tag_count[t] > 0.0).map(|t| tag_count[t] / total).collect();
        let mean = p.iter().sum::<f64>() / p.len().max(1) as f64;
        let theta = (p.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (p.len().max(2) - 1) as f64).sqrt();
        let lex = counts
            .into_iter()
            .map(|(w, v)| {
                let e = v.iter().map(|&(t, c)| (t, (c as f64 / tag_count[t as usize]).ln())).collect();
                (w, e)
            })
            .collect();
        Tagger { trans, lex, suffix, tag_count, total, theta, dict: DictHint::Soft(100.0) }
    }

    /// How many tags occurred in training.
    pub fn known_tags(&self) -> usize {
        (1..NT).filter(|&t| self.tag_count[t] > 0.0).count()
    }

    #[inline]
    fn tr(&self, a: usize, b: usize, c: usize) -> f64 {
        self.trans[(a * NT + b) * NT + c]
    }

    /// ln P(word | class) for all candidate classes.
    fn emissions(&self, w: &str) -> Vec<(usize, f64)> {
        let known = self.lex.get(w).or_else(|| self.lex.get(&w.to_lowercase()));
        if let Some(v) = known {
            return v.iter().map(|&(t, e)| (t as usize, e)).collect();
        }
        // unknown word: Bayes, P(w|t) ∝ P(t|suf)/P(t)
        let p = self.unknown(w);
        (1..NT)
            .filter(|&t| p[t] > 0.0 && self.tag_count[t] > 0.0)
            .map(|t| (t, (p[t] / (self.tag_count[t] / self.total)).ln()))
            .collect()
    }

    /// Word classes with probabilities P(class | word), most probable first (for explanation); the first value —
    /// whether the word is known from training.
    pub fn word_classes(&self, w: &str) -> (bool, Vec<(Tag, f64)>) {
        let known = self.lex.get(w).or_else(|| self.lex.get(&w.to_lowercase()));
        let mut v: Vec<(Tag, f64)> = match known {
            Some(v) => v.iter().map(|&(t, e)| (tag_of(t as usize), e.exp() * self.tag_count[t as usize])).collect(),
            None => {
                let p = self.unknown(w);
                (1..NT).filter(|&t| p[t] > 0.0 && self.tag_count[t] > 0.0).map(|t| (tag_of(t), p[t])).collect()
            }
        };
        let s: f64 = v.iter().map(|x| x.1).sum();
        for x in &mut v {
            x.1 /= s.max(f64::MIN_POSITIVE);
        }
        v.sort_by(|a, b| b.1.total_cmp(&a.1));
        (known.is_some(), v)
    }

    /// Unknown word: P(t | suffix) with recursive smoothing and the built-in dictionary hint.
    fn unknown(&self, w: &str) -> [f64; NT] {
        let chars: Vec<char> = w.to_lowercase().chars().collect();
        let mut p = [0.0f64; NT];
        for t in 0..NT {
            p[t] = self.tag_count[t] / self.total;
        }
        for k in 1..=MAX_SUFFIX.min(chars.len()) {
            let Some(v) = self.suffix.get(&suffix_key(cap(w), &chars[chars.len() - k..])) else { break };
            let tot: f64 = v.iter().map(|&(_, c)| c as f64).sum();
            let mut q = [0.0f64; NT];
            for &(t, c) in v {
                q[t as usize] = c as f64 / tot;
            }
            for t in 0..NT {
                p[t] = (q[t] + self.theta * p[t]) / (1.0 + self.theta);
            }
        }
        if self.dict != DictHint::Off {
            if let Some(f) = dict::form(&w.to_lowercase()) {
                let mut allowed = [false; NT];
                for r in dict::analyses(f) {
                    allowed[code(r.tag)] = true;
                }
                if cap(w) {
                    allowed[code(Tag::NNP)] = true;
                    allowed[code(Tag::NNPS)] = true;
                }
                for t in 1..NT {
                    match self.dict {
                        DictHint::Soft(k) if allowed[t] => p[t] *= k,
                        DictHint::Hard if !allowed[t] => p[t] = 0.0,
                        _ => {}
                    }
                }
            }
        }
        p
    }

    /// Exact Viterbi over class pairs: state at word i — (class i−1, class i).
    pub fn tag(&self, words: &[&str]) -> Vec<Tag> {
        let n = words.len();
        if n == 0 {
            return Vec::new();
        }
        let em: Vec<Vec<(usize, f64)>> = words.iter().map(|w| self.emissions(w)).collect();
        let bos = vec![(BOS, 0.0)];
        // candidates for position i−1 (for i = 0 — sentence boundary)
        let cand = |i: isize| -> &Vec<(usize, f64)> { if i < 0 { &bos } else { &em[i as usize] } };
        // δ[i][j·K + k]: best score with class j at i−1 and k at i; back — class number at i−2
        let mut delta: Vec<Vec<f64>> = Vec::with_capacity(n);
        let mut back: Vec<Vec<u32>> = Vec::with_capacity(n);
        for i in 0..n as isize {
            let (pp, p, cur) = (cand(i - 2), cand(i - 1), cand(i));
            let (np, nc) = (p.len(), cur.len());
            let mut d = vec![f64::NEG_INFINITY; np * nc];
            let mut bk = vec![0u32; np * nc];
            for (j, &(pj, _)) in p.iter().enumerate() {
                for (k, &(ck, e)) in cur.iter().enumerate() {
                    let (mut best, mut arg) = (f64::NEG_INFINITY, 0u32);
                    for (h, &(ph, _)) in pp.iter().enumerate() {
                        let prev = if i == 0 { 0.0 } else { delta[i as usize - 1][h * np + j] };
                        let s = prev + self.tr(ph, pj, ck);
                        if s > best {
                            best = s;
                            arg = h as u32;
                        }
                    }
                    d[j * nc + k] = best + e;
                    bk[j * nc + k] = arg;
                }
            }
            delta.push(d);
            back.push(bk);
        }
        // end of sentence: transition into the boundary
        let (p, cur) = (cand(n as isize - 2), cand(n as isize - 1));
        let nc = cur.len();
        let (mut bj, mut bk, mut best) = (0usize, 0usize, f64::NEG_INFINITY);
        for (j, &(pj, _)) in p.iter().enumerate() {
            for (k, &(ck, _)) in cur.iter().enumerate() {
                let s = delta[n - 1][j * nc + k] + self.tr(pj, ck, BOS);
                if s > best {
                    best = s;
                    bj = j;
                    bk = k;
                }
            }
        }
        // backward pass
        let mut idx = vec![0usize; n];
        idx[n - 1] = bk;
        if n >= 2 {
            idx[n - 2] = bj;
        }
        for i in (2..n).rev() {
            let nc = cand(i as isize).len();
            idx[i - 2] = back[i][idx[i - 1] * nc + idx[i]] as usize;
        }
        (0..n).map(|i| tag_of(em[i][idx[i]].0)).collect()
    }
}
