# Canonical payload projection

## forward

For state s, P(s) contains schema/course/lesson ledger and frames/same/texts/weights/parts/isa/per. Ordered map iteration makes texts, parts and per canonical vectors. Frames retain their original order. fingerprint(s)=H(JSON(P(s))); the primary observation here is full payload JSON.

## behavior-complement

Type: behavior_complement. Domain and observation are the forward relation above unless narrowed here. U consists of state/payload pairs; R={(s,P(s))}. The source-derived inner bound U minus R includes a payload that copies stale by_ent or lesson_focus and one that changes a persisted weight. A frame permutation is not generally excluded as another input: the relation binds each specific state to its own order.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation are the forward relation above unless narrowed here. P^-1(p) contains every live state with the named persisted fields equal to p and arbitrary omitted caches, indexes and focus. For digest-only observation h, union those fibers over every p with H(JSON(p))=h. Collision resistance is an engineering assumption, not injectivity.

## vector-orbits

Type: canonicalization_preimage. Domain and observation are the forward relation above unless narrowed here. For accepted unique-key texts/parts/per vectors, C maps each ordering to BTreeMap key order while preserving values. Independent permutations form one payload fiber. Duplicate keys are outside this admissible domain, and changing a value generally changes the payload even if a hash collision were possible.

## field-retention

Type: projection_dependency. Domain and observation are the forward relation above unless narrowed here. Changing any omitted live index leaves P unchanged. Changing one stored text changes P exactly at that entry, provided other fields are fixed. This is payload dependence, not a promise that a later answer changes; the bounded oracle compares full JSON.

## canonical-idempotence

Type: normalization_relation. Domain and observation are the forward relation above unless narrowed here. On admissible p, C(C(p))=C(p), where C=P after validated restoration. The equality depends on preservation of persisted frames and values. Canonicalization does not sort frames or deduplicate source entries; duplicates are rejected by the separate validation child.

## invariant

P has O(B) clone cost plus map traversal; no extra sorting pass is needed for already ordered live maps. The admissibility of arbitrary payloads is delegated to validation. SHA is shared with the whole unchanged Rust source.

## checks

oracle_canonical_payload_fiber_preserves_values_not_vector_order covers all eight 2x2x2 wire permutations and a value mutation. oracle_checkpoint_fiber_erases_stale_indexes_and_focus supplies omitted-field witnesses. General map cardinalities and hash collisions remain outside executable coverage.

Shared evidence is in ../../model.md#tests. Referenced older mutation controls are historical evidence, not rerun here. General semantic correspondence and unsampled behavior remain unknown.

## epoch-memory

Decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

## History

### 2026-10-29 · epoch-memory

2026-10-29: decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

### Undated · document

Subject: retention.rs, SHA-256 89715719216c1a94f9e701e24d4efea0b962bf04f5178d4916d19dc54536fb34. The manifest records the exact source interval. These source-derived relations are partial specifications; their evidence is bounded.
