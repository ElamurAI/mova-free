# Ordered lesson field fold and legacy admission

Subject: `world/src/knowledge.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

Initialize Lesson with file; trim lines and ignore blanks/comments; split the first colon. Scalar assignments overwrite; explain/fact/question vectors append through their helpers. Unknown keys, missing colon, numeric or question failures stop immediately. parse then requires positive n and nonempty facts/exercises/tests; parse_fields alone does not.

## invariant

Successful fold preserves list encounter order and last scalar values; file argument survives unchanged. A successful field fold can still fail legacy parse admission.

## cost

O(B) outer line/field scan plus string copies and invoked citation/question costs. Memory is retained Lesson strings, with overwritten scalar storage released.

## behavior-complement

Type: behavior_complement. In text/file/result transitions, exclude successful parse_fields after unknown fields, missing colon or invalid usize fields; exclude final scalar values from an overwritten earlier assignment. For parse exclude success with missing required vectors. Error-prefix effects are local, not published partial Lessons.

## reverse-reconstruction

Type: reverse_reconstruction. For a returned Lesson L, candidates include ignored comments/whitespace, arbitrary earlier valid scalar assignments overridden later, and ordered list lines whose helper projections equal L's vectors. For errors retain every prefix satisfying earlier steps then the matching failing line; subsequent text is unobserved.

## overwrite

Type: last_writer_relation. For each scalar field the last successfully decoded assignment decides its final value; distinct scalar-key reorderings commute absent errors, repeated-key reorderings need not. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## append-order

Type: ordered_fold. Explanation and fact vectors preserve encounter order independently; a layered fact appends two adjacent entries. Questions retain each field's own order. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## failure-prefix

Type: first_error_relation. An unknown first field masks a later malformed lesson number; numeric parsing may fail before end-of-document admission. No continuation after failure. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## admission-gap

Type: entry_contract. parse_fields admits empty or question-only structures; parse additionally requires n>0 and facts/exercises/tests. Retention parse_material instead requires n>0 and nonempty course. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

Question decoding, citation scanning and fact projection are ordered calls inside this fold; any helper failure prevents final admission. Retention calls parse_fields through its separate parse_material gate. Sibling normalization ranges overlap this calling model intentionally.

## checks

`knowledge::wave34_tests::field_fold_overwrites_scalars_appends_lists_and_fails_first` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.
