# algorithm-fast-fourier-transform

## descriptant

### fast_fourier_transform

Prospective model saved before implementation in model.md#prospective (relative from source directory). Original standard-library code. This epoch adds explicit supported domains and independent finite checks. See model.md for typed excluded-output and preimage relations. Partial syntax omits macro expansion and semantic resolution. No executable approximation profile is attached; useful cheaper metrics and evidence remain unsupported. No timing gain is claimed.

## model

# fast_fourier_transform mod.rs

## Forward

Radix-two Cooley–Tukey DFT: X[k]=sum x[j] exp(-2pi i jk/n); inverse conjugate sign and divide n. n is nonempty power2 <=65536, finite components bounded by 1e100. O(n log n) operations O(n) memory. Bit-reversed ordering then length-doubling butterflies. Floating results finite but no universal relative accuracy promise. Cooley and Tukey, Mathematics of Computation 19 (1965).

## Invariant

Each input component<=1e100 and n<=65536 bounds intermediates far below f64 overflow. Tests measure absolute error only on small fixtures; zero or cancellation makes a relative percentage guarantee unjustified.

## Cost

Radix-two Cooley–Tukey DFT: X[k]=sum x[j] exp(-2pi i jk/n); inverse conjugate sign and divide n. n is nonempty power2 <=65536, finite components bounded by 1e100. O(n log n) operations O(n) memory. Bit-reversed ordering then length-doubling butterflies. Floating results finite but no universal relative accuracy promise. Cooley and Tukey, Mathematics of Computation 19 (1965).

## behavior_complement

For validated input, output length differs from n or contains nonfinite components is excluded. The real DFT equation identifies an ideal target, but equality to it is not asserted for rounded f64 outputs; negative-phase and inverse normalization are required by the actual butterfly program.

## reverse_reconstruction

Ideal fixed-size DFT is invertible via inverse DFT. Rounded program fibers retain all finite bounded input bit patterns yielding the observed output; rounding may merge distinct inputs. Error InvalidValue admits any length-valid vector containing NaN, infinity or a component above1e100.

## roundoff_scope

Each input component<=1e100 and n<=65536 bounds intermediates far below f64 overflow. Tests measure absolute error only on small fixtures; zero or cancellation makes a relative percentage guarantee unjustified.

## permutation_schedule

Bit reversal is an involution; butterflies combine blocks of length2,4,...n. Every index is written once per stage, so n log2 n/2 butterflies execute.

## inverse_scaling

Forward uses negative phase; inverse uses positive phase then divides each coordinate by n. For n1 this is identity. Real identities do not certify f64 round-trip accuracy.

## validation_order

Length>65536 precedes non-power-two, which precedes finite/magnitude validation. Input is borrowed and unchanged; fresh output allocates O(n); allocator failure follows std policy.

## evidence

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: (see History). Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### Evidence

## prospective

### Prospective model

Radix-two Cooley–Tukey DFT: X[k]=sum x[j] exp(-2pi i jk/n); inverse conjugate sign and divide n. n is nonempty power2 <=65536, finite components bounded by 1e100. O(n log n) operations O(n) memory. Bit-reversed ordering then length-doubling butterflies. Floating results finite but no universal relative accuracy promise. Cooley and Tukey, Mathematics of Computation 19 (1965).

Validation precedes arithmetic. Modular inputs normalize before products; products fit u64. Resource caps are policy refusals, distinct from algebraic absence. Allocator exhaustion is not recoverable in this bounded std implementation. No callbacks. Independent small oracles, repeated debug/release and compiled mutants are planned. No executable approximation profile is supported: no validated useful cheaper metric yet. Expected asymptotic improvement is a hypothesis, not measured speed.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: $MOVA_DATA/runs/octopus-wave25-algebra-2026-10-30. Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.
