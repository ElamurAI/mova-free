# gparse-test-softmax-backward-bounded-check

## descriptant

### Source-bound description

CUDA gradient test builds three rows for widths1,3,17,33,257; compares fused/reference probabilities and weighted gradients, f64 central differences, narrow offsets/transposes, and stable masked logits.

## approximant

### Forward approximant

CUDA gradient test builds three rows for widths1,3,17,33,257; compares fused/reference probabilities and weighted gradients, f64 central differences, narrow offsets/transposes, and stable masked logits.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Every checked forward/backward discrepancy<2e-6; ±1000 logits stay finite and selected -1e9 columns exactly zero; upstream fused gradient absent.

#### Cost

O(sum widths) tensor storage and a fixed number of full-row objective evaluations per width.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Reference-gradient absence or threshold exceedance fails; no fallback to CPU is accepted because CUDA creation unwraps.

## reverse_reconstruction

Probabilities admit row-shifted logits; derivative tolerances admit intervals around finite-difference observations.

## layout_cases

narrow(0,1,2) tests nonzero contiguous offset; transpose tests contiguous conversion and backward through view.

## finite_difference

Four indices per width, possibly duplicated at width1; epsilon1e-4. Do not count these as four distinct coordinates at width1.

## stable_mask

[1000,999,-1e9] and [-1000,-1001,-1e9] produce finite rows and exact masked zeros on tested cells.

## negative_reference

ops::softmax_last_dim graph must lack input gradient; this is distinct from custom derivative correctness.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): cpu_f64_uses_reference; fused_softmax_matches_reference_gradients_and_finite_differences

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
