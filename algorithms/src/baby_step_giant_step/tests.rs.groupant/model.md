# algorithm-baby-step-giant-step-tests

## descriptant

### baby_step_giant_step

Prospective model saved before implementation in ../mod.rs.groupant/model.md#prospective (relative from source directory). Original standard-library code. This epoch adds explicit supported domains and independent finite checks. See model.md for typed excluded-output and preimage relations. Partial syntax omits macro expansion and semantic resolution. No executable approximation profile is attached; useful cheaper metrics and evidence remain unsupported. No timing gain is claimed.

## model

# baby_step_giant_step tests.rs

## Forward

Deterministic finite fixtures compare the public API with independent observations and validation outcomes.

## Invariant

Each exercised case either matches the declared oracle or fails an assertion. No random, clock, I/O or callbacks inside tests.

## Cost

Bounded finite fixture cost plus production calls. No timing claims; no benchmark in test harness.

## behavior_complement

A successful run excludes mismatched Result values on the exercised calls. It does not exclude defects outside those calls. The tests do not inject allocation failure or assert an Error result.

## reverse_reconstruction

Let F(P) be the vector of Result values observed at the calls reached by these tests. A passing transcript admits every deterministic implementation P with the asserted F(P); it does not reconstruct P uniquely. For example, a wrapper returning Some(0) only at (2, 0) and otherwise delegating to the actual API agrees on these fixtures but differs from the actual None there. The transcript also does not recover intermediate table or giant-step state.

## oracle_independence

Enumerate first20 powers of five bases and retain first occurrence; compare returned minimum. Large exponent p-2 and no-solution subgroup targets separately exercise giant scanning.

## boundary_partition

The eight named calls exercise zero and unit bases, absence, one normalization case and a large exponent. All assert Ok, including Ok(None); none asserts a validation or resource error. Allocator failure is not injected.

## fixture_projection

For each g in {0, 1, p-1, 3, 7}, the direct oracle retains min {e in 0..20: g^e mod p = t} for each encountered t. This independently generated prefix establishes a least exponent among that prefix, not by itself a proof that every target or every implementation is correct. The last boundary target is generated with the production module's private pow, so that target construction is not independent of that helper. A separately compiled wrapper changing (0,0) to None is rejected by the fixture replay; the (2,0) wrapper above passes it. The replay is an external copy of the assertions, while the actual tests are run separately.

## mutation_observation

One compiled production mutant must fail direct_oracle; exact source bytes restored before positive rerun. This only proves sensitivity to that chosen changed behavior, not mutation completeness.

## evidence

#### Bounded review

The actual two focused tests passed. An external Rust replay of their fixtures passed both the actual API and a wrapper that alters only (2, 0); the actual API returns Ok(None) there, while that wrapper returns Ok(Some(0)). A second wrapper altering (0, 0) was rejected. Two replay outputs matched. The replay copies the fixture assertions and does not establish universal correctness, an error rate, allocation behavior, or independence of the large-exponent target from the private pow helper. Raw probe and logs are retained in the cycle run directory.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: (see History). Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### Evidence

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

### 2026-11-01 · evidence

#### Bounded review — 2026-11-01

The actual two focused tests passed. An external Rust replay of their fixtures passed both the actual API and a wrapper that alters only (2, 0); the actual API returns Ok(None) there, while that wrapper returns Ok(Some(0)). A second wrapper altering (0, 0) was rejected. Two replay outputs matched. The replay copies the fixture assertions and does not establish universal correctness, an error rate, allocation behavior, or independence of the large-exponent target from the private pow helper. Raw probe and logs are retained in the cycle run directory.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: $MOVA_DATA/runs/octopus-wave25-algebra-2026-10-30. Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.
