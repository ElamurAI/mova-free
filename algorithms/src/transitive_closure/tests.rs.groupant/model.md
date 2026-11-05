# algorithm-transitive_closure-tests

## descriptant

### transitive_closure tests.rs

Execute bounded fixtures and independent oracles, assert exact outputs and return harness pass/failure identities. 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers.

Composition: Private packed rows compose pivot updates with checked pair queries; traversal supplies an independent relation.

No executable percentage-error surrogate: the public operation remains exact, and no useful approximate observation metric is claimed. Allocation failure, timing and general semantic resolution remain outside these models.

## approximant

### Forward and cost

Domain: Finite native test inputs, literal fixtures and oracle outcomes for transitive_closure; trusted Rust harness. Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.

Execute bounded fixtures and independent oracles, assert exact outputs and return harness pass/failure identities. 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers.

Cost: Finite enumeration cost includes independent oracle; no production timing or unbounded correctness inferred.

## invariant

### Invariant

Every passing comparison uses the same generated input for implementation and oracle. Named assertion failure rejects a mismatching observation; all domain coverage is finite.

## catalog

# Typed model family

## behavior-complement

Domain: Finite native test inputs, literal fixtures and oracle outcomes for transitive_closure; trusted Rust harness. Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Let U be declared inputs crossed with returned observations and R the forward relation in model.md#approximant. C=U minus R includes wrong first errors, wrong values and forbidden results. This is an exact specification complement and partial source-derived model, not a measured global Rust complement. For this test file, false acceptance of a mismatching finite fixture is excluded by assertions; passing unseen implementations remains possible. 
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## reverse-reconstruction

Domain: Finite native test inputs, literal fixtures and oracle outcomes for transitive_closure; trusted Rust harness. Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Pre(y)={x in domain | R(x,y)} retains all compatible inputs. A passing harness observation has many compatible implementations agreeing on tested cases; failure identities retain every implementation/oracle pair producing that failure. No unique recovery or inverse executor is claimed.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## oracle-independence

Domain: Finite native test inputs, literal fixtures and oracle outcomes for transitive_closure; trusted Rust harness. Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Hold generated input fixed and compare different recurrences or exhaustive subsets rather than a production wrapper. 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. The extra cross-algorithm check for Prim does not replace its independent subset optimum.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## mutation-sensitivity

Domain: Finite native test inputs, literal fixtures and oracle outcomes for transitive_closure; trusted Rust harness. Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: One compiled implementation mutant and one purpose-specific false oracle/literal per family fail a selected named assertion. Passing restored tests certifies only these finite observations; failure does not identify a unique cause.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## tests

### Finite evidence

4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original implementation and finite independent witnesses. O(V² ceil(V/64)+E) time; O(V ceil(V/64)) persistent storage and one row scratch. Limits bound packed storage; allocation failure remains outside typed errors.

### 2026-10-30 · tests

2026-10-30: 310 unit tests pass twice debug and twice release, with identical normalized names/results (sha256 3f3c86b69fdc0e79e1f9d3e885301a424073b04198a1874f587647a2a6d07a4e). This file contributes four tests. Six compiled controls (one implementation and one test oracle per algorithm) fail named assertions; restored 310-test suite passes. Algorithm all-target strict Clippy and narrow rustfmt pass. Raw logs are external; no performance gain is claimed.

This source control log sha256: fe01f65cc51e19f14d7968542c434334ee930da27f338ab3d04c559135244e82. False chain oracle uses u<v instead of u<=v; word_boundaries_and_direction fails.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: tests.rs sha256 a6d86338349cf4360467e2077a064f76ce23782d89263d955868aa3bf3e70e59.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs sha256 a6d86338349cf4360467e2077a064f76ce23782d89263d955868aa3bf3e70e59.

### Undated · oracle-independence

Type: reference_separation. Subject/revision: tests.rs sha256 a6d86338349cf4360467e2077a064f76ce23782d89263d955868aa3bf3e70e59.

### Undated · mutation-sensitivity

Type: falsification_boundary. Subject/revision: tests.rs sha256 a6d86338349cf4360467e2077a064f76ce23782d89263d955868aa3bf3e70e59.
