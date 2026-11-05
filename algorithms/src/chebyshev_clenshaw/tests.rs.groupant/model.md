# algorithm-chebyshev-clenshaw-tests

## descriptant

### chebyshev_clenshaw tests.rs

Executable finite checks for chebyshev_clenshaw: basis_oracle uses forward Chebyshev recurrence for degrees 0..19 at 33 dyadic points; endpoint, empty, nonfinite and cancellation fibers.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

## model

## behavior_complement

For fixed fixtures, complement consists of assertion traces differing from the required exact errors, statuses or tolerance comparisons; implementations agreeing only here remain possible. A passing trace excludes the compiled mutant but not arbitrary bugs outside fixtures.

## reverse_reconstruction

Pass reconstructs the set of implementations consistent with these finite assertions; it does not identify one implementation. Untested inputs remain arbitrary. At x=0, p=c0-c2+c4-... while odd coefficients are arbitrary: [2,3] and [2,9] both return 2. General real fibers are affine hyperplanes in coefficient space; rounding enlarges or moves fibers. Neither degree nor coefficients uniquely reconstruct.

## oracle_independence

basis_oracle uses forward Chebyshev recurrence for degrees 0..19 at 33 dyadic points; endpoint, empty, nonfinite and cancellation fibers.

## mutation_sensitivity

Change recurrence -b2 to +b2; basis_oracle fails

## finite_scope

Finite fixture agreement supplies empirical observations, not a semantic proof or universal error cap. Different implementations with the same fixture outputs remain in the pass fiber.


## approximant

### Forward model and invariant

Executable finite checks for chebyshev_clenshaw: basis_oracle uses forward Chebyshev recurrence for degrees 0..19 at 33 dyadic points; endpoint, empty, nonfinite and cancellation fibers.

Invariant: all successful scalar results are finite; invalid input and nonfinite arithmetic stay distinct from values. Simpson pending panels partition the original interval, cached values match their abscissae and evaluations stay within budget. Tests preserve fixture independence and never infer universal accuracy from passing samples.

Cost: O(n) validation plus O(n) recurrence; O(1) space, no allocation.

Numerical scope: real formulas motivate f64 recurrences, not bitwise real semantics. No universal relative bound near zero/cancellation. No useful cheaper certified executable surrogate chosen; this remains an explicit gap.

## tests

### Evidence

basis_oracle uses forward Chebyshev recurrence for degrees 0..19 at 33 dyadic points; endpoint, empty, nonfinite and cancellation fibers.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: independent polynomial/basis/integer fixtures and explicit finite-error partitions. Simpson hidden bump demonstrates limits of sampled estimates. An initial correction-denominator mutant survived tight-tolerance tests; a loose-tolerance quartic assertion was added to isolate the correction, and that mutant was then rejected. No speed claim; no percentages inferred from finite observations. Rust syntax projection is partial, without macro expansion, type resolution or semantic proof.

### 2026-10-30 · tests

Compiled production negative control: Change recurrence -b2 to +b2; basis_oracle fails. Original bytes restored and named positive test rerun. Full algorithms library passes twice debug and twice release with identical sorted test-name/results; receipts bind logs under $MOVA_DATA/runs/octopus-wave19-numeric-2026-10-30. Source projection generated twice and compared before installation. Strict crate all-target Clippy attempted; outcome in receipt.
