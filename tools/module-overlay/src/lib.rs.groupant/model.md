# Bound external source composition

## domain
Finite public Unix source trees U, disjoint external custom trees C, schema-1 manifest M, exact bound snapshot S, nonexistent output O with an existing physical parent. Stable inputs, ordinary allocation/IO success and exclusive output ownership are premises. Excluded namespaces, empty directories, timestamps and non-executable permission bits are outside the copied observation.
## forward
Let P be permitted paths and T the unique manifest targets. Base compatibility means observed(U|P)=S.files, including SHA-256, length and executable bit. For each p in the composed map, output(p)=read(C,M[p].source) if p is in T, otherwise read(U,p). Replace requires p in S and a matching original digest; add requires p absent from S. A receipt records sorted applied entries, canonical-manifest/base/output commitments, model-state partition and fixed Cargo argv. No Cargo/hook/worker is executed.
## invariant
Input paths are read only by composition. Snapshot CLI publication is outside upstream. Output is fresh and disjoint; writes use create_new and copied bytes are checked against their declaration. The generated receipt is excluded from the tree commitment. Known excluded paths never enter the output map. A success is not a semantic or publication approval. Hashes are cryptographic commitments, not mathematically injective inverses.
## cost
With N permitted files, B total copied/read bytes and C custom bytes: O(B+C+N log N) per bounded inventory/planning pass, with a fixed number of passes during stage and extra ancestor checks bounded by path depth. Memory O(N+maximum bounded file+discovered directory entries). Limits: 100000 files, 200000 entries, depth64, 64MiB/file, 2GiB/tree, 1024 overrides/recipes. No speedup/percentage-error surrogate is claimed.
## behavior-complement
Within the stated domain, accepted stage transitions exclude existing output, overlapping roots, mismatched bound snapshots/custom content, invalid replace/add branches, duplicate targets and ancestor-file collisions. Included symlink/special entries and forbidden names are excluded. This is a source-derived inner complement of accepted transitions; arbitrary secret/rights properties and hostile concurrent mutations are not classified.
## reverse-reconstruction
An output tree admits many upstream/custom antecedents. An overwritten original cannot be recovered from its replacement; the receipt retains an original digest and base snapshot commitment, not original bytes. Excluded Git/target/local data and empty directories are unconstrained. Keep the retained upstream/snapshot/custom package for reconstruction; no digest inversion or unique preimage is asserted.
## base-drift-partition
Observation is an ordered list of path rows (expected binding option, actual binding option). Neither/none does not occur: absent expected is addition, absent actual is removal, unequal present bindings are content/length/mode drift. Zero drift allows byte-base composition only. Counterexample to semantic compatibility: an unchanged replacement body can still call an API changed elsewhere after a deliberately reviewed rebase.
## model-freshness-partition
For every composed .rs source, missing adjacent manifest, invalid/missing source.sha256, unequal digest and equal digest are distinct observations. Equal digest is explicitly tagged model semantics unverified. Counterexample: a one-line manifest with a correct SHA has no model, invariant or registration proof; the tool must not promote it.
## side-effect-boundary
Validation/drift failure precedes output creation. Later IO failure can leave an effect prefix; cleanup of the fresh owned output is best effort, and only a final receipt marks success. Stable input rechecks do not establish a global atomic snapshot or defeat a malicious concurrent writer. No upstream writes occur in stage.
## checks
Actual native tests: overwrite_and_addition_leave_inputs_unchanged_and_repeat_exactly; upgrade_drift_is_diagnosed_before_output_creation; changed_custom_bytes_and_forged_base_bindings_reject; explicit_actions_duplicates_and_file_directory_collisions_reject; traversal_reserved_paths_and_unknown_hooks_reject; existing_outputs_and_overlapping_roots_are_preserved; symlinks_and_special_files_reject_without_following; ignored_artifacts_do_not_publish_and_private_tree_refuses; missing_stale_and_hash_matched_models_remain_distinct; cargo_recipes_are_paths_and_fixed_argv_not_shell_hooks; size_depth_and_snapshot_integrity_bounds_are_checked; executable_bit_is_bound_and_preserved.
## epoch-memory

Public-only standalone composer, three Rust owners. The earlier source exporter supplied architectural patterns, not a runtime private dependency. Source hash agreement, native fixtures and structural model checks remain separate from general proof. No live runtime/source/deployment or training was changed.

## masked-drift-preimage
Domain: stable admitted public file trees with fixed permitted path set P, fixed Replace target set T contained in P, and fixed replacement content/executable observations K on T. No Add/action/topology changes, collisions or resource-bound violations are included. Let B record literal file content and executable-bit observations. The ideal byte composition is F_K(B)=(B restricted to P minus T) union K. For an output O with O restricted to T equal to K, the preimage is all admissible B with B restricted to P minus T equal to O restricted to P minus T; otherwise it is empty. Overwritten originals are free only within this stated domain. Excluded names, empty directories, timestamps and non-executable permission bits remain outside the observation.

This reverse-reconstruction obernant refines the lost-original claim: final source content cannot recover overwritten originals. For fixed K and P, F_K(B0)=F_K(B1) iff the observed bases agree outside T. This is an algebraic property of the declared overwrite projection, not a universal Rust proof or an inverse of SHA-256. Native tree maps instead record digest/length/executable commitments; interpreting equal commitments as equal contents requires the ordinary cryptographic assumption. Finite actual-source checks additionally compare known literal output bytes.

Counterexample: an original returning1 and an original returning3 are each replaced by unchanged content returning2. After separately binding each valid base, both native stages produce the same source file map and output commitment. A changed unoverridden README survives the same procedure and changes the output. Evidence: the two named masked/visible drift Rust fixtures in debug/release, with no live upstream upgrade or API-compatibility claim.

## output-equality-admission-exclusion
Domain: the same fixed replacement-content projection, valid customization bindings and the old expected snapshot S. Observation now includes check's drift report, stage's Result/output existence and receipt provenance, rather than source bytes alone. Native exact-base admission requires equality of all permitted FileBinding commitments to S, including targets. Thus a base differing only at an overwritten target can satisfy F_K(B)=F_K(S) while check reports compatible=false and stage returns an error before creating its output. Output equality does not authorize ignoring drift.

To admit the changed base in the fixture, both the base-snapshot byte binding and its Replace original digest are explicitly rebound; the manifest itself is not fixed. The successful receipts have equal output_tree_sha256 but unequal base_tree_sha256, manifest_canonical_sha256 and applied.original_sha256 for the tested content change. Mode-only drift need not change original_sha256, so that narrower claim is not generalized. Binding-invalid input errors are distinct from a well-formed incompatible drift report. Evidence: focused actual-API fixtures plus an external target-insensitive-gate mutation rejected by the new regression. The production gate, recipe behavior and model-freshness limitations remain unchanged.

## History

### 2026-11-01 · epoch-memory

2026-11-01: public-only standalone composer, three Rust owners. The earlier source exporter supplied architectural patterns, not a runtime private dependency. Source hash agreement, native fixtures and structural model checks remain separate from general proof. No live runtime/source/deployment or training was changed.
