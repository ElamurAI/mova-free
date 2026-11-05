# algorithm-bridges

## descriptant

### bridges mod.rs groupant

Original Mova Rust code and fixtures; no copied problem-site implementations. Conceptual provenance: standard undirected DFS lowlink separator criteria (Tarjan, Depth-First Search and Linear Graph Algorithms, 1972) and Hierholzer edge-splicing traversal (1873), independently expressed using explicit stacks. Repository search found weighted Kruskal Edge, BFS and SCC but no reusable undirected separator or Euler implementation. The modules reuse bridges::adjacency and its endpoint Error contract; tests reuse independent graph fixtures/closure in bridges::tests.

#### Mathematical combiner

Combine source-bound partial syntax, forward relation, the ordered typed obernant family, invariant, cost and checks by intersecting their constraints. The sibling test groupant observes the exact public function; sibling graph consumers inherit adjacency validation and edge identity. Correct traversal in isolation cannot prove shared dependency wiring or parent library exports. Parent integration is a separate check. Syntax excludes macro expansion and semantic resolution.

#### Retained limits

O(V+E) time and O(V+E) auxiliary space. Each adjacency entry is consumed at most once; output scan/reversal is linear. No call-stack recursion. Validating endpoints costs O(E) before allocating V adjacency lists. Allocation failure is outside the model. Endpoint orientation preserves valid inputs only; invalid u-before-v precedence is observable. Euler canonical paths need not survive arbitrary edge permutation. Tests are finite and do not prove unbounded correctness. Executable approximation profiles are an explicit gap: separator identity and Euler feasibility are categorical contracts without a defined useful relaxed metric; exact oracle harnesses are not speed surrogates.

## approximant

### Forward approximant

Validate all endpoints in input order, u before v. Return ascending input edge IDs e exactly when c(G-e)>c(G), counting isolated vertices as components. Iterative DFS records discovery d[v], low[v], parent edge identity and next adjacency cursor. Child v yields a bridge iff low[v]>d[parent[v]]. Parallel parent edges remain back edges; loops cannot be bridges.

#### Cost

O(V+E) time and O(V+E) auxiliary space. Each adjacency entry is consumed at most once; output scan/reversal is linear. No call-stack recursion. Validating endpoints costs O(E) before allocating V adjacency lists. Allocation failure is outside the model.

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.

All endpoint checks precede traversal. Source correspondence remains partial.

## invariant

### Invariant

On DFS completion low[v] is the minimum discovery time reachable through its descendant tree and at most one non-parent edge. Each tree edge is tested once; the final ascending Boolean scan contains precisely separator edge identities.

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
Operator/target: Hold a valid graph fixed and permute input edge identities by bijection p. The bridge set becomes sorted p(B); reversing the edge list and mapping IDs back is executed.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## edge-deletion-fiber

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold G fixed; the output membership fiber for identity e is exactly c(G-e)>c(G). Independent transitive-closure deletion checks this equation on the finite corpus.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## parallel-cycle-fiber

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold endpoints fixed; a loop has identical endpoint connectivity after its deletion. If two distinct edges join the same endpoints, deleting either preserves reachability through the other; neither identity can occur in the bridge output.
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

Negative control: Change low[v] > seen[p] to low[v] >= seen[p]; bridges::tests::deletion_oracle rejects the compiled mutant.

## History

### 2026-10-30 · descriptant

#### Modeling epoch 2026-10-30

Prospective expectation: linear production cost, deterministic outcomes, deletion/trail-oracle agreement and compiled mutation rejection. 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Change low[v] > seen[p] to low[v] >= seen[p]; bridges::tests::deletion_oracle rejects the compiled mutant. Initial source formatting and a test-module visibility adjustment preceded the first successful build; no algorithmic fix was needed after oracle execution.

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

### 2026-10-30 · edge-deletion-fiber

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · parallel-cycle-fiber

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · iterative-resource

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · tests

2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness.

Run cargo test -p mova-algorithms bridges::tests after parent exports are integrated. This file source hash: 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db. Raw logs, prospective notes, compiled mutants and Rust orchestration are retained in the external run directory. No measured performance gain is claimed.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.

### Undated · orientation

Type: undirected_representation_equivalence. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.

### Undated · isolated-extension

Type: isolated_vertex_extension. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.

### Undated · identity-permutation

Type: identity_transport. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.

### Undated · edge-deletion-fiber

Type: separator_characterization. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.

### Undated · parallel-cycle-fiber

Type: cycle_exclusion. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.

### Undated · iterative-resource

Type: cursor_resource_relation. Subject/revision: algorithms/src/bridges/mod.rs, sha256 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db.
