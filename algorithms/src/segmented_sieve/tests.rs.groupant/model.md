# algorithm-segmented-sieve-tests

## descriptant

### segmented_sieve tests.rs

Execute five tests, requiring every assertion to hold. Trial division tries every integer divisor through n/d; it uses neither base sieving nor production primality. Bounded intervals, singleton endpoints and the known million-prime count supplement it.

For pollard_rho the initial test incorrectly assumed n=9 needs more than one iteration; actual first iteration finds divisor three. The exhaustion witness was corrected to n=25,budget=1. Production logic was unchanged by that correction.

## model

# segmented_sieve tests.rs model

## Domain

Five tests: every low=0..79 and high=low..99; exact width cap and prime count 78498 below one million; prime squares through 997^2; trial division on the last 101 values below 10^12; six interval concatenations; empty and nonempty output fibers.

## Forward

Execute five tests, requiring every assertion to hold. Trial division tries every integer divisor through n/d; it uses neither base sieving nor production primality. Bounded intervals, singleton endpoints and the known million-prime count supplement it.

## Invariant

Independent oracles do not call the implementation under test. Full-width product reconstruction using production primality is only a consistency check; independent small trial oracles are separate.

## Cost

Finite fixture harness only; oracle loops are explicitly bounded by the source fixtures. Production cost: Fresh base sieve costs O(sqrt(high) loglog(high)+sqrt(high)), interval marking O(width loglog(high)+sqrt(high)), enumeration O(width). Storage O(sqrt(high)+width+output), all bounded by endpoint/width caps. Use loglog(max(high,3)) for small endpoints.

## nonbehavior

Type: behavior_complement. Over terminating candidate implementations and pass/fail observations, exclude pass when any assertion fails and failure when all assertions hold. Nontermination and abort are outside this binary observation domain.

## implementation_fibers

Type: reverse_reconstruction. Pass reconstructs the set of implementations agreeing on every observed fixture; off-fixture behavior is unconstrained. Failure reconstructs implementations or incorrect oracle premises that cause a mismatch, not a unique defect.

## oracle

Type: oracle_correspondence. Trial division tries every integer divisor through n/d; it uses neither base sieving nor production primality. Bounded intervals, singleton endpoints and the known million-prime count supplement it.

## mutation

Type: counterfactual_boundary. Change output threshold >=2 to >=3; boundaries_and_limits must fail for singleton prime two.

## replay

Type: metamorphic_relation. Single-thread runs twice each in debug and release must have identical ordered normalized test records. Build output and timing are excluded. Finite agreement is not universal equivalence.

## Composition and gaps

The source is a small leaf; its validation, core arithmetic and result assembly compose under the invariant above. Tests observe the public operation plus any private helper explicitly named by tests. Partial syntax does not expand macros or provide a semantic AST. No useful certified approximate metric is supplied for exact discrete outputs; executable numerical approximants are an explicit gap. No performance benchmark or formal proof is claimed.

## evidence

### Evidence

Five tests: every low=0..79 and high=low..99; exact width cap and prime count 78498 below one million; prime squares through 997^2; trial division on the last 101 values below 10^12; six interval concatenations; empty and nonempty output fibers.

Change output threshold >=2 to >=3; boundaries_and_limits must fail for singleton prime two.

Syntax extraction is run twice and compared exactly; syntax.json is a partial Rust syntax projection, not semantic AST analysis. Root integration supplies registry validation; this branch does not modify the shared registry.

## prospective

### Prospective model

Domain inclusive 0<=low<=high<=10^12 and high-low+1<=10^6. Validate reversed, high, width in that order before allocation. Build base sieve through integer floor sqrt(high), then mark interval multiples from max(p*p,ceil(low/p)*p). Invariant surviving interval entries have no processed prime divisor; 0 and 1 excluded. Time O(sqrt(high) loglog(high)+width loglog(high)+sqrt(high)+width); storage O(sqrt(high)+width+output). Complement excludes composites, duplicates and missing primes. Reverse fibers are intervals containing exactly returned primes, possibly empty gaps. Singleton, interval concatenation, square boundaries and resource boundary specialize the model. Allocation failure is outside the normal-return model; bounds are not a guarantee of allocator success.

## root-wave18

### Root integration refinement

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model retained before implementation. Independent bounded oracles, extreme arithmetic fixtures and compiled intentional mutations test the claim. Five owned tests pass twice each in debug and release, with identical normalized records. Finite results do not prove all inputs.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `8e444fe32d755ebea114409cc7891a6420c47bd9ba786f92dff0aba4b6a57294`. Five tests per algorithm pass twice in debug and twice in release; normalized records compared by a Rust driver. Strict library Clippy and owned-file formatting pass. One compiled production mutation per algorithm fails a named assertion and restored bytes pass the positive suite.

External evidence under `$MOVA_DATA/runs/octopus-wave18-numbers-2026-10-30/`:

- `segmented_sieve-debug-1.log`: SHA-256 `be05226b58645ef26e6179185f2bd41460c04dad97862a30cfaa2817f7bb1e90`.

- `segmented_sieve-debug-2.log`: SHA-256 `1200e74a9ffa5e1f82d0828dc7bf93818b5b27ba3250eb955da2211f052ac91d`.

- `segmented_sieve-release-1.log`: SHA-256 `2a4c763b9ea96a1d77a540ba72e067214977a0567eff7f929c95a009cd73c9de`.

- `segmented_sieve-release-2.log`: SHA-256 `aae0f58856668c832898d467ba2af8721e7da6f145208de339f55f99be4529a7`.

- `segmented_sieve-mutant.log`: SHA-256 `f9396b6b33100156a0035e3dce76e1f2bfae190ea2cde1a1fec76abbd6ffd4cf`.

- `segmented_sieve-restored.log`: SHA-256 `86245cee05c896be04aad2c74b9db17448f96eca066a8b17311d5eb2349bcca2`.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-10-30 · root-wave18

Heading `### Root integration refinement — 2026-10-30` stays in root-wave18 as `### Root integration refinement`.

Strict all-target Clippy exposed a test-only style issue after the owner library-only check. The source now uses the equivalent standard slice/divisibility helper. Old test source SHA-256: 8e444fe32d755ebea114409cc7891a6420c47bd9ba786f92dff0aba4b6a57294; current SHA-256: e4b837f8779b5c8d84908166ac1136485a87cc7c92f36c5850121f8d26ba03f9. Owner receipts remain historical and are not rewritten. Final root execution is recorded in models/algorithm-library.groupant/model.md#wave18-evidence; raw receipts are under $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30/native-final. Syntax was projected twice identically after this edit. No production algorithm changed.

### Undated · descriptant

Source SHA-256: `e4b837f8779b5c8d84908166ac1136485a87cc7c92f36c5850121f8d26ba03f9`.
