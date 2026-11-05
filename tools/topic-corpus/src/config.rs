//! The run configuration (`mova-topic-corpus-v1`): where the paper library's files are, where closed
//! corpora and download requests go, and the bounds. Unknown fields are refused; every path is
//! absolute. A deployment keeps its configuration outside this crate.
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "mova-topic-corpus-v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    /// The library export (`papers-<date>.jsonl`: one work per line with ids, licence, text path,
    /// topics, concepts, authors).
    pub export: PathBuf,
    /// `from<TAB>to` library-relative folders (the citation graph inside the library).
    pub citations: PathBuf,
    /// `refs.jsonl`: `{paper, resolved: [folders]}` per citing paper.
    pub refs: PathBuf,
    /// Text root: `<rel>/text.md`, `text.local.md`, `eprint.md`, `refs.json`.
    pub text_root: PathBuf,
    /// OpenAlex topic taxonomy `topic<TAB>name<TAB>…<TAB>field…` (optional).
    #[serde(default)]
    pub topics: Option<PathBuf>,
    /// Work-author positions `folder position name openalex_author_id orcid … homepage …` (optional).
    #[serde(default)]
    pub authors: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Downloads {
    /// The library's `papers` binary (its `openalex lookup` reads the local OpenAlex snapshot).
    pub papers_bin: PathBuf,
    /// Where request run directories are created (`<runs_root>/topic-<key>-<date>/`).
    pub runs_root: PathBuf,
    /// The raw download root the stream driver writes to (slugs must be new there).
    pub raw_root: PathBuf,
    /// The shared stream driver and claim script, copied into each request run directory.
    pub driver: PathBuf,
    pub claim: PathBuf,
    /// At most this many works per request (the most cited first).
    #[serde(default = "d_max_requests")]
    pub max_requests: usize,
}
fn d_max_requests() -> usize {
    40
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    #[serde(default = "d_cited_by")]
    pub cited_by: usize,
    #[serde(default = "d_similar")]
    pub similar: usize,
    #[serde(default = "d_same_author")]
    pub same_author: usize,
    /// Sentences annotated per document at most.
    #[serde(default = "d_sentences")]
    pub max_sentences: usize,
    /// Documents per corpus at most (the paper first, then references, similar, citing, same author).
    #[serde(default = "d_docs")]
    pub max_docs: usize,
    /// The hot neighbourhood: the paper and the first `hot - 1` members are annotated at build time
    /// and pinned in the cache; every other document is read and parsed on request.
    #[serde(default = "d_hot")]
    pub hot: usize,
    /// Bytes of parsed documents kept in the corpus cache (least recently used evicted first;
    /// hot documents never).
    #[serde(default = "d_cache")]
    pub cache_max_bytes: u64,
}
fn d_hot() -> usize {
    12
}
fn d_cache() -> u64 {
    1 << 30
}
fn d_cited_by() -> usize {
    40
}
fn d_similar() -> usize {
    20
}
fn d_same_author() -> usize {
    10
}
fn d_sentences() -> usize {
    1500
}
fn d_docs() -> usize {
    80
}
impl Default for Limits {
    fn default() -> Self {
        Limits { cited_by: d_cited_by(), similar: d_similar(), same_author: d_same_author(), max_sentences: d_sentences(), max_docs: d_docs(), hot: d_hot(), cache_max_bytes: d_cache() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seaspider {
    /// Sea Spider's private socket (the Owls ask as caller `whale`).
    #[serde(default)]
    pub socket: Option<PathBuf>,
    /// Sea Spider's store (read-only here).
    pub store: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema: String,
    pub library: Library,
    #[serde(default)]
    pub downloads: Option<Downloads>,
    /// Closed corpora: `<corpora_root>/<key>/` (internal only).
    pub corpora_root: PathBuf,
    #[serde(default)]
    pub limits: Limits,
    #[serde(default)]
    pub seaspider: Option<Seaspider>,
    /// The shared list of sentences the parser reads badly (`hardcases`), kept until it learns them.
    #[serde(default)]
    pub hard_cases: Option<PathBuf>,
}

impl Config {
    pub fn load(p: &Path) -> Result<Config> {
        let b = std::fs::read(p).with_context(|| format!("config {}", p.display()))?;
        let c: Config = serde_json::from_slice(&b).with_context(|| format!("config {}", p.display()))?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA {
            bail!("schema must be {SCHEMA}");
        }
        let mut paths: Vec<&Path> = vec![&self.library.export, &self.library.citations, &self.library.refs, &self.library.text_root, &self.corpora_root];
        paths.extend(self.library.topics.as_deref());
        paths.extend(self.library.authors.as_deref());
        if let Some(d) = &self.downloads {
            paths.extend([d.papers_bin.as_path(), &d.runs_root, &d.raw_root, &d.driver, &d.claim]);
            if !(1..=500).contains(&d.max_requests) {
                bail!("downloads.max_requests must be 1..=500");
            }
        }
        paths.extend(self.hard_cases.as_deref());
        if let Some(s) = &self.seaspider {
            paths.push(&s.store);
            paths.extend(s.socket.as_deref());
        }
        if let Some(p) = paths.iter().find(|p| !p.is_absolute()) {
            bail!("every path must be absolute: {}", p.display());
        }
        let l = &self.limits;
        if l.cited_by > 1000 || l.similar > 200 || l.same_author > 200 || !(1..=20_000).contains(&l.max_sentences) || !(1..=1000).contains(&l.max_docs) || l.hot == 0 || l.hot > l.max_docs || l.cache_max_bytes < (1 << 20) {
            bail!("limits out of bounds");
        }
        Ok(())
    }
}
