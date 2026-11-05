# algorithm-subset-convolution

## descriptant

### subset_convolution

Prospective model saved before implementation in model.md#prospective (relative from source directory). Original standard-library code. This epoch adds explicit supported domains and independent finite checks. See model.md for typed excluded-output and preimage relations. Partial syntax omits macro expansion and semantic resolution. No executable approximation profile is attached; useful cheaper metrics and evidence remain unsupported. No timing gain is claimed.

## model

# subset_convolution mod.rs

## Forward

Ranked zeta transforms F[k,S]=sum_{T subset S,|T|=k}a[T]; H[k,S]=sum_i F[i,S]G[k-i,S]; Mobius invert then return H[|S|,S]. Equal power2 lengths<=4096; modulus998244353. O(d²2^d) time O(d2^d) memory. Bjorklund et al., Fourier meets Mobius, STOC 2007.

## Invariant

After Mobius inversion, evaluating rank=popcount(S) removes overlapping pairs because |A|+|B|=|A union B| iff A and B are disjoint.

## Cost

Ranked zeta transforms F[k,S]=sum_{T subset S,|T|=k}a[T]; H[k,S]=sum_i F[i,S]G[k-i,S]; Mobius invert then return H[|S|,S]. Equal power2 lengths<=4096; modulus998244353. O(d²2^d) time O(d2^d) memory. Bjorklund et al., Fourier meets Mobius, STOC 2007.

## behavior_complement

At validated a,b, any candidate y with y[S] unequal to sum_{T subset S}a[T]b[S xor T] modulo p, or residue>=p, is excluded. Shape-invalid input paired with Ok is excluded.

## reverse_reconstruction

For observed y retain pairs satisfying every subset convolution equation. Zero a with any b maps to zero; for fixed a with a[0] nonzero the triangular equations uniquely recover b. Raw residue representatives remain ambiguous.

## rank_filter

After Mobius inversion, evaluating rank=popcount(S) removes overlapping pairs because |A|+|B|=|A union B| iff A and B are disjoint.

## zeta_inversion

Each bit update adds the lower subset; reverse Mobius subtracts it. In the field these transforms are inverse linear maps over ranked arrays.

## arithmetic_bound

Inputs reduce modulo998244353; each product<(p-1)^2<2^60 and each accumulated reduced sum<2p. No checked overflow is needed within the validated size domain.

## validation_order

Either length>4096 returns TooLarge before equal/power2 shape checks. d<=12. Three ranked tables use O(d2^d) elements; allocations use std policy.

## evidence

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: (see History). Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### Evidence

## prospective

### Prospective model

Ranked zeta transforms F[k,S]=sum_{T subset S,|T|=k}a[T]; H[k,S]=sum_i F[i,S]G[k-i,S]; Mobius invert then return H[|S|,S]. Equal power2 lengths<=4096; modulus998244353. O(d²2^d) time O(d2^d) memory. Bjorklund et al., Fourier meets Mobius, STOC 2007.

Validation precedes arithmetic. Modular inputs normalize before products; products fit u64. Resource caps are policy refusals, distinct from algebraic absence. Allocator exhaustion is not recoverable in this bounded std implementation. No callbacks. Independent small oracles, repeated debug/release and compiled mutants are planned. No executable approximation profile is supported: no validated useful cheaper metric yet. Expected asymptotic improvement is a hypothesis, not measured speed.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: $MOVA_DATA/runs/octopus-wave25-algebra-2026-10-30. Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.
