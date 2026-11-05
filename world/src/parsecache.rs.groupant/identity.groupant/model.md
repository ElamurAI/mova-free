# parse-cache-identity

## descriptant

# parse-cache-identity

## Forward

Construct a sorted dependency map and hash its JSON; optional file absence is an explicit sentinel, while required file/read failures propagate.

## Invariant

Mode and dependency names determine projection membership; file paths do not themselves enter optional file digests. The identity does not contain per-text parse results.

## Cost

Linear in bytes read; file_hash uses a fixed 65536-byte buffer, while driver bytes and serialized dependency map are materialized.

## Composition

identity calls file_hash, optional_hash, execution mode and model path providers. Selected node envelope includes hash, file_hash, data_root, identity and optional_hash. External execution-mode and model-path providers remain explicit unmodeled dependencies.

## Epoch memory

## catalog

# parse-cache-identity typed models

## nonbehavior

Type: behavior_complement. Status: partial_source_derived_model.

Domain and observation: U is identity input-observation/output tuples including mandatory dependency read errors; R is identity().

Operator and approximation: C subset U minus R contains successful identity after a failing executable/model hash read, and adding a CPU-only gpu/* part without the GPU branch. U also includes arbitrary maps; an unsorted insertion-history-dependent JSON key order is excluded by BTreeMap.

Assumptions: Stable exec mode/env; ignore hash collisions when inferring changed digest from changed parts.

Evidence and falsifiable oracle: Source-derived only; proposed dependency error injection and two insertion-order serializations. See model.md#tests for executed bounds versus prospective work.

## antecedents

Type: reverse_reconstruction. Status: partial_source_derived_model.

Domain and observation: Observe K; hold current executable and schema fixed; vary remaining dependency readings.

Operator and approximation: All dependency observations d with H(JSON(parts(d)))=K remain compatible. Equal optional file bytes and absent path choices need not be distinguished. Arbitrary omitted environment entries vary freely; no SHA inversion algorithm or unique recovery is supplied.

Assumptions: Whitelist exactly as source at bound revision; hash compression necessarily has many preimages.

Evidence and falsifiable oracle: Probe optional-file fibers only; full-key preimage enumeration prospective. See model.md#tests for executed bounds versus prospective work.

## streaming-hash

Type: chunk_invariance. Status: partial_source_derived_model.

Domain and observation: file_hash reads one byte stream through a 65536-byte buffer; hash sees the same bytes in memory.

Operator and approximation: Provided all reads succeed and EOF represents the end of a stable stream, file_hash(p)=hash(bytes(p)), independent of short read partition. Time Theta(B), hash state and buffer O(65536), excluding returned digest.

Assumptions: No concurrent write; Read does not fail midstream.

Evidence and falsifiable oracle: Probe lengths 0,1,65535,65536,65537,131073 compare direct and file hashes. See model.md#tests for executed bounds versus prospective work.

## absence-boundary

Type: metadata_partition. Status: partial_source_derived_model.

Domain and observation: metadata(path) outcome followed by optional_hash.

Operator and approximation: NotFound -> Ok(absent); other metadata error -> Err; metadata Ok -> file_hash, including its read failures. A directory is not equivalent to absence. Absence is represented as a literal string distinct from hexadecimal digests.

Assumptions: Platform metadata error classes; ordinary digest formatting.

Evidence and falsifiable oracle: Probe two absent paths, equal existing files and directory rejection. See model.md#tests for executed bounds versus prospective work.

## tests

### Checks

See ../model.md#tests and ../model.md#evidence for executed bounds. Models are conditional source-derived specifications; only the named probe observations are executed. The identity fragment has streaming/optional-hash evidence but no actual identity call. Batch and envelope fragments are exercised by finite probe oracles. Syntax ranges establish snapshot anchoring, not semantic correspondence.

## History

### 2026-10-29 · Epoch memory

2026-10-29, source SHA-256 da30cef6ff680f3c2853c158dada7d661307d0bbf0464bf058624bdd63d2c384. First source-bound instance; runtime source unchanged. Inspection exposed two important non-rollback effects: duplicate missing occurrences are sent separately and the final occurrence wins; hit/miss counters advance before parsing succeeds. Refresh is a persistent flag rather than a one-call invalidation. The source-bound partial AST is not semantic proof: no macro expansion, name resolution, cfg evaluation or type inference. General real identity/finish checks and concurrent-writer behavior remain untested. No speedup or complete parser-dependency guarantee is claimed.

### 2026-10-30 · Epoch memory

2026-10-30: whole-source identity refreshed after appending native tests;
this fragment's source range and selected syntax nodes are unchanged.
See [native transition domains](../model.md#transitions) and [current evidence](../model.md#evidence).
Identity computation and finish remain unexecuted; fake open_key identities
exercise only the cache transition/envelope boundary.

### 2026-10-30 · document

Subject: world/src/parsecache.rs at SHA-256 6545414761854d33b7b41f8a864a7f4c59eebeaee721eef68772bb4a7a652479; epoch 2026-10-30. Scope: source bytes [384,3551). Transition horizon is one invocation unless an entry names a call history. Successful ordinary allocation; no usize counter overflow; a stable filesystem during each modeled read; no claim of parser linguistic correctness. Source inspection is evidence of a conditional specification, not a semantic proof. Evidence status is bounded_execution only where model.md#tests names the exact checked observation; all other oracles are prospective.

### 2026-10-30 · absence-boundary

Native 2026-10-30 refinements: [transition relations](../model.md#transitions); source scopes remain unchanged because tests were appended.
