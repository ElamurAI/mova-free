# algorithm-minimum-cost-flow-tests

## descriptant

### minimum_cost_flow tests

Original std-only Rust. Prospective model is retained beside implementation. Source citations identify algorithm provenance; no copied implementation. Exact bounded graph semantics are useful independently of any unmeasured optimization. Finite oracle checks and mutant discrimination improve confidence; groupant count alone proves no improvement.

## model

# minimum_cost_flow tests model

## Forward

Generate fixed exhaustive small domains for minimum_cost_flow; independent oracle predicts output, assert production correspondence plus validation/boundary behavior. All assertions passing returns Pass. Any mismatched fixture causes named failure; bounded success is not universal proof.

## Invariant

Each fixture is generated deterministically; oracle and production receive the same graph. Enumeration order and semantic assertions are independent of timing.

## Cost

Bounded exhaustive fixtures: flow 729 DAG capacity arrays times 4 limits, 64 cyclic graphs times 4 limits; MCS and core 1024 topologies (core 7 thresholds and 32 subsets); dominators 4096 graphs times 4 starts with deletion reachability; reduction 4096 graphs with edge-removal reachability. These are per-file domains, not all tests in every file. No benchmark or speed claim.

## nonbehavior

Type: behavior_complement. For a fixed fixture collection, Pass is excluded if any production observation differs from the independent oracle or validation assertion; a named Failure is excluded when its compared values agree. This relation excludes false-green and false-red harness observations; tests do not approximate unexecuted inputs.

## compatible_inputs

Type: reverse_reconstruction. Pass admits every implementation agreeing on all finite fixtures, including implementations that disagree elsewhere. Failure at fixture f admits implementations with its reported mismatch; it does not recover a unique bug. Distinct programs with identical observed fixture results lie in the same preimage.

## oracle_independence

Type: differential_oracle. Flow enumerates feasible edge assignments rather than residual paths; MCS checks all priority inequalities rather than updating weights; core unions all feasible subsets rather than peeling; dominators deletes candidate vertices rather than intersecting sets; reduction deletes each edge rather than Warshall reduction. The applicable independent characterization detects semantic disagreement.

## mutant_separation

Type: negative_control. A compiled production mutation for minimum_cost_flow must fail its recorded named assertion; exact original bytes are restored and the positive run must pass. This distinguishes at least the observed mutant from the accepted reference, not every possible defect.

## repeat_transport

Type: configuration_invariance. Two debug and two optimized standalone compilations produce identical normalized test-status lines with one test thread. Timing lines are omitted; equality concerns semantic status, not durations or complete machine behavior.

## Limits

Partial syntax omits macro expansion and type resolution. No executable approximate profile: categorical graph semantics and feasibility have no justified percentage relaxation here. Integer operations are bounded by caps; flow uses checked i128/u64 arithmetic, with Overflow defensive under current caps. No callbacks. Allocator failure is outside typed status.

## evidence

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to minimum_cost_flow. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: (see History). No timing or whole-system improvement claim. Source SHA-256: (see History).

### Evidence

## prospective

Finite independent graph characterizations and boundary tests planned before validation. No cost optimization or universal proof claimed.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to minimum_cost_flow. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: $MOVA_DATA/runs/octopus-wave25-graphs-2026-10-30/minimum_cost_flow-*.txt. No timing or whole-system improvement claim. Source SHA-256: 5f9fb5e4225efe0b0ad625b5265338c8984dd005a455c2c34aac52ee554a8415.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
