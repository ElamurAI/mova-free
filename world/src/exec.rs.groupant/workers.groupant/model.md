# execution-ordered-workers

## descriptant

# execution-ordered-workers

## Forward

Compute cap-bounded chunk size; spawn slices in order, collect all joins, collect Result, then flatten.

## Invariant

Positive cap gives ordered disjoint intervals covering all inputs. No successful partial result follows a joined panic.

## Cost

O(n+t) vector aggregation plus job work; storage retains outputs and join results. Allocation, thread creation failure and scheduler fairness are outside this approximation.

## Composition

This cross-cutting primitive serves cap 12 CPU parse and cap 16 general/GPU operations. Parent must preserve Result handling; joining does not imply transactional rollback.

## Epoch memory

## catalog

# execution-ordered-workers: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U is positive-cap input/job-completion vectors paired with returned results. R joins every worker and concatenates in spawn order. U minus R contains Ok after a joined panic and Ok with a permutation different from ordered concatenation, under normal allocation/unwind.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Existing order and panic tests.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Given y and deterministic f, each compatible x has length len(y) and x_i in f^-1(y_i). All compatible positive caps remain possible because output omits partition boundaries.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate f(x)=x mod 2 on short sequences.

## chunk-envelope

Type: partition_geometry. Domain, observation and operator: For c=max(ceil(n/t),1), worker j covers [jc,min((j+1)c,n)). These intervals partition [0,n); n=0 spawns zero workers.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate n=0..101 and cap=1..16.

## panic-frontier

Type: failure_recovery. Domain, observation and operator: Err retains existence of at least one join panic but discards worker index, payload and successful outputs. par_map changes the observation to an expect panic; completed effects are not reversed.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Distinct panic payloads and completion counter.


## tests

### Checks

Existing focused check: cargo test -p world --lib exec::tests -- --nocapture. Named tests: batches_preserve_order_and_cardinality (101 mapped values and empty input); worker_panics_are_errors_instead_of_partial_results (four panicking jobs). The file-root ../model.md#evidence records actual execution. Mode caching, exit behavior, mixed worker outcomes, GPU shape guards and GPU/CPU agreement remain untested here.

No training, live service or GPU execution; no speedup or broad semantic correctness claim.

Native correspondence is now executed: full finite pointwise fibers, mixed panic prefixes, output-drop accounting and stateful ticket projections. See ../model.md#tests and ../model.md#worker-laws for domains and actual results. The parser and arbitrary callback effects remain outside these tests.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 180f88005a4cd3852f649059e420951c1fe9ad4a699c9d175358169f25e43b4d; bytes [929,2029). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-29 · document

Epoch: 2026-10-29. Subject: exec.rs, SHA-256 180f88005a4cd3852f649059e420951c1fe9ad4a699c9d175358169f25e43b4d, bytes [929,2029). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless ../model.md#evidence records execution.
