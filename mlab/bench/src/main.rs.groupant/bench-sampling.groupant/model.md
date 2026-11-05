# mlab-numerics-bench-sampling

## descriptant

### mlab-numerics-bench-sampling

Lcg::next wrapping-updates u64 and takes upper53 bits; data consumes n² samples; best measures3 callbacks and keeps min duration while overwriting acc each call.

## approximant

### Forward relation

Lcg::next wrapping-updates u64 and takes upper53 bits; data consumes n² samples; best measures3 callbacks and keeps min duration while overwriting acc each call.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

u64 wrap and f64 dyadic conversion are defined; duration comparison cannot establish deterministic time.

#### Cost

O(n²) sampling; exactly3 callback calls plus Instant reads.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-bench-sampling

## behavior_complement

Callback count cannot be0,1 or2 on normal return. Seed0 first output excludes using old state's bits. Equal seed/n excludes different data.

## reverse_reconstruction

One output loses11 state bits; invertible odd LCG multiplier maps each compatible post-state to a prior seed. best's last result permits arbitrary earlier results and durations consistent with minimum.

## zero_data

n0 produces empty Vec without consuming LCG.

## last_value

best callback count3 returns last accumulator3, independent of timing winner.

## range

top53 integer in[0,2^53) implies output in[-.5,.5).

## resource_gap

n*n uses unchecked usize multiplication; huge dimensions are outside helper domain.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
