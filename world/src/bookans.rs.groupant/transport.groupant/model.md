# world-bookans-transport

## forward

prompt emits supplied note IDs/text in caller order. run sets subscription env, reads/splits book, annotates query, retrieves notes, exits if none, then teacher->trim->gate at most twice. Acceptance prints then optionally logs; rejection appends problems/previous answer; two rejections error.

## invariant

Supplied worlds require valid one-based event sentence anchors and consistent entity/state references. Book retrieval returns unique increasing in-range zero-based indices. Answer helpers do not mutate worlds. Copy gate acceptance establishes neither factual correctness nor a general copyright guarantee. Partial syntax is not semantic analysis.

## cost

Prompt bytes scale with notes/question/title; retries retain previous prompt/answer. Teacher/annotation/file latency and failure probability unknown.

## composition

Normalization feeds hashed membership and independent quote scanning. Retrieval combines index/lexical evidence before ranking and expansion. run feeds notes to prompt and at most two teacher calls, gates responses, then logs after printing. Query annotation, sentence splitting, teacher transport, environment mutation and log errors remain unmodeled semantic/effect dependencies. Local evidence does not certify that path.

## epoch-memory

Initial wave8 source review and native checks. Preserve ordering, ambiguity and external dependency gaps. No useful approximation metric is supplied for categorical dispatch, so executable Rust approximants remain absent. Tests are not fast surrogates. See root ../model.md#evidence for repeated profiles and compiled controls.

## checks

cargo test -p world --lib wave8_models -- --test-threads=1; repeat with --release before -p. Only named native helpers exercised; no live parser/teacher. Fragments use whole-file partial syntax projections plus source byte envelopes, not inferred types, expanded macros or semantic AST.

## behavior-complement

Universe U is admissible supplied inputs paired with categorical observations of scoped relation R below. The source-derived inner exclusion hypothesis rejects observations violating local priorities, thresholds or constructor outputs. Unmodeled branches and dependencies remain unknown; U minus a partial model is not a certified complement of the entire Rust file.

## reverse-reconstruction

B(y)={x in the declared domain:R(x,y)} retains all compatible latent tokens, scores, ties and states. Equal text or score does not identify a unique input/history. This is a relational specification, not an inverse solver. Native finite fibers give scoped evidence, not completeness outside their domain.

## trace-fiber

Successful printed answer may follow attempt1 or2; absent tries metadata histories ambiguous. Log-write failure does not imply no printed output.

## external-gap

Native prompt fixture checks order, IDs and instructions only; never teacher compliance, factual grounding or120-word limit. Query annotation and sentence splitting unmodeled.

## History

### 2026-10-30 · document

Partial source model, 2026-10-30. Each typed view shares this source revision, scope, assumptions and checks unless narrowed below. Observation is one finite helper call, not full conversation. Source-derived relations remain hypotheses supported by bounded correspondence; no general semantic proof.
