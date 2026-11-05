# annot-codex-batch-resume

## descriptant

### annot-codex-batch-resume

Snapshot equality and compatible partial results precede skip/resume; each new success syncs before final rename.

## approximant

### Scoped forward relation

Snapshot equality and compatible partial results precede skip/resume; each new success syncs before final rename.

Parent composition and symbolic resource costs remain in ../model.md#approximant. No executable percentage surrogate is supplied for this categorical/effect scope.

## invariant

### Scoped fragment: resume

Local scope: Snapshot equality and compatible partial results precede skip/resume; each new success syncs before final rename.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Invariants and limits

For an unmodified single-worker directory and successful file operations, final publication follows completion of every request; resumed valid successes are not called again. Lock and completion files are trusted filesystem state, not authenticated certificates. Error handling records sticky failure; RAII attempts lock removal and ignores cleanup failure.

- Fake executable only; no live Codex/API behavior evidence.
- Crash durability, PID reuse, hostile directory mutation, duplicate malformed partial cases not exhaustively executed.

## models

# Scoped fragment: resume

Local scope: Snapshot equality and compatible partial results precede skip/resume; each new success syncs before final rename.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

## behavior_complement

A run encountering sticky error cannot issue a new request. Invalid/duplicate request ids or legacy models are rejected before launch. A changed snapshot or invalid partial envelope prevents normal final publication. Existing final file bypasses all these validations, so stale externally forged final files are outside that implication.

## reverse_reconstruction

A final results file is compatible with fresh execution or any accepted ordered subset of prior success rows followed by the missing requests in request order; directory bytes cannot identify that history without external logs. Given immutable requests, success rows constrain provider=codex, unique known ids and matching model but do not uniquely reconstruct prompts or actual process behavior. A request-prefix history is an additional reachability premise for a directory generated solely by this worker from empty state, not a validation requirement on supplied partial files.

## failure_precedence

Final sentinel wins over error; error wins over request validation; request errors win over legacy marker and lock checks. Missing lock PID/stale PID produce errors requiring inspection.

## resume_relation

done is built only from succeeded codex partial rows matching requested model and unique known id; neither request-prefix shape nor request order is checked. Let Q be request ids in order and P accepted partial ids in their existing physical order. On successful completion, parsed result order is P concatenated with Q filtered to ids outside set(P); existing partial bytes are retained, and a missing final LF is repaired by one appended byte before the first new row. Each skipped ask corresponds to membership in done. Non-text content is checked later per request, so earlier successes can persist.

## publication_boundary

sync_data per partial row precedes rename; rename makes final name visible after all asks. No directory fsync, hostile concurrency or power-loss durability guarantee.

## worker_ownership

Async parent transfers Lock after successful spawn and awaits complete PID publication for that known child or successful completed results. Child writes PID through an exclusive synced temporary file and rename. Only fresh-child startup waits (bounded five seconds); a preexisting lock without pid remains an error. /proc existence does not prove existing process identity. See the root startup model for exact predicates and timeout effects.


## separator_state

Let B be the partial file bytes and tail(B) the segment after the last LF (all of B when it has none). `str::lines()` yields that tail as a final line, so a complete last row is accepted whether or not a terminator follows. Define sep(B) = tail(B).trim() is nonempty, that is, a real record lacks its terminator under the worker's own nonblank filter. After every row validates, and before any ask, the worker appends exactly one LF iff sep(B), then sync_data. Absent or empty files write nothing. A whitespace-only tail (`"   "`, `"\t"`, `"\r"`, or `row LF "  "`) is left untouched: the next row is appended after it, as before, so strict line-by-line readers still see only JSON rows and no blank physical line is created. A bare CR after a complete row (`row CR`) is trailing JSON whitespace: the row is accepted, sep is true, the appended LF makes a CRLF row; CR files are not rejected. Trailing spaces after a row without LF likewise get a LF. An invalid or truncated last row fails validation first, so no normalizing byte is written. Unicode-only whitespace tails (for example U+00A0) are blank to `trim()` but not JSON whitespace: they were and remain unsupported by strict JSON consumers, a preexisting boundary this change neither creates nor repairs. Baseline witness on 02f3d093: Q=[a,b,c], P=[b] without LF produced one physical line holding two objects (`{b}{a}`), two lines instead of three, unparsable at column 265.

## append_composition

final = B ++ (LF if sep(B)) ++ concat(row(r) ++ LF for remaining r in request order). rows(.) denotes the nonblank, trim-filtered parsed-row projection used by the worker and by annot parse_results, not a physical line count. Hence rows(final) = rows(B) ++ remaining and the original bytes are a byte prefix of the published file. A whitespace-only tail of B becomes leading whitespace of the first appended row. The file ends with LF whenever at least one row is appended or sep(B) held; an all-done file whose tail is blank keeps its old tail, so no blanket final-LF claim is made for every accepted file. Executed shapes: [b] with LF, without LF, with trailing spaces, with a bare CR, followed by LF plus spaces or CR; empty; whitespace-only space/tab/CR; all-done without LF (zero asks, final ends with LF).

## boundary_failure_fibers

Scope: the partial file codex-results.partial.jsonl only. Invalid row (truncated, unknown id, wrong model, duplicate) → error before any write to the partial file and before any ask; its bytes are unchanged. run/execute may already have written batch-id.txt, the codex-requests.jsonl snapshot, the lock directory with pid, and codex-error.txt after the failure; those effects are modeled by failure_precedence and worker_ownership. Valid rows and a failing ask k → partial = B ++ (LF if sep(B)) ++ rows of successes before k, sticky error recorded; a later explicit resume parses cleanly and never repeats rows(B), where rows(.) is the nonblank trim-filtered parsed-row projection, not the physical line count. A bare CR after a complete last row is accepted (JSON whitespace) and separated; the worker does not reject CR-bearing files. Partial files that already contain blank physical lines are accepted by the tolerant worker as before; strict compatibility is not claimed for them. Executed: prior [a] without LF then FAIL → one ask, partial equals prior ++ LF exactly, then resume to three rows with one more ask each for b and c.

## tests

### Scoped fragment: resume

Local scope: Snapshot equality and compatible partial results precede skip/resume; each new success syncs before final rename.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- failure_requires_explicit_resume_and_never_repeats_successes
- async_worker_publishes_only_complete_results
- legacy_teacher_is_rejected_before_launch
- changed_requests_cannot_reuse_partial_answers
- timeout_is_a_failure_without_partial_publication
- stale_worker_lock_is_reported
- partial_without_final_lf_keeps_record_separation_on_resume (wave38)
- partial_boundary_shapes_and_failed_next_request (wave38)
- strict_line_consumers_see_only_json_rows_after_resume (wave38 follow-up)

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- Fake executable only; no live Codex/API behavior evidence.
- Crash durability, PID reuse, hostile directory mutation, duplicate malformed partial cases not exhaustively executed.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped source-derived model. Byte scope [3688,6649); AST is shared partial projection, not a separate semantic proof.
