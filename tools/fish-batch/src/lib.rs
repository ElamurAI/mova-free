//! Bounded source-bound annotation packing. No execution, import, training or source repair.
use annotation_review_worker::annotation::{
    self, Candidate, ContextLine, Evaluation, Profile, Task, Use,
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
pub const MAX_PACKETS: usize = 256;
const HEADER: &str = "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tteacher";
const MOSAIC:&str="Cached artifact mosaic: producer/model epochs are not established; observations are correlated.";
pub fn hash(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes.as_ref()))
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
/// Whitespace-only alias projection; no case, punctuation or source-character rewriting.
pub fn normalized_text_sha256(text: &str) -> String {
    hash(text.split_whitespace().collect::<Vec<_>>().join(" "))
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct SourcePin {
    pub path: String,
    pub sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacketInput {
    pub schema: u32,
    pub ordinal: u64,
    /// Opaque caller-declared ordering family; no sentence-ID parsing.
    #[serde(default)]
    pub order_scope: String,
    pub task: Task,
    pub context_ordinals: BTreeMap<String, u64>,
    pub source_pins: Vec<SourcePin>,
    pub provenance: Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackOptions {
    pub max_targets: usize,
    pub max_preceding_context: usize,
    pub allow_mosaic_artifacts: bool,
    #[serde(default)]
    pub excluded_ids: BTreeSet<String>,
    #[serde(default)]
    pub excluded_text_sha256: BTreeSet<String>,
}
impl Default for PackOptions {
    fn default() -> Self {
        Self {
            max_targets: 4,
            max_preceding_context: 8,
            allow_mosaic_artifacts: false,
            excluded_ids: BTreeSet::new(),
            excluded_text_sha256: BTreeSet::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchPrepared {
    pub schema: u32,
    pub task: Task,
    pub profile: Profile,
    pub evaluation: Evaluation,
    pub request: Value,
    pub target_ids: Vec<String>,
    pub source_pins: Vec<SourcePin>,
    pub provenance: Value,
    pub review_only: bool,
    pub training_authorized: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreparationRejection {
    pub target_ids: Vec<String>,
    pub ordinal: u64,
    pub reason: String,
    pub model_invoked: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackReport {
    pub schema: u32,
    pub input_packets: usize,
    pub prepared_targets: usize,
    pub batches: Vec<BatchPrepared>,
    pub rejections: Vec<PreparationRejection>,
    pub boundaries: Vec<Value>,
    pub model_invoked: bool,
    pub training_authorized: bool,
}
#[derive(Clone, Debug)]
struct Normal {
    input: PacketInput,
    task: Task,
    adapters: Vec<Value>,
    removed: Vec<Value>,
}
/// Explicit header-only adapter. All row bytes remain unchanged.
pub fn adapt_pragmatics_header(raw: &str) -> Result<(String, Value)> {
    ensure!(
        !raw.contains('\r'),
        "CR line endings are outside this adapter"
    );
    let (head, rest) = raw.split_once('\n').context("pragmatic header needs LF")?;
    ensure!(
        head == HEADER || head == "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tconf",
        "unsupported pragmatic header"
    );
    let adapted = format!("{HEADER}\n{rest}");
    Ok((
        adapted.clone(),
        json!({"adapter":if head==HEADER{"identity"}else{"conf_to_opaque_teacher_header_only"},
 "raw_sha256":hash(raw),"adapted_sha256":hash(&adapted),"row_fields_changed":false}),
    ))
}
fn excluded(c: &ContextLine, o: &PackOptions) -> bool {
    o.excluded_ids.contains(&c.id)
        || o.excluded_text_sha256
            .contains(&normalized_text_sha256(&c.text))
}
fn normalize(input: PacketInput, o: &PackOptions) -> Result<Normal> {
    ensure!(
        input.schema == 1 && input.task.target_ids.len() == 1,
        "one explicit target per schema1 packet required"
    );
    ensure!(
        input.task.dataset_use != Use::Evaluation,
        "evaluation targets excluded"
    );
    ensure!(
        input.task.context_sha256 == hash(serde_json::to_vec(&input.task.context)?),
        "raw typed context binding mismatch"
    );
    ensure!(
        (1..=64).contains(&input.source_pins.len()) && input.provenance.is_object(),
        "bounded pins and object provenance required"
    );
    let mut pins = BTreeMap::new();
    for p in &input.source_pins {
        ensure!(
            std::path::Path::new(&p.path).is_absolute()
                && !p.path.contains('\0')
                && digest(&p.sha256),
            "invalid source pin declaration"
        );
        if let Some(old) = pins.insert(&p.path, &p.sha256) {
            ensure!(old == &p.sha256, "conflicting source pin declaration");
        }
    }
    let target = &input.task.target_ids[0];
    let mut seen = BTreeSet::new();
    let mut orders = BTreeSet::new();
    for c in &input.task.context {
        ensure!(
            seen.insert(c.id.clone())
                && orders.insert(
                    *input
                        .context_ordinals
                        .get(&c.id)
                        .context("context ordinal missing")?
                ),
            "duplicate context id/ordinal"
        );
    }
    ensure!(
        seen.len() == input.context_ordinals.len()
            && input.context_ordinals.get(target) == Some(&input.ordinal),
        "context/target ordinal mapping mismatch"
    );
    let tc = input
        .task
        .context
        .iter()
        .find(|c| &c.id == target)
        .context("target absent")?;
    ensure!(!excluded(tc, o), "excluded target id/text");
    let mut task = input.task.clone();
    let mut adapters = vec![];
    let mut removed = vec![];
    task.context.retain(|c| {
        let keep = !excluded(c, o);
        if !keep {
            removed.push(json!({"id":c.id,"reason":"source_exclusion"}));
        }
        keep
    });
    if o.allow_mosaic_artifacts {
        for tc in &mut task.context {
            if !tc.unresolved.iter().any(|s| s == MOSAIC) {
                ensure!(
                    tc.unresolved.len() < 8,
                    "no unresolved slot for mosaic declaration"
                );
                tc.unresolved.push(MOSAIC.into());
            }
        }
    }
    task.context_sha256 = hash(serde_json::to_vec(&task.context)?);
    for c in &mut task.candidates {
        ensure!(
            hash(&c.ast) == c.ast_sha256 && hash(&c.pragmatics) == c.pragmatics_sha256,
            "raw candidate hash mismatch"
        );
        let (adapted, evidence) = adapt_pragmatics_header(&c.pragmatics)?;
        adapters.push(json!({"producer":c.producer,"evidence":evidence}));
        c.pragmatics = adapted;
        c.pragmatics_sha256 = hash(&c.pragmatics);
    }
    ensure!(
        hash(&task.witnesses) == task.witnesses_sha256,
        "raw witness hash mismatch"
    );
    Ok(Normal {
        input,
        task,
        adapters,
        removed,
    })
}
fn group_key(n: &Normal, o: &PackOptions) -> Value {
    json!({"document":n.task.document,"order_scope":n.input.order_scope,"document_sha256":n.task.document_sha256,"source_version":n.task.source_version,
 "dataset_use":n.task.dataset_use,"agreement_revision":n.task.agreement_revision,
 "producers":n.task.candidates.iter().map(|c|if o.allow_mosaic_artifacts{json!([c.producer])}else{json!([c.producer,c.revision])}).collect::<Vec<_>>(),
 "witness_revision":if o.allow_mosaic_artifacts{Value::Null}else{json!(n.task.witnesses_revision)}})
}
fn merge_table<'a>(tables: impl Iterator<Item = &'a str>) -> Result<String> {
    let mut head = None;
    let mut out = String::new();
    for raw in tables {
        ensure!(
            !raw.contains('\r'),
            "CR table framing is outside this composition"
        );
        let mut lines = raw.lines();
        let h = lines.next().context("empty table")?;
        if let Some(first) = head {
            ensure!(first == h, "table header mismatch")
        } else {
            head = Some(h);
            out.push_str(h);
            out.push('\n');
        }
        for row in lines {
            ensure!(!row.is_empty(), "empty TSV row");
            out.push_str(row);
            out.push('\n');
        }
    }
    Ok(out)
}
fn join_ast<'a>(blocks: impl Iterator<Item = &'a str>) -> String {
    let mut out = String::new();
    for raw in blocks {
        out.push_str(raw);
        out.push_str("\n\n");
    }
    out
}
fn compose(items: &[Normal], profile: &Profile, o: &PackOptions) -> Result<BatchPrepared> {
    ensure!(
        !items.is_empty() && items.len() <= o.max_targets,
        "batch target cardinality"
    );
    let first = &items[0];
    let key = group_key(first, o);
    let mut ids = BTreeSet::new();
    let mut contexts = BTreeMap::<String, (u64, ContextLine)>::new();
    let mut ordinal_ids = BTreeMap::new();
    let mut pins = BTreeMap::new();
    let mut removed = vec![];
    let mut previous = None;
    for n in items {
        ensure!(
            group_key(n, o) == key,
            "document/source/producer compatibility boundary"
        );
        ensure!(
            previous.is_none_or(|p| p < n.input.ordinal),
            "target order boundary"
        );
        previous = Some(n.input.ordinal);
        let target = &n.task.target_ids[0];
        ensure!(ids.insert(target.clone()), "duplicate target");
        for p in &n.input.source_pins {
            if let Some(old) = pins.insert(p.path.clone(), p.sha256.clone()) {
                ensure!(old == p.sha256, "source pin conflict");
            }
        }
        ensure!(pins.len() <= 64, "batch source-pin cardinality exceeds64");
        for c in &n.task.context {
            let ordinal = n.input.context_ordinals[&c.id];
            if let Some(old) = ordinal_ids.insert(ordinal, c.id.clone()) {
                ensure!(old == c.id, "context ordinal aliases two IDs");
            }
            if let Some((old_order, old)) = contexts.get(&c.id) {
                ensure!(
                    *old_order == ordinal && serde_json::to_vec(old)? == serde_json::to_vec(c)?,
                    "context observation conflict"
                );
            } else {
                contexts.insert(c.id.clone(), (ordinal, c.clone()));
            }
        }
        removed.extend(n.removed.clone());
    }
    let last = items.last().unwrap().input.ordinal;
    let mut extra: Vec<_> = contexts
        .iter()
        .filter(|(id, (order, _))| !ids.contains(*id) && *order <= last)
        .map(|(id, (order, _))| (*order, id.clone()))
        .collect();
    extra.sort();
    let keep_extra: BTreeSet<_> = extra
        .into_iter()
        .rev()
        .take(o.max_preceding_context)
        .map(|(_, id)| id)
        .collect();
    let mut admitted = vec![];
    for (id, (order, c)) in contexts {
        if ids.contains(&id) || keep_extra.contains(&id) {
            admitted.push((order, c));
        } else {
            removed.push(json!({"id":id,"reason":"declared_bounded_context_window"}));
        }
    }
    admitted.sort_by_key(|(order, _)| *order);
    let context: Vec<_> = admitted.into_iter().map(|(_, c)| c).collect();
    ensure!(context.len() <= 16, "batch context cardinality");
    let mut candidates = vec![];
    let mut origins = vec![];
    for slot in 0..first.task.candidates.len() {
        let observations:Vec<_>=items.iter().map(|n|json!({"target_id":n.task.target_ids[0],"ordinal":n.input.ordinal,
 "producer":n.input.task.candidates[slot].producer,"revision":n.input.task.candidates[slot].revision,
 "raw_ast_sha256":n.input.task.candidates[slot].ast_sha256,"raw_pragmatics_sha256":n.input.task.candidates[slot].pragmatics_sha256,
 "adapted_pragmatics_sha256":n.task.candidates[slot].pragmatics_sha256})).collect();
        let original = &first.task.candidates[slot];
        let ast = join_ast(items.iter().map(|n| n.task.candidates[slot].ast.as_str()));
        let pragmatics = merge_table(
            items
                .iter()
                .map(|n| n.task.candidates[slot].pragmatics.as_str()),
        )?;
        let producer = if o.allow_mosaic_artifacts {
            let proposed = format!("cached-{}-mosaic", original.producer);
            if proposed.len() <= 128 {
                proposed
            } else {
                format!("cached-artifact-mosaic-{slot}")
            }
        } else {
            original.producer.clone()
        };
        let revision = if o.allow_mosaic_artifacts {
            hash(serde_json::to_vec(&observations)?)
        } else {
            original.revision.clone()
        };
        candidates.push(Candidate {
            producer,
            revision,
            ast_sha256: hash(&ast),
            ast,
            pragmatics_sha256: hash(&pragmatics),
            pragmatics,
        });
        origins.push(json!({"candidate_slot":slot,"observations":observations}));
    }
    let witnesses = merge_table(items.iter().map(|n| n.task.witnesses.as_str()))?;
    let witness_origins:Vec<_>=items.iter().map(|n|json!({"target_id":n.task.target_ids[0],"ordinal":n.input.ordinal,"revision":n.input.task.witnesses_revision,"raw_sha256":n.input.task.witnesses_sha256})).collect();
    let mut task = first.task.clone();
    task.id = "batch-pending".into();
    task.context = context;
    task.context_sha256 = hash(serde_json::to_vec(&task.context)?);
    task.target_ids = items.iter().map(|n| n.task.target_ids[0].clone()).collect();
    task.candidates = candidates;
    task.witnesses = witnesses;
    task.witnesses_sha256 = hash(&task.witnesses);
    if o.allow_mosaic_artifacts {
        task.witnesses_revision = hash(serde_json::to_vec(&witness_origins)?);
    }
    task.id = format!("b{}", &hash(serde_json::to_vec(&task)?)[..48]);
    let evaluation = annotation::evaluate(&task)?;
    let request = annotation::render(&task, &evaluation, profile)?;
    Ok(BatchPrepared {
        schema: 1,
        target_ids: task.target_ids.clone(),
        task,
        profile: profile.clone(),
        evaluation,
        request,
        source_pins: pins
            .into_iter()
            .map(|(path, sha256)| SourcePin { path, sha256 })
            .collect(),
        provenance: json!({"schema":1,"kind":"bounded_source_observation_batch","mosaic_artifacts":o.allow_mosaic_artifacts,
 "revision_semantics":if o.allow_mosaic_artifacts{"Hash of ordered cached observation bindings; not one frozen model/checkpoint."}else{"Preserved same declared source artifact revisions; independence not established."},
 "candidate_origins":origins,"witness_origins":witness_origins,"context_removals":removed,
 "ast_framing":"Exact raw AST blocks followed by two LF separators; rows and source characters unchanged.",
 "tsv_framing":"One identical header and original row fields separated by LF.",
 "pragmatic_independence":false,"packets":items.iter().map(|n|json!({"target_id":n.task.target_ids[0],"ordinal":n.input.ordinal,
 "input_task_sha256":hash(serde_json::to_vec(&n.input.task).unwrap()),"header_adapters":n.adapters,"provenance":n.input.provenance})).collect::<Vec<_>>()}),
        review_only: true,
        training_authorized: false,
    })
}
/// Greedy bounded packing. A failed combined proposal flushes the previous valid batch;
/// the next target is retained as its already-validated singleton, never discarded.
pub fn pack(packets: Vec<PacketInput>, profile: &Profile, o: &PackOptions) -> Result<PackReport> {
    ensure!(
        (1..=8).contains(&o.max_targets) && o.max_preceding_context <= 8,
        "target limit1..8 and preceding-context limit0..8 required"
    );
    ensure!(
        packets.len() <= MAX_PACKETS,
        "bounded input page exceeds256 packets"
    );
    ensure!(
        o.excluded_text_sha256.iter().all(|h| digest(h)),
        "invalid excluded-text hash"
    );
    let mut report = PackReport {
        schema: 1,
        input_packets: packets.len(),
        prepared_targets: 0,
        batches: vec![],
        rejections: vec![],
        boundaries: vec![],
        model_invoked: false,
        training_authorized: false,
    };
    let mut current = Vec::<Normal>::new();
    let mut prepared = None;
    let mut seen = BTreeSet::new();
    for packet in packets {
        let ids = packet.task.target_ids.clone();
        let ordinal = packet.ordinal;
        let attempt = (|| -> Result<Normal> {
            ensure!(ids.len() == 1, "single-target packet required");
            ensure!(
                seen.insert(ids[0].clone()),
                "duplicate target within input page"
            );
            normalize(packet, o)
        })();
        let n = match attempt {
            Ok(n) => n,
            Err(e) => {
                report.rejections.push(PreparationRejection {
                    target_ids: ids,
                    ordinal,
                    reason: format!("{e:#}"),
                    model_invoked: false,
                });
                continue;
            }
        };
        let singleton = match compose(std::slice::from_ref(&n), profile, o) {
            Ok(b) => b,
            Err(e) => {
                report.rejections.push(PreparationRejection {
                    target_ids: ids,
                    ordinal,
                    reason: format!("{e:#}"),
                    model_invoked: false,
                });
                continue;
            }
        };
        if current.is_empty() {
            current.push(n);
            prepared = Some(singleton);
            continue;
        }
        let combined = if current.len() >= o.max_targets {
            Err(anyhow::anyhow!("configured target limit"))
        } else {
            let mut proposal = current.clone();
            proposal.push(n.clone());
            compose(&proposal, profile, o)
        };
        match combined {
            Ok(b) => {
                current.push(n);
                prepared = Some(b)
            }
            Err(e) => {
                let b = prepared.take().unwrap();
                report.prepared_targets += b.target_ids.len();
                report.batches.push(b);
                report.boundaries.push(json!({"next_target_ids":ids,"reason":format!("{e:#}"),"next_target_retained":true}));
                current.clear();
                current.push(n);
                prepared = Some(singleton);
            }
        }
    }
    if let Some(b) = prepared {
        report.prepared_targets += b.target_ids.len();
        report.batches.push(b);
    }
    ensure!(
        report.prepared_targets + report.rejections.len() == report.input_packets,
        "packing conservation failed"
    );
    Ok(report)
}

/// Exact packer/native implementation declarations for immutable deployment identity.
pub fn bindings() -> Value {
    json!({"fish_batch_source":hash(include_bytes!("lib.rs")),"annotation":annotation::bindings()})
}

#[cfg(test)]
mod tests {
    use super::*;
    use annotation_review_worker::annotation::{Adapter, Family, Negation, VoiceScope};
    fn packet(n: u64, doc: &str) -> PacketInput {
        let id = format!("{doc}:t{n}");
        let c = ContextLine {
            id: id.clone(),
            text: "Birds sing.".into(),
            speaker: None,
            referents: BTreeMap::new(),
            negation: Negation::Unknown,
            observation_scope: VoiceScope::Unknown,
            unresolved: vec!["Synthetic unknown context.".into()],
        };
        let ast=format!("# sent_id = {id}\n# text = Birds sing.\n1\tBirds\tbird\tNOUN\tNNS\tNumber=Plur\t2\tnsubj\t_\t_\n2\tsing\tsing\tVERB\tVBP\tMood=Ind|Number=Plur|Person=3|Tense=Pres|VerbForm=Fin\t0\troot\t_\t_\n3\t.\t.\tPUNCT\t.\t_\t2\tpunct\t_\t_\n\n");
        let prag=format!("sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tconf\n{id}\tdecl\tassert\t0\t0\tnone\tnarrator\t0.8\n");
        let candidates = (0..2)
            .map(|i| Candidate {
                producer: format!("native-{i}"),
                revision: hash(format!("revision-{i}")),
                ast: ast.clone(),
                ast_sha256: hash(&ast),
                pragmatics: prag.clone(),
                pragmatics_sha256: hash(&prag),
            })
            .collect();
        let witnesses = format!(
            "sent_id\tnli1_act\tnli1_act_p\tnli2_act\tnli2_act_p\n{id}\tassert\t0.8\tassert\t0.8\n"
        );
        let task = Task {
            schema: 1,
            id: format!("packet-{n}"),
            document: doc.into(),
            document_sha256: hash(doc),
            source_version: hash("source"),
            context: vec![c],
            context_sha256: String::new(),
            target_ids: vec![id.clone()],
            dataset_use: Use::Unknown,
            candidates,
            witnesses: witnesses.clone(),
            witnesses_sha256: hash(&witnesses),
            witnesses_revision: hash("witness"),
            agreement_revision: annotation::bindings()["prag_agree"]
                .as_str()
                .unwrap()
                .into(),
        };
        let mut p = PacketInput {
            schema: 1,
            ordinal: n,
            order_scope: "fixture-order".into(),
            task,
            context_ordinals: BTreeMap::from([(id, n)]),
            source_pins: vec![SourcePin {
                path: "/fixture/source".into(),
                sha256: hash("source"),
            }],
            provenance: json!({"fixture":true}),
        };
        reseal(&mut p);
        p
    }
    fn reseal(p: &mut PacketInput) {
        p.task.context_sha256 = hash(serde_json::to_vec(&p.task.context).unwrap());
    }
    fn profile() -> Profile {
        Profile {
            family: Family::Sonnet,
            adapter: Adapter::Cli,
            executable: Some("/fixture/claude".into()),
            model: Some("sonnet".into()),
        }
    }
    #[test]
    fn four_targets_use_actual_native_gates_and_preserve_unknowns() {
        let packets = (0..5).map(|i| packet(i, "d")).collect();
        let a = pack(packets, &profile(), &PackOptions::default()).unwrap();
        assert_eq!(
            a.batches
                .iter()
                .map(|b| b.target_ids.len())
                .collect::<Vec<_>>(),
            vec![4, 1]
        );
        assert!(a.rejections.is_empty());
        assert_eq!(a.prepared_targets, 5);
        let b = &a.batches[0];
        assert_eq!(b.task.dataset_use, Use::Unknown);
        assert!(!b.training_authorized && b.review_only);
        assert!(!b.evaluation.eligible_for_gpu_admission_queue);
        assert!(b
            .evaluation
            .findings
            .iter()
            .any(|f| f.code == "not_teaching"));
        assert_eq!(b.task.candidates[0].pragmatics.lines().count(), 5);
    }
    #[test]
    fn artifact_mosaic_is_explicit_and_preserves_original_revisions() {
        let a = packet(0, "d");
        let mut b = packet(1, "d");
        b.task.candidates[0].revision = hash("other");
        b.task.witnesses_revision = hash("other-witness");
        let strict = pack(
            vec![a.clone(), b.clone()],
            &profile(),
            &PackOptions::default(),
        )
        .unwrap();
        assert_eq!(strict.batches.len(), 2);
        let o = PackOptions {
            allow_mosaic_artifacts: true,
            ..PackOptions::default()
        };
        let mixed = pack(vec![a, b.clone()], &profile(), &o).unwrap();
        assert_eq!(mixed.batches.len(), 1);
        let batch = &mixed.batches[0];
        assert!(batch.task.candidates[0].producer.ends_with("-mosaic"));
        assert_ne!(
            batch.task.candidates[0].revision,
            b.task.candidates[0].revision
        );
        assert_eq!(
            batch.provenance["candidate_origins"][0]["observations"][1]["revision"],
            b.task.candidates[0].revision
        );
        assert!(batch
            .task
            .context
            .iter()
            .all(|c| c.unresolved.iter().any(|s| s == MOSAIC)));
    }
    #[test]
    fn deterministic_packing_and_identity_boundaries() {
        let p: Vec<_> = (0..8).map(|i| packet(i, "d")).collect();
        let o = PackOptions {
            max_targets: 8,
            ..PackOptions::default()
        };
        let a = pack(p.clone(), &profile(), &o).unwrap();
        let b = pack(p, &profile(), &o).unwrap();
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&b).unwrap()
        );
        assert_eq!(a.batches[0].target_ids.len(), 8);
        let distinct = pack(vec![packet(0, "a"), packet(1, "b")], &profile(), &o).unwrap();
        assert_eq!(distinct.batches.len(), 2);
        let mut other_scope = packet(1, "d");
        other_scope.order_scope = "different-declared-order".into();
        assert_eq!(
            pack(vec![packet(0, "d"), other_scope], &profile(), &o)
                .unwrap()
                .batches
                .len(),
            2
        );
    }
    #[test]
    fn hundred_k_boundary_splits_without_losing_next_target() {
        let mut a = packet(0, "d");
        let mut b = packet(1, "d");
        for p in [&mut a, &mut b] {
            for c in &mut p.task.candidates {
                c.ast = format!("# padding = {}\n{}", "x".repeat(30_000), c.ast);
                c.ast_sha256 = hash(&c.ast);
            }
        }
        let r = pack(vec![a, b], &profile(), &PackOptions::default()).unwrap();
        assert_eq!(r.prepared_targets, 2);
        assert!(r.rejections.is_empty());
        assert_eq!(r.batches.len(), 2);
        assert!(r
            .boundaries
            .iter()
            .any(|v| v["reason"].as_str().unwrap().contains("100000")));
        assert!(r
            .batches
            .iter()
            .all(|b| b.request["submitted_utf8_bytes"].as_u64().unwrap() <= 100000));
        assert_eq!(r.batches[1].target_ids, vec!["d:t1"]);
    }
    #[test]
    fn exclusions_and_unknown_context_are_explicit() {
        let a = packet(0, "d");
        let mut b = packet(1, "d");
        b.task.context.insert(0, a.task.context[0].clone());
        b.context_ordinals.insert("d:t0".into(), 0);
        reseal(&mut b);
        let o = PackOptions {
            excluded_ids: BTreeSet::from(["d:t0".into()]),
            ..PackOptions::default()
        };
        let r = pack(vec![a, b], &profile(), &o).unwrap();
        assert_eq!(r.rejections.len(), 1);
        assert_eq!(r.batches.len(), 1);
        assert_eq!(r.batches[0].task.context.len(), 1);
        assert!(r.batches[0].provenance["context_removals"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["id"] == "d:t0" && v["reason"] == "source_exclusion"));
        let text_o = PackOptions {
            excluded_text_sha256: BTreeSet::from([normalized_text_sha256(" Birds  sing. ")]),
            ..PackOptions::default()
        };
        assert_eq!(
            pack(vec![packet(2, "d")], &profile(), &text_o)
                .unwrap()
                .rejections
                .len(),
            1
        );
    }
    #[test]
    fn malformed_hash_and_evaluation_never_hide_other_targets() {
        let a = packet(0, "d");
        let mut bad = packet(1, "d");
        bad.task.candidates[0].ast_sha256 = hash("wrong");
        let mut e = packet(2, "d");
        e.task.dataset_use = Use::Evaluation;
        let r = pack(
            vec![a, bad, e, packet(3, "d")],
            &profile(),
            &PackOptions::default(),
        )
        .unwrap();
        assert_eq!(r.prepared_targets, 2);
        assert_eq!(r.rejections.len(), 2);
        assert_eq!(r.batches[0].target_ids, vec!["d:t0", "d:t3"]);
        assert!(r
            .rejections
            .iter()
            .any(|v| v.reason.contains("raw candidate")));
        assert!(r.rejections.iter().any(|v| v.reason.contains("evaluation")));
    }
    #[test]
    fn bounded_context_declares_every_removed_id() {
        let mut p = packet(15, "d");
        for n in 0..15 {
            let c = packet(n, "d").task.context.remove(0);
            p.task.context.push(c);
            p.context_ordinals.insert(format!("d:t{n}"), n);
        }
        p.task.context.sort_by_key(|c| p.context_ordinals[&c.id]);
        reseal(&mut p);
        let r = pack(vec![p], &profile(), &PackOptions::default()).unwrap();
        let b = &r.batches[0];
        assert_eq!(b.task.context.len(), 9);
        let removed = b.provenance["context_removals"].as_array().unwrap();
        assert_eq!(removed.len(), 7);
        assert!(removed
            .iter()
            .all(|v| v["reason"] == "declared_bounded_context_window"));
        assert_eq!(b.task.context[0].id, "d:t7");
        assert_eq!(b.task.context.last().unwrap().id, "d:t15");
    }
    #[test]
    fn header_adaptation_retains_row_fields_and_rejects_other_protocols() {
        let raw="sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tconf\nd:t1\tdecl\tassert\t0\t0\tnone\tnarrator\t0.2,0.4\n";
        let (adapted, e) = adapt_pragmatics_header(raw).unwrap();
        assert_eq!(adapted.lines().nth(1), raw.lines().nth(1));
        assert_eq!(e["row_fields_changed"], false);
        assert!(adapt_pragmatics_header("id\tact\nx\tassert\n").is_err());
        assert!(adapt_pragmatics_header(&raw.replace('\n', "\r\n")).is_err());
    }
}
