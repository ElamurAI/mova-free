# algorithm-johnson-tests

## descriptant

### johnson tests.rs

Execute bounded fixtures and independent oracles, assert exact outputs and return harness pass/failure identities. 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle.

Composition: Existing Dijkstra accepts u64 edges, so reusing it would narrow valid reweighted i128 values. The local heap preserves the signed-wide domain; potential-shift tests check the restoration join.

No executable percentage-error surrogate: the public operation remains exact, and no useful approximate observation metric is claimed. Allocation failure, timing and general semantic resolution remain outside these models.

## approximant

### Forward and cost

Domain: Finite native test inputs, literal fixtures and oracle outcomes for johnson; trusted Rust harness. Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.

Execute bounded fixtures and independent oracles, assert exact outputs and return harness pass/failure identities. 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle.

Cost: Finite enumeration cost includes independent oracle; no production timing or unbounded correctness inferred.

## invariant

### Invariant

Every passing comparison uses the same generated input for implementation and oracle. Named assertion failure rejects a mismatching observation; all domain coverage is finite.

## catalog

# Typed model family

## behavior-complement

Domain: Finite native test inputs, literal fixtures and oracle outcomes for johnson; trusted Rust harness. Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Let U be declared inputs crossed with returned observations and R the forward relation in model.md#approximant. C=U minus R includes wrong first errors, wrong values and forbidden results. This is an exact specification complement and partial source-derived model, not a measured global Rust complement. For this test file, false acceptance of a mismatching finite fixture is excluded by assertions; passing unseen implementations remains possible. 
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## reverse-reconstruction

Domain: Finite native test inputs, literal fixtures and oracle outcomes for johnson; trusted Rust harness. Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Pre(y)={x in domain | R(x,y)} retains all compatible inputs. A passing harness observation has many compatible implementations agreeing on tested cases; failure identities retain every implementation/oracle pair producing that failure. No unique recovery or inverse executor is claimed.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## oracle-independence

Domain: Finite native test inputs, literal fixtures and oracle outcomes for johnson; trusted Rust harness. Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Hold generated input fixed and compare different recurrences or exhaustive subsets rather than a production wrapper. 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. The extra cross-algorithm check for Prim does not replace its independent subset optimum.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## mutation-sensitivity

Domain: Finite native test inputs, literal fixtures and oracle outcomes for johnson; trusted Rust harness. Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: One compiled implementation mutant and one purpose-specific false oracle/literal per family fail a selected named assertion. Passing restored tests certifies only these finite observations; failure does not identify a unique cause.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## tests

### Finite evidence

4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original implementation and finite independent witnesses. O(V(V+E)+V E log(E+1)) time; O(V²+V+E) storage. Heap duplicates are counted. Checked i128 arithmetic and explicit vertex/edge caps; allocator failure is not caught.

### 2026-10-30 · tests

2026-10-30: 310 unit tests pass twice debug and twice release, with identical normalized names/results (sha256 3f3c86b69fdc0e79e1f9d3e885301a424073b04198a1874f587647a2a6d07a4e). This file contributes four tests. Six compiled controls (one implementation and one test oracle per algorithm) fail named assertions; restored 310-test suite passes. Algorithm all-target strict Clippy and narrow rustfmt pass. Raw logs are external; no performance gain is claimed.

This source control log sha256: dc737ee4dcd4984e4aeddce86cd69f1bd3bbba51dc7725ebabd82c6696b73428. False oracle initializes Floyd diagonals to one; exhaustive_signed_graphs fails.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: tests.rs sha256 3d0bda19ad42ce327f7060169a73cfd33476076976e4850c9965c7891d357004.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs sha256 3d0bda19ad42ce327f7060169a73cfd33476076976e4850c9965c7891d357004.

### Undated · oracle-independence

Type: reference_separation. Subject/revision: tests.rs sha256 3d0bda19ad42ce327f7060169a73cfd33476076976e4850c9965c7891d357004.

### Undated · mutation-sensitivity

Type: falsification_boundary. Subject/revision: tests.rs sha256 3d0bda19ad42ce327f7060169a73cfd33476076976e4850c9965c7891d357004.
