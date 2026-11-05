# math-poly

## descriptant

### poly

Poly stores ascending Q coefficients, zero empty; constructors/operators trim. divrem cancels leading terms, gcd is monic Euclid. ratfunc combines reduced fractions while dens records encountered divisor numerators before cancellation; powers require target-independent integer |k|<=200. Yun squarefree stops after multiplicity400; solve_poly extracts rational candidates, exact quadratic surds after discriminant factorization, otherwise f64 Aberth (<=500 sweeps), with substitution/completeness/rebuild checks. solve_ineq gathers real critical points, samples float intervals, compares exact rational interior signs and emits intervals.

## approximant

### Forward and cost

Poly stores ascending Q coefficients, zero empty; constructors/operators trim. divrem cancels leading terms, gcd is monic Euclid. ratfunc combines reduced fractions while dens records encountered divisor numerators before cancellation; powers require target-independent integer |k|<=200. Yun squarefree stops after multiplicity400; solve_poly extracts rational candidates, exact quadratic surds after discriminant factorization, otherwise f64 Aberth (<=500 sweeps), with substitution/completeness/rebuild checks. solve_ineq gathers real critical points, samples float intervals, compares exact rational interior signs and emits intervals.

Dense multiply O(ab), long division O((a-b+1)b) rational operations; gcd depends on degree drops and coefficient widths, pow repeats multiplication. Rational conversion can grow degrees/widths. Root candidates enumerate divisor products or numeric candidates with Horner tests; nt factor costs compose. Aberth <=500 O(d²) sweeps/O(d) storage. Inequalities add denominator products, root work, sorting and samples.

No new executable Rust surrogate: exact values, branching, domains and effects require an observation-specific metric before supported 1/2/5/10% profiles. Existing floating diagnostics are not such a profile; no speed claim.

## invariant

### Domain and invariant

Public Poly(Vec) can violate normalization; divrem asserts nonzero d. Negative powers can retain zero denominator; i64 MIN abs and dimensions have unchecked extremes. Surd operations assume common d. Numeric roots may fail convergence or convert nonfinite without error; solve_poly returns possibly false checks. Inequality sort can panic on NaN, close roots merge; Ne exact-check fallback tests equality, so not a general proof. Adequate stack/allocation and normalized inputs required.

Poly stores ascending Q coefficients, zero empty; constructors/operators trim. divrem cancels leading terms, gcd is monic Euclid. ratfunc combines reduced fractions while dens records encountered divisor numerators before cancellation; powers require target-independent integer |k|<=200. Yun squarefree stops after multiplicity400; solve_poly extracts rational candidates, exact quadratic surds after discriminant factorization, otherwise f64 Aberth (<=500 sweeps), with substitution/completeness/rebuild checks. solve_ineq gathers real critical points, samples float intervals, compares exact rational interior signs and emits intervals.

Known divisor plus quotient/remainder reconstructs dividend uniquely as q*d+r on normalized Q[x]. Roots alone omit leading scalar and rational-domain holes; approximate roots specify neighborhoods, not exact inverse. num/den loses canceled holes; dens restores exclusions but not original syntax. Factors plus lead reconstruct polynomial if rebuild check passes. Many rational functions share one interval set.

## composition

### Composition

Poly stores ascending Q coefficients, zero empty; constructors/operators trim. divrem cancels leading terms, gcd is monic Euclid. ratfunc combines reduced fractions while dens records encountered divisor numerators before cancellation; powers require target-independent integer |k|<=200. Yun squarefree stops after multiplicity400; solve_poly extracts rational candidates, exact quadratic surds after discriminant factorization, otherwise f64 Aberth (<=500 sweeps), with substitution/completeness/rebuild checks. solve_ineq gathers real critical points, samples float intervals, compares exact rational interior signs and emits intervals.

calc dispatches to comb/nt. poly/linsys consume calc E and stateful Env for constants; deriv consumes E and calc::approx. fcheck consumes separate legacy expr::Expr/units, with no implicit lossless E adapter. Q/Z joins use big. Parent symbolic-computation models these edges; local checks do not certify application wiring. Nested scopes reuse parent syntax, and remaining helpers/types/tests are parent-modeled.

## models

# Typed models

## behavior_complement

For fixed admitted input/state, exclude outputs and effect traces contradicting this actual relation: Poly stores ascending Q coefficients, zero empty; constructors/operators trim. divrem cancels leading terms, gcd is monic Euclid. ratfunc combines reduced fractions while dens records encountered divisor numerators before cancellation; powers require target-independent integer |k|<=200. Yun squarefree stops after multiplicity400; solve_poly extracts rational candidates, exact quadratic surds after discriminant factorization, otherwise f64 Aberth (<=500 sweeps), with substitution/completeness/rebuild checks. solve_ineq gathers real critical points, samples float intervals, compares exact rational interior signs and emits intervals. These exclusions are conditional on the stated domain. Unavailable checks, passing residuals and resource limits do not establish global correctness.

## reverse_reconstruction

Known divisor plus quotient/remainder reconstructs dividend uniquely as q*d+r on normalized Q[x]. Roots alone omit leading scalar and rational-domain holes; approximate roots specify neighborhoods, not exact inverse. num/den loses canceled holes; dens restores exclusions but not original syntax. Factors plus lead reconstruct polynomial if rebuild check passes. Many rational functions share one interval set.

## normalization

Empty coefficients mean zero; trim removes trailing exact zeros. Public tuple states require caller normalization.

## domain_holes

Division records nonconstant divisor numerator before reduction; (x²-1)/(x-1) still excludes1. solve_poly of numerator alone does not enforce holes.

## root_evidence

Exact rational/surd substitution differs from residual<=1e-8 max(scale,1). Multiplicity counts/rebuild do not certify conditioning or original domain.

## interval_sampling

Float endpoints dedup by tolerance and numerator flag; holds checks retained dens, exact interior check omits them and Ne uses equality fallback. No certified enclosure.

## tests

### Actual-code evidence

Each selected named test passed twice debug/twice release, sorted outcomes identical:

- `test poly::tests::factor_and_ineq ... ok`
- `test poly::tests::rational_equation_excludes_holes ... ok`
- `test poly::tests::solve_quadratics_and_more ... ok`
- `test poly::wave28::division_and_canceled_hole ... ok`

Finite synthetic fixtures only; no teacher/API/corpus/GPU/heldout. No allocation/stack faults, huge sizes, deadline proof, or universal numerical bound.

Final validation: eight compiled mutants total (seven module boundaries plus linsys width guard), all compile 0 / test 101 and exact restoration / positive 0. Old-code count mismatch witness compile 0 / test 101 separately. Full-registry structural run reports only ancestor integration bindings; isolated source snapshots validate each owned sidecar. Strict Clippy: dependency invocation blocked in existing global build script; --no-deps reaches math and reports 113 existing library findings, retained externally, no clean lint claim. Added test blocks formatted independently; production layout preserved.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read before edits, bounded actual-code tests and source controls. Linsys shape defect was witnessed on old code and repaired; other semantics retained. External receipt name octopus-wave28-mathcore-2026-10-31 includes prospective.md. Registration is partial, not verification.

### 2026-10-31 · tests

Heading `### Actual-code evidence 2026-10-31` stays in tests as `### Actual-code evidence`.

Compiled source control: control-poly.patch.txt; compiler exit0, test nonzero, exact source restoration and focused positive exit0. Full logs and numeric exits retained externally in octopus-wave28-mathcore-2026-10-31. Linsys also retains shape-old compile 0 / test 101 witness before shape repair. Seven module controls plus the additional linsys width-guard control; fragment declarations are not extra tests.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
