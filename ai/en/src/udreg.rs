//! UD 2.18 registry for English — the annotation gate. Facts are taken from the UD validator
//! (UniversalDependencies/tools, `data/*.json`) into `data/ud-registry-en.tsv`:
//! - which "feature=value" pairs are allowed for each part of speech;
//! - which relations (with subtypes) are allowed;
//! - which lemmas can be auxiliaries (AUX); only "be" can be a copula (`cop`).
//!
//! A violation is not a model error but invalid annotation: such a block does not go into the corpus.

use std::collections::HashSet;

use crate::gram::{Feat, Feats, Rel, UPos};

static SRC: &str = include_str!("../data/ud-registry-en.tsv");

pub struct Registry {
    feats: HashSet<(UPos, Feat)>,
    deprels: HashSet<String>,
    aux: HashSet<String>,
}

impl Registry {
    pub fn load() -> Registry {
        let (mut feats, mut deprels, mut aux) = (HashSet::new(), HashSet::new(), HashSet::new());
        for line in SRC.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
            let c: Vec<&str> = line.split('\t').collect();
            match c.as_slice() {
                ["feat", upos, pair] => {
                    if let (Some(u), Some(f)) = (UPos::parse(upos), Feat::parse(pair)) {
                        feats.insert((u, f));
                    }
                }
                ["deprel", r] => {
                    deprels.insert(r.to_string());
                }
                ["aux", lemma, ..] => {
                    aux.insert(lemma.to_string());
                }
                _ => {}
            }
        }
        Registry { feats, deprels, aux }
    }

    /// Registry violations for one word (empty — everything is allowed).
    pub fn check(&self, upos: UPos, feats: Feats, rel: Rel, lemma: &str) -> Vec<String> {
        let mut out = Vec::new();
        for f in feats.iter() {
            if !self.feats.contains(&(upos, f)) {
                out.push(format!("feature {f} is not allowed for {upos}"));
            }
        }
        if !self.deprels.contains(rel.name()) {
            out.push(format!("relation {rel} is outside the UD 2.18 registry for English"));
        }
        if upos == UPos::AUX && !self.aux.contains(&lemma.to_lowercase()) {
            out.push(format!("AUX with lemma '{lemma}' is not in the list of auxiliaries"));
        }
        if rel == Rel::Cop && lemma.to_lowercase() != "be" {
            out.push(format!("cop with lemma '{lemma}' — only 'be' can be a copula"));
        }
        out
    }

    /// Allowed features for a part of speech — for the annotator prompt.
    pub fn allowed(&self, upos: UPos) -> Vec<Feat> {
        Feat::ALL.iter().copied().filter(|f| self.feats.contains(&(upos, *f))).collect()
    }

    pub fn deprels(&self) -> Vec<String> {
        let mut v: Vec<String> = self.deprels.iter().cloned().collect();
        v.sort();
        v
    }

    pub fn auxiliaries(&self) -> Vec<String> {
        let mut v: Vec<String> = self.aux.iter().cloned().collect();
        v.sort();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_gate() {
        let r = Registry::load();
        let f = |s: &str| Feats::parse(s).0;
        // allowed
        assert!(r.check(UPos::PRON, f("Case=Nom|Number=Sing|Person=1|PronType=Prs"), Rel::Nsubj, "I").is_empty());
        assert!(r.check(UPos::AUX, f("Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin"), Rel::Cop, "be").is_empty());
        // negative controls
        assert!(!r.check(UPos::NOUN, f("Tense=Past"), Rel::Obj, "dog").is_empty());
        assert!(!r.check(UPos::AUX, Feats::default(), Rel::Aux, "go").is_empty());
        assert!(!r.check(UPos::AUX, Feats::default(), Rel::Cop, "seem").is_empty());
        assert!(r.deprels().len() >= 50 && r.auxiliaries().contains(&"will".to_string()));
    }
}
