//! Tests: every layer's exact decode gives its input back, a damaged residual does not (the gate shows red), and a
//! decoder that drops a class shows that class's loss. The tests that need the `en` model run when the model file is
//! there (`$MOVA_DATA/en/models/en.bin`) and say so when it is not.

use std::collections::BTreeMap;

use en::gram::{Rel, Tag};
use prag::frames::{Kind, Link};

use crate::class::Class;
use crate::frames::{self, E, F, Step};
use crate::tally::{Residual, Start};
use crate::tree::{self, TNode};
use crate::world;

fn n(lemma: &str, tag: Tag, head: usize, rel: Rel) -> TNode {
    TNode { lemma: lemma.into(), tag, head, rel }
}

fn e(id: &str) -> E {
    E { id: id.into(), ..Default::default() }
}

fn f(kind: Kind, verb: &str, agent: Option<E>, patient: Option<E>) -> F {
    F { kind, verb: verb.into(), agent, patient, ..Default::default() }
}

#[test]
fn orders_and_projectivity() {
    // the(1) cat(2) sat(3) .(4)
    let t = vec![n("the", Tag::DT, 2, Rel::Det), n("cat", Tag::NN, 3, Rel::Nsubj), n("sit", Tag::VBD, 0, Rel::Root), n(".", Tag::Stop, 3, Rel::Punct)];
    assert!(tree::projective(&t));
    let id = vec![1, 2, 3, 4];
    assert_eq!(tree::emit(&tree::locals(&t, &id)), id);
    let other = vec![3, 4, 1, 2];
    assert_eq!(tree::emit(&tree::locals(&t, &other)), other);
    // negative control: crossing arcs are not projective
    let x = vec![n("a", Tag::DT, 3, Rel::Det), n("b", Tag::NN, 4, Rel::Nsubj), n("c", Tag::NN, 0, Rel::Root), n("d", Tag::NN, 3, Rel::Obj)];
    assert!(!tree::projective(&x));
}

#[test]
fn tree_script_gives_the_tree_back_and_a_damaged_one_does_not() {
    let parsed = vec![n("she", Tag::PRP, 2, Rel::Nsubj), n("see", Tag::VBD, 0, Rel::Root), n("the", Tag::DT, 4, Rel::Det), n("cat", Tag::NN, 2, Rel::Obj), n("today", Tag::NN, 2, Rel::OblTmod), n(".", Tag::Stop, 2, Rel::Punct)];
    let realized = vec![n("see", Tag::VBD, 0, Rel::Root), n("Alice", Tag::NNP, 1, Rel::Nsubj), n("cat", Tag::NNS, 1, Rel::Obj), n("a", Tag::DT, 3, Rel::Det), n(".", Tag::Stop, 1, Rel::Punct)];
    let forms: Vec<String> = ["She", "saw", "the", "cat", "today", "."].iter().map(|s| s.to_string()).collect();
    let mut res = Residual::default();
    let script = frames::encode(&parsed, &realized, &forms, &mut res);
    assert_eq!(frames::apply(&realized, &script), parsed);
    let items = |c: Class| res.cells.get(&(crate::class::Layer::Frames, c)).map_or(0, |x| x.items);
    assert_eq!(items(Class::PronName), 2, "she ~ Alice: the lemma and the tag");
    assert_eq!(items(Class::Dropped), 1, "today");
    assert_eq!(items(Class::Function), 1, "the for a");
    assert_eq!(items(Class::Quantity), 1, "cat NNS → NN");
    // negative control: a script without its inserts does not give the tree back
    let damaged: Vec<Step> = script.iter().filter(|s| !matches!(s, Step::Insert(_))).cloned().collect();
    assert_ne!(frames::apply(&realized, &damaged), parsed);
}

#[test]
fn world_gives_the_frames_back_and_a_damaged_residual_does_not() {
    let mut said = f(Kind::Say(prag::schema::Act::Assert), "say", Some(e("Alice")), None);
    said.quote = vec![1];
    let mut big = e("dog");
    big.attrs = vec!["big".into()];
    let mut she = e("Alice");
    she.pron = Some("she".into());
    let sents = vec![
        vec![said, f(Kind::Fact(Link::Is), "be", Some(big.clone()), None)],
        vec![],
        vec![f(Kind::Event, "see", Some(she), Some(e("dog")))],
        vec![f(Kind::Fact(Link::Is), "be", Some(big), None), f(Kind::Bare, "", None, Some(e("cat")))],
        vec![f(Kind::Event, "run", Some(e("dog")), None), f(Kind::Event, "bark", Some(e("dog")), None)],
    ];
    let mut res = Residual::default();
    let (w, r4) = world::encode(&sents, &mut res);
    assert_eq!(world::decode_exact(&w, &r4, sents.len()), sents);
    let items = |c: Class| res.cells.get(&(crate::class::Layer::World, c)).map_or(0, |x| x.items);
    assert_eq!(items(Class::PronName), 1);
    assert_eq!(items(Class::Repetition), 1);
    assert!(items(Class::Sentence) >= 2, "an empty sentence and a split «run / bark»");
    // the phrase without a predicate («cat») is a mention in the world's line, not a frame the world drops
    assert!(w.line.iter().any(|ev| ev.kind == Kind::Bare && ev.patient.as_ref().is_some_and(|p| p.id == "cat")));
    assert_eq!(items(Class::Dropped), 0);
    // negative control: without the patches the frames do not come back
    let mut bad = r4.clone();
    bad.patches.clear();
    assert_ne!(world::decode_exact(&w, &bad, sents.len()), sents);
    let mut bad = r4.clone();
    bad.toggles.clear();
    assert_ne!(world::decode_exact(&w, &bad, sents.len()), sents);
}

fn model() -> Option<en::annotate::Annotator> {
    let p = crate::model_path();
    if !p.exists() {
        eprintln!("SKIP: no en model at {}", p.display());
        return None;
    }
    Some(en::annotate::Annotator::load(&p).expect("en model"))
}

const TEXT: &str = "CHAPTER I.\n\n“Alice  was beginning to get very tired,” said the\nWhite Rabbit; ‘hath she not seen it?’\n\nThe dog didn't bark. She never came back, and there were 3 cats!\n";

/// The whole chain on a small text with odd spacing, quotes, archaic forms and line wraps: byte for byte back; and the
/// negative controls — a damaged residual makes the gate red, a tree decoder that drops negation shows that loss.
#[test]
fn exact_chain_and_negative_controls() {
    let Some(a) = model() else { return };
    let v = crate::vol::txt_volume("t", TEXT);
    let o = crate::pipeline::Opts { exact: true, normal: true, ..Default::default() };
    let t = crate::pipeline::run(&a, &v, &o).unwrap();
    assert_eq!(t.exact_ok, 1, "{:?}", t.exact_fail);
    let again = crate::pipeline::run(&a, &v, &o).unwrap();
    assert_eq!(format!("{:?}", t.residual.cells.keys().collect::<Vec<_>>()), format!("{:?}", again.residual.cells.keys().collect::<Vec<_>>()));
    assert_eq!(t.residual.layer_bits(crate::class::Layer::Frames), again.residual.layer_bits(crate::class::Layer::Frames));
    let neg = |t: &crate::tally::Tally| t.normal.get(&Start::Tree).and_then(|d| d.cells.get(&Class::Neg)).map_or(0, |c| c.items);
    let dropped = crate::pipeline::run(&a, &v, &crate::pipeline::Opts { drop: Some(Class::Neg), ..o }).unwrap();
    assert!(neg(&dropped) >= neg(&t) + 3, "not, n't, never: {} vs {}", neg(&dropped), neg(&t));
    let punct = |t: &crate::tally::Tally| t.normal.get(&Start::Tree).and_then(|d| d.cells.get(&Class::Punct)).map_or(0, |c| c.items);
    let nop = crate::pipeline::run(&a, &v, &crate::pipeline::Opts { drop: Some(Class::Punct), ..o }).unwrap();
    assert!(punct(&nop) >= punct(&t) + 8, "{} vs {}", punct(&nop), punct(&t));
    // a damaged tree residual: the words come back in another order or form
    let dec = tree::Dec { a: &a };
    let seg = crate::vol::segment(&a, "The quick brown fox jumped over the lazy dog.", 0, false, None).unwrap();
    let s = &seg.sents[0];
    let w = a.annotate(&s.forms);
    let tn = tree::of_words(&w);
    let mut res = Residual::default();
    let r2 = dec.encode(&tn, &s.forms, &s.space(), &mut res);
    assert_eq!(dec.decode_exact(&tn, &r2).0, s.forms);
    let mut bad = r2.clone();
    bad.spell = BTreeMap::new();
    bad.spell.insert(1, tree::Spell::Raw("A".into()));
    assert_ne!(dec.decode_exact(&tn, &bad).0, s.forms);
}

/// The realizer does not repeat a shared noun subject of coordinated frames; with the step off it does (the control).
#[test]
fn realizer_does_not_repeat_a_shared_subject() {
    let fs = vec![f(Kind::Event, "bark", Some(e("dog")), None), f(Kind::Event, "run", Some(e("dog")), None)];
    let subjects = |r: &mut frames::Realizer| r.sentence(&fs).iter().filter(|x| x.rel == Rel::Nsubj).count();
    assert_eq!(subjects(&mut frames::Realizer::with_off(&[])), 1);
    let mut off = frames::Realizer::default();
    off.off.insert("rt-shared-subject".into());
    assert_eq!(subjects(&mut off), 2, "negative control: the step off says the subject twice");
    // a pronoun subject is said again
    let mut she = e("Alice");
    she.pron = Some("she".into());
    let fs = vec![f(Kind::Event, "sing", Some(she.clone()), None), f(Kind::Event, "dance", Some(she), None)];
    assert_eq!(frames::Realizer::with_off(&[]).sentence(&fs).iter().filter(|x| x.rel == Rel::Nsubj).count(), 2);
}

/// The realizer says what the frames now keep: the possessor, the noun's modifiers, the clause's adverbs, and not the
/// subject a frame shares; with that content absent (or the step off) it does not.
#[test]
fn realizer_says_the_content_the_frames_keep() {
    let mut dog = e("dog");
    let mut his = e("Bill");
    his.pron = Some("his".into());
    dog.poss = Some(Box::new(his));
    dog.mods = vec![("of".into(), e("queen"))];
    let mut ran = f(Kind::Event, "run", Some(dog.clone()), None);
    ran.words = vec![(Rel::Advmod, "then".into(), Tag::RB)];
    let mut barked = f(Kind::Event, "bark", Some(dog), None);
    barked.unsaid = vec![prag::frames::Slot::Agent];
    let t = frames::Realizer::with_off(&[]).sentence(&[ran.clone(), barked.clone()]);
    let has = |t: &[TNode], l: &str, r: Rel| t.iter().any(|x| x.lemma == l && x.rel == r);
    assert!(has(&t, "his", Rel::NmodPoss) && has(&t, "queen", Rel::Nmod) && has(&t, "of", Rel::Case) && has(&t, "then", Rel::Advmod));
    let d = t.iter().position(|x| x.lemma == "dog").unwrap() + 1;
    assert!(!t.iter().any(|x| x.rel == Rel::Det && x.head == d), "a possessor, not an article");
    assert_eq!(t.iter().filter(|x| x.rel == Rel::Nsubj).count(), 1, "the shared subject once");
    // controls
    let t = frames::Realizer::with_off(&["rt-unsaid", "rt-shared-subject"]).sentence(&[ran.clone(), barked]);
    assert_eq!(t.iter().filter(|x| x.rel == Rel::Nsubj).count(), 2);
    let plain = f(Kind::Event, "run", Some(e("dog")), None);
    let t = frames::Realizer::with_off(&[]).sentence(&[plain]);
    assert!(!has(&t, "queen", Rel::Nmod) && !has(&t, "his", Rel::NmodPoss) && !has(&t, "then", Rel::Advmod));
}

/// The realizer says the kept function words; with PRAG_OFF=rt-function it says its old defaults (the control).
#[test]
fn realizer_says_the_function_words_the_frames_keep() {
    let mut dog = e("dog");
    dog.det = Some(("the".into(), Tag::DT));
    let mut box_ = e("box");
    box_.case = Some("on".into());
    let mut sat = f(Kind::Event, "sit", Some(dog), None);
    sat.place = Some(box_);
    sat.cc = Some("but".into());
    let lemmas = |t: &[TNode]| t.iter().map(|x| x.lemma.clone()).collect::<Vec<_>>();
    let t = lemmas(&frames::Realizer::with_off(&[]).sentence(std::slice::from_ref(&sat)));
    assert!(t.contains(&"the".into()) && t.contains(&"on".into()) && t.contains(&"but".into()) && !t.contains(&"a".into()) && !t.contains(&"in".into()), "{t:?}");
    let t = lemmas(&frames::Realizer::with_off(&["rt-function"]).sentence(&[sat]));
    assert!(t.contains(&"a".into()) && t.contains(&"in".into()) && !t.contains(&"on".into()) && !t.contains(&"but".into()), "{t:?}");
}

/// A linked clause hangs under the noun its link names, with its relation; with PRAG_OFF=rt-links it is a conjunct.
#[test]
fn realizer_hangs_clauses_where_the_links_say() {
    let sat = f(Kind::Event, "sit", Some(e("man")), None);
    let mut who = e("man");
    who.rel_word = Some(("who".into(), Tag::WP));
    let mut came = f(Kind::Event, "come", Some(who), None);
    came.link = Some((0, Rel::AclRelcl, Some("man".into())));
    let t = frames::Realizer::with_off(&[]).sentence(&[sat.clone(), came.clone()]);
    let man = t.iter().position(|x| x.lemma == "man").unwrap() + 1;
    assert!(t.iter().any(|x| x.lemma == "come" && x.rel == Rel::AclRelcl && x.head == man), "{t:?}");
    assert!(t.iter().any(|x| x.lemma == "who" && x.rel == Rel::Nsubj));
    let t = frames::Realizer::with_off(&["rt-links"]).sentence(&[sat, came]);
    assert!(t.iter().any(|x| x.lemma == "come" && x.rel == Rel::Conj) && !t.iter().any(|x| x.lemma == "who"), "{t:?}");
}

/// The realizer punctuates by clause structure: a coordinated clause with its own subject takes a comma; with
/// PRAG_OFF=rt-punct only the full stop is said (the control).
#[test]
fn realizer_punctuates_by_clause_structure() {
    let barked = f(Kind::Event, "bark", Some(e("dog")), None);
    let mut sat = f(Kind::Event, "sit", Some(e("cat")), None);
    sat.cc = Some("and".into());
    sat.link = Some((0, Rel::Conj, None));
    let commas = |t: &[TNode]| t.iter().filter(|x| x.lemma == ",").count();
    let t = frames::Realizer::with_off(&[]).sentence(&[barked.clone(), sat.clone()]);
    assert_eq!(commas(&t), 1, "{t:?}");
    assert!(t.iter().any(|x| x.lemma == "." && x.head == 1));
    let t = frames::Realizer::with_off(&["rt-punct"]).sentence(&[barked, sat]);
    assert_eq!(commas(&t), 0);
}

fn w(form: &str, lemma: &str, upos: en::gram::UPos, tag: Tag, head: usize, rel: Rel) -> en::annotate::Word {
    en::annotate::Word { form: form.into(), lemma: lemma.into(), upos, tag, feats: Default::default(), head, rel }
}

/// Tense, aspect, voice and modality: read from the tree into the frame (`prag::frames::tam_of`), said again by the
/// realizer, and the tree residual of the class is gone; the negative control — the same frame without its `Tam` —
/// gives the old present-or-past clause and the class's loss.
#[test]
fn tam_is_read_and_realized() {
    use en::gram::UPos;
    // the(1) cake(2) has(3) been(4) eaten(5) .(6)
    let words = vec![
        w("The", "the", UPos::DET, Tag::DT, 2, Rel::Det),
        w("cake", "cake", UPos::NOUN, Tag::NN, 5, Rel::NsubjPass),
        w("has", "have", UPos::AUX, Tag::VBZ, 5, Rel::Aux),
        w("been", "be", UPos::AUX, Tag::VBN, 5, Rel::AuxPass),
        w("eaten", "eat", UPos::VERB, Tag::VBN, 0, Rel::Root),
        w(".", ".", UPos::PUNCT, Tag::Stop, 5, Rel::Punct),
    ];
    let t = prag::frames::Tree::of_words(words.clone());
    let m = prag::frames::tam_of(&t, 4);
    assert!(m.read && m.perfect && m.passive && !m.progressive);
    assert_eq!(m.tense, prag::frames::Tense::Pres);
    assert_eq!(m.show(), "pres.perf.pass");
    // will not go; to go; Come!
    let fut = prag::frames::Tree::of_words(vec![w("She", "she", UPos::PRON, Tag::PRP, 3, Rel::Nsubj), w("will", "will", UPos::AUX, Tag::MD, 3, Rel::Aux), w("go", "go", UPos::VERB, Tag::VB, 0, Rel::Root)]);
    assert_eq!(prag::frames::tam_of(&fut, 2).tense, prag::frames::Tense::Fut);
    let imp = prag::frames::Tree::of_words(vec![w("Come", "come", UPos::VERB, Tag::VB, 0, Rel::Root), w("!", "!", UPos::PUNCT, Tag::Stop, 1, Rel::Punct)]);
    assert!(prag::frames::tam_of(&imp, 0).imperative);
    let can = prag::frames::Tree::of_words(vec![w("Fish", "fish", UPos::NOUN, Tag::NNS, 3, Rel::Nsubj), w("can", "can", UPos::AUX, Tag::MD, 3, Rel::Aux), w("swim", "swim", UPos::VERB, Tag::VB, 0, Rel::Root)]);
    assert_eq!(prag::frames::tam_of(&can, 2).modal, Some(prag::frames::Modal::Can));
    // the realizer says the chain, and the parsed tree needs no tense patch
    let parsed = tree::of_words(&words);
    let forms: Vec<String> = words.iter().map(|x| x.form.clone()).collect();
    let mut fr = f(Kind::Event, "eat", None, Some(e("cake")));
    fr.tam = m;
    let tense = |fr: &F| {
        let r = frames::Realizer::default().sentence(std::slice::from_ref(fr));
        let mut res = Residual::default();
        frames::encode(&parsed, &r, &forms, &mut res);
        (r, res.cells.get(&(crate::class::Layer::Frames, Class::Tense)).map_or(0, |x| x.items))
    };
    let (r, items) = tense(&fr);
    assert!(r.iter().any(|x| x.lemma == "have" && x.tag == Tag::VBZ && x.rel == Rel::Aux));
    assert!(r.iter().any(|x| x.lemma == "be" && x.tag == Tag::VBN && x.rel == Rel::AuxPass));
    assert!(r.iter().any(|x| x.lemma == "eat" && x.tag == Tag::VBN));
    assert_eq!(items, 0, "{r:?}");
    // negative control: no Tam, the old clause — «a cake ate» without its auxiliaries
    fr.tam = Default::default();
    let (r, items) = tense(&fr);
    assert!(!r.iter().any(|x| x.lemma == "have"));
    assert!(items >= 2, "{items}");
}

/// The referring-expression generator: an entity met in the sentence before, which the reader's resolver would take
/// the pronoun back to, is said as its pronoun in the form of its role; a first mention, a mention two sentences
/// on and a rival of the same gender stay names. The negative control: the plain decoder says no pronoun at all.
#[test]
fn pronouns_by_salience() {
    let mut she = e("Alice");
    she.named = true;
    she.pron = Some("she".into());
    let mut her = e("Alice");
    her.named = true;
    her.pron = Some("her".into());
    let mut alice = e("Alice");
    alice.named = true;
    let sents = vec![
        vec![f(Kind::Event, "see", Some(alice.clone()), Some(e("rabbit")))],
        vec![f(Kind::Event, "run", Some(she), None)],
        vec![f(Kind::Event, "call", Some(e("rabbit")), Some(her))],
        vec![f(Kind::Event, "sleep", Some(e("cat")), None)],
        vec![f(Kind::Event, "sleep", Some(e("dog")), None)],
        vec![f(Kind::Event, "wake", Some(alice), None)],
    ];
    let (w, _) = world::encode(&sents, &mut Residual::default());
    assert_eq!(w.ents["Alice"].pro, Some("she"));
    let plain = world::decode_plain(&w);
    let mut said = plain.clone();
    world::refer(&w, &mut said);
    let pron = |fs: &[F], k: usize, agent: bool| {
        let x = if agent { fs[k].agent.as_ref() } else { fs[k].patient.as_ref() };
        x.and_then(|e| e.pron.clone())
    };
    assert_eq!(pron(&said, 0, true), None, "the first mention is the name");
    assert_eq!(pron(&said, 1, true).as_deref(), Some("She"), "the subject of a new sentence");
    assert_eq!(pron(&said, 2, false).as_deref(), Some("her"), "an object");
    assert_eq!(pron(&said, 5, true), None, "two sentences on: the name again");
    // negative control
    assert!((0..plain.len()).all(|k| pron(&plain, k, true).is_none() && pron(&plain, k, false).is_none()));
}

/// A frame with its own `tam` says the same content as one without: the clause's roles, function words and links
/// do not depend on which verb group was built. The control: the same frame with the steps off loses them.
#[test]
fn the_tam_verb_group_keeps_the_frames_content() {
    let tam = prag::frames::Tam { read: true, tense: prag::frames::Tense::Past, ..Default::default() };
    let mut cake = e("cake");
    cake.det = Some(("the".into(), Tag::DT));
    let mut ate = f(Kind::Event, "eat", Some(e("girl")), Some(cake));
    ate.tam = tam;
    ate.words = vec![(Rel::Advmod, "then".into(), Tag::RB)];
    let mut left = f(Kind::Event, "leave", Some(e("girl")), None);
    left.tam = tam;
    left.unsaid = vec![prag::frames::Slot::Agent];
    left.link = Some((0, Rel::Conj, None));
    left.cc = Some("and".into());
    let t = frames::Realizer::with_off(&[]).sentence(&[ate.clone(), left.clone()]);
    let has = |t: &[TNode], l: &str, r: Rel| t.iter().any(|x| x.lemma == l && x.rel == r);
    assert!(has(&t, "the", Rel::Det) && has(&t, "then", Rel::Advmod) && has(&t, "leave", Rel::Conj) && has(&t, "and", Rel::Cc), "{t:?}");
    assert_eq!(t.iter().filter(|x| x.rel == Rel::Nsubj).count(), 1);
    let t = frames::Realizer::with_off(&["rt-function", "rt-unsaid", "rt-shared-subject"]).sentence(&[ate, left]);
    assert_eq!(t.iter().filter(|x| x.rel == Rel::Nsubj).count(), 2);
}

/// The block gate: every unit comes back byte for byte; the reader's discourse reset at every sentence leaves the
/// pronouns unresolved; one lost residual item of any layer makes the block red; two runs are the same.
#[test]
fn block_units_and_damage() {
    use crate::class::LAYERS;
    use crate::pipeline::{Opts, Unit, run};
    let Some(a) = model() else { return };
    let text = "GEN 1\nThe old man saw a small dog in the garden. He fed it bread. The dog thanked him.\n\nThen the man went home, and he slept.\n\n";
    let v = crate::vol::txt_volume("b", text);
    let o = Opts { exact: true, normal: true, ..Default::default() };
    let b = run(&a, &v, &o).unwrap();
    assert_eq!(b.exact_ok, 1, "{:?}", b.exact_fail);
    assert_eq!(b.whole.get(&Start::Tokens).map(|d| d.exact), Some(1));
    let s = run(&a, &v, &Opts { unit: Unit::Sentence, ..o }).unwrap();
    let w = run(&a, &v, &Opts { unit: Unit::World, ..o }).unwrap();
    assert_eq!((s.exact_ok, w.exact_ok), (1, 1));
    assert!(s.unresolved > b.unresolved, "{} vs {}", s.unresolved, b.unresolved);
    assert_eq!(w.unresolved, b.unresolved);
    for l in LAYERS {
        let d = run(&a, &v, &Opts { normal: false, damage: Some(l), ..o }).unwrap();
        assert_eq!(d.exact_ok + d.damaged, 1, "{}: a damaged block must be red", l.name());
    }
    let again = run(&a, &v, &o).unwrap();
    assert_eq!(format!("{:?}", b.residual.cells), format!("{:?}", again.residual.cells));
}

#[test]
fn site_pages_rebuild_their_trees_and_tables() {
    use crate::blocks::site_code;
    use crate::class::Layer;
    use crate::pipeline::{Opts, run};
    let Some(a) = model() else { return };
    // a page as the site writes it: the sentence, its tree as `en` gives it, its frames table from that tree
    let s = "The old man saw a small dog in the garden.";
    let rows: String = site_code::rows_for(&a, s).concat();
    let ws = site_code::words_for(&a, s);
    let (fs, _, _) = prag::frames::frames_of_tree(&prag::frames::Tree::of_words(ws), &mut prag::frames::Ctx::default(), false);
    let table = prag::frames_md::table(&fs);
    let page = format!("---\ntitle: A sentence\n---\n\n<!-- .ven lang=en -->\n{s}\n\n## The tree {{#tree}}\n\n```conllu\n# sent_id = t:1\n# text = {s}\n{rows}```\n\n## Pragmatics {{#pragmatics}}\n\n{table}<!-- pn -->\n- next →: [x](https://md.mova.elamur.ai/free/en/x/)\n");
    let v = crate::blocks::site_md::page_volume("p", &page);
    assert_eq!(v.text(), page);
    let o = Opts { exact: true, ..Default::default() };
    let t = run(&a, &v, &o).unwrap();
    assert_eq!(t.exact_ok, 1, "{:?}", t.exact_fail);
    // nothing stored for the tree and the table but the comment line, the reference and the two bits
    assert_eq!(t.derived["conllu"].free, 1);
    assert_eq!(t.derived["frames table"].free, 1);
    let tree_bits: f64 = t.residual.cells.iter().filter(|(k, _)| k.1 == Class::Conllu).map(|(_, c)| c.bits).sum();
    assert!(tree_bits < 0.05 * t.derived["conllu"].literal_bits, "{tree_bits}");
    // a page whose tree differs from `en`'s: the patch brings it back, and losing it is red
    let bad = page.replacen("\tnsubj\t", "\tobj\t", 1);
    let vb = crate::blocks::site_md::page_volume("p", &bad);
    let tb = run(&a, &vb, &o).unwrap();
    assert_eq!(tb.exact_ok, 1, "{:?}", tb.exact_fail);
    assert!(tb.residual.cells.get(&(Layer::Tree, Class::Conllu)).is_some_and(|c| c.kinds.contains_key("deprel")));
    let d = run(&a, &vb, &Opts { damage: Some(Layer::Tree), ..o }).unwrap();
    assert_eq!(d.exact_ok + d.damaged, 1, "a damaged page must be red");
    // the table of another tree is not this one's: rebuilt with a script, and red without it
    let other = bad.replace(&table, &table.replacen("**see**", "**look**", 1));
    let vo = crate::blocks::site_md::page_volume("p", &other);
    let to = run(&a, &vo, &o).unwrap();
    assert_eq!(to.exact_ok, 1, "{:?}", to.exact_fail);
    assert_eq!(to.derived["frames table"].free, 0);
    let dt = run(&a, &vo, &Opts { damage: Some(Layer::Frames), ..o }).unwrap();
    assert_eq!(dt.exact_ok + dt.damaged, 1);
}

#[test]
fn the_copier_gives_every_literal_back() {
    let mut c = crate::vol::Copier::default();
    let xs = ["[home](https://md.mova.elamur.ai/free/en/) / ", "[corpora](https://md.mova.elamur.ai/free/en/corpora/) / ", "](https://md.mova.elamur.ai/free/en/corpora/torah/jps/DEU.355/)", "](https://md.mova.elamur.ai/free/en/corpora/torah/jps/DEU.356/)", "שָׁ֑ם", ""];
    let mut codes = Vec::new();
    for x in xs {
        let (b, code) = c.code(x);
        assert!(b <= crate::class::lit_bits(x) + 1.0 + 1e-9);
        codes.push(code);
    }
    // the fourth is the third with one byte stored
    assert!(matches!(&codes[3], crate::vol::Copy::From { mid, .. } if mid == "6"), "{:?}", codes[3]);
    // negative control: a copy of another literal is not this one
    if let crate::vol::Copy::From { pre, suf, mid, .. } = &codes[3] {
        let wrong = crate::vol::Copy::From { from: 0, pre: *pre, suf: *suf, mid: mid.clone() };
        assert_ne!(std::panic::catch_unwind(|| c.decode(&wrong)).unwrap_or_default(), xs[3]);
    }
}

/// A sentence whose root is no predicate (a heading, a noun phrase, an interjection) is one bare frame that carries
/// the root and what hangs on it (`prag::frames`, the fragment), and a relative clause under it links to it: the
/// realizer says them all and no content word is dropped. Negative control: without the bare frame the words are
/// dropped.
#[test]
fn verbless_roots_are_fragments() {
    use en::gram::UPos;
    let dropped = |words: &[en::annotate::Word], fs: &[F]| {
        let parsed = tree::of_words(words);
        let forms: Vec<String> = words.iter().map(|x| x.form.clone()).collect();
        let r = frames::Realizer::with_off(&[]).sentence(fs);
        let mut res = Residual::default();
        frames::encode(&parsed, &r, &forms, &mut res);
        res.cells.get(&(crate::class::Layer::Frames, Class::Dropped)).map_or(0, |x| x.items)
    };
    let heading = vec![
        w("The", "the", UPos::DET, Tag::DT, 3, Rel::Det),
        w("coreference", "coreference", UPos::NOUN, Tag::NN, 3, Rel::Compound),
        w("sieve", "sieve", UPos::NOUN, Tag::NN, 0, Rel::Root),
        w("of", "of", UPos::ADP, Tag::IN, 6, Rel::Case),
        w("the", "the", UPos::DET, Tag::DT, 6, Rel::Det),
        w("dictionary", "dictionary", UPos::NOUN, Tag::NN, 3, Rel::Nmod),
        w(".", ".", UPos::PUNCT, Tag::Stop, 3, Rel::Punct),
    ];
    let relcl = vec![
        w("The", "the", UPos::DET, Tag::DT, 2, Rel::Det),
        w("dog", "dog", UPos::NOUN, Tag::NN, 0, Rel::Root),
        w("that", "that", UPos::PRON, Tag::WDT, 4, Rel::Nsubj),
        w("barked", "bark", UPos::VERB, Tag::VBD, 2, Rel::AclRelcl),
        w(".", ".", UPos::PUNCT, Tag::Stop, 2, Rel::Punct),
    ];
    let yes = vec![w("Yes", "yes", UPos::INTJ, Tag::UH, 0, Rel::Root), w(".", ".", UPos::PUNCT, Tag::Stop, 1, Rel::Punct)];
    for words in [&heading, &relcl, &yes] {
        let fs = frames::read(words, &mut prag::frames::Ctx::default());
        let bare: Vec<usize> = (0..fs.len()).filter(|&i| fs[i].kind == Kind::Bare).collect();
        assert_eq!(bare.len(), 1, "{fs:?}");
        assert!(fs.iter().all(|f| f.kind == Kind::Bare || f.link.as_ref().is_some_and(|l| l.0 == bare[0])), "{fs:?}");
        assert_eq!(dropped(words, &fs), 0, "{fs:?}");
        // negative control: the bare frame left out
        let rest: Vec<F> = fs.iter().filter(|f| f.kind != Kind::Bare).cloned().map(|mut f| {
            f.link = None;
            f
        }).collect();
        assert!(dropped(words, &rest) >= 1);
    }
}

/// What else a clause and a phrase say is kept (`prag::frames`, frame-rest, ent-rest, wh-free): a second object, a
/// free wh-word, the modifiers of a modifier («small and transparent»); the realizer says them and no content word
/// is missing. Negative control: the same frames without the kept noun phrases and words miss them.
#[test]
fn the_rest_of_a_clause_and_a_phrase_is_kept() {
    use en::gram::UPos;
    let dropped = |words: &[en::annotate::Word], fs: &[F]| {
        let parsed = tree::of_words(words);
        let forms: Vec<String> = words.iter().map(|x| x.form.clone()).collect();
        let r = frames::Realizer::with_off(&[]).sentence(fs);
        let mut res = Residual::default();
        frames::encode(&parsed, &r, &forms, &mut res);
        res.cells.iter().filter(|(k, _)| k.0 == crate::class::Layer::Frames && k.1 != crate::class::Class::Punct).flat_map(|(_, c)| c.kinds.iter()).filter(|(k, _)| k.starts_with("missing ")).map(|(_, n)| *n).sum::<u64>()
    };
    let model = vec![
        w("Mova", "Mova", UPos::PROPN, Tag::NNP, 8, Rel::Nsubj),
        w("is", "be", UPos::AUX, Tag::VBZ, 8, Rel::Cop),
        w("a", "a", UPos::DET, Tag::DT, 8, Rel::Det),
        w("small", "small", UPos::ADJ, Tag::JJ, 8, Rel::Amod),
        w("and", "and", UPos::CCONJ, Tag::CC, 6, Rel::Cc),
        w("transparent", "transparent", UPos::ADJ, Tag::JJ, 4, Rel::Conj),
        w("language", "language", UPos::NOUN, Tag::NN, 8, Rel::Compound),
        w("model", "model", UPos::NOUN, Tag::NN, 0, Rel::Root),
        w(".", ".", UPos::PUNCT, Tag::Stop, 8, Rel::Punct),
    ];
    let objects = vec![
        w("They", "they", UPos::PRON, Tag::PRP, 2, Rel::Nsubj),
        w("gave", "give", UPos::VERB, Tag::VBD, 0, Rel::Root),
        w("the", "the", UPos::DET, Tag::DT, 4, Rel::Det),
        w("room", "room", UPos::NOUN, Tag::NN, 2, Rel::Obj),
        w("a", "a", UPos::DET, Tag::DT, 6, Rel::Det),
        w("makeover", "makeover", UPos::NOUN, Tag::NN, 2, Rel::Obj),
        w(".", ".", UPos::PUNCT, Tag::Stop, 2, Rel::Punct),
    ];
    let who = vec![
        w("Who", "who", UPos::PRON, Tag::WP, 2, Rel::Nsubj),
        w("acts", "act", UPos::VERB, Tag::VBZ, 0, Rel::Root),
        w(":", ":", UPos::PUNCT, Tag::Colon, 4, Rel::Punct),
        w("something", "something", UPos::PRON, Tag::NN, 2, Rel::Obj),
        w(".", ".", UPos::PUNCT, Tag::Stop, 2, Rel::Punct),
    ];
    for words in [&model, &objects, &who] {
        let fs = frames::read(words, &mut prag::frames::Ctx::default());
        assert_eq!(dropped(words, &fs), 0, "{fs:?}");
        // negative control: without the clause's other noun phrases and words
        let bare: Vec<F> = fs.iter().cloned().map(|mut f| {
            f.nps.clear();
            f.words.clear();
            f
        }).collect();
        assert!(dropped(words, &bare) >= 1, "{fs:?}");
    }
}
