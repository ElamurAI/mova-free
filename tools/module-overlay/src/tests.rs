use super::*;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    root: PathBuf,
    upstream: PathBuf,
    custom: PathBuf,
    expected: Snapshot,
    manifest: Manifest,
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mova-module-overlay-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let upstream = root.join("upstream");
        let custom = root.join("custom");
        fs::create_dir_all(upstream.join("src")).unwrap();
        fs::create_dir_all(&custom).unwrap();
        fs::write(upstream.join("src/lib.rs"), "pub fn value() -> u8 { 1 }\n").unwrap();
        fs::write(upstream.join("README.md"), "Original source\n").unwrap();
        let expected = snapshot(&upstream, "release-one").unwrap();
        write_new_json(&custom.join("base.json"), &expected).unwrap();
        let replacement = b"pub fn value() -> u8 { 2 }\n";
        fs::write(custom.join("replacement.txt"), replacement).unwrap();
        let manifest = Manifest {
            schema: 1,
            base_snapshot: "base.json".into(),
            base_snapshot_sha256: sha(&fs::read(custom.join("base.json")).unwrap()),
            files: vec![Overlay {
                action: Action::Replace,
                target: "src/lib.rs".into(),
                source: "replacement.txt".into(),
                sha256: sha(replacement),
                expected_upstream_sha256: Some(expected.files["src/lib.rs"].sha256.clone()),
                executable: false,
            }],
            cargo_manifests: vec![],
        };
        Self {
            root,
            upstream,
            custom,
            expected,
            manifest,
        }
    }
    fn add(&mut self, target: &str, bytes: &[u8]) {
        let source = format!("custom-{}.txt", self.manifest.files.len());
        fs::write(self.custom.join(&source), bytes).unwrap();
        self.manifest.files.push(Overlay {
            action: Action::Add,
            target: target.into(),
            source,
            sha256: sha(bytes),
            expected_upstream_sha256: None,
            executable: false,
        });
    }
    fn stage(&self, name: &str) -> Result<Receipt> {
        stage(
            &self.upstream,
            &self.custom,
            &self.manifest,
            &self.expected,
            &self.root.join(name),
        )
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn overwrite_and_addition_leave_inputs_unchanged_and_repeat_exactly() {
    let mut f = Fixture::new();
    f.add("pipelines/custom.json", br#"{"steps":["read","review"]}"#);
    let before_upstream = snapshot(&f.upstream, "release-one").unwrap();
    let before_custom = snapshot(&f.custom, "custom-one").unwrap();
    let a = f.stage("a").unwrap();
    let b = f.stage("b").unwrap();
    assert_eq!(a, b);
    assert_eq!(
        fs::read(f.root.join("a").join(RECEIPT)).unwrap(),
        fs::read(f.root.join("b").join(RECEIPT)).unwrap()
    );
    assert_eq!(
        fs::read_to_string(f.root.join("a/src/lib.rs")).unwrap(),
        "pub fn value() -> u8 { 2 }\n"
    );
    assert_eq!(
        fs::read(f.root.join("a/pipelines/custom.json")).unwrap(),
        br#"{"steps":["read","review"]}"#
    );
    assert_eq!(
        snapshot(&f.upstream, "release-one").unwrap(),
        before_upstream
    );
    assert_eq!(snapshot(&f.custom, "custom-one").unwrap(), before_custom);
    assert_eq!(
        snapshot(&f.root.join("a"), "composed").unwrap().tree_sha256,
        a.output_tree_sha256
    );
}
#[test]
fn upgrade_drift_is_diagnosed_before_output_creation() {
    let f = Fixture::new();
    fs::write(f.upstream.join("src/lib.rs"), "new upstream API").unwrap();
    fs::remove_file(f.upstream.join("README.md")).unwrap();
    fs::write(f.upstream.join("new.txt"), "new").unwrap();
    let report = check(&f.upstream, &f.custom, &f.manifest, &f.expected).unwrap();
    assert!(!report.compatible);
    assert_eq!(report.drift.len(), 3);
    assert!(f.stage("out").is_err());
    assert!(!f.root.join("out").exists());
    assert_eq!(
        fs::read_to_string(f.upstream.join("src/lib.rs")).unwrap(),
        "new upstream API"
    );
}
#[test]
fn changed_custom_bytes_and_forged_base_bindings_reject() {
    let mut f = Fixture::new();
    fs::write(f.custom.join("replacement.txt"), "unreviewed").unwrap();
    assert!(f.stage("out").is_err());
    fs::write(
        f.custom.join("replacement.txt"),
        "pub fn value() -> u8 { 2 }\n",
    )
    .unwrap();
    f.manifest.base_snapshot_sha256 = "0".repeat(64);
    assert!(f.stage("out").is_err());
    f.manifest.base_snapshot_sha256 = sha(&fs::read(f.custom.join("base.json")).unwrap());
    let mut forged = f.expected.clone();
    forged.files.remove("README.md");
    forged.tree_sha256 = tree_hash(&forged.files).unwrap();
    assert!(
        stage(
            &f.upstream,
            &f.custom,
            &f.manifest,
            &forged,
            &f.root.join("out")
        )
        .is_err()
    );
}
#[test]
fn explicit_actions_duplicates_and_file_directory_collisions_reject() {
    let mut f = Fixture::new();
    f.manifest.files[0].action = Action::Add;
    assert!(f.stage("out").is_err());
    f.manifest.files[0].action = Action::Replace;
    f.manifest.files.push(f.manifest.files[0].clone());
    assert!(f.stage("out").is_err());
    f.manifest.files.pop();
    f.add("README.md/child", b"collision");
    assert!(f.stage("out").is_err());
    f.manifest.files.pop();
    f.add("missing.rs", b"new");
    f.manifest.files.last_mut().unwrap().action = Action::Replace;
    assert!(f.stage("out").is_err());
}
#[test]
fn traversal_reserved_paths_and_unknown_hooks_reject() {
    let mut f = Fixture::new();
    for path in [
        "../outside",
        "/absolute",
        "a/../b",
        "a//b",
        "a\\b",
        "C:drive",
        "peta/private",
        ".git/config",
        "a/target/x",
        RECEIPT,
    ] {
        f.manifest.files[0].target = path.into();
        assert!(f.stage("out").is_err(), "{path}");
    }
    f.manifest.files[0].target = "src/lib.rs".into();
    f.manifest.files[0].source = "../outside".into();
    assert!(f.stage("out").is_err());
    let mut value = serde_json::to_value(&f.manifest).unwrap();
    value["shell_hook"] = serde_json::json!("do anything");
    assert!(serde_json::from_value::<Manifest>(value).is_err());
}
#[test]
fn existing_outputs_and_overlapping_roots_are_preserved() {
    let f = Fixture::new();
    fs::create_dir(f.root.join("out")).unwrap();
    fs::write(f.root.join("out/keep"), "keep").unwrap();
    assert!(f.stage("out").is_err());
    assert_eq!(fs::read_to_string(f.root.join("out/keep")).unwrap(), "keep");
    assert!(
        stage(
            &f.upstream,
            &f.custom,
            &f.manifest,
            &f.expected,
            &f.upstream.join("nested")
        )
        .is_err()
    );
    assert!(
        stage(
            &f.upstream,
            &f.custom,
            &f.manifest,
            &f.expected,
            &f.custom.join("nested")
        )
        .is_err()
    );
    assert!(save_snapshot(&f.upstream, "label", &f.upstream.join("base.json")).is_err());
    assert!(!f.upstream.join("base.json").exists());
}
#[cfg(unix)]
#[test]
fn symlinks_and_special_files_reject_without_following() {
    use std::os::unix::{fs::symlink, net::UnixListener};
    let f = Fixture::new();
    symlink("src/lib.rs", f.upstream.join("alias")).unwrap();
    assert!(snapshot(&f.upstream, "label").is_err());
    fs::remove_file(f.upstream.join("alias")).unwrap();
    symlink(&f.upstream, f.root.join("alias-root")).unwrap();
    assert!(snapshot(&f.root.join("alias-root"), "label").is_err());
    fs::remove_file(f.custom.join("replacement.txt")).unwrap();
    symlink(
        f.upstream.join("src/lib.rs"),
        f.custom.join("replacement.txt"),
    )
    .unwrap();
    assert!(f.stage("out").is_err());
    let _socket = UnixListener::bind(f.upstream.join("socket")).unwrap();
    assert!(snapshot(&f.upstream, "label").is_err());
    symlink(&f.upstream, f.root.join("out-link")).unwrap();
    assert!(f.stage("out-link").is_err());
}
#[test]
fn ignored_artifacts_do_not_publish_and_private_tree_refuses() {
    let f = Fixture::new();
    for dir in [".git", "target"] {
        fs::create_dir(f.upstream.join(dir)).unwrap();
        fs::write(f.upstream.join(dir).join("private"), "not copied").unwrap();
    }
    fs::write(f.upstream.join(".env"), "not copied").unwrap();
    fs::write(f.upstream.join("local.md"), "not copied").unwrap();
    f.stage("out").unwrap();
    for item in [".git", "target", ".env", "local.md"] {
        assert!(!f.root.join("out").join(item).exists());
    }
    fs::create_dir(f.upstream.join("peta")).unwrap();
    assert!(snapshot(&f.upstream, "label").is_err());
}
#[test]
fn missing_stale_and_hash_matched_models_remain_distinct() {
    let mut f = Fixture::new();
    f.add("extra.rs", b"// unmodeled source");
    f.add(
        "src/lib.rs.groupant/manifest.json",
        format!(
            r#"{{"source":{{"sha256":"{}"}}}}"#,
            f.expected.files["src/lib.rs"].sha256
        )
        .as_bytes(),
    );
    let r = f.stage("stale").unwrap();
    assert_eq!(r.source_models["src/lib.rs"], "stale_source_binding");
    assert_eq!(r.source_models["extra.rs"], "missing");
    let n = f.manifest.files.len() - 1;
    let bytes = format!(
        r#"{{"source":{{"sha256":"{}"}}}}"#,
        f.manifest.files[0].sha256
    );
    fs::write(f.custom.join(&f.manifest.files[n].source), &bytes).unwrap();
    f.manifest.files[n].sha256 = sha(bytes.as_bytes());
    let r = f.stage("matched").unwrap();
    assert_eq!(
        r.source_models["src/lib.rs"],
        "hash_matches_model_semantics_unverified"
    );
}
#[test]
fn cargo_recipes_are_paths_and_fixed_argv_not_shell_hooks() {
    let mut f = Fixture::new();
    f.add(
        "extension/Cargo.toml",
        b"[package]\nname='fixture'\nversion='0.1.0'\n",
    );
    f.manifest.cargo_manifests = vec!["extension/Cargo.toml".into()];
    let receipt = f.stage("out").unwrap();
    assert_eq!(
        receipt.cargo_argv[0],
        [
            "cargo",
            "check",
            "--manifest-path",
            "extension/Cargo.toml",
            "--jobs",
            "2"
        ]
    );
    assert_eq!(receipt.cargo_argv[1][1], "build");
    assert!(
        !f.root.join("out/target").exists(),
        "composer never executes Cargo"
    );
    f.manifest.cargo_manifests = vec!["--eval shell".into()];
    assert!(f.stage("invalid").is_err());
}
#[test]
fn size_depth_and_snapshot_integrity_bounds_are_checked() {
    let f = Fixture::new();
    let big = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(f.upstream.join("large"))
        .unwrap();
    big.set_len(MAX_FILE + 1).unwrap();
    assert!(snapshot(&f.upstream, "label").is_err());
    fs::remove_file(f.upstream.join("large")).unwrap();
    let deep = f
        .upstream
        .join(std::iter::repeat_n("d", 66).collect::<Vec<_>>().join("/"));
    fs::create_dir_all(deep).unwrap();
    assert!(snapshot(&f.upstream, "label").is_err());
    let mut bad = f.expected.clone();
    bad.tree_sha256 = "0".repeat(64);
    assert!(validate_snapshot(&bad).is_err());
}
#[cfg(unix)]
#[test]
fn executable_bit_is_bound_and_preserved() {
    let f = Fixture::new();
    fs::set_permissions(
        f.upstream.join("README.md"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    assert!(
        !check(&f.upstream, &f.custom, &f.manifest, &f.expected)
            .unwrap()
            .compatible
    );
    fs::set_permissions(
        f.upstream.join("README.md"),
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    let mut manifest = f.manifest.clone();
    manifest.files[0].executable = true;
    stage(
        &f.upstream,
        &f.custom,
        &manifest,
        &f.expected,
        &f.root.join("out"),
    )
    .unwrap();
    assert_ne!(
        fs::metadata(f.root.join("out/src/lib.rs"))
            .unwrap()
            .permissions()
            .mode()
            & 0o111,
        0
    );
}

// Explicitly rebind a synthetic fixture after examining its changed base.
// This is not automatic upgrade approval or an operational source migration.
fn bind_reviewed_fixture_base(f: &mut Fixture, label: &str) {
    f.expected = snapshot(&f.upstream, label).unwrap();
    let bytes = serde_json::to_vec_pretty(&f.expected).unwrap();
    fs::write(f.custom.join("base.json"), &bytes).unwrap();
    f.manifest.base_snapshot_sha256 = sha(&bytes);
    for overlay in &mut f.manifest.files {
        if overlay.action == Action::Replace {
            overlay.expected_upstream_sha256 =
                Some(f.expected.files[&overlay.target].sha256.clone());
        }
    }
}

#[test]
fn overwritten_base_drift_is_masked_in_output_but_not_admission_or_provenance() {
    let mut f = Fixture::new();
    let first = f.stage("first").unwrap();
    let first_files = snapshot(&f.root.join("first"), "observation")
        .unwrap()
        .files;
    let replacement = fs::read(f.custom.join("replacement.txt")).unwrap();
    let old_original = first.applied[0].original_sha256.clone();
    fs::write(
        f.upstream.join("src/lib.rs"),
        "pub fn value() -> u8 { 3 }\n",
    )
    .unwrap();
    let drift = check(&f.upstream, &f.custom, &f.manifest, &f.expected).unwrap();
    assert!(
        !drift.compatible,
        "overwritten drift still violates exact-base admission"
    );
    assert_eq!(drift.drift.len(), 1);
    assert_eq!(drift.drift[0].path, "src/lib.rs");
    assert!(f.stage("old-base-refused").is_err());
    assert!(!f.root.join("old-base-refused").exists());

    bind_reviewed_fixture_base(&mut f, "release-two");
    assert_eq!(
        fs::read(f.custom.join("replacement.txt")).unwrap(),
        replacement
    );
    let second = f.stage("reviewed-second").unwrap();
    let second_files = snapshot(&f.root.join("reviewed-second"), "observation")
        .unwrap()
        .files;
    assert_eq!(first_files, second_files);
    assert_eq!(first.output_tree_sha256, second.output_tree_sha256);
    for output in ["first", "reviewed-second"] {
        assert_eq!(
            fs::read(f.root.join(output).join("src/lib.rs")).unwrap(),
            b"pub fn value() -> u8 { 2 }\n"
        );
        assert_eq!(
            fs::read(f.root.join(output).join("README.md")).unwrap(),
            b"Original source\n"
        );
    }
    assert_ne!(first.base_tree_sha256, second.base_tree_sha256);
    assert_ne!(old_original, second.applied[0].original_sha256);
    assert_ne!(
        first.manifest_canonical_sha256,
        second.manifest_canonical_sha256
    );
    assert_ne!(
        first, second,
        "source observation equality must not erase provenance"
    );
}

#[test]
fn unoverridden_base_drift_remains_visible_after_explicit_fixture_rebase() {
    let mut f = Fixture::new();
    let first = f.stage("first").unwrap();
    fs::write(f.upstream.join("README.md"), "Reviewed new original\n").unwrap();
    let drift = check(&f.upstream, &f.custom, &f.manifest, &f.expected).unwrap();
    assert!(!drift.compatible);
    assert_eq!(drift.drift.len(), 1);
    assert_eq!(drift.drift[0].path, "README.md");
    assert!(f.stage("old-base-refused").is_err());
    assert!(!f.root.join("old-base-refused").exists());
    bind_reviewed_fixture_base(&mut f, "release-two");
    let second = f.stage("reviewed-second").unwrap();
    assert_ne!(first.output_tree_sha256, second.output_tree_sha256);
    assert_eq!(
        fs::read(f.root.join("reviewed-second/README.md")).unwrap(),
        b"Reviewed new original\n"
    );
    assert_eq!(
        fs::read(f.root.join("reviewed-second/src/lib.rs")).unwrap(),
        b"pub fn value() -> u8 { 2 }\n"
    );
}
