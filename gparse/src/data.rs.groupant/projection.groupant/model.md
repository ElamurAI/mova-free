# gparse-data-projection

## descriptant

### Source-bound description

read/write project CoNLL-U to retained 8-column Word state and renumber on output.

## approximant

### Forward approximant

read/write project CoNLL-U to retained 8-column Word state and renumber on output.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Empty input produces no sentences; malformed heads become zero, comments-only blocks disappear.

#### Cost

O(input bytes) scanning/formatting plus retained words/comments.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

Range/decimal rows cannot enter Word output; invalid head does not produce an error.

## reverse_reconstruction

Ignored ID rows and DEPS/MISC cannot be reconstructed; all texts sharing retained words/comments are compatible.

## row_filter

c.len()<8 or ID contains '-' or '.' skips before indexing fields.

## empty_flush

Blank line emits only when words nonempty and always clears comments.

## serialization_projection

write assigns indices1..n and DEPS/MISC underscores; roles are omitted.

## head_parse

Malformed numeric head maps to zero, while parsed out-of-range head is preserved here.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): data::tests::features_line_up_with_words; data::cost_tests::cost_counts_real_batch_shapes_and_filters_skipped_sentences; data::wave30::projection_roles_and_zero_budget

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

read keeps column 10 as Word.misc (`_` when the row has only 8 or 9 columns); write is unchanged and still emits `_` for DEPS and MISC.
