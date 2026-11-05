# gparse-test-gpu-gradients

## descriptant

### Source-bound description

Tiny CUDA Net d=16/layers=1/heads=2 builds two-word synthetic batch; training arc-square loss must reach word,QKV,LN and arc weights with positive finite L1 gradient; Q,K,V slices checked separately. Inference fused control requires zero Q/K and nonzero V.

## approximant

### Forward approximant

Tiny CUDA Net d=16/layers=1/heads=2 builds two-word synthetic batch; training arc-square loss must reach word,QKV,LN and arc weights with positive finite L1 gradient; Q,K,V slices checked separately. Inference fused control requires zero Q/K and nonzero V.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Any QKV gradient alone is insufficient: separate Q/K routes must learn. Seed42/dropout0 bound the fixture, no training quality conclusion.

#### Cost

O(AFFIX*16) parameter allocation plus tiny attention/backward; no corpus or persisted optimizer.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

Tiny CUDA Net d=16/layers=1/heads=2 builds two-word synthetic batch; training arc-square loss must reach word,QKV,LN and arc weights with positive finite L1 gradient; Q,K,V slices checked separately. Inference fused control requires zero Q/K and nonzero V.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Any QKV gradient alone is insufficient: separate Q/K routes must learn. Seed42/dropout0 bound the fixture, no training quality conclusion.

Positive norms admit all tensor gradients with same support/norm condition; zero Q/K is compatible with many inference input/weight states and does not reconstruct prior training.

## models

# Typed model catalog

## behavior_complement

Passing selected assertions excludes their stated counterexamples within the fixtures. No corpus/model-file mutation occurs in selected synthetic checks.

## reverse_reconstruction

Positive norms admit all tensor gradients with same support/norm condition; zero Q/K is compatible with many inference input/weight states and does not reconstruct prior training.

## gradient_support

QKV rows partition into chunks(cfg.d); Q,K are first 2*d rows.

## effect_boundary

Selected test process owns local fixture tensors/state; explicit CUDA tests serialize. No teacher/API, dataset or external model reads selected.

## assurance_scope

Compiled selected assertions establish only finite observations; ignored experiments and backend allocation failures remain gaps.

## failure_precedence

Fixture creation and backend operations must succeed before assertions; a compile-only result is distinct from executed assertions.

## tests

### Checks and limits

training_attention_keeps_gradients_and_inference_control_breaks_them

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/tests/gpu_gradients.rs

- training_attention_keeps_gradients_and_inference_control_breaks_them: twice debug, twice release, all four exits 0.

#### Controls

0 source-local compiled assertion mutants; 0 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

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

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

The synthetic Cfg literal gains ngram: 0 (the reference input); the check itself is unchanged.
