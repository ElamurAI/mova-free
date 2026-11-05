# Checkpoint persistence and restoration

## forward

Let P(s) be the persisted payload projection and V(p) the load gates. On successful I/O, Save(s)=Envelope(P(s),H(JSON(P(s)))) and Load(e)=D(e.payload) iff schema, digest and V pass. D rebuilds indexes in frame order. The observation is the returned state or first error; filesystem failures remain explicit external outcomes.

## behavior-complement

Type: behavior_complement. Domain and observation are the forward relation above unless narrowed here. U is live-state/path/envelope/output tuples for one save or load call. R is the successful projection/seal or validated restore relation plus ordered failures. An inner bound of U minus R contains overwrite-success at an existing path, successful load with wrong schema/digest, and loaded true lesson_focus. This is a source-derived bound, not complement of an empirically fitted model.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation are the forward relation above unless narrowed here. For an observed restored state t, compatible envelopes have V(p), matching digest, and D(p)=t. Define C(p)=P(D(p)); compatible prior live states satisfy P(s)=C(p), not necessarily P(s)=p for noncanonical incoming vectors. A direct Save(s) wire antecedent additionally requires p=P(s), so noncanonical accepted p has no such direct save antecedent. Stale caches and focus remain unconstrained. A fingerprint alone admits the hash fiber, not a unique payload. Failed I/O observations do not identify unique prior disk contents.

## persistence-equivalence

Type: persistence_information_loss. Domain and observation are the forward relation above unless narrowed here. Define s~t iff P(s)=P(t). This discards parser caches, lookup indexes and lesson focus but preserves frame order, weights, source strings and lesson digests. D(P(s)) need not equal s; it belongs to the same persisted equivalence class when P(s) is admissible. Check unequal live fixtures with equal full payloads.

## write-prefix

Type: failure_effect_relation. Domain and observation are the forward relation above unless narrowed here. Save follows serialize -> exclusive-create -> write_all -> sync_all. Existing-path rejection preserves existing content; later failures may leave a newly created partial file. Successful sync does not establish directory-entry crash durability. A fault-injection oracle at each prefix remains proposed.

## restore-composition

Type: interaction_relation. Domain and observation are the forward relation above unless narrowed here. The canonical child supplies P/C, validation supplies V, and restore supplies D. Load success is V(p) followed by D(p); canonical payload observation is C(p)=P(D(p)). Validation and restore scopes overlap deliberately. Costs count validation and rebuilding once, rather than summing overlapping child envelopes.

## invariant

P clones stored graph O(B) payload bytes; serialization/hash O(B); validation and reconstruction add frame/entity/link walks and ordered-map operations. Cost constants and peak allocations are unmeasured. No claim that digest equality proves semantic equality.

## composition

Sequential relation: exists p, P(s)=p and V(p) and D(p)=t. Save and load have separate I/O prefixes; the relation does not imply an atomic transaction. Children canonical, validation and restore are overlapping views of the shared payload, not independent states. Validation must succeed before the restore loop returns a state. Reverse composition preserves all admissible p. Parent complement is U minus this composed R, not the union of child complements.

## checks

The existing canonical and erased-index oracles execute selected P/D correspondence. Tamper, schema and topology tests execute selected V failures; state roundtrip checks exclusive save. No I/O fault injection or successful fresh-process exam was run.

Shared evidence is in ../model.md#tests. Referenced older mutation controls are historical evidence, not rerun here. General semantic correspondence and unsampled behavior remain unknown.

## epoch-memory

Decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

## History

### 2026-10-29 · epoch-memory

2026-10-29: decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

### 2026-10-30 · document

#### audit-epoch-2026-10-30

The current source binding includes four appended synthetic oracles. Production
byte ranges remain unchanged. The [checkpoint audit](../model.md#audit) refines the
behavior-complement and preimage models with resealed first-error witnesses,
provenance/sentinel cases, ordered occurrence and quote-owner observations, and
serializer-versus-validator trust boundaries. The earlier proposed quote and
provenance checks now have finite evidence; exhaustive gates, semantic code
equivalence and successful cold examination remain gaps. Current whole-file
partial syntax is materialized in the root sidecar; this fragment retains its
reviewed byte-range scope without claiming a semantic or expanded AST.

### Undated · document

Subject: retention.rs, SHA-256 89715719216c1a94f9e701e24d4efea0b962bf04f5178d4916d19dc54536fb34. The manifest records the exact source interval. These source-derived relations are partial specifications; their evidence is bounded.
