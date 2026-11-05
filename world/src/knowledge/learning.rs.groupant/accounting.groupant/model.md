# learning-memo-accounting

## model

# accounting fragment

Source coverage: LearnWork declaration and accumulation implementation, at the manifest digest. This is a source-derived partial model, not a new runtime unit.

## Forward approximant

Represent each report as (e,r,i,n,t), where t has exactly n entries; equality retains length and trailing zeros. For nonoverflowing inputs x and y, add yields component count sums, n=max(n_x,n_y), and t_j=x_j+y_j for 0<=j<n, interpreting absent entries as zero. This aggregates reports; it does not execute visits.

## Invariant

The default report (zero counts, empty timing vector) is the identity. Addition is associative and commutative on nonoverflowing inputs, including exact vector shape: max is associative and commutative. All-zero nonempty timing vectors are not identities on the full domain. In particular []+[0]=[0], which differs from []. There is no trailing-zero canonicalization. Aggregation destroys attribution to calls.

## Cost approximation

O(max(n_x,n_y)) time including resizing, O(max(0,n_y-n_x)) additional retained timing entries.

## Behavior complement

Let U contain nonnegative input pairs and output reports with nonoverflowing arithmetic, and R be the actual relation on U. An inner approximant to U \\ R contains outputs whose count differs from its component sum, whose length differs from max(n_x,n_y), or whose present timing coordinate differs from its padded sum. This excluded relation is not merely a list of tests. Overflow executions are outside this model.

## Reverse reconstruction

For z, B(z) contains pairs (x,y) satisfying the count sums, max(n_x,n_y)=n_z, and padded timing sums at every coordinate of z. The length constraint is essential: [0] and [] denote distinct observations. In particular an empty output requires both timing vectors empty. Multiple partitions, including which input contributed a zero suffix, can produce the same report. Count and timing aggregates cannot uniquely recover lessons, call order, or per-call attribution.

## Specialized relation

Timing vectors align by pass index, not wall-clock order. Recovering per-lesson durations requires external provenance. These shape claims do not imply that a pass took zero real nanoseconds: the tests use synthetic reports.

## Epoch memory and limits

## tests

### Bounded accounting correspondence

Definitions live in ../../learning.rs and call the production LearnWork::add.

- accounting_add_matches_independent_table checks sixteen ordered pairs against literal expected reports: independent evaluated/reused/inert coordinates, empty timing vectors, explicit zeros, unequal lengths and trailing zeros.
- accounting_add_preserves_ledger_under_all_three_call_partitions assigns six independent event records to three calls in all 729 ways. Each call's input is constructed by counting event kinds and binning synthetic durations, without add. Both associations must produce the fixed observation (3,2,1; [11,7,5]); both default identity directions preserve each input's exact shape.
- accounting_add_bounded_preimages_preserve_shape_and_lose_attribution enumerates 56 reports (three binary counters and seven timing vectors of length at most two) and all 3,136 ordered pairs. Commutation holds on every pair. A declarative converse for (1,1,0; [1,0]) agrees with actual add on every candidate and admits exactly 32 ordered preimages: four count partitions times eight shape-aware timing partitions. Two explicit witnesses differ in call attribution and zero-suffix ownership. An explicit [0] output differs from the default empty vector.

The existing bounded_cache_reference_traces_cover_weights_orders_seeds_and_passes retains its 270 per-call cache cases. Its simplified evaluator does not establish full parser/inference correspondence.

Run from the repository root:

`cargo test -p world knowledge::learning --lib --locked`

Negative control: temporarily change the production timing resize from max to min. The focused literal-table oracle fails with exit 101 at pair 0,1: actual timing [] versus expected [0]. Restore the source byte-for-byte before the second seven-test pass. The mutation is absent from the committed source.

Limits: correspondence is bounded, uses synthetic durations, and excludes integer overflow. It neither measures real elapsed-time accuracy nor proves unrestricted arithmetic laws. Source hashes and AST projections are refreshed by integration after this test-only source addition.

## History

### 2026-10-29 · Epoch memory and limits

2026-10-29: decomposed existing source without moving code. Existing root observations and all twenty root obernants remain authoritative. Child anchors select source-bound partial syntax nodes; name resolution and macro expansion remain absent.

2026-10-29 prospective correspondence: sixteen independently tabulated ordered pairs will cover empty, trailing-zero and unequal-length shapes and isolated count coordinates; 729 assignments of six ledger records to three calls will retain the literal aggregate (3,2,1; [11,7,5]) under both associations. Default identity and commutation must hold without discarding shape. Distinct partitions must demonstrate ambiguous reverse reconstruction. Values are small, so overflow is excluded. Tests will call the actual add and a temporary source mutation must make a focused oracle fail. No production semantic change or measured performance gain is intended.

2026-10-29 measured correction and correspondence: the shape-preserving model above replaces the earlier coordinate-only reverse relation; empty and explicit-zero vectors cannot be silently identified. All sixteen literal pairs and all 729 ledger partitions passed. A separate 56-report domain yielded 3,136 ordered pair checks and exactly 32 compatible preimages of (1,1,0; [1,0]). Commutation held on that domain. The seven-test learning scope passed twice; a production max-to-min resize mutation failed the literal table on empty plus explicit zero, then the original source was restored. These measurements support only the bounded nonoverflowing observations in model.md#tests; production semantics were unchanged.

### 2026-10-29 · tests

2026-10-29 measured: two restored-source runs each passed all seven selected tests (126 filtered out), with the same named test outcomes. CPU test time was 0.02 s and 0.03 s; timing equality is not claimed. No teacher or GPU was used.
