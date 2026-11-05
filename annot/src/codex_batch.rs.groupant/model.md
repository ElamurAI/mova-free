# annot-codex-batch

## descriptant

### annot-codex-batch

Restartable local queue with sticky error, worker lock, immutable request snapshot, append-only partial successes and final rename. Live transport is delegated to mova_teacher; bounded tests replace its executable.

## approximant

### Forward approximant

run canonicalizes, returns immediately for existing final result, rejects sticky error, validates all requests and pending legacy marker, then acquires lock. Sync calls finish; async spawns current executable and transfers lock ownership. execute checks snapshot equality, validates partial lines, appends one LF if the segment after the last LF is a non-blank (real) record lacking its terminator, skips done ids, asks for each remaining request, appends and syncs success, then renames partial to final.

Cost: R requests, P partial results, B bytes: parse/set work O(B+R log R+P log P), partial request lookup O(P*R); storage O(B+R+P). Each remaining request invokes one external process through teacher and one sync_data. Wall time/network/model cost unspecified. No resource cap or crash durability theorem.

Executable approximation gap: categorical parsing, effects and validation lack a useful measured percent metric here. No executable surrogate or 1/2/5/10% profile is attached; an exact test is not an approximant.

Fresh async launch now transfers ownership after spawn and waits up to five seconds for the exact known child's complete PID or successful exited-child results. PID publication writes and syncs an exclusive temporary file before rename. Existing locks do not receive this startup wait. See [startup observations and failure partitions](model.md#startup); no request retry or throughput claim follows.

## invariant

### Invariants and limits

For an unmodified single-worker directory and successful file operations, final publication follows completion of every request; resumed valid successes are not called again. Lock and completion files are trusted filesystem state, not authenticated certificates. Error handling records sticky failure; RAII attempts lock removal and ignores cleanup failure.

- Fake executable only; no live Codex/API behavior evidence.
- Crash durability, PID reuse, hostile directory mutation, duplicate malformed partial cases not exhaustively executed.
- Separator repair covers a real last record lacking LF; whitespace-only tails are left in place; mid-row corruption is refused, not repaired; bare CR after a row is accepted as JSON whitespace.

For a successful fresh asynchronous startup under cooperative filesystem premises, a complete matching child PID or a successful child exit plus final results was observed before return. A startup timeout preserves child ownership and uncertainty; it does not imply cancellation or permit replay. Existing final-result shortcuts and unverified live-PID identity remain separate compatibility boundaries.

## composition

### Source composition

Nested scopes use UTF-8 byte offsets [start,end) against the exact source SHA-256. They share the parent lexical projection; no nested fragment becomes a second registry source. Source outside these scopes, including helper types/tests, remains parent-modeled.

- validation [393,1083): Parse all JSONL requests and require nonempty unique ids, permitted model and content array before queue effects.
- ownership [1186,3688): run/worker/finish compose completion and error sentinels with lock ownership, worker spawn and sticky error recording.
- resume [3688,6649): Snapshot equality and validated partial rows precede a one-byte separator repair when the segment after the last LF holds a non-whitespace (real) record; skip/resume follows, each new success syncs before final rename.

run canonicalizes, returns immediately for existing final result, rejects sticky error, validates all requests and pending legacy marker, then acquires lock. Sync calls finish; async spawns current executable and transfers lock ownership. execute checks snapshot equality, validates partial lines, skips done ids, asks for each remaining request, appends and syncs success, then renames partial to final.

Cross-file join: ann.rs supplies Ann/Item, parse/check/write; main supplies Outcome and finish; batch and llm prompts feed codex_batch; hints and repair influence draft content before gates. These joins do not establish remote correctness or crash durability.

## models

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
## ordered_subset_preimage

Domain: an immutable request sequence Q with unique ids, a partial file whose parsed nonblank row sequence P passes the source's provider/status/id/model checks, no preexisting final-result shortcut, and one worker completing all remaining asks and file operations successfully. The existing parsed row order P is arbitrary; known unique ids constrain it to an ordered subset of Q. The worker establishes a missing separator after a real final record; rows here use the nonblank parsed-row projection, with stricter physical-line consumer premises modeled separately. With ids(P) denoting its set, final parsed order is S=P ++ filter(Q,id not in ids(P)).

For fixed Q and observed S, compatible prior states retain every accepted P satisfying that equation, plus compatible row bodies and filesystem state. Q=[a,b,c], P=[b] gives S=[b,a,c]; P=[b,a] and P=[b,a,c] can give the same S. No unique history follows from a final file. If P was produced solely by this worker from an empty directory, without external edits, its successful calls form a request prefix; that narrower history yields request order after completion.

Assumptions include stable directory/request bytes, valid partial envelopes and successful remaining effects; a missing final LF is no longer a premise because the source repairs it before appending. This is an order/preimage model, not authentication, teacher-quality or crash-durability evidence. The arbitrary-subset witness Q=[a,b,c], P=[b] is executed by partial_without_final_lf_keeps_record_separation_on_resume (final ids [1,0,2]).

## separator_state

Let B be the partial file bytes and tail(B) the segment after the last LF (all of B when it has none). `str::lines()` yields that tail as a final line, so a complete last row is accepted whether or not a terminator follows. Define sep(B) = tail(B).trim() is nonempty, that is, a real record lacks its terminator under the worker's own nonblank filter. After every row validates, and before any ask, the worker appends exactly one LF iff sep(B), then sync_data. Absent or empty files write nothing. A whitespace-only tail (`"   "`, `"\t"`, `"\r"`, or `row LF "  "`) is left untouched: the next row is appended after it, as before, so strict line-by-line readers still see only JSON rows and no blank physical line is created. A bare CR after a complete row (`row CR`) is trailing JSON whitespace: the row is accepted, sep is true, the appended LF makes a CRLF row; CR files are not rejected. Trailing spaces after a row without LF likewise get a LF. An invalid or truncated last row fails validation first, so no normalizing byte is written. Unicode-only whitespace tails (for example U+00A0) are blank to `trim()` but not JSON whitespace: they were and remain unsupported by strict JSON consumers, a preexisting boundary this change neither creates nor repairs. Baseline witness on 02f3d093: Q=[a,b,c], P=[b] without LF produced one physical line holding two objects (`{b}{a}`), two lines instead of three, unparsable at column 265.

## append_composition

final = B ++ (LF if sep(B)) ++ concat(row(r) ++ LF for remaining r in request order). rows(.) denotes the nonblank, trim-filtered parsed-row projection used by the worker and by annot parse_results, not a physical line count. Hence rows(final) = rows(B) ++ remaining and the original bytes are a byte prefix of the published file. A whitespace-only tail of B becomes leading whitespace of the first appended row. The file ends with LF whenever at least one row is appended or sep(B) held; an all-done file whose tail is blank keeps its old tail, so no blanket final-LF claim is made for every accepted file. Executed shapes: [b] with LF, without LF, with trailing spaces, with a bare CR, followed by LF plus spaces or CR; empty; whitespace-only space/tab/CR; all-done without LF (zero asks, final ends with LF).

## boundary_failure_fibers

Scope: the partial file codex-results.partial.jsonl only. Invalid row (truncated, unknown id, wrong model, duplicate) → error before any write to the partial file and before any ask; its bytes are unchanged. run/execute may already have written batch-id.txt, the codex-requests.jsonl snapshot, the lock directory with pid, and codex-error.txt after the failure; those effects are modeled by failure_precedence and worker_ownership. Valid rows and a failing ask k → partial = B ++ (LF if sep(B)) ++ rows of successes before k, sticky error recorded; a later explicit resume parses cleanly and never repeats rows(B), where rows(.) is the nonblank trim-filtered parsed-row projection, not the physical line count. A bare CR after a complete last row is accepted (JSON whitespace) and separated; the worker does not reject CR-bearing files. Partial files that already contain blank physical lines are accepted by the tolerant worker as before; strict compatibility is not claimed for them. Executed: prior [a] without LF then FAIL → one ask, partial equals prior ++ LF exactly, then resume to three rows with one more ask each for b and c.

## tests

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

Validation limits: strict scoped Clippy exited101 on six pre-existing lints (three collapsible_if, one needless_range_loop, one type_complexity, one redundant_locals). Re-running with those four known lint categories allowed exited0. changed with an external augmented registry exited0 and found no missing changed-source models. Local groupants inspection reports only expected ancestor child/source-binding updates owned by integration; no owned leaf/fragment diagnostics.

## startup

# Worker PID publication and startup

## Forward

For a fresh asynchronous launch, run owns the new directory lock until spawn succeeds, then transfers ownership to the child before waiting. The child's PID is written and synced to an exclusively created temporary file, then renamed to pid. A successful async return observes either that exact Child::id or an already exited successful child with published results. Only the known newly spawned child receives a bounded five-second startup handshake. No request is retried by this handshake.

A preexisting directory follows the original fail-closed PID reader. Missing, empty, malformed and absent-process PIDs fail; numeric live-PID existence is not authenticated process identity. Existing final-result and sticky-error precedence is unchanged.

## atomic_pid_visibility

Domain: a fresh owned lock, cooperative single child and successful ordinary filesystem operations. Observation: externally visible pid bytes. Let p be the child's decimal PID and L the lock state. Publication transitions from L with no pid to L with pid=p through a fully written temporary file and one rename. Exclude a newly published empty or truncated PID. This is a partial behavior exclusion under the filesystem premises, not a power-loss or hostile-concurrency theorem. An unexpected temporary file is not overwritten; its existence fails publication.

Evidence: native complete-publication and unexpected-temporary-state controls. The live failure was an empty PID parse while a detached result subsequently completed. Deterministic source-wrapper control separately reproduces early async return by delaying the child startup.

## known_child_readiness

Domain: the Child handle returned by this invocation's successful spawn, with child PID p, no hostile directory mutation. An accepted live-start observation has pid=p; it cannot be reconstructed as a different known Child::id. A successful exited-child observation additionally requires final results. The observation does not recover teacher output, execution duration or prove PID-reuse immunity. Observing a final file through the preexisting fast path remains a different, trusted-filesystem case.

Evidence: delayed known-child admission, wrong/empty/malformed PID rejection, successful exit with results, failed exit and missing-result controls.

## startup_failure_partition

Domain: fresh async startup only. Missing pid while the known child is still alive permits bounded startup waiting. Wrong or malformed pid, unreadable state and unsuccessful child exit fail. Expiry retains child ownership, lock and outputs; it never kills the child or resubmits a request. Before spawn succeeds, RAII retains its original cleanup role. A timeout does not establish whether a later teacher call or publication occurred, so recovery must inspect durable results.

Counterexample to a generic retry: re-invoking a new directory after the polling error can duplicate a teacher call that already completed in the old directory. Missing or malformed preexisting lock files are not silently retried or removed.

Evidence: timeout leaves the known child alive and lock unchanged; existing missing/malformed/stale lock controls retain bytes and issue no requests. Native tests are local/fake only, not live teacher or full-pipeline evidence.

## Cost and limits

At most five seconds of startup waiting, checked in five-millisecond intervals, is added only to a fresh async launch. This is a configured wall-clock deadline, not a scheduler-latency guarantee; expensive teacher waiting is outside it. PID publication adds one small exclusive write, sync and rename. No throughput gain, numerical approximation percentage, crash durability, hostile-filesystem safety or authenticated existing-worker identity is claimed.

## Whole-worker publication-error observation

If publish_pid itself fails inside finish, the function returns before execute and before writing codex-error.txt. The worker's ordinary Lock drop can then remove its lock directory, including a conflicting temporary file. The helper-level create_new control establishes non-overwrite by publish_pid itself; it is not a whole-worker artifact-preservation guarantee. The parent observes the unsuccessful child exit and retains the worker log. This failure differs from handshake timeout, where the still-live child's ownership is deliberately retained. Current CLI callers exit after startup; process reaping and authenticated start-time identity for general long-lived callers remain outside this repair.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Prospective notes and numeric receipts retained externally under the annotation wave27 run. Four new inline tests supplement existing bounded tests. Preserve distinctions: compatibility cost is not subscription cost; mock process tests do not establish live teacher behavior; registry presence is partial modeling.

#### Source review 2026-10-31

The accepted-state model now distinguishes arbitrary ordered partial subsets from the narrower request-prefix histories generated by this worker alone. The ordered-subset preimage retains history ambiguity. This documentation change adds source reasoning only; existing mock-process receipts remain unchanged and no new teacher execution is claimed.

#### Separator boundary 2026-10-31 (wave38)

Confirmed with the fake-teacher harness: a supplied partial file whose last valid row lacked LF had the next success appended onto the same line. The source now appends exactly one LF after all rows validate and before any ask, retaining the original bytes; invalid rows still stop before any write. Typed models separator_state, append_composition and boundary_failure_fibers record the exact bytes, order and failure fibers; ordered_subset_preimage drops record separation from its premises.

Follow-up 2026-10-31: sep(B) now tests the segment after the last LF for non-whitespace, so whitespace-only tails stay glued to the next row (no blank physical line for strict readers) while a real unterminated record still gets its LF. Unicode-only whitespace tails remain a preexisting strict-JSON boundary.

### 2026-10-31 · tests

#### Wave38 separator boundary (2026-10-31)

Baseline red on 02f3d093 with the two new tests (debug, exit 101): resume after a supplied partial whose last valid row lacks LF glued the next JSON object onto that row (unparsable final line, trailing characters at column 265) and an all-done partial without LF was published without a terminator. After the one-byte separator repair: eight codex_batches tests plus eleven annot bin tests, debug twice and release twice, exit 0, sorted named outcomes identical. Command: cargo test --offline --locked -j2 -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. Fake executable only; no live Codex, Claude, API, training or corpus. Logs: external octopus-wave38-fable-2026-10-31 (baseline-red-debug.log, fixed-{debug,release}-{1,2}.log, prospective-model.md written before the change).

Follow-up (same day): the first predicate (last byte ≠ LF) turned whitespace-only tails into a blank physical line, which strict no-trim downstream readers reject (Opus cross-review). New test strict_line_consumers_see_only_json_rows_after_resume parses every physical line with serde_json and no filter: red on 46e3ee9 (exit 101, "EOF while parsing a value" on "   "), green after the tail predicate sep(B) = tail(B).trim() nonempty. Nine codex_batches tests plus eleven bin tests, debug once and release once, exit 0, identical named outcomes (followup-debug.log, followup-release.log, strict-baseline-red-46e3ee9.log).

### 2026-11-01 · descriptant

#### Paired diagnostic memory — 2026-11-01

The forward resume vidtvorant must retain both the parsed-row observation and the byte-framing effect. Append-composition and strict-consumer exclusions exposed the unterminated-record defect and then the blank-line defect in the first repair. Keeping only the tolerant reader projection would hide the second failure. The ordered-subset reconstruction is a separate model correction: accepted prior rows need not be a request prefix, and final rows do not identify a unique prior history. All these claims retain their stated domains; no teacher-quality, crash-durability or new runtime evidence is added here. See the [paired-review account](../../../docs/mathematical-modeling.md).

#### PID publication and known-child startup — 2026-11-01

A live audit found an annotation controller paused after polling an empty PID file, although its detached teacher batch later completed. The forward ownership model omitted visibility during PID write and the interval between spawn and child initialization. The repair atomically publishes a synced complete PID and waits at most five seconds only for the just-spawned known child or its completed result. The timeout/failure obernant preserves uncertain work; arbitrary preexisting missing/malformed/stale lock state remains fail-closed. Exact child equality during fresh startup does not authenticate a preexisting live PID or eliminate PID reuse.

The deterministic delayed-child wrapper imports the actual module: baseline returns before readiness (red), repaired source waits (green), in debug and release. Six new local controls plus nine existing fake-process integrations pass each profile. This is not a live teacher or full annotation application test. The [startup model](model.md#startup) separates visibility, compatible known-child observations, timeout effects and residual limits. Original runtime reservation and completed response were retained; no call was replayed by this change.

### 2026-11-01 · tests

#### PID startup regression — 2026-11-01

An external source-wrapper delays only its own codex-worker branch by200ms and submits an empty request batch. Original source67e8ff51 returns before complete PID/results (exit1); repaired source04e68151 waits and passes, in debug and release. This is a deterministic caller/worker ordering control, not timing evidence about Codex. Both actual module imports and baseline snapshot hashes are retained externally.

Six added local tests cover complete atomic PID publication and unexpected temporary state; delayed known-child readiness; wrong/empty/malformed fresh PID refusal; timeout with child/lock retained; success/error exit with required results; and existing missing/malformed/stale locks failing before request mutation. The nine existing codex_batches fake-process integration tests also pass. Total15 distinct checks per profile (30 executions), offline/locked and max2build jobs. The harness imports actual codex_batch.rs and actual existing integration source, with a minimal argument wrapper; it does not exercise the rest of annot's main application. No model or teacher call, GPU, training or corpus processing occurred.

Evidence: $MOVA_BIG/runs/annotation-pid-publication-2026-11-01/{baseline-red.log,baseline-red-release.log,fixed-green.log,fixed-green-release.log,focused-debug.log,focused-release.log}. Syntax projection generated twice identically. Source/ancestor bindings are updated separately from runtime activation.

The actual full annot release binary was also built offline/locked (two jobs). Its six startup unit checks and all nine existing fake-process codex_batches integration checks passed:15 additional native release executions. This closes the minimal-wrapper versus real binary concern without running unrelated annot tests. Logs: native-annot-build.log, native-startup-release.log, native-integration-release.log in the same external evidence root.
