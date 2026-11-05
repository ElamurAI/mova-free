# algorithm-knapsack

## descriptant

### Algorithm file groupant

This is the original standalone knapsack implementation. The authoritative Rust
source lives outside this sidecar. Forward recurrence, invariant and cost are
in model.md#approximant; five typed models are in model.md#models. The manifest binds the
current source hash and partial syntax projection. Syntax extraction does not
expand macros, resolve names or prove semantic correctness.

Composition: conjunction of the source identity, recurrence and invariant gives
a conditional implementation model; typed models answer distinct questions
about that relation. Their statements do not add to a global inverse identity.
Cost and tests qualify the model, rather than proving a larger caller's wiring.

## approximant

### Forward, invariant and cost

Initialize every capacity to zero. For each item (w,v), traverse c from C down
to w and assign best[c]=max(best[c],best[c-w]+v).
Invariant after processing k items: best[c] is the exact best subset value
using indices <k and weight<=c. For w>0 descending order reads only the prior
item stage; for w=0 each cell reads itself before its one write, adding v once.
Every accumulated total fits u128 for actual slices on supported usize<=64-bit
targets. Overflowing row dimensions fail before allocation.

Worst-case time O(N*(C+1)+C), auxiliary space O(C+1). Overweight items are
skipped; dimension checks are O(1); allocator refusal returns AllocationFailed.
This is pseudo-polynomial in numeric capacity, not polynomial in its bit length.
No huge-capacity allocation or runtime speedup experiment was performed.

## models

# Knapsack models

Subject/revision: mod.rs SHA-256 in manifest.json. Observation is one call
on finite item sequences and capacity C. Values are u64, weights and C usize.
Successful allocation is assumed for successful-result relations; usize has
at most 64 bits, so a slice's total value fits u128.

## behavior_complement

Universe U: (items,C,result), including errors and u128 results. Define
V(items,C)=max {sum values(S) | S subset of item indices, sum weights(S)<=C}.
Within representable, successfully allocated rows, every Ok(v) with v!=V
belongs to the specification complement. When C+1, row byte size or isize
allocation bound is exceeded, every Ok result is excluded and the specified
result is CapacityOverflow. This is a scoped exact specification complement,
hypothesized inner bound for actual absent behavior. Allocator failures are
not predicted by the value model. Oracles: exhaustive_subsets, dimension test;
ascending-capacity compiled mutant must fail.

## reverse_reconstruction

Fix C and observed Ok(v). Compatible item sequences satisfy V(items,C)=v,
equivalently every feasible subset value<=v and at least one equals v.
Recovering one maximizing subset does not recover the original items.
Enumerating bounded sequences and filtering with the subset oracle gives
complete preimages in that finite domain. Outside it no completeness claim.
Evidence: exhaustive_subsets; explicit preimage enumeration remains future.

## zero_weight_shift

Type: additive_extension. Append a distinct zero-weight item with value q.
On successful allocation V(items+[item],C)=V(items,C)+q. Each item is selected
at most once, including q=0. Repeated equal entries are separate indices.
Oracle: two zero-weight u64::MAX items at capacity zero yield twice that value;
exhaustive_subsets includes mixed zero/positive weights.

## capacity_order

Type: resource_monotonicity. For fixed items and C1<=C2,
V(items,C1)<=V(items,C2), whenever both rows allocate. The intervention grows
the feasible subset set. Equality is permitted; no additional item copies
become available. Exhaustive_subsets checks values at capacities 0..5,
and the one-item capacity-four witness excludes unbounded reuse.

## dimension_preimage

Type: first_failure_partition. Let w=C+1 and B=16w in mathematical integers.
CapacityOverflow antecedents are exactly C+1>usize::MAX, B>usize::MAX or
B>isize::MAX, before consulting item contents or attempting allocation.
Passing these guards is necessary, not sufficient, for Ok: reservation can fail.
The dimension test executes three boundary representatives without allocating
large rows. AllocationFailed has no exact environmental preimage model here.

## tests

### Checks and limits

The neighboring source contains bounded independent reference-oracle tests.
A copied source outside the repository was mutated, compiled, and run against
the unchanged tests; nonzero exit was required and observed. No production
source mutation remains. Source was formatted before syntax projection and
has not changed since passing checks. Full-workspace integration is the
parent groupant's responsibility. No exhaustive machine-domain proof is claimed.

Three tests pass: 7,381 sequences of up to four items from nine weight/value
pairs, each at six capacities = 44,286 subset-oracle comparisons. Separate tests
cover zero-weight wide sums and three overflow dimensions without huge allocation.
Mutant changes descending capacities to ascending and fails reuse-sensitive tests.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective expectations were saved outside the repository
before writing source. Independent finite oracles passed twice. A compiled
source mutation failed the same suite, demonstrating a red gate. These bounded
observations support the recurrence; they do not prove all inputs. Resource
exhaustion, expensive element comparisons and complete preimage execution are
explicit gaps. Preserve this evidence when extending the implementation.

Test extraction epoch, 2026-10-29: inline tests moved to sibling tests.rs with
their own source-bound groupant and registry entry; this module delegates through
a cfg(test) declaration. Production prefix remains byte-identical. Two new tests
extend bounded coverage; no production fix or timing gain is claimed.

### 2026-10-29 · tests

2026-10-29: compile independently with
`rustc --edition 2021 --test --crate-name mova_algorithm_knapsack algorithms/src/knapsack/mod.rs -o TEST_BINARY`.
Run TEST_BINARY twice with --test-threads=1. Both runs passed; test case outcomes
are deterministic. Elapsed-time footer is observational and may differ.

2026-10-29 expansion: old tests moved unchanged to ../tests.rs; two new tests
add broader independent-oracle and property coverage. See ../tests.rs.groupant/
for exact scope, costs and compiled negative-control evidence. Production prefix
was compared byte-for-byte with the pre-extraction snapshot and remains equal.
