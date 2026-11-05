# algorithm-misra-gries

## model

# Misra–Gries bounded heavy-hitter counters

## domain
Finite streams; BTreeMap<u64,u64>, accepted count N, cancellations D. Successful runtime/standard allocations assumed unless an explicit allocation error is returned. Configuration and numeric limits follow the forward model.

## forward
Insert or increment a key if room. Otherwise decrement all k positive counters and erase zeros, increasing D. Return [c,c+D], with absent c=0. Valid percentages are exactly 1,2,5,10; k=100/p.

## invariant
0<=c(x)<=f(x)<=c(x)+D, sum c+(k+1)D=N. Every cancellation removes k existing units and the uninserted arrival. Thus D<=floor(N/(k+1)), and interval width/N<=p/100 for N>0; N=0 is exact. This is additive frequency error normalized by all observations, never relative to f(x).

## cost
O(k) worst-case update, O(log k) query, O(k) memory; k=100/p

## behavior-complement
Pairs (stream, candidate bounds) with low>f(x), high<f(x), or width*100>N*p are excluded. Configurations outside the four percentages cannot yield a valid constructed state.

## reverse-reconstruction
For a full state retain streams whose cancellation execution yields exactly those counters and D. For a query [l,u] alone, compatible streams satisfy l<=f(x)<=u plus N and tier constraints; this necessary relaxation is not sufficient for exact state reconstruction. Permutations below capacity give identical states; cancellation loses identities.

## cancellation-mass
Every decrement event has k+1 distinct contributors, so counting removed mass gives N=sum c+(k+1)D; surviving counters stay positive.

## frequency-envelope
For each key at most one true occurrence is discarded per cancellation. D bounds lost mass for both retained and absent keys; dividing by N uses all events, with empty stream exact.

## bounded-state
At most k entries, k in {100,50,20,10}; no implicit unlimited exact table. O(k) retain traversal dominates worst update. BTreeMap allocation abort/panic is outside Result.

## atomic-count
Compute N+1 before mutation; overflow returns unchanged state. Every counter and D is <=N, so later additions are safe on reachable states.

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

2026-10-30. Original std-only implementation. Literature: Misra and Gries (1982), Finding repeated elements.

### 2026-10-30 · evidence

Heading `### Validation epoch 2026-10-30` stays in evidence as `### Validation`.

Source SHA256: ec15a021514e89b99e1e888498e67ab4c8ec9d9642366b2dced2697ec543d367. Syntax projection generated twice into fresh files and compared byte-for-byte; no semantic AST or proof.

External evidence is under $MOVA_DATA/runs/octopus-wave19-streams-2026-10-30. Hashes bind the actual retained logs:

- debug1.log: e12ddcad38ace3b71d1669d5227ac04a67b4b09d4df6fffa0866b721f9a141b4

- debug2.log: 9bb3140eafe99143e2b9cb53f01a23112f3839052e8b7b835e5cf6990e942e64

- release1.log: 776d8e4bf51d3bde2938e089601a7067084099d594fcf0e64cd99e8361cb1836

- release2.log: 154dfee217199167c25a28cb603f50cf51be5062c5b95de63c47769294a0b7b8

- clippy.log: 038083ac66185a114552e9a3f886ed4081785d8b5cdf03227be1447e9c9e7f9c

- normalized.txt: 9492a4f663676b851e9ae50322f7b0a0b4a2b3c0d8328257dfca18b04c1a9c2e

- misra_gries-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- misra_gries-mutant.log: 4c427bfa5f3cc658e3a55079d18c63356750cedf2a3e81c059c0eae12981d7b8

- misra_gries-restored.log: aa789c9b61cd8210f9ec7642acd520fd72da6bbf7c9174e078c3482cba146425

- misra_gries-mutation-hashes.txt: 524b94e88102e10903a7e27017e819aceb963bc7df9f6b544006dfded9395aa8
