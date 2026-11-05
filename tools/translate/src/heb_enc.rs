//! English → Hebrew: the Hebrew encoder out of the meta-world, and the measure of a Hebrew target verse.
//!
//! **The Hebrew encoder** is the Hebrew reader turned round, and learned the same way: on the training verse pairs
//! the reader aligned (each meta token owned by a Hebrew word, its chunk), the first meta token of a word's chunk
//! *emits* that word (lemma, UPOS, XPOS, FEATS, relation), the other tokens of the chunk emit nothing. To encode, each
//! meta token emits what it emitted most often in the most specific context seen twice (`profile::ctx_keys`:
//! concept, tag and neighbours; concept, tag and relation; concept and tag; concept). A word hangs on the word its
//! meta head's chain emitted first. The words come out in the meta-world's order.
//!
//! **A Hebrew target verse** (`measure`), with the split code of the English targets:
//! - *reader*: the script from the meta-world read from the English source to the one the Hebrew reader reads from
//!   the target verse (`measure::reader_block`, verse by verse);
//! - *encoder*: the Hebrew tree script from what the encoder writes for the target's own meta-world to the target's
//!   MACULA words (`roundtrip::mixed::heb::frames_encode`), and the words out of place (which, and where);
//! - *stack*: the target words to their forms and written words (the lookup generator of the training books,
//!   `mixed::heb::tree_encode`), and the written words to the verse text (`mixed::heb::tokens_up`).
//! The decoder runs the stages down; the gate compares the meta-world, the words, the written words and the text.

use std::collections::BTreeMap;

use roundtrip::class::{Class, Layer, pos_bits};
use roundtrip::mixed::heb::{self, HTree, HW, Lookup, Step};
use roundtrip::tally::Residual;

use crate::hebrew::{HTok, HVerse};
use crate::meta::MTok;
use crate::profile::{LEVELS, ctx_keys};

const MIN: [u32; LEVELS] = [2, 2, 2, 1];

/// A Hebrew word's signature: what the encoder emits for it.
fn sig(t: &HTok) -> String {
    format!("{}\t{}\t{}\t{}\t{}", t.lemma, t.upos, t.xpos, t.feats, t.rel)
}

#[derive(Clone, Debug, Default)]
pub struct HebEncoder {
    tables: Vec<BTreeMap<String, BTreeMap<String, u32>>>,
    pub tokens: u64,
}

impl HebEncoder {
    /// Learn on (Hebrew verse, meta-world, owner of each meta token): `shuffle` pairs each verse with another verse's
    /// meta-world (a negative control).
    pub fn train(pairs: &[(&HVerse, &[MTok], &[Option<usize>])], shuffle: bool) -> HebEncoder {
        let mut e = HebEncoder { tables: vec![BTreeMap::new(); LEVELS], tokens: 0 };
        let n = pairs.len();
        for i in 0..n {
            let (h, _, _) = pairs[i];
            let (_, m, owner) = pairs[if shuffle { (i + 7919) % n.max(1) } else { i }];
            let mut seen = vec![false; h.toks.len()];
            for j in 0..m.len() {
                let emit = match owner.get(j).copied().flatten() {
                    Some(k) if k < h.toks.len() && !seen[k] => {
                        seen[k] = true;
                        sig(&h.toks[k])
                    }
                    _ => String::new(),
                };
                for (l, ck) in ctx_keys(m, j).into_iter().enumerate() {
                    *e.tables[l].entry(ck).or_default().entry(emit.clone()).or_default() += 1;
                }
                e.tokens += 1;
            }
        }
        e
    }

    /// What token `j` emits: a word's signature or nothing.
    fn emit(&self, m: &[MTok], j: usize) -> String {
        for (l, ck) in ctx_keys(m, j).into_iter().enumerate() {
            if let Some(tb) = self.tables.get(l).and_then(|t| t.get(&ck)) {
                if tb.values().sum::<u32>() >= MIN[l] {
                    return tb.iter().fold((String::new(), 0u32), |acc, (c, &v)| if v > acc.1 { (c.clone(), v) } else { acc }).0;
                }
            }
        }
        String::new()
    }

    /// The Hebrew words of a meta-world.
    pub fn encode(&self, m: &[MTok]) -> Vec<HW> {
        let emits: Vec<String> = (0..m.len()).map(|j| self.emit(m, j)).collect();
        let mut idx = vec![None; m.len()];
        let mut out: Vec<HW> = Vec::new();
        for (j, s) in emits.iter().enumerate() {
            if s.is_empty() {
                continue;
            }
            let f: Vec<&str> = s.split('\t').collect();
            idx[j] = Some(out.len());
            out.push(HW { lemma: f[0].into(), upos: f[1].into(), xpos: f[2].into(), feats: f[3].into(), head: 0, rel: f.get(4).unwrap_or(&"dep").to_string() });
        }
        // a word hangs on the first word its meta head's chain emitted
        for j in 0..m.len() {
            let Some(w) = idx[j] else { continue };
            let (mut h, mut guard) = (m[j].head, 0);
            let mut head = 0;
            while h > 0 && guard <= m.len() {
                if let Some(x) = idx[h - 1] {
                    if x != w {
                        head = x + 1;
                    }
                    break;
                }
                h = m[h - 1].head;
                guard += 1;
            }
            out[w].head = head;
        }
        out
    }
}

/// The target's words as the Hebrew stack has them.
pub fn tree(v: &HVerse) -> HTree {
    HTree {
        words: v.toks.iter().map(|t| HW { lemma: t.lemma.clone(), upos: t.upos.clone(), xpos: t.xpos.clone(), feats: t.feats.clone(), head: t.head, rel: t.rel.clone() }).collect(),
        forms: v.toks.iter().map(|t| t.form.clone()).collect(),
        groups: v.groups.clone(),
    }
}

/// The words out of place in a script: those off the longest increasing run of the taken words' sources.
fn moved(script: &[Step]) -> Vec<usize> {
    let from: Vec<(usize, usize)> = script.iter().enumerate().filter_map(|(i, s)| if let Step::Take { from, .. } = s { Some((i, *from)) } else { None }).collect();
    let n = from.len();
    let mut tails: Vec<usize> = Vec::new();
    let mut tail_idx: Vec<usize> = Vec::new();
    let mut prev = vec![usize::MAX; n];
    for (k, &(_, x)) in from.iter().enumerate() {
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
    (0..n).filter(|&k| !keep[k]).map(|k| from[k].0).collect()
}

fn choose_bits(n: usize, k: usize) -> f64 {
    (0..k).map(|i| ((n - i) as f64).log2() - ((i + 1) as f64).log2()).sum()
}

/// Move a scratch residual's cells into a stage under one layer.
fn take(scratch: Residual, into: &mut Residual, l: Layer) {
    for ((_, c), cell) in scratch.cells {
        let mut r = Residual::default();
        r.cells.insert((l, c), cell);
        into.merge(&r);
    }
}

/// The outcome of one Hebrew target verse.
#[derive(Clone, Debug, Default)]
pub struct HOut {
    pub res: [Residual; 3],
    pub gate_ok: bool,
    pub fail: Option<String>,
    /// normal mode: the encoder's words from the source's meta-world, written by the lookup
    pub text: String,
    pub chrf: f64,
    pub exact: bool,
}

/// Translate into a Hebrew verse and measure it: `src` the meta-world read from the English source, `tgt_meta` the
/// one the Hebrew reader reads from the target verse.
pub fn measure(enc: &HebEncoder, lk: &Lookup, src: &[MTok], tgt_meta: &[MTok], v: &HVerse) -> HOut {
    let mut o = HOut::default();
    // normal mode
    let out_words = enc.encode(src);
    let (_, _, toks) = heb::tree_decode(lk, &out_words, None);
    o.text = heb::join_tokens(&[toks]);
    o.exact = o.text == v.text;
    o.chrf = en::text::chrf(&o.text, &v.text);
    // reader
    let (r0, ok0) = crate::measure::reader_block(&[src], &[tgt_meta]);
    o.res[0] = r0;
    // encoder: what it writes for the target's own meta-world, to the target's words
    let t = tree(v);
    let realized = enc.encode(tgt_meta);
    let mut scratch = Residual::default();
    let script = heb::frames_encode(&t.words, &t.forms, &realized, &mut scratch);
    take(scratch, &mut o.res[1], Layer::Frames);
    let mv = moved(&script);
    let n = t.words.len();
    if !mv.is_empty() {
        let each = choose_bits(n, mv.len()) / mv.len() as f64 + pos_bits(n);
        for &i in &mv {
            let l = t.words[i].lemma.clone();
            o.res[1].add(Layer::Frames, Class::Order, each, || format!("«{l}» out of place"));
        }
    }
    // stack: words → forms → written words → text
    let mut scratch = Residual::default();
    let (tk, r1) = heb::tokens_up(&v.text, &mut scratch);
    let flat: Vec<String> = tk.sents.iter().flatten().cloned().collect();
    // a verse whose written words are not its tree's (a paragraph mark glued to a word, a word written twice in the
    // text's line): the stack stores the text
    if flat.len() != t.groups.len() {
        let text = v.text.clone();
        o.res[2].add(Layer::Tree, Class::Spelling, roundtrip::class::lit_bits(&text), || format!("text stored: {} written words, {} in the tree", flat.len(), t.groups.len()));
        let words2 = heb::frames_apply(&enc.encode(tgt_meta), &script);
        o.fail = if !ok0 { Some("meta-world differs".into()) } else if words2 != t.words { Some("words differ".into()) } else { None };
        o.gate_ok = o.fail.is_none();
        return o;
    }
    let r2 = heb::tree_encode(lk, &t, &flat, &mut scratch);
    take(scratch, &mut o.res[2], Layer::Tree);
    // down: the gate
    let fail = |what: &str| Some(format!("{what} differs: «{}»", v.text));
    if !ok0 {
        o.fail = fail("meta-world");
    }
    let words2 = heb::frames_apply(&enc.encode(tgt_meta), &script);
    if o.fail.is_none() && words2 != t.words {
        o.fail = fail("words");
    }
    let (_, _, toks2) = heb::tree_decode(lk, &words2, Some(&r2));
    if o.fail.is_none() && toks2 != flat {
        o.fail = fail("written words");
    }
    // the written words back into the sentences the token layer cut
    let mut sents = Vec::new();
    let mut k = 0;
    for s in &tk.sents {
        sents.push(toks2[k.min(toks2.len())..(k + s.len()).min(toks2.len())].to_vec());
        k += s.len();
    }
    let text2 = heb::tokens_down(&heb::Toks { sents }, &r1);
    if o.fail.is_none() && text2 != v.text {
        o.fail = fail("text");
    }
    o.gate_ok = o.fail.is_none();
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hebrew::HTok;
    use en::gram::{Rel, Tag};

    fn tok(form: &str, upos: &str, head: usize, rel: &str) -> HTok {
        HTok { form: form.into(), lemma: form.into(), upos: upos.into(), head, rel: rel.into(), ..Default::default() }
    }

    #[test]
    fn hebrew_from_the_meta_world_and_back() {
        let v = HVerse { toks: vec![tok("בָּרָא", "VERB", 0, "root"), tok("אֱלֹהִים", "NOUN", 1, "nsubj")], text: "בָּרָא אֱלֹהִים".into(), groups: vec![1, 1], neg: false };
        let m = vec![MTok { concept: "god".into(), tag: Tag::NNP, head: 2, rel: Rel::Nsubj, span: (0, 1) }, MTok { concept: "create".into(), tag: Tag::VBD, head: 0, rel: Rel::Root, span: (1, 2) }];
        let owner = vec![Some(1), Some(0)];
        let pairs = vec![(&v, m.as_slice(), owner.as_slice()); 2];
        let enc = HebEncoder::train(&pairs, false);
        let words = enc.encode(&m);
        assert_eq!(words.iter().map(|w| w.lemma.as_str()).collect::<Vec<_>>(), ["אֱלֹהִים", "בָּרָא"]);
        assert_eq!((words[0].head, words[1].head), (2, 0));
        let lk = Lookup::from_conllu("1\tבָּרָא\tבָּרָא\tVERB\t_\t_\t0\troot\t_\t_\n2\tאֱלֹהִים\tאֱלֹהִים\tNOUN\t_\t_\t1\tnsubj\t_\t_\n", hbo::text::Level::Pointed);
        let o = measure(&enc, &lk, &m, &m, &v);
        assert!(o.gate_ok, "{:?}", o.fail);
        let total = |o: &HOut| o.res.iter().map(|r| r.cells.values().map(|c| c.bits).sum::<f64>()).sum::<f64>();
        // the encoder says the words in English order: one word out of place, nothing else
        assert!(o.res[1].cells.contains_key(&(Layer::Frames, Class::Order)));
        // negative control: an encoder that learned nothing writes nothing, and every word costs
        let none = measure(&HebEncoder::default(), &lk, &m, &m, &v);
        assert!(none.gate_ok, "{:?}", none.fail);
        assert!(total(&none) > total(&o), "{} vs {}", total(&none), total(&o));
    }
}
