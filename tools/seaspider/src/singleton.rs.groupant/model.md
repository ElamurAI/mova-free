# mova-seaspider-singleton

## model

# mova-seaspider-singleton

## Descriptant

One Sea Spider per host: an exclusive flock on the configured lock file held for the life of serve, fetch or robots; the holder records pid, role and start; a second start reports the holder. Source: `tools/seaspider/src/singleton.rs`.

## Forward approximant

acquire(path, role): create parent dirs, open, try_lock; WouldBlock -> Refused::Running{holder from the file}; else truncate and write {pid, role, started_unix}, fsync, return Singleton.

## Invariant

Lock file with one JSON line {pid, role, started_unix}; the lease is the open file description.

## Cost

One open, one flock attempt, one small write and fsync.

## Composition

Parent: `seaspider`.

## behavior_complement

no waiting for the lease, no stealing a lease, no cross-host or cross-lock-path exclusion; the kernel releases the lease at exit (no stale lease). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

the lock file recovers the last holder pid, role and start; earlier holders are not recoverable. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; not a proof.

## History

### 2026-11-04 · tests

cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04)

Negative controls 2026-11-04: a lock that always succeeds turns the singleton unit and CLI tests red; admitting every caller turns the served English test red (the Whale's fetch is queued); restored afterwards.
