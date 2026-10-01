//! Rust annotator of full UD on fixed tokens: PTB tagger, parser, lemmas from the built-in dictionary,
//! UPOS and FEATS — `en::ud` tables over the graph (with subject–verb agreement).
//!
//! Shared path for everyone who annotates with Rust without Opus: the `annot` draft and bronze
//! (`rust_annotate` there — the same order of steps) and the stories. Training — as in `annot`:
//! UD sentences with PTB tags, first the `Ud` tables, then the model.

use std::path::Path;

use anyhow::{Context, Result};

use crate::conllu::{self, Sentence};
use crate::gram::{Feats, Rel, Tag, UPos};
use crate::model::Model;
use crate::tok::{self, Tok};
use crate::ud::{Ud, View, kids_of};

/// An annotated word of full UD.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Word {
    pub form: String,
    pub lemma: String,
    pub upos: UPos,
    pub tag: Tag,
    pub feats: Feats,
    /// Head number (0 — root).
    pub head: usize,
    pub rel: Rel,
}

/// Trained annotator: the `en` model (tokenizer, tagger, parser, morphology) and the UPOS/FEATS tables.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Annotator {
    pub model: Model,
    pub ud: Ud,
}

impl Annotator {
    /// Training on UD sentences; only sentences with PTB tags are used (`Sentence::tagged`).
    pub fn train(sents: Vec<Sentence>) -> Annotator {
        let sents: Vec<Sentence> = sents.into_iter().filter(Sentence::tagged).collect();
        let ud = Ud::train(&sents);
        let model = Model::train(sents);
        Annotator { model, ud }
    }

    /// Training on CoNLL-U files (EWT train, ESLSpok train…).
    pub fn train_files<P: AsRef<Path>>(paths: &[P]) -> Result<Annotator> {
        let mut sents = Vec::new();
        for p in paths {
            sents.extend(conllu::read(p.as_ref())?.into_iter().filter(Sentence::tagged));
        }
        Ok(Annotator::train(sents))
    }

    /// Write to a model file (`store`: header with the dictionary fingerprint and checksum, then the contents). Writes via
    /// a temporary file alongside, so an interrupted write leaves no half-file under the real name.
    pub fn save(&self, path: &Path) -> Result<()> {
        let bytes = crate::store::pack(self)?;
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir).with_context(|| format!("dir {}", dir.display()))?;
        }
        let tmp = path.with_extension("part");
        std::fs::write(&tmp, &bytes).with_context(|| format!("writing {}", tmp.display()))?;
        std::fs::rename(&tmp, path).with_context(|| format!("renaming to {}", path.display()))?;
        Ok(())
    }

    /// Trained annotator from a `save` file — the same annotation as a freshly trained one.
    pub fn load(path: &Path) -> Result<Annotator> {
        let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        Self::from_bytes(&bytes).with_context(|| format!("model {}", path.display()))
    }

    /// The same from file bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Annotator> {
        Ok(crate::store::unpack(bytes)?)
    }

    /// Model file bytes (what `save` writes).
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(crate::store::pack(self)?)
    }

    /// Text tokens — the `en::tok` tokenizer with this model's dictionaries.
    pub fn tokenize(&self, text: &str) -> Vec<Tok> {
        tok::tokenize(text, &self.model.lx)
    }

    /// Full UD on the given tokens.
    pub fn annotate<S: AsRef<str>>(&self, forms: &[S]) -> Vec<Word> {
        annotate(&self.model, &self.ud, forms)
    }
}

/// Full UD on fixed tokens: tagger → parser → UPOS (from the node's children) → FEATS per sentence → lemmas.
pub fn annotate<S: AsRef<str>>(model: &Model, ud: &Ud, forms: &[S]) -> Vec<Word> {
    let words: Vec<&str> = forms.iter().map(AsRef::as_ref).collect();
    let tags = model.tagger.tag(&words);
    let tree = model.parser.parse(&words, &tags);
    let heads: Vec<usize> = tree.iter().map(|x| x.0).collect();
    let rels: Vec<Rel> = tree.iter().map(|x| x.1).collect();
    let kids = kids_of(&heads, &rels);
    let upos: Vec<UPos> = (0..words.len()).map(|i| ud.upos(&View { form: words[i], tag: tags[i], rel: rels[i], kids: kids[i], agr: 0 })).collect();
    let feats = ud.feats_sentence(&words, &tags, &upos, &heads, &rels);
    words
        .iter()
        .enumerate()
        .map(|(i, w)| Word { form: w.to_string(), lemma: model.morph.lemmatize(w, tags[i]), upos: upos[i], tag: tags[i], feats: feats[i], head: heads[i], rel: rels[i] })
        .collect()
}

/// CoNLL-U lines of a sentence — ten columns: DEPS "_", MISC — `SpaceAfter=No` if there is no space after the token
/// in the text (`space_after[i] == false`), otherwise "_".
pub fn rows(words: &[Word], space_after: &[bool]) -> Vec<[String; 10]> {
    words
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let misc = if space_after.get(i).copied().unwrap_or(true) { "_" } else { "SpaceAfter=No" };
            [
                (i + 1).to_string(),
                w.form.clone(),
                w.lemma.clone(),
                w.upos.to_string(),
                w.tag.to_string(),
                w.feats.to_string(),
                w.head.to_string(),
                w.rel.to_string(),
                "_".into(),
                misc.into(),
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny annotator on two sentences: forms and word count are preserved, the tree has one
    /// root, CoNLL-U lines have ten columns with `SpaceAfter=No` where there is no space.
    #[test]
    fn annotate_keeps_tokens_and_writes_rows() {
        let text = "1\tIt\tit\tPRON\tPRP\tCase=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs\t2\tnsubj\t_\t_\n\
                    2\trains\train\tVERB\tVBZ\tMood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin\t0\troot\t_\tSpaceAfter=No\n\
                    3\t.\t.\tPUNCT\t.\t_\t2\tpunct\t_\t_\n\n";
        let doc = conllu::Doc::parse(&format!("{text}{text}")).unwrap();
        let a = Annotator::train(doc.sents.iter().map(|s| s.sent.clone()).collect());
        let toks = a.tokenize("It rains.");
        let forms: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
        assert_eq!(forms, ["It", "rains", "."]);
        let w = a.annotate(&forms);
        assert_eq!(w.iter().map(|x| x.form.as_str()).collect::<Vec<_>>(), forms);
        assert_eq!(w.iter().filter(|x| x.head == 0).count(), 1);
        let r = rows(&w, &toks.iter().map(|t| t.space_after).collect::<Vec<_>>());
        assert_eq!(r.len(), 3);
        assert_eq!(r[1][9], "SpaceAfter=No");
        assert_eq!((r[0][0].as_str(), r[0][8].as_str(), r[0][9].as_str()), ("1", "_", "_"));
    }

    /// A CoNLL-U sentence from lines "form lemma UPOS XPOS FEATS head relation".
    fn sent(rows: &str) -> String {
        let mut out = String::new();
        for (i, l) in rows.lines().map(str::trim).filter(|l| !l.is_empty()).enumerate() {
            let c: Vec<&str> = l.split_whitespace().collect();
            out += &format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t_\t_\n", i + 1, c[0], c[1], c[2], c[3], c[4], c[5], c[6]);
        }
        out + "\n"
    }

    fn train_on(text: &str) -> Annotator {
        let doc = conllu::Doc::parse(text).unwrap();
        Annotator::train(doc.sents.iter().map(|s| s.sent.clone()).collect())
    }

    /// A model from file annotates the same as a freshly trained one: tags, lemmas, UPOS, FEATS, tree.
    /// Negative control:
    /// - a model trained on other data annotates differently — the comparison sees it;
    /// - a corrupted byte and a truncated file do not load.
    #[test]
    fn saved_model_annotates_the_same() {
        let a1 = sent("She she PRON PRP Case=Nom|Gender=Fem|Number=Sing|Person=3|PronType=Prs 2 nsubj
                       saw see VERB VBD Mood=Ind|Number=Sing|Person=3|Tense=Past|VerbForm=Fin 0 root
                       the the DET DT Definite=Def|PronType=Art 4 det
                       dogs dog NOUN NNS Number=Plur 2 obj
                       . . PUNCT . _ 2 punct");
        let a2 = sent("Dogs dog NOUN NNS Number=Plur 2 nsubj
                       run run VERB VBP Mood=Ind|Number=Plur|Person=3|Tense=Pres|VerbForm=Fin 0 root
                       home home ADV RB _ 2 advmod
                       . . PUNCT . _ 2 punct");
        let a = train_on(&format!("{a1}{a2}{a1}{a2}{a1}"));
        let bytes = a.to_bytes().unwrap();
        let b = Annotator::from_bytes(&bytes).unwrap();
        let texts = [&["She", "saw", "the", "dogs", "."][..], &["Dogs", "run", "home", "."], &["The", "cats", "saw", "her", "duck", "."]];
        for forms in texts {
            assert_eq!(a.annotate(forms), b.annotate(forms), "{forms:?}");
        }
        // negative control: another model — another annotation
        let other = train_on(&sent("She she PRON PRP _ 0 root
                                    saw saw NOUN NN Number=Sing 1 obj
                                    the the DET DT _ 4 det
                                    dogs dogs PROPN NNP Number=Sing 2 nmod
                                    . . PUNCT . _ 1 punct").repeat(3));
        assert_ne!(other.annotate(texts[0]), b.annotate(texts[0]));
        let mut bad = bytes.clone();
        let mid = bytes.len() / 2;
        bad[mid] ^= 1;
        assert!(Annotator::from_bytes(&bad).is_err());
        assert!(Annotator::from_bytes(&bytes[..bytes.len() - 1]).is_err());
    }
}
