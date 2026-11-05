# gparse-test-gelu-paths-bounded-check

## descriptant

### Source-bound description

cuda_values_and_derivatives evaluates 16001 f32 inputs i*0.001-8 on CUDA for erf/tanh GELU and their summed-output gradients; f64 central differences use epsilon=1e-4.

## approximant

### Forward approximant

cuda_values_and_derivatives evaluates 16001 f32 inputs i*0.001-8 on CUDA for erf/tanh GELU and their summed-output gradients; f64 central differences use epsilon=1e-4.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

All four arrays finite; max value<5e-4, derivative<1e-3, finite-difference error<1e-5.

#### Cost

O(16001) tensors and host values; finite-difference tanh formula evaluated twice per grid point.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

No NaN value/gradient or threshold-equal maximum passes strict comparisons.

## reverse_reconstruction

Passing maxima permit every elementwise error array within these bounds; values alone do not reconstruct derivative observations.

## grid_domain

Integer i=0..16000 yields finite f32 samples in [-8,8], with floating rounding rather than exact decimal arithmetic.

## numerical_metric

Three independent absolute-error thresholds; no relative-percent guarantee near zero.

## derivative_reference

Finite difference compares tanh derivative to f64 analytical expression numerically; it is not a CUDA proof.

## effect_boundary

Allocates CUDA tensors and transfers arrays, no corpus or model-file reads.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): activation_configuration_roundtrips; cuda_values_and_derivatives

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
