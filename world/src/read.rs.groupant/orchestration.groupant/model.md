# Parser, JSONL and external-check boundaries

Scope: partial source-bound relations over the observations named below. Sufficient memory and terminating dependencies assumed. This is not a universal proof or an executable percentage-error approximant.

## forward

summarize toggles quote state on every straight/open/close double quote and inserts ellipsis at entry; parses unquoted text, first-occurrence deduplicates. read_book writes after all selected summaries. check interprets response lines by in-chunk index.

## invariant

No quote nesting/balance guarantee. check accepts repeat indices and unbounded parsed rating values. Evidence stops before annotator/teacher acquisition.

## cost

External parse/call costs unknown. Successful asks ceil(rows/batch), batch>0 and no failure. Rows/output held in memory.

## composition

CLI read/check are separate. Serialized paragraph identity is the join. All effects here are source-inspected, not executed.

## behavior-complement

Type: behavior_complement. Subject: world-read-orchestration. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Successful fixed-response universe, batch>0: missing in-chunk index or missing parsable score cannot create accepted check row. Inner source-inspected relation; external failures unknown.

## reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read-orchestration. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Summary loses quoted speech and duplicate strings. check.jsonl cannot recover skipped lines or source text without rows/traces. Multiple histories produce the same projection.

## partial-effects

Type: effect_boundary. Subject: world-read-orchestration. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Directory creation precedes read; append_call precedes response processing; output replacement follows all batches. Failure prefixes can retain calls without final check output. No rollback law.

## checks

Native synthetic tests: cargo test -p world --lib read::wave9_models -- --test-threads=1. Test bodies define finite domains and fixtures. File-level ../model.md#evidence records repeated debug/release checks and a compiled negative control. No parser acquisition, teacher request, dictionary write or external assessment executed. Orchestration boundaries are source inspection only.

## epoch-memory

First scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

## History

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-read-orchestration, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Successful fixed-response universe, batch>0: missing in-chunk index or missing parsable score cannot create accepted check row. Inner source-inspected relation; external failures unknown.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read-orchestration, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Summary loses quoted speech and duplicate strings. check.jsonl cannot recover skipped lines or source text without rows/traces. Multiple histories produce the same projection.

### 2026-10-30 · partial-effects

Type: effect_boundary. Subject: world-read-orchestration, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Directory creation precedes read; append_call precedes response processing; output replacement follows all batches. Failure prefixes can retain calls without final check output. No rollback law.

### 2026-10-30 · epoch-memory

2026-10-30: first scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.
