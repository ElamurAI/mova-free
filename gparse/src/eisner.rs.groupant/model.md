# gparse-eisner

## descriptant

### Source-bound description

For square sc[h][d] with finite f32 intermediates, n=rows-1, complete/incomplete word spans combine smaller spans by f32 max-plus; root r maximizes sc[0][r]+C_right(1,r)+C_left(r,n). Explicit stack recovers n heads. len<=1 returns [], n=1 returns [0] before scores are read.

## approximant

### Forward approximant

For square sc[h][d] with finite f32 intermediates, n=rows-1, complete/incomplete word spans combine smaller spans by f32 max-plus; root r maximizes sc[0][r]+C_right(1,r)+C_left(r,n). Explicit stack recovers n heads. len<=1 returns [], n=1 returns [0] before scores are read.

Domain: valid source-specific dimensions and resources stated in invariant; no automatic production substitution. Numerical observations are finite checks, not an end-to-end relative bound. Executable Rust surrogate: unsupported gap; existing exact tests are not surrogates. Future metric-specific caps may be 1/2/5/10%, never above 10%.

## invariant

### Invariant and cost

On admitted square scores with finite intermediate arithmetic, one root child, valid head coordinates, no self-head and projective tree. Real max-sum optimality is the mathematical recurrence model; f32 rounding/association is not certified equivalent to a real-valued global optimum. Strict > retains first ascending split/root under computed equal values. No NaN/infinite/ragged matrix guarantee.

#### Cost

O(n³) arithmetic and O(n²) score/backpointer storage plus O(n) output/stack; unchecked (n+1)² allocation arithmetic.

Allocation, finite arithmetic, platform dimensions and external failures are not universally certified.

## composition

### Composition

For square sc[h][d] with finite f32 intermediates, n=rows-1, complete/incomplete word spans combine smaller spans by f32 max-plus; root r maximizes sc[0][r]+C_right(1,r)+C_left(r,n). Explicit stack recovers n heads. len<=1 returns [], n=1 returns [0] before scores are read.

#### Scoped children

The byte-scoped children project distinct internal relations through the parent syntax file. The parent combines them in actual source order. Remaining declarations/tests outside child ranges belong to this parent. Their correctness does not establish external caller preconditions.

#### Interfaces and limits

On admitted square scores with finite intermediate arithmetic, one root child, valid head coordinates, no self-head and projective tree. Real max-sum optimality is the mathematical recurrence model; f32 rounding/association is not certified equivalent to a real-valued global optimum. Strict > retains first ascending split/root under computed equal values. No NaN/infinite/ragged matrix guarantee.

For exact arithmetic (or an explicit order-preservation assumption for every candidate comparison), tree h defines score matrices whose tree sum dominates every projective single-root competitor, with implementation tie ordering on equality. For actual f32, the compatible set is defined by the rounded recurrence/backpointer decisions, not guaranteed identical to those real-score cones. Common per-dependent shifts preserve exact-real argmax; f32 equivalence additionally needs no rounding-order change. Singleton accepts every two-row matrix.

## models

# Typed model catalog

## behavior_complement

Admitted finite-intermediate-domain output cannot contain two roots or an arc outside 0..n; scores into dependent zero do not affect output. Singleton ignores even missing inner rows; no score validation occurs on that branch.

## reverse_reconstruction

For exact arithmetic (or an explicit order-preservation assumption for every candidate comparison), tree h defines score matrices whose tree sum dominates every projective single-root competitor, with implementation tie ordering on equality. For actual f32, the compatible set is defined by the rounded recurrence/backpointer decisions, not guaranteed identical to those real-score cones. Common per-dependent shifts preserve exact-real argmax; f32 equivalence additionally needs no rounding-order change. Singleton accepts every two-row matrix.

## max_plus

Incomplete span I_left/right(s,t)=max_r(C_left(s,r)+C_right(r+1,t))+edge; complete spans close through an incomplete child. These are exact-real model equations; actual operations round in f32, and exact-real optimality needs preserved candidate ordering.

## tie_policy

All-zero 3x3 gives heads [0,1]. First-ascending tie statements require every candidate sum encountered to be finite so initial -infinity is strictly improved; all-infinite candidates retain defaults instead.

## domain_boundary

Zero rows/root-only and singleton bypass recurrence. Ragged matrices n>=2 may panic; nonfinite values can leave defaults and are excluded.

## coordinate_transport

Inputs are head-major sc[h][d], unlike network arc tensor [batch,dependent,head]. Caller transposition is required.

## tests

### Checks and limits

eisner::tests::best_tree_with_one_root; eisner::wave30::empty_singleton_and_first_root_tie; decoder_accepts_no_words

Selected commands run twice debug and twice release; exact receipts and source-to-test mapping retained in wave30 gparse external run directory. Tests do not certify allocation failure, all NaN/overflow inputs, trained-model accuracy or corpus timing. Compiled mutants cover selected boundaries; per-source coverage is recorded in the external map, not presumed.

See [source execution evidence](model.md#evidence) for exact names and unexecuted paths.

## evidence

### Execution evidence

Source: gparse/src/eisner.rs

- eisner::tests::best_tree_with_one_root: twice debug, twice release, all four exits 0.
- eisner::wave30::empty_singleton_and_first_root_tie: twice debug, twice release, all four exits 0.
- decoder_accepts_no_words: twice debug, twice release, all four exits 0.
- synthetic_cpu_prediction_preserves_slots_and_fallback: twice debug, twice release, all four exits 0.

#### Controls

1 source-local compiled assertion mutants; 2 observed control tests including dependency mutations. Mutant compile exit0, assertion exit101, byte-exact restoration and positive rerun exit0. Per-source controls are listed in external source-test-map.json, not implied for every source.

#### Limits

Finite tiny fixtures only; no exhaustive optimality oracle. F32 intermediate overflow and real-order preservation uncertified.

CUDA softmax comparisons use absolute tolerance2e-6. GELU grid maxima observed: values0.0004734993, derivatives0.00086838007, finite difference0.0000005534244635896357; respective strict caps5e-4/1e-3/1e-5. These are finite observations, not percentage profiles or throughput claims.

Receipts: external wave30 gparse run native-commands.json, results.json, controls.json and source-test-map.json. Sorted exact test names/outcomes match per profile; numeric diagnostics and timings need not be byte-identical. Strict scoped library Clippy reports ten pre-existing production lints; appended test blocks were formatted independently without production reformatting.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model was recorded before edits. Added tests use bounded synthetic fixtures; production algorithms unchanged. Coverage is partial, not proof. No trained corpus, live teacher or throughput experiment was executed.

### 2026-10-31 · evidence

Heading `### Execution evidence — 2026-10-31` stays in evidence as `### Execution evidence`.
