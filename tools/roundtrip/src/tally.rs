//! Counters of residual items, bits and divergences, with a few examples each. Ordered maps only: merging the
//! volumes in their order gives the same numbers and the same examples on every run.

use std::collections::BTreeMap;

use crate::class::{Class, Layer};

/// How many examples a class keeps.
pub const KEEP: usize = 4;

#[derive(Clone, Debug, Default)]
pub struct Cell {
    pub items: u64,
    pub bits: f64,
    pub examples: Vec<String>,
    /// items by kind (the relation of a word the frames lack, the head tag of an order fix, the form of an escape …)
    pub kinds: BTreeMap<String, u64>,
    /// bits by kind
    pub kind_bits: BTreeMap<String, f64>,
}

impl Cell {
    fn add(&mut self, bits: f64, ex: Option<String>) {
        self.items += 1;
        self.bits += bits;
        if let Some(e) = ex {
            if self.examples.len() < KEEP {
                self.examples.push(e);
            }
        }
    }
    fn merge(&mut self, o: &Cell) {
        self.items += o.items;
        self.bits += o.bits;
        for (k, n) in &o.kinds {
            *self.kinds.entry(k.clone()).or_default() += n;
        }
        for (k, b) in &o.kind_bits {
            *self.kind_bits.entry(k.clone()).or_default() += b;
        }
        for e in &o.examples {
            if self.examples.len() < KEEP {
                self.examples.push(e.clone());
            }
        }
    }
}

/// The residual of the exact mode: per layer and class.
#[derive(Clone, Debug, Default)]
pub struct Residual {
    pub cells: BTreeMap<(Layer, Class), Cell>,
}

impl Residual {
    pub fn add(&mut self, l: Layer, c: Class, bits: f64, ex: impl FnOnce() -> String) {
        let cell = self.cells.entry((l, c)).or_default();
        let want = cell.examples.len() < KEEP;
        cell.add(bits, want.then(ex));
    }
    /// The same, with the item's kind counted.
    pub fn add_k(&mut self, l: Layer, c: Class, bits: f64, kind: String, ex: impl FnOnce() -> String) {
        self.add(l, c, bits, ex);
        if let Some(cell) = self.cells.get_mut(&(l, c)) {
            *cell.kind_bits.entry(kind.clone()).or_default() += bits;
            *cell.kinds.entry(kind).or_default() += 1;
        }
    }
    pub fn merge(&mut self, o: &Residual) {
        for (k, v) in &o.cells {
            self.cells.entry(*k).or_default().merge(v);
        }
    }
    #[cfg(test)]
    pub fn layer_bits(&self, l: Layer) -> f64 {
        self.cells.iter().filter(|(k, _)| k.0 == l).map(|(_, v)| v.bits).sum()
    }
}

/// The start of a normal-mode decode: which representation is given.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Start {
    Tokens,
    Tree,
    Frames,
    World,
    /// the same from a gold tree (a treebank's)
    GoldTree,
    GoldFrames,
    GoldWorld,
}

pub const STARTS: [Start; 7] = [Start::Tokens, Start::Tree, Start::Frames, Start::World, Start::GoldTree, Start::GoldFrames, Start::GoldWorld];

impl Start {
    pub fn name(self) -> &'static str {
        match self {
            Start::Tokens => "tokens",
            Start::Tree => "tree",
            Start::Frames => "frames",
            Start::World => "world",
            Start::GoldTree => "gold tree",
            Start::GoldFrames => "frames of gold tree",
            Start::GoldWorld => "world of gold tree",
        }
    }
}

/// Normal mode: divergence of the text decoded from one start.
#[derive(Clone, Debug, Default)]
pub struct Diverge {
    pub sents: u64,
    pub exact: u64,
    pub chrf: f64,
    pub tokens: u64,
    pub cells: BTreeMap<Class, Cell>,
}

impl Diverge {
    pub fn add(&mut self, c: Class, ex: impl FnOnce() -> String) {
        let cell = self.cells.entry(c).or_default();
        let want = cell.examples.len() < KEEP;
        cell.add(0.0, want.then(ex));
    }
    pub fn merge(&mut self, o: &Diverge) {
        self.sents += o.sents;
        self.exact += o.exact;
        self.chrf += o.chrf;
        self.tokens += o.tokens;
        for (k, v) in &o.cells {
            self.cells.entry(*k).or_default().merge(v);
        }
    }
}

/// A kind of piece the site decoder writes from the layers (`blocks::site_code`): how many, how many need nothing
/// stored but their fixed bits, and the bits of storing them whole against the bits of their code.
#[derive(Clone, Debug, Default)]
pub struct Derived {
    pub pieces: u64,
    pub free: u64,
    pub literal_bits: f64,
    pub code_bits: f64,
}

/// Everything one volume (or a set of them) measured.
#[derive(Clone, Debug, Default)]
pub struct Tally {
    pub volumes: u64,
    pub bytes: u64,
    pub sents: u64,
    pub tokens: u64,
    pub frames: u64,
    pub events: u64,
    /// volumes that came back byte for byte in the exact mode
    pub exact_ok: u64,
    /// first failure of the exact gate
    pub exact_fail: Vec<String>,
    /// sentences whose gold tree was used (tokens agree with the treebank's)
    pub gold_used: u64,
    pub gold_skipped: u64,
    pub residual: Residual,
    pub normal: BTreeMap<Start, Diverge>,
    /// normal mode, the whole volume as one text from each start (`sents` counts volumes)
    pub whole: BTreeMap<Start, Diverge>,
    /// volumes where the gate's negative control found a residual item to lose (`Opts::damage`)
    pub damaged: u64,
    /// mentions whose entity is a third-person pronoun itself (the reader did not resolve it)
    pub unresolved: u64,
    /// the site decoder's derived pieces by kind
    pub derived: BTreeMap<String, Derived>,
}

impl Tally {
    pub fn merge(&mut self, o: &Tally) {
        self.volumes += o.volumes;
        self.bytes += o.bytes;
        self.sents += o.sents;
        self.tokens += o.tokens;
        self.frames += o.frames;
        self.events += o.events;
        self.exact_ok += o.exact_ok;
        for e in &o.exact_fail {
            if self.exact_fail.len() < KEEP {
                self.exact_fail.push(e.clone());
            }
        }
        self.gold_used += o.gold_used;
        self.gold_skipped += o.gold_skipped;
        self.residual.merge(&o.residual);
        for (k, v) in &o.normal {
            self.normal.entry(*k).or_default().merge(v);
        }
        for (k, v) in &o.whole {
            self.whole.entry(*k).or_default().merge(v);
        }
        self.damaged += o.damaged;
        self.unresolved += o.unresolved;
        for (k, v) in &o.derived {
            let d = self.derived.entry(k.clone()).or_default();
            d.pieces += v.pieces;
            d.free += v.free;
            d.literal_bits += v.literal_bits;
            d.code_bits += v.code_bits;
        }
    }
}
#[cfg(test)]
mod wave32_tests {
    use super::*;
    #[test]
    fn capped_examples_additive_metrics() {
        let mut r = Residual::default();
        let calls = std::cell::Cell::new(0);
        for _ in 0..6 {
            r.add_k(Layer::Tokens, Class::Spacing, 2.0, "gap".into(), || {
                calls.set(calls.get() + 1);
                "x".into()
            });
        }
        assert_eq!(calls.get(), 4);
        let cell = &r.cells[&(Layer::Tokens, Class::Spacing)];
        assert_eq!((cell.items, cell.bits, cell.examples.len()), (6, 12.0, 4));
        let mut t = Tally {
            residual: r,
            volumes: 1,
            ..Default::default()
        };
        let copy = t.clone();
        t.merge(&copy);
        let c = &t.residual.cells[&(Layer::Tokens, Class::Spacing)];
        assert_eq!(
            (c.items, c.bits, c.kinds["gap"], c.examples.len()),
            (12, 24.0, 12, 4)
        );
        assert_eq!(t.volumes, 2);
    }
}
