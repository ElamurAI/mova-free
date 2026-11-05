# algorithm-extended-gcd-tests

## descriptant

### algorithm-extended-gcd-tests descriptant

The combiner relates source-bound partial syntax, forward recurrence, typed output-preimage and behavior-exclusion relations, invariants and finite observations. It preserves disagreement as an open obligation; structural consistency does not prove Rust semantics.

The bounded oracle enumerates common divisors without Euclidean recurrence. Full-width checks use checked Bezout identity, divisibility, coefficient bounds and gcd symmetry; those checks certify each observed gcd independently.

Prospective Euclidean row preservation and widened coefficient model survived bounded checks. Replacing old_x - quotient*x by old_x + quotient*x compiled and tests::bounded_divisor_oracle rejected it (one failure). Terminal zero-row coefficient bounds are distinct from returned-row product bounds; the loose square of u64 maximum would not justify i128 product safety.

## model

# algorithm-extended-gcd-tests

## Forward

Execute the finite assertions against the production module. Pass iff every fixture and relation assertion succeeds under terminating execution. 9,409 bounded pairs (0..=96 squared), 49 extremal pairs, explicit zero conventions, consecutive Fibonacci pairs up to u64 overflow, and 4,096 deterministic full-width pairs.

## Invariant

Fixtures have deterministic order and bounded domains. Oracle construction is independent of the implementation recurrence. A passing finite run is not a universal theorem.

## Cost

Bounded fixture schedule. Divisor enumeration across all bounded fixtures has an O(B^3) upper bound for scalar limit B; full-width and prefix checks have the explicit loop sizes in tests.rs. No asymptotic implementation performance is inferred from test duration.

## nonbehavior

Let F be the declared finite fixtures and U be terminating candidate implementations paired with pass/fail. R(P,pass) iff all assertions hold, and R(P,fail) iff some assertion fails or execution panics. U minus R consists of false pass/fail observations. Harness abort/nontermination is excluded.

## failure_fibers

A pass reconstructs every implementation satisfying all fixtures, with arbitrary off-fixture behavior. A failure reconstructs implementations or faulty oracle assumptions causing the observed mismatch; it does not uniquely identify the defect.

## oracle

The bounded oracle enumerates common divisors without Euclidean recurrence. Full-width checks use checked Bezout identity, divisibility, coefficient bounds and gcd symmetry; those checks certify each observed gcd independently. Three additional tests exercise 20 affine alternatives across five input pairs including zeros/ties, four divisor lattices and 20 bounded linear-solvability searches, five independent residue scans and one full-width u128 inverse check. The coefficient-policy counterexample rejects a valid alternate certificate; modulus/product premises are explicit.

## mutation

Current compiled mutant adds b/g to returned x. The new affine_family_is_not_the_coefficient_policy test rejects it. This mutation targets the coefficient output; the valid shifted-family countermodels separately distinguish identity from exact policy.

## schedule

9,409 bounded pairs (0..=96 squared), 49 extremal pairs, explicit zero conventions, consecutive Fibonacci pairs up to u64 overflow, and 4,096 deterministic full-width pairs. Fixed seeds and bounded loops define repeatable observations; wall-clock timing is not modeled. Three additional tests exercise 20 affine alternatives across five input pairs including zeros/ties, four divisor lattices and 20 bounded linear-solvability searches, five independent residue scans and one full-width u128 inverse check. The coefficient-policy counterexample rejects a valid alternate certificate; modulus/product premises are explicit.

## Limits

Exact implementation; mathematical models are not formal proofs. Partial syntax does not expand macros or resolve types. No numerical approximation profile or unsupported error budget is introduced.

## Current correspondence epoch

Three additional tests exercise 20 affine alternatives across five input pairs including zeros/ties, four divisor lattices and 20 bounded linear-solvability searches, five independent residue scans and one full-width u128 inverse check. The coefficient-policy counterexample rejects a valid alternate certificate; modulus/product premises are explicit.

Current compiled mutant adds b/g to returned x. The new affine_family_is_not_the_coefficient_policy test rejects it. This mutation targets the coefficient output; the valid shifted-family countermodels separately distinguish identity from exact policy.

## evidence

### Evidence

Three tests in this algorithm pass in debug and release. The worktree crate passes 103 unit tests plus one doctest in both modes; strict all-target clippy with -D warnings passes. 9,409 bounded pairs (0..=96 squared), 49 extremal pairs, explicit zero conventions, consecutive Fibonacci pairs up to u64 overflow, and 4,096 deterministic full-width pairs.

The bounded oracle enumerates common divisors without Euclidean recurrence. Full-width checks use checked Bezout identity, divisibility, coefficient bounds and gcd symmetry; those checks certify each observed gcd independently.

No throughput measurement, semantic AST proof, full-system inference, or executable approximation claim is made. Root integration separately registers these leaves and checks parent bindings.

## History

### 2026-10-30 · descriptant

2026-10-30: 9,409 bounded pairs (0..=96 squared), 49 extremal pairs, explicit zero conventions, consecutive Fibonacci pairs up to u64 overflow, and 4,096 deterministic full-width pairs.

#### Relation epoch, 2026-10-30

The combiner now separates full mathematical certificate/preimage relations from implementation selection and machine-resource premises. Implementation families contain eight typed relations; test families retain five with strengthened executed correspondence. Three targeted tests augment each prior suite, preserving earlier useful boundary observations. Finite tests and source projections leave general Rust semantics and allocator exhaustion unproved.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 377b23e294888bdadd98a33d7cc96a10aae96220ea35e8339c4d14c9a0d1bc29.

Replacing old_x - quotient*x by old_x + quotient*x compiled and tests::bounded_divisor_oracle rejected it (one failure). The mutation is applied to copied production source and compiled with rustc --test, not emulated by a different implementation. Logs and the Rust harness are external under MOVA_DATA/runs/octopus-alg3-number-2026-10-30. No huge allocation or actual OOM probe was performed.

#### Relation epoch, 2026-10-30

Current source SHA-256: a6e86038241ed801ad34ec81c215554711e55fef2f2de2ab308acca2bbf827c4. Six tests in this module pass twice in debug and twice in release; single-thread result records match within each mode. Strict all-target Clippy passes. A copied production-source mutant compiles and is rejected by a newly added relation test. Partial syntax regenerated twice with identical bytes. External logs and Rust runner: MOVA_DATA/runs/octopus-next-number-2026-10-30. Earlier evidence above remains historical; no timing or universal semantic claim follows.

Current reproducible evidence SHA-256 bindings:

- extended_gcd-debug-records.txt: d82df702d6539776f163156eb3ad9ed2fb7c319eacc3ad7c17533d8e51b99c1f
- extended_gcd-release-records.txt: d82df702d6539776f163156eb3ad9ed2fb7c319eacc3ad7c17533d8e51b99c1f
- extended_gcd-mutant-build.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
- extended_gcd-mutant-rejected.log: 6f30b78b26ea5aed235ec3f0e39e0bdd162053b9ffbbdb0ba4ad4ba4c9af5432
