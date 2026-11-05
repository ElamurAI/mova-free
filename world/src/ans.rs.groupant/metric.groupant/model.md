# world-ans-metric

## forward

baseline restricts to requested sections and keeps earliest maximal lemma overlap, unlike event ranking preference. LCS uses one-row DP; rouge_l tokenizes lowercase ASCII alphanumerics, returns zero for empty input, otherwise harmonic LCS precision/recall. rouge_q maximizes over nonblank references.

## invariant

Supplied worlds require valid one-based event sentence anchors and consistent entity/state references. Book retrieval returns unique increasing in-range zero-based indices. Answer helpers do not mutate worlds. Copy gate acceptance establishes neither factual correctness nor a general copyright guarantee. Partial syntax is not semantic analysis.

## cost

Baseline scans eligible sentence lexical expansions. LCS n*m updates, m+1 usize cells. Token allocations scale with bytes.

## composition

Question parsing feeds subject/anchor resolution, then intent dispatch and formatting. Lexical analyses, entity snapshots and P/B records are unmodeled semantic dependencies. Ranking and dispatch share calls: do not double-count costs. Metric evaluation does not establish answer accuracy. Native tests supply World and Frame directly to ranking/why; full parse-to-answer and other intent branches remain outside executed correspondence.

## epoch-memory

Initial wave8 source review and native checks. Preserve ordering, ambiguity and external dependency gaps. No useful approximation metric is supplied for categorical dispatch, so executable Rust approximants remain absent. Tests are not fast surrogates. See root ../model.md#evidence for repeated profiles and compiled controls.

## checks

cargo test -p world --lib wave8_models -- --test-threads=1; repeat with --release before -p. Only named native helpers exercised; no live parser/teacher. Fragments use whole-file partial syntax projections plus source byte envelopes, not inferred types, expanded macros or semantic AST.

## behavior-complement

Universe U is admissible supplied inputs paired with categorical observations of scoped relation R below. The source-derived inner exclusion hypothesis rejects observations violating local priorities, thresholds or constructor outputs. Unmodeled branches and dependencies remain unknown; U minus a partial model is not a certified complement of the entire Rust file.

## reverse-reconstruction

B(y)={x in the declared domain:R(x,y)} retains all compatible latent tokens, scores, ties and states. Equal text or score does not identify a unique input/history. This is a relational specification, not an inverse solver. Native finite fibers give scoped evidence, not completeness outside their domain.

## token-fiber

KING and king have same tokens; apostrophe splits unlike bookans words. LCS length loses matching witnesses.

## bounded-oracle

Binary sequences lengths 0..4 give 31*31=961 pairs. Independent subset enumeration checks maximum subsequence length without duplicating DP recurrence.

## wave34-invariant-correction

The older invariant paragraph includes book retrieval and copy-gate statements belonging to another source. Those are retained as historical overreach, superseded here: ans helpers are read-only over supplied World/Frame; event anchors must be valid one-based sentence indices; None has empty text independently of anchor. Ranking tie order and number conversion are scoped local observations. No book retrieval or copy-gate invariant is claimed for this source.

## wave34-composition

baseline is a sibling path and does not invoke LCS. rouge_l tokenizes each string, handles empty streams, consumes lcs-row and converts its integer to harmonic precision/recall. rouge_q filters blank references then maximizes scores. LCS preimages must preserve token order and repeated positions, while tokenization adds further byte-level ambiguity; the score is not semantic answer accuracy.

## History

### 2026-10-30 · document

Partial source model, 2026-10-30. Each typed view shares this source revision, scope, assumptions and checks unless narrowed below. Observation is one finite helper call, not full conversation. Source-derived relations remain hypotheses supported by bounded correspondence; no general semantic proof.
