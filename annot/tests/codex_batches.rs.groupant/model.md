# annot-tests-codex-batches

## descriptant

### annot-tests-codex-batches

Unix integration fixture drives the actual annot executable with a generated fake Codex process and temporary queue files. Assertions cover local orchestration only; fake teacher text does not certify annotation quality or live API behavior.

## approximant

### Forward approximant

Fixture::new creates a fresh process/time-named tempdir, writes requests and executable script. run supplies executable path, call-log path, fixture API env values, one-second timeout and sync/async flag to annot batch-run. Tests inspect status, calls, final sentinel and stderr; Drop removes fixture directory.

Cost: Small request count (<=2), one long timeout prompt of120000 bytes; six tests launch bounded subprocesses, timeout1s and async poll<=5s, with a fake hang30s killed by teacher timeout. Runtime depends on scheduling and process cleanup; no throughput inference.

Executable approximation gap: categorical parsing, effects and validation lack a useful measured percent metric here. No executable surrogate or 1/2/5/10% profile is attached; an exact test is not an approximant.

## invariant

### Invariants and limits

Each run uses MOVA_CODEX_BIN pointing to the synthetic script; script requires both API variables stripped before execution. No test intentionally contacts live Codex/API. Successful requests return sample OK and fixed token counts, not real model inference. Tempdir cleanup is best effort.

- Unix-only, finite scheduling deadlines, no live teacher or cloud behavior.
- No source mutation control directly targeting fixture implementation; controls target actual queue/production contracts.

## composition

### Source composition

Nested scopes use UTF-8 byte offsets [start,end) against the exact source SHA-256. They share the parent lexical projection; no nested fragment becomes a second registry source. Source outside these scopes, including helper types/tests, remains parent-modeled.

- fixture [173,2615): Temporary isolated fixture controls executable, environment, failure mode and call accounting.
- assertions [2615,5595): Six assertions exercise resume/publication/model/snapshot/timeout/lock paths of actual annot with fake process transport.

Fixture::new creates a fresh process/time-named tempdir, writes requests and executable script. run supplies executable path, call-log path, fixture API env values, one-second timeout and sync/async flag to annot batch-run. Tests inspect status, calls, final sentinel and stderr; Drop removes fixture directory.

Cross-file join: ann.rs supplies Ann/Item, parse/check/write; main supplies Outcome and finish; batch and llm prompts feed codex_batch; hints and repair influence draft content before gates. These joins do not establish remote correctness or crash durability.

## models

## behavior_complement

The fixture rejects a child inheriting API variables; failing/timeout runs cannot satisfy final-publication assertions. Passing fake tests excludes those observed local wiring failures only, not external transport or adversarial crash cases.

## reverse_reconstruction

Observed call count3 after failure/resume is compatible with two initial attempts plus one resumed request under recorded script state; it does not recover remote answers. Identical success JSON can come from many prompt strings because fake output ignores most input except FAIL selector.

## environment_isolation

Command-scoped env overrides avoid global process environment races. Fake shell tests key absence; supplied values are fixture-only.

## failure_resume_witness

First success plus FAIL produces no final file and two calls; sticky retry leaves calls unchanged; explicit error removal permits only remaining call, final repeated run adds none.

## publication_observer

Async test polls final existence at20ms intervals for<=5s then checks two lines/calls. This observes completion, not every intermediate filesystem state.

## timeout_and_lock

Long prompt tests writer/timeout behavior within5s. Stale lock PID4294967295 tests no-launch rejection; neither assertion proves all PID races safe.


## separator_witness

Supplied partial rows are written by success_row (worker envelope, no terminator) plus an explicit tail. Witness Q=[a,b,c], P=[b]: the final file must start with the exact prior bytes, end with LF, parse row by row and list ids [1,0,2] with two calls. Boundary shapes: empty and whitespace-only files add no record; all-done without LF publishes with zero calls and a terminator; a truncated last row is refused with "incomplete", zero calls and unchanged bytes; FAIL after a repaired boundary leaves exactly prior ++ LF and resumes without repeating a. The ids helper fails on any unparsable row, so a merged line is detected as an error, not as a lower line count.

## strict_consumer_witness

A strict reader parses every physical line with serde_json and no trim or blank filter. Priors: whitespace-only space, tab, CR; a terminated row followed by spaces or CR; a complete row followed by a bare CR. Each must publish prior bytes as prefix, end with LF, keep ask counts, and give ids [0,1,2] or [1,0,2] under the strict reader. Not claimed: files that already hold blank physical lines (accepted by the tolerant worker only), Unicode-only whitespace tails.

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
- Unix-only, finite scheduling deadlines, no live teacher or cloud behavior.
- No source mutation control directly targeting fixture implementation; controls target actual queue/production contracts.

Validation limits: strict scoped Clippy exited101 on six pre-existing lints (three collapsible_if, one needless_range_loop, one type_complexity, one redundant_locals). Re-running with those four known lint categories allowed exited0. changed with an external augmented registry exited0 and found no missing changed-source models. Local groupants inspection reports only expected ancestor child/source-binding updates owned by integration; no owned leaf/fragment diagnostics.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Prospective notes and numeric receipts retained externally under the annotation wave27 run. Four new inline tests supplement existing bounded tests. Preserve distinctions: compatibility cost is not subscription cost; mock process tests do not establish live teacher behavior; registry presence is partial modeling.

Epoch 2026-10-31 (wave38): two tests add success_row/ids helpers and supplied-partial witnesses (no LF, trailing spaces, LF, empty, whitespace-only, all-done, truncated, failing next request). The assertions fragment scope extends to the end of file.

### 2026-10-31 · tests

#### Wave38 separator boundary (2026-10-31)

Baseline red on 02f3d093 with the two new tests (debug, exit 101): resume after a supplied partial whose last valid row lacks LF glued the next JSON object onto that row (unparsable final line, trailing characters at column 265) and an all-done partial without LF was published without a terminator. After the one-byte separator repair: eight codex_batches tests plus eleven annot bin tests, debug twice and release twice, exit 0, sorted named outcomes identical. Command: cargo test --offline --locked -j2 -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. Fake executable only; no live Codex, Claude, API, training or corpus. Logs: external octopus-wave38-fable-2026-10-31 (baseline-red-debug.log, fixed-{debug,release}-{1,2}.log, prospective-model.md written before the change).

Follow-up (same day): the first predicate (last byte ≠ LF) turned whitespace-only tails into a blank physical line, which strict no-trim downstream readers reject (Opus cross-review). New test strict_line_consumers_see_only_json_rows_after_resume parses every physical line with serde_json and no filter: red on 46e3ee9 (exit 101, "EOF while parsing a value" on "   "), green after the tail predicate sep(B) = tail(B).trim() nonempty. Nine codex_batches tests plus eleven bin tests, debug once and release once, exit 0, identical named outcomes (followup-debug.log, followup-release.log, strict-baseline-red-46e3ee9.log).
