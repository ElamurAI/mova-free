# algorithm-modular-power

## descriptant

### modular_power mod epoch

Prospective assumptions are retained in model.md#prospective. The mathematical combiner reviews source, partial syntax, forward/typed models, invariants and finite evidence. It preserves predictions and failed controls when revising claims; it is not an autonomous proof generator.

Narrowing power squaring or accumulator multiplication to wrapping u64 compiled; each exactly filtered tests::zero_identity_and_full_width_products failed (exit 101; one failed, two filtered). Initial parity-only cases missed accumulator narrowing; the added (-2)^3 identity closed that observed gap.

Implementation and tests form separate leaf groupants. Parent registry wiring remains integration work. Partial syntax has opaque macros and no semantic/type resolution. No universal proof or application behavior is claimed.

## model

# modular_power mod model

## Forward

For m=0 return Err(ZeroModulus). Otherwise return Ok(r), 0<=r<m, r congruent to base^exponent modulo m. Exponent zero means identity including 0^0; m=1 always returns zero.

## Invariant

For positive m, result and power are in [0,m); result*power^remaining_exponent is congruent to the original power. Each right shift consumes a bit. Products of two residues below m<=u64::MAX fit u128 exactly.

## Cost

At most 64 bit iterations, O(1+log(exponent+1)) modular operations and O(1) space. Zero modulus exits before remainder. No timing claim.

## nonbehavior

Type: behavior_complement. U consists of all input triples paired with Err or Ok(u64). U minus R contains Ok when m=0, Err when m>0, and Ok(r) with m>0 but r>=m or wrong power congruence. Exact mathematical target, bounded-tested.

## residue_fibers

Type: reverse_reconstruction. Err admits exactly m=0 triples. Ok(r) admits all triples with m>r and b^e congruent to r mod m. Different bases/exponents/moduli retain ambiguity; e=0 restricts r=1%m and m=1 permits all b,e with r=0.

## base_reduction

Type: metamorphic_relation. For m>0, P(b,e,m)=P(b%m,e,m); changing a base by a multiple of m is valid only if it fits u64. Fixed-seed tests exercise reduction.

## exponent_addition

Type: algebraic_composition. For m>0 and a+c fitting u64, P(x,a+c,m)=P(x,a,m)*P(x,c,m) mod m. Tests use a,c<=31 and widened products.

## parity_boundary

Type: boundary_specialization. P(x,0,m)=1%m. For m>1, P(m-1,e,m)=1 for even e and m-1 for odd e. Explicit (-2)^3 mod u64::MAX probes accumulator multiplication overflow.

## evidence

### Evidence

Three focused tests pass twice with identical normalized statuses. 10,200 small repeated-multiplication cases (25 bases x17 exponents x24 positive moduli), zero/one modulus and exponent-zero boundaries, full-width parity/products, and 1,024 fixed-seed reduction/addition cases.

Small exponents use repeated multiplication, no binary exponent decomposition or repeated squaring. Parity, identity and exponent-addition laws add explicit algebraic constraints.

Narrowing power squaring or accumulator multiplication to wrapping u64 compiled; each exactly filtered tests::zero_identity_and_full_width_products failed (exit 101; one failed, two filtered). Initial parity-only cases missed accumulator narrowing; the added (-2)^3 identity closed that observed gap.

Commands: cargo test -p mova-algorithms --offline modular_power::tests:: -- --test-threads=1; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings. Owned-source rustfmt checks pass. Temporary library exports permit local crate checks but are not committed; parent owns integration. External copies compile with rustc --edition 2024 --test and run with --exact. Production sources stay unchanged.

Source-bound syntax is generated twice after formatting and compared. Structural evidence does not establish semantic correctness, school/inference wiring or throughput.

## prospective

### Prospective

Before implementation: Compose the repository-owned square-and-multiply idea in math/src/nt.rs::modpow_plain into a std-only u64 specialization; u128 products prevent intermediate overflow. Reject modulus zero, reduce identity modulo m, retain exponent-zero convention. Compare small exponents against repeated multiplication and full-width parity/reduction laws.

Predict at most 33 search steps or 64 exponent-bit steps respectively, constant auxiliary space. Fixed-seed and boundary fixtures, two repeated focused tests, strict Clippy and a compiled source mutant rejected by one specific test will supply bounded evidence. No general proof, timing gain, imported solution or downstream integration claim.

## History

### 2026-10-29 · descriptant

2026-10-29. Repository search found math/src/nt.rs::modpow_plain; this unsigned specialization composes its repository-owned square-and-multiply idea with exact u128 products. No external solutions or LeetCode imports.

### 2026-10-29 · evidence

Heading `### Evidence 2026-10-29` stays in evidence as `### Evidence`.

### 2026-10-29 · prospective

Heading `### Prospective epoch 2026-10-29` stays in prospective as `### Prospective`.
