# qagen-lemma-coverage

## forward
Count content answers, build set of all sentence form/lemma strings when needed, count hits against either answer field; validate and maximize per-anchor shares.

## invariant
Each answer occurrence counts separately; sentence duplicates do not increase hits. Invalid anchor is any zero or value greater than sentence length.

## behavior-complement
No implicit stemming or casefolding inside cover, no skipping duplicate callbacks, no last-anchor tie winner.

## reverse-reconstruction
Coverage observes membership only: reorder sentence lemmas or add duplicate entries without changing result. Winner output admits many distinct subthreshold losing anchors.

## cost
Two answer scans and one sentence-pool construction; no content-reference vector. Hash worst-case not claimed linear; allocation can fail.

## composition
Checked anchors prove sents[k-1] safe. Strictly increasing best coverage preserves stable tie choice without sorting.

## zero-case
No content -> None before sentence pool allocation; try_doubt still annotates all anchors on valid input for callback compatibility.

## membership
hit(w) iff w.content and either form or lemma belongs to union of sentence forms and lemmas.

## threshold
DOUBT=0.8 inclusive, compared with f64 quotient inherited from reference.

## error-erasure
try_doubt retains invalid-anchor error context; doubt maps Err to None for compatibility with its Option return type.

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
