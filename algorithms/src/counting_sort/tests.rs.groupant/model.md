# algorithm-counting-sort-tests

## descriptant

### counting_sort tests.rs epoch memory

Three tests pass in debug and release. All 21,845 arrays of lengths 0..7 over four values match std slice sorting. An 8,192-value fixed-seed array also matches. Boundary tests cover empty/zero budget, high minimum, range overflow, budget rejection and deterministic Vec capacity overflow; every error preserves input. A compiled mutant that fills each bucket with minimum rather than minimum+offset is rejected by boundaries_and_errors_are_atomic.

The combiner relates raw source, partial syntax, forward relation, typed obernants, invariant, cost and finite evidence. Preserve predictions and rejected mutants across future epochs. Internal radix byte stability is observable via distinct values sharing a digit; whole-sort payload stability is not exposed. Missing universal proof, allocator-exhaustion injection and downstream wiring remain explicit. Exact algorithms require no invented approximant profiles. Future executable approximants must honor 1%, 2%, 5%, 10% caps and source/evidence binding.

## model

# counting_sort tests.rs model

## Forward

Finite deterministic tests compare the associated exact implementation with standard comparison sorting and explicit boundary fixtures. Initial epoch: three tests pass in debug and release. All 21,845 arrays of lengths 0..7 over four values match std slice sorting. An 8,192-value fixed-seed array also matches. Boundary tests cover empty/zero budget, high minimum, range overflow, budget rejection and deterministic Vec capacity overflow; every error preserves input. A compiled mutant that fills each bucket with minimum rather than minimum+offset is rejected by boundaries_and_errors_are_atomic.

## Invariant

Exhaustive enumeration assigns each radix-four code to exactly one length-bounded input. Expected vectors are independently cloned and sorted by std comparison sorting. Failure assertions compare with original input. Tests do not mutate production source.

## Cost

Bounded enumeration sum_{k=0}^7 4^k = 21,845 cases plus fixed fixtures and 8,192-element oracle. Oracle cost O(n log n); test resource counts are finite, not throughput evidence.

## nonbehavior

Type: behavior_complement. In the universe of implementation behaviors and test outcomes, a passing outcome excludes disagreements observed by the enumerated fixtures and their independent oracle. Untested behaviors remain compatible; passing does not exclude every incorrect implementation.

## preimages

Type: reverse_reconstruction. A passing test outcome admits all implementations agreeing on the finite observed fixtures, including implementations differing elsewhere. A failed assertion constrains a compatible mismatch to its reported fixture. The test outcome cannot reconstruct unique implementation code.

## idempotence

Type: metamorphic_relation. Repeated sorting and fixed-seed replay retain the same observed vectors; this checks a bounded relation.

## relation-observation

Type: observational_equivalence. Passing relation tests admits implementations agreeing on the enumerated histogram or digit-subsequence fibers, mapped fixtures and resource or pass-order observations. It excludes only detected mismatches; whole-code equivalence and untested allocator outcomes remain unknown.

## countermodel-sensitivity

Type: counterfactual_model. The tests deliberately reject support-only inverse fibers and transformations with dropped domain premises. Compiled production mutants are tested separately against the new relation assertions. Their rejection establishes sensitivity to those faults, not completeness against all incorrect implementations.

## evidence

### Evidence

Three tests pass in debug and release. All 21,845 arrays of lengths 0..7 over four values match std slice sorting. An 8,192-value fixed-seed array also matches. Boundary tests cover empty/zero budget, high minimum, range overflow, budget rejection and deterministic Vec capacity overflow; every error preserves input. A compiled mutant that fills each bucket with minimum rather than minimum+offset is rejected by boundaries_and_errors_are_atomic.

Strict crate Clippy and owned rustfmt checks pass. Two deterministic debug executions have matching normalized test outcomes. Source-derived syntax is generated twice and compared. No timing, downstream wiring or universal verification claim.

## History

### 2026-10-30 · descriptant

2026-10-30. Repository search found no matching implementation in algorithms/src or math/src. Original English std-only code; no external solution imports. Prospective mathematics preceded implementation and remains in ../mod.rs.groupant/model.md#prospective.

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
