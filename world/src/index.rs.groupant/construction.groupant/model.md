# fact-index-construction

## descriptant

# fact-index-construction

## Forward

Construct fields and typed facts from event roles, current plans/beliefs, state deltas and relation snapshots; sort by line and insertion order and renumber.

## Invariant

Builder-created n equals final position; missing anchors initially use sent=1, while events overwrite sent. Entity histories are assumed nonempty when states()[0] is read.

## Cost

Traverse states/deltas and lexical expansions then O(F log F) final sorting; strings and lemma sets scale with emitted field text and entity expansion.

## Composition

Current entity names can enter historical facts through name_of/ent_text. Event snapshots and entity-current projections therefore mix time scopes; query/evidence parents must not infer a complete historical replay.

## Epoch memory

## catalog

# fact-index-construction: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U is valid world history to ordered built-fact vectors. R emits only the explicit event/entity/delta cases then sorts and renumbers. U minus R includes n-position mismatch, ordinary Set facts for skipped Type/Class/Parent/A/B/Mother/Father/PartOf fields, and removal facts from ignored set-removal deltas.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Minimal history fixtures for each skipped delta.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Given built facts, compatible histories retain all ignored fields and deltas plus arbitrary changes invisible to current names and emitted text. Fact projection is not injective; preserve every history yielding the same facts.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Pair histories differing only ignored deltas.

## feeling-transition

Type: delta_partition. Domain, observation and operator: With previous tags p and current c, emit c minus p when nonempty; else emit end=true only when c empty and p nonempty; otherwise no feeling fact. Then p becomes c, so partial removal is silent.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: p={sad,happy}, c={happy}, then empty.

## story-order

Type: renumbering_fiber. Domain, observation and operator: For emitted tuples (line,old_n), final order is ascending pair and new_n=position. Reverse from final vectors recovers relative line order but not original category iteration unless source kind constraints are retained.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Equal-line facts from different categories.


## tests

### Scoped checks

This scope remains source-inspected: the executed parent probe constructs Fact values directly and does not invoke Index::build or feeling transitions. INDEX-CORRESPONDENCE-001 remains open for extraction.

See [parent evidence](../model.md#evidence). Structural checks remain distinct from behavioral correspondence.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 4ce7907bbff2da349a8cf20d8edacb81a90d91263e03286cc01088a3d7a18479; bytes [8681,19441). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-30 · document

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Composition refinement (2026-10-30)

Construction remains source-inspected, without native extraction oracle. Its output must connect by the same Fact identities to query/evidence. B is not inverted by arbitrary synthetic facts; a score-compatible fact may have no valid builder antecedent. World::position rejects zero and absent anchors; TextPos documents one-based sentences. No production change was made for malformed public Fact.sent=0.

Epoch: 2026-10-30. Subject: index.rs, SHA-256 c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1, bytes [8681,19441). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless ../model.md#evidence records execution.

#### Current source binding (2026-10-30) (catalog)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

### 2026-10-30 · tests

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Native coverage mapping (2026-10-30)

No new extraction test: construction relations remain source-inspected. Native synthetic facts bypass the builder. See parent ../model.md#evidence for domain audit.
