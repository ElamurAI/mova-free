# algorithm-heap-sort-tests

## descriptant

### heap_sort tests groupant

Combiner: source identity, recurrence, invariant and checks constrain a
conditional model. Complement and converse answer different questions; no
additive inverse identity. Syntax is partial with opaque macros and no type
or name resolution. Finite evidence is not proof of all inputs or parent wiring.
Lawful Ord and successful execution are assumed; trait panics and allocation
failures are outside successful-call semantics. Preserve these observations
in future epochs. No runtime speedup claim.

## approximant

### Forward, invariant and cost

Generate bounded fixtures, compute independent references, assert agreement
and separate structural properties. References do not consult production
outputs. Count each symbol in ternary arrays and concatenate ordered buckets independently of heap operations.

Assumptions: Ascending result and preserved counts; custom Reverse order is lawful and requires no Clone.

Coverage: 3280 ternary arrays length 0..7; 2053 full-width seeded u64 entries sorted repeatedly; empty/singleton/1024 equal entries.
Test oracle costs are described in model.md#models; production cost: O(n log n) time, O(1) auxiliary space; lawful Ord and constant comparison/swap assumed.

## models

# heap_sort test models

Subject/revision: tests.rs at manifest SHA. Observe one test invocation under
lawful Ord and ordinary allocation. Partial source-derived models have bounded
evidence; no universal proof or arbitrary panic behavior is modeled.

## behavior_complement

U=(fixture, candidate result, verdict). Pass with violated oracle equality
or explicit property assertion is absent from the assertion graph. This exact
complement slice does not exclude implementations overfit to tested fixtures.
The compiled mutation (choose smaller child instead of larger child) is rejected by one exact filtered test.

## reverse_reconstruction

Passing fixtures constrain implementation outputs/properties only on this
finite domain; all agreeing functions remain compatible. Failure identifies
a counterexample, not a unique bug or source. The mutant is one member of
a failure fiber. Complete source reconstruction is not implemented.

## independent_oracle

Count each symbol in ternary arrays and concatenate ordered buckets independently of heap operations.
Independent control flow can still share contract misunderstandings; bounded
correspondence is evidence rather than a proof of all inputs.

## property_assumptions

Ascending result and preserved counts; custom Reverse order is lawful and requires no Clone.
Dropping lawful order or changing strictness invalidates these targets.
Witnesses and custom-order tests exercise only the declared bounded projection.

## bounded_coverage_cost

3280 ternary arrays length 0..7; 2053 full-width seeded u64 entries sorted repeatedly; empty/singleton/1024 equal entries.
Seeds use wrapping arithmetic. Generation is exponential in a fixed small
bound. Reference vectors add O(n) space; reference sorting work is separate
from the algorithm under test. Production cost: O(n log n) time, O(1) auxiliary space; lawful Ord and constant comparison/swap assumed.

## checks

### Evidence

Four tests for this algorithm passed twice with temporary local crate exports:
66 total tests, identical normalized outcomes. Strict all-target Clippy and
rustfmt passed. Temporary exports are not committed.

Coverage: 3280 ternary arrays length 0..7; 2053 full-width seeded u64 entries sorted repeatedly; empty/singleton/1024 equal entries.

External mutant (choose smaller child instead of larger child) compiled; exactly one filtered new exhaustive test
failed an assertion with exit 101. No mutant committed. One red control is not
mutation completeness. Parent owns final exports, registry and source bindings.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective predictions saved outside repository before
implementation. Original std-only code/fixtures; four algorithm tests pass
twice. Compiled red control: choose smaller child instead of larger child.

### 2026-10-29 · checks

Heading `### Evidence, 2026-10-29` stays in checks as `### Evidence`.
