# algorithm-exact-cover

## descriptant

### algorithm-exact-cover

Preserve original coordinates, deterministic ties and distinct validation/resource/infeasibility outcomes. Repeated observations constrain this epoch only.

## model

# algorithm-exact-cover

## Domain

<=64 columns, <=4096 rows of distinct valid coordinates; finite u64 node budget; Option row-index witness or ResourceExhausted.

## Forward

Bitmask Algorithm X chooses minimum compatible-row-count uncovered column, ascending column and row ties; recurse only on disjoint rows; every visited state consumes budget.

## Invariant

Chosen row masks are pairwise disjoint and remaining is their complement in the full universe. Search success covers every column exactly once.

## Cost

O(budget*columns*rows) time; O(rows*columns) worst-case branch stack space.

## nonbehavior

Type: behavior_complement.

Exclude overlapping/incomplete witnesses, invalid row acceptance, None from interrupted search, or zero-budget success.

## preimages

Type: reverse_reconstruction.

For Some(indices), row families must partition all columns at those indices and deterministic branch order must select them within budget. Unselected rows and additional sufficient budget are generally unrecoverable. None requires exhausted complete search, not exhausted budget.

## transition

Type: specialized_relation.

Budget extension preserves successful witness or completed infeasibility, but exhausted runs may resolve either way.

## order

Type: specialized_relation.

Empty rows never participate; adding empty rows changes later original indices but not feasibility.

## coordinate

Type: specialized_relation.

Permuting columns preserves feasibility, but minimum-column tie decisions can change witness order.

## fiber

Type: specialized_relation.

Every recursive branch removes at least one bit, bounding recursion depth by 64 independent of row count.

## errors

Type: error_partition.

Public size caps precede input-order validation, then arithmetic/search. Exact cover budget exhaustion is not infeasibility; closure capacity overflow is not infeasibility. Bounded index arithmetic cannot overflow. Checked i128 arborescence arithmetic remains defensive. Allocation failure is process-level, not returned; no callbacks or external writes. Empty cases and deterministic ties follow API docs.

## Limits

Partial syntax, no macro expansion/type resolution or full formal proof. No justified generic numerical surrogate for categorical witnesses/errors; supported 1/2/5/10% profiles remain absent, not fabricated. No timing claims. Allocation failure untested.

## evidence

#### Evidence

All row-subset families on zero through three columns (278 cases); independent row-subset counting oracle; budget zero/one, 64-bit boundary and duplicate validation.

Two debug and two release runs, compiled mutation detected by budget_ties_and_boundaries, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: (see History). No timing measurement. Allocation failures and defensive i128 overflow unexercised.

## History

### 2026-10-30 · descriptant

Original std-only implementation or original independent fixtures. See source citations. Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/prospective.md.

### 2026-10-30 · evidence

#### Evidence

2026-10-30: All row-subset families on zero through three columns (278 cases); independent row-subset counting oracle; budget zero/one, 64-bit boundary and duplicate validation.

Two debug and two release runs, compiled mutation detected by budget_ties_and_boundaries, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30. No timing measurement. Allocation failures and defensive i128 overflow unexercised.
