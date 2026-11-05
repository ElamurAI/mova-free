# algorithm-nesterov-quadratic-tests

## descriptant

### nesterov_quadratic — tests.rs

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

Report.x with Euclidean gradient residual ||Dx-b||2, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in ../mod.rs.groupant/model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

Nesterov state is (x,previous). Extrapolate before evaluating gradient: y=x+beta*(x-previous), beta=(sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)); then x'=y-(Dy-b)/L. Evaluating gradient at old x would produce a different second iterate.

Dependencies: gradient_descent_quadratic public Error/Report/Status only; original iteration and checked buffers local.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

Tests call nesterov_quadratic and compare against independent scalar closed forms, hand-computed recurrence iterates, report recomputation, and boundary relations. Finite deterministic assertions only, no sampling from held-out data. O(n*(k+1)) time and O(n) storage. Two checked vector allocations, no per-iteration allocation. Test fixture dimensions bounded by the documented API, with small oracle grids and capped 3000-step solves.

Tolerance is absolute, finite and positive; budget 0..100000. All index reductions use deterministic ascending order. Real equations are models, not f64 accuracy proofs. No generic percentage guarantee. Executable 1/2/5/10% surrogates are unsupported pending a useful metric and evidence.

## models

# Typed obernant family

## behavior_complement
For the test harness, a passing result is excluded if any named fixture yields a mismatch outside its explicit 1e-8 comparison tolerance or violates an exact status/error assertion. A noncompiling mutant is excluded from the set of successful mutation controls. The concrete production relations under test are:

For fixed valid input, a successful report with x length different from D length, nonfinite x/residual, iterations>budget, or Converged while its recomputed gradient norm>tolerance is excluded. For zero budget the only compatible x is x0; for invalid shape any Ok result is excluded.

## reverse_reconstruction
A passing test list has many compatible implementations: an implementation differing only on untested valid inputs can produce the same list. Thus reverse reconstruction yields an equivalence class over these finite fixtures, never unique source recovery. The production preimage ambiguities exercised or documented by those fixtures are:

At budget zero, output x uniquely recovers x0, but not D,b,tolerance. For any positive diagonal D and chosen z, b=Dz with x0=z gives converged z in zero updates, so infinitely many compatible objectives yield one solution. Recovering previous momentum states from only x is ambiguous; even with D,b known a converged status erases trajectory length choices. A single BB gradient step on scalar D at 1/D maps every initial point to b/D in exact arithmetic and is noninjective.

## independent_oracle
The 27-point grid compares against exact analytic x_i=b_i/d_i, independent of iterative trajectory. exact_recurrence has rational expected iterates for D=(1,4), b=(1,4), x0=0. residual_is_not_solution_error uses d=1e-6 to demonstrate that convergence status cannot identify coordinate accuracy.

## mutation_sensitivity
The exact_recurrence test checks a hand-derived two-step quadratic or three-step LASSO result. A compiled production recurrence mutant must fail that named test. Compilation failure alone is not a killed mutant. Passing identifies a finite observation, not proof of convergence.

## boundary_partition
Tests partition invalid shape/nonfinite operands/limits, accepted exact bounds, zero budget at convergence equality and above tolerance, dimension limits and zero-design or small-curvature cases. Success of the harness excludes those asserted violations on these fixtures only; untested resource exhaustion remains unknown.

## tests

### Executable checks

cargo test -p mova-algorithms nesterov_quadratic::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## validation

### Validation

Five tests passed twice in debug and twice in release, all four normalized named-test result lists identical. Production mutant compiled and failed the named exact_recurrence assertion; restored exact source bytes and reran all five tests successfully. All 643 algorithm library tests passed on the initial integrated source. Strict algorithm library Clippy passed. Ten source syntax projections were generated twice and byte-compared before installation.

No benchmark was run and no speedup is claimed. Tests are finite observations; no universal convergence, allocation-failure, forward-error or generic percentage guarantee. No executable approximation profiles are declared. Source raw code remains outside the sidecar.

## History

### 2026-10-30 · validation

Heading `### Validation — 2026-10-30` stays in validation as `### Validation`.

Source SHA256: 80b13b7f45ec6b1a35d922d6086167f353e82fd8bf2f28c9d83ca12d0fa752e8.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- nesterov_quadratic-debug-1.log: b1f767a559e4990fab5a34370af705740baefdb95a7882b86e722af06617a349

- nesterov_quadratic-debug-2.log: 563d544b576f83be8b5180be69c25e85f87728a9887a396407c7e6bcf665829e

- nesterov_quadratic-release-1.log: 64c62ed9255d9b8c3acb624be75a0e6973341939554f380276bdf6c0c4f0fa50

- nesterov_quadratic-release-2.log: 105a8ffffbc95c988b5c299740ac44a28f33718c94c1d9c3ced5ae8c11640914

- nesterov_quadratic-mutant.log: b9441b64ba1801e71b346bb5aa16154bb5594ede1b8028383a139026e33469a3

- nesterov_quadratic-restored.log: a323d0f6adbc6164a258539e4c614176baa18e9c1e8f1013d42b084d6ce83854

- nesterov_quadratic-normalized.txt: 577b1039cdfb275dcc313912e10266b7cafed2534aede5a8231dadd15f60aa10
