# algorithm-prime-factorization-tests

## descriptant

### algorithm-prime-factorization-tests combiner

Behavior complement models false reported outcomes; reverse reconstruction retains ambiguity of both pass and fail. Disagreement remains an obligation, never resolved by a registry label. Epoch observation: boundaries and independent oracles detect a compiled production mutation; successful finite checks leave off-fixture behavior unproved.

## model

# algorithm-prime-factorization-tests

## Forward

Report pass iff all fixed finite assertions hold; otherwise fail. Five tests: independent divisor/primality oracle n=1..2048; zero/one and eight prime squares; u64::MAX known factors, powers of two e=1..63 and 3^40; reconstruction/canonical certificates n=1..4096; valuation addition for 4096 pairs a,b=1..64.

## Invariant

Finite fixtures and independent oracle arithmetic stay within bounded integer domains. Passing tests leave all unobserved behavior open.

## Cost

Finite bounded enumeration of divisors or residues, symbolic unmeasured cost.

## nonbehavior

Type: behavior_complement. Domain: Five tests: independent divisor/primality oracle n=1..2048; zero/one and eight prime squares; u64::MAX known factors, powers of two e=1..63 and 3^40; reconstruction/canonical certificates n=1..4096; valuation addition for 4096 pairs a,b=1..64.

Over terminating candidate implementations paired with pass/fail, excluded observations comprise pass with any failed assertion and fail with every assertion satisfied. Aborts/nontermination are outside this binary domain.

## failure_fibers

Type: reverse_reconstruction. Domain: Five tests: independent divisor/primality oracle n=1..2048; zero/one and eight prime squares; u64::MAX known factors, powers of two e=1..63 and 3^40; reconstruction/canonical certificates n=1..4096; valuation addition for 4096 pairs a,b=1..64.

A pass reconstructs all implementations agreeing with every finite fixture, leaving arbitrary off-fixture behavior. Failure reconstructs implementations or faulty oracle premises causing the mismatch, never uniquely identifies a defect.

## oracle

Type: oracle_correspondence. Domain: Five tests: independent divisor/primality oracle n=1..2048; zero/one and eight prime squares; u64::MAX known factors, powers of two e=1..63 and 3^40; reconstruction/canonical certificates n=1..4096; valuation addition for 4096 pairs a,b=1..64.

Enumerate every candidate divisor of the original bounded n, certify prime by checking all smaller divisors, then determine valuation by powers dividing the original n. This uses neither shrinking remainder nor sqrt termination. Additional product/canonical certificates and multiplication relations test other observations.

## mutation

Type: counterfactual_boundary. Domain: Five tests: independent divisor/primality oracle n=1..2048; zero/one and eight prime squares; u64::MAX known factors, powers of two e=1..63 and 3^40; reconstruction/canonical certificates n=1..4096; valuation addition for 4096 pairs a,b=1..64.

Change trial boundary <= to <. This compiled production-source mutant loses the exponent of 2 at n=4 and is rejected by zero_one_and_prime_squares.

## schedule

Type: metamorphic_relation. Domain: Five tests: independent divisor/primality oracle n=1..2048; zero/one and eight prime squares; u64::MAX known factors, powers of two e=1..63 and 3^40; reconstruction/canonical certificates n=1..4096; valuation addition for 4096 pairs a,b=1..64.

Single-thread repeated debug/release runs preserve normalized test records, excluding timestamps, build progress and elapsed time. This is finite evidence, not universal equivalence.

## Limits

Partial syntax is not semantic AST analysis. Finite observations are not proof; no executable approximation claim.

## evidence

### Evidence

Five tests in this module pass; the temporarily integrated crate passes 177 unit tests twice in debug and twice in release. Single-thread normalized records match within each mode. Strict all-target Clippy and owned-file rustfmt checks pass. A compiled production mutant is rejected at a real assertion. Partial syntax regenerated twice with identical bytes.

Current-checker validation: an isolated Git snapshot of the four owned source/sidecar leaves passes groupants with zero diagnostics, four sidecars, zero missing leaf sidecars; its temporary parent is explicitly one missing group sidecar. changed against the task baseline reports all four changed Rust files registered, none missing, using the external leaf registry. Full parent-tree integration remains the integrating agent's responsibility. An initial historical checker/full-repository minimal-registry probe was rejected for unrelated ownership and old component-schema gaps; it is not counted as validation.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: combine the fixed harness source, independent reference observations, invariants and typed relations. Enumerate every candidate divisor of the original bounded n, certify prime by checking all smaller divisors, then determine valuation by powers dividing the original n. This uses neither shrinking remainder nor sqrt termination. Additional product/canonical certificates and multiplication relations test other observations.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: ebd06d8fbe790fced807c1e67b813a489e66f122e23654134af4a89accf2273d.

External evidence: MOVA_DATA/runs/octopus-wave4-number-2026-10-30, containing the Rust validation helper, raw logs and mutants. No throughput measurement, live API, teacher, GPU or semantic proof. Root integration separately binds parent groups and registry.

- debug-records.txt SHA-256: fa902d23abe56879ca09d425ce49c7eabfedb520d9356ce92da1e8e1c22e11cb

- release-records.txt SHA-256: fa902d23abe56879ca09d425ce49c7eabfedb520d9356ce92da1e8e1c22e11cb

- prime_factorization-mutant-build.log SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- prime_factorization-mutant-rejected.log SHA-256: 360c3c6af7c1648c91603b1f45754f0523a6c9693bfee5612838a8ec1d848c2b

- syntax.json SHA-256: d727e3c35ef629d848b487541da44ec23a48518f286f344aac02068a85952b95

### 2026-10-30 · prospective

#### Before execution

2026-10-30: inherited pre-implementation plan in the adjacent implementation groupant. Five tests: independent divisor/primality oracle n=1..2048; zero/one and eight prime squares; u64::MAX known factors, powers of two e=1..63 and 3^40; reconstruction/canonical certificates n=1..4096; valuation addition for 4096 pairs a,b=1..64. Expect deterministic identical normalized records in debug/release and a real assertion failure for Change trial boundary <= to <. This compiled production-source mutant loses the exponent of 2 at n=4 and is rejected by zero_one_and_prime_squares.
