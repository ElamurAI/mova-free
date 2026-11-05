# algorithm-integer-sqrt

## descriptant

### integer_sqrt mod epoch

Prospective assumptions are retained in model.md#prospective. The mathematical combiner reviews source, partial syntax, forward/typed models, invariants and finite evidence. It preserves predictions and failed controls when revising claims; it is not an autonomous proof generator.

Replacing <= by < in the square predicate compiled and exactly tests::square_neighbors_and_maximum failed (exit 101; one failed, two filtered).

Implementation and tests form separate leaf groupants. Parent registry wiring remains integration work. Partial syntax has opaque macros and no semantic/type resolution. No universal proof or application behavior is claimed.

## model

# integer_sqrt mod model

## Forward

For every u64 n, return r satisfying r^2<=n<(r+1)^2. Powers in this formula are mathematical integers.

## Invariant

0<=lo<=hi<=2^32; all k<lo satisfy k^2<=n, all k>=hi up to 2^32 satisfy k^2>n. The interval strictly shrinks. Zero is always accepted, so final lo>=1. Midpoint squares fit u128.

## Cost

At most 33 iterations, constant space; no allocations or floating point.

## nonbehavior

Type: behavior_complement. U=u64 x u64; R(n,r) iff r^2<=n<(r+1)^2. U minus R contains exactly candidates violating either inequality. Evaluate candidate squares mathematically, including invalid large r; exact target with bounded evidence.

## square_fibers

Type: reverse_reconstruction. Observed r<=u32::MAX admits all n in [r^2,min((r+1)^2-1,u64::MAX)]; other r admit none. Many inputs share a root, except zero.

## square_boundary

Type: boundary_specialization. For r<=u32::MAX, root(r^2)=r; if r>0, root(r^2-1)=r-1. Explicit square-neighbor tests check inclusive equality.

## adjacent_inputs

Type: metamorphic_relation. For n<u64::MAX, root(n)<=root(n+1)<=root(n)+1; increments occur only when n+1 is a square. Fixed-seed checks exercise the inequality.

## search_budget

Type: resource_observation. Initial width 2^32 shrinks through binary splitting; at most 33 iterations with u128 comparisons and O(1) state. This is a source-derived bound, not measured throughput.

## evidence

### Evidence

Three focused tests pass twice with identical normalized statuses. 65,537 inputs 0..=65536, nine square-root values with square/neighbor cases, u64::MAX and 4,096 fixed-seed full-width inputs with adjacent monotonicity.

Standard u64::isqrt is independent of this source; u128 square bounds provide a second constraint.

Replacing <= by < in the square predicate compiled and exactly tests::square_neighbors_and_maximum failed (exit 101; one failed, two filtered).

Commands: cargo test -p mova-algorithms --offline integer_sqrt::tests:: -- --test-threads=1; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings. Owned-source rustfmt checks pass. Temporary library exports permit local crate checks but are not committed; parent owns integration. External copies compile with rustc --edition 2024 --test and run with --exact. Production sources stay unchanged.

Source-bound syntax is generated twice after formatting and compared. Structural evidence does not establish semantic correctness, school/inference wiring or throughput.

## prospective

### Prospective

Before implementation: Use a half-open binary search over possible root values through 2^32, with u128 square comparisons; expect r^2 <= n < (r+1)^2 for every u64. Compare against standard u64::isqrt and explicit square neighbors.

Predict at most 33 search steps or 64 exponent-bit steps respectively, constant auxiliary space. Fixed-seed and boundary fixtures, two repeated focused tests, strict Clippy and a compiled source mutant rejected by one specific test will supply bounded evidence. No general proof, timing gain, imported solution or downstream integration claim.

## History

### 2026-10-29 · descriptant

2026-10-29. Repository search found math/src/calc.rs delegating arbitrary-precision roots to a dependency. This fixed-width binary search is newly authored; standard isqrt is test-only. No external solutions or LeetCode imports.

### 2026-10-29 · evidence

Heading `### Evidence 2026-10-29` stays in evidence as `### Evidence`.

### 2026-10-29 · prospective

Heading `### Prospective epoch 2026-10-29` stays in prospective as `### Prospective`.
