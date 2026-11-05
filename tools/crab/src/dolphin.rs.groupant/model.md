# peta-dolphin

## descriptant

### Dolphin audit adapter epoch

Main annotations remain read-only; proposals and raw critiques live in a separate review-only SQLite queue. Reservations are committed before returning work, are unique by sentence ID and survive restart without automatic replay. Quarantines spend quota conservatively. Valid reports must bind archived and current selected revisions; completed dev or train reviews never automatically become training data.

Eight focused tests and seven reused pragmatic-schema tests provide finite evidence, including unchanged source DB bytes. Source syntax and relational models are partial. Caller priority scheduling, actual consultation, semantic adjudication and later deliberate admission require separate evidence.

Integrated validation exposed a real bootstrap race: simultaneous WAL PRAGMAs/schema setup could return SQLITE_BUSY despite the connection busy timeout. The former 15-test local pass did not establish concurrency stability. A separate bounded OS lease now serializes bootstrap, WAL transition runs only when the mode differs, and fresh schema/source identity initialize in one transaction. Existing queues no longer run redundant DDL/meta inserts. The quota transaction and concurrent test remain unchanged. Ten consecutive focused concurrency runs and all 16 module/schema tests in debug/release passed; a held-lease negative control verifies timeout without reservation and recovery after release. This is finite evidence, not arbitrary-lock or power-loss proof.

## model

# Dolphin audit queue models

Partial source-derived models of this bounded adapter, not formal proofs or annotation-quality guarantees. Domain assumes one persistent run root and its source DB identity, trusted upstream eligible selections, ordinary SQLite/storage behavior and cooperating callers. No teacher or terminal is invoked here; caller priority/idle checks remain external.

## forward

With dolphin_auto=true, next reads an eligible teacher selection from a read-only DB snapshot. Let N be eligible selected sentence IDs, p=floor(1000000*configured_fraction) computed conservatively from decimal representation, and Q=min(floor(N*p/1000000),floor(N/100)). Default fraction is0.005 and fractions above0.01 refuse. Under an immediate audit-DB transaction, lifetime request count A (including quarantines) must satisfy A<Q and no pending reservation may exist. Deterministic hash/ID ordering selects one unseen ID whose full source hashes, pragmatic row and target labels match and whose basic integer tree has at most96tokens. Prompt UTF-8 bytes must be at most12000. Unique sentence-ID reservation is committed before returning the task.

finish validates echoed request/base hashes, archived source content, the current exact selected key/hashes, proposed ID/text/token identity, bounded basic UD tree/targets and pragmatic categories. Valid proposed revisions become completed review-only records in the separate audit DB. Invalid/non-JSON reports become terminal quarantines retaining raw critique. Identical terminal report is idempotent; replacement refuses. Neither outcome writes the main annotation DB or grants training eligibility.

## composition

Crab caller foreground/idle arbitration -> next reservation -> caller's existing authorized low-priority terminal consultation -> finish -> separate independent-review queue. The module supplies no consultation, scheduling, model invocation or training. status exposes pending metadata and explicit no-replay instructions, not a task-replay API. Upstream selection eligibility and future deliberate review admission are outside this adapter.

## invariant

For every successful reservation, A_after<=Q for the source snapshot used at that reservation. All states consume lifetime quota, and each sentence ID appears at most once per persistent queue. With monotone append-only source selection and unchanged fraction the current bound follows; source shrink or fraction reduction can reveal an existing breach and blocks more reservations. Recreating/deleting a queue or using another run root is outside this lifetime model. Prompt/task bounds count UTF-8 bytes and integer tokens respectively. Main source SQL uses READ_ONLY and query_only. Completed revisions remain review_only=true,training_allowed=false.

## cost

One eligible-selection scan, deterministic O(N log N) hash sort and bounded candidate loads per next call; no probabilistic sampling assumption. SQLite counts/reservation serialize in one immediate transaction. Reports are capped at128000serialized bytes; source/proposed tree checks are bounded by96tokens, with simple O(tokens squared) ancestor checks. Queue/status costs grow with lifetime audits, at most the configured fraction under domain assumptions. No measured end-to-end speedup claim.

## behavior_complement

Model an inner exclusion of successful reservation transitions with A_after>Q, repeated sentence ID, unfinished predecessor, too-long prompt/tree or disabled automatic audit. They cannot arise under the declared transaction/storage premises. Quarantine does not free budget. Counterexample to an expected hash percentage as a hard cap: small finite N can have arbitrary sample counts; this implementation uses an explicit integer floor, including zero at N<100 for1percent. Evidence: small-N/default, concurrent, restart/quarantine and byte/token negative controls. This is a source-derived relation with finite evidence, not a general crash proof.

## reverse_reconstruction

A request archives its exact source key, tree/prag hashes, full source pair and provenance. A completed review constrains compatible validated proposals and that archived base; it cannot reconstruct the helper's hidden reasoning or prove linguistic truth. Distinct reports can propose equal AST/prag revisions; the report hash preserves that distinction. Counterexample to recovering dispatch from reservation: the caller can crash after reservation commit and before any terminal call. Evidence: restart/no-replay and source-revision binding controls. No unique inverse of an external consultation is asserted.

## quota_budget_preimage

Type: resource_budget_converse. Holding source snapshot/fraction fixed, A existing distinct reserved IDs leave at most max(0,Q-A) additional lifetime slots. A pending row restricts the immediate next-call image to None even if slots remain. Completed and quarantined rows both stay in A. Counterexample to reclaiming failed attempts: a plain critique quarantines but the next call at quota remains None. Evidence: finite-floor and quarantine tests. The numeric percent is an audit policy cap, not an approximation-error bound.

## uncertain_dispatch_exclusion

Type: crash_state_preimage. A reserved row admits at least two external histories: never dispatched, or dispatched with lost response. Since these are indistinguishable here, next never returns that task again; status requires explicit log/operator reconciliation and finish. This excludes automatic replay under uncertainty while conservatively spending quota. Evidence: restart-never-replays fixture. No guarantee is made that the external caller respected this instruction.

## review_training_exclusion

Type: partial_exclusion. Project every adapter transition onto main selected annotation versions: the projection is identity because main connections are read-only, while proposed revisions live solely in the separate audit queue. Both train and dev source revisions yield training_allowed=false. Counterexample to 'valid independent correction automatically trains': completed record still has no admission path. Evidence: distinct valid revision with main file bytes unchanged and explicit dev-review control. A future external admission tool would need its own reviewed policy.

## revision_failure_partition

Type: error_partition over finish's ordered validators: missing/wrong echoed base; archived-content mismatch; unavailable/stale selected source; malformed or identity-changing proposal; unknown target/pragmatic labels. Each first failure yields quarantine and preserves the raw report, not a releasable revision. Several faults can coexist and an earlier one masks later ones; categories are trace-first-failure, not independent causes. Tests cover stale key/hash, bad base echo, tree cycle and pragmatic class rejection. Unsupported MWT, empty nodes and enhanced dependency rows remain explicit bounded-adapter exclusions.

## bootstrap serialization boundary

The integrated counterexample falsifies 'SQLite busy_timeout alone serializes concurrent journal_mode/DDL initialization.' Bootstrap now holds a dedicated OS flock with a five-second acquisition deadline; contention returns an error before any reservation if the lease remains unavailable. WAL transition occurs only when the recorded mode differs, and fresh schema plus source identity commit atomically. Existing initialized queues perform mode/schema/identity reads and set only connection-local synchronization settings. This lease is released before the unchanged immediate quota transaction; it never represents a teacher-dispatch lease or permission to replay work.

Holding the bootstrap lease excludes successful competing bootstrap within its wait budget, while release admits normal initialization. A native held-lease control checks that timeout consumes no sentence quota and release restores progress. Separate cooperating callers and ordinary SQLite/storage semantics remain assumptions; unrelated SQLite writers can still cause bounded busy errors. Lock-wait deadline is not a hard real-time scheduler guarantee. Ten repeated native concurrent reservations remain finite evidence, and the original integrated failure is retained externally.

## tests

### Bounded native evidence

Eight focused tests cover explicit floor quotas/default disable/smallN, restart without pending replay, quarantine spending, valid distinct review-only revisions with source DB byte equality, terminal idempotence/refused replacement, source revision changes/bad pragmatic labels, concurrent reservations, bad base echo/tree cycle/feature syntax,96token and12000UTF8byte request limits, and explicit dev review outside training. Seven reused prag/src/schema.rs tests exercise categorical transport. All15 test executions pass in debug and release, offline locked, at most two build jobs.

Bootstrap follow-up: the integrated suite initially failed the unchanged concurrent-reservation test with SQLITE_BUSY at queue initialization. After the narrow lease/bootstrap fix, concurrency-fix-1.log through concurrency-fix-10.log each pass the original four-thread test. tests-bootstrap-fixed-{debug,release}.log each show all 16 checks passing (nine focused plus seven schema). The new held-bootstrap-lease test checks a bounded deadline, no reservation on timeout and recovery after release. No actual runtime/DB/GPU/terminal actions were used for these checks.

## History

### 2026-11-01 · descriptant

2026-11-01: the independent terminal helper may examine only a small finite share of selected sentences, at lower priority than foreground consultation. This module never launches that helper. It supplies explicit lifetime reservation accounting rather than an expected hash-sampling percentage.

### 2026-11-01 · tests

Fixtures use temporary local SQLite DBs/model-label files only. No real teacher, Claude API, tmux, daemon, GPU, main DB or assessment-label invocation. External logs: $MOVA_BIG/runs/peta-dolphin-2026-11-01/tests-published-{debug,release}.log. Concurrent test is bounded evidence, not proof about arbitrary storage failure. MWT/empty/enhanced dependencies are outside the adapter.
