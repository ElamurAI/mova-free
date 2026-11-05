# Testing the algorithms

Each algorithm directory contains production code in `mod.rs`, unit tests in `tests.rs`, and a groupant beside each Rust file. Measured 2026-10-30 (evidence recorded 2026-10-31): the 200-module library passes 877 checks — 785 library tests (including fifteen composition tests in `src/lib.rs`), 91 tests in the external composition harnesses and one doctest — twice in debug and twice in release with identical normalized results. Strict package Clippy, formatting and structural validation pass; the [wave26 evidence](../models/algorithm-library.groupant/model.md#wave26-evidence) binds the receipts, and the [README](README.md#validation-and-measurements) lists earlier waves. Model inventory and declared test counts are distinct from executed evidence.

Run the focused suite from the repository root:

```sh
cargo test -p mova-algorithms --offline
cargo test -p mova-algorithms --offline --release
cargo fmt -p mova-algorithms --check
cargo clippy -p mova-algorithms --all-targets --offline -- -D warnings
```

Select one algorithm or the composition tests:

```sh
cargo test -p mova-algorithms --offline bfs::tests
cargo test -p mova-algorithms --offline knapsack::tests
cargo test -p mova-algorithms --offline composition_tests
```

Use `-- --test-threads=1` for stable execution order when comparing logs. Use a separate `CARGO_TARGET_DIR` for concurrent worktrees.

The tables below list the test sources of the first eighty modules and of the joins in each harness; every other module has its tests in `src/<module>/tests.rs`.

| Algorithm | Test source |
| --- | --- |
| Binary search | [tests.rs](src/binary_search/tests.rs) |
| Stable merge sort | [tests.rs](src/merge_sort/tests.rs) |
| GCD | [tests.rs](src/gcd/tests.rs) |
| BFS | [tests.rs](src/bfs/tests.rs) |
| Dijkstra | [tests.rs](src/dijkstra/tests.rs) |
| Topological sort | [tests.rs](src/topological_sort/tests.rs) |
| Disjoint sets | [tests.rs](src/disjoint_set/tests.rs) |
| KMP | [tests.rs](src/kmp/tests.rs) |
| Levenshtein | [tests.rs](src/levenshtein/tests.rs) |
| 0/1 knapsack | [tests.rs](src/knapsack/tests.rs) |
| Heap sort | [tests.rs](src/heap_sort/tests.rs) |
| Selection | [tests.rs](src/quickselect/tests.rs) |
| Strict LIS | [tests.rs](src/longest_increasing_subsequence/tests.rs) |
| Strong components | [tests.rs](src/strongly_connected_components/tests.rs) |
| Minimum spanning forest | [tests.rs](src/kruskal/tests.rs) |
| All-pairs shortest paths | [tests.rs](src/floyd_warshall/tests.rs) |
| Dynamic sums | [tests.rs](src/fenwick/tests.rs) |
| Immutable minima | [tests.rs](src/sparse_table/tests.rs) |
| Integer square root | [tests.rs](src/integer_sqrt/tests.rs) |
| Modular power | [tests.rs](src/modular_power/tests.rs) |
| Signed shortest paths | [tests.rs](src/bellman_ford/tests.rs) |
| Bipartite matching | [tests.rs](src/bipartite_matching/tests.rs) |
| Z prefix matches | [tests.rs](src/z_algorithm/tests.rs) |
| Palindrome radii | [tests.rs](src/manacher/tests.rs) |
| Dynamic minima | [tests.rs](src/segment_tree/tests.rs) |
| Sliding minima | [tests.rs](src/sliding_window_minimum/tests.rs) |
| Counting sort | [tests.rs](src/counting_sort/tests.rs) |
| Radix sort | [tests.rs](src/radix_sort/tests.rs) |
| Extended GCD | [tests.rs](src/extended_gcd/tests.rs) |
| Prime sieve | [tests.rs](src/sieve/tests.rs) |
| Binary-weight paths | [tests.rs](src/zero_one_bfs/tests.rs) |
| DAG paths | [tests.rs](src/dag_shortest_paths/tests.rs) |
| Multi-pattern matching | [tests.rs](src/aho_corasick/tests.rs) |
| Suffix array | [tests.rs](src/suffix_array/tests.rs) |
| Common subsequence | [tests.rs](src/longest_common_subsequence/tests.rs) |
| Interval scheduling | [tests.rs](src/weighted_interval_scheduling/tests.rs) |
| Convex hull | [tests.rs](src/convex_hull/tests.rs) |
| Closest pair | [tests.rs](src/closest_pair/tests.rs) |
| Prime factors | [tests.rs](src/prime_factorization/tests.rs) |
| Generalized CRT | [tests.rs](src/chinese_remainder/tests.rs) |
| Bridges | [tests.rs](src/bridges/tests.rs) |
| Articulation points | [tests.rs](src/articulation_points/tests.rs) |
| Euler trail | [tests.rs](src/eulerian_trail/tests.rs) |
| Maximum flow | [tests.rs](src/max_flow/tests.rs) |
| Forest LCA | [tests.rs](src/lowest_common_ancestor/tests.rs) |
| Suffix automaton | [tests.rs](src/suffix_automaton/tests.rs) |
| Huffman codes | [tests.rs](src/huffman/tests.rs) |
| Palindromic subsequence | [tests.rs](src/longest_palindromic_subsequence/tests.rs) |
| Coin change | [tests.rs](src/coin_change/tests.rs) |
| Matrix chain | [tests.rs](src/matrix_chain/tests.rs) |
| A* paths | [tests.rs](src/a_star/tests.rs) |
| Two-SAT | [tests.rs](src/two_sat/tests.rs) |
| Global minimum cut | [tests.rs](src/stoer_wagner/tests.rs) |
| Verified rolling-hash search | [tests.rs](src/rabin_karp/tests.rs) |
| Horspool search | [tests.rs](src/boyer_moore_horspool/tests.rs) |
| Common substring | [tests.rs](src/longest_common_substring/tests.rs) |
| Restricted alignment | [tests.rs](src/optimal_string_alignment/tests.rs) |
| Subset sum | [tests.rs](src/subset_sum/tests.rs) |
| Binomial coefficient | [tests.rs](src/binomial_coefficient/tests.rs) |
| Interval union | [tests.rs](src/interval_union/tests.rs) |
| Johnson all-pairs paths | [tests.rs](src/johnson/tests.rs) |
| Prim forest | [tests.rs](src/prim/tests.rs) |
| Packed reachability | [tests.rs](src/transitive_closure/tests.rs) |
| Adjacent suffix prefixes | [tests.rs](src/lcp_array/tests.rs) |
| Burrows-Wheeler transform | [tests.rs](src/burrows_wheeler/tests.rs) |
| Wildcards | [tests.rs](src/wildcard_matching/tests.rs) |
| Bounded knapsack | [tests.rs](src/bounded_knapsack/tests.rs) |
| LCS reconstruction | [tests.rs](src/hirschberg/tests.rs) |
| Linear recurrence | [tests.rs](src/linear_recurrence/tests.rs) |
| Finite-field convolution | [tests.rs](src/number_theoretic_transform/tests.rs) |

The seven additional composition tests in [src/lib.rs](src/lib.rs) check these relations:

- Bellman-Ford agrees with Dijkstra when signed graph weights are nonnegative and distances fit both representations.
- Maximum bipartite matching leaves no BFS path from an unmatched left vertex to an unmatched right vertex in the alternating residual graph.
- Z prefix matches recover the same nonempty-pattern offsets as KMP when a separator lies outside both input alphabets.
- Manacher spans have zero Levenshtein distance from their reversals; extending a maximal span at both ends, when possible, gives nonzero distance.
- Segment-tree snapshots, sparse tables and sliding-window minima agree after point updates, including leftmost ties.
- Counting sort and radix sort agree with generic merge sort on bounded integers, and binary-search lower bounds recover original ranks.
- Sieve primes connect extended-GCD Bezout coefficients, ordinary GCD and modular-power inverses for nonzero residues.

These are bounded interaction checks with explicit domain restrictions. Integrated execution results belong in the [parent evidence](../models/algorithm-library.groupant/model.md#tests); leaf observations retain their dates.

Small exhaustive domains use independent oracles. Larger deterministic cases exercise wider values, dimensions and histories. Transformation checks retain their assumptions: connectivity does not identify a DSU root; vertex relabeling need not preserve a lexicographic topological order; weight scaling must remain representable. Inputs are deterministic and the tests add no dependencies.

Compiled source mutations run only in external scratch copies. A mutation must compile and a selected newly added test must fail for the intended reason. A compiler error or a filtered run with zero tests is not a successful negative control. Ordinary `cargo test` never intentionally mutates the production source.

The [parent evidence](../models/algorithm-library.groupant/model.md#tests) records executed validation and limits. Each test groupant records its oracle, bounded acceptance relation, failure ambiguity and transformation assumptions. Passing finite checks does not prove all inputs.

The graph, Z/Manacher and range groupants also have executable certificate, compatible-input and metamorphic checks. These retain strict/injective transform premises, deterministic-output versus cardinality distinctions and incomplete constructive preimages. Parent evidence records their mutation controls.

## Joins in `composition_wave4`

Run `cargo test -p mova-algorithms --test composition_wave4 --offline` for the seven separate interaction checks. Their equations, domains, excluded generalizations and ambiguous preimages are in the adjacent model. The hull counterexample rejects treating a hull as a sufficient nearest-pair summary; empty-suffix and global-cycle distinctions prevent invalid substitutions. Independent leaf oracles remain separate evidence.

## Joins in `composition_wave5`

`cargo test -p mova-algorithms --test composition_wave5 --offline` runs seven restricted joins: matching/flow, indexed graph certificates, LCA/BFS, substring language/count, prefix-tree cost, palindrome lengths and unit-work coin/matrix costs. Leaf oracles remain independent.

## Joins in `composition_wave7`

Run `cargo test -p mova-algorithms --test composition_wave7 --offline` for eight additional public-API checks. The harness and its adjacent groupant retain the exact adapter domains; the parent catalog records their typed joins. Independent leaf oracles separately test optimum values, witnesses, validation errors and degenerate inputs.

The domains include globally consistent validated A* heuristics, literal-index validity and condensation ranks, symmetric nonnegative cuts with checked u128 totals and proper zero-oriented sides, byte-verified rolling-hash collisions and all empty-pattern boundaries, contiguous substring witnesses, restricted nonmetric OSA distance, subset targets capped at 1,000,000, cancellation before binomial multiplication, and half-open interval union. Cross-algorithm agreement does not remove those restrictions.

Ten leaf production mutants and one compiled knapsack-adapter mutant fail assertions; compiler failures and zero-test runs are not accepted as controls. Parent evidence and the external wave7 run directories preserve the receipts. No GPU, approximation-quality or throughput gain follows from these checks.

## Joins in `composition_wave10`

Run each algorithm filter independently:

```sh
cargo test -p mova-algorithms --offline johnson::tests
cargo test -p mova-algorithms --offline prim::tests
cargo test -p mova-algorithms --offline transitive_closure::tests
cargo test -p mova-algorithms --offline lcp_array::tests
cargo test -p mova-algorithms --offline burrows_wheeler::tests
cargo test -p mova-algorithms --offline wildcard_matching::tests
cargo test -p mova-algorithms --offline bounded_knapsack::tests
cargo test -p mova-algorithms --offline hirschberg::tests
cargo test -p mova-algorithms --offline linear_recurrence::tests
cargo test -p mova-algorithms --offline number_theoretic_transform::tests
cargo test -p mova-algorithms --offline --test composition_wave10
```

[composition_wave10.rs](tests/composition_wave10.rs) declares eight joins:

- Johnson rows agree with Bellman-Ford and packed reachability on graphs formed by potential shifts of positive weights; a disconnected negative cycle distinguishes global from source-reachable rejection.
- Prim and Kruskal share optimum weight/component count; original edge IDs reconstruct and preserve the input connectivity, without requiring identical witnesses.
- Suffix order and LCP recover the same distinct-substring count as the suffix automaton on the same bytes.
- BWT roundtrips preserve bytes before literal wildcard/KMP comparison; the generated needles contain neither wildcard metacharacter.
- Expanding bounded item multiplicities into independent 0/1 items preserves the objective within the shared value and resource domain.
- Hirschberg pairs preserve element equality, strict index order and LCS length, with the corresponding Levenshtein bounds.
- Repeated NTT multiplication by [1,1] agrees with exact binomial coefficients after reduction modulo the NTT prime.
- Recurrence terms convolved with [1,-c[0],...] vanish on the generated tail; seed and truncation boundaries are excluded from that assertion.

The independent leaf oracles are separate from these joins. Johnson uses signed Floyd-Warshall, Prim enumerates candidate forests, and closure uses direct graph traversal. LCP compares directly sorted suffixes and prefixes; BWT materializes rotations and enumerates reachable canonical encodings; wildcard matching recursively partitions star matches. Bounded knapsack enumerates multiplicities; Hirschberg enumerates subsequences and checks witness indices. Recurrence uses iterative term generation, while NTT uses schoolbook convolution and checks prime/root-order assumptions and transform roundtrips.

The 43 unit tests of these ten modules comprise 12 graph tests, 15 byte/text tests, eight dynamic-programming tests and eight numeric tests. Leaf sidecars retain source-bound evidence and compiled mutation controls. Raw receipts are external under $MOVA_DATA/runs/octopus-wave10-{graphs,text,dp,numeric}-2026-10-30; each adjacent evidence file binds its exact logs. A valid leaf run does not establish the root joins.

The compiled recurrence-sign adapter control fails its assertion. Run the complete debug/release commands above, repeat each profile with normalized identity/result comparison, and retain all-target Clippy, narrow formatting, structural registration and compiled adapter controls in the parent evidence.

These APIs are exact. Resource caps, graph cycle scope, byte interpretation, canonical BWT metadata, Hirschberg tie policy and modular arithmetic restrict substitutions. Allocation refusal is not universally injected, and the largest supported NTT buffer is not exercised by the numeric leaf. Its five timing batches at four supplied sizes test a prospective NTT/schoolbook crossover only; no GPU, training or full-course acceleration follows.

## Joins in `composition_wave11`

Run the leaf filters and public-API harness:

```sh
cargo test -p mova-algorithms --offline hopcroft_karp::tests
cargo test -p mova-algorithms --offline dinic::tests
cargo test -p mova-algorithms --offline tarjan_scc::tests
cargo test -p mova-algorithms --offline damerau_levenshtein::tests
cargo test -p mova-algorithms --offline palindromic_tree::tests
cargo test -p mova-algorithms --offline lyndon_factorization::tests
cargo test -p mova-algorithms --offline berlekamp_massey::tests
cargo test -p mova-algorithms --offline fast_walsh_hadamard::tests
cargo test -p mova-algorithms --offline hungarian::tests
cargo test -p mova-algorithms --offline held_karp::tests
cargo test -p mova-algorithms --offline --test composition_wave11
cargo test -p mova-algorithms --offline --release --test composition_wave11
```

| Algorithm | Test source |
| --- | --- |
| Hopcroft-Karp matching | [tests.rs](src/hopcroft_karp/tests.rs) |
| Blocking flow | [tests.rs](src/dinic/tests.rs) |
| Tarjan components | [tests.rs](src/tarjan_scc/tests.rs) |
| Unrestricted Damerau distance | [tests.rs](src/damerau_levenshtein/tests.rs) |
| Eertree | [tests.rs](src/palindromic_tree/tests.rs) |
| Lyndon factors | [tests.rs](src/lyndon_factorization/tests.rs) |
| Recurrence synthesis | [tests.rs](src/berlekamp_massey/tests.rs) |
| XOR convolution | [tests.rs](src/fast_walsh_hadamard/tests.rs) |
| Hungarian assignment | [tests.rs](src/hungarian/tests.rs) |
| Held-Karp tour | [tests.rs](src/held_karp/tests.rs) |

[composition_wave11.rs](tests/composition_wave11.rs) declares eight restricted joins:

- Hopcroft-Karp cardinality agrees with matching encoded as unit-capacity flow; individual optimum witnesses need not agree.
- Tarjan components agree with mutual packed reachability on the same directed vertices and arcs.
- Unrestricted Damerau distance is at most OSA distance, which is at most Levenshtein distance on the same bytes.
- Eertree occurrence counts agree with palindrome occurrences represented by Manacher radii; distinct palindromes and total occurrences remain different observations.
- Duval factors of doubled bytes connect to the minimum cyclic rotation used by BWT; periodic ties retain the BWT convention.
- Berlekamp-Massey coefficients feed Kitamasa with unchanged lag orientation, prime and initial terms; a finite compatible recurrence does not uniquely identify future data.
- FWHT XOR convolution agrees with NTT ordinary convolution only on supports where index addition has no carries, so i XOR j equals i+j.
- Hungarian assignment gives a cycle-cover lower bound for Held-Karp on the shared cost domain. An optimal assignment may contain multiple cycles and is not itself a Hamiltonian tour.

Independent leaf oracles remain separate evidence: graph fixtures enumerate matchings/cuts or compare reachability; text fixtures use edit-state search, explicit palindrome intervals and finite factorizations; numeric fixtures solve small Gaussian consistency systems and sum XOR pairs directly; optimization fixtures enumerate assignments and tours. The joins cannot broaden a leaf's resource domain, change its objective or turn finite correspondence into a proof.

Repeat complete debug/release runs and compare normalized test identities/results; retain strict all-target Clippy, narrow formatting, source registration, repeated syntax projections and compiled production/oracle/adapter controls. Leaf records retain their own executed evidence. The compiled adapter mutation b[4*i] to b[3*i] fails the named no-carry convolution assertion; exact restoration passes. Structural reports repeat identically and all 26 changed Rust sources pass registration; the repository-wide require-all gate remains red for the explicit backlog. Parent evidence retains receipts. Allocation failure is not uniformly injected. Current exact APIs add no approximate-error budget, GPU result or full-course performance claim.


## Joins in `composition_wave14`

These ten leaf modules declare 34 unit tests. Use the module filters below with `cargo test -p mova-algorithms --offline`; four structure modules name their test module `wave14_models`. Oracle scopes describe bounded executable coverage, not proofs for all supported inputs.

| Algorithm | Module filter | Oracle scope and test count |
| --- | --- | --- |
| Minimum bipartite cover | `bipartite_vertex_cover::tests` | 3 declared tests: all 689 simple graphs with each partition size 0..=3 against independent subset-cover and recursive-matching enumeration; duplicate/alternating witnesses, accepted caps and validation precedence. |
| Stable marriage | `stable_marriage::tests` | 3 declared tests: all 46,674 complete strict preference profiles for n=0..=3, independent blocking-pair checks and every stable assignment for proposer-optimality; replacement witness, malformed preferences and n=1000/1001 budget boundary. |
| Deterministic primality | `miller_rabin::tests` | 4 declared tests: trial division on 0..100,000; strong pseudoprime fixtures, large primes/products and Boolean output fibers. The bounded oracle does not itself establish the full-u64 witness theorem. |
| Prime-field square root | `tonelli_shanks::tests` | 4 declared tests: exhaustive root enumeration for every prime p<200 and radicands 0..3*p; invalid/composite validation before zero shortcuts, large-prime constructed squares, canonical smaller roots and normalization fibers. |
| Minimum cyclic rotation | `minimal_rotation::tests` | 4 declared tests: explicitly sorted rotations of all ternary-byte words through length 8; periodic/byte ties, a 4096-byte witness, usize boundary arithmetic helpers and false-oracle/fiber controls. |
| Shift-And matching | `shift_and::tests` | 4 declared tests: direct-window oracle for ternary-byte texts through length 6 and patterns through length 4; lengths through 64, overlaps, empty boundaries, typed rejection at 65/128 and false-oracle/fiber controls. |
| Subset zeta | `subset_zeta::wave14_models` | 3 declared tests: literal subset sums for every ternary vector of lengths 1,2,4,8 under moduli 1,2,4,7,u64::MAX; wide residues, direction, validation without mutation and accepted N=2²⁰ boundary. |
| Subset Möbius | `subset_mobius::wave14_models` | 3 declared tests: literal signed inclusion-exclusion for every ternary vector of lengths 1,2,4,8 under moduli 1,2,4,7,u64::MAX; wide residues, direction, validation without mutation and accepted N=2²⁰ boundary. |
| Inversion count | `inversion_count::wave14_models` | 3 declared tests: direct pair enumeration for every ternary array through length 8 with input preservation; ties/non-Clone ordering and exact reversed-array total at n=2²⁰, plus over-cap rejection. |
| Minimum Cartesian tree | `cartesian_tree::wave14_models` | 3 declared tests: recursive leftmost-interval-minimum oracle for every ternary array through length 8; literal links/ties/non-Clone ordering and an equal-value chain at n=2²⁰, plus over-cap rejection. |

Run the nine public-API joins in both profiles:

```sh
cargo test -p mova-algorithms --offline --test composition_wave14
cargo test -p mova-algorithms --offline --release --test composition_wave14
```

[composition_wave14.rs](tests/composition_wave14.rs) checks these restricted relations:

- Bipartite cover cardinality equals Hopcroft-Karp matching and unit-capacity Dinic flow on the same graph; selected vertices cover every edge and the residual cut has the flow value.
- Common proposer preferences and a shared receiver priority produce the inverse priority permutation; inversion counts agree under permutation inversion.
- Sieve membership agrees with primality on the bounded modulus range; accepted canonical roots agree with enumerated roots and modular squares.
- Shift-And agrees with KMP and direct byte-boundary matching, retaining empty patterns, overlaps and the 64-byte pattern limit.
- Minimal rotation agrees with BWT inversion at the first row and preserves the last column; periodic equal rotations retain the earliest original offset.
- Cartesian-tree parent links feed forest LCA, recovering the leftmost interval minimum and agreeing in value with sparse-table queries.
- Inversion counts vanish after sorting; original and reversed inversion counts sum to the number of unequal index pairs, excluding ties.
- Subset zeta followed by Möbius recovers normalized residues, including composite moduli and modulus one; direct subset sums check the forward stage.
- Pointwise products of subset-zeta transforms invert to OR convolution under the same modulus, checked against direct index-pair accumulation.

Worker receipts record ten compiled production mutations and eight false-oracle controls rejected by named assertions. The root control replaces `subset_mobius::transform(&mut za, modulus)` with `subset_zeta::transform(&mut za, modulus)` in the OR-convolution join. It compiles and fails `zeta_product_inverts_to_or_convolution`; exact restoration and positive checks pass. Raw root run evidence includes `algorithm-progress.log`, `clippy-initial.log` and `algorithm-adapter-mutant.log`, with source/log bindings retained in the [parent evidence](../models/algorithm-library.groupant/model.md#tests) and adjacent leaf sidecars.

Full-u64 primality depends on the documented fixed-witness theorem; finite trial division alone does not prove it. Tonelli–Shanks uses an ascending nonresidue search without a small worst-case search bound. Subset inverses recover residues rather than original representatives. Preference joins use complete strict lists, byte joins do not add Unicode semantics, and range joins retain leftmost ties. Allocation failure and trait panics are not uniformly injected. These native checks establish no benchmark gain, approximate-error budget, GPU behavior, training improvement or general proof.


## Joins in `composition_wave16`

Run the harness in both profiles:

```sh
cargo test -p mova-algorithms --offline --test composition_wave16
cargo test -p mova-algorithms --offline --release --test composition_wave16
```

Leaf filters use `<module>::tests`; implementations and their independent oracles are adjacent:

| Module | Test source | Principal scope |
| --- | --- | --- |
| `boruvka` | [tests.rs](src/boruvka/tests.rs) | Signed forest optimum, indexed witnesses, loops, parallel edges and validation. |
| `prufer` | [tests.rs](src/prufer/tests.rs) | Canonical labeled-tree coding, literal trees, degrees, malformed inputs and small empty conventions. |
| `bron_kerbosch` | [tests.rs](src/bron_kerbosch/tests.rs) | Exhaustive small graph/subset optimum and lexicographic witness; simple-graph validation. |
| `banded_levenshtein` | [tests.rs](src/banded_levenshtein/tests.rs) | Thresholded byte distance, band budget and shortcut precedence. |
| `bit_parallel_lcs` | [tests.rs](src/bit_parallel_lcs/tests.rs) | Byte subsequence length, top-bit/full-word boundaries and shorter-side rejection. |
| `totient_sieve` | [tests.rs](src/totient_sieve/tests.rs) | Inclusive totients, coprime counts and resource boundary. |
| `mobius_sieve` | [tests.rs](src/mobius_sieve/tests.rs) | Signed arithmetic-function values, square factors and resource boundary. |
| `fibonacci` | [tests.rs](src/fibonacci/tests.rs) | Modular recurrence, full-width arithmetic, modulus one and zero-modulus rejection. |
| `li_chao_tree` | [tests.rs](src/li_chao_tree/tests.rs) | Registered-coordinate minima, earliest ties, wide products and state/error boundaries. |
| `offline_dynamic_connectivity` | [tests.rs](src/offline_dynamic_connectivity/tests.rs) | Edge lifetimes, rollback partitions, query order and invalid updates. |

The eight public-API joins in [composition_wave16.rs](tests/composition_wave16.rs) retain distinct adapter obligations:

- Borůvka signed costs equal Kruskal/Prim costs after a nonnegative constant shift and cardinality correction; DSU verifies acyclic partition-preserving witnesses.
- Every bounded Prüfer word decodes to a tree with degree equal to one plus label multiplicity; DSU and literal path/star anchors supplement encoding roundtrips.
- A clique in the complement of each 3-by-3 bipartite graph is an independent set of size six minus the minimum cover. Explicit witnesses and all 64 subsets independently check optimum/ties.
- Banded edit results match exact Levenshtein at each threshold, while bit LCS matches exact LCS and bounds edit distance; all coordinates count bytes.
- Totient and Möbius tables satisfy divisor convolutions, coprime-count and sieve identities on the bounded positive domain, with separate zero conventions.
- Fibonacci agrees with fixed-order Kitamasa and an additive oracle; Cassini and composite-modulus zero cases retain modular arithmetic and avoid overflow.
- Li Chao lines encode a quadratic dynamic program; direct predecessor enumeration checks cost and earliest tied insertion ID.
- Offline connectivity matches independently rebuilt DSU states throughout deterministic add/remove timelines, including loops, reversed endpoints and literal deletion/reconnect anchors.

These ten leaf modules have 32 unit tests and eight external joins. Keep compiled production/oracle controls separate from an adapter control, and retain exact source restoration before positive reruns. See the [wave16 evidence](../models/algorithm-library.groupant/model.md#wave16-evidence) for final source/log bindings and bounded CPU timing observations. The joins of the later harnesses (`composition_wave18`, `19`, `20`, `25` and `groupant_wave26_transforms`) are described in their adjacent groupants.

Finite bounded checks do not establish whole-domain correctness, practical runtime for 64-vertex clique search or full Mova behavior. Bounded CPU timings do not establish whole-system or training gains. Allocation failure is not uniformly injected, and query summaries generally admit multiple compatible input histories.
