//! MMM models behind the shared `Classifier` interface — they run side by side and are compared on the same
//! features (`feats`), the same split and the same table:
//! - **perceptron** — averaged, "slot=value" indicators; explanation — the features that tipped the balance
//!   (weight difference between the chosen and the runner-up class);
//! - **IGTree** (Daelemans, van den Bosch, Weijters 1997; TiMBL) — memory as a prefix-tree graph:
//!   slots ordered by gain ratio, each node holds a default label; prediction — walking the graph while
//!   values match; explanation — the path walked;
//! - **k-NN (IB1)** (Aha et al. 1991; TiMBL) — full memory, distance — weighted overlap (weights —
//!   gain ratio), k nearest distances with k chosen on the training set (leave-one-out); explanation —
//!   the most similar sentences from memory.

use serde::{Deserialize, Serialize};

use crate::data::h64;
use crate::feats::{Dict, SLOTS, UNSEEN};

/// Prediction with explanation.
#[derive(Clone, Debug)]
pub struct Pred {
    pub class: usize,
    /// confidence: perceptron — margin over the runner-up class; IGTree — class share in the node; k-NN — vote share
    pub conf: f32,
    /// explanation: features with weights (perceptron), the path walked with the default label's share (IGTree),
    /// nearest from memory — sentence key and distance (k-NN)
    pub why: Vec<(String, f32)>,
}

/// Shared interface of field models.
pub trait Classifier: Send + Sync {
    fn kind(&self) -> Kind;
    fn predict(&self, x: &[u32]) -> Pred;
    /// model size in the `en::store` binary format
    fn bytes(&self) -> usize;
    /// short summary of the trained model (features, nodes, k)
    fn info(&self) -> String;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Kind {
    Perceptron,
    IgTree,
    Knn,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::Perceptron, Kind::IgTree, Kind::Knn];
    pub fn name(self) -> &'static str {
        match self {
            Kind::Perceptron => "perceptron",
            Kind::IgTree => "igtree",
            Kind::Knn => "knn",
        }
    }
}

/// Training data of a field.
pub struct Train<'a> {
    pub x: &'a [Vec<u32>],
    pub y: &'a [usize],
    pub names: &'a [&'a str],
    /// sentence keys (for k-NN explanations)
    pub keys: &'a [String],
    pub dict: &'a Dict,
    pub seed: u64,
}

impl Train<'_> {
    fn nclass(&self) -> usize {
        self.names.len()
    }
}

/// Train a model of the requested type.
pub fn fit(kind: Kind, t: &Train) -> Box<dyn Classifier> {
    match kind {
        Kind::Perceptron => Box::new(Perceptron::train(t, 12)),
        Kind::IgTree => Box::new(IgTree::train(t)),
        Kind::Knn => Box::new(Knn::train(t)),
    }
}

fn size<T: Serialize>(v: &T) -> usize {
    en::store::to_bytes(v).map(|b| b.len()).unwrap_or(0)
}

// ── Gain ratio ──────────────────────────────────────────────────────────────────────────────────

fn entropy(counts: &[u32]) -> f64 {
    let n: u32 = counts.iter().sum();
    if n == 0 {
        return 0.0;
    }
    counts.iter().filter(|&&c| c > 0).map(|&c| {
        let p = c as f64 / n as f64;
        -p * p.log2()
    }).sum()
}

/// Gain ratio of a slot: (H(Y) − Σ p(v) H(Y|v)) / SplitInfo (Quinlan 1993; TiMBL weights).
pub fn gain_ratio(x: &[Vec<u32>], y: &[usize], nclass: usize, s: usize) -> f64 {
    let mut by: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
    let mut all = vec![0u32; nclass];
    for (r, &c) in x.iter().zip(y) {
        by.entry(r[s]).or_insert_with(|| vec![0; nclass])[c] += 1;
        all[c] += 1;
    }
    let n = x.len() as f64;
    let h = entropy(&all);
    let (mut cond, mut split) = (0.0, 0.0);
    for cs in by.values() {
        let p = cs.iter().sum::<u32>() as f64 / n;
        cond += p * entropy(cs);
        split -= p * p.log2();
    }
    if split <= 1e-12 { 0.0 } else { ((h - cond) / split).max(0.0) }
}

fn weights(t: &Train) -> Vec<f64> {
    (0..SLOTS.len()).map(|s| gain_ratio(t.x, t.y, t.nclass(), s)).collect()
}

/// Class frequencies (ties are broken in favour of the more frequent class).
fn freq(y: &[usize], nclass: usize) -> Vec<u32> {
    let mut f = vec![0u32; nclass];
    for &c in y {
        f[c] += 1;
    }
    f
}

fn argmax_tie(counts: &[u32], prior: &[u32]) -> usize {
    (0..counts.len()).max_by(|&a, &b| counts[a].cmp(&counts[b]).then(prior[a].cmp(&prior[b])).then(b.cmp(&a))).unwrap_or(0)
}

// ── Perceptron ──────────────────────────────────────────────────────────────────────────────────

/// Averaged perceptron (Collins 2002) on "slot=value" indicators and a bias.
#[derive(Serialize, Deserialize)]
pub struct Perceptron {
    names: Vec<String>,
    /// start of the slot's feature numbers
    offsets: Vec<u32>,
    sizes: Vec<u32>,
    nfeat: usize,
    /// weights [class × feature], the last feature is the bias
    w: Vec<f32>,
    /// "slot=value" by feature number — for explanations
    shown: Vec<String>,
}

impl Perceptron {
    fn active(&self, x: &[u32]) -> Vec<usize> {
        let mut f: Vec<usize> = x.iter().enumerate().filter(|&(s, &v)| v != UNSEEN && v < self.sizes[s]).map(|(s, &v)| (self.offsets[s] + v) as usize).collect();
        f.push(self.nfeat - 1);
        f
    }

    fn scores(&self, f: &[usize]) -> Vec<f32> {
        let nc = self.names.len();
        (0..nc).map(|c| f.iter().map(|&k| self.w[c * self.nfeat + k]).sum()).collect()
    }

    pub fn train(t: &Train, epochs: usize) -> Perceptron {
        let nc = t.nclass();
        let sizes: Vec<u32> = t.dict.values.iter().map(|v| v.len() as u32).collect();
        let mut offsets = Vec::with_capacity(sizes.len());
        let mut acc = 0u32;
        for &s in &sizes {
            offsets.push(acc);
            acc += s;
        }
        let nfeat = acc as usize + 1;
        let shown: Vec<String> = (0..sizes.len()).flat_map(|s| (0..sizes[s]).map(move |v| (s, v))).map(|(s, v)| t.dict.show(s, v)).chain(["bias".to_string()]).collect();
        let mut p = Perceptron { names: t.names.iter().map(|x| x.to_string()).collect(), offsets, sizes, nfeat, w: vec![0.0; nc * nfeat], shown };
        // averaging: u accumulates step × update, average = w − u / steps
        let mut u = vec![0.0f32; nc * nfeat];
        let mut step = 1.0f32;
        let feats: Vec<Vec<usize>> = t.x.iter().map(|x| p.active(x)).collect();
        let mut order: Vec<usize> = (0..t.x.len()).collect();
        for e in 0..epochs {
            order.sort_by_key(|&i| h64(&i.to_string(), t.seed ^ (e as u64 + 1)));
            for &i in &order {
                let sc = p.scores(&feats[i]);
                let guess = (0..nc).max_by(|&a, &b| sc[a].partial_cmp(&sc[b]).unwrap().then(b.cmp(&a))).unwrap();
                let gold = t.y[i];
                if guess != gold {
                    for &k in &feats[i] {
                        p.w[gold * nfeat + k] += 1.0;
                        p.w[guess * nfeat + k] -= 1.0;
                        u[gold * nfeat + k] += step;
                        u[guess * nfeat + k] -= step;
                    }
                }
                step += 1.0;
            }
        }
        for (w, u) in p.w.iter_mut().zip(&u) {
            *w -= u / step;
        }
        p
    }
}

impl Classifier for Perceptron {
    fn kind(&self) -> Kind {
        Kind::Perceptron
    }

    fn predict(&self, x: &[u32]) -> Pred {
        let f = self.active(x);
        let sc = self.scores(&f);
        let mut ord: Vec<usize> = (0..sc.len()).collect();
        ord.sort_by(|&a, &b| sc[b].partial_cmp(&sc[a]).unwrap().then(a.cmp(&b)));
        let (c, r) = (ord[0], ord.get(1).copied().unwrap_or(ord[0]));
        // features that tipped the balance: contribution to the chosen class's margin over the runner-up
        let mut why: Vec<(String, f32)> = f.iter().map(|&k| (self.shown[k].clone(), self.w[c * self.nfeat + k] - self.w[r * self.nfeat + k])).filter(|x| x.1 > 0.0).collect();
        why.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        why.truncate(5);
        Pred { class: c, conf: sc[c] - sc[r], why }
    }

    fn bytes(&self) -> usize {
        size(self)
    }

    fn info(&self) -> String {
        format!("features {}", self.nfeat)
    }
}

// ── IGTree ──────────────────────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone)]
struct Node {
    class: u16,
    /// examples in the node and how many of them have the default label
    n: u32,
    hits: u32,
    /// (value of this level's slot, node), sorted by value
    kids: Vec<(u32, u32)>,
}

/// Memory as a prefix-tree graph (IGTree).
#[derive(Serialize, Deserialize)]
pub struct IgTree {
    names: Vec<String>,
    /// slot order by gain ratio, descending
    order: Vec<usize>,
    gr: Vec<f64>,
    nodes: Vec<Node>,
    shown: Vec<Vec<String>>,
}

impl IgTree {
    pub fn train(t: &Train) -> IgTree {
        let gr = weights(t);
        let mut order: Vec<usize> = (0..SLOTS.len()).collect();
        order.sort_by(|&a, &b| gr[b].partial_cmp(&gr[a]).unwrap().then(a.cmp(&b)));
        let prior = freq(t.y, t.nclass());
        let mut tree = IgTree { names: t.names.iter().map(|x| x.to_string()).collect(), order, gr, nodes: Vec::new(), shown: t.dict.values.clone() };
        let idx: Vec<usize> = (0..t.x.len()).collect();
        tree.build(t, &prior, &idx, 0);
        tree
    }

    /// Node for examples `idx` at depth `depth`; returns the node number.
    fn build(&mut self, t: &Train, prior: &[u32], idx: &[usize], depth: usize) -> u32 {
        let mut counts = vec![0u32; t.nclass()];
        for &i in idx {
            counts[t.y[i]] += 1;
        }
        let class = argmax_tie(&counts, prior);
        let me = self.nodes.len() as u32;
        self.nodes.push(Node { class: class as u16, n: idx.len() as u32, hits: counts[class], kids: Vec::new() });
        if counts[class] as usize == idx.len() || depth == self.order.len() {
            return me;
        }
        let s = self.order[depth];
        let mut parts: std::collections::BTreeMap<u32, Vec<usize>> = std::collections::BTreeMap::new();
        for &i in idx {
            parts.entry(t.x[i][s]).or_default().push(i);
        }
        let mut kids = Vec::new();
        for (v, part) in parts {
            let k = self.build(t, prior, &part, depth + 1);
            // pruning (TiMBL): a leaf with the same label as its parent adds nothing
            let kn = &self.nodes[k as usize];
            if kn.kids.is_empty() && kn.class as usize == class {
                self.nodes.truncate(k as usize);
                continue;
            }
            kids.push((v, k));
        }
        self.nodes[me as usize].kids = kids;
        me
    }
}

impl Classifier for IgTree {
    fn kind(&self) -> Kind {
        Kind::IgTree
    }

    fn predict(&self, x: &[u32]) -> Pred {
        let mut node = &self.nodes[0];
        let mut why = vec![(format!("root → {} {}/{}", self.names[node.class as usize], node.hits, node.n), node.hits as f32 / node.n.max(1) as f32)];
        for &s in &self.order {
            let v = x[s];
            let Ok(k) = node.kids.binary_search_by_key(&v, |x| x.0) else { break };
            node = &self.nodes[node.kids[k].1 as usize];
            let val = if v == UNSEEN { "?" } else { self.shown[s][v as usize].as_str() };
            why.push((format!("{}={val} → {} {}/{}", SLOTS[s], self.names[node.class as usize], node.hits, node.n), node.hits as f32 / node.n.max(1) as f32));
            if node.kids.is_empty() {
                break;
            }
        }
        Pred { class: node.class as usize, conf: node.hits as f32 / node.n.max(1) as f32, why }
    }

    fn bytes(&self) -> usize {
        size(&(&self.names, &self.order, &self.nodes))
    }

    fn info(&self) -> String {
        self.nodes.len().to_string()
    }
}

// ── k-NN (IB1) ──────────────────────────────────────────────────────────────────────────────────

/// Full memory and weighted overlap.
#[derive(Serialize, Deserialize)]
pub struct Knn {
    names: Vec<String>,
    w: Vec<f32>,
    x: Vec<Vec<u32>>,
    y: Vec<u16>,
    keys: Vec<String>,
    prior: Vec<u32>,
    /// k nearest distances (chosen by leave-one-out on the training set)
    pub k: usize,
}

impl Knn {
    pub fn train(t: &Train) -> Knn {
        let w: Vec<f32> = weights(t).into_iter().map(|g| g as f32).collect();
        let mut m = Knn { names: t.names.iter().map(|x| x.to_string()).collect(), w, x: t.x.to_vec(), y: t.y.iter().map(|&c| c as u16).collect(), keys: t.keys.to_vec(), prior: freq(t.y, t.nclass()), k: 1 };
        // choosing k: leave-one-out on the training set
        let cands = [1usize, 3, 5, 7, 11];
        let mut hit = [0usize; 5];
        for i in 0..m.x.len() {
            let d = m.dists(&m.x[i], Some(i));
            for (j, &k) in cands.iter().enumerate() {
                if m.vote(&d, k).0 == m.y[i] as usize {
                    hit[j] += 1;
                }
            }
        }
        let best = (0..cands.len()).max_by(|&a, &b| hit[a].cmp(&hit[b]).then(b.cmp(&a))).unwrap();
        m.k = cands[best];
        m
    }

    fn dist(&self, a: &[u32], b: &[u32]) -> f32 {
        a.iter().zip(b).zip(&self.w).map(|((x, y), w)| if x == y && *x != UNSEEN { 0.0 } else { *w }).sum()
    }

    /// (distance, index in memory), ascending; `skip` — the example itself (leave-one-out).
    fn dists(&self, x: &[u32], skip: Option<usize>) -> Vec<(f32, usize)> {
        let mut d: Vec<(f32, usize)> = self.x.iter().enumerate().filter(|(i, _)| Some(*i) != skip).map(|(i, m)| (self.dist(x, m), i)).collect();
        d.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.cmp(&b.1)));
        d
    }

    /// Voting among the k nearest distances (all neighbours at those distances, as in TiMBL).
    fn vote(&self, d: &[(f32, usize)], k: usize) -> (usize, usize, usize) {
        let mut counts = vec![0u32; self.names.len()];
        let (mut distinct, mut last, mut n) = (0usize, f32::NAN, 0usize);
        for &(dd, i) in d {
            if dd != last {
                distinct += 1;
                if distinct > k {
                    break;
                }
                last = dd;
            }
            counts[self.y[i] as usize] += 1;
            n += 1;
        }
        let c = argmax_tie(&counts, &self.prior);
        (c, counts[c] as usize, n)
    }
}

impl Classifier for Knn {
    fn kind(&self) -> Kind {
        Kind::Knn
    }

    fn predict(&self, x: &[u32]) -> Pred {
        let d = self.dists(x, None);
        let (c, votes, n) = self.vote(&d, self.k);
        // most similar with the same label — "this is a request because the nearest are: …"
        let why: Vec<(String, f32)> = d.iter().filter(|(_, i)| self.y[*i] as usize == c).take(3).map(|&(dd, i)| (self.keys[i].clone(), dd)).collect();
        Pred { class: c, conf: votes as f32 / n.max(1) as f32, why }
    }

    fn bytes(&self) -> usize {
        size(&(&self.w, &self.x, &self.y, &self.prior))
    }

    fn info(&self) -> String {
        format!("k={}", self.k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feats::Row;

    /// The three models learn a simple rule "w1=can and you → request, otherwise assert" and explain the decision;
    /// negative control — on shuffled labels the rule is not learned.
    #[test]
    fn three_models_learn_and_explain() {
        let mut rows: Vec<Row> = Vec::new();
        let mut y = Vec::new();
        for i in 0..200 {
            let can = i % 2 == 0;
            let you = i % 3 != 0;
            let mut r: Row = SLOTS.iter().map(|_| "-".to_string()).collect();
            r[0] = if can { "can".into() } else { format!("w{}", i % 7) };
            r[20] = if you { "yes".into() } else { "-".into() };
            r[3] = format!("z{}", i % 5);
            rows.push(r);
            y.push(if can && you { 1 } else { 0 });
        }
        let dict = Dict::build(&rows);
        let x: Vec<Vec<u32>> = rows.iter().map(|r| dict.encode(r)).collect();
        let keys: Vec<String> = (0..x.len()).map(|i| format!("s{i}")).collect();
        let names = ["assert", "request"];
        let t = Train { x: &x, y: &y, names: &names, keys: &keys, dict: &dict, seed: 1 };
        let mut probe: Row = SLOTS.iter().map(|_| "-".to_string()).collect();
        probe[0] = "can".into();
        probe[20] = "yes".into();
        probe[3] = "z9".into(); // unseen value
        let px = dict.encode(&probe);
        for k in Kind::ALL {
            let m = fit(k, &t);
            let acc = x.iter().zip(&y).filter(|(r, c)| m.predict(r).class == **c).count();
            assert!(acc >= 195, "{k:?} {acc}");
            let p = m.predict(&px);
            assert_eq!(p.class, 1, "{k:?}");
            assert!(!p.why.is_empty() && m.bytes() > 0, "{k:?}");
        }
        // explanation: the perceptron names w1=can, IGTree walks a path through w1=can
        let p = fit(Kind::Perceptron, &t).predict(&px);
        assert!(p.why.iter().any(|w| w.0 == "w1=can"), "{:?}", p.why);
        let p = fit(Kind::IgTree, &t).predict(&px);
        assert!(p.why.iter().any(|w| w.0.starts_with("w1=can")), "{:?}", p.why);
        // negative control: labels shuffled — accuracy on the rule drops to chance level
        let mut ys = y.clone();
        ys.sort_by_key(|&c| c); // all 0s first…
        let perm: Vec<usize> = { let mut o: Vec<usize> = (0..ys.len()).collect(); o.sort_by_key(|&i| h64(&i.to_string(), 5)); o };
        let shuffled: Vec<usize> = perm.iter().map(|&i| ys[i]).collect();
        let ts = Train { x: &x, y: &shuffled, names: &names, keys: &keys, dict: &dict, seed: 1 };
        for k in Kind::ALL {
            let m = fit(k, &ts);
            // on the true labels, a model trained on shuffled ones is no better than the majority class (2/3), with margin
            let acc = x.iter().zip(&y).filter(|(r, c)| m.predict(r).class == **c).count() as f64 / y.len() as f64;
            assert!(acc < 0.8, "{k:?} {acc}");
        }
    }
}
