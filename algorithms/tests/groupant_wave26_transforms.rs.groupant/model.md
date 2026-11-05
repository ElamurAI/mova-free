# algorithm-groupant-wave26-transforms

## descriptant

### Transform composition tests

### Prospective integration test model
Before implementation: orthonormal Haar and DCT preserve norm separately; composing DCT(Haar(x)) then inverse DCT then inverse Haar reconstructs x in real arithmetic, provided intermediate vectors stay inside input caps. Independent norm is sum of sample squares. Goertzel requested bins must match same bins of complex Bluestein on zero-imaginary samples; independent negative-phase trigonometric sums prevent shared-sign false agreement. Savitzky-Golay on degree<=d unit-grid polynomials reproduces target values, including shifted windows; independently construct unscaled Vandermonde QR fits and compare predictions and analytic values. Invalid domains must remain tagged errors rather than entering compositions. Finite fixture tolerances only; no universal rounding certificate. Three tests, bounded work; no timing. Compile one production sign mutant in Goertzel, require named spectral composition assertion failure, restore exact source and rerun. Harness twice debug/release with identical normalized outcomes.

#### Mathematical combiner

Partial syntax binds the test transition. Forward model specifies finite assertions; complement excludes incompatible observations; reconstruction describes implementation fibers. Orthogonal, spectral and smoothing specializations model public wiring. Independent analytic anchors constrain shared-kernel agreement; finite passing observations do not prove semantics. No macro expansion or semantic AST. Underlying algorithm references remain in Haar, DCT, Bluestein, Goertzel and Savitzky-Golay modules.

## model

## behavior_complement

Pass excludes any implementation whose fixture observations violate Parseval, inverse reconstruction, independent negative-phase DFT sums, analytic polynomial reproduction, cross-kernel agreement or explicit domain-error tags. It does not exclude untested numerical defects; same-output shared bugs can survive cross-comparison alone, hence independent anchors.

## reverse_reconstruction

Pass identifies the equivalence class of implementations satisfying the finite asserted observations within tolerances. Many implementations differ off-grid or outside fixtures and still pass; implementations differing within tolerance can also pass. A failed assertion identifies at least one violated observed relation, not a unique faulty kernel or source repair. Full transform inversion requires all coefficients and second-call domain admissibility; selected bins and local smoothing generally have ambiguous preimages.

## orthogonal_composition

Haar and DCT are real orthogonal maps with energy preservation; DCT(Haar(x)) inverse is Haar-inverse(DCT-inverse(y)), reversing order. Fixture amplitudes keep every intermediate in both API domains. Sum-of-squares is independent of transform recurrence; tolerances account for finite rounding.

## spectral_composition

Real input embeds as Complex(re=x,im=0). Goertzel's ordered repeated integer bins select Bluestein's negative-forward, unnormalized DFT entries. Independent direct sums for every selected k detect common sign/normalization mistakes; input n includes primes, powers and singleton.

## smoothing_composition

Known degree0..4 polynomials at n=17 lie in the seven-point fitting space. Every boundary output is checked against analytic polynomial and independent unscaled target-centered Vandermonde QR. QR comparison alone is shared dependency, so analytic values remain authoritative anchor.

## oracle_independence

Energy sums, trigonometric direct DFT and analytic polynomial values do not call production kernels to construct expected results. QR is a supplementary composition comparison, not the sole oracle.

## mutation_sensitivity

Negating Goertzel terminal imaginary component compiles and fails named spectral composition assertion; unchanged original bytes are restored before all-three positive rerun. Compile failure alone does not count as detection.

## finite_scope

Finite deterministic fixture/tolerance coverage is not all-f64 verification, a performance claim or a 1/2/5/10% surrogate certificate. Allocation exhaustion is not injected; callbacks are absent. Independent oracle formulas also round in f64.

## approximant

### Forward and invariant

Three integration tests run public APIs on bounded finite fixtures: orthogonal_composition_parseval_and_roundtrip checks individual Haar/DCT Parseval and DCT(Haar(x)) inversion; selected_spectrum_matches_bluestein_and_direct_dft checks requested Goertzel bins against Bluestein and independent negative-phase DFT sums; smoothing_matches_polynomials_and_unscaled_qr checks shifted-window smoothing against known polynomial values and independently formed unscaled QR. Invalid input assertions preserve distinct empty/shape/value policies.

Every successful test terminates without assertion panic and validates both independent expected values and composition wiring. Input data are immutable; errors must match explicit tags. O(sum n²) for DCT/direct DFT plus bounded local QR work, finite vectors up to 128 samples. Allocation policy inherits production and standard test allocations. No callback cost. Floating agreement uses absolute tolerance 2e-8*(1+abs(reference)), defining finite-observation neighborhoods, not a universal relative bound. Inverse laws depend on second-call magnitude caps.

## tests

Three tests twice debug and twice release, normalized pass records identical; Goertzel production imaginary-sign mutant compiled and failed selected_spectrum_matches_bluestein_and_direct_dft, restored exact source and all three positives pass. Strict focused Clippy and formatting pass; paired source syntax byte-identical.

Empty-domain differences, nondyadic lengths, bin-range errors, NaN/infinity, invalid smoothing window and underdetermined QR are asserted. No resource fault injection.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

Three tests twice debug and twice release, normalized pass records identical; Goertzel production imaginary-sign mutant compiled and failed selected_spectrum_matches_bluestein_and_direct_dft, restored exact source and all three positives pass. Strict focused Clippy and formatting pass; paired source syntax byte-identical.

Evidence: $MOVA_DATA/runs/octopus-wave26-transforms-2026-10-30. No timing or universal accuracy claim.
