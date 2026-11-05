# roundtrip-document-codecs

## descriptant

### roundtrip-document-codecs: wave32

Real joins: blocks::concat→Volume; site_md::inline→vol::with_marks via Block::written; site_md::repeat→Copier and vol::residual_with mirrors copy decisions; site_code::encode_heb/decode_conllu→site_heb::predict_with/diff/apply; site_md::listed and run_blocks→heb::par_each. Independent tagged paths: site_learn CLI adaptation and heb::run/model parsing are not invoked by pure codec tests; numeric report helpers are projections, not part of reconstruction inverse. R1 is only a block-text residual: substituting it for full Volume layout loses literals. Site-code Chain requires same ordered canonical readings and resource state; leaf identities alone do not prove this full wiring.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave32 report. Registration is not a universal proof or whole-system runtime claim.

## model

# roundtrip-document-codecs

## forward

Markdown/plain text → Volume(pieces,blocks,marks); chapter grouping rebases Text indexes without adding separators. Markdown repeat/give reclassifies stored pieces. Token forms+space bits+R1 reconstruct block text; volume_text restores markup/layout. Site derived codecs separately reconstruct Conllu/FrTable pieces from fixed canonical rows/readings plus patches/scripts. Pure Hebrew row prediction supplies site_code HRow canonical values; par_each preserves ordered job results for site listing/run wrappers. run reporting projects tallies, while site_learn pure markup/escape helpers provide dictionary representation.

## invariant

Generated Text indexes and markup byte offsets stay aligned across concat, page_volume and volume_text; generated line scripts preserve target bytes for fixed canonical lines. Exact diagnostic reconstruction is conditional on residual/resources, not semantic understanding or persistent learning. Greedy boundaries preserve chapter order and group membership; layout pieces do not imply any deployed-site behavior.

## cost

Let B text bytes,C chapters,P pieces,W words,L canonical/target line lengths,H history bytes. Group/layout copying O(B+C+P); inline worst O(B²); Copier bounded <=51 candidate comparisons but O(H) retained history, repeat clones H per trial. LCS O(L1*L2) time/memory; tree validation and Hebrew prediction can be quadratic in W. Parallel framework O(jobs+threads) with retained per-job slots; reporting adds cell scans/sorts. Resource loading and pipeline runtime unmeasured.

## composition

Real joins: blocks::concat→Volume; site_md::inline→vol::with_marks via Block::written; site_md::repeat→Copier and vol::residual_with mirrors copy decisions; site_code::encode_heb/decode_conllu→site_heb::predict_with/diff/apply; site_md::listed and run_blocks→heb::par_each. Independent tagged paths: site_learn CLI adaptation and heb::run/model parsing are not invoked by pure codec tests; numeric report helpers are projections, not part of reconstruction inverse. R1 is only a block-text residual: substituting it for full Volume layout loses literals. Site-code Chain requires same ordered canonical readings and resource state; leaf identities alone do not prove this full wiring.

## domain

Finite synthetic UTF-8 strings, valid Piece references/ordered byte-boundary marks, ten-field rows with unique canonical pair keys and '_' empty convention; delimiter-free nonempty core lexicon records, valid indices, representable unsigned counters and signed offset additions, finite bit costs. CPU/offline selected exact test names only. All external models, corpus/lesson reads, persistent learning, deployed sites and full pipelines excluded.

## limitations

Original production bytes preserved; six files append inline tests. No new repository Rust sources. Model declarations source-derived and finite tests falsify selected boundaries. Private-effect and annotator paths retain explicit inspection-only gaps. No percentage surrogate without useful metric; only future caps1/2/5/10%.

## checks

- blocks::heb::wave32::ordered_jobs_and_panics
- blocks::wave32::grouping_and_duplicate_id_projection
- blocks::tests::book_chapters_give_the_file_back_and_runs_stay_whole
- blocks::tests::a_chapter_without_headings_is_cut_at_paragraphs
- blocks::run::wave32::row_numeric_partitions
- blocks::site_code::wave32::script_fibers_and_reference_precedence
- blocks::site_code::tests::line_scripts_give_the_target_back
- blocks::site_code::tests::a_tree_needs_known_relations_and_a_root
- blocks::site_heb::wave32::patch_fields_and_canonical_pair_fibers
- blocks::site_heb::tests::affixes_cut_an_unknown_token
- blocks::site_heb::tests::a_learned_tree_comes_back_with_its_patches
- blocks::site_learn::tests::escaped_literals_come_back
- blocks::site_md::tests::a_page_gives_itself_back_and_its_prose_goes_up
- blocks::site_md::tests::inline_marks_and_order
- vol::wave32::copier_unicode_and_gap_reconstruction
- vol::tests::txt_pieces_give_the_text_back

## catalog

## behavior-complement

On valid synthetic codec transitions, concat introduces no separators; group/blocks keep each run within one group; script application never reorders retained lines; ordered scheduler results cannot swap job indices. Residual tallies exclude Given/derived literals at token layer; these exclusions are source-derived inner bounds, not universal pipeline proof.

## reverse-reconstruction

For fixed canonical tokens/rows and resource state, compatible antecedents retain all strings reconstructible through R1/marks/patches/scripts. Text-only, summed bit counts, natural keys, lowercased lemmas and learned majority lexicons admit many preimages. Reverse composition must retain shared Volume coordinates, history and Chain state; no unique original training or document origin follows.

## markup-coordinate-join

site_md::inline emits marks at current emitted UTF-8 byte length; vol::with_marks restores them. concat changes Text indices by block offset but does not change each block's local mark offsets. Swapping these coordinate spaces is invalid.

## hebrew-patch-join

site_code predicts Hebrew rows using lexicon/text/verse/given then site_heb::diff/apply. apply(diff(pred,lit))=lit requires ten fields and unique normalized pair keys; duplicate FEATS and empty string versus '_' are explicit gaps, not universal exactness.

## copy-history-join

repeat trials clone Copier and accept when len>=24 and cost<=0.25 literal_bits; residual_with reconstructs same previous prose order using remember/code. Different prior prose or disabled-repeat environment changes classification and cost; all history must agree.

## scheduler-report-join

par_each executes all indexed jobs and catches closure panics. run_blocks folds Results in input order, turns errors into one-volume red tally; row exact=exact_ok==1. Aggregation loses scheduling/error provenance and cannot reconstruct jobs from totals.
