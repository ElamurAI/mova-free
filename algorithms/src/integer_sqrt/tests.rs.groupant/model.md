# algorithm-integer-sqrt-tests

## descriptant

### integer_sqrt tests epoch

Prospective assumptions are retained in ../mod.rs.groupant/model.md#prospective. The mathematical combiner reviews source, partial syntax, forward/typed models, invariants and finite evidence. It preserves predictions and failed controls when revising claims; it is not an autonomous proof generator.

Replacing <= by < in the square predicate compiled and exactly tests::square_neighbors_and_maximum failed (exit 101; one failed, two filtered).

Implementation and tests form separate leaf groupants. Parent registry wiring remains integration work. Partial syntax has opaque macros and no semantic/type resolution. No universal proof or application behavior is claimed.

## model

# integer_sqrt tests model

## Forward

Accept exactly when all oracle assertions and metamorphic conditions succeed on this explicit finite fixture set: 65,537 inputs 0..=65536, nine square-root values with square/neighbor cases, u64::MAX and 4,096 fixed-seed full-width inputs with adjacent monotonicity.

## Invariant

Expected independent-oracle values do not invoke the tested routine; metamorphic sides explicitly do. Inputs are deterministic and all arithmetic bounded, checked or widened.

## Cost

Fixed finite work: 65,537 inputs 0..=65536, nine square-root values with square/neighbor cases, u64::MAX and 4,096 fixed-seed full-width inputs with adjacent monotonicity. No measured speed claim.

## nonbehavior

Type: behavior_complement. U is candidate implementations paired with pass/fail on the finite fixture set F. R(P,pass) iff all assertions succeed; R(P,fail) iff a terminating call/assertion panics or a mismatch occurs. U minus R comprises false passes despite failure and false failures despite all assertions holding. Harness crashes/nontermination are excluded.

## failure_fibers

Type: reverse_reconstruction. A pass admits all implementations agreeing on F with arbitrary off-fixture behavior. A failure admits all faulty implementations or faulty oracles producing that mismatch/panic; no unique defect reconstruction.

## oracle

Type: oracle_correspondence. Standard u64::isqrt is independent of this source; u128 square bounds provide a second constraint. Finite correspondence assumes the oracle; no universal proof.

## mutation

Type: counterfactual_boundary. Replacing <= by < in the square predicate compiled and exactly tests::square_neighbors_and_maximum failed (exit 101; one failed, two filtered). Intervention changes source operations with fixture inputs fixed.

## schedule

Type: resource_observation. 65,537 inputs 0..=65536, nine square-root values with square/neighbor cases, u64::MAX and 4,096 fixed-seed full-width inputs with adjacent monotonicity. Fixed seed, wrapping generator and loop bounds determine observations; wall-clock time is not part of the deterministic output.

## evidence

### Evidence

Three focused tests pass twice with identical normalized statuses. 65,537 inputs 0..=65536, nine square-root values with square/neighbor cases, u64::MAX and 4,096 fixed-seed full-width inputs with adjacent monotonicity.

Standard u64::isqrt is independent of this source; u128 square bounds provide a second constraint.

Replacing <= by < in the square predicate compiled and exactly tests::square_neighbors_and_maximum failed (exit 101; one failed, two filtered).

Commands: cargo test -p mova-algorithms --offline integer_sqrt::tests:: -- --test-threads=1; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings. Owned-source rustfmt checks pass. Temporary library exports permit local crate checks but are not committed; parent owns integration. External copies compile with rustc --edition 2024 --test and run with --exact. Production sources stay unchanged.

Source-bound syntax is generated twice after formatting and compared. Structural evidence does not establish semantic correctness, school/inference wiring or throughput.

## History

### 2026-10-29 · descriptant

2026-10-29. Repository search found math/src/calc.rs delegating arbitrary-precision roots to a dependency. This fixed-width binary search is newly authored; standard isqrt is test-only. No external solutions or LeetCode imports.

### 2026-10-29 · evidence

Heading `### Evidence 2026-10-29` stays in evidence as `### Evidence`.
