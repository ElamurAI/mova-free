# Narrow H2 fence automaton

Subject: current manifest source SHA and exact UTF-8 range [254,3689) of sections.rs. The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Scan lines with state Outside or (opening marker,run length). Outside, admit <=3-space backtick/tilde runs >=3 with backtick-info restriction; otherwise count exact column-zero H2 string. Inside, close only same marker at least opening length with whitespace-only suffix and skip every line from counting.

## Invariant

Only exact unfenced H2 lines count. Closing fence lines never count. The scan counts all occurrences so duplicate headings remain detectable; it is not a Markdown renderer.

## Cost

O(B) scanned bytes and O(H) heading-string allocation for B artifact bytes and H section bytes; constant fence state. Each component's section read rescans independently.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Parent tests additionally exercise integration and failure order. Current evidence is in ../../lib.rs.groupant/model.md#wave34-evidence; old evidence remains explicitly historical in ../../lib.rs.groupant/model.md#wave33-evidence at the source root.

## behavior_complement

Exclude positive heading increments for lines inside a recognized fence, padded headings, wrong heading level or heading text with trailing markup. These excluded transitions are defined by current fence state and exact line equality.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## reverse_reconstruction

Observed count k admits all texts whose outside-state exact-heading events number k. Fence contents and unrelated lines remain unconstrained; k=1 cannot recover a unique document.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## closing_threshold

Given state (m,L), a line closes iff indent<=3, marker=m, run>=L and remaining suffix is space/tab only. Short runs or other marker preserve the state.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## opening_info_constraint

Outside-state backticks with another backtick in suffix do not open; tilde suffixes have no equivalent restriction. The distinction is intentional narrow grammar.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## duplicate_count_semantics

A document with two admitted exact headings yields 2; the caller requires count=1 and rejects both missing and ambiguous sections. Early success at first match would weaken validation.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## unterminated_fence_suffix

After an opening without an admissible closer, all later headings are suppressed. Prefixing an opener can therefore change the compatible-document fiber for an unchanged suffix.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

### 2026-11-04 · document

#### Epoch 2026-11-04

Moved from `lib.rs` (component validation) to `sections.rs`, shared with consolidation. The automaton is now `lines` (per-line fenced flag, delimiter lines included), `unclosed_fence` and `heading_count`; the counted relation is unchanged: exact column-zero `## Name` lines outside fences. New in this range: `h2_names` lists every unfenced line starting with `## `. Additional oracle: `sections::tests::sections_follow_the_plain_h2_grammar` (duplicates and unclosed fences as negative controls).
