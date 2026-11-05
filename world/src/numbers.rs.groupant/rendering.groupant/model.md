# number-rendering

## descriptant

# number-rendering

## Forward

Render bounded nonnegative cardinals/ordinals and scan first quantity.

## Invariant

Quantity never invokes correction; suffix teen exceptions precede unit rules.

## Cost

Bounded renderer has bounded lookups/output length; quantity costs token scan and fixed vocabulary lookups.

## Composition

Ordinal renderer replaces only final word of cardinal renderer; quantity uses separate token/read helpers.

## Epoch memory

## catalog

# number-rendering typed models

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U=renderer n in[0,3000) paired strings or quantity-text paired Option i64; R=scoped calls. Candidate U minus R includes 11st/12nd/13rd rendering and a solely ordinal quantity result.

Evidence/falsification: Existing roundtrips and probe suffix oracle. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Quantity q retains every text whose first accepted cardinal is q, with compatible earlier ordinals/nonnumeric tokens and arbitrary later text; twice/two both yield2.

Evidence/falsification: Probe literal fibers. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## ordinal-tail

Type: string_rewrite. Domain, observation and operator: Split cardinal output at last space/hyphen; replace final word through ORDINALS base lookup, retain head. Unsupported last word remains unchanged.

Evidence/falsification: Existing ordinal loop. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## first-hit

Type: ordered_selection. Domain, observation and operator: At each token once/twice/thrice precedes digit then word recognition; ordinal word spans advance used tokens. Later cardinal cannot override earlier one.

Evidence/falsification: Probe three apples then five. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.


## tests

### Checks

Root ../model.md#evidence records direct real-source Rust tests and bounded probes, including false-formula rejection. Catalog checks marked proposed remain unexecuted. Syntax/manifest validation checks identity and structure only. No GPU, live teacher, full application run or general proof.

## History

### 2026-10-29 · Epoch memory

2026-10-29: source-derived initial model, source hash 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Scope envelope [9902,12440) is anchored to selected partial syntax nodes, not semantic proof or full coverage. Runtime source unchanged. AST omits macros expansion, name resolution, inferred types and cfg evaluation. Generated dictionary/global tables are fixed dependencies not bound by this file hash. Catalog false-formula controls and boundary observations refine claims without modifying production.

### 2026-10-29 · document

Epoch 2026-10-29; subject numbers.rs, SHA-256 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Domain is the bounded calls/traces stated below; observation horizon is one invocation unless explicitly a trace. Fixed compiled global/dictionary tables, successful allocation, unpoisoned locks and nonoverflowing arithmetic are assumptions. Every relation is a partial source-derived specification; Rust correspondence beyond recorded checks remains open. Behavior complements target U minus R and restrictions are candidate inner bounds, not proven complements. Reverse relations preserve all compatible antecedents. Hash traversal is an explicit hidden input for values.
