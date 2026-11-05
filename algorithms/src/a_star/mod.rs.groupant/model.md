# algorithm-a-star

## descriptant

### algorithm-a-star descriptant

#### Design

Validate start then goal, heuristic length, zero goal heuristic, all endpoints, then all edge inequalities h(u)<=w(u,v)+h(v). On admitted inputs A* orders snapshots by (g+h,g,vertex), discards stale distances and strictly relaxes edges. Return the first fresh goal pop with its predecessor chain; exhaustion means unreachable.

The combiner joins authoritative Rust, source-bound partial syntax, forward relation, first behavior complement, second reverse reconstruction, invariants and finite evidence. Oracle agreement constrains only observed projections. A returned optimal witness alone is insufficient to reconstruct the deterministic tie trace; inverse models preserve this ambiguity.

Canonical literature: Hart, Nilsson and Raphael, A Formal Basis for the Heuristic Determination of Minimum Cost Paths (1968). Original implementation and fixtures; no copied source.

#### Prospective predictions

The design predicts exact objective agreement with independent bounded enumeration, valid witnesses, deterministic normalized debug/release outcomes and rejection of the specified compiled mutant. Symbolic cost predicts no wall-clock speedup; no timing benchmark is claimed. These design predictions are separate from the measured records in model.md#evidence.

#### Retained observations and open work

This is the first modeling epoch for this file. Exact source/output agreement on finite fixtures does not prove the phase theorem, SCC implementation, or all possible heuristic graphs. Resource exhaustion and semantic AST analysis remain outside this epoch. No numerical approximation budget is justified for categorical decisions or arbitrary witness changes.

## model

# algorithm-a-star

## Domain

Finite directed adjacency lists of usize endpoints and u64 weights, usize start and goal, a u64 heuristic vector. Input remains immutable; allocations succeed. Observations are typed errors, None, or a u128 distance with a vertex witness.

## Forward

Validate start then goal, heuristic length, zero goal heuristic, all endpoints, then all edge inequalities h(u)<=w(u,v)+h(v). On admitted inputs A* orders snapshots by (g+h,g,vertex), discards stale distances and strictly relaxes edges. Return the first fresh goal pop with its predecessor chain; exhaustion means unreachable.

## Invariant

Each finite g is a realized walk cost. Consistency gives nondecreasing f along an improved path and implies admissibility by telescoping to the zero-heuristic goal. A fresh pop settles its vertex. Nonnegative weights and strict improvement prevent predecessor cycles, including zero cycles. Thus the first fresh goal pop is optimal. u128 checked arithmetic prevents wrapped priorities.

## Cost

Validation O(V+E); each vertex row is expanded at most once under consistency and there are at most E successful relaxations. Heap work O((V+E) log(V+E+1)), space O(V+E). These are symbolic operation bounds, not measured wall-clock predictions.

## nonbehavior

Type: behavior_complement.

Within the declared input/result universe, excluded transitions include accepting a failed validation, incorrect first-error payloads, returning a non-realizable or nonminimal path, a cyclic witness, None for a reachable goal, and wrapped arithmetic. This is a source-derived inner description of semantic nonbehavior, not an empirical complement of sampled outcomes.

## path_preimages

Type: reverse_reconstruction.

For a distance-only observation d, candidate antecedents are every admitted graph/start/goal/heuristic with a realizing path of cost d and no cheaper path; None has precisely unreachable-goal antecedents. For a full Path observation additionally require the deterministic heap/strict-parent trace to select those vertices. Omitting this trace is explicitly an outer reconstruction approximation. Errors recover inputs with exactly the first failing check. Self-loop changes and multiple consistent heuristics give nonunique fibers.

## potential_reweighting

Type: potential_relation.

For every admitted arc define reduced cost w+h(v)-h(u)>=0 in mathematical integers. A start-to-v path has reduced sum g+h(v)-h(start), so the A* priority differs from reduced distance by the fixed constant h(start). This explains settlement; it does not admit merely guessed heuristic values.

## heuristic_independence

Type: projection_relation.

On a fixed graph/start/goal every two admitted heuristic vectors produce the same distance or None. Their witness paths may differ because heap order changes. Zero heuristic is always admitted after endpoint validation and recovers Dijkstra distance semantics.

## path_certificate

Type: certificate_relation.

A returned vertex list begins at start, ends at goal, contains no repeated vertex and traverses actual arcs. With parallel arcs the minimum capacity-free edge weight between successive vertices sums to the returned distance. This realizes an upper bound; optimality additionally requires the settlement argument or an independent lower-bound oracle.

## weight_order

Type: order_relation.

Increasing edge weights on fixed topology cannot reduce the optimal distance. A previously consistent heuristic stays consistent under increases. Reachability stays fixed. Witness equality is not implied; decreasing weights can invalidate a previously admitted heuristic.

## validation_partition

Type: error_partition.

Start and goal errors precede vector-length and goal-value errors. Endpoints are checked globally before any consistency inequality, including disconnected components. Within each scan stored row and edge order fixes the first failure. ArithmeticOverflow is a defensive checked-sum error; current bounded fixtures exceed u64 but do not reach u128 exhaustion.

## redundant_loops

Type: closure_relation.

Adding nonnegative self loops preserves consistency, reachability, shortest distance and the chosen strict-improvement trace. Duplicating an identical arc also preserves strict improvements. This supplies concrete many-to-one full-output preimages rather than a claim of unique graph recovery.

## Limits

Partial source-derived relations with bounded executable evidence, not formal Rust proofs. Syntax omits macro expansion, name/type resolution and semantic analysis. No executable approximate surrogate is attached: categorical satisfiability, witness feasibility and error behavior have no justified numerical approximation metric here. Resource exhaustion remains outside scope.

## evidence

### Finite evidence

The simple-path DFS enumerates 729 three-vertex directed graphs, six non-loop arcs independently absent/zero/two. Nine endpoint pairs and two admitted heuristics give 13,122 calls, checking objective and path witness. Other fixtures cover rejection order, invalid disconnected components, two u64::MAX edges, stale snapshots, zero cycles, ties and loop fibers.

#### Scope and retained corrections

Raw logs and the full hash receipt are in the external run root. The initial strict Clippy pass rejected index-based matrix loops; these were rewritten to iterator/split-borrow loops before the repeated tests. The first Stoer-Wagner mutation selector matched both a contraction and a connection sum and stopped without editing that source; its selector was narrowed to the phase connection sum before compiling the successful negative control. u64 overflow boundaries are exercised through u128 totals; actual u128 exhaustion and allocation failure are not exercised. There is no live training, GPU, benchmark or school-quality evidence.

## History

### 2026-10-30 · descriptant

Heading `#### Design epoch 2026-10-30` stays in descriptant as `#### Design`.

### 2026-10-30 · evidence

Date: 2026-10-30. Source SHA-256: `76a320c9ade011864661dd9edcbad7ca6d200346cbdea407d626eaedf024670d`.

Validation completed after restoring the compiled mutations. External run root: `$MOVA_DATA/runs/octopus-wave7-graphs-2026-10-30/`. No teacher, GPU or live API execution is involved.

#### Executed validation

All 269 library tests pass twice in debug and twice in release, including six tests for this algorithm (18 additions across the three modules). Normalized sorted outcomes agree in all four executions: SHA-256 `f9d87c0f117ccf985e0cc849c0a0ed47d85c1acf513a9db7c52c1f16edc01337`. Strict `cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings` and focused rustfmt pass. Temporary library exports were restored before commit. Source SHA-256 `76a320c9ade011864661dd9edcbad7ca6d200346cbdea407d626eaedf024670d`; twice-generated syntax SHA-256 `4689f3c7bfa2bfda1758d1ca75ef8132b5430cb73bdca0e032e174bc5dd1f4ff`.

The production mutation is defined in this algorithm's test model. It compiled, executed the selected test and failed an assertion (not a compiler failure). Receipt `mutant-a_star.log` SHA-256 `4bf05740e08cdfdbc78188dd7600faf97d99ebbc030a46dae898ab9c4513efa7`. Restored production passes all 269 library tests. The changed-file gate reports all six new sources registered in its temporary scoped registry; groupant structural validation reports no diagnostics. These gates validate declarations and bindings, not semantics.

### 2026-11-01 · descriptant

#### Review epoch 2026-11-01

A diamond fixture distinguishes equal distance from equal witness under two consistent heuristics. Global validation also rejects an inconsistent disconnected edge before search. These observations sharpen the existing projection and error-partition models without changing their source or claiming a full proof. See model.md#evidence.

### 2026-11-01 · evidence

#### Heuristic projection review — 2026-11-01

For the diamond with arcs 0→1, 0→2, 1→3 and 2→3, each of weight 1, the actual `shortest_path` returns distance 2 via `[0,1,3]` with zero heuristic and distance 2 via `[0,2,3]` with `[0,1,0,0]`. Both heuristics satisfy every consistency inequality and vanish at goal 3. This falsifies full witness independence while supporting the distance projection on this fixture. A separate disconnected inconsistent arc is rejected before search, including when a valid goal path exists. The offline, locked external Rust probe called the source directly and produced identical outputs twice. This does not establish the general settlement argument or exercise arithmetic exhaustion; the earlier simple-path enumeration supplies a separate bounded objective oracle. No numerical approximation or runtime claim follows.
