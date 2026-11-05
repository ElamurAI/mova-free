# gparse-test-training-speed

## descriptant

### Source-bound description

Actual step helper builds arc/rel/tag cross-entropies, loss=arc+rel+0.5*(xpos+upos), backpropagates, optionally checks finite positive norms of word/QKV/arc/rel weights, calls AdamW step and returns pre-update loss. New tiny seeded CUDA single-step test checks positive finite loss, changed arc_w and finite updated tensor. Existing measured_optimizer_steps requires real model/corpus and is not run.

## approximant

### Forward approximant

Actual step helper builds arc/rel/tag cross-entropies, loss=arc+rel+0.5*(xpos+upos), backpropagates, optionally checks finite positive norms of word/QKV/arc/rel weights, calls AdamW step and returns pre-update loss. New tiny seeded CUDA single-step test checks positive finite loss, changed arc_w and finite updated tensor. Existing measured_optimizer_steps requires real model/corpus and is not run.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Only ephemeral VarMap/optimizer state changes. One changed tensor does not certify all optimizer moments or convergence; no throughput claim. Original experiment permits <=128 sentences of lengths24..40, 88 steps and trajectory delta<0.005*scale, but excluded.

#### Cost

One bounded tiny Net step with O(parameters) AdamW state; existing experiment much larger and not authorized/executed.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

Actual step helper builds arc/rel/tag cross-entropies, loss=arc+rel+0.5*(xpos+upos), backpropagates, optionally checks finite positive norms of word/QKV/arc/rel weights, calls AdamW step and returns pre-update loss. New tiny seeded CUDA single-step test checks positive finite loss, changed arc_w and finite updated tensor. Existing measured_optimizer_steps requires real model/corpus and is not run.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Only ephemeral VarMap/optimizer state changes. One changed tensor does not certify all optimizer moments or convergence; no throughput claim. Original experiment permits <=128 sentences of lengths24..40, 88 steps and trajectory delta<0.005*scale, but excluded.

Given returned loss, all pre-update logits with same cross-entropy and compatible labels are antecedents; post-step weights cannot recover moments or gradients without optimizer state.

## models

# Typed model catalog

## behavior_complement

Passing selected assertions excludes their stated counterexamples within the fixtures. No real-model/corpus optimizer experiment was executed.

## reverse_reconstruction

Given returned loss, all pre-update logits with same cross-entropy and compatible labels are antecedents; post-step weights cannot recover moments or gradients without optimizer state.

## optimizer_effect

Inspect happens before opt.step; failure in norm assertion prevents update. Returned loss is computed before update.

## effect_boundary

Selected test process owns local fixture tensors/state; explicit CUDA tests serialize. No teacher/API, dataset or external model reads selected.

## assurance_scope

Compiled selected assertions establish only finite observations; ignored experiments and backend allocation failures remain gaps.

## failure_precedence

Tensor/loss/backward failures precede optimizer update; inspect assertions also precede update.

## tests

### Checks and limits

synthetic_single_optimizer_step

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/tests/training_speed.rs

- synthetic_single_optimizer_step: twice debug, twice release, all four exits 0.

#### Controls

1 source-local compiled assertion mutants; 1 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Only synthetic_single_optimizer_step using actual step helper; measured_optimizer_steps explicitly not run.

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
