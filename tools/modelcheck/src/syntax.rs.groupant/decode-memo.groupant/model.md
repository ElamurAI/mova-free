# Bounded exact-byte JSON decode memo

Subject: current manifest source SHA and exact UTF-8 range [21794,22989). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Read artifact bytes on every call. In memo mode return the immutable saved Value only on full byte equality. Otherwise decode with serde_json; on success retain one entry only at <=8 MiB, clearing on oversized/reference success. Read or parse errors propagate without replacing the slot. New reads in reference mode always decode.

## Invariant

Returned Value equals decoding the just-read bytes; no validation result or filesystem metadata is cached. One source-tree reader is dropped after traversal. Byte cap bounds serialized retention, not serde Value allocator overhead.

## Cost

For m equal B-byte references, one O(B+N) parse plus O(mB) reads/equality and O(1) Rc clones replaces m JSON decodes. Full source/node validation still runs O(m(S+N)); a miss can temporarily retain old and new trees. Retained byte cap is 8 MiB.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: syntax::wave34_tests::artifact_decode_memo_matches_reference_and_observes_file_changes. Parent tests additionally exercise integration and failure order. Current evidence is in ../model.md#wave34-evidence; old evidence remains explicitly historical in ../model.md#wave33-evidence at the source root.

## behavior_complement

Exclude returning saved JSON for unequal bytes, missing files, read errors or malformed fresh JSON. Exclude retaining oversized successful input; a saved valid slot cannot convert a failed read into success.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::artifact_decode_memo_matches_reference_and_observes_file_changes. Finite observations do not establish universal completeness.

## reverse_reconstruction

For returned Value v, compatible just-read bytes are all serde_json encodings decoding to v. A hit further constrains bytes to saved bytes; a miss does not uniquely identify previous slot. Whitespace/object spelling can differ within the decoded fiber.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::artifact_decode_memo_matches_reference_and_observes_file_changes. Finite observations do not establish universal completeness.

## byte_identity_over_metadata

Cache identity is full Vec byte equality, with no path/mtime/length-only shortcut. Same-size edited JSON must decode anew. Equal bytes from different paths are safe only because each caller still confines and rereads its path.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::artifact_decode_memo_matches_reference_and_observes_file_changes. Finite observations do not establish universal completeness.

## error_preserves_slot

A read or decode failure leaves previous memo state unchanged but returns error. A later exact restoration can hit; the error itself is never memoized or suppressed.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::artifact_decode_memo_matches_reference_and_observes_file_changes. Finite observations do not establish universal completeness.

## capacity_transition

For a successful miss with B<=8388608 retain (bytes,Rc<Value>); for B>8388608 clear. Reference mode also clears and cannot hit. This is a retention policy, not a maximum admitted artifact size.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::artifact_decode_memo_matches_reference_and_observes_file_changes. Finite observations do not establish universal completeness.

## validation_independence

Consumers call validate_artifact after every read, including hits. Fresh source hash and all node guards remain active, so a source edit with unchanged artifact JSON still rejects. Across source-tree calls no memo is reused.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::artifact_decode_memo_matches_reference_and_observes_file_changes. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

### 2026-11-05 · public export

Reviewed after the regenerable-projection change: projections are read through ArtifactReader::projection, so an absent syntax.json is rebuilt from the current source and validated, never silently accepted; the per-file count syntax_projections_regenerated is reported (inventory-assembly, output totals). parse() became pub(crate) for that reader. Other relations unchanged.
