# algorithm-segmented-sieve

## descriptant

### segmented_sieve mod.rs

Validate reversed interval, then high cap, then width cap. Integer binary search computes floor sqrt(high). Base flags identify primes through that limit. For each base prime mark interval multiples from max(p*p,ceil(low/p)*p). Enumerate surviving values >=2 in ascending order.

For pollard_rho the initial test incorrectly assumed n=9 needs more than one iteration; actual first iteration finds divisor three. The exhaustion witness was corrected to n=25,budget=1. Production logic was unchanged by that correction.

## model

# segmented_sieve mod.rs model

## Domain

Inclusive u64 intervals and Result<Vec<u64>,SieveError>; accepted low<=high<=10^12 with width<=10^6.

## Forward

Validate reversed interval, then high cap, then width cap. Integer binary search computes floor sqrt(high). Base flags identify primes through that limit. For each base prime mark interval multiples from max(p*p,ceil(low/p)*p). Enumerate surviving values >=2 in ascending order.

## Invariant

Every composite up to high has a prime divisor at most sqrt(high). Base sieve marks composites before visiting their index. After each processed prime p, segment flags exclude exactly its multiples at or above p*p and all earlier marked multiples. Starting at p*p preserves p itself.

## Cost

Fresh base sieve costs O(sqrt(high) loglog(high)+sqrt(high)), interval marking O(width loglog(high)+sqrt(high)), enumeration O(width). Storage O(sqrt(high)+width+output), all bounded by endpoint/width caps. Use loglog(max(high,3)) for small endpoints.

## nonbehavior

Type: behavior_complement. Excluded successful outputs contain any composite, omit a prime in the interval, include values outside the interval, or have duplicate/unsorted values. Rejected-domain success and wrong validation precedence are excluded.

## interval_fibers

Type: reverse_reconstruction. For a nonempty output list, compatible accepted intervals contain all those primes and no additional primes. Their endpoints can move in adjacent composite gaps. Empty outputs reconstruct prime-free accepted intervals, not a unique interval: [14,16] and [24,28]. Error fibers follow reversed/high/width precedence.

## partition

Type: metamorphic_relation. Splitting an accepted [l,r] at k with l<=k<r gives output([l,k]) concatenated with output([k+1,r]); both subintervals satisfy caps. No deduplication is needed.

## square_frontier

Type: boundary_specialization. At p*p for a prime p the new base divisor must be included. Integer sqrt ensures perfect-square endpoints are marked correctly without floating rounding. p itself survives because marking starts at p*p.

## composite_witness

Type: certificate_relation. Every removed value >=2 is a multiple k*p with k>=p for a discovered base prime p. Every surviving value >=2 has no prime divisor <=sqrt(high), proving primality on the stated domain.

## resource_preflight

Type: resource_relation. Reversed order wins before high cap before width cap; no allocation occurs before these checks. Caps limit base and segment flags to 1000001 and 1000000 entries; output length<=width. This is not a recoverable allocator-failure guarantee.

## Composition and gaps

The source is a small leaf; its validation, core arithmetic and result assembly compose under the invariant above. Tests observe the public operation plus any private helper explicitly named by tests. Partial syntax does not expand macros or provide a semantic AST. No useful certified approximate metric is supplied for exact discrete outputs; executable numerical approximants are an explicit gap. No performance benchmark or formal proof is claimed.

## evidence

### Evidence

Five tests: every low=0..79 and high=low..99; exact width cap and prime count 78498 below one million; prime squares through 997^2; trial division on the last 101 values below 10^12; six interval concatenations; empty and nonempty output fibers.

Change output threshold >=2 to >=3; boundaries_and_limits must fail for singleton prime two.

Syntax extraction is run twice and compared exactly; syntax.json is a partial Rust syntax projection, not semantic AST analysis. Root integration supplies registry validation; this branch does not modify the shared registry.

## prospective

### Groupant prospective

#### Prospective model

Domain inclusive 0<=low<=high<=10^12 and high-low+1<=10^6. Validate reversed, high, width in that order before allocation. Build base sieve through integer floor sqrt(high), then mark interval multiples from max(p*p,ceil(low/p)*p). Invariant surviving interval entries have no processed prime divisor; 0 and 1 excluded. Time O(sqrt(high) loglog(high)+width loglog(high)+sqrt(high)+width); storage O(sqrt(high)+width+output). Complement excludes composites, duplicates and missing primes. Reverse fibers are intervals containing exactly returned primes, possibly empty gaps. Singleton, interval concatenation, square boundaries and resource boundary specialize the model. Allocation failure is outside the normal-return model; bounds are not a guarantee of allocator success.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model retained before implementation. Independent bounded oracles, extreme arithmetic fixtures and compiled intentional mutations test the claim. Five owned tests pass twice each in debug and release, with identical normalized records. Finite results do not prove all inputs.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `9c069e4fe604cba89bb44dd5278f86664de60489a8d72d736a1af1b14c14c434`. Five tests per algorithm pass twice in debug and twice in release; normalized records compared by a Rust driver. Strict library Clippy and owned-file formatting pass. One compiled production mutation per algorithm fails a named assertion and restored bytes pass the positive suite.

External evidence under `$MOVA_DATA/runs/octopus-wave18-numbers-2026-10-30/`:

- `segmented_sieve-debug-1.log`: SHA-256 `be05226b58645ef26e6179185f2bd41460c04dad97862a30cfaa2817f7bb1e90`.

- `segmented_sieve-debug-2.log`: SHA-256 `1200e74a9ffa5e1f82d0828dc7bf93818b5b27ba3250eb955da2211f052ac91d`.

- `segmented_sieve-release-1.log`: SHA-256 `2a4c763b9ea96a1d77a540ba72e067214977a0567eff7f929c95a009cd73c9de`.

- `segmented_sieve-release-2.log`: SHA-256 `aae0f58856668c832898d467ba2af8721e7da6f145208de339f55f99be4529a7`.

- `segmented_sieve-mutant.log`: SHA-256 `f9396b6b33100156a0035e3dce76e1f2bfae190ea2cde1a1fec76abbd6ffd4cf`.

- `segmented_sieve-restored.log`: SHA-256 `86245cee05c896be04aad2c74b9db17448f96eca066a8b17311d5eb2349bcca2`.

### 2026-10-30 · prospective

Heading `#### Prospective model — 2026-10-30` stays in prospective as `#### Prospective model`.

#### Module prospective

##### Prospective model — 2026-10-30

Domain inclusive 0<=low<=high<=10^12 and high-low+1<=10^6. Validate reversed, high, width in that order before allocation. Build base sieve through integer floor sqrt(high), then mark interval multiples from max(p*p,ceil(low/p)*p). Invariant surviving interval entries have no processed prime divisor; 0 and 1 excluded. Time O(sqrt(high) loglog(high)+width loglog(high)+sqrt(high)+width); storage O(sqrt(high)+width+output). Complement excludes composites, duplicates and missing primes. Reverse fibers are intervals containing exactly returned primes, possibly empty gaps. Singleton, interval concatenation, square boundaries and resource boundary specialize the model. Allocation failure is outside the normal-return model; bounds are not a guarantee of allocator success.

### Undated · descriptant

Source SHA-256: `9c069e4fe604cba89bb44dd5278f86664de60489a8d72d736a1af1b14c14c434`.
