# algorithm-maximum-weight-closure

## descriptant

### algorithm-maximum-weight-closure

Preserve original coordinates, deterministic ties and distinct validation/resource/infeasibility outcomes. Repeated observations constrain this epoch only.

## model

# algorithm-maximum-weight-closure

## Domain

<=512 i64 weights, <=131072 zero-based implications; positive sum plus one fits u64; sorted selected coordinates and i128 weight.

## Forward

Build source-positive, negative-sink, implication arcs with capacity P+1; reuse Dinic; residual source side maximizes closed weight P-flow.

## Invariant

A finite minimum cut cannot sever implication capacity P+1 since empty closure has cut P; selected weight equals P-cut.

## Cost

Dinic O(V^2 E) time, O(V+E) memory.

## nonbehavior

Type: behavior_complement.

Exclude nonclosed selected sets, unsorted coordinates, wrong weight or suboptimal closure and wrapping capacity totals.

## preimages

Type: reverse_reconstruction.

Compatible instances make the selected set closed and globally optimal with the reported sum; exact preimages also require residual tie choice. Many weight distributions share a sum and zero-weight disconnected vertices may be absent.

## transition

Type: specialized_relation.

Cut capacity equals P minus selected weight for every implication-respecting source side.

## order

Type: specialized_relation.

Implication cycles require all-or-none selection within each strongly connected component.

## coordinate

Type: specialized_relation.

Increasing one vertex weight cannot lower optimum, though selected witness can change.

## fiber

Type: specialized_relation.

Duplicate implications and self implications preserve objective; no universal witness uniqueness is claimed.

## errors

Type: error_partition.

Public size caps precede input-order validation, then arithmetic/search. Exact cover budget exhaustion is not infeasibility; closure capacity overflow is not infeasibility. Bounded index arithmetic cannot overflow. Checked i128 arborescence arithmetic remains defensive. Allocation failure is process-level, not returned; no callbacks or external writes. Empty cases and deterministic ties follow API docs.

## Limits

Partial syntax, no macro expansion/type resolution or full formal proof. No justified generic numerical surrogate for categorical witnesses/errors; supported 1/2/5/10% profiles remain absent, not fabricated. No timing claims. Allocation failure untested.

## evidence

#### Evidence

4096 implication graphs on four vertices under three weight vectors (12288 cases); independent 16-subset closure oracle and witness checks.

Two debug and two release runs, compiled mutation detected by extremes_and_errors, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: (see History). No timing measurement. Allocation failures and defensive i128 overflow unexercised.

## History

### 2026-10-30 · descriptant

Original std-only implementation or original independent fixtures. See source citations. Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/prospective.md.

### 2026-10-30 · evidence

#### Evidence

2026-10-30: 4096 implication graphs on four vertices under three weight vectors (12288 cases); independent 16-subset closure oracle and witness checks.

Two debug and two release runs, compiled mutation detected by extremes_and_errors, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30. No timing measurement. Allocation failures and defensive i128 overflow unexercised.
