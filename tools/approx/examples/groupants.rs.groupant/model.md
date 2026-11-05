# approximation-runner

## descriptant

### Runner descriptant

Compose four companion/reference relations through their respective metrics and the shared gate. Parent acceptance is their conjunction; one successful leaf cannot validate wiring. Rank distance differs from value error and floats from exact rational outputs.

Active caps are 1%, 2%, 5%, 10%. Classification records distinguish metric, method and assurance from finite observations; a fixed-mapping test protects the wiring and supplies no composition-wide accuracy bound.

## model

# Runner model

## Forward

The active budgets 1%, 2%, 5%, 10% validate fixed seeded fixtures: 8192 sqrt/rank values, 96 knapsack items at capacity 1500, 32x32 RSA. Sqrt uses scalar relative error against original and std; rank uses sorted-copy interval distance/n; knapsack checks loss against exact DP and L<=OPT<=U; RSA compares floats with production ratios, preserving zero. Emit four CLASSIFICATION rows per budget before its CHECK row, with all correctness records before TIME rows. Fixed mappings are sqrt/scalar_relative/quantized_numeric/analytical, rank/normalized_rank/certified_sample/per_input_certificate, knapsack/objective_loss/relaxation_certificate/per_input_certificate and rsa/probability_relative/floating_normalization/analytical. Each record separately labels the runner's observations finite_observation. These scoped classes do not assert overall accuracy. Seven rounds alternate order after warmup, reporting median[min,max] us/call. Allocation, API validation and fallback are timed; compilation, fixtures and independent correctness checks are excluded. Std baselines complement original implementations; rank has no partition postcondition.

## behavior-complement

Universe: fixture/budget tags paired with reported observations or test verdicts. Exclude wrong CHECK aggregates, errors exceeding budget, and successful acceptance of corrupt RSA or invalid metrics. No fixed TIME ratio is required: hardware and load vary. This is a partial specification complement with finite evidence.

## reverse-reconstruction

Fixed generator revisions reproduce inputs. Without these, every input with matching sums, rank, objective and RSA bit sum remains compatible: checksums are not injective. Timing fibers contain many schedules, allocations and hardware states. No unique execution is reconstructed.

## fallback-observation

Zero at sampled ranks and 100 elsewhere forces median fallback at every tier. Knapsack [(6,6),(5,4),(5,4)] at capacity 10 forces fallback and returns 8. These constructed fixtures have 100% fallback, not a population estimate. The retired-policy fixture fell back for ordinary rank only at 1% and accepted ordinary knapsack at all tiers; current-policy observations are determined by the runner.

## oracle-independence

Sorted input independently determines rank distance. Subset enumeration covers 19 seeds, five capacities and four budgets with eight items, checking objective loss and upper bounds independently of DP. Production rational RSA anchors floats. Halved output and NaN fail. Self-reported metadata alone is insufficient evidence.

## Invariant

Successful CHECK errors are finite and within budget. Rank values occur in input. Objectives cannot exceed exact optimum. Repeated observations and forced fallback agree. Invalid metrics/corrupt-output controls fail.

## Cost

Checks include O(n log n) sorting and O(items*capacity) DP. Seven timing rounds use 2 sqrt batches, 16 rank calls, 4 knapsack/RSA calls or 64 tiny fallback calls, plus warmups. All work is bounded CPU-only.

## Checks and limits

31 example tests pass under caps 1%, 2%, 5%, 10%, including fixed classification mapping, independent subset enumeration, repeated differential checks, actual halved/NaN RSA output rejection, and invalid errors just above 10%, 15% and 25%. Strict all-targets clippy passes. The runner emits 16 CLASSIFICATION, 4 CHECK and 32 TIME records; fresh benchmark measurements are recorded by integration.

## History

### 2026-10-29 · descriptant

Retired-policy epoch 2026-10-29: finite checks pass. Std baselines and forced fallbacks retain possible slowdowns. Timings describe these fixtures, not whole-Mova gains. Mathematical assumptions remain; executable acceptance adds finite evidence. Reverse reconstruction retains every compatible input/state rather than treating a checksum as injective.

### 2026-10-29 · fallback-observation

Zero at sampled ranks and 100 elsewhere forces median fallback at every tier. Knapsack [(6,6),(5,4),(5,4)] at capacity 10 forces fallback and returns 8. These constructed fixtures have 100% fallback, not a population estimate. The 2026-10-29 retired-policy fixture fell back for ordinary rank only at 1% and accepted ordinary knapsack at all tiers; current-policy observations are determined by the runner.

### 2026-10-29 · Checks and limits

Retired-policy epoch 2026-10-29: 28 example tests pass: three runner tests plus included companion/RSA reference tests, not 28 independent new oracles. Release executes 4 CHECK and 32 TIME rows. Clippy all-targets passes with companion divisibility lint corrected. A source-bound partial syntax projection is present; semantic AST remains absent. Companion files must exist at original adjacent paths. Root repeated integration runs with identical CHECK records; see docs/mathematical-modeling.md. Finite fixture results do not prove universal accuracy or whole-Mova speedup.

### 2026-10-30 · Checks and limits

2026-10-30: 31 example tests pass under caps 1%, 2%, 5%, 10%, including fixed classification mapping, independent subset enumeration, repeated differential checks, actual halved/NaN RSA output rejection, and invalid errors just above 10%, 15% and 25%. Strict all-targets clippy passes. The runner emits 16 CLASSIFICATION, 4 CHECK and 32 TIME records; fresh benchmark measurements are recorded by integration.
