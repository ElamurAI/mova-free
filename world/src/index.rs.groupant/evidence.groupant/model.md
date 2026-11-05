# fact-index-evidence

## descriptant

# fact-index-evidence

## Forward

Project fact fields through get/text/ids, test entity membership and format a human label with anchors and optional cause.

## Invariant

get returns the first field with a slot; text filters empty string from that first field only. ids concatenates all matching fields and preserves duplicates. Label is not a proof of truth.

## Cost

get/text scan until first match; ids scans all fields and allocates matching IDs; label cost is proportional to rendered key/value/anchors/cause bytes.

## Composition

Evidence projection consumes construction facts and can describe query results, but label omits scoring and status fields. Reconstruction must retain full fact alternatives; a readable label cannot certify inference provenance.

## Epoch memory

## catalog

# fact-index-evidence: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U is fact plus accessor/label choice to observation pairs. R selects first field for get/text, all matching IDs for ids and prescribed label fields. U minus R contains text returning a later nonempty same-slot field when the first is empty, and label showing neg/end merely because flags are true.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Two same-slot fields with first empty and different flags.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: A fixed label's preimage includes every Fact differing only in omitted fields (neg,end,by,line,sent,who and unrendered slots). Attr/Have/Part additionally constrain rendered first Value; other kinds do not. Formatting may introduce further ambiguity.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Construct two distinct facts with identical labels.

## id-membership

Type: membership_projection. Domain, observation and operator: has_id(i) is true iff i occurs in who or any field.ids. src alone does not make it true; repeated occurrences do not change this Boolean even though ids returns duplicates.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Fact with src=i but no membership, then duplicate field IDs.

## first-versus-all

Type: accessor_noncommutation. Domain, observation and operator: For repeated slot fields, get observes only the first while ids flattens all. Reordering fields can change text without changing ID multiset; therefore replacing ids by get.ids is not equivalent.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Swap same-slot fields with distinct text and IDs.


## first-predicate-versus-all-words

Type: projection_noncommutation. Domain: finite synthetic public Fact/Query values with positive sent, literal lemma sets and ordinary allocation. Predicate matching reads the first Pred field only, whereas word matching takes maximum over every field, including later Pred fields. A later matching Pred can add 1.5 to a word group without setting pred. Similarly text returns None on an empty first field while ids still concatenates later field IDs. These projections cannot be substituted for one another.

Falsifiable native check: word_maximum_and_predicate_first_field; roles_and_duplicate_field_evidence. Finite observations only; score is a heuristic, not truth probability.

## tests

### Scoped checks

Eight parent-probe cases vary neg/end/by and retain one label. This supplies bounded witnesses of information loss; it does not characterize the complete label preimage or all field-access behavior.

See [parent evidence](../model.md#evidence). Structural checks remain distinct from behavioral correspondence.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 4ce7907bbff2da349a8cf20d8edacb81a90d91263e03286cc01088a3d7a18479; bytes [3807,4908). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-30 · document

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Composition refinement (2026-10-30)

Joint evidence tests retain the same full Scored and label across 16 variants of neg/end/by/line and unmatched fields. A same-label altered first predicate changes score, rejecting the inference that label equivalence entails scoring equivalence. Rendered provenance is a lossy projection, not proof.

Epoch: 2026-10-30. Subject: index.rs, SHA-256 c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1, bytes [3807,4908). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless ../model.md#evidence records execution.

#### Current source binding (2026-10-30) (catalog)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

### 2026-10-30 · tests

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Native coverage mapping (2026-10-30)

Native roles_and_duplicate_field_evidence and label_and_score_joint_preimages cover accessors and joint fibers, including the same-label unequal-score control.
