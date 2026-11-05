# Supplied-tree skeleton

Scope: partial source-bound relations over the observations named below. Sufficient memory and terminating dependencies assumed. This is not a universal proof or an executable percentage-error approximant.

## forward

Reach root and selected Conj/Advcl/Ccomp/Parataxis dependents with allowed POS once, sort indices, render subjects/negation/modals/copulas and first selected role dependents. SAY renders subject, negation, original speech verb. phrase retains head plus direct possessive/adjective/compound children in index order.

## invariant

Empty tree gives empty output. Selected indices unique; duplicate strings are allowed here. Valid indices and aligned originals are premises; parser correctness excluded.

## cost

O(n*v) membership work plus O(d log d) per selected phrase and output allocation.

## composition

Tree::of_words constructs the child indices consumed by skeleton. Native tests execute this real join using synthetic Word values, without an annotator.

## behavior-complement

Type: behavior_complement. Subject: world-read-projection. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Valid supplied-tree/output universe: SAY does not render its Obj merely by Obj selection; no root implies no clauses. Source-derived inner exclusion, not textual truth.

## reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read-projection. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Unselected dependents can vary without changing projection. Eight subject/neg/object combinations and three speech objects are finite native observations; no inverse over all UD trees.

## carry-identity

Type: state_transition. Subject: world-read-projection. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Hold selected verbs ordered. Missing subject preserves last nonempty subject. Root plus subjectless conjunct fixture renders Alice twice; this is mechanical carry, not resolved discourse identity.

## checks

Native synthetic tests: cargo test -p world --lib read::wave9_models -- --test-threads=1. Test bodies define finite domains and fixtures. File-level ../model.md#evidence records repeated debug/release checks and a compiled negative control. No parser acquisition, teacher request, dictionary write or external assessment executed. Orchestration boundaries are source inspection only.

## epoch-memory

First scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

## wave34-composition

clause-carry discovers and sorts selected nodes, then calls phrase for explicit subjects and chosen roles. The carried subject is updated before copula/speech/action branch selection; phrase uses the same Tree/original index mapping. phrase cost is already included in the clause parent's rendering charge. Missing originals yield empty strings, while invalid Tree indices remain outside the declared domain. This composition does not acquire an annotator.

## History

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-read-projection, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Valid supplied-tree/output universe: SAY does not render its Obj merely by Obj selection; no root implies no clauses. Source-derived inner exclusion, not textual truth.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read-projection, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Unselected dependents can vary without changing projection. Eight subject/neg/object combinations and three speech objects are finite native observations; no inverse over all UD trees.

### 2026-10-30 · carry-identity

Type: state_transition. Subject: world-read-projection, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Hold selected verbs ordered. Missing subject preserves last nonempty subject. Root plus subjectless conjunct fixture renders Alice twice; this is mechanical carry, not resolved discourse identity.

### 2026-10-30 · epoch-memory

2026-10-30: first scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.
