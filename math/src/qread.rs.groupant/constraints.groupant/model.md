# math-qread-constraints

## descriptant

### math-qread-constraints

Event accumulation at upto sums Set/Add values and subtracts Sub values for matching events with t<=upto, excluding Is/Rel. Before that accumulation, relation/direct/movement/initial-state decisions inspect the full event list. For q=a*u+r with one remaining unknown, solve inserts u=-r/a, in encounter order over four passes.

## approximant

### Scoped forward relation

Event accumulation at upto sums Set/Add values and subtracts Sub values for matching events with t<=upto, excluding Is/Rel. Before that accumulation, relation/direct/movement/initial-state decisions inspect the full event list. For q=a*u+r with one remaining unknown, solve inserts u=-r/a, in encounter order over four passes.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Valid event and leaf indices are required. Relation recursion stops at depth3. Conditional temporal fold membership is preserved, but prefix causality is not guaranteed.

#### Cost

Repeated full-history scans occur before and during state queries; four solver passes repeatedly clone/substitute coefficient maps. Relation recursion has bounded depth3, not constant total event work.

## models

## behavior_complement

For owner=None, outside events cannot enter the accumulation. Is/Rel cannot be directly summed. An equation still containing two unknowns cannot add a solution in that visit. The implementation does not exclude dependence on future events when querying an earlier upto.

## reverse_reconstruction

With no future data, an initial Add may start at zero; adding a later Is for the same owner/thing can introduce an initial unknown into the same early-upto query. Distinct equation lists containing u-leaf0=0 or 2*u-2*leaf0=0 can yield the same solution, so neither prefix history nor equations are reconstructed uniquely.

## full_history_branching

direct, moved, initial_ev and told_state scans have no upto filter. Thus state_d(history,upto) is not generally equal to state_d(history restricted to t<=upto,upto). Only the final event fold is time-filtered.

## ordered_single_unknown

After substitution, exactly one unknown triggers assignment; zero or>=2 unknowns produce no insertion. Four passes can propagate short chains but are neither Gaussian elimination nor a consistency/uniqueness certificate.

## tests

### Scoped evidence

Static source review only; no nonempty event-state/solve fixture. Solver completeness and contradictions remain unverified.

Named actual source fixtures: none.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

#### Wave34 direct-helper refinement

Earlier source-only gaps and wave28 receipts above are historical. Current descendants add actual helper fixtures; see ../model.md#wave34-evidence and each descendant's Tests section. Full English parsing and general solver correctness remain gaps.

## wave34-composition

# Constraint descendant composition

single-unknown-passes constructs its equations by invoking temporal-fold through state. Exact-thing/head selection and owner/total routing happen before the fold. Fold branch decisions inspect full history, while its final accumulator filters by upto; therefore an early-time equation may depend on a later condition. The solver then freezes the equation vector and performs four ordered substitution passes using Lin's pruning relation.

An output solution preimage needs event histories whose routed state calls create compatible equations in that construction order. A correct local isolation formula does not establish that the selected event equation expresses the story. Two unknowns after substitution cause no insertion; zero unknowns skip consistency checking. Relations, whole/part and time construction remain source-derived beyond the bounded direct Is fixtures.

## Cost

Equation assembly includes repeated event scans and bounded-depth relation exploration, plus affine map costs. Four passes visit each equation and pay actual support-dependent substitution and isolation work. No Gaussian solver or convergence complexity is claimed.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.
