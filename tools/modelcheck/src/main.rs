//! CLI for the model-registry inventory and Lean-table extraction.
//! It runs no teacher, training or declared test commands.
use anyhow::{Result, anyhow, bail, ensure};
use mova_modelcheck::{Registry, changed_inventory, extract_table, inventory, sha, write_new};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::PathBuf;
use std::{env, fs, path::Path};
mod publication;
fn main() -> Result<()> {
    let a: Vec<_> = env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("compose-registry") if a.len() == 4 => {
            let base: Registry = serde_json::from_slice(&fs::read(&a[1])?)?;
            let overlay: Registry = serde_json::from_slice(&fs::read(&a[2])?)?;
            let composed = mova_modelcheck::compose_registry(&base, &overlay)?;
            write_new(Path::new(&a[3]), &serde_json::to_vec_pretty(&composed)?)
        }
        Some("consolidate") if a.len() == 4 || (a.len() == 5 && a[3] == "--dry-run") => {
            let dry_run = a.len() == 5;
            let out = Path::new(&a[a.len() - 1]);
            ensure!(!out.exists(), "report must be a new file");
            let report =
                mova_modelcheck::consolidate::consolidate(Path::new(&a[1]), &a[2], dry_run)?;
            write_new(out, &serde_json::to_vec_pretty(&report)?)?;
            println!(
                "{}",
                json!({"dry_run":dry_run,"merged_sidecars":report["merged_sidecars"],
                "unmerged":report["unmerged"].as_object().map_or(0, |m| m.len()),
                "former_files":report["former_files"],"hash_rebinds":report["hash_rebinds"].as_array().map_or(0, Vec::len),
                "stale_kept":report["stale_hash_references_kept"].as_array().map_or(0, Vec::len),
                "newly_stale_unhandled":report["newly_stale_unhandled"].as_array().map_or(0, Vec::len)})
            );
            Ok(())
        }
        Some("undate") if a.len() == 3 || (a.len() == 4 && a[2] == "--dry-run") => {
            let dry_run = a.len() == 4;
            let out = Path::new(&a[a.len() - 1]);
            ensure!(!out.exists(), "report must be a new file");
            let report = mova_modelcheck::undate::run(Path::new(&a[1]), dry_run)?;
            write_new(out, &serde_json::to_vec_pretty(&report)?)?;
            let n = |k: &str| report[k].as_array().map_or(0, Vec::len);
            println!(
                "{}",
                json!({"dry_run":dry_run,"model_files":report["model_files"],
                "changed_model_files":report["changed_model_files"],"moved_blocks":report["moved_blocks"],
                "moved_units":report["moved_units"],"renamed_headings":report["renamed_headings"],
                "protected_dated_h2":report["protected_dated_h2"],"json_files_changed":n("json_files_changed"),
                "hash_rebinds":n("hash_rebinds"),"stale_untouched":n("stale_bindings_untouched")})
            );
            Ok(())
        }
        Some("consolidate-check") if a.len() == 4 => {
            let report = mova_modelcheck::consolidate::check(Path::new(&a[1]), &a[2])?;
            write_new(Path::new(&a[3]), &serde_json::to_vec_pretty(&report)?)?;
            println!(
                "{}",
                json!({"valid":report["valid"],"sidecars":report["sidecars"],"originals":report["originals"],
                "failures":report["failures"].as_array().map_or(0, Vec::len)})
            );
            ensure!(
                report["valid"] == true,
                "consolidation check failed; see report"
            );
            Ok(())
        }
        Some("sidecar-links") if a.len() == 3 => {
            let report = mova_modelcheck::links::check(Path::new(&a[1]))?;
            write_new(Path::new(&a[2]), &serde_json::to_vec_pretty(&report)?)?;
            println!(
                "{}",
                json!({"valid":report["valid"],"scanned_markdown_files":report["scanned_markdown_files"],
                "files_with_dangling":report["files_with_dangling"],"dangling_references":report["dangling_references"]})
            );
            ensure!(
                report["valid"] == true,
                "dangling sidecar references; see report"
            );
            Ok(())
        }
        Some("relink-evidence") if a.len() == 3 || (a.len() == 4 && a[2] == "--dry-run") => {
            let dry_run = a.len() == 4;
            let out = Path::new(&a[a.len() - 1]);
            ensure!(!out.exists(), "report must be a new file");
            let report = mova_modelcheck::links::relink(Path::new(&a[1]), dry_run)?;
            write_new(out, &serde_json::to_vec_pretty(&report)?)?;
            println!(
                "{}",
                json!({"dry_run":dry_run,"changed_files":report["changed_files"].as_array().map_or(0, Vec::len),
                "rewrites":report["rewrites"],"rewrites_by_via":report["rewrites_by_via"],
                "unresolved":report["unresolved"]["dangling_references"]})
            );
            Ok(())
        }
        Some("syntax") if a.len() == 4 => {
            let report = mova_modelcheck::syntax::project(Path::new(&a[1]), &a[2])?;
            write_new(Path::new(&a[3]), &serde_json::to_vec_pretty(&report)?)
        }
        Some("groupants") if a.len() == 4 => {
            let bytes = fs::read(&a[2])?;
            let registry: Registry = serde_json::from_slice(&bytes)?;
            let report = mova_modelcheck::groupants(Path::new(&a[1]), &registry, &sha(&bytes))?;
            write_new(Path::new(&a[3]), &serde_json::to_vec_pretty(&report)?)?;
            println!(
                "{}",
                serde_json::json!({"valid": report["valid"],
                "sidecars":report["sidecars"],"missing_sidecars":report["missing_sidecars"],
                "group_sidecars":report["group_sidecars"],"missing_group_sidecars":report["missing_group_sidecars"],
                "diagnostics":report["diagnostics"],"verification_inferred":false})
            );
            ensure!(
                report["valid"] == true,
                "invalid groupant declarations; see report diagnostics"
            );
            Ok(())
        }
        Some("changed") if a.len() == 5 => {
            let bytes = fs::read(&a[2])?;
            let registry: Registry = serde_json::from_slice(&bytes)?;
            let report = changed_inventory(Path::new(&a[1]), &registry, &sha(&bytes), &a[3])?;
            write_new(Path::new(&a[4]), &serde_json::to_vec_pretty(&report)?)?;
            println!(
                "{}",
                serde_json::json!({"changed":report["changed_rust_files"],"missing":report["missing_changed_files"]})
            );
            ensure!(
                report["all_changed_files_registered"] == true,
                "changed Rust files without model contracts remain"
            );
            Ok(())
        }
        Some("inventory") if a.len() == 4 || a.len() == 5 => {
            let require = a.get(4).map(String::as_str);
            ensure!(
                require.is_none() || require == Some("--require-all"),
                "unknown inventory option"
            );
            let bytes = fs::read(&a[2])?;
            let r: Registry = serde_json::from_slice(&bytes)?;
            let report = inventory(Path::new(&a[1]), &r, &sha(&bytes))?;
            write_new(Path::new(&a[3]), &serde_json::to_vec_pretty(&report)?)?;
            println!(
                "{}",
                serde_json::json!({"rust_files":report["rust_files"],"registered":report["registered_files"],
    "missing":report["missing_files"],"full_system_verification":false})
            );
            ensure!(
                require.is_none() || report["all_rust_files_registered"] == true,
                "Rust files without model contracts remain"
            );
            Ok(())
        }
        Some("extract-table") if a.len() == 3 => {
            let table = extract_table(&fs::read_to_string(&a[1])?)?;
            write_new(Path::new(&a[2]), table.as_bytes())
        }
        Some("publication-catalog") if a.len() == 4 => {
            let store = store_root(&a[1], env::var_os("MOVA_GROUPANTS"))?;
            let policy = if a[2] == "-" {
                None
            } else {
                Some(Path::new(&a[2]))
            };
            let result = publication::catalog(&store, policy)?;
            publication::catalog_destination(&store, Path::new(&a[3]))?;
            write_new(Path::new(&a[3]), &serde_json::to_vec_pretty(&result)?)?;
            println!(
                "{}",
                json!({"manifests":result["inventory"]["manifests"],"eligible_global_policy_decisions":result["eligible_global_policy_decisions"],"requires_publication_check":true,"proof":false})
            );
            Ok(())
        }
        Some("publication-check") if a.len() == 6 || a.len() == 8 => {
            let store = store_root(&a[2], env::var_os("MOVA_GROUPANTS"))?;
            let report = if a.len() == 8 {
                publication::check_with_policy(
                    Path::new(&a[1]),
                    &store,
                    Path::new(&a[3]),
                    &a[4],
                    Some((Path::new(&a[5]), &a[6])),
                )?
            } else {
                publication::check(Path::new(&a[1]), &store, Path::new(&a[3]), &a[4])?
            };
            let out = Path::new(if a.len() == 8 { &a[7] } else { &a[5] });
            let parent = out.parent().filter(|p| !p.as_os_str().is_empty());
            let parent = fs::canonicalize(parent.unwrap_or(Path::new(".")))?;
            ensure!(
                !parent.starts_with(fs::canonicalize(&store)?),
                "publication report must remain outside the store"
            );
            write_new(out, &serde_json::to_vec_pretty(&report)?)?;
            println!("{report}");
            Ok(())
        }
        Some("library") if a.len() == 5 => {
            let store = store_root(&a[2], env::var_os("MOVA_GROUPANTS"))?;
            let report = library(Path::new(&a[1]), &store, &a[3])?;
            let out = Path::new(&a[4]);
            let parent = out.parent().filter(|p| !p.as_os_str().is_empty());
            let parent = fs::canonicalize(parent.unwrap_or(Path::new(".")))?;
            ensure!(
                !parent.starts_with(fs::canonicalize(&store)?),
                "report must not be written into the store"
            );
            write_new(out, &serde_json::to_vec_pretty(&report)?)?;
            println!("{report}");
            Ok(())
        }
        _ => bail!(
            "usage: mova-modelcheck publication-catalog STORE REVIEWS|- NEW_CATALOG | publication-check REPO STORE CATALOG REL_MANIFEST [REVIEWS EXPECTED_REVIEWS_SHA] NEW_REPORT | compose-registry BASE_REGISTRY OVERLAY NEW_JSON | syntax REPO SOURCE NEW_JSON | groupants REPO REGISTRY NEW_JSON | changed REPO REGISTRY BASE NEW_JSON | inventory REPO REGISTRY NEW_JSON [--require-all] | library REPO STORE|- RELATIVE_MANIFEST NEW_JSON | undate REPO [--dry-run] NEW_REPORT | sidecar-links REPO NEW_JSON | relink-evidence REPO [--dry-run] NEW_REPORT | extract-table LEAN_LOG NEW_TABLE"
        ),
    }
}

// External groupant library: one manifest, read-only, no scans or execution.
const MAX_FILE: u64 = 16 << 20;
const MAX_ARTIFACTS: usize = 256;

/// `-` selects MOVA_GROUPANTS; a missing or empty variable is an error, never a default.
fn store_root(arg: &str, var: Option<std::ffi::OsString>) -> Result<PathBuf> {
    if arg != "-" {
        return Ok(PathBuf::from(arg));
    }
    match var {
        Some(v) if !v.is_empty() => Ok(PathBuf::from(v)),
        _ => bail!("STORE is - but MOVA_GROUPANTS is not set"),
    }
}

/// Resolves a relative path below `base`, rejecting absolute, `..` and symlink escapes.
fn confined(base: &Path, rel: &str) -> Result<PathBuf> {
    ensure!(
        !rel.starts_with('/') && rel.split('/').all(|s| !matches!(s, "" | "." | "..")),
        "path must be relative without empty, . or .. segments: {rel}"
    );
    let full = fs::canonicalize(base.join(rel))?;
    ensure!(full.starts_with(base), "path escapes its root: {rel}");
    Ok(full)
}

fn read_limited(path: &Path) -> Result<Vec<u8>> {
    let meta = fs::metadata(path)?;
    ensure!(meta.is_file(), "not a regular file: {}", path.display());
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(MAX_FILE + 1)
        .read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 <= MAX_FILE,
        "file exceeds 16 MiB: {}",
        path.display()
    );
    Ok(bytes)
}

fn text<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    match v.get(key).and_then(Value::as_str) {
        Some(s) if !s.is_empty() => Ok(s),
        _ => bail!("missing nonempty string field {key}"),
    }
}

/// Checks a repository path against its declared hash; the hash is never refreshed.
fn bind(repo: &Path, path: &str, want: &str) -> Result<Vec<u8>> {
    let bytes = read_limited(&confined(repo, path)?)?;
    ensure!(sha(&bytes) == want, "source hash mismatch: {path}");
    Ok(bytes)
}

/// Every `path` string inside components must name a declared artifact.
fn component_paths(v: &Value, artifacts: &BTreeMap<String, String>) -> Result<()> {
    match v {
        Value::Object(m) => {
            if let Some(p) = m.get("path") {
                let p = p.as_str().unwrap_or_default();
                ensure!(
                    artifacts.contains_key(p),
                    "component path is not an artifact: {p}"
                );
            }
            m.values().try_for_each(|x| component_paths(x, artifacts))
        }
        Value::Array(xs) => xs.iter().try_for_each(|x| component_paths(x, artifacts)),
        _ => Ok(()),
    }
}

fn library(repo: &Path, store: &Path, rel: &str) -> Result<Value> {
    let (repo, store) = (fs::canonicalize(repo)?, fs::canonicalize(store)?);
    ensure!(
        !store.starts_with(&repo) && !repo.starts_with(&store),
        "groupant store must be outside the repository"
    );
    ensure!(
        fs::symlink_metadata(store.join(".git")).is_err(),
        "groupant store must not be a Git checkout"
    );
    let manifest_path = confined(&store, rel)?;
    let raw = read_limited(&manifest_path)?;
    let m: Value = serde_json::from_slice(&raw)?;
    ensure!(m["schema"] == 1, "manifest schema must be 1");
    let (id, kind) = (text(&m, "id")?, text(&m, "kind")?);
    ensure!(
        m.get("proof").is_none_or(|p| p == false),
        "external manifests cannot promote proof status"
    );
    let bundle = manifest_path.parent().expect("file has a parent");
    let list = m["artifacts"]
        .as_array()
        .ok_or_else(|| anyhow!("artifacts must be an array"))?;
    ensure!(
        list.len() <= MAX_ARTIFACTS,
        "more than {MAX_ARTIFACTS} artifacts"
    );
    let mut artifacts = BTreeMap::new();
    for item in list {
        let (path, want) = (text(item, "path")?, text(item, "sha256")?);
        ensure!(
            artifacts
                .insert(path.to_string(), want.to_string())
                .is_none(),
            "duplicate artifact path: {path}"
        );
        ensure!(
            sha(&read_limited(&confined(bundle, path)?)?) == want,
            "artifact hash mismatch: {path}"
        );
    }
    let local = |key: &str| -> Result<()> {
        let p = text(&m, key)?;
        ensure!(
            artifacts.contains_key(p),
            "{key} is not a local artifact: {p}"
        );
        Ok(())
    };
    let mut source_scope = "repository source bindings checked";
    let bindings = match kind {
        "source_survey_groupant" => {
            ensure!(
                m["status"] == "generated_source_survey_unverified",
                "survey must retain generated unverified status"
            );
            let source = &m["source"];
            text(source, "id")?;
            let digest = text(source, "sha256")?;
            ensure!(
                digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit()),
                "survey source needs a SHA-256 digest"
            );
            let source_kind = text(source, "kind")?;
            let format = text(source, "format")?;
            ensure!(
                matches!(
                    (source_kind, format),
                    ("algorithm", "rust") | ("paper", "markdown" | "epub")
                ),
                "unsupported survey source kind/format"
            );
            ensure!(m["components"].is_object(), "components must be an object");
            component_paths(&m["components"], &artifacts)?;
            for name in ["ast", "proof"] {
                ensure!(
                    m["components"][name]["status"] == "missing"
                        && m["components"][name].get("path").is_none(),
                    "generated survey must keep {name} missing"
                );
            }
            let description = text(&m["components"]["description"], "path")?;
            let bytes = read_limited(&confined(bundle, description)?)?;
            let description_text = std::str::from_utf8(&bytes)?;
            let scalars = description_text.chars().count();
            let words = description_text
                .split_whitespace()
                .filter(|token| token.chars().any(char::is_alphanumeric))
                .count();
            let profile = m
                .get("description_profile")
                .map(|v| {
                    v.as_str()
                        .ok_or_else(|| anyhow!("description profile must be a string"))
                })
                .transpose()?
                .unwrap_or("chars_4000_5000_v1");
            let (count, unit, tokenization) = match profile {
                "words_4000_5000_v1" => (
                    words,
                    "words",
                    "unicode_whitespace_tokens_containing_unicode_alphanumeric_v1",
                ),
                "chars_4000_5000_v1" => (scalars, "unicode_scalars", "unicode_scalars_v1"),
                _ => bail!("unsupported description profile"),
            };
            ensure!(
                (4000..=5000).contains(&count),
                "survey description must contain 4000..5000 declared units"
            );
            ensure!(
                m["description_unicode_scalars"].as_u64() == Some(scalars as u64),
                "survey Unicode scalar count mismatch"
            );
            if profile == "words_4000_5000_v1" || m.get("description_measure").is_some() {
                let metric = &m["description_measure"];
                ensure!(
                    metric["unit"] == unit
                        && metric["tokenization"] == tokenization
                        && metric["count"].as_u64() == Some(count as u64)
                        && metric["minimum"] == 4000
                        && metric["maximum"] == 5000,
                    "survey description measure mismatch"
                );
                ensure!(
                    m["description_words"].as_u64() == Some(words as u64),
                    "survey word count mismatch"
                );
            }
            match source_kind {
                "algorithm" => {
                    bind(&repo, text(source, "repository_path")?, digest)?;
                    source_scope =
                        "algorithm repository source hash checked; generated claims unverified";
                    1
                }
                "paper" => {
                    ensure!(
                        source.get("repository_path").is_none(),
                        "paper survey cannot imply an unchecked repository binding"
                    );
                    source_scope = "paper declared provenance only; original full text and scientific claims not checked";
                    0
                }
                _ => unreachable!(),
            }
        }
        "algorithm_groupant_bundle" => {
            let source = &m["source"];
            let want = text(source, "sha256")?;
            let bytes = bind(&repo, text(source, "repository_path")?, want)?;
            let snapshot = text(source, "snapshot")?;
            ensure!(
                artifacts.get(snapshot).map(String::as_str) == Some(want),
                "snapshot differs from source"
            );
            ensure!(
                read_limited(&confined(bundle, snapshot)?)? == bytes,
                "snapshot differs from source"
            );
            let canonical = &m["canonical_groupant"];
            bind(
                &repo,
                text(canonical, "repository_path")?,
                text(canonical, "sha256")?,
            )?;
            local("bibliography")?;
            local("evidence")?;
            ensure!(m["components"].is_object(), "components must be an object");
            component_paths(&m["components"], &artifacts)?;
            2
        }
        "paper_groupant" => {
            let doi = text(&m, "doi")?;
            ensure!(
                doi.strip_prefix("10.")
                    .and_then(|d| d.split_once('/'))
                    .is_some_and(|(r, s)| !r.is_empty() && !s.is_empty()),
                "doi must look like 10.<registrant>/<suffix>"
            );
            text(&m, "access_scope")?;
            let list = m["source_bindings"].as_array().filter(|l| !l.is_empty());
            let list = list.ok_or_else(|| anyhow!("source_bindings must be a nonempty array"))?;
            ensure!(
                list.len() <= MAX_ARTIFACTS,
                "more than {MAX_ARTIFACTS} source bindings"
            );
            for b in list {
                bind(&repo, text(b, "path")?, text(b, "sha256")?)?;
            }
            let components = m["components"]
                .as_object()
                .ok_or_else(|| anyhow!("components must be an object"))?;
            for (name, c) in components {
                ensure!(
                    c.get("path").is_some() != (c["status"] == "missing"),
                    "component {name} needs an artifact path or an explicit missing status"
                );
            }
            component_paths(&m["components"], &artifacts)?;
            list.len()
        }
        _ => bail!("unsupported manifest kind: {kind}"),
    };
    Ok(
        json!({"valid": true, "proof": false, "id": id, "kind": kind, "manifest": rel,
        "manifest_sha256": sha(&raw), "assets": artifacts.len(), "source_bindings": bindings, "source_check_scope": source_scope}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    fn temp(tag: &str) -> PathBuf {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let d = env::temp_dir().join(format!(
            "modelcheck-library-{}-{tag}-{n}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }
    fn put(root: &Path, rel: &str, bytes: &[u8]) -> String {
        let p = root.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, bytes).unwrap();
        sha(bytes)
    }
    /// Returns (repo, store) with one admitted bundle at `b/manifest.json`.
    fn fixture(kind: &str) -> (PathBuf, PathBuf, Value) {
        let (repo, store) = (temp("repo"), temp("store"));
        let src = put(&repo, "src/a.rs", b"fn a() {}\n");
        let canon = put(&repo, "src/a.rs.groupant/manifest.json", b"{}");
        let mut artifacts = vec![];
        for (p, b) in [
            ("source.rs.txt", &b"fn a() {}\n"[..]),
            ("bib.json", b"[]"),
            ("ev.json", b"{}"),
            ("m.md", b"# m"),
        ] {
            artifacts.push(json!({"path": p, "sha256": put(&store, &format!("b/{p}"), b)}));
        }
        let m = if kind == "paper" {
            json!({"schema": 1, "kind": "paper_groupant", "id": "paper-x", "doi": "10.1000/x",
                "access_scope": "open", "artifacts": artifacts,
                "source_bindings": [{"path": "src/a.rs", "sha256": src}],
                "components": {"descriptant": {"path": "m.md"}, "approximant": {"status": "missing"}}})
        } else {
            json!({"schema": 1, "kind": "algorithm_groupant_bundle", "id": "algorithm-a", "artifacts": artifacts,
                "source": {"repository_path": "src/a.rs", "sha256": src, "snapshot": "source.rs.txt"},
                "canonical_groupant": {"repository_path": "src/a.rs.groupant/manifest.json", "sha256": canon},
                "bibliography": "bib.json", "evidence": "ev.json",
                "components": {"obernants": [{"id": "x", "path": "m.md"}]}})
        };
        put(&store, "b/manifest.json", &serde_json::to_vec(&m).unwrap());
        (repo, store, m)
    }
    fn rewrite(store: &Path, m: &Value) {
        fs::write(
            store.join("b/manifest.json"),
            serde_json::to_vec(m).unwrap(),
        )
        .unwrap();
    }
    fn survey(kind: &str) -> (PathBuf, PathBuf, Value) {
        let (repo, store, mut m) = fixture("algorithm");
        let description = "λ".repeat(4500);
        let digest = put(&store, "b/description.md", description.as_bytes());
        m["kind"] = json!("source_survey_groupant");
        m["status"] = json!("generated_source_survey_unverified");
        m["description_unicode_scalars"] = json!(4500);
        m["artifacts"]
            .as_array_mut()
            .unwrap()
            .push(json!({"path":"description.md","sha256":digest}));
        m["source"]["kind"] = json!(kind);
        m["source"]["id"] = json!("survey-source");
        m["source"]["format"] = json!(if kind == "paper" { "markdown" } else { "rust" });
        m["source"].as_object_mut().unwrap().remove("snapshot");
        if kind == "paper" {
            m["source"]
                .as_object_mut()
                .unwrap()
                .remove("repository_path");
        }
        m["components"] = json!({"description":{"path":"description.md","status":"generated_draft"},
            "ast":{"status":"missing"},"proof":{"status":"missing"}});
        rewrite(&store, &m);
        (repo, store, m)
    }
    #[test]
    fn survey_integrity_and_provenance_scopes_are_distinct() {
        for (kind, bindings) in [("algorithm", 1), ("paper", 0)] {
            let (repo, store, _) = survey(kind);
            let a = library(&repo, &store, "b/manifest.json").unwrap();
            let b = library(&repo, &store, "b/manifest.json").unwrap();
            assert_eq!(a, b);
            assert_eq!(a["source_bindings"], bindings);
            assert_eq!(a["proof"], false);
            if kind == "paper" {
                assert!(
                    a["source_check_scope"]
                        .as_str()
                        .unwrap()
                        .contains("not checked")
                );
            }
        }
    }
    #[test]
    fn survey_rejects_stale_source_hash_length_and_promoted_claims() {
        let (repo, store, _) = survey("algorithm");
        fs::write(repo.join("src/a.rs"), "changed").unwrap();
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("source hash")
        );
        let (repo, store, mut m) = survey("paper");
        let bad = "λ".repeat(3999);
        m["artifacts"].as_array_mut().unwrap().last_mut().unwrap()["sha256"] =
            json!(put(&store, "b/description.md", bad.as_bytes()));
        m["description_unicode_scalars"] = json!(3999);
        rewrite(&store, &m);
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("4000..5000")
        );
        let (repo, store, m) = survey("paper");
        for (field, value) in [("kind", json!("unknown")), ("format", json!("rust"))] {
            let mut bad = m.clone();
            bad["source"][field] = value;
            rewrite(&store, &bad);
            assert!(library(&repo, &store, "b/manifest.json").is_err());
        }
        let mut bad = m.clone();
        bad["components"]["proof"] = json!({"path":"m.md","status":"present"});
        rewrite(&store, &bad);
        assert!(library(&repo, &store, "b/manifest.json").is_err());
        let mut bad = m.clone();
        bad["description_unicode_scalars"] = json!(4501);
        rewrite(&store, &bad);
        assert!(library(&repo, &store, "b/manifest.json").is_err());
        rewrite(&store, &m);
        fs::write(store.join("b/description.md"), "tampered").unwrap();
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("artifact hash")
        );
    }

    #[test]
    fn word_surveys_use_explicit_metric_and_legacy_editions_keep_character_units() {
        let (repo, store, mut m) = survey("paper");
        let description = "модель ".repeat(4300);
        let digest = put(&store, "b/description.md", description.as_bytes());
        m["artifacts"].as_array_mut().unwrap().last_mut().unwrap()["sha256"] = json!(digest);
        m["description_unicode_scalars"] = json!(description.chars().count());
        m["description_words"] = json!(4300);
        m["description_profile"] = json!("words_4000_5000_v1");
        m["description_measure"] = json!({"unit":"words","count":4300,"minimum":4000,"maximum":5000,
            "tokenization":"unicode_whitespace_tokens_containing_unicode_alphanumeric_v1"});
        rewrite(&store, &m);
        assert_eq!(
            library(&repo, &store, "b/manifest.json").unwrap()["valid"],
            true
        );
        for unit in ["unicode_scalars", "bytes", "tokens"] {
            let mut bad = m.clone();
            bad["description_measure"]["unit"] = json!(unit);
            rewrite(&store, &bad);
            assert!(library(&repo, &store, "b/manifest.json").is_err());
        }
        let mut bad = m.clone();
        bad.as_object_mut().unwrap().remove("description_profile");
        rewrite(&store, &bad);
        assert!(library(&repo, &store, "b/manifest.json").is_err()); // No inferred unit switch.
        let mut bad = m.clone();
        bad["description_words"] = json!(4299);
        rewrite(&store, &bad);
        assert!(library(&repo, &store, "b/manifest.json").is_err());
        let mut bad = m.clone();
        bad["description_profile"] = json!("words_vague");
        rewrite(&store, &bad);
        assert!(library(&repo, &store, "b/manifest.json").is_err());
        let (repo, store, _) = survey("paper");
        assert!(library(&repo, &store, "b/manifest.json").is_ok()); // Historical absent profile.
    }

    #[test]
    fn admits_algorithm_and_paper_without_proof() {
        for (kind, n) in [("algorithm", 2), ("paper", 1)] {
            let (repo, store, _) = fixture(kind);
            let r = library(&repo, &store, "b/manifest.json").unwrap();
            assert_eq!(
                (r["valid"].clone(), r["proof"].clone()),
                (json!(true), json!(false))
            );
            assert_eq!(
                (r["assets"].clone(), r["source_bindings"].clone()),
                (json!(4), json!(n))
            );
        }
    }
    #[test]
    fn rejects_tamper_mismatch_and_fabricated_components() {
        let (repo, store, _) = fixture("algorithm");
        fs::write(store.join("b/m.md"), "# changed").unwrap();
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("artifact hash")
        );
        let (repo, store, _) = fixture("paper");
        fs::write(repo.join("src/a.rs"), "fn b() {}\n").unwrap();
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("source hash")
        );
        let (repo, store, mut m) = fixture("paper");
        m["components"]["approximant"] = json!({});
        rewrite(&store, &m);
        assert!(library(&repo, &store, "b/manifest.json").is_err());
        m["components"]["approximant"] = json!({"status": "missing"});
        m["proof"] = json!(true);
        rewrite(&store, &m);
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("proof")
        );
    }
    #[test]
    fn rejects_traversal_symlink_escape_and_git_store() {
        let (repo, store, mut m) = fixture("algorithm");
        for bad in ["../b/manifest.json", "/etc/passwd", "b/./manifest.json"] {
            assert!(library(&repo, &store, bad).is_err(), "{bad}");
        }
        let outside = temp("outside");
        put(&outside, "secret.md", b"# m");
        std::os::unix::fs::symlink(outside.join("secret.md"), store.join("b/link.md")).unwrap();
        m["artifacts"][3]["path"] = json!("link.md");
        rewrite(&store, &m);
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("escapes")
        );
        let (repo, store, _) = fixture("algorithm");
        fs::create_dir(store.join(".git")).unwrap();
        assert!(library(&repo, &store, "b/manifest.json").is_err());
        let inner = repo.join("store");
        fs::create_dir(&inner).unwrap();
        assert!(
            library(&repo, &inner, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("outside")
        );
    }
    #[test]
    fn rejects_store_ancestor_and_excess_bindings() {
        let outer = temp("outer");
        let repo = outer.join("repo");
        fs::create_dir(&repo).unwrap();
        assert!(
            library(&repo, &outer, "x.json")
                .unwrap_err()
                .to_string()
                .contains("outside")
        );
        let (repo, store, mut m) = fixture("paper");
        m["source_bindings"] = json!(vec![m["source_bindings"][0].clone(); MAX_ARTIFACTS + 1]);
        rewrite(&store, &m);
        assert!(
            library(&repo, &store, "b/manifest.json")
                .unwrap_err()
                .to_string()
                .contains("source bindings")
        );
    }
    #[test]
    fn missing_store_variable_is_not_defaulted() {
        assert!(store_root("-", None).is_err());
        assert!(store_root("-", Some("".into())).is_err());
        assert_eq!(
            store_root("-", Some("/x".into())).unwrap(),
            PathBuf::from("/x")
        );
    }
}
