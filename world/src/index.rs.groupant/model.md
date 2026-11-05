# fact-index

## descriptant

# fact-index

## Forward

Build typed facts from world events and entity histories, sort by (line,insertion n), renumber, then score admitted facts and sort by descending score and ascending fact n.

## Invariant

Built facts satisfy n=vector position. Facts retain anchors separately from sentence used for section scoring. Scoring domain assumes sent>=1 and finite representable totals; arbitrary public Fact mutation need not satisfy these conditions.

## Cost

Build scans events, states and deltas, performs lexical/entity expansions, then O(F log F) sorting. Search scans kept facts with role/word membership work and sorts K scores in O(K log K). Hash-set membership is expected-time; lexical dependency cost is separate.

## Composition

Construction child supplies ordered facts; query child consumes fields/IDs/sentence; evidence child projects labels and selected fields. Query's section grandchild refines the score term. Shared n, sent and world text connect children. Parent inference-core must bind question construction and answer selection. Complement of composition is not union of local complements; evidence reverse retains scoring intermediates.

## Epoch memory

## catalog

# fact-index: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: Let U be valid World/query/filter to fact-and-score observations with sent>=1 and finite totals. R builds, scores and sorts. U minus R includes a kept result violating deterministic score ordering or equal-score n ordering, and built n values unequal to positions. Semantic truth and public malformed facts remain outside this projection.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Independent construction oracle remains proposed; bounded scoring/ranking/label observations are recorded in model.md#evidence. INDEX-CORRESPONDENCE-001 remains partially open.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: For an observed ranked projection y, R^-1(y) includes all worlds, queries and filters producing y. Dropped fields, unmatched query alternatives and omitted facts create ambiguity; rank or score never uniquely recovers world history.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate worlds/queries sharing one score sequence.

## fact-order

Type: stable_order_constraint. Domain, observation and operator: Build first appends facts in category iteration order, then sorts by (line,old n) and resets n. Equal-line ties preserve insertion order; final n is not entity ID or sentence number.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Construct equal-line event and attribute facts.

## anchor-split

Type: provenance_projection. Domain, observation and operator: B::push sets sent=first anchor or 1; event construction overwrites sent with e.pos.sent. Thus anchors can be empty or differ from scored sentence; labels display all anchors without guaranteeing support.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Event whose position differs from first anchor.

## field-lexicon

Type: set_projection. Domain, observation and operator: Field lemma set is lemset(text) union lemset(ent_text(id)) over IDs. Repeated lemma occurrences collapse in HashSet, while fields and their ID vectors retain their separate multiplicity.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Repeated word/ID synthetic fields.

## predicate-score

Type: boolean_saturation. Domain, observation and operator: Predicate contributes exactly 3 iff any query predicate alternative matches the first Pred field's lemma set. Further matching predicates do not add points.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: One versus many matching alternative groups.

## subject-score

Type: role_partition. Domain, observation and operator: For nonempty q.who, in-role membership yields +3 and who=true; only-any-field membership yields +1 and who=true; absence yields -2 and who=false. Empty q.who contributes zero. Agent tests f.who across all current Kind variants.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Role versus other-field versus absent table.

## other-multiplicity

Type: multiset_effect. Domain, observation and operator: Each occurrence in q.others contributes 1.5 if f.has_id, so duplicate IDs repeat the bonus. This list is not set-normalized by score.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Duplicate a matching other participant.

## word-maximum

Type: weighted_projection. Domain, observation and operator: For each q.words alternative group, add the maximum weight among matching fields, or zero. Sum across groups; repeating a group repeats its contribution. Pred=1.5, When/About=0.5, other slots=1.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: One group matching multiple fields and repeated groups.

## rank-ties

Type: ordering_fiber. Domain, observation and operator: Search applies keep before scoring and sorts descending finite score, then ascending n. Every permutation sharing tied scores and n compares equal; unique build n gives a total order.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Reorder input facts with equal scores and distinct n.

## temporal-state

Type: history_projection. Domain, observation and operator: Plans/beliefs use current values but first-state line/anchor; attributes use deltas per state. Feeling emits added tags, or an end only when empty after nonempty; partial removals have no standalone removal fact.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Three feeling transitions and changed plan stage.

## label-loss

Type: evidence_fiber. Domain, observation and operator: Label keeps src/key, Value only for Attr/Have/Part, all anchors and cause; it omits line, sent, neg, end, by and most fields. Distinct contradictory or ended facts can share a label.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Two facts differing only neg/end/by.

## half-unit-observation

Type: integer_feature_projection. Domain: finite synthetic public Fact/Query values with positive sent, literal lemma sets and ordinary allocation. Let P be first-predicate hit, H in {0,6,2,-4} the subject half-points, O the matching other-ID occurrence count, W the sum of per-word maximum weights in half-units, and T in {0,6,5,1,-3} the ordered section half-bonus. For bounded counts 2*score=6P+H+3O+W+T. Diagnostic words=W/2. This is an exact real-valued specification only while binary half-unit accumulation is representable; tests use small counts. Independent integer coefficients avoid using Slot::weight as their oracle.

Falsifiable native check: half_unit_score_oracle (432 observations). Finite observations only; score is a heuristic, not truth probability.

## tests

### Focused checks

The source-bound probe is [probe.rs.txt](probe.rs.txt). See [evidence](model.md#evidence)
for executed case counts, the false-formula control and remaining gaps.

Build the default CPU world library with cargo build -p world --lib --locked.
Compile the probe with rustc --edition=2024 --crate-name index_groupant_probe,
passing its artifact path, --extern world=PATH_TO_BUILT_WORLD_RLIB and
-L dependency=THE_SAME_TARGET/debug/deps. Run the resulting binary twice.
The --false-formula invocation is expected to fail with exit 101.

The probe tests 432 scores, three rankings and eight label preimages on constructed
facts. INDEX-CORRESPONDENCE-001 remains open for index construction, feeling
transitions and complete semantic correspondence. No parser, teacher, training,
live service or GPU execution is involved.

#### Native tests

Run cargo test -p world --lib --locked index::groupant_tests -- --test-threads=1,
then repeat with --release before the test filter. Run each twice and compare
normalized test-name/result lines; elapsed timings are not semantic output.
See model.md#evidence for six oracle domains and limits. The old probe remains a
historical supplementary fixture and does not replace native tests.

Run mova-modelcheck syntax REPO world/src/index.rs OUTPUT twice; compare JSON.
Run mova-modelcheck changed REPO REGISTRY BASE OUTPUT with the supplied updated
registry-node entry overlaid into an external registry copy. Shared registry
and parent groupants are deliberately integrated by the parent owner.

The semantic mutation control recompiles a temporary predicate bonus +4 in
place of +3, expects half_unit_score_oracle failure, restores exact source bytes
and reruns all six focused tests. This is a production mutation, not a changed
expected answer. Construction coverage remains explicitly absent.

## evidence

### Bounded fact-index correspondence

The probe.rs.txt artifact invokes the actual public Index::score, Index::search
and Fact::label APIs using constructed facts and a one-sentence World. It does
not run parsing or Index::build. Integration linked the existing default-CPU
world library built from the unchanged 5794fd531 source snapshot.

Executed twice with identical summaries:
- 432 score/flag observations: two predicate conditions, four subject conditions,
  three duplicate-other counts, three duplicate-word counts, and six literal
  section cases. Independent scalar tables supply the expected sums.
- Three ranking checks: negative-score facts remain after filtering; score ties
  use fact number; equal score and equal number retain input order while different
  predicate/word decompositions remain distinguishable in flags.
- Eight label preimages varying neg, end and by share the same emitted label.

The --false-formula control changes the independent expected predicate weight
from 3 to 4, leaving the production library unchanged. It fails with exit 101
at case 216. This rejects a false mathematical model; it is not a production
source-mutation test.

INDEX-CORRESPONDENCE-001 is partially covered. Construction, feeling transitions,
dictionary extraction, invalid anchors, all score combinations and full semantic
inference remain open. Valid-domain scoring assumes sent>=1 and finite small
counts. The sampled labels omit neg/end/by, so equal labels do not establish
equivalent facts. GPU behavior and speed are not measured.

Root plus four nested fragments still contain twenty-eight typed models.
AST gaps remain explicit. Structural inventory checks source/paths/ranges and
declared families; it does not certify these mathematical relations.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 4ce7907bbff2da349a8cf20d8edacb81a90d91263e03286cc01088a3d7a18479; bytes [0,23624). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-29 · evidence

2026-10-29, source SHA-256:

4ce7907bbff2da349a8cf20d8edacb81a90d91263e03286cc01088a3d7a18479.

### 2026-10-30 · document

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Composition refinement (2026-10-30)

Let B map world to facts, Q map (facts,query,text,keep) to ranked Scored and E map selected facts to labels. A joint output (ranked,labels) has preimage {w,q,k : Q(B(w),q,w.text,k)=ranked and E uses those same selected facts}. Label fibers cannot be substituted for score fibers. Native tests bypass B and establish only finite Q/E joins.

Epoch: 2026-10-30. Subject: index.rs, SHA-256 c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1, bytes [0,30859). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless model.md#evidence records execution.

#### Current source binding (2026-10-30) (catalog)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

### 2026-10-30 · tests

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Native coverage mapping (2026-10-30)

Six native tests complement the earlier external probe; see model.md#evidence for precise finite restrictions.

### 2026-10-30 · evidence

#### Current source binding (2026-10-30)

Subject revision sha256:c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1. Native tests append after byte 23624; all existing fragment scopes are unchanged. Root file scope is [0,30859). Earlier hashes and source sizes above are retained as dated epoch evidence, not current bindings.

#### Native oracle epoch (2026-10-30)

Current source SHA-256: c182d747ecc69b9e86a34ae5a49caa3841477c53719b2b8bb77b39e0ba4120d1.
Only a cfg(test) module was appended; production bytes [0,23624) and all
previous fragment scopes are unchanged.

Six independent native tests exercise actual APIs with synthetic public
Fact/Query/World structures and literal lemma sets:

- half_unit_score_oracle: 432 feature combinations, integer half-unit table,
  total and four diagnostic observations checked.
- rank_filter_projection_and_equal_key_stability: filtered negative scores,
  ascending n ties and two reversed equal-key inputs with distinct pred flags.
- roles_and_duplicate_field_evidence: five subject/role cases, repeated query
  subjects, duplicate and later-field IDs, empty first text and src exclusion.
- word_maximum_and_predicate_first_field: four lexical cases and a low-weight
  When/About maximum; first Pred saturation differs from all-field word lookup.
- section_priority_and_positive_missing_anchor: seven section cases including
  branch priority, positive absent sentence and u16::MAX, sec=0, empty secs,
  backward adjacency and irrelevant anchors.
- label_and_score_joint_preimages: 16 compatible full-Scored plus label
  antecedents; equal-label changed-predicate control changes score.

The fixture domain is finite small counts with one-based sent and no lexical
extraction, parser, teacher, training, live services or GPU. TextPos documents
one-based sentence numbers; World::position rejects anchor zero and absent
anchors. Although Fact fields are public, sent=0 remains outside that intended
domain. No production defect or robust unknown-anchor policy is invented here.
The initial World cursor's zero sentinel is not a valid accepted fact anchor.

Four typed specializations deepen integer feature observation, relational
query/section preimage joins, first-field versus all-field noncommutation and
ordered section fibers. There are now 32 typed models across root and four
fragments. Counts are inventory, not proof. Construction and dictionary
correspondence remain open; native tests cover score/search/evidence only.
Score is a retrieval heuristic, never a truth probability.

Validation artifacts live outside the repository. Run both debug and release
focused tests twice and compare normalized test-name/result rows; syntax
projections are compared byte-for-byte. The compiled negative control changes
the production predicate bonus from +3 to +4 while leaving the integer oracle
unchanged, then restores the source and repeats all six tests. This differs
from the earlier false-formula probe, which only changed expected results.
Whole-world Clippy is not claimed clean; unrelated modulo_one/never_loop
diagnostics remain outside this change.

#### Executed results (2026-10-30)

All six native tests passed twice in debug and twice in release. Normalized
test-name/result rows were identical within each profile; both syntax JSON
outputs were byte-identical. Changed-source registration reported one changed
Rust file and no missing entries using the external updated registry copy.
Five sidecar source bindings, UTF-8 ranges and all 32 catalog sections passed
the local structural check. The recompiled +4 predicate mutant failed the
integer oracle with exit 101 (observed half-score 8 versus expected 6). Exact
source restoration was followed by six passing debug tests. Artifact hashes
and source binding are in native-evidence.json. These are finite observations;
no extraction, semantic AST, speed gain or whole-world lint result is claimed.
