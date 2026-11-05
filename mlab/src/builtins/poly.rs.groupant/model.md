# mlab-numerics-poly

## descriptant

### mlab-numerics-poly

Descending-power coefficients feed real/complex Horner; polyfit builds real Vandermonde and solves via linalg::lstsq_svd. roots strips leading zero coefficients and trailing root zeros then eig of companion matrix. conv accumulates complex products; deconv performs real leading-term division and full-length remainder when dividend length>=divisor; a shorter dividend returns a cloned remainder preserving im. poly builds monic product over roots, derivative/integral use real coefficients.

## approximant

### Forward relation

Descending-power coefficients feed real/complex Horner; polyfit builds real Vandermonde and solves via linalg::lstsq_svd. roots strips leading zero coefficients and trailing root zeros then eig of companion matrix. conv accumulates complex products; deconv performs real leading-term division and full-length remainder when dividend length>=divisor; a shorter dividend returns a cloned remainder preserving im. poly builds monic product over roots, derivative/integral use real coefficients.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Coefficient order is highest degree first; convolution orientation follows first input, division follows dividend; poly() emits monic coefficient row and may discard tiny imaginary parts relative1e-10.

#### Cost

Horner O(P*X), conv O(PQ), long division O((Y-D+1)*D); roots O(degree³) eigen plus O(P²) repeated leading removals worst case; fit builds O(Mdegree) and dense SVD. No allocation caps.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## composition

### Composition

polyfit→linalg::lstsq_svd joins real Vandermonde columns with real y; roots/poly→Interp eig requires registered linalg builtins. Horner and conv use ops::cmul; deconv is real-only and cannot substitute for complex convolution inversion.

Fragments refer to disjoint useful source scopes in the single parent lexical projection. Registration and inline tests remain parent-modeled. Source callbacks and external backend implementations are dependencies, not duplicated ownership. Local tolerances cannot be composed into a subsystem percent guarantee.

## models

# Typed source models: mlab-numerics-poly

## behavior_complement

roots complex or nonfinite coefficients exclude eig; conv empty operands exclude success. polyfit extra outputs exclude solve even before length/degree validity. deconv zero first real divisor excludes complex cancellation semantics. This file does not uniformly preserve imaginary inputs.

## reverse_reconstruction

Horner at finite samples admits coefficient nullspace; full coefficient vector and fixed x determine values, not inverse uniquely. deconv quotient and remainder plus divisor reconstruct original real coefficients by padded convolution. poly roots are order-invariant mathematically and leading scaling is lost by roots.

## coefficient_transport

Vandermonde column j is x^(n-j), column-major; SVD result converted to row.

## root_zero_policy

Leading zeros removed, trailing zeros counted then appended as explicit roots; all-zero/constant polynomial returns0x0.

## complex_limits

polyval/conv/poly propagate complex parts. deconv long division uses re only but shorter-dividend remainder clones Y with im; polyder/polyint and polyfit use real data.

## division_shape

If dividend shorter than divisor, quotient scalar0 and optional remainder clones dividend; otherwise remainder retains original length with eliminated leading positions zeroed.


## tests

### Executed evidence

builtins::poly::wave30_tests::horner_and_division_contract: descending Horner at0/2, quotient/remainder, zero divisor. Companion eigen and fit paths remain source-modeled.

#### Native selection

    cargo test --offline --locked -p mlab --lib builtins::poly::wave30_tests::horner_and_division_contract -- --exact

Each selected test passed twice in debug and twice in release with numeric exit 0. Eight aggregate named outcomes were extracted from actual logs, sorted, and identical across all four runs. Post-control exact-restoration runs passed both profiles with separately retained numeric exits.

#### Compiled negative control

In this source, the expression

    let coef = rem[k] / d.re[0];

was temporarily replaced by

    let coef = 2.0 * rem[k] / d.re[0];

Cargo compiled successfully and the selected test reached an assertion failure with exit 101. The complete original source bytes were restored and compared before positive reruns. This control validates the selected observation, not all branches.

#### Scope and limits

Parent and fragment models distinguish executed fixtures from source-only branches. No benchmark main, held-out datasets, network, teacher or GPU run. No numerical approximation surrogate, exhaustive convergence proof, allocation-failure injection or speed claim. Strict Clippy was not rerun for this test/model-only change; added test blocks were individually rustfmt-formatted without changing production formatting. External receipts retain native commands, test maps, logs, exits, mutations and structural checks.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

### 2026-10-31 · tests

Heading `### Executed evidence — 2026-10-31` stays in tests as `### Executed evidence`.
