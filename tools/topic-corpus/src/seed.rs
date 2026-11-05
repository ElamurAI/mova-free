//! The Whale hook: before a Whale asks its consultant about a term, the closed topic corpus is read
//! natively. For the terms a Whale would ask about (`mova_whale::reader::gaps`: unknown words met in
//! at least two sentences, acronyms first) and any terms named explicitly, the best explaining
//! sentences of the neighbour papers (`corpus::explain`, library texts only — never a web page or a
//! draft) are read by the Whale's own answer reader (`mova_whale::knowledge::term_facts`), and the
//! typed facts are retained in the Whale's knowledge store with provenance `topic-corpus` (corpus,
//! document, sentence). The Whale's writer then explains those terms from its knowledge, and its
//! question selection skips them (it asks only about terms it has no facts for). No Whale code
//! changes: this writes the store between rounds, under the Whale's `learner.lock`.
use crate::corpus::{self, Corpus};
use anyhow::{anyhow, bail, Result};
use mova_whale::knowledge::{self, Item, Store};
use serde_json::{json, Value};
use std::path::Path;

/// The least explanation score of a seeding sentence (`corpus::explain`: a defining pattern adds 2
/// or 3 to the base 1).
pub const MIN_SCORE: f64 = 3.0;
/// The fact relations a seed keeps: what a term is, stands for or means.
pub const DEFINING: [&str; 3] = ["stands_for", "is_a", "means"];

pub struct SeedArgs<'a> {
    pub whale: &'a str,
    pub state_root: &'a Path,
    /// The paper's id (the Whale's `paper.id`) and its Markdown text.
    pub paper: &'a str,
    pub paper_text: &'a str,
    /// At most this many gap terms (besides `terms`).
    pub max_terms: usize,
    pub terms: &'a [String],
}

pub fn seed(c: &mut Corpus, a: &SeedArgs) -> Result<Value> {
    let lock = a.state_root.join("learner.lock");
    if !a.state_root.is_absolute() {
        bail!("state_root must be absolute");
    }
    mova_whale::util::private_dir(a.state_root).map_err(|e| anyhow!("{e}"))?;
    let Some(_lease) = mova_whale::util::try_lease(&lock, true).map_err(|e| anyhow!("{e}"))? else {
        bail!("{} is learning now ({} held): seed between rounds", a.whale, lock.display());
    };
    let mut store = Store::open(&a.state_root.join("knowledge")).map_err(|e| anyhow!("{e}"))?;
    let reading = mova_whale::reader::read(a.paper_text).map_err(|e| anyhow!("{e}"))?;
    let known = store.known_terms();
    let (gaps, _) = mova_whale::reader::gaps(&reading, usize::MAX, 0);
    let mut terms: Vec<String> = a.terms.to_vec();
    let mut n = 0;
    for (t, _) in gaps {
        if n >= a.max_terms {
            break;
        }
        if !terms.contains(&t) && !known.contains(&t) && !reading.definitions.contains_key(&t) {
            terms.push(t);
            n += 1;
        }
    }
    let ann = world::tree::annotator()?;
    let paper_doc = format!("lib:{}", a.paper);
    let paper_words: std::collections::BTreeSet<String> = crate::library::words(a.paper_text).into_iter().collect();
    let mut seeded = vec![];
    let mut without = vec![];
    for t in terms {
        // The known set holds normalized terms (`knowledge::norm_term`); an explicit term («GEC») is compared normalized.
        if known.contains(&knowledge::norm_term(&t)) || reading.definitions.contains_key(&t) {
            continue;
        }
        let mut done = false;
        for e in corpus::explain(c, &t, Some(&paper_doc), &["paper"], 5)? {
            // Only defining sentences (an expansion, «T is/means …», «… (T)»): a sentence that merely
            // mentions the term gives facts about something else too often.
            if e.score < MIN_SCORE {
                continue;
            }
            let close = c.doc(&e.doc).is_some_and(|d| d.roles.iter().any(|r| r == "reference" || r == "similar"));
            // An expansion from a farther document must use only words of the paper (an acronym
            // names different things in different fields: «DET» is a determiner here, a test there).
            let facts: Vec<_> = knowledge::term_facts(ann, &t, &e.text)
                .into_iter()
                .filter(|f| DEFINING.contains(&f.relation.as_str()))
                .filter(|f| close || f.relation != "stands_for" || crate::library::words(&f.object).iter().all(|w| paper_words.contains(w)))
                .collect();
            if facts.is_empty() {
                continue;
            }
            let item = Item {
                id: store.next_id(),
                whale: a.whale.into(),
                kind: "term".into(),
                key: t.clone(),
                paper: a.paper.into(),
                facts: facts.clone(),
                provenance: json!({"source": "topic-corpus", "corpus": c.key, "doc": e.doc, "sentence": e.s, "public_admissible": e.public_admissible, "reader": corpus::READER}),
                unix: mova_whale::util::now(),
            };
            store.retain(item.clone()).map_err(|e| anyhow!("{e}"))?;
            seeded.push(json!({"term": t, "item": item.id, "doc": e.doc, "sentence": e.s, "facts": facts}));
            done = true;
            break;
        }
        if !done {
            without.push(t);
        }
    }
    Ok(json!({"whale": a.whale, "paper": a.paper, "seeded": seeded, "no_facts": without}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::tests::{built, model};
    use std::os::unix::fs::DirBuilderExt;

    #[test]
    fn seeds_terms_from_neighbour_papers_and_respects_the_learner_lock() {
        if !model() {
            return;
        }
        let (d, mut c) = built("seed");
        let state = d.join("state/owl-1");
        std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&state).unwrap();
        let text = std::fs::read_to_string(d.join("texts/p/pa/pa-2017-error-types/text.md")).unwrap();
        let args = SeedArgs { whale: "owl-1", state_root: &state, paper: "pa-2017-error-types", paper_text: &text, max_terms: 4, terms: &["GEC".to_string()] };
        // Red control: while the Whale learns, nothing is written.
        let held = mova_whale::util::try_lease(&state.join("learner.lock"), true).unwrap().unwrap();
        assert!(seed(&mut c, &args).unwrap_err().to_string().contains("learning now"));
        drop(held);
        let r = seed(&mut c, &args).unwrap();
        let s = &r["seeded"][0];
        assert_eq!(s["term"], "GEC");
        assert_eq!(s["doc"], "lib:ra-2015-gec-evaluation");
        let store = Store::open(&state.join("knowledge")).unwrap();
        let it = store.items.iter().find(|i| i.key == "GEC").unwrap();
        assert_eq!(it.provenance["source"], "topic-corpus");
        assert!(it.facts.iter().any(|f| f.relation == "stands_for" && f.object == "Grammatical Error Correction"));
        assert!(store.known_terms().contains(&knowledge::norm_term("GEC")));
        // A second seed adds nothing for a term the Whale knows.
        let r2 = seed(&mut c, &args).unwrap();
        assert!(r2["seeded"].as_array().unwrap().iter().all(|x| x["term"] != "GEC"));
        let _ = std::fs::remove_dir_all(d);
    }
}
