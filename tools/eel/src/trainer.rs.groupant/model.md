# mova-eel-native-trainer

## descriptant

### Public native training engine

This is the actual snapshot-to-agreement-to-gparse learning/installation controller,
not a trait-only simulation. Backend selection is injectable; baseline, bounded
native continuation, reload, development/anchor gates and conditional compatible
weight installation stay in this public module. Adapter authority and heldout
separation are explicit assumptions, not inferred from an enum variant. Priority
selection is before ordinary DB polling and must durably acknowledge terminal work.

## model

# Parser learning controller model

Subject: the manifest-bound trainer source, called by the Peta CLI with an operator-owned, pinned configuration. Data and receipts are external. A store fingerprint describes selected paired revisions and real parser witnesses; it is not a correctness proof.

## Forward approximant

Poll a stable DB selection fingerprint. If its generation has a terminal receipt, wait for new data. A reservation without a terminal receipt pauses as uncertain instead of resubmitting. Under a single controller lease and shared GPU lease, create an immutable SQLite export, align the actual Stanza/Trankit/spaCy transformer witnesses by IDs/text/forms, then invoke existing treeagree rules. Missing witnesses stay outside that agreed training subset; they are never filled with teacher copies.

Admit only fresh ids whose prefix is in fresh_train_prefixes (each a lowercase name and ':'; default ['school:'], so existing configurations are unchanged; a hard-case deployment names ['hard:']); any other id fails the round as nonteaching. Filter the derived agreement against the entire selected teacher train view, dynamic lesson dev, pinned fixed dev, protected ID/text metadata and target vocabularies. Replay excludes every selected ID, including selected train IDs missing witnesses, plus dev lessons/texts, protected IDs/texts and question/math IDs. Too-long records stay in the DB but are excluded because the current parser skips lengths at least min(model limit,256). Count changed supervision signatures, not changed comments alone. Below the new-record threshold, retain the CPU agreement evidence without backing up weights or launching GPU commands.

Otherwise save a complete model configuration/vocabulary/weights backup, evaluate the same frozen development inputs, run bounded continuation training, require explicit CUDA plus finite losses and a saved checkpoint, reload the saved candidate, then gate LAS: first fixed development set may lose at most 0.5 percentage points; every later set must not regress. These are selection-biased development/silver regression measurements: the same dev sets select checkpoints. They are not heldout generalization.

Validate unchanged deployed weights, compatible configuration (including default Erf GELU) and identical vocabularies before an individually atomic weight replacement. Config/vocab files stay in place. Persist the new supervision baseline and terminal receipt. Failures before installation keep old weights; errors/crashes after the rename can leave new weights with an incomplete receipt, requiring inspection. A resource, malformed-output or uncertain failure creates trainer-specific PAUSE; annotation and DB ingestion remain independent. Finite development/anchor gate rejections are typed terminal REJECTED candidates, preserve the old model and do not pause. They cannot retry as the same generation/experiment.

## Invariant and cost

No teacher/API call or protected-label assessment command originates here. Exactly one cooperating trainer owns its controller lease. The parent GPU descriptor always remains CLOEXEC. Its raw descriptor is passed explicitly only to gparse command builders; Command::pre_exec clears CLOEXEC in that GPU child's post-fork copy. CPU agreement and unrelated parallel children do not inherit. Closing the parent descriptor does not explicitly unlock an intended child's still-live open-file-description lease. No automatic retry of a terminal or uncertain generation. Model/data identity changes require explicit new configuration/run root. Historical model exposure remains outside current exclusion guarantees.

Input scans cost O(T+R+D) bytes plus O(sum sentence length squared) head/cycle validation, vocabulary lookup and whole-file hashing. Model backup/installation costs O(W) bytes and retained disk space. The existing parser dominates GPU cost; no new runtime bound or speed claim is proved. Before export, qualifying backup and training, require a 10 GB free-disk floor and default 50 GB trainer run cap. These checks do not constitute a filesystem quota against unrelated writers.

## Composition

store::status -> immutable store::export -> actual treeagree -> filtered data -> gparse baseline/train/reload -> conditional weight install. A correct store row alone does not prove this wiring. Controller/gpu/model installer locks serve distinct scopes. No transaction atomically spans SQLite, artifacts and model weights. Native fake-process tests cover order, rejections and retained bytes; real GPU validation belongs to the parent campaign.

## Absolute developer anchor

An optional fixed `ewt_las_floor=A` adds LAS_after_0 >= A immediately before installation, after candidate reload and ordinary local comparison. Finite A must lie in [0,100]. Eel requires an explicit anchor; ordinary trainer callers retain compatibility when it is absent. This stops repeated locally tolerated losses from accumulating beneath the configured anchor. It applies only to the same pinned first fixed developer set; it is not a generalization guarantee or a bound on every metric.

## Planned sampled experiments

Ordinary training keeps GP_SAMPLE=1 by default. An explicitly numeric gp_sample in (0,1] changes the existing parser's sampling fraction, not source eligibility or quality gates. Eel supplies a bounded fraction with a fresh planned seed. Installed and rejected receipts bind seed, fraction, epoch count and learning rate. Only the typed CandidateRejected error from finite developer-LAS/anchor comparisons can yield REJECTED; resource/I/O/CPU/malformed/timeout failures remain failed/uncertain. Before writing a rejection receipt, verify the deployed weights still match the saved parent and the candidate hash is readable. These are new planned stochastic experiments, not automatic retries of an identical rejected generation.

## Public native training composition

The subject is tools/eel/src/trainer.rs. A TrainingData adapter replaces the two
private DB calls. Polling is a hint: a newer immutable exported generation is
legitimate and owns the reservation. Validate lowercase SHA-256 poll identities
before constructing generation paths, as already done for exported identities.
Configured native gparse/treeagree calls remain real; the public engine is neither
a queue ledger nor a semantic-training mock. Tests substitute executable fixtures
for bounded Rust wiring checks, not as evidence of actual GPU learning.

PriorityWork is evaluated before ordinary DB status/export. Reviewed selects its
immutable export through the identical leased/gated training pipeline; AwaitingReview
returns waiting_for_review without ordinary training. Terminal installed/rejected/
below-threshold receipts retain the diagnostic ID and call the adapter's durable
acknowledgement. A repeated same-run terminal generation retries acknowledgement,
never CUDA work. Acknowledgement is not atomic with installation; an adapter error
propagates and Eel pauses an uncertain round. Backends own cross-round deduplication,
review/source/privacy/heldout pins and deterministic queue ranking. Defaults preserve
ordinary callers; no private priority bridge or live migration follows from this API.

## catalog

# Typed controller obernants

Subject is the exact manifest-bound source. These are partial source derivations over cooperating processes and ordinary I/O, supported by finite native controls. They are not formal proofs or semantic annotation guarantees.

## behavior_complement

Domain: finite operator-owned config/data/model files, exclusive controller ownership and an intact generation journal. Observe child invocation separately from deployed bytes. If R is the successful-installed transition relation in universe U, invalid witness alignment, protected fresh data, unknown/reserved targets, missing explicit CUDA/finite losses/saved checkpoint, failed dev gate, incompatible vocabulary/config or changed old weights exclude an installed result before the installation boundary. These cases form a source-derived inner approximant to U minus R. Below-threshold cases exclude GPU invocation.

Assumptions: declared native binaries, trusted pinned evidence and no hostile concurrent writer. CPU fallback may execute but cannot pass activation. Counterexample to 'all errors retain old weights': a receipt error or crash after atomic weight rename may already have installed them. Oracle: mock CPU/regression controls keep old bytes; threshold control invokes no GPU command; uncertain reservation never submits. Evidence: bounded tests, not arbitrary power-loss behavior.

## reverse_reconstruction

Domain: complete before-model configuration, four vocabularies and weights, plus retained inputs/logs. Observe that model-file byte tuple, excluding optimizer state, process memory, auxiliary old logs and intent. The snapshot reconstructs those earlier bytes; current weights alone admit many prior checkpoints, orders and annotation histories. No unique inverse of gradient training is claimed.

Counterexample: a provenance comment change can yield identical supervision and weights. Missing historical teacher judgments cannot be recovered from a generation receipt. Oracle: success fixture retains exact old bytes beside installed new bytes; failure fixture preserves deployed old bytes. Evidence is finite; automatic recovery after an uncertain post-install crash is absent.

## supervision_fibers

Type: reverse_reconstruction for the training projection. Domain: validated integer-token CoNLL-U under this parser's no-role training mode. P records IDs and ordered (form, UPOS, XPOS, head, relation) rows. Compatible full annotations for observation y form {x:P(x)=y}; comments, lemmas, FEATS and MISC can vary inside a fiber. SHA256 signatures are operational bindings, not a collision-free mathematical theorem.

The trigger counts IDs whose P differs from the prior trained baseline. Counterexample to byte-change counting: add an annotation comment; P is equal. Counterexample to overbroad equivalence: NN becomes NNP; P differs. Oracle: supervision_signature_ignores_metadata_but_not_labels. Evidence: actual Rust projection; ignored fields may matter elsewhere in Mova.

## missing_witness_exclusion

Type: partial_exclusion. Domain: selected teacher train view S and actual complete three-parser witness subset W. Agreement trains on W; replay excludes every ID in S, including S minus W. With dev/protected exclusions E, replay IDs are contained in Replay minus (S union E). A teacher copy cannot supply an independent witness.

Assumptions: store provenance plus this worker's order/ID/text/form alignment. Missing coverage postpones a record, without proving it wrong. Counterexample to excluding only W: an unwitnessed selected ID returns through replay and bypasses agreement. Oracle: replay_excludes_selected_records_missing_witnesses. Evidence: finite control plus source-derived containment, not gold parsing accuracy.

## checkpoint_gate_partition

Type: error_partition over the first failed phase of a normal trace: data/agreement, threshold, baseline, bounded training, saved-candidate reload, score gate, model identity/install, receipt persistence. Multiple defects may coexist; these are first observed failures, not disjoint root causes. Gate i requires LAS_after_i >= LAS_before_i - delta_i, with delta_0=0.5 percentage points and delta_i=0 for later sets, and identical nonzero token counts.

These development sets also select the saved epoch, so results are selection-biased silver regression, not unseen-test generalization or numerical approximation guarantees. Counterexample to final-epoch metrics: an earlier epoch can be saved. Oracle: reload the saved candidate, exact-path score parsing and NaN/missing/duplicate-line controls. Handled resource/malformed failures pause only training; typed finite quality-gate rejection is terminal REJECTED with the old model retained; missing terminal receipts are uncertain on restart. Evidence: mock protocol/gate/idempotence checks; arbitrary death after installation remains manual recovery.

## inherited_gpu_lease

Type: resource_state_preimage. Domain: Linux cooperating flock users on one file, with a child inheriting an open file description. Observe whether a separately opened contender acquires the lease. Parent descriptor closure with a live inherited child still permits an unavailable lease; availability cannot reveal the holder identity. Release follows the last open-file-description reference, not controller liveness.

Counterexample: explicit LOCK_UN on parent drop would release the child's still-live GPU lease. This controller closes its descriptor without explicit unlock and clears FD_CLOEXEC only inside the intended GPU child's Command::pre_exec hook; the parent descriptor stays CLOEXEC. Oracle: a child holds the lease after parent closure; a contender succeeds after child exit. Evidence: native descriptor/process test, not distributed scheduling or fairness proof.

## absolute_anchor_exclusion

Type: partial_exclusion. Domain: an explicitly configured finite fixed anchor A and the unchanged first pinned development set, after a valid saved-candidate reload. Successful installation requires LAS_after_0 >= A in addition to local gates. Counterexample to relying on the local 0.5-point tolerance: 85.44 -> 85.00 -> 84.60 passes both local losses, but A=84.94 excludes the second activation. Oracle: absolute_anchor_prevents_cumulative_local_gate_drift checks this sequence and optional-field compatibility. Evidence: bounded Rust checks and source placement immediately before install; repeated development selection remains biased, not an unseen-data theorem.

## terminal_quality_partition

Type: error_partition over normal candidate evaluation. After valid finite metrics, a failed developer/anchor predicate G produces typed CandidateRejected; with readable candidate and unchanged parent weights this becomes terminal REJECTED, model untouched. A true G still requires all remaining installation checks. Malformed metrics, CPU fallback, I/O, timeout or diverged parent identity remain FAILED/uncertain and pause. Counterexample to string matching: an unrelated error containing a gate phrase is not this typed variant. Oracle: regression mock is rejected without PAUSE; CPU/malformed mocks fail with PAUSE, all preserve old weights. Ordinary same-generation replay remains blocked. Evidence: bounded native/mock checks, no changed gate thresholds or semantic/generalization claim.

## unrelated_child_exclusion

Type: partial_exclusion over descriptor inheritance. Domain: cooperating Unix parent threads, a CLOEXEC parent GPU descriptor and an intended GPU command with a child-only post-fork fcntl hook. The parent descriptor flags remain unchanged; unrelated exec children have no surviving reference to that descriptor. The intended child alone clears its copied flag and retains the shared open-file-description lease. Counterexample to a parent-global flag change: a simultaneously spawned unrelated long-lived child accidentally holds the lease after GPU work exits. Oracle: barrier-synchronized GPU and unrelated piped children; the GPU child keeps the lease after parent closure, but after its exit a contender acquires while the unrelated child is still alive. Evidence: native parallel debug/release tests plus source-derived fork/exec argument; no distributed/fairness proof.

## priority_lane_admission

Domain: PriorityWork plus native training configuration and versioned export.
Observation: calls to ordinary status/export, native commands and terminal ack.
Assumptions: adapter authenticates reviewed immutable data, ranks deterministically
and acknowledges durably/idempotently; existing developer/protected gates remain.
Relation: AwaitingReview => zero ordinary calls and zero gparse training;
Reviewed(e) => select e before ordinary status/export; terminal(e) => acknowledge
only installed/rejected/below_threshold. Counterexample: a raw deputy failure with
training_authorized=false cannot satisfy Reviewed; an ack write failure after
installation does not undo installed weights or authorize recomputation. Evidence:
three finite Rust priority fixtures including a post-install ack fault and no
compute replay. This is partial source-derived exclusion, not a fairness proof.

## polling_export_reverse_fiber

Domain: lowercase SHA-256 polling/export generation identities. Observation: final
reservation generation. Assumptions: immutable export has actual source bindings.
Relation: final generation reconstructs the exported snapshot identity, not the
previous polling identity; possible polls include older generations when writers
advance concurrently. Counterexample: demanding poll=export would reject legitimate
concurrent DB progress. Invalid ../escape poll fails before export/native commands.
Evidence: public entrypoint fixture with distinct valid poll/export digests and
invalid-poll exclusion control. Not a proof of snapshot isolation supplied by DB.

## priority_terminal_identity_fiber

Domain: a selected urgent diagnostic d, exported generation g and existing terminal
receipt r. Observation: public engine calls to backend acknowledgement. Assumptions:
IDs are lowercase SHA-256 and receipt identity is locally trusted. Relation:
ack(d,r) requires r.priority_diagnostic_id=d, in addition to an admitted terminal
outcome and matching generation path. Generation equality alone is insufficient.
Counterexample: an ordinary historical installed receipt for g lacks d and must
not acknowledge newly selected urgent work with the same data generation. The
public guard rejects before calling an arbitrary backend. Evidence: exact finite
historical-receipt control (zero native commands/zero ack), plus successful
below-threshold priority acknowledgement with unchanged model and no gparse call.
The separate conjecture that below-threshold durable receipts lost the ID was
falsified: generation_once already inserted it before receipt persistence. Making
the early return explicit improves the local value contract, not a persisted-bug fix.

## tests

### Finite checks

Fourteen inherited controls cover native command protocol, real file descriptor
inheritance, agreement/data filters, rejection/resource partitions, thresholds,
compatible weight install and no uncertain replay. Added controls exercise full
public Eel/native callpath, distinct poll/export identities, malformed poll-path
exclusion, reviewed-priority precedence, unreviewed-priority exclusion and terminal
ack failure without compute replay. Together with Eel/evidence tests:29 identities.
Synthetic fake-executable checks are not real GPU-learning/generalization evidence.
Two additional focused controls confirm below-threshold priority acknowledgement
without native GPU commands and exclude historical ordinary same-generation
receipts before any backend ack.31 distinct test identities have passed across
this review; the last two are focused checks after the added identity guard.

## History

### 2026-11-04 · tests

2026-11-04 (branch hard-loop): fresh_train_prefixes_default_to_school_and_admit_only_what_is_named: default school:, named prefixes admitted, malformed prefix lists refused; red: a hard: id fails prepare as nonteaching by default and passes when named. 35 engine tests pass.
