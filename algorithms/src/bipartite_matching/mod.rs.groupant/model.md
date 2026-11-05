# algorithm-bipartite-matching

## descriptant

### Modeling

Prospective design is retained in model.md#prospective. Existing dijkstra and floyd_warshall interfaces informed adjacency and validation choices; no imported solutions. Adding duplicate occurrences preserves maximum cardinality and feasibility. Reordering adjacency preserves optimum size but can change chosen mates. A long alternating chain requires reassignment of existing mates; simple greedy free-neighbor selection does not satisfy the contract.

The mathematical combiner reads source, source-bound partial syntax, forward and typed obernants, invariants and evidence together. The complement is first and the converse is second; neither is interchangeable with negative tests. Future epochs must retain failed controls and adjust claims to measurements. Models do not prove source correctness or downstream wiring.

Implementation and tests are separate leaves; registry import and parent wiring belong to integration. Syntax omits semantic analysis and expanded macros. Exact algorithms have no approximate replacement profile.

## model

# bipartite_matching mod model

## Forward

Validate every right endpoint in left/adjacency order; return the first InvalidEdge. Otherwise return one Option<usize> per left vertex, describing an injective edge subset of maximum cardinality. Process roots by increasing left index, FIFO discovery and adjacency order. Duplicate edges and empty/unequal partitions are valid. The selected optimum is deterministic for identical input but not promised lexicographically least or invariant under edge reordering.

## Invariant

Before root k, mates are inverse maps forming a maximum matching on left vertices <k. FIFO search marks each right predecessor once and each left once; matched-right transitions visit its left mate. A found free right yields an alternating path to root; flipping increases cardinality by one. Failure to find a path leaves the enlarged prefix maximum. No recursive call stack is used.

## Cost

Validation O(L+E); mate initialization O(L+R). Each of L searches initializes L+R slots and visits at most L left vertices, R right vertices and E edges; flipping uses O(L+R). Total O((L+1)(L+R+E)) time and O(L+R) auxiliary space, including the L=0,R>0 allocation. No timing claim.

## nonbehavior

Type: behavior_complement. Let X be allocated finite ordered adjacency vectors and usize right-partition sizes, and Y the typed Result<Vec<Option<usize>>,BipartiteMatchingError> observation space, excluding allocation failure and nontermination. Define R subset X times Y by first-invalid-edge validation followed by the exact ordered FIFO augmenting traversal in Forward, not by arbitrary maximum matching. Set U=X times Y and behavior_complement=U minus R. It includes every valid maximum differing from that deterministic selected vector, as well as invalid, infeasible and submaximum observations. Finite negative fixtures check selected memberships only.

## compatible_inputs

Type: reverse_reconstruction. For output mate vector m, compatible inputs have L=len(m), R greater than every present mate, contain every selected edge and have matching number equal to count(Some). Exact compatibility additionally requires the stated FIFO/adjacency traversal to select m. Different edge sets and duplicate multiplicities can fit one result; unique graph recovery is not claimed. InvalidEdge restricts inputs to the reported first invalid endpoint and all earlier endpoints valid.

## graph_laws

Type: metamorphic_relation. Adding duplicate occurrences preserves maximum cardinality and feasibility. Reordering adjacency preserves optimum size but can change chosen mates. A long alternating chain requires reassignment of existing mates; simple greedy free-neighbor selection does not satisfy the contract.

## residual_obstruction

Type: augmenting_path_obstruction. For a valid graph and feasible matching M, orient unmatched edges left-to-right and matched edges right-to-left. M is maximum iff no residual path connects a free left vertex to a free right vertex. Feasibility requires original-edge membership and distinct matched right endpoints. The production result is checked over all 512 simple 3-by-3 graphs. Duplicates do not change the cardinality relation.

## cover_duality

Type: optimality_certificate. From all free left vertices let Z_L,Z_R be alternating residual reachability. For a maximum matching, C=(L minus Z_L) union Z_R covers every edge and |C|=|M|. Thus any matching is at most |C| and returned M attains that bound; this is an independent certificate rather than a second augmenting solver. The 512-graph test checks feasibility, absence of a reached free right, every edge covered and equality of cardinalities. The empty cover is outside the cover relation whenever an edge exists.

## edge_extension

Type: cardinality_monotonicity. For a validated graph G and one added valid edge e, nu(G)<=nu(G+e)<=nu(G)+1. The lower bound retains any old matching; deleting e from a new maximum proves the upper bound. Exact selected mates need not be preserved. All 512 simple 3-by-3 graphs times nine edge insertions yield 4608 comparisons, including already-present edges.

## transpose_transport

Type: partition_duality. Transpose a valid L-by-R graph to R-by-L while preserving edges. Swapping endpoints bijects matchings and therefore preserves maximum cardinality. The deterministic output vector is not an invariant: its length and left order change and tie selection may change. Tests check cardinality for 512 square graphs, and a two-row permutation explicitly rejects vector identity. This finite square fixture does not exhaust rectangular graphs.

## singleton_reconstruction

Type: constructive_preimage. For any feasible output vector M of length L with distinct Some(r)<R, construct G_M by giving row l only its selected r, or no edges for None. Production returns exactly M because each present edge has a distinct right endpoint. Repeating that sole edge also returns M, showing nonunique antecedents whenever M is nonempty. This is a sound subset of the deterministic output preimage. It does not imply every graph where M is maximum yields M. Tests reconstruct outputs from 512 graphs and reject both duplicate-right output and a different maximum matching for an ordered complete 2-by-2 graph.

## evidence

### Evidence

Three tests per algorithm. Exhaustive all simple graphs on L,R=0..3: 689 graphs, with independent enumeration of all edge subsets and pairwise distinct endpoints. Check maximum cardinality, length, membership, right uniqueness and repeat equality. Boundary fixtures include duplicate edges, empty sides, deterministic ties, invalid endpoints and a 1,024-left alternating chain. A compiled mutant disabling matched-left queue traversal must fail reassignment_duplicates_and_ties.

Focused debug runs twice and release run require identical normalized per-test statuses; whole-crate debug tests and strict all-target Clippy are also checked. The compiled mutant is an external copy, with exactly one selected test failing (exit 101); production source remains intact. Source-bound syntax is generated twice and compared after formatting. These finite checks do not establish a universal proof, runtime improvement or application integration.

Commands: cargo test -p mova-algorithms --offline bipartite_matching::tests:: -- --test-threads=1; same with --release; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings; mova-modelcheck syntax REPO SOURCE OUTPUT. Temporary uncommitted library exports enable focused checks; integration owns exports and registry.

Observed result: all three focused tests passed in both debug runs and release, with identical normalized statuses. Both source projections were identical across generation runs. Strict Clippy passed. The named external mutant compiled and was rejected: 0 passed, 1 failed, 2 filtered out, exit 101. No universal or approximate-profile claim follows.

## prospective

### Prospective

Before implementation: validate every right endpoint, then augment once per left vertex in input order using FIFO search and adjacency order. Maintain inverse mate arrays and flip alternating paths. Predict O(E+L(L+R+E)) time and O(L+R) auxiliary space. Exhaustively compare all <=3 by <=3 simple graphs with subset enumeration of edge sets, verify edge membership and uniqueness. Duplicates, rectangular/empty sides, invalid endpoints, deterministic ties and a long alternating chain supplement. Compiled greedy-only mutant must fail.

Finite tests and syntax projections are evidence, not proofs. No approximate executable profile is proposed.

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch 2026-10-30` stays in descriptant as `### Modeling`.

#### Deep graph epoch 2026-10-30

Three additional tests connect the specialized mathematical relations to actual production results. Implementation/API/error behavior is unchanged. The new models distinguish exact deterministic outputs from coarser optimal-value observations and retain preimage ambiguity. Earlier evidence remains historical. No speed, approximation profile, general proof or production substitution is claimed.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

#### Deep graph evidence 2026-10-30

Source SHA-256: 5178f09b94721815f93f8c126e9f9a566fc6872c599e14054aa35140d7cc8d75. Six focused tests pass in debug and release, each repeated twice with identical named outcomes (12 tests across the two algorithms). Strict all-target Clippy passes. Two isolated mutant files compile, and each named new test rejects its algorithm's mutant. Mutant log SHA-256 for this algorithm: 55a2659eea112be07fe262e8f7d9c9d8b5f8fccad6112679e4a8d9abff4163e5. Raw logs live under MOVA_DATA/runs/octopus-deep-graphs-2026-10-30. Syntax generation is repeated twice per source and compared byte-for-byte.

Bellman fixtures: 576 potential-transport pairs, one reachable-cycle transport pair, 84 distance-certificate memberships (one accepted), 75 compatible graph constructions and 25 negative-slack exclusions. Matching fixtures: 512 residual/cover certificates, 512 transpose comparisons, 4608 one-edge insertions, 512 exact-output reconstructions with duplicate variants; duplicate-right and alternate-maximum observations are excluded. These are finite correspondence checks, not unrestricted proofs.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
