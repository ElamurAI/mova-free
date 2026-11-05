# gparse-test-softmax-backward

## descriptant

### Source-bound description

CUDA checks deterministic values/weights at widths 1,3,17,33,257, compares fused/reference forward and weighted VJP<2e-6 abs; independent f64 central differences at four coordinates each width use epsilon1e-4 and tolerance2e-6. Offset slices and transposes checked, large ±1000 rows remain finite and -1e9 columns zero; upstream fusion is no-gradient control. CPU F64 equality checks fallback.

## approximant

### Forward approximant

CUDA checks deterministic values/weights at widths 1,3,17,33,257, compares fused/reference forward and weighted VJP<2e-6 abs; independent f64 central differences at four coordinates each width use epsilon1e-4 and tolerance2e-6. Offset slices and transposes checked, large ±1000 rows remain finite and -1e9 columns zero; upstream fusion is no-gradient control. CPU F64 equality checks fallback.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

All thresholds are absolute finite-fixture measurements; no universal gradient error certificate. CPU dtype stays F64.

#### Cost

O(sum widths) tensor work and O(sum widths) for each selected finite-difference objective; tiny bounded GPU allocation.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

CUDA checks deterministic values/weights at widths 1,3,17,33,257, compares fused/reference forward and weighted VJP<2e-6 abs; independent f64 central differences at four coordinates each width use epsilon1e-4 and tolerance2e-6. Offset slices and transposes checked, large ±1000 rows remain finite and -1e9 columns zero; upstream fusion is no-gradient control. CPU F64 equality checks fallback.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

All thresholds are absolute finite-fixture measurements; no universal gradient error certificate. CPU dtype stays F64.

A passing finite-difference inequality allows any derivative in the tolerance interval, not exact unique gradient; softmax input compatible logits differ by row shifts.

## models

# Typed model catalog

## behavior_complement

Passing selected assertions excludes their stated counterexamples within the fixtures. No corpus/model-file mutation occurs in selected synthetic checks.

## reverse_reconstruction

A passing finite-difference inequality allows any derivative in the tolerance interval, not exact unique gradient; softmax input compatible logits differ by row shifts.

## numerical_metric

Mask zeros exact on two tested cells; forward and derivative comparisons use <2e-6 independently.

## effect_boundary

Selected test process owns local fixture tensors/state; explicit CUDA tests serialize. No teacher/API, dataset or external model reads selected.

## assurance_scope

Compiled selected assertions establish only finite observations; ignored experiments and backend allocation failures remain gaps.

## failure_precedence

Fixture creation and backend operations must succeed before assertions; a compile-only result is distinct from executed assertions.

## tests

### Checks and limits

cpu_f64_uses_reference; fused_softmax_matches_reference_gradients_and_finite_differences

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/tests/softmax_backward.rs

- cpu_f64_uses_reference: twice debug, twice release, all four exits 0.
- fused_softmax_matches_reference_gradients_and_finite_differences: twice debug, twice release, all four exits 0.

#### Controls

0 source-local compiled assertion mutants; 1 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Finite synthetic CPU/CUDA observations; no trained-corpus quality or all-input numeric guarantee.

CUDA softmax comparisons use absolute tolerance2e-6. GELU grid maxima observed: values0.0004734993, derivatives0.00086838007, finite difference0.0000005534244635896357; respective strict caps5e-4/1e-3/1e-5. These are finite observations, not percentage profiles or throughput claims.

Receipts: external wave30 gparse run native-commands.json, results.json, controls.json and source-test-map.json. Sorted exact test names/outcomes match per profile; numeric diagnostics and timings need not be byte-identical. Strict scoped library Clippy reports ten pre-existing production lints; appended test blocks were formatted independently without production reformatting.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-10-31 · evidence

Heading `### Execution evidence — 2026-10-31` stays in evidence as `### Execution evidence`.
