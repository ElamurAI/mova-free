# algorithm-composition-wave7

## descriptant

### Combiner and epoch memory

Combiner inputs are Rust, its partial syntax projection, the forward/invariant model, ten typed relations and retained native evidence. A candidate adapter change must preserve units and certificate laws before repeated tests can support it. Automatic generation or composition-wide proof is not implemented.

Measured revision: all eight joins passed four repeated profile runs; a compiled wrong knapsack value adapter failed. Initial strict Clippy found an indexed symmetric-matrix fixture loop and a needless Vec; replacing the loop with explicit edge pairs and the Vec with an array preserved fixture scope. This style correction is not an algorithmic performance claim. No new executable approximant is warranted for these categorical checks.

## approximant

### Forward and invariant

Accept iff all eight public-API interaction assertions pass. The forward observation is a sorted set of eight test-name/pass records; raw runtime order is scheduler-dependent. Inputs are immutable. Adapters preserve graph vertex IDs, capacities, element units and half-open interval boundaries. Numerical equality is supplemented by feasibility certificates where available. Exact join domains, conditional costs and known information loss are retained in model.md#models.

The suite has no recovered mutable state, probabilistic acceptance or external effects beyond native test execution. Panics identify failed assertions; allocation failure is outside the observed domains. This is an exact test harness, not a faster numerical approximant.

## models

# Typed interaction models

Revision is bound to the exact Rust hash in manifest. Targets below are source-derived relational specifications; actual correspondence is finite measured evidence. All calls have valid inputs and successful allocations. No error-domain or unrestricted semantic proof is claimed.

## behavior_complement

Universe: finite fixture executions with adapter inputs, public results, certificates and assertion verdicts. Exclude accepted observations that change units, omit reachable vertices, encode edge equality instead of inequality, double undirected cut capacity, discard overlapping matches, confuse substring and subsequence, use unrestricted edits, change value=weight, or count half-open endpoints. Passing finite fixtures cannot exclude unexecuted behavior.

## reverse_reconstruction

Fix an observed assertion transcript. Its preimage contains all implementations and adapters reproducing every fixture observation; the source implementation is one antecedent, not uniquely recovered. Feasibility and objective assertions constrain compatible witness antecedents. Distance/cut totals do not recover graph topology; match vectors do not recover strings; union coverage loses interval multiplicity. The wrong-adapter mutation removes one candidate from the finite accepted family.

## astar_distance_path_join

Domain: all 512 directed three-vertex topologies including loops, one deterministic 1/2-weight assignment per topology, every start/goal and two heuristics. Hold graph/endpoints fixed. h=0 and h(v)=1 except h(goal)=0 are consistent since weights are positive. Project A* optional Path to optional distance and compare Dijkstra[goal]; separately check endpoints, every directed edge, at most V vertices and summed edge weights. Equality of distance alone cannot certify the path. There are 9216 A* calls. Cost is the sum of bounded Dijkstra/A* calls, each O(V+E log(E+1)) up to initialization terms.

## sat_parity_join

Domain: 64 undirected three-vertex graphs with optional loops, plus zero variables. Each uv becomes (u OR v) AND (!u OR !v), exactly u!=v. BFS every uncolored component and assign distance parity. Existence of a SAT assignment equals absence of an edge with equal endpoint parity. Actual assignments must satisfy both clauses and edge inequality. Disconnected roots are independent; loops contradict. A wrong polarity adapter can preserve satisfiability on some graphs but fails the full bounded universe. Cost conservatively O(64*(V*(V+E)+SAT(V,E))).

## global_terminal_cut_join

Domain: 729 symmetric four-vertex matrices, each of six capacities independently in {0,1,2}. Replace each undirected capacity with both directed arcs; a cut counts only outgoing arcs and therefore preserves, rather than doubles, capacity. Global minimum equals min over sinks !=0 of max_flow(0,sink), because every proper cut separates zero from some sink. Returned side must contain zero, be sorted/proper, and its recomputed original crossing capacity must equal the reported weight. Cost O(729*(V^3+V*flow(V,E))). Disconnected zero cuts are included; lexicographic global optimality is not assumed.

## three_search_join

Domain: 127 binary texts length<=6 and 15 binary patterns length<=3, plus an actual wrapping-hash collision. Hold byte inputs fixed. Require full ordered Rabin-Karp and Horspool offset lists equal KMP; preserve overlaps and all empty-pattern boundaries. Comparing only counts would lose positions. Shared specification errors remain possible, so leaf independent oracles are separate evidence. There are 1905 enumerated pairs. Worst-case combined cost is O(n*m+n+m) per pair plus result storage.

## substring_index_join

Domain: 31 squared binary strings length<=4. Extract the witness by indices, require equal destination slice and membership in both suffix automata, and bound its length by LCS. Every substring is a subsequence, but the converse is false. Membership proves only feasibility: test every longer a slice against b's automaton and reject containment, independently checking maximality. Empty witness is accepted by both automata. Tie selection remains a leaf obligation. Cost includes index builds, O(n*m) LCS, and O(n^3) worst-case candidate byte visits.

## restricted_edit_join

Domain: 31 squared binary strings and CA/ABC, CA/AC fixtures. Restricted adjacent transposition adds options to Levenshtein, hence abs(length difference)<=OSA<=Levenshtein. Bounds alone do not determine the exact restricted distance. CA/AC distinguishes transposition support; CA/ABC retains OSA cost three and is not silently assigned unrestricted cost two. No triangle inequality follows. All costs use byte units. Work is O(n*m) per observer.

## subset_objective_cardinality_join

Domain: 127 arrays length<=6 over {0,2}, targets 0..13, plus unit arrays n=0..9. Map v to knapsack(weight=v,value=v), capacity target. Maximum value equals target iff subset_sum returns a witness. Check increasing distinct indices and exact witness sum. Zero items do not change attainable sums. For unit arrays, enumerate every index mask into a cardinality histogram, compare count[k] with C(n,k), require subset witness length k and reject n+1. This counts all subsets independently of the solver's single witness. Cost O(N*n*T + sum 2^n), on small finite bounds. A compiled value=0 adapter mutation must fail.

## interval_grid_fenwick_join

Domain: all ordered triples of the 15 valid integer intervals with endpoints [-2,2], giving 3375 cases. Cell x denotes [x,x+1), x=-2..1. Build Boolean coverage, not multiplicity, into Fenwick points. Full prefix sum equals union covered length. Check every cell's membership and strictly separated canonical output intervals. For every grid range, clipped union lengths equal Fenwick range_sum. Empty and touching intervals are included. Cost O(I log I+G*I+G^2*(I+log G)). The integer-grid relation does not claim coverage of arbitrary real endpoints.

## evidence

### Native validation

A native external Rust driver changed the actual knapsack adapter from value=v to value=0 while preserving weight=v, compiled the modified test target, and observed an assertion failure in subset_knapsack_and_unit_cardinality_binomial. Compile failures were rejected as evidence. The source was restored byte-for-byte before projection and commit.

## History

### 2026-10-30 · descriptant

2026-10-30 prospective prediction: shared public interfaces agree after explicitly specified adapters, but equality of projected scalars alone will miss witness and convention bugs. Therefore path, assignment, cut, substring and subset witnesses are checked separately; interval queries are checked at every bounded cell/range.

### 2026-10-30 · evidence

2026-10-30: eight tests passed twice in debug and twice in release using imported string, graph and discrete commits and temporary library exports. Sorted test-name/pass lines are identical across all four runs. Strict Clippy for the composition target and focused rustfmt passed. Partial syntax projection was run twice and compared byte-for-byte.

Artifacts: $MOVA_DATA/runs/octopus-wave7-composition-2026-10-30/{debug1,debug2,release1,release2}.log and .normalized; clippy.log, mutant.log, control.rs, control.log and syntax2.json. Finite measured outcomes are separate from unrestricted correctness, resource failures and full-system behavior. No teacher, live API, GPU or bus calls.
