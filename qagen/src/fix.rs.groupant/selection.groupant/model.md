# qagen-fix-selection

## forward
TSV header and eight columns yield QaRow; selection maps anchor sent_id to paragraph ordinal, selects explicit span failures and logs implicit lemma doubts.

## invariant
Batch zero rejected before input reads; selected items sort by paragraph index then pilot k; per paragraphs determine batch assignment.

## behavior-complement
Selection does not infer or repair an unknown anchor ID. Only raw/b000.txt existence guards reselection; other existing raw files and metadata errors are not fully protected.

## reverse-reconstruction
Selected items cannot recover successful explicit rows or most implicit rows. Empty selection admits zero rows or all valid rows.

## cost
TSV O(bytes) storage; anchor mapping scans sentence IDs; item sorting O(I log I); paragraphs and output text are cloned/buffered.

## composition
read_qa -> paragraph key lookup -> anchor conversion -> span/lemma decision -> sorted batches -> writes. Selection writes remain nontransactional.

## row-schema
Invalid header, column count, numeric parse, qtype/exim abort. Positivity/uniqueness of pilot k is not newly imposed.

## ordering
Stable text-order then question k; duplicate input coordinates are later rejected by checked batching.

## lemma-callbacks
Implicit rows invoke supplied Lemmas; no model loaded internally by select.

## selection-status
Summary explicit=span_ok+to_fix; doubtful implicit count is diagnostic, not a rejection.

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
