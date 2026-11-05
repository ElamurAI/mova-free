# algorithm-adaptive-simpson

## descriptant

### adaptive_simpson mod.rs

Deterministic callback on ordered finite endpoints, positive finite absolute tolerance, budget>=3 and depth<=64. Cache panel endpoints/midpoint, compare two half Simpson panels with coarse panel, accept corrected S2+(S2-S1)/15 when |S2-S1|/15<=local tolerance. Otherwise split tolerance and interval; report explicit budget or stagnation. EstimatedTolerance is a heuristic stopping reason, never an accuracy certificate.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

## model

## behavior_complement

Let D be validated inputs and F the source f64 transition relation including tagged errors/status. Behavior complement is all candidate (input, output) pairs not in F: in particular nonfinite successful values, wrong recurrence results and falsely universal error guarantees. Invalid-input observations are errors, not approximate numeric answers. Deterministic callback on ordered finite endpoints, positive finite absolute tolerance, budget>=3 and depth<=64. Cache panel endpoints/midpoint, compare two half Simpson panels with coarse panel, accept corrected S2+(S2-S1)/15 when |S2-S1|/15<=local tolerance. Otherwise split tolerance and interval; report explicit budget or stagnation. EstimatedTolerance is a heuristic stopping reason, never an accuracy certificate.

## reverse_reconstruction

Every callback agreeing on visited abscissae follows the same traversal and report. Zero function and a bump supported inside (0.1,0.2) share the initial five samples and produce identical zero reports despite integrals 0 and 0.1. Unvisited function values, smoothness, integral and derivative bounds cannot be reconstructed.

## sample_alias_fiber

A finite adaptive trace cannot exclude an unsampled bump. The tested bump returns zero at 0,1/4,1/2,3/4,1; estimate zero is compatible with nonzero integral.

## budget_partition

Budget>=3 funds initial samples; subsequent pair requires remaining>=2. Unresolved leaves contribute coarse panels and error sentinel MAX. Depth exhaustion contributes refined panel. First unresolved reason is retained; later unresolved reasons are not separately enumerated.

## stagnation_relation

Require strict a<quarter<midpoint<threequarter<b. Adjacent representable endpoints produce Stagnation. If tolerance halving underflows on an unresolved panel, report Stagnation instead of indefinite subdivision.

## additive_composition

Accepted panel tolerances halve on split; real allocated tolerances sum to original. Machine estimated-error sum is rounded, without universal <=tol guarantee. Under sufficiently smooth asymptotic behavior the /15 estimate is motivated by fourth-order error; finite samples give no bound.


## approximant

### Forward model and invariant

Deterministic callback on ordered finite endpoints, positive finite absolute tolerance, budget>=3 and depth<=64. Cache panel endpoints/midpoint, compare two half Simpson panels with coarse panel, accept corrected S2+(S2-S1)/15 when |S2-S1|/15<=local tolerance. Otherwise split tolerance and interval; report explicit budget or stagnation. EstimatedTolerance is a heuristic stopping reason, never an accuracy certificate.

Invariant: all successful scalar results are finite; invalid input and nonfinite arithmetic stay distinct from values. Simpson pending panels partition the original interval, cached values match their abscissae and evaluations stay within budget. Tests preserve fixture independence and never infer universal accuracy from passing samples.

Cost: O(evaluations) scalar work plus callback cost. Fixed 65-panel stack, depth cap 64, no heap allocation. Evaluation count never exceeds supplied usize budget; increment by two only after subtraction check. Callback own resources/panics excluded.

Numerical scope: real formulas motivate f64 recurrences, not bitwise real semantics. No universal relative bound near zero/cancellation. No useful cheaper certified executable surrogate chosen; this remains an explicit gap.

## tests

### Evidence

analytic_polynomials tests degrees 0..7 and sine against exact antiderivatives; budgets count actual calls, depth and adjacent-endpoint stagnation, hidden bump, overflow and invalid input.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: independent polynomial/basis/integer fixtures and explicit finite-error partitions. Simpson hidden bump demonstrates limits of sampled estimates. An initial correction-denominator mutant survived tight-tolerance tests; a loose-tolerance quartic assertion was added to isolate the correction, and that mutant was then rejected. No speed claim; no percentages inferred from finite observations. Rust syntax projection is partial, without macro expansion, type resolution or semantic proof.

### 2026-10-30 · tests

Compiled production negative control: Change Simpson midpoint weight from 2/3 to 1/3; analytic_polynomials fails. Original bytes restored and named positive test rerun. Full algorithms library passes twice debug and twice release with identical sorted test-name/results; receipts bind logs under $MOVA_DATA/runs/octopus-wave19-numeric-2026-10-30. Source projection generated twice and compared before installation. Strict crate all-target Clippy attempted; outcome in receipt.

### 2026-11-01 · descriptant

Epoch 2026-11-01: reviewed the odd/even evaluation-budget fiber and its
boundary control. This concerns callback trace and report, not the integral's
true error. Full floating-point semantics and arbitrary callback effects remain
outside the finite probe.

### 2026-11-01 · document

#### Budget parity fiber — 2026-11-01

For the same valid nonzero interval, tolerance, depth and deterministic finite
callback, budgets `2k+3` and `2k+4` (when representable) have the same callback
trace and report, provided execution returns normally. Initial sampling uses
three calls and each later sampling step needs two, so the unpaired extra unit
cannot change the budget decision. This is a budget-input fiber of the observed
execution, not reconstruction of the callback or a numerical accuracy bound.
Zero width instead uses no calls; invalid inputs, callback panics and nonfinite
arithmetic remain separate outcomes. A quartic on [0,1] with tolerance 1e-30
and depth 3 distinguishes budgets 4 and 5: the former stops on the coarse panel,
the latter evaluates a pair. A source-linked Rust probe checked 20 consecutive
odd/even budget pairs; this finite check does not prove all floating-point paths.

### 2026-11-01 · tests

2026-11-01 independent source-linked probe: deterministic quartic traces and
reports agree for budgets 3/4 through 41/42; budgets 4/5 differ. The paired-call
argument is source-derived; this fixture is finite evidence, not an accuracy
certificate or a proof of all callback behavior.
