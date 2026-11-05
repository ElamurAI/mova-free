# algorithm-shift-and-tests

## descriptant

### Description and epoch memory

Prospective design: use a single u64 state and fixed byte masks for Shift-And; eliminate candidate intervals without a doubled buffer for minimum rotation. Predicted costs: O(256+m+n+k) time; 256 u64 masks and constant state, plus k returned usize offsets. No arbitrary text cap; allocator exhaustion is not a typed error. No benchmark or speed claim is made.

Retained observations: empty-pattern terminal boundaries, top-bit masks and periodic earliest ties require explicit fixtures; a result index or match vector loses input identity. Saturated rotation advancement denotes exhaustion, not a wrapped candidate.

Classical attribution is in mod.rs: Baeza-Yates and Gonnet, CACM 35(10), 1992, 74–82; Booth, IPL 10(4–5), 1980, 240–242. No copied implementation. Partial syntax excludes macro expansion, name resolution and semantic AST. Finite observations and structural acceptance do not constitute a proof.

## approximant

### Forward, invariant and cost

For text t and pattern p, reject |p|>64 with PatternTooLong{|p|}; otherwise output ordered M={i:0<=i<=|t|-|p| and t[i..i+|p|]=p}. Empty p yields all boundaries, including |t|.

After consuming position j, bit q of state is set iff p[0..=q] equals the suffix of t[0..=j] of length q+1. Mask shifts are 0..63; advancing shifts by 1 and discards overflow bits. Accepting bit can only be set after m bytes, so output subtraction cannot underflow.

O(256+m+n+k) time; 256 u64 masks and constant state, plus k returned usize offsets. No arbitrary text cap; allocator exhaustion is not a typed error.

Test observation: each named test maps to success or an assertion failure. Success requires equality against independent window enumeration or materialized-and-sorted rotations plus literal boundary expectations. The finite suite covers only its stated domains; it is not the production function's entire domain.

The test oracle uses independent window enumeration (132253 text-pattern pairs) or sorting all materialized rotations (9841 ternary strings through length eight). Test-oracle cost is O(n*m) per matcher case and O(n^2 log n) per rotation case, including materialization. These oracle costs do not describe production complexity.

## models

# Typed models

All entries bind the adjacent manifest source revision. Domain and observation are stated per relation; these are source-derived specifications with finite correspondence evidence, not proofs. Test observation: each named test maps to success or an assertion failure. Success requires equality against independent window enumeration or materialized-and-sorted rotations plus literal boundary expectations. The finite suite covers only its stated domains; it is not the production function's entire domain.

## behavior_complement

Universe: finite byte-slice pairs and Result<Vec<usize>,PatternTooLong> outcomes. Absent outcomes include any success for m>64, any error for m<=64, any unsorted or duplicate offsets, any missing true match, any reported nonmatching window, and empty-pattern vectors missing even the terminal boundary. These conditions characterize the complement of the stated exact relation, subject to successful allocation.

At the test-run level, excluded passes include agreement with a nonoverlap false oracle or the later periodic minimum; oracle_control_and_fibers explicitly rejects those alternatives. The independent oracle must remain fixed when evaluating a production mutant.

## reverse_reconstruction

For a fixed pattern and observed offsets, retain all texts whose matching-window positions are exactly those offsets, including texts of different lengths when compatible. An empty result for pattern b is compatible with aaa and ccc. Error length L retains all texts and every pattern of length L>64; byte contents are lost. No unique inverse or executable complete reconstructor is supplied.

A passing test transcript admits many untested implementations, for example one with altered behavior only beyond the finite maximum. A failing assertion narrows the input/output discrepancy but does not uniquely identify the bug. Test inputs cannot be recovered from a success bit.

## accepting_bit

For 1<=m<=64, acceptance is state & (1<<(m-1)) != 0. The 64-byte fixtures observe the top bit directly; shifting by m would lose this constraint. Status: source-derived mathematical relation; checked by the adjacent named tests on bounded inputs, with no general semantic proof.

## overlap_trace

For pattern aaa and text aaaaa, the exact offsets are [0,1,2]; resetting state after a match is a counterfactual nonoverlap policy and is rejected by word_boundaries_and_overlaps. Status: source-derived mathematical relation; checked by the adjacent named tests on bounded inputs, with no general semantic proof.

## length_partition

Validation partitions all patterns at 64 before reading text. For m=65, both empty and nonempty text return identical typed length errors; this observation cannot reconstruct text. Status: source-derived mathematical relation; checked by the adjacent named tests on bounded inputs, with no general semantic proof.

## tests

### Evidence

Four named tests per module, each repeated twice in debug and twice in release. Independent exhaustive byte-window and all-rotation oracles; literal empty/overlap/top-bit/periodic fixtures; arithmetic helper boundary checks. Production and false-oracle mutation controls must compile and fail named assertions. Final evidence receipt records hashes and outcomes. No live parser, corpus, teacher, GPU or timing experiment.

See model.md#evidence for final source bindings, named mutation witnesses, repeated outcomes and raw receipt hashes.

## evidence

### Final evidence

Eight tests total: four per module, each passed twice in debug and twice in release with equal sorted named outcomes. Shift-And exhaustive oracle: 1093 texts times 121 patterns = 132253 comparisons. Rotation oracle: 9841 strings. Additional high-byte, 64-bit, overlap, empty and periodic fixtures; arithmetic helpers tested at usize::MAX. Four compiled controls were rejected by equality assertions and exact source restoration passed all eight tests. Strict all-target algorithm Clippy and narrow rustfmt pass. Four partial syntax projections generated twice identically; full-registry scaffold structural reports repeat identically and have no diagnostics. Scaffold parent manifests were restored and are not committed. No benchmark, semantic proof or live system run is claimed.

#### Raw receipt hashes

Raw files are retained in the external wave14 strings run bundle.

#### Rejected controls

Shift-And removes the start bit and fails word_boundaries_and_overlaps. Rotation changes min to max and fails periodic_and_byte_boundaries. The window oracle inverts equality and fails exhaustive_windows. The rotation oracle reverses ordering and fails exhaustive_all_rotations. Each log contains a compiled test run and named equality assertion failure.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and independent finite oracles added. For text t and pattern p, reject |p|>64 with PatternTooLong{|p|}; otherwise output ordered M={i:0<=i<=|t|-|p| and t[i..i+|p|]=p}. Empty p yields all boundaries, including |t|.

### 2026-10-30 · evidence

Heading `### Final evidence, 2026-10-30` stays in evidence as `### Final evidence`.

#### Source bindings

- algorithms/src/shift_and/mod.rs: `1ba487183cd53ba290e1ebae652d8c52dcf219d81ed682efc84f9b91847228a8`
- algorithms/src/shift_and/tests.rs: `2d5777b4b9f20f28356b389f55f48fc851cb300dcd98a8a3e440a6dd6d8adaaf`
- algorithms/src/minimal_rotation/mod.rs: `8743f6dbf5e7d55ad7037b02c7d8e7bd523e548210d92a668639e4c4758cefe1`
- algorithms/src/minimal_rotation/tests.rs: `7a9bc358e7460627aa9f5271317fb7ec94a0d0e962b8823ea3263b6111511aea`

- shift-debug-1.log: `41a4408b6f82ffdbb90ca8d53ac4908e288dd26cd3a5da0c0189cf18590bcf51`

- shift-debug-2.log: `2f62ad608b040795450d4ca0d0163e6fe78d6dce92bcc73e5fb59cca31d7ea36`

- shift-release-1.log: `2ce89631fd02dac8defc5e5e260a8d9cf36123e72e4a0c1e65b9f03313288091`

- shift-release-2.log: `ea5bde3b42f0183cfb5baacb22b7eae36734b5840cf6d082da32635d249d5803`

- rotation-debug-1.log: `8626f8bd6f30528192e71a6e559842ca5f8102ae963116f1929a880a9274f725`

- rotation-debug-2.log: `8626f8bd6f30528192e71a6e559842ca5f8102ae963116f1929a880a9274f725`

- rotation-release-1.log: `9313165d744d883f99b778e48b93e10ef96b70ff07b69039cbcc708f586cef8d`

- rotation-release-2.log: `660e24ce94b4d0ddc53402a8aacdaa892dc06858d0e5572c87be2c00eee9c5b4`

- shift-production.log: `dbabc06f1b1d85579320320fdfc90913caec54f3a533ac1418bc2ae8de28041a`

- rotation-production.log: `23cc297627081cabecc6a1f8307caa7e30c379da2e1f3526588135b922f92356`

- shift-oracle.log: `613a085a1bd039296993beac4020ff6ce0592e6e9f1c7579e23e05387768d968`

- rotation-oracle.log: `2aa7c5622b3c9ae3bde5f2cfcf61481de35f5524f8c508e1a330edd207157d1d`

- control-summary.log: `0aeb87723d8e1fddc39dff2727d93ca1d75e6840a68c6312f4d0586c8591d154`

- restored-shift.log: `9201ee106687d7a08154c6f119f722fe0419c5e4be649a79d608d6b7351186f0`

- restored-rotation.log: `74127ee594db50ab2b9180ace454d8f0091d7b036e77bdbaf285e9fed95348f5`

- clippy.log: `a5033247aab94e81ca8c243098d7d67ded69e41a74201ed8416df2ba36e7433a`

- full-groupants1.json: `1dac25653dffd2f507bc520ad13c8deda9d1fae11eb263f70f4aed121f2ea75c`

- full-groupants2.json: `1dac25653dffd2f507bc520ad13c8deda9d1fae11eb263f70f4aed121f2ea75c`
