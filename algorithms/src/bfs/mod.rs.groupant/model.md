# algorithm-bfs

## descriptant

### bfs groupant

Original deterministic graph algorithm, with immutable adjacency input, explicit errors and bounded independent oracle tests. Source stays adjacent and authoritative; manifest hash and partial syntax projection bind the revision. No macro expansion or semantic AST is claimed.

#### Mathematical combiner

Take source/AST, forward relation R, typed family, invariant, cost and measurements as inputs. Restrict R by the invariant and compare resulting outputs against the independently implemented finite oracle. The complement uses the explicit one-call universe; the converse retains all compatible antecedents. Specialized models fix their own observations and interventions. Disagreement revises source or assumptions; artifact presence is not semantic proof.

#### Limits

External run artifacts preserve prospective prediction, tests and mutants. No theorem prover or general inverse executor was used. Parent crate wiring requires separate integration checks.

#### Review correction

Independent model review found that the distance-layer fiber omitted uniqueness of the zero layer. The counterexample d=[0,0] for an edgeless two-vertex graph passed those incomplete constraints. The corrected relation requires d[v]=0 iff v=s, a valid source and a vector of length V; predecessor chains then anchor at the unique source. Reordering invariance remains source reasoning, not a separately executed metamorphic test.

## approximant

### Forward approximant

For valid G,s, d[v]=min number of directed edges in any s-to-v path; d[v]=None iff no such path. d[s]=0. First source error precedes first invalid edge in adjacency order.

#### Cost

O(V+E) time and O(V) auxiliary space. E counts edge multiplicity; validation includes unreachable components. Successful allocation and materialized inputs are assumed. No throughput benchmark is claimed.

## invariant

### Invariant

The queue contains discovered vertices in nondecreasing distance; each vertex enters once. Assigned distances are final minimum edge counts. Every assigned distance is at most V-1.

Inputs remain unchanged. Independent finite oracles support the specification; arbitrary-size correspondence is unproved.

## catalog

# Typed obernant family

## behavior-complement

Type: behavior_complement. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops and a usize source; outputs are tagged errors or vectors of optional usize distances.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: In U=domain inputs crossed with all declared return values for one call, C=U minus R where R is the forward specification. This excludes every wrong successful vector/order, incorrect error and wrong first-failure precedence. Exact for the specification; partial source-derived absent-behavior approximant for Rust. Allocation failure, panic and elapsed time are outside U.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: Floyd-Warshall on all 530 graphs with 1..=3 vertices and every source (1,570 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops and a usize source; outputs are tagged errors or vectors of optional usize distances.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For every return y, Pre(y)={x in domain | R(x,y)} retains ALL compatible graphs and sources. Success fibers include arbitrary unreachable structure and alternatives consistent with the equations. Error fibers include every input satisfying the precise first-failure rule; no unique recovery is assumed. This is a relational specification, not an implemented inverse executor.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: Floyd-Warshall on all 530 graphs with 1..=3 vertices and every source (1,570 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## reachability-fiber

Type: reachability_preimage. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops and a usize source; outputs are tagged errors or vectors of optional usize distances.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For fixed s and reachable set S, retain every valid G with Reach_G(s)=S. This permits arbitrary edges inside S and outside S, but excludes any edge from S to its complement and requires every vertex in S be reachable from s. Oracle: project each Floyd result to finite entries.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: Floyd-Warshall on all 530 graphs with 1..=3 vertices and every source (1,570 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## layer-constraints

Type: distance_layer_preimage. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops and a usize source; outputs are tagged errors or vectors of optional usize distances.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For fixed valid source s and distance vector d of length V with d[v]=0 iff v=s, compatibility requires that each finite nonzero layer vertex has a predecessor in the immediately preceding layer, and each edge u->v with finite d[u] has finite d[v]<=d[u]+1. These conditions characterize all compatible valid graphs, including arbitrary edges whose tail is unreachable. Oracle: exhaustive shortest-path distances.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: Floyd-Warshall on all 530 graphs with 1..=3 vertices and every source (1,570 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## adjacency-permutation

Type: representation_equivalence. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops and a usize source; outputs are tagged errors or vectors of optional usize distances.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: Holding the directed edge set and source fixed, permuting adjacency order and changing positive edge multiplicities preserves successful distances. Validation error order can change on malformed graphs and is excluded. Evidence: original duplicate-edge fixture and the new adjacency_permutation_and_duplicate_edges_preserve_distances test, checking 32 generated original/transformed graphs against Floyd for all sources.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: Floyd-Warshall on all 530 graphs with 1..=3 vertices and every source (1,570 source queries); boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## expanded-test-evidence

Current source subject is mod.rs; test implementation and its independent source hash are in tests.rs.groupant. Two original tests plus 40 generated graphs with 4..20 vertices, 32 duplicate/reordered graphs with 5..16 vertices, all sources, a 2048-vertex chain and a competing-frontier fixture. Seeds 0xb15120261029 and 0xb15220261029; generated diagnostics include case/source. queue.pop_front() changed to queue.pop_back(); only new long_chain_and_competing_frontier_paths fails exit 101. The mathematical production relations above are unchanged; the additional observations improve their finite evidence only.

## tests

### Checks

Two original tests plus 40 generated graphs with 4..20 vertices, 32 duplicate/reordered graphs with 5..16 vertices, all sources, a 2048-vertex chain and a competing-frontier fixture. Seeds 0xb15120261029 and 0xb15220261029; generated diagnostics include case/source.

Floyd-Warshall initializes diagonal zero, arcs one and other entries n+1. Each k applies D[u,v]=min(D[u,v],D[u,k]+D[k,v]); entries >n become None. It shares no FIFO discovery control flow with production. Explicit chain and competing-frontier expected vectors add independent fixtures.

Reverse adjacency vectors and repeat each listed edge three times. Minimum edge counts must stay identical for every source; compare both graphs against Floyd, not just each other.

New compiled source mutation: queue.pop_front() changed to queue.pop_back(); only new long_chain_and_competing_frontier_paths fails exit 101. Exactly one filtered new test failed, with exit101, not a timeout. External probe compared the full production prefix before cfg(test) with main 600f1d961 byte-for-byte. Cargo crate checks passed39 tests; strict Clippy and formatting passed. No production algorithm was changed.

## application-transfers

### Application transfers

Declared consumers of this algorithm's modeled mechanism:

- bfs-annotation: [annot/src/ann.rs](../../../../annot/src/ann.rs.groupant/model.md#transfer)
- bfs-conversion: [en/src/convert.rs](../../../../en/src/convert.rs.groupant/model.md#transfer)
- bfs-spatial-adjacency: [world/src/spartqa.rs](../../../../world/src/spartqa.rs.groupant/model.md#transfer)

[Root transfer catalog](../../../../models/mova.groupant/algorithm-transfers.json). These are scoped mechanism transfers; original algorithm ownership and numerical contracts remain unchanged. Evidence status is recorded in the catalog.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Prospective prediction was bounded oracle agreement, deterministic output, input preservation and a rejected source mutation. Two tests passed twice for this file. Floyd-Warshall on all 530 graphs with 1..=3 vertices and every source (1,570 source queries). Normalized logs agree excluding elapsed timing. The finite source mutant fails with exit 101. A preliminary Dijkstra greater-than relaxation mutant timed out on cycles; the reported negative control instead adds one to edge weights and yields a finite assertion failure. Retain this distinction between liveness and value evidence. No speed claim is made.

#### Expanded test epoch 2026-10-29

Original inline tests now reside in adjacent tests.rs, registered separately as algorithm-bfs-tests. The production prefix is byte-identical to main 600f1d961; only test-module wiring changes. Two original tests plus 40 generated graphs with 4..20 vertices, 32 duplicate/reordered graphs with 5..16 vertices, all sources, a 2048-vertex chain and a competing-frontier fixture. Seeds 0xb15120261029 and 0xb15220261029; generated diagnostics include case/source. Five tests passed twice with normalized-identical output; strict Clippy and formatting passed. queue.pop_front() changed to queue.pop_back(); only new long_chain_and_competing_frontier_paths fails exit 101. This newer evidence extends, rather than erases, the earlier small-domain observations. Tests and production syntax have separate source hashes; production AST does not inline the test module.

### 2026-10-29 · tests

2026-10-29 expanded test epoch: rustc --edition 2024 --test algorithms/src/bfs/mod.rs runs adjacent tests.rs. Five tests passed twice with normalized identical results. The original two tests and their assertions remain; earlier source mutation observations are retained in model.md#descriptant.

### 2026-11-01 · descriptant

#### Independent layer-fiber review 2026-11-01

For valid endpoints, a candidate distance vector with a unique zero at the source is compatible exactly when every finite positive layer has a predecessor in the preceding layer and every edge leaving a finite layer reaches a finite layer at most one greater. The predecessor chain supplies a path of the proposed length; the edge constraint bounds that proposed distance above by the length of every path. Together they give the minimum. This is a source-derived argument, not a mechanized proof of Rust execution.

An external Rust probe compared that predicate with the actual API across all simple directed graphs on one to three vertices, every source, and candidate entries None or 0 through V+1: 332,584 graph/source/vector comparisons on 2026-11-01. Removing the edge constraint falsely admits graph 0→1 with [0, None]. Separate fixtures check source-error precedence and global validation of an unreachable malformed row. The graph enumeration does not cover duplicates, larger graphs, allocation failures or arbitrary error fibers. The behavior complement remains a specification-relative exclusion; the converse is a relation, not an inverse implementation. No runtime or approximation guarantee follows.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
