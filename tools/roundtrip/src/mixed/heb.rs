//! The Hebrew route, and the token layer of the Aramaic one.
//!
//! **Text ↔ tokens** (Hebrew and Aramaic): the piece without the quote marks around it (`lead`, `trail`), in NFC,
//! cut by `hbo::text::tokenize` (spaces, maqaf, sof pasuq and punctuation); the decoder writes one space between
//! tokens, none after a maqaf or before a mark. Residual: the quote marks, a piece that is not NFC, every other gap.
//!
//! **Tokens ↔ tree** (Hebrew): the written words cut by `hbo`'s segmenter and parsed by `gparse` (the `pointed` or
//! `unpointed` model by the text). Down: each word's form is the commonest form of its lemma, tag and FEATS in the
//! parser's training trees (`lex.conllu`, a lookup generator); a word joins the next in one written word when it is a
//! prefix (ו ה ב כ ל מ ש as a conjunction, article, preposition or marker), a pronoun that is not a subject joins the
//! word before it; a written word is its words' forms, without a bare ה after a preposition (the swallowed article).
//! Residual: the accents of a form the lookup gives without them, a form it does not give, a word boundary the rule
//! misses, a written word that is not its words' forms. Aramaic has no parser: its tokens are the whole residual
//! (`unparsed`), and nothing above them holds it.
//!
//! **Tree ↔ frames** (Hebrew): one frame per verb — the verb's lemma, its subject, object and indirect object, its
//! obliques with their preposition, negation, adverbs; a nominal clause (a subject on a non-verb root) is a fact; a
//! verb of saying is a speech frame whose quote is its ccomp. `macula prag`'s rules read the sentence (form, act,
//! speech, carried from sentence to sentence) and the first frame keeps the reading. Down: the simplest honest
//! realizer — verb, subject, object, the obliques with their prepositions, a negation, the adjectives of a noun, «and»
//! between the frames — and an edit script to the parsed tree, as for English.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Result, anyhow};
use hbo::conllu::Sent;
use hbo::text::{Level, MAQAF, is_mark, norm};
use prag::frames::{Kind, Link};
use prag::schema::Act;
use unicode_normalization::UnicodeNormalization;

use crate::class::{Class, Layer, lemma_bits, lit_bits, pos_bits};
use crate::frames::{E, F};
use crate::tally::Residual;

// ---------------------------------------------------------------- text ↔ tokens


fn wrap(c: char) -> bool {
    c.is_whitespace() || matches!(c, '«' | '»' | '“' | '”' | '„' | '‘' | '’' | '"' | '\'' | '(' | ')' | '[' | ']')
}

/// A piece at the token layer: sentences of tokens.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Toks {
    pub sents: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
pub struct R1 {
    pub lead: String,
    pub trail: String,
    /// the words (runs between spaces) of the piece that are not NFC, as written, by their number
    pub raw: BTreeMap<usize, String>,
    /// gaps that are not the canonical one, by (sentence, token)
    pub gaps: BTreeMap<(usize, usize), String>,
    pub tail: String,
}

fn canon_gap(sents: &[Vec<String>], s: usize, k: usize) -> &'static str {
    if k == 0 {
        return if s == 0 { "" } else { " " };
    }
    let (p, t) = (&sents[s][k - 1], &sents[s][k]);
    if p.ends_with(MAQAF) || is_mark(t) { "" } else { " " }
}

/// The core of a piece (no quote marks around it), NFC.
pub fn core(p: &str) -> (usize, usize, String) {
    let lead = p.len() - p.trim_start_matches(wrap).len();
    if lead == p.len() {
        return (lead, 0, String::new());
    }
    let trail = p.len() - p.trim_end_matches(wrap).len();
    (lead, trail, p[lead..p.len() - trail].nfc().collect())
}

pub fn tokens_up(p: &str, res: &mut Residual) -> (Toks, R1) {
    let (lead, trail, nfc) = core(p);
    let mut r = R1 { lead: p[..lead].to_string(), trail: p[p.len() - trail..].to_string(), ..Default::default() };
    for x in [&r.lead, &r.trail] {
        if !x.is_empty() {
            let x2 = x.clone();
            res.add(Layer::Tokens, Class::Punct, lit_bits(x), || format!("quote marks {x2:?}"));
        }
    }
    let raw = &p[lead..p.len() - r.trail.len()];
    let chunks: Vec<&str> = raw.split_whitespace().collect();
    for (i, c) in chunks.iter().enumerate() {
        let n: String = c.nfc().collect();
        if n != *c {
            r.raw.insert(i, c.to_string());
            let c2 = c.to_string();
            res.add(Layer::Tokens, Class::Typography, lit_bits(c) + pos_bits(chunks.len()), || format!("not NFC: «{c2}»"));
        }
    }
    let sents = hbo::text::tokenize(&nfc);
    let ntok: usize = sents.iter().map(|s| s.len()).sum();
    let mut pos = 0;
    for (s, toks) in sents.iter().enumerate() {
        for (k, t) in toks.iter().enumerate() {
            let rest = &nfc[pos..];
            let at = rest.find(t.as_str()).unwrap_or(0);
            let g = &rest[..at];
            if g != canon_gap(&sents, s, k) {
                r.gaps.insert((s, k), g.to_string());
                let g2 = g.to_string();
                res.add(Layer::Tokens, if g.contains('\n') { Class::Layout } else { Class::Spacing }, lit_bits(g) + pos_bits(ntok), || format!("gap {g2:?} before «{t}»"));
            }
            pos += at + t.len();
        }
    }
    r.tail = nfc[pos..].to_string();
    if !r.tail.is_empty() {
        let t = r.tail.clone();
        res.add(Layer::Tokens, Class::Spacing, lit_bits(&t), || format!("tail {t:?}"));
    }
    (Toks { sents }, r)
}

pub fn tokens_down(t: &Toks, r: &R1) -> String {
    let mut core = String::new();
    for (s, toks) in t.sents.iter().enumerate() {
        for (k, x) in toks.iter().enumerate() {
            core.push_str(r.gaps.get(&(s, k)).map_or(canon_gap(&t.sents, s, k), String::as_str));
            core.push_str(x);
        }
    }
    core.push_str(&r.tail);
    format!("{}{}{}", r.lead, unnfc(&core, &r.raw), r.trail)
}

/// The words of an NFC text put back as written (NFC changes no space, so the runs between spaces line up).
fn unnfc(core: &str, raw: &BTreeMap<usize, String>) -> String {
    if raw.is_empty() {
        return core.to_string();
    }
    let mut out = String::new();
    let mut k = 0;
    let mut word = String::new();
    let flush = |word: &mut String, out: &mut String, k: &mut usize| {
        if !word.is_empty() {
            out.push_str(raw.get(k).map_or(word.as_str(), String::as_str));
            word.clear();
            *k += 1;
        }
    };
    for c in core.chars() {
        if c.is_whitespace() {
            flush(&mut word, &mut out, &mut k);
            out.push(c);
        } else {
            word.push(c);
        }
    }
    flush(&mut word, &mut out, &mut k);
    out
}

/// Canonical text of tokens (normal mode).
pub fn join_tokens(sents: &[Vec<String>]) -> String {
    let mut s = String::new();
    for (i, toks) in sents.iter().enumerate() {
        for (k, x) in toks.iter().enumerate() {
            s.push_str(canon_gap(sents, i, k));
            s.push_str(x);
        }
    }
    s
}

// ---------------------------------------------------------------- the models

/// The lookup generator of Hebrew forms: the commonest form of a lemma, tag and FEATS in some trees, backing off to
/// the lemma and tag, then the lemma; forms kept at a pointing level (accents off).
#[derive(Clone, Debug)]
pub struct Lookup {
    pub level: Level,
    full: BTreeMap<(String, String, String), String>,
    by_tag: BTreeMap<(String, String), String>,
    by_lemma: BTreeMap<String, String>,
    pub lemmas: BTreeSet<String>,
    /// with the word before it in its written word (its lemma, or «^» first): the form, when the trees were read
    /// with that context (`from_conllu_ctx`); empty otherwise
    ctx: BTreeMap<(String, String, String, String), String>,
}

/// A parser model: the segmenter and the lookup generator of its training trees.
pub struct HModel {
    pub seg: hbo::seg::Model,
    pub level: Level,
    pub lookup: Lookup,
    pub lemmas: BTreeSet<String>,
}

fn best(m: BTreeMap<String, BTreeMap<String, u32>>) -> BTreeMap<String, String> {
    m.into_iter().map(|(k, c)| (k, c.into_iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0))).unwrap().0)).collect()
}

impl Lookup {
    /// From CoNLL-U trees (the word lines; ranges and empty nodes left out).
    pub fn from_conllu(lex: &str, level: Level) -> Lookup {
        let mut full: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
        let mut by_tag: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
        let mut by_lemma: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
        let mut lemmas = BTreeSet::new();
        for l in lex.lines() {
            if l.starts_with('#') || l.is_empty() {
                continue;
            }
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() != 10 || c[0].contains('-') || c[0].contains('.') || c[2] == "_" {
                continue;
            }
            let f = norm(c[1], level);
            lemmas.insert(c[2].to_string());
            *full.entry(format!("{}\t{}\t{}", c[2], c[4], c[5])).or_default().entry(f.clone()).or_default() += 1;
            *by_tag.entry(format!("{}\t{}", c[2], c[4])).or_default().entry(f.clone()).or_default() += 1;
            *by_lemma.entry(c[2].to_string()).or_default().entry(f).or_default() += 1;
        }
        let split3 = |k: String| {
            let v: Vec<String> = k.split('\t').map(String::from).collect();
            (v[0].clone(), v[1].clone(), v[2].clone())
        };
        let split2 = |k: String| {
            let v: Vec<String> = k.split('\t').map(String::from).collect();
            (v[0].clone(), v[1].clone())
        };
        Lookup {
            ctx: BTreeMap::new(),
            level,
            full: best(full).into_iter().map(|(k, v)| (split3(k), v)).collect(),
            by_tag: best(by_tag).into_iter().map(|(k, v)| (split2(k), v)).collect(),
            by_lemma: best(by_lemma),
            lemmas,
        }
    }

    /// The same, and the commonest form of each word given the word before it in its written word (a prefix: the
    /// dagesh after וַ, the article's vowel), from the trees' multiword tokens.
    pub fn from_conllu_ctx(lex: &str, level: Level) -> Lookup {
        let mut l = Lookup::from_conllu(lex, level);
        let mut m: BTreeMap<(String, String, String, String), BTreeMap<String, u32>> = BTreeMap::new();
        let (mut range_end, mut prev) = (0usize, String::from("^"));
        for line in lex.lines() {
            if line.starts_with('#') || line.is_empty() {
                range_end = 0;
                prev = "^".into();
                continue;
            }
            let c: Vec<&str> = line.split('\t').collect();
            if c.len() != 10 || c[0].contains('.') {
                continue;
            }
            if let Some((_, b)) = c[0].split_once('-') {
                range_end = b.parse().unwrap_or(0);
                prev = "^".into();
                continue;
            }
            let id: usize = c[0].parse().unwrap_or(0);
            let p = if id <= range_end { prev.clone() } else { "^".to_string() };
            if c[2] != "_" {
                *m.entry((p, c[2].to_string(), c[4].to_string(), c[5].to_string())).or_default().entry(norm(c[1], level)).or_default() += 1;
            }
            prev = if id < range_end { c[2].to_string() } else { "^".into() };
        }
        l.ctx = m.into_iter().map(|(k, c)| (k, c.into_iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0))).unwrap().0)).collect();
        l
    }

    /// The form of a word given the word before it in its written word (None: the first), when the lookup has that
    /// context; else `form_of`.
    pub fn form_in(&self, w: &HW, prev: Option<&HW>) -> Option<String> {
        if !self.ctx.is_empty() && w.lemma != "_" {
            let p = prev.map_or("^".to_string(), |x| x.lemma.clone());
            if let Some(f) = self.ctx.get(&(p, w.lemma.clone(), w.xpos.clone(), w.feats.clone())) {
                return Some(f.clone());
            }
        }
        self.form_of(w)
    }

    /// The form of a word from its lemma, tag and FEATS: the commonest in the trees, backing off to the lemma and
    /// tag, then the lemma.
    pub fn form_of(&self, w: &HW) -> Option<String> {
        if w.lemma == "_" {
            return None;
        }
        self.full
            .get(&(w.lemma.clone(), w.xpos.clone(), w.feats.clone()))
            .or_else(|| self.by_tag.get(&(w.lemma.clone(), w.xpos.clone())))
            .or_else(|| self.by_lemma.get(&w.lemma))
            .cloned()
    }
}

impl HModel {
    pub fn load(dir: &Path) -> Result<HModel> {
        let seg = hbo::load_seg(dir).map_err(|e| anyhow!(e))?;
        let level = seg.level.unwrap_or(Level::Pointed);
        let lex = std::fs::read_to_string(dir.join("lex.conllu"))?;
        let lookup = Lookup::from_conllu(&lex, level);
        let lemmas = lookup.lemmas.clone();
        Ok(HModel { seg, level, lookup, lemmas })
    }

    pub fn form_of(&self, w: &HW) -> Option<String> {
        self.lookup.form_of(w)
    }
}

// ---------------------------------------------------------------- tokens ↔ tree

/// A Hebrew word of a tree (its form is the tree layer's; the frames layer compares the other fields).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HW {
    pub lemma: String,
    pub upos: String,
    pub xpos: String,
    pub feats: String,
    pub head: usize,
    pub rel: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTree {
    pub words: Vec<HW>,
    pub forms: Vec<String>,
    /// words per written word
    pub groups: Vec<usize>,
}

pub fn tree_of(s: &Sent) -> HTree {
    let mut t = HTree::default();
    for tok in &s.tokens {
        t.groups.push(tok.words.len());
        for w in &tok.words {
            let c = &w.cols;
            t.forms.push(c[1].clone());
            t.words.push(HW { lemma: c[2].clone(), upos: c[3].clone(), xpos: c[4].clone(), feats: c[5].clone(), head: c[6].parse().unwrap_or(0), rel: c[7].clone() });
        }
    }
    t
}

fn cons(s: &str) -> String {
    macula::consonants(s)
}

const PREFIXES: [&str; 8] = ["ו", "ה", "ב", "כ", "ל", "מ", "מן", "ש"];

/// Does word i join word i+1 in one written word?
pub fn joins(w: &[HW], i: usize) -> bool {
    let (a, b) = (&w[i], &w[i + 1]);
    if a.upos == "PUNCT" || b.upos == "PUNCT" {
        return false;
    }
    let prefix = matches!(a.upos.as_str(), "CCONJ" | "ADP" | "DET" | "SCONJ") && PREFIXES.contains(&cons(&a.lemma).as_str());
    let suffix = b.upos == "PRON" && !b.rel.starts_with("nsubj") && b.xpos != "personal" && b.xpos != "demonstrative";
    prefix || suffix
}

#[cfg(test)]
fn groups_of(w: &[HW]) -> Vec<usize> {
    let mut g = Vec::new();
    let mut n = 0;
    for i in 0..w.len() {
        n += 1;
        if i + 1 == w.len() || !joins(w, i) {
            g.push(n);
            n = 0;
        }
    }
    g
}

/// A written word from its words' forms: a bare ה after the first word is the swallowed article.
fn written(forms: &[String]) -> String {
    forms.iter().enumerate().filter(|(k, f)| !(*k > 0 && f.as_str() == "ה")).map(|(_, f)| f.as_str()).collect()
}

fn accents(f: &str) -> usize {
    f.chars().filter(|&c| matches!(c, '\u{0591}'..='\u{05AF}' | '\u{05BD}' | '\u{05BF}' | '\u{05C4}' | '\u{05C5}')).count()
}

#[derive(Clone, Debug, Default)]
pub struct R2 {
    pub forms: BTreeMap<usize, String>,
    pub joins: BTreeSet<usize>,
    pub written: BTreeMap<usize, String>,
}

/// The tree layer's residual of one sentence: its tokens from the tree.
pub fn tree_encode(m: &Lookup, t: &HTree, toks: &[String], res: &mut Residual) -> R2 {
    let mut r = R2::default();
    let n = t.words.len();
    let prev = prev_in_group(&t.groups, n);
    for (i, w) in t.words.iter().enumerate() {
        let f = &t.forms[i];
        let made = m.form_in(w, prev[i].map(|p| &t.words[p]));
        if made.as_deref() == Some(f.as_str()) {
            continue;
        }
        let units = hbo::text::units(f).len();
        let a = accents(f);
        let nf = norm(f, m.level);
        let (c, bits) = match &made {
            Some(g) if *g == nf && a > 0 => (Class::Accent, a as f64 * (5.0 + pos_bits(units))),
            Some(_) => (Class::Form, lit_bits(&nf) + a as f64 * (5.0 + pos_bits(units))),
            None => (Class::Spelling, lit_bits(f)),
        };
        let (f2, g2, l2) = (f.clone(), made.clone().unwrap_or_default(), w.lemma.clone());
        res.add_k(Layer::Tree, c, bits + pos_bits(n), w.xpos.clone(), || format!("«{f2}» ← {l2}/{} gives «{g2}»", w.xpos));
        r.forms.insert(i, f.clone());
    }
    // word boundaries
    let mut bound = BTreeSet::new();
    let mut at = 0;
    for g in &t.groups {
        at += g;
        bound.insert(at);
    }
    for i in 0..n.saturating_sub(1) {
        let is_bound = bound.contains(&(i + 1));
        if is_bound == joins(&t.words, i) {
            r.joins.insert(i);
            let (a, b) = (t.forms[i].clone(), t.forms[i + 1].clone());
            res.add_k(Layer::Tree, Class::Spacing, pos_bits(n), format!("{} {}", t.words[i].upos, t.words[i + 1].upos), || format!("«{a}»|«{b}»: written {}", if is_bound { "apart" } else { "together" }));
        }
    }
    let mut k = 0;
    for (j, g) in t.groups.iter().enumerate() {
        let w = written(&t.forms[k..k + g]);
        if toks.get(j).map(String::as_str) != Some(w.as_str()) {
            let tk = toks.get(j).cloned().unwrap_or_default();
            r.written.insert(j, tk.clone());
            res.add(Layer::Tree, Class::Spelling, lit_bits(&tk) + pos_bits(t.groups.len()), || format!("written «{tk}», its words give «{w}»"));
        }
        k += g;
    }
    r
}

/// The word before each word in its written word (by the groups' sizes).
fn prev_in_group(groups: &[usize], n: usize) -> Vec<Option<usize>> {
    let mut out = vec![None; n];
    let mut k = 0;
    for g in groups {
        for j in 1..*g {
            if k + j < n {
                out[k + j] = Some(k + j - 1);
            }
        }
        k += g;
    }
    out
}

/// Words → forms, groups, written words (exact with the residual; canonical without).
pub fn tree_decode(m: &Lookup, words: &[HW], r: Option<&R2>) -> (Vec<String>, Vec<usize>, Vec<String>) {
    let mut groups = Vec::new();
    let mut n = 0;
    for i in 0..words.len() {
        n += 1;
        let j = i + 1 < words.len() && joins(words, i);
        let j = if r.is_some_and(|r| r.joins.contains(&i)) { !j } else { j };
        if i + 1 == words.len() || !j {
            groups.push(n);
            n = 0;
        }
    }
    let prev = prev_in_group(&groups, words.len());
    let forms: Vec<String> = words.iter().enumerate().map(|(i, w)| r.and_then(|r| r.forms.get(&i).cloned()).or_else(|| m.form_in(w, prev[i].map(|p| &words[p]))).unwrap_or_else(|| w.lemma.replace('_', ""))).collect();
    let mut toks = Vec::new();
    let mut k = 0;
    for (j, g) in groups.iter().enumerate() {
        toks.push(r.and_then(|r| r.written.get(&j).cloned()).unwrap_or_else(|| written(&forms[k..k + g])));
        k += g;
    }
    (forms, groups, toks)
}

// ---------------------------------------------------------------- tree ↔ frames

/// The class of a Hebrew word that is missing or added.
pub fn hclass(w: &HW) -> Class {
    if w.upos == "PUNCT" || w.rel == "punct" {
        Class::Punct
    } else if w.xpos == "negative" {
        Class::Neg
    } else if w.upos == "NUM" || w.rel == "nummod" || w.xpos.ends_with("_number") {
        Class::Quantity
    } else if w.upos == "AUX" || w.rel == "cop" {
        Class::Tense
    } else if matches!(w.upos.as_str(), "ADP" | "DET" | "CCONJ" | "SCONJ" | "PART") || matches!(w.rel.split(':').next().unwrap_or(""), "case" | "det" | "mark" | "cc" | "fixed") {
        Class::Function
    } else {
        Class::Dropped
    }
}

fn hword_bits(w: &HW) -> f64 {
    match hclass(w) {
        Class::Punct => 5.0,
        Class::Function | Class::Neg | Class::Tense => 8.0,
        _ => lemma_bits(&w.lemma),
    }
}

/// The id of an entity or verb: its lemma, or `~` and its form when the parser has no lemma for it.
fn idf(w: &HW, form: &str) -> String {
    if w.lemma == "_" || w.lemma.is_empty() { format!("~{}", norm(form, Level::Pointed)) } else { w.lemma.clone() }
}

fn base(rel: &str) -> &str {
    rel.split(':').next().unwrap_or(rel)
}

/// Frames of one Hebrew sentence, with `macula prag`'s reading; `open` carries the speech state of the volume.
pub fn read(t: &HTree, open: &mut bool, rules: &macula::prag::Rules) -> Vec<F> {
    let w = &t.words;
    let n = w.len();
    let kids = |i: usize| -> Vec<usize> { (0..n).filter(|&k| w[k].head == i + 1).collect() };
    let ent = |i: usize| -> E {
        let ks = kids(i);
        E {
            id: idf(&w[i], &t.forms[i]),
            pron: (w[i].upos == "PRON").then(|| idf(&w[i], &t.forms[i])),
            plural: w[i].feats.contains("Number=Plur") || w[i].feats.contains("Number=Dual"),
            named: w[i].upos == "PROPN",
            attrs: ks.iter().filter(|&&k| matches!(base(&w[k].rel), "amod" | "nummod")).map(|&k| idf(&w[k], &t.forms[k])).collect(),
            and: ks.iter().filter(|&&k| base(&w[k].rel) == "conj" && w[k].upos != "VERB").map(|&k| idf(&w[k], &t.forms[k])).collect(),
            ..Default::default()
        }
    };
    // macula prag on the words (no marks), heads renumbered, the verb form in MISC Type=
    let keep: Vec<usize> = (0..n).filter(|&i| w[i].upos != "PUNCT").collect();
    let newi: BTreeMap<usize, usize> = keep.iter().enumerate().map(|(k, &i)| (i + 1, k + 1)).collect();
    let rows: Vec<Vec<String>> = keep
        .iter()
        .map(|&i| {
            let x = &w[i];
            let h = if x.head == 0 { 0 } else { newi.get(&x.head).copied().unwrap_or(0) };
            vec![newi[&(i + 1)].to_string(), t.forms[i].clone(), x.lemma.clone(), x.upos.clone(), x.xpos.clone(), x.feats.clone(), h.to_string(), x.rel.clone(), "_".into(), format!("Type={}", x.xpos)]
        })
        .collect();
    let rr: Vec<Vec<&str>> = rows.iter().map(|r| r.iter().map(String::as_str).collect()).collect();
    let reading = (!rr.is_empty()).then(|| macula::prag::read_sentence(&rr, *open, rules));
    if let Some(r) = &reading {
        *open = r.open;
    }
    let act = reading.as_ref().and_then(|r| Act::parse(r.row.act)).unwrap_or(Act::Other);
    // the predicates: verbs, and a non-verb root with a subject
    let preds: Vec<usize> = (0..n).filter(|&i| w[i].upos == "VERB" || (w[i].head == 0 && w[i].upos != "VERB" && kids(i).iter().any(|&k| base(&w[k].rel) == "nsubj"))).collect();
    let fidx: BTreeMap<usize, usize> = preds.iter().enumerate().map(|(k, &i)| (i, k)).collect();
    let mut fs: Vec<F> = Vec::new();
    for &i in &preds {
        let ks = kids(i);
        let role = |r: &str| ks.iter().find(|&&k| base(&w[k].rel) == r && w[k].upos != "VERB").map(|&k| ent(k));
        let verb = w[i].upos == "VERB";
        let saying = verb && macula::SAYING.contains(&cons(&w[i].lemma).as_str());
        let kind = if saying {
            Kind::Say(act)
        } else if verb {
            Kind::Event
        } else {
            Kind::Fact(Link::Is)
        };
        let obls: Vec<(String, E)> = ks
            .iter()
            .filter(|&&k| base(&w[k].rel) == "obl" && w[k].upos != "VERB")
            .map(|&k| {
                let p = kids(k).into_iter().find(|&c| base(&w[c].rel) == "case").map(|c| idf(&w[c], &t.forms[c])).unwrap_or_default();
                (p, ent(k))
            })
            .collect();
        let neg = ks.iter().any(|&k| w[k].xpos == "negative");
        let attrs: Vec<String> = ks.iter().filter(|&&k| base(&w[k].rel) == "advmod" && w[k].xpos != "negative").map(|&k| idf(&w[k], &t.forms[k])).collect();
        let quote: Vec<usize> = if saying { ks.iter().filter(|&&k| base(&w[k].rel) == "ccomp").filter_map(|k| fidx.get(k).copied()).collect() } else { Vec::new() };
        fs.push(F {
            kind,
            verb: idf(&w[i], &t.forms[i]),
            neg,
            agent: role("nsubj"),
            patient: if verb { role("obj") } else { Some(ent(i)) },
            recipient: role("iobj"),
            place: None,
            attrs,
            obls,
            quote,
            quoted: false,
            cause: None,
            next: None,
            cond: false,
            // Hebrew verb forms are not read into tense/aspect yet: the frame says «not read»
            tam: Default::default(),
            ..Default::default()
        });
    }
    if fs.is_empty() && !keep.is_empty() {
        // no predicate: the root as a phrase
        let root = (0..n).find(|&i| w[i].head == 0).unwrap_or(keep[0]);
        fs.push(F { kind: Kind::Bare, patient: Some(ent(root)), ..Default::default() });
    }
    if let (Some(r), Some(f)) = (&reading, fs.first_mut()) {
        f.attrs.insert(0, format!("@{}/{}/{:?}{}", r.row.form, r.row.act, r.speech, if r.negation { "/neg" } else { "" }));
    }
    fs
}

/// The realizer: frames → words (no forms).
pub fn realize(fs: &[F]) -> Vec<HW> {
    let mut out: Vec<HW> = Vec::new();
    let mut add = |out: &mut Vec<HW>, lemma: &str, upos: &str, xpos: &str, feats: &str, head: usize, rel: &str| -> usize {
        let lemma = if lemma.starts_with('~') { "_" } else { lemma };
        out.push(HW { lemma: lemma.into(), upos: upos.into(), xpos: xpos.into(), feats: feats.into(), head, rel: rel.into() });
        out.len()
    };
    fn np(out: &mut Vec<HW>, add: &mut dyn FnMut(&mut Vec<HW>, &str, &str, &str, &str, usize, &str) -> usize, e: &E, head: usize, rel: &str, prep: Option<&str>) {
        let (upos, xpos) = if e.pron.is_some() {
            ("PRON", "pronominal")
        } else if e.named {
            ("PROPN", "PROPN")
        } else {
            ("NOUN", "NOUN")
        };
        let feats = if e.plural { "Number=Plur" } else { "_" };
        let h = add(out, &e.id, upos, xpos, feats, head, rel);
        if let Some(p) = prep.filter(|p| !p.is_empty()) {
            add(out, p, "ADP", "ADP", "_", h, "case");
        }
        for a in &e.attrs {
            add(out, a, "ADJ", "adjective", "_", h, "amod");
        }
        for c in &e.and {
            let k = add(out, c, "NOUN", "NOUN", "_", h, "conj");
            add(out, "וְ", "CCONJ", "CCONJ", "_", k, "cc");
        }
    }
    let inner: BTreeSet<usize> = fs.iter().flat_map(|f| f.quote.iter().copied()).collect();
    let mut heads: Vec<usize> = vec![0; fs.len()];
    let mut root = 0;
    let narrate = fs.first().is_some_and(|f| f.attrs.first().is_some_and(|a| a.contains("/narrate/")));
    for (i, f) in fs.iter().enumerate() {
        let (head, rel) = if let Some(j) = (0..fs.len()).find(|&j| fs[j].quote.contains(&i) && heads[j] > 0) {
            (heads[j], "ccomp")
        } else if root == 0 {
            (0, "root")
        } else {
            (root, "conj")
        };
        let _ = inner.contains(&i);
        let h = match f.kind {
            Kind::Bare => match &f.patient {
                Some(e) => {
                    np(&mut out, &mut add, e, head, rel, None);
                    out.len() - e.attrs.len() - 2 * e.and.len()
                }
                None => continue,
            },
            Kind::Fact(_) => {
                let e = f.patient.clone().unwrap_or_default();
                np(&mut out, &mut add, &e, head, rel, None);
                out.len() - e.attrs.len() - 2 * e.and.len()
            }
            _ => add(&mut out, &f.verb, "VERB", if narrate { "wayyiqtol" } else { "qatal" }, "_", head, rel),
        };
        heads[i] = h;
        if root == 0 {
            root = h;
        } else if rel == "conj" {
            add(&mut out, "וְ", "CCONJ", "CCONJ", "_", h, "cc");
        }
        if f.neg {
            add(&mut out, "לֹא", "ADV", "negative", "_", h, "advmod");
        }
        if let Some(e) = &f.agent {
            np(&mut out, &mut add, e, h, "nsubj", None);
        }
        if !matches!(f.kind, Kind::Fact(_) | Kind::Bare) {
            if let Some(e) = &f.patient {
                np(&mut out, &mut add, e, h, "obj", None);
            }
        }
        if let Some(e) = &f.recipient {
            np(&mut out, &mut add, e, h, "iobj", None);
        }
        for (p, e) in &f.obls {
            np(&mut out, &mut add, e, h, "obl", Some(p));
        }
        for a in f.attrs.iter().filter(|a| !a.starts_with('@')) {
            add(&mut out, a, "ADV", "ADV", "_", h, "advmod");
        }
    }
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    Take { from: usize, patch: BTreeMap<u8, String>, head: Option<usize> },
    Insert(HW),
}

pub type R3 = Vec<Step>;

fn field(w: &HW, k: u8) -> &str {
    match k {
        0 => &w.lemma,
        1 => &w.upos,
        2 => &w.xpos,
        3 => &w.feats,
        _ => &w.rel,
    }
}

fn set_field(w: &mut HW, k: u8, v: &str) {
    let v = v.to_string();
    match k {
        0 => w.lemma = v,
        1 => w.upos = v,
        2 => w.xpos = v,
        3 => w.feats = v,
        _ => w.rel = v,
    }
}

/// The edit script realized → parsed words: align by lemma (consonants), then by aligned head and relation.
pub fn frames_encode(t: &[HW], forms: &[String], r: &[HW], res: &mut Residual) -> R3 {
    let mut a: Vec<Option<usize>> = vec![None; t.len()];
    let mut used = vec![false; r.len()];
    for i in 0..t.len() {
        let l = cons(&t[i].lemma);
        if l.is_empty() {
            continue;
        }
        if let Some(j) = (0..r.len()).find(|&j| !used[j] && cons(&r[j].lemma) == l) {
            a[i] = Some(j);
            used[j] = true;
        }
    }
    for _ in 0..2 {
        for i in 0..t.len() {
            if a[i].is_some() || hclass(&t[i]) == Class::Punct {
                continue;
            }
            let th = t[i].head;
            let rh = if th == 0 { Some(0) } else { a[th - 1].map(|j| j + 1) };
            let Some(rh) = rh else { continue };
            if let Some(j) = (0..r.len()).find(|&j| !used[j] && r[j].head == rh && base(&r[j].rel) == base(&t[i].rel)) {
                a[i] = Some(j);
                used[j] = true;
            }
        }
    }
    let n = t.len();
    let mut inv = vec![None; r.len()];
    for (i, x) in a.iter().enumerate() {
        if let Some(j) = x {
            inv[*j] = Some(i + 1);
        }
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let w = &t[i];
        let f = forms.get(i).cloned().unwrap_or_default();
        match a[i] {
            None => {
                let c = hclass(w);
                res.add_k(Layer::Frames, c, hword_bits(w) + 12.0 + pos_bits(n), format!("missing {}", w.rel), || format!("«{f}» {}/{} not in the frames", w.xpos, w.rel));
                out.push(Step::Insert(w.clone()));
            }
            Some(j) => {
                let x = &r[j];
                let mut patch = BTreeMap::new();
                for k in 0..4u8 {
                    if field(x, k) != field(w, k) {
                        patch.insert(k, field(w, k).to_string());
                        let c = match k {
                            0 => match hclass(w) {
                                Class::Dropped => Class::Lexical,
                                c => c,
                            },
                            1 => Class::Attachment,
                            2 if w.upos == "VERB" => Class::Tense,
                            _ => Class::Form,
                        };
                        let bits = if k == 0 { hword_bits(w) } else { 6.0 };
                        let (xv, wv) = (field(x, k).to_string(), field(w, k).to_string());
                        res.add_k(Layer::Frames, c, bits, ["lemma", "upos", "xpos", "feats"][k as usize].to_string(), || format!("«{f}»: realized {xv}, parsed {wv}"));
                    }
                }
                let mapped = if x.head == 0 { Some(0) } else { inv[x.head - 1] };
                let head = (mapped != Some(w.head)).then_some(w.head);
                if x.rel != w.rel {
                    patch.insert(4, w.rel.clone());
                }
                if head.is_some() || x.rel != w.rel {
                    let bits = if head.is_some() { pos_bits(n) } else { 0.0 } + if x.rel != w.rel { 6.0 } else { 0.0 };
                    let (xr, wr) = (x.rel.clone(), w.rel.clone());
                    res.add_k(Layer::Frames, Class::Attachment, bits, format!("{xr}→{wr}"), || format!("«{f}»: realized {xr}, parsed {wr}"));
                }
                out.push(Step::Take { from: j, patch, head });
            }
        }
    }
    for (j, x) in r.iter().enumerate() {
        if inv[j].is_none() {
            let c = hclass(x);
            let l = x.lemma.clone();
            res.add_k(Layer::Frames, c, pos_bits(r.len()), format!("realized {}", x.rel), || format!("realized «{l}» {} has no word in the text", x.rel));
        }
    }
    out
}

pub fn frames_apply(r: &[HW], script: &R3) -> Vec<HW> {
    let mut inv = vec![None; r.len()];
    for (i, s) in script.iter().enumerate() {
        if let Step::Take { from, .. } = s {
            inv[*from] = Some(i + 1);
        }
    }
    script
        .iter()
        .map(|s| match s {
            Step::Insert(w) => w.clone(),
            Step::Take { from, patch, head } => {
                let mut w = r[*from].clone();
                w.head = head.unwrap_or(if w.head == 0 { 0 } else { inv[w.head - 1].unwrap_or(0) });
                for (k, v) in patch {
                    set_field(&mut w, *k, v);
                }
                w
            }
        })
        .collect()
}

// ---------------------------------------------------------------- Aramaic: the tokens as they are

/// The residual of a sentence with no parser: every token.
pub fn unparsed(toks: &[String], res: &mut Residual) {
    for t in toks {
        let t2 = t.clone();
        res.add(Layer::Tree, Class::Unparsed, lit_bits(t), || t2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_come_back() {
        let p = "«וַיְהִי־כֵן׃  וַיַּרְא אֱלֹהִים׃»";
        let mut res = Residual::default();
        let (t, r) = tokens_up(p, &mut res);
        assert_eq!(tokens_down(&t, &r), p);
        assert_eq!(t.sents.len(), 2);
        // negative control: without the residual gaps the double space is lost
        let r2 = R1 { gaps: BTreeMap::new(), ..r.clone() };
        assert_ne!(tokens_down(&t, &r2), p);
        // a word that is not NFC (dagesh before qamats) comes back as written
        let q = "וְ ב\u{05BC}\u{05B8}א׃";
        let (t, r) = tokens_up(q, &mut res);
        assert_eq!(r.raw.len(), 1);
        assert_eq!(tokens_down(&t, &r), q);
    }

    #[test]
    fn script_gives_the_tree_back() {
        let t = vec![
            HW { lemma: "וְ".into(), upos: "CCONJ".into(), xpos: "CCONJ".into(), feats: "_".into(), head: 2, rel: "cc".into() },
            HW { lemma: "אָמַר".into(), upos: "VERB".into(), xpos: "wayyiqtol".into(), feats: "Number=Sing".into(), head: 0, rel: "root".into() },
            HW { lemma: "אֱלֹהִים".into(), upos: "NOUN".into(), xpos: "NOUN".into(), feats: "Number=Plur".into(), head: 2, rel: "nsubj".into() },
        ];
        let forms = vec!["וַ".to_string(), "יֹּאמֶר".to_string(), "אֱלֹהִים".to_string()];
        let tr = HTree { words: t.clone(), forms: forms.clone(), groups: vec![2, 1] };
        let mut open = false;
        let fs = read(&tr, &mut open, &macula::prag::Rules::new(&[], &[]));
        assert_eq!(fs.len(), 1);
        let r = realize(&fs);
        let mut res = Residual::default();
        let s = frames_encode(&t, &forms, &r, &mut res);
        assert_eq!(frames_apply(&r, &s), t);
        // negative control: a script without its inserts and patches does not give the tree back
        let bare: R3 = s.iter().map(|x| match x {
            Step::Take { from, .. } => Step::Take { from: *from, patch: BTreeMap::new(), head: None },
            Step::Insert(w) => Step::Insert(HW { lemma: "x".into(), ..w.clone() }),
        }).collect();
        assert_ne!(frames_apply(&r, &bare), t);
        assert_eq!(groups_of(&t), vec![2, 1]);
    }
}
