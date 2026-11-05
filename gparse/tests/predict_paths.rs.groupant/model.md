# gparse-test-predict-paths

## descriptant

### Source-bound description

New synthetic CPU Net d=8/one layer predicts empty, singleton and max_len=4 fallback; ordinary and scored outputs must match, singleton root label forced, fallback original tuples and zero scores retained. Original ignored GPU experiment loads external model/corpus and repeats six alternating rounds; not run.

## approximant

### Forward approximant

New synthetic CPU Net d=8/one layer predicts empty, singleton and max_len=4 fallback; ordinary and scored outputs must match, singleton root label forced, fallback original tuples and zero scores retained. Original ignored GPU experiment loads external model/corpus and repeats six alternating rounds; not run.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Synthetic scores finite; empty and overlength zero, original order retained. Exact selected tuple equality is not trained-model corpus accuracy.

#### Cost

Small CPU encoder plus O(1³) decode and fixture setup; original experiment scales with external sentence limit.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

New synthetic CPU Net d=8/one layer predicts empty, singleton and max_len=4 fallback; ordinary and scored outputs must match, singleton root label forced, fallback original tuples and zero scores retained. Original ignored GPU experiment loads external model/corpus and repeats six alternating rounds; not run.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Synthetic scores finite; empty and overlength zero, original order retained. Exact selected tuple equality is not trained-model corpus accuracy.

Fallback output admits input trees exactly copied at lengths>=max_len; short discrete predictions admit many parameter/logit states. Timing medians cannot reconstruct per-run values.

## models

# Typed model catalog

## behavior_complement

Passing selected assertions excludes their stated counterexamples within the fixtures. External saved-model/corpus experiment remains unexecuted; success of helper checks cannot establish it.

## reverse_reconstruction

Fallback output admits input trees exactly copied at lengths>=max_len; short discrete predictions admit many parameter/logit states. Timing medians cannot reconstruct per-run values.

## fallback_boundary

len==max_len takes fallback; len<max_len goes through decoder, empty skipped.

## effect_boundary

Selected test process owns local fixture tensors/state; explicit CUDA tests serialize. No teacher/API, dataset or external model reads selected.

## assurance_scope

Compiled selected assertions establish only finite observations; ignored experiments and backend allocation failures remain gaps.

## failure_precedence

Fixture creation and backend operations must succeed before assertions; a compile-only result is distinct from executed assertions.

## tests

### Checks and limits

synthetic_cpu_prediction_preserves_slots_and_fallback

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/tests/predict_paths.rs

- synthetic_cpu_prediction_preserves_slots_and_fallback: twice debug, twice release, all four exits 0.

#### Controls

0 source-local compiled assertion mutants; 1 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Only synthetic CPU fixture; original external model/corpus timing experiment not run.

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
