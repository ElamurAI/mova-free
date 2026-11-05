# coref-groupant-transfers

## descriptant

# coref-groupant-transfers

## epoch-memory

The observations deliberately include a rejected transfer: the [[5,4],[4,0]] policy counterexample separates greedy matching from global assignment. Positive transfer uses static total keys to preserve the existing greedy selector and a distinct exact-integer sign adapter for assignment objectives. Installed 128/129 residual fixtures activate sorting; the 127 fixture retains scanning. The integer relation covers 81 two-by-two matrices and fractional rejection. All five tests passed four times. A wrong integer sign adapter failed and was restored exactly.

## model

# coref-groupant-transfers

## forward

Five independent supplied-input observations call actual match_mentions, assign_max and algorithm-library primitives. They reject a global-assignment replacement of greedy overlap matching, retain duplicate exact-span priority, inspect empty/negative/rectangular assignment boundaries, compare stable matching identities with sorted total keys, and compare exactly representable integer maximum objectives with negative-cost minimum assignment and a tiny enumerated oracle. Expanded residual groups cross the min-side-128 sorted-selector threshold; 127-side cases retain scanning.

## invariant

Each test keeps the operation and metric explicit. Greedy identity equality is stronger than objective equality and differs from optimal assignment. Exact matches precede residual selection. Tie keys order prediction before gold. Integer maximum/minimum comparisons share rectangular shape, exact conversion and sign convention; fractional fixtures are outside that adapter's admitted relation.

## cost

Harness cost sums actual matching and separate oracle work for each constructed case. Bounded sorted residuals cost O(PG log(PG)) with PG<=262144 and min(P,G)>=128; other matching cases retain scan cost O(PG min(P,G)). Assignment is O(r^2 c) and tiny 2x2 objective enumeration is finite. Test allocation and padding are harness work; no measured speed claim is encoded.

## composition

Test-observation joins only: actual matcher output identities are checked against explicit policy expectations and actual heap-sorted total keys; actual float maximum assignment objectives are checked against independent integer minimum-assignment and enumerated objectives on their common domain. assign_max is not installed inside the greedy matcher. The algorithm-library nodes remain in their existing hierarchy.

## checks

["groupant_reject_global_assignment_as_greedy_replacement","groupant_exact_match_priority_survives_large_residual_group","groupant_assignment_transpose_negative_and_empty_boundaries","groupant_total_key_zero_ties_join_heap_sort","groupant_integer_assignment_relation_and_noninteger_rejection"]

## limitations

Named tests are finite observations: residual sides 128/129 activate sorting, 127 retains scanning, and 81 integer 2x2 matrices exercise the assignment relation. Reservation-failure and overflow fallback may remain source-local rather than forced by this harness. Equality of scalar optimal values does not imply equality of tied assignment vectors. Nonfinite/ragged and malformed partition semantics remain outside this evidence.

## catalog

# Typed transfer relations

Source-derived models; native evidence pending.

## excluded-policy-substitutions

Type: behavior_complement.

The bounded observation universe excludes reporting the optimum 8 as the greedy result for the leading [[5,4],[4,0]] case, allowing residual overlap to displace exact matches, and reversing zero-overlap ties. For the integer adapter it excludes accepting an unrepresentable fractional matrix as an exact integer comparison. These are different tagged exclusions, not one shared optimizer specification.

## compatible-observations

Type: reverse_reconstruction.

A match vector admits many span layouts and overlap tables; an optimal scalar admits many matrices and tied assignments. Retain the policy, group ordering, dimensions and integer-domain tag before taking preimages. Padding zeros and unobserved unmatched mentions can vary without determining unique antecedents.

## greedy-counterexample

Type: policy_non_equivalence.

Zero-pad a residual group with leading weights [[5,4],[4,0]]. Actual greedy selection must preserve its first 5 edge and identity choices although global assignment can attain 8. Enlarging the group to enter sorting must preserve this rejected replacement.

Named observation: groupant_reject_global_assignment_as_greedy_replacement.

## exact-stage

Type: stage_priority.

Duplicate exact spans are consumed in their stable order before residual pairs are selected. Expanded residual groups test continuation into the bounded sorted branch without allowing already consumed gold or prediction identities to reappear.

Named observation: groupant_exact_match_priority_survives_large_residual_group.

## assignment-boundaries

Type: shape_and_state_obligation.

Actual assign_max is observed on empty, negative and rectangular/transposed cases. The maximum-objective relation includes negative weights and shape orientation; finite cases constrain reused scratch/reset behavior without proving all matrices.

Named observation: groupant_assignment_transpose_negative_and_empty_boundaries.

## zero-total-order

Type: tie_order_projection.

Actual heap_sort orders (Reverse(overlap),prediction_index,gold_index) keys, while actual match_mentions is checked for stable identities on zero-overlap ties. Both observations are required: a correctly sorted key list alone does not prove endpoint filtering or exact-match priority.

Named observation: groupant_total_key_zero_ties_join_heap_sort.

## integer-sign-adapter

Type: mutual_domain_obligation.

On exactly representable small integer matrices, max_weight(A) equals -min_cost(-A); compare actual float maximum, integer Hungarian minimum and independent 2x2 enumeration. Reject fractional input from this exact adapter domain. No float-API rejection or unrestricted integer/floating equivalence is implied.

Named observation: groupant_integer_assignment_relation_and_noninteger_rejection.

## evidence

# coref-groupant-transfers evidence

## checks (evidence)

See models/mova.groupant/model.md#transfer-evidence, transfer-benchmark.md and transfer-validation.md for the combined receipts. Source revisions are bound by this manifest and paired partial syntax. Finite checks do not establish universal equivalence, floating-point optimality, parser quality or whole-system speed.

## History

### 2026-10-30 · epoch-memory

2026-10-30 finite transfer integration evidence.

### 2026-10-30 · checks (evidence)

2026-10-30: 5 focused checks passed twice in debug and twice in release with identical normalized observations. Five named groupant_ observations. Residual 128/129 cases exercise sorting, 127 exercises fallback. Tests include greedy-versus-optimal [[5,4],[4,0]], duplicate exact-span priority, zero ties, 81 integer 2x2 matrices and fractional-adapter rejection. A sign-adapter mutant failed; exact restoration passed.
