# algorithm-blossom-matching

## descriptant

### algorithm-blossom-matching

Preserve original coordinates, deterministic ties and distinct validation/resource/infeasibility outcomes. Repeated observations constrain this epoch only.

## model

# algorithm-blossom-matching

## Domain

Bounded undirected zero-based graph; symmetric optional mate array; loops ignored and duplicates collapsed.

## Forward

Alternating BFS contracts odd cycles to their common base; augmenting paths flip matched status and increase cardinality by one.

## Invariant

Mate symmetry, original-edge membership and disjointness survive augmentation; contraction preserves existence of augmenting paths.

## Cost

O(V^3+E log E) time, O(V+E) memory.

## nonbehavior

Type: behavior_complement.

Exclude nonsymmetric, self, nonexistent-edge or nonmaximum matchings; exclude endpoint acceptance and wrong first error.

## preimages

Type: reverse_reconstruction.

Compatible graphs contain returned pairs and admit no larger matching; exact preimages additionally reproduce sorted BFS choices. Adding loops or duplicate edges leaves output unchanged, so graph recovery is nonunique.

## transition

Type: specialized_relation.

Blossom lifting preserves augmenting-path existence through contracted odd alternating cycles.

## order

Type: specialized_relation.

Vertex relabeling preserves cardinality, but tie-selected mates need not commute.

## coordinate

Type: specialized_relation.

Adding graph edges cannot decrease optimal cardinality; matching identity may change.

## fiber

Type: specialized_relation.

Loops and duplicate adjacency occurrences disappear before search, defining exact output fibers.

## errors

Type: error_partition.

Public size caps precede input-order validation, then arithmetic/search. Exact cover budget exhaustion is not infeasibility; closure capacity overflow is not infeasibility. Bounded index arithmetic cannot overflow. Checked i128 arborescence arithmetic remains defensive. Allocation failure is process-level, not returned; no callbacks or external writes. Empty cases and deterministic ties follow API docs.

## Limits

Partial syntax, no macro expansion/type resolution or full formal proof. No justified generic numerical surrogate for categorical witnesses/errors; supported 1/2/5/10% profiles remain absent, not fabricated. No timing claims. Allocation failure untested.

## evidence

#### Evidence

33,868 simple graphs over zero through six vertices; independent subset recursion chooses unmatched or paired first vertex; validates symmetry and original edges.

Two debug and two release runs, compiled mutation detected by boundaries_and_flower, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: (see History). No timing measurement. Allocation failures and defensive i128 overflow unexercised.

## History

### 2026-10-30 · descriptant

Original std-only implementation or original independent fixtures. See source citations. Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/prospective.md.

### 2026-10-30 · evidence

#### Evidence

2026-10-30: 33,868 simple graphs over zero through six vertices; independent subset recursion chooses unmatched or paired first vertex; validates symmetry and original edges.

Two debug and two release runs, compiled mutation detected by boundaries_and_flower, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30. No timing measurement. Allocation failures and defensive i128 overflow unexercised.
