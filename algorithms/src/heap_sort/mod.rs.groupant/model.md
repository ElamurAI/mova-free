# algorithm-heap-sort

## descriptant

### heap_sort mod groupant

Combiner: source identity, recurrence, invariant and checks constrain a
conditional model. Complement and converse answer different questions; no
additive inverse identity. Syntax is partial with opaque macros and no type
or name resolution. Finite evidence is not proof of all inputs or parent wiring.
Lawful Ord and successful execution are assumed; trait panics and allocation
failures are outside successful-call semantics. Preserve these observations
in future epochs. No runtime speedup claim.

## approximant

### Forward, invariant and cost

Build max heap bottom-up, swap root with shrinking suffix endpoint, then sift down.

Invariant: Active prefix is a max heap; finalized suffix is sorted and dominates prefix.

Cost: O(n log n) time, O(1) auxiliary space; lawful Ord and constant comparison/swap assumed.

## models

# Heap-sort typed models

Subject/revision: mod.rs at manifest SHA. Observe one normally terminating call
under lawful stable Ord. Models are partial source-derived specifications.
Trait panics and process/resource failures are outside successful-call semantics.

## behavior_complement

U consists of input/output slice pairs. The absent-outcome model includes every
output not ascending or with a different element multiset. This is an exact
specification complement on values with equality consistent with Ord.
Equal-key identity order is unrestricted. The larger-child-choice mutant is
rejected by exhaustive_bucket_oracle; bounded tests are not universal proof.

## reverse_reconstruction

For sorted y, compatible inputs include every permutation of its values.
With distinct values this is the exact sorting preimage; equal-key identities
retain ambiguity rather than a stability promise. Original order is not uniquely
recoverable. Bucket tests check the multiset projection, not full inverse search.

## suffix_extraction

Type: phase_invariant. After k extractions, the final k cells are ascending
and at least every active-prefix cell. Swap the max root into the next suffix
cell, then restore the prefix heap. This source-derived internal phase model
is not directly instrumented by output-only tests.

## order_equivariance

Type: order_isomorphism. An injective order-preserving map f satisfies
sort(f(x))=f(sort(x)) at value level. Reverse ordering changes the relation and
sorts numeric values descending. A custom non-Clone Reverse wrapper is an
executed changed-order witness. Arbitrary relabelings need not preserve order.

## resource_bound

Type: resource_model. Bottom-up building O(n), n extractions each O(log n),
total O(n log n) comparisons/swaps and O(1) auxiliary storage.
root<end/2 makes 2*root+1 valid without overflow. Costs assume constant
comparison/swap; expensive Ord multiplies comparison cost. No timing claim.

## application-transfers

### Application transfers

Declared consumers of this algorithm's modeled mechanism:

- heap-greedy-mentions: [coref/src/score.rs](../../../../coref/src/score.rs.groupant/model.md#transfer)

[Root transfer catalog](../../../../models/mova.groupant/algorithm-transfers.json). These are scoped mechanism transfers; original algorithm ownership and numerical contracts remain unchanged. Evidence status is recorded in the catalog.

## checks

### Evidence

Four tests for this algorithm passed twice with temporary local crate exports:
66 total tests, identical normalized outcomes. Strict all-target Clippy and
rustfmt passed. Temporary exports are not committed.

Coverage: 3280 ternary arrays length 0..7; 2053 full-width seeded u64 entries sorted repeatedly; empty/singleton/1024 equal entries.

External mutant (choose smaller child instead of larger child) compiled; exactly one filtered new exhaustive test
failed an assertion with exit 101. No mutant committed. One red control is not
mutation completeness. Parent owns final exports, registry and source bindings.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective predictions saved outside repository before
implementation. Original std-only code/fixtures; four algorithm tests pass
twice. Compiled red control: choose smaller child instead of larger child.

### 2026-10-29 · checks

Heading `### Evidence, 2026-10-29` stays in checks as `### Evidence`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
