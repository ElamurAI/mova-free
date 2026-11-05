# algorithm-goertzel

## descriptant

### goertzel / mod.rs

Original Rust; original algorithm citation appears in module documentation. Existing FFT and Householder QR reused where appropriate.

#### Prospective epoch

### Prospective transform models
Before implementation: DCT II uses sqrt(1/n) DC and sqrt(2/n) others, cos(pi*(j+.5)*k/n), inverse transpose; n<=2048, empty identity; O(n²) work/O(n) space. Haar full dyadic transform packs coarse value then detail levels; pair sum/difference divided sqrt(2); n<=65536 power two, no extension; empty invalid; O(n) work/space. Bluestein uses chirp exp(sign*i*pi*j²/n), symmetric inverse chirp convolution padded to power two >=2n-1, existing FFT, n<=16384; inverse divides n; empty invalid; O(n log n)/O(n). Goertzel recurrence s=x+2cos(w)s1-s2, final real=cos(w)s1-s2, imaginary=sin(w)s1 (integer-bin phase correction), n<=65536, bins<=65536, all k<n; empty input only accepts empty bins; O(n*b)/O(b). SG n<=4096, odd window<=31 and <=n, degree<=6 and <window, shifted full window at boundaries; offsets scaled by window, Vandermonde QR solves local least squares, constant coefficient predicts target; O(n*w*d²)/O(w*d+n). All finite magnitudes <=1e50, no callbacks, allocations follow standard policy except reused QR fallible allocation. Validate shapes/limits before values then compute; roundoff and rank/nonfinite failures explicit, no universal percent or residual-to-solution guarantee. Polynomial reproduction, independent direct sums, orthogonality and reconstruction are finite checks. No timing or gain claim. References: Ahmed/Natarajan/Rao (1974) DCT; Haar (1910); Bluestein (1970) linear filtering DFT; Goertzel (1958); Savitzky/Golay (1964).

#### Mathematical combiner

Source and SHA-bound partial syntax define transition F; forward/invariant approximants model F, behavior complement models excluded pairs and reconstruction models compatible antecedents. Finite independent tests constrain F. Combining these observations is not a semantic proof; no macro expansion or semantic AST. Leaf children=[]; root integrates exports/ancestor bindings.

## model

## behavior_complement

For validated input x let F(x) be source-defined tagged floating output. Excluded relation is (x,y) where y differs from F(x), including wrong normalization, wrong grid/layout, invalid-domain success or nonfinite success. This models excluded outputs, not only a list of tests. For each integer bin k, theta=2*pi*k/n; p_next=x+2cos(theta)p-q, then q=p. Output real=cos(theta)p-q, imag=sin(theta)p using exp(-i*n*theta)=1. Negative-phase unnormalized real-input DFT; n and bin count<=65536, every k<n, finite |x|<=1e50. Empty input accepts only empty indices.

## reverse_reconstruction

Partial bins are noninjective: bin0 alone sees only sum, so [1,-1] and [0,0] share zero; empty bins erase all sample information, though values still validate. Complete spectrum determines real samples in exact arithmetic; rounding and omitted bins prevent general exact reconstruction.

## phase_correction

Terminal state has one sample of phase offset in the usual Goertzel form. Rotated final formula c*p-q+i*s*p matches negative DFT for integer bins only; arbitrary real frequency is outside API.

## selection_relation

Output length equals requested bin count and preserves request order including repeated k. No sorting, deduplication or interpolation.

## endpoint_conditioning

DC and Nyquist recurrence has repeated real roots; large states and terminal cancellation can amplify errors. Finite results and finite tests do not certify relative accuracy.

## empty_precedence

Length/bin validity precedes value validity, both represented InvalidInput; empty indices return empty only after validating all samples. No callbacks or hidden grid assumptions.

## approximant

### Forward model and invariant

For each integer bin k, theta=2*pi*k/n; p_next=x+2cos(theta)p-q, then q=p. Output real=cos(theta)p-q, imag=sin(theta)p using exp(-i*n*theta)=1. Negative-phase unnormalized real-input DFT; n and bin count<=65536, every k<n, finite |x|<=1e50. Empty input accepts only empty indices.

Cost: O(n*b) work/O(b) output, worst bounded product 2^32 scalar iterations. Standard allocator policy; no callbacks.

Successful values remain finite and input arrays are immutable. Orthogonal/DFT inverse laws require the transformed coefficients to satisfy the second call input cap; large admissible forward samples can produce coefficients above that cap, so the bounded API is not globally closed under round trips. Invalid domain checks precede computation; same error tag can cover several invalid premises. No callbacks; allocation failure is outside ordinary Result except errors inside reused QR. Transform algebra is exact over reals, implementation is rounded f64. Zero-reference relative error is undefined; cancellation precludes universal relative claims. No executable 1/2/5/10% surrogate certificate is supplied. Residuals, if present in reused QR, do not bound signal or solution error.

## tests

independent_dft computes trigonometric sum for all bins and n=1..55; phase_and_duplicates checks sign/order/repeated bins; dc_nyquist_limits checks endpoints and caps; rejected_domains checks invalid bins even for empty input and invalid values even for empty bins.

Control: Negating terminal imaginary component compiles and fails phase_and_duplicates.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

20 new tests pass in twice debug and twice release crate runs (713 total each) with identical normalized outcomes. Five compiled production mutants fail named assertions and exact source restoration passes four tests each. Strict all-target algorithm Clippy and focused formatting pass. No timing, speed or universal accuracy claim.

### 2026-10-30 · tests

Evidence: $MOVA_DATA/runs/octopus-wave26-transforms-2026-10-30. No allocator fault injection; finite arithmetic bounds and cap tests are not exhaustive f64 verification.
