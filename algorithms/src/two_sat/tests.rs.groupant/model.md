# algorithm-two-sat-tests

## descriptant

### algorithm-two-sat-tests descriptant

#### Design

Enumerate all 1,024 subsets of the ten unordered two-literal clauses over two variables. A separate four-valuation truth table checks satisfiability and every returned witness. Additional cases cover empty input, units, contradictions, tautologies, first validation failures, checked count overflow, a rank-order regression, clause permutations/duplicates, polarity flips and a 2,000-variable implication chain.

The combiner joins authoritative Rust, source-bound partial syntax, forward relation, first behavior complement, second reverse reconstruction, invariants and finite evidence. Oracle agreement constrains only observed projections. A returned optimal witness alone is insufficient to reconstruct the deterministic tie trace; inverse models preserve this ambiguity.

Canonical literature: Aspvall, Plass and Tarjan, A Linear-Time Algorithm for Testing the Truth of Certain Quantified Boolean Formulas (1979). This implementation reuses local SCC and topological primitives, with original adaptation and tests; deterministic canonical sorting adds logarithmic work.

#### Prospective predictions

The design predicts exact objective agreement with independent bounded enumeration, valid witnesses, deterministic normalized debug/release outcomes and rejection of the specified compiled mutant. Symbolic cost predicts no wall-clock speedup; no timing benchmark is claimed. These design predictions are separate from the measured records in model.md#evidence.

#### Retained observations and open work

This is the first modeling epoch for this file. Exact source/output agreement on finite fixtures does not prove the phase theorem, SCC implementation, or all possible heuristic graphs. Resource exhaustion and semantic AST analysis remain outside this epoch. No numerical approximation budget is justified for categorical decisions or arbitrary witness changes.

## model

# algorithm-two-sat-tests

## Domain

Fixed finite Rust fixtures and the production algorithm, observed as per-test pass or assertion failure. The compiler, Rust test runner and successful allocations are trusted. Test acceptance is not unrestricted implementation equivalence.

## Forward

Enumerate all 1,024 subsets of the ten unordered two-literal clauses over two variables. A separate four-valuation truth table checks satisfiability and every returned witness. Additional cases cover empty input, units, contradictions, tautologies, first validation failures, checked count overflow, a rank-order regression, clause permutations/duplicates, polarity flips and a 2,000-variable implication chain.

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

Enumerate all 1,024 subsets of the ten unordered two-literal clauses over two variables. A separate four-valuation truth table checks satisfiability and every returned witness. Additional cases cover empty input, units, contradictions, tautologies, first validation failures, checked count overflow, a rank-order regression, clause permutations/duplicates, polarity flips and a 2,000-variable implication chain.

## production_mutant

Type: counterfactual_relation.

Reverse the final component-rank comparison from > to <; the canonical_components_require_topological_ranks assertion must fail after compilation.

## finite_extension

Type: coverage_relation.

Extending the fixture set intersects the set of passing implementations with additional assertion constraints. An implementation can pass the current finite set and fail an extension. Repeating unchanged tests in debug and release constrains profile-dependent behavior but does not expand the graph/formula domain or prove total correctness.

## Limits

Partial source-derived relations with bounded executable evidence, not formal Rust proofs. Syntax omits macro expansion, name/type resolution and semantic analysis. No executable approximate surrogate is attached: categorical satisfiability, witness feasibility and error behavior have no justified numerical approximation metric here. Resource exhaustion remains outside scope.

## evidence

### Finite evidence

Enumerate all 1,024 subsets of the ten unordered two-literal clauses over two variables. A separate four-valuation truth table checks satisfiability and every returned witness. Additional cases cover empty input, units, contradictions, tautologies, first validation failures, checked count overflow, a rank-order regression, clause permutations/duplicates, polarity flips and a 2,000-variable implication chain.

#### Scope and retained corrections

Raw logs and the full hash receipt are in the external run root. The initial strict Clippy pass rejected index-based matrix loops; these were rewritten to iterator/split-borrow loops before the repeated tests. The first Stoer-Wagner mutation selector matched both a contraction and a connection sum and stopped without editing that source; its selector was narrowed to the phase connection sum before compiling the successful negative control. u64 overflow boundaries are exercised through u128 totals; actual u128 exhaustion and allocation failure are not exercised. There is no live training, GPU, benchmark or school-quality evidence.

## History

### 2026-10-30 · descriptant

Heading `#### Design epoch 2026-10-30` stays in descriptant as `#### Design`.

### 2026-10-30 · evidence

Date: 2026-10-30. Source SHA-256: `f061dcd98beaae11e970e8c9c035f742d92cfe2cb74bf6d68b4795a06b9e779c`.

Validation completed after restoring the compiled mutations. External run root: `$MOVA_DATA/runs/octopus-wave7-graphs-2026-10-30/`. No teacher, GPU or live API execution is involved.

#### Executed validation

All 269 library tests pass twice in debug and twice in release, including six tests for this algorithm (18 additions across the three modules). Normalized sorted outcomes agree in all four executions: SHA-256 `f9d87c0f117ccf985e0cc849c0a0ed47d85c1acf513a9db7c52c1f16edc01337`. Strict `cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings` and focused rustfmt pass. Temporary library exports were restored before commit. Source SHA-256 `f061dcd98beaae11e970e8c9c035f742d92cfe2cb74bf6d68b4795a06b9e779c`; twice-generated syntax SHA-256 `8db306f9ebbed41dc59b1e538a12b7709be099d4697b0e8dac34734186ec985f`.

The production mutation is defined in this algorithm's test model. It compiled, executed the selected test and failed an assertion (not a compiler failure). Receipt `mutant-two_sat.log` SHA-256 `30fc2aab4ae75ec5741cfed0945fc0ddafc95e836576d419fe16b505895b7c66`. Restored production passes all 269 library tests. The changed-file gate reports all six new sources registered in its temporary scoped registry; groupant structural validation reports no diagnostics. These gates validate declarations and bindings, not semantics.
