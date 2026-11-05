//! Mova's add-on modules: data packages (dictionary layers first) that Mova loads at run time, each under its own
//! licence, installed into the data directory from our server over HTTPS or by BitTorrent. A module is never part of
//! the repository, a release's core files or a binary (docs/modules.md).
//!
//! A module is a folder `<root>/<name>/<version>/` holding `module.json` and its files:
//!
//! ```json
//! {"schema": 1, "name": "example-dict", "version": "2026.11.04", "kind": "dictionary-layer",
//!  "licence": "CC-BY-SA-4.0", "licence_class": "cc-by-sa",
//!  "attribution": {"text": "From …, CC BY-SA 4.0", "sources": ["https://…"]},
//!  "files": [{"path": "layers/senses.tsv", "sha256": "…", "bytes": 1234}],
//!  "provides": [{"kind": "dictionary-layer", "layer": "senses", "path": "layers/senses.tsv"}],
//!  "requires_mova": ">=0.5"}
//! ```
//!
//! `attribution` is a text or `{"text", "sources"}`; a `provides` entry may add `"state"` (draft, the default, admitted
//! or derived) and, for an admitted layer, `"review"` (a listed file holding its review record). Unknown fields are
//! ignored, so later fields extend the format compatibly. Paths are relative, of `[A-Za-z0-9._-]` segments.
//!
//! Roots, searched in order: each folder of `MOVA_MODULES_PATH` (`:`-separated), then `$MOVA_DATA/modules`
//! (`MOVA_DATA` defaults to `~/mova-data`). A root inside a Git work tree is refused: modules never live in a repository.
//!
//!   mova modules list                                   installed modules, their licences, roots
//!   mova modules info NAME[@VERSION]                    the manifest, attribution and what it provides
//!   mova modules verify NAME[@VERSION]                  every file's size and SHA-256 against module.json
//!   mova modules install MANIFEST_URL|DIR|module.json   fetch (https:// or file://) or copy, check hashes, install
//!   mova modules install-torrent TORRENT|MAGNET         download with a BitTorrent client (aria2c), then install
//!   mova modules make-torrent DIR OUT.torrent           a .torrent of a module folder (BitTorrent v1), and its magnet
//!   mova modules remove NAME@VERSION                    remove one installed version
//!
//! Options: `--dir ROOT` (install into ROOT; searched first by the other commands), `--sha256 HASH` (pin the
//! module.json bytes on install), `--accept-nc` (needed to install a non-commercial / research-only module),
//! `--tracker URL` and `--web-seed URL` (make-torrent, repeatable).

pub mod fetch;
pub mod oewn;
pub mod torrent;

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The manifest schema this build reads.
pub const SCHEMA: u32 = 1;
/// The release of the module mechanism this build implements; `requires_mova` is checked against it. The mechanism
/// is part of release 0.5 (the workspace version 0.5.0).
pub const MOVA_RELEASE: &str = "0.5";

/// The licence class of a module: what Mova may do with it and with what it derives from it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LicenceClass {
    /// Ours, under the Elamur Cosmic License.
    Ours,
    /// Creative Commons Attribution-ShareAlike: derived material keeps the licence.
    CcBySa,
    /// Creative Commons Attribution: credit the source.
    CcBy,
    /// Public domain or CC0.
    PublicDomain,
    /// Non-commercial or research-only terms: internal runs and measurements only.
    NcResearch,
}

impl LicenceClass {
    pub fn as_str(self) -> &'static str {
        match self {
            LicenceClass::Ours => "ours",
            LicenceClass::CcBySa => "cc-by-sa",
            LicenceClass::CcBy => "cc-by",
            LicenceClass::PublicDomain => "public-domain",
            LicenceClass::NcResearch => "nc-research",
        }
    }
    /// What Mova derives from such a module carries its licence and never enters a shipped file.
    pub fn taints(self) -> bool {
        matches!(self, LicenceClass::CcBySa | LicenceClass::NcResearch)
    }
    /// Loaded only for internal runs and measurements.
    pub fn research_only(self) -> bool {
        self == LicenceClass::NcResearch
    }
    /// What the class means for a user, shown on install and by `info`.
    pub fn notice(self) -> &'static str {
        match self {
            LicenceClass::Ours => "ours (Elamur Cosmic License): use freely, except Big Brother",
            LicenceClass::PublicDomain => "public domain or CC0: no conditions",
            LicenceClass::CcBy => "attribution: credit the source when you share what you make with it",
            LicenceClass::CcBySa => "share-alike: Mova loads it at run time; whatever Mova derives from it is marked with this licence, must be shared under it, and is never written into Mova's repository or shipped files",
            LicenceClass::NcResearch => "non-commercial / research only: loaded only for internal runs and measurements (MOVA_DICT_USE=internal); what Mova derives from it is marked and never shipped; installing it needs --accept-nc",
        }
    }
}

/// A module's attribution: a text, or a text and its source URLs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attribution {
    Text(String),
    Full {
        text: String,
        #[serde(default, alias = "urls")]
        sources: Vec<String>,
    },
}

impl Attribution {
    pub fn text(&self) -> &str {
        match self {
            Attribution::Text(t) => t,
            Attribution::Full { text, .. } => text,
        }
    }
    pub fn sources(&self) -> &[String] {
        match self {
            Attribution::Text(_) => &[],
            Attribution::Full { sources, .. } => sources,
        }
    }
    /// One line: the text and its sources.
    pub fn line(&self) -> String {
        match self.sources() {
            [] => self.text().to_string(),
            s => format!("{} ({})", self.text(), s.join(", ")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provide {
    pub kind: String,
    #[serde(default)]
    pub layer: Option<String>,
    pub path: String,
    /// The layer's state for the dictionary store: draft (the default), admitted or derived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// For an admitted layer: the listed file holding its review record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<String>,
}

impl Provide {
    pub fn state(&self) -> &str {
        self.state.as_deref().unwrap_or("draft")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    pub name: String,
    pub version: String,
    pub kind: String,
    pub licence: String,
    pub licence_class: LicenceClass,
    pub attribution: Attribution,
    pub files: Vec<FileEntry>,
    pub provides: Vec<Provide>,
    #[serde(default)]
    pub requires_mova: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn safe_name(s: &str) -> bool {
    !s.is_empty() && s.len() <= 128 && !s.starts_with('.') && s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

/// A relative path of safe segments (no `.`, `..`, empty or hidden segment, no backslash).
pub fn safe_rel(p: &str) -> bool {
    !p.is_empty() && p.len() <= 512 && p.split('/').all(safe_name)
}

impl Manifest {
    pub fn parse(bytes: &[u8]) -> Result<Manifest> {
        let m: Manifest = serde_json::from_slice(bytes).context("parsing module.json")?;
        m.validate()?;
        Ok(m)
    }

    /// The format's rules (not the files: see `verify_dir`).
    pub fn validate(&self) -> Result<()> {
        ensure!(self.schema == SCHEMA, "unsupported module schema {} (this Mova reads {SCHEMA})", self.schema);
        ensure!(safe_name(&self.name), "a module name is [A-Za-z0-9._-], not hidden: «{}»", self.name);
        ensure!(safe_name(&self.version), "a module version is [A-Za-z0-9._-], not hidden: «{}»", self.version);
        ensure!(!self.kind.trim().is_empty(), "a module names its kind");
        ensure!(!self.licence.trim().is_empty(), "a module names its licence");
        ensure!(!self.attribution.text().trim().is_empty(), "a module names its attribution");
        ensure!(!self.files.is_empty(), "a module lists its files");
        let mut paths = BTreeSet::new();
        for f in &self.files {
            ensure!(safe_rel(&f.path), "unsafe file path «{}» (relative, [A-Za-z0-9._-] segments)", f.path);
            ensure!(f.path != "module.json", "module.json is the manifest, not a listed file");
            ensure!(f.sha256.len() == 64 && f.sha256.chars().all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c)), "file «{}»: SHA-256 is 64 lower-case hex digits", f.path);
            ensure!(paths.insert(f.path.as_str()), "file «{}» listed twice", f.path);
        }
        let mut layers = BTreeSet::new();
        for p in &self.provides {
            ensure!(paths.contains(p.path.as_str()), "provides «{}», which the files do not list", p.path);
            if p.kind == "dictionary-layer" {
                let l = p.layer.as_deref().unwrap_or("");
                ensure!(safe_name(l), "a dictionary layer has a name of [A-Za-z0-9._-]: «{l}»");
                ensure!(layers.insert(l), "dictionary layer «{l}» twice");
                ensure!(matches!(p.state(), "draft" | "admitted" | "derived"), "layer «{l}»: state is draft, admitted or derived, not «{}»", p.state());
                if p.state() == "admitted" {
                    let r = p.review.as_deref().unwrap_or("");
                    ensure!(paths.contains(r), "layer «{l}» is admitted: its review record must be a listed file");
                }
            }
        }
        requires(&self.requires_mova, MOVA_RELEASE).with_context(|| format!("module {}@{}", self.name, self.version))?;
        Ok(())
    }

    /// The dictionary layers it provides.
    pub fn dictionary_layers(&self) -> impl Iterator<Item = &Provide> {
        self.provides.iter().filter(|p| p.kind == "dictionary-layer")
    }

    pub fn file(&self, path: &str) -> Option<&FileEntry> {
        self.files.iter().find(|f| f.path == path)
    }

    /// The licence notice a user sees on install and in `info`.
    pub fn notice(&self) -> String {
        format!("{}@{}: licence {} [{}] — {}\n  attribution: {}", self.name, self.version, self.licence, self.licence_class.as_str(), self.licence_class.notice(), self.attribution.line())
    }
}

/// Compare two versions: segments split on `.`, `-`, `+`; numbers numerically, otherwise as text; missing segments
/// count as 0.
pub fn cmp_version(a: &str, b: &str) -> std::cmp::Ordering {
    let seg = |s: &str| s.split(['.', '-', '+']).map(|x| x.to_string()).collect::<Vec<_>>();
    let (x, y) = (seg(a), seg(b));
    for i in 0..x.len().max(y.len()) {
        let (p, q) = (x.get(i).map(String::as_str).unwrap_or("0"), y.get(i).map(String::as_str).unwrap_or("0"));
        let o = match (p.parse::<u64>(), q.parse::<u64>()) {
            (Ok(m), Ok(n)) => m.cmp(&n),
            _ => p.cmp(q),
        };
        if o != std::cmp::Ordering::Equal {
            return o;
        }
    }
    std::cmp::Ordering::Equal
}

/// Check a requirement such as `>=0.5` or `>=0.5, <1` against a version; empty means any.
pub fn requires(req: &str, have: &str) -> Result<()> {
    use std::cmp::Ordering::*;
    for c in req.split(',').map(str::trim).filter(|c| !c.is_empty()) {
        let (op, v) = [">=", "<=", "==", ">", "<", "="].iter().find_map(|op| c.strip_prefix(op).map(|v| (*op, v.trim()))).unwrap_or(("=", c));
        ensure!(!v.is_empty(), "requires_mova: no version in «{c}»");
        let o = cmp_version(have, v);
        let ok = match op {
            ">=" => o != Less,
            "<=" => o != Greater,
            ">" => o == Greater,
            "<" => o == Less,
            _ => o == Equal,
        };
        ensure!(ok, "requires Mova {req}; this Mova implements modules of release {have}");
    }
    Ok(())
}

// ---- roots and discovery ----------------------------------------------------------------------------------------

/// The data directory: `MOVA_DATA`, else `~/mova-data`.
pub fn data_dir() -> PathBuf {
    match std::env::var("MOVA_DATA") {
        Ok(d) if !d.is_empty() => PathBuf::from(d),
        _ => PathBuf::from(std::env::var("HOME").unwrap_or_default()).join("mova-data"),
    }
}

/// The module roots, in search order: `MOVA_MODULES_PATH` (`:`-separated), then `$MOVA_DATA/modules`.
pub fn roots() -> Vec<PathBuf> {
    let mut r: Vec<PathBuf> = std::env::var("MOVA_MODULES_PATH").unwrap_or_default().split(':').filter(|s| !s.is_empty()).map(PathBuf::from).collect();
    r.push(data_dir().join("modules"));
    r
}

/// Whether a path (existing or not) lies inside a Git work tree: its nearest existing ancestor, canonicalized, or
/// one of that one's ancestors holds `.git`. Modules and what Mova derives from share-alike modules never go there.
pub fn inside_git_worktree(path: &Path) -> bool {
    let abs = if path.is_absolute() { path.to_path_buf() } else { std::env::current_dir().unwrap_or_default().join(path) };
    let mut p: &Path = &abs;
    let existing = loop {
        if let Ok(c) = p.canonicalize() {
            break c;
        }
        match p.parent() {
            Some(q) => p = q,
            None => return false,
        }
    };
    existing.ancestors().any(|a| a.join(".git").exists())
}

/// An installed module.
#[derive(Clone, Debug)]
pub struct Installed {
    pub root: PathBuf,
    pub dir: PathBuf,
    pub manifest: Manifest,
    /// SHA-256 of the module.json bytes (a dictionary manifest may pin it).
    pub manifest_sha256: String,
}

#[derive(Debug, Default)]
pub struct Discovery {
    /// Every valid installed module, by (name, version), the first root's copy; later roots' copies are shadowed.
    pub modules: BTreeMap<(String, String), Installed>,
    pub shadowed: Vec<Installed>,
    /// Folders that look like modules but are not valid ones, with the reason.
    pub problems: Vec<String>,
}

fn sorted_dirs(d: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(d).into_iter().flatten().flatten().map(|e| e.path()).filter(|p| p.is_dir() && !p.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.'))).collect();
    v.sort();
    v
}

pub fn read_installed(root: &Path, dir: &Path) -> Result<Installed> {
    let bytes = std::fs::read(dir.join("module.json")).with_context(|| format!("reading {}/module.json", dir.display()))?;
    let manifest = Manifest::parse(&bytes).with_context(|| dir.display().to_string())?;
    let (n, v) = (dir.parent().and_then(|p| p.file_name()), dir.file_name());
    ensure!(n.is_some_and(|n| n == manifest.name.as_str()) && v.is_some_and(|v| v == manifest.version.as_str()), "{} holds module {}@{}: the folder must be <root>/{}/{}", dir.display(), manifest.name, manifest.version, manifest.name, manifest.version);
    Ok(Installed { root: root.to_path_buf(), dir: dir.to_path_buf(), manifest, manifest_sha256: sha256_hex(&bytes) })
}

pub fn discover(roots: &[PathBuf]) -> Discovery {
    let mut d = Discovery::default();
    for root in roots {
        for name_dir in sorted_dirs(root) {
            for ver_dir in sorted_dirs(&name_dir) {
                if !ver_dir.join("module.json").exists() {
                    continue;
                }
                match read_installed(root, &ver_dir) {
                    Ok(m) => {
                        let key = (m.manifest.name.clone(), m.manifest.version.clone());
                        if d.modules.contains_key(&key) {
                            d.shadowed.push(m);
                        } else {
                            d.modules.insert(key, m);
                        }
                    }
                    Err(e) => d.problems.push(format!("{e:#}")),
                }
            }
        }
    }
    d
}

/// `NAME` or `NAME@VERSION`.
pub fn split_ref(r: &str) -> (&str, Option<&str>) {
    match r.split_once('@') {
        Some((n, v)) => (n, Some(v)),
        None => (r, None),
    }
}

/// The installed module of a name (the highest version, or the named one).
pub fn find(roots: &[PathBuf], name: &str, version: Option<&str>) -> Result<Installed> {
    let d = discover(roots);
    let mut c: Vec<&Installed> = d.modules.values().filter(|m| m.manifest.name == name && version.is_none_or(|v| m.manifest.version == v)).collect();
    c.sort_by(|a, b| cmp_version(&a.manifest.version, &b.manifest.version));
    match c.last() {
        Some(m) => Ok((*m).clone()),
        None => bail!("module {name}{} is not installed (roots: {})", version.map(|v| format!("@{v}")).unwrap_or_default(), roots.iter().map(|r| r.display().to_string()).collect::<Vec<_>>().join(", ")),
    }
}

// ---- verification -------------------------------------------------------------------------------------------------

/// Read one listed file of a module folder and check its size and SHA-256.
pub fn read_verified(dir: &Path, f: &FileEntry) -> Result<Vec<u8>> {
    let p = dir.join(&f.path);
    let bytes = std::fs::read(&p).with_context(|| format!("reading {}", p.display()))?;
    ensure!(bytes.len() as u64 == f.bytes, "{}: {} bytes, module.json says {}", f.path, bytes.len(), f.bytes);
    let h = sha256_hex(&bytes);
    ensure!(h == f.sha256, "{}: SHA-256 {h}, module.json says {}", f.path, f.sha256);
    Ok(bytes)
}

/// Check every listed file of a module folder; returns the files present but not listed (reported, not refused).
pub fn verify_dir(dir: &Path, m: &Manifest) -> Result<Vec<String>> {
    for f in &m.files {
        read_verified(dir, f)?;
    }
    let listed: BTreeSet<&str> = m.files.iter().map(|f| f.path.as_str()).collect();
    let mut extra = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)?.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                let rel = p.strip_prefix(dir).unwrap_or(&p).to_string_lossy().replace('\\', "/");
                if rel != "module.json" && !listed.contains(rel.as_str()) {
                    extra.push(rel);
                }
            }
        }
    }
    extra.sort();
    Ok(extra)
}

// ---- install and remove -------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct InstallOptions {
    /// Pin the SHA-256 of the module.json bytes.
    pub sha256: Option<String>,
    /// Accept a non-commercial / research-only module.
    pub accept_nc: bool,
}

/// Fetch a URL: `https://` through curl (TLS, redirects, fail on HTTP errors), `file://` from disk; plain `http://`
/// and anything else are refused.
pub fn fetch(url: &str) -> Result<Vec<u8>> {
    if let Some(p) = url.strip_prefix("file://") {
        return std::fs::read(p).with_context(|| format!("reading {p}"));
    }
    ensure!(url.starts_with("https://"), "only https:// (or file://) URLs: {url}");
    let out = std::process::Command::new("curl").args(["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--retry", "2", url]).output().context("running curl (needed for https downloads)")?;
    ensure!(out.status.success(), "downloading {url} failed: {}", String::from_utf8_lossy(&out.stderr).trim());
    Ok(out.stdout)
}

fn check_admission(m: &Manifest, bytes: &[u8], opts: &InstallOptions) -> Result<()> {
    if let Some(h) = &opts.sha256 {
        ensure!(&sha256_hex(bytes) == h, "module.json has SHA-256 {}, --sha256 pins {h}", sha256_hex(bytes));
    }
    ensure!(!m.licence_class.research_only() || opts.accept_nc, "{} is non-commercial / research-only ({}): install it with --accept-nc for internal runs and measurements only", m.name, m.licence);
    Ok(())
}

fn check_root(root: &Path) -> Result<()> {
    ensure!(!inside_git_worktree(root), "{} is inside a Git work tree: modules never live in a repository", root.display());
    Ok(())
}

/// Stage, verify and move a module into `<root>/<name>/<version>`; `put` writes one listed file into the staging folder.
fn install_with(root: &Path, m: &Manifest, manifest_bytes: &[u8], mut put: impl FnMut(&FileEntry, &Path) -> Result<()>) -> Result<Installed> {
    check_root(root)?;
    let dest = root.join(&m.name).join(&m.version);
    if dest.join("module.json").exists() {
        let have = read_installed(root, &dest)?;
        ensure!(have.manifest_sha256 == sha256_hex(manifest_bytes), "{}@{} is installed in {} with another module.json; remove it first", m.name, m.version, dest.display());
        verify_dir(&dest, &have.manifest)?;
        return Ok(have);
    }
    ensure!(!dest.exists(), "{} exists and is not a module", dest.display());
    std::fs::create_dir_all(root)?;
    let staging = root.join(format!(".staging-{}-{}-{}", m.name, m.version, std::process::id()));
    ensure!(!staging.exists(), "{} exists (an interrupted install?)", staging.display());
    std::fs::create_dir_all(&staging)?;
    let r = (|| -> Result<()> {
        for f in &m.files {
            let p = staging.join(&f.path);
            std::fs::create_dir_all(p.parent().expect("a file path has a parent"))?;
            put(f, &p)?;
            read_verified(&staging, f)?;
        }
        std::fs::write(staging.join("module.json"), manifest_bytes)?;
        std::fs::create_dir_all(dest.parent().expect("<root>/<name>"))?;
        std::fs::rename(&staging, &dest)?;
        Ok(())
    })();
    if let Err(e) = r {
        let _ = std::fs::remove_dir_all(&staging);
        return Err(e);
    }
    read_installed(root, &dest)
}

/// Install from a module folder (or its module.json) on disk.
pub fn install_dir(src: &Path, root: &Path, opts: &InstallOptions) -> Result<Installed> {
    let dir = if src.is_dir() { src.to_path_buf() } else { src.parent().unwrap_or(Path::new(".")).to_path_buf() };
    let bytes = std::fs::read(dir.join("module.json")).with_context(|| format!("reading {}/module.json", dir.display()))?;
    let m = Manifest::parse(&bytes)?;
    check_admission(&m, &bytes, opts)?;
    verify_dir(&dir, &m)?;
    install_with(root, &m, &bytes, |f, p| {
        std::fs::copy(dir.join(&f.path), p)?;
        Ok(())
    })
}

/// Install from a module.json URL: the files are fetched next to it (`<base>/<path>`), each checked before the
/// module is moved into place.
pub fn install_url(url: &str, root: &Path, opts: &InstallOptions) -> Result<Installed> {
    let bytes = fetch(url)?;
    let m = Manifest::parse(&bytes)?;
    check_admission(&m, &bytes, opts)?;
    eprintln!("{}", m.notice());
    let base = url.rsplit_once('/').map(|x| x.0).context("a manifest URL has a folder")?;
    install_with(root, &m, &bytes, |f, p| {
        std::fs::write(p, fetch(&format!("{base}/{}", f.path))?)?;
        Ok(())
    })
}

/// Remove one installed version (the folder must be a valid module at `<root>/<name>/<version>`).
pub fn remove(roots: &[PathBuf], name: &str, version: &str) -> Result<PathBuf> {
    let m = find(roots, name, Some(version))?;
    let expect = m.root.join(&m.manifest.name).join(&m.manifest.version);
    ensure!(m.dir == expect, "refusing to remove {}: not <root>/<name>/<version>", m.dir.display());
    std::fs::remove_dir_all(&m.dir).with_context(|| format!("removing {}", m.dir.display()))?;
    let parent = m.root.join(&m.manifest.name);
    if std::fs::read_dir(&parent).map(|mut d| d.next().is_none()).unwrap_or(false) {
        std::fs::remove_dir(&parent)?;
    }
    Ok(m.dir)
}

/// A program on PATH.
pub fn which(prog: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|p| std::env::split_paths(&p).map(|d| d.join(prog)).find(|f| f.is_file()))
}

/// Find the module folder a download produced: the unique folder holding module.json, at most two levels down.
fn downloaded_module(dir: &Path) -> Result<PathBuf> {
    let mut found = Vec::new();
    for a in sorted_dirs(dir) {
        if a.join("module.json").is_file() {
            found.push(a.clone());
        }
        for b in sorted_dirs(&a) {
            if b.join("module.json").is_file() {
                found.push(b);
            }
        }
    }
    ensure!(found.len() == 1, "the download in {} holds {} module folders, expected one", dir.display(), found.len());
    Ok(found.remove(0))
}

/// The instructions printed when no supported BitTorrent client is installed.
pub fn torrent_instructions(src: &str, root: &Path) -> String {
    format!(
        "no supported BitTorrent client found (aria2c). Download the module with any client, then install the folder it holds:\n  \
         aria2c --seed-time=0 --dir DOWNLOADS '{src}'\n  \
         (or: transmission-cli --download-dir DOWNLOADS '{src}', and stop it when the download is complete)\n  \
         mova modules install DOWNLOADS/<name>-<version> --dir {}\n\
         Install aria2 (e.g. `apt install aria2`) to let `mova modules install-torrent` do this itself.",
        root.display()
    )
}

/// Download a module by BitTorrent with aria2c into a staging folder under the root, then install it from there.
pub fn install_torrent(src: &str, root: &Path, opts: &InstallOptions) -> Result<Installed> {
    check_root(root)?;
    let Some(aria) = which("aria2c") else { bail!("{}", torrent_instructions(src, root)) };
    std::fs::create_dir_all(root)?;
    let staging = root.join(format!(".torrent-{}", std::process::id()));
    ensure!(!staging.exists(), "{} exists", staging.display());
    let st = std::process::Command::new(aria).args(["--seed-time=0", "--follow-torrent=mem", "--bt-save-metadata=false", "--summary-interval=0", "--dir"]).arg(&staging).arg(src).status().context("running aria2c")?;
    ensure!(st.success(), "aria2c failed ({st}); the partial download is in {}", staging.display());
    let dir = downloaded_module(&staging)?;
    let r = install_dir(&dir, root, opts);
    if r.is_ok() {
        let _ = std::fs::remove_dir_all(&staging);
    }
    r
}

// ---- the command ----------------------------------------------------------------------------------------------------

struct Opts {
    pos: Vec<String>,
    dir: Option<PathBuf>,
    install: InstallOptions,
    trackers: Vec<String>,
    web_seeds: Vec<String>,
}

fn parse_opts(args: &[String]) -> Result<Opts> {
    let mut o = Opts { pos: Vec::new(), dir: None, install: InstallOptions::default(), trackers: Vec::new(), web_seeds: Vec::new() };
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        let mut val = || -> Result<String> {
            i += 1;
            args.get(i).cloned().with_context(|| format!("{a} needs a value"))
        };
        match a {
            "--dir" => o.dir = Some(PathBuf::from(val()?)),
            "--sha256" => o.install.sha256 = Some(val()?),
            "--tracker" => o.trackers.push(val()?),
            "--web-seed" => o.web_seeds.push(val()?),
            "--accept-nc" => o.install.accept_nc = true,
            _ if a.starts_with("--") => bail!("unknown option {a}"),
            _ => o.pos.push(a.to_string()),
        }
        i += 1;
    }
    Ok(o)
}

const USAGE: &str = "usage: mova modules list | info NAME[@VERSION] | verify NAME[@VERSION] | install MANIFEST_URL|DIR|module.json [--dir ROOT] [--sha256 HASH] [--accept-nc] | install-torrent TORRENT|MAGNET [--dir ROOT] [--accept-nc] | make-torrent DIR OUT.torrent [--tracker URL]… [--web-seed URL]… | remove NAME@VERSION [--dir ROOT]";

/// `mova modules …` (args after `modules`).
pub fn main_args(args: &[String]) -> Result<()> {
    let o = parse_opts(args)?;
    let mut roots = roots();
    if let Some(d) = &o.dir {
        roots.insert(0, d.clone());
    }
    let target = o.dir.clone().unwrap_or_else(|| roots.last().cloned().expect("the data root"));
    let arg = |i: usize| o.pos.get(i).map(String::as_str).context(USAGE);
    match o.pos.first().map(String::as_str) {
        Some("list") => {
            let d = discover(&roots);
            for m in d.modules.values() {
                let x = &m.manifest;
                let layers: Vec<&str> = x.dictionary_layers().filter_map(|p| p.layer.as_deref()).collect();
                println!("{}\t{}\t{}\t{}\t[{}]\t{}\t{}", x.name, x.version, x.kind, x.licence, x.licence_class.as_str(), if layers.is_empty() { "-".into() } else { layers.join(",") }, m.dir.display());
            }
            for m in &d.shadowed {
                println!("# shadowed by an earlier root: {}@{} in {}", m.manifest.name, m.manifest.version, m.dir.display());
            }
            for p in &d.problems {
                println!("# not a valid module: {p}");
            }
            if d.modules.is_empty() {
                println!("# no modules installed (roots: {})", roots.iter().map(|r| r.display().to_string()).collect::<Vec<_>>().join(", "));
            }
        }
        Some("info") => {
            let (n, v) = split_ref(arg(1)?);
            let m = find(&roots, n, v)?;
            println!("{}", m.manifest.notice());
            println!("  folder: {}\n  module.json SHA-256: {}\n  kind: {}\n  requires Mova: {}", m.dir.display(), m.manifest_sha256, m.manifest.kind, if m.manifest.requires_mova.is_empty() { "any" } else { &m.manifest.requires_mova });
            if let Some(d) = &m.manifest.description {
                println!("  {d}");
            }
            for p in &m.manifest.provides {
                println!("  provides {} {} ({}, {})", p.kind, p.layer.as_deref().unwrap_or("-"), p.path, p.state());
            }
            let total: u64 = m.manifest.files.iter().map(|f| f.bytes).sum();
            println!("  {} files, {total} bytes", m.manifest.files.len());
        }
        Some("verify") => {
            let (n, v) = split_ref(arg(1)?);
            let m = find(&roots, n, v)?;
            let extra = verify_dir(&m.dir, &m.manifest)?;
            println!("ok: {}@{}, {} files match module.json ({})", m.manifest.name, m.manifest.version, m.manifest.files.len(), m.dir.display());
            for e in extra {
                println!("# not listed in module.json: {e}");
            }
        }
        Some("install") => {
            let src = arg(1)?;
            let m = if src.starts_with("https://") || src.starts_with("file://") || src.starts_with("http://") {
                install_url(src, &target, &o.install)?
            } else {
                let m = install_dir(Path::new(src), &target, &o.install)?;
                eprintln!("{}", m.manifest.notice());
                m
            };
            println!("installed {}@{} in {}", m.manifest.name, m.manifest.version, m.dir.display());
        }
        Some("install-torrent") => {
            let m = install_torrent(arg(1)?, &target, &o.install)?;
            println!("{}\ninstalled {}@{} in {}", m.manifest.notice(), m.manifest.name, m.manifest.version, m.dir.display());
        }
        Some("make-torrent") => {
            let (dir, out) = (Path::new(arg(1)?), Path::new(arg(2)?));
            let t = torrent::make(dir, &o.trackers, &o.web_seeds)?;
            let mut f = std::fs::OpenOptions::new().create_new(true).write(true).open(out).with_context(|| format!("{} exists or cannot be made", out.display()))?;
            std::io::Write::write_all(&mut f, &t.bytes)?;
            println!("{}\t{} files, {} bytes, {} pieces of {} KiB\n{}", out.display(), t.files, t.total, t.pieces, t.piece_length / 1024, t.magnet);
        }
        Some("remove") => {
            let (n, v) = split_ref(arg(1)?);
            let v = v.context("remove names the version: NAME@VERSION")?;
            println!("removed {}", remove(&roots, n, v)?.display());
        }
        _ => bail!("{USAGE}"),
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("mova-modules-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    pub const LAYER: &str = "# word\tpart of speech\tmeaning for a pupil\tsense in the lessons\texamples\tnote\ntelly\tnoun\tA television set.\t\t\t\n";

    /// A module folder `<parent>/<name>-<version>` with one dictionary layer.
    pub fn make_module(parent: &Path, name: &str, version: &str, class: &str, licence: &str) -> PathBuf {
        let d = parent.join(format!("{name}-{version}"));
        std::fs::create_dir_all(d.join("layers")).unwrap();
        std::fs::write(d.join("layers/senses.tsv"), LAYER).unwrap();
        let m = serde_json::json!({"schema": 1, "name": name, "version": version, "kind": "dictionary-layer", "licence": licence,
            "licence_class": class, "attribution": {"text": "test source", "sources": ["https://example.org/"]},
            "files": [{"path": "layers/senses.tsv", "sha256": sha256_hex(LAYER.as_bytes()), "bytes": LAYER.len()}],
            "provides": [{"kind": "dictionary-layer", "layer": "senses", "path": "layers/senses.tsv"}], "requires_mova": ">=0.5"});
        std::fs::write(d.join("module.json"), serde_json::to_vec_pretty(&m).unwrap()).unwrap();
        d
    }

    #[test]
    fn manifest_rules_are_enforced() {
        let d = tmp("rules");
        let src = make_module(&d, "t", "1.0", "cc-by-sa", "CC-BY-SA-4.0");
        let bytes = std::fs::read(src.join("module.json")).unwrap();
        let m = Manifest::parse(&bytes).unwrap();
        assert_eq!(m.licence_class, LicenceClass::CcBySa);
        assert!(m.licence_class.taints());
        assert_eq!(m.attribution.sources(), ["https://example.org/".to_string()]);
        // a plain-text attribution is accepted too
        let mut v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        v["attribution"] = "From somewhere".into();
        assert!(Manifest::parse(&serde_json::to_vec(&v).unwrap()).is_ok());
        // unknown fields are ignored (compatible extension)
        v["future_field"] = 1.into();
        assert!(Manifest::parse(&serde_json::to_vec(&v).unwrap()).is_ok());
        // negative controls: each breaks one rule
        let bad = |f: &dyn Fn(&mut serde_json::Value)| {
            let mut w: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            f(&mut w);
            Manifest::parse(&serde_json::to_vec(&w).unwrap()).is_err()
        };
        assert!(bad(&|w| w["schema"] = 2.into()));
        assert!(bad(&|w| w["name"] = "../x".into()));
        assert!(bad(&|w| w["files"][0]["path"] = "../escape.tsv".into()));
        assert!(bad(&|w| w["files"][0]["path"] = "/abs.tsv".into()));
        assert!(bad(&|w| w["files"][0]["sha256"] = "XYZ".into()));
        assert!(bad(&|w| w["licence_class"] = "gpl".into()));
        assert!(bad(&|w| w["provides"][0]["path"] = "missing.tsv".into()));
        assert!(bad(&|w| w["requires_mova"] = ">=9.0".into()));
        assert!(bad(&|w| w["provides"][0]["state"] = "admitted".into()));
        assert!(bad(&|w| w["provides"][0]["state"] = "proposal".into()));
    }

    #[test]
    fn versions_and_requirements() {
        use std::cmp::Ordering::*;
        assert_eq!(cmp_version("0.10", "0.9"), Greater);
        assert_eq!(cmp_version("0.5", "0.5.0"), Equal);
        assert_eq!(cmp_version("2026.11.04", "2026.11.4"), Equal);
        assert!(requires(">=0.5", "0.5").is_ok());
        assert!(requires(">=0.5, <1", "0.5").is_ok());
        assert!(requires("", "0.5").is_ok());
        assert!(requires(">=0.6", "0.5").is_err());
        assert!(requires("<0.5", "0.5").is_err());
    }

    #[test]
    fn install_verify_list_remove() {
        let d = tmp("install");
        let src = make_module(&d.join("src"), "t", "1.0", "cc-by", "CC-BY-4.0");
        let root = d.join("root");
        let m = install_dir(&src, &root, &InstallOptions::default()).unwrap();
        assert_eq!(m.dir, root.join("t/1.0"));
        assert!(verify_dir(&m.dir, &m.manifest).unwrap().is_empty());
        // installing again is a no-op; a newer version sorts last
        install_dir(&src, &root, &InstallOptions::default()).unwrap();
        let src2 = make_module(&d.join("src"), "t", "1.10", "cc-by", "CC-BY-4.0");
        install_url(&format!("file://{}/module.json", src2.display()), &root, &InstallOptions::default()).unwrap();
        let roots = vec![root.clone()];
        assert_eq!(find(&roots, "t", None).unwrap().manifest.version, "1.10");
        assert_eq!(discover(&roots).modules.len(), 2);
        // a pinned manifest hash: right passes, wrong refused
        let h = sha256_hex(&std::fs::read(src.join("module.json")).unwrap());
        assert!(install_dir(&src, &d.join("root2"), &InstallOptions { sha256: Some(h), ..Default::default() }).is_ok());
        assert!(install_dir(&src, &d.join("root3"), &InstallOptions { sha256: Some("0".repeat(64)), ..Default::default() }).is_err());
        // a changed byte: verify and install go red
        std::fs::write(m.dir.join("layers/senses.tsv"), LAYER.replace("television", "televisiom")).unwrap();
        assert!(verify_dir(&m.dir, &m.manifest).is_err());
        std::fs::write(src.join("layers/senses.tsv"), LAYER.replace("television", "televisiom")).unwrap();
        assert!(install_dir(&src, &d.join("root4"), &InstallOptions::default()).is_err());
        assert!(!d.join("root4/t").exists());
        // http is refused
        assert!(install_url("http://example.org/module.json", &d.join("root5"), &InstallOptions::default()).is_err());
        // remove one version
        remove(&roots, "t", "1.0").unwrap();
        assert_eq!(find(&roots, "t", None).unwrap().manifest.version, "1.10");
        assert!(find(&roots, "t", Some("1.0")).is_err());
    }

    #[test]
    fn nc_needs_acceptance_and_a_repository_is_refused() {
        let d = tmp("nc");
        let src = make_module(&d.join("src"), "r", "1", "nc-research", "CC-BY-NC-4.0");
        assert!(install_dir(&src, &d.join("root"), &InstallOptions::default()).is_err());
        assert!(install_dir(&src, &d.join("root"), &InstallOptions { accept_nc: true, ..Default::default() }).is_ok());
        // a root inside a Git work tree: refused (the negative control: the same root without .git installs)
        let repo = d.join("repo");
        std::fs::create_dir_all(repo.join(".git")).unwrap();
        assert!(inside_git_worktree(&repo.join("data/modules")));
        let src = make_module(&d.join("src"), "p", "1", "public-domain", "CC0-1.0");
        assert!(install_dir(&src, &repo.join("data/modules"), &InstallOptions::default()).is_err());
        assert!(install_dir(&src, &d.join("plain/modules"), &InstallOptions::default()).is_ok());
    }

    #[test]
    fn torrent_without_a_client_prints_instructions() {
        let s = torrent_instructions("magnet:?xt=urn:btih:00", Path::new("/r"));
        assert!(s.contains("aria2c --seed-time=0") && s.contains("mova modules install"));
    }
}
