# qagen-fix-batching

## forward
try_new checks alignment then delegates stable grouping; prompt numbers each paragraph's items; Job adapter uses the same vectors.

## invariant
For checked input every item appears in exactly one matching paragraph batch and has in-range anchors; original per-paragraph item order survives.

## behavior-complement
try_new never drops orphan items or duplicates their emission via duplicate paragraphs. Legacy new remains unchecked; missing direct Job batch access can panic.

## reverse-reconstruction
Grouping loses input interleaving between paragraphs; multiple item permutations across distinct paragraphs yield the same prompt. Prompt text does not recover reason metadata.

## cost
Validation O((P+I) log(P+I)+anchors); grouping expected O(P+I) key hashing plus BTreeMap factors. Prompt item filtering can be O(P*I), not optimized here.

## composition
Validation closes missing/duplicate/batch/anchor preconditions before inherited grouping. Prompt and gate agree on stable item numbering when using checked batches.

## preflight-order
Duplicate paragraph first; per item missing paragraph, duplicate item, mismatched batch, invalid anchors. Empty paragraph/item collections accepted.

## legacy-domain
new retains previous HashMap behavior; callers needing validation choose try_new.

## callback-count
Job check calls gate once; it does not invoke teacher or lemma callbacks.

## batch-order
BTreeMap orders batch IDs; input paragraph order is retained within each batch.

## checks
Synthetic embedded Rust tests, 1024 independent coverage cases, preflight sentinel artifacts, callback trace and compiled branch/arithmetic controls; debug and release twice, model-loading test explicitly skipped. See parent ../model.md#evidence for observed outcomes, not proof.

## epoch-memory

Prospective model was saved before repair. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.

## references
Original project implementation and rule v2: docs/context.md, qagen/src/gate.rs and qagen/src/run.rs. Model conventions: docs/groupants.md, docs/mathematical-modeling.md and docs/mathematical-modeling.md. No external solution code copied.

## limitations
Finite local tests and partial syntax projections, no semantic AST or universal proof. Allocation failure and callback panic are outside successful-return model; filesystem races and write failures can interrupt publication. No live model/teacher/database/corpus used. No useful certified numeric surrogate is attached to these categorical/effectful interfaces; no invented error budget, substitution or speed claim.

## History

### 2026-10-30 · epoch-memory

Prospective model was saved before repair on 2026-10-30. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.
