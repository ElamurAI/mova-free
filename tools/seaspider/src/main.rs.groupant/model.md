# mova-seaspider-main

## model

# mova-seaspider-main

## Descriptant

seaspider CLI: fetch CONFIG QUEUE, get STORE URL|sha256:HEX, robots CONFIG URL, stats STORE. Source: `tools/seaspider/src/main.rs`.

## Forward approximant

Match exact argument shapes; fetch/robots load the config and run a current-thread tokio runtime; get writes the body to stdout and a one-line summary to stderr; stats prints JSON; anything else prints usage and exits 1.

## Invariant

Unknown commands or argument counts have no effect beyond the usage message and a non-zero exit.

## Cost

O(1) dispatch plus the chosen operation.

## Composition

Executable over crawl and store.

## behavior_complement

No flags, no daemon mode, no network without a config. Counterexample: 'seaspider fetch' alone exits non-zero (links_file_follow_mode_and_cli). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The printed summary or body identifies the operation's result; the configuration and queue are not recoverable from output. No unique inverse is claimed.

## tests

### Checks

End-to-end: get, stats and a usage error through the built binary.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (English)

2026-11-04: `serve CONFIG DEPUTY_CONFIG SOCKET` and `ask SOCKET ID [urgent]` (prints the English answer, then the JSON reply).

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: fetch and robots take the singleton lease; a refused serve/fetch/robots prints the holder and exits 5 (EXIT_RUNNING); `ask --from CALLER`.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.

2026-11-04 (English): cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs on 2026-11-04). Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

2026-11-04: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04) Negative controls 2026-11-04: a lock that always succeeds turns the singleton unit and CLI tests red; admitting every caller turns the served English test red (the Whale's fetch is queued); restored afterwards.
