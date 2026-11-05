# twin-ingestion

## catalog

# Spec ingestion, cohort and prospective preparation

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

validate then fingerprint/domain; either initialize prior or verify/replay parent; predict with sorted current feature values and stamp time.

## invariant

Filesystem errors may leave a partial ledger. Hashes assume collision resistance and trusted immutable reads; they do not authenticate data and are vulnerable to coordinated rewrites/TOCTOU. Explicit arbitrary commands are not sandboxed. There is no automatic search, teacher or shell interpreter, but caller can explicitly supply a shell executable. report validates stored hashes only when updated.json exists and does not replay the model.

## cost

Let H be bytes hashed and N tree entries, J JSON bytes and d feature count. Prepare/replay costs O(H+sum directory k log k+J+d^3); execution adds command duration and polling, output scans/hashes and repeated fingerprints; save requires serialization, file sync and directory sync. Memory includes model O(d^2), serialized JSON and per-directory maps. No duration guarantee follows from requested timeout.

## composition

Leaf fragment; calls file-level hashing/JSON and external library, and same_model in persistence fragment. Shared serialized field order is part of cohort construction.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See ../model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U is (Spec,parent,filesystem,prepare Result). Source-derived U\R includes successful preparation with mismatched sorted feature/prior keys or a parent with observed accepted=false. Missing parent files can fail before domain or acceptance checks. Existing changed-context and modified-evidence tests witness selected exclusions; proposed single-guard oracle covers remaining predicates.

## reverse-reconstruction

Given Forecast f, compatible inputs include its stored spec and any parent directory whose updated.json hash equals parent_checkpoint, whose evidence hashes/replay pass and whose resulting model equals model_before within implemented comparison. Directory path itself is not stored in parent_checkpoint. No-parent candidates must construct model_before from prior/ridge. Hash agreement constrains content but is not an implemented preimage solver.

## cohort-variation

Hold every domain tuple field and fingerprints fixed; changing feature values or environment values named in axes leaves the computed domain unchanged. Changing fixed env values changes serialized input (hash collision excluded), while changing timeout alone leaves domain unchanged. This precisely models allowed cohort variation, without claiming hardware or workload equivalence. Proposed oracle compares paired valid specs; no live benchmark needed.

## replay-tolerance

Parent replay uses previous.model_before and previous.spec.features rather than trusting previous.prediction. same_model accepts relative/absolute tolerance 1e-12 per zipped scalar and equal n. Candidate relation can include rounded historical values but helper lacks length checks; validation is not provenance authentication. Existing theta12345 mutation is rejected; proposed near-threshold cases remain unexecuted.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/main.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.
