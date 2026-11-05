# algorithm-lowest-common-ancestor

## descriptant

### lowest_common_ancestor mod.rs

Original std-only implementation and adjacent bounded checks. Classical concept: binary lifting/doubling of parent functions. No source copied.

The prospective artifact preceded implementation. The runtime checks refine this mathematical description; no timing claim, universal proof, macro expansion or semantic AST is supplied. Executable approximation is intentionally absent: these are exact APIs, with no justified lossy metric or substitution budget.

## model

# lowest_common_ancestor mod.rs model

## Forward

Validate parent bounds in vertex order; traverse parent chains from increasing starts with unvisited/gray/black states. Gray encounter reports Cycle at that vertex. Reverse each completed path to assign root and depth; build up[k][v]=up[k-1][up[k-1][v]] with absorbing roots. Query validates a then b, returns None across roots, otherwise lifts deeper endpoint by depth difference and lifts both from largest power down before returning shared parent.

## Invariant

Every black vertex has its actual root and edge distance to it. Gray vertices form exactly the current unfinished chain; gray re-entry is a directed parent cycle. Each doubling row is the stated power ancestor, roots absorbing. During query both endpoints remain ancestors of original endpoints and retain their common ancestors; equalization and simultaneous jumps preserve the desired deepest common vertex.

## Cost

O(n log(n+1)) preprocessing time and space for n>0, O(1) for empty construction. Each parent enters a path once; doubling has floor(log2 n)+1 rows. Query O(log(n+1)) time and O(1) extra space. Iterative construction has no depth-dependent call stack.

## nonbehavior

Type: behavior_complement. Let X be finite parent arrays or constructed forests with query pairs, Y the corresponding typed Result observations, and U=X times Y under sufficient allocation. R follows Forward validation and query rules. U minus R includes acceptance of cycles/out-of-range parents, wrong first errors, nondeep common ancestors and Some results across trees. Source-derived relation; finite tests do not prove all behavior.

## compatible_inputs

Type: reverse_reconstruction. For observed query result Some(c), compatible validated parent forests have c on both ancestor chains and no proper descendant of c common to both. None means distinct roots. Error observations constrain first invalid index or deterministic cycle entry. These fibers generally contain many forests; a result never uniquely reconstructs parents. Construction internals are not exposed.

## ancestor_intersection

Type: order_characterization. For a rooted forest define ancestor order including reflexivity. LCA(a,b) is the deepest member of Anc(a) intersection Anc(b), or None when empty. Tests use direct ancestor lists independently of doubling, enumerating every parent function for n=1..5 and rejecting cyclic functions.

## symmetry

Type: commutativity. On valid endpoints query(a,b)=query(b,a); query(a,a)=Some(a). Invalid endpoint error precedence is left-to-right and is not claimed symmetric. All valid finite fixtures compare swapped queries and long-chain fixtures check reflexivity.

## absorption

Type: ancestor_absorption. If query(a,b)=Some(c), then query(a,c)=query(c,b)=Some(c). This constrains c to the common ancestor chain; alone it does not guarantee deepest selection. Combined with the independent oracle it is checked for every pair in every validated n<=5 forest.

## relabeling

Type: equivariance. For bijection p of vertex labels, set parent'[p(v)]=p(parent[v]) with None preserved. Then successful query results transport by p. Cycle witness labels/error precedence are not claimed equivariant. The five-vertex fixture checks all 25 query pairs under a nontrivial permutation.

## leaf_extension

Type: conservative_extension. Add a new vertex whose parent is an existing vertex, without changing old parent entries. Every query between old vertices is unchanged, because old ancestor chains are unchanged. The fixture appends a leaf under a depth-two vertex and compares all old pairs.

## query_fiber

Type: constructive_preimage. For distinct a,b,c, any rooted forest with a and b direct children of c yields query(a,b)=Some(c). Additional components and descendants not lying on these chains preserve this observation, providing distinct antecedents. Tests compare two forests with result zero and exclude a chain whose deepest shared ancestor is one.

## evidence

### Evidence

For distinct a,b,c, any rooted forest with a and b direct children of c yields query(a,b)=Some(c). Additional components and descendants not lying on these chains preserve this observation, providing distinct antecedents. Tests compare two forests with result zero and exclude a chain whose deepest shared ancestor is one.

#### Measured outcome

All four focused tests passed in each of two debug and two release runs; named assertion transcripts were byte-identical after timing removal. Strict all-target Clippy with -D warnings and cargo fmt --check passed. Each source syntax projection was generated twice and was byte-identical. Each isolated production mutant compiled successfully and failed the named equality assertion.

Artifact SHA-256 bindings (external run label above):

## History

### 2026-10-30 · descriptant

2026-10-30 prospective epoch: O(n log(n+1)) preprocessing time and space for n>0, O(1) for empty construction. Each parent enters a path once; doubling has floor(log2 n)+1 rows. Query O(log(n+1)) time and O(1) extra space. Iterative construction has no depth-dependent call stack.

### 2026-10-30 · evidence

2026-10-30: planned and then measured in the dedicated graph-optimization run. See external run label octopus-wave5-graph-optimization-2026-10-30: prospective.md, debug/release twice normalized, strict-clippy.log, format.log, syntax twice and compiled production mutant logs. Four tests per algorithm. Bounded results are not universal verification.

- lowest_common_ancestor-debug-1.log: 5c945b9297e9100c9b43377b08a16b22ceafa2e43b0dccdafab30542c2bf886c

- lowest_common_ancestor-debug-2.log: bb6effb2906730a8eff65568e1801c317771e873a28e942bf7b00b93dd78d8b9

- lowest_common_ancestor-release-1.log: 3e74e3fa97adc7a888143ed626bbf084981dc0cbbdb8ca7343268a517055b102

- lowest_common_ancestor-release-2.log: 9e4f7e8de01307112b4bfa0ef489e4712d9ecfc0492e8c542584e10e4dbbb04b

- lowest_common_ancestor-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- lowest_common_ancestor-mutant-rejected.log: e1f76b95dc1ddce023f9662e11486200f1d15864557fd1e9853fcd879b0f255d

- strict-clippy.log: fba0904fdec91250d0aafb0674967baaad449a27cd510117ea1fb9defea286f1

- format.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- prospective.md: c041f70a6de4c683ebc42555609c330a5d55d45d83086f796a86ba4562c996c0

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
