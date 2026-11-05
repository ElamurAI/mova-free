# Immediate child admission and diagnostic reservation

Subject: current manifest source SHA and exact UTF-8 range [21538,27611). The observation is one completed scoped call with adequate resources; source-derived relations are partial specifications. Filesystem checks describe sequential reads, not an atomic snapshot.

## Forward

Read declarations; require an array and composition before depth admission. For each declaration reserve its ID, enforce a single normal .groupant path, require real immediate directory and regular manifest, canonicalize containment, then decode manifest. A preorder report slot is reserved even when loading fails. A failed load skips source/range/recursive validation and emits a child-manifest diagnostic.

## Invariant

No admitted child escapes its direct parent; failed loads remain visible. ID insertion precedes filesystem admission, so malformed children can consume IDs and affect later duplicate diagnostics.

## Cost

For K children, O(K log I) ID/path comparisons plus filesystem canonicalization and manifest decoding bytes. Successful children recursively add their own costs; the scope includes traversal and must not be added again to its parent estimate.

## Descriptant

Wave34: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.

## Tests

Exact native test: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Parent tests additionally exercise integration and failure order. Current evidence is in ../../model.md#wave34-evidence; old evidence remains explicitly historical in ../../model.md#wave33-evidence at the source root.

## behavior_complement

Over declaration/filesystem/report triples exclude available children reached through multi-component paths, symlink directories or symlink manifests. Also exclude descent at depth 64 for a nonempty list. This is a source-derived inner exclusion relation, with concurrent path replacement outside the claim.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## reverse_reconstruction

A report slot with parent P and depth d admits declarations in that parent's original order, including failed loads. A load-error observation leaves source contents unconstrained because no source-specific child validation ran; valid slots additionally constrain real direct directory identity.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## id_reservation_trace

Fix previous ID set I and declaration id x. The updated set is I union {x} whenever x is nonempty, even if the path later fails. Two equal IDs therefore cause the later duplicate independently of filesystem validity.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## load_failure_cut

Partition traces at name, metadata, canonical containment, regular manifest and JSON decode gates. The first failing gate determines load error; later fragment range and syntax errors are unreachable on that child.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## preorder_slot_stability

The slot index is reports.len before recursive descent, and descendants append later. Filling the saved index after recursion preserves parent-before-child report order even on nested errors.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## depth_budget_preimage

For an observed depth diagnostic, compatible states have a nonempty declarations array and depth >=64 after composition checks. Empty lists at that depth return without the depth error.

Assurance: source-derived partial relation; fixed source revision and stated scope. Oracle: groupant_tests::groupant_validation_negative_controls_and_open_extensions. Finite observations do not establish universal completeness.

## History

### 2026-10-31 · Descriptant

Wave34, 2026-10-31: deeper scope retained under its existing parent without replacing historical views. Decode reuse was proposed before edits and checked against actual reference decoding; behavioral controls and repeated native receipts remain finite evidence. No numerical surrogate is justified for categorical validation outcomes; no percentage accuracy or complete semantic proof is claimed.
