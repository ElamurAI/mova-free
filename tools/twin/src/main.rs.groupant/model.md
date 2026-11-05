# mathematical-twin-cli

## catalog

# Mathematical twin CLI

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

Parse command shape and spec; prepare validates, fingerprints and derives cohort, optionally verifies/replays a parent, then predicts. predict prints without spawning. run creates a fresh directory, copies binary, persists forecast before execution, checks local outputs/fingerprints, saves observation and only when accepted applies the library update and saves checkpoint. report projects stored evidence.

## invariant

Filesystem errors may leave a partial ledger. Hashes assume collision resistance and trusted immutable reads; they do not authenticate data and are vulnerable to coordinated rewrites/TOCTOU. Explicit arbitrary commands are not sandboxed. There is no automatic search, teacher or shell interpreter, but caller can explicitly supply a shell executable. report validates stored hashes only when updated.json exists and does not replay the model.

## cost

Let H be bytes hashed and N tree entries, J JSON bytes and d feature count. Prepare/replay costs O(H+sum directory k log k+J+d^3); execution adds command duration and polling, output scans/hashes and repeated fingerprints; save requires serialization, file sync and directory sync. Memory includes model O(d^2), serialized JSON and per-directory maps. No duration guarantee follows from requested timeout.

## composition

Ingestion includes validate/domain/prepare; execution consumes the immutable Forecast and filesystem; persistence owns main dispatch and ledger writes, invoking both in order. same_model is included in persistence scope and called from ingestion. Shared domain, features, parent evidence and model_before link the stages. Reverse composition retains all compatible intermediate ledgers; parent complement is not the union of stage complements. hash/save helpers remain file-level dependencies; avoid counting their calls twice.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks (catalog)

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U is finite CLI traces over argv, trusted filesystem snapshots and process events, observed as exit/result plus created files. R follows forward. Source-derived exclusions include predict launching execute, accepted=false reaching checkpoint creation in that run, and successful create_new overwriting an existing evidence file. Early errors may leave partial directories, so absence of updated.json does not imply no forecast or no execution. Existing ledger tests cover these scoped exclusions.

## reverse-reconstruction

For ledger output L, B(L) contains argv/spec/parent/filesystem/process traces whose projected files equal L. Replaying model_before with recorded feature values and seconds constrains a candidate checkpoint, while timing, stdout producer history and original input bytes are not recoverable from hashes alone. This is a candidate relation under trusted evidence, not actual-cause attribution. Existing parent-chain test validates one path; exhaustive trace inversion remains unimplemented.

## spec-admission

Spec and OutputCheck deny unknown JSON fields. validate requires schema1, nonempty family/context/provenance, absolute executable/cwd and existing cwd, absolute inputs, identical sorted prior/features keys, finite nonzero feature vector, axes named in env, timeout1..86400, outputs nonempty, normal relative components and no exact reserved path. It then calls Model::new. This does not ensure executable existence until fingerprint, input existence, axis uniqueness, statistical feature meaning, or usable inverse matrix at tiny ridge. Proposed oracle varies one predicate at a time.

## cohort-equivalence

At fixed hashing/serialization, domain equals H(schema,family,context,cwd,args,env excluding axes,axes,feature_source,sorted feature keys,prior,ridge,outputs,fingerprint). Feature values, varying axis values, timeout and executable path are omitted; binary bytes and named input hashes participate. Same binary bytes at different paths may share cohort. axes order and duplicates can change domain. Equality is a boundary declaration, not causal exchangeability. Existing changed-context test is one negative witness.

## fingerprint-tree

File hash uses 65536-byte chunks; directory hash is JSON of sorted UTF-8 entry names to recursive hashes, rejecting symlinks and unsupported types. Permissions, timestamps and absolute input paths are absent from content hashes; input map names remain. Directory/file hash formats have no explicit type discriminator. Proposed oracle compares reordered enumeration, renamed members and symlink rejection; no cryptographic uniqueness theorem is supplied.

## parent-replay

prepare with parent requires schema/domain agreement, hashes of forecast/observation, accepted observation, valid checkpoint model, prior forecast domain and observation schema1. It replays previous.model_before.observe(previous.spec.features, seconds), then same_model and hashes updated.json. It does not recursively audit all ancestors or revalidate every historical spec/prediction. Existing tampered theta/evidence checks reject selected mutations; coordinated self-consistent rewrites are outside authenticity assumptions.

## predict-passivity

For accepted argv predict SPEC [PARENT], prepare can read/hash many files and inspect a parent, then print forecast; it creates no run directory and never calls execute. Timestamp and reads remain effects, so repeated printed forecasts need not be byte-identical. Existing prediction_does_not_launch_the_command checks a marker stays absent.

## execution-isolation

execute uses copied executable directly, replaces {run} in args/env values, clears inherited environment then sets PATH/LANG and explicit env overrides, null stdin, separate stdout/stderr and a new process group. On timeout or wait error it SIGKILLs the group and waits, ignoring kill/wait results. This limits ordinary descendants but does not prevent escape, filesystem access or network use. Proposed oracle observes explicit env overrides; no live command or security proof is run.

## acceptance-projection

If execute returns Observation, accepted iff errors.is_empty. Errors include spawn/status/timeout, required output canonical containment/nonempty/hash/contains failures, changed input or source fingerprint, changed copied binary. Some I/O operations propagate Err before any Observation. Exit_code may be None for failures. Acceptance is declared marker/hash evidence, not general correctness. Existing failed/timeout/missing-output/changed-input test covers four rejecting families.

## durability-order

Successful run order: new directory, canonicalize, copy+hash binary, create_new/write/sync forecast, directory sync, execute, create_new/write/sync observation, accepted gate, observe, create_new/write/sync updated, directory sync. A crash between steps may leave only a prefix or partially written file; create_new prevents overwrites but no atomic whole-ledger transaction exists. Existing forecast-precedes test checks process-visible order, not power-loss durability.

## model-comparison

same_model compares n exactly and zipped theta/P/min/max/SSE scalars with abs(a-b)<=1e-12*(1+abs(a)). It does not independently compare iterator lengths, vector dimensions or layout; tolerance is asymmetric and not necessarily transitive. Nearby context validates checkpoint and replay models, and current predict checks feature dimension, but this helper alone is not structural equality. Proposed oracle uses unequal dimensions sharing a zipped prefix; no claim of full adversarial checkpoint validation.

## report-projection

report reads forecast and observation for each given directory in order, optionally checks checkpoint evidence hashes and prints selected family/features/forecast/actual/error/artifact fields. It does not execute, update, require accepted=true, replay checkpoints or validate every schema. Many ledgers map to the same report. Proposed reverse oracle varies omitted prior/cwd fields while preserving printed projection; hashes must be recomputed only in a trusted test fixture.

## checks

### Twin CLI checks

`cargo test -p mova-twin --locked` passed three library tests and three ledger integration tests: prospective forecast/parent chain, failure/timeout/output/input rejection, and prediction without launching its command. Tests run bounded local shell fixtures; no live benchmark, teacher or training was run. The suite includes wrong-cohort, changed evidence, changed model and duplicate-run-directory rejection.

Two initial `mova-modelcheck groupants` reports were byte-identical with no diagnostics. A deliberately stale library-fragment hash was rejected, then restored. Rust independently checked the two file roots, six fragments and forty-eight typed entries, including mandatory type order. These are structural and bounded behavioral checks; proposed catalog oracles are not all implemented.

The branch supplies registry-node.json for parent integration without editing the shared registry. It models filesystem trust, hash/replay limits, partial writes, timeout polling and the weak standalone same_model shape comparison explicitly. No live execution safety, cryptographic authentication, process sandbox or crash-atomic whole-ledger guarantee is asserted.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/main.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

### 2026-10-29 · checks

Executed 2026-10-29 against source bound by manifest.json.
