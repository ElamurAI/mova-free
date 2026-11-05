# Root identity and prior-parent interval safety

Subject: current manifest source SHA and exact UTF-8 range [18297,19049). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

At index zero require explicit null parent, file/File labels and whole-source interval. Otherwise convert parent to usize, require parent<index, index previously accepted ranges and require containment before appending the current interval.

## Invariant

Each lookup ranges[parent] is bounded by strict predecessor admission. A child may share its parent's range; adjacency and non-overlap of siblings are not required.

## Cost

O(1) checks and one interval append per node, O(N) retained ranges. There is no recursive ancestry walk; predecessor order plus containment supplies local structural checks.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Exclude root with missing/non-null parent or partial source interval; exclude nonroot self/future parent and children escaping their chosen parent. Exclusion concerns structural projections, not source semantic truth.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## reverse_reconstruction

Given accepted child interval [a,b) at i, compatible parent IDs are all p<i whose stored [s,e) satisfies s<=a<=b<=e. Overlapping earlier nodes can make this antecedent nonunique.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## predecessor_index_safety

Before iteration i, ranges.len=i. Strict p<i makes lookup safe. Weakening to p<=i admits self-parent and reaches a bounds panic before a clean diagnostic.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## root_presence_distinction

An explicit parent:null is required at root; an omitted field fails even though Value indexing would also look null. Root labels and full extent are independent conjuncts.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## containment_transitivity

On an accepted predecessor chain, repeated local containment implies every ancestor interval contains the child. This structural implication neither proves nearest-parent selection nor unique ancestry.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## sibling_overlap_freedom

Two nodes may name one predecessor and overlap or have equal intervals. The checker does not impose partition coverage, adjacency or disjointness constraints.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.
