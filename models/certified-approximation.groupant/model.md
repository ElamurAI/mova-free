# certified-approximation

## model

# certified-approximation

## forward

Compute bounded-loss feasible objectives, exact rational interpolation and quantized integer dot observations with distinct certificates.

## invariant

Feasibility, objective loss, conditional interpolation error and dot absolute/L1 error remain separately typed. Unsupported percentages and resource/arithmetic errors are categorical.

## cost

Knapsack value scaling O(n^3/epsilon); trimmed subset list O(n*L*log L), L=O(n*log(target)/epsilon) in the mathematical model. Interpolation validates O(n) then searches O(log n); dot O(n). Work and allocation limits apply.

## composition

Embedding subset values as equal knapsack weights/profits preserves the exact optimum. The two approximate outputs may differ while satisfying their own loss bounds. Conditional absolute interpolation error transports through a known Lipschitz map; dot L1 normalization does not imply relative scalar error under cancellation.

## epoch-memory

Wave19. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["fptas_objectives_join_exhaustive_subset_fibers","rational_certificates_join_polynomials_and_compensated_products"]

## limitations

Curvature bound is a caller premise, not inferred from knots. Overflow, work limits and allocation failures do not count as small approximation errors. Integer certificates do not provide a shared parent percentage or universal speed gain.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude infeasible or over-optimal FPTAS outputs, violated declared loss bounds, extrapolated interpolation, and true dot errors outside the absolute certificate. Exclude promotion of caller curvature to checked fact and promotion of L1-normalized dot error to cancellation-relative error.

## reverse-reconstruction

Type: reverse_reconstruction.

Objective certificates admit many item lists and optimal subsets. One interpolated ordinate admits many functions through the supplied knots; dot values admit sign/permutation and cancellation fibers. Reconstruct only constraints retained by the certificate, not unique original inputs.

## subset-embedding

Type: objective_embedding.

On representable inputs, each subset value v becomes a knapsack item of weight v and profit v with capacity target. Exact optima coincide. Each accepted epsilon result lies between (1-epsilon)*OPT and OPT; results need not coincide.

## absolute-transport

Type: lipschitz_error_transport.

If interpolation error <=e and downstream real map is L-Lipschitz on the reached interval, downstream discrepancy <=L*e. Any independent downstream error adds separately; floating conversion and missing curvature premises require extra treatment.

## cancellation

Type: normalization_countermodel.

For signed products near cancellation, sum(abs(products)) may be large while abs(sum(products)) is zero or tiny. A <=10% L1-normalized dot certificate supplies an absolute bound, not <=10% relative error of the scalar dot.

## wave19-evidence

### Family evidence

This family binds eight implementation/test sources through its manifest; views are partial source-derived models. Selected named joins are ["fptas_objectives_join_exhaustive_subset_fibers","rational_certificates_join_polynomials_and_compensated_products"] in algorithms/tests/composition_wave19.rs. Those checks pass four times as part of the 616-check suite. Leaf oracles and compiled production controls remain separately bound in adjacent evidence.

[Integrated evidence](../algorithm-library.groupant/model.md#wave19-evidence) records receipts; conditional laws outside these fixture scopes are not promoted to executed checks. No joint universal percentage bound follows from family membership.

## History

### 2026-10-30 · epoch-memory

Wave19, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave19-evidence

Heading `### Family evidence — 2026-10-30` stays in wave19-evidence as `### Family evidence`.
