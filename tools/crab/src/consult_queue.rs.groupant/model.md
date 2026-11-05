# crab-consult-queue

## model

# Crab consult queue — source-bound model

## Descriptant

`consult_queue.rs` keeps the consultations Crab owes its Dolphins: tasks the Octopus asks in English and consultations Crab composes itself after a refusal or a failing worker. It stores, orders and bounds them; delivery is Crab's consult protocol in `angel.rs`.

## Forward

Queue::open creates private 0700 directories. enqueue(dolphin, task, urgent, origin, trigger, now) -> entry {id cq-<now>-<n>-<hash8>, state waiting} plus waiting_ahead. waiting() sorts (priority != urgent, id); waiting_for(d) filters by Dolphin. mark(id, state, detail) records state transitions with times. trigger(t, key, trigger, task) -> None when the key exists or crab_recent(now) >= max_per_hour, else a bounded crab entry for t.dolphin. refusal_task, worker_trigger (failed or >= 3 consecutive failures; key worker:name:restarts:failures:state) and gpu_trigger (state failed; key gpu:at:hash) compose bounded plain-text tasks. record_advice writes a create-new advice record and marks the entry answered.

## Invariant

- Every stored task is non-empty plain text of at most 12000 bytes.
- A Crab-initiated entry always carries its trigger and key; one key yields at most one entry.
- Advice records always have actions_taken [] and automatic_execution false; this module calls no operation.
- Delivery order within a Dolphin is urgent first, then arrival; priority never changes a sent entry.

## Cost

O(n) directory listing and JSON decode per call for n <= 4096 entries; writes are single small files.

## Composition

`angel.rs` owns one Queue in its Context: consult_enqueue and consult_state requests, the refusal trigger after a refused ask, worker/GPU triggers and per-Dolphin delivery in consult_tick, which dispatches consult_opus and marks entries.

## behavior_complement

Type: behavior_complement. Not done: sending anything to a terminal (angel's consult protocol does), executing or parsing advice, retrying a failed delivery, deleting entries, judging advice.

## reverse_reconstruction

Type: reverse_reconstruction. The entry files recover, per Dolphin, the order of asks and their states with times; advice records recover which report answered which trigger (by hash). Task texts are stored; report texts are referenced, not copied.

## advice_is_not_action

Type: partial_exclusion. Domain: Crab-initiated consultations and their reports. Excluded output: any worker, marker or prompt operation caused by a report. Negative control: an injected start of Eel on advice containing "Start Eel" turns the integration test red.

## checks

### Checks

Tests use private tmux servers (`tmux -L`) and temporary directories; no real LLM terminal, model or network call.

## History

### 2026-11-04 · checks

Heading `### Checks (2026-11-04)` stays in checks as `### Checks`.

2026-11-04: 3 module tests (order and bounds, trigger dedupe/hour bound/size bound, composed tasks) and 2 angel integration tests on private tmux servers (urgent first, busy waits, refusal trigger, advice record with no action; four Dolphins routed by name, unpinned Copilot refused); red controls: advice executed, all consultations routed to the first Dolphin.
