# algorithm-totient-sieve

## descriptant

### algorithm-totient-sieve

Predicted cost: O(n log log n) arithmetic and O(n) storage. GCD oracle through2048 has O(N^2 log N) upper bound. Allocation failure follows Vec policy.

Compiled controls are rejected and exact bytes restored. Six file roots contain36 typed views, no fragments. Twice-identical partial syntax binds source; macros remain opaque. No semantic AST, formal proof, approximate surrogate or speed claim.

## model

# algorithm-totient-sieve

## Forward

Return inclusive phi table, phi(0)=0, phi(1)=1; phi(k) counts integers1..k coprime to k. Initialize phi(k)=k, then for each prime p eliminate its factor by phi(multiple)-=phi(multiple)/p. Reject n>1000000 before allocation. Reference: Hardy and Wright, An Introduction to the Theory of Numbers, chapter V, Euler's function and its product formula.

## Invariant

Entries equal k times product(1-1/p) over processed prime divisors; updates divide exactly. 0<=phi(k)<=k; cap bounds indices.

## Cost

O(n log log n) arithmetic and O(n) storage. GCD oracle through2048 has O(N^2 log N) upper bound. Allocation failure follows Vec policy.

## behavior_complement

Universe usize x Result<table,error>. Wrong lengths, wrong conventions and entries unequal to coprime counts are excluded for accepted n. For n>MAX_N all results except LimitExceeded carrying the requested n and MAX_N are excluded. This is a source-derived specification target, not a proof.

## reverse_reconstruction

Full successful table length recovers n uniquely. Scalar phi alone has ambiguous fibers: phi(3)=phi(4)=phi(6)=2. Error payload recovers n; projected error tag retains all n>MAX_N.

## coprime_oracle

GCD counting independently checks each n=1..2048 without prime-factor elimination.

## divisor_identity

For n>=1, sum_{d|n} phi(d)=n; tests check n<=512, partitioning integers by gcd.

## resource_partition

n=0,1 and MAX_N accepted; phi(MAX_N)=400000. MAX_N+1 and usize::MAX rejected before allocation.

## prime_update

phi(p)=p identifies prime p>=2. Omitting subtraction compiles and fails exhaustive_gcd_oracle.

## evidence

### Evidence

Nine tests total, three per algorithm, pass twice debug and twice release with identical normalized outcomes. Three compiled production controls fail exhaustive_gcd_oracle, exhaustive_factor_oracle and exhaustive_iterative_oracle; exact bytes restored before positives. Strict all-target Clippy and narrow formatting pass. External wave16 numeric bundle retains logs. Finite checks do not establish universal correctness or allocator success.

## History

### 2026-10-30 · descriptant

Epoch2026-10-30: original implementation and independent fixtures. Prospective relation: Return inclusive phi table, phi(0)=0, phi(1)=1; phi(k) counts integers1..k coprime to k. Initialize phi(k)=k, then for each prime p eliminate its factor by phi(multiple)-=phi(multiple)/p. Reject n>1000000 before allocation. Reference: Hardy and Wright, An Introduction to the Theory of Numbers, chapter V, Euler's function and its product formula.

### Undated · evidence

Source SHA-256: beea59a6e9d685e2b414cc4eca4fddb21d0087f7b9957d592763f085ec680ef6.

- debug-1-totient_sieve.log: e061a404779be971019b2d63adca6e2c027ce73d7e8d21793469286d388f5417

- debug-2-totient_sieve.log: a5e3a4fd4e68a9214222cc13e5c745d044da9db6de2b55a4571b0bf1c9d2fe1e

- release-1-totient_sieve.log: 5ec9a2f63ebd17b9db456d372c83fb6b552acb2c28cb3c6c476f528f0d2f6316

- release-2-totient_sieve.log: d71671b64a4564f68bf9688908592de260750274d4ba15bd5fbc3037c4fd176e

- mutant-totient_sieve.log: 8ac9bdc98245338db9d3d3ad551316f75d88b078eb8b49948ea1057b63e7f9a5
