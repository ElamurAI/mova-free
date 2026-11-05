# qagen-fix-collection

## forward
Load checked batches; strict retry-first final raw lookup, gate and render accepted fixes/rejections; compute calls JSON, serialize params, then write four files.

## invariant
Preflight failure leaves all four previous artifacts unchanged. Validated grouping and gate uniqueness ensure fixed<=items and unfixed=items-fixed.

## behavior-complement
Unreadable metadata is Err rather than batches_missing. Accounting parse/overflow failure cannot replace any artifact. Successful publication is not a transaction.

## reverse-reconstruction
Counts and by_reason maps merge item histories; identical aggregate params admit different questions and raw replies. QA retains only accepted rows.

## cost
All rendered artifacts coexist in memory; raw and normalization costs are added per batch. Accounting parses complete calls log. No measured throughput claim.

## composition
Checked grouping -> raw lookup/read -> gate -> anchor-ID projection/optional lemma doubt -> accounting -> JSON -> ordered writes. Strict lookup and checked grouping are essential wiring preconditions.

## raw-priority
try_final_raw gives retry priority; retries counter retains existing existence policy after strict lookup. Races between probes are not prevented.

## accounting
calls_json uses wave18 checked totals; preparation errors propagate before materialization.

## anchor-projection
Explicit gate anchors validated against paragraph; implicit anchors validated by try_new, enabling safe sent_id indexing.

## failure-boundary
Write failures after the first write can leave partial replacement; no lock or atomic multi-file commit.

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
