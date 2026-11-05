# Book segmentation fold

Scope: partial source-bound relations over the observations named below. Sufficient memory and terminating dependencies assumed. This is not a universal proof or an executable percentage-error approximant.

## forward

Fold over CR-trimmed lines holding optional title, completed paragraphs and pending text. Heading flushes old story; blank flushes paragraph; EOF flushes story. Heading requires preceding blank plus capital-letter or centered-title predicate.

## invariant

Stable story/paragraph order; retained paragraphs have >=4 words and do not start '['. Admission precedes filtering, so retained total can be <200. Clause indices are sorted; syntax does not prove semantic faithfulness.

## cost

O(L) scans/storage plus allocations.

## composition

Title recognition needs original indentation before whitespace normalization; admission and service filtering do not commute.

## behavior-complement

Type: behavior_complement. Subject: world-read-segmentation. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Text/output universe: output excludes unheaded preamble, excluded exact titles, bracket-starting or fewer-than-four-word paragraphs. Source-derived inner bound; arbitrary book layouts unknown.

## reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read-segmentation. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Compatible antecedents include whitespace variants preserving heading predicates, deleted service paragraphs and discarded headers. Lost byte layout cannot be reconstructed.

## threshold-fiber

Type: constraint_relation. Subject: world-read-segmentation. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. One uppercase heading and one four-word paragraph of n bytes, n=180..220, produces a story iff n>=200. Service filtering can subsequently drop total below 200; native fixtures distinguish stage order.

## checks

Native synthetic tests: cargo test -p world --lib read::wave9_models -- --test-threads=1. Test bodies define finite domains and fixtures. File-level ../model.md#evidence records repeated debug/release checks and a compiled negative control. No parser acquisition, teacher request, dictionary write or external assessment executed. Orchestration boundaries are source inspection only.

## epoch-memory

First scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

## wave34-composition

heading decides when the ordered line fold calls admission and replaces cur; blank lines append normalized pending paragraphs. EOF calls admission. Only after all admissions does service filter paragraphs then stories. Shared title/paragraph/buffer state and admission-before-filter order constrain the joint relation; separate child preimages must agree on that intermediate state. Missing headings discard preamble, and subthreshold flush consumes cur. Scanning is charged once at parent level, not summed across overlapping child observations.

## History

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-read-segmentation, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Text/output universe: output excludes unheaded preamble, excluded exact titles, bracket-starting or fewer-than-four-word paragraphs. Source-derived inner bound; arbitrary book layouts unknown.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read-segmentation, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Compatible antecedents include whitespace variants preserving heading predicates, deleted service paragraphs and discarded headers. Lost byte layout cannot be reconstructed.

### 2026-10-30 · threshold-fiber

Type: constraint_relation. Subject: world-read-segmentation, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. One uppercase heading and one four-word paragraph of n bytes, n=180..220, produces a story iff n>=200. Service filtering can subsequently drop total below 200; native fixtures distinguish stage order.

### 2026-10-30 · epoch-memory

2026-10-30: first scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.
