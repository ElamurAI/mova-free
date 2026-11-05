# gparse-cli-training

## descriptant

### Source-bound description

Training composes filtered data, optional initialization, sampled plans, joint cross entropy and AdamW; saves strict best full-dev mean.

## approximant

### Forward approximant

Training composes filtered data, optional initialization, sampled plans, joint cross entropy and AdamW; saves strict best full-dev mean.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Quick probes do not select checkpoints; output files can be partially updated on later failure.

#### Cost

Epochs times sampled batch network/backward/AdamW plus dev inference; O(parameters+corpus+batch graph) memory.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Quick-dev proxy cannot select checkpoint when dev sets exist.

## reverse_reconstruction

Saved weights alone omit optimizer moments, sample RNG and checkpoint trajectory; antecedents are nonunique.

## schedule

warm=min(est,100 init else2000); total=est*epochs; lr floors decay at0.02.

## sampling

Always sentences included; others sample each epoch, first plan reused for epoch1.

## checkpoint

Strict mean>best and full_dev/no-dev guard; last epoch always full.

## partial_effect

Creates metadata before epochs, saves weights on improved epoch and rewrites log afterward; no transaction.

## tests

### Checks and limits

This exact fragment remains source-derived and unexecuted in this epoch. Parent/sibling selected evidence, not runtime coverage of this fragment: wave30::score_zip_and_plan_failure_precedence; train/eval/annotate external effects remain unexecuted

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

The loss now runs over keep()'s rows: arc loss on rows with VoteHead ≥ GP_PA, relation loss on rows with Vote ≥ GP_PA (all rows without GP_PA), tags on all rows; optional soft arc targets (GP_SOFT) and a random-subset control (GP_PA_CONTROL=random, its own xorshift seeded from GP_SEED). Batches come from batch_full with the model's ngram. A new model may take the subword input (GP_SUBWORD=1).
