# world-spartqa

## model

# SpartQA inference transfer model

## behavior-complement

Observation universe: an in-memory Scene, query arguments, return value and unchanged scene after successful breach, holds or dir_path. Initial object IDs used by holds are valid; intermediate fact IDs need not occur in objs. Resource exhaustion and panicking initial invalid IDs are outside this successful-call relation. Exclude a changed Scene, directional inference without a compatible path, missing compatible paths, symmetric non-directional block inference, or loss of a qualifying alternate path. holds(x,r,x) remains false even for cycles; breach(a,r,a) requires a nonempty cycle or self-edge. dir_path may return a result on a nonempty return path. Reader, JSON/TSV IO and external category data are explicit unmodeled gaps.

## reverse-reconstruction

For directional holds with equal IDs, any otherwise valid Scene is compatible with false. For distinct object blocks, compatible antecedents have (true) or lack (false) a block path. Within a block or without both blocks, use object facts instead. For non-directional holds, compatible antecedents have the symmetric explicit fact, or distinct known blocks for Far. For dir_path=None there is no nonempty projected path. Some(false) requires a path but no path with an explicit Far edge or at least two fully qualified edges. Some(true) requires such a witness. Unrelated facts and object attributes can vary, so inverse recovery is nonunique. Finite fixtures execute representative fibers; no general preimage enumerator is claimed.

## forward

Project (a,rr,b) to a->b when rr=r, otherwise b->a when rr is directional and inverse(rr)=r. Preserve BTreeSet order per source row, including duplicate destinations from distinct facts. breach performs boolean reachability. holds retains self exclusion, symmetric non-directional facts, Far across distinct blocks and block delegation. Direct-edge membership is an exact boolean shortcut. Fact sets with at most 64 entries retain the original scan expansion through an allocation-free iterator enum. Larger sets preflight outgoing edges from the initial source with the same forward/inverse predicate; absent sources return false/None without rows. Otherwise use ordered adjacency. Threshold 64 is prospective, not a fitted optimum. dir_path traverses (vertex, far_so_far, min(steps,2), all_steps_qualified). Qualification means symmetric explicit Near, Far or Touch; Far accumulates by OR and qualification by AND. Target visits update a global OR without stopping other paths.

## invariant

Rows enumerate exactly the old scan's accepted destinations in encounter order. Queue entries represent attainable walks with accumulated state. Visited states prevent repeated expansion; root handling is unchanged. The product has at most 12 states per vertex. Facts and qualifiers are immutable during queries; indexes cannot become stale because they are built per call.

## cost

Let E be all facts, A projected edges, V incident vertices and U reached vertices. Building BTreeMap rows costs O(E log(V+1)) time and O(V+A) memory. Boolean traversal costs O((U+A) log(V+1)); direct successes cost O(log(E+1)). Product traversal has at most 12V states and 12A edge visits, plus BTreeSet qualifier probes O(log(E+1)) per transition. Old scans cost O(UE) or O(12VE), plus state/qualifier operations. The hybrid retains old scan costs for E<=64. Larger sets add O(E) outgoing-source preflight before row construction; an absent source costs O(E) and allocates no rows. Active shallow traversals near the threshold may still lose to construction overhead. Qualifier checks retain O(log(E+1)) per membership probe on both branches. No timing gain is claimed before isolated measurements. Reader costs are outside this scoped model.

## projection-fiber

Type: reverse_reconstruction. For fixed r and projected rows, compatible facts include forward r facts, inverse-direction facts mapping to those edges, and arbitrary other relations. Qualifiers are additionally constrained by dir_path. Replacing Left(a,b) with Right(b,a) preserves directional observations, though row order can change.

## product-state-sufficiency

Type: state_abstraction. Future transitions depend only on vertex, Far flag, capped steps and all-qualified flag. Dropping qualifier or Far bits merges observably distinct states. Stopping at the shortest target path is unsound: an unqualified direct edge can coexist with a longer fully qualified path. Independent fixed-point checks and a compiled qualifier mutation exercise the boundary.

## composition

Projection supplies rows to reachability and product-state children. holds selects block versus object domain before traversal. Correct rows alone do not establish qualifiers, cross-block behavior or output OR. Parsing/reading and external dependencies remain missing higher-level evidence.

## checks

Synthetic scenes: 1,024 three-object labeled graph/qualification fixtures across relations and endpoints, 64 block graph masks, inverse and qualification controls, row encounter order, and existing six reader tests. Debug/release repeats and compiled mutations are recorded in model.md#evidence. Ignored benchmark compares actual methods to old loops and is excluded from routine tests.

## epoch-memory

Prospective prediction: adjacency reduces repeated scans on long paths; construction may regress tiny, dense or missing-source calls. Direct membership was specified before implementation; no empirical threshold. Finite observations do not prove universal semantics or runtime gains. BFS distance API was rejected because it erases product-state witnesses and adds unrelated global validation errors.

## refinement-epoch

## evidence

### Evidence

#### Initial eager-adjacency epoch

`cargo test -p world --lib spartqa:: -- --test-threads=1` passed twice in debug and twice in release: 11 passed, 1 ignored. The same scoped debug run passed after exact restoration of both compiled mutants. Test-name/result lines are identical across all five final logs; compilation diagnostics and elapsed times are not compared. Existing dependency warnings remain. New tests include 1,024 three-vertex labeled fixtures, 64 block masks, inverse/qualifier controls, row ordering and dangling intermediate IDs. Six preexisting reader tests also passed. No live corpus, parser model, teacher, database, GPU or full school was exercised.

The inverse mutant disabled reverse-direction adjacency while retaining direct lookup. Dedicated control failed with Some(false) instead of Some(true). The qualifier mutant replaced accumulated qualification AND with OR; the same control failed with Some(true) instead of Some(false). Both compiled and failed test execution with exit 101. Original production bytes were restored and compared exactly before the final green run.

Two independent syntax generations agree byte for byte for each owned Rust source. Projections are partial Syn syntax, with no macro expansion, name resolution or semantic AST. File and scoped child declarations passed their local checks in an external combined registry; the combined groupant run retains only expected parent wiring/transitive-binding diagnostics until integration updates the authoritative registry and parent models. Registry entries supplied here keep children=[] for both files. Missing legacy coverage remains explicit.

Raw external log hashes:

The ignored `spartqa::inference_transfer_tests::benchmark_transfer` invokes actual production and retained baseline methods with equality-checked checksums. It covers tiny/long chains, dense direct queries, large direct queries and absent sources, separately for holds and dir_path. It has not been timed by this implementation worker; centralized isolated measurements are required before a speed claim. Eager adjacency construction has an explicit regression risk for small or absent-source traversals.

#### Hybrid refinement epoch

A centralized exploratory old-first pilot found eager building slower for missing-source queries (27.6/30.7x) and tiny chains (2.43/1.41x), respectively holds/dir_path. This pilot motivated model.md#prospective-refinement before code changes. It is not final timing evidence. The hybrid retains scan expansion at <=64 facts and performs a compatible-outgoing-edge preflight before row construction on larger graphs. Threshold 64 is not claimed optimal.

Final hybrid scoped tests passed twice in debug and twice in release: 12 passed, 1 ignored. Both compiled mutations were caught by hybrid_boundary_and_indexed_controls, then exact original bytes were restored and the complete scoped debug run passed. The inverse mutation disables only indexed inverse rows. The qualifier mutation changes AND to OR only when fact count exceeds ADJACENCY_SCAN_LIMIT; fallback states retain the original AND, so its failure cannot be attributed solely to tiny fixtures. Boundary fixtures at 64,65,96 facts cover inverse paths, alternate qualifiers, block projection and absent-source results. Original exhaustive small fixtures now explicitly cover the scan branch.

Fresh partial syntax projections agree byte for byte across two generations for each source. Fragment scopes and manifests bind the current source. External combined-registry groupant check again reports only integration-owned parent children/transitive-source binding gaps, no new file or fragment diagnostics. No production timing was run by this worker. The revised ignored benchmark has three warmups and seven alternating-order samples, including 64/65 boundary workloads; centralized quiet-window measurement remains the speed gate.

## prospective

### Prospective inference adjacency transfer

Prediction: replacing each frontier's full fact scan with one per-query adjacency build reduces long reachable-chain work from O(VE) to O(E log V + V log V + E). Product-state traversal retains at most 12 states per vertex. Memory rises by O(V+E). Initial direct-edge membership fast path in boolean searches avoids allocation for immediate answers; no threshold selected. Tiny, dense, absent-source and direct-target workloads may regress or expose build overhead. Gate based on synthetic measurements; reject or refine if regressions dominate useful cases. No timing claims before isolated measurements.

Validation: old production loops retained externally and as test references; independent finite reachability closure and product-state fixed point on small graphs; inverse-only edges, cycles, mixed qualifiers, direct-vs-long alternate paths, cross-block semantics. Run scoped debug/release tests twice. Compile inverse-edge and qualifier mutants, require targeted failures, restore exact source. Produce ignored actual-production/reference benchmark covering chain, dense, tiny and direct workloads, but do not run timing concurrently with builds. No live models, corpus, teachers, database or GPU.

## prospective-refinement

### Prospective refinement

The centralized exploratory eager-adjacency pilot observed missing-source calls 27.6/30.7 times slower and tiny-chain calls 2.43/1.41 times slower (holds/dir_path). The initial epoch and its pre-edit prediction remain preserved. Pilot order was old-first; these measurements motivate refinement and are not final benchmark evidence.

Before refinement: fact sets with at most 64 entries will keep scan-based frontier expansion. Larger sets first perform an O(E) outgoing-edge preflight using the exact forward/inverse predicate; absent outgoing edges return false/None without adjacency construction. Otherwise build ordered rows. Threshold 64 is a prospective engineering choice, not an observed optimum. Keep direct boolean membership shortcuts. The product-state transition and all-path OR are unchanged.

Use an allocation-free iterator enum to select original scan expansion versus indexed row expansion. Cost: tiny fallback retains O(UE) or O(12VE), including qualifier BTreeSet O(log(E+1)) probes; large active graphs pay O(E) preflight plus O(E log(V+1)) row construction. Absent sources pay O(E), no row allocations. Boundary tests at 64/65 facts and indexed-path inverse/qualification controls above 64 will prevent false test coverage. Existing exhaustive small fixtures remain fallback evidence.

Final benchmark will use three warmups and seven samples alternating baseline/production order. Include tiny, 64/65 boundary, absent-source, long-chain and direct workloads. No timing by implementation worker before central authorization. Run relevant debug/release repeats and compiled inverse/qualifier mutants, then regenerate source bindings and syntax.

## transfer

### BFS adjacency transfer into scene inference

Transferred mechanism: adjacency-only expansion, not the distance API. Destination: `world/src/spartqa.rs`, private `directional_rows` and `Scene::{breach,holds,dir_path}`. Facts project to forward requested-direction and reversed inverse-direction edges. Preserve original ID types and BTreeSet encounter order. No dense-ID remapping, global malformed-edge error or source validation is introduced. Intermediate fact IDs absent from objs remain traversable.

Shortest-distance substitution is rejected. The output includes an all-path qualifier OR. Product state tracks vertex, prior Far, capped steps and all-qualified; an unqualified direct target path cannot suppress a longer qualified witness. Preserve root visitation and positive-length cycles. No school data, model, teacher or GPU is used.

Prospective record predates edits; repository copy is model.md#prospective. Tests compare old loops and an independent fixed-point oracle. Per-call ordered adjacency cannot outlive fact mutation. Its build, map lookup, visited-set and qualifier-lookup costs are included in model.md; allocation overhead and absent-source regressions require isolated measurement.

#### Hybrid adapter refinement

After the exploratory eager-build regression, the adapter now keeps full-set scan expansion for <=64 facts. Larger sets check for an initial outgoing compatible edge before allocating rows. `DirectionalEdges` selects scans or rows without iterator allocation and preserves the same accepted-edge predicate and per-frontier order. This is a production adapter policy around the transferred adjacency mechanism, not a changed BFS or path semantics. Its threshold was chosen prospectively and is not claimed optimal. See model.md#prospective-refinement and the retained initial epoch for the motivation and final measurement plan.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective prediction: adjacency reduces repeated scans on long paths; construction may regress tiny, dense or missing-source calls. Direct membership was specified before implementation; no empirical threshold. Finite observations do not prove universal semantics or runtime gains. BFS distance API was rejected because it erases product-state witnesses and adds unrelated global validation errors.

### 2026-10-30 · refinement-epoch

2026-10-30: centralized exploratory eager-row pilot found absent-source calls 27.6/30.7x slower and tiny chains 2.43/1.41x slower (holds/dir_path). Preserve this failed prediction boundary. Before editing, model.md#prospective-refinement selected <=64 scan fallback and outgoing-source preflight for larger graphs. No claim of optimal threshold or final throughput. Boundary 64/65/96 tests exercise inverse and qualifier observations on both branches; indexed mutations use the new boundary control rather than relying on tiny fallback fixtures. Final benchmark adds three warmups and seven alternating-order samples. The iterator enum changes expansion access only, retaining product-state identity and all-path OR.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Production SHA-256: `8eff3f3d0bbec70e0b834dfb75b50c5016f543b736731ae8caa760c87c7ee647`.

Test source SHA-256: `ff11994eb42f98784a295bceeeb7f09af271914cf053b5487ad1dd9d9a9cc1c6`.

Base: `34811b38001b47df99fcba3e6a98967adccb4b72`.

| Artifact | SHA-256 |
| --- | --- |
| debug-final-1.log | 7841ecda940355549eb20b2a63a0fd49f6ccdaa23b99b6e5e561e32deb2c1caa |
| debug-final-2.log | 8c339658441cba051b6e0418b2c50a6d1af3268087ac88e5b6b08c663de33aa7 |
| release-1.log | b0a14e71ce1beb4a8af1cfda14797fcfc7c07d23a055cd16daf44407365b8c6c |
| release-2.log | 8a6a343a1f4de36bc27b25962c5b450360c8b6de7d8c8a9d5bcc6811b630550b |
| mutant-inverse.log | e1b6c45f488e7b24b50ad87aea0556a026903ca24a9332a39a6d333ccf095812 |
| mutant-qualifier.log | 2c05f9477e3262610faf101b609cb5f638a9b3914dc42e8d1b5fb240fcd84077 |
| restored-debug.log | 0d43b7546e64dae4a2ad49077e6da10e941938b9a0365f7eede9b57ff03ecb2b |

Current production SHA-256: `cea8049e1369f5948363a41d6ae85368faa3b9ede1075e0b8c79e92f79c1aa4d`.

Current test SHA-256: `9585c4e10757c7cf36a8077c45316ecfbbd4ac5253e4c87017a1163cc65c4b93`.

| Hybrid artifact | SHA-256 |
| --- | --- |
| hybrid-debug-1.log | ab658963e3c3728d822adaf50a50c81c5cf8ce32d235916c1d5dae6b5a37720d |
| hybrid-debug-2.log | d8fd8aba8be857998451e92901085b4a25d233a88940a0760f95349e29079bcd |
| hybrid-release-1.log | 6f4e3fac137c8cf7701fb79def161863190bfd1c77ed9f5bbf4bdc55ed681b9c |
| hybrid-release-2.log | 019dfc8869efdfd64f796fc084b8aab127a07260f100466bab6ff75099401e23 |
| hybrid-mutant-inverse.log | 8095ebe4c472fb35db47385c5a1bb2ab6d9e194d3e0cab28b1e52bb4b2974cc7 |
| hybrid-mutant-qualifier.log | 8f2cbabeb10ced2de14f7ed6c15ade3fd5015b3267ad6b68b70ebcad2fe90f31 |
| hybrid-restored-debug.log | e33b722edd11cfe2e2bdb14ad78e5fd5da69002e10780f5a44d010c08b050ccb |

### 2026-10-30 · prospective

Heading `### Prospective inference adjacency transfer — 2026-10-30` stays in prospective as `### Prospective inference adjacency transfer`.

Before production edits. Base 34811b38001b47df99fcba3e6a98967adccb4b72. Preserve exact breach, holds and dir_path observations on valid Scene objects; no source validation or shortest-distance semantics imported from algorithms BFS. Project relation facts to ordered outgoing rows including inverse directional edges. Preserve BTreeSet edge encounter order within each row, and product state (vertex,far,steps capped at 2,all qualified), including all-path OR and cycles.

### 2026-10-30 · prospective-refinement

Heading `### Prospective refinement — 2026-10-30` stays in prospective-refinement as `### Prospective refinement`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

### Undated · transfer

Source: `algorithms/src/bfs/mod.rs`, API `bfs(&[Vec<usize>], usize) -> Result<Vec<Option<usize>>, BfsError>`, SHA-256 `bdaab64e535076072d6c65af132c68a3bc0529d0321aed22c6d9d55998246add`.

Model inputs in `algorithms/src/bfs/mod.rs.groupant/`: `approximant.md` SHA-256 `9944f3c4228568a106703bd10eb0dffe19871724b5c8a47e494fbbdde7f6cbdd`; `invariant.md` SHA-256 `82f181256a3dc9b90ccad91e8f1163de89ff15e189ce13a9c4ce399d4828c688`; `catalog.md` SHA-256 `b2d22bcd01faf4d2589dfca48babf14793503ad46631a8da8a5afd80abfc1453`.
