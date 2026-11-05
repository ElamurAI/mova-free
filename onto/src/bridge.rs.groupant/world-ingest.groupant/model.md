# onto-bridge-world-ingest

## descriptant

### onto-bridge-world-ingest

Files→first unique content hash→used filename→typed char/obj/loc lines

## approximant

### Scoped forward

Files→first unique content hash→used filename→typed char/obj/loc lines

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

key requires leading space and =; quotes require closing quote

First identical bytes wins tale naming

UTF8 read error returns Err before processing that file

Reads only; no writes to source world

No executable percentage surrogate established.

## invariant

### Local observations

Missing type excludes Ent; later same-content copy excludes used/entity extension

key requires leading space and =; quotes require closing quote

First identical bytes wins tale naming

UTF8 read error returns Err before processing that file

Reads only; no writes to source world

## cost

### Scoped cost

O(B) read/hash/line scans across supplied files, O(E) emitted entities and O(U) distinct digest/used records. Entire current file string allocated; duplicate text is still read and hashed. File-read failure stops iteration.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

world-ingest supplies the parent relation: Files→first unique content hash→used filename→typed char/obj/loc lines

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Missing type excludes Ent; later same-content copy excludes used/entity extension

## reverse_reconstruction

Ent preimage permits ignored fields/lines but first unique filename determines tale

## field_boundary

key requires leading space and =; quotes require closing quote

## file_order

First identical bytes wins tale naming

## failure_prefix

UTF8 read error returns Err before processing that file

## effect_exclusion

Reads only; no writes to source world


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [932,2778); shared parent AST. Excludes unrelated parent operations and added test module.
