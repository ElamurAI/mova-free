# Missing-unit batch publication

Subject: `world/src/knowledge.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## callback-noninterference-premise

All cache initial/final, preservation, error non-effect and warm-cache relations below assume the supplied callback does not mutate this student's trees cache and does not leave a RefCell borrow active across publication. The FnOnce type does not enforce this: a closure may capture the same student and mutate its RefCell cache before returning either Ok or Err. Such reentrant callbacks are outside these relations. Callback effects on other state remain outside the observation; the helper does not roll them back. The current synthetic callbacks are nonmutating with respect to trees and establish only that domain.

## forward

Expand units for each input text/question flag, collect missing strings in BTreeSet, then call the provided parser once on sorted unique missing units. Empty todo returns without callback. On callback error or unequal result length return error before mutating trees; otherwise insert zipped outputs into the cache.

## invariant

Existing cached units remain; every requested missing unit receives exactly one returned parse vector only after full cardinality acceptance. This is cache publication atomicity relative to callback result, not callback-side-effect rollback.

## cost

With U distinct requested units, lookup/set work O(total unit bytes plus occurrence count*log(cache+U)), callback cost symbolic, and O(missing strings + returned parses) temporary memory. No parser speed claim.

## behavior-complement

Type: behavior_complement. Universe is (initial trees,texts,callback-result,return,final trees) transitions under the callback-noninterference-premise above; effects on other state are outside observation. Exclude changed cache on Err callback or result-count mismatch, parser invocation for empty todo, and successful publication omitting a missing key.

## reverse-reconstruction

Type: reverse_reconstruction. Given final cache and observed todo/returned parses, initial caches retain all final entries outside todo and lack todo keys. With todo hidden, many subsets of final keys and repeated input-unit histories are compatible; warm cache and freshly parsed identical entries cannot be distinguished from final keys alone.

## sorted-demand

Type: canonical_batch. BTreeSet removes duplicates and orders strings lexicographically independent of input occurrence order; callback sees exactly missing keys, not all source texts. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## publication-barrier

Type: failure_effect_relation. Cardinality validation follows callback completion but precedes mutable tree borrow and inserts. An error may have callback effects, yet no cache rows are committed by this helper. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## warm-cache

Type: idempotent_observation. Repeating requests after a complete successful fill makes todo empty and avoids callback; this observes cache membership, not full parser determinism. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## positional-contract

Type: alignment_assumption. Equal lengths only establishes cardinality. Correct text-to-parse association assumes callback results preserve request order; the helper does not inspect parse contents. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

Course and retained wrappers pass expanded texts here before ingestion/evaluation. Callback success and cardinality gate must both hold before cache publication; downstream reading is not reached on Result error. The test callback is synthetic and never invokes the parser.

## checks

`knowledge::wave34_tests::preparse_publishes_only_complete_sorted_missing_batches` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.
