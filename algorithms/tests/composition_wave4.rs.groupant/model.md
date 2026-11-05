# algorithm-wave4-compositions

## descriptant

### Cross-algorithm evidence boundary

The new file keeps seven client joins separate from the existing library-root tests. Its combiner takes public APIs, their source-bound leaf relations, adapter identities, this finite harness and the observed results. The parent algorithm-library groupant binds every participating source transitively.

Predictions precede execution in model.md. Independent leaf oracles remain necessary: two implementations can agree on the same wrong result. The hull counterexample and unequal cycle/empty-suffix domains deliberately preserve failures of tempting generalizations. Future changes must update source identity, syntax and evidence together. No automatic algorithm substitution or approximation budget is introduced.

## model

# Fourth-wave algorithm compositions

## Forward

Seven independent test-only client joins use exact library APIs. They are not one runtime pipeline. Prospective prediction: each stated equation holds in the finite fixtures; a changed search adapter must be rejected. Execution evidence is recorded separately.

## Invariant

Preserve vertex identities, byte offsets, original item/point indices and arithmetic widths across adapters. Positive interval lengths, validated endpoints and successful allocation are assumptions. Geometry uses i32 coordinates with i128 cross products and u128 squared distances. Hashes bind source versions, not semantic correctness.

## Cost

Each fixture pays reached child costs plus adapters and independent checks. For one graph/source, binary-distance composition costs O(V+E+E log(E+1)); the signed-DAG/Bellman-Ford join costs O(V(V+E)+V log(V+1)). Repeating sources and enumerating graphs multiply these costs. For text length n, K patterns of total length M and maximum length L, the search fixture includes automaton construction, O(n+z) query at fixed byte alphabet, O(Kn+M) KMP work, O(n log²(n+1)) suffix construction, O(KnL) direct prefix scans and observation sorting. The sequence fixture costs O(1+nm+n log(n+1)+m log(m+1)), including linear-membership intersection work bounded by nm. Scheduling adds O(n log(n+1)) plus O((n+1)(C+1)) knapsack work and original-index checks. Geometry runs a constant 49 translations with O(n log²(n+1)) child work each. For a fixed n and k prime-power factors, the arithmetic fixture includes O(sqrt(n)) factorization, O(k² log U) pairwise gcd checks and n+1 CRT solves, each O(k log U), plus a shared sieve. Enumeration costs belong to this harness, not standalone APIs. No school, GPU or application speedup follows.

## behavior_complement

Type: behavior_complement. Let U be the disjoint union of seven tagged input/observation spaces satisfying the section-specific premises. R is the corresponding forward equation or specified inequality. U minus R excludes disagreement under the declared adapters, wrong original-index witnesses, and treating hull-only nearest pairs as a general replacement. Rust tests sample U and do not enumerate arbitrary client behavior, resource failure or trait violations.

## reverse_reconstruction

Type: reverse_reconstruction. Pre(y) retains all inputs, intermediate objects and adapter identities satisfying the observed tag's equations. Distances admit multiple graphs; matching offsets omit unmatched bytes; sequence lengths omit witnesses; optimal weight omits rejected intervals; a hull omits interior points; residues specify a congruence class. None is a global unique inverse. Factorization with exact prime/exponent pairs reconstructs its positive integer; CRT recovers its canonical residue only after the modulus is fixed.

## binary_distance_embedding

Type: weight_domain_embedding. For valid directed G with weights in {0,1}, widen weights to u64 and optional zero-one distances to u128. The result equals Dijkstra on the same graph. Zero cycles and repeated edges are allowed. A distance vector does not recover edges or their unique weights. Finite fixtures enumerate all simple directed graphs of one through three vertices with a fixed endpoint-dependent binary weighting and all sources. Costs sum deque traversal, adapter and heap traversal; no speed comparison is inferred.

## acyclic_signed_distance

Type: acyclic_restriction_equivalence. On an endpoint-valid DAG with i64 weights, DAG relaxation equals Bellman-Ford under identical source and i128 arithmetic. Acyclicity is global for the DAG API; Bellman-Ford's error domain differs for unreachable cycles, so unrestricted API equivalence is excluded. Fixtures enumerate forward-edge DAGs of one through four vertices with negative and positive weights. Reverse fibers retain all compatible DAGs, not a uniquely selected topological order.

## multi_index_search

Type: search_observation_join. Aho-Corasick occurrences equal the union of KMP results for each pattern ID, sorted by (end,pattern). Duplicate pattern identities remain distinct; empty patterns match every boundary. For nonempty patterns, suffix-array positions whose suffix starts with the pattern give the same starts after numerical sorting. The empty suffix is absent from suffix_array, so its naive empty-pattern join is explicitly excluded. This projection loses failure links, suffix ranks outside the match interval and unmatched bytes. Fixtures use binary texts of length at most six and a fixed set including duplicates and empty patterns.

## sequence_edit_bounds

Type: subsequence_edit_sandwich. For finite Eq sequences A,B, let L=LCS(A,B), D=Levenshtein(A,B) with unit substitutions. Then max(|A|,|B|)-L <= D <= |A|+|B|-2L. Equality to the upper bound is not asserted because substitutions can be cheaper than delete+insert. Symmetry and sorted-distinct intersection provide additional bounded observations. Reverse lengths preserve many input pairs and many alignments. Tests enumerate binary sequences through length four; cost includes both quadratic DPs.

## interval_capacity_restrictions

Type: scheduling_capacity_embedding. If all positive-length intervals coincide, at most one item is selectable and the optimum equals unit-weight knapsack with capacity one. If intervals are pairwise disjoint in chronological order, every item is compatible and the optimum equals unit-weight knapsack with capacity item count. Compare values, original-index validity, temporal compatibility and summed witness weights, not tie-identical choices. Arbitrary overlap graphs are outside this capacity reduction. Reverse weight observations omit unused items and tie alternatives.

## hull_nearest_noncommutation

Type: lossy_geometric_noncommutation. Hull projection preserves extreme support but can delete the nearest pair. A square with two nearby interior points has squared nearest distance one, while its hull vertices have minimum squared distance one hundred. Thus nearest(hull(P)) need not equal nearest(P). Bounded common translation preserves both hull vertices up to that translation and the full-input nearest index pair/distance. Hull output has many interior-point preimages. The equation assumes no coordinate overflow; it does not transfer nearest-pair certificates from the hull to the whole cloud.

## prime_power_crt_roundtrip

Type: factor_residue_reconstruction. For 1<=n<=256, prime factorization gives powers q_i with product n and pairwise coprime bases, checked against the sieve. CRT on residues x mod q_i returns x mod n with modulus n; n=1 uses the empty system 0 mod 1. Full prime/exponent output reconstructs n, whereas residues recover x only modulo n. Arbitrary noncoprime/incompatible systems and intermediate modulus overflow remain leaf obligations. Enumeration and reconstruction overhead are distinct from standalone factorization or CRT costs.

## evidence

### Executed evidence

All seven public-API composition tests pass in debug and release. The complete library passes 214 unit tests and one doctest in both profiles. Strict all-target Clippy and crate formatting pass. Two subsequent single-threaded release-binary runs have identical named outcomes for all 214 unit tests and all seven integration tests; elapsed times are recorded separately, not compared for determinism.

The external compiled adapter control changes only the Aho observation's start offset to start+1. Exactly multi_pattern_and_suffix_search_preserve_byte_occurrences runs and fails its equality assertion with exit 101. Production sources are not mutated. Each of the ten new implementations additionally has a compiled production mutant rejected by an independent leaf test; see each leaf's evidence.

## History

### 2026-10-30 · evidence

Heading `### Executed evidence, 2026-10-30` stays in evidence as `### Executed evidence`.

The library boundary and this harness have twice-identical partial syntax projections. Parent source/child controls, full registry validation and changed-source registration are integration gates recorded in the parent evidence. Raw logs and process-inclusive timings: $MOVA_DATA/runs/octopus-algorithms-wave4-2026-10-30/. No GPU, teacher, school course, benchmark comparison or universal correctness proof is inferred.
