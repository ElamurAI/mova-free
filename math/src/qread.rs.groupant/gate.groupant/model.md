# math-qread-gate

## descriptant

### math-qread-gate

abstain returns the first reason in this order: absent value, absent expression, unread values, v<=1e-9, noninteger distance>1e-6, coefficient magnitude not near1, insufficient source occurrences/nonzero constant, duplicate source occurrences, unused question numbers, unused all_used numbers.

## approximant

### Scoped forward relation

abstain returns the first reason in this order: absent value, absent expression, unread values, v<=1e-9, noninteger distance>1e-6, coefficient magnitude not near1, insufficient source occurrences/nonzero constant, duplicate source occurrences, unused question numbers, unused all_used numbers.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Gate checks provenance and selected numerical predicates, not truth of the parsed story. Accepted finite output has>=2 flattened occurrences and near-unit coefficient magnitudes.

#### Cost

Flattening and sorting provenance dominates duplicate checks; required-number membership uses repeated Vec::contains, so it can be quadratic in occurrences.

## models

## behavior_complement

For valid indices and finite observations, accepting with unread nonempty, v<=1e-9 or a coefficient failing magnitude tolerance is excluded. Later predicates cannot replace an earlier returned reason. This does not exclude all NaN acceptance.

## reverse_reconstruction

The same 'unread numbers' output admits a positive integer answer or zero because unread is checked earlier. Accepted12 can arise from compound4*3 or direct sum5+7 with two distinct occurrences; acceptance does not recover the expression or source values.

## partitioned_reuse

When all_used=false, any duplicate flattened occurrence rejects. When true, duplicates are checked separately among compound leaves and simple leaves; a count used once in each partition may pass.

## required_occurrences

Every index in q_leaf0..nums must occur in the answer provenance. Only when all_used=true must all0..nums occur; irrelevant body-number omission otherwise is not rejected by this predicate.

## tests

### Scoped evidence

Gate precedence, positivity/integrality/coefficient reuse and compound provenance; not all-used duplicate partitions.

Named actual source fixtures: qread::wave28::{affine_provenance_and_gate_order; empty_query_and_compound_provenance}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

#### Wave34 direct-helper refinement

Earlier source-only gaps and wave28 receipts above are historical. Current descendants add actual helper fixtures; see ../model.md#wave34-evidence and each descendant's Tests section. Full English parsing and general solver correctness remain gaps.

## wave34-composition

# Gate descendant composition

The occurrence-partitions child is reached only after answer/expression presence, unread-number, positivity, integrality and coefficient predicates pass. It first checks flattened cardinality/constant, then chooses global or partitioned duplicate detection based on all_used. Required question occurrences precede the full-body requirement. It returns its first failure or acceptance to abstain; it does not evaluate the expression or verify the supplied value against leaves.

Compatible accepted antecedents therefore require the earlier numerical predicates and valid provenance indices as well as child membership. A compound count reused once outside the compound can pass in all_used mode; this does not permit reuse twice inside one partition. Earlier failures dominate any provenance result. Nonfinite public inputs remain outside the finite numerical interpretation.

## Cost

Earlier predicates scan coefficient values; this child flattens and sorts occurrence lists and performs repeated required-index membership. O(S log S + nums*S) upper accounting with O(S) temporary storage, plus coefficient scan. No optimization or measured throughput is asserted.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.
