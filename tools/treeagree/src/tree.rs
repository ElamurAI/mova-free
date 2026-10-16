//! Sentences in CoNLL-U and the gates every agreed tree must pass.
//!
//! Gates (the verifier):
//! - every word passes the UD 2.18 registry for English (`en::udreg`): relation, features for the part of speech,
//!   auxiliaries, `cop` only "be";
//! - heads within range, no self loops, exactly one root, and `root` exactly where the head is 0;
//! - no cycles;
//! - `punct` is attached projectively (no arc crosses it), as the UD validator wants;
//! - the relation fits the word's UPOS (`fits`): no `amod` on a NOUN, no `obj` on an ADV; a relation and a
//!   tag change together or not at all.

use std::collections::BTreeMap;

use en::gram::{Feats, Rel, Tag, UPos};
use en::udreg::Registry;

/// A word of the reference (Haiku) tree: the raw columns are kept, head and relation are parsed.
#[derive(Clone, Debug)]
pub struct Tok {
    pub cols: Vec<String>,
    pub upos: UPos,
    pub feats: Feats,
    pub head: usize,
    pub rel: Rel,
}

impl Tok {
    /// Retag the word: UPOS, XPOS, FEATS and lemma together.
    pub fn retag(&mut self, f: &crate::rules::PosFix) {
        self.upos = f.upos;
        self.feats = Feats::parse(&f.feats).0;
        self.cols[2] = f.lemma.clone();
        self.cols[3] = f.upos.name().to_string();
        self.cols[4] = f.tag.name().to_string();
        self.cols[5] = f.feats.clone();
    }
    pub fn form(&self) -> &str {
        &self.cols[1]
    }
    pub fn lemma(&self) -> &str {
        &self.cols[2]
    }
}

#[derive(Clone, Debug)]
pub struct Sent {
    pub comments: Vec<String>,
    pub id: String,
    pub course: String,
    pub text: String,
    pub toks: Vec<Tok>,
}

/// A parser's word: tags, head and relation as written (UD or not).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Wit {
    pub upos: String,
    pub xpos: String,
    pub head: usize,
    pub rel: String,
}

/// Blocks of CoNLL-U: (comment lines, word lines). Multiword ranges and empty nodes are not expected.
pub fn blocks(src: &str) -> Vec<(Vec<&str>, Vec<Vec<&str>>)> {
    let mut out = Vec::new();
    let (mut com, mut rows) = (Vec::new(), Vec::new());
    for l in src.lines() {
        if l.is_empty() {
            if !rows.is_empty() || !com.is_empty() {
                out.push((std::mem::take(&mut com), std::mem::take(&mut rows)));
            }
        } else if l.starts_with('#') {
            com.push(l);
        } else {
            rows.push(l.split('\t').collect());
        }
    }
    if !rows.is_empty() {
        out.push((com, rows));
    }
    out
}

fn comment<'a>(com: &[&'a str], key: &str) -> &'a str {
    com.iter().find_map(|c| c.strip_prefix("# ").and_then(|c| c.strip_prefix(key)).and_then(|c| c.strip_prefix(" = "))).unwrap_or("")
}

/// The reference file. A sentence whose columns do not parse is an error: the silver passed the same gates.
pub fn read_ref(src: &str) -> Result<Vec<Sent>, String> {
    let mut out = Vec::new();
    for (com, rows) in blocks(src) {
        let id = comment(&com, "sent_id").to_string();
        let mut toks = Vec::new();
        for (i, c) in rows.iter().enumerate() {
            if c.len() != 10 || c[0] != (i + 1).to_string() {
                return Err(format!("{id}: line {} is not a plain word line", i + 1));
            }
            let upos = UPos::parse(c[3]).ok_or_else(|| format!("{id}: UPOS {}", c[3]))?;
            let (feats, unknown) = Feats::parse(c[5]);
            if unknown > 0 {
                return Err(format!("{id}: FEATS {}", c[5]));
            }
            let rel = Rel::parse(c[7]).ok_or_else(|| format!("{id}: DEPREL {}", c[7]))?;
            let head = c[6].parse().map_err(|_| format!("{id}: HEAD {}", c[6]))?;
            toks.push(Tok { cols: c.iter().map(|s| s.to_string()).collect(), upos, feats, head, rel });
        }
        out.push(Sent {
            id,
            course: comment(&com, "course").to_string(),
            text: comment(&com, "text").to_string(),
            comments: com.iter().map(|s| s.to_string()).collect(),
            toks,
        });
    }
    Ok(out)
}

/// A parser's file: head and relation per word, in the order of the reference.
pub fn read_wit(src: &str) -> Vec<Vec<Wit>> {
    blocks(src)
        .into_iter()
        .map(|(_, rows)| {
            rows.iter()
                .map(|c| Wit {
                    upos: c.get(3).unwrap_or(&"_").to_string(),
                    xpos: c.get(4).unwrap_or(&"_").to_string(),
                    head: c.get(6).and_then(|h| h.parse().ok()).unwrap_or(0),
                    rel: c.get(7).unwrap_or(&"_").to_string(),
                })
                .collect()
        })
        .collect()
}

/// Why a tree fails the gates.
#[derive(Debug, PartialEq)]
pub enum Bad {
    Registry(usize, String),
    Head(usize),
    Roots(usize),
    RootRel(usize),
    Cycle(usize),
    PunctCross(usize),
    PosRel(usize),
}

impl std::fmt::Display for Bad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Bad::Registry(i, m) => write!(f, "registry at {i}: {m}"),
            Bad::Head(i) => write!(f, "head out of range at {i}"),
            Bad::Roots(n) => write!(f, "{n} roots"),
            Bad::RootRel(i) => write!(f, "root relation and head 0 disagree at {i}"),
            Bad::Cycle(i) => write!(f, "cycle through {i}"),
            Bad::PunctCross(i) => write!(f, "punct at {i} attached non-projectively"),
            Bad::PosRel(i) => write!(f, "relation does not fit the UPOS at {i}"),
        }
    }
}

impl Bad {
    pub fn kind(&self) -> &'static str {
        match self {
            Bad::Registry(..) => "registry",
            Bad::Head(_) => "head",
            Bad::Roots(_) => "roots",
            Bad::RootRel(_) => "root-rel",
            Bad::Cycle(_) => "cycle",
            Bad::PunctCross(_) => "punct-cross",
            Bad::PosRel(_) => "pos-rel",
        }
    }
}

/// Structure only: range, one root, root relation, no cycles (positions are 1-based, heads[i] for word i+1).
pub fn shape(heads: &[usize], rels: &[Rel]) -> Result<(), Bad> {
    let n = heads.len();
    for (i, (&h, &r)) in heads.iter().zip(rels).enumerate() {
        if h > n || h == i + 1 {
            return Err(Bad::Head(i + 1));
        }
        if (h == 0) != (r == Rel::Root) {
            return Err(Bad::RootRel(i + 1));
        }
    }
    let roots = heads.iter().filter(|&&h| h == 0).count();
    if roots != 1 {
        return Err(Bad::Roots(roots));
    }
    for i in 1..=n {
        let mut j = i;
        for _ in 0..=n {
            j = heads[j - 1];
            if j == 0 {
                break;
            }
        }
        if j != 0 {
            return Err(Bad::Cycle(i));
        }
    }
    Ok(())
}

/// Does any arc cross the arc of word `i` (1-based)?
pub fn crossed(heads: &[usize], i: usize) -> bool {
    let h = heads[i - 1];
    if h == 0 {
        return false;
    }
    let (a, b) = (i.min(h), i.max(h));
    heads.iter().enumerate().any(|(k, &hk)| {
        let k = k + 1;
        if hk == 0 || k == i {
            return false;
        }
        let (c, d) = (k.min(hk), k.max(hk));
        // strictly interleaved ends
        (c < a && a < d && d < b) || (a < c && c < b && b < d)
    })
}

/// All gate violations of a whole tree (empty — the tree passes).
pub fn violations(toks: &[Tok], heads: &[usize], rels: &[Rel], reg: &Registry) -> Vec<Bad> {
    let mut out = Vec::new();
    for (i, t) in toks.iter().enumerate() {
        if let Some(m) = reg.check(t.upos, t.feats, rels[i], t.lemma()).into_iter().next() {
            out.push(Bad::Registry(i + 1, m));
        }
    }
    for (i, t) in toks.iter().enumerate() {
        if !fits(rels[i], t.upos) {
            out.push(Bad::PosRel(i + 1));
        }
    }
    if let Err(b) = shape(heads, rels) {
        out.push(b);
        return out;
    }
    for i in 1..=toks.len() {
        if rels[i - 1] == Rel::Punct && crossed(heads, i) {
            out.push(Bad::PunctCross(i));
        }
    }
    out
}

/// Edits may not bring anything new: the shape must hold, and every violation must already be in the reference tree.
pub fn no_worse(toks: &[Tok], heads: &[usize], rels: &[Rel], base: &[Bad], reg: &Registry) -> Result<(), Bad> {
    shape(heads, rels)?;
    match violations(toks, heads, rels, reg).into_iter().find(|b| !base.contains(b)) {
        Some(b) => Err(b),
        None => Ok(()),
    }
}

/// Which UPOS a relation allows (closed-class relations and the nominal ones; clausal relations take any).
/// Sets cover what the school silver itself uses above 0.2% (UPOS per relation, 15.10).
pub fn fits(rel: Rel, u: UPos) -> bool {
    use UPos::*;
    let nominal = matches!(u, NOUN | PROPN | PRON | NUM | DET | SYM | X | ADJ | VERB);
    match rel {
        Rel::Amod => matches!(u, ADJ | VERB),
        Rel::Nummod => u == NUM,
        Rel::Det | Rel::DetPredet => matches!(u, DET | PRON),
        Rel::Case => matches!(u, ADP | PART | SYM | ADJ),
        Rel::Mark => matches!(u, SCONJ | PART | ADP | ADV),
        Rel::Cc | Rel::CcPreconj => matches!(u, CCONJ | SYM),
        Rel::CompoundPrt => matches!(u, ADP | ADV | PART),
        Rel::Compound => matches!(u, NOUN | PROPN | NUM | X | SYM | ADJ | VERB | ADV),
        Rel::Aux | Rel::AuxPass | Rel::Cop => u == AUX,
        Rel::Expl => u == PRON,
        Rel::Advmod => matches!(u, ADV | PART | ADJ | SYM),
        Rel::Punct => matches!(u, PUNCT | SYM),
        Rel::Nsubj | Rel::NsubjPass | Rel::NsubjOuter | Rel::Obj | Rel::Iobj | Rel::Obl | Rel::OblAgent | Rel::OblNpmod
        | Rel::OblTmod | Rel::OblUnmarked | Rel::Nmod | Rel::NmodDesc | Rel::NmodNpmod | Rel::NmodPoss | Rel::NmodTmod
        | Rel::NmodUnmarked | Rel::Vocative | Rel::Appos => nominal,
        _ => true,
    }
}

/// FEATS and lemma for a retagged word, learned from the reference itself: the most frequent pair for
/// (form, UPOS, XPOS), else FEATS for (UPOS, XPOS) and Haiku's lemma. Ties go to the smaller string (BTreeMap).
#[derive(Default)]
pub struct Lex {
    by_form: BTreeMap<(String, UPos, Tag), BTreeMap<(String, String), u32>>,
    by_tag: BTreeMap<(UPos, Tag), BTreeMap<String, u32>>,
}

impl Lex {
    pub fn learn(sents: &[Sent]) -> Lex {
        let mut l = Lex::default();
        for t in sents.iter().flat_map(|s| &s.toks) {
            let Some(tag) = Tag::parse(&t.cols[4]) else { continue };
            *l.by_form.entry((t.form().to_lowercase(), t.upos, tag)).or_default().entry((t.cols[5].clone(), t.lemma().to_string())).or_default() += 1;
            *l.by_tag.entry((t.upos, tag)).or_default().entry(t.cols[5].clone()).or_default() += 1;
        }
        l
    }

    fn top<K: Clone + Ord>(m: &BTreeMap<K, u32>) -> Option<K> {
        m.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(k, _)| k.clone())
    }

    /// (FEATS, lemma) for a word under new tags; None if the tag pair never occurs in the reference.
    pub fn get(&self, form: &str, lemma: &str, u: UPos, tag: Tag) -> Option<(String, String)> {
        if let Some(x) = self.by_form.get(&(form.to_lowercase(), u, tag)).and_then(Self::top) {
            return Some(x);
        }
        self.by_tag.get(&(u, tag)).and_then(Self::top).map(|f| (f, lemma.to_string()))
    }
}
