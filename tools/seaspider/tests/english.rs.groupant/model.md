# mova-seaspider-english-tests

## model

# mova-seaspider-english-tests

## Descriptant

English tests of Sea Spider with the native model: prompt-to-capability/argument table, refusal stages, and a served end-to-end run against a local HTTP fixture. Source: `tools/seaspider/tests/english.rs`.

## Forward approximant

Planner table: 20 positive prompts with expected capabilities and arguments, 12 refusals with expected stages and one review candidate each, strict policy refusing loopback and .test. Served: seaspider::animal::serve in a thread on a private socket; English answers for stats, status, robots (fetched now, then from the store without a request), queued fetches; status answered within 1.5 s during a 3 s fetch; page order /slow, /u1 (urgent), /n1 with /private/x never requested and /u1 starting after /slow ended; past-tense status with the robots rule; page summary; per-host stats; six refusals sending no request; repeated id returns the journaled answer; STOP ends the server.

## Invariant

No assertion relies on a host other than 127.0.0.1; every negative assertion is a path the server must not have seen.

## Cost

About 12 s wall time (2 s per-IP spacing and a 3 s stalled response); model load once per test.

## Composition

Parent: `seaspider`. Skipped with a message when MOVA_OCTOPUS_TEST_MODEL is unset; network only to 127.0.0.1 with a.test pinned there and private hosts allowed in the test config only.

## behavior_complement

Not covered: TLS, real DNS, IPv6, angel restarts after a crash, journals after a restart, many concurrent clients, a deputy without the model. Counterexample role: each of the three negative controls turns one of these tests red. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A pass shows these prompts and fixture behaviours held on that run with that model; it does not reconstruct timings or prove behaviour for other phrasings. No unique inverse is claimed.

## tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=(see History) (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs).

Negative controls: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site; not a proof.

Cargo test --manifest-path tools/seaspider/Cargo.toml --offline with the 0.4 model: 2 English tests pass with the updated expectation.

Cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=(see History) (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs) Negative controls: a lock that always succeeds turns the singleton unit and CLI tests red; admitting every caller turns the served English test red (the Whale's fetch is queued); restored afterwards.

### Checks

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Whales in English)

Epoch 2026-11-04: 'What is the status of URL?' is now read through its subject 'status' and refused at argument binding (argument_unbound) instead of unsupported_capability.

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: The served test configures service.lock and callers octopus, crab; it checks a refused second in-process serve, Crab answered through mova_octopus::relay, a Whale refused with nothing queued, a Crab-style animal_ask deputy relaying «Is the queue empty?», and requests.jsonl rows.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs on 2026-11-04).

Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

These are finite fixtures and one live smoke against the project's own site (2026-11-04); not a proof.

2026-11-04: cargo test --manifest-path tools/seaspider/Cargo.toml --offline with the 0.4 model: 2 English tests pass with the updated expectation.

2026-11-04: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04) Negative controls 2026-11-04: a lock that always succeeds turns the singleton unit and CLI tests red; admitting every caller turns the served English test red (the Whale's fetch is queued); restored afterwards.

### 2026-11-05 · document

#### Epoch 2026-11-05

"Is the queue empty?" failed under en.bin 2483516a (correct parse, «empty» root, no repair) and passes through the subject-observation rule. New controls: negated, past, unobserved-subject and imperative "Empty the queue." refusals, and five questions about fetching (including "Fetch URL?") that must never plan a fetch.
