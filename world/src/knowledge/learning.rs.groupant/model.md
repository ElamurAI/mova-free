# learning-memo

## composition

### File-root composition

The root retains the whole-file syntax projection, core model and twenty typed obernants. Its children refine two source regions: execution produces a LearnWork per call; accounting defines the record and how reports accumulate. Within execution, memo-policy and ordered-visits expose the classifier/caller coupling one level further. All anchors refer to the same whole-source digest; nesting organizes reasoning rather than extracting Rust modules.

Execution assumes accounting's default zero counts and empty timing vector; each visit increments exactly one count, and each timed pass appends one duration. LearnWork::add later combines reports componentwise with zero padding by pass index. That index alignment does not establish temporal alignment of different lessons. The add call sites lie outside this source and remain unmodeled here.

Composition assumes normal nonoverflowing arithmetic, fixed call-local world/options and the evaluator/learner obligations stated by execution. Summing valid work reports preserves count totals; it loses call and exercise attribution. A valid leaf does not prove the wrapper or aggregation callers obey these assumptions.

The file test module remains outside child coverage and provides the existing bounded oracles. Imports, complete external learner semantics, timing distributions and full inference correspondence are not decomposed or proved. The children do not partition the file and are not separate registry entries. Each child selects nodes from the root's partial syntax projection; its byte range is their enclosing interval. These checked anchors supply syntax/source identity, without establishing semantic correspondence or complete coverage.

## catalog

# Lesson-local memoization obernants

## absent-cache-traces

Type: behavior_complement. Status: partial_source_derived.

Model, domain and assumptions: U contains finite normal helper-call traces with fixed world/options, valid indices and nonoverflowing counters. Candidate inner bound on U minus actual R: reuse of Empty; evaluation/observation with zero passes; or work counters not summing to P*E. Source-derived, not a proved full complement.

Evidence/oracle: Focused tests cover selected exclusions; failures/overflow are outside U.

## compatible-learning-inputs

Type: reverse_reconstruction. Status: partial_source_derived.

Model, domain and assumptions: B(y) contains all initial weights, ordered exercises, pass counts and evaluator outcomes whose trace yields observed final weights/work counts y. It preserves ambiguity; counts cannot recover a unique lesson or initial state.

Evidence/oracle: Prospective distinct inert lessons with equal observations; no executable reverse solver.

## ask-block-equivalence

Type: weight_subspace_quotient. Status: partial_source_derived.

Model, domain and assumptions: For fixed ask, w~v iff their VIAS.len()*NF block is equal. A Scored memo accepts its stored block class. Equal answers additionally assume stable world/options and evaluator dependence on that block.

Evidence/oracle: changed_winner_invalidates_but_another_weight_block_does_not tests one changed and one irrelevant block.

## inert-antecedents

Type: eligibility_preimage. Status: partial_source_derived.

Model, domain and assumptions: For valid right index, Inert iff !understood OR count.is_some OR correct option has no best support. Correctness alone does not imply inertness.

Evidence/oracle: Reference fixture covers unsupported, count and unlearnable-correct categories; all Answer shapes not enumerated.

## correct-answer-instability

Type: decision_counterfactual. Status: partial_source_derived.

Model, domain and assumptions: An understood, non-count answer with supported correct option remains Scored even when currently correct. Changing its block can change the winner; the stored correct choice must not bypass invalidation.

Evidence/oracle: Existing changed-winner test supplies a concrete counterexample.

## preupdate-snapshot

Type: version_precondition. Status: partial_source_derived.

Model, domain and assumptions: The stored block precedes learn_answer. A later lookup is valid exactly when its current block equals that snapshot; a changing update invalidates its own memo.

Evidence/oracle: Existing 30-visit reference trace exercises real changes and recomputation.

## ordered-reference-equivalence

Type: trace_correspondence. Status: partial_source_derived.

Model, domain and assumptions: The target is equality of every ordered weight vector between cached and recomputed paths, under the shared evaluator contract. This is stronger than equal final scores.

Evidence/oracle: 270-case sweep over two weights, two orders, three lesson numbers and five pass counts passed.

## work-counter-simplex

Type: resource_accounting. Status: partial_source_derived.

Model, domain and assumptions: At normal return, e+r+i=P*E with all counters nonnegative and nonoverflowing. Many branch decompositions share this total; it cannot identify reused questions.

Evidence/oracle: 270-case sweep checks counters and observer count; existing fixture has 30 visits and nonzero reuse/inert work.

## zero-pass-boundary

Type: execution_complement. Status: partial_source_derived.

Model, domain and assumptions: Within learn_lesson_with, P=0 returns default work before preparation/evaluation/observation/timing. Outer learn_lesson still calls enabled first, so environment initialization is not excluded.

Evidence/oracle: Existing zero-pass check supplies a panic evaluator that must never run.

## empty-exercise-fiber

Type: empty_domain_projection. Status: partial_source_derived.

Model, domain and assumptions: With E=0 and P>0, counters and observer/evaluator calls are zero, but enabled timing may append P durations. Zero counters do not reconstruct P.

Evidence/oracle: Prospective empty-lesson probe in isolated timing configurations.

## call-local-reset

Type: lifetime_boundary. Status: partial_source_derived.

Model, domain and assumptions: Each call creates Empty memos; inert conclusions from earlier facts cannot survive a new call. First pass evaluates every exercise in a nonempty call.

Evidence/oracle: new_facts_cannot_reuse_an_inert_previous_lesson checks new support permits learning.

## exercise-order-counterfactual

Type: ordered_update_relation. Status: partial_source_derived.

Model, domain and assumptions: Permutation may change learned trajectories. Cached execution must match recomputation for that same order; order invariance is not claimed.

Evidence/oracle: 270-case sweep covers original/reversed order and P in {0,1,2,5,8}.

## shuffle-seed-preimage

Type: seed_encoding_preimage. Status: partial_source_derived.

Model, domain and assumptions: Seed=((lesson.n as u64)<<20) OR ((index as u64)<<1). Without range bounds the encoding is noninjective through bit overlap/truncation; fixed options and seed do not identify a unique lesson.

Evidence/oracle: Three lesson numbers covered; aliasing and shuffle-preimage experiments remain prospective.

## reuse-switch-observation

Type: configuration_latch. Status: partial_source_derived.

Model, domain and assumptions: OnceLock reads MOVA_LEARN_REUSE once: exact 0 disables, other values or lookup error enable. Later changes cannot replace that decision; direct helper calls bypass it.

Evidence/oracle: Prospective process-isolated switch probes; unit tests do not mutate global environment.

## timing-information-loss

Type: timing_projection. Status: partial_source_derived.

Model, domain and assumptions: pass_ns records per-pass elapsed time when enabled, not separate evaluator costs. u128 nanoseconds are cast to u64; no unbounded-duration preservation is asserted.

Evidence/oracle: Source inspection; timing distributions and wrap-scale cases are unmeasured.

## work-aggregation-preimage

Type: aggregation_preimage. Status: partial_source_derived.

Model, domain and assumptions: LearnWork::add sums counts and zero-padded pass vectors; the result loses partition attribution. Preimages include all compatible summand pairs, assuming no overflow.

Evidence/oracle: Prospective finite vector-pair oracle; timing does not become deterministic.

## memo-memory-budget

Type: resource_bound. Status: partial_source_derived.

Model, domain and assumptions: At most E memo slots exist. Scored stores a block and Answer; prepared options also consume space. Memo/preparation storage is O(E*(W+Answer footprint+option footprint)); enabled pass timing adds O(P) retained memory. This is not O(W) total.

Evidence/oracle: Symbolic bound only; allocator overhead and peak memory unmeasured.

## evaluation-cost-preimage

Type: cost_approximation. Status: partial_source_derived.

Model, domain and assumptions: Model preparation plus validity probes, evaluated*C_eval and reached update costs. Reuse removes evaluations while adding comparisons/snapshots; lower evaluated count alone is not a speedup.

Evidence/oracle: Visit counts tested; real-inference cost calibration remains separate.

## world-change-countermodel

Type: assumption_counterexample. Status: partial_source_derived.

Model, domain and assumptions: Block equality is insufficient if an evaluator reads changing world/options or other weight blocks. Such behavior is excluded by lifetime/dependency assumptions, not detected by Memo::valid.

Evidence/oracle: Cross-call new-facts test is executed; mutation within a call lies outside this model.

## inert-update-exclusion

Type: dependency_complement. Status: partial_source_derived.

Model, domain and assumptions: Skipping learn_answer for Inert is equivalent only if unsupported/count/no-correct-support answers cannot update relevant reference state. This is a learner dependency obligation.

Evidence/oracle: Fixture compares all ordered weight updates for these categories; other mutable effects remain outside the observation.

## tests

### Executable checks

Run: cargo test -p world knowledge::learning::tests --lib --locked

Existing checks cover changed-winner rejection with an unaffected other block,
a nontrivial 30-visit trace, reset after new facts and a zero-pass panic evaluator
that must not execute. No full school, teacher or GPU run was performed.

## core-model

### Learning memoization groupant

Subject: learning.rs at the manifest digest. The retained world and exercise data
are fixed during one call; weights change after ordered exercise visits.
Memo states: Empty, Inert, or Scored(answer, selected_weight_block).

#### Forward and invariant model

A valid memo either skips an inert update or reuses its Answer. Otherwise evaluate,
remember the pre-update block when reuse is enabled, then perform the applicable
update. Observe after each visit. Allocate fresh memos per call. With P passes
and E exercises, normal nonoverflowing runs satisfy evaluated+reused+inert=P*E.
A zero-pass helper call returns immediately.

The quotient is equality of the selected weight block, conditional on stable
world/options and evaluator dependence on that block. Caching targets equality
of every ordered weight update with recomputation. It does not imply identical
timings or all mutable caches/external effects.

#### Cost and evidence memory

Memoization follows Michie (1968), doi:10.1038/218019a0, already cited in source.
Reusing a Scored answer still performs learning. Comparisons, Answer storage and
block snapshots cost time/memory; fewer evaluations alone do not establish speedup.
Model preparation plus reached validity/evaluation/update work, then calibrate.

## History

### 2026-10-29 · composition

2026-10-29: first nested decomposition. Retained all earlier root evidence and models; no speed or accuracy change is claimed.

Syntax binding epoch, 2026-10-29: child scope envelopes now follow selected AST nodes, excluding adjacent indentation/newlines previously included in some manual ranges. Mathematical decomposition and source ownership are unchanged.

### 2026-10-29 · document

Subject: learning.rs at manifest.json SHA, 2026-10-29. Normal serial calls assume fixed world, prepared options/labels, evaluator dependence on the selected weight block, valid indices, sufficient weight storage and nonoverflowing arithmetic. Source derivation is not a general proof. Named tests cover stated finite domains; prospective oracles are unexecuted. Specialized types can overlap.

### 2026-10-29 · tests

2026-10-29: four tests passed. The new bounded_cache_reference_traces_cover_weights_orders_seeds_and_passes
covers 3*3 initial weight combinations, 2 exercise orders, 3 lesson numbers and
5 pass counts = 270 cases. It compares every observed weight vector and final
weights with recomputation, plus count/visit invariants. The evaluator is a
deterministic test fixture, not full parser/inference correspondence.

### 2026-10-29 · core-model

2026-10-29: four focused tests passed. The new 270-case sweep checks every ordered
weight vector against recomputation across initial weights, order, lesson seeds
and pass counts. It extends the existing single 30-visit example. Changed-winner
and new-facts controls establish why invalidation and call-local lifetime matter.
syntax.json supplies a partial source-bound AST projection. General semantic
equivalence remains a separate obligation. Timing storage adds O(P) memory when enabled.
