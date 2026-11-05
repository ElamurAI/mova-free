# qagen-fix-tests

## forward
Synthetic paragraphs/items test numbering, gating, TSV, Unicode labels, strict raw errors, four sentinels, alignment and successful collection.

## invariant
NoLemmas panics on accidental annotation in explicit-only tests; fixtures use scratch directories and finite data.

## behavior-complement
Tests do not exercise live teacher or filesystem crash consistency; a pass does not prove every permissive gate behavior.

## reverse-reconstruction
A sentinel failure localizes mutation-before-error but does not uniquely identify its cause. Same pass outcome admits many untested implementations.

## cost
Bounded local files, constant paragraphs/items, no corpus-scale work.

## composition
Negative controls compile production branches, observe named failing tests, restore exact source and rerun positives.

## sentinel-oracle
Four artifacts start with sentinel bytes; malformed calls requires Err and all bytes unchanged.

## fault-oracle
Unix self-symlink forces metadata ELOOP without permissions assumptions.

## alignment-oracle
Successful explicit collection maps item i to original pilot k and correct sent_id; invalid key/batch/anchors rejected.

## mutation-evidence
Unicode slicing, lossy raw, premature write and skipped batch check mutants reach test failures.

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
