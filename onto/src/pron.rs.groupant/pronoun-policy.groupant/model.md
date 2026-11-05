# onto-pron-pronoun-policy

## descriptant

### onto-pron-pronoun-policy

Ordered cases on(human,persona,gender) produce a fixed allowed singular pronoun slice

## approximant

### Scoped forward

Ordered cases on(human,persona,gender) produce a fixed allowed singular pronoun slice

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

[he] requires Masc and human Yes or persona

[she] requires Fem and human Yes or persona

Nonhuman/nonpersona cue returns it then he/she

[it] loses distinction Neut versus Unknown

No executable percentage surrogate established.

## invariant

### Local observations

Human Yes excludes only-it; neutral persona excludes only-he/she

[he] requires Masc and human Yes or persona

[she] requires Fem and human Yes or persona

Nonhuman/nonpersona cue returns it then he/she

[it] loses distinction Neut versus Unknown

## cost

### Scoped cost

Fixed ordered match has O(1) time and zero heap allocation; returns a static slice of one to three string literals. It does not charge the caller's Kb/WSD computations or inspect why text.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

pronoun-policy supplies the parent relation: Ordered cases on(human,persona,gender) produce a fixed allowed singular pronoun slice

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Human Yes excludes only-it; neutral persona excludes only-he/she

## reverse_reconstruction

[he,she,it] implies persona true,human!=Yes,gender neither Masc nor Fem; animate/concept/why unconstrained

## masc_fiber

[he] requires Masc and human Yes or persona

## fem_fiber

[she] requires Fem and human Yes or persona

## animal_gender

Nonhuman/nonpersona cue returns it then he/she

## default_fiber

[it] loses distinction Neut versus Unknown


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [1495,2088); shared parent AST. Excludes unrelated parent operations and added test module.
