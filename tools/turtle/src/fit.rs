//! The fit check of a gap question's wrong options: a wrong option may be neither a near-synonym of the answer nor a
//! word that also fills the gap.
//!
//! **Near-synonyms** (never drawn, in any form). By Open English WordNet: the two words share a synset, or one is a
//! kind of the other (a synset of one among the hypernyms, at any depth, of a synset of the other: «mum» → mother →
//! parent). By the meanings: the answer's meaning names the candidate as a whole word, or the candidate's meaning
//! names the answer (the layer's own meaning column and every sense of the pupils' dictionary).
//!
//! **The slot.** Mova's own annotator reads the example with the answer in place; the answer's tag (PTB) and whether
//! it governs a direct object make the slot. A candidate is shown in a surface form that must break the slot:
//!
//! - **agreement / word class**: none of the form's tags in Mova's built-in lexicon (`en::dict`) can stand in the
//!   slot (`group`): after «to», a modal or «did», only a base form fits, so «says» or «gone» break it; a noun,
//!   adjective or adverb slot is broken only by a form that is nothing but a finite verb («went», «says»);
//! - **determiner agreement**: a noun slot after «a», «an», «one», «each», «every», «another», «this» or «that» is
//!   broken by a form that is only plural («a parents»), one after «these», «those», «many», «several», «both» or
//!   «few» by a form that is only singular, «an» by a noun spelt with a consonant other than h («an robot»), «a» by a
//!   noun spelt with a, e, i or o («a order»);
//! - **subcategorisation**: a verb slot with a direct object, and a candidate verb that WordNet never gives a
//!   transitive frame (`vt…`, `ditransitive`), in its base form.
//!
//! A meaning-frame rule (a noun of another kind than the answer) is not used: tried on the commands layer on
//! 2026-11-04 with WordNet's lexicographer files, it let «do you know the teacher?» stand for «… the time?»; a
//! permissive verb takes nouns of any kind. Anything else is refused with the reason (`judge`). The rules are
//! conservative: a refusal costs only a candidate, a wrong acceptance costs a question with two right answers.
use anyhow::{Context, Result};
use en::gram::Tag;
use std::collections::{BTreeMap, BTreeSet};

/// The determiner right before a gap, as far as agreement goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Det {
    None,
    /// «a»: singular, and the next word not spelt with a, e, i or o
    A,
    /// «an»: singular, and the next word spelt with a vowel or h
    An,
    /// one, each, every, another, this, that
    Singular,
    /// these, those, many, several, both, few
    Plural,
}

impl Det {
    pub fn of(word: &str) -> Det {
        match word {
            "a" => Det::A,
            "an" => Det::An,
            "one" | "each" | "every" | "another" | "this" | "that" => Det::Singular,
            "these" | "those" | "many" | "several" | "both" | "few" => Det::Plural,
            _ => Det::None,
        }
    }
}

/// The slot of a gap: the answer's tag in its sentence, whether it governs a direct object, the determiner before it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub tag: Tag,
    pub object: bool,
    pub det: Det,
}

impl Slot {
    /// A slot of this tag with no object and no determiner.
    pub const fn of(tag: Tag) -> Slot {
        Slot { tag, object: false, det: Det::None }
    }
}

/// What the lexicons say about the words of a layer (WordNet's synsets, hypernyms and verb frames; the meanings).
#[derive(Clone, Debug, Default)]
pub struct Lex {
    /// word → its WordNet synset ids (every part of speech)
    pub synsets: BTreeMap<String, BTreeSet<String>>,
    /// word → the synsets above its synsets (hypernyms at any depth)
    pub above: BTreeMap<String, BTreeSet<String>>,
    /// verb word → whether any of its senses has a transitive frame
    pub transitive: BTreeMap<String, bool>,
    /// word → its meanings, lower case (the layer's and the pupils' dictionary's)
    pub meanings: BTreeMap<String, String>,
}

/// The tags that can stand in a slot of this tag.
pub fn group(t: Tag) -> Vec<Tag> {
    use Tag::*;
    match t {
        VB | VBP => vec![VB, VBP, VBD, VBG, MD],
        VBZ => vec![VBZ, VBD, MD],
        VBD => vec![VBD, VBP, VBZ, MD],
        VBN => vec![VBN, VBD, JJ],
        VBG => vec![VBG, NN, NNS, JJ],
        // a noun, adjective, adverb, particle or anything else: every tag but the finite verbs'
        _ => Tag::ALL.iter().copied().filter(|x| !matches!(x, VBD | VBZ | MD)).collect(),
    }
}

/// The tags Mova's built-in lexicon gives a lower-case form (empty: unknown).
pub fn tags_of(form: &str) -> BTreeSet<&'static str> {
    en::dict::form(form).map(|f| en::dict::analyses(f).map(|r| r.tag.name()).collect()).unwrap_or_default()
}

/// A lemma's forms in Mova's built-in lexicon that are both standard and used (in AGID, which alone also lists
/// forms like «justs», and attested in UD, which alone also holds misspellings like «orderd»), lower case, letters
/// only, with the lemma itself; the finite verb forms first (they break most slots), then in name order.
pub fn forms_of(lemma: &str) -> Vec<String> {
    // the form's sources over all its records, and its best rank
    let mut by: BTreeMap<String, (u8, u8)> = BTreeMap::new();
    if let Some(l) = en::dict::lemma(lemma) {
        for r in en::dict::paradigm(l) {
            let f = en::dict::form_text(r.form()).to_lowercase();
            if f.is_empty() || !f.chars().all(char::is_alphabetic) {
                continue;
            }
            let rank = match r.tag {
                Tag::VBZ => 0,
                Tag::VBD => 1,
                Tag::VBN => 2,
                _ => 3,
            };
            let e = by.entry(f).or_insert((u8::MAX, 0));
            e.0 = e.0.min(rank);
            e.1 |= r.src;
        }
    }
    let mut out: BTreeSet<(u8, String)> = by.into_iter().filter(|(_, (_, src))| src & en::dict::UD != 0 && src & en::dict::AGID != 0).map(|(f, (rank, _))| (rank, f)).collect();
    out.insert((3, lemma.to_string()));
    let mut seen = BTreeSet::new();
    out.into_iter().filter(|(_, f)| seen.insert(f.clone())).map(|x| x.1).collect()
}

fn whole_words(s: &str) -> BTreeSet<String> {
    s.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-').filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

fn is_verb_slot(t: Tag) -> bool {
    t.is_verb()
}

fn is_noun_slot(t: Tag) -> bool {
    matches!(t, Tag::NN | Tag::NNS | Tag::NNP | Tag::NNPS)
}

impl Lex {
    /// Why `cand` is a near-synonym of `answer`, if it is.
    pub fn kin(&self, answer: &str, cand: &str) -> Option<String> {
        let empty = BTreeSet::new();
        let (sa, sc) = (self.synsets.get(answer).unwrap_or(&empty), self.synsets.get(cand).unwrap_or(&empty));
        if let Some(s) = sa.intersection(sc).next() {
            return Some(format!("WordNet: «{cand}» shares the synset {s} with «{answer}»"));
        }
        if sc.intersection(self.above.get(answer).unwrap_or(&empty)).next().is_some() {
            return Some(format!("WordNet: «{answer}» is a kind of «{cand}»"));
        }
        if sa.intersection(self.above.get(cand).unwrap_or(&empty)).next().is_some() {
            return Some(format!("WordNet: «{cand}» is a kind of «{answer}»"));
        }
        let names = |m: &str, w: &str| self.meanings.get(m).is_some_and(|t| whole_words(t).contains(w));
        if names(answer, cand) {
            return Some(format!("dictionary: the meaning of «{answer}» names «{cand}»"));
        }
        if names(cand, answer) {
            return Some(format!("dictionary: the meaning of «{cand}» names «{answer}»"));
        }
        None
    }

    /// The form of `cand` to show as a wrong option for `answer` in a slot, with the rule it breaks; or why it is
    /// refused. `sentence` holds the sentence's words (lower case): a form found there is never shown.
    pub fn judge(&self, answer: &str, cand: &str, slot: Slot, sentence: &BTreeSet<String>) -> std::result::Result<(String, &'static str), String> {
        if cand == answer {
            return Err("the answer itself".into());
        }
        if sentence.contains(cand) {
            return Err(format!("«{cand}» is in the sentence"));
        }
        if let Some(why) = self.kin(answer, cand) {
            return Err(why);
        }
        if is_noun_slot(slot.tag)
            && let Some(f) = self.det_breaker(cand, slot.det, sentence)
        {
            return Ok((f, "determiner agreement"));
        }
        if is_verb_slot(slot.tag) && slot.object && self.transitive.get(cand) == Some(&false) && matches!(slot.tag, Tag::VB | Tag::VBP) {
            return Ok((cand.to_string(), "subcategorisation"));
        }
        let g: BTreeSet<&str> = group(slot.tag).iter().map(|t| t.name()).collect();
        for f in forms_of(cand) {
            if f == answer || sentence.contains(&f) {
                continue;
            }
            let t = tags_of(&f);
            if !t.is_empty() && t.is_disjoint(&g) {
                return Ok((f, "agreement"));
            }
        }
        Err(format!("every form of «{cand}» can stand where «{answer}» ({}) stands", slot.tag.name()))
    }

    /// A form of the noun `cand` that the determiner before the gap refuses, if there is one.
    fn det_breaker(&self, cand: &str, det: Det, sentence: &BTreeSet<String>) -> Option<String> {
        let only = |f: &str, t: &str| {
            let tags = tags_of(f);
            tags.len() == 1 && tags.contains(t)
        };
        let first = cand.chars().next()?;
        let noun = tags_of(cand).contains("NN");
        match det {
            Det::An if noun && !"aeiouh".contains(first) => Some(cand.to_string()),
            Det::A if noun && "aeio".contains(first) => Some(cand.to_string()),
            Det::Plural if only(cand, "NN") => Some(cand.to_string()),
            Det::A | Det::An | Det::Singular => forms_of(cand).into_iter().find(|f| only(f, "NNS") && !sentence.contains(f)),
            _ => None,
        }
    }

    /// Read the lexicons for these words: WordNet (synsets and hypernyms through `world::wordnet`; the verb frames
    /// from the OEWN entry files), and the meanings given here plus the pupils' dictionary.
    pub fn load(words: &BTreeSet<String>, meanings: &BTreeMap<String, String>) -> Result<Lex> {
        let wn = world::wordnet::wordnet();
        let mut lex = Lex::default();
        for w in words {
            let ss: BTreeSet<String> = wn.senses.get(w).into_iter().flatten().map(|s| s.synset.clone()).collect();
            let mut above = BTreeSet::new();
            let mut level: Vec<String> = ss.iter().cloned().collect();
            while !level.is_empty() && above.len() < 4000 {
                let mut next = Vec::new();
                for s in &level {
                    for h in wn.up.get(s).map(|x| x.0.clone()).unwrap_or_default() {
                        if above.insert(h.clone()) {
                            next.push(h);
                        }
                    }
                }
                level = next;
            }
            lex.synsets.insert(w.clone(), ss);
            lex.above.insert(w.clone(), above);
            let mut m = meanings.get(w).cloned().unwrap_or_default().to_lowercase();
            for e in world::dictstore::store().get(w) {
                m.push(' ');
                m.push_str(&e.meaning.to_lowercase());
            }
            lex.meanings.insert(w.clone(), m);
        }
        let dir = world::wordnet::oewn_dir();
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
            .with_context(|| format!("{}", dir.display()))?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("entries-") && n.ends_with(".json")))
            .collect();
        files.sort();
        for f in files {
            let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&f).with_context(|| format!("{}", f.display()))?)?;
            let Some(obj) = v.as_object() else { continue };
            for (lemma, by_pos) in obj {
                let w = lemma.to_lowercase();
                if !words.contains(&w) {
                    continue;
                }
                for (pos, e) in by_pos.as_object().into_iter().flatten() {
                    for s in e["sense"].as_array().into_iter().flatten() {
                        if pos == "v" {
                            let vt = s["subcat"].as_array().into_iter().flatten().filter_map(|x| x.as_str()).any(|x| x.starts_with("vt") || x == "ditransitive");
                            let t = lex.transitive.entry(w.clone()).or_insert(false);
                            *t |= vt;
                        }
                    }
                }
            }
        }
        Ok(lex)
    }
}

/// The slot of the first whole-word `answer` in `sentence`, read by Mova's annotator; None when its tokens do not
/// hold the word.
pub fn slot_of(a: &en::annotate::Annotator, sentence: &str, answer: &str) -> Option<Slot> {
    let toks: Vec<String> = a.tokenize(sentence).into_iter().map(|t| t.form).collect();
    let i = toks.iter().position(|t| t.to_lowercase() == answer)?;
    let words = a.annotate(&toks);
    let w = words.get(i)?;
    let object = words.iter().any(|x| x.head == i + 1 && matches!(x.rel.base(), en::gram::Rel::Obj | en::gram::Rel::Iobj));
    let det = i.checked_sub(1).map_or(Det::None, |j| Det::of(&toks[j].to_lowercase()));
    Some(Slot { tag: w.tag, object, det })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn set(xs: &[&str]) -> BTreeSet<String> {
        xs.iter().map(|s| s.to_string()).collect()
    }
    fn lex() -> Lex {
        let mut l = Lex::default();
        // only/just share a synset (merely, simply, just, only); mum is a kind of parent; time has no person sense
        l.synsets.insert("only".into(), set(&["adv-merely"]));
        l.synsets.insert("just".into(), set(&["adv-merely", "adj-fair"]));
        l.synsets.insert("mum".into(), set(&["n-mother"]));
        l.above.insert("mum".into(), set(&["n-parent", "n-person"]));
        l.synsets.insert("parent".into(), set(&["n-parent"]));
        l.transitive.insert("happen".into(), false);
        l.transitive.insert("carry".into(), true);
        l
    }
    const VB: Slot = Slot::of(Tag::VB);
    const RB: Slot = Slot::of(Tag::RB);
    const NN: Slot = Slot::of(Tag::NN);

    #[test]
    fn near_synonyms_are_refused_red_controls() {
        let l = lex();
        let s = BTreeSet::new();
        assert!(l.judge("only", "just", RB, &s).unwrap_err().contains("shares the synset"));
        assert!(l.judge("just", "only", RB, &s).unwrap_err().contains("shares the synset"));
        assert!(l.judge("mum", "parent", NN, &s).unwrap_err().contains("«mum» is a kind of «parent»"));
        assert!(l.judge("parent", "mum", NN, &s).unwrap_err().contains("«mum» is a kind of «parent»"));
        // the meanings: one names the other
        let mut m = Lex::default();
        m.meanings.insert("only".into(), "only means just this one and nothing more".into());
        assert!(m.judge("only", "just", RB, &s).unwrap_err().contains("names «just»"));
        assert!(m.judge("just", "only", RB, &s).unwrap_err().contains("names «just»"));
        // negative control: with an empty lexicon «just» is not a near-synonym, and only the slot refuses it
        assert!(Lex::default().kin("only", "just").is_none());
    }

    #[test]
    fn a_wrong_option_must_break_the_slot() {
        let l = lex();
        let s = set(&["then", "pause"]);
        // «go» in a base-form slot: «say» fits, «says» breaks the agreement
        assert_eq!(l.judge("go", "say", VB, &s), Ok(("says".to_string(), "agreement")));
        // an adverb slot: «out» and «now» fit in every form they have; a finite verb breaks it
        assert!(l.judge("now", "out", RB, &s).unwrap_err().contains("can stand"));
        assert!(l.judge("only", "now", RB, &s).unwrap_err().contains("can stand"));
        let (f, why) = l.judge("now", "go", RB, &s).unwrap();
        assert!(["goes", "went"].contains(&f.as_str()) && why == "agreement", "{f}");
        // a noun slot: another noun fits, whatever its kind (no meaning-frame rule)
        assert!(l.judge("mum", "dad", NN, &s).is_err_and(|e| e.contains("can stand")));
        assert!(l.judge("time", "teacher", NN, &s).is_err_and(|e| e.contains("can stand")));
        // ...unless the determiner refuses it: «an robot», «a order», «a parents», «these robot»
        let det = |d| Slot { tag: Tag::NN, object: false, det: d };
        assert_eq!(l.judge("order", "robot", det(Det::An), &s), Ok(("robot".to_string(), "determiner agreement")));
        assert_eq!(l.judge("order", "hour", det(Det::An), &s), Ok(("hours".to_string(), "determiner agreement")), "«an hour» fits, «an hours» does not");
        assert_eq!(l.judge("robot", "order", det(Det::A), &s), Ok(("order".to_string(), "determiner agreement")));
        assert_eq!(l.judge("robot", "teacher", det(Det::A), &s), Ok(("teachers".to_string(), "determiner agreement")), "«a teacher» fits");
        assert_eq!(l.judge("robot", "teacher", det(Det::Singular), &s), Ok(("teachers".to_string(), "determiner agreement")));
        assert_eq!(l.judge("robots", "teacher", det(Det::Plural), &s), Ok(("teacher".to_string(), "determiner agreement")));
        // a verb with an object: an intransitive verb breaks the frame in its base form, a transitive one does not
        let obj = Slot { tag: Tag::VB, object: true, det: Det::None };
        assert_eq!(l.judge("carry", "happen", obj, &s), Ok(("happen".to_string(), "subcategorisation")));
        assert_ne!(l.judge("happen", "carry", obj, &s).map(|x| x.0), Ok("carry".to_string()));
        // a form in the sentence is never shown
        assert!(l.judge("go", "pause", VB, &s).is_err());
    }

    #[test]
    fn the_lexicon_knows_the_forms() {
        assert!(tags_of("says").contains("VBZ") && !tags_of("says").contains("VB"));
        assert!(tags_of("say").contains("VB"));
        assert!(forms_of("say").contains(&"said".to_string()));
        assert!(tags_of("qzxv").is_empty());
    }
}
