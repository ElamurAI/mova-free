# algorithm-offline-dynamic-connectivity

## descriptant

### offline_dynamic_connectivity epoch memory

Retain the distinction between observation and internal representation: query ties preserve insertion identity; connectivity results preserve only partitions. The compiled faulty reduction/rollback controls establish sensitivity at named assertions. Original code follows the classic descriptions cited in module documentation; no copied solution.

No performance gain or approximate substitute claimed. These small files use file-level models, with no nested fragments. Source-bound partial syntax is generated twice identically; macro expansion and semantic AST remain absent. Allocation-failure behavior is not fault-injected.

## model

# offline_dynamic_connectivity mod.rs

## Forward

Validate caps, then timeline endpoints and canonical undirected edge lifecycle in chronological order. Add at a and Remove at b creates [a,b); an unremoved edge lasts to q. Place intervals in a time segment tree; rollback DSU unions exactly ancestors' edges. Emit reachability/reflexivity and component counts at query leaves, in order.

## Invariant

At each leaf the DSU partition equals connected components of active undirected edges. Canonical (min,max) identity rejects reversed duplicate additions. Self-loops have a lifecycle but induce no union. A successful union saves child root, parent root and former size; rollback restores these in reverse order with components+1. No path compression.

## Cost

n<=2^18 and q<=2^18. BTreeMap O(q log(q+1)); interval decomposition O(q log(q+1)) placements. Union by size bounds root depth O(log(n+1)), giving O(n+q log(q+1) log(n+1)) time and O(n+q log(q+1)) storage. Recursion depth O(log(q+1)); std allocator exhaustion is not injected.

## behavior_complement

Universe: valid timelines paired with query answer sequences. Exclude Connected(false) for equal valid vertices or for an active path, Connected(true) for disconnected vertices, and component counts other than the active graph's partition cardinality. Invalid endpoint/lifecycle traces paired with Ok are absent. A removed edge cannot affect later leaves without a new Add.

## reverse_reconstruction

Observed answers retain every valid timeline whose active graphs induce those observations. Edge sets are not recoverable: a triangle and its spanning tree have identical connectivity/component observations. With fixed timeline operations except unknown edges, retain all graph completions satisfying the observed equivalence relations; query-free histories all yield an empty vector.

## rollback_scope

An entry mark is the history length. After visiting both children, undo precisely the successful unions since that mark; redundant/self-loop unions add no records. A child-only edge must not leak into its sibling. Suppressing rollback compiles but rollback_and_lifecycle fails.

## lifecycle_partition

Caps are checked vertices first, then operation count. At each timeline step first invalid endpoint wins, then duplicate Add or absent Remove. Undirected reversal preserves identity; Remove ends activity before its own leaf. Empty vertices permit Components but no endpoint operation.

## component_information

Union of distinct roots lowers components by one, rollback raises it by one. Self-loops and redundant paths leave it fixed. Component counts discard labels and internal edge multiplicity; BFS on each independently reconstructed active graph observes the partition.

## evidence

### Evidence

4096 timelines with six toggles, 60 post-toggle queries and one initial component query each. BFS rebuild scans all e edges per dequeued vertex, O(n+n*e), under tiny fixed n; cap fixtures allocate up to 2^18 timeline entries. No timing benchmark.

Li Chao mutant changes query min reduction to max and fails exhaustive_envelopes. Connectivity mutant suppresses rollback and fails rollback_and_lifecycle. Six tests and two production controls total across the two algorithms. Finite evidence is not general correctness proof. No live parser/teacher/GPU, full-course or performance evidence.

External raw log hashes:

Structural receipts: four file sidecars, eighteen typed views (five per production file, four per tests file), zero fragments, four source-bound partial syntax projections. Each syntax artifact and groupant report generated twice identically. Temporary scaffold includes the base registry unchanged; no shared registry edits committed. Initial own-only scaffold was rejected for excluding existing ownership declarations, then corrected by including base nodes.

## History

### 2026-10-30 · descriptant

2026-10-30: choose exact bounded integer computation before implementation. n<=2^18 and q<=2^18. BTreeMap O(q log(q+1)); interval decomposition O(q log(q+1)) placements. Union by size bounds root depth O(log(n+1)), giving O(n+q log(q+1) log(n+1)) time and O(n+q log(q+1)) storage. Recursion depth O(log(q+1)); std allocator exhaustion is not injected.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: d26f4f177ad0ecbe9976c94498b21cc329d633b611cbb9eb8d8f7fa8d1f9c852. Three tests for this algorithm pass twice debug and twice release with identical normalized names/results. Strict all-target Clippy and narrow rustfmt pass. One production mutant per algorithm compiles and fails a named assertion, then exact source bytes are restored.

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
