# algorithm-prufer-tests

## descriptant

### algorithm-prufer-tests: epoch memory

On n>=2, degree(v)=1+occ(v) initially in decoding; each step consumes a leaf and one occurrence while maintaining the live-tree degree sum. Encoding validates n-1 distinct loopless edges and acyclicity, which implies connectivity for n>0. Codes have length max(n-2,0). n=0/1 encode/decode empty edges.

O(n log(n+1)) time for ordered duplicate checking, heap operations and output sort; O(n) storage. n<=1000000 checked before allocation. Encoding adjacency scans total O(n): each removed vertex scans its own adjacency once. Rejected wrong lengths avoid unbounded edge processing.

Retained observations: All words for n=0..6 compare both directions with independent adjacency-matrix leaf elimination, connectivity, edge uniqueness, canonical order and degree laws; explicit minimum-leaf example and invalid tree/code boundaries. The compiled production mutation fails canonical_minimum_leaf; exact original bytes precede the four positive runs. No timing comparison or speed improvement was attempted. Models describe intended relations and retain ambiguous reverse fibers; source identity and structural validation do not prove semantics.

Pruefer (1918), Neuer Beweis eines Satzes ueber Permutationen. Original heap implementation and original fixtures.

## model

# algorithm-prufer-tests

## Forward
Execute three named deterministic tests over the actual public API. All words for n=0..6 compare both directions with independent adjacency-matrix leaf elimination, connectivity, edge uniqueness, canonical order and degree laws; explicit minimum-leaf example and invalid tree/code boundaries. Acceptance is conjunction of all named assertions, not a proof over unenumerated inputs.

## Invariant
Independent oracles do not call production graph algorithms. Fixtures are deterministic and do not modify process-global state; positive runs use exactly restored production source.

## Cost
Bounded exhaustive enumeration only: subset/closure MST oracle, short word tree checks, or subset clique oracle. These test costs are exponential in their tiny fixed domains, not production performance measurements.

## behavior_complement
Universe U is candidate implementations x named outcomes. A wrong implementation violating a sampled assertion cannot yield all-pass. The compiled production mutant targeting canonical_minimum_leaf is a witnessed exclusion. Unsampled defects can still pass; finite coverage is explicit.

## reverse_reconstruction
The all-pass observation admits every implementation agreeing on sampled assertions, including ones wrong elsewhere. A failing assertion constrains the mismatching observation but need not uniquely identify a defect. Named identities preserve more information than only a count.

## oracle_independence
Independent witness: All words for n=0..6 compare both directions with independent adjacency-matrix leaf elimination, connectivity, edge uniqueness, canonical order and degree laws; explicit minimum-leaf example and invalid tree/code boundaries. The oracle uses a distinct representation or exhaustive candidate enumeration; self-roundtrip alone is insufficient.

## Provenance
Pruefer (1918), Neuer Beweis eines Satzes ueber Permutationen. Original heap implementation and original fixtures.

## Limitations
Partial syntax is source-bound, not a semantic AST; macros remain opaque. Finite exhaustive small domains and mutation witnesses are not formal proof. No runtime speed claim, approximation cap, automatic substitution or unbounded resource guarantee.

## independent_leaf_schedule
reference_encode reconstructs a Boolean adjacency matrix from decoded edges, scans labels in increasing order for a row with one true neighbor, emits that neighbor and removes the edge. It shares neither production heap nor production degree bookkeeping. Comparing its output to each original short word discriminates a decoder that preserves roundtrips only by sharing an incorrect leaf order with encode. canonical_minimum_leaf additionally binds a literal tree to [4,4,4,3]; the compiled production code.push(leaf) mutation fails this named assertion.

## degree_connectivity_discrimination
assert_tree independently checks n-1 edges, u<v<n, strict lexicographic edge order, reachability from 0, and degree(v)=1+occ(v) for n>1 (zero degree for n=1). Degree counts alone do not establish a tree: a triangle plus a disjoint edge on five vertices has four edges and degrees [2,2,2,1,1], yet is disconnected. Reachability rejects this candidate even though that degree vector matches a valid word's occurrence law. Every word through n=6 supplies 1443 checks across n=0..6; the empty cases use their explicit conventions.

## evidence

### Native checks

Three named tests pass twice debug and twice release with identical normalized names/outcomes. All words for n=0..6 compare both directions with independent adjacency-matrix leaf elimination, connectivity, edge uniqueness, canonical order and degree laws; explicit minimum-leaf example and invalid tree/code boundaries.

## History

### 2026-10-30 · descriptant

2026-10-30: prospective model selected exact capped algorithms with deterministic canonical outputs. For a labeled undirected tree, encode repeatedly removes the smallest live leaf and emits its neighbor until two remain. Decode counts occurrences, sets degree(v)=1+occ(v), repeatedly joins the smallest degree-one vertex to the next code symbol, then joins final leaves. Sort normalized output edges. Empty codes are disambiguated by explicit n=0,1,2.

Model refinement, 2026-10-30: five typed views now distinguish the two baseline relations and three concrete specializations. New validation, observation and oracle constraints retain which claims have native witnesses and which remain source-derived. Rust bytes, partial syntax and native evidence are unchanged.

### 2026-10-30 · evidence

Compiled production mutation for prufer fails the named assertion canonical_minimum_leaf; production bytes restored before positive repetitions. Strict all-target algorithms Clippy and narrow rustfmt pass. Six partial syntax projections generated twice identically; full scaffold groupants checked twice. Raw logs live in $MOVA_DATA/runs/octopus-wave16-graphs-2026-10-30; evidence.json binds hashes. No live corpus, parser, teacher, GPU or full-school execution.
