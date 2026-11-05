//! The encoder from the meta-world into English: the standard encoder of the language, and an author as a small
//! delta over it.
//!
//! **The standard** is learned on all the English we have (every Bible translation's training books, the school
//! lessons, the tales, the Talmud's English): each text is read into the meta-world by the shared reader, and every
//! meta token is seen next to the words written for it. For a concept in its context the standard keeps how often
//! each chunk of words was written: the language's own lexical choice, constructions, function words and marks.
//! Encoding takes the most specific context seen often enough — concept, tag and both neighbours; concept, tag and
//! relation; concept and tag; the concept — and its most frequent chunk. A concept never seen gets the bare form (the
//! concept inflected by `morph` from its tag). The sentence capital is the standard stack's rule, so a chunk is kept
//! in lower case at a sentence start.
//!
//! **An author's delta** is learned only from what the standard gets wrong on that author's training text: from the
//! coarsest context to the finest, an override «in this context write this chunk» is kept when it fixes at least
//! `MIN_GAIN` more of the author's tokens than the prediction before it and holds for at least half the context's
//! tokens. A finer override wins over a coarser one; where the delta says nothing the standard decides. Each
//! override keeps its counts, so every choice says why. «Author» is any style: a translator (JPS 1917, KJV, WEB),
//! a register of the school lessons, the tales.

use std::collections::BTreeMap;

use en::annotate::{Annotator, Word};

use crate::corpus::Corpus;
use crate::bible::VKey;
use crate::meta::{Lexicon, MTok};

pub const LEVELS: usize = 4;
pub const LEVEL_NAMES: [&str; LEVELS] = ["concept, tag and neighbours", "concept, tag and relation", "concept and tag", "concept"];
const MIN: [u32; LEVELS] = [2, 2, 2, 1];
/// an override must fix this many more tokens than the prediction before it
pub const MIN_GAIN: u32 = 3;
/// and hold for this share of its context's tokens
pub const TOLERANCE: f64 = 0.5;

pub fn ctx_keys(m: &[MTok], i: usize) -> [String; LEVELS] {
    let x = &m[i];
    let prev = if i == 0 { "^" } else { m[i - 1].concept.as_str() };
    let next = m.get(i + 1).map_or("$", |y| y.concept.as_str());
    [format!("{}|{}|{prev}|{next}", x.concept, x.tag), format!("{}|{}|{}", x.concept, x.tag, x.rel), format!("{}|{}", x.concept, x.tag), x.concept.clone()]
}

fn terminal(f: &str) -> bool {
    !f.is_empty() && f.chars().all(|c| matches!(c, '.' | '!' | '?'))
}

fn closer(f: &str) -> bool {
    matches!(f, "'" | "\"" | "\u{2019}" | "\u{201d}" | ")")
}

/// Whether token k starts a sentence: the first token, or the first after . ! ? and the closing quotes after them.
pub fn sentence_start(forms: &[String], k: usize) -> bool {
    let mut j = k;
    while j > 0 {
        let f = forms[j - 1].as_str();
        if closer(f) {
            j -= 1;
            continue;
        }
        return terminal(f);
    }
    true
}

fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// The forms with the sentence capital taken off (it is the stack's rule, not the author's).
pub fn decap(ws: &[Word]) -> Vec<String> {
    let forms: Vec<String> = ws.iter().map(|w| w.form.clone()).collect();
    (0..ws.len())
        .map(|k| {
            let f = &forms[k];
            let low = lower_first(f);
            let rest_low = f.chars().skip(1).all(|c| !c.is_uppercase());
            if sentence_start(&forms, k) && *f != low && rest_low && !ws[k].tag.is_proper() && !ws[k].lemma.eq_ignore_ascii_case("i") { low } else { f.clone() }
        })
        .collect()
}

/// The sentence capital on a run of forms.
pub fn recap(forms: &mut [String]) {
    for k in 0..forms.len() {
        if sentence_start(forms, k) {
            forms[k] = en::text::title(&forms[k]);
        }
    }
}

/// The bare form of a meta token: the concept's words, the first inflected from the tag; a name with its capital.
pub fn bare_form(a: &Annotator, x: &MTok) -> Vec<String> {
    let concept = crate::meta::bare_concept(&x.concept);
    if !concept.chars().any(char::is_alphanumeric) {
        return vec![concept.to_string()];
    }
    let words: Vec<&str> = concept.split('_').collect();
    let mut out: Vec<String> = Vec::with_capacity(words.len());
    for (i, w) in words.iter().enumerate() {
        let f = if i == 0 && !w.chars().any(char::is_uppercase) { a.model.morph.inflect(w, x.tag) } else { w.to_string() };
        out.push(if x.tag.is_proper() && i == 0 { en::text::title(&f) } else { f });
    }
    out
}

/// A text unit read into the meta-world, with the chunk written for each meta token.
pub struct Seen {
    pub m: Vec<MTok>,
    pub chunks: Vec<String>,
    /// the tokens the standard learns from (empty: all) — the verse lattice's backbone
    pub keep: Vec<bool>,
}

pub fn seen(lex: &Lexicon, ws: &[Word]) -> Seen {
    let m = lex.read(ws);
    let forms = decap(ws);
    let chunks = m.iter().map(|x| forms[x.span.0..x.span.1].join(" ")).collect();
    Seen { m, chunks, keep: Vec::new() }
}

/// The units of a corpus selected by `keep`, read.
pub fn seen_of(lex: &Lexicon, c: &Corpus, keep: &dyn Fn(VKey) -> bool) -> Vec<Seen> {
    c.verses.iter().filter(|(k, _)| keep(**k)).map(|(_, v)| seen(lex, &v.meta_words())).collect()
}

fn best(t: &BTreeMap<String, u32>) -> (String, u32, u32) {
    let total: u32 = t.values().sum();
    let (b, n) = t.iter().fold((String::new(), 0u32), |acc, (c, &v)| if v > acc.1 { (c.clone(), v) } else { acc });
    (b, n, total)
}

/// The standard encoder of English.
#[derive(Clone, Debug, Default)]
pub struct Standard {
    /// per context, per chunk: how often it was seen, and its weight (its corpus's vote)
    tables: Vec<BTreeMap<String, BTreeMap<String, (u32, f64)>>>,
    pub tokens: u64,
    /// voted per context: a context is used only when at least two corpora wrote it (else the next one back)
    pub voted: bool,
}

impl Standard {
    pub fn train(units: &[&Seen]) -> Standard {
        Self::train_weighted(&[(units.to_vec(), 1.0)])
    }

    /// The standard with a weight per corpus (`--std-vote corpus`: one vote per corpus, each token of a corpus
    /// weighing 1 / its tokens): the most weighty chunk is chosen, a context still needs its counts.
    pub fn train_weighted(corpora: &[(Vec<&Seen>, f64)]) -> Standard {
        let mut s = Standard { tables: vec![BTreeMap::new(); LEVELS], tokens: 0, voted: false };
        for (units, w) in corpora {
            for u in units {
                for i in 0..u.m.len() {
                    if u.keep.get(i) == Some(&false) {
                        continue;
                    }
                    for (l, key) in ctx_keys(&u.m, i).into_iter().enumerate() {
                        let e = s.tables[l].entry(key).or_default().entry(u.chunks[i].clone()).or_default();
                        e.0 += 1;
                        e.1 += w;
                    }
                    s.tokens += 1;
                }
            }
        }
        s
    }

    /// One vote per corpus in every context: each corpus spreads one vote over the chunks it writes there (its
    /// counts over its total); the chunk with most votes is chosen, a context still needs its counts. Three archaic
    /// Bibles then lose *hath* to six modern corpora that write *has*.
    pub fn train_voted(corpora: &[Vec<&Seen>]) -> Standard {
        let mut s = Standard { tables: vec![BTreeMap::new(); LEVELS], tokens: 0, voted: false };
        for units in corpora {
            let mut own: Vec<BTreeMap<String, BTreeMap<String, u32>>> = vec![BTreeMap::new(); LEVELS];
            for u in units {
                for i in 0..u.m.len() {
                    if u.keep.get(i) == Some(&false) {
                        continue;
                    }
                    for (l, key) in ctx_keys(&u.m, i).into_iter().enumerate() {
                        *own[l].entry(key).or_default().entry(u.chunks[i].clone()).or_default() += 1;
                    }
                    s.tokens += 1;
                }
            }
            for (l, tables) in own.into_iter().enumerate() {
                for (key, chunks) in tables {
                    let total: u32 = chunks.values().sum();
                    let g = s.tables[l].entry(key).or_default();
                    for (c, n) in chunks {
                        let e = g.entry(c).or_default();
                        e.0 += n;
                        e.1 += n as f64 / total as f64;
                    }
                }
            }
        }
        s.voted = true;
        s
    }

    /// The standard's chunk for token i, and why.
    pub fn choose(&self, a: &Annotator, m: &[MTok], i: usize) -> (String, String) {
        for (l, key) in ctx_keys(m, i).into_iter().enumerate() {
            if let Some(t) = self.tables[l].get(&key) {
                let total: u32 = t.values().map(|x| x.0).sum();
                let votes: f64 = t.values().map(|x| x.1).sum();
                if total >= MIN[l] && (!self.voted || l == LEVELS - 1 || votes >= 1.999) {
                    // the weightiest chunk; ties to the first in order
                    let (b, (n, _)) = t.iter().fold((String::new(), (0u32, f64::MIN)), |acc, (c, &(n, w))| if w > acc.1.1 { (c.clone(), (n, w)) } else { acc });
                    let w = format!("standard: «{key}» → «{b}» {n} of {total} ({})", LEVEL_NAMES[l]);
                    return (b, w);
                }
            }
        }
        // a clause start with a joining not seen in this context: the word as the standard writes it, the joining's
        // words before it
        if let Some((base, feat)) = m[i].concept.split_once("#j:") {
            let words = feat.split_once(':').map(|x| x.1).unwrap_or("").replace('_', " ");
            let mut mm = m.to_vec();
            mm[i].concept = base.to_string();
            let (c, w) = self.choose(a, &mm, i);
            let c = if words.is_empty() { c } else { format!("{words} {c}") };
            return (c, format!("{w}; joining «{words}»"));
        }
        (bare_form(a, &m[i]).join(" "), format!("{}: bare form", m[i].concept))
    }
}

/// One override of a delta.
#[derive(Clone, Debug)]
pub struct Override {
    pub chunk: String,
    /// tokens it fixes over the prediction before it
    pub gain: u32,
    /// the author's tokens in this context written so, and all of them
    pub n: u32,
    pub total: u32,
    /// what the prediction before it wrote
    pub was: String,
}

/// An author as a delta over the standard.
#[derive(Clone, Debug, Default)]
pub struct Delta {
    pub name: String,
    pub levels: Vec<BTreeMap<String, Override>>,
}

impl Delta {
    pub fn empty(name: &str) -> Delta {
        Delta { name: name.to_string(), levels: vec![BTreeMap::new(); LEVELS] }
    }

    /// Learn an author's delta from what the standard gets wrong on its training text. `shuffle`: a negative
    /// control — each unit's meta-world is paired with another unit's chunks, so the overrides learn nothing true.
    pub fn train(name: &str, std: &Standard, a: &Annotator, units: &[&Seen], shuffle: bool) -> Delta {
        struct Item {
            keys: [String; LEVELS],
            gold: String,
            pred: String,
        }
        let n = units.len();
        let mut items: Vec<Item> = Vec::new();
        for (u, s) in units.iter().enumerate() {
            let gold_of = |i: usize| -> Option<String> {
                if shuffle {
                    let o = units[(u + 7919) % n.max(1)];
                    o.chunks.get(i).cloned()
                } else {
                    Some(s.chunks[i].clone())
                }
            };
            for i in 0..s.m.len() {
                let Some(gold) = gold_of(i) else { continue };
                items.push(Item { keys: ctx_keys(&s.m, i), gold, pred: std.choose(a, &s.m, i).0 });
            }
        }
        let mut d = Delta::empty(name);
        for l in (0..LEVELS).rev() {
            let mut groups: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
            for (k, it) in items.iter().enumerate() {
                groups.entry(it.keys[l].as_str()).or_default().push(k);
            }
            let mut set: Vec<(String, Vec<usize>, Override)> = Vec::new();
            for (key, ks) in &groups {
                let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
                let mut right = 0u32;
                let mut was: BTreeMap<&str, u32> = BTreeMap::new();
                for &k in ks {
                    *counts.entry(items[k].gold.as_str()).or_default() += 1;
                    *was.entry(items[k].pred.as_str()).or_default() += 1;
                    if items[k].gold == items[k].pred {
                        right += 1;
                    }
                }
                let (c, nc) = counts.iter().fold(("", 0u32), |acc, (c, &v)| if v > acc.1 { (*c, v) } else { acc });
                let total = ks.len() as u32;
                if nc >= right + MIN_GAIN && nc as f64 >= TOLERANCE * total as f64 {
                    let w = was.iter().fold(("", 0u32), |acc, (c, &v)| if v > acc.1 { (*c, v) } else { acc }).0;
                    set.push((key.to_string(), ks.clone(), Override { chunk: c.to_string(), gain: nc - right, n: nc, total, was: w.to_string() }));
                }
            }
            for (key, ks, o) in set {
                for k in ks {
                    items[k].pred = o.chunk.clone();
                }
                d.levels[l].insert(key, o);
            }
        }
        d
    }

    /// The author's own words: for every concept and tag the author writes with one chunk in at least `share` of
    /// its places (and 3 times), that chunk — even where the standard already writes it in the author's own
    /// contexts, since another author's contexts lead the standard elsewhere (KJV's neighbours make it write
    /// *spake*). Finer overrides still win.
    pub fn with_own_words(mut self, units: &[&Seen], share: f64) -> Delta {
        let mut counts: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
        for s in units {
            for i in 0..s.m.len() {
                let key = ctx_keys(&s.m, i)[2].clone();
                *counts.entry(key).or_default().entry(s.chunks[i].clone()).or_default() += 1;
            }
        }
        for (key, t) in counts {
            if self.levels[2].contains_key(&key) {
                continue;
            }
            let total: u32 = t.values().sum();
            let (c, n) = t.iter().fold((String::new(), 0u32), |acc, (c, &v)| if v > acc.1 { (c.clone(), v) } else { acc });
            if n >= 3 && n as f64 >= share * total as f64 {
                self.levels[2].insert(key, Override { chunk: c, gain: 0, n, total, was: "(own word)".into() });
            }
        }
        self
    }

    /// The logic gate on an author's overrides: an override may change how the author writes a concept, never its
    /// logic. One whose chunk has other negations, modals or numbers than its concept (`logic::logical_words`) is
    /// taken out — it would add a «not» the meta-world does not hold, or change a number. Returns the overrides taken
    /// out.
    pub fn logic_gate(&mut self) -> Vec<String> {
        let mut out = Vec::new();
        for (l, m) in self.levels.iter_mut().enumerate() {
            m.retain(|key, o| {
                let concept = key.split('|').next().unwrap_or("");
                let cw: Vec<&str> = crate::meta::bare_concept(concept).split('_').collect();
                let ok = crate::logic::logical_words(&cw) == crate::logic::logical_words(&o.chunk.split(' ').collect::<Vec<_>>());
                if !ok {
                    out.push(format!("{} {}: «{key}» → «{}»", self.name, LEVEL_NAMES[l], o.chunk));
                }
                ok
            });
        }
        out
    }

    /// The archaism dial (arnold-1861): only the k overrides that fix most, the rest left to the standard.
    pub fn top(&self, k: usize) -> Delta {
        let mut all: Vec<(u32, usize, String)> = self.levels.iter().enumerate().flat_map(|(l, m)| m.iter().map(move |(key, o)| (o.gain, l, key.clone()))).collect();
        all.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)).then(x.2.cmp(&y.2)));
        let mut d = Delta::empty(&format!("{} top {k}", self.name));
        for (_, l, key) in all.into_iter().take(k) {
            d.levels[l].insert(key.clone(), self.levels[l][&key].clone());
        }
        d
    }

    pub fn size(&self) -> usize {
        self.levels.iter().map(|m| m.len()).sum()
    }

    /// The delta's size in bits: per override its level (2 bits), its key (concepts by the dictionary, a tag or
    /// relation 6 bits) and its chunk (each word by the dictionary, a mark 5 bits).
    pub fn bits(&self) -> f64 {
        let word = |w: &str| if w.chars().any(char::is_alphanumeric) { roundtrip::class::lemma_bits(&w.to_lowercase()) } else { 5.0 };
        let mut b = 0.0;
        for (l, m) in self.levels.iter().enumerate() {
            for (key, o) in m {
                let parts: Vec<&str> = key.split('|').collect();
                let kb: f64 = parts.iter().enumerate().map(|(i, p)| if (l == 0 && i != 1) || i == 0 { p.split('_').map(word).sum() } else { 6.0 }).sum();
                b += 2.0 + kb + o.chunk.split(' ').map(word).sum::<f64>();
            }
        }
        b
    }

    /// The override for token i (the finest context first), if any.
    pub fn get(&self, m: &[MTok], i: usize) -> Option<(&Override, String, usize)> {
        for (l, key) in ctx_keys(m, i).into_iter().enumerate() {
            if let Some(o) = self.levels.get(l).and_then(|x| x.get(&key)) {
                return Some((o, key, l));
            }
        }
        None
    }
}

/// An author's typography: the quotation marks it opens and closes with (none: it writes no quotes), and how it
/// writes the possessive — the mark, and whether a word ending in *s* takes the mark alone (*Moses’*).
#[derive(Clone, Debug, Default)]
pub struct Typo {
    pub open: Option<String>,
    pub close: Option<String>,
    pub mark: String,
    pub bare_after_s: bool,
}

const QUOTES: [&str; 5] = ["“", "”", "‘", "\"", "'"];

impl Typo {
    /// Learn from an author's training text: its most frequent opening quote, and its possessives.
    pub fn learn(c: &Corpus, keep: &dyn Fn(VKey) -> bool) -> Typo {
        let mut q: BTreeMap<&str, u32> = BTreeMap::new();
        let (mut curly, mut straight, mut bare, mut full) = (0u32, 0u32, 0u32, 0u32);
        for (k, v) in &c.verses {
            if !keep(*k) {
                continue;
            }
            for (i, w) in v.words.iter().enumerate() {
                let f = w.form.as_str();
                let prev_word = i > 0 && v.words[i - 1].form.chars().last().is_some_and(char::is_alphanumeric);
                if w.tag == en::gram::Tag::POS || (prev_word && matches!(f, "’" | "'" | "’s" | "'s") && w.rel != en::gram::Rel::Punct) {
                    if f.starts_with('’') {
                        curly += 1;
                    } else {
                        straight += 1;
                    }
                    if i > 0 && v.words[i - 1].form.ends_with('s') {
                        if f.len() <= 3 && !f.ends_with('s') {
                            bare += 1;
                        } else {
                            full += 1;
                        }
                    }
                } else if QUOTES.contains(&f) && w.rel == en::gram::Rel::Punct {
                    *q.entry(f).or_default() += 1;
                }
            }
        }
        let n = |x: &str| q.get(x).copied().unwrap_or(0);
        let (open, close) = if n("“") + n("”") > 0 {
            (Some("“"), Some("”"))
        } else if n("‘") > 0 {
            (Some("‘"), Some("’"))
        } else if n("'") > 0 {
            (Some("'"), Some("'"))
        } else if n("\"") > 0 {
            (Some("\""), Some("\""))
        } else {
            (None, None)
        };
        Typo { open: open.map(str::to_string), close: close.map(str::to_string), mark: if curly >= straight { "’".into() } else { "'".into() }, bare_after_s: bare > full }
    }

    /// The author's quotes (by parity: odd ones open, even ones close; none if the author writes none) and
    /// possessives on a run of forms.
    pub fn apply(&self, forms: &[String]) -> Vec<String> {
        let mut out: Vec<String> = Vec::with_capacity(forms.len());
        let mut open = false;
        for (i, f) in forms.iter().enumerate() {
            let prev_word = out.last().is_some_and(|p: &String| p.chars().last().is_some_and(char::is_alphanumeric));
            let poss = prev_word && matches!(f.as_str(), "’s" | "'s" | "’" | "'") && !(open && matches!(f.as_str(), "’" | "'") && forms.get(i + 1).is_none_or(|n| !n.chars().next().is_some_and(char::is_alphanumeric)));
            if poss {
                let ends_s = out.last().is_some_and(|p| p.ends_with('s'));
                out.push(if ends_s && self.bare_after_s { self.mark.clone() } else { format!("{}s", self.mark) });
                continue;
            }
            if QUOTES.contains(&f.as_str()) || f == "’" {
                let q = if open { self.close.clone() } else { self.open.clone() };
                open = !open;
                if let Some(q) = q {
                    out.push(q);
                }
                continue;
            }
            out.push(f.clone());
        }
        out
    }
}

/// An encoder: the standard with an author's delta (or none), or the bare forms (no settings at all).
pub struct Encoder<'a> {
    pub name: String,
    pub std: Option<&'a Standard>,
    pub delta: Option<&'a Delta>,
    /// the author's typography (quotes, possessives), applied to the words written
    pub typo: Option<&'a Typo>,
}

impl Encoder<'_> {
    /// The words for a meta-world, with the reason of every choice.
    pub fn encode(&self, a: &Annotator, m: &[MTok]) -> (Vec<String>, Vec<String>) {
        let mut forms = Vec::new();
        let mut why = Vec::new();
        for i in 0..m.len() {
            let (chunk, w) = match (self.delta.and_then(|d| d.get(m, i).map(|x| (d, x))), self.std) {
                (Some((d, (o, key, l))), _) => (o.chunk.clone(), format!("{} delta: «{key}» → «{}» ({} of {}, +{} over «{}», {})", d.name, o.chunk, o.n, o.total, o.gain, o.was, LEVEL_NAMES[l])),
                (None, Some(s)) => s.choose(a, m, i),
                (None, None) => (bare_form(a, &m[i]).join(" "), format!("{}: bare form", m[i].concept)),
            };
            forms.extend(chunk.split(' ').filter(|x| !x.is_empty()).map(str::to_string));
            why.push(w);
        }
        if let Some(t) = self.typo {
            forms = t.apply(&forms);
        }
        recap(&mut forms);
        (forms, why)
    }
}

/// The overrides of a delta, most useful first: (level, key, chunk, was, gain, n, total).
pub fn listing(d: &Delta, top: usize) -> Vec<(usize, String, Override)> {
    let mut v: Vec<(usize, String, Override)> = d.levels.iter().enumerate().flat_map(|(l, m)| m.iter().map(move |(k, o)| (l, k.clone(), o.clone()))).collect();
    v.sort_by(|x, y| y.2.gain.cmp(&x.2.gain).then(x.1.cmp(&y.1)));
    v.truncate(top);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typography_of_quotes_and_possessives() {
        let t = Typo { open: Some("“".into()), close: Some("”".into()), mark: "’".into(), bare_after_s: true };
        let f: Vec<String> = ["Moses", "’s", "minister", "said", ",", "'", "put", "off", "'", "."].iter().map(|x| x.to_string()).collect();
        let o = t.apply(&f);
        assert_eq!(o, ["Moses", "’", "minister", "said", ",", "“", "put", "off", "”", "."]);
        assert_eq!(crate::measure::join(&o), "Moses’ minister said, “put off”.");
        // an author with no quotes drops them
        let k = Typo { open: None, close: None, mark: "’".into(), bare_after_s: false };
        assert_eq!(k.apply(&f), ["Moses", "’s", "minister", "said", ",", "put", "off", "."]);
    }

    #[test]
    fn the_logic_gate_on_overrides() {
        let mut d = Delta::empty("x");
        let o = |c: &str| Override { chunk: c.into(), gain: 3, n: 3, total: 3, was: String::new() };
        d.levels[2].insert("go|VBD".into(), o("went"));
        d.levels[2].insert("not|RB".into(), o("not"));
        d.levels[3].insert("thou".into(), o("you"));
        // negative control: an override that adds a negation, or changes a number, is refused
        d.levels[2].insert("come|VBD".into(), o("did not come"));
        d.levels[2].insert("seven|CD".into(), o("eight"));
        let refused = d.logic_gate();
        assert_eq!(refused.len(), 2);
        assert_eq!(d.size(), 3);
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn contexts_votes_and_ties() {
        use en::gram::{Rel, Tag};
        let m = vec![MTok {
            concept: "say".into(),
            tag: Tag::VB,
            head: 0,
            rel: Rel::Root,
            span: (0, 1),
        }];
        let a = Seen {
            m: m.clone(),
            chunks: vec!["alpha".into()],
            keep: vec![],
        };
        let b = Seen {
            m,
            chunks: vec!["beta".into()],
            keep: vec![],
        };
        let s = Standard::train_voted(&[vec![&a, &a], vec![&b]]);
        let key = ctx_keys(&a.m, 0);
        assert_eq!(key[0], "say|VB|^|$");
        assert_eq!(s.tables[0][&key[0]]["alpha"], (2, 1.0));
        assert_eq!(s.tables[0][&key[0]]["beta"], (1, 1.0));
        assert_eq!(s.tokens, 3);
        assert!(s.voted);
        assert_eq!(
            best(&[("alpha".into(), 2), ("beta".into(), 2)].into()),
            ("alpha".into(), 2, 4)
        );
        let ignored = Seen {
            m: a.m.clone(),
            chunks: vec![],
            keep: vec![false],
        };
        assert_eq!(Standard::train(&[&ignored]).tokens, 0);
        let f = vec!["word".into(), ".".into(), "”".into(), "next".into()];
        assert!(sentence_start(&f, 3));
        assert!(!sentence_start(&f, 1));
    }
}
