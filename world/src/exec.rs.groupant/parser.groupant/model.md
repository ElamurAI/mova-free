# execution-mode-parser

## descriptant

# execution-mode-parser

## Forward

Latch requested mode; acquire annotator and execute either CPU map or GPU cached load/gather/predict/check/complete/scatter.

## Invariant

Successful GPU result preserves sentence count under shape guards, but labels and normalization are lossy; CPU/GPU accuracy agreement is unmeasured.

## Cost

CPU pays parse and scheduling work; GPU stores cloned nonempty tokens, predictions and completion results, proportional to reached token count plus cached model.

## Composition

Ordered-worker relation supplies tokenization and completion order. Model load and mode have independent OnceLocks. Parent holds exact/approximate backend distinction.

## Epoch memory

## catalog

# execution-mode-parser: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U is build/environment/backend-shape to tagged outcome pairs. R applies mode selection and the selected pipeline. U minus R contains CPU-only GPU success and GPU success with mismatched sentence/token dimensions; accuracy is unrestricted.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Configuration matrix and prediction stubs.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Observed cached Cpu permits missing, invalid-Unicode or cpu first lookup and arbitrary later edits. Gpu requires feature plus initial gpu. Output alone cannot recover the environment history.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Isolated environment-history probes.

## empty-scatter

Type: projection_fiber. Domain, observation and operator: I is the ordered nonempty-token position list; gather x[I], complete predictions and scatter into n empty vectors. Empty outputs do not uniquely imply empty tokenization because completion may also be empty.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Two empty and two nonempty positions with stub completion.

## load-failure-memory

Type: sticky_failure. Domain, observation and operator: OnceLock P stores first load Result; after successful annotator lookup, all later model-path edits preserve a cached load error. This failure fiber does not include attempts blocked earlier by annotator failure.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Loader-call counter with changed path.


## tests

### Checks

Existing focused check: cargo test -p world --lib exec::tests -- --nocapture. Named tests: batches_preserve_order_and_cardinality (101 mapped values and empty input); worker_panics_are_errors_instead_of_partial_results (four panicking jobs). The file-root ../model.md#evidence records actual execution. Mode caching, exit behavior, mixed worker outcomes, GPU shape guards and GPU/CPU agreement remain untested here.

No training, live service or GPU execution; no speedup or broad semantic correctness claim.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 180f88005a4cd3852f649059e420951c1fe9ad4a699c9d175358169f25e43b4d; bytes [2029,6328). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-29 · document

Epoch: 2026-10-29. Subject: exec.rs, SHA-256 180f88005a4cd3852f649059e420951c1fe9ad4a699c9d175358169f25e43b4d, bytes [2029,6328). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless ../model.md#evidence records execution.
