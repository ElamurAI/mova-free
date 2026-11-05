# Fresh source and projection metadata consistency

Subject: current manifest source SHA and exact UTF-8 range [16547,17204). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Read current source as UTF-8 and hash exact bytes; require expected SHA. Then require byte length and utf8_bytes_half_open unit, syn-2 parser and a nonempty array of nonblank limitation strings before touching nodes.

## Invariant

A parsed artifact memo never replaces source reads or hashing. Equal byte length is insufficient; digest consistency precedes metadata checks. No atomic repository snapshot is promised.

## Cost

O(S) source bytes read/hash with O(S) text storage, plus limitation string scan. Work repeats for each artifact validation even after a memo hit.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Exclude successful passage for changed source digest, invalid UTF-8 read, mismatched byte count/unit/parser or empty/nontext limitations. Cached JSON cannot exempt any of these exclusions.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## reverse_reconstruction

Given success and expected h, compatible source texts have SHA256(text)=h and length equal to metadata. Compatible limitation arrays contain at least one nonblank string, but their truth is not reconstructed.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## same_length_edit

Hold byte count constant and replace source tokens. A changed digest rejects before parser metadata or nodes, demonstrating byte count does not certify freshness.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## read_error_order

Source IO/UTF-8 failure occurs before fresh hash comparison; hash mismatch occurs before length/unit/parser/limitations. The same malformed metadata can therefore be masked by an earlier source failure.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## limitation_nonemptiness

Acceptance constrains each array element to nonempty trimmed text and requires the array itself nonempty. The validator neither recognizes a fixed limitation vocabulary nor checks its semantic truth.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## postread_race_limit

Success describes the bytes just read. A mutation after the read is outside the invariant; repeated rehashing narrows stale reuse but does not create filesystem transaction semantics.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::wave34_tests::cached_decode_still_validates_source_and_nodes_and_measures_cost. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.
