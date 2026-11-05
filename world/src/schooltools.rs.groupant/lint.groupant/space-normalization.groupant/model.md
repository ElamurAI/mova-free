# world-schooltools-lint-space-normalization

## forward

Split on literal newline including trailing empty segment. Each recognized option line rebuilds original leading indentation, trimmed key, colon-space, trimmed fields joined by space-bar-space, and original terminal CR if present. Nonoption lines are copied. Rejoin with newline.

## invariant

Newline segmentation and final newline presence persist; nonoption bytes persist. Option punctuation/words and field order persist after trim normalization, but whitespace around colon/fields is intentionally changed.

## cost

O(B+F) plus rebuilt output allocation; vectors of lines and fields use O(B+F) total storage.

## composition

lint with fix=true writes normalized bytes before computing findings; this pure helper has no I/O. Normalization does not remove internal tabs or fix semantic/duplicate errors. Parser and rebuild must share original line indentation.

## behavior-complement

Exclude modification of nonoption lines or disappearance of a final newline. Exclude a semantic option reorder or removal of a star by spacing repair. Whole-file CRLF is not forced on mixed-line input.

## reverse-reconstruction

For normalized y retain all original texts with identical newline segmentation, nonoption bytes and same trimmed option fields/key/indent/CR. Space counts around separators and field ends cannot be reconstructed from y.

## line_endings

Terminal CR is preserved per recognized original line; splitting on newline retains the empty final segment, distinguishing newline-terminated and unterminated texts.

## normalization_fixedpoint

On parsed option syntax with stable indentation and terminal CR, applying the same rebuild twice yields identical bytes. This does not imply lint-clean.

## tally_preservation

Canonical option scalar strings are unchanged by outer spacing normalization, so tally agrees before/after on the supported fixture domain.

## report_order

In lint orchestration, a successful fs::write can survive a later file failure. Pure fix_spaces proves nothing about durability or multi-file rollback.

## checks

Exact world library selections: schooltools::tests::fix_spaces_changes_spaces_only, schooltools::wave9_models::parser_canonical_fibers_and_tally. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
