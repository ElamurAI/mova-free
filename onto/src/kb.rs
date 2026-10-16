//! Knowledge on top of the graph: world layers, default property seeds with exceptions, extra
//! IsA/InstanceOf edges from seeds, inheritance with proof.
//!
//! **Layers:** real world → genre → collection → tale. A seed acts on its own layer and below: a query on layer L
//! sees only the seeds of L's ancestor layers (including L).
//!
//! **Inheritance** (`prop`):
//! 1. the most specific layer where at least one visible seed applies to the concept or its ancestor decides;
//! 2. within it the most specific concept wins: a descendant's seed overrides an ancestor's seed;
//! 3. two incomparable seeds with different values (the Nixon diamond) — "undetermined" with both proofs,
//!    not a random choice.
//!
//! **Strength:** `Strict` is not overridden on its own layer — a descendant with the opposite value on the same layer
//! causes a refusal at load time. A lower layer may explicitly rewrite anything: that is what layers are for.
//!
//! **Load gates:** unknown layer or parent, layer cycle, lemma not a member of the synset (a typo
//! in the id), duplicate id, contradiction on the same layer and concept, contradiction with `Strict`, IsA cycle after
//! the extra edges.

use std::collections::{HashMap, VecDeque};
use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::model::*;

/// A concept's property — an enum attribute.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Prop {
    Living,
    Animate,
    Human,
    CanSpeak,
    CanFly,
    CanSwim,
    CanWalk,
    Edible,
    LaysEggs,
    Fast,
    Exists,
    Pacifist,
    // character (v2): only on the "fable" genre layer, not in the real world
    Cunning,
    Vain,
    Greedy,
    Industrious,
    Wise,
}

impl Prop {
    pub const ALL: [Prop; 17] = [
        Prop::Living,
        Prop::Animate,
        Prop::Human,
        Prop::CanSpeak,
        Prop::CanFly,
        Prop::CanSwim,
        Prop::CanWalk,
        Prop::Edible,
        Prop::LaysEggs,
        Prop::Fast,
        Prop::Exists,
        Prop::Pacifist,
        Prop::Cunning,
        Prop::Vain,
        Prop::Greedy,
        Prop::Industrious,
        Prop::Wise,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Prop::Living => "Living",
            Prop::Animate => "Animate",
            Prop::Human => "Human",
            Prop::CanSpeak => "CanSpeak",
            Prop::CanFly => "CanFly",
            Prop::CanSwim => "CanSwim",
            Prop::CanWalk => "CanWalk",
            Prop::Edible => "Edible",
            Prop::LaysEggs => "LaysEggs",
            Prop::Fast => "Fast",
            Prop::Exists => "Exists",
            Prop::Pacifist => "Pacifist",
            Prop::Cunning => "Cunning",
            Prop::Vain => "Vain",
            Prop::Greedy => "Greedy",
            Prop::Industrious => "Industrious",
            Prop::Wise => "Wise",
        }
    }

    /// `CanFly`, `fly`, `can_fly`, `lays_eggs`, `talk` → property.
    pub fn parse(s: &str) -> Option<Prop> {
        let k = s.to_ascii_lowercase().replace(['_', '-'], "");
        let k = k.strip_prefix("can").filter(|r| !r.is_empty()).unwrap_or(&k).to_string();
        Some(match k.as_str() {
            "living" | "alive" => Prop::Living,
            "animate" => Prop::Animate,
            "human" => Prop::Human,
            "speak" | "talk" => Prop::CanSpeak,
            "fly" => Prop::CanFly,
            "swim" => Prop::CanSwim,
            "walk" => Prop::CanWalk,
            "edible" | "eat" => Prop::Edible,
            "layseggs" | "layeggs" => Prop::LaysEggs,
            "fast" => Prop::Fast,
            "exists" | "exist" | "real" => Prop::Exists,
            "pacifist" => Prop::Pacifist,
            "cunning" | "sly" => Prop::Cunning,
            "vain" => Prop::Vain,
            "greedy" => Prop::Greedy,
            "industrious" | "hardworking" => Prop::Industrious,
            "wise" => Prop::Wise,
            _ => return None,
        })
    }

    /// Selectional preference of a property: which lexicographer file to look in for the word's sense when
    /// there is no context. Body abilities are about animals (a whale swims as an animal, not as a "big person"), edibility is
    /// about food. The rest — the first sense.
    pub fn prefers(self) -> Option<&'static str> {
        match self {
            // fable character is about animal characters: "fox" is the beast, not a cunning person
            Prop::CanFly | Prop::CanSwim | Prop::CanWalk | Prop::LaysEggs | Prop::Fast | Prop::Cunning | Prop::Vain | Prop::Greedy | Prop::Industrious | Prop::Wise => Some("noun.animal"),
            Prop::Edible => Some("noun.food"),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Strength {
    Default,
    Strict,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LayerKind {
    Real,
    Genre,
    Collection,
    Tale,
}

impl LayerKind {
    fn parse(s: &str) -> Option<LayerKind> {
        Some(match s {
            "real" => LayerKind::Real,
            "genre" => LayerKind::Genre,
            "collection" => LayerKind::Collection,
            "tale" => LayerKind::Tale,
            _ => return None,
        })
    }
    pub fn uk(self) -> &'static str {
        match self {
            LayerKind::Real => "real world",
            LayerKind::Genre => "genre",
            LayerKind::Collection => "collection",
            LayerKind::Tale => "tale",
        }
    }
}

pub type LayerId = u16;

#[derive(Clone, Debug)]
pub struct Layer {
    pub name: String,
    pub kind: LayerKind,
    pub parent: Option<LayerId>,
    pub note: String,
}

/// A property seed.
#[derive(Clone, Debug)]
pub struct Assert {
    pub id: String,
    pub layer: LayerId,
    pub concept: Cid,
    pub prop: Prop,
    pub value: bool,
    pub strength: Strength,
    pub source: String,
    pub note: String,
}

/// An extra edge from a seed.
#[derive(Clone, Debug)]
pub struct Extra {
    pub id: String,
    pub rel: Rel,
    pub from: Cid,
    pub to: Cid,
    pub source: String,
}

/// A step of the chain upward: from a concept to the next one via a graph edge or a seed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Step {
    pub rel: Rel,
    pub to: Cid,
    /// number of the extra seed edge, if the step comes from a seed
    pub extra: Option<u32>,
}

/// Chain: start and steps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chain {
    pub start: Cid,
    pub steps: Vec<Step>,
}

impl Chain {
    pub fn end(&self) -> Cid {
        self.steps.last().map(|s| s.to).unwrap_or(self.start)
    }
}

/// Ancestors of a concept via IsA/InstanceOf (including itself): depth and predecessor for the chain.
pub struct Anc {
    pub start: Cid,
    /// concept → (depth, previous step: from where and how)
    pub map: HashMap<Cid, (u16, Option<(Cid, Step)>)>,
}

impl Anc {
    pub fn contains(&self, c: Cid) -> bool {
        self.map.contains_key(&c)
    }
    pub fn depth(&self, c: Cid) -> Option<u16> {
        self.map.get(&c).map(|x| x.0)
    }
    /// The shortest chain from the start to `c`.
    pub fn chain(&self, c: Cid) -> Option<Chain> {
        let mut steps = Vec::new();
        let mut cur = c;
        loop {
            let (_, prev) = self.map.get(&cur)?;
            match prev {
                None => break,
                Some((from, st)) => {
                    steps.push(*st);
                    cur = *from;
                }
            }
        }
        steps.reverse();
        Some(Chain { start: self.start, steps })
    }
}

fn chain_str_plain(g: &Graph, ch: &Chain) -> String {
    let mut s = g.show(ch.start);
    for st in &ch.steps {
        s.push_str(&format!(" —{}→ {}", st.rel.name(), g.show(st.to)));
    }
    s
}

/// Answer about a property.
#[derive(Clone, Debug)]
pub enum PropAnswer {
    /// value, winning seed, chain to its concept, defeated (overridden) seeds
    Known { value: bool, win: usize, chain: Chain, beaten: Vec<usize>, layer: LayerId },
    /// conflicting inheritance: seeds with both values, each with its own chain
    Undetermined { cands: Vec<(usize, Chain)>, layer: LayerId },
    /// no visible seed applies to the concept
    Unknown,
}

pub struct Kb {
    pub g: Graph,
    /// seed gates (strictness, cycles); only negative controls turn them off
    pub gates: bool,
    /// FLAW for a negative control: all layers are visible from any layer (the fable exception leaks into the real world)
    pub leak_all_layers: bool,
    pub layers: Vec<Layer>,
    pub asserts: Vec<Assert>,
    pub extra: Vec<Extra>,
    extra_out: HashMap<Cid, Vec<u32>>,
    by_concept: HashMap<Cid, Vec<usize>>,
}

/// TSV rows without comments and empty lines; each is (line number, fields).
fn tsv(path: &Path) -> Result<Vec<(usize, Vec<String>)>> {
    let s = std::fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    Ok(s.lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|(i, l)| (i + 1, l.split('\t').map(|x| x.trim().to_string()).collect()))
        .collect())
}

impl Kb {
    pub fn new(g: Graph) -> Kb {
        Kb {
            g,
            gates: true,
            leak_all_layers: false,
            layers: vec![Layer { name: "real".into(), kind: LayerKind::Real, parent: None, note: "real world".into() }],
            asserts: Vec::new(),
            extra: Vec::new(),
            extra_out: HashMap::new(),
            by_concept: HashMap::new(),
        }
    }

    pub fn layer_id(&self, name: &str) -> Option<LayerId> {
        self.layers.iter().position(|l| l.name == name).map(|i| i as LayerId)
    }

    /// A layer and its ancestors, from the most specific to the root.
    pub fn layer_chain(&self, l: LayerId) -> Vec<LayerId> {
        let mut v = vec![l];
        let mut cur = l;
        while let Some(p) = self.layers[cur as usize].parent {
            v.push(p);
            cur = p;
        }
        v
    }

    /// Synset from a seed: id and a lemma that must be its member.
    pub fn resolve_seed_concept(&self, lemma: &str, synset: &str) -> Result<Cid> {
        let c = self.g.find_synset(synset).with_context(|| format!("synset {synset} is not in OEWN"))?;
        let lemmas: Vec<&str> = self.g.members(c).iter().map(|w| self.g.word(*w)).collect();
        if !lemmas.iter().any(|m| m.eq_ignore_ascii_case(lemma)) {
            bail!("lemma «{lemma}» is not a member of synset {synset} (members: {})", lemmas.join(", "));
        }
        Ok(c)
    }

    pub fn load_layers(&mut self, path: &Path) -> Result<()> {
        for (ln, f) in tsv(path)? {
            if f.len() < 3 {
                bail!("{}:{ln}: need layer, kind, parent", path.display());
            }
            let kind = LayerKind::parse(&f[1]).with_context(|| format!("{}:{ln}: layer kind «{}»", path.display(), f[1]))?;
            if f[0] == "real" {
                if kind != LayerKind::Real || f[2] != "-" {
                    bail!("{}:{ln}: real is the root, without a parent", path.display());
                }
                continue;
            }
            if self.layer_id(&f[0]).is_some() {
                bail!("{}:{ln}: layer {} a second time", path.display(), f[0]);
            }
            let parent = self.layer_id(&f[2]).with_context(|| format!("{}:{ln}: unknown parent «{}» (the parent must come earlier in the text)", path.display(), f[2]))?;
            let pk = self.layers[parent as usize].kind as u8;
            if (kind as u8) <= pk {
                bail!("{}:{ln}: layer {} ({:?}) is not lower than its parent {} ({:?})", path.display(), f[0], kind, f[2], self.layers[parent as usize].kind);
            }
            self.layers.push(Layer { name: f[0].clone(), kind, parent: Some(parent), note: f.get(3).cloned().unwrap_or_default() });
        }
        Ok(())
    }

    pub fn load_relations(&mut self, path: &Path) -> Result<()> {
        for (ln, f) in tsv(path)? {
            if f.len() < 7 {
                bail!("{}:{ln}: need id, relation, lemma, synset, lemma, synset, source", path.display());
            }
            let rel = match f[1].as_str() {
                "IsA" => Rel::IsA,
                "InstanceOf" => Rel::InstanceOf,
                r => bail!("{}:{ln}: seed relation «{r}» — only IsA or InstanceOf", path.display()),
            };
            if f[6].is_empty() {
                bail!("{}:{ln}: seed without a source", path.display());
            }
            let from = self.resolve_seed_concept(&f[2], &f[3]).with_context(|| format!("{}:{ln}", path.display()))?;
            let to = self.resolve_seed_concept(&f[4], &f[5]).with_context(|| format!("{}:{ln}", path.display()))?;
            self.add_extra(Extra { id: f[0].clone(), rel, from, to, source: f[6].clone() })?;
        }
        Ok(())
    }

    /// Extra edge; gate — IsA cycle.
    pub fn add_extra(&mut self, e: Extra) -> Result<()> {
        if self.extra.iter().any(|x| x.id == e.id) {
            bail!("seed {} a second time", e.id);
        }
        // a new edge from → to closes a cycle if from is already an ancestor of to (or the same concept)
        let anc = self.ancestors(e.to);
        if let Some(ch) = anc.chain(e.from).filter(|_| self.gates) {
            bail!("gate: IsA cycle through seed {}: {} —{}→ {}", e.id, chain_str_plain(&self.g, &ch), e.rel.name(), self.g.show(e.to));
        }
        let i = self.extra.len() as u32;
        self.extra_out.entry(e.from).or_default().push(i);
        self.extra.push(e);
        Ok(())
    }

    pub fn load_defaults(&mut self, path: &Path) -> Result<()> {
        for (ln, f) in tsv(path)? {
            let at = || format!("{}:{ln}", path.display());
            if f.len() < 8 {
                bail!("{}: need id, layer, lemma, synset, property, value, strength, source", at());
            }
            let layer = self.layer_id(&f[1]).with_context(|| format!("{}: unknown layer «{}»", at(), f[1]))?;
            let concept = self.resolve_seed_concept(&f[2], &f[3]).with_context(at)?;
            let prop = Prop::parse(&f[4]).with_context(|| format!("{}: unknown property «{}»", at(), f[4]))?;
            let value = match f[5].as_str() {
                "yes" => true,
                "no" => false,
                v => bail!("{}: value «{v}» — only yes or no", at()),
            };
            let strength = match f[6].as_str() {
                "default" => Strength::Default,
                "strict" => Strength::Strict,
                v => bail!("{}: strength «{v}» — only default or strict", at()),
            };
            if f[7].is_empty() {
                bail!("{}: seed without a source", at());
            }
            self.add_assert(Assert { id: f[0].clone(), layer, concept, prop, value, strength, source: f[7].clone(), note: f.get(8).cloned().unwrap_or_default() })
                .with_context(at)?;
        }
        Ok(())
    }

    /// A new seed with gates: duplicate id, contradiction on the same concept and layer, contradiction with Strict.
    pub fn add_assert(&mut self, a: Assert) -> Result<()> {
        if self.asserts.iter().any(|x| x.id == a.id) {
            bail!("seed {} a second time", a.id);
        }
        for x in &self.asserts {
            if x.layer == a.layer && x.prop == a.prop && x.concept == a.concept && x.value != a.value {
                bail!("gate: {} contradicts {} ({} {} on the same concept and layer)", a.id, x.id, self.g.show(a.concept), a.prop.name());
            }
        }
        // Strict: a descendant with the opposite value on the same layer — refusal (both ways)
        let anc_a = self.ancestors(a.concept);
        for x in self.asserts.iter().filter(|_| self.gates) {
            if x.layer != a.layer || x.prop != a.prop || x.value == a.value {
                continue;
            }
            if x.strength == Strength::Strict && anc_a.contains(x.concept) {
                bail!("gate: {} ({}) contradicts strict {} ({}) of an ancestor on the same layer", a.id, self.g.show(a.concept), x.id, self.g.show(x.concept));
            }
            if a.strength == Strength::Strict && self.ancestors(x.concept).contains(a.concept) {
                bail!("gate: strict {} ({}) contradicts existing {} of a descendant ({}) on the same layer", a.id, self.g.show(a.concept), x.id, self.g.show(x.concept));
            }
        }
        let i = self.asserts.len();
        self.by_concept.entry(a.concept).or_default().push(i);
        self.asserts.push(a);
        Ok(())
    }

    /// Steps upward from a concept: IsA/InstanceOf of the graph and of seeds.
    pub fn up(&self, c: Cid) -> Vec<Step> {
        let mut v: Vec<Step> = self.g.out_edges(c).iter().filter(|e| e.rel.is_taxonomic()).map(|e| Step { rel: e.rel, to: e.to, extra: None }).collect();
        if let Some(xs) = self.extra_out.get(&c) {
            for &i in xs {
                let e = &self.extra[i as usize];
                v.push(Step { rel: e.rel, to: e.to, extra: Some(i) });
            }
        }
        v
    }

    /// Ancestors of a concept (BFS — shortest chains).
    pub fn ancestors(&self, c: Cid) -> Anc {
        let mut map = HashMap::new();
        map.insert(c, (0u16, None));
        let mut q = VecDeque::from([c]);
        while let Some(x) = q.pop_front() {
            let d = map[&x].0;
            for st in self.up(x) {
                if let std::collections::hash_map::Entry::Vacant(e) = map.entry(st.to) {
                    e.insert((d + 1, Some((x, st))));
                    q.push_back(st.to);
                }
            }
        }
        Anc { start: c, map }
    }

    /// Gate: a cycle in IsA/InstanceOf (graph + seeds). Returns the cycle if there is one.
    pub fn isa_cycle(&self) -> Option<Vec<Cid>> {
        let n = self.g.n_concepts();
        // 0 — not seen, 1 — on the stack, 2 — done
        let mut color = vec![0u8; n];
        let mut parent = vec![u32::MAX; n];
        for s in 0..n as Cid {
            if color[s as usize] != 0 {
                continue;
            }
            let mut stack: Vec<(Cid, Vec<Step>, usize)> = vec![(s, self.up(s), 0)];
            color[s as usize] = 1;
            while let Some((x, ups, i)) = stack.last_mut() {
                if *i < ups.len() {
                    let t = ups[*i].to;
                    *i += 1;
                    let x = *x;
                    match color[t as usize] {
                        0 => {
                            color[t as usize] = 1;
                            parent[t as usize] = x;
                            let u = self.up(t);
                            stack.push((t, u, 0));
                        }
                        1 => {
                            // cycle: t … x → t
                            let mut cyc = vec![t];
                            let mut cur = x;
                            while cur != t && cyc.len() <= n {
                                cyc.push(cur);
                                cur = parent[cur as usize];
                            }
                            cyc.push(t);
                            cyc[1..].reverse();
                            return Some(cyc);
                        }
                        _ => {}
                    }
                } else {
                    color[*x as usize] = 2;
                    stack.pop();
                }
            }
        }
        None
    }

    /// A concept's property on a layer: most specific layer → most specific concept → Nixon diamond.
    pub fn prop(&self, c: Cid, p: Prop, layer: LayerId) -> PropAnswer {
        let anc = self.ancestors(c);
        // property seeds on the ancestors (including itself)
        let mut all: Vec<usize> = Vec::new();
        for x in anc.map.keys() {
            if let Some(v) = self.by_concept.get(x) {
                all.extend(v.iter().copied().filter(|&i| self.asserts[i].prop == p));
            }
        }
        all.sort_unstable();
        let chain = if self.leak_all_layers { (0..self.layers.len() as LayerId).rev().collect() } else { self.layer_chain(layer) };
        for l in chain {
            let cands: Vec<usize> = all.iter().copied().filter(|&i| self.asserts[i].layer == l).collect();
            if cands.is_empty() {
                continue;
            }
            // the most specific: the candidate's concept is not an ancestor of another candidate's concept
            let ancs: Vec<Anc> = cands.iter().map(|&i| self.ancestors(self.asserts[i].concept)).collect();
            let minimal: Vec<usize> = (0..cands.len())
                .filter(|&k| {
                    let ck = self.asserts[cands[k]].concept;
                    !(0..cands.len()).any(|j| j != k && self.asserts[cands[j]].concept != ck && ancs[j].contains(ck))
                })
                .collect();
            let first = self.asserts[cands[minimal[0]]].value;
            let beaten: Vec<usize> = (0..cands.len()).filter(|k| !minimal.contains(k)).map(|k| cands[k]).collect();
            if minimal.iter().all(|&k| self.asserts[cands[k]].value == first) {
                // the nearest among equals — for the chain
                let win = minimal.iter().copied().min_by_key(|&k| (anc.depth(self.asserts[cands[k]].concept).unwrap_or(u16::MAX), cands[k])).expect("there is a candidate");
                let chain = anc.chain(self.asserts[cands[win]].concept).expect("ancestor in the map");
                return PropAnswer::Known { value: first, win: cands[win], chain, beaten, layer: l };
            }
            let cands = minimal.iter().map(|&k| (cands[k], anc.chain(self.asserts[cands[k]].concept).expect("ancestor"))).collect();
            return PropAnswer::Undetermined { cands, layer: l };
        }
        PropAnswer::Unknown
    }

    /// Path `a ⊑ b` (a is a kind or instance of b), the shortest.
    pub fn isa_chain(&self, a: Cid, b: Cid) -> Option<Chain> {
        self.ancestors(a).chain(b)
    }
}
