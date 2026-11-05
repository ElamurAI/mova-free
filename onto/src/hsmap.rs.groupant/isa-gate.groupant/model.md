# onto-hsmap-isa-gate

## descriptant

### onto-hsmap-isa-gate

Verdict=first witness in ancestry,other-sense,part,taxon,modifier,missing-head,made,source,LCA

## approximant

### Scoped forward

Verdict=first witness in ancestry,other-sense,part,taxon,modifier,missing-head,made,source,LCA

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Only first60 incoming MemberOf edges examined

query::parts_of includes inherited bounded meronym paths

Min sum depth then Cid; Near requires head depth1

No chosen head yields Unknown before material/LCA

No executable percentage surrogate established.

## invariant

### Local observations

Direct ancestor excludes weaker Part/OtherSense despite simultaneous evidence

Only first60 incoming MemberOf edges examined

query::parts_of includes inherited bounded meronym paths

Min sum depth then Cid; Near requires head depth1

No chosen head yields Unknown before material/LCA

## cost

### Scoped cost

Ancestry BFS for chosen sense, alternatives and chosen heads; query::parts_of adds owner/meronym work. Taxon scan at most60 member edges, each may BFS. LCA scans chosen ancestry; no teacher or filesystem effect.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

isa-gate supplies the parent relation: Verdict=first witness in ancestry,other-sense,part,taxon,modifier,missing-head,made,source,LCA

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Direct ancestor excludes weaker Part/OtherSense despite simultaneous evidence

## reverse_reconstruction

Yes iff an ancestor witness in heads_all exists at first gate; Part requires earlier failures

## taxon_cap

Only first60 incoming MemberOf edges examined

## part_join

query::parts_of includes inherited bounded meronym paths

## lca_tie

Min sum depth then Cid; Near requires head depth1

## fallback_domain

No chosen head yields Unknown before material/LCA


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [11385,14995); shared parent AST. Excludes unrelated parent operations and added test module.
