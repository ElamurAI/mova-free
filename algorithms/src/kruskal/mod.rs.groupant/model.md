# algorithm-kruskal

## descriptant

### kruskal implementation groupant

Original standard-library implementation, searched against existing repository mechanisms before writing. Kruskal composes with the existing disjoint-set module; its partition/union behavior is a dependency, not a duplicate implementation. Source remains adjacent; manifest hash binds this revision and syntax.json is a partial syn projection without macro expansion or semantic resolution.

#### Mathematical combiner

Inputs are source/AST, forward relation, complement, complete preimage relation, specialized observations, invariant, cost and checks. Intersect candidate behavior with invariant constraints and compare exact outputs to independent finite oracles. Transformations add cross-input relations. A compiled mutation must be rejected by one selected assertion test. Registration and source identity do not prove model semantics. Parent library wiring is reviewed separately.

#### Retained observations and limits

Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. O(V+E log(E+1)+E alpha(V)) time and O(V+E) auxiliary space; reuses crate::disjoint_set::DisjointSet with union-by-size and path compression. Canonical tie keys include original input indices, so reordering equal endpoint/weight records can change those returned identities. No teacher call, timing gain or full mathematical verification is claimed. Prospective notes, logs and compiled mutants remain outside the repository.

## approximant

### Forward approximant

Validate every endpoint in input order, u before v. First failure returns InvalidEndpoint(edge,vertex,V). Ignore valid loops; sort original indices by (weight,min(u,v),max(u,v),index). Accept precisely edges connecting different current DSU blocks. Return accepted input indices in this order, their checked u128 weight sum, and component count including isolates. The result is a minimum-weight spanning forest of each original connected component.

#### Cost

O(V+E log(E+1)+E alpha(V)) time and O(V+E) auxiliary space; reuses crate::disjoint_set::DisjointSet with union-by-size and path compression.

Finite materialized inputs and successful allocation are assumptions. Source inspection and bounded Rust tests support correspondence; this is not a machine-checked proof.

## invariant

### Invariant

After each sorted edge, selected edges are acyclic, DSU components equal selected-edge connectivity and total_weight equals the selected u64 weights summed in u128. A rejected nonloop closes a cycle; a successful union reduces component count by one. Endpoint validation makes DSU union errors unreachable. On <=64-bit targets at most V-1 selected edges imply sum fits u128; checked addition remains explicit.

Global validation precedes the algorithm. Inputs are borrowed immutably; success is deterministic for fixed ordered inputs.

## catalog

# Typed obernant family

## behavior-complement

Domain: vertex count V and finite ordered undirected multigraph Edge records with usize endpoints and u64 weights, on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: In the explicit one-call universe U of vertex count V and finite ordered undirected multigraph Edge records with usize endpoints and u64 weights, on <=64-bit usize targets crossed with declared returned values, C=U minus R for the forward relation R. C includes incorrect partitions/forests/distances, wrong error details or wrong first-failure precedence. This is the exact specification complement and a partial source-derived approximant of Rust behavior. Allocation failures, panics, timing and traces are outside U.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## reverse-reconstruction

Domain: vertex count V and finite ordered undirected multigraph Edge records with usize endpoints and u64 weights, on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: For each returned y, Pre(y)={x in domain | R(x,y)} retains ALL compatible inputs. For success, preserve every graph satisfying the forward equations and observable canonical ordering; for errors, preserve every graph matching that exact first-failure condition. Parallel alternatives, internal representations and unobserved edge choices are retained whenever compatible. No unique input recovery or executable inverse is claimed.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## orientation

Domain: vertex count V and finite ordered undirected multigraph Edge records with usize endpoints and u64 weights, on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: Fix ordered edge records and weights; reversing either endpoint pair preserves validation for valid graphs, canonical sorting keys, accepted original indices, components and weight. Invalid inputs are outside this equivalence because u-before-v error precedence is observable. Executed endpoint-reversal fixture.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## positive-scaling

Domain: vertex count V and finite ordered undirected multigraph Edge records with usize endpoints and u64 weights, on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: Fix graph and edge order; multiply each weight by k>=1 with each product fitting u64. Comparison/tie keys preserve their ordering, so selected indices/components are unchanged and total scales by k. Executed k=11 fixture only; general k remains modeled.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## rejected-edge

Domain: vertex count V and finite ordered undirected multigraph Edge records with usize endpoints and u64 weights, on <=64-bit usize targets.
Observation scope: one complete returned value; traces, timing and allocator failures excluded.
Operator/target: Fix the sorted prefix and its current partition. The preimage of rejection comprises exactly loops or endpoint pairs already connected in that prefix; acceptance comprises all distinct-block endpoint pairs. This characterizes compatible prefix partitions rather than recovering unique prior edges. Independent returned-forest closure checks support finite correspondence.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## tests

### Checks and measured evidence

Independent oracle: Enumerate all subsets of at most eight edges. Independent Boolean closure computes original components C. Retain subsets of size V-C with exactly original connectivity; that rank and connectivity force acyclic spanning forests even for loops and parallel arcs. Minimize u128 weight independently of production sorting and DSU; also verify returned indices are unique, in bounds and span the same connectivity.

Transformation: Reverse all edge endpoints and multiply all weights by eleven in a fixed fixture whose largest weight is seven; selected input indices and component count stay identical, while total weight scales by eleven.

Negative control: Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Only that exact filtered test ran; the report asserts one failed test and exit101, excluding timeout. Source mutation exists only in external scratch.

Run cargo test -p mova-algorithms kruskal::tests -- --test-threads=1 after parent export integration. The external probe also compiled standalone modules; Kruskal used a small wrapper exposing the unchanged disjoint-set implementation. Isolated crate total:66 passing tests. Strict cargo clippy --all-targets -- -D warnings and formatting passed. Reports preserve fixed seeds/cases and domain bounds.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Prospective prediction was four tests per algorithm, bounded independent oracle agreement, deterministic repeated outcomes and finite compiled mutant rejection. Each module's four tests passed twice; normalized logs agree after omitting elapsed time. The isolated crate with temporary local exports passed66 tests; strict Clippy and rustfmt checks passed. Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). No production bug was silently patched; initial lint fixes used is_multiple_of in fixtures only.

### 2026-10-29 · tests

2026-10-29: four targeted tests pass twice with normalized-identical output. Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: mod.rs at sha256 f708e48322b16e547bfa5075e34861e1c8239a496b50e7065288fc448022b7ac.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs at sha256 f708e48322b16e547bfa5075e34861e1c8239a496b50e7065288fc448022b7ac.

### Undated · orientation

Type: undirected_representation_equivalence. Subject/revision: mod.rs at sha256 f708e48322b16e547bfa5075e34861e1c8239a496b50e7065288fc448022b7ac.

### Undated · positive-scaling

Type: weight_scaling. Subject/revision: mod.rs at sha256 f708e48322b16e547bfa5075e34861e1c8239a496b50e7065288fc448022b7ac.

### Undated · rejected-edge

Type: cycle_preimage. Subject/revision: mod.rs at sha256 f708e48322b16e547bfa5075e34861e1c8239a496b50e7065288fc448022b7ac.
