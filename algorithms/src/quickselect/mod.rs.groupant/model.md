# algorithm-quickselect

## descriptant

### quickselect mod groupant

Combiner: source identity, recurrence, invariant and checks constrain a
conditional model. Complement and converse answer different questions; no
additive inverse identity. Syntax is partial with opaque macros and no type
or name resolution. Finite evidence is not proof of all inputs or parent wiring.
Lawful Ord and successful execution are assumed; trait panics and allocation
failures are outside successful-call semantics. Preserve these observations
in future epochs. No runtime speedup claim.

## approximant

### Forward, invariant and cost

Move midpoint pivot to last, partition strict-less values, retain side containing rank.

Invariant: Rank stays in shrinking interval; removed sides constrain selection and swaps preserve multiset.

Cost: O(n²) worst time, O(1) auxiliary space; invalid rank O(1).

## models

# Selection typed models

Subject/revision: mod.rs at manifest SHA. Observe complete successful calls
under lawful stable Ord. Trait panics are outside the successful-call model.
Partial source-derived claims have bounded evidence, not universal proofs.

## behavior_complement

U=(input x,rank k,output slice y,optional selected value r).
For k>=|x|, all outcomes except unchanged y=x,r=None are absent.
For valid k, exclude changed multiset, wrong sorted-rank value, prefix element
greater than y[k], or suffix element less than y[k]. This is the contract
complement; multiple valid within-partition arrangements remain allowed.
Reversed partition comparison fails exhaustive_all_ranks_and_partitions.

## reverse_reconstruction

Fix valid k and observed v. Compatible sequences contain v and satisfy
count(values<v)<=k<count(values<=v). Observing the full mutated slice further
fixes the multiset, without recovering original order. Invalid None with
unchanged y gives x=y and k>=|y|. Finite tests support these rank fibers;
complete preimage execution is not implemented.

## interval_progress

Type: decreasing_variant. The active interval contains k; removed sides
constrain the eventual selection. Move midpoint pivot to end, collect values
strictly less than pivot, then place pivot at boundary. A nonreturning step
removes pivot and one side, strictly decreasing interval length even on equal
values. Source reasoning supplies this model; equal-input tests exercise it.

## duplicate_rank_fiber

Type: equality_rank_interval. For fixed multiset with a values smaller than v
and e values equal to v, all k in [a,a+e) select an Ord-equivalent v. Fixed
input yields deterministic behavior, but no stable order or canonical identity
among distinguishable equal-key values is promised. Exhaustive ternary tests
exercise every rank and duplicate counts.

## resource_bound

Type: adversarial_cost. Each partition scans O(m) and removes at least one.
Sum of active lengths is O(n²); equal inputs at high ranks realize slow progress.
Auxiliary space O(1), no recursion/allocation. Invalid rank O(1), no swaps.
No randomized expected-linear claim or benchmark conclusion is made.

## checks

### Evidence

Four tests for this algorithm passed twice with temporary local crate exports:
66 total tests, identical normalized outcomes. Strict all-target Clippy and
rustfmt passed. Temporary exports are not committed.

Coverage: 3280 ternary arrays length 0..7, 24604 rank attempts including invalid endpoints; seeded 516 full-width entries and sorted/reverse/equal vectors.

External mutant (reverse strict partition comparison) compiled; exactly one filtered new exhaustive test
failed an assertion with exit 101. No mutant committed. One red control is not
mutation completeness. Parent owns final exports, registry and source bindings.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective predictions saved outside repository before
implementation. Original std-only code/fixtures; four algorithm tests pass
twice. Compiled red control: reverse strict partition comparison.

### 2026-10-29 · checks

Heading `### Evidence, 2026-10-29` stays in checks as `### Evidence`.
