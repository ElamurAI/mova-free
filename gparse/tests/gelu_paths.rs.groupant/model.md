# gparse-test-gelu-paths

## descriptant

### Source-bound description

Configuration test checks missing GELU defaults Erf, typo errors and Tanh save/load. CUDA grid computes erf/tanh outputs and autodiff at 16001 f32 samples [-8,8]; compares max abs values<0.0005, derivatives<0.001 and tanh central finite-difference<1e-5 (epsilon 1e-4). Corpus parser timing requires 2048 fitting sentences and is excluded.

## approximant

### Forward approximant

Configuration test checks missing GELU defaults Erf, typo errors and Tanh save/load. CUDA grid computes erf/tanh outputs and autodiff at 16001 f32 samples [-8,8]; compares max abs values<0.0005, derivatives<0.001 and tanh central finite-difference<1e-5 (epsilon 1e-4). Corpus parser timing requires 2048 fitting sentences and is excluded.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Finite grid thresholds do not imply all-real error or tree agreement; configuration rejection is categorical.

#### Cost

O(16001) activation/autodiff plus finite-difference operations; excluded experiment O(2048 inference plus 16 repeats).

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

Configuration test checks missing GELU defaults Erf, typo errors and Tanh save/load. CUDA grid computes erf/tanh outputs and autodiff at 16001 f32 samples [-8,8]; compares max abs values<0.0005, derivatives<0.001 and tanh central finite-difference<1e-5 (epsilon 1e-4). Corpus parser timing requires 2048 fitting sentences and is excluded.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Finite grid thresholds do not imply all-real error or tree agreement; configuration rejection is categorical.

Passing maxima admit many error arrays below thresholds; observed approximate outputs do not recover exact inputs/weights. Config text preimage includes ignored keys.

## models

# Typed model catalog

## behavior_complement

Passing selected assertions excludes their stated counterexamples within the fixtures. External saved-model/corpus experiment remains unexecuted; success of helper checks cannot establish it.

## reverse_reconstruction

Passing maxima admit many error arrays below thresholds; observed approximate outputs do not recover exact inputs/weights. Config text preimage includes ignored keys.

## numerical_metric

Absolute thresholds are three distinct metrics, not supported percent-budget profiles.

## effect_boundary

Selected test process owns local fixture tensors/state; explicit CUDA tests serialize. No teacher/API, dataset or external model reads selected.

## assurance_scope

Compiled selected assertions establish only finite observations; ignored experiments and backend allocation failures remain gaps.

## failure_precedence

Fixture creation and backend operations must succeed before assertions; a compile-only result is distinct from executed assertions.

## tests

### Checks and limits

activation_configuration_roundtrips; cuda_values_and_derivatives

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/tests/gelu_paths.rs

- activation_configuration_roundtrips: twice debug, twice release, all four exits 0.
- cuda_values_and_derivatives: twice debug, twice release, all four exits 0.

#### Controls

0 source-local compiled assertion mutants; 0 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Original parser_agreement_and_throughput corpus experiment not run.

CUDA softmax comparisons use absolute tolerance2e-6. GELU grid maxima observed: values0.0004734993, derivatives0.00086838007, finite difference0.0000005534244635896357; respective strict caps5e-4/1e-3/1e-5. These are finite observations, not percentage profiles or throughput claims.

Receipts: external wave30 gparse run native-commands.json, results.json, controls.json and source-test-map.json. Sorted exact test names/outcomes match per profile; numeric diagnostics and timings need not be byte-identical. Strict scoped library Clippy reports ten pre-existing production lints; appended test blocks were formatted independently without production reformatting.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-10-31 · evidence

Heading `### Execution evidence — 2026-10-31` stays in evidence as `### Execution evidence`.
