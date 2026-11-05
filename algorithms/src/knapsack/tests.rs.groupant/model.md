# algorithm-knapsack-tests

## descriptant

### knapsack test groupant

Combiner: source identity plus oracle construction plus assertion wiring form
a conditional finite-check model. Typed models answer separate questions; no
additive inverse identity is assumed. Syntax is partial, macros opaque.
A passing test does not establish parent wiring or full-domain correctness.

## approximant

### Forward, invariant and cost

Generate deterministic fixtures, compute the independently structured oracle,
compare production output, and fail on disagreement. Property loops hold their
listed assumptions fixed while changing order, padding, or a third sequence.
Invariant: each checked fixture has its reference computed without consulting
the output being tested; seeds advance by explicit wrapping arithmetic.
The model.md#models bounded_coverage_cost section gives algorithm-specific costs.
Assertion execution supplies bounded evidence; it does not prove the production
recurrence, all inputs, or source correctness from sidecar validity.

## models

# Knapsack test models

Subject: tests.rs at manifest SHA; observation: one test invocation. Assume
ordinary supported usize, successful small allocations, and fixed item values.
These are partial source-derived models with bounded runtime evidence.

## behavior_complement

Universe U=(items,C,proposed result,verdict). Let V be the subset-enumerated
maximum. Pass with result!=Ok(V) is absent from successful oracle assertions.
The exceptional dimension tests separately exclude successful allocation for
overflowing dimensions. This is an exact assertion-graph complement slice,
not a claim every wrong implementation is detected. A new larger-subset test
rejects ascending capacities, which permit repeated positive-weight items.

## reverse_reconstruction

For observed failing fixture/output, compatible implementations are all maps
producing that output there. Passing outputs retain all implementations equal
to V on the sampled domain. Several defects can share the same wrong value;
no unique defect or source reconstruction is implied. The compiled ascending
loop gives an executed member of the failure fiber.

## independent_oracle

Enumerate all masks of item indices, sum selected weights and u128 values,
then maximize feasible totals. No dynamic-programming row is reused.
Old tests check 44,286 cases. New twelve-item fixtures check 256 cases and
1,048,576 masks; capacity/permutation fixtures add 544 oracle cases and 139,264
masks. Bounded generated weights guarantee oracle usize sums cannot overflow.
The separate usize::MAX overweight witness uses a hand-derived result, avoiding
that oracle outside its safe-weight domain.

## property_assumptions

Permuting item indices preserves V and increasing capacity cannot decrease V,
provided values/weights are unchanged and each index is independently selectable
once. Reverse plus rotation exercises order changes; 32 eight-item lists at
capacities 0..16 check equality and adjacent monotonicity against subset values.
Zero-weight items are distinct selectable entries, including repeated values.

## bounded_coverage_cost

64 seeded twelve-item lists include zero weights and values near u64::MAX;
capacities are 0,1,7,15. A separate overweight usize::MAX item must be skipped.
The second family has 32 deterministic eight-item lists and 17 capacities.
Oracle work O(2^N*N), space O(N); DP O(N*(C+1)) time/O(C+1) space.
No huge allocation, exhaustive large-item search or allocator-failure injection.

## checks

### Evidence

Cargo crate library suite passed twice, single-threaded: 36 tests at this branch.
This file adds two tests to its preserved old suite. Strict Clippy on all crate
targets and cargo fmt --check passed. External scratch compiled a changed
production source with this exact test file; a filtered new test failed with
exit 101. KMP reset overlap fallback, Levenshtein doubled substitution cost,
and knapsack used ascending capacities. Only the respective mutant applies
to each file. No mutated code is committed. See model.md#models for exact coverage.

Repeated output comparison excludes Cargo build lines and elapsed-time footer.
Wall time is not an algorithm benchmark. New checks remain deterministic and
bounded, and no full input-space proof or allocation-failure coverage is claimed.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective expectations were saved outside the repository
before extraction. Existing inline tests were moved; two new tests extend
coverage. Production source prefix remained byte-identical. The new fixtures
passed twice and a specifically filtered NEW test rejected a compiled semantic
source mutant. Exact domains, assumptions and limitations are in model.md#models.

### 2026-10-29 · checks

Heading `### Evidence, 2026-10-29` stays in checks as `### Evidence`.
