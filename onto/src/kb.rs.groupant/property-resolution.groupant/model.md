# onto-kb-property-resolution

## descriptant

### onto-kb-property-resolution

A_l=visible ancestor assertions on layer l; l*=first nonempty layer; M=descendant-minimal concept candidates; Known iff values(M) singleton

## approximant

### Scoped forward

A_l=visible ancestor assertions on layer l; l*=first nonempty layer; M=descendant-minimal concept candidates; Known iff values(M) singleton

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

First nonempty layer decides before specificity

i removed if distinct candidate descends from i

Equal-value minimal candidates choose min(depth,index)

All visible A_l empty yields Unknown

No executable percentage surrogate established.

#### Visibility switch domain

The layer-chain equations, invisible-layer exclusion and invisible-seed preimage freedom require `leak_all_layers == false`. With the diagnostic switch true, prop instead scans all layer indices in reverse order, independent of the requested layer's parent chain; a nominally invisible layer can then decide. Index/acyclicity assumptions still apply. The executed fixtures keep the switch false; its compiled mutant deliberately sets true and fails the real-layer assertion.

## invariant

### Local observations

Mixed minimal values excludes Known; beaten list excludes candidates from shadowed layers

First nonempty layer decides before specificity

i removed if distinct candidate descends from i

Equal-value minimal candidates choose min(depth,index)

All visible A_l empty yields Unknown

#### Visibility switch domain

The layer-chain equations, invisible-layer exclusion and invisible-seed preimage freedom require `leak_all_layers == false`. With the diagnostic switch true, prop instead scans all layer indices in reverse order, independent of the requested layer's parent chain; a nominally invisible layer can then decide. Index/acyclicity assumptions still apply. The executed fixtures keep the switch false; its compiled mutant deliberately sets true and fails the real-layer assertion.

## cost

### Scoped cost

Initial BFS O(Va+Ea); collect/sort matching seeds O(Ac log Ac); per selected layer K ancestor maps O(sum reachable graphs) and O(K²) dominance comparisons. Stores those K maps and selected proof chains. Other layers scanned until first nonempty.

Symbolic accounting; no timing claim or whole-system speed bound.

## composition

### Parent join

property-resolution supplies the parent relation: A_l=visible ancestor assertions on layer l; l*=first nonempty layer; M=descendant-minimal concept candidates; Known iff values(M) singleton

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Mixed minimal values excludes Known; beaten list excludes candidates from shadowed layers

## reverse_reconstruction

Known preimage requires matching all minimal values and winner depth/index; invisible seed additions preserve answer

## layer_order

First nonempty layer decides before specificity

## dominance

i removed if distinct candidate descends from i

## winner_tie

Equal-value minimal candidates choose min(depth,index)

## unknown_fiber

All visible A_l empty yields Unknown


## Visibility switch domain

The layer-chain equations, invisible-layer exclusion and invisible-seed preimage freedom require `leak_all_layers == false`. With the diagnostic switch true, prop instead scans all layer indices in reverse order, independent of the requested layer's parent chain; a nominally invisible layer can then decide. Index/acyclicity assumptions still apply. The executed fixtures keep the switch false; its compiled mutant deliberately sets true and fails the real-layer assertion.

## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [19717,21981); shared parent AST. Excludes unrelated parent operations and added test module.
