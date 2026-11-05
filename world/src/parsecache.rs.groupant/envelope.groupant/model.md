# parse-cache-envelope

## descriptant

# parse-cache-envelope

## Forward

Write schema/key/checksum/tree envelope. Decode deserializes, checks schema and key, then canonical-tree checksum and returns trees.

## Invariant

Acceptance binds map bytes through canonical serialization; unknown envelope fields do not enter checksum. Checksum is integrity detection, not authentication.

## Cost

Serialization/checksum O(B); write clones map then serializes checksum input and envelope, so memory O(B) beyond stored map. Decode materializes input envelope and canonical map serialization.

## Composition

Calls hash, serde and store::write_atomic outside fragment scope. Key equality originates in the parent identity model. Output to parent is only tree map; counters/refresh/repaired are separately initialized. Write/decode range envelope contains the impl boundary and intervening syntax, without asserting all such bytes are semantically modeled.

## Epoch memory

## catalog

# parse-cache-envelope typed models

## nonbehavior

Type: behavior_complement. Status: partial_source_derived_model.

Domain and observation: U is all (byte-string,id,decode outcome) tuples; R is decode, including its deserialization failures.

Operator and approximation: C subset U minus R includes Ok with schema !=1, identity !=id or checksum unequal to canonical tree digest. Invalid bytes also cannot decode successfully. Deliberately altered valid trees plus recomputed checksum are not excluded.

Assumptions: No serialization error or digest-format anomaly.

Evidence and falsifiable oracle: Probe all eight combinations of three guard truth values; malformed JSON control. See model.md#tests for executed bounds versus prospective work.

## antecedents

Type: reverse_reconstruction. Status: partial_source_derived_model.

Domain and observation: Observe decoded T under fixed id.

Operator and approximation: Preimage consists of all JSON bytes deserializing into schema=1, that id, checksum H(JSON(T)), and T. Unknown members, whitespace and permissible key ordering do not affect this observation. Previous cache history, original question provenance and parser version beyond the supplied id are unrecoverable.

Assumptions: serde unknown-field behavior at bound implementation; collision resistance unnecessary for this acceptance equation.

Evidence and falsifiable oracle: Probe whitespace and added unknown envelope field; checks accepted edited trees with recomputed checksum. See model.md#tests for executed bounds versus prospective work.

## check-order

Type: first_error. Status: partial_source_derived_model.

Domain and observation: Malformed or typed Envelope bytes and id, observe returned error category.

Operator and approximation: Deserialization precedes combined schema/identity guard; checksum is checked only after both match. Schema and identity failures share the same message and cannot be uniquely distinguished from that error alone.

Assumptions: Compare categories, not unstable full anyhow formatting.

Evidence and falsifiable oracle: Probe eight guard combinations and verify identity-message priority over checksum mismatch. See model.md#tests for executed bounds versus prospective work.

## roundtrip

Type: conditional_identity. Status: partial_source_derived_model.

Domain and observation: Fixed serializable T,key and successful write followed by decode of resulting file.

Operator and approximation: decode(write_envelope(T,key),key)=T. This identity preserves Word tag, 128-bit feature value, head, relation, form/lemma, and empty trees. It is a left inverse on the data projection, not an inverse of memory counters, refresh flag or earlier cache history.

Assumptions: Single writer; actual store write succeeds; Word serde round-trip supports chosen values.

Evidence and falsifiable oracle: Existing test checks tag/features/empty trees; probe records cache round-trip and rejects corrupted contents. See model.md#tests for executed bounds versus prospective work.

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

Subject: world/src/parsecache.rs at SHA-256 6545414761854d33b7b41f8a864a7f4c59eebeaee721eef68772bb4a7a652479; epoch 2026-10-30. Scope: source bytes [6590,7254). Transition horizon is one invocation unless an entry names a call history. Successful ordinary allocation; no usize counter overflow; a stable filesystem during each modeled read; no claim of parser linguistic correctness. Source inspection is evidence of a conditional specification, not a semantic proof. Evidence status is bounded_execution only where model.md#tests names the exact checked observation; all other oracles are prospective.

### 2026-10-30 · roundtrip

Native 2026-10-30 refinements: [transition relations](../model.md#transitions); source scopes remain unchanged because tests were appended.
