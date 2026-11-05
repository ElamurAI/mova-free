# graph-structure

## model

# graph-structure

## forward

Compute capped minimum-cost maximum flow, deterministic maximum-cardinality selection, maximal induced k-core, start-relative dominator sets, and unique DAG transitive reduction. Graph observations remain tagged by directedness, vertex labels, edge multiplicity and objective.

## invariant

Flow respects capacity/conservation and exact cost; MCS chooses maximum selected-neighbor count with lowest-ID tie; k-core is maximal with live degree>=k; dominators characterize every start-to-v path, empty for unreachable v; reduction preserves reachability and keeps only essential DAG edges.

## cost

Flow V<=64,E<=512,limit<=1024: O((limit+1)V(E+V)) time/O(V+E) memory. Other V<=256,E<=65536: MCS and core O(V²+E) including matrix validation; dominators O(V²(V+E)) conservative fixed-point time/O(V²+E) memory; reduction O(V³+E)/O(V²). Matrix allocation dominates sparse validation. No timing claims.

## composition

The five implementations are std-only and have no production calls between them. Test joins can compare limit1/unit-cost admitted DAG flow with unweighted shortest-path distance; MCS/core share a simple-undirected graph after explicit validation. DAG reduction may feed reachability consumers but must not replace the graph for dominator queries. Directed-to-undirected conversion is an explicit semantic transformation, not shared representation.

## epoch-memory

Wave25. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

Named joins: reduction_reachability_rejects_dominator_transfer, unit_flow_bfs_join, clique_order_core_coordinates_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## limitations

Source/sink and all vertex IDs must retain coordinates across joins. Allocation failure is outside typed errors. Exact integer/categorical outputs admit no generic percentage relaxation; deterministic witness need not be a canonical optimum. Flow negative-cycle refusal is stronger than source-reachable refusal.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude flow witnesses violating capacity, conservation, exact sum of cost*flow or capped lexicographic optimum; MCS orders violating prefix priority/ties; cores omitting feasible induced subsets; dominator membership contradicted by an avoiding path; or reductions with changed closure/redundant retained edges. Exclude invalid directedness joins, wrapping checked arithmetic and treating rejected negative cycles as zero-cost empty output.

## reverse-reconstruction

Type: reverse_reconstruction.

Empty and complete simple graphs both yield ascending MCS order. At k=1 a three-vertex path and triangle both yield all vertices. Adding loops to a directed chain preserves its dominator sets; adding any subset of reachable pairs to a reduced DAG preserves its reduction. Flow value/cost/witness admits changes to unused capacities/costs that preserve optimality. Thus neither the aggregate result nor these individual projections reconstructs a unique original graph.

## flow-path-join

Type: conditional_shortest_path_relation.

For a directed acyclic graph with unit capacities and unit costs and distinct reachable terminals, requested flow1 has value1 and cost equal to the shortest source-sink edge count. If sink is unreachable, value0 and cost0 require a separate reachability tag. This relation does not extend to arbitrary weighted flow, negative cycles, or larger flow limits.

## flow-potential

Type: potential_reweighting.

For vertex potential h with transformed costs c'(u,v)=c(u,v)+h(u)-h(v) fitting i64, each feasible s-t flow changes total cost by value*(h(s)-h(t)); cycles keep their cost. Capped optimal value and negative-cycle admission are invariant. Witness tie order and checked finite arithmetic still belong to native semantics.

## dominance-reduction-rejection

Type: nonpreserving_graph_transform.

On edges 0->1,0->2,1->2 with start0, vertex1 does not dominate2. Reduction removes0->2, after which1 does dominate2. Closure equality therefore does not justify reduction before dominators, flow, edge-disjoint path or shortest-edge-count queries.

## core-order-join

Type: shared_domain_distinct_observations.

For K_n, MCS returns ascending IDs and k-core returns all IDs iff k<n, else empty. General MCS order is neither a core-number sequence nor a chordality proof. Removing labels or passing a directed adjacency to either API invalidates the shared-domain premise.

## dominator-deletion

Type: vertex_cut_characterization.

For reachable v, d is in Dom(v) iff deleting d destroys start-to-v reachability, counting removal of start as reaching nothing. Start and v belong to each reachable Dom(v). Unreachable rows remain empty; adding edges may remove dominators of already reachable vertices but also create newly reachable vertices.

## reduction-essentiality

Type: closure_fiber.

In a DAG, retain edge(u,v) iff deleting it destroys u-to-v reachability. Result is idempotent and unique on the normalized edge relation. Its preimage consists of the same reduced graph plus optional transitive edges and duplicate representations; cyclic graphs are excluded rather than arbitrarily reduced.

## core-maximality

Type: subset_union_certificate.

core_k is the union of all induced vertex subsets of minimum degree>=k; union preserves feasibility. Consequently core_l subset core_k for k<=l, adding a simple undirected edge cannot shrink a core, and peeling the returned induced graph again deletes nothing.

## wave25-evidence

# graph-structure: wave25 evidence

## checks (wave25-evidence)

Named joins: reduction_reachability_rejects_dominator_transfer, unit_flow_bfs_join, clique_order_core_coordinates_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## History

### 2026-10-30 · epoch-memory

Wave25, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · checks

2026-10-30: this family adds 11 source-local tests and 3 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.

### 2026-10-30 · checks (wave25-evidence)

2026-10-30: this family adds 11 source-local tests and 3 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.
