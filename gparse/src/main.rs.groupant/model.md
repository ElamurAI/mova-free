# gparse-cli

## descriptant

### Source-bound description

CLI dispatches train/eval/annotate/plan by arguments; plan parses positive budget before cfg and corpus. score counts only zipped sentence/token pairs, LAS additionally requires matching head, percentages divide max(n,1). train validates dev schedule, reads/selects corpora, optionally loads vocab/config/weights, allocates optimizer, writes model metadata, then runs shuffled batches with joint loss and warmup/decay. Full-dev strict improvements save weights; quick proxies cannot select checkpoint. annotate predicts, updates score comments, refills lexical fields and postverbal-not corrections then writes output.

## approximant

### Forward approximant

CLI dispatches train/eval/annotate/plan by arguments; plan parses positive budget before cfg and corpus. score counts only zipped sentence/token pairs, LAS additionally requires matching head, percentages divide max(n,1). train validates dev schedule, reads/selects corpora, optionally loads vocab/config/weights, allocates optimizer, writes model metadata, then runs shuffled batches with joint loss and warmup/decay. Full-dev strict improvements save weights; quick proxies cannot select checkpoint. annotate predicts, updates score comments, refills lexical fields and postverbal-not corrections then writes output.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Training effects are nontransactional; partial metadata/checkpoint/log writes may survive errors. Score empty=all zero and truncation is intentional source behavior, not aligned-input verification. Default learning loss arc+rel+half(xpos+upos). No live training selected.

#### Cost

Corpus bytes and network inference/training dominate; plan O(S log S); scoring O(min paired tokens); optimizer state O(parameters), batch BT² and gradients. Epoch estimates can differ after sampling; no timing certificate.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

CLI dispatches train/eval/annotate/plan by arguments; plan parses positive budget before cfg and corpus. score counts only zipped sentence/token pairs, LAS additionally requires matching head, percentages divide max(n,1). train validates dev schedule, reads/selects corpora, optionally loads vocab/config/weights, allocates optimizer, writes model metadata, then runs shuffled batches with joint loss and warmup/decay. Full-dev strict improvements save weights; quick proxies cannot select checkpoint. annotate predicts, updates score comments, refills lexical fields and postverbal-not corrections then writes output.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Training effects are nontransactional; partial metadata/checkpoint/log writes may survive errors. Score empty=all zero and truncation is intentional source behavior, not aligned-input verification. Default learning loss arc+rel+half(xpos+upos). No live training selected.

Scores constrain counts over zipped pairs, not full dataset size; many unpaired suffixes and correct/incorrect arrangements give same percentages. Saved checkpoint is compatible with many earlier epochs below best mean and does not reveal all optimizer/RNG state.

## models

# Typed model catalog

## behavior_complement

Zero or malformed plan budget excludes any cfg/corpus open. Incorrect head excludes LAS even if label matches. Proxy dev epoch cannot replace best checkpoint unless no dev files; equal full mean does not save. Annotation cannot preserve original raw formatting.

## reverse_reconstruction

Scores constrain counts over zipped pairs, not full dataset size; many unpaired suffixes and correct/incorrect arrangements give same percentages. Saved checkpoint is compatible with many earlier epochs below best mean and does not reveal all optimizer/RNG state.

## failure_precedence

Train dev_every/dev_quick validation precedes file reads. plan budget validation precedes cfg. annotate loads model before input/roles; output write follows all sentence edits.

## checkpoint_selection

full_dev=(ep%dev_every==0 || ep==epochs); save iff (devs empty || full_dev) and mean>best. Empty dev set uses ep as mean.

## schedule_state

lr_now=lr*min(step/warm,1)*max(1-step/total,0.02); total uses initial sampled batch count times epochs, not later exact counts.

## metric_projection

UAS/LAS/XPOS/UPOS percentages over zipped pairs can hide length mismatches; blank sets divide by one.

## tests

### Checks and limits

wave30::score_zip_and_plan_failure_precedence; train/eval/annotate external effects remain unexecuted

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/src/main.rs

- wave30::score_zip_and_plan_failure_precedence: twice debug, twice release, all four exits 0.

#### Controls

1 source-local compiled assertion mutants; 1 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Only score and plan invalid/zero-budget helpers; live train/eval/annotate file effects unexecuted.

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

train reads GP_SUBWORD (0/1; a new model takes ngram=GP_NGRAM_BUCKETS, default 2^18; a continued model must agree with its saved ngram or train fails), GP_PA (a share in [0,1]), GP_SOFT (0/1) and GP_PA_CONTROL=random (needs GP_PA). keep() selects the arc-loss rows (VoteHead ≥ threshold) and relation-loss rows (Vote ≥ threshold); without GP_PA, all rows. The losses are computed over the kept rows only; an empty set contributes a zero scalar. GP_SOFT replaces the arc cross entropy by −mean Σ q·log softmax with q from soft_targets (gold head share plus alternatives, normalized). With GP_PA set, a `partial N:` line per epoch reports kept shares. eval prints, after the unchanged overall line, indented UAS/LAS by length bucket (1–10, 11–20, 21–40, 41+), seen/unseen word (model vocabulary) and GUM genre (sent_id GUM_<genre>_); GP_EVAL_SCRAMBLE=1 scrambles the letters of unseen words by a word-seeded Fisher–Yates permutation before prediction.

### 2026-11-04 · behavior_complement

Since 2026-11-04: GP_PA=0 and GP_PA unset never drop a row (measured: byte-identical model.safetensors after a 2-epoch CUDA continuation, 2026-11-04); tags never lose supervision; a word without a vote never leaves a loss; the overall eval line never changes format (the Eel's parser reads it), and breakdown lines never start with the file path.

### 2026-11-04 · reverse_reconstruction

Since 2026-11-04: a breakdown row (words, UAS, LAS) is compatible with every per-word correctness assignment of that count; the random control's kept set is compatible with every same-size subset, recoverable only from its seeded generator.

### 2026-11-04 · tests

Since 2026-11-04: wave30::breakdown_by_length_vocabulary_and_genre, wave30::partial_annotation_keeps_and_drops (threshold 0 equals none; random control nested and reproducible; soft targets), wave30::scramble_is_a_fixed_permutation.

### 2026-11-04 · evidence

#### Execution evidence — 2026-11-04

cargo test --release -p gparse (CPU tests pass; CUDA/model tests ignored without their variables). CUDA runs in an external run directory: two identical 2-epoch continuations gave byte-identical weights; GP_PA=0 gave the same weights as no GP_PA, and both the same as the gparse built from main before this change (red controls). Dev numbers are in the worklog, not here.
