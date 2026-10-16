//! Coreference scorer: MUC (Vilain et al. 1995), B³ (Bagga & Baldwin 1998), CEAF-e and CEAF-m (Luo 2005),
//! CoNLL F1 — the mean of MUC, B³, CEAF-e. Formulas for predicted mentions without mention manipulation
//! follow Pradhan et al. 2014 (papers/p/pr/pradhan-2014-scoring-coreference-partitions-predicted-mentions, P14-2006):
//! a mention absent from the other partition simply contributes nothing.
//!
//! Mentions are matched by head (head match), as in the main CRAC 2023–2026 metric: gold and output
//! mentions match if they share the head; several mentions with the same head are told apart by span
//! (exact span first, then largest overlap). Heads come from `corefud::span_head` on each file's own tree.
//! The total over several documents is the sum of numerators and denominators (micro), as in the reference scorer.

use std::collections::HashMap;

use crate::corefud::Span;

/// Recall and precision numerators and denominators of one metric.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Counts {
    pub rn: f64,
    pub rd: f64,
    pub pn: f64,
    pub pd: f64,
}

impl Counts {
    pub fn add(&mut self, o: Counts) {
        self.rn += o.rn;
        self.rd += o.rd;
        self.pn += o.pn;
        self.pd += o.pd;
    }
    pub fn r(&self) -> f64 {
        if self.rd == 0.0 { 0.0 } else { self.rn / self.rd }
    }
    pub fn p(&self) -> f64 {
        if self.pd == 0.0 { 0.0 } else { self.pn / self.pd }
    }
    pub fn f(&self) -> f64 {
        let (r, p) = (self.r(), self.p());
        if r + p == 0.0 { 0.0 } else { 2.0 * r * p / (r + p) }
    }
}

/// Partition: an entity is a list of mention indices.
pub type Part = Vec<Vec<usize>>;

fn owner(p: &Part) -> HashMap<usize, usize> {
    let mut m = HashMap::new();
    for (e, ms) in p.iter().enumerate() {
        for &x in ms {
            m.insert(x, e);
        }
    }
    m
}

/// One side of MUC: Σ(|K| − |p(K)|) / Σ(|K| − 1); p(K) — the parts of K by entities of the other partition,
/// an unmatched mention is a separate part.
fn muc_side(k: &Part, r: &Part) -> (f64, f64) {
    let own = owner(r);
    let (mut num, mut den) = (0.0, 0.0);
    for e in k {
        if e.is_empty() {
            continue;
        }
        let mut parts: Vec<usize> = Vec::new();
        let mut lone = 0usize;
        for x in e {
            match own.get(x) {
                Some(&j) => {
                    if !parts.contains(&j) {
                        parts.push(j);
                    }
                }
                None => lone += 1,
            }
        }
        num += (e.len() - parts.len() - lone) as f64;
        den += (e.len() - 1) as f64;
    }
    (num, den)
}

pub fn muc(key: &Part, resp: &Part) -> Counts {
    let (rn, rd) = muc_side(key, resp);
    let (pn, pd) = muc_side(resp, key);
    Counts { rn, rd, pn, pd }
}

fn b3_side(k: &Part, r: &Part) -> (f64, f64) {
    let own = owner(r);
    let (mut num, mut den) = (0.0, 0.0);
    for e in k {
        if e.is_empty() {
            continue;
        }
        let mut inter: HashMap<usize, usize> = HashMap::new();
        for x in e {
            if let Some(&j) = own.get(x) {
                *inter.entry(j).or_default() += 1;
            }
        }
        num += inter.values().map(|&c| (c * c) as f64).sum::<f64>() / e.len() as f64;
        den += e.len() as f64;
    }
    (num, den)
}

pub fn b3(key: &Part, resp: &Part) -> Counts {
    let (rn, rd) = b3_side(key, resp);
    let (pn, pd) = b3_side(resp, key);
    Counts { rn, rd, pn, pd }
}

/// Entity overlaps: the matrix |K_i ∩ R_j|.
fn overlaps(key: &Part, resp: &Part) -> Vec<Vec<usize>> {
    let own = owner(resp);
    let mut m = vec![vec![0usize; resp.len()]; key.len()];
    for (i, e) in key.iter().enumerate() {
        for x in e {
            if let Some(&j) = own.get(x) {
                m[i][j] += 1;
            }
        }
    }
    m
}

/// Best assignment of rows to columns (Kuhn–Munkres), maximizing the weight sum; rows ≤ columns.
pub fn assign_max(w: &[Vec<f64>]) -> f64 {
    let n = w.len();
    if n == 0 {
        return 0.0;
    }
    let m = w[0].len();
    if m == 0 {
        return 0.0;
    }
    if n > m {
        let t: Vec<Vec<f64>> = (0..m).map(|j| (0..n).map(|i| w[i][j]).collect()).collect();
        return assign_max(&t);
    }
    let inf = f64::INFINITY;
    let (mut u, mut v) = (vec![0.0f64; n + 1], vec![0.0f64; m + 1]);
    let (mut p, mut way) = (vec![0usize; m + 1], vec![0usize; m + 1]);
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0usize;
        let mut minv = vec![inf; m + 1];
        let mut used = vec![false; m + 1];
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = inf;
            let mut j1 = 0usize;
            for j in 1..=m {
                if !used[j] {
                    let cur = -w[i0 - 1][j - 1] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=m {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }
    (1..=m).filter(|&j| p[j] != 0).map(|j| w[p[j] - 1][j - 1]).sum()
}

/// Best assignment over entities with non-zero overlap only (each component of the bipartite graph
/// separately): the same number as the full matrix, but faster.
fn best_alignment(ov: &[Vec<usize>], sim: impl Fn(usize, usize) -> f64) -> f64 {
    let (n, m) = (ov.len(), ov.first().map_or(0, |r| r.len()));
    // components: rows and columns linked by non-zero overlap
    let mut seen_r = vec![false; n];
    let mut seen_c = vec![false; m];
    let mut total = 0.0;
    for start in 0..n {
        if seen_r[start] || ov[start].iter().all(|&x| x == 0) {
            continue;
        }
        let (mut rows, mut cols) = (vec![start], Vec::new());
        seen_r[start] = true;
        let mut stack = vec![(true, start)];
        while let Some((is_row, x)) = stack.pop() {
            if is_row {
                for j in 0..m {
                    if ov[x][j] > 0 && !seen_c[j] {
                        seen_c[j] = true;
                        cols.push(j);
                        stack.push((false, j));
                    }
                }
            } else {
                for i in 0..n {
                    if ov[i][x] > 0 && !seen_r[i] {
                        seen_r[i] = true;
                        rows.push(i);
                        stack.push((true, i));
                    }
                }
            }
        }
        let w: Vec<Vec<f64>> = rows.iter().map(|&i| cols.iter().map(|&j| if ov[i][j] > 0 { sim(i, j) } else { 0.0 }).collect()).collect();
        total += assign_max(&w);
    }
    total
}

/// CEAF-e: φ4(K, R) = 2|K∩R| / (|K| + |R|); R = Σφ / |K entities|, P = Σφ / |R entities|.
pub fn ceafe(key: &Part, resp: &Part) -> Counts {
    let key: Part = key.iter().filter(|e| !e.is_empty()).cloned().collect();
    let resp: Part = resp.iter().filter(|e| !e.is_empty()).cloned().collect();
    let ov = overlaps(&key, &resp);
    let s = best_alignment(&ov, |i, j| 2.0 * ov[i][j] as f64 / (key[i].len() + resp[j].len()) as f64);
    Counts { rn: s, rd: key.len() as f64, pn: s, pd: resp.len() as f64 }
}

/// CEAF-m: φ3(K, R) = |K∩R|; R = Σφ / Σ|K|, P = Σφ / Σ|R|.
pub fn ceafm(key: &Part, resp: &Part) -> Counts {
    let ov = overlaps(key, resp);
    let s = best_alignment(&ov, |i, j| ov[i][j] as f64);
    let nk: usize = key.iter().map(Vec::len).sum();
    let nr: usize = resp.iter().map(Vec::len).sum();
    Counts { rn: s, rd: nk as f64, pn: s, pd: nr as f64 }
}

/// All metrics of one measurement.
#[derive(Clone, Copy, Debug, Default)]
pub struct Scores {
    pub muc: Counts,
    pub b3: Counts,
    pub ceafe: Counts,
    pub ceafm: Counts,
    /// Mentions: numerator is matches, denominators are gold and output.
    pub ment: Counts,
}

impl Scores {
    pub fn add(&mut self, o: &Scores) {
        self.muc.add(o.muc);
        self.b3.add(o.b3);
        self.ceafe.add(o.ceafe);
        self.ceafm.add(o.ceafm);
        self.ment.add(o.ment);
    }
    /// CoNLL F1 — the mean F1 of MUC, B³ and CEAF-e.
    pub fn conll(&self) -> f64 {
        (self.muc.f() + self.b3.f() + self.ceafe.f()) / 3.0
    }
}

pub fn score_parts(key: &Part, resp: &Part) -> Scores {
    Scores { muc: muc(key, resp), b3: b3(key, resp), ceafe: ceafe(key, resp), ceafm: ceafm(key, resp), ment: Counts::default() }
}

/// A mention for matching: span and head.
#[derive(Clone, Copy, Debug)]
pub struct MSpan {
    pub span: Span,
    pub head: usize,
}

/// How to match mentions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Matching {
    /// By head (CRAC); equal heads are resolved by span.
    Head,
    /// Exact span.
    Exact,
}

fn overlap(a: Span, b: Span) -> usize {
    if a.sent != b.sent {
        return 0;
    }
    let (s, e) = (a.start.max(b.start), a.end.min(b.end));
    if s > e { 0 } else { e - s + 1 }
}

/// For each output mention, the index of the gold mention or `None`.
pub fn match_mentions(gold: &[MSpan], pred: &[MSpan], how: Matching) -> Vec<Option<usize>> {
    let mut out = vec![None; pred.len()];
    let key = |m: &MSpan| match how {
        Matching::Head => (m.span.sent, m.head, 0, 0),
        Matching::Exact => (m.span.sent, m.span.start, m.span.end, 0),
    };
    let mut gi: HashMap<(usize, usize, usize, usize), Vec<usize>> = HashMap::new();
    for (i, g) in gold.iter().enumerate() {
        gi.entry(key(g)).or_default().push(i);
    }
    let mut pi: HashMap<(usize, usize, usize, usize), Vec<usize>> = HashMap::new();
    for (j, p) in pred.iter().enumerate() {
        pi.entry(key(p)).or_default().push(j);
    }
    for (k, ps) in &pi {
        let Some(gs) = gi.get(k) else { continue };
        let mut gfree: Vec<usize> = gs.clone();
        let mut pfree: Vec<usize> = ps.clone();
        // exact spans first
        pfree.retain(|&j| {
            if let Some(pos) = gfree.iter().position(|&i| gold[i].span == pred[j].span) {
                out[j] = Some(gfree.remove(pos));
                false
            } else {
                true
            }
        });
        // then the largest overlap (greedy, stable by order)
        while !pfree.is_empty() && !gfree.is_empty() {
            let mut best = (0usize, 0usize, 0usize);
            let mut found = false;
            for (a, &j) in pfree.iter().enumerate() {
                for (b, &i) in gfree.iter().enumerate() {
                    let o = overlap(gold[i].span, pred[j].span);
                    if !found || o > best.0 {
                        best = (o, a, b);
                        found = true;
                    }
                }
            }
            let (_, a, b) = best;
            out[pfree.remove(a)] = Some(gfree.remove(b));
        }
    }
    out
}

/// Score one document: gold and output entities as lists of mentions.
pub fn score_doc(gold: &[Vec<MSpan>], pred: &[Vec<MSpan>], how: Matching, singletons: bool) -> Scores {
    let gold: Vec<&Vec<MSpan>> = gold.iter().filter(|e| singletons || e.len() > 1).collect();
    let pred: Vec<&Vec<MSpan>> = pred.iter().filter(|e| singletons || e.len() > 1).collect();
    let gm: Vec<MSpan> = gold.iter().flat_map(|e| e.iter().copied()).collect();
    let pm: Vec<MSpan> = pred.iter().flat_map(|e| e.iter().copied()).collect();
    let mt = match_mentions(&gm, &pm, how);
    let mut key: Part = Vec::new();
    let mut n = 0usize;
    for e in &gold {
        key.push((n..n + e.len()).collect());
        n += e.len();
    }
    let mut resp: Part = Vec::new();
    let mut spurious = gm.len();
    let mut j = 0usize;
    for e in &pred {
        let mut ids = Vec::new();
        for _ in 0..e.len() {
            match mt[j] {
                Some(i) => ids.push(i),
                None => {
                    ids.push(spurious);
                    spurious += 1;
                }
            }
            j += 1;
        }
        resp.push(ids);
    }
    let mut s = score_parts(&key, &resp);
    let hit = mt.iter().filter(|x| x.is_some()).count() as f64;
    s.ment = Counts { rn: hit, rd: gm.len() as f64, pn: hit, pd: pm.len() as f64 };
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    /// Pradhan et al. 2014, §4: K = {a,b,c} {d,e,f,g}; R = {a,b} {c,d} {f,g,h,i}.
    #[test]
    fn pradhan_2014_example() {
        let (a, b, c, d, e, f, g, h, i) = (0, 1, 2, 3, 4, 5, 6, 7, 8);
        let key: Part = vec![vec![a, b, c], vec![d, e, f, g]];
        let resp: Part = vec![vec![a, b], vec![c, d], vec![f, g, h, i]];
        let m = muc(&key, &resp);
        assert!(close(m.r(), 0.4) && close(m.p(), 0.4) && close(m.f(), 0.4), "MUC {m:?}");
        let x = b3(&key, &resp);
        assert!(close(x.r(), 35.0 / 84.0) && close(x.p(), 0.5), "B3 {x:?}");
        // the paper's F1 "0.46" comes from rounded 0.42 and 0.50; the exact value is 0.4545…
        assert!(close(x.f(), 2.0 * (35.0 / 84.0) * 0.5 / (35.0 / 84.0 + 0.5)));
        assert!((x.f() - 0.4545).abs() < 1e-4);
        let ce = ceafe(&key, &resp);
        assert!(close(ce.r(), 0.65) && close(ce.p(), 1.3 / 3.0), "CEAF-e {ce:?}");
        assert!((ce.f() - 0.52).abs() < 0.005);
        let cm = ceafm(&key, &resp);
        assert!(close(cm.r(), 4.0 / 7.0) && close(cm.p(), 0.5), "CEAF-m {cm:?}");
        assert!((cm.f() - 0.53).abs() < 0.005);
    }

    /// Identity gives 100 everywhere; negative controls: one giant cluster, and all singletons.
    #[test]
    fn identity_and_negative_controls() {
        let key: Part = vec![vec![0, 1, 2], vec![3, 4], vec![5], vec![6, 7, 8, 9]];
        let s = score_parts(&key, &key);
        for c in [s.muc, s.b3, s.ceafe, s.ceafm] {
            assert!(close(c.f(), 1.0), "{c:?}");
        }
        // one giant cluster: MUC recall 1 but low precision; B³ and CEAF-e are low
        let giant: Part = vec![(0..10).collect()];
        let g = score_parts(&key, &giant);
        assert!(close(g.muc.r(), 1.0));
        assert!(close(g.muc.p(), 6.0 / 9.0));
        assert!(close(g.b3.p(), (9.0 + 4.0 + 1.0 + 16.0) / 100.0));
        assert!(close(g.b3.r(), 1.0));
        assert!(close(g.ceafe.r(), 2.0 * 4.0 / 14.0 / 4.0));
        assert!(g.conll() < 0.7, "giant {}", g.conll());
        // all singletons: MUC 0, B³ precision 1, CEAF-e low
        let single: Part = (0..10).map(|x| vec![x]).collect();
        let z = score_parts(&key, &single);
        assert!(close(z.muc.f(), 0.0));
        assert!(close(z.b3.p(), 1.0));
        assert!(close(z.b3.r(), (3.0 * (1.0 / 3.0) + 2.0 * 0.5 + 1.0 + 4.0 * 0.25) / 10.0));
        assert!(z.conll() < 0.6, "singletons {}", z.conll());
    }

    /// Kuhn–Munkres against brute-force permutations on small matrices.
    #[test]
    fn assignment_matches_brute_force() {
        fn perms(n: usize) -> Vec<Vec<usize>> {
            if n == 0 {
                return vec![vec![]];
            }
            let mut out = Vec::new();
            for p in perms(n - 1) {
                for k in 0..=p.len() {
                    let mut q = p.clone();
                    q.insert(k, n - 1);
                    out.push(q);
                }
            }
            out
        }
        let mut seed = 12345u64;
        let mut rnd = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((seed >> 33) % 7) as f64 / 6.0
        };
        for n in 1..=5 {
            for m in n..=6 {
                for _ in 0..20 {
                    let w: Vec<Vec<f64>> = (0..n).map(|_| (0..m).map(|_| rnd()).collect()).collect();
                    // brute force: pick n of m columns in every order
                    let mut best = 0.0f64;
                    for p in perms(m) {
                        let s: f64 = (0..n).map(|i| w[i][p[i]]).sum();
                        best = best.max(s);
                    }
                    assert!((assign_max(&w) - best).abs() < 1e-9, "{w:?}");
                    let t: Vec<Vec<f64>> = (0..m).map(|j| (0..n).map(|i| w[i][j]).collect()).collect();
                    assert!((assign_max(&t) - best).abs() < 1e-9);
                }
            }
        }
    }

    /// Head matching: same head — told apart by span; different head — no match.
    #[test]
    fn head_matching() {
        let sp = |s, a, b| Span { sent: s, start: a, end: b };
        // "John and Mary": the coordination and the first conjunct share the head John (0)
        let gold = [MSpan { span: sp(0, 0, 2), head: 0 }, MSpan { span: sp(0, 0, 0), head: 0 }, MSpan { span: sp(0, 2, 2), head: 2 }];
        let pred = [MSpan { span: sp(0, 0, 0), head: 0 }, MSpan { span: sp(0, 0, 3), head: 0 }, MSpan { span: sp(0, 4, 4), head: 4 }];
        let m = match_mentions(&gold, &pred, Matching::Head);
        assert_eq!(m, [Some(1), Some(0), None]);
        let e = match_mentions(&gold, &pred, Matching::Exact);
        assert_eq!(e, [Some(1), None, None]);
    }

    /// Singletons: without them, a one-mention entity disappears from both sides.
    #[test]
    fn singletons_filter() {
        let sp = |a| MSpan { span: Span { sent: 0, start: a, end: a }, head: a };
        let gold = vec![vec![sp(0), sp(1)], vec![sp(2)]];
        let pred = vec![vec![sp(0)], vec![sp(1)], vec![sp(2)]];
        let with = score_doc(&gold, &pred, Matching::Head, true);
        let without = score_doc(&gold, &pred, Matching::Head, false);
        assert!(close(with.ment.r(), 1.0) && close(with.b3.p(), 1.0));
        assert!(close(without.ment.pd, 0.0) && close(without.conll(), 0.0));
    }
}
