# number-reading

## descriptant

# number-reading

## Forward

Accumulate cur/total according to accepted kinds; ordinal terminates and scale flushes max(cur,1)*v.

## Invariant

Some word parse has used>0; nonoverflowing arithmetic assumed, no descending-scale guarantee.

## Cost

O(k*V) lookups for consumed k tokens with fixed V.

## Composition

Digit/word readers are alternative branches; caller lowercases and separately applies fraction/context guards.

## Epoch memory

## catalog

# number-reading typed models

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U=tagged bounded word-token or digit requests x typed results; R=reader transition. Candidate U minus R includes successful zero-token consumption and digit success with arbitrary alphabetic suffix.

Evidence/falsification: Existing and probe digit fixtures. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Value fibers include a hundred/one hundred, comma digit aliases and four ordinal suffix spellings independent of number. API tag and used count constrain but do not uniquify antecedents.

Evidence/falsification: Probe selected aliases. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## scale-accumulator

Type: state_transition. Domain, observation and operator: Scale sets total'=total+max(cur,1)*v,cur'=0; hundred sets cur'=max(cur,1)*100. Kind fits can admit unconventional scale ordering; not full English grammar.

Evidence/falsification: Generated short kind traces proposed. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## and-bridge

Type: reachability_precondition. Domain, observation and operator: and skips only after Hundred/Scale before Unit/Teen/Tens. a starts only before hundred/thousand/million/dozen, excluding billion. Neither bridge alone updates used.

Evidence/falsification: Existing hundred-and-five; a-billion oracle proposed. See root ../model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.


## tests

### Checks

Root ../model.md#evidence records direct real-source Rust tests and bounded probes, including false-formula rejection. Catalog checks marked proposed remain unexecuted. Syntax/manifest validation checks identity and structure only. No GPU, live teacher, full application run or general proof.

## History

### 2026-10-29 · Epoch memory

2026-10-29: source-derived initial model, source hash 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Scope envelope [6800,9778) is anchored to selected partial syntax nodes, not semantic proof or full coverage. Runtime source unchanged. AST omits macros expansion, name resolution, inferred types and cfg evaluation. Generated dictionary/global tables are fixed dependencies not bound by this file hash. Catalog false-formula controls and boundary observations refine claims without modifying production.

### 2026-10-29 · document

Epoch 2026-10-29; subject numbers.rs, SHA-256 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Domain is the bounded calls/traces stated below; observation horizon is one invocation unless explicitly a trace. Fixed compiled global/dictionary tables, successful allocation, unpoisoned locks and nonoverflowing arithmetic are assumptions. Every relation is a partial source-derived specification; Rust correspondence beyond recorded checks remains open. Behavior complements target U minus R and restrictions are candidate inner bounds, not proven complements. Reverse relations preserve all compatible antecedents. Hash traversal is an explicit hidden input for values.
