# algorithm-library

## model

# Algorithm library composition

Subject: 200 public modules; 413 bound algorithm source leaves; 15 internal and 91 external literal test declarations across 12 external harnesses, totaling 106 bounded composition checks. Five wave26 families organize 25 new modules. Counts use literal #[test] markers, not expanded macros. Mova checks are separate.

## Forward approximant

Let R_API be the disjoint tagged union of the 200 module API relations, retaining
each leaf's domain, error precedence and successful-allocation assumptions.
Let R_join contain the 106 compositions below and in the specialized catalog under lawful trait behavior.
R_slice is their tagged union with the source-bound test-harness observations.
No school-training or inference connection follows from the library's existence.

1. Stable sort/search: y preserves x's multiset and equal-key order;
   lower_bound(y,q)=count{x_i<q}, with first match at that rank iff q occurs.
2. Unit-weight distances: W(G) preserves directed endpoints and assigns each edge
   cost one; Dijkstra(W(G),s) is the u128 embedding of BFS(G,s), including None.
3. Undirected partition: union every edge from singleton DSU state;
   connected(u,v) iff BFS(G,u)[v] is Some, and components counts those classes.
4. Ordering joins: heap sort and stable merge sort agree on ordered values;
   quickselect(x,k) equals sorted(x)[k] for valid k. A strictly increasing
   subsequence of maximum length in sorted(x) has exactly one value per
   distinct order-equivalence class. Stability and a particular LIS witness
   are separate leaf obligations; these joins do not equate payload identities.
5. Weighted reachability: for valid nonnegative weighted G, each Floyd-Warshall
   row equals Dijkstra(G,s). Ignoring weights preserves reachability, so vertices
   u,v share an SCC iff both distances d[u,v] and d[v,u] are finite. Canonical
   component presentation does not add information beyond that partition.
6. Spanning forest: selected original edge indices identify an acyclic forest
   spanning the same undirected connectivity classes as the input. Its component
   count agrees with independent reachability, and its reported total equals
   the sum of selected original weights. These observations do not prove
   minimum weight; that remains a separate Kruskal leaf/oracle obligation.
7. Prefix reconstruction: after accepted Fenwick updates, recover
   a_i=P(i+1)-P(i), where P(0)=0 and P is the exact exclusive prefix sum.
   Build an immutable sparse table from this recovered snapshot. Its query
   on a valid nonempty interval equals the minimum of that interval in the
   updated reference sequence. Direct range sums equal P(end)-P(start), including
   zero on empty ranges; the sparse table returns no minimum for an empty range.
   No query is made against a stale snapshot.
8. Restricted square/root: if x is u64, x*x fits u64 and modulus m satisfies
   x*x<m<=u64::MAX, then modular_power(x,2,m)=x*x and integer_sqrt returns x.
   The no-reduction premise is essential. It excludes modulus zero and also
   any square for which no larger representable modulus exists.

9. Signed distance embedding: Bellman-Ford agrees with Dijkstra after an exact nonnegative weight/result conversion.
10. Matching residual: valid maximum matching leaves no free right vertex reachable from a free left in the alternating residual graph.
11. Prefix matching: sentinel-separated Z prefix lengths identify exactly the nonempty KMP matches.
12. Palindrome edit: each centered span has zero reverse edit distance and cannot extend at both ends.
13. Range snapshots: dynamic tree, static sparse table and window deque share values, exclusive bounds and leftmost ties.
14. Integer sorting: counting/radix/generic sorting preserve one ordered multiset and original ranks.
15. Prime inverse: normalized Bezout coefficients and Fermat powers give the same inverse for nonzero prime residues.

The seven new catalog sections specify precise domains, complementary exclusions, reverse ambiguity, fixture bounds and composed cost. In particular, palindrome DP is oracle overhead, and matching validity precedes its residual certificate.

## Invariant

Every adapter establishes the next API's domain and preserves shared identities.
Sorting preserves multiplicity; unit-weight conversion preserves endpoints;
undirected symmetry is necessary for a connectivity partition. Weighted graph
views retain all vertices, including isolated ones, and all endpoints are validated.
Returned Kruskal indices address the original edge array, not a sorted copy.
Path reachability includes the zero-length self path, including isolated vertices.

Fenwick updates are applied to the reference state only after acceptance. Invalid
or overflowing updates leave both states unchanged. On supported usize<=64 targets,
sums of stored i64 values fit i128; adjacent prefix differences recover i64 values.
The sparse table is rebuilt from the same post-update sequence before comparison.
Square/root checks admit inputs only after checked multiplication and strict modulus
comparison. No relation assumes invertibility after modular reduction.

Each changed leaf or adapter requires its local checks and the affected join checks.
Cross-algorithm agreement can hide shared defects, so independent leaf oracles remain
necessary evidence. Error enum equality is not inferred between different APIs.

## Cost

Count scalar comparison, cloning and fixed-width arithmetic as unit operations;
generic element costs and allocator effects remain separate. Costs compose only
along executed paths, not by adding every child of the ownership tree.

The original sort/search join costs O(n log(n+1)) time and O(n) scratch plus stack.
Unit-weight conversion plus BFS and Dijkstra conservatively costs
O((V+E)log(V+E+1)) time and O(V+E) space. All-source BFS for the DSU oracle costs
O(V(V+E)) time and retains O(V^2) observations, plus DSU queries/unions.

For the ordering join, heap/merge sorting and LIS cost O(n log(n+1)); a deterministic
quickselect call has worst-case O(n^2) unless its leaf proves a stronger bound.
Checking q independent ranks therefore adds O(q*n^2) worst-case work plus copying;
exhausting all n ranks can cost O(n^3). This is oracle work, not one query's cost.

Floyd-Warshall costs O(V^3+E) time and O(V^2) space; all Dijkstra rows add
O(V*(V+E log(E+1))) time. SCC traversal plus canonical output ordering is bounded
by O(V+E+V log(V+1)). Mutual-reachability comparisons add O(V^2) work.
Kruskal costs O(V+E log(E+1)+E alpha(V)); repeated reachability oracles may dominate
that bound. Edge-index accounting and selected-weight summation cost O(V+E).

With q accepted or rejected point updates, r complete snapshot rebuilds and k
range queries, the prefix/sparse join costs
O(n + q*(1+log(n+1)) + r*(1+n*log(n+1)) + k*(1+log(n+1))), plus
reference scans up to O(k*n). Each range check includes a Fenwick range_sum
as well as an O(1) sparse-table query; per-call constants cover empty inputs. Each rebuild includes prefix reconstruction and
sparse-table construction; omitting repeated rebuilds would undercount the suite. Sparse storage is
O(n log(n+1)); Fenwick state and recovered values use O(n).
The square/root join uses fixed-width modular arithmetic and an integer-root
computation bounded by the u64 width; no measured throughput follows.
Finite fixture counts and suite observations are recorded separately in model.md#tests.

## Composition and reconstruction

For relations R1;R2, Pre(z)={x | exists y: R1(x,y) and R2(y,z)}.
Join intermediate values and source/snapshot identities before projecting them away.
A rank or sorted sequence does not recover the original permutation. Distances and
SCC partitions do not determine a unique graph; a forest or total weight does not
determine discarded edges, original tie multiplicities or a unique optimal forest.

The prefix transform is a useful exception on its declared exact domain:
P(0)=0 and all adjacent differences in i64 determine one sequence by finite
differences, and re-summing it reproduces P. A range minimum alone is many-to-one,
and an arbitrary i128 vector is not necessarily a valid Fenwick prefix history.
For integer root r, antecedents include every n satisfying r^2<=n<(r+1)^2,
intersected with u64. The square/root join restores x only after restricting the
forward input to an exact nonnegative square with no modular reduction.

The complement of a composition is not a union of child complements: correct
leaves can be wired with wrong indices, stale snapshots or inadmissible scaling.
The parent catalog and source-bound harness groupants define additional tagged joins beyond the original three, with explicit domains and limits. Model-view counts remain distinct from the 70 executable composition relations.

## Epoch memory

Review of the initial algorithms corrected a maximal zero-sized-type Levenshtein
dimension boundary and a BFS reverse model missing the unique-zero source.
The test split made implementation and oracle identities separate obligations.
The new joins deliberately distinguish connectivity from minimum weight, sorted
values from stable identities, and an exact prefix inverse from lossy minimum or
modular observations. Source syntax remains partial and declarative preimages
are not general executable inverse solvers.

## Test-source binding

Under cfg(test), implementation module i loads adjacent tests.rs. Bind exact
implementation bytes P_i, test/oracle bytes T_i and fixed generated domain D_i.
For compiled, terminating executions, H(P_i,T_i,D_i)=Pass iff all executed checks
meet their declared expectations; a rejected observation does not identify a
unique implementation fault. Expected-panic checks retain their harness semantics.
Ordinary library builds omit test modules.

A partial syntax projection of mod.rs sees the test declaration but does not load
its body. Both source leaves must be bound. A passing run with an empty, substituted
or differently configured oracle is not evidence for the recorded suite. Production
semantics, oracle validity, actual execution and mutation sensitivity are distinct
obligations. The current suite contains fifteen internal composition tests and 55 external checks; all pass in the repeated integrated debug/release suite recorded in [wave16-evidence](model.md#wave16-evidence).
Executed results and mutation sensitivity are recorded in model.md#tests.

## Fourth-wave external joins

The separate registered composition_wave4 source contributes seven more observations: binary-weight distance embedding, acyclic signed distance restriction, multi-index byte search, LCS/edit bounds, interval/capacity restrictions, hull/nearest-pair noncommutation and prime-power residue reconstruction. Each catalog section preserves its own domain and inverse ambiguity. Detailed composed costs are in that harness groupant; no sum over the entire ownership tree is treated as one execution.

## Fifth-wave external joins

Seven more tagged joins preserve matching/flow partitions, indexed multigraph identity, forest rooting, substring language, code-tree weights, palindrome lengths and restricted unit-work costs. The external harness has its own source groupant; detailed domains, nonunique preimages and test-enumeration costs remain there and in the parent catalog.

## wave14-extension

Wave14 adds ten exact algorithms and nine native joins preserving graph, preference, byte, index and residue coordinates. Existing internal/external joins retain their original scopes.

Complement: Universe: tagged lawful graph, preference, residue, byte and range inputs with candidate adapter outputs. Exclude uncovered edges or cover/matching/flow cardinality disagreement, a noninverse common-preference assignment, composite prime-gate success or a noncanonical modular root, missing/extra byte matches, a nonminimal/later-tied rotation, a nonleftmost range minimum, wrong strict inversion accounting, failed normalized subset identity and OR output using XOR coordinates. These concrete exclusions are conditional on shared valid domains; finite checks do not prove all Rust paths.

Reverse: Retain each tag's compatible input fiber. Cover cardinality loses graph edges; inverse assignments recover priority only under common complete preferences; roots lose the sign representative; match vectors lose unmatched bytes; minimal indices lose text; minima lose larger values; inversion totals lose ordering; normalized transforms recover residues but not integer lifts; OR convolution admits multiple factor pairs. Reverse composition preserves identical shared coordinates. Passing transcripts also admit implementations differing outside tested bounds; no unique cause or general inverse is inferred.

Costs count reached APIs and adapter/oracle work only. Repeated native evidence is summarized in model.md#tests; final source and log bindings are maintained separately in model.md#wave14-evidence. Prior dated evidence remains historical. No wave14 benchmark or throughput gain is claimed.

## wave16-extension

Wave16 adds ten exact algorithms and eight native joins with explicit graph, byte, residue, predecessor and timeline coordinates; fifteen internal joins and previous external harness scopes remain.

Complement: Universe: tagged lawful graph, labeled-tree, byte, sieve, recurrence, envelope and timeline inputs with candidate observations. Exclude wrong signed forest shift/cardinality, tree degree/code inconsistency, wrong complement clique/tie, edit threshold or LCS bounds, divisor identities, recurrence/Cassini mismatch, DP values/predecessors and stale or reordered connectivity answers. Conditional concrete exclusions are not an all-Rust proof.

Reverse: Retain each tag's compatible inputs with identical shared coordinates. Costs/components lose graphs and ties; full fixed-n Pruefer codes recover labeled trees but degrees do not; edit/LCS lengths lose bytes; phi/mu scalars and modular Fibonacci lose indices; DP observations lose coordinates/losing lines; connectivity traces lose invisible edges/loops. Finite passing assertions admit implementations differing elsewhere.

Reached API costs only. Bounded native evidence is recorded in model.md#wave16-evidence; prior evidence retained.

## wave18-extension

Wave18 adds ten public algorithm modules and five separate tagged integration relations: canonical multigraph blocks to bridges and source counts, decoded bytes to matching/alignment, sieve primes to checked bounded factor products, floor sums to threshold counts, and persistent versions to wavelet order statistics. Each join preserves original vertex/edge identity, byte offset, exact integer domain, half-open range and immutable version coordinates. Domains are the literal six-vertex/seven-edge graph, 36 byte pairs, n=1..160 factors, 2275 bounded progression tuples and five length-five versions. Composition costs are sums of the reached API/adapter/oracle costs, including allocation and output, not a speedup claim. Distinct harness tests do not form one end-to-end algorithm.

Complement: Extend the algorithm-library complement with incompatible observations at these five tagged joins: edge loss/loop confusion, wrong graph coordinates or path multiplicity, byte-offset or traceback drift, invalid prime/product agreement, wrong strict-threshold conversion, or cross-version/range mismatch. Leaf correctness alone does not ensure adapter correctness. Thresholds through 60 are justified only by v<=48; snapshot casts require nonnegative i64 values. Behavior outside the bounded harness domains, allocation failure and general performance remain explicitly unestablished.

Reverse: For each successful join keep the intersection of compatible leaf antecedents after enforcing shared coordinates, not a unique recovered graph, text pair, progression or version history. Traceback columns and decoded canonical runs can recover supplied byte values; sorted factors recover n by bounded product. Counts, partitions, sums and multisets generally have non-singleton fibers, especially for graph endpoints, vertex permutations and branching update histories. Passing finite joins constrains only tested tagged inputs.

Reached API costs only. Bounded native evidence is recorded in model.md#wave18-evidence; prior evidence retained.

## Current wave18 evidence

110 implementations and 110 test sources, the API boundary and eight external harnesses form 229 registered algorithm source leaves. File trees contain 1,402 views; the parent has 80. Fifteen internal and 60 external checks give 75 composition relations. Final root execution passes 550 checks twice per profile with identical outcomes. See model.md#wave18-evidence for scope, controls and receipts; bounded CPU comparisons are in the parent model.md#wave18-benchmark. Source registration and finite execution are not universal proof.

## wave19-extension

Eight distinct bounded client relations cover all twenty modules and cross five new families through actual numeric transports. Independent diagonal minimizers, polynomial identities, KKT constants, direct centered sums, literal counts/ranks, exhaustive subset masks and exact integer arithmetic supplement cross-implementation equality. Inputs and error metrics remain tagged; this is not one serial end-to-end computation.

Complement: On the eight tagged bounded domains define R by the named independent identities and inequalities. Its complement includes mismatched polynomial bases, derivative signs, diagonal coordinates, threshold signs or mass, lost merged counts, wrong variance denominator, frequency bounds excluding literal counts, wrong inclusive-value/exclusive-rank endpoints, unattainable subset objectives, loss exceeding integer budget, and dot certificates normalized by |sum| instead of sum|products|. Invalid inputs, allocation failures and budget statuses are categorical, not numerical tolerance. These tests do not exhaust R's complement or establish universal floating-point accuracy.

Reverse: Retain every tagged input compatible with shared coordinates and observed identities. Full polynomial coefficients under a fixed-degree basis transform have a unique converse, and known nonzero signs reconstruct l1 coordinates from magnitudes, but arbitrary finite samples do not identify functions. Threshold zeros retain intervals; moments/frequency/histogram observations lose order; objective values retain many subset masks; dot values retain many vector pairs. The bounded exhaustive subset oracle checks nonempty feasible fibers. No general inverse execution engine or composition-wide percentage bound is asserted.

Reached API costs only. Bounded native evidence is recorded in model.md#wave19-evidence; prior evidence retained.

## wave20-extension

Eight production joins cover all twenty new modules. Factor solves, normal equations, scalar directions, thresholding, polynomial bases, integrated interpolants, attainable subset fibers and DP coordinates connect actual outputs and shared mathematical inputs. Independent exact oracles supplement implementation agreement.

Complement: On the tagged bounded joins, incompatible factors, matrix/sign conventions, derivative directions, basis coefficients, anchor coordinates, unattainable objectives or DP recurrence minima lie outside the admissible relation. Invalid inputs and resource statuses are categorical. These fixtures do not exhaust the complement, prove floating accuracy, or verify caller DP assumptions on arbitrary callbacks.

Reverse: Each output denotes all compatible antecedents: fixed nonsingular linear systems and fixed-degree full polynomial bases may be unique; optimizer values, integrals, subset objectives and DP minima generally admit many inputs/witnesses. Retained original coordinates establish compatibility, not an executable inverse or universal reconstruction.

Reached API costs only. Bounded native evidence is recorded in model.md#wave20-evidence; prior evidence retained.

## historical-wave19-subject

Subject: 270 source leaves (130 implementations, 130 test modules, one public boundary and nine external harnesses). Five family groups organize the twenty wave19 modules. The branch has 1632 file-tree views, 88 library-parent views and 25 new family views. There are 83 composition checks: fifteen internal and 68 external. These are tagged observations, not one serial pipeline. Exact transitive sources and immediate children are bound by manifests.

## historical-wave20-subject

Subject: 311 source leaves (150 implementations, 150 test modules, one API boundary and ten external algorithm harnesses), with 1865 file-tree views, 96 library-parent views and 45 family-group views. There are 91 algorithm compositions: fifteen internal and 76 external. Three new qagen joins and six historical surrogate sources are outside these algorithm totals. Binding and partial syntax certify declared provenance, not semantic completeness.

## historical-wave25-subject

Subject: 362 source leaves (175 implementations,175 test modules,one API boundary and11 external algorithm harnesses). Five additional family groups organize25 wave25 modules. There are103 algorithm composition checks:15 internal and88 external. Mova repairs,qagen joins and historical surrogates remain outside this algorithm subtree. Exact bindings and partial syntax do not establish semantic completeness.

## catalog

# Specialized library models

Models are partial descriptions of the source-bound algorithm branch. The first
two types are required. Executed bounded evidence for all fifty-three joins is in model.md#tests.

## Complement

Let U be the disjoint union of tagged valid-domain API, fifty-three composition and
fixed-suite harness input/observation spaces. Let R be their declared forward
relations, including both permitted successes and declared errors. Then
C_spec=U minus R excludes wrong ranks, distances, partitions, weights, prefix
differences, range minima, roots and shared identities. C_model approximates this
target through explicit equations and leaf exclusions; tests do not enumerate all
Rust behavior. Allocation failure, runtime and violated trait laws are outside U.
For compiled terminating harness runs, define Pass/Fail according to actual
assertion and expected-panic semantics, not a fabricated always-pass relation.
The child-complement union does not characterize every bad composition.
Oracle: independent leaf checks plus the selected join fixtures; the existing
weight-two adapter mutation is one negative witness, not proof of this complement.


Wave14 extension: Universe: tagged lawful graph, preference, residue, byte and range inputs with candidate adapter outputs. Exclude uncovered edges or cover/matching/flow cardinality disagreement, a noninverse common-preference assignment, composite prime-gate success or a noncanonical modular root, missing/extra byte matches, a nonminimal/later-tied rotation, a nonleftmost range minimum, wrong strict inversion accounting, failed normalized subset identity and OR output using XOR coordinates. These concrete exclusions are conditional on shared valid domains; finite checks do not prove all Rust paths.


Wave16 extension: Universe: tagged lawful graph, labeled-tree, byte, sieve, recurrence, envelope and timeline inputs with candidate observations. Exclude wrong signed forest shift/cardinality, tree degree/code inconsistency, wrong complement clique/tie, edit threshold or LCS bounds, divisor identities, recurrence/Cassini mismatch, DP values/predecessors and stale or reordered connectivity answers. Conditional concrete exclusions are not an all-Rust proof.


Wave18 extension: Extend the algorithm-library complement with incompatible observations at these five tagged joins: edge loss/loop confusion, wrong graph coordinates or path multiplicity, byte-offset or traceback drift, invalid prime/product agreement, wrong strict-threshold conversion, or cross-version/range mismatch. Leaf correctness alone does not ensure adapter correctness. Thresholds through 60 are justified only by v<=48; snapshot casts require nonnegative i64 values. Behavior outside the bounded harness domains, allocation failure and general performance remain explicitly unestablished.


Wave19 extension: On the eight tagged bounded domains define R by the named independent identities and inequalities. Its complement includes mismatched polynomial bases, derivative signs, diagonal coordinates, threshold signs or mass, lost merged counts, wrong variance denominator, frequency bounds excluding literal counts, wrong inclusive-value/exclusive-rank endpoints, unattainable subset objectives, loss exceeding integer budget, and dot certificates normalized by |sum| instead of sum|products|. Invalid inputs, allocation failures and budget statuses are categorical, not numerical tolerance. These tests do not exhaust R's complement or establish universal floating-point accuracy.


Wave20 extension: On the tagged bounded joins, incompatible factors, matrix/sign conventions, derivative directions, basis coefficients, anchor coordinates, unattainable objectives or DP recurrence minima lie outside the admissible relation. Invalid inputs and resource statuses are categorical. These fixtures do not exhaust the complement, prove floating accuracy, or verify caller DP assumptions on arbitrary callbacks.

## Joined preimages

Pre(z)={x | exists y: R1(x,y) and R2(y,z)} retains intermediate and snapshot
identities. Sorted observations retain compatible original permutations; distances,
SCC partitions and selected forests retain all compatible graphs and histories.
Prefix observations with P(0)=0 and valid adjacent differences determine one value
sequence, but do not reconstruct update history. Range minima and modular residues
remain lossy. An observed test failure has multiple compatible implementation,
adapter and oracle defects. These are relational descriptions, not general
implemented inverse generators or complete enumeration of all antecedents.


Wave14 extension: Retain each tag's compatible input fiber. Cover cardinality loses graph edges; inverse assignments recover priority only under common complete preferences; roots lose the sign representative; match vectors lose unmatched bytes; minimal indices lose text; minima lose larger values; inversion totals lose ordering; normalized transforms recover residues but not integer lifts; OR convolution admits multiple factor pairs. Reverse composition preserves identical shared coordinates. Passing transcripts also admit implementations differing outside tested bounds; no unique cause or general inverse is inferred.


Wave16 extension: Retain each tag's compatible inputs with identical shared coordinates. Costs/components lose graphs and ties; full fixed-n Pruefer codes recover labeled trees but degrees do not; edit/LCS lengths lose bytes; phi/mu scalars and modular Fibonacci lose indices; DP observations lose coordinates/losing lines; connectivity traces lose invisible edges/loops. Finite passing assertions admit implementations differing elsewhere.


Wave18 extension: For each successful join keep the intersection of compatible leaf antecedents after enforcing shared coordinates, not a unique recovered graph, text pair, progression or version history. Traceback columns and decoded canonical runs can recover supplied byte values; sorted factors recover n by bounded product. Counts, partitions, sums and multisets generally have non-singleton fibers, especially for graph endpoints, vertex permutations and branching update histories. Passing finite joins constrains only tested tagged inputs.


Wave19 extension: Retain every tagged input compatible with shared coordinates and observed identities. Full polynomial coefficients under a fixed-degree basis transform have a unique converse, and known nonzero signs reconstruct l1 coordinates from magnitudes, but arbitrary finite samples do not identify functions. Threshold zeros retain intervals; moments/frequency/histogram observations lose order; objective values retain many subset masks; dot values retain many vector pairs. The bounded exhaustive subset oracle checks nonempty feasible fibers. No general inverse execution engine or composition-wide percentage bound is asserted.


Wave20 extension: Each output denotes all compatible antecedents: fixed nonsingular linear systems and fixed-degree full polynomial bases may be unique; optimizer values, integrals, subset objectives and DP minima generally admit many inputs/witnesses. Retained original coordinates establish compatibility, not an executable inverse or universal reconstruction.

## Sort search rank

For y=stable_sort(x), lower_bound(y,q)=sum_i[x_i<q].
First match is Some(rank) iff sum_i[x_i=q]>0.
Rank and presence are invariant under input permutation. Payload stability remains
a separate obligation. Existing oracle: all ternary sequences of length 0 through
5 and needles -2 through 2, with rank/presence calculated from unsorted input.

## Unit weight distance

D(W(G),s)[v]=map(u128,B(G,s)[v]) for valid directed inputs. Unit weights make summed
path cost equal edge count. Duplicate edges and self loops are within the relation.
Existing parent fixtures exhaust one-or-zero-edge directed graphs on 1 through 3
vertices from every source. Leaf fixtures cover further edge forms. Invalid
unreachable edges are separately rejected by both APIs. Assigning weight two is
a countermodel for the join even when both leaf implementations are correct.

## Partition reachability

After all undirected edges are unioned, same_set(u,v) iff G has a path u to v.
Components count reachability classes. Roots are not BFS parents: partition
identity is weaker than forest identity. Directed reachability and deletions are
outside this equation. Existing oracle: every simple undirected graph on 1 through
4 vertices, all pairs, with an independent class count from reachability.

## Reached cost

A selected API costs its own work plus actual adapters, not all eighty algorithms.
Sequential costs in model.md preserve generic-operation and buffer assumptions.
Repeated selection, all-source paths and direct range scans are harness work.
Fixture timings are observations, not fitted asymptotic constants or training gains.

## Ordering structure join

Type order_structure_join. Domain: finite sequences of lawful ordered values;
successful allocations and value-preserving clones for the stable-sort branch.
Let S be the common ordered multiset from heap sort and merge sort. For 0<=k<n,
quickselect on an independent copy returns S[k] and preserves its input multiset.
For the sorted S, a maximum strictly increasing subsequence has length equal to
the number of distinct order-equivalence classes. Equal keys must not extend it.
Heap sort need not preserve stable payload identities; quickselect need not order
either partition; no particular LIS witness is inferred from length alone.

Reverse: (S,k,S[k],length) retains all original permutations and all appropriate
selection/LIS witnesses. Distinct-count equality describes the sorted-input
restriction, not LIS length for arbitrary input order. Exclusions include a wrong
rank or a non-strict subsequence of excessive length. Planned finite oracle:
compare the two sorts, selected ranks and distinct counts on the same inputs.
Leaf brute-force oracles remain necessary because agreement can share defects.

## Weighted reachability join

Type distance_partition_join. Domain: valid finite directed graphs with u64 edge
weights and successful exact u128 distance arithmetic. For every source s,
Floyd(G)[s]=Dijkstra(G,s), including None. Ignoring weights preserves reachability.
Thus u and v belong to the same SCC iff both matrix entries are finite; diagonal
zero-length paths make each isolated vertex its own SCC. Components are compared
as partitions; canonical sorting is checked by the SCC leaf.

Reverse: matrices and component labels admit many weighted adjacency lists,
including redundant edges and alternative equal-cost paths. Missing edges cannot
be recovered uniquely. Exclusions include distances that disagree after the
identity-preserving adapter or a partition inconsistent with mutual reachability.
Planned fixtures compare all rows and all vertex pairs. Shortest-path correctness
and SCC correctness still rely on their independent leaf oracles.

## Spanning forest accounting

Type spanning_forest_join. Domain: valid undirected edge lists with u64 weights,
parallel edges and self loops allowed, with a fixed vertex universe. Each selected
index must address the original edge list exactly once. Unioning selected edges
must succeed for each chosen edge (acyclicity), and the resulting connectivity
partition must equal the input graph's partition. Selected edge count is V-c,
and sum of their original weights equals the reported u128 total.
Deterministic tie policy and minimum weight are separate Kruskal obligations.

Reverse: a reported forest admits many original graphs with discarded edges,
self loops, weight ties and orders; the total alone is still less informative.
A heavier spanning tree can satisfy all connectivity/accounting equations, so
these checks must never be described as a minimality proof. Planned oracle:
independent BFS or DSU on original and selected edge views, edge-index accounting
and exact weight summation. The leaf's brute-force oracle checks optimality.

## Prefix difference reconstruction

Type prefix_difference_reconstruction. Domain: a finite sequence of stored i64
values on a supported usize<=64 target, exact i128 Fenwick sums, and accepted
checked point updates. P(k)=sum_{i<k}a_i with P(0)=0. Then
a_i=P(i+1)-P(i), and re-summing these differences reconstructs every P(k).
This inverse recovers values uniquely, not the history of accepted/rejected updates.
A valid candidate prefix vector needs length n+1, zero origin and every adjacent
difference in i64; arbitrary i128 vectors need not meet those conditions.

Build the sparse table from that same recovered snapshot; for 0<=l<r<=n its
query equals min(a[l..r]). Empty/invalid intervals follow the leaf's declared error
or optional-result policy. Rejected Fenwick updates must not advance the reference
or sparse snapshot. The range-min projection is many-to-one even though complete
prefix observations are injective. Planned oracle reconstructs every point after
updates and compares sparse minima with direct reference scans. A stale snapshot
or off-by-one exclusive boundary is an excluded joined observation.

## Restricted square root

Type restricted_left_inverse. Domain: x,m in u64 with checked square q=x*x and
q<m. This implies m>0. modular_power(x,2,m)=q and integer_sqrt(q)=x, so the
composition is identity on x only within this restricted forward domain.
It neither recovers m nor inverts a general residue or floor-square-root result.
If q=u64::MAX there is no representable admissible m; failed checked products
and inadmissible moduli must be excluded before invoking this law.

Reverse outside the restriction is relational: integer root r admits all
mathematical n in [r^2,(r+1)^2) intersected with u64; modular residues generally
admit multiple bases. Planned oracle chooses admissible squares/moduli and checks
the left-inverse equation. Leaf tests cover modulus-zero errors, modulus one and
non-square root boundaries. Multiplication and inequalities here are exact
mathematical statements, not wrapping-u64 arithmetic.

## Integration fixture domains

The root integration source defines these bounded families; execution outcomes remain separate:

- ordering_algorithms_share_ranks_and_strict_length: ternary sequences of lengths 0 through 5; every valid rank, partition inequalities, strict LIS indices and sorted distinct count.
- all_pairs_distances_identify_strong_components: every directed one-or-zero-edge graph on 0 through 3 vertices, self loops allowed, weights (u+v) mod 3; every source row and vertex pair.
- spanning_forest_preserves_original_connectivity: every simple undirected graph on 0 through 4 vertices plus a zero-weight self loop at every vertex; selected indices, union acyclicity, edge count, total weight and all-source reachability. This is not a brute-force minimality oracle.
- prefix_differences_rebuild_range_minimum_snapshots: lengths 0 through 12, three rounds of valid small point updates, all start<=end<=n intervals. Direct range sums also equal P(end)-P(start); empty ranges have sum zero and no minimum. Rejected-update atomicity remains a leaf check, not a claim about these all-success fixtures.
- unreduced_modular_squares_have_exact_integer_roots: bases 0 through 256 plus 65535, 2^31 and u32::MAX, with modulus square+1; 10^2 mod 7 explicitly demonstrates failure of the inverse when reduction occurs.

Replacing prefix(i+1) by prefix(i) makes every reconstructed difference zero. The compiled external mutation executes exactly the prefix reconstruction test and fails its equality assertion with exit 101; the production source remains unchanged.

## Signed distance embedding

Type signed_distance_embedding. Domain: valid nonnegative weights representable in i64, with exact distances representable in i128. Preserve vertices, adjacency and source while changing weight/result integer types. Bellman-Ford distances equal the widened Dijkstra distances, including unreachable None. There is no reachable negative cycle in this domain. Reverse observations retain many compatible weighted graphs, parallel edges and paths; this is not inversion of shortest paths. The complement contains disagreements under this adapter, not differences on signed inputs outside Dijkstra's domain. Fixtures: all directed edge masks at 1..3 vertices, weights (u+2v) mod 4, a parallel edge heavier by 3, every source. Costs add Bellman-Ford O(V(V+E)) and Dijkstra O(V+E log(E+1)); conversion adds O(V+E). Empty graph rejection remains a leaf obligation.

## Matching residual reachability

Type augmenting_path_certificate. Domain: a bipartite graph with fixed L,R and a valid returned matching M (each pair is an original edge, right partners are distinct). Orient unmatched edges left-to-right and matched edges right-to-left. Add a fresh source pointing to every unmatched left vertex. No unmatched right vertex may be BFS-reachable. By the augmenting-path characterization this is a maximal-cardinality certificate, stronger than inclusion-maximality; the theorem plus correct residual construction is required, and finite tests are not a formal proof. Preimages include every bipartite graph admitting the observed maximum matching; neither discarded edges nor a unique matching are recovered. The complement includes wrong endpoint transport, duplicate partners and a reachable free right. Fixtures exhaust L,R=0..3, including empty sides. Residual construction/BFS add O(L+R+E) time/space to matching cost.

## Prefix matching transport

Type sentinel_prefix_transport. Domain: nonempty byte pattern P and text T over {0,1,2}; separator 3 occurs in neither. For J=P+[3]+T, Z(J)[|P|+1+i]>=|P| iff KMP finds P at text offset i. Both indices count bytes, not characters. Empty-pattern boundary semantics belong to leaf tests and are excluded from this join. Reverse fibers retain all strings and patterns satisfying the observations, generally many-to-one; Z values do not determine actual bytes. A delimiter inside the alphabet invalidates this particular prefix-boundary construction's assumptions. Fixtures exhaust ternary text lengths 0..5 with five fixed nonempty patterns, including overlaps and an overlong pattern. Both calls cost O(|P|+|T|), plus combined-buffer construction.

## Palindrome edit identity

Type palindrome_reverse_identity. For a byte sequence X, let odd/even radii define the maximal centered half-open spans [l,r) specified by the Manacher leaf. Each span S obeys edit_distance(S,reverse(S))=0. If l>0 and r<|X|, extending by one byte on each side has positive distance from its reverse. Span bounds and center/parity must be retained: zero edit distance alone does not determine a center or maximal radius. The complement contains nonpalindromic or extendable reported spans. Reverse observations retain all strings with these palindromic constraints; renaming symbols often preserves radii. Fixtures exhaust binary lengths 0..6 and every center/parity. Manacher is O(n), but this all-span DP check is at most O(n^3) time with O(n) peak scratch. The harness is not a fast implementation of palindrome detection.

## Range snapshot transport

Type range_snapshot_transport. Domain: lawful ordered/cloned values, identical successful point-update history and valid nonempty half-open ranges. Tree.query(l,r) equals the immutable sparse-table minimum rebuilt from the current snapshot. Window minimum_indices(X,k)[l] equals tree.query_index(l,l+k), choosing the leftmost tied index. No stale snapshot or mixed inclusive/exclusive boundary is admitted. Singleton minima recover every current value; larger minima generally lose values and all observations lose update history. Fixtures use lengths 0..12, four snapshots including three update rounds, every window width and offset, direct leftmost scan, plus empty/oversize window boundaries. One snapshot adds O(n log(n+1)) sparse construction; all widths add O(n^2) window work and O(n^2 log(n+1)) tree queries, while direct scans can cost O(n^3). Leaf tests separately cover invalid-update atomicity.

## Integer sort rank transport

Type integer_order_rank_transport. Domain: usize values admitting the counting-sort bucket limit; exact lossless mapping to u64 and lawful generic order. Counting, radix and merge sort yield the same ordered multiset; lower_bound equals the count strictly below the target in the original sequence. This does not observe stable payload identities because the public radix/counting APIs sort integers. Preimages include all original permutations with the same multiplicities; rank does not restore order. Wrong conversion, lost duplicates or inadmissible counting spans are excluded. Fixtures exhaust quaternary lengths 0..6, budget four buckets, targets 0..4. Calls add O(n+span), O(8(n+256)), O(n log(n+1)) respectively; auxiliary buffers and oracle scans remain separate.

## Prime inverse transport

Type prime_bezout_inverse. Domain: a prime p emitted by sieve and representable in u64, 1<=a<p, exact arithmetic. Bezout gives a*x+p*y=gcd(a,p)=1, so x mod p is the unique inverse residue in [0,p). Fermat gives modular_power(a,p-2,p) equal to that residue, including p=2. Multiples of p have gcd p and are deliberately outside the inverse claim. The complement contains false prime assumptions, a wrong normalized coefficient or inverse residue. Reverse: given p and an invertible residue, a is unique modulo p; unrestricted a and Bezout coefficient pairs retain congruence-class ambiguity. Fixtures use sieve limit 127, every 1<=a<p, direct product congruence and a=p boundary. Sieve construction is followed by Euclidean and modular-power calls per admitted pair; this is not a claim that enumerating all residues is constant-time. No cryptographic timing or side-channel property is asserted.

Costs above count lawful scalar comparisons and value-preserving clones as constant operations; nonconstant element operations add their own costs.

## certificate-and-observation-boundary

Type certificate_projection. Domain: the declared valid finite domains of the six deepened graph, string and range leaves. Distinguish raw deterministic output R(x), a quotient observation Q(R(x)), and a certificate C(x,z). A valid distance vector needs both edge inequalities and source-reachable tight paths; a matching certificate needs valid distinct original edges and an alternating residual/minimum-cover argument. Scalar optimality alone does not identify the deterministic matching. Leftmost argmin witnesses retain indices, not just minimum values. Excluded projections include correct scalars with invalid witnesses or unjustified strengthening to unique raw outputs.

The new leaf correspondence tests exercise these relations against production functions; existing fifteen cross-algorithm joins remain separate. Their conjunction is bounded evidence, not a new runtime pipeline or universal proof. Reverse queries must retain witness/projection tags before discarding information. Costs count only certificates actually checked; no sum over every ownership child is a client runtime estimate.

## preimage-constraint-intersection

Type joint_preimage_constraint. For observations z1,...,zk of the same source x, Pre(z1,...,zk)=intersection_i Pre_i(zi), with shared lengths, symbol maps, snapshot and tie policy fixed. Separately plausible Z/radius coordinates can have an empty joint fiber. Byte reconstruction may need a representable alphabet; the finite constructive string witnesses are incomplete preimage samplers. Tree singleton queries reconstruct current values but lose update history; window witnesses usually leave multiple values possible. Noninjective symbol/order maps can destroy equality or leftmost ties, so they cannot replace strict/injective maps in the preservation laws.

Executed graph/string/range tests distinguish full output, cardinality, values and witnesses, include explicit nonimage/collapsed-order countermodels, and retain incomplete-constructor limits. This parent relation composes constraints conceptually; it does not implement a general inverse solver. Leaf reverse-model tests and source mutation controls are the evidence, not the declaration count.

## binary_distance_embedding

Type: weight_domain_embedding. For valid directed G with weights in {0,1}, widen weights to u64 and optional zero-one distances to u128. The result equals Dijkstra on the same graph. Zero cycles and repeated edges are allowed. A distance vector does not recover edges or their unique weights. Finite fixtures enumerate all simple directed graphs of one through three vertices with a fixed endpoint-dependent binary weighting and all sources. Costs sum deque traversal, adapter and heap traversal; no speed comparison is inferred.

## acyclic_signed_distance

Type: acyclic_restriction_equivalence. On an endpoint-valid DAG with i64 weights, DAG relaxation equals Bellman-Ford under identical source and i128 arithmetic. Acyclicity is global for the DAG API; Bellman-Ford's error domain differs for unreachable cycles, so unrestricted API equivalence is excluded. Fixtures enumerate forward-edge DAGs of one through four vertices with negative and positive weights. Reverse fibers retain all compatible DAGs, not a uniquely selected topological order.

## multi_index_search

Type: search_observation_join. Aho-Corasick occurrences equal the union of KMP results for each pattern ID, sorted by (end,pattern). Duplicate pattern identities remain distinct; empty patterns match every boundary. For nonempty patterns, suffix-array positions whose suffix starts with the pattern give the same starts after numerical sorting. The empty suffix is absent from suffix_array, so its naive empty-pattern join is explicitly excluded. This projection loses failure links, suffix ranks outside the match interval and unmatched bytes. Fixtures use binary texts of length at most six and a fixed set including duplicates and empty patterns.

## sequence_edit_bounds

Type: subsequence_edit_sandwich. For finite Eq sequences A,B, let L=LCS(A,B), D=Levenshtein(A,B) with unit substitutions. Then max(|A|,|B|)-L <= D <= |A|+|B|-2L. Equality to the upper bound is not asserted because substitutions can be cheaper than delete+insert. Symmetry and sorted-distinct intersection provide additional bounded observations. Reverse lengths preserve many input pairs and many alignments. Tests enumerate binary sequences through length four; cost includes both quadratic DPs.

## interval_capacity_restrictions

Type: scheduling_capacity_embedding. If all positive-length intervals coincide, at most one item is selectable and the optimum equals unit-weight knapsack with capacity one. If intervals are pairwise disjoint in chronological order, every item is compatible and the optimum equals unit-weight knapsack with capacity item count. Compare values, original-index validity, temporal compatibility and summed witness weights, not tie-identical choices. Arbitrary overlap graphs are outside this capacity reduction. Reverse weight observations omit unused items and tie alternatives.

## hull_nearest_noncommutation

Type: lossy_geometric_noncommutation. Hull projection preserves extreme support but can delete the nearest pair. A square with two nearby interior points has squared nearest distance one, while its hull vertices have minimum squared distance one hundred. Thus nearest(hull(P)) need not equal nearest(P). Bounded common translation preserves both hull vertices up to that translation and the full-input nearest index pair/distance. Hull output has many interior-point preimages. The equation assumes no coordinate overflow; it does not transfer nearest-pair certificates from the hull to the whole cloud.

## prime_power_crt_roundtrip

Type: factor_residue_reconstruction. For 1<=n<=256, prime factorization gives powers q_i with product n and pairwise coprime bases, checked against the sieve. CRT on residues x mod q_i returns x mod n with modulus n; n=1 uses the empty system 0 mod 1. Full prime/exponent output reconstructs n, whereas residues recover x only modulo n. Arbitrary noncoprime/incompatible systems and intermediate modulus overflow remain leaf obligations. Enumeration and reconstruction overhead are distinct from standalone factorization or CRT costs.

## matching_flow

Adapter makes s->L, L->R, R->t capacities one with disjoint offsets. Flow value equals matching size; positive middle-edge flow is itself a matching; final source-side cut capacity equals value. Reverse cardinality-k fiber retains all graphs of maximum matching size k, not one matching or edge order.

## graph_identity

For indexed undirected multigraph G, bridge iff c(G-e)>c(G), articulation iff c(G-v)>c(G), removed vertex excluded. Euler existence requires connected nonisolates and zero/two odd degrees; returned IDs permute original edges and each consecutive pair matches its named edge. Loops count twice and parallel IDs stay distinct. Reverse trail plus indexed endpoints constrains edges; connectivity counts alone cannot recover them.

## forest_distance

Parent forest -> symmetric adjacency preserves vertices. If a=LCA(u,v), dist(u,v)=depth(u)+depth(v)-2depth(a); cross-tree pairs return None. Ancestor lies on both parent chains. BFS distances forget root choice; inverse retains all rootings compatible with LCA observations, not a unique forest.

## substring_join

SAM.contains(q) iff KMP positions nonempty, including empty query. Distinct nonempty substrings equals n(n+1)/2 minus adjacent suffix-array LCP sum and literal substring-set size. Count inverse is nonunique; bounded query membership constrains only the observed language. LCP and suffix indices use the same text.

## prefix_tree

Codes are prefix-free original-symbol leaves. Sum w_i*length_i equals sum of descendant weights across trie edges. Independent subset recurrence C(S)=sum(S)+min[C(A)+C(S-A)] gives optimum full binary tree cost for >=2 symbols; empty zero, singleton stipulated one-bit code. Prefix codes reconstruct their trie, but scalar cost loses topology and weights; zero weights and ties remain admissible.

## palindrome_sandwich

LPS length(t)=LCS length(t,reverse(t)); longest contiguous Manacher palindrome <= LPS <= n. This relates lengths; arbitrary LCS witness need not be palindromic. Reverse fiber retains every string meeting observed lengths, not recovered symbols or positions.

## unit_work

Amount graph 0..a with x->x+c for positive coins embeds unlimited minimum count as BFS distance. Separately m>=1 square matrices side d>=1 cost (m-1)*d^3=minimum_coins([1],m-1)*d^3, since every parenthesization has m-1 merges. Restricted unit-work embeddings, not a general coin/matrix reduction. Reverse costs leave denominations, dimensions and parenthesizations ambiguous.

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

## signed-distance-reachability

Type: potential_reachability_composition.

256 four-vertex graphs have w(u,v)=1+h(v)-h(u), hence no negative cycles. Johnson rows equal Bellman-Ford rows; deleting weights yields closure of finite-distance pairs. An unreachable negative cycle distinguishes global from source-local rejection. Sum reached solver costs and preserve vertices.

## forest-connectivity

Type: original_edge_identity.

For 64 five-vertex snapshots, Prim/Kruskal optimum weight and component count agree; witnesses may differ. Original selected edges preserve closure including isolate and loop, count V-components and sum to reported weight. Closure alone cannot prove minimality; leaf oracles remain required.

## lcp-substring-count

Type: suffix_language_cardinality.

For 255 binary words through length 7, n(n+1)/2-sum(LCP) equals suffix-automaton distinct nonempty count. Same-byte SA coordinates identify contributed prefixes. Equal counts lose language identity. Cost includes SA construction, Kasai, automaton build and enumerated membership queries.

## bwt-wildcard-search

Type: reconstruction_search_composition.

For 127 binary texts through length 6 and 15 needles through length 3, inverse(transform(text)) restores bytes. Full wildcard '*' + literal needle + '*' agrees with nonempty KMP output, including empty needle. Needles have no wildcard bytes. Malformed transforms, escaping and normalization are outside this join. Canonical inverse includes re-transform cost.

## bounded-item-expansion

Type: multiplicity_encoding.

64 three-kind count assignments 0..3 and capacities 0..10 expand into distinct 0/1 items with identical optimum. Weights 0,1,2 and small values admit exact u64/u128 conversion; zero-weight copies retain multiplicity. Expansion may cost more than binary decomposition. Optimum cannot reconstruct chosen copies.

## lcs-witness-edit

Type: witness_projection_bounds.

For 961 binary-word pairs through length 4, Hirschberg increasing index pairs preserve original-coordinate equality and project to LCS length L. max(n,m)-L <= Levenshtein <= n+m-2L; insertion/deletion upper bound is not substitution-distance equality. Maximal witnesses can share length; Eq must be lawful.

## ntt-binomial

Type: coefficient_residue_identity.

Repeated NTT multiplication by [1,1] through degree 40 yields C(n,k) modulo 998244353. Compute exact u128 coefficient before reduction; retain low-degree-first orientation. A residue has many integer antecedents. Fixtures do not certify arbitrary transform lengths or floating transforms.

## recurrence-annihilation

Type: generating_function_relation.

Four kernels generate 32 Kitamasa terms under the NTT modulus. Q(x)=1-c0*x-...-c(k-1)*x^k annihilates coefficients k..31 of Q*A. Initial coefficients remain unconstrained; terms beyond the supplied prefix have truncation effects and are excluded. Preserve order and modular negation. A compiled sign-flip adapter must fail. Cost is 32 nth-term queries plus convolution.

## matching-unit-flow

Type: capacity_matching_embedding.

All 512 three-by-three bipartite graphs, with selected duplicated adjacency, become a shared unit-capacity source/left/right/sink network. HK and the earlier matcher agree in cardinality; Dinic and Edmonds-Karp agree with it. Original edge flows respect capacity/conservation and their source-side cut sums to the same value. Witnesses may differ. Include construction plus reached solver/certificate costs.

## lowlink-reachability

Type: canonical_partition_relation.

For all 512 directed three-vertex graphs including loops, Tarjan and the existing SCC API return the same canonical partition. Shared vertex ownership equals mutual reachability in the reflexive closure. Canonical sorting is additional presentation cost, not part of a blanket linear-time claim. The partition cannot reconstruct graph edges.

## unrestricted-edit-bounds

Type: edit_objective_inclusion.

For 225 binary-word pairs through length three, unrestricted Damerau distance is at most OSA, which is at most Levenshtein; symmetry is checked. CA to ABC distinguishes unrestricted distance 2 from OSA 3. The relation follows permitted-script inclusion, not equality of objectives. Byte units and leaf resource limits remain.

## palindrome-occurrence

Type: occurrence_projection_identity.

For 255 binary words through length seven, summed Eertree occurrence counts equal sum(Manacher odd+even radii), and maximum lengths agree. Returned Eertree ranges are palindromes at original byte coordinates. This parent check does not compare every individual occurrence count or longest-tie identity; independent leaf oracles carry those obligations.

## lyndon-rotation

Type: rotation_quotient_reconstruction.

For 255 binary words, factor ranges reconstruct the original bytes in nonincreasing factor order. The last doubled-text Lyndon factor starting before n gives a minimum rotation; compare its bytes with inverse(BWT-column,primary 0). Periodic offsets can differ while bytes agree. Empty text has an explicit case. Cost includes complete validated BWT inverse, not merely linear Duval.

## recurrence-prefix

Type: finite_prefix_continuation.

Four known generated recurrences of order at most three supply twelve prefix terms; BM coefficients feed Kitamasa with the same prime and lag order, reproducing 32 generated terms. Empty inferred order is handled as zero only in the known-zero fixture. Arbitrary finite prefixes admit other futures; this is not prediction of arbitrary unseen data.

## no-carry-convolution

Type: disjoint_bit_embedding.

Across 32 supplied coefficient fixtures, support of A uses low two index bits and support of B uses disjoint high bits. Thus i+j=i XOR j and all sums are below 16: FWHT output equals the NTT low coefficients and higher coefficients vanish. A separate overlap example differs at indices 0 and 2. A compiled wrong-position adapter breaks the premise and must fail. This is not unrestricted convolution equivalence.

## assignment-cycle-relaxation

Type: objective_relaxation_certificate.

64 complete directed fixtures use n=2..5, edge costs 0..8 and diagonal 9n+1. A valid cycle costs at most 8n, so the assignment optimum avoids diagonal and supplies a cycle-cover lower bound on Held-Karp. Check original-coordinate cost, feasible dual equality and tour witness. If assignment is one cycle, objectives agree; multiple cycles need not. Missing arcs and arbitrary diagonal penalties lie outside this adapter.

## algorithm-library-wave14-cover-flow

Type: unit_capacity_cover_flow.

For all 512 three-by-three bipartite graphs, the unit-capacity source-left/right-sink network preserves vertices. Every edge meets the returned cover; cover size equals matching size and Dinic flow, which equals its returned source-side cut capacity. Exclude uncovered edges or scalar disagreement. Equal cardinality leaves graph, matching and cover ambiguity. Cost composes reached matching/flow traversals on bounded networks; no dense-network performance claim.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-priority-inverse

Type: common_preference_permutation.

For n<=5, receivers share priority permutation q and proposers share receiver order 0..n. Assignment[q[r]]=r and inv(assignment)=inv(q). Exclude any different assignment in this complete strict common-preference family. Assignment uniquely recovers q only in this restricted family; inversion count has many fibers and arbitrary preference lists remain ambiguous.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-prime-square

Type: canonical_modular_root_transport.

For p<=100 primality equals sieve membership and composites reject sqrt_mod. For prime p and 0<=a<p, the result equals the first ascending brute root or None; returned r satisfies pow(r,2,p)=a and r<=p-r. Exclude composite-domain success, nonroots and the larger unequal sign representative. Squaring identifies r and p-r; canonicalization cannot recover integer lifts. Costs include bounded root enumeration and exact modular arithmetic.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-byte-matches

Type: bit_parallel_prefix_transport.

All binary texts through length six and patterns through four satisfy Shift-And=KMP=the starts_with boundary oracle. Empty patterns include the terminal boundary and overlaps remain. A 64-byte all-255 pattern in 66 bytes returns [0,1,2]; length 65 errors. Exclude character-coordinate substitution, lost overlaps and top-bit errors. Match vectors cannot recover nonmatching bytes; patterns longer than 64 are outside the successful equality domain.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-rotation-bwt

Type: cyclic_row_identity.

For ternary strings through length six, rotating at the chosen index equals inverse(BWT.last,0), retains the BWT last column and uses the first original start yielding those bytes. Empty input selects zero. Exclude a later periodic tie even when bytes/BWT agree. Last column without original primary row loses phase; the minimal representative cannot recover that phase. Bounded BWT joins remain separate from independent leaf rotation oracles.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-cartesian-lca

Type: leftmost_range_coordinate_transport.

For ternary arrays through length six and nonempty [l,r), LCA(l,r-1) on Cartesian parents equals argmin(value,index), and SparseTable returns that value. Exclude exclusive r as a vertex or later equal minima. Minima lose larger values and topology loses magnitudes. Original indices and earliest ties must agree. Bounded range enumeration adds quadratic queries.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-inversion-reversal

Type: strict_pair_accounting.

For ternary arrays through length six, inv(a)+inv(reverse(a)) equals the number of unequal indexed pairs. Merge sorting yields zero inversions. Equal pairs contribute neither direction. Exclude counting equality or using n(n-1)/2 with duplicates. Sorted values and inversion totals retain many compatible source permutations; zero cannot recover values.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-subset-inverse

Type: normalized_boolean_lattice_inverse.

For lengths 2^b with b<=5 and moduli 1,2,6,97,u64::MAX, zeta(a)[S]=sum_{T subset S}a[T] modulo m and Mobius(zeta(a))=a modulo m. Primality is unnecessary. Exclude missing empty subsets or requiring equality of integer lifts. The full transform recovers residues uniquely, but arbitrary u64 lifts remain ambiguous; m=1 collapses all inputs. Direct oracle is quadratic; transforms cost O(b*2^b).

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave14-or-convolution

Type: boolean_union_product_transport.

For eight-coordinate vectors, sixteen seeds and moduli 1,6,97,u64::MAX, Mobius(zeta(a)*zeta(b))[S]=sum_{i|j=S}a[i]b[j] modulo m. Products use u128 before reduction and stages preserve mask coordinates. Exclude XOR or polynomial-coordinate substitution. Output loses factors, including factor order; integer lifts are also ambiguous. Transform cost O(b*2^b), brute oracle O(4^b); no measured gain claimed.

Source-derived partial relation. Wave14 bounded integration evidence is recorded in model.md#wave14-evidence; prior evidence is retained separately.

## algorithm-library-wave16-forest-shift

Type: signed_cardinality_transport.

Test: signed_forest_shift_joins_boruvka_kruskal_prim.

Domain: five vertices (vertex 4 isolated), all 64 edge masks on vertices 0..4, three weight phases; weight ((3*i+phase) mod 11)-5. Add a -7 loop and an optional reversed parallel edge of weight 2. Also empty zero-vertex forest.

Forward: Boruvka signed edges become existing Kruskal/Prim unsigned edges by +7. Fixed spanning-forest cardinality r=V-components gives shifted optimum cost=original+7r. Compare exact costs, counts and components, not edge order. Rebuild the selected witness with existing DSU; reject cycles and compare all pairwise connectivity to the full graph.

Complement: wrong translated cost, cycles, missing connectivity or wrong r is excluded. Translation does not preserve arbitrary variable-cardinality subset objectives. Reverse: costs/components admit many graphs and tied forests; original graph plus selected indices recovers only the chosen witness. Invariant: same partition and exact sum. Cost: 192 bounded cases, native near O(E log E+V), witness O(V^2 alpha(V)); no large signed conversion claim.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave16-prufer-partition

Type: tree_code_transport.

Test: prufer_words_join_tree_degree_and_dsu_identity.

Domain: all n^(n-2) words for n=2..6: 1,3,16,125,1296 words; label range 0..n. Include n=0/1 conventions and literal star/path anchors.

Forward: decode, check n-1 canonical edges u<v<n, acyclic connected DSU, and degree(v)=1+word multiplicity(v). Encode back. Independently anchor star [1,1] to [(0,1),(1,2),(1,3)] and path [(0,1),(1,2),(2,3)] to [1,2].

Complement: incorrect size, cycle, multiplicity or roundtrip is excluded. Reverse: fixed-n labeled trees and codes are bijective under the declared deletion rule; degree observations alone are ambiguous. Empty output requires retained n to distinguish n=0/1. Invariant: tree partition and degree sum 2(n-1). Expected cost O(n log n) per native case plus DSU; enumeration is exponential and capped at six. Literal and degree evidence supplements inverse-only testing.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave16-clique-cover

Type: independent_witness_transport.

Test: complement_clique_joins_bipartite_cover_and_independent_witness.

Domain: all 512 simple 3-by-3 bipartite graphs. Left labels 0..3, right labels +3. Complement on all six vertices includes within-partition edges.

Forward: complement maximum clique is an original independent set; size=6-minimum_vertex_cover_size. Verify original edge coverage and independence for both clique and explicit cover-complement witness. Enumerate all 64 subsets to independently obtain lexicographically least maximum independent set and compare the entire clique.

Complement: bad +3 transport, missing within-partition complement edges, wrong size/nonindependence or wrong lexicographic choice is excluded. Reverse: sizes and even selected witnesses admit many graphs. Invariant: cover and complement partition labels, every edge touches cover. Cost: 512 bounded 64-subset scans plus native matching/clique; exponential clique cost remains outside this small domain.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave16-edit-lcs

Type: byte_metric_transport.

Test: bounded_edit_lcs_join_exact_lengths_and_metric_bounds.

Domain: 31 binary byte words of length 0..4, 961 ordered pairs; periodic byte pairs (63,66) and (64,67). Thresholds 0,1,2,3,4,64,usize::MAX. Coordinates are bytes, not Unicode characters.

Forward: banded result is Some(d) iff existing exact Levenshtein d<=cap, else None, both orientations. Bit LCS equals existing exact LCS length l; check max(n,m)-l<=d<=n+m-2l and abs(n-m)<=d. Substitutions prevent demanding equality to insertion/deletion distance.

Complement: wrong threshold result, asymmetry or failed bounds excluded. Reverse: (d,l,n,m) loses string contents. Invariant: accepted shorter LCS side<=64; zero/empty exactness. Errors: both sides 65 reject LCS; equal MAX_BAND_CELLS length at unlimited threshold requires one cell too many and errors; length-difference pruning returns None before allocation. Cost: reference O(nm), band O(max(n,m)*min(min(n,m)+1,2k+1)), bit LCS O(256+n+m). Allocation refusal untested.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave16-divisor-sieves

Type: inclusive_divisor_coordinates.

Test: totient_mobius_join_divisor_convolutions_gcd_and_primes.

Domain: inclusive 0..256 tables; phi0=mu0=0, phi1=mu1=1; divisor equations only n>=1.

Forward: sum phi(d)=n, sum mu(d)=[n=1], sum mu(d)*(n/d)=phi(n), each over divisors. Independently count gcd(k,n)=1 for k=1..n using existing gcd. Existing sieve membership iff n>=2 and phi(n)=n-1; primes have mu=-1, prime-square multiples mu=0.

Complement: incorrect conventions or equations excluded. Reverse: scalar phi/mu is many-to-one; preserve n/table coordinates, no unique input reconstruction. Invariant: inclusive indexing and signed Möbius contributions; no divisor loop at zero. Cost: bounded sieves plus conservative O(LIMIT^2 log LIMIT) oracle. Production 1,000,000-cap errors remain leaf obligations.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave16-fibonacci-recurrence

Type: modular_recurrence_identity.

Test: fibonacci_join_kitamasa_cassini_and_composite_zero.

Domain: m in [1,2,4,6,8,10,97,u64::MAX]; n=0..80 plus 2^32,2^63+7,u64::MAX. Cassini only n=1..80 avoids n+1 overflow.

Forward: fast doubling equals existing recurrence initial [0,1], coefficients [1,1]; small indices also equal independent additive loop. Cassini F(n-1)F(n+1)-F(n)^2=(-1)^n mod m uses u128 products and separately reduced terms. Literals F(12) mod8=0 and F(3) mod2=0 anchor zero behavior.

Complement: mismatch/residual error or accepting modulus zero excluded. Reverse: residues admit many n, m=1 collapses all; no field inverse assumed. Invariant: residues in 0..m, zero modulus rejected even n=0. Cost O(log(n+1)) at fixed order two. No arbitrary-recurrence claim.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave16-affine-dp

Type: chronological_predecessor_transport.

Test: li_chao_join_squared_cost_dp_and_earliest_predecessor.

Domain: six literal sequences, lengths 1..6, coordinates -4..4, repeated/unordered; penalties -3,0,2. dp0=0; conversions checked.

Forward: dp[i]=min_{j<i}(dp[j]+(x_i-x_j)^2+penalty). Insert slope -2*x_j, intercept dp[j]+x_j^2; query x_i then add x_i^2+penalty. Independent quadratic scan minimizes (cost,j), testing values and earliest ties. Insertion IDs equal chronological j.

Complement: future-state line, dropped term or wrong tied ID excluded. Reverse: dp/IDs lose coordinates and alternate predecessors, admit many histories. Invariant: query i sees precisely lines j<i; every queried coordinate registered. Cost native O(N log U), oracle O(N^2). No speed claim; other error domains remain leaf coverage.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave16-offline-partition

Type: timeline_partition_transport.

Test: offline_connectivity_join_rebuilt_dsu_and_literal_timeline.

Domain: n=1..6, eight phases, 48 deterministic edge toggles. Canonical undirected edge set, self-loops allowed; reverse remove orientation. Each toggle followed by Components and all ordered pair queries. Maximum operations 48*(2+6^2)=1824, below caps.

Forward: independently rebuild existing DSU from BTreeSet active edges at every step, compare answers in query order. Literal bridge deletion/loop/reconnect trace yields tagged observations [4,true,false,3,3,true]. Empty-universe Components=0.

Complement: stale deletion state, ordering mismatch, loop-induced merge or orientation dependence excluded. Reversed duplicate add and absent remove error. Reverse: query traces admit many edge histories; loops and temporary cycles may be invisible. Invariant: lifetimes alternate absent/present and query/state times agree. Oracle O(T*(E alpha(V)+V^2 alpha(V))); native rollback complexity remains its own contract. No concurrency/allocation simulation.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## algorithm-library-wave18-edge-block-path-join

Type: edge_partition_incidence_path_relation.

Domain is the literal undirected graph with six vertices and seven ordered edge identities: one triangle, one bridge, two parallel edges, one loop, and one isolate. Assert the exact canonical blocks [[0,1,2],[3],[4,5],[6]] and coverage of all IDs. Non-loop singleton blocks, explicitly excluding the loop, must equal bridges. Construct incidence nodes at vertex offset six from actual block membership, omit loop incidence, orient only newly visited incidences from vertex zero, and count on the resulting ten-node DAG. Expected counts [1,1,1,1,1,0,1,1,1,0] retain the original isolate and the omitted loop block node as zeros. A separate three-node DAG with duplicate 0->1 arcs must count [1,2,2]. Coordinates and original edge IDs must agree across modules. The orientation adapter builds a spanning traversal; these finite assertions do not independently prove every possible incidence graph is a forest. Cost is linear traversal plus BTreeSet membership construction O(E log(V+1)) and DAG ordering O(E+V log(V+1)) on these fixed tiny fixtures.

Source-derived partial relation. Wave18 bounded integration evidence is recorded in model.md#wave18-evidence; prior evidence is retained separately.

## algorithm-library-wave18-byte-codec-search-alignment

Type: byte_coordinate_traceback_relation.

Domain is six byte words (empty, a, b, aaabaaa, abababa and [0,255,0,0]), all 36 ordered pairs, lengths at most seven. Encode then decode with output cap equal to the original byte length; decoded bytes must equal the original. Feed decoded bytes to Two-Way matching and compare every overlapping original byte offset to literal slice equality; empty patterns match all boundaries 0..=length. Align decoded bytes against the other word under match 0, mismatch -1 and gap -1, with cap 1024 and at most 64 DP cells. Score must equal negative Levenshtein distance; projecting each traceback column reconstructs the respective original word, no column is (None,None), and independently summed column scores match. This relation uses bytes, not Unicode character positions, and equal optimal scores do not imply identical traceback choices. Per pair cost includes O(nm) alignment and distance, linear codec/search work plus output, and bounded literal-search comparisons; storage includes the DP table, runs, decoded bytes, offsets and traceback.

Source-derived partial relation. Wave18 bounded integration evidence is recorded in model.md#wave18-evidence; prior evidence is retained separately.

## algorithm-library-wave18-prime-factor-product

Type: prime_membership_factor_product_relation.

Domain is sieve interval [0,160] and every n=1..160, factorization budget 10000. Compare sieve output to independent literal trial divisibility over 2..n; require sorted factors, every factor in the verified prime list, product exactly n, and singleton factorization iff n is prime. The empty factorization of one has product one and is not prime. All factors and the entire product fit u64 in this bounded domain; no overflowing-product claim extends beyond it. Product reconstructs n but does not recover a factorization execution or consumed budget. Cost includes bounded segmented sieve construction, at most the supplied factorization budget per n plus its validation/primality work, and the deliberately quadratic-scale literal trial-divisibility oracle over the fixed bound; no factorization throughput or general budget-success theorem follows.

Source-derived partial relation. Wave18 bounded integration evidence is recorded in model.md#wave18-evidence; prior evidence is retained separately.

## algorithm-library-wave18-floor-wavelet-threshold

Type: horizontal_threshold_count_relation.

Domain comprises exactly 2275 tuples n=0..12, m=1..7, a,b=0..4. Materialize v_i=a*i+b for 0<=i<n, so 0<=v_i<=48. On the same full half-open range [0,n), use wavelet count_less with threshold k*m to count v_i>=k*m as n-count_less; sum for k=1..floor(60/m). Since every value is at most 48, this finite threshold cap includes every possible positive crossing and equals sum_i floor(v_i/m). Compare the u128 sum to floor_sum. Strict less-than is essential at exact multiples, n=0 gives zero, and m is never zero. Equal sums leave many progression tuples compatible. Per tuple cost is O(64n) wavelet construction, O(64*floor(60/m)) threshold queries, materialization O(n), and logarithmic Euclidean floor_sum work; the 2275 finite comparisons are not a general truncation guarantee for larger values.

Source-derived partial relation. Wave18 bounded integration evidence is recorded in model.md#wave18-evidence; prior evidence is retained separately.

## algorithm-library-wave18-persistent-wavelet-versions

Type: immutable_snapshot_order_statistic_relation.

Domain is five snapshots of length five: initial [3,1,3,0,5] and four point updates branching from versions 0,0,1,2. The returned version ID must equal the appended snapshot index; original versions remain queryable. Every value is nonnegative (0..9), so the i64-to-u64 cast preserves value; this adapter must not be generalized to negative values. For each of the five versions and all 21 half-open ranges [start,end), build a wavelet matrix from that exact snapshot, compare kth results with an independently sorted slice, sum all kth values in i128 and compare to persistent range_sum for the same version and endpoints. kth at rank equal to range length must be None, including empty ranges. Compare rank equality counts for values 0..10; version_count must be five. Order-statistic multisets and sums do not uniquely recover element order or branching history. Cost includes path-copy updates O(log 5), five O(64*5) wavelet constructions, all bounded range sorts and O(64) kth/rank queries, persistent O(log 5) sums, and retained snapshot/version storage.

Source-derived partial relation. Wave18 bounded integration evidence is recorded in model.md#wave18-evidence; prior evidence is retained separately.

## algorithm-library-wave19-quadratic-solvers

Type: diagonal_quadratic_derivative_relation.

Four diagonal systems at n=1..4, diagonal d_i=i+1, known minimizer i-1.5 and b_i=d_i*x_i. CG and diagonal GD coordinates, Horner derivative roots by bisection, and golden minimizers are within absolute 1e-6 of this independent minimizer. Derivative at CG result is within 1e-9. Scalar brackets [-4,4]; root budget100, golden200, CG100, GD10000. This bounded well-conditioned SPD/separable family does not imply nonlinear or ill-conditioned convergence. Cost includes O(n²) CG matvec per iteration, O(n) GD and bounded scalar callbacks.

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave19-polynomial-basis

Type: basis_transport_quadrature_relation.

Full c0 Chebyshev [2,3,-2,1] is identically monomial [4,0,-4,4]. For 33 dyadic points -1..1 compare both evaluations and independent literal polynomial/derivative to 1e-13 absolute. Integrate Clenshaw on [-1,1], compare to analytic 16/3 within 1e-12; Simpson budget1000 depth20 tol1e-10. Coefficient conventions and interval must match. Equal finitely sampled values do not identify arbitrary functions; basis transform is invertible only at fixed polynomial degree. O(33*4) evaluations plus bounded quadrature.

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave19-projection-threshold

Type: normal_cone_and_sign_reconstruction.

For x=[-3,1,-2,0], radius3 and threshold1, simplex of abs(x) must be [2,0,1,0], l1 projection and soft-threshold both [-2,0,-1,0]. These independent KKT values exclude a shared sign/threshold bug. Three small isotonic fixtures have preserved compensated sum, monotonicity, idempotence and normal-cone inequalities against ten constant feasible vectors -3..6. These inequalities are necessary observations, not a complete global optimality proof. Thresholded zeros admit intervals of antecedents; preserve original sign for nonzero reconstruction. Costs include sort at n4, PAVA and compensated sums.

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave19-merged-moments

Type: partitioned_centered_moment_relation.

Lengths1..24, values ((7*i)%11)-5, split at floor(n/2). Merge streaming halves; count equals n, mean and population/sample variances match compensated direct sums and squared deviations to absolute1e-12. n1 sample variance is not asserted. Centering and ddof must agree. Mean/variance leave order and generally many samples ambiguous. O(n) updates/direct oracle and O(1) merge with fixture storage O(n).

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave19-frequency-certificates

Type: additive_frequency_interval_relation.

Four supported tiers1,2,5,10 and deterministic 800-item stream key7 at every third position, else i%137. Both summaries total800. For keys0..140 compare inclusive bounds to independent BTreeMap frequency and verify interval width*100<=800*tier in exact integer arithmetic. This is additive frequency error normalized by total, not rare-key relative error. Counter tables and frequencies lose order. O(800*capacity) worst update costs plus exact map storage; normal BTreeMap allocation behavior remains categorical.

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave19-histogram-rank

Type: bin_value_rank_coordinate_relation.

257 values (17*i)%101, histogram inclusive0..100 width7, independently sorted ranks0..256. Every returned interval contains actual sorted value and requested rank; rank_start equals count below value_min and exclusive rank_end equals count<=value_max. Rank257 is absent. Value interval width and normalized rank error are distinct; no percentage claim. Histogram and ranks lose insertion order and within-bin positions. O(nlogn) sorting oracle plus bounded bin/query work.

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave19-fptas-subsets

Type: integer_objective_feasibility_fiber_relation.

n0..8 values (7*i+3)%13, weight=value, capacities0,1,7,19,40, supported tiers1,2,5,10. Enumerate all <=256 subset masks, retain attainable feasible sums and optimum. Both algorithms must return an attainable objective and objective*100 >= (100-tier)*OPT; zero optimum is therefore exact. Knapsack's absolute loss certificate also bounds actual loss and is <=tier*OPT. No selected-item witness is claimed by the API; bounded exhaustive membership proves existence here. Equal objectives leave multiple subsets compatible. Work caps2million; cost includes integer scaled DP/trimmed lists and exhaustive oracle.

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave19-rational-numeric

Type: integer_certificate_numeric_transport.

Affine samples (-4,-5),(0,3),(6,15), integer x=-4..6: rational interpolation equals exact 2x+3 by cross multiplication with zero curvature certificate and equals Horner. Simpson affine integral [-4,6] equals50 within1e-12. Quantized dot fixtures a=[101,-103,108,-100], b=[100s,100s,100,-100], s=-1,+1, steps1,2,4 and tier10: exact i128 dot equals compensated sum of exactly representable integer products, error lies within certificate and certificate*100<=10*sum|products|. Additional zero-L1 fixture has all-zero certificate. This metric is not error relative to cancelling dot; affine curvature premise is grounded by the known polynomial. O(n) validation/dot and bounded quadrature; rational output alone does not reconstruct knot set or original vector pairs.

Source-derived partial relation. Wave19 bounded integration evidence is recorded in model.md#wave19-evidence; prior evidence is retained separately.

## algorithm-library-wave20-factor-solves

Type: factorization_residual_transport.

A=[[4,1],[1,3]], b=[6,7]. Cholesky lower factor feeds lower then transposed-upper triangular solves. Gauss-Seidel and Jacobi independently solve original A,b, zero initial, tolerance1e-11 and budget1000. All coordinates equal literal [1,2] within1e-9, both stationary statuses Converged, reconstruction residual<=1e-12. Fixed positive-definite well-conditioned fixture; residual is not a universal solution-error certificate.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave20-qr-quadratic

Type: normal_equation_optimizer_transport.

Design [[1,0],[0,1],[1,1]], response [.25,.75,1], QR answer [.25,.75] within1e-12. Its normal equation Q=[[2,1],[1,2]], linear=[1.25,1.75] feeds box projected gradient and simplex Frank-Wolfe using objective .5*xQx-linear*x. Both within1e-4 of QR, budget10000/tol1e-9. Box [0,1]^2, simplex unit mass. Q strictly diagonally dominant and PSD. Equality on this unique minimizer does not certify arbitrary rank-deficient systems.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave20-scalar-search

Type: directional_derivative_step_relation.

Brent minimizes (x-2)^2 on [-4,5] tol1e-9 budget200; estimate within1e-6 of2. Armijo along x=4t uses value0=4, slope=-16, initial1, contraction.5,c=.1,budget30. Accepted step .5 maps to Brent minimizer; exact call count equals report, sufficient decrease independently checked. Callback pure except counted observations; no general global minimizer claim.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave20-lasso

Type: orthogonal_lasso_threshold_relation.

Identity triangular solve transports response [3,-2] into Lasso identity design, lambda.5,tol1e-10,budget1000. Independent scalar threshold oracle [2.5,-1.5] holds within1e-8 for objective .5||Ax-b||²+lambda||x||1. Zeroed coordinates would have interval preimages; this fixture is nonzero. Not a generic objective/solution accuracy bound.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave20-polynomial

Type: basis_transport_quadrature_relation.

Nodes (0,1),(.5,2),(1,4) and Bernstein controls [1,1.5,4] represent p=1+x+2x². At33 dyadic t in[0,1] compare each to literal polynomial within1e-12. Gauss order3 integrates barycentric callback on[0,1] to13/6 within1e-12. Basis transform at fixed degree is injective; arbitrary functions are not reconstructed from finite samples.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave20-monotone-quadrature

Type: shape_preserving_integral_transport.

Nodes (-1,-1),(0,2),(1,5) reproduce3x+2 at33 dyadic points in[-1,1], monotonicity and absolute1e-12. Romberg consumes cubic callback with tol1e-10, levels12,budget10000, statusConverged; both Romberg and Gauss order2 equal analytic integral4 within1e-10. Romberg successive differences are heuristic, not certified true integral error; nonaffine shape behavior needs separate leaf evidence.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave20-subset-knapsack

Type: integer_feasible_objective_fiber_relation.

n0..8, values(7i+3)%13, capacities0..32. Independent enumeration <=256 masks gives attainable sum set and feasible maximum. Bitset target decision equals set membership; meet-in-middle items weight=value, value=u64(value), max_states1024 equals exact maximum. Bitset max_words100. Value-only APIs do not reconstruct a unique mask; empty/zero cases preserve exact zero.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave20-structured-dp

Type: totally_monotone_coordinate_dp_transport.

n1..12, SMAWK squared distance matrix returns identity leftmost minima. Those returned coordinates define partition squared interval lengths and positive BST weights (coordinate mod4)+1. Compare partition for every1..n layer to independent cubic DP and BST to independent cubic root enumeration; callback budget10000, BST cells10000. Squared-distance matrix is Monge; convex interval costs have monotone leftmost splits; nonnegative successful-key weights satisfy Knuth conditions. Caller assumptions are not validated for arbitrary callbacks; minima/objectives retain many compatible matrices/costs/trees.

Source-derived partial relation. Wave20 bounded integration evidence is recorded in model.md#wave20-evidence; prior evidence is retained separately.

## algorithm-library-wave25-diagonal_stationarity_join

Type: diagonal_stationarity_join.

Join heavy-ball, Nesterov and BB reports on d=[1,3,9], b=[2,-3,4.5], initial zero, tolerance 1e-9 and cap1000. All Converged x approximate b/d within1e-8, independently recomputed Euclidean gradient norm matches report within1e-14. This projection forgets trajectories and iteration count; equal optima do not imply equal iteration histories or speed.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#diagonal_stationarity_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-lasso_soft_threshold_join

Type: lasso_soft_threshold_join.

For identity3 design and b=[3,-2,.25], lambda=.5, ISTA and FISTA converge to scalar soft-threshold oracle [2.5,-1.5,0]. Recompute .5||x-b||²+.5||x||1 and compare reported objective within1e-12. Identity admits this closed form; applying it to arbitrary correlated designs is excluded. KKT residual and coordinate error remain distinct metrics.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#lasso_soft_threshold_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-reduction_reachability_rejects_dominator_transfer

Type: reduction_reachability_rejects_dominator_transfer.

For all64 labelled increasing-edge DAGs on4 vertices, BFS reachability support is identical before/after reduction for each source; distances are deliberately not compared. Triangle 0->1,0->2,1->2 has dom(2)=[0,2], but reduced chain gives [0,1,2]. Thus reachability quotient has multiple dominance preimages and reduction-before-dominance is rejected.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#reduction_reachability_rejects_dominator_transfer.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-unit_flow_bfs_join

Type: unit_flow_bfs_join.

For all64 increasing-edge DAGs on4 vertices, assign capacity1/cost1 to each edge and limit1 from0 to3. If BFS distance exists flow.value=1 and cost=distance, otherwise both0. Per-edge capacities and per-vertex signed balances are checked. This equivalence depends on unit costs, one unit demand and admitted acyclic graph; negative cycles and general capacities/costs are not transferred.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#unit_flow_bfs_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-clique_order_core_coordinates_join

Type: clique_order_core_coordinates_join.

On complete simple graphs sizes1..8, smallest-ID tie MCS returns0..n and (n-1)-core is all vertices; n-core empty. Coordinates retained unchanged. Four-cycle also returns an MCS order and full2-core but has neither diagonal, so these shared observations do not certify chordality. Several nonisomorphic graphs can share order/core outputs.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#clique_order_core_coordinates_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-interpolation_polynomial_and_spline_affine_join

Type: interpolation_polynomial_and_spline_affine_join.

On nodes[-2,-.5,1,3], Neville and Newton match q^d for d0..3 at21 quarter-grid queries in domain. Natural spline agrees only on affine2q+1 using same nodes. Global cubic interpolation is not equated to natural spline for general degree3; natural boundary curvature constrains a different problem. Scalar queries have ambiguous polynomial preimages.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#interpolation_polynomial_and_spline_affine_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-quadrature_analytic_join

Type: quadrature_analytic_join.

Trapezoid tolerance1e-8/cap100000 and tanh-sinh tolerance1e-10/cap10000 integrate powers0..3 on[0,1]. Require EstimatedTolerance and compare independently to1/(d+1) with absolute tolerances2e-8 and1e-10. This finite observation provides no arbitrary callback accuracy bound; difference estimators miss unsampled spikes and tanh-sinh fixed-window tails.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#quadrature_analytic_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-fft_exact_field_fixture_join

Type: fft_exact_field_fixture_join.

For lengths1,3,17 and bounded integer coefficient patterns mod5/mod3, zero-pad to next power2; FFT negative-phase, complex pointwise multiplication, inverse normalization, then truncate to a.len+b.len-1. Compare against Karatsuba field product and independent double-loop integer convolution, all below field modulus and safely small f64 integers. Imaginary residual <=1e-8 and real error <=1e-8. Modular wrap and large coefficients are excluded from transfer.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#fft_exact_field_fixture_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-subset_triangular_reconstruction_rejects_or_join

Type: subset_triangular_reconstruction_rejects_or_join.

For length8 a[empty]=2, h=a*b disjoint subset convolution. Build matrix M[S,T]=a[S xor T] if T subset S else0; field solve uniquely recovers b and has empty nullspace since diagonal a[empty] invertible. Direct OR product for [0,1] with itself is[0,1], whereas disjoint convolution is[0,0]; ranked subset convolution cannot be replaced by ordinary zeta/OR multiplication. Zero empty coefficient removes this uniqueness premise.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#subset_triangular_reconstruction_rejects_or_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-discrete_log_target_join

Type: discrete_log_target_join.

Targets 3^e mod998244353 for e in{0,1,2,7,31} feed BSGS; recovered exponent equals e, modular re-power matches target and bounded earlier exponent scan finds no earlier solution. Explicit conventions0^0=1 and discrete_log(0,0)=1; base1,target2 has no solution. Modular exponent map has many preimages globally; the least-exponent contract selects one.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#discrete_log_target_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-bpe_codec_model_bound_join

Type: bpe_codec_model_bound_join.

Train BPE on repeated-byte fixture, require some token>255, serialize each u16 little-endian, LZ77/LZW roundtrip bytes, recover exact u16 sequence, then decode with the identical BPE model and output bound. Too-small output cap rejects; empty unrelated model rejects high tokens. Model identity is essential: a different compatible model may decode valid tokens differently, so this is not authentication. Endian-adapter mutant compiles and fails token equality.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#bpe_codec_model_bound_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave25-patience_myers_edit_observations_join

Type: patience_myers_edit_observations_join.

Four admitted byte pairs: patience patch reconstructs target at exact cap; Myers global distance agrees with scalar Levenshtein production DP, and number of non-Equal patience edits >= distance. abc->axc gives2 insert/delete edits vs Levenshtein1 substitution. Patch correctness does not imply minimality under a different edit operation set; width is within64 and byte rather than Unicode coordinates retained.

Declared observation scope: Bounded concrete composition observation. Source relation: algorithms/tests/composition_wave25.rs.groupant/model.md#patience_myers_edit_observations_join.

Source-derived partial relation. Wave25 integration evidence is pending in model.md#wave25-evidence; prior evidence is retained separately.

## algorithm-library-wave26-algorithms-tests-groupant_wave26_transforms-rs-orthogonal_composition

Type: orthogonal_composition.

Haar and DCT are real orthogonal maps with energy preservation; DCT(Haar(x)) inverse is Haar-inverse(DCT-inverse(y)), reversing order. Fixture amplitudes keep every intermediate in both API domains. Sum-of-squares is independent of transform recurrence; tolerances account for finite rounding.

Source relation: algorithms/tests/groupant_wave26_transforms.rs.groupant/model.md#orthogonal_composition. Native evidence remains in the source sidecar.

## algorithm-library-wave26-algorithms-tests-groupant_wave26_transforms-rs-spectral_composition

Type: spectral_composition.

Real input embeds as Complex(re=x,im=0). Goertzel's ordered repeated integer bins select Bluestein's negative-forward, unnormalized DFT entries. Independent direct sums for every selected k detect common sign/normalization mistakes; input n includes primes, powers and singleton.

Source relation: algorithms/tests/groupant_wave26_transforms.rs.groupant/model.md#spectral_composition. Native evidence remains in the source sidecar.

## algorithm-library-wave26-algorithms-tests-groupant_wave26_transforms-rs-smoothing_composition

Type: smoothing_composition.

Known degree0..4 polynomials at n=17 lie in the seven-point fitting space. Every boundary output is checked against analytic polynomial and independent unscaled target-centered Vandermonde QR. QR comparison alone is shared dependency, so analytic values remain authoritative anchor.

Source relation: algorithms/tests/groupant_wave26_transforms.rs.groupant/model.md#smoothing_composition. Native evidence remains in the source sidecar.

## algorithm-library-wave26-algorithms-tests-groupant_wave26_transforms-rs-oracle_independence

Type: oracle_independence.

Energy sums, trigonometric direct DFT and analytic polynomial values do not call production kernels to construct expected results. QR is a supplementary composition comparison, not the sole oracle.

Source relation: algorithms/tests/groupant_wave26_transforms.rs.groupant/model.md#oracle_independence. Native evidence remains in the source sidecar.

## algorithm-library-wave26-algorithms-tests-groupant_wave26_transforms-rs-mutation_sensitivity

Type: mutation_sensitivity.

Negating Goertzel terminal imaginary component compiles and fails named spectral composition assertion; unchanged original bytes are restored before all-three positive rerun. Compile failure alone does not count as detection.

Source relation: algorithms/tests/groupant_wave26_transforms.rs.groupant/model.md#mutation_sensitivity. Native evidence remains in the source sidecar.

## algorithm-library-wave26-algorithms-tests-groupant_wave26_transforms-rs-finite_scope

Type: finite_scope.

Finite deterministic fixture/tolerance coverage is not all-f64 verification, a performance claim or a 1/2/5/10% surrogate certificate. Allocation exhaustion is not injected; callbacks are absent. Independent oracle formulas also round in f64.

Source relation: algorithms/tests/groupant_wave26_transforms.rs.groupant/model.md#finite_scope. Native evidence remains in the source sidecar.

## tests

### Algorithm library evidence

#### Current 150-module suite

All 695 algorithm checks pass twice in debug and twice in release: 618 unit tests, 76 external checks and one doctest. All 69 qagen checks pass twice per profile: 56 library, three CLI and ten external checks. The live-model test lemma::tests::doubt_with_en_model is explicitly skipped in every qagen invocation, and PRAG overrides are cleared. Normalized named outcomes match across all four runs per package.

There are 91 algorithm compositions. [Wave20 evidence](model.md#wave20-evidence), [actual CPU observations](model.md#wave20-benchmark) and [final validation](model.md#wave20-validation) retain separate scopes.


#### Earlier implementation evidence

Before the original test-only iterator cleanups, two single-threaded release runs passed those 30 tests with identical normalized outcomes; process-inclusive durations were 56.714297 ms and 33.460187 ms. These earlier observations have a different fixture set and do not provide a speedup comparison with the expanded suite.

#### Current wave18 evidence

110 implementations and 110 test sources, the API boundary and eight external harnesses form 229 registered algorithm source leaves. File trees contain 1,402 views; the parent has 80. Fifteen internal and 60 external checks give 75 composition relations. Final root execution passes 550 checks twice per profile with identical outcomes. See model.md#wave18-evidence for scope, controls and receipts; bounded CPU comparisons are in the parent model.md#wave18-benchmark. Source registration and finite execution are not universal proof.

## edit-continuation

# Edit observation composition and retained state

## edit_score_append_composition

Domain: byte pattern p of length≤64, history t with |t|+1≤65536 and byte c.
Assumptions: ordinary completed calls, successful allocation for the generic
reference, valid slices, lawful byte equality. Both APIs observe the same
unit-cost global edit metric on this admitted domain; the new probe checks
selected continuations against an independent full-table oracle. General
source equivalence remains a specification-level claim, not a new proof.

Forward observation: Q_p(t)=(D(p,t),|t|). Reverse reconstruction: Q_p converse
retains every compatible history. Composing that converse with append-c and
distance yields `{D(p,t c) | Q_p(t)=(d,l)}`. This relation is not functional:
p=ab,d=l=1,c=b admits outputs 0 and 1 through histories a and b. Therefore a
parent cannot wire a cached scalar score into a unique exact continuation.
For this pattern even adding consumed length does not fix that loss. This is
an existential obstruction: empty/singleton patterns and one-class Eq types
retain scalar continuation laws. No universal failure for all patterns or
alphabets is claimed. Retain an appropriate
algorithm state or the history; no claim is made that a full row is minimal.

Behavior complement for a proposed scalar-update parent excludes any claimed
universal deterministic factorization through Q, because its single output
cannot equal both observations in the witness. A partial exclusion
`|new_score-old_score|≤1` remains valid on admitted continuations; it is weaker
than an exact value and is not a percentage approximant. The typed errors
must stay outside this numeric interval. Swapping pattern and text preserves
the metric but need not preserve the Myers error tag or admission.

Cost: rerunning the generic reference uses O(|p||t|) work and O(min lengths)
scratch; Myers uses O(256+|p|+|t|) work with fixed mask storage. These are
source-derived operation models, not measured timings. Keeping only Q reduces
state but loses the ability to continue exactly. Existing literal test-count
summaries are unchanged: this new external bounded harness is not another
crate test declaration. Source syntax projections remain unchanged.

Evidence status: source-derived argument plus finite actual-source probe,
128 ordered cap-grid calls and 1,278 equal-projection continuation comparisons;
176 separate after append. The 1,278 count includes 450 reflexive and 828
non-reflexive ordered comparisons; positive controls cover 90 empty/singleton
byte contexts and 16 one-class Eq cases. The fixed collision is an explicit rejected control.
See the leaf continuation models and evidence JSON for bound harness/output
hashes and reproduction. No production substitution, Lean proof, training,
GPU or language-quality result is inferred.

## wave14-evidence

### Wave14 bounded integration evidence

464 algorithm tests (416 unit, 47 external, one doc) and 101 selected Mova checks pass twice in debug and twice in release with identical normalized named outcomes. Mova checks comprise the previous 75 and 26 reached checks: eval 4, data 6, frames_md 7, tune 4 and five new joins. These selected Mova checks do not represent the complete Mova suite.

Ten new algorithm production mutations and eight false-oracle mutations fail named assertions in owner receipts. Four Mova production mutations fail named assertions. Both root adapters compile and fail their intended named assertions: zeta_product_inverts_to_or_convolution (wrong inverse stage) and batched_items_keep_source_identity_through_classification (wrong source window). Exact source bytes are restored before positive runs. Compilation failures do not count as semantic rejection.

Strict all-target algorithms Clippy and algorithms formatting pass. Historical world/prag warnings remain. No benchmark, throughput, GPU, teacher/parser acquisition, full-school run, held-out accuracy, persistent adoption or general proof is claimed. Tiny supplied-input fitting exercises actual classifier calls. Tuning validation participates in candidate choice and is not an untouched final test.

## wave16-evidence

### Wave16 bounded integration evidence

The complete 100-algorithm crate passes 504 checks (448 unit, 55 external composition, one doctest) twice in debug and twice in release. The 155 selected Mova checks also pass twice in each profile, with identical normalized named outcomes: 139 previous focused checks, 11 new unit checks, two existing fbench/silver tests and three new integration joins. This is a selected Mova suite, not full-system validation.

Ten new algorithm implementation mutants and two Mova production mutants fail named assertions in owner receipts, with exact restoration before positive runs. Root additionally compiles two adapter controls: wrong forest weight/cardinality correction and swapped one-based silver-row identity. Both fail their named assertions, then exact source bytes are restored. Compilation failure is not counted as semantic rejection.

Mova fixes bound duplicate-link credit by gold occurrences (multiset intersection) and retain context unless adjacent paragraphs share source/document identity and checked consecutive paragraph numbers. Old-code witnesses cover duplicate overcredit, cross-source suppression and overflow. Three root joins exercise notation/alignment/counts, row/item/class transport and batch/prompt coordinates. Unique-link semantics and public APIs remain unchanged.

Independent read-only review found no additional concrete defect. Coverage limits: graph/tree cap+1 checks do not establish maximum-size successful throughput; Li Chao scans an independent line envelope but shares widened arithmetic and value/ID ordering; the connectivity cap fixture does not stress worst-case interval storage. A 64-vertex clique representation cap does not make exponential search cheap.

The ten new implementations are exact within their declared domains; no new approximate Rust surrogate or Lean proof is asserted. No live parser/teacher/corpus acquisition, GPU, whole-school throughput or persistent-learning adoption was executed. No general semantic proof or training-speed gain follows from these checks.

#### Separate CPU measurements

The [string timing receipt](model.md#wave16-string-timings) records two bounded release measurements, their exact-output checks and source/binary/log hashes. These measurements are separate from native correctness and structural validation.

## wave16-string-timings

### Wave16 bounded CPU string timings

#### Observation and domain

Two executions of an external Rust benchmark compare exact specialized APIs with existing exact dynamic-programming APIs. Both runs check identical answers on all fixtures and retain identical result checksums in every timed pair. Inputs are supplied synthetic bytes.

Banded Levenshtein uses four equal-length pairs (384,512,384,512 bytes), threshold four and actual distance three; two fixtures substitute three bytes and two combine deletion, insertion and substitution. Every measured call returns Some(3); it does not approximate a larger distance. Bit-parallel LCS uses four pairs with shorter length64, longer length1024 and alphabets7,31,251,256; answers64,54,24,24. Its API rejects a shorter side above64.

#### Cost model before timing

Source models predict O(L min(S+1,2k+1)) band work instead of O(LS) full edit DP, and O(256+L+S) fixed-word LCS work instead of O(LS) scalar DP. Those formulas choose bounded fixtures; they do not predict machine-speed ratios. Allocation, initialization, branch costs and compiler decisions remain in timed calls. These measurements can calibrate a future dispatch model; no automatic substitution has been installed.

#### Method and results

Intel Core i5-13500, x86_64; ordinary shared-host conditions, no core pinning or frequency control. The Cargo release rlib is linked by rustc -O --edition=2024; exact compiler, CPU metadata and binary hashes are retained. Each execution uses three warmup rounds, then seven trials alternating specialized-first/reference-first. A band trial has24 repetitions of four fixtures (96 calls); an LCS trial has64 repetitions (256 calls). Inputs and outputs pass through black_box. No trial is discarded, including the slower first band trial. Below, each seven-trial median batch time is divided by its call count; ratios divide the two medians, not paired samples.

| Run | Comparison | Specialized us/call | DP us/call | DP/specialized |
|---|---|---:|---:|---:|
| 1 | banded_threshold4_vs_edit_dp | 8.942 | 206.765 | 23.12 |
| 1 | single_word_lcs_vs_lcs_dp | 0.670 | 73.999 | 110.50 |
| 2 | banded_threshold4_vs_edit_dp | 8.920 | 206.173 | 23.11 |
| 2 | single_word_lcs_vs_lcs_dp | 0.670 | 73.969 | 110.41 |

These are observed finite fixture timings, without confidence intervals. They do not establish whole-Mova, training, lesson, GPU, worst-case or other-domain gains. Error is zero for the checked outputs. Exact specialized domains and validation order remain unchanged.

## wave16-validation

### Wave16 final validation

The require-all gate exits 1 because 440 of 760 Rust files remain unregistered (357 public, 83 closed). This is the explicit backlog, not a passing whole-repository coverage result. Source binding and artifact structure do not prove semantic completeness. Native results remain in [wave16-evidence](model.md#wave16-evidence); bounded timings are separate.

~~~json
{
  "changed_rust_sources": 25,
  "groupants_repeat_identically": true,
  "receipts": [
    {
      "exit": 0,
      "file": "groupants-final-1.json",
      "mode": "groupants",
      "sha256": "39b6cdf84c456eae9142a342cf72b8e1aafea371581985813cb1e3dbef6e99b9"
    },
    {
      "exit": 0,
      "file": "groupants-final-2.json",
      "mode": "groupants",
      "sha256": "39b6cdf84c456eae9142a342cf72b8e1aafea371581985813cb1e3dbef6e99b9"
    },
    {
      "exit": 0,
      "file": "changed-final.json",
      "mode": "changed",
      "sha256": "5a9a11d4046b18144f944aaf1fb1e99900dd2978a3312165d331b55c81c8cca2"
    },
    {
      "exit": 1,
      "file": "require-all.json",
      "mode": "inventory",
      "sha256": "8915280898f82bff3577b8599481e6859af5afbf43a459b0f9299847f0fea635"
    }
  ],
  "require_all_expected_incomplete": true
}
~~~

## wave18-benchmark

### Wave18 bounded CPU measurements

Two process executions, each with three fixed warmups and seven alternating samples per method. Every timed batch checksum matches an independent answer computation; 120 normalized timing-record identities/checksums match across runs. Compilation is excluded. Durations include allocation, returned outputs and checksum work. Samples were not removed.

Search rows measure batches of 16 complete searches on 24,576 bytes with a 64-byte pattern. Matrix rows measure 1,024 kth queries over four repetitions of 256 long ranges on 8,192 u64 values. Query-only reuses an already built matrix; build-plus-query creates and drops four matrices. The direct baseline clones and sorts each queried range. These are specific baseline/workload comparisons, not all-input or whole-Mova claims.

| Fixture | Specialized median, run 1 / 2 (ms per batch) | Reference median, run 1 / 2 (ms per batch) | Reference / specialized, run 1 / 2 |
|---|---:|---:|---:|
| random_single_match | 0.394 / 0.214 | 0.372 / 0.203 | 0.94 / 0.95 |
| dense_ab_periodic | 0.420 / 0.420 | 0.464 / 0.460 | 1.10 / 1.10 |
| a_prefix_ending_b | 0.249 / 0.249 | 1.069 / 1.073 | 4.29 / 4.31 |
| random_mismatch | 0.266 / 0.265 | 0.209 / 0.210 | 0.79 / 0.79 |
| query_only | 0.835 / 0.807 | 39.717 / 39.799 | 47.54 / 49.34 |
| build_queries | 19.936 / 20.001 | 39.745 / 39.774 | 1.99 / 1.99 |

Model name:                              13th Gen Intel(R) Core(TM) i5-13500. Compiler: rustc 1.99.0 (b940084d7 2026-09-28). APIs linked from the workspace release library, external harness compiled with rustc -O. Raw runs retain all minima/maxima and samples; host scheduling and warm caches limit extrapolation. Ratios below one mean the specialized algorithm was slower. No GPU, training, school-throughput, approximation-budget or universal performance guarantee is inferred.

#### Prospective model retained

## wave18-evidence

### Wave18 algorithm evidence

The library contains 110 original deterministic algorithms. This epoch adds biconnected components, DAG path counts, Euclidean floor sums, segmented sieving, bounded Pollard rho factorization, Two-Way byte matching, Needleman–Wunsch alignment, canonical run-length coding, a wavelet matrix and a persistent segment tree. Every implementation and test source has an adjacent groupant. Standard library only; no imported problem statements or solution code.

Final root validation: 550 checks (489 unit, 60 external integration and one doctest) passed twice in debug and twice in release, with identical normalized named outcomes. Five new joins exercise all ten APIs; there are now 15 internal and 60 external composition checks. These relations retain edge IDs, bytes, arithmetic domains, half-open ranges and immutable versions.

Owners recorded ten compiled production mutations, each rejected by a named assertion and then restored. Root additionally changed the alignment adapter's mismatch score; its intended score assertion failed, then exact restoration passed. Compiler failures are not mutation rejection. Full root validation was repeated after three test-only Clippy refinements; owner receipts remain historical. Strict all-target algorithm Clippy and formatting pass.

Independent review found no concrete defect in the inspected domains. Finite oracles include 64,897 byte-search pairs, 14,400 alignment/path cases, 9,841 ternary RLE strings, cycle/path enumeration, direct floor sums, trial division and literal array snapshots. Large factorization checks partly reuse Miller–Rabin; small trial division and literal products provide separate evidence. Rho budget exhaustion is explicit. Alignment rejects any overflowing candidate score, including a losing candidate. Some allocation failures are outside typed error contracts. These are not universal proofs or new approximation-error guarantees.

CPU results in model.md#wave18-benchmark retain improvements and regressions from two executions. They do not establish Mova training, school or GPU throughput.

## wave18-validation

### Wave18 final validation

The require-all gate exits 1 because 431 of 784 Rust files remain unregistered (348 public, 83 closed). This explicit backlog is not semantic verification. [Native evidence](model.md#wave18-evidence) and [bounded timings](model.md#wave18-benchmark) retain their own scopes and limits.

~~~json
{
  "changed_rust_sources": 25,
  "groupants_repeat_identically": true,
  "receipts": [
    {
      "exit": 0,
      "file": "groupants-final-1.json",
      "mode": "groupants",
      "sha256": "a1762377cb03906cf22424307e975f3b5d4479486058cc02f1b9859a507c4deb"
    },
    {
      "exit": 0,
      "file": "groupants-final-2.json",
      "mode": "groupants",
      "sha256": "a1762377cb03906cf22424307e975f3b5d4479486058cc02f1b9859a507c4deb"
    },
    {
      "exit": 0,
      "file": "changed-final.json",
      "mode": "changed",
      "sha256": "6ce0de09a88041ede83dfc5ce32da6d4fcf305cbc8151ec0502c8a026a356963"
    },
    {
      "exit": 1,
      "file": "require-all.json",
      "mode": "inventory",
      "sha256": "576759bf997926cc5412e58ffc120d251d61140977e182dcfff800aa79be4008"
    }
  ],
  "require_all_expected_incomplete": true
}
~~~

## wave19-benchmark

### Wave19 bounded CPU measurements

Two executions, three untimed paired warmups, seven alternating measured samples per pair. Compilation excluded; fixed synthetic inputs. Horner evaluates value and derivative for 256 points and 16 coefficients, 200 repeats per sample. FPTAS compares 32 items at capacity100000, tier10%, three solves per sample. Candidate includes certificate checks and allocation. Horner baseline computes both outputs but omits finite validation.

| Kernel/run | Baseline median ns | Candidate median ns | Baseline/candidate |
|---|---:|---:|---:|
| horner/1 | 705168 | 1796802 | 0.39246 |
| knapsack_fptas/1 | 7061949 | 922175 | 7.65793 |
| horner/2 | 461842 | 1409968 | 0.32755 |
| knapsack_fptas/2 | 7084893 | 938534 | 7.54889 |

Knapsack returned 144,189 against exact 144,189 in both runs: zero observed loss, integer loss numerator 100,940 (divided by 100), and 166,304 counted DP cells/input. No fallback. The API returns no item witness; independent exhaustive feasibility fibers are checked on small integration fixtures. Horner maximum absolute value/derivative difference was 5.5511151231257827e-17 on 256 points. Horner was slower in both executions; first-run timings vary more. These observations do not establish universal speed, GPU benefit or Mova training throughput.

All observed CHECK lines match across executions. All seven samples, source and receipt hashes are retained externally; the full samples follow.

Run1 horner:

~~~text
SAMPLE horner 0 baseline_ns=895002 candidate_ns=1876550
SAMPLE horner 1 baseline_ns=894575 candidate_ns=1873483
SAMPLE horner 2 baseline_ns=959872 candidate_ns=1413947
SAMPLE horner 3 baseline_ns=463299 candidate_ns=1406529
SAMPLE horner 4 baseline_ns=465124 candidate_ns=1541455
SAMPLE horner 5 baseline_ns=655376 candidate_ns=1796802
SAMPLE horner 6 baseline_ns=705168 candidate_ns=2139512
~~~

Run1 knapsack_fptas:

~~~text
SAMPLE knapsack_fptas 0 baseline_ns=7050870 candidate_ns=922175
SAMPLE knapsack_fptas 1 baseline_ns=7064345 candidate_ns=924144
SAMPLE knapsack_fptas 2 baseline_ns=7076104 candidate_ns=939491
SAMPLE knapsack_fptas 3 baseline_ns=7061949 candidate_ns=920190
SAMPLE knapsack_fptas 4 baseline_ns=7059790 candidate_ns=949152
SAMPLE knapsack_fptas 5 baseline_ns=7037303 candidate_ns=902889
SAMPLE knapsack_fptas 6 baseline_ns=7075276 candidate_ns=905155
~~~

Run2 horner:

~~~text
SAMPLE horner 0 baseline_ns=461842 candidate_ns=1404718
SAMPLE horner 1 baseline_ns=460755 candidate_ns=1412457
SAMPLE horner 2 baseline_ns=462000 candidate_ns=1412974
SAMPLE horner 3 baseline_ns=461859 candidate_ns=1410069
SAMPLE horner 4 baseline_ns=460299 candidate_ns=1404017
SAMPLE horner 5 baseline_ns=460995 candidate_ns=1409968
SAMPLE horner 6 baseline_ns=462276 candidate_ns=1404618
~~~

Run2 knapsack_fptas:

~~~text
SAMPLE knapsack_fptas 0 baseline_ns=7094639 candidate_ns=948617
SAMPLE knapsack_fptas 1 baseline_ns=7084893 candidate_ns=919300
SAMPLE knapsack_fptas 2 baseline_ns=7121250 candidate_ns=938534
SAMPLE knapsack_fptas 3 baseline_ns=7070460 candidate_ns=918831
SAMPLE knapsack_fptas 4 baseline_ns=7072956 candidate_ns=939669
SAMPLE knapsack_fptas 5 baseline_ns=7079965 candidate_ns=898788
SAMPLE knapsack_fptas 6 baseline_ns=7099951 candidate_ns=961308
~~~

#### Prospective model retained

### Frozen benchmark fixture before measurements

Knapsack: 32 items, weight_i=500+(7919*i)%9000, value_i=100+(3571*i)%10000, capacity100000. Existing exact capacity DP vs knapsack_fptas tier10 with work_budget50million; three repetitions per sample. This API exposes objective but no subset witness; feasibility is covered by external small exhaustive integration fixtures, not inferred from objective<=OPT alone on benchmark. Check exact baseline stable, approximate objective<=OPT, value*100>=90*OPT, actual loss*100<=certificate.loss_numerator<=10*OPT. Zero OPT exact; no silent fallback. Record declared DP work and actual relative objective loss.

Each pair seven samples, even samples baseline then candidate, odd candidate then baseline, Instant elapsed after preflight with compilation excluded, black_box results. Print each duration and median/min/max; repeat executable twice and compare CHECK lines only, never timing. No expected speed ordering. Work is bounded <43million capacity DP cell visits across both runs per pair? Correction: 32*100001*3*7*2=134401344 baseline cell visits across two runs, plus preflight/check visits; this is a small synthetic CPU benchmark, not training/GPU/corpus work. Do not claim this arithmetic is an exact runtime operation count.
Root pre-measurement refinement: run three untimed paired warmups before seven alternating measured pairs. Compile outside measurements. Horner candidate includes finite checks; direct-monomial baseline does not. Both compute value and derivative. Report regressions as well as gains.

## wave19-evidence

### Wave19 integrated evidence

The library contains 130 modules. All 616 checks pass twice in debug and twice in release: 547 unit tests, 68 external integration tests and one doctest. Normalized named outcomes match across all four runs. There are 83 composition checks, fifteen internal and 68 external across nine harnesses. Eight new joins cover all twenty added modules with independently grounded finite identities.

Twenty implementation files and twenty test files have source-bound groupants; the new harness adds one source. Five family groups model solver, certificate, stream, projection and numerical interactions. The repository inventory is 394 registered sources, 21 groups, 327 local fragments, 742 nodes, 4162 typed views, 1282 type labels and 384 partial syntax projections. 1252 fragment views are already included in source-tree totals. Coverage is not complete semantics or proof.

Worker receipts record 21 compiled production mutants rejected by named assertions (five numerical, four in each other quartet). A surviving initial Simpson correction mutant motivated a stronger quartic fixture; the final correction mutant is rejected. Root adds two compiled adapter mutations: incorrect cubic basis coefficient and incorrect subset-to-knapsack weight embedding. Both fail named assertions; exact source restoration precedes the repeated positive suite. Stale transitive binding and wrong immediate-child controls on the new certified-approximation group are rejected and restored. Strict algorithms all-target Clippy passes.

Separate bounded CPU observations and all samples are in [wave19-benchmark](model.md#wave19-benchmark). No live teacher, corpus, model loading, full-school, GPU or whole-Mova training run is involved. No Lean or universal floating-point proof is claimed.

## wave19-validation

### Wave19 final structural validation

Require-all deliberately exits 1: 431 of 825 Rust files remain unregistered, comprising 348 public and 83 closed. The backlog is explicit; structural registration is not a semantic proof. [Native evidence](model.md#wave19-evidence) and [CPU observations](model.md#wave19-benchmark) have distinct scopes.

~~~json
{
  "changed_sources": 42,
  "format_pass": true,
  "groupants_identical": true,
  "receipts": [
    {
      "exit": 0,
      "file": "groupants-final-1.json",
      "mode": "groupants",
      "sha256": "fad7c14acd0b2cc3667b681ef5432dce073b6696627de9082d4590c4b6bd69e4"
    },
    {
      "exit": 0,
      "file": "groupants-final-2.json",
      "mode": "groupants",
      "sha256": "fad7c14acd0b2cc3667b681ef5432dce073b6696627de9082d4590c4b6bd69e4"
    },
    {
      "exit": 0,
      "file": "changed-final.json",
      "mode": "changed",
      "sha256": "189205230fcecfb97e702247ca4b4a4c6d68d3fd92941664d3740f2c6b4b640c"
    },
    {
      "exit": 1,
      "file": "require-all.json",
      "mode": "inventory",
      "sha256": "71dfe81935f2542a0c42921c7eb0cf1e7262714512d70d39978ef3ddf5f6465e"
    }
  ],
  "require_all_expected_incomplete": true
}
~~~

## wave20-benchmark

### Wave20 paired CPU measurements

Recorded compiler: rustc 1.99.0 (b940084d7 2026-09-28). Benchmark executables were compiled with Rust edition 2024 and optimization level -O; subset and qagen benchmarks linked the integrated release rlibs identified below. The hashes retain the measured epoch, before subsequent rebuilds.

Two sequential executions per runner; each fixture/baseline has three paired warmups and seven alternating-order samples. Table values are batch median microseconds (baseline / candidate), not per-call times. Ratio is baseline/candidate; below 1 means candidate slower. All samples retained; no retiming or run exclusion.

| Fixture / baseline | Run 1 baseline / candidate µs | Run 2 baseline / candidate µs | Ratios run 1 / run 2 |
|---|---:|---:|---:|
| subset/dense/production_witness | 6664.861 / 240.030 | 6675.864 / 239.625 | 27.766783 / 27.859631 |
| subset/dense/scalar_boolean | 5264.053 / 239.841 | 5308.260 / 237.489 | 21.948095 / 22.351604 |
| subset/tiny/production_witness | 77.922 / 45.830 | 44.700 / 25.085 | 1.700240 / 1.781941 |
| subset/tiny/scalar_boolean | 208.504 / 198.172 | 29.462 / 25.508 | 1.052137 / 1.155010 |
| subset/unreachable_even/production_witness | 17430.714 / 393.022 | 17451.856 / 395.971 | 44.350479 / 44.073571 |
| subset/unreachable_even/scalar_boolean | 12504.100 / 374.966 | 12637.806 / 374.924 | 33.347290 / 33.707647 |
| cover/n128 | 44410.267 / 43235.933 | 44394.041 / 43068.374 | 1.027161 / 1.030781 |
| cover/n32 | 12436.934 / 11836.093 | 12492.397 / 11681.220 | 1.050763 / 1.069443 |
| cover/n4 | 1656.645 / 1666.512 | 1660.964 / 1659.416 | 0.994079 / 1.000933 |
| horner/degree15 | 3782.043 / 1795.881 | 3781.339 / 1796.137 | 2.105954 / 2.105262 |
| horner/degree255 | 7988.950 / 7409.081 | 8004.083 / 7414.307 | 1.078265 / 1.079546 |

Verification: exactly 7 samples per 11 fixture/baseline groups per execution; sample indices, recomputed medians and printed ratios verified. Repeated CHECK records, sample repetition counts, Horner checksums and qagen callback counts match exactly across runs.

Observed regression: cover/n4 run 1 candidate 0.595601% slower (baseline/candidate 0.994079).

Qagen n = 4 run 2 is approximately flat (ratio 1.000933); the first-run small regression does not support a stable gain. No other fixture median regressed in these executions.

- Fixed CPU microbenchmarks only; no model, teacher, database, corpus, training or end-to-end Mova throughput measured.
- Scalar_boolean is an independent descending scalar decision DP; production_witness is existing subset_sum and reconstructs a witness when reachable. Both include capacity-sized allocation; packed candidate returns only a decision.
- All three subset decisions agree. Tiny timings vary substantially across runs, including candidate batches with the same input; preserve this variability and do not generalize tiny-case ratios.
- Qagen valid stub calls remain 17 → 17 and invalid anchors perform 0 calls. Cover removes a temporary filtered Vec; each HashSet uses fresh randomized seeds contributing timing variation. No reduced real-model call count or service throughput claim.
- The Horner prospective also proposed direct-monomial timing. This epoch measures the pre-change Horner baseline only; direct monomials remain a correctness oracle, with no new timing claim.
- Horner uses deterministic degree 15/255 arrays and 16 bounded x values. Successful value/derivative bits and batch checksums match. Rejected-input paths can do more work with a final finite gate and were not timed.
- Rounded numeric outputs and exact integer decisions are observed; timings establish neither universal speedup nor universal numerical accuracy.
- Ratios divide seven-sample batch medians, not medians of paired ratios. Both executions retained.
- Timestamp audit: benchmark logs span 22:15:04.652–08.268+0100; scoped Clippy log begins 22:15:20.576 and ends 22:15:28.780; structural controls files 22:15:45.876–47.368. These suspected team jobs did not overlap timings. No retiming performed.

Raw logs and source bindings (SHA256):

#### Prospective model retained

## wave20-evidence

### Wave20 integrated evidence

The library contains 150 modules. All 695 algorithm checks pass twice in debug and twice in release: 618 unit tests, 76 external checks and one doctest. All 69 qagen checks pass twice per profile: 56 library, three CLI and ten external checks. The live-model test lemma::tests::doubt_with_en_model is explicitly skipped in every qagen invocation, and PRAG overrides are cleared. Normalized named outcomes match across all four runs per package.

There are 91 algorithm compositions: fifteen internal plus 76 external across ten harnesses. Eight new algorithm joins cover the twenty additions. Three new qagen joins are counted separately.

Inventory: 440 of 867 Rust sources registered, leaving 427 unregistered. There are 801 groupant nodes (440 source roots, 25 groups, 336 local fragments), 4509 typed views, 1391 type labels and 430 partial syntax projections. The 1306 fragment views are included in source-tree totals. This wave adds 347 views, nine fragments and 46 source registrations: 40 new algorithm implementation/test files, four existing qagen files and two new harnesses. It adds 42 Rust files and changes 49. The algorithm subtree binds 311 sources (150 implementation/test pairs, its API boundary and ten harnesses), with 1865 file-tree views, 96 library-parent views and 45 family-group views; six historical executable-surrogate sources are registered elsewhere.

Worker evidence records 21 compiled new-algorithm mutants (six DP, five each in linear algebra, optimization and interpolation), plus nine compiled qagen mutants. Root adds one production Horner mutant and two compiled adapter mutants. Controls reject through named runtime assertions, and exact restoration precedes positive tests. Qagen also retains five old-code failing witnesses (four library and one CLI). Specific control sensitivity is not universal fault detection.

Both package-scoped checks use cargo clippy -p PACKAGE --all-targets --no-deps --offline -- -D warnings and return zero. Focused formatting checks 49 changed sources successfully; rustfmt-pre-final.log records that check. The broader qagen dependency check failed on existing en/build.rs and global/build.rs lints, retained in clippy-qagen-dependencies.log. Dependencies are not claimed clean. Final post-documentation structural/fmt receipts are recorded separately in model.md#wave20-validation.

[Actual CPU measurements](model.md#wave20-benchmark) retain the supplied report without invented performance numbers. [Final validation](model.md#wave20-validation) records later structural gates. No live teacher, model loading, corpus/database operation, full-school, GPU or whole-Mova training run is involved. No universal floating-point accuracy, semantic completeness, Lean proof or end-to-end throughput claim follows.

## wave20-validation

### Wave20 final structural validation

The algorithm parent resolves 311 actual transitive source bindings, totaling 1865 file-tree views. Membership follows manifest bindings and registry source paths rather than an ID prefix.

Require-all deliberately exits 1: 427 of 867 Rust sources remain unregistered. The refreshed public and closed backlog artifacts validate the split of 344 public and 83 closed sources; inventory independently validates the total. Structural registration is not a semantic proof. [Native evidence](model.md#wave20-evidence) and [CPU observations](model.md#wave20-benchmark) retain separate scopes.

~~~json
{
  "algorithm_file_tree_views": 1865,
  "algorithm_transitive_source_bindings": 311,
  "base": "5182886a19ee83e797b332eae0bbc8518252825f",
  "changed_sources": 49,
  "format_pass": true,
  "fragments": 336,
  "groupants_identical": true,
  "groups": 25,
  "nodes": 801,
  "partial_syntax_projections": 430,
  "receipts": [
    {
      "exit": 0,
      "file": "groupants-final-1.json",
      "mode": "groupants",
      "sha256": "ba900738642b787089a0ac697b52038cf2220e0833f4fc8bd70afd917869e50c"
    },
    {
      "exit": 0,
      "file": "groupants-final-2.json",
      "mode": "groupants",
      "sha256": "ba900738642b787089a0ac697b52038cf2220e0833f4fc8bd70afd917869e50c"
    },
    {
      "exit": 0,
      "file": "changed-final.json",
      "mode": "changed",
      "sha256": "6b0dedcbc78e72deda7006f754175805c62e00d65ce0944bb4babbd3b73e8f7c"
    },
    {
      "exit": 1,
      "file": "require-all.json",
      "mode": "inventory",
      "sha256": "ac8d5ceb561ac7a4834fc9472e116027ac7decc4e2b65d7a47229362f6255d01"
    }
  ],
  "registered_sources": 440,
  "require_all_expected_incomplete": true,
  "rust_sources": 867,
  "type_labels": 1391,
  "typed_views": 4509,
  "unregistered_sources": 427,
  "validated_backlog_split": {
    "closed": 83,
    "public": 344
  }
}
~~~

## wave25-benchmark

### Algorithm CPU measurements

Both executions have the same six nonempty CHECK records. Parsed and validated 112 timed pairs (8 shapes × 7 samples × 2 runs), plus 28 standalone Newton-construction samples. Every sequence retains its original alternating AB/BA order and repetition count. Reported medians/minima/maxima match the raw rounded samples. All eight before/after source, library, harness and binary hashes match; this parser also rechecked the bound artifacts. No source edits, retuning, reruns or sample exclusions were performed for this analysis.

The ratio B/A is the ratio of the two seven-sample medians in a run; it is **not** the median of paired ratios. Values above one favor A. All durations below are nanoseconds per operation; an interpolation operation is a batch of 128 queries, while the other operations are one distance or polynomial multiplication. Ranges retain every sample from both runs.

| Fixture (A versus B) | A median run 1 / run 2 | A all-sample range | B median run 1 / run 2 | B all-sample range | B/A run 1 / run 2 |
|---|---:|---:|---:|---:|---:|
| myers_vs_dp_p64_t64 | 272.369 / 187.607 | 187.562–610.959 | 4308.381 / 3915.957 | 3903.832–12771.316 | 15.818168 / 20.873146 |
| myers_vs_dp_p64_t1024 | 2730.750 / 2708.000 | 2706.297–2793.094 | 61511.438 / 61443.094 | 61381.016–61634.734 | 22.525474 / 22.689473 |
| cached_newton_vs_barycentric_n8_q128 | 1579.125 / 1581.625 | 1577.250–1592.000 | 6480.250 / 6477.000 | 6449.250–7192.000 | 4.103697 / 4.095155 |
| cold_newton_vs_barycentric_n8_q128 | 1676.375 / 1667.625 | 1661.875–1807.250 | 6482.250 / 6559.625 | 6430.000–7132.875 | 3.866826 / 3.933513 |
| cached_newton_vs_barycentric_n32_q128 | 7157.625 / 6871.500 | 6703.125–7351.000 | 75144.875 / 75159.000 | 75005.375–75891.500 | 10.498577 / 10.937787 |
| cold_newton_vs_barycentric_n32_q128 | 7537.750 / 7526.375 | 7496.875–7735.875 | 75118.750 / 75137.250 | 75013.250–76055.125 | 9.965673 / 9.983192 |
| karatsuba_vs_naive_n16 | 325.855 / 325.863 | 325.301–336.430 | 318.211 / 317.480 | 316.934–324.977 | 0.976540 / 0.974275 |
| karatsuba_vs_naive_n1024 | 257603.500 / 257755.000 | 256029.500–258601.500 | 1099836.000 / 1099875.000 | 1098899.500–1101073.500 | 4.269492 / 4.267134 |

Karatsuba at n=16 regressed: A/B−1 is 2.402% and 2.640% slower in the two runs. At n=1024 Karatsuba wins on these fixtures. The first 64-byte Myers run has substantial early-sample variation (both implementations), retained without exclusion; the second run is much tighter. These observations do not identify a causal explanation for the variability.

#### Newton setup and scope

| Construction | Run 1 median and range (ns) | Run 2 median and range (ns) |
|---|---:|---:|
| newton_construction_n8 | 65.891 (65.734–66.672) | 67.930 (66.617–75.867) |
| newton_construction_n32 | 793.086 (789.664–812.062) | 794.453 (785.992–798.094) |

Cached Newton excludes construction. Cold Newton includes constructing its two vectors and divided differences once per 128-query batch. The barycentric API validates nodes and recomputes weights on every query in both comparisons. A cached barycentric implementation was not measured; these results compare the current APIs under repeated reuse, not intrinsic superiority of interpolation formulas. Standalone setup cannot be subtracted from another noisy median to derive a reliable evaluation cost. Interpolation timings include the same rolling result checksum; convolution timings include the same output checksum, normalization and allocation. Fixture construction and oracle checks are excluded.

#### Output checks

- CHECK myers pattern=64 text=64 distance=5 exact_agreement=true
- CHECK myers pattern=64 text=1024 distance=960 exact_agreement=true
- CHECK interpolation n=8 queries=128 newton_max_abs=0.00000000000000000e0 bary_max_abs=8.88178419700125232e-16 newton_max_normalized=0.00000000000000000e0 bary_max_normalized=7.90864958063415888e-16
- CHECK interpolation n=32 queries=128 newton_max_abs=0.00000000000000000e0 bary_max_abs=4.43574066366636544e-10 newton_max_normalized=0.00000000000000000e0 bary_max_normalized=4.24306253114839638e-10
- CHECK karatsuba n=16 coefficients=31 checksum=16715221694875202277 exact_agreement=true
- CHECK karatsuba n=1024 coefficients=2047 checksum=2309886160783491685 exact_agreement=true

The affine interpolation oracle is y=2x+3. Maximum normalized error is |computed−expected|/max(1,|expected|); the finite fixture gate was 1e-8. Newton was exact on these dyadic affine fixtures; barycentric maximum absolute error at n=32 was about 4.44e-10. This is finite evidence, not a generic percentage guarantee or a claim about ill-conditioned/high-degree data. Myers agrees exactly with the production Levenshtein baseline. Both polynomial implementations agree coefficient-by-coefficient with an independent u128 modular convolution oracle.

#### Retained evidence and limitations

benchmark-algorithms-summary.json retains every raw pair and setup sample, all per-run summaries, CHECK lines and source/binary bindings. The original logs remain benchmark-algorithms-1.log and benchmark-algorithms-2.log. The prospective note and harness define three warmup calls per side, fixed repetition counts, black_box barriers and seven alternating pairs. These timings are selected release CPU kernels only, not training, GPU, held-out task performance or whole-Mova throughput. No model is automatically substituted into production.

The supplied logs contain complete terminal summaries and no failure records; they do not themselves encode benchmark process exit codes. The root execution receipt is authoritative for those exit statuses. The analysis process separately checks all assertions and writes a successful completion receipt only after both output files are written.

## wave25-evidence

### Wave25 finite evidence

The library now contains 175 modules: 25 new implementations with 25 separate test sources, plus a new 12-observation composition harness. Five family groups organize accelerated optimization, graph structure, interpolation/quadrature, algebraic transforms and byte sequences. This adds 51 source groupants and five groups. Current registration is 500 sources and 33 groups; the algorithm subtree binds 362 sources. There are 103 bounded composition checks: 15 internal and 88 across 11 external harnesses.

#### Checks

The complete algorithm package passed 782 checks twice in debug and twice in release, including its doctest, with identical normalized observations. The new source-local tests total 75: optimization 25, graphs 11, numerical methods 15, algebra 10 and sequences 14. The independent harness adds 12 checks, including rejected dominance-preserving reduction and OR-convolution substitutions. Strict package all-target Clippy passed; final formatting passed for all 54 changed Rust sources.

Root inspected 27 compiled assertion-failure receipts: one production mutant per new algorithm, the independent endian adapter, and conversion's first-error diagnostic. Each worker restored source bytes and reran positive checks; the integrated positive tests and source bindings confirm the retained code. Three additional spatial mutants belong to the rejected experiment and are excluded from that retained count. Two structural manifest controls and three copied-catalog controls separately check stale hashes and identities; they do not validate mathematical truth.

All 54 changed Rust sources have source hashes and byte-identical paired partial syntax extractions. These syntax projections omit semantic resolution and macro expansion. Forward relations, invariants, ambiguous preimages, behavior complements and specialized views remain scoped source models rather than general proofs. No new executable percentage certificate or Lean proof is claimed; quadrature estimators, solver residuals and finite floating-point checks retain distinct meanings.

#### Mova feedback

Conversion now uses functional-graph coloring for inputs above 64 tokens, preserving its own preflight and first failing head-chain start. It passed 19 focused checks plus two cross-gate observations twice per profile. Restored spatial inference passed 12 checks twice per profile, making 33 retained Mova checks. The original 34-check candidate run remains separately recorded.

The tree refinement reduces allocation overhead and improves selected chain timings by about 8.38–9.11x against fd540, including 9.01–9.04x at n65. Stars and tiny shapes include regressions; the n2048 star takes about 33–34% longer. The spatial lazy-index prediction failed on boundary product traversal (2.11–2.15 times parent latency), so that production change was rejected and its exact fd540 implementation restored. [Current transfer decisions](../mova.groupant/model.md#transfer-wave25) and the source-local reports retain every shape and rejected candidate revision.

#### Measurements and limits

[Algorithm measurements](model.md#wave25-benchmark) preserve 112 measured pairs, 28 setup samples, six identical output-check records and artifact digests. Myers agrees exactly with the edit-distance reference; modular polynomial products agree with an independent oracle. Interpolation evidence is limited to declared affine fixtures and current API caching behavior. Small Karatsuba regresses about 2.4–2.6%; the first short Myers execution has substantial variation. Mova candidate measurements preserve 840 pairs across both parent and legacy baselines; those include the rejected spatial experiment.

## wave25-validation

### Wave25 validation

#### Executed gates

- Algorithms: 782 checks twice debug and twice release, normalized outcomes identical. Includes 75 new source-local tests, 12 new composition observations and the library doctest. Strict package all-target Clippy passes.
- Retained Mova: 33 checks twice per profile (en conversion 19, spatial inference 12, cross-gate observations 2). Restored-world tests were rerun; unchanged en/annot results were reused. Candidate 34-check results are not substituted for retained results.
- Controls: root inspected 27 compiled assertion-failure receipts for retained new behavior. Two deliberately corrupted group manifests and three copied-catalog controls fail as intended; exact manifest restoration and a positive structural check pass.
- Source checks: all 54 changed Rust files pass formatting and have current paired identical partial syntax projections. All 54 are registered; changed-source missing list is empty.
- Groupants: two byte-identical reports pass with zero diagnostics; 500 source sidecars, 33 groups and 344 nested fragments (877 counted nodes including fragments). Registration describes scoped artifacts, not universal verification.
- Catalog: two byte-identical structural audits accept all five records. The original before-baseline remains historical; current conversion mechanism is coloring and retained spatial behavior is fd540.
- Inventory: 500 of 923 Rust files registered; 423 remain (340 public, 83 closed). The require-all gate exits 1 as expected. Coverage queues were regenerated from this exact registry.
- Timing: algorithm checks/112 pairs/28 setup samples agree with source and binary digests. The en/world experiment retains 840 pairs, including rejected spatial results. Gains and regressions are recorded rather than fitted away.

The whole-world Clippy invocation encounters pre-existing lint failures outside these edits; no workspace-wide clean lint or test result is claimed. No live teacher, model/corpus/database mutation, GPU, full-school or held-out run occurred. No universal numerical guarantee or Lean proof follows from these checks.

## wave26-evidence

### Wave26 evidence — completed UTC

Focused Mova checks pass 30 tests in each profile twice: conversion 19, synthetic CoNLL-U four, functional-graph/Doc/gate joins five and annotation/conversion joins two. These scopes reuse unchanged conversion receipts and the initial successful CoNLL-U debug run; every reused process had exit zero and its normalized names were checked. The real-treebank test was explicitly skipped. Strict all-target algorithm Clippy and changed-source rustfmt passed. Whole-workspace Clippy and live corpus, teacher, training, school, GPU and held-out evaluation were not run.

The final arborescence implementation fixes repeated completed-path traversal. On the independent nested-contraction family at V=257, primary parent advances decrease from 1,065,024 to 24,640 with the same objective. Each nonroot parent advances once per contraction; cycle labeling touches disjoint cycles. O(V+E) per round gives O(VE+V²) overall. Counts are not timings. Independent review also checked matching, exact cover, arborescence, functional graphs, HMM/covariance, ordered indexes and extreme coordinates; finite oracles do not establish universal correctness.

Thirty retained compiled controls are audited separately: 25 new kernels, the arborescence traversal repair, two CoNLL-U repairs and two composition adapters. Each intended assertion fails in compiled code and exact restoration passes. The rejected conversion candidate has its own checks and is not counted as adopted production. Structural negative controls reject a stale source binding and nonexistent child, restore exact bytes and pass again. Five existing transfer records remain source/model-bound; catalog validation is structural.

Models remain partial. Source hashes and syntax projections bind bytes, without macro expansion or semantic AST proof. Numeric residuals and estimators do not imply a universal 1/2/5/10% solution-error certificate. No aggregate Mova speed or quality gain follows from the library tests.

Receipt precision: all 30 compiled controls show named assertions and positive restorations. Numeric exit101 is recorded for 23; seven combinatorial/repair/harness runners retain non-success without the numeric code. No code is inferred for those seven; see controls-audit.json and model.md#wave26-validation.

## wave26-validation

### Wave26 coverage and structural validation

The registry contains 553 source leaves and 38 groups. Source manifests contain another 347 scoped fragments, yielding 938 counted nodes including fragments. Inventory is 975 Rust sources, with 422 missing (339 public and 83 closed). This epoch adds 52 Rust files and models one existing uncovered source, CoNLL-U. A larger algorithm library alone does not reduce the legacy backlog.

The algorithm-library subtree binds 413 files: 200 implementation/test pairs, the public boundary and 12 external composition harnesses. Six existing executable-surrogate files belong to the separate surrogate hierarchy, so the algorithms directory inventory is 419. Composition declarations total 106 (15 internal plus 91 external); these are already included in the 877 executed checks.

All 54 changed Rust sources are registered and source-bound. Groupant validation reports no diagnostics; all 38 groups have manifests and transitive bindings. New/changed source projections were generated twice identically and hashes checked at integration. Partial syntax lacks macro expansion and semantic AST analysis; ten older registered sources lack these projections. Counts are not proof coverage.

Two structural mutations corrupt a source hash and add an unknown child. Both are rejected, exact bytes restored and the positive gate passes again. The five-record transfer catalog passes structural audit and negative controls. Full-inventory enforcement is expected to fail because 422 files remain unmodeled.

The audit finds 30 compiled named assertion failures and positive restorations. Numeric exit101 is retained for 23; seven combinatorial/repair/harness receipts record non-success without the numeric exit code. No numeric code is inferred for those seven. Their runner guards and test failure logs demonstrate failure and restoration; this receipt limitation is explicit.

## History

### 2026-10-29 · tests

#### Twenty-algorithm suite, 2026-10-29

The library now contains twenty algorithms in separate directories. The focused debug and release runs pass **97 unit tests and one doctest**: the previous 54 tests, 38 new algorithm checks and five additional composition checks. Formatting and strict all-target Clippy pass. New implementations cover heap sort, quickselect, strict LIS, canonical SCCs, Kruskal forests, Floyd-Warshall, Fenwick sums, sparse-table minima, integer square root and modular power.

All ten new algorithms have independently reviewed finite oracles and a compiled semantic mutation rejected by an exactly selected single test. Numeric checks include an additional modular-accumulator overflow mutation. The integration also compiles a copy in which prefix(i+1) is replaced by prefix(i): the prefix reconstruction/sparse snapshot test runs once and rejects the resulting zero differences with exit 101. The production source is preserved.

The five new joins check ordering/rank/strict-length agreement; all-pairs distances and SCC mutual reachability; forest connectivity and selected-weight accounting; exact prefix-difference reconstruction followed by fresh range-minimum queries; and square/root inversion before modular reduction. Leaf oracles separately check properties that agreement alone cannot prove, especially minimum forest weight.

Two single-threaded release runs pass all 97 tests with identical output after excluding elapsed timing. Rust Instant process-inclusive durations were 118.349964 ms and 112.476861 ms. Parallel unit-test observations were 0.18 s debug and 0.02 s release, excluding compilation/doctests. These measurements describe this fixture suite and do not establish application speedups.

Twenty new source leaves and five additional parent models bring the algorithm branch to 41 source leaves plus its parent, with 216 typed models. Global structural reports repeat byte-identically and validate 68 file sidecars, 8 groups, 75 fragments and 1021 models; 505 legacy Rust sources remain without sidecars. Stale parent-source and wrong-child controls both reject with exit 1 and restore the original manifest. All 21 changed/new Rust sources pass the registration gate.

Run evidence: $MOVA_DATA/runs/octopus-algorithms-wave2-2026-10-29/. Per-algorithm groupants reference their own independent oracle and mutation evidence. The parent preserves full source/oracle identities and explicit inverse-domain assumptions; no unrestricted proof, school/inference integration or GPU improvement is inferred.

#### Expanded test suite, 2026-10-29

The focused crate passes **54 unit tests and one documentation test** in debug and release. All 30 earlier checks are retained; 24 new tests cover wider deterministic inputs and lawful transformations. Formatting and strict all-target Clippy pass. Production prefixes before the cfg(test) declaration are byte-identical to the preceding implementation; tests now live in ten adjacent tests.rs files.

New coverage includes duplicate-rich custom ordering, full-width GCD checked by a binary-GCD oracle, larger graph distance/order fixtures, multigraph and adjacency transformations, 5,400 mixed DSU operations and depth-six compression, 2,564 KMP oracle cases plus padding relations, Unicode edit cases and 2,197 distance triangles, and 800 larger knapsack subset comparisons. Exact finite domains and assumptions are in the per-test groupants.

Ten compiled source mutations, one per algorithm, each fail a specifically selected NEW test with exit 101. Every filtered run executes exactly one test; independent review inspected the failures and reran the DSU mutant. Compilation errors and empty filtered runs are not counted as successful controls. Production worktree sources were not mutated.

Two single-threaded release suite runs pass all 54 tests with identical normalized output, excluding the elapsed-time footer. Rust Instant process-inclusive observations were 79.298375 ms and 95.420246 ms. The ordinary parallel test runner reported 0.18 s debug and 0.03 s release for unit tests; these exclude compilation and doctest compilation. These are fixture-specific observations, not benchmark or speedup conclusions.

All twenty changed/added implementation and test sources are registered. Structural reports repeat byte-identically and remain valid: 48 file sidecars, 8 inter-file groups, 75 fragments, 916 typed models; 505 older Rust sources remain unmodeled. Deliberately stale parent source hashes and wrong child identities both return exit 1, and the original manifest is restored. The algorithm branch now binds 21 source files and 111 typed models including its six parent models.

Evidence is stored outside the repository in $MOVA_DATA/runs/octopus-algorithm-tests-2026-10-29/ and the worker run directories named by each test sidecar. Source identities bind implementations and oracles together; partial syntax does not expand external test modules. Finite tests, source mutation sensitivity and structural validation do not establish universal semantics.

The initial 2026-10-29 crate passed 30 unit tests and one doctest in both profiles. Its three composition tests cover original-multiset rank after sorting, BFS versus unit-weight Dijkstra, and DSU versus undirected reachability. A compiled adapter mutation replacing weights one with two failed the exact distance join with exit 101: [0,2] versus [0,1].

Initial structural reports validated 38 file sidecars, 8 groups, 75 fragments and 866 models; the initial algorithm branch comprised eleven source leaves and one parent with 61 models. Eleven new Rust sources passed the registration gate. These records remain in $MOVA_DATA/runs/octopus-algorithms-2026-10-29/. The later directory move preserved all source bytes and validated the new paths.

### 2026-10-30 · document

#### Model correspondence epoch 2026-10-30

Six graph/string/range implementations now each have eight typed views and five test-source views. Eighteen further leaf tests exercise certificates, representable preimage witnesses, joint feasibility, strict/injective transforms and snapshot/update laws against production APIs. The original fifteen cross-algorithm joins remain. Two parent views distinguish certificate/observation projections and simultaneous preimage intersections. These are 315 leaf views plus 20 parent views across the same 61 sources, with no change to exact algorithm implementations or APIs.

#### Historical wave14 integration scope, 2026-10-30

The wave14 harness adds nine external joins to the prior 53 compositions. The ownership tree covers ninety algorithms and 187 source leaves, with 1,145 leaf views and 67 parent views. Fifteen internal checks and 47 external checks across six harnesses give 62 composition relations. Detailed adapter relations remain in the specialized catalog and harness groupants. They preserve graph vertices, complete strict preferences, byte offsets, original indices, modular rings and each leaf's validation and resource constraints. No execution runs all leaves as one pipeline.

Current root evidence is 416 unit tests, 47 external integration checks and one doctest (464 native checks), passing twice in debug and twice in release with identical normalized named outcomes; strict all-target Clippy and formatting pass. Ten worker production controls, eight false-oracle controls and the root wrong-inverse OR-convolution adapter reject named assertions; exact restoration and positives pass. Earlier historical epochs retain their corresponding source and test totals; model.md#tests records the current evidence separately. No benchmark gain or universal proof follows.

#### Historical wave16 integration scope, 2026-10-30

The library has one hundred algorithms, 208 bound source leaves, 1,259 leaf views and 75 parent views. Eight composition_wave16 joins extend the prior 62 to 70: fifteen internal and 55 external across seven harnesses. They retain signed forest cardinality under weight shifts, labeled-tree identity, graph complements, byte-string bounds, arithmetic identities, modular recurrence coordinates, DP predecessor ties and connectivity timeline order. Leaf resource caps and error domains still govern each reached call; exact maximum clique search remains exponential.

The integrated suite passes 504 algorithm native checks twice in debug and twice in release with identical normalized named outcomes; strict all-target Clippy and formatting pass. See [wave16-evidence](model.md#wave16-evidence). Prior wave14 counts above remain historical evidence. The separately passing 155-check selected Mova suite is not part of the algorithm total. Source registration and cross-algorithm agreement do not establish general correctness; separate bounded CPU string timings imply no full-Mova or training gain.

### 2026-10-30 · tests

Heading `#### Current 150-module suite, 2026-10-30` stays in tests as `#### Current 150-module suite`.

#### Historical 130-module suite, 2026-10-30

All 616 native checks pass twice per profile, with 83 selected composition checks. [Wave19 evidence](model.md#wave19-evidence) binds repeated outcomes, 21 leaf production controls, two adapter controls and structural rejection tests. [Measurements](model.md#wave19-benchmark) preserve both FPTAS gains and Horner regressions.

#### Historical one-hundred-algorithm suite, 2026-10-30

Wave16 binds 208 source leaves: one hundred implementations, one hundred test modules, the public boundary and seven external harnesses. They contain 1,259 leaf views; the parent declares 75 views. Seventy composition relations comprise fifteen internal and 55 external joins. Counts of sources, declared relations and executed tests remain distinct.

The integrated suite passes 504 native algorithm checks twice in debug and twice in release with identical normalized named outcomes. Strict all-target Clippy and formatting pass; [wave16-evidence](model.md#wave16-evidence) retains the final receipts. Eight new external joins cover signed forests, labeled trees, complement cliques, byte edit/LCS bounds, divisor identities, modular Fibonacci, Li Chao DP and offline connectivity. Their finite fixtures preserve the exact adapters and leaf domains.

Worker receipts retain bounded independent oracles and compiled mutation failures with exact restoration. Final integrated mutation, Clippy, formatting and structural outcomes are retained in [wave16-evidence](model.md#wave16-evidence). The 155-check selected Mova suite also passes twice per profile with identical normalized named outcomes and remains separate from the algorithm suite. Bounded CPU string microbenchmark observations do not imply full-Mova or training gains; no approximate-error budget or universal semantic proof is inferred.

#### Historical ninety-algorithm suite, 2026-10-30

The full suite passes 416 unit tests, 47 external checks and one doctest (464 native checks), twice in debug and twice in release with identical normalized named outcomes. Strict all-target algorithms Clippy and formatting pass. The branch binds 187 sources: ninety implementations, ninety test modules, the public library boundary and six external harnesses. It declares 1,145 leaf views and 67 parent views. Its 62 composition relations comprise fifteen internal and 47 external checks; these source, view and execution counts are distinct.

The ten new algorithms add 34 unit tests; composition_wave14 adds nine public-API joins. They preserve cover/matching/flow cardinality, common-preference permutation identities, prime/root consistency, byte matches, earliest cyclic rotations, Cartesian LCA/range minima, inversion/sorting laws, normalized subset inversion and OR convolution. Finite agreement does not enlarge any leaf's accepted domain.

Worker receipts record ten compiled production mutations and eight false-oracle controls rejected by named assertions. The root wrong-inverse adapter replaces subset Möbius with a second subset zeta transform in zeta_product_inverts_to_or_convolution. It compiles and fails the named equality assertion; exact source restoration and positive checks pass. Root progress, Clippy and adapter logs retain the outcomes; model.md#wave14-evidence carries final source/evidence bindings separately.

Primality retains its fixed-witness theorem premise, Tonelli-Shanks its conservative ascending nonresidue-search cost, and subset inversion its normalized-residue observation. Allocation failures and lawful-trait violations are not uniformly injected. No wave14 benchmark, approximation budget, GPU result, training gain or general semantic proof is claimed. The following sections preserve older source snapshots and their dated measurements.

#### Historical eighty-algorithm suite, 2026-10-30

The full suite passes 382 unit tests, 38 external integration tests and one doctest twice in debug and twice in release, with identical normalized named outcomes. Strict all-target algorithms Clippy and crate formatting pass. The ten added implementation/test pairs supply 130 typed views; the eight-join harness adds ten views and the parent adds eight. The library now binds 166 source leaves, with 1030 leaf views and 58 parent views. Its 53 compositions comprise fifteen internal and 38 external checks.

Twenty compiled implementation/oracle controls across the new leaves reject named assertions. The root separately changes the no-carry convolution adapter from b[4*i] to b[3*i]: the compiled join fails its intended equality assertion, and exact restored source passes every positive repeat. Independent reviews checked graph traversal/certificates, finite edit-state oracles, recurrence arithmetic, optimization witnesses and the root text joins. Aggregate palindrome agreement does not compare each individual count or longest-tie identity. Finite recurrence agreement does not identify arbitrary unseen continuations.

Prospective leaf cost models were calibrated with bounded native release observations. Dinic beat Edmonds-Karp on the supplied 256/1024 disjoint-lane graphs, while one-edge examples lost. Hopcroft-Karp gained on 256/4096 independent pairs, while tiny inputs lost. FWHT lost against naive XOR convolution at dense lengths 2 and 8 and won at 32, 128 and 512 in its three recorded rounds. These are shape-, size- and machine-specific observations; leaf descriptants and evidence retain ranges and hashes. They establish no universal gain, approximation profile, GPU result or school throughput increase.

Repeated structural reports are identical; all 26 changed Rust sources are registered. Stale-binding and wrong-child controls reject and restore. Source syntax is a repeated partial projection, not an expanded semantic AST. Owner receipts and the root octopus-wave11-integration-2026-10-30 receipt set retain the exact runs. Earlier sections describe historical source snapshots.

#### Seventy-algorithm suite, 2026-10-30

The root suite passes 341 unit tests, 30 external integration tests and one doctest twice in debug and twice in release with identical normalized named outcomes. Strict all-target algorithm Clippy and crate formatting pass. Ten new implementations and ten independent test modules add 127 views; the eight-join harness adds ten, and the parent adds eight. The branch binds 145 source leaves with 890 leaf views and 50 parent views. There are 45 compositions: fifteen internal and thirty external.

Twenty compiled implementation/test-oracle controls across the new leaves fail named assertions. A separate compiled recurrence-denominator sign adapter fails its intended join assertion; restored source passes the complete suite. Independent review checked graph rejection domains, witness identity, recurrence/NTT arithmetic and DP overflow/memory. Parallel-edge forest identity is covered by leaf tests rather than the new parent fixture.

Five release timing batches compare NTT to schoolbook on equal-length operands. Median nanoseconds (NTT/schoolbook):16=7006/602;128=24345/23031;512=54594/176265;2048=242709/2661481. This supports the prospective small-input overhead/crossover model at these supplied sizes only. No universal speedup, school quality, GPU gain or new approximation bound is inferred. The exact APIs retain resource caps, canonical byte/BWT interpretation, checked arithmetic and explicit error ordering.

Receipts: external octopus-wave10-integration-2026-10-30 run and each adjacent owner evidence file. The following sections retain earlier source snapshots.

#### Sixty-algorithm suite, 2026-10-30

Integrated root checks pass **298 unit tests, 22 external integration tests and one doctest**, twice in debug and twice in release with identical normalized outcomes. Strict all-target algorithm Clippy and formatting pass. The ten additions are A*, two-SAT, Stoer-Wagner cut, Rabin-Karp, Boyer-Moore-Horspool, longest common substring, optimal string alignment, subset sum, binomial coefficient and interval union. They add 47 unit tests; the new external harness adds eight checks to the prior fourteen.

The branch now binds **124 source leaves**: sixty implementations, sixty test modules, one library boundary and three external harnesses. Its **37 compositions** comprise fifteen internal and 22 external checks. Leaf models preserve independent oracle assumptions; parent joins preserve the actual adapters and compatible-input constraints. Source and registry bindings are refreshed by integration. These counts do not imply unrestricted correctness.

Ten compiled production mutants, one per new algorithm, fail selected leaf assertions. A separate compiled knapsack-adapter mutant fails its selected integration assertion. The controls distinguish leaf semantics from adapter wiring; compilation failures and empty filtered runs do not count. Current run receipts are under `$MOVA_DATA/runs/octopus-wave7-integration-2026-10-30/`, with the composition and graph/string/discrete owner evidence in their named wave7 directories.

The domain boundaries remain explicit: A* validates global consistency and h(goal)=0; minimum cut returns a checked u128 weight and proper zero-oriented witness; rolling-hash equality requires byte verification and empty patterns match every boundary; OSA is restricted and nonmetric; subset targets above 1,000,000 reject before allocation. No new approximate profile, GPU result, throughput comparison or school/inference quality gain is inferred.

The following sections are historical measurement epochs; their smaller counts describe the corresponding source snapshots.

#### Thirty-algorithm suite, 2026-10-30

The library contains thirty exact algorithms. Debug and release each pass **137 unit tests and one doctest**: the prior 97 tests plus 33 new leaf checks and seven composition checks. Formatting and strict all-target Clippy pass. The new modules are Bellman-Ford, bipartite matching, Z prefix matching, Manacher palindrome radii, segment tree, sliding-window minimum, counting sort, radix sort, extended GCD and the inclusive prime sieve.

Seven new compositions check signed/nonnegative distance embeddings; absence of augmenting paths via BFS on the matching residual graph; sentinel-separated Z matches versus KMP; palindrome spans versus reversed edit distance; current tree/sparse/window minima; integer/generic sorting and ranks; and prime Bezout inverses versus modular exponentiation. Their exact domains, costs, complements and ambiguous preimages are in model.md#catalog. Agreement is supplementary to independent leaf oracles.

Each new algorithm has a compiled external source mutant rejected by a selected leaf assertion (ten controls total). A separate compiled range-adapter mutant changes only tree.query_index(start,end) to tree.query_index(start,end-1); the intended snapshot equality assertion rejects it with exit 101. An initial control changed the shared oracle range instead and hit an empty-range unwrap, so it was discarded as evidence and replaced by the adapter-only mutation. Production sources remain unchanged.

Two serial release-binary runs pass all 137 unit tests with identical normalized result records. Process times were 206.57 and 198.25 ms on this shared CPU; compilation and doctests are excluded. The ordinary parallel test harness reported 0.22 s debug and 0.02 s release in its first integrated runs. These differ in scheduling/scope and are not an algorithm-speedup comparison. Per-algorithm operation throughput, GPU throughput and school performance were not measured.

Twenty new source leaves and seven parent models bring the algorithm branch to **61 source leaves and 291 typed models**, including 18 parent models. The current inventory validates 100 file sidecars, 9 groups, 75 fragments and 1131 models; 504 legacy Rust files remain without sidecars. All 21 new/changed Rust sources pass the registration gate. Deliberately stale parent-source hashes and wrong child identities are rejected, and the manifest is restored. Two final structural inventories and two API syntax projections are byte-identical.

Independent review corrected the empty-left matching cost, explicit palindrome reverse-radius bounds, and cubic upper bounds for number-theory test oracles. All ten implementations and the seven new joins were reviewed. Allocator-exhaustion paths are not fault-injected; generic trait panics remain outside error-atomicity contracts. Finite checks, source projections and metadata validation do not prove universal correctness.

#### Deepened correspondence, 2026-10-30

The focused library passes 155 unit tests and one doctest in debug/release, adding 18 tests for six existing algorithms. Each pair repeats deterministic outcomes and syntax; six compiled production mutants are rejected by new tests. Graph certificates, constructive string constraints and range snapshot/order laws have explicit premises and incomplete-preimage limits. Strict algorithm Clippy passes. The library has 315 leaf models and 20 parent models; none of these counts implies universal verification.

#### Sorting and number relations, 2026-10-30

The integrated suite passes 167 unit tests and one doctest in debug and release. Twelve new tests cover counting/radix sort, extended GCD and sieve, bringing these four families to eight implementation models and five test models each. Native leaf runs repeat with identical normalized outcomes; eight compiled production mutants are rejected across sorting and number families. A minimum-only countermodel survives monotone-transform agreement but is rejected by histogram conservation, recording a concrete limitation of the former oracle. Strict all-target algorithm Clippy passes. The branch now contains 335 leaf views plus 20 parent views. Exact domains and evidence remain in each sidecar; this batch adds no algorithm/API or throughput claim.

#### Forty-algorithm suite, 2026-10-30

The integrated library passes 214 unit tests, seven external integration tests and one doctest in debug/release: 47 new leaf tests and seven new joins extend the prior 167 unit tests. Strict all-target Clippy and crate formatting pass. The new algorithms are zero-one BFS, DAG shortest paths, Aho-Corasick, suffix array, LCS, weighted interval scheduling, convex hull, closest pair, prime factorization and generalized CRT. Each has eight implementation and five test relations, source-bound syntax and one compiled production mutant rejected by an independent leaf check.

Two single-threaded release-binary repeats preserve all named outcomes. Process-inclusive observations are 328.68/495.37 ms for the 214-test unit binary and 22.36/17.92 ms for the seven-join binary, on a shared CPU. Compilation and doctests are excluded. These measurements have no matched baseline and imply no speedup. The ordinary parallel harness reported 0.28 s debug and 0.04 s release for unit tests in the first integrated runs.

The separate compiled adapter mutant changes only Aho observation start to start+1; the selected multi-index search test rejects at its equality assertion with exit 101. This supplies an integration control separate from the ten leaf mutations. The successful production source remains unchanged. Graph cycle domains, empty suffixes, interval ties, CRT overflow ordering and hull information loss are explicitly modeled.

The branch binds 82 source leaves with 474 leaf models and 27 parent views. Global inventory: 123 file sidecars, 9 groups, 75 fragments and 1361 models; 504 legacy files remain uncovered. Twice-identical syntax and inventory reports, stale-binding/wrong-child controls, changed-source registration and staged/range secret guards accompany integration. Raw evidence: $MOVA_DATA/runs/octopus-algorithms-wave4-2026-10-30/ and the five wave4 worker runs. No live parser/teacher/training, GPU, new Lean proof or numerical approximation substitution is introduced.

#### Fifty-algorithm suite, 2026-10-30

Integrated debug/release checks pass 251 unit tests, fourteen external joins and one doctest. The fifth batch adds 37 leaf tests and seven joins. Every new implementation has eight typed models and its test file five; twenty new leaf sources plus the new harness are registered. The library subtree binds 103 sources with 613 leaf models and 34 parent views.

Ten compiled leaf production mutants fail independent assertions; a separate compiled flow-offset adapter mutant fails its selected join. Initial and final integrated unit outcomes are compared after sorting test names; owner debug/release repetitions and source projections are deterministic. Strict all-target algorithm Clippy and crate formatting pass. Matrix overflow handling is tested with one overflowing candidate and a representable optimum exactly u128::MAX. Geometry/training/GPU behavior is not inferred. Raw evidence: $MOVA_DATA/runs/octopus-algorithms-wave5-2026-10-30/ and wave5 worker directories.

### 2026-10-30 · wave14-evidence

Heading `### Wave14 bounded integration evidence, 2026-10-30` stays in wave14-evidence as `### Wave14 bounded integration evidence`.

Raw receipts: $MOVA_DATA/runs/octopus-wave14-integration-2026-10-30. native-evidence-hashes.json SHA-256 d95a91417adeb7f8e755d26f900b6508ea17759291407f33153917a42396280f; algorithm-adapter-mutant.log d2279a28cc474377b1d9321e15d4c779b0af8507f81ac39e83338bc9084a02a2; mova-adapter-mutant.log 60567fa4d984ac58834be268cdb221a37caf30bd5d07e9a3d6a2d89c16fdac87; mova-all-normalized.txt fb492ea8097fd74fe50cdceb5d61cb49efc74baa6cb152f7699e546dcbb3ac20; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. The structural controls reject stale parent-source bindings and wrong child identities, restoring exact bytes. Source syntax remains partial, with opaque macros and no semantic AST.

### 2026-10-30 · wave16-evidence

Heading `### Wave16 bounded integration evidence, 2026-10-30` stays in wave16-evidence as `### Wave16 bounded integration evidence`.

Raw receipts: $MOVA_DATA/runs/octopus-wave16-integration-2026-10-30. native-evidence-hashes.json SHA-256 f93920d688e961ce9dd02be865389cbe54db3093416464e379c650f16578841d; algorithm-normalized.txt 1335d34b718e04458dac54697f8822eb8f02b0f408dbb547852dfaa2d60a7164; mova-normalized.txt f0ca49c0b0f12e5febe6e41657b4d098e8918171918f8ae53bd96868ac5f9328; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Structural controls reject a stale parent source hash and a wrong child identity, then restore exact bytes. Owned partial syntax projections repeat identically; macro bodies remain opaque. Owner receipts remain adjacent to sources; formatting-only followups retain original owner native hashes, while root repeats bind final bytes.

### 2026-10-30 · wave16-string-timings

Heading `### Wave16 bounded CPU string timings, 2026-10-30` stays in wave16-string-timings as `### Wave16 bounded CPU string timings`.

#### Reproduction and hashes

External source, executable and raw CSV files: $MOVA_DATA/runs/octopus-wave16-integration-2026-10-30. The executable ran twice sequentially after native suites completed. The source contains all fixtures and trial logic. Existing leaf citations describe the methods; these timings are our own observation.

- string-bench.rs: c64034628e942d3443717feff61ce36bc113e7ff4c06e290ff11408578ec7c38
- string-bench: f9c05d9cd0e3d0913c5e8c8adc873bd78abaf8e78b48a3cd37cd508d099ecdc8
- string-bench-1.csv: 6c78f3426298d4744c1e2d79bdaf1fd125c844bffb152ed2ff552db5e1a01fd9
- string-bench-2.csv: b11000ef73c0b249fc1566cf85a94cfe756ffdc2362c2eca863d85c7e7d62cb7
- bench-rustc.txt: b569b0c98ea3133c7252b4b8ea6b39b7b353813df0dd5f5c412c355691424112
- bench-cpu.txt: 84c5cc8a48187d0b55ae5d331e83c0b9aeebe8fbf3a093e53e4448866b5705f6
- $MOVA_DATA/target/octopus-wave5-integration/release/deps/libmova_algorithms-10bc952b79526a73.rlib: 75370d23f0473f1df262ac699e3c64b6a6322a75501148e623490e347a6d0c14
- Cargo.toml: cdab4c47aa1590e8a2c2147badfdaeccd8cb3a6eb5e5551c4b34bd1194e9c901
- algorithms/src/banded_levenshtein/mod.rs: b848d6edce86819e23f140bee8a37054ebb930e5d414a5bec1617c8c1f99b690
- algorithms/src/bit_parallel_lcs/mod.rs: 7cace61982a54fac084c8306581c460a10945a7b2f10f8d43e7478babb1365fc
- algorithms/src/levenshtein/mod.rs: f762f7d12b5ee35141cf679db6723099c10e39b7a009c08e6b0337b743ccb724
- algorithms/src/longest_common_subsequence/mod.rs: fec6e6aeea147e5954f49bcc927484c2ba68465f1495a01ad1bbb507e5f72f51

### 2026-10-30 · wave16-validation

Heading `### Wave16 final validation, 2026-10-30` stays in wave16-validation as `### Wave16 final validation`.

All 320 registered file sidecars, 15 inter-file groups and 293 fragments pass structural validation; repeated reports are byte-identical. They contain 3573 typed views with 1079 labels and 310 partial syntax projections. All 25 changed Rust sources pass registration against base bc75fed7731f79ca541f78d6a1698de2be40b076.

External run: $MOVA_DATA/runs/octopus-wave16-integration-2026-10-30. gates-summary.json SHA-256 7ed9facf6db7a093361232796aab088d9d9e426a8606dbc103c4b09ced9ba3db.

### 2026-10-30 · wave18-benchmark

Heading `### Wave18 bounded CPU measurements — 2026-10-30` stays in wave18-benchmark as `### Wave18 bounded CPU measurements`.

Source SHA-256 20b57c991f61192b2b05e085edf8303d56e43290fe10b7e1dfd36f10ffc9cccf. bench-1.tsv cdee8ca4858559397c5c46e07cc66dc9c4dbb8e8c05dde6c411bae39971807b3. bench-2.tsv 174ce30ddd75707c86661729da5adc41d9ab6a6fd309f09ccdbc8f2c9d3bfa6a.

#### Wave18 CPU measurement prediction — 2026-10-30

Saved before benchmark implementation and execution. Two bounded comparisons use generated bytes/integers only.

Two-Way versus existing KMP: both return all exact byte offsets and have linear worst-case scan complexity plus output. Two-Way avoids the O(pattern length) failure array; constants and periodicity can still make it slower. Four fixed text/pattern pairs include sparse matches, dense periodic matches, a repeated-prefix mismatch and arbitrary bytes. Predicted result agreement is exact; no universal speed ordering is predicted. Measure complete calls, including preprocessing and output allocation.

Wavelet matrix versus direct per-range sorting: static u64 values, 256 deterministic nonempty ranges with a valid zero-based rank. Matrix build costs O(64 n) time and storage; each kth costs O(64). Baseline clones each range and sorts it, costing O(sum L_i log L_i). Measure matrix query-only, matrix build plus all queries, and baseline clone/sort/query separately. Expected amortized improvement for many long ranges does not imply improvement for one short query or dynamic workloads. Independent sorted slices establish answers outside timing; checksums must match on every timed batch.

Use seven alternating-order samples per pair after fixed warmup, two independent process executions, black_box at API boundaries, fixed batch sizes, all sample durations retained. No data-dependent trial removal. Capture CPU/compiler and code hash. Native correctness, compilation time, scheduling noise and observed timing remain separate. No GPU, end-to-end Mova, training or approximation-error claim follows.

Raw timing harness and records: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30.
##### Receipt bindings

External run: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30. Native evidence index SHA-256: f85b3e074aa997321aff915acd6068610e33c9e50322525af7b6f529a7d7f422. Algorithm normalized outcomes: a307113f556e3e4dab5bf677df657899f439ffab39a1c8e1113ed23c97c1c733. Qagen normalized outcomes: 34efc5330be15215be0220412a9af2a517680394f975a7011f75a799a5210d21. Structural controls: a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Benchmark summary: 8ad77b5c224d73c393111ef5f6e14de3bb1d2b8108f5f01a89f90eee8d4a3c20.

### 2026-10-30 · wave18-evidence

Heading `### Wave18 algorithm evidence — 2026-10-30` stays in wave18-evidence as `### Wave18 algorithm evidence`.

#### Wave18 integration scope — 2026-10-30

Base ae72362aaa65659aa444cb7d090a05636c0e0691. Inventory: 353/784 Rust files registered, 431 pending (348 public, 83 closed); 696 nodes, 16 groups, 327 fragments, 3,899 typed views, 1,196 labels and 343 partial syntax projections. Added 24 registered file entries: 20 new algorithm implementation/test files, two existing qagen sources and two new harnesses; 13 fragments and 223 views. The library export source also changes, giving 25 changed Rust files.

Declared groupants pass structural validation. Repeated syntax projections match byte-for-byte. Injected stale bindings and wrong child identities reject, then restore exactly. Root corrected two imported file-node child lists and 32 graph heading references found by integration validation. These are model-wiring repairs. Source binding and section validation do not prove semantics, expanded ASTs or a complete dependency closure.

Final receipts cover 550 algorithm checks and 55 focused qagen checks, each twice in debug and twice in release. Unrelated Mova subsystems and the whole school were not rerun. No numerical approximation tier is introduced. Complement models and compatible-antecedent models remain distinct, with explicit information loss.

Raw receipts: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30; final native logs in native-final/. Prospective models and prior dated evidence remain. Question generation belongs to school-support; no inference-core or live acquisition pipeline is introduced.


##### Receipt bindings

External run: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30. Native evidence index SHA-256: f85b3e074aa997321aff915acd6068610e33c9e50322525af7b6f529a7d7f422. Algorithm normalized outcomes: a307113f556e3e4dab5bf677df657899f439ffab39a1c8e1113ed23c97c1c733. Qagen normalized outcomes: 34efc5330be15215be0220412a9af2a517680394f975a7011f75a799a5210d21. Structural controls: a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Benchmark summary: 8ad77b5c224d73c393111ef5f6e14de3bb1d2b8108f5f01a89f90eee8d4a3c20.

### 2026-10-30 · wave18-validation

Heading `### Wave18 final validation — 2026-10-30` stays in wave18-validation as `### Wave18 final validation`.

All 353 registered file sidecars, 16 groups and 327 fragments pass structural validation; repeated reports match byte-for-byte. There are 3899 typed views, 1196 labels and 343 partial syntax projections. All 25 changed Rust files pass registration against base ae72362aaa65659aa444cb7d090a05636c0e0691.

External run: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30. gates-summary.json SHA-256 c66b28b060dc611011b932b724406b2fb332b1c31ed190c4801efd500da1aa96.

### 2026-10-30 · wave19-benchmark

Heading `### Wave19 bounded CPU measurements — 2026-10-30` stays in wave19-benchmark as `### Wave19 bounded CPU measurements`.

Machine: Intel Core i5-13500; rustc 1.99.0, optimized Rust benchmark linked to the actual workspace release library. External run: $MOVA_DATA/runs/octopus-wave19-integration-2026-10-30. Benchmark source SHA-256 3c4eab1f5024c80901684231b759bce33b69b3320e82c0e535727498e88b653c; evidence receipt SHA-256 1e26c2f14bcfe85fff4a56094ff8b61f13c4b12142e036a7644396a9ed805715.

2026-10-30. Source benchmark_wave19.rs, std-only CPU, no timings yet. Horner value+derivative versus direct ascending monomial value+derivative: coefficients c_i=(i%5-2)/16 for i0..15, x_j=(j-128)/256 for j0..255; 200 complete batches per sample. Exact fixture range [-0.5,127/256] refines earlier broader prospective [-0.75,0.75]. Pair now computes identical value AND derivative observation. Preflight max absolute difference <=1e-11 per component, plus batch checksum equality tolerance1e-8.

### 2026-10-30 · wave19-evidence

Heading `### Wave19 integrated evidence — 2026-10-30` stays in wave19-evidence as `### Wave19 integrated evidence`.

New harness source SHA-256 894e55be473b27d0b67921ff2b44f0e2269999966a14273897cf0d198196f097. Library boundary SHA-256 7ccf1cf9f0600c178ff893a46efd0744edb27900a59061cbb1b91543f3f93856. Paired partial syntax projections match. Syntax omits expansion and semantic resolution. Mathematical caller assumptions, integer certificates and bounded numerical checks are distinct. The four-component shared-stream law is conditional and not jointly exercised by the current harness; moment/histogram fixtures are separate from the joint frequency fixture.

External receipts: $MOVA_DATA/runs/octopus-wave19-integration-2026-10-30/evidence-hashes.json, SHA-256 1e26c2f14bcfe85fff4a56094ff8b61f13c4b12142e036a7644396a9ed805715. Worker receipts remain in each leaf evidence file. The external source/control/native logs are immutable epoch records.

### 2026-10-30 · wave19-validation

Heading `### Wave19 final structural validation — 2026-10-30` stays in wave19-validation as `### Wave19 final structural validation`.

All 394 registered file sidecars and 21 groups pass; repeated reports match byte-for-byte. The inventory has 742 nodes, 4162 declared views and 384 partial syntax projections. All 42 changed Rust sources are registered against base 414ec19a0a4b2e0066334c87f364b987d9220109; focused formatting passes.

External gates-summary.json SHA-256 42a47e2e1f1ec6718f690aea6f6a57cd6ba3ad94926a17898ce47c922dd61fd7.

### 2026-10-30 · wave20-benchmark

Heading `### Wave20 paired CPU measurements — 2026-10-30` stays in wave20-benchmark as `### Wave20 paired CPU measurements`.

- $MOVA_DATA/target/octopus-wave5-integration/release/deps/libmova_algorithms-10bc952b79526a73.rlib: 1efe61591527b3a00b7d66bfa25b193c400960ea0151452c4c403af41a2474f7

- $MOVA_DATA/target/octopus-wave5-integration/release/deps/libqagen-1b63b762bc07d27c.rlib: c8794b999992c19b21dd882c14727c8c61aa40504839de9f768f58cbd74eab4f

- $MOVA_DATA/work/octopus-wave20-integration/algorithms/src/horner/mod.rs: 591283fb09317b37ebe6aadf1d0e6704737909a9aa91fda410abaa827cfd4de3

- bench-algorithms-1.txt: 379febb9ffe37a58dd7dec0c36a3604d69c055eef5a443918b69455e274a7678

- bench-algorithms-2.txt: 385bb774cf563d97548412a6b5fa53b0a69389ad719b3668e5970986e383d63d

- bench-horner-1.txt: d1386777d2f464f15ee15d6eadd30642e879fb4c6e600d164ba72eaef2067eac

- bench-horner-2.txt: afb41cdf8cab9a2d389c3a4709b0e1e162aba63cd12de5f870814c9a31b32af3

- bench-qagen-1.txt: 2d36c6cb4ec776a2ed451bf0e55de1e6bd877f97f42dc3c96e0ef0634e7e7c6c

- bench-qagen-2.txt: eed66d87576b748858d95b60fdd61a0cd3a61c8656e04dbd74e9998a686495d3

- benchmark-amendment-prospective.md: 5aa1658e856deade2d5c0a18747227b0bc37080ca7c2b40994c842293d40f245

- benchmark_algorithms.rs: f311cf23fa566c08d7bb982287048bffc5c1d3cbd374e92ee47a2392659fd0a0

- benchmark_horner.rs: aceedd6a44cdcae2ed47dddb76d2729e9e703d430da360ee4c2f6bd1d8321345

- benchmark_qagen.rs: 229ddb156cb3226f889ea998c0b7d5b2651875bdce14092f028db555482a2e06

- horner-before.rs: 0b468f42194662921f4716501003c2d0c41a9395878ed06573f5a54289e0ba63

- horner-prospective.md: c89ae01412930138185fed7aa65b261b2077e279a7e98491885aacd37ad1a15d

- parse-benchmarks.rs: 80b7df757ede4068fd2c3c8a74e7ca3945cd7373a84332ba6f3246caa659b09e

#### Receipt binding

Exact benchmark-results.md SHA-256 1a18983dee3c46fedab750ca87b909d27cce5075007a9011a3719e9aa4d13c30; benchmark-summary.json SHA-256 bc25c8e4c21a6950df57217dbe56c8e2f20e8771f0606574a831c239010ed59b; receipt-list SHA-256 cf45cadbd9243e7f99536b2079533028482a3929e81794131d09c67594f10810. External run: $MOVA_DATA/runs/octopus-wave20-integration-2026-10-30. Fixture metrics do not imply whole-Mova throughput.

#### Wave20 independent integration prospective — 2026-10-30

Eight bounded algorithm joins cover twenty production modules: SPD factor/triangular/iterative solve; QR least-squares transported to quadratic optimization; scalar Brent/Armijo; Lasso threshold solution; interpolation/Bezier/quadrature; monotone cubic with Romberg; subset sum transported to knapsack; totally-monotone/partition/BST DP oracles. Independent exact fixtures supplement cross-implementation agreement. Residual observations are not solution-error certificates on ill-conditioned systems; quadrature differences are not rigorous error bounds. Matrix optimizer domains and DP assumptions remain explicit.

Three synthetic qagen joins exercise real validation/gating/repair paths under deterministic injected lemma stubs. No teacher, model, corpus or database execution. Equal output rows do not recover original order or metadata.

Before measurement: compare bitset subset sum with existing boolean DP on identical deterministic bounded item/target fixtures, exact decision equality, two executions, seven alternating-order timing samples, black-box outputs and bounded repeats. Prediction: packed words reduce O(n*T) scalar work to O(n*ceil(T/64)); tiny cases may regress. Qagen stub call-count profiles compare valid/invalid anchor routes with same deterministic stub, predicting invalid inputs perform zero analysis and lawful behavior remains unchanged. Stub wall time is not model/service throughput. Root runs timings after harness review. No global speedup claim.

Validation: actual production joins and independent oracles; twice debug/release normalized outcomes; compiled semantic negative controls; exact source hashes retained by parent integration. External harness pending integration; worker sources read-only to reviewer.

##### Prospective amendment retained

#### Benchmark prospective amendment — 2026-10-30

Before timing, add an independent descending scalar boolean DP baseline to packed subset reachability, retaining the production witness-DP comparison separately. All three decisions must agree on the same tiny, dense and unreachable-even fixtures. Each baseline/candidate pair gets three untimed paired warmups and seven alternating-order samples with identical bounded repeats. CHECK/SAMPLE/MEDIAN label baseline_kind=scalar_boolean or production_witness. The scalar boolean baseline returns no witness and uses capacity-sized storage; this is a decision-kernel comparison, not proof of application-wide gain.

Before qagen cover samples, add three untimed paired old/new warmups. Each invocation constructs a new HashSet with fresh randomized hash state, which may vary execution cost even when outputs match exactly. Preserve all samples and compare repeated-run outcomes; do not claim byte-identical timing or model throughput. No measurements performed for this amendment.

### 2026-10-30 · wave20-evidence

Heading `### Wave20 integrated evidence — 2026-10-30` stays in wave20-evidence as `### Wave20 integrated evidence`.

Algorithm harness SHA-256 79314b1ffe5671cc4bd2afdccfdddc841a35571dea0b68a6901da13df79447bf. Qagen harness SHA-256 f50e7759c6a4e14d228678f323ac8d9449888c9241f610009231335c1063d5d4. API boundary SHA-256 4018c06a7f6ec66375f0b46897d41cf118eb94b296a7b630a8b7130febb01260. Paired partial syntax matches for the API boundary, both harnesses and both final import-formatted optimizer sources; expansion and semantic resolution remain absent. Mechanical import ordering in coordinate_descent_lasso and frank_wolfe follows worker execution, so worker source snapshots remain historical and current integrated native receipts cover the final sources. Benchmark rlib hashes retain the actual pre-format binaries measured; the measured bitset/subset-sum, qagen and Horner sources are unchanged.

External receipts: $MOVA_DATA/runs/octopus-wave20-integration-2026-10-30/evidence-hashes.json, SHA-256 cf45cadbd9243e7f99536b2079533028482a3929e81794131d09c67594f10810. Worker evidence remains in adjacent source groupants.

### 2026-10-30 · wave20-validation

Heading `### Wave20 final structural validation — 2026-10-30` stays in wave20-validation as `### Wave20 final structural validation`.

All 440 registered source sidecars and 25 groups pass; two structural reports match byte-for-byte. The inventory has 801 nodes (440 sources, 25 groups and 336 fragments), 4509 declared typed views, 1391 type labels and 430 partial syntax projections. All 49 changed Rust sources are registered against base 5182886a19ee83e797b332eae0bbc8518252825f; focused formatting passes.

External gates-summary.json SHA-256 529b13e155ea99d2c3a3cec66bc264358ef8cbbce3a3e821cc2c1f578dbfe442.

### 2026-10-30 · wave25-benchmark

Heading `### Algorithm CPU measurements — 2026-10-30` stays in wave25-benchmark as `### Algorithm CPU measurements`.

Root benchmark process 1600423 exited 0 after both serial executions and artifact verification. Raw files are retained under $MOVA_DATA/runs/octopus-wave25-integration-2026-10-30/. The process ID is an execution receipt, not a reproducibility guarantee.

### 2026-10-30 · wave25-evidence

Heading `### Wave25 finite evidence — 2026-10-30` stays in wave25-evidence as `### Wave25 finite evidence`.

These are synthetic release CPU measurements. No representative application frequencies, school learning/retention, GPU throughput, live teacher or whole-system accuracy were measured. The full world Clippy invocation encounters pre-existing idioms.rs modulo_one and know.rs never_loop failures; a clean workspace-wide lint result is not claimed. Raw evidence is retained under $MOVA_DATA/runs/octopus-wave25-integration-2026-10-30 and the sibling wave25 worker runs. [Structural receipts](model.md#wave25-validation) record remaining coverage and executed gates.

### 2026-10-30 · wave25-validation

Heading `### Wave25 validation — 2026-10-30` stays in wave25-validation as `### Wave25 validation`.

Base `fd540f37badaad97b53d56ba81684bbbc2ca4b9b`. This epoch adds 25 algorithms and retains conversion coloring; the spatial lazy-index candidate was rejected and its source restored exactly. Adjacent manifests and the transfer catalog bind current sources and models. Historical candidate receipts remain separate.

#### External receipt digests

Files are retained under `$MOVA_DATA/runs/octopus-wave25-integration-2026-10-30/`.

| Receipt | SHA-256 |
|---|---|
| native-algorithms-summary.json | f783969244707dbcaa742157e728a62f06a2dee59a5bebc99b2568af60f647b8 |
| native-retained-summary.json | d69c28d87d7a3ab0b49e69f7b9e3bb2c86aa3bfb8a7a3280cbb6ee65d92e7faa |
| retained-controls-audit.json | f376b097cd03500cb6e4974f14fdcf77967293858f4a67f6144529c52c0b11ac |
| groupants-final-1.json | c77df2de1bea900e0748b5dfa98962bebcab656eea1bda7992590508cfb64a90 |
| changed-final.json | 5a5908b02f39284f1c540500cee0bf9fe723d835ff4637c2050b7020332ee56e |
| inventory-final.json | ab46b21384c0c63c073497e5d2cb565ab1b7c42d7156bf397f5e3e33f5285919 |
| structural-controls-summary.json | dcb588fee1a0b25f2441c39e259f7c94b82e1c3a957c6ed7734cbb94a0be2151 |
| catalog-final-1.json | 12afbae3cab718a4427d73f5db1091c4165031b7e7abab0999a5817be0a85c1f |
| catalog-controls/controls.json | 3dd34e1947465b56b98f5a2d1f1657c53a030ca9fbd72e89ba38d1f2bc3f0e5c |
| benchmark-algorithms-summary.json | e26f4964e5f09d8d99aad8c4fa10f661a97aff1771cfeea82fde43404dd02a36 |

### 2026-10-30 · wave26-evidence

The task epoch retains its 2026-10-30 label. The library now exports 200 algorithms in separate directories. Its final state passes 877 tests twice in debug and twice in release, with identical normalized outcomes: 785 library tests, 91 external composition tests and one doctest. The 15 internal composition tests are included in the 785; the total of 106 composition observations must not be added again. All 782 previous test names remain; 92 source-local and three transform-composition tests were added.

Receipts: $MOVA_DATA/runs/octopus-wave26-integration-2026-10-30/{native-algorithms-summary.json,native-mova-summary.json,controls-audit.json,structural-controls-summary.json,catalog-audit.json}. Worker receipts are in the corresponding wave26-{indexes,linear,combinatorial,transforms,probability,mova} runs. Helpers preserve early failed orchestration checks separately: reading an incomplete log and an off-by-one provisional expected count were corrected before final validation; no failed native test was relabeled as a pass.

### 2026-10-30 · wave26-validation

Details: [wave26 evidence](model.md#wave26-evidence). Root receipts are under $MOVA_DATA/runs/octopus-wave26-integration-2026-10-30/: groupants-initial.json, changed-initial.json, inventory.json, stats.json, controls-audit.json, structural-controls-summary.json, catalog-audit.json, catalog-controls/, final and post-merge reports. The coverage queue is refreshed after registry finalization. No universal correctness, floating-error bound or whole-system verification is inferred.

### 2026-10-31 · wave26-evidence

Heading `### Wave26 evidence — completed 2026-10-31 UTC` stays in wave26-evidence as `### Wave26 evidence — completed UTC`.

### 2026-11-01 · historical-wave25-subject

2026-11-01: [edit-continuation](model.md#edit-continuation) adds a scoped parent composition obstruction: exact scalar child outputs and consumed length do not suffice for an exact continuation. Retained row/bit state is a separate obligation; the new external harness does not change literal native test counts.

### 2026-11-01 · document

Date: 2026-11-01. Sources: generic Levenshtein and Myers bit-vector modules,
bound by the parent manifest and this epoch's evidence receipt.
