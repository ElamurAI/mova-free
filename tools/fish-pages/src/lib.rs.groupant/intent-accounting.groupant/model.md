# Intent/accounting fragment

## domain
Source scope: plans/admit and bounded Pin/config parsing in the parent Rust file. Inputs are pinned global intents and one producer admission; observations are source ownership and prelaunch refusal. This is a partial projection of the parent's syntax, not a separate implementation.

## forward
Verify disjoint source ownership across all intents, then require actual pinned Task target IDs, deferred and rejected source IDs to account for the current intent exactly once. Each intent contains1..256 sources; tasks have1..8 targets. The ready file and task bytes remain bound by independent SHA pins.

## behavior-complement
Within this domain, launch admission is excluded by missing/foreign/duplicate IDs, overlap between queued targets and gaps, unexplained gaps, invalid pins or unsafe intent paths. Formula: admission implies Q disjoint-union D disjoint-union R=I. Counterexample: adding a correct target to both Q and D must fail even when its AST is valid. Structural completeness says nothing about omitted corpus IDs outside the declared plan. Evidence: fake Rust duplicate/unaccounted/gap fixtures.

## reverse-reconstruction
A complete partition determines source-accounting membership, but many producer histories and annotation bytes can yield it. Deferred(reason) does not uniquely reconstruct a failed parser state; the bound artifact is needed. Observing a tiny complete plan cannot recover the absent larger corpus. Assumptions: immutable pins and honest producer provenance; no semantic or global-completeness proof.

## invariant
Every intended ID enters exactly one prelaunch account and source counts remain distinct from job counts. No truncation, native-label invention, database mutation or training permission occurs here.

## cost
Global ownership indexing O(N log N), current page membership O(M log M) plus bounded task bytes, M<=256. These are symbolic costs, not measured speed/error percentages.

## composition
Producer intent -> ready page -> actual pinned Task/gap partition -> imported swarm. Native syntax/label semantics are checked downstream; held-out/source-phase selection remains a producer obligation.

## checks
Shares the parent's eight finite fake Rust fixtures, especially duplicate, foreign/unaccounted and explicit-gap controls. No independent full corpus or live-provider test.
