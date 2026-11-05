# algorithm-barzilai-borwein-tests

## descriptant

### barzilai_borwein — tests.rs

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

Report.x with Euclidean gradient residual ||Dx-b||2, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in ../mod.rs.groupant/model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

BB history retains previous x. Step uses s.s/(s.Ds), clipped to [1/L,1/mu]; sy=0 falls back to 1/L. Floating underflow can trigger fallback. The clipped spectral step is distinct from fixed gradient descent.

Dependencies: gradient_descent_quadratic public Error/Report/Status only; original iteration and checked buffers local.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

Tests call barzilai_borwein and compare against independent scalar closed forms, hand-computed recurrence iterates, report recomputation, and boundary relations. Finite deterministic assertions only, no sampling from held-out data. For a production call, O(n*(k+1)) time and O(n) storage, with two checked vector allocations and no per-iteration allocation. Test fixture dimensions are bounded by the documented API; the grid uses a 1000-step budget.

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

The exact_recurrence test checks hand-derived one- and two-step quadratic iterates. The receipt records one compiled recurrence mutant rejected by that test; it does not establish sensitivity to every recurrence mutation. Compilation failure alone is not a killed mutant. Passing identifies a finite observation, not proof of convergence.

## boundary_partition
Tests partition invalid shape/nonfinite operands/limits, accepted exact bounds, zero budget at convergence equality and above tolerance, dimension limits and a small-curvature case. Success of the harness excludes those asserted violations on these fixtures only; untested resource exhaustion remains unknown.

## tests

### Executable checks

cargo test -p mova-algorithms barzilai_borwein::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## validation

### Validation

Five tests passed twice in debug and twice in release, all four normalized named-test result lists identical. Production mutant compiled and failed the named exact_recurrence assertion; restored exact source bytes and reran all five tests successfully. All 643 algorithm library tests passed on the initial integrated source. Strict algorithm library Clippy passed. Ten source syntax projections were generated twice and byte-compared before installation.

No benchmark was run and no speedup is claimed. Tests are finite observations; no universal convergence, allocation-failure, forward-error or generic percentage guarantee. No executable approximation profiles are declared. Source raw code remains outside the sidecar.

## History

### 2026-10-30 · mutation_sensitivity

The exact_recurrence test checks hand-derived one- and two-step quadratic iterates. The 2026-10-30 receipt records one compiled recurrence mutant rejected by that test; it does not establish sensitivity to every recurrence mutation. Compilation failure alone is not a killed mutant. Passing identifies a finite observation, not proof of convergence.

### 2026-10-30 · validation

Heading `### Validation — 2026-10-30` stays in validation as `### Validation`.

Source SHA256: 778c6352ec13941ccc139ea8d2a2e1ff1b67c01c397ec2747f5990264c1b99d2.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- barzilai_borwein-debug-1.log: 4efc9accc1d0bff2a9598f50cae00b1dc1ef21ed54b5ff87b6980f6689989ace

- barzilai_borwein-debug-2.log: 0c6dd1404baef85dfbb523459a4eb8782190a840fa44535aece91f2dc8ba4933

- barzilai_borwein-release-1.log: 24da6fc861f03d88a0c67e1fcfd99b3dfa2b99910525eb02aaa69283d16c130b

- barzilai_borwein-release-2.log: 507c634af37925921acb8fea277316953d2b3af96ead8ea84a2861a28ad294ca

- barzilai_borwein-mutant.log: 08d1a3979c57188f6ee1fbdfc3d5fc45f6517f0ae5b5afb05f08a0736b47e29a

- barzilai_borwein-restored.log: 16e19fcfe2dddd4a901514f86bc5e50dd8cb90429622bcbc4484672b9d1ec5ae

- barzilai_borwein-normalized.txt: 8c7837b86732faafdae45eb17e5b23be009e23cf12eb963d8d9e036aa4f3d41f

### 2026-11-01 · reverse_reconstruction

On 2026-11-01 an external wrapper delegated to the actual API except for the successful scalar call D=[3], b=[3], x0=[0], tolerance=1e-12, budget=1, where it replaced x=[1] by x=[0]. All five existing tests passed against that wrapper; a separate scalar optimum assertion rejected it. This deliberately artificial mutation demonstrates a transcript fiber, not a likely defect or an exhaustive mutation analysis. Its report is internally inconsistent, illustrating that the harness does not check every valid input.

### 2026-11-01 · validation

#### Bounded transcript review — 2026-11-01

An external Rust harness compiled the actual production module and reused this exact test file against an artificial delegating wrapper. All five tests passed despite the wrapper changing the returned coordinate on an untested scalar call; a separate closed-form assertion failed on that wrapper. The actual API returns the expected scalar optimum. The witness demonstrates non-unique reconstruction from the test outcomes only. It does not model all Rust implementations or prove floating-point convergence. Detailed harness and logs remain in the cycle run directory.
