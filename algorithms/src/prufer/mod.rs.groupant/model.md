# algorithm-prufer

## descriptant

### algorithm-prufer: epoch memory

On n>=2, degree(v)=1+occ(v) initially in decoding; each step consumes a leaf and one occurrence while maintaining the live-tree degree sum. Encoding validates n-1 distinct loopless edges and acyclicity, which implies connectivity for n>0. Codes have length max(n-2,0). n=0/1 encode/decode empty edges.

O(n log(n+1)) time for ordered duplicate checking, heap operations and output sort; O(n) storage. n<=1000000 checked before allocation. Encoding adjacency scans total O(n): each removed vertex scans its own adjacency once. Rejected wrong lengths avoid unbounded edge processing.

Retained observations: All words for n=0..6 compare both directions with independent adjacency-matrix leaf elimination, connectivity, edge uniqueness, canonical order and degree laws; explicit minimum-leaf example and invalid tree/code boundaries. The compiled production mutation fails canonical_minimum_leaf; exact original bytes precede the four positive runs. No timing comparison or speed improvement was attempted. Models describe intended relations and retain ambiguous reverse fibers; source identity and structural validation do not prove semantics.

Pruefer (1918), Neuer Beweis eines Satzes ueber Permutationen. Original heap implementation and original fixtures.

## model

# algorithm-prufer

## Forward
For a labeled undirected tree, encode repeatedly removes the smallest live leaf and emits its neighbor until two remain. Decode counts occurrences, sets degree(v)=1+occ(v), repeatedly joins the smallest degree-one vertex to the next code symbol, then joins final leaves. Sort normalized output edges. Empty codes are disambiguated by explicit n=0,1,2.

## Invariant
On n>=2, degree(v)=1+occ(v) initially in decoding; each step consumes a leaf and one occurrence while maintaining the live-tree degree sum. Encoding validates n-1 distinct loopless edges and acyclicity, which implies connectivity for n>0. Codes have length max(n-2,0). n=0/1 encode/decode empty edges.

## Cost
O(n log(n+1)) time for ordered duplicate checking, heap operations and output sort; O(n) storage. n<=1000000 checked before allocation. Encoding adjacency scans total O(n): each removed vertex scans its own adjacency once. Rejected wrong lengths avoid unbounded edge processing.

## behavior_complement
U contains labeled edge lists or words, explicit n, and candidate Result values. Wrong lengths/labels, loops, reversed duplicates, or cyclic n-1 edge lists cannot yield Ok. For valid trees, code from a nonminimum current leaf is excluded; decode outputs with wrong degrees, noncanonical edge order, cycles or missing connectivity are excluded.

## reverse_reconstruction
For n>=2 fixed n, the valid code identifies one undirected labeled tree; encoding forgets edge order and endpoint orientation, so its concrete edge-list fiber contains all permutations and orientations of that tree. Decode with fixed n has one compatible word. Without n, [] also represents the distinct n=0,1,2 conventions; omitting n loses that distinction.

## degree_fiber
For n>=2, fixing just decoded degree vector d admits every word of length n-2 with exactly d(v)-1 occurrences of v; occurrence order is lost by the degree observation. Different such words may encode different trees. Full code retains this order and restores the unique labeled edge set.

## Provenance
Pruefer (1918), Neuer Beweis eines Satzes ueber Permutationen. Original heap implementation and original fixtures.

## Limitations
Partial syntax is source-bound, not a semantic AST; macros remain opaque. Finite exhaustive small domains and mutation witnesses are not formal proof. No runtime speed claim, approximation cap, automatic substitution or unbounded resource guarantee.

## tree_validation_partition
encode first rejects n>MAX_VERTICES, then edges.len()!=n.saturating_sub(1), then processes edges in order: u label, v label, self-loop, normalized duplicate, DSU cycle. A reversed repeat reports RepeatedEdge before the cycle check; a first cycle reports NotTree before inspecting later edges. With n-1 distinct loopless edges, acyclicity implies connectedness for n>0. decode checks cap then length then labels. Thus a wrong-length word with an invalid symbol yields WrongLength. n=0/1 accept only empty edge lists and words.

## canonical_tree_identity
For fixed n>=2, decode(encode(n,E)) equals the sorted set of normalized edges {(min(u,v),max(u,v))}; it need not equal E as an ordered oriented list. encode(decode(n,C))=C for every valid word. At each elimination the heap chooses the smallest live degree-one label, so the star/path fixture [(4,0),(4,1),(4,2),(4,3),(3,5)] yields [4,4,4,3]. The degree fiber alone loses this temporal order: for n=4 codes [0,1] and [1,0] have the same degrees [2,2,1,1] but different decoded trees.

## evidence

### Native checks

Three named tests pass twice debug and twice release with identical normalized names/outcomes. All words for n=0..6 compare both directions with independent adjacency-matrix leaf elimination, connectivity, edge uniqueness, canonical order and degree laws; explicit minimum-leaf example and invalid tree/code boundaries.

## History

### 2026-10-30 · descriptant

2026-10-30: prospective model selected exact capped algorithms with deterministic canonical outputs. For a labeled undirected tree, encode repeatedly removes the smallest live leaf and emits its neighbor until two remain. Decode counts occurrences, sets degree(v)=1+occ(v), repeatedly joins the smallest degree-one vertex to the next code symbol, then joins final leaves. Sort normalized output edges. Empty codes are disambiguated by explicit n=0,1,2.

Model refinement, 2026-10-30: five typed views now distinguish the two baseline relations and three concrete specializations. New validation, observation and oracle constraints retain which claims have native witnesses and which remain source-derived. Rust bytes, partial syntax and native evidence are unchanged.

### 2026-10-30 · evidence

Compiled production mutation for prufer fails the named assertion canonical_minimum_leaf; production bytes restored before positive repetitions. Strict all-target algorithms Clippy and narrow rustfmt pass. Six partial syntax projections generated twice identically; full scaffold groupants checked twice. Raw logs live in $MOVA_DATA/runs/octopus-wave16-graphs-2026-10-30; evidence.json binds hashes. No live corpus, parser, teacher, GPU or full-school execution.
