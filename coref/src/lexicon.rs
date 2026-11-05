//! A gender, number and animacy lexicon of common nouns, learned from Mova's own parses (Bergsma and Lin 2006,
//! `bergsma-2006-bootstrapping-path-based-pronoun-resolution`: noun–pronoun pairs joined by high-precision
//! dependency paths, counted over a large corpus, smoothed into a per-noun distribution).
//!
//! Counting (`hits`) reads one tree at a time and finds pairs of a common noun and a third-person pronoun joined by
//! a hand-written path. Each path has an id, a test and an explanation:
//! - `refl`: the noun is the subject of the predicate a reflexive depends on («the farmer hurt himself»), or the
//!   reflexive stands right under the noun («the king himself»);
//! - `poss`: a possessive in an object or oblique of the predicate whose subject is the noun («the hen sat on her eggs»);
//! - `say`: the noun is the subject of a verb of saying or thinking and the pronoun the subject of its clause
//!   complement, outside quotes («the miller said he was poor»);
//! - `pred`: the pronoun is the subject of the noun as a predicate, without negation («she was a widow»);
//! - `appos`: the noun and the pronoun in apposition («he, the tailor, …»);
//! - `relpron`: a relative clause on the noun starts with who/whom/whose (animate) or which (inanimate).
//!
//! Classes: masc (he, him, his, himself), fem (she, her, hers, herself), neut (it, its, itself), plur (they, them,
//! their, themselves), who, which. A plural noun with a singular pronoun is dropped (a parser or path error); a
//! singular noun with «they» is kept as `plur` (notional agreement: «the family … their»).
//!
//! Only common nouns (NOUN) are counted: gender is never guessed from a name (`docs/context.md`, "Document level").
//!
//! The table (`build`) keeps the counts per noun with their sources and paths, and smoothed shares
//! p_c = (n_c + α·π_c) / (n + α) with the corpus-wide prior π over masc/fem/neut. `feats` turns a row into the
//! mention features the sieve uses: animate when the personal share (masc+fem+who) is at least `TAU`, inanimate
//! when the neuter share (neut+which) is; gender only from the he/she/it counts. A noun seen fewer than `MIN_N`
//! times stays unknown.
//!
//! The table lives outside the repository (`$MOVA_DATA/coref/gender.tsv`, or `COREF_GENDER=<path>`); an internal-only
//! table (Wikipedia provenance or an explicit internal marker in its header) is refused unless `COREF_GENDER_INTERNAL=1`
//! (internal comparison runs only); CC BY-SA alone does not trigger it.
//! `COREF_GENDER=off` turns the lexicon off everywhere; `COREF_X=no-gender-table` turns it off in the sieve only.

use std::collections::BTreeMap;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Result, bail};
use en::conllu::Doc as UdDoc;
use en::gram::{Feat, Rel, UPos};

use crate::doc::{self, Sent};
use crate::mention::{ANIM, FEM, INAN, MASC, NEUT};

pub const CLASSES: [&str; 6] = ["masc", "fem", "neut", "plur", "who", "which"];
pub const MASC_C: usize = 0;
pub const FEM_C: usize = 1;
pub const NEUT_C: usize = 2;
pub const PLUR_C: usize = 3;
pub const WHO_C: usize = 4;
pub const WHICH_C: usize = 5;
pub const PATHS: [&str; 6] = ["refl", "poss", "say", "pred", "appos", "relpron"];

/// Minimal evidence for a decision and the share that decides.
pub const MIN_N: u64 = 3;
pub const TAU: f64 = 0.8;
/// Smoothing weight of the prior.
pub const ALPHA: f64 = 2.0;

/// One noun–pronoun pair found by a path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    pub lemma: String,
    pub class: usize,
    pub path: &'static str,
    /// Noun and pronoun (or relative word) positions, 0-based.
    pub noun: usize,
    pub pron: usize,
}

/// The pronoun class and whether the word is nominative, possessive or reflexive.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PForm {
    Nom,
    Acc,
    Poss,
    Refl,
}

fn pron_class(low: &str) -> Option<(usize, PForm)> {
    use PForm::*;
    Some(match low {
        "he" => (MASC_C, Nom),
        "him" => (MASC_C, Acc),
        "his" => (MASC_C, Poss),
        "himself" => (MASC_C, Refl),
        "she" => (FEM_C, Nom),
        "her" => (FEM_C, Acc),
        "hers" => (FEM_C, Poss),
        "herself" => (FEM_C, Refl),
        "it" => (NEUT_C, Nom),
        "its" => (NEUT_C, Poss),
        "itself" => (NEUT_C, Refl),
        "they" => (PLUR_C, Nom),
        "them" => (PLUR_C, Acc),
        "their" | "theirs" => (PLUR_C, Poss),
        "themselves" => (PLUR_C, Refl),
        _ => return None,
    })
}

/// Verbs of saying and thinking whose clause complement's subject is often their subject. Hand-made closed list.
const SAY: [&str; 24] = ["say", "tell", "reply", "answer", "think", "believe", "know", "feel", "hope", "fear", "declare", "explain", "cry", "shout", "whisper", "suppose", "promise", "swear", "claim", "insist", "admit", "realize", "realise", "decide"];

/// A common noun that can be counted: NOUN, not a modifier inside a name or compound, not in a coordination, a
/// lemma of letters (and inner hyphens or apostrophes).
fn countable(s: &Sent, n: usize) -> bool {
    if s.upos(n) != Some(UPos::NOUN) || matches!(s.rel(n).base(), Rel::Compound | Rel::Flat | Rel::Fixed | Rel::Conj | Rel::Goeswith | Rel::Reparandum) {
        return false;
    }
    if s.kids_rel(n, Rel::Conj).next().is_some() {
        return false;
    }
    let l = s.lem(n);
    l.chars().count() >= 2 && l.chars().all(|c| c.is_alphabetic() || c == '-' || c == '\'') && l.chars().next().is_some_and(|c| c.is_alphabetic())
}

fn plural(s: &Sent, n: usize) -> bool {
    s.has(n, Feat::NumberPlur)
}

fn subject(s: &Sent, v: usize) -> Option<usize> {
    s.kids[v].iter().copied().find(|&k| matches!(s.rel(k), Rel::Nsubj | Rel::NsubjPass))
}

fn negated(s: &Sent, h: usize) -> bool {
    s.kids[h].iter().any(|&k| matches!(s.low(k).as_str(), "not" | "n't" | "never" | "no"))
}

/// Pairs found by the paths in one tree, in word order of the pronoun.
pub fn hits(s: &Sent) -> Vec<Hit> {
    let mut out = Vec::new();
    let mut push = |s: &Sent, n: usize, p: usize, class: usize, path: &'static str| {
        if !countable(s, n) || s.quote[n] != s.quote[p] || s.kids_rel(n, Rel::Det).any(|d| matches!(s.low(d).as_str(), "what" | "which" | "whose" | "whatever")) {
            return;
        }
        let pl = plural(s, n);
        if pl && class != PLUR_C && class < WHO_C {
            return;
        }
        out.push(Hit { lemma: s.lem(n), class, path, noun: n, pron: p });
    };
    for p in 0..s.len() {
        if s.upos(p) != Some(UPos::PRON) && s.upos(p) != Some(UPos::DET) {
            continue;
        }
        let low = s.low(p);
        let Some((class, form)) = pron_class(&low) else { continue };
        let Some(h) = s.parent(p) else { continue };
        match form {
            PForm::Refl => {
                if s.rel(p).base() == Rel::Nsubj {
                    continue;
                }
                if s.upos(h) == Some(UPos::NOUN) && h < p && matches!(s.rel(p).base(), Rel::Nmod | Rel::Appos | Rel::Dep) && s.kids_rel(p, Rel::Case).next().is_none() {
                    push(s, h, p, class, "refl");
                } else if let Some(n) = subject(s, h)
                    && n < p
                {
                    push(s, n, p, class, "refl");
                }
            }
            PForm::Poss | PForm::Acc if s.rel(p) == Rel::NmodPoss || form == PForm::Poss && s.rel(p).base() == Rel::Nmod => {
                // «her» as a possessive (nmod:poss); the possessed noun is an object or oblique of the predicate
                if !matches!(s.rel(h).base(), Rel::Obj | Rel::Obl | Rel::Iobj) {
                    continue;
                }
                let Some(v) = s.parent(h) else { continue };
                if let Some(n) = subject(s, v)
                    && n < p
                    && !s.dominates(n, p)
                {
                    push(s, n, p, class, "poss");
                }
            }
            PForm::Nom => {
                let r = s.rel(p);
                if !matches!(r, Rel::Nsubj | Rel::NsubjPass) {
                    if r == Rel::Appos && h < p {
                        push(s, h, p, class, "appos");
                    }
                    continue;
                }
                // pred: «she was a widow»
                if s.upos(h) == Some(UPos::NOUN) && s.kids_rel(h, Rel::Cop).next().is_some() && s.kids_rel(h, Rel::Case).next().is_none() && !negated(s, h) && !s.kids[h].iter().any(|&k| matches!(s.rel(k).base(), Rel::Ccomp | Rel::Csubj | Rel::Advcl) || s.rel(k) == Rel::Acl && s.kids_rel(k, Rel::Mark).next().is_some()) {
                    if !(class == NEUT_C && s.kids[h].iter().any(|&k| s.rel(k).base() == Rel::Acl)) {
                        push(s, h, p, class, "pred");
                    }
                }
                // say: «the miller said he was poor»
                if s.rel(h).base() == Rel::Ccomp
                    && let Some(v) = s.parent(h)
                    && s.upos(v) == Some(UPos::VERB)
                    && SAY.contains(&s.lem(v).as_str())
                    && let Some(n) = subject(s, v)
                    && n < p
                    && s.quote[h] == s.quote[v]
                {
                    push(s, n, p, class, "say");
                }
                // appos: «he, the tailor»
                for k in s.kids_rel(p, Rel::Appos).collect::<Vec<_>>() {
                    push(s, k, p, class, "appos");
                }
            }
            _ => {}
        }
    }
    // relpron: the first word of a relative clause on the noun (after a preposition: «to whom»)
    for n in 0..s.len() {
        for &c in &s.kids[n] {
            if s.rel(c) != Rel::AclRelcl || c < n {
                continue;
            }
            let (a, _) = s.sub[c];
            let mut w = a;
            while w < c && s.upos(w) == Some(UPos::ADP) {
                w += 1;
            }
            if w <= n {
                continue;
            }
            let class = match s.low(w).as_str() {
                "who" | "whom" | "whose" => WHO_C,
                "which" => WHICH_C,
                _ => continue,
            };
            push(s, n, w, class, "relpron");
        }
    }
    out.sort_by_key(|h| (h.pron, h.noun, h.path));
    out
}

/// Counts per (lemma, source, path, class) and the smallest example sentence id.
#[derive(Default, Debug)]
pub struct Counts {
    pub rows: BTreeMap<(String, String, String, usize), (u64, String)>,
    pub sentences: BTreeMap<String, u64>,
}

impl Counts {
    pub fn add(&mut self, lemma: &str, source: &str, path: &str, class: usize, n: u64, example: &str) {
        let e = self.rows.entry((lemma.to_string(), source.to_string(), path.to_string(), class)).or_insert((0, String::new()));
        e.0 += n;
        if e.1.is_empty() || (!example.is_empty() && example < e.1.as_str()) {
            e.1 = example.to_string();
        }
    }
    /// Part file: `lemma source path class count example`, with `# sentences <source> <n>` lines.
    pub fn write_part(&self) -> String {
        let mut o = String::new();
        for (src, n) in &self.sentences {
            o += &format!("# sentences\t{src}\t{n}\n");
        }
        for ((l, src, path, c), (n, ex)) in &self.rows {
            o += &format!("{l}\t{src}\t{path}\t{}\t{n}\t{ex}\n", CLASSES[*c]);
        }
        o
    }
    pub fn read_part(&mut self, text: &str) -> Result<()> {
        for (i, line) in text.lines().enumerate() {
            if let Some(r) = line.strip_prefix("# sentences\t") {
                let (src, n) = r.split_once('\t').context("sentences line")?;
                *self.sentences.entry(src.to_string()).or_default() += n.parse::<u64>()?;
                continue;
            }
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let c: Vec<&str> = line.split('\t').collect();
            if c.len() != 6 {
                bail!("part line {}: {} columns", i + 1, c.len());
            }
            let class = CLASSES.iter().position(|x| *x == c[3]).with_context(|| format!("class {}", c[3]))?;
            self.add(c[0], c[1], c[2], class, c[4].parse()?, c[5]);
        }
        Ok(())
    }
}

/// Counts pairs over a CoNLL-U stream (a file or `-` for stdin), in blocks of sentences.
pub fn count_stream(source: &str, input: &str, counts: &mut Counts) -> Result<()> {
    let reader: Box<dyn BufRead> = if input == "-" { Box::new(std::io::stdin().lock()) } else { Box::new(std::io::BufReader::new(std::fs::File::open(input).with_context(|| input.to_string())?)) };
    let mut block = String::new();
    let mut nsent = 0usize;
    let flush = |block: &mut String, counts: &mut Counts| -> Result<()> {
        if block.trim().is_empty() {
            block.clear();
            return Ok(());
        }
        let ud = UdDoc::parse(block)?;
        for d in doc::documents(&ud)? {
            *counts.sentences.entry(source.to_string()).or_default() += d.sents.len() as u64;
            for s in &d.sents {
                for h in hits(s) {
                    counts.add(&h.lemma, source, h.path, h.class, 1, &format!("{source}:{}", s.id));
                }
            }
        }
        block.clear();
        Ok(())
    };
    for line in reader.lines() {
        let line = line?;
        // newdoc comments of one source do not matter for counting; drop them so a block is one document
        if line.starts_with("# newdoc") || line.starts_with("# global.") {
            continue;
        }
        block.push_str(&line);
        block.push('\n');
        if line.is_empty() {
            nsent += 1;
            if nsent.is_multiple_of(5000) {
                flush(&mut block, counts)?;
            }
        }
    }
    flush(&mut block, counts)
}

/// A row of the table.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Row {
    pub counts: [u64; 6],
}

impl Row {
    fn gn(&self) -> u64 {
        self.counts[MASC_C] + self.counts[FEM_C] + self.counts[NEUT_C]
    }
    fn an(&self) -> u64 {
        self.counts[MASC_C] + self.counts[FEM_C] + self.counts[NEUT_C] + self.counts[WHO_C] + self.counts[WHICH_C]
    }
}

/// The loaded table: lemma → counts, and the priors.
#[derive(Debug, Default)]
pub struct Table {
    pub rows: BTreeMap<String, Row>,
    pub prior: [f64; 3],
    pub prior_anim: f64,
    pub min_n: u64,
    pub tau: f64,
    pub alpha: f64,
}

/// Features for a mention from the table, with the reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Feats {
    pub gender: u8,
    pub animacy: u8,
    pub why: String,
}

impl Table {
    pub fn from_counts(rows: BTreeMap<String, Row>) -> Table {
        let mut t = Table { rows, min_n: env_u64("COREF_GENDER_MIN", MIN_N), tau: env_f64("COREF_GENDER_TAU", TAU), alpha: env_f64("COREF_GENDER_ALPHA", ALPHA), ..Default::default() };
        let (mut g, mut a, mut an) = ([0u64; 3], 0u64, 0u64);
        for r in t.rows.values() {
            for (k, gk) in g.iter_mut().enumerate() {
                *gk += r.counts[k];
            }
            a += r.counts[MASC_C] + r.counts[FEM_C] + r.counts[WHO_C];
            an += r.an();
        }
        let gs = g.iter().sum::<u64>().max(1) as f64;
        t.prior = [g[0] as f64 / gs, g[1] as f64 / gs, g[2] as f64 / gs];
        t.prior_anim = a as f64 / an.max(1) as f64;
        t
    }

    /// Smoothed shares of masc, fem, neut among the he/she/it counts.
    pub fn shares(&self, r: &Row) -> [f64; 3] {
        let n = r.gn() as f64;
        let mut p = [0.0; 3];
        for (k, pk) in p.iter_mut().enumerate() {
            *pk = (r.counts[k] as f64 + self.alpha * self.prior[k]) / (n + self.alpha);
        }
        p
    }

    /// Smoothed animate share (masc+fem+who over those and neut+which).
    pub fn anim_share(&self, r: &Row) -> f64 {
        let a = (r.counts[MASC_C] + r.counts[FEM_C] + r.counts[WHO_C]) as f64;
        (a + self.alpha * self.prior_anim) / (r.an() as f64 + self.alpha)
    }

    pub fn feats(&self, lemma: &str) -> Option<Feats> {
        let r = self.rows.get(lemma)?;
        let mut f = Feats { gender: 0, animacy: 0, why: String::new() };
        if r.an() >= self.min_n {
            let pa = self.anim_share(r);
            if pa >= self.tau {
                f.animacy = ANIM;
            } else if 1.0 - pa >= self.tau {
                f.animacy = INAN;
            }
        }
        if r.gn() >= self.min_n {
            let p = self.shares(r);
            if p[MASC_C] >= self.tau {
                f.gender = MASC;
            } else if p[FEM_C] >= self.tau {
                f.gender = FEM;
            } else if p[NEUT_C] >= self.tau {
                f.gender = NEUT;
            } else if p[MASC_C] + p[FEM_C] >= self.tau {
                f.gender = MASC | FEM;
            }
        }
        // a neuter noun is inanimate and an animate one is not neuter: keep the two decisions consistent
        if f.gender == NEUT && f.animacy == ANIM || f.animacy == INAN && f.gender & (MASC | FEM) != 0 {
            return None;
        }
        if f.gender == 0 && f.animacy == 0 {
            return None;
        }
        let c = &r.counts;
        f.why = format!("lexicon «{lemma}»: he {} she {} it {} they {} who {} which {}", c[0], c[1], c[2], c[3], c[4], c[5]);
        Some(f)
    }
}

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn env_f64(k: &str, d: f64) -> f64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

/// Builds the table text from merged counts: a header with provenance, then one row per lemma.
pub fn build(counts: &Counts, header: &[String]) -> String {
    let mut rows: BTreeMap<String, Row> = BTreeMap::new();
    let mut src: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    let mut paths: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    let mut ex: BTreeMap<String, String> = BTreeMap::new();
    let mut by_path: BTreeMap<(String, usize), u64> = BTreeMap::new();
    for ((l, s, p, c), (n, e)) in &counts.rows {
        rows.entry(l.clone()).or_default().counts[*c] += n;
        *src.entry(l.clone()).or_default().entry(s.clone()).or_default() += n;
        *paths.entry(l.clone()).or_default().entry(p.clone()).or_default() += n;
        *by_path.entry((p.clone(), *c)).or_default() += n;
        let x = ex.entry(l.clone()).or_insert_with(|| e.clone());
        if e < x {
            *x = e.clone();
        }
    }
    let t = Table::from_counts(rows);
    let mut o = String::new();
    o += "# coref noun lexicon: gender, number and animacy counted from Mova's own parses (coref lexicon; Bergsma & Lin 2006 paths)\n";
    for h in header {
        o += &format!("# {h}\n");
    }
    for (s, n) in &counts.sentences {
        o += &format!("# source {s}: {n} sentences\n");
    }
    for p in PATHS {
        let v: Vec<String> = CLASSES.iter().enumerate().map(|(c, name)| format!("{name} {}", by_path.get(&(p.to_string(), c)).unwrap_or(&0))).collect();
        o += &format!("# path {p}: {}\n", v.join(", "));
    }
    o += &format!("# prior masc {:.4} fem {:.4} neut {:.4}; animate {:.4}; smoothing alpha {}\n", t.prior[0], t.prior[1], t.prior[2], t.prior_anim, t.alpha);
    o += "lemma\tmasc\tfem\tneut\tplur\twho\twhich\tp_masc\tp_fem\tp_neut\tp_anim\tsources\tpaths\texample\n";
    for (l, r) in &t.rows {
        let p = t.shares(r);
        let c = &r.counts;
        let s: Vec<String> = src[l].iter().map(|(k, v)| format!("{k}:{v}")).collect();
        let ps: Vec<String> = paths[l].iter().map(|(k, v)| format!("{k}:{v}")).collect();
        o += &format!("{l}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{}\t{}\t{}\n", c[0], c[1], c[2], c[3], c[4], c[5], p[0], p[1], p[2], t.anim_share(r), s.join(","), ps.join(","), ex[l]);
    }
    o
}

/// Reads a table file (the counts; the shares are recomputed with the current parameters).
pub fn parse_table(text: &str) -> Result<Table> {
    let mut rows = BTreeMap::new();
    for (i, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.starts_with("lemma\t") || line.is_empty() {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 7 {
            bail!("table line {}: {} columns", i + 1, c.len());
        }
        let mut r = Row::default();
        for k in 0..6 {
            r.counts[k] = c[k + 1].parse().with_context(|| format!("table line {}", i + 1))?;
        }
        rows.insert(c[0].to_string(), r);
    }
    Ok(Table::from_counts(rows))
}

/// Where the table is: `COREF_GENDER`, else `$MOVA_DATA/coref/gender.tsv`, else `~/mova-data/coref/gender.tsv`.
pub fn default_path() -> Option<PathBuf> {
    match std::env::var("COREF_GENDER") {
        Ok(v) if v == "off" => return None,
        Ok(v) if !v.is_empty() => return Some(PathBuf::from(v)),
        _ => {}
    }
    let data = std::env::var("MOVA_DATA").map(PathBuf::from).ok().or_else(|| std::env::var("HOME").ok().map(|h| Path::new(&h).join("mova-data")))?;
    Some(data.join("coref/gender.tsv"))
}

/// Whether a table's header marks it as internal-only: counted from Wikipedia (`# source wikipedia`), or declared so
/// (`# use: internal`, or a licence line that says internal). CC BY-SA alone is not internal: the released table is a
/// public CC BY-SA product.
fn internal_only(text: &str) -> bool {
    text.lines().take_while(|l| l.starts_with('#')).any(|l| {
        let l = l.to_lowercase();
        l.starts_with("# use: internal") || l.starts_with("# source wikipedia") || (l.starts_with("# license:") && l.contains("internal"))
    })
}

/// Loads a table, refusing an internal-only one unless internal use is asked for.
pub fn load(path: &Path) -> Result<Table> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    if internal_only(&text) && std::env::var("COREF_GENDER_INTERNAL").as_deref() != Ok("1") {
        bail!("{}: an internal-only table (Wikipedia-derived) is for internal comparison only (COREF_GENDER_INTERNAL=1)", path.display());
    }
    parse_table(&text)
}

#[cfg(test)]
mod internal_tests {
    use super::internal_only;

    #[test]
    fn cc_by_sa_alone_is_public_and_wikipedia_is_internal() {
        // the released table: public, whatever its licence line says
        assert!(!internal_only("# coref noun lexicon\n# License: CC BY-SA 4.0 — counted from public-domain tales\n# source tales: 1 sentences\nfarmer\t1\n"));
        assert!(!internal_only("# coref noun lexicon\n# source school: 1 sentences\n"));
        // negative controls: the Wikipedia table and explicit internal markers are refused
        assert!(internal_only("# License: CC BY-SA 4.0 — counted from English Wikipedia; internal comparison only\n"));
        assert!(internal_only("# coref noun lexicon\n# source wikipedia: 127096 sentences\n"));
        assert!(internal_only("# use: internal\n"));
        // a marker after the header does not count
        assert!(!internal_only("# source tales: 1 sentences\nfarmer\t1\n# source wikipedia: 1 sentences\n"));
    }
}

/// The process-wide table (loaded once; `None` when off or missing).
pub fn table() -> Option<&'static Table> {
    static T: OnceLock<Option<Table>> = OnceLock::new();
    T.get_or_init(|| {
        let p = default_path()?;
        if !p.exists() {
            return None;
        }
        match load(&p) {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!("coref lexicon: {e:#}; running without it");
                None
            }
        }
    })
    .as_ref()
}

#[cfg(test)]
thread_local! {
    /// A table for the tests of this thread, in place of the process-wide one.
    pub static TEST_TABLE: std::cell::RefCell<Option<Table>> = const { std::cell::RefCell::new(None) };
}

/// Features of a common noun from the process-wide table.
pub fn noun_feats(lemma: &str) -> Option<Feats> {
    // unit tests are hermetic: only the thread's own table, never the one in the data directory
    #[cfg(test)]
    return TEST_TABLE.with(|t| t.borrow().as_ref().and_then(|t| t.feats(lemma)));
    #[cfg(not(test))]
    table()?.feats(lemma)
}

/// The negative control: every lemma gets the counts of the lemma `k` places after it in sorted order (a
/// derangement for 0 < k < n), so the table keeps its sizes and shares but loses what each noun is.
pub fn rotate(t: &Table, k: usize) -> Table {
    let keys: Vec<&String> = t.rows.keys().collect();
    let n = keys.len();
    let rows = keys.iter().enumerate().map(|(i, l)| ((*l).clone(), t.rows[keys[(i + k) % n.max(1)]].clone())).collect();
    Table::from_counts(rows)
}

/// Writes a table back as counts (the header names what it is).
pub fn write_counts(t: &Table, header: &[String]) -> String {
    let mut o = String::new();
    for h in header {
        o += &format!("# {h}\n");
    }
    o += "lemma\tmasc\tfem\tneut\tplur\twho\twhich\n";
    for (l, r) in &t.rows {
        let c = &r.counts;
        o += &format!("{l}\t{}\t{}\t{}\t{}\t{}\t{}\n", c[0], c[1], c[2], c[3], c[4], c[5]);
    }
    o
}
