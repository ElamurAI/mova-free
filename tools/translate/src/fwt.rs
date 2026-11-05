//! Function words between two authors: a small table, learned per author pair, of the determiners, prepositions,
//! conjunctions and markers one author writes where the other writes none. It works on the meta-world: a function
//! token of the source the target drops in this context goes; a run of function tokens the target puts between two
//! source tokens comes in. Everything else is the interlingua's. A rule is kept when it holds for at least 3 cases and
//! half of its context's. It needs the pair's own aligned verses, so a pair never trained has none.

use std::collections::{BTreeMap, BTreeSet};

use en::gram::{Rel, Tag};
use roundtrip::class::{Class, of_word};

use crate::bible::VKey;
use crate::corpus::Corpus;
use crate::meta::{Lexicon, MTok, gaps, lcs};

fn is_function(x: &MTok) -> bool {
    !roundtrip::class::is_punct_tag(x.tag) && of_word(x.tag, x.rel, &x.concept) == Class::Function
}

#[derive(Clone, Debug, Default)]
pub struct FwTable {
    pub name: String,
    /// function concept | before | after → (dropped, kept)
    drop: BTreeMap<String, (u32, u32)>,
    /// before | after → inserted run («concept/TAG …») → count; and the cases with nothing inserted
    ins: BTreeMap<String, BTreeMap<String, u32>>,
    none: BTreeMap<String, u32>,
    pub drops: BTreeSet<String>,
    pub inserts: BTreeMap<String, String>,
    /// concepts the target never writes, and what it writes in their place (*unto* → *to*, *thee* → *you*)
    pub subs: BTreeMap<String, String>,
}

fn key3(m: &[MTok], i: usize) -> String {
    let prev = if i == 0 { "^" } else { m[i - 1].concept.as_str() };
    let next = m.get(i + 1).map_or("$", |x| x.concept.as_str());
    format!("{}|{prev}|{next}", m[i].concept)
}

fn key2(m: &[MTok], i: usize) -> String {
    // between token i-1 and token i of the source
    let prev = if i == 0 { "^" } else { m[i - 1].concept.as_str() };
    let next = m.get(i).map_or("$", |x| x.concept.as_str());
    format!("{prev}|{next}")
}

impl FwTable {
    /// Learn from the pair's training verses (source a, target b). `shuffle`: each source verse with another target
    /// verse (a negative control).
    pub fn train(name: &str, lex: &Lexicon, a: &Corpus, b: &Corpus, verses: &BTreeSet<VKey>, shuffle: bool) -> FwTable {
        let ks: Vec<VKey> = verses.iter().filter(|k| a.verses.contains_key(k) && b.verses.contains_key(k)).copied().collect();
        let n = ks.len();
        let mut t = FwTable { name: name.to_string(), ..Default::default() };
        let (mut occ_a, mut occ_b): (BTreeMap<String, u32>, BTreeMap<String, u32>) = (BTreeMap::new(), BTreeMap::new());
        let mut swap: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
        for (i, k) in ks.iter().enumerate() {
            let kb = if shuffle { ks[(i + 7919) % n.max(1)] } else { *k };
            let ma = lex.read(&a.verses[k].meta_words());
            let mb = lex.read(&b.verses[&kb].meta_words());
            let ca: Vec<String> = ma.iter().map(|x| x.concept.clone()).collect();
            let cb: Vec<String> = mb.iter().map(|x| x.concept.clone()).collect();
            let an = lcs(&ca, &cb);
            for c in &ca {
                *occ_a.entry(c.clone()).or_default() += 1;
            }
            for c in &cb {
                *occ_b.entry(c.clone()).or_default() += 1;
            }
            for ((a0, a1), (b0, b1)) in gaps(ma.len(), mb.len(), &an) {
                if a1 == a0 + 1 && b1 == b0 + 1 {
                    *swap.entry(ca[a0].clone()).or_default().entry(cb[b0].clone()).or_default() += 1;
                }
            }
            for &(ia, _) in &an {
                if is_function(&ma[ia]) {
                    t.drop.entry(key3(&ma, ia)).or_default().1 += 1;
                }
            }
            // places with nothing between two neighbouring anchors on either side
            for w in an.windows(2) {
                if w[1].0 == w[0].0 + 1 && w[1].1 == w[0].1 + 1 {
                    *t.none.entry(key2(&ma, w[1].0)).or_default() += 1;
                }
            }
            for ((a0, a1), (b0, b1)) in gaps(ma.len(), mb.len(), &an) {
                let fa = (a0..a1).all(|x| is_function(&ma[x]));
                let fb = (b0..b1).all(|x| is_function(&mb[x]));
                if b0 == b1 && fa {
                    for x in a0..a1 {
                        t.drop.entry(key3(&ma, x)).or_default().0 += 1;
                    }
                } else if a0 == a1 && fb && b1 > b0 {
                    let run = (b0..b1).map(|x| format!("{}/{}", mb[x].concept, mb[x].tag)).collect::<Vec<_>>().join(" ");
                    *t.ins.entry(key2(&ma, a0)).or_default().entry(run).or_default() += 1;
                }
            }
        }
        // a concept the target writes in less than 5% of the places the source does, swapped to what the target
        // writes there most (at least 5 times and in 30% of the source's places)
        for (x, m) in &swap {
            let (na, nb) = (occ_a.get(x).copied().unwrap_or(0), occ_b.get(x).copied().unwrap_or(0));
            let (z, c) = m.iter().fold((String::new(), 0u32), |acc, (z, &v)| if v > acc.1 { (z.clone(), v) } else { acc });
            if nb as f64 <= 0.05 * na as f64 && c >= 5 && c as f64 >= 0.3 * na as f64 {
                t.subs.insert(x.clone(), z);
            }
        }
        for (k, &(d, kept)) in &t.drop {
            if d >= 3 && d as f64 >= 0.5 * (d + kept) as f64 {
                t.drops.insert(k.clone());
            }
        }
        for (k, runs) in &t.ins {
            let (best, c) = runs.iter().fold((String::new(), 0u32), |acc, (r, &v)| if v > acc.1 { (r.clone(), v) } else { acc });
            let total: u32 = runs.values().sum::<u32>() + t.none.get(k).copied().unwrap_or(0);
            if c >= 3 && c as f64 >= 0.5 * total as f64 {
                t.inserts.insert(k.clone(), best);
            }
        }
        t
    }

    pub fn size(&self) -> usize {
        self.drops.len() + self.inserts.len() + self.subs.len()
    }

    /// The source's meta-world with the target's function-word habits applied.
    pub fn apply(&self, m: &[MTok]) -> Vec<MTok> {
        let swapped: Vec<MTok> = m.iter().map(|x| match self.subs.get(&x.concept) {
            Some(z) => MTok { concept: z.clone(), ..x.clone() },
            None => x.clone(),
        }).collect();
        let m = &swapped[..];
        let n = m.len();
        let gone: Vec<bool> = (0..n).map(|i| is_function(&m[i]) && self.drops.contains(&key3(m, i))).collect();
        // new positions: inserted runs before token i, then token i if kept
        let mut out: Vec<MTok> = Vec::new();
        let mut new_of = vec![0usize; n];
        let mut inserted: Vec<(usize, usize)> = Vec::new(); // (out index, source token it goes before)
        for i in 0..=n {
            if let Some(run) = self.inserts.get(&key2(m, i)) {
                for e in run.split(' ').filter(|x| !x.is_empty()) {
                    let (c, tg) = e.rsplit_once('/').unwrap_or((e, "IN"));
                    inserted.push((out.len(), i));
                    out.push(MTok { concept: c.to_string(), tag: Tag::parse(tg).unwrap_or(Tag::IN), head: 0, rel: Rel::Det, span: (0, 0) });
                }
            }
            if i < n && !gone[i] {
                new_of[i] = out.len() + 1;
                out.push(m[i].clone());
            }
        }
        // heads: a kept token's head through dropped ones; an inserted one on the next kept token (or the previous)
        let up = |mut h: usize| -> usize {
            for _ in 0..=n {
                if h == 0 {
                    return 0;
                }
                if !gone[h - 1] {
                    return new_of[h - 1];
                }
                h = m[h - 1].head;
            }
            0
        };
        for i in 0..n {
            if !gone[i] {
                let x = new_of[i] - 1;
                out[x].head = up(m[i].head);
            }
        }
        for (oi, before) in inserted {
            let next = (before..n).find(|&j| !gone[j]).map(|j| new_of[j]).or_else(|| (0..before).rev().find(|&j| !gone[j]).map(|j| new_of[j]));
            out[oi].head = next.filter(|&h| h != oi + 1).unwrap_or(0);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mt(c: &str, tag: Tag, head: usize, rel: Rel) -> MTok {
        MTok { concept: c.into(), tag, head, rel, span: (0, 0) }
    }

    #[test]
    fn a_fronted_part_moves_after_its_root() {
        let m = vec![mt("after", Tag::IN, 3, Rel::Obl), mt("death", Tag::NN, 1, Rel::Nmod), mt("come", Tag::VBD, 0, Rel::Root), mt("it", Tag::PRP, 3, Rel::Nsubj)];
        let mut t = ReorderTable::default();
        t.moves.insert("obl|pre|after".into());
        let o = t.apply(&m);
        let cs: Vec<&str> = o.iter().map(|x| x.concept.as_str()).collect();
        assert_eq!(cs, vec!["come", "after", "death", "it"]);
        assert_eq!((o[0].head, o[1].head, o[2].head, o[3].head), (0, 1, 2, 1));
        // and back: the same part after its root goes to the clause's start
        let mut back = ReorderTable::default();
        back.moves.insert("obl|post|after".into());
        let b = back.apply(&o);
        assert_eq!(b.iter().map(|x| x.concept.as_str()).collect::<Vec<_>>(), vec!["after", "death", "come", "it"]);
        // an empty table changes nothing
        assert_eq!(ReorderTable::default().apply(&m), m);
    }

    #[test]
    fn drops_and_inserts() {
        let m = vec![mt("and", Tag::CC, 3, Rel::Cc), mt("he", Tag::PRP, 3, Rel::Nsubj), mt("say", Tag::VBD, 0, Rel::Root)];
        let mut t = FwTable::default();
        t.drops.insert("and|^|he".into());
        t.inserts.insert("say|$".into(), "unto/IN".into());
        let o = t.apply(&m);
        let cs: Vec<&str> = o.iter().map(|x| x.concept.as_str()).collect();
        assert_eq!(cs, vec!["he", "say", "unto"]);
        assert_eq!(o[0].head, 2);
        assert_eq!(o[1].head, 0);
        assert_eq!(o[2].head, 2);
        // an empty table changes nothing
        assert_eq!(FwTable::default().apply(&m), m);
    }
}

/// The joining of clauses, as a choice conditioned on the source: for a target author, how often it writes each
/// joining (`#j:<relation>:<joiners>`) where a source author wrote another, at clause starts the two share (the
/// longest common subsequence of the concepts without the feature). Learned from every trained pair into the author;
/// a mapping is kept with at least 3 cases and half of the source joining's.
#[derive(Clone, Debug, Default)]
pub struct JoinTable {
    pub name: String,
    pub map: BTreeMap<String, String>,
}

fn split_join(c: &str) -> (&str, Option<&str>) {
    match c.split_once("#j:") {
        Some((b, f)) => (b, Some(f)),
        None => (c, None),
    }
}

impl JoinTable {
    pub fn train(name: &str, lex: &Lexicon, pairs: &[(&Corpus, &Corpus)], verses: &BTreeSet<VKey>, shuffle: bool) -> JoinTable {
        let mut counts: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
        for (a, b) in pairs {
            let ks: Vec<VKey> = verses.iter().filter(|k| a.verses.contains_key(k) && b.verses.contains_key(k)).copied().collect();
            let n = ks.len();
            for (i, k) in ks.iter().enumerate() {
                let kb = if shuffle { ks[(i + 7919) % n.max(1)] } else { *k };
                let ma = lex.read(&a.verses[k].meta_words());
                let mb = lex.read(&b.verses[&kb].meta_words());
                let ca: Vec<String> = ma.iter().map(|x| split_join(&x.concept).0.to_string()).collect();
                let cb: Vec<String> = mb.iter().map(|x| split_join(&x.concept).0.to_string()).collect();
                for (ia, ib) in lcs(&ca, &cb) {
                    if let (Some(fa), Some(fb)) = (split_join(&ma[ia].concept).1, split_join(&mb[ib].concept).1) {
                        *counts.entry(fa.to_string()).or_default().entry(fb.to_string()).or_default() += 1;
                    }
                }
            }
        }
        let mut t = JoinTable { name: name.to_string(), map: BTreeMap::new() };
        for (src, m) in counts {
            let total: u32 = m.values().sum();
            let (best, c) = m.iter().fold((String::new(), 0u32), |acc, (x, &v)| if v > acc.1 { (x.clone(), v) } else { acc });
            if c >= 3 && c as f64 >= 0.5 * total as f64 && best != src {
                t.map.insert(src, best);
            }
        }
        t
    }

    pub fn apply(&self, m: &[MTok]) -> Vec<MTok> {
        m.iter()
            .map(|x| match split_join(&x.concept) {
                (b, Some(f)) => match self.map.get(f) {
                    Some(t) => MTok { concept: format!("{b}#j:{t}"), ..x.clone() },
                    None => x.clone(),
                },
                _ => x.clone(),
            })
            .collect()
    }
}

/// The place of a clause's parts, learned per target author: where a source author puts a part of a clause before
/// its predicate («Now after the death of Moses … it came to pass») and the target after it, or the reverse. A part is
/// a dependent of a clause's root (subject, object, oblique, modifier, clause link) with its subtree; it is keyed by
/// its relation, its side in the source and its first concept. A move is kept with at least 5 cases and half of the
/// key's.
#[derive(Clone, Debug, Default)]
pub struct ReorderTable {
    pub name: String,
    /// relation | side | first concept → the other side
    pub moves: BTreeSet<String>,
    /// the keys moved most often, with their counts (for the report)
    pub top: Vec<String>,
}

fn kids_of(m: &[MTok], h: usize) -> Vec<usize> {
    (0..m.len()).filter(|&i| m[i].head == h + 1).collect()
}

/// The subtree of a meta token as an index range, if it is contiguous.
fn span_of(m: &[MTok], d: usize) -> Option<(usize, usize)> {
    let mut inside = vec![false; m.len()];
    inside[d] = true;
    let mut grew = true;
    while grew {
        grew = false;
        for i in 0..m.len() {
            let h = m[i].head;
            if !inside[i] && h > 0 && inside[h - 1] {
                inside[i] = true;
                grew = true;
            }
        }
    }
    let lo = (0..m.len()).find(|&i| inside[i])?;
    let hi = (0..m.len()).rev().find(|&i| inside[i])?;
    (lo..=hi).all(|i| inside[i]).then_some((lo, hi + 1))
}

fn part_key(m: &[MTok], d: usize, root: usize, span: (usize, usize)) -> String {
    let side = if d < root { "pre" } else { "post" };
    format!("{}|{side}|{}", m[d].rel, crate::meta::bare_concept(&m[span.0].concept))
}

impl ReorderTable {
    pub fn train(name: &str, lex: &Lexicon, pairs: &[(&Corpus, &Corpus)], verses: &BTreeSet<VKey>, shuffle: bool) -> ReorderTable {
        let mut counts: BTreeMap<String, (u32, u32)> = BTreeMap::new();
        for (a, b) in pairs {
            let ks: Vec<VKey> = verses.iter().filter(|k| a.verses.contains_key(k) && b.verses.contains_key(k)).copied().collect();
            let n = ks.len();
            for (i, k) in ks.iter().enumerate() {
                let kb = if shuffle { ks[(i + 7919) % n.max(1)] } else { *k };
                let ma = lex.read(&a.verses[k].meta_words());
                let mb = lex.read(&b.verses[&kb].meta_words());
                let ca: Vec<String> = ma.iter().map(|x| x.concept.clone()).collect();
                let cb: Vec<String> = mb.iter().map(|x| x.concept.clone()).collect();
                // words of the same concept that each verse has once align wherever they stand (the longest common
                // subsequence keeps the order, so it never sees a part move); the rest by the subsequence
                let mut a2b: BTreeMap<usize, usize> = lcs(&ca, &cb).into_iter().collect();
                let count = |c: &[String], x: &str| c.iter().filter(|y| *y == x).count();
                for (ia, x) in ca.iter().enumerate() {
                    if count(&ca, x) == 1 && count(&cb, x) == 1 {
                        a2b.insert(ia, cb.iter().position(|y| y == x).unwrap());
                    }
                }
                for r in (0..ma.len()).filter(|&r| ma[r].head == 0) {
                    let Some(&rb) = a2b.get(&r) else { continue };
                    for d in kids_of(&ma, r) {
                        if ma[d].rel == Rel::Punct {
                            continue;
                        }
                        let Some(sp) = span_of(&ma, d) else { continue };
                        let Some(&db) = a2b.get(&d) else { continue };
                        if mb[db].head != rb + 1 {
                            continue;
                        }
                        let e = counts.entry(part_key(&ma, d, r, sp)).or_default();
                        if (d < r) != (db < rb) {
                            e.0 += 1;
                        } else {
                            e.1 += 1;
                        }
                    }
                }
            }
        }
        let mut t = ReorderTable { name: name.to_string(), moves: BTreeSet::new(), top: Vec::new() };
        let mut top: Vec<(u32, u32, String)> = counts.iter().map(|(k, &(m, st))| (m, st, k.clone())).collect();
        top.sort_by(|a, b| b.0.cmp(&a.0).then(a.2.cmp(&b.2)));
        t.top = top.into_iter().take(8).map(|(m, st, k)| format!("{k} {m}/{}", m + st)).collect();
        for (k, (mv, stay)) in counts {
            if mv >= 5 && mv as f64 >= 0.5 * (mv + stay) as f64 {
                t.moves.insert(k);
            }
        }
        t
    }

    /// The source's meta-world with its parts moved where the target puts them: a part after its root moves to the
    /// start of the root's clause; a part before it moves right after the root.
    pub fn apply(&self, m: &[MTok]) -> Vec<MTok> {
        let mut order: Vec<usize> = (0..m.len()).collect();
        for r in (0..m.len()).filter(|&r| m[r].head == 0) {
            for d in kids_of(m, r) {
                let Some(sp) = span_of(m, d) else { continue };
                if !self.moves.contains(&part_key(m, d, r, sp)) {
                    continue;
                }
                let Some(clause) = span_of(m, r) else { continue };
                let part: Vec<usize> = (sp.0..sp.1).collect();
                order.retain(|x| !part.contains(x));
                let at = if d > r {
                    // to the start of the clause: before its first word still in place
                    order.iter().position(|&x| x >= clause.0 && x < clause.1).unwrap_or(0)
                } else {
                    order.iter().position(|&x| x == r).map_or(order.len(), |p| p + 1)
                };
                for (j, x) in part.into_iter().enumerate() {
                    order.insert(at + j, x);
                }
            }
        }
        let mut new_of = vec![0usize; m.len()];
        for (ni, &oi) in order.iter().enumerate() {
            new_of[oi] = ni + 1;
        }
        order.iter().map(|&oi| MTok { head: if m[oi].head == 0 { 0 } else { new_of[m[oi].head - 1] }, ..m[oi].clone() }).collect()
    }
}
