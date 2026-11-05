# coref-score

## model

# coref-score

## forward
Compute MUC, B3, CEAF-e, CEAF-m and mention counts from matched partitions. Matching feeds scorer identities; CEAF overlap components feed floating assignment.

## invariant
Inputs are immutable. Matching never reuses a gold mention and never crosses key buckets. Scratch lifetime changes preserve row-entry values; each successful augmenting path increases matched rows by one. Metric ratios retain zero-denominator behavior.

## behavior-complement
On admitted normally terminating inputs, excluded observations include reused gold identities, cross-bucket pairs, a residual selection preceding a strictly higher-priority available edge, and changed assignment objective under scratch reuse. This is a necessary-condition inner model of absent outputs, not exhaustive behavior enumeration.

## reverse-reconstruction
An output admits many compatible antecedents: unmatched mentions and nonselected edge weights can vary while leaving decisions unchanged; scalar metrics forget concrete mention labels. The preimage consists of inputs inducing the same ordered exact/greedy choices and numeric counts. Uniform label renaming gives compatible partition metrics, but recovering original inputs uniquely is impossible.

## tie-selection
Exact-span matches use first available gold input; residual priorities are (negative overlap, prediction ordinal, gold ordinal). Unique edge keys remove dependence on heap stability. Hungarian strict slack improvements and ascending scans remain unchanged.

## resource-partition
Residual min side below 128, checked product above 262144 or reserve error selects original scan before mutation. Successful static ordering has at most 262144 three-usize edges and two masks. This does not bound original group maps or runtime of fallback.

## state-reset
Persistent minv and used vectors are refilled each row. Equality to freshly allocated vectors at the loop boundary gives equal subsequent operations. Values from previous rows must not leak into current search.

## observation-projection
The full matching vector and exact f64 bits are differential observations; checking only overlap totals would miss identity changes. CEAF-e fractions are not quantized. Counts and output labels are separate observables.

## cost
Legacy residual scan O(PG min(P,G)); admitted sorted path O(PG log(PG)) time/O(PG) storage. Exact-span first pass unchanged. Assignment O(r²c) time/O(r+c) augmentation scratch; original rows>columns also materializes a transpose in O(rows*columns) time and storage; two scratch allocations replace 2r. Dense overlap O(KR+mentions) space/time terms remain. Pilot threshold16 regressions led to admission128; final warmed timing, including regressions, is recorded in models/mova.groupant/model.md#transfer-benchmark.

## composition
Metrics depend on matched identities, not merely overlap objective. Exact matching then residual priority establishes ids before score_doc partitions. Hungarian component alignment retains original arithmetic order. This fragment is part of the parent scorer; fragment correctness alone does not prove end-to-end metric interpretation.

## checks
Nine focused Rust tests twice debug/release; 2694 external differential cases twice debug/release. Independent assignment enumeration and residual choice oracle, explicit tie and resource gates. Compiled mutants and restoration recorded in parent evidence.

## epoch-memory

Initial prospective threshold16 and resource model preceded edits; exploratory regressions at16/32 prompted prospective amendment128, the lowest measured winning size, not an optimal crossover. See model.md#epoch1-evidence and model.md#prospective for retained history. Initial zero-slack mutant survived existing nonnegative assignment oracle; the negative-cost row-reset witness detects it. Preserve signed-domain fixtures when refactoring assignment scratch.

## limitations
Full-file syntax is available, but proofs of every metric and malformed partition behavior remain gaps. Partial syntax projection has no semantic resolution, macro expansion or general correctness proof. No corpus, teacher, GPU, model or database test. No numeric approximant attached: outputs are exact categorical/float identity comparisons rather than an approximate substitute.

## evidence

### Evidence

## prospective

### Prospective transfer

Before production edits: preserve exact spans first and stable residual greedy choices. For residual groups with min(P,G)>=16 and P*G<=262144, materialize uniquely ordered (Reverse(overlap), original prediction ordinal, original gold ordinal) edges and heap-sort them. Checked product, checked allocation and all other sizes use the original scanning algorithm. Predicted crossover is uncertain; benchmark singleton, 8,32,128,256 groups before deciding. Maximum edge payload is 6 MiB on 64-bit targets, plus endpoint masks. Expected large-group cost O(PG log(PG)) instead of O(PG min(P,G)); no task-score change predicted.

Move Hungarian minv/used scratch allocation outside the row loop and fill every row. Expected allocation count falls from 2r to 2, with unchanged floating-point operation and accumulation ordering. Scope: finite regular matrices whose baseline terminates normally; NaNs/ragged matrices have no new promise. Timing gain may be negligible on computation-heavy matrices.

## epoch1-evidence

### Evidence

#### Integration checks

The changed-source check with a temporary registry reports coref/src/score.rs covered and no missing sources. Groupant validation reaches the new leaf without leaf diagnostics; full registry validation still requires root-owned parent children/source bindings (root child-list and missing coref transitive binding diagnostics). This branch does not edit those shared parents. Integration must rerun the groupant check after wiring.

## transfer

### Algorithm transfers

#### Static greedy edge order

Alpha maps residual predicted/gold mentions to all edges (Reverse(inclusive overlap), predicted ordinal, gold ordinal). Beta scans ascending edges and accepts exactly those with both endpoints free. Every edge key is distinct, so heap sort instability cannot change identity order. The highest-priority remaining edge in each legacy round is the first not-yet-excluded sorted edge: induction gives the same full matching vector. Exact spans are removed before alpha, and residual ordinal order agrees with original input order. Zero-weight edges remain admissible. Head/sentence buckets are disjoint, so HashMap bucket traversal cannot affect outputs.

Current admission requires both sides at least 128 and checked product at most 262144. Failure to reserve any of three buffers returns false before output writes and preserves the original scanning fallback. On 64-bit machines the edge payload is at most 6 MiB, plus masks and allocation overhead. This is a payload bound, not an allocator-residency guarantee. The threshold was prospective and remains subject to measured regressions. Cost changes from O(PG min(P,G)) to O(PG log(PG)); exact-span preprocessing remains unchanged and can be quadratic.

Rejected near-match: Hungarian global optimal assignment is not this greedy contract. Even if its total overlap is larger, choosing a different tie or early pair changes mention identities and downstream scoring.

#### Hungarian scratch lifetime

Alpha maps a fresh per-row slack/visited allocation to persistent vectors filled with infinity/false at the same row boundary. Beta observes the final f64 assignment objective. The loop starts with identical values and retains every comparison, arithmetic expression and summation order. On finite regular matrices where the legacy routine terminates normally, the allocation lifetime change leaves all scalar states equal. Row count scratch allocations decrease from 2r to two; fill work remains O(rc), overall O(r²c). Output is a scalar, not the algorithm module's assignment/dual certificate.

Rejected direct API substitution: the library takes i64 minimum costs, widens to i128 and rejects excessive dimensions/work or malformed input. CEAF-e uses fractional f64 maximum similarities; integer quantization or new cap failures violate the existing interface. We transfer buffer lifecycle, not the integer numerical guarantees. NaN, infinity, ragged inputs and nonterminating legacy behavior have no new guarantee. Finite overflow-prone floating inputs remain outside the measured domain.

#### Admission epoch correction

The initial implementation admitted min-side 16. Centralized exploratory timing rejected that crossover: sizes 16 and 32 regressed, while 128 and 256 improved on the synthetic same-head fixture. The adopted threshold 128 is the lowest measured winning point, not an optimality claim. The original prospective record and model.md#epoch1-evidence remain; final warmed alternating-pair measurement is pending.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial prospective threshold16 and resource model preceded edits; exploratory regressions at16/32 prompted prospective amendment128, the lowest measured winning size, not an optimal crossover. See model.md#epoch1-evidence and model.md#prospective for retained history. Initial zero-slack mutant survived existing nonnegative assignment oracle; the negative-cost row-reset witness detects it. Preserve signed-domain fixtures when refactoring assignment scratch.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA256 da0433c1ce72ec0e5bb3b99c522543b6f677e1cd255d2d5767a9818be200393b. Nine focused tests pass twice debug and twice release; 2694 external full-output/bitwise differential cases pass twice debug and release. Two compiled mutants fail their named tests: reversed edge priorities and zero slack reset. The initial zero-slack mutant survived the existing nonnegative brute-force oracle; a signed row-reset witness detects it. Source is restored. Paired syntax projections are identical. Initial centralized exploratory timing is recorded in model.md#prospective; final three-warmup/seven-paired-sample runner is prepared but not executed here. Tests now exercise active sizes128/129/256 and private admission boundary127/128. External receipts live under $MOVA_DATA/runs/octopus-transfer-coref-2026-10-30.

#### Receipt hashes
- baseline.rs: c8a4cc6cada8ab37ef95aa1e1151d13e39e6d01c0587053d6aa09434b0c53ef7
- changed.log: 26ffc56d81e9374b50d5461e14203a7f6853d75aeb8a101de6aa7636eb4d3483
- debug1.log: 5af1bce2287a249dac6e380ebe9b77396cea3eb37fb6db5ac9ee5b581ddee63f
- debug2.log: fdf004e454b97aa4cdb7ae59356b9d1d887176b8a57a6f773e340f80b743b853
- diff-debug1.log: 3f4fa2b10916a7c1e3ae9772fbb8edd0b807febf86e399f480c815f47606b5da
- diff-debug2.log: 0d02f7ab952d677e2c58e69a1f3b1adc46ebd961aaaaef88e88cc901081500f4
- diff-release1.log: b2bff8319bc57c82ecbb3068c5d4320e275182af88cc1ebf93462a592ac6d449
- diff-release2.log: a21fb5dfe5fca1eccc780a20254ce25ccd279b55af7fa195db4164f27d4ed45b
- epoch1-mutant-scratch.log: eadb816371199caedc7330c726f8367a892454a58905ebf21d0acc434872f5c6
- epoch1-mutant-tie.log: a6b4618eecae2db249cc2e9bc65197744c104041adc03064a18a6769742f2dec
- groupants.log: 79f63e9a8bb897a3d4d0b36abe510d0a01d2cf301b8d2170d3210fc88d71be55
- models.log: 93cb87da8fc25961d55b34e2102274ff39ab153f2d2240c7263d46362e89f9d2
- mutant-scratch-initial-survived.log: b5d1e93f4f0f915211a517d156c8146966559527668f0ba826fb2e3ea534e785
- mutant-scratch.log: 84d0c898b8f06f9a61ffe9b0d724801c055a7ba5f8727cac32917bac492b463c
- mutant-tie.log: 032c5aba0df226124bbf0e00692c4927fc0c12b9fcda60a0b9fc3d04ad9dc1e5
- probe.rs: 62758de83de6477de235df87c9291be44ce145c01d565a7bcd5b07b2d788da6f
- release1.log: e53f86e804358d60e57b0f87c826af26f9001947a398b2cb8553d18604c98a25
- release2.log: 34ac2945c60fe2503662986cd313676140d9d5e1bf2dbe413f0b22fa36d1f6a9
- restored.log: 423b4057bd9a42448e6b6e49b3ddd610da5f2333637e9a6e392754a854abf787
- secret-guard-final.log: da72346e960c7a7ba153bdd8a41b9949923b918193f7231eca75c229cd3f3c5e
- secret-guard.log: da72346e960c7a7ba153bdd8a41b9949923b918193f7231eca75c229cd3f3c5e
- threshold128-debug1.log: e44b6266e45c9921e71ebc44e8270fb204997843909ee60077e92dce58819f9d
- threshold128-debug2.log: 5a57e97fecd38c5af1d83c85cf11f2d5305d3d23c44b2bb26449f17f991d81a0
- threshold128-diff-debug1.log: d5496f1d51716d0431ecb488e499737c93986a2e17d299d4a6744ed539d61e10
- threshold128-diff-debug2.log: 15663aa39ddbe5a6bb73f2b9267b9bddc85d724aaae94309c2fe36596db8c564
- threshold128-diff-release1.log: 50f98383339d29763a4226285b4e45d97507c7200ded1187cf92101541502893
- threshold128-diff-release2.log: babc33323b1552ae383e6f9c973709ce31f3d53040b4e6729f0547fcab28e462
- threshold128-models.log: 8f13b82a1b01fffa458452dd89f4c48a7c6ee8d63352e1608b3481dacb5e82cb
- threshold128-release1.log: 35bab7e0d9cfdaf486ab132d6168ac96e3bc4fafcf83fe458cc75b138705914c
- threshold128-release2.log: f28ff235bc8cee9baf40b9881c70382570590639e2f77f905d8e079f1509518d
- threshold128-restored.log: 72bedc456c12efa8b044f151d2fc8d575f05171c4e94c8a0e105703879c35fb3

### 2026-10-30 · prospective

Heading `### Prospective transfer — 2026-10-30` stays in prospective as `### Prospective transfer`.

Differential reference is the unedited source preserved externally from 34811b38001b47df99fcba3e6a98967adccb4b72. Run Rust correctness twice in debug/release, independent assignment oracle and matching choice oracle, and compiled tie/scratch-reset mutants. Timing is centralized separately; no measured gain is currently claimed.

#### Prospective amendment after exploratory timing — 2026-10-30

The initial threshold 16 is rejected for adoption: centralized pilot observes 16 at 1.55–1.59 times baseline cost and 32 at 1.21–1.22 times baseline cost; 128 is the lowest measured winning size (1.16 times speedup), 256 reaches 2.05 times speedup. Raise the threshold to 128, without claiming optimal crossover or general workload speedup. Preserve the initial epoch above. Before final timing, extend active-branch correctness/tie fixtures to 128/129/256, assert branch admission at 127/128, rerun compiled tie control and differential checks. Final runner uses three warmups and seven alternating paired samples; no final timing has yet run.

### 2026-10-30 · epoch1-evidence

Heading `### Evidence — 2026-10-30` stays in epoch1-evidence as `### Evidence`.

Source SHA256 952a76e4c6d4735354e522ec095ce236e79851553210a63720a85e5e929d7268. Nine focused tests pass twice debug and twice release; 2646 external full-output/bitwise differential cases pass twice debug and release. Two compiled mutants fail their named tests: reversed edge priorities and zero slack reset. The initial zero-slack mutant survived the existing nonnegative brute-force oracle; a signed row-reset witness detects it. Source is restored. Paired syntax projections are identical. Timing is prepared but not executed here. External receipts live under $MOVA_DATA/runs/octopus-transfer-coref-2026-10-30.

#### Receipt hashes
- baseline.rs: c8a4cc6cada8ab37ef95aa1e1151d13e39e6d01c0587053d6aa09434b0c53ef7
- debug1.log: 5af1bce2287a249dac6e380ebe9b77396cea3eb37fb6db5ac9ee5b581ddee63f
- debug2.log: fdf004e454b97aa4cdb7ae59356b9d1d887176b8a57a6f773e340f80b743b853
- diff-debug1.log: 3f4fa2b10916a7c1e3ae9772fbb8edd0b807febf86e399f480c815f47606b5da
- diff-debug2.log: 0d02f7ab952d677e2c58e69a1f3b1adc46ebd961aaaaef88e88cc901081500f4
- diff-release1.log: b2bff8319bc57c82ecbb3068c5d4320e275182af88cc1ebf93462a592ac6d449
- diff-release2.log: a21fb5dfe5fca1eccc780a20254ce25ccd279b55af7fa195db4164f27d4ed45b
- groupants.log: 4b77034c2f908bb62d789815974995b215482dc948a47edcdf4be3192c506cc6
- models.log: 93cb87da8fc25961d55b34e2102274ff39ab153f2d2240c7263d46362e89f9d2
- mutant-scratch-initial-survived.log: b5d1e93f4f0f915211a517d156c8146966559527668f0ba826fb2e3ea534e785
- mutant-scratch.log: eadb816371199caedc7330c726f8367a892454a58905ebf21d0acc434872f5c6
- mutant-tie.log: a6b4618eecae2db249cc2e9bc65197744c104041adc03064a18a6769742f2dec
- probe.rs: 16a31d5fbbb96fb657ebeda3e4f10dc2491aa4364be680090d89a414bbf316bb
- release1.log: e53f86e804358d60e57b0f87c826af26f9001947a398b2cb8553d18604c98a25
- release2.log: 34ac2945c60fe2503662986cd313676140d9d5e1bf2dbe413f0b22fa36d1f6a9
- restored.log: 423b4057bd9a42448e6b6e49b3ddd610da5f2333637e9a6e392754a854abf787
- secret-guard.log: da72346e960c7a7ba153bdd8a41b9949923b918193f7231eca75c229cd3f3c5e

### 2026-10-30 · transfer

Heading `### Algorithm transfers — 2026-10-30` stays in transfer as `### Algorithm transfers`.

Source: algorithms/src/heap_sort/mod.rs, SHA256 a8fd695ce2150dee0bc22a33a600c7ecb679c26ccff5375c4199661d5077d525.

Forward model: mod.rs.groupant/approximant.md, SHA256 08644cb93b9075edac88d9829655210916d10992b826fc3c77a96c814ebc9ae8.

Typed models: mod.rs.groupant/models.md, SHA256 87a6aa43628baba7494eb7177f9f48dfffef3728dc28e80f176bff7793a502c6.

Source: algorithms/src/hungarian/mod.rs, SHA256 1b62b3f1e62564c22b7194971369452a2c6bffae56ea6e98dace3430239a0365.

Forward model: mod.rs.groupant/approximant.md, SHA256 b5aabf93c94945b6020afc15315ecf203a4fe3f33f57cf5f40790855032b555c.

Typed models: mod.rs.groupant/models.md, SHA256 70bfeddaa4501758395b3d477cff20a30f168bc45e79637c73f58030006093d3.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
