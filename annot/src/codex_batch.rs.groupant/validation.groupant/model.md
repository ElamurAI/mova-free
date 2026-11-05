# annot-codex-batch-validation

## descriptant

### annot-codex-batch-validation

Parse all JSONL requests and require nonempty unique ids, permitted model and content array before queue effects.

## approximant

### Scoped forward relation

Parse all JSONL requests and require nonempty unique ids, permitted model and content array before queue effects.

Parent composition and symbolic resource costs remain in ../model.md#approximant. No executable percentage surrogate is supplied for this categorical/effect scope.

## invariant

### Scoped fragment: validation

Local scope: Parse all JSONL requests and require nonempty unique ids, permitted model and content array before queue effects.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Invariants and limits

For an unmodified single-worker directory and successful file operations, final publication follows completion of every request; resumed valid successes are not called again. Lock and completion files are trusted filesystem state, not authenticated certificates. Error handling records sticky failure; RAII attempts lock removal and ignores cleanup failure.

- Fake executable only; no live Codex/API behavior evidence.
- Crash durability, PID reuse, hostile directory mutation, duplicate malformed partial cases not exhaustively executed.

## models

# Scoped fragment: validation

Local scope: Parse all JSONL requests and require nonempty unique ids, permitted model and content array before queue effects.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

## behavior_complement

A run encountering sticky error cannot issue a new request. Invalid/duplicate request ids or legacy models are rejected before launch. A changed snapshot or invalid partial envelope prevents normal final publication. Existing final file bypasses all these validations, so stale externally forged final files are outside that implication.

## reverse_reconstruction

A final results file is compatible with fresh execution or a valid prior prefix plus resumed suffix; directory bytes cannot identify which without external logs. Given immutable requests, success rows constrain provider=codex, unique known ids and matching model but do not uniquely reconstruct prompts or actual process behavior.

## failure_precedence

Final sentinel wins over error; error wins over request validation; request errors win over legacy marker and lock checks. Missing lock PID/stale PID produce errors requiring inspection.

## resume_relation

done is built only from succeeded codex partial rows matching requested model and unique id; each skipped ask corresponds to membership in done. Non-text content is checked later per request, so earlier successes can persist.

## publication_boundary

sync_data per partial row precedes rename; rename makes final name visible after all asks. No directory fsync, hostile concurrency or power-loss durability guarantee.

## worker_ownership

Async parent transfers Lock after successful spawn and awaits complete PID publication for that known child or successful completed results. Child writes PID through an exclusive synced temporary file and rename. Only fresh-child startup waits (bounded five seconds); a preexisting lock without pid remains an error. /proc existence does not prove existing process identity. See the root startup model for exact predicates and timeout effects.


## tests

### Scoped fragment: validation

Local scope: Parse all JSONL requests and require nonempty unique ids, permitted model and content array before queue effects.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- failure_requires_explicit_resume_and_never_repeats_successes
- async_worker_publishes_only_complete_results
- legacy_teacher_is_rejected_before_launch
- changed_requests_cannot_reuse_partial_answers
- timeout_is_a_failure_without_partial_publication
- stale_worker_lock_is_reported

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- Fake executable only; no live Codex/API behavior evidence.
- Crash durability, PID reuse, hostile directory mutation, duplicate malformed partial cases not exhaustively executed.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped source-derived model. Byte scope [393,1083); AST is shared partial projection, not a separate semantic proof.
