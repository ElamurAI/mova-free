# algorithm-sieve-tests

## descriptant

### algorithm-sieve-tests descriptant

The combiner relates source-bound partial syntax, forward recurrence, typed output-preimage and behavior-exclusion relations, invariants and finite observations. It preserves disagreement as an open obligation; structural consistency does not prove Rust semantics.

Primality is tested independently by trial division over every divisor 2..value, without a sieve or square-root stopping rule. Prefix tests check order and inclusivity. Capacity tests reject before any huge allocation request.

Prospective guarded inclusive marking and fully fallible storage model survived bounded checks. Replacing composite clearing flags[multiple]=0 by flags[multiple]=1 compiled and tests::inclusive_trial_divisor_oracle rejected it (one failure). A strict-clippy diagnostic changed the trial divisor spelling to is_multiple_of without changing the oracle. Real allocator exhaustion remains unforced.

## model

# algorithm-sieve-tests

## Forward

Execute the finite assertions against the production module. Pass iff every fixture and relation assertion succeeds under terminating execution. 385 independent prime-prefix cases for limits 0..=384, explicit 2/3/49 boundaries, a 4,096 limit fixture with 564 primes, 12 prefix checks, and deterministic impossible-capacity rejection.

## Invariant

Fixtures have deterministic order and bounded domains. Oracle construction is independent of the implementation recurrence. A passing finite run is not a universal theorem.

## Cost

Bounded fixture schedule. Divisor enumeration across all bounded fixtures has an O(B^3) upper bound for scalar limit B; full-width and prefix checks have the explicit loop sizes in tests.rs. No asymptotic implementation performance is inferred from test duration.

## nonbehavior

Let F be the declared finite fixtures and U be terminating candidate implementations paired with pass/fail. R(P,pass) iff all assertions hold, and R(P,fail) iff some assertion fails or execution panics. U minus R consists of false pass/fail observations. Harness abort/nontermination is excluded.

## failure_fibers

A pass reconstructs every implementation satisfying all fixtures, with arbitrary off-fixture behavior. A failure reconstructs implementations or faulty oracle assumptions causing the observed mismatch; it does not uniquely identify the defect.

## oracle

Primality is tested independently by trial division over every divisor 2..value, without a sieve or square-root stopping rule. Prefix tests check order and inclusivity. Capacity tests reject before any huge allocation request. Three additional tests exercise eleven independent composite-factor witnesses at 1024 and their own inclusive limits plus every returned prime at 1024, five complete prime-gap fibers including both excluded endpoints and missing-prefix countermodels, and typed capacity rejection preserving preexisting vectors. Actual allocator exhaustion remains unforced.

## mutation

Current compiled mutant changes the square guard from <= to <. The new composite_witnesses_partition_the_prefix test rejects the retained square 4 at its inclusive limit.

## schedule

385 independent prime-prefix cases for limits 0..=384, explicit 2/3/49 boundaries, a 4,096 limit fixture with 564 primes, 12 prefix checks, and deterministic impossible-capacity rejection. Fixed seeds and bounded loops define repeatable observations; wall-clock timing is not modeled. Three additional tests exercise eleven independent composite-factor witnesses at 1024 and their own inclusive limits plus every returned prime at 1024, five complete prime-gap fibers including both excluded endpoints and missing-prefix countermodels, and typed capacity rejection preserving preexisting vectors. Actual allocator exhaustion remains unforced.

## Limits

Exact implementation; mathematical models are not formal proofs. Partial syntax does not expand macros or resolve types. No numerical approximation profile or unsupported error budget is introduced.

## Current correspondence epoch

Three additional tests exercise eleven independent composite-factor witnesses at 1024 and their own inclusive limits plus every returned prime at 1024, five complete prime-gap fibers including both excluded endpoints and missing-prefix countermodels, and typed capacity rejection preserving preexisting vectors. Actual allocator exhaustion remains unforced.

Current compiled mutant changes the square guard from <= to <. The new composite_witnesses_partition_the_prefix test rejects the retained square 4 at its inclusive limit.

## evidence

### Evidence

Three tests in this algorithm pass in debug and release. The worktree crate passes 103 unit tests plus one doctest in both modes; strict all-target clippy with -D warnings passes. 385 independent prime-prefix cases for limits 0..=384, explicit 2/3/49 boundaries, a 4,096 limit fixture with 564 primes, 12 prefix checks, and deterministic impossible-capacity rejection.

Primality is tested independently by trial division over every divisor 2..value, without a sieve or square-root stopping rule. Prefix tests check order and inclusivity. Capacity tests reject before any huge allocation request.

No throughput measurement, semantic AST proof, full-system inference, or executable approximation claim is made. Root integration separately registers these leaves and checks parent bindings.

## History

### 2026-10-30 · descriptant

2026-10-30: 385 independent prime-prefix cases for limits 0..=384, explicit 2/3/49 boundaries, a 4,096 limit fixture with 564 primes, 12 prefix checks, and deterministic impossible-capacity rejection.

#### Relation epoch, 2026-10-30

The combiner now separates full mathematical certificate/preimage relations from implementation selection and machine-resource premises. Implementation families contain eight typed relations; test families retain five with strengthened executed correspondence. Three targeted tests augment each prior suite, preserving earlier useful boundary observations. Finite tests and source projections leave general Rust semantics and allocator exhaustion unproved.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: e8debe962a90a672b0455031f3fb9c5f0e80f97ddff883a9f6fc031c50d03374.

Replacing composite clearing flags[multiple]=0 by flags[multiple]=1 compiled and tests::inclusive_trial_divisor_oracle rejected it (one failure). The mutation is applied to copied production source and compiled with rustc --test, not emulated by a different implementation. Logs and the Rust harness are external under MOVA_DATA/runs/octopus-alg3-number-2026-10-30. No huge allocation or actual OOM probe was performed.

#### Relation epoch, 2026-10-30

Current source SHA-256: 0353e3b10ce8e9af4107f0a13ab9034e282257a93c868eb37779616a2e9cea5f. Six tests in this module pass twice in debug and twice in release; single-thread result records match within each mode. Strict all-target Clippy passes. A copied production-source mutant compiles and is rejected by a newly added relation test. Partial syntax regenerated twice with identical bytes. External logs and Rust runner: MOVA_DATA/runs/octopus-next-number-2026-10-30. Earlier evidence above remains historical; no timing or universal semantic claim follows.

Current reproducible evidence SHA-256 bindings:

- sieve-debug-records.txt: 17da698fa4e55f12876c5a49993a04d3749a522a71ca62c37ce458919dd30b7d
- sieve-release-records.txt: 17da698fa4e55f12876c5a49993a04d3749a522a71ca62c37ce458919dd30b7d
- sieve-mutant-build.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
- sieve-mutant-rejected.log: e37aa094e9dc0a1e33e997dcd947eb298de3966ec0d64ac1b7bf873653367c00
