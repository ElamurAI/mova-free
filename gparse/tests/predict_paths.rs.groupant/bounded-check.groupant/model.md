# gparse-test-predict-paths-bounded-check

## descriptant

### Source-bound description

Original ignored unscored_matches_scored_and_repeated_batches loads GP_TEST_MODEL and GP_TEST_CORPUS, filters fitting sentences, compares scored/plain outputs through six alternating rounds and checks fallback/empty cases.

## approximant

### Forward approximant

Original ignored unscored_matches_scored_and_repeated_batches loads GP_TEST_MODEL and GP_TEST_CORPUS, filters fitting sentences, compares scored/plain outputs through six alternating rounds and checks fallback/empty cases.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Requires CUDA and nonempty fitting corpus; every repeated output equals the initial scored reference.

#### Cost

Model I/O plus at least 14 predictions of bounded selected corpus; unexecuted in this epoch.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

A changed tuple fails equality; no statement about latency gain follows merely from timing output.

## reverse_reconstruction

A passing trace admits any model/corpus producing the same categorical repeated trees; six durations do not uniquely determine timing medians.

## input_boundary

GP_TEST_SENTENCES defaults1024; filter requires 0<len<max_len and then take(limit).

## repeat_protocol

Six rounds alternate ordering; equality is exact tuple comparison per run.

## timing_observation

Median averages sorted duration positions2,3; no explicit device synchronize appears in this probe.

## unexecuted_effect

External model/corpus loading and benchmark remain excluded; new CPU test below this scope supplies separate bounded evidence.

## tests

### Checks and limits

This exact fragment remains source-derived and unexecuted in this epoch. Parent/sibling selected evidence, not runtime coverage of this fragment: synthetic_cpu_prediction_preserves_slots_and_fallback

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
