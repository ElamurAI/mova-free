# algorithm-modular-power-tests

## descriptant

### modular_power tests epoch

Prospective assumptions are retained in ../mod.rs.groupant/model.md#prospective. The mathematical combiner reviews source, partial syntax, forward/typed models, invariants and finite evidence. It preserves predictions and failed controls when revising claims; it is not an autonomous proof generator.

Narrowing power squaring or accumulator multiplication to wrapping u64 compiled; each exactly filtered tests::zero_identity_and_full_width_products failed (exit 101; one failed, two filtered). Initial parity-only cases missed accumulator narrowing; the added (-2)^3 identity closed that observed gap.

Implementation and tests form separate leaf groupants. Parent registry wiring remains integration work. Partial syntax has opaque macros and no semantic/type resolution. No universal proof or application behavior is claimed.

## model

# modular_power tests model

## Forward

Accept exactly when all oracle assertions and metamorphic conditions succeed on this explicit finite fixture set: 10,200 small repeated-multiplication cases (25 bases x17 exponents x24 positive moduli), zero/one modulus and exponent-zero boundaries, full-width parity/products, and 1,024 fixed-seed reduction/addition cases.

## Invariant

Expected independent-oracle values do not invoke the tested routine; metamorphic sides explicitly do. Inputs are deterministic and all arithmetic bounded, checked or widened.

## Cost

Fixed finite work: 10,200 small repeated-multiplication cases (25 bases x17 exponents x24 positive moduli), zero/one modulus and exponent-zero boundaries, full-width parity/products, and 1,024 fixed-seed reduction/addition cases. No measured speed claim.

## nonbehavior

Type: behavior_complement. U is candidate implementations paired with pass/fail on the finite fixture set F. R(P,pass) iff all assertions succeed; R(P,fail) iff a terminating call/assertion panics or a mismatch occurs. U minus R comprises false passes despite failure and false failures despite all assertions holding. Harness crashes/nontermination are excluded.

## failure_fibers

Type: reverse_reconstruction. A pass admits all implementations agreeing on F with arbitrary off-fixture behavior. A failure admits all faulty implementations or faulty oracles producing that mismatch/panic; no unique defect reconstruction.

## oracle

Type: oracle_correspondence. Small exponents use repeated multiplication, no binary exponent decomposition or repeated squaring. Parity, identity and exponent-addition laws add explicit algebraic constraints. Finite correspondence assumes the oracle; no universal proof.

## mutation

Type: counterfactual_boundary. Narrowing power squaring or accumulator multiplication to wrapping u64 compiled; each exactly filtered tests::zero_identity_and_full_width_products failed (exit 101; one failed, two filtered). Initial parity-only cases missed accumulator narrowing; the added (-2)^3 identity closed that observed gap. Intervention changes source operations with fixture inputs fixed.

## schedule

Type: resource_observation. 10,200 small repeated-multiplication cases (25 bases x17 exponents x24 positive moduli), zero/one modulus and exponent-zero boundaries, full-width parity/products, and 1,024 fixed-seed reduction/addition cases. Fixed seed, wrapping generator and loop bounds determine observations; wall-clock time is not part of the deterministic output.

## evidence

### Evidence

Three focused tests pass twice with identical normalized statuses. 10,200 small repeated-multiplication cases (25 bases x17 exponents x24 positive moduli), zero/one modulus and exponent-zero boundaries, full-width parity/products, and 1,024 fixed-seed reduction/addition cases.

Small exponents use repeated multiplication, no binary exponent decomposition or repeated squaring. Parity, identity and exponent-addition laws add explicit algebraic constraints.

Narrowing power squaring or accumulator multiplication to wrapping u64 compiled; each exactly filtered tests::zero_identity_and_full_width_products failed (exit 101; one failed, two filtered). Initial parity-only cases missed accumulator narrowing; the added (-2)^3 identity closed that observed gap.

Commands: cargo test -p mova-algorithms --offline modular_power::tests:: -- --test-threads=1; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings. Owned-source rustfmt checks pass. Temporary library exports permit local crate checks but are not committed; parent owns integration. External copies compile with rustc --edition 2024 --test and run with --exact. Production sources stay unchanged.

Source-bound syntax is generated twice after formatting and compared. Structural evidence does not establish semantic correctness, school/inference wiring or throughput.

## History

### 2026-10-29 · descriptant

2026-10-29. Repository search found math/src/nt.rs::modpow_plain; this unsigned specialization composes its repository-owned square-and-multiply idea with exact u128 products. No external solutions or LeetCode imports.

### 2026-10-29 · evidence

Heading `### Evidence 2026-10-29` stays in evidence as `### Evidence`.
