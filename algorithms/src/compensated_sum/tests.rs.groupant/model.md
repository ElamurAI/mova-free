# algorithm-compensated-sum-tests

## descriptant

### compensated_sum tests.rs

Executable finite checks for compensated_sum: cancellation_oracle covers both magnitude branches and exact i64 sums for 100 finite integer fixtures; subnormal/empty/overflow fixtures.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

## model

## behavior_complement

For fixed fixtures, complement consists of assertion traces differing from the required exact errors, statuses or tolerance comparisons; implementations agreeing only here remain possible. A passing trace excludes the compiled mutant but not arbitrary bugs outside fixtures.

## reverse_reconstruction

Pass reconstructs the set of implementations consistent with these finite assertions; it does not identify one implementation. Untested inputs remain arbitrary. [1,2] and [3] both map to 3; zero-sum insertions preserve real totals but may change rounded traversal. The fiber contains many lengths/orders and cannot recover terms or correction state from final sum.

## oracle_independence

cancellation_oracle covers both magnitude branches and exact i64 sums for 100 finite integer fixtures; subnormal/empty/overflow fixtures.

## mutation_sensitivity

Replace final total+correction with total-correction; cancellation_oracle fails

## finite_scope

Finite fixture agreement supplies empirical observations, not a semantic proof or universal error cap. Different implementations with the same fixture outputs remain in the pass fiber.


## approximant

### Forward model and invariant

Executable finite checks for compensated_sum: cancellation_oracle covers both magnitude branches and exact i64 sums for 100 finite integer fixtures; subnormal/empty/overflow fixtures.

Invariant: all successful scalar results are finite; invalid input and nonfinite arithmetic stay distinct from values. Simpson pending panels partition the original interval, cached values match their abscissae and evaluations stay within budget. Tests preserve fixture independence and never infer universal accuracy from passing samples.

Cost: O(n) validation and reduction, O(1) state, no allocation.

Numerical scope: real formulas motivate f64 recurrences, not bitwise real semantics. No universal relative bound near zero/cancellation. No useful cheaper certified executable surrogate chosen; this remains an explicit gap.

## tests

### Evidence

cancellation_oracle covers both magnitude branches and exact i64 sums for 100 finite integer fixtures; subnormal/empty/overflow fixtures.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: independent polynomial/basis/integer fixtures and explicit finite-error partitions. Simpson hidden bump demonstrates limits of sampled estimates. An initial correction-denominator mutant survived tight-tolerance tests; a loose-tolerance quartic assertion was added to isolate the correction, and that mutant was then rejected. No speed claim; no percentages inferred from finite observations. Rust syntax projection is partial, without macro expansion, type resolution or semantic proof.

### 2026-10-30 · tests

Compiled production negative control: Replace final total+correction with total-correction; cancellation_oracle fails. Original bytes restored and named positive test rerun. Full algorithms library passes twice debug and twice release with identical sorted test-name/results; receipts bind logs under $MOVA_DATA/runs/octopus-wave19-numeric-2026-10-30. Source projection generated twice and compared before installation. Strict crate all-target Clippy attempted; outcome in receipt.
