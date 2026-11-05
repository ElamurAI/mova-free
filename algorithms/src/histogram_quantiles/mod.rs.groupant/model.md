# algorithm-histogram-quantiles

## model

# Discrete fixed-width quantile intervals

## domain
Finite streams; Inclusive min,max,width; exact per-bucket u64 counts, total N. Successful runtime/standard allocations assumed unless an explicit allocation error is returned. Configuration and numeric limits follow the forward model.

## forward
For domain [a,z], width w>0, bucket j covers [a+jw,min(z,a+(j+1)w-1)]. Query zero-based r walks exact cumulative counts and returns first bucket with cumulative end>r, along with rank interval [start,end).

## invariant
sum buckets=N. For sorted stream s and valid rank r, returned L<=s[r]<=U and start<=r<end. start equals count(x<L); end equals count(x<=U). These are exact bucket/rank statements, not an approximate scalar value or <=10% rank guarantee. Repeated values and empty buckets retain correct cumulative ranks.

## cost
O(b) construction/query/state, O(1) update; b<=1,000,000

## behavior-complement
Universe is domain-valid streams and candidate rectangles. Rectangles excluding sorted value at requested rank, including an exclusive right rank endpoint, or crossing the declared domain are excluded. Invalid rank cannot return Some.

## reverse-reconstruction
Given complete bucket counts, retain every multiset placing each count anywhere in its bucket, and every permutation. Query-only intervals impose cumulative constraints but do not recover other buckets uniquely. Singleton values 0 and 99 share the same width-100 bucket state.

## rank-rectangle
For selected bucket sum before<=r<sum through; no rank error estimate is substituted for this explicit interval. A densely populated bucket can span the entire rank domain.

## endpoint-arithmetic
Compute bucket count and upper endpoints in u128, including max=u64::MAX. The final bucket is clipped to max. Width may exceed domain length.

## bucket-budget
At most 1,000,000 buckets (8 MB payload); too many rejected before allocation. try_reserve_exact returns Allocation on reported allocation failure. Query performs no allocation.

## atomic-count
Reject out-of-domain values before indexing and total overflow before increment. Each bucket<=total bounds addition. Errors leave state identical.

## tests
Four focused tests per module cover independent references, boundary/error states, nonunique fibers, and an executable compiled mutation guard. See model.md#evidence for actual runs and restored-source controls.

## descriptant
The adjacent source remains authoritative. This model separates exact stream counts, additive frequency intervals, discrete rank/value rectangles and rounded numerical moments; these error metrics cannot be interchanged. syntax.json is a source-bound partial syntax projection without macro expansion or semantic AST. Prospective commitments precede implementation in model.md#prospective. No standalone executable surrogate is attached: production already uses bounded summaries, and a faster surrogate with a separately meaningful assurance metric remains an explicit gap. Finite tests do not certify numerical behavior universally or speed.

## evidence

### Validation

Actual CPU checks: 16 new focused tests, four per algorithm, included in all 505 library tests. Twice debug and twice release passed, with identical normalized 505 named test outcomes. No timing or speedup claim. Strict algorithms Clippy all-targets passed. Focused rustfmt passed. Source-only rustc test binaries compiled successfully for each production mutant; all four mutants failed their named guard assertion, source bytes were restored and each four-test positive suite passed.

Independent references: exact BTreeMap stream frequencies for 3000 deterministic events at all four capacities; exhaustive 625 length-four arrays across five histogram widths compared with sorted values; exhaustive 3125 length-five bounded integer arrays, every partition, compared with independent integer sum and squared-sum moments. Boundary cases include empty streams, invalid tiers, endpoints, atomic count overflow, nonfinite inputs, and moment overflow. Translation is tested on a fixed binary-exact fixture, not universally certified.

Compiled mutants: MG suppress cancellation count (cancellation_conservation: one cancellation epoch); Space-Saving erase inherited error (replacement_certificate_and_tie: replacement inherits one unit error); histogram include exclusive rank endpoint (bucket_boundary_mutation_guard: rank at boundary selects next bucket); moments use old residual (variance_mutation_guard: Welford residual yields population variance one).

Scope: finite reference correspondence, deterministic outcomes and mutation sensitivity; no universal numerical certificate, deployment, live teacher/model/corpus jobs or benchmark claims. Allocator aborts and full Rust semantic analysis remain outside these tests.

## prospective

### Prospective model (before implementation)
Streams own four std-only algorithms. Misra-Gries decrements k counters when full, preserving estimate <= frequency <= estimate + cancellations, cancellations <= N/(k+1). Space-Saving replaces smallest estimate (key breaks ties) and stores inherited error; retained frequency in [estimate-error,estimate], absent frequency <= min estimate. k=100/p for p in {1,2,5,10}, hence additive absolute frequency error/N <= p/100 (zero stream exact). BTreeMap uses O(k) storage, O(k) worst update. Checked N overflow is atomic; allocator failure outside Result.
Histogram stores fixed width buckets of discrete bounded u64 domain, capped at 1,000,000 buckets. Query by exact zero-based rank returns bucket value interval and occupied rank interval; no relative value or <=10% rank guarantee is declared. O(1) updates, O(b) query and memory. u128 bucket arithmetic handles MAX endpoints; allocation errors returned at construction.
Moments stores count, mean, unnormalized squared deviations using Welford updates and Chan merge. O(1) storage/work; count<=2^53, reject nonfinite input and nonfinite intermediate results atomically. IEEE rounding means no universal relative error cap; underflow may lose subnormal variance, cancellation/large offsets require scoped evidence. Independent integer sums and sums-of-squares oracles on bounded integral data, translation fixtures, partition merging. No standalone executable surrogate: these are production approximations; faster surrogate and certified floating error remain gaps.
All four: exhaustive bounded fixtures, adversarial edges, repeated debug/release equality, compiled mutant controls, source-bound groupants and partial syntax projections. No speed claims.

## History

### 2026-10-30 · document

2026-10-30. Original std-only implementation. Literature: Cormode et al. (2012), Synopses for Massive Data: Samples, Histograms, Wavelets, Sketches.

### 2026-10-30 · evidence

Heading `### Validation epoch 2026-10-30` stays in evidence as `### Validation`.

Source SHA256: ab6c2ca180991fd201f61ac868b6984bfc00260fdc977061365beb10360f95d4. Syntax projection generated twice into fresh files and compared byte-for-byte; no semantic AST or proof.

External evidence is under $MOVA_DATA/runs/octopus-wave19-streams-2026-10-30. Hashes bind the actual retained logs:

- debug1.log: e12ddcad38ace3b71d1669d5227ac04a67b4b09d4df6fffa0866b721f9a141b4

- debug2.log: 9bb3140eafe99143e2b9cb53f01a23112f3839052e8b7b835e5cf6990e942e64

- release1.log: 776d8e4bf51d3bde2938e089601a7067084099d594fcf0e64cd99e8361cb1836

- release2.log: 154dfee217199167c25a28cb603f50cf51be5062c5b95de63c47769294a0b7b8

- clippy.log: 038083ac66185a114552e9a3f886ed4081785d8b5cdf03227be1447e9c9e7f9c

- normalized.txt: 9492a4f663676b851e9ae50322f7b0a0b4a2b3c0d8328257dfca18b04c1a9c2e

- histogram_quantiles-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- histogram_quantiles-mutant.log: 114b8712f1c1f9f8fd6f0ce8261df6ed868d4752f097db1f95e76e1a2e216add

- histogram_quantiles-restored.log: d34b4045006a51e34f16135dc6309ac2315c284afb59f41fdfa7be1ed7b1d52e

- histogram_quantiles-mutation-hashes.txt: 3bb5f483a9a5f7ac03f1eac48eedc6a40e9709107cd8409d4ee559cb5ed9ab3d
