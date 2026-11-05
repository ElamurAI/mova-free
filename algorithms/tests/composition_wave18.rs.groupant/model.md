# algorithm-composition-wave18

## model

# Wave18 native joins

## forward

Wave18 adds ten public algorithm modules and five separate tagged integration relations: canonical multigraph blocks to bridges and source counts, decoded bytes to matching/alignment, sieve primes to checked bounded factor products, floor sums to threshold counts, and persistent versions to wavelet order statistics. Each join preserves original vertex/edge identity, byte offset, exact integer domain, half-open range and immutable version coordinates. Domains are the literal six-vertex/seven-edge graph, 36 byte pairs, n=1..160 factors, 2275 bounded progression tuples and five length-five versions. Composition costs are sums of the reached API/adapter/oracle costs, including allocation and output, not a speedup claim. Distinct harness tests do not form one end-to-end algorithm.

## invariant

Each named fixture uses identical leaf coordinates and lawful domains. Edge IDs partition exactly once; loop exclusion is explicit. Byte offsets and traceback projections refer to original bytes. Arithmetic products/counts fit the stated finite domains. Threshold crossings require 0<=v<=48 and m>0. Persistent snapshot casts require nonnegative values, and all queries retain the exact version and half-open range. Independent literal oracles supplement cross-implementation equality.

## cost

Five bounded test families: graph traversal/set construction plus topological ordering; 36 byte-pair comparisons with lengths <=7 and <=64 alignment DP cells; sieve and factorization n=1..160 at budget 10000 plus literal divisibility; 2275 threshold tuples with O(64n+64*floor(60/m)) wavelet work and Euclidean floor_sum; five length-five versions with all 21 ranges each, sorted-slice oracles, O(64) wavelet queries and logarithmic tree operations. Allocations, retained versions, output vectors and oracle work are included. No measured speed claim or asymptotic correctness proof.

## composition

Each named test is a separate tagged relation. Composition requires identical graph vertices, original edge IDs, byte offsets, exact integer domains, range endpoints and immutable version identities; Mova joins retain batch/source coordinates, raw-output preference and checked ledger observations. Distinct tests do not constitute one end-to-end pipeline. Independent finite or literal oracles remain distinct from cross-implementation equality.

## epoch-memory

# Wave18 independent integration harness prospective model

Graph join: canonical edge blocks partition input IDs; non-loop singleton blocks equal bridges. Convert a literal block forest into a DAG and count unique root paths. Parallel edge identities and singleton loops remain distinct. Ambiguity: many graphs share block sizes, so partition plus literal identity checks supplement counts. Tiny fixture costs are linear after block construction.

Text join: run encode/decode, feed decoded bytes to Two-Way search, independently enumerate slice equality, align original and decoded sequences. Needleman-Wunsch score is compared to existing Levenshtein distance under match=0,mismatch=-1,gap=-1. Independently reconstruct inputs and sum column costs. Empty inputs and zero bytes are included. No universal inference from finite fixtures.

Number join: segmented primes and Pollard factors checked against literal divisibility/product; floor_sum models counts of sorted wavelet values less than a threshold on a simple arithmetic progression. Persistent versions retain sums of snapshots and wavelet order statistics for each snapshot. Outputs do not uniquely reconstruct histories or all original permutations. Fixture costs bounded by small exhaustive loops, not performance measurements.

Qagen join: synthetic raw/retry and calls JSONL flow through public raw resolution, gather, calls_json and collect. Rule V1 avoids model loading. Distinct retry answers show precedence; malformed accounting and u64 overflow must preserve qa/rejects/params sentinels; retry ELOOP must propagate through gather with valid primary. No teacher, database, corpus or model. Runtime and memory linear in fixture bytes and calls. Reverse fiber: counts identify compatible accepted rows but cannot recover discarded text or call order. Concurrent writers and late-write failure atomicity remain untested.

Negative control: mutate alignment adapter mismatch score from -1 to 0, compile, and require edit_distance_join_alignment_score assertion failure on unequal literal texts. Restore exact harness bytes and repeat positive. This tests actual new API wiring, not an unconsumed proxy.

## Harness model specification for root sidecars

Mandatory behavior_complement first: the algorithm harness excludes biological interpretation, resource exhaustion, universal correctness, large-data performance and signed-to-unsigned conversion outside its nonnegative snapshot domain. Qagen harness excludes transport/model/database calls, filesystem races and rollback after an actual write failure. Assertions are only finite observations.

Mandatory reverse_reconstruction second: alignment columns reconstruct both byte inputs; decoded runs reconstruct original bytes only for canonical runs; factors reconstruct n via product; historical sums and wavelet multisets leave permutations and update histories ambiguous. Qagen retry identity plus row content identifies the chosen raw fixture, but aggregate accounting retains many compatible logs. Sentinel preservation reconstructs unchanged preflight outputs only for tested errors.

Specialized algorithm views: (1) edge partition plus singleton bridge correspondence with explicit self-loop exclusion; (2) block incidence orientation and source path count, including isolates; (3) byte codec/search offsets and alignment trace reconstruction; (4) primality/product factorization join; (5) floor-sum horizontal threshold-count identity; (6) persistent historical sums joined with wavelet kth/rank and independent sorted snapshots. Specialized qagen views: (1) retry file precedence joins accepted answer and persisted anchor ID; (2) call sums and retries join serialized params; (3) accounting errors preserve all three previous artifacts; (4) metadata failure is not silently reclassified as missing or primary fallback.

Named finite domains and costs: block_forest_join_path_counts uses 6 original vertices, 7 edges and 10 incidence nodes plus a 3-node parallel DAG. run_decode_join_byte_search_and_edit_alignment uses 6 original byte words, 36 ordered pairs, lengths <=7 and <=64 DP cells; sum O(nm+n+m+offsets) over this fixture. segmented_primes_join_factorization_and_literal_divisibility uses n=1..160 and interval [0,160], factors with budget 10,000, independent trial divisibility. floor_sum_join_wavelet_threshold_counts uses n=0..12,m=1..7,a,b=0..4 (2,275 combinations), maximum value 48, thresholds up to 60, wavelet construction O(64n) per combination. persistent_versions_join_wavelet_order_statistics uses 5 versions of length 5, all 21 half-open ranges per version and equality queries 0..10. Qagen uses one paragraph, one-row primary/retry, at most two calls per log, three sentinel outputs, two accounting error classes, and Unix ELOOP. No certified approximant metric is proposed; exact fixture comparisons are not approximants.


## checks

5 named tests pass twice in debug and twice in release. The compiled adapter control fails its named assertion, with exact restoration; repeated partial syntax agrees. model.md#evidence records final native and structural receipts. Macro bodies remain opaque; semantic AST and whole-system proof are absent.

## catalog

# Wave18 supplied-input relations

Source-derived specifications; bounded native evidence and repeated partial syntax are recorded in model.md#evidence. No general proof or complete inverse engine is claimed.

## behavior-complement

Type: behavior_complement. Named check: all five named algorithm joins.

For the tagged finite input domains of the five tests, let R contain exactly the adapter outputs satisfying the stated literal or independent oracle relations. The complement (D x O) minus R includes dropped or duplicated edge IDs, treating loops as singleton bridges, incorrect incidence-to-DAG coordinates, lost parallel arc choices, shifted byte offsets, incorrect alignment reconstruction or score, composite sieve entries, wrong factor products, incorrect threshold inequalities, and reading the wrong immutable version or half-open range. Invalid large-resource domains, signed-to-unsigned casts of negative snapshots and unrestricted algorithm behavior are outside D. A red adapter control is evidence about this complement, not its definition.

## reverse-reconstruction

Type: reverse_reconstruction. Named check: all five named algorithm joins.

Given observed passing outputs retain all compatible tagged antecedents: edge partitions and path counts leave many endpoint graphs possible; alignment columns recover the two supplied byte sequences by deleting gaps, while an optimal score alone does not recover sequences or a unique traceback. Canonical encoded runs decode to the original bytes, but arbitrary noncanonical run streams need not have unique encodings. Factors recover n by product within the supplied range, whereas aggregate floor sums do not recover progression parameters. Wavelet multisets and persistent range sums generally leave permutations and update histories ambiguous. The fixture fixes coordinates and version identities externally; a finite pass constrains only these tested antecedents.

## edge-block-path-join

Type: edge_partition_incidence_path_relation. Named check: block_forest_join_path_counts.

Domain is the literal undirected graph with six vertices and seven ordered edge identities: one triangle, one bridge, two parallel edges, one loop, and one isolate. Assert the exact canonical blocks [[0,1,2],[3],[4,5],[6]] and coverage of all IDs. Non-loop singleton blocks, explicitly excluding the loop, must equal bridges. Construct incidence nodes at vertex offset six from actual block membership, omit loop incidence, orient only newly visited incidences from vertex zero, and count on the resulting ten-node DAG. Expected counts [1,1,1,1,1,0,1,1,1,0] retain the original isolate and the omitted loop block node as zeros. A separate three-node DAG with duplicate 0->1 arcs must count [1,2,2]. Coordinates and original edge IDs must agree across modules. The orientation adapter builds a spanning traversal; these finite assertions do not independently prove every possible incidence graph is a forest. Cost is linear traversal plus BTreeSet membership construction O(E log(V+1)) and DAG ordering O(E+V log(V+1)) on these fixed tiny fixtures.

## byte-codec-search-alignment

Type: byte_coordinate_traceback_relation. Named check: run_decode_join_byte_search_and_edit_alignment.

Domain is six byte words (empty, a, b, aaabaaa, abababa and [0,255,0,0]), all 36 ordered pairs, lengths at most seven. Encode then decode with output cap equal to the original byte length; decoded bytes must equal the original. Feed decoded bytes to Two-Way matching and compare every overlapping original byte offset to literal slice equality; empty patterns match all boundaries 0..=length. Align decoded bytes against the other word under match 0, mismatch -1 and gap -1, with cap 1024 and at most 64 DP cells. Score must equal negative Levenshtein distance; projecting each traceback column reconstructs the respective original word, no column is (None,None), and independently summed column scores match. This relation uses bytes, not Unicode character positions, and equal optimal scores do not imply identical traceback choices. Per pair cost includes O(nm) alignment and distance, linear codec/search work plus output, and bounded literal-search comparisons; storage includes the DP table, runs, decoded bytes, offsets and traceback.

## prime-factor-product

Type: prime_membership_factor_product_relation. Named check: segmented_primes_join_factorization_and_literal_divisibility.

Domain is sieve interval [0,160] and every n=1..160, factorization budget 10000. Compare sieve output to independent literal trial divisibility over 2..n; require sorted factors, every factor in the verified prime list, product exactly n, and singleton factorization iff n is prime. The empty factorization of one has product one and is not prime. All factors and the entire product fit u64 in this bounded domain; no overflowing-product claim extends beyond it. Product reconstructs n but does not recover a factorization execution or consumed budget. Cost includes bounded segmented sieve construction, at most the supplied factorization budget per n plus its validation/primality work, and the deliberately quadratic-scale literal trial-divisibility oracle over the fixed bound; no factorization throughput or general budget-success theorem follows.

## floor-wavelet-threshold

Type: horizontal_threshold_count_relation. Named check: floor_sum_join_wavelet_threshold_counts.

Domain comprises exactly 2275 tuples n=0..12, m=1..7, a,b=0..4. Materialize v_i=a*i+b for 0<=i<n, so 0<=v_i<=48. On the same full half-open range [0,n), use wavelet count_less with threshold k*m to count v_i>=k*m as n-count_less; sum for k=1..floor(60/m). Since every value is at most 48, this finite threshold cap includes every possible positive crossing and equals sum_i floor(v_i/m). Compare the u128 sum to floor_sum. Strict less-than is essential at exact multiples, n=0 gives zero, and m is never zero. Equal sums leave many progression tuples compatible. Per tuple cost is O(64n) wavelet construction, O(64*floor(60/m)) threshold queries, materialization O(n), and logarithmic Euclidean floor_sum work; the 2275 finite comparisons are not a general truncation guarantee for larger values.

## persistent-wavelet-versions

Type: immutable_snapshot_order_statistic_relation. Named check: persistent_versions_join_wavelet_order_statistics.

Domain is five snapshots of length five: initial [3,1,3,0,5] and four point updates branching from versions 0,0,1,2. The returned version ID must equal the appended snapshot index; original versions remain queryable. Every value is nonnegative (0..9), so the i64-to-u64 cast preserves value; this adapter must not be generalized to negative values. For each of the five versions and all 21 half-open ranges [start,end), build a wavelet matrix from that exact snapshot, compare kth results with an independently sorted slice, sum all kth values in i128 and compare to persistent range_sum for the same version and endpoints. kth at rank equal to range length must be None, including empty ranges. Compare rank equality counts for values 0..10; version_count must be five. Order-statistic multisets and sums do not uniquely recover element order or branching history. Cost includes path-copy updates O(log 5), five O(64*5) wavelet constructions, all bounded range sorts and O(64) kth/rank queries, persistent O(log 5) sums, and retained snapshot/version storage.

## evidence

### Wave18 algorithm evidence

The library contains 110 original deterministic algorithms. This epoch adds biconnected components, DAG path counts, Euclidean floor sums, segmented sieving, bounded Pollard rho factorization, Two-Way byte matching, Needleman–Wunsch alignment, canonical run-length coding, a wavelet matrix and a persistent segment tree. Every implementation and test source has an adjacent groupant. Standard library only; no imported problem statements or solution code.

Final root validation: 550 checks (489 unit, 60 external integration and one doctest) passed twice in debug and twice in release, with identical normalized named outcomes. Five new joins exercise all ten APIs; there are now 15 internal and 60 external composition checks. These relations retain edge IDs, bytes, arithmetic domains, half-open ranges and immutable versions.

Owners recorded ten compiled production mutations, each rejected by a named assertion and then restored. Root additionally changed the alignment adapter's mismatch score; its intended score assertion failed, then exact restoration passed. Compiler failures are not mutation rejection. Full root validation was repeated after three test-only Clippy refinements; owner receipts remain historical. Strict all-target algorithm Clippy and formatting pass.

Independent review found no concrete defect in the inspected domains. Finite oracles include 64,897 byte-search pairs, 14,400 alignment/path cases, 9,841 ternary RLE strings, cycle/path enumeration, direct floor sums, trial division and literal array snapshots. Large factorization checks partly reuse Miller–Rabin; small trial division and literal products provide separate evidence. Rho budget exhaustion is explicit. Alignment rejects any overflowing candidate score, including a losing candidate. Some allocation failures are outside typed error contracts. These are not universal proofs or new approximation-error guarantees.

CPU results in wave18-benchmark.md retain improvements and regressions from two executions. They do not establish Mova training, school or GPU throughput.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below; no speed or accuracy gain is claimed.

### 2026-10-30 · document

Date: 2026-10-30. Saved before harness implementation.

### 2026-10-30 · evidence

Heading `### Wave18 algorithm evidence — 2026-10-30` stays in evidence as `### Wave18 algorithm evidence`.

#### Wave18 integration scope — 2026-10-30

Base ae72362aaa65659aa444cb7d090a05636c0e0691. Inventory: 353/784 Rust files registered, 431 pending (348 public, 83 closed); 696 nodes, 16 groups, 327 fragments, 3,899 typed views, 1,196 labels and 343 partial syntax projections. Added 24 registered file entries: 20 new algorithm implementation/test files, two existing qagen sources and two new harnesses; 13 fragments and 223 views. The library export source also changes, giving 25 changed Rust files.

Declared groupants pass structural validation. Repeated syntax projections match byte-for-byte. Injected stale bindings and wrong child identities reject, then restore exactly. Root corrected two imported file-node child lists and 32 graph heading references found by integration validation. These are model-wiring repairs. Source binding and section validation do not prove semantics, expanded ASTs or a complete dependency closure.

Final receipts cover 550 algorithm checks and 55 focused qagen checks, each twice in debug and twice in release. Unrelated Mova subsystems and the whole school were not rerun. No numerical approximation tier is introduced. Complement models and compatible-antecedent models remain distinct, with explicit information loss.

Raw receipts: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30; final native logs in native-final/. Prospective models and prior dated evidence remain. Question generation belongs to school-support; no inference-core or live acquisition pipeline is introduced.


##### Receipt bindings

External run: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30. Native evidence index SHA-256: f85b3e074aa997321aff915acd6068610e33c9e50322525af7b6f529a7d7f422. Algorithm normalized outcomes: a307113f556e3e4dab5bf677df657899f439ffab39a1c8e1113ed23c97c1c733. Qagen normalized outcomes: 34efc5330be15215be0220412a9af2a517680394f975a7011f75a799a5210d21. Structural controls: a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Benchmark summary: 8ad77b5c224d73c393111ef5f6e14de3bb1d2b8108f5f01a89f90eee8d4a3c20.

##### Selected dependency snapshots

These reached production files are execution snapshots, not a complete semantic dependency closure.

- algorithms/src/bridges/mod.rs: 72204db4ba66993be2366219bc6aa00825548df331aa0dfd9d498729c3d4b6db
- algorithms/src/topological_sort/mod.rs: c0621a09327b1b3a07f8daadaa85d44a4e948f8b1013770b2a6be42d47787baf
- algorithms/src/miller_rabin/mod.rs: 0e6f4c3f59e549dc83106dbc98834b9eac28366202b094d60f67c288996f270b
- algorithms/src/gcd/mod.rs: abce5c9a9f0a2de26a6dbe392180e6a1c20eea179623fd5ede84434679b0d0e3
- algorithms/src/modular_power/mod.rs: be662de347e48f75c3a12b6238d7bca7d65114a1a79e4627723aab4e16299991
- algorithms/src/levenshtein/mod.rs: f762f7d12b5ee35141cf679db6723099c10e39b7a009c08e6b0337b743ccb724
- prag/src/opus.rs: 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136
- qagen/src/gate.rs: 09328f711abf4b6453f95bed3d873deb205c8801d939841974416fe74e3e5806
- qagen/src/schema.rs: 03e113198711324f029d8e9e4d9fc899416e3144ea856240b9d962775c78c2fe
- qagen/src/select.rs: 0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554
- qagen/src/run.rs: a34e297bdb69e178791983d8b9f6b35b1df34f223f15f1f13bc65d29153f2abd
- qagen/src/report.rs: 4abf85ffbd32f22802a89ef625780b288d92e92616c247a4d591e37fe10977c2

### Undated · evidence

#### Wave18 supplied-input native evidence

Source SHA-256 f202dd5b2c21c0211ebbf5b762a8f77c92e0a1abb49bfccdd028ab98c372f566. Repeated partial syntax SHA-256 d5a5524c70cff1df3e66f834f840497d523c0bb9e3042ab3e04fc039b9fe6143.
