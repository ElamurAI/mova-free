# mlab-numerics-bench

## descriptant

### mlab-numerics-bench

LCG state advances modulo2^64 by multiplier6364136223846793005 and increment1442695040888963407; top53 bits map to [-.5,.5). data returns n² draws. best runs callback exactly3 times and returns minimum elapsed seconds plus last callback value. main sets faer sequential mode and performs fixed large backend comparisons, reporting residuals and times.

## approximant

### Forward relation

LCG state advances modulo2^64 by multiplier6364136223846793005 and increment1442695040888963407; top53 bits map to [-.5,.5). data returns n² draws. best runs callback exactly3 times and returns minimum elapsed seconds plus last callback value. main sets faer sequential mode and performs fixed large backend comparisons, reporting residuals and times.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Wrapping state arithmetic is intentional. Seed/dimension determine data exactly. Time is nondeterministic; tests only check finite nonnegative duration and callback count.

#### Cost

data O(n²) time/storage, best3 callback costs; main dense1000 multiply/solve and500 eig O(n³) per call with backend memory. No timings measured in this epoch.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## composition

### Composition

data supplies identical column-major buffers to faer and nalgebra; best returns timing minimum and final observation separately. This is not the same aggregation as retaining the fastest callback's checksum. Bench main remains unexecuted.

Fragments refer to disjoint useful source scopes in the single parent lexical projection. Registration and inline tests remain parent-modeled. Source callbacks and external backend implementations are dependencies, not duplicated ownership. Local tolerances cannot be composed into a subsystem percent guarantee.

## models

# Typed source models: mlab-numerics-bench

## behavior_complement

best excludes coupling the returned value to the fastest sample: it always returns last acc. data excludes entropy/network/corpus reads. Helper evidence excludes executing or measuring main's actual benchmark.

## reverse_reconstruction

Each sampled f64 identifies high53 post-state bits and allows2^11 low-bit antecedents before inverse LCG; sequential samples constrain the state further. best output time/value cannot recover all3 durations or earlier values.

## lcg_bits

State0 first advance equals increment; shift11 and division2^53 produces an exactly representable bounded dyadic then minus.5.

## repetition_semantics

Three calls returning1,2,3 result in last=3 even when first is fastest.

## effect_boundary

main alters global faer parallelism and prints results; helper tests do neither.

## residual_observations

main prints Ax-b and AV-VD residuals, but no hard pass/fail tolerance; product corners are a checksum, not full correctness.


## tests

### Executed evidence

wave30_tests::bounded_generator_and_three_calls: four-cell deterministic data, bounded range, first LCG state, three calls and last accumulator. Main backend benchmark excluded.

#### Native selection

    cargo test --offline --locked -p mlab-backend-bench --bin mlab-backend-bench wave30_tests::bounded_generator_and_three_calls -- --exact

Each selected test passed twice in debug and twice in release with numeric exit 0. Eight aggregate named outcomes were extracted from actual logs, sorted, and identical across all four runs. Post-control exact-restoration runs passed both profiles with separately retained numeric exits.

#### Compiled negative control

In this source, the expression

    for _ in 0..3 {

was temporarily replaced by

    for _ in 0..2 {

Cargo compiled successfully and the selected test reached an assertion failure with exit 101. The complete original source bytes were restored and compared before positive reruns. This control validates the selected observation, not all branches.

#### Scope and limits

Parent and fragment models distinguish executed fixtures from source-only branches. No benchmark main, held-out datasets, network, teacher or GPU run. No numerical approximation surrogate, exhaustive convergence proof, allocation-failure injection or speed claim. Strict Clippy was not rerun for this test/model-only change; added test blocks were individually rustfmt-formatted without changing production formatting. External receipts retain native commands, test maps, logs, exits, mutations and structural checks.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

### 2026-10-31 · tests

Heading `### Executed evidence — 2026-10-31` stays in tests as `### Executed evidence`.
