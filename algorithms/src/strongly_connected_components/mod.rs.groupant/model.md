# algorithm-strongly-connected-components

## descriptant

### strongly_connected_components implementation groupant

Original standard-library implementation, searched against existing repository mechanisms before writing.  Source remains adjacent; manifest hash binds this revision and syntax.json is a partial syn projection without macro expansion or semantic resolution.

#### Mathematical combiner

Inputs are source/AST, forward relation, complement, complete preimage relation, specialized observations, invariant, cost and checks. Intersect candidate behavior with invariant constraints and compare exact outputs to independent finite oracles. Transformations add cross-input relations. A compiled mutation must be rejected by one selected assertion test. Registration and source identity do not prove model semantics. Parent library wiring is reviewed separately.

#### Retained observations and limits

Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. O(V+E+V log(V+1)) time, O(V+E) auxiliary space; sorting is included, so canonical output is not claimed linear-time. The long-chain case checks explicit-stack behavior at10,000 vertices; arbitrary allocation capacity remains unproved. No teacher call, timing gain or full mathematical verification is claimed. Prospective notes, logs and compiled mutants remain outside the repository.

## approximant

### Forward approximant

Validate every endpoint in adjacency order before traversal; first invalid arc returns InvalidEdge(from,to,V). Otherwise return the unique partition with u,v in the same block iff both are mutually reachable. Sort each block ascending and order blocks by their smallest element.

#### Cost

O(V+E+V log(V+1)) time, O(V+E) auxiliary space; sorting is included, so canonical output is not claimed linear-time.

Finite materialized inputs and successful allocation are assumptions. Source inspection and bounded Rust tests support correspondence; this is not a machine-checked proof.

## invariant

### Invariant

First DFS records vertices on exit after all adjacency entries have been inspected; each stack frame stores its next edge. Second traversal uses the transposed graph and decreasing finish order, assigns each vertex once, then canonicalizes nonempty blocks. All graph traversals use explicit stacks.

Global validation precedes the algorithm. Inputs are borrowed immutably; success is deterministic for fixed ordered inputs.

## catalog

# Typed obernant family

## behavior-complement

Domain: finite directed adjacency lists, including empty graphs, loops and parallel arcs.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: In the explicit one-call universe U of finite directed adjacency lists, including empty graphs, loops and parallel arcs crossed with declared returned values, C=U minus R for the forward relation R. C includes incorrect partitions/forests/distances, wrong error details or wrong first-failure precedence. This is the exact specification complement and a partial source-derived approximant of Rust behavior. Allocation failures, panics, timing and traces are outside U.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## reverse-reconstruction

Domain: finite directed adjacency lists, including empty graphs, loops and parallel arcs.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: For each returned y, Pre(y)={x in domain | R(x,y)} retains ALL compatible inputs. For success, preserve every graph satisfying the forward equations and observable canonical ordering; for errors, preserve every graph matching that exact first-failure condition. Parallel alternatives, internal representations and unobserved edge choices are retained whenever compatible. No unique input recovery or executable inverse is claimed.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## transpose-fiber

Domain: finite directed adjacency lists, including empty graphs, loops and parallel arcs.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: Fix V and a valid graph G. Every graph obtained by transposing all arcs, changing positive multiplicities or adjacency order has the same mutual-reachability partition; vertex labels are held fixed. Tested with seeded transpose/duplicate fixtures.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## edge-addition

Domain: finite directed adjacency lists, including empty graphs, loops and parallel arcs.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: For fixed V and valid G, adding arcs can only merge existing SCC blocks, never split one: mutual reachability in G implies mutual reachability in G+Enew. Exact graph relation; source-derived model, not separately executed as a metamorphic test.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## condensation

Domain: finite directed adjacency lists, including empty graphs, loops and parallel arcs.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: Map each vertex to its returned block and keep cross-block arcs. The quotient is acyclic; a quotient cycle would imply mutual reachability and contradict distinct blocks. Holds for all compatible antecedent graphs, not a unique reconstruction. Source-derived reasoning; no separate quotient executor.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## tests

### Checks and measured evidence

Independent oracle: Independent Boolean transitive closure computes all-pairs reachability; group u,v iff both directions hold. It does not use finish ordering, graph transposition or the implementation stacks.

Transformation: Transpose every arc, duplicate arcs and reverse adjacency order; the canonical SCC partition is unchanged.

Negative control: Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Only that exact filtered test ran; the report asserts one failed test and exit101, excluding timeout. Source mutation exists only in external scratch.

Run cargo test -p mova-algorithms strongly_connected_components::tests -- --test-threads=1 after parent export integration. The external probe also compiled standalone modules; Kruskal used a small wrapper exposing the unchanged disjoint-set implementation. Isolated crate total:66 passing tests. Strict cargo clippy --all-targets -- -D warnings and formatting passed. Reports preserve fixed seeds/cases and domain bounds.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Prospective prediction was four tests per algorithm, bounded independent oracle agreement, deterministic repeated outcomes and finite compiled mutant rejection. Each module's four tests passed twice; normalized logs agree after omitting elapsed time. The isolated crate with temporary local exports passed66 tests; strict Clippy and rustfmt checks passed. Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). No production bug was silently patched; initial lint fixes used is_multiple_of in fixtures only.

### 2026-10-29 · tests

2026-10-29: four targeted tests pass twice with normalized-identical output. Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: mod.rs at sha256 5b20b37768e1ff0bb867c32b27f94fc64fec40ea6245c61498c01bbb2a59925e.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs at sha256 5b20b37768e1ff0bb867c32b27f94fc64fec40ea6245c61498c01bbb2a59925e.

### Undated · transpose-fiber

Type: representation_equivalence. Subject/revision: mod.rs at sha256 5b20b37768e1ff0bb867c32b27f94fc64fec40ea6245c61498c01bbb2a59925e.

### Undated · edge-addition

Type: partition_coarsening. Subject/revision: mod.rs at sha256 5b20b37768e1ff0bb867c32b27f94fc64fec40ea6245c61498c01bbb2a59925e.

### Undated · condensation

Type: acyclic_quotient. Subject/revision: mod.rs at sha256 5b20b37768e1ff0bb867c32b27f94fc64fec40ea6245c61498c01bbb2a59925e.
