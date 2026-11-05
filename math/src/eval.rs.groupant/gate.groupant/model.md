# math-eval-gate

## descriptant

Scoped gate fragment, source bytes [34232,35748).

Expr -> eval using Rat/units -> Q plus Ctx -> gate using fcheck. Raw eval success is insufficient: existing checks or SI mismatch can reject. Context mutations survive raw errors; answer owns fresh context. No global relative-error budget follows.

Inherited family observations are parent context; only the explicit byte range belongs to this fragment.

## approximant

eval recursively produces Q and appends Ctx evidence. Exact pairs use Rat; mixed approximate pairs use f64. Aggregates convert to first operand units. answer uses fresh context then gate checks independent fcheck SI value and recorded booleans.

O(N) traversal plus numeric costs; aggregate O(k log k); exact probability enumeration O(T*2^T) for T<=16. Trace size includes formatted intermediates; stack/allocation unbounded. Gate tolerance is policy, not universal numerical assurance.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

Point+point and point scaling are excluded, dimension mismatch excludes conversion. Gate rejects any false check or SI disagreement; raw eval does not enforce gate. Empty Add/Mul indexing, nonfinite handling, unchecked count sum and exponent narrowing remain gaps.

Output Q admits many Expr/environment/trace antecedents and equivalent units. Fixed-scale exact conversion has unique inverse only when all arithmetic succeeds. Gate accepts SI-equivalent representations, not a unique syntax or provenance.

Cost: O(N) traversal plus numeric costs; aggregate O(k log k); exact probability enumeration O(T*2^T) for T<=16. Trace size includes formatted intermediates; stack/allocation unbounded. Gate tolerance is policy, not universal numerical assurance.

Finite fixtures do not prove all-domain correctness or resource behavior.

## models

## behavior_complement

Answer is excluded by any false Ctx check, fcheck None, or false SI agreement. Raw eval success is insufficient. Equivalent display units are permitted; upstream structural validity is assumed.

## reverse_reconstruction

Accepted Answer has a set of expression/value/context antecedents producing its retained Q/steps/notes and checks augmented with independent comparison. Numeric SI agreement cannot identify original syntax or provenance uniquely.

## temperature_kind

Q.base transports affine points to Kelvin and multiplicative units by scale. Gate compares transported numbers and does not independently validate point/delta structural legality in publicly constructed Q.

## failure_precedence

answer evaluates before gate. Gate calls fcheck before Q.base; base error propagates before failed-check aggregation. fcheck None appends failure without evaluating base. Previous checks are inspected after independent processing.

## gate_boundary

Tolerance is1e-9*(1+max(abs(b),abs(fv))). Checks append, preserving previous false values. The compiled control bypasses failed-list rejection and must fail the wrong-answer test. This is a gate policy, not a universal approximation guarantee.

## empty_nonfinite

An empty prior check list still receives the independent check. NaN comparisons fail ordinarily; infinities require separate analysis, so no finite-answer guarantee is claimed. Ctx variable bindings are not passed through this interface to fcheck.

## tests

Actual source module filter: eval::. Inspected synthetic module tests only, selected independently; numeric run statuses, sorted named outcomes and compiled controls retained in wave27-math external receipts. No full crate suite, corpus, teacher, GPU or training. See source-test-map.json for exact evidence and limitations.
