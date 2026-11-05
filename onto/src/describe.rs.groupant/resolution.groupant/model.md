# onto-describe-resolution

## descriptant

### onto-describe-resolution

resolve(w)=explicit_id(w) or context_core(w) or bridge(w)

## approximant

### Scoped forward

resolve(w)=explicit_id(w) or context_core(w) or bridge(w)

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Core pick context equality, or absent context/single candidate

None iff all ordered branches fail

Absent core synset falls through bridge

Resolution does not alter Graph/Kb

No executable percentage surrogate established.

## invariant

### Local observations

Existing explicit id excludes competing lemma result

Core pick context equality, or absent context/single candidate

None iff all ordered branches fail

Absent core synset falls through bridge

Resolution does not alter Graph/Kb

## cost

### Scoped cost

Explicit synset lookup then core lemma scan and bridge lookups; O(core entries + candidate lookup cost) worst case, O(word bytes) spelling intermediates. No graph traversal or output report allocation here.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

resolution supplies the parent relation: resolve(w)=explicit_id(w) or context_core(w) or bridge(w)

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Existing explicit id excludes competing lemma result

## reverse_reconstruction

Cid fiber admits ID strings and lemma variants through distinct branches

## context_gate

Core pick context equality, or absent context/single candidate

## missing_fiber

None iff all ordered branches fail

## core_bridge

Absent core synset falls through bridge

## read_only

Resolution does not alter Graph/Kb


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [2544,3331); shared parent AST. Excludes unrelated parent operations and added test module.
