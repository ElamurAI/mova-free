# dynamic-programming-optimization

## model

# dynamic-programming-optimization

## forward

Exact integer subset/knapsack objectives and structured matrix/partition/tree dynamic programming with explicit work and allocation bounds.

## invariant

Exact feasible objectives, caller structural assumptions, checked integer arithmetic and leftmost ties retain distinct contracts. Resource errors never masquerade as approximation.

## cost

Bitset O(n*ceil((T+1)/64)) packed work; MITM O(2^(n/2) log2^(n/2)); SMAWK O(rows+cols) callbacks; partition O(layers*n*log n); Knuth O(n²). Callback/allocations count and caps remain leaf-specific.

## composition

Weight=profit subset embedding links target reachability with attainable knapsack objectives. Totally-monotone identity minima transport coordinates into convex interval costs and positive BST weights in the harness.

## epoch-memory

Wave20. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["bitset_and_mitm_join_exhaustive_attainable_objectives","smawk_partition_and_knuth_join_independent_dp_oracles"]

## limitations

SMAWK every submatrix must have monotone leftmost minima; merely full-matrix monotone minima is insufficient. Partition monotone optima must hold at every layer. These properties are not verified dynamically. BST is successful-key-only objective.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

For exact integer targets, exclude a bitset true decision with no subset summing to target, a knapsack objective unattainable under capacity or below the feasible maximum, a SMAWK column that is not the leftmost row minimum, a partition value violating the minimum over exactly the requested nonempty contiguous intervals, or a BST value violating the successful-key interval recurrence. SMAWK/partition exclusions are conditional on total-monotonicity/monotone-split premises: arbitrary callbacks outside those premises may yield incorrect answers without validation errors. Native categorical exclusions also include overflow wraparound, violating word/state/cell or callback caps, and returning partial answers on budget failure. These integer targets do not carry f64 tolerances.

## reverse-reconstruction

Type: reverse_reconstruction.

A reachability true output retains all item multisets having at least one target-sum subset; false retains those having none. For [1,1] at target1, two distinct index masks share the same decision and knapsack objective, and objective-only output does not recover either witness. Fixed SMAWK minima j_i constrain each lawful row by M[i,j_i]<=M[i,j] and strict inequality for earlier j on ties; row offsets preserve minimizer indices while changing values. A partition scalar retains many interval-cost arrays and boundary choices, subject to nonempty contiguous ranges and structural premises; it does not reconstruct a split witness. BST cost similarly admits different weight vectors or tied optimal trees. Checked integer values are exact when successful under the stated premises, while an error identifies only a broad resource/domain fiber.

## subset-embedding

Type: attainability_objective_relation.

Embedding each v as weight=v,value=v gives feasible objectives equal to subset sums<=capacity. Decision(target) is true iff optimum(target)=target, provided both resource domains admit input. Many masks may give the same objective.

## monotone-reduction

Type: conditional_search_restriction.

SMAWK elimination is valid for total monotonicity with left ties; divide/conquer recursion restricts split search using nondecreasing leftmost optimal indices. Violating premises may silently yield wrong results, not a validation error.

## tree-recurrence

Type: interval_root_restriction.

For nonnegative successful-key weights, C[i,j]=sum(w[i..j])+min_r(C[i,r]+C[r+1,j]); Knuth monotone root bounds reduce cubic root enumeration to quadratic. Equal costs can retain multiple trees; harness compares independent full enumeration.

## wave20-evidence

### Family evidence

This family binds ten implementation/test sources. Named joins ["bitset_and_mitm_join_exhaustive_attainable_objectives","smawk_partition_and_knuth_join_independent_dp_oracles"] in algorithms/tests/composition_wave20.rs pass four times within the 695-check algorithm suite.

Forward: Exact integer subset/knapsack objectives and structured matrix/partition/tree dynamic programming with explicit work and allocation bounds.

Invariant: Exact feasible objectives, caller structural assumptions, checked integer arithmetic and leftmost ties retain distinct contracts. Resource errors never masquerade as approximation.

Cost: Bitset O(n*ceil((T+1)/64)) packed work; MITM O(2^(n/2) log2^(n/2)); SMAWK O(rows+cols) callbacks; partition O(layers*n*log n); Knuth O(n²). Callback/allocations count and caps remain leaf-specific.

Assumptions: SMAWK every submatrix must have monotone leftmost minima; merely full-matrix monotone minima is insufficient. Partition monotone optima must hold at every layer. These properties are not verified dynamically. BST is successful-key-only objective.

[Integrated evidence](../algorithm-library.groupant/model.md#wave20-evidence) records actual receipts. Leaf oracles remain separate from cross-implementation agreement; family membership cannot verify unchecked callback premises or a shared error budget.

## History

### 2026-10-30 · epoch-memory

Wave20, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave20-evidence

Heading `### Family evidence — 2026-10-30` stays in wave20-evidence as `### Family evidence`.
