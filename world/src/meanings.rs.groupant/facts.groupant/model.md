# Fact gates and append identity

Scope: partial source-bound relations over the observations named below. Sufficient memory and terminating dependencies assumed. This is not a universal proof or an executable percentage-error approximant.

## forward

facts_uncached finds sentences mentioning entry, normalizes numbers, requires self/pronoun/copula anchoring, then passes bounded token prefix to facts_in. Helper scans from offset, appends n>1 after approved predecessor, stems eligible next token; neither clears nor deduplicates.

## invariant

Existing output prefix preserved; from>=len is identity; initial bare number refused. Stems are not validated dimensions.

## cost

O(n.saturating_sub(from)) helper token scan plus stem/output allocation. Upstream normalization and dictionary lookup extra.

## composition

Upstream anchoring/window selection constrain facts_in. Helper oracle does not prove source sentence refers to intended entity.

## behavior-complement

Type: behavior_complement. Subject: world-meanings-facts. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Supplied slice universe excludes appended n<=1, initial number and numbers after was/near. Empty units are allowed. Inner relation has native bounded evidence.

## reverse-reconstruction

Type: reverse_reconstruction. Subject: world-meanings-facts. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Approved predecessor spellings and noun forms with same stem share fact fibers. Empty result cannot identify no numbers versus refused numbers versus exhausted offset.

## append-identity

Type: state_transition. Subject: world-meanings-facts. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. F(tokens,from,out)=out concatenated with accepted facts. from>=len is identity. Eleven blocked-unit fixtures preserve seeded pair; no entity-resolution guarantee follows.

## checks

Native synthetic tests: cargo test -p world --lib meanings::wave9_models -- --test-threads=1. Test bodies define finite domains and fixtures. File-level ../model.md#evidence records repeated debug/release checks and a compiled negative control. No parser acquisition, teacher request, dictionary write or external assessment executed. Orchestration boundaries are source inspection only.

## epoch-memory

First scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

## History

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-meanings-facts, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Supplied slice universe excludes appended n<=1, initial number and numbers after was/near. Empty units are allowed. Inner relation has native bounded evidence.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-meanings-facts, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Approved predecessor spellings and noun forms with same stem share fact fibers. Empty result cannot identify no numbers versus refused numbers versus exhausted offset.

### 2026-10-30 · append-identity

Type: state_transition. Subject: world-meanings-facts, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. F(tokens,from,out)=out concatenated with accepted facts. from>=len is identity. Eleven blocked-unit fixtures preserve seeded pair; no entity-resolution guarantee follows.

### 2026-10-30 · epoch-memory

2026-10-30: first scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.
