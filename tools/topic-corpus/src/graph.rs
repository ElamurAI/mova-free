//! Stage 1: the topic graph of one paper — its topics and concepts, authors, references (inside the
//! library or external with their ids), the library works citing it, similar works by keywords and
//! topics, and other works of its authors. Every node carries its ids, licence class, whether it may
//! feed a public output, and which text we hold. Deterministic: the same library gives byte-identical
//! output.
use crate::config::Limits;
use crate::library::{Author, Library, RefEntry, Topic, Work};
use crate::licence::{self, Class, Origin};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SCHEMA: &str = "mova-topic-graph-v1";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: String,
    pub title: String,
    pub year: Option<i64>,
    pub authors: Vec<String>,
    pub doi: Option<String>,
    pub arxiv: Option<String>,
    pub acl: Option<String>,
    pub openalex: Option<String>,
    pub licence: String,
    pub licence_class: Class,
    pub public_admissible: bool,
    /// `open` (text.md, not local-only), `local` (text.local.md, eprint.md or local-only), `none`.
    pub text: String,
    pub text_path: Option<String>,
    pub cited_by: i64,
    /// Similarity score (similar works only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Reference {
    /// Position in the paper's extracted bibliography (1-based); 0 for a library link without an entry.
    pub n: usize,
    pub entry: RefEntry,
    /// The library work it names, when we have it.
    pub work: Option<Node>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicRef {
    pub id: String,
    pub name: String,
    pub subfield: String,
    pub field: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorRef {
    pub position: usize,
    pub name: String,
    pub openalex: Option<String>,
    pub orcid: Option<String>,
    pub homepage: Option<String>,
    pub h_index: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neighbourhood {
    pub schema: String,
    pub paper: Node,
    pub topics: Vec<TopicRef>,
    pub concepts: Vec<String>,
    pub tags: Vec<String>,
    pub authors: Vec<AuthorRef>,
    pub references: Vec<Reference>,
    pub cited_by: Vec<Node>,
    pub cited_by_total: usize,
    pub similar: Vec<Node>,
    pub same_author: Vec<Node>,
}

impl Neighbourhood {
    /// Every library work of the neighbourhood with its roles, the paper first, then by role order
    /// (reference, similar, cited_by, same_author) and position; each id once.
    pub fn members(&self) -> Vec<(Node, Vec<&'static str>)> {
        let mut order: Vec<String> = vec![];
        let mut roles: BTreeMap<String, (Node, Vec<&'static str>)> = BTreeMap::new();
        let mut add = |n: &Node, r: &'static str| {
            let e = roles.entry(n.id.clone()).or_insert_with(|| {
                order.push(n.id.clone());
                (n.clone(), vec![])
            });
            if !e.1.contains(&r) {
                e.1.push(r);
            }
        };
        add(&self.paper, "paper");
        for r in &self.references {
            if let Some(w) = &r.work {
                add(w, "reference");
            }
        }
        for n in &self.similar {
            add(n, "similar");
        }
        for n in &self.cited_by {
            add(n, "cited_by");
        }
        for n in &self.same_author {
            add(n, "same_author");
        }
        order.into_iter().map(|id| roles.remove(&id).unwrap()).collect()
    }
}

pub fn node(lib: &Library, w: &Work) -> Node {
    let class = Library::class(w);
    let (text, path) = match lib.text(w) {
        Some((p, true)) => ("local", Some(p.display().to_string())),
        Some((p, false)) => ("open", Some(p.display().to_string())),
        None => ("none", None),
    };
    Node {
        id: w.id.clone(),
        title: w.title.clone(),
        year: w.year,
        authors: w.authors.iter().take(6).cloned().collect(),
        doi: w.doi.clone(),
        arxiv: w.arxiv.clone(),
        acl: w.acl.clone(),
        openalex: w.openalex.clone(),
        licence: w.licence.clone(),
        licence_class: class,
        public_admissible: licence::public_admissible(Origin::Library, class, w.text_local_only || text == "local"),
        text: text.into(),
        text_path: path,
        cited_by: w.cited_by,
        score: None,
    }
}

fn round4(x: f64) -> f64 {
    (x * 10_000.0).round() / 10_000.0
}

/// Similar works: an idf-weighted cosine of title+abstract keywords, plus shared OpenAlex topics
/// (half a point for all of the paper's topics) and a shared concept (0.2). A work needs a keyword
/// score of at least 0.05 or a shared topic.
pub fn similar(lib: &Library, p: &Work, k: usize) -> Vec<(f64, usize)> {
    let n = lib.works.len().max(1) as f64;
    let idf = |t: u32| (n / (1.0 + lib.df.get(t as usize).copied().unwrap_or(0) as f64)).ln().max(0.0);
    let norm = |kw: &[u32]| kw.iter().map(|&t| idf(t).powi(2)).sum::<f64>().sqrt();
    let pn = norm(&p.keywords);
    let ptop: BTreeSet<&String> = p.oa_topics.iter().collect();
    let pcon: BTreeSet<&String> = p.concepts.iter().collect();
    let mut out: Vec<(f64, usize)> = vec![];
    for (i, w) in lib.works.iter().enumerate() {
        if w.id == p.id {
            continue;
        }
        let shared: f64 = w.keywords.iter().filter(|t| p.keywords.binary_search(t).is_ok()).map(|&t| idf(t).powi(2)).sum();
        let cos = if pn > 0.0 && shared > 0.0 { shared / (pn * norm(&w.keywords)) } else { 0.0 };
        let st = w.oa_topics.iter().filter(|t| ptop.contains(t)).count();
        if cos < 0.05 && st == 0 {
            continue;
        }
        let topic = if ptop.is_empty() { 0.0 } else { 0.5 * st as f64 / ptop.len() as f64 };
        let con = if w.concepts.iter().any(|c| pcon.contains(c)) { 0.2 } else { 0.0 };
        out.push((round4(cos + topic + con), i));
    }
    out.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| lib.works[a.1].id.cmp(&lib.works[b.1].id)));
    out.truncate(k);
    out
}

pub fn build(lib: &Library, paper: &str, limits: &Limits) -> Result<Neighbourhood> {
    let Some(p) = lib.find(paper) else { bail!("{paper} is not in the library export") };
    let one: BTreeSet<String> = [p.id.clone()].into();
    // References: the extracted bibliography, resolved against the library; library links of the
    // nightly resolver that no entry matched are added after it.
    let bib = lib.bibliography(p)?;
    let mut references: Vec<Reference> = vec![];
    let mut linked: BTreeSet<String> = BTreeSet::new();
    for (i, e) in bib.into_iter().enumerate() {
        let work = lib.resolve(&e).filter(|w| w.id != p.id).map(|w| node(lib, w));
        if let Some(w) = &work {
            linked.insert(w.id.clone());
        }
        references.push(Reference { n: i + 1, entry: e, work });
    }
    for id in lib.resolved_refs(&one)?.remove(&p.id).unwrap_or_default() {
        if id != p.id && linked.insert(id.clone()) {
            let w = lib.get(&id).unwrap();
            references.push(Reference { n: 0, entry: RefEntry { title: Some(w.title.clone()), year: w.year, ..Default::default() }, work: Some(node(lib, w)) });
        }
    }
    let citers = lib.citers(&one)?.remove(&p.id).unwrap_or_default();
    let mut cited: Vec<&Work> = citers.iter().filter_map(|id| lib.get(id)).collect();
    cited.sort_by(|a, b| b.cited_by.cmp(&a.cited_by).then_with(|| a.id.cmp(&b.id)));
    let cited_by_total = cited.len();
    let cited_by = cited.into_iter().take(limits.cited_by).map(|w| node(lib, w)).collect();
    let similar = similar(lib, p, limits.similar)
        .into_iter()
        .map(|(s, i)| {
            let mut n = node(lib, &lib.works[i]);
            n.score = Some(s);
            n
        })
        .collect();
    let ids: BTreeSet<&String> = p.author_ids.iter().filter(|a| !a.is_empty()).collect();
    let mut same: Vec<&Work> = lib.works.iter().filter(|w| w.id != p.id && w.author_ids.iter().any(|a| ids.contains(a))).collect();
    same.sort_by(|a, b| b.cited_by.cmp(&a.cited_by).then_with(|| a.id.cmp(&b.id)));
    let same_author = same.into_iter().take(limits.same_author).map(|w| node(lib, w)).collect();
    let topics = p
        .oa_topics
        .iter()
        .map(|t| match lib.topics.get(t) {
            Some(Topic { id, name, subfield, field }) => TopicRef { id: id.clone(), name: name.clone(), subfield: subfield.clone(), field: field.clone() },
            None => TopicRef { id: t.clone(), name: String::new(), subfield: String::new(), field: String::new() },
        })
        .collect();
    let authors = lib
        .authors(&one)?
        .remove(&p.id)
        .unwrap_or_default()
        .into_iter()
        .map(|Author { position, name, openalex, orcid, homepage, h_index }| AuthorRef { position, name, openalex, orcid, homepage, h_index })
        .collect::<Vec<_>>();
    let authors = if authors.is_empty() { p.authors.iter().enumerate().map(|(i, n)| AuthorRef { position: i + 1, name: n.clone(), openalex: p.author_ids.get(i).cloned(), orcid: None, homepage: None, h_index: None }).collect() } else { authors };
    Ok(Neighbourhood { schema: SCHEMA.into(), paper: node(lib, p), topics, concepts: p.concepts.clone(), tags: p.tags.clone(), authors, references, cited_by, cited_by_total, similar, same_author })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::tests::{fixture, tmp};

    #[test]
    fn neighbourhood_of_the_fixture_paper() {
        let d = tmp("graph");
        let lib = Library::load(&fixture(&d)).unwrap();
        let g = build(&lib, "acl:P17-1074", &Limits::default()).unwrap();
        assert_eq!(g.paper.id, "pa-2017-error-types");
        assert_eq!(g.paper.text, "open");
        assert!(g.paper.public_admissible);
        assert_eq!(g.topics[0].name, "Grammatical error correction");
        assert_eq!(g.authors[0].homepage.as_deref(), Some("https://ann.example.org/"));
        assert_eq!(g.references.len(), 5);
        let r2 = g.references[1].work.as_ref().unwrap();
        assert_eq!((r2.id.as_str(), r2.text.as_str(), r2.public_admissible), ("rb-2012-m2-scorer", "local", false));
        assert!(g.references[2].work.is_none() && g.references[2].entry.doi.is_some());
        assert_eq!(g.cited_by_total, 2);
        // The most cited citer first.
        assert_eq!(g.cited_by[0].id, "cb-2021-error-types-german");
        assert_eq!(g.similar[0].id, "sa-2019-annotation-error-types");
        assert!(g.similar.iter().all(|n| n.id != "oz-2010-protein-folding"));
        assert_eq!(g.same_author.iter().map(|n| n.id.as_str()).collect::<Vec<_>>(), vec!["cb-2021-error-types-german"]);
        let m = g.members();
        assert_eq!(m[0].1, vec!["paper"]);
        assert!(m.iter().any(|(n, r)| n.id == "cb-2021-error-types-german" && r.contains(&"cited_by") && r.contains(&"same_author")));
        // Deterministic.
        let again = build(&lib, "pa-2017-error-types", &Limits::default()).unwrap();
        assert_eq!(serde_json::to_string(&g).unwrap(), serde_json::to_string(&again).unwrap());
        assert!(build(&lib, "nobody-1999-nothing", &Limits::default()).is_err());
        let _ = std::fs::remove_dir_all(d);
    }
}
