# algorithm-prim-tests

## descriptant

### prim tests.rs

Execute bounded fixtures and independent oracles, assert exact outputs and return harness pass/failure identities. 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling.

Composition: Uses existing Kruskal Edge and Forest records; algorithmic optimum is additionally compared with Kruskal. Tie trace need not coincide.

No executable percentage-error surrogate: the public operation remains exact, and no useful approximate observation metric is claimed. Allocation failure, timing and general semantic resolution remain outside these models.

## approximant

### Forward and cost

Domain: Finite native test inputs, literal fixtures and oracle outcomes for prim; trusted Rust harness. Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.

Execute bounded fixtures and independent oracles, assert exact outputs and return harness pass/failure identities. 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling.

Cost: Finite enumeration cost includes independent oracle; no production timing or unbounded correctness inferred.

## invariant

### Invariant

Every passing comparison uses the same generated input for implementation and oracle. Named assertion failure rejects a mismatching observation; all domain coverage is finite.

## catalog

# Typed model family

## behavior-complement

Domain: Finite native test inputs, literal fixtures and oracle outcomes for prim; trusted Rust harness. Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Let U be declared inputs crossed with returned observations and R the forward relation in model.md#approximant. C=U minus R includes wrong first errors, wrong values and forbidden results. This is an exact specification complement and partial source-derived model, not a measured global Rust complement. For this test file, false acceptance of a mismatching finite fixture is excluded by assertions; passing unseen implementations remains possible. 
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## reverse-reconstruction

Domain: Finite native test inputs, literal fixtures and oracle outcomes for prim; trusted Rust harness. Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Pre(y)={x in domain | R(x,y)} retains all compatible inputs. A passing harness observation has many compatible implementations agreeing on tested cases; failure identities retain every implementation/oracle pair producing that failure. No unique recovery or inverse executor is claimed.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## oracle-independence

Domain: Finite native test inputs, literal fixtures and oracle outcomes for prim; trusted Rust harness. Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Hold generated input fixed and compare different recurrences or exhaustive subsets rather than a production wrapper. 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. The extra cross-algorithm check for Prim does not replace its independent subset optimum.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## mutation-sensitivity

Domain: Finite native test inputs, literal fixtures and oracle outcomes for prim; trusted Rust harness. Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: One compiled implementation mutant and one purpose-specific false oracle/literal per family fail a selected named assertion. Passing restored tests certifies only these finite observations; failure does not identify a unique cause.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## tests

### Finite evidence

4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original implementation and finite independent witnesses. O(V+E log(E+1)) time and O(V+E) storage. Every edge is inserted at most twice; stale internal edges are charged. Limits precede allocations; allocator failures are not typed.

### 2026-10-30 · tests

2026-10-30: 310 unit tests pass twice debug and twice release, with identical normalized names/results (sha256 3f3c86b69fdc0e79e1f9d3e885301a424073b04198a1874f587647a2a6d07a4e). This file contributes four tests. Six compiled controls (one implementation and one test oracle per algorithm) fail named assertions; restored 310-test suite passes. Algorithm all-target strict Clippy and narrow rustfmt pass. Raw logs are external; no performance gain is claimed.

This source control log sha256: f107a209dfba0f4d1391e86f0fb12090c6cb3586fd92425f0f065e9b5f37f53f. False subset oracle reports best+1; exhaustive_forests fails.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: tests.rs sha256 426c850115b00d94b2c07a051fde0e4531ac009b793d92e1035e740061a40d16.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs sha256 426c850115b00d94b2c07a051fde0e4531ac009b793d92e1035e740061a40d16.

### Undated · oracle-independence

Type: reference_separation. Subject/revision: tests.rs sha256 426c850115b00d94b2c07a051fde0e4531ac009b793d92e1035e740061a40d16.

### Undated · mutation-sensitivity

Type: falsification_boundary. Subject/revision: tests.rs sha256 426c850115b00d94b2c07a051fde0e4531ac009b793d92e1035e740061a40d16.
