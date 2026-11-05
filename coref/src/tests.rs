//! Sieve tests on hand-written sentences (our own text, UD trees by hand) and controls: the output does not depend on gold
//! `Entity=`; written CorefUD reads back with the same mentions.

use en::conllu::Doc as UdDoc;

use crate::corefud;
use crate::doc;
use crate::eval;
use crate::sieve::{Config, Resolver, Sieve};

/// A CoNLL-U line from short columns: form, UPOS, XPOS, FEATS, head, relation, MISC.
fn row(i: usize, c: &str) -> String {
    let v: Vec<&str> = c.split(' ').collect();
    let misc = v.get(6).copied().unwrap_or("_");
    format!("{i}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t_\t{misc}", v[0], v[0].to_lowercase(), v[1], v[2], v[3], v[4], v[5])
}

fn sent(id: &str, extra: &[&str], words: &[&str]) -> String {
    let text: Vec<&str> = words.iter().map(|w| w.split(' ').next().unwrap()).collect();
    let mut s = format!("# sent_id = {id}\n");
    for e in extra {
        s.push_str(e);
        s.push('\n');
    }
    s.push_str(&format!("# text = {}\n", text.join(" ")));
    for (i, w) in words.iter().enumerate() {
        s.push_str(&row(i + 1, w));
        s.push('\n');
    }
    s.push('\n');
    s
}

const PRS3M: &str = "Case=Nom|Gender=Masc|Number=Sing|Person=3|PronType=Prs";
const VBD: &str = "Mood=Ind|Number=Sing|Person=3|Tense=Past|VerbForm=Fin";

/// Mr. Smith, a doctor, saw Mary. He saw himself. Mary is a nurse who likes him.
/// We do not guess gender from names: "He" finds Smith through the title Mr. (the text says it itself).
fn story() -> String {
    let mut t = String::from("# newdoc id = t1\n");
    t += &sent(
        "t1-1",
        &[],
        &[
            "Mr. PROPN NNP Number=Sing 2 compound Entity=(1-person",
            "Smith PROPN NNP Number=Sing 7 nsubj Entity=1)",
            ", PUNCT , _ 5 punct",
            "a DET DT Definite=Ind|PronType=Art 5 det Entity=(1-person",
            "doctor NOUN NN Number=Sing 2 appos Entity=1)",
            ", PUNCT , _ 5 punct",
            &format!("saw VERB VBD {VBD} 0 root"),
            "Mary PROPN NNP Number=Sing 7 obj Entity=(2-person)",
            ". PUNCT . _ 7 punct",
        ],
    );
    t += &sent(
        "t1-2",
        &[],
        &[&format!("He PRON PRP {PRS3M} 2 nsubj Entity=(1-person)"), &format!("saw VERB VBD {VBD} 0 root"), "himself PRON PRP Case=Acc|Gender=Masc|Number=Sing|Person=3|PronType=Prs|Reflex=Yes 2 obj Entity=(1-person)", ". PUNCT . _ 2 punct"],
    );
    t += &sent(
        "t1-3",
        &[],
        &[
            "Mary PROPN NNP Number=Sing 4 nsubj Entity=(2-person)",
            "is AUX VBZ Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin 4 cop",
            "a DET DT Definite=Ind|PronType=Art 4 det Entity=(2-person",
            "nurse NOUN NN Number=Sing 0 root",
            "who PRON WP PronType=Rel 6 nsubj",
            "likes VERB VBZ Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin 4 acl:relcl",
            "him PRON PRP Case=Acc|Gender=Masc|Number=Sing|Person=3|PronType=Prs 6 obj Entity=(1-person)2)",
            ". PUNCT . _ 4 punct",
        ],
    );
    t
}

fn resolve(text: &str) -> (Vec<doc::Document>, Vec<eval::Sys>) {
    let ud = UdDoc::parse(text).unwrap();
    let docs = doc::documents(&ud).unwrap();
    let sys = docs.iter().map(|d| eval::resolve(d, &Config::default())).collect();
    (docs, sys)
}

/// Texts of the mentions of the cluster that contains a mention with this text.
fn chain_of(d: &doc::Document, s: &eval::Sys, text: &str) -> Vec<String> {
    let t = |m: usize| {
        let x = &s.ms[m];
        d.sents[x.span.sent].text(x.span.start, x.span.end)
    };
    let c = s.clusters.iter().find(|c| c.iter().any(|&m| t(m) == text)).unwrap_or_else(|| panic!("no mention '{text}'"));
    c.iter().map(|&m| t(m)).collect()
}

fn sieve_of(s: &eval::Sys, d: &doc::Document, text: &str) -> Option<Sieve> {
    s.links.iter().find(|l| {
        let x = &s.ms[l.from];
        d.sents[x.span.sent].text(x.span.start, x.span.end) == text
    })
    .map(|l| l.sieve)
}

#[test]
fn story_chains_and_reasons() {
    let (docs, sys) = resolve(&story());
    let (d, s) = (&docs[0], &sys[0]);
    let john = chain_of(d, s, "Mr. Smith");
    assert_eq!(john, ["Mr. Smith", "a doctor", "He", "himself", "him"], "{john:?}");
    let mary = chain_of(d, s, "a nurse who likes him");
    assert!(mary.contains(&"Mary".to_string()) && mary.contains(&"who".to_string()), "{mary:?}");
    assert_eq!(sieve_of(s, d, "a doctor"), Some(Sieve::Appos));
    assert_eq!(sieve_of(s, d, "himself"), Some(Sieve::Reflexive));
    assert_eq!(sieve_of(s, d, "a nurse who likes him"), Some(Sieve::Pred));
    assert_eq!(sieve_of(s, d, "who"), Some(Sieve::Relpron));
    // "him": the co-argument "who" (cluster Mary) is rejected by principle B — linked to Mr. Smith
    assert_eq!(sieve_of(s, d, "him"), Some(Sieve::Pronoun));
    let why = &s.links.iter().find(|l| l.sieve == Sieve::Pronoun && d.sents[s.ms[l.from].span.sent].id == "t1-3").unwrap().why;
    assert!(why.contains("Hobbs"), "{why}");
    // both "Mary" — exact match
    assert_eq!(chain_of(d, s, "Mary").iter().filter(|x| *x == "Mary").count(), 2);
}

/// Negative control of principle B: without it "him" goes to the Mary cluster via "who".
#[test]
fn principle_b_is_what_blocks_who() {
    let (docs, _) = resolve(&story());
    let d = &docs[0];
    let mut r = Resolver::new(d, Config::default());
    r.run();
    let him = r.ms.iter().position(|m| d.sents[m.span.sent].id == "t1-3" && m.head_low == "him").unwrap();
    let who = r.ms.iter().position(|m| d.sents[m.span.sent].id == "t1-3" && m.head_low == "who").unwrap();
    assert_ne!(r.cluster_id(him), r.cluster_id(who));
}

/// Speakers: "I" of the same speaker is one person; "you" to the addressee is their "I".
#[test]
fn speakers() {
    let mut t = String::from("# newdoc id = t2\n");
    let prs1 = "Case=Nom|Number=Sing|Person=1|PronType=Prs";
    let prs2 = "Case=Nom|Person=2|PronType=Prs";
    t += &sent("t2-1", &["# speaker = Ann", "# addressee = Bob"], &[&format!("I PRON PRP {prs1} 2 nsubj"), "am AUX VBP Mood=Ind|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin 0 root", ". PUNCT . _ 2 punct"]);
    t += &sent("t2-2", &["# speaker = Bob", "# addressee = Ann"], &[&format!("You PRON PRP {prs2} 2 nsubj"), "are AUX VBP Mood=Ind|Tense=Pres|VerbForm=Fin 0 root", ". PUNCT . _ 2 punct"]);
    t += &sent("t2-3", &["# speaker = Bob", "# addressee = Ann"], &[&format!("I PRON PRP {prs1} 2 nsubj"), "know VERB VBP Mood=Ind|Tense=Pres|VerbForm=Fin 0 root", ". PUNCT . _ 2 punct"]);
    t += &sent("t2-4", &["# speaker = Ann", "# addressee = Bob"], &[&format!("I PRON PRP {prs1} 2 nsubj"), "know VERB VBP Mood=Ind|Tense=Pres|VerbForm=Fin 0 root", ". PUNCT . _ 2 punct"]);
    let (docs, sys) = resolve(&t);
    let (d, s) = (&docs[0], &sys[0]);
    let ids = |c: &Vec<usize>| c.iter().map(|&m| d.sents[s.ms[m].span.sent].id.clone()).collect::<Vec<_>>();
    let ann = s.clusters.iter().find(|c| c.iter().any(|&m| d.sents[s.ms[m].span.sent].id == "t2-1")).unwrap();
    assert_eq!(ids(ann), ["t2-1", "t2-2", "t2-4"]);
    let bob = s.clusters.iter().find(|c| c.iter().any(|&m| d.sents[s.ms[m].span.sent].id == "t2-3")).unwrap();
    assert_eq!(ids(bob), ["t2-3"]);
}

/// Leak control: the output is the same whether MISC has gold Entity or not.
#[test]
fn resolver_ignores_gold_entities() {
    let text = story();
    let mut ud = UdDoc::parse(&text).unwrap();
    let a: Vec<Vec<corefud::OutMention>> = doc::documents(&ud).unwrap().iter().map(|d| eval::out_mentions(&eval::resolve(d, &Config::default()))).collect();
    corefud::strip(&mut ud);
    assert!(!ud.text().contains("Entity="));
    let b: Vec<Vec<corefud::OutMention>> = doc::documents(&ud).unwrap().iter().map(|d| eval::out_mentions(&eval::resolve(d, &Config::default()))).collect();
    let key = |v: &Vec<Vec<corefud::OutMention>>| v.iter().flatten().map(|m| (m.span, m.eid, m.note.clone())).collect::<Vec<_>>();
    assert_eq!(key(&a), key(&b));
}

/// Writing CorefUD and reading it back: the same mentions and entities; the scorer "output against itself" gives 100.
#[test]
fn write_read_round_trip() {
    let text = story();
    let mut ud = UdDoc::parse(&text).unwrap();
    let docs = doc::documents(&ud).unwrap();
    let outs: Vec<Vec<corefud::OutMention>> = docs.iter().map(|d| eval::out_mentions(&eval::resolve(d, &Config::default()))).collect();
    corefud::write(&mut ud, &docs, &outs).unwrap();
    let back = UdDoc::parse(&ud.text()).unwrap();
    assert!(back.text().contains("# global.Entity = eid-etype-head-other"));
    let bdocs = doc::documents(&back).unwrap();
    let fm = corefud::read_mentions(&bdocs[0]).unwrap();
    let mut a: Vec<(corefud::Span, String)> = outs[0].iter().map(|m| (m.span, format!("e{}", m.eid))).collect();
    let mut b: Vec<(corefud::Span, String)> = fm.iter().map(|m| (m.span, m.eid.clone())).collect();
    a.sort();
    b.sort();
    assert_eq!(a, b);
    let g = eval::gold_of(&bdocs[0]).unwrap();
    let s = eval::score_all(std::slice::from_ref(&g), &[g.entities.clone()], crate::score::Matching::Head, true);
    assert!((s.conll() - 1.0).abs() < 1e-9);
    // the gold annotation in the story is readable: John Smith, a doctor, He, himself, him — one entity
    let gold = eval::gold_of(&docs[0]).unwrap();
    assert_eq!(gold.entities[0].len(), 5);
}

/// «The box fell. It <verb>.»: the last sentence's «it» with the given predicate (word, UPOS, XPOS, FEATS).
fn box_then_it(pred: &[&str]) -> String {
    let mut t = String::from("# newdoc id = t3\n");
    t += &sent("t3-1", &[], &["The DET DT Definite=Def|PronType=Art 2 det", "box NOUN NN Number=Sing 3 nsubj", &format!("fell VERB VBD {VBD} 0 root"), ". PUNCT . _ 3 punct"]);
    let it = "It PRON PRP Case=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs";
    let mut words = vec![format!("{it} {} nsubj", pred.len() + 1)];
    for (k, w) in pred.iter().enumerate() {
        words.push(if k + 1 == pred.len() { format!("{w} 0 root") } else { format!("{w} {} cop", pred.len() + 1) });
    }
    words.push(format!(". PUNCT . _ {} punct", pred.len() + 1));
    let refs: Vec<&str> = words.iter().map(String::as_str).collect();
    t += &sent("t3-2", &[], &refs);
    // the lemmas of the test verbs (`row` takes the form as the lemma)
    for (form, lemma) in [("rained", "rain"), ("happened", "happen"), ("broke", "break"), ("was", "be")] {
        t = t.replace(&format!("\t{form}\t{form}\t"), &format!("\t{form}\t{lemma}\t"));
    }
    t
}

/// Pleonastic and event «it» are not linked to a noun (loaiciga-2017 I1, kolhatkar-2018 I1); an anaphoric one is —
/// the negative control: the same text with «broke» links «It» to «The box».
#[test]
fn it_readings_keep_it_from_nouns() {
    let (docs, sys) = resolve(&box_then_it(&[&format!("broke VERB VBD {VBD}")]));
    assert_eq!(chain_of(&docs[0], &sys[0], "It"), ["The box", "It"], "the control: an anaphoric «it» is linked");
    for pred in [vec![format!("rained VERB VBD {VBD}")], vec![format!("happened VERB VBD {VBD}")], vec!["was AUX VBD _".to_string(), "true ADJ JJ Degree=Pos".to_string()]] {
        let p: Vec<&str> = pred.iter().map(String::as_str).collect();
        let (docs, sys) = resolve(&box_then_it(&p));
        let d = &docs[0];
        let it = sys[0].ms.iter().position(|m| m.head_low == "it").unwrap();
        assert!(sys[0].links.iter().all(|l| l.from != it), "«it {}» must not be linked", pred.join(" "));
        let rd = crate::it::reading(&d.sents[1], 0);
        assert_ne!(rd, crate::it::Reading::Anaphoric, "{pred:?}");
    }
}

/// Notional «they» (zeldes-2018 I1–I2): «The family left. They were happy.» links «They» to «The family»; with a plural
/// that agrees strictly («The family met the guests.») «They» goes to the plural — the control that the relaxed
/// agreement is only a fallback.
#[test]
fn notional_they_only_without_a_plural() {
    let they = "They PRON PRP Case=Nom|Number=Plur|Person=3|PronType=Prs";
    let story = |second: &[&str]| {
        let mut t = String::from("# newdoc id = t4\n");
        let mut first = vec!["The DET DT Definite=Def|PronType=Art 2 det", "family NOUN NN Number=Sing 3 nsubj"];
        first.extend_from_slice(second);
        t += &sent("t4-1", &[], &first);
        t += &sent("t4-2", &[], &[&format!("{they} 3 nsubj"), "were AUX VBD Mood=Ind|Number=Plur|Tense=Past|VerbForm=Fin 3 cop", "happy ADJ JJ Degree=Pos 0 root", ". PUNCT . _ 3 punct"]);
        t
    };
    let left = format!("left VERB VBD {VBD} 0 root");
    let (docs, sys) = resolve(&story(&[&left, ". PUNCT . _ 3 punct"]));
    assert_eq!(chain_of(&docs[0], &sys[0], "They"), ["The family", "They"]);
    let met = format!("met VERB VBD {VBD} 0 root");
    let (docs, sys) = resolve(&story(&[&met, "the DET DT Definite=Def|PronType=Art 5 det", "guests NOUN NNS Number=Plur 3 obj", ". PUNCT . _ 3 punct"]));
    assert_eq!(chain_of(&docs[0], &sys[0], "They"), ["the guests", "They"]);
}

#[test]
fn wave27_document_failure_order_quotes_and_retree() {
    let invalid = sent("bad", &[], &["A NOUN NN _ 9 dep", "B NOUN NN _ 2 dep"]);
    assert!(
        doc::documents(&UdDoc::parse(&invalid).unwrap())
            .unwrap_err()
            .to_string()
            .contains("outside")
    );
    let cycle = sent("cycle", &[], &["A NOUN NN _ 2 dep", "B NOUN NN _ 1 dep"]);
    assert!(
        doc::documents(&UdDoc::parse(&cycle).unwrap())
            .unwrap_err()
            .to_string()
            .contains("cycle")
    );
    let mut text = sent(
        "q1",
        &[],
        &["“ PUNCT `` _ 2 punct", "box NOUN NN Number=Sing 0 root"],
    );
    text += &sent("q2", &[], &["lid NOUN NN Number=Sing 0 root"]);
    text += &sent("q3", &["# newpar"], &["other NOUN NN Number=Sing 0 root"]);
    let gold = UdDoc::parse(&text).unwrap();
    let docs = doc::documents(&gold).unwrap();
    assert_eq!(
        docs[0]
            .sents
            .iter()
            .map(|s| s.quote.clone())
            .collect::<Vec<_>>(),
        vec![vec![1, 1], vec![1], vec![0]]
    );
    let replacement = UdDoc::parse(&text.replace("Number=Sing", "Number=Plur")).unwrap();
    let retreed = doc::retree(&gold, &replacement).unwrap();
    assert!(retreed.text().contains("Number=Plur"));
    assert!(retreed.text().contains("# newpar"));
    let mismatched = UdDoc::parse(&text.replace("box", "crate")).unwrap();
    assert!(
        doc::retree(&gold, &mismatched)
            .unwrap_err()
            .to_string()
            .contains("word 2")
    );
}

#[test]
fn wave27_entity_boundaries_and_head_ties() {
    let read = |text: &str| {
        let ud = UdDoc::parse(text).unwrap();
        corefud::read_mentions(&doc::documents(&ud).unwrap()[0])
    };
    let bad = sent("bad", &[], &["box NOUN NN _ 0 root Entity=1)"]);
    assert!(
        read(&bad)
            .unwrap_err()
            .to_string()
            .contains("no open mention")
    );
    let mut cross = sent("a", &[], &["box NOUN NN _ 0 root Entity=(1"]);
    cross += &sent("b", &[], &["lid NOUN NN _ 0 root Entity=1)"]);
    assert!(read(&cross).unwrap_err().to_string().contains("crosses"));
    let nested = sent(
        "nested",
        &[],
        &[
            "A NOUN NN _ 0 root Entity=(1(1)",
            "B NOUN NN _ 0 root Entity=1)",
        ],
    );
    let mentions = read(&nested).unwrap();
    assert_eq!(
        mentions
            .iter()
            .map(|m| (m.span.start, m.span.end))
            .collect::<Vec<_>>(),
        vec![(0, 1), (0, 0)]
    );
    let ud = UdDoc::parse(&sent(
        "heads",
        &[],
        &[
            ", PUNCT , _ 0 root",
            "box NOUN NN _ 0 root",
            "lid NOUN NN _ 0 root",
        ],
    ))
    .unwrap();
    let docs = doc::documents(&ud).unwrap();
    assert_eq!(
        corefud::span_head(
            &docs[0].sents[0],
            corefud::Span {
                sent: 0,
                start: 0,
                end: 2
            }
        ),
        1
    );
}

#[test]
fn wave27_coordination_mentions_and_visibility() {
    let text = sent(
        "coord",
        &[],
        &[
            "Ada PROPN NNP Number=Sing 0 root",
            "and CCONJ CC _ 3 cc",
            "Bea PROPN NNP Number=Sing 1 conj",
        ],
    );
    let ud = UdDoc::parse(&text).unwrap();
    let docs = doc::documents(&ud).unwrap();
    let ms = crate::mention::detect(&docs[0]);
    assert_eq!(
        ms.iter()
            .map(|m| (m.span.start, m.span.end, m.coord, m.number))
            .collect::<Vec<_>>(),
        vec![
            (0, 2, true, crate::mention::PL),
            (0, 0, false, crate::mention::SG),
            (2, 2, false, crate::mention::SG)
        ]
    );
    let sys = eval::resolve(
        &docs[0],
        &Config {
            sieves: vec![],
            ..Config::default()
        },
    );
    assert_eq!(eval::out_mentions(&sys).len(), 2);
    let mut grouped = sys;
    grouped.clusters = vec![vec![0, 1], vec![2]];
    assert_eq!(eval::out_mentions(&grouped).len(), 3);
}

#[test]
fn wave27_event_antecedent_and_empty_evaluation() {
    let text = box_then_it(&[&format!("happened VERB VBD {VBD}")]);
    let ud = UdDoc::parse(&text).unwrap();
    let docs = doc::documents(&ud).unwrap();
    assert_eq!(
        crate::it::antecedent(&docs[0].sents, 1, 0),
        Some((0, 2, 0, 2))
    );
    assert_eq!(crate::it::antecedent(&docs[0].sents, 0, 0), None);
    let empty = UdDoc::parse("").unwrap();
    let input = eval::Input {
        label: "synthetic".into(),
        ud: empty,
        docs: vec![],
    };
    let mut report = String::new();
    eval::run("empty", &[], &input, &mut report, None, None).unwrap();
    assert!(report.contains("Documents 0, gold mentions 0, entities 0"));
    let mut failed_report = String::new();
    let bad = eval::Input {
        label: "bad".into(),
        ud,
        docs,
    };
    assert!(
        eval::run("bad", &[], &bad, &mut failed_report, None, None)
            .unwrap_err()
            .to_string()
            .contains("documents")
    );
    assert!(failed_report.is_empty());
}

// ---- the noun lexicon (`crate::lexicon`): each path, its negative controls, the decision and the sieve feature ----

fn lex_hits(words: &[&str]) -> Vec<(String, &'static str, &'static str)> {
    let ud = UdDoc::parse(&sent("x", &[], words)).unwrap();
    let d = doc::documents(&ud).unwrap();
    crate::lexicon::hits(&d[0].sents[0]).into_iter().map(|h| (h.lemma, crate::lexicon::CLASSES[h.class], h.path)).collect()
}

const NSG: &str = "Number=Sing";

#[test]
fn lexicon_paths_find_their_pairs() {
    // The farmer hurt himself .
    let refl = lex_hits(&["The DET DT _ 2 det", "farmer NOUN NN Number=Sing 3 nsubj", "hurt VERB VBD _ 0 root", "himself PRON PRP Reflex=Yes 3 obj", ". PUNCT . _ 3 punct"]);
    assert_eq!(refl, vec![("farmer".into(), "masc", "refl")]);
    // The hen sat on her eggs .
    let poss = lex_hits(&["The DET DT _ 2 det", "hen NOUN NN Number=Sing 3 nsubj", "sat VERB VBD _ 0 root", "on ADP IN _ 6 case", "her PRON PRP$ Poss=Yes 6 nmod:poss", "eggs NOUN NNS Number=Plur 3 obl", ". PUNCT . _ 3 punct"]);
    assert_eq!(poss, vec![("hen".into(), "fem", "poss")]);
    // The millers say they are poor . (the test rows take the form as the lemma)
    let say = lex_hits(&["The DET DT _ 2 det", "millers NOUN NNS Number=Plur 3 nsubj", "say VERB VBP _ 0 root", "they PRON PRP Case=Nom 6 nsubj", "are AUX VBP _ 6 cop", "poor ADJ JJ _ 3 ccomp", ". PUNCT . _ 3 punct"]);
    assert_eq!(say, vec![("millers".into(), "plur", "say")]);
    // She was a widow .
    let pred = lex_hits(&["She PRON PRP Case=Nom 4 nsubj", "was AUX VBD _ 4 cop", "a DET DT _ 4 det", "widow NOUN NN Number=Sing 0 root", ". PUNCT . _ 4 punct"]);
    assert_eq!(pred, vec![("widow".into(), "fem", "pred")]);
    // the man who came
    let rel = lex_hits(&["the DET DT _ 2 det", "man NOUN NN Number=Sing 0 root", "who PRON WP PronType=Rel 4 nsubj", "came VERB VBD _ 2 acl:relcl"]);
    assert_eq!(rel, vec![("man".into(), "who", "relpron")]);
}

#[test]
fn lexicon_paths_negative_controls() {
    // negation: He is not a woman . — no pair
    assert!(lex_hits(&["He PRON PRP Case=Nom 5 nsubj", "is AUX VBZ _ 5 cop", "not PART RB _ 5 advmod", "a DET DT _ 5 det", "woman NOUN NN Number=Sing 0 root"]).is_empty());
    // a preposition: he was in the road — not a predicate noun
    assert!(lex_hits(&["he PRON PRP Case=Nom 5 nsubj", "was AUX VBD _ 5 cop", "in ADP IN _ 5 case", "the DET DT _ 5 det", "road NOUN NN Number=Sing 0 root"]).is_empty());
    // a name is never counted: John hurt himself .
    assert!(lex_hits(&["John PROPN NNP Number=Sing 2 nsubj", "hurt VERB VBD _ 0 root", "himself PRON PRP Reflex=Yes 2 obj"]).is_empty());
    // a plural noun with a singular pronoun is a path error: The boys hurt himself .
    assert!(lex_hits(&["The DET DT _ 2 det", "boys NOUN NNS Number=Plur 3 nsubj", "hurt VERB VBD _ 0 root", "himself PRON PRP Reflex=Yes 3 obj"]).is_empty());
    // a possessive inside the subject itself is not the subject's: The man's wife lost her ring — wife (fem), not man
    let h = lex_hits(&["The DET DT _ 2 det", "man NOUN NN Number=Sing 4 nmod:poss", "'s PART POS _ 2 case", "wife NOUN NN Number=Sing 5 nsubj", "lost VERB VBD _ 0 root", "her PRON PRP$ Poss=Yes 7 nmod:poss", "ring NOUN NN Number=Sing 5 obj"]);
    assert_eq!(h, vec![("wife".into(), "fem", "poss")]);
}

fn lex_table(rows: &[(&str, [u64; 6])]) -> crate::lexicon::Table {
    crate::lexicon::Table::from_counts(rows.iter().map(|(l, c)| (l.to_string(), crate::lexicon::Row { counts: *c })).collect())
}

#[test]
fn lexicon_decisions_need_evidence_and_a_clear_share() {
    use crate::mention::{ANIM, FEM, INAN, MASC, NEUT};
    let t = lex_table(&[("farmer", [40, 2, 1, 5, 9, 0]), ("widow", [0, 30, 0, 0, 3, 0]), ("stone", [0, 0, 50, 4, 0, 6]), ("fox", [10, 2, 9, 1, 0, 0]), ("doctor", [1, 0, 0, 0, 0, 0])]);
    let f = t.feats("farmer").unwrap();
    assert_eq!((f.gender, f.animacy), (MASC, ANIM));
    assert_eq!(t.feats("widow").map(|f| (f.gender, f.animacy)), Some((FEM, ANIM)));
    assert_eq!(t.feats("stone").map(|f| (f.gender, f.animacy)), Some((NEUT, INAN)));
    // mixed (he and it) stays unknown; one count is not evidence
    assert_eq!(t.feats("fox"), None);
    assert_eq!(t.feats("doctor"), None);
    assert_eq!(t.feats("unseen"), None);
    // the rotation control moves every row to another lemma
    let r = crate::lexicon::rotate(&t, 1);
    assert!(t.rows.keys().all(|l| r.rows[l] != t.rows[l]));
}

#[test]
fn lexicon_feature_keeps_it_from_a_person_noun() {
    // The farmer came . It was late . — «it» is pleonastic here; use: The farmer saw the stone . It fell .
    let mut text = String::from("# newdoc id = lx\n");
    text += &sent("lx-1", &[], &["The DET DT _ 2 det", "farmer NOUN NN Number=Sing 3 nsubj", "saw VERB VBD _ 0 root", "the DET DT _ 5 det", "stone NOUN NN Number=Sing 3 obj", ". PUNCT . _ 3 punct"]);
    text += &sent("lx-2", &[], &["It PRON PRP Case=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs 2 nsubj", "fell VERB VBD _ 0 root", ". PUNCT . _ 2 punct"]);
    // without the table «it» takes the subject first (Hobbs order: subject before object)
    crate::lexicon::TEST_TABLE.with(|t| *t.borrow_mut() = Some(lex_table(&[])));
    let (docs, sys) = resolve(&text);
    let without = chain_of(&docs[0], &sys[0], "It");
    crate::lexicon::TEST_TABLE.with(|t| *t.borrow_mut() = Some(lex_table(&[("farmer", [40, 2, 1, 5, 9, 0]), ("stone", [0, 0, 50, 4, 0, 6])])));
    let (docs, sys) = resolve(&text);
    let with = chain_of(&docs[0], &sys[0], "It");
    crate::lexicon::TEST_TABLE.with(|t| *t.borrow_mut() = None);
    assert!(without.iter().any(|m| m.contains("farmer")), "{without:?}");
    assert!(with.iter().any(|m| m.contains("stone")) && !with.iter().any(|m| m.contains("farmer")), "{with:?}");
}
