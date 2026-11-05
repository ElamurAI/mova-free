# qagen-fix

## model

# qagen-fix

## forward
Read pilot rows, select span failures, group validated fix items, render teacher prompts, gate replies, and collect four replacement artifacts.

## invariant
Checked construction has unique paragraph and item coordinates with valid nonempty anchors; collect completes fallible preparation before writes. Accepted count never exceeds validated item count.

## behavior-complement
In the universe of supplied items, replies and filesystem states, collect does not silently turn strict metadata errors into missing batches, or overwrite artifacts after accounting failure. Legacy new and direct gate still require aligned caller inputs.

## reverse-reconstruction
Fixed QA determines accepted item coordinates/text but loses raw formatting, discarded replies and original inferred meaning. The same empty fixed output admits absent raw batches and fully rejected batches.

## cost
Selection stores rows/paragraphs/rendered strings. try_new O((P+I) log(P+I)+A) time and O(P+I) validation storage, then existing HashMap batching. Gate stores O(reply bytes) plus normalized sentence allocations; collection keeps full artifacts in memory.

## composition
Selection anchors feed items; checked constructor links items to paragraphs; raw lookup feeds batch gate; accepted local indices map back through the same item list. Accounting/serialization precedes four nontransactional writes.

## validation-order
Collection: load paragraphs, load items, checked grouping, strict final raw/read/gate/render, calls accounting, JSON serialization, writes. Callback panic is not converted to Err.

## item-coordinates
Fix.i indexes 1-based item order, separate from Item.k pilot question number. Paragraph key uses existing doc#para encoding and assumes that project key convention.

## allocation
No global resource cap added. UTF8-safe item-prefix inspection allocates nothing; prompt repeatedly computes keys and gate repeatedly builds Sents, retained optimization gaps.

## publication
qa.tsv, rejects.tsv, changes.md, params.json writes are ordered but lack rollback/fsync; a write error/crash can leave mixed artifacts.

## checks
Synthetic embedded Rust tests, 1024 independent coverage cases, preflight sentinel artifacts, callback trace and compiled branch/arithmetic controls; debug and release twice, model-loading test explicitly skipped. See parent model.md#evidence for observed outcomes, not proof.

## epoch-memory

Prospective model was saved before repair. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.

## references
Original project implementation and rule v2: docs/context.md, qagen/src/gate.rs and qagen/src/run.rs. Model conventions: docs/groupants.md, docs/mathematical-modeling.md and docs/mathematical-modeling.md. No external solution code copied.

## limitations
Finite local tests and partial syntax projections, no semantic AST or universal proof. Allocation failure and callback panic are outside successful-return model; filesystem races and write failures can interrupt publication. No live model/teacher/database/corpus used. No useful certified numeric surrogate is attached to these categorical/effectful interfaces; no invented error budget, substitution or speed claim.

## evidence

### Evidence

Old library witnesses: 49 passed, four failed, one model test explicitly filtered. Old CLI: duplicate-last-wins passed, missing-value witness failed. New final suites: 56 library tests and three CLI tests pass twice in debug and twice in release; one model-loading test explicitly skipped. Nine compiled production mutants reached named failing tests, source restored after each. A tie mutant initially survived the duplicate-endpoint witness; corrected fixture detects it. Strict focused Clippy and paired syntax projections recorded. No live model/teacher/database/corpus operations. Cover removes a content-reference Vec; no end-to-end speed claim.

## prospective

### Prospective qagen repairs

Domain: finite synthetic paragraphs/items, UTF-8 replies, local scratch filesystem and stub Lemmas. No live teacher/model/db/corpus. cover computes hit/content count, None for zero content; replacing collected content references by count/filtered iterator removes one allocation without changing f64 quotient. Pool can be skipped with zero content. doubt validates all one-based anchors first; valid-input callback order and count remain answer then each anchor, including duplicates (trait does not promise purity). Equal best coverage retains first anchor. Fallible try_doubt exposes invalid input, legacy doubt maps error to None.

Fixes try_new validates unique paragraph keys and item (paragraph,k) keys, membership, batch agreement and nonempty/in-range anchors; legacy new remains compatible. collect uses checked preparation, strict raw lookup and calls accounting before any output writes. Four artifact writes remain nontransactional; races/crashes outside scope. Gate uses UTF8-safe prefix lookup instead of byte slicing. Current threshold/tie decisions unchanged. Costs validation O((P+I) log(P+I)+anchors), cover O(A+S) expected hash time with O(S) pool, doubt sum of callback and cover costs.

CLI missing option values (including another option token) and unknown flags error before dispatch effects; duplicate values retain last-wins behavior. Pure parsing tests establish finite witnesses, not all external dispatch contracts. Codex teacher docs align existing transport.

Negative behavior relations: invalid anchors never invoke callback in try_doubt; Unicode junk never panics; accounting/raw preparation errors never replace artifact bytes. Inverse: coverage ratio admits many word lists and anchor permutations; fixed QA loses rejected text; parsed options lose overwritten duplicate values. Old-code witnesses and compiled mutations will demonstrate real gate failures; twice debug/release outcomes must match. No useful numerical surrogate for categorical/effectful operations, no generic error cap or measured speed claim planned.

## History

### 2026-10-30 · epoch-memory

Prospective model was saved before repair on 2026-10-30. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA256: f491a1f6223c6912def81ac3b1fb330fe08196538e1a4d7484cde914246c035f. External receipts: $MOVA_DATA/runs/octopus-wave20-qagen-2026-10-30/.

#### External log hashes
- clippy-all.log: b247b5ac9bbca06d3294cc6d7f3dce2aa6500250c1b6d1aee0f9b79f8841fcfa
- clippy.log: ab771ad1ecf7427783319dcd232981a7487d3e170f549901d56ca1230fe0a8cb
- control-checked_batches_reject_inconsistent_items-branch.log: 9337d8217553ef1133c987eaa9fb88ebff19a8afce478c1e183b1fc4729b86ee
- control-checked_doubt_validates_before_callbacks_and_keeps_order-branch.log: 291dd1dede4483c1a6b9a6582d83f75d94dedb3b3b34ff035b9c9987db8ae560
- control-collect_accounting_error_preserves_four_artifacts-branch.log: baff47fb29b8ee52932abec067382c5a1579da2f31437a665a23a2d3a3b28cc4
- control-collect_raw_metadata_error_is_not_missing-branch.log: b3534662982a2ca9d3c1dd2c8fc7b95122c8b73aa6955558aef2b26c1e29c4d2
- control-cover_matches_independent_small_oracle-arithmetic.log: 598b01635e6820f19289e9f17a68530b1c55524e889614b9f0a46a5156d21abf
- control-cover_matches_independent_small_oracle-branch.log: 9df2b844caa7994e3e487b5d8cc4f88c70dce887a3099066203677ff0bb8f49a
- control-missing_option_value_is_rejected-branch.log: 282b28515809e97cd6ec38263f151a9c1f9ec2dc54a978fb0bd5abf8792c25d3
- control-unicode_item_junk_does_not_panic-branch.log: 779cc2d685614902c6a7a03fa327da8facb95e1824b282eed5a8c6571d2f6eb7
- control-unknown_options_are_rejected_per_command-branch.log: d2beb318db3cc6fe5aa5340a05d4f74a6037756c72947573a1d4a8fbb35ce3fb
- debug-1.log: 9bee9a722f51000ddf97511a3df1e28a979e1da89c3ad9385e6f512e1f303b97
- debug-2.log: 12aae2e60cece008bf139588bc18bdb5c1fd26e54f609c8948f1a110de79c492
- initial.log: ee96ffa9c4ec6664dcf19a49b42deb883cd334124509126173e4180f1d849072
- initial2.log: 38d09704276b5781142caae753933c970e44036e71c0b3df283635ef584145cc
- models.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
- old-cli.log: f08f2a4e2c949bf19d6d1db3cb725413888f6fc725deaf3f1c200a42e92f5045
- old-debug.log: d335e71e125323c00b27a645e10460753d49f4d0d57a8c1d25aaed59ef4bfe88
- prospective.md: ce7ebbe01deccc7904b980c836dd683ed3a37ac98ba424dca37df018930b15ad
- release-1.log: c9ce577c288c32b618ed1dc7cb827f0b46d6096045b5b6e80cb76ea17178359f
- release-2.log: 765302b1bd52b253610dbad9a89a99cfef0511ebf4514788572e84be49c0f1fb
- verification.log: 565b141128ee1e5a0aeaf2a75d0363d8bf216c4749f8fc58195e6d73a8f1513b
