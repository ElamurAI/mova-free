//! SpartQA-human (Mirzaee et al. 2021; NC data — measurements only, nothing from it goes into the code): human descriptions
//! of scenes with blocks and shapes, questions YN / FB / CO / FR. A test of level 3 on live language.
//!
//! - **Mentions**: [quantifier/article] [size] [color] shape — words from level 1 categories
//!   (`global/seeds/shortcuts/scene.md`), one-edit typos tolerated; a block is a standalone capital letter
//!   (lower case after «block» / at the end after «in»); «it / that» — the previous thing; «this block» — the current block.
//!   An indefinite mention creates a thing («two circles» — two things), a definite one looks up a compatible already
//!   mentioned one (first in the current block); a definite plural («both blue objects», «the circles») is a group.
//! - **Relations** — words between neighbouring mentions (left, above, near, touching…); fronted
//!   («Near and below X is Y») — Y relative to X; coordinated («X is touching A and is near Y») — from the clause subject.
//!   Touching a block edge is a separate fact with its side (left / right / top / bottom).
//! - **Question phrases**: a mention with modifiers — «which/that is R …», a bare «R …» in phrase positions,
//!   «touching the bottom edge of block B», «between two circles»; a phrase denotes the set of things satisfying it.
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
            // one-edit typos of the long relation words («aove», «toucing»)
            _ => {
                let colour_like = global::concept_words("scene_color").iter().any(|c| near_word(w, c));
                return ["above", "below", "touching", "beneath", "underneath"].iter().find(|r| w.len() >= 4 && w != "blow" && !colour_like && near_word(w, r)).and_then(|r| R::of(r));
            }
        })
    }
}

fn cat(name: &str, w: &str) -> bool {
    global::concept_words(name).contains(&w)
}

/// One edit (substitution, insertion, deletion, or a swap of neighbours in words of 5+ letters) — «circe», «sqaure» → …
fn near_word(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    let (x, y): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    if x.len().abs_diff(y.len()) > 1 || x.len() < 4 {
        return false;
    }
    if x.len() == y.len() && x.len() >= 5 {
        let diff: Vec<usize> = (0..x.len()).filter(|&k| x[k] != y[k]).collect();
        if diff.len() == 2 && diff[1] == diff[0] + 1 && x[diff[0]] == y[diff[1]] && x[diff[1]] == y[diff[0]] {
            return true;
        }
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

/// Frequent words one edit away from a colour or size word («block» — «black», «while» — «white»): never typos.
const NOT_TYPOS: &[&str] = &["block", "blocks", "bock", "blank", "while", "write", "fellow", "shall", "greed", "crown", "medial"];

/// Size or colour word → (is size, normalised value); one-edit typos of sizes (4+ letters) and colours (5+ letters).
fn attr_of(w: &str) -> Option<(bool, String)> {
    let norm = |w: &str| -> Option<(bool, String)> {
        if cat("scene_size", w) {
            return Some((true, match w {
                "big" | "huge" => "large",
                "little" | "tiny" => "small",
                _ => w,
            }
            .to_string()));
        }
        if w == "midsize" || w == "mid-sized" || w == "midsized" {
            return Some((true, "medium".into()));
        }
        if cat("scene_color", w) {
            return Some((false, if w == "grey" { "gray".into() } else { w.into() }));
        }
        None
    };
    if let Some(a) = norm(w) {
        return Some(a);
    }
    if NOT_TYPOS.contains(&w) || R::of(w).is_some() {
        return None;
    }
    if w.len() >= 4 {
        if let Some(s) = global::concept_words("scene_size").iter().find(|s| s.len() >= 4 && near_word(w, s)) {
            return norm(s);
        }
    }
    if w.len() >= 5 {
        if let Some(c) = global::concept_words("scene_color").iter().find(|c| c.len() >= 4 && near_word(w, c)) {
            return norm(c);
        }
    }
    None
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
    /// «two circles» — how many
    count: usize,
    /// «the second circle», «the circle number two»
    ordinal: Option<usize>,
    /// «another», «other»
    other: bool,
    /// «one of the triangles»
    one_of: bool,
    /// «two big and medium triangles»: attributes of the further members (is size, value)
    alt: Vec<(bool, String)>,
    /// «two of the circles»
    partitive: bool,
    /// «one of them», «the other» — out of the last group
    of_group: bool,
}

#[derive(Clone, Debug)]
enum Tok {
    W(String),
    Block(char),
    M(Desc),
    Pron,
    /// «this block», «the block», «a block» — the block in context (in questions — any block)
    CurBlock,
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

/// A plural with coordinated attributes («the yellow and black objects») — one description per attribute.
fn variants(d: &Desc) -> Vec<Desc> {
    let mut out = vec![d.clone()];
    if d.plural || d.all {
        for (is_size, v) in &d.alt {
            let mut e = d.clone();
            *(if *is_size { &mut e.size } else { &mut e.color }) = Some(v.clone());
            out.push(e);
        }
    }
    out
}

fn number(w: &str) -> Option<usize> {
    Some(match w {
        "one" | "1" | "first" => 1,
        "two" | "2" | "second" => 2,
        "three" | "3" | "third" => 3,
        "four" | "4" | "fourth" => 4,
        "five" | "5" | "fifth" => 5,
        _ => return None,
    })
}

/// «first», «second», «third» (one-edit typos too: «fist»).
fn ordinal_of(w: &str) -> Option<usize> {
    ["first", "second", "third", "fourth", "fifth"].iter().position(|o| near_word(w, o) && (w.len() >= 4 || w == *o)).map(|p| p + 1)
}

fn is_block_letter(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    (s.len() == 1 && c.is_ascii_uppercase() && c != 'I').then_some(c)
}

/// Words of a sentence: split on spaces, commas, brackets, quotes; «ablue» → «a blue» (a glued article).
fn words_of(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    for w in raw.split(|c: char| c.is_whitespace() || matches!(c, ',' | '(' | ')' | '"' | ':' | '!')).filter(|w| !w.is_empty()) {
        let low = w.to_lowercase();
        if let Some(rest) = low.strip_prefix('a').filter(|r| r.len() >= 3 && (cat("scene_color", r) || cat("scene_size", r))) {
            out.push("a".to_string());
            out.push(rest.to_string());
        } else {
            out.push(w.to_string());
        }
    }
    out
}

/// Words and mentions of a sentence.
fn scan(text: &str) -> Vec<Vec<Tok>> {
    let mut sents = Vec::new();
    for raw in text.split(['.', '?', ';']) {
        let words = words_of(raw);
        let low: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
        let mut out: Vec<Tok> = Vec::new();
        let mut i = 0;
        // a block letter at position k: a capital; a lower-case one after «block», or after «in» at the end
        let block_at = |k: usize| -> Option<char> {
            let w = words.get(k)?;
            if let Some(c) = is_block_letter(w) {
                return Some(c);
            }
            let c = w.chars().next()?;
            let prev = k.checked_sub(1).map(|p| low[p].as_str());
            (w.len() == 1 && c.is_ascii_lowercase() && c != 'i' && (prev == Some("block") || prev == Some("in") && k + 1 == words.len()))
                .then(|| c.to_ascii_uppercase())
        };
        while i < words.len() {
            let orig = &words[i];
            let w = low[i].clone();
            // block: a standalone capital letter (except the article «A» before a description)
            if let Some(b) = block_at(i) {
                let next = low.get(i + 1).cloned().unwrap_or_default();
                let article = orig == "A"
                    && (attr_of(&next).is_some() || shape_of(&next).is_some() || cat("scene_any", &next) || matches!(next.as_str(), "block" | "second" | "third" | "few"));
                if !article {
                    out.push(Tok::Block(b));
                    i += 1;
                    continue;
                }
            }
            // «this block», «the block», «a block» without a letter — the block in context
            if matches!(w.as_str(), "this" | "that" | "the" | "a" | "its" | "same" | "each" | "any") && low.get(i + 1).is_some_and(|x| x == "block") && block_at(i + 2).is_none() {
                out.push(Tok::CurBlock);
                i += 2;
                continue;
            }
            // description: [quantifier] [the] [size] [color]… shape [number k] [in [block] X]
            let mut j = i;
            let mut d = Desc::default();
            let q = w.as_str();
            if q == "one" && low.get(i + 1).is_some_and(|x| x == "of") && low.get(i + 2).is_some_and(|x| matches!(x.as_str(), "them" | "these" | "those")) {
                // «one of them» — a member of the last group
                d.one_of = true;
                d.of_group = true;
                out.push(Tok::M(d));
                i += 3;
                continue;
            } else if q == "one" && low.get(i + 1).is_some_and(|x| x == "of") {
                // «one of the triangles» — a single member of the group
                d.indef = true;
                d.one_of = true;
                j += 2;
            } else if number(q).is_some_and(|n| n >= 2) && low.get(i + 1).is_some_and(|x| x == "of") {
                // «two of the circles» — a part of a group
                d.count = number(q).unwrap_or(0);
                d.partitive = true;
                j += 2;
            } else if matches!(q, "a" | "an" | "any" | "some" | "another") || number(q).is_some_and(|_| !matches!(q, "first" | "second" | "third" | "fourth" | "fifth")) {
                d.indef = true;
                d.other = q == "another";
                if q != "one" && q != "1" {
                    d.count = number(q).unwrap_or(0);
                }
                j += 1;
            } else if matches!(q, "all" | "every" | "each" | "both") {
                d.all = true;
                j += 1;
                if low.get(j).is_some_and(|x| x == "of") {
                    j += 1;
                }
            } else if q == "no" {
                d.neg = true;
                j += 1;
            }
            if low.get(j).is_some_and(|x| x == "the" || x == "these" || x == "those") {
                j += 1;
            }
            if low.get(j).is_some_and(|x| x == "other") {
                d.other = true;
                j += 1;
            }
            if let Some(n) = low.get(j).and_then(|x| ordinal_of(x)) {
                d.ordinal = Some(n);
                j += 1;
            }
            let start_attr = j;
            while let Some(x) = low.get(j) {
                if let Some((is_size, v)) = attr_of(x) {
                    if is_size {
                        d.size = Some(v);
                    } else {
                        d.color = Some(v);
                    }
                } else if x == "and" && j > start_attr && low.get(j + 1).is_some_and(|y| attr_of(y).is_some()) {
                    // «two big and medium triangles» — the second member gets the second attribute
                    if let Some(a) = low.get(j + 1).and_then(|y| attr_of(y)) {
                        d.alt.push(a);
                    }
                    j += 2;
                    continue;
                } else {
                    break;
                }
                j += 1;
            }
            let head = low.get(j).cloned().unwrap_or_default();
            let is_head = shape_of(&head).is_some() || cat("scene_any", &head);
            // «the other», «another» without a noun — the rest of the last group
            if !is_head && d.other && j == start_attr && !matches!(head.as_str(), "block" | "blocks") {
                d.of_group = true;
                out.push(Tok::M(d));
                i = j;
                continue;
            }
            // «the small black in A» — a description without a noun
            if !is_head && j > start_attr && j > i && start_attr > i && block_at(j).is_none() {
                out.push(Tok::M(d));
                i = j;
                continue;
            }
            if is_head && (j > start_attr || j > i || head != "one") {
                d.shape = shape_of(&head);
                d.plural = (head.ends_with('s') && head != "this" || d.count >= 2) && !d.one_of;
                if head == "one" || head == "ones" {
                    d.shape = None;
                }
                let mut k = j + 1;
                // «the square number two»
                if low.get(k).is_some_and(|x| x == "number") {
                    if let Some(n) = low.get(k + 1).and_then(|x| number(x)) {
                        d.ordinal = Some(n);
                        k += 2;
                    }
                }
                // «… in A», «… in block A», «… of block A» — restriction to a block
                if low.get(k).is_some_and(|x| x == "in" || x == "of" || x == "inside") {
                    let kb = if low.get(k + 1).is_some_and(|x| x == "block" || x == "the") { k + 2 } else { k + 1 };
                    let kb = if low.get(kb).is_some_and(|x| x == "block") { kb + 1 } else { kb };
                    if let Some(b) = block_at(kb).filter(|_| low[k] != "of" || low[k + 1] == "block") {
                        d.block = Some(b);
                        k = kb + 1;
                    }
                }
                out.push(Tok::M(d));
                i = k;
                continue;
            }
            // «that» right after a mention opens a relative clause, it is not a pronoun
            let relative = w == "that" && matches!(out.last(), Some(Tok::M(_)));
            if matches!(w.as_str(), "it" | "that" | "this" | "them" | "they") && !relative {
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
    /// touching a block edge: (thing, side — None when not said)
    edge: BTreeSet<(usize, Option<R>)>,
}

/// Relation participant: a thing, a group of things or a block.
#[derive(Clone, Debug, PartialEq)]
enum Ent {
    O(usize),
    G(Vec<usize>),
    B(char),
}

impl Ent {
    fn objs(&self) -> Vec<usize> {
        match self {
            Ent::O(o) => vec![*o],
            Ent::G(v) => v.clone(),
            Ent::B(_) => vec![],
        }
    }
}

/// Words allowed before a block letter that sets the block context («In block A, …», «Block B has …», «And in C …»).
const CONTEXT_LEAD: &[&str] = &["in", "inside", "within", "block", "and", "then", "also", "the", "for", "now", "finally", "lastly", "last", "first", "second", "third", "is"];

impl Scene {
    pub fn read(text: &str) -> Scene {
        let mut sc = Scene::default();
        let mut cur: Option<char> = None;
        let mut last: Option<usize> = None;
        let mut last_shape: Option<String> = None;
        // things of a counted group not yet singled out («three circles … a red one»)
        let mut pending: Vec<usize> = Vec::new();
        let mut prev_subj: Option<usize> = None;
        // things singled out of a group («one of the circles», «the first circle») — «the other» is not them
        let mut singled: Vec<usize> = Vec::new();
        // the last group of things («two yellow circles», «both blue objects»)
        let mut last_group: Vec<usize> = Vec::new();
        // the block the previous sentence was about («Block B is right of A. It has …»)
        let mut topic: Option<char> = None;
        for sent in scan(text) {
            let has_m = sent.iter().any(|t| matches!(t, Tok::M(_)));
            let first = sent.iter().position(|t| !matches!(t, Tok::W(w) if w == "block" || w == "the" || w == "and"));
            let it_has = matches!(first.map(|f| &sent[f]), Some(Tok::Pron)) && matches!(first.and_then(|f| sent.get(f + 1)), Some(Tok::W(w)) if matches!(w.as_str(), "has" | "have" | "contains" | "includes" | "holds"));
            let mut sent = sent;
            if it_has {
                if let Some(b) = topic {
                    cur = Some(b);
                }
                sent.remove(first.unwrap());
            }
            topic = match first.map(|f| &sent[f]) {
                Some(Tok::Block(b)) if !it_has => Some(*b),
                _ => if it_has { topic } else { None },
            };
            // block context: «In A», «Inside B», «Block C has», «block called A»
            for (k, t) in sent.iter().enumerate() {
                if let Tok::Block(b) = t {
                    sc.blocks.insert(*b);
                    let lead = sent[..k].iter().all(|t| matches!(t, Tok::W(w) if CONTEXT_LEAD.contains(&w.as_str())));
                    let before_m = !sent[..k].iter().any(|t| matches!(t, Tok::M(_)));
                    let in_lead = sent[..k].iter().any(|t| matches!(t, Tok::W(w) if matches!(w.as_str(), "in" | "inside" | "within")));
                    if lead && before_m && (has_m || in_lead) && (k > 0 || sent.len() > 1) {
                        cur = Some(*b);
                    }
                }
            }
            if sent.iter().any(|t| matches!(t, Tok::W(w) if w == "block" || w == "blocks") || matches!(t, Tok::CurBlock)) {
                // a sentence about blocks — relations between them
                let bl: Vec<(usize, char)> = sent.iter().enumerate().filter_map(|(k, t)| if let Tok::Block(b) = t { Some((k, *b)) } else { None }).collect();
                let mut prev: Option<(char, Vec<R>)> = None;
                for w in bl.windows(2) {
                    let seg = &sent[w[0].0 + 1..w[1].0];
                    let (rels, _) = rels_in(seg);
                    // «C is below block A and B» — a list continues the previous relation
                    if rels.is_empty() && seg.iter().all(|t| matches!(t, Tok::W(x) if x == "and" || x == "block")) {
                        if let Some((l, rs)) = &prev {
                            for &r in rs {
                                if *l != w[1].1 {
                                    sc.bfacts.insert((*l, r, w[1].1));
                                }
                            }
                        }
                        continue;
                    }
                    // «B is to the right of A and also is above C» — a coordinated predicate of the first block
                    let left = if matches!(seg.first(), Some(Tok::W(x)) if x == "and") && w[0].0 != bl[0].0 { bl[0].1 } else { w[0].1 };
                    for &r in &rels {
                        if left != w[1].1 {
                            sc.bfacts.insert((left, r, w[1].1));
                        }
                    }
                    prev = (!rels.is_empty()).then(|| (left, rels));
                }
                // an introduced block becomes the context: «block called A», «There is another block C …»
                let intro = |k: usize| matches!(k.checked_sub(1).map(|p| &sent[p]), Some(Tok::W(w)) if matches!(w.as_str(), "called" | "call" | "named" | "block"));
                let naming = |t: Option<&Tok>| matches!(t, Some(Tok::W(w)) if matches!(w.as_str(), "called" | "call" | "named" | "name"));
                // «called A», «we call it B»
                let named: Vec<char> = bl
                    .iter()
                    .filter(|&&(k, _)| naming(k.checked_sub(1).map(|p| &sent[p])) || matches!(k.checked_sub(1).map(|p| &sent[p]), Some(Tok::Pron)) && naming(k.checked_sub(2).map(|p| &sent[p])))
                    .map(|x| x.1)
                    .collect();
                let introduced: Vec<char> = bl.iter().filter(|&&(k, _)| intro(k)).map(|x| x.1).collect();
                let new_block = sent.iter().any(|t| matches!(t, Tok::W(w) if matches!(w.as_str(), "there" | "another" | "new" | "second" | "third" | "last")));
                // «another block C», «a block B»
                let fresh: Vec<char> = bl
                    .iter()
                    .filter(|&&(k, _)| k >= 2 && matches!(&sent[k - 1], Tok::W(w) if w == "block") && matches!(&sent[k - 2], Tok::W(w) if matches!(w.as_str(), "another" | "a" | "new" | "one" | "is" | "are")))
                    .map(|x| x.1)
                    .collect();
                if bl.len() <= 2 && named.len() == 1 {
                    cur = Some(named[0]);
                } else if fresh.len() == 1 {
                    cur = Some(fresh[0]);
                } else if bl.len() == 1 && introduced.len() == 1 && new_block {
                    cur = Some(introduced[0]);
                }
                // «Below block A there is block C», «another block below the block A, we call it B» — the relation before
                // the first block holds for the second one
                if let [(k0, x), (k1, y)] = bl[..] {
                    let (front, _) = rels_in(&sent[..k0]);
                    if x != y && (named.contains(&y) || fresh.contains(&y)) && !front.is_empty() && rels_in(&sent[k0 + 1..k1]).0.is_empty() {
                        for r in front {
                            sc.bfacts.insert((y, r, x));
                        }
                    }
                }
                if !has_m {
                    continue;
                }
            }
            // participants in text order
            let mut ents: Vec<(usize, Ent)> = Vec::new();
            for (k, t) in sent.iter().enumerate() {
                match t {
                    Tok::M(d) => {
                        let mut d = d.clone();
                        if d.shape.is_none() && d.color.is_some() && last_shape.is_some() && !d.plural {
                            d.shape = last_shape.clone();
                        }
                        if d.of_group {
                            // «one of them», «the other (one)», «another» — a member of the last group not singled out yet
                            let taken: Vec<usize> = ents.iter().filter_map(|e| if let Ent::O(o) = e.1 { Some(o) } else { None }).chain(singled.iter().copied()).collect();
                            if let Some(&o) = last_group.iter().find(|o| !taken.contains(o)) {
                                singled.push(o);
                                ents.push((k, Ent::O(o)));
                                last = Some(o);
                            }
                            continue;
                        }
                        if d.partitive {
                            // «two of the circles» — the first members not singled out yet
                            let mut pool: Vec<usize> = pending.iter().copied().filter(|&o| matches(&sc.objs[o], &d)).collect();
                            let more: Vec<usize> = (0..sc.objs.len()).filter(|&o| sc.objs[o].block == cur && matches(&sc.objs[o], &d) && !singled.contains(&o) && !pool.contains(&o)).collect();
                            pool.extend(more);
                            let g: Vec<usize> = pool.into_iter().take(d.count).collect();
                            pending.retain(|o| !g.contains(o));
                            last_group = g.clone();
                            ents.push((k, Ent::G(g)));
                            continue;
                        }
                        if d.plural && d.indef {
                            // «3 squares» — a group of new things; the shape for the following «a brown one»
                            last_shape = d.shape.clone();
                            if d.count >= 2 {
                                let mut g = Vec::new();
                                let mut fresh = Vec::new();
                                while g.len() < d.count {
                                    if let Some(p) = pending.iter().position(|&o| sc.objs[o].block == d.block.or(cur) && matches(&sc.objs[o], &d)) {
                                        let o = pending.remove(p);
                                        let ob = &mut sc.objs[o];
                                        ob.size = ob.size.take().or(d.size.clone());
                                        ob.color = ob.color.take().or(d.color.clone());
                                        ob.shape = ob.shape.take().or(d.shape.clone());
                                        g.push(o);
                                        continue;
                                    }
                                    let mut ob = Obj { size: d.size.clone(), color: d.color.clone(), shape: d.shape.clone(), block: d.block.or(cur) };
                                    if let Some((is_size, v)) = g.len().checked_sub(1).and_then(|m| d.alt.get(m)) {
                                        *(if *is_size { &mut ob.size } else { &mut ob.color }) = Some(v.clone());
                                    }
                                    sc.objs.push(ob);
                                    g.push(sc.objs.len() - 1);
                                    fresh.push(sc.objs.len() - 1);
                                }
                                pending.extend(fresh);
                                last_group = g.clone();
                                ents.push((k, Ent::G(g)));
                            }
                            continue;
                        }
                        if d.plural || d.all {
                            // a definite plural — the group of compatible things (in the current block first);
                            // «the yellow and black objects» — of either description
                            let vs = variants(&d);
                            let m = |o: usize| vs.iter().any(|v| matches(&sc.objs[o], v));
                            let mut g: Vec<usize> = (0..sc.objs.len()).filter(|&o| sc.objs[o].block == cur && m(o)).collect();
                            if g.is_empty() {
                                g = (0..sc.objs.len()).filter(|&o| m(o)).collect();
                            }
                            if !g.is_empty() {
                                last_group = g.clone();
                                ents.push((k, Ent::G(g)));
                            }
                            continue;
                        }
                        let found = if d.one_of {
                            let p = pending.iter().position(|&o| matches(&sc.objs[o], &d));
                            let f = p.map(|p| pending.remove(p)).or_else(|| (0..sc.objs.len()).find(|&o| sc.objs[o].block == cur && matches(&sc.objs[o], &d)));
                            singled.extend(f);
                            f
                        } else if d.indef {
                            // a counted group member that was not singled out yet
                            let p = pending.iter().position(|&o| sc.objs[o].block == d.block.or(cur) && matches(&sc.objs[o], &d) && d.shape.is_some());
                            p.map(|p| pending.remove(p))
                        } else if let Some(n) = d.ordinal {
                            let mut g: Vec<usize> = (0..sc.objs.len()).filter(|&o| sc.objs[o].block == cur && matches(&sc.objs[o], &d)).collect();
                            if g.is_empty() {
                                g = (0..sc.objs.len()).filter(|&o| matches(&sc.objs[o], &d)).collect();
                            }
                            singled.extend(g.get(n - 1));
                            g.get(n - 1).copied()
                        } else {
                            // «the other one» — not one already named in this sentence, nor one singled out before
                            let taken: Vec<usize> = if d.other { ents.iter().flat_map(|e| e.1.objs()).chain(singled.iter().copied()).collect() } else { vec![] };
                            let ok = |o: usize| matches(&sc.objs[o], &d) && !taken.contains(&o);
                            let mut f = (0..sc.objs.len()).rev().find(|&o| sc.objs[o].block == cur && ok(o));
                            if f.is_none() {
                                f = (0..sc.objs.len()).rev().find(|&o| ok(o));
                            }
                            f
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
                        // a sentence-initial «It is …» — the subject of the previous sentence; otherwise the last thing
                        let o = if ents.is_empty() && k <= 1 { prev_subj.or(last) } else { last };
                        if let Some(o) = o {
                            ents.push((k, Ent::O(o)));
                        }
                    }
                    Tok::Block(b) => ents.push((k, Ent::B(*b))),
                    Tok::CurBlock => {
                        if let Some(b) = cur {
                            ents.push((k, Ent::B(b)));
                        }
                    }
                    Tok::W(_) => {}
                }
            }
            if let Some(Ent::O(o)) = ents.first().map(|e| &e.1) {
                prev_subj = Some(*o);
            }
            // an edge without a block letter: «a huge red rectangle touching bottom and right edge»
            for j in 0..ents.len() {
                let end = ents.get(j + 1).map(|e| e.0).unwrap_or(sent.len());
                if matches!(ents.get(j + 1).map(|e| &e.1), Some(Ent::B(_))) || matches!(ents[j].1, Ent::B(_)) {
                    continue;
                }
                let seg = &sent[ents[j].0 + 1..end];
                if let Some(e) = seg.iter().position(is_edge_word) {
                    if seg[..e].iter().any(|t| matches!(t, Tok::W(w) if R::of(w) == Some(R::Touch))) {
                        let sides: Vec<R> = seg[..e].iter().filter_map(|t| if let Tok::W(w) = t { R::of(w).filter(|r| r.directional()) } else { None }).collect();
                        for x in ents[j].1.objs() {
                            if sides.is_empty() {
                                sc.edge.insert((x, None));
                            }
                            for &s in &sides {
                                sc.edge.insert((x, Some(s)));
                            }
                        }
                    }
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
                            sc.add(&next.1, *r, &ents[j].1, seg);
                        }
                        used_front[j] = true;
                    }
                }
            }
            // the clause subject — for coordinated predicates «X is touching A and is near Y»
            let subj = if used_front.first() == Some(&true) { ents.get(1) } else { ents.first() }.map(|e| e.1.clone());
            // the last applied predicate — for lists «X is above A, B and C»
            let mut prev: Option<(Ent, Vec<R>, Vec<Tok>)> = None;
            for j in 1..ents.len() {
                if used_front[j] {
                    prev = None;
                    continue;
                }
                let seg = &sent[ents[j - 1].0 + 1..ents[j].0];
                let (rels, _) = rels_in(seg);
                let is_obj = |e: &Ent| !matches!(e, Ent::B(_));
                // a list item: nothing but «and» between two things — the same predicate as the previous item
                if rels.is_empty() && seg.iter().all(|t| matches!(t, Tok::W(w) if w == "and")) && is_obj(&ents[j].1) && is_obj(&ents[j - 1].1) {
                    if let Some((l, rs, pseg)) = &prev {
                        for &r in rs {
                            sc.add(l, r, &ents[j].1, pseg);
                        }
                    }
                    continue;
                }
                let coord = j >= 2 && matches!(seg.first(), Some(Tok::W(w)) if w == "and") && !rels.is_empty();
                let mut left = if coord { subj.clone().unwrap_or_else(|| ents[j - 1].1.clone()) } else { ents[j - 1].1.clone() };
                // «X is touching the top edge of this block which is above Y» — «which» skips the block back to X
                let rel_clause = matches!(seg.first(), Some(Tok::W(w)) if w == "which" || w == "that");
                if matches!(left, Ent::B(_)) && is_obj(&ents[j].1) && rel_clause {
                    if let Some(e) = ents[..j - 1].iter().rev().find(|e| is_obj(&e.1)) {
                        left = e.1.clone();
                    }
                }
                // «a rectangle touching bottom and right edge, a circle …» — the edge is the block's, not the next thing's
                let block_edge = seg.iter().position(is_edge_word).is_some_and(|e| !matches!(seg.get(e + 1), Some(Tok::W(w)) if w == "of"));
                if is_obj(&ents[j].1) && block_edge {
                    prev = None;
                    continue;
                }
                for &r in &rels {
                    sc.add(&left, r, &ents[j].1, seg);
                }
                prev = (!rels.is_empty()).then(|| (left, rels, seg.to_vec()));
            }
            // predicate attributes: «The first one is yellow», «Another circle is big and black»
            for (k, e) in &ents {
                if !matches!(e, Ent::B(_)) {
                    if matches!(sent.get(k + 1), Some(Tok::W(w)) if matches!(w.as_str(), "is" | "was" | "are" | "were")) {
                        let mut t = k + 2;
                        while let Some(Tok::W(w)) = sent.get(t) {
                            if let Some((is_size, v)) = attr_of(w) {
                                for o in e.objs() {
                                    let ob = &mut sc.objs[o];
                                    let slot = if is_size { &mut ob.size } else { &mut ob.color };
                                    if slot.is_none() {
                                        *slot = Some(v.clone());
                                    }
                                }
                            } else if !matches!(w.as_str(), "and" | "also" | "both" | "all") {
                                break;
                            }
                            t += 1;
                        }
                    }
                }
            }
        }
        sc.edge_order();
        sc
    }

    /// In one block a thing touching the top edge is above every thing that does not touch it, and a thing touching
    /// the bottom edge is below every thing that does not; left / right likewise — nothing can be beyond a border that
    /// another thing touches. Not added against what the text already says.
    fn edge_order(&mut self) {
        let side = |sc: &Scene, x: usize, s: R| sc.edge.contains(&(x, Some(s)));
        let mut add = Vec::new();
        for x in 0..self.objs.len() {
            for y in 0..self.objs.len() {
                if x == y || self.objs[x].block.is_none() || self.objs[x].block != self.objs[y].block {
                    continue;
                }
                for (hi, lo) in [(R::Above, R::Below), (R::Left, R::Right)] {
                    let higher = side(self, x, hi) && !side(self, y, hi) || side(self, y, lo) && !side(self, x, lo);
                    if higher && !self.holds(y, hi, x) {
                        add.push((x, hi, y));
                    }
                }
            }
        }
        self.facts.extend(add);
    }

    fn add(&mut self, a: &Ent, r: R, b: &Ent, seg: &[Tok]) {
        let edge = seg.iter().any(|t| matches!(t, Tok::W(w) if matches!(w.as_str(), "edge" | "edges" | "side" | "sides" | "wall" | "border")));
        match (a, b) {
            (Ent::B(x), Ent::B(y)) if x != y => {
                self.bfacts.insert((*x, r, *y));
            }
            (Ent::O(_) | Ent::G(_), Ent::B(bl)) => {
                if r == R::Touch || edge {
                    let sides: Vec<R> = seg.iter().filter_map(|t| if let Tok::W(w) = t { R::of(w).filter(|r| r.directional()) } else { None }).collect();
                    for x in a.objs() {
                        if sides.is_empty() {
                            self.edge.insert((x, None));
                        }
                        for &s in &sides {
                            self.edge.insert((x, Some(s)));
                        }
                        if self.objs[x].block.is_none() {
                            self.objs[x].block = Some(*bl);
                        }
                    }
                }
            }
            _ => {
                for x in a.objs() {
                    for y in b.objs() {
                        if x != y {
                            self.facts.insert((x, r, y));
                        }
                    }
                }
            }
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

    /// Path of direction r from x to y; Some(whether the path implies «far»), None — no path. Far: a far step, or two
    /// steps or more whose distances were all qualified (near / far) — a thing in between at the narrator's scale.
    fn dir_path(&self, x: usize, r: R, y: usize) -> Option<bool> {
        let dist = |u: usize, v: usize| [R::Near, R::Far, R::Touch].iter().any(|&d| self.facts.contains(&(u, d, v)) || self.facts.contains(&(v, d, u)));
        // state: (thing, far so far, steps (capped at 2), all steps qualified)
        let mut seen: BTreeSet<(usize, bool, u8, bool)> = BTreeSet::new();
        let mut q = VecDeque::from([(x, false, 0u8, true)]);
        let mut best: Option<bool> = None;
        while let Some((u, far, steps, qual)) = q.pop_front() {
            for &(a, rr, b) in &self.facts {
                let v = if a == u && rr == r { b } else if b == u && rr.directional() && rr.inv() == r { a } else { continue };
                let (steps, qual) = ((steps + 1).min(2), qual && dist(u, v));
                let f = far || self.facts.contains(&(u, R::Far, v)) || self.facts.contains(&(v, R::Far, u));
                if v == y {
                    best = Some(best.unwrap_or(false) || f || steps >= 2 && qual);
                }
                if seen.insert((v, f, steps, qual)) {
                    q.push_back((v, f, steps, qual));
                }
            }
        }
        best
    }

    fn far(&self, x: usize, y: usize) -> bool {
        let (bx, by) = (self.objs[x].block, self.objs[y].block);
        self.facts.contains(&(x, R::Far, y)) || self.facts.contains(&(y, R::Far, x)) || bx.is_some() && by.is_some() && bx != by
            || !self.near(x, y) && [R::Left, R::Right, R::Above, R::Below].iter().any(|&r| self.dir_path(x, r, y) == Some(true))
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

    /// Things under a question description: strict match; if none — things whose unknown attributes leave it possible
    /// («the small blue thing in C» when the story said only «a blue triangle» there).
    fn find(&self, d: &Desc) -> Vec<usize> {
        let strict: Vec<usize> = (0..self.objs.len()).filter(|&o| matches_q(&self.objs[o], d)).collect();
        if !strict.is_empty() {
            return strict;
        }
        (0..self.objs.len()).filter(|&o| matches(&self.objs[o], d)).collect()
    }

    /// Whether x touches an edge: sides — all of them («the top and right edge»; empty — any edge), block — of that block.
    fn touches_edge(&self, x: usize, sides: &[R], block: Option<char>) -> bool {
        block.is_none_or(|b| self.objs[x].block == Some(b))
            && if sides.is_empty() { self.edge.iter().any(|&(o, _)| o == x) } else { sides.iter().all(|&s| self.edge.contains(&(x, Some(s)))) }
    }

    /// x between two different things of ys: on a line (left of one and right of the other, or above and below).
    fn between(&self, x: usize, ys: &[usize]) -> bool {
        ys.iter().any(|&a| ys.iter().any(|&b| a != b && a != x && b != x && (self.tv(x, R::Left, a) == Some(true) && self.tv(x, R::Right, b) == Some(true) || self.tv(x, R::Above, a) == Some(true) && self.tv(x, R::Below, b) == Some(true))))
    }

    /// Things a phrase denotes; block — restriction of the head (FB: «which block has a square above the big triangle» —
    /// the triangle may be in another block).
    fn resolve(&self, p: &Phrase, block: Option<char>) -> Vec<usize> {
        let mut d = p.d.clone();
        if d.block.is_none() {
            d.block = block;
        }
        let mut xs: Vec<usize> = Vec::new();
        for v in variants(&d) {
            xs.extend(self.find(&v));
        }
        xs.sort();
        xs.dedup();
        if let Some(n) = d.ordinal {
            xs = xs.get(n - 1).copied().into_iter().collect();
        }
        for m in &p.mods {
            xs.retain(|&x| match m {
                Mod::Rel(rels, t) => {
                    let ys = self.resolve(t, None);
                    ys.iter().any(|&y| y != x && self.tv_all(x, rels, y) == Some(true))
                }
                Mod::Edge(sides, b) => self.touches_edge(x, sides, b.or(block)),
                Mod::Between(t, t2) => {
                    let mut ys = self.resolve(t, None);
                    if let Some(t2) = t2 {
                        ys.extend(self.resolve(t2, None));
                    }
                    self.between(x, &ys)
                }
            });
        }
        xs
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

/// A question phrase: a mention with modifiers.
#[derive(Clone, Debug, Default)]
struct Phrase {
    d: Desc,
    mods: Vec<Mod>,
}

#[derive(Clone, Debug)]
enum Mod {
    /// «which is above the circle»
    Rel(Vec<R>, Box<Phrase>),
    /// «touching the bottom edge of block B»: sides (empty — any), block
    Edge(Vec<R>, Option<char>),
    /// «between two circles», «between a red object and a white triangle»
    Between(Box<Phrase>, Option<Box<Phrase>>),
}

fn is_edge_word(t: &Tok) -> bool {
    matches!(t, Tok::W(w) if matches!(w.as_str(), "edge" | "edges" | "side" | "sides" | "border" | "wall"))
}

/// An edge phrase from position k: «[touching] the bottom [and right] edge of [the] block B» → (sides, block, end).
fn edge_phrase(toks: &[Tok], k: usize) -> Option<(Vec<R>, Option<char>, usize)> {
    let mut i = k;
    let mut sides = Vec::new();
    while i < toks.len() && !is_edge_word(&toks[i]) {
        match &toks[i] {
            Tok::W(w) => {
                if let Some(r) = R::of(w).filter(|r| r.directional()) {
                    sides.push(r);
                }
            }
            _ => return None,
        }
        i += 1;
    }
    if i >= toks.len() {
        return None;
    }
    i += 1;
    let mut block = None;
    while i < toks.len() {
        match &toks[i] {
            Tok::W(w) if matches!(w.as_str(), "of" | "the" | "block" | "its" | "a" | "in") => i += 1,
            Tok::Block(b) => {
                block = Some(*b);
                i += 1;
                break;
            }
            Tok::CurBlock | Tok::Pron => {
                i += 1;
                break;
            }
            _ => break,
        }
    }
    Some((sides, block, i))
}

/// A phrase from the mention at position k up to `hi`: «which/that is R …» always; a bare «R …» if `bare`.
fn parse_np(toks: &[Tok], k: usize, hi: usize, bare: bool) -> (Phrase, usize) {
    let Tok::M(d) = &toks[k] else { return (Phrase::default(), k + 1) };
    let mut p = Phrase { d: d.clone(), mods: Vec::new() };
    let mut i = k + 1;
    loop {
        let mut j = i;
        let mut marked = false;
        while j < hi && matches!(&toks[j], Tok::W(w) if matches!(w.as_str(), "which" | "that" | "who" | "is" | "are" | "and")) {
            marked |= matches!(&toks[j], Tok::W(w) if matches!(w.as_str(), "which" | "that" | "who"));
            j += 1;
        }
        if !(marked || bare) || j >= hi {
            break;
        }
        // «between X and Y», «between two Xs»
        if matches!(&toks[j], Tok::W(w) if w == "between") {
            if let Some(m1) = (j + 1..hi).find(|&t| matches!(toks[t], Tok::M(_))) {
                let (t1, e1) = parse_np(toks, m1, hi, false);
                let and2 = (e1 < hi && matches!(&toks[e1], Tok::W(w) if w == "and")).then(|| (e1 + 1..hi).find(|&t| matches!(toks[t], Tok::M(_)))).flatten();
                if let Some(m2) = and2 {
                    let (t2, e2) = parse_np(toks, m2, hi, false);
                    p.mods.push(Mod::Between(Box::new(t1), Some(Box::new(t2))));
                    i = e2;
                } else {
                    p.mods.push(Mod::Between(Box::new(t1), None));
                    i = e1;
                }
                continue;
            }
            break;
        }
        // relation words up to the next mention or edge
        let mut e = j;
        while e < hi && matches!(&toks[e], Tok::W(w) if !matches!(w.as_str(), "which" | "that" | "or")) && !is_edge_word(&toks[e]) {
            e += 1;
        }
        let (rels, _) = rels_in(&toks[j..e.min(hi)]);
        if rels.is_empty() {
            break;
        }
        if e < hi && is_edge_word(&toks[e]) {
            if let Some((sides, b, end)) = edge_phrase(toks, j) {
                p.mods.push(Mod::Edge(sides, b));
                i = end;
                continue;
            }
            break;
        }
        if e < hi && matches!(toks[e], Tok::M(_)) {
            let (t, end) = parse_np(toks, e, hi, bare);
            p.mods.push(Mod::Rel(rels, Box::new(t)));
            i = end;
            continue;
        }
        break;
    }
    (p, i)
}

fn mention_positions(toks: &[Tok]) -> Vec<usize> {
    (0..toks.len()).filter(|&k| matches!(toks[k], Tok::M(_))).collect()
}

fn qtoks(q: &str) -> Vec<Tok> {
    scan(q).into_iter().flatten().collect()
}

/// A description matching several things — the ones in the block of the other participant, if any
/// («the small yellow square» next to «the large yellow thing in B» — the square in B).
fn prefer_block(sc: &Scene, xs: &mut Vec<usize>, ys: &[usize]) {
    let blocks: BTreeSet<Option<char>> = ys.iter().map(|&y| sc.objs[y].block).collect();
    if xs.len() > 1 && xs.iter().any(|&x| blocks.contains(&sc.objs[x].block)) {
        xs.retain(|&x| blocks.contains(&sc.objs[x].block));
    }
}

/// Quantified truth over a set: all — every one true / some false; some — some true / every one false.
fn quant(v: &[Option<bool>], all: bool) -> Option<bool> {
    if all {
        if v.iter().all(|t| *t == Some(true)) {
            Some(true)
        } else if v.contains(&Some(false)) {
            Some(false)
        } else {
            None
        }
    } else if v.contains(&Some(true)) {
        Some(true)
    } else if v.iter().all(|t| *t == Some(false)) {
        Some(false)
    } else {
        None
    }
}

fn yn(sc: &Scene, q: &str) -> String {
    let toks = qtoks(q);
    let ms = mention_positions(&toks);
    let words: Vec<&str> = toks.iter().filter_map(|t| if let Tok::W(w) = t { Some(w.as_str()) } else { None }).collect();
    let Some(&k0) = ms.first() else { return "DK".into() };
    // «Does block B have all of the circles inside it?» — containment
    let blk = toks[..k0].iter().find_map(|t| if let Tok::Block(b) = t { Some(*b) } else { None });
    if let Some(b) = blk.filter(|_| words.iter().any(|w| matches!(*w, "have" | "has" | "contain" | "contains"))) {
        let p = parse_np(&toks, k0, toks.len(), true).0;
        let (all, inb) = (sc.resolve(&p, None), sc.resolve(&p, Some(b)));
        let has = if p.d.all || words.contains(&"all") { !inb.is_empty() && inb.len() == all.len() } else { !inb.is_empty() };
        return if has { "Yes" } else { "No" }.into();
    }
    // subject with its marked modifiers («the circle which is left of a square»)
    let (subj, i) = parse_np(&toks, k0, toks.len(), false);
    let xs = sc.resolve(&subj, None);
    let a = subj.d.clone();
    let sub_all = a.all || a.plural && !a.indef;
    // main relation words up to the object
    let next_m = (i..toks.len()).find(|&t| matches!(toks[t], Tok::M(_)));
    let seg_end = next_m.unwrap_or(toks.len());
    let (rels, _) = rels_in(&toks[i..seg_end]);
    // «… touching the bottom edge of a block?» — the object is an edge
    if let Some(e) = (i..seg_end).find(|&t| is_edge_word(&toks[t])) {
        if let Some((sides, b, _)) = edge_phrase(&toks, (i..e).find(|&t| matches!(&toks[t], Tok::W(w) if R::of(w).is_some())).unwrap_or(e)) {
            if xs.is_empty() {
                return "No".into();
            }
            let v: Vec<Option<bool>> = xs.iter().map(|&x| Some(sc.touches_edge(x, &sides, b))).collect();
            return if quant(&v, sub_all) == Some(true) { "Yes" } else { "No" }.into();
        }
    }
    // «Are the large yellow things in A near each other?» — pairs within one set
    let each_other = words.windows(2).any(|w| w == ["each", "other"]) || words.contains(&"together");
    let (b, ys) = match next_m {
        Some(k1) => {
            let (obj, _) = parse_np(&toks, k1, toks.len(), true);
            let mut ys = sc.resolve(&obj, None);
            // «the other small yellow thing», «another circle» — excluding the subject itself
            if (obj.d.other || words.contains(&"other")) && xs.len() == 1 {
                ys.retain(|y| *y != xs[0]);
            }
            (obj.d, ys)
        }
        None if each_other => (a.clone(), xs.clone()),
        None => return "DK".into(),
    };
    if xs.is_empty() || ys.is_empty() || rels.is_empty() {
        return "DK".into();
    }
    if each_other && next_m.is_none() {
        let mut v = Vec::new();
        for &x in &xs {
            for &y in &ys {
                if x < y {
                    v.push(sc.tv_all(x, &rels, y));
                }
            }
        }
        return if !v.is_empty() && v.iter().all(|t| *t == Some(true)) { "Yes" } else { "No" }.into();
    }
    let obj_all = b.all || b.plural && !b.indef;
    // over the object: all / at least one; then over the subject the same way
    let v: Vec<Option<bool>> = xs.iter().map(|&x| quant(&ys.iter().filter(|&&y| y != x).map(|&y| sc.tv_all(x, &rels, y)).collect::<Vec<_>>(), obj_all)).collect();
    // measured on the training split: the gold is almost closed-world (DK — 16 of 161, mostly when the thing is absent),
    // so «not inferred» → No; «Is a X not R a Y?» — the negation of the positive question
    let neg = toks[i..seg_end].iter().any(|t| matches!(t, Tok::W(w) if w == "not"));
    if (quant(&v, sub_all) == Some(true)) != neg { "Yes" } else { "No" }.into()
}

fn fb(sc: &Scene, q: &str) -> Vec<String> {
    let toks = qtoks(q);
    let words: Vec<String> = toks.iter().filter_map(|t| if let Tok::W(w) = t { Some(w.clone()) } else { None }).collect();
    let ms = mention_positions(&toks);
    // «What object has all of the triangles» — the asked word is the block, the mention after it is not the thing
    let asked = |k: usize| k <= 1 && matches!(&toks[k], Tok::M(d) if d.color.is_none() && d.size.is_none() && d.shape.is_none() && !d.plural);
    let Some(&k0) = ms.iter().find(|&&k| !asked(k)) else { return vec![] };
    let (p, _) = parse_np(&toks, k0, toks.len(), true);
    let neg = p.d.neg || words.iter().any(|w| matches!(w.as_str(), "not" | "doesn't" | "doesnt" | "deosn't" | "don't" | "none" | "without"));
    let all_q = p.d.all || words.iter().any(|w| w == "all");
    // «… touching the edge of it» right after the mention
    let mut p = p;
    if !p.mods.iter().any(|m| matches!(m, Mod::Edge(..))) {
        if let Some(e) = (k0..toks.len()).find(|&t| is_edge_word(&toks[t])) {
            if let Some(s) = (k0..e).find(|&t| matches!(&toks[t], Tok::W(w) if R::of(w).is_some())) {
                if let Some((sides, b, _)) = edge_phrase(&toks, s) {
                    p.mods.push(Mod::Edge(sides, b));
                }
            }
        }
    }
    let mut out = Vec::new();
    let total = sc.resolve(&p, None);
    for &b in &sc.blocks {
        let inb = sc.resolve(&p, Some(b)).len();
        let has = if all_q { inb > 0 && inb == total.len() } else { inb > 0 };
        if has != neg {
            out.push(b.to_string());
        }
    }
    out
}

/// The question part before the candidates («…, the A or the B?»).
fn question_head(q: &str) -> &str {
    let end = q.find([',', '?']).unwrap_or(q.len());
    &q[..end]
}

fn co(sc: &Scene, q: &str, cands: &[String]) -> usize {
    // «What object is to the left of the X, the A or the B?» / «What is above the X? the A or the B»
    let toks = qtoks(question_head(q));
    let ms = mention_positions(&toks);
    // «what object» — a mention right after the question word is the asked thing, not the reference
    let asked = ms.first().filter(|&&k| k <= 1 && matches!(&toks[k], Tok::M(d) if d.color.is_none() && d.size.is_none())).copied();
    let start = asked.map(|k| k + 1).unwrap_or(0);
    let Some(&kr) = ms.iter().find(|&&k| k >= start) else { return 3 };
    let seg = &toks[start..kr];
    let (rels, _) = rels_in(seg);
    let neg = seg.iter().any(|t| matches!(t, Tok::W(w) if w == "not"));
    let between = seg.iter().any(|t| matches!(t, Tok::W(w) if w == "between"));
    if rels.is_empty() && !between {
        return 3;
    }
    let (refp, _) = parse_np(&toks, kr, toks.len(), true);
    let ys = sc.resolve(&refp, None);
    let mut ref2 = Vec::new();
    if between {
        // «between a small red object and a white triangle»
        if let Some(&k2) = ms.iter().find(|&&k| k > kr) {
            ref2 = sc.resolve(&parse_np(&toks, k2, toks.len(), true).0, None);
        }
    }
    let test = |c: &str| -> bool {
        let ct = qtoks(c);
        let Some(&kc) = mention_positions(&ct).first() else { return false };
        let cp = parse_np(&ct, kc, ct.len(), true).0;
        let mut xs = sc.resolve(&cp, None);
        if !cp.d.indef {
            prefer_block(sc, &mut xs, &ys);
        }
        if xs.is_empty() || ys.is_empty() {
            return false;
        }
        if between {
            let mut all = ys.clone();
            all.extend(&ref2);
            return xs.iter().any(|&x| sc.between(x, &all));
        }
        if neg {
            // «not R»: closed world, as the gold — no R inferred to any of the reference things
            xs.iter().any(|&x| ys.iter().all(|&yy| sc.tv_all(x, &rels, yy) != Some(true)))
        } else {
            xs.iter().any(|&x| ys.iter().any(|&yy| yy != x && sc.tv_all(x, &rels, yy) == Some(true)))
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

/// FR question → its two phrases: «the relation between P1 and P2», «where is P1 regarding (to) P2».
fn fr_parts(toks: &[Tok]) -> Option<(Phrase, Phrase)> {
    let ms = mention_positions(toks);
    let &k0 = ms.first()?;
    // «the relation between the black and blue objects» — one plural with two attributes
    if let (1, Tok::M(d)) = (ms.len(), &toks[k0]) {
        let vs = variants(d);
        if vs.len() == 2 {
            let (mut a, mut b) = (vs[0].clone(), vs[1].clone());
            a.plural = false;
            b.plural = false;
            return Some((Phrase { d: a, mods: vec![] }, Phrase { d: b, mods: vec![] }));
        }
    }
    // the split: «and» / «regarding» / «relative» / «compared» followed (soon) by a mention
    let split = (k0 + 1..toks.len()).find(|&t| {
        matches!(&toks[t], Tok::W(w) if matches!(w.as_str(), "and" | "regarding" | "relative" | "compared" | "with" | "from"))
            && (t + 1..toks.len().min(t + 3)).any(|u| matches!(toks[u], Tok::M(_)))
    })?;
    let &k1 = ms.iter().find(|&&k| k > split)?;
    let (p1, _) = parse_np(toks, k0, split, true);
    let (p2, _) = parse_np(toks, k1, toks.len(), true);
    Some((p1, p2))
}

fn fr(sc: &Scene, q: &str, cands: &[String]) -> Vec<usize> {
    let toks = qtoks(q);
    let dk = cands.iter().position(|c| c.trim() == "DK");
    let Some((p1, p2)) = fr_parts(&toks) else { return dk.into_iter().collect() };
    let (mut xs, mut ys) = (sc.resolve(&p1, None), sc.resolve(&p2, None));
    let single = |p: &Phrase| !p.d.plural && !p.d.all && !p.d.indef;
    if single(&p1) {
        prefer_block(sc, &mut xs, &ys);
    }
    if single(&p2) {
        prefer_block(sc, &mut ys, &xs);
    }
    if p2.d.other && xs.len() == 1 {
        ys.retain(|y| *y != xs[0]);
    }
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
    // touching is the stronger answer: «near» is not listed beside it
    let idx = |r: R| cand_r.iter().find(|c| c.1 == r).map(|c| c.0);
    if let (Some(t), Some(n)) = (idx(R::Touch), idx(R::Near)) {
        if out.contains(&t) {
            out.retain(|&i| i != n);
        }
    }
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
    // SPARTQA_DUMP=<file>: per-question correctness («story\tq_id\ttype\tok») for paired comparisons
    let mut dump = String::new();
    for (si, st) in v["data"].as_array().cloned().unwrap_or_default().into_iter().enumerate() {
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
                    let ok = gold.as_array().and_then(|a| a.first()).and_then(|x| x.as_str()).map(str::trim) == Some(p.as_str());
                    (p, ok)
                }
                "FB" => {
                    let mut p = fb(&sc, qs);
                    p.sort();
                    // gold strings carry stray spaces (["A", " B"]) — annotation formatting, trimmed
                    let mut g: Vec<String> = gold.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.trim().to_string())).collect()).unwrap_or_default();
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
            dump.push_str(&format!("{si}\t{}\t{t}\t{}\n", q["q_id"], u8::from(ok)));
            let e = res.entry(t.clone()).or_default();
            e.1 += 1;
            if ok {
                e.0 += 1;
            } else if shown < show {
                shown += 1;
                println!("  {t} {qs} → {pred} (expected {gold})");
                if std::env::var("SPARTQA_TOKS").is_ok() {
                    println!("    {:?}", qtoks(qs));
                }
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
    if let Ok(f) = std::env::var("SPARTQA_DUMP") {
        std::fs::write(&f, &dump)?;
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

/// Contrast renaming (a permutation, so nothing collides): colours, shapes and block letters.
const CONTRAST: &[(&str, &str)] = &[
    ("blue", "purple"),
    ("purple", "blue"),
    ("yellow", "orange"),
    ("orange", "yellow"),
    ("black", "pink"),
    ("pink", "black"),
    ("red", "brown"),
    ("brown", "red"),
    ("green", "gray"),
    ("gray", "green"),
    ("grey", "green"),
    ("square", "pentagon"),
    ("pentagon", "square"),
    ("circle", "hexagon"),
    ("hexagon", "circle"),
    ("triangle", "diamond"),
    ("diamond", "triangle"),
    ("rectangle", "star"),
    ("star", "rectangle"),
];
const CONTRAST_BLOCKS: &[(char, char)] = &[('A', 'D'), ('D', 'A'), ('B', 'E'), ('E', 'B'), ('C', 'F'), ('F', 'C')];

/// Rename one word, keeping plural and capitalisation; typos of shapes (one edit) are renamed too.
fn contrast_word(w: &str) -> Option<String> {
    let low = w.to_lowercase();
    let (stem, suf) = if let Some(s) = low.strip_suffix("es").filter(|s| s.ends_with('x')) {
        (s.to_string(), "es")
    } else if let Some(s) = low.strip_suffix('s').filter(|s| s.len() > 2 && !s.ends_with('s')) {
        (s.to_string(), "s")
    } else {
        (low.clone(), "")
    };
    // a glued article «ablue» → «apurple»
    if let Some(rest) = low.strip_prefix('a').filter(|r| r.len() >= 3 && cat("scene_color", r)) {
        return contrast_word(rest).map(|r| format!("{}{}", &w[..1], r.to_lowercase()));
    }
    let rep = CONTRAST.iter().find(|(a, _)| *a == stem).or_else(|| {
        // a misspelt shape or colour («triange», «yelow»): words of 5+ letters, not frequent look-alikes («block»)
        (stem.len() >= 5 && !NOT_TYPOS.contains(&stem.as_str())).then(|| CONTRAST.iter().find(|(a, _)| near_word(&stem, a))).flatten()
    })?;
    let mut out = rep.1.to_string();
    if !suf.is_empty() {
        out.push_str(if out.ends_with('x') { "es" } else { "s" });
    }
    if w.chars().next().is_some_and(|c| c.is_uppercase()) {
        out = out[..1].to_uppercase() + &out[1..];
    }
    Some(out)
}

/// Rename a text: words from CONTRAST, block letters (a standalone capital; a sentence-initial «A» is a block only
/// before a verb or punctuation, otherwise the article), «a/an» re-agreed with the new next word.
pub fn contrast_text(text: &str) -> String {
    // split into alphabetic words and the rest
    let mut parts: Vec<(bool, String)> = Vec::new();
    for c in text.chars() {
        let alpha = c.is_ascii_alphabetic();
        match parts.last_mut() {
            Some((a, s)) if *a == alpha => s.push(c),
            _ => parts.push((alpha, c.to_string())),
        }
    }
    let n = parts.len();
    let mut out: Vec<String> = parts.iter().map(|p| p.1.clone()).collect();
    for i in 0..n {
        if !parts[i].0 {
            continue;
        }
        let w = parts[i].1.as_str();
        if let Some(r) = contrast_word(w) {
            out[i] = r;
            continue;
        }
        let ch = w.chars().next().unwrap();
        // a lower-case block letter: after «block», or after «in» with no word following but «or / and»
        if w.len() == 1 && ch.is_ascii_lowercase() {
            let prev = (0..i).rev().find(|&k| parts[k].0).map(|k| parts[k].1.to_lowercase());
            let next = (i + 1..n).find(|&k| parts[k].0).map(|k| parts[k].1.to_lowercase());
            let block = prev.as_deref() == Some("block") || prev.as_deref() == Some("in") && next.as_deref().is_none_or(|x| x == "or" || x == "and");
            if let Some(&(_, to)) = CONTRAST_BLOCKS.iter().find(|(a, _)| *a == ch.to_ascii_uppercase()).filter(|_| block) {
                out[i] = to.to_ascii_lowercase().to_string();
            }
            continue;
        }
        if w.len() == 1 && ch.is_ascii_uppercase() {
            if let Some(&(_, to)) = CONTRAST_BLOCKS.iter().find(|(a, _)| *a == ch) {
                let initial = (0..i).rev().find(|&k| !parts[k].1.trim().is_empty()).is_none_or(|k| !parts[k].0 && parts[k].1.trim().ends_with(['.', '?', '!']));
                let after_sep = parts.get(i + 1).map(|p| p.1.as_str()).unwrap_or("");
                let next = parts.get(i + 2).map(|p| p.1.as_str()).unwrap_or("");
                let verb_next = matches!(next, "is" | "has" | "contains" | "and" | "s" | "was" | "seems" | "appears");
                let article = ch == 'A' && initial && after_sep.trim().is_empty() && !after_sep.is_empty() && !verb_next;
                if !article {
                    out[i] = to.to_string();
                }
            }
        }
    }
    // a / an agreement after renaming
    for i in 0..n {
        let low = out[i].to_lowercase();
        if parts[i].0 && (low == "a" || low == "an") && parts[i].1.len() <= 2 && out[i].len() <= 2 {
            if parts[i].1.len() == 1 && parts[i].1 != out[i] {
                continue; // a renamed block letter
            }
            if let Some(next) = out.get(i + 2).filter(|_| parts.get(i + 1).is_some_and(|p| p.1 == " ")) {
                let vowel = next.chars().next().is_some_and(|c| "aeiouAEIOU".contains(c));
                let cap = out[i].starts_with(|c: char| c.is_uppercase());
                out[i] = match (vowel, cap) {
                    (true, false) => "an",
                    (true, true) => "An",
                    (false, false) => "a",
                    (false, true) => "A",
                }
                .into();
            }
        }
    }
    out.concat()
}

/// Contrast version of a SpartQA file: stories, questions, candidate answers and block-letter answers renamed.
pub fn contrast(src: &Path, dst: &Path) -> Result<()> {
    let text = std::fs::read_to_string(src).with_context(|| format!("{}", src.display()))?;
    let mut v: serde_json::Value = serde_json::from_str(&text)?;
    fn walk(v: &mut serde_json::Value) {
        match v {
            serde_json::Value::String(s) => *s = contrast_text(s),
            serde_json::Value::Array(a) => a.iter_mut().for_each(walk),
            _ => {}
        }
    }
    for st in v["data"].as_array_mut().into_iter().flatten() {
        walk(&mut st["story"]);
        for q in st["questions"].as_array_mut().into_iter().flatten() {
            walk(&mut q["question"]);
            walk(&mut q["candidate_answers"]);
            walk(&mut q["answer"]);
        }
    }
    if let Some(d) = dst.parent() {
        std::fs::create_dir_all(d)?;
    }
    std::fs::write(dst, serde_json::to_string_pretty(&v)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_renames_consistently() {
        assert_eq!(
            contrast_text("A small blue circle is in A. Block B has a yellow triangle and an oval. A is left of C."),
            "A small purple hexagon is in D. Block E has an orange diamond and an oval. D is left of F."
        );
        assert_eq!(contrast_text("Are all squares in B?"), "Are all pentagons in E?");
        assert_eq!(contrast_text("The black triange"), "The pink diamond");
        assert_eq!(contrast_text(" the yelow thing in c "), " the orange thing in f ");
        assert_eq!(contrast_text("Which block has ablue square in block a?"), "Which block has apurple pentagon in block d?");
    }

    #[test]
    fn typos_and_block_letters() {
        assert!(near_word("sqaure", "square"));
        assert!(!near_word("block", "blocks") || attr_of("block").is_none());
        assert_eq!(attr_of("yelow"), Some((false, "yellow".into())));
        assert_eq!(attr_of("smal"), Some((true, "small".into())));
        assert_eq!(attr_of("block"), None);
        assert_eq!(R::of("toucing"), Some(R::Touch));
        assert_eq!(R::of("yelow"), None);
        let t = qtoks("the small yellow thing in c");
        assert!(matches!(&t[0], Tok::M(d) if d.block == Some('C')));
        let t = qtoks("Is the circle in block B above the square?");
        assert!(matches!(&t[1], Tok::M(d) if d.block == Some('B') && d.shape.as_deref() == Some("circle")));
    }

    fn obj(sc: &Scene, color: &str, shape: &str) -> usize {
        sc.objs.iter().position(|o| o.color.as_deref() == Some(color) && o.shape.as_deref() == Some(shape)).unwrap()
    }

    #[test]
    fn story_context_groups_lists() {
        let sc = Scene::read(
            "There are two blocks A and B. In block A, there is a red square near a blue circle and a green triangle. \
             Block B is to the right of block A. It has two yellow circles. One of the circles is touching the top edge of this block. \
             The other yellow circle is touching the bottom edge of the block and is near a black square.",
        );
        let (sq, ci, tr) = (obj(&sc, "red", "square"), obj(&sc, "blue", "circle"), obj(&sc, "green", "triangle"));
        assert_eq!(sc.objs[sq].block, Some('A'));
        // a list continues the relation: the square is near the triangle too
        assert_eq!(sc.tv(sq, R::Near, ci), Some(true));
        assert_eq!(sc.tv(sq, R::Near, tr), Some(true));
        // «It has» after a sentence about block B; a counted group of two
        let ys: Vec<usize> = (0..sc.objs.len()).filter(|&o| sc.objs[o].color.as_deref() == Some("yellow")).collect();
        assert_eq!(ys.len(), 2);
        assert!(ys.iter().all(|&o| sc.objs[o].block == Some('B')));
        // one touches the top edge, the other the bottom edge: the first is above the second (edge order)
        assert_eq!(sc.tv(ys[0], R::Above, ys[1]), Some(true));
        // coordination: the other circle (not the touching block) is near the black square
        assert_eq!(sc.tv(ys[1], R::Near, obj(&sc, "black", "square")), Some(true));
        // blocks: right of → objects in different blocks are far, and directions carry over
        assert_eq!(sc.tv(ys[0], R::Right, sq), Some(true));
        // a thing touching the top border is above a thing in the block that does not touch it
        assert_eq!(sc.tv(ys[0], R::Above, obj(&sc, "black", "square")), Some(true));
        assert_eq!(sc.tv(sq, R::Above, ci), None);
    }

    #[test]
    fn relative_that_and_asked_word() {
        let t = qtoks("Are all objects that are touching the right edge of a block left of a circle?");
        assert!(matches!(&t[2], Tok::W(w) if w == "that"));
        let sc = Scene::read("There are two blocks A and B. In A, there is a red triangle. In B, there is a blue circle.");
        assert_eq!(fb(&sc, "What object has all of the triangles inside of it?"), vec!["A".to_string()]);
    }

    #[test]
    fn story_fronted_both_and_far_path() {
        let sc = Scene::read(
            "In A, there is a small blue square far above a small blue circle. There is a black triangle far to the right of both blue objects. \
             Near and to the left of the blue circle is a red circle. Near and to the left of the red circle is a green circle.",
        );
        let (bsq, bci, tri) = (obj(&sc, "blue", "square"), obj(&sc, "blue", "circle"), obj(&sc, "black", "triangle"));
        assert_eq!(sc.tv(tri, R::Right, bsq), Some(true));
        assert_eq!(sc.tv(tri, R::Right, bci), Some(true));
        let (rc, gc) = (obj(&sc, "red", "circle"), obj(&sc, "green", "circle"));
        assert_eq!(sc.tv(rc, R::Left, bci), Some(true));
        // two qualified near steps in one direction — far at the narrator's scale
        assert_eq!(sc.tv(gc, R::Far, bci), Some(true));
        assert_eq!(sc.tv(gc, R::Far, rc), Some(false));
    }

    #[test]
    fn questions_phrases() {
        let sc = Scene::read(
            "There are two blocks A and B. Block A is above block B. In A, there is a yellow square touching the left edge of A and a red circle to the right of the yellow square. \
             In B, there is a blue triangle touching the bottom edge of B and a blue circle near and above the blue triangle.",
        );
        assert_eq!(yn(&sc, "Is the square touching the left edge of a block?"), "Yes");
        assert_eq!(yn(&sc, "Is the circle which is to the right of a square above the blue triangle?"), "Yes");
        assert_eq!(yn(&sc, "Is a red circle not above a blue triangle?"), "No");
        assert_eq!(fb(&sc, "Which block has a circle above a triangle?"), vec!["A".to_string(), "B".to_string()]);
        assert_eq!(fb(&sc, "Which block has an object touching the bottom edge of it?"), vec!["B".to_string()]);
        let c = |a: &str, b: &str| vec![a.to_string(), b.to_string()];
        assert_eq!(co(&sc, "What is above the blue triangle? the red circle or the blue circle?", &c("the red circle", "the blue circle")), 2);
        assert_eq!(co(&sc, "What object is to the right of the yellow square, the red circle in a or the blue circle?", &c("the red circle in a", "the blue circle")), 0);
        let cands: Vec<String> = ["left", "right", "above", "below", "near to", "far from", "touching", "DK"].iter().map(|s| s.to_string()).collect();
        let mut r = fr(&sc, "What is the relation between the circle near the blue triangle and the yellow square?", &cands);
        r.sort();
        assert_eq!(r, vec![3, 5]);
    }
}
