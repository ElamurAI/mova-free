# value-fields

## descriptant

# value-fields

## Forward

Tokenize whitespace outside toggled quotes; skip caret tokens; split first equals and trim quote ends; gaps queries catalogs.

## Invariant

Duplicate keys overwrite; map iteration is lexical, positional order preserved; unbalanced quotes are tolerated.

## Cost

O(B+K log(1+F)) parsing for K key/value tokens and F distinct keys, plus string comparison costs; exact gap dedup can cost O(g^2) comparisons plus catalogs.

## Composition

Shared fields feeds judge/gaps; gaps inspects fewer state keys than judge.

## Epoch memory

## catalog

# value-fields typed models

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U=text x fields/gap observations; R=quote-toggle parsing and gap walk. Candidate U minus R includes caret positional token, multiple simultaneous values for same key or unknown-gap report for known value.

Evidence/falsification: Probe duplicate key/unknown verb. See root ../model.md#current-evidence for current execution and ../model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Parsed map fibers include reordered unique assignments and arbitrary overwritten earlier duplicates. Quoted whitespace constrains tokens; gap fibers permit ignored commands/keys.

Evidence/falsification: Direct parser enumeration proposed. See root ../model.md#current-evidence for current execution and ../model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## last-write

Type: overwrite_relation. Domain, observation and operator: kv[k] is last parsed value for k; quote trimming is not escape handling and first equals divides key/value.

Evidence/falsification: Probe sincere=1 sincere=0. See root ../model.md#current-evidence for current execution and ../model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## gap-dedup

Type: set_projection. Domain, observation and operator: First exact diagnostic string survives; within-set diagnostic order follows lexical field keys. Distinct keys with same unknown value give distinct diagnostics.

Evidence/falsification: Probe repeated unknown event. See root ../model.md#current-evidence for current execution and ../model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.


## tests

### Checks

Root ../model.md#evidence records direct real-source Rust tests and bounded probes, including false-formula rejection. Catalog checks marked proposed remain unexecuted. Syntax/manifest validation checks identity and structure only. No GPU, live teacher, full application run or general proof.

## History

### 2026-10-29 · Epoch memory

2026-10-29: source-derived initial model, source hash f450843ba1c92f35884eb71667d432f36a34f6316c9b8a252811e0dac87ddbe6. Scope envelope [1459,3911) is anchored to selected partial syntax nodes, not semantic proof or full coverage. Runtime source unchanged. AST omits macros expansion, name resolution, inferred types and cfg evaluation. Generated dictionary/global tables are fixed dependencies not bound by this file hash. Catalog false-formula controls and boundary observations refine claims without modifying production.

2026-10-29 current policy epoch: source b885d25364939e38e130ff598a24a46efd21f500ba51e27cd4465ceddb56f064. Regenerated syntax and reselected source-node envelopes replace historical anchors above. Historical unchanged-source claims refer only to their epoch. Current correspondence and dependency limits appear in the file-root ../model.md#current-evidence.

### 2026-10-29 · document

Epoch 2026-10-29; subject values.rs, SHA-256 b885d25364939e38e130ff598a24a46efd21f500ba51e27cd4465ceddb56f064. Domain is the bounded calls/traces stated below; observation horizon is one invocation unless explicitly a trace. Fixed compiled global/dictionary tables, successful allocation, unpoisoned locks and nonoverflowing arithmetic are assumptions. Every relation is a partial source-derived specification; Rust correspondence beyond recorded checks remains open. Behavior complements target U minus R and restrictions are candidate inner bounds, not proven complements. Reverse relations preserve all compatible antecedents. Fields and goal peers use lexical order; harm eligibility uses command-start history. Historical traversal evidence is retained separately.
