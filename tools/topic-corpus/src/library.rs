//! The paper library as the topic graph reads it: the export (one work per line), the citation
//! edges inside the library, the resolved reference lists, the per-paper reference caches
//! (`refs.json`), the OpenAlex topic taxonomy and the author positions. Read-only; nothing here
//! makes a network request.
use crate::config;
use crate::licence::{self, Class};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// One library work (the export fields the graph needs).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Work {
    pub id: String,
    pub rel: String,
    pub title: String,
    pub year: Option<i64>,
    pub authors: Vec<String>,
    pub author_ids: Vec<String>,
    pub doi: Option<String>,
    pub arxiv: Option<String>,
    pub acl: Option<String>,
    pub openalex: Option<String>,
    pub licence: String,
    pub licence_url: Option<String>,
    pub text_path: Option<String>,
    pub text_local_only: bool,
    pub oa_topics: Vec<String>,
    pub concepts: Vec<String>,
    pub tags: Vec<String>,
    pub cited_by: i64,
    /// Keyword ids of title and abstract (`Library::words`), sorted, unique.
    #[serde(skip)]
    pub keywords: Vec<u32>,
}

#[derive(Deserialize)]
struct ExportRow {
    id: String,
    #[serde(default)]
    rel: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    year: Option<i64>,
    #[serde(default)]
    authors: Option<Vec<String>>,
    #[serde(default)]
    author_ids: Option<Vec<String>>,
    #[serde(default)]
    doi: Option<String>,
    #[serde(default)]
    arxiv: Option<String>,
    #[serde(default)]
    acl: Option<String>,
    #[serde(default)]
    openalex: Option<String>,
    #[serde(default)]
    licence: Option<String>,
    #[serde(default)]
    licence_url: Option<String>,
    #[serde(default)]
    text_path: Option<String>,
    #[serde(default)]
    text_local_only: Option<bool>,
    #[serde(default)]
    oa_topics: Option<Vec<String>>,
    #[serde(default)]
    concepts: Option<Vec<String>>,
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    cited_by: Option<i64>,
    #[serde(default, rename = "abstract")]
    abstract_: Option<String>,
}

/// Words left out of keyword sets.
const STOP: [&str; 48] = [
    "the", "and", "for", "that", "this", "with", "from", "are", "was", "were", "which", "have", "has", "been", "their", "they", "these", "those", "its", "our", "can", "not", "but", "also",
    "such", "than", "then", "there", "into", "each", "use", "used", "using", "one", "two", "may", "more", "most", "other", "all", "paper", "we", "show", "new", "based", "approach", "how", "what",
];

/// Lowercase alphanumeric words of at least three characters, stop words left out.
pub fn words(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| w.chars().count() >= 3).map(str::to_lowercase).filter(|w| !STOP.contains(&w.as_str())).collect()
}

/// The normalized title key: lowercase alphanumeric words joined by single spaces.
pub fn title_key(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect::<Vec<_>>().join(" ")
}

pub fn bare_doi(s: &str) -> String {
    let l = s.trim().to_lowercase();
    l.trim_start_matches("https://doi.org/").trim_start_matches("http://doi.org/").trim_start_matches("http://dx.doi.org/").trim_start_matches("doi:").trim_end_matches(['.', ',', ';']).to_string()
}

/// The ACL Anthology id a DOI names (`10.18653/v1/P17-1074` → `P17-1074`).
pub fn acl_of_doi(doi: &str) -> Option<String> {
    let d = bare_doi(doi);
    let rest = d.strip_prefix("10.18653/v1/")?;
    let mut id = rest.to_string();
    // The Anthology writes letters of old ids in upper case («P17-1074», «2020.acl-main.12»).
    if id.len() == 8 && id.as_bytes()[3] == b'-' {
        id = id.to_uppercase();
    }
    Some(id)
}

/// A topic of the OpenAlex taxonomy.
#[derive(Clone, Debug, Serialize)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub subfield: String,
    pub field: String,
}

/// One author position of a work.
#[derive(Clone, Debug, Serialize)]
pub struct Author {
    pub position: usize,
    pub name: String,
    pub openalex: Option<String>,
    pub orcid: Option<String>,
    pub homepage: Option<String>,
    pub h_index: Option<i64>,
}

/// One entry of a paper's extracted bibliography (`refs.json`).
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct RefEntry {
    #[serde(default)]
    pub raw: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub year: Option<i64>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub doi: Option<String>,
    #[serde(default)]
    pub arxiv: Option<String>,
    #[serde(default)]
    pub acl: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

pub struct Library {
    pub cfg: config::Library,
    pub works: Vec<Work>,
    pub by_id: BTreeMap<String, usize>,
    by_doi: HashMap<String, usize>,
    by_arxiv: HashMap<String, usize>,
    by_acl: HashMap<String, usize>,
    by_title: HashMap<String, Vec<usize>>,
    /// Keyword document frequencies (by keyword id).
    pub df: Vec<u32>,
    pub topics: BTreeMap<String, Topic>,
}

fn base(rel: &str) -> &str {
    rel.rsplit('/').next().unwrap_or(rel)
}

fn lines(p: &Path) -> Result<impl Iterator<Item = std::io::Result<String>>> {
    Ok(BufReader::with_capacity(1 << 20, std::fs::File::open(p).with_context(|| format!("open {}", p.display()))?).lines())
}

impl Library {
    pub fn load(cfg: &config::Library) -> Result<Library> {
        let mut works = vec![];
        let mut vocab: HashMap<String, u32> = HashMap::new();
        let mut df: Vec<u32> = vec![];
        for l in lines(&cfg.export)? {
            let l = l?;
            if l.trim().is_empty() {
                continue;
            }
            let r: ExportRow = serde_json::from_str(&l).with_context(|| format!("export row in {}", cfg.export.display()))?;
            let title = r.title.unwrap_or_default();
            let mut kw: Vec<u32> = words(&format!("{title} {}", r.abstract_.unwrap_or_default()))
                .into_iter()
                .map(|w| {
                    let n = vocab.len() as u32;
                    *vocab.entry(w).or_insert(n)
                })
                .collect();
            kw.sort_unstable();
            kw.dedup();
            for &k in &kw {
                if k as usize >= df.len() {
                    df.resize(k as usize + 1, 0);
                }
                df[k as usize] += 1;
            }
            works.push(Work {
                rel: r.rel.unwrap_or_else(|| r.id.clone()),
                id: r.id,
                title,
                year: r.year,
                authors: r.authors.unwrap_or_default(),
                author_ids: r.author_ids.unwrap_or_default(),
                doi: r.doi.filter(|d| !d.is_empty()).map(|d| bare_doi(&d)),
                arxiv: r.arxiv.filter(|d| !d.is_empty()),
                acl: r.acl.filter(|d| !d.is_empty()),
                openalex: r.openalex.filter(|d| !d.is_empty()),
                licence: r.licence.unwrap_or_default(),
                licence_url: r.licence_url,
                text_path: r.text_path,
                text_local_only: r.text_local_only.unwrap_or(false),
                oa_topics: r.oa_topics.unwrap_or_default(),
                concepts: r.concepts.unwrap_or_default(),
                tags: r.tags.unwrap_or_default(),
                cited_by: r.cited_by.unwrap_or(0),
                keywords: kw,
            });
        }
        let mut lib = Library { cfg: cfg.clone(), works: vec![], by_id: BTreeMap::new(), by_doi: HashMap::new(), by_arxiv: HashMap::new(), by_acl: HashMap::new(), by_title: HashMap::new(), df, topics: BTreeMap::new() };
        for (i, w) in works.iter().enumerate() {
            // The first row of an id wins (the export has none twice; a repeat is ignored).
            lib.by_id.entry(w.id.clone()).or_insert(i);
            if let Some(d) = &w.doi {
                lib.by_doi.entry(d.clone()).or_insert(i);
                if let Some(a) = acl_of_doi(d) {
                    lib.by_acl.entry(a.to_lowercase()).or_insert(i);
                }
            }
            if let Some(a) = &w.arxiv {
                lib.by_arxiv.entry(a.to_lowercase()).or_insert(i);
            }
            if let Some(a) = &w.acl {
                lib.by_acl.entry(a.to_lowercase()).or_insert(i);
            }
            let tk = title_key(&w.title);
            if tk.split(' ').count() >= 3 {
                lib.by_title.entry(tk).or_default().push(i);
            }
        }
        lib.works = works;
        if let Some(t) = &cfg.topics {
            for l in lines(t)? {
                let l = l?;
                let c: Vec<&str> = l.split('\t').collect();
                if c.len() >= 6 && c[0].starts_with('T') && c[0] != "topic" {
                    lib.topics.insert(c[0].into(), Topic { id: c[0].into(), name: c[1].into(), subfield: c[3].into(), field: c[5].into() });
                }
            }
        }
        Ok(lib)
    }

    pub fn get(&self, id: &str) -> Option<&Work> {
        self.by_id.get(id).map(|&i| &self.works[i])
    }

    /// A work named by library id, DOI, arXiv id, ACL id or OpenAlex id.
    pub fn find(&self, key: &str) -> Option<&Work> {
        let k = key.trim();
        if let Some(w) = self.get(k) {
            return Some(w);
        }
        if let Some(d) = k.strip_prefix("doi:") {
            return self.by_doi.get(&bare_doi(d)).map(|&i| &self.works[i]);
        }
        if let Some(a) = k.strip_prefix("arxiv:") {
            return self.by_arxiv.get(&a.to_lowercase()).map(|&i| &self.works[i]);
        }
        if let Some(a) = k.strip_prefix("acl:") {
            return self.by_acl.get(&a.to_lowercase()).map(|&i| &self.works[i]);
        }
        if k.starts_with("10.") || k.contains("doi.org/") {
            return self.by_doi.get(&bare_doi(k)).map(|&i| &self.works[i]);
        }
        None
    }

    /// The library work a bibliography entry names: by DOI, arXiv id or ACL id, else by normalized
    /// title (at least three words) with the year within one.
    pub fn resolve(&self, r: &RefEntry) -> Option<&Work> {
        if let Some(i) = r.doi.as_deref().and_then(|d| self.by_doi.get(&bare_doi(d))) {
            return Some(&self.works[*i]);
        }
        if let Some(i) = r.arxiv.as_deref().and_then(|a| self.by_arxiv.get(&a.to_lowercase())) {
            return Some(&self.works[*i]);
        }
        if let Some(i) = r.acl.as_deref().and_then(|a| self.by_acl.get(&a.to_lowercase())) {
            return Some(&self.works[*i]);
        }
        let t = r.title.as_deref().map(title_key)?;
        let cands = self.by_title.get(&t)?;
        let mut best = cands.iter().filter(|&&i| match (r.year, self.works[i].year) {
            (Some(a), Some(b)) => (a - b).abs() <= 1,
            _ => true,
        });
        best.next().map(|&i| &self.works[i])
    }

    /// The directory of a work's texts.
    pub fn text_dir(&self, w: &Work) -> PathBuf {
        match &w.text_path {
            Some(p) if !p.is_empty() => Path::new(p).parent().map(Path::to_path_buf).unwrap_or_else(|| self.cfg.text_root.join(&w.rel)),
            _ => self.cfg.text_root.join(&w.rel),
        }
    }

    /// The text we hold for a work: `text.md` (open copy) unless the work is local-only, else
    /// `text.local.md`, else `eprint.md`. `(path, local_only)`.
    pub fn text(&self, w: &Work) -> Option<(PathBuf, bool)> {
        let d = self.text_dir(w);
        let open = d.join("text.md");
        if open.is_file() {
            return Some((open, w.text_local_only));
        }
        for f in ["text.local.md", "eprint.md"] {
            let p = d.join(f);
            if p.is_file() {
                return Some((p, true));
            }
        }
        None
    }

    /// The extracted bibliography of a work (`refs.json` beside its text), empty without one.
    pub fn bibliography(&self, w: &Work) -> Result<Vec<RefEntry>> {
        let p = self.text_dir(w).join("refs.json");
        if !p.is_file() {
            return Ok(vec![]);
        }
        let v: Value = serde_json::from_slice(&std::fs::read(&p)?).with_context(|| format!("{}", p.display()))?;
        Ok(v["refs"].as_array().map(|a| a.iter().filter_map(|x| serde_json::from_value(x.clone()).ok()).collect()).unwrap_or_default())
    }

    /// Library works the given works cite (`refs.jsonl` resolved lists), by citing id.
    pub fn resolved_refs(&self, citing: &BTreeSet<String>) -> Result<BTreeMap<String, Vec<String>>> {
        let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for l in lines(&self.cfg.refs)? {
            let l = l?;
            if l.trim().is_empty() {
                continue;
            }
            let v: Value = serde_json::from_str(&l)?;
            let p = base(v["paper"].as_str().unwrap_or("")).to_string();
            if citing.contains(&p) {
                let r: Vec<String> = v["resolved"].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).map(|x| base(x).to_string()).filter(|x| self.by_id.contains_key(x)).collect()).unwrap_or_default();
                out.entry(p).or_default().extend(r);
            }
        }
        Ok(out)
    }

    /// Library works citing each of the given works (`citations.tsv`), by cited id; sorted ids.
    pub fn citers(&self, cited: &BTreeSet<String>) -> Result<BTreeMap<String, BTreeSet<String>>> {
        let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for l in lines(&self.cfg.citations)? {
            let l = l?;
            let Some((from, to)) = l.split_once('\t') else { continue };
            let to = base(to.trim());
            if cited.contains(to) {
                let f = base(from.trim());
                if self.by_id.contains_key(f) {
                    out.entry(to.to_string()).or_default().insert(f.to_string());
                }
            }
        }
        Ok(out)
    }

    /// Author positions of the given works (`authors.tsv`), by work id.
    pub fn authors(&self, ids: &BTreeSet<String>) -> Result<BTreeMap<String, Vec<Author>>> {
        let mut out: BTreeMap<String, Vec<Author>> = BTreeMap::new();
        let Some(p) = &self.cfg.authors else { return Ok(out) };
        let mut head: Vec<String> = vec![];
        for l in lines(p)? {
            let l = l?;
            let c: Vec<&str> = l.split('\t').collect();
            if head.is_empty() {
                head = c.iter().map(|s| s.to_string()).collect();
                continue;
            }
            let col = |n: &str| head.iter().position(|h| h == n).and_then(|i| c.get(i)).map(|s| s.trim()).filter(|s| !s.is_empty()).map(str::to_string);
            let Some(f) = col("folder") else { continue };
            if !ids.contains(&f) {
                continue;
            }
            out.entry(f).or_default().push(Author {
                position: col("position").and_then(|x| x.parse().ok()).unwrap_or(0),
                name: col("name").unwrap_or_default(),
                openalex: col("openalex_author_id"),
                orcid: col("orcid"),
                homepage: col("homepage"),
                h_index: col("h_index").and_then(|x| x.parse().ok()),
            });
        }
        for v in out.values_mut() {
            v.sort_by_key(|a| a.position);
        }
        Ok(out)
    }

    pub fn class(w: &Work) -> Class {
        licence::classify(&w.licence)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    /// A small library under `dir`: P (the paper) cites R1 and R2 (resolved) and an external work
    /// with a DOI; C1 and C2 cite P; S shares P's topic and keywords; O is unrelated. Texts: P and R1
    /// open, R2 local-only, C1 none.
    pub fn fixture(dir: &Path) -> config::Library {
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir).unwrap();
        let tr = dir.join("texts");
        let rows = vec![
            json!({"id":"pa-2017-error-types","rel":"p/pa/pa-2017-error-types","title":"Automatic annotation of grammatical error types","year":2017,"authors":["Ann Pa","Bo Qu"],"author_ids":["A1","A2"],"doi":"10.18653/v1/P17-1074","acl":"P17-1074","licence":"CC BY 4.0 (ACL Anthology)","oa_topics":["T1","T2"],"concepts":["grammatical-error-correction"],"cited_by":300,"abstract":"We annotate grammatical error types automatically with a rule-based classifier.","text_path":tr.join("p/pa/pa-2017-error-types/text.md")}),
            json!({"id":"ra-2015-gec-evaluation","rel":"r/ra/ra-2015-gec-evaluation","title":"Evaluation of grammatical error correction","year":2015,"authors":["Ray A"],"acl":"P15-1068","licence":"CC BY 4.0 (ACL Anthology)","oa_topics":["T1"],"cited_by":120,"text_path":tr.join("r/ra/ra-2015-gec-evaluation/text.md")}),
            json!({"id":"rb-2012-m2-scorer","rel":"r/rb/rb-2012-m2-scorer","title":"Better evaluation for grammatical error correction","year":2012,"authors":["Rob B"],"licence":"CC BY-NC-SA 3.0 (ACL Anthology)","oa_topics":["T1"],"cited_by":500,"text_local_only":true,"text_path":tr.join("r/rb/rb-2012-m2-scorer/text.md")}),
            json!({"id":"ca-2020-error-types-arabic","rel":"c/ca/ca-2020-error-types-arabic","title":"Arabic grammatical error types","year":2020,"authors":["Cy A"],"licence":"CC BY 4.0","oa_topics":["T2"],"cited_by":10}),
            json!({"id":"cb-2021-error-types-german","rel":"c/cb/cb-2021-error-types-german","title":"German error type annotation","year":2021,"authors":["Ann Pa"],"author_ids":["A1"],"licence":"arXiv — non-exclusive license to distribute","oa_topics":["T3"],"cited_by":40}),
            json!({"id":"sa-2019-annotation-error-types","rel":"s/sa/sa-2019-annotation-error-types","title":"Annotation of grammatical error types for learner corpora","year":2019,"authors":["Sam A"],"licence":"CC BY 4.0","oa_topics":["T1","T2"],"cited_by":25,"abstract":"Grammatical error types annotated automatically in learner corpora."}),
            json!({"id":"oz-2010-protein-folding","rel":"o/oz/oz-2010-protein-folding","title":"Protein folding kinetics","year":2010,"authors":["Oz Z"],"licence":"CC BY 4.0","oa_topics":["T9"],"cited_by":900}),
        ];
        let export = dir.join("export.jsonl");
        std::fs::write(&export, rows.iter().map(|r| r.to_string() + "\n").collect::<String>()).unwrap();
        let w = |rel: &str, f: &str, s: &str| {
            let p = tr.join(rel).join(f);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, s).unwrap();
        };
        w("p/pa/pa-2017-error-types", "text.md", "# Automatic annotation of grammatical error types\n\n## Introduction\n\nWe present ERRANT, a toolkit that annotates parallel sentences with error types for GEC. The M2 scorer is the standard metric for GEC.\n\n## References\n\nRay A. 2015. Evaluation.\n");
        w("r/ra/ra-2015-gec-evaluation", "text.md", "# Evaluation of grammatical error correction\n\n## Introduction\n\nGrammatical Error Correction (GEC) is the task of correcting errors in text written by learners. The M2 scorer is a metric that compares edits with gold edits.\n");
        w("r/rb/rb-2012-m2-scorer", "text.local.md", "# Better evaluation\n\n## Method\n\nThe MaxMatch scorer computes the edits with the maximal overlap with the gold standard.\n");
        w(
            "p/pa/pa-2017-error-types",
            "refs.json",
            &json!({"source":"text","refs":[
                {"raw":"Ray A. 2015. Evaluation of grammatical error correction. ACL P15-1068.","acl":"P15-1068","year":2015,"title":"Evaluation of grammatical error correction","author":"A"},
                {"raw":"Rob B. 2012. Better evaluation for grammatical error correction.","year":2012,"title":"Better evaluation for grammatical error correction","author":"B"},
                {"raw":"Xu X. 2014. The CoNLL-2014 shared task. doi:10.3115/v1/W14-1701","doi":"10.3115/v1/W14-1701","year":2014,"title":"The CoNLL-2014 shared task on grammatical error correction","author":"Xu"},
                {"raw":"Yo Y. 2011. A new dataset. arXiv:1101.0001","arxiv":"1101.0001","year":2011,"title":"A new dataset and method for automatically grading ESOL texts","author":"Yo"},
                {"raw":"Zed Z. 2019. Neural models of learner text errors.","year":2019,"title":"Neural models of learner text errors","author":"Zed"}
            ]})
            .to_string(),
        );
        let refs = dir.join("refs.jsonl");
        std::fs::write(&refs, json!({"paper":"p/pa/pa-2017-error-types","resolved":["r/ra/ra-2015-gec-evaluation","old/layout/rb-2012-m2-scorer"]}).to_string() + "\n").unwrap();
        let cit = dir.join("citations.tsv");
        std::fs::write(&cit, "from\tto\nc/ca/ca-2020-error-types-arabic\tp/pa/pa-2017-error-types\nc/cb/cb-2021-error-types-german\tp/pa/pa-2017-error-types\np/pa/pa-2017-error-types\tr/ra/ra-2015-gec-evaluation\n").unwrap();
        let topics = dir.join("topics.tsv");
        std::fs::write(&topics, "topic\tname\tsubfield_id\tsubfield\tfield_id\tfield\tdomain_id\tdomain\nT1\tGrammatical error correction\t1\tLinguistics\t2\tArts\t3\tH\nT2\tLearner corpora\t1\tLinguistics\t2\tArts\t3\tH\nT9\tProteins\t4\tBio\t5\tLife\t6\tL\n").unwrap();
        let authors = dir.join("authors.tsv");
        std::fs::write(&authors, "folder\tposition\tname\topenalex_author_id\torcid\ts2_author_url\thomepage\tworks_count\tcited_by_count\th_index\n pa-2017-error-types\t1\tAnn Pa\thttps://openalex.org/A1\t\t\thttps://ann.example.org/\t10\t100\t5\npa-2017-error-types\t2\tBo Qu\thttps://openalex.org/A2\t\t\t\t3\t9\t2\n".replace("\n pa", "\npa")).unwrap();
        config::Library { export, citations: cit, refs, text_root: tr, topics: Some(topics), authors: Some(authors) }
    }

    pub fn tmp(name: &str) -> PathBuf {
        std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("topic-corpus-{name}-{}", std::process::id()))
    }

    #[test]
    fn loads_resolves_and_finds_texts() {
        let d = tmp("lib");
        let lib = Library::load(&fixture(&d)).unwrap();
        assert_eq!(lib.works.len(), 7);
        assert_eq!(lib.find("acl:p17-1074").unwrap().id, "pa-2017-error-types");
        assert_eq!(lib.find("doi:https://doi.org/10.18653/V1/P17-1074").unwrap().id, "pa-2017-error-types");
        let p = lib.get("pa-2017-error-types").unwrap();
        let bib = lib.bibliography(p).unwrap();
        assert_eq!(bib.len(), 5);
        assert_eq!(lib.resolve(&bib[0]).unwrap().id, "ra-2015-gec-evaluation");
        assert_eq!(lib.resolve(&bib[1]).unwrap().id, "rb-2012-m2-scorer");
        assert!(lib.resolve(&bib[2]).is_none());
        let (_, local) = lib.text(lib.get("rb-2012-m2-scorer").unwrap()).unwrap();
        assert!(local);
        assert!(lib.text(lib.get("ca-2020-error-types-arabic").unwrap()).is_none());
        let refs = lib.resolved_refs(&["pa-2017-error-types".to_string()].into()).unwrap();
        // A stale layout path still names the folder by its last component.
        assert_eq!(refs["pa-2017-error-types"], vec!["ra-2015-gec-evaluation", "rb-2012-m2-scorer"]);
        let c = lib.citers(&["pa-2017-error-types".to_string()].into()).unwrap();
        assert_eq!(c["pa-2017-error-types"].len(), 2);
        assert_eq!(acl_of_doi("10.18653/v1/p17-1074").as_deref(), Some("P17-1074"));
        let _ = std::fs::remove_dir_all(d);
    }
}
