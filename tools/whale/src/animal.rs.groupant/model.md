# mova-whale-animal

## Descriptant

The Whales as one of Mova's animals: a persistent singleton service on a private socket; English prompts through the native deputy (describe/count/status/show/pause/resume), English answers; each admitted request becomes a new one-item pinned Whale run in a child of the pinned mova-whale executable; request journal with no replay. Source: `tools/whale/src/animal.rs`.

## Forward approximant

serve: load AnimalConfig (survey skeleton without run_root/items/workers, pinned executable and catalog) and deputy config, check whale capabilities, Deputy::load, Service::start (service.lock, executable hash, Spec::load, Catalog::load with pin check, recover requests.jsonl, spawn workers), angel_prioritized with answer(). Hands: describe -> resolve each URL/id in the catalog; already running/waiting/failed/described -> said, nothing queued; else hash the source, journal queued, push Job; count -> store buckets with complete versions + journal totals; status/show -> state, journal, store bucket versions (opening of the length-field artifact); pause/resume -> PAUSE marker. Worker: while not paused pick (!urgent, seq), journal started, run_job: create runs/<job>, whale::prepare one-item survey, check executable hash, spawn `whale CONFIG --once` (MOVA_CODEX_BIN from config), read the terminal receipt -> Finished; journal finished.

## Invariant

A running request is never interrupted by a prompt; waiting order is urgent first then arrival; started is journaled before preparation; a started request without a receipt is uncertain and never relaunched; a source with a failed attempt is not attempted again by a prompt; no existing queue, manifest or bundle is rewritten; refused prompts queue nothing.

## Cost

Start: one pass over the catalog (hash + index; 186,435 rows read in ~10 s). Answers: O(log n) catalog lookups, one bucket directory read; count: one directory read of the store kind. Work: one child per request, at most one teacher call.

## Composition

Parent: whale-workers. Uses whale::prepare and the Whale run (child process), spec::Spec for the bundle layout, util pins/leases, mova_octopus Deputy/socket/realize, answer.rs for English.

## behavior_complement

Not done: fetching URLs (catalog only), preemption, cancellation, retrying failed or uncertain sources, scheduling by time, non-English answers, writing descriptions itself (the configured teacher writes; Mova only reads prompts and realizes answers). Counterexample: an urgent request arriving during a running description waits for it (unit and served tests; arrival-only ordering turns them red). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From requests.jsonl one recovers each request's source item (path, sha256, format), priority, sequence, start/launch/finish times, outcome, bundle manifest and word count; from runs/<job>/survey the prepared prompt, reservation and receipt; from the deputy ledger the prompt and plan per request id. The interleaving of answers with work and the child's wall timings inside its call are not recoverable. No unique inverse is claimed.

## History

### 2026-11-04 · Cost

Start: one pass over the catalog (hash + index; 186,435 rows read in ~10 s on 2026-11-04). Answers: O(log n) catalog lookups, one bucket directory read; count: one directory read of the store kind. Work: one child per request, at most one teacher call.

### 2026-11-04 · document

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: Optional `seaspider_socket` (absolute) in AnimalConfig; unset by default.

### 2026-11-05 · document

#### Epoch 2026-11-05 (questions only observe)

`ACTING` lists the operations that change state (describe, pause, resume); `check_capabilities` refuses a deputy configuration that declares any of them `observes`, so a question can never queue, pause or resume work. count, status and show are declared observing in the example and live configurations.
