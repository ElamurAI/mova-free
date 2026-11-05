# coref-doc

## descriptant

### coref-doc

Build document tree views, quote state and speaker metadata; transplant aligned trees and adapt annotate::Word values.

State: Raw UdDoc plus independent tokens, kids, depth, subtree intervals, quote ids, speakers and raw Entity views.

from_words cyclic-input downstream termination is not established; no fault injection, u32 exhaustion or nonprojective exhaustive checks.

## approximant

### Forward model

For each nonempty sentence, validate all heads<=n then walk each parent path to reject cycles after depth>n. kids[p] records source order. Descending-depth propagation computes sub[p]=hull(sub[p],sub[child]). Split at any newdoc prefix and mark quotes per document, resetting open (not qid) at paragraph boundaries. retree clones gold, validates nonempty-sentence counts then word counts/forms, replaces six syntax columns, clears DEPS and refreshes. from_words clamps head>n to0 and bounds parent walks without rejecting cycles.

Domain: acyclic, in-range document views from checked parsing unless explicitly discussing permissive adapters; stable environment during each call; finite addressable allocations. Public vectors can be malformed and panic.

No runtime substitute or numerical error budget is established.

## invariant

### Invariant and cost

For acyclic in-range input, depth[root]=0 and each edge increases depth by1; sub is an interval hull, not necessarily a contiguous descendant set. Retree preserves gold raw comments/MISC/forms and returns no partial clone on errors. documents ignores empty sentences; from_words retains them.

For sentence lengths n: O(sum n² + sum n log n + raw bytes), O(N+raw bytes) storage. retree clones all gold bytes and checks aligned words; from_words also traverses depth quadratically. u32 depth/qid and usize lengths have no explicit overflow policy.

Integer counters/lengths use Rust native arithmetic with no separate overflow certificate. Allocation exhaustion, panics and I/O failures are not experimentally characterized.

## composition

### Source-fragment composition

Fragments project exact byte ranges of this source; shared types/imports outside ranges remain prerequisites. Models compose through actual caller/callee state, not an error-budget sum.

- tree: bytes [3456,5792); Build checked forest views and metadata with error precedence heads -> cycles -> Entity on nonwords.
- quotes: bytes [5792,7610); Paragraph/document segmentation controls quote automaton.
- adapters: bytes [7610,10966); retree validates aligned raw forms; from_words is a permissive adapter with a strictly weaker validity contract.

Parent relation: For each nonempty sentence, validate all heads<=n then walk each parent path to reject cycles after depth>n. kids[p] records source order. Descending-depth propagation computes sub[p]=hull(sub[p],sub[child]). Split at any newdoc prefix and mark quotes per document, resetting open (not qid) at paragraph boundaries. retree clones gold, validates nonempty-sentence counts then word counts/forms, replaces six syntax columns, clears DEPS and refreshes. from_words clamps head>n to0 and bounds parent walks without rejecting cycles.

Integration: checked document tree -> mention spans/features -> ordered resolver partition -> eval visibility -> CorefUD rows or existing score module. Source-specific exceptions and projection losses remain in model.md#models.

## models

# Typed models

## behavior_complement

documents success excludes out-of-range heads and directed cycles in nonempty parsed sentences; does not require exactly one root or projectivity. Entity on nonword rows is rejected after tree validation. from_words success excludes no cycles: do not transport the stronger documents guarantee.

## reverse_reconstruction

For a built view D, compatible raw inputs preserve its token projection, retained metadata and Entity values while allowing discarded comments/DEPS and skipped empty sentences. retree output admits compatible gold/tree pairs agreeing on aligned forms, with the six replaced syntax columns from trees and remaining raw columns from gold except word DEPS, which is forced to _. Original overwritten syntax and original word DEPS are unrecoverable.

## tree_validation

All head bounds precede cycle checks; cycle checks precede nonword Entity rejection. Empty parsed sentences bypass build validation.

## quote_automaton

State(open,qid): opening while closed increments qid; closing while open labels then closes; paragraph resets open only. Quotes can span sentences within one document.

## retree_alignment

Align only nonempty sentences, then exact case-sensitive forms by word offset, preserve nonword gold rows, clear word DEPS and refresh after each complete sentence.

## adapter_domain

from_words sets every space_after=true, no gold/speaker and newpar=false; clamps bad heads but permits cycles. downstream DFS mention detection needs acyclicity despite bounded depth walk.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- tests::wave27_document_failure_order_quotes_and_retree
- tests::resolver_ignores_gold_entities
- wave27_tests::synthetic_load_preserves_gold_and_rejects_mismatched_trees

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: from_words cyclic-input downstream termination is not established; no fault injection, u32 exhaustion or nonprojective exhaustive checks.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compiled actual-source control: tests::wave27_document_failure_order_quotes_and_retree. Compile exit0, assertion-run exit101, exact bytes restored, positive rerun exit0. Retained logs: control-doc-compile.log, control-doc-red.log, control-doc-restored.log.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

Strict scoped Clippy: exit101 on four pre-existing production lints (corefud unnecessary_sort_by, doc if_same_then_else and clone_on_copy, mention collapsible_if). Those source files are byte-identical to the base. No unrelated lint refactor was made.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective analysis preceded test additions. Production behavior is unchanged. A registry entry records partial source-bound modeling, not verification.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.

Review witness, 2026-10-31: an external Rust binary linked the actual coref/doc public APIs against bounded four-word synthetic input. It confirmed crossing same-eid spans [0,2],[1,3] reconstruct as [0,3],[1,2], existing Document Entity caches remain stale after raw write, and retree forces output word DEPS to _ while preserving the original input. Compile and execution exited0. This single debug witness is additional to the paired suite, not a new paired/release test. Source and numeric receipts: annotation wave27 external run, coref-review-witness.rs and coref-review-witness*.exit/log.
