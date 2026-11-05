# peta-supervisor

## descriptant

### Peta supervisor

The useful design distinction is reservation versus success: an absent or dead PID is not evidence that a teacher call did not happen. A launch journal therefore blocks ambiguous restart rather than hiding retries. Another distinction is pool parallelism versus GPU concurrency: teacher requests overlap; GPU entrypoints use a shared lock owned by main.rs. File and child state have explicit limits rather than an all-or-nothing claim.

The initial writer-start barrier was corrected before publication: preparation initializes exact empty campaign identities, not just configuration paths. Repeat preparation also rehashes current source/queue/heldout bytes. Native fixtures cover the boundaries without starting any real teacher or GPU process.

A final independent review rejected the temporary-only legacy lock: it did not exclude the old full queue restarting later. run now retains and inherits a lifetime lease and pins coverage identity across preparation/run. Receipt classes remain separate, including missing/quarantined work that is not silently retried. A bounded context-union addition also excludes exact-text aliases and joined blocks from the existing controller's prefix context without changing the source snapshot or original protected corpus.

## model

# Peta annotation supervision

## Forward and composition

Let Q be the unique-ID immutable legacy queue, L the union of receipt/reservation IDs, H protected heldout IDs and exact-text aliases, J IDs in joined source blocks, and P the pending-state predicate. Preparation computes R=[q in Q: P(q) and id(q) not in L union H union J] in original order. For n in 1..8, shard i receives R positions k with k mod n=i. Distinct shards have disjoint ID sets; interleaving recovers R. The Rust checks pin actual config/source/queue/heldout bytes before both fresh and idempotent preparation. A stopped marker alone is insufficient: status must be stopped, legacy controller flock free, and every reservation must have a matching receipt. Uncertain jobs are never resubmitted automatically.

Publication stages queue/config files, ready empty campaign/ledger/jobs and the exact review-controller campaign identity, then renames the prepared annotation directory. This lets the writer read an empty committed campaign before any annotation child starts. No source or assessment labels are generated here. Existing reviewworker -> isolated cycle -> Rust teacher transport -> Codex is the annotation path; provider model identity remains unchanged.

run launches each prepared reviewworker once, journaling a reservation before spawn and PID/start stamp after spawn. It never automatically retries failed or uncertain children. A common/local STOP propagates owned STOP markers and lets active cycles finish; PAUSE propagates before launching children and throughout supervision. Owned markers alone are removed on a deliberate clean restart. Parent death may leave running children; unresolved journals refuse a new launch. An explicit STOP on such a restart propagates before returning an unresolved-state error.

GPU serialization is delegated to the executable aliases owned by main.rs. Generated configs replace gparse and both Python interpreter entrypoints with these aliases, retain the real executables under CYCLE_PETA_* fields, and share one GPU lock path with training. This module does not itself prove GPU exclusion or enforce it inside arbitrary external programs. Teacher waits can overlap while GPU inference/training calls serialize.

## Invariant and cost

The pending shard ID union is exactly R; no legacy-covered, heldout ID/text alias or joined-block ID is submitted by this preparation. Existing controller checks selected source/tree hashes again. Quiescent trusted legacy artifacts are assumed; the model is not a provenance authenticator. Per-controller locks and reservations remain necessary beyond the parent journal. Preparation is O(source bytes + queue bytes + receipt IDs log receipt IDs); source metadata streams, queue bytes and shard buffers are held in memory. Each worker still loads the full source independently. Runtime supervision is O(n) per poll; sleep is 1..30 seconds and no throughput speedup is asserted.

## Limits

Filesystem operations are individually synced/atomic, not one cross-process transaction. A preparation failure can leave an unpublished staging directory. There is no automatic cleanup, detached-process kill or requeue. PID stamps aid diagnosis only; running/uncertain records are refused even when a PID vanished. Old-controller STOP remains an intentional migration prerequisite on restarts. Source-groupant syntax is partial and does not establish semantic or linguistic correctness.

## Legacy lease and context boundaries

Preparation records separate legacy per-ID receipt classes: accepted, quarantined, missing/rejected and other excluded. Every class remains excluded from automatic resubmission; a missing result is not success. The covered-ID set and class counts are content-bound into preparation identity. run re-acquires the legacy controller flock after prepare returns, rechecks the covered identity and retains the lease for its full lifetime. The descriptor is intentionally inherited by annotation descendants, so a parent exit alone does not permit the legacy full queue to restart while those descendants live. A bounded native fixture observes refusal of a second legacy lock while children run, verifies the descriptor survives executable handoff and observes release only after completion. This coordinates cooperating controllers, not arbitrary code ignoring flock.

To protect prefix context as well as selected IDs, preparation publishes a new context-exclusions.conllu containing the unchanged original heldout bytes plus header-only ID/text entries for exact-text aliases and joined-source IDs. Child controller heldout paths and identities bind this union. Existing controller context construction consults the resulting ID set. A second streamed source pass derives metadata and rechecks source hash; no large source rewrite occurs. The union is an inference/context exclusion artifact, never an assessment corpus or training source. The original protected file is unchanged. These claims depend on the existing controller's ID-exclusion behavior; they are not properties of arbitrary external parsers.

## default_fish_workers

## catalog

# Typed obernants

## behavior_complement

Domain: unique-ID immutable queue, trusted pinned inputs, quiescent legacy campaign, and normal filesystem behavior. Observe submitted shard IDs. With eligible R defined in model.md, a submission containing an ID outside R is excluded by the modeled preparation relation. In particular, receipt/reservation IDs, protected IDs/text aliases, joined-source IDs and nonpending states are excluded. This is a source-derived partial exclusion, not a proof from a negative test alone. Counterexample to the broader claim 'STOP is sufficient': STOP can coexist with an active old controller or unresolved reservation. Native controls verify both refusals and protected alias exclusion.

## reverse_reconstruction

Domain: all n ordered shard sequences and their published worker count. Observation is their ordered queue records. Interleave(shards)=R reconstructs the filtered pending sequence uniquely because assignment uses position modulo n. It cannot recover excluded queue records, teacher outputs, original source semantics or complete old history from shards alone. Many Q/L/H/J choices produce the same R. Counterexample to unique original-queue recovery: append a covered record to Q; it remains absent from every shard. Evidence: the deterministic disjoint-shard fixture and byte-equal idempotent manifest, not a machine-checked theorem.

## child_launch_uncertainty

Type: error_partition over launch trace boundaries. Domain: one supervisor with its flock and a persisted child journal. A reserved record precedes spawn; a running record follows PID publication; finished requires a successful child exit with idle/stopped controller status. A crash between reservation and PID publication admits both no launch and a live orphan, so the compatible predecessor set is non-singleton. Refusal avoids guessing and duplicate teacher work. Counterexample: a dead recorded PID does not establish that no detached worker remains. Native controls use reserved/running journals, a failing fake worker and explicit STOP propagation; they do not inject real power loss.

## shard_membership_converse

Type: reverse_reconstruction specialized to assignment. Domain: fixed n and remaining-sequence position k. Observation slot i constrains compatible positions to {k >= 0: k mod n=i}; the slot alone cannot recover the queue position or sentence. This differs from whole-sequence interleaving above. Counterexample to slot identifying an ID: positions i and i+n share a slot. Evidence: actual round-robin source and disjoint fixture; no formal proof or cardinality-based semantic claim.

## legacy_replay_exclusion

Type: partial_exclusion over cooperating controller starts. Domain: a prepared pool, its pinned legacy coverage and one successful lifetime legacy flock acquisition. During the inherited lease, an independent legacy controller cannot acquire its exclusive controller lock; thus replay from the old full queue is excluded through that entrypoint. Counterexample to merely checking at preparation: the old daemon can start after the temporary guard is released. The new run lease closes that boundary and survives exec in descendants. Oracle: legacy_lease_is_held_and_inherited_until_children_finish. Status: partial source-derived exclusion and finite process evidence; noncooperating writers and filesystem lock violations remain outside the model.

## Targeted child lease inheritance

The parent retains FD_CLOEXEC. Only the selected annotation child's pre_exec hook clears that descriptor flag after fork. An unrelated exec therefore cannot extend the legacy lease lifetime. This is a scoped partial exclusion over command inheritance, not a claim that fork creates no transient descriptor copy. A finite control confirms that the intended child retains the lock while alive and an unrelated longer-lived child does not retain it after the intended child exits. Trusted cooperative descendants may continue inheriting the lease as required.

Preparation-only leases explicitly LOCK_UN on scope exit: they are not part of descendant lifetime protection, and a transient fork duplicate must not extend their scope. A retained duplicate-descriptor negative control confirms that a competing lock is refused before scope exit and admitted afterward. Lifetime legacy leases remain close-only and intentionally inherited by the selected descendants, as checked separately.

## tests

### Native checks

Twelve focused supervisor tests run in an external harness with cached dependencies, CARGO_BUILD_JOBS=2, --offline --locked and --test-threads=1. They cover deterministic disjoint shards and identity tamper; heldout exact-text aliases and joined sources; legacy STOP/status/flock and uncertain reservations; initialized empty store-readable campaigns and changed heldout rejection; pool lock and reserved/running launch refusal; three fake children exactly once; failed-child no-retry and orphan STOP propagation; common STOP preventing start.

Tests launch only small fake shell workers, never Codex, parsers, GPU training or live campaign writes. Earlier parallel execution exposed parent-global close-on-exec clearing: unrelated child execs could retain a fixture's legacy lease. Inheritance now clears the flag only in the intended command's pre_exec child. A dedicated control observes the parent's close-on-exec flag, intended-child retention and independent exec exclusion. The failed exploratory runs remain external historical evidence. These are finite process/filesystem controls, not power-loss or arbitrary concurrent-writer proofs.

Follow-up controls additionally separate accepted/quarantined/missing receipt counts and reject changed coverage after preparation; observe the inherited legacy lock across child execution and its eventual release; and build a context union containing heldout aliases/joined IDs while byte-preserving the original protected file. Final debug/release runs each pass twelve tests in serial mode.

## History

### 2026-11-01 · descriptant

Epoch 2026-11-01: the operator authorizes parallel Rust-controlled annotation through the existing Codex teacher, durable DB versions and a separate GPU trainer. This node formalizes migration from one audit worker into disjoint immutable annotation campaigns and fail-closed child supervision. It preserves provider/model identifiers and existing private paths; current display name is Peta.

### 2026-11-04 · default_fish_workers

A config without `workers` shards into crate::DEFAULT_FISH_WORKERS (2) workers; the 1..=8 bound is unchanged. The worker count enters the frozen identity, so an existing run root prepared with the former implicit 3 is refused as an identity change rather than silently re-sharded (2026-11-04).
