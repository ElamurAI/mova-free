# algorithm-discrete-cosine-transform

## descriptant

### discrete_cosine_transform / mod.rs

Original Rust; original algorithm citation appears in module documentation. Existing FFT and Householder QR reused where appropriate.

#### Prospective epoch

### Prospective transform models
Before implementation: DCT II uses sqrt(1/n) DC and sqrt(2/n) others, cos(pi*(j+.5)*k/n), inverse transpose; n<=2048, empty identity; O(n²) work/O(n) space. Haar full dyadic transform packs coarse value then detail levels; pair sum/difference divided sqrt(2); n<=65536 power two, no extension; empty invalid; O(n) work/space. Bluestein uses chirp exp(sign*i*pi*j²/n), symmetric inverse chirp convolution padded to power two >=2n-1, existing FFT, n<=16384; inverse divides n; empty invalid; O(n log n)/O(n). Goertzel recurrence s=x+2cos(w)s1-s2, final real=cos(w)s1-s2, imaginary=sin(w)s1 (integer-bin phase correction), n<=65536, bins<=65536, all k<n; empty input only accepts empty bins; O(n*b)/O(b). SG n<=4096, odd window<=31 and <=n, degree<=6 and <window, shifted full window at boundaries; offsets scaled by window, Vandermonde QR solves local least squares, constant coefficient predicts target; O(n*w*d²)/O(w*d+n). All finite magnitudes <=1e50, no callbacks, allocations follow standard policy except reused QR fallible allocation. Validate shapes/limits before values then compute; roundoff and rank/nonfinite failures explicit, no universal percent or residual-to-solution guarantee. Polynomial reproduction, independent direct sums, orthogonality and reconstruction are finite checks. No timing or gain claim. References: Ahmed/Natarajan/Rao (1974) DCT; Haar (1910); Bluestein (1970) linear filtering DFT; Goertzel (1958); Savitzky/Golay (1964).

#### Mathematical combiner

Source and SHA-bound partial syntax define transition F; forward/invariant approximants model F, behavior complement models excluded pairs and reconstruction models compatible antecedents. Finite independent tests constrain F. Combining these observations is not a semantic proof; no macro expansion or semantic AST. Leaf children=[]; root integrates exports/ancestor bindings.

## model

## behavior_complement

For validated input x let F(x) be source-defined tagged floating output. Excluded relation is (x,y) where y differs from F(x), including wrong normalization, wrong grid/layout, invalid-domain success or nonfinite success. This models excluded outputs, not only a list of tests. C[k,j]=alpha[k] cos(pi*(j+1/2)*k/n); alpha[0]=1/sqrt(n), others sqrt(2/n). Forward C*x, inverse C-transpose*x. n<=2048, empty identity, all finite |x|<=1e50, no extension or callbacks.

## reverse_reconstruction

For fixed n the real orthogonal transform is bijective with inverse C-transpose. Rounded outputs may collapse nearby inputs, so exact f64 input recovery is not promised. Empty output uniquely identifies empty input within successful domain; error tags have many invalid preimages.

## normalization_relation

DC of constant a is a*sqrt(n), other modes vanish in real arithmetic. A factor-of-sqrt(2) discrepancy is excluded by normalization, not tolerated as an approximation.

## transpose_reconstruction

Swapping sample and frequency for inverse retains weight indexed by frequency. Matrix transpose, not another forward DCT-II, gives real inverse.

## grid_boundary

Samples are half shifted j+1/2 in cosine phase. No endpoint duplication, padding or truncation; singleton is identity and empty does no division.

## roundoff_scope

Success is finite, but cancellation near zero makes relative error unbounded; energy and reconstruction checks are finite observations, not certificates.

## approximant

### Forward model and invariant

C[k,j]=alpha[k] cos(pi*(j+1/2)*k/n); alpha[0]=1/sqrt(n), others sqrt(2/n). Forward C*x, inverse C-transpose*x. n<=2048, empty identity, all finite |x|<=1e50, no extension or callbacks.

Cost: O(n²) work, O(n) output. Standard allocator exhaustion policy.

Successful values remain finite and input arrays are immutable. Orthogonal/DFT inverse laws require the transformed coefficients to satisfy the second call input cap; large admissible forward samples can produce coefficients above that cap, so the bounded API is not globally closed under round trips. Invalid domain checks precede computation; same error tag can cover several invalid premises. No callbacks; allocation failure is outside ordinary Result except errors inside reused QR. Transform algebra is exact over reals, implementation is rounded f64. Zero-reference relative error is undefined; cancellation precludes universal relative claims. No executable 1/2/5/10% surrogate certificate is supplied. Residuals, if present in reused QR, do not bound signal or solution error.

## tests

basis_and_direct_sum independently evaluates every basis impulse for n=1..19; roundtrip_and_energy covers n=0..65; dc_and_limits covers constant, empty and cap; invalid_values rejects nonfinite and excessive magnitude.

Control: Changing non-DC normalization sqrt(2/n) to sqrt(1/n) compiles and fails basis_and_direct_sum.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

20 new tests pass in twice debug and twice release crate runs (713 total each) with identical normalized outcomes. Five compiled production mutants fail named assertions and exact source restoration passes four tests each. Strict all-target algorithm Clippy and focused formatting pass. No timing, speed or universal accuracy claim.

### 2026-10-30 · tests

Evidence: $MOVA_DATA/runs/octopus-wave26-transforms-2026-10-30. No allocator fault injection; finite arithmetic bounds and cap tests are not exhaustive f64 verification.
