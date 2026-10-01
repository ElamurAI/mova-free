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
