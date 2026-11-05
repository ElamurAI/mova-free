# crab-lib

## descriptant

### crab-lib

Public reusable Crab module boundary.

This source/privacy checkpoint separates reusable worker machinery from concrete deployment policy. It does not change live configs, binary pins, DBs or uncertain jobs.

#### Native Mova bridge

The public crate re-exports agent_mova as native_mova. collect_fish_reports directly invokes its collector, which checks hash-bound native/external artifacts and uses Mova's mlab table grouping. recall_fish_state directly invokes pinned retained graph recall. Neither function starts a shell, teacher, training operation or daemon. Existing control/store/supervisor/Dolphin APIs and specialized private-handler registration remain unchanged.

A native_fish_report reference must decode as the native schema and agree with envelope identity, actor, capability, source, findings and outcome. external_review_evidence is deliberately different: its labels are caller declarations over bounded hash-checked bytes. The output preserves those format tags, declared outcomes and every input envelope. Collection is systematization through Mova, not a learned log classifier, truth vote or semantic approval.

The dependency is public and one-way. Concrete task queues, runtime paths and report contents remain external. Exposing capabilities does not migrate any live daemon or certify an operational invocation. This bridge's local tests use uncertain report fixtures, external review bytes and a bad recall state pin; they do not ingest a new graph or train.

## model

# crab-lib

## forward

Public reusable Crab module boundary.

Export real control, versioned annotation storage, supervision and low-priority independent audit modules without a private Cargo edge.

## behavior_complement

Domain: explicit bounded CLI/configuration or trusted embedding calls in this file. Observation: returned errors/results and whether the selected downstream entrypoint was reached. Assume trusted local code/filesystem. Exclude an implicit private Whale/Eel implementation from public crate exports. Formula: accepted action implies declared dispatch preconditions; violating them excludes successful dispatch. This is a partial source-derived exclusion rather than a complete model of downstream behavior.

## reverse_reconstruction

Module names identify APIs, not a unique deployment configuration or runtime history. Formula: R(y) = {x | declared dispatch relation relates x to y}. Preserve every compatible input/configuration and hidden downstream history; do not infer a unique source task from a generic error. Counterexample: many invalid commands share the same usage error.

## invariant

Public code has no private actor dependency. Private embedding passes only server-side configuration to an explicitly installed handler. Scientific validity, learning admission and external-process success remain separate.

## cost

Argument/configuration decoding scales with admitted bytes; selected downstream cost is not bounded by this wrapper. No speedup or numerical approximation claim.

## composition

The public core owns control/store/supervisor/Dolphin logic. Concrete deployment handlers import it through a one-way Cargo edge. Running a daemon is independent of an active root chat. Durable review checkpoints retain explicit semantic/training/code-application boundaries.

## evidence

Checks: cargo test --manifest-path tools/crab/Cargo.toml --release --offline --locked -j2. Evidence remains finite local Rust checks; no theorem or live deployment claim.

## native_forward_composition

Domain: a validated native ReportIndex, owned artifact root and nonexistent output directory, or a valid pinned RecallTask and existing immutable artifacts. The bridge relation is C(root,index,out)=agent_mova.collect(root,index,out) and R(task,state,trace)=agent_mova.recall(task,state,trace); it adds no transformation or model verdict. Collection group keys are (kind,declared_outcome). Every admitted envelope appears once in the ordered review queue; sum of group counts equals envelope count. Native structural evidence and external declarations retain distinct format tags. Recall returns the native report only after checkpoint/trace/source lineage checks. No universal planning or classification claim follows.

## native_evidence_partition

Type: evidence_format_partition. Domain: admitted artifact references with native_fish_report or external_review_evidence. Observation: format tag, hash verification, native decoded fields and declared envelope fields. Native admission requires digest(bytes)=reference.sha256 and native(schema,id,actor,kind,outcome,source,findings)=envelope fields with schema1. External evidence requires bounded hash agreement but makes no decoded native-report claim. semantic_admission is false for both.

Counterexample: relabeling a native uncertain report as supported, changing its id/source/findings/schema, or substituting its artifact hash must fail before output creation. The same text may be explicitly admitted as external evidence with declared labels; that is a different evidence type, not a native verdict. Local Rust controls exercise both routes, including deterministic two-run collection and lossless queue retention. Assumptions: trusted ancestor directories, SHA integrity and downstream native validation. These are finite checks, not semantic proof.

## retained_lineage_exclusion

Type: retained_state_lineage_exclusion. Domain: native read-only recall over one existing source-scoped checkpoint, with explicit state-byte hash, payload fingerprint, parent-trace hash/id and source-window hash. Observation: native errors and before/after state fingerprints. A bad state hash excludes successful bridge recall before graph decoding. Successful recall retains state fingerprint and zero new frames/changed weights as reported by the native capability; no source passage replay or train call occurs here.

Counterexample: identical question text with a different checkpoint is not the same pinned invocation. The bridge's negative fixture supplies changed bytes and a missing parent; native refusal must identify state binding first and leave bytes unchanged. Positive cold recall through this bridge remains a root-owned checkpoint until invoked against existing evidence; no new graph ingestion was run by this change.

## native_reverse_reconstruction

Type: grouping_preimage. Given only grouped counts, compatible inputs include every admitted ordered envelope list with those keys/counts and any verified compatible artifacts. Counts alone do not reconstruct content or order. The retained full review queue instead preserves the input envelope list; artifact hashes constrain bytes under the no-collision premise but do not authenticate authors or certify claims. Error messages also have multiple compatible antecedents.

## native_cost_and_effects

Collection inherits caps: at most32 envelopes,1–3 references each,256KiB/file and1MiB total artifact bytes, plus bounded metadata/grouping and exclusive report/queue publication. Recall inherits a2MiB checkpoint bound and the cost of native graph decode/answer analysis. The wrapper performs no extra model pass, subprocess or retry. Output publication is not transactional: an I/O failure can leave an exclusive directory or one output; preserve it for inspection. No measured speedup or approximation percentage is claimed.

## worker_supervision_export

The crate exports `workers` (configured worker supervision). The export adds no private edge and no default
deployment: workers exist only when a trusted Crab configuration lists them.

## default_fish_workers

## tests

### Local checks

cargo test --manifest-path tools/crab/Cargo.toml --release --offline --locked -j2

No model/teacher/training call; normal Rust checks use temporary fixtures. Keep negative controls and downstream limitations distinct.

Native bridge controls:
- collection_reaches_native_mova_and_keeps_external_review_labels_explicit: actual Mova grouping, full queue retention, declared external evidence, two byte-identical fresh outputs and existing-output refusal.
- native_identity_source_outcome_and_hash_substitution_fail_before_output: bad native identity/outcome/hash/findings/source and schema are rejected.
- recall_bridge_rejects_changed_checkpoint_before_native_decoding: state pin refusal precedes parent read and leaves input unchanged.

No source trace ingestion, teacher/training, deployment daemon, operational DB or model call. Positive recall from existing checkpoints remains root-owned.

## History

### 2026-11-01 · tests

Evidence on 2026-11-01: offline locked debug and release checks pass (68 library and5 CLI tests, including the six new bridge controls), max2 build jobs. Native collection ran twice on fresh local fixture outputs with byte-identical reports. No operational graph ingestion or positive checkpoint recall was performed by this bridge change.

### 2026-11-04 · document

#### Dolphin pod (2026-11-04)

Exports `consult_queue` (Dolphin consult queue) and `tmux_worker` (Dolphin holder).

### 2026-11-04 · default_fish_workers

DEFAULT_FISH_WORKERS = 2 is the public Fish pool size when a deployment config omits `workers` (recommended minimum; configs may name up to 8). Used by angel status observation and supervisor sharding (2026-11-04).

### 2026-11-04 · document

#### Modules (2026-11-04)

Exports flow (work-flow probes) and council (Dolphin council and attention queue).

#### Epoch 2026-11-04 (pool)

module pool.

#### Epoch 2026-11-04 (pool)

module pool.
