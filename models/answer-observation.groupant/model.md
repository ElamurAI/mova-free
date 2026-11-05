# answer-observation

## model

# Answer observation interaction

## forward

This parent owns reader.rs, ans.rs, qud.rs and sense.rs as a modeling subtree. Its behavior is a tagged relation over separate entry points, not one invented pipeline. Reader uses ans lexical/rendering/baseline helpers, qframe parsing, index search and span extraction on joined worlds/trees. ans also offers a standalone world-answer path. QUD tracks supplied or annotated sentences and consults sense animacy for noun Who slots. Sense independently emits parse plausibility flags from fixed global knowledge. Common lexical/tree/world context constrains all joins.

## invariant

Preserve source tags, one-based sentence conventions and explicit missing anchors. World/index/tree alignment is a caller obligation distinct from range safety. Coverage remains occurrence-based; qlemmas remains set-valued. Stack changes retain unrelated open questions and ordered IDs. Finite scores, valid tree references and the same lexical snapshot are hypotheses. No confidence calibration, semantic truth, unique reverse reconstruction or lossless answer rendering follows.

## cost

Use branch-specific child costs, not the sum of every owned file. A reader call pays question parsing + ans baseline + index/routing + span/coverage on at most the selected anchor, plus rendering; shared lexical expansions can repeat. Cover over W occurrences and T tree tokens costs roughly O((W+T)*L) expected lookup/expansion work. Standalone ans ranking is O(E*(Q*L+H)+E log E), with possible relaxed retry. QUD over S sentences, T tokens and Q open questions has conservative O(S*Q*T^2 + S*T log(S*T)) work plus lexical lookup and reasons. Sense check is O(V*T) plus global queries. Parsing, dictionary initialization, IO and memory failures remain external; no throughput or composed approximation bound.

## composition

Ownership children are world-reader, world-ans, world-qud and world-sense. Actual call edges are reader->ans, reader->qframe/index/span and qud->sense->global; tree/world/types and lexical en dependencies cross this subtree. Ownership is not the call graph: no reader->qud or ans->sense execution is asserted. Composition joins the same Qa/frame, ordered facts, source/own-world anchor interpretation and lexical version. All reverse relations retain these shared intermediates. Structural source bindings cover owned children only; cross-subtree dependencies require parent inventory and independent evidence.

## epoch-memory

First parent epoch. Prospective predictions: duplicate question words survive coverage while disappearing from qlemmas; QUD noun Who acceptance follows current sense animacy under otherwise eligible frames; valid anchors alone do not establish source alignment. Source inspection confirms the reader owner now uses checked subtraction for anchor lookup; final binding remains integrator-owned. Keep section preference distinct from baseline hard filtering and selective QUD removal distinct from popping the newest stack item. No useful continuous error metric supports a Rust surrogate for these categorical policies.

## checks

Integration harness world/tests/groupant_joins.rs contains question_frame_reader_join_preserves_multiset_coverage (512 supplied Tree/QFrame cases) and sense_animacy_joins_who_question_eligibility (four nouns). The first has independent count/set expectations; the second consults real sense::animacy and checks wiring only. Leaf native checks cover confidence fibers, section envelopes, score priorities, QUD histories and sense roles. The six-test harness passes twice per profile after integration; suggested dedicated coverage/animacy parent mutations remain unexecuted. See model.md#evidence.

## limitations

Full qframe/parser acquisition, reader::read end-to-end arbitration, lexical factual validity, arbitrary malformed trees, large integer boundaries, nonfinite scores, and all world alignment scenarios are outside executed parent evidence. No teacher/GPU/corpus or training run. Partial child syntax does not become a composed semantic AST. Final children and exact source bindings are reconciled; structural validation checks their identity, not semantic completeness.

## Input and measurement expansion

The first ten views retain their answer/reader/QUD/sense scope. The input and report branches add reconstruction identity, ordered QA records and denominator-aware observations. Their actual call edges pass through existing ans metrics and world state, with source and world identity retained. These additions do not turn a finite metric value into an independent quality certificate. The formerly external ftqa source now has its own scoped groupant; earlier statements that it was unregistered describe the preceding epoch.

## catalog

# Typed interaction views

Every view is source-derived and partial, conditional on the final integrated source bindings and stated domains. Named finite tests support implementation correspondence only in their declared fixtures; no universal semantic claim is made.

## behavior-complement

Type: behavior_complement. Domain U consists of tagged supplied-world answer, reader-arbitration, QUD-transition and sense-flag observations with finite scores, valid dependency structures and a fixed lexical snapshot. An inner exclusion model rejects reader state selection below confidence six, normalized coverage outside [0,1], complete QUD attachment removing unrelated questions, or noun Who eligibility contradicted by fixed animacy (absent alternative fillers). These exclusions concern implemented policies; semantically wrong answers remain possible. Child complements cannot simply be united because the tags and shared intermediates differ.

## reverse-reconstruction

Type: reverse_reconstruction. For parent relation R, B(y)={x : R(x,y)} retains compatible worlds, fact orders, trees, query frames, lexical snapshot and prior QUD stack. Reader text may come from baseline, span or slot; retain that source tag and anchor if observed. Equal text alone loses this distinction. Reverse composition unions over shared compatible intermediates, rather than multiplying independent score, frame and lexical fibers. No unique history or inverse solver is claimed.

## anchor-identity

Type: interface_obligation. Reader joins fact.sent to own trees through checked_sub(1) then get. At the reviewed repaired owner snapshot zero/out-of-range gives no tree, eliminating the local subtraction panic. This does not validate fw/ix/own alignment: an in-range anchor may refer to different text. A high-confidence slot can still carry its supplied candidate sentence without a tree; do not infer every output anchor is valid. ans event ranking still indexes text by event.sent-1 and requires valid one-based anchors. Root must bind the repaired reader revision before promoting this current-policy statement.

## score-confidence

Type: constraint_relation. ans score_words gives each query class one highest-priority contribution: verb 3, participant 1.5, text 1, else zero; duplicate classes can contribute again. Reader consumes index Scored values and sconf keeps score when pred or words>=2, otherwise caps at 5.5; several routing branches supply their own confidences. Selection threshold six is a heuristic policy, not calibrated correctness probability. Neither ROUGE-L nor confidence is interchangeable with factual truth or QUD completeness.

## coverage-multiset

Type: encoding_relation. For retained frame word occurrence list W and anchor lemma set L, cover=sum_{w in W}1[lemmas(w) intersects L]/|W|, with empty W mapped to zero. qlemmas instead unions expansions into a set. Both combine main content/subject and subordinate content/subject and skip stop/short words. Duplicating apple in [apple,pear] changes cover from 1/2 to 2/3 against apple without changing qlemmas. The 512-case join checks count multiplicities independently on three synthetic terms; replacing cover with set overlap violates this interface.

## section-policy

Type: priority_relation. Section semantics belong to each API: ans event ranking adds +3 own, +0.5 adjacent, -1.5 other, so distant events remain eligible; index score similarly uses bonuses, with future handling. ans baseline hard-filters to q.secs and returns (0,empty) if none selected. reader sec_span forms the min/max matching sentence envelope, including intervening nonselected sections, with a whole-text fallback when none match. It is not exact section membership. No parent-wide hard-filter or preference rule is valid.

## arbitration-source

Type: priority_relation. Reader first obtains baseline, candidate, candidate anchor tree, span and coverage. No candidate or conf<6 selects baseline. At sufficient confidence, Describe/Num with nonempty slot prefers slot; otherwise coverage>=0.5 with span selects span. Remaining priority is slot, any span, baseline. Happen clause/after and adjective feeling paths can lift effective coverage to 0.5. This is a source-derived branch model; helper/join tests do not yet exercise the entire parser-to-arbitration route.

## lexical-who-coupling

Type: dependency_relation. QUD animate accepts PROPN without lookup, and NOUN only when sense::animacy returns Animate. Other filler conditions still require role, novelty and unresolved status. Sense derives Animate/Inanimate only when exactly one category family survives exception suppression; both/neither produce Unknown. Its own parse-error checker excludes PROPN/pronouns and checks VERB nodes, a different domain from QUD eligibility. Four controlled noun Who frames compare QUD attachment to the actual animacy result; this is a dependent wiring oracle, not independent validation of noun facts.

## selective-stack-transition

Type: state_transition. Let K be ordered open questions and E the indices congruent with a new assertion. If E is nonempty choose max(E), regardless of completeness. Complete removes only K[max(E)]; partial leaves all K intact. A newer partial match can block an older complete one. If E empty, accommodation produces implicit attachment without pushing; questions push their sentence ID. Given lemmas and last subject update afterward. This transition is separate from reader confidence; a complete QUD answer need not be factually true.

## observation-loss

Type: information_loss. Answer text, None-vs-State, source, anchor, notes, QUD reasons and sense flags are distinct observation levels. ans None may retain an anchor despite empty text; number-word normalization loses digit spelling; ROUGE tokenization loses case/punctuation. QUD givenness preserves requested order and duplicate terms even though given/answer are sets. Unknown animacy yielding no sense violation is absence of a detector signal, not evidence of plausibility. Hold the observation projection fixed before comparing fibers or agreement.

## qa-record-metric-join

Type: metric_input_identity.

The input branch reconstructs ordered stories and QA records with story names, question numbers, section references and two answer references. report::eval_tale uses that exact record to obtain state/anchor/baseline projections; ROUGE is relative to its references, not a truth predicate. Composition must preserve the document, section numbering, record multiplicity and source tag. A normalization-equivalent question string alone cannot identify a measurement row. Child and native-join evidence state the actually executed subdomains; broad data reconstruction and answer quality are separate claims.

## question-order-fibers

Type: ordered_identifier_preimage.

Story order is first appearance in input; question numbers are positions within that story. Normalized section duplication and whitespace can erase source distinctions, while inserting a prior record can change later identifiers. For an aggregate output retain all compatible ordered inputs, reconstruction choices, world states and reference texts; equal averages do not reconstruct question identities. A sample keyed by story and question number requires that same identity epoch. No inverse dataset reconstruction is implemented.

## metric-denominators

Type: projection_denominator_boundary.

A reachable aggregate adds one to n for every row and to answered only for state-source rows. All-question scores divide by max(n,1); answered-only scores divide by max(answered,1). Thus zero answered rows do not mean the full-question score was measured on the same denominator as a nonempty answered subset. Report rows with unrecognized attribute tags can contribute to totals while the fixed attribute display omits their bucket. Manual verdict inputs and publicly constructible rows can violate ordinary finite-score expectations; their acceptance is not factual validation. Formatting loses precision, so reconstruction retains all pre-format scores compatible with the displayed value.

## positional-answer-identity

Type: cardinality_identity_gate.

rreport requires equal question/answer counts before scoring. Equal counts are necessary, not sufficient: caller ordering must retain question identity. Answering, rounded metrics and verdicts are separate observations. Native mismatches reject before scoring; supplied aggregate fixtures do not certify nonempty answering.

## answer-observation-wave12-state-grade-provenance

Type: answer_provenance_denominator.

Construct Ada in Hall through typed commands. Parsed questions yield one hit, one miss, one ordinary abstention and one separately counted honest control. Invalid-first-query fallback agrees with the direct answer and preserves observed location history. Natural-language question understanding is outside this check.

Wave12 source-derived view; the supplied domain and observation are above. Executed scoped integration evidence is recorded in world/tests/groupant_wave12.rs.groupant/model.md#evidence. Fixed dependencies and shared identities are premises; finite native joins do not establish full-pipeline behavior or recover unique input history.

## answer-observation-wave12-narrative-query-order

Type: temporal_anchor_transport.

A three-event chain queried in permuted order retains E1,E2,E3 and anchors 1,2,3 through QA3 sort and grading; reversed expectation fails. Literal grading agrees with QA, while expected none intentionally differs. Missing events and empty sort abstain. Arbitrary narrative metadata remains outside the fixture.

Wave12 source-derived view; the supplied domain and observation are above. Executed scoped integration evidence is recorded in world/tests/groupant_wave12.rs.groupant/model.md#evidence. Fixed dependencies and shared identities are premises; finite native joins do not establish full-pipeline behavior or recover unique input history.

## evidence

### Answer observation evidence

#### Source inspection

Read actual owner source and adjacent model catalogs for reader, ans, qud and
sense. Reviewed source SHA-256 identities (inspection snapshots, not final bindings):

Reader inspection confirms checked_sub(1) then get in anchor_tree. This removes
the local zero-anchor subtraction fault, but does not guarantee that a returned
slot's sentence is valid or that source and own worlds agree. The parent model
states both limitations. ans ranking still requires valid one-based event anchors.

Read the integration harness world/tests/groupant_joins.rs and relevant index
section-scoring lines. No claim that ownership children form a sequential call
chain: reader uses ans plus external qframe/index/span; qud calls sense animacy.

#### Executed versus planned evidence

The integrator reported an initial debug pass of all six groupant_joins tests.
That report is provenance, not a new execution by this parent author. Two tests
specifically address this group's interfaces:

- question_frame_reader_join_preserves_multiset_coverage: eight sentence term
  subsets times 64 three-term count vectors = 512 cases. Arithmetic counts and
  expected sets are independent of production cover/qlemmas.
- sense_animacy_joins_who_question_eligibility: person, dog, stone and an unknown
  synthetic noun under controlled Who/run frames. Expected admission is obtained
  from actual sense::animacy, establishing dependency wiring, not lexical truth.

The other four joins concern neighboring ownership groups and are not credited
as this parent's independent validation. Leaf evidence remains in the respective
source sidecars; no aggregate leaf-test count or repeated release result is
invented here.

Integrated debug/release repetition and structural validation are recorded below.
Dedicated controls for this parent remain suggestions:
discard duplicate question terms before coverage; admit all NOUN Who fillers
regardless of sense animacy. Such controls must compile and fail the named joins
before being described as executed.

#### Scope

No full reader::read/parser acquisition was run by this parent task. No live
teacher, GPU, training, corpus evaluation, IO pipeline or factuality validation.
No executable percentage-error surrogate or composed semantic AST is supplied.
Source correspondence outside the finite joins is an inspected hypothesis.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first parent epoch. Prospective predictions: duplicate question words survive coverage while disappearing from qlemmas; QUD noun Who acceptance follows current sense animacy under otherwise eligible frames; valid anchors alone do not establish source alignment. Source inspection confirms the reader owner now uses checked subtraction for anchor lookup; final binding remains integrator-owned. Keep section preference distinct from baseline hard filtering and selective QUD removal distinct from popping the newest stack item. No useful continuous error metric supports a Rust surrogate for these categorical policies.

### 2026-10-30 · evidence

2026-10-30. This parent adds ten typed interaction views without editing Rust,
the central registry or ancestor manifests. At authoring, child IDs and transitive bindings awaited integration. The final
manifest now contains reconciled children and exact source bindings.

| Source | SHA-256 |
|---|---|
| world/src/reader.rs | 383aeebdc514654bdeeba383bfe6b5c00d6c799ba65f06f29621da0392dfaf1b |
| world/src/ans.rs | c8cadb5f0b8205ded6fe4fdf3c1e558fac4a894fae2c4041ac62c4ae63ca1690 |
| world/src/qud.rs | 68ab6b4d5bfabd85537f076f949432157e9f61d6bfbc65f94fc4d722c374b74f |
| world/src/sense.rs | 32033a19e1ffaf797f7eabb7a82bc564ddcf3146db700bbb4000ee6eec91a0a8 |

#### Integrated native evidence, 2026-10-30

The integrated checkout passes 138 focused native tests twice in debug and twice in release, with identical normalized test identities/results. This includes 61 new leaf tests, six new actual-API joins and 71 selected existing regressions. The six joins all pass after the reversed-operand control was restored. Leaf source controls, the original defective branches and the late-invalid-index-filter mutation fail the named assertions in their respective evidence. All registered child identities and transitive hashes are reconciled in the final manifests. These checks do not execute a full reader/parser, teacher, school corpus or GPU benchmark, and do not constitute a whole-world lint result.
