# twin-execution

## catalog

# Process execution and evidence acceptance

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

Spawn copied executable with explicit environment/process group; collect status and duration, check outputs/fingerprints, derive errors and acceptance.

## invariant

Filesystem errors may leave a partial ledger. Hashes assume collision resistance and trusted immutable reads; they do not authenticate data and are vulnerable to coordinated rewrites/TOCTOU. Explicit arbitrary commands are not sandboxed. There is no automatic search, teacher or shell interpreter, but caller can explicitly supply a shell executable. report validates stored hashes only when updated.json exists and does not replay the model.

## cost

Let H be bytes hashed and N tree entries, J JSON bytes and d feature count. Prepare/replay costs O(H+sum directory k log k+J+d^3); execution adds command duration and polling, output scans/hashes and repeated fingerprints; save requires serialization, file sync and directory sync. Memory includes model O(d^2), serialized JSON and per-directory maps. No duration guarantee follows from requested timeout.

## composition

Leaf fragment. Main persists returned observation and gates learning; execute alone never updates Model. Filesystem/process trust is an explicit dependency.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See ../model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U contains process/filesystem event traces mapped to returned Observation or Err. U\R includes returned accepted=true while accumulated errors are nonempty, including observed timeout, nonzero exit or output marker mismatch. Some post-run hash/open failures return Err and produce no Observation at all. Existing timeout/missing-output tests distinguish failed observations from updates.

## reverse-reconstruction

Given Observation with accepted=true, compatible traces have successful process completion, all required output checks and stable fingerprints, with durations projecting to seconds. Different executions and elapsed-time schedules may have identical summary fields. rejected observations constrain only reported errors, not every hidden event; an error list is not a full process trace. Proposed oracle groups mocked event traces; no solver is implemented.

## timeout-state

Poll try_wait first; successful exit is accepted even if elapsed has just crossed timeout. Only Ok(None) with elapsed>=timeout triggers kill path; earlier polls sleep5ms. Therefore timeout is a polling condition, not a hard completion upper bound. Scheduler, wait and output hashing may extend runtime. Existing sleeping fixture confirms rejection, not a latency bound.

## artifact-boundary

Required output must canonicalize under canonical run dir, be nonempty regular file, match optional hash and contain each marker in UTF-8 when markers requested. A symlink resolving inside is allowed; containment check plus later reads is not atomic against races. stdout/stderr hashes are collected even without output checks. Proposed oracle uses internal/external symlinks and binary contents to test this boundary.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/main.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.
