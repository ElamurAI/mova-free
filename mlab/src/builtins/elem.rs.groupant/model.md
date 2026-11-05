# mlab-numerics-elem

## descriptant

### mlab-numerics-elem

register installs unary/binary numeric closures. sqrt/log/log2/log10 promote negative real arrays to complex; exp/sin/cos preserve complex mathematics. Rounding maps each part then narrows. complex explicitly retains imaginary storage even when zero. Number-theory closures operate on f64/cast integers, not an arbitrary-precision integer domain.

## approximant

### Forward relation

register installs unary/binary numeric closures. sqrt/log/log2/log10 promote negative real arrays to complex; exp/sin/cos preserve complex mathematics. Rounding maps each part then narrows. complex explicitly retains imaginary storage even when zero. Number-theory closures operate on f64/cast integers, not an arbitrary-precision integer domain.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Column-major positions are retained by maps; class becomes Double except explicit Logical predicates. No uniform finite-result guarantee: domain errors in elementary functions can yield NaN/Inf, while some real_only cases return MError.

#### Cost

O(N) unary/binary cells plus map2 broadcasting; primes O(n log log n) time/O(n) memory, trial division O(sqrt(n)) per integer, combinations O(k*C(n,k)) output. Casts and dimension overflow lack resource limits.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## composition

### Composition

Registration closures call mat_arg/scalar_arg and ops::map2. cmap→csqrt preserves cell order and constructs narrowed complex Mat; explicit complex intentionally preserves imaginary storage. No implicit complex support should be inferred for map-only closures.

Fragments refer to disjoint useful source scopes in the single parent lexical projection. Registration and inline tests remain parent-modeled. Source callbacks and external backend implementations are dependencies, not duplicated ownership. Local tolerances cannot be composed into a subsystem percent guarantee.

## models

# Typed source models: mlab-numerics-elem

## behavior_complement

For valid storage, unary maps cannot permute cells or alter rows/cols. A real-only trigonometric call excludes success when im storage exists. complex(1,0) excludes an isreal representation. mod(x,0)=x excludes rem's zero-divisor NaN behavior; these operations are not interchangeable.

## reverse_reconstruction

Preimage of abs(r) contains all complex cells of modulus r; rounding fibers contain intervals. sqrt on its finite principal branch has antecedent z=w*w only with branch restrictions; gamma poles and many floating outputs are noninjective. Retained dimensions constrain compatible matrix antecedents but do not identify their values.

## complex_branch

sqrt(-9+0i)=0+3i; sqrt(3-4i)=2-i. cmap uses im_or_zeros and Mat::complex narrowing; explicit complex() bypasses narrowing.

## zero_and_poles

gamma at zero/negative integers returns positive Inf before libm; gamma(5)=24. sign(0)=0; mod divisor zero returns input before finite checks.

## failure_precedence

mat_arg precedes complex promotion. nthroot validates nonzero integer n before rejecting negative X with even n. primes floors then requires finite before n<2 shortcut.

## representation_transport

map uses only re; cmap zips re with imaginary storage. Predicate output is Logical. Scalar/broadcast binary functions use ops::map2; integer domain validation differs across gcd, factorial and isprime.


## tests

### Executed evidence

builtins::elem::wave30_tests::complex_mapping_contract: principal complex roots, shape, gamma pole and positive integer.

#### Native selection

    cargo test --offline --locked -p mlab --lib builtins::elem::wave30_tests::complex_mapping_contract -- --exact

Each selected test passed twice in debug and twice in release with numeric exit 0. Eight aggregate named outcomes were extracted from actual logs, sorted, and identical across all four runs. Post-control exact-restoration runs passed both profiles with separately retained numeric exits.

#### Compiled negative control

In this source, the expression

    (0.0, (-x).sqrt())

was temporarily replaced by

    (0.0, 2.0 * (-x).sqrt())

Cargo compiled successfully and the selected test reached an assertion failure with exit 101. The complete original source bytes were restored and compared before positive reruns. This control validates the selected observation, not all branches.

#### Scope and limits

Parent and fragment models distinguish executed fixtures from source-only branches. No benchmark main, held-out datasets, network, teacher or GPU run. No numerical approximation surrogate, exhaustive convergence proof, allocation-failure injection or speed claim. Strict Clippy was not rerun for this test/model-only change; added test blocks were individually rustfmt-formatted without changing production formatting. External receipts retain native commands, test maps, logs, exits, mutations and structural checks.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

### 2026-10-31 · tests

Heading `### Executed evidence — 2026-10-31` stays in tests as `### Executed evidence`.
