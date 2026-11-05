# interpolation-quadrature

## model

# interpolation-quadrature

## forward

Evaluate nodal interpolation via Neville and reusable Newton coefficients, construct/evaluate natural cubic splines, and integrate callbacks with global dyadic trapezoid or fixed-window tanh-sinh refinement. Numeric values, estimators, statuses and callback traces remain distinct observations.

## invariant

Interpolation preserves its specified node/basis recurrence; spline uses zero endpoint second derivatives and closed-domain queries. Quadrature never exceeds caller budget, commits only complete levels, and labels consecutive-level differences as estimates rather than true-error bounds.

## cost

Neville O(n²) per query/O(n) space; Newton O(n²) build then O(n) per query/O(n) space; natural spline O(n) build/storage and O(log n) query. Both quadratures O(evaluations) arithmetic work plus geometry checks/O(1) space, excluding callback cost; tanh-sinh uses transcendental transforms and recomputes all level samples. Interpolation has no hard count cap beyond finite allocated input; quadrature uses usize budget with checked doubling.

## composition

All five are std-only with no production intermodule dependencies. Neville and Newton share the unique degree<n real interpolation polynomial for identical nodes/ordinates. Natural spline shares affine data but is generally a different function for curved data. A validated interpolator can be wrapped as a deterministic quadrature callback only on a compatible query interval and with explicit handling of its Result errors.

## epoch-memory

Wave25. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

Named joins: interpolation_polynomial_and_spline_affine_join, quadrature_analytic_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## limitations

Polynomial and spline identities concern real arithmetic, not bitwise f64 agreement. Conditioning, cancellation and unsampled behavior remain unbounded. Deterministic callback is required to interpret repeated samples as one function; callback panic/resource cost and allocator failure are outside Result contracts. Tanh-sinh window [-3,3] omits tails.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude invalid node/order success, wrong rounded divided-difference or triangular recurrence, nonfinite successful results, spline extrapolation success and wrong knot value. For quadrature exclude calls on invalid input, calls beyond budget, partial-level estimates presented as complete, or an EstimatedTolerance status inconsistent with the computed consecutive-level criterion. Real polynomial reproduction does not require bitwise equality across distinct f64 algorithms.

## reverse-reconstruction

Type: reverse_reconstruction.

At fixed distinct nodes, complete real Newton coefficients identify the nodal polynomial; one query value leaves a large affine ordinate fiber. At q=0, functions x and0 both yield0. Natural curvature data alone loses the affine component. For quadrature a callback perturbation vanishing on every visited point gives the same execution while potentially changing the integral; rounded observations may merge additional inputs.

## polynomial-basis-join

Type: conditional_basis_equivalence.

Neville and Newton represent the same unique real polynomial of degree<n from common samples. Their traversal differs: Neville updates ascending rolling widths; Newton builds descending divided-difference columns and evaluates by nested products. Comparisons require finite scale-aware tolerance, not exact equality; extrapolation is admitted but may amplify error.

## spline-join-boundary

Type: rejected_interpolant_equivalence.

Affine samples yield the same real affine function for polynomial interpolation and natural spline; generic polynomial data do not. Natural endpoint curvature0 usually differs from the interpolating polynomial's second derivative, so agreement at all knots alone cannot justify substituting one callback for the other between knots.

## callback-adapter

Type: typed_error_transport.

To integrate a stored spline, [a,b] must lie in its closed knot interval; Newton only requires finite query but may fail arithmetic. An adapter must preserve failure through an explicit error/side channel; unwrap panic or mapping an error to0 changes semantics. Quadrature itself only recognizes nonfinite callback values and does not know the interpolator error type.

## trapezoid-trace

Type: sample_reuse_relation.

Trapezoid calls a then b, followed by each new midpoint left-to-right exactly once. A full next level must fit budget before geometry/callbacks. Initial budget2 returns estimated_error=None; a five-call trace can comprise2+1+2 samples. Geometry collision returns Stagnation before that level calls.

## tanh-sinh-trace

Type: truncated_transform_relation.

x(t)=mid+half*tanh(pi/2*sinh(t)), t in[-3,3]; transformed endpoints are interior x, half weighted, with7,13,25,... calls per level and no sample reuse. Strictly increasing interior geometry is checked first. Before any complete level, a Stagnation report's value0/None is a sentinel, not a computed integral.

## estimator-rejection

Type: unsampled_information_loss.

Trapezoid |Tnew-Told|/3 is a smooth asymptotic heuristic; tanh-sinh |Qnew-Qold| excludes truncation tails. A zero callback and a bump outside finite sampled points can have identical zero estimators but different integrals. Neither estimator composes into a generic1/2/5/10% accuracy bound or certifies interpolation error against an unknown function.

## wave25-evidence

# interpolation-quadrature: wave25 evidence

## checks (wave25-evidence)

Named joins: interpolation_polynomial_and_spline_affine_join, quadrature_analytic_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## History

### 2026-10-30 · epoch-memory

Wave25, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · checks

2026-10-30: this family adds 15 source-local tests and 2 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.

### 2026-10-30 · checks (wave25-evidence)

2026-10-30: this family adds 15 source-local tests and 2 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.
