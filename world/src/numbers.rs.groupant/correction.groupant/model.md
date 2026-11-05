# number-correction

## descriptant

# number-correction

## Forward

Weighted OSA ranks vocabulary by distance then integer Dice then fixed order; exact input string memoizes result.

## Invariant

Exact words/dozen return None; under-three-byte or non-ASCII-letter strings reject.

## Cost

O(m*n) distance time and matrix space per candidate; unbounded distinct-key cache.

## Composition

Trigram sets supply secondary score; caller normalize separately gates contextual application.

## Epoch memory

## catalog

# number-correction typed models

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U=strings x Option vocabulary; R=correct. Candidate inner bound U minus R includes Some for exact words, short strings or non-ASCII input, or winners above candidate threshold.

Evidence/falsification: Probe rejection literals. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Some(v) fiber is every admitted string with eligible lexicographically best v; None combines admission rejection and no eligible candidate. Cache state remains ambiguous.

Evidence/falsification: Finite alphabet oracle proposed. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## transposition-weight

Type: weighted_metric. Domain, observation and operator: Insertion/deletion/substitution cost2; adjacent transposition cost1 via d[i-2][j-2]. This is optimal-string-alignment, not unrestricted Damerau distance.

Evidence/falsification: Probe ab/ba and false weight2 formula. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## tie-stability

Type: selection_order. Domain, observation and operator: Equal-distance candidate replaces incumbent only for strictly greater integer Dice; exact ties retain cardinal/ordinal/dozen vocabulary order. Integer division can coalesce distinct rational scores.

Evidence/falsification: Equal-score enumeration proposed. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.


## tests

### Checks

Root ../model.md#evidence records direct real-source Rust tests and bounded probes, including false-formula rejection. Catalog checks marked proposed remain unexecuted. Syntax/manifest validation checks identity and structure only. No GPU, live teacher, full application run or general proof.

## History

### 2026-10-29 · Epoch memory

2026-10-29: source-derived initial model, source hash 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Scope envelope [4010,6542) is anchored to selected partial syntax nodes, not semantic proof or full coverage. Runtime source unchanged. AST omits macros expansion, name resolution, inferred types and cfg evaluation. Generated dictionary/global tables are fixed dependencies not bound by this file hash. Catalog false-formula controls and boundary observations refine claims without modifying production.

### 2026-10-29 · document

Epoch 2026-10-29; subject numbers.rs, SHA-256 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Domain is the bounded calls/traces stated below; observation horizon is one invocation unless explicitly a trace. Fixed compiled global/dictionary tables, successful allocation, unpoisoned locks and nonoverflowing arithmetic are assumptions. Every relation is a partial source-derived specification; Rust correspondence beyond recorded checks remains open. Behavior complements target U minus R and restrictions are candidate inner bounds, not proven complements. Reverse relations preserve all compatible antecedents. Hash traversal is an explicit hidden input for values.
