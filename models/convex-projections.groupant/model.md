# convex-projections

## model

# convex-projections

## forward

Project finite vectors onto monotone, simplex and L1-ball constraints and apply the L1 proximal soft-threshold map.

## invariant

Real maps have unique minimizers and KKT conditions; native f64 routines preserve explicit validation and acceptance tolerances rather than universal exact feasibility.

## cost

Unit-weight PAVA and soft threshold O(n) work/storage; sorted simplex and L1 projection O(n log n) work/O(n) storage. Length <=10^6 with fallible allocation.

## composition

Outside the L1 ball, its projection equals signed simplex projection of magnitudes and soft threshold at the common active magnitude difference. Isotonic regression preserves total sum in real arithmetic; finite checks separately test residual prefixes. Residual-prefix evidence is in the leaf partition oracle; the parent harness checks totals, order, idempotence and selected normal-cone inequalities.

## epoch-memory

Wave19. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["projections_join_threshold_reconstruction_and_mean"]

## limitations

Real normal-cone and KKT laws do not by themselves prove floating-point outputs. Simplex relative mass residual acceptance is <=64*n*EPSILON, not objective error. L1 may exceed radius within tolerance. Overflow/cancellation can reject mathematically solvable inputs.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

At the real-model level exclude KKT-incompatible outputs for the tagged convex set/proximal problem. At the native level exclude different rounded transitions and invalid accepted results; arithmetic/allocation errors are retained, and tolerance-sized feasibility deviations are not silently discarded.

## reverse-reconstruction

Type: reverse_reconstruction.

For fixed convex set C and output y in C, compatible real inputs are y+N_C(y). Soft-threshold zero coordinate has input fiber [-lambda,lambda]; nonzero coordinate reconstructs x_i=y_i+lambda*sign(y_i). This many-to-one model does not uniquely invert rounded implementations.

## signed-simplex

Type: projection_transport.

When ||x||1>r, projection onto the L1 ball is sign(x)*P_simplex(abs(x),r) in real arithmetic. On the interior the identity branch applies instead. Floating mass residual is bounded only by the leaf acceptance condition.

## threshold-kkt

Type: proximal_kkt_relation.

A shared nonnegative threshold lambda satisfies y_i=sign(x_i)*max(abs(x_i)-lambda,0), with sum(abs(y))=r on an active L1 boundary. A common threshold witness links simplex and soft-threshold fixtures, with explicit additive numerical tolerance.

## isotonic-residual

Type: order_cone_certificate.

For real unit-weight isotonic regression, residual prefix sums sum(x-y) are nonnegative, total residual is zero, output is nondecreasing, and active block sums agree. These KKT observations are checked independently on bounded fixtures.

## wave19-evidence

### Family evidence

This family binds eight implementation/test sources through its manifest; views are partial source-derived models. Selected named joins are ["projections_join_threshold_reconstruction_and_mean"] in algorithms/tests/composition_wave19.rs. Those checks pass four times as part of the 616-check suite. Leaf oracles and compiled production controls remain separately bound in adjacent evidence.

[Integrated evidence](../algorithm-library.groupant/model.md#wave19-evidence) records receipts; conditional laws outside these fixture scopes are not promoted to executed checks. No joint universal percentage bound follows from family membership.

## History

### 2026-10-30 · epoch-memory

Wave19, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave19-evidence

Heading `### Family evidence — 2026-10-30` stays in wave19-evidence as `### Family evidence`.
