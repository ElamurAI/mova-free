# math-steps-transition

## descriptant

### math-steps-transition

SHIFT increases next and stack length by1, then fix_leaf_vals restores qs[leaf].val; SKIP increases next only. REDUCE/FAR consumes two stack items and appends one operation tree, representative=max(source indices). legal precedes apply only in generated search.

## approximant

### Scoped forward relation

SHIFT increases next and stack length by1, then fix_leaf_vals restores qs[leaf].val; SKIP increases next only. REDUCE/FAR consumes two stack items and appends one operation tree, representative=max(source indices). legal precedes apply only in generated search.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Aligned arrays and valid direct replay actions are preconditions. n<=64 is needed for oracle shift-mask operations. Empty prediction has no final single-item stack.

#### Cost

At most3N+2 rounds expand up to20 actions per beam state, with feature generation, state/tree cloning and candidate sorting; ACTIONS counts expansions, not wall time or exact arithmetic work.

## models

## behavior_complement

Generated SHIFT cannot consume a forbidden/out-of-range input. Generated reduction cannot divide by zero or make negative subtraction. oracle cannot accept overlapping leaf masks. Direct replay is not covered by these exclusions and may panic on invalid actions.

## reverse_reconstruction

Two legal reduction schedules for (a+b)*(c+d) can produce the same tree; skipped unused quantities can differ while the resulting scalar agrees. Tree::Leaf0 and another equal-valued leaf can agree numerically without the same provenance.

## stack_accounting

For a valid prefix, next=#SHIFT+#SKIP and stack_len=#SHIFT-#REDUCE-#FAR. Every reduction preserves the union of operand source leaves; no reuse transition is present.

## oracle_reachability

oracle uses 64-bit masks and reduces only sibling subtrees within stack distance1..3. Disjoint leaf masks are necessary but do not alone guarantee a returned action sequence for arbitrary n/tree order.

## tests

### Scoped evidence

Oracle/replay and empty prediction only; no nonempty beam or training.

Named actual source fixtures: steps::wave28::{equation_oracle_and_exact_replay; calculator_validation_and_script_bounds}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.
