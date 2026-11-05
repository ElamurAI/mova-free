# gparse-data-lexical

## descriptant

### Source-bound description

Lex counts (form,upos,xpos)->(feats,lemma), tries EModE before counts and preserves informative old verb lemma when morphology repeats form.

## approximant

### Forward approximant

Lex counts (form,upos,xpos)->(feats,lemma), tries EModE before counts and preserves informative old verb lemma when morphology repeats form.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Equal count keys choose smaller; unknown relations reduce base or dep unless vocabulary empty.

#### Cost

O(tokens*log distinct keys) learning and map lookup plus delegated morphology/table work.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Known table result cannot be superseded by corpus frequencies.

## reverse_reconstruction

Chosen lemma/features admit all count maps with same winning key, and all corpora yielding those maps.

## precedence

fill chooses EModE table, form+tag top, then tag top/morphology.

## tie_policy

Equal frequency chooses lexicographically smaller key through reversed key comparator.

## lemma_retention

Verb/AUX retains informative old lemma when new lowercase lemma repeats lowercase form.

## relation_fallback

Empty relation set accepts r; otherwise exact r, known colon base, then dep.

## tests

### Checks and limits

This exact fragment remains source-derived and unexecuted in this epoch. Parent/sibling selected evidence, not runtime coverage of this fragment: data::tests::features_line_up_with_words; data::cost_tests::cost_counts_real_batch_shapes_and_filters_skipped_sentences; data::wave30::projection_roles_and_zero_budget

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
