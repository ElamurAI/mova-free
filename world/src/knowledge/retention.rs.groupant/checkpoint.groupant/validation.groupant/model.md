# retention-checkpoint-validation

## model

# Ordered checkpoint acceptance gates

## forward

After read/deserialize, gates occur in order: schema, payload digest, weight length, unique source keys, source-ledger membership, per-frame source and global-link bounds, unique parts/per keys, kind provenance. Let g1..gk be this traversal's predicates; acceptance is their conjunction, first rejection is least failing index.

## behavior-complement

Type: behavior_complement. Domain and observation are the forward relation above unless narrowed here. U is decoded-envelope/outcome pairs with fixed runtime weight layout; R returns success only when every gate holds. An inner bound U minus R includes success for a bad schema, recomputed-hash out-of-range frame edge, missing source ledger, or duplicate relation vector key. Frame::link is not part of global-edge rejection.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation are the forward relation above unless narrowed here. Success reconstructs the entire set of envelopes satisfying all gates, not original source text or teacher intent. Error i admits inputs satisfying earlier predicates and violating i, with later predicates unconstrained. Deserialization errors have a separate byte-level fiber outside decoded-envelope scope.

## first-error

Type: decision_partition. Domain and observation are the forward relation above unless narrowed here. E_i={e | all j<i: g_j(e), not g_i(e)} for a gate occurrence i. An observed error message has the union of E_i over occurrences with that message; it need not identify the exact failed edge or frame. Frame iteration and quote/cause/next traversal refine the order. A bad digest shadows topology faults; resealing an invalid graph moves the observation to the topology gate. This partition specifies errors, not repair instructions.

## provenance-boundary

Type: admissibility_relation. Domain and observation are the forward relation above unless narrowed here. Every texts key must name a ledger lesson; every frame src must name a texts key. Kind relations accept src.0=KINDS_SRC or an existing texts key. Thus sentinel kind provenance bypasses text lookup, while frame provenance has no corresponding exemption. Ledger digest correctness against source text is not recomputed here.

## topology-scope

Type: structural_boundary. Domain and observation are the forward relation above unless narrowed here. For N frames, each quote, cause and next target must lie in [0,N). This allows cycles and self-links; sentence-local link is unchecked by this gate. Acyclicity or causal truth cannot be inferred from successful load. Proposed boundary oracle varies N and every global target independently.

## invariant

Each gate is fail-fast before index rebuild. Expected work is serialized bytes plus ordered-map construction and traversed graph links; exact cost depends on rejection position. Rebuilding a local temporary student before final kind validation does not expose a returned partial Retained.

## checks

tamper_and_unsupported_schema_are_rejected and structurally_invalid_graph_with_valid_hash_is_rejected execute selected gates. Roundtrip and canonical oracles exercise accepted fixtures. Exhaustive first-error, duplicate kind provenance and all graph boundary combinations remain proposed.

Shared evidence is in ../../model.md#tests. Referenced older mutation controls are historical evidence, not rerun here. General semantic correspondence and unsampled behavior remain unknown.

## epoch-memory

Decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

## wave34-composition

### Wave34 composition

Envelope prefix gates schema then digest then weight count. Source-link prefix collects texts, checks uniqueness, checks ledger membership, then checks each frame source before its ordered quote/cause/next targets. Temporary student initialization precedes kind-key collection and provenance checks; successful validation then permits restore. Every earlier rejection suppresses later work; local allocations are not returned partial retained state.

Reverse composition preserves compatible intermediate values and their shared ledger/course/cache constraints. Parent behavior complement is relative to its own coupled trace universe, not a union of child complements. Costs include each executed call once. Existing model.md/composition.md retain historical models and evidence.

## History

### 2026-10-29 · epoch-memory

2026-10-29: decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

### 2026-10-30 · document

#### audit-epoch-2026-10-30

The current source binding includes four appended synthetic oracles. Production
byte ranges remain unchanged. The [checkpoint audit](../../model.md#audit) refines the
behavior-complement and preimage models with resealed first-error witnesses,
provenance/sentinel cases, ordered occurrence and quote-owner observations, and
serializer-versus-validator trust boundaries. The earlier proposed quote and
provenance checks now have finite evidence; exhaustive gates, semantic code
equivalence and successful cold examination remain gaps. Current whole-file
partial syntax is materialized in the root sidecar; this fragment retains its
reviewed byte-range scope without claiming a semantic or expanded AST.

### Undated · document

Subject: retention.rs, SHA-256 89715719216c1a94f9e701e24d4efea0b962bf04f5178d4916d19dc54536fb34. The manifest records the exact source interval. These source-derived relations are partial specifications; their evidence is bounded.
