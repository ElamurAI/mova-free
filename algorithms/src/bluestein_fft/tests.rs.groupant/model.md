# algorithm-bluestein-fft-tests

## descriptant

### bluestein_fft / tests.rs

Original Rust; original algorithm citation appears in module documentation. Existing FFT and Householder QR reused where appropriate.

#### Prospective epoch

### Prospective transform models
Before implementation: DCT II uses sqrt(1/n) DC and sqrt(2/n) others, cos(pi*(j+.5)*k/n), inverse transpose; n<=2048, empty identity; O(n²) work/O(n) space. Haar full dyadic transform packs coarse value then detail levels; pair sum/difference divided sqrt(2); n<=65536 power two, no extension; empty invalid; O(n) work/space. Bluestein uses chirp exp(sign*i*pi*j²/n), symmetric inverse chirp convolution padded to power two >=2n-1, existing FFT, n<=16384; inverse divides n; empty invalid; O(n log n)/O(n). Goertzel recurrence s=x+2cos(w)s1-s2, final real=cos(w)s1-s2, imaginary=sin(w)s1 (integer-bin phase correction), n<=65536, bins<=65536, all k<n; empty input only accepts empty bins; O(n*b)/O(b). SG n<=4096, odd window<=31 and <=n, degree<=6 and <window, shifted full window at boundaries; offsets scaled by window, Vandermonde QR solves local least squares, constant coefficient predicts target; O(n*w*d²)/O(w*d+n). All finite magnitudes <=1e50, no callbacks, allocations follow standard policy except reused QR fallible allocation. Validate shapes/limits before values then compute; roundoff and rank/nonfinite failures explicit, no universal percent or residual-to-solution guarantee. Polynomial reproduction, independent direct sums, orthogonality and reconstruction are finite checks. No timing or gain claim. References: Ahmed/Natarajan/Rao (1974) DCT; Haar (1910); Bluestein (1970) linear filtering DFT; Goertzel (1958); Savitzky/Golay (1964).

#### Mathematical combiner

Source and SHA-bound partial syntax define transition F; forward/invariant approximants model F, behavior complement models excluded pairs and reconstruction models compatible antecedents. Finite independent tests constrain F. Combining these observations is not a semantic proof; no macro expansion or semantic AST. Leaf children=[]; root integrates exports/ancestor bindings.

## model

## behavior_complement

Pass excludes any implementation violating a named asserted finite observation, including compiled mutant: Swapping the forward/inverse chirp sign compiles and fails independent_dft. It does not exclude unseen-input errors.

## reverse_reconstruction

Pass has all implementations agreeing on checked fixture observations as preimages; behavior outside fixtures can differ. Fixed n full real-complex DFT is invertible using conjugate sign and 1/n; rounded arithmetic loses exact injectivity. Spectrum magnitude alone would have phase/shift ambiguity but API preserves complex spectrum. InvalidInput conflates empty, oversized and invalid-value preimages.

## oracle_independence

Independent basis/block sums, direct trigonometric sums or analytic polynomial/weight expectations: independent_dft compares both directions to separate trigonometric sum for all n=1..35; roundtrip covers primes/nonpowers and powers; impulse_boundary exercises n=16384; rejected_domains checks caps and real/imaginary nonfinite values.

## mutation_sensitivity

Swapping the forward/inverse chirp sign compiles and fails independent_dft.

## finite_scope

Tests cover finite chosen domains and tolerances; approximate equality allows neighborhoods of implementations. Passing cases do not prove all machine inputs, resource behavior or a universal numerical bound.

## approximant

### Forward model and invariant

Finite fixture-to-assertion transition. independent_dft compares both directions to separate trigonometric sum for all n=1..35; roundtrip covers primes/nonpowers and powers; impulse_boundary exercises n=16384; rejected_domains checks caps and real/imaginary nonfinite values.

Cost: O(m log m) work/O(m) space, m<=32768. Three FFT calls; standard allocation behavior; kernel errors propagated.

Successful values remain finite and input arrays are immutable. Orthogonal/DFT inverse laws require the transformed coefficients to satisfy the second call input cap; large admissible forward samples can produce coefficients above that cap, so the bounded API is not globally closed under round trips. Invalid domain checks precede computation; same error tag can cover several invalid premises. No callbacks; allocation failure is outside ordinary Result except errors inside reused QR. Transform algebra is exact over reals, implementation is rounded f64. Zero-reference relative error is undefined; cancellation precludes universal relative claims. No executable 1/2/5/10% surrogate certificate is supplied. Residuals, if present in reused QR, do not bound signal or solution error.

## tests

independent_dft compares both directions to separate trigonometric sum for all n=1..35; roundtrip covers primes/nonpowers and powers; impulse_boundary exercises n=16384; rejected_domains checks caps and real/imaginary nonfinite values.

Control: Swapping the forward/inverse chirp sign compiles and fails independent_dft.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

20 new tests pass in twice debug and twice release crate runs (713 total each) with identical normalized outcomes. Five compiled production mutants fail named assertions and exact source restoration passes four tests each. Strict all-target algorithm Clippy and focused formatting pass. No timing, speed or universal accuracy claim.

### 2026-10-30 · tests

Evidence: $MOVA_DATA/runs/octopus-wave26-transforms-2026-10-30. No allocator fault injection; finite arithmetic bounds and cap tests are not exhaustive f64 verification.
