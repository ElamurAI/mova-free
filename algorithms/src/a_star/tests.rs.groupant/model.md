# algorithm-a-star-tests

## descriptant

### algorithm-a-star-tests descriptant

#### Design

The simple-path DFS enumerates 729 three-vertex directed graphs, six non-loop arcs independently absent/zero/two. Nine endpoint pairs and two admitted heuristics give 13,122 calls, checking objective and path witness. Other fixtures cover rejection order, invalid disconnected components, two u64::MAX edges, stale snapshots, zero cycles, ties and loop fibers.

The combiner joins authoritative Rust, source-bound partial syntax, forward relation, first behavior complement, second reverse reconstruction, invariants and finite evidence. Oracle agreement constrains only observed projections. A returned optimal witness alone is insufficient to reconstruct the deterministic tie trace; inverse models preserve this ambiguity.

Canonical literature: Hart, Nilsson and Raphael, A Formal Basis for the Heuristic Determination of Minimum Cost Paths (1968). Original implementation and fixtures; no copied source.

#### Prospective predictions

The design predicts exact objective agreement with independent bounded enumeration, valid witnesses, deterministic normalized debug/release outcomes and rejection of the specified compiled mutant. Symbolic cost predicts no wall-clock speedup; no timing benchmark is claimed. These design predictions are separate from the measured records in model.md#evidence.

#### Retained observations and open work

This is the first modeling epoch for this file. Exact source/output agreement on finite fixtures does not prove the phase theorem, SCC implementation, or all possible heuristic graphs. Resource exhaustion and semantic AST analysis remain outside this epoch. No numerical approximation budget is justified for categorical decisions or arbitrary witness changes.

## model

# algorithm-a-star-tests

## Domain

Fixed finite Rust fixtures and the production algorithm, observed as per-test pass or assertion failure. The compiler, Rust test runner and successful allocations are trusted. Test acceptance is not unrestricted implementation equivalence.

## Forward

The simple-path DFS enumerates 729 three-vertex directed graphs, six non-loop arcs independently absent/zero/two. Nine endpoint pairs and two admitted heuristics give 13,122 calls, checking objective and path witness. Other fixtures cover rejection order, invalid disconnected components, two u64::MAX edges, stale snapshots, zero cycles, ties and loop fibers.

## Invariant

The independent oracle does not call the production algorithm to construct expected objective values. Every successful result witness is checked against the original input. Counts refer to enumerated inputs/calls, not a probability distribution or universal proof.

## Cost

The fixed oracle domains are bounded: simple paths use DFS exponential in vertex count; truth tables cost O(2^V C); cut enumeration costs O(2^V V^2). Only the relevant oracle executes in each file. Production runtime bounds cannot be inferred from this test cost.

## nonbehavior

Type: behavior_complement.

In the universe of fixture/reference/output triples, acceptance excludes any tested objective mismatch, invalid returned witness or mismatched explicit error payload. Behavior outside the named fixture set is not excluded. This acceptance relation is a finite predicate defined by assertions, not a claim that passing forbids all production bugs.

## failure_preimages

Type: reverse_reconstruction.

A failed equality assertion reconstructs the candidate implementation/reference/fixture triples producing unequal compared projections at that location. A pass reconstructs the equivalence class of implementations agreeing on all observed projections, which can include faulty implementations outside the tested domain. No unique source bug or causal diagnosis follows from the failure text.

## independent_oracle

Type: oracle_relation.

The simple-path DFS enumerates 729 three-vertex directed graphs, six non-loop arcs independently absent/zero/two. Nine endpoint pairs and two admitted heuristics give 13,122 calls, checking objective and path witness. Other fixtures cover rejection order, invalid disconnected components, two u64::MAX edges, stale snapshots, zero cycles, ties and loop fibers.

## production_mutant

Type: counterfactual_relation.

Replace candidate cost increment u128::from(weight) by zero; the wide_costs_and_heuristic_addition assertion must fail after successful compilation.

## finite_extension

Type: coverage_relation.

Extending the fixture set intersects the set of passing implementations with additional assertion constraints. An implementation can pass the current finite set and fail an extension. Repeating unchanged tests in debug and release constrains profile-dependent behavior but does not expand the graph/formula domain or prove total correctness.

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

Date: 2026-10-30. Source SHA-256: `16cb696512e77d992bbd92f59830a6349c35625bb6e645cd98ff3a8dc4e648cf`.

Validation completed after restoring the compiled mutations. External run root: `$MOVA_DATA/runs/octopus-wave7-graphs-2026-10-30/`. No teacher, GPU or live API execution is involved.

#### Executed validation

All 269 library tests pass twice in debug and twice in release, including six tests for this algorithm (18 additions across the three modules). Normalized sorted outcomes agree in all four executions: SHA-256 `f9d87c0f117ccf985e0cc849c0a0ed47d85c1acf513a9db7c52c1f16edc01337`. Strict `cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings` and focused rustfmt pass. Temporary library exports were restored before commit. Source SHA-256 `16cb696512e77d992bbd92f59830a6349c35625bb6e645cd98ff3a8dc4e648cf`; twice-generated syntax SHA-256 `a12ffa031736ffe7e1777892fce7de7dcb395564339739c623ef9794ec64a15a`.

The production mutation is defined in this algorithm's test model. It compiled, executed the selected test and failed an assertion (not a compiler failure). Receipt `mutant-a_star.log` SHA-256 `4bf05740e08cdfdbc78188dd7600faf97d99ebbc030a46dae898ab9c4513efa7`. Restored production passes all 269 library tests. The changed-file gate reports all six new sources registered in its temporary scoped registry; groupant structural validation reports no diagnostics. These gates validate declarations and bindings, not semantics.

### 2026-11-01 · descriptant

#### Review epoch 2026-11-01

The witness checks minimum parallel-edge cost for each vertex pair; it cannot recover the actual edge or relaxation trace. The finite source-backed counterexample and compatible-edge fiber are recorded in the model and evidence. The exhaustive graph family excludes loops and parallel arcs, which are covered only by other fixed fixtures. No generalized proof is inferred.

### 2026-11-01 · document

#### Parallel-edge observation boundary — 2026-11-01

For a returned vertex sequence `p`, the witness projects each adjacent pair to the minimum weight among parallel input arcs and sums those minima in `u128`. Its accepted result therefore has at least one compatible selection of input arcs with the reported cost, provided the checked endpoint, simplicity and edge-existence assertions hold. It does not identify the arcs actually relaxed by the search. A graph with parallel weights 9 and 2 and a graph with only weight 2 can both produce `([0, 1], 2)`; selecting the weight-9 arc is incompatible with that cost. Equal-weight parallel arcs add further indistinguishable choices. The output-to-input converse is a fiber of compatible graphs and edge selections, not a unique search trace. This finite example refutes identifying the reported vertex witness with a particular input arc; it does not refute witness feasibility or establish general A* correctness. The exhaustive three-vertex graph family has no parallel edges or self-loops; separate fixed fixtures exercise both. Arithmetic exhaustion and allocation failures remain outside the checked domain.

### 2026-11-01 · evidence

#### Independent review — 2026-11-01

An external Rust probe includes the actual A* module and checks parallel weights 9 and 2 against a separate enumeration of compatible edge weights. The real API returns vertices `[0, 1]` and distance 2; the weight-9 selection fails the cost check, while a graph containing only the weight-2 edge returns the same `Path`. Two probe outputs agree. The six existing A* tests were rerun offline and locked. This is a finite observation of the witness projection, not evidence of which internal edge was relaxed, a universal proof, or an approximation guarantee. The probe, prospective hypothesis and test log are retained in the independent review cycle directory.
