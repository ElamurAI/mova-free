# algorithm-dijkstra

## descriptant

### dijkstra groupant

Original deterministic graph algorithm, with immutable adjacency input, explicit errors and bounded independent oracle tests. Source stays adjacent and authoritative; manifest hash and partial syntax projection bind the revision. No macro expansion or semantic AST is claimed.

#### Mathematical combiner

Take source/AST, forward relation R, typed family, invariant, cost and measurements as inputs. Restrict R by the invariant and compare resulting outputs against the independently implemented finite oracle. The complement uses the explicit one-call universe; the converse retains all compatible antecedents. Specialized models fix their own observations and interventions. Disagreement revises source or assumptions; artifact presence is not semantic proof.

#### Limits

External run artifacts preserve prospective prediction, tests and mutants. No theorem prover or general inverse executor was used. Parent crate wiring requires separate integration checks.

## approximant

### Forward approximant

For valid G,s with u64 nonnegative weights, d[v] is the exact minimum u128 path sum or None when unreachable. Source validation precedes global edge validation. Checked candidate addition yields DistanceOverflow if not representable; on supported <=64-bit usize platforms every simple path plus one edge fits u128.

#### Cost

O(V+E log(E+1)) time and O(V+E) auxiliary space for lazy heap entries. E counts edge multiplicity; validation includes unreachable components. Successful allocation and materialized inputs are assumed. No throughput benchmark is claimed.

## invariant

### Invariant

A nonstale popped distance is final. Strict improvements enter the heap; equal zero-weight alternatives do not. No wrapped arithmetic participates in ordering. A shortest path can be chosen simple.

Inputs remain unchanged. Independent finite oracles support the specification; arbitrary-size correspondence is unproved.

## catalog

# Typed obernant family

## behavior-complement

Type: behavior_complement. Subject/revision: mod.rs at manifest source hash.
Domain: all finite weighted directed adjacency lists with u64 weights and usize source on <=64-bit usize targets; outputs are tagged errors or optional u128 distance vectors.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: In U=domain inputs crossed with all declared return values for one call, C=U minus R where R is the forward specification. This excludes every wrong successful vector/order, incorrect error and wrong first-failure precedence. Exact for the specification; partial source-derived absent-behavior approximant for Rust. Allocation failure, panic and elapsed time are outside U.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent synchronous Bellman-Ford on all 19,767 graphs with 1..=3 vertices, each arc absent/weight 0/weight 2 and every source (59,214 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs at manifest source hash.
Domain: all finite weighted directed adjacency lists with u64 weights and usize source on <=64-bit usize targets; outputs are tagged errors or optional u128 distance vectors.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For every return y, Pre(y)={x in domain | R(x,y)} retains ALL compatible graphs and sources. Success fibers include arbitrary unreachable structure and alternatives consistent with the equations. Error fibers include every input satisfying the precise first-failure rule; no unique recovery is assumed. This is a relational specification, not an implemented inverse executor.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent synchronous Bellman-Ford on all 19,767 graphs with 1..=3 vertices, each arc absent/weight 0/weight 2 and every source (59,214 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## zero-fiber

Type: zero_distance_preimage. Subject/revision: mod.rs at manifest source hash.
Domain: all finite weighted directed adjacency lists with u64 weights and usize source on <=64-bit usize targets; outputs are tagged errors or optional u128 distance vectors.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For fixed s,Z, all valid weighted graphs whose zero-weight reachability from s equals Z produce exactly Z as their zero-distance vertices. Positive edges cannot enter a zero-cost path. Oracle: zero-weight cycles and exhaustive Bellman-Ford.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent synchronous Bellman-Ford on all 19,767 graphs with 1..=3 vertices, each arc absent/weight 0/weight 2 and every source (59,214 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## cost-fiber

Type: shortest_cost_preimage. Subject/revision: mod.rs at manifest source hash.
Domain: all finite weighted directed adjacency lists with u64 weights and usize source on <=64-bit usize targets; outputs are tagged errors or optional u128 distance vectors.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For fixed s,t,c, retain all graphs with an s-to-t path costing c and no s-to-t path costing less than c. All path alternatives and vertex labels remain free; no unique graph recovery is implied. For c=None retain exactly graphs without any path. Oracle: inspect Bellman-Ford target coordinate.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent synchronous Bellman-Ford on all 19,767 graphs with 1..=3 vertices, each arc absent/weight 0/weight 2 and every source (59,214 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## weight-scaling

Type: counterfactual_weight_scaling. Subject/revision: mod.rs at manifest source hash.
Domain: all finite weighted directed adjacency lists with u64 weights and usize source on <=64-bit usize targets; outputs are tagged errors or optional u128 distance vectors.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: Holding topology and source fixed, multiply all weights by integer k>=1 provided every new edge fits u64. All finite distances multiply by k, reachability is unchanged. k=0 is deliberately excluded from this positive-scale model. Oracle evidence: weight_scaling_and_edge_splitting_preserve_path_costs checks factor seven on 24 generated graphs with safe weights below 1,000,000, for every original source. General factors remain modeled but not separately exercised.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent synchronous Bellman-Ford on all 19,767 graphs with 1..=3 vertices, each arc absent/weight 0/weight 2 and every source (59,214 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## expanded-test-evidence

Current source subject is mod.rs; test implementation and its independent source hash are in tests.rs.groupant. Two original tests plus 36 generated graphs with 4..16 vertices and full-width u64/zero/MAX weights, 24 scaling/splitting graphs with 4..9 vertices, all sources, and a zero-cycle/stale-entry chain with sums beyond two u64::MAX. Seeds 0xd15120261029 and 0xd15220261029; diagnostics include case/source. u128::from(weight) changed to u128::from(weight as u32); only new wider_seeded_full_width_weights_match_bellman_ford fails exit 101. The mathematical production relations above are unchanged; the additional observations improve their finite evidence only.

## tests

### Checks

Two original tests plus 36 generated graphs with 4..16 vertices and full-width u64/zero/MAX weights, 24 scaling/splitting graphs with 4..9 vertices, all sources, and a zero-cycle/stale-entry chain with sums beyond two u64::MAX. Seeds 0xd15120261029 and 0xd15220261029; diagnostics include case/source.

Synchronous Bellman-Ford copies the previous distance vector for each of V-1 rounds and relaxes from that previous vector only. Its path-length recurrence shares neither heap ordering nor stale-entry handling with production; sums are u128. An explicit wide-sum fixture adds hand-computed expected values.

Weights are <1,000,000 before multiplication by seven, so u64 products are safe; finite distances scale sevenfold. Separately replace each edge with two segments through its own fresh vertex, weights floor(w/2) and w-floor(w/2). Restrict results to original vertices and compare against the original independent oracle.

New compiled source mutation: u128::from(weight) changed to u128::from(weight as u32); only new wider_seeded_full_width_weights_match_bellman_ford fails exit 101. Exactly one filtered new test failed, with exit101, not a timeout. External probe compared the full production prefix before cfg(test) with main 600f1d961 byte-for-byte. Cargo crate checks passed39 tests; strict Clippy and formatting passed. No production algorithm was changed.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Prospective prediction was bounded oracle agreement, deterministic output, input preservation and a rejected source mutation. Two tests passed twice for this file. independent synchronous Bellman-Ford on all 19,767 graphs with 1..=3 vertices, each arc absent/weight 0/weight 2 and every source (59,214 source queries). Normalized logs agree excluding elapsed timing. The finite source mutant fails with exit 101. A preliminary Dijkstra greater-than relaxation mutant timed out on cycles; the reported negative control instead adds one to edge weights and yields a finite assertion failure. Retain this distinction between liveness and value evidence. No speed claim is made.

#### Expanded test epoch 2026-10-29

Original inline tests now reside in adjacent tests.rs, registered separately as algorithm-dijkstra-tests. The production prefix is byte-identical to main 600f1d961; only test-module wiring changes. Two original tests plus 36 generated graphs with 4..16 vertices and full-width u64/zero/MAX weights, 24 scaling/splitting graphs with 4..9 vertices, all sources, and a zero-cycle/stale-entry chain with sums beyond two u64::MAX. Seeds 0xd15120261029 and 0xd15220261029; diagnostics include case/source. Five tests passed twice with normalized-identical output; strict Clippy and formatting passed. u128::from(weight) changed to u128::from(weight as u32); only new wider_seeded_full_width_weights_match_bellman_ford fails exit 101. This newer evidence extends, rather than erases, the earlier small-domain observations. Tests and production syntax have separate source hashes; production AST does not inline the test module.

### 2026-10-29 · tests

2026-10-29 expanded test epoch: rustc --edition 2024 --test algorithms/src/dijkstra/mod.rs runs adjacent tests.rs. Five tests passed twice with normalized identical results. The original two tests and their assertions remain; earlier source mutation observations are retained in model.md#descriptant.
