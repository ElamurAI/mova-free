# mova-whale-english-tests

## Descriptant

End-to-end English tests of the Whales with Mova's 0.4 model: planner table (18 prompts, 10 refusals) and a served run through the real mova-whale child with a fake Codex executable and a 4,000..5,000-word fixture spec. Source: `tools/whale/tests/english.rs`.

## Forward approximant

Planner: Deputy::review per prompt -> capability list and first action's arguments, or failure stage with a review candidate. Served: fixture spec/catalog/fake codex -> animal::serve thread -> ask prompts over the socket -> assert answers, journal order, calls.log order and bundle results.

## Invariant

No model or network is called: the teacher is a fake executable bound by the config's codex path; skipped without MOVA_OCTOPUS_TEST_MODEL.

## Cost

About 60 s with the debug build (model load twice).

## Composition

Parent: whale-workers; exercises animal.rs, answer.rs, whale.rs, contract.rs and the octopus deputy.

## behavior_complement

Not covered: live teachers, concurrent workers > 1 in the served run, restart of a served process, every phrasing. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Failures name the prompt, plan and failure stage; passing runs leave no artifacts (temporary directory removed). No unique inverse is claimed.

## History

### 2026-11-05 · document

#### Epoch 2026-11-05

"Survey paper-2023-gamma." was pinned as a refusal because the 0.4 parse read «Survey» as a noun; the frame repair reads it as the imperative, so it is a positive now, with "Survey this paper: paper-2023-gamma." (colon rule). New controls: "Survey the paper.", "Do not survey this paper: X.", "Delete this paper: X.", "Describe this paper: X, not Y." and six questions about describing, pausing and resuming that must never plan those actions.
