# algorithm-library-api

## descriptant

### Library API groupant

200 public modules; 413 bound algorithm source leaves; 15 internal and 91 external literal test declarations across 12 external harnesses, totaling 106 bounded composition checks. Five wave26 families organize 25 new modules. Counts use literal #[test] markers, not expanded macros. Mova checks are separate.

#### Mathematical combiner

Combine leaf relations only after an adapter establishes the next domain and preserves indices, vertex identities, arithmetic restrictions or update snapshots. The parent models ordering/rank, distance/reachability, forest accounting, exact prefix differences and a restricted square/root inverse. The five file-level typed views retain the required complement and reverse relation plus the original rank, unit-weight and partition transports. Additional specialized joins live at the parent level.

#### Limits

Descriptions and finite evidence do not prove every client composition. Cross-implementation agreement can hide shared defects; independent leaf oracles and explicit source-mutation controls retain separate roles. Declarative preimages are not a universal inverse engine. Partial syntax has no macro expansion or semantic resolution. No unmeasured speedup or unexecuted fixture count is claimed.

#### Current wave18 evidence

110 implementations and 110 test sources, the API boundary and eight external harnesses form 229 registered algorithm source leaves. File trees contain 1,402 views; the parent has 80. Fifteen internal and 60 external checks give 75 composition relations. Final root execution passes 550 checks twice per profile with identical outcomes. See model.md#wave18-evidence for scope, controls and receipts; bounded CPU comparisons are in the parent wave18-benchmark.md. Source registration and finite execution are not universal proof.

#### wave19-boundary

The new numerical and approximation families carry explicit assumptions, error metrics and failure statuses. Parent hierarchy binds all reached sources; this export file alone does not certify them.

#### wave20-boundary

Twenty new algorithm modules retain explicit finite arithmetic, callback and structural premises. The algorithm branch binds 311 sources: 150 implementation/test pairs, this boundary and ten external harnesses. Fifteen internal plus 76 external algorithm joins give 91; three qagen joins remain separate. Six historical executable surrogate sources belong elsewhere in the registry. Source registration and finite observations imply neither general proof nor whole-Mova speedup.

#### Wave20 executed boundary

The 150-module suite passes 695 native algorithm checks twice per profile with identical normalized outcomes. Fifteen internal and 76 external joins give 91 compositions; the parent binds 311 algorithm sources. Qagen separately passes 69 checks with its live-model test skipped. Parent wave20-evidence.md and wave20-benchmark.md retain actual receipts and limits.

#### wave25-boundary

25 additional modules and 12 additional algorithm joins produce 175 modules, 362 bound algorithm sources and 103 joins (15+88). Final finite native and timing evidence is recorded in models/algorithm-library.groupant/model.md#wave25-evidence. No generic numerical percentage guarantee or measured whole-Mova gain follows from source registration.

#### wave26-boundary

200 public modules; 413 bound algorithm source leaves; 15 internal and 91 external literal test declarations across 12 external harnesses, totaling 106 bounded composition checks. Five wave26 families organize 25 new modules. Counts use literal #[test] markers, not expanded macros. Mova checks are separate.

## model

# Library API source groupant

## Forward

The library exports 200 modules and forbids unsafe Rust. Fifteen test-only
compositions exercise selected client joins; they are not public wrapper APIs.
The tagged successful observations are:

1. Sort/search: sorted multiset S, lower-bound k=count{x in A:x<t}, and first
   match Some(k) iff t occurs.
2. Unit weights: BFS edge-count distances equal Dijkstra distances after assigning
   every edge weight one and widening finite results to u128.
3. Undirected partition: DSU unions yield the same all-pair membership relation
   and class count as BFS reachability.
4. Ordering structure: heap/merge ordered values agree, quickselect at each tested
   valid rank equals that sorted value, and LIS on the sorted sequence has length
   equal to its number of distinct order-equivalence classes.
5. Weighted reachability: Floyd-Warshall rows agree with Dijkstra; SCC membership
   equals mutual finite reachability in the same graph's distance matrix.
6. Spanning forest: chosen original edges preserve connectivity; selected weight
   sum and class count agree with the report. Minimality is a leaf obligation.
7. Prefix reconstruction: exact adjacent Fenwick prefix differences recover the
   current values; a rebuilt sparse table agrees with direct range minima, and
   range sums agree with direct sums from that same snapshot.
8. Restricted square/root: for representable x*x<m, modular squaring preserves
   the square and integer square root recovers x.

Read leaf contracts for errors, lawful traits and representable domains. The
documentation example composes sort/search and calls gcd. Exports introduce no
call-time initialization. Each composition retains its own tag and assumptions;
the library does not execute all fifteen as one runtime pipeline.

9. Signed distance embedding: Bellman-Ford agrees with Dijkstra after an exact nonnegative weight/result conversion.
10. Matching residual: valid maximum matching leaves no free right vertex reachable from a free left in the alternating residual graph.
11. Prefix matching: sentinel-separated Z prefix lengths identify exactly the nonempty KMP matches.
12. Palindrome edit: each centered span has zero reverse edit distance and cannot extend at both ends.
13. Range snapshots: dynamic tree, static sparse table and window deque share values, exclusive bounds and leftmost ties.
14. Integer sorting: counting/radix/generic sorting preserve one ordered multiset and original ranks.
15. Prime inverse: normalized Bezout coefficients and Fermat powers give the same inverse for nonzero prime residues.

The seven new catalog sections specify precise domains, complementary exclusions, reverse ambiguity, fixture bounds and composed cost. In particular, palindrome DP is oracle overhead, and matching validity precedes its residual certificate.

## Invariant

Sorting establishes ordered-search input. Graph adapters preserve vertex identities,
endpoint validity and zero-length self reachability; undirected symmetry is required
for DSU connectivity. Original Kruskal edge indices must not be confused with positions
in an internal sorted edge array. Connectivity does not imply minimum weight.

Separate quickselect copies preserve the same input multiset. Heap sort's instability
does not invalidate value-order equality, and strict LIS is not a nondecreasing
subsequence. Fenwick and reference updates advance only on accepted checked updates;
sparse queries use a reconstruction from that exact post-update state. Arithmetic
joins admit checked products and strict modulus bounds before asserting an inverse.

The lib source projection does not bind external modules. The parent binds all
implementation and test sources, including their own finite oracle assumptions.

## Cost

Client costs are conditional on reached calls. The original three joins cost
O(n log(n+1)) for sort/search, O(V+E log(E+1)) for BFS/Dijkstra after adaptation,
and O(V(V+E)) for all-source BFS plus DSU work, retaining O(V^2) rows.

The ordering join adds heap/merge/LIS O(n log(n+1)); q independent quickselect
calls conservatively add O(q*n^2) worst-case time. All-rank testing can therefore
be cubic despite one selection often being cheaper. The graph join combines
O(V^3+E) Floyd-Warshall with V Dijkstra calls and SCC work including canonical output
sorting. Forest oracles can cost more than Kruskal's O(V+E log(E+1)+E alpha(V)).
Prefix reconstruction needs O(n log(n+1)) prefix-query work and sparse construction,
with O(log(n+1)) point updates and Fenwick range-sum queries, plus O(1) sparse queries. Each snapshot rebuild repeats construction; direct oracle scans and per-call constant overhead are extra.
The square/root join is bounded by fixed integer width. Full formulas, assumptions
and additional storage are in the parent model. Fixture counts, test execution
and timings remain separate evidence; no total-library speedup is claimed.

## behavior-complement

Type: behavior_complement. U is the disjoint union of the fifteen tagged successful
input/observation spaces in Forward, restricted to their stated domains and lawful
traits. R is the corresponding conjunction of equations for each tag. U minus R
excludes wrong sorted multisets, ranks, strict lengths, distance matrices,
reachability partitions, selected-edge accounting, prefix reconstructions,
range minima and restricted square/root identities.

The complement is an exact specification target with only partial source-derived
Rust correspondence. Child agreement alone is weaker: implementations can share
a defect, and connectivity alone cannot certify an optimum forest. Independent
leaf oracles supply separate finite evidence. Errors, allocation failures, panics,
timings and arbitrary client programs are outside this successful-observation U.

## reverse-reconstruction

Type: reverse_reconstruction. For the sort/search tag, observe sorted S, valid
cut k and optional first match h. Retain all original sequences A with S's multiset
and targets t such that S[i]<t for i<k and S[i]>=t otherwise; h=Some(k) iff
k<len(S) and S[k]=t, otherwise h=None. Inconsistent observations have empty
preimage. For empty S, k=0,h=None every target remains possible.
This precisely scoped preimage does not uniquely recover original order.

Other tags retain all intermediate values and identities satisfying their forward
equations: matrices/partitions admit multiple graphs, and selected forests lose
discarded edges and histories. Complete valid prefix observations are injective
on value sequences through adjacent differences, but still lose update history;
range minima are lossy. Floor roots and modular residues generally have many
antecedents. The restricted square/root join restores x only after its explicit
square/no-reduction premises. Parent catalog entries state those additional
preimages; no general inverse execution engine is implemented.

## sorting-search-rank

Type: rank_transport. Original occurrence count strictly below t survives sorting.
It equals lower_bound(S,t); original membership chooses Some(count) or None.
Duplicate multiplicity matters. Existing fixtures cover all 364 sequences of
length 0..5 over three values and five targets, for 1,820 query observations.
The oracle counts unsorted input rather than searching a sorted oracle output.

## unit-weight-distances

Type: representation_transport. Replacing every edge by weight one preserves
paths and makes their total cost equal their edge count. Dijkstra embeds BFS
distances in u128 and preserves None. This does not invert arbitrary weights.
Existing fixtures cover 530 directed graphs at V=1..3 from every source,
1,570 source observations, with self edges allowed. An unreachable malformed-edge
fixture separately checks both APIs reject; distinct error enums are not equated.

## partition-reachability

Type: partition_observation. Unioning every undirected edge generates exactly
undirected path reachability. connected agrees with BFS and components counts
equivalence classes. Raw representatives and compression history are unobserved.
Existing fixtures cover all 75 simple undirected graphs at V=1..4 and all pairs,
with class count determined by absence of an earlier reachable vertex.
Empty-universe and invalid-index state frames remain leaf checks.

## Checks and limits

The source-bound syntax omits macro expansion, cfg evaluation, external module
loading and semantic resolution. Integration refreshed this source hash
and syntax after final code edits, and the current parent binds all 208 source leaves: one hundred implementations, one hundred test modules, this boundary and seven external harnesses. The branch declares 1,259 leaf views and 75 parent views.
A registration or structurally valid report is not a proof of behavior or a
full-workspace test result.

## New fixture domains

The integration source defines ordering inputs over the ternary alphabet at lengths 0..5; directed graph masks at 0..3 vertices with weights (u+v) mod 3; undirected graph masks at 0..4 vertices plus zero self loops; prefix/sparse snapshots at lengths 0..12 for three valid update rounds and all half-open ranges; and checked squares for bases 0..256 plus 65535, 2^31 and u32::MAX with modulus square+1. The square test also checks a reduction counterexample. These code-defined bounds were executed in the integrated debug/release suite. The prefix fixtures compare direct sums and minima, including empty-range sum zero and None minimum; rejected-update state frames remain leaf evidence.

## Wave16 final source receipts

See [wave16-evidence](model.md#wave16-evidence) for final source/syntax hashes and repeated full-crate execution. Earlier epochs remain historical.

## wave14-evidence

### Wave14 library boundary

464 algorithm tests (416 unit, 47 external, one doc) and 101 selected Mova checks pass twice in debug and twice in release with identical normalized named outcomes. Mova checks comprise the previous 75 and 26 reached checks: eval 4, data 6, frames_md 7, tune 4 and five new joins. These selected Mova checks do not represent the complete Mova suite.

Ten new algorithm production mutations and eight false-oracle mutations fail named assertions in owner receipts. Four Mova production mutations fail named assertions. Both root adapters compile and fail their intended named assertions: zeta_product_inverts_to_or_convolution (wrong inverse stage) and batched_items_keep_source_identity_through_classification (wrong source window). Exact source bytes are restored before positive runs. Compilation failures do not count as semantic rejection.

Strict all-target algorithms Clippy and algorithms formatting pass. Historical world/prag warnings remain. No benchmark, throughput, GPU, teacher/parser acquisition, full-school run, held-out accuracy, persistent adoption or general proof is claimed. Tiny supplied-input fitting exercises actual classifier calls. Tuning validation participates in candidate choice and is not an untouched final test.

## wave16-evidence

### Wave16 library boundary evidence

The complete 100-algorithm crate passes 504 checks (448 unit, 55 external composition, one doctest) twice in debug and twice in release. The 155 selected Mova checks also pass twice in each profile, with identical normalized named outcomes: 139 previous focused checks, 11 new unit checks, two existing fbench/silver tests and three new integration joins. This is a selected Mova suite, not full-system validation.

Ten new algorithm implementation mutants and two Mova production mutants fail named assertions in owner receipts, with exact restoration before positive runs. Root additionally compiles two adapter controls: wrong forest weight/cardinality correction and swapped one-based silver-row identity. Both fail their named assertions, then exact source bytes are restored. Compilation failure is not counted as semantic rejection.

Mova fixes bound duplicate-link credit by gold occurrences (multiset intersection) and retain context unless adjacent paragraphs share source/document identity and checked consecutive paragraph numbers. Old-code witnesses cover duplicate overcredit, cross-source suppression and overflow. Three root joins exercise notation/alignment/counts, row/item/class transport and batch/prompt coordinates. Unique-link semantics and public APIs remain unchanged.

Independent read-only review found no additional concrete defect. Coverage limits: graph/tree cap+1 checks do not establish maximum-size successful throughput; Li Chao scans an independent line envelope but shares widened arithmetic and value/ID ordering; the connectivity cap fixture does not stress worst-case interval storage. A 64-vertex clique representation cap does not make exponential search cheap.

The ten new implementations are exact within their declared domains; no new approximate Rust surrogate or Lean proof is asserted. No live parser/teacher/corpus acquisition, GPU, whole-school throughput or persistent-learning adoption was executed. No general semantic proof or training-speed gain follows from these checks.

## wave18-evidence

### Wave18 algorithm evidence

The library contains 110 original deterministic algorithms. This epoch adds biconnected components, DAG path counts, Euclidean floor sums, segmented sieving, bounded Pollard rho factorization, Two-Way byte matching, Needleman–Wunsch alignment, canonical run-length coding, a wavelet matrix and a persistent segment tree. Every implementation and test source has an adjacent groupant. Standard library only; no imported problem statements or solution code.

Final root validation: 550 checks (489 unit, 60 external integration and one doctest) passed twice in debug and twice in release, with identical normalized named outcomes. Five new joins exercise all ten APIs; there are now 15 internal and 60 external composition checks. These relations retain edge IDs, bytes, arithmetic domains, half-open ranges and immutable versions.

Owners recorded ten compiled production mutations, each rejected by a named assertion and then restored. Root additionally changed the alignment adapter's mismatch score; its intended score assertion failed, then exact restoration passed. Compiler failures are not mutation rejection. Full root validation was repeated after three test-only Clippy refinements; owner receipts remain historical. Strict all-target algorithm Clippy and formatting pass.

Independent review found no concrete defect in the inspected domains. Finite oracles include 64,897 byte-search pairs, 14,400 alignment/path cases, 9,841 ternary RLE strings, cycle/path enumeration, direct floor sums, trial division and literal array snapshots. Large factorization checks partly reuse Miller–Rabin; small trial division and literal products provide separate evidence. Rho budget exhaustion is explicit. Alignment rejects any overflowing candidate score, including a losing candidate. Some allocation failures are outside typed error contracts. These are not universal proofs or new approximation-error guarantees.

CPU results in wave18-benchmark.md retain improvements and regressions from two executions. They do not establish Mova training, school or GPU throughput.

## wave26-evidence

### Wave26 evidence — completed UTC

Focused Mova checks pass 30 tests in each profile twice: conversion 19, synthetic CoNLL-U four, functional-graph/Doc/gate joins five and annotation/conversion joins two. These scopes reuse unchanged conversion receipts and the initial successful CoNLL-U debug run; every reused process had exit zero and its normalized names were checked. The real-treebank test was explicitly skipped. Strict all-target algorithm Clippy and changed-source rustfmt passed. Whole-workspace Clippy and live corpus, teacher, training, school, GPU and held-out evaluation were not run.

The final arborescence implementation fixes repeated completed-path traversal. On the independent nested-contraction family at V=257, primary parent advances decrease from 1,065,024 to 24,640 with the same objective. Each nonroot parent advances once per contraction; cycle labeling touches disjoint cycles. O(V+E) per round gives O(VE+V²) overall. Counts are not timings. Independent review also checked matching, exact cover, arborescence, functional graphs, HMM/covariance, ordered indexes and extreme coordinates; finite oracles do not establish universal correctness.

Thirty retained compiled controls are audited separately: 25 new kernels, the arborescence traversal repair, two CoNLL-U repairs and two composition adapters. Each intended assertion fails in compiled code and exact restoration passes. The rejected conversion candidate has its own checks and is not counted as adopted production. Structural negative controls reject a stale source binding and nonexistent child, restore exact bytes and pass again. Five existing transfer records remain source/model-bound; catalog validation is structural.

Models remain partial. Source hashes and syntax projections bind bytes, without macro expansion or semantic AST proof. Numeric residuals and estimators do not imply a universal 1/2/5/10% solution-error certificate. No aggregate Mova speed or quality gain follows from the library tests.

Receipt precision: all 30 compiled controls show named assertions and positive restorations. Numeric exit101 is recorded for 23; seven combinatorial/repair/harness runners retain non-success without the numeric code. No code is inferred for those seven; see controls-audit.json and wave26-validation.md.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

The earlier integrated suite passed 54 unit tests and a doctest, including three original composition checks. Five further composition families pass within the integrated 97-test debug/release suite; the parent records execution and the rejected prefix adapter mutation. Connectivity is weaker than forest optimality. Equal sorted values do not imply stable payload identities. Prefix differences recover a complete current value sequence, while minima, residues and update histories remain ambiguous. A source hash for lib.rs does not bind external implementations or oracle modules.

### 2026-10-29 · Checks and limits

Historical evidence: on 2026-10-29 the integrated suite passed 97 unit tests and one doctest in
debug and release, retaining the earlier 54 tests and adding 38 leaf tests plus
five composition tests. A compiled prefix adapter mutation is rejected by its
exact selected test. Parent tests.md records the execution evidence.
Their specialized models are in the parent catalog under Ordering structure join,
Weighted reachability join, Spanning forest accounting, Prefix difference
reconstruction and Restricted square root.

### 2026-10-30 · descriptant

#### Modeling epoch 2026-10-30

Seven more joins retain signed-distance conversions, alternating matching paths, delimiter/byte and palindrome boundaries, current range snapshots, integer spans and prime-domain assumptions. The integrated debug/release suite passes 137 unit tests and one doctest; a wrong exclusive endpoint is rejected by its compiled adapter control. Models remain scoped relations rather than a universal inverse engine.

Historical fourth-batch epoch: ten public exports were added. Its seven external client joins have their own registered source sidecar; this file still owns fifteen internal joins. Root syntax is refreshed after exports change, while that epoch's algorithm-library parent bound 82 descendant sources.

#### Historical sixty-algorithm boundary epoch, 2026-10-30

Sixty implementations and sixty test modules, this library boundary and three external harnesses form 124 bound source leaves. The fifteen internal joins remain here; external joins bring the branch total to 37. Eight newest harness views are maintained by the parent, not duplicated in this boundary model.

Root validation passes 298 unit tests, 22 integration tests and one doctest twice in each profile with identical normalized outcomes, plus strict all-target Clippy and formatting. Ten compiled leaf controls and one knapsack-adapter control reject. These are finite observations, with no GPU or approximation gain inferred.

#### Historical seventy-algorithm boundary, 2026-10-30

Seventy implementations and seventy test modules, this boundary and four external harnesses form 145 source leaves. Fifteen internal joins plus 30 external joins give 45 compositions. Root validation passes 341 unit tests, 30 integration tests and one doctest twice per profile; strict Clippy/formatting pass. The new recurrence-sign adapter fails its named assertion. Parent binds external revisions; the API source hash alone does not.

#### Historical eighty-algorithm boundary, 2026-10-30

Eighty implementations, eighty test modules, this boundary and five external harnesses form 166 source leaves with 1030 leaf views. The parent has 58 views; fifteen internal and 38 external joins give 53 compositions. The eight wave11 joins preserve matching/flow cardinality, SCC/reachability identities, edit-distance inequalities, palindrome occurrence counts, doubled-word factor/rotation structure, recurrence coefficient orientation, carry-free convolution supports and assignment/tour objective restrictions.

Integrated validation passes 382 unit tests, 38 external checks and one doctest (421 total), twice in debug and twice in release with identical normalized outcomes. Strict all-target Clippy and formatting pass. A compiled b[4*i] to b[3*i] adapter mutation fails the no-carry convolution test; exact restored code passes. Repeated source projections and structural checks bind this declared slice, while independent leaf oracles remain necessary against shared algorithm errors. Source registration is not universal semantic verification; this boundary hash alone does not bind external implementations.

#### Historical ninety-algorithm boundary, 2026-10-30

Ninety implementations, ninety test modules, this boundary and six external harnesses form 187 source leaves with 1,145 leaf views. The parent has 67 views; fifteen internal and 47 external joins give 62 composition relations. Nine wave14 joins preserve graph, preference, byte, index and residue identities. External harness bodies remain outside this file's syntax projection; the parent supplies their separate source bindings.

Integrated validation passes 416 unit tests, 47 external checks and one doctest (464 native checks), twice in debug and twice in release with identical normalized named outcomes. Strict all-target Clippy and formatting pass. Ten production controls and eight false-oracle controls reject named assertions in worker evidence. The compiled root wrong-inverse OR-convolution adapter also rejects; exact restoration and positive checks pass. Parent receipts retain execution identities. Finite models and tests do not establish unrestricted correctness, allocation-failure coverage or a benchmark gain.

#### Historical wave16 boundary, 2026-10-30

One hundred implementation/test pairs, this boundary and seven external harnesses form 208 registered source leaves with 1,259 leaf views; the parent has 75 views. Fifteen internal joins and 55 external joins give 70 composition relations. The eight new external relations preserve graph, tree, byte, residue, DP predecessor and timeline coordinates. They are client observations with distinct domains, not one serial execution of all algorithms.

The integrated suite passes 504 algorithm checks twice per debug/release profile with identical normalized named outcomes. Strict all-target Clippy and formatting pass; [wave16-evidence](model.md#wave16-evidence) records final execution and source bindings. Historical passing epochs above retain their original counts. This file's partial syntax does not load external modules or harness bodies, and its source hash alone cannot bind them; the parent supplies those identities. No speed gain, full-system verification or general inverse solver is implied.

### 2026-10-30 · document

#### Modeling epoch 2026-10-30

The thirty-algorithm suite passes 137 unit tests and one doctest in debug and release: 33 new leaf tests and seven new joins extend the prior 97. Two serial release runs have identical normalized result records. A compiled exclusive-end adapter mutation is rejected by the range-snapshot equality assertion. Source syntax and all transitive parent bindings were refreshed after formatting; see parent tests.md for evidence and limits.

#### Historical eighty-algorithm boundary, 2026-10-30

The eighty-algorithm boundary owned fifteen internal composition tests; five external harnesses owned another 38, giving 53 composition observations. The parent bound 166 source leaves with 1,030 leaf views and 58 parent views. External harness bodies were not loaded by this file's partial syntax projection.

That integration passed 382 unit tests, 38 external checks and one doctest (421 total), twice per debug/release profile with identical normalized outcomes; strict all-target Clippy and formatting passed. The compiled b[4*i] to b[3*i] adapter mutation failed the no-carry convolution assertion and exact restoration passed. Detailed historical domains and receipts remain in the parent evidence.

#### Historical ninety-algorithm boundary, 2026-10-30

The public boundary exports ninety exact modules and retains fifteen internal composition tests. Six external harnesses own 47 further checks, giving 62 composition relations. The parent binds 187 sources: ninety implementations, ninety test modules, this boundary and six harnesses. The branch declares 1,145 leaf views and 67 parent views. Neither this source hash nor its partial syntax projection loads or binds external module bodies by itself.

Root checks pass 416 unit tests, 47 external checks and one doctest (464 native checks), twice in debug and twice in release with identical normalized named outcomes; strict all-target Clippy and formatting pass. Ten worker production controls, eight false-oracle controls and the root wrong-inverse OR-convolution adapter reject named assertions. Restored positives pass. Each adapter retains its own domain and ambiguity; no benchmark, new approximation budget, GPU result or general proof is claimed.

#### Current wave16 boundary, 2026-10-30

One hundred implementation/test pairs, this boundary and seven external harnesses form 208 registered source leaves with 1,259 leaf views; the parent has 75 views. Fifteen internal joins and 55 external joins give 70 composition relations. The eight new external relations preserve graph, tree, byte, residue, DP predecessor and timeline coordinates. They are client observations with distinct domains, not one serial execution of all algorithms.

The integrated suite passes 504 algorithm checks twice per debug/release profile with identical normalized named outcomes. Strict all-target Clippy and formatting pass; [wave16-evidence](model.md#wave16-evidence) records final execution and source bindings. Historical passing epochs above retain their original counts. This file's partial syntax does not load external modules or harness bodies, and its source hash alone cannot bind them; the parent supplies those identities. No speed gain, full-system verification or general inverse solver is implied.

### 2026-10-30 · wave14-evidence

Raw receipts: $MOVA_DATA/runs/octopus-wave14-integration-2026-10-30. native-evidence-hashes.json SHA-256 d95a91417adeb7f8e755d26f900b6508ea17759291407f33153917a42396280f; algorithm-adapter-mutant.log d2279a28cc474377b1d9321e15d4c779b0af8507f81ac39e83338bc9084a02a2; mova-adapter-mutant.log 60567fa4d984ac58834be268cdb221a37caf30bd5d07e9a3d6a2d89c16fdac87; mova-all-normalized.txt fb492ea8097fd74fe50cdceb5d61cb49efc74baa6cb152f7699e546dcbb3ac20; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. The structural controls reject stale parent-source bindings and wrong child identities, restoring exact bytes. Source syntax remains partial, with opaque macros and no semantic AST.

### 2026-10-30 · wave16-evidence

Raw receipts: $MOVA_DATA/runs/octopus-wave16-integration-2026-10-30. native-evidence-hashes.json SHA-256 f93920d688e961ce9dd02be865389cbe54db3093416464e379c650f16578841d; algorithm-normalized.txt 1335d34b718e04458dac54697f8822eb8f02b0f408dbb547852dfaa2d60a7164; mova-normalized.txt f0ca49c0b0f12e5febe6e41657b4d098e8918171918f8ae53bd96868ac5f9328; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Structural controls reject a stale parent source hash and a wrong child identity, then restore exact bytes. Owned partial syntax projections repeat identically; macro bodies remain opaque. Owner receipts remain adjacent to sources; formatting-only followups retain original owner native hashes, while root repeats bind final bytes.

### 2026-10-30 · wave18-evidence

Heading `### Wave18 algorithm evidence — 2026-10-30` stays in wave18-evidence as `### Wave18 algorithm evidence`.

#### Wave18 integration scope — 2026-10-30

Base ae72362aaa65659aa444cb7d090a05636c0e0691. Inventory: 353/784 Rust files registered, 431 pending (348 public, 83 closed); 696 nodes, 16 groups, 327 fragments, 3,899 typed views, 1,196 labels and 343 partial syntax projections. Added 24 registered file entries: 20 new algorithm implementation/test files, two existing qagen sources and two new harnesses; 13 fragments and 223 views. The library export source also changes, giving 25 changed Rust files.

Declared groupants pass structural validation. Repeated syntax projections match byte-for-byte. Injected stale bindings and wrong child identities reject, then restore exactly. Root corrected two imported file-node child lists and 32 graph heading references found by integration validation. These are model-wiring repairs. Source binding and section validation do not prove semantics, expanded ASTs or a complete dependency closure.

Final receipts cover 550 algorithm checks and 55 focused qagen checks, each twice in debug and twice in release. Unrelated Mova subsystems and the whole school were not rerun. No numerical approximation tier is introduced. Complement models and compatible-antecedent models remain distinct, with explicit information loss.

Raw receipts: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30; final native logs in native-final/. Prospective models and prior dated evidence remain. Question generation belongs to school-support; no inference-core or live acquisition pipeline is introduced.


##### Receipt bindings

External run: $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30. Native evidence index SHA-256: f85b3e074aa997321aff915acd6068610e33c9e50322525af7b6f529a7d7f422. Algorithm normalized outcomes: a307113f556e3e4dab5bf677df657899f439ffab39a1c8e1113ed23c97c1c733. Qagen normalized outcomes: 34efc5330be15215be0220412a9af2a517680394f975a7011f75a799a5210d21. Structural controls: a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Benchmark summary: 8ad77b5c224d73c393111ef5f6e14de3bb1d2b8108f5f01a89f90eee8d4a3c20.

Current library source SHA-256 3998124bea6b8140fdec62cc7581261790f6bba319efd6100aa622f160abf5e3. Partial syntax SHA-256 26c2c47f21e8b21e3c9d788bd98a5f6d4a3e10dbac7bd0e77f3ccf44fc39eb66. External modules remain separately bound by the parent.

### 2026-10-30 · wave26-evidence

The task epoch retains its 2026-10-30 label. The library now exports 200 algorithms in separate directories. Its final state passes 877 tests twice in debug and twice in release, with identical normalized outcomes: 785 library tests, 91 external composition tests and one doctest. The 15 internal composition tests are included in the 785; the total of 106 composition observations must not be added again. All 782 previous test names remain; 92 source-local and three transform-composition tests were added.

Receipts: $MOVA_DATA/runs/octopus-wave26-integration-2026-10-30/{native-algorithms-summary.json,native-mova-summary.json,controls-audit.json,structural-controls-summary.json,catalog-audit.json}. Worker receipts are in the corresponding wave26-{indexes,linear,combinatorial,transforms,probability,mova} runs. Helpers preserve early failed orchestration checks separately: reading an incomplete log and an off-by-one provisional expected count were corrected before final validation; no failed native test was relabeled as a pass.

### 2026-10-31 · wave26-evidence

Heading `### Wave26 evidence — completed 2026-10-31 UTC` stays in wave26-evidence as `### Wave26 evidence — completed UTC`.

### Undated · wave14-evidence

Source SHA-256 a264338cbb0cf4ea813ce974a0e8ba79aad0b3d394e69f0692a2371e6d878e10; twice-identical partial syntax SHA-256 5e5c40901d130bd824444d113c06ad2d96cd755a4f2a661e514456078f3a03c8. Fifteen internal joins remain owned by this source; 47 external joins belong to six separate harness sources.

### Undated · wave16-evidence

Final source SHA-256 dea495c0a93b5766acb236692dc827db2db4589b423cbb46be5535064feaae69; twice-identical partial syntax SHA-256 ed7d6d1cfd82b40e407400ec0f434ea3b0b75419819a329c93d2cae6c6b0b3da. The 15 internal joins are included in 448 unit tests; seven external harnesses add 55 joins, giving 70 composition relations.
