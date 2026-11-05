# algorithm-offline-dynamic-connectivity-tests

## descriptant

### offline_dynamic_connectivity epoch memory

Retain the distinction between observation and internal representation: query ties preserve insertion identity; connectivity results preserve only partitions. The compiled faulty reduction/rollback controls establish sensitivity at named assertions. Original code follows the classic descriptions cited in module documentation; no copied solution.

No performance gain or approximate substitute claimed. These small files use file-level models, with no nested fragments. Source-bound partial syntax is generated twice identically; macro expansion and semantic AST remain absent. Allocation-failure behavior is not fault-injected.

## model

# offline_dynamic_connectivity tests.rs

## Forward

Execute all 4^6 edge-toggle sequences over three vertices (including a self-loop), querying all nine ordered pairs and component count after each toggle. Rebuild active-edge BFS labels independently per query. Add lifecycle, empty-domain, rollback and cap witnesses.

## Invariant

Fixture order and integer arithmetic deterministic; independent oracles do not invoke production helpers. Test outcome observes scheduled assertions only, not all possible inputs or allocation effects.

## Cost

4096 timelines with six toggles, 60 post-toggle queries and one initial component query each. BFS rebuild scans all e edges per dequeued vertex, O(n+n*e), under tiny fixed n; cap fixtures allocate up to 2^18 timeline entries. No timing benchmark.

## behavior_complement

Universe: implementations paired with normal/panic outcomes on the scheduled timelines. Normal completion is excluded when any connectivity/component answer disagrees with the independent active-edge BFS or literal rollback/lifecycle/cap assertions. Unsampled histories remain unconstrained.

## reverse_reconstruction

Normal completion retains all implementations agreeing on the finite observations, including different algorithms and faults outside these fixtures. A failing named assertion yields a concrete witness, not a unique faulty source reconstruction. Finite success does not recover implementation identity.

## oracle_independence

Execute all 4^6 edge-toggle sequences over three vertices (including a self-loop), querying all nine ordered pairs and component count after each toggle. Rebuild active-edge BFS labels independently per query. Add lifecycle, empty-domain, rollback and cap witnesses.

## control_scope

One compiled production mutation is required to fail a named assertion. Exact saved bytes are restored before repeated positive runs. This witnesses sensitivity to that fault only; no exhaustive fault detection follows.

## evidence

### Evidence

4096 timelines with six toggles, 60 post-toggle queries and one initial component query each. BFS rebuild scans all e edges per dequeued vertex, O(n+n*e), under tiny fixed n; cap fixtures allocate up to 2^18 timeline entries. No timing benchmark.

Li Chao mutant changes query min reduction to max and fails exhaustive_envelopes. Connectivity mutant suppresses rollback and fails rollback_and_lifecycle. Six tests and two production controls total across the two algorithms. Finite evidence is not general correctness proof. No live parser/teacher/GPU, full-course or performance evidence.

External raw log hashes:

Structural receipts: four file sidecars, eighteen typed views (five per production file, four per tests file), zero fragments, four source-bound partial syntax projections. Each syntax artifact and groupant report generated twice identically. Temporary scaffold includes the base registry unchanged; no shared registry edits committed. Initial own-only scaffold was rejected for excluding existing ownership declarations, then corrected by including base nodes.

## History

### 2026-10-30 · descriptant

2026-10-30: choose exact bounded integer computation before implementation. 4096 timelines with six toggles, 60 post-toggle queries and one initial component query each. BFS rebuild scans all e edges per dequeued vertex, O(n+n*e), under tiny fixed n; cap fixtures allocate up to 2^18 timeline entries. No timing benchmark.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: c7f0e9526f087da03de94a426998d0db70f6d8db2b84a4e217647c2fea413380. Three tests for this algorithm pass twice debug and twice release with identical normalized names/results. Strict all-target Clippy and narrow rustfmt pass. One production mutant per algorithm compiles and fails a named assertion, then exact source bytes are restored.

- offline_dynamic_connectivity-debug-1.log: 0194c70d5b748f5eaef81830ff6f4e2ee78576409109add2536f1408065a4a43

- offline_dynamic_connectivity-debug-2.log: 0158f891405ca016ab33515d72ce027c6fca7117f362f7f13fb100414b086c99

- offline_dynamic_connectivity-release-1.log: 8df5b2c988b247305e1e2d5d8bf8b089622588e351e207365e160d0c6cb599fb

- offline_dynamic_connectivity-release-2.log: 3989ee10b0d45e459fa6bf0d4e2ade27a98987bf36cac7dea3913c370fd4748d

- mutant-offline_dynamic_connectivity.log: ba0c6f9a4d0e116a70716150b073ea98f35d69c44465e94ae70bf2cebbc88169

- clippy.log: 999837d8357e2d33f39923fa376807b31cc10864ed60f5c06e089e5eae83c248

- check.log: 3b85fdb7533db33eb312c32b2094a66227d8ed4399e049ad5930461499eaf4de

- groupants-1.json: 07c0b20d6224715f6344d25485b47d19a0bca2cf20e787b5cd4848c073d4d3cf

- groupants-2.json: 07c0b20d6224715f6344d25485b47d19a0bca2cf20e787b5cd4848c073d4d3cf

- changed.json: 2684631f0c0110701eb8bfc5d8a70f9973f8634f19c5aa6c24c59a7e6f377c15
