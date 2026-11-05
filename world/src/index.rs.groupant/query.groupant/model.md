# fact-index-query

## descriptant

# fact-index-query

## Forward

Score predicate, subject-role, other-ID, word and section features; filter facts before ranking by descending score then n.

## Invariant

For valid sent>=1 and finite scores, sorting uses a defined comparison. q.others and q.words are lists and preserve duplicate effects; Pred uses the first matching slot field only.

## Cost

Per kept fact: query alternative/field scans and ID membership; q.words best-field search is proportional to total alternatives times fields in the simple bound. Search sorts K kept facts in O(K log K).

## Composition

Section child supplies one additive term from f.sent and world text, not all anchors. Predicate, subject and words share fields but are separately counted; do not deduplicate their evidence or add another child cost for the same scoring pass.

## Epoch memory

## catalog

# fact-index-query: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U is valid fact/query/world-text to finite Scored transitions. R sums the five terms and returns diagnostic flags. U minus R includes +3 predicate when no first-Pred alternative matches and +3 subject when only another role matches. Malformed sent=0 is outside domain due to subtraction underflow behavior.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Independent literal feature-score table.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: For score s, compatible antecedents are all feature assignments whose sum equals s, including role and section tradeoffs. Scored flags narrow but do not identify matched IDs or lexical alternatives; why text is not a full provenance trace.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate feature combinations sharing total.

## duplicates

Type: multiplicity_response. Domain, observation and operator: Adding one matching q.others occurrence adds 1.5; adding a repeated q.words group adds its best field weight. Adding another matching predicate group changes nothing once pred=true.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Single-feature duplicate interventions.

## tie-filter

Type: ranking_constraint. Domain, observation and operator: Search result set equals keep-filtered facts, including negative-score facts; there is no score cutoff or top-k truncation. Order is descending score and ascending n, not anchor order.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: All-negative scores with keep predicate and n tie tests.


## query-preimage-join

Type: relational_join_preimage. Domain: finite synthetic public Fact/Query values with positive sent, literal lemma sets and ordinary allocation. For full Scored y, compatible tuples (fact,query,text) satisfy the joint constraints on n, total, pred, who, words, insec and why. Sum-only fibers are larger. Composition with section is an existential join on the same sent/text/secs/fut; independently choosing a compatible section and incompatible query is invalid. With fixed query, ignored fact neg/end/by/line and unmatched extra fields yield compatible full-Scored antecedents; this does not reconstruct an input world.

Falsifiable native check: label_and_score_joint_preimages (16 joint antecedents; equal-label unequal-score control). Finite observations only; score is a heuristic, not truth probability.

## tests

### Scoped checks

The parent probe checks 432 literal score/flag cases, duplicate contributions and three ranking cases. Finite valid-anchor fixtures provide bounded evidence; arbitrary queries and full semantic quality remain open.

See [parent evidence](../model.md#evidence). Structural checks remain distinct from behavioral correspondence.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 4ce7907bbff2da349a8cf20d8edacb81a90d91263e03286cc01088a3d7a18479; bytes [19441,23624). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-30 · document

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Composition refinement (2026-10-30)

The independent oracle uses half-unit integer tables, never Slot::weight or Index::score for expected values. Section, predicate and subject terms share one fact/query state. Keep is applied before scoring, negative candidates remain, then ties use n; duplicate n preserves stable input order. Rank is a retrieval heuristic, not truth probability.

Epoch: 2026-10-30. Subject: index.rs, SHA-256 c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1, bytes [19441,23624). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless ../model.md#evidence records execution.

#### Current source binding (2026-10-30) (catalog)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

### 2026-10-30 · tests

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Native coverage mapping (2026-10-30)

Native half_unit_score_oracle, rank_filter_projection_and_equal_key_stability, word_maximum_and_predicate_first_field and label_and_score_joint_preimages cover finite query/composition observations.
