//! The configuration (`mova-hard-cases-v1`): one JSON file, absolute paths, unknown fields refused.
use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "mova-hard-cases-v1";

/// A file pinned by its SHA-256.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pin {
    pub path: PathBuf,
    pub sha256: String,
}

impl Pin {
    /// The file, after checking its hash.
    pub fn check(&self) -> Result<&Path> {
        ensure!(self.path.is_absolute(), "{}: absolute path required", self.path.display());
        let h = crate::file_sha(&self.path)?;
        ensure!(h == self.sha256, "{}: SHA-256 {h}, pinned {}", self.path.display(), self.sha256);
        Ok(&self.path)
    }
}

/// How to get a source's document as a bundle: a command and the fields of the case's `where` to
/// append as its arguments (Wikipedia: `["article"]`; topic corpora: `["corpus", "doc"]`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub command: Vec<String>,
    pub pointer: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gparse {
    /// The native gparse executable.
    pub executable: PathBuf,
    /// The deployed model directory (the one the Eel installs into), read only here.
    pub model_dir: PathBuf,
    /// `GP_LEX` for `gparse annotate`: the corpus that refills lemmas and features.
    #[serde(default)]
    pub lex: Option<PathBuf>,
}

/// The Fish priority lane.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fish {
    /// The annotation profile (Terra through Codex CLI), pinned.
    pub profile: Pin,
    /// The Fish executable the pagers run (`annotation-review-worker`), pinned.
    pub executable: Pin,
    /// Lane pagers per wave (1..8): the generic pagers this many lane pagers stand in for.
    #[serde(default = "two")]
    pub pagers: usize,
    /// Targets per page (intent), 1..256.
    #[serde(default = "page")]
    pub page_size: usize,
    /// Targets per Fish task, 1..8.
    #[serde(default = "four")]
    pub max_targets: usize,
    /// Preceding context sentences per target, 0..8.
    #[serde(default = "eight")]
    pub context: usize,
    /// Cases per wave.
    #[serde(default = "wave")]
    pub max_cases: usize,
    #[serde(default = "timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "poll")]
    pub poll_seconds: u64,
    pub not_after_unix: u64,
}

fn two() -> usize {
    2
}
fn page() -> usize {
    32
}
fn four() -> usize {
    4
}
fn eight() -> usize {
    8
}
fn wave() -> usize {
    256
}
fn timeout() -> u64 {
    600
}
fn poll() -> u64 {
    5
}

/// The three open parsers the Eel's tree agreement needs, and the Stanza bridge the judge uses.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Witness {
    /// Python of Stanza and spaCy (`$MOVA_DATA/parsers/venv/bin/python`).
    pub venv: PathBuf,
    /// Python of Trankit (`$MOVA_DATA/parsers/venv310/bin/python`).
    pub venv310: PathBuf,
    /// The neural runner (`neural-offline.py <parser> <run dir>`).
    pub neural: PathBuf,
    /// The Stanza bridge on Mova's own tokens (`noise_witness.py <work dir>`), CPU.
    pub bridge: PathBuf,
    /// The shared GPU lease file (the Eel's `gpu_lock`).
    pub gpu_lock: PathBuf,
    /// How long to wait for the GPU lease, seconds.
    #[serde(default = "lease_wait")]
    pub lease_wait_seconds: u64,
}

fn lease_wait() -> u64 {
    3600
}

/// The fixed measurement sample.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Measure {
    /// Cases in the fixed sample (the measure split first).
    #[serde(default = "sample")]
    pub sample: usize,
    /// EWT dev for LAS (pinned).
    pub ewt_dev: Option<Pin>,
}

fn sample() -> usize {
    300
}

impl Default for Measure {
    fn default() -> Self {
        Measure { sample: sample(), ewt_dev: None }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema: String,
    /// The one shared hard-cases list.
    pub list: PathBuf,
    /// Waves, ledger, harvested annotations, measurements.
    pub lane_root: PathBuf,
    /// Source name (as in the list) → how to get its documents.
    pub sources: BTreeMap<String, Source>,
    #[serde(default)]
    pub policy: crate::policy::Policy,
    /// Held-out sets: no case whose text (exact or whitespace-normalized) is in them is used.
    #[serde(default)]
    pub heldout: Vec<Pin>,
    /// `en.bin` for the pragmatics labeler (bundles without labels).
    pub en_model: PathBuf,
    /// The pragmatics SLM's silver run.
    pub prag_silver: PathBuf,
    pub gparse: Gparse,
    pub fish: Option<Fish>,
    pub witness: Option<Witness>,
    #[serde(default)]
    pub measure: Measure,
    /// The hard-case Eel round: the public Eel engine's configuration for each track
    /// (`shipped`, `internal`), as `mova_eel::trainer::run` reads it.
    #[serde(default)]
    pub eel: BTreeMap<String, serde_json::Value>,
    /// The lane's priority among the Fish and its share of the limit guard.
    #[serde(default)]
    pub control: Option<crate::control::Control>,
}

impl Config {
    pub fn load(p: &Path) -> Result<Config> {
        let c: Config = serde_json::from_slice(&std::fs::read(p).with_context(|| format!("{}", p.display()))?).with_context(|| format!("{}", p.display()))?;
        if c.schema != SCHEMA {
            bail!("{}: schema {} (expected {SCHEMA})", p.display(), c.schema);
        }
        for q in [&c.list, &c.lane_root, &c.en_model, &c.prag_silver, &c.gparse.executable, &c.gparse.model_dir] {
            ensure!(q.is_absolute(), "{}: absolute paths required", q.display());
        }
        if let Some(f) = &c.fish {
            ensure!((1..=8).contains(&f.pagers), "fish.pagers: 1..8");
            ensure!((1..=256).contains(&f.page_size), "fish.page_size: 1..256");
            ensure!((1..=8).contains(&f.max_targets), "fish.max_targets: 1..8");
            ensure!(f.context <= 8, "fish.context: 0..8");
            ensure!((1..=900).contains(&f.timeout_seconds), "fish.timeout_seconds: 1..900");
            ensure!((1..=60).contains(&f.poll_seconds), "fish.poll_seconds: 1..60");
        }
        for (t, e) in &c.eel {
            ensure!(t == "shipped" || t == "internal", "eel: tracks are shipped and internal, not {t}");
            ensure!(e.is_object(), "eel.{t}: an Eel engine configuration object");
        }
        c.policy.validate()?;
        Ok(c)
    }
}
