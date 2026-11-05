# roundtrip-block-layout

## descriptant

### Description and epoch memory

group greedily emits contiguous ranges when group differs or w+n>max before current chapter; concat rebases Text indices by prior block count and uses first canon_sep. blocks maps each nonempty range to id/words/OR para_cut. like indexes ids last-wins, emits requested known ids in template order, omits empty matches. book_text scans keyword/caps headings after blank lines, preserves text and falls back to paragraph runs above HARD_MAX. Corpus adapters read and normalize records.

## approximant

### Forward relation

group greedily emits contiguous ranges when group differs or w+n>max before current chapter; concat rebases Text indices by prior block count and uses first canon_sep. blocks maps each nonempty range to id/words/OR para_cut. like indexes ids last-wins, emits requested known ids in template order, omits empty matches. book_text scans keyword/caps headings after blank lines, preserves text and falls back to paragraph runs above HARD_MAX. Corpus adapters read and normalize records.

Domain: Well-indexed volumes, valid UTF-8, representable word sums and index offsets. File adapters require readable matching files; no external corpus used.

No executable percentage surrogate is supplied: these categorical codec/effect observations have no justified lossy metric. Future caps are 1/2/5/10% only after defining metric, zero behavior and differential reference.

## composition

### Local composition

group greedily emits contiguous ranges when group differs or w+n>max before current chapter; concat rebases Text indices by prior block count and uses first canon_sep. blocks maps each nonempty range to id/words/OR para_cut. like indexes ids last-wins, emits requested known ids in template order, omits empty matches. book_text scans keyword/caps headings after blank lines, preserves text and falls back to paragraph runs above HARD_MAX. Corpus adapters read and normalize records.

Children scope selected internal mechanisms by byte ranges; remaining branches are covered only by the parent partial model. Their costs are included in parent costs, not added again. Shared values and caller order are those named above; sibling fragments are tagged independent where no direct call is named.

## catalog

# Source-bound typed models

Each section is a partial source-derived relation on the bound snapshot and declared domain; finite checks are falsification evidence, not a universal proof. Observation horizon is one call unless history is explicitly named.

## behavior_complement

Valid grouping never crosses group boundary and never splits a chapter; it can exceed max for oversized singleton. concat does not invent separator bytes. Missing ids in like cannot create placeholders.

Domain and held state: Well-indexed volumes, valid UTF-8, representable word sums and index offsets. File adapters require readable matching files; no external corpus used.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## reverse_reconstruction

Given concatenated text alone, every compatible chapter partition, metadata labeling and separator choice remains possible. Given ranges and chapters, concat reconstructs bytes uniquely. like cannot recover omitted chapters or earlier duplicate-id entries.

Domain and held state: Well-indexed volumes, valid UTF-8, representable word sums and index offsets. File adapters require readable matching files; no external corpus used.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## greedy_threshold

Equality w+n=max stays together; > splits except at first chapter. Zero-word chapters can share max=0.

Domain and held state: Well-indexed volumes, valid UTF-8, representable word sums and index offsets. File adapters require readable matching files; no external corpus used.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## index_rebasing

Text(b) becomes Text(b+off); literal pieces unchanged; first canon_sep controls new Volume.

Domain and held state: Well-indexed volumes, valid UTF-8, representable word sums and index offsets. File adapters require readable matching files; no external corpus used.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## duplicate_id

BTreeMap collect replaces earlier equal ids; template references preserve order and duplicate requests.

Domain and held state: Well-indexed volumes, valid UTF-8, representable word sums and index offsets. File adapters require readable matching files; no external corpus used.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## paragraph_limit

para_runs checks the first line at each paragraph start, not full upcoming paragraph; max is not a hard bound for arbitrary multi-line paragraphs.

Domain and held state: Well-indexed volumes, valid UTF-8, representable word sums and index offsets. File adapters require readable matching files; no external corpus used.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.


## tests

### Actual checks

- `blocks::wave32::grouping_and_duplicate_id_projection`
- `blocks::tests::book_chapters_give_the_file_back_and_runs_stay_whole`
- `blocks::tests::a_chapter_without_headings_is_cut_at_paragraphs`

The listed parent/source tests exercise only their named mechanisms; other model statements are source inspection. Sixteen selected actual roundtrip library tests passed twice debug and twice release (64 exact-name executions), with identical sorted test outcomes. Eight compiled production-behavior mutants, one per source, failed assertions; exact originals were restored and the selected check passed. No assertion-only mutants or surviving compiled mutants. An initial mutation selector was ambiguous and stopped before writing that source; corrected selector retained separately.

## invariant-and-cost

### Invariants and cost

Valid grouping never crosses group boundary and never splits a chapter; it can exceed max for oversized singleton. concat does not invent separator bytes. Missing ids in like cannot create placeholders.

Given concatenated text alone, every compatible chapter partition, metadata labeling and separator choice remains possible. Given ranges and chapters, concat reconstructs bytes uniquely. like cannot recover omitted chapters or earlier duplicate-id entries.

#### Cost

group O(total text bytes) word scans; concat O(copied text+pieces+blocks). like map O(C log C), lookups O(K log C) plus cloning. book_text O(bytes) typical, repeated chapter reconstruction and allocations can add copying. File adapters materialize files/maps; recursion has no explicit symlink cycle guard.

Integer additions/products and allocation sizes must fit native types. Floating costs require finite values; no throughput or all-input guarantee follows from synthetic tests.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained externally before edits. Existing production bytes preserved; only bounded inline tests appended to six owned files. Models distinguish byte reconstruction with residual from semantic understanding and persistent learning. Finite tests and compiled mutants support selected boundaries only. Source inspection identified noninjective projections and explicit admissibility restrictions; no bug repair or performance claim.

Combiner input is this source snapshot, syntax projection, models, costs and retained receipts. Proposed next epoch: target the stated unexecuted branches with bounded witnesses before expanding any claim. AST remains partial, macros opaque; no name resolution or proof.

### 2026-10-31 · tests

Raw logs, numeric statuses, original/restored hashes, prospective.md, native-commands.json and syntax pairs: `$MOVA_DATA/runs/octopus-wave32-rt_blocks-2026-10-31`. No live model/corpus/site/learning workflow or full pipeline was executed.
