# mlab-numerics-fft

## descriptant

### mlab-numerics-fft

transform reads matrix, optional nonempty numeric N's first real cell, dimension, then empty early return. For each selected slice it copies/truncates input to N, zero-pads remainder, calls rustfft forward/inverse and scales inverse by 1/N. Row vectors default dim2, others dim1; every admitted dimension other than1, including dim>=3, aliases dim2; output uses column-major indices.

## approximant

### Forward relation

transform reads matrix, optional nonempty numeric N's first real cell, dimension, then empty early return. For each selected slice it copies/truncates input to N, zero-pads remainder, calls rustfft forward/inverse and scales inverse by 1/N. Row vectors default dim2, others dim1; every admitted dimension other than1, including dim>=3, aliases dim2; output uses column-major indices.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

For dim1 output rows=N and cols=input.cols; for every admitted dim>=2 output rows=input.rows and cols=N. Empty input bypasses resizing after argument validation. No finite/N allocation cap; Inf casts and planner/resource faults remain gaps.

#### Cost

O(S*N log N) FFT plus planning, O(S*N+input size+planner workspace) storage; no speed measurement.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## composition

### Composition

Registry fft/ifft share transform with inverse flag. Mat column-major mapping mediates rustfft's contiguous buffers. Mat::complex may narrow only after transform; exact layout is part of the join. FFT dimensions>=3 alias dim2, unlike reduce::slices singleton behavior for higher dimensions.

Fragments refer to disjoint useful source scopes in the single parent lexical projection. Registration and inline tests remain parent-modeled. Source callbacks and external backend implementations are dependencies, not duplicated ownership. Local tolerances cannot be composed into a subsystem percent guarantee.

## models

# Typed source models: mlab-numerics-fft

## behavior_complement

Invalid N<1 or fractional N excludes success even for empty input. Empty input excludes planning and resizing after validation. Forward excludes inverse normalization. Inputs beyond N cannot affect truncated output.

## reverse_reconstruction

For fixed dimension and N, inverse DFT reconstructs padded/truncated complex slices up to rounding. If N<len, arbitrary tails share output; if N>len, compatible inputs must map to zero padded suffix after inverse. Narrowed imaginary storage loses representation identity.

## dimension_geometry

dim1 reads s*rows+t; every admitted dim>=2 reads t*rows+s and acts as dim2. For a2x3 input fft(A,4,3) returns2x4, not a third-dimensional array. Output uses corresponding out_r stride.

## normalization

Inverse multiplies every coefficient by 1/N; forward scale1. Four-one spectrum maps to [1,0,0,0].

## empty_precedence

N and dim validation happen before m.is_empty; valid empty input is cloned unchanged.

## padding_loss

[1,0] at N4 pads to [1,0,0,0] and spectrum[1,1,1,1]; N smaller discards tail.


## tests

### Executed evidence

builtins::fft::wave30_tests::padding_inverse_and_validation: padded impulse, normalized inverse, invalid empty N and empty clone.

#### Native selection

    cargo test --offline --locked -p mlab --lib builtins::fft::wave30_tests::padding_inverse_and_validation -- --exact

Each selected test passed twice in debug and twice in release with numeric exit 0. Eight aggregate named outcomes were extracted from actual logs, sorted, and identical across all four runs. Post-control exact-restoration runs passed both profiles with separately retained numeric exits.

#### Compiled negative control

In this source, the expression

    1.0 / n as f64

was temporarily replaced by

    2.0 / n as f64

Cargo compiled successfully and the selected test reached an assertion failure with exit 101. The complete original source bytes were restored and compared before positive reruns. This control validates the selected observation, not all branches.

#### Scope and limits

Parent and fragment models distinguish executed fixtures from source-only branches. No benchmark main, held-out datasets, network, teacher or GPU run. No numerical approximation surrogate, exhaustive convergence proof, allocation-failure injection or speed claim. Strict Clippy was not rerun for this test/model-only change; added test blocks were individually rustfmt-formatted without changing production formatting. External receipts retain native commands, test maps, logs, exits, mutations and structural checks.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

### 2026-10-31 · tests

Heading `### Executed evidence — 2026-10-31` stays in tests as `### Executed evidence`.
