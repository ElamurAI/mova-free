# roundtrip-layer-residuals

## descriptant

### roundtrip-layer-residuals: wave32

Real joins are pipeline -> frames/tree/world/tally, main -> pipeline/report, tests -> synthetic layer APIs. class supplies labels/costs. lib exposes API/path; reporting and path assembly are tagged independent from codec reconstruction. Root owns volume/mixed sibling joins.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave32 report. Registration is not a universal proof or whole-system runtime claim.

## model

# roundtrip-layer-residuals

## forward

pipeline composes world decode_exact -> Realizer.sentence -> frames.apply -> tree.decode_exact -> vol.decode_blocks. Frame scripts transport realized origins into parsed heads; world residuals restore global frames and local sentence links. report consumes Tally; CLI selects groups/options and merges per-volume tallies.

## invariant

Exact diagnostic reconstruction requires matching decoder flags/state and all residuals; normal decode and report scores are separate observations. No persistent knowledge or semantic equivalence is implied.

## cost

Sum actual layer costs: greedy frame alignment O(parsed*realized*lemma bytes), world sentence grouping potentially quadratic in frames and quote membership, local-tree sorting sum degree log degree, report cell sorting; annotation/model/I/O costs explicitly unmeasured. World decode_exact also costs O(S*Z+S) time for S requested sentences and Z retained empty-sentence indices, with O(S) output vectors; all-empty input gives quadratic membership comparisons despite zero frames.

## composition

Real joins are pipeline -> frames/tree/world/tally, main -> pipeline/report, tests -> synthetic layer APIs. class supplies labels/costs. lib exposes API/path; reporting and path assembly are tagged independent from codec reconstruction. Root owns volume/mixed sibling joins.

## domain

Small synthetic valid rooted trees, finite frames with sentence-local valid links and bounded allocations; nonoverflowing indices/counters, stable empty override environment.

## limitations

No Annotator/model loading, live corpus, teacher, GPU or complete CLI pipeline execution. Final trailing token spacing false for R2 reconstruction. Invalid arbitrary scripts/graphs are outside exact law.

## checks

- class::wave32::precedence_and_bits
- pipeline::wave32::grouping_drop_and_damage
- report::wave32::zero_normalization_and_rows
- tests::orders_and_projectivity
- tests::tree_script_gives_the_tree_back_and_a_damaged_one_does_not
- tests::world_gives_the_frames_back_and_a_damaged_residual_does_not
- tests::realizer_does_not_repeat_a_shared_subject
- tests::realizer_says_the_content_the_frames_keep
- tests::realizer_says_the_function_words_the_frames_keep
- tests::realizer_hangs_clauses_where_the_links_say
- tests::realizer_punctuates_by_clause_structure
- tests::tam_is_read_and_realized
- tests::pronouns_by_salience
- tests::the_tam_verb_group_keeps_the_frames_content
- tests::verbless_roots_are_fragments
- tests::the_rest_of_a_clause_and_a_phrase_is_kept
- cargo test --offline --locked -p roundtrip --bin roundtrip wave32::spec_failure_order_and_classes -- --exact --test-threads=1

## catalog

## behavior-complement

Exact_ok cannot follow an earlier retained representation mismatch; normal mode cannot recover omitted residual fields by assertion. No persistence/learning effects are performed by pure residual helpers.

## reverse-reconstruction

Encoder-produced residuals with stable state give compatible original sentence/tree/frame witnesses; tally and normal text admit many antecedents.

## head-coordinate-transport

R3 maps zero-based realized origins to one-based parsed heads; R4 maps local frame links to global indices then kept-event coordinates and back.

## decoder-state

Realizer seen ids and off flags must match encode/decode passes; same output alone does not recover state.

## empty-sentences

R4 empties plus XOR starts and nsents restore the original partition, while pipeline Unit changes reset groups.

## diagnostic-cost

Diagnostic bits and equal-set averages are not actual compressed byte lengths or semantic-quality percentages.
