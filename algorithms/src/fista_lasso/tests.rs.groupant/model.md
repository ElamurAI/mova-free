# algorithm-fista-lasso-tests

## descriptant

### fista_lasso — tests.rs

A rectangular 1..256 by 1..64, entries/response/lambda magnitudes <=1e6, lambda>=0. A nonzero requires ||A||F²>=1e-12; zero A accepted. The solution may be nonunique.

Report.x with objective .5||Ax-b||²+lambda||x||1, KKT infinity residual, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in ../mod.rs.groupant/model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

FISTA state (x,y,t) starts (0,0,1). Gradient evaluated at y; reports evaluate x. t'=(1+sqrt(1+4t²))/2, y'=x'+(t-1)/t'*(x'-x). First two x iterates equal ISTA; the third distinguishes acceleration. Objective need not decrease at each update.

Dependencies: projected_gradient_box report/buffer and soft_threshold. FISTA additionally composes proximal_gradient_lasso validation/evaluation/prox; all helper preconditions are established by shared validation, with scale=0 handled by converged zero-design stopping.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

Tests call fista_lasso and compare against independent scalar closed forms, hand-computed recurrence iterates, report recomputation, and boundary relations. Finite deterministic assertions only, no sampling from held-out data. O(m*n*(k+1)) time with two gradient/objective evaluations per update, O(n) storage. Three initial checked vectors and one soft_threshold output per update. Shares validation/evaluation/prox with ISTA; distinct t,y recurrence. Test fixture dimensions bounded by the documented API, with small oracle grids and capped 3000-step solves.

Tolerance is absolute, finite and positive; budget 0..100000. All index reductions use deterministic ascending order. Real equations are models, not f64 accuracy proofs. No generic percentage guarantee. Executable 1/2/5/10% surrogates are unsupported pending a useful metric and evidence.

## models

# Typed obernant family

## behavior_complement
For the test harness, a passing result is excluded if any named fixture yields a mismatch outside its explicit 1e-8 comparison tolerance or violates an exact status/error assertion. A noncompiling mutant is excluded from the set of successful mutation controls. The concrete production relations under test are:

For fixed valid input, success with wrong column dimension, nonfinite x/objective/KKT residual, iterations>budget, or Converged with KKT violation>tolerance is excluded. With zero budget only x=0 is compatible. For all-zero A every valid report is zero at iteration 0, value=.5||b||². Invalid shape cannot map to success.

## reverse_reconstruction
A passing test list has many compatible implementations: an implementation differing only on untested valid inputs can produce the same list. Thus reverse reconstruction yields an equivalence class over these finite fixtures, never unique source recovery. The production preimage ambiguities exercised or documented by those fixtures are:

At zero budget the zero vector has preimages from all valid A,b,lambda with status determined by their KKT residual. At stationarity, gradients belong to -lambda*sign(x) on nonzeros and [-lambda,lambda] on zeros; this relation recovers compatible responses, not a unique one. For rank-deficient A, adding a response component in null(A') preserves gradient but changes objective. Even retaining objective permits sign choices of such components. Threshold output 0 has a whole interval [-lambda/L,lambda/L] of pre-proximal coordinates. FISTA t and extrapolated y cannot be recovered from x alone.

## independent_oracle
The 15-case signed diagonal grid solves scalar KKT inequalities directly, including threshold ties. Correlated-design fixture constructs x=(1,-2) and residual=(-.25,0,.25) so A'r matches the L1 subgradient; rank-deficient and zero-column fixtures check nonunique and inactive behavior. Three-step hand formulas distinguish ISTA and FISTA.

## mutation_sensitivity
The exact_recurrence test checks a hand-derived two-step quadratic or three-step LASSO result. A compiled production recurrence mutant must fail that named test. Compilation failure alone is not a killed mutant. Passing identifies a finite observation, not proof of convergence.

## boundary_partition
Tests partition invalid shape/nonfinite operands/limits, accepted exact bounds, zero budget at convergence equality and above tolerance, dimension limits and zero-design or small-curvature cases. Success of the harness excludes those asserted violations on these fixtures only; untested resource exhaustion remains unknown.

## tests

### Executable checks

cargo test -p mova-algorithms fista_lasso::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## validation

### Validation

Five tests passed twice in debug and twice in release, all four normalized named-test result lists identical. Production mutant compiled and failed the named exact_recurrence assertion; restored exact source bytes and reran all five tests successfully. All 643 algorithm library tests passed on the initial integrated source. Strict algorithm library Clippy passed. Ten source syntax projections were generated twice and byte-compared before installation.

No benchmark was run and no speedup is claimed. Tests are finite observations; no universal convergence, allocation-failure, forward-error or generic percentage guarantee. No executable approximation profiles are declared. Source raw code remains outside the sidecar.

## History

### 2026-10-30 · validation

Heading `### Validation — 2026-10-30` stays in validation as `### Validation`.

Source SHA256: cebfe82bb2b0e432f5482492abb0294df2417a802e1094c2fbaa7d151622519e.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- fista_lasso-debug-1.log: 9169929ae35506c207790e8bc858ffa3aa5448f4209c7ad3817ad874c3aea05c

- fista_lasso-debug-2.log: 76b00c4833da1db81736a2563f6e792ed159a4cd3f2b6216610a019dd27d29dc

- fista_lasso-release-1.log: 6978e7c5fab3a18a504af254904d0fd5b4a9df8ecbfe14cce150d51919fc9670

- fista_lasso-release-2.log: 343c9c73154c3540db8ada2d6a45779c26a296e9864e4ed48a00715ab9becca7

- fista_lasso-mutant.log: dbf4d04739c0793db20442cf43a709cd89e1d4a12a0c184d13fafc61fc30dabd

- fista_lasso-restored.log: ec15b309edeb5b4e10b750349c33c8389b776f7726f948099268b8e25b9d614f

- fista_lasso-normalized.txt: b936691836da64f8ce919a1c7209460a397b81427d41f3d40360ed6e16d64e4a
