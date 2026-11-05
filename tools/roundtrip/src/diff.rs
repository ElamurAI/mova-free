//! Normal mode: how a decoded sentence diverges from the original, word by word, in classes.
//!
//! The words are aligned in steps: the longest common subsequence of forms; then the same form out of order (order);
//! the same form but for case (case); the same lemma (tense for verbs, form otherwise); a pronoun and a name in the
//! same relation (pronoun/name); content words of the same part of speech in the same relation (lexical). What is
//! left is missing or added, each in the class of its word (punctuation, negation, quantity, tense, function word,
//! dropped content). Spacing is counted on the aligned neighbours.

use en::gram::{Rel, Tag};

use crate::class::{self, Class};
use crate::tally::Diverge;

#[derive(Clone, Debug)]
pub struct Tk {
    pub form: String,
    pub lemma: String,
    pub tag: Tag,
    pub rel: Rel,
    pub space: bool,
}

fn lcs_pairs(a: &[Tk], b: &[Tk]) -> Vec<(usize, usize)> {
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i][j] = if a[i].form == b[j].form { dp[i + 1][j + 1] + 1 } else { dp[i + 1][j].max(dp[i][j + 1]) };
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < n && j < m {
        if a[i].form == b[j].form {
            out.push((i, j));
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

/// Compare the decoded tokens with the original; counts go to `d` (one item per word of a class).
pub fn compare(orig: &[Tk], out: &[Tk], orig_s: &str, out_s: &str, d: &mut Diverge) {
    d.sents += 1;
    d.tokens += orig.len() as u64;
    d.chrf += en::text::chrf(out_s, orig_s);
    if orig_s == out_s {
        d.exact += 1;
        return;
    }
    let ctx = || format!("«{}» → «{}»", cut(orig_s), cut(out_s));
    let pairs = lcs_pairs(orig, out);
    let mut oa = vec![None; orig.len()];
    let mut pa = vec![None; out.len()];
    for &(i, j) in &pairs {
        oa[i] = Some(j);
        pa[j] = Some(i);
    }
    for w in pairs.windows(2) {
        let ((i, j), (i2, j2)) = (w[0], w[1]);
        if i2 == i + 1 && j2 == j + 1 && orig[i].space != out[j].space {
            d.add(Class::Spacing, || format!("«{}{}{}» {}", orig[i].form, if out[j].space { " " } else { "" }, orig[i2].form, ctx()));
        }
    }
    let pair = |pred: &dyn Fn(&Tk, &Tk) -> bool, class: &dyn Fn(&Tk, &Tk) -> Class, oa: &mut Vec<Option<usize>>, pa: &mut Vec<Option<usize>>, d: &mut Diverge| {
        for i in 0..orig.len() {
            if oa[i].is_some() {
                continue;
            }
            if let Some(j) = (0..out.len()).find(|&j| pa[j].is_none() && pred(&orig[i], &out[j])) {
                oa[i] = Some(j);
                pa[j] = Some(i);
                let c = class(&orig[i], &out[j]);
                d.add(c, || format!("«{}» ~ «{}» {}", orig[i].form, out[j].form, ctx()));
            }
        }
    };
    pair(&|a, b| a.form == b.form, &|_, _| Class::Order, &mut oa, &mut pa, d);
    pair(&|a, b| crate::tree::typewriter(&a.form) == crate::tree::typewriter(&b.form), &|_, _| Class::Typography, &mut oa, &mut pa, d);
    pair(&|a, b| a.form.to_lowercase() == b.form.to_lowercase(), &|_, _| Class::Case, &mut oa, &mut pa, d);
    pair(
        &|a, b| a.lemma.to_lowercase() == b.lemma.to_lowercase(),
        &|a, b| if (a.tag.is_verb() || a.tag == Tag::MD) && (b.tag.is_verb() || b.tag == Tag::MD) { Class::Tense } else { Class::Form },
        &mut oa,
        &mut pa,
        d,
    );
    let pron = |t: Tag| matches!(t, Tag::PRP | Tag::PRPS);
    pair(&|a, b| ((pron(a.tag) && b.tag.is_noun()) || (pron(b.tag) && a.tag.is_noun())) && a.rel.base() == b.rel.base(), &|_, _| Class::PronName, &mut oa, &mut pa, d);
    pair(
        &|a, b| {
            let c = class::of_word(a.tag, a.rel, &a.lemma);
            c == Class::Dropped && class::of_word(b.tag, b.rel, &b.lemma) == Class::Dropped && class::coarse(a.tag) == class::coarse(b.tag) && a.rel.base() == b.rel.base()
        },
        &|_, _| Class::Lexical,
        &mut oa,
        &mut pa,
        d,
    );
    for i in 0..orig.len() {
        if oa[i].is_none() {
            let c = class::of_word(orig[i].tag, orig[i].rel, &orig[i].lemma);
            d.add(c, || format!("missing «{}» {}", orig[i].form, ctx()));
        }
    }
    for j in 0..out.len() {
        if pa[j].is_none() {
            let c = class::of_word(out[j].tag, out[j].rel, &out[j].lemma);
            d.add(c, || format!("added «{}» {}", out[j].form, ctx()));
        }
    }
}

fn cut(s: &str) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() > 90 { c[..90].iter().collect::<String>() + "…" } else { s.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tk(form: &str, lemma: &str, tag: Tag, rel: Rel) -> Tk {
        Tk { form: form.into(), lemma: lemma.into(), tag, rel, space: true }
    }

    /// Each kind of change lands in its class; an identical sentence is exact and has no items.
    #[test]
    fn classes_of_changes() {
        let o = vec![tk("He", "he", Tag::PRP, Rel::Nsubj), tk("did", "do", Tag::VBD, Rel::Aux), tk("not", "not", Tag::RB, Rel::Advmod), tk("see", "see", Tag::VB, Rel::Root), tk("three", "three", Tag::CD, Rel::Nummod), tk("dogs", "dog", Tag::NNS, Rel::Obj), tk(".", ".", Tag::Stop, Rel::Punct)];
        let mut d = Diverge::default();
        compare(&o, &o, "He did not see three dogs.", "He did not see three dogs.", &mut d);
        assert_eq!((d.exact, d.cells.len()), (1, 0));
        let p = vec![tk("Tom", "Tom", Tag::NNP, Rel::Nsubj), tk("saw", "see", Tag::VBD, Rel::Root), tk("dog", "dog", Tag::NN, Rel::Obj)];
        let mut d = Diverge::default();
        compare(&o, &p, "He did not see three dogs.", "Tom saw dog", &mut d);
        let n = |c: Class| d.cells.get(&c).map_or(0, |x| x.items);
        assert_eq!(n(Class::PronName), 1);
        assert_eq!(n(Class::Neg), 1);
        assert_eq!(n(Class::Quantity), 1);
        assert_eq!(n(Class::Punct), 1);
        assert_eq!(n(Class::Tense), 2, "see/saw and the missing «did»");
        assert_eq!(n(Class::Form), 1, "dogs/dog");
        // negative control: no order, no lexical change was made
        assert_eq!(n(Class::Order) + n(Class::Lexical), 0);
    }
}
