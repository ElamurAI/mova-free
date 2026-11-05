# mova-seaspider-animal

## model

# mova-seaspider-animal

## Descriptant

Sea Spider as one of Mova's animals: English prompts through the native deputy, one fetch stream with an urgent-first job queue, answers from journals, private socket serve/ask. Source: `tools/seaspider/src/animal.rs`.

## Forward approximant

serve: load Sea Spider and deputy configs, check seaspider capabilities (operation fetch/robots/stats/page/status with slots urls/target/host/url/none), Deputy::load, private socket dir, Service::start (recover jobs.jsonl: unfinished jobs recorded interrupted, not replayed; spawn worker that opens the Crawler), angel_prioritized with handler answer(). answer: Deputy::run_with with Hands{urgent} for handler 'seaspider'; reply {answer: answer::report, report: compact}. Hands: fetch -> enqueue Job (<=64 waiting) and answer at once; robots -> stored_explanation if fresh, else enqueue and wait <=15 s only if idle; stats/page/status -> Reader/store journal and shared state. Worker: pick waiting job by (!urgent, seq), run each URL with Crawler::run([url]), log url and finished events; STOP: finish the current URL, record the rest not started.

## Invariant

A running job is never interrupted by a prompt; at most one request is in flight (single stream); waiting order is urgent first then arrival; no job is replayed after a restart; refused prompts queue nothing.

## Cost

Answers: O(journal) for store questions, O(1) for status; fetch work bounded below by 2 s per request on one IP.

## Composition

Parent: `seaspider`. One worker thread owns the only Crawler (store writer lock, robots cache, per-IP timers); shared job state behind a mutex; jobs.jsonl fsynced per event; deputy ledger per request id.

## behavior_complement

Not done: concurrent fetching, cancellation, scheduling by time, cross-process politeness with other Sea Spider processes, replay of interrupted jobs, non-English answers. Counterexample: an urgent prompt arriving during a 3 s fetch waits for it (served test; arrival-only ordering turns it red). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From jobs.jsonl one recovers each job's URLs, priority, sequence, start/finish times and per-URL outcomes; from the deputy ledger the prompt and plan per request id. Wall timings of waits and the exact interleaving of answers with fetches are not recoverable. No unique inverse is claimed.

## tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=(see History) (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs).

Negative controls: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site; not a proof.

Cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=(see History) (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs) Negative controls: a lock that always succeeds turns the singleton unit and CLI tests red; admitting every caller turns the served English test red (the Whale's fetch is queued); restored afterwards.

### Checks

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: serve takes the singleton lease first; the socket handler is answer_request: the caller (from, else octopus) is checked against service.callers, requests.jsonl gets one fsynced line per request (id, caller, peer pid and exe, priority, prompt sha256, admitted); a refused caller gets an English refusal and nothing runs; queued jobs carry asked_by.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs on 2026-11-04).

Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site (2026-11-04); not a proof.

2026-11-04: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04) Negative controls 2026-11-04: a lock that always succeeds turns the singleton unit and CLI tests red; admitting every caller turns the served English test red (the Whale's fetch is queued); restored afterwards.

### 2026-11-05 · document

#### Epoch 2026-11-05 (questions only observe)

`ACTING` lists fetch; `check_capabilities` refuses a deputy configuration that declares it `observes`. robots, stats, page and status are declared observing, so "Did you fetch URL?" is answered by an observing reading or refused, never by a fetch.
