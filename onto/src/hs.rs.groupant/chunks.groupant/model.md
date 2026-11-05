# onto-hs-chunks

## descriptant

### onto-hs-chunks

(tokens,exclude,cont,level)→ordered branch features; parenthesis restores exclude/cont

## approximant

### Scoped forward

(tokens,exclude,cont,level)→ordered branch features; parenthesis restores exclude/cont

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Bare NP inherits Of/Has/Purpose/Exclude at level

not toggles neg0 except noun-list exclusion

First maximal-level Item else Head

u8 colon depth requires<=255 to avoid overflow

No executable percentage surrogate established.

## invariant

### Local observations

whether excludes distinguishing feat; excluded nouns cannot become Head/Item

Bare NP inherits Of/Has/Purpose/Exclude at level

not toggles neg0 except noun-list exclusion

First maximal-level Item else Head

u8 colon depth requires<=255 to avoid overflow

## cost

### Scoped cost

Token index vectors and repeated noun-group/tail scans can take O(T²) time and copying; state vectors O(colon levels), features O(emissions). Parenthesis state cloning adds O(levels) per nested chunk. Recursive tail depth/resource exhaustion unbounded outside finite domain.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

chunks supplies the parent relation: (tokens,exclude,cont,level)→ordered branch features; parenthesis restores exclude/cont

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

whether excludes distinguishing feat; excluded nouns cannot become Head/Item

## reverse_reconstruction

Many filler/punctuation variants yield same features with same branch states

## continuation

Bare NP inherits Of/Has/Purpose/Exclude at level

## negation

not toggles neg0 except noun-list exclusion

## main_selection

First maximal-level Item else Head

## level_boundary

u8 colon depth requires<=255 to avoid overflow


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [31095,54284); shared parent AST. Excludes unrelated parent operations and added test module.
