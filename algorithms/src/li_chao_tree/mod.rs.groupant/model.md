# algorithm-li-chao-tree

## descriptant

### li_chao_tree epoch memory

Retain the distinction between observation and internal representation: query ties preserve insertion identity; connectivity results preserve only partitions. The compiled faulty reduction/rollback controls establish sensitivity at named assertions. Original code follows the classic descriptions cited in module documentation; no copied solution.

No performance gain or approximate substitute claimed. These small files use file-level models, with no nested fragments. Source-bound partial syntax is generated twice identically; macro expansion and semantic AST remain absent. Allocation-failure behavior is not fault-injected.

## model

# li_chao_tree mod.rs

## Forward

Given fixed sorted distinct coordinates X and insertion sequence L, query x in X returns lexicographic min (a*x+b,insertion_id) over L, evaluated exactly in i128. Unknown x returns UnknownCoordinate before NoLines. Constructor caps raw input before sorting/deduplication; cap-rejected insertion preserves tree and IDs.

## Invariant

Each occupied interval node stores its best surviving line at midpoint; a displaced line can beat it on at most one side, since the difference of affine functions has at most one crossing. Query's root-to-leaf candidates include the global minimum. Equal value uses insertion ID. i64*i64+i64 stays strictly within i128 limits; no wrapping.

## Cost

Raw n<=2^18, distinct m<=n. Sort O(n log(n+1)), storage 4m nodes plus m coordinates; insertion/query O(log(m+1)), recursion depth O(log(m+1)); insertion count<=2^20. Allocator failure follows std and is not a typed recoverable result.

## behavior_complement

Universe: admitted coordinate sets and insertion/query histories paired with candidate results. Exclude any successful unknown-coordinate query, any value below all inserted lines or above their minimum, or a later ID tied with an earlier minimum. Empty coordinate construction cannot succeed; empty-line registered query cannot succeed; cap-rejected insertion cannot consume an ID.

## reverse_reconstruction

Given fixed X, an observed (x,v,id) retains all line histories where line id evaluates to v at x, every other line has value>=v, and every earlier ID has value>v. Intercepts/slopes and nonwinning lines remain ambiguous: at x=0, lines (0,2) and (7,2) both give v=2. Across several queries intersect these constraints; dominated lines remain unrecoverable.

## tie_fiber

Lexicographic (value,id) ordering makes identical lines choose ID0, and crossing lines choose earlier insertion exactly at equality. Reversing the query reduction can return a dominated path candidate; exhaustive_envelopes rejects this compiled mutation.

## coordinate_projection

Sorting/deduplication maps [3,1,3] and [1,3] to the same domain. Raw cap checks occur before this projection, so repeated coordinates exceeding the cap still fail. Queries at unregistered points are rejected even when between registered extremes.

## numeric_domain

For all i64 a,x,b, |a*x|<=2^126 and adding b remains inside signed i128. Explicit MIN/MAX coefficient/coordinate fixtures compare literal wide arithmetic; no floating interpolation or saturation occurs.

## evidence

### Evidence

15625 triples, 234375 exact query comparisons, plus 2^20 identical insertions and accepted/rejected coordinate cap fixtures. Finite bounded test cost; no timing benchmark.

Li Chao mutant changes query min reduction to max and fails exhaustive_envelopes. Connectivity mutant suppresses rollback and fails rollback_and_lifecycle. Six tests and two production controls total across the two algorithms. Finite evidence is not general correctness proof. No live parser/teacher/GPU, full-course or performance evidence.

External raw log hashes:

Structural receipts: four file sidecars, eighteen typed views (five per production file, four per tests file), zero fragments, four source-bound partial syntax projections. Each syntax artifact and groupant report generated twice identically. Temporary scaffold includes the base registry unchanged; no shared registry edits committed. Initial own-only scaffold was rejected for excluding existing ownership declarations, then corrected by including base nodes.

## History

### 2026-10-30 · descriptant

2026-10-30: choose exact bounded integer computation before implementation. Raw n<=2^18, distinct m<=n. Sort O(n log(n+1)), storage 4m nodes plus m coordinates; insertion/query O(log(m+1)), recursion depth O(log(m+1)); insertion count<=2^20. Allocator failure follows std and is not a typed recoverable result.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 131fc4e32f41d0cbcf21be978aee039a96e98849ee9bcb410a6fa4740a9fc9eb. Three tests for this algorithm pass twice debug and twice release with identical normalized names/results. Strict all-target Clippy and narrow rustfmt pass. One production mutant per algorithm compiles and fails a named assertion, then exact source bytes are restored.

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
