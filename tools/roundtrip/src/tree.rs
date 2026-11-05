//! The second layer, tokens ↔ UD tree.
//!
//! The tree is what a decoder may read: lemma, PTB tag, head and relation of every word — an unordered tree. The
//! decoder is `en`'s generator: `lin.rs` gives the word order, `morph.rs` the form from lemma and tag, positional
//! rules give a/an and the capital of the first word, and the spacing rules of `en::text::detok` give the spaces.
//! The residual is what the decoder gets wrong: the local order of a head's dependents (or the whole order of a
//! non-projective tree), the case of a word beyond the rules, the form itself where lemma and tag do not give it
//! back, and the space bits the rules miss.

use std::collections::{BTreeMap, BTreeSet};

use en::annotate::{Annotator, Word};
use en::gram::{Rel, Tag};

use crate::class::{Class, Layer, lit_bits, perm_bits, pos_bits};
use crate::tally::Residual;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TNode {
    pub lemma: String,
    pub tag: Tag,
    /// 1-based, 0 — root
    pub head: usize,
    pub rel: Rel,
}

pub fn of_words(w: &[Word]) -> Vec<TNode> {
    w.iter().map(|x| TNode { lemma: x.lemma.clone(), tag: x.tag, head: x.head, rel: x.rel }).collect()
}

/// Case of a form beyond the lemma's own spelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaseX {
    AsIs,
    Title,
    Upper,
    /// as is, and not capitalized at the start of the sentence either
    NoCap,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spell {
    Case(CaseX),
    Raw(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Order {
    /// the local order (dependents and the head itself) of the heads the generator gets wrong
    Heads(BTreeMap<usize, Vec<usize>>),
    /// the whole order (a non-projective tree, which the generator cannot give)
    Full(Vec<usize>),
}

/// The residual of one sentence at the tree layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct R2 {
    pub order: Order,
    pub spell: BTreeMap<usize, Spell>,
    /// output positions (between word k and k+1) where the space rule is flipped
    pub space: BTreeSet<usize>,
}

const NO_SPACE_BEFORE: [&str; 18] = [",", ".", "!", "?", ";", ":", ")", "]", "}", "%", "...", "n't", "'s", "'re", "'ve", "'ll", "'d", "'m"];
const NO_SPACE_AFTER: [&str; 5] = ["(", "[", "{", "$", "#"];

/// The spacing rule of `en::text::detok` for words that were not adjacent.
pub fn rule_space(prev: &str, f: &str) -> bool {
    !(NO_SPACE_BEFORE.contains(&f) || NO_SPACE_AFTER.contains(&prev))
}

fn title(s: &str) -> String {
    en::text::title(s)
}

/// The a/an rule (as `en::model`): «an» before a vowel letter.
fn article(next: Option<&str>) -> &'static str {
    match next.and_then(|w| w.chars().next()) {
        Some(c) if matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}

fn is_article(n: &TNode) -> bool {
    n.tag == Tag::DT && n.lemma.eq_ignore_ascii_case("a")
}

/// The case the rules give: a proper noun and «I» (the form: not «me» of the lemma I) are capitalized.
pub fn rule_case(n: &TNode, base: &str) -> CaseX {
    if n.tag.is_proper() || (n.tag == Tag::PRP && base.eq_ignore_ascii_case("i")) { CaseX::Title } else { CaseX::AsIs }
}

/// A step of the decoders switched off (ROUNDTRIP_OFF=name,name — for measuring what each step brings): `feats`
/// (inflection from the tree's FEATS), `refgen` (pronouns chosen by the world's decoder), `world-bare` (the world keeps no
/// bare frame).
pub fn off(name: &str) -> bool {
    std::env::var("ROUNDTRIP_OFF").is_ok_and(|v| v.split(',').any(|x| x == name))
}

fn apply_case(f: &str, c: CaseX) -> String {
    match c {
        CaseX::AsIs | CaseX::NoCap => f.to_string(),
        CaseX::Title => title(f),
        CaseX::Upper => f.to_uppercase(),
    }
}

/// Children lists (index 0 — the virtual root), heads clamped into range.
pub fn kids(t: &[TNode]) -> Vec<Vec<usize>> {
    let n = t.len();
    let mut k = vec![Vec::new(); n + 1];
    for (i, x) in t.iter().enumerate() {
        k[if x.head <= n { x.head } else { 0 }].push(i + 1);
    }
    k
}

/// Whether every subtree is contiguous in the text order.
pub fn projective(t: &[TNode]) -> bool {
    let n = t.len();
    let k = kids(t);
    let mut lo = vec![usize::MAX; n + 1];
    let mut hi = vec![0usize; n + 1];
    let mut size = vec![0usize; n + 1];
    // post-order without recursion
    let mut stack = vec![(0usize, false)];
    while let Some((h, done)) = stack.pop() {
        if done {
            let (mut l, mut r, mut s) = if h > 0 { (h, h, 1) } else { (usize::MAX, 0, 0) };
            for &c in &k[h] {
                l = l.min(lo[c]);
                r = r.max(hi[c]);
                s += size[c];
            }
            lo[h] = l;
            hi[h] = r;
            size[h] = s;
        } else {
            stack.push((h, true));
            for &c in &k[h] {
                stack.push((c, false));
            }
        }
    }
    (1..=n).all(|i| hi[i] + 1 - lo[i] == size[i]) && size[0] == n
}

/// The local order of every head (its dependents and itself) in a given word order.
pub fn locals(t: &[TNode], order: &[usize]) -> Vec<Vec<usize>> {
    let n = t.len();
    let mut pos = vec![0usize; n + 1];
    for (k, &i) in order.iter().enumerate() {
        pos[i] = k;
    }
    kids(t)
        .into_iter()
        .enumerate()
        .map(|(h, mut ks)| {
            if h > 0 {
                ks.push(h);
            }
            ks.sort_by_key(|&x| pos[x]);
            ks
        })
        .collect()
}

/// Words in order from the local orders (projective).
pub fn emit(loc: &[Vec<usize>]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    while let Some((h, k)) = stack.pop() {
        if k >= loc[h].len() {
            continue;
        }
        stack.push((h, k + 1));
        let x = loc[h][k];
        if x == h {
            out.push(h);
        } else {
            stack.push((x, 0));
        }
    }
    out
}

/// Typographic marks the dictionary reads as their typewriter forms.
pub fn typewriter(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\u{2019}' | '\u{2018}' => '\'',
            '\u{201c}' | '\u{201d}' => '"',
            c => c,
        })
        .collect()
}

/// Why a form does not come back from its lemma and tag: typography (’ for '), another form of the same lemma
/// (agreement, a variant: were/was, learnt/learned), another known word (a lemma or tag that does not fit the form), or
/// no known word at all (the source's own spelling).
fn raw_class(form: &str, made: &str, lemma: &str) -> Class {
    let lower = form.to_lowercase();
    let tw = typewriter(&lower);
    if tw != lower && (tw == made.to_lowercase() || tw == typewriter(made).to_lowercase()) {
        return Class::Typography;
    }
    match en::dict::form(&tw) {
        Some(f) if en::dict::analyses(f).any(|r| en::dict::text(r.lemma()).eq_ignore_ascii_case(lemma)) => Class::Form,
        Some(_) => Class::Choice,
        None => Class::Spelling,
    }
}

/// What a decoder may be told besides the tree: the order to start from (1-based word numbers, a permutation), forms
/// to write for some words (by word number) instead of the rules' forms, and whether a word after . ! ? takes the
/// sentence capital (a span of several sentences). The default is the plain decoder.
#[derive(Clone, Debug, Default)]
pub struct Hints {
    pub order: Option<Vec<usize>>,
    pub forms: BTreeMap<usize, String>,
    pub cap_after_stop: bool,
}

pub struct Dec<'a> {
    pub a: &'a Annotator,
}

impl<'a> Dec<'a> {
    /// `lin.rs`'s order; None if it is not a permutation (the depth guard).
    pub fn lin_order(&self, t: &[TNode]) -> Option<Vec<usize>> {
        let nodes: Vec<en::lin::Node> = t.iter().map(|x| en::lin::Node { tag: x.tag, rel: x.rel, head: x.head, lex: en::dict::lex(&x.lemma) }).collect();
        let qmark = t.iter().any(|x| x.rel == Rel::Punct && x.lemma.contains('?'));
        let q = en::lin::mood(&nodes, qmark);
        let o = self.a.model.lin.order(&nodes, q);
        let mut seen = vec![false; t.len() + 1];
        let ok = o.len() == t.len() && o.iter().all(|&i| i >= 1 && i <= t.len() && !std::mem::replace(&mut seen[i], true));
        ok.then_some(o)
    }

    /// The forms `morph` gives the words of a tree: from lemma, tag and the FEATS agreement reads from the tree
    /// (`en::morph::tree_feats`: pronoun case, the person and number of «be», these/those), or from lemma and tag
    /// alone with `ROUNDTRIP_OFF=feats`.
    pub fn base_forms(&self, t: &[TNode]) -> Vec<String> {
        let m = &self.a.model.morph;
        if off("feats") {
            return t.iter().map(|n| m.inflect(&n.lemma, n.tag)).collect();
        }
        let lemmas: Vec<&str> = t.iter().map(|n| n.lemma.as_str()).collect();
        let tags: Vec<Tag> = t.iter().map(|n| n.tag).collect();
        let heads: Vec<usize> = t.iter().map(|n| n.head).collect();
        let rels: Vec<Rel> = t.iter().map(|n| n.rel).collect();
        let feats = en::morph::tree_feats(&lemmas, &tags, &heads, &rels);
        t.iter().zip(feats).map(|(n, f)| m.inflect_feats(&n.lemma, n.tag, f)).collect()
    }

    /// The form of a word: inflection (`base`, or a/an by the next word), case, and the capital at the sentence start.
    pub fn form_at(&self, n: &TNode, base: &str, c: CaseX, first: bool, next: Option<&str>) -> String {
        let f = if is_article(n) { article(next).to_string() } else { base.to_string() };
        let f = apply_case(&f, c);
        if first && c != CaseX::NoCap { title(&f) } else { f }
    }

    /// Forms in output order: inflection, case (the residual's or the rules'), a/an by the next output word, the
    /// capital of the first word.
    pub fn render(&self, t: &[TNode], order: &[usize], spell: &BTreeMap<usize, Spell>) -> Vec<String> {
        self.render_h(t, order, spell, &Hints::default())
    }

    /// Whether the word at output position k takes the sentence capital: the first word, and with
    /// `Hints::cap_after_stop` a word after . ! ? (and the closing quotes after them).
    fn first_at(forms_before: &[String], k: usize, h: &Hints) -> bool {
        if k == 0 {
            return true;
        }
        if !h.cap_after_stop {
            return false;
        }
        let mut j = k;
        while j > 0 {
            let f = forms_before[j - 1].as_str();
            if matches!(f, "'" | "\"" | "\u{2019}" | "\u{201d}" | ")") {
                j -= 1;
                continue;
            }
            return !f.is_empty() && f.chars().all(|c| matches!(c, '.' | '!' | '?'));
        }
        false
    }

    /// `render` with hints. The capital of a sentence start needs the forms before, which a/an (the next form) does
    /// not: the first pass fixes every form but the capital, the second the capitals. Without hints this is `render`.
    pub fn render_h(&self, t: &[TNode], order: &[usize], spell: &BTreeMap<usize, Spell>, h: &Hints) -> Vec<String> {
        let n = order.len();
        let base = self.base_forms(t);
        let mut plain: Vec<String> = vec![String::new(); n];
        for k in (0..n).rev() {
            let i = order[k];
            let node = &t[i - 1];
            let b = &base[i - 1];
            let next = plain.get(k + 1).map(String::as_str);
            plain[k] = match spell.get(&i) {
                Some(Spell::Raw(s)) => s.clone(),
                Some(Spell::Case(c)) => self.form_at(node, b, *c, false, next),
                None => match h.forms.get(&i) {
                    Some(f) => f.clone(),
                    None => self.form_at(node, b, rule_case(node, b), false, next),
                },
            };
        }
        // the capital where a sentence starts (a rule word or a Case residual; raw and hinted forms are as given)
        let mut forms = plain.clone();
        for k in 0..n {
            let i = order[k];
            let raw = matches!(spell.get(&i), Some(Spell::Raw(_))) || (spell.get(&i).is_none() && h.forms.contains_key(&i));
            if raw || matches!(spell.get(&i), Some(Spell::Case(CaseX::NoCap))) {
                continue;
            }
            if Self::first_at(&forms, k, h) {
                forms[k] = title(&plain[k]);
            }
        }
        forms
    }

    /// Canonical decode, no residual: words in `lin.rs` order (or text order if it fails), forms by the rules,
    /// spaces by the rules.
    pub fn decode(&self, t: &[TNode]) -> (Vec<usize>, Vec<String>, Vec<bool>) {
        let order = self.lin_order(t).unwrap_or_else(|| (1..=t.len()).collect());
        let forms = self.render(t, &order, &BTreeMap::new());
        let space = (0..forms.len()).map(|k| forms.get(k + 1).is_some_and(|n| rule_space(&forms[k], n))).collect();
        (order, forms, space)
    }

    /// Exact decode with the residual.
    pub fn decode_exact(&self, t: &[TNode], r: &R2) -> (Vec<String>, Vec<bool>) {
        self.decode_exact_h(t, r, &Hints::default())
    }

    /// The order a decoder starts from: the hinted one, else `lin.rs`'s.
    fn base_order(&self, t: &[TNode], h: &Hints) -> Option<Vec<usize>> {
        match &h.order {
            Some(o) => Some(o.clone()),
            None => self.lin_order(t),
        }
    }

    /// Exact decode with the residual and hints.
    pub fn decode_exact_h(&self, t: &[TNode], r: &R2, h: &Hints) -> (Vec<String>, Vec<bool>) {
        let order = match &r.order {
            Order::Full(o) => o.clone(),
            Order::Heads(over) => {
                let lin = self.base_order(t, h).unwrap_or_else(|| (1..=t.len()).collect());
                let mut loc = locals(t, &lin);
                for (hd, l) in over {
                    loc[*hd] = l.clone();
                }
                emit(&loc)
            }
        };
        let forms = self.render_h(t, &order, &r.spell, h);
        let space = (0..forms.len()).map(|k| forms.get(k + 1).is_some_and(|n| rule_space(&forms[k], n) != r.space.contains(&k))).collect();
        (forms, space)
    }

    /// The residual that brings the sentence (`forms`, `space`) back from its tree `t` (words in text order).
    pub fn encode(&self, t: &[TNode], forms: &[String], space: &[bool], res: &mut Residual) -> R2 {
        self.encode_h(t, forms, space, &Hints::default(), res)
    }

    /// `encode` with hints: the order to start from and forms to expect (`Hints`).
    pub fn encode_h(&self, t: &[TNode], forms: &[String], space: &[bool], h: &Hints, res: &mut Residual) -> R2 {
        let n = t.len();
        let ident: Vec<usize> = (1..=n).collect();
        // order
        let lin = self.base_order(t, h);
        let order = match (&lin, projective(t)) {
            (Some(lin), true) => {
                let lo = locals(t, lin);
                let orig = locals(t, &ident);
                let mut over = BTreeMap::new();
                for h in 0..=n {
                    if lo[h] != orig[h] {
                        let k = orig[h].len();
                        let show = |l: &[usize]| l.iter().map(|&i| if i == 0 { "ROOT".into() } else { forms[i - 1].clone() }).collect::<Vec<_>>().join(" ");
                        let (a, b) = (show(&lo[h]), show(&orig[h]));
                        let head = if h == 0 { "ROOT".to_string() } else { format!("{}/{}", forms[h - 1], t[h - 1].tag) };
                        res.add_k(Layer::Tree, Class::Order, perm_bits(k) + pos_bits(n), if h == 0 { "ROOT".into() } else { format!("{}", t[h - 1].rel.base()) }, || format!("head {head}: generated «{a}», text «{b}»"));
                        over.insert(h, orig[h].clone());
                    }
                }
                Order::Heads(over)
            }
            (l, proj) => {
                let why = if !proj { "non-projective" } else if l.is_none() { "generator failed" } else { "" };
                let s = forms.join(" ");
                res.add(Layer::Tree, Class::Order, perm_bits(n), || format!("{why}: whole order of «{s}»"));
                Order::Full(ident.clone())
            }
        };
        // spelling: the hinted form or the rules' case first, then any case, then the form itself
        let mut spell = BTreeMap::new();
        let base = self.base_forms(t);
        for i in 1..=n {
            let node = &t[i - 1];
            let b = &base[i - 1];
            let rc = rule_case(node, b);
            let next = forms.get(i).map(String::as_str);
            let first = Self::first_at(forms, i - 1, h);
            let expected = match h.forms.get(&i) {
                Some(f) => f.clone(),
                None => self.form_at(node, b, rc, first, next),
            };
            if expected == forms[i - 1] {
                continue;
            }
            let got = [rc, CaseX::AsIs, CaseX::Title, CaseX::Upper, CaseX::NoCap].into_iter().find(|&c| self.form_at(node, b, c, first, next) == forms[i - 1]);
            match got {
                Some(c) if c == rc && !h.forms.contains_key(&i) => {}
                Some(c) => {
                    let f = forms[i - 1].clone();
                    res.add_k(Layer::Tree, Class::Case, 2.0 + pos_bits(n), format!("{c:?}"), || format!("«{f}» ({c:?} beyond the rules)"));
                    spell.insert(i, Spell::Case(c));
                }
                None => {
                    let f = forms[i - 1].clone();
                    let made = h.forms.get(&i).cloned().unwrap_or_else(|| b.clone());
                    let class = raw_class(&f, &made, &node.lemma);
                    res.add_k(Layer::Tree, class, lit_bits(&f) + pos_bits(n), f.to_lowercase(), || format!("«{f}» ← {}/{} gives «{made}»", node.lemma, node.tag));
                    spell.insert(i, Spell::Raw(f));
                }
            }
        }
        // spaces: the rule on the original neighbours
        let mut flips = BTreeSet::new();
        for k in 0..n.saturating_sub(1) {
            if rule_space(&forms[k], &forms[k + 1]) != space[k] {
                let (a, b, s) = (forms[k].clone(), forms[k + 1].clone(), space[k]);
                res.add_k(Layer::Tree, Class::Spacing, 1.0 + pos_bits(n), format!("{a}{}{b}", if s { "␣" } else { "" }), || format!("«{a}»{}«{b}»", if s { " " } else { "" }));
                flips.insert(k);
            }
        }
        R2 { order, spell, space: flips }
    }
}
