# world-school-student-state-retained-codec

## forward

Decode lines in order: ignore # and widths below five; parse four current columns with invalid numbers mapped to zero; widths >=9 also replace accumulator columns. A later five-column duplicate replaces w but leaves an earlier acc row. Encode sorted union keys; emit nonzero w or any explicit acc, and only emit accumulator columns when present.

## invariant

For format-safe keys (no tab/newline and no leading #), numeric table selection survives canonical encoding. Current-only zero rows disappear; orphan acc rows gain a zero current row. This is projection, not arbitrary map identity.

## cost

R rows, K keys, B bytes: decoding O(B+R log K), encoding O(K log K+B), O(K) maps and O(B) output.

## composition

Student load invokes this decoder; save creates a directory then calls write_atomic with this encoder. The parent selects acc globally when nonempty. Presence is therefore a control bit shared with selection, not merely a numeric zero. Filesystem publication and global paths are unexecuted.

## behavior-complement

In the supplied TSV/map universe, exclude decoding a short duplicate as deletion of an existing accumulator; exclude encoding explicit zero acc as absent; exclude invented nonzero values for omitted or malformed numeric cells. Unsafe key strings are outside the roundtrip relation.

## reverse-reconstruction

Given decoded maps, antecedents are all ordered TSV rows whose last >=5-column occurrence sets each w and whose last >=9-column occurrence sets each acc; comments, malformed zero spellings and overridden rows remain free. Given encoded bytes, compatible input maps may additionally contain omitted current-only zero keys.

## accumulator_activation

The observation acc.is_empty after decode/encode equals its input value for format-safe maps, including explicit zero rows; w-only state and zero-acc state can choose different operations.

## duplicate_width

Two rows of the same key commute only if their effective current replacements agree and accumulator writes do not conflict. A five-column row cannot undo a prior nine-column accumulator.

## canonical_projection

Encode(decode(encode(s)))=encode(s) for format-safe representable rows. Decoding may insert zero w for orphan acc; equality is byte-canonical, not every original key-set identity.

## row_order

Encoder key union is a BTreeSet: output is lexical key order regardless of insertion order. Within a row current coordinates precede accumulator coordinates in Add/Sub/Mul/Div order.

## checks

Exact world library selections: school::wave12_models::retained_student_format_is_ordered_and_legacy_tolerant, school::wave12_models::legacy_retention_does_not_change_answer_table, school::wave12_models::zero_accumulator_activation_survives_retention. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.
