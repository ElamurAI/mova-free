# algorithm-two-sat

## descriptant

### algorithm-two-sat descriptant

#### Design

Reject doubled variable-count overflow then first out-of-range literal in clause order, left before right. Encode negative literal 2v, positive 2v+1; each (a OR b) adds NOT a -> b and NOT b -> a. Reuse canonical SCC partition, reject any variable sharing its negation's SCC, build condensation and reuse lexicographic topological sort. Assign v true exactly when rank(SCC(positive v)) exceeds rank(SCC(negative v)).

The combiner joins authoritative Rust, source-bound partial syntax, forward relation, first behavior complement, second reverse reconstruction, invariants and finite evidence. Oracle agreement constrains only observed projections. A returned optimal witness alone is insufficient to reconstruct the deterministic tie trace; inverse models preserve this ambiguity.

Canonical literature: Aspvall, Plass and Tarjan, A Linear-Time Algorithm for Testing the Truth of Certain Quantified Boolean Formulas (1979). This implementation reuses local SCC and topological primitives, with original adaptation and tests; deterministic canonical sorting adds logarithmic work.

#### Prospective predictions

The design predicts exact objective agreement with independent bounded enumeration, valid witnesses, deterministic normalized debug/release outcomes and rejection of the specified compiled mutant. Symbolic cost predicts no wall-clock speedup; no timing benchmark is claimed. These design predictions are separate from the measured records in model.md#evidence.

#### Retained observations and open work

This is the first modeling epoch for this file. Exact source/output agreement on finite fixtures does not prove the phase theorem, SCC implementation, or all possible heuristic graphs. Resource exhaustion and semantic AST analysis remain outside this epoch. No numerical approximation budget is justified for categorical decisions or arbitrary witness changes.

## model

# algorithm-two-sat

## Domain

A usize variable count and finite list of disjunction pairs Literal{variable,positive}; literals must address the declared variables. Observations are ordered validation errors, None, or a complete Boolean assignment. Allocations succeed; input immutable.

## Forward

Reject doubled variable-count overflow then first out-of-range literal in clause order, left before right. Encode negative literal 2v, positive 2v+1; each (a OR b) adds NOT a -> b and NOT b -> a. Reuse canonical SCC partition, reject any variable sharing its negation's SCC, build condensation and reuse lexicographic topological sort. Assign v true exactly when rank(SCC(positive v)) exceeds rank(SCC(negative v)).

## Invariant

Every implication has its contrapositive from the same clause. SCC membership is mutual reachability. A component and its negated component are distinct for satisfiable instances. With condensation topological ranks, choosing the later literal cannot falsify a clause: both false literals would impose a rank inequality cycle through their implications. Canonical SCC IDs alone do not meet this premise.

## Cost

For V variables and C clauses the implication graph has 2V nodes and 2C arcs. Existing SCC sorting plus condensation heap sort yields O(V+C+V log(V+1)) time and O(V+C) auxiliary storage. Duplicate arcs remain counted; no deduplication cost hidden.

## nonbehavior

Type: behavior_complement.

In the declared input/result universe exclude accepted invalid literals, wrapped node count, incomplete assignments, any returned assignment falsifying a clause, and None for a formula with a satisfying assignment. Deterministic tie-rule violations are also excluded even if the returned assignment satisfies the formula; this source-derived relation is not inferred merely from negative tests.

## assignment_preimages

Type: reverse_reconstruction.

For output Some(a), compatible formulas on |a| variables satisfy every clause under a and have the canonical SCC/condensation rank inequalities selecting exactly a. The satisfaction condition alone is an outer preimage approximation, since some satisfying assignments are not selected. None corresponds to formulas with a variable and its negation mutually reachable. Error preimages recover count overflow or the first invalid literal position and payload. Duplicate clauses supply distinct full-output antecedents.

## unsat_certificate

Type: certificate_relation.

If a variable and its negation lie in one SCC there are implication paths in both directions. Each satisfying valuation must preserve truth along implications, so either truth choice contradicts the opposite endpoint. This is a mathematical certificate relation; the API returns None without exposing the actual paths.

## condensation_join

Type: composition_relation.

The SCC helper sorts components by minimum vertex, not topological order. Map every implication through its component owner, omit self arcs, preserve duplicates, and obtain topological ranks through the existing DAG helper. Assignment correctness depends on this wiring; SCC child correctness alone does not establish the solver.

## clause_order

Type: permutation_relation.

Permuting or duplicating clauses preserves the implication reachability and canonical SCCs. Condensation arc multiplicity/order cannot alter smallest-ready-vertex topological choice, hence the exact assignment is unchanged. Validation errors are excluded from this relation because clause indices and first-fault ordering can change.

## literal_negation

Type: equivariance_relation.

Complementing any chosen subset of variable polarities maps satisfying assignments bijectively by flipping the same bits. Satisfiability is invariant. The solver's selected assignment need not transform identically, because numeric component tie breaking is not equivariant; only the transformed witness's feasibility is asserted.

## clause_strengthening

Type: order_relation.

Adding clauses can only remove satisfying valuations. Unsatisfiability is upward closed under valid clause additions. A newly returned assignment, if any, satisfies the old formula; assignment identity is not preserved in general.

## boundary_formulas

Type: boundary_specialization.

Zero variables with no clauses yields the empty assignment; variables with no clauses are all true under the canonical numeric rank tie rule. A repeated literal is a unit clause. Opposite unit clauses force None. Tautologies impose no truth constraint, though a deterministic selected witness remains governed by ranks.

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

Date: 2026-10-30. Source SHA-256: `d74209db33fe4542c35b288194dfb98cbdfaa641eaede823b8c245ee150cf802`.

Validation completed after restoring the compiled mutations. External run root: `$MOVA_DATA/runs/octopus-wave7-graphs-2026-10-30/`. No teacher, GPU or live API execution is involved.

#### Executed validation

All 269 library tests pass twice in debug and twice in release, including six tests for this algorithm (18 additions across the three modules). Normalized sorted outcomes agree in all four executions: SHA-256 `f9d87c0f117ccf985e0cc849c0a0ed47d85c1acf513a9db7c52c1f16edc01337`. Strict `cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings` and focused rustfmt pass. Temporary library exports were restored before commit. Source SHA-256 `d74209db33fe4542c35b288194dfb98cbdfaa641eaede823b8c245ee150cf802`; twice-generated syntax SHA-256 `53e306beaa275b3a5070fd8c32aad87285eff8aa28257400518337814a600316`.

The production mutation is defined in this algorithm's test model. It compiled, executed the selected test and failed an assertion (not a compiler failure). Receipt `mutant-two_sat.log` SHA-256 `30fc2aab4ae75ec5741cfed0945fc0ddafc95e836576d419fe16b505895b7c66`. Restored production passes all 269 library tests. The changed-file gate reports all six new sources registered in its temporary scoped registry; groupant structural validation reports no diagnostics. These gates validate declarations and bindings, not semantics.
