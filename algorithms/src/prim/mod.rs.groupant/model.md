# algorithm-prim

## descriptant

### prim mod.rs

Check limits then each edge endpoint in input order, u before v. Ignore validated loops. Visit components from smallest unvisited root. Insert incident edges in a min heap keyed by (weight,min endpoint,max endpoint,index), discard edges with both ends visited, and accept each crossing minimum into the growing tree. Sum weights in checked u128.

Composition: Uses existing Kruskal Edge and Forest records; algorithmic optimum is additionally compared with Kruskal. Tie trace need not coincide.

No executable percentage-error surrogate: the public operation remains exact, and no useful approximate observation metric is claimed. Allocation failure, timing and general semantic resolution remain outside these models.

## approximant

### Forward and cost

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.

Check limits then each edge endpoint in input order, u before v. Ignore validated loops. Visit components from smallest unvisited root. Insert incident edges in a min heap keyed by (weight,min endpoint,max endpoint,index), discard edges with both ends visited, and accept each crossing minimum into the growing tree. Sum weights in checked u128.

Cost: O(V+E log(E+1)) time and O(V+E) storage. Every edge is inserted at most twice; stale internal edges are charged. Limits precede allocations; allocator failures are not typed.

## invariant

### Invariant

Accepted edges cross the visited cut and never create a cycle. Every input connected component is spanned; accepted edge count is V-components. Total weight is minimum among all forests spanning original components. Isolated vertices count as components.

## catalog

# Typed model family

## behavior-complement

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Let U be declared inputs crossed with returned observations and R the forward relation in model.md#approximant. C=U minus R includes wrong first errors, wrong values and forbidden results. This is an exact specification complement and partial source-derived model, not a measured global Rust complement. Accepted edges cross the visited cut and never create a cycle. Every input connected component is spanned; accepted edge count is V-components. Total weight is minimum among all forests spanning original components. Isolated vertices count as components. 
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## reverse-reconstruction

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Pre(y)={x in domain | R(x,y)} retains all compatible inputs. Outputs lose redundant edges and intermediate heap/pivot traces; errors retain all graphs sharing their first-failure condition. No unique recovery or inverse executor is claimed.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## cut-choice

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: At each nonempty frontier the smallest heap key among crossing edges is safe by the minimum-cut exchange rule. Discarded internal edges cannot connect new vertices. Exhaustive subset optimum verifies finite instances independently.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## tie-order

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Fix ordered edge records: roots increase; candidate order is (weight,min endpoint,max endpoint,index). Parallel equal edges prefer the earlier original index when simultaneously eligible. Output order can differ from Kruskal; only optimum weight/components are compared across algorithms.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## orientation

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Swapping endpoints of valid edges preserves heap keys, adjacency incidence, accepted original indices, total and components. Invalid inputs do not obey this equivalence because u-before-v validation is observable.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## scaling

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Multiply all weights by positive integer k while keeping u64 products representable. Heap comparisons and ties stay identical; accepted indices/components stay fixed and total multiplies by k. Executed k=7.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## loop-fiber

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Adding valid self-loops preserves output indices of earlier edges, total and components within limits. A forest observation cannot recover ignored loops, discarded cycle edges or their weights. Executed redundant-loop fixture.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: 761 weighted simple graphs on 0..4 vertices, each possible edge absent/0/3, compared with independent subset spanning optimum and traversed components; Kruskal weight/component agreement; deterministic ties, parallel edges, loops, isolates, wide sum, limits, orientation and scaling. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## validation

Domain: Vertex count and ordered undirected kruskal::Edge records with u64 weights, at most one million vertices and edges. Return kruskal::Forest with original indices in Prim acceptance order.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Vertex/edge caps precede endpoint checks. InvalidEndpoint selects first input edge then u before v. u128 checked sum accommodates paths exceeding u64::MAX; wide two-edge fixture verifies this. TotalWeightOverflow is a defensive branch not reached under caps.
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

This source control log sha256: 4b75b712ed0b053c0d2b1d8f722f3a73b68a8ba90e0a31fe5780fb71d961292b. Mutant increments components by two; exhaustive_forests fails.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.

### Undated · cut-choice

Type: greedy_optimality. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.

### Undated · tie-order

Type: canonical_trace. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.

### Undated · orientation

Type: undirected_equivalence. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.

### Undated · scaling

Type: positive_weight_equivariance. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.

### Undated · loop-fiber

Type: ignored_edge_preimage. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.

### Undated · validation

Type: error_and_numeric_boundary. Subject/revision: mod.rs sha256 222dd71ec5789ca54d3665bab9ffcae3f01e0b9c4d35ecf093e96e70c733e6b6.
