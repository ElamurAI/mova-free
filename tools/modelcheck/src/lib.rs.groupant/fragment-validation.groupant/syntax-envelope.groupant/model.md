# Projection selection and exact envelope

Subject: current manifest source SHA and exact UTF-8 range [27698,29521). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Absent syntax_nodes returns success. Otherwise confine and reread/decode the selected artifact through the file-tree reader, fully validate current source and all nodes, then require a nonempty distinct unsigned ID list. Lookup each node by validated index, accumulate min start/max end, and compare exactly with fragment byte_range.

## Invariant

The selection cannot use duplicate/missing IDs or a wider/narrower range. Memo reuse is decoding only; integrity checks precede selected ID consumption on every call.

## Cost

Artifact read/equality O(B), validation O(source bytes + N), selection O(S log S), with one O(B+N) decode on each memo miss. S selected IDs do not replace whole-artifact validation.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: groupant_tests::fragment_syntax_anchors_bind_envelopes_and_current_source. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Exclude success for opted-in anchors whose selected set is empty/nonunique, whose IDs do not exist, or whose range differs from extrema. Invalid unselected nodes also exclude success because validation runs before selection.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::fragment_syntax_anchors_bind_envelopes_and_current_source. Finite observations do not establish universal completeness.

## reverse_reconstruction

Given envelope [a,b), compatible ID sets have minimum start a and maximum end b and are nonempty and distinct. Interior nodes, overlapping nodes and permutations remain ambiguous; root inclusion is allowed if it yields the envelope.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::fragment_syntax_anchors_bind_envelopes_and_current_source. Finite observations do not establish universal completeness.

## integrity_before_selection

The observation order is artifact admission and full integrity, then IDs, then final equality. A stale projection paired with duplicate IDs reports artifact staleness first.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::fragment_syntax_anchors_bind_envelopes_and_current_source. Finite observations do not establish universal completeness.

## permutation_quotient

For a fixed valid projection, permuting a distinct selected list preserves extrema and acceptance. Adding a duplicate changes acceptance despite preserving extrema.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::fragment_syntax_anchors_bind_envelopes_and_current_source. Finite observations do not establish universal completeness.

## unselected_damage_dependency

Hold selected nodes fixed and corrupt another projection node. The full-artifact gate still rejects, so an envelope-valid subset cannot hide artifact damage.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::fragment_syntax_anchors_bind_envelopes_and_current_source. Finite observations do not establish universal completeness.

## gap_preservation

A selection of disjoint nodes takes their enclosing interval, including gaps. This does not assert the model covers the omitted source or reconstruct internal statements.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::fragment_syntax_anchors_bind_envelopes_and_current_source. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

### 2026-11-05 · public export

Reviewed after the regenerable-projection change: projections are read through ArtifactReader::projection, so an absent syntax.json is rebuilt from the current source and validated, never silently accepted; the per-file count syntax_projections_regenerated is reported (inventory-assembly, output totals). parse() became pub(crate) for that reader. Other relations unchanged.
