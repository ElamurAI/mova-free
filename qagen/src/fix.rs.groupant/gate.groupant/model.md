# qagen-fix-gate

## forward
Classify numbered reply lines, reject batch format when thresholds fail, then validate first response per item as explicit span or implicit fallback.

## invariant
Valid item numbers are 1..M; at most one accepted result per item. UTF8 text cannot panic at the item prefix branch; equal duplicate lines after first are rejected.

## behavior-complement
Malformed UTF8-boundary labels count as junk, not a slicing panic. Direct misaligned paras/items remain outside gate's validated caller domain.

## reverse-reconstruction
Accepted spans admit many whitespace/case label variants and rejected trailing duplicates. An implicit result preserves original answer/anchors, losing the raw label formatting.

## cost
O(reply bytes) parsing/storage plus per-item Sents normalization and span checks. Finite usize counters assume process-representable input allocation; no new input-byte cap.

## composition
Batch-wide threshold checks precede per-item checks. Implicit acceptance uses original anchors; checked caller must establish their validity.

## thresholds
No numbered lines first, outside IDs second, junk*5>total+junk third, missing*4>M fourth; strict inequalities retain boundary behavior.

## unicode
get(..4) succeeds only at character boundary; slicing tail follows ASCII prefix proof.

## duplicate-policy
First numbered line is checked; subsequent lines cannot rescue a rejected first line.

## implicit-policy
An old verbatim answer rejects implicit fallback; otherwise answer/anchors are copied unchanged.

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
