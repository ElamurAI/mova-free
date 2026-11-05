//! End-to-end checks of the dossier → world → query path (they need Mova's model: `WORLD_EN_MODEL`).
use crate::dossier::tests::{cfg, tmp};
use crate::dossier::{Dossier, Kind, NewText};
use crate::query;
use crate::world::World;
use std::collections::BTreeMap;
use std::path::Path;

fn model() -> bool {
    let ok = std::env::var_os("WORLD_EN_MODEL").is_some_and(|p| Path::new(&p).is_file());
    if !ok {
        eprintln!("skip: WORLD_EN_MODEL is not set to a Mova model (en.bin)");
    }
    ok
}

const A: &str = "# Automatic annotation of error types\n\n## Introduction\n\nGrammatical Error Correction (GEC) is the task of correcting errors in text written by learners. We present ERRANT, a toolkit that annotates edits with error types. ERRANT classifies each edit with rules.\n\n## Results\n\nERRANT achieves an accuracy of 95% on the test set.\n";
const B: &str = "# Evaluating grammatical error correction\n\n## Introduction\n\nGEC systems are usually evaluated with the M2 scorer. The M2 scorer is a metric that compares system edits with gold edits. We show that the error types of ERRANT help the evaluation.\n";
const C: &str = "# Protein folding\n\n## Introduction\n\nProteins fold into stable structures. Folding kinetics is measured with fluorescence.\n";

fn add(d: &Dossier, epoch: u64, kind: Kind, text: &str, licence: Option<&str>) -> String {
    d.add(NewText { epoch, kind, text, source: "test", licence, local_only: false, inputs: vec![], created_unix: Some(1) }).unwrap().id
}

fn files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut m = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(p) = stack.pop() {
        for e in std::fs::read_dir(&p).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                stack.push(p);
            } else {
                m.insert(p.strip_prefix(root).unwrap().display().to_string(), std::fs::read(&p).unwrap());
            }
        }
    }
    m.remove("world.lock");
    m
}

fn build(name: &str) -> (std::path::PathBuf, crate::config::Config) {
    let d = tmp(name);
    let c = cfg(&d);
    add(&Dossier::open(&c, "pa-2017-error-types").unwrap(), 0, Kind::Original, A, Some("CC BY 4.0"));
    add(&Dossier::open(&c, "ra-2015-gec-evaluation").unwrap(), 0, Kind::Original, B, Some("CC BY-SA 4.0"));
    let x = Dossier::open(&c, "oz-2010-protein-folding").unwrap();
    add(&x, 0, Kind::Original, C, Some("CC BY 4.0"));
    add(&x, 0, Kind::Web, "# A blog\n\n## Post\n\nProteins fold fast in the cell.\n", Some("CC0 1.0"));
    let mut w = World::open(&c).unwrap();
    w.add(&c, 1, &[]).unwrap();
    (d, c)
}

#[test]
fn world_queries_cite_and_are_deterministic() {
    if !model() {
        return;
    }
    let (d1, c1) = build("w1");
    let (d2, c2) = build("w2");
    // Two fresh builds are byte-identical.
    assert_eq!(files(&c1.world_root), files(&c2.world_root));
    let mut w = World::open(&c1).unwrap();
    assert_eq!(w.meta.docs, 3);
    // Licence: the CC BY-SA original and the CC0 page are never public in the world.
    for t in &w.texts {
        if t.kind == "web" || t.id.starts_with("ra-") {
            assert!(!t.public_ok, "{}", t.id);
        }
    }
    let e = query::entity(&mut w, "GEC", 1).unwrap();
    assert_eq!(e["kind"], "acronym");
    assert!(e["definitions"].as_array().unwrap().iter().any(|x| x["expansion"].as_str().unwrap().to_lowercase().contains("grammatical error correction")));
    let ex = query::explain(&mut w, "GEC", 5, 0, 1).unwrap();
    let first = &ex["explanations"][0];
    assert!(first["sentence"].as_str().unwrap().contains("Grammatical Error Correction (GEC)"), "{ex}");
    // Every statement and explanation cites a document of the world and a sentence of its text.
    let pack = query::context(&mut w, "pa-2017-error-types", 1, 20, 1, None).unwrap();
    let mut cites = vec![];
    for s in pack["statements"].as_array().unwrap() {
        cites.push(s.clone());
    }
    for en in pack["entities"].as_array().unwrap() {
        cites.extend(en["explanations"].as_array().unwrap().iter().cloned());
    }
    assert!(!pack["statements"].as_array().unwrap().is_empty());
    for c in &cites {
        let doc = c["doc"].as_str().unwrap();
        assert!(w.docs.contains_key(doc), "{c}");
        let t = w.texts.iter().find(|t| t.id == c["text"].as_str().unwrap()).expect("text resolves");
        assert_eq!(t.doc, doc);
        let s = w.sentence(t.t, c["s"].as_u64().unwrap() as u32).unwrap();
        assert!(c.get("sentence").is_none_or(|x| x == &serde_json::json!(s.text)));
    }
    // The GEC papers are each other's nearest; the protein paper is not related by entities.
    let r = query::related(&mut w, "pa-2017-error-types", 10, 0, 1).unwrap();
    assert_eq!(r["related"][0]["doc"], "ra-2015-gec-evaluation");
    let nb = query::neighbourhood(&mut w, "pa-2017-error-types", 1, 20, 1).unwrap();
    assert_eq!(nb["docs"][0]["distance"], 0);
    assert_eq!(nb["docs"][1]["doc"], "ra-2015-gec-evaluation");
    let mut lines = 0;
    query::neighbourhood_content(&mut w, "pa-2017-error-types", 1, 20, 1, &mut |_| {
        lines += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(lines, nb["docs"].as_array().unwrap().iter().map(|x| x["sentences"].as_u64().unwrap()).sum::<u64>() as usize);
    let s = query::sentences(&mut w, "edits", 3, 0, 1).unwrap();
    assert!(!s["sentences"].as_array().unwrap().is_empty());
    // Byte cap: the pack shrinks and says what it cut.
    let small = query::context(&mut w, "pa-2017-error-types", 1, 20, 1, Some(1500)).unwrap();
    assert!(small["bytes"].as_u64().unwrap() <= 1600 && small["cut"]["statements"].as_u64().unwrap() > 0);
    let _ = (std::fs::remove_dir_all(d1), std::fs::remove_dir_all(d2));
}

#[test]
fn epochs_are_incremental_and_old_worlds_stay() {
    if !model() {
        return;
    }
    let (d, c) = build("epochs");
    let before = files(&c.world_root);
    let w0 = World::open(&c).unwrap();
    let s1 = w0.stats(1);
    // Epoch 1: Mova writes about the first paper. world(1) does not see it; world(2) does.
    let x = Dossier::open(&c, "pa-2017-error-types").unwrap();
    let id = add(&x, 1, Kind::Mova, "ERRANT is a toolkit for error types. It annotates edits automatically.", None);
    let mut w = World::open(&c).unwrap();
    // Building epoch 1 again adds nothing.
    assert_eq!(w.add(&c, 1, &[]).unwrap()["added_texts"], serde_json::json!([]));
    let r = w.add(&c, 2, &[]).unwrap();
    assert_eq!(r["added_texts"], serde_json::json!([id]));
    // A plain draft without headings is still read (the heading fallback).
    assert!(World::open(&c).unwrap().texts.last().unwrap().sentences > 0);
    let after = files(&c.world_root);
    // Old shards are untouched; texts.jsonl only grew.
    for (k, v) in &before {
        if k.starts_with("shards/") {
            assert_eq!(after.get(k), Some(v), "{k}");
        }
    }
    assert!(after["texts.jsonl"].starts_with(&before["texts.jsonl"]));
    let w = World::open(&c).unwrap();
    assert_eq!(w.stats(1)["texts"], s1["texts"]);
    assert_eq!(w.stats(2)["texts"].as_u64().unwrap(), s1["texts"].as_u64().unwrap() + 1);
    let mut w = w;
    let at1 = query::explain(&mut w, "ERRANT", 10, 0, 1).unwrap();
    assert!(at1["explanations"].as_array().unwrap().iter().all(|e| e["epoch"] == 0));
    // A changed dossier file is refused and nothing is written.
    add(&x, 2, Kind::Sol, "A consultant draft about ERRANT.", None);
    let row = x.select(Some(2), Some(Kind::Sol)).unwrap().remove(0);
    std::fs::write(x.dir.join(&row.file), "edited").unwrap();
    let snapshot = files(&c.world_root);
    assert!(w.add(&c, 3, &[]).unwrap_err().to_string().contains("changed"));
    assert_eq!(files(&c.world_root), snapshot);
    let _ = std::fs::remove_dir_all(d);
}
