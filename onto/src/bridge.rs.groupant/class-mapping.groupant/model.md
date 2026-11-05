# onto-bridge-class-mapping

## descriptant

### onto-bridge-class-mapping

class→first successful(exact,variants,head); sense=first context match else senses[0]

## approximant

### Scoped forward

class→first successful(exact,variants,head); sense=first context match else senses[0]

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Exact<Variant<Head<Missing

All contexts mapping to lexfile(c) can return matched=true

Different hyphen/underscore forms can share head result

Missing returns(None,false); Found must have at least one sense

No executable percentage surrogate established.

## invariant

### Local observations

Head excludes any earlier-tier success; Missing excludes all candidates

Exact<Variant<Head<Missing

All contexts mapping to lexfile(c) can return matched=true

Different hyphen/underscore forms can share head result

Missing returns(None,false); Found must have at least one sense

## cost

### Scoped cost

At most four lookup attempts plus one final-head lookup; each delegates binary word lookup and sense collection. Context selection O(S) per successful tier; allocates spelling variants O(class bytes). No file I/O.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

class-mapping supplies the parent relation: class→first successful(exact,variants,head); sense=first context match else senses[0]

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Head excludes any earlier-tier success; Missing excludes all candidates

## reverse_reconstruction

(tier,c,matched)⁻¹ includes all class spellings satisfying earlier-tier emptiness and selected sense order

## tier_order

Exact<Variant<Head<Missing

## context_fiber

All contexts mapping to lexfile(c) can return matched=true

## fallback_identity

Different hyphen/underscore forms can share head result

## empty_domain

Missing returns(None,false); Found must have at least one sense


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [3995,5336); shared parent AST. Excludes unrelated parent operations and added test module.
