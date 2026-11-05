# approximation-interpolation

## model

# approximation-interpolation

## forward

Evaluate interpolation polynomials and shape-preserving cubics, Bernstein curves and bounded quadrature.

## invariant

Node domain/spacing and basis coordinates remain explicit; no extrapolation for interpolators; quadrature errors and statuses remain distinct from true-integral certificates.

## cost

Barycentric O(n²) evaluation, monotone cubic O(n), de Casteljau O(n²); Gauss<=5 callbacks; Romberg<=20 levels and explicit evaluation budget. Finite allocation/stack limits retained.

## composition

A fixed polynomial can be represented by Lagrange nodes and Bernstein controls; quadrature integrates resulting callback. Affine monotone cubic equals the line, providing an independent exact real integral.

## epoch-memory

Wave20. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["barycentric_bezier_and_gauss_join_polynomial_identity","monotone_cubic_romberg_and_gauss_join_affine_integral"]

## limitations

Polynomial/basis identities are real arithmetic. Shape preservation does not bound error against unknown original function. Romberg diagonal differences can miss unsampled structure and are heuristic.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

For the real target, exclude a Lagrange polynomial failing its nodal equations, a Bernstein evaluation outside the control convex hull for t in[0,1], and shape-preserving cubic segments with overshoot contrary to their slope construction. Conditional Gauss polynomial exactness excludes wrong integrals only for the stated degree premise in real arithmetic. Native f64 results are not required to satisfy those identities bit-for-bit: their complement contains wrong rounded recurrences, wrong basis/interval coordinates, incorrect knot return values, excess callbacks or inaccurate status bookkeeping. Romberg estimated_error=0 does not exclude a nonzero true integral error; unknown off-sample function behavior remains allowed.

## reverse-reconstruction

Type: reverse_reconstruction.

At fixed distinct nodes and fixed degree<n, a complete set of real nodal ordinates identifies the interpolation polynomial, but one evaluated ordinate leaves an affine coefficient fiber. At Bernstein t=0 only the first control is observed and all later controls remain free; at interior t, perturbations of controls whose Bernstein-weighted sum is zero retain the value. Monotone sample data do not identify the true between-node function. For quadrature samples x_j, f and f+h with h(x_j)=0 have the same sampled execution but may have different integrals; h can be a scaled squared product of (x-x_j). Native rounded equality may enlarge these fibers, and callback budget/status do not restore unsampled information.

## basis-change

Type: polynomial_basis_transport.

Degree2 controls [1,1.5,4] and nodes (0,1),(.5,2),(1,4) both denote1+x+2x². Fixed-degree complete basis data is invertible in real arithmetic; finite scalar observations admit many arbitrary functions.

## quadrature-exactness

Type: conditional_polynomial_integral.

Order n Gauss integrates degree<=2n-1 polynomials exactly in real arithmetic. Native constants and arithmetic introduce roundoff; fixtures use additive1e-12 and do not certify all scales.

## shape-versus-error

Type: shape_information_loss.

Monotone sampled ordinates constrain interpolant shape but leave unknown between-node true functions ambiguous. Equal integral/point observations also retain many control sets or functions.

## wave20-evidence

### Family evidence

This family binds ten implementation/test sources. Named joins ["barycentric_bezier_and_gauss_join_polynomial_identity","monotone_cubic_romberg_and_gauss_join_affine_integral"] in algorithms/tests/composition_wave20.rs pass four times within the 695-check algorithm suite.

Forward: Evaluate interpolation polynomials and shape-preserving cubics, Bernstein curves and bounded quadrature.

Invariant: Node domain/spacing and basis coordinates remain explicit; no extrapolation for interpolators; quadrature errors and statuses remain distinct from true-integral certificates.

Cost: Barycentric O(n²) evaluation, monotone cubic O(n), de Casteljau O(n²); Gauss<=5 callbacks; Romberg<=20 levels and explicit evaluation budget. Finite allocation/stack limits retained.

Assumptions: Polynomial/basis identities are real arithmetic. Shape preservation does not bound error against unknown original function. Romberg diagonal differences can miss unsampled structure and are heuristic.

[Integrated evidence](../algorithm-library.groupant/model.md#wave20-evidence) records actual receipts. Leaf oracles remain separate from cross-implementation agreement; family membership cannot verify unchecked callback premises or a shared error budget.

## History

### 2026-10-30 · epoch-memory

Wave20, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave20-evidence

Heading `### Family evidence — 2026-10-30` stays in wave20-evidence as `### Family evidence`.

### 2026-11-01 · shape-versus-error

The Bernstein test child observes only its actual calls. A 2026-11-01 Rust probe found that a singleton-only mutant at `([42], 0.5)` agrees on all 361 call occurrences but differs from the production API at that witness. Consequently, composing this child's pass result with the interpolation parent cannot identify the full evaluator or exclude that unsampled fault. This is distinct from the parent’s control-vector preimage at a fixed evaluation point; neither fiber supplies a machine error bound.
