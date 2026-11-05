//! Additive scope policy for one immutable external manifest version.
//! Publication policy, integrity and mathematical assurance are separate observations.
use anyhow::{Result, anyhow, bail, ensure};
use mova_modelcheck::sha;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Component, Path, PathBuf},
};
const MAX_JSON: u64 = 16 * 1024 * 1024;
const MAX_ENTRIES: usize = 50_000;
const MAX_SCAN: usize = 50_000;
fn text<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow!("missing nonempty {k}"))
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn normal(s: &str) -> bool {
    !s.is_empty()
        && !s.contains(['\\', '\0'])
        && !s.starts_with('/')
        && s.split('/').all(|p| !p.is_empty() && !p.starts_with('.'))
        && Path::new(s)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
fn read(p: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    ensure!(
        m.is_file() && !m.file_type().is_symlink(),
        "publication input must be a regular nonsymlink file"
    );
    let mut b = Vec::new();
    fs::File::open(p)?.take(MAX_JSON + 1).read_to_end(&mut b)?;
    ensure!(b.len() as u64 <= MAX_JSON, "publication input exceeds16MiB");
    Ok(b)
}
fn load(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&read(p)?)?)
}
fn store_root(p: &Path) -> Result<PathBuf> {
    let m = fs::symlink_metadata(p)?;
    ensure!(
        m.is_dir() && !m.file_type().is_symlink(),
        "publication store must be a real directory"
    );
    let p = fs::canonicalize(p)?;
    for ancestor in p.ancestors() {
        match fs::symlink_metadata(ancestor.join(".git")) {
            Ok(_) => bail!("publication store must remain outside any Git ancestor"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(p)
}
fn confined(base: &Path, rel: &str) -> Result<PathBuf> {
    ensure!(normal(rel), "publication path must be normal and relative");
    let mut p = base.to_path_buf();
    for part in Path::new(rel).components() {
        let Component::Normal(part) = part else {
            bail!("publication path component")
        };
        p.push(part);
        ensure!(
            !fs::symlink_metadata(&p)?.file_type().is_symlink(),
            "publication symlinks are excluded"
        );
    }
    ensure!(p.starts_with(base), "publication path escaped root");
    Ok(p)
}
fn scope(v: &Value) -> Option<&str> {
    v["library_scope"]
        .as_str()
        .filter(|s| matches!(*s, "global" | "mova_implementation" | "unreviewed"))
}
fn implementation(m: &Value) -> bool {
    match m["kind"].as_str() {
        Some("algorithm_groupant_bundle") => true,
        Some("paper_groupant") => m["source_bindings"]
            .as_array()
            .is_some_and(|s| !s.is_empty()),
        Some("source_survey_groupant") => {
            m["source"]["kind"] == "algorithm"
                || m["source"].get("repository_path").is_some()
                || m["source_bindings"]
                    .as_array()
                    .is_some_and(|s| !s.is_empty())
        }
        _ => false,
    }
}
fn supported_global(m: &Value) -> bool {
    m["schema"] == 1
        && m["kind"] == "source_survey_groupant"
        && m["status"] == "generated_source_survey_unverified"
        && m["source"]["kind"] == "paper"
        && !implementation(m)
}
fn entry_map(v: &Value) -> Result<BTreeMap<String, Value>> {
    let list = v["entries"]
        .as_array()
        .ok_or_else(|| anyhow!("entries must be an array"))?;
    ensure!(list.len() <= MAX_ENTRIES, "too many publication entries");
    let mut out = BTreeMap::new();
    for e in list {
        let p = text(e, "manifest")?;
        ensure!(
            normal(p) && p.ends_with("/manifest.json"),
            "entry manifest path is invalid"
        );
        ensure!(
            digest(text(e, "manifest_sha256")?),
            "entry needs exact manifest digest"
        );
        ensure!(
            out.insert(p.to_string(), e.clone()).is_none(),
            "duplicate/conflicting publication manifest identity"
        );
    }
    Ok(out)
}
#[derive(Default)]
struct ArtifactInventory {
    paths: BTreeSet<String>,
    hashes: BTreeSet<String>,
}
fn artifact_inventory(store: &Path, paths: &[String]) -> Result<ArtifactInventory> {
    let mut out = ArtifactInventory::default();
    let mut count = 0;
    for rel in paths {
        let m = load(&confined(store, rel)?)?;
        let artifacts = m["artifacts"]
            .as_array()
            .ok_or_else(|| anyhow!("complete artifact inventory unavailable"))?;
        ensure!(
            artifacts.len() <= 256,
            "artifact inventory per-manifest bound"
        );
        for a in artifacts {
            count += 1;
            ensure!(count <= MAX_SCAN, "artifact inventory bound exceeded");
            let p = text(a, "path")?;
            let h = text(a, "sha256")?;
            ensure!(normal(p) && digest(h), "invalid inventoried artifact");
            let joined = Path::new(rel).parent().unwrap().join(p);
            out.paths.insert(
                joined
                    .to_str()
                    .ok_or_else(|| anyhow!("artifact UTF8"))?
                    .to_owned(),
            );
            out.hashes.insert(h.to_owned());
        }
    }
    Ok(out)
}
fn evidence(store: &Path, e: &Value, inventory: &ArtifactInventory) -> Result<()> {
    let list = e["evidence"]
        .as_array()
        .filter(|x| !x.is_empty())
        .ok_or_else(|| anyhow!("global admission requires review evidence"))?;
    ensure!(list.len() <= 256, "too much review evidence");
    let mut seen = BTreeSet::new();
    for a in list {
        let p = text(a, "path")?;
        let want = text(a, "sha256")?;
        ensure!(
            normal(p) && digest(want) && seen.insert(p),
            "invalid or duplicate evidence pin"
        );
        ensure!(
            p.starts_with("reviews/"),
            "evidence must be in reserved reviews/"
        );
        let path = confined(store, p)?;
        let mut ancestor = path.parent();
        while let Some(dir) = ancestor {
            match fs::symlink_metadata(dir.join("manifest.json")) {
                Ok(_) => bail!("review evidence cannot have a manifest ancestor"),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
            if dir == store {
                break;
            }
            ancestor = dir.parent();
        }
        ensure!(
            !inventory.paths.contains(p) && !inventory.hashes.contains(want),
            "generated artifact cannot serve as independent review evidence"
        );
        ensure!(sha(&read(&path)?) == want, "review evidence hash mismatch");
    }
    Ok(())
}
fn allowed(store: &Path, manifest_rel: &str, m: &Value, e: &Value) -> Result<Vec<Value>> {
    let list = e["allowed_artifacts"]
        .as_array()
        .filter(|x| !x.is_empty())
        .ok_or_else(|| anyhow!("global admission requires explicit allowed_artifacts"))?;
    ensure!(list.len() <= 256, "too many allowed artifacts");
    let all = m["artifacts"]
        .as_array()
        .ok_or_else(|| anyhow!("manifest artifacts missing"))?;
    let mut declared = BTreeMap::new();
    for a in all {
        let p = text(a, "path")?;
        let h = text(a, "sha256")?;
        ensure!(
            normal(p) && digest(h) && declared.insert(p, h).is_none(),
            "invalid duplicate declared artifact"
        );
    }
    let bundle = Path::new(manifest_rel)
        .parent()
        .ok_or_else(|| anyhow!("manifest parent"))?;
    let mut seen = BTreeSet::new();
    let mut result = Vec::new();
    for a in list {
        let p = text(a, "path")?;
        let h = text(a, "sha256")?;
        ensure!(
            normal(p) && digest(h) && seen.insert(p),
            "invalid or duplicate allowed artifact"
        );
        ensure!(
            declared.get(p).copied() == Some(h),
            "allowed artifact is not the exact declared version"
        );
        // The existing paper-survey schema has exactly one admitted narrative role.
        ensure!(
            m["components"]["description"]["path"] == p,
            "only declared description narrative artifacts may be published"
        );
        ensure!(
            m["source"]["sha256"] != h && m["source"]["snapshot"] != p,
            "raw source bytes cannot be global publication artifacts"
        );
        let rel = bundle
            .join(p)
            .to_str()
            .ok_or_else(|| anyhow!("artifact UTF8"))?
            .to_owned();
        ensure!(
            sha(&read(&confined(store, &rel)?)?) == h,
            "allowed artifact hash mismatch"
        );
        result.push(json!({"path":p,"sha256":h}));
    }
    Ok(result)
}
fn global_policy(
    store: &Path,
    rel: &str,
    m: &Value,
    e: &Value,
    inventory: &ArtifactInventory,
) -> Result<Vec<Value>> {
    ensure!(scope(e) == Some("global"), "explicit global scope required");
    ensure!(
        !implementation(m),
        "Mova implementation snapshots cannot be promoted to global"
    );
    ensure!(supported_global(m), "unsupported global exposition kind");
    ensure!(
        e["editorial_status"] == "independently_reviewed",
        "independent editorial review required"
    );
    ensure!(
        e["rights"]["status"] == "permitted"
            && e["rights"]["public_export_allowed"] == true
            && e["public_export_allowed"] == true,
        "explicit reviewed publication rights required"
    );
    ensure!(
        m["public_export_allowed"] == true
            && m["local_only"] == false
            && m["license"]["local_only"] == false
            && m["license"]["public_export_allowed"] != false,
        "manifest rights/access do not affirm publication"
    );
    text(e, "reason")?;
    evidence(store, e, inventory)?;
    allowed(store, rel, m, e)
}
fn walk(
    store: &Path,
    dir: &Path,
    out: &mut Vec<String>,
    visited: &mut usize,
    depth: usize,
) -> Result<()> {
    ensure!(
        depth <= 16,
        "publication walk depth exceeded; inventory incomplete"
    );
    let mut entries = fs::read_dir(dir)?.collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        if e.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        *visited += 1;
        ensure!(
            *visited <= MAX_SCAN,
            "publication walk bound exceeded; inventory incomplete"
        );
        let ft = e.file_type()?;
        ensure!(
            !ft.is_symlink(),
            "publication walk rejects symlinks; inventory incomplete"
        );
        if ft.is_dir() {
            walk(store, &e.path(), out, visited, depth + 1)?
        } else if ft.is_file() && e.file_name() == "manifest.json" {
            ensure!(
                out.len() < MAX_ENTRIES,
                "publication manifest bound exceeded; inventory incomplete"
            );
            out.push(
                e.path()
                    .strip_prefix(store)?
                    .to_str()
                    .ok_or_else(|| anyhow!("manifest UTF8"))?
                    .to_owned(),
            );
        }
    }
    Ok(())
}
/// Classify a bounded snapshot without modifying any bundle or collection index.
pub(crate) fn catalog(store: &Path, reviews: Option<&Path>) -> Result<Value> {
    let store = store_root(store)?;
    let (policy, policy_sha) = if let Some(p) = reviews {
        let bytes = read(p)?;
        let v: Value = serde_json::from_slice(&bytes)?;
        ensure!(
            v["schema"] == 1 && v["kind"] == "groupant_publication_reviews",
            "unsupported review policy"
        );
        (entry_map(&v)?, Some(sha(&bytes)))
    } else {
        (BTreeMap::new(), None)
    };
    let mut paths = Vec::new();
    let mut visited = 0;
    walk(&store, &store, &mut paths, &mut visited, 0)?;
    paths.sort();
    let present = paths.iter().cloned().collect::<BTreeSet<_>>();
    ensure!(
        policy.keys().all(|p| present.contains(p)),
        "review policy names an absent manifest"
    );
    let artifact_inventory = if policy.values().any(|e| scope(e) == Some("global")) {
        artifact_inventory(&store, &paths)?
    } else {
        ArtifactInventory::default()
    };
    let mut entries = Vec::new();
    for rel in paths {
        let raw = read(&confined(&store, &rel)?)?;
        let m: Value = serde_json::from_slice(&raw)?;
        let digest = sha(&raw);
        let imp = implementation(&m);
        let mut e = json!({"manifest":rel,"manifest_sha256":digest,"id":m["id"],"library_scope":if imp{"mova_implementation"}else{"unreviewed"},"editorial_status":if imp{"implementation_snapshot"}else{"unreviewed"},"rights":{"status":"not_reviewed","public_export_allowed":false},"public_export_allowed":false,"reason":if imp{"Mova-bound implementation material; not an independent general-subject approval"}else{"No independent global editorial and rights decision"},"evidence":[],"allowed_artifacts":[]});
        if let Some(decision) = policy.get(&rel) {
            ensure!(
                decision["manifest_sha256"] == digest,
                "review decision has stale manifest digest"
            );
            if scope(decision) == Some("global") {
                let assets = global_policy(&store, &rel, &m, decision, &artifact_inventory)?;
                e = decision.clone();
                e["id"] = m["id"].clone();
                e["allowed_artifacts"] = json!(assets);
            } else if scope(decision) == Some("mova_implementation")
                || scope(decision) == Some("unreviewed")
            {
                e["reason"] = decision
                    .get("reason")
                    .cloned()
                    .unwrap_or(e["reason"].clone());
            } else {
                e["reason"] = json!("Unknown or absent requested scope is excluded");
            }
        }
        entries.push(e);
    }
    let global = entries
        .iter()
        .filter(|e| scope(e) == Some("global"))
        .count();
    let inventory_sha = sha(&serde_json::to_vec(
        &entries
            .iter()
            .map(|e| json!({"manifest":e["manifest"],"manifest_sha256":e["manifest_sha256"]}))
            .collect::<Vec<_>>(),
    )?);
    let result = json!({"schema":1,"kind":"groupant_publication_catalog","default_library_scope":"unreviewed","publication_channel":"global_groupant_library","complete":true,"requires_publication_check":true,"inventory_sha256":inventory_sha,"review_policy_sha256":policy_sha,"inventory":{"visited_entries":visited,"manifests":entries.len(),"maximum_entries":MAX_SCAN},"eligible_global_policy_decisions":global,"entries":entries,"scope":"Trusted versioned policy decisions; current reader integrity and exact artifact allowlists remain required. No mathematical proof or scientific truth is inferred."});
    ensure!(
        serde_json::to_vec(&result)?.len() as u64 <= MAX_JSON,
        "catalog exceeds bounded JSON size; no incomplete catalog emitted"
    );
    Ok(result)
}
fn denied(
    rel: &str,
    manifest_sha: Option<&str>,
    catalog_sha: Option<&str>,
    reason: &str,
    scope: &str,
) -> Value {
    json!({"schema":1,"kind":"groupant_publication_check","manifest":rel,"manifest_sha256":manifest_sha,"catalog_sha256":catalog_sha,"effective_library_scope":scope,"public_export_allowed":false,"allowed_artifacts":[],"reason":reason,"integrity_checked":false,"proof":false})
}
/// Compatibility path: without a caller-pinned policy, this can only deny.
pub(crate) fn check(repo: &Path, store: &Path, catalog_path: &Path, rel: &str) -> Result<Value> {
    check_with_policy(repo, store, catalog_path, rel, None)
}
fn normalized_entry(e: &Value) -> Result<Value> {
    let mut v = json!({});
    for k in [
        "manifest",
        "manifest_sha256",
        "id",
        "library_scope",
        "editorial_status",
        "public_export_allowed",
        "reason",
    ] {
        v[k] = e[k].clone();
    }
    v["rights"] = json!({"status":e["rights"]["status"],"public_export_allowed":e["rights"]["public_export_allowed"]});
    for k in ["evidence", "allowed_artifacts"] {
        let mut list = e[k]
            .as_array()
            .ok_or_else(|| anyhow!("missing decision list"))?
            .iter()
            .map(|a| json!({"path":a["path"],"sha256":a["sha256"]}))
            .collect::<Vec<_>>();
        list.sort_by_key(|a| {
            (
                a["path"].as_str().unwrap_or("").to_owned(),
                a["sha256"].as_str().unwrap_or("").to_owned(),
            )
        });
        v[k] = json!(list);
    }
    Ok(v)
}
/// Positive admission additionally requires the caller's expected review-file digest.
pub(crate) fn check_with_policy(
    repo: &Path,
    store: &Path,
    catalog_path: &Path,
    rel: &str,
    policy: Option<(&Path, &str)>,
) -> Result<Value> {
    let store = store_root(store)?;
    ensure!(
        normal(rel) && rel.ends_with("/manifest.json"),
        "invalid publication manifest path"
    );
    let path = match confined(&store, rel) {
        Ok(p) => p,
        Err(e) => {
            return Ok(denied(
                rel,
                None,
                None,
                &format!("manifest unavailable: {e}"),
                "unreviewed",
            ));
        }
    };
    let raw = read(&path)?;
    let manifest_sha = sha(&raw);
    let m: Value = serde_json::from_slice(&raw)?;
    if !catalog_path.is_file() {
        return Ok(denied(
            rel,
            Some(&manifest_sha),
            None,
            "publication catalog absent",
            "unreviewed",
        ));
    }
    let bytes = read(catalog_path)?;
    let cat_sha = sha(&bytes);
    let c: Value = serde_json::from_slice(&bytes)?;
    if c["schema"] != 1
        || c["kind"] != "groupant_publication_catalog"
        || c["default_library_scope"] != "unreviewed"
        || c["complete"] != true
    {
        return Ok(denied(
            rel,
            Some(&manifest_sha),
            Some(&cat_sha),
            "catalog unsupported or incomplete",
            "unreviewed",
        ));
    }
    let map = entry_map(&c)?;
    let Some(e) = map.get(rel) else {
        return Ok(denied(
            rel,
            Some(&manifest_sha),
            Some(&cat_sha),
            "manifest has no explicit catalog decision",
            "unreviewed",
        ));
    };
    if e["manifest_sha256"] != manifest_sha {
        return Ok(denied(
            rel,
            Some(&manifest_sha),
            Some(&cat_sha),
            "catalog manifest digest is stale",
            "unreviewed",
        ));
    }
    let requested = scope(e).unwrap_or("unreviewed");
    if requested != "global" {
        return Ok(denied(
            rel,
            Some(&manifest_sha),
            Some(&cat_sha),
            "scope is not explicitly reviewed global",
            requested,
        ));
    }
    let (policy_path, expected) = match policy {
        Some(p) => p,
        None => {
            return Ok(denied(
                rel,
                Some(&manifest_sha),
                Some(&cat_sha),
                "positive admission requires caller-pinned reviews file and expected SHA",
                "unreviewed",
            ));
        }
    };
    let current = (|| -> Result<Value> {
        ensure!(digest(expected), "invalid expected reviews digest");
        ensure!(
            sha(&read(policy_path)?) == expected,
            "caller review-policy pin mismatch"
        );
        ensure!(
            c["review_policy_sha256"] == expected,
            "catalog review-policy digest mismatch"
        );
        let current = catalog(&store, Some(policy_path))?;
        ensure!(
            current["review_policy_sha256"] == expected,
            "review policy changed during check"
        );
        ensure!(
            c["inventory_sha256"] == current["inventory_sha256"],
            "catalog inventory is stale or not pinned"
        );
        let entries = entry_map(&current)?;
        let decision = entries
            .get(rel)
            .ok_or_else(|| anyhow!("policy decision absent"))?;
        ensure!(
            normalized_entry(e)? == normalized_entry(decision)?,
            "catalog entry differs from caller-pinned review decision"
        );
        Ok(current)
    })();
    let current = match current {
        Ok(v) => v,
        Err(error) => {
            return Ok(denied(
                rel,
                Some(&manifest_sha),
                Some(&cat_sha),
                &format!("pinned review validation failed: {error}"),
                "unreviewed",
            ));
        }
    };
    let assets = entry_map(&current)?.get(rel).unwrap()["allowed_artifacts"].clone();
    let integrity = match super::library(repo, &store, rel) {
        Ok(v) => v,
        Err(error) => {
            return Ok(denied(
                rel,
                Some(&manifest_sha),
                Some(&cat_sha),
                &format!("reader integrity failed: {error}"),
                "unreviewed",
            ));
        }
    };
    ensure!(
        integrity["manifest_sha256"] == manifest_sha,
        "manifest changed during publication check"
    );
    Ok(
        json!({"schema":1,"kind":"groupant_publication_check","manifest":rel,"manifest_sha256":manifest_sha,"catalog_sha256":cat_sha,"effective_library_scope":"global","editorial_status":"independently_reviewed","public_export_allowed":true,"allowed_artifacts":assets,"review_policy_sha256":expected,"mathematical_status":m["status"],"integrity_checked":true,"integrity":integrity,"reason":e["reason"],"evidence":e["evidence"],"rights":e["rights"],"proof":false,"scope":"Artifact-limited admission on a fixed trusted local filesystem; editorial review is a caller-pinned trusted assertion, not authorship, rights ownership or scientific proof. The immutable generated/unverified mathematical status is preserved independently. The legacy reader may permit internal symlinks in unselected assets; no atomic snapshot or concurrent-filesystem security is claimed."}),
    )
}
/// Additive catalog metadata may live at the store root or outside the store, never in a bundle.
pub(crate) fn catalog_destination(store: &Path, out: &Path) -> Result<()> {
    let store = store_root(store)?;
    let parent = out
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = fs::canonicalize(parent)?;
    ensure!(
        !parent.starts_with(&store) || parent == store,
        "catalog destination must not modify a bundle directory"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    fn check(repo: &Path, store: &Path, cp: &Path, rel: &str) -> Result<Value> {
        let p = cp.parent().unwrap().join("reviews.json");
        if p.is_file() {
            let h = sha(&read(&p)?);
            super::check_with_policy(repo, store, cp, rel, Some((&p, &h)))
        } else {
            super::check(repo, store, cp, rel)
        }
    }
    struct F {
        base: PathBuf,
        repo: PathBuf,
        store: PathBuf,
        rel: &'static str,
    }
    impl Drop for F {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.base);
        }
    }
    fn put(root: &Path, rel: &str, b: &[u8]) -> String {
        let p = root.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, b).unwrap();
        sha(b)
    }
    fn jput(p: &Path, v: &Value) {
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
    }
    fn fixture() -> F {
        let base = std::env::temp_dir().join(format!(
            "publication-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let repo = base.join("repo");
        let store = base.join("store");
        fs::create_dir_all(&repo).unwrap();
        fs::create_dir_all(&store).unwrap();
        let rel = "paper/edition/manifest.json";
        let d = "x".repeat(4000);
        let h = put(&store, "paper/edition/description.md", d.as_bytes());
        let auxiliary = put(&store, "paper/edition/auxiliary.json", b"{}");
        put(
            &store,
            "reviews/editorial.md",
            b"Independent editorial and rights fixture review.",
        );
        let m = json!({"schema":1,"kind":"source_survey_groupant","id":"paper-survey","status":"generated_source_survey_unverified","source":{"id":"paper","kind":"paper","sha256":"a".repeat(64),"format":"markdown"},"description_unicode_scalars":4000,"local_only":false,"public_export_allowed":true,"license":{"local_only":false},"components":{"description":{"path":"description.md","status":"generated_draft"},"ast":{"status":"missing"},"proof":{"status":"missing"}},"artifacts":[{"path":"description.md","sha256":h},{"path":"auxiliary.json","sha256":auxiliary}]});
        jput(&store.join(rel), &m);
        F {
            base,
            repo,
            store,
            rel,
        }
    }
    fn decision(f: &F) -> Value {
        let m = load(&f.store.join(f.rel)).unwrap();
        json!({"manifest":f.rel,"manifest_sha256":sha(&read(&f.store.join(f.rel)).unwrap()),"library_scope":"global","editorial_status":"independently_reviewed","rights":{"status":"permitted","public_export_allowed":true},"public_export_allowed":true,"reason":"Explicit synthetic editorial and rights approval; not a proof.","evidence":[{"path":"reviews/editorial.md","sha256":sha(&read(&f.store.join("reviews/editorial.md")).unwrap())}],"allowed_artifacts":[{"path":"description.md","sha256":m["artifacts"][0]["sha256"]}]})
    }
    fn reviewed(f: &F) -> (Value, PathBuf) {
        let policy = f.base.join("reviews.json");
        jput(
            &policy,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[decision(f)]}),
        );
        let c = catalog(&f.store, Some(&policy)).unwrap();
        let cp = f.base.join("catalog.json");
        jput(&cp, &c);
        (c, cp)
    }
    #[test]
    fn unreviewed_defaults_and_manifest_self_labels_do_not_publish() {
        let f = fixture();
        let mut m = load(&f.store.join(f.rel)).unwrap();
        m["library_scope"] = json!("global");
        jput(&f.store.join(f.rel), &m);
        let c = catalog(&f.store, None).unwrap();
        assert_eq!(c, catalog(&f.store, None).unwrap());
        assert_eq!(c["entries"][0]["library_scope"], "unreviewed");
        assert_eq!(c["eligible_global_policy_decisions"], 0);
        let cp = f.base.join("none.json");
        assert_eq!(
            check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
        jput(&cp, &c);
        assert_eq!(
            check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
    }
    #[test]
    fn explicit_global_is_artifact_limited_and_runs_actual_reader() {
        let f = fixture();
        let (c, cp) = reviewed(&f);
        assert_eq!(c["eligible_global_policy_decisions"], 1);
        let r = check(&f.repo, &f.store, &cp, f.rel).unwrap();
        assert_eq!(r["public_export_allowed"], true);
        assert_eq!(r["integrity_checked"], true);
        assert_eq!(r["integrity"]["source_bindings"], 0);
        assert_eq!(r["allowed_artifacts"].as_array().unwrap().len(), 1);
        assert_eq!(r["allowed_artifacts"][0]["path"], "description.md");
        assert_eq!(r["proof"], false);
    }
    #[test]
    fn source_snapshots_and_mova_paper_bindings_never_promote() {
        for kind in [
            "algorithm_groupant_bundle",
            "paper_groupant",
            "source_survey_groupant",
        ] {
            let f = fixture();
            let mut m = load(&f.store.join(f.rel)).unwrap();
            m["kind"] = json!(kind);
            if kind == "paper_groupant" {
                m["source_bindings"] = json!([{"path":"code.rs","sha256":"b".repeat(64)}]);
            }
            if kind == "source_survey_groupant" {
                m["source"]["kind"] = json!("algorithm");
                m["source"]["repository_path"] = json!("algorithms/src/a/mod.rs");
            }
            jput(&f.store.join(f.rel), &m);
            let plain = catalog(&f.store, None).unwrap();
            assert_eq!(plain["entries"][0]["library_scope"], "mova_implementation");
            let policy = f.base.join("bad-review.json");
            jput(
                &policy,
                &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[decision(&f)]}),
            );
            assert!(
                catalog(&f.store, Some(&policy))
                    .unwrap_err()
                    .to_string()
                    .contains("implementation")
            );
        }
    }
    #[test]
    fn stale_manifest_policy_and_catalog_fail_closed() {
        let f = fixture();
        let (_, cp) = reviewed(&f);
        let mut m = load(&f.store.join(f.rel)).unwrap();
        m["edition"] = json!("changed");
        jput(&f.store.join(f.rel), &m);
        let r = check(&f.repo, &f.store, &cp, f.rel).unwrap();
        assert_eq!(r["effective_library_scope"], "unreviewed");
        assert_eq!(r["public_export_allowed"], false);
        let policy = f.base.join("reviews.json");
        assert!(catalog(&f.store, Some(&policy)).is_err());
    }
    #[test]
    fn unknown_scope_incomplete_catalog_and_new_manifest_are_excluded() {
        let f = fixture();
        let (mut c, cp) = reviewed(&f);
        c["entries"][0]["library_scope"] = json!("global-ish");
        jput(&cp, &c);
        assert_eq!(
            check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
        c["entries"][0]["library_scope"] = json!("global");
        c["complete"] = json!(false);
        jput(&cp, &c);
        assert_eq!(
            check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
        c["complete"] = json!(true);
        jput(&cp, &c);
        let new = "paper/later/manifest.json";
        let raw = read(&f.store.join(f.rel)).unwrap();
        put(&f.store, new, &raw);
        assert_eq!(
            check(&f.repo, &f.store, &cp, new).unwrap()["public_export_allowed"],
            false
        );
    }
    #[test]
    fn missing_rights_review_evidence_and_allowlist_cannot_admit() {
        for field in [
            "rights",
            "editorial_status",
            "evidence",
            "allowed_artifacts",
            "public_export_allowed",
        ] {
            let f = fixture();
            let (mut c, cp) = reviewed(&f);
            c["entries"][0].as_object_mut().unwrap().remove(field);
            jput(&cp, &c);
            assert_eq!(
                check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
                false
            );
        }
    }
    #[test]
    fn immutable_restrictions_cannot_be_overridden_by_policy() {
        for (parent, field, value) in [
            ("", "local_only", true),
            ("license", "local_only", true),
            ("", "public_export_allowed", false),
            ("license", "public_export_allowed", false),
        ] {
            let f = fixture();
            let mut m = load(&f.store.join(f.rel)).unwrap();
            if parent.is_empty() {
                m[field] = json!(value);
            } else {
                m[parent][field] = json!(value);
            }
            jput(&f.store.join(f.rel), &m);
            let policy = f.base.join("restricted.json");
            jput(
                &policy,
                &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[decision(&f)]}),
            );
            assert!(catalog(&f.store, Some(&policy)).is_err());
            let mut c = catalog(&f.store, None).unwrap();
            c["entries"][0] = decision(&f);
            let cp = f.base.join("restricted-catalog.json");
            jput(&cp, &c);
            assert_eq!(
                check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
                false
            );
        }
    }
    #[test]
    fn stale_evidence_and_nonallowed_asset_corruption_are_distinct_denials() {
        let f = fixture();
        let (_, cp) = reviewed(&f);
        put(&f.store, "reviews/editorial.md", b"changed review");
        let r = check(&f.repo, &f.store, &cp, f.rel).unwrap();
        assert_eq!(r["public_export_allowed"], false);
        assert!(r["reason"].as_str().unwrap().contains("evidence"));
        let g = fixture();
        let (_, cg) = reviewed(&g);
        put(
            &g.store,
            "paper/edition/auxiliary.json",
            b"changed unlisted asset",
        );
        let r = check(&g.repo, &g.store, &cg, g.rel).unwrap();
        assert_eq!(r["public_export_allowed"], false);
        assert!(
            r["reason"]
                .as_str()
                .unwrap()
                .contains("reader integrity failed")
        );
    }
    #[test]
    fn duplicate_decisions_paths_and_artifacts_are_rejected() {
        let f = fixture();
        let policy = f.base.join("duplicate.json");
        let d = decision(&f);
        jput(
            &policy,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[d.clone(),d]}),
        );
        assert!(catalog(&f.store, Some(&policy)).is_err());
        let (mut c, cp) = reviewed(&f);
        let e = c["entries"][0].clone();
        c["entries"].as_array_mut().unwrap().push(e);
        jput(&cp, &c);
        assert!(check(&f.repo, &f.store, &cp, f.rel).is_err());
        let (mut c, cp) = reviewed(&f);
        let a = c["entries"][0]["allowed_artifacts"][0].clone();
        c["entries"][0]["allowed_artifacts"]
            .as_array_mut()
            .unwrap()
            .push(a);
        jput(&cp, &c);
        assert_eq!(
            check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
    }
    #[test]
    fn path_escape_and_symlinks_are_not_admitted() {
        let f = fixture();
        let (_, cp) = reviewed(&f);
        assert!(check(&f.repo, &f.store, &cp, "../paper/edition/manifest.json").is_err());
        let (mut c, cp) = reviewed(&f);
        c["entries"][0]["allowed_artifacts"][0]["path"] = json!("../description.md");
        jput(&cp, &c);
        assert_eq!(
            check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
        std::os::unix::fs::symlink(f.store.join("paper"), f.store.join("alias")).unwrap();
        assert!(catalog(&f.store, None).is_err());
        assert_eq!(
            check(&f.repo, &f.store, &cp, "alias/edition/manifest.json").unwrap()["public_export_allowed"],
            false
        );
    }
    #[test]
    fn unknown_exposition_and_raw_code_allowlist_stay_excluded() {
        let f = fixture();
        let mut m = load(&f.store.join(f.rel)).unwrap();
        m["kind"] = json!("new_generic_exposition");
        jput(&f.store.join(f.rel), &m);
        assert_eq!(
            catalog(&f.store, None).unwrap()["entries"][0]["library_scope"],
            "unreviewed"
        );
        let policy = f.base.join("unknown.json");
        jput(
            &policy,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[decision(&f)]}),
        );
        assert!(catalog(&f.store, Some(&policy)).is_err());
        let g = fixture();
        let h = put(&g.store, "paper/edition/source.rs.txt", b"pub fn raw(){}");
        let mut m = load(&g.store.join(g.rel)).unwrap();
        m["artifacts"]
            .as_array_mut()
            .unwrap()
            .push(json!({"path":"source.rs.txt","sha256":h}));
        jput(&g.store.join(g.rel), &m);
        let mut d = decision(&g);
        d["allowed_artifacts"] = json!([{"path":"source.rs.txt","sha256":h}]);
        jput(
            &policy,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[d]}),
        );
        assert!(
            catalog(&g.store, Some(&policy))
                .unwrap_err()
                .to_string()
                .contains("description narrative")
        );
    }
    #[test]
    fn caller_pin_and_policy_entry_are_required_for_positive_admission() {
        let f = fixture();
        let (mut c, cp) = reviewed(&f);
        assert_eq!(
            super::check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
        let p = f.base.join("reviews.json");
        let actual = sha(&read(&p).unwrap());
        let r =
            super::check_with_policy(&f.repo, &f.store, &cp, f.rel, Some((&p, &"0".repeat(64))))
                .unwrap();
        assert_eq!(r["public_export_allowed"], false);
        assert!(r["reason"].as_str().unwrap().contains("pin mismatch"));
        c["entries"][0]["reason"] = json!("Forged catalog-only editorial decision");
        jput(&cp, &c);
        let r =
            super::check_with_policy(&f.repo, &f.store, &cp, f.rel, Some((&p, &actual))).unwrap();
        assert_eq!(r["public_export_allowed"], false);
        assert!(r["reason"].as_str().unwrap().contains("differs"));
        c["review_policy_sha256"] = json!("f".repeat(64));
        jput(&cp, &c);
        assert_eq!(
            check(&f.repo, &f.store, &cp, f.rel).unwrap()["public_export_allowed"],
            false
        );
    }
    #[test]
    fn normalized_entries_preserve_order_independent_policy_and_assurance() {
        let f = fixture();
        put(
            &f.store,
            "reviews/second.md",
            b"Second independent fixture note",
        );
        let mut d = decision(&f);
        d["evidence"].as_array_mut().unwrap().push(json!({"path":"reviews/second.md","sha256":sha(&read(&f.store.join("reviews/second.md")).unwrap())}));
        let p = f.base.join("reviews.json");
        jput(
            &p,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[d]}),
        );
        let mut c = catalog(&f.store, Some(&p)).unwrap();
        c["entries"][0]["evidence"]
            .as_array_mut()
            .unwrap()
            .reverse();
        let cp = f.base.join("catalog.json");
        jput(&cp, &c);
        let r = check(&f.repo, &f.store, &cp, f.rel).unwrap();
        assert_eq!(r["public_export_allowed"], true);
        assert_eq!(r["editorial_status"], "independently_reviewed");
        assert_eq!(
            r["mathematical_status"],
            "generated_source_survey_unverified"
        );
        assert_eq!(r["proof"], false);
        assert_eq!(r["review_policy_sha256"], sha(&read(&p).unwrap()));
    }
    #[test]
    fn changed_policy_or_inventory_cannot_reuse_old_admission() {
        let f = fixture();
        let (_, cp) = reviewed(&f);
        let p = f.base.join("reviews.json");
        let expected = sha(&read(&p).unwrap());
        let mut policy = load(&p).unwrap();
        policy["entries"][0]["reason"] = json!("Changed policy edition");
        jput(&p, &policy);
        assert_eq!(
            super::check_with_policy(&f.repo, &f.store, &cp, f.rel, Some((&p, &expected))).unwrap()
                ["public_export_allowed"],
            false
        );
        let g = fixture();
        let (_, cg) = reviewed(&g);
        put(
            &g.store,
            "paper/new/manifest.json",
            &read(&g.store.join(g.rel)).unwrap(),
        );
        let r = check(&g.repo, &g.store, &cg, g.rel).unwrap();
        assert_eq!(r["public_export_allowed"], false);
        assert!(r["reason"].as_str().unwrap().contains("inventory"));
    }
    #[test]
    fn generated_artifacts_cannot_supply_reserved_review_evidence() {
        let f = fixture();
        let mut d = decision(&f);
        let m = load(&f.store.join(f.rel)).unwrap();
        d["evidence"] =
            json!([{"path":"paper/edition/description.md","sha256":m["artifacts"][0]["sha256"]}]);
        let p = f.base.join("reviews.json");
        jput(
            &p,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[d]}),
        );
        assert!(
            catalog(&f.store, Some(&p))
                .unwrap_err()
                .to_string()
                .contains("reserved reviews")
        );
        let g = fixture();
        put(
            &g.store,
            "reviews/editorial.md",
            &read(&g.store.join("paper/edition/description.md")).unwrap(),
        );
        jput(
            &p,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[decision(&g)]}),
        );
        assert!(
            catalog(&g.store, Some(&p))
                .unwrap_err()
                .to_string()
                .contains("generated artifact")
        );
        let h = fixture();
        put(&h.store, "reviews/nested/note.md", b"Independent note");
        put(&h.store, "reviews/manifest.json", b"{}");
        let e = json!({"evidence":[{"path":"reviews/nested/note.md","sha256":sha(b"Independent note")}]});
        assert!(
            evidence(&h.store, &e, &ArtifactInventory::default())
                .unwrap_err()
                .to_string()
                .contains("manifest ancestor")
        );
        let mut inventory = ArtifactInventory::default();
        inventory.paths.insert("reviews/editorial.md".to_owned());
        assert!(
            evidence(&f.store, &decision(&f), &inventory)
                .unwrap_err()
                .to_string()
                .contains("generated artifact")
        );
    }
    #[test]
    fn narrative_role_and_source_digest_bound_artifact_publication() {
        let f = fixture();
        let mut d = decision(&f);
        let m = load(&f.store.join(f.rel)).unwrap();
        d["allowed_artifacts"] = json!([m["artifacts"][1].clone()]);
        let p = f.base.join("reviews.json");
        jput(
            &p,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[d]}),
        );
        assert!(
            catalog(&f.store, Some(&p))
                .unwrap_err()
                .to_string()
                .contains("description narrative")
        );
        let g = fixture();
        let mut m = load(&g.store.join(g.rel)).unwrap();
        m["source"]["sha256"] = m["artifacts"][0]["sha256"].clone();
        jput(&g.store.join(g.rel), &m);
        jput(
            &p,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[decision(&g)]}),
        );
        assert!(
            catalog(&g.store, Some(&p))
                .unwrap_err()
                .to_string()
                .contains("raw source bytes")
        );
    }
    #[test]
    fn hidden_paths_git_ancestors_and_incomplete_artifact_inventory_exclude() {
        assert!(!normal(".hidden/edition/manifest.json"));
        assert!(!normal("paper/.hidden/manifest.json"));
        let f = fixture();
        fs::create_dir(f.base.join(".git")).unwrap();
        assert!(catalog(&f.store, None).is_err());
        let g = fixture();
        let p = g.base.join("reviews.json");
        jput(
            &p,
            &json!({"schema":1,"kind":"groupant_publication_reviews","entries":[decision(&g)]}),
        );
        put(&g.store, "unknown/manifest.json", b"{}");
        assert!(
            catalog(&g.store, Some(&p))
                .unwrap_err()
                .to_string()
                .contains("artifact inventory")
        );
    }
    #[test]
    fn bounded_scan_and_destinations_do_not_modify_bundles() {
        let f = fixture();
        let mut paths = Vec::new();
        let mut visited = MAX_SCAN;
        assert!(walk(&f.store, &f.store, &mut paths, &mut visited, 0).is_err());
        assert!(catalog_destination(&f.store, &f.store.join("publication.json")).is_ok());
        assert!(
            catalog_destination(&f.store, &f.store.join("paper/edition/publication.json")).is_err()
        );
        assert!(catalog_destination(&f.store, &f.base.join("catalog.json")).is_ok());
    }
}
