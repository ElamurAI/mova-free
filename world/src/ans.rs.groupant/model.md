# world-ans

## model

# world-ans

## forward

Scoped answer composition: lexical/question interpretation -> subject/anchor resolution -> intent dispatch -> State(text,note,anchor) or None(empty text,note,anchor). Ranking weights each query class exclusively: verb 3, participant 1.5, text 1, then role/section bonuses. Selected why paths prioritize explicit why, causal how, cause event, agent belief, goal, fulfilled intent; feeling-specific search precedes ordinary events. Other branches are read but not comprehensively modeled.

## invariant

Supplied worlds require valid one-based event sentence anchors and consistent entity/state references. Book retrieval returns unique increasing in-range zero-based indices. Answer helpers do not mutate worlds. Copy gate acceptance establishes neither factual correctness nor a general copyright guarantee. Partial syntax is not semantic analysis.

## cost

For E events, Q query classes, L lemma-comparison work and H snapshot/history work: ranking O(E*(Q*L+H)+E log E), potentially twice on fallback. Dispatch includes branch-dependent histories; no complete bound. LCS n*m cell updates and m+1 usize cells.

## composition

Question parsing feeds subject/anchor resolution, then intent dispatch and formatting. Lexical analyses, entity snapshots and P/B records are unmodeled semantic dependencies. Ranking and dispatch share calls: do not double-count costs. Metric evaluation does not establish answer accuracy. Native tests supply World and Frame directly to ranking/why; full parse-to-answer and other intent branches remain outside executed correspondence.

## epoch-memory

Initial wave8 source review and native checks. Preserve ordering, ambiguity and external dependency gaps. No useful approximation metric is supplied for categorical dispatch, so executable Rust approximants remain absent. Tests are not fast surrogates. See root model.md#evidence for repeated profiles and compiled controls.

## checks

cargo test -p world --lib wave8_models -- --test-threads=1; repeat with --release before -p. Only named native helpers exercised; no live parser/teacher. Fragments use whole-file partial syntax projections plus source byte envelopes, not inferred types, expanded macros or semantic AST.

## behavior-complement

Universe U is admissible supplied inputs paired with categorical observations of scoped relation R below. The source-derived inner exclusion hypothesis rejects observations violating local priorities, thresholds or constructor outputs. Unmodeled branches and dependencies remain unknown; U minus a partial model is not a certified complement of the entire Rust file.

## reverse-reconstruction

B(y)={x in the declared domain:R(x,y)} retains all compatible latent tokens, scores, ties and states. Equal text or score does not identify a unique input/history. This is a relational specification, not an inverse solver. Native finite fibers give scoped evidence, not completeness outside their domain.

## score-fiber

One query class scoring 3 has four (verb,who,text) Boolean preimages: 100,110,101,111. Lower-priority hits are lost. Native test enumerates all eight patterns.

## section-preference

Hold content/role fixed: own section adds 3, adjacent 0.5, otherwise -1.5. Section is preference, not eligibility. Equal total sorts by sentence then stable encounter order.

## fallback-partition

Strict pass uses need_verb and min=1; a nonempty strict result excludes all relaxed candidates. Only empty strict with need_verb=true retries without verb gate and min=2. Participant support can admit subthreshold content if total>=4.

## anchor-projection

None retains empty text despite nonempty anchor. State num_words normalizes space-separated 0..20 including 007; punctuation-attached numbers remain. Original digit spelling cannot be recovered.

## why-priority

Subjectless one-event fixtures: explicit why beats causal how; absent both and downstream slots yields None with event anchor. Feeling/history paths retain dependency gaps.

## metric-boundary

ROUGE-L is zero if either token stream empty; otherwise 2*LCS/(n+m) modulo floating arithmetic. Tokenization loses case/punctuation; score 1 does not imply byte equality.

## wave34-review

## wave34-invariant-correction

The older invariant paragraph includes book retrieval and copy-gate statements belonging to another source. Those are retained as historical overreach, superseded here: ans helpers are read-only over supplied World/Frame; event anchors must be valid one-based sentence indices; None has empty text independently of anchor. Ranking tie order and number conversion are scoped local observations. No book retrieval or copy-gate invariant is claimed for this source.

## evidence

### Answer helper evidence

`score_precedence_and_fibers` enumerates eight membership patterns and all four score-three preimages. `lcs_matches_subsequence_oracle` covers 961 binary-sequence pairs using subset enumeration. World fixtures exercise section preference, sentence ties and strict-to-relaxed fallback. Supplied Frame tests exercise why-before-how precedence and None with nonempty anchor. Constructors cover lossy numeric formatting and section-span fallback.

Compiled production control changes score_words verb increment from 3 to 2: `score_precedence_and_fibers` fails its named equality assertion. Source is restored before final checks; no runtime ans changes are retained. Control log: `ans-mutant.log`.

Each leaf and its fragments pass isolated groupant structural validation against a copied exact source/sidecar tree and one-node registry (zero diagnostics). Full registry/ancestor integration belongs to the parent task; a truncated registry against the whole checkout correctly reports unrelated ownership mismatches, and disconnected entries are rejected. Structural checks do not establish semantic correspondence. Current bindings are source exact; macros remain opaque.

No live parser, teacher, network, GPU or training. Lexicon tables and supplied World semantics remain dependencies. This is selected helper correspondence, not full answer dispatch accuracy. Recommended parent boundary: question frame -> entity/anchor resolution -> ranked candidate -> intent-specific slot -> separate state/anchor output; tie order, snapshots and section semantics must be shared across that interaction.

## wave34-evidence

### Wave34 helper evidence

Fifteen exact world library helper tests passed twice debug and twice release; sorted target-qualified numeric-status lists agree. Eight belong to read and seven to ans. New fixtures cover centered heading predicates, direct modifiers/original-form holes, modal/carry reset, participant rescue at exactly four, and ASCII-space numeric fibers. Existing independent 961-pair subsequence oracle remains selected. Three compiled production mutants were rejected by named runtime assertions: heading indentation, phrase determiner selection and event rescue total. Each restored backup hash matches exactly and its named test passed after restoration. No compilation rejection is counted as behavior evidence.

Syntax was generated twice after formatting additions and compared byte-for-byte; descendants share this one projection. Five new child groupants here retain actual partial AST/range bindings, six typed views each and explicit costs. Full source and historical claims reviewed before edits. No runtime bug fix, semantic task gain, performance gain or percentage approximation is claimed. Parser acquisition, model loading, teacher/corpus/frozen assessment, training and full answer pipeline remain unexecuted.

## History

### 2026-10-30 · document

Partial source model, 2026-10-30. Each typed view shares this source revision, scope, assumptions and checks unless narrowed below. Observation is one finite helper call, not full conversation. Source-derived relations remain hypotheses supported by bounded correspondence; no general semantic proof.

### 2026-10-30 · evidence

Heading `### Answer helper evidence — 2026-10-30` stays in evidence as `### Answer helper evidence`.

Five native ans tests and six bookans tests pass twice debug and twice release under filter `wave8_models`, single test thread. A restored debug run also passes all eleven. Normalized identities/results match across four profiles: SHA-256 `cf53c00ccc34b8a9cceb369b05c4253f2f9efc486fe7d2f69b5a7c15ee1fe67c` (joined successful test lines, without final newline). Raw logs live outside the repository under run label `octopus-wave8-answers-2026-10-30`, files `debug1.log`, `debug2.log`, `release1.log`, `release2.log`, `restored.log`.

Source SHA-256: `c8cadb5f0b8205ded6fe4fdf3c1e558fac4a894fae2c4041ac62c4ae63ca1690`. Syntax SHA-256: `ee6762602c2eb23bdbe1448470cf9639a753309e60504c8331fe74782f6668d3`. Two syntax generations match byte-for-byte. Three scoped fragments, eight root typed views and four per fragment give twenty typed views for this file tree.

### 2026-10-31 · wave34-review

2026-10-31: full source read; production bytes retained and cfg(test) coverage appended. Earlier evidence applies to its recorded hash only. Current source, syntax and selected helper receipts are in model.md#wave34-evidence. No training, annotation model, teacher, live corpus or full intent pipeline was executed.

### 2026-10-31 · wave34-evidence

Heading `### Wave34 helper evidence, 2026-10-31` stays in wave34-evidence as `### Wave34 helper evidence`.

Source SHA-256: d8d94c891e347d2b640744b20c4662dc35064368e7d4d9253c4954b4297e1e09. Production and pre-existing tests are byte-identical to base; only an appended cfg(test) module changes source. Earlier model.md#evidence remains a historical wave8/wave9 receipt, not a current-source claim.

Raw logs, command definitions, numeric statuses and source/test/mutation maps live under external run label octopus-wave34-reasoning-2026-10-31. Native log hashes are cataloged in evidence-summary.json. Current syntax SHA-256: 15a3c2f1acd5799af1cbad50c715b9d22602fd26b0b9ea39eaaa7476db7b30bb.
