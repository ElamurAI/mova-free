//! The segmenter: a written word into MACULA's words (prefixes, the host, suffixes), learned from the trees.
//!
//! A split is a list of pieces, each a number of letter units cut from the written word, or `H`, the article a
//! preposition swallowed (בַּ = בְּ + הַ, written ה). The written word is found in this order:
//!
//! 1. `lex`: the word was seen in training; its commonest split is taken.
//! 2. Otherwise the prefixes, then the suffixes, by learned tables:
//!    - `pre`: the prefixes are read off the word's first k units (k from `max_k` down to 1, the longest context seen at
//!      least `min` times), as they split most often in the rarer words of training (seen at most `rare` times);
//!    - `rest`: what follows the prefixes was seen in training after its own prefixes; its commonest suffix is taken;
//!    - `suf`: otherwise the suffixes are read off the last k units of that rest, as `pre` reads the first.
//!
//! Every choice can say why: the rule, the context and the counts.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::conllu::{Sent, Token, Word};
use crate::text::{Level, norm, units};
use unicode_normalization::UnicodeNormalization;

/// A piece of a written word: a word of `n` letters (each with its marks), or `H`, the swallowed article. A word
/// that `lead`s takes the vowel of the letter before it: MACULA cuts a suffix before that vowel (כֻּלּ + ָם, with the
/// dagesh left on the host).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Cut(usize, bool),
    H,
}

pub fn label(ps: &[Piece]) -> String {
    ps.iter().map(|p| match p {
        Piece::Cut(n, false) => n.to_string(),
        Piece::Cut(n, true) => format!("^{n}"),
        Piece::H => "H".into(),
    }).collect::<Vec<_>>().join(".")
}

pub fn parse_label(s: &str) -> Vec<Piece> {
    if s.is_empty() {
        return Vec::new();
    }
    s.split('.').map(|x| match x {
        "H" => Piece::H,
        _ => match x.strip_prefix('^') {
            Some(n) => Piece::Cut(n.parse().unwrap_or(1), true),
            None => Piece::Cut(x.parse().unwrap_or(1), false),
        },
    }).collect()
}

fn letters(ps: &[Piece]) -> usize {
    ps.iter().map(|p| if let Piece::Cut(n, _) = p { *n } else { 0 }).sum()
}

/// A vowel point: what a word that leads takes from the letter before it.
fn vowel(c: char) -> bool {
    matches!(c, '\u{05B0}'..='\u{05BB}' | '\u{05C7}')
}

/// A token's split in the trees: the prefixes, the host and the suffixes.
#[derive(Clone, Debug, PartialEq)]
pub struct Gold {
    pub pre: Vec<Piece>,
    pub host: Piece,
    pub suf: Vec<Piece>,
}

impl Gold {
    pub fn all(&self) -> Vec<Piece> {
        let mut v = self.pre.clone();
        v.push(self.host);
        v.extend(&self.suf);
        v
    }
}

fn elided(w: &Word) -> bool {
    w.form() == "ה" && w.misc("MId").is_some_and(|m| m.ends_with('ה'))
}

/// A suffix word: a pronoun suffix (MACULA `Type=pronominal`), or the directional or paragogic ה and נ (X).
fn suffix_word(w: &Word) -> bool {
    (w.upos() == "PRON" && w.misc("Type") == Some("pronominal")) || w.upos() == "X"
}

/// The split of a token of the trees, at a level of pointing; None when the pieces, cut from the written word, do
/// not give its words' forms.
pub fn gold(t: &Token, level: Level) -> Option<Gold> {
    let key = norm(&t.form, level);
    let mut pieces = Vec::new();
    let mut forms = Vec::new();
    for w in &t.words {
        if elided(w) && t.words.len() > 1 {
            pieces.push(Piece::H);
            forms.push("ה".to_string());
        } else {
            let f = norm(w.form(), level);
            let u = units(&f);
            let lead = f.chars().next().is_some_and(crate::text::combining);
            pieces.push(Piece::Cut(u.len() - lead as usize, lead));
            forms.push(f);
        }
    }
    if pieces.iter().any(|p| matches!(p, Piece::Cut(0, _))) || letters(&pieces) != units(&key).len() {
        return None;
    }
    if Model::cut(&key, &pieces).into_iter().map(|x| x.0).collect::<Vec<_>>() != forms {
        return None;
    }
    let mut h = t.words.len() - 1;
    while h > 0 && suffix_word(&t.words[h]) && pieces[h - 1] != Piece::H {
        h -= 1;
    }
    if pieces[h] == Piece::H {
        return None;
    }
    Some(Gold { pre: pieces[..h].to_vec(), host: pieces[h], suf: pieces[h + 1..].to_vec() })
}

#[derive(Clone, Debug)]
pub struct Opts {
    pub max_k: usize,
    pub min: u32,
    pub rare: u32,
    pub rest: bool,
    pub lex: bool,
    /// rank the candidates of an unseen word by the product of the prefix and suffix shares (`joint`), not the
    /// prefixes first
    pub joint: bool,
    /// a prefix chain (the letters of the prefixes: ו.ב.H) or a suffix must be seen at least this many times in
    /// training (0: no check)
    pub chain: u32,
    /// unseen words: a candidate whose host was never a host in training has its share multiplied by this
    /// (`hostlex:0.3`; `hostlex` alone is 0, a hard filter: keep the known hosts if any), and all are ranked by share
    pub hostlex: Option<f64>,
    /// a partly pointed word not in the lexicon: the lexicon's forms whose points include its points (`partial`)
    pub partial: bool,
    /// a seen word the lexicon splits more than one way: its splits weighed by its neighbours (`ctx:K`, K the
    /// smoothing toward the lexicon's shares)
    pub ctx: Option<f64>,
    /// the consonant check on every candidate (on by default; `no-check` turns it off, as a negative control)
    pub check: bool,
}

impl Default for Opts {
    fn default() -> Opts {
        Opts { max_k: 4, min: 3, rare: 2, rest: true, lex: true, joint: false, chain: 0, hostlex: None, partial: false, check: true, ctx: None }
    }
}

impl Opts {
    /// The switches on: `joint,chain:2,hostlex,partial`.
    pub fn with(&self) -> String {
        let mut v = Vec::new();
        if self.joint {
            v.push("joint".to_string());
        }
        if self.chain > 0 {
            v.push(format!("chain:{}", self.chain));
        }
        if let Some(b) = self.hostlex {
            v.push(if b == 0.0 { "hostlex".into() } else { format!("hostlex:{b}") });
        }
        if self.partial {
            v.push("partial".into());
        }
        if !self.check {
            v.push("no-check".into());
        }
        if let Some(k) = self.ctx {
            v.push(format!("ctx:{k}"));
        }
        v.join(",")
    }
    /// Switches on (`joint,chain:2,hostlex,partial`), or off with `no-` (`no-hostlex`, `chain:0`).
    pub fn set_with(&mut self, s: &str) {
        for w in s.split(',').filter(|x| !x.is_empty()) {
            let (on, w) = match w.strip_prefix("no-") {
                Some(x) => (false, x),
                None => (true, w),
            };
            match w.split_once(':') {
                Some(("chain", n)) => self.chain = if on { n.parse().unwrap_or(1) } else { 0 },
                Some(("hostlex", b)) => self.hostlex = if on { Some(b.parse().unwrap_or(0.0)) } else { None },
                Some(("ctx", k)) => self.ctx = if on { Some(k.parse().unwrap_or(1.0)) } else { None },
                _ => match w {
                    "joint" => self.joint = on,
                    "chain" => self.chain = on as u32,
                    "hostlex" => self.hostlex = on.then_some(0.0),
                    "partial" => self.partial = on,
                    "check" => self.check = on,
                    _ => {}
                },
            }
        }
    }
}

type Counts = BTreeMap<String, BTreeMap<String, u32>>;

#[derive(Default)]
pub struct Model {
    pub level: Option<Level>,
    pub opts: Opts,
    pub lex: Counts,
    pub rest: Counts,
    pub pre: Counts,
    pub suf: Counts,
    /// the splits of the words the lexicon splits more than one way, by a neighbour (`<form> p:<word before>`,
    /// `<form> n:<word after>`)
    pub ctx: Counts,
    /// derived from `lex` (`derive`): prefix chains and suffixes with their counts, hosts, and the lexicon's forms by
    /// their letters
    pub chains: BTreeMap<String, u32>,
    pub sufs: BTreeMap<String, u32>,
    pub hosts: BTreeMap<String, u32>,
    pub by_letters: BTreeMap<String, Vec<String>>,
    /// with `partial`: the unpointed model, for a partly pointed word the lexicon does not know (`thinly`)
    pub fallback: Option<Box<Model>>,
}

/// A candidate split of a written word, with its share (the product of the shares of the tables used) and why.
#[derive(Clone, Debug, PartialEq)]
pub struct Cand {
    pub ps: Vec<Piece>,
    pub p: f64,
    pub rule: &'static str,
    pub why: String,
    /// the host's piece (for the rules' candidates; 0 for the lexicon's)
    pub h: usize,
}

/// The letters of a split's pieces, the host being piece `h`: (prefix chain, host, suffix chain), points dropped in
/// the chains (`H` for the swallowed article), the host at the form's level; None when the split does not fit.
pub fn parts(key: &str, ps: &[Piece], h: usize) -> Option<(String, String, String)> {
    let u = units(key);
    if h >= ps.len() || letters(ps) != u.len() || ps.iter().any(|p| matches!(p, Piece::Cut(0, _))) {
        return None;
    }
    let mut at = 0;
    let mut strs: Vec<String> = Vec::new();
    for p in ps {
        match *p {
            Piece::H => strs.push("H".into()),
            Piece::Cut(n, _) => {
                strs.push(u[at..at + n].concat());
                at += n;
            }
        }
    }
    let bare = |v: &[String]| v.iter().map(|s| norm(s, Level::Unpointed)).collect::<Vec<_>>().join(".");
    Some((bare(&strs[..h]), strs[h].clone(), bare(&strs[h + 1..])))
}

/// The marks on each letter unit of a form.
fn marks(key: &str) -> Vec<(char, Vec<char>)> {
    units(key).into_iter().map(|u| {
        let mut cs = u.chars();
        let base = cs.next().unwrap_or(' ');
        let mut m: Vec<char> = cs.collect();
        m.sort();
        (base, m)
    }).collect()
}

/// The context of the i-th written word of a sentence: the word before it (`p:`) and after it (`n:`), `<s>` and
/// `</s>` at the ends.
pub fn neighbours(keys: &[String], i: usize) -> [String; 2] {
    let p = if i == 0 { "<s>" } else { keys[i - 1].as_str() };
    let n = keys.get(i + 1).map(String::as_str).unwrap_or("</s>");
    [format!("p:{p}"), format!("n:{n}")]
}

/// Is a pointed word only partly pointed: a letter inside it (not the last, not a mater ו י א ה) without a point?
pub fn thinly(key: &str) -> bool {
    // a letter whose vowel is the next letter's (ר before וּ or וֹ), a mater, or a final form (a ketiv written
    // close) counts as pointed
    let m = marks(key.trim_end_matches(crate::text::MAQAF));
    (0..m.len().saturating_sub(1)).any(|i| {
        let (c, ms) = &m[i];
        let next_vowel = m[i + 1].0 == 'ו' && !m[i + 1].1.is_empty();
        c.is_alphabetic() && !matches!(c, 'ו' | 'י' | 'א' | 'ה' | 'ך' | 'ם' | 'ן' | 'ף' | 'ץ') && ms.is_empty() && !next_vowel
    })
}

/// Do the points of `part` all appear on the same letters of `full`?
fn within(part: &str, full: &str) -> bool {
    let (a, b) = (marks(part), marks(full));
    a.len() == b.len() && a.iter().zip(&b).all(|(x, y)| x.0 == y.0 && x.1.iter().all(|c| y.1.contains(c)))
}

fn add(m: &mut Counts, k: &str, l: &str) {
    *m.entry(k.to_string()).or_default().entry(l.to_string()).or_default() += 1;
}

/// The commonest label, the smaller on a tie; with its count and the total.
fn top(m: &BTreeMap<String, u32>) -> (String, u32, u32) {
    let total = m.values().sum();
    let (l, n) = m.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(l, n)| (l.clone(), *n)).unwrap_or_default();
    (l, n, total)
}

/// Why a token was split as it was.
#[derive(Clone, Debug, PartialEq)]
pub struct Why {
    pub rule: &'static str,
    pub text: String,
}

impl Model {
    pub fn learn(sents: &[Sent], level: Level, opts: Opts) -> (Model, usize) {
        let mut m = Model { level: Some(level), opts, ..Default::default() };
        let mut bad = 0;
        // (key, gold) of every token; frequencies first, for the rare-word tables
        let mut seen: Vec<(Vec<String>, String, Gold)> = Vec::new();
        let mut freq: BTreeMap<String, u32> = BTreeMap::new();
        for t in sents.iter().flat_map(|s| &s.tokens) {
            if t.words.iter().all(|w| w.upos() == "PUNCT") {
                continue;
            }
            let Some(g) = gold(t, level) else {
                bad += 1;
                continue;
            };
            let key = norm(&t.form, level);
            *freq.entry(key.clone()).or_default() += 1;
            let u: Vec<String> = units(&key).into_iter().map(String::from).collect();
            add(&mut m.lex, &key, &label(&g.all()));
            if let Some((c, h, s)) = parts(&key, &g.all(), g.pre.len()) {
                *m.chains.entry(c).or_default() += 1;
                *m.hosts.entry(h).or_default() += 1;
                *m.sufs.entry(s).or_default() += 1;
            }
            seen.push((u, key, g));
        }
        m.index();
        for (u, key, g) in &seen {
            let p = letters(&g.pre);
            let rest: String = u[p..].concat();
            add(&mut m.rest, &rest, &label(&g.suf));
            if freq[key] > m.opts.rare {
                continue;
            }
            for k in 1..=m.opts.max_k.min(u.len()) {
                add(&mut m.pre, &u[..k].concat(), &label(&g.pre));
            }
            let r = &u[p..];
            for k in 1..=m.opts.max_k.min(r.len()) {
                add(&mut m.suf, &r[r.len() - k..].concat(), &label(&g.suf));
            }
        }
        // the neighbours of the words the lexicon splits more than one way
        for s in sents {
            let keys: Vec<String> = s.tokens.iter().map(|t| norm(&t.form, level)).collect();
            for (i, t) in s.tokens.iter().enumerate() {
                if m.lex.get(&keys[i]).is_none_or(|x| x.len() < 2) {
                    continue;
                }
                let Some(g) = gold(t, level) else { continue };
                for f in neighbours(&keys, i) {
                    add(&mut m.ctx, &format!("{} {f}", keys[i]), &label(&g.all()));
                }
            }
        }
        (m, bad)
    }

    /// Is a chain (prefixes or suffixes, as letters) legal: seen at least `chain` times? The empty chain always is;
    /// with no chain tables (an old model) every chain is.
    fn legal(&self, table: &BTreeMap<String, u32>, c: &str) -> bool {
        self.opts.chain == 0 || c.is_empty() || table.is_empty() || table.get(c).is_some_and(|&n| n >= self.opts.chain)
    }

    /// The labels of a distribution that `ok` accepts, commonest first (the smaller label on a tie), with the total.
    fn ranked(m: &BTreeMap<String, u32>, ok: impl Fn(&str) -> bool) -> (Vec<(String, u32)>, u32) {
        let mut v: Vec<(String, u32)> = m.iter().filter(|(l, _)| ok(l)).map(|(l, n)| (l.clone(), *n)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        (v, m.values().sum())
    }

    /// The longest context (k units) seen at least `min` times with a label that fits in `room` letters: its context,
    /// its acceptable labels (commonest first) and its total. Without the chain check, the context's commonest label
    /// itself must fit, as it always did.
    fn by_context(&self, table: &Counts, ctxs: &[String], room: usize, ok: impl Fn(&str) -> bool) -> Option<(String, Vec<(String, u32)>, u32)> {
        for c in ctxs.iter().rev() {
            if let Some(m) = table.get(c) {
                let fits = |l: &str| letters(&parse_label(l)) < room;
                let (v, total) = Self::ranked(m, |l| fits(l) && ok(l));
                let accept = if self.opts.chain == 0 { fits(&top(m).0) } else { !v.is_empty() };
                if total >= self.opts.min && accept && !v.is_empty() {
                    return Some((c.clone(), v, total));
                }
            }
        }
        None
    }

    /// The lexicon's splits of a form, commonest first; a partly pointed form not in the lexicon (with `partial`)
    /// takes the splits of the lexicon's forms whose points include its own.
    fn lex_dist(&self, key: &str) -> Option<(BTreeMap<String, u32>, &'static str)> {
        if let Some(m) = self.lex.get(key) {
            return Some((m.clone(), "lex"));
        }
        if !self.opts.partial || self.level != Some(Level::Pointed) {
            return None;
        }
        let mut merged: BTreeMap<String, u32> = BTreeMap::new();
        for k in self.by_letters.get(&norm(key, Level::Unpointed))?.iter().filter(|k| within(key, k)) {
            for (l, n) in &self.lex[k] {
                *merged.entry(l.clone()).or_default() += n;
            }
        }
        (!merged.is_empty()).then_some((merged, "lexp"))
    }

    /// The candidate splits of a written word (any pointing; it is normalized to the model's level), best first. The
    /// first is the split `split` gives: a seen word's commonest split; for an unseen word the prefixes first, then
    /// the suffixes (or, with `joint`, the best product of the two shares). The rest follow by share.
    pub fn raw_candidates(&self, form: &str) -> Vec<Cand> {
        let level = self.level.unwrap_or(Level::Pointed);
        let key = norm(form, level);
        let u: Vec<&str> = units(&key);
        let n = u.len();
        let fits = |ps: &[Piece]| letters(ps) == n && ps.iter().all(|p| !matches!(p, Piece::Cut(0, _)));
        if self.opts.lex {
            if let Some((m, rule)) = self.lex_dist(&key) {
                let (v, t) = Self::ranked(&m, |l| fits(&parse_label(l)));
                if v.first().is_some_and(|(l, _)| *l == top(&m).0) {
                    return v.into_iter().map(|(l, c)| Cand { ps: parse_label(&l), p: c as f64 / t as f64, rule, why: format!("{rule}:{c}/{t}"), h: 0 }).collect();
                }
            }
        }
        if let Some(fb) = self.fallback.as_ref().filter(|_| self.opts.partial && self.level == Some(Level::Pointed) && thinly(&key)) {
            return fb.raw_candidates(form).into_iter().map(|mut c| {
                c.why = format!("unpointed:{}", c.why);
                c
            }).collect();
        }
        let bare = |s: &[&str]| s.iter().map(|x| norm(x, Level::Unpointed)).collect::<String>();
        // the letters of a label's pieces over some units, as a chain (ו.ב.H)
        let chain = |us: &[&str], l: &str| -> String {
            let mut at = 0;
            parse_label(l).iter().map(|p| match *p {
                Piece::H => "H".to_string(),
                Piece::Cut(k, _) => {
                    let s = bare(&us[at.min(us.len())..(at + k).min(us.len())]);
                    at += k;
                    s
                }
            }).collect::<Vec<_>>().join(".")
        };
        let firsts: Vec<String> = (1..=self.opts.max_k.min(n)).map(|k| u[..k].concat()).collect();
        let pres: Vec<(Vec<Piece>, f64, String)> = match self.by_context(&self.pre, &firsts, n, |l| self.legal(&self.chains, &chain(&u, l))) {
            Some((c, v, t)) => v.into_iter().map(|(l, k)| (parse_label(&l), k as f64 / t as f64, format!("pre:{c}→{}:{k}/{t}", if l.is_empty() { "-" } else { &l }))).collect(),
            None => vec![(Vec::new(), 1.0, "pre:none".to_string())],
        };
        let mut out: Vec<Cand> = Vec::new();
        for (pre, pp, pwhy) in pres {
            let p = letters(&pre);
            let r = &u[p..];
            let rest: String = r.concat();
            let sok = |l: &str| {
                let ps = parse_label(l);
                self.legal(&self.sufs, &chain(&r[r.len() - letters(&ps).min(r.len())..], l))
            };
            let from_rest = self.rest.get(&rest).filter(|_| self.opts.rest).and_then(|m| {
                let (v, t) = Self::ranked(m, |l| letters(&parse_label(l)) < r.len() && sok(l));
                let accept = if self.opts.chain == 0 { letters(&parse_label(&top(m).0)) < r.len() } else { !v.is_empty() };
                (accept && !v.is_empty()).then(|| v.into_iter().map(|(l, k)| (parse_label(&l), k as f64 / t as f64, format!(" rest:{rest}→{}:{k}/{t}", if l.is_empty() { "-" } else { &l }))).collect::<Vec<_>>())
            });
            let sufs = from_rest.unwrap_or_else(|| {
                let lasts: Vec<String> = (1..=self.opts.max_k.min(r.len())).map(|k| r[r.len() - k..].concat()).collect();
                match self.by_context(&self.suf, &lasts, r.len(), sok) {
                    Some((c, v, t)) => v.into_iter().map(|(l, k)| (parse_label(&l), k as f64 / t as f64, format!(" suf:{c}→{}:{k}/{t}", if l.is_empty() { "-" } else { &l }))).collect(),
                    None => vec![(Vec::new(), 1.0, " suf:none".to_string())],
                }
            });
            for (suf, sp, swhy) in sufs {
                let mut ps = pre.clone();
                ps.push(Piece::Cut(r.len() - letters(&suf), false));
                ps.extend(suf);
                out.push(Cand { ps, p: pp * sp, rule: "rules", why: format!("{pwhy}{swhy}"), h: pre.len() });
            }
        }
        // the hosts of training (Rubinstein 2021): keep the candidates whose host was seen as one, if any is
        let hostlex = self.opts.hostlex.filter(|_| !self.hosts.is_empty());
        if let Some(b) = hostlex {
            let known = |c: &Cand| parts(&key, &c.ps, c.h).is_some_and(|(_, h, _)| self.hosts.contains_key(&h));
            if b == 0.0 {
                if out.iter().any(known) {
                    out.retain(known);
                }
            } else {
                for c in out.iter_mut() {
                    if !known(c) {
                        c.p *= b;
                        c.why.push_str(&format!(",hostlex:×{b}"));
                    }
                }
            }
        }
        // the first stays the greedy choice (prefixes first) unless `joint` or `hostlex`; the others by share, stable
        let first = if self.opts.joint || hostlex.is_some() || out.is_empty() { None } else { Some(out.remove(0)) };
        out.sort_by(|a, b| b.p.partial_cmp(&a.p).unwrap_or(std::cmp::Ordering::Equal));
        if let Some(f) = first {
            out.insert(0, f);
        }
        out
    }

    /// The candidates that pass the consonant check (`spells`), best first; when none does, the word unsplit (rule
    /// `check`). With `no-check` the candidates are left as they come.
    pub fn candidates(&self, form: &str) -> Vec<Cand> {
        let mut c = self.raw_candidates(form);
        if self.opts.check {
            let key = norm(form, self.level.unwrap_or(Level::Pointed));
            c.retain(|x| Self::spells(&key, &x.ps));
            if c.is_empty() {
                c.push(Cand { ps: vec![Piece::Cut(units(&key).len(), false)], p: 1.0, rule: "check", why: "check:none-spells".into(), h: 0 });
            }
        }
        c
    }
    /// The candidates of the i-th written word of a sentence (`keys`: the sentence's words at the model's level). With
    /// `ctx:K`, a word the lexicon splits more than one way has each split's share replaced, first by the word before
    /// and then by the word after, with q = (c(neighbour, split) + K · share) / (c(neighbour) + K): the share so far
    /// smoothed toward that neighbour's counts. The splits are then re-ranked.
    pub fn candidates_in(&self, keys: &[String], i: usize) -> Vec<Cand> {
        let mut c = self.candidates(&keys[i]);
        let Some(k) = self.opts.ctx else { return c };
        if c.len() < 2 || c[0].rule != "lex" {
            return c;
        }
        for f in neighbours(keys, i) {
            let Some(m) = self.ctx.get(&format!("{} {f}", keys[i])) else { continue };
            let total: u32 = m.values().sum();
            for x in c.iter_mut() {
                let n = m.get(&label(&x.ps)).copied().unwrap_or(0);
                let q = (n as f64 + k * x.p) / (total as f64 + k);
                if (q - x.p).abs() > 1e-12 {
                    x.why.push_str(&format!(",{f}:{n}/{total}"));
                }
                x.p = q;
            }
        }
        c.sort_by(|a, b| b.p.partial_cmp(&a.p).unwrap_or(std::cmp::Ordering::Equal));
        c
    }

    /// The split of the i-th written word of a sentence: its first candidate in context.
    pub fn split_in(&self, keys: &[String], i: usize) -> (Vec<Piece>, Why) {
        let c = self.candidates_in(keys, i).into_iter().next().expect("a word has a candidate");
        (c.ps, Why { rule: c.rule, text: c.why })
    }

    /// The words of a sentence at the model's level, for `candidates_in`.
    pub fn keys(&self, forms: &[&str]) -> Vec<String> {
        forms.iter().map(|f| norm(f, self.level.unwrap_or(Level::Pointed))).collect()
    }

    /// The split of a written word (any pointing; it is normalized to the model's level): the first candidate.
    pub fn split(&self, form: &str) -> (Vec<Piece>, Why) {
        let c = self.candidates(form).into_iter().next().expect("a word has a candidate");
        (c.ps, Why { rule: c.rule, text: c.why })
    }

    /// Cut a written form into the forms of its words (an `H` piece is the swallowed article, written ה; a piece that
    /// leads takes the vowels of the letter before it).
    pub fn cut(form: &str, ps: &[Piece]) -> Vec<(String, bool)> {
        let u = units(form);
        let mut at = 0;
        let mut out: Vec<(String, bool)> = Vec::new();
        for p in ps {
            match *p {
                Piece::H => out.push(("ה".to_string(), true)),
                Piece::Cut(n, lead) => {
                    let mut f = String::new();
                    if lead {
                        // the vowels of the last letter written before, moved here
                        if let Some(prev) = out.iter_mut().rev().find(|x| !x.1) {
                            f = prev.0.chars().rev().take_while(|&c| crate::text::combining(c)).filter(|&c| vowel(c)).collect::<Vec<_>>().into_iter().rev().collect();
                            let keep: Vec<char> = {
                                let cs: Vec<char> = prev.0.chars().collect();
                                let tail = cs.iter().rev().take_while(|&&c| crate::text::combining(c)).count();
                                let mut k: Vec<char> = cs[..cs.len() - tail].to_vec();
                                k.extend(cs[cs.len() - tail..].iter().filter(|&&c| !vowel(c)));
                                k
                            };
                            prev.0 = keep.into_iter().collect();
                        }
                    }
                    f.push_str(&u[at..(at + n).min(u.len())].concat());
                    out.push((f, false));
                    at += n;
                }
            }
        }
        out.into_iter().map(|(f, h)| (f.nfc().collect(), h)).collect()
    }

    /// The consonant check (Naaijer 2023): the words of a split, the swallowed article left out, spell the written
    /// word's letters back exactly.
    pub fn spells(form: &str, ps: &[Piece]) -> bool {
        let n = units(form).len();
        if letters(ps) != n || ps.iter().any(|p| matches!(p, Piece::Cut(0, _))) {
            return false;
        }
        let back: String = Model::cut(form, ps).into_iter().filter(|x| !x.1).map(|x| x.0).collect();
        norm(&back, Level::Unpointed) == norm(form, Level::Unpointed)
    }

    pub fn save(&self) -> String {
        let o = &self.opts;
        let mut s = format!("# hbo segmenter\tlevel={}\tmax_k={}\tmin={}\trare={}", self.level.unwrap_or(Level::Pointed).name(), o.max_k, o.min, o.rare);
        let with = o.with();
        if !with.is_empty() {
            write!(s, "\twith={with}").unwrap();
        }
        s.push('\n');
        for (name, t) in [("lex", &self.lex), ("rest", &self.rest), ("pre", &self.pre), ("suf", &self.suf), ("ctx", &self.ctx)] {
            for (k, m) in t {
                for (l, n) in m {
                    writeln!(s, "{name}\t{k}\t{l}\t{n}").unwrap();
                }
            }
        }
        for (name, t) in [("chain", &self.chains), ("sufx", &self.sufs), ("host", &self.hosts)] {
            for (k, n) in t {
                writeln!(s, "{name}\t{k}\t-\t{n}").unwrap();
            }
        }
        s
    }

    /// The lexicon's forms by their letters (for `partial`).
    fn index(&mut self) {
        self.by_letters.clear();
        for k in self.lex.keys() {
            self.by_letters.entry(norm(k, Level::Unpointed)).or_default().push(k.clone());
        }
    }

    pub fn load(src: &str) -> Model {
        let mut m = Model::default();
        for l in src.lines() {
            let c: Vec<&str> = l.split('\t').collect();
            if l.starts_with('#') {
                for kv in &c[1..] {
                    match kv.split_once('=') {
                        Some(("level", v)) => m.level = Level::parse(v),
                        Some(("max_k", v)) => m.opts.max_k = v.parse().unwrap_or(m.opts.max_k),
                        Some(("min", v)) => m.opts.min = v.parse().unwrap_or(m.opts.min),
                        Some(("rare", v)) => m.opts.rare = v.parse().unwrap_or(m.opts.rare),
                        Some(("with", v)) => m.opts.set_with(v),
                        _ => {}
                    }
                }
                continue;
            }
            if c.len() != 4 {
                continue;
            }
            let n: u32 = c[3].parse().unwrap_or(0);
            let one = match c[0] {
                "chain" => Some(&mut m.chains),
                "sufx" => Some(&mut m.sufs),
                "host" => Some(&mut m.hosts),
                _ => None,
            };
            if let Some(t) = one {
                *t.entry(c[1].to_string()).or_default() += n;
                continue;
            }
            let t = match c[0] {
                "lex" => &mut m.lex,
                "rest" => &mut m.rest,
                "pre" => &mut m.pre,
                "suf" => &mut m.suf,
                "ctx" => &mut m.ctx,
                _ => continue,
            };
            *t.entry(c[1].to_string()).or_default().entry(c[2].to_string()).or_default() += n;
        }
        m.index();
        m
    }
}

/// The measure of a segmenter on gold trees.
#[derive(Default, Debug)]
pub struct Score {
    pub tokens: usize,
    pub right: usize,
    pub oov: usize,
    pub oov_right: usize,
    pub multi: usize,
    pub multi_right: usize,
    pub gold_words: usize,
    pub pred_words: usize,
    pub both_words: usize,
    /// per rule: (tokens, right)
    pub by_rule: BTreeMap<&'static str, (usize, usize)>,
    /// (gold label, predicted label) of the errors
    pub confusions: BTreeMap<(String, String), usize>,
    pub unspelled: usize,
    /// the errors split (Seker 2020): the gold split not among the candidates at all, or among them but not chosen
    pub missing: usize,
    pub unchosen: usize,
    /// tokens whose gold split is among the first k candidates, k = 1 … 5
    pub oracle: [usize; 5],
    /// tokens whose first candidate, before the consonant check, does not spell the written word
    pub check_failed: usize,
}

/// Word spans of a split: (start letter, letters, piece), the piece 0 plain, 1 leading, 2 the swallowed article.
fn spans(ps: &[Piece]) -> Vec<(usize, usize, u8)> {
    let mut at = 0;
    ps.iter().map(|p| match p {
        Piece::H => (at, 0, 2),
        Piece::Cut(n, lead) => {
            at += n;
            (at - n, *n, *lead as u8)
        }
    }).collect()
}

pub fn evaluate(m: &Model, sents: &[Sent]) -> Score {
    evaluate_thinned(m, sents, None)
}

/// The measure on partly pointed text: each token's points dropped with the share `thin` (`text::thin`); the gold
/// split is read from the full pointing, and a split counts as right when its pieces cut the same letters (a suffix
/// that takes the vowel before it is not told apart, as the vowel may be gone).
pub fn evaluate_thinned(m: &Model, sents: &[Sent], thin: Option<f64>) -> Score {
    let level = m.level.unwrap_or(Level::Pointed);
    let plain = |ps: &[Piece]| ps.iter().map(|p| if let Piece::Cut(n, _) = p { Piece::Cut(*n, false) } else { *p }).collect::<Vec<_>>();
    let mut s = Score::default();
    let mut at = 0;
    // every token with its sentence's keys and its place in it
    let mut toks: Vec<(&Token, std::rc::Rc<Vec<String>>, usize)> = Vec::new();
    for x in sents {
        let keys: Vec<String> = x.tokens.iter().enumerate().map(|(j, t)| norm(&thin.map(|r| crate::text::thin(&t.form, r, (at + j) as u64)).unwrap_or_else(|| t.form.clone()), level)).collect();
        at += x.tokens.len();
        let keys = std::rc::Rc::new(keys);
        toks.extend(x.tokens.iter().enumerate().map(|(j, t)| (t, keys.clone(), j)));
    }
    for (t, keys, j) in toks {
        if t.words.iter().all(|w| w.upos() == "PUNCT") {
            continue;
        }
        let Some(g) = gold(t, if thin.is_some() { Level::Pointed } else { level }) else {
            s.unspelled += 1;
            continue;
        };
        let (want, form) = match thin {
            Some(_) => (plain(&g.all()), keys[j].clone()),
            None => (g.all(), t.form.clone()),
        };
        let mut cands = m.candidates_in(&keys, j);
        if thin.is_some() {
            for c in cands.iter_mut() {
                c.ps = plain(&c.ps);
            }
        }
        let t = &Token { form, ..t.clone() };
        let (got, why) = (cands[0].ps.clone(), Why { rule: cands[0].rule, text: String::new() });
        let ok = want == got;
        let rank = cands.iter().position(|c| c.ps == want);
        s.check_failed += m.raw_candidates(&t.form).first().is_none_or(|c| !Model::spells(&norm(&t.form, level), &c.ps)) as usize;
        for k in 0..5 {
            s.oracle[k] += rank.is_some_and(|r| r <= k) as usize;
        }
        if !ok {
            if rank.is_some() {
                s.unchosen += 1;
            } else {
                s.missing += 1;
            }
        }
        s.tokens += 1;
        s.right += ok as usize;
        if !m.lex.contains_key(&norm(&t.form, level)) {
            s.oov += 1;
            s.oov_right += ok as usize;
        }
        if want.len() > 1 {
            s.multi += 1;
            s.multi_right += ok as usize;
        }
        let e = s.by_rule.entry(why.rule).or_default();
        e.0 += 1;
        e.1 += ok as usize;
        let (a, b) = (spans(&want), spans(&got));
        s.gold_words += a.len();
        s.pred_words += b.len();
        s.both_words += a.iter().filter(|x| b.contains(x)).count();
        if !ok {
            *s.confusions.entry((label(&want), label(&got))).or_default() += 1;
        }
    }
    s
}

impl Score {
    /// Another score's counts added to this one.
    pub fn add(&mut self, o: &Score) {
        self.tokens += o.tokens;
        self.right += o.right;
        self.oov += o.oov;
        self.oov_right += o.oov_right;
        self.multi += o.multi;
        self.multi_right += o.multi_right;
        self.gold_words += o.gold_words;
        self.pred_words += o.pred_words;
        self.both_words += o.both_words;
        self.unspelled += o.unspelled;
        self.missing += o.missing;
        self.unchosen += o.unchosen;
        for k in 0..5 {
            self.oracle[k] += o.oracle[k];
        }
        self.check_failed += o.check_failed;
        for (r, (n, k)) in &o.by_rule {
            let e = self.by_rule.entry(r).or_default();
            e.0 += n;
            e.1 += k;
        }
        for (c, n) in &o.confusions {
            *self.confusions.entry(c.clone()).or_default() += n;
        }
    }

    pub fn report(&self) -> String {
        let pct = |a: usize, b: usize| 100. * a as f64 / b.max(1) as f64;
        let p = pct(self.both_words, self.pred_words);
        let r = pct(self.both_words, self.gold_words);
        let mut out = format!(
            "tokens {}: split right {:.2}% | unseen {} ({:.2}% of tokens): {:.2}% | of more than one word {}: {:.2}% | words P {:.2} R {:.2} F1 {:.2} | not spelled by their words {}\n",
            self.tokens, pct(self.right, self.tokens), self.oov, pct(self.oov, self.tokens), pct(self.oov_right, self.oov), self.multi, pct(self.multi_right, self.multi), p, r, 2. * p * r / (p + r).max(1e-9), self.unspelled
        );
        writeln!(out, "  errors: gold split not among the candidates {}, among them but not chosen {} | gold within the first 1/2/3/5 candidates: {:.2} {:.2} {:.2} {:.2}",
            self.missing, self.unchosen, pct(self.oracle[0], self.tokens), pct(self.oracle[1], self.tokens), pct(self.oracle[2], self.tokens), pct(self.oracle[4], self.tokens)).unwrap();
        writeln!(out, "  consonant check: {} tokens whose first candidate does not spell the word", self.check_failed).unwrap();
        for (rule, (n, k)) in &self.by_rule {
            writeln!(out, "  {rule}: {n} tokens, {:.2}% right", pct(*k, *n)).unwrap();
        }
        let mut c: Vec<_> = self.confusions.iter().collect();
        c.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        out += "  errors (gold → ours): ";
        out += &c.iter().take(12).map(|((g, p), n)| format!("{g}→{p} {n}")).collect::<Vec<_>>().join(", ");
        out.push('\n');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conllu::read;

    const TREES: &str = "1-3\tבָּאָרֶץ\t_\t_\t_\t_\t_\t_\t_\t_\n1\tבָּ\t_\tADP\t_\t_\t3\tcase\t_\tMId=o1\n2\tה\t_\tDET\t_\t_\t3\tdet\t_\tMId=o1ה\n3\tאָרֶץ\t_\tNOUN\t_\t_\t0\troot\t_\tMId=o2\n\n\
1-4\tוּבְבֵיתוֹ\t_\t_\t_\t_\t_\t_\t_\t_\n1\tוּ\t_\tCCONJ\t_\t_\t3\tcc\t_\t_\n2\tבְ\t_\tADP\t_\t_\t3\tcase\t_\t_\n3\tבֵית\t_\tNOUN\t_\t_\t0\troot\t_\t_\n4\tוֹ\t_\tPRON\t_\t_\t3\tnmod:poss\t_\tType=pronominal\n\n";

    #[test]
    fn gold_splits() {
        let s = read(TREES);
        let g = gold(&s[0].tokens[0], Level::Pointed).unwrap();
        assert_eq!(label(&g.all()), "1.H.3");
        assert_eq!((label(&g.pre), g.host), ("1.H".to_string(), Piece::Cut(3, false)));
        let g = gold(&s[1].tokens[0], Level::Pointed).unwrap();
        assert_eq!((label(&g.pre), g.host, label(&g.suf)), ("1.1".to_string(), Piece::Cut(3, false), "1".to_string()));
        // a suffix that takes the vowel of the host's last letter («all of them»: כֻּלּ + ָם, the dagesh stays)
        let t = &read("1-2\tכֻּלָּם\t_\t_\t_\t_\t_\t_\t_\t_\n1\tכֻּלּ\t_\tNOUN\t_\t_\t0\troot\t_\t_\n2\tָם\t_\tPRON\t_\t_\t1\tnmod:poss\t_\tType=pronominal\n\n")[0].tokens[0];
        let g = gold(t, Level::Pointed).unwrap();
        assert_eq!(label(&g.all()), "2.^1");
        assert_eq!(Model::cut("כֻּלָּם", &g.all()).into_iter().map(|x| x.0).collect::<Vec<_>>(), ["כֻּלּ", "ָם"]);
        assert_eq!(label(&gold(t, Level::Unpointed).unwrap().all()), "2.1");
        // negative control: a token whose words do not spell it has no gold split
        let mut t = s[1].tokens[0].clone();
        t.form = "וּבְבֵיתָהּ".into();
        assert_eq!(gold(&t, Level::Pointed), None);
    }

    #[test]
    fn learned_rules_split_an_unseen_word() {
        // training: וּ + בְ prefixes and וֹ suffixes, each seen in rare words
        let mut src = String::new();
        for host in ["בַיִת", "דָבָר", "שָׂדֶה", "כֶרֶם"] {
            src += &format!("1-4\tוּבְ{host}וֹ\t_\t_\t_\t_\t_\t_\t_\t_\n1\tוּ\t_\tCCONJ\t_\t_\t3\tcc\t_\t_\n2\tבְ\t_\tADP\t_\t_\t3\tcase\t_\t_\n3\t{host}\t_\tNOUN\t_\t_\t0\troot\t_\t_\n4\tוֹ\t_\tPRON\t_\t_\t3\tnmod:poss\t_\tType=pronominal\n\n");
        }
        let train = read(&src);
        let opts = Opts { min: 2, ..Opts::default() };
        let (m, bad) = Model::learn(&train, Level::Pointed, opts);
        assert_eq!(bad, 0);
        let (ps, why) = m.split("וּבְהַרוֹ");
        assert_eq!(label(&ps), "1.1.2.1", "{why:?}");
        assert_eq!(why.rule, "rules");
        assert_eq!(Model::cut("וּבְהַרוֹ", &ps).iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), ["וּ", "בְ", "הַר", "וֹ"]);
        // a seen word comes from the lexicon, accents or not
        assert_eq!(m.split("וּבְדָבָ֖רוֹ").1.rule, "lex");
        // saved and loaded, the model splits the same
        let m2 = Model::load(&m.save());
        assert_eq!(m2.split("וּבְהַרוֹ").0, ps);
        // negative control: with the rules' tables empty, nothing is split and the measure shows red
        let empty = Model { level: Some(Level::Pointed), opts: Opts { min: 2, ..Opts::default() }, ..Default::default() };
        assert_eq!(label(&empty.split("וּבְהַרוֹ").0), "5");
        let test = read("1-4\tוּבְהַרוֹ\t_\t_\t_\t_\t_\t_\t_\t_\n1\tוּ\t_\tCCONJ\t_\t_\t3\tcc\t_\t_\n2\tבְ\t_\tADP\t_\t_\t3\tcase\t_\t_\n3\tהַר\t_\tNOUN\t_\t_\t0\troot\t_\t_\n4\tוֹ\t_\tPRON\t_\t_\t3\tnmod:poss\t_\tType=pronominal\n\n");
        assert_eq!(evaluate(&m2, &test).right, 1);
        assert_eq!(evaluate(&empty, &test).right, 0);
    }

    #[test]
    fn prefix_chain_filter() {
        // training: only וּ + בְ before a host; the one-letter context וּ learns «two prefixes»
        let mut src = String::new();
        for host in ["בַיִת", "דָבָר", "שָׂדֶה"] {
            src += &format!("1-3\tוּבְ{host}\t_\t_\t_\t_\t_\t_\t_\t_\n1\tוּ\t_\tCCONJ\t_\t_\t3\tcc\t_\t_\n2\tבְ\t_\tADP\t_\t_\t3\tcase\t_\t_\n3\t{host}\t_\tNOUN\t_\t_\t0\troot\t_\t_\n\n");
        }
        let (mut m, _) = Model::learn(&read(&src), Level::Pointed, Opts { min: 2, max_k: 1, ..Opts::default() });
        assert_eq!(m.chains.get("ו.ב"), Some(&3));
        // negative control: without the check, וּ + תְ is taken for two prefixes
        assert_eq!(label(&m.split("וּתְהִי").0), "1.1.2");
        // with it, ו.ת was never a chain: the context gives nothing legal and the word keeps its letters whole
        m.opts.set_with("chain:1");
        assert_eq!(label(&m.split("וּתְהִי").0), "4");
        assert_eq!(label(&m.split("וּבְהַר").0), "1.1.2");
    }

    #[test]
    fn host_lexicon_and_partial_points() {
        let w = |form: &str, ws: &[(&str, &str)], misc: &str| {
            let mut s = format!("1-{}\t{form}\t_\t_\t_\t_\t_\t_\t_\t_\n", ws.len());
            for (i, (f, u)) in ws.iter().enumerate() {
                s += &format!("{}\t{f}\t_\t{u}\t_\t_\t0\troot\t_\t{}\n", i + 1, if *u == "PRON" { misc } else { "_" });
            }
            s + "\n"
        };
        // the rare words ending in תו: twice whole, once a host + the suffix ו; the host שבת seen alone
        let mut src = String::new();
        src += &w("כתבתו", &[("כתבת", "NOUN"), ("ו", "PRON")], "Type=pronominal");
        src += "1\tקרתו\t_\tNOUN\t_\t_\t0\troot\t_\t_\n\n1\tמרתו\t_\tNOUN\t_\t_\t0\troot\t_\t_\n\n1\tשבת\t_\tNOUN\t_\t_\t0\troot\t_\t_\n\n";
        let (mut m, _) = Model::learn(&read(&src), Level::Unpointed, Opts { min: 1, max_k: 2, ..Opts::default() });
        // negative control: by the suffix table alone, an unseen word ending in תו is whole
        assert_eq!(label(&m.split("שבתו").0), "4");
        // with the host check, the split whose host was seen wins (hard, or soft enough)…
        m.opts.set_with("hostlex");
        assert_eq!(label(&m.split("שבתו").0), "3.1");
        m.opts.set_with("hostlex:0.3");
        assert_eq!(label(&m.split("שבתו").0), "3.1");
        // …not when the check is mild, nor when no host is known (זבת)
        m.opts.set_with("hostlex:0.9");
        assert_eq!(label(&m.split("שבתו").0), "4");
        m.opts.set_with("hostlex");
        assert_eq!(label(&m.split("זבתו").0), "4");
        // a partly pointed word: the lexicon's pointed forms whose points include its own
        let (mut p, _) = Model::learn(&read("1-2\tבְּבֵיתוֹ\t_\t_\t_\t_\t_\t_\t_\t_\n1\tבְּ\t_\tADP\t_\t_\t2\tcase\t_\t_\n2\tבֵיתוֹ\t_\tNOUN\t_\t_\t0\troot\t_\t_\n\n"), Level::Pointed, Opts::default());
        assert!(within("בְּבֵית", "בְּבֵית") && within("בבֵית", "בְּבֵית") && !within("בַּבֵית", "בְּבֵית"));
        assert_eq!(p.split("בבֵיתוֹ").1.rule, "rules");
        p.opts.set_with("partial");
        let (ps, why) = p.split("בבֵיתוֹ");
        assert_eq!((label(&ps), why.rule), ("1.4".to_string(), "lexp"));
        // a word pointed in full is not thin; one with a bare letter inside is (ר before וּ is not bare)
        assert!(!thinly("בְּבֵיתוֹ") && !thinly("רוּת") && thinly("בבֵיתוֹ"));
        // thinning is deterministic and drops only points
        let t = crate::text::thin("בְּבֵיתוֹ", 0.5, 7);
        assert_eq!(t, crate::text::thin("בְּבֵיתוֹ", 0.5, 7));
        assert_eq!(norm(&t, Level::Unpointed), "בביתו");
        assert_eq!(crate::text::thin("בְּבֵיתוֹ", 1.0, 7), "בביתו");
    }

    #[test]
    fn neighbours_choose_among_the_lexicon_splits() {
        let sent = |before: &str, def: bool| {
            let mut s = format!("1\t{before}\t_\tVERB\t_\t_\t0\troot\t_\t_\n");
            if def {
                s += "2-4\tבבית\t_\t_\t_\t_\t_\t_\t_\t_\n2\tב\t_\tADP\t_\t_\t4\tcase\t_\t_\n3\tה\t_\tDET\t_\t_\t4\tdet\t_\tMId=xה\n4\tבית\t_\tNOUN\t_\t_\t1\tobl\t_\t_\n\n";
            } else {
                s += "2-3\tבבית\t_\t_\t_\t_\t_\t_\t_\t_\n2\tב\t_\tADP\t_\t_\t3\tcase\t_\t_\n3\tבית\t_\tNOUN\t_\t_\t1\tobl\t_\t_\n\n";
            }
            s
        };
        let src = format!("{}{}{}{}{}", sent("ישב", false), sent("ישב", false), sent("ישב", false), sent("בא", true), sent("בא", true));
        let (mut m, _) = Model::learn(&read(&src), Level::Unpointed, Opts { min: 1, ..Opts::default() });
        // a word after it that training never saw, so only the word before speaks
        let keys = |w: &str| vec![w.to_string(), "בבית".to_string(), "זזז".to_string()];
        // negative control: without the neighbours the commoner split wins everywhere
        assert_eq!(label(&m.split_in(&keys("בא"), 1).0), "1.3");
        m.opts.set_with("ctx:0.5");
        let (ps, why) = m.split_in(&keys("בא"), 1);
        assert_eq!(label(&ps), "1.H.3");
        assert!(why.text.contains("p:בא:2/2"), "{}", why.text);
        assert_eq!(label(&m.split_in(&keys("ישב"), 1).0), "1.3");
        // a neighbour never seen leaves the lexicon's choice
        assert_eq!(label(&m.split_in(&keys("הלך"), 1).0), "1.3");
        // saved and loaded, the same
        let m2 = Model::load(&m.save());
        assert_eq!(m2.opts.ctx, Some(0.5));
        assert_eq!(label(&m2.split_in(&keys("בא"), 1).0), "1.H.3");
    }

    #[test]
    fn consonant_check() {
        let p = |s: &str| parse_label(s);
        assert!(Model::spells("וּבְהַרוֹ", &p("1.1.2.1")));
        assert!(Model::spells("בָּאָרֶץ", &p("1.H.3")));
        assert!(Model::spells("כֻּלָּם", &p("2.^1")));
        // negative control: a split that loses or invents a letter fails
        assert!(!Model::spells("וּבְהַרוֹ", &p("1.1.2")));
        assert!(!Model::spells("וּבְהַרוֹ", &p("1.1.2.2")));
        assert!(!Model::spells("וּבְהַרוֹ", &p("1.0.4")));
        // a word nothing can split gets the unsplit fallback, which spells it
        let m = Model { level: Some(Level::Pointed), ..Default::default() };
        let c = m.candidates("הַר");
        assert!(c.len() == 1 && Model::spells("הַר", &c[0].ps));
    }

    #[test]
    fn errors_split_into_missing_and_unchosen() {
        // a seen word with two splits: the commoner is chosen, the other is still a candidate
        let tok = |w: &str, ps: &[(&str, &str)]| {
            let mut s = format!("1-{}\t{w}\t_\t_\t_\t_\t_\t_\t_\t_\n", ps.len());
            for (i, (f, u)) in ps.iter().enumerate() {
                s += &format!("{}\t{f}\t_\t{u}\t_\t_\t0\troot\t_\t_\n", i + 1);
            }
            s + "\n"
        };
        let one = tok("בבית", &[("ב", "ADP"), ("בית", "NOUN")]);
        let two = tok("בבית", &[("ב", "ADP"), ("ה", "DET"), ("בית", "NOUN")]).replace("\tDET\t_\t_\t0\troot\t_\t_", "\tDET\t_\t_\t0\troot\t_\tMId=xה");
        let train = read(&format!("{one}{one}{two}"));
        let (m, _) = Model::learn(&train, Level::Unpointed, Opts { min: 1, ..Opts::default() });
        let c = m.candidates("בבית");
        assert_eq!(c.iter().map(|x| label(&x.ps)).collect::<Vec<_>>(), ["1.3", "1.H.3"]);
        assert_eq!(c[0].ps, m.split("בבית").0);
        let s = evaluate(&m, &read(&two));
        assert_eq!((s.right, s.unchosen, s.missing, s.oracle[0], s.oracle[1]), (0, 1, 0, 0, 1));
        // negative control: a model that never saw the split has it missing, not unchosen
        let (m1, _) = Model::learn(&read(&one), Level::Unpointed, Opts { min: 1, ..Opts::default() });
        let s = evaluate(&m1, &read(&two));
        assert_eq!((s.unchosen, s.missing, s.oracle[4]), (0, 1, 0));
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn supplied_states_context_fallback_and_codec() {
        let src = "# hbo segmenter\tlevel=unpointed\twith=ctx:1\nlex\tאב\t1.1\t2\nlex\tאב\t2\t2\nctx\tאב p:<s>\t2\t3\n";
        let m = Model::load(src);
        assert_eq!(label(&m.split("אב").0), "1.1");
        let keys = m.keys(&["אב"]);
        let c = m.candidates_in(&keys, 0);
        assert_eq!(label(&c[0].ps), "2");
        assert!((c[0].p - 0.875).abs() < 1e-12);
        assert_eq!(Model::load(&m.save()).candidates_in(&keys, 0), c);
        let dup = Model::load("lex\tאב\t2\t1\nlex\tאב\t2\t2\n");
        assert_eq!(dup.lex["אב"]["2"], 3);
        let mut p = Model {
            level: Some(Level::Pointed),
            ..Default::default()
        };
        p.opts.partial = true;
        p.fallback = Some(Box::new(m));
        assert_eq!(label(&p.split("אב").0), "2");
        // Alef is a mater; use a bare non-mater interior letter to enter fallback.
        p.fallback
            .as_mut()
            .unwrap()
            .lex
            .insert("בב".into(), [("1.1".into(), 1)].into());
        assert!(p.split("בב").1.text.starts_with("unpointed:"));
        assert_eq!(label(&p.split("בב").0), "1.1");
        assert!(Model::load(&p.save()).fallback.is_none());
        assert_eq!(
            parse_label("oops.^bad.H"),
            [Piece::Cut(1, false), Piece::Cut(1, true), Piece::H]
        );
        assert_eq!(
            Model::cut("כֻּלָּם", &parse_label("2.^1"))
                .iter()
                .map(|x| x.0.as_str())
                .collect::<Vec<_>>(),
            ["כֻּלּ", "ָם"]
        );
        assert!(!Model::spells("אב", &parse_label("1")));
        assert_eq!(
            Model::default().candidates("")[0].ps,
            [Piece::Cut(0, false)]
        );
        assert!(!Model::spells("", &[Piece::Cut(0, false)]));
        assert!(parts("אב", &parse_label("1.1"), 2).is_none());
        assert_eq!(Score::default().report().contains("NaN"), false);
    }
}
