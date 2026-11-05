# Node coordinate and label admission

Subject: current manifest source SHA and exact UTF-8 range [17620,18297). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Convert unsigned start/end to usize; require ordered endpoints and a valid UTF-8 source slice; then require nonempty category/variant strings and Boolean opaque. Root/parent checks are later and separate.

## Invariant

Zero-width syntax nodes are allowed, unlike nonempty fragment ranges. Endpoints inside a multibyte scalar reject. Labels need only be nonblank; category semantics remain unchecked.

## Cost

Constant field access and endpoint checks per node plus label trimming; O(sum label bytes) over the projection. UTF-8 get checks bounds without rescanning the whole source slice.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Exclude passage with negative/noninteger/unrepresentable offsets, start>end, offsets beyond text, split UTF-8 boundaries, blank category/variant or nonboolean opacity.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## reverse_reconstruction

Given admitted [a,b), compatible source texts have scalar boundaries at a and b and length at least b. Empty spans and arbitrary slice contents remain possible; admitted labels do not recover Rust syntax.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## empty_span_partition

start=end is admitted when the point is a UTF-8 boundary. This differs from fragment coverage a<b and must remain distinct when composing syntax and fragment checks.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## coordinate_width

On a platform with narrower usize, u64 coordinates must convert successfully before indexing. The relation explicitly includes platform width, not an assumed 64-bit cast.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## label_opacity_projection

The accepted label space is all nonblank strings with an independent Boolean opacity. Unknown categories are therefore structurally admissible and semantic source matching remains omitted.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## endpoint_failure_priority

Start conversion precedes end conversion; interval admission precedes labels. Multiple malformed fields expose this order through their first diagnostic.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.
