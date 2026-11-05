# algorithm-composition-wave20

## model

# Wave20 native joins

## forward

Eight production joins cover all twenty new modules. Factor solves, normal equations, scalar directions, thresholding, polynomial bases, integrated interpolants, attainable subset fibers and DP coordinates connect actual outputs and shared mathematical inputs. Independent exact oracles supplement implementation agreement.

## invariant

Tags preserve matrix objective sign, interval coordinates, basis conventions, item identities and DP assumptions. Absolute tolerances apply only to named finite fixtures; no composition-wide percentage guarantee. No callback accesses external state except counted synthetic calls.

## cost

Eight bounded domains: n2 matrix solvers with<=10000 iterations; <=200 scalar iterations;33 polynomial/shape samples; quadrature budget10000; n<=8 exhaustive256 subsets; n<=12 independent cubic DP. All allocations/oracle work count; no runtime improvement claimed.

## composition

Each named test is a separate tagged relation. Composition requires matching coefficient conventions, finite domains, normalization metrics, probability mass or radius, observation coordinates and explicit tolerance assumptions. Error caps apply to their declared metric only; no uniform whole-pipeline percentage follows. Distinct tests do not constitute one end-to-end pipeline. Independent finite or literal oracles remain distinct from cross-implementation equality.

## epoch-memory

# Wave20 independent integration prospective

Eight bounded algorithm joins cover twenty production modules: SPD factor/triangular/iterative solve; QR least-squares transported to quadratic optimization; scalar Brent/Armijo; Lasso threshold solution; interpolation/Bezier/quadrature; monotone cubic with Romberg; subset sum transported to knapsack; totally-monotone/partition/BST DP oracles. Independent exact fixtures supplement cross-implementation agreement. Residual observations are not solution-error certificates on ill-conditioned systems; quadrature differences are not rigorous error bounds. Matrix optimizer domains and DP assumptions remain explicit.

Three synthetic qagen joins exercise real validation/gating/repair paths under deterministic injected lemma stubs. No teacher, model, corpus or database execution. Equal output rows do not recover original order or metadata.

Before measurement: compare bitset subset sum with existing boolean DP on identical deterministic bounded item/target fixtures, exact decision equality, two executions, seven alternating-order timing samples, black-box outputs and bounded repeats. Prediction: packed words reduce O(n*T) scalar work to O(n*ceil(T/64)); tiny cases may regress. Qagen stub call-count profiles compare valid/invalid anchor routes with same deterministic stub, predicting invalid inputs perform zero analysis and lawful behavior remains unchanged. Stub wall time is not model/service throughput. Root runs timings after harness review. No global speedup claim.

Validation: actual production joins and independent oracles; twice debug/release normalized outcomes; compiled semantic negative controls; exact source hashes retained by parent integration. External harness pending integration; worker sources read-only to reviewer.


## checks

8 named joins pass twice per profile; compiled adapter rejects, restoration passes and paired partial syntax matches. See model.md#evidence. Macro expansion and semantic AST remain absent.

## catalog

# Wave20 supplied-input relations

Source-derived specifications; bounded native evidence and repeated partial syntax are recorded in model.md#evidence. No general proof or complete inverse engine is claimed.

## behavior-complement

Type: behavior_complement. Named check: all named joins.

On the tagged bounded joins, incompatible factors, matrix/sign conventions, derivative directions, basis coefficients, anchor coordinates, unattainable objectives or DP recurrence minima lie outside the admissible relation. Invalid inputs and resource statuses are categorical. These fixtures do not exhaust the complement, prove floating accuracy, or verify caller DP assumptions on arbitrary callbacks.

## reverse-reconstruction

Type: reverse_reconstruction. Named check: all named joins.

Each output denotes all compatible antecedents: fixed nonsingular linear systems and fixed-degree full polynomial bases may be unique; optimizer values, integrals, subset objectives and DP minima generally admit many inputs/witnesses. Retained original coordinates establish compatibility, not an executable inverse or universal reconstruction.

## factor-solves

Type: factorization_residual_transport. Named check: factor_triangular_and_stationary_solvers_join_spd_system.

A=[[4,1],[1,3]], b=[6,7]. Cholesky lower factor feeds lower then transposed-upper triangular solves. Gauss-Seidel and Jacobi independently solve original A,b, zero initial, tolerance1e-11 and budget1000. All coordinates equal literal [1,2] within1e-9, both stationary statuses Converged, reconstruction residual<=1e-12. Fixed positive-definite well-conditioned fixture; residual is not a universal solution-error certificate.

## qr-quadratic

Type: normal_equation_optimizer_transport. Named check: qr_and_constrained_optimizers_join_known_quadratic.

Design [[1,0],[0,1],[1,1]], response [.25,.75,1], QR answer [.25,.75] within1e-12. Its normal equation Q=[[2,1],[1,2]], linear=[1.25,1.75] feeds box projected gradient and simplex Frank-Wolfe using objective .5*xQx-linear*x. Both within1e-4 of QR, budget10000/tol1e-9. Box [0,1]^2, simplex unit mass. Q strictly diagonally dominant and PSD. Equality on this unique minimizer does not certify arbitrary rank-deficient systems.

## scalar-search

Type: directional_derivative_step_relation. Named check: brent_and_armijo_join_scalar_quadratic.

Brent minimizes (x-2)^2 on [-4,5] tol1e-9 budget200; estimate within1e-6 of2. Armijo along x=4t uses value0=4, slope=-16, initial1, contraction.5,c=.1,budget30. Accepted step .5 maps to Brent minimizer; exact call count equals report, sufficient decrease independently checked. Callback pure except counted observations; no general global minimizer claim.

## lasso

Type: orthogonal_lasso_threshold_relation. Named check: lasso_and_triangular_identity_join_soft_threshold_solution.

Identity triangular solve transports response [3,-2] into Lasso identity design, lambda.5,tol1e-10,budget1000. Independent scalar threshold oracle [2.5,-1.5] holds within1e-8 for objective .5||Ax-b||²+lambda||x||1. Zeroed coordinates would have interval preimages; this fixture is nonzero. Not a generic objective/solution accuracy bound.

## polynomial

Type: basis_transport_quadrature_relation. Named check: barycentric_bezier_and_gauss_join_polynomial_identity.

Nodes (0,1),(.5,2),(1,4) and Bernstein controls [1,1.5,4] represent p=1+x+2x². At33 dyadic t in[0,1] compare each to literal polynomial within1e-12. Gauss order3 integrates barycentric callback on[0,1] to13/6 within1e-12. Basis transform at fixed degree is injective; arbitrary functions are not reconstructed from finite samples.

## monotone-quadrature

Type: shape_preserving_integral_transport. Named check: monotone_cubic_romberg_and_gauss_join_affine_integral.

Nodes (-1,-1),(0,2),(1,5) reproduce3x+2 at33 dyadic points in[-1,1], monotonicity and absolute1e-12. Romberg consumes cubic callback with tol1e-10, levels12,budget10000, statusConverged; both Romberg and Gauss order2 equal analytic integral4 within1e-10. Romberg successive differences are heuristic, not certified true integral error; nonaffine shape behavior needs separate leaf evidence.

## subset-knapsack

Type: integer_feasible_objective_fiber_relation. Named check: bitset_and_mitm_join_exhaustive_attainable_objectives.

n0..8, values(7i+3)%13, capacities0..32. Independent enumeration <=256 masks gives attainable sum set and feasible maximum. Bitset target decision equals set membership; meet-in-middle items weight=value, value=u64(value), max_states1024 equals exact maximum. Bitset max_words100. Value-only APIs do not reconstruct a unique mask; empty/zero cases preserve exact zero.

## structured-dp

Type: totally_monotone_coordinate_dp_transport. Named check: smawk_partition_and_knuth_join_independent_dp_oracles.

n1..12, SMAWK squared distance matrix returns identity leftmost minima. Those returned coordinates define partition squared interval lengths and positive BST weights (coordinate mod4)+1. Compare partition for every1..n layer to independent cubic DP and BST to independent cubic root enumeration; callback budget10000, BST cells10000. Squared-distance matrix is Monge; convex interval costs have monotone leftmost splits; nonnegative successful-key weights satisfy Knuth conditions. Caller assumptions are not validated for arbitrary callbacks; minima/objectives retain many compatible matrices/costs/trees.

## evidence

### Wave20 harness evidence

Forward: Eight production joins cover all twenty new modules. Factor solves, normal equations, scalar directions, thresholding, polynomial bases, integrated interpolants, attainable subset fibers and DP coordinates connect actual outputs and shared mathematical inputs. Independent exact oracles supplement implementation agreement.

Invariant: Tags preserve matrix objective sign, interval coordinates, basis conventions, item identities and DP assumptions. Absolute tolerances apply only to named finite fixtures; no composition-wide percentage guarantee. No callback accesses external state except counted synthetic calls.

[Native receipts](../../../models/algorithm-library.groupant/model.md#wave20-evidence) bind actual logs; finite observations do not prove arbitrary input behavior or inverse uniqueness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below; no speed or accuracy gain is claimed.

### 2026-10-30 · document

Heading `# Wave20 independent integration prospective — 2026-10-30` stays in document as `# Wave20 independent integration prospective`.

### Undated · evidence

8 named joins pass twice in debug and twice in release within the 695-check package suite. Corresponding compiled adapter mutation rejects by assertion, exact restoration passes and paired partial syntax matches. Source SHA-256 79314b1ffe5671cc4bd2afdccfdddc841a35571dea0b68a6901da13df79447bf.
