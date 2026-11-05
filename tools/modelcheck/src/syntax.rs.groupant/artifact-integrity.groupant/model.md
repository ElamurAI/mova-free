# modelcheck-syntax-artifact-integrity

## descriptant

### Source-bound JSON projection identity, UTF-8 ranges and parent integrity groupant

Accepted artifact is structurally bound to current read bytes. Parent<index makes ranges[parent] safe; UTF-8 get validates boundaries. The validator does not reparse or verify category/variant corresponds to source semantics.

Useful prior epochs remain in the parent descriptant. This child uses exact byte anchors and the same whole-source hash: editing unrelated bytes also invalidates its binding. The fragment is a partial structural/behavior model, not a proof. No executable percentage surrogate is supplied: categorical gates have no defined useful loss metric; any future profile must use supported caps 1, 2, 5 or 10 percent with explicit domain and evidence.

## catalog

Wave33 historical scope and evidence follow. Current source hashes and refreshed ranges are in manifest.json; wave34 refinements and evidence are at the source root.

# Source-bound JSON projection identity, UTF-8 ranges and parent integrity typed models

Subject/revision: source SHA-256 in manifest; scope [15791 , 19104). Domain is one completed scoped call on a fixed snapshot with adequate resources, unless narrowed below. Observation includes return/error and declared filesystem effects. Models are source-derived partial relational constraints; complements are inner exclusion hypotheses, converse relations retain compatible-input ambiguity. No universal proof or inverse executor is claimed. Evidence: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Mutation evidence and limits are recorded in ../model.md#wave33-evidence.

## behavior_complement

Exclude acceptance of stale/missing identity, changed source digest, wrong byte count/unit/parser, empty nodes, noncontiguous IDs, invalid UTF-8 slice or parent escape.

Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## reverse_reconstruction

Accepted artifact constrains source to matching path/digest/byte count and interval nesting; arbitrary nonempty category/variant strings remain structurally admissible. Semantic Rust tree is not uniquely reconstructed.

Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## parent_index_safety

For nonroot index i require unsigned convertible parent p<i, then lookup ranges[p]. This establishes vector bounds and containment without recursive traversal.

Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## utf8_range_feasibility

start<=end and text.get(start..end) succeeds. Zero-width nodes are allowed here; fragment byte_range independently requires nonempty. Distinguish the two contracts.

Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## reread_consistency_boundary

Expected SHA and artifact SHA must agree and the freshly read source hash must match. Mutation after this read is outside guarantees; this is not a repository transaction.

Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## integrity_semantics_gap

A projection with valid labels, ranges and metadata can pass without representing the actual parsed AST. Current tests constrain structural integrity only; reparse correspondence remains explicitly unsupported.

Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Fixed source scope and the stated domain are required; finite checks do not establish untested generality.

## tests

### Native correspondence

Exact named checks: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges.

See ../model.md#wave33-evidence for current repeated profiles, actual production mutants and scope limits. AST generation alone is not behavioral evidence. Each reference test body was read before execution.

## forward

### Scoped forward model

Source-bound JSON projection identity, UTF-8 ranges and parent integrity

Check schema/kind/path/digest, reread source and rehash against expected digest, then byte count/unit/parser/limitations. Iterate nodes requiring contiguous index IDs, valid ranges/categories/opacity, full-file root and earlier containing parent.

## invariant-and-cost

### Invariant and cost

Accepted artifact is structurally bound to current read bytes. Parent<index makes ranges[parent] safe; UTF-8 get validates boundaries. The validator does not reparse or verify category/variant corresponds to source semantics.

Read/hash O(B), node loop O(N) with O(N) stored ranges, plus already parsed artifact JSON. Repeated fragment references repeat all work; no shared cache or atomic repository snapshot.

Wave34 correction: validation still repeats on every reference; the caller can reuse decoded JSON after fresh byte equality. Header, freshness, coordinate and parent child views partition these checks. Previous no-cache statements describe wave33.

## wave34-composition


### artifact-header

Require schema 1 and rust_syntax_projection kind; strip repository prefix from source and require UTF-8 relative path; compare exact artifact path; require Some expected SHA and equal artifact digest before any source read.

The child refines the parent call at this exact source range. Shared source identity and error order remain coupled; this view does not add a second execution or independent cost. Header failure prevents source reread and node inspection. Path comparison uses repository-relative identity supplied by the caller, not artifact-provided path traversal.

### source-reread

Read current source as UTF-8 and hash exact bytes; require expected SHA. Then require byte length and utf8_bytes_half_open unit, syn-2 parser and a nonempty array of nonblank limitation strings before touching nodes.

The child refines the parent call at this exact source range. Shared source identity and error order remain coupled; this view does not add a second execution or independent cost. A parsed artifact memo never replaces source reads or hashing. Equal byte length is insufficient; digest consistency precedes metadata checks. No atomic repository snapshot is promised.

### node-coordinates

Convert unsigned start/end to usize; require ordered endpoints and a valid UTF-8 source slice; then require nonempty category/variant strings and Boolean opaque. Root/parent checks are later and separate.

The child refines the parent call at this exact source range. Shared source identity and error order remain coupled; this view does not add a second execution or independent cost. Zero-width syntax nodes are allowed, unlike nonempty fragment ranges. Endpoints inside a multibyte scalar reject. Labels need only be nonblank; category semantics remain unchecked.

### parent-containment

At index zero require explicit null parent, file/File labels and whole-source interval. Otherwise convert parent to usize, require parent<index, index previously accepted ranges and require containment before appending the current interval.

The child refines the parent call at this exact source range. Shared source identity and error order remain coupled; this view does not add a second execution or independent cost. Each lookup ranges[parent] is bounded by strict predecessor admission. A child may share its parent's range; adjacency and non-overlap of siblings are not required.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: materialized a scoped view after full-source review. Check schema/kind/path/digest, reread source and rehash against expected digest, then byte count/unit/parser/limitations. Iterate nodes requiring contiguous index IDs, valid ranges/categories/opacity, full-file root and earlier containing parent.
