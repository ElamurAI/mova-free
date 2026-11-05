# gparse-test-inference-softmax-comparison

## descriptant

### Source-bound description

compare checks outer sentence count, then each paired token count, counts unequal sentence vectors and unequal tuples, returns (sentences,tokens).

## approximant

### Forward approximant

compare checks outer sentence count, then each paired token count, counts unequal sentence vectors and unequal tuples, returns (sentences,tokens).

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Aligned identical tuple sequences return (0,0); one tag change returns (1,1).

#### Cost

O(paired tuples plus compared string bytes), O(1) auxiliary counters.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Shape mismatches cannot silently count as zero: outer or inner assert panics.

## reverse_reconstruction

Counts admit every placement and kind of tuple mismatch consistent with the two totals.

## shape_precedence

Outer length assertion occurs before loop; inner length assertion occurs before counting that sentence.

## tuple_metric

Head, relation, XPOS or UPOS difference each contributes one token, not one per field.

## sentence_metric

Multiple changed tokens in one sentence add one to sentence counter.

## effect_boundary

Pure borrowed comparisons; no CUDA or environment changes.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): compare_counts_tuple_changes_and_shape_errors

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
