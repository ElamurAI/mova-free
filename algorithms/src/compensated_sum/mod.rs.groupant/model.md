# algorithm-compensated-sum

## descriptant

### compensated_sum mod.rs

Neumaier accumulates total and correction, selecting correction formula by magnitude. Return total+correction with all inputs and each state finite. Mathematical target is the real sum, but f64 rounding and underflow remain.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

## model

## behavior_complement

Let D be validated inputs and F the source f64 transition relation including tagged errors/status. Behavior complement is all candidate (input, output) pairs not in F: in particular nonfinite successful values, wrong recurrence results and falsely universal error guarantees. Invalid-input observations are errors, not approximate numeric answers. Neumaier accumulates total and correction, selecting correction formula by magnitude. Return total+correction with all inputs and each state finite. Mathematical target is the real sum, but f64 rounding and underflow remain.

## reverse_reconstruction

[1,2] and [3] both map to 3; zero-sum insertions preserve real totals but may change rounded traversal. The fiber contains many lengths/orders and cannot recover terms or correction state from final sum.

## magnitude_branch

When abs(total)>=abs(value), correction adds (total-next)+value, otherwise (value-next)+total. Both cancellation orders are exercised.

## lost_unit_recovery

[1e16,1,-1e16] yields 1; naive left fold yields 0. This particular witness is exact but does not establish exactness for all slices.

## subnormal_boundary

Two smallest positive subnormals return twice that quantum. Gradual underflow semantics assumed; no flush-to-zero transformation is requested.

## overflow_refusal

MAX+MAX is rejected; compensation never licenses an infinite total. Reject nonfinite input before state changes. Mathematical summability does not remove order-dependent overflow.


## approximant

### Forward model and invariant

Neumaier accumulates total and correction, selecting correction formula by magnitude. Return total+correction with all inputs and each state finite. Mathematical target is the real sum, but f64 rounding and underflow remain.

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
