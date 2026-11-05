# parse-cache-batch

## descriptant

# parse-cache-batch

## Forward

Compute missing by pre-call membership with refresh override; increment counters; invoke parser once iff missing nonempty; reject partial results; insert in occurrence order and return final-map clones.

## Invariant

On ordinary parser errors, map remains unchanged and counters count attempts. Output count on success equals input occurrences, not number of unique texts.

## Cost

n membership checks, k insertions and n output lookups plus parser and Word clone costs; duplicate missing keys can increase parser cost without increasing final map size.

## Composition

The FnOnce parser and input text sequence are external dependencies. T is shared with envelope serialization; counters feed finish publication even after a failed parse. The model does not infer full rollback from a failed child call.

## Epoch memory

## catalog

# parse-cache-batch typed models

## nonbehavior

Type: behavior_complement. Status: partial_source_derived_model.

Domain and observation: U is parse_with call tuples; fix input map, flags and supplied parser outcome.

Operator and approximation: Candidate inner C subset U minus R excludes a parser invocation for an empty missing list and cache insertion before successful batch-length validation; it also excludes unique-key deduplication when a missing text repeats.

Assumptions: No resource failure or integer overflow.

Evidence and falsifiable oracle: Finite oracle records parser call count, exact requested occurrence list and final map; duplicate-count false hypothesis rejected. See model.md#tests for executed bounds versus prospective work.

## antecedents

Type: reverse_reconstruction. Status: partial_source_derived_model.

Domain and observation: Observe final map and ordered outputs for fixed X and refresh flag.

Operator and approximation: Compatible T,P satisfy pre-call missing membership, last insertion for every missing key, preservation outside missing keys and output lookup in final T'. All earlier duplicate P entries and overwritten old values in refresh mode remain free. Counter differences fix missing occurrence count, not unique missing keys.

Assumptions: All successful parser results have exact batch size; output observation alone does not expose untouched keys.

Evidence and falsifiable oracle: Probe enumerates 16 map/parser antecedents for duplicate X and compares output fibers exhaustively. See model.md#tests for executed bounds versus prospective work.

## failed-ledger

Type: failure_state. Status: partial_source_derived_model.

Domain and observation: Fixed T,h,m,X, parser Err or wrong length.

Operator and approximation: T'=T; h'=h+n-k,m'=m+k. Counter retry on the same cache counts attempts again. An Err is therefore not rollback of the whole cache state.

Assumptions: No parser access to private cache and no counter overflow.

Evidence and falsifiable oracle: Probe explicit Err and short Ok; repeat one failing call and verify doubled increments. See model.md#tests for executed bounds versus prospective work.

## order-and-clone

Type: output_projection. Status: partial_source_derived_model.

Domain and observation: Successful parse_with call with X of n occurrences.

Operator and approximation: Return [clone(T'[X_0]),...,clone(T'[X_(n-1)])]. Every duplicate key shares equal output despite different earlier parser answers; changing a returned Word later cannot mutate stored Word data. T' is a text-key map, not a per-occurrence log.

Assumptions: Word clone is value-deep for its owned fields at bound en implementation.

Evidence and falsifiable oracle: Probe mutates returned head and checks a subsequent cache hit preserves stored marker; indexed duplicate parser markers expose final-value projection. See model.md#tests for executed bounds versus prospective work.

## tests

### Checks

See ../model.md#tests and ../model.md#evidence for executed bounds. Models are conditional source-derived specifications; only the named probe observations are executed. The identity fragment has streaming/optional-hash evidence but no actual identity call. Batch and envelope fragments are exercised by finite probe oracles. Syntax ranges establish snapshot anchoring, not semantic correspondence.

## History

### 2026-10-29 · Epoch memory

2026-10-29, source SHA-256 da30cef6ff680f3c2853c158dada7d661307d0bbf0464bf058624bdd63d2c384. First source-bound instance; runtime source unchanged. Inspection exposed two important non-rollback effects: duplicate missing occurrences are sent separately and the final occurrence wins; hit/miss counters advance before parsing succeeds. Refresh is a persistent flag rather than a one-call invalidation. The source-bound partial AST is not semantic proof: no macro expansion, name resolution, cfg evaluation or type inference. General real identity/finish checks and concurrent-writer behavior remain untested. No speedup or complete parser-dependency guarantee is claimed.

### 2026-10-30 · Epoch memory

2026-10-30: whole-source identity refreshed after appending native tests;
this fragment's source range and selected syntax nodes are unchanged.
See [native transition domains](../model.md#transitions) and [current evidence](../model.md#evidence).
Identity computation and finish remain unexecuted; fake open_key identities
exercise only the cache transition/envelope boundary.

### 2026-10-30 · document

Subject: world/src/parsecache.rs at SHA-256 6545414761854d33b7b41f8a864a7f4c59eebeaee721eef68772bb4a7a652479; epoch 2026-10-30. Scope: source bytes [5254,6324). Transition horizon is one invocation unless an entry names a call history. Successful ordinary allocation; no usize counter overflow; a stable filesystem during each modeled read; no claim of parser linguistic correctness. Source inspection is evidence of a conditional specification, not a semantic proof. Evidence status is bounded_execution only where model.md#tests names the exact checked observation; all other oracles are prospective.

### 2026-10-30 · order-and-clone

Native 2026-10-30 refinements: [transition relations](../model.md#transitions); source scopes remain unchanged because tests were appended.
