# onto-kb-seed-gates

## descriptant

### onto-kb-seed-gates

insert(a)=validate duplicate,same-node conflict,strict ancestry; then append+index

## approximant

### Scoped forward

insert(a)=validate duplicate,same-node conflict,strict ancestry; then append+index

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Same layer/property opposite value with strict ancestor fails both insertion directions

Duplicate before conflict before strict

One insertion atomic; file sequence not atomic

by_concept records exact appended index

No executable percentage surrogate established.

## invariant

### Local observations

Rejected seed excludes mutation; gates=false cannot bypass duplicate/same-node conflict

Same layer/property opposite value with strict ancestor fails both insertion directions

Duplicate before conflict before strict

One insertion atomic; file sequence not atomic

by_concept records exact appended index

## cost

### Scoped cost

Duplicate/same-node scans O(A); ancestor(a) BFS plus potentially one BFS per opposite strict ancestor check. Temporary ancestors can occupy O(Va) each sequentially. Successful index append expected O(1) after scans; failure before mutation.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

seed-gates supplies the parent relation: insert(a)=validate duplicate,same-node conflict,strict ancestry; then append+index

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Rejected seed excludes mutation; gates=false cannot bypass duplicate/same-node conflict

## reverse_reconstruction

Successful appended state recoverable by removing last seed/index if no public mutation

## strict_relation

Same layer/property opposite value with strict ancestor fails both insertion directions

## failure_order

Duplicate before conflict before strict

## row_transaction

One insertion atomic; file sequence not atomic

## index_invariant

by_concept records exact appended index


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [15201,16694); shared parent AST. Excludes unrelated parent operations and added test module.
