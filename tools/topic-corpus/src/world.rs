//! The small world of one paper, built against the big cross corpus: the paper's entities (acronyms,
//! names, frequent nouns) with where the paper mentions and defines them and which documents of the
//! corpus speak of them, and its statements sorted into claims, methods and results with their
//! frames and entities. A Whale writes its description from this world and cites the documents it
//! links to. Written to `<corpus>/world.json` (internal).
use crate::corpus::{self, Corpus, Sentence};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const SCHEMA: &str = "mova-paper-world-v1";

/// An acronym: 2–8 ASCII capitals or digits with at least two capitals.
pub fn acronym(w: &str) -> bool {
    (2..=8).contains(&w.chars().count()) && w.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) && w.chars().filter(|c| c.is_ascii_uppercase()).count() >= 2
}

/// The kind of a statement by its part and wording.
pub fn kind(s: &Sentence) -> &'static str {
    let part = s.part.to_lowercase();
    let lemmas: BTreeSet<&str> = s.words.iter().map(|w| w.1.as_str()).collect();
    let number = s.words.iter().any(|w| w.2 == "NUM") && (s.text.contains('%') || ["score", "accuracy", "precision", "recall", "f1", "f-score", "bleu", "improvement"].iter().any(|k| s.text.to_lowercase().contains(k)));
    if number || ["result", "evaluation", "experiment", "analysis"].iter().any(|k| part.contains(k)) {
        "result"
    } else if ["method", "approach", "model", "system", "algorithm", "architecture", "framework", "implementation", "data"].iter().any(|k| part.contains(k)) {
        "method"
    } else if ["show", "find", "propose", "present", "demonstrate", "achieve", "outperform", "introduce", "argue", "conclude"].iter().any(|v| lemmas.contains(v)) {
        "claim"
    } else {
        "statement"
    }
}

pub fn build(c: &mut Corpus, max_entities: usize) -> Result<Value> {
    let paper = c.paper();
    let pid = format!("lib:{paper}");
    let sents = c.parsed(&pid)?;
    // Entities: acronyms and proper names met at least twice, nouns met at least three times.
    let mut count: BTreeMap<String, (usize, Vec<usize>)> = BTreeMap::new();
    for s in &sents {
        let mut seen = BTreeSet::new();
        for w in &s.words {
            let key = if acronym(&w.0) || (w.2 == "PROPN" && w.0.chars().next().is_some_and(char::is_uppercase)) {
                w.0.clone()
            } else if w.2 == "NOUN" && w.1.chars().count() >= 4 && w.1.chars().all(char::is_alphabetic) {
                w.1.clone()
            } else {
                continue;
            };
            if seen.insert(key.clone()) {
                let e = count.entry(key).or_default();
                e.0 += 1;
                e.1.push(s.s);
            }
        }
    }
    let mut ents: Vec<(String, usize, Vec<usize>)> = count.into_iter().filter(|(k, (n, _))| if k.chars().next().is_some_and(char::is_uppercase) { *n >= 2 } else { *n >= 3 }).map(|(k, (n, v))| (k, n, v)).collect();
    ents.sort_by(|a, b| acronym(&b.0).cmp(&acronym(&a.0)).then(b.1.cmp(&a.1)).then_with(|| a.0.cmp(&b.0)));
    ents.truncate(max_entities);
    let ids: Vec<String> = c.docs.iter().filter(|d| d.id != pid).map(|d| d.id.clone()).collect();
    let mut entities = vec![];
    for (i, (e, n, at)) in ents.iter().enumerate() {
        let defined = sents.iter().find_map(|s| {
            let forms: Vec<String> = s.words.iter().map(|w| w.0.clone()).collect();
            if acronym(e) { mova_whale::knowledge::expansion(e, &forms).map(|x| json!({"s": s.s, "expansion": x})) } else { None }
        });
        let mut links = vec![];
        for id in &ids {
            let t = c.text(id)?;
            let hits = if e.chars().next().is_some_and(char::is_uppercase) { t.matches(e.as_str()).count() } else { t.to_lowercase().matches(e.as_str()).count() };
            if hits > 0 {
                let d = c.doc(id).unwrap();
                links.push((hits, id.clone(), d.kind.clone()));
            }
        }
        links.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        links.truncate(8);
        // The best explanation from a neighbour paper, for the first entities only (parsing is lazy).
        let best = if i < 10 && defined.is_none() { corpus::explain(c, e, Some(&pid), &["paper"], 1)?.into_iter().next().map(|x| json!({"doc": x.doc, "s": x.s, "text": x.text})) } else { None };
        entities.push(json!({"entity": e, "mentions": n, "sentences": at.iter().take(10).collect::<Vec<_>>(), "defined": defined, "explained_by": best, "linked": links.iter().map(|(h, id, k)| json!({"doc": id, "kind": k, "occurrences": h})).collect::<Vec<_>>()}));
    }
    let names: BTreeSet<&str> = ents.iter().map(|e| e.0.as_str()).collect();
    let statements: Vec<Value> = sents
        .iter()
        .filter(|s| !s.frames.is_empty())
        .map(|s| {
            let es: BTreeSet<&str> = s.words.iter().flat_map(|w| [w.0.as_str(), w.1.as_str()]).filter(|x| names.contains(x)).collect();
            json!({"s": s.s, "kind": kind(s), "part": s.part, "text": s.text, "frames": s.frames, "entities": es})
        })
        .collect();
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for st in &statements {
        *kinds.entry(st["kind"].as_str().unwrap()).or_default() += 1;
    }
    let v = json!({"schema": SCHEMA, "paper": paper, "reader": corpus::READER, "sentences": sents.len(), "kinds": kinds, "entities": entities, "statements": statements});
    corpus::write_private(&c.dir.join("world.json"), &serde_json::to_vec_pretty(&v)?)?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::tests::{built, model};

    #[test]
    fn the_paper_world_links_entities_to_the_corpus() {
        if !model() {
            return;
        }
        let (d, mut c) = built("world");
        let v = build(&mut c, 20).unwrap();
        let gec = v["entities"].as_array().unwrap().iter().find(|e| e["entity"] == "GEC").expect("GEC is an entity");
        assert_eq!(gec["explained_by"]["doc"], "lib:ra-2015-gec-evaluation");
        assert!(gec["linked"].as_array().unwrap().iter().any(|l| l["doc"] == "lib:ra-2015-gec-evaluation"));
        assert!(v["statements"].as_array().unwrap().iter().all(|s| s["kind"].is_string()));
        let again = build(&mut c, 20).unwrap();
        assert_eq!(v, again);
        let _ = std::fs::remove_dir_all(d);
    }
}
