# quantity-reading

## model

# Quantity reading and exact script observations

## forward

With text T and fixed dependency context K, mentions produces an ordered array M. The real opchoice text path calls mentions_all, enumerates integer candidates C(M), extracts feature occurrences and selects using weights. treeread independently acquires words through frames::parse_words, constructs Q/Ask and applies ordered schemas. quant separately parses anchored rational scripts and folds commands over sentences into state, answer and diagnostics. Neither topic nor coincident answers establishes a production opchoice-to-quant or treeread-to-mentions edge.

The explicit integration adapter takes three Mention values in {0,1,2,3}. For every actual candidate c it constructs fully parenthesized E(c,M), parses "@1 ask E", and runs it on one sentence containing all three numbers. Its relation requires answer=rational(c.value), empty diagnostics and empty state. This is a test construction, not automatic production substitution.

## invariant

Candidate tuples preserve distinct positions, first-pair commutative canonicalization and distinct third index. Arithmetic uses checked i64 operations, nonnegative-result subtraction and integral division. Score arithmetic still needs bounded weights/features. Quant uses exact rational arithmetic subject to resources and gates. The embedding is one-directional: admitted integer candidates embed in rational expressions; arbitrary rational programs do not embed in Cand.

Preserve candidate order, full Mention identity and sentence anchors. K includes normalization, dictionaries, feature environment and parser/weight caches. treeread requires representable intermediates; f64 numeric-surface parsing and coarse thing matching are not exact linguistic interpretation.

## cost

For n mentions there are at most 3n(n-1) first candidates; only for n=3..5 can each gain up to 4(n-2) second steps. For C candidates, F feature occurrences and W weights, scoring costs O(C F log W), excluding lexical acquisition/features. Mention extraction can scan quadratically in tokens; repeated tree-child/tag scans are conservatively cubic per sentence under the child model, plus parsing.

For script length L, E expression nodes, N extracted numbers and rational bit length B, parsing costs O(L), origin checks O(E N) rational comparisons, execution sums operation costs at B plus state-map costs. The bridge executes every candidate and is not an optimization. Units are visits/comparisons/bytes, not measured time. Acquisition, allocation bounds and live training remain unknown.

## composition

Actual edges: opchoice -> mentions; treeread -> frames, outside the immediate children. Quant consumes independent script/sentence inputs and math::big. The test bridge explicitly supplies expressions and sentences. Compose only along these named edges.

Reverse composition unions compatible intermediate states with the same M, indices, sentence prefixes and K. Multiplying independent fibers may admit different occurrences or future-sentence numbers. This parent is a tagged family of observations, not a claim all branches jointly answer a question.

## behavior-complement

Type: behavior_complement. U contains tagged admitted API input/output pairs with fixed K and sufficient resources, plus the finite adapter domain. An inner absent-behavior model excludes reused indices, invalid division, unrepresentable candidate results and second steps outside n=3..5. It excludes green quant outputs with invalid anchors, foreign literals, undefined references or zero division, and destination credit from missing-source moves. Within the finite bridge domain, mismatching rational/candidate answers are excluded.

This partial source-derived absence has finite evidence. It does not exclude incorrect language answers, score overflow outside bounds or state effects accompanied by diagnostics. No complete complement of English meaning is supplied.

## reverse-reconstruction

Type: reverse_reconstruction. For tagged y, B(y) retains every admissible text, record array, candidate/weight state or anchored script history yielding y with fixed K. Input inclusion x in B(F(x)) holds for this relation; uniqueness requires injectivity. Answer fibers union all equal-valued candidate tuples and compatible script histories, including overwritten values and earlier asks.

The join exhausts candidate tuples for 64 arrays, not all texts/scripts. Scalar 1 admits 1+0, 1*1 and 2/2 among many antecedents. Gold equality cannot reconstruct unique quantity identity, narrative intent or causal provenance.

## integer-embedding

Type: encoding_relation. E(c,M) computes M[x] op M[y], then, if present, op2 with that result on the left and M[z] on the right. On admitted small-domain candidates exact rational evaluation equals the integer result. An independent widened i128 oracle constructs expected tuples before real quant APIs execute.

The converse fails: rational scripts admit fractions, negative results, references, state updates and longer expressions absent from Cand. Checked-i64 overflow omission is not rational overflow policy. Adapter agreement validates neither extraction nor learned choice.

## occurrence-identity

Type: constraint_relation. Candidate operands are positions: equal-valued or fully equal records at distinct positions may participate. mentions::best instead excludes every record equal to its excluded Mention across all fields, including sentence, tree and raw words. Equal value or score alone does not define this identity.

Permuting M may change canonical indices, features and tied choices while preserving its value multiset. Scalars erase occurrence information. Native exclusion fixtures and tuple oracles observe distinct identity policies; neither may be replaced by answer-value deduplication.

## second-step-order

Type: priority_relation. Extension means ((M[x] op M[y]) op2 M[z]), not M[x] op (M[y] op2 M[z]). First Add/Mul pairs use canonical orientation; directed operations retain valid orientations. The first result remains the second step's left operand. First/extended tuples coexist even when values coincide.

The bridge checks actual indices and association. Tuple-set equality and equal set/vector cardinality check omissions, additions and duplicates; selection order is separate. Reversing adapter operands preserves some commutative cases but breaks directed cases; the compiled control reaches the quantity gate assertion.

## selection-context

Type: state_dependency. Fixed candidates, feature occurrences and bounded weights determine the first maximum. Repeated feature strings contribute repeatedly; set projection is insufficient. A winner's compatible weight region has strict comparisons against earlier candidates and nonstrict comparisons against later ones. Answer fibers union these regions over equal-valued winners.

Feature flags, cached kind settings, dictionaries and weight publication are dependencies. The bridge bypasses scoring. top's lexical tie ordering must not replace choose's enumeration ordering.

## reading-branches

Type: information_loss. treeread Q/Ask, holder inheritance, tags and schema priority differ from Mention extraction. Its comparison ties retain the first candidate, unlike best's strict tie refusal. Total/rate/each can include question quantities; total precedes rate/sharing. Its numeric parser passes through f64, unlike exact quant arithmetic.

Equal scalar answers may arise from different antecedents and prove neither parsing nor units nor complete question exclusion. Supplied-word tests isolate mechanics; frames::parse_words remains external acquisition.

## anchor-provenance

Type: provenance_constraint. At command anchor j, admitted literals come from sentences 1..j plus fixed implied constants, 0, 1 and allowed reciprocals. Even after later sentences were extracted, membership must use the endpoint for j. Visiting 2,1,2 cannot license at anchor 1 a number learned only at anchor 2.

The peer correction records prefix endpoints instead of using all accumulated numbers. This constrains literal membership only: state references follow command order, not sentence provenance. For example, @2 set a.x=19; @1 set b.x=a.x; @1 ask b.x can remain green when 19 occurs only in sentence 2. No temporal provenance guarantee applies to referenced state. The bridge uses only anchor 1 with all operands present; backward-anchor literal safety and reference leakage are separate child regression observations.

## diagnostic-effects

Type: state_transition. Invalid anchors and evaluation failures skip effects. Foreign literals add diagnostics but may still change state or answer. Errors therefore do not imply rollback; dropping diagnostics changes semantics. Acceptance requires no diagnostics and applicable answer/gold conditions.

Missing-source move now skips transfer and cannot credit the destination. Valid transfer between distinct existing keys conserves their sum; missing destination starts at zero; same-key transfer preserves value. Conservation does not certify narrative ownership. Ask-only bridge scripts cannot independently verify transfers.

## checks

Parent filter: cargo test -p world --test groupant_joins operation_candidates_join_exact_quantity_scripts -- --test-threads=1. It checks 64 triples against an independent candidate tuple oracle, rejects duplicates and runs over 1000 exact scripts. Child filters: mentions::wave8_models, treeread::wave8_models, opchoice::wave8_models, quant::wave8_models. Child premises remain authoritative.

## epoch-memory

Root reports initial six-test join suite passed debug. Its compiled reversed-operand control fails the named join's empty-error gate first, producing (1 / (0 + 0)) and division-by-zero/no-ask diagnostics, not an equality failure. Final repeated profiles and hashes are root-owned evidence. No algorithm or performance claim is added.

## limitations

No teacher scripts, held-out language accuracy, annotation acquisition, live dictionaries, weight publication, general malformed-input safety or resource guarantees. The bridge supplies synthetic Mention records, bypassing extraction, treeread parsing and learned selection. Final source bindings and repeated integration evidence are recorded in manifest.json and model.md#evidence. Child syntax projections are not a composed semantic AST.

## wave28-quantitative-understanding

qworld and deduct consume steps::Prob with aligned qs/descs. qworld rates change no cell immediately, but owner/thing context is updated before operation selection and can affect subsequent bare quantities. qread Set adds whereas qworld Set overwrites; qread's final event fold respects upto, while direct/relation/movement/initial-state decisions can inspect future events, so the complete function is not a prefix-state operator. steps oracle excludes repeated leaves; deduct gold can reuse a subtree but search excludes self-pairs. lex lookup requires caller normalization; understand does it. Missing-path test stops before Annotator load.

## catalog


## wave28-quantitative-understanding-representation-transport

Exact rational cells cannot be equated with f64 search without conversion/error domain. Preserve source occurrence indices and distinct additive versus overwrite Set semantics.

## wave28-quantitative-understanding-search-supervision

steps oracle forbids repeated leaves; deduct gold reuses identical nodes but self-combination search candidates absent, so syntactically accepted gold need not be reachable.

## wave28-quantitative-understanding-effect-boundary

Only missing path metadata/tempdir effects exercised; Annotator and frozen suite remain excluded.

## wave28-quantitative-understanding-cost-composition

Feature extraction, candidate clone/tree construction and exact arithmetic costs survive composition; no measured speed or global percentage bound.

## evidence

### Parent evidence boundary

The named operation_candidates_join_exact_quantity_scripts test enumerates all 64 triples from 0..3. An independent i128 oracle builds complete expected candidate tuples; set equality and set/vector size equality check membership and duplicate absence. Actual candidates are encoded into fully parenthesized scripts using their operand positions, then parsed and run through quant. Assertions require no diagnostics, rational answer equality and empty state. The test requires over 1000 evaluated scripts; exact total is not reported here.

Root reported the initial six-test integration suite passed debug. The compiled reversed-operand adapter reaches the named quantity join and fails its empty-error assertion: (1 / (0 + 0)) produces division-by-zero/no-ask diagnostics. It is not described as an equality-assertion failure. Restoration, repeated debug/release results and final evidence hashes are root-owned and must be recorded during integration.

Leaf evidence remains in the four adjacent sidecars. In particular, backward-anchor literal membership, command-order reference leakage, missing-source transfer and effectful foreign-literal diagnostics are child observations; the ask-only single-anchor bridge does not test them.

Manifest children and source bindings were pending at authoring and are now reconciled. Intended direct sources: world/src/mentions.rs, world/src/treeread.rs, world/src/opchoice.rs, world/src/quant.rs. Root wires actual registry IDs and final hashes. No composed semantic AST, runtime benchmark, approximation budget or whole-answer proof is claimed.

## History

### 2026-10-30 · epoch-memory

2026-10-30: initial parent epoch. Prediction: correctly indexed/associated candidates embed into rational execution on the admitted finite domain; reversing operands rejects. Peer models identify checked candidate overflow, anchor-prefix provenance and missing-source transfer corrections. Foreign-literal diagnostics remain effectful.

### 2026-10-30 · evidence

2026-10-30. Parent owner inspected world/tests/groupant_joins.rs in the integration worktree and the current mentions, treeread, opchoice and quant models/source. This parent adds no production code and reruns no teacher, parser or corpus.

#### Integrated native evidence, 2026-10-30

The integrated checkout passes 138 focused native tests twice in debug and twice in release, with identical normalized test identities/results. This includes 61 new leaf tests, six new actual-API joins and 71 selected existing regressions. The six joins all pass after the reversed-operand control was restored. Leaf source controls, the original defective branches and the late-invalid-index-filter mutation fail the named assertions in their respective evidence. All registered child identities and transitive hashes are reconciled in the final manifests. These checks do not execute a full reader/parser, teacher, school corpus or GPU benchmark, and do not constitute a whole-world lint result.
