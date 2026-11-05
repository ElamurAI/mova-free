# algorithm-space-saving-tests

## model

# Space-Saving deterministic frequency intervals tests

## domain
Finite streams; BTreeMap key to (estimate,error), capacity k, total N. Successful runtime/standard allocations assumed unless an explicit allocation error is returned. Configuration and numeric limits follow the forward model.

## forward
Execute finite fixture families, compare independent reference observations, enforce bounds and atomic errors, return assertion verdicts. Passing means all implemented finite predicates held, not a proof for unseen streams.

## invariant
Fixtures and oracles are deterministic and std-only. Accepted certificate predicates must contain independent exact observations; expected errors leave snapshots unchanged. Test-only private state injection reaches count boundaries without infeasible streams.

## cost
Finite fixture enumeration; dominant frequency test scans 141 query keys for each of 3000 updates across four capacities; moments enumerate 3125 arrays and every split; histogram enumerates 625 arrays over five widths. No benchmark speed claim.

## behavior-complement
Universe pairs source variants with possible test verdicts. Failing any explicit oracle or invariant predicate excludes a passing verdict for that fixture. This is an inner complement of the finite harness relation; untested faults can still pass.

## reverse-reconstruction
The preimage of a passing transcript includes every implementation agreeing on tested observations, including implementations differing elsewhere. A failure transcript restricts at least one observed predicate, not a unique faulty line. Exact assertions and deterministic fixtures retain more information than aggregate pass counts.

## independent-oracle
Frequency tests accumulate exact BTreeMap counts outside production, histogram tests sort all short arrays, moment tests compute exact bounded integer sums and sums of squares. These reference mechanisms do not reuse the production recurrence.

## boundary-coverage
Tests cover unsupported configuration, empty state, extreme endpoints, overflow and unchanged error states. Enumerated arrays/streams cover finite cases only; no random universal confidence estimate.

## discrimination
Named compiled production mutants must fail an assertion after compiling successfully; restored original bytes must pass. This establishes gate sensitivity to that fault, not all faults.

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

2026-10-30. Original std-only implementation. Literature: Metwally, Agrawal and El Abbadi (2005), Efficient computation of frequent and top-k elements in data streams.

### 2026-10-30 · evidence

Heading `### Validation epoch 2026-10-30` stays in evidence as `### Validation`.

Source SHA256: c71708674a0151bb07677de0846c78c113b9cea99a7e89e2bc0349c0eb25f451. Syntax projection generated twice into fresh files and compared byte-for-byte; no semantic AST or proof.

External evidence is under $MOVA_DATA/runs/octopus-wave19-streams-2026-10-30. Hashes bind the actual retained logs:

- debug1.log: e12ddcad38ace3b71d1669d5227ac04a67b4b09d4df6fffa0866b721f9a141b4

- debug2.log: 9bb3140eafe99143e2b9cb53f01a23112f3839052e8b7b835e5cf6990e942e64

- release1.log: 776d8e4bf51d3bde2938e089601a7067084099d594fcf0e64cd99e8361cb1836

- release2.log: 154dfee217199167c25a28cb603f50cf51be5062c5b95de63c47769294a0b7b8

- clippy.log: 038083ac66185a114552e9a3f886ed4081785d8b5cdf03227be1447e9c9e7f9c

- normalized.txt: 9492a4f663676b851e9ae50322f7b0a0b4a2b3c0d8328257dfca18b04c1a9c2e

- space_saving-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- space_saving-mutant.log: 0182744b9eb25069ca2c001744da327a6e774eec1c90bda6541e02995120d3d1

- space_saving-restored.log: a6d7303bd1e9eab70b37d829539fb1b4e769a4f400e3d3a40d9c383de76f30a2

- space_saving-mutation-hashes.txt: 5de039790c4a9523fc8ee4839f412a52c9fde378277d76e65fd6b8f8f0bb1b33
