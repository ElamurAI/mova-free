# roundtrip-volume-residual-gap-residual

## descriptant

### Description and epoch memory

residual delegates residual_with: classify literal pieces, optionally copy markup/repeats, charge inline mark coordinates; then record leading/noncanonical gaps and tails by block/sentence/token.

## approximant

### Forward relation

residual delegates residual_with: classify literal pieces, optionally copy markup/repeats, charge inline mark coordinates; then record leading/noncanonical gaps and tails by block/sentence/token.

Domain: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

No executable percentage surrogate is supplied: these categorical codec/effect observations have no justified lossy metric. Future caps are 1/2/5/10% only after defining metric, zero behavior and differential reference.

## catalog

# Source-bound typed models

Each section is a partial source-derived relation on the bound snapshot and declared domain; finite checks are falsification evidence, not a universal proof. Observation horizon is one call unless history is explicitly named.

## behavior_complement

Given/Conllu/FrTable literal pieces contribute no token residual; empty or single-space interior gaps are canonical and absent from R1.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## reverse_reconstruction

Given tokens/space bits and R1, admitted blocks reconstruct; R1 alone permits arbitrary token forms and canonical gaps. Residual numerical tally cannot recover gap strings.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## first_gap

First sentence canonical prefix empty; later prefix one space. Noncanonical even empty later gap stored.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## gap_class

Contains newline→Layout, otherwise Spacing, including tabs/carriage returns.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## tail_storage

Nonempty block tail stored unconditionally with literal bits.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.

## piece_boundary

Volume keeps literal pieces separately; R1 reconstructs text blocks only and cannot recover layout pieces.

Domain and held state: Valid Piece indices, Sent vectors and nonempty sentence forms; mark/gap UTF-8 boundaries; copy indices and pre/suf within earlier strings; representable counts and finite symbolic bit sums. Annotator and file readers unexecuted.

Status: partial source-derived model. Oracle: the named scoped tests in model.md#tests; unexercised branches remain inspection hypotheses.


## tests

### Actual checks

- `vol::wave32::copier_unicode_and_gap_reconstruction`
- `vol::tests::txt_pieces_give_the_text_back`

The listed parent/source tests exercise only their named mechanisms; other model statements are source inspection. Sixteen selected actual roundtrip library tests passed twice debug and twice release (64 exact-name executions), with identical sorted test outcomes. Eight compiled production-behavior mutants, one per source, failed assertions; exact originals were restored and the selected check passed. No assertion-only mutants or surviving compiled mutants. An initial mutation selector was ambiguous and stopped before writing that source; corrected selector retained separately.

## invariant-and-cost

### Invariants and cost

Given/Conllu/FrTable literal pieces contribute no token residual; empty or single-space interior gaps are canonical and absent from R1.

Given tokens/space bits and R1, admitted blocks reconstruct; R1 alone permits arbitrary token forms and canonical gaps. Residual numerical tally cannot recover gap strings.

#### Cost

Linear scan plus BTree inserts O(R log R) and copier charges; output strings proportional to retained gap bytes.

Integer additions/products and allocation sizes must fit native types. Floating costs require finite values; no throughput or all-input guarantee follows from synthetic tests.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained externally before edits. Existing production bytes preserved; only bounded inline tests appended to six owned files. Models distinguish byte reconstruction with residual from semantic understanding and persistent learning. Finite tests and compiled mutants support selected boundaries only. Source inspection identified noninjective projections and explicit admissibility restrictions; no bug repair or performance claim.

Combiner input is this source snapshot, syntax projection, models, costs and retained receipts. Proposed next epoch: target the stated unexecuted branches with bounded witnesses before expanding any claim. AST remains partial, macros opaque; no name resolution or proof.

### 2026-10-31 · tests

Raw logs, numeric statuses, original/restored hashes, prospective.md, native-commands.json and syntax pairs: `$MOVA_DATA/runs/octopus-wave32-rt_blocks-2026-10-31`. No live model/corpus/site/learning workflow or full pipeline was executed.
