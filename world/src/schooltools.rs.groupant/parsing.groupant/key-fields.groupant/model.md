# world-schooltools-parsing-key-fields

## forward

Trim line, reject leading #, split at first colon, trim key, accept exactly exercise/test/exam. Split value at every | and trim each field, retaining empty fields and arbitrary width.

## invariant

Recognized malformed option lines remain Some; this parser does not enforce five fields or one answer marker. Returned slices borrow original text.

## cost

O(B+F) scanning/vector storage for B bytes and F fields; no copied field strings.

## composition

Parsing precedes canon in tally/lint/edit. Lint separately emits tabs on all lines before recognizing option lines; parse refusal does not suppress that nonoption diagnostic.

## behavior-complement

Exclude recognition of comments, uppercase Test and fact keys; exclude splitting key/value at a later colon; exclude dropping empty fields between adjacent bars. Recognition is not content validity.

## reverse-reconstruction

For observed (key,fields), compatible lines include any surrounding/field whitespace with the same trimmed slices, plus colons within values. None includes no colon, comments and every unrecognized key. Whitespace and its byte positions are lost.

## first_colon

Only first colon separates the kind; subsequent colons are ordinary value characters until bar splitting.

## empty_field_retention

A recognized value with m bars returns m+1 fields, including leading/trailing/adjacent empty values; even no bar returns one field.

## key_case

Three literal lowercase keys after trimming are admitted; case folding is absent. Comment recognition precedes colon lookup.

## borrowed_projection

Field strings are trim-normalized borrowed slices; parser output cannot recover indentation or spacing around bars without retaining the original input.

## checks

Exact world library selections: schooltools::wave9_models::parser_canonical_fibers_and_tally. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
