//! UD trees for the question reader: the `en` annotator (model `ud-ewt-eslspok.bin`; variable `WORLD_EN_MODEL`,
//! otherwise `data/en/models/ud-ewt-eslspok.bin`), loaded once per process. Tokens — `en::tok`,
//! lowercase, like FairytaleQA texts. No model — an error, without a silent fallback to rules.

use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use en::annotate::{Annotator, Word};
pub use en::gram::{Rel, Tag as PTag, UPos};

pub fn model_path() -> PathBuf {
    std::env::var("WORLD_EN_MODEL")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../en/models/ud-ewt-eslspok.bin")))
}

static ANN: OnceLock<Result<Annotator, String>> = OnceLock::new();

/// The `en` annotator (once per process).
pub fn annotator() -> Result<&'static Annotator> {
    ANN.get_or_init(|| Annotator::load(&model_path()).map_err(|e| format!("{e:#}"))).as_ref().map_err(|e| anyhow!("en model {}: {e}", model_path().display()))
}

/// A sentence with a UD tree: words (lowercase lemma) and the children of each node.
#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub w: Vec<Word>,
    kids: Vec<Vec<usize>>,
}

impl Tree {
    /// Text tokens (`en::tok`) → full UD.
    pub fn parse(a: &Annotator, text: &str) -> Tree {
        let forms: Vec<String> = a.tokenize(text).into_iter().map(|t| t.form.to_lowercase()).collect();
        Tree::of_forms(a, &forms)
    }

    pub fn of_forms(a: &Annotator, forms: &[String]) -> Tree {
        if forms.is_empty() {
            return Tree::default();
        }
        // FairytaleQA texts are lowercase; the tagger was trained on cased EWT, so the first word and
        // words outside the `en` lexicon (names: assipattle, kittlerumpit) get a capital letter
        let cased: Vec<String> = forms
            .iter()
            .enumerate()
            .map(|(i, f)| if i == 0 || f == "i" || name_like(f) { cap(f) } else { f.clone() })
            .collect();
        let mut w = a.annotate(&cased);
        for x in &mut w {
            x.form = x.form.to_lowercase();
            x.lemma = x.lemma.to_lowercase();
        }
        let mut kids = vec![Vec::new(); w.len()];
        for (i, x) in w.iter().enumerate() {
            if x.head > 0 && x.head <= kids.len() {
                kids[x.head - 1].push(i);
            }
        }
        Tree { w, kids }
    }

    pub fn len(&self) -> usize {
        self.w.len()
    }

    pub fn is_empty(&self) -> bool {
        self.w.is_empty()
    }

    pub fn head(&self, i: usize) -> Option<usize> {
        let h = self.w[i].head;
        (h > 0 && h <= self.w.len()).then(|| h - 1)
    }

    pub fn root(&self) -> Option<usize> {
        self.w.iter().position(|x| x.head == 0)
    }

    /// Relation without subtype (aux:pass → aux).
    pub fn base(&self, i: usize) -> Rel {
        self.w[i].rel.base()
    }

    pub fn kids(&self, i: usize) -> &[usize] {
        &self.kids[i]
    }

    pub fn kids_of(&self, i: usize, r: Rel) -> Vec<usize> {
        self.kids[i].iter().copied().filter(|k| self.base(*k) == r).collect()
    }

    pub fn form(&self, i: usize) -> &str {
        &self.w[i].form
    }

    pub fn lemma(&self, i: usize) -> &str {
        &self.w[i].lemma
    }

    pub fn upos(&self, i: usize) -> UPos {
        self.w[i].upos
    }

    pub fn tag(&self, i: usize) -> PTag {
        self.w[i].tag
    }

    /// Subtree of `i` in word order, without the subtrees of `cut` nodes (`i` itself stays).
    pub fn sub(&self, i: usize, cut: &[usize]) -> Vec<usize> {
        let mut out = Vec::new();
        let mut st = vec![i];
        while let Some(x) = st.pop() {
            if x != i && cut.contains(&x) {
                continue;
            }
            out.push(x);
            st.extend(self.kids[x].iter().copied());
        }
        out.sort_unstable();
        out
    }

    /// `a` is an ancestor of `b` (or `b` itself).
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        for _ in 0..self.w.len() + 1 {
            if a == b {
                return true;
            }
            match self.head(b) {
                Some(h) => b = h,
                None => return false,
            }
        }
        false
    }

    /// Text of the nodes joined by spaces (FairytaleQA tokens).
    pub fn text(&self, ix: &[usize]) -> String {
        ix.iter().map(|i| self.w[*i].form.as_str()).collect::<Vec<_>>().join(" ")
    }

    /// Content word by UPOS (noun, verb, adjective, adverb, number, proper noun).
    pub fn content(&self, i: usize) -> bool {
        matches!(self.w[i].upos, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ | UPos::ADV | UPos::NUM)
    }

    /// The tree on one line: `form/lemma/UPOS/head/relation` (for development).
    pub fn show(&self) -> String {
        self.w.iter().enumerate().map(|(i, x)| format!("{}:{}/{}/{}>{}", i + 1, x.form, x.upos, x.rel, x.head)).collect::<Vec<_>>().join(" ")
    }
}

/// A word outside the `en` lexicon without common English suffixes is most likely a name (assipattle, kittlerumpit);
/// "ashamed", "unaccustomed" are not.
fn name_like(f: &str) -> bool {
    const SUF: &[&str] = &["ed", "ing", "ly", "ness", "ful", "less", "ous", "ive", "able", "ible", "tion", "sion", "ment", "est", "ish", "ity", "ies", "ize", "ise"];
    f.len() > 2 && f.chars().all(|c| c.is_ascii_lowercase()) && en::dict::form(f).is_none() && !SUF.iter().any(|x| f.ends_with(x))
}

fn cap(w: &str) -> String {
    let mut c = w.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}
