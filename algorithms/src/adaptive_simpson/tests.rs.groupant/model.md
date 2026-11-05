# algorithm-adaptive-simpson-tests

## descriptant

### adaptive_simpson tests.rs

Executable finite checks for adaptive_simpson: analytic_polynomials tests degrees 0..7 and sine against exact antiderivatives; budgets count actual calls, depth and adjacent-endpoint stagnation, hidden bump, overflow and invalid input.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

## model

## behavior_complement

For fixed fixtures, complement consists of assertion traces differing from the required exact errors, statuses or tolerance comparisons; implementations agreeing only here remain possible. A passing trace excludes the compiled mutant but not arbitrary bugs outside fixtures.

## reverse_reconstruction

Pass reconstructs the set of implementations consistent with these finite assertions; it does not identify one implementation. Untested inputs remain arbitrary. With fixed parameters and deterministic callbacks that return the same values at every abscissa visited by the first run, the second follows the same traversal and report, assuming normal completion. The zero function and the indicator of (0.1,0.2) share the initial five samples and produce identical zero reports despite real integrals 0 and 0.1. Unvisited function values, smoothness, integral and derivative bounds cannot be reconstructed.

## oracle_independence

analytic_polynomials tests degrees 0..7 and sine against exact antiderivatives; budgets count actual calls, depth and adjacent-endpoint stagnation, hidden bump, overflow and invalid input.

## mutation_sensitivity

Change Simpson midpoint weight from 2/3 to 1/3; analytic_polynomials fails

## finite_scope

Finite fixture agreement supplies empirical observations, not a semantic proof or universal error cap. Different implementations with the same fixture outputs remain in the pass fiber.

## approximant

### Forward model and invariant

Executable finite checks for adaptive_simpson: analytic_polynomials tests degrees 0..7 and sine against exact antiderivatives; budgets count actual calls, depth and adjacent-endpoint stagnation, hidden bump, overflow and invalid input.

Invariant: all successful scalar results are finite; invalid input and nonfinite arithmetic stay distinct from values. Simpson pending panels partition the original interval, cached values match their abscissae and evaluations stay within budget. Tests preserve fixture independence and never infer universal accuracy from passing samples.

Cost: O(evaluations) scalar work plus callback cost. Fixed 65-panel stack, depth cap 64, no heap allocation. Evaluation count never exceeds supplied usize budget; increment by two only after subtraction check. Callback own resources/panics excluded.

Numerical scope: real formulas motivate f64 recurrences, not bitwise real semantics. No universal relative bound near zero/cancellation. No useful cheaper certified executable surrogate chosen; this remains an explicit gap.

## tests

### Evidence

analytic_polynomials tests degrees 0..7 and sine against exact antiderivatives; budgets count actual calls, depth and adjacent-endpoint stagnation, hidden bump, overflow and invalid input.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: independent polynomial/basis/integer fixtures and explicit finite-error partitions. Simpson hidden bump demonstrates limits of sampled estimates. An initial correction-denominator mutant survived tight-tolerance tests; a loose-tolerance quartic assertion was added to isolate the correction, and that mutant was then rejected. No speed claim; no percentages inferred from finite observations. Rust syntax projection is partial, without macro expansion, type resolution or semantic proof.

### 2026-10-30 · tests

Compiled production negative control: Change Simpson midpoint weight from 2/3 to 1/3; analytic_polynomials fails. Original bytes restored and named positive test rerun. Full algorithms library passes twice debug and twice release with identical sorted test-name/results; receipts bind logs under $MOVA_DATA/runs/octopus-wave19-numeric-2026-10-30. Source projection generated twice and compared before installation. Strict crate all-target Clippy attempted; outcome in receipt.

### 2026-11-01 · document

#### Sampled-output boundary — 2026-11-01

The hidden-indicator assertion compares full reports; it does not independently
calculate either integral. On the fixture, sampling observes 0, 1/2, 1, 1/4,
3/4 in that order. Both functions return zero there, giving equal reports with
five evaluations. The integral difference follows separately from interval
length, in real arithmetic. A source-linked Rust control with indicator
(0.2,0.3) changes the value at 1/4 and produces a different report. Thus the
sample-agreement premise matters; different samples do not generally imply
different reports. This observation does not characterize all report fibers,
prove smooth-function error bounds, or model callback side effects.
