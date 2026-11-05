# algorithm-transitive_closure

## descriptant

### transitive_closure mod.rs

Check size limits then every target in adjacency order. Initialize diagonal and arcs in u64 words; for k in vertex order, clone row k and OR it into each row containing bit k. reachable(u,v) returns None for either invalid endpoint, else the stored bit.

Composition: Private packed rows compose pivot updates with checked pair queries; traversal supplies an independent relation.

No executable percentage-error surrogate: the public operation remains exact, and no useful approximate observation metric is claimed. Allocation failure, timing and general semantic resolution remain outside these models.

## approximant

### Forward and cost

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.

Check size limits then every target in adjacency order. Initialize diagonal and arcs in u64 words; for k in vertex order, clone row k and OR it into each row containing bit k. reachable(u,v) returns None for either invalid endpoint, else the stored bit.

Cost: O(V² ceil(V/64)+E) time; O(V ceil(V/64)) persistent storage and one row scratch. Limits bound packed storage; allocation failure remains outside typed errors.

## invariant

### Invariant

After pivot k, bits represent walks with internal vertices among 0..k. All diagonal bits stay set. Padding bits stay zero. Final relation is reflexive and transitive, contains every arc and is minimal with these properties.

## catalog

# Typed model family

## behavior-complement

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Let U be declared inputs crossed with returned observations and R the forward relation in model.md#approximant. C=U minus R includes wrong first errors, wrong values and forbidden results. This is an exact specification complement and partial source-derived model, not a measured global Rust complement. After pivot k, bits represent walks with internal vertices among 0..k. All diagonal bits stay set. Padding bits stay zero. Final relation is reflexive and transitive, contains every arc and is minimal with these properties. 
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## reverse-reconstruction

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Pre(y)={x in domain | R(x,y)} retains all compatible inputs. Outputs lose redundant edges and intermediate heap/pivot traces; errors retain all graphs sharing their first-failure condition. No unique recovery or inverse executor is claimed.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## reflexive-boundary

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Even an isolated valid vertex reaches itself by an empty path; an empty graph has no valid pair. None is not Some(false). Executed empty, one-vertex and invalid-query fixtures.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## word-layout

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Vertex v maps to word floor(v/64), bit v mod64. Unused final-word bits remain zero and inaccessible. Directed chains across 63/64/65 and 127/128/129 verify boundary behavior.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## saturation

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Replacing arcs by every true output pair yields exactly the same closure, provided edge cap remains satisfied. The saturated representation loses original edge selection; executed three-vertex saturation.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## duplicate-fiber

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Duplicate arcs and existing self-loops preserve all reachable pairs when limits remain satisfied, but can change ResourceLimit status at the boundary. Executed duplicate removal and exhaustive loop enumeration.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## direction

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: Reachable(u,v) does not imply Reachable(v,u); increasing directed chains satisfy reachable(u,v) iff u<=v. Exhaustive traversal and word-boundary chain checks falsify accidental symmetrization.
Approximation: exact specification under stated assumptions; partial source correspondence.
Assumptions: standard Rust integer semantics, trusted compiler and successful allocation; limits and order as documented.
Evidence: 4 tests: all directed graphs including loops on 0..4 vertices (66067 graphs) versus independent stack traversal; directed chains at seven word-boundary sizes; vertex/edge limits, invalid query/edge details, closure idempotence and duplicate fibers. See model.md#tests for execution and controls.
Status: partial_source_derived_model. No semantic AST or general proof.

## validation

Domain: Ordered directed adjacency lists of usize targets; at most 8192 vertices and one million arcs; complete Result of private packed reflexive reachability plus queries.
Observation: complete one-call return; tests observe named pass/failure.
Operator: ResourceLimit is determined before endpoints. Otherwise the first adjacency-order invalid target defines InvalidEdge; all graph rows are validated before any packed output. Queries reject both out-of-range endpoints independently.
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

This source control log sha256: c7c78656cc3e9d7aa2e74b5753f4ebcd36794226534dbd4290382f2d77633f1f. Mutant omits reflexive initialization; exhaustive_directed_graphs fails.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.

### Undated · reflexive-boundary

Type: empty_path_semantics. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.

### Undated · word-layout

Type: representation_projection. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.

### Undated · saturation

Type: closure_idempotence. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.

### Undated · duplicate-fiber

Type: multigraph_equivalence. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.

### Undated · direction

Type: directed_order. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.

### Undated · validation

Type: first_failure_partition. Subject/revision: mod.rs sha256 8676dda3bae42c97f11356a8288b364f8375795229a9a296240292fe4f106aee.
