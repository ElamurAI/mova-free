# numerical-evaluation

## model

# numerical-evaluation

## forward

Evaluate power and Chebyshev polynomials, compensated finite sums and budgeted adaptive quadrature with a separately labeled heuristic error estimate.

## invariant

Keep coefficient conventions, finite results, callback evaluations, budgets and numerical assurance distinct. A successful heuristic tolerance status is not an analytical integration certificate.

## cost

Horner, Clenshaw and Neumaier O(n) arithmetic/O(1) scratch; Simpson O(evaluations) with bounded depth<=64 and fixed panel stack. Callback costs are additional.

## composition

Fixed-degree Chebyshev-to-power conversion and analytic polynomial integrals provide independent identities. The harness uses the cubic [2,3,-2,1] in Chebyshev form and [4,0,-4,4] in power form on 33 dyadic points. The separate quadratic formula is an analytical relation. Native comparisons use explicit additive tolerances.

## epoch-memory

Wave19. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["polynomial_basis_join_derivative_and_quadrature","rational_certificates_join_polynomials_and_compensated_products"]

## limitations

Finite intermediate overflow and cancellation are explicit. Simpson's local estimate can miss unsampled structure; callbacks may violate smoothness. Neither residual samples nor finite successful tests certify all functions or a shared <=10% error.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude outputs outside each tagged native transition, successful nonfinite values, excess callback budgets and Simpson estimates promoted to universal guarantees. Distinct error/status observations are not numerical approximation loss.

## reverse-reconstruction

Type: reverse_reconstruction.

At x=0 Horner retains c0,c1 and loses higher coefficients; Clenshaw retains an alternating even-coefficient sum and loses odd terms. Sum fibers include [1,2] and [3]. Quadrature samples permit arbitrary off-sample perturbations, so reports do not identify a unique integrand.

## basis-conversion

Type: polynomial_basis_transport.

Chebyshev coefficients [a,b,c] represent power coefficients [a-c,b,2c]. Evaluations agree in real arithmetic. Floating tests require scale-aware additive tolerance; derivative comparison must use transformed coefficients.

## analytic-integral

Type: antiderivative_certificate.

A power polynomial has exact real integral sum a_k*(b^(k+1)-a^(k+1))/(k+1). This supplies an independent finite quadrature oracle; general callback error remains heuristic.

## sample-blindness

Type: finite_observation_complement.

A function vanishing at all sampled abscissae can have a nonzero integral between them. Equal sampled values and error estimates therefore admit different exact integrals; an explicit hidden-bump test witnesses this gap. The witness is in the leaf validation_and_unsampled_fiber test.

## wave19-evidence

### Family evidence

This family binds eight implementation/test sources through its manifest; views are partial source-derived models. Selected named joins are ["polynomial_basis_join_derivative_and_quadrature","rational_certificates_join_polynomials_and_compensated_products"] in algorithms/tests/composition_wave19.rs. Those checks pass four times as part of the 616-check suite. Leaf oracles and compiled production controls remain separately bound in adjacent evidence.

[Integrated evidence](../algorithm-library.groupant/model.md#wave19-evidence) records receipts; conditional laws outside these fixture scopes are not promoted to executed checks. No joint universal percentage bound follows from family membership.

## History

### 2026-10-30 · epoch-memory

Wave19, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave19-evidence

Heading `### Family evidence — 2026-10-30` stays in wave19-evidence as `### Family evidence`.
