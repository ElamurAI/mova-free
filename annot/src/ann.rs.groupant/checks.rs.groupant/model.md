# annot-ann-bfs-checks

## descriptant

### annot-ann-bfs-checks

Bounded exact annotation gate differential checks and an ignored manual benchmark.

#### Mathematical combiner

Combine source-bound partial syntax, the forward relation, typed obernants, invariants and finite checks. Restrict the forward relation by stage guards; compose the guarded graph adapter with BFS and project the first unreachable token. Use observed disagreement to revise the adapter or model. The source is authoritative; missing macro expansion, name resolution and full legacy semantic coverage remain explicit.

#### Hybrid correction after exploratory timing

The initial unconditional-BFS implementation preserved semantics but root's baseline-first exploratory pilot observed converter star regressions of 1.95–4.21x and n=8/32 chain regressions of 3.33x/2.03x. Those observations motivated an amendment, not a final performance claim. The original epoch remains recorded. The amended hybrid retains old small and cheap walks, then applies BFS only after a linear work budget. Benchmark methodology now uses three warmup pairs and seven alternating-order samples; no amended timings are claimed here.

## approximant

### Forward approximant

The old check body is retained as baseline; 126126 head vectors for n=0..6 compare exact Result values. Named precedence and long-shape tests exercise negative controls. An ignored benchmark compares full production and baseline gates on preconstructed chain/star fixtures; it is not run by ordinary test commands.

For the transferred gate, after earlier predicates pass, let H(i) be the head and H(0)=0. Success iff every i in 1..=n reaches 0 under iteration of H. Otherwise return the least i whose orbit never reaches 0. Reversed adjacency satisfies G[h]={i:H(i)=h}. Reachability from 0 in G is equivalent to termination under H. Correct leaves alone do not establish the adapter; exhaustive differential checks test its wiring.

Scope remains partial for unmodified parsing/formatting/action details; no semantic AST or full application proof is claimed.


## invariant

### Harness invariant

Both implementations receive the same immutable fixture and exact Result values are compared. Annotation registry is initialized once in exhaustive comparisons. Base-(n+1) enumeration visits each head vector once including empty input; named malformed cases lie outside that enumeration. A copied old implementation can share legacy bugs, so these checks establish regression agreement rather than overall linguistic correctness. The ignored benchmark must run serially and checks output agreement before timing.


## catalog

# Typed obernants

## behavior-complement
Let U consist of a candidate pair of gate implementations, the fixed fixture schedule, and tagged harness observations (normal assertion completion or assertion failure). The forward harness relation R reports completion exactly when every scheduled comparison and named expected-value assertion holds; C=U\R excludes reporting completion with a violated assertion or failure when all assertions hold, assuming successful fixture construction and allocation. This finite operational model does not exclude bugs outside the schedule.

## reverse-reconstruction
Pre(completion) contains every candidate gate agreeing with the retained baseline on all enumerated inputs and the named expected values. It includes arbitrarily wrong behavior on untested inputs. Failure preimages retain every candidate whose first mismatching assertion produces the observed diagnostic. No candidate is uniquely reconstructed.

## reachability-preimage
The fixture generator maps integer code in 0..(n+1)^n to a unique head vector by base-(n+1) digits. Its converse recovers that integer from the ordered digits. Empty n contributes one vector. This fixture relation is exact for n<=6; it does not recover lexical annotations from a gate result.

## error-precedence
The explicit tail-first fixture distinguishes ascending first failure from the reverse-scan mutant. The metamorphic intervention changes production scan order while retaining fixtures and the old baseline. This models the effect of an implementation intervention, not an inverse proof.

## representation-equivalence
Reordering independent fixture comparisons preserves whether all comparisons hold, but may change the first failure diagnostic. Fixtures and original inputs are borrowed by both gates; benchmark black_box does not certify optimization resistance beyond the executed compiler/runtime.

## resource-refinement
Exhaustive fixture count is sum(n=0..6)(n+1)^n=126126. Baseline head chasing dominates long chains quadratically. The ignored benchmark constructs fixtures once and measures repeated full gate calls, excluding setup and registry construction. Passing functional checks supplies no timing result.


## tests

### Checks

Manual benchmark only: cargo test -p annot --release bfs_transfer_benchmark -- --ignored --nocapture --test-threads=1, then equivalent -p en. It compares full gates on fixed synthetic chains/stars, with setup outside timing and black_box inputs/results. No model or corpus access. Timing remains deferred; no performance result is asserted.


#### Hybrid checks

A fourth active-fallback test per consumer covers the 64/65 threshold and a long rooted prefix exhausting the budget before a later cycle. Reverse-projection mutants run this test rather than the small precedence fixture, which now correctly takes the old-walk branch. Eight tests pass twice debug/release; original focused tests pass twice. The ignored benchmark uses 3 warmup pairs and 7 alternating-order samples at n=8,32,64,65,128,512,2048; setup is excluded. No timings run by this worker.

## evidence

### Evidence

Six new tests pass twice in debug and release: each consumer exhausts 126126 head maps, checks exact precedence, and covers nine long-shape fixtures. Two manual benchmarks remain ignored and unmeasured. Existing converter tests: 14 pass twice; existing annotation tests: 1 passes twice. Normalized named test outcomes match across repeated runs (elapsed time and compiler output excluded). Each reverse-scan production mutant compiles, fails the tail-first assertion with exit 101 (4 versus 1), and restored source passes. Groupant validation reported valid=true and no diagnostics, with legacy missing coverage retained. No teacher/model/database/corpus/GPU actions.

Source and baseline hashes:

Runtime logs and original source snapshots remain outside the repository. Hashes identify the evidence, not an analytical proof. Larger safe follow-up candidates are all en library unit tests and annot ann/repair unit tests; model-dependent or live integration suites were not run.

## History

### 2026-10-30 · descriptant

#### Epoch 2026-10-30

Prospective prediction: preserve exact errors and reduce worst-case cycle-phase work, paying linear memory. Original code is retained externally and in the test-only baseline. Exhaustive head maps, named precedence fixtures, actual compiled semantic mutants, and paired syntax projections are used; see model.md#tests for completed evidence. Timings remain deferred to a serial run. No earlier file-model epoch existed; legacy behavior is summarized rather than claimed fully verified.

### 2026-10-30 · approximant

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · invariant

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · document

#### Hybrid amendment 2026-10-30

For n<=64 the original ordered cycle walks run unchanged. For n>64, ordered walks share a saturating 4*n step budget. A detected cycle returns the original diagnostic immediately. Completion returns Ok without BFS. Exhaustion discards partial reachability work and invokes the exact reverse-head BFS adapter over the whole input, preserving the least failing start even when exhaustion happened on an earlier rooted prefix. Prior input/head/root predicates are unchanged. Budget exhaustion is not an error and never implies a cycle.

Cost: tiny input work is bounded by 65*n; larger prefixes perform at most 4*n head steps before O(n) BFS. Thus worst-case cycle-phase time is O(n). Auxiliary space is constant for the walk-only branch and O(n) on fallback; converter's preexisting heads vector remains O(n). Saturation cannot increase work beyond the mathematical 4*n bound. Stars complete their walks without adjacency allocation. No universal throughput gain follows from this bound.

### 2026-10-30 · tests

2026-10-30: three transfer tests per consumer: exhaustive 126126 head maps (n=0..6), precedence fixtures and long chain/star/cycle-tail shapes (n=8,64,1024). Compare exact production Result to the retained old implementation. Debug and release execute twice; existing ann and convert unit tests execute twice. Source mutant reverses the final scan, causing the tail-first fixture to report word/token 4 instead of 1; reject the compiled mutant and restore source before passing the named test again. Evidence hashes are recorded after execution. Paired syntax JSON outputs must match byte-for-byte. These finite checks are not a proof for arbitrary inputs.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

- annot/src/ann.rs: 045f8efecd2114094439ec08054e3f62be6d8aacfe22fd18b2c4ceb1c25fa210

- en/src/convert.rs: b848de0ce16bf6f621bda9ac9a294730a3412f3ae227b30d47b3e961ca646634

- annot/src/ann.rs.groupant/checks.rs: 01a31faa266ba2f53f2bb4b6391d70c5ac5ecd65b1e9b2b0dc042cdeda8fddec

- en/src/convert.rs.groupant/checks.rs: 2e3bbf17bae35f3922c395785a2f2dfc5e0caa0d35fec196df8ddf1a5e01eb45

- External ann.baseline.rs: e6d149639113b39b087805b32ca7d5b433483dc3a36e6b0a473930a37b46d727

- External convert.baseline.rs: b3e808371759f739f1bf319f63e410d337505b89463ce5d86886abab241a36f5

- External prospective.md: 059ec85fe6b9bef93351c46a6aab285be7b5fb52c878c96ad7aef7e74470c1f0

- External debug-final1.log: d9950f0bff6ca94b7a5449f0ee9cf139d5da89fb8a7e7d79eb79ccc211d6a5d9

- External debug-final2.log: edebb38010a510d66198ea417c9bbd246a574b53ea9b08a3f84a4837f27622c5

- External release-final1.log: 42bea56b01434ab26b9f2772e0346ee9ceefdf418f88066b7eddccb644691619

- External release-final2.log: ba2520448351c36dfde8616c1240f41f5a9a2d38c904857448b4db1d805acffd

- External en-existing1.log: c9924f7db9b5d09fabbdb0dab186d7e6f7be70352dcd5b0e2edec9ffcc5c87b3

- External en-existing2.log: 80f979bc8ad0edaa93eb653adca772f611f5aeb2adef9e8bd492a2f2da9e916d

- External annot-existing1.log: d68d763012771a7e1ac6c55556889cc81550d262733986a7deb1219e2541eec4

- External annot-existing2.log: 962069dc2681c5ef2c70665f9323578a78d9121b5f6b09ff4a619321c0e13f04

- External annot-mutant.log: 535fdb38b65429426d79fb21fcbb8473c9316492557e977a0dbd763873653a49

- External annot-restored.log: 2a8d1dc4533601ad43e1290437ae5d66e23f4543ec544ff2539edd6fe97ea975

- External en-mutant.log: 45cb400480d3296d5d5274c72a669781ef09cf5a1caed93d06ac735308773763

- External en-restored.log: e4318a6db10457d646bad262fc0abc377a2179202856b3ab08dcacbb8fe5642e

#### Hybrid evidence 2026-10-30

Eight transfer tests pass twice in debug/release (four per consumer, plus two ignored manual benchmarks). The old exhaustive 126126 vectors per consumer exercise the small-input branch. New cases exercise 64/65 boundaries, rooted chains and stars through n=129, and a 129-word rooted prefix followed by a tail entering a cycle: the budget expires before the first failing token 100. Both compiled reverse-projection mutants are rejected on this active BFS branch (exit 101); restored tests pass. Existing 14 converter and one annotation tests pass twice. Normalized test names/outcomes match in both repetitions. No timing was executed for this amendment.

Current source/evidence hashes:

- annot/src/ann.rs: 459a17bfd13ed4a6ba41719dac5dd97bac86bda51ad0b1ab2e176855c57aee2b
- en/src/convert.rs: b1076c61d8bc50673f47d8040b59c306edc3a934eba4582ab9a67ef9cddd86a6
- annot/src/ann.rs.groupant/checks.rs: 9cd22a2863d0e448e24362a6fc0f64ca97f499663bac1efd4a22544c5f16113a
- en/src/convert.rs.groupant/checks.rs: 773cfb5d98bdab8b3fd45646b404e8c75e1d46ddc9e19ba582f69812a67f0ae2
- hybrid-prospective.md: 2d1324336d1e6fa0f47e8200a8a42dac6c74975031393984bff3577e3f108516
- External hybrid/debug-final1.log: 008ddfbba5a67c8610aee8a15e406a61e8ccf0e662f7f52d39f3ab480dd09c55
- External hybrid/debug-final2.log: d1ec976696aa9a11c33924f6db6d8e3044142c20832a03dbdf25698819e7c308
- External hybrid/release-final1.log: a1ff670b57401e7d02e8b894540878fbd93c76e1b6152cf5a1a17ab1cf18febc
- External hybrid/release-final2.log: c97a901f41b0597ba441b7edc43f07c4fa356d5b1664fe4715b9bb0c2cc0fe89
- External hybrid/annot-mutant.log: 4eeaa4046853ee2b65ae7b9b8cf4b6d702b6124b52990647d517b9f54000339f
- External hybrid/annot-restored.log: 7bb2651295e729140a3b7243df85d3e1eb9cd16a2563f6ec8844b7024ef9fbd8
- External hybrid/en-mutant.log: 6ae354103c923ad4b855d66134a2ab000b585e5d24796aa677e7e7e30ecb6509
- External hybrid/en-restored.log: 8c8293a045400432a8857530acc89616b8119446aeb867b0092693a3c15cd663
