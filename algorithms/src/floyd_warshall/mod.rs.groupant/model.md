# algorithm-floyd-warshall

## descriptant

### floyd_warshall implementation groupant

Original standard-library implementation, searched against existing repository mechanisms before writing.  Source remains adjacent; manifest hash binds this revision and syntax.json is a partial syn projection without macro expansion or semantic resolution.

#### Mathematical combiner

Inputs are source/AST, forward relation, complement, complete preimage relation, specialized observations, invariant, cost and checks. Intersect candidate behavior with invariant constraints and compare exact outputs to independent finite oracles. Transformations add cross-input relations. A compiled mutation must be rejected by one selected assertion test. Registration and source identity do not prove model semantics. Parent library wiring is reviewed separately.

#### Retained observations and limits

Four tests: 19,768 directed graphs with 0..3 vertices and each arc absent/weight0/weight3; 48 seeded graphs with 4..6 vertices and full-width/MAX weights (seed 0xf10d20261029); boundary/parallel/zero-cycle/wide-sum fixtures; transpose plus duplicate/reordered arcs and factor7 scaling. O(V^3+E) time and O(V^2) auxiliary space including output; cloning one pivot row per stage costs O(V^2) total copying. Allocation failures are outside Result semantics; no flattened V*V index multiplication is used. The oracle may recurse because its domain is at most six vertices; production has no recursion. Negative-weight graphs are outside the u64 API. No teacher call, timing gain or full mathematical verification is claimed. Prospective notes, logs and compiled mutants remain outside the repository.

## approximant

### Forward approximant

Validate all endpoints in adjacency order before matrix construction. First invalid arc returns InvalidEdge(from,to,V). Otherwise return V-by-V matrix D where D[u,u]=0, D[u,v] is the minimum directed path weight in u128, or None when unreachable. Parallel edges take their minimum; zero-cost cycles do not alter diagonal zero. Checked candidate addition returns DistanceOverflow rather than wrapping.

#### Cost

O(V^3+E) time and O(V^2) auxiliary space including output; cloning one pivot row per stage costs O(V^2) total copying. Allocation failures are outside Result semantics; no flattened V*V index multiplication is used.

Finite materialized inputs and successful allocation are assumptions. Source inspection and bounded Rust tests support correspondence; this is not a machine-checked proof.

## invariant

### Invariant

After pivot k, each matrix entry is the minimum cost among paths whose internal vertices lie in processed pivots. A cloned pivot row and each row's pivot-column value represent the same recurrence because nonnegative weights preserve zero diagonal. None means no permitted path. Input is immutable. Actual candidate costs fit u128 on materializable <=64-bit graphs; checked additions retain explicit failure behavior.

Global validation precedes the algorithm. Inputs are borrowed immutably; success is deterministic for fixed ordered inputs.

## catalog

# Typed obernant family

## behavior-complement

Domain: finite directed adjacency lists with usize endpoints and nonnegative u64 weights on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: In the explicit one-call universe U of finite directed adjacency lists with usize endpoints and nonnegative u64 weights on <=64-bit usize targets crossed with declared returned values, C=U minus R for the forward relation R. C includes incorrect partitions/forests/distances, wrong error details or wrong first-failure precedence. This is the exact specification complement and a partial source-derived approximant of Rust behavior. Allocation failures, panics, timing and traces are outside U.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 19,768 directed graphs with 0..3 vertices and each arc absent/weight0/weight3; 48 seeded graphs with 4..6 vertices and full-width/MAX weights (seed 0xf10d20261029); boundary/parallel/zero-cycle/wide-sum fixtures; transpose plus duplicate/reordered arcs and factor7 scaling. Four tests pass twice with identical normalized logs; Skip the last pivot by iterating 0..n.saturating_sub(1). Compiled mutant is rejected by exactly exhaustive_ternary_graphs_match_simple_paths (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## reverse-reconstruction

Domain: finite directed adjacency lists with usize endpoints and nonnegative u64 weights on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: For each returned y, Pre(y)={x in domain | R(x,y)} retains ALL compatible inputs. For success, preserve every graph satisfying the forward equations and observable canonical ordering; for errors, preserve every graph matching that exact first-failure condition. Parallel alternatives, internal representations and unobserved edge choices are retained whenever compatible. No unique input recovery or executable inverse is claimed.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 19,768 directed graphs with 0..3 vertices and each arc absent/weight0/weight3; 48 seeded graphs with 4..6 vertices and full-width/MAX weights (seed 0xf10d20261029); boundary/parallel/zero-cycle/wide-sum fixtures; transpose plus duplicate/reordered arcs and factor7 scaling. Four tests pass twice with identical normalized logs; Skip the last pivot by iterating 0..n.saturating_sub(1). Compiled mutant is rejected by exactly exhaustive_ternary_graphs_match_simple_paths (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## transpose

Domain: finite directed adjacency lists with usize endpoints and nonnegative u64 weights on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: For fixed valid weighted G, reverse all arcs preserving weights. The output matrix is transposed, because reversing each path is a weight-preserving bijection. Includes unreachable pairs and empty paths. Executed combined transformation fixture.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 19,768 directed graphs with 0..3 vertices and each arc absent/weight0/weight3; 48 seeded graphs with 4..6 vertices and full-width/MAX weights (seed 0xf10d20261029); boundary/parallel/zero-cycle/wide-sum fixtures; transpose plus duplicate/reordered arcs and factor7 scaling. Four tests pass twice with identical normalized logs; Skip the last pivot by iterating 0..n.saturating_sub(1). Compiled mutant is rejected by exactly exhaustive_ternary_graphs_match_simple_paths (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## scaling

Domain: finite directed adjacency lists with usize endpoints and nonnegative u64 weights on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: For fixed topology and k>=1 with all edge products fitting u64, finite distances multiply by k and None stays None. Executed k=7 fixture, not a claim of testing every factor.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 19,768 directed graphs with 0..3 vertices and each arc absent/weight0/weight3; 48 seeded graphs with 4..6 vertices and full-width/MAX weights (seed 0xf10d20261029); boundary/parallel/zero-cycle/wide-sum fixtures; transpose plus duplicate/reordered arcs and factor7 scaling. Four tests pass twice with identical normalized logs; Skip the last pivot by iterating 0..n.saturating_sub(1). Compiled mutant is rejected by exactly exhaustive_ternary_graphs_match_simple_paths (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## edge-insertion

Domain: finite directed adjacency lists with usize endpoints and nonnegative u64 weights on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: Fix old exact matrix D and add arc a->b of nonnegative weight w. New D'[u,v]=min(D[u,v],D[u,a]+w+D[b,v]), with absent summands infinite. An optimal path need not use the new edge more than once because cycles are nonnegative. Source-derived mathematical model; separate insertion experiment remains unexecuted.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 19,768 directed graphs with 0..3 vertices and each arc absent/weight0/weight3; 48 seeded graphs with 4..6 vertices and full-width/MAX weights (seed 0xf10d20261029); boundary/parallel/zero-cycle/wide-sum fixtures; transpose plus duplicate/reordered arcs and factor7 scaling. Four tests pass twice with identical normalized logs; Skip the last pivot by iterating 0..n.saturating_sub(1). Compiled mutant is rejected by exactly exhaustive_ternary_graphs_match_simple_paths (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## tests

### Checks and measured evidence

Independent oracle: Enumerate every simple path by bounded recursive DFS and minimize costs per target, including the empty path. Since weights are nonnegative, deleting cycles cannot increase cost, so simple paths suffice. The oracle does not invoke Dijkstra, Floyd recurrence or production code.

Transformation: Transpose all arcs, duplicate/reverse adjacency and scale weights by seven in the bounded fixture; output[u,v] must equal seven times original[v,u], preserving None.

Negative control: Skip the last pivot by iterating 0..n.saturating_sub(1). Compiled mutant is rejected by exactly exhaustive_ternary_graphs_match_simple_paths (one selected failed test, exit101). Only that exact filtered test ran; the report asserts one failed test and exit101, excluding timeout. Source mutation exists only in external scratch.

Run cargo test -p mova-algorithms floyd_warshall::tests -- --test-threads=1 after parent export integration. The external probe also compiled standalone modules; Kruskal used a small wrapper exposing the unchanged disjoint-set implementation. Isolated crate total:66 passing tests. Strict cargo clippy --all-targets -- -D warnings and formatting passed. Reports preserve fixed seeds/cases and domain bounds.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Prospective prediction was four tests per algorithm, bounded independent oracle agreement, deterministic repeated outcomes and finite compiled mutant rejection. Each module's four tests passed twice; normalized logs agree after omitting elapsed time. The isolated crate with temporary local exports passed66 tests; strict Clippy and rustfmt checks passed. Skip the last pivot by iterating 0..n.saturating_sub(1). Compiled mutant is rejected by exactly exhaustive_ternary_graphs_match_simple_paths (one selected failed test, exit101). No production bug was silently patched; initial lint fixes used is_multiple_of in fixtures only.

### 2026-10-29 · tests

2026-10-29: four targeted tests pass twice with normalized-identical output. Four tests: 19,768 directed graphs with 0..3 vertices and each arc absent/weight0/weight3; 48 seeded graphs with 4..6 vertices and full-width/MAX weights (seed 0xf10d20261029); boundary/parallel/zero-cycle/wide-sum fixtures; transpose plus duplicate/reordered arcs and factor7 scaling.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: mod.rs at sha256 715eaefa9d4eec73ed9dcd2a2f2d1ced2ef7032f7f6a3670285557cf214049e0.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs at sha256 715eaefa9d4eec73ed9dcd2a2f2d1ced2ef7032f7f6a3670285557cf214049e0.

### Undated · transpose

Type: distance_matrix_transpose. Subject/revision: mod.rs at sha256 715eaefa9d4eec73ed9dcd2a2f2d1ced2ef7032f7f6a3670285557cf214049e0.

### Undated · scaling

Type: positive_weight_scaling. Subject/revision: mod.rs at sha256 715eaefa9d4eec73ed9dcd2a2f2d1ced2ef7032f7f6a3670285557cf214049e0.

### Undated · edge-insertion

Type: min_plus_update. Subject/revision: mod.rs at sha256 715eaefa9d4eec73ed9dcd2a2f2d1ced2ef7032f7f6a3670285557cf214049e0.
