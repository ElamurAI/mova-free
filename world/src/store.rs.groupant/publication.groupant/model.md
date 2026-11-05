# world-store-publication

## descriptant

# Reference publication

## Forward

ref_set creates refs, formats hash plus newline, writes a PID temporary file, then renames it. ref_get reads UTF-8, trims and suppresses empty/error results. Source scopes cover these three functions only.

## Invariant

Successful serialized replacement installs the supplied bytes under the target name. No fsync, unique staging nonce, name confinement, multi-file commit or cleanup-on-error is supplied.

## Cost

O(bytes) writing and reference reading, plus directory and rename filesystem operations. Crash recovery and metadata persistence latency are not measured.

## Epoch memory

## catalog

# world-store-publication: typed model catalog

Domain: finite valid byte files, bounded IDs and successful allocation; an operation's arguments plus its initial filesystem and scheduler state are inputs. Observations include returns/errors and written bytes. No simultaneous writer, path traversal, malformed offsets or crash is assumed unless explicitly modeled. Source-derived conditional relation, not a proof of all Rust executions. Evidence: ../model.md#tests distinguishes executed probes from proposed oracles.

## nonbehavior

Type: behavior_complement.

U is safe-path single-writer calls x returns/file bytes. For source relation R, U minus R contains successful ref_set with final bytes unequal to hash plus newline and ref_get Some(empty). Inner bound under successful normal filesystem operations. Probe checks newline and empty suppression.

## antecedents

Type: reverse_reconstruction.

The fiber of ref_get Some(h) contains every readable UTF-8 byte string whose trim is h, h nonempty; None merges read errors, invalid UTF-8 and whitespace-only content. A successful write does not reconstruct overwritten bytes. Probe distinguishes whitespace, missing and invalid UTF-8.

## rename-failure

Type: failure_partition.

If temporary write succeeds and rename fails, Result is Err, temporary bytes may remain, and target remains whatever the OS left; no cleanup/rollback is implemented. Probe uses a target directory and checks the temp file remains.

## aliasing

Type: path_equivalence.

For fixed PID, with_extension maps a.bin and a.tsv to the same a.tmp-PID. Thus two distinct destination names may share staging storage. Model excludes concurrent writers; path equality oracle checks this collision without racing.

## History

### 2026-10-29 · Epoch memory

2026-10-29: fixed-PID extension replacement aliases distinct filename extensions. Negative observations and preimage fibers must retain old file bytes and whitespace ambiguity. See the parent tests for isolated successful/failed rename evidence.

### Undated · document

Every entry is a separate observation/constraint. Status: partial source-derived conditional model. Subject: world/src/store.rs at SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Reverse operators retain all compatible antecedents; complements are candidate inner bounds, not complements of an empirically fitted forward function. Proposed oracles are not executed evidence.
