//! One verse translated and measured, in both modes.
//!
//! Exact mode: the target verse must come back byte for byte, and what it takes is split by stage:
//! - **reader** (world → meta-world): the edit script from the meta-world read from the source to the one read from
//!   the target (roundtrip's tree script, on meta tokens), plus the meta tokens out of place. Zero when source and
//!   target are the same text: this is the disagreement of the interlingua.
//! - **encoder** (meta-world → the target author's words: the standard encoder with the author's delta): the tree
//!   script from the parse of what the encoder writes for the target's own meta-world to the target's standard tree.
//!   It does not depend on the source: the encoder's loss as an autoencoder.
//! - **stack** (the standard English stack below): the target tree to its tokens with roundtrip's tree decoder, told
//!   the encoder's order and forms (`tree::Hints`), and the tokens to the verse text (literal gaps, the tail).
//! The decoder runs the stages down and the gate compares every layer on the way and the text at the end.
//!
//! Normal mode: the encoder with the target author's delta writes the source's meta-world with no residual; exact match and chrF
//! against the target verse, and roundtrip's divergence classes on the parsed output.

use std::collections::BTreeMap;

use en::annotate::{Annotator, Word};
use roundtrip::class::{Class, Layer, lit_bits, pos_bits};
use roundtrip::diff::{self, Tk};
use roundtrip::frames::{self, R3, Step};
use roundtrip::tally::{Diverge, Residual};
use roundtrip::tree::{self, Dec, Hints, TNode};

use crate::corpus::{PVerse, parse_tokens};
use crate::meta::{MTok, tnodes};
use crate::profile::Encoder;

/// The stages of the exact residual.
pub const STAGES: [&str; 3] = ["reader", "encoder", "stack"];

#[derive(Clone, Debug, Default)]
pub struct Out {
    /// per stage
    pub res: [Residual; 3],
    pub gate_ok: bool,
    pub fail: Option<String>,
    pub text: String,
    pub exact: bool,
    pub chrf: f64,
    pub div: Diverge,
    pub why: Vec<String>,
    /// the direct code of a translation: the target from the normal-mode output (tree script + stack), no split
    pub direct: Option<Residual>,
    pub direct_ok: bool,
}

/// The source of a translation: a meta-world read from the source, or (a baseline) the target words given directly.
pub enum Src<'a> {
    Meta(&'a [MTok]),
    Words(Vec<String>),
}

/// The base order of a script's target words: each taken word at the place of the word it was taken from, an inserted
/// word right after the word before it.
fn base_order(script: &R3) -> Vec<usize> {
    let mut keys: Vec<((i64, usize), usize)> = Vec::with_capacity(script.len());
    let (mut last, mut sub) = (-1i64, 0usize);
    for (i, s) in script.iter().enumerate() {
        match s {
            Step::Take { from, .. } => {
                last = *from as i64;
                sub = 0;
                keys.push(((last, 0), i + 1));
            }
            Step::Insert(_) => {
                sub += 1;
                keys.push(((last, sub), i + 1));
            }
        }
    }
    keys.sort();
    keys.into_iter().map(|(_, i)| i).collect()
}

/// The words out of place against a base order: those off its longest increasing run. The residual names each one
/// and its place.
fn moved(base: &[usize]) -> Vec<usize> {
    // longest increasing subsequence, O(n log n), the lowest tails
    let n = base.len();
    let mut tails: Vec<usize> = Vec::new();
    let mut tail_idx: Vec<usize> = Vec::new();
    let mut prev = vec![usize::MAX; n];
    for (k, &x) in base.iter().enumerate() {
        let p = tails.partition_point(|&t| t < x);
        if p > 0 {
            prev[k] = tail_idx[p - 1];
        }
        if p == tails.len() {
            tails.push(x);
            tail_idx.push(k);
        } else {
            tails[p] = x;
            tail_idx[p] = k;
        }
    }
    let mut keep = vec![false; n];
    let mut k = tail_idx.last().copied().unwrap_or(usize::MAX);
    while k != usize::MAX {
        keep[k] = true;
        k = prev[k];
    }
    let mut out: Vec<usize> = (0..n).filter(|&k| !keep[k]).map(|k| base[k]).collect();
    out.sort();
    out
}

/// Decode an order: the base without the moved words must be increasing; the moved words go to their places.
fn order_ok(base: &[usize], mv: &[usize]) -> bool {
    let kept: Vec<usize> = base.iter().copied().filter(|x| mv.binary_search(x).is_err()).collect();
    kept.windows(2).all(|w| w[0] < w[1]) && kept.len() + mv.len() == base.len()
}

/// Pairs of words two trees in text order share: the longest common subsequence of lemmas, then the words of gaps of
/// equal length place by place (a word for a word). The script's own alignment fills in the rest.
fn prealign(t: &[TNode], r: &[TNode]) -> Vec<Option<usize>> {
    let a: Vec<String> = t.iter().map(|x| x.lemma.to_lowercase()).collect();
    let b: Vec<String> = r.iter().map(|x| x.lemma.to_lowercase()).collect();
    let an = crate::meta::lcs(&a, &b);
    let mut out = vec![None; t.len()];
    for &(i, j) in &an {
        out[i] = Some(j);
    }
    for ((a0, a1), (b0, b1)) in crate::meta::gaps(a.len(), b.len(), &an) {
        if a1 - a0 == b1 - b0 {
            for d in 0..a1 - a0 {
                out[a0 + d] = Some(b0 + d);
            }
        }
    }
    out
}

/// log2 of n choose k.
fn choose_bits(n: usize, k: usize) -> f64 {
    (0..k).map(|i| ((n - i) as f64).log2() - ((i + 1) as f64).log2()).sum()
}

/// The reader stage's bits for one pair of meta-worlds (the script and the words out of place), as `Run::verse`
/// counts them: the measure the lexicon's judge can be calibrated on.
pub fn reader_bits(src: &[MTok], tgt: &[MTok]) -> f64 {
    reader_res(src, tgt).0.cells.values().map(|c| c.bits).sum()
}

/// The reader stage's residual for one pair of meta-worlds, by class (the tokens out of place under `Order`), and
/// what has no counterpart: the target tokens the script inserts (nothing in the source stands for them) and the
/// source tokens it takes nothing from.
pub fn reader_res(src: &[MTok], tgt: &[MTok]) -> (Residual, Vec<usize>, Vec<usize>) {
    let (sm, tm) = (tnodes(src), tnodes(tgt));
    let concepts: Vec<String> = tm.iter().map(|x| x.lemma.clone()).collect();
    let mut scratch = Residual::default();
    let script = frames::encode_with(&tm, &sm, &concepts, &prealign(&tm, &sm), &mut scratch);
    let mv = moved(&base_order(&script));
    if !mv.is_empty() {
        let each = choose_bits(tm.len(), mv.len()) / mv.len() as f64 + pos_bits(tm.len());
        for _ in &mv {
            scratch.add(Layer::World, Class::Order, each, String::new);
        }
    }
    let mut taken = vec![false; src.len()];
    let mut inserted = Vec::new();
    for (i, s) in script.iter().enumerate() {
        match s {
            Step::Take { from, .. } => {
                if let Some(t) = taken.get_mut(*from) {
                    *t = true;
                }
            }
            Step::Insert(_) => inserted.push(i),
        }
    }
    let deleted = (0..src.len()).filter(|&i| !taken[i]).collect();
    (scratch, inserted, deleted)
}

/// The reader stage's script for one pair of meta-worlds: one step per target token, taking a source token (with
/// what to patch) or inserting one. What `reader_res` measures, as steps, for the residual rules (`rules`).
pub fn reader_script(src: &[MTok], tgt: &[MTok]) -> R3 {
    let (sm, tm) = (tnodes(src), tnodes(tgt));
    let concepts: Vec<String> = tm.iter().map(|x| x.lemma.clone()).collect();
    let mut scratch = Residual::default();
    frames::encode_with(&tm, &sm, &concepts, &prealign(&tm, &sm), &mut scratch)
}

/// The reader stage over a whole block: the verses' meta-worlds read as one (a boundary token «§» after each
/// verse, heads shifted), one script from the source's to the target's and the tokens out of place. Content may move
/// between verses. Returns the residual and whether the decoder gives the target's block meta-world back.
pub fn reader_block(src: &[&[MTok]], tgt: &[&[MTok]]) -> (Residual, bool) {
    let join = |vs: &[&[MTok]]| -> Vec<MTok> {
        let mut out: Vec<MTok> = Vec::new();
        for v in vs {
            let off = out.len();
            out.extend(v.iter().map(|x| MTok { head: if x.head == 0 { 0 } else { x.head + off }, ..x.clone() }));
            out.push(MTok { concept: "§".into(), tag: en::gram::Tag::Stop, head: 0, rel: en::gram::Rel::Root, span: (0, 0) });
        }
        out
    };
    let (sm, tm) = (tnodes(&join(src)), tnodes(&join(tgt)));
    let concepts: Vec<String> = tm.iter().map(|x| x.lemma.clone()).collect();
    let mut scratch = Residual::default();
    let script = frames::encode_with(&tm, &sm, &concepts, &prealign(&tm, &sm), &mut scratch);
    let mut res = Residual::default();
    take(scratch, &mut res, Layer::World);
    let base = base_order(&script);
    let mv = moved(&base);
    let each = if mv.is_empty() { 0.0 } else { choose_bits(tm.len(), mv.len()) / mv.len() as f64 + pos_bits(tm.len()) };
    for &x in &mv {
        let c = concepts[x - 1].clone();
        res.add(Layer::World, Class::Order, each, || format!("«{c}» out of place"));
    }
    let ok = frames::apply(&sm, &script) == tm && order_ok(&base, &mv);
    (res, ok)
}

/// The decoder's hints for the target tree from the profile's words and the script: the base order, and each taken
/// word's form as the profile wrote it (unless the script patches its lemma or tag).
fn hints(script: &R3, pred: &[String]) -> Hints {
    let mut h = Hints { order: Some(base_order(script)), forms: BTreeMap::new(), cap_after_stop: true };
    for (i, s) in script.iter().enumerate() {
        if let Step::Take { from, lemma: None, tag: None, .. } = s {
            if let Some(f) = pred.get(*from) {
                h.forms.insert(i + 1, f.clone());
            }
        }
    }
    h
}

/// The text layer of a verse: every gap that is not the canonical one ("" at the start, " " after a space bit, ""
/// otherwise), and the tail.
#[derive(Clone, Debug, Default)]
struct RText {
    gaps: BTreeMap<usize, String>,
    tail: String,
}

fn text_encode(v: &PVerse, res: &mut Residual) -> RText {
    let space = v.space();
    let n = v.forms.len();
    let mut r = RText { tail: v.tail.clone(), ..Default::default() };
    for k in 0..n {
        let canon = if k > 0 && space[k - 1] { " " } else { "" };
        if v.gaps[k] != canon {
            let g = v.gaps[k].clone();
            let c = if g.contains('\n') { Class::Layout } else { Class::Spacing };
            res.add(Layer::Tokens, c, lit_bits(&g) + pos_bits(n), || format!("gap {g:?} before «{}»", v.forms[k]));
            r.gaps.insert(k, g);
        }
    }
    if !v.tail.is_empty() {
        let t = v.tail.clone();
        res.add(Layer::Tokens, Class::Spacing, lit_bits(&t), || format!("tail {t:?}"));
    }
    r
}

fn text_decode(forms: &[String], space: &[bool], r: &RText) -> String {
    let mut s = String::new();
    for (k, f) in forms.iter().enumerate() {
        let canon = if k > 0 && space[k - 1] { " " } else { "" };
        s.push_str(r.gaps.get(&k).map_or(canon, String::as_str));
        s.push_str(f);
    }
    s.push_str(&r.tail);
    s
}

fn rule_spaces(forms: &[String]) -> Vec<bool> {
    (0..forms.len()).map(|k| forms.get(k + 1).is_some_and(|n| tree::rule_space(&forms[k], n))).collect()
}

pub fn join(forms: &[String]) -> String {
    // the spacing rules, and the written text's own: no space inside a pair of quotes, none before a possessive
    let mut sp = rule_spaces(forms);
    let mut open = false;
    for k in 0..forms.len() {
        let f = forms[k].as_str();
        let prev_word = k > 0 && forms[k - 1].chars().last().is_some_and(char::is_alphanumeric);
        if prev_word && matches!(f, "’s" | "'s" | "’" | "'") && !open {
            sp[k - 1] = false;
            continue;
        }
        if matches!(f, "“" | "”" | "‘" | "\"" | "'") || (f == "’" && open) {
            if open {
                if k > 0 {
                    sp[k - 1] = false;
                }
            } else if k + 1 < forms.len() {
                sp[k] = false;
            }
            open = !open;
        }
    }
    roundtrip::vol::join(forms, &sp)
}

fn tks(ws: &[Word], space: &[bool]) -> Vec<Tk> {
    ws.iter().enumerate().map(|(k, w)| Tk { form: w.form.clone(), lemma: w.lemma.clone(), tag: w.tag, rel: w.rel, space: space[k] }).collect()
}

fn mtoks(t: &[TNode]) -> Vec<MTok> {
    t.iter().map(|x| MTok { concept: x.lemma.clone(), tag: x.tag, head: x.head, rel: x.rel, span: (0, 0) }).collect()
}

/// Move the cells of a scratch residual into a stage, under one layer name.
fn take(scratch: Residual, into: &mut Residual, l: Layer) {
    for ((_, c), cell) in scratch.cells {
        let mut r = Residual::default();
        r.cells.insert((l, c), cell);
        into.merge(&r);
    }
}

pub struct Run<'a> {
    pub a: &'a Annotator,
    /// the encoder into the target: the standard with the target author's delta
    pub enc: &'a Encoder<'a>,
    /// the reader of the target author (the shared English reader)
    pub read: &'a (dyn Fn(&[Word]) -> Vec<MTok> + Sync),
    /// a negative control of the gate: corrupt the stack's residual (one space flip) before decoding
    pub corrupt: bool,
    /// keep the reasons of the profile's choices
    pub why: bool,
    /// also measure the direct code (the target from the normal-mode output)
    pub direct: bool,
}

impl Run<'_> {
    pub fn verse(&self, src: &Src, tgt: &PVerse) -> Out {
        let a = self.a;
        let dec = Dec { a };
        let mut o = Out::default();
        let ttree = tgt.tree();
        let space = tgt.space();
        // normal mode
        let (out_forms, why) = match src {
            Src::Meta(m) => self.enc.encode(a, m),
            Src::Words(w) => (w.clone(), Vec::new()),
        };
        if self.why {
            o.why = why;
        }
        o.text = join(&out_forms);
        o.exact = o.text == tgt.text;
        o.chrf = en::text::chrf(&o.text, &tgt.text);
        let out_words = parse_tokens(a, &out_forms);
        diff::compare(&tks(&tgt.words, &space), &tks(&out_words, &rule_spaces(&out_forms)), &tgt.text, &o.text, &mut o.div);
        // exact mode, up
        let tmeta = (self.read)(&tgt.meta_words());
        let tm = tnodes(&tmeta);
        let mut s1: Option<(Vec<TNode>, R3, Vec<usize>, Vec<usize>)> = None;
        let pred: Vec<String> = match src {
            Src::Meta(m) => {
                let sm = tnodes(m);
                let concepts: Vec<String> = tm.iter().map(|x| x.lemma.clone()).collect();
                let mut scratch = Residual::default();
                let script = frames::encode_with(&tm, &sm, &concepts, &prealign(&tm, &sm), &mut scratch);
                take(scratch, &mut o.res[0], Layer::World);
                let base = base_order(&script);
                let mv = moved(&base);
                // which words moved (n choose k) and where each goes
                let each = if mv.is_empty() { 0.0 } else { choose_bits(tm.len(), mv.len()) / mv.len() as f64 + pos_bits(tm.len()) };
                for &x in &mv {
                    let c = concepts[x - 1].clone();
                    o.res[0].add(Layer::World, Class::Order, each, || format!("«{c}» out of place"));
                }
                s1 = Some((sm, script, base, mv));
                self.enc.encode(a, &tmeta).0
            }
            Src::Words(w) => w.clone(),
        };
        let ptree = tree::of_words(&parse_tokens(a, &pred));
        let mut scratch = Residual::default();
        let s2 = frames::encode_with(&ttree, &ptree, &tgt.forms, &prealign(&ttree, &ptree), &mut scratch);
        take(scratch, &mut o.res[1], Layer::Frames);
        let h = hints(&s2, &pred);
        let mut r2 = dec.encode_h(&ttree, &tgt.forms, &space, &h, &mut o.res[2]);
        let rt = text_encode(tgt, &mut o.res[2]);
        if self.corrupt {
            if !r2.space.remove(&0) {
                r2.space.insert(0);
            }
        }
        // exact mode, down: the gate
        let fail = |what: &str| Some(format!("{what} differs: «{}»", tgt.text));
        let pred2: Vec<String> = match (&s1, src) {
            (Some((sm, script, base, mv)), Src::Meta(_)) => {
                let tm2 = frames::apply(sm, script);
                if tm2 != tm {
                    o.fail = fail("meta-world");
                }
                if o.fail.is_none() && !order_ok(base, mv) {
                    o.fail = fail("meta order");
                }
                self.enc.encode(a, &mtoks(&tm2)).0
            }
            (_, Src::Words(w)) => w.clone(),
            _ => unreachable!(),
        };
        let ptree2 = tree::of_words(&parse_tokens(a, &pred2));
        let t2 = frames::apply(&ptree2, &s2);
        if o.fail.is_none() && t2 != ttree {
            o.fail = fail("tree");
        }
        let h2 = hints(&s2, &pred2);
        let (f2, sp2) = dec.decode_exact_h(&t2, &r2, &h2);
        if o.fail.is_none() && (f2 != tgt.forms || sp2 != space) {
            o.fail = fail("tokens");
        }
        let text2 = text_decode(&f2, &sp2, &rt);
        if o.fail.is_none() && text2 != tgt.text {
            o.fail = fail("text");
        }
        o.gate_ok = o.fail.is_none();
        if let (Src::Meta(_), true) = (src, self.direct) {
            let d = self.verse(&Src::Words(out_forms), tgt);
            let mut all = Residual::default();
            for r in &d.res {
                all.merge(r);
            }
            o.direct = Some(all);
            o.direct_ok = d.gate_ok;
        }
        o
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_residual() {
        let base = vec![1, 4, 2, 3, 5];
        let mv = moved(&base);
        assert_eq!(mv, vec![4]);
        assert!(order_ok(&base, &mv));
        // negative control: without the moved word the order does not come back
        assert!(!order_ok(&base, &[]));
        assert!(moved(&[1, 2, 3]).is_empty());
    }

    #[test]
    fn a_block_reads_as_one() {
        use en::gram::{Rel, Tag};
        let v = |c: &[&str]| -> Vec<MTok> { c.iter().enumerate().map(|(i, x)| MTok { concept: x.to_string(), tag: Tag::NN, head: if i == 0 { 0 } else { 1 }, rel: if i == 0 { Rel::Root } else { Rel::Dep }, span: (0, 0) }).collect() };
        let (a, b) = (v(&["god", "say", "light"]), v(&["light", "be", "good"]));
        let (r, ok) = reader_block(&[&a, &b], &[&a, &b]);
        assert!(ok);
        assert_eq!(r.cells.values().map(|c| c.bits).sum::<f64>(), 0.0);
        // negative control: the verses in the other order cost bits, and still come back
        let (r2, ok2) = reader_block(&[&b, &a], &[&a, &b]);
        assert!(ok2);
        assert!(r2.cells.values().map(|c| c.bits).sum::<f64>() > 0.0);
    }

    #[test]
    fn text_layer_back() {
        let v = PVerse { text: "In  the end.".into(), forms: vec!["In".into(), "the".into(), "end".into(), ".".into()], gaps: vec!["".into(), "  ".into(), " ".into(), "".into()], ..Default::default() };
        let mut res = Residual::default();
        let r = text_encode(&v, &mut res);
        assert_eq!(text_decode(&v.forms, &v.space(), &r), v.text);
        assert_eq!(r.gaps.len(), 1);
        // negative control: without the residual the double space is lost
        assert_ne!(text_decode(&v.forms, &v.space(), &RText::default()), v.text);
    }
}
