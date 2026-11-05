# world-ans-ranking

## forward

Supplied lemma-class score sums first match per query token: verb 3, who 1.5, text 1. Eligibility is (!need_verb || vhit) && (content>=min || (!who.empty && total>=4)). Role adds 3 for requested role, 1 other role, -2 absent, zero if who empty. Section adds 3/0.5/-1.5. Sort descending score then ascending sentence; relaxed retry follows strict emptiness only.

## invariant

Supplied worlds require valid one-based event sentence anchors and consistent entity/state references. Book retrieval returns unique increasing in-range zero-based indices. Answer helpers do not mutate worlds. Copy gate acceptance establishes neither factual correctness nor a general copyright guarantee. Partial syntax is not semantic analysis.

## cost

E event materializations and O(E log E) comparisons; ev_words scans P/B entities for every event and expands lexical classes. No indexed speedup claimed.

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

## ties

Stable equal-score/equal-sentence ties preserve encounter order. Fixture stores three distinct sentence positions reversed and checks final order.

## lexicon-boundary

lem_all/content/ev_words remain semantic dependencies. A verb hit can come from any content query word, not necessarily parsed main verb. Repeated query words accumulate scores; alternatives within a class do not.

## wave34-invariant-correction

The older invariant paragraph includes book retrieval and copy-gate statements belonging to another source. Those are retained as historical overreach, superseded here: ans helpers are read-only over supplied World/Frame; event anchors must be valid one-based sentence indices; None has empty text independently of anchor. Ranking tie order and number conversion are scoped local observations. No book retrieval or copy-gate invariant is claimed for this source.

## wave34-composition

ev_words and lem_all materialize evidence, then score-priority provides content score and vhit to eligibility. Eligibility checks required verb first, then roles/sections, then the content/rescue gate, then ordered sort. find_events invokes the strict pass first and invokes relaxed min=2 only if strict empty and need_verb. Children share the exact score/vhit and event identities; scalar score alone is insufficient to reconstruct eligibility.

## History

### 2026-10-30 · document

Partial source model, 2026-10-30. Each typed view shares this source revision, scope, assumptions and checks unless narrowed below. Observation is one finite helper call, not full conversation. Source-derived relations remain hypotheses supported by bounded correspondence; no general semantic proof.
