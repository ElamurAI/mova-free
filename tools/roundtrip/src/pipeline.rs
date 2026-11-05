//! One volume through every layer and back, in both modes.

use std::collections::BTreeMap;
use std::ops::Range;

use anyhow::Result;
use en::annotate::{Annotator, Word};

use crate::class::Layer;
use crate::diff::{self, Tk};
use crate::frames::{self, F, Realizer, Step};
use crate::tally::{Diverge, Start, Tally};
use crate::tree::{self, Dec, TNode};
use crate::vol::{self, Sent, Volume};
use crate::world;

/// The unit of the discourse: what shares one discourse context, one realizer and one world.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Unit {
    /// the whole volume (a block): one context and one world, never reset
    #[default]
    Block,
    /// every sentence alone: the context, the realizer and the world reset at every sentence (the control); the
    /// frames themselves change (a pronoun the reader cannot resolve stays a pronoun)
    Sentence,
    /// the frames read with the volume's one discourse, the realizer and the world reset at every sentence: the same
    /// frames as `Block`, each sentence's world alone
    World,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Opts {
    pub exact: bool,
    pub normal: bool,
    /// a negative control: the tree decoder drops every word of this class (normal mode)
    pub drop: Option<crate::class::Class>,
    pub unit: Unit,
    /// the gate's negative control: the exact decode loses one residual item of this layer (the first it has)
    pub damage: Option<crate::class::Layer>,
}

fn tks(t: &[TNode], order: &[usize], forms: &[String], space: &[bool]) -> Vec<Tk> {
    order.iter().enumerate().map(|(k, &i)| Tk { form: forms[k].clone(), lemma: t[i - 1].lemma.clone(), tag: t[i - 1].tag, rel: t[i - 1].rel, space: space[k] }).collect()
}

fn orig_tks(w: &[Word], space: &[bool]) -> Vec<Tk> {
    w.iter().enumerate().map(|(k, x)| Tk { form: x.form.clone(), lemma: x.lemma.clone(), tag: x.tag, rel: x.rel, space: space[k] }).collect()
}

/// A tree with the words of one class removed (their dependents go to the removed word's head).
fn drop_class(t: &[TNode], c: crate::class::Class) -> Vec<TNode> {
    let gone: Vec<bool> = t.iter().map(|x| crate::class::of_word(x.tag, x.rel, &x.lemma) == c).collect();
    let mut newi = vec![0usize; t.len() + 1];
    let mut k = 0;
    for i in 0..t.len() {
        if !gone[i] {
            k += 1;
            newi[i + 1] = k;
        }
    }
    let up = |mut h: usize| {
        let mut guard = 0;
        while h > 0 && gone[h - 1] && guard < t.len() {
            h = t[h - 1].head;
            guard += 1;
        }
        if h > 0 && gone[h - 1] { 0 } else { newi[h] }
    };
    t.iter().enumerate().filter(|(i, _)| !gone[*i]).map(|(_, x)| TNode { head: up(x.head), ..x.clone() }).collect()
}

/// Decode a tree to text with the canonical decoder and compare with the original sentence; the decoded text.
fn measure(dec: &Dec, t: &[TNode], orig: &[Tk], orig_s: &str, d: &mut Diverge) -> String {
    if t.is_empty() {
        diff::compare(orig, &[], orig_s, "", d);
        return String::new();
    }
    let (order, forms, space) = dec.decode(t);
    let out = tks(t, &order, &forms, &space);
    let s = vol::join(&forms, &space);
    diff::compare(orig, &out, orig_s, &s, d);
    s
}

/// The runs of sentences that share one discourse context, one realizer and one world.
pub fn groups(n: usize, unit: Unit) -> Vec<Range<usize>> {
    match unit {
        Unit::Block if n > 0 => vec![0..n],
        Unit::Block => Vec::new(),
        Unit::Sentence | Unit::World => (0..n).map(|i| i..i + 1).collect(),
    }
}

/// Mentions whose entity is a third-person pronoun itself: the reader did not resolve it to an entity.
fn unresolved(fs: &[F]) -> u64 {
    const PRO: [&str; 12] = ["he", "him", "she", "her", "it", "they", "them", "his", "its", "their", "himself", "herself"];
    fs.iter().flat_map(|f| f.ents()).filter(|e| PRO.contains(&e.id.to_lowercase().as_str())).count() as u64
}

/// The gate's negative control: the first residual item of a layer is lost. Whether there was one to lose.
fn damage(l: Layer, r1: &mut vol::R1, r2: &mut [tree::R2], r3: &mut [frames::R3], r4: &mut [world::R4]) -> bool {
    match l {
        Layer::Tokens => {
            if let Some(k) = r1.gaps.keys().next().copied() {
                r1.gaps.remove(&k);
                true
            } else if let Some(k) = r1.tails.keys().next().copied() {
                r1.tails.remove(&k);
                true
            } else {
                false
            }
        }
        Layer::Tree => {
            for r in r2.iter_mut() {
                if let Some(k) = r.spell.keys().next().copied() {
                    r.spell.remove(&k);
                    return true;
                }
                if let Some(k) = r.space.iter().next().copied() {
                    r.space.remove(&k);
                    return true;
                }
                match &mut r.order {
                    tree::Order::Heads(h) => {
                        if let Some(k) = h.keys().next().copied() {
                            h.remove(&k);
                            return true;
                        }
                    }
                    tree::Order::Full(_) => {
                        r.order = tree::Order::Heads(Default::default());
                        return true;
                    }
                }
            }
            false
        }
        Layer::Frames => {
            for r in r3.iter_mut() {
                let hit = r.iter().position(|s| match s {
                    Step::Insert(_) => true,
                    Step::Take { lemma, tag, rel, head, .. } => lemma.is_some() || tag.is_some() || rel.is_some() || head.is_some(),
                });
                if let Some(k) = hit {
                    match &mut r[k] {
                        // the inserted word loses its lemma (the script keeps its length: the tree stays a tree)
                        Step::Insert(x) => {
                            x.lemma.clear();
                        }
                        Step::Take { lemma, tag, rel, head, .. } => {
                            (*lemma, *tag, *rel, *head) = (None, None, None, None);
                        }
                    }
                    return true;
                }
            }
            false
        }
        Layer::World => {
            for r in r4.iter_mut() {
                if let Some(k) = r.patches.keys().next().copied() {
                    r.patches.remove(&k);
                    return true;
                }
                if let Some(k) = r.toggles.iter().next().copied() {
                    r.toggles.remove(&k);
                    return true;
                }
                if let Some(k) = r.inserts.keys().next().copied() {
                    r.inserts.remove(&k);
                    return true;
                }
            }
            false
        }
        Layer::Route => false,
    }
}

pub fn run(a: &Annotator, v: &Volume, o: &Opts) -> Result<Tally> {
    let dec = Dec { a };
    let mut tally = Tally { volumes: 1, bytes: v.text().len() as u64, ..Default::default() };
    let mut segs = Vec::with_capacity(v.blocks.len());
    for (b, blk) in v.blocks.iter().enumerate() {
        segs.push(vol::segment(a, &blk.text, b, blk.whole, blk.gold.as_ref())?);
    }
    let sents: Vec<&Sent> = segs.iter().flat_map(|s| s.sents.iter()).collect();
    let n = sents.len();
    tally.sents = n as u64;
    tally.tokens = sents.iter().map(|s| s.forms.len() as u64).sum();
    let words: Vec<Vec<Word>> = sents.iter().map(|s| a.annotate(&s.forms)).collect();
    let trees: Vec<Vec<TNode>> = words.iter().map(|w| tree::of_words(w)).collect();
    let spaces: Vec<Vec<bool>> = sents.iter().map(|s| s.space()).collect();
    let gs = groups(n, o.unit);
    // the discourse of the reader: one for the volume, unless the unit resets it at every sentence
    let ctx_gs = groups(n, if o.unit == Unit::Sentence { Unit::Sentence } else { Unit::Block });
    let res = &mut tally.residual;
    // up, with the residual of every layer; the realizer and the world are one per group
    // a site page's markup is coded against the markup before it (`vol::Copier`)
    let site = crate::blocks::site_code::is_site(v);
    let mut copier = crate::blocks::site_learn::copier_for(v);
    let mut r1 = vol::residual_with(v, &segs, res, site.then_some(&mut copier));
    let mut r2: Vec<tree::R2> = (0..n).map(|i| dec.encode(&trees[i], &sents[i].forms, &spaces[i], res)).collect();
    let read_all = |ws: &[&Vec<Word>]| -> Vec<Vec<F>> {
        let mut out = Vec::with_capacity(n);
        for g in &ctx_gs {
            let mut ctx = prag::frames::Ctx::default();
            for i in g.clone() {
                out.push(frames::read(ws[i], &mut ctx));
            }
        }
        out
    };
    let fs: Vec<Vec<F>> = read_all(&words.iter().collect::<Vec<_>>());
    let mut realized: Vec<Vec<TNode>> = Vec::with_capacity(n);
    for g in &gs {
        let mut real = Realizer::default();
        for i in g.clone() {
            realized.push(real.sentence(&fs[i]));
        }
    }
    tally.frames = fs.iter().map(|f| f.len() as u64).sum();
    tally.unresolved = fs.iter().map(|f| unresolved(f)).sum();
    let mut r3: Vec<frames::R3> = (0..n).map(|i| frames::encode(&trees[i], &realized[i], &sents[i].forms, res)).collect();
    let (worlds, mut r4s): (Vec<world::World>, Vec<world::R4>) = gs.iter().map(|g| world::encode(&fs[g.clone()], res)).unzip();
    tally.events = worlds.iter().map(|w| w.line.len() as u64).sum();
    // down, exact: world → frames → tree → tokens → text
    if o.exact {
        if let Some(l) = o.damage {
            tally.damaged = damage(l, &mut r1, &mut r2, &mut r3, &mut r4s) as u64;
        }
        let fail = |what: &str, i: usize| format!("{}: {what} differs at sentence {i}: «{}»", v.name, sents.get(i).map(|s| s.original()).unwrap_or_default());
        let fs2: Vec<Vec<F>> = gs.iter().zip(worlds.iter().zip(&r4s)).flat_map(|(g, (w, r))| world::decode_exact(w, r, g.len())).collect();
        let mut bad = (0..n).find(|&i| fs2.get(i) != fs.get(i)).map(|i| fail("frames", i));
        let mut toks2: Vec<Vec<(Vec<String>, Vec<bool>)>> = vec![Vec::new(); v.blocks.len()];
        for g in &gs {
            let mut real2 = Realizer::default();
            for i in g.clone() {
                let rt = real2.sentence(&fs2[i]);
                let t2 = frames::apply(&rt, &r3[i]);
                if t2 != trees[i] {
                    // the gate is red; a tree that is not the parsed one may not even be a tree: not decoded
                    bad = bad.or_else(|| Some(fail("tree", i)));
                    continue;
                }
                let (f2, s2) = dec.decode_exact(&t2, &r2[i]);
                if bad.is_none() && (f2 != sents[i].forms || s2 != spaces[i]) {
                    bad = Some(fail("tokens", i));
                }
                toks2[sents[i].block].push((f2, s2));
            }
        }
        let blocks = vol::decode_blocks(v.blocks.len(), &toks2, &r1);
        let text = vol::volume_text(v, &blocks);
        if text == v.text() && bad.is_none() {
            tally.exact_ok = 1;
        } else {
            tally.exact_fail.push(bad.unwrap_or_else(|| format!("{}: text differs", v.name)));
        }
        // a page's CoNLL-U and frames tables, written from the layers (`blocks::site_code`)
        if site {
            crate::blocks::site_code::run(a, v, o.damage, &mut copier, &mut tally);
        }
    }
    // down, normal: from every start, no residual; every sentence, and the whole volume as one text
    if o.normal {
        let gwords: Vec<&Vec<Word>> = (0..n).map(|i| sents[i].gold.as_ref().unwrap_or(&words[i])).collect();
        let gfs: Vec<Vec<F>> = read_all(&gwords);
        let gws: Vec<world::World> = gs.iter().map(|g| world::encode(&gfs[g.clone()], &mut crate::tally::Residual::default()).0).collect();
        let wfs: Vec<Vec<F>> = gs.iter().zip(&worlds).flat_map(|(g, w)| world::decode_in(w, &fs[g.clone()])).collect();
        let gwfs: Vec<Vec<F>> = gs.iter().zip(&gws).flat_map(|(g, w)| world::decode_in(w, &gfs[g.clone()])).collect();
        let mut whole: BTreeMap<Start, Vec<String>> = BTreeMap::new();
        let mut whole_orig: Vec<String> = Vec::with_capacity(n);
        for g in &gs {
            let (mut rw, mut rg, mut rgw) = (Realizer::default(), Realizer::default(), Realizer::default());
            for i in g.clone() {
                let orig = orig_tks(&words[i], &spaces[i]);
                // the sentence on one line: line wraps and double spaces are layout, measured by the exact mode
                let orig_s = sents[i].original().split_whitespace().collect::<Vec<_>>().join(" ");
                let mut d = std::mem::take(&mut tally.normal);
                let tok_s = vol::join(&sents[i].forms, &spaces[i]);
                diff::compare(&orig, &orig, &orig_s, &tok_s, d.entry(Start::Tokens).or_default());
                whole.entry(Start::Tokens).or_default().push(tok_s);
                let t = match o.drop {
                    Some(c) => drop_class(&trees[i], c),
                    None => trees[i].clone(),
                };
                let s = measure(&dec, &t, &orig, &orig_s, d.entry(Start::Tree).or_default());
                whole.entry(Start::Tree).or_default().push(s);
                let s = measure(&dec, &realized[i], &orig, &orig_s, d.entry(Start::Frames).or_default());
                whole.entry(Start::Frames).or_default().push(s);
                let tw = rw.sentence(&wfs[i]);
                let s = measure(&dec, &tw, &orig, &orig_s, d.entry(Start::World).or_default());
                whole.entry(Start::World).or_default().push(s);
                let tg = rg.sentence(&gfs[i]);
                let tgw = rgw.sentence(&gwfs[i]);
                if let Some(gd) = &sents[i].gold {
                    tally.gold_used += 1;
                    let gorig = orig_tks(gd, &spaces[i]);
                    measure(&dec, &tree::of_words(gd), &gorig, &orig_s, d.entry(Start::GoldTree).or_default());
                    measure(&dec, &tg, &gorig, &orig_s, d.entry(Start::GoldFrames).or_default());
                    measure(&dec, &tgw, &gorig, &orig_s, d.entry(Start::GoldWorld).or_default());
                } else if v.blocks[sents[i].block].gold.is_some() {
                    tally.gold_skipped += 1;
                }
                tally.normal = d;
                whole_orig.push(orig_s);
            }
        }
        // the whole volume: its sentences on one line, decoded from each start
        let join = |v: &[String]| v.iter().filter(|s| !s.is_empty()).cloned().collect::<Vec<_>>().join(" ");
        let orig = join(&whole_orig);
        for (st, outs) in &whole {
            let out = join(outs);
            let d = tally.whole.entry(*st).or_default();
            d.sents += 1;
            d.tokens += tally.tokens;
            d.chrf += en::text::chrf(&out, &orig);
            if out == orig {
                d.exact += 1;
            }
        }
    }
    Ok(tally)
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn grouping_drop_and_damage() {
        assert!(groups(0, Unit::Block).is_empty());
        assert_eq!(groups(3, Unit::Block), vec![0..3]);
        assert_eq!(groups(2, Unit::Sentence), vec![0..1, 1..2]);
        assert_eq!(groups(2, Unit::World), groups(2, Unit::Sentence));
        let t = vec![
            TNode {
                lemma: "run".into(),
                tag: en::gram::Tag::VB,
                head: 0,
                rel: en::gram::Rel::Root,
            },
            TNode {
                lemma: "not".into(),
                tag: en::gram::Tag::RB,
                head: 1,
                rel: en::gram::Rel::Advmod,
            },
            TNode {
                lemma: "today".into(),
                tag: en::gram::Tag::NN,
                head: 2,
                rel: en::gram::Rel::Obl,
            },
        ];
        let out = drop_class(&t, crate::class::Class::Neg);
        assert_eq!(out.len(), 2);
        assert_eq!(out[1].head, 1);
        let mut r1 = vol::R1::default();
        let mut r3 = vec![vec![Step::Insert(t[0].clone())]];
        assert!(damage(Layer::Frames, &mut r1, &mut [], &mut r3, &mut []));
        assert_eq!(r3[0].len(), 1);
        assert!(matches!(&r3[0][0], Step::Insert(x) if x.lemma.is_empty()));
        assert!(!damage(Layer::Route, &mut r1, &mut [], &mut [], &mut []));
    }
}
