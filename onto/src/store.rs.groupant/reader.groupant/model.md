# onto-store-reader

## descriptant

### onto-store-reader

Cursor p starts0; take(n) if n<=B-p returns b[p..p+n] and p'=p+n<=B

## approximant

### Scoped forward

Cursor p starts0; take(n) if n<=B-p returns b[p..p+n] and p'=p+n<=B

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

p0=0<=B; successful guarded step keeps p'<=B

u64→usize checked; u32 byte count checked_mul4

string requires String::from_utf8 after bounded bytes

Bounded slice before vector allocation, no total RAM guarantee

No executable percentage surrogate established.

## invariant

### Local observations

n>remaining excludes success and cursor advance; oversized n cannot overflow addition

p0=0<=B; successful guarded step keeps p'<=B

u64→usize checked; u32 byte count checked_mul4

string requires String::from_utf8 after bounded bytes

Bounded slice before vector allocation, no total RAM guarantee

## cost

### Scoped cost

O(n) copied array bytes; take itself O(1), cursor O(1).

Restriction to this operator's traversed inputs; parent total is not charged to every fragment. No standalone measurement.

## composition

### Parent join

reader supplies the parent relation: Cursor p starts0; take(n) if n<=B-p returns b[p..p+n] and p'=p+n<=B

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

n>remaining excludes success and cursor advance; oversized n cannot overflow addition

## reverse_reconstruction

Returned slice/new cursor recovers old p=p'-n; unrelated bytes unconstrained

## cursor_induction

p0=0<=B; successful guarded step keeps p'<=B

## array_length

u64→usize checked; u32 byte count checked_mul4

## utf8_gate

string requires String::from_utf8 after bounded bytes

## allocation_boundary

Bounded slice before vector allocation, no total RAM guarantee


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [1951,2948); shared parent AST. Excludes unrelated parent operations and added test module.
