# TSV parsing and last-write projection

Scope: partial source-bound relations over the observations named below. Sufficient memory and terminating dependencies assumed. This is not a universal proof or an executable percentage-error approximant.

## forward

Ignore lines starting '#'; split tabs, accept >=6 fields, lowercase first field, store empty date plus fields1..5. BTreeMap keeps last duplicate. Empty keys/fields not rejected when six fields exist.

## invariant

Sorted lowercase keys and six fields; no date acquisition. Projection is not full schema validation.

## cost

O(B+r log D) comparison-weighted parsing/map insertion; accepted strings stored.

## composition

improved supplies embedded bytes to improved_from. improved_dates returns empty map: parser performs no external date read.

## behavior-complement

Type: behavior_complement. Subject: world-meanings-records. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. TSV/output universe: fewer than six fields cannot create row; fields after index5 cannot change stored record. Source-derived inner absence, finite fixtures.

## reverse-reconstruction

Type: reverse_reconstruction. Subject: world-meanings-records. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Comments, malformed rows, extra columns and overridden earlier rows form compatible antecedents. No unique raw file or edit chronology recoverable.

## last-write

Type: state_transition. Subject: world-meanings-records. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Accepted rows sharing lowercase k reduce by replacement. Word then WORD retains second record; tests compare all six fields and extra-column equality.

## checks

Native synthetic tests: cargo test -p world --lib meanings::wave9_models -- --test-threads=1. Test bodies define finite domains and fixtures. File-level ../model.md#evidence records repeated debug/release checks and a compiled negative control. No parser acquisition, teacher request, dictionary write or external assessment executed. Orchestration boundaries are source inspection only.

## epoch-memory

First scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

## History

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-meanings-records, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. TSV/output universe: fewer than six fields cannot create row; fields after index5 cannot change stored record. Source-derived inner absence, finite fixtures.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-meanings-records, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Comments, malformed rows, extra columns and overridden earlier rows form compatible antecedents. No unique raw file or edit chronology recoverable.

### 2026-10-30 · last-write

Type: state_transition. Subject: world-meanings-records, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Accepted rows sharing lowercase k reduce by replacement. Word then WORD retains second record; tests compare all six fields and extra-column equality.

### 2026-10-30 · epoch-memory

2026-10-30: first scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.
