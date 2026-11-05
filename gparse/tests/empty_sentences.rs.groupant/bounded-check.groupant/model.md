# gparse-test-empty-sentences-bounded-check

## descriptant

### Source-bound description

The decoder_accepts_no_words test supplies a one-row root-only matrix and asserts an empty head vector.

## approximant

### Forward approximant

The decoder_accepts_no_words test supplies a one-row root-only matrix and asserts an empty head vector.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Root-only matrix contains no syntactic words; only CPU decoder path runs.

#### Cost

O(1) fixture and decoder early return.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

No nonempty head vector passes this root-only assertion.

## reverse_reconstruction

All one-row matrices are compatible with an empty decoder result, because their cells are not inspected.

## empty_boundary

decode([vec![0.]]) returns []; this does not test general score validation.

## effect_boundary

This scope does not load a model or initialize CUDA.

## test_observation

The assertion observes vector emptiness only, not allocations or matrix cell values.

## branch_coverage

The len<=1 early-return branch runs; n=1 and DP branches do not.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): decoder_accepts_no_words; GPU empty saved-model case excluded

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
