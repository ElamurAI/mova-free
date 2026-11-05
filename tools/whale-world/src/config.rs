//! The configuration (`mova-whale-world-v1`): where dossiers and the world live, where the importers
//! read, the licence policy and the cache sizes. Unknown fields are refused; every path is
//! absolute. A deployment keeps its configuration outside this crate.
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "mova-whale-world-v1";

/// Earlier descriptions on a site tree: `<root>/<a>/<ab>/<abc>/<doc>/<file>` or
/// `<root>/<a>/<ab>/<doc>/<file>`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteOld {
    pub roots: Vec<PathBuf>,
    /// File names to import, in order (each becomes one `site-old` text).
    #[serde(default = "d_site_files")]
    pub files: Vec<String>,
    /// The licence statement recorded for these texts.
    #[serde(default = "d_site_licence")]
    pub licence: String,
}
fn d_site_files() -> Vec<String> {
    vec!["README.md".into()]
}
fn d_site_licence() -> String {
    "earlier description (internal)".into()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rounds {
    /// Only the last round's description of each learner state.
    #[default]
    Last,
    /// Every round's description.
    All,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    /// Parsed text shards kept decoded in one process (least recently used dropped first).
    #[serde(default = "d_cache")]
    pub cache_shards: usize,
    /// Index buckets kept decoded in one process.
    #[serde(default = "d_cache")]
    pub cache_buckets: usize,
    /// Include the cached parses of the documents' topic-corpus neighbours as world documents.
    #[serde(default = "d_true")]
    pub neighbours: bool,
    /// Default size of a writer's context pack in bytes (`world context`).
    #[serde(default = "d_context")]
    pub context_bytes: usize,
}
fn d_cache() -> usize {
    256
}
fn d_true() -> bool {
    true
}
fn d_context() -> usize {
    60_000
}
impl Default for Limits {
    fn default() -> Self {
        Limits { cache_shards: d_cache(), cache_buckets: d_cache(), neighbours: true, context_bytes: d_context() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema: String,
    /// `<dossier_root>/<doc-id>/` per document.
    pub dossier_root: PathBuf,
    /// The global world's directory.
    pub world_root: PathBuf,
    /// The paper library (originals, licences, the document graph).
    #[serde(default)]
    pub library: Option<topic_corpus::config::Library>,
    /// Per-paper topic corpora (`mova-topic-corpus`): web pages, consultant drafts, neighbours.
    #[serde(default)]
    pub corpora_root: Option<PathBuf>,
    #[serde(default)]
    pub site_old: Option<SiteOld>,
    /// Learner state roots holding `papers/<doc>/round-NNN/description.txt`.
    #[serde(default)]
    pub mova_state_roots: Vec<PathBuf>,
    #[serde(default)]
    pub mova_rounds: Rounds,
    /// Whether open-licensed library originals may be marked `public_ok` (false: everything internal).
    #[serde(default)]
    pub public_originals: bool,
    #[serde(default)]
    pub limits: Limits,
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
        let mut paths: Vec<&Path> = vec![&self.dossier_root, &self.world_root];
        if let Some(l) = &self.library {
            paths.extend([l.export.as_path(), &l.citations, &l.refs, &l.text_root]);
        }
        paths.extend(self.corpora_root.as_deref());
        if let Some(s) = &self.site_old {
            paths.extend(s.roots.iter().map(PathBuf::as_path));
            if s.files.iter().any(|f| f.is_empty() || f.contains('/') || f.starts_with('.')) {
                bail!("site_old.files must be plain file names");
            }
        }
        paths.extend(self.mova_state_roots.iter().map(PathBuf::as_path));
        if let Some(p) = paths.iter().find(|p| !p.is_absolute()) {
            bail!("every path must be absolute: {}", p.display());
        }
        if self.limits.cache_shards == 0 || self.limits.cache_buckets == 0 || self.limits.context_bytes < 1000 {
            bail!("limits out of bounds");
        }
        Ok(())
    }
}
