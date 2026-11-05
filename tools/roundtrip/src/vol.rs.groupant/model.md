# roundtrip-volume-residual

## descriptant

### Description and epoch memory

txt_volume preserves paragraphs and layout, with_marks inserts markup at byte coordinates. segment aligns tokens against text then cut partitions punctuation boundaries; residual records noncanonical gaps/tails and charges layout/markup. decode_blocks combines canonical spaces and residual overrides, volume_text reapplies pieces/marks. Copier scans next/recent/head/tail candidates, picks strictly cheaper copy, verifies decode, updates counters/history.

## approximant

### Forward relation

txt_volume preserves paragraphs and layout, with_marks inserts markup at byte coordinates. segment aligns tokens against text then cut partitions punctuation boundaries; residual records noncanonical gaps/tails and charges layout/markup. decode_blocks combines canonical spaces and residual overrides, volume_text reapplies pieces/marks. Copier scans next/recent/head/tail candidates, picks strictly cheaper copy, verifies decode, updates counters/history.

Domain: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

No executable percentage surrogate is supplied: these categorical codec/effect observations have no justified lossy metric. Future caps are 1/2/5/10% only after defining metric, zero behavior and differential reference.

## composition

### Local composition

txt_volume preserves paragraphs and layout, with_marks inserts markup at byte coordinates. segment aligns tokens against text then cut partitions punctuation boundaries; residual records noncanonical gaps/tails and charges layout/markup. decode_blocks combines canonical spaces and residual overrides, volume_text reapplies pieces/marks. Copier scans next/recent/head/tail candidates, picks strictly cheaper copy, verifies decode, updates counters/history.

Children scope selected internal mechanisms by byte ranges; remaining branches are covered only by the parent partial model. Their costs are included in parent costs, not added again. Shared values and caller order are those named above; sibling fragments are tagged independent where no direct call is named.

## catalog

# Source-bound typed models

Each section is a partial source-derived relation on the bound snapshot and declared domain; finite checks are falsification evidence, not a universal proof. Observation horizon is one call unless history is explicitly named.

## behavior_complement

Residual coordinate override wins over canonical gaps; metadata always charged; Given/Conllu/FrTable literals excluded here. Copier never selects equal-cost copy over current candidate; returned code costs <=whole flag+literal bits on admitted domain.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## reverse_reconstruction

Token forms, spacing bits, gap/tail residual, pieces and ordered marks reconstruct bytes. Forms alone have many whitespace/layout antecedents. Copy plus referenced history reconstructs literal, but code alone does not recover absent history. Counter totals lose sequence.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## gap_coordinates

R1 key=(block,sentence,token), tail key=block; canonical gap before first sentence empty, later sentence one space.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## utf8_copy

Prefix/suffix lengths trimmed to common char boundaries, disjoint within min source/target length.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## history_tie

next candidate first, recent descending then head/tail; only strictly smaller cost replaces.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## mark_precondition

Offsets clamped to length but not char boundary; decreasing offsets insert at current cursor, so generated sorted boundaries are contract.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.


## tests

### Actual checks

- `vol::wave32::copier_unicode_and_gap_reconstruction`
- `vol::tests::txt_pieces_give_the_text_back`

The listed parent/source tests exercise only their named mechanisms; other model statements are source inspection. Sixteen selected actual roundtrip library tests passed twice debug and twice release (64 exact-name executions), with identical sorted test outcomes. Eight compiled production-behavior mutants, one per source, failed assertions; exact originals were restored and the selected check passed. No assertion-only mutants or surviving compiled mutants. An initial mutation selector was ambiguous and stopped before writing that source; corrected selector retained separately.

## invariant-and-cost

### Invariants and cost

Residual coordinate override wins over canonical gaps; metadata always charged; Given/Conllu/FrTable literals excluded here. Copier never selects equal-cost copy over current candidate; returned code costs <=whole flag+literal bits on admitted domain.

Token forms, spacing bits, gap/tail residual, pieces and ordered marks reconstruct bytes. Forms alone have many whitespace/layout antecedents. Copy plus referenced history reconstructs literal, but code alone does not recover absent history. Counter totals lose sequence.

#### Cost

Plain layout O(bytes); cut worst bounded closers scan can be quadratic in long closer sequences. residual O(pieces+tokens+marks+stored bytes) excluding copier. Copier retains all history O(H bytes); <=51 candidates but each comparison O(length), map O(log histories). repeat caller may clone entire history.

Integer additions/products and allocation sizes must fit native types. Floating costs require finite values; no throughput or all-input guarantee follows from synthetic tests.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained externally before edits. Existing production bytes preserved; only bounded inline tests appended to six owned files. Models distinguish byte reconstruction with residual from semantic understanding and persistent learning. Finite tests and compiled mutants support selected boundaries only. Source inspection identified noninjective projections and explicit admissibility restrictions; no bug repair or performance claim.

Combiner input is this source snapshot, syntax projection, models, costs and retained receipts. Proposed next epoch: target the stated unexecuted branches with bounded witnesses before expanding any claim. AST remains partial, macros opaque; no name resolution or proof.

### 2026-10-31 · tests

Raw logs, numeric statuses, original/restored hashes, prospective.md, native-commands.json and syntax pairs: `$MOVA_DATA/runs/octopus-wave32-rt_blocks-2026-10-31`. No live model/corpus/site/learning workflow or full pipeline was executed.
