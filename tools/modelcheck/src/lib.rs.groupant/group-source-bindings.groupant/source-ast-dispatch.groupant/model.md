# Source identity and opt-in AST dispatch

Subject: current manifest source SHA and exact UTF-8 range [35679,37221). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Accumulate schema/kind/id/status diagnostics, canonical source identity mismatch and expected SHA mismatch. Validate all components even after identity errors. Only rust_syntax_projection format triggers confined artifact read/decode and full syntax validation; append its error after earlier diagnostics. A caller-owned reader shares identical decoded bytes across this source tree.

## Invariant

Changing only manifest SHA cannot refresh stale artifact source metadata. Reference mode and memo mode execute identical validation after decoding; errors accumulate rather than fail fast across manifest/component/AST phases.

## Cost

Confinement and component costs plus source rehash/node validation for every AST opt-in. A one-entry decoder memo reduces repeated JSON allocation but leaves reads, hashes, component validation and diagnostic order intact.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Exclude diagnostic-free manifests naming another canonical source or stale expected digest. Exclude promotion of an invalid declared projection to success merely because decoding was cached.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## reverse_reconstruction

A clean observation constrains schema/kind/status, canonical adjacent source, matching expected SHA, all component checks and opted-in artifact integrity. It does not recover descriptive truth or a unique source from SHA.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## accumulation_order

Independent identity faults do not suppress component diagnostics; AST validation runs afterward. The ordered output is a concatenation of these stage diagnostics, not just the first failure.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## format_optin_partition

Hold manifest fields fixed except ast.format. Only the exact rust_syntax_projection label enters the syntax gate; generic references still undergo component reference checks in other cases.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## reader_lifetime_boundary

The groupants source loop creates one reader and moves it into FragmentValidation after the root. Other sources and subsequent invocations start empty; no validation result survives a call.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## rebound_manifest_residual

After editing source and refreshing only manifest SHA, the artifact digest comparison remains stale. Compatible repair requires regenerating or rebinding actual artifact content with valid ranges; a manifest-only change is insufficient.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: syntax::artifact_tests::declared_projection_rejects_stale_wrong_source_and_bad_ranges. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

### 2026-11-05 · public export

Reviewed after the regenerable-projection change: projections are read through ArtifactReader::projection, so an absent syntax.json is rebuilt from the current source and validated, never silently accepted; the per-file count syntax_projections_regenerated is reported (inventory-assembly, output totals). parse() became pub(crate) for that reader. Other relations unchanged.
