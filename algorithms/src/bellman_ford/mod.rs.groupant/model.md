# algorithm-bellman-ford

## descriptant

### Modeling

Prospective design is retained in model.md#prospective. Existing dijkstra and floyd_warshall interfaces informed adjacency and validation choices; no imported solutions. Adding duplicate identical edges or permuting adjacency leaves successful distances and reachable-cycle status unchanged; invalid-edge precedence may change under permutation. Zero-weight cycles remain valid. Unreachable negative cycles cannot affect the source result.

The mathematical combiner reads source, source-bound partial syntax, forward and typed obernants, invariants and evidence together. The complement is first and the converse is second; neither is interchangeable with negative tests. Future epochs must retain failed controls and adjust claims to measurements. Models do not prove source correctness or downstream wiring.

Implementation and tests are separate leaves; registry import and parent wiring belong to integration. Syntax omits semantic analysis and expanded macros. Exact algorithms have no approximate replacement profile.

## model

# bellman_ford mod model

## Forward

Reject an invalid source first, then the first invalid endpoint in row/edge order. On valid graphs return None exactly at unreachable vertices and Some of the minimum walk weight elsewhere, unless a source-reachable negative cycle exists, which returns ReachableNegativeCycle. All signed i64 edge weights, duplicates, loops and disconnected components are supported. i128 checked addition returns DistanceOverflow if representation fails; on current <=64-bit usize targets at most V edges per synchronous round fit i128.

## Invariant

After k completed synchronous rounds d[v] is the minimum weight among source-to-v walks of at most k edges (None if none exist). Each next row includes all previous distances. A strict improvement in round V entails a reachable negative cycle; stabilization entails all edge inequalities. Source distance is zero on successful output.

## Cost

Validation O(V+E), then at most V rounds each cloning V entries and inspecting V+E data: O(V(V+E)) time, O(V) auxiliary space. Costs include an empty-edge graph. No timing claim.

## nonbehavior

Type: behavior_complement. Let X be allocated finite adjacency vectors with i64 weights and usize sources, and Y the typed Result<Vec<Option<i128>>,BellmanFordError> observation space, excluding allocation failure and nontermination. Let R subset X times Y be the graph of the Forward relation including source-before-edge validation and first checked-overflow precedence. Define U=X times Y and behavior_complement=U minus R. Thus every input has its unique permitted observation in R; all other typed observations are in the complement. The shortest-walk characterization assumes representable synchronous sums (guaranteed on current <=64-bit usize targets). Finite negative fixtures check selected memberships, not this whole universe.

## compatible_inputs

Type: reverse_reconstruction. Given successful vector d, compatible antecedents are all validated graphs and valid sources whose reachable minimum walk weights equal d and have no reachable negative cycle. Many graphs fit the same d: unused edges may change while preserving minima. InvalidSource recovers source>=V; InvalidEdge recovers a valid source and first offending edge; ReachableNegativeCycle recovers existence and source reachability, not its unique vertices. DistanceOverflow describes first failed checked sum.

## graph_laws

Type: metamorphic_relation. Adding duplicate identical edges or permuting adjacency leaves successful distances and reachable-cycle status unchanged; invalid-edge precedence may change under permutation. Zero-weight cycles remain valid. Unreachable negative cycles cannot affect the source result.

## potential_transport

Type: potential_reweighting. For validated G, valid source s and integer potential h, define w_h(u,v)=w(u,v)+h(u)-h(v). Require every transformed edge to fit i64 and all evaluated sums to fit i128. Then reachability is unchanged and every walk telescopes, so successful d_h(v)=d(v)+h(s)-h(v), with None preserved; reachable negative-cycle status is invariant because cycle potential sums vanish. This is an exact conditional relation, not arbitrary floating-point reweighting. The 64-topology, three-potential, three-source test gives 576 successful paired observations plus a negative-cycle pair.

## tight_distance_certificate

Type: attainment_certificate. Fix a valid graph, source s and candidate d of length V with d[s]=0. Let F={v:d[v] is Some}. Require no edge from F to its complement, all finite-edge inequalities d[v]<=d[u]+w(u,v), and source reachability of every v in F using tight edges d[v]=d[u]+w(u,v). These conditions are equivalent to successful shortest distances on the no-overflow domain: inequalities give a lower bound on all walks and tight paths attain it. None vertices cannot be reached. Tight cycles have zero weight. Inequalities alone are insufficient. The certificate fixture enumerates 84 candidates and accepts exactly one.

## distance_graph_fiber

Type: constructive_preimage. Fix d=[0,a,b,None], source 0 and i64 a,b with representable b-a+t. Start with edges 0->1 weight a and 0->2 weight b; add 1->2 weight b-a+t for t>=0 and any subgraph on unreachable vertex 3 without edges from reachable vertices into it. All these different graphs reproduce d; t<0 leaves this fiber. This is a sound constructive subset of the full preimage, not unique recovery or a complete graph enumerator. Tests cover 25 target vectors, 75 compatible graphs and 25 excluded graphs.

## reachable_cycle_obstruction

Type: cycle_obstruction. For valid endpoints and arithmetic, reachable negative cycles are exactly the obstruction to a finite shortest-walk vector. A cycle disconnected from s cannot obstruct it. Under potential transport each cycle weight and its source reachability are unchanged. A negative self-loop at an unreachable vertex can coexist with a valid distance certificate; a reachable negative cycle cannot satisfy its summed inequalities. Evidence includes the negative-cycle transport pair and 75 disconnected-loop constructions.

## slack_edge_extension

Type: monotone_extension. Hold successful d and source fixed. An additional edge u->v preserves d if u is unreachable, or both endpoints are reachable and w>=d[v]-d[u], provided w fits i64. Existing tight witnesses survive and no shorter path is introduced. Adding an edge from a reachable u to unreachable v changes reachability; a negative slack edge between reachable vertices changes d or creates a reachable negative cycle. The constructive-fiber test checks slack 0,1,2 and excludes -1; it does not cover all valid i128 distances.

## evidence

### Evidence

Three tests per algorithm. Signed exhaustive oracle covers n=1..3 with each ordered pair absent, -1 or +1: 19,767 graphs and 59,214 source queries. Independent Floyd intermediate-vertex closure supplies reference weights and negative-cycle reachability. Boundary tests include zero cycles, disconnected negative self-loop, parallel edges, i64 extremes and validation precedence. A compiled mutant disabling last-round cycle rejection must fail cycles_and_full_width_paths.

Focused debug runs twice and release run require identical normalized per-test statuses; whole-crate debug tests and strict all-target Clippy are also checked. The compiled mutant is an external copy, with exactly one selected test failing (exit 101); production source remains intact. Source-bound syntax is generated twice and compared after formatting. These finite checks do not establish a universal proof, runtime improvement or application integration.

Commands: cargo test -p mova-algorithms --offline bellman_ford::tests:: -- --test-threads=1; same with --release; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings; mova-modelcheck syntax REPO SOURCE OUTPUT. Temporary uncommitted library exports enable focused checks; integration owns exports and registry.

Observed result: all three focused tests passed in both debug runs and release, with identical normalized statuses. Both source projections were identical across generation runs. Strict Clippy passed. The named external mutant compiled and was rejected: 0 passed, 1 failed, 2 filtered out, exit 101. No universal or approximate-profile claim follows.

## prospective

### Prospective

Before implementation: validate source then every edge, including unreachable edges. Synchronous distances after k rounds minimize walks of at most k edges; improvement in round V means a reachable negative cycle. Signed i64 weights widen to checked i128. Predict O(V(V+E)) time and O(V) auxiliary space. Independent Floyd closure on every <=3 vertex graph with each ordered pair absent, weight -1 or +1; boundary fixtures cover zero cycles, disconnected negative cycles, parallel arcs and full-width paths. Compiled cycle-detection mutant must fail.

Finite tests and syntax projections are evidence, not proofs. No approximate executable profile is proposed.

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch 2026-10-30` stays in descriptant as `### Modeling`.

#### Deep graph epoch 2026-10-30

Three additional tests connect the specialized mathematical relations to actual production results. Implementation/API/error behavior is unchanged. The new models distinguish exact deterministic outputs from coarser optimal-value observations and retain preimage ambiguity. Earlier evidence remains historical. No speed, approximation profile, general proof or production substitution is claimed.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

#### Deep graph evidence 2026-10-30

Source SHA-256: 5a4f12d9d4be30194a32ba3e4cd86ed40180674c0555d63cea3b643b0cfe4dc6. Six focused tests pass in debug and release, each repeated twice with identical named outcomes (12 tests across the two algorithms). Strict all-target Clippy passes. Two isolated mutant files compile, and each named new test rejects its algorithm's mutant. Mutant log SHA-256 for this algorithm: 07438187ebd41ab50bb24304003cb1ee9c279387c1d9840fd63b15fe58da8776. Raw logs live under MOVA_DATA/runs/octopus-deep-graphs-2026-10-30. Syntax generation is repeated twice per source and compared byte-for-byte.

Bellman fixtures: 576 potential-transport pairs, one reachable-cycle transport pair, 84 distance-certificate memberships (one accepted), 75 compatible graph constructions and 25 negative-slack exclusions. Matching fixtures: 512 residual/cover certificates, 512 transpose comparisons, 4608 one-edge insertions, 512 exact-output reconstructions with duplicate variants; duplicate-right and alternate-maximum observations are excluded. These are finite correspondence checks, not unrestricted proofs.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.

### 2026-11-01 · document

#### Validation prefix and error fiber — review 2026-11-01

For a valid source, if the first invalid endpoint in row/edge order is
`(u,v)`, the result is `InvalidEdge { from: u, to: v, vertices: n }`.
This validation includes unreachable rows and precedes cycle detection. The
reported error leaves the offending weight and later edges undetermined;
changing the first invalid endpoint changes the observation. An invalid source
preempts the edge error. For example, a reachable negative self-loop alongside
an invalid edge in a disconnected row yields the edge error, not the cycle
error. Removing the invalid edge exposes the cycle. These are error-partition
and partial converse observations, not a complete inverse or a proof of all
behavior. An external actual-source Rust probe checked these cases twice on
2026-11-01; allocation failure is outside the observation.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
