# mova-seaspider-singleton-tests

## model

# mova-seaspider-singleton-tests

## Descriptant

CLI test of the singleton: with the lease held, serve, fetch and robots exit 5 naming the holder and create no socket or store; released, serve proceeds past the lease. Source: `tools/seaspider/tests/singleton.rs`.

## Forward approximant

Hold the lease in-process, run the built binary three times (serve, fetch, robots), expect exit 5 and the English holder report; drop the lease and expect serve to fail later on the missing deputy instead.

## Invariant

Temporary directory with a config naming its own service.lock; no network.

## Cost

Four short process runs.

## Composition

Parent: `seaspider`.

## behavior_complement

not covered: two real served processes, crash release timing, other users. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

a pass shows these refusals on that run; it reconstructs nothing else. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; not a proof.

## History

### 2026-11-04 · tests

cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04)

Negative controls 2026-11-04: a lock that always succeeds turns the singleton unit and CLI tests red; admitting every caller turns the served English test red (the Whale's fetch is queued); restored afterwards.
