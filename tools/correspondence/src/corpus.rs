//! Deterministic ownership coverage and typed finite-relation comparisons.
//! Missing formalizations remain explicit; catalogue equality is not source equivalence.
use anyhow::{Context, Result, anyhow, ensure};
use mova_correspondence::{
    Catalog, Formula, MAX_FORMULAS, Role, compare, compose_summary, validate_formula,
};
use mova_modelcheck::{Node, Registry, inventory, sha, validate};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
};

const MAX_INPUT_BYTES: u64 = 16 * 1024 * 1024;
const MAX_CANDIDATES: usize = 100_000;
const MAX_REGISTRY_NODES: usize = 100_000;

fn bounded_read(path: &Path) -> Result<Vec<u8>> {
    ensure!(
        fs::metadata(path)?.len() <= MAX_INPUT_BYTES,
        "input exceeds byte cap"
    );
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(
        &mut std::io::Read::take(fs::File::open(path)?, MAX_INPUT_BYTES + 1),
        &mut bytes,
    )?;
    ensure!(
        bytes.len() as u64 <= MAX_INPUT_BYTES,
        "input grew beyond byte cap"
    );
    Ok(bytes)
}

fn confined(root: &Path, relative: &str) -> Result<std::path::PathBuf> {
    ensure!(
        !relative.is_empty()
            && Path::new(relative)
                .components()
                .all(|p| matches!(p, Component::Normal(_))),
        "unconfined artifact path"
    );
    let path = fs::canonicalize(root.join(relative))?;
    ensure!(
        path.starts_with(root) && path.is_file(),
        "artifact escapes repository or is not a file"
    );
    Ok(path)
}

fn descendants<'a>(id: &'a str, nodes: &BTreeMap<&'a str, &'a Node>) -> BTreeSet<&'a str> {
    let mut result = BTreeSet::new();
    let mut stack = vec![id];
    while let Some(id) = stack.pop() {
        if result.insert(id) {
            stack.extend(nodes[id].children.iter().map(String::as_str));
        }
    }
    result
}

fn algorithm_source(source: &str) -> bool {
    let parts: Vec<_> = source.split('/').collect();
    parts.len() == 4 && parts[0] == "algorithms" && parts[1] == "src" && parts[3] == "mod.rs"
}

fn role_name(role: &Role) -> &'static str {
    match role {
        Role::Forward => "forward",
        Role::ReverseReconstruction => "reverse_reconstruction",
        Role::BehaviorComplement => "behavior_complement",
    }
}

fn bind_formula(root: &Path, formula: &Formula, nodes: &BTreeMap<&str, &Node>) -> Result<()> {
    validate_formula(formula).map_err(|e| anyhow!(e))?;
    let node = nodes
        .get(formula.groupant_id.as_str())
        .context("unknown formula groupant ID")?;
    let source = confined(root, &formula.source.path)?;
    ensure!(
        sha(&bounded_read(&source)?) == formula.source.sha256,
        "stale formula source binding: {}",
        formula.id
    );
    let owned = descendants(&node.id, nodes);
    ensure!(
        owned
            .iter()
            .any(|id| nodes[id].source.as_deref() == Some(formula.source.path.as_str())),
        "formula source is not owned by groupant: {}",
        formula.id
    );
    let sidecar = node
        .source
        .as_ref()
        .map(|s| format!("{s}.groupant"))
        .unwrap_or_else(|| {
            node.groupant_path
                .clone()
                .unwrap_or_else(|| format!("models/{}.groupant", node.id))
        });
    ensure!(
        Path::new(&formula.model.path).starts_with(&sidecar),
        "model artifact does not belong to formula groupant"
    );
    // `file#Section` binds only that section's bytes (one plain H2 section of model.md).
    let (model_file, model_section) =
        mova_modelcheck::sections::split_reference(&formula.model.path);
    let model = confined(root, model_file)?;
    ensure!(
        model.starts_with(root.join(&sidecar)),
        "model symlink leaves declared owner sidecar"
    );
    let model_bytes = bounded_read(&model)?;
    let bound = match model_section {
        None => model_bytes,
        Some(section) => {
            let text = String::from_utf8(model_bytes).context("section reference needs UTF-8")?;
            mova_modelcheck::sections::section_bytes(&text, section)
                .with_context(|| {
                    format!(
                        "model section {section} missing or ambiguous: {}",
                        formula.id
                    )
                })?
                .as_bytes()
                .to_vec()
        }
    };
    ensure!(
        sha(&bound) == formula.model.sha256,
        "stale model artifact binding: {}",
        formula.id
    );
    ensure!(
        !formula.model.section.trim().is_empty() && !formula.source.symbol.trim().is_empty(),
        "empty source/model locator"
    );
    // Locators identify review scope; no Rust symbol resolution or Markdown proof extraction.
    Ok(())
}

/// Recognizes declared AST construction only; no semantic or independent source evidence.
fn derivation_provenance(formulas: &[Formula]) -> Result<Value> {
    derivation_provenance_with_budget(formulas, MAX_CANDIDATES)
}

fn derivation_provenance_with_budget(formulas: &[Formula], edge_budget: usize) -> Result<Value> {
    let ordered: BTreeMap<_, _> = formulas.iter().map(|f| (f.id.as_str(), f)).collect();
    let mut records = Vec::new();
    let mut recognized = 0;
    let mut converse_count = 0;
    let mut complement_count = 0;
    let mut edge_count = 0;
    for formula in ordered.values() {
        let construction = match (&formula.role, &formula.expr) {
            (Role::ReverseReconstruction, mova_correspondence::Expr::Converse { arg }) => {
                Some(("converse", arg.as_ref(), true))
            }
            (Role::BehaviorComplement, mova_correspondence::Expr::Complement { arg }) => {
                Some(("complement", arg.as_ref(), false))
            }
            _ => None,
        };
        let mut bases = BTreeSet::new();
        if let Some((_, arg, swap)) = construction {
            for base in ordered.values().filter(|f| f.role == Role::Forward) {
                let (domain, codomain) = if swap {
                    (&base.contract.codomain, &base.contract.domain)
                } else {
                    (&base.contract.domain, &base.contract.codomain)
                };
                if formula.groupant_id == base.groupant_id
                    && formula.source == base.source
                    && &formula.contract.domain == domain
                    && &formula.contract.codomain == codomain
                    && formula.contract.observation == base.contract.observation
                    && formula.contract.assumptions == base.contract.assumptions
                    && formula.contract.effects == base.contract.effects
                    && arg == &base.expr
                {
                    ensure!(
                        edge_count < edge_budget,
                        "provenance derivation edge cap exceeded"
                    );
                    bases.insert(base.id.as_str());
                    edge_count += 1;
                }
            }
        }
        let status = if formula.role == Role::Forward {
            "declared_forward"
        } else if bases.is_empty() {
            "not_recognized"
        } else {
            recognized += 1;
            match construction.unwrap().0 {
                "converse" => converse_count += 1,
                _ => complement_count += 1,
            }
            "recognized_derived"
        };
        let edges: Vec<_> = bases
            .iter()
            .map(|base| {
                json!({
                    "base_formula_id":base,
                    "construction":construction.unwrap().0
                })
            })
            .collect();
        records.push(json!({
            "formula_id":formula.id,"role":role_name(&formula.role),
            "status":status,"derivation_edges":edges
        }));
    }
    Ok(json!({
        "formulas":records,
        "counts":{"recognized_derived_formulas":recognized,
            "recognized_converse_formulas":converse_count,
            "recognized_complement_formulas":complement_count,
            "derivation_edges":edge_count},
        "scope":"Exact declared top-level AST construction with matching groupant, full source binding and contract. Recognized derived roles are not independent evidence from forward comparisons. Not recognized and declared forward do not assert independent verification. Source evidence remains separate; this is not a semantic proof.",
        "proof":false
    }))
}

pub fn scan(root: &Path, registry_path: &Path, catalog_path: &Path) -> Result<Value> {
    let root = fs::canonicalize(root)?;
    let registry_bytes = bounded_read(registry_path)?;
    let registry: Registry = serde_json::from_slice(&registry_bytes)?;
    ensure!(
        registry.nodes.len() <= MAX_REGISTRY_NODES,
        "registry node cap exceeded"
    );
    validate(&registry)?;
    let discovered = mova_modelcheck::rust_files(&root)?;
    ensure!(
        discovered.len() <= MAX_REGISTRY_NODES,
        "source count cap exceeded"
    );
    let mut total_source_bytes = 0u64;
    for source in &discovered {
        let size = fs::metadata(confined(&root, source)?)?.len();
        ensure!(size <= MAX_INPUT_BYTES, "source exceeds byte cap");
        total_source_bytes = total_source_bytes
            .checked_add(size)
            .context("source bytes overflow")?;
        ensure!(
            total_source_bytes <= 256 * 1024 * 1024,
            "aggregate source byte cap exceeded"
        );
    }
    let source_inventory = inventory(&root, &registry, &sha(&registry_bytes))?;
    let nodes: BTreeMap<_, _> = registry.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let algorithm_tree = if nodes.contains_key("algorithm-library") {
        descendants("algorithm-library", &nodes)
    } else {
        BTreeSet::new()
    };
    let algorithms: BTreeSet<_> = registry
        .nodes
        .iter()
        .filter(|n| n.source.as_deref().is_some_and(algorithm_source))
        .map(|n| n.id.as_str())
        .collect();
    let discovered_algorithms = source_inventory["files"]
        .as_array()
        .context("invalid source inventory")?
        .iter()
        .filter(|f| f["source"].as_str().is_some_and(algorithm_source))
        .count();
    ensure!(
        discovered_algorithms == algorithms.len(),
        "algorithm implementations missing registry groupants"
    );
    let sources: BTreeSet<_> = registry
        .nodes
        .iter()
        .filter(|n| {
            n.source
                .as_deref()
                .is_some_and(|s| !s.starts_with("algorithms/"))
        })
        .map(|n| n.id.as_str())
        .collect();
    let mut groups = BTreeSet::new();
    let mut mixed_groups = BTreeSet::new();
    for node in registry
        .nodes
        .iter()
        .filter(|n| n.source.is_none() && !algorithm_tree.contains(n.id.as_str()))
    {
        let children = descendants(&node.id, &nodes);
        let algorithm = children.iter().any(|id| {
            nodes[id]
                .source
                .as_deref()
                .is_some_and(|s| s.starts_with("algorithms/"))
        });
        let nonalgorithm = children.iter().any(|id| {
            nodes[id]
                .source
                .as_deref()
                .is_some_and(|s| !s.starts_with("algorithms/"))
        });
        if algorithm && nonalgorithm {
            mixed_groups.insert(node.id.as_str());
        } else if nonalgorithm {
            groups.insert(node.id.as_str());
        }
    }
    let catalog_bytes = bounded_read(catalog_path)?;
    let mut catalog: Catalog = serde_json::from_slice(&catalog_bytes)?;
    ensure!(catalog.schema_version == 1, "unsupported catalogue schema");
    ensure!(
        catalog.formulas.len() <= MAX_FORMULAS,
        "formula count cap exceeded"
    );
    catalog.formulas.sort_by(|a, b| a.id.cmp(&b.id));
    let mut ids = BTreeSet::new();
    for formula in &catalog.formulas {
        ensure!(ids.insert(formula.id.as_str()), "duplicate formula ID");
        bind_formula(&root, formula, &nodes)?;
    }
    let provenance = derivation_provenance(&catalog.formulas)?;
    let roles = [
        Role::Forward,
        Role::ReverseReconstruction,
        Role::BehaviorComplement,
    ];
    let mut missing = Vec::new();
    for (tier, set) in [
        ("algorithm", &algorithms),
        ("nonalgorithm_source", &sources),
        ("nonalgorithm_group", &groups),
        ("mixed_group", &mixed_groups),
    ] {
        for id in set {
            for role in &roles {
                if !catalog
                    .formulas
                    .iter()
                    .any(|f| f.groupant_id == *id && f.role == *role)
                {
                    missing.push(json!({"tier":tier,"groupant_id":id,"role":role_name(role),"status":"typed_formula_missing"}));
                }
            }
        }
    }
    // Equality uses the full contract; composition indexes typed intermediate endpoints.
    let mut right_index: BTreeMap<(String, Role, String, String), Vec<&Formula>> = BTreeMap::new();
    for formula in &catalog.formulas {
        let tier = if sources.contains(formula.groupant_id.as_str()) {
            "source"
        } else if groups.contains(formula.groupant_id.as_str()) {
            "group"
        } else if mixed_groups.contains(formula.groupant_id.as_str()) {
            "mixed_group"
        } else {
            continue;
        };
        for (kind, key) in [
            ("equality", serde_json::to_string(&formula.contract)?),
            (
                "domain",
                serde_json::to_string(&(
                    &formula.contract.domain,
                    &formula.contract.assumptions,
                    &formula.contract.effects,
                ))?,
            ),
            (
                "codomain",
                serde_json::to_string(&(
                    &formula.contract.codomain,
                    &formula.contract.assumptions,
                    &formula.contract.effects,
                ))?,
            ),
        ] {
            right_index
                .entry((tier.into(), formula.role.clone(), kind.into(), key))
                .or_default()
                .push(formula);
        }
    }
    let directions = [
        ("forward_forward", Role::Forward, Role::Forward),
        (
            "reverse_reverse",
            Role::ReverseReconstruction,
            Role::ReverseReconstruction,
        ),
        (
            "forward_reverse",
            Role::Forward,
            Role::ReverseReconstruction,
        ),
        (
            "reverse_forward",
            Role::ReverseReconstruction,
            Role::Forward,
        ),
        (
            "behavior_complement_comparison",
            Role::BehaviorComplement,
            Role::BehaviorComplement,
        ),
    ];
    let mut comparisons = Vec::new();
    let mut structural = Vec::new();
    let mut coincidences = Vec::new();
    let mut counterexamples = Vec::new();
    let mut summaries = Vec::new();
    let mut compositions = Vec::new();
    for (tier, set) in [
        ("source", &sources),
        ("group", &groups),
        ("mixed_group", &mixed_groups),
    ] {
        for (direction, left_role, right_role) in &directions {
            let before = comparisons.len();
            let lefts: Vec<_> = catalog
                .formulas
                .iter()
                .filter(|f| algorithms.contains(f.groupant_id.as_str()) && f.role == *left_role)
                .collect();
            let rights: Vec<_> = catalog
                .formulas
                .iter()
                .filter(|f| set.contains(f.groupant_id.as_str()) && f.role == *right_role)
                .collect();
            let formula_pairs = lefts
                .len()
                .checked_mul(rights.len())
                .context("formula pair overflow")?;
            let left_nodes: BTreeSet<_> = lefts.iter().map(|f| f.groupant_id.as_str()).collect();
            let right_nodes: BTreeSet<_> = rights.iter().map(|f| f.groupant_id.as_str()).collect();
            for left in &lefts {
                let mut keys = vec![("equality", serde_json::to_string(&left.contract)?)];
                if *left_role != Role::BehaviorComplement {
                    keys.push((
                        "domain",
                        serde_json::to_string(&(
                            &left.contract.codomain,
                            &left.contract.assumptions,
                            &left.contract.effects,
                        ))?,
                    ));
                    keys.push((
                        "codomain",
                        serde_json::to_string(&(
                            &left.contract.domain,
                            &left.contract.assumptions,
                            &left.contract.effects,
                        ))?,
                    ));
                }
                let mut admitted = BTreeMap::new();
                for (kind, key) in keys {
                    if let Some(rights) =
                        right_index.get(&(tier.to_string(), right_role.clone(), kind.into(), key))
                    {
                        for right in rights {
                            admitted.insert(right.id.as_str(), *right);
                        }
                    }
                }
                for right in admitted.values() {
                    ensure!(
                        comparisons.len() < MAX_CANDIDATES,
                        "candidate pair cap exceeded"
                    );
                    let comparison = compare(left, right).map_err(|e| anyhow!(e))?;
                    let record = json!({"tier":tier,"direction":direction,"comparison":comparison});
                    if comparison.structural_match {
                        structural.push(record.clone());
                    } else if comparison.finite_equal {
                        coincidences.push(record.clone());
                    }
                    if comparison.counterexample.is_some() {
                        counterexamples.push(record.clone());
                    }
                    comparisons.push(record);
                    if *left_role != Role::BehaviorComplement {
                        for (order, first, then) in [
                            ("algorithm_then_application", *left, *right),
                            ("application_then_algorithm", *right, *left),
                        ] {
                            let summary = compose_summary(first, then).map_err(|e| anyhow!(e))?;
                            compositions.push(json!({"tier":tier,"direction":direction,"order":order,"composition":summary}));
                        }
                    }
                }
            }
            let compared = comparisons.len() - before;
            summaries.push(json!({"tier":tier,"direction":direction,
                "groupant_pair_opportunities":algorithms.len().checked_mul(set.len()).context("pair count overflow")?,
                "groupant_pairs_with_both_roles":left_nodes.len()*right_nodes.len(),
                "typed_formula_pair_opportunities":formula_pairs,"typed_formula_pairs_compared":compared,
                "typed_formula_pairs_index_incompatible":formula_pairs-compared,
                "unformalized_status":"groupant pairs without both typed formula roles",
                "proof":false}));
        }
    }
    Ok(json!({
        "schema_version":1,"kind":"bounded_groupant_correspondence_scan",
        "registry_sha256":sha(&registry_bytes),"catalog_sha256":sha(&catalog_bytes),
        "coverage":{"algorithm_implementations":algorithms.len(),"nonalgorithm_sources":sources.len(),"nonalgorithm_groups":groups.len(),"mixed_groups":mixed_groups.len(),"catalog_formulas":catalog.formulas.len(),"missing_typed_roles":missing.len(),"compared_formula_pairs":comparisons.len()},
        "tiers":{"algorithm_implementations":algorithms,"nonalgorithm_sources":sources,"nonalgorithm_groups":groups,"mixed_groups":mixed_groups},
        "directions":summaries,"missing_typed_formulas":missing,
        "declared_derivation_provenance":provenance,
        "structural_equality_candidates":structural,"finite_coincidences":coincidences,"finite_counterexamples":counterexamples,
        "comparisons":comparisons,"finite_composition_exploration":compositions,"source_inventory":source_inventory,
        "limits":{"input_bytes":MAX_INPUT_BYTES,"formulas":MAX_FORMULAS,"candidate_pairs":MAX_CANDIDATES,"provenance_derivation_edges":MAX_CANDIDATES,"registry_nodes":MAX_REGISTRY_NODES,"source_files":MAX_REGISTRY_NODES,"aggregate_source_bytes_preflight":256 * 1024 * 1024},
        "scope":"Declared bounded typed relation models only. Source and model hashes establish revision bindings, not semantic correspondence. Pure nonalgorithm and mixed group tiers are separate; algorithm-only groups are excluded. Symbol and section locators remain unresolved review annotations. No arbitrary Markdown interpretation, universal proof or production substitution.",
        "proof":false,"production_substitution":false
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn algorithm_membership_excludes_test_and_surrogate_paths() {
        assert!(algorithm_source("algorithms/src/bfs/mod.rs"));
        for path in [
            "algorithms/src/lib.rs",
            "algorithms/src/bfs/tests.rs",
            "algorithms/src/bfs/mod.rs.groupant/rust_approximant.rs",
            "math/src/mod.rs",
        ] {
            assert!(!algorithm_source(path));
        }
    }
    #[test]
    fn unconfined_paths_fail_before_reading() {
        let root = fs::canonicalize(".").unwrap();
        for path in ["", "../Cargo.toml", "/etc/passwd"] {
            assert!(confined(&root, path).is_err());
        }
    }
}

#[cfg(test)]
mod binding_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        root: std::path::PathBuf,
        node: Node,
        formula: Formula,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn fixture() -> Fixture {
        let root = std::env::temp_dir().join(format!(
            "mova-correspondence-binding-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("unit.rs.groupant")).unwrap();
        fs::write(root.join("unit.rs"), "fn unit() {}").unwrap();
        fs::write(
            root.join("unit.rs.groupant/model.md"),
            "# Model\nBounded observation",
        )
        .unwrap();
        let node: Node = serde_json::from_value(json!({
            "id":"unit","source":"unit.rs","children":[],"description":"fixture","state":"x","cost":"1",
            "forward":["fixture"],"inverse":["fixture"],"checks":["fixture"],"limitations":["fixture"]
        })).unwrap();
        let sort = json!({"name":"bit","values":["0","1"]});
        let formula = serde_json::from_value(json!({
            "id":"unit-forward","groupant_id":"unit","role":"forward",
            "contract":{"domain":sort,"codomain":sort,"observation":"fixture","assumptions":[],"effects":[]},
            "expr":{"op":"identity","sort":sort},
            "source":{"path":"unit.rs","sha256":sha(b"fn unit() {}"),"symbol":"unit"},
            "model":{"path":"unit.rs.groupant/model.md","sha256":sha(b"# Model\nBounded observation"),"section":"@document"}
        })).unwrap();
        Fixture {
            root: fs::canonicalize(root).unwrap(),
            node,
            formula,
        }
    }
    #[test]
    fn valid_bindings_and_stale_source_are_distinct() {
        let f = fixture();
        let nodes = BTreeMap::from([(f.node.id.as_str(), &f.node)]);
        assert!(bind_formula(&f.root, &f.formula, &nodes).is_ok());
        fs::write(f.root.join("unit.rs"), "fn changed() {}").unwrap();
        assert!(
            bind_formula(&f.root, &f.formula, &nodes)
                .unwrap_err()
                .to_string()
                .contains("stale formula source")
        );
    }
    #[test]
    fn section_reference_binds_only_its_section() {
        let mut f = fixture();
        let nodes = BTreeMap::from([(f.node.id.as_str(), &f.node)]);
        fs::write(
            f.root.join("unit.rs.groupant/model.md"),
            "# M\n\n## Forward\n\nf\n\n## Other\n\no\n",
        )
        .unwrap();
        f.formula.model.path = "unit.rs.groupant/model.md#Forward".into();
        f.formula.model.sha256 = sha(b"## Forward\n\nf");
        assert!(bind_formula(&f.root, &f.formula, &nodes).is_ok());
        // Another section's edit leaves the binding fresh; its own edit makes it stale.
        fs::write(
            f.root.join("unit.rs.groupant/model.md"),
            "# M\n\n## Forward\n\nf\n\n## Other\n\nchanged\n",
        )
        .unwrap();
        assert!(bind_formula(&f.root, &f.formula, &nodes).is_ok());
        fs::write(
            f.root.join("unit.rs.groupant/model.md"),
            "# M\n\n## Forward\n\ng\n",
        )
        .unwrap();
        assert!(
            bind_formula(&f.root, &f.formula, &nodes)
                .unwrap_err()
                .to_string()
                .contains("stale model")
        );
        f.formula.model.path = "unit.rs.groupant/model.md#Missing".into();
        assert!(bind_formula(&f.root, &f.formula, &nodes).is_err());
    }
    #[test]
    fn stale_model_unknown_owner_and_foreign_model_reject() {
        let mut f = fixture();
        let nodes = BTreeMap::from([(f.node.id.as_str(), &f.node)]);
        let original = f.formula.model.sha256.clone();
        f.formula.model.sha256 = "0".repeat(64);
        assert!(bind_formula(&f.root, &f.formula, &nodes).is_err());
        f.formula.model.sha256 = original;
        f.formula.groupant_id = "unknown".into();
        assert!(bind_formula(&f.root, &f.formula, &nodes).is_err());
        f.formula.groupant_id = "unit".into();
        f.formula.model.path = "unit.rs".into();
        assert!(bind_formula(&f.root, &f.formula, &nodes).is_err());
    }
    #[test]
    fn full_scan_is_deterministic_and_rejects_duplicate_catalogue_ids() {
        let f = fixture();
        fs::create_dir_all(f.root.join("algorithms/src/example/mod.rs.groupant")).unwrap();
        fs::write(f.root.join("algorithms/src/example/mod.rs"), "fn unit() {}").unwrap();
        fs::write(
            f.root
                .join("algorithms/src/example/mod.rs.groupant/model.md"),
            "# Model\nBounded observation",
        )
        .unwrap();
        let mut algorithm_node = f.node.clone();
        algorithm_node.id = "algorithm-example".into();
        algorithm_node.source = Some("algorithms/src/example/mod.rs".into());
        let mut root_node = f.node.clone();
        root_node.id = "root".into();
        root_node.source = None;
        root_node.children = vec![algorithm_node.id.clone(), f.node.id.clone()];
        let registry = Registry {
            schema: 1,
            root: "root".into(),
            nodes: vec![root_node, algorithm_node, f.node.clone()],
        };
        let registry_path = f.root.join("registry.json");
        fs::write(&registry_path, serde_json::to_vec(&registry).unwrap()).unwrap();
        let mut algorithm_formula = serde_json::to_value(&f.formula).unwrap();
        algorithm_formula["id"] = json!("algorithm-forward");
        algorithm_formula["groupant_id"] = json!("algorithm-example");
        algorithm_formula["source"]["path"] = json!("algorithms/src/example/mod.rs");
        algorithm_formula["model"]["path"] =
            json!("algorithms/src/example/mod.rs.groupant/model.md");
        let mut catalog = json!({"schema_version":1,"formulas":[algorithm_formula,serde_json::to_value(&f.formula).unwrap()]});
        let catalog_path = f.root.join("catalog.json");
        fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
        assert!(
            std::process::Command::new("git")
                .arg("-C")
                .arg(&f.root)
                .arg("init")
                .output()
                .unwrap()
                .status
                .success()
        );
        let a = scan(&f.root, &registry_path, &catalog_path).unwrap();
        let b = scan(&f.root, &registry_path, &catalog_path).unwrap();
        assert_eq!(a, b);
        assert_eq!(a["coverage"]["algorithm_implementations"], 1);
        assert_eq!(a["coverage"]["nonalgorithm_sources"], 1);
        assert_eq!(a["coverage"]["mixed_groups"], 1);
        assert_eq!(a["coverage"]["nonalgorithm_groups"], 0);
        assert_eq!(a["coverage"]["missing_typed_roles"], 7);
        assert_eq!(
            a["structural_equality_candidates"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            a["finite_composition_exploration"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        let duplicate = catalog["formulas"][0].clone();
        catalog["formulas"].as_array_mut().unwrap().push(duplicate);
        fs::write(&catalog_path, serde_json::to_vec(&catalog).unwrap()).unwrap();
        assert!(
            scan(&f.root, &registry_path, &catalog_path)
                .unwrap_err()
                .to_string()
                .contains("duplicate")
        );
    }
    #[cfg(unix)]
    #[test]
    fn symlink_escape_rejects() {
        let f = fixture();
        std::os::unix::fs::symlink(std::env::temp_dir(), f.root.join("outside")).unwrap();
        assert!(confined(&f.root, "outside").is_err());
        fs::create_dir_all(f.root.join("foreign.rs.groupant")).unwrap();
        fs::write(
            f.root.join("foreign.rs.groupant/model.md"),
            b"# Model\nBounded observation",
        )
        .unwrap();
        fs::remove_file(f.root.join("unit.rs.groupant/model.md")).unwrap();
        std::os::unix::fs::symlink(
            f.root.join("foreign.rs.groupant/model.md"),
            f.root.join("unit.rs.groupant/model.md"),
        )
        .unwrap();
        let nodes = BTreeMap::from([(f.node.id.as_str(), &f.node)]);
        assert!(
            bind_formula(&f.root, &f.formula, &nodes)
                .unwrap_err()
                .to_string()
                .contains("model symlink")
        );
    }
}

#[cfg(test)]
mod provenance_tests {
    use super::*;
    use mova_correspondence::Expr;

    fn catalogue() -> Catalog {
        serde_json::from_str(include_str!("../../../models/correspondences.json")).unwrap()
    }

    fn pair() -> (Formula, Formula) {
        let catalog = catalogue();
        let base = catalog
            .formulas
            .iter()
            .find(|f| f.id == "gcd.algorithm")
            .unwrap()
            .clone();
        let mut derived = base.clone();
        derived.id = "derived".into();
        derived.role = Role::ReverseReconstruction;
        derived.expr = Expr::Converse {
            arg: Box::new(base.expr.clone()),
        };
        std::mem::swap(&mut derived.contract.domain, &mut derived.contract.codomain);
        (base, derived)
    }

    fn assert_unrecognized(base: Formula, derived: Formula) {
        let report = derivation_provenance(&[base, derived]).unwrap();
        assert_eq!(report["counts"]["recognized_derived_formulas"], 0);
        let record = report["formulas"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["formula_id"] == "derived")
            .unwrap();
        assert_eq!(record["status"], "not_recognized");
        assert_eq!(record["derivation_edges"], json!([]));
    }

    #[test]
    fn provenance_catalogue_gcd_constructions_are_derived_not_independent() {
        let catalog = catalogue();
        let gcd: Vec<_> = catalog
            .formulas
            .into_iter()
            .filter(|f| f.id.starts_with("gcd."))
            .collect();
        for formula in &gcd {
            validate_formula(formula).unwrap();
        }
        let report = derivation_provenance(&gcd).unwrap();
        assert_eq!(report["counts"]["recognized_derived_formulas"], 4);
        assert_eq!(report["counts"]["recognized_converse_formulas"], 2);
        assert_eq!(report["counts"]["recognized_complement_formulas"], 2);
        assert_eq!(report["counts"]["derivation_edges"], 4);
        assert_eq!(report["proof"], false);
    }

    #[test]
    fn provenance_rejects_same_atom_name_with_different_pairs() {
        let (base, mut derived) = pair();
        if let Expr::Converse { arg } = &mut derived.expr {
            if let Expr::Atom { pairs, .. } = arg.as_mut() {
                pairs.pop();
            }
        }
        validate_formula(&derived).unwrap();
        assert_unrecognized(base, derived);
    }

    #[test]
    fn provenance_requires_full_source_groupant_and_contract() {
        let (base, derived) = pair();
        for field in [
            "path",
            "sha256",
            "symbol",
            "groupant",
            "assumptions",
            "effects",
            "observation",
            "coordinates",
            "endpoints",
        ] {
            let mut changed = derived.clone();
            match field {
                "path" => changed.source.path.push_str(".other"),
                "sha256" => changed.source.sha256 = "0".repeat(64),
                "symbol" => changed.source.symbol.push_str(".other"),
                "groupant" => changed.groupant_id.push_str(".other"),
                "assumptions" => changed.contract.assumptions.reverse(),
                "effects" => changed.contract.effects.push("different".into()),
                "observation" => changed.contract.observation.push_str(" different"),
                "coordinates" => {
                    changed.contract.domain.values.reverse();
                    if let Expr::Converse { arg } = &mut changed.expr {
                        if let Expr::Atom { codomain, .. } = arg.as_mut() {
                            codomain.values.reverse();
                        }
                    }
                }
                "endpoints" => {
                    std::mem::swap(&mut changed.contract.domain, &mut changed.contract.codomain)
                }
                _ => unreachable!(),
            }
            assert_ne!(changed, derived, "{field}");
            assert_unrecognized(base.clone(), changed);
        }
    }

    #[test]
    fn provenance_retains_all_sorted_exact_bases_without_counting_edges_as_records() {
        let (base, derived) = pair();
        let mut alias = base.clone();
        alias.id = "a-forward-alias".into();
        alias.model.section = "separate model annotation".into();
        let report =
            derivation_provenance(&[base.clone(), derived.clone(), alias.clone()]).unwrap();
        assert_eq!(
            report,
            derivation_provenance(&[alias, derived, base]).unwrap()
        );
        assert_eq!(report["counts"]["recognized_derived_formulas"], 1);
        assert_eq!(report["counts"]["derivation_edges"], 2);
        let record = &report["formulas"][1];
        assert_eq!(record["formula_id"], "derived");
        assert_eq!(
            record["derivation_edges"],
            json!([
                {"base_formula_id":"a-forward-alias","construction":"converse"},
                {"base_formula_id":"gcd.algorithm","construction":"converse"}
            ])
        );
        assert_eq!(report["formulas"][0]["status"], "declared_forward");
    }

    #[test]
    fn provenance_edge_budget_accepts_boundary_and_rejects_next_edge() {
        let (base, derived) = pair();
        let mut alias = base.clone();
        alias.id = "a-forward-alias".into();
        let mut other = derived.clone();
        other.id = "other-derived".into();
        let formulas = [base, derived, alias, other];
        let accepted = derivation_provenance_with_budget(&formulas, 4).unwrap();
        assert_eq!(accepted["counts"]["derivation_edges"], 4);
        assert_eq!(accepted["counts"]["recognized_derived_formulas"], 2);
        let mut reversed = formulas.clone();
        reversed.reverse();
        assert_eq!(
            accepted,
            derivation_provenance_with_budget(&reversed, 4).unwrap()
        );
        for budget in [0, 1, 2, 3] {
            assert!(
                derivation_provenance_with_budget(&formulas, budget)
                    .unwrap_err()
                    .to_string()
                    .contains("provenance derivation edge cap")
            );
        }
        assert_eq!(
            derivation_provenance_with_budget(&[], 0).unwrap()["counts"]["derivation_edges"],
            0
        );
        let forward_only = [&formulas[0]].map(Clone::clone);
        assert!(derivation_provenance_with_budget(&forward_only, 0).is_ok());
    }

    #[test]
    fn provenance_does_not_infer_from_role_words_or_finite_equality() {
        let (base, mut derived) = pair();
        let converse = derived.expr.clone();
        derived.expr = Expr::Union {
            args: vec![converse],
        };
        validate_formula(&derived).unwrap();
        assert_unrecognized(base.clone(), derived.clone());
        derived.expr = base.expr.clone();
        derived.contract = base.contract.clone();
        assert_unrecognized(base.clone(), derived.clone());
        derived.expr = Expr::Complement {
            arg: Box::new(base.expr.clone()),
        };
        // A complementary expression in a reverse role is not a recognized reverse derivation.
        assert_unrecognized(base.clone(), derived.clone());
        derived.role = Role::BehaviorComplement;
        let report = derivation_provenance(&[base, derived]).unwrap();
        assert_eq!(report["counts"]["recognized_complement_formulas"], 1);
    }
}
