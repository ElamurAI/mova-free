# Confined hashed-reference verification

Subject: current manifest source SHA and exact UTF-8 range [46076,47398). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Require nonempty path; canonicalize a relative reference to a file under root; require expected SHA text; read current bytes and compare SHA-256; return canonical path only on equality. Callers separately constrain Rust sidecar placement and source object equality.

## Invariant

Hash verification always rereads bytes and admits no cached filesystem result. Digest equality establishes declared-byte integrity, not model correctness or unique preimage.

## Cost

One canonicalization and file-type check plus O(B) read/hash for B bytes, O(B) temporary read storage. Repeated evidence references repeat this work.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: groupant_tests::rust_approximants_validate_bindings_budgets_and_evidence. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Over reference/filesystem/result triples exclude successful returned paths outside root, directories, missing expected hashes or changed bytes whose SHA differs. The exclusion assumes the confined target is not concurrently replaced.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::rust_approximants_validate_bindings_budgets_and_evidence. Finite observations do not establish universal completeness.

## reverse_reconstruction

Given returned canonical path p and expected h, compatible references are relative spellings resolving to p with current bytes b satisfying SHA256(b)=h. Hash collisions are not ruled out and original spelling is not recovered.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::rust_approximants_validate_bindings_budgets_and_evidence. Finite observations do not establish universal completeness.

## first_failure_order

Missing path precedes confinement; confinement precedes missing hash; reading precedes digest comparison. A missing file with missing hash therefore yields filesystem failure, not missing-SHA diagnostic.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::rust_approximants_validate_bindings_budgets_and_evidence. Finite observations do not establish universal completeness.

## alias_fiber

Different sidecar-relative spellings or internal symlinks can resolve to one canonical file. The hash gate preserves that canonical identity while generic reference spelling is forgotten in the return value.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::rust_approximants_validate_bindings_budgets_and_evidence. Finite observations do not establish universal completeness.

## evidence_edit_sensitivity

Fix path and expected hash. Any byte edit changing digest must reject even if file size and modification metadata match. Executable evidence declarations do not use the parsed JSON memo.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::rust_approximants_validate_bindings_budgets_and_evidence. Finite observations do not establish universal completeness.

## caller_refinement

Success here is necessary but insufficient for Rust-approximant artifact acceptance: the caller still requires a .rs suffix inside the sidecar and source-object equality. Do not infer those stronger constraints from this helper.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::rust_approximants_validate_bindings_budgets_and_evidence. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

### 2026-11-04 · document

#### Epoch 2026-11-04

`reference_bytes` joins this range. A reference `file#Section` splits at the first `#`; the file is confined as before and the digest covers only the unique plain H2 section (heading line through the last nonblank line before the next plain H2). A missing or duplicate heading is an error before any digest comparison. Converse: a fresh section digest admits every file whose selected section has those bytes, whatever the other sections hold; edits elsewhere in the file leave it fresh. Negative controls: `consolidate::tests::merges_rewrites_rebinds_and_checks` (missing section rejected) and `mova-correspondence` `section_reference_binds_only_its_section` (own-section edit stale, other-section edit fresh).
