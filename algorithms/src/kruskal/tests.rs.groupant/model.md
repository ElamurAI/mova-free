# algorithm-kruskal-tests

## descriptant

### kruskal test groupant

Four original tests with independent bounded reference logic and explicit transformed/boundary cases. Kruskal composes with the existing disjoint-set module; its partition/union behavior is a dependency, not a duplicate implementation. Source remains adjacent; manifest hash binds this revision and syntax.json is a partial syn projection without macro expansion or semantic resolution.

#### Mathematical combiner

Inputs are source/AST, forward relation, complement, complete preimage relation, specialized observations, invariant, cost and checks. Intersect candidate behavior with invariant constraints and compare exact outputs to independent finite oracles. Transformations add cross-input relations. A compiled mutation must be rejected by one selected assertion test. Registration and source identity do not prove model semantics. Parent library wiring is reviewed separately.

#### Retained observations and limits

Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Subset oracle O(2^E*(V^3+E)) with E<=8,V<=6; 761 exhaustive graphs have E<=6,V<=4, plus40 seeded multigraphs. Memory O(V^2+E). Canonical tie keys include original input indices, so reordering equal endpoint/weight records can change those returned identities. No teacher call, timing gain or full mathematical verification is claimed. Prospective notes, logs and compiled mutants remain outside the repository.

## approximant

### Forward approximant

The fixed four-test schedule maps candidate returns to exact assertion verdicts. Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Enumerate all subsets of at most eight edges. Independent Boolean closure computes original components C. Retain subsets of size V-C with exactly original connectivity; that rank and connectivity force acyclic spanning forests even for loops and parallel arcs. Minimize u128 weight independently of production sorting and DSU; also verify returned indices are unique, in bounds and span the same connectivity.

#### Cost

Subset oracle O(2^E*(V^3+E)) with E<=8,V<=6; 761 exhaustive graphs have E<=6,V<=4, plus40 seeded multigraphs. Memory O(V^2+E).

Finite materialized inputs and successful allocation are assumptions. Source inspection and bounded Rust tests support correspondence; this is not a machine-checked proof.

## invariant

### Invariant

Fixed seeds use explicit wrapping u64 arithmetic; generated valid endpoints stay in range. Expected results are computed without invoking the algorithm under test. Oracle bounds keep exhaustive or recursive enumeration finite. Candidate inputs remain unchanged where asserted.

Reverse all edge endpoints and multiply all weights by eleven in a fixed fixture whose largest weight is seven; selected input indices and component count stay identical, while total weight scales by eleven.

## catalog

# Typed obernant family

## behavior-complement

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Declare U as all deterministic terminating candidate return tables on this fixed fixture schedule crossed with four pass/fail verdicts. R is assertion evaluation. U minus R excludes passing when any executed assertion disagrees with its independent expected value, and excludes failure when all assertions pass. Exact assertion specification; actual Rust correspondence is partial. Compilation failure, timeout and resource failure are outside U.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## reverse-reconstruction

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Pre(y) retains ALL candidate return tables whose assertions produce observed verdicts y. Pass constrains only tested calls, leaving untested inputs arbitrary. Failure preserves every compatible first failing assertion and earlier passing returns. No unique source implementation follows from these observations.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## oracle-independence

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Enumerate all subsets of at most eight edges. Independent Boolean closure computes original components C. Retain subsets of size V-C with exactly original connectivity; that rank and connectivity force acyclic spanning forests even for loops and parallel arcs. Minimize u128 weight independently of production sorting and DSU; also verify returned indices are unique, in bounds and span the same connectivity. Hold fixture inputs fixed; require exact equality with this independently specified reference and explicit boundary assertions. No unrestricted oracle proof is claimed.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## transformations

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Reverse all edge endpoints and multiply all weights by eleven in a fixed fixture whose largest weight is seven; selected input indices and component count stay identical, while total weight scales by eleven. Keep all compatible original/transformed graph pairs under these constraints; do not infer a unique original graph from results.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Four tests pass twice with identical normalized logs; Sort by Reverse(edge.weight) instead of weight. Compiled mutant is rejected by exactly all_small_weighted_graphs_match_subset_oracle (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## coverage-budget

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Four tests: 761 simple undirected graphs on 0..4 vertices, each possible arc absent/weight0/weight3; 40 seeded 8-edge multigraphs on 2..6 vertices (seed 0x6b7520261029); exact deterministic-tie/loop/parallel/invalid/empty/MAX fixtures; endpoint reversal and factor11 scaling. Subset oracle O(2^E*(V^3+E)) with E<=8,V<=6; 761 exhaustive graphs have E<=6,V<=4, plus40 seeded multigraphs. Memory O(V^2+E). These counts and bounds define the measured finite domain; compatible runs share the schedule, not a promised wall-clock duration.
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

Type: behavior_complement. Subject/revision: tests.rs at sha256 a109915e9d634425226e6828aff20525909d44a322b05d6f41a764ef2a7e2913.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs at sha256 a109915e9d634425226e6828aff20525909d44a322b05d6f41a764ef2a7e2913.

### Undated · oracle-independence

Type: independent_reference_relation. Subject/revision: tests.rs at sha256 a109915e9d634425226e6828aff20525909d44a322b05d6f41a764ef2a7e2913.

### Undated · transformations

Type: metamorphic_relation. Subject/revision: tests.rs at sha256 a109915e9d634425226e6828aff20525909d44a322b05d6f41a764ef2a7e2913.

### Undated · coverage-budget

Type: bounded_resource_preimage. Subject/revision: tests.rs at sha256 a109915e9d634425226e6828aff20525909d44a322b05d6f41a764ef2a7e2913.
