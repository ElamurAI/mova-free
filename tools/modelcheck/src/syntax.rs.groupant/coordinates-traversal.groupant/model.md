# modelcheck-syntax-coordinates-traversal

## descriptant

### BOM-aware Unicode scalar coordinate table and selected AST preorder visitor groupant

Root covers original bytes; parent precedes child and contains child interval. Scalar columns are not bytes or grapheme columns. Invalid lookup sets invalid_span and parse later fails; node construction may continue internally.

Useful prior epochs remain in the parent descriptant. This child uses exact byte anchors and the same whole-source hash: editing unrelated bytes also invalidates its binding. The fragment is a partial structural/behavior model, not a proof. No executable percentage surrogate is supplied: categorical gates have no defined useful loss metric; any future profile must use supported caps 1, 2, 5 or 10 percent with explicit domain and evidence.

## catalog

Wave33 historical scope and evidence follow. Current source hashes and refreshed ranges are in manifest.json; wave34 refinements and evidence are at the source root.

# BOM-aware Unicode scalar coordinate table and selected AST preorder visitor typed models

Subject/revision: source SHA-256 in manifest; scope [2943 , 9398). Domain is one completed scoped call on a fixed snapshot with adequate resources, unless narrowed below. Observation includes return/error and declared filesystem effects. Models are source-derived partial relational constraints; complements are inner exclusion hypotheses, converse relations retain compatible-input ambiguity. No universal proof or inverse executor is claimed. Evidence: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape. Mutation evidence and limits are recorded in ../model.md#wave33-evidence.

## behavior_complement

Exclude returned nodes with self/later parent, split-codepoint endpoints or parent escape; exclude interpreting macro body tokens or attribute arguments as projected expression children.

Oracle: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## reverse_reconstruction

For fixed source, line/scalar coordinates map to unique stored byte boundaries; without source, node intervals/triples admit many syntax trees and omitted fields. Digest observation constrains but does not prove unique reconstruction.

Oracle: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## bom_offset_translation

A stripped leading BOM contributes base=3 to parser span offsets while root [0,B) and hash use original bytes. No-BOM base=0; BOM+shebang+CRLF is separately exercised.

Oracle: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## scalar_boundary_table

On LF start a new line at offset+1; otherwise append offset+len_utf8(ch). CR is a scalar on CRLF lines. Stored offsets are Unicode scalar boundaries, not grapheme cluster positions.

Oracle: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## preorder_stack_induction

enter assigns id=nodes.len and parent=stack.last, pushes id; corresponding leave pops after descendants. Parent-before-child follows stack discipline but selected syntax omits intermediate AST fields.

Oracle: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## opacity_projection_fiber

Macro/Verbatim/Unknown nodes mark opaque. Changing well-delimited macro tokens leaves selected structural triples compatible; omitted attributes can change bytes and offsets without changing those triples.

Oracle: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## tests

### Native correspondence

Exact named checks: syntax::tests::nested_projection_is_deterministic_and_source_bound; syntax::tests::unicode_crlf_spans_are_exact_byte_ranges; syntax::tests::bom_and_shebang_keep_original_source_offsets; syntax::tests::macros_are_opaque_and_invalid_rust_fails; syntax::tests::omitted_attributes_macro_bodies_and_external_modules_preserve_shape.

See ../model.md#wave33-evidence for current repeated profiles, actual production mutants and scope limits. AST generation alone is not behavioral evidence. Each reference test body was read before execution.

## forward

### Scoped forward model

BOM-aware Unicode scalar coordinate table and selected AST preorder visitor

Build per-line scalar boundary byte offsets, shifting base by three for leading BOM. enter converts Span endpoints, checks containment, appends sequential node ID and pushes; visitors recursively visit selected syn fields then pop. Macros enter/leave without body traversal; attributes are skipped.

## invariant-and-cost

### Invariant and cost

Root covers original bytes; parent precedes child and contains child interval. Scalar columns are not bytes or grapheme columns. Invalid lookup sets invalid_span and parse later fails; node construction may continue internally.

Boundary table O(B) byte-equivalent storage, selected nodes O(N), stack O(D). enter offsets O(1); visitor O(N plus unprojected visited syn fields). Parsing/recursive stack exhaustion is outside finite test evidence.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: materialized a scoped view after full-source review. Build per-line scalar boundary byte offsets, shifting base by three for leading BOM. enter converts Span endpoints, checks containment, appends sequential node ID and pushes; visitors recursively visit selected syn fields then pop. Macros enter/leave without body traversal; attributes are skipped.
