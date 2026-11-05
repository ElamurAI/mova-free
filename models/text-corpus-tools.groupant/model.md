# text-corpus-tools

## descriptant

### text-corpus-tools: wave33

Actual joins are within each tool; no cross-tool Rust caller is invented. A possible explicit artifact adapter maps Gutenberg fetched pg-id text into school book_paragraphs -> sentences, losing headers/underscores/short fragments. OCR output is Unicode line text and is not directly a school address TSV. Wiktionary page raw wikitext can be supplied to count/emit helpers only with edition/title/namespace context; these tools do not call one another. Tests exercise actual local helper joins only; cross-tool adapters remain source-derived.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave33 report. Registration is not a universal proof or whole-system runtime claim.

## model

# text-corpus-tools

## forward

Tagged union of seven independent CLI relations: Gutenberg CSV selection/fetch; Kaggle CSV feature/rule/group prediction; OCR normalization/correction/alignment/masking; school text cleanup/sentence/address export; Wiktionary count, idiom emission and exact-page lookup. Real in-file joins include csv_rows -> select/death_years gates, feats -> learn/predict_list, Channel -> Corrector -> token, letter_stream -> align, usfm_verses -> verse_sentences, EnAgg.page -> en_line/flush, labels/regions/plain -> emit, and read_stream -> find_page.

## invariant

Tagged routing prevents treating similarly named text artifacts as interchangeable. UTF-8 bytes, chars, row indices, passenger ids and verse coordinates are separate spaces. Repeated pure fixtures match across profiles. Lossy normalizers and aggregated counts do not uniquely recover input. File/output effects are sequential and lack rollback.

## cost

Add costs only along reached call chains, not across independent tools. Character/CSV scans generally O(bytes); conservative repeated wiki/cleanup scans O(bytes^2); OCR DP O((n+1)(m+1)) memory/time, word alignment O((N+1)(M+1)*(L+1)^2); Kaggle 36 candidates/row, at most twelve retained rules and thirteen candidate-building iterations and dense neighbor dedup O(N^3) per reconstruction; indexed pages decode streams repeatedly per title. Network/process/corpus throughput unmeasured.

## composition

Actual joins are within each tool; no cross-tool Rust caller is invented. A possible explicit artifact adapter maps Gutenberg fetched pg-id text into school book_paragraphs -> sentences, losing headers/underscores/short fragments. OCR output is Unicode line text and is not directly a school address TSV. Wiktionary page raw wikitext can be supplied to count/emit helpers only with edition/title/namespace context; these tools do not call one another. Tests exercise actual local helper joins only; cross-tool adapters remain source-derived.

## domain

Bounded synthetic UTF-8 text, byte/char slices, passenger rows and in-memory XML/wikitext; finite positive OCR score parameters, bounded arithmetic and allocations. CLI argument/filesystem/network/model/corpus environments are modeled conditionally and not executed.

## limitations

CPU only. Existing nine tests and seven new exact tests inspected. No downloads, full CLI effects, model loads, live teacher or corpus use. Original production prefixes unchanged; only inline tests. No percent approximation metric/guarantee established.

## checks

- gutenberg::tests::csv_and_death_years
- gutenberg::wave33::csv_boundaries_and_date_fibers
- kaggle::wave33::features_groups_and_empty_learning
- ocrfix::tests::brackets_rejoined
- ocrfix::tests::stream_and_cer
- ocrfix::tests::channel_corrects_and_keeps
- ocrfix::tests::negative_control_known_word_stays
- ocrfix::wave33::channel_depth_alignment_and_masks
- school-sents::tests::scans_close_pages_and_hyphens
- school-sents::tests::english_filter_and_hebrew_count
- school-sents::tests::quoted_stops
- school-sents::tests::hebrew_verses
- school-sents::wave33::normalization_and_verse_identity
- wikt-count::wave33::entities_sections_and_translation_partition
- wikt-idioms::wave33::gloss_labels_and_emit_precedence
- wikt-page::wave33::xml_ids_text_and_empty_page

## catalog

## behavior-complement

For each tagged relation, excluded effects follow its gates: no later matching Gutenberg group, no transitive Kaggle group edge, no short OCR word change, no cut-only two-word school sentence, no translation-subpage En section, no unlabeled single-word idiom row, no mismatched exact XML page. These are scoped source-derived exclusions; independent tool complements are not unioned into an invented pipeline.

## reverse-reconstruction

Recover compatible antecedents per tag: catalog rows satisfying first-match/date predicates; passenger feature/group-label fibers; pointed texts sharing corrected skeleton; markup texts sharing emitted sentence; page multisets sharing counts; wikitext sharing gloss; XML sharing Page fields. Preserve multiple possibilities and hidden rejected inputs; no unique corpus reconstruction.

## text-coordinate-transport

School verse marks are UTF-8 clean-string byte offsets; OCR edit alignments use char indices; Kaggle uses passenger u32 ids; Wiktionary streams use compressed byte offsets. An adapter must carry the coordinate tag and cannot reuse equal numeric values across spaces.

## lossy-artifact-join

Potential Gutenberg text -> school book cleanup strips header/footer, first line behavior, underscores and short cut fragments; reverse is a preimage including lost text. Raw Wiktionary Page.text requires edition/title context to enter count/emit helpers and does not imply six-column idiom TSV equivalence.

## effect-prefix-order

Each effectful command admits successful prefixes before later failure: pool before verses/counts, earlier Gutenberg rows before write failure, earlier requested pages before missing/parse failure. Pure helper successes do not establish atomic publication or remote availability.

## determinism-domain

BTreeMap rule conditions and explicit lexical tie keys stabilize selected paths; En reports sort ties by key, while Ukrainian morph/emptydef equal-count rows lack tie keys. Fixed synthetic helper outcomes cannot certify those unexecuted report orders or external subprocess schedules.

## History

### 2026-11-04 · descriptant

#### Epoch 2026-11-04: wikt-extract

The group gains three leaves for `tools/wikt-extract` (wikitext rendering, English entries, the streamed build and
module packaging). Its real in-crate join is `parts -> run(stream_rows -> pages -> entry::entries ->
wikitext::render) -> row -> sorted layer -> module.json`; like the other tools it calls no sibling tool (it repeats
`wikt-page`'s index-to-stream naming rule). Its output, the `wiktionary-en` add-on module, is CC BY-SA data outside the
repository. Observed on the full English dump: identical layer bytes for 6 and 3 threads; 2 702 of 1 491 059 entries
carry template gaps.
