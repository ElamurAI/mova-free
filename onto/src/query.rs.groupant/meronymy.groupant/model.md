# onto-query-meronymy

## descriptant

### onto-query-meronymy

owners=sort ancestors(depth,Cid); walk incoming PartOf<=3, MemberOf/MadeOf at depth0

## approximant

### Scoped forward

owners=sort ancestors(depth,Cid); walk incoming PartOf<=3, MemberOf/MadeOf at depth0

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Dedup by(part,rel), not owner

Inherited owner carries BFS taxonomic Chain

Incoming PartOf/MemberOf; outgoing MadeOf

Missing lexical or part witness→Unknown

No executable percentage surrogate established.

## invariant

### Local observations

Depth4 parts and recursive members excluded; absent has_part witness excludes No

Dedup by(part,rel), not owner

Inherited owner carries BFS taxonomic Chain

Incoming PartOf/MemberOf; outgoing MadeOf

Missing lexical or part witness→Unknown

## cost

### Scoped cost

Sort Va owners O(Va log Va); traverse incoming PartOf frontiers to depth3 for each owner with global pair dedup. Work proportional to visited adjacency plus owner chains/path clones; has_part further computes ancestry per candidate part and compares target senses.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

meronymy supplies the parent relation: owners=sort ancestors(depth,Cid); walk incoming PartOf<=3, MemberOf/MadeOf at depth0

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Depth4 parts and recursive members excluded; absent has_part witness excludes No

## reverse_reconstruction

Returned path constrains first owner and CSR edges; later duplicates/deeper edges unconstrained

## seen_relation

Dedup by(part,rel), not owner

## owner_proof

Inherited owner carries BFS taxonomic Chain

## part_direction

Incoming PartOf/MemberOf; outgoing MadeOf

## unknown_boundary

Missing lexical or part witness→Unknown


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [14379,18068); shared parent AST. Excludes unrelated parent operations and added test module.
