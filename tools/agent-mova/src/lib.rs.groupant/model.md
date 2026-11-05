# agent-mova-library

## model

# Native capability model
## Forward
Domain: validated schema-1 bounded tasks and owned regular artifacts on Linux. Observation: returned report, graph/checkpoint bytes, directory publication and error. Let W be Story reading the ordered window, q its location question, G the Retained graph, and A(G,q,L) the unique positive-score native answer over four alternatives. Trace admits v only when W(q)=v in L, every source read succeeds, the conservative context gate is clear, and A(ingest(G,subject-in-v),q,L)=v with selected own-lesson support. A saved checkpoint is that resulting graph. This is a source-scoped inference, not a truth predicate.
## Composition
Story inference and PragStudent analysis have different representations but share Mova dependencies. Agreement does not make independent evidence. Retained publication follows agreement; recall consumes pinned bytes through a sealed snapshot, not a replay of the window. Collector groups declared metadata after checking artifact bindings; its review queue is not a graph-training input.
## Invariant
For successful recall, fingerprint_after=fingerprint_before=task.state_fingerprint=parent.after_state_sha256. New frames and weight updates are zero. For uncertain trace, no checkpoint is saved. Collection preserves the ordered envelope list exactly and semantic_admission=false. Exclusive output names avoid overwrite, but I/O errors can leave partial artifacts.
## Cost
Let B be admitted bytes, N<=16 sentences, R<=32 reports, G graph size, and Cw,Ca native inference/scoring costs. Trace costs O(B)+Cw+Ca+O(serialized G); dependency internals are not assumed linear. Recall reads/decodes bounded G and scores four alternatives. Collection hashes at most 1 MiB and calls native table grouping, with key comparisons dependent on bounded labels. No measured speed advantage or error percentage.
## Descriptant
Vidtvorant scope is task-specific location behavior and durable episode knowledge. No general executor, arbitrary shell tool, biological assertion or automatic semantic correction exists here. Unknowns remain visible. The parent document hash is declared; the sentence-array hash is verified. Trusted directory ancestry and collision resistance are premises. Linux memfd sealing pins verified state bytes across the existing path-only decoder.
## behavior_complement
Domain and observation as Forward. An inner approximation of excluded successful outputs is E={(t,r): invalid window hash; duplicate alternatives; unsupported speaker context; unsupported/ambiguous native conclusion; collector native artifact identity/outcome mismatch; recall pin mismatch}. Each gate excludes the corresponding supported publication, not every malformed meaning. Counterexample to total coverage: an unmarked ambiguous third-person sentence may pass syntax/context checks. Evidence: source-derived gates, five focused tests and finite CLI negative controls; no complete semantic complement proof.
## reverse_reconstruction
For a supported report r, preimage F^-1(r) contains compatible ordered windows, task alternatives and ambient kind graphs satisfying the native admission relation. It is generally not a singleton: extra irrelevant sentences can preserve the same answer. Report input hashes narrow byte identity only when the corresponding task bytes are retained; they do not reveal missing text. A fingerprint does not reconstruct graph bytes. Evidence: relation inspected, synthetic retained graph restored; no injectivity theorem.
## retention_lineage
Domain: recall tuple(task,state,parent report). Observe accepted cold answer. Necessary equalities are H(state_bytes)=task.state_sha256, H(parent_bytes)=task.trace_report_sha256, parent.id=task.trace_id, parent.window_hash=task.source_window_sha256, and decoded fingerprint=task.state_fingerprint=parent.after_state_sha256. Counterexample to question-only cache identity: two states can answer the same question differently; the task pins prevent them sharing input identity. Hash binding is not authentic authorship. Evidence: source checks, successful cold restoration and wrong-state rejection.
## context_exclusion
Domain: bounded windows with optional speaker labels. Observe retention admission. Any speaker tag, recognized quote mark or case-insensitive first-person marker implies review-required uncertainty. Counterexample to a universal pragmatics gate: unmarked free indirect speech may escape this filter. The function is a partial exclusion, not a reverse reconstruction or automatic annotation repair. Evidence: focused marker tests and an actual speaker-tagged native trace.
## error_partition
Invalid task/window pins fail before creating the run directory. Native gaps/disagreement return uncertain reports without checkpoint. Decode/binding/file errors return Result::Err. Publication I/O can fail after creating a directory or checkpoint; absence of a final report does not prove zero side effects. Evidence: ordered source inspection and hash/schema negative controls; crash/concurrent-hostile-filesystem behavior remains unproved.

## artifact_identity_binding
Domain: a declared envelope m=(id,actor,kind,outcome,source,findings), artifact bytes b and reference digest h. Observation: collection acceptance and review queue. The predicate H(b)=h binds bytes but says nothing about m. For native_fish_report, acceptance additionally requires m=projection(decode_FishReport(b)) and schema=1. Counterexample to hash-only admission: retain b and its correct digest while replacing uncertain with supported, or id native-a with native-b. The hash still passes; the native projection check rejects before collection publication. Focused Rust controls exercise both mutations and an unchanged positive control. For external_review_evidence, decoding is intentionally absent: m remains a caller declaration and the stored artifact format preserves that uncertainty. Neither relation authenticates authorship or semantic truth. This is a source-derived necessary-condition model with finite executable evidence, not a proof of all report correctness.

## Two-flag admission projection

Fix one structurally valid native report, its correct digest and every declared
coordinate except id and outcome. Use a fresh writable output directory and
cooperative stable inputs; exclude unrelated parse, I/O, budget and other-field
failures. Observe collection admission only, never semantic approval. In the
ordered input domain [(id differs, outcome differs), (id differs, outcome agrees),
(id agrees, outcome differs), (id agrees, outcome agrees)], the observed relation
is [(0, refused), (1, refused), (2, refused), (3, admitted)].

The focused collector Rust test enumerates all four listed cases against the
actual collect function, holding report bytes and digest fixed. The hash-only
candidate admits all four and is falsified by the first three. This is finite
source agreement on a conditioned projection, not full-program equivalence or
proof of hash injectivity, scientific correctness or author identity.

The reverse fiber of refused is {0,1,2}; admission has fiber {3}. A refusal alone
cannot identify which compared field differed. The behavior complement is the
complement of the four observed input/output pairs inside this explicit 4 by 2
rectangle, not the complement of arbitrary program behavior. Converse and
complement formulas are exact mathematical derivations of the same forward
table and are dependent evidence. They are not separate Rust validations.

A composed forward-then-converse observation retains compatibility classes,
not the original flag input: the three refused cases collapse together. The
external_review_evidence branch is outside this model and preserves caller-
declared review metadata instead of claiming native identity conformance.

## Evidence-binding group partition

Domain: envelopes that passed the collector preflight. Define beta(r) as
native_bound when at least one referenced native artifact was decoded and matched
its envelope, and declared_only otherwise. The grouping key is
kappa(r)=(kind(r), outcome(r), beta(r)); hence equal keys preserve beta. This
structural binding category does not establish authorship, independent sources,
execution of the claimed task or semantic truth.

Counterexample to the former two-coordinate key: a native-bound envelope and a
merely declared envelope with the same kind/outcome collapsed into one count.
The focused Rust fixture now requires two groups with different binding labels
while preserving both envelopes. Archived groups without the field deserialize
as legacy_unclassified; missing evidence is not reconstructed as native-bound.
Evidence is finite plus source inspection; no full classifier or proof claim.

## checks

### Checks and evidence

This is finite synthetic evidence. No teacher call, full corpus, held-out assessment, daemon restart or learned general planner was exercised. No general semantic proof or speed comparison is supplied. Initial yanked-dependency lock resolution and three local compile errors were corrected before successful builds; their external logs remain preserved.

## History

### 2026-11-01 · checks

2026-11-01: offline locked two-job debug build; five unit tests passed. Two fresh CLI traces and two fresh recalls were byte-identical. Trace added one graph frame and changed zero weights; recall changed neither. Unknown subject and speaker-tagged context were uncertain, without checkpoint publication. Wrong state/window hashes and native outcome substitution were rejected. Native table collection retained three reports including uncertainty and reproduced identical bytes.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

#### Correspondence rebind 2026-11-04

Migration: the three catalogue formulas `native.identity_projection.forward`, `.reverse` and `.complement` (`models/correspondences.json`, symbol `collect`) were rebound from source SHA-256 `3c03bb63…` to `ffd0df19…`, 2026-11-04, operator decision. The only change between the two revisions is commit `8cff45c64` (the Shark engine renamed to the Whales), which changes one actor label string (`specialist-shark` → `specialist-whale`) and nothing in `collect` or the two-flag admission projection; the bound model section is unchanged. A deliberate, reviewed rebind, not a refresh of an unexplained drift.
