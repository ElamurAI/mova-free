# algorithm-karatsuba-polynomial

## descriptant

### karatsuba_polynomial

Prospective model saved before implementation in model.md#prospective (relative from source directory). Original standard-library code. This epoch adds explicit supported domains and independent finite checks. See model.md for typed excluded-output and preimage relations. Partial syntax omits macro expansion and semantic resolution. No executable approximation profile is attached; useful cheaper metrics and evidence remain unsupported. No timing gain is claimed.

## model

# karatsuba_polynomial mod.rs

## Forward

Ascending coefficient polynomials over prime998244353. P=low*low + x^m((low+high)*(low+high)-low*low-high*high)+x^(2m)high*high. Pad power2 and trim to a.len+b.len-1; empty gives empty; lengths<=4096. O(n^log2(3)) time O(n) live memory. Karatsuba and Ofman, Multiplication of many-digital numbers by automatic computers (1962).

## Invariant

(a0+a1)(b0+b1)-a0b0-a1b1=a0b1+a1b0 supplies middle coefficients; recursion splits padded equal power2 vectors and uses schoolbook at n<=16.

## Cost

Ascending coefficient polynomials over prime998244353. P=low*low + x^m((low+high)*(low+high)-low*low-high*high)+x^(2m)high*high. Pad power2 and trim to a.len+b.len-1; empty gives empty; lengths<=4096. O(n^log2(3)) time O(n) live memory. Karatsuba and Ofman, Multiplication of many-digital numbers by automatic computers (1962).

## behavior_complement

Accepted nonempty pair excludes outputs with length other than len(a)+len(b)-1 or coefficient not congruent to sum a[i]b[k-i]. Zero trailing coefficients must remain. Over-cap lengths exclude Ok even with empty partner.

## reverse_reconstruction

Observed product admits all factor pairs over the fixed field matching coefficient length. Scaling a by nonzero c and b by c^-1 preserves product; zero products and empty outputs are particularly nonunique. Inputs are also ambiguous modulo p.

## cross_identity

(a0+a1)(b0+b1)-a0b0-a1b1=a0b1+a1b0 supplies middle coefficients; recursion splits padded equal power2 vectors and uses schoolbook at n<=16.

## padding_trim

Padding by zeros preserves polynomial value; truncation removes only padding beyond exact output length, never original trailing zeros.

## arithmetic_bound

Normalized products fit u64; combined middle update plus2p subtracts lo and hi without underflow and stays below4p.

## validation_order

Either length>4096 returns TooLarge first; then either empty returns empty; else allocations and recursion. O(n^log2 3) work and O(n) simultaneously live storage; std allocator policy.

## evidence

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: (see History). Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### Evidence

## prospective

### Prospective model

Ascending coefficient polynomials over prime998244353. P=low*low + x^m((low+high)*(low+high)-low*low-high*high)+x^(2m)high*high. Pad power2 and trim to a.len+b.len-1; empty gives empty; lengths<=4096. O(n^log2(3)) time O(n) live memory. Karatsuba and Ofman, Multiplication of many-digital numbers by automatic computers (1962).

Validation precedes arithmetic. Modular inputs normalize before products; products fit u64. Resource caps are policy refusals, distinct from algebraic absence. Allocator exhaustion is not recoverable in this bounded std implementation. No callbacks. Independent small oracles, repeated debug/release and compiled mutants are planned. No executable approximation profile is supported: no validated useful cheaper metric yet. Expected asymptotic improvement is a hypothesis, not measured speed.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: $MOVA_DATA/runs/octopus-wave25-algebra-2026-10-30. Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.
