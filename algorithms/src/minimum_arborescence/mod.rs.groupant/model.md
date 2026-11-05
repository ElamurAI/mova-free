# algorithm-minimum-arborescence

## descriptant

### algorithm-minimum-arborescence

Preserve original coordinates, deterministic ties and distinct validation/resource/infeasibility outcomes. Repeated observations constrain this epoch only.

## model

# algorithm-minimum-arborescence

## Domain

1..512 vertices, <=131072 edges, i64 weights, valid zero-based root; Option<i128> minimum outward tree objective.

## Forward

Choose minimum incoming arcs, sum costs, contract directed cycles and subtract destination incoming cost from crossing arcs; repeat until acyclic or no incoming arc.

## Invariant

Reduced optimum plus accumulated incoming costs equals original optimum; root has no incoming cost. Every feasible final tree reaches every vertex from root.

## Cost

O(VE+V^2) time, O(V+E) memory.

## nonbehavior

Type: behavior_complement.

Exclude suboptimal totals, None on feasible inputs, acceptance of invalid coordinates, and arithmetic wrapping.

## preimages

Type: reverse_reconstruction.

For Some(w), compatible graphs have a root-reaching branching of weight w and none cheaper. Internal edges/loops and tied trees are unrecoverable from the scalar. None reconstructs graphs with a nonroot component unreachable from root.

## transition

Type: specialized_relation.

Each cycle contraction maps entering choices by subtracting the chosen incoming cost.

## order

Type: specialized_relation.

Adding edges cannot increase a feasible optimum; it can repair infeasibility.

## coordinate

Type: specialized_relation.

Adding k to every nonloop edge shifts any feasible objective by (V-1)k when representable.

## fiber

Type: specialized_relation.

Permuting vertices and root preserves objective; edge ordering only chooses tied contraction paths.

## errors

Type: error_partition.

Public size caps precede input-order validation, then arithmetic/search. Exact cover budget exhaustion is not infeasibility; closure capacity overflow is not infeasibility. Bounded index arithmetic cannot overflow. Checked i128 arborescence arithmetic remains defensive. Allocation failure is process-level, not returned; no callbacks or external writes. Empty cases and deterministic ties follow API docs.

## Limits

Partial syntax, no macro expansion/type resolution or full formal proof. No justified generic numerical surrogate for categorical witnesses/errors; supported 1/2/5/10% profiles remain absent, not fabricated. No timing claims. Allocation failure untested.

## evidence

### Evidence

Repair epoch: five focused tests twice debug/release, identical normalized outcomes; compiled completed-traversal stop-condition mutant failed the visit-count assertion and exact restoration passed. Nested-contraction counted probe compares parent and repaired sources without timing. Refreshed paired syntax and source bindings; strict algorithm all-target Clippy. Repair logs use arbo-* in the same receipt directory.

Counted nested family (k=8,16,32,64,128): parent primary advances 324, 2312, 17424, 135200, 1065024; repaired 100, 392, 1552, 6176, 24640. Both return objective k in every case. These are operation counts, not elapsed time or throughput. The retained parent source and instrumented sources are external receipts.

## History

### 2026-10-30 · descriptant

Original std-only implementation or original independent fixtures. See source citations. Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/prospective.md.

Independent cost review falsified the original scan bound. The revised scan stops at every prior traversal; prospective repair and parent source retained in $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/arbo-repair-prospective.md and arbo-parent.rs.

### 2026-10-30 · document

#### Cost repair epoch 2026-10-30

Cycle scan now stamps each nonroot vertex exactly once per contraction. Reaching an earlier stamped traversal stops; only seen[v]==current_start closes a new cycle. Cycle labeling visits disjoint cycles once. Every selected parent edge is nonloop, hence every contracted cycle has at least two vertices; at most V-1 contractions. Per round O(V+E), total O(VE+V²), O(V+E) memory. Counter records primary parent advances; debug invariant visits<V. Deterministic cycle discovery order and objective/error semantics are unchanged.

The initial implementation revisited completed root-reaching chains per start. Independent review exhibited 1065024 primary steps for n257/e383 in the nested family, contradicting the originally declared asymptotic bound. Retain that falsified epoch; the revised source enforces a single stamp per round. Counts are operation observations, not runtime benchmarks.

### 2026-10-30 · evidence

2026-10-30: 4096 directed graph subsets on four vertices, four roots each, mixed signed costs; independent edge-subset reachability/indegree oracle.

Two debug and two release runs, compiled mutation detected by contractions_extremes_errors, restored positive rerun, strict all-target Clippy and twice-identical partial syntax. Logs: $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30. No timing measurement. Allocation failures and defensive i128 overflow unexercised.
