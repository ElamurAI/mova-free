//! Sentence annotation in full UD as Rust structures: a token with lemma, UPOS, PTB tag (XPOS), FEATS
//! features, head and relation (with subtype). Reading and writing CoNLL-U, gates, annotation comparison.
//!
//! Gates:
//! - format: columns, forms match the given tokens;
//! - closed inventories: UPOS, PTB, FEATS pairs, relations;
//! - UD 2.18 registry for English (`en::udreg`): features allowed for their part of speech, relation from
//!   the registry, AUX only from the list of auxiliaries, `cop` only "be";
//! - tree: heads within range, exactly one root, no cycles.

use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};

use en::gram::{Feats, Rel, Tag, UPos};
use en::udreg::Registry;

/// An annotated word.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Tok {
    pub form: String,
    pub lemma: String,
    pub upos: UPos,
    pub tag: Tag,
    pub feats: Feats,
    pub head: usize,
    pub rel: Rel,
}

pub type Ann = Vec<Tok>;

/// A sentence to annotate: id, tokens (fixed), gold (if any).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub id: String,
    pub text: String,
    pub forms: Vec<String>,
    pub gold: Option<Ann>,
}

/// CoNLL-U block for a sentence (DEPS and MISC are "_").
pub fn write_block(id: &str, a: &Ann) -> String {
    let mut s = format!("# sent_id = {id}\n");
    for (i, t) in a.iter().enumerate() {
        let _ = writeln!(s, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t_\t_", i + 1, t.form, t.lemma, t.upos, t.tag, t.feats, t.head, t.rel);
    }
    s.push('\n');
    s
}

/// Why a block failed a gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bad {
    Count(usize, usize),
    Form(usize),
    Upos(usize, String),
    Tag(usize, String),
    Feat(usize, String),
    Rel(usize, String),
    Registry(usize, String),
    Head(usize),
    Roots(usize),
    Cycle(usize),
    Columns(usize),
}

impl std::fmt::Display for Bad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Bad::Count(a, b) => write!(f, "{a} tokens, expected {b}"),
            Bad::Form(i) => write!(f, "form of token {i} differs from the given one"),
            Bad::Upos(i, t) => write!(f, "UPOS \"{t}\" of token {i} is outside UD"),
            Bad::Tag(i, t) => write!(f, "tag \"{t}\" of token {i} is outside PTB"),
            Bad::Feat(i, t) => write!(f, "FEATS \"{t}\" of token {i}: unknown pair"),
            Bad::Rel(i, r) => write!(f, "relation \"{r}\" of token {i} is outside UD"),
            Bad::Registry(i, m) => write!(f, "token {i}: {m}"),
            Bad::Head(i) => write!(f, "head of token {i} is outside the sentence"),
            Bad::Roots(n) => write!(f, "{n} roots, expected 1"),
            Bad::Cycle(i) => write!(f, "cycle through token {i}"),
            Bad::Columns(i) => write!(f, "line {i}: fewer than 8 columns"),
        }
    }
}

/// CoNLL-U blocks from the reply text: (id, token lines).
pub fn split_blocks(text: &str) -> Vec<(String, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    for line in text.lines() {
        let l = line.trim_end();
        if let Some(id) = l.strip_prefix("# sent_id = ") {
            out.push((id.trim().to_string(), Vec::new()));
        } else if l.starts_with(|c: char| c.is_ascii_digit()) {
            if let Some(last) = out.last_mut() {
                last.1.push(l.to_string());
            }
        }
    }
    out
}

/// How many relations were normalized to EWT conventions (`ewt_rel`) — for the run report.
pub static EWT_FIXED: AtomicUsize = AtomicUsize::new(0);

/// Relations that Opus takes from other treebanks while EWT 2.18 writes them differently:
/// - `det:poss` (other UD languages) → `nmod:poss`: possessive pronouns and 's; Opus high — in ~3% of sentences;
/// - `:tmod`, `:npmod` (before UD 2.15) → `:unmarked`; the seed rule `en.errors.tmod-npmod` found 12
///   such cases in 292 calibration sentences on 25.09, all genuine.
pub fn ewt_rel(r: &str) -> Option<&'static str> {
    match r {
        "det:poss" => Some("nmod:poss"),
        "obl:tmod" | "obl:npmod" => Some("obl:unmarked"),
        "nmod:tmod" | "nmod:npmod" => Some("nmod:unmarked"),
        _ => None,
    }
}

/// Parsing and gates of a block. The only normalization is relations to EWT conventions (`ewt_rel`), with a counter.
pub fn parse_block(lines: &[String], forms: &[String], reg: &Registry) -> Result<Ann, Bad> {
    let mut a: Ann = Vec::new();
    for (k, l) in lines.iter().enumerate() {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 8 {
            return Err(Bad::Columns(k + 1));
        }
        // multiword ranges and empty nodes are skipped
        if c[0].contains('-') || c[0].contains('.') {
            continue;
        }
        let i = a.len() + 1;
        let upos = UPos::parse(c[3]).ok_or_else(|| Bad::Upos(i, c[3].to_string()))?;
        let tag = Tag::parse(c[4]).ok_or_else(|| Bad::Tag(i, c[4].to_string()))?;
        let (feats, unknown) = Feats::parse(c[5]);
        if unknown > 0 {
            return Err(Bad::Feat(i, c[5].to_string()));
        }
        let rel_s = match ewt_rel(c[7]) {
            Some(r) => {
                EWT_FIXED.fetch_add(1, Ordering::Relaxed);
                r
            }
            None => c[7],
        };
        let rel = Rel::parse(rel_s).ok_or_else(|| Bad::Rel(i, c[7].to_string()))?;
        let head: usize = c[6].parse().map_err(|_| Bad::Head(i))?;
        a.push(Tok { form: c[1].to_string(), lemma: c[2].to_string(), upos, tag, feats, head, rel });
    }
    check(&a, forms, reg)?;
    Ok(a)
}

/// Gates for forms, registry and tree.
pub fn check(a: &Ann, forms: &[String], reg: &Registry) -> Result<(), Bad> {
    if a.len() != forms.len() {
        return Err(Bad::Count(a.len(), forms.len()));
    }
    for (i, t) in a.iter().enumerate() {
        if t.form != forms[i] {
            return Err(Bad::Form(i + 1));
        }
        if t.head > a.len() || t.head == i + 1 {
            return Err(Bad::Head(i + 1));
        }
        if let Some(m) = reg.check(t.upos, t.feats, t.rel, &t.lemma).into_iter().next() {
            return Err(Bad::Registry(i + 1, m));
        }
    }
    let roots = a.iter().filter(|t| t.head == 0).count();
    if roots != 1 {
        return Err(Bad::Roots(roots));
    }
    for i in 1..=a.len() {
        let mut j = i;
        for _ in 0..=a.len() {
            j = a[j - 1].head;
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

/// Words for the CoNLL 2018 metrics (`en::ud::Scores`).
pub fn words(a: &Ann) -> Vec<en::ud::Word> {
    a.iter().map(|t| en::ud::Word { upos: Some(t.upos), tag: Some(t.tag), feats: t.feats, lemma: t.lemma.clone(), head: t.head, rel: t.rel }).collect()
}

/// Disagreements between two annotations — for the Opus second look.
pub fn diff(a: &Ann, b: &Ann) -> Vec<String> {
    let mut out = Vec::new();
    for (i, (x, y)) in a.iter().zip(b).enumerate() {
        let mut d = Vec::new();
        if x.upos != y.upos {
            d.push(format!("UPOS {} vs {}", x.upos, y.upos));
        }
        if x.tag != y.tag {
            d.push(format!("XPOS {} vs {}", x.tag, y.tag));
        }
        if x.feats != y.feats {
            d.push(format!("FEATS {} vs {}", x.feats, y.feats));
        }
        if x.lemma.to_lowercase() != y.lemma.to_lowercase() {
            d.push(format!("LEMMA {} vs {}", x.lemma, y.lemma));
        }
        if x.head != y.head || x.rel != y.rel {
            d.push(format!("HEAD/DEPREL {}/{} vs {}/{}", x.head, x.rel, y.head, y.rel));
        }
        if !d.is_empty() {
            out.push(format!("token {} «{}»: {}", i + 1, x.form, d.join("; ")));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(form: &str, upos: UPos, tag: Tag, feats: &str, head: usize, rel: Rel) -> Tok {
        Tok { form: form.into(), lemma: form.to_lowercase(), upos, tag, feats: Feats::parse(feats).0, head, rel }
    }

    #[test]
    fn gates_catch_bad_trees_and_registry() {
        let reg = Registry::load();
        let forms: Vec<String> = ["It", "rains", "."].iter().map(|s| s.to_string()).collect();
        let ok = vec![
            t("It", UPos::PRON, Tag::PRP, "Case=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs", 2, Rel::Nsubj),
            Tok { lemma: "rain".into(), ..t("rains", UPos::VERB, Tag::VBZ, "Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin", 0, Rel::Root) },
            t(".", UPos::PUNCT, Tag::Stop, "_", 2, Rel::Punct),
        ];
        assert_eq!(check(&ok, &forms, &reg), Ok(()));
        // negative controls: two roots, a cycle, a foreign form, a feature not for this part of speech
        let mut two = ok.clone();
        two[0].head = 0;
        two[0].rel = Rel::Root;
        assert_eq!(check(&two, &forms, &reg), Err(Bad::Roots(2)));
        let mut cyc = ok.clone();
        cyc[0].head = 3;
        cyc[2].head = 1;
        assert_eq!(check(&cyc, &forms, &reg), Err(Bad::Cycle(1)));
        let mut wrong = ok.clone();
        wrong[0].form = "He".into();
        assert_eq!(check(&wrong, &forms, &reg), Err(Bad::Form(1)));
        let mut feat = ok.clone();
        feat[2].feats = Feats::parse("Tense=Past").0;
        assert!(matches!(check(&feat, &forms, &reg), Err(Bad::Registry(3, _))));
        // write → parse → the same
        let block = write_block("s1", &ok);
        let (_, lines) = split_blocks(&block).remove(0);
        assert_eq!(parse_block(&lines, &forms, &reg), Ok(ok.clone()));
        // det:poss → nmod:poss; another foreign subtype is rejected
        let its: Vec<String> = ["Its", "rain", "."].iter().map(|s| s.to_string()).collect();
        let poss = |rel: &str| vec![
            format!("1\tIts\tits\tPRON\tPRP$\tCase=Gen|Gender=Neut|Number=Sing|Person=3|Poss=Yes|PronType=Prs\t2\t{rel}\t_\t_"),
            "2\train\train\tNOUN\tNN\tNumber=Sing\t0\troot\t_\t_".to_string(),
            "3\t.\t.\tPUNCT\t.\t_\t2\tpunct\t_\t_".to_string(),
        ];
        assert_eq!(parse_block(&poss("det:poss"), &its, &reg).map(|a| a[0].rel), Ok(Rel::NmodPoss));
        assert_eq!(parse_block(&poss("det:foo"), &its, &reg), Err(Bad::Rel(1, "det:foo".into())));
        assert_eq!(ewt_rel("obl:tmod"), Some("obl:unmarked"));
        assert_eq!(ewt_rel("nmod:npmod"), Some("nmod:unmarked"));
        assert_eq!(ewt_rel("obl"), None);
    }
}
