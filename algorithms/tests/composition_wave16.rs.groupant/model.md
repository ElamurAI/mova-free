# algorithm-composition-wave16

## model

# Wave16 native joins

## forward

Eight tagged joins preserve signed forest weights, labeled codes, complement witnesses, byte metrics, divisors, recurrence residues, DP predecessors and timeline partitions.

## invariant

Identical vertices, byte offsets, moduli, registered coordinates, insertion IDs and query times across adapters.

## cost

Bounded exponential graph/code enumeration, quadratic byte/DP/divisor references and rebuilt DSU timelines plus reached API costs; no measured gain.

## composition

Each named test is a separate tagged relation. Composition requires identical graph vertices, byte coordinates, moduli, insertion IDs and timeline steps; Mova joins retain frame permutations, one-based batch positions, closed class coordinates and source/document/paragraph identities. Distinct tests do not constitute one end-to-end pipeline. Independent finite or literal oracles remain distinct from cross-implementation equality.

## epoch-memory

Prospective design retained below; no speed or accuracy gain is claimed.

# Wave16 eight native joins: prospective model

## Shared abstract relation and limits

Fix lawful input domain D, native stage relations R_i and adapters C. The forward target composes R_i while retaining C's coordinate/state constraints. The observation is a native assertion trace. Metric: exact disagreements against the declared scalar/witness relation; acceptance requires zero disagreements, including zero-valued outputs. No relative-error division, approximate-error budget or timing gain is claimed.

Behavior_complement comes first: in U=D times the declared observation space, absent behavior is U minus R. Concrete predicates below describe mathematical exclusions, not a verified complement of all Rust executions. Reverse_reconstruction is the compatible-input fiber {x in D:R(x,y)}. Preserve ambiguity and adapter premises; cost, length and query observations discard topology, contents and history. Correct leaves do not prove wiring. Finite joins do not prove the library, Mova or a pipeline-wide bound.

## Signed forest shift

Test: signed_forest_shift_joins_boruvka_kruskal_prim.

Domain: five vertices (vertex 4 isolated), all 64 edge masks on vertices 0..4, three weight phases; weight ((3*i+phase) mod 11)-5. Add a -7 loop and an optional reversed parallel edge of weight 2. Also empty zero-vertex forest.

Forward: Boruvka signed edges become existing Kruskal/Prim unsigned edges by +7. Fixed spanning-forest cardinality r=V-components gives shifted optimum cost=original+7r. Compare exact costs, counts and components, not edge order. Rebuild the selected witness with existing DSU; reject cycles and compare all pairwise connectivity to the full graph.

Complement: wrong translated cost, cycles, missing connectivity or wrong r is excluded. Translation does not preserve arbitrary variable-cardinality subset objectives. Reverse: costs/components admit many graphs and tied forests; original graph plus selected indices recovers only the chosen witness. Invariant: same partition and exact sum. Cost: 192 bounded cases, native near O(E log E+V), witness O(V^2 alpha(V)); no large signed conversion claim.

## Prüfer word/tree/partition

Test: prufer_words_join_tree_degree_and_dsu_identity.

Domain: all n^(n-2) words for n=2..6: 1,3,16,125,1296 words; label range 0..n. Include n=0/1 conventions and literal star/path anchors.

Forward: decode, check n-1 canonical edges u<v<n, acyclic connected DSU, and degree(v)=1+word multiplicity(v). Encode back. Independently anchor star [1,1] to [(0,1),(1,2),(1,3)] and path [(0,1),(1,2),(2,3)] to [1,2].

Complement: incorrect size, cycle, multiplicity or roundtrip is excluded. Reverse: fixed-n labeled trees and codes are bijective under the declared deletion rule; degree observations alone are ambiguous. Empty output requires retained n to distinguish n=0/1. Invariant: tree partition and degree sum 2(n-1). Expected cost O(n log n) per native case plus DSU; enumeration is exponential and capped at six. Literal and degree evidence supplements inverse-only testing.

## Complement clique and bipartite cover

Test: complement_clique_joins_bipartite_cover_and_independent_witness.

Domain: all 512 simple 3-by-3 bipartite graphs. Left labels 0..3, right labels +3. Complement on all six vertices includes within-partition edges.

Forward: complement maximum clique is an original independent set; size=6-minimum_vertex_cover_size. Verify original edge coverage and independence for both clique and explicit cover-complement witness. Enumerate all 64 subsets to independently obtain lexicographically least maximum independent set and compare the entire clique.

Complement: bad +3 transport, missing within-partition complement edges, wrong size/nonindependence or wrong lexicographic choice is excluded. Reverse: sizes and even selected witnesses admit many graphs. Invariant: cover and complement partition labels, every edge touches cover. Cost: 512 bounded 64-subset scans plus native matching/clique; exponential clique cost remains outside this small domain.

## Banded edit and bit LCS

Test: bounded_edit_lcs_join_exact_lengths_and_metric_bounds.

Domain: 31 binary byte words of length 0..4, 961 ordered pairs; periodic byte pairs (63,66) and (64,67). Thresholds 0,1,2,3,4,64,usize::MAX. Coordinates are bytes, not Unicode characters.

Forward: banded result is Some(d) iff existing exact Levenshtein d<=cap, else None, both orientations. Bit LCS equals existing exact LCS length l; check max(n,m)-l<=d<=n+m-2l and abs(n-m)<=d. Substitutions prevent demanding equality to insertion/deletion distance.

Complement: wrong threshold result, asymmetry or failed bounds excluded. Reverse: (d,l,n,m) loses string contents. Invariant: accepted shorter LCS side<=64; zero/empty exactness. Errors: both sides 65 reject LCS; equal MAX_BAND_CELLS length at unlimited threshold requires one cell too many and errors; length-difference pruning returns None before allocation. Cost: reference O(nm), band O(max(n,m)*min(min(n,m)+1,2k+1)), bit LCS O(256+n+m). Allocation refusal untested.

## Totient/Möbius divisors

Test: totient_mobius_join_divisor_convolutions_gcd_and_primes.

Domain: inclusive 0..256 tables; phi0=mu0=0, phi1=mu1=1; divisor equations only n>=1.

Forward: sum phi(d)=n, sum mu(d)=[n=1], sum mu(d)*(n/d)=phi(n), each over divisors. Independently count gcd(k,n)=1 for k=1..n using existing gcd. Existing sieve membership iff n>=2 and phi(n)=n-1; primes have mu=-1, prime-square multiples mu=0.

Complement: incorrect conventions or equations excluded. Reverse: scalar phi/mu is many-to-one; preserve n/table coordinates, no unique input reconstruction. Invariant: inclusive indexing and signed Möbius contributions; no divisor loop at zero. Cost: bounded sieves plus conservative O(LIMIT^2 log LIMIT) oracle. Production 1,000,000-cap errors remain leaf obligations.

## Fibonacci/Kitamasa/Cassini

Test: fibonacci_join_kitamasa_cassini_and_composite_zero.

Domain: m in [1,2,4,6,8,10,97,u64::MAX]; n=0..80 plus 2^32,2^63+7,u64::MAX. Cassini only n=1..80 avoids n+1 overflow.

Forward: fast doubling equals existing recurrence initial [0,1], coefficients [1,1]; small indices also equal independent additive loop. Cassini F(n-1)F(n+1)-F(n)^2=(-1)^n mod m uses u128 products and separately reduced terms. Literals F(12) mod8=0 and F(3) mod2=0 anchor zero behavior.

Complement: mismatch/residual error or accepting modulus zero excluded. Reverse: residues admit many n, m=1 collapses all; no field inverse assumed. Invariant: residues in 0..m, zero modulus rejected even n=0. Cost O(log(n+1)) at fixed order two. No arbitrary-recurrence claim.

## Li Chao quadratic DP

Test: li_chao_join_squared_cost_dp_and_earliest_predecessor.

Domain: six literal sequences, lengths 1..6, coordinates -4..4, repeated/unordered; penalties -3,0,2. dp0=0; conversions checked.

Forward: dp[i]=min_{j<i}(dp[j]+(x_i-x_j)^2+penalty). Insert slope -2*x_j, intercept dp[j]+x_j^2; query x_i then add x_i^2+penalty. Independent quadratic scan minimizes (cost,j), testing values and earliest ties. Insertion IDs equal chronological j.

Complement: future-state line, dropped term or wrong tied ID excluded. Reverse: dp/IDs lose coordinates and alternate predecessors, admit many histories. Invariant: query i sees precisely lines j<i; every queried coordinate registered. Cost native O(N log U), oracle O(N^2). No speed claim; other error domains remain leaf coverage.

## Offline connectivity and rebuilt partition

Test: offline_connectivity_join_rebuilt_dsu_and_literal_timeline.

Domain: n=1..6, eight phases, 48 deterministic edge toggles. Canonical undirected edge set, self-loops allowed; reverse remove orientation. Each toggle followed by Components and all ordered pair queries. Maximum operations 48*(2+6^2)=1824, below caps.

Forward: independently rebuild existing DSU from BTreeSet active edges at every step, compare answers in query order. Literal bridge deletion/loop/reconnect trace yields tagged observations [4,true,false,3,3,true]. Empty-universe Components=0.

Complement: stale deletion state, ordering mismatch, loop-induced merge or orientation dependence excluded. Reversed duplicate add and absent remove error. Reverse: query traces admit many edge histories; loops and temporary cycles may be invisible. Invariant: lifetimes alternate absent/present and query/state times agree. Oracle O(T*(E alpha(V)+V^2 alpha(V))); native rollback complexity remains its own contract. No concurrency/allocation simulation.

## Evidence status

Exactly eight tests; no compilation or execution by this authoring agent and no repository changes. Root must bind actual source, format/compile, repeat debug/release results and run adapter negative control. Independent leaf brute-force/mutation evidence remains separate. Prospective models are not formal proofs or measured observations.


## checks

8 named tests pass twice in debug and twice in release. A compiled adapter control fails its named assertion and exact bytes are restored. Partial syntax repeats identically; model.md#evidence records native and structural receipts. Macro bodies remain opaque; semantic AST and whole-system proof are absent.

## catalog

# Wave16 supplied-input relations

Source-derived specifications; bounded native evidence and repeated partial syntax are recorded in model.md#evidence. No general proof or complete inverse engine is claimed.

## behavior-complement

Type: behavior_complement. Named check: all eight algorithm joins.

Universe: tagged lawful graph, labeled-tree, byte, sieve, recurrence, envelope and timeline inputs with candidate observations. Exclude wrong signed forest shift/cardinality, tree degree/code inconsistency, wrong complement clique/tie, edit threshold or LCS bounds, divisor identities, recurrence/Cassini mismatch, DP values/predecessors and stale or reordered connectivity answers. Conditional concrete exclusions are not an all-Rust proof.

## reverse-reconstruction

Type: reverse_reconstruction. Named check: all eight algorithm joins.

Retain each tag's compatible inputs with identical shared coordinates. Costs/components lose graphs and ties; full fixed-n Pruefer codes recover labeled trees but degrees do not; edit/LCS lengths lose bytes; phi/mu scalars and modular Fibonacci lose indices; DP observations lose coordinates/losing lines; connectivity traces lose invisible edges/loops. Finite passing assertions admit implementations differing elsewhere.

## forest-shift

Type: signed_cardinality_transport. Named check: signed_forest_shift_joins_boruvka_kruskal_prim.

Test: signed_forest_shift_joins_boruvka_kruskal_prim.

Domain: five vertices (vertex 4 isolated), all 64 edge masks on vertices 0..4, three weight phases; weight ((3*i+phase) mod 11)-5. Add a -7 loop and an optional reversed parallel edge of weight 2. Also empty zero-vertex forest.

Forward: Boruvka signed edges become existing Kruskal/Prim unsigned edges by +7. Fixed spanning-forest cardinality r=V-components gives shifted optimum cost=original+7r. Compare exact costs, counts and components, not edge order. Rebuild the selected witness with existing DSU; reject cycles and compare all pairwise connectivity to the full graph.

Complement: wrong translated cost, cycles, missing connectivity or wrong r is excluded. Translation does not preserve arbitrary variable-cardinality subset objectives. Reverse: costs/components admit many graphs and tied forests; original graph plus selected indices recovers only the chosen witness. Invariant: same partition and exact sum. Cost: 192 bounded cases, native near O(E log E+V), witness O(V^2 alpha(V)); no large signed conversion claim.

## prufer-partition

Type: tree_code_transport. Named check: prufer_words_join_tree_degree_and_dsu_identity.

Test: prufer_words_join_tree_degree_and_dsu_identity.

Domain: all n^(n-2) words for n=2..6: 1,3,16,125,1296 words; label range 0..n. Include n=0/1 conventions and literal star/path anchors.

Forward: decode, check n-1 canonical edges u<v<n, acyclic connected DSU, and degree(v)=1+word multiplicity(v). Encode back. Independently anchor star [1,1] to [(0,1),(1,2),(1,3)] and path [(0,1),(1,2),(2,3)] to [1,2].

Complement: incorrect size, cycle, multiplicity or roundtrip is excluded. Reverse: fixed-n labeled trees and codes are bijective under the declared deletion rule; degree observations alone are ambiguous. Empty output requires retained n to distinguish n=0/1. Invariant: tree partition and degree sum 2(n-1). Expected cost O(n log n) per native case plus DSU; enumeration is exponential and capped at six. Literal and degree evidence supplements inverse-only testing.

## clique-cover

Type: independent_witness_transport. Named check: complement_clique_joins_bipartite_cover_and_independent_witness.

Test: complement_clique_joins_bipartite_cover_and_independent_witness.

Domain: all 512 simple 3-by-3 bipartite graphs. Left labels 0..3, right labels +3. Complement on all six vertices includes within-partition edges.

Forward: complement maximum clique is an original independent set; size=6-minimum_vertex_cover_size. Verify original edge coverage and independence for both clique and explicit cover-complement witness. Enumerate all 64 subsets to independently obtain lexicographically least maximum independent set and compare the entire clique.

Complement: bad +3 transport, missing within-partition complement edges, wrong size/nonindependence or wrong lexicographic choice is excluded. Reverse: sizes and even selected witnesses admit many graphs. Invariant: cover and complement partition labels, every edge touches cover. Cost: 512 bounded 64-subset scans plus native matching/clique; exponential clique cost remains outside this small domain.

## edit-lcs

Type: byte_metric_transport. Named check: bounded_edit_lcs_join_exact_lengths_and_metric_bounds.

Test: bounded_edit_lcs_join_exact_lengths_and_metric_bounds.

Domain: 31 binary byte words of length 0..4, 961 ordered pairs; periodic byte pairs (63,66) and (64,67). Thresholds 0,1,2,3,4,64,usize::MAX. Coordinates are bytes, not Unicode characters.

Forward: banded result is Some(d) iff existing exact Levenshtein d<=cap, else None, both orientations. Bit LCS equals existing exact LCS length l; check max(n,m)-l<=d<=n+m-2l and abs(n-m)<=d. Substitutions prevent demanding equality to insertion/deletion distance.

Complement: wrong threshold result, asymmetry or failed bounds excluded. Reverse: (d,l,n,m) loses string contents. Invariant: accepted shorter LCS side<=64; zero/empty exactness. Errors: both sides 65 reject LCS; equal MAX_BAND_CELLS length at unlimited threshold requires one cell too many and errors; length-difference pruning returns None before allocation. Cost: reference O(nm), band O(max(n,m)*min(min(n,m)+1,2k+1)), bit LCS O(256+n+m). Allocation refusal untested.

## divisor-sieves

Type: inclusive_divisor_coordinates. Named check: totient_mobius_join_divisor_convolutions_gcd_and_primes.

Test: totient_mobius_join_divisor_convolutions_gcd_and_primes.

Domain: inclusive 0..256 tables; phi0=mu0=0, phi1=mu1=1; divisor equations only n>=1.

Forward: sum phi(d)=n, sum mu(d)=[n=1], sum mu(d)*(n/d)=phi(n), each over divisors. Independently count gcd(k,n)=1 for k=1..n using existing gcd. Existing sieve membership iff n>=2 and phi(n)=n-1; primes have mu=-1, prime-square multiples mu=0.

Complement: incorrect conventions or equations excluded. Reverse: scalar phi/mu is many-to-one; preserve n/table coordinates, no unique input reconstruction. Invariant: inclusive indexing and signed Möbius contributions; no divisor loop at zero. Cost: bounded sieves plus conservative O(LIMIT^2 log LIMIT) oracle. Production 1,000,000-cap errors remain leaf obligations.

## fibonacci-recurrence

Type: modular_recurrence_identity. Named check: fibonacci_join_kitamasa_cassini_and_composite_zero.

Test: fibonacci_join_kitamasa_cassini_and_composite_zero.

Domain: m in [1,2,4,6,8,10,97,u64::MAX]; n=0..80 plus 2^32,2^63+7,u64::MAX. Cassini only n=1..80 avoids n+1 overflow.

Forward: fast doubling equals existing recurrence initial [0,1], coefficients [1,1]; small indices also equal independent additive loop. Cassini F(n-1)F(n+1)-F(n)^2=(-1)^n mod m uses u128 products and separately reduced terms. Literals F(12) mod8=0 and F(3) mod2=0 anchor zero behavior.

Complement: mismatch/residual error or accepting modulus zero excluded. Reverse: residues admit many n, m=1 collapses all; no field inverse assumed. Invariant: residues in 0..m, zero modulus rejected even n=0. Cost O(log(n+1)) at fixed order two. No arbitrary-recurrence claim.

## affine-dp

Type: chronological_predecessor_transport. Named check: li_chao_join_squared_cost_dp_and_earliest_predecessor.

Test: li_chao_join_squared_cost_dp_and_earliest_predecessor.

Domain: six literal sequences, lengths 1..6, coordinates -4..4, repeated/unordered; penalties -3,0,2. dp0=0; conversions checked.

Forward: dp[i]=min_{j<i}(dp[j]+(x_i-x_j)^2+penalty). Insert slope -2*x_j, intercept dp[j]+x_j^2; query x_i then add x_i^2+penalty. Independent quadratic scan minimizes (cost,j), testing values and earliest ties. Insertion IDs equal chronological j.

Complement: future-state line, dropped term or wrong tied ID excluded. Reverse: dp/IDs lose coordinates and alternate predecessors, admit many histories. Invariant: query i sees precisely lines j<i; every queried coordinate registered. Cost native O(N log U), oracle O(N^2). No speed claim; other error domains remain leaf coverage.

## offline-partition

Type: timeline_partition_transport. Named check: offline_connectivity_join_rebuilt_dsu_and_literal_timeline.

Test: offline_connectivity_join_rebuilt_dsu_and_literal_timeline.

Domain: n=1..6, eight phases, 48 deterministic edge toggles. Canonical undirected edge set, self-loops allowed; reverse remove orientation. Each toggle followed by Components and all ordered pair queries. Maximum operations 48*(2+6^2)=1824, below caps.

Forward: independently rebuild existing DSU from BTreeSet active edges at every step, compare answers in query order. Literal bridge deletion/loop/reconnect trace yields tagged observations [4,true,false,3,3,true]. Empty-universe Components=0.

Complement: stale deletion state, ordering mismatch, loop-induced merge or orientation dependence excluded. Reversed duplicate add and absent remove error. Reverse: query traces admit many edge histories; loops and temporary cycles may be invisible. Invariant: lifetimes alternate absent/present and query/state times agree. Oracle O(T*(E alpha(V)+V^2 alpha(V))); native rollback complexity remains its own contract. No concurrency/allocation simulation.

## evidence

### Wave16 bounded evidence

The complete 100-algorithm crate passes 504 checks (448 unit, 55 external composition, one doctest) twice in debug and twice in release. The 155 selected Mova checks also pass twice in each profile, with identical normalized named outcomes: 139 previous focused checks, 11 new unit checks, two existing fbench/silver tests and three new integration joins. This is a selected Mova suite, not full-system validation.

Ten new algorithm implementation mutants and two Mova production mutants fail named assertions in owner receipts, with exact restoration before positive runs. Root additionally compiles two adapter controls: wrong forest weight/cardinality correction and swapped one-based silver-row identity. Both fail their named assertions, then exact source bytes are restored. Compilation failure is not counted as semantic rejection.

Mova fixes bound duplicate-link credit by gold occurrences (multiset intersection) and retain context unless adjacent paragraphs share source/document identity and checked consecutive paragraph numbers. Old-code witnesses cover duplicate overcredit, cross-source suppression and overflow. Three root joins exercise notation/alignment/counts, row/item/class transport and batch/prompt coordinates. Unique-link semantics and public APIs remain unchanged.

Independent read-only review found no additional concrete defect. Coverage limits: graph/tree cap+1 checks do not establish maximum-size successful throughput; Li Chao scans an independent line envelope but shares widened arithmetic and value/ID ordering; the connectivity cap fixture does not stress worst-case interval storage. A 64-vertex clique representation cap does not make exponential search cheap.

The ten new implementations are exact within their declared domains; no new approximate Rust surrogate or Lean proof is asserted. No live parser/teacher/corpus acquisition, GPU, whole-school throughput or persistent-learning adoption was executed. No general semantic proof or training-speed gain follows from these checks.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below; no speed or accuracy gain is claimed.

### 2026-10-30 · document

Written 2026-10-30 before compilation/execution. External draft only. Source inspection and worker API confirmation establish signatures, not runtime correctness. Root owns formatting, compilation, repeated evidence, adapter control and source binding. All ten new algorithms occur in eight test bodies.

### 2026-10-30 · evidence

Heading `### Wave16 bounded evidence, 2026-10-30` stays in evidence as `### Wave16 bounded evidence`.

8 new native joins. Source SHA-256 2e76f3c5c145f97f5f1e71536245f0769b094907acd3a92f1d3705e31be41e3b; twice-identical partial syntax SHA-256 7d73c503a2721f232d596177004f75fd1b30d20fdec731902d6e77691833be19.

Raw receipts: $MOVA_DATA/runs/octopus-wave16-integration-2026-10-30. native-evidence-hashes.json SHA-256 f93920d688e961ce9dd02be865389cbe54db3093416464e379c650f16578841d; algorithm-normalized.txt 1335d34b718e04458dac54697f8822eb8f02b0f408dbb547852dfaa2d60a7164; mova-normalized.txt f0ca49c0b0f12e5febe6e41657b4d098e8918171918f8ae53bd96868ac5f9328; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Structural controls reject a stale parent source hash and a wrong child identity, then restore exact bytes. Owned partial syntax projections repeat identically; macro bodies remain opaque. Owner receipts remain adjacent to sources; formatting-only followups retain original owner native hashes, while root repeats bind final bytes.
