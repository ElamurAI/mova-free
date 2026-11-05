# world-schooltools-editing-edit-sequence

## forward

Start with split newline buffer. For each edit validate line bounds, option recognition, nonempty separator-free new text, exactly one old option match after stripping stars, actual change, five fields, right-answer protection and casefold uniqueness. Successful edits rebuild that line preserving CR/star position; errors accumulate and later edits still run. Return buffer only if no errors.

## invariant

No partial text escapes on Err. Later edits observe earlier successful tentative replacements even after an earlier refusal. Untouched lines and final newline persist. No filesystem operation occurs here.

## cost

O(B) split/join plus per-edit line clone/parse, O(F log F) uniqueness comparisons and string folding. Working-buffer and error storage O(B+diagnostic bytes).

## composition

edit reads rows in table order grouped by file, calls apply_edits per file and writes only Ok unless dry. This is per-file refusal, not all-files transaction; successful prior files can remain written if another fails.

## behavior-complement

Exclude Ok with any collected error, forbidden new separator, ambiguous old option or duplicate casefolded options. Exclude changing question through an option index. Refusal does not imply the temporary buffer was never changed.

## reverse-reconstruction

For fixed edit list and successful y, retain originals whose sequential replay produces y with no errors. Old strings constrain replaced fields but original spacing is lost. Err diagnostics admit many texts and tentative successful histories, not a unique pre-state.

## ordered_replacements

Sequence b->z then z->y succeeds; reverse order need not. Lookup is against the current buffer, not the initial text.

## diagnostic_accumulation

First failure is recorded and iteration continues. Final Err suppresses every tentative change from the returned result even though subsequent validation sees earlier successful changes.

## answer_marker

A matched right option keeps its original '*' and position; canonical before/after answer guards reject indirect changes but permit explicitly replacing the old right text.

## casefold_uniqueness

Option uniqueness uses Unicode lowercase strings, not trim-only identity; duplicates are rejected after constructing the candidate line, before installing it. No Unicode normalization claim.

## checks

Exact world library selections: schooltools::wave9_models::edit_transaction_and_argument_effects. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
