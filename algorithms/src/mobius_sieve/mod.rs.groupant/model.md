# algorithm-mobius-sieve

## descriptant

### algorithm-mobius-sieve

Predicted cost: O(n) sieve time and storage. Independent trial factorization through10000 has O(N sqrt N) upper bound. Allocation failure follows Vec policy.

Compiled controls are rejected and exact bytes restored. Six file roots contain36 typed views, no fragments. Twice-identical partial syntax binds source; macros remain opaque. No semantic AST, formal proof, approximate surrogate or speed claim.

## model

# algorithm-mobius-sieve

## Forward

Return inclusive mu table with mu(0)=0,mu(1)=1. Positive square-divisible k maps0; otherwise (-1)^r for r distinct prime factors. Linear sieve assigns each composite by least prime: p divides i => mu(ip)=0 and stop; else mu(ip)=-mu(i). Reject n>1000000 before allocation. Reference: Hardy and Wright, An Introduction to the Theory of Numbers, chapter XVI, arithmetical functions.

## Invariant

Primes ascend; each composite assigned once by least prime. Values in {-1,0,1}; p<=n/i checked before product. Divisible branch stops duplicate generation.

## Cost

O(n) sieve time and storage. Independent trial factorization through10000 has O(N sqrt N) upper bound. Allocation failure follows Vec policy.

## behavior_complement

Universe usize x Result<table,error>. Wrong length, conventions, square-divisibility zeros or squarefree parity are absent on accepted n. Rejected n excludes all success and all mismatched error payloads. Specification target relies on the stated sieve invariant.

## reverse_reconstruction

Full table length recovers n. Scalar mu=0 retains square-divisible positives plus0; mu=1 retains1 and even-factor squarefree inputs; mu=-1 retains odd-factor squarefree inputs. Error tag alone retains every n>MAX_N.

## factor_oracle

Trial division and repeated-factor detection independently compare every n=0..10000, without sieve prime discovery.

## divisor_identity

sum_{d|n} mu(d)=1 for n=1 and0 for n>1. Tests check n<=512; no n=0 extension.

## resource_partition

Zero and one are valid; MAX_N accepted with mu(MAX_N)=0; MAX_N+1 and usize::MAX rejected before allocation.

## least_prime

The first prime dividing i stops generation. Changing the other branch sign compiles and fails exhaustive_factor_oracle.

## evidence

### Evidence

Nine tests total, three per algorithm, pass twice debug and twice release with identical normalized outcomes. Three compiled production controls fail exhaustive_gcd_oracle, exhaustive_factor_oracle and exhaustive_iterative_oracle; exact bytes restored before positives. Strict all-target Clippy and narrow formatting pass. External wave16 numeric bundle retains logs. Finite checks do not establish universal correctness or allocator success.

## History

### 2026-10-30 · descriptant

Epoch2026-10-30: original implementation and independent fixtures. Prospective relation: Return inclusive mu table with mu(0)=0,mu(1)=1. Positive square-divisible k maps0; otherwise (-1)^r for r distinct prime factors. Linear sieve assigns each composite by least prime: p divides i => mu(ip)=0 and stop; else mu(ip)=-mu(i). Reject n>1000000 before allocation. Reference: Hardy and Wright, An Introduction to the Theory of Numbers, chapter XVI, arithmetical functions.

### Undated · evidence

Source SHA-256: e386d4d21b45b14753dba68639bc35fbe8d7a9d61464abddc20c6fe801a34c46.

- debug-1-mobius_sieve.log: a43ee28acac72986c7fd16ecbc4a596e3717dc7b69bfb7422e9c2e14bc3d0ad2

- debug-2-mobius_sieve.log: b9c19a1fe2b12b1ec42cc51c2c800b181cb2d353dea895afefbf7d729c5de96b

- release-1-mobius_sieve.log: 3d751bb23d4c0820235ddce02784b0e24752f5a9266a25ff1ddfdeea674bf6c1

- release-2-mobius_sieve.log: 8044f21edb7c039f930ccab6a3056ca36b8bede55c572fd62239f44f32cda1c6

- mutant-mobius_sieve.log: 4a1888786fe96054fd5e7a379fc033d76390b133291b03d0fdb8b21343cb0f26
