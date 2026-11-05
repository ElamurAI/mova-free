# algorithm-pollard-rho-tests

## descriptant

### pollard_rho tests.rs

Execute five tests, requiring every assertion to hold. Independent trial division produces full small-number prime multisets; known full-width factorizations and exact u128 reconstruction cover larger cases. The modular step is compared to repeated modular addition, independent of multiplication.

For pollard_rho the initial test incorrectly assumed n=9 needs more than one iteration; actual first iteration finds divisor three. The exhaustion witness was corrected to n=25,budget=1. Production logic was unchanged by that correction.

## model

# pollard_rho tests.rs model

## Domain

Five tests: independent trial factorization n=1..4999; zero/cap/exhaustion, free prime/even cases; u64::MAX known factors, 3^40, 1000000007*1000000009, largest known prime below u64::MAX; seven deterministic replay/budget-extension cases; additive modular-product oracle for n=3..29,x<n,c=1..9.

## Forward

Execute five tests, requiring every assertion to hold. Independent trial division produces full small-number prime multisets; known full-width factorizations and exact u128 reconstruction cover larger cases. The modular step is compared to repeated modular addition, independent of multiplication.

## Invariant

Independent oracles do not call the implementation under test. Full-width product reconstruction using production primality is only a consistency check; independent small trial oracles are separate.

## Cost

Finite fixture harness only; oracle loops are explicitly bounded by the source fixtures. Production cost: At most budget Floyd iterations across splits and restarts, each three modular squarings and one Euclidean gcd. At most 127 pending-node visits for a u64 factor tree, each Miller-Rabin O(log(n+1)) word work. Conservative O((budget+64)*log(n+1)) word operations, O(log(n+1)) pending/output words. This is a hard work cap, not an expected success claim.

## nonbehavior

Type: behavior_complement. Over terminating candidate implementations and pass/fail observations, exclude pass when any assertion fails and failure when all assertions hold. Nontermination and abort are outside this binary observation domain.

## implementation_fibers

Type: reverse_reconstruction. Pass reconstructs the set of implementations agreeing on every observed fixture; off-fixture behavior is unconstrained. Failure reconstructs implementations or incorrect oracle premises that cause a mismatch, not a unique defect.

## oracle

Type: oracle_correspondence. Independent trial division produces full small-number prime multisets; known full-width factorizations and exact u128 reconstruction cover larger cases. The modular step is compared to repeated modular addition, independent of multiplication.

## mutation

Type: counterfactual_boundary. Emit three on the even-composite split instead of two; boundaries_and_budget must fail on 2^63.

## replay

Type: metamorphic_relation. Single-thread runs twice each in debug and release must have identical ordered normalized test records. Build output and timing are excluded. Finite agreement is not universal equivalence.

## Composition and gaps

The source is a small leaf; its validation, core arithmetic and result assembly compose under the invariant above. Tests observe the public operation plus any private helper explicitly named by tests. Partial syntax does not expand macros or provide a semantic AST. No useful certified approximate metric is supplied for exact discrete outputs; executable numerical approximants are an explicit gap. No performance benchmark or formal proof is claimed.

## evidence

### Evidence

Five tests: independent trial factorization n=1..4999; zero/cap/exhaustion, free prime/even cases; u64::MAX known factors, 3^40, 1000000007*1000000009, largest known prime below u64::MAX; seven deterministic replay/budget-extension cases; additive modular-product oracle for n=3..29,x<n,c=1..9.

Emit three on the even-composite split instead of two; boundaries_and_budget must fail on 2^63.

Syntax extraction is run twice and compared exactly; syntax.json is a partial Rust syntax projection, not semantic AST analysis. Root integration supplies registry validation; this branch does not modify the shared registry.

## prospective

### Prospective model

Domain n:u64 and budget<=10^7 Floyd iterations; zero invalid (before budget validation), one yields empty. Prime inputs require no rho budget. Reuse deterministic Miller-Rabin and Euclidean gcd, exact u128 square plus c modular arithmetic. Deterministic c restarts on collision, with global consumed iteration count; no guarantee of success within any budget. Each iteration performs three polynomial evaluations plus gcd; pending split stack and output have <=64 entries. Invariant product(pending)*product(primes)=original and each emitted factor is prime. O((budget+64)*log(n)) word-operation conservative bound including primality; O(log n) words. Complement excludes incomplete successful factors and hangs. Successful output reconstructs n uniquely by multiplication but not budget; exhausted output cannot identify n. Prime-power, product reconstruction, budget monotonic success, and deterministic replay specialize model.

## root-wave18

### Root integration refinement

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model retained before implementation. Independent bounded oracles, extreme arithmetic fixtures and compiled intentional mutations test the claim. Five owned tests pass twice each in debug and release, with identical normalized records. Finite results do not prove all inputs.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `9dc84e061044dd0f0e915c268668164c34130d6537f59051a60e0500c20d689c`. Five tests per algorithm pass twice in debug and twice in release; normalized records compared by a Rust driver. Strict library Clippy and owned-file formatting pass. One compiled production mutation per algorithm fails a named assertion and restored bytes pass the positive suite.

External evidence under `$MOVA_DATA/runs/octopus-wave18-numbers-2026-10-30/`:

- `pollard_rho-debug-1.log`: SHA-256 `9758cfd6078528c523f63a8f6c72a6b7ec1ebdec37387a29d1ead040046a2c14`.

- `pollard_rho-debug-2.log`: SHA-256 `598d3a802e75de03bb0c4041d4fdc2ab2e3665fd621138f0e54afe5979cce6fd`.

- `pollard_rho-release-1.log`: SHA-256 `64ef2510ce388035d3c0c867ba97cd62b37873a00126afebc5044d395ecba5b5`.

- `pollard_rho-release-2.log`: SHA-256 `c2cfed586585fe3197a7de263072894817e4c6775abe3c5662850e64d6f9c0dc`.

- `pollard_rho-mutant.log`: SHA-256 `f161e35f0867062da3e0f6365170332fe3505bd58bdc1f560d3cf5d0e89721c5`.

- `pollard_rho-restored.log`: SHA-256 `a9a907a93f31a6cd5e17d3f4303f37a20f05f256c1102ade91f760144b632baf`.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-10-30 · root-wave18

Heading `### Root integration refinement — 2026-10-30` stays in root-wave18 as `### Root integration refinement`.

Strict all-target Clippy exposed a test-only style issue after the owner library-only check. The source now uses the equivalent standard slice/divisibility helper. Old test source SHA-256: 9dc84e061044dd0f0e915c268668164c34130d6537f59051a60e0500c20d689c; current SHA-256: 9d6b40e7ef478d6bb079264ef0fdfff255d6bd70cb38b38b56f66da49dff01fa. Owner receipts remain historical and are not rewritten. Final root execution is recorded in models/algorithm-library.groupant/model.md#wave18-evidence; raw receipts are under $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30/native-final. Syntax was projected twice identically after this edit. No production algorithm changed.

### Undated · descriptant

Source SHA-256: `9d6b40e7ef478d6bb079264ef0fdfff255d6bd70cb38b38b56f66da49dff01fa`.
