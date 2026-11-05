//! Inventory of forward/inverse file contracts and their abstraction tree.
//! A registered model is a declaration, not proof or test execution evidence.
//! Runtime measurements and code/model correspondence remain separate checks.
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Component, Path},
    process::Command,
};

pub mod syntax;

pub fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub schema: u32,
    pub root: String,
    pub nodes: Vec<Node>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    pub id: String,
    #[serde(default)]
    pub source: Option<String>,
    /// Optional repository-relative directory for a group manifest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupant_path: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,
    pub description: String,
    pub state: String,
    pub cost: String,
    pub forward: Vec<String>,
    pub inverse: Vec<String>,
    pub checks: Vec<String>,
    pub limitations: Vec<String>,
}
fn relative(path: &str) -> bool {
    !path.is_empty()
        && Path::new(path)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
pub fn validate(r: &Registry) -> Result<()> {
    ensure!(
        r.schema == 1 && !r.nodes.is_empty(),
        "registry schema or nodes missing"
    );
    let mut ids = BTreeMap::new();
    let mut sources = BTreeSet::new();
    for n in &r.nodes {
        ensure!(
            !n.id.trim().is_empty() && ids.insert(n.id.as_str(), n).is_none(),
            "duplicate or empty model ID"
        );
        ensure!(
            !n.description.trim().is_empty()
                && !n.state.trim().is_empty()
                && !n.cost.trim().is_empty()
                && !n.forward.is_empty()
                && !n.inverse.is_empty()
                && !n.checks.is_empty()
                && !n.limitations.is_empty(),
            "model {} needs description, state, both directions, checks and limits",
            n.id
        );
        ensure!(
            n.forward
                .iter()
                .chain(&n.inverse)
                .chain(&n.checks)
                .chain(&n.limitations)
                .all(|s| !s.trim().is_empty()),
            "empty model clause"
        );
        if let Some(path) = &n.groupant_path {
            ensure!(
                n.source.is_none() && relative(path) && path.ends_with(".groupant"),
                "groupant_path must be a relative group directory on a group node"
            );
        }
        if let Some(p) = &n.source {
            ensure!(
                relative(p) && p.ends_with(".rs") && sources.insert(p.as_str()),
                "invalid or duplicate Rust source"
            );
            ensure!(n.children.is_empty(), "file model cannot own child files");
        } else {
            ensure!(!n.children.is_empty(), "group model needs children");
        }
    }
    ensure!(ids.contains_key(r.root.as_str()), "root model missing");
    let mut parent = BTreeMap::new();
    for n in &r.nodes {
        for c in &n.children {
            ensure!(
                ids.contains_key(c.as_str()) && parent.insert(c.as_str(), n.id.as_str()).is_none(),
                "unknown or multiply owned model"
            );
        }
    }
    ensure!(
        !parent.contains_key(r.root.as_str()),
        "root model has a parent"
    );
    let mut reached = BTreeSet::new();
    let mut stack = vec![r.root.as_str()];
    while let Some(id) = stack.pop() {
        ensure!(reached.insert(id), "model tree contains a cycle");
        stack.extend(ids[id].children.iter().map(String::as_str));
    }
    ensure!(
        reached.len() == r.nodes.len(),
        "unreachable model or disconnected cycle"
    );
    Ok(())
}
/// Compose a validated base with explicit overlay nodes into one validated tree.
/// The overlay may refer to base IDs. Neither input is changed or deduplicated.
pub fn compose_registry(base: &Registry, overlay: &Registry) -> Result<Registry> {
    validate(base)?;
    ensure!(overlay.schema == 1, "overlay registry schema must be 1");
    let mut nodes = base.nodes.clone();
    nodes.extend(overlay.nodes.iter().cloned());
    let composed = Registry {
        schema: 1,
        root: overlay.root.clone(),
        nodes,
    };
    validate(&composed)?;
    Ok(composed)
}

pub fn rust_files(root: &Path) -> Result<Vec<String>> {
    let result = Command::new("git")
        .arg("-C")
        .arg(root)
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
            "*.rs",
        ])
        .output()?;
    ensure!(result.status.success(), "cannot inventory repository files");
    let mut paths = BTreeSet::new();
    for bytes in result.stdout.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        let p = std::str::from_utf8(bytes)?.to_string();
        ensure!(relative(&p), "invalid Git source path");
        paths.insert(p);
    }
    Ok(paths.into_iter().collect())
}
/// Membership in the sorted path list returned by `rust_files`.
pub(crate) fn discovered_source(files: &[String], source: &str) -> bool {
    files
        .binary_search_by(|path| path.as_str().cmp(source))
        .is_ok()
}
pub fn inventory(root: &Path, r: &Registry, registry_sha: &str) -> Result<Value> {
    validate(r)?;
    let base = fs::canonicalize(root)?;
    let files = rust_files(root)?;
    let by_source: BTreeMap<_, _> = r
        .nodes
        .iter()
        .filter_map(|n| n.source.as_ref().map(|s| (s.as_str(), n.id.as_str())))
        .collect();
    let actual: BTreeSet<_> = files.iter().map(String::as_str).collect();
    for p in by_source.keys() {
        ensure!(
            actual.contains(p),
            "registered source missing from Git inventory: {p}"
        );
    }
    let mut entries = Vec::new();
    let mut dirs = BTreeMap::<String, (usize, usize)>::new();
    let mut registered = 0;
    for p in &files {
        let canonical = fs::canonicalize(root.join(p))?;
        ensure!(
            canonical.starts_with(&base),
            "source symlink leaves repository"
        );
        let model = by_source.get(p.as_str()).copied();
        registered += usize::from(model.is_some());
        let mut d = Path::new(p).parent();
        while let Some(dir) = d {
            let key = if dir.as_os_str().is_empty() {
                ".".into()
            } else {
                dir.to_string_lossy().into_owned()
            };
            let v = dirs.entry(key).or_default();
            v.0 += 1;
            v.1 += usize::from(model.is_some());
            d = dir.parent();
        }
        entries.push(
            json!({"source":p,"sha256":sha(&fs::read(canonical)?),"model":model,
   "status":if model.is_some(){"registered_partial_model"}else{"model_missing"},
   "verification_inferred":false}),
        );
    }
    let logical: Vec<_> = r
        .nodes
        .iter()
        .map(|n| {
            json!({"id":n.id,"children":n.children,"source":n.source,
  "status":"declared; verification and composition scope require separate evidence"})
        })
        .collect();
    Ok(
        json!({"schema":1,"registry_sha256":registry_sha,"rust_files":files.len(),"registered_files":registered,
  "missing_files":files.len()-registered,"all_rust_files_registered":registered==files.len(),
  "full_system_verification":false,"files":entries,"logical_tree":logical,
  "directory_tree":dirs.into_iter().map(|(path,(total,known))|json!({"directory":path,"files":total,
    "registered":known,"missing":total-known})).collect::<Vec<_>>()}),
    )
}
/// Migration gate: changed/new Rust sources must have a model even while the
/// older repository backlog is explicit. A declaration still is not verification.
pub fn changed_inventory(
    root: &Path,
    r: &Registry,
    registry_sha: &str,
    base: &str,
) -> Result<Value> {
    ensure!(
        !base.is_empty() && !base.starts_with('-'),
        "base revision must not be empty or a Git option"
    );
    let mut report = inventory(root, r, registry_sha)?;
    let mut changed = BTreeSet::new();
    for args in [
        vec![
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--name-only",
            "-z",
            base,
            "--",
            "*.rs",
        ],
        vec![
            "ls-files",
            "-z",
            "--others",
            "--exclude-standard",
            "--",
            "*.rs",
        ],
    ] {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()?;
        ensure!(output.status.success(), "cannot inspect changed Rust files");
        for bytes in output.stdout.split(|b| *b == 0).filter(|b| !b.is_empty()) {
            let p = std::str::from_utf8(bytes)?.to_string();
            ensure!(relative(&p), "invalid changed source path");
            if root.join(&p).is_file() {
                changed.insert(p);
            }
        }
    }
    let modeled: BTreeSet<_> = r.nodes.iter().filter_map(|n| n.source.as_deref()).collect();
    let missing: Vec<_> = changed
        .iter()
        .filter(|p| !modeled.contains(p.as_str()))
        .cloned()
        .collect();
    report["changed_rust_files"] = serde_json::to_value(&changed)?;
    report["missing_changed_files"] = serde_json::to_value(&missing)?;
    report["all_changed_files_registered"] = json!(missing.is_empty());
    Ok(report)
}

pub fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    Ok(())
}
/// Extract a complete finite Lean output table, rejecting partial/error logs.
/// This validates format and coverage, not the truth of the policy in its rows.
pub fn extract_table(log: &str) -> Result<String> {
    ensure!(
        !log.contains("error:")
            && !log.contains("sorryAx")
            && !log.contains("declaration uses 'sorry'"),
        "Lean log contains an error or incomplete proof"
    );
    let header = "# before\tafter\told_before\told_after\trecommend";
    let (_, tail) = log
        .split_once(header)
        .context("Lean table header missing")?;
    let mut seen = BTreeSet::new();
    let mut lines = vec![header.to_string()];
    for line in tail.lines().filter(|s| !s.trim().is_empty()) {
        let c: Vec<_> = line.split('\t').collect();
        ensure!(c.len() == 5, "invalid Lean table row");
        let v = c[..4]
            .iter()
            .map(|s| s.parse::<u8>().map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()?;
        ensure!(
            v.iter().all(|n| *n < 4) && matches!(c[4], "true" | "false") && seen.insert(v),
            "duplicate/out-of-domain Lean table row"
        );
        lines.push(line.into());
    }
    ensure!(seen.len() == 256, "partial Lean table");
    Ok(lines.join("\n") + "\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn leaf() -> Node {
        Node {
            id: "leaf".into(),
            source: Some("src/lib.rs".into()),
            groupant_path: None,
            children: vec![],
            description: "Leaf transformation".into(),
            state: "S".into(),
            cost: "bounded work".into(),
            forward: vec!["S -> S'".into()],
            inverse: vec!["forbidden transitions excluded".into()],
            checks: vec!["unit test".into()],
            limitations: vec!["declaration only".into()],
        }
    }
    fn registry() -> Registry {
        let mut group = leaf();
        group.id = "root".into();
        group.source = None;
        group.children = vec!["leaf".into()];
        Registry {
            schema: 1,
            root: "root".into(),
            nodes: vec![group, leaf()],
        }
    }
    #[test]
    fn hierarchy_requires_both_directions_and_a_single_reachable_tree() {
        let r = registry();
        validate(&r).unwrap();
        let mut bad = r.clone();
        bad.nodes[1].inverse.clear();
        assert!(validate(&bad).is_err());
        bad = r.clone();
        bad.nodes[0].children.push("leaf".into());
        assert!(validate(&bad).is_err());
        bad = r.clone();
        bad.nodes[1].source = None;
        bad.nodes[1].children = vec!["root".into()];
        assert!(validate(&bad).is_err());
        bad = r;
        let mut unowned = leaf();
        unowned.id = "orphan".into();
        unowned.source = Some("orphan.rs".into());
        bad.nodes.push(unowned);
        assert!(validate(&bad).is_err());
    }
    #[test]
    fn lean_bridge_rejects_partial_duplicate_and_failed_output() {
        let mut log = String::from("# before\tafter\told_before\told_after\trecommend\n");
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        log += &format!("{a}\t{b}\t{c}\t{d}\tfalse\n");
                    }
                }
            }
        }
        assert_eq!(extract_table(&log).unwrap().lines().count(), 257);
        assert!(extract_table(&(log.clone() + "0\t0\t0\t0\tfalse\n")).is_err());
        assert!(extract_table(&log.lines().take(12).collect::<Vec<_>>().join("\n")).is_err());
        assert!(extract_table(&("error: failed proof\n".to_string() + &log)).is_err());
    }
    #[test]
    fn changed_gate_rejects_revision_options_before_touching_a_repository() {
        for base in ["", "--output=unwanted-file"] {
            let error = changed_inventory(Path::new("not-a-repository"), &registry(), "sha", base)
                .unwrap_err();
            assert!(error.to_string().contains("base revision"));
        }
    }
}

/// Inspect adjacent, open groupant manifests without executing their check strings.
/// Absence is migration backlog; a malformed declared artifact is an error.
pub fn groupants(root: &Path, r: &Registry, registry_sha: &str) -> Result<Value> {
    validate(r)?;
    // Registry IDs become filenames only in groupant discovery.
    for node in &r.nodes {
        ensure!(
            safe_groupant_id(&node.id),
            "unsafe groupant registry ID: {}",
            node.id
        );
    }
    let root = fs::canonicalize(root)?;
    let files = rust_files(&root)?;
    let by_source: BTreeMap<_, _> = r
        .nodes
        .iter()
        .filter_map(|n| n.source.as_ref().map(|s| (s.as_str(), n.id.as_str())))
        .collect();
    let mut diagnostics = Vec::<String>::new();
    let mut entries = Vec::new();
    let mut ids = BTreeSet::new();
    for source in &files {
        let mut errors = Vec::new();
        let source_path = root.join(source);
        let source_sha = match confined_file(&root, &root, source).and_then(fs::read) {
            Ok(bytes) => Some(sha(&bytes)),
            Err(e) => {
                errors.push(format!("source: {e}"));
                None
            }
        };
        let sidecar = format!("{source}.groupant");
        let dir = root.join(&sidecar);
        let mut manifest = Value::Null;
        let mut coverage = required_family(&[]);
        let mut subgroupants = Vec::new();
        let exists = fs::symlink_metadata(&dir).is_ok();
        let mut regenerated = 0;
        if exists {
            match confined_file(&root, &dir, "manifest.json")
                .and_then(fs::read)
                .map_err(anyhow::Error::from)
                .and_then(|b| serde_json::from_slice::<Value>(&b).map_err(Into::into))
            {
                Ok(value) => {
                    manifest = value;
                    let mut artifacts = syntax::ArtifactReader::new();
                    validate_source_groupant(
                        &root,
                        &dir,
                        &source_path,
                        source_sha.as_deref(),
                        &manifest,
                        "groupant",
                        &mut artifacts,
                        &mut errors,
                    );
                    if let Some(id) = nonempty(&manifest["id"]) {
                        if !ids.insert(id.to_string()) {
                            errors.push(format!("duplicate groupant ID: {id}"));
                        }
                    }
                    let source_text = confined_file(&root, &root, source)
                        .and_then(fs::read_to_string)
                        .ok();
                    let mut fragments = FragmentValidation {
                        root: &root,
                        source: &source_path,
                        source_sha: source_sha.as_deref(),
                        source_text: source_text.as_deref(),
                        ids: &mut ids,
                        reports: &mut subgroupants,
                        artifacts,
                    };
                    fragments.children(&dir, &manifest, 0, None, &mut errors);
                    regenerated = fragments.artifacts.regenerated;
                    if let Some(node) = manifest.get("registry").and_then(|v| v.get("node")) {
                        if node.as_str() != by_source.get(source.as_str()).copied() {
                            errors.push("registry node does not match source ownership".into());
                        }
                    }
                    coverage = required_family(
                        manifest["components"]["obernants"]
                            .as_array()
                            .map(Vec::as_slice)
                            .unwrap_or(&[]),
                    );
                }
                Err(e) => errors.push(format!("manifest: {e}")),
            }
        }
        diagnostics.extend(errors.iter().map(|e| format!("{sidecar}: {e}")));
        entries.push(json!({
            "source":source, "sha256":source_sha, "model":by_source.get(source.as_str()),
            "sidecar":if exists {Some(sidecar)} else {None},
            "status":if !errors.is_empty() {"invalid"} else if exists {"available_partial"} else {"not_materialized"},
            "manifest":manifest, "required_types":coverage, "diagnostics":errors,
            "subgroupants":subgroupants,
            "syntax_projections_regenerated":regenerated,
            "verification_inferred":false
        }));
    }
    for source in by_source.keys() {
        if !discovered_source(&files, source) {
            diagnostics.push(format!(
                "registered source missing from Git inventory: {source}"
            ));
        }
    }
    let nodes: BTreeMap<_, _> = r.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let hashes: BTreeMap<_, _> = entries
        .iter()
        .filter_map(|v| Some((v["source"].as_str()?, v["sha256"].as_str()?)))
        .collect();
    let mut logical = Vec::new();
    let mut group_sidecars = 0;
    let mut missing_group_sidecars = 0;
    for n in nodes.values() {
        if n.source.is_some() {
            logical.push(json!({"id":n.id, "source":n.source, "children":n.children,
                "status":"see_file_entry", "required_types":null, "verification_inferred":false}));
            continue;
        }
        let sidecar = n
            .groupant_path
            .clone()
            .unwrap_or_else(|| format!("models/{}.groupant", n.id));
        let dir = root.join(&sidecar);
        let exists = fs::symlink_metadata(&dir).is_ok();
        let mut errors = Vec::new();
        let mut manifest = Value::Null;
        if exists {
            group_sidecars += 1;
            match confined_file(&root, &dir, "manifest.json")
                .and_then(fs::read)
                .map_err(anyhow::Error::from)
                .and_then(|b| serde_json::from_slice::<Value>(&b).map_err(Into::into))
            {
                Ok(value) => {
                    manifest = value;
                    validate_group_manifest(
                        &root,
                        &dir,
                        n,
                        &nodes,
                        &hashes,
                        &manifest,
                        &mut errors,
                    );
                    if let Some(id) = nonempty(&manifest["id"]) {
                        if !ids.insert(id.to_string()) {
                            errors.push(format!("duplicate groupant ID: {id}"));
                        }
                    }
                }
                Err(e) => errors.push(format!("manifest: {e}")),
            }
        } else {
            missing_group_sidecars += 1;
        }
        let coverage = required_family(
            manifest["components"]["obernants"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        );
        diagnostics.extend(errors.iter().map(|e| format!("{sidecar}: {e}")));
        logical.push(json!({"id":n.id, "source":n.source, "children":n.children,
            "sidecar":if exists {Some(sidecar)} else {None},
            "status":if !errors.is_empty() {"invalid"} else if exists {"available_partial"} else {"not_materialized"},
            "manifest":manifest, "diagnostics":errors, "required_types":coverage,
            "verification_inferred":false}));
    }
    Ok(
        json!({"schema":1,"registry_sha256":registry_sha,"rust_files":files.len(),
        "group_sidecars":group_sidecars,"missing_group_sidecars":missing_group_sidecars,
        "subgroupants":entries.iter().map(|v| v["subgroupants"].as_array().map_or(0, Vec::len)).sum::<usize>(),
        "sidecars":entries.iter().filter(|v| !v["sidecar"].is_null()).count(),
        "syntax_projections_regenerated":entries.iter().map(|v| v["syntax_projections_regenerated"].as_u64().unwrap_or(0)).sum::<u64>(),
        "unregistered_files":files.iter().filter(|p| !by_source.contains_key(p.as_str())).count(),
        "missing_sidecars":entries.iter().filter(|v| v["sidecar"].is_null()).count(),
        "valid":diagnostics.is_empty(),"diagnostics":diagnostics,"files":entries,
        "logical_tree":logical,"verification_inferred":false,"full_system_verification":false}),
    )
}

// Local decomposition does not change the inter-file registry tree.
const MAX_FRAGMENT_DEPTH: usize = 64;
struct FragmentValidation<'a> {
    root: &'a Path,
    source: &'a Path,
    source_sha: Option<&'a str>,
    source_text: Option<&'a str>,
    ids: &'a mut BTreeSet<String>,
    reports: &'a mut Vec<Value>,
    artifacts: syntax::ArtifactReader,
}
impl FragmentValidation<'_> {
    fn children(
        &mut self,
        dir: &Path,
        parent: &Value,
        depth: usize,
        ancestor_range: Option<(u64, u64)>,
        errors: &mut Vec<String>,
    ) {
        let Some(declarations) = parent.get("children") else {
            return;
        };
        let Some(children) = declarations.as_array() else {
            errors.push("children must be an array of {id, path} records".into());
            return;
        };
        if children.is_empty() {
            return;
        }
        let composition = &parent["components"]["composition"];
        if !composition.is_object()
            || (nonempty(&composition["description"]).is_none()
                && nonempty(&composition["path"]).is_none())
        {
            errors.push("nonempty children require components.composition with a description or path artifact".into());
        }
        // validate_components already checks every declared composition record.
        if depth >= MAX_FRAGMENT_DEPTH {
            errors.push(format!(
                "nested groupant depth exceeds {MAX_FRAGMENT_DEPTH}"
            ));
            return;
        }
        let mut paths = BTreeSet::new();
        for declaration in children {
            let mut child_errors = Vec::new();
            let id = nonempty(&declaration["id"]);
            match id {
                Some(id) if self.ids.insert(id.to_string()) => {}
                Some(id) => child_errors.push(format!("duplicate groupant ID: {id}")),
                None => child_errors.push("child declaration needs a nonempty id".into()),
            }
            let name = nonempty(&declaration["path"]).unwrap_or("");
            let child_dir = dir.join(name);
            let mut manifest = Value::Null;
            let loaded = (|| -> Result<Value> {
                ensure!(
                    !name.is_empty()
                        && name.ends_with(".groupant")
                        && Path::new(name).components().count() == 1
                        && matches!(
                            Path::new(name).components().next(),
                            Some(std::path::Component::Normal(_))
                        )
                        && Path::new(name).file_name().and_then(|p| p.to_str()) == Some(name),
                    "child path must be one normal component ending .groupant"
                );
                ensure!(paths.insert(name), "duplicate child path: {name}");
                let metadata = fs::symlink_metadata(&child_dir)?;
                ensure!(
                    metadata.is_dir() && !metadata.file_type().is_symlink(),
                    "child directory must be a real directory, not a symlink"
                );
                let canonical_parent = fs::canonicalize(dir)?;
                let canonical_child = fs::canonicalize(&child_dir)?;
                ensure!(
                    canonical_child.starts_with(self.root)
                        && canonical_child.parent() == Some(canonical_parent.as_path()),
                    "child directory must remain directly inside its parent"
                );
                let path = child_dir.join("manifest.json");
                ensure!(
                    fs::symlink_metadata(&path)?.file_type().is_file(),
                    "child manifest must be a regular file, not a symlink"
                );
                let path = confined_file(&canonical_child, &canonical_child, "manifest.json")?;
                Ok(serde_json::from_slice(&fs::read(path)?)?)
            })();
            let slot = self.reports.len();
            self.reports.push(Value::Null);
            match loaded {
                Ok(value) => {
                    manifest = value;
                    validate_source_groupant(
                        self.root,
                        &child_dir,
                        self.source,
                        self.source_sha,
                        &manifest,
                        "groupant_fragment",
                        &mut self.artifacts,
                        &mut child_errors,
                    );
                    if nonempty(&manifest["id"]) != id {
                        child_errors.push("child manifest ID must match its declaration".into());
                        if let Some(actual) = nonempty(&manifest["id"]) {
                            if !self.ids.insert(actual.to_string()) {
                                child_errors.push(format!("duplicate groupant ID: {actual}"));
                            }
                        }
                    }
                    let range = self.range(&manifest, ancestor_range, &mut child_errors);
                    if let Err(e) = self.syntax_anchor(&child_dir, &manifest, range) {
                        child_errors.push(format!("coverage.syntax_nodes: {e}"));
                    }
                    self.children(
                        &child_dir,
                        &manifest,
                        depth + 1,
                        range.or(ancestor_range),
                        &mut child_errors,
                    );
                }
                Err(e) => child_errors.push(format!("child manifest: {e}")),
            }
            if !child_errors.is_empty() {
                errors.push(format!(
                    "child {}: {}",
                    id.unwrap_or("<missing>"),
                    child_errors.join("; ")
                ));
            }
            self.reports[slot] = json!({
                "parent":parent["id"], "id":declaration["id"], "depth":depth + 1,
                "sidecar":child_dir.strip_prefix(self.root).ok().map(|p| p.to_string_lossy()),
                "manifest":manifest, "diagnostics":child_errors,
                "status":if child_errors.is_empty() {"available_partial"} else {"invalid"},
                "required_types":required_family(manifest["components"]["obernants"].as_array().map(Vec::as_slice).unwrap_or(&[])),
                "verification_inferred":false
            });
        }
    }
    // Model ancestry can cross-cut syntax ancestry; only source and bounds bind them.
    fn syntax_anchor(
        &mut self,
        dir: &Path,
        manifest: &Value,
        range: Option<(u64, u64)>,
    ) -> Result<()> {
        let Some(anchor) = manifest["coverage"].get("syntax_nodes") else {
            return Ok(());
        };
        let path = nonempty(&anchor["path"]).context("requires a projection path")?;
        let projection = self.artifacts.projection(self.root, dir, path, self.source)?;
        syntax::validate_artifact(self.root, self.source, self.source_sha, &projection)?;
        let ids = anchor["node_ids"]
            .as_array()
            .context("node_ids must be a nonempty array of distinct unsigned integers")?;
        ensure!(!ids.is_empty(), "node_ids must be nonempty");
        let nodes = projection["nodes"]
            .as_array()
            .context("projection nodes missing")?;
        let mut selected = BTreeSet::new();
        let (mut start, mut end) = (u64::MAX, 0);
        for id in ids {
            let id = id
                .as_u64()
                .context("node_ids must contain unsigned integers")?;
            ensure!(selected.insert(id), "duplicate node ID: {id}");
            let node = usize::try_from(id)
                .ok()
                .and_then(|id| nodes.get(id))
                .with_context(|| format!("node ID does not exist: {id}"))?;
            // validate_artifact established contiguous IDs and valid source ranges.
            start = start.min(node["start"].as_u64().context("node start missing")?);
            end = end.max(node["end"].as_u64().context("node end missing")?);
        }
        ensure!(
            range == Some((start, end)),
            "coverage.byte_range must equal the minimal enclosing syntax node span [{start},{end})"
        );
        Ok(())
    }
    fn range(
        &self,
        m: &Value,
        ancestor: Option<(u64, u64)>,
        errors: &mut Vec<String>,
    ) -> Option<(u64, u64)> {
        if nonempty(&m["coverage"]["description"]).is_none() {
            errors.push("fragment coverage.description must be nonempty text".into());
        }
        let Some(range) = m["coverage"].get("byte_range") else {
            if ancestor.is_some() {
                errors.push(
                    "fragment under a ranged ancestor must declare coverage.byte_range".into(),
                );
            }
            return None;
        };
        let bounds = range["start"].as_u64().zip(range["end"].as_u64());
        let valid = bounds.is_some_and(|(start, end)| {
            start < end
                && self.source_text.is_some_and(|text| {
                    end <= text.len() as u64
                        && text.is_char_boundary(start as usize)
                        && text.is_char_boundary(end as usize)
                })
                && ancestor.is_none_or(|(a, b)| a <= start && end <= b)
        });
        if !valid {
            errors.push("coverage.byte_range must be a nonempty UTF-8 half-open range within the source and every ranged ancestor".into());
            return None;
        }
        bounds
    }
}

fn safe_groupant_id(id: &str) -> bool {
    !id.is_empty()
        && id != "."
        && id != ".."
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}

fn validate_group_manifest(
    root: &Path,
    dir: &Path,
    node: &Node,
    nodes: &BTreeMap<&str, &Node>,
    hashes: &BTreeMap<&str, &str>,
    m: &Value,
    errors: &mut Vec<String>,
) {
    if !m.is_object()
        || m["schema"] != 1
        || m["kind"] != "groupant_group"
        || m["id"].as_str() != Some(node.id.as_str())
        || nonempty(&m["status"]).is_none()
    {
        errors.push(
            "group manifest needs schema 1, kind groupant_group, registry ID and nonempty status"
                .into(),
        );
    }
    let expected_children: BTreeSet<_> = node.children.iter().map(String::as_str).collect();
    match m["children"].as_array() {
        Some(children) => {
            let declared: BTreeSet<_> = children.iter().filter_map(Value::as_str).collect();
            if declared != expected_children || declared.len() != children.len() {
                errors.push(
                    "children must name exactly the immediate registry children without duplicates"
                        .into(),
                );
            }
        }
        None => errors.push("children must be an array".into()),
    }
    let mut expected = BTreeSet::new();
    let mut stack = vec![node];
    while let Some(descendant) = stack.pop() {
        if let Some(source) = &descendant.source {
            expected.insert(source.as_str());
        }
        stack.extend(descendant.children.iter().map(|id| nodes[id.as_str()]));
    }
    let mut declared = BTreeSet::new();
    match m["source_bindings"].as_array() {
        Some(bindings) => {
            for binding in bindings {
                let Some(path) = nonempty(&binding["path"]) else {
                    errors.push("source binding needs a repository-relative path".into());
                    continue;
                };
                if !relative(path) || !expected.contains(path) || !declared.insert(path) {
                    errors.push(format!(
                        "unexpected, unsafe or duplicate source binding: {path}"
                    ));
                }
                // Hashes were obtained from confined actual Git sources above.
                if hashes.get(path).copied().is_none()
                    || nonempty(&binding["sha256"]) != hashes.get(path).copied()
                {
                    errors.push(format!(
                        "source binding SHA-256 is missing or stale: {path}"
                    ));
                }
            }
        }
        None => errors.push("source_bindings must be an array".into()),
    }
    for path in expected.difference(&declared) {
        errors.push(format!("missing transitive source binding: {path}"));
    }
    if let Some(owner) = m.get("registry").and_then(|v| v.get("node")) {
        if owner.as_str() != Some(node.id.as_str()) {
            errors.push("registry node does not match group ownership".into());
        }
    }
    validate_components(root, dir, m, errors);
}

// Canonicalization permits sidecar-relative ../ references but rejects escapes,
// absolute paths and non-files, including symlinks pointing outside the repo.
fn confined_file(root: &Path, base: &Path, name: &str) -> std::io::Result<std::path::PathBuf> {
    if name.trim().is_empty() || Path::new(name).is_absolute() {
        return Err(std::io::Error::other("expected a relative file reference"));
    }
    let path = fs::canonicalize(base.join(name))?;
    if !path.starts_with(root) || !path.is_file() {
        return Err(std::io::Error::other(
            "reference is not a file inside the repository",
        ));
    }
    Ok(path)
}
fn nonempty(v: &Value) -> Option<&str> {
    v.as_str().filter(|s| !s.trim().is_empty())
}
fn required_family(entries: &[Value]) -> Value {
    json!(
        ["behavior_complement", "reverse_reconstruction"].map(|kind| {
            let matches: Vec<_> = entries.iter().filter(|v| v["type"] == kind).collect();
            json!({"type":kind,"status":if matches.is_empty() {"missing_record"}
            else if matches.iter().all(|v| v["status"] == "not_materialized") {"not_materialized"}
            else {"declared_requires_review"},
            "ids":matches.iter().filter_map(|v| v["id"].as_str()).collect::<Vec<_>>()})
        })
    )
}
#[cfg(test)]
fn validate_groupant(
    root: &Path,
    dir: &Path,
    source: &Path,
    source_sha: Option<&str>,
    m: &Value,
    errors: &mut Vec<String>,
) {
    validate_source_groupant(
        root,
        dir,
        source,
        source_sha,
        m,
        "groupant",
        &mut syntax::ArtifactReader::new(),
        errors,
    );
}
fn validate_source_groupant(
    root: &Path,
    dir: &Path,
    source: &Path,
    source_sha: Option<&str>,
    m: &Value,
    kind: &str,
    artifacts: &mut syntax::ArtifactReader,
    errors: &mut Vec<String>,
) {
    if !m.is_object()
        || m["schema"] != 1
        || m["kind"] != kind
        || nonempty(&m["id"]).is_none()
        || nonempty(&m["status"]).is_none()
    {
        errors.push(format!(
            "manifest needs schema 1, kind {kind}, nonempty id and status"
        ));
    }
    let subject = nonempty(&m["source"]["path"]).and_then(|p| confined_file(root, dir, p).ok());
    if subject.as_deref() != fs::canonicalize(source).ok().as_deref() {
        errors
            .push("source path must identify the adjacent Rust file inside the repository".into());
    }
    if source_sha.is_none() || nonempty(&m["source"]["sha256"]) != source_sha {
        errors.push("source SHA-256 is missing or stale".into());
    }
    validate_components(root, dir, m, errors);
    let ast = &m["components"]["ast"];
    if ast["format"] == "rust_syntax_projection" {
        let checked = (|| -> Result<()> {
            let path = nonempty(&ast["path"]).context("syntax projection requires path")?;
            let value = artifacts.projection(root, dir, path, source)?;
            syntax::validate_artifact(root, source, source_sha, &value)
        })();
        if let Err(e) = checked {
            errors.push(format!("ast rust_syntax_projection: {e}"));
        }
    }
}
fn validate_components(root: &Path, dir: &Path, m: &Value, errors: &mut Vec<String>) {
    let Some(components) = m["components"].as_object() else {
        errors.push("components must be an object".into());
        return;
    };
    for name in ["ast", "approximant", "invariant", "descriptant"] {
        if !components.contains_key(name) {
            errors.push(format!("missing core component record: {name}"));
        }
    }
    if let Some(attachments) = components.get("rust_approximants") {
        validate_rust_approximants(root, dir, m, attachments, errors);
    }
    for (name, component) in components {
        if name == "obernants" || name == "rust_approximants" {
            continue;
        }
        validate_component(root, dir, name, component, errors);
    }
    let Some(family) = components.get("obernants").and_then(Value::as_array) else {
        errors.push("components.obernants must be an array".into());
        return;
    };
    let mut ids = BTreeSet::new();
    for (index, member) in family.iter().enumerate() {
        let label = format!("obernants[{index}]");
        validate_component(root, dir, &label, member, errors);
        match nonempty(&member["id"]) {
            Some(id) if ids.insert(id) => {}
            _ => errors.push(format!("{label}: missing or duplicate ID")),
        }
        if nonempty(&member["type"]).is_none() {
            errors.push(format!("{label}: missing type"));
        }
        let status = nonempty(&member["status"]).unwrap_or("");
        if status == "inapplicable" && nonempty(&member["reason"]).is_none() {
            errors.push(format!("{label}: inapplicable requires a reason"));
        }
        if status != "not_materialized"
            && status != "inapplicable"
            && nonempty(&member["path"]).is_none()
            && nonempty(&member["description"]).is_none()
        {
            errors.push(format!(
                "{label}: materialized model needs an artifact reference"
            ));
        }
    }
    for kind in ["behavior_complement", "reverse_reconstruction"] {
        if !family.iter().any(|v| v["type"] == kind) {
            errors.push(format!("missing required obernant record: {kind}"));
        }
    }
    if let Some(registry) = m.get("registry") {
        validate_references(root, dir, "registry", registry, None, errors);
    }
}
// This checks declarations and bindings, never execution, error claims or speed.
fn validate_rust_approximants(
    root: &Path,
    dir: &Path,
    manifest: &Value,
    attachments: &Value,
    errors: &mut Vec<String>,
) {
    let Some(records) = attachments.as_array() else {
        errors.push("rust_approximants must be an array".into());
        return;
    };
    let mut ids = BTreeSet::new();
    for (index, record) in records.iter().enumerate() {
        let label = format!("rust_approximants[{index}]");
        if !record.is_object() {
            errors.push(format!("{label}: expected an object"));
            continue;
        }
        match nonempty(&record["id"]) {
            Some(id) if ids.insert(id) => {}
            _ => errors.push(format!("{label}: missing or duplicate ID")),
        }
        for field in ["metric", "domain", "status"] {
            if nonempty(&record[field]).is_none() {
                errors.push(format!("{label}.{field}: expected nonempty text"));
            }
        }
        if !record["assumptions"]
            .as_array()
            .is_some_and(|items| items.iter().all(|v| nonempty(v).is_some()))
        {
            errors.push(format!(
                "{label}.assumptions: expected an array of nonempty strings"
            ));
        }
        if !matches!(
            record["bound_kind"].as_str(),
            Some("analytical" | "empirical")
        ) {
            errors.push(format!(
                "{label}.bound_kind: expected analytical or empirical"
            ));
        }
        let classification = &record["classification"];
        for (field, allowed) in [
            (
                "metric_kind",
                &[
                    "scalar_relative",
                    "normalized_rank",
                    "objective_loss",
                    "probability_relative",
                ][..],
            ),
            (
                "method_kind",
                &[
                    "quantized_numeric",
                    "certified_sample",
                    "relaxation_certificate",
                    "floating_normalization",
                    "interval_certificate",
                ][..],
            ),
            (
                "assurance_kind",
                &["analytical", "per_input_certificate", "finite_observation"][..],
            ),
        ] {
            if !classification[field]
                .as_str()
                .is_some_and(|value| allowed.contains(&value))
            {
                errors.push(format!(
                    "{label}.classification.{field}: expected one of {}",
                    allowed.join(", ")
                ));
            }
        }
        if !matches!(
            (
                record["bound_kind"].as_str(),
                classification["assurance_kind"].as_str()
            ),
            (Some("empirical"), Some("finite_observation"))
                | (
                    Some("analytical"),
                    Some("analytical" | "per_input_certificate")
                )
        ) {
            errors.push(format!(
                "{label}.classification.assurance_kind: must agree with bound_kind"
            ));
        }
        let budget = record["budget_percent"].as_f64();
        if !budget.is_some_and(|b| matches!(b, 1.0 | 2.0 | 5.0 | 10.0)) {
            errors.push(format!(
                "{label}.budget_percent: only 1, 2, 5, 10 are allowed"
            ));
        }
        if !record["error_percent"].as_f64().is_some_and(|error| {
            error.is_finite() && (0.0..=10.0).contains(&error) && budget.is_some_and(|b| error <= b)
        }) {
            errors.push(format!(
                "{label}.error_percent: expected finite nonnegative error within budget and at most 10 percent"
            ));
        }
        let artifact = validate_hashed_reference(root, dir, record);
        match artifact {
            Ok(path) => {
                let confined = fs::canonicalize(dir).is_ok_and(|sidecar| path.starts_with(sidecar));
                if !confined || path.extension().and_then(|v| v.to_str()) != Some("rs") {
                    errors.push(format!(
                        "{label}.path: Rust artifact must be inside its sidecar"
                    ));
                }
            }
            Err(e) => errors.push(format!("{label}: {e}")),
        }
        if record["source"] != manifest["source"] || !record["source"].is_object() {
            errors.push(format!(
                "{label}.source: must equal the manifest source binding"
            ));
        }
        if let Err(e) = validate_hashed_reference(root, dir, &record["source"]) {
            errors.push(format!("{label}.source: {e}"));
        }
        match record["evidence"]
            .as_array()
            .filter(|items| !items.is_empty())
        {
            Some(items) => {
                for (i, evidence) in items.iter().enumerate() {
                    if let Err(e) = validate_hashed_reference(root, dir, evidence) {
                        errors.push(format!("{label}.evidence[{i}]: {e}"));
                    }
                }
            }
            None => errors.push(format!(
                "{label}.evidence: expected nonempty array of hashed references"
            )),
        }
    }
}
fn validate_hashed_reference(
    root: &Path,
    dir: &Path,
    record: &Value,
) -> Result<std::path::PathBuf> {
    let reference = nonempty(&record["path"]).context("missing file path")?;
    let expected = nonempty(&record["sha256"]).context("missing SHA-256")?;
    let (path, bound) = reference_bytes(root, dir, reference)?;
    ensure!(expected == sha(&bound), "SHA-256 is stale");
    Ok(path)
}
/// Resolves `file` or `file#Section` relative to `dir` and returns the confined file
/// with the bytes a hash binds: the whole file, or only the unique section's bytes
/// (heading through the line before the next plain H2, trailing blank lines removed).
pub fn reference_bytes(
    root: &Path,
    dir: &Path,
    reference: &str,
) -> Result<(std::path::PathBuf, Vec<u8>)> {
    let (file, section) = sections::split_reference(reference);
    let path = confined_file(root, dir, file)?;
    let bytes = fs::read(&path)?;
    let Some(section) = section else {
        return Ok((path, bytes));
    };
    let text = String::from_utf8(bytes).context("section reference needs a UTF-8 artifact")?;
    let selected = sections::section_bytes(&text, section).with_context(|| {
        format!("expected exactly one plain H2 heading '## {section}' in {file}")
    })?;
    Ok((path, selected.as_bytes().to_vec()))
}
fn valid_section(section: &str) -> bool {
    !section.trim().is_empty() && section == section.trim() && !section.contains(['\n', '\r'])
}
fn validate_component(root: &Path, dir: &Path, label: &str, v: &Value, errors: &mut Vec<String>) {
    if !v.is_object() || nonempty(&v["status"]).is_none() {
        errors.push(format!(
            "{label}: component needs an object and nonempty status"
        ));
        return;
    }
    let status = nonempty(&v["status"]);
    validate_references(root, dir, label, v, status, errors);
    if let Some(section) = v.get("section") {
        let Some(section) = nonempty(section).filter(|s| valid_section(s)) else {
            errors.push(format!("{label}.section: expected nonempty single-line string without surrounding whitespace"));
            return;
        };
        let Some(path) = nonempty(&v["path"]) else {
            errors.push(format!("{label}.section: requires a path artifact"));
            return;
        };
        let (file, inline) = sections::split_reference(path);
        if inline.is_some_and(|s| s != section) {
            errors.push(format!(
                "{label}.section: differs from the section in path {path}"
            ));
            return;
        }
        match confined_file(root, dir, file).and_then(fs::read_to_string) {
            Ok(text) => {
                let count = sections::heading_count(&text, section);
                if count != 1 {
                    errors.push(format!("{label}.section: expected exactly one plain H2 heading '## {section}' in {file}, found {count}"));
                } else if inline.is_none() {
                    empty_selection(&format!("{label}.section"), file, section, status, &text, errors);
                }
            }
            Err(e) => errors.push(format!(
                "{label}.section: cannot read UTF-8 artifact {file}: {e}"
            )),
        }
    }
}
/// Statuses that claim no materialized content; every other status claims some.
const UNMATERIALIZED: [&str; 2] = ["not_materialized", "inapplicable"];
/// A component whose status claims materialized content must select a section with a
/// body beyond its heading (text or fenced lines, not only headings and blank lines).
fn empty_selection(
    label: &str,
    file: &str,
    section: &str,
    status: Option<&str>,
    text: &str,
    errors: &mut Vec<String>,
) {
    let Some(status) = status.filter(|s| !UNMATERIALIZED.contains(s)) else {
        return;
    };
    if sections::section_has_body(text, section) == Some(false) {
        errors.push(format!("{label}: status {status} claims content, but '## {section}' in {file} has no body beyond its heading"));
    }
}
// A reference is `file` or `file#Section`; the section must be one plain H2 heading
// of the narrow catalog grammar in `sections` (not a Markdown renderer).
fn validate_references(
    root: &Path,
    dir: &Path,
    label: &str,
    v: &Value,
    status: Option<&str>,
    errors: &mut Vec<String>,
) {
    for field in ["path", "description"] {
        if let Some(reference) = v.get(field) {
            match nonempty(reference) {
                Some(p) => {
                    let (file, section) = sections::split_reference(p);
                    // an absent syntax projection is regenerable (syntax::ArtifactReader::projection validates it)
                    if label == "ast"
                        && field == "path"
                        && v["format"] == "rust_syntax_projection"
                        && Path::new(file).file_name().is_some_and(|f| f == "syntax.json")
                        && fs::symlink_metadata(dir.join(file)).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound)
                    {
                        continue;
                    }
                    match confined_file(root, dir, file) {
                        Err(e) => errors.push(format!("{label}.{field}: {p}: {e}")),
                        Ok(path) => {
                            let Some(section) = section else { continue };
                            if !valid_section(section) {
                                errors.push(format!("{label}.{field}: {p}: invalid section name"));
                                continue;
                            }
                            match fs::read_to_string(&path) {
                                Ok(text) => {
                                    let count = sections::heading_count(&text, section);
                                    if count != 1 {
                                        errors.push(format!("{label}.{field}: expected exactly one plain H2 heading '## {section}' in {file}, found {count}"));
                                    } else {
                                        empty_selection(&format!("{label}.{field}"), file, section, status, &text, errors);
                                    }
                                }
                                Err(e) => errors.push(format!(
                                    "{label}.{field}: cannot read UTF-8 artifact {file}: {e}"
                                )),
                            }
                        }
                    }
                }
                None => errors.push(format!("{label}.{field}: expected nonempty file reference")),
            }
        }
    }
}

#[cfg(test)]
mod groupant_tests {
    #[test]
    fn rust_approximants_validate_bindings_budgets_and_evidence() {
        let root =
            std::env::temp_dir().join(format!("mova-rust-approximants-{}", std::process::id()));
        let dir = root.join("a.rs.groupant");
        fs::create_dir_all(&dir).unwrap();
        fs::write(root.join("a.rs"), "fn original() {}").unwrap();
        fs::write(dir.join("approx.rs"), "fn approximate() {}").unwrap();
        fs::write(dir.join("evidence.md"), "bounded domain").unwrap();
        let source = json!({"path":"../a.rs","sha256":sha(b"fn original() {}")});
        let record = json!({
            "id":"profile", "path":"approx.rs", "sha256":sha(b"fn approximate() {}"),
            "source":source, "budget_percent":1, "error_percent":0,
            "metric":"relative error", "domain":"nonzero inputs", "assumptions":[],
            "bound_kind":"analytical", "status":"partial",
            "classification":{"metric_kind":"scalar_relative", "method_kind":"quantized_numeric", "assurance_kind":"analytical"},
            "evidence":[{"path":"evidence.md","sha256":sha(b"bounded domain")}]
        });
        let mut manifest = json!({"source":source,"components":{
            "ast":{"status":"not_materialized"},
            "approximant":{"status":"not_materialized"},
            "invariant":{"status":"not_materialized"},
            "descriptant":{"status":"not_materialized"},
            "obernants":[
                {"id":"nonbehavior","type":"behavior_complement","status":"not_materialized"},
                {"id":"reverse","type":"reverse_reconstruction","status":"not_materialized"}
            ]
        }});
        let check = |m: &Value| {
            let mut errors = Vec::new();
            validate_components(&root, &dir, m, &mut errors);
            errors
        };
        assert!(check(&manifest).is_empty(), "legacy manifests remain valid");
        for budget in [1, 2, 5, 10] {
            let mut profile = record.clone();
            profile["budget_percent"] = json!(budget);
            profile["error_percent"] = json!(budget);
            manifest["components"]["rust_approximants"] = json!([profile]);
            assert!(check(&manifest).is_empty(), "{:?}", check(&manifest));
        }
        for (field, value, diagnostic) in [
            ("sha256", json!("stale"), "SHA-256"),
            ("path", json!("../a.rs"), "SHA-256"),
            ("budget_percent", json!(26), "budget_percent"),
            ("budget_percent", json!(0), "budget_percent"),
            ("budget_percent", json!(11), "budget_percent"),
            ("budget_percent", json!(15), "budget_percent"),
            ("budget_percent", json!(25), "budget_percent"),
            ("budget_percent", json!(10.000001), "budget_percent"),
            ("error_percent", json!(1.1), "error_percent"),
            ("error_percent", json!(-1), "error_percent"),
            ("error_percent", json!("NaN"), "error_percent"),
            ("error_percent", json!("Infinity"), "error_percent"),
            ("error_percent", Value::Null, "error_percent"),
            ("evidence", json!([]), "evidence"),
            (
                "evidence",
                json!([{"path":"missing","sha256":"none"}]),
                "evidence",
            ),
            (
                "source",
                json!({"path":"../a.rs","sha256":"stale"}),
                "source",
            ),
        ] {
            let mut bad = record.clone();
            bad[field] = value;
            manifest["components"]["rust_approximants"] = json!([bad]);
            let errors = check(&manifest);
            assert!(
                errors.iter().any(|s| s.contains(diagnostic)),
                "{field}: {errors:?}"
            );
        }
        for metric in [
            "scalar_relative",
            "normalized_rank",
            "objective_loss",
            "probability_relative",
        ] {
            for method in [
                "quantized_numeric",
                "certified_sample",
                "relaxation_certificate",
                "floating_normalization",
                "interval_certificate",
            ] {
                for (bound, assurance) in [
                    ("analytical", "analytical"),
                    ("analytical", "per_input_certificate"),
                    ("empirical", "finite_observation"),
                ] {
                    let mut profile = record.clone();
                    profile["classification"] = json!({
                        "metric_kind":metric, "method_kind":method, "assurance_kind":assurance,
                        "extension":{"path":"opaque extension"}
                    });
                    profile["bound_kind"] = json!(bound);
                    manifest["components"]["rust_approximants"] = json!([profile]);
                    assert!(check(&manifest).is_empty(), "{:?}", check(&manifest));
                }
            }
        }
        for field in ["metric_kind", "method_kind", "assurance_kind"] {
            for value in [json!("unknown"), json!(""), json!(1), Value::Null] {
                let mut bad = record.clone();
                bad["classification"][field] = value;
                manifest["components"]["rust_approximants"] = json!([bad]);
                assert!(
                    check(&manifest)
                        .iter()
                        .any(|s| s.contains(&format!("classification.{field}")))
                );
            }
            let mut bad = record.clone();
            bad["classification"].as_object_mut().unwrap().remove(field);
            manifest["components"]["rust_approximants"] = json!([bad]);
            assert!(
                check(&manifest)
                    .iter()
                    .any(|s| s.contains(&format!("classification.{field}")))
            );
        }
        for classification in [json!({}), json!([]), json!("analytical"), Value::Null] {
            let mut bad = record.clone();
            bad["classification"] = classification;
            manifest["components"]["rust_approximants"] = json!([bad]);
            assert!(
                check(&manifest)
                    .iter()
                    .any(|s| s.contains("classification"))
            );
        }
        let mut missing = record.clone();
        missing.as_object_mut().unwrap().remove("classification");
        manifest["components"]["rust_approximants"] = json!([missing]);
        assert!(
            check(&manifest)
                .iter()
                .any(|s| s.contains("classification"))
        );
        for (bound, assurance) in [
            ("empirical", "analytical"),
            ("empirical", "per_input_certificate"),
            ("analytical", "finite_observation"),
        ] {
            let mut bad = record.clone();
            bad["bound_kind"] = json!(bound);
            bad["classification"]["assurance_kind"] = json!(assurance);
            manifest["components"]["rust_approximants"] = json!([bad]);
            assert!(
                check(&manifest)
                    .iter()
                    .any(|s| s.contains("must agree with bound_kind"))
            );
        }
        for budget in [10, 11, 15, 25] {
            let mut bad = record.clone();
            bad["budget_percent"] = json!(budget);
            bad["error_percent"] = json!(10.000001);
            manifest["components"]["rust_approximants"] = json!([bad]);
            assert!(check(&manifest).iter().any(|s| s.contains("error_percent")));
        }
        let mut escaped = record.clone();
        escaped["path"] = json!("../a.rs");
        escaped["sha256"] = source["sha256"].clone();
        manifest["components"]["rust_approximants"] = json!([escaped]);
        assert!(
            check(&manifest)
                .iter()
                .any(|s| s.contains("inside its sidecar"))
        );
        let mut stale_evidence = record.clone();
        stale_evidence["evidence"][0]["sha256"] = json!("stale");
        manifest["components"]["rust_approximants"] = json!([stale_evidence]);
        assert!(
            check(&manifest)
                .iter()
                .any(|s| s.contains("evidence[0]: SHA-256"))
        );
        manifest["components"]["rust_approximants"] = json!([record, record]);
        assert!(check(&manifest).iter().any(|s| s.contains("duplicate ID")));
        // JSON cannot represent nonfinite numbers: lexical encodings fail parsing.
        assert!(serde_json::from_str::<Value>("1e999").is_err());
        assert!(serde_json::from_str::<Value>("NaN").is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn fragment_syntax_anchors_bind_envelopes_and_current_source() {
        let root =
            std::env::temp_dir().join(format!("mova-fragment-anchor-test-{}", std::process::id()));
        let sidecar = root.join("a.rs.groupant");
        let dir = sidecar.join("part.groupant");
        fs::create_dir_all(&dir).unwrap();
        assert!(
            Command::new("git")
                .args(["init", "-q"])
                .arg(&root)
                .status()
                .unwrap()
                .success()
        );
        let root = fs::canonicalize(root).unwrap();
        let text = "fn café() {}\n\nfn b() {}\n";
        fs::write(root.join("a.rs"), text).unwrap();
        fs::write(sidecar.join("model.md"), "composition").unwrap();
        let registry: Registry = serde_json::from_value(json!({
            "schema":1,"root":"leaf","nodes":[{
                "id":"leaf","source":"a.rs","description":"model","state":"state","cost":"linear",
                "forward":["forward"],"inverse":["reverse"],"checks":["must not run"],
                "limitations":["partial"]}]
        }))
        .unwrap();
        let components = json!({
            "ast":{"status":"not_materialized"},
            "approximant":{"status":"not_materialized"},
            "invariant":{"status":"not_materialized"},
            "descriptant":{"status":"not_materialized"},
            "obernants":[
                {"id":"b","type":"behavior_complement","status":"not_materialized"},
                {"id":"r","type":"reverse_reconstruction","status":"not_materialized"}]
        });
        let mut parent = json!({
            "schema":1,"kind":"groupant","id":"leaf","status":"partial",
            "source":{"path":"../a.rs","sha256":sha(text.as_bytes())},
            "components":components,
            "children":[{"id":"part","path":"part.groupant"}]
        });
        parent["components"]["composition"] = json!({"status":"partial","path":"model.md"});
        fs::write(
            sidecar.join("manifest.json"),
            serde_json::to_vec(&parent).unwrap(),
        )
        .unwrap();
        let projection = serde_json::to_value(syntax::project(&root, "a.rs").unwrap()).unwrap();
        let nodes = projection["nodes"].as_array().unwrap();
        let item_ids: Vec<u64> = nodes
            .iter()
            .filter(|n| n["category"] == "item")
            .map(|n| n["id"].as_u64().unwrap())
            .collect();
        assert_eq!(item_ids.len(), 2);
        let first_end = text.find("\n").unwrap() as u64;
        let last_end = text.len() as u64 - 1;
        assert_eq!(nodes[item_ids[0] as usize]["start"], 0);
        assert_eq!(nodes[item_ids[0] as usize]["end"], first_end);
        assert!(nodes[item_ids[1] as usize]["start"].as_u64().unwrap() > first_end);
        let mut fragment = json!({
            "schema":1,"kind":"groupant_fragment","id":"part","status":"partial",
            "source":{"path":"../../a.rs","sha256":sha(text.as_bytes())},
            "components":components,
            "coverage":{"description":"selected operations","byte_range":{"start":0,"end":first_end},
                "syntax_nodes":{"path":"../syntax.json","node_ids":[item_ids[0]]}}
        });
        let report = |m: &Value, p: &Value| {
            fs::write(dir.join("manifest.json"), serde_json::to_vec(m).unwrap()).unwrap();
            fs::write(sidecar.join("syntax.json"), serde_json::to_vec(p).unwrap()).unwrap();
            groupants(&root, &registry, "registry-hash").unwrap()
        };
        // Independent expected spans: one item, disjoint items with a gap, root/item overlap.
        for (ids, end) in [
            (json!([item_ids[0]]), first_end),
            (json!([item_ids[1], item_ids[0]]), last_end),
            (json!([item_ids[0], item_ids[1]]), last_end),
            (json!([item_ids[0], 0]), text.len() as u64),
        ] {
            fragment["coverage"]["syntax_nodes"]["node_ids"] = ids;
            fragment["coverage"]["byte_range"]["end"] = json!(end);
            let good = report(&fragment, &projection);
            assert_eq!(good["valid"], true, "{good}");
            assert_eq!(good, report(&fragment, &projection));
            assert_eq!(good["files"][0]["subgroupants"][0]["manifest"], fragment);
        }
        fragment["coverage"]["syntax_nodes"]["node_ids"] = json!([item_ids[0]]);
        fragment["coverage"]["byte_range"]["end"] = json!(first_end);
        for ids in [
            json!(null),
            json!([]),
            json!([item_ids[0], item_ids[0]]),
            json!(["1"]),
            json!([1.5]),
            json!([-1]),
            json!([u64::MAX]),
            json!([nodes.len()]),
            json!([true]),
        ] {
            let mut bad = fragment.clone();
            bad["coverage"]["syntax_nodes"]["node_ids"] = ids;
            let got = report(&bad, &projection);
            assert_eq!(got["valid"], false, "{bad}");
            assert!(
                got["diagnostics"]
                    .to_string()
                    .contains("coverage.syntax_nodes")
            );
        }
        for field in ["node_ids", "path"] {
            let mut bad = fragment.clone();
            bad["coverage"]["syntax_nodes"]
                .as_object_mut()
                .unwrap()
                .remove(field);
            assert_eq!(report(&bad, &projection)["valid"], false);
        }
        for path in [
            json!(null),
            json!(""),
            json!("missing.json"),
            json!(".."),
            json!(sidecar.join("syntax.json").to_str().unwrap()),
            json!("../../../outside.json"),
        ] {
            let mut bad = fragment.clone();
            bad["coverage"]["syntax_nodes"]["path"] = path;
            assert_eq!(report(&bad, &projection)["valid"], false);
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/etc/passwd", dir.join("escape.json")).unwrap();
            let mut bad = fragment.clone();
            bad["coverage"]["syntax_nodes"]["path"] = json!("escape.json");
            assert_eq!(report(&bad, &projection)["valid"], false);
        }
        for range in [
            json!(null),
            json!({"start":0,"end":last_end}),
            json!({"start":1,"end":first_end}),
            json!({"start":0,"end":first_end-1}),
        ] {
            let mut bad = fragment.clone();
            bad["coverage"]["byte_range"] = range;
            assert_eq!(report(&bad, &projection)["valid"], false);
        }
        let mut bad = fragment.clone();
        bad["coverage"]
            .as_object_mut()
            .unwrap()
            .remove("byte_range");
        assert_eq!(report(&bad, &projection)["valid"], false);
        for case in 0..6 {
            let mut bad_projection = projection.clone();
            match case {
                0 => bad_projection["source"]["path"] = json!("other.rs"),
                1 => bad_projection["source"]["sha256"] = json!("stale"),
                2 => bad_projection["source"]["bytes"] = json!(0),
                3 => bad_projection["nodes"][1]["id"] = json!(999),
                4 => bad_projection["nodes"][1]["end"] = json!(text.len() + 1),
                _ => bad_projection["kind"] = json!("other"),
            }
            let got = report(&fragment, &bad_projection);
            assert_eq!(got["valid"], false, "case {case}: {got}");
            assert!(
                got["diagnostics"]
                    .to_string()
                    .contains("coverage.syntax_nodes")
            );
        }
        // A readable but non-JSON file is still invalid even though generic references pass.
        report(&fragment, &projection);
        fs::write(sidecar.join("syntax.json"), "{malformed").unwrap();
        assert_eq!(groupants(&root, &registry, "hash").unwrap()["valid"], false);
        // An absent projection is regenerable: rebuilt in memory, validated and counted, not an error.
        report(&fragment, &projection);
        fs::remove_file(sidecar.join("syntax.json")).unwrap();
        let rebuilt = groupants(&root, &registry, "hash").unwrap();
        assert_eq!(rebuilt["valid"], true, "{}", rebuilt["diagnostics"]);
        assert!(rebuilt["syntax_projections_regenerated"].as_u64().unwrap() >= 1);
        // Negative control: regeneration does not make a stale manifest binding current.
        fs::write(root.join("a.rs"), format!("{text}// edited\n")).unwrap();
        let stale_rebuilt = groupants(&root, &registry, "hash").unwrap();
        assert_eq!(stale_rebuilt["valid"], false);
        assert!(stale_rebuilt["diagnostics"].to_string().contains("SHA-256"));
        fs::write(root.join("a.rs"), &text).unwrap();
        // Rebinding manifests after even an append cannot silently refresh the projection.
        let changed = format!("{text}// changed\n");
        fs::write(root.join("a.rs"), &changed).unwrap();
        parent["source"]["sha256"] = json!(sha(changed.as_bytes()));
        fragment["source"]["sha256"] = parent["source"]["sha256"].clone();
        fs::write(
            sidecar.join("manifest.json"),
            serde_json::to_vec(&parent).unwrap(),
        )
        .unwrap();
        let stale = report(&fragment, &projection);
        assert_eq!(stale["valid"], false);
        assert!(
            stale["diagnostics"]
                .to_string()
                .contains("projection source SHA-256")
        );
        // Removing opt-in restores the established range-only contract.
        fragment["coverage"]
            .as_object_mut()
            .unwrap()
            .remove("syntax_nodes");
        assert_eq!(report(&fragment, &projection)["valid"], true);
        fs::remove_dir_all(root).unwrap();
    }

    fn nested_controls(root: &Path, original: &Value, report: &impl Fn(&Value) -> Value) {
        let child_dir = root.join("a.rs.groupant/memo.groupant");
        let grand_dir = child_dir.join("detail.groupant");
        fs::create_dir_all(&grand_dir).unwrap();
        fs::write(child_dir.join("model.md"), "local interaction model").unwrap();
        fs::write(grand_dir.join("model.md"), "local model").unwrap();
        let mut parent = original.clone();
        parent["children"] = json!([{"id":"memo","path":"memo.groupant"}]);
        parent["components"]["composition"] = json!({"status":"partial","path":"model.md"});
        let mut child = original.clone();
        child["kind"] = json!("groupant_fragment");
        child["id"] = json!("memo");
        child["source"]["path"] = json!("../../a.rs");
        child["coverage"] =
            json!({"description":"memory operations","byte_range":{"start":0,"end":9}});
        child["children"] = json!([{"id":"detail","path":"detail.groupant"}]);
        child["components"]["composition"] = json!({"status":"partial","description":"model.md"});
        let mut grand = child.clone();
        grand["id"] = json!("detail");
        grand["source"]["path"] = json!("../../../a.rs");
        grand["coverage"]["byte_range"] = json!({"start":3,"end":4});
        grand.as_object_mut().unwrap().remove("children");
        let write = |c: &Value, g: &Value| {
            fs::write(
                child_dir.join("manifest.json"),
                serde_json::to_vec(c).unwrap(),
            )
            .unwrap();
            fs::write(
                grand_dir.join("manifest.json"),
                serde_json::to_vec(g).unwrap(),
            )
            .unwrap();
        };
        write(&child, &grand);
        // Undeclared directories remain open extension material.
        fs::create_dir_all(child_dir.join("unlisted.groupant")).unwrap();
        fs::write(
            child_dir.join("unlisted.groupant/manifest.json"),
            "malformed",
        )
        .unwrap();
        let good = report(&parent);
        assert_eq!(good["valid"], true, "{good}");
        assert_eq!(good, report(&parent));
        assert_eq!(good["subgroupants"], 2);
        let entries = &good["files"][0]["subgroupants"];
        assert_eq!(entries[0]["parent"], "leaf");
        assert_eq!(entries[0]["id"], "memo");
        assert_eq!(entries[0]["depth"], 1);
        assert_eq!(entries[1]["parent"], "memo");
        assert_eq!(entries[1]["depth"], 2);
        assert_eq!(
            entries[1]["required_types"][0]["type"],
            "behavior_complement"
        );
        assert_eq!(good["logical_tree"][0]["children"], json!([]));

        let mut projection = serde_json::to_value(syntax::project(root, "a.rs").unwrap()).unwrap();
        let mut projected = child.clone();
        projected["components"]["ast"] =
            json!({"status":"partial","format":"rust_syntax_projection","path":"syntax.json"});
        fs::write(
            child_dir.join("syntax.json"),
            serde_json::to_vec(&projection).unwrap(),
        )
        .unwrap();
        write(&projected, &grand);
        assert_eq!(report(&parent)["valid"], true);
        // the declared ast projection absent: regenerable, still valid and counted; a stale manifest stays red elsewhere
        fs::remove_file(child_dir.join("syntax.json")).unwrap();
        let regenerated = report(&parent);
        assert_eq!(regenerated["valid"], true, "{}", regenerated["diagnostics"]);
        assert!(regenerated["syntax_projections_regenerated"].as_u64().unwrap() >= 1);
        // negative control: another missing ast artifact name is still an error
        projected["components"]["ast"]["path"] = json!("other.json");
        write(&projected, &grand);
        assert_eq!(report(&parent)["valid"], false);
        projected["components"]["ast"]["path"] = json!("syntax.json");
        write(&projected, &grand);
        projection["source"]["sha256"] = json!("stale");
        fs::write(
            child_dir.join("syntax.json"),
            serde_json::to_vec(&projection).unwrap(),
        )
        .unwrap();
        assert!(
            report(&parent)["diagnostics"]
                .to_string()
                .contains("ast rust_syntax_projection")
        );
        projection = serde_json::to_value(syntax::project(root, "a.rs").unwrap()).unwrap();
        projection["nodes"][0]["end"] = json!(10000);
        fs::write(
            child_dir.join("syntax.json"),
            serde_json::to_vec(&projection).unwrap(),
        )
        .unwrap();
        assert_eq!(report(&parent)["valid"], false);

        for case in 0..27 {
            let mut p = parent.clone();
            let mut c = child.clone();
            let mut g = grand.clone();
            match case {
                0 => p["children"] = json!(null),
                1 => p["children"] = json!([42]),
                2 => p["children"][0]["path"] = json!("../memo.groupant"),
                3 => p["children"][0]["path"] = json!("memo.groupant/detail.groupant"),
                4 => p["children"][0]["path"] = json!("/memo.groupant"),
                5 => p["children"][0]["path"] = json!("memo.groupant/"),
                6 => p["children"][0]["path"] = json!("missing.groupant"),
                7 => p["children"][0]["path"] = json!("model.md"),
                8 => {
                    p["components"]
                        .as_object_mut()
                        .unwrap()
                        .remove("composition");
                }
                9 => p["components"]["composition"] = json!({"status":"partial"}),
                10 => p["components"]["composition"]["path"] = json!("missing.md"),
                11 => c["kind"] = json!("groupant"),
                12 => c["id"] = json!("wrong"),
                13 => c["source"]["path"] = json!("model.md"),
                14 => c["source"]["sha256"] = json!("stale"),
                15 => c["coverage"]["description"] = json!(" "),
                16 => c["coverage"]["byte_range"] = json!({"start":0,"end":10}),
                17 => c["coverage"]["byte_range"] = json!({"start":4,"end":4}),
                18 => c["coverage"]["byte_range"] = json!({"start":-1,"end":4}),
                19 => g["coverage"]["byte_range"] = json!({"start":0,"end":10}),
                20 => {
                    g["coverage"].as_object_mut().unwrap().remove("byte_range");
                }
                21 => c["components"]["obernants"] = json!([]),
                22 => c["components"]["composition"] = json!({"status":"partial"}),
                23 => {
                    p["children"]
                        .as_array_mut()
                        .unwrap()
                        .push(json!({"id":"other","path":"memo.groupant"}));
                }
                24 => {
                    p["children"][0]["id"] = json!("leaf");
                    c["id"] = json!("leaf");
                }
                25 => {
                    c["children"][0]["id"] = json!("memo");
                    g["id"] = json!("memo");
                }
                _ => c["components"]["invariant"] = json!(null),
            }
            write(&c, &g);
            let invalid = report(&p);
            assert_eq!(invalid["valid"], false, "nested case {case}: {invalid}");
            assert_eq!(invalid["files"][0]["status"], "invalid");
        }
        write(&child, &grand);
        let sibling_dir = root.join("a.rs.groupant/sibling.groupant");
        fs::create_dir(&sibling_dir).unwrap();
        fs::write(sibling_dir.join("model.md"), "sibling model").unwrap();
        let mut sibling = child.clone();
        sibling["id"] = json!("sibling");
        sibling.as_object_mut().unwrap().remove("children");
        fs::write(
            sibling_dir.join("manifest.json"),
            serde_json::to_vec(&sibling).unwrap(),
        )
        .unwrap();
        let mut siblings = parent.clone();
        siblings["children"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"sibling","path":"sibling.groupant"}));
        let overlap = report(&siblings);
        assert_eq!(overlap["valid"], true);
        assert_eq!(overlap["subgroupants"], 3);
        assert_eq!(overlap["files"][0]["subgroupants"][2]["id"], "sibling");
        fs::remove_dir_all(sibling_dir).unwrap();

        let legacy_dir = root.join("legacy.rs.groupant");
        fs::create_dir(&legacy_dir).unwrap();
        fs::write(legacy_dir.join("model.md"), "other source").unwrap();
        let mut duplicate = original.clone();
        duplicate["id"] = json!("memo");
        duplicate["source"] = json!({"path":"../legacy.rs","sha256":sha(b"// backlog")});
        fs::write(
            legacy_dir.join("manifest.json"),
            serde_json::to_vec(&duplicate).unwrap(),
        )
        .unwrap();
        assert!(
            report(&parent)["diagnostics"]
                .to_string()
                .contains("duplicate groupant ID: memo")
        );
        fs::remove_dir_all(legacy_dir).unwrap();

        // Optional ranges do not assert complete source coverage.
        let mut c = child.clone();
        let mut g = grand.clone();
        c["coverage"].as_object_mut().unwrap().remove("byte_range");
        g["coverage"].as_object_mut().unwrap().remove("byte_range");
        write(&c, &g);
        assert_eq!(report(&parent)["valid"], true);
        write(&child, &grand);
        fs::write(grand_dir.join("manifest.json"), "{bad").unwrap();
        let bad = report(&parent);
        assert_eq!(bad["valid"], false);
        assert_eq!(bad["files"][0]["subgroupants"][0]["status"], "invalid");
        assert_eq!(bad["files"][0]["subgroupants"][1]["status"], "invalid");
        fs::remove_file(grand_dir.join("manifest.json")).unwrap();
        assert_eq!(report(&parent)["valid"], false);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("../manifest.json", grand_dir.join("manifest.json"))
                .unwrap();
            assert_eq!(report(&parent)["valid"], false);
            fs::remove_file(grand_dir.join("manifest.json")).unwrap();
            fs::remove_dir_all(&grand_dir).unwrap();
            std::os::unix::fs::symlink("..", &grand_dir).unwrap();
            assert_eq!(report(&parent)["valid"], false);
            fs::remove_file(&grand_dir).unwrap();
        }
        fs::remove_dir_all(&child_dir).unwrap();

        // Exercise the actual traversal boundary, not just the guard in isolation.
        let mut deep_parent = parent.clone();
        deep_parent["children"] = json!([{"id":"depth_1","path":"d.groupant"}]);
        let mut path = root.join("a.rs.groupant");
        for depth in 1..=MAX_FRAGMENT_DEPTH + 1 {
            path = path.join("d.groupant");
            fs::create_dir(&path).unwrap();
            fs::write(path.join("model.md"), "depth model").unwrap();
            let mut fragment = child.clone();
            fragment["id"] = json!(format!("depth_{depth}"));
            fragment["source"]["path"] = json!(format!("{}a.rs", "../".repeat(depth + 1)));
            fragment["children"] = if depth <= MAX_FRAGMENT_DEPTH {
                json!([{"id":format!("depth_{}", depth + 1),"path":"d.groupant"}])
            } else {
                json!([])
            };
            fs::write(
                path.join("manifest.json"),
                serde_json::to_vec(&fragment).unwrap(),
            )
            .unwrap();
        }
        let deep = report(&deep_parent);
        assert_eq!(deep["valid"], false);
        assert_eq!(deep["subgroupants"], MAX_FRAGMENT_DEPTH);
        assert!(deep["diagnostics"].to_string().contains("depth exceeds"));
        fs::remove_dir_all(root.join("a.rs.groupant/d.groupant")).unwrap();
        assert_eq!(report(original)["valid"], true);
    }

    #[test]
    fn fragment_utf8_ranges_and_depth_guard() {
        let root = std::env::temp_dir();
        let mut ids = BTreeSet::new();
        let mut reports = Vec::new();
        let mut validator = FragmentValidation {
            root: &root,
            source: &root,
            source_sha: None,
            source_text: Some("aéz"),
            ids: &mut ids,
            reports: &mut reports,
            artifacts: syntax::ArtifactReader::new(),
        };
        let mut m = json!({"coverage":{"description":"unicode","byte_range":{"start":1,"end":3}}});
        let mut errors = Vec::new();
        assert_eq!(validator.range(&m, None, &mut errors), Some((1, 3)));
        assert!(errors.is_empty());
        for (start, end) in [(1, 2), (2, 3), (3, 1), (0, 5), (0, u64::MAX)] {
            m["coverage"]["byte_range"] = json!({"start":start,"end":end});
            assert!(validator.range(&m, None, &mut errors).is_none());
        }
        m["coverage"]["byte_range"] = json!({"start":0,"end":4});
        assert!(validator.range(&m, Some((1, 3)), &mut errors).is_none());
        let parent = json!({"children":[{"id":"never-read","path":"no.groupant"}],
            "components":{"composition":{"status":"partial","path":"model.md"}}});
        errors.clear();
        validator.children(&root, &parent, MAX_FRAGMENT_DEPTH, None, &mut errors);
        assert_eq!(
            errors,
            vec![format!(
                "nested groupant depth exceeds {MAX_FRAGMENT_DEPTH}"
            )]
        );
        assert!(validator.reports.is_empty());
    }
    use super::*;

    // Run the same artifact controls through real file and group inventories.
    fn section_controls(original: &Value, artifact: &Path, report: impl Fn(&Value) -> Value) {
        let saved = fs::read(artifact).unwrap();
        let mut m = original.clone();
        m["components"]["invariant"]["section"] = json!("Catalog entry");
        m["components"]["invariant"]["future"] =
            json!({"section":42,"path":"not-a-reference","checks":["must not execute"]});
        for text in [
            "## Catalog entry\nbody\n",
            "## Catalog entry\r\nbody\r\n",
            "```md\n## Catalog entry\n```\n## Catalog entry\nbody\n",
            "   ~~~~ text\n## Catalog entry\n  ~~~~~ \t\n## Catalog entry\nbody\n",
            "## Catalog entry\n\n```sh\ncargo test\n```\n",
        ] {
            fs::write(artifact, text).unwrap();
            let good = report(&m);
            assert_eq!(good["valid"], true, "{text:?}: {}", good["diagnostics"]);
            assert_eq!(good, report(&m));
        }
        for text in [
            "## Other\n",
            "## Catalog entry\n## Catalog entry\n",
            "# Catalog entry\n### Catalog entry\n",
            " ## Catalog entry\n## Catalog entry ##\n## Catalog entry \n",
            "```md\n## Catalog entry\n```\n",
            "~~~\n## Catalog entry\n~~~\n",
            "   ```\n## Catalog entry\n   ```\n",
            "````\n```\n## Catalog entry\n",
            "~~~\n```\n## Catalog entry\n",
            "```\n``` not a closing fence\n## Catalog entry\n",
            "```\n## Catalog entry\n",
        ] {
            fs::write(artifact, text).unwrap();
            let bad = report(&m);
            assert_eq!(bad["valid"], false, "{text:?}");
            assert!(bad["diagnostics"].to_string().contains("invariant.section"));
        }
        fs::write(artifact, "## Catalog entry\nbody\n").unwrap();
        for section in [
            json!(null),
            json!(42),
            json!(""),
            json!(" "),
            json!(" padded"),
            json!("two\nlines"),
        ] {
            let mut bad = m.clone();
            bad["components"]["invariant"]["section"] = section;
            assert_eq!(report(&bad)["valid"], false);
        }
        let mut bad = m.clone();
        bad["components"]["invariant"]
            .as_object_mut()
            .unwrap()
            .remove("path");
        bad["components"]["invariant"]["description"] = json!("model.md");
        assert_eq!(report(&bad)["valid"], false);
        bad = m.clone();
        bad["components"]["invariant"]["path"] = json!("missing.md");
        assert_eq!(report(&bad)["valid"], false);
        fs::write(artifact, [0xff, 0xfe]).unwrap();
        assert_eq!(report(&m)["valid"], false);
        assert_eq!(report(original)["valid"], true); // No section preserves binary references.
        fs::write(artifact, "## Catalog entry\nbody\n").unwrap();
        for name in ["ast", "approximant", "descriptant", "extra_component"] {
            let mut extra = original.clone();
            extra["components"][name] =
                json!({"status":"partial","path":"model.md","section":"Catalog entry"});
            assert_eq!(report(&extra)["valid"], true);
            extra["components"][name]["section"] = json!("Missing");
            assert_eq!(report(&extra)["valid"], false);
        }
        let mut typed = original.clone();
        typed["components"]["obernants"][1]["section"] = json!("Catalog entry");
        assert_eq!(report(&typed)["valid"], true);
        typed["components"]["obernants"][1]["section"] = json!("Missing");
        assert_eq!(report(&typed)["valid"], false);
        // Negative controls: a heading-only section (blank lines and subheadings only)
        // cannot carry a component whose status claims content; a gap status can.
        for text in ["## Catalog entry\n", "## Catalog entry\n\n#### Cost\n\n## Next\nx\n"] {
            fs::write(artifact, text).unwrap();
            for name in ["approximant", "tests"] {
                let mut empty = original.clone();
                empty["components"][name] =
                    json!({"status":"partial_source_model","path":"model.md","section":"Catalog entry"});
                let bad = report(&empty);
                assert_eq!(bad["valid"], false, "{name} {text:?}");
                assert!(bad["diagnostics"].to_string().contains("claims content"));
                empty["components"][name] = json!({"status":"partial","path":"model.md#Catalog entry"});
                assert_eq!(report(&empty)["valid"], false, "inline {name} {text:?}");
                empty["components"][name]["status"] = json!("not_materialized");
                assert_eq!(report(&empty)["valid"], true, "gap {name} {text:?}");
            }
            let mut typed = original.clone();
            typed["components"]["obernants"][1]["section"] = json!("Catalog entry");
            assert_eq!(report(&typed)["valid"], false, "obernant {text:?}");
            typed["components"]["obernants"][1]["status"] = json!("not_materialized");
            assert_eq!(report(&typed)["valid"], true, "gap obernant {text:?}");
        }
        fs::write(artifact, saved).unwrap();
    }

    #[test]
    fn groups_bind_transitive_sources_and_preserve_open_families() {
        let root =
            std::env::temp_dir().join(format!("mova-group-tree-test-{}", std::process::id()));
        fs::create_dir_all(root.join("models/root.groupant")).unwrap();
        assert!(
            Command::new("git")
                .args(["init", "-q"])
                .arg(&root)
                .status()
                .unwrap()
                .success()
        );
        fs::write(root.join("a.rs"), "// a").unwrap();
        fs::write(root.join("b.rs"), "// b").unwrap();
        fs::write(
            root.join("models/root.groupant/model.md"),
            "partial group model",
        )
        .unwrap();
        let make_node = |id: &str, source: Option<&str>, children: Vec<&str>| Node {
            id: id.into(),
            source: source.map(str::to_string),
            groupant_path: None,
            children: children.into_iter().map(str::to_string).collect(),
            description: "description".into(),
            state: "state".into(),
            cost: "linear".into(),
            forward: vec!["forward".into()],
            inverse: vec!["inverse".into()],
            checks: vec!["must not execute".into()],
            limitations: vec!["partial".into()],
        };
        let r = Registry {
            schema: 1,
            root: "root".into(),
            nodes: vec![
                make_node("root", None, vec!["nested", "b"]),
                make_node("nested", None, vec!["a"]),
                make_node("a", Some("a.rs"), vec![]),
                make_node("b", Some("b.rs"), vec![]),
            ],
        };
        let mut family = vec![
            json!({"id":"b","type":"behavior_complement","status":"not_materialized"}),
            json!({"id":"r","type":"reverse_reconstruction","status":"partial","path":"model.md"}),
        ];
        for index in 0..36 {
            family.push(
                json!({"id":format!("extra-{index}"),"type":format!("specialized-{index}"),
                "status":"partial","description":"model.md","future":{"value":index}}),
            );
        }
        let original = json!({"schema":1,"kind":"groupant_group","id":"root","status":"partial",
            "children":["b","nested"],"source_bindings":[
                {"path":"b.rs","sha256":sha(b"// b")},{"path":"a.rs","sha256":sha(b"// a")}],
            "components":{"ast":{"status":"not_materialized"},
                "approximant":{"status":"partial","path":"model.md"},
                "invariant":{"status":"partial","path":"model.md"},
                "descriptant":{"status":"present","path":"model.md"},"obernants":family},
            "future_extension":{"checks":["must not execute"]}});
        let report = |m: &Value| {
            fs::write(
                root.join("models/root.groupant/manifest.json"),
                serde_json::to_vec(m).unwrap(),
            )
            .unwrap();
            groupants(&root, &r, "registry-hash").unwrap()
        };
        section_controls(
            &original,
            &root.join("models/root.groupant/model.md"),
            &report,
        );
        let good = report(&original);
        assert_eq!(good, report(&original));
        assert_eq!(good["valid"], true);
        assert_eq!(good["group_sidecars"], 1);
        assert_eq!(good["missing_group_sidecars"], 1);
        assert_eq!(good["missing_sidecars"], 2);
        let group = good["logical_tree"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == "root")
            .unwrap();
        assert_eq!(group["manifest"], original);
        assert_eq!(group["required_types"][0]["status"], "not_materialized");
        for case in 0..14 {
            let mut m = original.clone();
            match case {
                0 => m["children"] = json!(["a", "b"]),
                1 => m["children"] = json!(["nested"]),
                2 => m["children"] = json!(["nested", "b", "b"]),
                3 => m["source_bindings"][1]["sha256"] = json!("stale"),
                4 => {
                    m["source_bindings"].as_array_mut().unwrap().pop();
                }
                5 => m["source_bindings"][1]["path"] = json!("../a.rs"),
                6 => m["source_bindings"][1] = m["source_bindings"][0].clone(),
                7 => m["components"]["obernants"][1]["type"] = json!("other"),
                8 => m["components"]["invariant"]["path"] = json!("/etc/passwd"),
                9 => m["id"] = json!("wrong"),
                10 => m["source_bindings"] = json!(null),
                11 => m["children"] = json!(null),
                12 => m["source_bindings"][1]["path"] = json!("unowned.rs"),
                _ => m["components"]["obernants"][1]["id"] = json!("b"),
            }
            assert_eq!(report(&m)["valid"], false, "case {case}");
        }
        fs::write(root.join("a.rs"), "// changed").unwrap();
        assert_eq!(report(&original)["valid"], false);
        fs::write(root.join("a.rs"), "// a").unwrap();
        fs::create_dir_all(root.join("models/nested.groupant")).unwrap();
        fs::write(
            root.join("models/nested.groupant/model.md"),
            "partial group model",
        )
        .unwrap();
        let mut nested = original.clone();
        nested["id"] = json!("nested");
        nested["children"] = json!(["a"]);
        nested["source_bindings"] = json!([{"path":"a.rs","sha256":sha(b"// a")}]);
        fs::write(
            root.join("models/nested.groupant/manifest.json"),
            serde_json::to_vec(&nested).unwrap(),
        )
        .unwrap();
        let both = report(&original);
        assert_eq!(both["valid"], true);
        assert_eq!(both["group_sidecars"], 2);
        assert_eq!(both["missing_group_sidecars"], 0);
        fs::remove_dir_all(root.join("models/nested.groupant")).unwrap();
        fs::create_dir_all(root.join("a.rs.groupant")).unwrap();
        let mut leaf = original.clone();
        leaf["kind"] = json!("groupant");
        leaf["source"] = json!({"path":"../a.rs","sha256":sha(b"// a")});
        leaf["components"]["obernants"] = json!([
            {"id":"b","type":"behavior_complement","status":"not_materialized"},
            {"id":"r","type":"reverse_reconstruction","status":"not_materialized"}]);
        for name in ["ast", "approximant", "invariant", "descriptant"] {
            leaf["components"][name] = json!({"status":"not_materialized"});
        }
        fs::write(
            root.join("a.rs.groupant/manifest.json"),
            serde_json::to_vec(&leaf).unwrap(),
        )
        .unwrap();
        assert!(
            report(&original)["diagnostics"]
                .to_string()
                .contains("duplicate groupant ID")
        );
        fs::remove_dir_all(root.join("a.rs.groupant")).unwrap();
        fs::write(
            root.join("models/root.groupant/manifest.json"),
            "{malformed",
        )
        .unwrap();
        let malformed = groupants(&root, &r, "registry-hash").unwrap();
        assert_eq!(malformed["valid"], false);
        assert_eq!(malformed["group_sidecars"], 1);
        fs::create_dir_all(root.join("models/nested.groupant")).unwrap();
        fs::write(root.join("models/nested.groupant/manifest.json"), "{}").unwrap();
        assert_eq!(report(&original)["valid"], false);
        fs::remove_dir_all(root.join("models/nested.groupant")).unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/etc", root.join("models/nested.groupant")).unwrap();
            assert_eq!(report(&original)["valid"], false);
            fs::remove_file(root.join("models/nested.groupant")).unwrap();
            std::os::unix::fs::symlink("/etc/passwd", root.join("models/root.groupant/escape"))
                .unwrap();
            let mut m = original.clone();
            m["components"]["invariant"]["path"] = json!("escape");
            assert_eq!(report(&m)["valid"], false);
        }
        let mut unsafe_registry = r.clone();
        unsafe_registry.root = "../escape".into();
        unsafe_registry.nodes[0].id = "../escape".into();
        assert!(
            groupants(&root, &unsafe_registry, "hash")
                .unwrap_err()
                .to_string()
                .contains("unsafe")
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn groupant_validation_negative_controls_and_open_extensions() {
        let root = std::env::temp_dir().join(format!("mova-groupant-test-{}", std::process::id()));
        fs::create_dir_all(root.join("a.rs.groupant")).unwrap();
        assert!(
            Command::new("git")
                .args(["init", "-q"])
                .arg(&root)
                .status()
                .unwrap()
                .success()
        );
        fs::write(root.join("a.rs"), "fn a() {}").unwrap();
        fs::write(root.join("legacy.rs"), "// backlog").unwrap();
        fs::write(root.join("a.rs.groupant/model.md"), "partial model").unwrap();
        let root = fs::canonicalize(root).unwrap();
        let r: Registry = serde_json::from_value(json!({"schema":1,"root":"leaf","nodes":[{
            "id":"leaf","source":"a.rs","description":"model","state":"state","cost":"linear",
            "forward":["forward"],"inverse":["reverse"],"checks":["must not run"],
            "limitations":["partial"]}]}))
        .unwrap();
        let original = json!({"schema":1,"kind":"groupant","id":"leaf","status":"partial",
            "source":{"path":"../a.rs","sha256":sha(b"fn a() {}")},
            "components":{"ast":{"status":"not_materialized"},
                "approximant":{"status":"partial","path":"model.md"},
                "invariant":{"status":"partial","path":"model.md"},
                "descriptant":{"status":"present","path":"model.md"},
                "obernants":[
                    {"id":"b","type":"behavior_complement","status":"not_materialized"},
                    {"id":"r","type":"reverse_reconstruction","status":"partial","path":"model.md"},
                    {"id":"b2","type":"behavior_complement","status":"not_materialized"},
                    {"id":"x","type":"future_type","status":"not_materialized","extra":{"a":1}}]},
            "extensions":{"future":[1,2,3]}});
        let report = |m: &Value| {
            fs::write(
                root.join("a.rs.groupant/manifest.json"),
                serde_json::to_vec(m).unwrap(),
            )
            .unwrap();
            groupants(&root, &r, "registry-hash").unwrap()
        };
        section_controls(&original, &root.join("a.rs.groupant/model.md"), &report);
        nested_controls(&root, &original, &report);
        let a = report(&original);
        assert_eq!(a, report(&original));
        assert_eq!(a["valid"], true);
        assert_eq!(a["missing_sidecars"], 1);
        assert_eq!(a["unregistered_files"], 1);
        assert_eq!(a["files"][0]["manifest"], original);
        assert_eq!(
            a["files"][0]["required_types"][0]["status"],
            "not_materialized"
        );
        assert_eq!(a["files"][0]["verification_inferred"], false);
        for case in 0..9 {
            let mut m = original.clone();
            match case {
                0 => m["source"]["sha256"] = json!("stale"),
                1 => m["components"]["invariant"]["path"] = json!("missing.md"),
                2 => m["components"]["obernants"][1]["id"] = json!("b"),
                3 => {
                    m["components"]["obernants"]
                        .as_array_mut()
                        .unwrap()
                        .remove(1);
                }
                4 => m["components"]["obernants"][0] = json!(42),
                5 => m["source"]["path"] = json!("model.md"),
                6 => m["components"]["obernants"][0]["status"] = json!("inapplicable"),
                7 => {
                    m["components"]["obernants"][1]
                        .as_object_mut()
                        .unwrap()
                        .remove("status");
                }
                _ => m["components"]["invariant"]["path"] = json!("/etc/passwd"),
            }
            let a = report(&m);
            assert_eq!(a["valid"], false, "case {case}");
            assert!(!a["diagnostics"].as_array().unwrap().is_empty());
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/etc/passwd", root.join("a.rs.groupant/escape")).unwrap();
            let mut m = original.clone();
            m["components"]["invariant"]["path"] = json!("escape");
            assert_eq!(report(&m)["valid"], false);
        }
        fs::write(root.join("legacy.rs.groupant"), "not a directory").unwrap();
        assert_eq!(report(&original)["valid"], false);
        fs::remove_file(root.join("legacy.rs.groupant")).unwrap();
        fs::create_dir(root.join("legacy.rs.groupant")).unwrap();
        fs::write(root.join("legacy.rs.groupant/manifest.json"), "{malformed").unwrap();
        assert_eq!(report(&original)["valid"], false);
        let mut duplicate = original.clone();
        duplicate["source"] = json!({"path":"../legacy.rs","sha256":sha(b"// backlog")});
        fs::write(
            root.join("legacy.rs.groupant/manifest.json"),
            serde_json::to_vec(&duplicate).unwrap(),
        )
        .unwrap();
        assert!(
            report(&original)["diagnostics"]
                .to_string()
                .contains("duplicate groupant ID")
        );
        fs::remove_dir_all(root).unwrap();
    }
}

#[cfg(test)]
mod wave33_tests {
    use super::*;
    use std::{hint::black_box, time::Instant};

    #[test]
    fn sorted_membership_matches_linear_reference_and_measures_cost() {
        fn reference(files: &[String], query: &str) -> bool {
            files.iter().any(|p| p == query)
        }
        for files in [
            vec![],
            vec!["a.rs".to_string()],
            vec!["a.rs", "a/child.rs", "aa.rs", "café.rs", "π.rs", "🐙.rs"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        ] {
            assert!(files.windows(2).all(|w| w[0] < w[1]));
            for query in [
                "",
                "a",
                "a.rs",
                "a/child.rs",
                "aa.rs",
                "b.rs",
                "café.rs",
                "π.rs",
                "🐙.rs",
                "🐙z.rs",
            ] {
                assert_eq!(
                    discovered_source(&files, query),
                    reference(&files, query),
                    "{query}"
                );
            }
        }
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let actual = rust_files(&repository).unwrap();
        let synthetic: Vec<_> = (0..4096)
            .map(|i| format!("src/bucket_{:02}/module_{i:05}.rs", i / 128))
            .collect();
        for (label, files) in [("repository_paths", actual), ("synthetic_4096", synthetic)] {
            assert!(files.windows(2).all(|w| w[0] < w[1]));
            let mut queries = files.clone();
            queries.extend(files.iter().step_by(8).map(|p| format!("{p}.absent")));
            queries.push(String::new());
            queries.push("🦀/absent.rs".to_owned());
            let expected: Vec<_> = queries.iter().map(|q| reference(&files, q)).collect();
            let observed: Vec<_> = queries
                .iter()
                .map(|q| discovered_source(&files, q))
                .collect();
            assert_eq!(observed, expected);
            let expected_hits = expected.iter().filter(|&&hit| hit).count();
            let measure = |binary: bool| {
                let start = Instant::now();
                let mut hits = 0;
                for _ in 0..3 {
                    for query in &queries {
                        let files = black_box(files.as_slice());
                        let query = black_box(query.as_str());
                        hits += usize::from(if binary {
                            discovered_source(files, query)
                        } else {
                            reference(files, query)
                        });
                    }
                }
                let elapsed = start.elapsed().as_nanos();
                assert_eq!(hits, expected_hits * 3);
                (elapsed, hits)
            };
            for sample in 0..5 {
                let (old, new) = if sample % 2 == 0 {
                    let old = measure(false);
                    (old, measure(true))
                } else {
                    let new = measure(true);
                    (measure(false), new)
                };
                println!(
                    "TIMING label={label} sample={sample} paths={} queries={} repetitions=3 reference_ns={} binary_ns={} hits={}",
                    files.len(),
                    queries.len(),
                    old.0,
                    new.0,
                    new.1
                );
            }
        }
    }
}

#[cfg(test)]
mod registry_scope_tests {
    use super::*;

    fn fixture(name: &str) -> (std::path::PathBuf, Registry) {
        let root =
            std::env::temp_dir().join(format!("mova-registry-{name}-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        fs::create_dir_all(root.join("models/root.groupant")).unwrap();
        assert!(
            Command::new("git")
                .args(["init", "-q"])
                .arg(&root)
                .status()
                .unwrap()
                .success()
        );
        fs::write(root.join("a.rs"), "fn a() {}\n").unwrap();
        let node = json!({"id":"a","source":"a.rs","description":"finite fixture","state":"files",
            "cost":"bounded","forward":["source to declaration"],"inverse":["compatible sources"],
            "checks":["synthetic files"],"limitations":["not semantic proof"]});
        let mut group = node.clone();
        group["id"] = json!("root");
        group.as_object_mut().unwrap().remove("source");
        group["children"] = json!(["a"]);
        let registry =
            serde_json::from_value(json!({"schema":1,"root":"root","nodes":[group,node]})).unwrap();
        let components = json!({
            "ast":{"status":"not_materialized"},"approximant":{"status":"not_materialized"},
            "invariant":{"status":"not_materialized"},"descriptant":{"status":"not_materialized"},
            "obernants":[{"id":"b","type":"behavior_complement","status":"not_materialized"},
                {"id":"r","type":"reverse_reconstruction","status":"not_materialized"}]});
        let manifest = json!({"schema":1,"kind":"groupant_group","id":"root","status":"partial",
            "children":["a"],"source_bindings":[{"path":"a.rs","sha256":sha(b"fn a() {}\n")}],
            "components":components});
        fs::write(
            root.join("models/root.groupant/manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        (root, registry)
    }

    #[test]
    fn group_directory_override_preserves_default_and_checks_confinement() {
        let (root, mut registry) = fixture("directory");
        let original = groupants(&root, &registry, "hash").unwrap();
        assert_eq!(original["valid"], true);
        assert_eq!(original["group_sidecars"], 1);
        assert!(
            serde_json::to_value(&registry).unwrap()["nodes"][0]
                .get("groupant_path")
                .is_none()
        );
        fs::create_dir_all(root.join("scoped")).unwrap();
        fs::rename(
            root.join("models/root.groupant"),
            root.join("scoped/owned.groupant"),
        )
        .unwrap();
        fs::create_dir_all(root.join("models/root.groupant")).unwrap();
        fs::write(
            root.join("models/root.groupant/manifest.json"),
            b"invalid default",
        )
        .unwrap();
        registry.nodes[0].groupant_path = Some("scoped/owned.groupant".into());
        let encoded = serde_json::to_vec(&registry).unwrap();
        let decoded: Registry = serde_json::from_slice(&encoded).unwrap();
        let report = groupants(&root, &decoded, "hash").unwrap();
        assert_eq!(report["valid"], true, "{report}");
        assert_eq!(report["group_sidecars"], 1);
        assert_eq!(
            report["logical_tree"][1]["sidecar"],
            "scoped/owned.groupant"
        );
        assert_eq!(report, groupants(&root, &decoded, "hash").unwrap());

        for path in [
            "",
            "/escape.groupant",
            "../escape.groupant",
            "scoped/../escape.groupant",
            "bad.txt",
        ] {
            registry.nodes[0].groupant_path = Some(path.into());
            assert!(validate(&registry).is_err(), "{path}");
        }
        registry.nodes[0].groupant_path = Some("scoped/owned.groupant".into());
        registry.nodes[1].groupant_path = Some("scoped/leaf.groupant".into());
        assert!(validate(&registry).is_err());
        registry.nodes[1].groupant_path = None;
        fs::write(root.join("a.rs"), "fn changed() {}\n").unwrap();
        assert_eq!(groupants(&root, &registry, "hash").unwrap()["valid"], false);
        fs::write(root.join("a.rs"), "fn a() {}\n").unwrap();
        #[cfg(unix)]
        {
            let outside = root.with_extension("outside");
            if outside.exists() {
                fs::remove_dir_all(&outside).unwrap();
            }
            fs::rename(root.join("scoped/owned.groupant"), &outside).unwrap();
            std::os::unix::fs::symlink(&outside, root.join("scoped/owned.groupant")).unwrap();
            let escaped = groupants(&root, &registry, "hash").unwrap();
            assert_eq!(escaped["valid"], false);
            assert!(
                escaped["diagnostics"]
                    .to_string()
                    .contains("not a file inside the repository")
            );
            fs::remove_dir_all(outside).unwrap();
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn composition_preserves_inputs_and_rejects_invalid_union_trees() {
        let (root, base) = fixture("compose");
        let mut parent = base.nodes[0].clone();
        parent.id = "union".into();
        parent.children = vec!["root".into(), "b".into()];
        let mut leaf = base.nodes[1].clone();
        leaf.id = "b".into();
        leaf.source = Some("b.rs".into());
        let overlay = Registry {
            schema: 1,
            root: "union".into(),
            nodes: vec![parent, leaf],
        };
        assert!(
            validate(&overlay).is_err(),
            "base root intentionally external"
        );
        let before = (
            serde_json::to_vec(&base).unwrap(),
            serde_json::to_vec(&overlay).unwrap(),
        );
        let composed = compose_registry(&base, &overlay).unwrap();
        assert_eq!(composed.root, "union");
        assert_eq!(
            composed
                .nodes
                .iter()
                .map(|n| n.id.as_str())
                .collect::<Vec<_>>(),
            ["root", "a", "union", "b"]
        );
        assert_eq!(
            serde_json::to_vec(&composed).unwrap(),
            serde_json::to_vec(&compose_registry(&base, &overlay).unwrap()).unwrap()
        );
        assert_eq!(
            before,
            (
                serde_json::to_vec(&base).unwrap(),
                serde_json::to_vec(&overlay).unwrap()
            )
        );
        for case in 0..8 {
            let mut bad = overlay.clone();
            match case {
                0 => bad.nodes[1].id = "a".into(),
                1 => bad.nodes[1].source = Some("a.rs".into()),
                2 => bad.nodes[0].children.push("missing".into()),
                3 => {
                    bad.nodes[0].children.pop();
                }
                4 => bad.schema = 2,
                5 => bad.nodes[0].children.push("union".into()),
                6 => bad.root = "missing".into(),
                _ => bad.nodes[0].children.push("root".into()),
            }
            assert!(compose_registry(&base, &bad).is_err(), "case {case}");
        }
        let mut bad_base = base.clone();
        bad_base.schema = 2;
        assert!(compose_registry(&bad_base, &overlay).is_err());
        let mut bad_base = base.clone();
        bad_base.nodes[0].children.push("b".into());
        assert!(
            compose_registry(&bad_base, &overlay).is_err(),
            "overlay must not repair invalid base"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn unowned_sidecar_without_registry_pointer_can_join_a_composed_registry() {
        let (root, mut registry) = fixture("composition");
        fs::write(root.join("b.rs"), "fn b() {}\n").unwrap();
        fs::create_dir_all(root.join("b.rs.groupant")).unwrap();
        let group_path = root.join("models/root.groupant/manifest.json");
        let mut group: Value = serde_json::from_slice(&fs::read(&group_path).unwrap()).unwrap();
        let mut manifest = json!({"schema":1,"kind":"groupant","id":"b","status":"partial",
            "source":{"path":"../b.rs","sha256":sha(b"fn b() {}\n")},"components":group["components"]});
        let path = root.join("b.rs.groupant/manifest.json");
        fs::write(&path, serde_json::to_vec(&manifest).unwrap()).unwrap();
        let partial = groupants(&root, &registry, "hash").unwrap();
        assert_eq!(partial["valid"], true, "{partial}");
        assert_eq!(partial["unregistered_files"], 1);
        assert_eq!(partial["files"][1]["model"], Value::Null);
        assert_eq!(partial["files"][1]["manifest"]["id"], "b");
        manifest["registry"] = json!({"node":"b"});
        fs::write(&path, serde_json::to_vec(&manifest).unwrap()).unwrap();
        assert_eq!(groupants(&root, &registry, "hash").unwrap()["valid"], false);
        let mut node = registry.nodes[1].clone();
        node.id = "b".into();
        node.source = Some("b.rs".into());
        registry.nodes.push(node);
        registry.nodes[0].children.push("b".into());
        group["children"] = json!(["a", "b"]);
        group["source_bindings"]
            .as_array_mut()
            .unwrap()
            .push(json!({"path":"b.rs","sha256":sha(b"fn b() {}\n")}));
        fs::write(group_path, serde_json::to_vec(&group).unwrap()).unwrap();
        let joined = groupants(&root, &registry, "union-hash").unwrap();
        assert_eq!(joined["valid"], true, "{joined}");
        assert_eq!(joined["unregistered_files"], 0);
        assert_eq!(joined["files"][1]["model"], "b");
        assert_eq!(
            inventory(&root, &registry, "union-hash").unwrap()["all_rust_files_registered"],
            true
        );
        fs::remove_dir_all(root).unwrap();
    }
}

pub mod consolidate;
pub mod sections;
pub mod undate;
pub mod links;
