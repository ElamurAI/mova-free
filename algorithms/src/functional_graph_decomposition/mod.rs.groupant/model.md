# algorithm-functional-graph-decomposition

## descriptant

### algorithm-functional-graph-decomposition

Preserve original coordinates, deterministic ties and distinct validation/resource/infeasibility outcomes. Repeated observations constrain this epoch only.

## model

# algorithm-functional-graph-decomposition

## Domain

<=1000000 valid zero-based successors; cycles, per-vertex cycle ID, original entry coordinate, distance.

## Forward

Peel indegree-zero tails; enumerate residual cycles from least unseen original coordinate in successor order; reverse peeling propagates entry, ID and distance.

## Invariant

Every vertex belongs to exactly one eventual cycle; entry is first cycle vertex encountered and distance is the exact number of successor applications to entry.

## Cost

O(V) time and memory.

## nonbehavior

Type: behavior_complement.

Exclude wrong coordinate conventions, noncanonical cycle rotations/order, positive cycle distances, or entries not reached at the reported distance.

## preimages

Type: reverse_reconstruction.

Cycles reconstruct successor edges on cycle vertices exactly. Tail v with distance d>0 may point to any vertex with same entry and distance d-1, so tail predecessors are generally ambiguous. These choices reconstruct exactly the decomposition under canonical IDs.

## transition

Type: specialized_relation.

For noncycle v, distance[v]=distance[successor[v]]+1 and entry and ID are inherited.

## order

Type: specialized_relation.

Cycle vertices have entry[v]=v and distance zero; listed cycle orientation follows successor.

## coordinate

Type: specialized_relation.

Relabeling preserves distances and cycle membership but recanonicalizes cycle IDs/rotations.

## fiber

Type: specialized_relation.

Deleting a leaf tail preserves remaining entry coordinates and distances while coordinate renumbering must be explicit.

## errors

Type: error_partition.

Public size caps precede input-order validation, then arithmetic/search. Exact cover budget exhaustion is not infeasibility; closure capacity overflow is not infeasibility. Bounded index arithmetic cannot overflow. Checked i128 arborescence arithmetic remains defensive. Allocation failure is process-level, not returned; no callbacks or external writes. Empty cases and deterministic ties follow API docs.

## Limits

Partial syntax, no macro expansion/type resolution or full formal proof. No justified generic numerical surrogate for categorical witnesses/errors; supported 1/2/5/10% profiles remain absent, not fabricated. No timing claims. Allocation failure untested.

## evidence

#### Evidence

All successor functions up to six vertices (50070 cases including empty); independent per-start walk until repetition validates entry, distance and cycle edges.

Two debug and two release runs, compiled mutation detected by coordinates_and_errors, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: (see History). No timing measurement. Allocation failures and defensive i128 overflow unexercised.

## History

### 2026-10-30 · descriptant

Original std-only implementation or original independent fixtures. See source citations. Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/prospective.md.

### 2026-10-30 · evidence

#### Evidence

2026-10-30: All successor functions up to six vertices (50070 cases including empty); independent per-start walk until repetition validates entry, distance and cycle edges.

Two debug and two release runs, compiled mutation detected by coordinates_and_errors, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30. No timing measurement. Allocation failures and defensive i128 overflow unexercised.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
