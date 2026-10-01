//! v5 (30.09): a word problem as a graph of subproblems (DAG) — the idea of DeductReasoner (Jie et al. 2022), but transparent.
//!
//! Quantity pool: the numbers of the text and all intermediate results; nothing disappears, so a quantity can be used
//! twice (half of 12 goes both into "gave away" and into "bought the same amount again"). A step is a pair from the pool and an operation, the result
//! is a new quantity in the pool; STOP — the last created quantity is the answer. The fast core computes each step.
//!
//! Model — an averaged structured perceptron with a beam and early update; the features are the same
//! non-lexical ones as in `steps` (verb classes, rates, units, subject and question words) — for each quantity
//! via its "representative" (the right leaf, as in Roy & Roth 2018). Weights — a dense array with feature
//! hashing: enumerating all pairs of the pool must be fast.

use std::collections::HashMap;

use crate::arith::{Op, Tree};
use crate::steps::{Desc, Prob};

const BITS: u32 = 22;
const SIZE: usize = 1 << BITS;

fn mix(a: u64, b: u64) -> usize {
    let mut x = a ^ b.wrapping_mul(0x9E3779B97F4A7C15);
    x ^= x >> 29;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 32;
    (x as usize) & (SIZE - 1)
}

fn h(s: &str) -> u64 {
    let mut x: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        x ^= b as u64;
        x = x.wrapping_mul(0x100000001b3);
    }
    x
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// pool[i] ∘ pool[j]
    Comb(u8, u8, Op),
    Stop,
}

const OPS: [Op; 4] = [Op::Add, Op::Sub, Op::Mul, Op::Div];

fn op_id(o: Op) -> u64 {
    match o {
        Op::Add => 1,
        Op::Sub => 2,
        Op::Mul => 3,
        Op::Div => 4,
    }
}

pub fn tkey(t: &Tree) -> String {
    match t {
        Tree::Leaf(i) => format!("{i}"),
        Tree::Node(op, l, r) => {
            let (a, b) = (tkey(l), tkey(r));
            let (a, b) = if matches!(op, Op::Add | Op::Mul) && a > b { (b, a) } else { (a, b) };
            format!("({a}{op:?}{b})")
        }
    }
}

/// Gold as a set of graph nodes (step order is free) and a root.
pub struct Gold {
    pub nodes: std::collections::HashSet<String>,
    pub root: String,
    pub steps: Vec<Step>,
}

pub fn gold_of(tree: &Tree, n: usize) -> Option<Gold> {
    let steps = gold_steps(tree, n)?;
    let mut nodes = std::collections::HashSet::new();
    fn walk(t: &Tree, s: &mut std::collections::HashSet<String>) {
        if let Tree::Node(_, l, r) = t {
            s.insert(tkey(t));
            walk(l, s);
            walk(r, s);
        }
    }
    walk(tree, &mut nodes);
    Some(Gold { nodes, root: tkey(tree), steps })
}

#[derive(Clone)]
struct Node {
    val: f64,
    tree: Tree,
    key: String,
    rep: usize,
    leaf: bool,
    uses: u8,
}

#[derive(Clone)]
struct St {
    pool: Vec<Node>,
    steps: Vec<Step>,
    score: f64,
    done: bool,
}

/// Static features: for each number and each pair of numbers (representatives) — string hashes.
pub struct Pre {
    one: Vec<Vec<u64>>,
    pair: Vec<Vec<Vec<u64>>>,
    stop: Vec<Vec<u64>>,
    n: usize,
}

fn desc_feats(tag: &str, d: &Desc, p: &Prob, f: &mut Vec<String>) {
    f.push(format!("{tag}vc:{}", d.vclass));
    f.push(format!("{tag}rate:{}", !d.per.is_empty()));
    f.push(format!("{tag}subjq:{}", !d.subj.is_empty() && d.subj == p.qsubj));
    f.push(format!("{tag}qbind:{}", p.qcontent.iter().any(|w| d.ctxw.contains(w) || *w == d.unit)));
    f.push(format!("{tag}vc_q:{}|{}", d.vclass, p.qvclass));
    for c in &d.cues {
        f.push(format!("{tag}c:{c}"));
    }
}

pub fn precompute(p: &Prob) -> Pre {
    let n = p.qs.len();
    let d = |i: usize| p.descs.get(i).cloned().unwrap_or_default();
    let mut one = Vec::new();
    for i in 0..n {
        let mut f = vec!["bias".to_string()];
        desc_feats("a", &d(i), p, &mut f);
        f.push(format!("const:{}", p.qs[i].name.starts_with("const")));
        one.push(f.iter().map(|s| h(s)).collect());
    }
    let mut pair = vec![vec![Vec::new(); n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (a, b) = (d(i), d(j));
            let mut f = Vec::new();
            f.push(format!("same_u:{}", !a.unit.is_empty() && a.unit == b.unit));
            f.push(format!("same_s:{}", !a.subj.is_empty() && a.subj == b.subj));
            f.push(format!("same_sent:{}", a.sent == b.sent));
            f.push(format!("order:{}", i < j));
            f.push(format!("aper_bu:{}", !a.per.is_empty() && a.per == b.unit));
            f.push(format!("bper_au:{}", !b.per.is_empty() && b.per == a.unit));
            f.push(format!("vcvc:{}|{}", a.vclass, b.vclass));
            f.push(format!("rates:{}|{}", !a.per.is_empty(), !b.per.is_empty()));
            f.push(format!("su_vc:{}|{}|{}", a.unit == b.unit, a.vclass, b.vclass));
            for c in &b.cues {
                f.push(format!("bc_su:{c}|{}", a.unit == b.unit));
            }
            for w in &p.qwords {
                if matches!(w.as_str(), "more" | "less" | "than" | "each" | "total" | "left" | "per" | "together" | "altogether" | "remain" | "times" | "average" | "difference") {
                    f.push(format!("q:{w}"));
                }
            }
            pair[i][j] = f.iter().map(|s| h(s)).collect();
        }
    }
    let mut stop = Vec::new();
    for i in 0..n {
        let mut f = vec!["stop_bias".to_string()];
        let di = d(i);
        f.push(format!("stop_qbind:{}", p.qcontent.iter().any(|w| di.ctxw.contains(w) || *w == di.unit)));
        f.push(format!("stop_uq:{}", p.qwords.contains(&di.unit)));
        stop.push(f.iter().map(|s| h(s)).collect());
    }
    Pre { one, pair, stop, n }
}

pub struct Model {
    w: Vec<f32>,
    acc: Vec<f32>,
    stamp: Vec<u32>,
    t: u32,
    pub beam: usize,
    pub max_steps: usize,
}

impl Model {
    pub fn new(beam: usize) -> Model {
        Model { w: vec![0.0; SIZE], acc: vec![0.0; SIZE], stamp: vec![0; SIZE], t: 0, beam, max_steps: 8 }
    }

    fn w(&self, k: usize) -> f64 {
        self.w[k] as f64
    }

    /// Step features as weight indices.
    fn step_idx(&self, pre: &Pre, st: &St, s: Step, out: &mut Vec<usize>) {
        match s {
            Step::Stop => {
                let last = st.pool.last().unwrap();
                let tag = h("STOP");
                for &f in &pre.stop[last.rep] {
                    out.push(mix(f, tag));
                }
                // unused numbers tied to the question — stopping is bad
                let unused = st.pool[..pre.n].iter().filter(|x| x.uses == 0).count();
                out.push(mix(0xA24BAED4963EE407 ^ unused.min(3) as u64, tag));
                out.push(mix(0x9FB21C651E98DF25 ^ st.steps.len().min(6) as u64, tag));
            }
            Step::Comb(i, j, op) => {
                let (a, b) = (&st.pool[i as usize], &st.pool[j as usize]);
                let tag = op_id(op);
                for &f in &pre.one[a.rep] {
                    out.push(mix(f, tag * 101 + 1));
                }
                for &f in &pre.one[b.rep] {
                    out.push(mix(f, tag * 101 + 2));
                }
                if a.rep != b.rep {
                    for &f in &pre.pair[a.rep][b.rep] {
                        out.push(mix(f, tag * 101 + 3));
                    }
                }
                let last = st.pool.len() - 1;
                let dynf: [(u64, u64); 8] = [
                    (1, a.leaf as u64),
                    (2, b.leaf as u64),
                    (3, (i as usize == last && !a.leaf) as u64),
                    (4, (j as usize == last && !b.leaf) as u64),
                    (5, a.uses.min(2) as u64),
                    (6, b.uses.min(2) as u64),
                    (7, st.steps.len().min(5) as u64),
                    (8, (a.val < b.val) as u64),
                ];
                for (k, v) in dynf {
                    out.push(mix(0xD1B54A32D192ED03 ^ (k << 8 | v), tag * 101 + 4));
                }
            }
        }
    }

    fn score(&self, pre: &Pre, st: &St, s: Step, buf: &mut Vec<usize>) -> f64 {
        buf.clear();
        self.step_idx(pre, st, s, buf);
        buf.iter().map(|&k| self.w(k)).sum()
    }

    fn apply(st: &St, s: Step) -> Option<St> {
        let mut n = st.clone();
        n.steps.push(s);
        match s {
            Step::Stop => n.done = true,
            Step::Comb(i, j, op) => {
                let (a, b) = (&st.pool[i as usize], &st.pool[j as usize]);
                let v = match op {
                    Op::Add => a.val + b.val,
                    Op::Sub => a.val - b.val,
                    Op::Mul => a.val * b.val,
                    Op::Div => {
                        if b.val == 0.0 {
                            return None;
                        }
                        a.val / b.val
                    }
                };
                if !v.is_finite() || v < 0.0 || v > 1e12 {
                    return None;
                }
                let tree = Tree::Node(op, Box::new(a.tree.clone()), Box::new(b.tree.clone()));
                let rep = if a.leaf && !b.leaf { a.rep } else { b.rep };
                n.pool[i as usize].uses += 1;
                n.pool[j as usize].uses += 1;
                let key = tkey(&tree);
                n.pool.push(Node { val: v, tree, key, rep, leaf: false, uses: 0 });
            }
        }
        Some(n)
    }

    /// State consistent with gold: every created node is from the gold graph, no repeats; STOP — only at the root.
    fn consistent(st: &St, g: &Gold, n: usize) -> bool {
        let created = &st.pool[n..];
        let mut seen = std::collections::HashSet::new();
        for x in created {
            if !g.nodes.contains(&x.key) || !seen.insert(&x.key) {
                return false;
            }
        }
        !st.done || created.last().is_some_and(|x| x.key == g.root)
    }

    fn start(p: &Prob) -> St {
        St { pool: p.qs.iter().enumerate().map(|(i, q)| Node { val: q.val, tree: Tree::Leaf(i), key: format!("{i}"), rep: i, leaf: true, uses: 0 }).collect(), steps: Vec::new(), score: 0.0, done: false }
    }

    fn cands(st: &St) -> Vec<Step> {
        let m = st.pool.len();
        let mut v = Vec::new();
        if st.steps.iter().any(|s| matches!(s, Step::Comb(..))) {
            v.push(Step::Stop);
        }
        for i in 0..m {
            for j in 0..m {
                if i == j {
                    continue;
                }
                for op in OPS {
                    // + and × are symmetric — only i < j
                    if matches!(op, Op::Add | Op::Mul) && i > j {
                        continue;
                    }
                    v.push(Step::Comb(i as u8, j as u8, op));
                }
            }
        }
        v
    }

    /// Beam; with `gold` — a dynamic oracle: the gold path follows the model's best step among
    /// those that create a node of the gold graph; early update when the beam has no consistent state.
    fn run(&self, p: &Prob, pre: &Pre, gold: Option<&Gold>) -> (Vec<St>, Option<(St, St)>) {
        let n = p.qs.len();
        let mut beam = vec![Self::start(p)];
        let mut gst = Self::start(p);
        let mut finals = Vec::new();
        let mut buf = Vec::new();
        for k in 0..=self.max_steps {
            let mut next: Vec<St> = Vec::new();
            for st in &beam {
                if st.done {
                    finals.push(st.clone());
                    continue;
                }
                for s in Self::cands(st) {
                    if k == self.max_steps && s != Step::Stop {
                        continue;
                    }
                    let sc = self.score(pre, st, s, &mut buf);
                    if let Some(mut nx) = Self::apply(st, s) {
                        nx.score = st.score + sc;
                        next.push(nx);
                    }
                }
            }
            if let Some(g) = gold {
                // gold path: the model's best consistent step
                if !gst.done {
                    let mut best: Option<St> = None;
                    for s in Self::cands(&gst) {
                        let sc = self.score(pre, &gst, s, &mut buf);
                        if let Some(mut nx) = Self::apply(&gst, s) {
                            nx.score = gst.score + sc;
                            if Self::consistent(&nx, g, n) && best.as_ref().is_none_or(|b| nx.score > b.score) {
                                best = Some(nx);
                            }
                        }
                    }
                    match best {
                        Some(b) => gst = b,
                        None => return (finals, None),
                    }
                }
            }
            if next.is_empty() {
                break;
            }
            next.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
            next.truncate(self.beam);
            if let Some(g) = gold {
                let ok = next.iter().any(|s| Self::consistent(s, g, n)) || finals.iter().any(|s| Self::consistent(s, g, n));
                if !ok {
                    return (finals, Some((gst, next[0].clone())));
                }
            }
            beam = next;
        }
        finals.extend(beam.into_iter().filter(|s| s.done));
        if let Some(g) = gold {
            // extend the gold path to STOP for the final update
            let mut guard = 0;
            while !gst.done && guard < 12 {
                guard += 1;
                let mut best: Option<St> = None;
                for s in Self::cands(&gst) {
                    let sc = self.score(pre, &gst, s, &mut buf);
                    if let Some(mut nx) = Self::apply(&gst, s) {
                        nx.score = gst.score + sc;
                        if Self::consistent(&nx, g, n) && best.as_ref().is_none_or(|b| nx.score > b.score) {
                            best = Some(nx);
                        }
                    }
                }
                match best {
                    Some(b) => gst = b,
                    None => break,
                }
            }
            return (finals, Some((gst, St { pool: Vec::new(), steps: Vec::new(), score: f64::NAN, done: false })));
        }
        (finals, None)
    }

    fn seq_idx(&self, p: &Prob, pre: &Pre, steps: &[Step]) -> Vec<usize> {
        let mut st = Self::start(p);
        let mut out = Vec::new();
        for &s in steps {
            self.step_idx(pre, &st, s, &mut out);
            match Self::apply(&st, s) {
                Some(n) => st = n,
                None => break,
            }
        }
        out
    }

    fn update(&mut self, idx: &[usize], d: f32) {
        let t = self.t;
        for &k in idx {
            self.acc[k] += self.w[k] * (t - self.stamp[k]) as f32;
            self.stamp[k] = t;
            self.w[k] += d;
        }
    }

    pub fn learn(&mut self, p: &Prob, pre: &Pre, gold: &Gold) -> bool {
        self.t += 1;
        let n = p.qs.len();
        let (finals, res) = self.run(p, pre, Some(gold));
        let Some((gst, early)) = res else { return false };
        if !early.score.is_nan() {
            // early update: gold prefix vs the best in the beam
            let g = self.seq_idx(p, pre, &gst.steps);
            let b = self.seq_idx(p, pre, &early.steps);
            self.update(&g, 1.0);
            self.update(&b, -1.0);
            return false;
        }
        let Some(best) = finals.iter().max_by(|a, b| a.score.partial_cmp(&b.score).unwrap()) else { return false };
        if Self::consistent(best, gold, n) {
            return true;
        }
        if !gst.done {
            return false;
        }
        let g = self.seq_idx(p, pre, &gst.steps);
        let b = self.seq_idx(p, pre, &best.steps);
        self.update(&g, 1.0);
        self.update(&b, -1.0);
        false
    }

    pub fn averaged(&self) -> Model {
        let t = self.t.max(1);
        let w: Vec<f32> = (0..SIZE).map(|k| (self.acc[k] + self.w[k] * (t - self.stamp[k]) as f32) / t as f32).collect();
        Model { w, acc: Vec::new(), stamp: Vec::new(), t: 0, beam: self.beam, max_steps: self.max_steps }
    }

    /// Top-K distinct answers (non-negative integers): (tree, score, value).
    pub fn topk(&self, p: &Prob, pre: &Pre, k: usize) -> Vec<(Tree, f64, f64)> {
        let (mut finals, _) = self.run(p, pre, None);
        finals.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        let mut out: Vec<(Tree, f64, f64)> = Vec::new();
        for f in finals {
            let last = f.pool.last().unwrap();
            if last.leaf || (last.val - last.val.round()).abs() > 1e-6 {
                continue;
            }
            if out.iter().any(|x| (x.2 - last.val).abs() < 1e-6) {
                continue;
            }
            out.push((last.tree.clone(), f.score, last.val));
            if out.len() >= k {
                break;
            }
        }
        out
    }
}

/// Gold sequence of steps from a tree: identical subtrees — one quantity (DAG); order — postfix.
pub fn gold_steps(tree: &Tree, n: usize) -> Option<Vec<Step>> {
    fn key(t: &Tree) -> String {
        match t {
            Tree::Leaf(i) => format!("{i}"),
            Tree::Node(op, l, r) => format!("({}{:?}{})", key(l), op, key(r)),
        }
    }
    fn go(t: &Tree, ids: &mut HashMap<String, usize>, pool: &mut usize, out: &mut Vec<Step>) -> Option<usize> {
        match t {
            Tree::Leaf(i) => Some(*i),
            Tree::Node(op, l, r) => {
                let k = key(t);
                if let Some(&id) = ids.get(&k) {
                    return Some(id);
                }
                let a = go(l, ids, pool, out)?;
                let b = go(r, ids, pool, out)?;
                if a > 255 || b > 255 {
                    return None;
                }
                // + and × — canonically the smaller index on the left (as in candidates)
                let (a, b) = if matches!(op, Op::Add | Op::Mul) && a > b { (b, a) } else { (a, b) };
                out.push(Step::Comb(a as u8, b as u8, *op));
                let id = *pool;
                *pool += 1;
                ids.insert(k, id);
                Some(id)
            }
        }
    }
    let mut out = Vec::new();
    let mut pool = n;
    let mut ids = HashMap::new();
    go(tree, &mut ids, &mut pool, &mut out)?;
    if out.is_empty() || out.len() > 8 {
        return None;
    }
    out.push(Step::Stop);
    Some(out)
}
