//! Tests of the 0.3 additions: the StepGame lexical reader and its formats, the grammar constraints of UD English,
//! narrower variants of rules (the negative list), the family's scoped changes and pool lines. Each check has its
//! negative control: the same function must say "no" where it should.

use en::annotate::Word;
use en::gram::{Feats, Rel, Tag, UPos};

fn w(form: &str, upos: UPos, head: usize, rel: Rel) -> Word {
    Word { form: form.into(), lemma: form.to_lowercase(), upos, tag: Tag::NN, feats: Feats::default(), head, rel }
}

#[test]
fn stepgame_lexical_reader_reads_new_phrasings() {
    use crate::stepgame::lexical;
    // vector of the first mentioned agent relative to the second
    assert_eq!(lexical("Q is up and to the left of Z.", 'Q', 'Z'), Some((-1, 1)));
    assert_eq!(lexical("Diagonally, Q lies southeast of Z.", 'Q', 'Z'), Some((1, -1)));
    // the reverse view: Z is the reference, Q on its left → Z is right of Q
    assert_eq!(lexical("Z has Q immediately on its left.", 'Z', 'Q'), Some((1, 0)));
    assert_eq!(lexical("From Z, go down one and right one to find Q.", 'Z', 'Q'), Some((-1, 1)));
    // "right" before under/on/where means "directly", not a side
    assert_eq!(lexical("Q rests right under Z.", 'Q', 'Z'), Some((0, -1)));
    assert_eq!(lexical("Q and Z coincide.", 'Q', 'Z'), Some((0, 0)));
    // negative controls: clock faces and conflicting words are left to the perceptron
    assert_eq!(lexical("Q is at the 3 o'clock position relative to Z.", 'Q', 'Z'), None);
    assert_eq!(lexical("Q is left of Z and right of it.", 'Q', 'Z'), None);
    assert_eq!(lexical("Q waits for Z.", 'Q', 'Z'), None);
}

#[test]
fn stepgame_phrasing_and_corrected_format() {
    use crate::stepgame::{load_txt, phrasing};
    assert_eq!(phrasing("K is above N."), "a1 is above a2".replace("a1", "A1").replace("a2", "A2"));
    assert_eq!(phrasing("K is above N."), phrasing("B is above C."));
    assert_ne!(phrasing("K is above N."), phrasing("K is below N."));
    let items = load_txt("1 O is to the left of Z.\n2 What is the relation of the agent O to the agent Z?\tleft\t1\n1 A is above B.\n2 B is above C.\n3 What is the relation of the agent A to the agent C?\tabove\t1\n");
    assert_eq!(items.len(), 2);
    assert_eq!(items[1].story.len(), 2, "a line numbered 1 starts a new story");
    assert_eq!(items[1].label, "above");
}

#[test]
fn grammar_constraints_catch_two_subjects_case_and_vocatives() {
    use crate::induce::violations;
    // "The man saw the dog ." — clean
    let ok = vec![w("The", UPos::DET, 2, Rel::Det), w("man", UPos::NOUN, 3, Rel::Nsubj), w("saw", UPos::VERB, 0, Rel::Root), w("the", UPos::DET, 5, Rel::Det), w("dog", UPos::NOUN, 3, Rel::Obj), w(".", UPos::PUNCT, 3, Rel::Punct)];
    assert_eq!(violations(&ok), 0);
    // two subjects of one verb
    let mut two = ok.clone();
    two[4].rel = Rel::Nsubj;
    assert!(violations(&two) >= 1);
    // a subject with a preposition ("at length she raised him": length as nsubj)
    let case = vec![w("At", UPos::ADP, 2, Rel::Case), w("length", UPos::NOUN, 4, Rel::Nsubj), w("she", UPos::PRON, 4, Rel::Nsubj), w("raised", UPos::VERB, 0, Rel::Root)];
    assert!(violations(&case) >= 2, "case-marked subject and two subjects");
    // the possessive 's is not a preposition
    let poss = vec![w("Bailey", UPos::PROPN, 3, Rel::Nsubj), w("'s", UPos::PART, 1, Rel::Case), w("trained", UPos::VERB, 0, Rel::Root)];
    assert_eq!(violations(&poss), 0);
    // a vocative as subject: "Look , sir !"
    let voc = vec![w("Look", UPos::VERB, 0, Rel::Root), w(",", UPos::PUNCT, 1, Rel::Punct), w("sir", UPos::NOUN, 1, Rel::Nsubj), w("!", UPos::PUNCT, 1, Rel::Punct)];
    assert!(violations(&voc) >= 1);
    let mut voc_ok = voc.clone();
    voc_ok[2].rel = Rel::Vocative;
    assert_eq!(violations(&voc_ok), 0, "negative control: the right label breaks nothing");
}

#[test]
fn narrower_variants_and_scoped_changes() {
    use crate::induce::{Change, parse_rules};
    let r = |l: &str| parse_rules(&format!("{l}\n")).into_iter().next().unwrap().1;
    let bad = r("domain\tparataxis\tconj\t0:15,4:1");
    assert!(bad.covers(&r("domain\tparataxis\tconj\t0:15,4:1,14:0")), "a narrower variant");
    assert!(!bad.covers(&r("domain\tparataxis\tconj\t4:1")), "a wider rule is not a variant");
    assert!(!bad.covers(&r("domain\tparataxis\tacl\t0:15,4:1")), "other labels");
    // a change keeps working on another rule set
    let base = vec![r("general\tnsubj\tobl\t17:1")];
    let add = Change::Add(r("general\tobj\tobl\t17:1"));
    assert_eq!(add.apply_to(&base).len(), 2);
    assert!(add.holds_in(&add.apply_to(&base)));
    let rm = Change::Remove(base[0].clone());
    assert!(rm.apply_to(&base).is_empty());
    let rep = Change::Replace(base[0].clone(), r("general\tnsubj\tobl\t17:1,0:7"));
    assert_eq!(rep.apply_to(&base)[0], r("general\tnsubj\tobl\t17:1,0:7"));
}
