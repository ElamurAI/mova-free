//! Tests on a small synthetic graph (no raw data): snapshot, gates, inheritance, layers, the Nixon
//! diamond, queries, evaluation, Wikidata dump parsing. Every gate comes with a negative control.

use crate::kb::*;
use crate::model::*;
use crate::query::{self, Verdict};

/// Synsets: (offset, lemma, lexfile, gloss) and IsA/InstanceOf/PartOf edges between their numbers.
fn mini() -> Graph {
    let syn: &[(u32, &str, &str)] = &[
        (1, "entity", "noun.Tops"),     // 0
        (2, "object", "noun.Tops"),     // 1
        (3, "animal", "noun.Tops"),     // 2
        (4, "bird", "noun.animal"),     // 3
        (5, "penguin", "noun.animal"),  // 4
        (6, "crow", "noun.animal"),     // 5
        (7, "mammal", "noun.animal"),   // 6
        (8, "bat", "noun.animal"),      // 7
        (9, "whale", "noun.animal"),    // 8
        (10, "fish", "noun.animal"),    // 9
        (11, "person", "noun.Tops"),    // 10
        (12, "Quaker", "noun.person"),  // 11
        (13, "Republican", "noun.person"), // 12
        (14, "Nixon", "noun.person"),   // 13
        (15, "wing", "noun.animal"),    // 14
        (16, "whale", "noun.person"),   // 15 — "a big person", the first sense of whale
        (17, "stone", "noun.object"),   // 16
    ];
    let isa: &[(u32, u32)] = &[(1, 0), (2, 1), (3, 2), (4, 3), (5, 3), (6, 2), (7, 6), (8, 6), (9, 2), (10, 1), (11, 10), (12, 10), (15, 10), (16, 1), (14, 1)];
    let inst: &[(u32, u32)] = &[(13, 10)];
    let part: &[(u32, u32)] = &[(14, 3)];
    let mut g = Graph::default();
    // words: sorted
    let mut words: Vec<&str> = syn.iter().map(|s| s.1).collect();
    words.sort_unstable();
    words.dedup();
    g.word_off.push(0);
    for w in &words {
        g.words.push_str(w);
        g.word_off.push(g.words.len() as u32);
    }
    g.gloss_off.push(0);
    g.name_off.push(0);
    g.mem_off.push(0);
    for (k, w, lf) in syn {
        g.src.push(Src::Oewn);
        g.key.push(*k);
        g.pos.push(Pos::Noun);
        g.lexfile.push(lexfile_id(lf).unwrap());
        g.flags.push(0);
        g.ili.push(0);
        g.gloss.push_str(&format!("gloss of {w}"));
        g.gloss_off.push(g.gloss.len() as u32);
        g.name_off.push(0);
        g.mem.push(words.binary_search(w).unwrap() as Wid);
        g.mem_off.push(g.mem.len() as u32);
    }
    // senses: whale — first "a big person" (15), then the animal (8)
    let mut per: Vec<Vec<Cid>> = vec![vec![]; words.len()];
    for (i, (_, w, _)) in syn.iter().enumerate() {
        per[words.binary_search(w).unwrap()].push(i as Cid);
    }
    let wh = words.binary_search(&"whale").unwrap();
    per[wh] = vec![15, 8];
    g.sense_off.push(0);
    for v in per {
        g.senses.extend(v);
        g.sense_off.push(g.senses.len() as u32);
    }
    let n = syn.len();
    let mut edges: Vec<(Cid, Edge)> = Vec::new();
    for (a, b) in isa {
        edges.push((*a, Edge { rel: Rel::IsA, to: *b }));
    }
    for (a, b) in inst {
        edges.push((*a, Edge { rel: Rel::InstanceOf, to: *b }));
    }
    for (a, b) in part {
        edges.push((*a, Edge { rel: Rel::PartOf, to: *b }));
    }
    let csr = |mut v: Vec<(Cid, Edge)>| {
        v.sort_by_key(|(s, e)| (*s, e.rel as u8, e.to));
        let mut off = vec![0u32; n + 1];
        for (s, _) in &v {
            off[*s as usize + 1] += 1;
        }
        for i in 0..n {
            off[i + 1] += off[i];
        }
        (off, v.into_iter().map(|x| x.1).collect::<Vec<_>>())
    };
    (g.out_off, g.out) = csr(edges.clone());
    (g.in_off, g.inn) = csr(edges.iter().map(|(s, e)| (e.to, Edge { rel: e.rel, to: *s })).collect());
    crate::store::check(&g).unwrap();
    g
}

fn a(id: &str, layer: LayerId, c: Cid, p: Prop, v: bool, s: Strength) -> Assert {
    Assert { id: id.into(), layer, concept: c, prop: p, value: v, strength: s, source: "test".into(), note: String::new() }
}

fn kb() -> Kb {
    let mut k = Kb::new(mini());
    k.layers.push(Layer { name: "fable".into(), kind: LayerKind::Genre, parent: Some(0), note: String::new() });
    k.layers.push(Layer { name: "tale:x".into(), kind: LayerKind::Tale, parent: Some(1), note: String::new() });
    k.add_assert(a("d1", 0, 3, Prop::CanFly, true, Strength::Default)).unwrap();
    k.add_assert(a("d2", 0, 4, Prop::CanFly, false, Strength::Default)).unwrap();
    k.add_assert(a("d3", 0, 6, Prop::CanFly, false, Strength::Default)).unwrap();
    k.add_assert(a("d4", 0, 7, Prop::CanFly, true, Strength::Default)).unwrap();
    k.add_assert(a("d5", 0, 2, Prop::CanSpeak, false, Strength::Default)).unwrap();
    k.add_assert(a("d6", 1, 2, Prop::CanSpeak, true, Strength::Default)).unwrap();
    k.add_assert(a("d7", 0, 3, Prop::LaysEggs, true, Strength::Strict)).unwrap();
    k.add_assert(a("d8", 0, 11, Prop::Pacifist, true, Strength::Default)).unwrap();
    k.add_assert(a("d9", 0, 12, Prop::Pacifist, false, Strength::Default)).unwrap();
    k.add_assert(a("d10", 0, 8, Prop::CanSwim, true, Strength::Default)).unwrap();
    k.add_extra(Extra { id: "r1".into(), rel: Rel::InstanceOf, from: 13, to: 11, source: "test".into() }).unwrap();
    k.add_extra(Extra { id: "r2".into(), rel: Rel::InstanceOf, from: 13, to: 12, source: "test".into() }).unwrap();
    k
}

fn val(k: &Kb, c: Cid, p: Prop, l: LayerId) -> Option<bool> {
    match k.prop(c, p, l) {
        PropAnswer::Known { value, .. } => Some(value),
        _ => None,
    }
}

#[test]
fn snapshot_round_trip_and_corruption() {
    let g = mini();
    let b = crate::store::pack(&g);
    assert_eq!(crate::store::unpack(&b).unwrap(), g);
    for i in [0, 8, 12, 22, 30, 40, b.len() / 2, b.len() - 1] {
        let mut x = b.clone();
        x[i] ^= 0x10;
        assert!(crate::store::unpack(&x).is_err(), "byte {i} corrupted, yet the snapshot was read");
    }
    assert!(crate::store::unpack(&b[..b.len() - 1]).is_err());
    let mut long = b.clone();
    long.push(0);
    assert!(crate::store::unpack(&long).is_err());
}

#[test]
fn defaults_exceptions_most_specific_wins() {
    let k = kb();
    assert_eq!(val(&k, 5, Prop::CanFly, 0), Some(true), "a crow flies — from bird");
    assert_eq!(val(&k, 4, Prop::CanFly, 0), Some(false), "the penguin is an exception");
    assert_eq!(val(&k, 7, Prop::CanFly, 0), Some(true), "the bat is an exception among mammals");
    assert_eq!(val(&k, 8, Prop::CanFly, 0), Some(false), "the whale is a mammal");
    assert!(matches!(k.prop(16, Prop::CanFly, 0), PropAnswer::Unknown), "no seeds about stone — unknown");
    // proof: the chain to the winning seed's concept and the overridden ancestor seed
    match k.prop(4, Prop::CanFly, 0) {
        PropAnswer::Known { win, chain, beaten, .. } => {
            assert_eq!(k.asserts[win].id, "d2");
            assert!(chain.steps.is_empty());
            assert_eq!(beaten.iter().map(|i| k.asserts[*i].id.as_str()).collect::<Vec<_>>(), ["d1"]);
        }
        x => panic!("{x:?}"),
    }
}

#[test]
fn layers_do_not_leak() {
    let k = kb();
    assert_eq!(val(&k, 5, Prop::CanSpeak, 0), Some(false), "a real crow does not speak");
    assert_eq!(val(&k, 5, Prop::CanSpeak, 1), Some(true), "a fable crow does");
    assert_eq!(val(&k, 5, Prop::CanSpeak, 2), Some(true), "a tale inherits the genre");
    assert_eq!(val(&k, 4, Prop::CanFly, 2), Some(false), "a real-world exception is visible in a tale too");
    // negative control: the flaw "all layers visible" — the real crow starts talking
    let mut bad = kb();
    bad.leak_all_layers = true;
    assert_eq!(val(&bad, 5, Prop::CanSpeak, 0), Some(true));
}

#[test]
fn nixon_diamond_is_undetermined_with_both_proofs() {
    let k = kb();
    match k.prop(13, Prop::Pacifist, 0) {
        PropAnswer::Undetermined { cands, .. } => {
            let mut ids: Vec<&str> = cands.iter().map(|(i, _)| k.asserts[*i].id.as_str()).collect();
            ids.sort();
            assert_eq!(ids, ["d8", "d9"]);
            assert!(cands.iter().all(|(_, ch)| ch.steps.len() == 1 && ch.steps[0].extra.is_some()));
        }
        x => panic!("the diamond must be undetermined: {x:?}"),
    }
}

#[test]
fn gates_are_not_mute() {
    let mut k = kb();
    // IsA cycle: animal IsA crow — refusal
    let e = k.add_extra(Extra { id: "x".into(), rel: Rel::IsA, from: 2, to: 5, source: "t".into() }).unwrap_err().to_string();
    assert!(e.contains("cycle"), "{e}");
    assert!(k.isa_cycle().is_none());
    // past the gate — the full check finds the cycle
    k.gates = false;
    k.add_extra(Extra { id: "x".into(), rel: Rel::IsA, from: 2, to: 5, source: "t".into() }).unwrap();
    assert!(k.isa_cycle().is_some());
    // Strict: penguin LaysEggs=no on the same layer — refusal; on the fable layer — allowed
    let mut k = kb();
    assert!(k.add_assert(a("x1", 0, 4, Prop::LaysEggs, false, Strength::Default)).is_err());
    assert!(k.add_assert(a("x2", 1, 4, Prop::LaysEggs, false, Strength::Default)).is_ok());
    // contradiction on the same concept and layer, duplicate id
    assert!(k.add_assert(a("x3", 0, 3, Prop::CanFly, false, Strength::Default)).is_err());
    assert!(k.add_assert(a("d1", 0, 9, Prop::CanFly, false, Strength::Default)).is_err());
    // the lemma must be a member of the synset
    assert!(k.resolve_seed_concept("bird", "oewn-00000004-n").is_ok());
    assert!(k.resolve_seed_concept("crow", "oewn-00000004-n").is_err());
    assert!(k.resolve_seed_concept("bird", "oewn-99999999-n").is_err());
}

#[test]
fn queries_answer_with_proofs_and_refuse_to_guess() {
    let k = kb();
    let yes = query::isa(&k, "crow", "animal");
    assert_eq!(yes.verdict, Verdict::Yes);
    assert!(yes.lines.iter().any(|l| l.contains("IsA")));
    // whale: the first sense is "a big person", but with "fish"/"mammal" the context is the animal
    assert_eq!(query::isa(&k, "whale", "mammal").verdict, Verdict::Yes);
    let no = query::isa(&k, "whale", "fish");
    assert_eq!(no.verdict, Verdict::No);
    assert!(no.lines.iter().any(|l| l.contains("diverge")));
    assert_eq!(query::isa(&k, "bat", "bird").verdict, Verdict::No);
    assert_eq!(query::isa(&k, "blorft", "animal").verdict, Verdict::Unknown);
    assert_eq!(query::isa(&k, "crow", "blorft").verdict, Verdict::Unknown);
    // a property with a preference: CanSwim — the animal sense, not "a big person"
    assert_eq!(query::prop(&k, "whale", Prop::CanSwim, 0).verdict, Verdict::Yes);
    assert_eq!(query::prop(&k, "crows", Prop::CanFly, 0).verdict, Verdict::Yes, "plural");
    assert_eq!(query::prop(&k, "zzyzx", Prop::CanFly, 0).verdict, Verdict::Unknown);
    assert_eq!(query::prop(&k, "Nixon", Prop::Pacifist, 0).verdict, Verdict::Undetermined);
    // parts are inherited: a crow has a wing from bird; what is missing is "unknown", not "no"
    assert_eq!(query::has_part(&k, "crow", "wing", Rel::PartOf).verdict, Verdict::Yes);
    assert_eq!(query::has_part(&k, "fish", "wing", Rel::PartOf).verdict, Verdict::Unknown);
    // a query as a string with a layer
    assert_eq!(query::ask(&k, "can crow speak --layer fable").unwrap().verdict, Verdict::Yes);
    assert!(query::ask(&k, "can crow speak --layer nowhere").is_err());
}

#[test]
fn scoring() {
    use crate::eval::{score, Status};
    let ans = |v: Verdict, values: &[&str]| query::Answer { verdict: v, values: values.iter().map(|s| s.to_string()).collect(), lines: vec![] };
    assert_eq!(score(&ans(Verdict::Yes, &[]), "yes"), Status::Correct);
    assert_eq!(score(&ans(Verdict::Unknown, &[]), "yes"), Status::Unknown);
    assert_eq!(score(&ans(Verdict::No, &[]), "yes"), Status::Wrong);
    assert_eq!(score(&ans(Verdict::No, &[]), "unknown"), Status::Wrong);
    assert_eq!(score(&ans(Verdict::Unknown, &[]), "unknown"), Status::Correct);
    assert_eq!(score(&ans(Verdict::Yes, &["Commerce_buy", "Fall_for"]), "Commerce_buy"), Status::Correct);
    assert_eq!(score(&ans(Verdict::Yes, &["Fall_for"]), "Commerce_buy"), Status::Wrong);
}

#[test]
fn frozen_hash_gate() {
    let dir = std::env::temp_dir().join(format!("onto-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join("set.tsv");
    std::fs::write(&p, "q1\tisa\tisa crow bird\tyes\n").unwrap();
    std::fs::write(dir.join("set.sha256"), format!("{}  set.tsv\n", crate::eval::sha256_hex(b"q1\tisa\tisa crow bird\tyes\n"))).unwrap();
    assert!(crate::eval::read_frozen(&p).is_ok());
    std::fs::write(&p, "q1\tisa\tisa crow bird\tno\n").unwrap();
    assert!(crate::eval::read_frozen(&p).is_err(), "a changed set must not pass");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn world_command_fields() {
    let l = r#"char C1 type=animal class=crow name="the Crow" at=L1 gender=female ^s1"#;
    assert_eq!(crate::bridge::tale_of("cmds-2451-fox-crow.txt"), "2451");
    assert_eq!(crate::bridge::tale_of("cmds-v3-merchant.txt"), "arabian-nights");
    let dir = std::env::temp_dir().join(format!("onto-world-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join("cmds-2451.txt");
    std::fs::write(&f, format!("{l}\nobj O1 type=food class=meat name=\"a bit of meat\" ^s1\nevent E1 verb=steal agent=C1 ^s1\n")).unwrap();
    let (ents, _) = crate::bridge::read_world(&[f]).unwrap();
    assert_eq!(ents.len(), 2);
    assert_eq!(ents[0].class.as_deref(), Some("crow"));
    assert_eq!(ents[0].name.as_deref(), Some("the Crow"));
    assert_eq!(ents[1].ty, "food");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn prop_names_parse() {
    assert_eq!(Prop::parse("fly"), Some(Prop::CanFly));
    assert_eq!(Prop::parse("CanFly"), Some(Prop::CanFly));
    assert_eq!(Prop::parse("lays_eggs"), Some(Prop::LaysEggs));
    assert_eq!(Prop::parse("talk"), Some(Prop::CanSpeak));
    assert_eq!(Prop::parse("food"), None);
}

/// v3: pronoun API — gender, animacy, humanness; a fable character; an unknown word — None.
#[test]
fn pronoun_api() {
    use crate::pron::*;
    assert_eq!(gender_cue("adult female chicken").map(|x| x.0), Some(Gender::Fem));
    assert_eq!(gender_cue("a male sovereign; ruler of a kingdom").map(|x| x.0), Some(Gender::Masc));
    assert!(gender_cue("a large spotted feline").is_none());
    assert!(gender_cue("either male or female").is_none(), "both cues — unknown, not a guess");
    let mut k = kb();
    k.add_assert(a("h1", 0, 2, Prop::Animate, true, Strength::Default)).unwrap();
    k.add_assert(a("h2", 0, 10, Prop::Human, true, Strength::Default)).unwrap();
    k.add_assert(a("h3", 0, 2, Prop::Human, false, Strength::Default)).unwrap();
    k.add_assert(a("h4", 0, 1, Prop::Animate, false, Strength::Default)).unwrap();
    k.add_assert(a("h5", 0, 10, Prop::Animate, true, Strength::Default)).unwrap();
    let real = Ctx { sentence: None, layer: 0 };
    let crow = info(&k, "crow", &real).unwrap();
    assert_eq!((crow.animate, crow.human, crow.gender), (Tri::Yes, Tri::No, Gender::Neut));
    assert_eq!(crow.pronouns(), &["it"]);
    // in a fable the crow speaks (d6) — a character: he, she or it
    let fable = Ctx { sentence: None, layer: 1 };
    let f = info(&k, "crow", &fable).unwrap();
    assert!(f.persona && f.pronouns().contains(&"she") && f.pronouns().contains(&"it"));
    // Nixon is a person, no gender in the gloss → he or she
    let n = info(&k, "Nixon", &real).unwrap();
    assert_eq!((n.human, n.animate, n.gender), (Tri::Yes, Tri::Yes, Gender::Unknown));
    assert_eq!(n.pronouns(), &["he", "she"]);
    // a stone is inanimate
    let s = info(&k, "stone", &real).unwrap();
    assert_eq!((s.animate, s.pronouns()), (Tri::No, &["it"][..]));
    assert_eq!(gender(&k, "stone", &real), Some(Gender::Neut));
    assert_eq!(human(&k, "crow", &real), Some(Tri::No));
    assert_eq!(animate(&k, "Nixon", &real), Some(Tri::Yes));
    // an unknown word — None, not a guess
    assert!(info(&k, "blorft", &real).is_none());
}

/// v3: core completeness gate — a removed cell, an unknown status, two numbers — an error naming the concept and field.
#[test]
fn core_completeness_gate() {
    use crate::coregate::parse;
    let core = "fox\tanimal\nbread\tfood\n";
    let row = |c: &str, d: &str, f: &str, st: &str| format!("{c}\t{d}\toewn-1-n\t{f}\t{st}\t\t\t\t\t\t\t\t\t\t\t\t\t\t\twhy\n");
    let mut ok = String::new();
    for (c, d) in [("fox", "animal"), ("bread", "food")] {
        for f in crate::coregate::FIELDS {
            ok.push_str(&row(c, d, f, "unknown"));
        }
    }
    assert!(parse(core, &ok).is_ok());
    let cut: String = ok.lines().filter(|l| !(l.starts_with("bread") && l.contains("\tmass\t"))).map(|l| format!("{l}\n")).collect();
    let e = parse(core, &cut).err().unwrap();
    assert!(e.contains("bread") && e.contains("mass"), "{e}");
    let bad = ok.replacen("\tunknown\t", "\tmaybe\t", 1);
    assert!(parse(core, &bad).is_err());
    let none_food = ok.replacen("fox\tanimal\toewn-1-n\tfood\tunknown", "fox\tanimal\toewn-1-n\tfood\tnone", 1);
    assert!(parse(core, &none_food).err().unwrap().contains("none"));
}

/// v3: WSD — word stems and context without stop words.
#[test]
fn wsd_stems() {
    use crate::wsd::{content, stem};
    assert_eq!(stem("dolphins"), "dolphin");
    assert_eq!(stem("berries"), "berry");
    assert_eq!(stem("glass"), "glass");
    assert_eq!(content("The old oak dropped its acorns"), vec!["old", "oak", "dropp", "acorn"]);
}

#[test]
fn hs_isa_gate() {
    // IsA gate for HS: "goods ⊂ group head" — yes, other sense, part, sisters, modifier; negative control — "no"
    use crate::hsmap::{isa_verdict, IsaV};
    let k = kb();
    let v = |x: Cid, xs: &[Cid], h: Cid, mods: &[Cid]| isa_verdict(&k, x, xs, &[h], &[h], mods).0;
    assert_eq!(v(5, &[5], 3, &[]), IsaV::Yes, "crow ⊂ bird");
    assert_eq!(v(15, &[15, 8], 6, &[]), IsaV::OtherSense, "whale the person, but whale the animal ⊂ mammal");
    assert_eq!(v(14, &[14], 3, &[]), IsaV::Part, "wing is a part of bird");
    assert_eq!(v(4, &[4], 5, &[]), IsaV::Near, "penguin and crow are sisters under bird");
    assert_eq!(v(16, &[16], 9, &[1]), IsaV::Modifier, "stone ⊂ object — a modifier of the group head");
    // negative control: goods from a different group
    assert_eq!(v(16, &[16], 3, &[]), IsaV::No, "stone ⊄ bird");
    assert_eq!(v(5, &[5], 11, &[]), IsaV::No, "crow ⊄ Quaker");
}
