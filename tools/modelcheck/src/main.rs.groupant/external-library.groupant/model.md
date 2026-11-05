# External reference-library reader

Subject: helper/test fragment of adjacent main.rs at the manifest SHA-256 and byte range. Fixed immutable readable filesystem snapshots, ordinary completed calls and adequate resources are assumed. Canonicalization/read races are not prevented; this is artifact integrity for trusted storage, not a concurrent filesystem sandbox.

Forward: require explicit STORE or MOVA_GROUPANTS for '-'; repository and store must be disjoint after canonicalization and store/.git absent. Read one schema1 algorithm/paper manifest, no collection scan. Resolve confined relative paths, at most256artifacts and256paper source bindings; every regular-file read is capped at16MiB. Check hashes without refreshing or executing commands. Algorithm snapshots equal their bound source; paper components have artifact paths or explicit missing status. Metadata/use-model integrity does not establish full-paper semantics.

## behavior_complement

Domain: completed reader calls on the fixed snapshot; observation is Result plus enclosing CLI report effect. Exclude valid receipts for stale source/artifact bindings, undeclared component paths, escaping/overlapping namespaces, budget excess, absent required configuration or fabricated paper components. These are source-derived partial exclusions with focused negative checks, not a filesystem security proof or licence validator. Invalid reader input fails before report creation.

## reverse_reconstruction

A valid report constrains manifest hash, ID/kind and counts, while admitting all compatible fixed filesystems. It cannot uniquely reconstruct input spelling, root paths, mathematical content, text access or earlier state. Counts do not reconstruct a groupant. No inverse executor or universal proof is supplied.

## namespace_admission

Paths cannot have empty/dot/parent segments or escape their canonical root via symlinks. Repo/store overlap and a Git marker reject before reading a bundle. Reports cannot be written into the store. Each file's16MiB cap is separate from artifact/source-binding count caps. No atomic multi-file snapshot or TOCTOU guarantee is claimed.

## missing_model_partition

Paper components distinguish declared artifacts from status missing. Bibliographic metadata and an implementation-use model do not supply absent paper ASTs or full mathematical models. The reader checks this basic partition, paths and hashes; it does not interpret formulas, licences or all nested status strings. Algorithm editions retain their own partial evidence labels.

Cost: O(J+B+S) read/hash work plus ordered-map overhead for manifest bytes J, artifact bytes B and repository binding bytes S. Working storage is O(J+largest_file) apart from decoded metadata. Up to256*16MiB of artifacts can be read in a selected call. No throughput or numeric-error claim follows. Exclusive report writing preserves existing outputs but does not guarantee rollback after a later write/sync failure.

Evidence: five focused inline bin tests check admitted algorithm/paper fixtures, hash/source mismatch, fabricated components, proof flag, traversal/symlink/Git/overlap rejection, binding-count cap and missing configuration. Real relocated algorithm/paper manifests and false controls have separate external receipts. Source projection is partial syntax, not a semantic proof.

## Source-survey specialization (2026-11-01)

The `source_survey_groupant` branch adds a distinct generated/unverified observation. Let B be the number of verified repository bindings: B=1 for an algorithm with matching `repository_path` bytes and SHA; B=0 for a paper. Paper external digests are declared provenance only. Both require a valid admitted source-kind/format pair, intact artifacts/components, a 4000..5000 Unicode-scalar description whose stored count agrees, and explicitly missing AST/proof. These are shape/integrity conditions; a repeated filler paragraph is a counterexample to any inference of useful content from acceptance.

The specialized behavior complement excludes accepted receipts for stale algorithm source hashes, unsupported formats, invented proof/AST paths, description-count mismatch and artifact corruption. Reverse reconstruction from an admitted paper receipt has a non-singleton fibre: the reader did not access the original source, so no unique source or scientific truth is recovered. Algorithm reconstruction binds current repository bytes only under fixed-read and SHA assumptions, with no scientific claim checked. Seven CLI fixtures pass debug/release; the added deterministic repeat and negative cases are finite observations, with no percentage-error or formal-proof claim.
