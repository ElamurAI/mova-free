# algorithm-dag-path-count

## model

# algorithm-dag-path-count

## Domain
Finite materialized ordered graphs and machine-sized indices, fixed input, successful allocation. Observations are returned Result values; allocator failure and wall-clock timing are outside the relation.

## Forward
Reject an invalid source before inspecting edges. Run topological_sort on the entire graph: first invalid adjacency target precedes any cycle error. In a globally valid DAG set c[source]=1, then add c[u] once along each arc u->v with checked u128 addition. Count includes the empty path; unreachable vertices get zero. Overflow is the first failing (from,to) in smallest-available topological order and input adjacency order.

## Cost
O(E+V log(V+1)) time and O(V) auxiliary space from the reused heap-based topological_sort, plus O(V+E) checked DP. Output has V u128 values. Counts cannot wrap: checked_add returns an error at the first overflow. Unreachable components can have exponentially many internal paths without overflow because all propagated source counts there are zero.

## Invariant
After processing a topological prefix, each count equals the sum of contributions from already processed incoming arc identities plus the source empty-path indicator. Before processing a vertex all its predecessors have contributed. Global acyclicity is already established before any count or overflow result is emitted.

## behavior-complement
Type: behavior_complement. For domain D and Result space Y let R be the stated exact forward relation. The behavior complement is (D x Y) minus R, including wrong memberships/counts, malformed canonical outputs, wrong errors and wrong precedence. This is a specification complement; correspondence to source is partial and is not a proof.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## reverse-reconstruction
Type: reverse_reconstruction. For successful c and source s retain every DAG whose arc multiplicities satisfy c[v]=[v=s]+sum incoming c[u], within u128. Counts do not reconstruct edges; zero-count components are especially ambiguous. Error fibers retain inputs with the stated validation failure or first ordered overflowing addition.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## parallel
Type: arc_multiplicity. Duplicating an arc duplicates its predecessor contribution, not merely reachability; downstream increments equal the number of continuations.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## unreachable
Type: zero_extension. A disconnected acyclic component has zero source counts even when its own path count exceeds u128; a cycle there still rejects the whole input.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## overflow
Type: checked_arithmetic_boundary. A reachable exact count of u128::MAX is accepted; the next positive contribution fails. Error endpoint is determined by traversal order, not a shortest witness.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## renaming
Type: vertex_transport. A vertex bijection permutes successful counts and source. Overflow witnesses can change because numeric tie ordering changes.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## validation
Type: error_precedence. Invalid source precedes invalid edge, which precedes global cycle, which precedes count overflow. A disconnected cycle is never hidden by reachability.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## Composition
Intersect the source projection, forward relation, ordered typed obernants, invariants and finite evidence. Tests constrain observations but do not prove all wiring. Parent library exports and registry integration are checked separately.

## Epoch memory
Prospective predictions were saved before implementation in model.md#prospective. Original code and fixtures; conceptual origin: Tarjan (1972) lowlink blocks and Kahn (1962) topological ordering with standard DAG recurrence. Existing bridges adjacency and topological_sort were inspected and reused. Production checks matched the first implementation; no semantic repair was needed.

## Limits
Allocation failure is excluded. Syntax projection has no macro expansion or semantic resolution. Finite oracles are not unrestricted proofs. An executable approximate surrogate is an explicit gap: no useful certified relaxed metric was established for edge partitions or exact checked counts; the exact oracle is not a speed surrogate.

## evidence

### Evidence

All 456 algorithm library tests passed twice in debug and twice in release; normalized sorted test records were identical. Each new module contributes four tests. Production mutations: replace the lowlink block boundary with true, rejected by articulation_parallel_loops; replace source count one with zero, rejected by exhaustive_path_enumeration. Both compiled and failed named equality assertions; original source bytes were restored and all four tests per module passed again. Strict production Clippy passed. These are finite observations, not speed or unrestricted correctness claims.

## prospective

### Prospective model
Validate source, then global graph via topological_sort, then checked u128 DAG recurrence. Count empty path once; parallel arcs distinct. O(E+V log(V+1)) time, O(V) auxiliary storage. Overflow at first deterministic failed addition. Complement includes wrapping, remote-cycle acceptance and incorrect validation precedence. Reverse fiber retains every DAG satisfying the count recurrence, not unique adjacency. Allocation failure excluded; no useful certified approximation metric established.

## History

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

External evidence under $MOVA_DATA/runs/octopus-wave18-graphs-2026-10-30:

- debug1.log: SHA-256 52e7cb6f6804a8f61117fa43e11a2d1b38c676bd6ac2c349c0792c962e7ca8a4

- debug2.log: SHA-256 4cadee5560ff9f7b3b08e46b8bb567484fb81467c987b63af10bc3e8f295f611

- release1.log: SHA-256 b4c009e36790c35faed12b982dbfe6ca9497392852475f7d413621ae2cb7ed62

- release2.log: SHA-256 b06e3f678be609b17a452d67f20a6a5c06f863297690f75c1b4987aeb92bd0c5

- biconnected_components-negative.log: SHA-256 981e8729d09a2cd71fdf0e04704473e09fac3612e949cf2fd8011930b103449b

- dag_path_count-negative.log: SHA-256 c1e9876c08d4d9576d9c0f9589629e55d06dce40bd12687247783812d3efa819

- biconnected_components-restored.log: SHA-256 23bf086daee1a998579ff3e5f6bcd888be1920b0ca2f91d541b239065db9751e

- dag_path_count-restored.log: SHA-256 7f5bb86f3fc328658d1ebfe46f01a4ea0a8b2d2c98b4280d063057dafdb883a8

- verification.log: SHA-256 5bb68adc6867078baf71c9847f8e324385ff00a3eedb030cb3f8779b2e4aa555

- clippy.log: SHA-256 279a2a959ef9d6d5796b3d61c052347cef8707316fbe2730f51e2120605785b0

Source SHA-256: b373088f752be5d16a5de4fc5189b923fe7c65f240ac462598221fd38919856e. Prospective record: model.md#prospective. Syntax projection generated independently twice and compared byte-for-byte.

### 2026-10-30 · prospective

Heading `### Prospective model, 2026-10-30` stays in prospective as `### Prospective model`.

### Undated · Domain

Source: algorithms/src/dag_path_count/mod.rs; SHA-256: b373088f752be5d16a5de4fc5189b923fe7c65f240ac462598221fd38919856e. Observation is one completed invocation. Models are exact specifications with partial source correspondence, not semantic AST proofs.
