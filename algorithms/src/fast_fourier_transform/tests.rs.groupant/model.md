# algorithm-fast-fourier-transform-tests

## descriptant

### fast_fourier_transform

Prospective model saved before implementation in ../mod.rs.groupant/model.md#prospective (relative from source directory). Original standard-library code. This epoch adds explicit supported domains and independent finite checks. See model.md for typed excluded-output and preimage relations. Partial syntax omits macro expansion and semantic resolution. No executable approximation profile is attached; useful cheaper metrics and evidence remain unsupported. No timing gain is claimed.

## model

# fast_fourier_transform tests.rs

## Forward

Deterministic finite fixtures compare the public API with independent observations and validation outcomes.

## Invariant

Each exercised case either matches the declared oracle or fails an assertion. No random, clock, I/O or callbacks inside tests.

## Cost

Bounded finite fixture cost plus production calls. No timing claims; no benchmark in test harness.

## behavior_complement

A successful run excludes any exercised mismatch between production and independent oracle, accepted invalid fixture, or failure of a named residual/dimension assertion. It does not exclude defects outside the finite fixture set.

## reverse_reconstruction

An observed passing transcript admits all production programs agreeing on exercised observations, including programs wrong on unseen inputs. The original and any implementation with identical fixture outputs are indistinguishable by this harness.

## oracle_independence

Direct quadratic complex DFT independently evaluates sin/cos for each j*k, with absolute tolerance1e-10; inverse roundtrip tolerance1e-12. Floating oracle is also rounded, not certified exact.

## boundary_partition

Named boundaries test validation errors, degenerate cases and normalization where applicable. Allocator failure is not injected; production resource status and algebraic absence are separate assertions.

## mutation_observation

One compiled production mutant must fail direct_oracle; exact source bytes restored before positive rerun. This only proves sensitivity to that chosen changed behavior, not mutation completeness.

## evidence

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: (see History). Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### Evidence

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: $MOVA_DATA/runs/octopus-wave25-algebra-2026-10-30. Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.
