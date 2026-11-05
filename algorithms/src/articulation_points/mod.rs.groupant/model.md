# algorithm-articulation-points

## descriptant

### articulation_points mod.rs groupant

Original Mova Rust code and fixtures; no copied problem-site implementations. Conceptual provenance: standard undirected DFS lowlink separator criteria (Tarjan, Depth-First Search and Linear Graph Algorithms, 1972) and Hierholzer edge-splicing traversal (1873), independently expressed using explicit stacks. Repository search found weighted Kruskal Edge, BFS and SCC but no reusable undirected separator or Euler implementation. The modules reuse bridges::adjacency and its endpoint Error contract; tests reuse independent graph fixtures/closure in bridges::tests.

#### Mathematical combiner

Combine source-bound partial syntax, forward relation, the ordered typed obernant family, invariant, cost and checks by intersecting their constraints. The sibling test groupant observes the exact public function; sibling graph consumers inherit adjacency validation and edge identity. Correct traversal in isolation cannot prove shared dependency wiring or parent library exports. Parent integration is a separate check. Syntax excludes macro expansion and semantic resolution.

#### Retained limits

O(V+E) time and O(V+E) auxiliary space. Each adjacency entry is consumed at most once; output scan/reversal is linear. No call-stack recursion. Validating endpoints costs O(E) before allocating V adjacency lists. Allocation failure is outside the model. Endpoint orientation preserves valid inputs only; invalid u-before-v precedence is observable. Euler canonical paths need not survive arbitrary edge permutation. Tests are finite and do not prove unbounded correctness. Executable approximation profiles are an explicit gap: separator identity and Euler feasibility are categorical contracts without a defined useful relaxed metric; exact oracle harnesses are not speed surrogates.

## approximant

### Forward approximant

Use bridges::adjacency for validation and ordered undirected adjacency; first invalid edge and endpoint precedence are inherited exactly. Return ascending vertices v exactly when c(G-v)>c(G), where G-v removes v and all its incident edges. Iterative DFS uses lowlink and parent edge identities. A nonroot p is marked when a child v has low[v]>=d[p]; a root is marked precisely when it has more than one DFS child.

#### Cost

O(V+E) time and O(V+E) auxiliary space. Each adjacency entry is consumed at most once; output scan/reversal is linear. No call-stack recursion. Validating endpoints costs O(E) before allocating V adjacency lists. Allocation failure is outside the model.

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.

All endpoint checks precede traversal. Source correspondence remains partial.

## invariant

### Invariant

Parent edges identify DFS tree edges uniquely in a multigraph. Completed children propagate lowlink once. A nonroot cut flag accumulates separated descendant subtrees; root flags depend only on DFS child count. Output flags are scanned in vertex order.

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
Operator/target: Hold vertex names fixed and permute edges arbitrarily; articulation set is unchanged. Under vertex bijection p the answer is sorted p(A). Edge reversal and reversed vertex numbering are executed.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## vertex-deletion-fiber

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold G fixed; v belongs to the output iff c(G-v)>c(G), with v absent from the component universe. The Boolean closure oracle checks every candidate vertex independently.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## root-boundary

Domain: Finite ordered undirected multigraph Edge {u: usize, v: usize} inputs and vertex count V; successful allocations and representable materialized vectors.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold a DFS forest fixed. A root is a cut vertex iff it has at least two DFS children. A nonroot with a child returning only to itself is still a cut vertex: low[child]=d[parent] satisfies the non-strict threshold. Isolated roots and roots with one child are excluded.
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

Negative control: Change low[v] >= seen[p] to low[v] > seen[p]; articulation_points::tests::deletion_oracle rejects the compiled mutant.

## History

### 2026-10-30 · descriptant

#### Modeling epoch 2026-10-30

Prospective expectation: linear production cost, deterministic outcomes, deletion/trail-oracle agreement and compiled mutation rejection. 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Change low[v] >= seen[p] to low[v] > seen[p]; articulation_points::tests::deletion_oracle rejects the compiled mutant. Initial source formatting and a test-module visibility adjustment preceded the first successful build; no algorithmic fix was needed after oracle execution.

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

### 2026-10-30 · vertex-deletion-fiber

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · root-boundary

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · iterative-resource

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · tests

2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness.

Run cargo test -p mova-algorithms articulation_points::tests after parent exports are integrated. This file source hash: c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114. Raw logs, prospective notes, compiled mutants and Rust orchestration are retained in the external run directory. No measured performance gain is claimed.

### 2026-11-01 · descriptant

#### Review 2026-11-01: vertex and edge deletion observations

The shared adjacency and parent-edge identity support two distinct observations: articulation tests vertex deletion with a non-strict lowlink threshold, while bridges tests edge deletion with a strict threshold. In the graph with edges (0,1), (1,2), (2,3), (3,1), the actual APIs return vertex `[1]` and edge ID `[0]`; an independent component-count probe confirms both. The DFS child at 2 can return to 1, so equality is meaningful: changing either threshold conflates the two deletion questions. A path (0,1), (1,2) returns articulation `[1]` and bridges `[0,1]`, showing the same vertex result has a different edge fiber. These finite checks do not prove all graphs or reconstruct the DFS state from outputs. The shared adjacency error contract is upstream of both traversals; this observation assumes valid endpoints and excludes errors and allocation failure.

### 2026-11-01 · document

#### Vertex and edge observation boundary — 2026-11-01

For valid graphs, the specification projections are A(G) = {v | c(G-v) > c(G)} and B(G) = {i | c(G-e_i) > c(G)}. The latter retains input edge identities and is not a reconstruction of the former: a triangle with a tail and a two-edge path both have A = {1}, but B is {0} and {0,1} respectively. The triangle-tail witness distinguishes the non-strict nonroot vertex threshold from the strict bridge threshold. This is a partial cross-API observation, not a behavior complement or an inverse, and the two finite witnesses are not a general proof. Errors and allocator failure lie outside this relation.

### 2026-11-01 · tests

2026-11-01: an external Rust probe calls both actual APIs and independently counts components after each vertex and edge deletion for a triangle attached to a tail. It also checks a path with the same articulation output but different bridge output. Both probe runs match byte for byte. The proposed strict articulation threshold and non-strict bridge threshold are falsified by the triangle-tail oracle; no mutant was executed in this review. This is finite evidence, not a universal proof.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.

### Undated · orientation

Type: undirected_representation_equivalence. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.

### Undated · isolated-extension

Type: isolated_vertex_extension. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.

### Undated · identity-permutation

Type: identity_transport. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.

### Undated · vertex-deletion-fiber

Type: separator_characterization. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.

### Undated · root-boundary

Type: root_child_partition. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.

### Undated · iterative-resource

Type: cursor_resource_relation. Subject/revision: algorithms/src/articulation_points/mod.rs, sha256 c4fedb337fef1f5527f7e9d8fafce655529e34fe572ca9efa8e142c7e0382114.
