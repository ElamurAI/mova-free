# algorithm-articulation-points-tests

## descriptant

### articulation_points tests.rs groupant

Original Mova Rust code and fixtures; no copied problem-site implementations. Conceptual provenance: standard undirected DFS lowlink separator criteria (Tarjan, Depth-First Search and Linear Graph Algorithms, 1972) and Hierholzer edge-splicing traversal (1873), independently expressed using explicit stacks. Repository search found weighted Kruskal Edge, BFS and SCC but no reusable undirected separator or Euler implementation. The modules reuse bridges::adjacency and its endpoint Error contract; tests reuse independent graph fixtures/closure in bridges::tests.

#### Mathematical combiner

Combine source-bound partial syntax, forward relation, the ordered typed obernant family, invariant, cost and checks by intersecting their constraints. The sibling test groupant observes the exact public function; sibling graph consumers inherit adjacency validation and edge identity. Correct traversal in isolation cannot prove shared dependency wiring or parent library exports. Parent integration is a separate check. Syntax excludes macro expansion and semantic resolution.

#### Retained limits

Finite corpus cost: connectivity closure is O(V^3) per deletion and exhaustive trail search is O(E! * E) in the worst case; bounded at V<=5, E<=10. Deep-chain production fixture is O(30000) time/space; no general oracle efficiency claim. Endpoint orientation preserves valid inputs only; invalid u-before-v precedence is observable. Euler canonical paths need not survive arbitrary edge permutation. Tests are finite and do not prove unbounded correctness. Executable approximation profiles are an explicit gap: separator identity and Euler feasibility are categorical contracts without a defined useful relaxed metric; exact oracle harnesses are not speed surrogates.

## approximant

### Forward approximant

Generate all bounded ordered multigraphs and simple graphs; independently compute deletion connectivity or exhaustively enumerate edge trails. Assert exact production agreement, validate witnesses, exercise boundaries and transformations, and run a deep chain. Return harness pass only if every assertion holds.

#### Cost

Finite corpus cost: connectivity closure is O(V^3) per deletion and exhaustive trail search is O(E! * E) in the worst case; bounded at V<=5, E<=10. Deep-chain production fixture is O(30000) time/space; no general oracle efficiency claim.

Domain: Fixed deterministic Rust fixtures and production functions on the finite graph corpus; one complete test invocation with pass/fail assertions.

All endpoint checks precede traversal. Source correspondence remains partial.

## invariant

### Invariant

Every corpus edge endpoint is within V; the deletion oracle removes the vertex from component counting, and trail validation checks unique IDs, incidence and length independently. Shared helpers reside in bridges::tests; only cfg(test) exposes them.

Borrowed inputs remain immutable; outputs and failures are deterministic for fixed ordered inputs.

## catalog

# Typed obernant family

## behavior-complement

Domain: Fixed deterministic Rust fixtures and production functions on the finite graph corpus; one complete test invocation with pass/fail assertions.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Let U be fixed harness configurations crossed with pass/fail outcomes and R the forward assertion semantics. U minus R excludes a passing run with a violated checked equality or malformed witness. This is an exact specification complement and only a partial hypothesis about compiler/runner correspondence; finite passing evidence is not its definition.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## reverse-reconstruction

Domain: Fixed deterministic Rust fixtures and production functions on the finite graph corpus; one complete test invocation with pass/fail assertions.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: The preimage of pass comprises every implementation and runner state agreeing with all executed observations, including incorrect implementations outside sampled cases. The preimage of assertion failure comprises every state falsifying at least one reached assertion; it does not uniquely locate a defect.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## corpus-boundary

Domain: Fixed deterministic Rust fixtures and production functions on the finite graph corpus; one complete test invocation with pass/fail assertions.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: The corpus covers all ordered sequences over unordered endpoint records of lengths 0..5 on V<=3, plus all simple graphs on V=4,5. Orientations beyond this representation are exercised by reversal; this is not every multigraph on five vertices.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## mutant-detection

Domain: Fixed deterministic Rust fixtures and production functions on the finite graph corpus; one complete test invocation with pass/fail assertions.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Change low[v] >= seen[p] to low[v] > seen[p]; articulation_points::tests::deletion_oracle rejects the compiled mutant.
Approximation semantics: exact specification relation under stated assumptions; partial source-derived correspondence, not an unrestricted proof.
Assumptions: trusted Rust compiler and standard library, successful allocation, fixed ordered inputs, test runner executes every selected assertion.
Status: partial_source_derived_model.

## oracle-separation

Domain: Fixed deterministic Rust fixtures and production functions on the finite graph corpus; one complete test invocation with pass/fail assertions.
Observation scope: one completed invocation; allocator failure, timing and external effects excluded.
Operator/target: Hold a finite graph fixed. Boolean transitive closure after deletion does not share lowlink code. Direct edge-trail search does not use degree/connectivity existence tests or production adjacency. Shared fixture enumeration and Edge data remain common dependencies; oracle bugs are still possible.
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

### 2026-10-30 · corpus-boundary

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · mutant-detection

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · oracle-separation

Evidence: 2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness. Specialized unexecuted generalizations remain mathematical reasoning.

### 2026-10-30 · tests

2026-10-30: five tests per module; all 229 crate tests passed twice in debug and twice in release with identical normalized test records. Strict Clippy and rustfmt passed. Each module's compiled production mutant failed one exact oracle test with an assertion and exit 101. Exhaustive corpus: 10,790 graphs (all ordered non-oriented edge records of length 0..5 on 0..3 vertices, plus all simple graphs on 4 and 5 vertices). Fixtures cover invalid endpoint precedence, usize::MAX, loops, parallel edges, disconnected support, isolates, empty graphs, and a 30,000-vertex chain. Finite tests do not establish unrestricted correctness.

Run cargo test -p mova-algorithms articulation_points::tests after parent exports are integrated. This file source hash: fae905d370de49b62b595b3762fbd37dc5df61491c6b5ec815d8bdf7363460c3. Raw logs, prospective notes, compiled mutants and Rust orchestration are retained in the external run directory. No measured performance gain is claimed.

### 2026-11-01 · descriptant

#### Bounded review 2026-11-01

The deletion test observes the strict increase `c(G-v) > c(G)`; the helper excludes the deleted vertex itself from the count. For vertices 0, 1, 2 with only edge 0–1, deleting isolated 2 changes the count from 2 to 1, so replacing `>` by `!=` falsely labels 2. An independent stack-based component count and the actual production API confirmed this witness; the five focused tests passed. This is a finite control, not a proof for arbitrary graphs. The transformation test checks endpoint/order reversal together with adding an isolate, and vertex reflection separately; it does not independently isolate every transformation or reconstruct the original graph. The pass fiber includes implementations that agree only on these observations. The generic forward description above also mentions trail enumeration, which belongs to a sibling test module, not this source; no trail oracle is invoked here.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/articulation_points/tests.rs, sha256 fae905d370de49b62b595b3762fbd37dc5df61491c6b5ec815d8bdf7363460c3.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/articulation_points/tests.rs, sha256 fae905d370de49b62b595b3762fbd37dc5df61491c6b5ec815d8bdf7363460c3.

### Undated · corpus-boundary

Type: finite_domain_coverage. Subject/revision: algorithms/src/articulation_points/tests.rs, sha256 fae905d370de49b62b595b3762fbd37dc5df61491c6b5ec815d8bdf7363460c3.

### Undated · mutant-detection

Type: counterfactual_assertion_sensitivity. Subject/revision: algorithms/src/articulation_points/tests.rs, sha256 fae905d370de49b62b595b3762fbd37dc5df61491c6b5ec815d8bdf7363460c3.

### Undated · oracle-separation

Type: independent_observation_relation. Subject/revision: algorithms/src/articulation_points/tests.rs, sha256 fae905d370de49b62b595b3762fbd37dc5df61491c6b5ec815d8bdf7363460c3.
