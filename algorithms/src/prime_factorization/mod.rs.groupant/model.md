# algorithm-prime-factorization

## descriptant

### algorithm-prime-factorization combiner

The combiner joins source-bound partial syntax, forward model, behavior complement first, reconstruction second, invariant and finite tests. Each disagreement is an open obligation; valid leaves alone do not prove wiring. Enumerate every candidate divisor of the original bounded n, certify prime by checking all smaller divisors, then determine valuation by powers dividing the original n. This uses neither shrinking remainder nor sqrt termination. Additional product/canonical certificates and multiplication relations test other observations.

Trial division and unique factorization are classical number theory; see Donald E. Knuth, The Art of Computer Programming, Volume 2, Section 4.5.4 (factoring into primes). Implementation and tests are original.

Epoch memory: At each trial candidate d, original n equals remaining times emitted prime powers. Remaining has no prime divisor below d. Dividing out d preserves the identity. Once d>remaining/d, any remaining>1 is prime. Candidate increments cannot overflow: loop entry bounds d by sqrt(u64::MAX); exponent <=63. Output size <=63. Resource exhaustion is outside the returned Error model.

## model

# algorithm-prime-factorization

## Forward

F(0)=Err(ZeroInput). For 1<=n<=u64::MAX, F(n)=Ok([(p1,e1),...,(pk,ek)]) with strictly increasing primes, positive exponents, and product p_i^e_i=n. The empty product gives F(1)=Ok([]). Fundamental unique factorization makes this mathematical target single-valued.

## Invariant

At each trial candidate d, original n equals remaining times emitted prime powers. Remaining has no prime divisor below d. Dividing out d preserves the identity. Once d>remaining/d, any remaining>1 is prime. Candidate increments cannot overflow: loop entry bounds d by sqrt(u64::MAX); exponent <=63. Output size <=63. Resource exhaustion is outside the returned Error model.

## Cost

Worst-case O(sqrt(n)) word trial divisions, O(log n) output entries, O(1) auxiliary state. Huge primes may be impractical. No throughput measurement.

## nonbehavior

Type: behavior_complement. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

Over the stated input/output universe, U minus graph(F) excludes nonzero errors, accepted zero, composite bases, unsorted or duplicate primes, zero exponents, and any product unequal to n. Product equality alone is insufficient, as [(4,1),(3,1)] at n=12 shows.

## product_preimage

Type: reverse_reconstruction. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

For a canonical factor vector v, the unique compatible input is its mathematical product if <=u64::MAX, including empty v ->1. Noncanonical vectors have empty preimages. Err(ZeroInput) reconstructs exactly 0. Unlike general lossy outputs, this complete canonical output is injective.

## valuation_sum

Type: algebraic_relation. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

For positive a,b with a*b<=u64::MAX, v_p(a*b)=v_p(a)+v_p(b) for every prime p; merge sorted factor vectors by adding exponents. No machine-overflow product is admitted.

## prime_power

Type: boundary_specialization. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

For prime p and e>=1 with p^e<=u64::MAX, F(p^e)=[(p,e)]. Zero is rejected and one has empty support. Composite bases must first be factored.

## divisor_lattice

Type: order_relation. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

For positive a,b, a divides b iff every prime exponent of a is <= the corresponding exponent of b, absent primes having exponent zero. This relates factorization to divisibility, including one.

## certificate

Type: certificate_relation. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

Sorted distinct prime bases, positive exponents and checked exact product equal to n certify the entire canonical output by unique factorization; product alone fails to exclude composite bases.

## count_functions

Type: projection_relation. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

For positive n with factor vector (p,e), the number of positive divisors is product(e+1); distinct prime count is vector length. Products here are mathematical integers; any optional machine projection requires checked arithmetic.

## trial_cutoff

Type: termination_relation. Domain: All u64 inputs and candidate Result<Vec<(u64,u32)>,Error> observations under successful allocation and termination.

For remaining>1 with no prime divisor below d and d>remaining/d, remaining is prime. Equality must still be tested: d*d=remaining at prime squares. This condition avoids forming overflowing d*d.

## Limits

Exact algorithm; finite tests and partial syntax do not prove Rust semantics, expand macros, or resolve types. No approximate executable profile applies to these exact integer outputs. Resource exhaustion and operational timing remain unmodeled.

## evidence

### Evidence

Five tests in this module pass; the temporarily integrated crate passes 177 unit tests twice in debug and twice in release. Single-thread normalized records match within each mode. Strict all-target Clippy and owned-file rustfmt checks pass. A compiled production mutant is rejected at a real assertion. Partial syntax regenerated twice with identical bytes.

Current-checker validation: an isolated Git snapshot of the four owned source/sidecar leaves passes groupants with zero diagnostics, four sidecars, zero missing leaf sidecars; its temporary parent is explicitly one missing group sidecar. changed against the task baseline reports all four changed Rust files registered, none missing, using the external leaf registry. Full parent-tree integration remains the integrating agent's responsibility. An initial historical checker/full-repository minimal-registry probe was rejected for unrelated ownership and old component-schema gaps; it is not counted as validation.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: F(0)=Err(ZeroInput). For 1<=n<=u64::MAX, F(n)=Ok([(p1,e1),...,(pk,ek)]) with strictly increasing primes, positive exponents, and product p_i^e_i=n. The empty product gives F(1)=Ok([]). Fundamental unique factorization makes this mathematical target single-valued.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: e3c96c648dd551b0316db000aea3bbb19d4d257d33129eca41f78d3edf9a0271.

External evidence: MOVA_DATA/runs/octopus-wave4-number-2026-10-30, containing the Rust validation helper, raw logs and mutants. No throughput measurement, live API, teacher, GPU or semantic proof. Root integration separately binds parent groups and registry.

- debug-records.txt SHA-256: fa902d23abe56879ca09d425ce49c7eabfedb520d9356ce92da1e8e1c22e11cb

- release-records.txt SHA-256: fa902d23abe56879ca09d425ce49c7eabfedb520d9356ce92da1e8e1c22e11cb

- prime_factorization-mutant-build.log SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- prime_factorization-mutant-rejected.log SHA-256: 360c3c6af7c1648c91603b1f45754f0523a6c9693bfee5612838a8ec1d848c2b

- syntax.json SHA-256: bf9b8ae4a86e0a65566a4ce4cb4c336cb90c0b2ea7d8c9f5a7699c7272d48055

### 2026-10-30 · prospective

Before code, 2026-10-30: trial division maintains n = remaining * product(p^e), emitted primes ascending, and remaining has no divisor below current candidate. Terminate candidate > remaining/candidate; remaining > 1 is prime. Bound O(sqrt(n)) word divisions, O(log n) output; no huge-prime throughput claim. Planned oracle enumerates every divisor and independently tests primality on n<=2048. Zero rejects, one empty; product uniquely reconstructs accepted n.
