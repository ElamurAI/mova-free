# algorithm-johnson

## descriptant

### johnson mod.rs

Check size limits, then every target in row order. Synchronous Bellman-Ford potentials start at zero at every vertex (implicit super-source); an improvement on round V rejects a negative cycle anywhere. Transform w to w+h[u]-h[v], run lazy minimum-heap Dijkstra for each source with i128 keys, then restore d=d_prime-h[source]+h[target].

Composition: Existing Dijkstra accepts u64 edges, so reusing it would narrow valid reweighted i128 values. The local heap preserves the signed-wide domain; potential-shift tests check the restoration join.

No executable percentage-error surrogate: the public operation remains exact, and no useful approximate observation metric is claimed. Allocation failure, timing and general semantic resolution remain outside these models.

## approximant

### Forward and cost

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.

Check size limits, then every target in row order. Synchronous Bellman-Ford potentials start at zero at every vertex (implicit super-source); an improvement on round V rejects a negative cycle anywhere. Transform w to w+h[u]-h[v], run lazy minimum-heap Dijkstra for each source with i128 keys, then restore d=d_prime-h[source]+h[target].

Cost: O(V(V+E)+V E log(E+1)) time; O(V²+V+E) storage. Heap duplicates are counted. Checked i128 arithmetic and explicit vertex/edge caps; allocator failure is not caught.

## invariant

### Invariant

Accepted potentials satisfy h[v]<=h[u]+w for every edge. Reweighted weights are nonnegative. At successful return every diagonal is zero, absent cells mean unreachable, and finite cells are minimum walk weights.

## catalog

# Typed model family

## behavior-complement

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Let U be declared inputs crossed with returned observations and R the forward relation in model.md#approximant. C=U minus R includes wrong first errors, wrong values and forbidden results. This is an exact specification complement and partial source-derived model, not a measured global Rust complement. Accepted potentials satisfy h[v]<=h[u]+w for every edge. Reweighted weights are nonnegative. At successful return every diagonal is zero, absent cells mean unreachable, and finite cells are minimum walk weights. 
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## reverse-reconstruction

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Pre(y)={x in domain | R(x,y)} retains all compatible inputs. Outputs lose redundant edges and intermediate heap/pivot traces; errors retain all graphs sharing their first-failure condition. No unique recovery or inverse executor is claimed.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## potential-telescoping

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: For fixed valid graph and integer vertex labels p, replace w(u,v) by w(u,v)+p[v]-p[u] when all transformed edges fit i64. Cycle sums and reachability stay fixed, and each finite D[u,v] changes by p[v]-p[u]. Executed potential_shift_and_information_loss.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## global-cycle

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: A negative cycle in any component makes the entire result NegativeCycle, even if unreachable from vertex zero. This differs from source-local Bellman-Ford. Exhaustive signed graphs and disconnected self-loop witness exercise the distinction.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## wide-reweighting

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Under edge and vertex caps, potentials are sums of at most V original i64 weights; transformed weights can exceed u64::MAX. They remain i128 throughout. The three-MIN chain plus MAX shortcut is an executed narrowing counterexample.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## first-failure

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: ResourceLimit precedes invalid endpoints; first invalid target in adjacency order precedes cycle detection, including unreachable rows. Overflow is a checked fallback; finite tests do not exercise i128 overflow, which caps exclude on ordinary targets.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## redundant-arc

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Adding a parallel arc whose weight is at least the existing minimum walk distance between its endpoints preserves the matrix on a cycle-free graph, provided limits remain satisfied. Multiple graph preimages thus share one matrix. Executed redundant weight100 fixture.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: exhaustive ternary absent/-1/+2 directed graphs including loops on 0..3 vertices (19768 graphs); disconnected cycle, global-invalid precedence, vertex/edge limits, wide i128 reweighting, potential shifts and redundant arcs. Independent Floyd-Warshall supplies minimum-distance and global cycle oracle. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## heap-staleness

Domain: Ordered directed adjacency lists with i64 weights; at most 1024 vertices and one million edges. Returned observation is the complete Result of an i128 optional distance matrix.
Observation: complete one-call return; tests observe named pass/failure.
Operator: A popped (d,u) is expanded only when d equals the current tentative entry. Better later paths make prior entries stale. Equal-distance alternatives do not enqueue another improvement; deterministic (distance,vertex) order affects work but not the matrix.
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

This source control log sha256: 91e12a5cae85ccba50af098f7f16b3c6ef6add9e201db5c37691d79d00004ac0. Mutant initializes the source distance to one; exhaustive_signed_graphs fails.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.

### Undated · potential-telescoping

Type: potential_equivariance. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.

### Undated · global-cycle

Type: negative_cycle_domain. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.

### Undated · wide-reweighting

Type: numeric_representation. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.

### Undated · first-failure

Type: error_precedence. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.

### Undated · redundant-arc

Type: information_loss. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.

### Undated · heap-staleness

Type: frontier_filter. Subject/revision: mod.rs sha256 e30d960026937156118eb5d7cc9d38f00cc0d967e7724462bd98400ee0740f79.
