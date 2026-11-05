# algorithm-zero-one-bfs-tests

## descriptant

### algorithm-zero-one-bfs-tests descriptant

The combiner joins the authoritative source hash and partial syntax with forward path semantics, first behavior complement and second result-preimage relation, invariants, cost and executed finite evidence. Agreement preserves a bounded correspondence claim; it does not turn AST syntax into a proof of Rust behavior. The implementation/test boundary is explicit: witness paths certify distances and finite oracle passes constrain implementations only on observed inputs.

The mathematical mechanism is binary-weight deque specialization of Dijkstra, as discussed by cp-algorithms, 0-1 BFS; implementation and fixtures are original, with no copied source.

Retained observations: disconnected invalid input remains observable; distance vectors have many compatible graph antecedents; allocation failure and defensive overflow branches have no finite fixture evidence. The reversed-relaxation control on cyclic inputs did not terminate promptly and was stopped; the final zero-increment control compiles and produces an actual assertion failure.

## model

# algorithm-zero-one-bfs-tests

## Domain

Deterministic finite fixture executions and assertion verdict vectors for the adjacent implementation; not an all-input proof.

## Forward

Six tests: enumerate all 3^9 directed three-vertex graphs, each of three sources, and compare against recursive enumeration of simple paths; then assert global ordered errors, stale improvements/zero cycles/parallel edges, vertex relabeling and duplication, a 2048-vertex chain plus isolated vertex, and a nonunique distance fiber.

## Invariant

Oracle enumerations are independent of production scheduling and use immutable generated graphs. Assertions compare full vectors; all test loops are deterministic. Bounded costs and arithmetic follow fixture sizes; no randomness or external data.

## Cost

Finite enumeration 3^9 graphs at 3 sources, simple-path oracle factorial in vertex count in general but n=3 here; other fixtures linear in 2048. Symbolic suite cost, no timing claim.

## nonbehavior

Type: behavior_complement.

In the universe of fixture/verdict pairs, excluded behavior contains a passing verdict when any encoded expected value differs, or a claim of universal correctness from these finite fixtures. The independent oracle enumerates paths rather than reproducing the production scheduling mechanism.

## verdict_preimage

Type: reverse_reconstruction.

Given the all-pass assertion vector, recover all candidate implementations agreeing with the fixture map on exactly the executed inputs. This set includes the reference and implementations that fail on untested inputs; it does not identify unique source code. A failing oracle assertion constrains the preimage to implementations disagreeing on that concrete graph/source/output.

## oracle_correspondence

Type: finite_relation.

Simple paths suffice because all weights are nonnegative: deleting a cycle never raises cost. Enumeration covers 19,683 graph codes and 59,049 graph/source comparisons including self loops. Oracle seen-state prevents cyclic recursion.

## mutation_separation

Type: distinguishability_relation.

Replacing the production checked edge increment by zero must disagree on a reachable unit-weight edge; the exhaustive oracle assertion rejects this compiled mutation. An initial reversed-comparison mutant allowed positive-cycle growth and was manually stopped, so it is not counted as an assertion witness.

## coverage_boundary

Type: coverage_relation.

Fixtures sample graph semantics, source/endpoint priority, disconnected behavior and metamorphic identities. They do not exercise allocation failure or the defensive checked-distance overflow return. Repeatability means identical normalized named test outcomes across two debug and two release runs, not identical timings.

## Limits

Source-derived mathematical models with finite executable correspondence, not formal Rust proofs. Partial syntax omits macro expansion, type resolution and semantic AST. Exact production implementation; no approximate surrogate or numerical error guarantee is claimed. Resource exhaustion is outside the successful-call model.

## evidence

### Evidence

Simple paths suffice because all weights are nonnegative: deleting a cycle never raises cost. Enumeration covers 19,683 graph codes and 59,049 graph/source comparisons including self loops. Oracle seen-state prevents cyclic recursion.

These observations establish finite correspondence only. No timing superiority, exhaustive machine-domain proof, approximation error budget or overflow-error test coverage is claimed.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: Six tests: enumerate all 3^9 directed three-vertex graphs, each of three sources, and compare against recursive enumeration of simple paths; then assert global ordered errors, stale improvements/zero cycles/parallel edges, vertex relabeling and duplication, a 2048-vertex chain plus isolated vertex, and a nonunique distance fiber.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 87b2ff0120fc61a24d12fcd23697ca53aa5d7405f3ae7c2d29f9b66fff7c526b.

Six tests for this algorithm passed in two debug and two release full-crate runs (179 tests each); normalized named outcomes were identical. Strict all-target Clippy and formatting checks passed. Source syntax was projected twice and compared byte-for-byte. A compiled production mutant was rejected by the independent finite oracle assertion. External run bundle: MOVA_DATA/runs/octopus-wave4-graphs-2026-10-30 (debug1/2, release1/2, normalized outcomes, clippy, format, zero_one_bfs-mutant and syntax logs; validate.rs records commands).

### 2026-10-30 · prospective

#### Symbolic expectations and scope

2026-10-30: Expected exact path semantics and Finite enumeration 3^9 graphs at 3 sources, simple-path oracle factorial in vertex count in general but n=3 here; other fixtures linear in 2048. Symbolic suite cost, no timing claim. These are mathematical expectations, not calibrated performance forecasts. The initial implementation plan selected deque snapshots and strict improvement to avoid stale scans and zero-cycle churn. Finite tests subsequently check the stated fixtures; no runtime benchmark or probabilistic approximation is proposed.
