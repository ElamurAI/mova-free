# onto-pron-cue-parser

## descriptant

### onto-pron-cue-parser

F=first female cue,M=first male cue; output iff exactly one exists

## approximant

### Scoped forward

F=first female cue,M=first male cue; output iff exactly one exists

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Unicode alphanumeric split then lowercase

Within selected class first textual occurrence returned

Both and neither map None, distinct antecedents

No graph or filesystem effects

No executable percentage surrogate established.

## invariant

### Local observations

queen and king excludes direct gender cue

Unicode alphanumeric split then lowercase

Within selected class first textual occurrence returned

Both and neither map None, distinct antecedents

No graph or filesystem effects

## cost

### Scoped cost

O(text characters) and fixed-list membership scans.

Restriction to this operator's traversed inputs; parent total is not charged to every fragment. No standalone measurement.

## composition

### Parent join

cue-parser supplies the parent relation: F=first female cue,M=first male cue; output iff exactly one exists

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

queen and king excludes direct gender cue

## reverse_reconstruction

Preimage contains arbitrary noncue words and case/punctuation, excludes opposite cue

## token_coordinates

Unicode alphanumeric split then lowercase

## first_cue

Within selected class first textual occurrence returned

## ambiguous_none

Both and neither map None, distinct antecedents

## no_effect

No graph or filesystem effects


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [2665,3151); shared parent AST. Excludes unrelated parent operations and added test module.
