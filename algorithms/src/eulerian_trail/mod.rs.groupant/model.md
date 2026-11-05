# algorithm-eulerian-trail

## descriptant

### eulerian_trail mod.rs groupant

Original Mova Rust code and fixtures; no copied problem-site implementations. Conceptual provenance: standard undirected DFS lowlink separator criteria (Tarjan, Depth-First Search and Linear Graph Algorithms, 1972) and Hierholzer edge-splicing traversal (1873), independently expressed using explicit stacks. Repository search found weighted Kruskal Edge, BFS and SCC but no reusable undirected separator or Euler implementation. The modules reuse bridges::adjacency and its endpoint Error contract; tests reuse independent graph fixtures/closure in bridges::tests.

#### Mathematical combiner

Combine source-bound partial syntax, forward relation, the ordered typed obernant family, invariant, cost and checks by intersecting their constraints. The sibling test groupant observes the exact public function; sibling graph consumers inherit adjacency validation and edge identity. Correct traversal in isolation cannot prove shared dependency wiring or parent library exports. Parent integration is a separate check. Syntax excludes macro expansion and semantic resolution.

#### Retained limits

O(V+E) time and O(V+E) auxiliary space. Each adjacency entry is consumed at most once; output scan/reversal is linear. No call-stack recursion. Validating endpoints costs O(E) before allocating V adjacency lists. Allocation failure is outside the model. Endpoint orientation preserves valid inputs only; invalid u-before-v precedence is observable. Euler canonical paths need not survive arbitrary edge permutation. Tests are finite and do not prove unbounded correctness. Executable approximation profiles are an explicit gap: separator identity and Euler feasibility are categorical contracts without a defined useful relaxed metric; exact oracle harnesses are not speed surrogates.

## approximant

### Forward approximant

Use bridges::adjacency for complete endpoint validation before existence decisions. Empty edges return Some(Trail([],[])) if V=0 and Some(Trail([0],[])) otherwise. For nonempty edges require zero or two odd-degree vertices. Start at the smallest odd vertex, otherwise the smallest incident vertex. Iterative Hierholzer scans adjacency in input edge order, marks each edge identity once, and collects vertices and incoming edges on stack exit; reverse both. Return None if collected edge count differs from E, thereby excluding disconnected edge support.

#### Cost

O(V+E) time and O(V+E) auxiliary space. Each adjacency entry is consumed at most once; output scan/reversal is linear. No call-stack recursion. Validating endpoints costs O(E) before allocating V adjacency lists. Allocation failure is outside the model.

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.

All endpoint checks precede traversal. Source correspondence remains partial.

## invariant

### Invariant

Each used identity enters the explicit stack exactly once; loops have two adjacency entries but one identity. Exit order and incoming edge labels reconstruct a contiguous trail after reversal. Successful output has E unique IDs and E+1 valid vertices for nonempty input; disconnected support fails the final count.

Borrowed inputs remain immutable; outputs and failures are deterministic for fixed ordered inputs.

## catalog

# Typed obernant family

## behavior-complement

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Let U be declared inputs crossed with all returned Result values, and R the forward specification relation. C=U minus R includes wrong separator memberships, malformed/noncanonical trails, wrong existence decisions and incorrect first-error details. This is an exact specification complement, with only partial source-derived correspondence to actual Rust; no complement guarantee follows merely from a forward approximation.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## reverse-reconstruction

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: For output y retain Pre(y)={G | R(G,y)} with all compatible V, ordered edges and orientation choices. Separator vectors constrain deletion connectivity rather than recover a unique graph. Euler outputs constrain identity-specific incidences and deterministic traversal; None retains all non-Eulerian valid graphs. Error fibers retain all inputs whose first invalid endpoint equals the reported edge, vertex and V.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## orientation

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold valid ordered edge IDs fixed and swap u/v for any edge; undirected adjacency traversal and results are unchanged. Invalid endpoint precedence is excluded from this equivalence. All-edge reversal is executed on the corpus.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## isolated-extension

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold nonempty valid edges fixed and append an isolated vertex. Bridge/articulation answers and the canonical Euler trail are unchanged. Empty Euler input has a special V=0 to V=1 vertex-list change, explicitly excluded.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## identity-permutation

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold an undirected graph fixed and permute input edges. Existence is invariant, but the canonical returned path can change with adjacency order; mapping edge IDs alone need not restore the original path. This distinction is modeled, not an assertion of canonical permutation invariance.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## existence-fiber

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: For nonempty valid G, Some is possible iff all incident vertices form one connected component and exactly zero or two degrees are odd, with loops contributing two. The independent oracle enumerates edge trails directly without using this criterion.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## trail-preimage

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Fix a successful observed trail (vertices, IDs) and V. Compatible graphs assign each identity the unordered endpoints of the corresponding consecutive vertex pair, allowing either orientation, and must reproduce the declared deterministic start/traversal. For empty trail use the explicit V=0 versus V>0 convention. Orientation ambiguity remains; this is not unique graph reconstruction.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## iterative-resource

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold V,E fixed and assume successful allocation. Each of 2E adjacency entries advances one cursor at most once; each discovered vertex or used edge adds at most one active frame. Thus explicit storage is O(V+E) and DFS call-stack depth is constant. A 30000-vertex chain tests a finite boundary, not an allocator bound.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## tests

### Checks and evidence

Negative control: Replace if trail.edges.len() != edges.len() with if false; eulerian_trail::tests::exhaustive_trail_oracle rejects the compiled mutant.

## History

### 2026-10-30 · descriptant

#### Modeling epoch 2026-10-30

Prospective expectation: linear production cost, deterministic outcomes, deletion/trail-oracle agreement and compiled mutation rejection. 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Replace if trail.edges.len() != edges.len() with if false; eulerian_trail::tests::exhaustive_trail_oracle rejects the compiled mutant. Initial source formatting and a test-module visibility adjustment preceded the first successful build; no algorithmic fix was needed after oracle execution.

### 2026-10-30 · behavior-complement

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · reverse-reconstruction

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · orientation

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · isolated-extension

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · identity-permutation

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · existence-fiber

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · trail-preimage

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · iterative-resource

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · tests

2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness.

Run cargo test -p mova-algorithms eulerian_trail::tests after parent exports are integrated. This file source hash: abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d. Raw logs, prospective notes, compiled mutants and Rust orchestration are retained in the external run directory. No measured performance gain is claimed.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.

### Undated · orientation

Type: undirected_representation_equivalence. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.

### Undated · isolated-extension

Type: isolated_vertex_extension. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.

### Undated · identity-permutation

Type: identity_transport. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.

### Undated · existence-fiber

Type: degree_connectivity_precondition. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.

### Undated · trail-preimage

Type: edge_incidence_reconstruction. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.

### Undated · iterative-resource

Type: cursor_resource_relation. Subject/revision: algorithms/src/eulerian_trail/mod.rs, sha256 abbe71d3cf6a69454b00e5a2dcb59b90946216e53ea5c5fb3887062dcaf7891d.
