# math-poly-inequalities

## descriptant

### math-poly-inequalities

solve_ineq collects approximate critical points, samples intervals and merges through accepted endpoints. dens preserves canceled holes. Many functions share interval output. Nonfinite sort, close roots and Ne exact-check mismatch limit guarantees; count root work plus per-interval evaluations.

## approximant

### Scoped forward and cost

solve_ineq collects approximate critical points, samples intervals and merges through accepted endpoints. dens preserves canceled holes. Many functions share interval output. Nonfinite sort, close roots and Ne exact-check mismatch limit guarantees; count root work plus per-interval evaluations.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

solve_ineq collects approximate critical points, samples intervals and merges through accepted endpoints. dens preserves canceled holes. Many functions share interval output. Nonfinite sort, close roots and Ne exact-check mismatch limit guarantees; count root work plus per-interval evaluations.

Source range[22090,25985) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

A sample where reduced denominator or any retained dens evaluates zero excludes inclusion. An interval sample failing requested sign excludes the corresponding open interval from output; this is float-sample policy, not an exact theorem.

## reverse_reconstruction

Output(-2,2) for Lt is compatible with x²-4 and positive multiples, among other rational functions. Interval text cannot recover coefficients, multiplicities or original canceled factors.

## critical_merge

Gather real roots from numerator and product of all denominator factors; dedup within tolerance1e-12*max(|v|,1) only when numerator flag matches. Same coordinate from both sides can remain duplicated.

## interior_choice

Unbounded intervals use boundary±1 or0; bounded intervals use midpoint. Convert sample f64 to simplest Q and check exact num/den sign, while actual inside also tests dens in f64.

## operator_mismatch

holds supports Ne as v!=0. Exact interior check handles Lt/Le/Gt/Ge, otherwise v==0, so Ne diagnostics can disagree even when floating interval membership follows Ne.

## endpoint_join

Merge neighboring inside intervals only if holds at shared endpoint is true; otherwise emit open/closed boundary or isolated point. Float rounding, collapsed roots and NaN sorting can invalidate exact endpoint claims.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

solve_ineq collects approximate critical points, samples intervals and merges through accepted endpoints. dens preserves canceled holes. Many functions share interval output. Nonfinite sort, close roots and Ne exact-check mismatch limit guarantees; count root work plus per-interval evaluations.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.
