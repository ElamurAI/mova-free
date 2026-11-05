# fact-index-section-prior

## descriptant

# fact-index-section-prior

## Forward

Resolve sec from world sentence at sent-1, falling back to 0 for out-of-range positive sent; choose one section bonus by ordered conditions.

## Invariant

Assume sent>=1. Insec equals q.secs.contains(sec) even if q.secs is empty, in which case it is false and bonus is zero. f.anchors do not participate.

## Cost

One sentence lookup and up to several linear scans of q.secs; constant extra state. Parent already includes this scan cost.

## Composition

This grandchild feeds exactly one additive term to query scoring. Future-next priority precedes generic adjacency; same section precedes both. World sentence mapping is an external dependency, not inferred from anchors.

## Epoch memory

## catalog

# fact-index-section-prior: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U contains (positive sent, world section map, secs,fut,bonus,insec). R follows ordered branches: empty=0, same=3, future next=2.5, adjacent=0.5, otherwise=-1.5. U minus R includes additive combinations of these mutually exclusive bonuses and next=2.5 without fut.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Exhaustive small section/future truth table.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: Bonus 2.5 implies nonempty secs, sec absent from secs, fut=true and some x with sec=x+1. Bonus 0.5 requires adjacency and failure of earlier branches. Preserve all sets/lists and world maps meeting these constraints.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate compatible section-list fibers.

## fallback

Type: boundary_projection. Domain, observation and operator: Positive out-of-range sent maps to sec=0, which may receive same-section bonus if q.secs includes 0. sent=0 underflows the subtraction in checked builds and is not covered by this valid-domain model.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Boundary sent values with and without section 0.

## future-counterfactual

Type: counterfactual_rule. Domain, observation and operator: Hold sec,secs fixed and toggle fut. Only outside-same cases with an immediately preceding queried section move from adjacency 0.5 to 2.5; score delta is 2.0. All other finite inputs preserve bonus.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate sec=0..3 and section subsets.


## section-branch-fiber

Type: ordered_partition_preimage. Domain: finite synthetic public Fact/Query values with positive sent, literal lemma sets and ordinary allocation. For positive sent let sec=text[sent-1].section if present, otherwise 0. Bonus=3 fiber is secs containing sec; bonus=2.5 fiber excludes same-section and requires fut and a preceding sec-1 member; bonus=0.5 fiber excludes both prior cases and requires adjacent membership; bonus=-1.5 excludes them all with nonempty secs; empty secs gives 0. At sec=0 no preceding section exists, so fut cannot make fallback a next-section match. Anchors are free in this projection. Preserve every compatible text map and query list, including duplicates.

Falsifiable native check: section_priority_and_positive_missing_anchor (7 boundary/priority cases). Finite observations only; score is a heuristic, not truth probability.

## tests

### Scoped checks

The parent probe includes six literal section cases across 432 feature combinations: absent sections, same section, next section with and without prediction, preceding section and distant section. Extreme anchors and all multi-section sets remain open.

See [parent evidence](../../model.md#evidence). Structural checks remain distinct from behavioral correspondence.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 4ce7907bbff2da349a8cf20d8edacb81a90d91263e03286cc01088a3d7a18479; bytes [22480,23120). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-30 · document

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Composition refinement (2026-10-30)

Seven native cases observe branch priority, backward adjacency under fut, positive missing sentences including u16::MAX, section-zero fallback, empty query sections and irrelevant anchors. No observation extends to sent=0: the domain is one-based by TextPos and the world anchor gate.

Epoch: 2026-10-30. Subject: index.rs, SHA-256 c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1, bytes [22480,23120). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless ../../model.md#evidence records execution.

#### Current source binding (2026-10-30) (catalog)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

### 2026-10-30 · tests

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Native coverage mapping (2026-10-30)

Native section_priority_and_positive_missing_anchor covers seven boundary and priority cases. half_unit_score_oracle composes six section cases with other feature terms. Zero sentence remains outside contract.
