# algorithm-exact-cover-tests

## descriptant

### algorithm-exact-cover-tests

Preserve original coordinates, deterministic ties and distinct validation/resource/infeasibility outcomes. Repeated observations constrain this epoch only.

## model

# algorithm-exact-cover-tests

## Domain

Finite fixture/assertion observations; trusted compiler and test runner; passing is not unrestricted equivalence.

## Forward

All row-subset families on zero through three columns (278 cases); independent row-subset counting oracle; budget zero/one, 64-bit boundary and duplicate validation.

## Invariant

Oracle is independently formulated; compare objective/feasibility plus witness invariants, not exact tie witness except named fixtures.

## Cost

O(budget*columns*rows) time; O(rows*columns) worst-case branch stack space.

## nonbehavior

Type: behavior_complement.

Accepted fixture/output pairs exclude mismatched oracle objectives, invalid witnesses and wrong tested errors. Outside-fixture behavior remains unconstrained.

## preimages

Type: reverse_reconstruction.

A pass reconstructs all implementations agreeing on the finite observations, including faulty unseen extensions. A failed assertion reconstructs unequal compared values at that fixture; it does not uniquely identify a source bug.

## oracle

Type: oracle_relation.

All row-subset families on zero through three columns (278 cases); independent row-subset counting oracle; budget zero/one, 64-bit boundary and duplicate validation.

## mutant

Type: counterfactual_relation.

Compiled production mutant must fail the named budget_ties_and_boundaries assertion; exact restoration followed by positive rerun.

## coverage

Type: coverage_relation.

Adding fixtures intersects the passing implementation set; profile repeats constrain profile-dependent bugs without proving universal correctness.

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
