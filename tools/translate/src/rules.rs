//! Residual rules between two translations: what, applied to one translation's meta-world, brings it nearest the
//! other's. For an ordered pair (source → target) the reader's script from the source's meta-world to the target's
//! names, verse by verse, what it patches and what it leaves out; each patch is a candidate rule:
//! - **concept**: the source's concept A is the target's B in the same place (another word for one thing, or another
//!   thing: «behold» for «have not»);
//! - **tag**: the same concept with another part of speech or form (a past for a present, a plural for a singular);
//! - **drop**: a source concept the target has nothing for.
//!
//! A rule is learned on one half of the development blocks and measured on the other (two folds, each rule on the
//! half it was not learned on): its gain is the bits of the reader's residual it saves on the verses it applies to.
//! The same rule is measured with the verses paired by the wrong number inside their block (the control): a rule
//! that helps as much there is a habit of the target's style, not a correspondence of the two texts. Rules are
//! ranked by their gain, then joined greedily — each next rule measured on top of those before it — so the list
//! says how much of the distance each one removes. The idea of learning a transfer as a short list of rewrite rules
//! ranked by what they gain is transformation-based learning (brill-1995).

use std::collections::BTreeMap;

use en::gram::Tag;
use roundtrip::frames::Step;

use crate::measure;
use crate::meta::MTok;

/// A rule: what it rewrites in the source's meta-world.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Concept(String, String),
    Tag(String, Tag, Tag),
    Drop(String),
}

impl Rule {
    pub fn show(&self) -> String {
        match self {
            Rule::Concept(a, b) => format!("«{a}» → «{b}»"),
            Rule::Tag(c, a, b) => format!("«{c}» {a:?} → {b:?}"),
            Rule::Drop(c) => format!("drop «{c}»"),
        }
    }
    pub fn kind(&self) -> &'static str {
        match self {
            Rule::Concept(..) => "concept",
            Rule::Tag(..) => "tag",
            Rule::Drop(..) => "drop",
        }
    }
    fn touches(&self, ms: &[MTok]) -> bool {
        match self {
            Rule::Concept(a, _) | Rule::Drop(a) => ms.iter().any(|m| &m.concept == a),
            Rule::Tag(c, a, _) => ms.iter().any(|m| &m.concept == c && m.tag == *a),
        }
    }
    /// The source's meta-world with the rule applied; a dropped token's dependents go to its head.
    pub fn apply(&self, ms: &[MTok]) -> Vec<MTok> {
        match self {
            Rule::Concept(a, b) => ms.iter().map(|m| if &m.concept == a { MTok { concept: b.clone(), ..m.clone() } } else { m.clone() }).collect(),
            Rule::Tag(c, a, b) => ms.iter().map(|m| if &m.concept == c && m.tag == *a { MTok { tag: *b, ..m.clone() } } else { m.clone() }).collect(),
            Rule::Drop(c) => {
                let gone: Vec<bool> = ms.iter().map(|m| &m.concept == c).collect();
                // a head through dropped tokens to the first kept one (or the root)
                let up = |mut h: usize| {
                    while h > 0 && gone[h - 1] {
                        h = ms[h - 1].head;
                    }
                    h
                };
                let new_id: Vec<usize> = gone.iter().scan(0usize, |n, g| { if !g { *n += 1; } Some(*n) }).collect();
                ms.iter().enumerate().filter(|(i, _)| !gone[*i]).map(|(_, m)| { let h = up(m.head); MTok { head: if h == 0 { 0 } else { new_id[h - 1] }, ..m.clone() } }).collect()
            }
        }
    }
}

/// The candidate rules of one verse pair, from the reader's script.
pub fn candidates(ms: &[MTok], mt: &[MTok]) -> Vec<Rule> {
    let mut out = Vec::new();
    let mut taken = vec![false; ms.len()];
    for (j, st) in measure::reader_script(ms, mt).iter().enumerate() {
        if let Step::Take { from, lemma, tag, .. } = st {
            let Some(s) = ms.get(*from) else { continue };
            taken[*from] = true;
            if let Some(l) = lemma {
                let t = mt.get(j).map(|m| m.concept.clone()).unwrap_or_else(|| l.clone());
                if t != s.concept {
                    out.push(Rule::Concept(s.concept.clone(), t));
                }
            }
            if let Some(t) = tag {
                if *t != s.tag {
                    out.push(Rule::Tag(s.concept.clone(), s.tag, *t));
                }
            }
        }
    }
    for (i, t) in taken.iter().enumerate() {
        if !t {
            out.push(Rule::Drop(ms[i].concept.clone()));
        }
    }
    out
}

/// One verse pair of a translation pair: its fold (the block's parity), the source's and the target's meta-world, and
/// the source paired by the wrong number (the control).
pub struct VPair<'a> {
    pub fold: usize,
    pub src: &'a [MTok],
    pub tgt: &'a [MTok],
    pub wrong: &'a [MTok],
}

/// A measured rule.
#[derive(Clone, Debug)]
pub struct Found {
    pub rule: Rule,
    /// verse pairs it was learned from (both folds)
    pub support: usize,
    /// bits saved on the held-out fold, on the true pairing and on the wrong one
    pub gain: f64,
    pub control: f64,
    /// bits saved by this rule on top of the ones before it in the greedy list
    pub step: f64,
}

/// The bits a rule saves on the verse pairs it applies to, against each pair's bits without it.
fn gain(rule: &Rule, vs: &[(&VPair, f64, f64)], wrong: bool) -> f64 {
    vs.iter()
        .map(|(v, base, base_wrong)| {
            let s = if wrong { v.wrong } else { v.src };
            if !rule.touches(s) {
                return 0.0;
            }
            (if wrong { base_wrong } else { base }) - measure::reader_bits(&rule.apply(s), v.tgt)
        })
        .sum()
}

/// The candidates kept per fold: the most frequent ones (the rest are measured too rarely to rank).
pub const CANDIDATES: usize = 30;

/// The rules of one ordered pair: learned per fold, measured on the other, ranked, then joined greedily (`top`).
pub fn learn(vs: &[VPair], min_support: usize, top: usize) -> Vec<Found> {
    // each verse pair's bits without a rule, on the true pairing and on the wrong one, measured once
    let based: Vec<(&VPair, f64, f64)> = vs.iter().map(|v| (v, measure::reader_bits(v.src, v.tgt), measure::reader_bits(v.wrong, v.tgt))).collect();
    let mut found: BTreeMap<Rule, Found> = BTreeMap::new();
    for fold in 0..2 {
        let test_on: Vec<(&VPair, f64, f64)> = based.iter().filter(|b| b.0.fold == fold).copied().collect();
        let mut count: BTreeMap<Rule, usize> = BTreeMap::new();
        for v in vs.iter().filter(|v| v.fold != fold) {
            let mut seen: Vec<Rule> = candidates(v.src, v.tgt);
            seen.sort();
            seen.dedup();
            for r in seen {
                *count.entry(r).or_default() += 1;
            }
        }
        let mut frequent: Vec<(Rule, usize)> = count.into_iter().filter(|(_, n)| *n >= min_support).collect();
        frequent.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        for (r, n) in frequent.into_iter().take(CANDIDATES) {
            let g = gain(&r, &test_on, false);
            let c = gain(&r, &test_on, true);
            let f = found.entry(r.clone()).or_insert(Found { rule: r, support: 0, gain: 0.0, control: 0.0, step: 0.0 });
            f.support += n;
            f.gain += g;
            f.control += c;
        }
    }
    let mut ranked: Vec<Found> = found.into_values().filter(|f| f.gain > 0.0).collect();
    ranked.sort_by(|a, b| b.gain.total_cmp(&a.gain).then_with(|| a.rule.cmp(&b.rule)));
    // greedy: each next rule on top of the ones before it, on all the verses
    let mut cur: Vec<(Vec<MTok>, f64)> = based.iter().map(|(v, b, _)| (v.src.to_vec(), *b)).collect();
    let mut kept = Vec::new();
    for mut f in ranked.into_iter().take(top) {
        let mut step = 0.0;
        let next: Vec<(Vec<MTok>, f64)> = cur
            .iter()
            .zip(vs)
            .map(|((s, b), v)| {
                if !f.rule.touches(s) {
                    return (s.clone(), *b);
                }
                let n = f.rule.apply(s);
                let nb = measure::reader_bits(&n, v.tgt);
                step += b - nb;
                (n, nb)
            })
            .collect();
        if step <= 0.0 {
            continue;
        }
        f.step = step;
        cur = next;
        kept.push(f);
    }
    kept
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::Rel;

    fn t(c: &str, tag: Tag, head: usize) -> MTok {
        MTok { concept: c.into(), tag, head, rel: Rel::Dep, span: (0, 0) }
    }

    #[test]
    fn drop_reattaches_and_a_rule_that_fixes_gains() {
        // «behold I command thee» against «I command thee»
        let src = vec![t("behold", Tag::UH, 3), t("i", Tag::PRP, 3), t("command", Tag::VBP, 0), t("thee", Tag::PRP, 3)];
        let tgt = vec![t("i", Tag::PRP, 2), t("command", Tag::VBP, 0), t("thee", Tag::PRP, 2)];
        let d = Rule::Drop("behold".into()).apply(&src);
        assert_eq!(d.len(), 3);
        assert_eq!(d.iter().map(|m| m.head).collect::<Vec<_>>(), vec![2, 0, 2]);
        assert!(candidates(&src, &tgt).contains(&Rule::Drop("behold".into())));
        let vs: Vec<VPair> = (0..6).map(|i| VPair { fold: i % 2, src: &src, tgt: &tgt, wrong: &tgt }).collect();
        let found = learn(&vs, 2, 5);
        assert_eq!(found.first().map(|f| f.rule.clone()), Some(Rule::Drop("behold".into())));
        assert!(found[0].gain > 0.0 && found[0].step > 0.0);
        // negative control: the same texts on both sides give no rule
        let same: Vec<VPair> = (0..6).map(|i| VPair { fold: i % 2, src: &tgt, tgt: &tgt, wrong: &tgt }).collect();
        assert!(learn(&same, 2, 5).is_empty());
    }
}
