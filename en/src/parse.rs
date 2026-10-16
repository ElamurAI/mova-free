//! Dependency parser — a table-driven transition automaton (arc-standard): state = stack + buffer,
//! commands SHIFT / LEFT(label) / RIGHT(label). Parsing uses a beam (8 hypotheses; training with early
//! update, Zhang & Clark 2008): EWT test LAS 79.00 → 79.77, dev 77.33 → 79.17, PUD 74.48 → 76.33. Decisions come from "feature → action weight" tables,
//! trained by an averaged perceptron with a static oracle on UD trees (Nivre 2004; Zhang & Nivre
//! 2011 — features simplified). Two-stage: action type (3 classes) and label (UD relation, arcs only) —
//! so the tables stay small. Features are numbers: template id + word and tag codes.

use crate::conllu::Sentence;
use crate::ctx::{Codes, Ctx, G_ENT};
use crate::gram::{Rel, Tag};
use crate::hash::{FastMap, key};

/// Automaton action.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    Shift,
    Left,
    Right,
}
const ACTS: [Act; 3] = [Act::Shift, Act::Left, Act::Right];

/// Position codes in features: none, root, unknown word; then tags / words.
const NONE: u64 = 0;
const ROOT: u64 = 1;
const UNK: u64 = 2;

fn tcode(t: Tag) -> u64 {
    2 + t as u64
}

/// Averaged perceptron weight table: feature → row of weights per class.
struct Table {
    k: usize,
    idx: FastMap<u64, u32>,
    w: Vec<f32>,
    u: Vec<f32>,
}

/// Table in the model file: rows in id order, weights stored sparsely (a label row mostly has
/// only a few nonzero classes out of k). The averaging accumulator `u` is only needed for training — not written.
#[derive(serde::Serialize, serde::Deserialize)]
struct TableFile {
    k: u32,
    /// row → feature
    #[serde(with = "crate::store::u64s")]
    keys: Vec<u64>,
    /// how many nonzero weights in each row
    #[serde(with = "crate::store::bytes")]
    nnz: Vec<u8>,
    /// class and weight of each nonzero one (bits as is: −0.0 is written too)
    #[serde(with = "crate::store::bytes")]
    cls: Vec<u8>,
    #[serde(with = "crate::store::f32s")]
    val: Vec<f32>,
}

impl serde::Serialize for Table {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.k > u8::MAX as usize || self.w.len() != self.idx.len() * self.k {
            return Err(serde::ser::Error::custom(format!("weight table: k = {}, weights {}, rows {}", self.k, self.w.len(), self.idx.len())));
        }
        let n = self.idx.len();
        let mut keys = vec![0u64; n];
        for (&f, &r) in &self.idx {
            keys[r as usize] = f;
        }
        let (mut nnz, mut cls, mut val) = (Vec::with_capacity(n), Vec::new(), Vec::new());
        for row in self.w.chunks_exact(self.k.max(1)) {
            let before = val.len();
            for (j, &x) in row.iter().enumerate() {
                if x.to_bits() != 0 {
                    cls.push(j as u8);
                    val.push(x);
                }
            }
            nnz.push((val.len() - before) as u8);
        }
        TableFile { k: self.k as u32, keys, nnz, cls, val }.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for Table {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let f = TableFile::deserialize(d)?;
        let (k, n) = (f.k as usize, f.keys.len());
        if f.nnz.len() != n || f.cls.len() != f.val.len() || f.nnz.iter().map(|&c| c as usize).sum::<usize>() != f.val.len() {
            return Err(D::Error::custom("weight table: lengths do not match"));
        }
        let mut idx: FastMap<u64, u32> = FastMap::with_capacity_and_hasher(n, Default::default());
        for (r, &key) in f.keys.iter().enumerate() {
            if idx.insert(key, r as u32).is_some() {
                return Err(D::Error::custom("weight table: feature twice"));
            }
        }
        let mut w = vec![0.0f32; n * k];
        let mut p = 0;
        for (r, &c) in f.nnz.iter().enumerate() {
            for _ in 0..c {
                let j = f.cls[p] as usize;
                if j >= k {
                    return Err(D::Error::custom(format!("weight table: class {j} with k = {k}")));
                }
                w[r * k + j] = f.val[p];
                p += 1;
            }
        }
        Ok(Table { k, idx, w, u: Vec::new() })
    }
}

impl Table {
    fn new(k: usize) -> Self {
        Table { k, idx: FastMap::default(), w: Vec::new(), u: Vec::new() }
    }
    fn row(&mut self, f: u64) -> usize {
        let k = self.k;
        let n = self.idx.len() as u32;
        let i = *self.idx.entry(f).or_insert(n);
        if i == n {
            self.w.extend(std::iter::repeat_n(0.0, k));
            self.u.extend(std::iter::repeat_n(0.0, k));
        }
        i as usize * k
    }
    fn scores(&self, feats: &[u64]) -> Vec<f32> {
        let mut s = vec![0.0f32; self.k];
        for f in feats {
            if let Some(&i) = self.idx.get(f) {
                let base = i as usize * self.k;
                for (j, x) in s.iter_mut().enumerate() {
                    *x += self.w[base + j];
                }
            }
        }
        s
    }
    fn update(&mut self, feats: &[u64], gold: usize, pred: usize, c: f32) {
        for &f in feats {
            let base = self.row(f);
            self.w[base + gold] += 1.0;
            self.u[base + gold] += c;
            self.w[base + pred] -= 1.0;
            self.u[base + pred] -= c;
        }
    }
    /// One update of class `k` by `d` (for the beam: +1 to the gold path, −1 to the wrong one).
    fn add(&mut self, feats: &[u64], k: usize, d: f32, c: f32) {
        for &f in feats {
            let base = self.row(f);
            self.w[base + k] += d;
            self.u[base + k] += d * c;
        }
    }
    /// Averaging: w ← w − u/c (Daumé's trick).
    fn average(&mut self, c: f32) {
        for (w, u) in self.w.iter_mut().zip(&self.u) {
            *w -= u / c;
        }
    }
}

#[derive(Clone)]
struct Config {
    stack: Vec<usize>,
    b: usize,
    n: usize,
    head: Vec<Option<usize>>,
    label: Vec<Rel>,
    /// leftmost/rightmost attached child and the number of left/right children
    lc: Vec<Option<usize>>,
    rc: Vec<Option<usize>>,
    nl: Vec<u8>,
    nr: Vec<u8>,
}

impl Config {
    fn new(n: usize) -> Self {
        Config {
            stack: vec![0],
            b: 1,
            n,
            head: vec![None; n + 1],
            label: vec![Rel::Dep; n + 1],
            lc: vec![None; n + 1],
            rc: vec![None; n + 1],
            nl: vec![0; n + 1],
            nr: vec![0; n + 1],
        }
    }
    fn terminal(&self) -> bool {
        self.b > self.n && self.stack.len() == 1
    }
    fn valid(&self, a: Act) -> bool {
        let s = self.stack.len();
        match a {
            Act::Shift => self.b <= self.n,
            Act::Left => s >= 2 && self.stack[s - 2] != 0,
            Act::Right => s >= 2 && (self.stack[s - 2] != 0 || self.b > self.n),
        }
    }
    fn attach(&mut self, h: usize, d: usize, l: Rel) {
        self.head[d] = Some(h);
        self.label[d] = l;
        if d < h {
            self.nl[h] = self.nl[h].saturating_add(1);
            if self.lc[h].is_none_or(|x| d < x) {
                self.lc[h] = Some(d);
            }
        } else {
            self.nr[h] = self.nr[h].saturating_add(1);
            if self.rc[h].is_none_or(|x| d > x) {
                self.rc[h] = Some(d);
            }
        }
    }
    /// Head and dependent of the arc that a LEFT or RIGHT action would create.
    fn arc(&self, a: Act) -> (usize, usize) {
        let k = self.stack.len();
        if a == Act::Left { (self.stack[k - 1], self.stack[k - 2]) } else { (self.stack[k - 2], self.stack[k - 1]) }
    }
    fn apply(&mut self, a: Act, l: Rel) {
        match a {
            Act::Shift => {
                self.stack.push(self.b);
                self.b += 1;
            }
            Act::Left => {
                let s0 = self.stack.pop().expect("s0");
                let s1 = self.stack.pop().expect("s1");
                self.attach(s0, s1, l);
                self.stack.push(s0);
            }
            Act::Right => {
                let s0 = self.stack.pop().expect("s0");
                let s1 = *self.stack.last().expect("s1");
                self.attach(s1, s0, l);
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Parser {
    types: Table,
    labeler: Table,
    /// relations from training in order of first appearance (the label is chosen only among them; on equal
    /// weights — the later one in this order, which fixes the perceptron trajectory from zero weights)
    labels: Vec<Rel>,
    /// word (lowercase, frequency ≥ 2) → id
    words: FastMap<String, u32>,
    c: f32,
    /// beam width at parse time (1 = greedy)
    pub beam: usize,
}

/// Beam hypothesis: automaton state, sum of action scores, path (action, label), whether it is the gold path.
#[derive(Clone)]
struct Item {
    c: Config,
    score: f32,
    path: Vec<(Act, Rel)>,
    gold: bool,
}

/// Parser input: word and tag codes; position 0 is the root. Document context (`crate::ctx`):
/// sentence-level codes and the entity code of each position (0 = root or none); without context — empty.
struct Sent {
    w: Vec<u64>,
    t: Vec<u64>,
    cx: Vec<(u64, u64)>,
    ent: Vec<u64>,
}

impl Sent {
    /// Transition context features: for each group × tag of stack top and × tag of buffer front;
    /// entity — the code of the position itself.
    fn ctx_feats(&self, s0: Option<usize>, b0: Option<usize>, out: &mut Vec<u64>) {
        let t = |x: Option<usize>| x.map_or(NONE, |i| self.t[i]);
        let e = |x: Option<usize>| x.map_or(0, |i| self.ent.get(i).copied().unwrap_or(0));
        for &(g, v) in &self.cx {
            out.push(key(&[400, g, v, t(s0)]));
            out.push(key(&[401, g, v, t(b0)]));
        }
        if !self.ent.is_empty() {
            out.push(key(&[400, G_ENT, e(s0), t(s0)]));
            out.push(key(&[401, G_ENT, e(b0), t(b0)]));
        }
    }

    /// Label context features: for each group × head tag and × dependent tag (with direction).
    fn ctx_label_feats(&self, head: usize, dep: usize, dir: u64, out: &mut Vec<u64>) {
        for &(g, v) in &self.cx {
            out.push(key(&[500, g, v, dir, self.t[head]]));
            out.push(key(&[501, g, v, dir, self.t[dep]]));
        }
        if !self.ent.is_empty() {
            out.push(key(&[500, G_ENT, self.ent[head], dir, self.t[head]]));
            out.push(key(&[501, G_ENT, self.ent[dep], dir, self.t[dep]]));
        }
    }
}

/// Parse step (for explanation): stack and buffer front before the action, weight sums of the three actions (SHIFT, LEFT,
/// RIGHT), chosen action, created arc.
#[derive(Clone, Debug)]
pub struct Step {
    pub stack: Vec<usize>,
    pub buffer: usize,
    pub scores: [f32; 3],
    pub act: Act,
    pub arc: Option<(usize, usize, Rel)>,
}

impl Parser {
    fn sent(&self, words: &[&str], tags: &[Tag], ctx: Option<&Ctx>) -> Sent {
        let mut w = vec![ROOT];
        w.extend(words.iter().map(|x| self.words.get(&x.to_lowercase()).map_or(UNK, |&i| 3 + i as u64)));
        let mut t = vec![ROOT];
        t.extend(tags.iter().map(|&x| tcode(x)));
        let cx = ctx.map_or_else(Codes::default, |c| Codes::of(c, words.len()));
        let ent = if cx.ent.is_empty() { Vec::new() } else { std::iter::once(0).chain(cx.ent.iter().copied()).collect() };
        Sent { w, t, cx: cx.sent, ent }
    }

    fn feats(s: &Sent, c: &Config) -> Vec<u64> {
        let st = |i: usize| c.stack.len().checked_sub(i + 1).map(|k| c.stack[k]);
        let bf = |i: usize| (c.b + i <= c.n).then_some(c.b + i);
        let w = |x: Option<usize>| x.map_or(NONE, |i| s.w[i]);
        let t = |x: Option<usize>| x.map_or(NONE, |i| s.t[i]);
        let lab = |x: Option<usize>| x.map_or(NONE, |i| 1 + c.label[i] as u64);
        let (s0, s1, s2, b0, b1, b2) = (st(0), st(1), st(2), bf(0), bf(1), bf(2));
        let lc = |x: Option<usize>| x.and_then(|i| c.lc[i]);
        let rc = |x: Option<usize>| x.and_then(|i| c.rc[i]);
        let dist = match (s0, s1) {
            (Some(a), Some(b)) => (a - b).min(5) as u64,
            _ => 0,
        };
        let val = |x: Option<usize>| x.map_or((0, 0), |i| (c.nl[i] as u64, c.nr[i] as u64));
        let (v0, v1) = (val(s0), val(s1));
        let mut f = vec![
            key(&[1, w(s0)]),
            key(&[2, t(s0)]),
            key(&[3, w(s0), t(s0)]),
            key(&[4, w(s1)]),
            key(&[5, t(s1)]),
            key(&[6, w(s1), t(s1)]),
            key(&[7, w(b0)]),
            key(&[8, t(b0)]),
            key(&[9, w(b0), t(b0)]),
            key(&[10, w(b1), t(b1)]),
            key(&[11, t(b2)]),
            key(&[12, t(s0), t(s1)]),
            key(&[13, w(s0), t(s1)]),
            key(&[14, t(s0), w(s1)]),
            key(&[15, w(s0), w(s1)]),
            key(&[16, t(s0), t(b0)]),
            key(&[17, w(s0), t(b0)]),
            key(&[18, t(s0), w(b0)]),
            key(&[19, t(s0), t(s1), t(b0)]),
            key(&[20, t(s0), t(s1), t(s2)]),
            key(&[21, t(s0), t(b0), t(b1)]),
            key(&[22, t(s1), t(s0), t(lc(s0))]),
            key(&[23, t(s1), t(s0), t(rc(s0))]),
            key(&[24, t(s1), t(s0), t(lc(s1))]),
            key(&[25, t(s1), t(s0), t(rc(s1))]),
            key(&[26, t(s0), lab(lc(s0)), lab(rc(s0))]),
            key(&[27, t(s1), lab(lc(s1)), lab(rc(s1))]),
            key(&[28, t(s0), t(s1), dist]),
            key(&[29, w(s0), t(s1), dist]),
            key(&[30, t(s0), v0.0, v0.1]),
            key(&[31, t(s1), v1.0, v1.1]),
            key(&[32, w(s0), v0.0, v0.1]),
            key(&[33, t(s0), t(s1), t(b0), t(b1)]),
        ];
        if !s.cx.is_empty() || !s.ent.is_empty() {
            s.ctx_feats(s0, b0, &mut f);
        }
        f
    }

    /// Arc label features: head, dependent, direction.
    fn label_feats(s: &Sent, c: &Config, head: usize, dep: usize) -> Vec<u64> {
        let dir = (dep < head) as u64;
        let d = head.abs_diff(dep).min(5) as u64;
        let (hw, ht, dw, dt) = (s.w[head], s.t[head], s.w[dep], s.t[dep]);
        let lab = |x: Option<usize>| x.map_or(NONE, |i| 1 + c.label[i] as u64);
        let mut f = vec![
            key(&[101, dir, ht]),
            key(&[102, dir, dt]),
            key(&[103, dir, ht, dt]),
            key(&[104, dir, hw]),
            key(&[105, dir, dw]),
            key(&[106, dir, hw, dt]),
            key(&[107, dir, ht, dw]),
            key(&[108, dir, ht, dt, d]),
            key(&[109, dir, dw, dt]),
            key(&[110, dir, dt, lab(c.lc[dep]), lab(c.rc[dep])]),
            key(&[111, dir, ht, dt, c.nl[head] as u64, c.nr[head] as u64]),
            key(&[112, dir, hw, dw]),
        ];
        if !s.cx.is_empty() || !s.ent.is_empty() {
            s.ctx_label_feats(head, dep, dir, &mut f);
        }
        f
    }

    /// Static arc-standard oracle: (action, label) or None if the tree is non-projective here.
    fn oracle(c: &Config, gold_head: &[usize], gold_label: &[Rel], gold_kids: &[u16], attached: &[u16]) -> Option<(Act, Rel)> {
        let n = c.stack.len();
        if n >= 2 {
            let (s0, s1) = (c.stack[n - 1], c.stack[n - 2]);
            if s1 != 0 && gold_head[s1] == s0 {
                return Some((Act::Left, gold_label[s1]));
            }
            if gold_head[s0] == s1 && attached[s0] == gold_kids[s0] && (s1 != 0 || c.b > c.n) {
                return Some((Act::Right, gold_label[s0]));
            }
        }
        (c.b <= c.n).then_some((Act::Shift, Rel::Dep))
    }

    fn best_act(&self, f: &[u64], c: &Config) -> Option<Act> {
        let sc = self.types.scores(f);
        ACTS.into_iter().filter(|&a| c.valid(a)).max_by(|&a, &b| sc[a as usize].total_cmp(&sc[b as usize]))
    }

    fn best_label(&self, lf: &[u64]) -> Rel {
        let ls = self.labeler.scores(lf);
        self.labels.iter().copied().max_by(|a, b| ls[a.idx()].total_cmp(&ls[b.idx()])).unwrap_or(Rel::Dep)
    }

    pub fn train(sents: &[Sentence], epochs: usize) -> Parser {
        let mut p = Parser { types: Table::new(3), labeler: Table::new(Rel::N), labels: Vec::new(), words: FastMap::default(), c: 1.0, beam: 1 };
        let sents: Vec<&Sentence> = sents.iter().filter(|s| s.tagged()).collect();
        let mut freq: FastMap<String, u32> = FastMap::default();
        for s in &sents {
            for t in &s.tokens {
                *freq.entry(t.form.to_lowercase()).or_default() += 1;
                if !p.labels.contains(&t.rel) {
                    p.labels.push(t.rel);
                }
            }
        }
        let mut known: Vec<String> = freq.into_iter().filter(|&(_, c)| c >= 2).map(|(w, _)| w).collect();
        known.sort_unstable();
        p.words = known.into_iter().enumerate().map(|(i, w)| (w, i as u32)).collect();
        let mut order: Vec<usize> = (0..sents.len()).collect();
        let mut rng = crate::ctx::seeded(0x9E3779B97F4A7C15u64);
        for epoch in 0..epochs {
            // deterministic shuffle (xorshift) — reproducible training
            for i in (1..order.len()).rev() {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                order.swap(i, (rng % (i as u64 + 1)) as usize);
            }
            let (mut ok, mut tot) = (0usize, 0usize);
            for &si in &order {
                let s = sents[si];
                let n = s.tokens.len();
                let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
                let tags: Vec<Tag> = s.tokens.iter().map(|t| t.tag.expect("tagged")).collect();
                let sent = p.sent(&words, &tags, None);
                let mut gh = vec![0usize; n + 1];
                let mut gl = vec![Rel::Dep; n + 1];
                let mut kids = vec![0u16; n + 1];
                for (i, t) in s.tokens.iter().enumerate() {
                    gh[i + 1] = t.head;
                    gl[i + 1] = t.rel;
                    kids[t.head] += 1;
                }
                let mut attached = vec![0u16; n + 1];
                let mut c = Config::new(n);
                while !c.terminal() {
                    let Some((ga, glab)) = Self::oracle(&c, &gh, &gl, &kids, &attached) else { break };
                    let f = Self::feats(&sent, &c);
                    let pa = p.best_act(&f, &c).unwrap_or(ga);
                    tot += 1;
                    if pa == ga {
                        ok += 1;
                    } else {
                        p.types.update(&f, ga as usize, pa as usize, p.c);
                    }
                    if ga != Act::Shift {
                        let (hd, dp) = c.arc(ga);
                        let lf = Self::label_feats(&sent, &c, hd, dp);
                        let pl = p.best_label(&lf);
                        if pl != glab {
                            p.labeler.update(&lf, glab.idx(), pl.idx(), p.c);
                        }
                        attached[hd] += 1;
                    }
                    c.apply(ga, glab);
                    p.c += 1.0;
                }
            }
            eprintln!("  epoch {}: actions {:.2}%", epoch + 1, 100.0 * ok as f64 / tot.max(1) as f64);
        }
        p.types.average(p.c);
        p.labeler.average(p.c);
        p
    }

    /// Beam training (Zhang & Clark 2008): path score = sum of action scores, updates are
    /// early (gold path fell out of the beam: +1 to its prefix, −1 to the best) and at the end (best
    /// full path is not gold). Labels are learned greedily on the gold path, as before.
    pub fn train_beam(sents: &[Sentence], epochs: usize, width: usize) -> Parser {
        Self::train_beam_ctx(sents, None, epochs, width)
    }

    /// Same with document context: `ctx` is the context of each sentence in `sents` (same order).
    /// `None` — identical to `train_beam`, to the bit.
    pub fn train_beam_ctx(sents: &[Sentence], ctx: Option<&[Ctx]>, epochs: usize, width: usize) -> Parser {
        if let Some(c) = ctx {
            assert_eq!(c.len(), sents.len(), "context for every sentence");
        }
        let mut p = Parser { types: Table::new(3), labeler: Table::new(Rel::N), labels: Vec::new(), words: FastMap::default(), c: 1.0, beam: width };
        let ctxs: Vec<Option<&Ctx>> = (0..sents.len()).filter(|&k| sents[k].tagged()).map(|k| ctx.map(|c| &c[k])).collect();
        let sents: Vec<&Sentence> = sents.iter().filter(|s| s.tagged()).collect();
        let mut freq: FastMap<String, u32> = FastMap::default();
        for s in &sents {
            for t in &s.tokens {
                *freq.entry(t.form.to_lowercase()).or_default() += 1;
                if !p.labels.contains(&t.rel) {
                    p.labels.push(t.rel);
                }
            }
        }
        let mut known: Vec<String> = freq.into_iter().filter(|&(_, c)| c >= 2).map(|(w, _)| w).collect();
        known.sort_unstable();
        p.words = known.into_iter().enumerate().map(|(i, w)| (w, i as u32)).collect();
        let mut order: Vec<usize> = (0..sents.len()).collect();
        let mut rng = crate::ctx::seeded(0x9E3779B97F4A7C15u64);
        for epoch in 0..epochs {
            for i in (1..order.len()).rev() {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                order.swap(i, (rng % (i as u64 + 1)) as usize);
            }
            let (mut early, mut full, mut ok) = (0usize, 0usize, 0usize);
            for &si in &order {
                let s = sents[si];
                let n = s.tokens.len();
                let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
                let tags: Vec<Tag> = s.tokens.iter().map(|t| t.tag.expect("tagged")).collect();
                let sent = p.sent(&words, &tags, ctxs[si]);
                let mut gh = vec![0usize; n + 1];
                let mut gl = vec![Rel::Dep; n + 1];
                let mut kids = vec![0u16; n + 1];
                for (i, t) in s.tokens.iter().enumerate() {
                    gh[i + 1] = t.head;
                    gl[i + 1] = t.rel;
                    kids[t.head] += 1;
                }
                // gold path (projective trees only) + labels greedily on it
                let mut gold: Vec<(Act, Rel)> = Vec::with_capacity(2 * n);
                let mut attached = vec![0u16; n + 1];
                let mut c = Config::new(n);
                let mut proj = true;
                while !c.terminal() {
                    let Some((ga, glab)) = Self::oracle(&c, &gh, &gl, &kids, &attached) else {
                        proj = false;
                        break;
                    };
                    if ga != Act::Shift {
                        let (hd, dp) = c.arc(ga);
                        let lf = Self::label_feats(&sent, &c, hd, dp);
                        let pl = p.best_label(&lf);
                        if pl != glab {
                            p.labeler.update(&lf, glab.idx(), pl.idx(), p.c);
                        }
                        attached[hd] += 1;
                    }
                    gold.push((ga, glab));
                    c.apply(ga, glab);
                }
                if !proj {
                    continue;
                }
                let mut beam = vec![Item { c: Config::new(n), score: 0.0, path: Vec::new(), gold: true }];
                let mut violated = false;
                for (t, &(ga, glab)) in gold.iter().enumerate() {
                    beam = p.step(&sent, &beam, Some((ga, glab)));
                    p.c += 1.0;
                    if !beam.iter().any(|it| it.gold) {
                        p.update_paths(&sent, n, &gold[..=t], &beam[0].path);
                        violated = true;
                        early += 1;
                        break;
                    }
                }
                if !violated {
                    if beam[0].gold {
                        ok += 1;
                    } else {
                        let pred = beam[0].path.clone();
                        p.update_paths(&sent, n, &gold, &pred);
                        full += 1;
                    }
                }
            }
            eprintln!("  epoch {}: beam {width}, error-free sentences {ok}, early updates {early}, full {full}", epoch + 1);
        }
        p.types.average(p.c);
        p.labeler.average(p.c);
        p
    }

    /// Beam step: each hypothesis × each allowed action, the best `beam` remain. `gold` is the
    /// gold-path action at this step (during training): the gold hypothesis takes the gold label.
    fn step(&self, sent: &Sent, beam: &[Item], gold: Option<(Act, Rel)>) -> Vec<Item> {
        self.step_w(sent, beam, gold, self.beam)
    }

    fn step_w(&self, sent: &Sent, beam: &[Item], gold: Option<(Act, Rel)>, width: usize) -> Vec<Item> {
        let mut cand: Vec<(f32, usize, Act)> = Vec::with_capacity(beam.len() * 3);
        for (bi, it) in beam.iter().enumerate() {
            if it.c.terminal() {
                continue;
            }
            let sc = self.types.scores(&Self::feats(sent, &it.c));
            for a in ACTS {
                if it.c.valid(a) {
                    cand.push((it.score + sc[a as usize], bi, a));
                }
            }
        }
        // on ties the gold hypothesis ranks lower (so a tie does not hide an error)
        let is_gold = |bi: usize, a: Act| beam[bi].gold && gold.is_some_and(|(ga, _)| ga == a);
        cand.sort_by(|x, y| y.0.total_cmp(&x.0).then(is_gold(x.1, x.2).cmp(&is_gold(y.1, y.2))));
        cand.truncate(width.max(1));
        cand.into_iter()
            .map(|(score, bi, a)| {
                let src = &beam[bi];
                let is_gold = src.gold && gold.is_some_and(|(ga, _)| ga == a);
                let l = match (a, is_gold, gold) {
                    (Act::Shift, _, _) => Rel::Dep,
                    (_, true, Some((_, gl))) => gl,
                    _ => {
                        let (hd, dp) = src.c.arc(a);
                        self.best_label(&Self::label_feats(sent, &src.c, hd, dp))
                    }
                };
                let mut c = src.c.clone();
                c.apply(a, l);
                let mut path = src.path.clone();
                path.push((a, l));
                Item { c, score, path, gold: is_gold }
            })
            .collect()
    }

    /// Path update: features of each gold-path step get +1 for its action, the wrong path's get −1.
    fn update_paths(&mut self, sent: &Sent, n: usize, gold: &[(Act, Rel)], pred: &[(Act, Rel)]) {
        for (path, d) in [(gold, 1.0f32), (pred, -1.0)] {
            let mut c = Config::new(n);
            for &(a, l) in path {
                let f = Self::feats(sent, &c);
                self.types.add(&f, a as usize, d, self.c);
                c.apply(a, l);
            }
        }
    }

    /// Parse: (head, relation) for each word 1..n.
    pub fn parse(&self, words: &[&str], tags: &[Tag]) -> Vec<(usize, Rel)> {
        if self.beam <= 1 {
            return self.run(&self.sent(words, tags, None), words.len(), None);
        }
        self.parse_steps(words, tags).0
    }

    /// Parse with document context (`None` — same as `parse`).
    pub fn parse_ctx(&self, words: &[&str], tags: &[Tag], ctx: Option<&Ctx>) -> Vec<(usize, Rel)> {
        let sent = self.sent(words, tags, ctx);
        let n = words.len();
        if self.beam <= 1 {
            return self.run(&sent, n, None);
        }
        let path = self.beam_path(&sent, n);
        let mut c = Config::new(n);
        for (a, l) in path {
            c.apply(a, l);
        }
        (1..=n).map(|i| (c.head[i].unwrap_or(0), c.label[i])).collect()
    }

    /// Parse with a log of automaton steps (for explanation). With a beam — the steps of the winning path.
    pub fn parse_steps(&self, words: &[&str], tags: &[Tag]) -> (Vec<(usize, Rel)>, Vec<Step>) {
        let mut steps = Vec::new();
        if self.beam <= 1 {
            let tree = self.run(&self.sent(words, tags, None), words.len(), Some(&mut steps));
            return (tree, steps);
        }
        let sent = self.sent(words, tags, None);
        let path = self.beam_path(&sent, words.len());
        let mut c = Config::new(words.len());
        for (a, l) in path {
            let sc = self.types.scores(&Self::feats(&sent, &c));
            let arc = (a != Act::Shift).then(|| {
                let (hd, dp) = c.arc(a);
                (hd, dp, l)
            });
            steps.push(Step { stack: c.stack.clone(), buffer: c.b, scores: [sc[0], sc[1], sc[2]], act: a, arc });
            c.apply(a, l);
        }
        ((1..=words.len()).map(|i| (c.head[i].unwrap_or(0), c.label[i])).collect(), steps)
    }

    /// The `k` best distinct trees of a beam of width `k` (best first) with their path scores — candidates for
    /// reranking with knowledge the parser does not have (e.g. the absurdity matrix).
    pub fn parse_kbest(&self, words: &[&str], tags: &[Tag], k: usize) -> Vec<(f32, Vec<(usize, Rel)>)> {
        let n = words.len();
        let sent = self.sent(words, tags, None);
        let mut beam = vec![Item { c: Config::new(n), score: 0.0, path: Vec::new(), gold: false }];
        while beam.iter().any(|it| !it.c.terminal()) {
            let next = self.step_w(&sent, &beam, None, k);
            if next.is_empty() {
                break;
            }
            beam = next;
        }
        let mut out: Vec<(f32, Vec<(usize, Rel)>)> = Vec::new();
        for it in beam {
            let t: Vec<(usize, Rel)> = (1..=n).map(|i| (it.c.head[i].unwrap_or(0), it.c.label[i])).collect();
            if !out.iter().any(|(_, o)| *o == t) {
                out.push((it.score, t));
            }
        }
        out
    }

    /// Path (action, label) of the best beam hypothesis.

    fn beam_path(&self, sent: &Sent, n: usize) -> Vec<(Act, Rel)> {
        let mut beam = vec![Item { c: Config::new(n), score: 0.0, path: Vec::new(), gold: false }];
        while beam.iter().any(|it| !it.c.terminal()) {
            let next = self.step(sent, &beam, None);
            if next.is_empty() {
                break;
            }
            beam = next;
        }
        beam.swap_remove(0).path
    }

    fn run(&self, sent: &Sent, n: usize, mut log: Option<&mut Vec<Step>>) -> Vec<(usize, Rel)> {
        let mut c = Config::new(n);
        while !c.terminal() {
            let f = Self::feats(&sent, &c);
            let sc = self.types.scores(&f);
            let Some(a) = ACTS.into_iter().filter(|&a| c.valid(a)).max_by(|&a, &b| sc[a as usize].total_cmp(&sc[b as usize])) else { break };
            let mut l = Rel::Dep;
            let mut arc = None;
            if a != Act::Shift {
                let (hd, dp) = c.arc(a);
                l = self.best_label(&Self::label_feats(&sent, &c, hd, dp));
                arc = Some((hd, dp, l));
            }
            if let Some(log) = log.as_deref_mut() {
                log.push(Step { stack: c.stack.clone(), buffer: c.b, scores: [sc[0], sc[1], sc[2]], act: a, arc });
            }
            c.apply(a, l);
        }
        (1..=n).map(|i| (c.head[i].unwrap_or(0), c.label[i])).collect()
    }
}
