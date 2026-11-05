# gparse-test-empty-sentences

## descriptant

### Source-bound description

decoder_accepts_no_words calls actual decoder root-only and expects empty. The ignored GPU test loads GP_TEST_MODEL, compares mixed empty/nonempty parse_forms with standalone predictions and asserts empty scored outputs.

## approximant

### Forward approximant

decoder_accepts_no_words calls actual decoder root-only and expects empty. The ignored GPU test loads GP_TEST_MODEL, compares mixed empty/nonempty parse_forms with standalone predictions and asserts empty scored outputs.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

No words means no decoder heads. Saved-model CUDA branch checks outer position retention, but is unexecuted here.

#### Cost

O(1) CPU check; ignored GPU path includes model loading and repeated inference.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

decoder_accepts_no_words calls actual decoder root-only and expects empty. The ignored GPU test loads GP_TEST_MODEL, compares mixed empty/nonempty parse_forms with standalone predictions and asserts empty scored outputs.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

No words means no decoder heads. Saved-model CUDA branch checks outer position retention, but is unexecuted here.

Successful empty assertion is compatible with any one-row score input; GPU trees do not recover original weights or tokenization.

## models

# Typed model catalog

## behavior_complement

Passing selected assertions excludes their stated counterexamples within the fixtures. External saved-model/corpus experiment remains unexecuted; success of helper checks cannot establish it.

## reverse_reconstruction

Successful empty assertion is compatible with any one-row score input; GPU trees do not recover original weights or tokenization.

## empty_boundary

[[],words,[],words] must retain four slots in ignored GPU check.

## effect_boundary

Selected test process owns local fixture tensors/state; explicit CUDA tests serialize. No teacher/API, dataset or external model reads selected.

## assurance_scope

Compiled selected assertions establish only finite observations; ignored experiments and backend allocation failures remain gaps.

## failure_precedence

Fixture creation and backend operations must succeed before assertions; a compile-only result is distinct from executed assertions.

## tests

### Checks and limits

decoder_accepts_no_words; GPU empty saved-model case excluded

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/tests/empty_sentences.rs

- decoder_accepts_no_words: twice debug, twice release, all four exits 0.

#### Controls

0 source-local compiled assertion mutants; 0 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Original GPU saved-model case not run.

CUDA softmax comparisons use absolute tolerance2e-6. GELU grid maxima observed: values0.0004734993, derivatives0.00086838007, finite difference0.0000005534244635896357; respective strict caps5e-4/1e-3/1e-5. These are finite observations, not percentage profiles or throughput claims.

Receipts: external wave30 gparse run native-commands.json, results.json, controls.json and source-test-map.json. Sorted exact test names/outcomes match per profile; numeric diagnostics and timings need not be byte-identical. Strict scoped library Clippy reports ten pre-existing production lints; appended test blocks were formatted independently without production reformatting.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-10-31 · evidence

Heading `### Execution evidence — 2026-10-31` stays in evidence as `### Execution evidence`.
