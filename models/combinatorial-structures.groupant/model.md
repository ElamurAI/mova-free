# combinatorial-structures

## model

# combinatorial-structures

## forward

Tagged exact combinatorial relations: general-graph maximum-cardinality matching, exact cover, functional-graph decomposition, maximum-weight closure and rooted minimum arborescence. Each retains its own feasible witness and objective. Matching maximizes |M| over disjoint edges; exact cover enforces sum_selected_rows 1[column in row]=1. Closure maximizes sum_{v in C}w_v under implications. Arborescence returns min_T sum_e w_e. Functional output satisfies f^distance(v)(v)=entry(v) on canonical cycle cycle_id(v).

## invariant

Witness indexes refer to the original graph/row universe. Matching disjointness, exact column coverage, functional successor components, closure implications and rooted directed reachability are separate invariants. Infeasible, invalid and successful-empty outcomes remain distinct.

## cost

Functional decomposition O(n) time/space. Exact cover may be exponential; matching and closure use concrete leaf bounds. Arborescence scans each nonroot parent once per contraction and labels disjoint cycles: O(V+E) per round, at most V rounds, O(VE+V²) total. The nested-contraction witness at V=257 reduced primary parent advances from 1065024 to 24640 with the same optimum; this is an operation count, not timing. Adapter construction and allocation add cost.

## composition

Closure uses exact reduction capacities and implication edges. Arborescence exposes Result<Option<i128>,Error> objective only, no edge witness. For heads define successor[0]=0 and successor[i]=head[i] after validation: token i reaches root iff cycle_id[i]=cycle_id[0]. Caller empty/root-label/first-error policy remains separate. Matching-to-cover needs explicit columns and objective handling.

## limitations

Successful allocations and each leaf's finite domain/error precedence. These are tagged relations, not automatic production adoption. Floating residuals are not solution-error guarantees. No generic 1/2/5/10% composition-wide profile; finite evidence remains separate from analytical claims.

## epoch-memory

Wave26. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.

## checks

["cargo test -p mova-algorithms blossom_matching::tests","cargo test -p mova-algorithms exact_cover::tests","cargo test -p mova-algorithms functional_graph_decomposition::tests","cargo test -p mova-algorithms maximum_weight_closure::tests","cargo test -p mova-algorithms minimum_arborescence::tests"]

## wave26-executed-checks

# combinatorial-structures: wave26 checks

13 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## catalog

# Typed family relations

## behavior-complement

Type: behavior_complement.

Exclude overlapping matched edges, nonexact coverage, closure implication violations, wrong cycle/entry/distance, or nonminimum arborescence cost. None means infeasible, not cost zero; a returned arborescence edge witness is outside the API.

## reverse-reconstruction

Type: reverse_reconstruction.

Arborescence cost c admits all rooted graphs with optimum c; selected edges are unrecoverable. Matching/cover/closure witnesses admit different unused edges/rows/weights. Cycle/entry/distance summaries admit distinct successor forests between adjacent distance layers.

## objective_projection

Type: objective_projection.

Arborescence returns cost only; edge-witness lifting is not exposed. Matching, cover and closure retain their separate original-index witnesses.

## feasibility_objective

Type: feasibility_objective.

Maximum cardinality, exact coverage, closure weight and arborescence cost are different objectives with different feasibility domains.

## reduction_capacity

Type: reduction_capacity.

Closure reductions require checked capacity arithmetic and exact implication encoding; finite capacities cannot silently replace an intended hard implication.

## component_ambiguity

Type: component_ambiguity.

After validating heads and setting successor[0]=0, cycle_id[i]=cycle_id[0] iff i reaches root. This adapter preserves caller-specific empty, root-label and error precedence rules.

## wave26-evidence

### combinatorial-structures: wave26 checks

13 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## History

### 2026-10-30 · epoch-memory

Wave26, 2026-10-30. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.
