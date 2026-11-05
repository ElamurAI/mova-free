# algorithm-floor-sum-tests

## descriptant

### floor_sum tests.rs

Execute five tests, requiring every assertion to hold. Direct independent summation over bounded n uses u128 products and division without Euclidean reciprocity; full-width n uses analytically known triangular and constant sums.

For pollard_rho the initial test incorrectly assumed n=9 needs more than one iteration; actual first iteration finds divisor three. The exhaustion witness was corrected to n=25,budget=1. Production logic was unchanged by that correction.

## model

# floor_sum tests.rs model

## Domain

Five tests: exhaustive n=0..23,m=1..23,a,b=0..23 (317952 tuples); full-width exact and overflow fixtures; 950 prefix/quotient pairs; distinct sum fibers; 1000 seeded full-width coefficients with n<50.

## Forward

Execute five tests, requiring every assertion to hold. Direct independent summation over bounded n uses u128 products and division without Euclidean reciprocity; full-width n uses analytically known triangular and constant sums.

## Invariant

Independent oracles do not call the implementation under test. Full-width product reconstruction using production primality is only a consistency check; independent small trial oracles are separate.

## Cost

Finite fixture harness only; oracle loops are explicitly bounded by the source fixtures. Production cost: O(log(max(a,m)+1)) Euclidean word iterations and O(1) storage; checked u128 operations, no allocation.

## nonbehavior

Type: behavior_complement. Over terminating candidate implementations and pass/fail observations, exclude pass when any assertion fails and failure when all assertions hold. Nontermination and abort are outside this binary observation domain.

## implementation_fibers

Type: reverse_reconstruction. Pass reconstructs the set of implementations agreeing on every observed fixture; off-fixture behavior is unconstrained. Failure reconstructs implementations or incorrect oracle premises that cause a mismatch, not a unique defect.

## oracle

Type: oracle_correspondence. Direct independent summation over bounded n uses u128 products and division without Euclidean reciprocity; full-width n uses analytically known triangular and constant sums.

## mutation

Type: counterfactual_boundary. Initialize accumulator to one instead of zero; boundaries_and_overflow must fail after successful compilation.

## replay

Type: metamorphic_relation. Single-thread runs twice each in debug and release must have identical ordered normalized test records. Build output and timing are excluded. Finite agreement is not universal equivalence.

## Composition and gaps

The source is a small leaf; its validation, core arithmetic and result assembly compose under the invariant above. Tests observe the public operation plus any private helper explicitly named by tests. Partial syntax does not expand macros or provide a semantic AST. No useful certified approximate metric is supplied for exact discrete outputs; executable numerical approximants are an explicit gap. No performance benchmark or formal proof is claimed.

## evidence

### Evidence

Five tests: exhaustive n=0..23,m=1..23,a,b=0..23 (317952 tuples); full-width exact and overflow fixtures; 950 prefix/quotient pairs; distinct sum fibers; 1000 seeded full-width coefficients with n<50.

Initialize accumulator to one instead of zero; boundaries_and_overflow must fail after successful compilation.

Syntax extraction is run twice and compared exactly; syntax.json is a partial Rust syntax projection, not semantic AST analysis. Root integration supplies registry validation; this branch does not modify the shared registry.

## prospective

### Prospective model

Domain u64 n,m,a,b, m>0; exact sum i=0..n-1 floor((a*i+b)/m) when it fits u128, otherwise Overflow. Zero modulus wins even for n=0. Euclidean reduction removes coefficient and intercept quotients then transposes lattice points. Invariant answer+remaining sum equals original. Checked nonnegative contributions avoid cancellation and false overflow. O(log(max(a,m)+1)) word steps, O(1) storage. Complement excludes wrong sums and incorrect error choice. Reverse output fibers are all parameter tuples with that sum; sum zero includes n=0 and a=b=0. Quotient shifts, prefix differences, lattice transpose, and overflow boundary are specialized views. No approximate surrogate metric is useful for exact integer answers.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model retained before implementation. Independent bounded oracles, extreme arithmetic fixtures and compiled intentional mutations test the claim. Five owned tests pass twice each in debug and release, with identical normalized records. Finite results do not prove all inputs.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `6239f9a0bcb7a6eadb8b83be21278c60154a47c8519a4768e13d4b015ea22da8`. Five tests per algorithm pass twice in debug and twice in release; normalized records compared by a Rust driver. Strict library Clippy and owned-file formatting pass. One compiled production mutation per algorithm fails a named assertion and restored bytes pass the positive suite.

External evidence under `$MOVA_DATA/runs/octopus-wave18-numbers-2026-10-30/`:

- `floor_sum-debug-1.log`: SHA-256 `5ba2b8e983e00cd043dbfe3ce062a142429c2e98b7d6a2e52e37f3138111699a`.

- `floor_sum-debug-2.log`: SHA-256 `7d21e7d314b6b0a341ffbb2d20a221482df6e27bd61df614818a6d1d6124f352`.

- `floor_sum-release-1.log`: SHA-256 `aeadfccd107a83716c3bb4bbd5b3219fe0d0da10daed1e51036cb640ea806961`.

- `floor_sum-release-2.log`: SHA-256 `2152214d2f9e1fef0e06bc56017c2e12c138ad998de0b09578461bfa8909dd36`.

- `floor_sum-mutant.log`: SHA-256 `c8cda694a73b009a281e598ba5da7e8132e3978109f4dcc51468f801f8db9f77`.

- `floor_sum-restored.log`: SHA-256 `bdf915fe6d5be3f2821f0b2836769f9330f58c313e94613b445f3739f33856a4`.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### Undated · descriptant

Source SHA-256: `6239f9a0bcb7a6eadb8b83be21278c60154a47c8519a4768e13d4b015ea22da8`.
