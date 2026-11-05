# en-dict

## descriptant

### en-dict
Frozen blob offsets, probe tables, records and fingerprint cache

## approximant

### Forward relation
Probe str_key&mask then byte equality, zero entry terminates, linear wrap. Local/invalid Sym text empty, lower NONE, known false, paradigm empty. Records preserve blob order.

No executable Rust surrogate attached: no demonstrated metric for preserving these categorical, stateful or effect observations. Supported future caps are 1/2/5/10%, not a claim of current error.

## invariant

### Domain, invariant and cost
Probe str_key&mask then byte equality, zero entry terminates, linear wrap. Local/invalid Sym text empty, lower NONE, known false, paradigm empty. Records preserve blob order.
No runtime disk reads; hash alone never matches; corrupted table can loop or panic.
Expected short probes, bound needs empty slot invariant; fingerprint O(blob) once; records O(r); paradigm allocates boxed iterator.
Inputs must satisfy the indexing/generated-data assumptions stated by the typed catalog; allocation failure and overflow beyond bounded fixtures remain untested.

## composition

### Mathematical combiner
Source bytes and twice-generated syntax constrain this model. Forward relation: Probe str_key&mask then byte equality, zero entry terminates, linear wrap. Local/invalid Sym text empty, lower NONE, known false, paradigm empty. Records preserve blob order.
Reverse relation: Valid unique lemma text recovers Sym; analyses and paradigms have multiple tag/source antecedents; invalid Syms collapse to empty.
The ordered typed family records independent exclusions, compatible antecedents and source-specific boundaries. Fragment ranges compose through actual control flow; they do not cover all helpers.
Epoch memory: empty learned support, lossy projections and non-atomic effects cannot inherit stronger downstream guarantees.

## models

## behavior_complement
No runtime disk reads; hash alone never matches; corrupted table can loop or panic.

## reverse_reconstruction
Valid unique lemma text recovers Sym; analyses and paradigms have multiple tag/source antecedents; invalid Syms collapse to empty.

## probe_termination
Linear probe requires a power-of-two mask/table with an empty slot; malformed generated layouts are trusted.

## namespace_boundary
Local and out-of-range Sym have empty views; Form is private but Rec::form can construct handles from public arbitrary Rec id.

## record_layout
Record width12: tag byte, source byte, id offset4, count offset8; global ids interpreted differently in analysis/paradigm.

## fingerprint_binding
Fingerprint memoizes checksum of BLOB; downstream store couples persisted model to blob, not merely source names.

## tests

### Actual source checks
Named check: dict::wave27_tests::invalid_namespace_and_frozen_lookup. Command: cargo test -p en --lib wave27_tests -- --test-threads=1 (also --release). Four runs: each 10 passed, 0 failed, numeric exit0; sorted named outcomes equal. This file contributes one named check per run.
Control present for this source: false. Hash zeroed byte contribution, text changed empty chrF to1, lexgen removed three-cell PastPart copy: each compiled and failed one assertion (exit101), then exact source restoration and ten-test positive rerun (exit0). Other sources have no compiled mutation claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: read complete source before prospective model and bounded tests. Production code retained byte-for-byte; only inline synthetic checks added. Coverage is partial, not a proof.

### 2026-10-31 · tests

See evidence.json and external octopus-wave27-lexical-2026-10-31 receipts. Static built dictionary read is part of the linked library; no corpus or external services used. Unknown support, malformed model/blob, resource failures, cycles and exhaustive Unicode behavior are unexecuted gaps.

### 2026-11-05 · fingerprint_binding

2026-11-05: `RELEASED_FINGERPRINT` = 0xe3e0cb7d31c7e204, read from header bytes 20..28 of en.bin `2483516a…`; `licence_tests::fingerprint_is_the_released_one` pins it, and `fingerprint_pin_shows_red` (one byte appended, one byte flipped) is its negative control. `LICENCE` (`include_str!("../data/LICENSE.md")`) is a separate static string, never part of BLOB: the test also checks that no licence text appears in the blob. Changing data/LICENSE.md moves no lemma number and no fingerprint.

### 2026-11-05 · tests

Named check (2026-11-05): `cargo test -p en --lib licence` — `dict::licence_tests::fingerprint_is_the_released_one`, `dict::licence_tests::fingerprint_pin_shows_red` and `lexgen::personal_tests::licence_cites_every_source` passed (3 of 3).
