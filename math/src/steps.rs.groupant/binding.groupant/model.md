# math-steps-binding

## descriptant

### math-steps-binding

ignore_set v1 collects real quantities lacking question-context overlap, returning them only if>=2 real quantities remain. v2 uses target head/modifier matches over quantities with units; returns unmatched unit-bearing indices only if at least1 bound item, a nonempty ignored set and>=2 real survivors.

## approximant

### Scoped forward relation

ignore_set v1 collects real quantities lacking question-context overlap, returning them only if>=2 real quantities remain. v2 uses target head/modifier matches over quantities with units; returns unmatched unit-bearing indices only if at least1 bound item, a nonempty ignored set and>=2 real survivors.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Returned indices belong to real input quantities. Ignore selection is a heuristic feature/selection boundary, not proof that excluded numbers are irrelevant.

#### Cost

Membership checks repeatedly scan target/modifier/context lists; collecting bound/ignored indices uses Vec::contains. Training BindModel separately costs epochs*items*features and is not executed here.

## models

## behavior_complement

A const-prefixed index cannot enter the returned ignore set. A result leaving fewer than2 real quantities is excluded. Missing-unit quantities in v2 cannot be ignored merely for lacking a unit.

## reverse_reconstruction

With two bound apples plus one orange, [orange_index] can result for many subject/verb/number values. An empty set admits no targets, no unmatched units, or the survivor-count gate failing; it does not identify the reason.

## modifier_compatibility

v2 treats empty target modifiers or empty description modifiers as compatible; otherwise any shared modifier suffices. It does not demand full modifier-set equality.

## environment_dispatch

MATH_IGNORE_V=1 selects context-overlap v1; all other values use v2. Identical Prob can yield different ignore sets under these environments; constants are excluded in both.

## tests

### Scoped evidence

Static source review only; no ignore_set or BindModel execution.

Named actual source fixtures: none.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.
