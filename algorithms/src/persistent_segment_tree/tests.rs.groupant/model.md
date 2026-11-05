# algorithm-persistent-segment-tree-tests

## model

# Persistent tree finite checks

## domain
Lengths 1..17, 24 deterministic updates each, branching from prior versions.
Every historical version and every interval is queried after each update.
Additional empty, invalid-ID/index/range, i64 extremes and defensive-overflow
fixtures. The defensive test deliberately injects an unreachable public state.

## forward
Clone selected Vec snapshot, replace one element, retain every previous Vec.
For every historical version compute each interval sum by independent i128
iteration. Compare previous arena prefix exactly and bound new node count.
Invalid calls compare exact root/arena vectors. Wide-sum fixture checks
cancellation and values beyond i64. Inject MAX into one internal node to
exercise checked-add rollback after allocation of a replacement leaf.

## invariant
Oracle snapshots share no mutable vectors. Expected sums never traverse
production children. Historical observations are checked repeatedly after
branching updates, not only immediately after construction.

## cost
408 updates plus fixtures. Per length n, checking all histories after u
updates uses O(u^2 n^3) naive oracle summation and O(u^2 n^2 log n) tree work.
u=24 and n<=17 bound the suite. Snapshot storage O(un). No large-memory,
concurrency or allocation-failure coverage is claimed.

## behavior-complement
Type behavior_complement. Universe pairs implementations with finite
verdicts. Exclude pass for any mismatch between queried history and Vec
snapshot, any arena-prefix mutation, incorrect node growth, or changed
arena/roots after a checked error in the fixtures. Passing outside this
bounded trace remains unconstrained.

## reverse-reconstruction
Type reverse_reconstruction. A passing verdict identifies a class of programs
agreeing on all sampled traces, not unique implementation. A failing current
sum with correct historical sum admits update-value, path or aggregation
defects. An exact prefix mismatch narrows the fiber to mutated stored state
but does not identify the mutation mechanism by itself.

## independent-history
Type snapshot_trace_relation. Each reference version is a full cloned Vec
with one assignment, independent of path-copy arena layout and child IDs.
All ranges expose prefix, suffix, singleton and cross-split sums.

## rollback-observation
Type exact_state_frame. Invalid version is checked before index/range.
State comparisons use full Node and root vectors. The deliberately corrupted
overflow fixture checks defensive truncation only, not public reachability
or normal arithmetic overflow on supported <=64-bit targets.

## allocation-bound
Type finite_path_budget. Assert initial 2n-1 and per-update growth bounded
by ceil(log2 n)+1; compare prior node prefix to detect rewriting despite
correct latest-version answers. Non-power-of-two lengths distinguish shape
assumptions that powers alone cannot exercise.

## tests
Five tests pass twice in each profile. Compiled replacement mutant adds one
to leaf values; replacement_control expects 11 and fails by assertion. Exact
source restoration and positive rerun recorded in model.md#evidence.

## descriptant
Independent snapshots, full historical observation and exact arena-prefix
comparison check complementary properties. Internal overflow injection has
a narrower scope than the public API tests and is labeled accordingly.
Partial syntax is not semantic AST/proof. Numerical approximation of verdicts
would hide contract failures; no certified surrogate is offered.

## evidence

### Evidence

5 tests pass twice debug and twice release. Normalized named outcomes match within each profile. Compiled control replacement_control: replacement leaf value incremented by one fails by named assertion (exit 101), then source is restored byte-for-byte and all focused tests pass. Syntax projections run twice and compare byte-for-byte.

## root-wave18

### Root integration refinement

## History

### 2026-10-30 · document

2026-10-30. Original Vec snapshot oracle and boundary fixtures.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

External logs: $MOVA_DATA/runs/octopus-wave18-structures-2026-10-30/. Finite tests provide no universal proof or timing guarantee.

Source algorithms/src/persistent_segment_tree/tests.rs: sha256 b904b01d146a1a03631be251d811c15d6453e82edd79a2ee1b4b0de72a9e7112

- persistent_segment_tree-debug-1.log: sha256 17bc81ad86efc73497bb9408b21afdf2cd42b8413ae9e661efbc35e5cc0a02c5

- persistent_segment_tree-debug-2.log: sha256 41fa8b446fe847a65dae9b03eae10396e8f2444b46ae673e6fe2caaedcd43063

- persistent_segment_tree-release-1.log: sha256 29bc9d8f14663c5a85a0b56708d920494dabc618d879c33dd0039b278bdc3c13

- persistent_segment_tree-release-2.log: sha256 e2de0a8a92e7e1bcc389e4d67ee19ee974b4507bbf67d03453aa80195925f9d0

- persistent_segment_tree-mutant.log: sha256 45034d1881c87296bbc863b173cfa9c9cc944702199d1794324462e71c7b7488

- persistent_segment_tree-restored.log: sha256 10faf1f126daeeb9b952938663ec0fe546c3a7ca1cad658ac0b71b92f5662cbc

- syntax.json: sha256 48bae6fcbf96652741389ae60b7f5d15849936fd26c5a0d6644cda3e3d210635

### 2026-10-30 · root-wave18

Heading `### Root integration refinement — 2026-10-30` stays in root-wave18 as `### Root integration refinement`.

Strict all-target Clippy exposed a test-only style issue after the owner library-only check. The source now uses the equivalent standard slice/divisibility helper. Old test source SHA-256: b904b01d146a1a03631be251d811c15d6453e82edd79a2ee1b4b0de72a9e7112; current SHA-256: 601ff003b0cc92195150b1c21fd0838aec9d8d3805e20fd7f7c83ce53c3d861d. Owner receipts remain historical and are not rewritten. Final root execution is recorded in models/algorithm-library.groupant/model.md#wave18-evidence; raw receipts are under $MOVA_DATA/runs/octopus-wave18-integration-2026-10-30/native-final. Syntax was projected twice identically after this edit. No production algorithm changed.
