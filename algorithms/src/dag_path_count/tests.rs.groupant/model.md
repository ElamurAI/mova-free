# algorithm-dag-path-count-tests

## model

# algorithm-dag-path-count-tests

## Domain
Finite fixtures in this test module, a trusted Rust compiler and standard library, and successful allocation; oracle recursion is restricted to four-vertex DAGs.

## Forward
Enumerate actual recursive arc paths independently of topological DP for every ternary multiplicity assignment on six four-vertex forward pairs (729 DAGs), every source and reversed vertex labels. Test exact u128 maximum, deterministic overflow, cycle precedence, unreachable exponential component and 30000-vertex chain.

## Cost
Exponential small-input oracle cost is intentional and bounded by enumerated fixtures; long-chain checks use linear output storage. No performance conclusion follows from these tests.

## Invariant
Each enumerated fixture is checked against a separately expressed expected result; all test assertions must execute.

## behavior-complement
Type: behavior_complement. For the declared finite harness relation H, its complement consists of false passes, false failures, skipped intended assertions, wrong enumerated corpus or disagreement with the explicit expected results. This specification is not inferred merely from seeing a red control.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## reverse-reconstruction
Type: reverse_reconstruction. Given a passing harness result, retain every implementation agreeing on the executed finite inputs; behavior outside that corpus remains unconstrained. Given an assertion failure, retain every candidate and fixture yielding that discrepancy. Neither outcome uniquely reconstructs production code.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## oracle
Type: independent_path_enumeration. Recursive enumeration follows each distinct arc and increments every reached prefix endpoint; it uses neither indegrees nor DP.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## boundary
Type: integer_boundary_witness. Powers 2^0 through 2^127 sum to u128::MAX. An additional 2^127 contribution must overflow at (127,128).
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## global
Type: validation_scope_witness. A cycle outside source reachability and a cycle beside an overflowing component both reject before counting.
Domain and observation scope are those above; assumptions are fixed inputs, successful allocation and trusted compiler/library. Evidence: model.md#evidence. Status: partial_source_derived_model.

## Composition
Intersect the source projection, forward relation, ordered typed obernants, invariants and finite evidence. Tests constrain observations but do not prove all wiring. Parent library exports and registry integration are checked separately.

## Epoch memory
Prospective predictions were saved before implementation in ../mod.rs.groupant/model.md#prospective. Original code and fixtures; conceptual origin: Tarjan (1972) lowlink blocks and Kahn (1962) topological ordering with standard DAG recurrence. Existing bridges adjacency and topological_sort were inspected and reused. Production checks matched the first implementation; no semantic repair was needed.

## Limits
Allocation failure is excluded. Syntax projection has no macro expansion or semantic resolution. Finite oracles are not unrestricted proofs. An executable approximate surrogate is an explicit gap: no useful certified relaxed metric was established for edge partitions or exact checked counts; the exact oracle is not a speed surrogate.

## evidence

### Evidence

All 456 algorithm library tests passed twice in debug and twice in release; normalized sorted test records were identical. Each new module contributes four tests. Production mutations: replace the lowlink block boundary with true, rejected by articulation_parallel_loops; replace source count one with zero, rejected by exhaustive_path_enumeration. Both compiled and failed named equality assertions; original source bytes were restored and all four tests per module passed again. Strict production Clippy passed. These are finite observations, not speed or unrestricted correctness claims.

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

Source SHA-256: d32093e393ceb0c04668678f34b09e4866755d1fb473979ad315abd558a3a615. Prospective record: ../mod.rs.groupant/model.md#prospective. Syntax projection generated independently twice and compared byte-for-byte.

### Undated · Domain

Source: algorithms/src/dag_path_count/tests.rs; SHA-256: d32093e393ceb0c04668678f34b09e4866755d1fb473979ad315abd558a3a615. Observation is one completed invocation. Models are exact specifications with partial source correspondence, not semantic AST proofs.
