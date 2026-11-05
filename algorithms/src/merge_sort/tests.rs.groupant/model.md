# algorithm-merge-sort-tests

## descriptant

### Test extraction

The existing two inline tests were moved into adjacent tests.rs without changing their assertions. Two new tests extend the admitted fixture domain; production behavior is unchanged. Expected occurrence order is concatenated original equal-key subsequences in key order; it uses no recursive merging or standard sort.

The mathematical combiner reads Rust/partial syntax, finite acceptance relation, typed models and actual pass/fail observations. This epoch preserves earlier leaf evidence and adds successful repeated runs and a rejected source mutant. Failure fibers remain ambiguous; no automated proof or universal bug-localizer is claimed.

Changing left-on-equality <= to < is rejected by wider_stable_identity_buckets (compiled external copy, exit 101).

The shared parent must wire this test source as a registered sibling and refresh source bindings. Local models do not establish that integration. Macro bodies remain opaque in the partial syntax projection.

## model

# merge_sort test groupant

## Forward

For the explicit fixture set F, run existing and new assertions against super's implementation. Accept iff all observed outputs equal oracle expectations and metamorphic equalities hold. Execution assumes ordinary successful allocations and lawful traits.

Finite domain: Old exhaustive ternary stable identity inputs and boundaries; new fixed LCG seed 0x6a09e667f3bcc909, six lengths {3,17,33,65,129,513}, keys modulo 11; Reverse sequences of lengths {0,1,2,31,256}.

## Invariant

Production code is accessed through super::*; existing assertions are retained. Expected values do not call the algorithm under test except on the explicitly declared metamorphic sides. All generated values use deterministic arithmetic with explicit wrapping where intended.

## Cost

Six wider identity fixtures totaling 760 elements, each sorted twice; five Reverse fixtures totaling 290 elements. Bucket oracle O(K*n) with K<=97, deterministic fixed generators; no throughput claim.

## nonbehavior

Type: behavior_complement. Let F be the explicit finite fixture set below, O(f) the oracle expectations and U the candidate implementations paired with pass/fail observations over F. R(P,pass) iff every assertion succeeds under the fixed execution assumptions; R(P,fail) iff the terminating execution encounters an assertion mismatch or a caught panic. These two branches define R on the declared terminating, compiled-execution universe. U minus R consists of false pass observations for any mismatch and false failure observations when every assertion holds. Conditional exact harness model; panics count as failures, harness crashes/timeouts are outside this two-outcome universe.

## failure_fibers

Type: reverse_reconstruction. Given a failing fixture and observed mismatch, retain all candidate implementations whose observations violate that assertion. Wrong tie policy, wrong bounds, wrong arithmetic and fixture-oracle defects may share the same failure. Passing admits every implementation agreeing on this finite F and leaves all off-fixture behavior unconstrained; no unique fault reconstruction.

## oracle_independence

Type: oracle_correspondence. Expected occurrence order is concatenated original equal-key subsequences in key order; it uses no recursive merging or standard sort. Agreement is finite evidence, with oracle bugs still possible; the designated mutation is a falsifiable disagreement witness.

## transformations

Type: metamorphic_relation. A second sort must preserve every key/identity pair. Reverse wrappers require descending numeric key order while keeping the declared total-order contract. This relation holds only within admitted domains; it does not identify a unique input from output.

## finite_schedule

Type: resource_observation. Six wider identity fixtures totaling 760 elements, each sorted twice; five Reverse fixtures totaling 290 elements. Bucket oracle O(K*n) with K<=97, deterministic fixed generators; no throughput claim. The harness fixes fixture order, seeds and assertions; wall-clock duration and allocation behavior are not deterministic outputs.

## evidence

### Evidence

Four focused tests pass twice with identical normalized test statuses: two preserved tests and two new tests. Full crate check at this isolated base passes 36 tests plus one doctest. Owned-source rustfmt check and strict all-target crate Clippy pass. Changing left-on-equality <= to < is rejected by wider_stable_identity_buckets (compiled external copy, exit 101).

Old exhaustive ternary stable identity inputs and boundaries; new fixed LCG seed 0x6a09e667f3bcc909, six lengths {3,17,33,65,129,513}, keys modulo 11; Reverse sequences of lengths {0,1,2,31,256}.

Expected occurrence order is concatenated original equal-key subsequences in key order; it uses no recursive merging or standard sort.

Six wider identity fixtures totaling 760 elements, each sorted twice; five Reverse fixtures totaling 290 elements. Bucket oracle O(K*n) with K<=97, deterministic fixed generators; no throughput claim.

Commands: cargo test -p mova-algorithms --offline merge_sort::tests:: -- --test-threads=1 (twice); cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings. External mutation uses rustc --edition 2024 --test on a copied mod.rs with adjacent tests.rs, filtering only the named new test. Source production prefix is preserved; only test module placement changes. Parent registry and source-binding integration remain separate.

## History

### 2026-10-29 · descriptant

Heading `### Test extraction epoch 2026-10-29` stays in descriptant as `### Test extraction`.

### 2026-10-29 · evidence

Heading `### Evidence 2026-10-29` stays in evidence as `### Evidence`.
