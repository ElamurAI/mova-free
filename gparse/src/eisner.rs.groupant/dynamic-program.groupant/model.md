# gparse-eisner-dynamic-program

## descriptant

### Source-bound description

Span filling uses rounded f32 max-plus, stores earliest split for strict improvements, and selects one root. Exact-real maximum-tree interpretation requires exact arithmetic or candidate-order preservation.

## approximant

### Forward approximant

Span filling uses rounded f32 max-plus, stores earliest split for strict improvements, and selects one root. Exact-real maximum-tree interpretation requires exact arithmetic or candidate-order preservation.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Square scores with finite intermediates are needed for meaningful maximizing backpointers; f32 rounding can differ from real max-plus ordering.

#### Cost

O(n³) arithmetic, O(n²) charts/backpointers.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## models

# Typed model catalog

## behavior_complement

A later equal candidate cannot replace earlier maximizing split/root.

## reverse_reconstruction

A chosen split is compatible with candidate vectors whose rounded f32 comparisons select it; real-score inequality cones are valid only with order preservation. All -infinity candidates preserve fallback indices, which need not be first ascending.

## chart_base

Complete singleton spans initialized0, others -infinity.

## incomplete_join

Best C_left(s,r)+C_right(r+1,t) receives edge in either direction.

## root_constraint

Only final root choice adds sc[0][r], ensuring one root child on admitted domain.

## tie_order

For representable finite candidates, strict > and ascending r retain first maximum. If all candidates are -infinity, default a1=t can survive instead.

## tests

### Checks and limits

Selected parent checks relevant to this fragment (not exhaustive): eisner::tests::best_tree_with_one_root; eisner::wave30::empty_singleton_and_first_root_tie; decoder_accepts_no_words

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.
