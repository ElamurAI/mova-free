//! Stage 5: citations in descriptions. Every sentence a Whale writes carries its basis in the
//! round's `draft.json` (the paper's sentences it restates, the knowledge items it uses); every
//! knowledge item carries its provenance (the paper itself, a document of the closed topic corpus,
//! or the Whale's consultation). This module turns that into a cited description: each sentence ends
//! with its source numbers, and a bibliography with stable ids (`lib:<library id>`, `web:<hash>`,
//! `sol:<draft>`, `consult:<whale>`) closes it. In public mode a description may cite only sources
//! admitted for public use: the paper and neighbour papers under open licences; a sentence based on
//! a page Sea Spider fetched, a consultant's draft or a closed paper refuses the whole output.
use crate::corpus::{self, Corpus, Doc};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
struct DraftItem {
    text: String,
    kind: String,
    #[serde(default)]
    sources: Vec<usize>,
    #[serde(default)]
    knowledge: Vec<String>,
    #[serde(default)]
    section: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Source {
    pub n: usize,
    pub id: String,
    pub entry: String,
    pub public_admissible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cited {
    pub markdown: String,
    pub sources: Vec<Source>,
    /// Per written sentence: its text and the source numbers.
    pub sentences: Vec<Value>,
}

/// A bibliography entry from a corpus document.
pub fn entry(d: &Doc) -> String {
    let c = &d.cite;
    match d.kind.as_str() {
        "paper" => {
            let authors: Vec<&str> = c["authors"].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect()).unwrap_or_default();
            let who = match authors.len() {
                0 => String::new(),
                1 | 2 => authors.join(" and "),
                _ => format!("{} et al.", authors[0]),
            };
            let year = c["year"].as_i64().map(|y| format!(" ({y})")).unwrap_or_default();
            let link = if let Some(x) = c["doi"].as_str() {
                format!("https://doi.org/{x}")
            } else if let Some(x) = c["acl"].as_str() {
                format!("https://aclanthology.org/{x}/")
            } else if let Some(x) = c["arxiv"].as_str() {
                format!("https://arxiv.org/abs/{x}")
            } else {
                String::new()
            };
            let mut s = format!("{who}{year}. {}.", d.title.trim_end_matches('.'));
            if !link.is_empty() {
                s.push_str(&format!(" {link}."));
            }
            if !d.licence.is_empty() {
                s.push_str(&format!(" Licence: {}.", d.licence));
            }
            s.trim_start_matches(". ").trim().to_string()
        }
        "web" => format!("{}. {} (page; licence: {}).", d.title, c["url"].as_str().unwrap_or(&d.source), if d.licence.is_empty() { "not stated" } else { &d.licence }),
        "author" => format!("Note on {} (prepared from library metadata).", d.title),
        _ => format!("{} (internal draft article, low quality).", d.title),
    }
}

/// Cite a Whale round. `round` is the round directory (`draft.json`), `state_root` the Whale's state
/// (`knowledge/items.jsonl`), `c` the paper's closed topic corpus.
pub fn cite(c: &Corpus, paper: &str, round: &Path, state_root: &Path, whale: &str, public: bool) -> Result<Cited> {
    let draft: Value = serde_json::from_slice(&std::fs::read(round.join("draft.json")).with_context(|| format!("{}/draft.json", round.display()))?)?;
    let items: Vec<DraftItem> = serde_json::from_value(draft["items"].clone())?;
    let mut prov: BTreeMap<String, Value> = BTreeMap::new();
    let kp = state_root.join("knowledge/items.jsonl");
    if kp.exists() {
        for l in std::fs::read_to_string(&kp)?.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(l)?;
            if let Some(id) = v["id"].as_str() {
                prov.insert(id.to_string(), v["provenance"].clone());
            }
        }
    }
    let paper_doc = format!("lib:{paper}");
    let mut order: Vec<String> = vec![];
    let mut sources: BTreeMap<String, Source> = BTreeMap::new();
    let mut refused: Vec<String> = vec![];
    let mut reg = |id: &str, sentence: &str| -> Result<usize> {
        if let Some(s) = sources.get(id) {
            return Ok(s.n);
        }
        let (entry, admissible) = if let Some(w) = id.strip_prefix("consult:") {
            (format!("Notes from the consultation of the describer «{w}» (not a published source; terms only)."), true)
        } else {
            let d = c.doc(id).ok_or_else(|| anyhow!("source {id} is not in the corpus {}", c.dir.display()))?;
            (entry(d), corpus::assert_public(d).is_ok())
        };
        if public && !admissible {
            refused.push(format!("{id}: «{sentence}»"));
        }
        let n = order.len() + 1;
        order.push(id.to_string());
        sources.insert(id.to_string(), Source { n, id: id.to_string(), entry, public_admissible: admissible });
        Ok(n)
    };
    let mut paragraphs: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let mut per = vec![];
    for it in &items {
        let mut ids: Vec<String> = vec![];
        if matches!(it.kind.as_str(), "source" | "reading" | "structure") || !it.sources.is_empty() {
            ids.push(paper_doc.clone());
        }
        for k in &it.knowledge {
            let p = prov.get(k).cloned().unwrap_or(Value::Null);
            let id = match p["source"].as_str() {
                Some("topic-corpus") => p["doc"].as_str().map(str::to_string),
                Some("terminal") | Some("reread") => Some(format!("consult:{whale}")),
                // The paper's own definitions («p-…» items) and unknown items: the paper.
                _ => Some(paper_doc.clone()),
            };
            if let Some(id) = id {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        if ids.is_empty() {
            ids.push(paper_doc.clone());
        }
        let mut ns: Vec<usize> = ids.iter().map(|id| reg(id, &it.text)).collect::<Result<_>>()?;
        ns.sort_unstable();
        ns.dedup();
        let marks = ns.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
        let t = it.text.trim();
        let (body, end) = match t.char_indices().last() {
            Some((i, c)) if matches!(c, '.' | '?' | '!') => (&t[..i], c.to_string()),
            _ => (t, ".".to_string()),
        };
        let cited = format!("{body} [{marks}]{end}");
        paragraphs.entry(it.section.map(|s| s + 1).unwrap_or(0)).or_default().push(cited.clone());
        per.push(json!({"text": it.text, "kind": it.kind, "sources": ns, "cited": cited}));
    }
    if !refused.is_empty() {
        bail!("public description refused: {} sentence source(s) are not admitted for public use (pages fetched by Sea Spider, drafts and closed papers stay internal): {}", refused.len(), refused.join("; "));
    }
    let mut md = String::new();
    for (_, ps) in paragraphs {
        md.push_str(&ps.join(" "));
        md.push_str("\n\n");
    }
    md.push_str("## Bibliography\n\n");
    let list: Vec<Source> = order.iter().map(|id| sources[id].clone()).collect();
    for s in &list {
        md.push_str(&format!("[{}] {} `{}`\n", s.n, s.entry, s.id));
    }
    Ok(Cited { markdown: md, sources: list, sentences: per })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::tests::{built, model, web_doc};

    fn write_round(dir: &Path, items: Value) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join("draft.json"), json!({"items": items}).to_string()).unwrap();
    }

    #[test]
    fn every_sentence_cites_and_public_refuses_web_sources() {
        if !model() {
            return;
        }
        let (d, mut c) = built("cite");
        c.register(web_doc("web:abc", "CC BY 4.0"), "# A blog\n\n## Post\n\nThe M2 scorer is a metric for grammatical error correction systems.\n").unwrap();
        let state = d.join("state");
        std::fs::create_dir_all(state.join("knowledge")).unwrap();
        let k = [
            json!({"id":"k00001","whale":"owl-1","kind":"term","key":"GEC","paper":"pa-2017-error-types","facts":[],"provenance":{"source":"topic-corpus","doc":"lib:ra-2015-gec-evaluation","sentence":0},"unix":0}),
            json!({"id":"k00002","whale":"owl-1","kind":"term","key":"M2","paper":"pa-2017-error-types","facts":[],"provenance":{"source":"terminal","question":"q1"},"unix":0}),
            json!({"id":"k00003","whale":"owl-1","kind":"term","key":"blog","paper":"pa-2017-error-types","facts":[],"provenance":{"source":"topic-corpus","doc":"web:abc","sentence":0},"unix":0}),
        ];
        std::fs::write(state.join("knowledge/items.jsonl"), k.iter().map(|v| v.to_string() + "\n").collect::<String>()).unwrap();
        let round = d.join("round-001");
        write_round(&round, json!([
            {"text":"The paper presents ERRANT.","kind":"source","sources":[3],"knowledge":[],"section":0},
            {"text":"«GEC» is short for Grammatical Error Correction.","kind":"explain","sources":[],"knowledge":["k00001"],"section":0},
            {"text":"I understand «M2» as a scorer.","kind":"explain","sources":[],"knowledge":["k00002"],"section":1}
        ]));
        let r = cite(&c, "pa-2017-error-types", &round, &state, "owl-1", true).unwrap();
        assert!(r.markdown.starts_with("The paper presents ERRANT [1]. «GEC» is short for Grammatical Error Correction [2]."));
        assert!(r.markdown.contains("I understand «M2» as a scorer [3]."));
        assert!(r.markdown.contains("[2] Ray A (2015). Evaluation of grammatical error correction. https://aclanthology.org/P15-1068/. Licence: CC BY 4.0 (ACL Anthology). `lib:ra-2015-gec-evaluation`"));
        assert_eq!(r.sources.len(), 3);
        assert!(r.sentences.iter().all(|s| !s["sources"].as_array().unwrap().is_empty()));
        // A sentence based on a page Sea Spider fetched: internal output is cited, public refused.
        write_round(&round, json!([{"text":"A blog explains the scorer.","kind":"explain","sources":[],"knowledge":["k00003"],"section":0}]));
        let internal = cite(&c, "pa-2017-error-types", &round, &state, "owl-1", false).unwrap();
        assert!(internal.markdown.contains("`web:abc`"));
        assert!(!internal.sources[0].public_admissible);
        let e = cite(&c, "pa-2017-error-types", &round, &state, "owl-1", true).unwrap_err().to_string();
        assert!(e.contains("web:abc"), "{e}");
        // A closed neighbour paper is refused publicly too.
        let k4 = json!({"id":"k00004","whale":"owl-1","kind":"term","key":"MaxMatch","paper":"pa-2017-error-types","facts":[],"provenance":{"source":"topic-corpus","doc":"lib:rb-2012-m2-scorer","sentence":0},"unix":0});
        std::fs::write(state.join("knowledge/items.jsonl"), k4.to_string() + "\n").unwrap();
        write_round(&round, json!([{"text":"MaxMatch computes edits.","kind":"explain","knowledge":["k00004"]}]));
        assert!(cite(&c, "pa-2017-error-types", &round, &state, "owl-1", true).is_err());
        assert!(cite(&c, "pa-2017-error-types", &round, &state, "owl-1", false).is_ok());
        let _ = std::fs::remove_dir_all(d);
    }
}
