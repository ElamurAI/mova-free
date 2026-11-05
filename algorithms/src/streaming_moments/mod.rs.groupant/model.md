# algorithm-streaming-moments

## model

# Welford updates and Chan merge

## domain
Finite streams; Accepted n<=2^53, f64 mean and unnormalized M2. Successful runtime/standard allocations assumed unless an explicit allocation error is returned. Configuration and numeric limits follow the forward model.

## forward
Update n'=n+1, delta=x-mean, mean'=mean+delta/n', M2'=M2+delta*(x-mean'). Merge with d=other.mean-mean, n'=n+m, mean'=mean+d*m/n', M2'=M2+other.M2+d*d*n*(m/n'). Empty operands are identity. Population variance M2/n; sample variance M2/(n-1).

## invariant
In exact reals M2=sum (x-mean)^2, and merge computes moments of concatenation. Implemented IEEE f64 stores finite mean and nonnegative finite M2; count is exactly representable. Rounding may make order and partition observable. No universal relative bound, including near-zero variance. Subnormal products can underflow to zero. Intermediate overflow rejects atomically even if rearranged real expression is representable.

## cost
O(1) update/merge/query and memory; no input history stored

## behavior-complement
Universe consists of finite input sequences or valid summary pairs and candidate transitions. Successful states with wrong count, nonfinite fields, negative M2, or mutated state on returned error are excluded. Mathematical negative variance and nonfinite input acceptance are outside forward relation.

## reverse-reconstruction
For exact real summaries the fiber is all sequences with given n, sum=n*mean, sum squares=M2+n*mean^2; order is unobserved and even multisets need not be unique. Actual floating fiber additionally requires the specified rounded execution to yield the fields; real equal moments alone do not ensure bitwise equality. [1,3] and [3,1] are a concrete binary-exact fiber.

## translation
In exact reals adding c shifts mean by c and preserves M2. Floating implementation checks selected exactly representable shifted fixtures; large offsets can lose differences. No universal translation invariance claim.

## partition-merge
Real Chan formula joins means by count-weighted average and adds between-group scatter d^2*n*m/(n+m). Every split of bounded integer fixtures is checked against independent sums/squares.

## finite-state
Reject NaN/infinity input; reject nonfinite intermediate mean/M2 or negative M2. n<=2^53 avoids rounding counts; very large delta or delta squared may overflow. Variance underflow is allowed and documented.

## atomic-count
All candidate arithmetic is local; assign fields only after checks. CountLimit and Overflow preserve original state, including merging opposite extreme means. No heap allocation.

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

2026-10-30. Original std-only implementation. Literature: Welford (1962), Note on a method for calculating corrected sums of squares and products; Chan, Golub and LeVeque (1983), Algorithms for computing the sample variance.

### 2026-10-30 · evidence

Heading `### Validation epoch 2026-10-30` stays in evidence as `### Validation`.

Source SHA256: 041cb5390bc2e65365bde5e4d5081eeb5c75433081560111547a4c75f6589852. Syntax projection generated twice into fresh files and compared byte-for-byte; no semantic AST or proof.

External evidence is under $MOVA_DATA/runs/octopus-wave19-streams-2026-10-30. Hashes bind the actual retained logs:

- debug1.log: e12ddcad38ace3b71d1669d5227ac04a67b4b09d4df6fffa0866b721f9a141b4

- debug2.log: 9bb3140eafe99143e2b9cb53f01a23112f3839052e8b7b835e5cf6990e942e64

- release1.log: 776d8e4bf51d3bde2938e089601a7067084099d594fcf0e64cd99e8361cb1836

- release2.log: 154dfee217199167c25a28cb603f50cf51be5062c5b95de63c47769294a0b7b8

- clippy.log: 038083ac66185a114552e9a3f886ed4081785d8b5cdf03227be1447e9c9e7f9c

- normalized.txt: 9492a4f663676b851e9ae50322f7b0a0b4a2b3c0d8328257dfca18b04c1a9c2e

- streaming_moments-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- streaming_moments-mutant.log: 8031c37cde3f8490fb0e2d5a6a991f28b4bdd028411b69b3b51ac3acefb8f98a

- streaming_moments-restored.log: 740f9415202f4577f02a383316b71bbf32c93a7f248146161a28ab362afae62f

- streaming_moments-mutation-hashes.txt: fef2a080da65b437f912032a08f9e4c601dfad68bba69cdcc8286662d7e64848
