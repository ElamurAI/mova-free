//! UD trees for the question reader: the `en` annotator (variable `WORLD_EN_MODEL`, otherwise
//! `$MOVA_DATA/en/models/en.bin` — trained on EWT + ESLSpok + the school's parse database, 15.10: the student
//! answers 43.4% of the lesson tests with it against 39.8% with the 0.3 model, EWT + ESLSpok only), loaded once per process. Tokens — `en::tok`,
//! lowercase, like FairytaleQA texts. No model — an error, without a silent fallback to rules.

use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use en::annotate::{Annotator, Word};
pub use en::gram::{Rel, Tag as PTag, UPos};

pub fn model_path() -> PathBuf {
    std::env::var("WORLD_EN_MODEL")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("en/models/en.bin"))
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
        // matrix-driven label repairs (inverted subject, bare oblique): `rerank::repairs`, docs/knowledge.md#the-absurdity-matrix
        if std::env::var("WORLD_NO_REPAIR").is_err() {
            crate::rerank::repair_words(&mut w);
        }
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

    /// Text with its case kept (the lessons are cased, so names stay PROPN) → full UD; forms and lemmas lower case.
    pub fn parse_cased(a: &Annotator, text: &str) -> Tree {
        let forms: Vec<String> = a.tokenize(text).into_iter().map(|t| t.form).collect();
        if forms.is_empty() {
            return Tree::default();
        }
        let mut w = a.annotate(&forms);
        if std::env::var("WORLD_NO_REPAIR").is_err() {
            crate::rerank::repair_words(&mut w);
        }
        for x in &mut w {
            x.form = x.form.to_lowercase();
            x.lemma = x.lemma.to_lowercase();
        }
        Tree::of_words(w)
    }

    /// A tree from annotated words (heads 1-based).
    pub fn of_words(w: Vec<Word>) -> Tree {
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
            // Each node occurs in at most one child list. A reachable cycle must
            // therefore contain the start; suppress its closing edge.
            st.extend(self.kids[x].iter().copied().filter(|child| *child != i));
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

    /// The tree on one line: `index:form/UPOS/relation>head` (for development).
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

#[cfg(test)]
mod wave7_tree {
    use super::*;
    fn word(head: usize, rel: Rel, upos: UPos, form: &str) -> Word {
        Word { head, rel, upos, form: form.into(), lemma: form.into(), tag: PTag::NN, feats: Default::default() }
    }
    #[test]
    fn head_fibers_and_child_order() {
        for a in 0..6 { for b in 0..6 { for c in 0..6 {
            let heads = [a,b,c];
            let t = Tree::of_words(heads.iter().map(|h| word(*h, Rel::Obj, UPos::NOUN, "x")).collect());
            assert_eq!(t.root(), heads.iter().position(|h| *h == 0));
            for parent in 0..3 {
                let expected: Vec<_> = (0..3).filter(|i| heads[*i] == parent + 1).collect();
                assert_eq!(t.kids(parent), expected);
                assert_eq!(t.kids_of(parent, Rel::Obj), expected);
                assert_eq!(t.head(parent), match heads[parent] { 1=>Some(0),2=>Some(1),3=>Some(2),_=>None });
            }
        }}}
        let fiber: Vec<_> = (0..6).filter(|h| Tree::of_words(vec![word(*h,Rel::Obj,UPos::NOUN,"x")]).head(0).is_none()).collect();
        assert_eq!(fiber, vec![0,2,3,4,5]);
    }
    #[test]
    fn forest_subtree_cut_and_dominance() {
        for b in 0..2 { for c in 0..3 { for d in 0..4 {
            let heads = [0,b,c,d];
            let t = Tree::of_words(heads.iter().map(|h| word(*h,Rel::Obj,UPos::NOUN,"x")).collect());
            for root in 0..4 { for mask in 0..16 {
                let cut: Vec<_>=(0..4).filter(|i| mask & (1<<i) != 0).collect();
                let mut expected=vec![];
                for node in 0..4 {
                    let mut p=node;
                    let mut reachable=false;
                    loop {
                        if p==root { reachable=true; break; }
                        if cut.contains(&p) || heads[p]==0 { break; }
                        p=heads[p]-1;
                    }
                    if reachable { expected.push(node); }
                }
                assert_eq!(t.sub(root,&cut),expected);
                if mask==0 { for node in 0..4 { assert_eq!(t.dominates(root,node),expected.contains(&node)); } }
            }}
        }}}
    }
    #[test]
    fn cycles_have_bounded_dominance() {
        let t=Tree::of_words(vec![word(2,Rel::Obj,UPos::NOUN,"a"),word(1,Rel::Obj,UPos::NOUN,"b"),word(0,Rel::Obj,UPos::NOUN,"c")]);
        assert!(t.dominates(0,1));
        assert!(!t.dominates(2,0));
        assert_eq!(t.root(),Some(2));
        assert_eq!(t.sub(0, &[]), vec![0, 1]);
        assert_eq!(t.sub(0, &[0]), vec![0, 1]);
        assert_eq!(t.sub(0, &[1]), vec![0]);
    }
    #[test]
    fn observations_and_index_error() {
        let t=Tree::of_words(vec![word(0,Rel::Obj,UPos::NOUN,"Oak"),word(1,Rel::Obj,UPos::DET,"the")]);
        assert_eq!(t.text(&[1,0,0]),"the Oak Oak");
        assert_eq!(t.form(0),"Oak"); assert_eq!(t.lemma(0),"Oak");
        assert!(t.content(0)); assert!(!t.content(1));
        assert_eq!(t.tag(0),PTag::NN);
        assert_eq!(cap("ßeta"),"SSeta"); assert_eq!(cap(""),"");
        assert!(std::panic::catch_unwind(||t.head(2)).is_err());
        assert!(Tree::default().is_empty());
        assert_eq!(Tree::default().root(),None);
    }

    #[test]
    fn functional_graph_sub_matches_visited_reachability() {
        let mut comparisons = 0;
        for n in 1usize..=4 {
            let radix = n + 2; // Root zero, every valid head, and one invalid head.
            for code in 0..radix.pow(n as u32) {
                let mut digits = code;
                let heads: Vec<_> = (0..n).map(|_| {
                    let h = digits % radix;
                    digits /= radix;
                    h
                }).collect();
                let t = Tree::of_words(heads.iter().map(|h| word(*h, Rel::Obj, UPos::NOUN, "x")).collect());
                for root in 0..n {
                    for mask in 0..(1usize << n) {
                        let cut: Vec<_> = (0..n).filter(|i| mask & (1 << i) != 0).collect();
                        // Independent graph search: scan raw heads, mark before
                        // expansion, and never use the implementation's kids.
                        let mut seen = vec![false; n];
                        let mut queue = vec![root];
                        while let Some(x) = queue.pop() {
                            if seen[x] || (x != root && cut.contains(&x)) { continue; }
                            seen[x] = true;
                            for child in 0..n {
                                if heads[child] == x + 1 { queue.push(child); }
                            }
                        }
                        let expected: Vec<_> = (0..n).filter(|i| seen[*i]).collect();
                        assert_eq!(t.sub(root, &cut), expected, "heads={heads:?}, root={root}, cut={cut:?}");
                        comparisons += 1;
                    }
                }
            }
        }
        assert_eq!(comparisons, 86078);
    }
}
