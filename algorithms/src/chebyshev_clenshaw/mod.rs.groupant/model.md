# algorithm-chebyshev-clenshaw

## descriptant

### chebyshev_clenshaw mod.rs

Full coefficient real series p(x)=sum c_k T_k(x), T_0=1, T_1=x. Clenshaw b_k=2*x*b_(k+1)-b_(k+2)+c_k and p=c0+x*b1-b2. Actual f64 operation order follows source, including validation and overflow errors.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

## model

## behavior_complement

Let D be validated inputs and F the source f64 transition relation including tagged errors/status. Behavior complement is all candidate (input, output) pairs not in F: in particular nonfinite successful values, wrong recurrence results and falsely universal error guarantees. Invalid-input observations are errors, not approximate numeric answers. Full coefficient real series p(x)=sum c_k T_k(x), T_0=1, T_1=x. Clenshaw b_k=2*x*b_(k+1)-b_(k+2)+c_k and p=c0+x*b1-b2. Actual f64 operation order follows source, including validation and overflow errors.

## reverse_reconstruction

At x=0, p=c0-c2+c4-... while odd coefficients are arbitrary: [2,3] and [2,9] both return 2. General real fibers are affine hyperplanes in coefficient space; rounding enlarges or moves fibers. Neither degree nor coefficients uniquely reconstruct.

## endpoint_parity

At x=1 real output=sum c; at x=-1 output=sum (-1)^k c. Endpoint recurrence may overflow even if a rearranged sum would not; finite arithmetic is part of acceptance.

## coefficient_convention

Constant [7] returns 7, never 3.5; empty list returns zero. Appending trailing zero preserves real polynomial, though extra rounded operations can differ for general inputs.

## basis_conversion

Degree two coefficients [a,b,c] convert to power basis [a-c,b,2c]. This gives an independent real relation to Horner; finite floating comparisons require additive tolerance.

## domain_partition

Reject NaN/infinite coefficients or x outside [-1,1] before recurrence, even for empty series. Reject nonfinite recurrence/result separately. No claim outside interval.


## approximant

### Forward model and invariant

Full coefficient real series p(x)=sum c_k T_k(x), T_0=1, T_1=x. Clenshaw b_k=2*x*b_(k+1)-b_(k+2)+c_k and p=c0+x*b1-b2. Actual f64 operation order follows source, including validation and overflow errors.

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
