# Projection schema and source header admission

Subject: current manifest source SHA and exact UTF-8 range [16015,16547). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Require schema 1 and rust_syntax_projection kind; strip repository prefix from source and require UTF-8 relative path; compare exact artifact path; require Some expected SHA and equal artifact digest before any source read.

## Invariant

Header failure prevents source reread and node inspection. Path comparison uses repository-relative identity supplied by the caller, not artifact-provided path traversal.

## Cost

Constant field checks plus O(path and digest length) comparisons; no node traversal or source IO occurs in this prefix.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Exclude successful passage with wrong schema/kind, outside-root source, non-UTF-8 relative path, mismatched declared path or absent/stale expected SHA. This prefix does not assert later node acceptance.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## reverse_reconstruction

A passed header constrains artifact (schema,kind,path,sha) to (1,rust_syntax_projection,relative(source),expected). All nodes and source current bytes remain compatible until later gates.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## schema_priority

When schema and source digest are both invalid, schema error is first. A nonexistent source does not override an earlier malformed header.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## root_relative_identity

Fix root and source. The accepted path is exactly strip_prefix(root), converted to UTF-8. Different but canonically equivalent path spellings are not normalized by this prefix.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## expected_option_boundary

None expected SHA always rejects even if the artifact field is null. Equality of absent values cannot establish binding.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## header_semantics_gap

An artifact can pass this prefix with arbitrary node data. Structural validity and agreement with actual parsed Rust require separate stages and are not inferred.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.
