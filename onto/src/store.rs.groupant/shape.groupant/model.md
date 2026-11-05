# onto-store-shape

## descriptant

### onto-store-shape

For each CSR off require len=rows+1,start0,end=total,monotone; validate UTF8/refs

## approximant

### Scoped forward

For each CSR off require len=rows+1,start0,end=total,monotone; validate UTF8/refs

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

String offsets must char boundaries

mem<nw,senses<n,edge.to<n,LU refs bounded

Keys/lexfiles/inverse CSR/cycles not checked

Scalar lengths→offsets→UTF8→word/concept refs→edge/LU refs

No executable percentage surrogate established.

## invariant

### Local observations

Invalid offset/reference excludes Ok; valid shape does not exclude inverse-edge mismatch

String offsets must char boundaries

mem<nw,senses<n,edge.to<n,LU refs bounded

Keys/lexfiles/inverse CSR/cycles not checked

Scalar lengths→offsets→UTF8→word/concept refs→edge/LU refs

## cost

### Scoped cost

O(offsets+references+string boundary checks), no graph reachability traversal.

Restriction to this operator's traversed inputs; parent total is not charged to every fragment. No standalone measurement.

## composition

### Parent join

shape supplies the parent relation: For each CSR off require len=rows+1,start0,end=total,monotone; validate UTF8/refs

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Invalid offset/reference excludes Ok; valid shape does not exclude inverse-edge mismatch

## reverse_reconstruction

Ok preimage entire set of shape-consistent graphs; not unique/sorted/DAG ontology

## offset_domain

String offsets must char boundaries

## reference_domain

mem<nw,senses<n,edge.to<n,LU refs bounded

## semantic_gap

Keys/lexfiles/inverse CSR/cycles not checked

## failure_order

Scalar lengths→offsets→UTF8→word/concept refs→edge/LU refs


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [988,8930); shared parent AST. Excludes unrelated parent operations and added test module.
