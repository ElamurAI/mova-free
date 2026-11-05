# execution-native-correspondence

## descriptant

### Finite worker harness groupant

This fragment separates the native observation machinery from the worker implementation. Its combiner consumes the source slice, worker relations, independently constructed expectations and execution logs. The root keeps source hashes, source syntax and actual child ranges together. Useful epoch observation: callback values, callback effects and owned-output destruction require different assertions; a simple ordered-vector test covers only the first.

## model

# Worker correspondence harness

## Forward

The four test functions enumerate exact modulo-two map fibers, predict callback prefixes from the host partition, count owned output creation/drop, and project stateful callback outputs into ordered inputs plus a ticket multiset. These are separate finite observations of actual worker functions. The returned harness outcome is Pass only if every reached assertion holds; an assertion panic is Fail.

## Invariant

Source byte range covers correspondence_tests only. Parent source identity binds called functions separately; this test slice alone is not their implementation. Tests assume positive caps, stable available_parallelism, ordinary unwind and nonpanicking observation counters. The panic tests deliberately inject failures and catch the expected Result rather than requiring successful callbacks.

## Cost

Modulo fibers enumerate sum(4^n,n=0..3)=85 inputs for each of two caps, and compare sum(3^n,n=0..3)=40 output candidates per cap. Empty inputs create no workers. Remaining tests use bounded host-sized chunks and 31-ticket batches. Thread creation, joins and allocation are real fixture costs; they do not measure parser throughput.

## nonbehavior

Type: behavior_complement. In the universe of terminating instrumented harness executions, R obeys actual assertion semantics. The complement includes Pass with a mismatching fiber, skipped successful-chunk effects, an undropped created output, or a wrong returned input order. Only instrumented observables are modeled; a defect outside these observations may still pass.

## antecedents

Type: reverse_reconstruction. A passing result admits all implementations producing these finite projections, including incorrect implementations elsewhere. A failed assertion identifies a violated projection, not a unique algorithm defect, scheduling history or repair. Several panic payloads and chunk schedules share the same observed error string.

## observation-partition

Type: test_observation_partition. Output values, executed callback visits, owned-value drops and hidden-state tickets are four distinct projections. Equality of one does not establish another: empty returned results can coexist with completed side effects, and ordered indices can coexist with state-dependent values. The parent worker laws join only matching fixture identities.

## mutation-sensitivity

Type: oracle_sensitivity. Reversing callback order inside each chunk leaves cardinality and many multisets intact but changes pointwise output preimages. A source-extracted compiled mutant must fail the selected fiber test; this is one witnessed rejection, not mutation completeness. Native world tests separately bind the real module configuration.

## Evidence

Four native correspondence tests plus the two existing exec tests are run in debug/release. The source-extracted four-test reference and reversed-callback mutant have separate logs in the parent run directory. Exact execution counts and repeated results belong to ../model.md#tests. Parser, GPU, environment mode latch and model loading are outside these tests.
