# twin-persistence

## catalog

# CLI dispatch and prospective ledger persistence

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

main branches report/predict/run; run serializes prospective evidence before process execution, saves accepted update afterward, and same_model compares replay fields.

## invariant

Filesystem errors may leave a partial ledger. Hashes assume collision resistance and trusted immutable reads; they do not authenticate data and are vulnerable to coordinated rewrites/TOCTOU. Explicit arbitrary commands are not sandboxed. There is no automatic search, teacher or shell interpreter, but caller can explicitly supply a shell executable. report validates stored hashes only when updated.json exists and does not replay the model.

## cost

Let H be bytes hashed and N tree entries, J JSON bytes and d feature count. Prepare/replay costs O(H+sum directory k log k+J+d^3); execution adds command duration and polling, output scans/hashes and repeated fingerprints; save requires serialization, file sync and directory sync. Memory includes model O(d^2), serialized JSON and per-directory maps. No duration guarantee follows from requested timeout.

## composition

Leaf fragment. Main invokes ingestion and execution and library update; hash/save helpers lie outside this byte range. Sequential composition must preserve the same forecast/model/features across calls.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See ../model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U is command argv/filesystem to finite persisted-file/exit traces. U\R includes a successful run overwriting an existing run directory, learning after o.accepted=false, or predict invoking execute. Errors may follow durable forecast/observation writes, so nonzero exit cannot imply no effects. Existing duplicate-directory and predict marker tests witness this scoped complement.

## reverse-reconstruction

Given a complete ledger checkpoint, candidate prior states are constrained by stored forecast model_before and observe(features,seconds), plus saved hashes/domain. Given only a report row, omitted checkpoint, spec, process and raw artifacts leave many compatible ledgers. Prefix ledgers can correspond to multiple failure stages; absence alone cannot uniquely recover the interruption point. Proposed oracle enumerates interrupted stages with identical visible prefixes.

## report-admission

report accepts any number>=1 of supplied directories, reads f/o, and if updated.json exists verifies only its two evidence hashes before projection. It does not run prepare, require schemas/domains agree or enforce accepted. Thus report success is weaker than parent admission. Proposed oracle constructs a readable rejected observation that reports successfully.

## checkpoint-order

Only after saved observation and accepted gate does main consume f.model_before and observe, compute checkpoint hashes, create_new updated.json and sync directory. Model error can leave accepted observation without checkpoint. Forecast file is synced before execute, but failed writes can leave partial files; prefix persistence is not whole-run atomicity. Existing prospective-chain test checks order and retained forecast bytes.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/main.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.
