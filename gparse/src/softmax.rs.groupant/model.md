# gparse-softmax

## descriptant

### Source-bound description

last_dim dispatches nonempty CUDA F32 to contiguous custom Softmax; otherwise Candle reference last-dimension softmax. cuda_fwd validates contiguous offsets, row dimension/nonempty, i32 columns/u32 rows, allocates output and launches existing softmax_f32 with retained source offset and grid(rows,1,1), block(1,32,1). bwd returns p*(upstream-sum_last(p*upstream)).

## approximant

### Forward approximant

last_dim dispatches nonempty CUDA F32 to contiguous custom Softmax; otherwise Candle reference last-dimension softmax. cuda_fwd validates contiguous offsets, row dimension/nonempty, i32 columns/u32 rows, allocates output and launches existing softmax_f32 with retained source offset and grid(rows,1,1), block(1,32,1). bwd returns p*(upstream-sum_last(p*upstream)).

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

Output shape matches input; finite real row relation sums probabilities to one, while numerical agreement is only finite-tested. Backward row-sum zero mathematically; singleton derivative zero. CPU custom kernel rejects; public wrapper routes CPU to reference.

#### Cost

O(elements) forward and backward plus reductions; contiguous conversion may allocate/copy O(elements). CUDA allocation/launch/device errors propagate; no speed measurement or all-domain error cap.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

last_dim dispatches nonempty CUDA F32 to contiguous custom Softmax; otherwise Candle reference last-dimension softmax. cuda_fwd validates contiguous offsets, row dimension/nonempty, i32 columns/u32 rows, allocates output and launches existing softmax_f32 with retained source offset and grid(rows,1,1), block(1,32,1). bwd returns p*(upstream-sum_last(p*upstream)).

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

Output shape matches input; finite real row relation sums probabilities to one, while numerical agreement is only finite-tested. Backward row-sum zero mathematically; singleton derivative zero. CPU custom kernel rejects; public wrapper routes CPU to reference.

For positive normalized row p, compatible finite real logits are log(p)+constant; zero probabilities may be finite underflow/masking in f32 and do not uniquely identify -infinity. Gradient output admits upstream shifts by a row constant.

## models

# Typed model catalog

## behavior_complement

CPU/F64 inputs cannot use the custom CUDA F32 kernel through last_dim. Noncontiguous input is copied before custom invocation; raw custom rejects noncontiguous. CUDA error path does not silently use CPU fallback.

## reverse_reconstruction

For positive normalized row p, compatible finite real logits are log(p)+constant; zero probabilities may be finite underflow/masking in f32 and do not uniquely identify -infinity. Gradient output admits upstream shifts by a row constant.

## gradient_transport

VJP is diag(p)g-p(pᵀg); unlike no-backward upstream fused op, custom propagates into logits.

## layout_transport

contiguous_offsets start..end preserves narrow-view offset; transpose becomes contiguous before launch.

## failure_precedence

Raw CUDA path checks contiguous -> last dimension -> nonempty -> integer conversions -> dtype slice -> kernel -> alloc -> launch.

## numerical_observation

Selected widths 1,3,17,33,257 test abs forward/gradient/finite-difference error<2e-6; this is not relative percentage or universal guarantee.

## tests

### Checks and limits

cpu_f64_uses_reference; fused_softmax_matches_reference_gradients_and_finite_differences

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/src/softmax.rs

- cpu_f64_uses_reference: twice debug, twice release, all four exits 0.
- fused_softmax_matches_reference_gradients_and_finite_differences: twice debug, twice release, all four exits 0.
- synthetic_single_optimizer_step: twice debug, twice release, all four exits 0.

#### Controls

1 source-local compiled assertion mutants; 2 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Finite synthetic CPU/CUDA observations; no trained-corpus quality or all-input numeric guarantee.

CUDA softmax comparisons use absolute tolerance2e-6. GELU grid maxima observed: values0.0004734993, derivatives0.00086838007, finite difference0.0000005534244635896357; respective strict caps5e-4/1e-3/1e-5. These are finite observations, not percentage profiles or throughput claims.

Receipts: external wave30 gparse run native-commands.json, results.json, controls.json and source-test-map.json. Sorted exact test names/outcomes match per profile; numeric diagnostics and timings need not be byte-identical. Strict scoped library Clippy reports ten pre-existing production lints; appended test blocks were formatted independently without production reformatting.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-10-31 · evidence

Heading `### Execution evidence — 2026-10-31` stays in evidence as `### Execution evidence`.
