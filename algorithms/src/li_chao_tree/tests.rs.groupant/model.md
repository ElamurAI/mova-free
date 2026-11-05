# algorithm-li-chao-tree-tests

## descriptant

### li_chao_tree epoch memory

Retain the distinction between observation and internal representation: query ties preserve insertion identity; connectivity results preserve only partitions. The compiled faulty reduction/rollback controls establish sensitivity at named assertions. Original code follows the classic descriptions cited in module documentation; no copied solution.

No performance gain or approximate substitute claimed. These small files use file-level models, with no nested fragments. Source-bound partial syntax is generated twice identically; macro expansion and semantic AST remain absent. Allocation-failure behavior is not fault-injected.

## model

# li_chao_tree tests.rs

## Forward

Execute all 25^3 ordered triples of slopes/intercepts in -2..=2. After each insertion query five coordinates and compare literal enumeration of all inserted lines, retaining earliest ID. Add MIN/MAX, duplicate-coordinate, empty/unknown, cap and tie fixtures.

## Invariant

Fixture order and integer arithmetic deterministic; independent oracles do not invoke production helpers. Test outcome observes scheduled assertions only, not all possible inputs or allocation effects.

## Cost

15625 triples, 234375 exact query comparisons, plus 2^20 identical insertions and accepted/rejected coordinate cap fixtures. Finite bounded test cost; no timing benchmark.

## behavior_complement

Universe: implementations paired with normal/panic outcomes on the fixed fixtures. Normal completion is excluded if a returned value/ID disagrees with direct enumeration, if a tie selects a later ID, or boundary errors/cap state preservation differ. Unsampled implementations remain unconstrained.

## reverse_reconstruction

Normal completion retains all implementations agreeing on the finite observations, including different algorithms and faults outside these fixtures. A failing named assertion yields a concrete witness, not a unique faulty source reconstruction. Finite success does not recover implementation identity.

## oracle_independence

Execute all 25^3 ordered triples of slopes/intercepts in -2..=2. After each insertion query five coordinates and compare literal enumeration of all inserted lines, retaining earliest ID. Add MIN/MAX, duplicate-coordinate, empty/unknown, cap and tie fixtures.

## control_scope

One compiled production mutation is required to fail a named assertion. Exact saved bytes are restored before repeated positive runs. This witnesses sensitivity to that fault only; no exhaustive fault detection follows.

## evidence

### Evidence

15625 triples, 234375 exact query comparisons, plus 2^20 identical insertions and accepted/rejected coordinate cap fixtures. Finite bounded test cost; no timing benchmark.

Li Chao mutant changes query min reduction to max and fails exhaustive_envelopes. Connectivity mutant suppresses rollback and fails rollback_and_lifecycle. Six tests and two production controls total across the two algorithms. Finite evidence is not general correctness proof. No live parser/teacher/GPU, full-course or performance evidence.

External raw log hashes:

Structural receipts: four file sidecars, eighteen typed views (five per production file, four per tests file), zero fragments, four source-bound partial syntax projections. Each syntax artifact and groupant report generated twice identically. Temporary scaffold includes the base registry unchanged; no shared registry edits committed. Initial own-only scaffold was rejected for excluding existing ownership declarations, then corrected by including base nodes.

## History

### 2026-10-30 · descriptant

2026-10-30: choose exact bounded integer computation before implementation. 15625 triples, 234375 exact query comparisons, plus 2^20 identical insertions and accepted/rejected coordinate cap fixtures. Finite bounded test cost; no timing benchmark.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 18732adb4526230276350a3c3a452f515e061ec84a5ac4c6956e604659c4e05f. Three tests for this algorithm pass twice debug and twice release with identical normalized names/results. Strict all-target Clippy and narrow rustfmt pass. One production mutant per algorithm compiles and fails a named assertion, then exact source bytes are restored.

- li_chao_tree-debug-1.log: 687640980e1bf24e31543834663e3c5cbe1e5a7bf2f6cc1222b97853776fe233

- li_chao_tree-debug-2.log: 72852b921325550691d13252bc4c347c280f77b7e63f8b16dec50db5d1363de2

- li_chao_tree-release-1.log: 6d888dbb1cf62c740a1a7a550c6af192310831d058877a028d485b243514a7b9

- li_chao_tree-release-2.log: f2a4cd2701f54bb3d33a54356ef487d748e7398b73ba2188c6a3c6a6ac412fb3

- mutant-li_chao_tree.log: 9a3bd984ae011320809e9937a4af599b90a46409e39c402374477d59448858a4

- clippy.log: 999837d8357e2d33f39923fa376807b31cc10864ed60f5c06e089e5eae83c248

- check.log: 3b85fdb7533db33eb312c32b2094a66227d8ed4399e049ad5930461499eaf4de

- groupants-1.json: 07c0b20d6224715f6344d25485b47d19a0bca2cf20e787b5cd4848c073d4d3cf

- groupants-2.json: 07c0b20d6224715f6344d25485b47d19a0bca2cf20e787b5cd4848c073d4d3cf

- changed.json: 2684631f0c0110701eb8bfc5d8a70f9973f8634f19c5aa6c24c59a7e6f377c15
