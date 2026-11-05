# algorithm-counting-sort

## descriptant

### counting_sort mod.rs epoch memory

Three tests pass in debug and release. All 21,845 arrays of lengths 0..7 over four values match std slice sorting. An 8,192-value fixed-seed array also matches. Boundary tests cover empty/zero budget, high minimum, range overflow, budget rejection and deterministic Vec capacity overflow; every error preserves input. A compiled mutant that fills each bucket with minimum rather than minimum+offset is rejected by boundaries_and_errors_are_atomic.

The combiner relates raw source, partial syntax, forward relation, typed obernants, invariant, cost and finite evidence. Preserve predictions and rejected mutants across future epochs. Internal radix byte stability is observable via distinct values sharing a digit; whole-sort payload stability is not exposed. Missing universal proof, allocator-exhaustion injection and downstream wiring remain explicit. Exact algorithms require no invented approximant profiles. Future executable approximants must honor 1%, 2%, 5%, 10% caps and source/evidence binding.

## model

# counting_sort mod.rs model

## Forward

For empty input return Ok unchanged. Otherwise span=max-min+1: overflow returns RangeOverflow; span above budget returns BucketLimitExceeded; fallible histogram reserve failure returns AllocationFailed. Each error preserves input. Success returns the unique ascending permutation with the same multiplicities.

## Invariant

Histogram sum equals processed input length. Each bucket counts exactly occurrences of minimum+offset. During reconstruction written equals sum of completed buckets; written+count never exceeds n. minimum+offset<=maximum. Counts cannot exceed slice length. All fallible operations precede input writes.

## Cost

O(n+span) time and O(span) auxiliary usize elements on success. Validation errors O(n) time and O(1) space. No timing measurements.

## nonbehavior

Type: behavior_complement. Within observations (input, budget, allocator outcome, Result, output slice), exclude changed input on Err, changed length or multiplicity, nonascending success, success past a range/budget failure, and error variants inconsistent with validation precedence. This is a source-derived relation, not just negative fixtures.

## preimages

Type: reverse_reconstruction. A successful sorted output admits every input permutation of that multiset, with budget at least its inclusive span and successful reserve; empty output admits empty input and any budget. Errors admit unchanged input with matching overflow, budget or allocation branch. Original order is generally not recoverable.

## idempotence

Type: metamorphic_relation. For successful sorting S, S(S(x))=S(x). Translation by a constant preserving usize bounds preserves order and span. Tests check idempotence and the narrow span at usize::MAX.

## histogram-fiber

Type: reverse_reconstruction. Domain: Finite usize slices, exact multiplicity map H, fixed success and budget.

For a fixed ascending y and successful allocation, S(x,b)=y iff H(x)=H(y) and b>=span(y), with empty input needing zero buckets. All distinct permutations survive; support alone is insufficient. Oracle histogram_fibers_retain_multiplicity_not_order enumerates 256 four-place inputs and finds 12 predecessors of [2,2,4,5].

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## monotone-relabel

Type: metamorphic_relation. Domain: Finite usize slices with representable strictly increasing image and separate admissible budgets.

For strictly increasing f on the finite input support with every image representable, S(f(x),b2)=f(S(x,b1)) when both allocations succeed and budgets cover their respective inclusive spans. A nonlinear or affine map may change the required budget. Wrapping arithmetic violates the premise. The checked affine map 3v+11 and insufficient transformed budget are tested.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## budget-threshold

Type: resource_preimage. Domain: Fixed nonempty input, representable span, budget in usize, allocation fixed successful when feasible.

For nonempty x with representable span s, successful budget antecedents are b>=s conditional on successful allocation; b<s yields precisely BucketLimitExceeded{s,b} unchanged. This is an upper ray, independent of input length. Repeated singleton-valued inputs need one bucket. The test scans budgets 0 through 6 at span 3.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## first-failure

Type: failure_partition. Domain: Call transitions including range, budget and allocator outcome; ordinary termination.

Validation partitions calls in order: empty succeeds; unrepresentable inclusive span yields RangeOverflow regardless of budget; otherwise s>b yields BucketLimitExceeded; otherwise reserve failure yields AllocationFailed; otherwise success. All error fibers retain original order because no writes precede reserve. Capacity overflow is deterministic evidence; physical allocator exhaustion is untested.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## histogram-reconstruction

Type: conservation_relation. Domain: Internal successful counting and reconstruction over span-sized histogram; no allocation failure.

After counting, H[j] equals multiplicity of min+j. Reconstruction writes disjoint intervals [sum_{i<j}H[i],sum_{i<=j}H[i]) with min+j. These intervals partition [0,n), including empty buckets; histogram zeros do not remove gaps from the resource span. Replacing min+j by min preserves length but violates values and histogram, rejected by the compiled mutant.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## evidence

### Evidence

Three tests pass in debug and release. All 21,845 arrays of lengths 0..7 over four values match std slice sorting. An 8,192-value fixed-seed array also matches. Boundary tests cover empty/zero budget, high minimum, range overflow, budget rejection and deterministic Vec capacity overflow; every error preserves input. A compiled mutant that fills each bucket with minimum rather than minimum+offset is rejected by boundaries_and_errors_are_atomic.

Strict crate Clippy and owned rustfmt checks pass. Two deterministic debug executions have matching normalized test outcomes. Source-derived syntax is generated twice and compared. No timing, downstream wiring or universal verification claim.

## prospective

### Prospective

Before implementation: repository search in algorithms/src and math/src found no counting_sort or radix_sort. Implement original exact integer sorting, with comparison sorting used only as an independent test oracle. Counting histogram spans min..=max with checked arithmetic and caller bucket cap; allocation finishes before mutation. Radix uses eight stable least-significant-byte passes. Predict O(n+span) time/O(span) memory and O(8(n+256)) time/O(n+256) memory respectively. Preserve length and multiplicities. Empty input succeeds without allocation. No timing gain or universal verification claim. Validate exhaustive bounded arrays, extremes, deterministic larger inputs, debug/release and strict Clippy; reject a compiled implementation mutant.

## History

### 2026-10-30 · descriptant

2026-10-30. Repository search found no matching implementation in algorithms/src or math/src. Original English std-only code; no external solution imports. Prospective mathematics preceded implementation and remains in model.md#prospective.

#### Relation refinement 2026-10-30

Added histogram and digit-subsequence fibers, bounded monotone transforms and explicit resource or pass-composition boundaries. The model distinguishes whole-sort order loss from pass-local subsequence retention. Three new Rust relation tests per algorithm reject explicit false premises. The implementation and public API remain unchanged. Models retain allocator uncertainty and scalar stability limits. No timing claim or approximate execution profile is introduced; supported caps remain at most 10%.

Finite confirmation: debug/release twice and strict Clippy passed; all six targeted compiled mutant checks showed red. The monotone relation alone admits a minimum-only implementation, so multiplicity-fiber checks remain independently necessary. See model.md#evidence for the preserved surviving countermodel and log hashes.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

#### Relation refinement checks 2026-10-30

Focused debug and release suites each ran twice with identical normalized per-test outcomes (durations excluded): six counting tests and seven radix tests. Strict crate Clippy, all targets with warnings denied, passed. Three new production-linked relation tests per module include explicit false-relation countermodels. The counting histogram test enumerates 256 inputs and admits 12 predecessors; the radix pass test enumerates 256 inputs and admits six.

Each new test separately rejected a compiled production mutant. Counting mutants: fill every bucket with minimum (histogram fiber), omit minimum from output value (monotone relabel), reject budget equality (budget fiber). Radix mutants: reverse source traversal in scatter (pass fiber and pass composition), omit low-byte pass (full-sort fiber and monotone relabel). Six compiled mutant runs each failed the selected test. An initial minimum-fill mutant survived the monotone-only test because taking minima commutes with increasing maps; this is retained as evidence that the metamorphic law alone does not prove sorting. The histogram test rejects it.

Production implementations and APIs are unchanged. Four source-bound syntax projections reproduced identically twice. Allocation exhaustion remains untested; counting deterministic capacity overflow has existing coverage. No timing gain or approximation claim.

External evidence SHA-256 (logs stored outside the repository):

- counting_sort-debug-1.log: c08ce4809adac081fe6551109a7bcb5db3adca0d1a8ebfa9c361f196138a501c
- counting_sort-debug-2.log: 07913ca41483f5d6690ce43373486311cab41c9501ed449302f98ef335029502
- counting_sort-release-1.log: 10b63efad7b72dc16d6dc7f9e3047a86cfda100eec8bfd3ac5fb9d9a3dae07fa
- counting_sort-release-2.log: 10b63efad7b72dc16d6dc7f9e3047a86cfda100eec8bfd3ac5fb9d9a3dae07fa
- clippy.log: ee1d3fffa2022e962e63e0dbe4d35c9c198450a20c29fc09030f635dbb38bfcd
- mutant-0.log: a642fdcebbbc244d94819cb2956fefea9a9f253c7ad43936779ca82bb05b4b45
- mutant-1.log: 6683faeb02de903f5e0dde796e3dea7b3ca1502254c5f2b9f6abff1ace85c089
- mutant-2.log: bda10496c7aa4d24a2df7792094444f14b9cb070e8bef9af4672f1a562a0483d

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.
