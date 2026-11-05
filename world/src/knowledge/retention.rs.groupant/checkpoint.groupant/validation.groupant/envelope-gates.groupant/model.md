# Envelope integrity and weight-layout prefix

Subject: `world/src/knowledge/retention.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

Read bytes, deserialize Envelope, check schema exactly, compare hash of reserialized payload with stored digest, then compare weight count with start_weights().len(). Successful prefix hands payload graph to provenance validation; it has not yet accepted a retained state.

## invariant

Schema failure masks digest/layout failures; digest failure masks layout failure. Input file is read only. Digest binds structured payload serialization rather than literal input JSON bytes.

## cost

O(B) file read, JSON decode and serialization/hash with O(B+decoded payload) memory, plus fixed runtime weight-vector construction. Filesystem latency unmodeled.

## behavior-complement

Type: behavior_complement. Over decoded envelopes and prefix outcomes, exclude passage with wrong schema, wrong reserialized payload digest or wrong weight count. Checksum agreement does not exclude forged testimony or arbitrary ledger strings, so authenticity is outside this inner bound.

## reverse-reconstruction

Type: reverse_reconstruction. For a schema error, all decoded payloads with wrong schema are compatible regardless of later fields. Hash error requires correct schema and unequal digest; layout error also requires matching digest and wrong count. Prefix success retains every envelope satisfying those three predicates, not a unique author or original byte encoding.

## schema-precedence

Type: first_error_relation. The schema predicate runs before checksum comparison; a malicious or accidental unknown schema is reported independently of whether later integrity also fails. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## structured-checksum

Type: serialization_identity. Whitespace/JSON representation can vary while decoded payload and its reserialized digest stay equal; raw file bytes are not the hashed identity. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## layout-shape

Type: compatibility_boundary. Weight vector length equals the current table length; equal length alone cannot certify feature meanings across code revisions. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## trust-limit

Type: authentication_exclusion. An adversary or ordinary editor can recompute a valid checksum for arbitrary testimony/ledger strings; load has no signing-key or teaching-truth verification. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

Checkpoint validation enters this prefix first; filesystem/deserialization errors precede schema, hash and layout. Source-link and kind gates follow only on prefix success. Checkpoint restoration remains outside the prefix.

## checks

`knowledge::retention::tests::oracle_checkpoint_integrity_is_not_authentication_or_validation` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.
