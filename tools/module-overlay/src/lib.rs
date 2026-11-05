//! Bound external source overlays. No command execution or upstream writes.
//! Stable input trees and an exclusively owned output parent are required;
//! this is a bounded composer, not a publication/licensing or semantic proof.
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

pub const RECEIPT: &str = "mova-overlay-receipt.json";
const MAX_FILES: usize = 100_000;
const MAX_FILE: u64 = 64 * 1024 * 1024;
const MAX_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const MAX_OVERRIDES: usize = 1024;
pub fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FileBinding {
    pub sha256: String,
    pub bytes: u64,
    pub executable: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub schema: u32,
    /// Operator-supplied provenance label, not a verified Git commit.
    pub revision_label: String,
    pub files: BTreeMap<String, FileBinding>,
    pub tree_sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Replace,
    Add,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Overlay {
    pub action: Action,
    pub target: String,
    pub source: String,
    pub sha256: String,
    pub expected_upstream_sha256: Option<String>,
    #[serde(default)]
    pub executable: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    pub base_snapshot: String,
    pub base_snapshot_sha256: String,
    pub files: Vec<Overlay>,
    /// Paths only. The tool emits bounded Cargo argv and never runs them.
    #[serde(default)]
    pub cargo_manifests: Vec<String>,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Drift {
    pub path: String,
    pub expected: Option<FileBinding>,
    pub actual: Option<FileBinding>,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct CheckReport {
    pub schema: u32,
    pub compatible: bool,
    pub base_revision_label: String,
    pub expected_tree_sha256: String,
    pub actual_tree_sha256: String,
    pub drift: Vec<Drift>,
    pub compatibility_claim: String,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Applied {
    pub target: String,
    pub action: Action,
    pub original_sha256: Option<String>,
    pub source: String,
    pub content_sha256: String,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Receipt {
    pub schema: u32,
    pub base_revision_label: String,
    pub base_tree_sha256: String,
    pub manifest_canonical_sha256: String,
    pub output_tree_sha256: String,
    pub applied: Vec<Applied>,
    pub source_models: BTreeMap<String, String>,
    pub cargo_argv: Vec<Vec<String>>,
    pub compatibility_claim: String,
}

fn digest_valid(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn excluded(name: &str) -> bool {
    matches!(name, ".git" | "target" | ".env" | "local.md") || name == RECEIPT
}
fn relative(text: &str) -> Result<PathBuf> {
    ensure!(
        !text.is_empty()
            && text.len() <= 4096
            && !text.contains(['\\', ':'])
            && !text.chars().any(char::is_control),
        "unsafe relative path: {text}"
    );
    ensure!(
        text.split('/').all(|s| !s.is_empty()
            && s != "."
            && s != ".."
            && !excluded(s)
            && s != "peta"),
        "reserved/noncanonical path: {text}"
    );
    let path = Path::new(text);
    ensure!(
        !path.is_absolute() && path.components().all(|c| matches!(c, Component::Normal(_))),
        "relative path required"
    );
    ensure!(text != RECEIPT, "receipt path is reserved");
    Ok(path.into())
}
fn absolute(path: &Path) -> Result<PathBuf> {
    let full = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    // Normalize '.' introduced by ordinary CLI paths, but reject '..'.
    let mut normal = PathBuf::new();
    for part in full.components() {
        match part {
            Component::RootDir | Component::Normal(_) => normal.push(part.as_os_str()),
            Component::CurDir => {}
            _ => bail!("parent/prefix component refused"),
        }
    }
    Ok(normal)
}
fn no_symlink(path: &Path) -> Result<()> {
    let mut at = PathBuf::new();
    for part in absolute(path)?.components() {
        at.push(part.as_os_str());
        ensure!(
            !fs::symlink_metadata(&at)?.file_type().is_symlink(),
            "symlink refused: {}",
            at.display()
        );
    }
    Ok(())
}
fn root(path: &Path) -> Result<PathBuf> {
    let full = absolute(path)?;
    no_symlink(&full)?;
    ensure!(fs::symlink_metadata(&full)?.is_dir(), "directory required");
    Ok(full)
}
fn bounded_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(MAX_FILE + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= MAX_FILE, "file grew over byte bound");
    Ok(bytes)
}
fn read(root: &Path, rel: &str) -> Result<Vec<u8>> {
    let path = root.join(relative(rel)?);
    no_symlink(&path)?;
    let meta = fs::symlink_metadata(&path)?;
    ensure!(
        meta.is_file() && meta.len() <= MAX_FILE,
        "regular bounded file required: {rel}"
    );
    let bytes = bounded_bytes(&path)?;
    ensure!(bytes.len() as u64 <= MAX_FILE, "file grew over bound");
    Ok(bytes)
}
fn executable(meta: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        meta.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        let _ = meta;
        false
    }
}
fn tree_hash(files: &BTreeMap<String, FileBinding>) -> Result<String> {
    Ok(sha(&serde_json::to_vec(files)?))
}
fn walk(
    base: &Path,
    dir: &Path,
    files: &mut BTreeMap<String, FileBinding>,
    total: &mut u64,
    nodes: &mut usize,
    depth: usize,
) -> Result<()> {
    ensure!(depth <= 64, "tree depth bound exceeded");
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        *nodes += 1;
        ensure!(*nodes <= 200_000, "tree entry bound exceeded");
        entries.push(entry?);
    }
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("non-UTF8 path refused"))?;
        if excluded(&name) {
            continue;
        }
        ensure!(
            name != "peta",
            "private peta subtree refused; use an approved public source export"
        );
        let path = entry.path();
        let rel = path
            .strip_prefix(base)?
            .to_str()
            .context("UTF8 relative path")?
            .replace(std::path::MAIN_SEPARATOR, "/");
        relative(&rel)?;
        let meta = fs::symlink_metadata(&path)?;
        ensure!(!meta.file_type().is_symlink(), "symlink refused: {rel}");
        if meta.is_dir() {
            walk(base, &path, files, total, nodes, depth + 1)?;
        } else {
            ensure!(meta.is_file(), "special file refused: {rel}");
            ensure!(files.len() < MAX_FILES, "too many files");
            ensure!(meta.len() <= MAX_FILE, "file exceeds bound");
            let bytes = bounded_bytes(&path)?;
            *total = total
                .checked_add(bytes.len() as u64)
                .context("byte counter overflow")?;
            ensure!(
                *total <= MAX_BYTES && bytes.len() as u64 <= MAX_FILE,
                "tree/file byte bound exceeded"
            );
            files.insert(
                rel,
                FileBinding {
                    sha256: sha(&bytes),
                    bytes: bytes.len() as u64,
                    executable: executable(&meta),
                },
            );
        }
    }
    Ok(())
}
pub fn snapshot(upstream: &Path, revision_label: &str) -> Result<Snapshot> {
    ensure!(
        !revision_label.is_empty()
            && revision_label.len() <= 128
            && !revision_label.chars().any(char::is_control),
        "bounded revision label required"
    );
    let upstream = root(upstream)?;
    let mut files = BTreeMap::new();
    walk(&upstream, &upstream, &mut files, &mut 0, &mut 0, 0)?;
    Ok(Snapshot {
        schema: 1,
        revision_label: revision_label.into(),
        tree_sha256: tree_hash(&files)?,
        files,
    })
}
pub fn save_snapshot(upstream: &Path, revision: &str, output: &Path) -> Result<()> {
    let upstream = root(upstream)?;
    let output = absolute(output)?;
    ensure!(
        !output.starts_with(&upstream),
        "snapshot output must be outside upstream"
    );
    write_new_json(&output, &snapshot(&upstream, revision)?)
}
fn write_new_json<T: Serialize>(output: &Path, value: &T) -> Result<()> {
    let output = absolute(output)?;
    no_symlink(output.parent().context("output parent")?)?;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut f, value)?;
    f.write_all(b"\n")?;
    Ok(())
}
pub fn load(custom: &Path, manifest_rel: &str) -> Result<(Manifest, Snapshot)> {
    let custom = root(custom)?;
    let bytes = read(&custom, manifest_rel)?;
    let manifest: Manifest = serde_json::from_slice(&bytes)?;
    ensure!(
        manifest.schema == 1 && manifest.files.len() <= MAX_OVERRIDES,
        "unsupported/oversized manifest"
    );
    let bytes = read(&custom, &manifest.base_snapshot)?;
    ensure!(
        digest_valid(&manifest.base_snapshot_sha256)
            && sha(&bytes) == manifest.base_snapshot_sha256,
        "base snapshot content binding mismatch"
    );
    let expected: Snapshot = serde_json::from_slice(&bytes)?;
    validate_snapshot(&expected)?;
    Ok((manifest, expected))
}
fn validate_snapshot(value: &Snapshot) -> Result<()> {
    ensure!(
        !value.revision_label.is_empty()
            && value.revision_label.len() <= 128
            && !value.revision_label.chars().any(char::is_control),
        "bounded revision label required"
    );
    ensure!(
        value.schema == 1 && value.files.len() <= MAX_FILES,
        "unsupported/oversized snapshot"
    );
    let mut bytes = 0u64;
    for (path, binding) in &value.files {
        relative(path)?;
        ensure!(
            digest_valid(&binding.sha256) && binding.bytes <= MAX_FILE,
            "invalid source binding"
        );
        bytes = bytes
            .checked_add(binding.bytes)
            .context("byte counter overflow")?;
    }
    ensure!(
        bytes <= MAX_BYTES && tree_hash(&value.files)? == value.tree_sha256,
        "invalid snapshot tree binding"
    );
    Ok(())
}
fn independent(upstream: &Path, custom: &Path) -> Result<(PathBuf, PathBuf)> {
    let upstream = root(upstream)?;
    let custom = root(custom)?;
    ensure!(
        !upstream.starts_with(&custom) && !custom.starts_with(&upstream),
        "upstream and customization roots must be disjoint"
    );
    Ok((upstream, custom))
}
fn planned(
    custom: &Path,
    manifest: &Manifest,
    expected: &Snapshot,
) -> Result<BTreeMap<String, FileBinding>> {
    ensure!(
        manifest.schema == 1
            && manifest.files.len() <= MAX_OVERRIDES
            && manifest.cargo_manifests.len() <= MAX_OVERRIDES,
        "manifest bound/schema"
    );
    let bound = read(custom, &manifest.base_snapshot)?;
    ensure!(
        digest_valid(&manifest.base_snapshot_sha256)
            && sha(&bound) == manifest.base_snapshot_sha256,
        "base snapshot content binding mismatch"
    );
    ensure!(
        serde_json::from_slice::<Snapshot>(&bound)? == *expected,
        "supplied snapshot differs from bound snapshot"
    );
    let mut files = expected.files.clone();
    let mut targets = BTreeSet::new();
    for overlay in &manifest.files {
        relative(&overlay.target)?;
        ensure!(
            targets.insert(overlay.target.clone()),
            "duplicate overlay target: {}",
            overlay.target
        );
        ensure!(digest_valid(&overlay.sha256), "invalid overlay digest");
        let bytes = read(custom, &overlay.source)?;
        ensure!(
            sha(&bytes) == overlay.sha256,
            "custom content binding mismatch: {}",
            overlay.source
        );
        match (
            &overlay.action,
            expected.files.get(&overlay.target),
            &overlay.expected_upstream_sha256,
        ) {
            (Action::Replace, Some(base), Some(hash)) if hash == &base.sha256 => {}
            (Action::Add, None, None) => {}
            _ => bail!("collision/action/base binding mismatch: {}", overlay.target),
        }
        files.insert(
            overlay.target.clone(),
            FileBinding {
                sha256: overlay.sha256.clone(),
                bytes: bytes.len() as u64,
                executable: overlay.executable,
            },
        );
    }
    ensure!(
        files.len() <= MAX_FILES && files.values().map(|b| b.bytes).sum::<u64>() <= MAX_BYTES,
        "composed tree bound exceeded"
    );
    for path in files.keys() {
        let mut parent = Path::new(path).parent();
        while let Some(p) = parent.filter(|p| !p.as_os_str().is_empty()) {
            ensure!(
                !files.contains_key(p.to_str().context("UTF8 parent")?),
                "file/directory collision at {}",
                p.display()
            );
            parent = p.parent();
        }
    }
    let mut cargo = BTreeSet::new();
    for path in &manifest.cargo_manifests {
        relative(path)?;
        ensure!(
            Path::new(path)
                .file_name()
                .is_some_and(|p| p == "Cargo.toml")
                && files.contains_key(path)
                && cargo.insert(path),
            "invalid/duplicate Cargo manifest recipe"
        );
    }
    Ok(files)
}
const CLAIM: &str = "Exact declared bytes and base-tree compatibility only; no API, semantic, licensing, model-registration or Cargo-check success is inferred.";
pub fn check(
    upstream: &Path,
    custom: &Path,
    manifest: &Manifest,
    expected: &Snapshot,
) -> Result<CheckReport> {
    validate_snapshot(expected)?;
    let (upstream, custom) = independent(upstream, custom)?;
    planned(&custom, manifest, expected)?;
    let actual = snapshot(&upstream, &expected.revision_label)?;
    let paths: BTreeSet<_> = expected.files.keys().chain(actual.files.keys()).collect();
    let drift = paths
        .into_iter()
        .filter_map(|path| {
            let before = expected.files.get(path);
            let after = actual.files.get(path);
            (before != after).then(|| Drift {
                path: path.clone(),
                expected: before.cloned(),
                actual: after.cloned(),
            })
        })
        .collect::<Vec<_>>();
    Ok(CheckReport {
        schema: 1,
        compatible: drift.is_empty(),
        base_revision_label: expected.revision_label.clone(),
        expected_tree_sha256: expected.tree_sha256.clone(),
        actual_tree_sha256: actual.tree_sha256,
        drift,
        compatibility_claim: CLAIM.into(),
    })
}
fn write_bound(output: &Path, rel: &str, bytes: &[u8], binding: &FileBinding) -> Result<()> {
    ensure!(
        sha(bytes) == binding.sha256 && bytes.len() as u64 == binding.bytes,
        "source changed while composing: {rel}"
    );
    let path = output.join(relative(rel)?);
    fs::create_dir_all(path.parent().context("file parent")?)?;
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)?;
    f.write_all(bytes)?;
    #[cfg(unix)]
    f.set_permissions(fs::Permissions::from_mode(if binding.executable {
        0o755
    } else {
        0o644
    }))?;
    Ok(())
}
pub fn stage(
    upstream: &Path,
    custom: &Path,
    manifest: &Manifest,
    expected: &Snapshot,
    output: &Path,
) -> Result<Receipt> {
    let (upstream, custom) = independent(upstream, custom)?;
    let output = absolute(output)?;
    let parent = root(output.parent().context("output parent")?)?;
    ensure!(
        !output.starts_with(&upstream)
            && !output.starts_with(&custom)
            && !upstream.starts_with(&output)
            && !custom.starts_with(&output),
        "output must be outside input roots"
    );
    ensure!(
        parent == output.parent().unwrap(),
        "physical output parent required"
    );
    ensure!(
        !output.exists()
            && fs::symlink_metadata(&output)
                .is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound),
        "fresh output required"
    );
    let report = check(&upstream, &custom, manifest, expected)?;
    ensure!(
        report.compatible,
        "upstream drift in {} paths; run check and review a new base before staging",
        report.drift.len()
    );
    let files = planned(&custom, manifest, expected)?;
    fs::create_dir(&output)?;
    let result = (|| {
        let replacements: BTreeMap<_, _> = manifest
            .files
            .iter()
            .map(|o| (o.target.as_str(), o))
            .collect();
        let mut applied = Vec::new();
        for (path, binding) in &files {
            let bytes = if let Some(overlay) = replacements.get(path.as_str()) {
                applied.push(Applied {
                    target: path.clone(),
                    action: overlay.action.clone(),
                    original_sha256: expected.files.get(path).map(|f| f.sha256.clone()),
                    source: overlay.source.clone(),
                    content_sha256: overlay.sha256.clone(),
                });
                read(&custom, &overlay.source)?
            } else {
                read(&upstream, path)?
            };
            write_bound(&output, path, &bytes, binding)?;
        }
        let mut models = BTreeMap::new();
        for (path, binding) in files.iter().filter(|(p, _)| p.ends_with(".rs")) {
            let side = format!("{path}.groupant/manifest.json");
            let status = match files.get(&side) {
                None => "missing",
                Some(_) => {
                    let v = serde_json::from_slice::<serde_json::Value>(&read(&output, &side)?);
                    match v.ok().and_then(|v| {
                        v.pointer("/source/sha256")
                            .and_then(|s| s.as_str())
                            .map(str::to_owned)
                    }) {
                        Some(h) if h == binding.sha256 => "hash_matches_model_semantics_unverified",
                        Some(_) => "stale_source_binding",
                        None => "invalid_or_missing_source_binding",
                    }
                }
            };
            models.insert(path.clone(), status.into());
        }
        // Check inputs again; no live filesystem snapshot/lock is claimed.
        ensure!(
            check(&upstream, &custom, manifest, expected)?.compatible,
            "upstream changed during composition"
        );
        let mut cargo_paths = manifest.cargo_manifests.clone();
        cargo_paths.sort();
        let cargo_argv = cargo_paths
            .iter()
            .flat_map(|p| {
                ["check", "build"].map(|verb| {
                    vec![
                        "cargo".into(),
                        verb.into(),
                        "--manifest-path".into(),
                        p.clone(),
                        "--jobs".into(),
                        "2".into(),
                    ]
                })
            })
            .collect();
        let receipt = Receipt {
            schema: 1,
            base_revision_label: expected.revision_label.clone(),
            base_tree_sha256: expected.tree_sha256.clone(),
            manifest_canonical_sha256: sha(&serde_json::to_vec(manifest)?),
            output_tree_sha256: tree_hash(&files)?,
            applied,
            source_models: models,
            cargo_argv,
            compatibility_claim: CLAIM.into(),
        };
        write_new_json(&output.join(RECEIPT), &receipt)?;
        Ok(receipt)
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(&output);
    }
    result
}

#[cfg(test)]
mod tests;
