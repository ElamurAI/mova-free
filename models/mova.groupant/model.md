# mova

## catalog

# mova typed model catalog

All entries are partial specifications with fixed-source and trusted-filesystem assumptions from model.md#core-model.
Evidence is source inspection, not execution of every oracle. Oracles are proposed unless model.md#tests records execution.
ASTs and executable reverse reconstruction are not materialized. Specialized views overlap; counts are not independent proofs.

## absent-registered-success

Type: behavior_complement. Domain/observation: Finite attempted traces in the registered command slice.
Operator/approximation: For U all input/output trace pairs and R actual successful traces, C={t: accepted grade join mismatches learner/assessment identity OR accepted registry has an invalid tree} is a source-derived inner approximation to U minus R.
Limits/assumptions: Only named implemented guards; no complement of all Mova semantics.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Swap assessment identity and duplicate a registry parent; both success observations must disappear.

## compatible-root-observations

Type: reverse_reconstruction. Domain/observation: Command-tagged candidate receipts, placement flags and inventory reports.
Operator/approximation: B(y)={x: R_slice(x,y)} retains all compatible command inputs and intermediate artifacts.
Limits/assumptions: Relational specification, not executable inversion; hashes and totals do not recover unique histories.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Construct distinct compatible inputs with equal score projection; retain both antecedents.

## registered-source-closure

Type: dependency_closure. Domain/observation: Root tree and source bindings.
Operator/approximation: D(mova)=union of registered sources reachable through immediate children; bind exactly D.
Limits/assumptions: Unregistered semantic dependencies remain outside D.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Mutate a descendant byte without refreshing root hash; groupants must reject.

## root-child-partition

Type: ownership_partition. Domain/observation: Registered IDs, source paths and parent edges.
Operator/approximation: Each nonroot node has one parent, root reachability holds, and each leaf owns a distinct path.
Limits/assumptions: Model ownership is not runtime call ownership.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Duplicate a child across root branches and reject.

## study-grade-square

Type: interaction_alignment. Domain/observation: Four grades used by study completion.
Operator/approximation: Join the baseline/candidate learner identity pair across the old/new assessment pair before projecting gain and regression; identities within either pair may coincide.
Limits/assumptions: Individually correct grades do not establish correct wiring; inference semantics opaque.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Substitute an individually valid foreign-learner corner and require rejection.

## placement-study-nonjoin

Type: scope_boundary. Domain/observation: Placement flags and retained-learning artifacts.
Operator/approximation: Current root relation is a tagged union of command families; joins exist within families. No implemented placement-to-study promotion bridge is asserted.
Limits/assumptions: Global/RAM engine and automatic transfer remain intended design.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Require explicit shared artifact identity in any future bridge before adding its join.

## model-evidence-nonjoin

Type: evidence_separation. Domain/observation: Declared check strings and executed receipts.
Operator/approximation: Registry->check-name projection cannot generate execution evidence; verification_inferred remains false.
Limits/assumptions: Structural acceptance does not establish test adequacy.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Put a nonexistent check name in a valid disposable registry and observe no execution.

## assessment-information-boundary

Type: information_flow. Domain/observation: Exercise feedback, frozen IDs and exams.
Operator/approximation: New exercise source IDs exclude previously frozen IDs; isolated exam consumes saved state and question-only input with passive learning counters.
Limits/assumptions: ID exclusion does not prove absence of semantic leakage.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Inject a frozen source ID into a candidate exercise and check rejection.

## provenance-preservation

Type: provenance_constraint. Domain/observation: Packet, review, stage, study and candidate identities.
Operator/approximation: Follow hash-bound edges; candidate prior-state identity must equal baseline identity.
Limits/assumptions: Trusted executable/filesystem and collision resistance assumed.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Alter staged bytes without refreshing receipt and require failure.

## durable-active-projection

Type: state_projection. Domain/observation: Placement policy output.
Operator/approximation: Project independent retained-global and active-eligible flags; activation eligibility does not remove global provenance.
Limits/assumptions: Flags express policy, not actual storage/residency.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Enumerate metadata while varying importance and preserve durable flag.

## candidate-publication-boundary

Type: reachability_boundary. Domain/observation: Recommendation and publication fields.
Operator/approximation: recommended=true does not imply published=true; study completion emits evidence.
Limits/assumptions: Other explicit commands may publish.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Inspect successful finish receipt and canonical artifact hashes.

## state-history-fibers

Type: information_loss. Domain/observation: Learner fingerprints and aggregate scores.
Operator/approximation: Keep all teaching histories compatible with observed projection; do not choose a unique inverse.
Limits/assumptions: Not a claim that every conceivable history collision is realizable.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Vary ignored assessment metadata and preserve identity.

## root-failure-prefixes

Type: failure_prefix. Domain/observation: Serial attempts and artifacts.
Operator/approximation: Failure can leave completed-operation prefixes and partial writes; branch terminal gates determine success.
Limits/assumptions: No root transaction or concurrent snapshot is asserted.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Fail at an output boundary; incomplete artifact presence must not establish completion.

## model-refresh-frontier

Type: change_impact. Domain/observation: Changed registered leaf and ancestors.
Operator/approximation: Structural refresh set is the leaf plus its materialized ancestors; semantic review can extend beyond tree edges.
Limits/assumptions: Refreshing hashes only rebinds bytes.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Change one leaf and inspect stale materialized ancestors.

## slice-coverage-denominator

Type: coverage_projection. Domain/observation: Git Rust set F, registered set D and sidecar set S.
Operator/approximation: Report |D|/|F| and |S|/|F| independently; group counts are a separate axis.
Limits/assumptions: Git-ignored files and non-Rust code remain outside F; nonignored untracked Rust files are included.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Add one visible unregistered Rust fixture and observe missing count growth.

## independent-command-budget

Type: resource_budget. Domain/observation: One selected command branch b.
Operator/approximation: T=T_dispatch+c_b; sum child costs only for an explicitly orchestrated sequence.
Limits/assumptions: Abstract root children do not all run on every command.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Instrument bounded inventory and verify no study/teacher subprocess.

## shared-identity-cost

Type: cost_projection. Domain/observation: Repeated hash joins with byte counts H_i.
Operator/approximation: Sequential hash cost approximates sum_i O(H_i), not O(unique bytes) without a cache.
Limits/assumptions: Filesystem latency and constants unmeasured.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Future Rust instrumentation counts hash input bytes before claiming deduplication.

## root-precondition-meet

Type: precondition. Domain/observation: Composition of branch observations.
Operator/approximation: Require each branch precondition and equality of every actually shared artifact identity.
Limits/assumptions: Future cross-branch joins have unresolved premises.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Combine reports from different source snapshots; refuse a same-snapshot claim.

## fixed-snapshot-repeatability

Type: determinism. Domain/observation: Two inventories with identical inputs.
Operator/approximation: Ordered traversal implies byte-identical reports under fixed registry, Git paths, sources and sidecars.
Limits/assumptions: Concurrent writes break premise; GPU behavior is outside scope.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Run groupants twice to new paths and compare bytes.

## future-combiner-counterfactual

Type: counterfactual_policy. Domain/observation: Hypothetical automatic model-update loop.
Operator/approximation: R_future adds propose->validate->measure->revise transitions to current command slice.
Limits/assumptions: No materialized AST, automatic semantic generator or effectiveness result.
Evidence: Registered child contracts and self-study core model; modelcheck and formal-harness source inspection. Cross-family joins remain unimplemented.
Oracle: Require executable traces and negative controls before declaring this loop implemented.

## observation-indexed-memory-equivalence

Type: observation_indexed_state_equivalence. Domain/observation: registered retention and parse-cache APIs, each with its own observation projection O; these are separate branches, not a serial pipeline.
Operator/approximation: Define s equivalent_O t exactly when O(s)=O(t). Retention canonicalization can erase wire-vector order while preserving the retained fingerprint. Cache parser failure can preserve map O_map while changing attempted-operation counters O_attempts. Thus equal storage projections do not imply equal complete traces. Cache-to-direct parsing equality additionally requires a fixed text-extensional parser and compatible cached entries.
Limits/assumptions: Equality is indexed by a declared projection and compatible domains. No shared global snapshot, identical answer behavior, authentic training history or full learner-cache bisimulation is asserted. A changed valid checkpoint with a refreshed checksum can still be accepted.
Evidence: Native retention and cache fixtures, and the synthetic fresh-process retention probe; source-bound child evidence records exact executed cases.
Oracle: Hold restored semantic state fixed while permuting accepted wire vectors; hold cache entries fixed through a failed parser call while counters advance. Reject unrehashed checkpoint tampering and accept/reidentify the valid rehashed variant.

The current observation-indexed memory view also covers separate score/rank and worker observations: equal score/label projections can erase distinct field identities, while equal ordered-input projections can retain different hidden callback ticket states. Native fixtures establish these bounded examples independently; they do not establish a global snapshot or a composed end-to-end execution.

The local-state epoch adds checked ancestry and tolerant line reconstruction to the registered memory branch. The root keeps artifact recovery, semantic retention and prospective global/RAM policies separate. Query and span finite projections are independent evidence; they do not create a new cross-family learning pipeline.

## Sixty-algorithm and source-coverage epoch

The current tree adds ten exact algorithms, eight native cross-algorithm joins and eight previously unregistered Mova source files. A corpus-statistics subgroup records real helper edges and distinct extraction/token policies. Parent source bindings include all registered descendants. The whole-repository coverage queue records every still-unregistered source; queue entries are pending work, not models or proofs. Native command/world and algorithm joins preserve actual adapter identity, while external parser, live I/O, performance and full-system obligations remain explicit.

## Reading and answer integration epoch

Two new interaction groups organize quantitative reading and answer observations; corpus and inference views preserve actual shared inputs across the ownership tree. Six native joins supplement the new leaf correspondence tests. The source queue records all remaining unregistered files, including dependencies called by these fixtures. Native acceptance, partial syntax and registry completeness on changed files do not imply full-system correctness or retained learning.

## Narrative and school support epoch

Ten more existing Mova sources now have adjacent scoped groupants. Narrative-state models partial time order, frame/world identity and transaction boundaries; school-support keeps sampling, option handling, supplied-tree matching and pragmatic induction as distinct tagged mechanisms. Five supplied-input native joins cross real APIs. File fragments expose remaining parser, persistence, malformed-state and orchestration gaps. Input progress, empty CSV records, invalid restore indices and sign-tail arithmetic receive witnessed regression checks. Model counts and structural validation still do not establish full-system correctness, a global inverse or full-course speedup.

## Seventy algorithms and lexical observations

The library adds exact graph, byte, modular-polynomial and dynamic-programming mechanisms with eight supplied-input joins. Four more Mova source trees model candidate selection, positional answer metrics, expression token identity and bounded lexical graph traversal. A branch's presence does not imply a school/runtime connection. Shared lexical snapshots, coordinate maps, answer order and graph truncation remain explicit dependencies; historical score measurements retain their earlier semantics.

## Eighty algorithms and four supplied-input Mova trees

The exact library adds flow, matching, lowlink components, unrestricted byte edits, palindrome/Lyndon structure, finite recurrence synthesis, XOR convolution, assignment and Hamiltonian cycles. Eight actual API joins preserve their distinct domains; existence of the library is not a runtime school connection. Four new Mova file trees describe configured argument/domain judgments, frame serialization/scoring and OCR primer reconstruction. Native tests follow supplied annotations and parsed observations through actual helpers. Parser/teacher execution, live books and full training remain separate obligations. Declared models retain allocation, identity, ambiguity and error-order boundaries rather than treating finite checks as a proof.

## tests

### Evidence and checks

- groupants twice: valid=true, no diagnostics; 3 group sidecars and 1 missing group; 6 file sidecars and 525 missing file sidecars. Reports were byte-identical.
- All declared source hashes, transitive descendants, child sets, registry references and component paths passed structural validation.
- changed against ab4ba4321: changed=[] and missing=[]; no Rust source was modified by this change.
- A bounded standalone Rust probe checked 20 exact manifest section references in this catalog, one matching heading each, and domain/operator/limits/evidence/oracle fields. Both new catalogs passed (40 entries total).
- git diff --check passed before staging.

These checks establish artifact consistency, not mathematical adequacy. No live teacher, training, full Lean harness or new task-accuracy/performance measurement was run. ASTs and executable reverse reconstruction remain absent. Integration must refresh ancestor hashes after any registered source changes and rerun structural validation.

## core-model

# Partial registered Mova root

## Forward approximant

The intended whole-system state S=(mechanisms K, parameters theta, durable knowledge G, active knowledge H, queues Q, artifacts A) remains a design abstraction. The implemented observation model here is a tagged relation R_slice=R_study union R_knowledge-catalog-policy union R_model-tools-and-twin union R_partial-inference union R_algorithms. Tags preserve which command produced each result. It is not a serial pipeline through the children.

Within self-study, source/review/stage hashes, baseline/candidate identities and four assessment grades have implemented joins. Before/after pairs share exact scored assessments and the four corners use the same baseline/candidate identity pair across both assessments (the two identities may coincide). Recommendation requires new_after>new_before and old_after>=old_before after binding. The knowledge branch includes metadata recommendations and compiled catalog lookups; neither implements the full global/RAM engine. Infrastructure joins registered source paths with Git-visible files and source-bound sidecars, and separately projects source syntax. Inventory inspects declarations without executing their check strings. A separate mathematical-twin branch models prospective runtime evidence and RLS correction. Partial inference registers cached execution, frame adaptation, world queries, fact indexing, answer spans, types and lexical observations, with intervening world construction and reader selection explicit. Value judgment now uses canonical maps and prior-command harm; parallel store construction and filesystem captures retain their own scheduling/snapshot assumptions, so local ordering does not imply whole-system determinism.

There is no demonstrated implemented join from placement flags to study promotion, nor from model declaration to runtime truth. A future root composition must provide explicit shared identities and preconditions before replacing this tagged union by a stronger model.

## Invariants and cost

Declared ownership forms a rooted tree; every root source binding covers one transitive registered leaf exactly. This does not forbid cross-tree runtime dependencies. Branch-local evidence must keep its source snapshot, command and observation scope. A structural-valid report never becomes full-system verification. Candidate recommendation never by itself becomes publication. Policy eligibility never by itself becomes stored or resident knowledge.

For a selected branch b, T approximately equals dispatch+c_b. Explicit sequences can sum child costs subject to shared-cache and concurrency premises; the abstract tree itself is not an execution schedule. Hash work scales with bytes actually reread at joins. No total-system speed or task-accuracy measurement is claimed.

## Descriptant and epoch memory

The following paragraphs retain earlier modeling epochs; their source and algorithm counts are historical. Current counts and evidence are in the latest coverage snapshot and wave report.

Prior registry observations preserved: no complete-system guarantee follows from a partial file set; score changes alone do not prove retained knowledge or improvement. The self-study epoch exposed same-size foreign assessment acceptance under count-only checks, so exact assessment and learner-pair joins remain independent obligations. Metadata policy separates durable provenance from activation and compilation. Infrastructure separates registration, sidecar availability and tested semantics.

This epoch contributes twenty typed views with explicit domains and falsifiable oracles. Their evidence is source inspection/child contracts, except structural runs named in model.md#tests. Remaining unregistered files and the semantic gaps of registered sources stay explicit in the coverage queue and local contracts. Parser/inference correctness, full storage/residency, source namespaces, concurrency, filesystem crash recovery, semantic teacher quality and automatic model generation remain gaps. Hashes assume trusted bytes and collision resistance; validation is not a filesystem snapshot.

The sampled-store epoch connects one abstract carry law to a concrete failed regression and a Rust repair. Changed-model sampled sets now persist and return a single complete selection. The proof is about sets; source tests and measured file operations remain separate evidence. Pointwise delta identity, concurrent store snapshots and full-school performance remain outside this repair.

The semantic-delta epoch adds a derivation branch under partial inference. One stored annotation supplies lexical indexing and packed rule inputs through different projections. A local equality repair preserves lemma/tag changes for future updates; complete delta propagation remains conditional on dictionary and publication invariants. The root retains this actual data edge without inferring that all rule-learning or knowledge-retention paths have been verified.

The algorithm-library branch contains thirty original deterministic algorithms and fifteen explicit composition checks across ordering, graph, range and arithmetic interfaces. These are reusable library/test relations; no automatic school or inference data edge is claimed. Its sixty-one source leaves (thirty implementations, thirty test modules and the library boundary) carry partial syntax projections and 315 typed models; the parent carries twenty composition views.

The model-infrastructure branch now includes executable Rust approximants: twelve source leaves (shared contract, two runners, eight surrogates and the exact RSA reference) and a scoped interaction group. Thirty-two declared profiles cover eight kernels and budgets up to 10%. This adds measured numerical models, not a whole-system approximation. Typed error metrics, fallback costs, finite evidence and conditional propagation are specified in models/rust-approximants.groupant/model.md#model and docs/mathematical-modeling.md.

The executable-groupant epoch strengthens six algorithm families and teacher/assessment boundaries. Temporal modeling found and repaired a decoder that accepted completion before answer or mixed multiple turns. A synthetic local interface probe connects decoded text to draft staging, physical question splitting and grade identities. It does not claim actual learning, unseen assessments or a production end-to-end run.

## Wave27 source-tree expansion

Six partial interaction groups connect fifty existing sources: lexical encodings and trained candidate support; parser/graph/persistence boundaries; English orchestration; CorefUD/resolver/evaluation state; annotation queue/patch effects; and exact/symbolic arithmetic gates. They retain distinct coordinate systems, lossy inverses and effect ordering. Registration now reaches 603 of 975 sources; 372 remain pending. Thirty-three compiled mutants, repeated native suites and an independent crossing-span witness constrain the new claims. See [wave27 evidence](model.md#wave27-evidence). The algorithm library remains at 200 implementations; its later family models supersede the initial thirty-algorithm snapshot above.

## Wave28 mathematical and dialogue branches

Source registration now reaches 653 of 975 files through six additional groups and 81 scoped fragments. Formula syntax and Expr transport feed different gates; calc::E is a separate representation. Actual-source observations show that representation roundtrip and finite derivative samples do not preserve numerical definedness by themselves. The linear-system verifier now checks shape before substitution. These local repairs/observations refine the tree without certifying unexecuted training, live effects or the remaining 322 sources. See [wave28 evidence](model.md#wave28-evidence).

## Wave29 knowledge and world branches

Source registration reaches 703 of 975 files through six added groups and 73 scoped fragments. Graph snapshots preserve the tested arrays but discard Kb layers/defaults; shape checks do not imply incoming/outgoing consistency. Heap/local identifiers need a namespace adapter before ontology indexing. Counterexamples drove two checked-integer repairs, while reviews refined layer, finite-score and absent-key premises. The model tree records real joins and separate tagged paths; it is not an execution schedule or a complete proof. See [wave29 evidence](model.md#wave29-evidence).

## Wave30 neural and computing branches

Column-major matrices join parser/indexing, tree or VM execution, numerical builtins and memory VFS. Parse errors precede execution; runtime errors preserve completed effects. File operations have distinct warning, status and exception channels. Neural parsing separates training and inference gradient paths. Reverse models record compatible antecedents after loss of shape, precision, serialization or state. Layers is a tagged sibling, not an invented mlab caller. See [wave30 evidence](model.md#wave30-evidence).

## Wave31 filesystem, runtime and adapter branches

VFS state transitions join path resolution, quota preflight, copy-on-write trees and a separate history journal. WASI translates file errors into a distinct numeric coordinate system; late guest-memory failures can follow completed effects. Rule TSV representation feeds matching and bounded choice lattices; completed duties are removed before final evaluation. Letter tables and corpus normalizers are tagged paths with different normalization domains. Annotation-store process/SQL helpers retain their external database gap. Reverse models retain ambiguity after normalization, dropped history, quantization and representation loss. See [wave31 evidence](model.md#wave31-evidence).

## Wave32 Hebrew segmentation, roundtrip and translation branches

Hebrew text normalization and segmentation transport UTF-8 byte spans and candidate lattice scores; a candidate score gain is not a quality measurement. Roundtrip joins token, tree, frame, world and document representations with residuals that can recover exact target bytes under decoder assumptions. That diagnostic differs from residual-free normal decoding and does not establish persistent learning. Dependency heads, joined-tree offsets and source byte positions have separate coordinate domains.

Translation readers and encoders connect through meta tokens; orchestration preserves the distinction between all target records and paired-source aggregates. Report and tally adapters retain their actual TSV column schemas, population filters and rate denominators rather than treating similarly named rates as interchangeable. IBM1's alignment prior shapes finite alignment scores; it is not evidence of semantic accuracy or translation quality. Source-derived CLI, corpus, annotator/model, training and page paths retain their explicit execution gaps. Reverse models preserve ambiguity after normalization, aggregation and lossy projection. See [wave32 evidence](model.md#wave32-evidence).

## Wave33 public-tool closure and validation branches

Bibliography identity, corpus parsing, teacher diagnostics, treebank comparison and library ingestion now have source-bound model branches. Synthetic parser and temporary-file tests do not establish live download, teacher, corpus or training correctness. Complement and reverse models retain ambiguity from filtering, normalization and lost information. Modelcheck adds scoped representations for inventory, validation, syntax and command dispatch; ordered membership admits an exact binary-search optimization. Bibliography source identity requires the actual HTTP(S) authority, avoiding foreign-host substring collisions. These are local behavioral improvements; source registration alone does not establish whole-system correctness or training throughput. See [wave33 evidence](model.md#wave33-evidence).

## Wave34 nested behavior and composition boundaries

New scoped descendants refine existing school, retained-knowledge, reading/answer, mathematical and validation models. School integer operations and exact calculator values share only their admitted domain; ordered abstention remains a separate decision from computing a value. Lesson editing and assessment fingerprints distinguish teaching-only changes from changes to frozen question identity. A fresh-process checkpoint roundtrip with early exam rejection checks persistence and admission boundaries, not learned answer quality. Reader question classification and answer frames preserve their distinct outputs. Validator artifact reuse requires current byte identity and retains source/range validation; finite report equality is not a universal proof. See [wave34 evidence](model.md#wave34-evidence).

## Wave35 complete registration and scoped composition

A validated base registry can be combined with explicit overlay nodes into one rooted ownership tree. Composition preserves node order/values, rejects ownership/tree conflicts and uses exclusive output creation; it does not execute checks or establish runtime dependency compatibility. Group directories may be explicitly confined within the repository. The public root retains its public ownership boundary. Its explicit cost component links the existing branch-cost model. Full local registration is now accounted for separately from semantic evidence and unexecuted effects; see model.md#wave35-evidence.

## transfer-benchmark

### Historical epoch through fd540 — model.md#transfer-benchmark

#### Retained historical document

### Final algorithm transfer CPU confirmation

The fixed hybrid revisions preserve the checked outputs and improve large-chain and large-residual cases, but do not dominate the old code. Retain the measured regressions when deciding adoption. No further threshold fitting or retiming was performed.

All eight serial invocations passed. Coref ran 2,694 differential cases per invocation. The parser validated 64 shapes × seven alternating-order pairs × two executions = 896 measured pairs. All emitted baseline/production and repeated checksums agree. Annotation/en emit no checksum: their full-result prechecks and every measured unwrap succeeded. Before/after binary and source hashes match.

#### Observed tradeoffs

- Annotation: 128/512/2048 chains improve about 1.67–1.68× / 4.84–4.89× / 18.31–18.39×. Stars and short trees are mostly near parity, with individual regressions retained below.
- Conversion: 128/512/2048 chains improve 1.85–1.87× / 7.11–7.13× / 27.35×. The n65 chain immediately above the hybrid boundary is about 41–42% slower. Large stars are about 7% slower. The n64 chain varies between 0.841× and 0.981× median speedup.
- Coreference: sorted residual groups of 128/129 improve 1.14–1.15× and 256 improve 2.05–2.06×. The n8 same-head scan case is about 6–8% slower, n127 about 1% slower. Assignment 16 improves 1.39–1.41×, 64 improves 1.13×, 32×256 improves 1.07–1.08×, but 128×128 regresses about 1%.
- World: chain 256 improves 10.85–11.01× for holds and 5.02–5.23× for product traversal. Dense-direct holds improves 31.91–32.35× and product about 2.02×. Small/short exploration remains a regression: chain 3 holds about 45% slower; chain 32 holds 13–14% and product 15–27% slower; boundary 65 holds 43–45% slower. Missing-source holds remains 10–15% slower, while its product traversal is near parity or faster. The pilot's roughly 30× eager-allocation regression is gone.

These results justify scoped cost models, not a claim that every call gets faster. A conservative adoption decision can retain the current asymptotic improvement with explicit bounded regressions, or defer an individual replacement if its application input distribution favors the slow cases. No representative workload frequencies were measured, so a whole-application net gain is unknown. Do not turn the largest microbenchmark speedup into a throughput claim.

#### Every measured shape

Speedup = baseline time / production time. Above 1 is faster production; below 1 is slower. The last column is the minimum–maximum of all 14 paired speedups, not a confidence interval. Raw timing medians in nanoseconds and checksums remain in `final-summary.tsv`; all samples and execution order are in `final-samples.tsv`.

| Suite | Shape | Median run1 | Median run2 | All-pair range |
| --- | --- | ---: | ---: | ---: |
| annot | n=128 shape=chain reps=32 | 1.683× | 1.670× | 1.648–1.741× |
| annot | n=128 shape=star reps=32 | 0.996× | 1.001× | 0.987–1.041× |
| annot | n=2048 shape=chain reps=32 | 18.307× | 18.394× | 18.094–18.478× |
| annot | n=2048 shape=star reps=32 | 1.004× | 0.988× | 0.952–1.043× |
| annot | n=32 shape=chain reps=1000 | 1.004× | 1.002× | 0.988–1.025× |
| annot | n=32 shape=star reps=1000 | 1.005× | 1.000× | 0.970–1.039× |
| annot | n=512 shape=chain reps=32 | 4.888× | 4.844× | 4.769–4.914× |
| annot | n=512 shape=star reps=32 | 1.000× | 0.979× | 0.965–1.022× |
| annot | n=64 shape=chain reps=1000 | 1.093× | 1.091× | 1.084–1.130× |
| annot | n=64 shape=star reps=1000 | 1.001× | 0.966× | 0.931–1.021× |
| annot | n=65 shape=chain reps=1000 | 1.095× | 1.095× | 1.078–1.116× |
| annot | n=65 shape=star reps=1000 | 0.990× | 0.984× | 0.953–1.025× |
| annot | n=8 shape=chain reps=1000 | 0.999× | 1.003× | 0.985–1.225× |
| annot | n=8 shape=star reps=1000 | 0.999× | 0.997× | 0.976–1.035× |
| coref | assignment n=128 m=128 reps=100 | 0.991× | 0.994× | 0.977–1.005× |
| coref | assignment n=16 m=16 reps=100 | 1.389× | 1.405× | 1.380–1.445× |
| coref | assignment n=32 m=256 reps=100 | 1.083× | 1.072× | 1.063–1.107× |
| coref | assignment n=64 m=64 reps=100 | 1.133× | 1.129× | 1.074–1.201× |
| coref | matching n=1 mode=0 reps=1000 | 1.102× | 1.099× | 1.062–1.132× |
| coref | matching n=1 mode=1 reps=1000 | 1.093× | 1.105× | 1.045–1.121× |
| coref | matching n=127 mode=0 reps=30 | 0.991× | 0.988× | 0.987–0.996× |
| coref | matching n=127 mode=1 reps=30 | 1.003× | 1.002× | 0.965–1.047× |
| coref | matching n=128 mode=0 reps=30 | 1.144× | 1.151× | 1.142–1.156× |
| coref | matching n=128 mode=1 reps=30 | 1.002× | 0.999× | 0.980–1.028× |
| coref | matching n=129 mode=0 reps=30 | 1.144× | 1.148× | 1.136–1.154× |
| coref | matching n=129 mode=1 reps=30 | 1.028× | 1.010× | 0.966–1.063× |
| coref | matching n=16 mode=0 reps=1000 | 0.986× | 0.995× | 0.980–1.009× |
| coref | matching n=16 mode=1 reps=1000 | 0.980× | 0.981× | 0.942–1.024× |
| coref | matching n=256 mode=0 reps=30 | 2.053× | 2.056× | 2.048–2.063× |
| coref | matching n=256 mode=1 reps=30 | 1.010× | 0.998× | 0.976–1.054× |
| coref | matching n=32 mode=0 reps=1000 | 1.000× | 0.998× | 0.988–1.004× |
| coref | matching n=32 mode=1 reps=1000 | 0.993× | 1.000× | 0.969–1.030× |
| coref | matching n=8 mode=0 reps=1000 | 0.930× | 0.944× | 0.915–0.965× |
| coref | matching n=8 mode=1 reps=1000 | 1.006× | 0.992× | 0.929–1.044× |
| en | n=128 shape=chain reps=32 | 1.848× | 1.866× | 1.787–1.885× |
| en | n=128 shape=star reps=32 | 0.937× | 1.023× | 0.890–1.100× |
| en | n=2048 shape=chain reps=32 | 27.351× | 27.350× | 26.849–27.629× |
| en | n=2048 shape=star reps=32 | 0.934× | 0.934× | 0.925–0.963× |
| en | n=32 shape=chain reps=1000 | 1.117× | 1.097× | 0.967–1.320× |
| en | n=32 shape=star reps=1000 | 1.019× | 1.007× | 0.661–1.030× |
| en | n=512 shape=chain reps=32 | 7.111× | 7.129× | 7.043–7.158× |
| en | n=512 shape=star reps=32 | 0.937× | 0.936× | 0.916–0.952× |
| en | n=64 shape=chain reps=1000 | 0.841× | 0.981× | 0.811–1.009× |
| en | n=64 shape=star reps=1000 | 1.009× | 0.984× | 0.842–1.019× |
| en | n=65 shape=chain reps=1000 | 0.710× | 0.705× | 0.648–0.724× |
| en | n=65 shape=star reps=1000 | 0.949× | 0.941× | 0.927–0.958× |
| en | n=8 shape=chain reps=1000 | 0.980× | 0.980× | 0.980–0.983× |
| en | n=8 shape=star reps=1000 | 0.974× | 0.974× | 0.974–0.978× |
| world | boundary-64 product=false reps=500 | 0.821× | 0.809× | 0.769–0.869× |
| world | boundary-64 product=true reps=500 | 0.765× | 0.776× | 0.704–0.782× |
| world | boundary-65 product=false reps=500 | 0.688× | 0.701× | 0.677–0.730× |
| world | boundary-65 product=true reps=500 | 1.632× | 1.506× | 1.467–1.705× |
| world | chain-256 product=false reps=50 | 10.848× | 11.014× | 10.175–11.766× |
| world | chain-256 product=true reps=50 | 5.231× | 5.018× | 4.940–5.700× |
| world | chain-3 product=false reps=500 | 0.690× | 0.693× | 0.682–0.709× |
| world | chain-3 product=true reps=500 | 0.966× | 0.938× | 0.934–0.966× |
| world | chain-32 product=false reps=500 | 0.881× | 0.887× | 0.795–0.961× |
| world | chain-32 product=true reps=500 | 0.872× | 0.790× | 0.761–0.891× |
| world | dense-direct product=false reps=30 | 31.911× | 32.354× | 26.968–34.799× |
| world | dense-direct product=true reps=30 | 2.035× | 2.018× | 1.969–2.058× |
| world | large-direct product=false reps=300 | 2.187× | 2.208× | 2.078–2.294× |
| world | large-direct product=true reps=300 | 4.012× | 3.973× | 3.850–4.192× |
| world | missing-source product=false reps=300 | 0.911× | 0.867× | 0.774–0.976× |
| world | missing-source product=true reps=300 | 1.009× | 1.166× | 0.896–1.211× |

#### Unmeasured scope

Fixtures are synthetic, deterministic and bounded. The timed seams include their local adapter/allocation work but not parser throughput, live models, teacher calls, corpus/database work, full-school retention, GPU work or total process startup. No representative distribution, peak RSS/allocation counters, randomized uncertainty interval or whole-system metric was measured. Very short direct-edge timings are especially sensitive to timer/compiler/cache effects. Functional checks are finite observations; source-bound models and structural validity do not establish universal semantic proof. Two executions check reproducibility of observations, not immunity to machine noise.

## transfer-evidence

### Historical epoch through fd540 — model.md#transfer-evidence

#### Retained historical document

### Algorithm-to-Mova transfer evidence

Five declared transfers connect BFS to two dependency gates, heap-sort to greedy mention matching, Hungarian resource reuse to assignment, and BFS adjacency to spatial/product-state traversal. The application tree adds nine source groupants, three groups and eight nested fragments. These are explicit model-to-model adapters, not a semantic-search engine. Algorithm ownership remains under algorithm-library; the transfer graph does not invent runtime calls.

#### Checks

The integrated source passed 51 focused checks twice in debug and twice in release with identical normalized observations: annotation 5, conversion 18, coreference 9, spatial inference 12, independent annotation joins 2 and independent coreference joins 5. Each annotation/conversion source-local enumeration compares 126,126 head vectors. The external integrated release coreference runner also passed 2,694 differential cases per invocation. Spatial tests include a fixed-point oracle over 1,024 small labeled graphs and 64 block masks, indexed boundaries and intermediate fact IDs absent from the object array.

Six worker production mutants were caught: two active-BFS direction mutants, coreference tie and scratch-reset mutants, and spatial inverse-edge and qualifier mutants. Root additionally compiled two independent adapter mutants (reversed head projection and integer sign); named assertions failed, exact bytes were restored, and positive named tests passed. Three copied-catalog controls rejected a stale application hash, stale algorithm-model hash and wrong application identity. Catalog checks establish binding and structure only.

All nine current Rust sources have byte-identical paired partial syntax extractions and current source hashes. These projections do not establish complete semantic ASTs. Detailed structural receipts and coverage limits are in [transfer-validation](model.md#transfer-validation).

#### Mathematical obligations and rejected transfers

- Dependency gates: after each caller's own preflight, a token reaches virtual root iff reversed-head reachability marks it. Report the smallest unreachable token, which may be a tail entering a cycle. Empty and self-head policies differ between the two callers. The hybrid keeps direct walks for n<=64 and bounds larger walk work by 4n before BFS.
- Greedy matching: exact spans remain first; total keys (descending overlap, prediction ordinal, gold ordinal) retain repeated greedy selection, including zero-overlap ties. Sorting is admitted only for both residual sides >=128, checked edge count <=262144 and successful reservation. A global assignment replacement is rejected: [[5,4],[4,0]] produces greedy weight 5 versus optimal weight 8.
- Assignment: scratch reuse preserves the existing arithmetic and reset order. Integer maximum/minimum objective observations require exact conversion and sign agreement; arbitrary fractional coercion is rejected. Augmentation scratch is O(r+c); rows>columns also materializes an O(rows*columns) transpose. No universal floating-point optimality proof is claimed.
- Spatial traversal: ordered adjacency retains inverse directional edges. Product state includes far/step/qualifier information and nonempty cycles, so shortest distance alone cannot substitute for all-path observations. Only dereferenced initial query IDs must exist in the object array; intermediate fact IDs may be absent. At <=64 facts retain scanning; larger calls avoid building rows when no outgoing edge exists.

These source-derived obligations plus finite Rust checks support the scoped replacements. No Lean proof or new approximation certificate was produced. The transfers aim to preserve outputs, rather than spend the user's allowed approximation error budget.

#### Measured decision

[The complete timing report](model.md#transfer-benchmark), [all 128 run medians](transfer-timings.tsv) and [all 896 pairs](transfer-samples.tsv) retain improvements and regressions. Eight serial release invocations used three warmup pairs and seven alternating-order measured pairs each; two executions cover 64 shapes. All emitted checksums match, annotation/conversion full-result prechecks pass, and before/after source and binary hashes match. The exploratory pilot was retained externally and used for one refinement before this confirmation.

Retain the bounded asymptotic improvements with their measured tradeoffs: chain validation reaches 18.4x/27.4x, 256-mention matching about 2.05x, and the world chain-256 reachability case about 10.9x. Tiny/boundary cases can be roughly 45% slower, including converter n65 and short world traversals; those regressions remain visible. Representative input frequencies and whole-Mova throughput are unknown. A future dispatch change needs a new model revision and fresh focused evidence.

## transfer-validation

### Historical epoch through fd540 — model.md#transfer-validation

#### Retained historical document

### Algorithm transfer validation

- Native: 51 focused checks passed twice in debug and twice in release; normalized observations match. Two independent adapter mutants compiled, failed named assertions, were restored byte-for-byte and passed their positive checks. Six production mutants were checked in the separate worker revisions that were integrated unchanged.
- Syntax/format: paired syntax extractions match for all nine changed Rust sources; final rustfmt checks pass for those nine sources.
- Groupants: two byte-identical structural reports pass with zero diagnostics, 449 source sidecars and 28 group sidecars. The registered hierarchy contains 344 nested fragments; this counts model scope, not semantic verification.
- Changed-source gate: all nine changed/new Rust files since the base have registered adjacent contracts; missing list is empty.
- Transfer catalog: two byte-identical audits accept all five records. Copied-catalog controls reject altered source hash, model hash and identity with the expected diagnostics. This is structural binding, not semantic proof or discovery.
- Coverage: 449 of 872 Rust sources registered; 423 remain (340 public and 83 closed). The require-all gate exits 1 as expected. Coverage queues were regenerated from this inventory; whole-system verification remains false.
- Timing: 64 synthetic shapes, two executions and seven alternating measured pairs after three warmup pairs: 896 pairs. Improvements and regressions are preserved in the adjacent report and TSV files. This is CPU microbenchmark evidence, not full-school, GPU or whole-application throughput.

#### Receipt digests

The model checker and catalog auditor validate declared structure and hashes. Native tests are finite observations. No new Lean proof, universal floating-point guarantee, semantic accuracy result or full-repository test result is claimed.

## transfer-wave25

### Wave25 retained transfer scope

#### Measured retained improvement

Coloring gives about 9.01–9.04x on the n65 chain and 8.38–9.11x on the longer chains against fd540. At n65 it reduces allocations from 69 to 2 and requested bytes from 5272 to 586. Large stars incur one additional color allocation; the n2048 star is about 33–34% slower, and n64-chain timings also regress. See [all conversion timings](../../en/src/convert.rs.groupant/model.md#wave25-performance). This kernel tradeoff is retained; representative Mova input frequencies and overall throughput remain unmeasured.

Current structural and native receipts are in [wave25 validation](../algorithm-library.groupant/model.md#wave25-validation). The algorithm-library additions and rejected spatial experiment have separate evidence and are not relabeled as training or GPU gains.

## transfer-wave26

### Wave26 retained scope UTC

The new functional-graph decomposition independently models the existing converter gate. After raw preflight, add virtual root 0 with successor[0]=0 and successor[i]=head[i]. Every token must enter root's component; otherwise the first failed token is the least outside it, including tails. Five tests cover 3,413 head maps, independent walks, raw-versus-derived head policy, special rows and threshold boundaries. This is a test/model composition; the production gate retains its implementation.

#### Rejected candidate

The candidate removes one allocation for larger trees and improves 32/64-token chains about 2.3–6.0x against the current parent, but the star-recovery prediction fails. The 64-token middle-star ratio reaches 0.696x, about 44% greater latency; most large shapes regress 3–6%. No representative workload win is established, so it is rejected without retuning. Historical gains do not cancel current-parent regressions. Six serial executions contain 2,310 measured pairs; all planned results are preserved. Measurement used an exclusive build/test-free slot.

Five existing transfer catalog records remain unchanged and source-bound. Historical before objects retain their original epoch. Current native evidence is 877 algorithm checks and 30 focused Mova checks, each twice per debug/release profile. [Validation](../algorithm-library.groupant/model.md#wave26-validation) and [evidence](../algorithm-library.groupant/model.md#wave26-evidence) distinguish structural models, finite behavior and measured resources. No whole-school, training, GPU or linguistic-quality improvement was measured.

## transfers

### Algorithm transfers

The five declared adapters are recorded in [algorithm-transfers.json](algorithm-transfers.json). Current application and algorithm views bind exact source, manifest and component bytes. Original before objects are historical 34811b baseline bindings, not the wave25 comparison. [Wave25 scope](model.md#transfer-wave25) identifies fd540 as the comparison revision and links current source-local evidence. The three retained transfer evidence/benchmark/validation documents are historical. Conversion now realizes the root-reachability observation through coloring, without runtime BFS. Catalog validation is structural, not proof or semantic discovery.

World lazy indexing was rejected; the spatial catalog record and source/model bytes are restored to fd540. Conversion coloring remains current. See model.md#transfer-wave25 for candidate identity, rejection evidence and the pending retained-suite distinction.

## wave27-evidence

### Wave27: fifty existing Mova sources

| Group | Existing sources | Local fragments | Named worker checks | Compiled mutants rejected |
|---|---:|---:|---:|---:|
| English lexical | 10 | 10 | 10 | 3 |
| English structure | 9 | 17 | 9 | 3 |
| English orchestration | 8 | 5 | 14 | 7 |
| Coreference pipeline | 9 | 24 | 13 | 7 |
| Annotation orchestration | 7 | 19 | 17 | 6 |
| Symbolic arithmetic | 7 | 2 | 22 | 7 |
| Total | 50 | 77 | 85 | 33 |

These are source-bound partial models: forward relation, invariants, costs, behavior complement, ambiguous reverse reconstruction, specialized typed views, tests and domain limits. The six parent groups record actual representation, ordering and failure relations. All previously pending en, coref and annot sources now have registrations; this does not close their semantic gaps. The tree has 603 of 975 Rust sources registered, 44 groups and 424 scoped fragments. There are 372 pending sources (289 public, 83 closed). See [source identities](wave27-coverage.json) and [remaining queue](../../docs/groupants.md).

#### Executed evidence

The six source suites each passed twice debug and twice release with identical sorted outcomes: 85 distinct named tests and 340 worker test instances. The integrated tree passed all 85 names in both profiles. Its standalone en build-script harness additionally executes the hash test imported from en/src/hash.rs, giving 86 target-qualified checks per profile, 172 integration instances. The initial root accounting expected 85 and stopped after all debug checks passed; the corrected accounting reused those successful receipts and ran release. This was a harness count failure, not a native test failure.

Thirty-three production mutations compiled successfully, then failed named assertions (compile exit0, test exit101). Exact source restoration and positive reruns succeeded. Two root structural mutations (stale transitive source hash and unknown immediate child) were rejected with exit1; bytes were restored and the positive gate passed. Paired partial syntax projections for all 50 sources are identical. Whole-tree groupant structure and changed-source registration pass, with no missing model among the 34 changed Rust files. Inventory --require-all intentionally fails while 372 sources remain pending.

Build-script generation also has three isolated synthetic scenarios repeated four times. Library reexport boundaries carry compilation/visibility evidence only. Fake local executables exercise annotation queue behavior; no live teacher/API behavior is inferred. The broad existing repair test may skip without its dictionary, so an unconditional shape/lowercase boundary test supplements it.

#### Review feedback applied

- Construct cost includes total words, target arcs, map/key work, callbacks and repeated emitted sentence bytes. Store cost includes typed visitor work; short input can advertise many zero-byte values. Unit multiplication includes cloning/scanning: O(a+b+ab).
- Annotation patches accept or reject each id independently. Accepted ids must be seen and unbroken; absent ids are individually rejected.
- CorefUD write/read needs same-eid stack-compatible spans. An independent actual-code debug witness shows crossing spans [0,2] and [1,3] reconstructing as [0,3] and [1,2]. Raw writes leave existing Document Entity views stale; rebuilding documents is required. Retree clears word DEPS, so the inverse loses those original columns. The witness also confirmed input preservation. This single experiment is separate from the repeated source suites.

#### Limits and receipts

Strict Clippy remains blocked by existing lints: en/build.rs, four coref lints, six annotation lints and 113 math diagnostics in the recorded invocations. Annotation passes only when its four existing lint categories are allowed. No all-workspace lint-clean claim is made.

Important uncovered behavior includes unsupported empty-trained tagger inference, malformed models/blobs, cycles and invalid index inputs, unchecked numeric exponents/counters, nonfinite or empty variadics, allocation/I/O failures, live transport and crash durability. No executable percentage surrogate was added for categorical/effect interfaces without a defensible error metric. Existing supported approximation caps remain 1%, 2%, 5% and 10%. Finite source tests, partial syntax and structural validation are separate from Lean proofs, whole-system accuracy or speed measurements.

## wave28-evidence

### Wave28: fifty existing sources and a repaired verification boundary

| Group | Existing files | Scoped children | Native checks | Compiled mutants rejected |
|---|---:|---:|---:|---:|
| Symbolic computation | 7 | 16 | 18 | 8 |
| Quantitative understanding | 7 | 13 | 13 | 7 |
| Math orchestration | 14 | 12 | 23 | 12 |
| Formula syntax | 8 | 24 | 27 | 4 |
| Formula bridge | 8 | 10 | 15 | 7 |
| World dialogue | 6 | 6 | 9 | 4 |
| Total | 50 | 81 | 105 | 42 |

The tree now registers 653 of 975 Rust sources, with 50 groups and 505 scoped fragments. There are 322 pending sources: 239 public and 83 closed. Each new child shares its parent's source syntax projection and binds an explicit byte range. Registration and finite tests do not establish complete semantics. [Source identities](wave28-coverage.json) and [remaining work](../../docs/groupants.md) retain that distinction.

#### Verification repair

Previously, linsys::check_solution zipped equations with right-hand sides and coefficients with the proposed solution. Mismatched lengths could silently omit data and return a successful check. It now requires len(A)=len(b) and every row width=len(x) before computing exact dot products. The order is count check, width check, arithmetic. The valid empty system remains vacuously accepted, even for a nonempty x; gauss retains its separate shape preconditions.

An old-code regression compiled with exit0 and failed its assertion with exit101. Repaired checks and a separate compiled width-guard mutant constrain the change. Source-delta review inspected all 35 changed Rust files: only the six guard lines alter production behavior. Other changes append tests or extract a test-only load_path helper without changing production loading.

#### Executed checks

All 105 native names passed twice debug and twice release in isolated worker trees. Integration reran the exact selected names once in each profile with identical outcomes: 210 native test instances. One inspected parse/print doctest also passed in both profiles; workers ran it twice per profile. Two bounded CLI paths ran in both profiles: math with no arguments returns before model loading, and mathsolve calc evaluates 2+3*4 to 14 in an isolated output directory. Library reexports and unexecuted main branches remain compile/visibility evidence only.

The 42 mutation controls compiled, failed assertions, restored exact source bytes and passed positive reruns. This count excludes the separate old-code shape regression. Two root structural controls reject stale transitive hashes and unknown immediate children; restoration returns the gate to green. Fifty source syntax projections repeat identically. Final whole-tree declarations and all 35 changed-source registrations pass. Inventory --require-all intentionally fails while 322 files remain pending.

Five external Rust checks call actual source APIs across group boundaries, twice per profile with identical names and registry versions pinned to the workspace lock. Four closed rational formulas preserve independently stated exact values through LaTeX/Expr conversion. A foreign result fails the numerical gate. Division by zero can roundtrip structurally while numerical evaluation rejects it. Empty Add prints an empty Node but has no successful bridge inverse. simplify(0/x) erases the x=0 domain hole while the sampled derivative gate can accept observations away from it. These observations enrich the symbolic-arithmetic parent; Expr and calc::E remain distinct representations.

#### Review feedback and evidence limits

Nested models were revised from generic wrappers to concrete transitions, excluded results, compatible-input examples and specialized views. Cross review corrected per-operator fingerprint support, enumeration versus claim timeout accounting, overflow of accumulated perceptron scores, rate-row context effects, future-event dependencies in historical state queries, macro End restoration scope, term-specific number juxtaposition and zero-numerator literal folding. These are source-derived qualifications unless a named runtime witness is recorded.

The initial integration harness needed normalization of libtest's should-panic outcome suffix; successful native command receipts were reused. The canonical groupant checker also rejected H1 section headings in eighteen model catalogs; they were normalized to its required H2 form and the full gate rerun. Initial failures remain in external receipts. The preliminary join experiment used its own dependency resolution; it is excluded from the four final workspace-pinned runs.

Strict Clippy is still blocked by existing findings: 113 in the math library, three in bigbench, nine/fourteen in the recorded latex scopes, and 259 in world. These counts describe individual invocations, not unique repository-wide defects. No all-workspace lint-clean claim is made.

No live teacher, Lean invocation/proof, corpus, frozen assessment, GPU, model/database mutation, full-school or expensive benchmark run was used. Public malformed inputs, resource exhaustion, broad search/training behavior, transport and crash durability remain explicit gaps. No new executable percentage surrogate or speed claim is made; supported future approximation caps remain 1%, 2%, 5%, and 10% with a defined metric and evidence.

## wave29-evidence

### Wave29: fifty knowledge and world sources

| Group | Existing sources | Scoped children | Native outcomes | Compiled mutations |
|---|---:|---:|---:|---:|
| World rule learning | 7 | 11 | 15 | 6 |
| World evaluation boundaries | 7 | 14 | 17 | 4 |
| Graph language encoding | 8 | 12 | 18 | 8 |
| Ontology core selection | 9 | 8 | 26 | 8 |
| Ontology query state | 9 | 18 | 15 | 8 |
| Ontology external alignment | 10 | 10 | 21 | 10 |
| Total | 50 | 73 | 112 | 44 |

The tree now registers 703 of 975 Rust sources, with 56 groups and 578 scoped fragments. There are 272 pending sources: 189 public and 83 closed. Registration is not complete semantic verification. [Source identities](wave29-coverage.json) and the [remaining queue](../../docs/groupants.md) retain the distinction.

#### Two repaired integer boundaries

kg::Registry::heap previously computed each heap successor with unchecked u32 addition. A tiny valid registry containing the maximum global heap caused overflow. Allocation now finds the maximum and checks its successor before insertion; exhaustion returns an error without changing the registry.

onto::store::R::take previously tested p+n against buffer length using an addition that could itself overflow. A 46-byte checksum-valid snapshot declaring an array of length u64::MAX exposed the failure. With the internal invariant 0<=p<=B, the new guard n<=B-p establishes p'=p+n<=B before slicing. This repairs the demonstrated cursor failure; it does not establish exhaustive malformed-input or resource safety.

Both previous implementations compiled and failed their regression assertions. Corrected cases pass in debug and release, including an independent root probe. Audit of all 50 source identities found 39 changed files and 11 byte-identical files. The only production exceptions are these two repairs and a behavior-preserving main-to-probe(args) extraction in the retention example. Other additions are 37 cfg(test) modules and two integration-test functions; no broad formatting changes were found.

#### Executed checks

All 112 target-qualified native outcomes passed twice debug and twice release in worker trees. Integration repeated the exact names once per profile with matching outcomes, totaling 224 instances. The count includes one global-build subprocess driver that is a no-op in ordinary enumeration; excluding that driver gives 111 outcomes. The same source test compiled into two targets is counted per target. CLI helper tests establish Command construction and cleanup, not successful school execution.

Forty-four source mutations compiled, failed actual assertions and were restored exactly before positive reruns. Two additional pre-fix regressions are counted separately. Ontoquery's local count of nine controls includes eight mutants and its store regression; an earlier hsmap mutation that failed compilation is excluded. Root also rejected a stale transitive source hash and an unknown child, restored bytes and reran the positive structural gate. Fifty syntax pairs match their source hashes. Whole-tree groupants, changed-source registration and the existing transfer catalog audit pass. Inventory --require-all intentionally fails while 272 sources remain pending.

Five root tests import actual model/store/kb and kg id/cols modules, compiled without their cfg(test) modules. The tests run twice per profile with dependency versions pinned to the workspace lock. Three original observations passed before repair; the two integer regressions failed as intended. All five pass after repair. These are bounded behavior checks, not speed measurements.

#### Composition feedback

Graph snapshot roundtrip preserves the tested concept identities, dictionary and edges. It does not serialize Kb assertions, additional layers or extra edges; rebuilding Kb from Graph alone loses the corresponding answers. Structural array validation also accepts a graph whose incoming and outgoing edges disagree while all indices remain in bounds. Its successful result therefore does not certify relational consistency.

kg heap/local identifiers remain distinct through sorting, filtering, column persistence and registry reopening. Ontology Cid is an index in a particular Graph; no automatic identity-preserving adapter from kg Id is supplied. The knowledge-memory parent now records four observed views covering state loss, structural versus semantic validity, identifier namespaces and allocation failure boundaries.

Cross review refined actual domains rather than adding labels: verb-object versus prepositional government; ASCII digit rejection; ordered floating-point reductions; duplicate total labels in evaluation; NaN ranking behavior; layer-leak mode; and absent-key constraints in the holder-map preimage. Parent and child models include explicit relations, set-valued antecedents, costs, failure ordering and source ranges.

#### Evidence limits

Initial ontocore query runs omitted ONTO_PRIOR isolation. The affected test was repeated twice per profile with an asserted absent prior; integration sets an absent run-local prior for all ontology commands. Initial receipts remain available and are not presented as fully isolated evidence. Scoped kg Clippy reports 12 existing findings; no repository-wide lint-clean result is claimed.

No live teacher, corpus, held-out evaluation, model/database mutation, full-school run, GPU execution or Lean proof was performed. induce_gpu.rs has source/syntax models, with native CUDA compilation and execution explicitly unverified. Full ingestion, successful retained-learning diagnostics, broad search, crashes, concurrent writers and arbitrary malformed public states remain partial. No new executable percentage surrogate or performance gain is claimed; supported future caps remain 1%, 2%, 5% and 10% with a defined metric and evidence.

## wave30-evidence

### Wave30: neural parsing and computing-language groupants

Fifty previously unregistered Rust sources now have source-bound model trees. Six groups add 88 scoped fragments. The registry contains 753 of 975 owned Rust sources, 62 groups, 666 fragments and 1,481 total nodes; 222 sources remain pending (139 public, 83 closed). There are 8,617 typed model views, 2,790 type labels and 743 partial syntax projections. Registration records partial models and evidence, not complete proof.

| Group | Legacy files | New fragments | Selected native tests |
|---|---:|---:|---:|
| neural-parser-paths | 13 | 19 | 17 |
| matrix-expression | 7 | 9 | 7 |
| computing-execution | 7 | 14 | 12 |
| matrix-numerics | 7 | 12 | 8 |
| statistical-computing | 7 | 23 | 7 |
| computing-io | 9 | 11 | 13 |

#### Models and composition

Each source and fragment includes a forward relation, behavior complement, compatible-antecedent reconstruction, invariant, symbolic cost, descriptant and checks. Complex files have scoped children tied to exact byte ranges and a shared parent syntax projection. All 138 source/fragment manifests explicitly bind their existing invariant-and-cost document as the cost component; the six group costs have their own section. No executable percentage surrogate or global numerical error bound is claimed.

The computing pipeline carries column-major matrices from lexer/parser and indexing through tree or VM execution, numerical builtins and memory VFS. Models distinguish parse-before-execution from runtime prefix effects, ordinary from compound indexed assignment, warning/status/error channels, and lossy CSV/JSON/display projections. Layers routing is a separate tagged relation, not a mlab call edge. Its score/threshold relation explicitly assumes finite values.

Neural models separate training and inference attention paths. Eisner claims constrain intermediate arithmetic and ordering; finite inputs alone cannot guarantee no f32 overflow or exact-real optimality. Network costs retain embedding work at zero layers, projection/tag work and the attention head memory factor.

Cross-reviews corrected power association, empty-target class adoption, reshape arithmetic order, serialization of NaN, recursive formatting cost, FFT dimensions above two, shaped-empty reduction costs, and pass-through exceptions in deconv/diff. The fzero inverse describes actual stopping branches, including nonzero residuals and iteration exhaustion; it does not assume an exact root. Statistical models distinguish paired-length and option checks, date-only versus rounded datetime output, aggregate missing-value exceptions and name-deduplication collision costs.

#### Executed evidence

All 64 target-qualified selected tests passed twice in debug and twice in release in worker worktrees: 256 executions. The combined integration ran the same 64 selected outcomes once per profile, with exact-name set equality. Four individually selected ignored CUDA checks exercised GELU values/derivatives, attention gradients, softmax gradients with finite differences, and one tiny synthetic optimizer step. GPU numerical checks use their declared tolerances; matching test outcomes do not imply bit-identical floating results.

Six independent integration fixtures passed twice per profile. Each executes both the tree interpreter and VM against literal or independently calculated expectations: column-major indexing/reduction; a 2x2 linear solve and warning gate; FFT spectrum, inverse and energy; parse/runtime effect order; matrix CSV/table means in memory VFS; and filesystem warning/status/error channels. Solve, residual and inverse-FFT checks also constrain imaginary components. The prototype's mistaken delete-status assumption was corrected after source inspection and retained in external evidence; it was an oracle mistake, not a Mova defect.

The raw-log audit separates 37 rejected source-behavior mutations from 12 modified-assertion controls. Source mutations include seven test-helper/fixture behaviors, so the total is not a count of production repairs. All rejected controls compiled and reached assertions; restored positive checks are recorded. One exploratory RNG multiplier mutation survived and is excluded from rejected totals. It exposes an untested golden-stream property. Historical byte restoration for gparse and expression assertion controls relies on successful runner equality checks and current source bindings; other controls additionally retain backups, snapshots or restored hashes.

Two structural controls reject a stale source hash and an unknown child, then restore exact bytes and pass the positive check. Fresh integration syntax was generated twice for all 50 sources; every pair was byte-identical and matched the installed projection. Structural and syntax checks do not prove mathematical semantics.

#### Source isolation and limits

A syn-based audit checked all 50 sources: 36 append only test items, 14 preserve all bytes. Added items comprise 27 cfg(test) modules and 11 standalone integration test functions. No production behavior or Rust-source count changed. Full source bodies and appended suffixes parse, and no changed Rust path falls outside the selected set.

This epoch does not execute real-model/corpus experiments, live teacher calls, frozen VMM/Octave/Boost/NIST suites, full-school tests or the benchmark main. Reexports/AST representation and private harness helpers have narrower evidence than executable semantic paths. Large/adaptive numerical paths, malformed public buffers, resource overflow and several external effects remain explicit gaps. Scoped Clippy attempts were blocked by existing production lints (89 in mlab and 10 in gparse); no global lint-clean result is claimed. There is no new speed measurement or Lean proof in this coverage batch.

## wave31-evidence

### Wave31: filesystem, runtime and adapter groupants

Fifty existing sources add six composition groups, 61 scoped fragments and 734 typed views. Registration reaches 803 of 975 owned Rust sources; 172 remain pending (89 public, 83 closed). The whole tree contains 68 groups, 727 fragments, 1598 nodes, 9351 views, 3195 type labels and 793 syntax projections. Registration denotes partial source-bound models, not complete semantic coverage.

| Group | Sources | Native test names | Worker named executions | Compiled source controls |
|---|---:|---:|---:|---:|
| Virtual filesystem |8|20|80|7|
| WASI runtime |5|11|44|5|
| Rule court |10|19|76|3|
| Letter tables |5|16|64|6|
| Annotation store |12|16|64|6|
| Corpus adapters |10|30|120|6|
| Total |50|112|448|33|

Each worker test passed twice debug and twice release. Names are qualified by package and target; these count named executions rather than cargo invocations. Integration repeats all 112 names once per profile (224 additional passes). Six external composition/domain probes run twice per profile (24 passes). All fixtures are bounded CPU work.

#### Models and independent checks

All 117 new source, fragment and composition manifests explicitly declare AST, approximant, invariant, cost, descriptant, tests and typed models. Source and fragment AST references share source projections; composed groups explicitly mark a semantic AST as not materialized. Typed families begin with behavior complement and ambiguous reverse reconstruction. Fifty source syntax projections were independently regenerated twice and matched installed versions. Structural controls reject a stale source hash and unknown child; exact restoration returns the positive gate to green.

The 33 compiled controls comprise 31 production-function/table changes and 2 helper changes: the VFS benchmark formatter and WASI newest_mtime helper. Raw receipts establish compilation, named runtime rejection, exact restoration and a restored pass. There are no new assertion-only controls or retained survivors. Existing Fault toggles and invalid-input/shuffled-label checks remain separate. Final control originals match integrated source bytes.

Independent probes call actual libraries for snapshot branching/restoration; imported WASI proc_exit, trap and fuel outcomes; Rule TSV roundtrip followed by lattice/evaluation; all 484 ordered letter pairs against a triangular rank oracle; tagged normalization domains; and langseg text quantization with score accumulation. External dependency versions remain a subset of the workspace lock. One imported guest function does not establish complete ABI compliance.

The tiny quantization fixture observes absolute per-row error ≤ 0.0000501 and a sum-of-row-errors score bound. This is representation evidence, not linguistic accuracy or a percentage-budget executable surrogate. The normalization probe is an explicit adapter; no production macula-to-langseg call is invented.

#### Review corrections

Timer events contain userdata/type, not timestamps. Recovering minimum duration requires before/after virtual clock observations, timer-only execution and nonsaturating advancement. Codec roundtrip domains exclude leading exclamation-mark ambiguity and empty arguments. Letter CLI complements state concrete excluded successful reports after invalid arguments. Tokenizer cost includes quadratic unmatched-opening-tag input; scoring permits signed finite values with nonnegative switch costs.

VFS costs include cwd copying and possible last-reference tree destruction. Snapshots exclude journal and policy. Single-file quota rejection does not imply transactionality of mkdir/import/export. Late WASI output faults can preserve earlier effects. Annotation-store post-execution errors do not establish rollback.

Root preflight corrected two test expectations: completed obligations are removed before evaluation, and duty labels use act_verb rather than the full clause. Those failures are retained and excluded from positive totals; they were not production repairs.

#### Boundaries and artifacts

Production logic is unchanged: 28 sources retain exact baseline bytes plus test-only additions; 22 remain byte-identical. Complete files and suffixes were parsed, including verification of the cfg(test) parent of the WASI unit-test module. No repository Rust source was added.

DuckDB was unavailable. Store evidence covers actual helpers, temporary files and CoNLL fixtures; SQL COPY, migrations, views and transaction semantics remain conditional. No live corpus, teacher, training, GPU, full guest/frozen suite or benchmark main was run. Host races, resource exhaustion, arithmetic extremes, legal validity, historical/physical truth of quoted tables and complete CLI effects remain outside the finite domains. No universal proof, new Lean proof, measured speedup or percentage-accuracy certificate is claimed.

## wave32-evidence

### Wave32: Hebrew segmentation, roundtrip and translation

#### Inventory and model scope

The leaves contain 62 scoped children: 8, 14, 11, 12, 9 and 8 respectively. Their 672 local typed model entries comprise 84, 138, 114, 126, 108 and 102 entries. Together with composition groups and inherited parent views, the whole-tree increase is 734 typed models, recorded in the [coverage identities](wave32-coverage.json). Registration reaches 853 of 975 sources, leaving 122 pending: 39 public and 83 closed. See the [coverage queue](../../docs/groupants.md).

Every selected source has a bound partial syntax projection, forward relation, invariant, cost model, descriptant and typed family beginning with behavior complement and reverse reconstruction. Children share parent syntax artifacts and declare exact byte ranges. Partial syntax does not provide macro expansion, name resolution or a proof of semantic correspondence.

Production review distinguishes 34 files with test-only suffixes from 16 unchanged files. All original production bytes remain exact prefixes. There are no production changes and no new repository Rust sources.

#### Executed evidence and acceptance

Worker evidence accounting confirms 90 target-qualified native names, each run twice in debug and twice in release: 360 positive executions. Named outcomes were compared rather than relying on aggregate pass totals. The integrated tree passed those 90 names in both profiles, adding 180 executions. Eight independent composition/domain probes passed twice per profile, adding 32 outcomes. Exact outcome names matched across each repeated scope.

Independent accounting confirms 37 compiled production-behavior mutants rejected after successful compilation. Thirty-six fail direct assertions; one Hebrew segmentation control fails through a missing-key panic while evaluating an assertion operand. This distinction is retained rather than calling every failure an assertion mismatch. There are zero test/benchmark-helper mutants, zero compiled assertion-only controls and zero surviving exploratory mutants. Original bytes and hashes were restored and positive checks rerun. Integrated binding compares every audited backup with the final source bytes and hashes.

Publication into the repository additionally requires the native and join summaries, two intended structural-control rejections with exact restoration, 50 fresh matching syntax pairs, evidence accounting, integrated control binding, empty component-check issues and zero production changes. Final structural, inventory, transfer-catalog, whitespace and secret-guard checks remain separate gates. Inventory acceptance retains the 122-file backlog; it is not full coverage.

#### What the observations mean

For an admitted codec domain and fixed canonical state, residual-assisted reconstruction can satisfy `D(E(x), state) = x`. The state includes required layouts, histories, coordinates, dictionaries and residuals. This diagnostic does not establish the meaning or quality of residual-free decoding, nor persistent knowledge acquired from a lesson. Reverse relations retain ambiguity where normalization, aggregation or projection discards information.

Review corrected the distinction between group-preserving chapter boundaries and arbitrary concatenation: concatenation itself introduces no separator and imposes no group boundary. Cost models now retain empty-input work proportional to `O(SZ)` where the two outer domains remain traversed, lattice and per-prefix work, and report rendering costs. UTF-8 byte offsets differ from token indices and dependency heads; joins must transport the right coordinates. Similarly named TSV outputs and rates retain different schemas, population filters and denominators.

A repeated-verb negation permutation counterexample limits claims that logical comparisons preserve event identity. IBM1 normalization uses finite floating-point arithmetic, and its alignment prior is a model choice rather than evidence of translation quality. Candidate score gains likewise do not imply output-quality gains. The corpus model includes empty-sentence iteration and initial word cloning; the product inverse of parallel mapping requires a pure stateless callback, otherwise shared state and schedule remain part of the relation.

A probe-formatting command accidentally followed a path import and formatted the IBM source. The diff was saved and the source restored before import and final evidence collection. Final source bindings and the production audit govern the accepted snapshot.

#### Limits

Evidence is finite CPU execution of synthetic helpers and compositions. No GPU, live corpus, teacher, external annotator/model, full CLI, training or site workflow ran. Effectful paths remain explicitly source-derived. This batch adds no Lean proof, speed claim or percentage-accuracy guarantee; it does not establish full-system correctness.

## wave33-evidence

### Wave33: public source coverage and validator improvements

#### Result and coverage

This wave adds 39 source groupants, five composition groups and 52 nested groupants: 96 new nodes in total. Eleven nested nodes refine three already-registered modelcheck sources. Existing modelcheck catalogs retain prior models and gain scoped validation, syntax and command-dispatch models. There are 608 additional typed views.

Current inventory: 892 of 975 owned Rust files registered. All public sources in this inventory are registered; 83 closed-area files remain separately queued. Totals: 79 groups, 841 fragments, 1812 nodes, 10693 typed views, 3895 distinct type labels and 884 materialized syntax projections. Registration is a source-bound partial model, not a complete behavioral proof. All 99 new or improved nodes declare AST, forward model, invariant, cost, descriptant, tests and typed inverse/complement components.

| New composition group | Existing sources newly registered |
|---|---:|
| math-library-ingestion | 9 |
| cycle-teacher-diagnostics | 9 |
| treebank-comparison-tools | 10 |
| text-corpus-tools | 7 |
| bibliography-value-tools | 4 |

#### Production changes

The bibliography formerly admitted any URL containing the text aclanthology.org. A foreign host or path could therefore shadow an official record with the same extracted ID and enter an ACL manifest. Entry::anthology_id now requires HTTP(S) and the exact ASCII-case-insensitive authority; ID-indexed consumers and topic JSONL share this admission rule. The topic path still retains matched foreign entries in its descriptive abstracts. Old-code fixtures fail before the repair; the independent actual-library join returns a path-spoofed title before repair and the official title afterward.

ID extraction ignores query and fragment, but row still appends .pdf to original URL text. Host admission does not guarantee canonical usable PDF URLs for query/fragment-bearing inputs. Ports, userinfo and relative URL forms are unsupported by this narrow admission rule. No live download or license-authenticity claim follows from these tests.

Modelcheck now uses binary search at its two membership callsites. rust_files collects a BTreeSet and returns sorted unique paths; String ordering and str comparison agree. Under a fixed discovery snapshot, binary membership and the former existential linear scan have the same Boolean result. Worst-case path comparisons change from O(QP) to O(Q log(P+1)), with string-comparison byte costs separate. No lossy numerical approximation is introduced.

The source audit parses all 42 selected files and replays exact independently reviewed replacements against the base bytes. Twenty-five files changed, of which three contain production changes; seventeen remain byte-identical. Other additions are parsed test modules/functions, including a reviewed insertion inside the existing syntax test module. No new repository Rust source is added.

#### Measurements

The actual helper is compared with the old iterator expression on empty, singleton, prefix and Unicode fixtures, 975 discovered repository paths plus misses, and 4096 synthetic paths plus misses. Full Boolean vectors and hit totals agree. Each release execution takes five paired samples in alternating order, with three repetitions per sample and black_box inputs.

| Helper fixture | Linear median | Binary median | Observed ratio |
|---|---:|---:|---:|
| Repository paths, release run 1 | 0.823 ms | 0.296 ms | 2.78 |
| Repository paths, release run 2 | 0.821 ms | 0.300 ms | 2.74 |
| Synthetic 4096 paths, run 1 | 55.612 ms | 1.040 ms | 53.49 |
| Synthetic 4096 paths, run 2 | 56.178 ms | 1.039 ms | 54.08 |

The synthetic common-prefix workload particularly penalizes linear scanning. These are helper measurements; they exclude Git discovery, hashing, artifact reads and subtree traversal.

Root additionally ran three alternating baseline/candidate pairs of the complete groupants command on the integrated repository. Full JSON reports are equal. Median baseline: 3.130027 s; candidate: 3.144357 s; baseline/candidate ratio: 0.995. This run shows no complete-command improvement. These short shared-host observations include process startup, discovery, file reads, hashing and JSON output; compilation is excluded. They are not a stable throughput guarantee. Neither helper nor checker measurements establish a training, lesson, exam or GPU speedup. The installed executable was not replaced.

#### Native and negative evidence

- 106 distinct target-qualified Rust tests pass twice debug and twice release in worker trees: 424 named outcomes. Root repeats the same 106 names once per profile on integrated sources: 212 outcomes, with identical name sets.
- Six independently authored actual-library joins pass twice per profile: 24 outcomes. They cover bibliography identity/foreign-host shadowing, sorted Git inventory and hashing, syntax/sidecar staleness, discovery confinement and changed-source inventory.
- Eleven actual modelcheck CLI scenarios pass twice per profile: 44 process outcomes, counted separately from Rust test names. They include success and failure paths, exclusive output creation, invalid syntax, persisted invalid report, require-all failure, revision-option rejection and proof-table handling.
- Independent raw-log accounting accepts 35 compiled production mutants: 34 assertion failures and one unwrap_err runtime panic. Zero assertion-only mutations or compile failures count as behavior kills. Every accepted backup matches final integrated bytes and hashes; restored checks pass.
- The superseded cycle parser-count panic remains in external history and is excluded from the accepted 35-control catalog.
- Two structural corruptions are rejected, then restored byte-exactly with a positive gate. These are metadata controls, separate from compiled mutants.
- Forty-two fresh paired syntax projections match each other and installed sidecars. Structural identity/range checks do not establish mathematical truth.
- The optimized checker also matches a temporary actual CLI using the old linear predicate byte-for-byte. Root's complete-command comparison independently checks the integrated reports.

#### Review corrections and model boundaries

Independent cross-review corrected the Kaggle learner bound: at most twelve retained rules can require thirteen candidate-building iterations. Cascade costs now include repeated hashing of variable-length sentence IDs during sort_by_key. Bibliography models explicitly separate exact-host identity from PDF URL normalization. Sixteen mathlib nodes initially had only five declared views; a documentation supplement added and independently verified a sixth source-specific view to every affected node. Totals above count actual manifests.

Models preserve important differences: tree readers have distinct coordinate/admission domains; aggregate statistics do not reconstruct omitted empty nodes; direct Kaggle shared-key neighbors are not transitive components; failed collection can leave earlier database effects; reservation counts attempts, not successful teacher results. The lesson example's preparsed/repeated-learning diagnostic is not treated as a saved-state knowledge test. Partial syntax leaves macros opaque and does not load external modules.

All execution was bounded CPU helper/process work with synthetic inputs or temporary files, apart from read-only repository metadata needed for validation. No live network/DNS, corpus download, teacher call, model loading, training, lesson/exam run or site publication was performed. GCS, effectful Duck paths, ignored live teacher and some CLI/example paths retain explicit source/compile-only gaps. No new Lean proof, universal equivalence proof, or certified percentage surrogate is claimed.

#### Reproduction and next work

Public source identities and hashes are in [wave33 coverage](wave33-coverage.json); the current queue is in [groupant coverage](../../docs/groupants.md). Adjacent source sidecars record scoped models and exact test names. External wave33 receipts retain commands, numeric exits, raw output, source backups, timing samples and independent review corrections.

Run inventory, groupants and changed against the current source/registry epoch. require-all still fails intentionally for the 83 unregistered closed-area sources. Refine partial semantics and executable approximants based on actual callers; avoid treating an empty public queue as proof that all behavior has been tested.

## wave34-evidence

### Wave34: fifty nested groupants and faster artifact validation

#### Scope

Exactly fifty new scoped groupants deepen ten existing source trees. Each new node declares its source scope, partial AST reference, forward model, invariant, cost, descriptant, tests and at least six typed views, beginning with behavior complement and reverse reconstruction. Existing node IDs and useful epoch evidence are preserved. There are 300 additional typed views.

| Existing source pair | New scoped nodes | Main modeled boundaries |
|---|---:|---|
| world school / schooltools | 10 | retained codecs, marginal election, event play, answer priority, fold residues, field/marker parsing, normalization and edits |
| world knowledge / retention | 10 | citation/fact/field folds, tail recognition, cache publication, envelope/provenance gates, retained digests and numbering |
| world read / ans | 10 | segment admission, service filtering, phrase/clause carry, ranking/eligibility, numeric output, why fallback and LCS |
| math calc / qread | 10 | lexical/power rules, exact and floating zero, ranged binding, fingerprints, sparse substitution, state folds, solver passes and provenance |
| modelcheck library / syntax | 10 | child admission, syntax envelopes, component dispatch, digest/section checks, source freshness, coordinates, parents and decode memo |

Current totals: 892 registered sources of 975, 79 composition groups, 891 nested fragments, 1862 nodes, 10993 typed views, 4004 distinct type labels and 885 materialized syntax projections. The public registration queue remains empty; 83 closed-area sources remain separately pending. This wave adds no repository Rust files or top-level registration count. Registration is not complete semantic proof.

#### Exact production optimization

Modelcheck now retains one recently decoded syntax JSON artifact within a source root and its descendant traversal. Every reference still confines the path, rereads the file, compares exact bytes and validates the returned value against the current source, header, coordinates and node relationships. Identical serialized bytes can share an immutable Rc<Value>; changed, malformed, missing or directory inputs retain their ordinary errors. There is no cross-source or cross-invocation cache.

The 8 MiB threshold applies to retained serialized bytes. It neither bounds parsed allocation nor total/peak process memory. Larger valid inputs remain admitted but are not retained. A miss can temporarily hold old and new allocations. Errors do not become cached successes. The opt-out is MOVA_MODELCHECK_REFERENCE_JSON=1, which decodes every reference.

The mathematical premise is exact equality of decoder input bytes, not a hash approximation. Parsing/allocating identical JSON can be eliminated on hits; byte rereads, byte comparisons and all semantic/structural validation work remain. A source change still invalidates an unchanged cached projection when validation rereads the source. Sequential filesystem races remain possible as before; this does not establish an atomic repository snapshot.

Production changed only in modelcheck lib.rs and syntax.rs. The full ten-file audit parses Rust and compares production tokens after removing top-level test-only modules/functions; both production deltas are independently reviewed and bound to exact baseline/current source hashes. Nine files changed in total: seven append test-only modules, two contain the reviewed optimization; retention.rs itself remains unchanged. No new Rust source is introduced.

#### Full-command measurement

Before importing changes, root built an immutable release executable from the exact base with the same offline locked profile used for the candidate. Three full groupants-command pairs ran in alternating order on the final integrated source/registry contents, with fresh output paths. All six complete reports are equal.

| Complete groupants command | Median |
|---|---:|
| Exact base executable | 3.730097 s |
| Candidate with decode memo | 1.474814 s |

Observed base/candidate ratio: 2.529. Times include process startup, Git discovery, artifact reads, source hashing, validation and JSON output; compilation is excluded. These are three-pair shared-host CPU observations, not a universal throughput guarantee. They measure groupant checking, not lesson/exam/training speed. The installed tool was not replaced.

Worker measurements separately compared reference mode and memo mode on synthetic decode/validation repetitions and complete worker reports. Those worker reports retained expected stale ancestor diagnostics; root measurements above use the fully integrated valid registry. No old-code failure is claimed for this exact optimization.

#### Evidence

- Fifty-five distinct target-qualified native Rust tests pass twice debug and twice release in worker trees: 220 positive named outcomes. Root repeats the same 55 once per profile on integrated sources:110 outcomes with matching names.
- Six independently authored actual-library joins pass twice per profile: 24 outcomes. They join school integer domains with exact calc; calc values with ordered qread abstention; school edits/splitting with assessment identity; reader question types with answer frames; discovery with syntax/hash/exclusive writes; and saved Retained state with a fresh subprocess.
- The fresh-process retention probe roundtrips an empty constructed state and exercises early exam admission errors without mutation. It does not execute a successful exam, load an annotation model or demonstrate learned answer quality.
- Independent raw-log accounting accepts 14 compiled production controls: 11 direct assertion failures, one expected-error unwrap_err panic, one Result::unwrap panic and one production bounds panic. Assertion-only edits and compile failures count as zero behavior kills. All 14 backups match final integrated source bytes and hashes; exact restored tests pass.
- Two separate structural metadata corruptions reject, restore byte-exactly and return to a positive gate. They are not compiled production mutants.
- Ten fresh paired syntax projections match each other and the declared sidecars. Final source hashes also match the independent join receipts.
- Structural, changed-source, transfer and diff gates pass. require-all remains intentionally nonzero for 83 closed-area missing registrations. These gates do not infer mathematical truth.

#### Corrections from independent review

Eight precise documentation edits resolve four model-boundary issues and one evidence-classification issue:

1. Preparse cache preservation/reconstruction requires a callback that does not mutate the same RefCell cache or leave a borrow active. FnOnce alone does not enforce that premise. Callback effects on other state are not rolled back.
2. Star removal precedes whitespace trimming; a newly exposed star may remain at the beginning of the canonical answer.
3. The provenance gate rejects abs(e.c)>1e-9, not every nonzero constant. This differs from the separate 1e-12 sparse-support pruning threshold.
4. Exact zero divided by infinity bypasses the exact shortcut but can return finite signed floating zero after ordinary evaluation. Shortcut exclusion is not universal rejection of nonfinite operands.
5. Removing the retention kind sentinel rejects through an unexpected-Err unwrap panic; it is not a direct assertion failure.

Source-specific models retain ambiguity after normalization, aggregation and lossy projection. Fixed-modulus fingerprints explicitly admit collisions and provide no probabilistic guarantee. The four-pass single-unknown solver is not general elimination and does not check every contradiction. No percentage surrogate or new Lean proof is claimed.

Integration also normalized legacy registry mirror list fields without changing their prose and replaced the root's obsolete claim that most Rust sources were unregistered. Earlier dated evidence remains intact.

#### Reproduction and limits

[Scoped identities](wave34-coverage.json) bind all 50 new nodes and the ten current/baseline source hashes. [Coverage queue](../../docs/groupants.md) records remaining registration work. Source-local sidecars retain exact test names and current/historical evidence distinctions; external wave34 receipts retain commands, numeric exits, logs, mutation backups, timing samples and independent reviews.

All execution used bounded CPU synthetic inputs, temporary files/processes or read-only repository metadata. No live teacher/API, corpus download, model loading, full lesson/exam/training run, frozen evaluation or site publication was performed. Preserve those unexecuted paths as gaps. Refine the models and test actual caller interactions before using them to justify training or answer-quality claims.

## wave35-evidence

### Wave35: complete source registration

The model checker now supports confined group directories and deterministic base/overlay composition, with duplicate/tree validation and exclusive output creation. Private paths remain in local artifacts. The public registry remains independently valid.

Selected integrated actual-source checks: 172 names in each debug/release profile, 344 passing executions. 85 fresh syntax pairs matched installed artifacts. Combined require-all, changed-source and both public/combined structural checks passed. Two structural metadata controls showed red, restored exactly and returned green. Worker test/control accounting and independent production reviews are retained in local evidence.

Reviewed production changes span 10 files: registry infrastructure and bounded input-edge repairs. Source hashes and finite helper evidence are distinguished from extracted-snapshot comparisons and unexecuted whole workflows. No whole-system proof, speedup, teacher/training run or frozen-assessment result is claimed. The older production locks of some local tools remain unchanged; external locked adapters compile the actual merged sources for these checks.

Independent raw-log audit confirms 191 worker test identities and 764 final passing outcomes across two debug/two release runs. Seventeen compiled production controls fail at runtime (sixteen assertions, one unwrap panic); original snapshots are restored, with one later test-only formatting change explicitly accounted for. Eight distinct old-code regression witnesses support the eight bounded repairs. A separate actual linked-crate oracle checks all 4165 directed overlays on 1..4 nodes in its declared domain, accepting the predicted 13 trees; all four profile/repeat passes agree. These iterations are not counted as 4165 independent test functions.

A bounded CPU measurement of the combined groupant check took 1.792s; complete source inventory took 0.071s. Registry composition took 0.021s and 0.016s in two equal-output runs. These are whole-command wall times for this snapshot, with compilation excluded; no baseline speed comparison is inferred.

## History

### 2026-10-29 · document

Subject/revision: this registry group at manifest.json source_bindings, inspected 2026-10-29.

### 2026-10-29 · tests

Subject: manifest-bound registered sources at the 2026-10-29 modeling epoch. Catalog oracles are proposed except the structural checks recorded here; no per-entry semantic execution is implied.

Executed 2026-10-29 with the installed mova-modelcheck on baseline ab4ba4321 plus these two groupants:

### 2026-10-30 · document

#### School state and pragmatic observation, 2026-10-30

Eleven existing sources and a ten-join harness extend the source-bound tree. A pragmatic-observation parent binds agreement, invariants, expression meaning, role/context and irony code. School joins retain arithmetic domains, generated answer identities, codec activation and grouped folds. Answer observations preserve state provenance, temporal anchors and separate control denominators. Pure table preservation is evidence for retained representation, not fresh-process learning or durable storage. Teacher/cache/publication and nonempty mind/parser paths remain source-inspected gaps. Prospective relations and later counterexamples remain in adjacent descriptants.

### 2026-10-30 · tests

#### Retention/cache integration, 2026-10-30

The current registered tree has 102 source sidecars, 9 inter-file groups, 75 nested fragments and 1215 typed models. Source syntax remains a partial projection (91 files), not a complete AST proof. Integrated debug/release checks pass 191 tests plus one doctest: 167 algorithm tests, 16 retention tests, 6 cache tests and 2 fresh-process parent tests. The child helper is marked ignored for direct harness discovery and explicitly executed by its parents. Ten compiled production mutants across the four algorithms, cache and retention reject; the fresh-process probe has a separate false-expectation control.

Repeated structural reports, ancestor source binding checks, changed-source registration, stale-source/wrong-child controls and secret guards accompany the integration. No live parser, teacher, training, full school course, fresh Lean proof or throughput comparison is included. Full-system verification remains false; 504 legacy owned Rust files remain without sidecars. Evidence: $MOVA_DATA/runs/octopus-groupants-next-2026-10-30/.

### 2026-10-30 · transfer-benchmark

Heading `### Final algorithm transfer CPU confirmation — 2026-10-30` stays in transfer-benchmark as `### Final algorithm transfer CPU confirmation`.

Three warmup pairs precede each seven-pair sequence. Seven pairs give a 4/3 order split, not perfect balance; coref starts production-first, the other suites baseline-first. Timing used integrated release code; the external coref runner linked the restored integrated coref library. Raw receipts, hashes and exact methodology are under `$MOVA_DATA/runs/octopus-transfer-integration-2026-10-30/` (`final-{coref,annot,en,world}-{1,2}.log`, `final-input-sha256.txt`, `final-binary-sha256.txt`, both postcheck logs, `final-methodology.md`).

### 2026-10-30 · transfer-evidence

Heading `### Algorithm-to-Mova transfer evidence — 2026-10-30` stays in transfer-evidence as `### Algorithm-to-Mova transfer evidence`.

Raw commands, logs, source/binary digests, pilot records, native-summary.json, adapter-controls-summary.json and catalog control receipts are retained under $MOVA_DATA/runs/octopus-transfer-integration-2026-10-30; source-local worker receipts are in the sibling octopus-transfer-{annotation,coref,world}-2026-10-30 directories. All runs are synthetic CPU checks: no live teacher, corpus/database mutation, GPU run, full-school pass or held-out evaluation.

### 2026-10-30 · transfer-validation

Heading `### Algorithm transfer validation — 2026-10-30` stays in transfer-validation as `### Algorithm transfer validation`.

Base: `34811b38001b47df99fcba3e6a98967adccb4b72`. Current sources and model components are bound in [algorithm-transfers.json](algorithm-transfers.json), adjacent manifests and partial syntax. Evidence below applies to those bytes, not an arbitrary future edit.

These external JSON receipts are retained under `$MOVA_DATA/runs/octopus-transfer-integration-2026-10-30/`. They distinguish executed outcomes from prospective model text.

| Receipt | SHA-256 |
|---|---|
| native-summary.json | be18071fbdad1007019fb154fb33febd0222afaead2004bacde0e4aa6e2e16fa |
| adapter-controls-summary.json | 723b806f5abf0593cc3e673d93072ae1e4a5ef4a446e0be63484e6c3912dc896 |
| final-groupants-1.json | 5c4ca8be495db9f09205acbf9e19aab13bd17d3c2ab68596caf2a69a3d88701a |
| final-changed.json | 3dcabfb233b9f73767c98e9b66889c430f24db54f511d54709cde98c34b5c5e6 |
| final-inventory.json | c212b0a09f6106da814d158894eb77ecbe368208e5147026a49a0fdd88724bcc |
| catalog-final-audit-1.json | 12afbae3cab718a4427d73f5db1091c4165031b7e7abab0999a5817be0a85c1f |
| catalog-final-controls/controls.json | 3dd34e1947465b56b98f5a2d1f1657c53a030ca9fbd72e89ba38d1f2bc3f0e5c |

### 2026-10-30 · transfer-wave25

Heading `### Wave25 retained transfer scope — 2026-10-30` stays in transfer-wave25 as `### Wave25 retained transfer scope`.

Retain conversion's ordered coloring after the <=64 walk branch. Reject world four-frontier lazy indexing on measured regressions; world source, inference test source, sidecars, spatial parent models and spatial catalog record are restored to fd540f37badaad97b53d56ba81684bbbc2ca4b9b. Retained spatial dispatch scans E<=64, performs an outgoing-edge preflight for larger sets, and constructs ordered rows for present starts; boolean direct membership shortcuts and existing product-state aggregation remain. Annotation and coreference mechanisms are unchanged.

Rejected candidate commit 09a7e88594352fc02e0c24e8f57fe62bba8c2af0; candidate world/src/spartqa.rs SHA-256 0eaf4ae70113af3ab4ee10f2edf726acadb70821c2d31e7921e93310176cfa9c. [Preserved rejection report](../spatial-inference.groupant/model.md#wave25-rejected-performance) records boundary65 product speed 0.466–0.473x parent, about 2.11–2.15 times latency. This outweighs selected small-query gains for adoption; no retuning followed measurement. Candidate correctness checks and allocation observations are not discarded, but do not imply retained performance.

Native scope: the 34-test candidate receipts remain candidate evidence. The final retained configuration passed 33 focused checks twice in debug and twice in release: conversion 19, restored spatial inference 12 and cross-gate observations 2. Unchanged conversion/annotation results were reused; restored-world checks were rerun. Normalized outcomes match within each suite. Root receipts belong under $MOVA_DATA/runs/octopus-wave25-integration-2026-10-30; candidate receipts remain under $MOVA_DATA/runs/octopus-wave25-mova-2026-10-30.

Wave25 comparison is fd540f37badaad97b53d56ba81684bbbc2ca4b9b, which already contains registered models and prior transfer optimizations. Catalog before objects preserve the original historical 34811b38001b47df99fcba3e6a98967adccb4b72 baseline; absent_unregistered applies only there. Historical transfer-evidence/benchmark/validation documents retain their original source-bound results. Restored spatial evidence applies to its exact restored bytes; it is not evidence for the rejected candidate. Conversion's current evidence is in [its source-local evidence](../../en/src/convert.rs.groupant/model.md#evidence) and [adapter](../../en/src/convert.rs.groupant/model.md#transfer). Catalog auditing is structural only.

### 2026-10-30 · transfer-wave26

The epoch label follows task start on 2026-10-30. Base: 5c1a2b9823231a2854bc52cfc04f48983c32cb22. Adopt two CoNLL-U correctness repairs: reset permissive multiword-range state at sentence boundaries; publish the derived sentence and word map together only after successful refresh. Failed refresh retains caller raw edits. Temporary map storage is the explicit correctness tradeoff; no parser speedup is claimed.

### 2026-10-31 · document

Subject/revision: registry node mova and its public descendant Rust sources at the current manifest digests, refreshed 2026-10-31. This remains a partial semantic model. The root owns self-study, knowledge-memory, model-infrastructure, inference-core and algorithm-library. Leaf syntax projections are partial; a composed semantic AST, executable root reverse engine and autonomous combiner remain absent.

### 2026-10-31 · transfer-wave26

Heading `### Wave26 retained scope — 2026-10-31 UTC` stays in transfer-wave26 as `### Wave26 retained scope UTC`.

The in-place certification experiment is preserved on commit 4e3fa202be19a137801ad276a301eec298bff222; implementation commit 2ada8b8be929438c1c4083e6684f534d75a4e16c. Candidate source SHA256: 2eb27f331cd7bee9a1b7e5426915ac0a22d89ba473ca0966a5efdf9e6e2f7be5. Candidate checks SHA256: d24d2847d1644bf9dc915171c057bf57f8fcb181c5fb1c3396228010504f550b. Only the [prospective model](../syntax-validation.groupant/model.md#wave26-rejected-prospective) and [complete timings](../syntax-validation.groupant/model.md#wave26-rejected-performance) are copied into this integration; production conversion source and sidecars retain base bytes.

### 2026-10-31 · wave27-evidence

2026-10-31. This batch removes 50 existing files from the uncovered-source queue. It adds no Rust files and leaves the algorithm library at 200 implementations. Source additions are inline tests; normalized baseline comparison of all 34 changed Rust files found no production behavior changes.

Retained local receipts are under $MOVA_DATA/runs/octopus-wave27-integration-2026-10-31 and the corresponding lexical, structure, control, coref, annotation and math run directories. Each source sidecar gives scoped checks and limitations. Integration receipts include native-summary.json, stats.json, selection.json, structural-controls-summary.json and production-delta-review.md; the annotation run holds coref-review-witness receipts. No corpus, GPU, full-school, held-out or live model run occurred.

### 2026-10-31 · wave28-evidence

2026-10-31. Fifty previously unregistered files gain source-bound partial models. This completes source registration for math and latex and adds six world dialogue files. There are no new Rust source files or algorithm implementations; the library remains at 200 algorithms.

Retained receipts: $MOVA_DATA/runs/octopus-wave28-integration-2026-10-31 and the six corresponding mathcore, mathread, mathcontrol, latexsyntax, latexbridge and dialogue directories. Root records include native-summary.json, join-summary.json, supplement-summary.json, structural-controls-summary.json, production-delta-review.md, source identities, stats and final gates. Source sidecars contain the per-file scope and limits.

### 2026-10-31 · wave29-evidence

2026-10-31. Fifty previously unregistered existing Rust files gain source-bound partial models. This completes source registration for world, onto, kg and global. Six groupants add 73 scoped children; each child shares its parent syntax projection. No new Rust source or algorithm implementation was added; the algorithm library remains at 200.

Receipts: $MOVA_DATA/runs/octopus-wave29-integration-2026-10-31 and the six corresponding worker directories. Root evidence includes native-summary.json, join-summary.json, evidence-accounting.json, production-delta-review.json, structural-controls-summary.json, source identities and final gates. Per-source model.md#tests and descriptants state the executed scope and remaining gaps.

### 2026-10-31 · wave30-evidence

#### Reproduction and identities

The [coverage identities](wave30-coverage.json) bind baseline and current hashes for all 50 sources. The [coverage queue](../../docs/groupants.md) records the remaining work. External receipts and Rust orchestration are retained in `$MOVA_DATA/runs/octopus-wave30-integration-2026-10-31`, with worker directories using their group suffixes. They include exact native commands/environments, source-test maps, four-round worker evidence, mutation/restoration logs, prospective relations, cross-reviews, fresh syntax pairs, production-delta audit, inventory and final gates. Integration native receipts must be fresh; successful logs are never silently reused.

### 2026-10-31 · wave31-evidence

[Source identities](wave31-coverage.json) bind all 50 originals. The [coverage queue](../../docs/groupants.md) tracks remaining work. Private receipts are under $MOVA_DATA/runs/octopus-wave31-integration-2026-10-31 and the six worker run directories: native/join summaries, independent accounting, source-prefix audit, syntax pairs, structural controls, peer reviews and command/status logs. Final accounting supersedes its retained prefinal snapshot and binds refreshed corpus receipts. Source changes require refreshed identities and evidence.

### 2026-10-31 · wave32-evidence

2026-10-31. This batch models 50 existing Rust sources in six groups: [Hebrew segmentation](../hebrew-segmentation-lattice.groupant/manifest.json), [roundtrip layers](../roundtrip-layer-residuals.groupant/manifest.json), [document codecs](../roundtrip-document-codecs.groupant/manifest.json), [mixed routing](../roundtrip-mixed-routing.groupant/manifest.json), [translation codecs](../translation-meta-codecs.groupant/manifest.json) and [translation evaluation](../translation-evaluation-pipeline.groupant/manifest.json).

### 2026-10-31 · wave33-evidence

Date: 2026-10-31. Base: 1d16752b51b7a4cecc878d0a4f1b1693c45269f2.

### 2026-10-31 · wave34-evidence

Date: 2026-10-31. Base: ab2909f7fc408b75fc5e2e962e13761f944f3088.

### 2026-10-31 · wave35-evidence

2026-10-31. Added 83 previously unregistered source groupants, 8 scoped fragments and 15 composition groups. The local combined registry now accounts for 975/975 Rust sources; 892 public and 83 local-only. There are 1968 groupant nodes, 11631 typed views, 4268 type labels and 968 partial syntax artifacts. Structural declarations remain partial semantic models.

### 2026-11-01 · document

#### Public worker and learning boundaries — 2026-11-01

The public crab-orchestration branch keeps concurrent annotation, durable revisions and review/export artifacts distinct from downstream parameter activation. Existing file-DB snapshots remain historical; continuous ingestion uses its version store. Child admission and model gates compose only through matching source/revision/ID/token/split/lease identities. No root transaction spans SQLite commits and external weights, and no annotation count establishes truth. The finite correspondence catalogue does not yet formalize this entire process composition.

### Undated · transfer-benchmark

The retained text below describes the transfer-integration revision contained in fd540f37badaad97b53d56ba81684bbbc2ca4b9b. Its original baseline 34811b38001b47df99fcba3e6a98967adccb4b72 and all measured counts, controls, hashes and timings remain historical. They do not validate later source bytes. Wave25 compares against fd540f37badaad97b53d56ba81684bbbc2ca4b9b; see [current wave25 scope](model.md#transfer-wave25). No old receipt is relabeled as coloring or lazy-index evidence.

### Undated · transfer-evidence

The retained text below describes the transfer-integration revision contained in fd540f37badaad97b53d56ba81684bbbc2ca4b9b. Its original baseline 34811b38001b47df99fcba3e6a98967adccb4b72 and all measured counts, controls, hashes and timings remain historical. They do not validate later source bytes. Wave25 compares against fd540f37badaad97b53d56ba81684bbbc2ca4b9b; see [current wave25 scope](model.md#transfer-wave25). No old receipt is relabeled as coloring or lazy-index evidence.

### Undated · transfer-validation

The retained text below describes the transfer-integration revision contained in fd540f37badaad97b53d56ba81684bbbc2ca4b9b. Its original baseline 34811b38001b47df99fcba3e6a98967adccb4b72 and all measured counts, controls, hashes and timings remain historical. They do not validate later source bytes. Wave25 compares against fd540f37badaad97b53d56ba81684bbbc2ca4b9b; see [current wave25 scope](model.md#transfer-wave25). No old receipt is relabeled as coloring or lazy-index evidence.
