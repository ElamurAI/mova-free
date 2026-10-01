//! SpartQA-human (Mirzaee et al. 2021; NC data — measurements only, nothing from it goes into the code): human descriptions
//! of scenes with blocks and shapes, questions YN / FB / CO / FR. A test of level 3 on live language.
//!
//! - **Mentions**: [quantifier/article] [size] [color] shape — words from level 1 categories
//!   (`global/seeds/shortcuts/scene.md`); a block is a standalone capital letter; «it / that» — the previous thing.
//!   An indefinite mention creates a thing, a definite one looks up a compatible already mentioned one (first in the current block).
//! - **Relations** — words between neighbouring mentions (left, above, near, touching…); fronted
//!   («Near and below X is Y») — Y relative to X. Touching a block edge is a separate fact.
//! - **Inference**: directions are transitive and have inverses (level 1, `relations.md`), between blocks — via
//!   block relations; nearness and touching are symmetric, not transitive.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

use anyhow::{Context, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum R {
    Left,
    Right,
    Above,
    Below,
    Near,
    Far,
    Touch,
}

impl R {
    fn inv(self) -> R {
        match self {
            R::Left => R::Right,
            R::Right => R::Left,
            R::Above => R::Below,
            R::Below => R::Above,
            x => x,
        }
    }
    fn directional(self) -> bool {
        matches!(self, R::Left | R::Right | R::Above | R::Below)
    }
    /// Word → relation (synonyms from level 1: top = above etc.).
    fn of(w: &str) -> Option<R> {
        Some(match w {
            "left" => R::Left,
            "right" => R::Right,
            "above" | "over" | "top" | "atop" | "up" | "upper" => R::Above,
            "below" | "under" | "beneath" | "bottom" | "underneath" | "lower" | "down" => R::Below,
            "near" | "close" | "next" | "nearby" | "beside" => R::Near,
            "far" | "away" => R::Far,
            "touching" | "touch" | "touches" | "touched" | "against" => R::Touch,
            _ => return None,
        })
    }
}

fn cat(name: &str, w: &str) -> bool {
    global::concept_words(name).contains(&w)
}

/// One edit (substitution, insertion, deletion) — «circe» → circle.
fn near_word(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    let (x, y): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    if x.len().abs_diff(y.len()) > 1 || x.len() < 4 {
        return false;
    }
    let (mut i, mut j, mut d) = (0, 0, 0);
    while i < x.len() && j < y.len() {
        if x[i] == y[j] {
            i += 1;
            j += 1;
            continue;
        }
        d += 1;
        if d > 1 {
            return false;
        }
        match x.len().cmp(&y.len()) {
            std::cmp::Ordering::Greater => i += 1,
            std::cmp::Ordering::Less => j += 1,
            _ => {
                i += 1;
                j += 1;
            }
        }
    }
    d + (x.len() - i) + (y.len() - j) <= 1
}

fn shape_of(w: &str) -> Option<String> {
    let sing = w.strip_suffix("es").filter(|s| s.ends_with('x') || s.ends_with("ss")).or_else(|| w.strip_suffix('s')).unwrap_or(w);
    global::concept_words("scene_shape").iter().find(|s| near_word(sing, s) || near_word(w, s)).map(|s| s.to_string())
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Desc {
    size: Option<String>,
    color: Option<String>,
    shape: Option<String>,
    block: Option<char>,
    plural: bool,
    /// a / an / any / some / another / number
    indef: bool,
    all: bool,
    neg: bool,
}

#[derive(Clone, Debug)]
enum Tok {
    W(String),
    Block(char),
    M(Desc),
    Pron,
}

#[derive(Clone, Debug)]
struct Obj {
    size: Option<String>,
    color: Option<String>,
    shape: Option<String>,
    block: Option<char>,
}

fn matches(o: &Obj, d: &Desc) -> bool {
    let f = |a: &Option<String>, b: &Option<String>| b.is_none() || a.is_none() || a == b;
    f(&o.size, &d.size) && f(&o.color, &d.color) && f(&o.shape, &d.shape) && d.block.is_none_or(|b| o.block == Some(b))
}

/// Strict match for questions: what the description lacks — anything; what it has — must match.
fn matches_q(o: &Obj, d: &Desc) -> bool {
    let f = |a: &Option<String>, b: &Option<String>| b.is_none() || a == b;
    f(&o.size, &d.size) && f(&o.color, &d.color) && f(&o.shape, &d.shape) && d.block.is_none_or(|b| o.block == Some(b))
}

/// Words and mentions of a sentence.
fn scan(text: &str) -> Vec<Vec<Tok>> {
    let mut sents = Vec::new();
    for raw in text.split(['.', '?', ';']) {
        let words: Vec<String> = raw.split(|c: char| c.is_whitespace() || c == ',').filter(|w| !w.is_empty()).map(String::from).collect();
        let mut out: Vec<Tok> = Vec::new();
        let mut i = 0;
        while i < words.len() {
            let orig = &words[i];
            let w = orig.to_lowercase();
            // block: a standalone capital letter (except the article «A» at the start before a description)
            if orig.len() == 1 && orig.chars().next().unwrap().is_ascii_uppercase() {
                let next = words.get(i + 1).map(|x| x.to_lowercase()).unwrap_or_default();
                let article = orig == "A" && (cat("scene_size", &next) || cat("scene_color", &next) || shape_of(&next).is_some() || cat("scene_any", &next) || next == "block" || next == "second" || next == "third");
                if !article {
                    out.push(Tok::Block(orig.chars().next().unwrap()));
                    i += 1;
                    continue;
                }
            }
            // description: [quantifier] [size] [color]… shape
            let mut j = i;
            let mut d = Desc::default();
            let q = w.as_str();
            if matches!(q, "a" | "an" | "any" | "some" | "another" | "one" | "two" | "three" | "four" | "five" | "2" | "3" | "4" | "5") {
                d.indef = true;
                j += 1;
            } else if matches!(q, "all" | "every" | "each") {
                d.all = true;
                j += 1;
                if words.get(j).is_some_and(|x| x.to_lowercase() == "of") {
                    j += 1;
                }
            } else if q == "no" {
                d.neg = true;
                j += 1;
            }
            if words.get(j).is_some_and(|x| x.to_lowercase() == "the") {
                j += 1;
            }
            let start_attr = j;
            while let Some(x) = words.get(j).map(|x| x.to_lowercase()) {
                if cat("scene_size", &x) {
                    d.size = Some(match x.as_str() {
                        "big" | "huge" => "large".into(),
                        "little" | "tiny" => "small".into(),
                        _ => x,
                    });
                } else if cat("scene_color", &x) {
                    d.color = Some(if x == "grey" { "gray".into() } else { x });
                } else {
                    break;
                }
                j += 1;
            }
            let head = words.get(j).map(|x| x.to_lowercase()).unwrap_or_default();
            let is_head = shape_of(&head).is_some() || cat("scene_any", &head);
            // «the small black in A» — a description without a noun
            if !is_head && j > start_attr && j > i && start_attr > i {
                d.plural = false;
                out.push(Tok::M(d));
                i = j;
                continue;
            }
            if is_head && (j > start_attr || j > i || head != "one") {
                d.shape = shape_of(&head);
                d.plural = head.ends_with('s') && head != "this";
                if head == "one" || head == "ones" {
                    d.shape = None;
                }
                // «… in A» — restriction to a block
                if words.get(j + 1).is_some_and(|x| x.to_lowercase() == "in") {
                    if let Some(b) = words.get(j + 2).filter(|x| x.len() == 1 && x.chars().next().unwrap().is_ascii_uppercase()) {
                        d.block = b.chars().next();
                        out.push(Tok::M(d));
                        i = j + 3;
                        continue;
                    }
                }
                out.push(Tok::M(d));
                i = j + 1;
                continue;
            }
            if matches!(w.as_str(), "it" | "that" | "this" | "them" | "they") {
                out.push(Tok::Pron);
            } else {
                out.push(Tok::W(w));
            }
            i += 1;
        }
        if !out.is_empty() {
            sents.push(out);
        }
    }
    sents
}

#[derive(Default)]
pub struct Scene {
    objs: Vec<Obj>,
    facts: BTreeSet<(usize, R, usize)>,
    blocks: BTreeSet<char>,
    bfacts: BTreeSet<(char, R, char)>,
    /// touching a block edge
    edge: BTreeSet<usize>,
}

/// Relation participant: a thing or a block.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Ent {
    O(usize),
    B(char),
}

impl Scene {
    pub fn read(text: &str) -> Scene {
        let mut sc = Scene::default();
        let mut cur: Option<char> = None;
        let mut last: Option<usize> = None;
        let mut last_shape: Option<String> = None;
        for sent in scan(text) {
            // block context: «In A», «Inside B», «block called A»
            for (k, t) in sent.iter().enumerate() {
                if let Tok::Block(b) = t {
                    sc.blocks.insert(*b);
                    let prev = k.checked_sub(1).and_then(|p| match &sent[p] {
                        Tok::W(w) => Some(w.as_str()),
                        _ => None,
                    });
                    if matches!(prev, Some("in" | "inside" | "within")) && !sent[..k].iter().any(|t| matches!(t, Tok::M(_))) {
                        cur = Some(*b);
                    }
                }
            }
            if sent.iter().any(|t| matches!(t, Tok::W(w) if w == "block" || w == "blocks")) {
                // a sentence about blocks — only relations between them
                let bl: Vec<(usize, char)> = sent.iter().enumerate().filter_map(|(k, t)| if let Tok::Block(b) = t { Some((k, *b)) } else { None }).collect();
                for w in bl.windows(2) {
                    let rels = rels_in(&sent[w[0].0 + 1..w[1].0]);
                    for r in rels.0 {
                        sc.bfacts.insert((w[0].1, r, w[1].1));
                    }
                }
                if let Some(&(_, b)) = bl.last() {
                    if sent.iter().any(|t| matches!(t, Tok::W(w) if w == "called" || w == "call" || w == "named")) && bl.len() == 1 {
                        cur = Some(b);
                    }
                }
                if sent.iter().all(|t| !matches!(t, Tok::M(_))) {
                    continue;
                }
            }
            // participants in text order
            let mut ents: Vec<(usize, Ent)> = Vec::new();
            for (k, t) in sent.iter().enumerate() {
                match t {
                    Tok::M(d) => {
                        let mut d = d.clone();
                        if d.shape.is_none() && !d.plural {
                            if let Some(Tok::W(h)) = sent.get(k).filter(|_| false) {
                                let _ = h;
                            }
                        }
                        if d.plural && d.indef {
                            // «3 squares» — a group: the shape for the following «a brown one»
                            last_shape = d.shape.clone();
                            continue;
                        }
                        if d.shape.is_none() && d.color.is_some() && last_shape.is_some() {
                            d.shape = last_shape.clone();
                        }
                        let definite = !d.indef;
                        let found = if definite {
                            let mut f = (0..sc.objs.len()).rev().find(|&o| sc.objs[o].block == cur && matches(&sc.objs[o], &d));
                            if f.is_none() {
                                f = (0..sc.objs.len()).rev().find(|&o| matches(&sc.objs[o], &d));
                            }
                            f
                        } else {
                            None
                        };
                        let o = found.unwrap_or_else(|| {
                            sc.objs.push(Obj { size: d.size.clone(), color: d.color.clone(), shape: d.shape.clone(), block: d.block.or(cur) });
                            sc.objs.len() - 1
                        });
                        // refinement from a repeated mention
                        let ob = &mut sc.objs[o];
                        if ob.size.is_none() {
                            ob.size = d.size.clone();
                        }
                        if ob.color.is_none() {
                            ob.color = d.color.clone();
                        }
                        if ob.shape.is_none() {
                            ob.shape = d.shape.clone();
                        }
                        ents.push((k, Ent::O(o)));
                        last = Some(o);
                    }
                    Tok::Pron => {
                        if let Some(o) = last {
                            ents.push((k, Ent::O(o)));
                        }
                    }
                    Tok::Block(b) => ents.push((k, Ent::B(*b))),
                    Tok::W(_) => {}
                }
            }
            // fronted: before the first participant — a relation, then «is/are» without relations
            let mut used_front = vec![false; ents.len()];
            for j in 0..ents.len() {
                let lo = if j == 0 { 0 } else { ents[j - 1].0 + 1 };
                let seg = &sent[lo..ents[j].0];
                let (rels, has_verb_before) = rels_in(seg);
                if rels.is_empty() || has_verb_before {
                    continue;
                }
                if let Some(next) = ents.get(j + 1) {
                    let seg2 = &sent[ents[j].0 + 1..next.0];
                    if rels_in(seg2).0.is_empty() && seg2.iter().any(|t| matches!(t, Tok::W(w) if w == "is" || w == "are" || w == "there" || w == "sits" || w == "lies")) {
                        for r in &rels {
                            sc.add(next.1, *r, ents[j].1, seg);
                        }
                        used_front[j] = true;
                    }
                }
            }
            for j in 1..ents.len() {
                if used_front[j] {
                    continue;
                }
                let seg = &sent[ents[j - 1].0 + 1..ents[j].0];
                let (rels, _) = rels_in(seg);
                for r in rels {
                    sc.add(ents[j - 1].1, r, ents[j].1, seg);
                }
            }
        }
        sc
    }

    fn add(&mut self, a: Ent, r: R, b: Ent, seg: &[Tok]) {
        let edge = seg.iter().any(|t| matches!(t, Tok::W(w) if w == "edge" || w == "edges" || w == "side" || w == "sides" || w == "wall"));
        match (a, b) {
            (Ent::O(x), Ent::O(y)) if x != y => {
                self.facts.insert((x, r, y));
            }
            (Ent::O(x), Ent::B(_)) => {
                if r == R::Touch || edge {
                    self.edge.insert(x);
                }
            }
            (Ent::B(x), Ent::B(y)) if x != y => {
                self.bfacts.insert((x, r, y));
            }
            _ => {}
        }
    }

    fn breach(&self, a: char, r: R, b: char) -> bool {
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([a]);
        while let Some(x) = q.pop_front() {
            for &(u, rr, v) in &self.bfacts {
                let step = if u == x && rr == r { Some(v) } else if v == x && rr.inv() == r && rr.directional() { Some(u) } else { None };
                if let Some(y) = step {
                    if y == b {
                        return true;
                    }
                    if seen.insert(y) {
                        q.push_back(y);
                    }
                }
            }
        }
        false
    }

    /// Whether «x r y» is inferred.
    pub fn holds(&self, x: usize, r: R, y: usize) -> bool {
        if x == y {
            return false;
        }
        let (bx, by) = (self.objs[x].block, self.objs[y].block);
        if !r.directional() {
            if self.facts.contains(&(x, r, y)) || self.facts.contains(&(y, r, x)) {
                return true;
            }
            // in different blocks — far
            return r == R::Far && bx.is_some() && by.is_some() && bx != by;
        }
        if let (Some(a), Some(b)) = (bx, by) {
            if a != b {
                return self.breach(a, r, b);
            }
        }
        // transitively within the facts
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([x]);
        while let Some(u) = q.pop_front() {
            for &(a, rr, b) in &self.facts {
                let step = if a == u && rr == r { Some(b) } else if b == u && rr.directional() && rr.inv() == r { Some(a) } else { None };
                if let Some(v) = step {
                    if v == y {
                        return true;
                    }
                    if seen.insert(v) {
                        q.push_back(v);
                    }
                }
            }
        }
        false
    }

    /// Path of direction r from x to y; Some(whether a «far» step was on the path), None — no path.
    fn dir_path(&self, x: usize, r: R, y: usize) -> Option<bool> {
        let mut seen: BTreeSet<(usize, bool)> = BTreeSet::new();
        let mut q = VecDeque::from([(x, false)]);
        let mut best: Option<bool> = None;
        while let Some((u, far)) = q.pop_front() {
            for &(a, rr, b) in &self.facts {
                let v = if a == u && rr == r { b } else if b == u && rr.directional() && rr.inv() == r { a } else { continue };
                let f = far || self.facts.contains(&(u, R::Far, v)) || self.facts.contains(&(v, R::Far, u));
                if v == y {
                    best = Some(best.unwrap_or(false) || f);
                }
                if seen.insert((v, f)) {
                    q.push_back((v, f));
                }
            }
        }
        best
    }

    fn far(&self, x: usize, y: usize) -> bool {
        let (bx, by) = (self.objs[x].block, self.objs[y].block);
        self.facts.contains(&(x, R::Far, y)) || self.facts.contains(&(y, R::Far, x)) || bx.is_some() && by.is_some() && bx != by
            || [R::Left, R::Right, R::Above, R::Below].iter().any(|&r| self.dir_path(x, r, y) == Some(true))
    }

    fn near(&self, x: usize, y: usize) -> bool {
        [R::Near, R::Touch].iter().any(|&r| self.facts.contains(&(x, r, y)) || self.facts.contains(&(y, r, x)))
    }

    /// Three-valued: Some(true) — inferred, Some(false) — the opposite is inferred, None — unknown.
    pub fn tv(&self, x: usize, r: R, y: usize) -> Option<bool> {
        if x == y {
            return Some(false);
        }
        match r {
            R::Near => {
                if self.near(x, y) {
                    Some(true)
                } else if self.far(x, y) {
                    Some(false)
                } else {
                    None
                }
            }
            R::Far => {
                if self.far(x, y) {
                    Some(true)
                } else if self.near(x, y) {
                    Some(false)
                } else {
                    None
                }
            }
            R::Touch => {
                if self.facts.contains(&(x, R::Touch, y)) || self.facts.contains(&(y, R::Touch, x)) {
                    Some(true)
                } else if self.far(x, y) {
                    Some(false)
                } else {
                    None
                }
            }
            _ => {
                if self.holds(x, r, y) {
                    Some(true)
                } else if self.holds(x, r.inv(), y) {
                    Some(false)
                } else {
                    None
                }
            }
        }
    }

    /// Several relations together («near and above»): all yes — yes; any no — no.
    fn tv_all(&self, x: usize, rels: &[R], y: usize) -> Option<bool> {
        let v: Vec<Option<bool>> = rels.iter().map(|&r| self.tv(x, r, y)).collect();
        if v.contains(&Some(false)) {
            Some(false)
        } else if v.iter().all(|t| *t == Some(true)) {
            Some(true)
        } else {
            None
        }
    }

    fn find(&self, d: &Desc) -> Vec<usize> {
        (0..self.objs.len()).filter(|&o| matches_q(&self.objs[o], d)).collect()
    }
}

/// Relations in a fragment: (relation, whether «is/are» stands before the relation word).
fn rels_in(seg: &[Tok]) -> (Vec<R>, bool) {
    let mut out = Vec::new();
    let mut verb_before = false;
    for t in seg {
        if let Tok::W(w) = t {
            if out.is_empty() && matches!(w.as_str(), "is" | "are" | "sits" | "lies" | "stands" | "appears" | "located") {
                verb_before = true;
            }
            if let Some(r) = R::of(w) {
                if !out.contains(&r) {
                    out.push(r);
                }
            }
        }
    }
    (out, verb_before)
}

/// Question → (subject, relation, object).
fn q_parts(q: &str) -> Option<(Desc, Vec<R>, Desc)> {
    let sents = scan(q);
    let toks: Vec<Tok> = sents.into_iter().flatten().collect();
    let ms: Vec<(usize, Desc)> = toks.iter().enumerate().filter_map(|(k, t)| if let Tok::M(d) = t { Some((k, d.clone())) } else { None }).collect();
    if ms.len() < 2 {
        return None;
    }
    let (rels, _) = rels_in(&toks[ms[0].0 + 1..ms[1].0]);
    Some((ms[0].1.clone(), rels, ms[1].1.clone()))
}

fn yn(sc: &Scene, q: &str) -> String {
    let toks: Vec<Tok> = scan(q).into_iter().flatten().collect();
    let ms: Vec<(usize, Desc)> = toks.iter().enumerate().filter_map(|(k, t)| if let Tok::M(d) = t { Some((k, d.clone())) } else { None }).collect();
    let words: Vec<&str> = toks.iter().filter_map(|t| if let Tok::W(w) = t { Some(w.as_str()) } else { None }).collect();
    let Some((k0, a)) = ms.first().cloned() else { return "DK".into() };
    // «Are the large yellow things in A near each other?» — pairs within one set
    let each_other = words.windows(2).any(|w| w == ["each", "other"]);
    let (b, seg_end) = match ms.get(1) {
        Some((k1, d)) => (d.clone(), *k1),
        None if each_other => (a.clone(), toks.len()),
        None => return "DK".into(),
    };
    let (rels, _) = rels_in(&toks[k0 + 1..seg_end]);
    let xs = sc.find(&a);
    let mut ys = sc.find(&b);
    // «the other small yellow thing» — excluding the subject itself
    if words.contains(&"other") && xs.len() == 1 {
        ys.retain(|y| *y != xs[0]);
    }
    if xs.is_empty() || ys.is_empty() || rels.is_empty() {
        return "DK".into();
    }
    let pairs: Vec<Option<bool>> = if each_other {
        let mut v = Vec::new();
        for &x in &xs {
            for &y in &ys {
                if x < y {
                    v.push(sc.tv_all(x, &rels, y));
                }
            }
        }
        if v.is_empty() {
            return "DK".into();
        }
        return if v.iter().all(|t| *t == Some(true)) { "Yes".into() } else { "No".into() };
    } else {
        Vec::new()
    };
    let _ = pairs;
    let sub_all = a.all || a.plural && !a.indef;
    let obj_all = b.all || b.plural && !b.indef;
    // over the object: all / at least one; then over the subject the same way
    let over_y = |x: usize| -> Option<bool> {
        let v: Vec<Option<bool>> = ys.iter().map(|&y| sc.tv_all(x, &rels, y)).collect();
        if obj_all {
            if v.iter().all(|t| *t == Some(true)) { Some(true) } else if v.contains(&Some(false)) { Some(false) } else { None }
        } else if v.contains(&Some(true)) {
            Some(true)
        } else if v.iter().all(|t| *t == Some(false)) {
            Some(false)
        } else {
            None
        }
    };
    let v: Vec<Option<bool>> = xs.iter().map(|&x| over_y(x)).collect();
    let res = if sub_all {
        if v.iter().all(|t| *t == Some(true)) { Some(true) } else if v.contains(&Some(false)) { Some(false) } else { None }
    } else if v.contains(&Some(true)) {
        Some(true)
    } else if v.iter().all(|t| *t == Some(false)) {
        Some(false)
    } else {
        None
    };
    // measured on the training split: the gold is almost closed-world (DK — 16 of 161, mostly when the thing is absent),
    // so «not inferred» → No
    match res {
        Some(true) => "Yes",
        _ => "No",
    }
    .into()
}

fn fb(sc: &Scene, q: &str) -> Vec<String> {
    let toks: Vec<Tok> = scan(q).into_iter().flatten().collect();
    let words: Vec<String> = toks.iter().filter_map(|t| if let Tok::W(w) = t { Some(w.clone()) } else { None }).collect();
    let neg = words.iter().any(|w| matches!(w.as_str(), "not" | "doesn't" | "no" | "none"));
    let d = toks.iter().find_map(|t| if let Tok::M(d) = t { Some(d.clone()) } else { None }).unwrap_or_default();
    let edge = words.iter().any(|w| w == "edge" || w == "edges");
    let all_q = d.all || words.iter().any(|w| w == "all");
    let mut out = Vec::new();
    for &b in &sc.blocks {
        let mut dd = d.clone();
        dd.block = None;
        let objs: Vec<usize> = sc.find(&dd).into_iter().filter(|&o| !edge || sc.edge.contains(&o)).collect();
        let inb = objs.iter().filter(|&&o| sc.objs[o].block == Some(b)).count();
        let has = if all_q { inb > 0 && inb == objs.len() } else { inb > 0 };
        if has != neg {
            out.push(b.to_string());
        }
    }
    out
}

fn co(sc: &Scene, q: &str, cands: &[String]) -> usize {
    // «What object is to the left of the X, the A or the B?»: the first mention is «what object», the second is the reference
    let toks: Vec<Tok> = scan(q).into_iter().flatten().collect();
    let ms: Vec<(usize, Desc)> = toks.iter().enumerate().filter_map(|(k, t)| if let Tok::M(d) = t { Some((k, d.clone())) } else { None }).collect();
    if ms.len() < 2 {
        return 3;
    }
    let seg = &toks[ms[0].0 + 1..ms[1].0];
    let (rels, _) = rels_in(seg);
    let neg = seg.iter().any(|t| matches!(t, Tok::W(w) if w == "not"));
    if rels.is_empty() {
        return 3;
    }
    let ys = sc.find(&ms[1].1);
    let test = |c: &str| -> bool {
        let d = scan(c).into_iter().flatten().find_map(|t| if let Tok::M(d) = t { Some(d) } else { None });
        let Some(d) = d else { return false };
        let xs = sc.find(&d);
        if xs.is_empty() || ys.is_empty() {
            return false;
        }
        if neg {
            xs.iter().any(|&x| ys.iter().any(|&yy| sc.tv_all(x, &rels, yy) == Some(false)))
        } else {
            xs.iter().any(|&x| ys.iter().any(|&yy| sc.tv_all(x, &rels, yy) == Some(true)))
        }
    };
    let (a, b) = (cands.first().map(|c| test(c)).unwrap_or(false), cands.get(1).map(|c| test(c)).unwrap_or(false));
    match (a, b) {
        (true, false) => 0,
        (false, true) => 1,
        (true, true) => 2,
        _ => 3,
    }
}

fn fr(sc: &Scene, q: &str, cands: &[String]) -> Vec<usize> {
    let toks: Vec<Tok> = scan(q).into_iter().flatten().collect();
    let ms: Vec<Desc> = toks.iter().filter_map(|t| if let Tok::M(d) = t { Some(d.clone()) } else { None }).collect();
    let dk = cands.iter().position(|c| c.trim() == "DK");
    if ms.len() < 2 {
        return dk.into_iter().collect();
    }
    let (xs, ys) = (sc.find(&ms[0]), sc.find(&ms[1]));
    // several things under the description: relations shared by all pairs about which something is inferred
    let cand_r: Vec<(usize, R)> = cands.iter().enumerate().filter_map(|(i, c)| c.split_whitespace().next().and_then(R::of).map(|r| (i, r))).collect();
    let mut out: Option<Vec<usize>> = None;
    for &x in &xs {
        for &y in &ys {
            if x == y {
                continue;
            }
            let v: Vec<usize> = cand_r.iter().filter(|(_, r)| sc.tv(x, *r, y) == Some(true)).map(|x| x.0).collect();
            if v.is_empty() {
                continue;
            }
            out = Some(match out {
                None => v,
                Some(o) => o.into_iter().filter(|i| v.contains(i)).collect(),
            });
        }
    }
    let mut out = out.unwrap_or_default();
    if out.is_empty() {
        out.extend(dk);
    }
    out
}

/// Run over a file: accuracy by type.
pub fn eval(path: &Path, show: usize) -> Result<()> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let v: serde_json::Value = serde_json::from_str(&text)?;
    let mut res: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut shown = 0;
    for st in v["data"].as_array().cloned().unwrap_or_default() {
        let story = st["story"].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join(" ")).unwrap_or_default();
        let sc = Scene::read(&story);
        for q in st["questions"].as_array().cloned().unwrap_or_default() {
            let t = q["q_type"].as_str().unwrap_or("").to_string();
            let qs = q["question"].as_str().unwrap_or("");
            let gold = &q["answer"];
            let cands: Vec<String> = q["candidate_answers"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
            let (pred, ok) = match t.as_str() {
                "YN" => {
                    let p = yn(&sc, qs);
                    let ok = gold.as_array().and_then(|a| a.first()).and_then(|x| x.as_str()) == Some(p.as_str());
                    (p, ok)
                }
                "FB" => {
                    let mut p = fb(&sc, qs);
                    p.sort();
                    let mut g: Vec<String> = gold.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
                    g.sort();
                    (format!("{p:?}"), p == g)
                }
                "CO" => {
                    let p = co(&sc, qs, &cands);
                    let ok = gold.as_array().and_then(|a| a.first()).and_then(|x| x.as_u64()) == Some(p as u64);
                    (p.to_string(), ok)
                }
                "FR" => {
                    let mut p = fr(&sc, qs, &cands);
                    p.sort();
                    let mut g: Vec<usize> = gold.as_array().map(|a| a.iter().filter_map(|x| x.as_u64().map(|n| n as usize)).collect()).unwrap_or_default();
                    g.sort();
                    (format!("{p:?}"), p == g)
                }
                _ => continue,
            };
            let e = res.entry(t.clone()).or_default();
            e.1 += 1;
            if ok {
                e.0 += 1;
            } else if shown < show {
                shown += 1;
                println!("  {t} {qs} → {pred} (expected {gold})");
                if std::env::var("SPARTQA_SCENE").is_ok() {
                    println!("    {story}");
                    for (i, o) in sc.objs.iter().enumerate() {
                        print!("    #{i} {} {} {} {:?};", o.size.as_deref().unwrap_or("-"), o.color.as_deref().unwrap_or("-"), o.shape.as_deref().unwrap_or("-"), o.block);
                    }
                    println!("\n    facts {:?} blocks {:?} edge {:?}", sc.facts, sc.bfacts, sc.edge);
                }
            }
        }
        if std::env::var("SPARTQA_DEBUG").is_ok_and(|f| story.contains(&f)) {
            println!("{story}");
            for (i, o) in sc.objs.iter().enumerate() {
                println!("  #{i} {:?} {:?} {:?} in {:?}", o.size, o.color, o.shape, o.block);
            }
            println!("  facts {:?}\n  blocks {:?}", sc.facts, sc.bfacts);
        }
    }
    let (mut a, mut n) = (0, 0);
    for (t, (ok, all)) in &res {
        println!("{t}: {ok}/{all} ({:.1}%)", 100.0 * *ok as f64 / *all as f64);
        a += ok;
        n += all;
    }
    println!("total {a}/{n} ({:.1}%)", 100.0 * a as f64 / n as f64);
    Ok(())
}
