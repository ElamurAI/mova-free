//! Agreement rules. Haiku's tree is the default; a word changes only when a rule below fires, and every rule
//! names its evidence. Witnesses: Stanza and Trankit (UD, the two strongest on the school: UAS−p 87.5 / 87.9
//! against Haiku, 86.5 against each other) and spaCy trf (ClearNLP head rules, an independent third view; a `pobj`
//! is lifted from its preposition to the preposition's head to compare with UD).
//!
//! Order is priority: a word gets the edit of the first rule that fires for it.

use en::gram::{Rel, Tag, UPos};

use crate::tree::{Lex, Tok, Wit};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Rule {
    /// UD 2: `cc` attaches to the conjunct that follows it, not to the first one.
    CcNext,
    /// UD 2: a comma or semicolon before a conjunct attaches to that conjunct.
    PunctNext,
    /// UD 2: all conjuncts hang on the first one, not chained.
    ConjFirst,
    /// UD: a verb particle after its verb ("pick up") is `compound:prt`, not plain `compound`.
    Particle,
    /// Both UD parsers agree on another head and relation, and spaCy trf picks the same head.
    Strong,
    /// Punctuation: both UD parsers agree on another head.
    PunctAgree,
    /// Same head everywhere, both UD parsers agree on another relation.
    RelAgree,
    /// Both UD parsers agree on another UPOS and XPOS (alone, or riding on the word's other edit).
    PosAgree,
}

impl Rule {
    pub const ALL: [Rule; 8] = [Rule::CcNext, Rule::PunctNext, Rule::ConjFirst, Rule::Particle, Rule::Strong, Rule::PunctAgree, Rule::RelAgree, Rule::PosAgree];
    pub fn id(self) -> &'static str {
        match self {
            Rule::CcNext => "R1-cc-next",
            Rule::PunctNext => "R2-punct-next",
            Rule::ConjFirst => "R3-conj-first",
            Rule::Particle => "R4-particle",
            Rule::Strong => "R5-strong",
            Rule::PunctAgree => "R6-punct-agree",
            Rule::RelAgree => "R7-rel-agree",
            Rule::PosAgree => "R8-pos-agree",
        }
    }
}

/// New tags of a word: UPOS, XPOS, FEATS (as written in the reference) and lemma.
#[derive(Clone, Debug, PartialEq)]
pub struct PosFix {
    pub upos: UPos,
    pub tag: Tag,
    pub feats: String,
    pub lemma: String,
}

/// One proposed change of word `pos` (1-based); `fix` retags it in the same step.
#[derive(Clone, Debug, PartialEq)]
pub struct Edit {
    pub pos: usize,
    pub head: usize,
    pub rel: Rel,
    pub rule: Rule,
    pub fix: Option<PosFix>,
}

/// Everything the rules see for one sentence.
pub struct View<'a> {
    pub h: &'a [Tok],
    pub s: &'a [Wit],
    pub t: &'a [Wit],
    pub p: Option<&'a [Wit]>,
    pub lex: &'a Lex,
}

/// A parser relation as a UD relation (unknown or non-UD → None).
fn ud(r: &str) -> Option<Rel> {
    Rel::parse(r).or_else(|| Rel::parse(r.split(':').next().unwrap_or(r)))
}

impl View<'_> {
    /// spaCy's head for word `pos`, comparable with UD.
    pub fn p_head(&self, pos: usize) -> Option<usize> {
        let p = self.p?;
        let w = &p[pos - 1];
        if w.rel == "pobj" && w.head > 0 { Some(p[w.head - 1].head) } else { Some(w.head) }
    }

    /// The first conjunct of the coordination headed by `h` (in Haiku's tree, chains included) after `pos`.
    fn next_conj(&self, h: usize, pos: usize) -> Option<usize> {
        let mut set = vec![h];
        let mut k = 0;
        while k < set.len() {
            let x = set[k];
            set.extend((1..=self.h.len()).filter(|&c| self.h[c - 1].head == x && self.h[c - 1].rel == Rel::Conj));
            k += 1;
        }
        (pos + 1..=self.h.len()).find(|&c| self.h[c - 1].rel == Rel::Conj && set.contains(&self.h[c - 1].head))
    }

    /// Does word `pos` have a `cc` dependent in Haiku's tree? Then it is a conjunct and must stay one.
    fn has_cc(&self, pos: usize) -> bool {
        self.h.iter().any(|t| t.head == pos && t.rel.base() == Rel::Cc)
    }

    /// R8: both UD parsers tag the word alike and differently from Haiku; the tag pair is known in the reference.
    fn pos_fix(&self, pos: usize) -> Option<PosFix> {
        let (t, s, tr) = (&self.h[pos - 1], &self.s[pos - 1], &self.t[pos - 1]);
        if s.upos != tr.upos || s.xpos != tr.xpos {
            return None;
        }
        let (u, tag) = (UPos::parse(&s.upos)?, Tag::parse(&s.xpos)?);
        if u == t.upos && t.cols[4] == s.xpos {
            return None;
        }
        let (feats, lemma) = self.lex.get(t.form(), t.lemma(), u, tag)?;
        Some(PosFix { upos: u, tag, feats, lemma })
    }

    /// The relation both UD parsers give (same full relation → it; same base → the base; otherwise None).
    fn st_rel(&self, pos: usize) -> Option<Rel> {
        let (a, b) = (ud(&self.s[pos - 1].rel)?, ud(&self.t[pos - 1].rel)?);
        if a == b {
            Some(a)
        } else if a.base() == b.base() {
            Some(a.base())
        } else {
            None
        }
    }
}

/// Words whose Haiku analysis no rule overrides: names and foreign stretches, fixed expressions, typos.
fn frozen(t: &Tok) -> bool {
    t.upos == UPos::X || matches!(t.rel, Rel::Flat | Rel::FlatForeign | Rel::Fixed | Rel::Goeswith | Rel::Reparandum)
}

/// The relation to write when the head moves: keep Haiku's subtype if the base relation is the same.
fn pick(haiku: Rel, agreed: Rel) -> Rel {
    if haiku.base() == agreed.base() { haiku } else { agreed }
}

pub fn propose(v: &View) -> Vec<Edit> {
    let mut out: Vec<Edit> = Vec::new();
    for pos in 1..=v.h.len() {
        let t = &v.h[pos - 1];
        if frozen(t) {
            continue;
        }
        let (s, tr) = (&v.s[pos - 1], &v.t[pos - 1]);
        let either = |c: usize| s.head == c || tr.head == c;
        let e = |head, rel, rule| Some(Edit { pos, head, rel, rule, fix: None });
        let edit = if t.rel == Rel::Cc && t.head < pos && t.head > 0 {
            // R1: cc on the left conjunct while a conjunct of the same word follows
            v.next_conj(t.head, pos).filter(|&c| either(c)).and_then(|c| e(c, Rel::Cc, Rule::CcNext))
        } else if t.rel == Rel::Punct && matches!(t.form(), "," | ";") && t.head < pos && t.head > 0
            && v.next_conj(t.head, pos).is_some_and(|c| either(c))
        {
            // R2: the comma before a later conjunct
            v.next_conj(t.head, pos).and_then(|c| e(c, Rel::Punct, Rule::PunctNext))
        } else if t.rel == Rel::Conj && t.head > 0 && v.h[t.head - 1].rel == Rel::Conj && v.h[t.head - 1].head > 0
            && s.head == v.h[t.head - 1].head && tr.head == s.head
        {
            // R3: a chained conjunct goes to the first one, when both parsers say so
            e(v.h[t.head - 1].head, Rel::Conj, Rule::ConjFirst)
        } else if t.rel == Rel::Compound && matches!(t.upos, UPos::ADP | UPos::PART | UPos::ADV) && t.head > 0 && t.head < pos
            && v.h[t.head - 1].upos == UPos::VERB
        {
            // R4: a particle after its verb
            e(t.head, Rel::CompoundPrt, Rule::Particle)
        } else if t.rel != Rel::Punct && s.head == tr.head && s.head != t.head && v.p_head(pos) == Some(s.head) {
            // R5: two UD parsers and spaCy against Haiku
            v.st_rel(pos).filter(|r| *r != Rel::Dep).filter(|r| !(t.rel == Rel::Conj && r.base() != Rel::Conj && v.has_cc(pos))).and_then(|r| {
                let r = if s.head == 0 { Rel::Root } else { pick(t.rel, r) };
                if (s.head == 0) != (r == Rel::Root) { None } else { e(s.head, r, Rule::Strong) }
            })
        } else if t.rel == Rel::Punct && s.head == tr.head && s.head != t.head && s.head > 0 && s.rel == "punct" && tr.rel == "punct" {
            // R6: punctuation, both UD parsers (spaCy follows other punctuation rules)
            e(s.head, Rel::Punct, Rule::PunctAgree)
        } else if t.head > 0 && s.head == t.head && tr.head == t.head && t.rel != Rel::Punct {
            // R7: the head is not in question, the relation is
            // a conjunct that has its own `cc` stays `conj`
            v.st_rel(pos)
                .filter(|r| r.base() != t.rel.base() && !matches!(r, Rel::Dep | Rel::Root | Rel::Punct))
                .filter(|_| !(t.rel == Rel::Conj && v.has_cc(pos)))
                .and_then(|r| e(t.head, r, Rule::RelAgree))
        } else {
            None
        };
        let fix = v.pos_fix(pos);
        match (edit, fix) {
            (Some(mut x), f) => {
                x.fix = f;
                out.push(x);
            }
            // alone, a retag must not leave the relation behind: both parsers keep Haiku's base relation
            (None, Some(f)) if [s, tr].iter().all(|w| ud(&w.rel).is_some_and(|r| r.base() == t.rel.base())) => {
                out.push(Edit { pos, head: t.head, rel: t.rel, rule: Rule::PosAgree, fix: Some(f) })
            }
            (None, _) => {}
        }
    }
    out
}
