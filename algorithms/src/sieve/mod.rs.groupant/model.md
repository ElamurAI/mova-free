# algorithm-sieve

## descriptant

### algorithm-sieve descriptant

The combiner relates source-bound partial syntax, forward recurrence, typed output-preimage and behavior-exclusion relations, invariants and finite observations. It preserves disagreement as an open obligation; structural consistency does not prove Rust semantics.

Primality is tested independently by trial division over every divisor 2..value, without a sieve or square-root stopping rule. Prefix tests check order and inclusivity. Capacity tests reject before any huge allocation request.

Prospective guarded inclusive marking and fully fallible storage model survived bounded checks. Replacing composite clearing flags[multiple]=0 by flags[multiple]=1 compiled and tests::inclusive_trial_divisor_oracle rejected it (one failure). A strict-clippy diagnostic changed the trial divisor spelling to is_multiple_of without changing the oracle. Real allocator exhaustion remains unforced.

## model

# algorithm-sieve

## Forward

For n<2 return an empty vector. Otherwise check n+1 and byte-storage representability. On successful fallible reservations, return exactly the increasing primes <=n by Eratosthenes marking from p squared. SizeOverflow denotes checked count/byte capacity failure; AllocationFailed denotes a reported try_reserve_exact error. Output reservation is fallible as well as flags reservation.

## Invariant

Flags 0 and 1 are cleared. Before processing p, all composites having a prime factor <p have been cleared; clearing multiples of p from p squared preserves all primes and eventually removes every composite. p<=n/p implies p*p<=n without multiplication overflow. Increment occurs only when n-multiple>=p, so multiple+p<=n. Checked count*size_of(T)<=isize::MAX precedes reservation. The reserve helper is private and called on empty vectors; resize and output pushes stay within reserved capacities.

## Cost

O(n log log n) marking time plus O(n) initialization/count/output scan for n>=3; O(n) auxiliary flags plus O(pi(n)) output words. Symbolic cost; no timing claim.

## nonbehavior

In U of limits, allocator traces and candidate results, U minus R includes successful vectors with composites, omissions, duplicates, descending order or values above n; success after deterministic size rejection; and typed allocation failure without a reported reserve failure. Abort/nontermination is outside the modeled terminating Result universe.

## prime_prefix_fibers

A successful vector P admits all representable limits n for which P is exactly the inclusive prime prefix and allocation succeeds. Empty P admits n=0,1. For a nonempty prime prefix ending at p, limits range from p through one below the next prime, intersected with capacity constraints. Error observations retain size-rejection or allocator-failure antecedents; AllocationFailed does not identify a unique limit.

## prefix_relation

For successful calls with m<=n, sieve(m) is sieve(n) restricted to entries <=m. Resource success at one size does not imply success at another under arbitrary allocator state.

## square_boundary

For prime p with representable p*p, p*p is composite and omitted, whereas p itself is retained. The p<=n/p guard includes equality.

## resource_boundary

n=usize::MAX fails checked addition; n=isize::MAX fails byte capacity for n+1 before reservation. Output usize capacity multiplication and isize bound are checked separately. Real OOM is not forced; allocator failure mapping is source reviewed, process-level aborts excluded.

## Limits

Exact implementation; mathematical models are not formal proofs. Partial syntax does not expand macros or resolve types. No numerical approximation profile or unsupported error budget is introduced.

## factor_witness

For any successfully returned prime prefix at n and each composite v in [2,n], a least divisor d>1 is prime, d<=v/d, and d occurs in the prefix while v does not. Conversely a proper factor 1<d<v with d dividing v certifies exclusion. Values 0 and 1 have separate nonprime reasons; absence alone at a limit smaller than v does not certify compositeness.

## gap_interval_converse

Let P be a complete increasing prime prefix ending at p and q its next prime. A successful sieve(n)=P iff p<=n<q, intersected with size representability and successful reservations. This requires prefix completeness: an arbitrary sorted list of primes ending at p need not have any antecedent. Empty P admits exactly n=0,1. At machine boundaries express next prime mathematically and intersect with usize; allocator traces remain part of the antecedent.

## typed_storage_traces

For n>=2 deterministic gates require n+1<=isize::MAX for u8 flags and pi(n)*size_of(usize)<=isize::MAX for output; arithmetic multiplication/addition must be representable. Passing gates is not a success guarantee: either try_reserve_exact may report AllocationFailed. SizeOverflow and AllocationFailed are distinct observations; a reported allocation error does not reconstruct the failing reservation or n uniquely. The private reserve helper checks count bytes before allocation and preserves an existing vector on deterministic rejection; its production callers start empty.

## evidence

### Evidence

Three tests in this algorithm pass in debug and release. The worktree crate passes 103 unit tests plus one doctest in both modes; strict all-target clippy with -D warnings passes. 385 independent prime-prefix cases for limits 0..=384, explicit 2/3/49 boundaries, a 4,096 limit fixture with 564 primes, 12 prefix checks, and deterministic impossible-capacity rejection.

Primality is tested independently by trial division over every divisor 2..value, without a sieve or square-root stopping rule. Prefix tests check order and inclusivity. Capacity tests reject before any huge allocation request.

No throughput measurement, semantic AST proof, full-system inference, or executable approximation claim is made. Root integration separately registers these leaves and checks parent bindings.

## prospective

### Prospective model

Before implementation: specialize the existing math/src/nt.rs sieve to usize without narrowing, checking limit+1 and all allocation capacities. A byte flag represents each inclusive candidate. Mark multiples beginning at p squared only while p<=limit/p, with subtraction guarding increment. After primes up to p are processed, no multiple of those primes remains flagged. Reserve exact output capacity after counting flags. Expected O(n log log n) time and O(n) space, including linear output scan. Allocation failure is a typed Result, subject to the allocator's try_reserve contract. Probe arithmetic capacity rejection without requesting enormous memory; ordinary OOM is not forced.

## History

### 2026-10-30 · descriptant

2026-10-30: For n<2 return an empty vector. Otherwise check n+1 and byte-storage representability. On successful fallible reservations, return exactly the increasing primes <=n by Eratosthenes marking from p squared. SizeOverflow denotes checked count/byte capacity failure; AllocationFailed denotes a reported try_reserve_exact error. Output reservation is fallible as well as flags reservation.

#### Relation epoch, 2026-10-30

The combiner now separates full mathematical certificate/preimage relations from implementation selection and machine-resource premises. Implementation families contain eight typed relations; test families retain five with strengthened executed correspondence. Three targeted tests augment each prior suite, preserving earlier useful boundary observations. Finite tests and source projections leave general Rust semantics and allocator exhaustion unproved.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 48cc204f2b0765533fa30718cdbf94874b9c5d37b3893da6b738316ec897c2d6.

Replacing composite clearing flags[multiple]=0 by flags[multiple]=1 compiled and tests::inclusive_trial_divisor_oracle rejected it (one failure). The mutation is applied to copied production source and compiled with rustc --test, not emulated by a different implementation. Logs and the Rust harness are external under MOVA_DATA/runs/octopus-alg3-number-2026-10-30. No huge allocation or actual OOM probe was performed.

#### Relation epoch, 2026-10-30

Current source SHA-256: 48cc204f2b0765533fa30718cdbf94874b9c5d37b3893da6b738316ec897c2d6. Six tests in this module pass twice in debug and twice in release; single-thread result records match within each mode. Strict all-target Clippy passes. A copied production-source mutant compiles and is rejected by a newly added relation test. Partial syntax regenerated twice with identical bytes. External logs and Rust runner: MOVA_DATA/runs/octopus-next-number-2026-10-30. Earlier evidence above remains historical; no timing or universal semantic claim follows.

Current reproducible evidence SHA-256 bindings:

- sieve-debug-records.txt: 17da698fa4e55f12876c5a49993a04d3749a522a71ca62c37ce458919dd30b7d
- sieve-release-records.txt: 17da698fa4e55f12876c5a49993a04d3749a522a71ca62c37ce458919dd30b7d
- sieve-mutant-build.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
- sieve-mutant-rejected.log: e37aa094e9dc0a1e33e997dcd947eb298de3966ec0d64ac1b7bf873653367c00

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.
